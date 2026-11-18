//! USB classes.
//!
//! A class contributes interface and endpoint descriptors to the configuration
//! and gets the control requests and endpoint events it is interested in. All
//! callbacks have a no-op default, so a class only overrides what it needs.

pub mod cdc_acm;

use crate::bus::{InterfaceNumber, StringIndex, UsbBus};
use crate::control::{ControlIn, ControlOut};
use crate::descriptor::DescriptorWriter;
use crate::endpoint::EndpointAddress;
use crate::error::UsbResult;

/// One function of a composite device.
pub trait UsbClass<B: UsbBus> {
    /// Append this class's interface and endpoint descriptors.
    fn configuration_descriptors(&self, writer: &mut DescriptorWriter<'_>) -> UsbResult<()> {
        let _ = writer;
        Ok(())
    }

    /// Return the string for an index handed out by
    /// [`UsbBusAllocator::string`](crate::bus::UsbBusAllocator::string).
    fn string(&self, index: StringIndex, lang_id: u16) -> Option<&str> {
        let _ = (index, lang_id);
        None
    }

    /// Handle a control IN request. Accept or reject it, or drop the handle to
    /// pass it to the next class.
    fn control_in(&mut self, transfer: ControlIn<'_, B>) {
        let _ = transfer;
    }

    /// Handle a control OUT request. Accept or reject it, or drop the handle to
    /// pass it to the next class.
    fn control_out(&mut self, transfer: ControlOut<'_, B>) {
        let _ = transfer;
    }

    /// A packet arrived on `address`.
    fn endpoint_out(&mut self, address: EndpointAddress) {
        let _ = address;
    }

    /// The packet queued on `address` was collected by the host.
    fn endpoint_in_complete(&mut self, address: EndpointAddress) {
        let _ = address;
    }

    /// A SETUP packet arrived on a non-zero control endpoint.
    fn endpoint_setup(&mut self, address: EndpointAddress) {
        let _ = address;
    }

    /// The host reset the bus.
    fn reset(&mut self) {}

    /// The host selected the configuration.
    fn configure(&mut self) {}

    /// The host deselected the configuration.
    fn deconfigure(&mut self) {}

    /// Called after every batch of endpoint events.
    fn poll(&mut self) {}

    /// The current alternate setting of `interface`, if this class owns it.
    fn alt_setting(&mut self, interface: InterfaceNumber) -> Option<u8> {
        let _ = interface;
        None
    }

    /// Select an alternate setting. Return `true` when the interface is ours.
    fn set_alt_setting(&mut self, interface: InterfaceNumber, alternate: u8) -> bool {
        let _ = (interface, alternate);
        false
    }
}
