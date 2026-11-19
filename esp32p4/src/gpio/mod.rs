//! GPIO (General Purpose Input/Output) HAL for ESP32-P4
//!
//! This module provides a type-safe interface for configuring and using GPIO
//! pins on the ESP32-P4. Each pin is a distinct type, so a pin cannot be
//! configured by two drivers at once.
//!
//! # Features
//!
//! - Electrical pad configuration: function, pulls, drive strength, open-drain
//! - GPIO input/output with `OUT_W1TS`/`OUT_W1TC` (no read-modify-write on the
//!   shared register)
//! - Peripheral signal routing through the GPIO matrix, with named constants
//!   for every signal in [`signal`]
//! - Four interrupt outputs with edge/level selection, per-line enable and
//!   status handling
//! - Light-sleep wake-up enable
//!
//! # Architecture Overview
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────────────┐
//! │                        GPIO Hardware Architecture                    │
//! ├──────────────────────────────────────────────────────────────────────┤
//! │                                                                      │
//! │  ┌────────────────────────────────────────────────────────────────┐  │
//! │  │                 GPIO matrix (peripheral routing)               │  │
//! │  │                                                                │  │
//! │  │  • FUNC<n>_OUT_SEL_CFG: OUT_SEL picks peripheral output signal │  │
//! │  │    (256 = the pin's own GPIO_OUT bit)                          │  │
//! │  │  • OEN_SEL: output enable comes from GPIO_ENABLE               │  │
//! │  │  • GPIO_OUT / GPIO_ENABLE with W1TS/W1TC aliases               │  │
//! │  │                                                                │  │
//! │  │  Registers: OUT, OUT1, ENABLE, ENABLE1, IN, IN1, FUNC*_CFG    │  │
//! │  └────────────────────────────────────────────────────────────────┘  │
//! │                                                                      │
//! │  ┌────────────────────────────────────────────────────────────────┐  │
//! │  │                 IO MUX (electrical configuration)              │  │
//! │  │                                                                │  │
//! │  │  • MCU_SEL: pin function (1 = GPIO on every pin)               │  │
//! │  │  • FUN_IE / FUN_WPU / FUN_WPD: input buffer and pulls          │  │
//! │  │  • FUN_DRV: drive strength (~5/10/20/40 mA)                    │  │
//! │  │                                                                │  │
//! │  │  Registers: IO_MUX_GPIO0 .. IO_MUX_GPIO53                      │  │
//! │  └────────────────────────────────────────────────────────────────┘  │
//! │                                                                      │
//! └──────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Pin Count
//!
//! | Pins        | Notes                                            |
//! |-------------|--------------------------------------------------|
//! | GPIO0-15    | Low-power pins; also reachable from the LP domain |
//! | GPIO16-53   | High-performance pins                            |
//!
//! GPIO54-56 exist in the GPIO register blocks but have no HP IO MUX register,
//! so this module exposes GPIO0-53 only.
//!
//! # Ownership Model
//!
//! ```text
//! Peripherals::take()
//!       │
//!       ├── GPIO ────────┐
//!       ├── IO_MUX ──────┼──► Pins::init() ──► pins.pin::<N>()
//!       │                │                          │
//!       │  HP_SYS_CLKRST ┘  (borrowed)              ├──► led.into_output()
//!       │                                           └──► button.into_input_pullup()
//!       │
//!       └── ...                                    unsafe { Pin::<N>::steal() } (ISR)
//! ```
//!
//! 1. [`Pins`] is created by consuming `GPIO` and `IO_MUX` and borrowing
//!    `HP_SYS_CLKRST`; it opens the register clocks and releases the IO MUX
//!    from reset.
//! 2. [`Pins::pin()`] returns a `Pin<N>` token for a specific pin number.
//! 3. Configuration methods take `&mut self`, I/O methods take `&self`.
//!
//! # Example
//!
//! ## Blink an LED
//!
//! ```ignore
//! let dp = Peripherals::take().unwrap();
//! let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
//!
//! let mut led = pins.pin::<2>();
//! led.into_output();
//!
//! loop {
//!     led.toggle();
//! }
//! ```
//!
//! ## Button input with pull-up
//!
//! ```ignore
//! let mut button = pins.pin::<3>();
//! button.into_input_pullup();
//!
//! if button.get() {
//!     // released (active-low)
//! }
//! ```
//!
//! ## Button interrupt
//!
//! ```ignore
//! use esp32p4::gpio::{InterruptLine, InterruptType, Pin};
//! use esp32p4::interrupt::{self, CpuInterrupt, Level};
//!
//! fn isr() {
//!     // SAFETY: `Pins::init` has run and nobody reconfigures the pin.
//!     let button = unsafe { Pin::<3>::steal() };
//!     if button.interrupt_status() {
//!         button.clear_interrupt();
//!         // wake the main loop ...
//!     }
//! }
//!
//! let mut button = pins.pin::<3>();
//! button.into_input_pullup();
//! button.set_interrupt_type(InterruptType::NegativeEdge);
//! button.enable_interrupt(InterruptLine::Line0);
//!
//! let handler = unsafe {
//!     interrupt::claim(
//!         InterruptLine::Line0.source(),
//!         CpuInterrupt::new(21),
//!         Level::LOW,
//!         isr,
//!     )
//! };
//! handler.enable();
//! unsafe { interrupt::enable() };
//! ```
//!
//! ## Route a UART to two pins
//!
//! ```ignore
//! use esp32p4::gpio::signal::{InputSignal, OutputSignal};
//!
//! tx.set_output_signal(OutputSignal::UART0_TXD_PAD_OUT);
//! rx.set_input_signal(InputSignal::UART0_RXD_PAD_IN);
//! ```

