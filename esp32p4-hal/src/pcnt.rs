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
//! use esp32p4_hal::pcnt::{ChannelConfig, ChannelIndex, Pcnt, Trigger, UnitIndex};
//!
//! let pcnt = Pcnt::init(dp.PCNT, &dp.HP_SYS_CLKRST);
//!
//! let mut pin = pins.pin::<2>();
//! pin.route_input_signal(InputSignal::PCNT_SIG_CH0_PAD_IN0);
//!
//! pcnt.configure_channel(UnitIndex::Zero, ChannelIndex::Zero, ChannelConfig::rising_edge());
//! pcnt.set_limits(UnitIndex::Zero, 0, 1000);
//! pcnt.enable(UnitIndex::Zero, Trigger::HIGH_LIMIT);
//!
//! // ... after some pulses:
//! let count = pcnt.count(UnitIndex::Zero);
//! if pcnt.latched(UnitIndex::Zero).high_limit {
//!     pcnt.clear(UnitIndex::Zero);
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

/// A counting unit: hardware unit 0–3.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UnitIndex {
    /// Unit 0.
    Zero,
    /// Unit 1.
    One,
    /// Unit 2.
    Two,
    /// Unit 3.
    Three,
}

/// One of the two channels of a unit.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChannelIndex {
    /// Channel 0.
    Zero,
    /// Channel 1.
    One,
}

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

trait UnitConf0 {
    fn set_channel(&self, channel: ChannelIndex, config: &ChannelConfig);
    fn set_filter(&self, threshold: Option<u16>);
    fn set_watch_points(&self, trigger: Trigger);
}

trait UnitConf1 {
    fn set_thresholds(&self, threshold0: i16, threshold1: i16);
}

trait UnitConf2 {
    fn set_limits(&self, low: i16, high: i16);
}

trait UnitStatus {
    fn latched(&self) -> Trigger;
}

macro_rules! impl_unit {
    ($conf0:ident, $conf1:ident, $conf2:ident, $status:ident) => {
        impl UnitConf0 for pcnt::$conf0 {
            fn set_channel(&self, channel: ChannelIndex, config: &ChannelConfig) {
                match channel {
                    ChannelIndex::Zero => {
                        self.write_ch0_pos_mode_u(config.rising.field());
                        self.write_ch0_neg_mode_u(config.falling.field());
                        self.write_ch0_hctrl_mode_u(config.high_control.field());
                        self.write_ch0_lctrl_mode_u(config.low_control.field());
                    }
                    ChannelIndex::One => {
                        self.write_ch1_pos_mode_u(config.rising.field());
                        self.write_ch1_neg_mode_u(config.falling.field());
                        self.write_ch1_hctrl_mode_u(config.high_control.field());
                        self.write_ch1_lctrl_mode_u(config.low_control.field());
                    }
                }
            }

            fn set_filter(&self, threshold: Option<u16>) {
                match threshold {
                    Some(threshold) => {
                        self.write_filter_thres_u(u32::from(threshold));
                        self.set_filter_en_u();
                    }
                    None => self.clr_filter_en_u(),
                }
            }

            fn set_watch_points(&self, trigger: Trigger) {
                if trigger.high_limit {
                    self.set_thr_h_lim_en_u();
                } else {
                    self.clr_thr_h_lim_en_u();
                }
                if trigger.low_limit {
                    self.set_thr_l_lim_en_u();
                } else {
                    self.clr_thr_l_lim_en_u();
                }
                if trigger.threshold0 {
                    self.set_thr_thres0_en_u();
                } else {
                    self.clr_thr_thres0_en_u();
                }
                if trigger.threshold1 {
                    self.set_thr_thres1_en_u();
                } else {
                    self.clr_thr_thres1_en_u();
                }
                if trigger.zero {
                    self.set_thr_zero_en_u();
                } else {
                    self.clr_thr_zero_en_u();
                }
            }
        }

        impl UnitConf1 for pcnt::$conf1 {
            fn set_thresholds(&self, threshold0: i16, threshold1: i16) {
                self.write_cnt_thres0_u(threshold0 as u16 as u32);
                self.write_cnt_thres1_u(threshold1 as u16 as u32);
            }
        }

        impl UnitConf2 for pcnt::$conf2 {
            fn set_limits(&self, low: i16, high: i16) {
                self.write_cnt_l_lim_u(low as u16 as u32);
                self.write_cnt_h_lim_u(high as u16 as u32);
            }
        }

        impl UnitStatus for pcnt::$status {
            fn latched(&self) -> Trigger {
                Trigger {
                    high_limit: self.cnt_thr_h_lim_lat_u(),
                    low_limit: self.cnt_thr_l_lim_lat_u(),
                    threshold0: self.cnt_thr_thres0_lat_u(),
                    threshold1: self.cnt_thr_thres1_lat_u(),
                    zero: self.cnt_thr_zero_lat_u(),
                }
            }
        }
    };
}

