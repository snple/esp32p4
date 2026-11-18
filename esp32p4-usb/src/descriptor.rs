//! USB descriptor construction.
//!
//! [`DescriptorWriter`] appends descriptors to a caller-provided buffer and
//! back-patches the configuration header (`wTotalLength`, `bNumInterfaces`) when
//! [`end_configuration`](DescriptorWriter::end_configuration) is called. Classes
//! use it from
//! [`UsbClass::configuration_descriptors`](crate::class::UsbClass::configuration_descriptors).
//!
//! # Example
//!
//! ```ignore
//! let mut buf = [0u8; 64];
//! let mut w = DescriptorWriter::new(&mut buf);
//! w.configuration(0x80, 50)?;             // bus powered, 100 mA
//! w.interface(iface, 1, class_code::CDC, 0x02, 0x00)?;
//! w.endpoint(EndpointAddress::in_(1), EndpointType::Interrupt, 8, 16)?;
//! w.end_configuration();
//! ```

use crate::bus::{InterfaceNumber, StringIndex};
use crate::endpoint::{EndpointAddress, EndpointType};
use crate::error::{UsbError, UsbResult};

/// Standard descriptor type codes.
pub mod descriptor_type {
    pub const DEVICE: u8 = 1;
    pub const CONFIGURATION: u8 = 2;
    pub const STRING: u8 = 3;
    pub const INTERFACE: u8 = 4;
    pub const ENDPOINT: u8 = 5;
    pub const DEVICE_QUALIFIER: u8 = 6;
    pub const OTHER_SPEED_CONFIGURATION: u8 = 7;
    pub const INTERFACE_ASSOCIATION: u8 = 11;
    pub const BOS: u8 = 15;
    pub const DEVICE_CAPABILITY: u8 = 16;
}

/// Language identifiers (ISO 639-1 packed as USB LANGID).
pub mod lang_id {
    /// English (United States).
    pub const EN_US: u16 = 0x0409;
}

/// Common interface class codes.
pub mod class_code {
    /// Device class is defined per interface.
    pub const NONE: u8 = 0x00;
    /// Communications Device Class.
    pub const CDC: u8 = 0x02;
    /// CDC data interface.
    pub const CDC_DATA: u8 = 0x0A;
    /// Human Interface Device.
    pub const HID: u8 = 0x03;
    /// Miscellaneous device (used with interface association descriptors).
    pub const MISC: u8 = 0xEF;
    /// Vendor specific.
    pub const VENDOR: u8 = 0xFF;
}

/// CDC subclass codes.
pub mod cdc_subclass {
    /// Abstract Control Model.
    pub const ACM: u8 = 0x02;
}

/// CDC protocol codes.
pub mod cdc_protocol {
    /// No class-specific protocol.
    pub const NONE: u8 = 0x00;
    /// AT commands (Hayes compatible).
    pub const AT_COMMANDS: u8 = 0x01;
}

/// CDC class-specific descriptor subtypes.
pub mod cdc_descriptor {
    /// Descriptor type for class-specific interface descriptors.
    pub const CS_INTERFACE: u8 = 0x24;
    /// Header functional descriptor.
    pub const HEADER: u8 = 0x00;
    /// Call management functional descriptor.
    pub const CALL_MANAGEMENT: u8 = 0x01;
    /// Abstract control management functional descriptor.
    pub const ACM: u8 = 0x02;
    /// Union functional descriptor.
    pub const UNION: u8 = 0x06;
}

/// Cursor over a descriptor buffer.
pub struct DescriptorWriter<'a> {
    buf: &'a mut [u8],
    pos: usize,
    /// Offset of `wTotalLength` in the open configuration descriptor.
    total_len_pos: Option<usize>,
    /// Offset of `bNumInterfaces` in the open configuration descriptor.
    num_interfaces_pos: Option<usize>,
    /// Interfaces written for the open configuration.
    num_interfaces: u8,
}

