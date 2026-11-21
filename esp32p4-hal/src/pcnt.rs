//! Pulse count controller (PCNT).
//!
//! Four independent units, each with two channels and one 16-bit signed
//! counter. Both channels drive the same counter, so one unit can count two
//! pulse sources up or down.
//!
//! Every channel has a pulse input (`PCNT_SIG_CH0/1_PAD_IN0..3`) and a control
//! input (`PCNT_CTRL_CH0/1_PAD_IN0..3`); both are GPIO-matrix signals, so any
//! GPIO can carry them. Each unit also has one glitch filter that applies to
//! all four of its inputs.
//!
//! # Features
//!
//! - Rising, falling or both edges of the pulse input, each mapped to hold,
//!   count up or count down
//! - Control input level (high/low) selecting a different action each
//! - Per-unit glitch filter, in APB clock cycles
//! - Upper/lower limits with wrap-around, two thresholds, and a zero watch
//!   point, each with a latched status and an interrupt
//! - Counter clear and pause
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::gpio::signal::InputSignal;
//! use esp32p4_hal::gpio::{PinOps, Pins};
//! use esp32p4_hal::pcnt::{ChannelConfig, Pcnt, Trigger};
//!
//! let pcnt = Pcnt::init(dp.PCNT, &dp.HP_SYS_CLKRST);
//!
//! let mut pin = pins.pin::<2>();
//! pin.route_input_signal(InputSignal::PCNT_SIG_CH0_PAD_IN0);
//!
//! // The unit and the channel live in the type: `unit` is `Unit<'_, 0>`.
//! let unit = pcnt.unit::<0>();
//! unit.channel::<0>().configure(ChannelConfig::rising_edge());
//! unit.set_limits(0, 1000);
//! unit.enable(Trigger::HIGH_LIMIT);
//!
//! // ... after some pulses:
//! let count = unit.count();
//! if unit.latched().high_limit {
//!     unit.clear();
//! }
//! ```
//!
//! # Hardware Notes
//!
//! - The pulse mode (`POS_MODE`/`NEG_MODE`) and the control mode
//!   (`HCTRL_MODE`/`LCTRL_MODE`) spell the same three actions with different
//!   numbers: pulse `0 = hold, 1 = up, 2 = down`, control `0 = up, 1 = down,
//!   2 = hold`. [`EdgeAction`] and [`ControlMode`] keep them apart.
//! - Reconfiguring the limits only takes effect when the counter next reaches
//!   one of the five watch points; the thresholds apply immediately.
//! - The counter wraps at the limits rather than saturating, so a limit pair
//!   that is too narrow silently turns the count into a sawtooth.

use esp32p4_pac::{HP_SYS_CLKRST, PCNT, pcnt};

/// What a pulse edge does (`POS_MODE` / `NEG_MODE`).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EdgeAction {
    /// No counting.
    #[default]
    Hold,
    /// Count up.
    Increase,
    /// Count down.
    Decrease,
}

impl EdgeAction {
    const fn field(self) -> u32 {
        match self {
            Self::Hold => 0,
            Self::Increase => 1,
            Self::Decrease => 2,
        }
    }
}

/// What the control input level selects (`HCTRL_MODE` / `LCTRL_MODE`).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ControlMode {
    /// Count up.
    #[default]
    Increase,
    /// Count down.
    Decrease,
    /// Stop counting.
    Hold,
}

impl ControlMode {
    const fn field(self) -> u32 {
        match self {
            Self::Increase => 0,
            Self::Decrease => 1,
            Self::Hold => 2,
        }
    }
}

/// Static configuration of one channel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ChannelConfig {
    /// Action on a rising edge of the pulse input.
    pub rising: EdgeAction,
    /// Action on a falling edge of the pulse input.
    pub falling: EdgeAction,
    /// Action while the control input is high.
    pub high_control: ControlMode,
    /// Action while the control input is low.
    pub low_control: ControlMode,
}

