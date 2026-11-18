//! General-purpose timers (`TIMG0` / `TIMG1`).
//!
//! The chip has two timer groups, each with two independent 54-bit counters and
//! one watchdog. A counter runs off `XTAL_CLK` (40 MHz), `RC_FAST_CLK`
//! (~17.5 MHz) or `PLL_F80M_CLK` (80 MHz) through a 16-bit prescaler, counts up
//! or down, can be reloaded by software, and can reload itself from its reload
//! value when its alarm fires. The default configuration divides the 40 MHz
//! crystal by 40, so one tick is one microsecond.
//!
//! Each counter is its own type, named after its interrupt source:
//!
//! | Type | Hardware | Matrix source |
//! |------|----------|---------------|
//! | [`Tg0T0`] | `TIMG0` timer 0 | `Interrupt::TG0_T0` |
//! | [`Tg0T1`] | `TIMG0` timer 1 | `Interrupt::TG0_T1` |
//! | [`Tg1T0`] | `TIMG1` timer 0 | `Interrupt::TG1_T0` |
//! | [`Tg1T1`] | `TIMG1` timer 1 | `Interrupt::TG1_T1` |
//!
//! [`TimerGroup0::split`] and [`TimerGroup1::split`] hand out the two timers of
//! a group; each one owns its CPU line while an interrupt is enabled. The two
//! timers of a group share the group's interrupt enable/status registers, which
//! the driver hides.
//!
//! Every method takes `&self`, because all state lives in the peripheral's
//! volatile registers. That is what makes `unsafe { Tg0T0::steal() }` useful:
//! an interrupt handler (or any other module) gets a `Steal<Tg0T0>` and with it
//! the same access as the owner. The only thing a stolen copy lacks is the
//! claimed CPU line, which the owning instance keeps to itself.
//!
//! Unlike `SYSTIMER`, these counters are not free-running across the whole
//! program: they stop, reload and reset on request. Use them for alarms and
//! periodic ticks; use [`crate::time::Instant`] for a monotonic clock, and
//! [`crate::systimer`] for alarms on it.
//!
//! The vendor SVD does not describe timer 1 of either group, so this module
//! talks to the registers through a hand-written register layer (`regs.rs`),
//! derived from the TRM.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4::interrupt::{self, CpuInterrupt};
//! use esp32p4::time::Duration;
//! use esp32p4::timer::TimerGroup0;
//!
//! fn tick() {
//!     // Runs in interrupt context: keep it short.
//! }
//!
//! let (fast, slow) = TimerGroup0::new(dp.TIMG0, &dp.HP_SYS_CLKRST).split();
//! fast.enable_interrupt(CpuInterrupt::new(21), tick);
//! fast.start_periodic(Duration::from_millis(100));
//!
//! slow.set_direction(Direction::Down);
//! slow.start_periodic(Duration::from_secs(1));
//! interrupt::enable();
//! ```
//!
//! # Access from an interrupt
//!
//! ```ignore
//! const PERIOD_TICKS: u64 = 500_000; // 500 ms at 1 MHz
//!
//! fn overrun() {
//!     // SAFETY: `main` owns this timer and keeps it alive.
//!     let timer = unsafe { Tg0T0::steal() };
//!     OVERRUNS.fetch_add(1, Ordering::Relaxed);
//!     timer.set_alarm(timer.counter() + PERIOD_TICKS);
//! }
//! ```

use core::cell::RefCell;
use core::sync::atomic::{AtomicUsize, Ordering};

use esp32p4_pac::{HP_SYS_CLKRST, TIMG0, TIMG1};

use crate::Steal;
use crate::interrupt::{self, CpuInterrupt, Interrupt, InterruptHandler, Level};
use crate::time::{self, Duration};

mod regs;

use regs::{GROUP0_BASE, GROUP1_BASE, MAX_VALUE, Regs};

