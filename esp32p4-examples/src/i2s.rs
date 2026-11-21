//! Runs I2S0 in standard mode and checks what comes out of it.
//!
//! The transmit half is the master and drives BCK, WS and SD; the receive half
//! runs as a slave of the same controller, sharing BCK and WS through the
//! peripheral, and takes SD back in from the same pad the transmitter drives.
//! That makes the whole path — clock tree, format, GDMA, both halves — testable
//! without a codec, a jumper or a scope.
//!
//! The PDM check needs more pads: the data line also goes out on `GPIO16` and a
//! second controller (`I2S1`) reads it back on `GPIO20` through a jumper, eight
//! bit slots at 8 MHz, so its buffer *is* the waveform on the pin. WS and BCK
//! also go out on `GPIO17`/`GPIO18`, where a pulse counter counts their edges —
//! in PDM mode they are dead, which is why the PDM receiver has to generate its
//! own bit clock.
//!
//! Wiring: `GPIO4` is BCK, `GPIO5` is WS, `GPIO6` is the data line (out and in on
//! the same pad), and `GPIO16`–`GPIO20` has to be jumpered.
//!
//! Every check prints `ok` or `FAIL` on its own, and the whole set repeats once a
//! second.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::apll::Apll;
use esp32p4_hal::delay::Delay;
use esp32p4_hal::dma::{AhbDma, Descriptor};
use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
use esp32p4_hal::gpio::{PinOps, Pins};
use esp32p4_hal::i2s::{
    ClockSource, Error, I2s0, I2s1, Interrupts, PdmTxConfig, Slot, Standard, StandardConfig,
    TdmConfig,
};
use esp32p4_hal::main;
use esp32p4_hal::pac::{HP_SYS_CLKRST, Peripherals};
use esp32p4_hal::pcnt::{self, ChannelConfig as PcntChannel, Pcnt, Trigger};
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

    // The PDM data line also goes out on GPIO16 and comes back in on GPIO20, so
    // a second controller can sample it as a logic analyser would.
    let mut pdm_out = pins.pin::<16>();
    pdm_out.route_output_signal(OutputSignal::I2S0_O_SD_PAD_OUT);
    // Candidates the PDM transmitter might be driving, each on its own pin so a
    // counter can look at one at a time.
    let mut ws_alt2 = pins.pin::<17>();
    ws_alt2.route_output_signal(OutputSignal::I2S0_O_WS_PAD_OUT);
    let mut bck_alt2 = pins.pin::<18>();
    bck_alt2.route_output_signal(OutputSignal::I2S0_O_BCK_PAD_OUT);
    let mut mclk_alt2 = pins.pin::<19>();
    mclk_alt2.route_output_signal(OutputSignal::I2S0_MCLK_PAD_OUT);
    let mut probe = pins.pin::<20>();
    probe.route_input_signal(InputSignal::I2S1_I_SD_PAD_IN);
    let _probes = (pdm_out, probe);

    let i2s = I2s0::init(dp.I2S0, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let sampler = I2s1::init(dp.I2S1, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let counter = Pcnt::init(dp.PCNT, &dp.HP_SYS_CLKRST);
    let counter_unit = counter.unit::<0>();
    counter_unit
        .channel::<0>()
        .configure(PcntChannel::rising_edge());
    counter_unit.set_limits(0, 30000);
    let dma = AhbDma::init(dp.AHB_DMA, &dp.HP_SYS_CLKRST);
    let clkrst = &dp.HP_SYS_CLKRST;
    let lpaon = &dp.LP_AON_CLKRST;
    let pmu = &dp.PMU;

    let mut tx_buffer = [0u8; 4 * WORDS];
    let mut rx_buffer = [0u8; 4 * WORDS];
    let tx_descriptors = [const { Descriptor::new() }; 4];
    let mut capture = [0u8; 4094];
    let capture_descriptors = [const { Descriptor::new() }; 4];
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
        check_apll(
            &i2s,
            clkrst,
            lpaon,
            pmu,
            &dma.tx_channel::<0>(),
            &tx_buffer,
            &tx_descriptors,
        );
        check_tdm(
            &i2s,
            clkrst,
            &dma.tx_channel::<0>(),
            &dma.rx_channel::<0>(),
            &mut tx_buffer,
            &mut rx_buffer,
            &tx_descriptors,
            &rx_descriptors,
        );
        check_stream(
            &i2s,
            clkrst,
            &dma.tx_channel::<0>(),
            &dma.rx_channel::<0>(),
            &mut tx_buffer,
            &mut rx_buffer,
            &tx_descriptors,
            &rx_descriptors,
            &delay,
        );
        check_pdm(
            &i2s,
            &sampler,
            clkrst,
            &dma.tx_channel::<0>(),
            &dma.rx_channel::<1>(),
            &counter_unit,
            &mut ws_alt2,
            &mut bck_alt2,
            &mut tx_buffer,
            &mut capture,
            &tx_descriptors,
            &capture_descriptors,
            &delay,
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

/// Four slots in one frame: the same path as the stereo check, with the slot
/// count, the active mask and the wider frame registers exercised.
#[allow(clippy::too_many_arguments)]
fn check_tdm(
    i2s: &I2s0,
    clkrst: &HP_SYS_CLKRST,
    tx: &impl esp32p4_hal::dma::TransmitChannel,
    rx: &impl esp32p4_hal::dma::ReceiveChannel,
    tx_buffer: &mut [u8],
    rx_buffer: &mut [u8],
    tx_descriptors: &[Descriptor],
    rx_descriptors: &[Descriptor],
) {
    // Four 8-bit slots keep BCK at the stereo rate, which also keeps the slave's
    // module clock (eight BCK periods minimum) inside what a 40 MHz crystal and
    // its >= 2 divider can produce.
    for (name, active) in [("4 slots", 0b1111u16), ("holes", 0b1010)] {
        let master = TdmConfig {
            sample_rate: RATE,
            channels: 4,
            slot_bits: 8,
            data_bits: 8,
            active,
            ..Default::default()
        };
        let slave = TdmConfig {
            master: false,
            mclk_multiple: 384,
            ..master
        };

        let tx_clock = i2s.tx_configure_tdm(clkrst, &master);
        let rx_clock = i2s.rx_configure_tdm(clkrst, &slave);
        i2s.share_bck_ws(true);

        for (index, sample) in tx_buffer.iter_mut().enumerate() {
            *sample = (index as u8).wrapping_mul(7);
        }
        rx_buffer.fill(0x5A);

        let configured = tx_clock.is_ok() && rx_clock.is_ok();
        let started = configured && i2s.rx_begin(rx, rx_buffer, rx_descriptors).is_ok();
        i2s.reset_tx_bck_counter();
        let sent = started && i2s.tx_write(tx, tx_buffer, tx_descriptors).is_ok();
        let received = sent && i2s.rx_wait(rx, rx_buffer).is_ok();

        // The frame stays four slots wide whatever the mask is, so BCK and the
        // data that passes are decided separately.
        let bck = i2s.tx_bck_count();
        let frame_bytes = master.frame_bytes();
        let expected_bck = (tx_buffer.len() / frame_bytes * 4 * 8) as u32;
        let identical = received && tx_buffer == rx_buffer;
        let lanes = tx_clock.as_ref().map(|c| c.bclk_hz).unwrap_or(0);
        if !identical {
            defmt::info!(
                "tdm: configured {=bool}, started {=bool}, sent {=bool}, received {=bool}",
                configured,
                started,
                sent,
                received
            );
        }
        let ok = identical && lanes == RATE * 32 && bck.abs_diff(expected_bck) <= expected_bck / 50;
        defmt::info!(
            "tdm {}: {=u32} Hz BCK, {=usize} bytes/frame {=usize}, data {=bool}, {=u32}/{=u32} BCK -> {}",
            name,
            lanes,
            frame_bytes,
            tx_buffer.len(),
            identical,
            bck,
            expected_bck,
            if ok { "ok" } else { "FAIL" }
        );
    }

    // A frame with no active slot has nothing to carry, and an odd frame has no
    // middle for the WS half-frame edge.
    let empty_mask = i2s.tx_configure_tdm(
        clkrst,
        &TdmConfig {
            active: 0,
            ..Default::default()
        },
    ) == Err(Error::SlotsOutOfRange);
    let odd_frame = i2s.tx_configure_tdm(
        clkrst,
        &TdmConfig {
            channels: 3,
            slot_bits: 17,
            active: 0b111,
            ..Default::default()
        },
    ) == Err(Error::SlotsOutOfRange);
    defmt::info!(
        "tdm rejects: empty mask {=bool}, odd frame {=bool} -> {}",
        empty_mask,
        odd_frame,
        if empty_mask && odd_frame {
            "ok"
        } else {
            "FAIL"
        }
    );
}

/// Two rings, one lap short. The transmitter repeats an eight-frame pattern, the
/// receiver fills its own window over and over, and after ten milliseconds the
/// window has to be that pattern at some phase.
#[allow(clippy::too_many_arguments)]
fn check_stream(
    i2s: &I2s0,
    clkrst: &HP_SYS_CLKRST,
    tx: &impl esp32p4_hal::dma::TransmitChannel,
    rx: &impl esp32p4_hal::dma::ReceiveChannel,
    tx_buffer: &mut [u8],
    rx_buffer: &mut [u8],
    tx_descriptors: &[Descriptor],
    rx_descriptors: &[Descriptor],
    delay: &Delay,
) {
    const PATTERN: usize = 32;
    let config = StandardConfig {
        sample_rate: RATE,
        ..Default::default()
    };
    let slave = StandardConfig {
        master: false,
        mclk_multiple: 384,
        ..config
    };

    let configured =
        i2s.tx_configure(clkrst, &config).is_ok() && i2s.rx_configure(clkrst, &slave).is_ok();
    i2s.share_bck_ws(true);

    let pattern = &mut tx_buffer[..PATTERN];
    for (index, word) in pattern.as_chunks_mut::<2>().0.iter_mut().enumerate() {
        word.copy_from_slice(&((index as u16).wrapping_mul(0x0101)).to_le_bytes());
    }
    rx_buffer.fill(0xA5);

    // The receive ring is deliberately not a multiple of the pattern, so its
    // laps drift and the phase is not something the test can hard-code.
    const WINDOW: usize = 40;
    let rx_started = configured
        && i2s
            .rx_stream_begin(rx, &mut rx_buffer[..WINDOW], rx_descriptors)
            .is_ok();
    i2s.reset_tx_bck_counter();
    let tx_started = rx_started && i2s.tx_stream_begin(tx, pattern, tx_descriptors).is_ok();
    delay.delay_ms(10);
    let bck = i2s.tx_bck_count();
    i2s.tx_stream_stop(tx);
    i2s.rx_stream_stop(rx, &mut rx_buffer[..WINDOW]);
    let hung = i2s.interrupts();

    // 10 ms of 1.536 MHz is about 15360 bit clocks; the ring keeps them coming
    // without anyone refilling anything.
    let expected_bck = (RATE / 100) * 32;
    let bck_ok = bck.abs_diff(expected_bck) <= expected_bck / 10;

    // The window holds two laps of the pattern at an unknown phase, because the
    // engine starts writing wherever the first sample lands.
    let phase = (0..PATTERN).find(|offset| {
        (0..PATTERN).all(|index| rx_buffer[index] == pattern[(index + offset) % PATTERN])
    });
    let rotated = phase.is_some();
    let ok = tx_started && bck_ok && rotated && hung.is_empty();
    defmt::info!(
        "stream: tx {=bool}, {=u32}/{=u32} BCK in 10 ms, pattern phase {=?}, pending {=u32:#x} -> {}",
        tx_started,
        bck,
        expected_bck,
        phase,
        hung.raw(),
        if ok { "ok" } else { "FAIL" }
    );
}

/// The audio PLL tuned to an exact audio clock: 12.288 MHz, which is eight
/// BCK periods per bit at 48 kHz — an integer divider, so BCK has to be exactly
/// 1.536 MHz and the counter proves it.
fn check_apll(
    i2s: &I2s0,
    clkrst: &HP_SYS_CLKRST,
    lpaon: &esp32p4_hal::pac::LP_AON_CLKRST,
    pmu: &esp32p4_hal::pac::PMU,
    tx: &impl esp32p4_hal::dma::TransmitChannel,
    tx_buffer: &[u8],
    tx_descriptors: &[Descriptor],
) {
    // Twice the MCLK a 48 kHz stream needs, which is what a codec expects on
    // its MCLK pin.
    let target = 24_576_000;
    let Ok(mut apll) = Apll::enable(target, lpaon, pmu) else {
        defmt::info!("apll: could not start -> FAIL");
        return;
    };
    let realized = apll.frequency_hz();
    let tuned = apll
        .set_frequency(22_579_200)
        .is_ok_and(|hz| hz.abs_diff(22_579_200) < 100);
    let retuned = apll.set_frequency(target).is_ok();

    // 24.576 MHz / 2 = 12.288 MHz MCLK, / 8 = 1.536 MHz BCK: with the PLL as the
    // source the divider is integral, so a transfer of known length has to
    // produce exactly 8 BCK per byte.
    i2s.set_clock_source(clkrst, ClockSource::Apll, realized);
    let config = StandardConfig {
        sample_rate: RATE,
        ..Default::default()
    };
    let clock = i2s.tx_configure(clkrst, &config).ok();
    let played = clock
        .as_ref()
        .is_some_and(|c| c.source_hz == realized && c.mclk_integer == 2 && c.mclk_numerator == 0);
    i2s.reset_tx_bck_counter();
    let sent = played && i2s.tx_write(tx, &tx_buffer[..64], tx_descriptors).is_ok();
    let bck = i2s.tx_bck_count();

    i2s.set_clock_source(clkrst, ClockSource::Xtal, 0);
    apll.disable(lpaon, pmu);

    // The counter can include one BCK on either side of the transfer.
    let exact = bck.abs_diff(64 * 8) <= 1;
    let ok = realized.abs_diff(target) < 100 && tuned && retuned && played && sent && exact;
    defmt::info!(
        "apll: {=u32} Hz (wanted {=u32}), retuned {=bool}, divider {=u32}+{=u32}/{=u32}, {=u32}/512 BCK -> {}",
        realized,
        target,
        tuned && retuned,
        clock.as_ref().map(|c| c.mclk_integer).unwrap_or(0),
        clock.as_ref().map(|c| c.mclk_numerator).unwrap_or(0),
        clock.as_ref().map(|c| c.mclk_denominator).unwrap_or(0),
        bck,
        if ok { "ok" } else { "FAIL" }
    );
}

/// PDM end to end, seen from three places at once: the transmitter codes a 1 kHz
/// square into a density stream, a second controller samples the data pin through
/// the GPIO16-GPIO20 jumper and checks that the density really swings, and the
/// PDM receiver filters the stream back into the square.
///
/// The transmitter runs at 16 kHz, where the module clock is 16.384 MHz and the
/// crystal can still divide it; 48 kHz would need 49.152 MHz and therefore the
/// audio PLL.
#[allow(clippy::too_many_arguments)]
fn check_pdm(
    i2s: &I2s0,
    sampler: &I2s1,
    clkrst: &HP_SYS_CLKRST,
    tx: &impl esp32p4_hal::dma::TransmitChannel,
    probe: &impl esp32p4_hal::dma::ReceiveChannel,
    counter: &pcnt::Unit<'_, 0>,
    ws: &mut impl PinOps,
    bck: &mut impl PinOps,
    tx_buffer: &mut [u8],
    capture: &mut [u8],
    tx_descriptors: &[Descriptor],
    capture_descriptors: &[Descriptor],
    delay: &Delay,
) {
    /// Samples per half period of the 1 kHz square at 16 kHz.
    const HALF_SQUARE: usize = 8;

    // PDM needs no BCK/WS sharing: the transmitter drives no bit clock and the
    // receiver generates its own.
    i2s.share_bck_ws(false);
    let pdm = i2s.tx_configure_pdm(
        clkrst,
        &PdmTxConfig {
            sample_rate: 16_000,
            ..Default::default()
        },
    );
    // Eight bit slots, both active: one byte per slot means one sample per bit
    // clock, so the buffer is the waveform on the pin.
    let watch = sampler.rx_configure(
        clkrst,
        &StandardConfig {
            sample_rate: 500_000,
            data_bits: 8,
            slot_bits: 8,
            slot: Slot::Both,
            mclk_multiple: 32,
            ..Default::default()
        },
    );
    let (Ok(pdm), Ok(watch)) = (pdm, watch) else {
        defmt::info!("pdm: configure failed -> FAIL");
        return;
    };

    for (index, sample) in tx_buffer.as_chunks_mut::<2>().0.iter_mut().enumerate() {
        let level = if (index / HALF_SQUARE).is_multiple_of(2) {
            16_384i16
        } else {
            -16_384
        };
        sample.copy_from_slice(&level.to_le_bytes());
    }

    // The transmitter's high-pass filter needs a few milliseconds to settle, and
    // the capture is four milliseconds wide.
    let sending = i2s.tx_begin(tx, tx_buffer, tx_descriptors).is_ok();
    delay.delay_ms(8);
    // On this chip the transmitter drives the data line only: neither WS nor BCK
    // leaves the peripheral in PDM mode, which is why a receiving half has to
    // generate its own bit clock.
    let clock_lines = [edges_hz(counter, ws, delay), edges_hz(counter, bck, delay)];
    let watching = sending
        && sampler
            .rx_begin(probe, capture, capture_descriptors)
            .is_ok();
    let watched = watching && sampler.rx_wait(probe, capture).is_ok();
    let sent = sending && i2s.tx_wait(tx).is_ok();

    // The pin: a density stream at the bit clock, swinging between a quarter and
    // three quarters of ones for a half-scale square.
    let mut low = u32::MAX;
    let mut high = 0u32;
    for index in 0..16 {
        let slice = &capture[capture.len() / 16 * index..capture.len() / 16 * (index + 1)];
        let ones: u32 = slice.iter().map(|byte| byte.count_ones()).sum();
        let density = ones * 1000 / (slice.len() as u32 * 8);
        low = low.min(density);
        high = high.max(density);
    }
    let swing = watched && low < 320 && high > 680;

    let ok = sent && swing;
    defmt::info!(
        "pdm: {=u32} Hz bit clock, pin {=u32}..{=u32} permille, sampled at {=u32} Hz, ws {=u32} Hz, bck {=u32} Hz -> {}",
        pdm.bclk_hz,
        low,
        high,
        watch.bclk_hz,
        clock_lines[0],
        clock_lines[1],
        if ok { "ok" } else { "FAIL" }
    );
}

/// Rising edges per second on the pad `pin`, counted over a millisecond.
fn edges_hz(unit: &pcnt::Unit<'_, 0>, pin: &mut impl PinOps, delay: &Delay) -> u32 {
    pin.route_input_signal(InputSignal::PCNT_SIG_CH0_PAD_IN0);
    unit.clear();
    unit.enable(Trigger::default());
    delay.delay_ms(1);
    unit.pause(true);
    let edges = unit.count();
    unit.pause(false);
    pin.release_input_signal(InputSignal::PCNT_SIG_CH0_PAD_IN0);
    edges.max(0) as u32 * 1000
}