use esp32p4_pac::{GPIO, HP_SYS_CLKRST, IO_MUX, Interrupt};
use pastey::paste;

pub mod signal;

pub use signal::{InputSignal, OutputSignal};

/// Output signal number that selects the GPIO matrix's own output register.
///
/// `OUT_SEL = 256` makes the pin output the value of its `GPIO_OUT` bit.
const OUTPUT_SIGNAL_GPIO: u32 = 256;

/// Number of GPIO pins with an HP IO MUX register (GPIO0..GPIO53).
pub const GPIO_COUNT: u32 = 54;

/// IO MUX function selection.
///
/// The value selects which peripheral drives the pin. Function 1 is the GPIO
/// function on every pin; the other functions are pin-specific — see the
/// "HP IO MUX Functions List" table in the ESP32-P4 TRM.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Function {
    /// IO MUX function 0 (pin-specific peripheral signal).
    Function0 = 0,
    /// IO MUX function 1 — GPIO, routed through the GPIO matrix.
    Gpio = 1,
    /// IO MUX function 2 (pin-specific peripheral signal).
    Function2 = 2,
    /// IO MUX function 3 (pin-specific peripheral signal).
    Function3 = 3,
    /// IO MUX function 4 (pin-specific peripheral signal).
    Function4 = 4,
    /// IO MUX function 5 (pin-specific peripheral signal).
    Function5 = 5,
    /// IO MUX function 6 (pin-specific peripheral signal).
    Function6 = 6,
    /// IO MUX function 7 (pin-specific peripheral signal).
    Function7 = 7,
}

/// Pull resistor configuration.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pull {
    /// No pull resistor (floating input).
    None,
    /// Pull-up resistor.
    Up,
    /// Pull-down resistor.
    Down,
}

/// Output drive strength (approximate).
///
/// The values map directly to `IO_MUX_GPIOx.FUN_DRV`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DriveStrength {
    /// ~5 mA
    _5mA = 0,
    /// ~10 mA
    _10mA = 1,
    /// ~20 mA
    _20mA = 2,
    /// ~40 mA
    _40mA = 3,
}

/// Interrupt edge or level selected by `GPIO_PINn.INT_TYPE`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum InterruptType {
    /// Interrupt disabled.
    Disabled = 0,
    /// Positive (rising) edge.
    PositiveEdge = 1,
    /// Negative (falling) edge.
    NegativeEdge = 2,
    /// Either edge.
    AnyEdge = 3,
    /// Low level.
    LowLevel = 4,
    /// High level.
    HighLevel = 5,
}

/// One of the four GPIO interrupt outputs (`GPIO_INTR0`..`GPIO_INTR3`).
///
/// `INT_ENA` has bits 13, 14, 16 and 17 for these lines; bit 15 is reserved.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InterruptLine {
    /// `GPIO_INTR0` (`Interrupt::GPIO_INT0`).
    Line0,
    /// `GPIO_INTR1` (`Interrupt::GPIO_INT1`).
    Line1,
    /// `GPIO_INTR2` (`Interrupt::GPIO_INT2`).
    Line2,
    /// `GPIO_INTR3` (`Interrupt::GPIO_INT3`).
    Line3,
}

impl InterruptLine {
    /// All four lines, in order.
    pub const ALL: [Self; 4] = [Self::Line0, Self::Line1, Self::Line2, Self::Line3];

    /// Position of the line's enable bit inside the `INT_ENA` field.
    #[inline]
    const fn field_bit(self) -> u32 {
        match self {
            Self::Line0 => 0,
            Self::Line1 => 1,
            Self::Line2 => 3,
            Self::Line3 => 4,
        }
    }

