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

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
use esp32p4_hal::time::Instant;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use core::sync::atomic::{AtomicU32, Ordering};

use esp32p4_hal::gpio::Pins;
use esp32p4_hal::interrupt::{self, CpuInterrupt};
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::time::Duration;
use esp32p4_hal::timer::{Direction, Timer00, TimerGroup0};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

const LED_PIN: u32 = 2;
/// CPU lines this example borrows out of the 32 the CLIC has.
const FAST_LINE: CpuInterrupt = CpuInterrupt::new(21);
const SLOW_LINE: CpuInterrupt = CpuInterrupt::new(22);

static FAST_TICKS: AtomicU32 = AtomicU32::new(0);
static SLOW_TICKS: AtomicU32 = AtomicU32::new(0);
/// Low half of the first timer's counter, sampled by the handler.
static FAST_COUNTER: AtomicU32 = AtomicU32::new(0);

fn fast_tick() {
    FAST_TICKS.fetch_add(1, Ordering::Relaxed);

    // The handler does not own the timer, but a stolen copy is enough.
    // SAFETY: `main` keeps this timer alive for the whole program.
    let timer = unsafe { Timer00::steal() };
    FAST_COUNTER.store(timer.counter() as u32, Ordering::Relaxed);
}

fn slow_tick() {
    SLOW_TICKS.fetch_add(1, Ordering::Relaxed);
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let mut led = pins.pin::<LED_PIN>();
    led.into_output();

    // The group owns the peripheral, both counters, the watchdog and the CPU
    // lines, like the hardware.
    let group = TimerGroup0::init(dp.TIMG0, &dp.HP_SYS_CLKRST);
    let fast = &group.timer0;
    let slow = &group.timer1;

    // 100 ms, counting up from zero: the counter restarts on every alarm, so
    // the time spent in the handler does not accumulate.
    group.enable_timer0_interrupt(FAST_LINE, fast_tick);
    fast.set_reload_value(0);
    fast.set_alarm(fast.ticks(Duration::from_millis(100)));
    fast.set_auto_reload(true);
    fast.start();

    // One second, counting down from the period to zero.
    slow.set_direction(Direction::Down);
    group.enable_timer1_interrupt(SLOW_LINE, slow_tick);
    slow.set_reload_value(slow.ticks(Duration::from_secs(1)));
    slow.set_alarm(0);
    slow.set_auto_reload(true);
    slow.start();

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
                    "fast {} at {} slow {} elapsed {}us",
                    fast_ticks,
                    FAST_COUNTER.load(Ordering::Relaxed),
                    SLOW_TICKS.load(Ordering::Relaxed),
                    started.elapsed().as_micros()
                );
            }
        }
    }
}
