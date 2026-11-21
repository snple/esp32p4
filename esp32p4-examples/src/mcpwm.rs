//! Drives MCPWM0 on GPIO2 and checks the result on the same pad.
//!
//! Three layers are checked separately, so a failure says where to look:
//!
//! 1. the counter runs and covers the programmed period,
//! 2. a forced level reaches the pad (read back through `GPIO_IN`),
//! 3. the generator's timer events produce a waveform — the pad is fed into the
//!    RMT receiver, which measures the high and low time in 40 MHz ticks.
//!
//! The example prints `ok`/`FAIL` per layer instead of panicking, so a single
//! broken layer still reports the others, and repeats the whole set once a
//! second so the log shows up whenever a monitor attaches.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::delay::Delay;
use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
use esp32p4_hal::gpio::{Level, Pin, PinOps, Pins};
use esp32p4_hal::main;
use esp32p4_hal::mcpwm::{
    ClockSource, DeadTime, GeneratorConfig, Mcpwm0, OperatorOutput, TimerConfig,
};
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::rmt::{ClockSource as RmtClock, Pulse, Rmt, RxChannelIndex, RxConfig};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Pad shared by the MCPWM output and the RMT receiver.
const PIN: u32 = 2;

/// PWM frequency.
const FREQUENCY: u32 = 20_000;

/// Dead time used by the complementary check, in 40 MHz ticks. The two edges
/// differ on purpose: the measured high time tells which delay was applied to
/// the rising edge.
const DEAD_RISING: u16 = 10;
const DEAD_FALLING: u16 = 30;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let delay = Delay::init(dp.SYSTIMER);

    let mut pin = pins.pin::<PIN>();
    pin.route_output_signal(OutputSignal::PWM0_CH0_A_PAD_OUT);
    pin.route_input_signal(InputSignal::RMT_SIG_PAD_IN0);

    let mcpwm = Mcpwm0::init(dp.MCPWM0, &dp.HP_SYS_CLKRST);
    mcpwm
        .set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::Xtal40m, 1)
        .unwrap();
    let timer = mcpwm.timer::<0>();
    timer
        .configure(&dp.HP_SYS_CLKRST, TimerConfig::new(FREQUENCY))
        .unwrap();

    // The receiver runs from the same 40 MHz as the timer, so its ticks can be
    // compared with the timer's directly.
    let rmt = Rmt::init(dp.RMT, &dp.HP_SYS_CLKRST);
    rmt.set_clock_source(&dp.HP_SYS_CLKRST, RmtClock::Xtal40m, 1)
        .unwrap();
    // 4000 ticks = 100 µs, above the longest level of a 20 kHz period.
    rmt.configure_rx(RxChannelIndex::Zero, &RxConfig::new(1, 4_000))
        .unwrap();

    let period = u32::from(timer.period());
    let clock = mcpwm.group_clock_hz(&dp.HP_SYS_CLKRST).unwrap();
    defmt::info!(
        "mcpwm: {=u32} Hz from {=u32} Hz, period {=u32} ticks",
        FREQUENCY,
        clock,
        period
    );

    // The checks are cheap, so they run once a second: the log then shows up
    // whenever the monitor attaches, without needing a reset.
    loop {
        check_counter(&mcpwm, &delay, period);
        check_forced_level(&mcpwm, &pin, &delay, (period / 2) as u16);

        for percent in [25u32, 50, 75] {
            let duty = period * percent / 100;
            check_waveform(&mcpwm, &rmt, &delay, percent, duty as u16);
        }

        check_complementary(&mcpwm, &rmt, &delay, period);

        delay.delay_ms(1_000);
    }
}

/// Layer 1: the timer counts and reaches the programmed period.
fn check_counter(mcpwm: &Mcpwm0, delay: &Delay, period: u32) {
    let timer = mcpwm.timer::<0>();
    timer.start();
    let mut low = u16::MAX;
    let mut high = 0;
    // One period is 50 µs, so 20 samples 5 µs apart cover several of them.
    for _ in 0..20 {
        delay.delay_us(5);
        let value = timer.counter();
        low = low.min(value);
        high = high.max(value);
    }
    timer.stop();

    let ok = high > low && u32::from(high) <= period;
    defmt::info!(
        "counter: saw {=u16}..{=u16} of {=u32} -> {}",
        low,
        high,
        period,
        if ok { "ok" } else { "FAIL" }
    );
}

