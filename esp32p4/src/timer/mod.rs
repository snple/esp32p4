//! General-purpose timers and the main system watchdog (`TIMG0` / `TIMG1`).
//!
//! The chip has two timer groups. Each group contains two independent 54-bit
//! general-purpose counters and one main system watchdog (MWDT):
//!
//! ```text
//! TimerGroup0 ─┬─ Timer00     (matrix source TG0_T0)
//!              ├─ Timer01     (TG0_T1)
//!              └─ Watchdog0   (TG0_WDT)
//! TimerGroup1 ─┬─ Timer10     (TG1_T0)
//!              ├─ Timer11     (TG1_T1)
//!              └─ Watchdog1   (TG1_WDT)
//! ```
//!
//! A counter runs off `XTAL_CLK` (40 MHz), `RC_FAST_CLK` (~17.5 MHz) or
//! `PLL_F80M_CLK` (80 MHz) through a 16-bit prescaler. The default divides the
//! 40 MHz crystal by 40, so one tick is one microsecond.
//!
//! # Using a timer
//!
//! [`TimerGroup0::init`] takes `TIMG0`, brings the group into a known state and
//! hands out its parts the way the hardware has them: `group.timer0`,
//! `group.timer1` and `group.wdt`. Nothing starts on its own: configure the
//! counter, then call [`Timer00::start`].
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
//! let group = TimerGroup0::init(dp.TIMG0, &dp.HP_SYS_CLKRST);
//!
//! // The group owns the CPU line: it claims it, routes the counter's alarm to
//! // it, and gives it back when the interrupt or the whole group is dropped.
//! group.enable_timer0_interrupt(CpuInterrupt::new(21), tick);  // CPU line 21
//!
//! let timer = &group.timer0;
//! timer.set_reload_value(0);                            // restart from here
//! timer.set_alarm(timer.ticks(Duration::from_millis(100)));
//! timer.set_auto_reload(true);                          // and again, every 100 ms
//! timer.start();
//! interrupt::enable();
//! ```
//!
//! Every method of a counter takes `&self`, and `unsafe { Timer00::steal() }`
//! hands the same register access to an interrupt handler or another module. A
//! stolen counter never touches the hardware: it neither starts the counter nor
//! owns a CPU line — the lines belong to the group.
//!
//! # The watchdog
//!
//! [`Watchdog0`] / [`Watchdog1`] are the MWDTs: four-stage countdowns that can
//! raise an interrupt or reset the CPU or the whole system. They are configured
//! through the group, and need feeding before the first stage runs out.
//!
//! # Registers
//!
//! Everything goes through `esp32p4-pac`. The vendor SVD stops at timer 0 of
//! each group and leaves timer 1 in a reserved hole, so `timg.svd` completes the
//! two blocks (see `xtask/gen_timg.py`) and `cargo xtask gen-pac` merges them
//! over the vendor's peripherals. Each part works through a handle to its own
//! group, so group 1 goes through `TIMG1`'s registers instead of a cast to group
//! 0's.

use core::cell::RefCell;
use core::sync::atomic::{AtomicUsize, Ordering};

use esp32p4_pac::{TIMG0, TIMG1, hp_sys_clkrst};
use pastey::paste;

use crate::Steal;
use crate::interrupt::{self, CpuInterrupt, Interrupt, InterruptHandler, Level};
use crate::time::{self, Duration};

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
/// [`TimerGroup0::init`].
const WDT_CLOCK_HZ: u32 = 40_000_000;

/// Value that unlocks the watchdog's configuration registers.
const WDT_UNLOCK_KEY: u32 = 0x50d8_3aa1;

/// Number of general-purpose timers on the chip.
const TIMER_COUNT: usize = 4;

/// User callbacks, one per counter, called by the generated interrupt shims.
static HANDLERS: [AtomicUsize; TIMER_COUNT] = [const { AtomicUsize::new(0) }; TIMER_COUNT];

/// The clock controller's register block.
fn clkrst() -> &'static hp_sys_clkrst::RegisterBlock {
    // SAFETY: the clock controller is always mapped and lives for the program.
    unsafe { &*esp32p4_pac::HP_SYS_CLKRST::PTR }
}