impl<'a> DescriptorWriter<'a> {
    /// Start writing at the beginning of `buf`.
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self {
            buf,
            pos: 0,
            total_len_pos: None,
            num_interfaces_pos: None,
            num_interfaces: 0,
        }
    }

    /// Number of bytes written so far.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Append a descriptor of `descriptor_type` whose body is produced by `body`.
    ///
    /// `body` receives the remaining buffer space and returns the body length;
    /// the two header bytes (`bLength`, `bDescriptorType`) are prepended
    /// automatically. Bodies are limited to 254 bytes by the `bLength` field.
    pub fn write_with(
        &mut self,
        descriptor_type: u8,
        body: impl FnOnce(&mut [u8]) -> UsbResult<usize>,
    ) -> UsbResult<()> {
        if self.pos + 2 > self.buf.len() {
            return Err(UsbError::BufferOverflow);
        }
        let body_start = self.pos + 2;
        let body_end = (body_start + 254).min(self.buf.len());
        let body_len = body(&mut self.buf[body_start..body_end])?;
        let total = body_len + 2;
        if total > 255 || self.pos + total > self.buf.len() {
            return Err(UsbError::BufferOverflow);
        }
        self.buf[self.pos] = total as u8;
        self.buf[self.pos + 1] = descriptor_type;
        self.pos += total;
        Ok(())
    }

    /// Append a descriptor with a fixed body.
    pub fn write(&mut self, descriptor_type: u8, body: &[u8]) -> UsbResult<()> {
        self.write_with(descriptor_type, |buf| {
            if body.len() > buf.len() {
                return Err(UsbError::BufferOverflow);
            }
            buf[..body.len()].copy_from_slice(body);
            Ok(body.len())
        })
    }

    /// Start the configuration descriptor.
    ///
    /// `attributes` holds `bmAttributes` (bit 7 reserved-set, bit 6 self powered,
    /// bit 5 remote wake-up) and `max_power` is the bus current in 2 mA units.
    /// The total length and interface count are filled in by
    /// [`end_configuration`](Self::end_configuration).
    pub fn configuration(&mut self, attributes: u8, max_power: u8) -> UsbResult<()> {
        self.total_len_pos = Some(self.pos + 2);
        self.num_interfaces_pos = Some(self.pos + 4);
        self.num_interfaces = 0;
        self.write(
            descriptor_type::CONFIGURATION,
            &[
                0, // wTotalLength, patched later
                0, 0, // bNumInterfaces, patched later
                1, // bConfigurationValue
                0, // iConfiguration
                attributes, max_power,
            ],
        )
    }

    /// Close the configuration descriptor and patch its header.
    pub fn end_configuration(&mut self) {
        if let (Some(total_pos), Some(count_pos)) = (self.total_len_pos, self.num_interfaces_pos) {
            let total = self.pos as u16;
            self.buf[total_pos] = total as u8;
            self.buf[total_pos + 1] = (total >> 8) as u8;
            self.buf[count_pos] = self.num_interfaces;
        }
        self.total_len_pos = None;
        self.num_interfaces_pos = None;
    }

    /// Append an Interface Association Descriptor.
    ///
    /// Composite devices group a function's interfaces with an IAD so that the
    /// host binds one driver to all of them.
    pub fn interface_association(
        &mut self,
        first_interface: InterfaceNumber,
        interface_count: u8,
        function_class: u8,
        function_subclass: u8,
        function_protocol: u8,
    ) -> UsbResult<()> {
        self.write(
            descriptor_type::INTERFACE_ASSOCIATION,
            &[
                first_interface.into(),
                interface_count,
                function_class,
                function_subclass,
                function_protocol,
                0, // iFunction
            ],
        )
    }

    /// Append an Interface descriptor and count it in `bNumInterfaces`.
    pub fn interface(
        &mut self,
        number: InterfaceNumber,
        num_endpoints: u8,
        class: u8,
        subclass: u8,
        protocol: u8,
    ) -> UsbResult<()> {
        self.interface_with_string(
            number,
            num_endpoints,
            class,
            subclass,
            protocol,
            StringIndex::NONE,
        )
    }

    /// Append an Interface descriptor with an `iInterface` string index.
    pub fn interface_with_string(
        &mut self,
        number: InterfaceNumber,
        num_endpoints: u8,
        class: u8,
        subclass: u8,
        protocol: u8,
        string: StringIndex,
    ) -> UsbResult<()> {
        self.num_interfaces = self.num_interfaces.saturating_add(1);
        self.write(
            descriptor_type::INTERFACE,
            &[
                number.into(),
                0, // bAlternateSetting
                num_endpoints,
                class,
                subclass,
                protocol,
                string.into(),
            ],
        )
    }

    /// Append an Interface descriptor for a non-zero alternate setting.
    ///
    /// Alternate settings share the interface number, so this does *not*
    /// increment `bNumInterfaces`.
    pub fn interface_alt(
        &mut self,
        number: InterfaceNumber,
        alternate_setting: u8,
        num_endpoints: u8,
        class: u8,
        subclass: u8,
        protocol: u8,
    ) -> UsbResult<()> {
        self.write(
            descriptor_type::INTERFACE,
            &[
                number.into(),
                alternate_setting,
                num_endpoints,
                class,
                subclass,
                protocol,
                0, // iInterface
            ],
        )
    }

    /// Append an Endpoint descriptor.
    pub fn endpoint(
        &mut self,
        address: EndpointAddress,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> UsbResult<()> {
        self.write(
            descriptor_type::ENDPOINT,
            &[
                address.raw(),
                ep_type.bm_attributes(),
                max_packet_size as u8,
                (max_packet_size >> 8) as u8,
                interval,
            ],
        )
    }

    /// Encode `s` as a USB string descriptor in `buf`.
    ///
    /// Returns the number of bytes written. Characters outside the BMP are
    /// replaced with U+FFFD, as the descriptor is UTF-16LE.
    pub fn write_string(buf: &mut [u8], s: &str) -> UsbResult<usize> {
        if buf.len() < 2 {
            return Err(UsbError::BufferOverflow);
        }
        let mut len = 0;
        for c in s.chars().take((buf.len() - 2) / 2) {
            let unit = if (c as u32) <= 0xffff {
                c as u16
            } else {
                0xfffd
            };
            buf[2 + len * 2] = unit as u8;
            buf[3 + len * 2] = (unit >> 8) as u8;
            len += 1;
        }
        let total = 2 + len * 2;
        buf[0] = total as u8;
        buf[1] = descriptor_type::STRING;
        Ok(total)
    }

    /// Encode the language-ID descriptor (string index 0) in `buf`.
    pub fn write_lang_ids(buf: &mut [u8], ids: &[u16]) -> UsbResult<usize> {
        let total = 2 + ids.len() * 2;
        if total > buf.len() {
            return Err(UsbError::BufferOverflow);
        }
        buf[0] = total as u8;
        buf[1] = descriptor_type::STRING;
        for (i, id) in ids.iter().enumerate() {
            buf[2 + i * 2] = *id as u8;
            buf[3 + i * 2] = (*id >> 8) as u8;
        }
        Ok(total)
    }
}
