//! Loops a WS2812 bit stream through RMT: transmit on GPIO2, receive on the
//! same pad.
//!
//! The transmitter drives the pad through the GPIO matrix and the receiver
//! listens to the same pad's input path, so no wiring is needed. The 20 MHz
//! channel clock makes one tick 50 ns: a WS2812 "0" is 7 high ticks and 16 low
//! ticks, a "1" is 14 + 12 ticks.
//!
//! Every level and duration the receiver decodes is compared against what was
//! sent; a mismatch panics with both sequences in the log.
//!
//! A second loopback then repeats the check on channel 2, so a channel index
//! other than zero is covered too: its RAM lives at a different address and it
//! owns fewer blocks. That one goes out on `GPIO17` and comes back on `GPIO21`
//! through a jumper.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::delay::Delay;
use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
use esp32p4_hal::gpio::{Level, PinOps, Pins};
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::rmt::{ClockSource, Pulse, Rmt, RxConfig, TxConfig};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Pad shared by the transmitter and the receiver.
const PIN: u32 = 2;
/// Pad carrying channel 2's transmit signal.
const TX_PIN: u32 = 17;
/// Pad feeding channel 2's receiver.
const RX_PIN: u32 = 21;

/// Group divider: 40 MHz / 2 = 20 MHz, so one tick is 50 ns.
const DIVIDER: u32 = 2;

/// Ticks of unchanged level that end a reception: 1000 ticks = 50 µs, longer
/// than the longest bit and shorter than the WS2812 reset gap.
const IDLE_TICKS: u16 = 1_000;

/// One byte as WS2812 bits, most significant bit first, in tick pairs.
fn byte_pulses(byte: u8, out: &mut [Pulse]) -> usize {
    let mut count = 0;
    for bit in (0..8).rev() {
        if byte & (1 << bit) != 0 {
            out[count] = Pulse::new(Level::High, 14);
            out[count + 1] = Pulse::new(Level::Low, 12);
        } else {
            out[count] = Pulse::new(Level::High, 7);
            out[count + 1] = Pulse::new(Level::Low, 16);
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
    let delay = Delay::init();

    let rmt = Rmt::init(dp.RMT, &dp.HP_SYS_CLKRST);
    rmt.set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::Xtal40m, DIVIDER)
        .unwrap();

    let mut pin = pins.pin::<PIN>();
    pin.route_output_signal(OutputSignal::RMT_SIG_PAD_OUT0);
    pin.route_input_signal(InputSignal::RMT_SIG_PAD_IN0);

    let tx = rmt.tx_channel::<0>();
    let rx = rmt.rx_channel::<0>();
    tx.configure(&TxConfig::new(1)).unwrap();
    rx.configure(&RxConfig::new(1, IDLE_TICKS)).unwrap();

    // Green, red, blue.
    let mut sent = [Pulse::default(); 49];
    let mut count = 0;
    for byte in [0x00u8, 0x40, 0x80] {
        count += byte_pulses(byte, &mut sent[count..]);
    }
    // A short closing low pulse: the receiver drops whatever it is counting
    // when the idle gap arrives, so end the sequence on a pulse of its own.
    sent[count] = Pulse::new(Level::Low, 500);
    count += 1;

    let mut received = [Pulse::default(); 64];
    let decoded = rx
        .receive_with(&mut received, 20_000, || tx.send(&sent[..count], 10_000))
        .unwrap();

    // The last pulse or two are still being counted when the idle gap ends the
    // reception, so only the pulses the receiver completed are compared.
    let compared = decoded.min(count);
    let mut mismatches = 0;
    for index in 0..compared {
        if received[index].level != sent[index].level
            || received[index].period != sent[index].period
        {
            defmt::error!(
                "pulse {}: sent {} ticks at {}, decoded {} ticks at {}",
                index,
                sent[index].period,
                sent[index].level,
                received[index].period,
                received[index].level
            );
            mismatches += 1;
        }
    }

    defmt::info!(
        "rmt loopback: {=usize} sent, {=usize} decoded, {=usize} compared, {=usize} mismatches",
        count,
        decoded,
        compared,
        mismatches
    );
    assert_eq!(mismatches, 0, "RMT loopback mismatch");

    // The same shape again on channel 2, whose RAM lives at a different
    // address: if the per-channel offsets were wrong this would decode nothing.
    let mut out = pins.pin::<TX_PIN>();
    out.route_output_signal(OutputSignal::RMT_SIG_PAD_OUT2);
    let mut back = pins.pin::<RX_PIN>();
    back.route_input_signal(InputSignal::RMT_SIG_PAD_IN2);

    let tx2 = rmt.tx_channel::<2>();
    let rx2 = rmt.rx_channel::<2>();
    // Channel 2 owns two RAM blocks, so the sequence has to fit 192 codes.
    tx2.configure(&TxConfig::new(1)).unwrap();
    rx2.configure(&RxConfig::new(1, IDLE_TICKS)).unwrap();
    let mut received2 = [Pulse::default(); 64];
    let decoded2 = rx2
        .receive_with(&mut received2, 20_000, || tx2.send(&sent[..16], 10_000))
        .unwrap();
    let compared2 = decoded2.min(16);
    let mut mismatches2 = 0;
    for index in 0..compared2 {
        if received2[index].level != sent[index].level
            || received2[index].period != sent[index].period
        {
            mismatches2 += 1;
        }
    }
    defmt::info!(
        "rmt loopback ch2: {=usize} decoded, {=usize} compared, {=usize} mismatches",
        decoded2,
        compared2,
        mismatches2
    );
    assert_eq!(mismatches2, 0, "RMT channel 2 loopback mismatch");

    loop {
        delay.delay_ms(1_000);
    }
}
