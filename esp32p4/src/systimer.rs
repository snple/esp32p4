//! Alarms on the system timer's comparators.
//!
//! `SYSTIMER` unit 0 is the monotonic clock behind [`crate::time::Instant`]: it
//! is 52 bits wide, ticks at a fixed 16 MHz and never stops. Its comparators
//! turn that clock into wake-ups. Comparator 0 belongs to
//! [`Delay`](crate::delay::Delay), which borrows it to sleep; the other two are
//! handed out as [`Alarm1`] and [`Alarm2`].
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
//! Every method takes `&self`, exactly like the general-purpose timers in
//! [`crate::timer`]: the owning instance carries the comparator claim and the
//! claimed CPU line, and `unsafe { Alarm1::steal() }` gives an interrupt handler
//! the same register access without either.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4::interrupt::{self, CpuInterrupt};
//! use esp32p4::systimer::Alarm1;
//! use esp32p4::time::{Duration, Instant};
//!
//! fn tick() {
//!     // Runs in interrupt context: keep it short.
//! }
//!
//! let alarm = Alarm1::new();
//! alarm.enable_interrupt(CpuInterrupt::new(23), tick);
//! alarm.set_period(Duration::from_millis(10));  // every 10 ms
//! interrupt::enable();
//!
//! let one_shot = Alarm2::new();
//! one_shot.set_target(Instant::now() + Duration::from_secs(2));
//! ```

use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

use esp32p4_pac::SYSTIMER;

use crate::Steal;
use crate::interrupt::{self, CpuInterrupt, Interrupt, InterruptHandler, Level};
use crate::time::{self, Duration, Instant};

/// Widest alarm period: `TARGETx_PERIOD` is 26 bits.
const MAX_PERIOD_TICKS: u64 = (1 << 26) - 1;

/// Number of usable comparators.
const ALARM_COUNT: usize = 2;

/// Bit per comparator, tracking which ones an alarm owns.
static CLAIMED: AtomicU32 = AtomicU32::new(0);

/// User callbacks, one per comparator.
static HANDLERS: [AtomicUsize; ALARM_COUNT] = [const { AtomicUsize::new(0) }; ALARM_COUNT];

fn systimer() -> &'static esp32p4_pac::systimer::RegisterBlock {
    // SAFETY: the system timer's register block lives for the whole program.
    unsafe { &*SYSTIMER::PTR }
}

