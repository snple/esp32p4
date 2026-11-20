//! Motor control PWM (MCPWM).
//!
//! Each of the two controllers (MCPWM0 and MCPWM1) has three timers and three
//! operators; every operator drives a `PWMxA` / `PWMxB` output pair through a
//! dead-time generator. The outputs are GPIO-matrix signals
//! (`PWM0_CH0_A_PAD_OUT` and friends), so any GPIO can carry them.
//!
//! # Features
//!
//! - Timers: `XTAL_40M` / `RC_FAST` / `PLL_F160M` clock with a group divider,
//!   per-timer prescaler, up / down / up-down counting, start and stop, and
//!   counter readback
//! - Operators: comparator A/B, per-event output actions (hold, low, high,
//!   toggle) on timer zero, timer period and both comparators
//! - Dead time: separate rising and falling delays with optional inversion
//! - Edge-aligned and complementary PWM built from those pieces
//!
//! Not implemented: fault detection, capture, carrier modulation, sync inputs
//! and the ETM hooks.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::gpio::signal::OutputSignal;
//! use esp32p4_hal::gpio::{PinOps, Pins};
//! use esp32p4_hal::mcpwm::{
//!     ClockSource, GeneratorConfig, Mcpwm0, OperatorIndex, TimerConfig, TimerIndex,
//! };
//!
//! let mcpwm = Mcpwm0::init(dp.MCPWM0, &dp.HP_SYS_CLKRST);
//! mcpwm.set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::PllF160m, 1).unwrap();
//! mcpwm.configure_timer(TimerIndex::Zero, TimerConfig::new(20_000)).unwrap();
//!
//! let mut pin = pins.pin::<2>();
//! pin.route_output_signal(OutputSignal::PWM0_CH0_A_PAD_OUT);
//!
//! // 25 % on output A, 50 % on output B, both on the same timer.
//! mcpwm
//!     .configure_generator(
//!         OperatorIndex::Zero,
//!         GeneratorConfig::edge_aligned(TimerIndex::Zero, 60, 30),
//!     )
//!     .unwrap();
//! mcpwm.start(TimerIndex::Zero);
//! ```
//!
//! # Hardware Notes
//!
//! - A PWM output only moves when the generator has an action for the event;
//!   leaving `TEZ`/`TEA` on hold produces a flat line, not a waveform.
//! - The update-method fields are write-only: the driver writes 1 (apply on the
//!   next timer-zero event) for the period and generates actions, so a change
//!   can take up to one PWM period to appear.
//! - The dead-time generator is bypassed per output out of reset; the driver
//!   only un-bypasses it when a [`DeadTime`] is configured.

use esp32p4_pac::{HP_SYS_CLKRST, MCPWM0, MCPWM1, mcpwm0, mcpwm1};

/// Which of the three timers drives an operator.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimerIndex {
    /// Timer 0.
    Zero,
    /// Timer 1.
    One,
    /// Timer 2.
    Two,
}

impl TimerIndex {
    const fn field(self) -> u32 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
        }
    }
}

/// A PWM operator, i.e. an output pair.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OperatorIndex {
    /// Operator 0 (`PWM0A`/`PWM0B`).
    Zero,
    /// Operator 1.
    One,
    /// Operator 2.
    Two,
}

/// Clock feeding the prescaler.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockSource {
    /// `XTAL_CLK`, 40 MHz.
    Xtal40m,
    /// `RC_FAST_CLK`.
    RcFast,
    /// `PLL_F160M_CLK`, 160 MHz. The reset default.
    #[default]
    PllF160m,
}

impl ClockSource {
    /// Nominal frequency in Hz.
    pub const fn hz(self) -> u32 {
        match self {
            Self::Xtal40m => crate::clock::XTAL_HZ,
            Self::RcFast => crate::clock::RC_FAST_HZ,
            Self::PllF160m => 160_000_000,
        }
    }

    const fn field(self) -> u32 {
        match self {
            Self::Xtal40m => 0,
            Self::RcFast => 1,
            Self::PllF160m => 2,
        }
    }
}