impl ChannelConfig {
    /// Count every rising edge; the control input is ignored (held low).
    pub const fn rising_edge() -> Self {
        Self {
            rising: EdgeAction::Increase,
            falling: EdgeAction::Hold,
            high_control: ControlMode::Hold,
            low_control: ControlMode::Increase,
        }
    }

    /// Count every edge in one direction, i.e. a quadrature-style full step.
    pub const fn both_edges() -> Self {
        Self {
            rising: EdgeAction::Increase,
            falling: EdgeAction::Increase,
            high_control: ControlMode::Hold,
            low_control: ControlMode::Increase,
        }
    }

    /// Count up on a rising edge while the control input is low and down while
    /// it is high, the usual direction input of a rotary encoder.
    pub const fn direction_control() -> Self {
        Self {
            rising: EdgeAction::Increase,
            falling: EdgeAction::Hold,
            high_control: ControlMode::Decrease,
            low_control: ControlMode::Increase,
        }
    }
}

/// The five watch points of a unit.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Trigger {
    /// Counter reached the upper limit.
    pub high_limit: bool,
    /// Counter reached the lower limit.
    pub low_limit: bool,
    /// Counter equals threshold 0.
    pub threshold0: bool,
    /// Counter equals threshold 1.
    pub threshold1: bool,
    /// Counter is zero.
    pub zero: bool,
}

impl Trigger {
    /// Only the upper limit.
    pub const HIGH_LIMIT: Self = Self {
        high_limit: true,
        ..Self::new()
    };

    /// Only the lower limit.
    pub const LOW_LIMIT: Self = Self {
        low_limit: true,
        ..Self::new()
    };

    /// Nothing enabled.
    pub const fn new() -> Self {
        Self {
            high_limit: false,
            low_limit: false,
            threshold0: false,
            threshold1: false,
            zero: false,
        }
    }

    /// Any watch point set.
    pub const fn any(self) -> bool {
        self.high_limit || self.low_limit || self.threshold0 || self.threshold1 || self.zero
    }
}

/// Errors from the configuration setters.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The filter threshold exceeds 1023 APB clock cycles.
    FilterThreshold,
}

/// Pulse count controller owning the PAC peripheral.
pub struct Pcnt {
    pcnt: PCNT,
}

impl Pcnt {
    /// Initialises the controller's bus clock and releases its reset.
    pub fn init(pcnt: PCNT, clkrst: &HP_SYS_CLKRST) -> Self {
        clkrst.soc_clk_ctrl2.set_reg_pcnt_apb_clk_en();
        clkrst.hp_rst_en1.set_reg_rst_en_pcnt();
        clkrst.hp_rst_en1.clr_reg_rst_en_pcnt();
        Self { pcnt }
    }

    /// Obtains a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure the controller was initialised and that the
    /// returned handle is not used to reconfigure it concurrently.
    #[inline]
    pub unsafe fn steal() -> crate::Steal<Self> {
        crate::Steal(Self {
            pcnt: unsafe { PCNT::steal() },
        })
    }

    /// The underlying register block.
    #[inline]
    pub fn regs(&self) -> &pcnt::RegisterBlock {
        &self.pcnt
    }

    /// Returns the PAC peripheral.
    #[inline]
    pub fn release(self) -> PCNT {
        self.pcnt
    }

    /// A handle to unit `N`.
    ///
    /// `N` is 0–3; anything else fails to compile.
    #[inline]
    pub fn unit<const N: u8>(&self) -> Unit<'_, N> {
        const { assert!(N < 4, "this controller has four units, 0 to 3") };
        Unit(self)
    }
}

/// One counting unit, `N` of four.
///
/// The handle carries the unit number in its type, so a unit cannot be mixed up
/// with another one at run time; the register names behind it are literals that
/// the macros below generate once per unit.
pub struct Unit<'a, const N: u8>(&'a Pcnt);

