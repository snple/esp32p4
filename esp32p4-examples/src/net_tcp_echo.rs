//! DHCP client and a TCP echo server on port 7, over the on-chip Ethernet MAC.
//!
//! Everything is `no_std` and heap-free: the socket table, the TCP windows and
//! the DMA rings all live in `static`s. The loop is interrupt-driven — the MAC's
//! DMA completion line and the system timer's comparator wake the core out of
//! `wfi` — but there is no executor and no async anywhere.
//!
//! ```text
//! cargo run --bin net_tcp_echo
//! # then, from the host:
//! nc <address printed on the console> 7
//! ```
//!
//! Wiring is the ESP32-P4 function board, the same map as the `emac` example: an
//! IP101GRI at MDIO address 1 (`PHY_AD0` strapped high, `PHY_AD3` low) with its
//! reset on GPIO51.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

use esp32p4::delay::{self, Delay};
use esp32p4::efuse::Efuse;
use esp32p4::emac::{Emac, EmacBuffers, EmacConfig, EmacResources, RmiiPins};
use esp32p4::gpio::Pins;
use esp32p4::main;
use esp32p4::net::{Net, SocketSet, SocketStorage, TcpBuffers, smoltcp};
use esp32p4::pac::Peripherals;
use esp32p4::usb_serial_jtag::UsbSerialJtag;
use smoltcp::iface::SocketHandle;
use smoltcp::socket::tcp;

/// Receive and transmit slots for the DMA rings.
///
/// At 100 Mbit/s a full-size frame arrives every ~120 µs, while the loop below
/// spends far longer than that on the console output, so a short ring overflows
/// and the stack sees holes it has to repair with retransmissions. Sixteen slots
/// ride out a burst long enough for the loop to come back and harvest them.
static mut EMAC_BUFFERS: EmacBuffers<16, 8> = EmacBuffers::new();