    /// The interrupt-matrix source this line feeds.
    #[inline]
    pub const fn source(self) -> Interrupt {
        match self {
            Self::Line0 => Interrupt::GPIO_INT0,
            Self::Line1 => Interrupt::GPIO_INT1,
            Self::Line2 => Interrupt::GPIO_INT2,
            Self::Line3 => Interrupt::GPIO_INT3,
        }
    }
}

/// Which signal drives the pin's output enable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OutputEnable {
    /// `GPIO_ENABLE` bit `n` (the pin behaves as a normal GPIO output).
    Gpio,
    /// The peripheral's own output-enable signal (open-drain buses such as I2C).
    Peripheral,
}

/// The pin operations a peripheral driver needs.
///
/// Implemented for every [`Pin`], so a driver can be generic over the pin
/// instead of over the pin number:
///
/// ```ignore
/// let mut uart = Uart0::init(dp.UART0, UartConfig::default(), &dp.HP_SYS_CLKRST);
/// uart.set_pins(pins.pin::<20>(), pins.pin::<21>());
/// ```
///
/// Unlike the fixed-function pins of other MCUs, the ESP32-P4 GPIO matrix
/// accepts *any* pin for *any* peripheral signal, so the trait does not
/// restrict which pin a driver may use — it only gives the driver a way to
/// reach the routing registers.
pub trait PinOps {
    /// The GPIO number.
    fn number(&self) -> u32;

    /// Route a peripheral output signal to this pin (push-pull, output enable
    /// from `GPIO_ENABLE`).
    fn route_output_signal(&mut self, signal: OutputSignal);

    /// Route a peripheral output signal to this pin, with the output enable
    /// coming from the peripheral (open-drain buses such as I2C).
    fn route_output_signal_open_drain(&mut self, signal: OutputSignal);

    /// Connect a peripheral input signal to this pin, enabling its input
    /// buffer.
    fn route_input_signal(&mut self, signal: InputSignal);

    /// Return the pad to its own `GPIO_OUT` register.
    fn release_output_signal(&mut self);

    /// Detach a peripheral input signal from this pin.
    fn release_input_signal(&mut self, signal: InputSignal);

    /// Attach or detach the pad's pull-up.
    fn set_pull_up_enabled(&mut self, enabled: bool);

    /// Enable or disable the input buffer.
    fn set_input_buffer_enabled(&mut self, enabled: bool);

    /// Switch the pad's output driver to open-drain.
    fn set_open_drain_enabled(&mut self, enabled: bool);
}

/// Latched interrupt status of all 54 pins.
#[derive(Clone, Copy, Debug)]
pub struct InterruptStatus {
    low: u32,
    high: u32,
}

impl InterruptStatus {
    /// The two raw status words: GPIO0-31 and GPIO32-53.
    #[inline]
    pub const fn raw(self) -> (u32, u32) {
        (self.low, self.high)
    }

    /// Whether pin `pin` latched an interrupt.
    #[inline]
    pub fn is_set(self, pin: u32) -> bool {
        assert!(pin < GPIO_COUNT, "GPIO pin number must be 0-53");
        let mask = 1u32 << (pin % 32);
        if pin < 32 {
            self.low & mask != 0
        } else {
            self.high & mask != 0
        }
    }
}

// ============================================================================
// Pin Implementation Macro
// ============================================================================

