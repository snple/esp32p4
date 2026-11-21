//! Blocking delays using the system timer.
//!
//! The counter and the tick rate live in [`crate::time`]; this module only adds
//! comparator 0 on top of them, which lets the core park in `wfi` instead of
//! spinning through a delay.
//!
//! # Two kinds of wait
//!
//! [`delay_us`] and [`delay_ms`] need no handle: they spin on the free-running
//! counter, and the counter starts itself on first use. Drivers that have to
//! pause for a few microseconds in the middle of a register sequence use these,
//! because they cannot own `SYSTIMER` — the application may already hold it —
//! and because parking the core inside such a sequence is not wanted anyway.
//!
//! [`Delay`] is the owned handle for lengths where parking the core pays off:
//! its `sleep_*` methods arm comparator 0 and use `wfi`. That needs comparator 0
//! routed to a CPU interrupt line, which happens on the first use of either
//! [`Delay::init`] or [`sleep_ms`] and never again.

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

/// Routes comparator 0 to its CPU interrupt line, once.
///
/// Idempotent: the line belongs to the program, not to one `Delay` instance, so
/// whichever entry point runs first installs it and the rest are a single atomic
/// swap. Doing it here rather than requiring a [`Delay`] to exist means a caller
/// cannot forget it and then park the core with `wfi` on a comparator nothing
/// will ever acknowledge.
fn route_target0() {
    time::start();

    if INTERRUPT_ROUTED.swap(true, Ordering::AcqRel) {
        return;
    }

    // SAFETY: the handler only clears the comparator and sets a flag, and it
    // cannot fire before a comparator is armed.
    let handler = unsafe {
        interrupt::claim(
            Interrupt::SYSTIMER_TARGET0,
            interrupt::lines::SYSTIMER_TARGET0,
            Level::LOW,
            target0_isr,
        )
    };
    handler.enable();
    handler.leak();
}

/// Sleep for `ms` milliseconds, waking early on any interrupt.
///
/// Driver-friendly wrapper around [`Delay::sleep_ms`] for code that does not
/// own a [`Delay`] handle; the comparator is routed on first use.
#[inline]
pub fn sleep_ms(ms: u32) -> bool {
    route_target0();
    Delay {
        systimer: unsafe { SYSTIMER::steal() },
    }
    .sleep_ms(ms)
}

/// Waits at least `us` microseconds, without owning the timer.
///
/// Spins on the shared counter, so it works before any [`Delay`] exists and
/// takes no lock or peripheral handle. Use [`Delay::delay_us`] instead only when
/// a handle is already at hand; both wait the same length.
#[inline]
pub fn delay_us(us: u32) {
    spin_ticks(us as u64 * TICKS_PER_US);
}

/// Waits at least `ms` milliseconds, without owning the timer.
///
/// See [`delay_us`] for when to use this rather than a [`Delay`] handle.
#[inline]
pub fn delay_ms(ms: u32) {
    spin_ticks(ms as u64 * 1000 * TICKS_PER_US);
}

/// Spins until `ticks` counter ticks have passed.
#[inline]
fn spin_ticks(ticks: u64) {
    let start = time::read_ticks();
    while time::read_ticks().wrapping_sub(start) < ticks {
        core::hint::spin_loop();
    }
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
        route_target0();
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
