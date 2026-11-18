//! General-purpose timers and the main system watchdog (`TIMG0` / `TIMG1`).
//!
//! The chip has two timer groups. Each group contains two independent 54-bit
//! general-purpose counters and one main system watchdog (MWDT):
//!
//! ```text
//! TimerGroup0 ─┬─ Timer<0, 0>   (matrix source TG0_T0)
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
//! [`TimerGroup0::new`] brings the group into a known state and hands out its
//! parts the way the hardware has them: `group.timer0`, `group.timer1` and
//! `group.wdt`. Nothing starts on its own: configure the counter, then call
//! [`Timer::start`].
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
//! let group = TimerGroup0::new(dp.TIMG0, &dp.HP_SYS_CLKRST);
//! let timer = &group.timer0;
//!
//! timer.enable_interrupt(CpuInterrupt::new(21), tick);  // CPU line 21
//! timer.set_reload_value(0);                            // restart from here
//! timer.set_alarm(timer.ticks(Duration::from_millis(100)));
//! timer.set_auto_reload(true);                          // and again, every 100 ms
//! timer.start();
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
//! [`Watchdog`] is the MWDT: a four-stage countdown that can raise an interrupt
//! or reset the CPU or the whole system. It is configured through the group, and
//! needs feeding before the first stage runs out.
//!
//! # Registers
//!
//! Everything goes through `esp32p4-pac`. The vendor SVD stops at timer 0 of
//! each group and leaves timer 1 in a reserved hole, so `timg.svd` completes the
//! two blocks (see `xtask/gen_timg.py`) and `cargo xtask gen-pac` merges them
//! over the vendor's peripherals. Both groups are the same IP, so the driver
//! views group 1 through group 0's register block type.

use core::cell::RefCell;
use core::sync::atomic::{AtomicUsize, Ordering};

use esp32p4_pac::{TIMG0, TIMG1, hp_sys_clkrst, timg0};
use pastey::paste;

use crate::Steal;
use crate::interrupt::{self, CpuInterrupt, Interrupt, InterruptHandler, Level};
use crate::time::{self, Duration};

/// The clock controller's registers, as the timers see them.
type Clkrst = hp_sys_clkrst::RegisterBlock;

/// The two groups are the same IP, so one register block type covers both.
const _: () = assert!(
    core::mem::size_of::<timg0::RegisterBlock>()
        == core::mem::size_of::<esp32p4_pac::timg1::RegisterBlock>(),
    "the timer groups must share a register layout"
);

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

/// Widest counter and alarm value: 22 high bits and 32 low bits.
const MAX_VALUE: u64 = (1 << 54) - 1;

/// Frequency the watchdog counts at: `XTAL_CLK`, selected by
/// [`TimerGroup0::new`].
const WDT_CLOCK_HZ: u32 = 40_000_000;

/// Value that unlocks the watchdog's configuration registers.
const WDT_UNLOCK_KEY: u32 = 0x50d8_3aa1;

/// Number of general-purpose timers on the chip.
const TIMER_COUNT: usize = 4;

/// User callbacks, one per timer, called by the shims below.
static HANDLERS: [AtomicUsize; TIMER_COUNT] = [const { AtomicUsize::new(0) }; TIMER_COUNT];

/// The register block of timer group `G`.
fn block<const G: u8>() -> &'static timg0::RegisterBlock {
    const { assert!(G < 2, "no such timer group") };
    // SAFETY: both groups are the same IP at fixed addresses, and the layout
    // check above keeps the cast honest.
    unsafe {
        if G == 0 {
            &*TIMG0::PTR
        } else {
            &*(TIMG1::PTR as *const timg0::RegisterBlock)
        }
    }
}

/// The clock controller's register block.
fn clkrst() -> &'static Clkrst {
    // SAFETY: the clock controller is always mapped and lives for the program.
    unsafe { &*esp32p4_pac::HP_SYS_CLKRST::PTR }
}

// ============================================================================
// Interrupt plumbing
// ============================================================================

