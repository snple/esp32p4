//! Take both general-purpose timers of `TIMG0` and run them from alarms.
//!
//! The first timer fires an interrupt every 100 ms and the second one every
//! second while counting down. The handlers only bump a counter; the main loop
//! does the visible work, which is what a real handler should look like.
//!
//! ```text
//! cargo run --bin timer
//! ```

#![no_std]
#![no_main]

// `--features defmt` turns on the HAL's log statements for every binary, so
// each of them needs a logger and a timestamp source of its own.
#[cfg(feature = "defmt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
use esp32p4::time::Instant;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

use core::sync::atomic::{AtomicU32, Ordering};

use esp32p4::gpio::Pins;
use esp32p4::interrupt::{self, CpuInterrupt};
use esp32p4::main;
use esp32p4::pac::Peripherals;
use esp32p4::time::Duration;
use esp32p4::timer::{Direction, TimerGroup0};

const LED_PIN: u32 = 2;
/// CPU lines this example borrows out of the 32 the CLIC has.
const FAST_LINE: CpuInterrupt = CpuInterrupt::new(21);
const SLOW_LINE: CpuInterrupt = CpuInterrupt::new(22);

static FAST_TICKS: AtomicU32 = AtomicU32::new(0);
static SLOW_TICKS: AtomicU32 = AtomicU32::new(0);

fn fast_tick() {
    FAST_TICKS.fetch_add(1, Ordering::Relaxed);
}

fn slow_tick() {
    SLOW_TICKS.fetch_add(1, Ordering::Relaxed);
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let mut led = pins.pin::<LED_PIN>();
    led.into_output();

    let (mut fast, mut slow) = TimerGroup0::new(dp.TIMG0, &dp.HP_SYS_CLKRST).split();

    // 100 ms, counting up from zero: the counter restarts on every alarm, so
    // the time spent in the handler does not accumulate.
    fast.enable_interrupt(FAST_LINE, fast_tick);
    fast.start_periodic(Duration::from_millis(100));

    // One second, counting down from the period to zero.
    slow.set_direction(Direction::Down);
    slow.enable_interrupt(SLOW_LINE, slow_tick);
    slow.start_periodic(Duration::from_secs(1));

    // The handlers only run once global interrupts are enabled.
    // SAFETY: both shims acknowledge their timer before doing anything else.
    unsafe { interrupt::enable() };

    #[cfg(feature = "defmt")]
    let started = Instant::now();
    let mut fast_seen = 0;

    loop {
        let fast_ticks = FAST_TICKS.load(Ordering::Relaxed);
        if fast_ticks != fast_seen {
            fast_seen = fast_ticks;

            // Blink every fifth 100 ms tick, i.e. every 500 ms.
            if fast_ticks.is_multiple_of(5) {
                led.toggle();

                #[cfg(feature = "defmt")]
                defmt::info!(
                    "fast {} slow {} elapsed {}us",
                    fast_ticks,
                    SLOW_TICKS.load(Ordering::Relaxed),
                    started.elapsed().as_micros()
                );
            }
        }
    }
}
