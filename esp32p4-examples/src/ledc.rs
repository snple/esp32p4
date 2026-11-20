//! Sweeps one LEDC channel through several duties and a fade.
//!
//! The PWM signal goes out on `GPIO2`. The same pad's input buffer stays
//! enabled, so the example can read the waveform back with no extra wiring: it
//! samples the pin in a tight loop and compares the measured high ratio with
//! the duty it programmed. A 1 kHz period is 1000 µs and one sample costs about
//! a microsecond, so a few thousand samples average out to within a percent.
//!
//! A linear fade then ramps the duty from zero to full over 100 periods, which
//! the example checks through `fade_done` and the duty readback.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::delay::Delay;
use esp32p4_hal::gpio::signal::OutputSignal;
use esp32p4_hal::gpio::{Pin, PinOps, Pins};
use esp32p4_hal::ledc::{
    ChannelConfig, ChannelIndex, ClockSource, FadeRange, Ledc, Level, TimerConfig, TimerIndex,
};
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// PWM output pad.
const PWM_PIN: u32 = 2;

/// 1 kHz at 12-bit resolution: divider = 40 MHz / (1 kHz * 4096).
const FREQUENCY: u32 = 1_000;
const RESOLUTION: u32 = 12;

/// Samples taken per duty, which is also the permille divisor.
const SAMPLES: u32 = 20_000;

/// Duties to measure, in percent.
const DUTIES: [u32; 5] = [0, 25, 50, 75, 100];

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let delay = Delay::init(dp.SYSTIMER);

    let ledc = Ledc::init(dp.LEDC, &dp.HP_SYS_CLKRST);
    ledc.set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::Xtal40m);
    ledc.configure_timer(
        TimerIndex::Zero,
        TimerConfig {
            clock: ClockSource::Xtal40m,
            frequency: FREQUENCY,
            resolution: RESOLUTION,
        },
    )
    .unwrap();

    let mut pwm = pins.pin::<PWM_PIN>();
    pwm.route_output_signal(OutputSignal::LEDC_LS_SIG_OUT_PAD_OUT0);
    // Keep the input path alive so the same pad can be sampled back.
    pwm.set_input_buffer_enabled(true);

    ledc.configure_channel(ChannelIndex::Zero, ChannelConfig::new(TimerIndex::Zero));
    ledc.start(TimerIndex::Zero);

    let max = ledc.max_duty(TimerIndex::Zero);
    defmt::info!(
        "timer: {=u32} Hz, {=u32} bit, max duty {=u32}",
        FREQUENCY,
        RESOLUTION,
        max
    );

    for percent in DUTIES {
        ledc.set_duty(ChannelIndex::Zero, max * percent / 100);
        // The new duty lands on the next overflow, so let a few periods pass.
        delay.delay_ms(5);

        let high = sample_high_ratio(&pwm, SAMPLES);
        defmt::info!(
            "duty {=u32}%: readback {=u32}, pin high {=u32}.{=u32}%",
            percent,
            ledc.duty(ChannelIndex::Zero),
            high * 100 / SAMPLES,
            (high * 10_000 / SAMPLES) % 100
        );
    }

    // Ramp from zero to full: 100 steps of one period each, 100 ms in total.
    ledc.set_duty(ChannelIndex::Zero, 0);
    ledc.set_fade_range(
        ChannelIndex::Zero,
        0,
        FadeRange {
            increasing: true,
            cycles: 1,
            scale: (max / 100) as u16,
            steps: 100,
        },
    )
    .unwrap();
    ledc.set_fade_range_count(ChannelIndex::Zero, 1).unwrap();
    ledc.clear_fade_done(ChannelIndex::Zero);
    ledc.start_fade(ChannelIndex::Zero);

    delay.delay_ms(10);
    let mid = ledc.duty(ChannelIndex::Zero);

    // 100 steps of one period each: 100 ms, with a wide margin.
    let mut waited = 0;
    while !ledc.fade_done(ChannelIndex::Zero) && waited < 2_000 {
        delay.delay_ms(1);
        waited += 1;
    }
    let end = ledc.duty(ChannelIndex::Zero);
    let done = ledc.fade_done(ChannelIndex::Zero);
    ledc.clear_fade_done(ChannelIndex::Zero);

    defmt::info!(
        "fade: done {=bool}, {=u32} after 10 ms, {=u32} at the end (max {=u32})",
        done,
        mid,
        end,
        max
    );

    // Disabling the output drives the idle level instead of the waveform.
    ledc.set_idle_level(ChannelIndex::Zero, Level::Low);
    ledc.enable(ChannelIndex::Zero, false);
    delay.delay_ms(2);
    defmt::info!("idle: pin high {=bool} (expected false)", pwm.get());

    loop {
        delay.delay_ms(1_000);
    }
}

/// Samples the pad in a tight loop and returns how many reads were high.
fn sample_high_ratio(pin: &Pin<PWM_PIN>, samples: u32) -> u32 {
    let mut high = 0;
    for _ in 0..samples {
        if pin.get() {
            high += 1;
        }
    }
    high
}