impl<'a, const N: u8> Unit<'a, N> {
    /// A handle to channel `C` of this unit.
    ///
    /// `C` is 0 or 1; anything else fails to compile.
    #[inline]
    pub fn channel<const C: u8>(&self) -> Channel<'a, N, C> {
        const { assert!(C < 2, "a unit has two channels, 0 and 1") };
        Channel(self.0)
    }
}

/// One channel of one unit.
///
/// Both index parameters are part of the type, so `Channel<'_, 1, 0>` is a
/// different type from `Channel<'_, 0, 0>` and from `Channel<'_, 1, 1>`.
pub struct Channel<'a, const N: u8, const C: u8>(&'a Pcnt);

/// Turns the five watch-point enables into register writes.
///
/// The setters share their names across units, so this is written once and
/// expanded inside each unit's `impl`.
macro_rules! write_watch_points {
    ($conf0:expr, $trigger:expr) => {{
        let conf0 = $conf0;
        let trigger = $trigger;
        if trigger.high_limit {
            conf0.set_thr_h_lim_en_u();
        } else {
            conf0.clr_thr_h_lim_en_u();
        }
        if trigger.low_limit {
            conf0.set_thr_l_lim_en_u();
        } else {
            conf0.clr_thr_l_lim_en_u();
        }
        if trigger.threshold0 {
            conf0.set_thr_thres0_en_u();
        } else {
            conf0.clr_thr_thres0_en_u();
        }
        if trigger.threshold1 {
            conf0.set_thr_thres1_en_u();
        } else {
            conf0.clr_thr_thres1_en_u();
        }
        if trigger.zero {
            conf0.set_thr_zero_en_u();
        } else {
            conf0.clr_thr_zero_en_u();
        }
    }};
}

