//! The system timer: two free-running counters (`UNIT0`, `UNIT1`) and three
//! comparators (`COMP0`, `COMP1`, `COMP2`).
//!
//! `UNIT0` counts at a fixed 16 MHz — the crystal's 40 MHz divided by 2.5 — in 52
//! bits, so it wraps after about 8.9 years. It is the chip's monotonic clock and
//! the only clock here that is not tied to a clock tree. `UNIT1` runs at the same
//! rate and stays off until something asks for it.
//!
//! # Ownership
//!
//! [`Systimer`] is the only owner of the peripheral. Its parts are named after
//! the registers they drive, and each holds a handle to the peripheral its owner
//! holds — the same shape as the counters in [`crate::timer`]:
//!
//! | Type | Hardware | Part of |
//! |------|----------|---------|
//! | [`Unit0`] | the `UNIT0` counter | [`unit0`](Systimer::unit0) |
//! | [`Unit1`] | the `UNIT1` counter | [`unit1`](Systimer::unit1) |
//! | [`Alarm0`] | comparator `COMP0` | [`alarm0`](Systimer::alarm0) |
//! | [`Alarm1`] | comparator `COMP1` | [`alarm1`](Systimer::alarm1) |
//! | [`Alarm2`] | comparator `COMP2` | [`alarm2`](Systimer::alarm2) |
//!
//! Every method of a part takes `&self`, and `unsafe { Unit0::steal() }` or
//! `unsafe { Alarm0::steal() }` hands the same register access to an interrupt
//! handler, another driver or the other core. A stolen copy never starts a
//! counter, arms a comparator or claims a CPU line — those belong to the
//! [`Systimer`].
//!
//! # Alarm modes
//!
//! - **target**: one interrupt when the counter reaches an absolute deadline.
//! - **period**: an interrupt every `δt`, which the hardware re-arms by itself.
//!   The 26-bit period register caps `δt` at about 4.19 seconds.
//!
//! Both are level interrupts: the handler (or [`Alarm0::clear`]) has to drop the
//! request. A comparator picks which unit it watches with
//! [`Alarm0::set_timer_unit0`] / [`Alarm0::set_timer_unit1`]; the choice survives
//! re-arming.
//!
//! | Comparator | Matrix source |
//! |------------|---------------|
//! | [`Alarm0`] | `Interrupt::SYSTIMER_TARGET0` |
//! | [`Alarm1`] | `Interrupt::SYSTIMER_TARGET1` |
//! | [`Alarm2`] | `Interrupt::SYSTIMER_TARGET2` |
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::delay::DelayUs;
//! use esp32p4_hal::interrupt::{self, CpuInterrupt};
//! use esp32p4_hal::systimer::Systimer;
//! use esp32p4_hal::time::Duration;
//!
//! fn tick() {
//!     // Runs in interrupt context: keep it short.
//! }
//!
//! let systimer = Systimer::init(dp.SYSTIMER);
//! systimer.unit0.start();  // nothing runs until the program says so
//!
//! systimer.enable_alarm1_interrupt(CpuInterrupt::new(23), tick);
//! systimer.alarm1.set_period(Duration::from_millis(10));
//! interrupt::enable();
//!
//! systimer.alarm2.set_target(systimer.unit0.now() + Duration::from_secs(2));
//!
//! // The counter also waits, so no other timer is needed.
//! systimer.unit0.delay_ms(5);
//! ```

use core::cell::RefCell;

use esp32p4_pac::SYSTIMER;
use pastey::paste;

use crate::Steal;
use crate::delay::DelayUs;
use crate::interrupt::{self, CpuInterrupt, HandlerSlot, Interrupt, InterruptHandler, Level};
use crate::time::{Duration, Instant};