/// Counting mode of a timer.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CountMode {
    /// Count up from zero to the period, then wrap.
    #[default]
    Up,
    /// Count down from the period to zero, then wrap.
    Down,
    /// Count up, then down, then up again.
    UpDown,
}

impl CountMode {
    const fn field(self) -> u32 {
        match self {
            Self::Up => 1,
            Self::Down => 2,
            Self::UpDown => 3,
        }
    }
}

/// What an output does when an event occurs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwmAction {
    /// Leave the output as it is.
    #[default]
    Hold,
    /// Drive low.
    Low,
    /// Drive high.
    High,
    /// Flip the level.
    Toggle,
}

impl PwmAction {
    const fn field(self) -> u32 {
        match self {
            Self::Hold => 0,
            Self::Low => 1,
            Self::High => 2,
            Self::Toggle => 3,
        }
    }
}

/// Timer setup.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TimerConfig {
    /// PWM frequency in Hz.
    pub frequency: u32,
    /// Clock source; the timer derives its prescaler from
    /// [`ClockSource::hz`], so keep it in step with
    /// [`Mcpwm0::set_clock_source`].
    pub clock: ClockSource,
    /// Counting mode.
    pub mode: CountMode,
}

impl TimerConfig {
    /// A timer on the default clock (`PLL_F160M`) counting up.
    pub const fn new(frequency: u32) -> Self {
        Self {
            frequency,
            clock: ClockSource::PllF160m,
            mode: CountMode::Up,
        }
    }
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self::new(20_000)
    }
}

/// Inputs of one generator output.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutputActions {
    /// Action when the timer reaches zero.
    pub zero: PwmAction,
    /// Action when the timer reaches the period.
    pub period: PwmAction,
    /// Action when the counter matches comparator A.
    pub comparator_a: PwmAction,
    /// Action when the counter matches comparator B.
    pub comparator_b: PwmAction,
}

impl OutputActions {
    /// All actions on hold.
    pub const fn new() -> Self {
        Self {
            zero: PwmAction::Hold,
            period: PwmAction::Hold,
            comparator_a: PwmAction::Hold,
            comparator_b: PwmAction::Hold,
        }
    }
}

/// Dead-time settings for one operator.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DeadTime {
    /// Rising-edge delay, in dead-time clock ticks.
    pub rising: u16,
    /// Falling-edge delay, in dead-time clock ticks.
    pub falling: u16,
    /// Invert output A after the dead-time generator.
    pub invert_a: bool,
    /// Invert output B after the dead-time generator.
    pub invert_b: bool,
}

/// Operator setup.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GeneratorConfig {
    /// Timer that drives the operator.
    pub timer: TimerIndex,
    /// Value of comparator A, in timer ticks.
    pub comparator_a: u16,
    /// Value of comparator B, in timer ticks.
    pub comparator_b: u16,
    /// Actions for output A.
    pub actions_a: OutputActions,
    /// Actions for output B.
    pub actions_b: OutputActions,
    /// Dead time, or `None` to pass both outputs through unchanged.
    pub dead_time: Option<DeadTime>,
}

impl GeneratorConfig {
    /// Two independent edge-aligned outputs sharing one timer.
    ///
    /// Output A rises at the start of the period and falls at `duty_a`, output
    /// B does the same at `duty_b`; both are in timer ticks and the period is
    /// `timer value + 1`.
    pub const fn edge_aligned(timer: TimerIndex, duty_a: u16, duty_b: u16) -> Self {
        Self {
            timer,
            comparator_a: duty_a,
            comparator_b: duty_b,
            actions_a: OutputActions {
                zero: PwmAction::High,
                comparator_a: PwmAction::Low,
                ..OutputActions::new()
            },
            actions_b: OutputActions {
                zero: PwmAction::High,
                comparator_b: PwmAction::Low,
                ..OutputActions::new()
            },
            dead_time: None,
        }
    }