/// Implements `Pin<N>` with compile-time known register names.
///
/// Pins 0-31 live in `OUT`/`ENABLE`/`IN`, pins 32+ in the `*1` counterparts;
/// the `low` / `high` arms pick the right registers and bit position, so the
/// generated code uses a fixed register name instead of a runtime lookup.
macro_rules! impl_pin {
    ($n:literal, low) => {
        impl_pin!(
            @impl $n,
            1u32 << $n,
            enable_w1ts, enable_w1tc, enable,
            out_w1ts, out_w1tc, out, r#in,
            status_w1ts, status_w1tc, status, status_next,
            intr0, intr1, intr2, intr3
        );
    };
    ($n:literal, high) => {
        impl_pin!(
            @impl $n,
            1u32 << ($n - 32),
            enable1_w1ts, enable1_w1tc, enable1,
            out1_w1ts, out1_w1tc, out1, in1,
            status1_w1ts, status1_w1tc, status1, status_next1,
            intr10, intr11, intr12, intr13
        );
    };
    (@impl $n:literal, $mask:expr, $en_set:ident, $en_clr:ident, $en_reg:ident, $out_set:ident, $out_clr:ident, $out_reg:ident, $in_reg:ident, $status_set:ident, $status_clr:ident, $status_reg:ident, $status_next:ident, $intr0:ident, $intr1:ident, $intr2:ident, $intr3:ident) => {
        paste! {
            impl Pin<$n> {
                /// Pin number.
                pub const PIN: u32 = $n;

                /// Bit mask for this pin in the bank's 32-bit GPIO registers.
                pub const MASK: u32 = $mask;

                /// Obtain a `Pin<N>` handle without ownership checks.
                ///
                /// Useful in ISR context, where pin ownership is held by
                /// `main()` and not accessible.
                ///
                /// # Safety
                ///
                /// The caller must ensure that:
                /// - the GPIO subsystem has been initialised (via [`Pins::init()`]);
                /// - no data races occur — i.e. the same pin is not reconfigured
                ///   concurrently from another context.
                #[inline]
                pub const unsafe fn steal() -> crate::Steal<Self> {
                    crate::Steal(Self { _marker: core::marker::PhantomData })
                }

                // ================================================================
                // Internal register accessors
                // ================================================================

                #[inline(always)]
                fn gpio(&self) -> GPIO {
                    // SAFETY: `Pins::init()` owns the peripheral; a ZST handle is
                    // only a register-block pointer.
                    unsafe { GPIO::steal() }
                }

                #[inline(always)]
                fn io_mux(&self) -> IO_MUX {
                    // SAFETY: as above.
                    unsafe { IO_MUX::steal() }
                }

                // ================================================================
                // Function Selection
                // ================================================================

                /// Set the IO MUX function.
                ///
                /// # Example
                ///
                /// ```ignore
                /// pin.set_function(Function::Gpio);
                /// ```
                #[inline]
                pub fn set_function(&mut self, func: Function) {
                    self.io_mux().[<gpio $n>].write_mcu_sel(func as u32);
                }

                /// Get the current IO MUX function.
                #[inline]
                pub fn function(&self) -> u32 {
                    self.io_mux().[<gpio $n>].mcu_sel()
                }

                /// Set function to GPIO (routed through the GPIO matrix).
                #[inline]
                pub fn set_function_gpio(&mut self) {
                    self.set_function(Function::Gpio);
                }

                // ================================================================
                // Pad Configuration
                // ================================================================

                /// Reset the pad and GPIO-matrix routing to a known state.
                ///
                /// This method:
                /// - Selects the GPIO function and routes `GPIO_OUT` to the pad
                /// - Lets `GPIO_ENABLE` control the pad output enable
                /// - Disables the input buffer and both pull resistors
                /// - Sets ~20 mA drive strength
                ///
                /// Call this before using a pin for the first time.
                #[inline]
                pub fn init_pad(&mut self) {
                    self.set_function_gpio();

                    let pad = &self.io_mux().[<gpio $n>];
                    pad.clr_fun_ie();
                    pad.clr_fun_wpu();
                    pad.clr_fun_wpd();
                    pad.write_fun_drv(DriveStrength::_20mA as u32);

                    let matrix = &self.gpio().[<func $n _out_sel_cfg>];
                    matrix.write_out_sel(OUTPUT_SIGNAL_GPIO);
                    matrix.set_oen_sel();
                }

                /// Set pull resistor configuration.
                #[inline]
                pub fn set_pull(&mut self, pull: Pull) {
                    let pad = &self.io_mux().[<gpio $n>];
                    match pull {
                        Pull::None => {
                            pad.clr_fun_wpu();
                            pad.clr_fun_wpd();
                        }
                        Pull::Up => {
                            pad.set_fun_wpu();
                            pad.clr_fun_wpd();
                        }
                        Pull::Down => {
                            pad.clr_fun_wpu();
                            pad.set_fun_wpd();
                        }
                    }
                }

                /// Enable pull-up resistor.
                #[inline]
                pub fn set_pull_up(&mut self) {
                    self.set_pull(Pull::Up);
                }

                /// Enable pull-down resistor.
                #[inline]
                pub fn set_pull_down(&mut self) {
                    self.set_pull(Pull::Down);
                }

                /// Disable pull resistors.
                #[inline]
                pub fn set_pull_none(&mut self) {
                    self.set_pull(Pull::None);
                }

                /// Set output drive strength.
                #[inline]
                pub fn set_drive_strength(&mut self, strength: DriveStrength) {
                    self.io_mux().[<gpio $n>].write_fun_drv(strength as u32);
                }

                /// Enable the input buffer.
                #[inline]
                pub fn enable_input(&mut self) {
                    self.io_mux().[<gpio $n>].set_fun_ie();
                }

                /// Disable the input buffer.
                #[inline]
                pub fn disable_input(&mut self) {
                    self.io_mux().[<gpio $n>].clr_fun_ie();
                }

                // ================================================================
                // Direction Control
                // ================================================================

                /// Enable the pin's output driver.
                ///
                /// Uses the write-1-to-set register, so other pins are untouched.
                #[inline]
                pub fn set_direction_out(&mut self) {
                    self.gpio().$en_set.[<write_ $en_set>](Self::MASK);
                }

                /// Disable the pin's output driver (high impedance).
                ///
                /// Uses the write-1-to-clear register, so other pins are untouched.
                #[inline]
                pub fn set_direction_in(&mut self) {
                    self.gpio().$en_clr.[<write_ $en_clr>](Self::MASK);
                }

                /// Check if the output driver is enabled.
                #[inline]
                pub fn is_output(&self) -> bool {
                    (self.gpio().$en_reg.data() & Self::MASK) != 0
                }

                // ================================================================
                // Input/Output Operations
                // ================================================================

                /// Get the input value.
                ///
                /// Returns `true` if the pin is high, `false` if low.
                #[inline]
                pub fn get(&self) -> bool {
                    (self.gpio().$in_reg.data_next() & Self::MASK) != 0
                }

                /// Set output high.
                ///
                /// Uses the write-1-to-set register, so other pins are untouched.
                #[inline]
                pub fn set_high(&self) {
                    self.gpio().$out_set.[<write_ $out_set>](Self::MASK);
                }

                /// Set output low.
                ///
                /// Uses the write-1-to-clear register, so other pins are untouched.
                #[inline]
                pub fn set_low(&self) {
                    self.gpio().$out_clr.[<write_ $out_clr>](Self::MASK);
                }

                /// Toggle output.
                ///
                /// The GPIO block has no XOR register, so this reads the current
                /// output level and sets or clears it.
                #[inline]
                pub fn toggle(&self) {
                    if self.output_level() {
                        self.set_low();
                    } else {
                        self.set_high();
                    }
                }

                /// Set output to a specific value.
                #[inline]
                pub fn set(&self, high: bool) {
                    if high {
                        self.set_high();
                    } else {
                        self.set_low();
                    }
                }

                /// Get the current output level (not the actual pin state).
                ///
                /// This reads the output register, not the input. Use [`Pin::get`]
                /// to read the pin itself.
                #[inline]
                pub fn output_level(&self) -> bool {
                    (self.gpio().$out_reg.data_orig() & Self::MASK) != 0
                }

                // ================================================================
                // GPIO Matrix Routing
                // ================================================================

                /// Route a peripheral output signal to this pin.
                ///
                /// Selects the GPIO function in the IO MUX, writes `signal` into
                /// `FUNCn_OUT_SEL_CFG`, takes the output enable from
                /// `GPIO_ENABLE` and enables the driver. Use this for push-pull
                /// peripherals (UART TX, SPI clock, LEDC, RMT, ...).
                ///
                /// Open-drain buses such as I2C need the peripheral's own output
                /// enable: use [`Pin::set_output_signal_with_oe`] with
                /// [`OutputEnable::Peripheral`].
                #[inline]
                pub fn set_output_signal(&mut self, signal: OutputSignal) {
                    self.set_output_signal_with_oe(signal, OutputEnable::Gpio);
                    self.set_direction_out();
                }

                /// Route a peripheral output signal and choose the output-enable
                /// source.
                #[inline]
                pub fn set_output_signal_with_oe(&mut self, signal: OutputSignal, oe: OutputEnable) {
                    self.set_function_gpio();
                    let matrix = &self.gpio().[<func $n _out_sel_cfg>];
                    matrix.write_out_sel(signal.index() as u32);
                    match oe {
                        OutputEnable::Gpio => matrix.set_oen_sel(),
                        OutputEnable::Peripheral => matrix.clr_oen_sel(),
                    }
                }

                /// Return the pin to its own `GPIO_OUT` register.
                #[inline]
                pub fn clear_output_signal(&mut self) {
                    let matrix = &self.gpio().[<func $n _out_sel_cfg>];
                    matrix.write_out_sel(OUTPUT_SIGNAL_GPIO);
                    matrix.set_oen_sel();
                }

                /// The output signal currently routed to this pin.
                #[inline]
                pub fn output_signal(&self) -> OutputSignal {
                    OutputSignal::from_raw(self.gpio().[<func $n _out_sel_cfg>].out_sel() as u16)
                }

                /// Invert the output signal before it reaches the pad.
                #[inline]
                pub fn set_output_inverted(&mut self, inverted: bool) {
                    let matrix = &self.gpio().[<func $n _out_sel_cfg>];
                    if inverted {
                        matrix.set_inv_sel();
                    } else {
                        matrix.clr_inv_sel();
                    }
                }

                /// Configure the pad as open-drain.
                #[inline]
                pub fn set_open_drain(&mut self, open_drain: bool) {
                    let pin = &self.gpio().[<pin $n>];
                    if open_drain {
                        pin.set_pad_driver();
                    } else {
                        pin.clr_pad_driver();
                    }
                }

                /// Connect a peripheral input signal to this pin.
                ///
                /// Enables the input buffer, selects the GPIO function and points
                /// the signal's `FUNCn_IN_SEL_CFG` at this pin. The same pin may
                /// feed several signals; configure each one with this method.
                #[inline]
                pub fn set_input_signal(&mut self, signal: InputSignal) {
                    self.set_function_gpio();
                    self.enable_input();
                    signal::select_input_pin(signal, Self::PIN);
                }

                /// Disconnect a peripheral input signal from any pin.
                ///
                /// Leaves the pad and the input buffer alone, since another
                /// signal may still use the pin.
                #[inline]
                pub fn clear_input_signal(&mut self, signal: InputSignal) {
                    signal::bypass_input_signal(signal);
                }

                /// Invert a peripheral input signal on its way to the peripheral.
                ///
                /// The inversion belongs to the signal, not to the pin, so this
                /// affects every pin the signal is routed from.
                #[inline]
                pub fn set_input_inverted(&mut self, signal: InputSignal, inverted: bool) {
                    signal::invert_input_signal(signal, inverted);
                }

                // ================================================================
                // Interrupts
                // ================================================================

                /// Set the trigger condition (`GPIO_PINn.INT_TYPE`).
                #[inline]
                pub fn set_interrupt_type(&mut self, kind: InterruptType) {
                    self.gpio().[<pin $n>].write_int_type(kind as u32);
                }

                /// Read the trigger condition.
                #[inline]
                pub fn interrupt_type(&self) -> InterruptType {
                    match self.gpio().[<pin $n>].int_type() {
                        1 => InterruptType::PositiveEdge,
                        2 => InterruptType::NegativeEdge,
                        3 => InterruptType::AnyEdge,
                        4 => InterruptType::LowLevel,
                        5 => InterruptType::HighLevel,
                        _ => InterruptType::Disabled,
                    }
                }

                /// Enable this pin as a source of `line`.
                #[inline]
                pub fn enable_interrupt(&mut self, line: InterruptLine) {
                    let pin = &self.gpio().[<pin $n>];
                    pin.write_int_ena(pin.int_ena() | (1 << line.field_bit()));
                }

                /// Stop this pin from driving `line`.
                #[inline]
                pub fn disable_interrupt(&mut self, line: InterruptLine) {
                    let pin = &self.gpio().[<pin $n>];
                    pin.write_int_ena(pin.int_ena() & !(1 << line.field_bit()));
                }

                /// Whether this pin drives `line`.
                #[inline]
                pub fn interrupt_enabled(&self, line: InterruptLine) -> bool {
                    self.gpio().[<pin $n>].int_ena() & (1 << line.field_bit()) != 0
                }

                /// Disable every GPIO interrupt line for this pin.
                #[inline]
                pub fn disable_all_interrupts(&mut self) {
                    self.gpio().[<pin $n>].write_int_ena(0);
                }

                /// Whether the pin's latched interrupt status bit is set.
                ///
                /// This is the raw `GPIO_STATUS_INTERRUPT` latch, shared by all
                /// four lines; it stays set until [`Pin::clear_interrupt`].
                #[inline]
                pub fn interrupt_status(&self) -> bool {
                    self.gpio().$status_reg.read_bits() & Self::MASK != 0
                }

                /// Clear the pin's latched interrupt status.
                #[inline]
                pub fn clear_interrupt(&self) {
                    self.gpio().$status_clr.[<write_ $status_clr>](Self::MASK);
                }

                /// Whether `line` is currently masked-in and asserting.
                ///
                /// Reads `GPIO_INTRn`/`GPIO_INTR1_n`, the per-line masked status.
                #[inline]
                pub fn interrupt_pending(&self, line: InterruptLine) -> bool {
                    let status = match line {
                        InterruptLine::Line0 => self.gpio().$intr0.read_bits(),
                        InterruptLine::Line1 => self.gpio().$intr1.read_bits(),
                        InterruptLine::Line2 => self.gpio().$intr2.read_bits(),
                        InterruptLine::Line3 => self.gpio().$intr3.read_bits(),
                    };
                    status & Self::MASK != 0
                }

                /// The raw trigger condition, independent of `INT_ENA`.
                ///
                /// Useful in a shared handler that needs to know whether the pin
                /// is still asserting.
                #[inline]
                pub fn interrupt_source(&self) -> bool {
                    self.gpio().$status_next.read_bits() & Self::MASK != 0
                }

                /// Allow this pin to wake the CPU from Light-sleep.
                #[inline]
                pub fn enable_wakeup(&mut self) {
                    self.gpio().[<pin $n>].set_wakeup_enable();
                }

                /// Stop this pin from waking the CPU.
                #[inline]
                pub fn disable_wakeup(&mut self) {
                    self.gpio().[<pin $n>].clr_wakeup_enable();
                }

                // ================================================================
                // Convenience Methods
                // ================================================================

                /// Configure as GPIO output.
                ///
                /// This method:
                /// 1. Resets the pad and matrix routing ([`Pin::init_pad`])
                /// 2. Enables the output driver
                ///
                /// # Returns
                ///
                /// Reference to self for method chaining.
                #[inline]
                pub fn into_output(&mut self) -> &Self {
                    self.init_pad();
                    self.set_direction_out();
                    self
                }

                /// Configure as GPIO input (no pull).
                ///
                /// This method:
                /// 1. Resets the pad and matrix routing ([`Pin::init_pad`])
                /// 2. Enables the input buffer
                /// 3. Disables the output driver
                #[inline]
                pub fn into_input(&mut self) -> &Self {
                    self.init_pad();
                    self.enable_input();
                    self.set_direction_in();
                    self
                }

                /// Configure as GPIO input with pull-up.
                ///
                /// Suitable for active-low buttons and open-drain signals.
                #[inline]
                pub fn into_input_pullup(&mut self) -> &Self {
                    self.into_input();
                    self.set_pull_up();
                    self
                }

                /// Configure as GPIO input with pull-down.
                ///
                /// Suitable for active-high buttons.
                #[inline]
                pub fn into_input_pulldown(&mut self) -> &Self {
                    self.into_input();
                    self.set_pull_down();
                    self
                }
            }

            impl crate::gpio::PinOps for Pin<$n> {
                #[inline]
                fn number(&self) -> u32 {
                    $n
                }

                #[inline]
                fn route_output_signal(&mut self, signal: OutputSignal) {
                    Pin::<$n>::set_output_signal(self, signal);
                }

                #[inline]
                fn route_output_signal_open_drain(&mut self, signal: OutputSignal) {
                    Pin::<$n>::set_output_signal_with_oe(self, signal, OutputEnable::Peripheral);
                }

                #[inline]
                fn route_input_signal(&mut self, signal: InputSignal) {
                    Pin::<$n>::set_input_signal(self, signal);
                }

                #[inline]
                fn release_output_signal(&mut self) {
                    Pin::<$n>::clear_output_signal(self);
                }

                #[inline]
                fn release_input_signal(&mut self, signal: InputSignal) {
                    Pin::<$n>::clear_input_signal(self, signal);
                }

                #[inline]
                fn set_pull_up_enabled(&mut self, enabled: bool) {
                    Pin::<$n>::set_pull(self, if enabled { Pull::Up } else { Pull::None });
                }

                #[inline]
                fn set_input_buffer_enabled(&mut self, enabled: bool) {
                    if enabled {
                        Pin::<$n>::enable_input(self);
                    } else {
                        Pin::<$n>::disable_input(self);
                    }
                }

                #[inline]
                fn set_open_drain_enabled(&mut self, enabled: bool) {
                    Pin::<$n>::set_open_drain(self, enabled);
                }
            }

            impl core::fmt::Debug for Pin<$n> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("Pin")
                        .field("number", &$n)
                        .field("function", &self.function())
                        .field("is_output", &self.is_output())
                        .finish()
                }
            }
        }
    };
}