/// Calls the user's handler of counter `index`.
fn dispatch(index: usize) {
    let handler = HANDLERS[index].load(Ordering::Acquire);
    if handler != 0 {
        // SAFETY: `HANDLERS` only ever holds values stored by the timer groups,
        // which store `fn()` pointers.
        let handler: fn() = unsafe { core::mem::transmute::<usize, fn()>(handler) };
        handler();
    }
}

// ============================================================================
// General-purpose timer
// ============================================================================

/// Generates one general-purpose counter and its register-level API.
///
/// The four counters are separate types — `Timer00`, `Timer01`, `Timer10` and
/// `Timer11` — each holding a handle to its own group, so the registers, the
/// matrix source and the interrupt shim are all fixed at compile time. The PAC
/// names each register after its instance (`t0config`, `t1config`, …), so the
/// register names are pasted per invocation.
macro_rules! impl_timer {
    (
        $(#[$attr:meta])*
        $Timer:ident, $TIMG:ident, $G:literal, $N:literal, $source:ident, $shim:ident,
        $index:literal
    ) => {
        $(#[$attr])*
        pub struct $Timer {
            /// This counter's group. The group owns the same peripheral; the
            /// handle is what makes the registers reachable from a counter.
            timg: $TIMG,
        }

        impl $Timer {
            const fn new() -> Self {
                // SAFETY: the group that created this counter owns the
                // peripheral, so the HAL already has exclusive access to its
                // registers. This is a second handle to them, and the counters
                // only ever touch their own registers.
                Self {
                    timg: unsafe { $TIMG::steal() },
                }
            }

            /// Create a copy of this counter without owning it.
            ///
            /// This is analogous to [`Pin::steal`](crate::gpio::Pin::steal): it
            /// lets an interrupt handler or another module reach the counter
            /// registers while the real owner lives elsewhere (for example in
            /// `main`). Nothing is written to the hardware, so a stolen copy
            /// never disturbs a running counter, and dropping it neither stops
            /// the counter nor touches an interrupt — the CPU lines belong to
            /// the group.
            ///
            /// # Safety
            ///
            /// The caller must ensure that a [`TimerGroup0`]/[`TimerGroup1`] has
            /// already been created, so the clock is on and the prescaler is
            /// programmed, and that the copy is not used to reconfigure the
            /// counter behind the owner's back.
            #[inline]
            pub unsafe fn steal() -> Steal<Self> {
                Steal(Self::new())
            }

            /// The clock feeding this counter's prescaler.
            pub fn clock_source(&self) -> ClockSource {
                paste! {
                    let ctrl = &clkrst().[<peri_clk_ctrl2 $G>];
                    ClockSource::from_select(ctrl.[<reg_timergrp $G _t $N _src_sel>]())
                }
            }

            /// Switch the counter to another clock source.
            ///
            /// The prescaler divisor stays as it is, so the tick rate changes
            /// with the source. Stop the counter first if it is running.
            pub fn set_clock_source(&self, clock: ClockSource) {
                paste! {
                    // `SOC_CLK_CTRL2` lets the CPU reach the group's registers
                    // at all, `PERI_CLK_CTRL2n` selects the source and gates
                    // this counter's clock.
                    let clkrst = clkrst();
                    clkrst.soc_clk_ctrl2.[<set_reg_timergrp $G _apb_clk_en>]();
                    let ctrl = &clkrst.[<peri_clk_ctrl2 $G>];
                    ctrl.[<write_reg_timergrp $G _t $N _src_sel>](clock.select());
                    ctrl.[<set_reg_timergrp $G _t $N _clk_en>]();
                }
            }

            /// Put this counter back into its reset configuration.
            fn reset(&self) {
                paste! {
                    // The bootloader may have left the counter running with an
                    // alarm armed. Stop it before touching the prescaler.
                    let config = &self.timg.[<t $N config>];
                    config.clr_en();
                    config.write_divider(DEFAULT_DIVIDER);
                    config.trigger_divcnt_rst();
                    config.clr_alarm_en();
                    config.clr_autoreload();
                    config.set_increase();
                    config.clr_use_xtal();
                    self.timg.[<t $N loadhi>].write_load_hi(0);
                    self.timg.[<t $N loadlo>].write_load_lo(0);
                    self.timg.[<t $N load>].write_load(1);
                    self.timg.int_ena_timers.[<clr_t $N _int_ena>]();
                    self.timg.int_clr_timers.[<trigger_t $N _int_clr>]();
                }
            }

            /// The prescaler divisor, 2 to 65536.
            pub fn divider(&self) -> u32 {
                paste! {
                    match self.timg.[<t $N config>].divider() {
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
                    let config = &self.timg.[<t $N config>];
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

            /// Convert a span into this counter's ticks, rounding down but never
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
                    if self.timg.[<t $N config>].increase() {
                        Direction::Up
                    } else {
                        Direction::Down
                    }
                }
            }

            /// Count up or down. The change takes effect immediately.
            pub fn set_direction(&self, direction: Direction) {
                paste! {
                    let config = &self.timg.[<t $N config>];
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
                paste! { self.timg.[<t $N config>].set_en(); }
            }

            /// Stop the counter; it keeps its value and resumes from it.
            pub fn stop(&self) {
                paste! { self.timg.[<t $N config>].clr_en(); }
            }

            /// Whether the counter is running.
            pub fn is_running(&self) -> bool {
                paste! { self.timg.[<t $N config>].en() }
            }

            /// Latch and return the 54-bit counter value, in timer ticks.
            pub fn counter(&self) -> u64 {
                paste! {
                    let update = &self.timg.[<t $N update>];
                    update.set_update();
                    while update.update() {
                        core::hint::spin_loop();
                    }
                    let hi = self.timg.[<t $N hi>].hi() as u64;
                    let lo = self.timg.[<t $N lo>].lo() as u64;
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
                    let hi = self.timg.[<t $N loadhi>].load_hi() as u64;
                    let lo = self.timg.[<t $N loadlo>].load_lo() as u64;
                    (hi << 32) | lo
                }
            }

            /// Set the value the counter starts from: on [`start`](Self::start),
            /// on [`reload`](Self::reload), and every time the alarm fires with
            /// auto-reload on.
            pub fn set_reload_value(&self, value: u64) {
                assert!(value <= MAX_VALUE, "the counter is 54 bits wide");
                paste! {
                    self.timg.[<t $N loadhi>].write_load_hi((value >> 32) as u32);
                    self.timg.[<t $N loadlo>].write_load_lo(value as u32);
                }
            }

            /// Copy the reload value into the counter now.
            pub fn reload(&self) {
                paste! { self.timg.[<t $N load>].write_load(1); }
            }

            /// Reload the counter from its reload value when the alarm fires.
            ///
            /// This also decides what the interrupt handler does: with
            /// auto-reload on it re-arms the alarm after acknowledging it, which
            /// is what makes a periodic alarm repeat. If auto-reload is turned
            /// off while an alarm is armed, the next alarm is the last one.
            pub fn set_auto_reload(&self, enable: bool) {
                paste! {
                    let config = &self.timg.[<t $N config>];
                    if enable {
                        config.set_autoreload();
                    } else {
                        config.clr_autoreload();
                    }
                }
            }

            /// Whether the counter reloads itself when the alarm fires.
            pub fn auto_reload(&self) -> bool {
                paste! { self.timg.[<t $N config>].autoreload() }
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
                    self.timg.[<t $N alarmhi>].write_alarm_hi((ticks >> 32) as u32);
                    self.timg.[<t $N alarmlo>].write_alarm_lo(ticks as u32);
                }
                // A request left over from an earlier alarm would fire
                // immediately.
                self.clear_alarm();
                paste! { self.timg.[<t $N config>].set_alarm_en(); }
            }

            /// The armed alarm value, in timer ticks.
            pub fn alarm(&self) -> u64 {
                paste! {
                    let hi = self.timg.[<t $N alarmhi>].alarm_hi() as u64;
                    let lo = self.timg.[<t $N alarmlo>].alarm_lo() as u64;
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
                paste! { self.timg.[<t $N config>].clr_alarm_en(); }
            }

            /// Whether the alarm is armed.
            pub fn alarm_armed(&self) -> bool {
                paste! { self.timg.[<t $N config>].alarm_en() }
            }

            /// Whether the alarm fired since the last clear.
            pub fn alarm_pending(&self) -> bool {
                paste! { self.timg.int_raw_timers.[<t $N _int_raw>]() }
            }

            /// Drop a pending alarm request.
            pub fn clear_alarm(&self) {
                paste! { self.timg.int_clr_timers.[<trigger_t $N _int_clr>]() }
            }

            /// Whether the alarm interrupt is unmasked.
            pub fn alarm_interrupt_enabled(&self) -> bool {
                paste! { self.timg.int_ena_timers.[<t $N _int_ena>]() }
            }

            /// Unmask the alarm interrupt.
            fn enable_alarm_interrupt(&self) {
                paste! { self.timg.int_ena_timers.[<set_t $N _int_ena>](); }
            }

            /// Mask the alarm interrupt and drop a pending request.
            fn disable_alarm_interrupt(&self) {
                paste! {
                    self.timg.int_ena_timers.[<clr_t $N _int_ena>]();
                    self.timg.int_clr_timers.[<trigger_t $N _int_clr>]();
                }
            }

            /// Acknowledge the alarm and re-arm it in periodic mode.
            ///
            /// The request is level-triggered, so it has to be dropped before
            /// returning or the line stays asserted and the trap repeats. The
            /// hardware clears `ALARM_EN` on every alarm; with auto-reload the
            /// counter restarts from the reload value, so arming it again is all
            /// that is needed to get the next period.
            fn acknowledge_alarm(&self) {
                paste! {
                    self.timg.int_clr_timers.[<trigger_t $N _int_clr>]();
                    let config = &self.timg.[<t $N config>];
                    if config.autoreload() {
                        config.set_alarm_en();
                    }
                }
            }

            /// The interrupt matrix source feeding this counter.
            const SOURCE: Interrupt = Interrupt::$source;
            /// The shim the group installs on the counter's CPU line.
            const SHIM: fn() = $shim;
            /// Slot of this counter in the HAL's tables.
            const INDEX: usize = $index;
        }

        /// Interrupt shim of this counter: acknowledges its alarm, then calls the
        /// user's handler.
        fn $shim() {
            $Timer::new().acknowledge_alarm();
            dispatch($index);
        }
    };
}

impl_timer! {
    /// `TIMG0` counter 0, the source of `TG0_T0`.
    Timer00, TIMG0, 0, 0, TG0_T0, alarm00, 0
}

impl_timer! {
    /// `TIMG0` counter 1, the source of `TG0_T1`.
    Timer01, TIMG0, 0, 1, TG0_T1, alarm01, 1
}

impl_timer! {
    /// `TIMG1` counter 0, the source of `TG1_T0`.
    Timer10, TIMG1, 1, 0, TG1_T0, alarm10, 2
}

impl_timer! {
    /// `TIMG1` counter 1, the source of `TG1_T1`.
    Timer11, TIMG1, 1, 1, TG1_T1, alarm11, 3
}

// ============================================================================
// Main system watchdog
// ============================================================================

/// Generates one main system watchdog and its register-level API.
///
/// `Watchdog0` and `Watchdog1` are separate types, each holding a handle to its
/// own group, exactly like the counters.
macro_rules! impl_watchdog {
    ($(#[$attr:meta])* $Watchdog:ident, $TIMG:ident) => {
        $(#[$attr])*
        pub struct $Watchdog {
            /// This watchdog's group.
            timg: $TIMG,
        }

        impl $Watchdog {
            const fn new() -> Self {
                // SAFETY: the group that created this watchdog owns the
                // peripheral; this is a second handle to its registers, and the
                // watchdog only touches the MWDT registers.
                Self {
                    timg: unsafe { $TIMG::steal() },
                }
            }

            /// Create a copy of this watchdog without owning it.
            ///
            /// Useful for feeding the watchdog from a place that does not own
            /// the group, such as a task tick. Nothing is written to the
            /// hardware.
            ///
            /// # Safety
            ///
            /// The caller must ensure that a [`TimerGroup0`]/[`TimerGroup1`] has
            /// already been created for this group, and that the copy is not
            /// used to reconfigure the watchdog behind the owner's back.
            #[inline]
            pub unsafe fn steal() -> Steal<Self> {
                Steal(Self::new())
            }

            /// Leave a known stage layout behind, while the registers are
            /// writable.
            ///
            /// The bootloader may leave the watchdog configuration unlocked.
            /// Stage 0 resets the system, the others are off, and the reset
            /// pulses are long enough to be seen.
            fn init(&self) {
                self.unlocked(|timg| {
                    timg.wdtconfig0
                        .write_wdt_stg0(MwdtStageAction::ResetSystem.select());
                    timg.wdtconfig0
                        .write_wdt_stg1(MwdtStageAction::Off.select());
                    timg.wdtconfig0
                        .write_wdt_stg2(MwdtStageAction::Off.select());
                    timg.wdtconfig0
                        .write_wdt_stg3(MwdtStageAction::Off.select());
                    timg.wdtconfig0.clr_wdt_flashboot_mod_en();
                    timg.wdtconfig0.write_wdt_cpu_reset_length(7);
                    timg.wdtconfig0.write_wdt_sys_reset_length(7);
                    timg.wdtconfig0.trigger_wdt_conf_update_en();
                });
            }

            /// Whether the watchdog is counting.
            pub fn is_enabled(&self) -> bool {
                self.timg.wdtconfig0.wdt_en()
            }

            /// Unlock the configuration registers, run `f`, and lock them again.
            ///
            /// The unlock key is the register's own reset value: writing anything
            /// else — zero, in practice — turns write protection back on.
            fn unlocked<R>(&self, f: impl FnOnce(&$TIMG) -> R) -> R {
                self.timg.wdtwprotect.write_wdt_wkey(WDT_UNLOCK_KEY);
                let result = f(&self.timg);
                self.timg.wdtwprotect.write_wdt_wkey(0);
                result
            }

            /// Start the watchdog.
            ///
            /// The stage layout is the one the group's `init` left behind: stage 0
            /// resets the system, stages 1 to 3 are off. Use
            /// [`set_stage_action`](Self::set_stage_action) to change that and
            /// [`set_timeout`](Self::set_timeout) to set how long each stage
            /// runs; both work before or after this call. Enabling also turns off
            /// the flash-boot protection, which would otherwise keep counting
            /// while disabled.
            pub fn enable(&self) {
                self.unlocked(|timg| {
                    timg.wdtconfig0.clr_wdt_flashboot_mod_en();
                    timg.wdtconfig0.set_wdt_en();
                    timg.wdtconfig0.trigger_wdt_conf_update_en();
                });
            }

            /// Stop the watchdog.
            pub fn disable(&self) {
                self.unlocked(|timg| {
                    timg.wdtconfig0.clr_wdt_en();
                    timg.wdtconfig0.clr_wdt_flashboot_mod_en();
                    timg.wdtconfig0.trigger_wdt_conf_update_en();
                });
            }

            /// Restart the countdown from the beginning of stage 0.
            pub fn feed(&self) {
                self.unlocked(|timg| timg.wdtfeed.write_wdt_feed(1));
            }

            /// Set the timeout of one stage.
            ///
            /// The watchdog counts at 40 MHz; the prescaler is widened
            /// automatically so timeouts longer than the 32-bit tick register
            /// allows still fit.
            pub fn set_timeout(&self, stage: MwdtStage, timeout: Duration) {
                let ticks =
                    timeout.as_ticks() as u128 * WDT_CLOCK_HZ as u128 / time::TICK_HZ as u128;
                assert!(ticks > 0, "a watchdog timeout must not be zero");

                let prescaler = ticks.div_ceil(u32::MAX as u128).clamp(1, u16::MAX as u128) as u32;
                let hold = ticks.div_ceil(prescaler as u128);
                assert!(
                    hold <= u32::MAX as u128,
                    "watchdog timeout too long for the 16-bit prescaler"
                );
                let hold = hold as u32;

                self.unlocked(|timg| {
                    timg.wdtconfig1.write_wdt_clk_prescale(prescaler);
                    timg.wdtconfig1.trigger_wdt_divcnt_rst();
                    match stage.number() {
                        0 => timg.wdtconfig2.write_wdt_stg0_hold(hold),
                        1 => timg.wdtconfig3.write_wdt_stg1_hold(hold),
                        2 => timg.wdtconfig4.write_wdt_stg2_hold(hold),
                        _ => timg.wdtconfig5.write_wdt_stg3_hold(hold),
                    }
                    timg.wdtconfig0.trigger_wdt_conf_update_en();
                });
            }

            /// Choose what one stage does when it times out.
            pub fn set_stage_action(&self, stage: MwdtStage, action: MwdtStageAction) {
                self.unlocked(|timg| {
                    let action = action.select();
                    match stage.number() {
                        0 => timg.wdtconfig0.write_wdt_stg0(action),
                        1 => timg.wdtconfig0.write_wdt_stg1(action),
                        2 => timg.wdtconfig0.write_wdt_stg2(action),
                        _ => timg.wdtconfig0.write_wdt_stg3(action),
                    }
                    timg.wdtconfig0.trigger_wdt_conf_update_en();
                });
            }
        }
    };
}

impl_watchdog! {
    /// `TIMG0`'s main system watchdog.
    ///
    /// The MWDT counts at `XTAL_CLK` (40 MHz) divided by a prescaler, through up
    /// to four stages. [`enable`](Self::enable) starts counting; from then on the
    /// program has to [`feed`](Self::feed) it before stage 0 runs out.
    ///
    /// [`crate::startup`] disables the watchdogs the bootloader leaves running,
    /// so a program that never touches this type cannot be reset by them.
    Watchdog0, TIMG0
}

impl_watchdog! {
    /// `TIMG1`'s main system watchdog.
    ///
    /// See [`Watchdog0`] for the stage model and the boot-time state.
    Watchdog1, TIMG1
}

// ============================================================================
// Timer groups
// ============================================================================

/// Generates one timer group: the struct that owns the peripheral and the
/// counters, and the code that brings them into a known state.
///
/// The group is what owns the CPU lines: `enable_timer0_interrupt` and friends
/// claim a line for a counter and hold it until the group drops. The counters
/// themselves are pure register access.
macro_rules! impl_timer_group {
    (
        $(#[$attr:meta])*
        $TimerGroup:ident, $TIMG:ident, $G:literal, $Timer0:ident, $Timer1:ident, $Watchdog:ident
    ) => {
        $(#[$attr])*
        pub struct $TimerGroup {
            /// The group's peripheral, `None` once `release` took it back.
            timg: Option<$TIMG>,
            /// General-purpose timer 0.
            pub timer0: $Timer0,
            /// General-purpose timer 1.
            pub timer1: $Timer1,
            /// The group's main system watchdog.
            pub wdt: $Watchdog,
            /// The CPU line the first counter has claimed, if any.
            irq0: RefCell<Option<InterruptHandler>>,
            /// The CPU line the second counter has claimed, if any.
            irq1: RefCell<Option<InterruptHandler>>,
        }

        impl $TimerGroup {
            /// Take the group and bring it into a known state.
            ///
            /// Opens the group's clock gates, selects the crystal for both
            /// counters and the watchdog, and leaves the counters stopped with
            /// their alarms disarmed. The bootloader's watchdogs are already off
            /// by then (see [`crate::startup`]); this does not re-enable them.
            pub fn init(timg: $TIMG, clkrst: &hp_sys_clkrst::RegisterBlock) -> Self {
                let group = Self {
                    timg: Some(timg),
                    timer0: $Timer0::new(),
                    timer1: $Timer1::new(),
                    wdt: $Watchdog::new(),
                    irq0: RefCell::new(None),
                    irq1: RefCell::new(None),
                };

                // The APB gate lets the CPU reach the group at all; the watchdog
                // has a gate and a source selector of its own.
                paste! {
                    clkrst.soc_clk_ctrl2.[<set_reg_timergrp $G _apb_clk_en>]();
                    let ctrl = &clkrst.[<peri_clk_ctrl2 $G>];
                    ctrl.[<set_reg_timergrp $G _wdt_clk_en>]();
                    ctrl.[<write_reg_timergrp $G _wdt_src_sel>](ClockSource::Xtal.select());
                }

                group.timer0.set_clock_source(ClockSource::Xtal);
                group.timer0.reset();
                group.timer1.set_clock_source(ClockSource::Xtal);
                group.timer1.reset();
                group.wdt.init();

                group
            }

            /// Give the peripheral back.
            ///
            /// The CPU lines are released first, exactly as on drop. The parts
            /// stay with the group; only the peripheral leaves.
            pub fn release(mut self) -> $TIMG {
                self.disable_timer0_interrupt();
                self.disable_timer1_interrupt();
                self.timg.take().expect("the group releases once")
            }

            /// Route `timer0`'s alarm to `line` and call `handler` in interrupt
            /// context.
            ///
            /// Claims the line the way [`interrupt::claim`] does, so this panics
            /// if the line is already taken. The alarm still has to be armed with
            /// `timer0.set_alarm`, and global interrupts have to be enabled with
            /// [`interrupt::enable`].
            pub fn enable_timer0_interrupt(&self, line: CpuInterrupt, handler: fn()) {
                self.claim(
                    &self.irq0,
                    $Timer0::INDEX,
                    $Timer0::SOURCE,
                    $Timer0::SHIM,
                    line,
                    handler,
                );
                self.timer0.clear_alarm();
                self.timer0.enable_alarm_interrupt();
            }

            /// Mask `timer0`'s alarm interrupt and give its CPU line back.
            pub fn disable_timer0_interrupt(&self) {
                self.timer0.disable_alarm_interrupt();
                self.release_line(&self.irq0, $Timer0::INDEX);
            }

            /// Whether `timer0`'s alarm interrupt is unmasked.
            pub fn timer0_interrupt_enabled(&self) -> bool {
                self.timer0.alarm_interrupt_enabled()
            }

            /// Route `timer1`'s alarm to `line` and call `handler` in interrupt
            /// context.
            ///
            /// See [`enable_timer0_interrupt`](Self::enable_timer0_interrupt).
            pub fn enable_timer1_interrupt(&self, line: CpuInterrupt, handler: fn()) {
                self.claim(
                    &self.irq1,
                    $Timer1::INDEX,
                    $Timer1::SOURCE,
                    $Timer1::SHIM,
                    line,
                    handler,
                );
                self.timer1.clear_alarm();
                self.timer1.enable_alarm_interrupt();
            }

            /// Mask `timer1`'s alarm interrupt and give its CPU line back.
            pub fn disable_timer1_interrupt(&self) {
                self.timer1.disable_alarm_interrupt();
                self.release_line(&self.irq1, $Timer1::INDEX);
            }

            /// Whether `timer1`'s alarm interrupt is unmasked.
            pub fn timer1_interrupt_enabled(&self) -> bool {
                self.timer1.alarm_interrupt_enabled()
            }

            /// Take `line` for the counter that owns `slot`.
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
                    "this timer already has an interrupt handler"
                );

                HANDLERS[index].store(handler as usize, Ordering::Release);

                // SAFETY: the shim only touches its counter's status bits before
                // calling the user's function, and `claim` makes sure no other
                // driver uses the line.
                let claimed = unsafe { interrupt::claim(source, line, Level::LOW, shim) };
                claimed.enable();
                *slot.borrow_mut() = Some(claimed);
            }

            /// Give the line in `slot` back.
            fn release_line(&self, slot: &RefCell<Option<InterruptHandler>>, index: usize) {
                if let Some(handler) = slot.borrow_mut().take() {
                    // No shim may fire once the line is gone.
                    HANDLERS[index].store(0, Ordering::Release);
                    // Dropping the handler unregisters the shim and frees the
                    // line.
                    drop(handler);
                }
            }
        }

        impl Drop for $TimerGroup {
            fn drop(&mut self) {
                self.disable_timer0_interrupt();
                self.disable_timer1_interrupt();
            }
        }
    };
}

impl_timer_group! {
    /// `TIMG0`: two general-purpose timers and the main system watchdog.
    ///
    /// The group owns the peripheral and its parts the way the hardware does:
    /// [`timer0`](Self::timer0) and [`timer1`](Self::timer1) are the two
    /// counters and [`wdt`](Self::wdt) is the watchdog, each with its own type
    /// so the registers, matrix source and interrupt shim are resolved at
    /// compile time. It also owns the CPU lines the counters use.
    TimerGroup0, TIMG0, 0, Timer00, Timer01, Watchdog0
}

impl_timer_group! {
    /// `TIMG1`: two general-purpose timers and the main system watchdog.
    ///
    /// See [`TimerGroup0`]; the parts are [`Timer10`], [`Timer11`] and
    /// [`Watchdog1`].
    TimerGroup1, TIMG1, 1, Timer10, Timer11, Watchdog1
}
