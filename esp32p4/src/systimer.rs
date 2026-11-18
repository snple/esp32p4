//! Alarms on the system timer's comparators.
//!
//! `SYSTIMER` unit 0 is the monotonic clock behind [`crate::time::Instant`]: it
//! is 52 bits wide, ticks at a fixed 16 MHz and never stops. Its comparators
//! turn that clock into wake-ups. Comparator 0 belongs to
//! [`Delay`](crate::delay::Delay), which borrows it to sleep; this module hands
//! out the other two as [`Alarm`]s.
//!
//! An alarm has two modes:
//!
//! - **target**: one interrupt when the counter reaches an absolute deadline.
//!   The deadline is a full 52-bit counter value, i.e. an [`Instant`].
//! - **period**: an interrupt every `δt`, where `δt` is stored in a 26-bit
//!   register and is therefore limited to about 67 seconds.
//!
//! Both are level interrupts: the handler (or [`AlarmHandle::clear`]) has to
//! drop the request. The periodic mode re-arms itself in hardware, so nothing
//! has to be reprogrammed after each alarm.
//!
//! | Comparator | Matrix source |
//! |------------|---------------|
//! | COMP1 | `Interrupt::SYSTIMER_TARGET1` |
//! | COMP2 | `Interrupt::SYSTIMER_TARGET2` |
//!
//! # Two levels of handle
//!
//! Like [`Timer`](crate::timer::Timer), [`Alarm`] owns the CPU line while an
//! interrupt is enabled, and every register operation lives on the `Copy`
//! [`AlarmHandle`] that it dereferences to. A handler can rebuild a handle with
//! [`AlarmHandle::steal`].
//!
//! # Example
//!
//! ```ignore
//! use esp32p4::interrupt::{self, CpuInterrupt};
//! use esp32p4::systimer::{Alarm, Comparator};
//! use esp32p4::time::{Duration, Instant};
//!
//! fn tick() {
//!     // Runs in interrupt context: keep it short.
//! }
//!
//! let alarm = Alarm::new(Comparator::One);
//! alarm.enable_interrupt(CpuInterrupt::new(23), tick);
//! alarm.set_period(Duration::from_millis(10));  // every 10 ms
//! interrupt::enable();
//!
//! // One-shot, at an absolute point in time
//! let other = Alarm::new(Comparator::Two);
//! other.set_target(Instant::now() + Duration::from_secs(2));
//! ```

use core::cell::RefCell;
use core::ops::Deref;
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

use esp32p4_pac::SYSTIMER;

use crate::Steal;
use crate::interrupt::{self, CpuInterrupt, Interrupt, InterruptHandler, Level};
use crate::time::{self, Duration, Instant};

/// A comparator of the system timer that can be used as an alarm.
///
/// COMP0 is not available: [`Delay`](crate::delay::Delay) owns it for its
/// timed sleeps.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Comparator {
    /// COMP1, fed by `SYSTIMER_TARGET1`.
    One,
    /// COMP2, fed by `SYSTIMER_TARGET2`.
    Two,
}

impl Comparator {
    /// The comparator's index in the register file, 1 or 2.
    const fn number(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
        }
    }

    /// The interrupt matrix source of this comparator.
    const fn source(self) -> Interrupt {
        match self {
            Self::One => Interrupt::SYSTIMER_TARGET1,
            Self::Two => Interrupt::SYSTIMER_TARGET2,
        }
    }

    /// The table index of this comparator.
    const fn index(self) -> usize {
        self.number() as usize - 1
    }
}

/// Widest alarm period: `TARGETx_PERIOD` is 26 bits.
const MAX_PERIOD_TICKS: u64 = (1 << 26) - 1;

/// Bit per comparator, tracking which ones an [`Alarm`] owns.
static CLAIMED: AtomicU32 = AtomicU32::new(0);

/// User callbacks, one per comparator.
static HANDLERS: [AtomicUsize; 2] = [const { AtomicUsize::new(0) }; 2];

fn systimer() -> &'static esp32p4_pac::systimer::RegisterBlock {
    // SAFETY: the system timer's register block lives for the whole program.
    unsafe { &*SYSTIMER::PTR }
}

/// Alarm dispatch shared by the two shims.
///
/// Both modes are level interrupts, so acknowledging the comparator is the
/// whole job; a periodic alarm re-arms itself in hardware.
fn alarm(index: usize) {
    let handle = AlarmHandle {
        comparator: (index + 1) as u8,
    };
    handle.clear();

    let handler = HANDLERS[index].load(Ordering::Acquire);
    if handler != 0 {
        // SAFETY: only `Alarm::enable_interrupt` stores here, always a `fn()`.
        let handler: fn() = unsafe { core::mem::transmute::<usize, fn()>(handler) };
        handler();
    }
}

fn alarm1() {
    alarm(0)
}

fn alarm2() {
    alarm(1)
}

/// One shim per comparator, so the trap handler knows which bit to clear.
const SHIMS: [fn(); 2] = [alarm1, alarm2];

