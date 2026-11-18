//! Endpoint addresses, transfer types and endpoint handles.
//!
//! An [`Endpoint`] handle borrows the [`UsbBus`] that owns the hardware, so the
//! bus cannot be dropped while a class still holds endpoints. Handles are
//! direction-typed ([`EndpointIn`] / [`EndpointOut`]), which makes it impossible
//! to read from an IN endpoint or write to an OUT endpoint by accident.

use core::marker::PhantomData;

use crate::bus::UsbBus;
use crate::error::UsbResult;

/// Marker trait for the two endpoint directions.
pub trait EndpointDir: Sized + 'static {
    /// Direction of endpoints created with this marker.
    const DIRECTION: Direction;
}

/// Device-to-host (IN) marker.
pub struct In;

impl EndpointDir for In {
    const DIRECTION: Direction = Direction::In;
}

/// Host-to-device (OUT) marker.
pub struct Out;

impl EndpointDir for Out {
    const DIRECTION: Direction = Direction::Out;
}

/// Transfer direction, encoded in bit 7 of the endpoint address.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Direction {
    /// Host to device (`0x00`).
    Out = 0x00,
    /// Device to host (`0x80`).
    In = 0x80,
}

impl From<u8> for Direction {
    #[inline]
    fn from(value: u8) -> Self {
        if value & 0x80 != 0 {
            Self::In
        } else {
            Self::Out
        }
    }
}

/// Transfer type of an endpoint.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EndpointType {
    /// Control endpoint; only EP0 is used in this stack.
    Control,
    /// Isochronous endpoint.
    Isochronous {
        /// Synchronisation feedback mode.
        synchronization: IsoSynchronization,
        /// How the endpoint uses its bandwidth.
        usage: IsoUsage,
    },
    /// Bulk endpoint: large, reliable, no latency guarantee.
    Bulk,
    /// Interrupt endpoint: small, reliable, bounded latency.
    Interrupt,
}

impl EndpointType {
    /// Encode the transfer type in the endpoint descriptor's `bmAttributes`.
    pub fn bm_attributes(self) -> u8 {
        match self {
            Self::Control => 0b00,
            Self::Bulk => 0b10,
            Self::Interrupt => 0b11,
            Self::Isochronous {
                synchronization,
                usage,
            } => {
                let sync = match synchronization {
                    IsoSynchronization::None => 0b00,
                    IsoSynchronization::Asynchronous => 0b01,
                    IsoSynchronization::Adaptive => 0b10,
                    IsoSynchronization::Synchronous => 0b11,
                };
                let usage = match usage {
                    IsoUsage::Data => 0b00,
                    IsoUsage::Feedback => 0b01,
                    IsoUsage::ImplicitFeedbackData => 0b10,
                };
                0b01 | (sync << 2) | (usage << 4)
            }
        }
    }
}

/// Isochronous synchronisation type.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IsoSynchronization {
    /// No synchronisation.
    None,
    /// The endpoint adapts its clock to the host's.
    Asynchronous,
    /// The endpoint's clock is adjusted by feedback.
    Adaptive,
    /// The endpoint is locked to the host's clock.
    Synchronous,
}

/// Isochronous usage type.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IsoUsage {
    /// Data endpoint.
    Data,
    /// Feedback endpoint.
    Feedback,
    /// Implicit feedback data endpoint.
    ImplicitFeedbackData,
}

/// Endpoint number plus direction, as encoded in a USB endpoint address byte.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EndpointAddress(u8);

impl EndpointAddress {
    /// EP0 OUT: the control endpoint's host-to-device direction.
    pub const EP0_OUT: Self = Self(0x00);
    /// EP0 IN: the control endpoint's device-to-host direction.
    pub const EP0_IN: Self = Self(0x80);

    /// Build an address from a number (0..=15) and a direction.
    #[inline]
    pub const fn new(number: u8, direction: Direction) -> Self {
        Self((number & 0x0f) | direction as u8)
    }

    /// Build an OUT address.
    #[inline]
    pub const fn out(number: u8) -> Self {
        Self(number & 0x0f)
    }

    /// Build an IN address.
    #[inline]
    pub const fn in_(number: u8) -> Self {
        Self((number & 0x0f) | 0x80)
    }

