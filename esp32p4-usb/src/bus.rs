//! The USB bus trait and the endpoint allocator.
//!
//! # Two phases
//!
//! ```text
//! UsbBusAllocator::new(bus)
//!        │  alloc_ep() per endpoint, interface(), string()
//!        ▼
//! UsbDeviceBuilder::build() ──► freeze() ──► bus.enable()
//!        │                                      │
//!        └── UsbDevice + class handles ◄────────┘  (all share &B)
//! ```
//!
//! Before the freeze, endpoints and interfaces are handed out through
//! `&mut B`; afterwards only `&B` exists, and all hardware access goes through
//! interior mutability. That mirrors how the classes hold endpoint handles for
//! as long as the device lives.

use core::cell::{Cell, UnsafeCell};

use crate::endpoint::{Direction, Endpoint, EndpointAddress, EndpointDir, EndpointType};
use crate::error::{UsbError, UsbResult};

/// A batch of events reported by [`UsbBus::poll`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PollResult {
    /// Nothing happened.
    None,
    /// The host reset the bus.
    Reset,
    /// Endpoint activity: bit `n` of each mask belongs to endpoint number `n`.
    Data {
        /// OUT endpoints that received a packet.
        ep_out: u16,
        /// IN endpoints whose packet left.
        ep_in_complete: u16,
        /// Endpoints that received a SETUP packet (in practice only EP0).
        ep_setup: u16,
    },
    /// The host suspended the bus.
    Suspend,
    /// The bus left suspend (resume signalling or a reset).
    Resume,
}

impl PollResult {
    /// Whether any endpoint event is set.
    pub fn is_data(self) -> bool {
        matches!(self, Self::Data { .. })
    }
}

/// Hardware abstraction behind the device stack.
///
/// Implementors own one USB device controller. Only [`alloc_ep`](Self::alloc_ep)
/// and [`enable`](Self::enable) take `&mut self`; everything else takes `&self`
/// and must use interior mutability, because [`UsbDevice`](crate::device::UsbDevice)
/// and the endpoint handles share the bus.
pub trait UsbBus: Sized {
    /// Reserve an endpoint.
    ///
    /// `address` pins the endpoint number when `Some`; otherwise the next free
    /// number in `direction` is used.
    fn alloc_ep(
        &mut self,
        direction: Direction,
        address: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> UsbResult<EndpointAddress>;

    /// Configure the FIFOs and connect to the bus.
    ///
    /// Called once, after the last endpoint was allocated. The host resets and
    /// enumerates the device shortly afterwards.
    fn enable(&mut self);

    /// Return the endpoints to their post-`alloc_ep` state after a bus reset.
    fn reset(&self);

    /// Program the USB device address.
    fn set_device_address(&self, address: u8);

    /// Collect pending bus events.
    ///
    /// Must be called often enough to keep the shared RX FIFO from overflowing;
    /// every few milliseconds is ample.
    fn poll(&self) -> PollResult;

    /// Queue one packet on an IN endpoint.
    fn write(&self, address: EndpointAddress, data: &[u8]) -> UsbResult<usize>;

    /// Pop one packet from an OUT endpoint.
    fn read(&self, address: EndpointAddress, buffer: &mut [u8]) -> UsbResult<usize>;

    /// Set or clear the STALL handshake.
    fn set_stalled(&self, address: EndpointAddress, stalled: bool);

    /// Whether the endpoint currently answers with STALL.
    fn is_stalled(&self, address: EndpointAddress) -> bool;

    /// Enter suspend (the host stopped sending SOFs).
    fn suspend(&self);

    /// Leave suspend.
    fn resume(&self);

    /// Force a disconnect/reconnect so the host re-enumerates.
    fn force_reset(&self) -> UsbResult<()> {
        Err(UsbError::Unsupported)
    }

    /// Whether [`set_device_address`](Self::set_device_address) has to be called
    /// *before* the status stage of `SET_ADDRESS`.
    ///
    /// The USB specification applies the new address after the status stage;
    /// controllers such as the DWC OTG need it before.
    const SET_ADDRESS_BEFORE_STATUS: bool = false;
}

/// An allocated interface number.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InterfaceNumber(pub(crate) u8);

impl From<InterfaceNumber> for u8 {
    #[inline]
    fn from(number: InterfaceNumber) -> Self {
        number.0
    }
}

/// An allocated string-descriptor index.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StringIndex(u8);

impl StringIndex {
    /// Index 0, which means "no string".
    pub const NONE: Self = Self(0);

