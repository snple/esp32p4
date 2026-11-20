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
//! - Per-pad direct IO MUX functions from [`pins`]: the TRM's function table,
//!   including which pads the low-power domain can reach
//! - Four interrupt outputs with edge/level selection, per-line enable and
//!   status handling
//! - Light-sleep wake-up enable
//!
//! # Module Layout
//!
//! | Module | Contents |
//! |--------|----------|
//! | this module | shared types, [`Pins`], [`PinOps`], the matrix signal constants' re-exports |
//! | [`pin`] | [`Pin<N>`], generated per pad by `impl_pin!` |
//! | [`pins`] | per-pad capability table ([`PadDesc`], [`MuxFunction`]) |
//! | [`signal`] | GPIO matrix input/output signal indices |
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
//! use esp32p4_hal::gpio::{InterruptLine, InterruptType, Pin};
//! use esp32p4_hal::interrupt::{self, CpuInterrupt, Level};
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
//! use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
//!
//! tx.set_output_signal(OutputSignal::UART0_TXD_PAD_OUT);
//! rx.set_input_signal(InputSignal::UART0_RXD_PAD_IN);
//! ```

use esp32p4_pac::{GPIO, HP_SYS_CLKRST, IO_MUX, Interrupt};

pub mod pin;
pub mod pins;
pub mod signal;

pub use pin::Pin;
pub use pins::{AnalogFunction, MuxFunction, PadDesc};
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
/// Most HP peripheral signals reach a pad through the GPIO matrix, which
/// accepts every pin, so a driver usually does not need to constrain its pin
/// argument. Not everything is routable, though: the GMAC PHY/RMII data and
/// clock signals, the USB data lines and the analog functions have dedicated
/// pads, and the LP peripherals are limited to GPIO0-15. [`pins::PadDesc`]
/// records those capabilities; drivers for such peripherals check them instead
/// of relying on the matrix.
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

    /// Attach or detach the pad's pull-down.
    fn set_pull_down_enabled(&mut self, enabled: bool);

    /// Enable or disable the input buffer.
    fn set_input_buffer_enabled(&mut self, enabled: bool);

    /// Enable or disable the pad's output driver (`GPIO_ENABLE`).
    fn set_output_enabled(&mut self, enabled: bool);

    /// Switch the pad's output driver to open-drain.
    fn set_open_drain_enabled(&mut self, enabled: bool);

    /// The pad's capabilities, from the IO MUX description table.
    fn pad(&self) -> &'static pins::PadDesc;

    /// Whether the low-power domain can reach this pad.
    fn is_lp_capable(&self) -> bool;

    /// The direct IO MUX function currently selected.
    fn mux_function(&self) -> Option<pins::MuxFunction>;

    /// Selects a direct IO MUX function.
    ///
    /// # Panics
    ///
    /// Panics if the pad has no such function; see
    /// [`supports_mux_function`](PinOps::supports_mux_function).
    fn set_mux_function(&mut self, function: pins::MuxFunction);

    /// Whether the pad has the direct IO MUX function `function`.
    fn supports_mux_function(&self, function: pins::MuxFunction) -> bool;
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
        Pin::new()
    }

    /// Release the underlying peripherals.
    #[inline]
    pub fn release(self) -> (GPIO, IO_MUX) {
        (self._gpio, self._io_mux)
    }
}