/// Generates the per-unit and per-channel methods.
///
/// The register block spells every unit out (`u0_conf0`, `u1_conf0`, …) instead
/// of holding an array, so the unit number has to be part of the *names*. Doing
/// that here, once per unit, keeps the driver free of any run-time dispatch: a
/// call on `Unit<'_, 1>` compiles to the address of `u1_conf0`.
macro_rules! unit_body {
    ($n:literal) => {
        pastey::paste! {
            impl<'a> Unit<'a, $n> {
                /// Enables the unit's glitch filter at `threshold` APB clock
                /// cycles, or turns it off with `None`.
                ///
                /// # Errors
                ///
                /// [`Error::FilterThreshold`] when the threshold exceeds 1023.
                pub fn configure_filter(&self, threshold: Option<u16>) -> Result<(), Error> {
                    if let Some(threshold) = threshold
                        && threshold > 1023
                    {
                        return Err(Error::FilterThreshold);
                    }
                    let conf0 = &self.0.regs().[<u $n _conf0>];
                    match threshold {
                        Some(threshold) => {
                            conf0.write_filter_thres_u(u32::from(threshold));
                            conf0.set_filter_en_u();
                        }
                        None => conf0.clr_filter_en_u(),
                    }
                    Ok(())
                }

                /// Sets the wrap-around limits.
                ///
                /// Counting up past `high` wraps to the lower limit, counting
                /// down past `low` wraps to the upper one.
                pub fn set_limits(&self, low: i16, high: i16) {
                    let conf2 = &self.0.regs().[<u $n _conf2>];
                    conf2.write_cnt_l_lim_u(low as u16 as u32);
                    conf2.write_cnt_h_lim_u(high as u16 as u32);
                }

                /// Sets the two intermediate thresholds, which apply at once.
                pub fn set_thresholds(&self, threshold0: i16, threshold1: i16) {
                    let conf1 = &self.0.regs().[<u $n _conf1>];
                    conf1.write_cnt_thres0_u(threshold0 as u16 as u32);
                    conf1.write_cnt_thres1_u(threshold1 as u16 as u32);
                }

                /// The counter, as a signed value.
                pub fn count(&self) -> i16 {
                    self.0.regs().[<u $n _cnt>].pulse_cnt_u() as u16 as i16
                }

                /// Resets the counter to zero.
                ///
                /// `PULSE_CNT_RST_Un` is an ordinary read/write bit, not a
                /// self-clearing one, so it is set and then cleared; leaving it
                /// set would hold the counter at zero.
                pub fn clear(&self) {
                    let ctrl = &self.0.regs().ctrl;
                    ctrl.[<set_pulse_cnt_rst_u $n>]();
                    ctrl.[<clr_pulse_cnt_rst_u $n>]();
                }

                /// Freezes or resumes the counter, keeping its value.
                pub fn pause(&self, pause: bool) {
                    // `CNT_PAUSE_Un` is a plain read/write bit.
                    let ctrl = &self.0.regs().ctrl;
                    if pause {
                        ctrl.[<set_cnt_pause_u $n>]();
                    } else {
                        ctrl.[<clr_cnt_pause_u $n>]();
                    }
                }

                /// Enables exactly the watch points in `trigger`, both as watch
                /// points and as interrupt sources.
                pub fn enable(&self, trigger: Trigger) {
                    let conf0 = &self.0.regs().[<u $n _conf0>];
                    write_watch_points!(conf0, trigger);
                    let int_ena = &self.0.regs().int_ena;
                    if trigger.any() {
                        int_ena.[<set_cnt_thr_event_u $n _int_ena>]();
                    } else {
                        int_ena.[<clr_cnt_thr_event_u $n _int_ena>]();
                    }
                }

                /// Which watch points have fired since the last
                /// [`clear_latched`](Self::clear_latched).
                pub fn latched(&self) -> Trigger {
                    let status = &self.0.regs().[<u $n _status>];
                    Trigger {
                        high_limit: status.cnt_thr_h_lim_lat_u(),
                        low_limit: status.cnt_thr_l_lim_lat_u(),
                        threshold0: status.cnt_thr_thres0_lat_u(),
                        threshold1: status.cnt_thr_thres1_lat_u(),
                        zero: status.cnt_thr_zero_lat_u(),
                    }
                }

                /// Clears the latched watch-point status and the unit's
                /// interrupt.
                pub fn clear_latched(&self) {
                    self.0
                        .regs()
                        .int_clr
                        .[<trigger_cnt_thr_event_u $n _int_clr>]();
                }

                /// Whether the unit has a pending watch-point interrupt.
                pub fn pending(&self) -> bool {
                    self.0.regs().int_raw.[<cnt_thr_event_u $n _int_raw>]()
                }
            }

            impl<'a> Channel<'a, $n, 0> {
                /// Programs this channel's edge and control actions.
                pub fn configure(&self, config: ChannelConfig) {
                    let conf0 = &self.0.regs().[<u $n _conf0>];
                    conf0.write_ch0_pos_mode_u(config.rising.field());
                    conf0.write_ch0_neg_mode_u(config.falling.field());
                    conf0.write_ch0_hctrl_mode_u(config.high_control.field());
                    conf0.write_ch0_lctrl_mode_u(config.low_control.field());
                }
            }

            impl<'a> Channel<'a, $n, 1> {
                /// Programs this channel's edge and control actions.
                pub fn configure(&self, config: ChannelConfig) {
                    let conf0 = &self.0.regs().[<u $n _conf0>];
                    conf0.write_ch1_pos_mode_u(config.rising.field());
                    conf0.write_ch1_neg_mode_u(config.falling.field());
                    conf0.write_ch1_hctrl_mode_u(config.high_control.field());
                    conf0.write_ch1_lctrl_mode_u(config.low_control.field());
                }
            }
        }
    };
}

unit_body!(0);
unit_body!(1);
unit_body!(2);
unit_body!(3);