    /// A complementary pair: A is high for `duty` ticks, B is its inverse, and
    /// the dead time keeps both low around the transitions.
    pub const fn complementary(timer: TimerIndex, duty: u16, dead_time: DeadTime) -> Self {
        Self {
            timer,
            comparator_a: duty,
            comparator_b: 0,
            actions_a: OutputActions {
                zero: PwmAction::High,
                comparator_a: PwmAction::Low,
                ..OutputActions::new()
            },
            actions_b: OutputActions {
                zero: PwmAction::Low,
                comparator_a: PwmAction::High,
                ..OutputActions::new()
            },
            dead_time: Some(dead_time),
        }
    }
}

/// Errors from the configuration setters.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The frequency is at or above the clock, or the divider is 0.
    FrequencyTooHigh,
    /// The frequency needs a prescaler beyond 256.
    FrequencyTooLow,
    /// The timer's period is below `period + 1` ticks for the comparators.
    ComparatorOutOfRange,
}

// ============================================================================
// Register traits
// ============================================================================

trait TimerPeriod {
    fn write_period(&self, prescale: u32, period: u32);
}

trait TimerControl {
    fn write_mode(&self, mode: CountMode);
    fn write_start(&self, enable: bool);
}

trait TimerValue {
    fn read_value(&self) -> u32;
}

trait ComparatorValue {
    fn write_value(&self, value: u32);
}

trait ComparatorUpdate {
    fn write_update(&self);
}

trait GeneratorActions {
    fn write_action(&self, event: ComparatorEvent, rising: bool, value: u32);
}

trait DeadTimeRegs {
    fn write_dead_time(&self, dead_time: DeadTime);
    fn bypass(&self);
    fn invert(&self, a: bool, b: bool);
}

trait DeadTimeValue {
    fn write_delay(&self, value: u32);
}

/// Which comparator/timer event an action belongs to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ComparatorEvent {
    Zero,
    Period,
    ComparatorA,
    ComparatorB,
}

macro_rules! impl_timer {
    ($m:ident, $cfg0:ident, $cfg1:ident, $status:ident) => {
        impl TimerPeriod for $m::$cfg0 {
            fn write_period(&self, prescale: u32, period: u32) {
                self.write_timer_prescale(prescale);
                self.write_timer_period_upmethod(0b0001);
                self.write_timer_period(period);
            }
        }

        impl TimerControl for $m::$cfg1 {
            fn write_mode(&self, mode: CountMode) {
                self.write_timer_mod(mode.field());
            }

            fn write_start(&self, enable: bool) {
                // TIMER_START is a 3-bit command, not a run flag: 2 = start
                // and run, 0 = stop (and 1 = "stop at the next period", which
                // reads like "start" but never does).
                self.write_timer_start(if enable { 2 } else { 0 });
            }
        }

        impl TimerValue for $m::$status {
            fn read_value(&self) -> u32 {
                self.timer_value()
            }
        }
    };
}

impl_timer!(mcpwm0, Timer0Cfg0, Timer0Cfg1, Timer0Status);
impl_timer!(mcpwm0, Timer1Cfg0, Timer1Cfg1, Timer1Status);
impl_timer!(mcpwm0, Timer2Cfg0, Timer2Cfg1, Timer2Status);
impl_timer!(mcpwm1, Timer0Cfg0, Timer0Cfg1, Timer0Status);
impl_timer!(mcpwm1, Timer1Cfg0, Timer1Cfg1, Timer1Status);
impl_timer!(mcpwm1, Timer2Cfg0, Timer2Cfg1, Timer2Status);

macro_rules! impl_comparator {
    ($m:ident, $tstmp_a:ident, $tstmp_b:ident, $stmp_cfg:ident) => {
        impl ComparatorValue for $m::$tstmp_a {
            fn write_value(&self, value: u32) {
                self.write_cmpr_a(value);
            }
        }

        impl ComparatorValue for $m::$tstmp_b {
            fn write_value(&self, value: u32) {
                self.write_cmpr_b(value);
            }
        }

        impl ComparatorUpdate for $m::$stmp_cfg {
            fn write_update(&self) {
                self.write_cmpr_a_upmethod(0b0001);
                self.write_cmpr_b_upmethod(0b0001);
            }
        }
    };
}

