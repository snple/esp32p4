//! Device state machine, standard requests and the [`UsbDeviceBuilder`].
//!
//! [`UsbDevice::poll`] is the only entry point: it drains the bus, answers the
//! standard control requests itself, forwards everything else to the classes and
//! reports whether any of them may have data to move.
//!
//! ```ignore
//! loop {
//!     if device.poll(&mut [&mut serial]) {
//!         let mut buf = [0u8; 64];
//!         if let Ok(n) = serial.read(&mut buf) {
//!             serial.write(&buf[..n]).ok();
//!         }
//!     }
//! }
//! ```

use core::cmp::min;

use crate::bus::{InterfaceNumber, PollResult, StringIndex, UsbBus};
use crate::class::UsbClass;
use crate::control::{
    ControlIn, ControlOut, ControlPipe, Recipient, Request, RequestType, feature, standard_request,
};
use crate::descriptor::{DescriptorWriter, class_code, descriptor_type, lang_id};
use crate::endpoint::{Direction, EndpointAddress, EndpointType};
use crate::error::UsbResult;

/// Where the device is in the USB enumeration state machine.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceState {
    /// Powered, default address, not configured.
    Default,
    /// The host assigned an address.
    Addressed,
    /// The host selected a configuration; endpoints are live.
    Configured,
    /// The bus is suspended.
    Suspend,
}

/// `bConfigurationValue` when no configuration is selected.
pub const CONFIGURATION_NONE: u8 = 0;
/// `bConfigurationValue` of the single configuration this stack exposes.
pub const CONFIGURATION_VALUE: u8 = 1;
/// `bAlternateSetting` used by default.
pub const DEFAULT_ALTERNATE_SETTING: u8 = 0;

/// Static device properties.
pub struct DeviceInfo<'a> {
    /// USB-IF vendor ID.
    pub vendor_id: u16,
    /// Product ID.
    pub product_id: u16,
    /// Device class (`0x00` = defined per interface).
    pub device_class: u8,
    /// Device subclass.
    pub device_sub_class: u8,
    /// Device protocol.
    pub device_protocol: u8,
    /// EP0 maximum packet size: 8, 16, 32 or 64.
    pub max_packet_size_0: u8,
    /// Device release in BCD (`0x0100` = 1.00).
    pub device_release: u16,
    /// USB specification compliance (`0x0200` = USB 2.0).
    pub usb_rev: u16,
    /// Manufacturer string.
    pub manufacturer: Option<&'a str>,
    /// Product string.
    pub product: Option<&'a str>,
    /// Serial-number string.
    pub serial_number: Option<&'a str>,
    /// Whether the device has its own power supply.
    pub self_powered: bool,
    /// Whether the device may issue remote wake-up.
    pub supports_remote_wakeup: bool,
    /// Bus current in 2 mA units.
    pub max_power: u8,
}

/// A composite USB device built from one or more [`UsbClass`]es.
pub struct UsbDevice<'a, B: UsbBus> {
    bus: &'a B,
    info: DeviceInfo<'a>,
    control: ControlPipe,
    state: DeviceState,
    pending_address: u8,
    remote_wakeup_enabled: bool,
    self_powered: bool,
    state_before_suspend: Option<DeviceState>,
}

impl<'a, B: UsbBus> UsbDevice<'a, B> {
    fn build(allocator: &'a crate::bus::UsbBusAllocator<B>, info: DeviceInfo<'a>) -> Self {
        // EP0 must be the first allocation: the controller assumes both
        // directions of endpoint 0 exist.
        allocator
            .alloc::<crate::endpoint::Out>(
                Some(EndpointAddress::EP0_OUT),
                EndpointType::Control,
                info.max_packet_size_0 as u16,
                0,
            )
            .expect("EP0 OUT allocation failed");
        allocator
            .alloc::<crate::endpoint::In>(
                Some(EndpointAddress::EP0_IN),
                EndpointType::Control,
                info.max_packet_size_0 as u16,
                0,
            )
            .expect("EP0 IN allocation failed");

        let bus = allocator.freeze();
        Self {
            bus,
            self_powered: info.self_powered,
            info,
            control: ControlPipe::new(0),
            state: DeviceState::Default,
            pending_address: 0,
            remote_wakeup_enabled: false,
            state_before_suspend: None,
        }
    }

    /// The current enumeration state.
    #[inline]
    pub fn state(&self) -> DeviceState {
        self.state
    }

