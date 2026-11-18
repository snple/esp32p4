//! CDC ACM (Communications Device Class, Abstract Control Model).
//!
//! Exposes a virtual serial port. The host sees two interfaces — a communication
//! interface with a notification endpoint and a data interface with a bulk IN
//! and a bulk OUT endpoint — optionally grouped by an interface association
//! descriptor so that a composite device still binds one driver.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4_usb::class::cdc_acm::CdcAcm;
//! use esp32p4_usb::{UsbBusAllocator, UsbDeviceBuilder};
//!
//! let allocator = UsbBusAllocator::new(usb);
//! let mut serial = CdcAcm::new(&allocator, 64);
//! let mut device = UsbDeviceBuilder::new(&allocator, 0x303a, 0x1001)
//!     .product("ESP32-P4 serial")
//!     .composite()
//!     .build();
//!
//! loop {
//!     if device.poll(&mut [&mut serial]) {
//!         let mut buf = [0u8; 64];
//!         if let Ok(n) = serial.read(&mut buf) {
//!             serial.write(&buf[..n]).ok();
//!         }
//!     }
//! }
//! ```

use crate::bus::{InterfaceNumber, UsbBus, UsbBusAllocator};
use crate::class::UsbClass;
use crate::control::{ControlIn, ControlOut, Recipient, RequestType};
use crate::descriptor::{DescriptorWriter, cdc_descriptor, cdc_protocol, cdc_subclass, class_code};
use crate::endpoint::{EndpointIn, EndpointOut};
use crate::error::{UsbError, UsbResult};

/// CDC class requests (`bRequest`).
mod request {
    /// Set the line coding (7 bytes, host to device).
    pub const SET_LINE_CODING: u8 = 0x20;
    /// Read the line coding (7 bytes, device to host).
    pub const GET_LINE_CODING: u8 = 0x21;
    /// Set DTR and RTS.
    pub const SET_CONTROL_LINE_STATE: u8 = 0x22;
    /// Send a break condition.
    pub const SEND_BREAK: u8 = 0x23;
}

/// Serial line parameters (the 7-byte CDC line-coding structure).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LineCoding {
    /// Bits per second.
    pub baud_rate: u32,
    /// Stop bits: 0 = 1, 1 = 1.5, 2 = 2.
    pub stop_bits: u8,
    /// Parity: 0 = none, 1 = odd, 2 = even, 3 = mark, 4 = space.
    pub parity: u8,
    /// Data bits: 5, 6, 7, 8 or 16.
    pub data_bits: u8,
}

impl Default for LineCoding {
    fn default() -> Self {
        Self {
            baud_rate: 115_200,
            stop_bits: 0,
            parity: 0,
            data_bits: 8,
        }
    }
}

impl LineCoding {
    fn to_bytes(self) -> [u8; 7] {
        let baud = self.baud_rate.to_le_bytes();
        [
            baud[0],
            baud[1],
            baud[2],
            baud[3],
            self.stop_bits,
            self.parity,
            self.data_bits,
        ]
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 7 {
            return None;
        }
        Some(Self {
            baud_rate: u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            stop_bits: bytes[4],
            parity: bytes[5],
            data_bits: bytes[6],
        })
    }
}

/// A CDC ACM virtual serial port.
pub struct CdcAcm<'a, B: UsbBus> {
    comm_if: InterfaceNumber,
    data_if: InterfaceNumber,
    notification: EndpointIn<'a, B>,
    bulk_in: EndpointIn<'a, B>,
    bulk_out: EndpointOut<'a, B>,
    line_coding: LineCoding,
    dtr: bool,
    rts: bool,
    composite: bool,
}

impl<'a, B: UsbBus> CdcAcm<'a, B> {
    /// Allocate the interfaces and endpoints of one virtual serial port.
    ///
    /// `max_packet_size` is the bulk packet size: 64 for full speed, up to 512
    /// for high speed.
    pub fn new(allocator: &'a UsbBusAllocator<B>, max_packet_size: u16) -> Self {
        Self {
            comm_if: allocator.interface(),
            data_if: allocator.interface(),
            notification: allocator.interrupt(8, 16),
            bulk_in: allocator.bulk(max_packet_size),
            bulk_out: allocator.bulk(max_packet_size),
            line_coding: LineCoding::default(),
            dtr: false,
            rts: false,
            composite: false,
        }
    }

    /// Add an interface association descriptor, for use in a composite device.
    ///
    /// Call this before [`UsbDeviceBuilder::build`](crate::device::UsbDeviceBuilder::build).
    pub fn with_interface_association(mut self) -> Self {
        self.composite = true;
        self
    }

    /// The communication interface number.
    #[inline]
    pub fn communication_interface(&self) -> InterfaceNumber {
        self.comm_if
    }

    /// The data interface number.
    #[inline]
    pub fn data_interface(&self) -> InterfaceNumber {
        self.data_if
    }

    /// The line coding the host selected last.
    #[inline]
    pub fn line_coding(&self) -> LineCoding {
        self.line_coding
    }

    /// Whether the host raised DTR (a terminal opened the port).
    #[inline]
    pub fn dtr(&self) -> bool {
        self.dtr
    }

    /// Whether the host raised RTS.
    #[inline]
    pub fn rts(&self) -> bool {
        self.rts
    }

