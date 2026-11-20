//! # esp32p4-hal
//!
//! Hardware Abstraction Layer for the ESP32-P4 RISC-V microcontroller.
//!
//! ## Features
//!
//! - `rt` - Link the `esp32p4-rt` startup code and trap handler
//! - `panic-handler` - Provide the default `#[panic_handler]` (disable it if
//!   the application supplies its own)
//! - `defmt` - `defmt::Format` implementations and log statements
//! - `defmt-usb` - Also register the USB Serial/JTAG port as the global
//!   `defmt` logger, so `espflash monitor` shows the logs
//! - `net` - Synchronous TCP/IP over the EMAC (`smoltcp`), heap-free
//! - `net-alloc` - Same, with the socket table allocated from the global
//!   allocator
//! - `net-ipv6` - Add IPv6, SLAAC and fragmentation to `net`
//!
//! ## Panic And Logging
//!
//! `panic-handler` is enabled by default. On a panic it first disables machine
//! interrupts:
//!
//! - With `defmt` enabled, the message is emitted through the active `defmt`
//!   transport selected by the binary, such as `defmt-rtt` or the USB
//!   Serial/JTAG logger.
//! - Without `defmt`, no formatted output is attempted.
//!
//! The core then executes `ebreak`. A debugger halts at the panic site; on a
//! board with no debugger the breakpoint takes the runtime's exception path,
//! which stores `mcause`/`mepc` in [`rt::TRAP_CAUSE`] / [`rt::TRAP_EPC`] and
//! parks the core. A reentrant panic, for example from the logger while the
//! message is being formatted, skips the second print instead of recursing.
//!
//! ## Example
//!
//! ```ignore
//! #![no_std]
//! #![no_main]
//!
//! use esp32p4_hal::gpio::Pins;
//! use esp32p4_hal::main;
//! use esp32p4_hal::pac::Peripherals;
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

// The `net-alloc` socket table and the PSRAM allocator documentation both build
// on `alloc`; nothing else in the crate requires a heap.
#[cfg(feature = "net-alloc")]
extern crate alloc;

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
///
/// [`timer::Timer00`] and [`systimer::Alarm1`] follow the same rule: they take
/// `&self` everywhere, so a stolen copy can run the whole register-level API
/// from an interrupt. What `steal()` never gives access to is a method that
/// takes `&mut self`.
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

pub use esp32p4_macro::{main, ramfunc, tcmdata, tcmfunc};
pub use esp32p4_pac as pac;
pub use esp32p4_pac::{Interrupt, Peripherals};
pub use esp32p4_rt as rt;

pub mod adc;
pub mod cache;
pub mod clock;
pub mod delay;
pub mod dma;
pub mod efuse;
pub mod emac;
pub mod gpio;
pub mod i2c;
pub mod interrupt;
pub mod ledc;
pub mod mmu;
#[cfg(feature = "net")]
pub mod net;
pub mod psram;
pub mod regi2c;
pub mod reset;
pub mod rmt;
pub mod sdhost;
pub mod spi;
pub mod spin;
pub mod startup;
pub mod system;
pub mod systimer;
pub mod tcm;
pub mod time;
pub mod timer;
pub mod tsens;
pub mod uart;
pub mod usb_serial_jtag;
