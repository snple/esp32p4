//! Alarms on the system timer's comparators.
//!
//! `SYSTIMER` unit 0 is the monotonic clock behind [`crate::time::Instant`]: it
//! is 52 bits wide, ticks at a fixed 16 MHz and never stops. [`Systimer`] owns
//! the peripheral and hands out its comparators: [`Alarm1`] sits on COMP1,
//! [`Alarm2`] on COMP2, and [`Delay`] uses COMP0.
//!
//! An alarm has two modes:
//!
//! - **target**: one interrupt when the counter reaches an absolute deadline.
//!   The deadline is a full 52-bit counter value, i.e. an [`Instant`].
//! - **period**: an interrupt every `δt`, where `δt` is stored in a 26-bit
//!   register and is therefore limited to about 67 seconds.
//!
//! Both are level interrupts: the handler (or [`clear`](Alarm1::clear)) has to
//! drop the request. The periodic mode re-arms itself in hardware, so nothing
//! has to be reprogrammed after each alarm.
//!
//! | Type | Hardware | Matrix source |
//! |------|----------|---------------|
//! | [`Alarm1`] | COMP1 | `Interrupt::SYSTIMER_TARGET1` |
//! | [`Alarm2`] | COMP2 | `Interrupt::SYSTIMER_TARGET2` |
//!
//! Every method of an alarm takes `&self`, exactly like the general-purpose
//! counters in [`crate::timer`]: each alarm holds a handle to the timer it
//! belongs to, and `unsafe { Alarm1::steal() }` gives an interrupt handler the
//! same register access. The CPU lines belong to [`Systimer`], which claims them
//! for an alarm and gives them back when the alarm or the whole timer is
//! dropped.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::interrupt::{self, CpuInterrupt};
//! use esp32p4_hal::systimer::Systimer;
//! use esp32p4_hal::time::{Duration, Instant};
//!
//! fn tick() {
//!     // Runs in interrupt context: keep it short.
//! }
//!
//! let systimer = Systimer::init(dp.SYSTIMER);
//!
//! // The timer owns the CPU line, like a timer group owns its counters'.
//! systimer.enable_alarm1_interrupt(CpuInterrupt::new(23), tick);
//! systimer.alarm1.set_period(Duration::from_millis(10));  // every 10 ms
//! interrupt::enable();
//!
//! systimer.alarm2.set_target(Instant::now() + Duration::from_secs(2));
//! ```

use core::cell::RefCell;
use core::sync::atomic::{AtomicUsize, Ordering};

use esp32p4_pac::SYSTIMER;
use pastey::paste;

use crate::Steal;
use crate::delay::Delay;
use crate::interrupt::{self, CpuInterrupt, Interrupt, InterruptHandler, Level};
use crate::time::{self, Duration, Instant};

/// Widest alarm period: `TARGETx_PERIOD` is 26 bits.
const MAX_PERIOD_TICKS: u64 = (1 << 26) - 1;

/// Number of comparators the HAL hands out as alarms.
const ALARM_COUNT: usize = 2;

/// User callbacks, one per alarm, called by the generated interrupt shims.
static HANDLERS: [AtomicUsize; ALARM_COUNT] = [const { AtomicUsize::new(0) }; ALARM_COUNT];

/// Calls the user's handler of alarm `index`.
fn dispatch(index: usize) {
    let handler = HANDLERS[index].load(Ordering::Acquire);
    if handler != 0 {
        // SAFETY: `HANDLERS` only ever holds values stored by the timer, which
        // stores `fn()` pointers.
        let handler: fn() = unsafe { core::mem::transmute::<usize, fn()>(handler) };
        handler();
    }
}

// ============================================================================
// Alarms
// ============================================================================