    /// Read one packet from the host.
    ///
    /// Returns [`UsbError::WouldBlock`] when nothing has arrived; the endpoint
    /// stays armed, so the data is not lost.
    pub fn read(&self, buffer: &mut [u8]) -> UsbResult<usize> {
        self.bulk_out.read(buffer)
    }

    /// Queue one packet for the host.
    ///
    /// Returns [`UsbError::WouldBlock`] while the previous packet is still in
    /// flight, and [`UsbError::BufferOverflow`] when `data` is longer than the
    /// bulk packet size.
    pub fn write(&self, data: &[u8]) -> UsbResult<usize> {
        self.bulk_in.write(data)
    }

    /// Queue `data`, splitting it into endpoint-sized packets.
    ///
    /// While the endpoint is busy the call spins, spending one unit of `budget`
    /// per attempt; the budget bounds the time spent waiting for a host that is
    /// not collecting. Returns the number of bytes queued.
    pub fn write_all(&self, data: &[u8], mut budget: u32) -> usize {
        let max_packet = self.bulk_in.max_packet_size() as usize;
        let mut sent = 0;
        while sent < data.len() {
            let end = (sent + max_packet).min(data.len());
            loop {
                match self.bulk_in.write(&data[sent..end]) {
                    Ok(_) => break,
                    Err(UsbError::WouldBlock) if budget > 0 => {
                        budget -= 1;
                        core::hint::spin_loop();
                    }
                    Err(_) => return sent,
                }
            }
            sent = end;
        }
        sent
    }
}

impl<B: UsbBus> UsbClass<B> for CdcAcm<'_, B> {
    fn configuration_descriptors(&self, writer: &mut DescriptorWriter<'_>) -> UsbResult<()> {
        if self.composite {
            writer.interface_association(
                self.comm_if,
                2,
                class_code::CDC,
                cdc_subclass::ACM,
                cdc_protocol::AT_COMMANDS,
            )?;
        }

        // Communication interface: one notification endpoint.
        writer.interface(
            self.comm_if,
            1,
            class_code::CDC,
            cdc_subclass::ACM,
            cdc_protocol::AT_COMMANDS,
        )?;
        // Header functional descriptor, CDC 1.10.
        writer.write(
            cdc_descriptor::CS_INTERFACE,
            &[cdc_descriptor::HEADER, 0x10, 0x01],
        )?;
        // Call management: no call handling, data goes to the data interface.
        writer.write(
            cdc_descriptor::CS_INTERFACE,
            &[
                cdc_descriptor::CALL_MANAGEMENT,
                0x00, // bmCapabilities: no call management
                self.data_if.into(),
            ],
        )?;
        // ACM: supports Set/Get_Line_Coding and Set_Control_Line_State.
        writer.write(cdc_descriptor::CS_INTERFACE, &[cdc_descriptor::ACM, 0x02])?;
        // Union: the data interface is subordinate to the communication one.
        writer.write(
            cdc_descriptor::CS_INTERFACE,
            &[
                cdc_descriptor::UNION,
                self.comm_if.into(),
                self.data_if.into(),
            ],
        )?;
        writer.endpoint(
            self.notification.address(),
            self.notification.ep_type(),
            self.notification.max_packet_size(),
            self.notification.interval(),
        )?;

        // Data interface: bulk IN and bulk OUT.
        writer.interface(self.data_if, 2, class_code::CDC_DATA, 0x00, 0x00)?;
        writer.endpoint(
            self.bulk_in.address(),
            self.bulk_in.ep_type(),
            self.bulk_in.max_packet_size(),
            0,
        )?;
        writer.endpoint(
            self.bulk_out.address(),
            self.bulk_out.ep_type(),
            self.bulk_out.max_packet_size(),
            0,
        )?;
        Ok(())
    }

    fn control_in(&mut self, transfer: ControlIn<'_, B>) {
        let request = *transfer.request();
        if request.request_type != RequestType::Class
            || request.recipient != Recipient::Interface
            || request.index != u16::from(u8::from(self.comm_if))
        {
            return;
        }
        match request.request {
            request::GET_LINE_CODING => {
                let coding = self.line_coding.to_bytes();
                let _ = transfer.accept(|buffer| {
                    buffer[..coding.len()].copy_from_slice(&coding);
                    Ok(coding.len())
                });
            }
            _ => {
                let _ = transfer.reject();
            }
        }
    }

    fn control_out(&mut self, transfer: ControlOut<'_, B>) {
        let request = *transfer.request();
        if request.request_type != RequestType::Class
            || request.recipient != Recipient::Interface
            || request.index != u16::from(u8::from(self.comm_if))
        {
            return;
        }
        match request.request {
            request::SET_LINE_CODING => {
                if let Some(coding) = LineCoding::from_bytes(transfer.data()) {
                    self.line_coding = coding;
                    debug!("CDC: line coding {:?}", coding);
                    let _ = transfer.accept();
                } else {
                    let _ = transfer.reject();
                }
            }
            request::SET_CONTROL_LINE_STATE => {
                self.dtr = request.value & 0x0001 != 0;
                self.rts = request.value & 0x0002 != 0;
                debug!("CDC: dtr={} rts={}", self.dtr, self.rts);
                let _ = transfer.accept();
            }
            request::SEND_BREAK => {
                let _ = transfer.accept();
            }
            _ => {
                let _ = transfer.reject();
            }
        }
    }

    fn reset(&mut self) {
        self.dtr = false;
        self.rts = false;
    }
}
