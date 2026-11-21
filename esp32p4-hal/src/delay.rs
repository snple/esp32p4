//! Blocking delays: spin on the shared counter, or park the core on its own.
//!
//! # Two mechanisms, because `SYSTIMER` is one peripheral for the whole chip
//!
//! `SYSTIMER` has a single counter, three comparators, and both HP cores reach
//! the same registers. Spinning on the counter is harmless — a read only triggers
//! the snapshot handshake — so [`delay_us`] and [`delay_ms`] are free functions
//! that need no handle and work on any core.
//!
//! Parking the core is different. It needs a comparator to wake it up, and a
//! comparator is a single hardware object: the core that armed it is the one that
//! has to acknowledge it. Handing the same comparator to both cores means one
//! core's wakeup disarms the other's — the second core then sits in `wfi` until
//! some unrelated interrupt arrives, which is exactly the failure mode a shared
//! timer invites. The CLINT machine timer gives each core a comparator of its
//! own, so [`Delay::sleep_ticks`] uses that and two cores can sleep at once.
//!
//! [`crate::systimer::Systimer`] consequently owns all three `SYSTIMER`
//! comparators, and nothing here touches them.
//!
//! # Tick rates
//!
//! Spinning measures `SYSTIMER` ticks: 16 MHz straight from the crystal, so the
//! rate never moves. Sleeping measures machine timer ticks, which are CPU cycles
//! and therefore follow the CPU frequency. The conversion is calibrated against
//! `SYSTIMER` — an independent reference, so no frequency is written into the
//! code — and re-calibrated whenever [`crate::clock::generation`] reports that the
//! clock tree moved.
//!
//! # One machine timer interrupt per core
//!
//! Sleeping claims this core's machine timer interrupt (CLIC id 7) the first time
//! it parks. A program that wants that interrupt for its own tick should use
//! [`crate::machine_timer::MachineTimer`] instead of sleeping: the second claimant
//! panics rather than silently taking over.

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::clock;
use crate::interrupt::{self, Level};
use crate::machine_timer::MachineTimer;
use crate::multicore::CoreId;
use crate::time::{self, Instant, TICKS_PER_US};

/// Set by this core's wakeup handler, consumed by [`Delay::sleep_ticks`].
static FIRED: [AtomicBool; CoreId::COUNT] = [const { AtomicBool::new(false) }; CoreId::COUNT];

/// Whether the wakeup handler is installed on this core.
static INSTALLED: [AtomicBool; CoreId::COUNT] = [const { AtomicBool::new(false) }; CoreId::COUNT];

/// Machine timer ticks per microsecond, and the clock generation it holds for.
///
/// Zero means "not calibrated yet"; [`crate::clock::generation`] starts at one, so
/// the two can never be confused.
static MTIME_TICKS_PER_US: AtomicU32 = AtomicU32::new(0);
static MTIME_GENERATION: AtomicU32 = AtomicU32::new(0);

/// Length of the calibration window, in `SYSTIMER` ticks.
///
/// Two hundred microseconds: even at the slowest CPU frequency this chip runs at,
/// the window spans thousands of machine timer ticks, so dividing the two
/// measurements gives the rate to well under 0.1 %.
const CALIBRATION_TICKS: u64 = 200 * TICKS_PER_US;

/// How many times the calibration retries until two runs agree.
const CALIBRATION_TRIES: u32 = 3;

/// This core's wakeup handler.
///
/// The machine timer interrupt is level-triggered on `counter >= comparator`, so
/// acknowledging it means moving the comparator out of the way — which is also
/// what makes an early wake-up leave no pending request behind.
fn wakeup_isr() {
    // SAFETY: `Delay` only arms this core's comparator, and the handler cannot
    // fire before it has.
    let timer = unsafe { MachineTimer::steal() };
    timer.set_target(u64::MAX);
    FIRED[CoreId::current().as_index()].store(true, Ordering::Release);
}

/// Installs the wakeup handler on this core, once.
fn install_wakeup(core: usize) {
    if INSTALLED[core].swap(true, Ordering::AcqRel) {
        return;
    }
    // Panics if something else already owns this core's machine timer interrupt,
    // which beats two drivers quietly overwriting each other's handler.
    // SAFETY: nothing else in this module touches the comparator.
    unsafe { MachineTimer::steal() }.enable_interrupt(Level::LOW, wakeup_isr);
}

