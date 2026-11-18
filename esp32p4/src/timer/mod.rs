//! General-purpose timers (`TIMG0` / `TIMG1`).
//!
//! The chip has two timer groups, each with two independent 54-bit counters
//! (`T0`, `T1`) and one watchdog. A counter runs off `XTAL_CLK` (40 MHz),
//! `RC_FAST_CLK` (~17.5 MHz) or `PLL_F80M_CLK` (80 MHz) through a 16-bit
//! prescaler, counts up or down, can be reloaded by software, and can reload
//! itself from its reload value when its alarm fires. The default
//! configuration divides the 40 MHz crystal by 40, so one tick is one
//! microsecond.
//!
//! | Timer | Matrix source | Notes |
//! |-------|---------------|-------|
//! | `TIMG0` T0 / T1 | `Interrupt::TG0_T0` / `TG0_T1` | |
//! | `TIMG1` T0 / T1 | `Interrupt::TG1_T0` / `TG1_T1` | |
//!
//! Each timer has its own matrix source, so a handler does not have to work out
//! which timer fired; the two timers of a group still share the group's
//! interrupt enable/status registers, which [`Timer`] handles for you.
//!
//! Unlike `SYSTIMER`, these counters are not free-running across the whole
//! program: they stop, reload and reset on request. Use them for alarms and
//! periodic ticks; use [`crate::time::Instant`] for a monotonic clock.
//!
//! The vendor SVD does not describe timer 1 of either group, so this module
//! talks to the registers through a hand-written register layer (`regs.rs`),
//! derived from the TRM.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4::interrupt::{self, CpuInterrupt, Level};
//! use esp32p4::time::Duration;
//! use esp32p4::timer::TimerGroup0;
//!
//! fn tick() {
//!     // Runs in interrupt context: keep it short.
//! }
//!
//! let (mut timer, _unused) = TimerGroup0::new(dp.TIMG0, &dp.HP_SYS_CLKRST).split();
//! timer.enable_interrupt(CpuInterrupt::new(21), tick);
//! timer.start_periodic(Duration::from_millis(500));
//! interrupt::enable();
//! ```

use core::sync::atomic::{AtomicUsize, Ordering};

use esp32p4_pac::{HP_SYS_CLKRST, TIMG0, TIMG1};

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

/// The prescaler that makes one tick one microsecond out of the crystal.
const DEFAULT_DIVIDER: u32 = 40;

/// Number of general-purpose timers on the chip.
const TIMER_COUNT: usize = 4;

/// Register layout of every timer, indexed by `group * 2 + unit`.
const TIMERS: [Regs; TIMER_COUNT] = [
    Regs::new(GROUP0_BASE, 0),
    Regs::new(GROUP0_BASE, 1),
    Regs::new(GROUP1_BASE, 0),
    Regs::new(GROUP1_BASE, 1),
];

/// User callbacks, one per timer, called by the shims below.
static HANDLERS: [AtomicUsize; TIMER_COUNT] = [const { AtomicUsize::new(0) }; TIMER_COUNT];