    pub(crate) const fn new(index: u8) -> Self {
        Self(index)
    }
}

impl From<StringIndex> for u8 {
    #[inline]
    fn from(index: StringIndex) -> Self {
        index.0
    }
}

/// Hands out endpoints, interfaces and string indices, then freezes the bus.
pub struct UsbBusAllocator<B: UsbBus> {
    bus: UnsafeCell<B>,
    frozen: Cell<bool>,
    next_interface: Cell<u8>,
    next_string: Cell<u8>,
}

// SAFETY: the allocator is a single-owner object before the freeze; afterwards
// only shared access to `B` remains, so `B`'s own thread-safety bounds apply.
unsafe impl<B: UsbBus + Send> Send for UsbBusAllocator<B> {}
unsafe impl<B: UsbBus + Sync> Sync for UsbBusAllocator<B> {}

impl<B: UsbBus> UsbBusAllocator<B> {
    /// Wrap a bus that has not been enabled yet.
    pub fn new(bus: B) -> Self {
        Self {
            bus: UnsafeCell::new(bus),
            frozen: Cell::new(false),
            next_interface: Cell::new(0),
            // 0 = language IDs, 1..=3 = manufacturer, product, serial.
            next_string: Cell::new(4),
        }
    }

    /// Allocate an endpoint.
    ///
    /// Prefer the typed helpers ([`bulk`](Self::bulk), [`interrupt`](Self::interrupt))
    /// in class constructors.
    pub fn alloc<D: EndpointDir>(
        &self,
        address: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> UsbResult<Endpoint<'_, B, D>> {
        assert!(
            !self.frozen.get(),
            "endpoints cannot be allocated after the device was built"
        );
        // SAFETY: `frozen` is false, so no `&B` derived from this cell exists
        // yet; allocation is strictly sequential.
        let address = unsafe {
            (*self.bus.get()).alloc_ep(D::DIRECTION, address, ep_type, max_packet_size, interval)?
        };
        // SAFETY: after this call returns no further `&mut B` is created;
        // `freeze` sets `frozen` before handing out `&B`.
        let bus = unsafe { &*self.bus.get() };
        Ok(Endpoint::new(
            bus,
            address,
            ep_type,
            max_packet_size,
            interval,
        ))
    }

    /// Allocate a control endpoint.
    pub fn control<D: EndpointDir>(&self, max_packet_size: u16) -> Endpoint<'_, B, D> {
        self.alloc(None, EndpointType::Control, max_packet_size, 0)
            .expect("control endpoint allocation failed")
    }

    /// Allocate a bulk endpoint.
    pub fn bulk<D: EndpointDir>(&self, max_packet_size: u16) -> Endpoint<'_, B, D> {
        self.alloc(None, EndpointType::Bulk, max_packet_size, 0)
            .expect("bulk endpoint allocation failed")
    }

    /// Allocate an interrupt endpoint.
    pub fn interrupt<D: EndpointDir>(
        &self,
        max_packet_size: u16,
        interval: u8,
    ) -> Endpoint<'_, B, D> {
        self.alloc(None, EndpointType::Interrupt, max_packet_size, interval)
            .expect("interrupt endpoint allocation failed")
    }

    /// Allocate an isochronous endpoint.
    pub fn isochronous<D: EndpointDir>(
        &self,
        synchronization: crate::endpoint::IsoSynchronization,
        usage: crate::endpoint::IsoUsage,
        max_packet_size: u16,
        interval: u8,
    ) -> Endpoint<'_, B, D> {
        self.alloc(
            None,
            EndpointType::Isochronous {
                synchronization,
                usage,
            },
            max_packet_size,
            interval,
        )
        .expect("isochronous endpoint allocation failed")
    }

    /// Allocate the next interface number.
    pub fn interface(&self) -> InterfaceNumber {
        assert!(
            !self.frozen.get(),
            "interfaces cannot be allocated after the device was built"
        );
        let number = self.next_interface.get();
        self.next_interface.set(number + 1);
        InterfaceNumber(number)
    }

    /// Allocate the next string-descriptor index.
    pub fn string(&self) -> StringIndex {
        assert!(
            !self.frozen.get(),
            "strings cannot be allocated after the device was built"
        );
        let index = self.next_string.get();
        self.next_string.set(index + 1);
        StringIndex(index)
    }

    /// Enable the bus and return the shared reference every user holds.
    pub(crate) fn freeze(&self) -> &B {
        if !self.frozen.get() {
            self.frozen.set(true);
            // SAFETY: this is the last `&mut B`; `frozen` prevents any later
            // `&mut B` from being created through the allocator.
            unsafe { (*self.bus.get()).enable() };
        }
        // SAFETY: the bus lives as long as `self` and is never mutably borrowed
        // again after the freeze.
        unsafe { &*self.bus.get() }
    }
}
