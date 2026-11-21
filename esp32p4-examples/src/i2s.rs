//! Runs I2S0 in standard mode and checks what comes out of it.
//!
//! The transmit half is the master and drives BCK, WS and SD; the receive half
//! runs as a slave of the same controller, sharing BCK and WS through the
//! peripheral, and takes SD back in from the same pad the transmitter drives.
//! That makes the whole path — clock tree, format, GDMA, both halves — testable
//! without a codec, a jumper or a scope.
//!
//! Wiring if pads are watched: `GPIO4` is BCK, `GPIO5` is WS, `GPIO6` is the
//! data line (out and in on the same pad).
//!
//! Every check prints `ok` or `FAIL` on its own, and the whole set repeats once a
//! second.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::delay::Delay;
use esp32p4_hal::dma::{AhbDma, Descriptor};
use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
use esp32p4_hal::gpio::{PinOps, Pins};
use esp32p4_hal::i2s::{Error, I2s0, Interrupts, Slot, Standard, StandardConfig};
use esp32p4_hal::main;
use esp32p4_hal::pac::{HP_SYS_CLKRST, Peripherals};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Pad carrying `I2S0O_BCK_out`.
const BCK_PIN: u32 = 4;
/// Pad carrying `I2S0O_WS_out`.
const WS_PIN: u32 = 5;
/// Pad carrying the data line in both directions.
const SD_PIN: u32 = 6;

/// Sampling rate the checks run at.
const RATE: u32 = 48_000;
/// Words in the loopback buffer; stereo, so half as many frames.
const WORDS: usize = 256;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let delay = Delay::init(dp.SYSTIMER);

    // The transmit half drives all three lines; the receive half reads the
    // same pads, so the peripheral sees exactly what it drives.
    let mut bck = pins.pin::<BCK_PIN>();
    bck.route_output_signal(OutputSignal::I2S0_O_BCK_PAD_OUT);
    let mut ws = pins.pin::<WS_PIN>();
    ws.route_output_signal(OutputSignal::I2S0_O_WS_PAD_OUT);
    let mut sd = pins.pin::<SD_PIN>();
    sd.route_output_signal(OutputSignal::I2S0_O_SD_PAD_OUT);
    sd.route_input_signal(InputSignal::I2S0_I_SD_PAD_IN);
    let _pads = (bck, ws, sd);

    let i2s = I2s0::init(dp.I2S0, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let dma = AhbDma::init(dp.AHB_DMA, &dp.HP_SYS_CLKRST);
    let clkrst = &dp.HP_SYS_CLKRST;

    let mut tx_buffer = [0u8; 4 * WORDS];
    let mut rx_buffer = [0u8; 4 * WORDS];
    let tx_descriptors = [const { Descriptor::new() }; 4];
    let rx_descriptors = [const { Descriptor::new() }; 4];

    loop {
        check_clock(&i2s, clkrst);
        check_format(&i2s, clkrst);
        check_rejects(&i2s, clkrst);
        check_loopback(
            &i2s,
            clkrst,
            &dma.tx_channel::<0>(),
            &dma.rx_channel::<0>(),
            &mut tx_buffer,
            &mut rx_buffer,
            &tx_descriptors,
            &rx_descriptors,
        );
        check_interrupts(
            &i2s,
            clkrst,
            &dma.tx_channel::<0>(),
            &mut tx_buffer,
            &tx_descriptors,
        );

        delay.delay_ms(1_000);
    }
}

/// The clock tree turns a sample rate into an MCLK and a BCK that the 40 MHz
/// crystal can actually produce.
fn check_clock(i2s: &I2s0, clkrst: &HP_SYS_CLKRST) {
    let Some(clock) = i2s.tx_configure(clkrst, &StandardConfig::default()).ok() else {
        defmt::info!("clock: configure failed -> FAIL");
        return;
    };

    // 48 kHz x 256 = 12.288 MHz needs 40/12.288 = 3 + 131/512; BCK is 1/8 of it.
    let mclk_off = clock.mclk_hz.abs_diff(RATE * 256);
    let ok = clock.source_hz == 40_000_000
        && clock.bclk_hz == RATE * 32
        && clock.bck_divisor == 8
        && clock.mclk_integer == 3
        && clock.mclk_numerator == 131
        && clock.mclk_denominator == 512
        && mclk_off <= (RATE * 256) / 1000;
    defmt::info!(
        "clock: {=u32} Hz source / ({=u32} + {=u32}/{=u32}) = {=u32} Hz MCLK, /{=u32} = {=u32} Hz BCK -> {}",
        clock.source_hz,
        clock.mclk_integer,
        clock.mclk_numerator,
        clock.mclk_denominator,
        clock.mclk_hz,
        clock.bck_divisor,
        clock.bclk_hz,
        if ok { "ok" } else { "FAIL" }
    );
}