/// Room for the DHCP socket plus the pool of echo sockets.
static mut SOCKET_STORAGE: [SocketStorage<'static>; 4] = [SocketStorage::EMPTY; 4];

/// Receive and transmit windows, one pair per echo socket.
static mut TCP_BUFFERS: [TcpBuffers; ECHO_SOCKETS] = [TcpBuffers::new(), TcpBuffers::new()];

/// MDIO address the IP101GRI is strapped to: `PHY_AD0` high, `PHY_AD3` low.
const PHY_ADDRESS: u8 = 1;
/// Active-low PHY reset on the function board.
const PHY_RESET_PIN: u32 = 51;

const ECHO_PORT: u16 = 7;
/// How long a graceful close may take before the socket is reset instead.
const CLOSE_TIMEOUT_US: u64 = 500_000;
/// How long an established connection may stay silent before it is dropped.
///
/// A peer that times out without closing leaves its socket in `Established`
/// forever, and both listeners can end up parked on dead connections. Ten
/// seconds of silence is well past anything an echo client needs.
const IDLE_TIMEOUT_US: u64 = 10_000_000;

/// Number of sockets listening on the echo port.
///
/// A closing socket sits in `LastAck` for up to [`CLOSE_TIMEOUT_US`] waiting for
/// the peer's last ACK and cannot accept anything meanwhile. A spare listener
/// makes that window harmless: a client arriving while the other socket is
/// closing is served instead of being reset.
const ECHO_SOCKETS: usize = 2;

/// One echo socket plus the bookkeeping its close path needs.
struct EchoSocket {
    handle: SocketHandle,
    closing_since: Option<u64>,
    last_state: tcp::State,
    /// Last time bytes moved on this socket, or it became established.
    last_activity: u64,
    echoed: u32,
}

impl EchoSocket {
    /// Placeholder for a socket that is added to the table later.
    fn new() -> Self {
        Self {
            handle: SocketHandle::default(),
            closing_since: None,
            last_state: tcp::State::Closed,
            last_activity: 0,
            echoed: 0,
        }
    }

    /// Echo whatever the peer sent and keep the socket listening between
    /// connections.
    fn service(&mut self, sockets: &mut SocketSet<'_>, now_us: u64) {
        let socket = sockets.get_mut::<tcp::Socket>(self.handle);

        // Only take what the transmit window can hold: bytes taken out of the
        // receive window are already acknowledged, so the peer never sends them
        // again and dropping them here would silently shorten the stream.
        let mut buf = [0u8; 512];
        let mut moved = 0u32;
        while socket.may_send() && socket.can_recv() {
            let room = socket.send_capacity() - socket.send_queue();
            if room == 0 {
                break;
            }
            let want = room.min(buf.len());
            let Ok(n) = socket.recv_slice(&mut buf[..want]) else {
                break;
            };
            // `n <= room`, so the send cannot fail; if it ever does, the bytes
            // are already gone from the receive buffer and there is nothing
            // better to do than let the next pass continue.
            if socket.send_slice(&buf[..n]).is_err() {
                break;
            }
            moved += n as u32;
        }
        // One line per pass: a log statement per 512-byte chunk is slow enough
        // over USB to let the next burst overrun the receive ring.
        if moved > 0 {
            self.echoed += moved;
            self.last_activity = now_us;
            defmt::info!("tcp: echoed {} bytes ({} total)", moved, self.echoed);
        }

        // Track the state before acting on it: the idle check below must not
        // fire on the very pass that brings the socket up.
        let state = socket.state();
        if state != self.last_state {
            defmt::info!("tcp {}: {} after {} bytes", self.handle, state, self.echoed);
            if state == tcp::State::Established {
                self.last_activity = now_us;
            }
            self.last_state = state;
        }

        match state {
            // A peer that closed leaves the socket half-closed: without our own
            // FIN it stays in CloseWait and the next client gets a reset.
            tcp::State::CloseWait => {
                socket.close();
                self.closing_since = Some(now_us);
            }
            // Our FIN is out and the peer has to acknowledge it. If that last
            // ACK never comes (a lossy link drops it), waiting keeps this socket
            // out of `Listen`, so give the graceful close a bounded time.
            tcp::State::LastAck => {
                let since = *self.closing_since.get_or_insert(now_us);
                if now_us.wrapping_sub(since) > CLOSE_TIMEOUT_US {
                    socket.abort();
                    self.closing_since = None;
                }
            }
            // Closed and TIME-WAIT both accept a fresh `listen`, and asking
            // while already listening succeeds without doing anything.
            tcp::State::Closed | tcp::State::TimeWait => {
                if socket.listen(ECHO_PORT).is_ok() {
                    self.echoed = 0;
                }
                self.closing_since = None;
            }
            // An established connection that goes quiet is dead: drop it, or
            // the socket never returns to `Listen`.
            tcp::State::Established => {
                if now_us.wrapping_sub(self.last_activity) > IDLE_TIMEOUT_US {
                    defmt::warn!("tcp {}: no data for 10 s, dropping", self.handle);
                    socket.abort();
                    self.last_activity = now_us;
                }
            }
            _ => self.closing_since = None,
        }
    }
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    // The USB Serial/JTAG logger silently drops everything until its clock gate
    // is open, so the port is opened before the first log statement.
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let delay = Delay::init(dp.SYSTIMER);

    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);

    // Reset the PHY before touching the MAC: assert nRST, then give the
    // oscillator time to start.
    let mut phy_reset = pins.pin::<PHY_RESET_PIN>();
    phy_reset.into_output().set_low();
    delay.delay_ms(100);
    phy_reset.set_high();
    delay.delay_ms(300);

    let rmii = RmiiPins::new(
        pins.pin::<49>(), // TXEN
        pins.pin::<34>(), // TXD0
        pins.pin::<35>(), // TXD1
        pins.pin::<30>(), // RXD1
        pins.pin::<29>(), // RXD0
        pins.pin::<28>(), // CRS_DV
        pins.pin::<50>(), // REF_CLK (50 MHz from the PHY)
        pins.pin::<31>(), // MDC
        pins.pin::<52>(), // MDIO
    );
    let (gpio, io_mux) = pins.release();

    // The factory address from eFuse beats the driver's `02:00:00:00:00:01`
    // default: a locally administered address upsets some bridges, and every
    // board on the same LAN would otherwise look alike.
    let mac = Efuse::init(dp.EFUSE).mac();

    // SAFETY: the storages are only used by this function, for the whole run.
    let emac_buffers = unsafe { &mut *core::ptr::addr_of_mut!(EMAC_BUFFERS) };
    let mut emac = Emac::init(
        dp.EMAC,
        rmii,
        EmacConfig {
            mac_address: mac,
            phy_address: PHY_ADDRESS,
            ..EmacConfig::default()
        },
        emac_buffers,
        EmacResources {
            gpio: &gpio,
            io_mux: &io_mux,
            hp_sys: &dp.HP_SYS,
            clkrst: &dp.HP_SYS_CLKRST,
            lp_clkrst: &dp.LP_AON_CLKRST,
        },
    );

    let mac = emac.mac_address();
    defmt::info!(
        "EMAC {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0],
        mac[1],
        mac[2],
        mac[3],
        mac[4],
        mac[5]
    );

    let link = emac.autonegotiate();
    if link.up {
        defmt::info!("link up: {} {}", link.speed, link.duplex);
        // The very first frame after auto-negotiation is regularly lost — the
        // PHY has just finished training. A short settle keeps the first DHCP
        // DISCOVER from going nowhere.
        delay.delay_ms(200);
    } else {
        defmt::warn!("no link: check the cable and the PHY strapping");
    }

    // SAFETY: handed to the socket set once, below.
    let storage = unsafe { &mut *core::ptr::addr_of_mut!(SOCKET_STORAGE) };
    let mut net = Net::new(
        &mut emac,
        SocketSet::new(&mut storage[..]),
        mac,
        // A per-boot seed keeps TCP's initial sequence numbers unpredictable
        // enough to avoid collisions after a reset.
        delay::micros(),
    );
    net.start_dhcp();

    defmt::info!("waiting for DHCP…");
    // The server probes the offered address before answering, so allow more
    // than the usual few seconds.
    match net.wait_dhcp(&mut emac, 30_000) {
        Some(info) => {
            let addr = info.address.address().octets();
            let router = info.router.map(|r| r.octets()).unwrap_or([0; 4]);
            defmt::info!(
                "got {}.{}.{}.{}/{} via {}.{}.{}.{}",
                addr[0],
                addr[1],
                addr[2],
                addr[3],
                info.address.prefix_len(),
                router[0],
                router[1],
                router[2],
                router[3],
            );
        }
        None => {
            defmt::warn!("DHCP timed out; falling back to 10.0.0.2/24");
            net.set_ipv4(
                smoltcp::wire::Ipv4Cidr::new(smoltcp::wire::Ipv4Address::new(10, 0, 0, 2), 24),
                Some(smoltcp::wire::Ipv4Address::new(10, 0, 0, 1)),
            );
        }
    }
    defmt::info!("echo server on port {}", ECHO_PORT);

    // Interrupt-driven from here on: the MAC's DMA completion line and the
    // system timer's comparator both wake the core out of `wfi`.
    emac.enable_interrupts(esp32p4::interrupt::lines::EMAC);
    // SAFETY: both handlers are installed and the peripheral sources cleared.
    unsafe { esp32p4::interrupt::enable() };

    // SAFETY: handed to the sockets once, below.
    let tcp_buffers = unsafe { &mut *core::ptr::addr_of_mut!(TCP_BUFFERS) };
    let mut echoes: [EchoSocket; ECHO_SOCKETS] = core::array::from_fn(|_| EchoSocket::new());
    for (echo, buffers) in echoes.iter_mut().zip(tcp_buffers.iter_mut()) {
        let socket = tcp::Socket::new(
            tcp::SocketBuffer::new(&mut buffers.rx[..]),
            tcp::SocketBuffer::new(&mut buffers.tx[..]),
        );
        echo.handle = net.sockets_mut().add(socket);
    }

    let mut ticks = 0u32;
    loop {
        // Park the core until the MAC or the stack's next timer has something
        // to do, then run the whole loop body.
        net.poll_wait(&mut emac, 20);

        for echo in &mut echoes {
            echo.service(net.sockets_mut(), delay.now_us());
        }

        // Data written to a socket only leaves the stack on the next `poll`.
        // Flushing here keeps the echo from waiting for the next wake-up (a
        // full sleep period, ~20 ms).
        net.poll(&mut emac);

        // Report the link once in a while so a cable pull is visible.
        if ticks.is_multiple_of(50) {
            let link = emac.poll_link();
            let ip = net.ipv4_addr().map(|addr| addr.octets());
            let states: [tcp::State; ECHO_SOCKETS] = core::array::from_fn(|i| {
                net.sockets().get::<tcp::Socket>(echoes[i].handle).state()
            });
            defmt::info!(
                "tick {} at {} ms: link {}, ip {:?}, tcp {:?} / {:?}",
                ticks,
                delay.now_us() / 1000,
                link.up,
                ip,
                states[0],
                states[1]
            );
        }
        ticks = ticks.wrapping_add(1);
    }
}
