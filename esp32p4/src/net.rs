//! Synchronous TCP/IP over the on-chip Ethernet MAC.
//!
//! This module adapts [`smoltcp`] to the EMAC driver: [`Emac`] implements
//! [`smoltcp::phy::Device`], and [`Net`] bundles an interface with a socket set
//! that the application drives from its own main loop.
//!
//! There is no executor, no async and no hidden thread — a call to
//! [`Net::poll`] moves every frame that is currently available in either
//! direction, and returns. That keeps the stack usable from a bare
//! `loop { }` and lets the application decide when to give up the CPU.
//!
//! ```text
//!   main loop                      Net                        EMAC DMA
//!   ─────────                      ───                        ────────
//!   net.poll(&mut emac)  ────────► iface.poll ──────────────► RX ring
//!                                        ◄──────────────────  TX ring
//!   sockets.get_mut::<tcp>()  ────► user data
//! ```
//!
//! # Allocator
//!
//! The stack itself is heap-free. Only the *container* of the socket table can
//! be heap-backed:
//!
//! | Feature | `SocketSet` storage |
//! |---------|---------------------|
//! | `net` (default) | caller-provided `&mut [SocketStorage]`, e.g. a `static` |
//! | `net-alloc` | `Vec` from the global allocator, e.g. the PSRAM heap |
//!
//! Sockets themselves always take caller-provided buffers for TCP; UDP and
//! ICMP have fixed-size metadata.
//!
//! # Example
//!
//! ```ignore
//! use esp32p4::net::{Net, SocketStorage, smoltcp};
//!
//! static mut SOCKET_STORAGE: [SocketStorage<'static>; 4] =
//!     [SocketStorage::EMPTY; 4];
//!
//! let mut emac = Emac::init(/* … */);
//! // SAFETY: the storage is only handed out once.
//! let sockets = smoltcp::iface::SocketSet::new(
//!     &mut *core::ptr::addr_of_mut!(SOCKET_STORAGE)[..],
//! );
//! let mut net = Net::new(&mut emac, sockets, mac_address, random_seed);
//! net.start_dhcp();
//!
//! if net.wait_dhcp(&mut emac, 5_000).is_some() {
//!     // … add sockets, then poll forever
//! }
//! ```

use smoltcp::iface::{Config, Interface, PollResult, SocketHandle};
use smoltcp::phy::{ChecksumCapabilities, Device, DeviceCapabilities, Medium, RxToken, TxToken};
use smoltcp::socket::dhcpv4;
use smoltcp::time::{Duration, Instant};
use smoltcp::wire::{EthernetAddress, HardwareAddress, IpCidr, Ipv4Address, Ipv4Cidr};

use crate::emac::{Emac, MAX_FRAME_SIZE};

pub use smoltcp;
pub use smoltcp::iface::{SocketSet, SocketStorage};
pub use smoltcp::wire::{IpAddress, IpEndpoint};

/// Largest Ethernet frame the stack may hand to the MAC: header plus a 1500-byte
/// payload, FCS excluded.
pub const MTU: usize = 1514;

/// The current time, as the stack wants it.
///
/// Millisecond resolution is enough for `smoltcp`'s timers; the system timer
/// underneath is microsecond accurate.
#[inline]
pub fn now() -> Instant {
    Instant::from_millis((crate::delay::micros() / 1000) as i64)
}

// ============================================================================
// `smoltcp::phy::Device` for the EMAC
// ============================================================================

/// A received frame handed to the stack.
///
/// Dropping it without [`RxToken::consume`] leaves the descriptor in the ring,
/// so the stack always consumes what it is given.
pub struct EmacRxToken<'a, 'd, const RX: usize, const TX: usize> {
    frame: &'a [u8],
    emac: &'a Emac<'d, RX, TX>,
}