impl_comparator!(mcpwm0, Gen0TstmpA, Gen0TstmpB, Gen0StmpCfg);
impl_comparator!(mcpwm0, Gen1TstmpA, Gen1TstmpB, Gen1StmpCfg);
impl_comparator!(mcpwm0, Gen2TstmpA, Gen2TstmpB, Gen2StmpCfg);
impl_comparator!(mcpwm1, Gen0TstmpA, Gen0TstmpB, Gen0StmpCfg);
impl_comparator!(mcpwm1, Gen1TstmpA, Gen1TstmpB, Gen1StmpCfg);
impl_comparator!(mcpwm1, Gen2TstmpA, Gen2TstmpB, Gen2StmpCfg);

macro_rules! impl_generator {
    ($m:ident, $gen:ident) => {
        pastey::paste! {
            impl GeneratorActions for $m::$gen {
                fn write_action(&self, event: ComparatorEvent, rising: bool, value: u32) {
                    match (event, rising) {
                        (ComparatorEvent::Zero, true) => self.[<write_ut0>](value),
                        (ComparatorEvent::Zero, false) => self.[<write_dt0>](value),
                        (ComparatorEvent::Period, true) => self.[<write_utep>](value),
                        (ComparatorEvent::Period, false) => self.[<write_dtep>](value),
                        (ComparatorEvent::ComparatorA, true) => self.[<write_utea>](value),
                        (ComparatorEvent::ComparatorA, false) => self.[<write_dtea>](value),
                        (ComparatorEvent::ComparatorB, true) => self.[<write_uteb>](value),
                        (ComparatorEvent::ComparatorB, false) => self.[<write_dteb>](value),
                    }
                }
            }
        }
    };
}

impl_generator!(mcpwm0, Gen0A);
impl_generator!(mcpwm0, Gen0B);
impl_generator!(mcpwm0, Gen1A);
impl_generator!(mcpwm0, Gen1B);
impl_generator!(mcpwm0, Gen2A);
impl_generator!(mcpwm0, Gen2B);
impl_generator!(mcpwm1, Gen0A);
impl_generator!(mcpwm1, Gen0B);
impl_generator!(mcpwm1, Gen1A);
impl_generator!(mcpwm1, Gen1B);
impl_generator!(mcpwm1, Gen2A);
impl_generator!(mcpwm1, Gen2B);

macro_rules! impl_dead_time {
    ($m:ident, $dt_cfg:ident, $dt_fed:ident, $dt_red:ident) => {
        impl DeadTimeRegs for $m::$dt_cfg {
            fn write_dead_time(&self, _dead_time: DeadTime) {
                self.write_db_fed_upmethod(0b0001);
                self.write_db_red_upmethod(0b0001);
            }

            fn bypass(&self) {
                self.set_db_a_outbypass();
                self.set_db_b_outbypass();
            }

            fn invert(&self, a: bool, b: bool) {
                self.clr_db_a_outbypass();
                self.clr_db_b_outbypass();
                if a {
                    self.set_db_a_outswap();
                } else {
                    self.clr_db_a_outswap();
                }
                if b {
                    self.set_db_b_outswap();
                } else {
                    self.clr_db_b_outswap();
                }
            }
        }

        impl DeadTimeValue for $m::$dt_fed {
            fn write_delay(&self, value: u32) {
                self.write_db_fed(value);
            }
        }

        impl DeadTimeValue for $m::$dt_red {
            fn write_delay(&self, value: u32) {
                self.write_db_red(value);
            }
        }
    };
}

impl_dead_time!(mcpwm0, Dt0Cfg, Dt0FedCfg, Dt0RedCfg);
impl_dead_time!(mcpwm0, Dt1Cfg, Dt1FedCfg, Dt1RedCfg);
impl_dead_time!(mcpwm0, Dt2Cfg, Dt2FedCfg, Dt2RedCfg);
impl_dead_time!(mcpwm1, Dt0Cfg, Dt0FedCfg, Dt0RedCfg);
impl_dead_time!(mcpwm1, Dt1Cfg, Dt1FedCfg, Dt1RedCfg);
impl_dead_time!(mcpwm1, Dt2Cfg, Dt2FedCfg, Dt2RedCfg);

