//! Motor control PWM (MCPWM).
//!
//! Each of the two controllers (MCPWM0 and MCPWM1) has three timers, three
//! operators and three capture channels. An operator drives a `PWMxA` /
//! `PWMxB` output pair through a dead-time generator, and the outputs are
//! GPIO-matrix signals (`PWM0_CH0_A_PAD_OUT` and friends), so any GPIO can
//! carry them.
//!
//! # Handles
//!
//! The PAC gives every timer, operator, capture channel and fault input its own
//! register type (`Timer0Cfg0`, `Timer1Cfg0`, ...) with identical field names.
//! Instead of dispatching on an index at run time, this module hands out a
//! typed handle per instance:
//!
//! - [`mc0::Timer<'_, N>`], [`mc0::Operator<'_, N>`], [`mc0::Capture<'_, N>`],
//!   [`mc0::Fault<'_, N>`] and [`mc0::CaptureTimer`] for MCPWM0;
//!   [`mc1::Timer<'_, N>`] and friends for MCPWM1
//! - [`Mcpwm0::timer`] / [`Mcpwm0::operator`] / [`Mcpwm0::capture`] /
//!   [`Mcpwm0::fault`] / [`Mcpwm0::capture_timer`] build them, with `N`
//!   range-checked at compile time
//!
//! Each handle's inherent implementation is generated once per instance by a
//! macro, so the register access lives in a single place per family while the
//! compiler picks the right registers at the call site. The result is the shape
//! RP2350's `pwm` module uses: no trait, no `dyn`, no run-time dispatch, and the
//! instance identity carried by the type. Every remaining `match` in the driver
//! decodes a *value* (a counting mode, a trip shape, an event's bit offset),
//! never an instance.
//!
//! The one thing that is *not* an instance is an event or a task: `EVT_EN`,
//! `TASK_EN`, the `INT_*` registers and the `SOC_ETM` ids form one contiguous
//! bit table per controller, so a signal is an offset. The handles hand those
//! out too — [`mc0::Timer::zero_event`], [`mc0::Capture::capture_task`],
//! [`mc0::Fault::mask`] and friends — so no API asks the caller to name an
//! instance by number.
//!
//! # Features
//!
//! - Timers: `XTAL_40M` / `RC_FAST` / `PLL_F160M` clock with a group divider,
//!   per-timer prescaler, up / down / up-down counting, start and stop, and
//!   counter readback
//! - Operators: comparator A/B, per-event output actions (hold, low, high,
//!   toggle) on timer zero, timer period and both comparators
//! - Dead time: separate rising and falling delays, counted in timer ticks
//! - Forced levels: hold an output high or low until it is released
//! - Sync: an input mux (another timer's sync output or a GPIO), a phase the
//!   counter reloads to, and a sync output other timers can ride on
//! - Fault detection: three GPIO fault inputs (one handle each), cycle-by-cycle
//!   and one-shot
//!   trips with per-output actions, software trips and a latched-trip status
//! - Capture: a free-running 32-bit timer and three channels with edge
//!   selection, input inversion and a 1–256 prescaler
//! - Carrier: pulse modulation of the outputs for transformer-isolated gates
//! - ETM: every event and task of the controller can be wired to the matrix
//!   ([`crate::etm`]) after being enabled here
//! - Event comparators: two extra counter values per operator that only feed
//!   the ETM, for doing something at a fixed point of the PWM period
//! - Interrupts: each [`Event`] has an interrupt bit that can be enabled,
//!   polled and acknowledged; [`Mcpwm0::SOURCE`] is the line to claim with
//!   [`crate::interrupt::claim`]
//! - Edge-aligned and complementary PWM built from those pieces
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_hal::gpio::signal::OutputSignal;
//! use esp32p4_hal::gpio::{PinOps, Pins};
//! use esp32p4_hal::mcpwm::{ClockSource, GeneratorConfig, Mcpwm0, TimerConfig};
//!
//! let mcpwm = Mcpwm0::init(dp.MCPWM0, &dp.HP_SYS_CLKRST);
//! mcpwm
//!     .set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::PllF160m, 1)
//!     .unwrap();
//!
//! let timer = mcpwm.timer::<0>();
//! timer
//!     .configure(&dp.HP_SYS_CLKRST, TimerConfig::new(20_000))
//!     .unwrap();
//!
//! let mut pin = pins.pin::<2>();
//! pin.route_output_signal(OutputSignal::PWM0_CH0_A_PAD_OUT);
//!
//! // 25 % on output A, 50 % on output B, both on the same timer.
//! mcpwm
//!     .operator::<0>()
//!     .configure(&timer, GeneratorConfig::edge_aligned(60, 30));
//! timer.start();
//! ```
//!
//! # Hardware Notes
//!
//! - The "timer reached zero" action lives in `UTEZ` / `DTEZ`. `UT0` / `DT0` are
//!   a different pair: they fire on `event_t0`, i.e. on a fault or on a sync
//!   input, neither of which this driver uses. Writing the zero action into
//!   `UT0` leaves the generator without any action that ever drives the output
//!   high, so the pad stays flat while the timer itself keeps counting.
//! - A PWM output only moves when the generator has an action for the event;
//!   leaving `TEZ`/`TEA` on hold produces a flat line, not a waveform.
//! - Every configuration register is written with the immediate update method
//!   (`0`), so no shadow register waits for a timer event before it takes
//!   effect. Changes therefore land in the period they are written in.
//! - The dead-time generator is bypassed per output out of reset; the driver
//!   only un-bypasses it when a [`DeadTime`] is configured, and then counts the
//!   delays in timer ticks (the TRM's `PT_clk`).
//! - A cycle-by-cycle trip is released at the event [`CbcRefresh`] selects, so
//!   the timer has to be running for it to recover; a one-shot trip stays
//!   latched until [`mc0::Operator::release_trip`]. Both are only cleared on a
//!   rising edge of their software trigger, which is why the driver drives the
//!   trigger low and then high instead of just flipping it.
//! - One carrier period is `8 * (prescale + 1)` `PWM_clk` ticks: the factor of
//!   eight comes from the three-bit duty field.
//! - An event only reaches the ETM matrix once [`Mcpwm0::enable_event`] set its
//!   `EVT_EN` bit, and a task only acts once [`Mcpwm0::enable_task`] set its
//!   `TASK_EN` bit; the matrix itself is [`crate::etm`].
//! - `EVT_EN`, `INT_ENA`, `INT_RAW`, `INT_ST` and `INT_CLR` share one bit layout,
//!   so the same [`Event`] indexes all of them. `INT_CLR` is
//!   write-1-to-clear and `INT_ST` only reports sources whose enable bit is
//!   set — an interrupt that is not enabled stays invisible there.

use esp32p4_pac::{HP_SYS_CLKRST, Interrupt, MCPWM0, MCPWM1, mcpwm0, mcpwm1};

use crate::gpio::Level;

// ============================================================================
// Value types
// ============================================================================