/// Format arithmetic: slot width, mono frames and the resulting bit clock.
fn check_format(i2s: &I2s0, clkrst: &HP_SYS_CLKRST) {
    let wide = StandardConfig {
        slot_bits: 32,
        data_bits: 16,
        ..Default::default()
    };
    let mono = StandardConfig {
        slot: Slot::Left,
        slot_bits: 24,
        data_bits: 24,
        // A multiple of three keeps the 24-bit frame dividing exactly.
        mclk_multiple: 384,
        ..Default::default()
    };
    let left = StandardConfig {
        slot: Slot::Left,
        ..Default::default()
    };
    let msb = StandardConfig {
        standard: Standard::MsbAligned,
        ws_idle_high: true,
        ..Default::default()
    };

    let wide_clock = i2s.tx_configure(clkrst, &wide).ok();
    let mono_clock = i2s.tx_configure(clkrst, &mono).ok();
    let left_clock = i2s.tx_configure(clkrst, &left).ok();
    let msb_clock = i2s.tx_configure(clkrst, &msb).ok();

    // A frame is always two slots wide, so a mono stream keeps BCK and only
    // halves the bytes it pulls out of the DMA buffer.
    let ok = wide_clock.is_some_and(|c| c.bclk_hz == RATE * 64 && c.bck_divisor == 4)
        && mono_clock.is_some_and(|c| c.bclk_hz == RATE * 48 && c.bck_divisor == 8)
        && left_clock.is_some_and(|c| c.bclk_hz == RATE * 32)
        && msb_clock.is_some_and(|c| c.bclk_hz == RATE * 32)
        && mono.frame_bytes() == 3
        && wide.frame_bytes() == 8;
    defmt::info!(
        "format: 32-bit slots {=?}, mono 24-bit {=?}, left-only 16-bit {=?} -> {}",
        wide_clock.map(|c| c.bclk_hz),
        mono_clock.map(|c| c.bclk_hz),
        left_clock.map(|c| c.bclk_hz),
        if ok { "ok" } else { "FAIL" }
    );
}

/// Configurations the hardware cannot produce are refused instead of being
/// rounded into something that nearly works.
fn check_rejects(i2s: &I2s0, clkrst: &HP_SYS_CLKRST) {
    let too_wide = i2s.tx_configure(
        clkrst,
        &StandardConfig {
            data_bits: 24,
            slot_bits: 16,
            ..Default::default()
        },
    ) == Err(Error::DataWiderThanSlot);
    let no_bits = i2s.tx_configure(
        clkrst,
        &StandardConfig {
            data_bits: 0,
            ..Default::default()
        },
    ) == Err(Error::BitsOutOfRange);
    let too_fast = i2s.tx_configure(
        clkrst,
        &StandardConfig {
            sample_rate: 5_000_000,
            ..Default::default()
        },
    ) == Err(Error::RateUnsupported);
    // 100 Hz x 256 is below what the crystal can divide to.
    let too_slow = i2s.tx_configure(
        clkrst,
        &StandardConfig {
            sample_rate: 100,
            ..Default::default()
        },
    ) == Err(Error::RateUnsupported);
    // A slave samples with a module clock, which needs eight BCK per bit; 128x
    // the sample rate is only four.
    let slave_too_slow = i2s.rx_configure(
        clkrst,
        &StandardConfig {
            master: false,
            mclk_multiple: 128,
            ..Default::default()
        },
    ) == Err(Error::SlaveClockTooSlow);

    let ok = too_wide && no_bits && too_fast && too_slow && slave_too_slow;
    defmt::info!(
        "rejects: wide {=bool}, no bits {=bool}, fast {=bool}, slow {=bool}, slave {=bool} -> {}",
        too_wide,
        no_bits,
        too_fast,
        too_slow,
        slave_too_slow,
        if ok { "ok" } else { "FAIL" }
    );
}