    /// The raw address byte.
    #[inline]
    pub const fn raw(self) -> u8 {
        self.0
    }

    /// The endpoint number (0..=15).
    #[inline]
    pub const fn number(self) -> u8 {
        self.0 & 0x0f
    }

    /// The direction bit.
    #[inline]
    pub const fn direction(self) -> Direction {
        if self.is_in() {
            Direction::In
        } else {
            Direction::Out
        }
    }

    /// Whether this is an IN (device-to-host) endpoint.
    #[inline]
    pub const fn is_in(self) -> bool {
        self.0 & 0x80 != 0
    }

    /// Whether this is an OUT (host-to-device) endpoint.
    #[inline]
    pub const fn is_out(self) -> bool {
        !self.is_in()
    }
}

impl From<u8> for EndpointAddress {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<EndpointAddress> for u8 {
    #[inline]
    fn from(address: EndpointAddress) -> Self {
        address.0
    }
}

impl core::fmt::Debug for EndpointAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "EP{} {}",
            self.number(),
            if self.is_in() { "IN" } else { "OUT" }
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for EndpointAddress {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EP{} {}",
            self.number(),
            if self.is_in() { "IN" } else { "OUT" }
        );
    }
}

/// A handle to one allocated endpoint.
///
/// `'a` is the lifetime of the [`crate::bus::UsbBusAllocator`] the endpoint was
/// allocated from.
pub struct Endpoint<'a, B: UsbBus, D: EndpointDir> {
    bus: &'a B,
    address: EndpointAddress,
    ep_type: EndpointType,
    max_packet_size: u16,
    interval: u8,
    _dir: PhantomData<fn() -> D>,
}

impl<'a, B: UsbBus, D: EndpointDir> Endpoint<'a, B, D> {
    pub(crate) fn new(
        bus: &'a B,
        address: EndpointAddress,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> Self {
        Self {
            bus,
            address,
            ep_type,
            max_packet_size,
            interval,
            _dir: PhantomData,
        }
    }

    /// The endpoint address.
    #[inline]
    pub fn address(&self) -> EndpointAddress {
        self.address
    }

    /// The endpoint's transfer type.
    #[inline]
    pub fn ep_type(&self) -> EndpointType {
        self.ep_type
    }

    /// The endpoint's maximum packet size, in bytes.
    #[inline]
    pub fn max_packet_size(&self) -> u16 {
        self.max_packet_size
    }

    /// The polling interval, in frames (interrupt and isochronous endpoints).
    #[inline]
    pub fn interval(&self) -> u8 {
        self.interval
    }

    /// Stall the endpoint, answering further tokens with a STALL handshake.
    pub fn stall(&self) {
        self.bus.set_stalled(self.address, true);
    }

    /// Clear a stall condition and reset the endpoint's data toggle.
    pub fn unstall(&self) {
        self.bus.set_stalled(self.address, false);
    }
}

/// A device-to-host endpoint handle.
pub type EndpointIn<'a, B> = Endpoint<'a, B, In>;
/// A host-to-device endpoint handle.
pub type EndpointOut<'a, B> = Endpoint<'a, B, Out>;

impl<B: UsbBus> EndpointIn<'_, B> {
    /// Queue one packet.
    ///
    /// Returns the number of bytes queued, which is `data.len()`. A packet
    /// larger than the endpoint's maximum packet size is rejected with
    /// [`UsbError::BufferOverflow`](crate::error::UsbError::BufferOverflow); an in-flight packet with
    /// [`UsbError::WouldBlock`](crate::error::UsbError::WouldBlock).
    pub fn write(&self, data: &[u8]) -> UsbResult<usize> {
        self.bus.write(self.address, data)
    }
}

impl<B: UsbBus> EndpointOut<'_, B> {
    /// Pop one received packet into `buf`.
    ///
    /// Returns [`UsbError::WouldBlock`](crate::error::UsbError::WouldBlock) when nothing has arrived yet. The bus
    /// keeps the endpoint armed, so the packet is not lost.
    pub fn read(&self, buf: &mut [u8]) -> UsbResult<usize> {
        self.bus.read(self.address, buf)
    }
}