impl<const RX: usize, const TX: usize> RxToken for EmacRxToken<'_, '_, RX, TX> {
    fn consume<R, F>(self, f: F) -> R
    where
        F: FnOnce(&[u8]) -> R,
    {
        let result = f(self.frame);
        // The frame lives in the DMA buffer, so the descriptor goes back to the
        // MAC only after the stack is done reading it.
        self.emac.release_frame();
        result
    }
}

/// Permission to build one frame in the next transmit slot.
pub struct EmacTxToken<'a, 'd, const RX: usize, const TX: usize> {
    emac: &'a Emac<'d, RX, TX>,
}

impl<const RX: usize, const TX: usize> TxToken for EmacTxToken<'_, '_, RX, TX> {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        // `Device::transmit` only hands out a token when a descriptor is free,
        // and nothing else can take it in between: the driver is single-threaded
        // and every interrupt is masked. `len` is capped by our `MTU`.
        match self.emac.transmit_with(len, f) {
            Ok(result) => result,
            Err(_) => unreachable!("TX descriptor was taken between transmit() and consume()"),
        }
    }
}

impl<'d, const RX: usize, const TX: usize> Device for Emac<'d, RX, TX> {
    type RxToken<'a>
        = EmacRxToken<'a, 'd, RX, TX>
    where
        Self: 'a;
    type TxToken<'a>
        = EmacTxToken<'a, 'd, RX, TX>
    where
        Self: 'a;

    fn receive(&mut self, _timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        // Both tokens only need shared access: the rings keep their cursors in
        // `Cell`s, and a received frame and a transmit slot never overlap.
        let this: &Self = self;
        let frame = this.receive()?;
        Some((
            EmacRxToken { frame, emac: this },
            EmacTxToken { emac: this },
        ))
    }

    fn transmit(&mut self, _timestamp: Instant) -> Option<Self::TxToken<'_>> {
        let this: &Self = self;
        this.transmit_ready().then_some(EmacTxToken { emac: this })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.medium = Medium::Ethernet;
        caps.max_transmission_unit = MTU;
        // The ring bounds how much can be in flight, not how large a frame is.
        caps.max_burst_size = None;
        // The MAC can verify and insert checksums, but the driver leaves that
        // off: software checksums are cheap on this core and the hardware path
        // has extra descriptor requirements.
        caps.checksum = ChecksumCapabilities::default();
        caps
    }
}

// ============================================================================
// Interface + sockets
// ============================================================================

/// What the DHCP client learned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DhcpInfo {
    /// Address and prefix length handed out by the server.
    pub address: Ipv4Cidr,
    /// Default gateway, if the server sent one.
    pub router: Option<Ipv4Address>,
    /// First DNS server, if the server sent one.
    pub dns: Option<Ipv4Address>,
}

/// Network interface plus its socket table.
///
/// Owns the `smoltcp` interface and the socket set; the [`Emac`] stays with the
/// application so MDIO/PHY access and link polling remain available.
pub struct Net<'a> {
    iface: Interface,
    sockets: SocketSet<'a>,
    dhcp: Option<SocketHandle>,
    dhcp_info: Option<DhcpInfo>,
}

impl<'a> Net<'a> {
    /// Build the interface on top of a started MAC.
    ///
    /// `random_seed` feeds TCP's initial sequence numbers; derive it from
    /// something that differs per boot (the system timer is fine).
    pub fn new<const RX: usize, const TX: usize>(
        device: &mut Emac<'_, RX, TX>,
        sockets: SocketSet<'a>,
        hardware_addr: [u8; 6],
        random_seed: u64,
    ) -> Self {
        let mut config = Config::new(HardwareAddress::Ethernet(EthernetAddress(hardware_addr)));
        config.random_seed = random_seed;

        Self {
            iface: Interface::new(config, device, now()),
            sockets,
            dhcp: None,
            dhcp_info: None,
        }
    }