/// Generates one alarm per comparator, the way `timer` generates one type per
/// counter.
///
/// The PAC names each comparator's registers after its instance (`target1_lo`,
/// `target2_lo`, …), so the names are pasted per invocation instead of being
/// matched at run time.
macro_rules! impl_alarm {
    (
        $(#[$attr:meta])*
        $Alarm:ident, $N:literal, $source:ident, $shim:ident, $index:literal
    ) => {
        $(#[$attr])*
        pub struct $Alarm {
            /// The system timer this comparator belongs to. The owner holds the
            /// same peripheral; the handle is what makes the registers reachable
            /// from an alarm.
            systimer: SYSTIMER,
        }

        impl $Alarm {
            const fn new() -> Self {
                // SAFETY: the `Systimer` that created this alarm owns the
                // peripheral, so the HAL already has exclusive access to its
                // registers. This is a second handle to them, and the alarm only
                // touches its own target registers.
                Self {
                    systimer: unsafe { SYSTIMER::steal() },
                }
            }

            /// Create a copy of this alarm without owning it.
            ///
            /// This is analogous to [`Pin::steal`](crate::gpio::Pin::steal): it
            /// lets an interrupt handler or another module reach the comparator
            /// registers while the owner lives elsewhere. Nothing is written to
            /// the hardware, so a stolen copy never disturbs a running alarm,
            /// and dropping it neither disarms the comparator nor touches an
            /// interrupt — the CPU lines belong to [`Systimer`].
            ///
            /// # Safety
            ///
            /// The caller must ensure that a [`Systimer`] exists and stays alive,
            /// and that the copy is not used to reconfigure the alarm behind the
            /// owner's back.
            #[inline]
            pub unsafe fn steal() -> Steal<Self> {
                Steal(Self::new())
            }

            /// Arm a one-shot alarm at an absolute deadline.
            ///
            /// A deadline that has already passed fires immediately.
            pub fn set_target(&self, deadline: Instant) {
                self.set_target_ticks(deadline.as_ticks());
            }

            /// Arm a one-shot alarm at an absolute counter value, in 16 MHz
            /// ticks.
            pub fn set_target_ticks(&self, ticks: u64) {
                paste! {
                    self.systimer.[<target $N _lo>].[<write_timer_target $N _lo>](ticks as u32);
                    self.systimer.[<target $N _hi>].[<write_timer_target $N _hi>](
                        (ticks >> 32) as u32
                    );
                    let conf = &self.systimer.[<target $N _conf>];
                    conf.[<clr_target $N _timer_unit_sel>]();
                    conf.[<clr_target $N _period_mode>]();
                    self.systimer.[<comp $N _load>].[<trigger_timer_comp $N _load>]();
                }
                self.start();
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

                paste! {
                    let conf = &self.systimer.[<target $N _conf>];
                    conf.[<write_target $N _period>](ticks as u32);
                    conf.[<clr_target $N _timer_unit_sel>]();
                    self.systimer.[<comp $N _load>].[<trigger_timer_comp $N _load>]();
                    // The TRM asks for period mode to be cleared and set again
                    // after the period has been loaded.
                    conf.[<clr_target $N _period_mode>]();
                    conf.[<set_target $N _period_mode>]();
                }
                self.start();
            }

            /// The configured period, zero while in target mode.
            pub fn period(&self) -> Duration {
                paste! {
                    let conf = &self.systimer.[<target $N _conf>];
                    if conf.[<target $N _period_mode>]() {
                        Duration::from_ticks(conf.[<target $N _period>]() as u64)
                    } else {
                        Duration::ZERO
                    }
                }
            }

            /// The alarm value the comparator is actually using.
            ///
            /// In target mode this is the deadline; in period mode it is the
            /// threshold of the next alarm, which the hardware keeps moving
            /// forward.
            pub fn target(&self) -> Instant {
                paste! {
                    let hi = self.systimer.[<real_target $N _hi>].[<target $N _hi_ro>]();
                    let lo = self.systimer.[<real_target $N _lo>].[<target $N _lo_ro>]();
                    Instant::from_ticks(((hi as u64) << 32) | lo as u64)
                }
            }

            /// Whether the alarm fires repeatedly or once.
            pub fn is_periodic(&self) -> bool {
                paste! { self.systimer.[<target $N _conf>].[<target $N _period_mode>]() }
            }

            /// Stop comparing. A request that is already pending is dropped.
            pub fn disarm(&self) {
                self.stop();
                self.clear();
            }

            /// Whether the alarm fired since the last clear.
            pub fn pending(&self) -> bool {
                paste! { self.systimer.int_raw.[<target $N _int_raw>]() }
            }

            /// Drop a pending alarm request.
            pub fn clear(&self) {
                paste! { self.systimer.int_clr.[<trigger_target $N _int_clr>]() }
            }

            /// Whether the alarm interrupt is unmasked.
            pub fn interrupt_enabled(&self) -> bool {
                paste! { self.systimer.int_ena.[<target $N _int_ena>]() }
            }

            /// Unmask the alarm interrupt.
            fn unmask_interrupt(&self) {
                paste! { self.systimer.int_ena.[<set_target $N _int_ena>](); }
            }

            /// Mask the alarm interrupt and drop a pending request.
            fn mask_interrupt(&self) {
                paste! {
                    self.systimer.int_ena.[<clr_target $N _int_ena>]();
                    self.systimer.int_clr.[<trigger_target $N _int_clr>]();
                }
            }

            /// Start the comparator.
            fn start(&self) {
                paste! { self.systimer.conf.[<set_target $N _work_en>](); }
            }

            /// Stop the comparator.
            fn stop(&self) {
                paste! { self.systimer.conf.[<clr_target $N _work_en>](); }
            }

            /// The interrupt matrix source feeding this comparator.
            const SOURCE: Interrupt = Interrupt::$source;
            /// The shim the timer installs on the comparator's CPU line.
            const SHIM: fn() = $shim;
            /// Slot of this comparator in the HAL's tables.
            const INDEX: usize = $index;
        }

        /// Interrupt shim of this comparator: acknowledges the alarm, then calls
        /// the user's handler.
        fn $shim() {
            $Alarm::new().clear();
            dispatch($index);
        }
    };
}

impl_alarm! {
    /// The first spare comparator, `SYSTIMER` COMP1.
    Alarm1, 1, SYSTIMER_TARGET1, alarm1, 0
}

impl_alarm! {
    /// The second spare comparator, `SYSTIMER` COMP2.
    Alarm2, 2, SYSTIMER_TARGET2, alarm2, 1
}

// ============================================================================
// The system timer
// ============================================================================

/// `SYSTIMER`: the free-running counter and the comparators built on it.
///
/// The type owns the peripheral and the CPU lines its alarms use. Comparator 0
/// belongs to [`Delay`], which gets its own handle to the same registers from
/// [`delay`](Self::delay).
pub struct Systimer {
    /// `None` once [`release`](Self::release) took the peripheral back.
    systimer: Option<SYSTIMER>,
    /// Comparator 1, the source of `SYSTIMER_TARGET1`.
    pub alarm1: Alarm1,
    /// Comparator 2, the source of `SYSTIMER_TARGET2`.
    pub alarm2: Alarm2,
    /// The CPU line the first alarm has claimed, if any.
    irq1: RefCell<Option<InterruptHandler>>,
    /// The CPU line the second alarm has claimed, if any.
    irq2: RefCell<Option<InterruptHandler>>,
}

impl Systimer {
    /// Take the system timer and its alarms.
    ///
    /// Starts the counter if nothing else has, so the alarms have a clock to
    /// compare against, and leaves both comparators stopped and disarmed.
    pub fn init(systimer: SYSTIMER) -> Self {
        time::start();

        let this = Self {
            systimer: Some(systimer),
            alarm1: Alarm1::new(),
            alarm2: Alarm2::new(),
            irq1: RefCell::new(None),
            irq2: RefCell::new(None),
        };
        this.alarm1.disarm();
        this.alarm2.disarm();
        this
    }

    /// Give the peripheral back.
    ///
    /// The CPU lines are released first, exactly as on drop.
    pub fn release(mut self) -> SYSTIMER {
        self.disable_alarm1_interrupt();
        self.disable_alarm2_interrupt();
        self.systimer.take().expect("the timer releases once")
    }

    /// A blocking delay driver on comparator 0.
    ///
    /// Comparator 0 is shared by every [`Delay`], so it is a driver of its own
    /// rather than a part of this type; it reaches the same counter through its
    /// own handle.
    pub fn delay(&self) -> Delay {
        // SAFETY: this type owns the peripheral; the delay only touches
        // comparator 0 and the counter.
        Delay::init(unsafe { SYSTIMER::steal() })
    }

    /// Route `alarm1` to `line` and call `handler` in interrupt context.
    ///
    /// Claims the line the way [`interrupt::claim`] does, so this panics if the
    /// line is already taken. The alarm still has to be armed with
    /// [`Alarm1::set_target`], [`Alarm1::set_target_ticks`] or
    /// [`Alarm1::set_period`], and global interrupts have to be enabled with
    /// [`interrupt::enable`].
    pub fn enable_alarm1_interrupt(&self, line: CpuInterrupt, handler: fn()) {
        self.claim(
            &self.irq1,
            Alarm1::INDEX,
            Alarm1::SOURCE,
            Alarm1::SHIM,
            line,
            handler,
        );
        self.alarm1.clear();
        self.alarm1.unmask_interrupt();
    }

    /// Mask `alarm1`'s interrupt and give its CPU line back.
    pub fn disable_alarm1_interrupt(&self) {
        self.alarm1.mask_interrupt();
        self.release_line(&self.irq1, Alarm1::INDEX);
    }

    /// Whether `alarm1`'s interrupt is unmasked.
    pub fn alarm1_interrupt_enabled(&self) -> bool {
        self.alarm1.interrupt_enabled()
    }

    /// Route `alarm2` to `line` and call `handler` in interrupt context.
    ///
    /// See [`enable_alarm1_interrupt`](Self::enable_alarm1_interrupt).
    pub fn enable_alarm2_interrupt(&self, line: CpuInterrupt, handler: fn()) {
        self.claim(
            &self.irq2,
            Alarm2::INDEX,
            Alarm2::SOURCE,
            Alarm2::SHIM,
            line,
            handler,
        );
        self.alarm2.clear();
        self.alarm2.unmask_interrupt();
    }

    /// Mask `alarm2`'s interrupt and give its CPU line back.
    pub fn disable_alarm2_interrupt(&self) {
        self.alarm2.mask_interrupt();
        self.release_line(&self.irq2, Alarm2::INDEX);
    }

    /// Whether `alarm2`'s interrupt is unmasked.
    pub fn alarm2_interrupt_enabled(&self) -> bool {
        self.alarm2.interrupt_enabled()
    }

    /// Take `line` for the alarm that owns `slot`.
    fn claim(
        &self,
        slot: &RefCell<Option<InterruptHandler>>,
        index: usize,
        source: Interrupt,
        shim: fn(),
        line: CpuInterrupt,
        handler: fn(),
    ) {
        assert!(
            slot.borrow().is_none(),
            "this alarm already has an interrupt handler"
        );

        HANDLERS[index].store(handler as usize, Ordering::Release);

        // SAFETY: the shim only acknowledges its comparator before calling the
        // user's function, and `claim` makes sure no other driver uses the line.
        let claimed = unsafe { interrupt::claim(source, line, Level::LOW, shim) };
        claimed.enable();
        *slot.borrow_mut() = Some(claimed);
    }

    /// Give the line in `slot` back.
    fn release_line(&self, slot: &RefCell<Option<InterruptHandler>>, index: usize) {
        if let Some(handler) = slot.borrow_mut().take() {
            // No shim may fire once the line is gone.
            HANDLERS[index].store(0, Ordering::Release);
            // Dropping the handler unregisters the shim and frees the line.
            drop(handler);
        }
    }
}

impl Drop for Systimer {
    fn drop(&mut self) {
        self.disable_alarm1_interrupt();
        self.disable_alarm2_interrupt();
    }
}
