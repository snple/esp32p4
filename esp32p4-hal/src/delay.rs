//! Blocking delays using the system timer.
//!
//! The counter and the tick rate live in [`crate::time`]; this module only adds
//! comparator 0 on top of them, which lets the core park in `wfi` instead of
//! spinning through a delay.

use core::sync::atomic::{AtomicBool, Ordering};

use esp32p4_pac::SYSTIMER;

use crate::interrupt::{self, Interrupt, Level};
use crate::time::{self, TICKS_PER_US};

/// Set by the target-0 interrupt handler, consumed by [`Delay::sleep_us`].
static TIMER_FIRED: AtomicBool = AtomicBool::new(false);

/// Whether the target-0 interrupt has been routed.
///
/// Every [`Delay`] instance shares comparator 0, so the routing must not run
/// more than once.
static INTERRUPT_ROUTED: AtomicBool = AtomicBool::new(false);

/// Target-0 interrupt handler.
///
/// The comparator is single-shot: acknowledge it here, and let the caller's
/// `wfi` loop notice the flag. `SYSTIMER` is reached by pointer because an
/// interrupt handler cannot borrow the driver.
fn target0_isr() {
    let systimer = unsafe { &*SYSTIMER::PTR };
    systimer.int_clr.trigger_target0_int_clr();
    TIMER_FIRED.store(true, Ordering::Release);
}

/// Blocking delay driver backed by `SYSTIMER` unit 0.
pub struct Delay {
    systimer: SYSTIMER,
}

/// Sleep for `ms` milliseconds, waking early on any interrupt.
///
/// Driver-friendly wrapper around [`Delay::sleep_ms`] for code that does not
/// own a [`Delay`] handle. [`Delay::init`] must have run at least once so the
/// timer is counting and its interrupt is routed.
#[inline]
pub fn sleep_ms(ms: u32) -> bool {
    Delay {
        systimer: unsafe { SYSTIMER::steal() },
    }
    .sleep_ms(ms)
}

/// Microseconds since boot, readable without owning the timer.
///
/// Handy as a `defmt` timestamp: `defmt::timestamp!("{=u64:us}", micros());`
#[inline]
pub fn micros() -> u64 {
    time::Instant::now().as_ticks() / TICKS_PER_US
}

impl Delay {
    /// Create a delay driver.
    ///
    /// Starts the counter if that has not happened yet and routes comparator 0
    /// to its interrupt line.
    #[inline]
    pub fn init(systimer: SYSTIMER) -> Self {
        time::start();

        // Route target 0 to its CPU interrupt line so `sleep_us` can park the
        // core instead of spinning. The line is unmasked but only fires once
        // `sleep_us` arms a comparator, and interrupts stay globally off until
        // the application enables them.
        if !INTERRUPT_ROUTED.swap(true, Ordering::AcqRel) {
            // SAFETY: the handler only clears the comparator and sets a flag,
            // and it cannot fire before a comparator is armed.
            let handler = unsafe {
                interrupt::claim(
                    Interrupt::SYSTIMER_TARGET0,
                    interrupt::lines::SYSTIMER_TARGET0,
                    Level::LOW,
                    target0_isr,
                )
            };
            handler.enable();
            // The delay driver is created on every `micros()` call, so the line
            // belongs to the program rather than to one instance.
            handler.leak();
        }

        Self { systimer }
    }

    /// Read the current 52-bit counter value.
    #[inline]
    fn ticks(&self) -> u64 {
        time::read_ticks()
    }

    /// Microseconds since boot.
    #[inline]
    pub fn now_us(&self) -> u64 {
        self.ticks() / TICKS_PER_US
    }

    /// Wait for at least `us` microseconds.
    #[inline]
    pub fn delay_us(&self, us: u32) {
        let start = self.ticks();
        let target = us as u64 * TICKS_PER_US;
        while self.ticks().wrapping_sub(start) < target {
            core::hint::spin_loop();
        }
    }

    /// Wait for at least `ms` milliseconds.
    #[inline]
    pub fn delay_ms(&self, ms: u32) {
        let start = self.ticks();
        let target = ms as u64 * 1000 * TICKS_PER_US;
        while self.ticks().wrapping_sub(start) < target {
            core::hint::spin_loop();
        }
    }

