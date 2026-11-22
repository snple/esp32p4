//! Per-pin GPIO implementation.
//!
//! [`Pin<N>`] is generated for every pad by `impl_pin!`, which fixes the
//! register names at compile time: pins 0-31 use `OUT`/`ENABLE`/`IN`, pins 32+
//! use the `*1` counterparts, so no runtime table lookup is needed.
//!
//! Configuration methods take `&mut self`, I/O methods take `&self`, and the
//! pin is a zero-sized token: [`Pins::pin`](super::Pins::pin) hands out any
//! number of handles, and `unsafe { Pin::<N>::steal() }` is the escape hatch
//! for interrupt handlers.

use esp32p4_pac::{GPIO, IO_MUX};
use pastey::paste;

use super::{
    DriveStrength, Function, InputSignal, InterruptLine, InterruptType, OUTPUT_SIGNAL_GPIO,
    OutputEnable, OutputSignal, Pull, signal,
};

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
                /// - the GPIO subsystem has been initialised (via [`Pins::init`](super::Pins::init));
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

                /// What this pad can do, from the TRM's IO MUX function table.
                #[inline]
                pub fn pad(&self) -> &'static super::pins::PadDesc {
                    super::pins::pad($n).expect("GPIO pin has no IO MUX register")
                }

                /// Whether the low-power domain can reach this pad.
                #[inline]
                pub fn is_lp_capable(&self) -> bool {
                    self.pad().lp
                }

                /// The direct IO MUX function currently selected, if it has a
                /// name.
                ///
                /// The GPIO function (`MCU_SEL = 1`) reads back as
                /// [`MuxFunction::Gpio`](super::pins::MuxFunction::Gpio).
                #[inline]
                pub fn mux_function(&self) -> Option<super::pins::MuxFunction> {
                    let selector = self.function();
                    self.pad()
                        .functions()
                        .get(selector as usize)
                        .copied()
                        .flatten()
                }

                /// Selects a direct IO MUX function.
                ///
                /// This bypasses the GPIO matrix: the pad is driven by the
                /// peripheral's fixed IO MUX function, which has lower latency
                /// than a matrix route. Selecting
                /// [`MuxFunction::Gpio`](super::pins::MuxFunction::Gpio) is the
                /// same as [`Pin::set_function_gpio`].
                ///
                /// # Panics
                ///
                /// Panics if this pad has no such function; check
                /// [`Pin::supports_mux_function`] first when the pin number is
                /// not fixed.
                pub fn set_mux_function(&mut self, function: super::pins::MuxFunction) {
                    let selector = self.pad().function_index(function).unwrap_or_else(|| {
                        panic!(
                            "GPIO{} has no {} function",
                            $n,
                            function.name()
                        )
                    });
                    self.io_mux().[<gpio $n>].write_mcu_sel(selector);
                }

                /// Whether this pad has the direct IO MUX function `function`.
                #[inline]
                pub fn supports_mux_function(&self, function: super::pins::MuxFunction) -> bool {
                    self.pad().function_index(function).is_some()
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

                /// Binds this pin to one of the eight ETM event channels.
                ///
                /// The pin has to be an input, and the edge that should reach
                /// the matrix comes from [`set_interrupt_type`](Self::set_interrupt_type):
                /// `PositiveEdge` for [`EtmEvent::Rising`](super::EtmEvent::Rising),
                /// and so on. Only GPIO0-GPIO7 have an id in the matrix.
                ///
                /// # Panics
                ///
                /// Panics when this pin is not one of GPIO0-GPIO7 or the channel
                /// is out of range.
                #[inline]
                pub fn bind_etm_event(&mut self, channel: u8) {
                    assert!(
                        $n < super::ETM_PINS,
                        "the ETM matrix only carries GPIO0-GPIO{}",
                        super::ETM_PINS - 1
                    );
                    super::bind_etm_event($n, channel);
                }

                /// Binds this pin to an ETM task channel and enables the task.
                ///
                /// The channel is the one [`EtmTask::Set`](super::EtmTask::Set)
                /// and friends name; one channel may drive several pins.
                ///
                /// # Panics
                ///
                /// Panics when the channel is out of range.
                #[inline]
                pub fn bind_etm_task(&mut self, channel: u8) {
                    super::bind_etm_task($n, channel);
                }

                /// Enables or disables the ETM task on this pin.
                #[inline]
                pub fn enable_etm_task(&mut self, enable: bool) {
                    super::enable_etm_task($n, enable);
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
                fn set_pull_down_enabled(&mut self, enabled: bool) {
                    Pin::<$n>::set_pull(self, if enabled { Pull::Down } else { Pull::None });
                }

                #[inline]
                fn set_output_enabled(&mut self, enabled: bool) {
                    if enabled {
                        Pin::<$n>::set_direction_out(self);
                    } else {
                        Pin::<$n>::set_direction_in(self);
                    }
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

                #[inline]
                fn pad(&self) -> &'static super::pins::PadDesc {
                    Pin::<$n>::pad(self)
                }

                #[inline]
                fn is_lp_capable(&self) -> bool {
                    Pin::<$n>::is_lp_capable(self)
                }

                #[inline]
                fn mux_function(&self) -> Option<super::pins::MuxFunction> {
                    Pin::<$n>::mux_function(self)
                }

                #[inline]
                fn set_mux_function(&mut self, function: super::pins::MuxFunction) {
                    Pin::<$n>::set_mux_function(self, function);
                }

                #[inline]
                fn supports_mux_function(&self, function: super::pins::MuxFunction) -> bool {
                    Pin::<$n>::supports_mux_function(self, function)
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
/// Obtained from [`Pins::pin`](super::Pins::pin). The type parameter is the pin number, so the
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

impl<const N: u32> Pin<N> {
    /// Creates a zero-sized handle.
    ///
    /// Only [`Pins::pin`](super::Pins::pin) and the per-pin `steal` use this;
    /// it is `pub(super)` so the ownership story stays in one place.
    #[inline]
    pub(super) const fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }
}

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