/// One of the two outputs of an operator.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OperatorOutput {
    /// `PWMxA`, the one driven by the A actions (`GENn_A`).
    A,
    /// `PWMxB`, the one driven by the B actions (`GENn_B`).
    B,
}

/// Clock feeding the group divider.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockSource {
    /// `XTAL_CLK`, 40 MHz. The reset value of the selector.
    #[default]
    Xtal40m,
    /// `RC_FAST_CLK`, whose frequency is only accurate to a few percent.
    RcFast,
    /// `PLL_F160M_CLK`, 160 MHz.
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

    const fn from_field(field: u32) -> Option<Self> {
        match field {
            0 => Some(Self::Xtal40m),
            1 => Some(Self::RcFast),
            2 => Some(Self::PllF160m),
            _ => None,
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

/// Direction the counter takes after a sync event reloaded its phase.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CountDirection {
    /// Count up.
    Up,
    /// Count down.
    Down,
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
    /// Counting mode.
    pub mode: CountMode,
}

impl TimerConfig {
    /// A timer counting up at `frequency`.
    pub const fn new(frequency: u32) -> Self {
        Self {
            frequency,
            mode: CountMode::Up,
        }
    }

    /// Same as [`TimerConfig::new`], with an explicit counting mode.
    pub const fn with_mode(frequency: u32, mode: CountMode) -> Self {
        Self { frequency, mode }
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
///
/// The dead-time generator builds the pair out of output A: output A is A with
/// its rising edge held back by [`rising`](Self::rising) ticks, output B is its
/// inverse with its own rising edge held back by [`falling`](Self::falling)
/// ticks. The two sides of a half bridge therefore never turn on together, and
/// each edge can get the delay its gate driver needs.
///
/// Output B is produced inside the dead-time generator, so a generator's B
/// actions and comparator B are not used while a dead time is configured:
/// output A alone defines the duty cycle. Both delays are counted in timer
/// ticks, exactly like the comparators — and, like any counter, they only move
/// while the timer runs. With the timer stopped a level change that has to cross
/// a delayed edge never reaches the pad, so combine a stopped timer with
/// [`mc0::Operator::force`] only after bypassing the dead time.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DeadTime {
    /// Rising-edge delay (RED), in timer ticks: how long output A waits before
    /// going high.
    pub rising: u16,
    /// Falling-edge delay (FED), in timer ticks: how long output B waits before
    /// going high, i.e. the gap after output A falls.
    pub falling: u16,
    /// `true` for the active-high pair (A high while B low), `false` for the
    /// active-low pair that inverted gate drivers need.
    pub active_high: bool,
}

impl DeadTime {
    /// Active-high complementary pair (TRM mode 2), the usual half-bridge
    /// drive.
    pub const fn active_high(rising: u16, falling: u16) -> Self {
        Self {
            rising,
            falling,
            active_high: true,
        }
    }

    /// Active-low complementary pair (TRM mode 3), for gate drivers with
    /// inverting inputs.
    pub const fn active_low(rising: u16, falling: u16) -> Self {
        Self {
            rising,
            falling,
            active_high: false,
        }
    }
}

/// Operator setup.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GeneratorConfig {
    /// Value of comparator A, in timer ticks.
    pub comparator_a: u16,
    /// Value of comparator B, in timer ticks.
    pub comparator_b: u16,
    /// Actions for output A.
    pub actions_a: OutputActions,
    /// Actions for output B.
    pub actions_b: OutputActions,
    /// Dead time, or `None` to pass both outputs through unchanged.
    ///
    /// When it is set, the dead-time generator derives output B from output A,
    /// so `comparator_b` and `actions_b` no longer reach the pad.
    pub dead_time: Option<DeadTime>,
}

impl GeneratorConfig {
    /// Two independent edge-aligned outputs sharing one timer.
    ///
    /// Output A rises at the start of the period and falls at `duty_a`, output
    /// B does the same at `duty_b`; both are in timer ticks and the period is
    /// `timer value + 1`.
    pub const fn edge_aligned(duty_a: u16, duty_b: u16) -> Self {
        Self {
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

    /// A complementary pair: output A is high for `duty` ticks and output B is
    /// its inverse, with `dead_time` keeping the transitions apart.
    ///
    /// Output A defines the duty; output B is derived from it by the dead-time
    /// generator, so the B actions only mirror A's for readability.
    pub const fn complementary(duty: u16, dead_time: DeadTime) -> Self {
        let actions = OutputActions {
            zero: PwmAction::High,
            comparator_a: PwmAction::Low,
            ..OutputActions::new()
        };
        Self {
            comparator_a: duty,
            comparator_b: 0,
            actions_a: actions,
            actions_b: actions,
            dead_time: Some(dead_time),
        }
    }
}

// ============================================================================
// Sync
// ============================================================================

/// Where a timer takes its sync input from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncSource {
    /// The sync output of timer 0.
    Timer0,
    /// The sync output of timer 1.
    Timer1,
    /// The sync output of timer 2.
    Timer2,
    /// `SYNC0` from the GPIO matrix (`PWM0_SYNC0_PAD_IN`).
    Gpio0,
    /// `SYNC1` from the GPIO matrix.
    Gpio1,
    /// `SYNC2` from the GPIO matrix.
    Gpio2,
}

impl SyncSource {
    const fn field(self) -> u32 {
        match self {
            Self::Timer0 => 1,
            Self::Timer1 => 2,
            Self::Timer2 => 3,
            Self::Gpio0 => 4,
            Self::Gpio1 => 5,
            Self::Gpio2 => 6,
        }
    }
}

/// What a timer drives its sync output with.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncOutSource {
    /// Forward the sync input, so one source can fan out to several timers.
    Input,
    /// A pulse at every zero event.
    Zero,
    /// A pulse at every period event.
    Period,
    /// Nothing: the output stays low.
    Off,
}

impl SyncOutSource {
    const fn field(self) -> u32 {
        match self {
            Self::Input => 0,
            Self::Zero => 1,
            Self::Period => 2,
            Self::Off => 3,
        }
    }
}

// ============================================================================
// Fault detection
// ============================================================================

/// A set of fault inputs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FaultMask(u8);

impl FaultMask {
    /// No input at all.
    pub const NONE: Self = Self(0);
    /// Every input.
    pub const ALL: Self = Self(0b111);

    /// Adds the input at `index`.
    pub const fn with_index(self, index: u8) -> Self {
        Self(self.0 | (1 << index))
    }

    /// Whether the input at `index` is in the set.
    const fn contains_index(self, index: u8) -> bool {
        self.0 & (1 << index) != 0
    }
}

/// How long a trip holds the outputs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TripMode {
    /// Released again at the next refresh event; the shape to use for current
    /// limiting.
    CycleByCycle,
    /// Held until [`mc0::Operator::release_trip`] clears it; the shape to use for
    /// over-current shutdown.
    OneShot,
}

/// When a cycle-by-cycle trip is refreshed, i.e. released again.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CbcRefresh {
    /// Never; a fault that stays asserted keeps the trip on.
    Never,
    /// At every zero event.
    #[default]
    Zero,
    /// At every period event.
    Period,
    /// At both zero and period events.
    Both,
}