    /// Build the interface with a heap-allocated socket table.
    #[cfg(feature = "net-alloc")]
    pub fn new_alloc<const RX: usize, const TX: usize>(
        device: &mut Emac<'_, RX, TX>,
        hardware_addr: [u8; 6],
        random_seed: u64,
    ) -> Self {
        let sockets = SocketSet::new(alloc::vec::Vec::new());
        Self::new(device, sockets, hardware_addr, random_seed)
    }

    /// Move every frame the hardware has and run the stack's timers.
    ///
    /// Returns `true` when a socket changed state, so callers can check their
    /// sockets only when it is worth it.
    pub fn poll<const RX: usize, const TX: usize>(
        &mut self,
        device: &mut Emac<'_, RX, TX>,
    ) -> bool {
        let now = now();
        let mut changed =
            self.iface.poll(now, device, &mut self.sockets) == PollResult::SocketStateChanged;

        if let Some(handle) = self.dhcp {
            // Read the event out before touching the interface: the event
            // borrows the socket.
            let update = match self.sockets.get_mut::<dhcpv4::Socket>(handle).poll() {
                Some(dhcpv4::Event::Configured(config)) => Some(Some(DhcpInfo {
                    address: config.address,
                    router: config.router,
                    dns: config.dns_servers.first().copied(),
                })),
                Some(dhcpv4::Event::Deconfigured) => Some(None),
                None => None,
            };

            match update {
                Some(Some(info)) => {
                    self.set_ipv4(info.address, info.router);
                    self.dhcp_info = Some(info);
                    changed = true;
                    info!(
                        "net: DHCP {:?} via {:?}, dns {:?}",
                        info.address, info.router, info.dns
                    );
                }
                Some(None) => {
                    self.clear_ipv4();
                    self.dhcp_info = None;
                    changed = true;
                    warn!("net: DHCP lease lost");
                }
                None => {}
            }
        }

        changed
    }

    /// Poll, then park the core until the next interrupt or timer.
    ///
    /// This is the loop body for a board that has the MAC interrupt wired up:
    /// the core stops in `wfi` instead of spinning, and wakes on either a frame
    /// arriving or the stack's next timer becoming due — whichever comes first,
    /// capped at `max_sleep_ms` so link changes still get noticed.
    ///
    /// Returns `true` when a socket changed state, i.e. when the caller should
    /// look at its sockets before sleeping again.
    ///
    /// Requires [`Emac::enable_interrupts`] to have run and machine interrupts
    /// to be enabled; without them it degrades to polling with a spin delay.
    pub fn poll_wait<const RX: usize, const TX: usize>(
        &mut self,
        device: &mut Emac<'_, RX, TX>,
        max_sleep_ms: u32,
    ) -> bool {
        if self.poll(device) {
            return true;
        }

        // Consume the flag *before* sleeping so a frame that arrived during
        // `poll` is not lost and does not cost a full sleep period.
        device.take_interrupt();

        let sleep_ms = self
            .poll_delay()
            .map(|delay| delay.total_millis() as u32)
            .unwrap_or(max_sleep_ms)
            .min(max_sleep_ms);

        if sleep_ms > 0 {
            crate::delay::sleep_ms(sleep_ms);
        }
        false
    }

    /// How long the stack may sleep before it has work to do.
    ///
    /// `None` means it has nothing scheduled — sleep until the next packet or
    /// poll again after a fixed interval.
    pub fn poll_delay(&mut self) -> Option<Duration> {
        self.iface.poll_delay(now(), &self.sockets)
    }