    /// Whether the host selected a configuration.
    #[inline]
    pub fn is_configured(&self) -> bool {
        self.state == DeviceState::Configured
    }

    /// Borrow the underlying bus, e.g. for platform-specific operations.
    #[inline]
    pub fn bus(&self) -> &B {
        self.bus
    }

    /// Whether the host enabled remote wake-up.
    #[inline]
    pub fn remote_wakeup_enabled(&self) -> bool {
        self.remote_wakeup_enabled
    }

    /// Update the self-powered bit reported by `GET_STATUS`.
    pub fn set_self_powered(&mut self, self_powered: bool) {
        self.self_powered = self_powered;
    }

    /// Force a disconnect so the host enumerates the device again.
    pub fn force_reset(&mut self) -> UsbResult<()> {
        self.bus.force_reset()
    }

    /// Service the bus and dispatch everything that arrived.
    ///
    /// Returns `true` when a class may have data to move (a packet arrived, a
    /// packet left, or the configuration changed).
    ///
    /// Call it at least every few milliseconds: the controller's RX FIFO is
    /// small and shared by all OUT endpoints.
    pub fn poll(&mut self, classes: &mut Classes<'_, B>) -> bool {
        let event = self.bus.poll();

        if self.state == DeviceState::Suspend {
            match event {
                PollResult::Suspend | PollResult::None => return false,
                _ => {
                    self.bus.resume();
                    self.state = self
                        .state_before_suspend
                        .take()
                        .unwrap_or(DeviceState::Default);
                }
            }
        }

        match event {
            PollResult::None => false,
            PollResult::Reset => {
                self.handle_reset(classes);
                true
            }
            PollResult::Suspend => {
                self.bus.suspend();
                self.state_before_suspend = Some(self.state);
                self.state = DeviceState::Suspend;
                false
            }
            PollResult::Resume => false,
            PollResult::Data {
                ep_out,
                ep_in_complete,
                ep_setup,
            } => self.handle_data(classes, ep_out, ep_in_complete, ep_setup),
        }
    }

    fn handle_reset(&mut self, classes: &mut Classes<'_, B>) {
        self.state = DeviceState::Default;
        self.pending_address = 0;
        self.remote_wakeup_enabled = false;
        self.control = ControlPipe::new(self.info.max_packet_size_0 as u16);
        self.bus.reset();
        for class in classes.iter_mut() {
            class.reset();
        }
        debug!("USB: bus reset");
    }

    fn handle_data(
        &mut self,
        classes: &mut Classes<'_, B>,
        ep_out: u16,
        ep_in_complete: u16,
        ep_setup: u16,
    ) -> bool {
        let mut active = false;

        // EP0 first, in the order the hardware can produce the events.
        if (ep_setup | ep_out | ep_in_complete) & 1 != 0 {
            active = true;
            if ep_setup & 1 != 0 {
                if let Some(request) = self.control.handle_setup(self.bus) {
                    self.dispatch(classes, request);
                }
            } else if ep_out & 1 != 0 {
                match self.control.handle_out(self.bus) {
                    Ok(Some(request)) => self.dispatch(classes, request),
                    Ok(None) => {}
                    Err(_e) => warn!("USB: EP0 OUT failed: {}", _e),
                }
            } else {
                match self.control.handle_in_complete(self.bus) {
                    Ok(complete) => {
                        if complete && !B::SET_ADDRESS_BEFORE_STATUS && self.pending_address != 0 {
                            self.bus.set_device_address(self.pending_address);
                            self.pending_address = 0;
                            self.state = DeviceState::Addressed;
                        }
                    }
                    Err(_e) => warn!("USB: EP0 IN failed: {}", _e),
                }
            }
        }

        for number in 1..16u8 {
            let bit = 1u16 << number;
            if ep_setup & bit != 0 {
                for class in classes.iter_mut() {
                    class.endpoint_setup(EndpointAddress::out(number));
                }
                active = true;
            }
            if ep_out & bit != 0 {
                for class in classes.iter_mut() {
                    class.endpoint_out(EndpointAddress::out(number));
                }
                active = true;
            }
            if ep_in_complete & bit != 0 {
                for class in classes.iter_mut() {
                    class.endpoint_in_complete(EndpointAddress::in_(number));
                }
                active = true;
            }
        }

        for class in classes.iter_mut() {
            class.poll();
        }

        active
    }