impl CbcRefresh {
    const fn field(self) -> u32 {
        match self {
            Self::Never => 0,
            Self::Zero => 1,
            Self::Period => 2,
            Self::Both => 3,
        }
    }
}

/// What a fault does to the outputs of one operator.
///
/// Call [`mc0::Operator::configure_trip`] once per mode: the cycle-by-cycle and the
/// one-shot setup keep their own actions and sources, so one operator can limit
/// current on a fault while another one shuts down hard.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TripConfig {
    /// Which shape the trip takes.
    pub mode: TripMode,
    /// Fault inputs that trigger it.
    pub sources: FaultMask,
    /// Also let [`mc0::Operator::software_trip`] trigger it.
    pub software: bool,
    /// Action applied to output A while the trip is active.
    pub output_a: PwmAction,
    /// Action applied to output B while the trip is active.
    pub output_b: PwmAction,
    /// Refresh point of a cycle-by-cycle trip; ignored for one-shot trips.
    pub refresh: CbcRefresh,
}

/// Which trips are active on an operator.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TripStatus {
    /// A cycle-by-cycle trip is tripping right now.
    pub cycle_by_cycle: bool,
    /// A one-shot trip is latched.
    pub one_shot: bool,
}

// ============================================================================
// Capture
// ============================================================================

/// Which edges of the capture input trigger a capture.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CaptureEdge {
    /// Rising edges only.
    Rising,
    /// Falling edges only.
    Falling,
    /// Both edges.
    Both,
}

impl CaptureEdge {
    const fn field(self) -> u32 {
        match self {
            Self::Rising => 0b10,
            Self::Falling => 0b01,
            Self::Both => 0b11,
        }
    }
}

/// Capture channel setup.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CaptureConfig {
    /// Edges that trigger a capture.
    pub edge: CaptureEdge,
    /// Input prescaler, 1–256: one capture every `prescale` edges.
    pub prescale: u16,
    /// Invert the input before the prescaler.
    pub invert: bool,
}

// ============================================================================
// Carrier
// ============================================================================

/// Carrier (chopper) setup for one operator.
///
/// The carrier replaces a High level of the PWM output with a burst of pulses,
/// which is what lets a gate be driven through a transformer: the transformer
/// only passes the AC part. One carrier period is `8 * (prescale + 1)` `PWM_clk`
/// ticks — the factor of eight is fixed, the field only scales it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CarrierConfig {
    /// Carrier clock divider, 1–16: the carrier period is
    /// `8 * (prescale + 1)` `PWM_clk` ticks.
    pub prescale: u16,
    /// Carrier duty cycle in eighths of a carrier period, 0–7.
    pub duty: u8,
    /// Width of the first pulse of each burst, in carrier periods, 1–16.
    pub first_pulse: u8,
    /// Invert the signal before the carrier is applied.
    pub invert_input: bool,
    /// Invert the signal after the carrier is applied.
    pub invert_output: bool,
}

// ============================================================================
// ETM
// ============================================================================

/// One event of a controller, as its offset in the event bit table.
///
/// An event is not an instance: `EVT_EN`, the `INT_*` registers and the
/// `SOC_ETM` event ids are one contiguous table per controller, so a signal is
/// identified by its offset. The handles build these values
/// ([`mc0::Timer::zero_event`](mc0::Timer) and friends), which keeps the
/// instance identity in the handle's type instead of in a number written at the
/// call site.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Event(u8);

impl Event {
    /// The offset in the event bit table, which is also the offset from the
    /// controller's first ETM event id.
    pub const fn raw(self) -> u8 {
        self.0
    }

    const fn at(offset: u8) -> Self {
        Self(offset)
    }
}

/// One task of a controller, as its offset in the task bit table.
///
/// Built by the handles, like [`Event`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Task(u8);

impl Task {
    /// The offset in the task bit table, which is also the offset from the
    /// controller's first ETM task id.
    pub const fn raw(self) -> u8 {
        self.0
    }

    const fn at(offset: u8) -> Self {
        Self(offset)
    }
}

/// One of the two extra event comparators of an operator
/// (`OPn_TSTMP_E1` / `OPn_TSTMP_E2`).
///
/// Unlike the A/B comparators these do not touch the outputs: they only feed
/// the ETM, so they can start a conversion, a capture or another peripheral at
/// a fixed point of the PWM period.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EventComparator {
    /// `OPn_TSTMP_E1`.
    E1,
    /// `OPn_TSTMP_E2`.
    E2,
}

impl EventComparator {
    /// Offsets the comparator's three `EVT_EN2` bits and its three ETM ids.
    const fn index(self) -> u32 {
        match self {
            Self::E1 => 0,
            Self::E2 => 1,
        }
    }
}

/// Errors from the configuration setters.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The frequency is zero or at or above the group clock.
    FrequencyTooHigh,
    /// The frequency needs a prescaler beyond 256.
    FrequencyTooLow,
    /// The group clock divider is outside 1–256.
    DividerOutOfRange,
    /// The group clock selector reads back as a source this driver does not
    /// know the frequency of.
    UnknownClockSource,
    /// A prescaler, duty cycle or pulse width does not fit its register.
    OutOfRange,
}

// ============================================================================
// Per-signal bit tables
// ============================================================================
//
// `EVT_EN`, `TASK_EN`, `EVT_EN2` and the `INT_*` registers hold one bit per
// signal rather than bitfields, and both controllers share the layout, so they
// need neither a PAC accessor per instance nor a per-instance method: an offset
// plus the bit index is enough.

const INT_ENA_OFFSET: u32 = 0x0110;
const INT_ST_OFFSET: u32 = 0x0118;
const INT_CLR_OFFSET: u32 = 0x011c;
const EVT_EN_OFFSET: u32 = 0x0120;
const TASK_EN_OFFSET: u32 = 0x0124;
const EVT_EN2_OFFSET: u32 = 0x0128;

fn set_indexed_bit(address: u32, bit: u32, set: bool) {
    let register = address as *mut u32;
    let mask = 1 << bit;
    unsafe {
        let bits = core::ptr::read_volatile(register);
        core::ptr::write_volatile(register, if set { bits | mask } else { bits & !mask });
    }
}

fn read_indexed_bit(address: u32, bit: u32) -> bool {
    unsafe { core::ptr::read_volatile(address as *const u32) & (1 << bit) != 0 }
}

fn write_indexed_bit(address: u32, bit: u32) {
    unsafe { core::ptr::write_volatile(address as *mut u32, 1 << bit) }
}