/// Machine timer ticks per microsecond, measured against `SYSTIMER`.
///
/// Re-measures when the clock tree has moved since the cached value was taken, so
/// a program that changes the CPU frequency keeps getting correct sleeps.
fn mtime_ticks_per_us() -> u64 {
    let generation = clock::generation();
    if MTIME_GENERATION.load(Ordering::Acquire) == generation {
        let cached = MTIME_TICKS_PER_US.load(Ordering::Relaxed);
        if cached != 0 {
            return u64::from(cached);
        }
    }
    calibrate(generation)
}

/// Measures the machine timer against `SYSTIMER`.
///
/// Both counters are sampled as a pair, machine timer first, at each end of the
/// window. That order matters: the two `SYSTIMER` reads then happen at the same
/// offset from their machine timer reads, so the read cost cancels and only the
/// *difference* in read cost between the two ends shows up in the ratio. Sampling
/// one before and one after the window instead biased the first measurement
/// enough to make the first sleep 20 % short (measured).
///
/// Two runs have to agree before the value is kept, which catches the slow first
/// access to the machine timer registers.
#[cold]
fn calibrate(generation: u32) -> u64 {
    // SAFETY: reading the counter needs no ownership, and this core's comparator
    // is only read.
    let timer = unsafe { MachineTimer::steal() };

    let mut ticks = 0u64;
    let mut previous = 0u64;
    for _ in 0..CALIBRATION_TRIES {
        let mtime_start = timer.now();
        let systimer_start = time::read_ticks();
        while time::read_ticks().wrapping_sub(systimer_start) < CALIBRATION_TICKS {
            core::hint::spin_loop();
        }
        let mtime_delta = timer.now().wrapping_sub(mtime_start);
        let systimer_delta = time::read_ticks().wrapping_sub(systimer_start);

        // `SYSTIMER` counts 16 ticks per microsecond, so the ratio of the two
        // deltas is microseconds × 16 and the division lands on ticks per
        // microsecond.
        ticks = (mtime_delta * TICKS_PER_US / systimer_delta).max(1);
        if previous != 0 && ticks.abs_diff(previous) * 1_000 <= previous {
            break;
        }
        previous = ticks;
    }

    MTIME_TICKS_PER_US.store(ticks as u32, Ordering::Release);
    MTIME_GENERATION.store(generation, Ordering::Release);
    ticks
}

/// Waits at least `us` microseconds, without owning anything.
///
/// Spins on the shared counter, so it works before any [`Delay`] exists and on
/// either core. Use it — not [`Delay::sleep_us`] — inside a register sequence:
/// parking the core there would cost more than the wait.
#[inline]
pub fn delay_us(us: u32) {
    spin_ticks(u64::from(us) * TICKS_PER_US);
}

/// Waits at least `ms` milliseconds, without owning anything.
///
/// See [`delay_us`] for when to use this rather than sleeping.
#[inline]
pub fn delay_ms(ms: u32) {
    spin_ticks(u64::from(ms) * 1_000 * TICKS_PER_US);
}

/// Spins until `ticks` `SYSTIMER` ticks have passed.
#[inline]
fn spin_ticks(ticks: u64) {
    let start = time::read_ticks();
    while time::read_ticks().wrapping_sub(start) < ticks {
        core::hint::spin_loop();
    }
}

/// Sleep for `ms` milliseconds on this core, waking early on any interrupt.
///
/// Driver-friendly wrapper around [`Delay::sleep_ms`] for code that has no handle
/// at hand. Not reentrant: it arms the same per-core comparator a [`Delay`] would.
#[inline]
pub fn sleep_ms(ms: u32) -> bool {
    Delay::init().sleep_ms(ms)
}

/// Microseconds since boot, readable without owning the timer.
///
/// Handy as a `defmt` timestamp: `defmt::timestamp!("{=u64:us}", micros());`
#[inline]
pub fn micros() -> u64 {
    time::Instant::now().as_ticks() / TICKS_PER_US
}

/// Blocking delays for the core that creates it.
///
/// Owns no peripheral: spinning reads the shared counter, and sleeping uses this
/// core's machine timer. That is what lets each core hold its own.
#[derive(Clone, Copy, Debug, Default)]
pub struct Delay;

impl Delay {
    /// Creates a handle for the calling core.
    ///
    /// Nothing is claimed here; the machine timer interrupt is taken on the first
    /// sleep.
    #[inline]
    pub const fn init() -> Self {
        Self
    }

    /// Read the current `SYSTIMER` counter value.
    #[inline]
    fn ticks(&self) -> u64 {
        time::read_ticks()
    }