/// A clock a timer can prescale.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockSource {
    /// The 40 MHz crystal.
    Xtal,
    /// The fast RC oscillator, about 17.5 MHz and not temperature stable.
    RcFast,
    /// The 80 MHz PLL output.
    PllF80m,
}

impl ClockSource {
    /// The nominal frequency of the source, in Hz.
    pub const fn frequency_hz(self) -> u32 {
        match self {
            Self::Xtal => 40_000_000,
            Self::RcFast => 17_500_000,
            Self::PllF80m => 80_000_000,
        }
    }

    /// The value `HP_SYS_CLKRST` wants in `TIMERGRPn_Tx_SRC_SEL`.
    const fn select(self) -> u32 {
        match self {
            Self::Xtal => 0,
            Self::RcFast => 1,
            Self::PllF80m => 2,
        }
    }

    /// The source a `TIMERGRPn_Tx_SRC_SEL` value selects.
    const fn from_select(value: u32) -> Self {
        match value {
            1 => Self::RcFast,
            2 => Self::PllF80m,
            _ => Self::Xtal,
        }
    }
}

/// Which way a timer counts.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Direction {
    /// Count up, from the reload value towards the maximum.
    Up,
    /// Count down, from the reload value towards zero.
    Down,
}

/// The clock controller's registers, as the timers see them.
type ClockRegs = esp32p4_pac::hp_sys_clkrst::RegisterBlock;

/// The prescaler that makes one tick one microsecond out of the crystal.
const DEFAULT_DIVIDER: u32 = 40;

/// Number of general-purpose timers on the chip.
const TIMER_COUNT: usize = 4;

/// User callbacks, one per timer, called by the shims below.
static HANDLERS: [AtomicUsize; TIMER_COUNT] = [const { AtomicUsize::new(0) }; TIMER_COUNT];

/// Alarm dispatch shared by the four shims.
///
/// Acknowledge the timer, re-arm it if it is running in periodic mode, and call
/// the user's handler. The shim has to do the first two steps because the user's
/// handler is a bare `fn` and cannot reach the timer.
fn alarm(index: usize) {
    let regs = Regs::at(index);

    // Level interrupt: clear the request before returning, or the line stays
    // asserted and the trap repeats.
    regs.clear_interrupt();

    // Hardware clears `ALARM_EN` on every alarm. With auto-reload the counter
    // restarts from the reload value, so re-arming is all that is needed to
    // get the next period.
    if regs.auto_reload() {
        regs.set_alarm_enabled(true);
    }

    let handler = HANDLERS[index].load(Ordering::Acquire);
    if handler != 0 {
        // SAFETY: `HANDLERS` only ever holds values stored by
        // `enable_interrupt`, which stores `fn()` pointers.
        let handler: fn() = unsafe { core::mem::transmute::<usize, fn()>(handler) };
        handler();
    }
}

fn alarm0() {
    alarm(0)
}

fn alarm1() {
    alarm(1)
}

fn alarm2() {
    alarm(2)
}

fn alarm3() {
    alarm(3)
}

/// Selects the clock and opens the bus and timer gates for one timer.
///
/// Two gates are involved: `SOC_CLK_CTRL2` lets the CPU reach the group's
/// registers at all, and `PERI_CLK_CTRL2n` picks the source and gates the
/// counter clock of one timer. ESP-IDF opens both when a timer is claimed.
fn program_clock(clkrst: &ClockRegs, group: u8, unit: u8, clock: ClockSource) {
    let source = clock.select();
    if group == 0 {
        clkrst.soc_clk_ctrl2.set_reg_timergrp0_apb_clk_en();
        let ctrl = &clkrst.peri_clk_ctrl20;
        if unit == 0 {
            ctrl.write_reg_timergrp0_t0_src_sel(source);
            ctrl.set_reg_timergrp0_t0_clk_en();
        } else {
            ctrl.write_reg_timergrp0_t1_src_sel(source);
            ctrl.set_reg_timergrp0_t1_clk_en();
        }
    } else {
        clkrst.soc_clk_ctrl2.set_reg_timergrp1_apb_clk_en();
        let ctrl = &clkrst.peri_clk_ctrl21;
        if unit == 0 {
            ctrl.write_reg_timergrp1_t0_src_sel(source);
            ctrl.set_reg_timergrp1_t0_clk_en();
        } else {
            ctrl.write_reg_timergrp1_t1_src_sel(source);
            ctrl.set_reg_timergrp1_t1_clk_en();
        }
    }
}