/// Writes all eight action fields of one generator register.
macro_rules! write_generator_actions {
    ($generator:expr, $actions:expr) => {{
        let generator = &$generator;
        let actions = &$actions;
        // The zero event is UTEZ/DTEZ. UT0/DT0 look like the right fields but
        // belong to event_t0 (fault / sync).
        generator.write_utez(actions.zero.field());
        generator.write_dtez(actions.zero.field());
        generator.write_utep(actions.period.field());
        generator.write_dtep(actions.period.field());
        generator.write_utea(actions.comparator_a.field());
        generator.write_dtea(actions.comparator_a.field());
        generator.write_uteb(actions.comparator_b.field());
        generator.write_dteb(actions.comparator_b.field());
    }};
}

// ============================================================================
// Per-instance implementations
// ============================================================================
//
// One body per family, instantiated once per hardware instance with that
// instance's register names substituted. The compiler therefore resolves every
// register access at the call site; nothing is dispatched at run time and no
// adapter type is needed.

macro_rules! timer_body {
    ($n:literal, $cfg0:ident, $cfg1:ident, $status:ident, $sync:ident) => {
        pastey::paste! {
            impl<'a> Timer<'a, $n> {
                /// Programs the timer from a frequency.
                ///
                /// `frequency` is the PWM frequency in every counting mode: up
                /// and down counting span `PWM_clk / (frequency * prescale)`
                /// ticks, the up-down walk spans twice the period register. The
                /// smallest prescaler that keeps the period register inside 16
                /// bits is used. `PWM_clk` is whatever
                /// [`set_clock_source`](Mcpwm0::set_clock_source) last
                /// produced, read back from the clock registers.
                ///
                /// Like the TRM says, a new prescaler only takes effect once
                /// the timer is (re)started, so configure a running timer's
                /// period with [`stop`](Self::stop) / [`start`](Self::start)
                /// around it.
                ///
                /// # Errors
                ///
                /// [`Error::FrequencyTooHigh`] when the frequency is zero or at
                /// or above `PWM_clk`, [`Error::FrequencyTooLow`] when the
                /// prescaler would have to exceed 256.
                pub fn configure(
                    &self,
                    clkrst: &HP_SYS_CLKRST,
                    config: TimerConfig,
                ) -> Result<(), Error> {
                    let clock = u64::from(self.0.group_clock_hz(clkrst)?);
                    let frequency = u64::from(config.frequency);
                    if frequency == 0 || frequency >= clock {
                        return Err(Error::FrequencyTooHigh);
                    }
                    let mut prescale = 1u64;
                    loop {
                        let ticks = clock / (frequency * prescale);
                        // Up / down counting spends `period + 1` ticks per PWM
                        // period; the up-down walk takes `2 * period`, which is
                        // why the TRM writes that mode's period without the -1.
                        let period = match config.mode {
                            CountMode::UpDown => ticks / 2,
                            CountMode::Up | CountMode::Down => ticks.saturating_sub(1),
                        };
                        if (1..=65_535).contains(&period) {
                            let cfg0 = &self.0.regs().$cfg0;
                            // The prescaler field holds `prescale - 1`.
                            cfg0.write_timer_prescale(prescale as u32 - 1);
                            // Update immediately: while the timer is still
                            // stopped, an update gated on TEZ would leave the
                            // active period at its reset value.
                            cfg0.write_timer_period_upmethod(0);
                            cfg0.write_timer_period(period as u32);
                            self.0.regs().$cfg1.write_timer_mod(config.mode.field());
                            return Ok(());
                        }
                        prescale += 1;
                        if prescale > 256 {
                            return Err(Error::FrequencyTooLow);
                        }
                    }
                }

                /// Runs the timer.
                ///
                /// `TIMERn_START` is a command, not a run flag: `2` means
                /// "start and keep running", while `0` and `1` ask the timer to
                /// stop at the next zero or period event.
                pub fn start(&self) {
                    self.0.regs().$cfg1.write_timer_start(2);
                }

                /// Asks the timer to stop at its next zero event, i.e. at the
                /// end of the current period.
                pub fn stop(&self) {
                    self.0.regs().$cfg1.write_timer_start(0);
                }

                /// The counter value.
                pub fn counter(&self) -> u16 {
                    self.0.regs().$status.timer_value() as u16
                }

                /// The period in ticks, i.e. the value written to
                /// `TIMERn_PERIOD`.
                pub fn period(&self) -> u16 {
                    self.0.regs().$cfg0.timer_period() as u16
                }

                /// Points the timer at its sync source; `None` disconnects the
                /// input (software sync still works).
                pub fn sync_input(&self, source: Option<SyncSource>) {
                    let field = source.map_or(0, SyncSource::field);
                    self.0.regs().timer_synci_cfg.[<write_timer $n _syncisel>](field);
                }

                /// Lets a sync event reload the counter from the phase register.
                pub fn enable_sync_input(&self, enable: bool) {
                    let sync = &self.0.regs().$sync;
                    if enable {
                        sync.set_timer_synci_en();
                    } else {
                        sync.clr_timer_synci_en();
                    }
                }

                /// Selects what the timer's own sync output carries.
                pub fn sync_output(&self, source: SyncOutSource) {
                    self.0.regs().$sync.write_timer_synco_sel(source.field());
                }

                /// Sets the counter value and direction a sync event reloads.
                ///
                /// In up-down counting the direction matters: a phase of 0 can
                /// only count up and a phase of the period can only count down.
                pub fn set_sync_phase(&self, phase: u16, direction: CountDirection) {
                    let sync = &self.0.regs().$sync;
                    sync.write_timer_phase(u32::from(phase));
                    match direction {
                        CountDirection::Up => sync.clr_timer_phase_direction(),
                        CountDirection::Down => sync.set_timer_phase_direction(),
                    }
                }

                /// Generates one software sync event.
                pub fn software_sync(&self) {
                    // SW generates one event per edge; drive a rising edge so
                    // every call fires exactly once.
                    let sync = &self.0.regs().$sync;
                    sync.clr_sw();
                    sync.set_sw();
                }

                /// The event raised when this timer stops counting.
                pub const fn stop_event(&self) -> Event {
                    Event::at($n)
                }

                /// The event raised when this timer reaches zero.
                pub const fn zero_event(&self) -> Event {
                    Event::at(3 + $n)
                }

                /// The event raised when this timer reaches its period.
                pub const fn period_event(&self) -> Event {
                    Event::at(6 + $n)
                }

                /// The task that runs a sync event on this timer.
                pub const fn sync_task(&self) -> Task {
                    Task::at(7 + $n)
                }

                /// The task that reloads this timer's period register.
                pub const fn update_period_task(&self) -> Task {
                    Task::at(10 + $n)
                }
            }
        }
    };
}