/// Alarm dispatch shared by the four shims.
///
/// Acknowledge the timer, re-arm it if it is running in periodic mode, and call
/// the user's handler. The shim has to do the first two steps because the user's
/// handler is a bare `fn` and cannot reach the timer.
fn alarm(index: usize) {
    let regs = TIMERS[index];

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
        // `Timer::enable_interrupt`, which stores `fn()` pointers.
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

/// One shim per timer, so the trap handler knows which registers to poke.
const SHIMS: [fn(); TIMER_COUNT] = [alarm0, alarm1, alarm2, alarm3];

/// Selects the clock and opens the bus and timer gates for one timer.
///
/// Two gates are involved: `SOC_CLK_CTRL2` lets the CPU reach the group's
/// registers at all, and `PERI_CLK_CTRL2n` picks the source and gates the
/// counter clock of one timer. ESP-IDF opens both when a timer is claimed.
fn program_clock(clkrst: &HP_SYS_CLKRST, group: u8, unit: u8, clock: ClockSource) {
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

/// `TIMG0`: two general-purpose timers and the group's watchdog.
pub struct TimerGroup0<'d> {
    clkrst: &'d HP_SYS_CLKRST,
    _timg: TIMG0,
}

impl<'d> TimerGroup0<'d> {
    /// Take ownership of the group.
    pub fn new(timg: TIMG0, clkrst: &'d HP_SYS_CLKRST) -> Self {
        Self {
            clkrst,
            _timg: timg,
        }
    }

    /// Hand out the group's two timers, each reset and clocked from the crystal.
    pub fn split(self) -> (Timer<'d>, Timer<'d>) {
        (
            Timer::new(GROUP0_BASE, 0, 0, self.clkrst),
            Timer::new(GROUP0_BASE, 0, 1, self.clkrst),
        )
    }
}

/// `TIMG1`: two general-purpose timers and the group's watchdog.
pub struct TimerGroup1<'d> {
    clkrst: &'d HP_SYS_CLKRST,
    _timg: TIMG1,
}

impl<'d> TimerGroup1<'d> {
    /// Take ownership of the group.
    pub fn new(timg: TIMG1, clkrst: &'d HP_SYS_CLKRST) -> Self {
        Self {
            clkrst,
            _timg: timg,
        }
    }

    /// Hand out the group's two timers, each reset and clocked from the crystal.
    pub fn split(self) -> (Timer<'d>, Timer<'d>) {
        (
            Timer::new(GROUP1_BASE, 1, 0, self.clkrst),
            Timer::new(GROUP1_BASE, 1, 1, self.clkrst),
        )
    }
}

/// One general-purpose timer.
pub struct Timer<'d> {
    regs: Regs,
    group: u8,
    unit: u8,
    clock: ClockSource,
    divider: u32,
    clkrst: &'d HP_SYS_CLKRST,
    handler: Option<InterruptHandler>,
}

impl<'d> Timer<'d> {
    /// Reset the timer to a known state: stopped, cleared, alarm disarmed,
    /// crystal clock divided by 40, counting up.
    fn new(group_base: usize, group: u8, unit: u8, clkrst: &'d HP_SYS_CLKRST) -> Self {
        let regs = Regs::new(group_base, unit);
        let clock = ClockSource::Xtal;

        // The bootloader may have left the timer running with an alarm armed.
        // Stop it before touching the prescaler, and open the clock gate before
        // anything else so the configuration actually reaches the hardware.
        regs.set_running(false);
        program_clock(clkrst, group, unit, clock);
        regs.set_divider(DEFAULT_DIVIDER);

        regs.set_alarm_enabled(false);
        regs.enable_interrupt(false);
        regs.clear_interrupt();
        regs.set_auto_reload(false);
        regs.set_counting_up(true);
        regs.set_reload_value(0);
        regs.reload();

        Self {
            regs,
            group,
            unit,
            clock,
            divider: DEFAULT_DIVIDER,
            clkrst,
            handler: None,
        }
    }

    /// Index of this timer in the HAL's tables.
    fn index(&self) -> usize {
        self.group as usize * 2 + self.unit as usize
    }

    /// The interrupt matrix source feeding this timer.
    fn source(&self) -> Interrupt {
        match (self.group, self.unit) {
            (0, 0) => Interrupt::TG0_T0,
            (0, _) => Interrupt::TG0_T1,
            (1, 0) => Interrupt::TG1_T0,
            _ => Interrupt::TG1_T1,
        }
    }

    /// The clock feeding this timer's prescaler.
    pub fn clock_source(&self) -> ClockSource {
        self.clock
    }

    /// Switch the timer to another clock source.
    ///
    /// The prescaler divisor stays as it is, so the tick rate changes with the
    /// source. Stop the counter first if it is running.
    pub fn set_clock_source(&mut self, clock: ClockSource) {
        program_clock(self.clkrst, self.group, self.unit, clock);
        self.clock = clock;
    }

    /// The prescaler divisor, 2 to 65536.
    pub fn divider(&self) -> u32 {
        self.divider
    }