/// Acknowledge the alarm of timer `unit` and re-arm it in periodic mode.
fn acknowledge(block: &timg0::RegisterBlock, unit: u8) {
    // Level interrupt: clear the request before returning, or the line stays
    // asserted and the trap repeats.
    if unit == 0 {
        block.int_clr_timers.trigger_t0_int_clr();
    } else {
        block.int_clr_timers.trigger_t1_int_clr();
    }

    // Hardware clears `ALARM_EN` on every alarm. With auto-reload the counter
    // restarts from the reload value, so arming it again is all that is needed
    // to get the next period.
    let auto_reload = if unit == 0 {
        block.t0config.autoreload()
    } else {
        block.t1config.autoreload()
    };
    if auto_reload {
        if unit == 0 {
            block.t0config.set_alarm_en();
        } else {
            block.t1config.set_alarm_en();
        }
    }
}

/// Calls the user's handler of timer `index`.
fn dispatch(index: usize) {
    let handler = HANDLERS[index].load(Ordering::Acquire);
    if handler != 0 {
        // SAFETY: `HANDLERS` only ever holds values stored by
        // `enable_interrupt`, which stores `fn()` pointers.
        let handler: fn() = unsafe { core::mem::transmute::<usize, fn()>(handler) };
        handler();
    }
}

fn alarm00() {
    acknowledge(block::<0>(), 0);
    dispatch(0);
}

fn alarm01() {
    acknowledge(block::<0>(), 1);
    dispatch(1);
}

fn alarm10() {
    acknowledge(block::<1>(), 0);
    dispatch(2);
}

fn alarm11() {
    acknowledge(block::<1>(), 1);
    dispatch(3);
}

/// One shim per timer, so the trap handler knows which registers to poke.
const SHIMS: [fn(); TIMER_COUNT] = [alarm00, alarm01, alarm10, alarm11];

/// Interrupt bookkeeping for one timer.
///
/// Holding this is what owns the CPU line: dropping it masks the timer, drops
/// the request, unregisters the shim and hands the line back. Keeping it out of
/// the timer type itself is what lets a stolen copy stay inert.
struct Irq {
    /// Held for its `Drop`, which unregisters the shim and releases the line.
    _handler: InterruptHandler,
    block: &'static timg0::RegisterBlock,
    unit: u8,
    index: usize,
}

impl Drop for Irq {
    fn drop(&mut self) {
        if self.unit == 0 {
            self.block.int_ena_timers.clr_t0_int_ena();
            self.block.int_clr_timers.trigger_t0_int_clr();
        } else {
            self.block.int_ena_timers.clr_t1_int_ena();
            self.block.int_clr_timers.trigger_t1_int_clr();
        }
        HANDLERS[self.index].store(0, Ordering::Release);
    }
}

// ============================================================================
// Clocking
// ============================================================================

