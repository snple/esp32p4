//! # esp32p4
//!
//! Hardware Abstraction Layer for the ESP32-P4 RISC-V microcontroller.
//!
//! ## Features
//!
//! - `rt` - Link the `esp32p4-rt` startup code and trap handler
//! - `panic-handler` - Provide a default `#[panic_handler]` that parks the
//!   core (disable it if the application supplies its own)
//! - `defmt` - `defmt::Format` implementations and log statements
//! - `defmt-usb-logger` - Register the USB Serial/JTAG port as the global
//!   `defmt` logger, so `espflash monitor` shows the logs
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
//!     let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
//!     let mut led = pins.pin::<2>();
//!     led.into_output();
//!
//!     loop {
//!         led.toggle();
//!     }
//! }
//! ```

#![no_std]

// ============================================================================
// Logging
// ============================================================================

/// Logs a message when the `defmt` feature is enabled.
///
/// The statements expand to nothing otherwise, so hot paths can be annotated
/// without a `#[cfg]` at every call site.
macro_rules! info {
    ($($arg:tt)*) => {
        #[cfg(feature = "defmt")]
        ::defmt::info!($($arg)*);
    };
}

macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "defmt")]
        ::defmt::debug!($($arg)*);
    };
}

macro_rules! warn {
    ($($arg:tt)*) => {
        #[cfg(feature = "defmt")]
        ::defmt::warn!($($arg)*);
    };
}

macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(feature = "defmt")]
        ::defmt::error!($($arg)*);
    };
}

#[cfg(feature = "panic-handler")]
mod panic_handler;

// ============================================================================
// `critical-section` implementation
// ============================================================================

/// `critical-section` backend: mask machine interrupts.
///
/// The token remembers whether interrupts were enabled, so nested critical
/// sections behave correctly; only `mstatus.MIE` is touched, leaving the FPU
/// state and the rest of `mstatus` alone.
#[cfg(feature = "critical-section-impl")]
struct SingleCoreCriticalSection;

#[cfg(feature = "critical-section-impl")]
critical_section::set_impl!(SingleCoreCriticalSection);

#[cfg(feature = "critical-section-impl")]
unsafe impl critical_section::Impl for SingleCoreCriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        use esp32p4_rt::register::mstatus;

        let was_enabled = mstatus::mie();
        mstatus::clear_mie();
        was_enabled
    }

    unsafe fn release(restore_state: critical_section::RawRestoreState) {
        use esp32p4_rt::register::mstatus;

        if restore_state {
            // SAFETY: interrupts were enabled when the section started.
            unsafe { mstatus::set_mie() };
        }
    }
}

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

pub mod cache;
pub mod delay;
pub mod emac;
pub mod gpio;
pub mod psram;
pub mod regi2c;
pub mod startup;
pub mod system;
pub mod usb_serial_jtag;