fn write_actions(generator: &dyn GeneratorActions, actions: &OutputActions) {
    for (event, action) in [
        (ComparatorEvent::Zero, actions.zero),
        (ComparatorEvent::Period, actions.period),
        (ComparatorEvent::ComparatorA, actions.comparator_a),
        (ComparatorEvent::ComparatorB, actions.comparator_b),
    ] {
        generator.write_action(event, true, action.field());
        generator.write_action(event, false, action.field());
    }
}

macro_rules! impl_mcpwm {
    ($Name:ident, $PAC:ident, $mod:ident, $NUM:literal, $apb:ident, $clk_en:ident, $src_sel:ident, $div:ident, $rst:ident) => {
        pastey::paste! {
            /// MCPWM controller owning the PAC peripheral.
            pub struct $Name {
                mcpwm: $PAC,
            }

            impl $Name {
                /// Controller number, 0 or 1.
                pub const NUM: u8 = $NUM;

                /// Initialises the bus clock, the peripheral clock and releases
                /// the reset.
                pub fn init(mcpwm: $PAC, clkrst: &HP_SYS_CLKRST) -> Self {
                    clkrst.soc_clk_ctrl2.[<set_ $apb>]();
                    clkrst.hp_rst_en1.[<set_ $rst>]();
                    clkrst.hp_rst_en1.[<clr_ $rst>]();
                    clkrst.peri_clk_ctrl20.[<set_ $clk_en>]();
                    Self { mcpwm }
                }

                /// Obtains a handle without ownership checks.
                ///
                /// # Safety
                ///
                /// The caller must ensure the controller was initialised and
                /// that the returned handle is not used to reconfigure it
                /// concurrently.
                #[inline]
                pub unsafe fn steal() -> crate::Steal<Self> {
                    crate::Steal(Self {
                        mcpwm: unsafe { $PAC::steal() },
                    })
                }

                /// The underlying register block.
                #[inline]
                pub fn regs(&self) -> &$mod::RegisterBlock {
                    &self.mcpwm
                }

                /// Returns the PAC peripheral.
                #[inline]
                pub fn release(self) -> $PAC {
                    self.mcpwm
                }

                /// Selects the clock source and its group divider.
                ///
                /// # Errors
                ///
                /// [`Error::FrequencyTooHigh`] when `divider` is outside 1–256.
                pub fn set_clock_source(
                    &self,
                    clkrst: &HP_SYS_CLKRST,
                    source: ClockSource,
                    divider: u32,
                ) -> Result<(), Error> {
                    if !(1..=256).contains(&divider) {
                        return Err(Error::FrequencyTooHigh);
                    }
                    clkrst.peri_clk_ctrl20.[<write_ $src_sel>](source.field());
                    clkrst.peri_clk_ctrl20.[<write_ $div>](divider - 1);
                    clkrst.peri_clk_ctrl20.[<set_ $clk_en>]();
                    Ok(())
                }

                fn timer_period(&self, timer: TimerIndex) -> &dyn TimerPeriod {
                    match timer {
                        TimerIndex::Zero => &self.mcpwm.timer0_cfg0,
                        TimerIndex::One => &self.mcpwm.timer1_cfg0,
                        TimerIndex::Two => &self.mcpwm.timer2_cfg0,
                    }
                }

                fn timer_control(&self, timer: TimerIndex) -> &dyn TimerControl {
                    match timer {
                        TimerIndex::Zero => &self.mcpwm.timer0_cfg1,
                        TimerIndex::One => &self.mcpwm.timer1_cfg1,
                        TimerIndex::Two => &self.mcpwm.timer2_cfg1,
                    }
                }

                fn timer_value(&self, timer: TimerIndex) -> &dyn TimerValue {
                    match timer {
                        TimerIndex::Zero => &self.mcpwm.timer0_status,
                        TimerIndex::One => &self.mcpwm.timer1_status,
                        TimerIndex::Two => &self.mcpwm.timer2_status,
                    }
                }

                /// Configures a timer from its frequency.
                ///
                /// The timer's period is `clock / (frequency * prescale) - 1`,
                /// with the smallest prescaler that keeps the period inside 16
                /// bits.
                ///
                /// # Errors
                ///
                /// [`Error::FrequencyTooHigh`] when the frequency is at or above
                /// the clock, [`Error::FrequencyTooLow`] when the prescaler
                /// would have to exceed 256.
                pub fn configure_timer(
                    &self,
                    timer: TimerIndex,
                    config: TimerConfig,
                ) -> Result<(), Error> {
                    let clock = config.clock.hz() as u64;
                    let frequency = config.frequency as u64;
                    if frequency == 0 || frequency >= clock {
                        return Err(Error::FrequencyTooHigh);
                    }
                    let mut prescale = 1u64;
                    loop {
                        let ticks = clock / (frequency * prescale);
                        if (2..=65_536).contains(&ticks) {
                            self.timer_period(timer)
                                .write_period(prescale as u32 - 1, ticks as u32 - 1);
                            self.timer_control(timer).write_mode(config.mode);
                            return Ok(());
                        }
                        prescale += 1;
                        if prescale > 256 {
                            return Err(Error::FrequencyTooLow);
                        }
                    }
                }

                /// Runs the timer.
                pub fn start(&self, timer: TimerIndex) {
                    self.timer_control(timer).write_start(true);
                }

                /// Stops the timer, freezing its counter.
                pub fn stop(&self, timer: TimerIndex) {
                    self.timer_control(timer).write_start(false);
                }

                /// The timer's counter value.
                pub fn counter(&self, timer: TimerIndex) -> u16 {
                    self.timer_value(timer).read_value() as u16
                }

                /// The timer's period in ticks, i.e. the value written to
                /// `TIMERn_PERIOD`.
                pub fn period(&self, timer: TimerIndex) -> u16 {
                    let period = match timer {
                        TimerIndex::Zero => self.mcpwm.timer0_cfg0.timer_period(),
                        TimerIndex::One => self.mcpwm.timer1_cfg0.timer_period(),
                        TimerIndex::Two => self.mcpwm.timer2_cfg0.timer_period(),
                    };
                    period as u16
                }

                /// Applies `config` to an operator: timer selection, both
                /// comparators, the output actions and the dead time.
                ///
                /// # Errors
                ///
                /// [`Error::ComparatorOutOfRange`] when a comparator is above the
                /// timer's period.
                pub fn configure_generator(
                    &self,
                    operator: OperatorIndex,
                    config: GeneratorConfig,
                ) -> Result<(), Error> {
                    let period = self.period(config.timer);
                    if config.comparator_a > period || config.comparator_b > period {
                        return Err(Error::ComparatorOutOfRange);
                    }

                    match operator {
                        OperatorIndex::Zero => self
                            .mcpwm
                            .operator_timersel
                            .write_operator0_timersel(config.timer.field()),
                        OperatorIndex::One => self
                            .mcpwm
                            .operator_timersel
                            .write_operator1_timersel(config.timer.field()),
                        OperatorIndex::Two => self
                            .mcpwm
                            .operator_timersel
                            .write_operator2_timersel(config.timer.field()),
                    }

                    match operator {
                        OperatorIndex::Zero => {
                            self.mcpwm.gen0_stmp_cfg.write_update();
                            self.mcpwm
                                .gen0_tstmp_a
                                .write_value(u32::from(config.comparator_a));
                            self.mcpwm
                                .gen0_tstmp_b
                                .write_value(u32::from(config.comparator_b));
                            write_actions(&self.mcpwm.gen0_a, &config.actions_a);
                            write_actions(&self.mcpwm.gen0_b, &config.actions_b);
                            apply_dead_time(
                                &self.mcpwm.dt0_cfg,
                                &self.mcpwm.dt0_fed_cfg,
                                &self.mcpwm.dt0_red_cfg,
                                config.dead_time,
                            );
                        }
                        OperatorIndex::One => {
                            self.mcpwm.gen1_stmp_cfg.write_update();
                            self.mcpwm
                                .gen1_tstmp_a
                                .write_value(u32::from(config.comparator_a));
                            self.mcpwm
                                .gen1_tstmp_b
                                .write_value(u32::from(config.comparator_b));
                            write_actions(&self.mcpwm.gen1_a, &config.actions_a);
                            write_actions(&self.mcpwm.gen1_b, &config.actions_b);
                            apply_dead_time(
                                &self.mcpwm.dt1_cfg,
                                &self.mcpwm.dt1_fed_cfg,
                                &self.mcpwm.dt1_red_cfg,
                                config.dead_time,
                            );
                        }
                        OperatorIndex::Two => {
                            self.mcpwm.gen2_stmp_cfg.write_update();
                            self.mcpwm
                                .gen2_tstmp_a
                                .write_value(u32::from(config.comparator_a));
                            self.mcpwm
                                .gen2_tstmp_b
                                .write_value(u32::from(config.comparator_b));
                            write_actions(&self.mcpwm.gen2_a, &config.actions_a);
                            write_actions(&self.mcpwm.gen2_b, &config.actions_b);
                            apply_dead_time(
                                &self.mcpwm.dt2_cfg,
                                &self.mcpwm.dt2_fed_cfg,
                                &self.mcpwm.dt2_red_cfg,
                                config.dead_time,
                            );
                        }
                    }
                    Ok(())
                }

                /// Updates both comparators without touching anything else.
                pub fn set_comparators(&self, operator: OperatorIndex, a: u16, b: u16) {
                    match operator {
                        OperatorIndex::Zero => {
                            self.mcpwm.gen0_stmp_cfg.write_update();
                            self.mcpwm.gen0_tstmp_a.write_value(u32::from(a));
                            self.mcpwm.gen0_tstmp_b.write_value(u32::from(b));
                        }
                        OperatorIndex::One => {
                            self.mcpwm.gen1_stmp_cfg.write_update();
                            self.mcpwm.gen1_tstmp_a.write_value(u32::from(a));
                            self.mcpwm.gen1_tstmp_b.write_value(u32::from(b));
                        }
                        OperatorIndex::Two => {
                            self.mcpwm.gen2_stmp_cfg.write_update();
                            self.mcpwm.gen2_tstmp_a.write_value(u32::from(a));
                            self.mcpwm.gen2_tstmp_b.write_value(u32::from(b));
                        }
                    }
                }
            }
        }
    };
}

fn apply_dead_time(
    config: &dyn DeadTimeRegs,
    falling: &dyn DeadTimeValue,
    rising: &dyn DeadTimeValue,
    dead_time: Option<DeadTime>,
) {
    match dead_time {
        Some(dead_time) => {
            config.write_dead_time(dead_time);
            // FED delays the falling edge, RED the rising one.
            falling.write_delay(u32::from(dead_time.rising));
            rising.write_delay(u32::from(dead_time.falling));
            config.invert(dead_time.invert_a, dead_time.invert_b);
        }
        None => config.bypass(),
    }
}

impl_mcpwm!(
    Mcpwm0,
    MCPWM0,
    mcpwm0,
    0,
    reg_mcpwm0_apb_clk_en,
    reg_mcpwm0_clk_en,
    reg_mcpwm0_clk_src_sel,
    reg_mcpwm0_clk_div_num,
    reg_rst_en_pwm0
);
impl_mcpwm!(
    Mcpwm1,
    MCPWM1,
    mcpwm1,
    1,
    reg_mcpwm1_apb_clk_en,
    reg_mcpwm1_clk_en,
    reg_mcpwm1_clk_src_sel,
    reg_mcpwm1_clk_div_num,
    reg_rst_en_pwm1
);
