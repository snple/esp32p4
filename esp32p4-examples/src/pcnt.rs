//! Counts LEDC pulses with PCNT, both on GPIO2.
//!
//! LEDC generates a 10 kHz square wave, so the counter should see ten rising
//! edges per millisecond. PCNT counts for 100 ms and the example compares the
//! result with 1000. It also sets the two thresholds inside that range and the
//! upper limit below it, so the run shows the threshold flags latching and the
//! counter wrapping.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::delay::Delay;
use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
use esp32p4_hal::gpio::{PinOps, Pins};
use esp32p4_hal::ledc::{ChannelConfig, ChannelIndex, ClockSource, Ledc, TimerConfig, TimerIndex};
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::pcnt::{self, ChannelConfig as PcntChannel, Pcnt, Trigger, UnitIndex};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Pad shared by the LEDC output and the PCNT input.
const PIN: u32 = 2;

/// LEDC frequency; one period is one PCNT edge.
const FREQUENCY: u32 = 10_000;

/// How long to count.
const WINDOW_MS: u32 = 100;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let delay = Delay::init(dp.SYSTIMER);

    let mut pin = pins.pin::<PIN>();
    pin.route_output_signal(OutputSignal::LEDC_LS_SIG_OUT_PAD_OUT1);
    pin.route_input_signal(InputSignal::PCNT_SIG_CH0_PAD_IN0);

    let ledc = Ledc::init(dp.LEDC, &dp.HP_SYS_CLKRST);
    ledc.configure_timer(
        TimerIndex::One,
        TimerConfig {
            clock: ClockSource::Xtal40m,
            frequency: FREQUENCY,
            resolution: 10,
        },
    )
    .unwrap();
    ledc.configure_channel(ChannelIndex::One, ChannelConfig::new(TimerIndex::One));
    ledc.set_duty(ChannelIndex::One, 512);

    let pcnt = Pcnt::init(dp.PCNT, &dp.HP_SYS_CLKRST);
    pcnt.configure_channel(
        UnitIndex::Zero,
        pcnt::ChannelIndex::Zero,
        PcntChannel::rising_edge(),
    );
    // Drop edges shorter than 10 APB cycles (a few hundred ns).
    pcnt.configure_filter(UnitIndex::Zero, Some(10)).unwrap();
    pcnt.set_limits(UnitIndex::Zero, 0, 800);
    pcnt.set_thresholds(UnitIndex::Zero, 250, 750);
    pcnt.enable(
        UnitIndex::Zero,
        Trigger {
            high_limit: true,
            low_limit: true,
            threshold0: true,
            threshold1: true,
            zero: true,
        },
    );
    pcnt.clear(UnitIndex::Zero);
    pcnt.clear_latched(UnitIndex::Zero);

    ledc.start(TimerIndex::One);
    delay.delay_ms(WINDOW_MS);
    ledc.stop(TimerIndex::One);

    let count = pcnt.count(UnitIndex::Zero);
    let latched = pcnt.latched(UnitIndex::Zero);

    defmt::info!(
        "pcnt: {=u32} edges in {=u32} ms ({=u32} kHz), counter {=i16}",
        FREQUENCY * WINDOW_MS / 1_000,
        WINDOW_MS,
        FREQUENCY / 1_000,
        count
    );
    defmt::info!(
        "pcnt flags: high {} low {} thres0 {} thres1 {} zero {} pending {}",
        latched.high_limit,
        latched.low_limit,
        latched.threshold0,
        latched.threshold1,
        latched.zero,
        pcnt.pending(UnitIndex::Zero)
    );
    // 1000 edges with an 800-step wrap leave ~200; the window boundary can add
    // or drop one edge.
    assert!(
        (195..=205).contains(&count),
        "PCNT wrap count out of range: {}",
        count
    );

    pcnt.clear(UnitIndex::Zero);
    pcnt.clear_latched(UnitIndex::Zero);
    pcnt.pause(UnitIndex::Zero, true);
    ledc.start(TimerIndex::One);
    delay.delay_ms(10);
    let paused = pcnt.count(UnitIndex::Zero);
    pcnt.pause(UnitIndex::Zero, false);
    delay.delay_ms(10);
    let running = pcnt.count(UnitIndex::Zero);
    ledc.stop(TimerIndex::One);
    defmt::info!(
        "pcnt pause: {=i16} while paused, {=i16} after resuming",
        paused,
        running
    );
    assert_eq!(paused, 0, "PCNT counted while paused");
    assert!(running > 50, "PCNT did not resume");

    loop {
        delay.delay_ms(1_000);
    }
}