/// Layer 2: the pad really follows the generator, level for level.
///
/// The dead-time generator is bypassed first: its delays are counted in timer
/// ticks, so with the timer stopped a delayed edge never completes and a forced
/// level would not show up on the pad.
fn check_forced_level(mcpwm: &Mcpwm0, pin: &Pin<PIN>, delay: &Delay, duty: u16) {
    let timer = mcpwm.timer::<0>();
    let operator = mcpwm.operator::<0>();
    operator.configure(&timer, GeneratorConfig::edge_aligned(duty, 0));
    operator.force(OperatorOutput::A, Level::High);
    let high = count_high(pin, delay);
    operator.force(OperatorOutput::A, Level::Low);
    let low = count_high(pin, delay);
    operator.release_force(OperatorOutput::A);

    let ok = high == 16 && low == 0;
    defmt::info!(
        "pad: forced high read {=u32}/16, forced low read {=u32}/16 -> {}",
        high,
        low,
        if ok { "ok" } else { "FAIL" }
    );
}

/// How many of 16 reads, spaced 1 ms apart, see the pad high. Sampling over a
/// window instead of once makes the check immune to where in the PWM period the
/// read lands.
fn count_high(pin: &Pin<PIN>, delay: &Delay) -> u32 {
    let mut high = 0;
    for _ in 0..16 {
        delay.delay_ms(1);
        if pin.get() {
            high += 1;
        }
    }
    high
}

/// Layer 3: timer events shape the pad. The RMT receiver reports the level
/// times, which are compared with the comparator value.
fn check_waveform(mcpwm: &Mcpwm0, rmt: &Rmt, delay: &Delay, percent: u32, duty: u16) {
    let timer = mcpwm.timer::<0>();
    mcpwm
        .operator::<0>()
        .configure(&timer, GeneratorConfig::edge_aligned(duty, 0));
    timer.start();

    let (high, low) = measure(rmt, delay);

    timer.stop();

    // The timer and the RMT receiver both run from the 40 MHz crystal, so the
    // measured high time should be within a few ticks of the comparator.
    let ok = high.abs_diff(u32::from(duty)) <= 4 && low > 0;
    defmt::info!(
        "duty {=u32}%: comparator {=u16}, measured high {=u32}, low {=u32} -> {}",
        percent,
        duty,
        high,
        low,
        if ok { "ok" } else { "FAIL" }
    );
}

/// Layer 4: the dead-time generator keeps the complementary pair apart.
fn check_complementary(mcpwm: &Mcpwm0, rmt: &Rmt, delay: &Delay, period: u32) {
    let duty = (period / 2) as u16;
    let timer = mcpwm.timer::<0>();
    mcpwm.operator::<0>().configure(
        &timer,
        GeneratorConfig::complementary(duty, DeadTime::active_high(DEAD_RISING, DEAD_FALLING)),
    );
    timer.start();

    let (high, low) = measure(rmt, delay);

    timer.stop();

    // Output A only pays the rising-edge delay, so its high time is the
    // comparator value minus the RED ticks; the falling delay shows up on
    // output B instead.
    let expected = u32::from(duty).saturating_sub(u32::from(DEAD_RISING));
    let ok = high.abs_diff(expected) <= 4 && low > u32::from(DEAD_FALLING);
    defmt::info!(
        "complementary: comparator {=u16} - RED, measured high {=u32}, low {=u32} -> {}",
        duty,
        high,
        low,
        if ok { "ok" } else { "FAIL" }
    );
}

/// Average high and low time of the pulses the RMT receiver sees, in 40 MHz
/// ticks.
fn measure(rmt: &Rmt, delay: &Delay) -> (u32, u32) {
    let mut pulses = [Pulse::default(); 16];
    rmt.clear_rx_done(RxChannelIndex::Zero);
    rmt.start_rx(RxChannelIndex::Zero);
    delay.delay_ms(3);
    rmt.stop_rx(RxChannelIndex::Zero);
    let count = rmt.read_pulses(RxChannelIndex::Zero, &mut pulses);
    // The first level is whatever the receiver caught mid-pulse, so leave it
    // out of the average.
    let pulses = if count > 2 {
        &pulses[1..count]
    } else {
        &pulses[..count]
    };

    let mut high_sum = 0u32;
    let mut high_count = 0u32;
    let mut low_sum = 0u32;
    let mut low_count = 0u32;
    for pulse in pulses.iter() {
        match pulse.level {
            Level::High => {
                high_sum += u32::from(pulse.period);
                high_count += 1;
            }
            Level::Low => {
                low_sum += u32::from(pulse.period);
                low_count += 1;
            }
        }
    }

    (
        high_sum.checked_div(high_count).unwrap_or(0),
        low_sum.checked_div(low_count).unwrap_or(0),
    )
}