    /// Microseconds since boot.
    #[inline]
    pub fn now_us(&self) -> u64 {
        self.ticks() / TICKS_PER_US
    }

    /// Wait for at least `us` microseconds, spinning.
    #[inline]
    pub fn delay_us(&self, us: u32) {
        delay_us(us);
    }

    /// Wait for at least `ms` milliseconds, spinning.
    #[inline]
    pub fn delay_ms(&self, ms: u32) {
        delay_ms(ms);
    }

    /// Sleep for `us` microseconds, waking early on any interrupt.
    ///
    /// The deadline is kept in `SYSTIMER` ticks, which are the crystal's 16 MHz and
    /// do not move, and the machine timer only arms the wakeup. A conversion that
    /// came out short — a calibration taken while the CPU frequency was still
    /// settling, say — therefore costs one more short sleep instead of a sleep that
    /// ends early: no sleep returns before its deadline.
    pub fn sleep_us(&self, us: u32) -> bool {
        if us == 0 {
            return true;
        }
        let deadline = self.ticks() + u64::from(us) * TICKS_PER_US;
        self.sleep_until_ticks(deadline)
    }

    /// Sleep for `ms` milliseconds, waking early on any interrupt.
    pub fn sleep_ms(&self, ms: u32) -> bool {
        if ms == 0 {
            return true;
        }
        let deadline = self.ticks() + u64::from(ms) * 1_000 * TICKS_PER_US;
        self.sleep_until_ticks(deadline)
    }

    /// Sleeps until a `SYSTIMER` deadline, re-arming if a wakeup came early.
    fn sleep_until_ticks(&self, deadline: u64) -> bool {
        // Every re-arm shortens the estimate, so a handful of rounds covers even a
        // wildly wrong one; after that the remainder is short enough to spin.
        for _ in 0..4 {
            let now = self.ticks();
            let Some(remaining) = deadline.checked_sub(now) else {
                return true;
            };
            let ticks = remaining.div_ceil(TICKS_PER_US) * mtime_ticks_per_us();
            if !self.sleep_ticks(ticks) {
                return false;
            }
        }
        let now = self.ticks();
        if now >= deadline {
            return true;
        }
        spin_ticks(deadline - now);
        true
    }

    /// Sleep for `ticks` machine timer ticks, waking early on any interrupt.
    ///
    /// Arms this core's own comparator and parks the core with `wfi`, so the CPU
    /// stops instead of spinning — and because the comparator is per core, another
    /// core sleeping at the same time is not disturbed. Returns `true` if the full
    /// period elapsed, `false` if an interrupt woke the core first, which is what
    /// the caller of a poll loop expects.
    ///
    /// Global interrupts must be enabled for the timer to wake the core; with
    /// `mstatus.MIE` clear this falls back to spinning.
    pub fn sleep_ticks(&self, ticks: u64) -> bool {
        if ticks == 0 {
            return true;
        }

        let core = CoreId::current().as_index();
        install_wakeup(core);

        // SAFETY: the comparator belongs to this core and only this code arms it.
        let timer = unsafe { MachineTimer::steal() };
        let deadline = timer.now().wrapping_add(ticks);

        FIRED[core].store(false, Ordering::Release);
        timer.set_target(deadline);

        let mut elapsed = true;
        // Any interrupt ends the sleep: the handler may have produced work the
        // caller has to service, and the caller re-arms when it comes back.
        let epoch = interrupt::local_epoch();
        loop {
            if FIRED[core].swap(false, Ordering::AcqRel) {
                // Woken by the comparator; check the clock in case another
                // interrupt fired first.
                elapsed = timer.now() >= deadline;
                break;
            }

            if timer.now() >= deadline {
                break;
            }

            if interrupt::local_epoch() != epoch {
                elapsed = false;
                break;
            }

            if interrupt::is_enabled() {
                interrupt::sleep();
            } else {
                core::hint::spin_loop();
            }
        }

        // Acknowledge, and disarm when the loop left early.
        timer.set_target(u64::MAX);
        elapsed
    }

    /// Sleep until `deadline`, waking early on any interrupt.
    ///
    /// Same contract as [`Delay::sleep_ticks`]: `true` if the deadline was
    /// reached, `false` if an interrupt woke the core first. A deadline that has
    /// already passed returns immediately.
    pub fn sleep_until(&self, deadline: Instant) -> bool {
        self.sleep_until_ticks(deadline.as_ticks())
    }
}
