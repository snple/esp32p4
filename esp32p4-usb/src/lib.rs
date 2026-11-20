//! # esp32p4-usb
//!
//! USB 2.0 device stack for the ESP32-P4's two on-chip OTG controllers.
//!
//! The MCU-independent part of the stack (bus trait, control pipe, device
//! state machine, CDC-ACM and HID classes) lives in the shared
//! [`usb-stack`](https://docs.rs/usb-stack) crate and is re-exported here;
//! this crate adds the Synopsys DesignWare OTG driver.
//!
//! ```text
//!   application / class          class::cdc_acm, your own UsbClass
//!   ────────────────────────     (usb-stack)
//!   device::UsbDevice            enumeration, standard requests, EP0
//!   ────────────────────────     (usb-stack)
//!   bus::UsbBus                  trait: allocate, poll, read, write
//!   ────────────────────────     (usb-stack)
//!   p4::UsbFs / p4::UsbHs        DWC OTG slave-mode driver (this crate)
//! ```
//!
//! The stack is fully synchronous: there is no interrupt handler, no DMA and no
//! `async`. [`UsbDevice::poll`](usb_stack::device::UsbDevice::poll) moves
//! packets in and out of the controller's FIFOs and dispatches them to the
//! classes, so it has to be called from the main loop every few milliseconds.
//!
//! ## Features
//!
//! - Both controllers: full speed (12 Mbit/s, GPIO26/27) and high speed
//!   (480 Mbit/s, internal UTMI+ PHY)
//! - Standard device requests, string/configuration descriptors, composite
//!   devices with interface association descriptors
//! - [`class::cdc_acm::CdcAcm`] virtual serial port (64-byte packets on the FS
//!   core, 512-byte packets on the HS core)
//! - Endpoint handles for custom classes (`bulk`, `interrupt`, `isochronous`)
//!
//! ## Example
//!
//! ```ignore
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
//!     let mut serial = CdcAcm::new(&allocator);
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

/// Logging facade (re-exported from [`usb_stack`]).
pub use usb_stack::log;

// ── Shared stack ──────────────────────────────────────────────────────────────

pub use usb_stack::{bus, class, control, descriptor, device, endpoint, error};

pub use usb_stack::{
    BULK_MAX_PACKET_SIZE, CONTROL_MAX_PACKET_SIZE, DescriptorWriter, DeviceInfo, DeviceState,
    Direction, Endpoint, EndpointAddress, EndpointIn, EndpointOut, EndpointType, In,
    InterfaceNumber, IsoSynchronization, IsoUsage, Out, PollResult, StringIndex, UsbBus,
    UsbBusAllocator, UsbClass, UsbDevice, UsbDeviceBuilder, UsbError, UsbResult,
};

// ── ESP32-P4 implementation ───────────────────────────────────────────────────

mod dwc;
pub mod p4;

pub use p4::{FsResources, HsResources, UsbFs, UsbHs};

/// Maximum packet size of a full-speed bulk or interrupt endpoint.
pub const FS_MAX_PACKET_SIZE: u16 = 64;

/// Maximum packet size of a high-speed bulk endpoint.
pub const HS_MAX_PACKET_SIZE: u16 = 512;