    /// Sleep for `us` microseconds, waking early on any interrupt.
    ///
    /// See [`Delay::sleep_ticks`] for the details.
    #[inline]
    pub fn sleep_us(&self, us: u32) -> bool {
        self.sleep_ticks(us as u64 * TICKS_PER_US)
    }

    /// Sleep for `ms` milliseconds, waking early on any interrupt.
    ///
    /// See [`Delay::sleep_ticks`] for the details.
    #[inline]
    pub fn sleep_ms(&self, ms: u32) -> bool {
        self.sleep_ticks(ms as u64 * 1000 * TICKS_PER_US)
    }

    /// Sleep for `ticks` counter ticks, waking early on any interrupt.
    ///
    /// Arms the target-0 comparator and parks the core with `wfi`, so the CPU
    /// stops instead of spinning. Returns `true` if the full period elapsed,
    /// `false` if an interrupt woke the core first — the caller is expected to
    /// run its handler and call again, which is what a poll loop does.
    ///
    /// Global interrupts must be enabled for the timer to wake the core; with
    /// `mstatus.MIE` clear this falls back to spinning.
    pub fn sleep_ticks(&self, ticks: u64) -> bool {
        if ticks == 0 {
            return true;
        }

        let deadline = self.ticks().wrapping_add(ticks);

        self.systimer
            .target0_lo
            .write_timer_target0_lo(deadline as u32);
        self.systimer
            .target0_hi
            .write_timer_target0_hi((deadline >> 32) as u32);
        // Unit 0 counts, and the comparator is single-shot (`PERIOD = 0`).
        // `TARGET0_WORK_EN` resets to zero, so the comparator does nothing
        // until it is set here — without it the `wfi` below would only wake on
        // some other interrupt.
        self.systimer.target0_conf.clr_target0_timer_unit_sel();
        self.systimer.target0_conf.clr_target0_period_mode();
        self.systimer.conf.set_target0_work_en();
        self.systimer.comp0_load.trigger_timer_comp0_load();
        // A previous sleep that ended on the counter check (rather than on the
        // comparator interrupt) leaves the request latched. Enabling the source
        // would then fire it immediately and end this sleep at once — the
        // measured effect was every other 20 ms sleep returning in 0 us.
        self.systimer.int_clr.trigger_target0_int_clr();
        TIMER_FIRED.store(false, Ordering::Release);
        self.systimer.int_ena.set_target0_int_ena();

        let mut elapsed = true;
        // Any interrupt ends the sleep: the handler may have produced work the
        // caller has to service (a received frame, a completed transmission),
        // and the caller re-arms the comparator when it comes back.
        let epoch = interrupt::epoch();
        loop {
            if TIMER_FIRED.swap(false, Ordering::AcqRel) {
                // Woken by the comparator; check the clock in case another
                // interrupt fired first.
                elapsed = self.ticks() >= deadline;
                break;
            }

            if self.ticks() >= deadline {
                break;
            }

            if interrupt::epoch() != epoch {
                elapsed = false;
                break;
            }

            if interrupt::is_enabled() {
                interrupt::sleep();
            } else {
                core::hint::spin_loop();
            }
        }

        self.systimer.int_ena.clr_target0_int_ena();
        self.systimer.conf.clr_target0_work_en();
        elapsed
    }

    /// Sleep until `deadline`, waking early on any interrupt.
    ///
    /// Same contract as [`Delay::sleep_ticks`]: `true` if the deadline was
    /// reached, `false` if an interrupt woke the core first. A deadline that
    /// has already passed returns immediately.
    pub fn sleep_until(&self, deadline: crate::time::Instant) -> bool {
        let now = self.ticks();
        match deadline.as_ticks().checked_sub(now) {
            Some(ticks) => self.sleep_ticks(ticks),
            None => true,
        }
    }

    /// Release the underlying `SYSTIMER` peripheral.
    #[inline]
    pub fn release(self) -> SYSTIMER {
        self.systimer
    }
}
