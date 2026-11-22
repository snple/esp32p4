//! Loops a 100-LED WS2812 frame through RMT with GDMA on both ends.
//!
//! Transmit channel 3 and receive channel 3 are the only ones whose RAM the
//! GDMA interface can reach, and one of them holds 48 words — 96 pulse codes.
//! The frame here is 4800 codes, fifty times the RAM, so it only goes out (and
//! comes back) because the engine keeps refilling the RAM while the
//! transmitter plays it.
//!
//! The pad is its own loopback: transmit on `GPIO2`, receive from the same pad
//! through the input path, no wiring. The 20 MHz channel clock makes one tick
//! 50 ns: a WS2812 "0" is 7 high ticks and 16 low ticks, a "1" is 14 + 12.
//!
//! Every level and duration the receiver decodes is compared against what was
//! sent; the example panics on the first mismatch.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::delay;
use esp32p4_hal::dma::{AhbDma, Descriptor};
use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
use esp32p4_hal::gpio::{Level, PinOps, Pins};
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::rmt::{ClockSource, Code, Pulse, Rmt, RxConfig, TxConfig};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Pad shared by the transmitter and the receiver.
const PIN: u32 = 2;

/// Group divider: 40 MHz / 2 = 20 MHz, so one tick is 50 ns.
const DIVIDER: u32 = 2;

/// Ticks of unchanged level that end a reception: 1000 ticks = 50 µs, longer
/// than the longest bit and shorter than the closing gap of the frame.
const IDLE_TICKS: u16 = 1_000;

/// LEDs in the frame.
const LEDS: usize = 100;
/// Pulse codes the frame carries: 24 bits per LED, two codes per bit.
const CODES: usize = LEDS * 48;
/// One code per closing gap, plus the end marker.
const SENT: usize = CODES + 2;
/// Room for the end marker the receiver appends.
const RECEIVED: usize = CODES + 8;

/// A descriptor carries at most 4095 bytes, i.e. 2047 codes.
const DESCRIPTORS: usize = (SENT * core::mem::size_of::<Code>()).div_ceil(4095);

static mut SENT_PULSES: [Code; SENT] = [Code::END; SENT];
static mut RECEIVED_PULSES: [Code; RECEIVED] = [Code::END; RECEIVED];
static mut TX_DESCRIPTORS: [Descriptor; DESCRIPTORS] = [const { Descriptor::new() }; DESCRIPTORS];
static mut RX_DESCRIPTORS: [Descriptor; DESCRIPTORS] = [const { Descriptor::new() }; DESCRIPTORS];

/// One byte as WS2812 bits, most significant bit first, in tick pairs.
fn byte_codes(byte: u8, out: &mut [Code]) -> usize {
    let mut count = 0;
    for bit in (0..8).rev() {
        if byte & (1 << bit) != 0 {
            out[count] = Pulse::new(Level::High, 14).into();
            out[count + 1] = Pulse::new(Level::Low, 12).into();
        } else {
            out[count] = Pulse::new(Level::High, 7).into();
            out[count + 1] = Pulse::new(Level::Low, 16).into();
        }
        count += 2;
    }
    count
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);

    let rmt = Rmt::init(dp.RMT, &dp.HP_SYS_CLKRST);
    rmt.set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::Xtal40m, DIVIDER)
        .unwrap();
    let dma = AhbDma::init(dp.AHB_DMA, &dp.HP_SYS_CLKRST);

    let mut pin = pins.pin::<PIN>();
    pin.route_output_signal(OutputSignal::RMT_SIG_PAD_OUT3);
    pin.route_input_signal(InputSignal::RMT_SIG_PAD_IN3);

    let tx = rmt.tx_channel::<3>();
    let rx = rmt.rx_channel::<3>();
    tx.configure(&TxConfig::new(1)).unwrap();
    rx.configure(&RxConfig::new(1, IDLE_TICKS)).unwrap();
    let dma_tx = dma.tx_channel::<1>();
    let dma_rx = dma.rx_channel::<0>();

    // SAFETY: the statics are used by nothing else, and only one frame runs.
    let sent = unsafe { &mut *core::ptr::addr_of_mut!(SENT_PULSES) };
    let received = unsafe { &mut *core::ptr::addr_of_mut!(RECEIVED_PULSES) };
    let tx_descriptors = unsafe { &*core::ptr::addr_of!(TX_DESCRIPTORS) };
    let rx_descriptors = unsafe { &*core::ptr::addr_of!(RX_DESCRIPTORS) };

    // Green, red, blue per LED, with the level walking upwards so that no two
    // neighbouring codes look alike.
    let mut count = 0;
    for index in 0..LEDS {
        let level = (index % 6) as u8 * 0x20;
        for byte in [level, level ^ 0xff, level.wrapping_add(0x40)] {
            count += byte_codes(byte, &mut sent[count..]);
        }
    }
    // The closing gap the receiver's idle threshold ends the frame on, then
    // the marker the transmitter stops at.
    sent[count] = Pulse::new(Level::Low, 1_500).into();
    count += 1;
    sent[count] = Code::END;
    count += 1;
    assert_eq!(count, SENT, "the frame has to fill the buffer exactly");

    delay::delay_ms(100);
    let decoded = rx
        .receive_dma_with(&dma_rx, received, rx_descriptors, 50_000, || {
            tx.send_dma(&dma_tx, &sent[..count], tx_descriptors, 50_000)
        })
        .unwrap();

    // The receiver is still counting the closing gap when the idle threshold
    // ends the reception, so it reports an end marker in its place: compare
    // everything before it and check that the tail really is that marker.
    let compared = decoded.min(count).saturating_sub(1);
    let mut mismatches = 0;
    for index in 0..compared {
        if received[index] != sent[index] {
            if mismatches < 4 {
                defmt::error!(
                    "code {}: sent {=u16:x}, decoded {=u16:x}",
                    index,
                    sent[index].raw(),
                    received[index].raw()
                );
            }
            mismatches += 1;
        }
    }

    defmt::info!(
        "rmt dma loopback: {} codes sent from {}-byte buffers, {} decoded, {} compared, {} mismatches",
        count,
        SENT * core::mem::size_of::<Code>(),
        decoded,
        compared,
        mismatches
    );
    assert_eq!(mismatches, 0, "RMT DMA loopback mismatch");
    assert!(
        received[compared].is_end(),
        "the reception has to end on a marker"
    );
    defmt::info!("rmt dma loopback -> ok");

    loop {
        delay::delay_ms(1_000);
    }
}
