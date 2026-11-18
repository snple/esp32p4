//! # esp32p4
//!
//! Hardware Abstraction Layer for the ESP32-P4 RISC-V microcontroller.
//!
//! ## Features
//!
//! - `rt` - Link the `esp32p4-rt` startup code and trap handler
//! - `panic-handler` - Provide a default `#[panic_handler]` that parks the
//!   core (disable it if the application supplies its own)
//!
//! ## Example
//!
//! ```ignore
//! #![no_std]
//! #![no_main]
//!
//! use esp32p4::gpio::Pins;
//! use esp32p4::main;
//! use esp32p4::pac::Peripherals;
//!
//! #[main]
//! fn main() -> ! {
//!     let dp = Peripherals::take().unwrap();
//!     let pins = Pins::init(dp.GPIO, dp.IO_MUX, dp.HP_SYS_CLKRST);
//!     let mut led = pins.pin::<2>();
//!     led.into_output();
//!
//!     loop {
//!         led.toggle();
//!     }
//! }
//! ```

#![no_std]

#[cfg(feature = "panic-handler")]
mod panic_handler;

// ============================================================================
// Steal wrapper — returned by `steal()` methods across HAL drivers
// ============================================================================

/// Zero-cost wrapper returned by `steal()` methods.
///
/// Implements [`Deref`](core::ops::Deref) but **not** `DerefMut`, so the caller
/// can only obtain `&T` — preventing access to `&mut self` methods (such as the
/// pin configuration methods on [`gpio::Pin`]).
///
/// ## Example
///
/// ```ignore
/// let pin = unsafe { Pin::<25>::steal() }; // Steal<Pin<25>>
/// pin.set_high();                          // &self — OK via Deref
/// pin.into_output();                       // &mut self — compile error
/// ```
pub struct Steal<T>(pub(crate) T);

impl<T> core::ops::Deref for Steal<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> AsRef<T> for Steal<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

pub use pastey;

pub use esp32p4_macro::main;
pub use esp32p4_pac as pac;
pub use esp32p4_pac::{Interrupt, Peripherals};
pub use esp32p4_rt as rt;

pub mod delay;
pub mod gpio;
pub mod psram;
pub mod startup;
pub mod system;