/// The clock currently feeding one timer's prescaler.
fn clock_source_of(clkrst: &ClockRegs, group: u8, unit: u8) -> ClockSource {
    let selected = match (group, unit) {
        (0, 0) => clkrst.peri_clk_ctrl20.reg_timergrp0_t0_src_sel(),
        (0, _) => clkrst.peri_clk_ctrl20.reg_timergrp0_t1_src_sel(),
        (1, 0) => clkrst.peri_clk_ctrl21.reg_timergrp1_t0_src_sel(),
        _ => clkrst.peri_clk_ctrl21.reg_timergrp1_t1_src_sel(),
    };
    ClockSource::from_select(selected)
}

/// Interrupt bookkeeping for one timer.
///
/// Holding this is what owns the CPU line: dropping it masks the timer, drops
/// the request, unregisters the shim and hands the line back. Keeping it out of
/// the timer type itself is what lets a stolen copy stay inert.
struct Irq {
    /// Held for its `Drop`, which unregisters the shim and releases the line.
    _handler: InterruptHandler,
    regs: Regs,
    index: usize,
}

impl Drop for Irq {
    fn drop(&mut self) {
        self.regs.enable_interrupt(false);
        self.regs.clear_interrupt();
        HANDLERS[self.index].store(0, Ordering::Release);
    }
}

/// `TIMG0`: two general-purpose timers and the group's watchdog.
pub struct TimerGroup0<'d> {
    clkrst: &'d ClockRegs,
    _timg: TIMG0,
}

impl<'d> TimerGroup0<'d> {
    /// Take ownership of the group.
    pub fn new(timg: TIMG0, clkrst: &'d HP_SYS_CLKRST) -> Self {
        Self {
            clkrst: &**clkrst,
            _timg: timg,
        }
    }

    /// Reset and hand out the group's two timers.
    pub fn split(self) -> (Tg0T0<'d>, Tg0T1<'d>) {
        (Tg0T0::init(self.clkrst), Tg0T1::init(self.clkrst))
    }
}

/// `TIMG1`: two general-purpose timers and the group's watchdog.
pub struct TimerGroup1<'d> {
    clkrst: &'d ClockRegs,
    _timg: TIMG1,
}

impl<'d> TimerGroup1<'d> {
    /// Take ownership of the group.
    pub fn new(timg: TIMG1, clkrst: &'d HP_SYS_CLKRST) -> Self {
        Self {
            clkrst: &**clkrst,
            _timg: timg,
        }
    }

    /// Reset and hand out the group's two timers.
    pub fn split(self) -> (Tg1T0<'d>, Tg1T1<'d>) {
        (Tg1T0::init(self.clkrst), Tg1T1::init(self.clkrst))
    }
}