/// Selects the clock and opens the timer gate of one counter.
///
/// Two gates are involved: `SOC_CLK_CTRL2` lets the CPU reach the group's
/// registers at all, and `PERI_CLK_CTRL2n` picks the source and gates the
/// counter clock of one timer. ESP-IDF opens both when a timer is claimed.
fn program_clock(group: u8, unit: u8, clock: ClockSource) {
    let clkrst = clkrst();
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
fn clock_source_of(group: u8, unit: u8) -> ClockSource {
    let clkrst = clkrst();
    let selected = if group == 0 {
        let ctrl = &clkrst.peri_clk_ctrl20;
        if unit == 0 {
            ctrl.reg_timergrp0_t0_src_sel()
        } else {
            ctrl.reg_timergrp0_t1_src_sel()
        }
    } else {
        let ctrl = &clkrst.peri_clk_ctrl21;
        if unit == 0 {
            ctrl.reg_timergrp1_t0_src_sel()
        } else {
            ctrl.reg_timergrp1_t1_src_sel()
        }
    };
    ClockSource::from_select(selected)
}

/// Puts one counter back into its reset configuration.
fn reset_timer(block: &timg0::RegisterBlock, unit: u8) {
    // The PAC gives each instance its own register types, so the field access
    // is written once and instantiated per timer.
    macro_rules! reset {
        ($config:ident, $load_hi:ident, $load_lo:ident, $load:ident) => {{
            // The bootloader may have left the timer running with an alarm
            // armed. Stop it before touching the prescaler.
            let config = &block.$config;
            config.clr_en();
            config.write_divider(DEFAULT_DIVIDER);
            config.trigger_divcnt_rst();
            config.clr_alarm_en();
            config.clr_autoreload();
            config.set_increase();
            config.clr_use_xtal();
            block.$load_hi.write_load_hi(0);
            block.$load_lo.write_load_lo(0);
            block.$load.write_load(1);
        }};
    }

    if unit == 0 {
        reset!(t0config, t0loadhi, t0loadlo, t0load);
        block.int_ena_timers.clr_t0_int_ena();
        block.int_clr_timers.trigger_t0_int_clr();
    } else {
        reset!(t1config, t1loadhi, t1loadlo, t1load);
        block.int_ena_timers.clr_t1_int_ena();
        block.int_clr_timers.trigger_t1_int_clr();
    }
}

/// Opens the clocks and resets both counters of one group.
fn init_group(group: u8, clkrst: &hp_sys_clkrst::RegisterBlock) {
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

    let block = if group == 0 {
        block::<0>()
    } else {
        block::<1>()
    };

    for unit in 0..2u8 {
        program_clock(group, unit, ClockSource::Xtal);
        reset_timer(block, unit);
    }

    // The bootloader may leave the watchdog configuration unlocked. Give it a
    // known stage layout while the registers are writable: stage 0 resets the
    // system, the others are off, and the reset pulses are long enough to be
    // seen.
    block.wdtwprotect.write_wdt_wkey(WDT_UNLOCK_KEY);
    block
        .wdtconfig0
        .write_wdt_stg0(MwdtStageAction::ResetSystem.select());
    block
        .wdtconfig0
        .write_wdt_stg1(MwdtStageAction::Off.select());
    block
        .wdtconfig0
        .write_wdt_stg2(MwdtStageAction::Off.select());
    block
        .wdtconfig0
        .write_wdt_stg3(MwdtStageAction::Off.select());
    block.wdtconfig0.clr_wdt_flashboot_mod_en();
    block.wdtconfig0.write_wdt_cpu_reset_length(7);
    block.wdtconfig0.write_wdt_sys_reset_length(7);
    block.wdtconfig0.trigger_wdt_conf_update_en();
    block.wdtwprotect.write_wdt_wkey(0);
}

// ============================================================================
// Timer groups
// ============================================================================

/// `TIMG0`: two general-purpose timers and the main system watchdog.
///
/// The group owns its parts, the way the hardware does: [`timer0`](Self::timer0)
/// and [`timer1`](Self::timer1) are the two counters and [`wdt`](Self::wdt) is
/// the watchdog, each with its own type so the register, matrix source and
/// interrupt plumbing are resolved at compile time.
pub struct TimerGroup0 {
    /// General-purpose timer 0, the source of `TG0_T0`.
    pub timer0: Timer<0, 0>,
    /// General-purpose timer 1, the source of `TG0_T1`.
    pub timer1: Timer<0, 1>,
    /// The group's main system watchdog.
    pub wdt: Watchdog<0>,
    _timg: TIMG0,
}

impl TimerGroup0 {
    /// Take the group and bring it into a known state.
    ///
    /// Opens the group's clock gates, selects the crystal for both counters and
    /// the watchdog, and leaves the counters stopped with their alarms
    /// disarmed. The bootloader's watchdogs are already off by then (see
    /// [`crate::startup`]); this does not re-enable them.
    pub fn new(timg: TIMG0, clkrst: &hp_sys_clkrst::RegisterBlock) -> Self {
        init_group(0, clkrst);
        Self {
            timer0: Timer::new(),
            timer1: Timer::new(),
            wdt: Watchdog::new(),
            _timg: timg,
        }
    }

    /// Give the group's parts back.
    #[inline]
    pub fn release(self) -> TIMG0 {
        self._timg
    }
}

/// `TIMG1`: two general-purpose timers and the main system watchdog.
///
/// See [`TimerGroup0`]; the parts are `Timer<1, 0>`, `Timer<1, 1>` and
/// `Watchdog<1>`.
pub struct TimerGroup1 {
    /// General-purpose timer 0, the source of `TG1_T0`.
    pub timer0: Timer<1, 0>,
    /// General-purpose timer 1, the source of `TG1_T1`.
    pub timer1: Timer<1, 1>,
    /// The group's main system watchdog.
    pub wdt: Watchdog<1>,
    _timg: TIMG1,
}

impl TimerGroup1 {
    /// Take the group and bring it into a known state.
    ///
    /// See [`TimerGroup0::new`].
    pub fn new(timg: TIMG1, clkrst: &hp_sys_clkrst::RegisterBlock) -> Self {
        init_group(1, clkrst);
        Self {
            timer0: Timer::new(),
            timer1: Timer::new(),
            wdt: Watchdog::new(),
            _timg: timg,
        }
    }

    /// Give the group's parts back.
    #[inline]
    pub fn release(self) -> TIMG1 {
        self._timg
    }
}

// ============================================================================
// General-purpose timer
// ============================================================================

/// Compile-time identity of one general-purpose timer.
///
/// `G` is the timer group (0 or 1) and `N` the timer inside it (0 or 1), so
/// `Timer<1, 0>` is `TIMG1`'s first counter.
pub struct Timer<const G: u8, const N: u8> {
    irq: RefCell<Option<Irq>>,
}

impl<const G: u8, const N: u8> Timer<G, N> {
    const fn new() -> Self {
        Self {
            irq: RefCell::new(None),
        }
    }

    /// The clock feeding this timer's prescaler.
    pub fn clock_source(&self) -> ClockSource {
        clock_source_of(G, N)
    }

    /// Switch the timer to another clock source.
    ///
    /// The prescaler divisor stays as it is, so the tick rate changes with the
    /// source. Stop the counter first if it is running.
    pub fn set_clock_source(&self, clock: ClockSource) {
        program_clock(G, N, clock);
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
    ///   [`enable_interrupt`](Timer::enable_interrupt).
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        const { assert!(G < 2 && N < 2, "no such general-purpose timer") };
        Steal(Self::new())
    }
}

/// Implements the register-level API for one concrete timer.
///
/// The PAC names each register after its instance (`t0config`, `t1config`, …),
/// so the names are pasted per invocation instead of being matched at run time.
macro_rules! impl_timer {
    ($G:literal, $N:literal, $source:ident, $shim:ident, $index:literal) => {
        impl Timer<$G, $N> {
            /// The register block of this timer's group.
            #[inline(always)]
            fn block(&self) -> &'static timg0::RegisterBlock {
                block::<$G>()
            }

            /// The interrupt matrix source feeding this timer.
            const SOURCE: Interrupt = Interrupt::$source;
            /// Index of this timer in the HAL's tables.
            const INDEX: usize = $index;

            /// The prescaler divisor, 2 to 65536.
            pub fn divider(&self) -> u32 {
                paste! {
                    match self.block().[<t $N config>].divider() {
                        0 => 65536,
                        1 | 2 => 2,
                        n => n,
                    }
                }
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
                // 65536 does not fit the 16-bit field; zero means 65536.
                let encoded = if divider >= 65536 { 0 } else { divider };
                paste! {
                    let config = &self.block().[<t $N config>];
                    config.write_divider(encoded);
                    config.trigger_divcnt_rst();
                }
            }

            /// The frequency of the counter clock, in Hz.
            ///
            /// Rounded down, so a prescaler that does not divide the source
            /// exactly makes this slightly pessimistic.
            pub fn tick_hz(&self) -> u32 {
                self.clock_source().frequency_hz() / self.divider()
            }

            /// Convert a span into this timer's ticks, rounding down but never
            /// to zero.
            ///
            /// Counter and alarm values are in timer ticks, which is why the
            /// duration has to be converted: at the default 1 MHz one tick is
            /// one microsecond, but a different clock or prescaler changes that.
            pub fn ticks(&self, duration: Duration) -> u64 {
                let ticks =
                    duration.as_ticks() as u128 * self.tick_hz() as u128 / time::TICK_HZ as u128;
                ticks.clamp(1, MAX_VALUE as u128) as u64
            }

            /// Which way the counter runs.
            pub fn direction(&self) -> Direction {
                paste! {
                    if self.block().[<t $N config>].increase() {
                        Direction::Up
                    } else {
                        Direction::Down
                    }
                }
            }

            /// Count up or down. The change takes effect immediately.
            pub fn set_direction(&self, direction: Direction) {
                paste! {
                    let config = &self.block().[<t $N config>];
                    if direction == Direction::Up {
                        config.set_increase();
                    } else {
                        config.clr_increase();
                    }
                }
            }

            /// Reset the counter to its reload value and start counting.
            ///
            /// The counter is stopped while it is reloaded, so a restart is
            /// deterministic: the first tick after `start` is tick one. The
            /// alarm is not armed here — [`set_alarm`](Self::set_alarm) does
            /// that.
            pub fn start(&self) {
                self.stop();
                self.reload();
                paste! { self.block().[<t $N config>].set_en(); }
            }

            /// Stop the counter; it keeps its value and resumes from it.
            pub fn stop(&self) {
                paste! { self.block().[<t $N config>].clr_en(); }
            }

            /// Whether the counter is running.
            pub fn is_running(&self) -> bool {
                paste! { self.block().[<t $N config>].en() }
            }

            /// Latch and return the 54-bit counter value, in timer ticks.
            pub fn counter(&self) -> u64 {
                paste! {
                    let block = self.block();
                    let update = &block.[<t $N update>];
                    update.set_update();
                    while update.update() {
                        core::hint::spin_loop();
                    }
                    let hi = block.[<t $N hi>].hi() as u64;
                    let lo = block.[<t $N lo>].lo() as u64;
                    (hi << 32) | lo
                }
            }

            /// Copy `value` into the counter immediately.
            pub fn set_counter(&self, value: u64) {
                self.set_reload_value(value);
                self.reload();
            }

            /// The value the counter starts from.
            pub fn reload_value(&self) -> u64 {
                paste! {
                    let block = self.block();
                    let hi = block.[<t $N loadhi>].load_hi() as u64;
                    let lo = block.[<t $N loadlo>].load_lo() as u64;
                    (hi << 32) | lo
                }
            }

            /// Set the value the counter starts from: on [`start`](Self::start),
            /// on [`reload`](Self::reload), and every time the alarm fires with
            /// auto-reload on.
            pub fn set_reload_value(&self, value: u64) {
                assert!(value <= MAX_VALUE, "the counter is 54 bits wide");
                paste! {
                    let block = self.block();
                    block.[<t $N loadhi>].write_load_hi((value >> 32) as u32);
                    block.[<t $N loadlo>].write_load_lo(value as u32);
                }
            }

            /// Copy the reload value into the counter now.
            pub fn reload(&self) {
                paste! { self.block().[<t $N load>].write_load(1); }
            }

            /// Reload the counter from its reload value when the alarm fires.
            ///
            /// This also decides what the interrupt handler does: with
            /// auto-reload on it re-arms the alarm after acknowledging it, which
            /// is what makes a periodic alarm repeat. If auto-reload is turned
            /// off while an alarm is armed, the next alarm is the last one.
            pub fn set_auto_reload(&self, enable: bool) {
                paste! {
                    let config = &self.block().[<t $N config>];
                    if enable {
                        config.set_autoreload();
                    } else {
                        config.clr_autoreload();
                    }
                }
            }

            /// Whether the counter reloads itself when the alarm fires.
            pub fn auto_reload(&self) -> bool {
                paste! { self.block().[<t $N config>].autoreload() }
            }

            /// Arm the alarm for the absolute counter value `ticks`.
            ///
            /// Combine with [`set_reload_value`](Self::set_reload_value) and
            /// [`set_auto_reload`](Self::set_auto_reload) for a periodic alarm,
            /// or with [`set_alarm_in`](Self::set_alarm_in) for one measured
            /// from now.
            pub fn set_alarm(&self, ticks: u64) {
                assert!(ticks <= MAX_VALUE, "the alarm is 54 bits wide");
                paste! {
                    let block = self.block();
                    block.[<t $N alarmhi>].write_alarm_hi((ticks >> 32) as u32);
                    block.[<t $N alarmlo>].write_alarm_lo(ticks as u32);
                }
                // A request left over from an earlier alarm would fire
                // immediately.
                self.clear_alarm();
                paste! { self.block().[<t $N config>].set_alarm_en(); }
            }

            /// The armed alarm value, in timer ticks.
            pub fn alarm(&self) -> u64 {
                paste! {
                    let block = self.block();
                    let hi = block.[<t $N alarmhi>].alarm_hi() as u64;
                    let lo = block.[<t $N alarmlo>].alarm_lo() as u64;
                    (hi << 32) | lo
                }
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

            /// Disarm the alarm. A request that is already pending stays pending
            /// until [`clear_alarm`](Self::clear_alarm).
            pub fn disarm(&self) {
                paste! { self.block().[<t $N config>].clr_alarm_en(); }
            }

            /// Whether the alarm is armed.
            pub fn alarm_armed(&self) -> bool {
                paste! { self.block().[<t $N config>].alarm_en() }
            }

            /// Whether the alarm fired since the last clear.
            pub fn alarm_pending(&self) -> bool {
                paste! { self.block().int_raw_timers.[<t $N _int_raw>]() }
            }

            /// Drop a pending alarm request.
            pub fn clear_alarm(&self) {
                paste! { self.block().int_clr_timers.[<trigger_t $N _int_clr>]() }
            }

            /// Route this timer's alarm to `line` and call `handler` in
            /// interrupt context.
            ///
            /// Claims the line the way [`interrupt::claim`] does, so the timer
            /// panics if the line is already taken. The alarm still has to be
            /// armed with [`set_alarm`](Self::set_alarm) or
            /// [`set_alarm_in`](Self::set_alarm_in), and global interrupts have
            /// to be enabled with [`interrupt::enable`].
            pub fn enable_interrupt(&self, line: CpuInterrupt, handler: fn()) {
                assert!(
                    self.irq.borrow().is_none(),
                    "this timer already has an interrupt handler"
                );

                HANDLERS[Self::INDEX].store(handler as usize, Ordering::Release);

                // SAFETY: the shim only touches this timer's status bits before
                // calling the user's function, and `claim` makes sure no other
                // driver uses the line.
                let claimed =
                    unsafe { interrupt::claim(Self::SOURCE, line, Level::LOW, SHIMS[Self::INDEX]) };
                claimed.enable();
                self.clear_alarm();
                paste! { self.block().int_ena_timers.[<set_t $N _int_ena>](); }
                *self.irq.borrow_mut() = Some(Irq {
                    _handler: claimed,
                    block: self.block(),
                    unit: $N,
                    index: Self::INDEX,
                });
            }

            /// Mask the alarm interrupt and give its CPU line back.
            pub fn disable_interrupt(&self) {
                // Dropping the bookkeeping masks the timer, drops the request,
                // forgets the shim and releases the line.
                self.irq.borrow_mut().take();
            }

            /// Whether the alarm interrupt is unmasked.
            pub fn interrupt_enabled(&self) -> bool {
                paste! { self.block().int_ena_timers.[<t $N _int_ena>]() }
            }
        }
    };
}

impl_timer!(0, 0, TG0_T0, alarm00, 0);
impl_timer!(0, 1, TG0_T1, alarm01, 1);
impl_timer!(1, 0, TG1_T0, alarm10, 2);
impl_timer!(1, 1, TG1_T1, alarm11, 3);

// ============================================================================
// Main system watchdog
// ============================================================================

/// The main system watchdog of one timer group.
///
/// The MWDT counts at `XTAL_CLK` (40 MHz) divided by a prescaler, through up to
/// four stages. [`enable`](Self::enable) starts counting; from then on the
/// program has to [`feed`](Self::feed) it before stage 0 runs out.
///
/// [`crate::startup`] disables the watchdogs the bootloader leaves running, so a
/// program that never touches this type cannot be reset by them.
pub struct Watchdog<const G: u8> {
    _private: (),
}

impl<const G: u8> Watchdog<G> {
    const fn new() -> Self {
        Self { _private: () }
    }

    /// The register block of this watchdog's group.
    #[inline(always)]
    fn block(&self) -> &'static timg0::RegisterBlock {
        block::<G>()
    }

    /// Create a copy of this watchdog without owning it.
    ///
    /// Useful for feeding the watchdog from a place that does not own the
    /// group, such as a task tick. Nothing is written to the hardware.
    ///
    /// # Safety
    ///
    /// The caller must ensure that a [`TimerGroup0`]/[`TimerGroup1`] has already
    /// been created for this group, and that the copy is not used to
    /// reconfigure the watchdog behind the owner's back.
    #[inline]
    pub unsafe fn steal() -> Steal<Self> {
        const { assert!(G < 2, "no such timer group") };
        Steal(Self::new())
    }

    /// Whether the watchdog is counting.
    pub fn is_enabled(&self) -> bool {
        self.block().wdtconfig0.wdt_en()
    }

    /// Unlock the configuration registers, run `f`, and lock them again.
    ///
    /// The unlock key is the register's own reset value: writing anything else
    /// — zero, in practice — turns write protection back on.
    fn unlocked<R>(&self, f: impl FnOnce(&timg0::RegisterBlock) -> R) -> R {
        let block = self.block();
        block.wdtwprotect.write_wdt_wkey(WDT_UNLOCK_KEY);
        let result = f(block);
        block.wdtwprotect.write_wdt_wkey(0);
        result
    }

    /// Start the watchdog.
    ///
    /// The stage layout is the one [`TimerGroup0::new`] left behind: stage 0
    /// resets the system, stages 1 to 3 are off. Use
    /// [`set_stage_action`](Self::set_stage_action) to change that and
    /// [`set_timeout`](Self::set_timeout) to set how long each stage runs; both
    /// work before or after this call. Enabling also turns off the flash-boot
    /// protection, which would otherwise keep counting while disabled.
    pub fn enable(&self) {
        self.unlocked(|block| {
            block.wdtconfig0.clr_wdt_flashboot_mod_en();
            block.wdtconfig0.set_wdt_en();
            block.wdtconfig0.trigger_wdt_conf_update_en();
        });
    }

    /// Stop the watchdog.
    pub fn disable(&self) {
        self.unlocked(|block| {
            block.wdtconfig0.clr_wdt_en();
            block.wdtconfig0.clr_wdt_flashboot_mod_en();
            block.wdtconfig0.trigger_wdt_conf_update_en();
        });
    }

    /// Restart the countdown from the beginning of stage 0.
    pub fn feed(&self) {
        self.unlocked(|block| block.wdtfeed.write_wdt_feed(1));
    }

    /// Set the timeout of one stage.
    ///
    /// The watchdog counts at 40 MHz; the prescaler is widened automatically so
    /// timeouts longer than the 32-bit tick register allows still fit.
    pub fn set_timeout(&self, stage: MwdtStage, timeout: Duration) {
        let ticks = timeout.as_ticks() as u128 * WDT_CLOCK_HZ as u128 / time::TICK_HZ as u128;
        assert!(ticks > 0, "a watchdog timeout must not be zero");

        let prescaler = ticks.div_ceil(u32::MAX as u128).clamp(1, u16::MAX as u128) as u32;
        let hold = ticks.div_ceil(prescaler as u128);
        assert!(
            hold <= u32::MAX as u128,
            "watchdog timeout too long for the 16-bit prescaler"
        );
        let hold = hold as u32;

        self.unlocked(|block| {
            block.wdtconfig1.write_wdt_clk_prescale(prescaler);
            block.wdtconfig1.trigger_wdt_divcnt_rst();
            match stage.number() {
                0 => block.wdtconfig2.write_wdt_stg0_hold(hold),
                1 => block.wdtconfig3.write_wdt_stg1_hold(hold),
                2 => block.wdtconfig4.write_wdt_stg2_hold(hold),
                _ => block.wdtconfig5.write_wdt_stg3_hold(hold),
            }
            block.wdtconfig0.trigger_wdt_conf_update_en();
        });
    }

    /// Choose what one stage does when it times out.
    pub fn set_stage_action(&self, stage: MwdtStage, action: MwdtStageAction) {
        self.unlocked(|block| {
            let action = action.select();
            match stage.number() {
                0 => block.wdtconfig0.write_wdt_stg0(action),
                1 => block.wdtconfig0.write_wdt_stg1(action),
                2 => block.wdtconfig0.write_wdt_stg2(action),
                _ => block.wdtconfig0.write_wdt_stg3(action),
            }
            block.wdtconfig0.trigger_wdt_conf_update_en();
        });
    }
}