    fn dispatch(&mut self, classes: &mut Classes<'_, B>, request: Request) {
        if request.direction == Direction::In {
            self.control_in(classes, request);
        } else {
            self.control_out(classes, request);
        }
    }

    fn control_in(&mut self, classes: &mut Classes<'_, B>, request: Request) {
        for class in classes.iter_mut() {
            class.control_in(ControlIn::new(&mut self.control, self.bus, &request));
            if !self.control.waiting_for_response() {
                return;
            }
        }

        if request.request_type != RequestType::Standard {
            if self.control.waiting_for_response() {
                let _ = self.control.reject(self.bus);
            }
            return;
        }

        match (request.recipient, request.request) {
            (Recipient::Device, standard_request::GET_STATUS) => {
                let mut status = 0u16;
                if self.self_powered {
                    status |= 0x0001;
                }
                if self.remote_wakeup_enabled {
                    status |= 0x0002;
                }
                let _ = ControlIn::new(&mut self.control, self.bus, &request)
                    .accept_with(&status.to_le_bytes());
            }
            (Recipient::Interface, standard_request::GET_STATUS) => {
                let _ = ControlIn::new(&mut self.control, self.bus, &request)
                    .accept_with(&0u16.to_le_bytes());
            }
            (Recipient::Endpoint, standard_request::GET_STATUS) => {
                let stalled = self.bus.is_stalled(request.endpoint_address());
                let status = if stalled { 0x0001u16 } else { 0 };
                let _ = ControlIn::new(&mut self.control, self.bus, &request)
                    .accept_with(&status.to_le_bytes());
            }
            (Recipient::Device, standard_request::GET_DESCRIPTOR) => {
                self.get_descriptor(classes, request);
            }
            (Recipient::Device, standard_request::GET_CONFIGURATION) => {
                let value = if self.state == DeviceState::Configured {
                    CONFIGURATION_VALUE
                } else {
                    CONFIGURATION_NONE
                };
                let _ = ControlIn::new(&mut self.control, self.bus, &request).accept_with(&[value]);
            }
            (Recipient::Interface, standard_request::GET_INTERFACE) => {
                if request.index > u8::MAX as u16 {
                    let _ = ControlIn::new(&mut self.control, self.bus, &request).reject();
                    return;
                }
                let interface = InterfaceNumber(request.index as u8);
                for class in classes.iter_mut() {
                    if let Some(alt) = class.alt_setting(interface) {
                        let _ = ControlIn::new(&mut self.control, self.bus, &request)
                            .accept_with(&[alt]);
                        return;
                    }
                }
                let _ = ControlIn::new(&mut self.control, self.bus, &request)
                    .accept_with(&[DEFAULT_ALTERNATE_SETTING]);
            }
            _ => {}
        }

        if self.control.waiting_for_response() {
            let _ = self.control.reject(self.bus);
        }
    }

