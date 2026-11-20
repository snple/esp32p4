//! # esp32p4-usb
//!
//! USB 2.0 device and host stack for the ESP32-P4's two on-chip OTG
//! controllers.
//!
//! Everything above the peripheral driver — the device stack (bus trait, control
//! pipe, state machine, classes) and the host stack (enumeration, control
//! transfers, bulk and interrupt pipes, HID) — lives in the shared
//! [`usb-stack`](https://docs.rs/usb-stack) crate and is re-exported here; this
//! crate adds the Synopsys DesignWare OTG drivers for both directions.
//!
//! ```text
//!   device mode                        host mode
//!   ────────────────────────           ────────────────────────
//!   class::cdc_acm, UsbClass           host::class::hid::Keyboard
//!   device::UsbDevice                  host::Device / host::Pipe
//!   bus::UsbBus                        host::HostController
//!   ────────────────────────           ────────────────────────
//!   (usb-stack, re-exported)           (usb-stack, re-exported)
//!   UsbFs / UsbHs                      UsbHostFs / UsbHostHs
//!   ────────────────────────           ────────────────────────
//!   (this crate)                       (this crate)
//! ```
//!
//! Both directions are fully synchronous: there is no interrupt handler, no DMA
//! and no `async`. [`UsbDevice::poll`](usb_stack::device::UsbDevice::poll) moves
//! packets in and out of the controller's FIFOs and dispatches them to the
//! classes, so it has to be called from the main loop every few milliseconds;
//! the host driver blocks in [`Host::enumerate`](host::Host::enumerate) and in
//! the pipe transfers until the transaction completes.
//!
//! A core belongs to one direction at a time: the device and host drivers share
//! the "core taken" flag, so initialising both panics.
//!
//! # The two OTG cores
//!
//! | Core | Base | Speed | PHY | Drivers |
//! |------|------|-------|-----|---------|
//! | `USB_OTG11` | `0x5004_0000` | 12 Mbit/s | internal full-speed PHY, D+/D- on GPIO27/26 | [`UsbFs`] / [`UsbHostFs`] |
//! | `USB_OTG20` | `0x5000_0000` | 480 Mbit/s | internal UTMI+ PHY, no GPIOs | [`UsbHs`] / [`UsbHostHs`] |
//!
//! Both are Synopsys DesignWare OTG controllers driven in **slave mode**: the
//! CPU pushes and pops every packet through the FIFOs, there is no DMA and no
//! interrupt.
//!
//! ## Full-speed port
//!
//! The full-speed core has no pads of its own: it is wired to one of the chip's
//! two full-speed PHYs through the GPIO matrix, and it needs five OTG status
//! inputs (`IDDIG`, `AVALID`, `BVALID`, `VBUSVALID`, `SESSEND`). Both drivers
//! force those inputs to a constant session — B-device for the peripheral role,
//! A-device for the host role — since the board has no OTG wiring.
//!
//! By default the OTG FS core uses PHY 2 (GPIO26/27) while the USB Serial/JTAG
//! controller uses PHY 1 (GPIO24/25). If an application uses both ports it can
//! swap them with `esp32p4_hal::usb_serial_jtag::UsbSerialJtag::select_phy`; these
//! drivers follow whichever PHY the swap logic assigns, they do not touch it.
//!
//! ## High-speed port
//!
//! The high-speed core drives an internal UTMI+ PHY, so no GPIOs are involved.
//! `HP_SYS.OTG_SUSPENDM` has to be set or the PHY never reports a disconnect,
//! and the 15 kΩ pull-downs on D+/D− have to follow the direction: connected in
//! host mode, disconnected in device mode. From chip revision 3.0 they are
//! software controlled through `LP_SYS` (see [`resources::HsResources`]);
//! earlier revisions leave them to the OTG core, which ignores the write.
//!
//! ## Features
//!
//! - Both controllers, both directions: full speed (12 Mbit/s, GPIO26/27) and
//!   high speed (480 Mbit/s, internal UTMI+ PHY)
//! - Device: standard device requests, string/configuration descriptors,
//!   composite devices with interface association descriptors
//! - Device: [`class::cdc_acm::CdcAcm`] virtual serial port (64-byte packets on
//!   the FS core, 512-byte packets on the HS core), HID, GS_USB, i2c-tiny-usb
//!   and UAC 1.0 classes
//! - Device: endpoint handles for custom classes (`bulk`, `interrupt`,
//!   `isochronous`)
//! - Host: root port reset and enumeration at low, full and high speed, control,
//!   bulk and interrupt transfers, descriptor parsing, a HID boot keyboard
//!   driver ([`host::class::hid::Keyboard`]) and hubs
//!   ([`host::class::hub::Hub`])
//!
//! ## Hosts and hubs
//!
//! Both cores enumerate a hub and the devices behind it. The full-speed core is
//! the simple case: it already runs at FS, so a FS hub and its FS/LS devices
//! need no special handling beyond the low-speed signalling bit the driver sets
//! for a LS device behind a hub.
//!
//! The high-speed core has no transaction translator in this driver (and the
//! IP does not document the split-transaction register), so a FS/LS device
//! behind a high-speed hub cannot be reached. Either put the hub on the
//! full-speed port, or power the high-speed core with
//! [`UsbHostHs::new_fsls_only`]: it then never chirps, the hub enumerates at
//! full speed, and downstream FS/LS devices work without splits. High-speed
//! devices behind a high-speed hub work as long as the core runs at high speed.
//!
//! ## Example
//!
//! ```ignore
//! use esp32p4_hal::main;
//! use esp32p4_hal::pac::Peripherals;
//! use esp32p4_usb::class::cdc_acm::CdcAcm;
//! use esp32p4_usb::{FsResources, UsbFs};
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

