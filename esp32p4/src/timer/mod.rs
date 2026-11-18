//! General-purpose timers and the main system watchdog (`TIMG0` / `TIMG1`).
//!
//! The chip has two timer groups. Each group contains two independent 54-bit
//! general-purpose counters and one main system watchdog (MWDT):
//!
//! ```text
//! TimerGroup0 ─┬─ Timer<0, 0>   (matrix source TG0_T0, CPU line is your choice)
//!              ├─ Timer<0, 1>   (TG0_T1)
//!              └─ Watchdog<0>   (TG0_WDT)
//! TimerGroup1 ─┬─ Timer<1, 0>   (TG1_T0)
//!              ├─ Timer<1, 1>   (TG1_T1)
//!              └─ Watchdog<1>   (TG1_WDT)
//! ```
//!
//! A counter runs off `XTAL_CLK` (40 MHz), `RC_FAST_CLK` (~17.5 MHz) or
//! `PLL_F80M_CLK` (80 MHz) through a 16-bit prescaler. The default divides the
//! 40 MHz crystal by 40, so one tick is one microsecond.
//!
//! # Using a timer
//!
//! Nothing starts on its own: configure the counter, then call [`Timer::start`].
//!
//! ```ignore
//! use esp32p4::interrupt;
//! use esp32p4::time::Duration;
//! use esp32p4::timer::TimerGroup0;
//!
//! fn tick() {
//!     // Runs in interrupt context: keep it short.
//! }
//!
//! let group = TimerGroup0::new(dp.TIMG0, &dp.HP_SYS_CLKRST);
//! let timer = &group.timer0;
//!
//! let ticks = timer.ticks(Duration::from_millis(100));
//! timer.set_reload_value(0);       // where the counter restarts
//! timer.set_alarm(ticks);          // fire at 100 ms
//! timer.set_auto_reload(true);     // and again, every 100 ms
//! timer.enable_interrupt(CpuInterrupt::new(21), tick);  // CPU line 21
//! timer.start();                   // reset the counter and run
//! interrupt::enable();
//! ```
//!
//! Every method takes `&self`, so a timer behind a shared reference is enough,
//! and `unsafe { Timer::<0, 0>::steal() }` hands the very same type to an
//! interrupt handler or another module. A stolen timer never touches the
//! hardware: it neither starts the counter nor owns the CPU line.
//!
//! # The watchdog
//!
//! [`Watchdog`] is the MWDT: a two-to-four stage countdown that can raise an
//! interrupt or reset the CPU or the whole system. It is configured through the
//! group, and needs feeding before the first stage runs out.
//!
//! The vendor SVD does not describe timer 1 of either group, so the counters
//! are reached through a hand-written register layer (`regs.rs`) derived from
//! the TRM. The watchdog registers are part of the same layer so that one
//! implementation serves both groups.

use core::cell::RefCell;
use core::sync::atomic::{AtomicUsize, Ordering};

use esp32p4_pac::{HP_SYS_CLKRST, TIMG0, TIMG1};

use crate::Steal;
use crate::interrupt::{self, CpuInterrupt, Interrupt, InterruptHandler, Level};
use crate::time::{self, Duration};

mod regs;

use regs::{MAX_VALUE, Regs, WdtRegs, group_base};

/// The clock controller's registers, as the timers see them.
type ClockRegs = esp32p4_pac::hp_sys_clkrst::RegisterBlock;

/// A clock a timer or the watchdog can prescale.
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

    /// The value the clock controller wants in a `*_SRC_SEL` field.
    const fn select(self) -> u32 {
        match self {
            Self::Xtal => 0,
            Self::RcFast => 1,
            Self::PllF80m => 2,
        }
    }

    /// The source a `*_SRC_SEL` value selects.
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

/// A stage of the main system watchdog.
///
/// The stages run one after another: when stage 0 times out its action is
/// taken, and unless that was a reset, stage 1 starts counting, and so on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MwdtStage {
    /// First stage, the one a feed restarts.
    Stage0,
    /// Second stage.
    Stage1,
    /// Third stage.
    Stage2,
    /// Fourth stage.
    Stage3,
}