    fn control_out(&mut self, classes: &mut Classes<'_, B>, request: Request) {
        for class in classes.iter_mut() {
            class.control_out(ControlOut::new(&mut self.control, self.bus, &request));
            if !self.control.waiting_for_response() {
                return;
            }
        }

        if request.request_type != RequestType::Standard {
            if self.control.waiting_for_response() {
                let _ = self.control.reject(self.bus);
            }
            return;
        }

        match (request.recipient, request.request, request.value) {
            (Recipient::Device, standard_request::CLEAR_FEATURE, feature::DEVICE_REMOTE_WAKEUP) => {
                self.remote_wakeup_enabled = false;
                let _ = ControlOut::new(&mut self.control, self.bus, &request).accept();
            }
            (Recipient::Endpoint, standard_request::CLEAR_FEATURE, feature::ENDPOINT_HALT) => {
                self.bus.set_stalled(request.endpoint_address(), false);
                let _ = ControlOut::new(&mut self.control, self.bus, &request).accept();
            }
            (Recipient::Device, standard_request::SET_FEATURE, feature::DEVICE_REMOTE_WAKEUP) => {
                self.remote_wakeup_enabled = true;
                let _ = ControlOut::new(&mut self.control, self.bus, &request).accept();
            }
            (Recipient::Endpoint, standard_request::SET_FEATURE, feature::ENDPOINT_HALT) => {
                self.bus.set_stalled(request.endpoint_address(), true);
                let _ = ControlOut::new(&mut self.control, self.bus, &request).accept();
            }
            (Recipient::Device, standard_request::SET_ADDRESS, 1..=127) => {
                let address = request.value as u8;
                if B::SET_ADDRESS_BEFORE_STATUS {
                    self.bus.set_device_address(address);
                    self.state = DeviceState::Addressed;
                } else {
                    self.pending_address = address;
                }
                let _ = ControlOut::new(&mut self.control, self.bus, &request).accept();
                debug!("USB: address {}", address);
            }
            (Recipient::Device, standard_request::SET_CONFIGURATION, _) => {
                match request.value as u8 {
                    CONFIGURATION_NONE => {
                        self.state = DeviceState::Addressed;
                        self.bus.reset();
                        for class in classes.iter_mut() {
                            class.deconfigure();
                        }
                    }
                    CONFIGURATION_VALUE => {
                        self.state = DeviceState::Configured;
                        for class in classes.iter_mut() {
                            class.configure();
                        }
                        debug!("USB: configured");
                    }
                    _ => {
                        let _ = ControlOut::new(&mut self.control, self.bus, &request).reject();
                        return;
                    }
                }
                let _ = ControlOut::new(&mut self.control, self.bus, &request).accept();
            }
            (Recipient::Interface, standard_request::SET_INTERFACE, _) => {
                if request.index > u8::MAX as u16 {
                    let _ = ControlOut::new(&mut self.control, self.bus, &request).reject();
                    return;
                }
                let interface = InterfaceNumber(request.index as u8);
                let alt = request.value as u8;
                for class in classes.iter_mut() {
                    if class.set_alt_setting(interface, alt) {
                        let _ = ControlOut::new(&mut self.control, self.bus, &request).accept();
                        return;
                    }
                }
                let _ = ControlOut::new(&mut self.control, self.bus, &request).reject();
            }
            _ => {}
        }

        if self.control.waiting_for_response() {
            let _ = self.control.reject(self.bus);
        }
    }

    fn get_descriptor(&mut self, classes: &mut Classes<'_, B>, request: Request) {
        let (kind, index) = request.descriptor_type_index();

        let handled = match kind {
            descriptor_type::DEVICE => {
                let descriptor = self.device_descriptor();
                let len = min(descriptor.len(), request.length as usize);
                ControlIn::new(&mut self.control, self.bus, &request)
                    .accept_with(&descriptor[..len])
                    .is_ok()
            }
            descriptor_type::CONFIGURATION => {
                let mut buffer = [0u8; 256];
                let mut writer = DescriptorWriter::new(&mut buffer);

                let attributes = 0x80
                    | if self.info.self_powered { 0x40 } else { 0 }
                    | if self.info.supports_remote_wakeup {
                        0x20
                    } else {
                        0
                    };
                if writer
                    .configuration(attributes, self.info.max_power)
                    .is_err()
                {
                    return;
                }
                for class in classes.iter() {
                    if class.configuration_descriptors(&mut writer).is_err() {
                        return;
                    }
                }
                writer.end_configuration();
                let len = min(writer.position(), request.length as usize);
                ControlIn::new(&mut self.control, self.bus, &request)
                    .accept_with(&buffer[..len])
                    .is_ok()
            }
            descriptor_type::STRING => {
                let mut buffer = [0u8; 128];
                let written = match index {
                    0 => DescriptorWriter::write_lang_ids(&mut buffer, &[lang_id::EN_US]).ok(),
                    1 => self
                        .info
                        .manufacturer
                        .and_then(|s| DescriptorWriter::write_string(&mut buffer, s).ok()),
                    2 => self
                        .info
                        .product
                        .and_then(|s| DescriptorWriter::write_string(&mut buffer, s).ok()),
                    3 => self
                        .info
                        .serial_number
                        .and_then(|s| DescriptorWriter::write_string(&mut buffer, s).ok()),
                    other => {
                        let index = StringIndex::new(other);
                        classes
                            .iter()
                            .find_map(|class| class.string(index, request.index))
                            .and_then(|s| DescriptorWriter::write_string(&mut buffer, s).ok())
                    }
                };
                match written {
                    Some(total) => {
                        let len = min(total, request.length as usize);
                        ControlIn::new(&mut self.control, self.bus, &request)
                            .accept_with(&buffer[..len])
                            .is_ok()
                    }
                    None => false,
                }
            }
            _ => false,
        };

        if !handled && self.control.waiting_for_response() {
            let _ = self.control.reject(self.bus);
        }
    }