// ============================================================================
// Pin Handle
// ============================================================================

/// A single GPIO pin.
///
/// Obtained from [`Pins::pin()`]. The type parameter is the pin number, so the
/// register access compiles to a fixed load/store instead of a table lookup.
///
/// In ISR context, a pin handle can be obtained via
/// `unsafe { Pin::<N>::steal() }`, provided the GPIO subsystem has already been
/// initialised.
pub struct Pin<const N: u32> {
    _marker: core::marker::PhantomData<*const ()>,
}

// SAFETY: `Pin<N>` is a zero-sized ownership token. Ownership can move between
// cores / ISRs (`Send`), but `&Pin<N>` must not be shared across cores
// (`!Sync`, guaranteed by `PhantomData<*const ()>`), preventing concurrent
// reconfiguration of the same pad.
unsafe impl<const N: u32> Send for Pin<N> {}

// ============================================================================
// Generate Pin implementations for all pins
// ============================================================================

// GPIO0-31: OUT / ENABLE / IN
impl_pin!(0, low);
impl_pin!(1, low);
impl_pin!(2, low);
impl_pin!(3, low);
impl_pin!(4, low);
impl_pin!(5, low);
impl_pin!(6, low);
impl_pin!(7, low);
impl_pin!(8, low);
impl_pin!(9, low);
impl_pin!(10, low);
impl_pin!(11, low);
impl_pin!(12, low);
impl_pin!(13, low);
impl_pin!(14, low);
impl_pin!(15, low);
impl_pin!(16, low);
impl_pin!(17, low);
impl_pin!(18, low);
impl_pin!(19, low);
impl_pin!(20, low);
impl_pin!(21, low);
impl_pin!(22, low);
impl_pin!(23, low);
impl_pin!(24, low);
impl_pin!(25, low);
impl_pin!(26, low);
impl_pin!(27, low);
impl_pin!(28, low);
impl_pin!(29, low);
impl_pin!(30, low);
impl_pin!(31, low);