impl MwdtStage {
    /// The stage's number, 0 to 3.
    const fn number(self) -> u8 {
        match self {
            Self::Stage0 => 0,
            Self::Stage1 => 1,
            Self::Stage2 => 2,
            Self::Stage3 => 3,
        }
    }
}

/// What a watchdog stage does when it times out.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MwdtStageAction {
    /// Do nothing and let the next stage run.
    Off,
    /// Raise the group's watchdog interrupt.
    ///
    /// The interrupt is level-triggered and only drops when the watchdog is
    /// fed; this HAL does not route it, so use it with your own handler on
    /// `Interrupt::TG0_WDT` / `TG1_WDT`.
    Interrupt,
    /// Reset the CPU core that is running the watchdog.
    ResetCpu,
    /// Reset the whole system.
    ResetSystem,
}

impl MwdtStageAction {
    /// The value the hardware wants for this action.
    const fn select(self) -> u32 {
        match self {
            Self::Off => 0,
            Self::Interrupt => 1,
            Self::ResetCpu => 2,
            Self::ResetSystem => 3,
        }
    }
}

/// The prescaler that makes one tick one microsecond out of the crystal.
const DEFAULT_DIVIDER: u32 = 40;

/// Frequency the watchdog counts at: `XTAL_CLK`, which is what the group
/// initialisation selects.
const WDT_CLOCK_HZ: u32 = 40_000_000;

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

/// The clock controller's register block.
///
/// The group constructors take the PAC token, but the timers and the watchdog
/// are reachable after `new` has returned (and from a stolen copy), so the
/// helpers read the block through its address.
fn clock_regs() -> &'static ClockRegs {
    // SAFETY: the clock controller is always mapped and lives for the program.
    unsafe { &*HP_SYS_CLKRST::PTR }
}