/// Runs one direction pair and reports whether the bytes came back unchanged.
///
/// The transmit half is the master; the receive half is its slave and shares BCK
/// and WS through the peripheral, so only the data line is looped through a pad.
#[allow(clippy::too_many_arguments)]
fn loopback(
    i2s: &I2s0,
    clkrst: &HP_SYS_CLKRST,
    tx: &impl esp32p4_hal::dma::TransmitChannel,
    rx: &impl esp32p4_hal::dma::ReceiveChannel,
    config: &StandardConfig,
    tx_buffer: &mut [u8],
    rx_buffer: &mut [u8],
    tx_descriptors: &[Descriptor],
    rx_descriptors: &[Descriptor],
) -> (bool, u32, u32) {
    let master = *config;
    let slave = StandardConfig {
        master: false,
        // A slave needs a module clock of at least eight BCK periods.
        mclk_multiple: 384,
        ..master
    };

    let configured =
        i2s.tx_configure(clkrst, &master).is_ok() && i2s.rx_configure(clkrst, &slave).is_ok();
    i2s.share_bck_ws(true);

    for (index, word) in tx_buffer.as_chunks_mut::<2>().0.iter_mut().enumerate() {
        word.copy_from_slice(&((index as u16).wrapping_mul(64)).to_le_bytes());
    }
    rx_buffer.fill(0xA5);

    let started = configured && i2s.rx_begin(rx, rx_buffer, rx_descriptors).is_ok();
    i2s.reset_tx_bck_counter();
    let sent = started && i2s.tx_write(tx, tx_buffer, tx_descriptors).is_ok();
    let received = sent && i2s.rx_wait(rx, rx_buffer).is_ok();

    let bck = i2s.tx_bck_count();
    // One BCK per bit, and a frame is two slots wide.
    let frame_bytes = config.frame_bytes();
    let expected_bck = (tx_buffer.len() / frame_bytes * usize::from(config.slot_bits) * 2) as u32;

    let identical = received && tx_buffer == rx_buffer;
    if !identical {
        defmt::info!(
            "loopback: sent {=usize} bytes, configured {=bool}, started {=bool}, sent {=bool}, received {=bool}",
            tx_buffer.len(),
            configured,
            started,
            sent,
            received
        );
    }
    (identical, bck, expected_bck)
}

/// The whole path, twice: a stereo stream and a mono one.
#[allow(clippy::too_many_arguments)]
fn check_loopback(
    i2s: &I2s0,
    clkrst: &HP_SYS_CLKRST,
    tx: &impl esp32p4_hal::dma::TransmitChannel,
    rx: &impl esp32p4_hal::dma::ReceiveChannel,
    tx_buffer: &mut [u8],
    rx_buffer: &mut [u8],
    tx_descriptors: &[Descriptor],
    rx_descriptors: &[Descriptor],
) {
    let stereo = StandardConfig {
        sample_rate: RATE,
        ..Default::default()
    };
    let mono = StandardConfig {
        sample_rate: RATE,
        slot: Slot::Left,
        ..Default::default()
    };

    for (name, config) in [("stereo", stereo), ("mono", mono)] {
        let (identical, bck, expected_bck) = loopback(
            i2s,
            clkrst,
            tx,
            rx,
            &config,
            tx_buffer,
            rx_buffer,
            tx_descriptors,
            rx_descriptors,
        );
        let bck_ok = bck.abs_diff(expected_bck) <= expected_bck / 50;
        defmt::info!(
            "loopback {}: {=usize} bytes, data {=bool}, {=u32}/{=u32} BCK -> {}",
            name,
            tx_buffer.len(),
            identical,
            bck,
            expected_bck,
            if identical && bck_ok { "ok" } else { "FAIL" }
        );
    }
}

/// The status bits a disabled stream would never set: `TX_DONE` latches when the
/// last word left, and clearing it puts the status register back to zero.
fn check_interrupts(
    i2s: &I2s0,
    clkrst: &HP_SYS_CLKRST,
    tx: &impl esp32p4_hal::dma::TransmitChannel,
    tx_buffer: &mut [u8],
    tx_descriptors: &[Descriptor],
) {
    let config = StandardConfig {
        sample_rate: RATE,
        slot: Slot::Both,
        ..Default::default()
    };
    let configured = i2s.tx_configure(clkrst, &config).is_ok();
    i2s.share_bck_ws(false);
    i2s.disable_interrupts();
    i2s.clear_interrupts(
        Interrupts::TX_DONE
            .with(Interrupts::RX_DONE)
            .with(Interrupts::TX_HUNG)
            .with(Interrupts::RX_HUNG),
    );
    let idle = i2s.interrupts().is_empty();

    // The source is enabled but never routed to a CPU line, so this only sets
    // the peripheral's own status bit.
    i2s.enable_interrupts(Interrupts::TX_DONE);
    let sent = i2s.tx_write(tx, tx_buffer, tx_descriptors).is_ok();
    let fired = i2s.interrupts().contains(Interrupts::TX_DONE);
    i2s.clear_interrupts(Interrupts::TX_DONE);
    let cleared = i2s.interrupts().is_empty();
    i2s.disable_interrupts();

    let ok = configured && idle && sent && fired && cleared;
    defmt::info!(
        "interrupt: idle {=bool}, fired {=bool}, cleared {=bool} -> {}",
        idle,
        fired,
        cleared,
        if ok { "ok" } else { "FAIL" }
    );
}