// GPIO32-53: OUT1 / ENABLE1 / IN1
impl_pin!(32, high);
impl_pin!(33, high);
impl_pin!(34, high);
impl_pin!(35, high);
impl_pin!(36, high);
impl_pin!(37, high);
impl_pin!(38, high);
impl_pin!(39, high);
impl_pin!(40, high);
impl_pin!(41, high);
impl_pin!(42, high);
impl_pin!(43, high);
impl_pin!(44, high);
impl_pin!(45, high);
impl_pin!(46, high);
impl_pin!(47, high);
impl_pin!(48, high);
impl_pin!(49, high);
impl_pin!(50, high);
impl_pin!(51, high);
impl_pin!(52, high);
impl_pin!(53, high);

// ============================================================================
// Pins Container
// ============================================================================

/// GPIO controller, owning the PAC peripherals.
///
/// Create once with [`Pins::init`], then obtain individual [`Pin`] handles with
/// [`Pins::pin`].
///
/// # Example
///
/// ```ignore
/// let dp = Peripherals::take().unwrap();
/// let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
///
/// let mut led = pins.pin::<2>();
/// led.into_output();
///
/// let mut button = pins.pin::<3>();
/// button.into_input_pullup();
/// ```
pub struct Pins {
    _gpio: GPIO,
    _io_mux: IO_MUX,
}