/// Selects the clock and opens the timer gate of one counter.
///
/// Two gates are involved: `SOC_CLK_CTRL2` lets the CPU reach the group's
/// registers at all, and `PERI_CLK_CTRL2n` picks the source and gates the
/// counter clock of one timer. ESP-IDF opens both when a timer is claimed.
fn program_clock(clkrst: &ClockRegs, group: u8, unit: u8, clock: ClockSource) {
    let source = clock.select();
    if group == 0 {
        let ctrl = &clkrst.peri_clk_ctrl20;
        if unit == 0 {
            ctrl.write_reg_timergrp0_t0_src_sel(source);
            ctrl.set_reg_timergrp0_t0_clk_en();
        } else {
            ctrl.write_reg_timergrp0_t1_src_sel(source);
            ctrl.set_reg_timergrp0_t1_clk_en();
        }
    } else {
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

/// `TIMG0`: two general-purpose timers and the main system watchdog.
pub struct TimerGroup0 {
    /// General-purpose timer 0, the source of `TG0_T0`.
    pub timer0: Timer<0, 0>,
    /// General-purpose timer 1, the source of `TG0_T1`.
    pub timer1: Timer<0, 1>,
    /// The group's main system watchdog.
    pub wdt: Watchdog<0>,
}

impl TimerGroup0 {
    /// Take the group and bring it into a known state.
    ///
    /// Opens the group's clock gates, selects the crystal for both counters and
    /// the watchdog, and leaves the counters stopped with their alarms
    /// disarmed. The bootloader's watchdogs are already off by then (see
    /// [`crate::startup`]); this does not re-enable them.
    pub fn new(_timg: TIMG0, clkrst: &HP_SYS_CLKRST) -> Self {
        // The token is consumed so that no second group can be built from the
        // same peripheral; the registers themselves are reached by address.
        init_group(0, clkrst);

        Self {
            timer0: Timer::new(),
            timer1: Timer::new(),
            wdt: Watchdog::new(),
        }
    }
}

/// `TIMG1`: two general-purpose timers and the main system watchdog.
pub struct TimerGroup1 {
    /// General-purpose timer 0, the source of `TG1_T0`.
    pub timer0: Timer<1, 0>,
    /// General-purpose timer 1, the source of `TG1_T1`.
    pub timer1: Timer<1, 1>,
    /// The group's main system watchdog.
    pub wdt: Watchdog<1>,
}

impl TimerGroup1 {
    /// Take the group and bring it into a known state.
    ///
    /// See [`TimerGroup0::new`].
    pub fn new(_timg: TIMG1, clkrst: &HP_SYS_CLKRST) -> Self {
        init_group(1, clkrst);

        Self {
            timer0: Timer::new(),
            timer1: Timer::new(),
            wdt: Watchdog::new(),
        }
    }
}

/// Opens the clocks and resets both counters of one group.
fn init_group(group: u8, clkrst: &HP_SYS_CLKRST) {
    let clkrst = &**clkrst;

    // The APB gate lets the CPU reach the group at all; the watchdog has its
    // own gate and source selector.
    if group == 0 {
        clkrst.soc_clk_ctrl2.set_reg_timergrp0_apb_clk_en();
        clkrst.peri_clk_ctrl20.set_reg_timergrp0_wdt_clk_en();
        clkrst
            .peri_clk_ctrl20
            .write_reg_timergrp0_wdt_src_sel(ClockSource::Xtal.select());
    } else {
        clkrst.soc_clk_ctrl2.set_reg_timergrp1_apb_clk_en();
        clkrst.peri_clk_ctrl21.set_reg_timergrp1_wdt_clk_en();
        clkrst
            .peri_clk_ctrl21
            .write_reg_timergrp1_wdt_src_sel(ClockSource::Xtal.select());
    }

    for unit in 0..2u8 {
        let regs = Regs::new(group_base(group), unit);

        // The bootloader may have left the timer running with an alarm armed.
        // Stop it before touching the prescaler, and open the clock gate first
        // so the configuration lands.
        regs.set_running(false);
        program_clock(clkrst, group, unit, ClockSource::Xtal);
        regs.set_divider(DEFAULT_DIVIDER);

        regs.set_alarm_enabled(false);
        regs.enable_interrupt(false);
        regs.clear_interrupt();
        regs.set_auto_reload(false);
        regs.set_counting_up(true);
        regs.set_reload_value(0);
        regs.reload();
    }

    // The bootloader may leave the watchdog configuration unlocked. Give it a
    // known stage layout while the registers are writable: stage 0 resets the
    // system, the others are off.
    WdtRegs::new(group_base(group)).with_unlocked(|wdt| {
        wdt.set_defaults();
        wdt.update();
    });
}

/// Compile-time identity of one general-purpose timer.
///
/// `G` is the timer group (0 or 1) and `N` the timer inside it (0 or 1), so
/// `Timer<1, 0>` is `TIMG1`'s first counter.
pub struct Timer<const G: u8, const N: u8> {
    irq: RefCell<Option<Irq>>,
}

impl<const G: u8, const N: u8> Timer<G, N> {
    /// The registers of this timer.
    const fn regs() -> Regs {
        Regs::at(G as usize * 2 + N as usize)
    }

    /// The interrupt matrix source feeding this timer.
    const fn source() -> Interrupt {
        match (G, N) {
            (0, 0) => Interrupt::TG0_T0,
            (0, 1) => Interrupt::TG0_T1,
            (1, 0) => Interrupt::TG1_T0,
            _ => Interrupt::TG1_T1,
        }
    }

    /// Index of this timer in the HAL's tables.
    const fn index() -> usize {
        G as usize * 2 + N as usize
    }

    const fn new() -> Self {
        Self {
            irq: RefCell::new(None),
        }
    }

    /// Create a copy of this timer without owning it.
    ///
    /// This is analogous to [`Pin::steal`](crate::gpio::Pin::steal): it lets an
    /// interrupt handler or another module reach the timer registers while the
    /// real owner lives elsewhere (for example in `main`). Nothing is written to
    /// the hardware, so a stolen copy never disturbs a running timer, and
    /// dropping it neither stops the counter nor touches the interrupt.
    ///
    /// # Safety
    ///
    /// The caller must ensure that
    ///
    /// - a [`TimerGroup0`]/[`TimerGroup1`] has already been created, so the
    ///   clock is on and the prescaler is programmed;
    /// - the copy is not used to reconfigure the counter behind the owner's
    ///   back, and not used to claim a second CPU line with
    ///   [`enable_interrupt`](Self::enable_interrupt).
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        const { assert!(G < 2 && N < 2, "no such general-purpose timer") };
        Steal(Self::new())
    }

    /// The clock feeding this timer's prescaler.
    pub fn clock_source(&self) -> ClockSource {
        clock_source_of(clock_regs(), G, N)
    }

    /// Switch the timer to another clock source.
    ///
    /// The prescaler divisor stays as it is, so the tick rate changes with the
    /// source. Stop the counter first if it is running.
    pub fn set_clock_source(&self, clock: ClockSource) {
        program_clock(clock_regs(), G, N, clock);
    }

    /// The prescaler divisor, 2 to 65536.
    pub fn divider(&self) -> u32 {
        Self::regs().divider()
    }

    /// Reprogram the prescaler. The counter must be stopped.
    ///
    /// The counter clock is `clock_source().frequency_hz() / divider`; the
    /// default of 40 turns the 40 MHz crystal into a 1 MHz tick.
    pub fn set_divider(&self, divider: u32) {
        assert!(
            (2..=65536).contains(&divider),
            "the prescaler divides by 2..=65536"
        );
        assert!(
            !self.is_running(),
            "stop the timer before changing its prescaler"
        );
        Self::regs().set_divider(divider);
    }

    /// The frequency of the counter clock, in Hz.
    ///
    /// Rounded down, so a prescaler that does not divide the source exactly
    /// makes this slightly pessimistic.
    pub fn tick_hz(&self) -> u32 {
        self.clock_source().frequency_hz() / self.divider()
    }

    /// Convert a span into this timer's ticks, rounding down but never to zero.
    ///
    /// Counter and alarm values are in timer ticks, which is why the duration
    /// has to be converted: at the default 1 MHz one tick is one microsecond,
    /// but a different clock or prescaler changes that.
    pub fn ticks(&self, duration: Duration) -> u64 {
        let ticks = duration.as_ticks() as u128 * self.tick_hz() as u128 / time::TICK_HZ as u128;
        ticks.clamp(1, MAX_VALUE as u128) as u64
    }

    /// Which way the counter runs.
    pub fn direction(&self) -> Direction {
        if Self::regs().counting_up() {
            Direction::Up
        } else {
            Direction::Down
        }
    }

    /// Count up or down. The change takes effect immediately.
    pub fn set_direction(&self, direction: Direction) {
        Self::regs().set_counting_up(direction == Direction::Up);
    }

    /// Reset the counter to its reload value and start counting.
    ///
    /// The counter is stopped while it is reloaded, so a restart is
    /// deterministic: the first tick after `start` is tick one. The alarm is
    /// not armed here — [`set_alarm`](Self::set_alarm) does that.
    pub fn start(&self) {
        self.stop();
        self.reload();
        Self::regs().set_running(true);
    }

    /// Stop the counter; it keeps its value and resumes from it.
    pub fn stop(&self) {
        Self::regs().set_running(false);
    }

    /// Whether the counter is running.
    pub fn is_running(&self) -> bool {
        Self::regs().running()
    }

    /// Latch and return the 54-bit counter value, in timer ticks.
    pub fn counter(&self) -> u64 {
        Self::regs().counter()
    }

    /// Copy `value` into the counter immediately.
    pub fn set_counter(&self, value: u64) {
        assert!(value <= MAX_VALUE, "the counter is 54 bits wide");
        Self::regs().set_reload_value(value);
        Self::regs().reload();
    }

    /// The value the counter starts from.
    pub fn reload_value(&self) -> u64 {
        Self::regs().reload_value()
    }

    /// Set the value the counter starts from: on [`start`](Self::start), on
    /// [`reload`](Self::reload), and every time the alarm fires with auto-reload
    /// on.
    pub fn set_reload_value(&self, value: u64) {
        assert!(value <= MAX_VALUE, "the counter is 54 bits wide");
        Self::regs().set_reload_value(value);
    }

    /// Copy the reload value into the counter now.
    pub fn reload(&self) {
        Self::regs().reload();
    }

    /// Reload the counter from its reload value when the alarm fires.
    ///
    /// This also decides what the interrupt handler does: with auto-reload on
    /// it re-arms the alarm after acknowledging it, which is what makes a
    /// periodic alarm repeat. If auto-reload is turned off while an alarm is
    /// armed, the next alarm is the last one.
    pub fn set_auto_reload(&self, enable: bool) {
        Self::regs().set_auto_reload(enable);
    }

    /// Whether the counter reloads itself when the alarm fires.
    pub fn auto_reload(&self) -> bool {
        Self::regs().auto_reload()
    }

    /// Arm the alarm for the absolute counter value `ticks`.
    ///
    /// Combine with [`set_reload_value`](Self::set_reload_value) and
    /// [`set_auto_reload`](Self::set_auto_reload) for a periodic alarm, or with
    /// [`set_alarm_in`](Self::set_alarm_in) for one measured from now.
    pub fn set_alarm(&self, ticks: u64) {
        assert!(ticks <= MAX_VALUE, "the alarm is 54 bits wide");
        Self::regs().set_alarm_value(ticks);
        // A request left over from an earlier alarm would fire immediately.
        Self::regs().clear_interrupt();
        Self::regs().set_alarm_enabled(true);
    }

    /// The armed alarm value, in timer ticks.
    pub fn alarm(&self) -> u64 {
        Self::regs().alarm_value()
    }

    /// Arm the alarm for `delay` from the current counter value.
    ///
    /// The counter is not reset, so this is the one-shot counterpart of
    /// [`set_alarm`](Self::set_alarm).
    pub fn set_alarm_in(&self, delay: Duration) {
        let ticks = self.ticks(delay);
        let now = self.counter();
        let deadline = match self.direction() {
            Direction::Up => now.saturating_add(ticks),
            Direction::Down => now.saturating_sub(ticks),
        };
        self.set_alarm(deadline);
    }

    /// Disarm the alarm. A request that is already pending stays pending until
    /// [`clear_alarm`](Self::clear_alarm).
    pub fn disarm(&self) {
        Self::regs().set_alarm_enabled(false);
    }

    /// Whether the alarm is armed.
    pub fn alarm_armed(&self) -> bool {
        Self::regs().alarm_enabled()
    }

    /// Whether the alarm fired since the last clear.
    pub fn alarm_pending(&self) -> bool {
        Self::regs().interrupt_pending()
    }

    /// Drop a pending alarm request.
    pub fn clear_alarm(&self) {
        Self::regs().clear_interrupt();
    }

    /// Route this timer's alarm to `line` and call `handler` in interrupt
    /// context.
    ///
    /// Claims the line the way [`interrupt::claim`] does, so the timer panics if
    /// the line is already taken. The alarm still has to be armed with
    /// [`set_alarm`](Self::set_alarm) or [`set_alarm_in`](Self::set_alarm_in),
    /// and global interrupts have to be enabled with [`interrupt::enable`].
    pub fn enable_interrupt(&self, line: CpuInterrupt, handler: fn()) {
        assert!(
            self.irq.borrow().is_none(),
            "this timer already has an interrupt handler"
        );

        HANDLERS[Self::index()].store(handler as usize, Ordering::Release);

        // SAFETY: the shim only touches this timer's status bits before calling
        // the user's function, and `claim` makes sure no other driver uses the
        // line.
        let claimed =
            unsafe { interrupt::claim(Self::source(), line, Level::LOW, SHIMS[Self::index()]) };
        claimed.enable();
        Self::regs().clear_interrupt();
        Self::regs().enable_interrupt(true);
        *self.irq.borrow_mut() = Some(Irq {
            _handler: claimed,
            regs: Self::regs(),
            index: Self::index(),
        });
    }

    /// Mask the alarm interrupt and give its CPU line back.
    pub fn disable_interrupt(&self) {
        // Dropping the bookkeeping masks the timer, drops the request, forgets
        // the shim and releases the line.
        self.irq.borrow_mut().take();
    }

    /// Whether the alarm interrupt is unmasked.
    pub fn interrupt_enabled(&self) -> bool {
        Self::regs().interrupt_enabled()
    }
}

/// The main system watchdog of one timer group.
///
/// The MWDT counts at `XTAL_CLK` (40 MHz) divided by a prescaler, through up to
/// four stages. [`enable`](Self::enable) installs a plain configuration — stage
/// 0 resets the system, the others are off — and starts counting; from then on
/// the program has to [`feed`](Self::feed) it before the stage runs out.
///
/// [`crate::startup`] disables the watchdogs the bootloader leaves running, so
/// a program that never touches this type cannot be reset by them.
pub struct Watchdog<const G: u8> {
    _private: (),
}

impl<const G: u8> Watchdog<G> {
    /// The registers of this group's watchdog.
    const fn regs() -> WdtRegs {
        WdtRegs::new(group_base(G))
    }

    const fn new() -> Self {
        Self { _private: () }
    }

    /// Whether the watchdog is counting.
    pub fn is_enabled(&self) -> bool {
        Self::regs().enabled()
    }

    /// Start the watchdog.
    ///
    /// The stage layout is the one [`TimerGroup0::new`] left behind: stage 0
    /// resets the system, stages 1 to 3 are off. Use
    /// [`set_stage_action`](Self::set_stage_action) to change that and
    /// [`set_timeout`](Self::set_timeout) to set how long each stage runs; both
    /// work before or after this call. Enabling also turns off the flash-boot
    /// protection, which would otherwise keep counting while disabled.
    pub fn enable(&mut self) {
        Self::regs().with_unlocked(|regs| {
            regs.disable_flashboot();
            regs.set_enabled(true);
            regs.update();
        });
    }

    /// Stop the watchdog.
    ///
    /// This also drops the flash-boot protection, which is the part that can
    /// reset the chip while the counter is stopped.
    pub fn disable(&mut self) {
        Self::regs().with_unlocked(|regs| {
            regs.set_enabled(false);
            regs.disable_flashboot();
            regs.update();
        });
    }

    /// Restart the countdown from the beginning of stage 0.
    pub fn feed(&mut self) {
        Self::regs().with_unlocked(|regs| regs.feed());
    }

    /// Set the timeout of one stage.
    ///
    /// The watchdog counts at 40 MHz; the prescaler is widened automatically so
    /// timeouts longer than the 32-bit tick register allows still fit.
    pub fn set_timeout(&mut self, stage: MwdtStage, timeout: Duration) {
        let ticks = timeout.as_ticks() as u128 * WDT_CLOCK_HZ as u128 / time::TICK_HZ as u128;
        assert!(ticks > 0, "a watchdog timeout must not be zero");

        let prescaler = ticks.div_ceil(u32::MAX as u128).clamp(1, u16::MAX as u128) as u32;
        let hold = ticks.div_ceil(prescaler as u128);
        assert!(
            hold <= u32::MAX as u128,
            "watchdog timeout too long for the 16-bit prescaler"
        );
        let hold = hold as u32;

        Self::regs().with_unlocked(|regs| {
            regs.set_prescaler(prescaler);
            regs.set_stage_timeout(stage.number(), hold);
            regs.update();
        });
    }

    /// Choose what one stage does when it times out.
    pub fn set_stage_action(&mut self, stage: MwdtStage, action: MwdtStageAction) {
        Self::regs().with_unlocked(|regs| {
            regs.set_stage_action(stage.number(), action.select());
            regs.update();
        });
    }
}