/// A handle to one system timer comparator.
///
/// A handle owns nothing: every method takes `&self` and only touches the
/// hardware, so it can be copied with [`Alarm::handle`] or rebuilt in an
/// interrupt handler with [`AlarmHandle::steal`].
#[derive(Clone, Copy)]
pub struct AlarmHandle {
    comparator: u8,
}

impl AlarmHandle {
    const fn new(comparator: Comparator) -> Self {
        Self {
            comparator: comparator.number(),
        }
    }

    /// Take a handle without owning the comparator, for use in an interrupt
    /// handler or another module.
    ///
    /// # Safety
    ///
    /// The caller must ensure that an [`Alarm`] for this comparator exists and
    /// stays alive, so nothing else reprograms it underneath.
    pub const unsafe fn steal(comparator: Comparator) -> Steal<Self> {
        Steal(Self::new(comparator))
    }

    /// The comparator this handle refers to.
    pub const fn comparator(&self) -> Comparator {
        if self.comparator == 1 {
            Comparator::One
        } else {
            Comparator::Two
        }
    }

    /// Arm a one-shot alarm at an absolute counter value.
    ///
    /// A deadline that has already passed fires the alarm immediately.
    pub fn set_target(&self, deadline: Instant) {
        self.set_target_ticks(deadline.as_ticks());
    }

    /// Arm a one-shot alarm at an absolute counter value, in 16 MHz ticks.
    pub fn set_target_ticks(&self, ticks: u64) {
        let systimer = systimer();
        match self.comparator {
            1 => {
                systimer.target1_lo.write_timer_target1_lo(ticks as u32);
                systimer
                    .target1_hi
                    .write_timer_target1_hi((ticks >> 32) as u32);
                systimer.target1_conf.clr_target1_timer_unit_sel();
                systimer.target1_conf.clr_target1_period_mode();
                systimer.comp1_load.trigger_timer_comp1_load();
            }
            _ => {
                systimer.target2_lo.write_timer_target2_lo(ticks as u32);
                systimer
                    .target2_hi
                    .write_timer_target2_hi((ticks >> 32) as u32);
                systimer.target2_conf.clr_target2_timer_unit_sel();
                systimer.target2_conf.clr_target2_period_mode();
                systimer.comp2_load.trigger_timer_comp2_load();
            }
        }
        self.set_work_enabled(true);
    }

    /// Arm a periodic alarm that fires every `period`.
    ///
    /// The period is limited to about 67 seconds by the 26-bit period
    /// register; longer intervals need a target alarm re-armed from the
    /// handler.
    pub fn set_period(&self, period: Duration) {
        let ticks = period.as_ticks();
        assert!(
            (1..=MAX_PERIOD_TICKS).contains(&ticks),
            "the period register holds 1..=2^26-1 system timer ticks"
        );

        let systimer = systimer();
        match self.comparator {
            1 => {
                systimer.target1_conf.write_target1_period(ticks as u32);
                systimer.target1_conf.clr_target1_timer_unit_sel();
                systimer.comp1_load.trigger_timer_comp1_load();
                // The TRM asks for period mode to be cleared and set again
                // after the period has been loaded.
                systimer.target1_conf.clr_target1_period_mode();
                systimer.target1_conf.set_target1_period_mode();
            }
            _ => {
                systimer.target2_conf.write_target2_period(ticks as u32);
                systimer.target2_conf.clr_target2_timer_unit_sel();
                systimer.comp2_load.trigger_timer_comp2_load();
                systimer.target2_conf.clr_target2_period_mode();
                systimer.target2_conf.set_target2_period_mode();
            }
        }
        self.set_work_enabled(true);
    }

    /// The configured period, zero when the alarm is in target mode.
    pub fn period(&self) -> Duration {
        let systimer = systimer();
        let (period_mode, ticks) = match self.comparator {
            1 => (
                systimer.target1_conf.target1_period_mode(),
                systimer.target1_conf.target1_period(),
            ),
            _ => (
                systimer.target2_conf.target2_period_mode(),
                systimer.target2_conf.target2_period(),
            ),
        };
        if period_mode {
            Duration::from_ticks(ticks as u64)
        } else {
            Duration::ZERO
        }
    }

    /// The alarm value the comparator is actually using.
    ///
    /// In target mode this is the deadline; in period mode it is the threshold
    /// of the next alarm, which the hardware keeps moving forward.
    pub fn target(&self) -> Instant {
        let systimer = systimer();
        let (hi, lo) = match self.comparator {
            1 => (
                systimer.real_target1_hi.target1_hi_ro(),
                systimer.real_target1_lo.target1_lo_ro(),
            ),
            _ => (
                systimer.real_target2_hi.target2_hi_ro(),
                systimer.real_target2_lo.target2_lo_ro(),
            ),
        };
        Instant::from_ticks(((hi as u64) << 32) | lo as u64)
    }

    /// Whether the alarm fires repeatedly or once.
    pub fn is_periodic(&self) -> bool {
        let systimer = systimer();
        match self.comparator {
            1 => systimer.target1_conf.target1_period_mode(),
            _ => systimer.target2_conf.target2_period_mode(),
        }
    }