macro_rules! operator_body {
    ($n:literal, $gen_cfg0:ident, $stmp_cfg:ident, $tstmp_a:ident, $tstmp_b:ident,
     $gen_force:ident, $gen_a:ident, $gen_b:ident, $dt_cfg:ident, $dt_fed:ident, $dt_red:ident,
     $carrier:ident, $fh_cfg0:ident, $fh_cfg1:ident, $fh_status:ident, $e1:ident, $e2:ident) => {
        pastey::paste! {
            impl<'a> Operator<'a, $n> {
                /// Applies `config` to the operator and binds it to `timer`:
                /// timer selection, both comparators, the output actions and the
                /// dead time.
                ///
                /// The handle is what carries the timer number into the
                /// `OPERATORn_TIMERSEL` field, so an operator cannot be bound to
                /// a timer that was not written at the call site. It is read as a
                /// type, not as a value: `T` lands in the register as a constant.
                ///
                /// Comparators and dead-time delays are counted in that timer's
                /// ticks, so they must fit inside its period — a comparator above
                /// the period simply never matches, which leaves the output flat.
                /// The timer that is handed in is therefore the one whose period
                /// the values are meant against.
                pub fn configure<const T: u8>(
                    &self,
                    _timer: &Timer<'_, T>,
                    config: GeneratorConfig,
                ) {
                    let regs = self.0.regs();
                    regs.operator_timersel.[<write_operator $n _timersel>](u32::from(T));

                    // GEN_CFG_UPMETHOD gates the action fields of GENn_A/GENn_B;
                    // 0 applies them immediately.
                    regs.$gen_cfg0.write_gen_cfg_upmethod(0);

                    // Leaving a forced level behind would hide the generator's
                    // own actions.
                    let force = &regs.$gen_force;
                    force.write_gen_cntuforce_upmethod(0);
                    force.write_gen_a_cntuforce_mode(0);
                    force.write_gen_b_cntuforce_mode(0);
                    force.write_gen_a_nciforce_mode(0);
                    force.write_gen_b_nciforce_mode(0);

                    let stmp = &regs.$stmp_cfg;
                    stmp.write_cmpr_a_upmethod(0);
                    stmp.write_cmpr_b_upmethod(0);
                    regs.$tstmp_a.write_cmpr_a(u32::from(config.comparator_a));
                    regs.$tstmp_b.write_cmpr_b(u32::from(config.comparator_b));

                    write_generator_actions!(regs.$gen_a, config.actions_a);
                    write_generator_actions!(regs.$gen_b, config.actions_b);

                    // Dead time: bypassed when there is none, otherwise counted
                    // in timer ticks (`PT_clk`) with both delays applied.
                    let dt = &regs.$dt_cfg;
                    match config.dead_time {
                        Some(dead_time) => {
                            dt.write_db_fed_upmethod(0);
                            dt.write_db_red_upmethod(0);
                            dt.set_db_clk_sel();
                            // S0/S1: run both outputs through the generator.
                            dt.clr_db_a_outbypass();
                            dt.clr_db_b_outbypass();
                            // S4/S5: both delay blocks read output A, S8: one
                            // delay block per path.
                            dt.clr_db_red_insel();
                            dt.clr_db_fed_insel();
                            dt.clr_db_deb_mode();
                            // S2/S3 pick the pair's polarity.
                            if dead_time.active_high {
                                dt.clr_db_red_outinvert();
                                dt.set_db_fed_outinvert();
                            } else {
                                dt.set_db_red_outinvert();
                                dt.clr_db_fed_outinvert();
                            }
                            // RED delays the rising edge, FED the falling one.
                            regs.$dt_red.write_db_red(u32::from(dead_time.rising));
                            regs.$dt_fed.write_db_fed(u32::from(dead_time.falling));
                        }
                        None => {
                            dt.set_db_a_outbypass();
                            dt.set_db_b_outbypass();
                        }
                    }
                }

                /// Updates both comparators without touching anything else.
                pub fn set_comparators(&self, a: u16, b: u16) {
                    let regs = self.0.regs();
                    let stmp = &regs.$stmp_cfg;
                    stmp.write_cmpr_a_upmethod(0);
                    stmp.write_cmpr_b_upmethod(0);
                    regs.$tstmp_a.write_cmpr_a(u32::from(a));
                    regs.$tstmp_b.write_cmpr_b(u32::from(b));
                }

                /// Holds one output at a fixed level, ignoring the timer events,
                /// until [`release_force`](Self::release_force) is called.
                ///
                /// The force is applied through the generator's continuous
                /// software force, so it works whether or not the timer runs —
                /// which makes it the quickest way to prove that the pad path is
                /// wired up. It sits before the dead-time generator, so a
                /// configured [`DeadTime`] still delays the level, and inverts it
                /// on output B.
                pub fn force(&self, output: OperatorOutput, level: Level) {
                    self.set_force(output, Some(level));
                }

                /// Hands one output back to the generator's timer events.
                pub fn release_force(&self, output: OperatorOutput) {
                    self.set_force(output, None);
                }

                fn set_force(&self, output: OperatorOutput, level: Option<Level>) {
                    // An immediate update makes the force take effect at once;
                    // the mode selects low (1) or high (2), and 0 releases the
                    // output back to the generator.
                    let force = &self.0.regs().$gen_force;
                    force.write_gen_cntuforce_upmethod(0);
                    let mode = match level {
                        None => 0,
                        Some(Level::Low) => 1,
                        Some(Level::High) => 2,
                    };
                    match output {
                        OperatorOutput::A => force.write_gen_a_cntuforce_mode(mode),
                        OperatorOutput::B => force.write_gen_b_cntuforce_mode(mode),
                    }
                }

                /// Sets up one fault shape.
                ///
                /// The actions are written for both counting directions, and
                /// only the fields of `config.mode` are touched, so the two
                /// shapes can be configured independently.
                pub fn configure_trip(&self, config: TripConfig) {
                    let regs = self.0.regs();
                    let fh = &regs.$fh_cfg0;
                    let mask = config.sources;
                    let a = config.output_a.field();
                    let b = config.output_b.field();
                    match config.mode {
                        TripMode::CycleByCycle => {
                            fh.write_tz_a_cbc_u(a);
                            fh.write_tz_a_cbc_d(a);
                            fh.write_tz_b_cbc_u(b);
                            fh.write_tz_b_cbc_d(b);
                            if mask.contains_index(0) {
                                fh.set_tz_f0_cbc();
                            } else {
                                fh.clr_tz_f0_cbc();
                            }
                            if mask.contains_index(1) {
                                fh.set_tz_f1_cbc();
                            } else {
                                fh.clr_tz_f1_cbc();
                            }
                            if mask.contains_index(2) {
                                fh.set_tz_f2_cbc();
                            } else {
                                fh.clr_tz_f2_cbc();
                            }
                            if config.software {
                                fh.set_tz_sw_cbc();
                            } else {
                                fh.clr_tz_sw_cbc();
                            }
                            regs.$fh_cfg1.write_tz_cbcpulse(config.refresh.field());
                        }
                        TripMode::OneShot => {
                            fh.write_tz_a_ost_u(a);
                            fh.write_tz_a_ost_d(a);
                            fh.write_tz_b_ost_u(b);
                            fh.write_tz_b_ost_d(b);
                            if mask.contains_index(0) {
                                fh.set_tz_f0_ost();
                            } else {
                                fh.clr_tz_f0_ost();
                            }
                            if mask.contains_index(1) {
                                fh.set_tz_f1_ost();
                            } else {
                                fh.clr_tz_f1_ost();
                            }
                            if mask.contains_index(2) {
                                fh.set_tz_f2_ost();
                            } else {
                                fh.clr_tz_f2_ost();
                            }
                            if config.software {
                                fh.set_tz_sw_ost();
                            } else {
                                fh.clr_tz_sw_ost();
                            }
                        }
                    }
                }

                /// Which trips are active right now.
                pub fn trip_status(&self) -> TripStatus {
                    let status = &self.0.regs().$fh_status;
                    TripStatus {
                        cycle_by_cycle: status.tz_cbc_on(),
                        one_shot: status.tz_ost_on(),
                    }
                }

                /// Clears a latched one-shot trip, handing the outputs back to
                /// the generator.
                pub fn release_trip(&self) {
                    // CLR_OST triggers on an edge; drive a rising one every time
                    // so a second release is not lost.
                    let cfg1 = &self.0.regs().$fh_cfg1;
                    cfg1.clr_tz_clr_ost();
                    cfg1.set_tz_clr_ost();
                }

                /// Trips the operator from software. It only has an effect when
                /// the matching [`TripConfig`] set `software`.
                pub fn software_trip(&self, mode: TripMode) {
                    let cfg1 = &self.0.regs().$fh_cfg1;
                    match mode {
                        TripMode::CycleByCycle => {
                            cfg1.clr_tz_force_cbc();
                            cfg1.set_tz_force_cbc();
                        }
                        TripMode::OneShot => {
                            cfg1.clr_tz_force_ost();
                            cfg1.set_tz_force_ost();
                        }
                    }
                }

                /// Applies the carrier to the outputs and enables it.
                ///
                /// # Errors
                ///
                /// [`Error::OutOfRange`] when `prescale` or `first_pulse` is
                /// outside 1–16, or `duty` is above 7.
                pub fn configure_carrier(&self, config: CarrierConfig) -> Result<(), Error> {
                    if !(1..=16).contains(&config.prescale)
                        || config.duty > 7
                        || !(1..=16).contains(&config.first_pulse)
                    {
                        return Err(Error::OutOfRange);
                    }
                    let carrier = &self.0.regs().$carrier;
                    // The register holds `prescale - 1` and `first_pulse - 1`.
                    carrier.write_chopper_prescale(u32::from(config.prescale) - 1);
                    carrier.write_chopper_duty(u32::from(config.duty));
                    carrier.write_chopper_oshtwth(u32::from(config.first_pulse) - 1);
                    if config.invert_input {
                        carrier.set_chopper_in_invert();
                    } else {
                        carrier.clr_chopper_in_invert();
                    }
                    if config.invert_output {
                        carrier.set_chopper_out_invert();
                    } else {
                        carrier.clr_chopper_out_invert();
                    }
                    carrier.set_chopper_en();
                    Ok(())
                }

                /// Bypasses the carrier again.
                pub fn disable_carrier(&self) {
                    self.0.regs().$carrier.clr_chopper_en();
                }

                /// Sets the counter value at which one of the operator's extra
                /// event comparators fires.
                ///
                /// The comparator only produces an ETM event: enable it with
                /// [`enable_event_comparator`](Self::enable_event_comparator)
                /// and read its id from
                /// [`event_comparator_id`](Self::event_comparator_id).
                pub fn set_event_comparator(&self, comparator: EventComparator, value: u16) {
                    match comparator {
                        EventComparator::E1 => {
                            self.0.regs().$e1.write_op_tstmp_e1(u32::from(value))
                        }
                        EventComparator::E2 => {
                            self.0.regs().$e2.write_op_tstmp_e2(u32::from(value))
                        }
                    }
                }

                /// Exposes an event comparator to the ETM matrix.
                pub fn enable_event_comparator(&self, comparator: EventComparator, enable: bool) {
                    // `EVT_EN2` has three bits per comparator, one per operator.
                    let bit = comparator.index() * 3 + $n;
                    set_indexed_bit(self.0.base() + EVT_EN2_OFFSET, bit, enable)
                }

                /// The `SOC_ETM` id of an event comparator.
                pub fn event_comparator_id(
                    &self,
                    comparator: EventComparator,
                ) -> crate::etm::EventId {
                    crate::etm::EventId::new(
                        self.0.event_base() + 30 + (comparator.index() as u8) * 3 + $n as u8,
                    )
                }

                /// The event raised when the counter matches timestamp A.
                pub const fn timestamp_a_event(&self) -> Event {
                    Event::at(9 + $n)
                }

                /// The event raised when the counter matches timestamp B.
                pub const fn timestamp_b_event(&self) -> Event {
                    Event::at(12 + $n)
                }

                /// The event raised when this operator applies a cycle-by-cycle
                /// trip.
                pub const fn cycle_by_cycle_event(&self) -> Event {
                    Event::at(21 + $n)
                }

                /// The event raised when this operator applies a one-shot trip.
                pub const fn one_shot_event(&self) -> Event {
                    Event::at(24 + $n)
                }

                /// The task that reloads timestamp A's active register.
                pub const fn update_timestamp_a_task(&self) -> Task {
                    Task::at($n)
                }

                /// The task that reloads timestamp B's active register.
                pub const fn update_timestamp_b_task(&self) -> Task {
                    Task::at(3 + $n)
                }

                /// The task that applies a one-shot trip to this operator.
                pub const fn one_shot_task(&self) -> Task {
                    Task::at(13 + $n)
                }

                /// The task that clears a latched one-shot trip of this operator.
                pub const fn release_task(&self) -> Task {
                    Task::at(16 + $n)
                }
            }
        }
    };
}