    /// The socket table, for adding and driving sockets.
    pub fn sockets(&self) -> &SocketSet<'a> {
        &self.sockets
    }

    /// The socket table, mutably.
    pub fn sockets_mut(&mut self) -> &mut SocketSet<'a> {
        &mut self.sockets
    }

    /// The underlying interface.
    pub fn interface(&self) -> &Interface {
        &self.iface
    }

    /// The underlying interface, mutably.
    pub fn interface_mut(&mut self) -> &mut Interface {
        &mut self.iface
    }

    /// Configure a static IPv4 address and optional default gateway.
    pub fn set_ipv4(&mut self, address: Ipv4Cidr, gateway: Option<Ipv4Address>) {
        self.iface.update_ip_addrs(|addrs| {
            addrs.clear();
            // The interface has room for at least one address.
            let _ = addrs.push(IpCidr::Ipv4(address));
        });

        match gateway {
            Some(gateway) => {
                let _ = self.iface.routes_mut().add_default_ipv4_route(gateway);
            }
            None => {
                self.iface.routes_mut().remove_default_ipv4_route();
            }
        }
    }

    /// Drop every IPv4 address and the default route.
    pub fn clear_ipv4(&mut self) {
        self.iface.update_ip_addrs(|addrs| addrs.clear());
        self.iface.routes_mut().remove_default_ipv4_route();
    }

    /// The first IPv4 address the interface holds, if any.
    pub fn ipv4_addr(&self) -> Option<Ipv4Address> {
        self.iface.ipv4_addr()
    }

    /// Start the DHCP client, unless it is already running.
    pub fn start_dhcp(&mut self) {
        if self.dhcp.is_none() {
            self.dhcp = Some(self.sockets.add(dhcpv4::Socket::new()));
        }
    }

    /// Stop the DHCP client and release the lease it applied.
    pub fn stop_dhcp(&mut self) {
        if let Some(handle) = self.dhcp.take() {
            self.sockets.remove(handle);
            self.dhcp_info = None;
        }
    }

    /// What the DHCP client last learned.
    pub fn dhcp_info(&self) -> Option<DhcpInfo> {
        self.dhcp_info
    }

    /// Poll until the DHCP client has an address, or `timeout_ms` elapses.
    ///
    /// This busy-spins: without interrupts there is nothing to wait on. Use it
    /// during start-up, not in a hot loop.
    pub fn wait_dhcp<const RX: usize, const TX: usize>(
        &mut self,
        device: &mut Emac<'_, RX, TX>,
        timeout_ms: u32,
    ) -> Option<DhcpInfo> {
        let deadline = crate::delay::micros() + timeout_ms as u64 * 1000;

        loop {
            self.poll(device);
            if let Some(info) = self.dhcp_info {
                return Some(info);
            }
            if crate::delay::micros() >= deadline {
                return None;
            }
        }
    }
}

// ============================================================================
// Socket setup helpers
// ============================================================================

/// Frame buffers for one TCP socket.
///
/// TCP needs a receive and a transmit window. 8 KiB each is several segments
/// worth: the receive buffer doubles as smoltcp's reassembly buffer, and one
/// that is barely one window deep fills up completely with out-of-order
/// segments as soon as the first segment of a flight is lost. The advertised
/// window then pins at zero — the hole can never be filled — and the connection
/// stalls until the peer gives up and reconnects.
#[repr(C, align(8))]
pub struct TcpBuffers {
    /// Window for data arriving from the peer.
    pub rx: [u8; Self::DEFAULT_SIZE],
    /// Window for data we send.
    pub tx: [u8; Self::DEFAULT_SIZE],
}

impl TcpBuffers {
    /// Buffer size used by [`TcpBuffers::new`].
    pub const DEFAULT_SIZE: usize = 8192;

    /// A zeroed pair of buffers, usable in `static` initialisers.
    pub const fn new() -> Self {
        Self {
            rx: [0; Self::DEFAULT_SIZE],
            tx: [0; Self::DEFAULT_SIZE],
        }
    }
}

impl Default for TcpBuffers {
    fn default() -> Self {
        Self::new()
    }
}

/// Largest frame the MAC can carry, as a convenience re-export.
pub const MAX_FRAME_SIZE_INCL_FCS: usize = MAX_FRAME_SIZE;