    /// Stop comparing. A request that is already pending is dropped.
    pub fn disarm(&self) {
        self.set_work_enabled(false);
        self.clear();
    }

    /// Whether the alarm fired since the last clear.
    pub fn pending(&self) -> bool {
        let systimer = systimer();
        match self.comparator {
            1 => systimer.int_raw.target1_int_raw(),
            _ => systimer.int_raw.target2_int_raw(),
        }
    }

    /// Drop a pending alarm request.
    pub fn clear(&self) {
        let systimer = systimer();
        match self.comparator {
            1 => systimer.int_clr.trigger_target1_int_clr(),
            _ => systimer.int_clr.trigger_target2_int_clr(),
        }
    }

    /// Whether the alarm interrupt is unmasked.
    pub fn interrupt_enabled(&self) -> bool {
        let systimer = systimer();
        match self.comparator {
            1 => systimer.int_ena.target1_int_ena(),
            _ => systimer.int_ena.target2_int_ena(),
        }
    }

    /// Start or stop the comparator.
    fn set_work_enabled(&self, enable: bool) {
        let systimer = systimer();
        match (self.comparator, enable) {
            (1, true) => systimer.conf.set_target1_work_en(),
            (1, false) => systimer.conf.clr_target1_work_en(),
            (_, true) => systimer.conf.set_target2_work_en(),
            (_, false) => systimer.conf.clr_target2_work_en(),
        }
    }

    /// Unmask or mask the interrupt request.
    fn set_interrupt_enabled(&self, enable: bool) {
        let systimer = systimer();
        match (self.comparator, enable) {
            (1, true) => systimer.int_ena.set_target1_int_ena(),
            (1, false) => systimer.int_ena.clr_target1_int_ena(),
            (_, true) => systimer.int_ena.set_target2_int_ena(),
            (_, false) => systimer.int_ena.clr_target2_int_ena(),
        }
    }
}

/// One comparator of the system timer, owned by the caller.
///
/// Dereferences to the [`AlarmHandle`] with the register-level API; the methods
/// here are the ones that claim the CPU line.
pub struct Alarm {
    handle: AlarmHandle,
    handler: RefCell<Option<InterruptHandler>>,
}

impl Deref for Alarm {
    type Target = AlarmHandle;

    #[inline]
    fn deref(&self) -> &AlarmHandle {
        &self.handle
    }
}

impl Alarm {
    /// Take one of the spare comparators.
    ///
    /// Panics if that comparator is already owned. Starts the counter if
    /// nothing else has, so the alarm has a clock to compare against.
    pub fn new(comparator: Comparator) -> Self {
        let bit = 1 << comparator.number();
        let mut claimed = CLAIMED.load(Ordering::Relaxed);
        loop {
            assert!(
                claimed & bit == 0,
                "this system timer comparator is already in use"
            );
            match CLAIMED.compare_exchange_weak(
                claimed,
                claimed | bit,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => claimed = actual,
            }
        }

        time::start();

        let handle = AlarmHandle::new(comparator);
        handle.disarm();
        Self {
            handle,
            handler: RefCell::new(None),
        }
    }

    /// A copy of this alarm's register handle.
    #[inline]
    pub fn handle(&self) -> AlarmHandle {
        self.handle
    }

    /// Route this alarm to `line` and call `handler` in interrupt context.
    ///
    /// Claims the line the way [`interrupt::claim`] does, so the alarm panics
    /// if the line is already taken. The alarm still has to be armed with
    /// [`AlarmHandle::set_target`] or [`AlarmHandle::set_period`], and global
    /// interrupts have to be enabled with [`interrupt::enable`].
    pub fn enable_interrupt(&self, line: CpuInterrupt, handler: fn()) {
        assert!(
            self.handler.borrow().is_none(),
            "this alarm already has an interrupt handler"
        );

        let index = self.handle.comparator().index();
        HANDLERS[index].store(handler as usize, Ordering::Release);

        // SAFETY: the shim only acknowledges the comparator before calling the
        // user's function, and `claim` reserves the line.
        let claimed = unsafe {
            interrupt::claim(
                self.handle.comparator().source(),
                line,
                Level::LOW,
                SHIMS[index],
            )
        };
        claimed.enable();
        self.clear();
        self.handle.set_interrupt_enabled(true);
        *self.handler.borrow_mut() = Some(claimed);
    }

    /// Mask the alarm interrupt and give its CPU line back.
    pub fn disable_interrupt(&self) {
        self.handle.set_interrupt_enabled(false);
        self.clear();
        HANDLERS[self.handle.comparator().index()].store(0, Ordering::Release);
        self.handler.borrow_mut().take();
    }

    /// Arm a one-shot alarm `delay` from now.
    pub fn start_in(&self, delay: Duration) {
        self.handle.set_target(Instant::now() + delay);
    }
}

impl Drop for Alarm {
    fn drop(&mut self) {
        self.handle.disarm();
        self.handle.set_interrupt_enabled(false);
        HANDLERS[self.handle.comparator().index()].store(0, Ordering::Release);
        CLAIMED.fetch_and(!(1u32 << self.handle.comparator), Ordering::AcqRel);
    }
}