macro_rules! fault_body {
    ($n:literal) => {
        pastey::paste! {
            impl<'a> Fault<'a, $n> {
                /// Enables the input and picks the level that trips.
                ///
                /// The GPIO matrix side is the caller's: route
                /// [`InputSignal::PWM0_F0_PAD_IN`](crate::gpio::signal::InputSignal::PWM0_F0_PAD_IN)
                /// and friends first.
                pub fn enable(&self, active_high: bool) {
                    let reg = &self.0.regs().fault_detect;
                    if active_high {
                        reg.[<set_f $n _pole>]();
                    } else {
                        reg.[<clr_f $n _pole>]();
                    }
                    reg.[<set_f $n _en>]();
                }

                /// Stops the input from generating events.
                pub fn disable(&self) {
                    self.0.regs().fault_detect.[<clr_f $n _en>]();
                }

                /// Whether the input is asserting right now.
                pub fn active(&self) -> bool {
                    self.0.regs().fault_detect.[<event_f $n>]()
                }

                /// The event raised when this input asserts.
                pub const fn event(&self) -> Event {
                    Event::at(15 + $n)
                }

                /// The event raised when this input is released again.
                pub const fn cleared_event(&self) -> Event {
                    Event::at(18 + $n)
                }

                /// A mask holding just this input, for [`TripConfig::sources`].
                pub const fn mask(&self) -> FaultMask {
                    FaultMask::NONE.with_index($n)
                }
            }
        }
    };
}

