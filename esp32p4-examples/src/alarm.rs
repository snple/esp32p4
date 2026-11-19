//! Wake up on system timer comparators.
//!
//! The first comparator runs a 250 ms periodic alarm that toggles the LED, the
//! second one is armed once for 5 s and polled; the main loop sleeps until a
//! deadline instead of for a duration.
//!
//! ```text
//! cargo run --bin alarm
//! ```

#![no_std]
#![no_main]

// `--features defmt` turns on the HAL's log statements for every binary, so
// each of them needs a logger and a timestamp source of its own: `defmt-rtt`
// installs the RTT one, `defmt-usb` the USB Serial/JTAG one.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

use core::sync::atomic::{AtomicU32, Ordering};

use esp32p4::gpio::Pins;
use esp32p4::interrupt::{self, CpuInterrupt};
use esp32p4::main;
use esp32p4::pac::Peripherals;
use esp32p4::systimer::Systimer;
use esp32p4::time::{Duration, Instant};

const LED_PIN: u32 = 2;
const ALARM_LINE: CpuInterrupt = CpuInterrupt::new(21);

static TICKS: AtomicU32 = AtomicU32::new(0);

fn tick() {
    TICKS.fetch_add(1, Ordering::Relaxed);
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let mut led = pins.pin::<LED_PIN>();
    led.into_output();
    // The timer owns the peripheral and the alarms' CPU lines; the delay
    // driver shares comparator 0 through its own handle.
    let systimer = Systimer::init(dp.SYSTIMER);
    let delay = systimer.delay();

    // COMP1: an interrupt every 250 ms. The hardware re-arms the comparator,
    // so the handler only has to acknowledge it (which the shim does).
    let periodic = &systimer.alarm1;
    systimer.enable_alarm1_interrupt(ALARM_LINE, tick);
    periodic.set_period(Duration::from_millis(250));

    // COMP2: a single deadline 5 s out, polled from the main loop.
    let one_shot = &systimer.alarm2;
    one_shot.set_target(Instant::now() + Duration::from_secs(5));

    unsafe { interrupt::enable() };

    let mut seen = 0;
    loop {
        let ticks = TICKS.load(Ordering::Relaxed);
        if ticks != seen {
            seen = ticks;
            led.toggle();

            #[cfg(feature = "defmt")]
            defmt::info!("alarm {} every {}us", ticks, periodic.period().as_micros());
        }

        if one_shot.pending() {
            one_shot.clear();

            #[cfg(feature = "defmt")]
            defmt::info!("one-shot fired at {}us", Instant::now().as_ticks() / 16);
        }

        // Park the core until the next check; any interrupt wakes it early.
        delay.sleep_until(Instant::now() + Duration::from_millis(5));
    }
}