impl_unit!(U0Conf0, U0Conf1, U0Conf2, U0Status);
impl_unit!(U1Conf0, U1Conf1, U1Conf2, U1Status);
impl_unit!(U2Conf0, U2Conf1, U2Conf2, U2Status);
impl_unit!(U3Conf0, U3Conf1, U3Conf2, U3Status);

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

    fn conf0(&self, unit: UnitIndex) -> &dyn UnitConf0 {
        match unit {
            UnitIndex::Zero => &self.pcnt.u0_conf0,
            UnitIndex::One => &self.pcnt.u1_conf0,
            UnitIndex::Two => &self.pcnt.u2_conf0,
            UnitIndex::Three => &self.pcnt.u3_conf0,
        }
    }

    fn conf1(&self, unit: UnitIndex) -> &dyn UnitConf1 {
        match unit {
            UnitIndex::Zero => &self.pcnt.u0_conf1,
            UnitIndex::One => &self.pcnt.u1_conf1,
            UnitIndex::Two => &self.pcnt.u2_conf1,
            UnitIndex::Three => &self.pcnt.u3_conf1,
        }
    }

    fn conf2(&self, unit: UnitIndex) -> &dyn UnitConf2 {
        match unit {
            UnitIndex::Zero => &self.pcnt.u0_conf2,
            UnitIndex::One => &self.pcnt.u1_conf2,
            UnitIndex::Two => &self.pcnt.u2_conf2,
            UnitIndex::Three => &self.pcnt.u3_conf2,
        }
    }

    fn status(&self, unit: UnitIndex) -> &dyn UnitStatus {
        match unit {
            UnitIndex::Zero => &self.pcnt.u0_status,
            UnitIndex::One => &self.pcnt.u1_status,
            UnitIndex::Two => &self.pcnt.u2_status,
            UnitIndex::Three => &self.pcnt.u3_status,
        }
    }

    /// Configures one channel's edge and control actions.
    pub fn configure_channel(&self, unit: UnitIndex, channel: ChannelIndex, config: ChannelConfig) {
        self.conf0(unit).set_channel(channel, &config);
    }

    /// Enables the unit's glitch filter at `threshold` APB clock cycles, or
    /// turns it off with `None`.
    ///
    /// # Errors
    ///
    /// [`Error::FilterThreshold`] when the threshold exceeds 1023 cycles.
    pub fn configure_filter(&self, unit: UnitIndex, threshold: Option<u16>) -> Result<(), Error> {
        if let Some(threshold) = threshold
            && threshold > 1023
        {
            return Err(Error::FilterThreshold);
        }
        self.conf0(unit).set_filter(threshold);
        Ok(())
    }

    /// Sets the wrap-around limits.
    ///
    /// Counting up past `high` wraps to the lower limit, counting down past
    /// `low` wraps to the upper one.
    pub fn set_limits(&self, unit: UnitIndex, low: i16, high: i16) {
        self.conf2(unit).set_limits(low, high);
    }

    /// Sets the two intermediate thresholds, which apply immediately.
    pub fn set_thresholds(&self, unit: UnitIndex, threshold0: i16, threshold1: i16) {
        self.conf1(unit).set_thresholds(threshold0, threshold1);
    }

    /// The counter, as a signed value.
    pub fn count(&self, unit: UnitIndex) -> i16 {
        let raw = match unit {
            UnitIndex::Zero => self.pcnt.u0_cnt.pulse_cnt_u(),
            UnitIndex::One => self.pcnt.u1_cnt.pulse_cnt_u(),
            UnitIndex::Two => self.pcnt.u2_cnt.pulse_cnt_u(),
            UnitIndex::Three => self.pcnt.u3_cnt.pulse_cnt_u(),
        };
        raw as u16 as i16
    }

    /// Resets the counter to zero.
    ///
    /// `PULSE_CNT_RST_Un` is an ordinary read/write bit, not a self-clearing
    /// one, so it is set and then cleared; leaving it set would hold the
    /// counter at zero.
    pub fn clear(&self, unit: UnitIndex) {
        let ctrl = &self.pcnt.ctrl;
        match unit {
            UnitIndex::Zero => {
                ctrl.set_pulse_cnt_rst_u0();
                ctrl.clr_pulse_cnt_rst_u0();
            }
            UnitIndex::One => {
                ctrl.set_pulse_cnt_rst_u1();
                ctrl.clr_pulse_cnt_rst_u1();
            }
            UnitIndex::Two => {
                ctrl.set_pulse_cnt_rst_u2();
                ctrl.clr_pulse_cnt_rst_u2();
            }
            UnitIndex::Three => {
                ctrl.set_pulse_cnt_rst_u3();
                ctrl.clr_pulse_cnt_rst_u3();
            }
        }
    }

    /// Freezes or resumes the counter, keeping its value.
    pub fn pause(&self, unit: UnitIndex, pause: bool) {
        // `CNT_PAUSE_Un` is a plain read/write bit.
        let ctrl = &self.pcnt.ctrl;
        match (unit, pause) {
            (UnitIndex::Zero, true) => ctrl.set_cnt_pause_u0(),
            (UnitIndex::Zero, false) => ctrl.clr_cnt_pause_u0(),
            (UnitIndex::One, true) => ctrl.set_cnt_pause_u1(),
            (UnitIndex::One, false) => ctrl.clr_cnt_pause_u1(),
            (UnitIndex::Two, true) => ctrl.set_cnt_pause_u2(),
            (UnitIndex::Two, false) => ctrl.clr_cnt_pause_u2(),
            (UnitIndex::Three, true) => ctrl.set_cnt_pause_u3(),
            (UnitIndex::Three, false) => ctrl.clr_cnt_pause_u3(),
        }
    }

    /// Enables exactly the watch points in `trigger`, both as watch points and
    /// as interrupt sources.
    pub fn enable(&self, unit: UnitIndex, trigger: Trigger) {
        self.conf0(unit).set_watch_points(trigger);
        match (unit, trigger.any()) {
            (UnitIndex::Zero, true) => self.pcnt.int_ena.set_cnt_thr_event_u0_int_ena(),
            (UnitIndex::Zero, false) => self.pcnt.int_ena.clr_cnt_thr_event_u0_int_ena(),
            (UnitIndex::One, true) => self.pcnt.int_ena.set_cnt_thr_event_u1_int_ena(),
            (UnitIndex::One, false) => self.pcnt.int_ena.clr_cnt_thr_event_u1_int_ena(),
            (UnitIndex::Two, true) => self.pcnt.int_ena.set_cnt_thr_event_u2_int_ena(),
            (UnitIndex::Two, false) => self.pcnt.int_ena.clr_cnt_thr_event_u2_int_ena(),
            (UnitIndex::Three, true) => self.pcnt.int_ena.set_cnt_thr_event_u3_int_ena(),
            (UnitIndex::Three, false) => self.pcnt.int_ena.clr_cnt_thr_event_u3_int_ena(),
        }
    }

    /// Which watch points have fired since the last [`clear_latched`].
    ///
    /// [`clear_latched`]: Self::clear_latched
    pub fn latched(&self, unit: UnitIndex) -> Trigger {
        self.status(unit).latched()
    }

    /// Clears the latched watch-point status and the unit's interrupt.
    pub fn clear_latched(&self, unit: UnitIndex) {
        match unit {
            UnitIndex::Zero => self.pcnt.int_clr.trigger_cnt_thr_event_u0_int_clr(),
            UnitIndex::One => self.pcnt.int_clr.trigger_cnt_thr_event_u1_int_clr(),
            UnitIndex::Two => self.pcnt.int_clr.trigger_cnt_thr_event_u2_int_clr(),
            UnitIndex::Three => self.pcnt.int_clr.trigger_cnt_thr_event_u3_int_clr(),
        }
    }

    /// Whether the unit has a pending watch-point interrupt.
    pub fn pending(&self, unit: UnitIndex) -> bool {
        match unit {
            UnitIndex::Zero => self.pcnt.int_raw.cnt_thr_event_u0_int_raw(),
            UnitIndex::One => self.pcnt.int_raw.cnt_thr_event_u1_int_raw(),
            UnitIndex::Two => self.pcnt.int_raw.cnt_thr_event_u2_int_raw(),
            UnitIndex::Three => self.pcnt.int_raw.cnt_thr_event_u3_int_raw(),
        }
    }
}
