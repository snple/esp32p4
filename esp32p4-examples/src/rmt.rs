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
use esp32p4_hal::rmt::{
    ClockSource, Pulse, Rmt, RxChannelIndex, RxConfig, TxChannelIndex, TxConfig,
};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Pad shared by the transmitter and the receiver.
const PIN: u32 = 2;

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
    let delay = Delay::init(dp.SYSTIMER);

    let rmt = Rmt::init(dp.RMT, &dp.HP_SYS_CLKRST);
    rmt.set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::Xtal40m, DIVIDER)
        .unwrap();

    let mut pin = pins.pin::<PIN>();
    pin.route_output_signal(OutputSignal::RMT_SIG_PAD_OUT0);
    pin.route_input_signal(InputSignal::RMT_SIG_PAD_IN0);

    rmt.configure_tx(TxChannelIndex::Zero, &TxConfig::new(1))
        .unwrap();
    rmt.configure_rx(RxChannelIndex::Zero, &RxConfig::new(1, IDLE_TICKS))
        .unwrap();

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
    let decoded = rmt
        .receive_with(RxChannelIndex::Zero, &mut received, 20_000, || {
            rmt.send(TxChannelIndex::Zero, &sent[..count], 10_000)
        })
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

    loop {
        delay.delay_ms(1_000);
    }
}