macro_rules! capture_body {
    ($n:literal, $cfg:ident, $value:ident) => {
        pastey::paste! {
            impl<'a> Capture<'a, $n> {
                /// Sets the edge, prescaler and inversion of the channel.
                ///
                /// # Errors
                ///
                /// [`Error::OutOfRange`] when `prescale` is outside 1–256.
                pub fn configure(&self, config: CaptureConfig) -> Result<(), Error> {
                    if !(1..=256).contains(&config.prescale) {
                        return Err(Error::OutOfRange);
                    }
                    let cfg = &self.0.regs().$cfg;
                    // The register holds `prescale - 1`.
                    cfg.write_cap_prescale(u32::from(config.prescale) - 1);
                    cfg.write_cap_mode(config.edge.field());
                    if config.invert {
                        cfg.set_cap_in_invert();
                    } else {
                        cfg.clr_cap_in_invert();
                    }
                    Ok(())
                }

                /// Enables or disables the channel.
                pub fn enable(&self, enable: bool) {
                    let cfg = &self.0.regs().$cfg;
                    if enable {
                        cfg.set_cap_en();
                    } else {
                        cfg.clr_cap_en();
                    }
                }

                /// The value captured last, in capture-timer ticks.
                pub fn value(&self) -> u32 {
                    self.0.regs().$value.cap_value()
                }

                /// Whether the last capture was triggered by a rising edge.
                pub fn last_rising(&self) -> bool {
                    !self.0.regs().cap_status.[<cap $n _edge>]()
                }

                /// Captures from software.
                pub fn capture_now(&self) {
                    // CAP_SW is write-only: writing 1 captures once.
                    self.0.regs().$cfg.trigger_cap_sw();
                }

                /// The event raised when this channel captures.
                pub const fn event(&self) -> Event {
                    Event::at(27 + $n)
                }

                /// The task that captures on this channel.
                pub const fn capture_task(&self) -> Task {
                    Task::at(19 + $n)
                }
            }
        }
    };
}

// ============================================================================
// Controller API modules
// ============================================================================
//
// One module per controller, each holding that controller's handle types. The
// bodies above are shared: they name `self.0.regs().<field>`, and the field
// names come from the per-instance macros, so `mc0::Timer<'_, 0>` and
// `mc1::Timer<'_, 0>` are generated from the same source text.

macro_rules! impl_api {
    ($api:ident, $ctrl:ident) => {
        pub mod $api {
            //! Typed handles of this controller.

            use super::*;

            /// A timer.
            ///
            /// Built by [`Mcpwm0::timer`] / [`Mcpwm1::timer`], which check `N`
            /// at compile time.
            pub struct Timer<'a, const N: u8>(pub(super) &'a $ctrl);

            /// An operator, i.e. one output pair with its dead time, fault
            /// handling and carrier.
            pub struct Operator<'a, const N: u8>(pub(super) &'a $ctrl);

            /// A capture channel.
            pub struct Capture<'a, const N: u8>(pub(super) &'a $ctrl);

            /// A fault input.
            pub struct Fault<'a, const N: u8>(pub(super) &'a $ctrl);

            /// The free-running capture timer, one per controller.
            pub struct CaptureTimer<'a>(pub(super) &'a $ctrl);

            impl<'a> CaptureTimer<'a> {
                /// Starts the 32-bit counter the capture channels sample.
                pub fn start(&self) {
                    self.0.regs().cap_timer_cfg.set_cap_timer_en();
                }

                /// Stops it; captured values stop advancing.
                pub fn stop(&self) {
                    self.0.regs().cap_timer_cfg.clr_cap_timer_en();
                }

                /// The value it reloads with on a sync event.
                pub fn set_phase(&self, phase: u32) {
                    self.0.regs().cap_timer_phase.write_cap_phase(phase);
                }

                /// Lets a sync event reload the timer from its phase register;
                /// `None` disables the reload.
                pub fn sync_input(&self, source: Option<SyncSource>) {
                    let cfg = &self.0.regs().cap_timer_cfg;
                    match source {
                        Some(source) => {
                            cfg.write_cap_synci_sel(source.field());
                            cfg.set_cap_synci_en();
                        }
                        None => cfg.clr_cap_synci_en(),
                    }
                }

                /// Triggers one sync of the capture timer from software.
                pub fn software_sync(&self) {
                    self.0.regs().cap_timer_cfg.trigger_cap_sync_sw();
                }
            }

            timer_body!(0, timer0_cfg0, timer0_cfg1, timer0_status, timer0_sync);
            timer_body!(1, timer1_cfg0, timer1_cfg1, timer1_status, timer1_sync);
            timer_body!(2, timer2_cfg0, timer2_cfg1, timer2_status, timer2_sync);

            operator_body!(
                0,
                gen0_cfg0,
                gen0_stmp_cfg,
                gen0_tstmp_a,
                gen0_tstmp_b,
                gen0_force,
                gen0_a,
                gen0_b,
                dt0_cfg,
                dt0_fed_cfg,
                dt0_red_cfg,
                carrier0_cfg,
                fh0_cfg0,
                fh0_cfg1,
                fh0_status,
                op0_tstmp_e1,
                op0_tstmp_e2
            );
            operator_body!(
                1,
                gen1_cfg0,
                gen1_stmp_cfg,
                gen1_tstmp_a,
                gen1_tstmp_b,
                gen1_force,
                gen1_a,
                gen1_b,
                dt1_cfg,
                dt1_fed_cfg,
                dt1_red_cfg,
                carrier1_cfg,
                fh1_cfg0,
                fh1_cfg1,
                fh1_status,
                op1_tstmp_e1,
                op1_tstmp_e2
            );
            operator_body!(
                2,
                gen2_cfg0,
                gen2_stmp_cfg,
                gen2_tstmp_a,
                gen2_tstmp_b,
                gen2_force,
                gen2_a,
                gen2_b,
                dt2_cfg,
                dt2_fed_cfg,
                dt2_red_cfg,
                carrier2_cfg,
                fh2_cfg0,
                fh2_cfg1,
                fh2_status,
                op2_tstmp_e1,
                op2_tstmp_e2
            );

            fault_body!(0);
            fault_body!(1);
            fault_body!(2);

            capture_body!(0, cap_ch0_cfg, cap_ch0);
            capture_body!(1, cap_ch1_cfg, cap_ch1);
            capture_body!(2, cap_ch2_cfg, cap_ch2);
        }
    };
}

impl_api!(mc0, Mcpwm0);
impl_api!(mc1, Mcpwm1);

// ============================================================================
// Controllers
// ============================================================================

