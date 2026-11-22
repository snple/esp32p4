//! Starts the temperature sensor from the ETM and lets it drive a pin.
//!
//! The sensor's ETM side is smaller than the ADC's: one task that takes a single
//! sample and one event that fires when a monitored reading leaves its window.
//! Both are exercised here, with the system timer as the event source and GPIO2
//! as the sink, so nothing needs wiring:
//!
//! 1. a window the current reading already violates is armed, but the
//!    hardware's own sampling stays off; `COMP0` then runs
//!    `TMPSNSR_TASK_START_SAMPLE`, and the over-limit interrupt latches — one
//!    sample was taken without the CPU and without `SAMPLE_EN`;
//! 2. `TMPSNSR_EVT_OVER_LIMIT` drives `GPIO_TASK_CH2_SET`, so the pin rises
//!    while continuous monitoring runs and the CPU only watches;
//! 3. the reading itself still converts to a sane temperature afterwards.
//!
//! `TMPSNSR_TASK_STOP_SAMPLE` is deliberately absent: on this chip it neither
//! clears `SAMPLE_EN` nor stops continuous monitoring, so there is nothing to
//! check. See [`tsens::Task`].
//!
//! The window is derived from the reading rather than hard-coded, because the
//! raw value depends on the die and on the measurement range.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::clock::ClockTree;
use esp32p4_hal::delay;
use esp32p4_hal::efuse::Efuse;
use esp32p4_hal::etm::Etm;
use esp32p4_hal::gpio::{EtmTask, Pins};
use esp32p4_hal::machine_timer::MachineTimer;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::systimer::{self, Systimer};
use esp32p4_hal::time::Duration;
use esp32p4_hal::tsens::{self, TemperatureSensor};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Pin the over-limit event drives through the matrix's task side.
const FLAG_PIN: u32 = 2;
/// Cycles of the low-power clock between the monitor's own samples.
const SAMPLE_RATE: u16 = 200;
/// How long the timer waits before firing a task.
const DELAY_US: u64 = 2_000;
/// Longest a trip is given, in microseconds.
const TRIP_TIMEOUT_US: u64 = 100_000;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    let efuse = Efuse::init(dp.EFUSE);
    let clocks = ClockTree::init_from_efuse(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST, &efuse);
    let _timer = MachineTimer::init(clocks.cpu_hz());
    let (clocks, _lp_aon) = clocks.release();

    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &clocks);
    let etm = Etm::init(dp.SOC_ETM, &clocks);
    let systimer = Systimer::init(dp.SYSTIMER);
    let mut sensor = TemperatureSensor::init(dp.LP_TSENS, &dp.LP_PERI);

    systimer.unit0.start();
    systimer.enable_events(true);
    sensor.set_sample_rate(SAMPLE_RATE);
    sensor.enable_sampling(false);

    // A window the current reading falls outside of, so the very next sample
    // trips the monitor.
    let raw = sensor.raw();
    let (low, high) = tripping_window(raw);
    sensor.set_thresholds(low, high);
    sensor.set_monitoring_delta(false);
    sensor.enable_monitoring(true);
    sensor.clear_interrupt();

    // ---- 1. one sample, taken by the timer --------------------------------
    let mut sample_channel = etm.free_channel().expect("no free ETM channel");
    sample_channel.connect(systimer::Event::Alarm0, tsens::Task::StartSample);
    sample_channel.enable(true);
    systimer.alarm0.set_target(
        systimer
            .unit0
            .now()
            .saturating_add(Duration::from_micros(DELAY_US)),
    );
    let sampled = wait_for(|| sensor.interrupt_pending());
    systimer.alarm0.clear();
    sample_channel.enable(false);
    let continuous = sensor.sampling();
    defmt::info!(
        "start task: over-limit latched {}, continuous sampling {} (window {}..={}, reading {})",
        sampled,
        continuous,
        low,
        high,
        raw
    );
    assert!(sampled, "the start task did not take a sample");
    assert!(
        !continuous,
        "the start task must not switch on continuous sampling"
    );
    sensor.clear_interrupt();

    // ---- 2. the event drives a pin ----------------------------------------
    let mut flag = pins.pin::<FLAG_PIN>();
    flag.init_pad();
    flag.enable_input();
    flag.set_direction_out();
    flag.set_low();
    flag.bind_etm_task(FLAG_PIN as u8);

    let mut flag_channel = etm.free_channel().expect("no free ETM channel");
    flag_channel.connect(tsens::Event::OverLimit, EtmTask::Set(FLAG_PIN as u8));
    flag_channel.enable(true);
    sensor.enable_sampling(true);
    let tripped = wait_for(|| flag.get());
    flag_channel.enable(false);
    flag.enable_etm_task(false);
    sensor.enable_sampling(false);
    sensor.enable_monitoring(false);
    sensor.clear_interrupt();
    defmt::info!("over-limit event: flag {}", tripped);
    assert!(tripped, "the over-limit event did not reach the pin");

    // ---- 3. the sensor still reads ----------------------------------------
    let millidegrees = sensor.update();
    defmt::info!(
        "reading: {}.{:03} °C, range {:?}",
        millidegrees.div_euclid(1_000),
        millidegrees.rem_euclid(1_000),
        sensor.range()
    );
    defmt::info!("temperature sensor ETM self-check passed");

    loop {
        delay::delay_ms(1_000);
    }
}

/// A threshold window the reading `raw` already violates, in raw sensor units.
fn tripping_window(raw: u8) -> (u8, u8) {
    if raw >= 5 {
        (0, raw - 5)
    } else {
        (raw + 5, 255)
    }
}

/// Polls `check` for [`TRIP_TIMEOUT_US`].
fn wait_for(check: impl Fn() -> bool) -> bool {
    let deadline = delay::micros() + TRIP_TIMEOUT_US;
    while delay::micros() < deadline {
        if check() {
            return true;
        }
        core::hint::spin_loop();
    }
    false
}