    fn device_descriptor(&self) -> [u8; 18] {
        let info = &self.info;
        let i_manufacturer = if info.manufacturer.is_some() { 1 } else { 0 };
        let i_product = if info.product.is_some() { 2 } else { 0 };
        let i_serial = if info.serial_number.is_some() { 3 } else { 0 };
        [
            18,
            descriptor_type::DEVICE,
            info.usb_rev as u8,
            (info.usb_rev >> 8) as u8,
            info.device_class,
            info.device_sub_class,
            info.device_protocol,
            info.max_packet_size_0,
            info.vendor_id as u8,
            (info.vendor_id >> 8) as u8,
            info.product_id as u8,
            (info.product_id >> 8) as u8,
            info.device_release as u8,
            (info.device_release >> 8) as u8,
            i_manufacturer,
            i_product,
            i_serial,
            1, // bNumConfigurations
        ]
    }
}

/// The list of classes a device drives.
pub type Classes<'c, B> = [&'c mut dyn UsbClass<B>];

/// Builder for [`UsbDevice`].
pub struct UsbDeviceBuilder<'a, B: UsbBus> {
    allocator: &'a crate::bus::UsbBusAllocator<B>,
    info: DeviceInfo<'a>,
}

impl<'a, B: UsbBus> UsbDeviceBuilder<'a, B> {
    /// Start a device description with the mandatory vendor and product IDs.
    pub fn new(
        allocator: &'a crate::bus::UsbBusAllocator<B>,
        vendor_id: u16,
        product_id: u16,
    ) -> Self {
        Self {
            allocator,
            info: DeviceInfo {
                vendor_id,
                product_id,
                device_class: class_code::NONE,
                device_sub_class: 0,
                device_protocol: 0,
                max_packet_size_0: 64,
                device_release: 0x0100,
                usb_rev: 0x0200,
                manufacturer: None,
                product: None,
                serial_number: None,
                self_powered: false,
                supports_remote_wakeup: false,
                max_power: 50, // 100 mA
            },
        }
    }

    /// Allocate EP0, enable the bus and return the device.
    pub fn build(self) -> UsbDevice<'a, B> {
        UsbDevice::build(self.allocator, self.info)
    }

    /// Set the device class code.
    pub fn device_class(mut self, class: u8) -> Self {
        self.info.device_class = class;
        self
    }

    /// Set the device subclass code.
    pub fn device_sub_class(mut self, subclass: u8) -> Self {
        self.info.device_sub_class = subclass;
        self
    }

    /// Set the device protocol code.
    pub fn device_protocol(mut self, protocol: u8) -> Self {
        self.info.device_protocol = protocol;
        self
    }

    /// Set the device release number in BCD.
    pub fn device_release(mut self, release: u16) -> Self {
        self.info.device_release = release;
        self
    }

    /// Set the `bcdUSB` field.
    pub fn usb_rev(mut self, rev: u16) -> Self {
        self.info.usb_rev = rev;
        self
    }

    /// Set the EP0 packet size (8, 16, 32 or 64).
    pub fn max_packet_size_0(mut self, size: u8) -> Self {
        self.info.max_packet_size_0 = size;
        self
    }

    /// Set the manufacturer string.
    pub fn manufacturer(mut self, manufacturer: &'a str) -> Self {
        self.info.manufacturer = Some(manufacturer);
        self
    }

    /// Set the product string.
    pub fn product(mut self, product: &'a str) -> Self {
        self.info.product = Some(product);
        self
    }

    /// Set the serial-number string.
    pub fn serial_number(mut self, serial: &'a str) -> Self {
        self.info.serial_number = Some(serial);
        self
    }

    /// Declare that the device has its own power supply.
    pub fn self_powered(mut self, self_powered: bool) -> Self {
        self.info.self_powered = self_powered;
        self
    }

    /// Declare that the device supports remote wake-up.
    pub fn supports_remote_wakeup(mut self, supports: bool) -> Self {
        self.info.supports_remote_wakeup = supports;
        self
    }

    /// Set the maximum bus current, in milliamperes (stored in 2 mA units).
    pub fn max_power_ma(mut self, milliamps: u16) -> Self {
        self.info.max_power = (milliamps / 2).min(250) as u8;
        self
    }

    /// Mark the device as composite: class `0xEF`, subclass `0x02`, protocol `0x01`.
    pub fn composite(mut self) -> Self {
        self.info.device_class = class_code::MISC;
        self.info.device_sub_class = 0x02;
        self.info.device_protocol = 0x01;
        self
    }
}