/// Generates one timer type per counter, the way the rp2350 HAL generates
/// `Timer0`/`Timer1`.
macro_rules! timers {
    ($(
        $(#[$attr:meta])*
        $Timer:ident => $source:ident, $base:expr, $group:literal, $unit:literal, $index:literal, $shim:ident;
    )+) => {
        $(
            $(#[$attr])*
            pub struct $Timer<'d> {
                clkrst: &'d ClockRegs,
                irq: RefCell<Option<Irq>>,
            }

            impl $Timer<'static> {
                /// Create a copy of this timer without owning it.
                ///
                /// This is analogous to [`Pin::steal`](crate::gpio::Pin::steal):
                /// it lets an interrupt handler or another module reach the
                /// timer registers while the real owner lives elsewhere (for
                /// example in `main`). Nothing is written to the hardware, so a
                /// stolen copy never disturbs a running timer, and dropping it
                /// does not mask or stop anything.
                ///
                /// # Safety
                ///
                /// The caller must ensure that
                ///
                /// - a [`TimerGroup0`]/[`TimerGroup1`] has already been split,
                ///   so the clock is on and the prescaler is programmed;
                /// - the copy is not used to reconfigure the counter behind the
                ///   owner's back, and not used to claim a second CPU line with
                ///   [`enable_interrupt`](Self::enable_interrupt).
                #[inline]
                pub unsafe fn steal() -> Steal<Self> {
                    Steal(Self {
                        // SAFETY: the clock controller is always mapped; the
                        // caller guarantees the timer itself is initialised.
                        clkrst: unsafe { &*HP_SYS_CLKRST::PTR },
                        irq: RefCell::new(None),
                    })
                }
            }

            impl<'d> $Timer<'d> {
                /// The registers of this timer.
                const REGS: Regs = Regs::new($base, $unit);
                /// The interrupt matrix source feeding this timer.
                const SOURCE: Interrupt = Interrupt::$source;
                /// Index of this timer in the HAL's tables.
                const INDEX: usize = $index;

                /// Reset the timer to a known state: stopped, cleared, alarm
                /// disarmed, crystal clock divided by 40, counting up.
                fn init(clkrst: &'d ClockRegs) -> Self {
                    // The bootloader may have left the timer running with an
                    // alarm armed. Stop it before touching the prescaler, and
                    // open the clock gate first so the configuration lands.
                    Self::REGS.set_running(false);
                    program_clock(clkrst, $group, $unit, ClockSource::Xtal);
                    Self::REGS.set_divider(DEFAULT_DIVIDER);

                    Self::REGS.set_alarm_enabled(false);
                    Self::REGS.enable_interrupt(false);
                    Self::REGS.clear_interrupt();
                    Self::REGS.set_auto_reload(false);
                    Self::REGS.set_counting_up(true);
                    Self::REGS.set_reload_value(0);
                    Self::REGS.reload();

                    Self {
                        clkrst,
                        irq: RefCell::new(None),
                    }
                }

                /// The clock feeding this timer's prescaler.
                pub fn clock_source(&self) -> ClockSource {
                    clock_source_of(self.clkrst, $group, $unit)
                }

                /// Switch the timer to another clock source.
                ///
                /// The prescaler divisor stays as it is, so the tick rate
                /// changes with the source. Stop the counter first if it is
                /// running.
                pub fn set_clock_source(&self, clock: ClockSource) {
                    program_clock(self.clkrst, $group, $unit, clock);
                }

                /// The prescaler divisor, 2 to 65536.
                pub fn divider(&self) -> u32 {
                    Self::REGS.divider()
                }

                /// Reprogram the prescaler. The counter must be stopped.
                ///
                /// The counter clock is `clock_source().frequency_hz() / divider`;
                /// the default of 40 turns the 40 MHz crystal into a 1 MHz tick.
                pub fn set_divider(&self, divider: u32) {
                    assert!(
                        (2..=65536).contains(&divider),
                        "the prescaler divides by 2..=65536"
                    );
                    assert!(
                        !self.is_running(),
                        "stop the timer before changing its prescaler"
                    );
                    Self::REGS.set_divider(divider);
                }

                /// The frequency of the counter clock, in Hz.
                ///
                /// Rounded down, so a prescaler that does not divide the source
                /// exactly makes this slightly pessimistic.
                pub fn tick_hz(&self) -> u32 {
                    self.clock_source().frequency_hz() / self.divider()
                }

                /// Which way the counter runs.
                pub fn direction(&self) -> Direction {
                    if Self::REGS.counting_up() {
                        Direction::Up
                    } else {
                        Direction::Down
                    }
                }

                /// Count up or down. The change takes effect immediately.
                pub fn set_direction(&self, direction: Direction) {
                    Self::REGS.set_counting_up(direction == Direction::Up);
                }

                /// Start the counter.
                pub fn start(&self) {
                    Self::REGS.set_running(true);
                }

                /// Stop the counter; it keeps its value and resumes from it.
                pub fn stop(&self) {
                    Self::REGS.set_running(false);
                }

                /// Whether the counter is running.
                pub fn is_running(&self) -> bool {
                    Self::REGS.running()
                }

                /// Latch and return the 54-bit counter value, in timer ticks.
                pub fn counter(&self) -> u64 {
                    Self::REGS.counter()
                }

                /// Copy `value` into the counter immediately.
                pub fn set_counter(&self, value: u64) {
                    assert!(value <= MAX_VALUE, "the counter is 54 bits wide");
                    Self::REGS.set_reload_value(value);
                    Self::REGS.reload();
                }

                /// Set the value the counter reloads from, now or at the next
                /// alarm.
                ///
                /// This does not change the running counter; call
                /// [`reload`](Self::reload) or let an auto-reload alarm do it.
                pub fn set_reload_value(&self, value: u64) {
                    assert!(value <= MAX_VALUE, "the counter is 54 bits wide");
                    Self::REGS.set_reload_value(value);
                }

                /// Copy the reload value into the counter now.
                pub fn reload(&self) {
                    Self::REGS.reload();
                }

                /// Reload the counter from its reload value when the alarm
                /// fires.
                ///
                /// This also decides what the interrupt handler does: with
                /// auto-reload on it re-arms the alarm after acknowledging it,
                /// which is what makes [`start_periodic`](Self::start_periodic)
                /// periodic. If auto-reload is turned off while an alarm is
                /// armed, the next alarm is the last one.
                pub fn set_auto_reload(&self, enable: bool) {
                    Self::REGS.set_auto_reload(enable);
                }

                /// Whether the counter reloads itself when the alarm fires.
                pub fn auto_reload(&self) -> bool {
                    Self::REGS.auto_reload()
                }

                /// Arm the alarm for the absolute counter value `ticks`.
                ///
                /// A value the counter has already passed fires the alarm
                /// immediately (for an up-counter); use
                /// [`set_alarm_in`](Self::set_alarm_in) to avoid that.
                pub fn set_alarm(&self, ticks: u64) {
                    assert!(ticks <= MAX_VALUE, "the alarm is 54 bits wide");
                    Self::REGS.set_alarm_value(ticks);
                    // A request left over from an earlier alarm would fire
                    // immediately.
                    Self::REGS.clear_interrupt();
                    Self::REGS.set_alarm_enabled(true);
                }

                /// Arm the alarm for `delay` from now.
                pub fn set_alarm_in(&self, delay: Duration) {
                    let ticks = self.duration_to_ticks(delay);
                    let now = self.counter();
                    let deadline = match self.direction() {
                        Direction::Up => now.saturating_add(ticks),
                        Direction::Down => now.saturating_sub(ticks),
                    };
                    self.set_alarm(deadline);
                }

                /// Arm a one-shot alarm `delay` from now and start the counter.
                ///
                /// The alarm fires once; the counter keeps running past the
                /// alarm value unless auto-reload is enabled.
                pub fn start_alarm(&self, delay: Duration) {
                    self.set_auto_reload(false);
                    self.set_alarm_in(delay);
                    self.start();
                }

                /// Start a periodic alarm that repeats every `period`.
                ///
                /// The counter restarts from zero (or from `period`, when
                /// counting down) each time the alarm fires, so the period does
                /// not accumulate the delay of the interrupt handler.
                /// [`enable_interrupt`](Self::enable_interrupt) decides what
                /// runs at each alarm.
                pub fn start_periodic(&self, period: Duration) {
                    let ticks = self.duration_to_ticks(period);

                    self.stop();
                    match self.direction() {
                        Direction::Up => {
                            self.set_reload_value(0);
                            self.reload();
                            self.set_alarm(ticks);
                        }
                        Direction::Down => {
                            self.set_reload_value(ticks);
                            self.reload();
                            self.set_alarm(0);
                        }
                    }
                    self.set_auto_reload(true);
                    self.start();
                }

                /// Disarm the alarm. A request that is already pending stays
                /// pending until [`clear_alarm`](Self::clear_alarm).
                pub fn disarm(&self) {
                    Self::REGS.set_alarm_enabled(false);
                }

                /// Whether the alarm is armed.
                pub fn alarm_armed(&self) -> bool {
                    Self::REGS.alarm_enabled()
                }

                /// Whether the alarm fired since the last clear.
                pub fn alarm_pending(&self) -> bool {
                    Self::REGS.interrupt_pending()
                }

                /// Drop a pending alarm request.
                pub fn clear_alarm(&self) {
                    Self::REGS.clear_interrupt();
                }

                /// Route this timer's alarm to `line` and call `handler` in
                /// interrupt context.
                ///
                /// Claims the line the way [`interrupt::claim`] does, so the
                /// timer panics if the line is already taken. The alarm still
                /// has to be armed with [`set_alarm`](Self::set_alarm),
                /// [`start_alarm`](Self::start_alarm) or
                /// [`start_periodic`](Self::start_periodic), and global
                /// interrupts have to be enabled with [`interrupt::enable`].
                pub fn enable_interrupt(&self, line: CpuInterrupt, handler: fn()) {
                    assert!(
                        self.irq.borrow().is_none(),
                        "this timer already has an interrupt handler"
                    );

                    HANDLERS[Self::INDEX].store(handler as usize, Ordering::Release);

                    // SAFETY: the shim only touches this timer's status bits
                    // before calling the user's function, and `claim` makes sure
                    // no other driver uses the line.
                    let claimed =
                        unsafe { interrupt::claim(Self::SOURCE, line, Level::LOW, $shim) };
                    claimed.enable();
                    Self::REGS.clear_interrupt();
                    Self::REGS.enable_interrupt(true);
                    *self.irq.borrow_mut() = Some(Irq {
                        _handler: claimed,
                        regs: Self::REGS,
                        index: Self::INDEX,
                    });
                }

                /// Mask the alarm interrupt and give its CPU line back.
                pub fn disable_interrupt(&self) {
                    // Dropping the bookkeeping masks the timer, drops the
                    // request, forgets the shim and releases the line.
                    self.irq.borrow_mut().take();
                }

                /// Whether the alarm interrupt is unmasked.
                pub fn interrupt_enabled(&self) -> bool {
                    Self::REGS.interrupt_enabled()
                }

                /// Convert a span to timer ticks, rounding down but never to
                /// zero.
                fn duration_to_ticks(&self, duration: Duration) -> u64 {
                    let ticks =
                        duration.as_ticks() as u128 * self.tick_hz() as u128 / time::TICK_HZ as u128;
                    ticks.clamp(1, MAX_VALUE as u128) as u64
                }
            }
        )+
    };
}

timers! {
    /// The first timer of `TIMG0`.
    Tg0T0 => TG0_T0, GROUP0_BASE, 0, 0, 0, alarm0;
    /// The second timer of `TIMG0`.
    Tg0T1 => TG0_T1, GROUP0_BASE, 0, 1, 1, alarm1;
    /// The first timer of `TIMG1`.
    Tg1T0 => TG1_T0, GROUP1_BASE, 1, 0, 2, alarm2;
    /// The second timer of `TIMG1`.
    Tg1T1 => TG1_T1, GROUP1_BASE, 1, 1, 3, alarm3;
}