impl Pins {
    /// Initialise the GPIO subsystem.
    ///
    /// Takes the GPIO and IO MUX peripherals — those are exclusively ours — and
    /// borrows the clock/reset block, which only a few clock-gate bits are
    /// flipped in and which other drivers such as
    /// [`UsbSerialJtag::init`](crate::usb_serial_jtag::UsbSerialJtag::init) need
    /// as well. Then:
    ///
    /// 1. opens the GPIO register clock (`GPIO_CLOCK_GATE.CLK_EN`);
    /// 2. clears every latched interrupt status, so an unmasked line cannot
    ///    fire on bootloader leftovers;
    /// 3. opens the IO MUX clock gate (`HP_SYS_CLKRST.PERI_CLK_CTRL26`);
    /// 4. opens the IO MUX APB clock (`HP_SYS_CLKRST.SOC_CLK_CTRL3`);
    /// 5. releases the IO MUX from reset (`HP_SYS_CLKRST.HP_RST_EN1`).
    ///
    /// `HP_SYS_CLKRST` is needed because ESP32-P4 gates and resets the IO MUX
    /// from the HP system clock/reset block rather than from the IO MUX itself.
    #[inline]
    pub fn init(gpio: GPIO, io_mux: IO_MUX, clkrst: &HP_SYS_CLKRST) -> Self {
        gpio.clock_gate.set_clk_en();
        // The bootloader does not promise a clean interrupt state; a stale
        // latched status would fire as soon as a line is unmasked.
        gpio.status_w1tc.write_bits(u32::MAX);
        gpio.status1_w1tc.write_bits(u32::MAX);

        clkrst.peri_clk_ctrl26.set_reg_iomux_clk_en();
        clkrst.soc_clk_ctrl3.set_reg_iomux_apb_clk_en();
        clkrst.hp_rst_en1.clr_reg_rst_en_iomux();

        // The reset release reaches the IO MUX through the APB clock domain;
        // give it a few cycles before the first pad write.
        for _ in 0..8 {
            core::hint::spin_loop();
        }

        Self {
            _gpio: gpio,
            _io_mux: io_mux,
        }
    }

    /// The latched interrupt status of every pin.
    #[inline]
    pub fn interrupt_status(&self) -> InterruptStatus {
        InterruptStatus {
            low: self._gpio.status.read_bits(),
            high: self._gpio.status1.read_bits(),
        }
    }

    /// Clear every pin's latched interrupt status.
    #[inline]
    pub fn clear_interrupts(&self) {
        self._gpio.status_w1tc.write_bits(u32::MAX);
        self._gpio.status1_w1tc.write_bits(u32::MAX);
    }

    /// Obtain a handle to GPIO pin `N`.
    ///
    /// # Panics
    ///
    /// Compile-time error if `N >= 54`.
    #[inline]
    pub fn pin<const N: u32>(&self) -> Pin<N> {
        const { assert!(N < GPIO_COUNT, "GPIO pin number must be 0-53") };
        Pin {
            _marker: core::marker::PhantomData,
        }
    }

    /// Release the underlying peripherals.
    #[inline]
    pub fn release(self) -> (GPIO, IO_MUX) {
        (self._gpio, self._io_mux)
    }
}