    /// Reprogram the prescaler. The counter must be stopped.
    ///
    /// The counter clock is `clock_source().frequency_hz() / divider`; the
    /// default of 40 turns the 40 MHz crystal into a 1 MHz tick.
    pub fn set_divider(&mut self, divider: u32) {
        assert!(
            (2..=65536).contains(&divider),
            "the prescaler divides by 2..=65536"
        );
        assert!(
            !self.regs.running(),
            "stop the timer before changing its prescaler"
        );
        self.regs.set_divider(divider);
        self.divider = divider;
    }

    /// The frequency of the counter clock, in Hz.
    ///
    /// Rounded down, so a prescaler that does not divide the source exactly
    /// makes this slightly pessimistic.
    pub fn tick_hz(&self) -> u32 {
        self.clock.frequency_hz() / self.divider
    }

    /// Which way the counter runs.
    pub fn direction(&self) -> Direction {
        if self.regs.counting_up() {
            Direction::Up
        } else {
            Direction::Down
        }
    }

    /// Count up or down. The change takes effect immediately.
    pub fn set_direction(&mut self, direction: Direction) {
        self.regs.set_counting_up(direction == Direction::Up);
    }

    /// Start the counter.
    pub fn start(&mut self) {
        self.regs.set_running(true);
    }

    /// Stop the counter; it keeps its value and resumes from it.
    pub fn stop(&mut self) {
        self.regs.set_running(false);
    }

    /// Whether the counter is running.
    pub fn is_running(&self) -> bool {
        self.regs.running()
    }

    /// Latch and return the 54-bit counter value, in timer ticks.
    pub fn counter(&self) -> u64 {
        self.regs.counter()
    }

    /// Copy `value` into the counter immediately.
    pub fn set_counter(&mut self, value: u64) {
        assert!(value <= MAX_VALUE, "the counter is 54 bits wide");
        self.regs.set_reload_value(value);
        self.regs.reload();
    }

    /// Set the value the counter reloads from, now or at the next alarm.
    ///
    /// This does not change the running counter; call [`Timer::reload`] or let
    /// an auto-reload alarm do it.
    pub fn set_reload_value(&mut self, value: u64) {
        assert!(value <= MAX_VALUE, "the counter is 54 bits wide");
        self.regs.set_reload_value(value);
    }

    /// Copy the reload value into the counter now.
    pub fn reload(&mut self) {
        self.regs.reload();
    }

    /// Reload the counter from its reload value when the alarm fires.
    ///
    /// This also decides what the interrupt handler does: with auto-reload on
    /// it re-arms the alarm after acknowledging it, which is what makes
    /// [`Timer::start_periodic`] periodic. If auto-reload is turned off while
    /// an alarm is armed, the next alarm is the last one.
    pub fn set_auto_reload(&mut self, enable: bool) {
        self.regs.set_auto_reload(enable);
    }

    /// Whether the counter reloads itself when the alarm fires.
    pub fn auto_reload(&self) -> bool {
        self.regs.auto_reload()
    }

    /// Arm the alarm for the absolute counter value `ticks`.
    ///
    /// A value the counter has already passed fires the alarm immediately (for
    /// an up-counter); use [`Timer::set_alarm_in`] to avoid that.
    pub fn set_alarm(&mut self, ticks: u64) {
        assert!(ticks <= MAX_VALUE, "the alarm is 54 bits wide");
        self.regs.set_alarm_value(ticks);
        // A request left over from an earlier alarm would fire immediately.
        self.regs.clear_interrupt();
        self.regs.set_alarm_enabled(true);
    }

    /// Arm the alarm for `delay` from now.
    pub fn set_alarm_in(&mut self, delay: Duration) {
        let ticks = self.duration_to_ticks(delay);
        let now = self.regs.counter();
        let deadline = match self.direction() {
            Direction::Up => now.saturating_add(ticks),
            Direction::Down => now.saturating_sub(ticks),
        };
        self.set_alarm(deadline);
    }

