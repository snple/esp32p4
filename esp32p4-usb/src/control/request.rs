//! USB SETUP packet parsing.

use crate::endpoint::{Direction, EndpointAddress};
use crate::error::{UsbError, UsbResult};

/// `bmRequestType` bits 6:5 — who defined the request.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RequestType {
    /// Defined by the USB specification.
    Standard = 0,
    /// Defined by the device class.
    Class = 1,
    /// Vendor specific.
    Vendor = 2,
    /// Reserved.
    Reserved = 3,
}

impl From<u8> for RequestType {
    fn from(value: u8) -> Self {
        match (value >> 5) & 0x03 {
            0 => Self::Standard,
            1 => Self::Class,
            2 => Self::Vendor,
            _ => Self::Reserved,
        }
    }
}

/// `bmRequestType` bits 4:0 — the request's target.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Recipient {
    /// The whole device.
    Device = 0,
    /// One interface.
    Interface = 1,
    /// One endpoint.
    Endpoint = 2,
    /// Some other element.
    Other = 3,
    /// Reserved.
    Reserved = 4,
}

impl From<u8> for Recipient {
    fn from(value: u8) -> Self {
        match value & 0x1f {
            0 => Self::Device,
            1 => Self::Interface,
            2 => Self::Endpoint,
            3 => Self::Other,
            _ => Self::Reserved,
        }
    }
}

/// Standard request codes (`bRequest`).
pub mod standard_request {
    pub const GET_STATUS: u8 = 0;
    pub const CLEAR_FEATURE: u8 = 1;
    pub const SET_FEATURE: u8 = 3;
    pub const SET_ADDRESS: u8 = 5;
    pub const GET_DESCRIPTOR: u8 = 6;
    pub const SET_DESCRIPTOR: u8 = 7;
    pub const GET_CONFIGURATION: u8 = 8;
    pub const SET_CONFIGURATION: u8 = 9;
    pub const GET_INTERFACE: u8 = 10;
    pub const SET_INTERFACE: u8 = 11;
    pub const SYNCH_FRAME: u8 = 12;
}

/// Standard feature selectors (`wValue`).
pub mod feature {
    pub const ENDPOINT_HALT: u16 = 0;
    pub const DEVICE_REMOTE_WAKEUP: u16 = 1;
    pub const TEST_MODE: u16 = 2;
}

/// A parsed 8-byte SETUP packet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Request {
    /// Data-stage direction.
    pub direction: Direction,
    /// Who defined the request.
    pub request_type: RequestType,
    /// Who the request targets.
    pub recipient: Recipient,
    /// Request code.
    pub request: u8,
    /// `wValue`.
    pub value: u16,
    /// `wIndex`.
    pub index: u16,
    /// `wLength`: bytes in the data stage.
    pub length: u16,
}

impl Request {
    /// Parse a SETUP packet.
    pub fn parse(packet: &[u8]) -> UsbResult<Self> {
        if packet.len() < 8 {
            return Err(UsbError::ParseError);
        }
        let request_type = packet[0];
        Ok(Self {
            direction: Direction::from(request_type),
            request_type: RequestType::from(request_type),
            recipient: Recipient::from(request_type),
            request: packet[1],
            value: u16::from_le_bytes([packet[2], packet[3]]),
            index: u16::from_le_bytes([packet[4], packet[5]]),
            length: u16::from_le_bytes([packet[6], packet[7]]),
        })
    }

    /// `wValue` split into the descriptor type and index used by
    /// `GET_DESCRIPTOR`.
    #[inline]
    pub fn descriptor_type_index(&self) -> (u8, u8) {
        ((self.value >> 8) as u8, self.value as u8)
    }

    /// `wIndex` as an endpoint address, for endpoint-directed requests.
    #[inline]
    pub fn endpoint_address(&self) -> EndpointAddress {
        EndpointAddress::from((self.index & 0x8f) as u8)
    }
}