macro_rules! impl_mcpwm {
    ($Name:ident, $PAC:ident, $mod:ident, $api:ident, $apb:ident, $clk_en:ident, $src_sel:ident, $div:ident, $rst:ident, $evt_base:literal, $task_base:literal, $irq:ident) => {
        pastey::paste! {
            /// MCPWM controller owning the PAC peripheral.
            pub struct $Name {
                mcpwm: $PAC,
            }

            impl $Name {
                /// The interrupt source of this controller.
                pub const SOURCE: Interrupt = Interrupt::$irq;

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

                /// Address of the register block, for the bit-table helpers.
                #[inline]
                pub(crate) const fn base(&self) -> u32 {
                    $mod::BASE
                }

                /// First ETM event id of this controller.
                #[inline]
                pub(crate) const fn event_base(&self) -> u8 {
                    $evt_base
                }

                /// Returns the PAC peripheral.
                #[inline]
                pub fn release(self) -> $PAC {
                    self.mcpwm
                }

                /// A handle to timer `N`.
                ///
                /// `N` is 0–2; anything else fails to compile.
                #[inline]
                pub fn timer<const N: u8>(&self) -> $api::Timer<'_, N> {
                    const {
                        assert!(N < 3, "MCPWM has three timers, 0 to 2")
                    };
                    $api::Timer(self)
                }

                /// A handle to operator `N`.
                ///
                /// `N` is 0–2; anything else fails to compile.
                #[inline]
                pub fn operator<const N: u8>(&self) -> $api::Operator<'_, N> {
                    const {
                        assert!(N < 3, "MCPWM has three operators, 0 to 2")
                    };
                    $api::Operator(self)
                }

                /// A handle to capture channel `N`.
                ///
                /// `N` is 0–2; anything else fails to compile.
                #[inline]
                pub fn capture<const N: u8>(&self) -> $api::Capture<'_, N> {
                    const {
                        assert!(N < 3, "MCPWM has three capture channels, 0 to 2")
                    };
                    $api::Capture(self)
                }

                /// A handle to fault input `N`.
                ///
                /// `N` is 0–2; anything else fails to compile.
                #[inline]
                pub fn fault<const N: u8>(&self) -> $api::Fault<'_, N> {
                    const {
                        assert!(N < 3, "MCPWM has three fault inputs, 0 to 2")
                    };
                    $api::Fault(self)
                }

                /// A handle to the capture timer.
                #[inline]
                pub fn capture_timer(&self) -> $api::CaptureTimer<'_> {
                    $api::CaptureTimer(self)
                }

                /// Selects the clock source and its group divider.
                ///
                /// The divider sets `PWM_clk = source / divider`; timers derive
                /// their own prescaler from `PWM_clk`, which
                /// [`mc0::Timer::configure`] reads back from the clock registers
                /// rather than being told.
                ///
                /// # Errors
                ///
                /// [`Error::DividerOutOfRange`] when `divider` is outside 1–256.
                pub fn set_clock_source(
                    &self,
                    clkrst: &HP_SYS_CLKRST,
                    source: ClockSource,
                    divider: u32,
                ) -> Result<(), Error> {
                    if !(1..=256).contains(&divider) {
                        return Err(Error::DividerOutOfRange);
                    }
                    clkrst.peri_clk_ctrl20.[<write_ $src_sel>](source.field());
                    clkrst.peri_clk_ctrl20.[<write_ $div>](divider - 1);
                    clkrst.peri_clk_ctrl20.[<set_ $clk_en>]();
                    Ok(())
                }

                /// The frequency the group divider produces, read back from
                /// `PERI_CLK_CTRL20`.
                ///
                /// # Errors
                ///
                /// [`Error::UnknownClockSource`] when the selector holds a value
                /// whose frequency this driver does not know.
                pub fn group_clock_hz(&self, clkrst: &HP_SYS_CLKRST) -> Result<u32, Error> {
                    let source = ClockSource::from_field(clkrst.peri_clk_ctrl20.[<$src_sel>]())
                        .ok_or(Error::UnknownClockSource)?;
                    let divider = clkrst.peri_clk_ctrl20.[<$div>]() + 1;
                    Ok(source.hz() / divider)
                }

                // ============================================================
                // ETM
                // ============================================================

                /// Exposes one of this controller's events to the ETM matrix.
                pub fn enable_event(&self, event: Event, enable: bool) {
                    // `EVT_EN` has one bit per event, at the event's ETM offset.
                    set_indexed_bit($mod::BASE + EVT_EN_OFFSET, u32::from(event.raw()), enable)
                }

                /// Lets an ETM task drive this controller.
                pub fn enable_task(&self, task: Task, enable: bool) {
                    set_indexed_bit($mod::BASE + TASK_EN_OFFSET, u32::from(task.raw()), enable)
                }

                /// The task that stops every timer and holds the outputs.
                pub const fn stop_generators_task(&self) -> Task {
                    Task::at(6)
                }

                /// The `SOC_ETM` id of an event of this controller.
                pub fn event_id(&self, event: Event) -> crate::etm::EventId {
                    crate::etm::EventId::new($evt_base + event.raw())
                }

                /// The `SOC_ETM` id of a task of this controller.
                pub fn task_id(&self, task: Task) -> crate::etm::TaskId {
                    crate::etm::TaskId::new($task_base + task.raw())
                }

                // ============================================================
                // Interrupts
                // ============================================================

                /// Enables the interrupt of one event.
                ///
                /// The bit is the same one that [`enable_event`](Self::enable_event)
                /// uses, so an event can drive the ETM, raise an interrupt, or
                /// both.
                pub fn enable_interrupt(&self, event: Event, enable: bool) {
                    set_indexed_bit(
                        $mod::BASE + INT_ENA_OFFSET,
                        u32::from(event.raw()),
                        enable,
                    )
                }

                /// Whether an enabled event's interrupt is pending.
                pub fn interrupt_pending(&self, event: Event) -> bool {
                    read_indexed_bit($mod::BASE + INT_ST_OFFSET, u32::from(event.raw()))
                }

                /// Acknowledges one event's interrupt.
                pub fn clear_interrupt(&self, event: Event) {
                    // `INT_CLR` is write-1-to-clear.
                    write_indexed_bit($mod::BASE + INT_CLR_OFFSET, u32::from(event.raw()))
                }

                /// Acknowledges every interrupt of this controller.
                pub fn clear_interrupts(&self) {
                    unsafe {
                        core::ptr::write_volatile(
                            ($mod::BASE + INT_CLR_OFFSET) as *mut u32,
                            (1 << 30) - 1,
                        )
                    }
                }
            }
        }
    };
}

impl_mcpwm!(
    Mcpwm0,
    MCPWM0,
    mcpwm0,
    mc0,
    reg_mcpwm0_apb_clk_en,
    reg_mcpwm0_clk_en,
    reg_mcpwm0_clk_src_sel,
    reg_mcpwm0_clk_div_num,
    reg_rst_en_pwm0,
    60,
    113,
    PWM0
);
impl_mcpwm!(
    Mcpwm1,
    MCPWM1,
    mcpwm1,
    mc1,
    reg_mcpwm1_apb_clk_en,
    reg_mcpwm1_clk_en,
    reg_mcpwm1_clk_src_sel,
    reg_mcpwm1_clk_div_num,
    reg_rst_en_pwm1,
    96,
    135,
    PWM1
);