    /// Arm a one-shot alarm `delay` from now and start the counter.
    ///
    /// The alarm fires once; the counter keeps running past the alarm value
    /// unless auto-reload is enabled.
    pub fn start_alarm(&mut self, delay: Duration) {
        self.regs.set_auto_reload(false);
        self.set_alarm_in(delay);
        self.start();
    }

    /// Start a periodic alarm that repeats every `period`.
    ///
    /// The counter restarts from zero (or from `period`, when counting down)
    /// each time the alarm fires, so the period does not accumulate the delay
    /// of the interrupt handler. [`Timer::enable_interrupt`] decides what runs
    /// at each alarm.
    pub fn start_periodic(&mut self, period: Duration) {
        let ticks = self.duration_to_ticks(period);

        self.regs.set_running(false);
        match self.direction() {
            Direction::Up => {
                self.regs.set_reload_value(0);
                self.regs.reload();
                self.set_alarm(ticks);
            }
            Direction::Down => {
                self.regs.set_reload_value(ticks);
                self.regs.reload();
                self.set_alarm(0);
            }
        }
        self.regs.set_auto_reload(true);
        self.start();
    }

    /// Disarm the alarm. A request that is already pending stays pending until
    /// [`Timer::clear_alarm`].
    pub fn disarm(&mut self) {
        self.regs.set_alarm_enabled(false);
    }

    /// Whether the alarm is armed.
    pub fn alarm_armed(&self) -> bool {
        self.regs.alarm_enabled()
    }

    /// Whether the alarm fired since the last clear.
    pub fn alarm_pending(&self) -> bool {
        self.regs.interrupt_pending()
    }

    /// Drop a pending alarm request.
    pub fn clear_alarm(&mut self) {
        self.regs.clear_interrupt();
    }

    /// Route this timer's alarm to `line` and call `handler` in interrupt
    /// context.
    ///
    /// Claims the line the way [`interrupt::claim`] does, so the timer panics if
    /// the line is already taken. The alarm still has to be armed with
    /// [`Timer::set_alarm`], [`Timer::start_alarm`] or
    /// [`Timer::start_periodic`], and global interrupts have to be enabled with
    /// [`interrupt::enable`].
    pub fn enable_interrupt(&mut self, line: CpuInterrupt, handler: fn()) {
        assert!(
            self.handler.is_none(),
            "this timer already has an interrupt handler"
        );

        let index = self.index();
        HANDLERS[index].store(handler as usize, Ordering::Release);

        // SAFETY: the shim only touches this timer's status bits before calling
        // the user's function, and `claim` makes sure no other driver uses the
        // line.
        let claimed = unsafe { interrupt::claim(self.source(), line, Level::LOW, SHIMS[index]) };
        claimed.enable();
        self.regs.clear_interrupt();
        self.regs.enable_interrupt(true);
        self.handler = Some(claimed);
    }

    /// Mask the alarm interrupt and give its CPU line back.
    pub fn disable_interrupt(&mut self) {
        self.regs.enable_interrupt(false);
        self.regs.clear_interrupt();
        HANDLERS[self.index()].store(0, Ordering::Release);
        // Dropping the handler masks the line, forgets the shim and releases it.
        self.handler = None;
    }

    /// Whether the alarm interrupt is unmasked.
    pub fn interrupt_enabled(&self) -> bool {
        self.regs.interrupt_enabled()
    }

    /// Convert a span to timer ticks, rounding down but never to zero.
    fn duration_to_ticks(&self, duration: Duration) -> u64 {
        let ticks = duration.as_ticks() as u128 * self.tick_hz() as u128 / time::TICK_HZ as u128;
        ticks.clamp(1, MAX_VALUE as u128) as u64
    }
}

impl Drop for Timer<'_> {
    fn drop(&mut self) {
        // A dropped timer must not keep running or poking the interrupt matrix.
        self.regs.set_alarm_enabled(false);
        self.regs.enable_interrupt(false);
        self.regs.clear_interrupt();
        self.regs.set_running(false);
        HANDLERS[self.index()].store(0, Ordering::Release);
    }
}