/// Counter ticks per second, for both units.
///
/// `SYSTIMER` runs off `XTAL_CLK` (40 MHz) through a fixed divide-by-2.5, so it
/// counts 16 million times a second and one tick is 62.5 ns. The source is
/// `HP_SYS_CLKRST.PERI_CLK_CTRL21.REG_SYSTIMER_CLK_SRC_SEL`, which resets to
/// `XTAL_CLK`; switching it to `RC_FAST_CLK` would change this and nothing in
/// this HAL does.
pub const TICK_HZ: u32 = 16_000_000;

/// Counter ticks per microsecond, for both units: 16 MHz / 1 MHz.
pub const TICKS_PER_US: u64 = (TICK_HZ / 1_000_000) as u64;

/// Widest alarm period: `TARGETx_PERIOD` is 26 bits, so about 4.19 seconds.
const MAX_PERIOD_TICKS: u64 = (1 << 26) - 1;

// ============================================================================
// Counter units
// ============================================================================

/// Generates one counter-unit view per instance.
///
/// The PAC names each unit's registers after its instance (`unit0_op`,
/// `unit1_value_hi`, …), so the names are pasted per invocation instead of being
/// matched at run time.
macro_rules! impl_unit {
    (
        $(#[$attr:meta])*
        $Unit:ident, $N:literal, $op:ident, $value_hi:ident, $value_lo:ident
    ) => {
        $(#[$attr])*
        ///
        /// Holds a handle to the peripheral its [`Systimer`] owns, like the
        /// counters in [`crate::timer`]. Reading only triggers the snapshot
        /// handshake, so a stolen copy disturbs nothing.
        pub struct $Unit {
            regs: SYSTIMER,
        }

        impl $Unit {
            const fn new() -> Self {
                // SAFETY: the `Systimer` that created this unit owns the
                // peripheral, so the HAL already has exclusive access to its
                // registers. This is a second handle to them, and the unit only
                // reads its own counter.
                Self {
                    regs: unsafe { SYSTIMER::steal() },
                }
            }

            /// Create a copy of this counter without owning it.
            ///
            /// This is analogous to [`Pin::steal`](crate::gpio::Pin::steal): it
            /// lets an interrupt handler or the other core read the clock while
            /// the owner lives elsewhere. Nothing is written to the hardware, so
            /// a stolen copy never disturbs a running counter.
            ///
            /// # Safety
            ///
            /// The caller must ensure that a [`Systimer`] exists, so the counter
            /// is running.
            #[inline]
            pub unsafe fn steal() -> Steal<Self> {
                Steal(Self::new())
            }

            /// The counter as 52 bits of ticks since it started.
            ///
            /// The counter runs in the `CNT_CLK` domain and cannot be read
            /// directly: the update register asks the hardware to copy it into
            /// `UNITn_VALUE_HI/LO`, and `VALUE_VALID` says when the copy is done.
            #[inline]
            pub fn ticks(&self) -> u64 {
                let regs = &self.regs;

                paste! {
                    regs.[<$op>].[<trigger_timer_unit $N _update>]();
                    while !regs.[<$op>].[<timer_unit $N _value_valid>]() {
                        core::hint::spin_loop();
                    }
                }

                // The snapshot registers are one copy for the whole chip, not
                // one per core: the other core can trigger an update between the
                // high and the low read. That only changes the result when the
                // two snapshots straddle the low word's 32-bit wrap — otherwise
                // the high word is identical and mixing them is harmless — so
                // the wrong value appears about once every 268 seconds, and only
                // when the reads collide. Reading the low word on both sides of
                // the high word detects the collision; the retry costs a few
                // cycles when it happens and nothing otherwise. ESP-IDF reads it
                // the same way for the same reason.
                paste! {
                    let mut lo = regs.[<$value_lo>].read_bits();
                    loop {
                        let hi = regs.[<$value_hi>].read_bits();
                        let again = regs.[<$value_lo>].read_bits();
                        if again == lo {
                            return (u64::from(hi) << 32) | u64::from(lo);
                        }
                        lo = again;
                    }
                }
            }

            /// Microseconds since the counter started.
            #[inline]
            pub fn micros(&self) -> u64 {
                self.ticks() / TICKS_PER_US
            }

            /// The counter as an [`Instant`].
            ///
            /// The epoch is wherever this unit was last reloaded, so an `Instant`
            /// from `UNIT1` is not comparable with one from `UNIT0`.
            #[inline]
            pub fn now(&self) -> Instant {
                Instant::from_micros(self.micros())
            }
        }

        impl $Unit {
            /// Starts this counter.
            ///
            /// Nothing starts on its own: an owner leaves both counters stopped,
            /// and the program starts what it needs. The stall-on-halt bits are
            /// cleared first so the counter keeps counting while a core sits idle.
            pub fn start(&self) {
                paste! {
                    self.regs.conf.[<clr_timer_unit $N _core0_stall_en>]();
                    self.regs.conf.[<clr_timer_unit $N _core1_stall_en>]();
                    self.regs.conf.[<set_timer_unit $N _work_en>]();
                }
            }

            /// Stops this counter.
            pub fn stop(&self) {
                paste! { self.regs.conf.[<clr_timer_unit $N _work_en>](); }
            }

            /// Whether this counter is counting.
            pub fn is_running(&self) -> bool {
                paste! { self.regs.conf.[<timer_unit $N _work_en>]() }
            }

            /// Reloads this counter with `ticks`.
            ///
            /// The value goes through a shadow register and is latched by the
            /// load trigger, so the counter never sees a half-written 52-bit
            /// value.
            pub fn set_ticks(&self, ticks: u64) {
                paste! {
                    self.regs.[<unit $N _load_hi>].[<write_timer_unit $N _load_hi>](
                        (ticks >> 32) as u32
                    );
                    self.regs.[<unit $N _load_lo>].[<write_timer_unit $N _load_lo>](
                        ticks as u32
                    );
                    self.regs.[<unit $N _load>].[<trigger_timer_unit $N _load>]();
                }
            }
        }

        impl DelayUs for $Unit {
            /// Busy-waits `us` microseconds on this counter.
            ///
            /// The counter has to be running: a stopped one never advances, so the
            /// wait would never end. `UNIT0` always runs; start `UNIT1` with
            /// [`Unit1::start`] before waiting on it.
            #[inline]
            fn delay_us(&self, us: u32) {
                if us == 0 {
                    return;
                }
                let start = self.ticks();
                let ticks = u64::from(us) * TICKS_PER_US;
                while self.ticks().wrapping_sub(start) < ticks {
                    core::hint::spin_loop();
                }
            }
        }
    };
}

impl_unit! {
    /// The `UNIT0` counter, the chip's monotonic clock.
    ///
    /// [`Systimer::init`] starts it, so every view obtained from a live owner
    /// counts.
    Unit0, 0, unit0_op, unit0_value_hi, unit0_value_lo
}

impl_unit! {
    /// The `UNIT1` counter.
    ///
    /// Read it only after [`Unit1::start`]: until then `UNIT1` is not
    /// counting and reads return whatever the register holds.
    Unit1, 1, unit1_op, unit1_value_hi, unit1_value_lo
}

// ============================================================================
// Comparators
// ============================================================================

/// Generates one comparator view per instance.
///
/// The PAC names each comparator's registers after its instance (`target1_lo`,
/// `target2_lo`, …), so the names are pasted per invocation instead of being
/// matched at run time.
macro_rules! impl_alarm {
    (
        $(#[$attr:meta])*
        $Alarm:ident, $N:literal, $shim:ident, $slot:ident
    ) => {
        static $slot: HandlerSlot = HandlerSlot::new();

        $(#[$attr])*
        ///
        /// Holds a handle to the peripheral its [`Systimer`] owns, like the
        /// counters in [`crate::timer`], and only ever takes `&self`, so a stolen
        /// copy can drive the whole register-level API.
        pub struct $Alarm {
            regs: SYSTIMER,
        }

        impl $Alarm {
            const fn new() -> Self {
                // SAFETY: the `Systimer` that created this comparator owns the
                // peripheral, so the HAL already has exclusive access to its
                // registers. This is a second handle to them, and the comparator
                // only touches its own target registers.
                Self {
                    regs: unsafe { SYSTIMER::steal() },
                }
            }

            /// Create a copy of this comparator without owning it.
            ///
            /// This is analogous to [`Pin::steal`](crate::gpio::Pin::steal): it
            /// lets an interrupt handler or another module reach the comparator
            /// registers while the owner lives elsewhere. Nothing is written to
            /// the hardware, so a stolen copy never disturbs a running alarm, and
            /// dropping it neither disarms the comparator nor touches an
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

            /// Arm a one-shot alarm at an absolute deadline on `UNIT0`.
            ///
            /// The deadline is rounded down to a whole microsecond, which is the
            /// resolution [`Instant`] carries. A deadline that has already passed
            /// fires immediately.
            ///
            /// # Panics
            ///
            /// Panics when the comparator watches `UNIT1`: an [`Instant`] is
            /// `UNIT0`'s timeline, and `UNIT1` counts from wherever it was
            /// reloaded. Use [`set_target_ticks`](Self::set_target_ticks) there.
            pub fn set_target(&self, deadline: Instant) {
                assert!(
                    !self.watches_unit1(),
                    "an Instant is UNIT0's timeline; arm UNIT1 with set_target_ticks"
                );
                self.set_target_ticks(deadline.as_micros().saturating_mul(TICKS_PER_US));
            }

            /// Arm a one-shot alarm at an absolute counter value, in ticks.
            pub fn set_target_ticks(&self, ticks: u64) {
                paste! {
                    self.regs.[<target $N _lo>].[<write_timer_target $N _lo>](ticks as u32);
                    self.regs.[<target $N _hi>].[<write_timer_target $N _hi>](
                        (ticks >> 32) as u32
                    );
                    self.regs.[<target $N _conf>].[<clr_target $N _period_mode>]();
                    self.regs.[<comp $N _load>].[<trigger_timer_comp $N _load>]();
                }
                self.start();
            }

            /// Arm a periodic alarm that fires every `period`.
            ///
            /// The 26-bit period register caps the period at about 4.19 seconds;
            /// longer intervals need a target alarm re-armed from the handler.
            pub fn set_period(&self, period: Duration) {
                self.set_period_ticks(period.as_ticks(TICK_HZ));
            }

            /// Arm a periodic alarm that fires every `ticks` counter ticks.
            ///
            /// # Panics
            ///
            /// Panics unless `ticks` is in `1..=2^26-1`, the range the period
            /// register can hold.
            pub fn set_period_ticks(&self, ticks: u64) {
                assert!(
                    (1..=MAX_PERIOD_TICKS).contains(&ticks),
                    "the period register holds 1..=2^26-1 system timer ticks"
                );

                paste! {
                    let conf = &self.regs.[<target $N _conf>];
                    conf.[<write_target $N _period>](ticks as u32);
                    self.regs.[<comp $N _load>].[<trigger_timer_comp $N _load>]();
                    // The TRM asks for period mode to be cleared and set again
                    // after the period has been loaded.
                    conf.[<clr_target $N _period_mode>]();
                    conf.[<set_target $N _period_mode>]();
                }
                self.start();
            }

            /// The configured period, zero while in target mode.
            pub fn period(&self) -> Duration {
                Duration::from_micros(self.period_ticks() / TICKS_PER_US)
            }

            /// The configured period in ticks, zero while in target mode.
            pub fn period_ticks(&self) -> u64 {
                paste! {
                    if self.regs.[<target $N _conf>].[<target $N _period_mode>]() {
                        self.regs.[<target $N _conf>].[<target $N _period>]() as u64
                    } else {
                        0
                    }
                }
            }

            /// The alarm value the comparator is actually using.
            ///
            /// In target mode this is the deadline; in period mode it is the
            /// threshold of the next alarm, which the hardware keeps moving
            /// forward.
            pub fn target(&self) -> Instant {
                Instant::from_micros(self.target_ticks() / TICKS_PER_US)
            }

            /// The alarm value in ticks, as [`target`](Self::target) but exact.
            pub fn target_ticks(&self) -> u64 {
                paste! {
                    let hi = self.regs.[<real_target $N _hi>].[<target $N _hi_ro>]();
                    let lo = self.regs.[<real_target $N _lo>].[<target $N _lo_ro>]();
                    (u64::from(hi) << 32) | u64::from(lo)
                }
            }

            /// Compare against `UNIT0`.
            pub fn set_timer_unit0(&self) {
                paste! { self.regs.[<target $N _conf>].[<clr_target $N _timer_unit_sel>](); }
            }

            /// Compare against `UNIT1`.
            ///
            /// Both units run at the same rate, so this only changes which
            /// counter the deadline refers to. `UNIT1` must be running already —
            /// see [`Unit1::start`].
            pub fn set_timer_unit1(&self) {
                paste! { self.regs.[<target $N _conf>].[<set_target $N _timer_unit_sel>](); }
            }

            /// Whether this comparator watches `UNIT1` rather than `UNIT0`.
            pub fn watches_unit1(&self) -> bool {
                paste! { self.regs.[<target $N _conf>].[<target $N _timer_unit_sel>]() }
            }

            /// Whether the alarm fires repeatedly or once.
            pub fn is_periodic(&self) -> bool {
                paste! { self.regs.[<target $N _conf>].[<target $N _period_mode>]() }
            }

            /// Stop comparing. A request that is already pending is dropped.
            pub fn disarm(&self) {
                self.stop();
                self.clear();
            }

            /// Whether the alarm fired since the last clear.
            pub fn pending(&self) -> bool {
                paste! { self.regs.int_raw.[<target $N _int_raw>]() }
            }

            /// Whether the request is pending *and* unmasked.
            ///
            /// Unlike [`pending`](Self::pending), which reads the raw status,
            /// this is the interrupt the CPU line would actually see.
            pub fn interrupt_status(&self) -> bool {
                paste! { self.regs.int_st.[<target $N _int_st>]() }
            }

            /// Drop a pending alarm request.
            pub fn clear(&self) {
                paste! { self.regs.int_clr.[<trigger_target $N _int_clr>]() }
            }

            /// Whether the alarm interrupt is unmasked.
            pub fn interrupt_enabled(&self) -> bool {
                paste! { self.regs.int_ena.[<target $N _int_ena>]() }
            }

            /// Unmask the alarm interrupt.
            fn unmask_interrupt(&self) {
                paste! { self.regs.int_ena.[<set_target $N _int_ena>](); }
            }

            /// Mask the alarm interrupt and drop a pending request.
            fn mask_interrupt(&self) {
                paste! {
                    self.regs.int_ena.[<clr_target $N _int_ena>]();
                    self.regs.int_clr.[<trigger_target $N _int_clr>]();
                }
            }

            /// Start the comparator.
            fn start(&self) {
                paste! { self.regs.conf.[<set_target $N _work_en>](); }
            }

            /// Stop the comparator.
            fn stop(&self) {
                paste! { self.regs.conf.[<clr_target $N _work_en>](); }
            }
        }

        /// Interrupt shim of this comparator: acknowledges the alarm, then calls
        /// the user's handler.
        ///
        /// It clears the request through the register block directly rather than
        /// through `$Alarm`, so a handler that forgets to acknowledge cannot
        /// leave the line asserted.
        fn $shim() {
            // SAFETY: the shim only runs while the `Systimer` that claimed this
            // line is alive, and it touches nothing but this comparator's own
            // request bit.
            let regs = unsafe { &*SYSTIMER::PTR };
            paste! { regs.int_clr.[<trigger_target $N _int_clr>](); }
            $slot.call();
        }
    };
}

impl_alarm! {
    /// Comparator `COMP0`.
    Alarm0, 0, alarm0_isr, ALARM0_HANDLER
}

impl_alarm! {
    /// Comparator `COMP1`.
    Alarm1, 1, alarm1_isr, ALARM1_HANDLER
}

impl_alarm! {
    /// Comparator `COMP2`.
    Alarm2, 2, alarm2_isr, ALARM2_HANDLER
}

// ============================================================================
// The system timer
// ============================================================================

/// `SYSTIMER`: the only owner of the peripheral.
///
/// Obtain one with [`init`](Self::init), which leaves both counters stopped and
/// every comparator disarmed and masked. Start what you need with
/// [`Unit0::start`] or [`Unit1::start`], read the clock through
/// [`unit0`](Self::unit0), and arm comparators through [`alarm0`](Self::alarm0),
/// [`alarm1`](Self::alarm1) and [`alarm2`](Self::alarm2).
pub struct Systimer {
    /// `None` once [`release`](Self::release) gave the peripheral away.
    systimer: Option<SYSTIMER>,
    /// The `UNIT0` counter.
    pub unit0: Unit0,
    /// The `UNIT1` counter.
    pub unit1: Unit1,
    /// Comparator `COMP0`.
    pub alarm0: Alarm0,
    /// Comparator `COMP1`.
    pub alarm1: Alarm1,
    /// Comparator `COMP2`.
    pub alarm2: Alarm2,
    /// The CPU line `COMP0` has claimed, if any.
    irq0: RefCell<Option<InterruptHandler>>,
    /// The CPU line `COMP1` has claimed, if any.
    irq1: RefCell<Option<InterruptHandler>>,
    /// The CPU line `COMP2` has claimed, if any.
    irq2: RefCell<Option<InterruptHandler>>,
}

impl Systimer {
    /// Takes the system timer and brings it into a known state.
    ///
    /// Opens the peripheral's clock gate, stops both counters and clears their
    /// stall-on-halt bits, and leaves every comparator stopped, pointed at
    /// `UNIT0` and masked. Nothing runs and nothing fires until the program
    /// starts a counter and arms an alarm.
    pub fn init(systimer: SYSTIMER) -> Self {
        systimer.conf.set_clk_en();
        systimer.conf.clr_timer_unit0_work_en();
        systimer.conf.clr_timer_unit0_core0_stall_en();
        systimer.conf.clr_timer_unit0_core1_stall_en();
        systimer.conf.clr_timer_unit1_work_en();
        systimer.conf.clr_timer_unit1_core0_stall_en();
        systimer.conf.clr_timer_unit1_core1_stall_en();

        let this = Self {
            systimer: Some(systimer),
            unit0: Unit0::new(),
            unit1: Unit1::new(),
            alarm0: Alarm0::new(),
            alarm1: Alarm1::new(),
            alarm2: Alarm2::new(),
            irq0: RefCell::new(None),
            irq1: RefCell::new(None),
            irq2: RefCell::new(None),
        };

        // A program that reset into us may have left a comparator armed and its
        // CPU line unmasked. Undoing both is what makes `init` mean "known
        // state" rather than "mostly known state".
        this.alarm0.set_timer_unit0();
        this.alarm1.set_timer_unit0();
        this.alarm2.set_timer_unit0();
        this.alarm0.mask_interrupt();
        this.alarm1.mask_interrupt();
        this.alarm2.mask_interrupt();
        this.alarm0.disarm();
        this.alarm1.disarm();
        this.alarm2.disarm();
        this
    }

    /// Gives the peripheral back.
    ///
    /// The CPU lines are released first, exactly as on drop.
    pub fn release(mut self) -> SYSTIMER {
        self.disable_alarm0_interrupt();
        self.disable_alarm1_interrupt();
        self.disable_alarm2_interrupt();
        self.systimer.take().expect("the timer releases once")
    }

    /// Routes `COMP0` to `line` and calls `handler` in interrupt context.
    ///
    /// Claims the line the way [`interrupt::claim`] does, so this panics if the
    /// line is already taken. The alarm still has to be armed with
    /// [`Alarm0::set_target`] or [`Alarm0::set_period`], and global interrupts
    /// have to be enabled with [`interrupt::enable`].
    pub fn enable_alarm0_interrupt(&self, line: CpuInterrupt, handler: fn()) {
        ALARM0_HANDLER.set(handler);
        self.claim(&self.irq0, Interrupt::SYSTIMER_TARGET0, alarm0_isr, line);
        self.alarm0.clear();
        self.alarm0.unmask_interrupt();
    }

    /// Masks `COMP0`'s interrupt and gives its CPU line back.
    pub fn disable_alarm0_interrupt(&self) {
        self.alarm0.mask_interrupt();
        self.release_line(&self.irq0);
        ALARM0_HANDLER.clear();
    }

    /// Whether `COMP0`'s interrupt is unmasked.
    pub fn alarm0_interrupt_enabled(&self) -> bool {
        self.alarm0.interrupt_enabled()
    }

    /// Routes `COMP1` to `line` and calls `handler` in interrupt context.
    ///
    /// See [`enable_alarm0_interrupt`](Self::enable_alarm0_interrupt).
    pub fn enable_alarm1_interrupt(&self, line: CpuInterrupt, handler: fn()) {
        ALARM1_HANDLER.set(handler);
        self.claim(&self.irq1, Interrupt::SYSTIMER_TARGET1, alarm1_isr, line);
        self.alarm1.clear();
        self.alarm1.unmask_interrupt();
    }

    /// Masks `COMP1`'s interrupt and gives its CPU line back.
    pub fn disable_alarm1_interrupt(&self) {
        self.alarm1.mask_interrupt();
        self.release_line(&self.irq1);
        ALARM1_HANDLER.clear();
    }

    /// Whether `COMP1`'s interrupt is unmasked.
    pub fn alarm1_interrupt_enabled(&self) -> bool {
        self.alarm1.interrupt_enabled()
    }

    /// Routes `COMP2` to `line` and calls `handler` in interrupt context.
    ///
    /// See [`enable_alarm0_interrupt`](Self::enable_alarm0_interrupt).
    pub fn enable_alarm2_interrupt(&self, line: CpuInterrupt, handler: fn()) {
        ALARM2_HANDLER.set(handler);
        self.claim(&self.irq2, Interrupt::SYSTIMER_TARGET2, alarm2_isr, line);
        self.alarm2.clear();
        self.alarm2.unmask_interrupt();
    }

    /// Masks `COMP2`'s interrupt and gives its CPU line back.
    pub fn disable_alarm2_interrupt(&self) {
        self.alarm2.mask_interrupt();
        self.release_line(&self.irq2);
        ALARM2_HANDLER.clear();
    }

    /// Whether `COMP2`'s interrupt is unmasked.
    pub fn alarm2_interrupt_enabled(&self) -> bool {
        self.alarm2.interrupt_enabled()
    }

    /// Takes `line` for the comparator that owns `slot`.
    fn claim(
        &self,
        slot: &RefCell<Option<InterruptHandler>>,
        source: Interrupt,
        shim: fn(),
        line: CpuInterrupt,
    ) {
        assert!(
            slot.borrow().is_none(),
            "this comparator already has an interrupt handler"
        );

        // SAFETY: the shim only acknowledges its comparator before calling the
        // user's function, and `claim` makes sure no other driver uses the line.
        let claimed = unsafe { interrupt::claim(source, line, Level::LOW, shim) };
        claimed.enable();
        *slot.borrow_mut() = Some(claimed);
    }

    /// Gives the line in `slot` back.
    fn release_line(&self, slot: &RefCell<Option<InterruptHandler>>) {
        // Dropping the handler unregisters the shim and frees the line.
        drop(slot.borrow_mut().take());
    }
}

impl Drop for Systimer {
    fn drop(&mut self) {
        // `release` already handed the peripheral to its caller. Writing to it
        // now would be touching hardware somebody else owns.
        if self.systimer.is_none() {
            return;
        }
        self.disable_alarm0_interrupt();
        self.disable_alarm1_interrupt();
        self.disable_alarm2_interrupt();
    }
}
