//! # esp32p4-usb
//!
//! USB 2.0 device stack for the ESP32-P4's two on-chip OTG controllers.
//!
//! ```text
//!   application / class          class::cdc_acm, your own UsbClass
//!   ────────────────────────
//!   device::UsbDevice            enumeration, standard requests, EP0
//!   ────────────────────────
//!   bus::UsbBus                  trait: allocate, poll, read, write
//!   ────────────────────────
//!   p4::UsbFs / p4::UsbHs        DWC OTG slave-mode driver
//! ```
//!
//! The stack is fully synchronous: there is no interrupt handler, no DMA and no
//! `async`. [`UsbDevice::poll`](device::UsbDevice::poll) moves packets in and
//! out of the controller's FIFOs and dispatches them to the classes, so it has
//! to be called from the main loop every few milliseconds.
//!
//! ## Features
//!
//! - Both controllers: full speed (12 Mbit/s, GPIO26/27) and high speed
//!   (480 Mbit/s, internal UTMI+ PHY)
//! - Standard device requests, string/configuration descriptors, composite
//!   devices with interface association descriptors
//! - [`class::cdc_acm::CdcAcm`] virtual serial port
//! - Endpoint handles for custom classes (`bulk`, `interrupt`, `isochronous`)
//!
//! ## Example
//!
//! ```ignore
//! use core::fmt::Write;
//!
//! use esp32p4_hal::main;
//! use esp32p4_hal::pac::Peripherals;
//! use esp32p4_usb::class::cdc_acm::CdcAcm;
//! use esp32p4_usb::p4::{FsResources, UsbFs};
//! use esp32p4_usb::{UsbBusAllocator, UsbDeviceBuilder};
//!
//! #[main]
//! fn main() -> ! {
//!     let dp = Peripherals::take().unwrap();
//!
//!     let allocator = UsbBusAllocator::new(UsbFs::new(FsResources {
//!         clkrst: &dp.HP_SYS_CLKRST,
//!         lp_aon_clkrst: &dp.LP_AON_CLKRST,
//!         usb_wrap: &dp.USB_WRAP,
//!         gpio: &dp.GPIO,
//!         io_mux: &dp.IO_MUX,
//!     }));
//!     let mut serial = CdcAcm::new(&allocator, 64);
//!     let mut device = UsbDeviceBuilder::new(&allocator, 0x303a, 0x1001)
//!         .product("ESP32-P4 serial")
//!         .build();
//!
//!     loop {
//!         if device.poll(&mut [&mut serial]) {
//!             let mut buffer = [0u8; 64];
//!             if let Ok(n) = serial.read(&mut buffer) {
//!                 let _ = serial.write(&buffer[..n]);
//!             }
//!         }
//!     }
//! }
//! ```

#![no_std]

// ============================================================================
// Logging
// ============================================================================

macro_rules! debug {
    ($($arg:tt)*) => {{
        #[cfg(feature = "defmt")]
        ::defmt::debug!($($arg)*);
    }};
}

macro_rules! warn {
    ($($arg:tt)*) => {{
        #[cfg(feature = "defmt")]
        ::defmt::warn!($($arg)*);
    }};
}

pub mod bus;
pub mod class;
pub mod control;
pub mod descriptor;
pub mod device;
mod dwc;
pub mod endpoint;
pub mod error;
pub mod p4;

pub use bus::{InterfaceNumber, PollResult, StringIndex, UsbBus, UsbBusAllocator};
pub use class::UsbClass;
pub use descriptor::DescriptorWriter;
pub use device::{DeviceInfo, DeviceState, UsbDevice, UsbDeviceBuilder};
pub use endpoint::{
    Direction, Endpoint, EndpointAddress, EndpointIn, EndpointOut, EndpointType, In, Out,
};
pub use error::{UsbError, UsbResult};
pub use p4::{FsResources, HsResources, UsbFs, UsbHs};

/// Maximum packet size of a full-speed bulk or interrupt endpoint.
pub const FS_MAX_PACKET_SIZE: u16 = 64;

/// Maximum packet size of a high-speed bulk endpoint.
pub const HS_MAX_PACKET_SIZE: u16 = 512;