/// Alarm dispatch shared by the two shims.
///
/// Both modes are level interrupts, so acknowledging the comparator is the
/// whole job; a periodic alarm re-arms itself in hardware.
fn alarm(index: usize) {
    let comp = Comp {
        number: (index + 1) as u8,
    };
    comp.clear();

    let handler = HANDLERS[index].load(Ordering::Acquire);
    if handler != 0 {
        // SAFETY: only `enable_interrupt` stores here, always a `fn()`.
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

/// One comparator's registers.
///
/// The PAC exposes COMP1 and COMP2 as separate register types, so the two arms
/// of each method pick the matching one; the comparator number is a literal in
/// every caller and the branches fold away.
#[derive(Clone, Copy)]
struct Comp {
    number: u8,
}

impl Comp {
    /// Arm a one-shot alarm at an absolute counter value, in 16 MHz ticks.
    fn set_target(&self, ticks: u64) {
        let systimer = systimer();
        match self.number {
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

    /// Arm a periodic alarm that fires every `ticks`.
    fn set_period(&self, ticks: u64) {
        assert!(
            (1..=MAX_PERIOD_TICKS).contains(&ticks),
            "the period register holds 1..=2^26-1 system timer ticks"
        );

        let systimer = systimer();
        match self.number {
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

    /// The configured period, zero in target mode.
    fn period(&self) -> Duration {
        let systimer = systimer();
        let (period_mode, ticks) = match self.number {
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
    fn target(&self) -> Instant {
        let systimer = systimer();
        let (hi, lo) = match self.number {
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
    fn is_periodic(&self) -> bool {
        let systimer = systimer();
        match self.number {
            1 => systimer.target1_conf.target1_period_mode(),
            _ => systimer.target2_conf.target2_period_mode(),
        }
    }

    /// Stop comparing and drop any pending request.
    fn disarm(&self) {
        self.set_work_enabled(false);
        self.clear();
    }

    /// Whether the alarm fired since the last clear.
    fn pending(&self) -> bool {
        let systimer = systimer();
        match self.number {
            1 => systimer.int_raw.target1_int_raw(),
            _ => systimer.int_raw.target2_int_raw(),
        }
    }

    /// Drop a pending alarm request.
    fn clear(&self) {
        let systimer = systimer();
        match self.number {
            1 => systimer.int_clr.trigger_target1_int_clr(),
            _ => systimer.int_clr.trigger_target2_int_clr(),
        }
    }

    /// Unmask or mask the interrupt request.
    fn set_interrupt_enabled(&self, enable: bool) {
        let systimer = systimer();
        match (self.number, enable) {
            (1, true) => systimer.int_ena.set_target1_int_ena(),
            (1, false) => systimer.int_ena.clr_target1_int_ena(),
            (_, true) => systimer.int_ena.set_target2_int_ena(),
            (_, false) => systimer.int_ena.clr_target2_int_ena(),
        }
    }

    /// Whether the interrupt request is unmasked.
    fn interrupt_enabled(&self) -> bool {
        let systimer = systimer();
        match self.number {
            1 => systimer.int_ena.target1_int_ena(),
            _ => systimer.int_ena.target2_int_ena(),
        }
    }

    /// Start or stop the comparator.
    fn set_work_enabled(&self, enable: bool) {
        let systimer = systimer();
        match (self.number, enable) {
            (1, true) => systimer.conf.set_target1_work_en(),
            (1, false) => systimer.conf.clr_target1_work_en(),
            (_, true) => systimer.conf.set_target2_work_en(),
            (_, false) => systimer.conf.clr_target2_work_en(),
        }
    }
}

/// Claim on a comparator, released when it is dropped.
struct Claim(u8);

impl Drop for Claim {
    fn drop(&mut self) {
        CLAIMED.fetch_and(!(1u32 << self.0), Ordering::AcqRel);
    }
}

/// Interrupt bookkeeping for one comparator.
struct Irq {
    /// Held for its `Drop`, which unregisters the shim and releases the line.
    _handler: InterruptHandler,
    comp: Comp,
    index: usize,
}

impl Drop for Irq {
    fn drop(&mut self) {
        self.comp.set_interrupt_enabled(false);
        self.comp.clear();
        HANDLERS[self.index].store(0, Ordering::Release);
    }
}

/// Generates one alarm type per comparator, the way `timer` generates one type
/// per general-purpose counter.
macro_rules! alarms {
    ($(
        $(#[$attr:meta])*
        $Alarm:ident => $number:literal, $source:ident, $index:literal, $shim:ident;
    )+) => {
        $(
            $(#[$attr])*
            pub struct $Alarm {
                /// `Some` only for the instance that created the alarm: it is
                /// held for its `Drop`, which frees the comparator.
                _claim: Option<Claim>,
                irq: RefCell<Option<Irq>>,
            }

            impl $Alarm {
                /// The comparator this alarm drives.
                const COMP: Comp = Comp { number: $number };
                /// The interrupt matrix source feeding this comparator.
                const SOURCE: Interrupt = Interrupt::$source;
                /// Index of this alarm in the HAL's tables.
                const INDEX: usize = $index;

                /// Take the comparator.
                ///
                /// Panics if it is already owned. Starts the counter if nothing
                /// else has, so the alarm has a clock to compare against.
                pub fn new() -> Self {
                    let bit = 1 << $number;
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
                    Self::COMP.disarm();

                    Self {
                        _claim: Some(Claim($number)),
                        irq: RefCell::new(None),
                    }
                }

                /// Create a copy of this alarm without owning the comparator.
                ///
                /// This is analogous to [`Pin::steal`](crate::gpio::Pin::steal):
                /// it lets an interrupt handler or another module reach the
                /// comparator registers while the owner lives elsewhere.
                /// Nothing is written to the hardware, so a stolen copy never
                /// disturbs a running alarm, and dropping it neither frees the
                /// comparator nor touches the interrupt.
                ///
                /// # Safety
                ///
                /// The caller must ensure that an [`Alarm1`]/[`Alarm2`] for this
                /// comparator exists and stays alive, and that the copy is not
                /// used to reconfigure the alarm behind the owner's back or to
                /// claim a second CPU line with
                /// [`enable_interrupt`](Self::enable_interrupt).
                #[inline]
                pub unsafe fn steal() -> Steal<Self> {
                    Steal(Self {
                        _claim: None,
                        irq: RefCell::new(None),
                    })
                }

                /// Arm a one-shot alarm at an absolute deadline.
                ///
                /// A deadline that has already passed fires immediately.
                pub fn set_target(&self, deadline: Instant) {
                    Self::COMP.set_target(deadline.as_ticks());
                }

                /// Arm a one-shot alarm at an absolute counter value, in 16 MHz
                /// ticks.
                pub fn set_target_ticks(&self, ticks: u64) {
                    Self::COMP.set_target(ticks);
                }

                /// Arm a periodic alarm that fires every `period`.
                ///
                /// The period is limited to about 67 seconds by the 26-bit
                /// period register; longer intervals need a target alarm
                /// re-armed from the handler.
                pub fn set_period(&self, period: Duration) {
                    Self::COMP.set_period(period.as_ticks());
                }

                /// The configured period, zero while in target mode.
                pub fn period(&self) -> Duration {
                    Self::COMP.period()
                }

                /// The alarm value the comparator is actually using.
                ///
                /// In target mode this is the deadline; in period mode it is the
                /// threshold of the next alarm, which the hardware keeps moving
                /// forward.
                pub fn target(&self) -> Instant {
                    Self::COMP.target()
                }

                /// Whether the alarm fires repeatedly or once.
                pub fn is_periodic(&self) -> bool {
                    Self::COMP.is_periodic()
                }

                /// Stop comparing. A request that is already pending is dropped.
                pub fn disarm(&self) {
                    Self::COMP.disarm();
                }

                /// Whether the alarm fired since the last clear.
                pub fn pending(&self) -> bool {
                    Self::COMP.pending()
                }

                /// Drop a pending alarm request.
                pub fn clear(&self) {
                    Self::COMP.clear();
                }

                /// Whether the alarm interrupt is unmasked.
                pub fn interrupt_enabled(&self) -> bool {
                    Self::COMP.interrupt_enabled()
                }

                /// Route this alarm to `line` and call `handler` in interrupt
                /// context.
                ///
                /// Claims the line the way [`interrupt::claim`] does, so the
                /// alarm panics if the line is already taken. The alarm still
                /// has to be armed with [`set_target`](Self::set_target),
                /// [`set_target_ticks`](Self::set_target_ticks) or
                /// [`set_period`](Self::set_period), and global interrupts have
                /// to be enabled with [`interrupt::enable`].
                pub fn enable_interrupt(&self, line: CpuInterrupt, handler: fn()) {
                    assert!(
                        self.irq.borrow().is_none(),
                        "this alarm already has an interrupt handler"
                    );

                    HANDLERS[Self::INDEX].store(handler as usize, Ordering::Release);

                    // SAFETY: the shim only acknowledges the comparator before
                    // calling the user's function, and `claim` reserves the
                    // line.
                    let claimed =
                        unsafe { interrupt::claim(Self::SOURCE, line, Level::LOW, $shim) };
                    claimed.enable();
                    Self::COMP.clear();
                    Self::COMP.set_interrupt_enabled(true);
                    *self.irq.borrow_mut() = Some(Irq {
                        _handler: claimed,
                        comp: Self::COMP,
                        index: Self::INDEX,
                    });
                }

                /// Mask the alarm interrupt and give its CPU line back.
                pub fn disable_interrupt(&self) {
                    // Dropping the bookkeeping masks the comparator, drops the
                    // request and releases the line.
                    self.irq.borrow_mut().take();
                }
            }

            impl Default for $Alarm {
                fn default() -> Self {
                    Self::new()
                }
            }
        )+
    };
}

alarms! {
    /// The first spare comparator, `SYSTIMER` COMP1.
    Alarm1 => 1, SYSTIMER_TARGET1, 0, alarm1;
    /// The second spare comparator, `SYSTIMER` COMP2.
    Alarm2 => 2, SYSTIMER_TARGET2, 1, alarm2;
}