// ── Shared stack (usb-stack) ─────────────────────────────────────────────────

pub use usb_stack::{BULK_MAX_PACKET_SIZE, CONTROL_MAX_PACKET_SIZE};
pub use usb_stack::{descriptor, endpoint, request};

// Device side: bus trait, control pipe, state machine and classes.
pub use usb_stack::device::{
    DescriptorWriter, DeviceInfo, DeviceState, Direction, Endpoint, EndpointAddress, EndpointIn,
    EndpointOut, EndpointType, In, InterfaceNumber, IsoSynchronization, IsoUsage, Out, PollResult,
    StringIndex, UsbBus, UsbBusAllocator, UsbClass, UsbDevice, UsbDeviceBuilder, UsbError,
    UsbResult,
};
pub use usb_stack::device::{bus, class, control, error};
pub use usb_stack::request::{Feature, Recipient, Request, RequestType, StandardRequest};

// Host side: port/channel trait, enumeration, pipes and the HID keyboard.
pub use usb_stack::host;

// ── ESP32-P4 implementation ───────────────────────────────────────────────────

// DesignWare OTG register map, shared by both directions.
mod dwc;
// Clock, PHY and pad plumbing, shared by both drivers.
pub mod resources;
// The drivers. The protocol layers they implement are the `device` and `host`
// modules re-exported above, so these stay private and only their types are
// exported from the crate root.
mod device_driver;
mod host_driver;

pub use device_driver::{
    MAX_ENDPOINT_PACKET_SIZE, MAX_SUPPORTED_ENDPOINTS, OUT_STAGING_BYTES, UsbFs, UsbHs,
};
pub use host_driver::{UsbHostFs, UsbHostHs};
pub use resources::{FsResources, HsResources, USB_OTG_FS_BASE, USB_OTG_HS_BASE};

/// Maximum packet size of a full-speed bulk or interrupt endpoint.
pub const FS_MAX_PACKET_SIZE: u16 = 64;

/// Maximum packet size of a high-speed bulk endpoint.
pub const HS_MAX_PACKET_SIZE: u16 = 512;
