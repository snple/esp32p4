//! DHCP client and a TCP echo server on port 7, over the on-chip Ethernet MAC.
//!
//! Everything is `no_std` and heap-free: the socket table, the TCP windows and
//! the DMA rings all live in `static`s. The loop polls the stack, so there is no
//! executor and no async anywhere. It is *not* parked in `wfi` between polls:
//! `Net::poll_wait` exists for that, but on this board entering `wfi` while the
//! MAC is receiving makes the loop miss frames (measured: one iteration every
//! ~460 ms instead of the timer period), which shows up as TCP retransmissions.
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
use esp32p4::emac::{Emac, EmacBuffers, EmacConfig, EmacResources, RmiiPins};
use esp32p4::gpio::Pins;
use esp32p4::main;
use esp32p4::net::{Net, SocketSet, SocketStorage, TcpBuffers, smoltcp};
use esp32p4::pac::Peripherals;
use esp32p4::usb_serial_jtag::UsbSerialJtag;
use smoltcp::socket::tcp;

/// Eight receive slots keep a burst of frames while the loop is busy echoing.
static mut EMAC_BUFFERS: EmacBuffers<8, 4> = EmacBuffers::new();

/// Room for the DHCP socket plus a couple of application sockets.
static mut SOCKET_STORAGE: [SocketStorage<'static>; 4] = [SocketStorage::EMPTY; 4];

/// Receive and transmit windows of the echo socket.
static mut TCP_BUFFERS: TcpBuffers = TcpBuffers::new();

/// MDIO address the IP101GRI is strapped to: `PHY_AD0` high, `PHY_AD3` low.
const PHY_ADDRESS: u8 = 1;
/// Active-low PHY reset on the function board.
const PHY_RESET_PIN: u32 = 51;

const ECHO_PORT: u16 = 7;

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

    // SAFETY: the storages are only used by this function, for the whole run.
    let emac_buffers = unsafe { &mut *core::ptr::addr_of_mut!(EMAC_BUFFERS) };
    let mut emac = Emac::init(
        dp.EMAC,
        rmii,
        EmacConfig {
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

    // SAFETY: handed to the socket once, below.
    let tcp_buffers = unsafe { &mut *core::ptr::addr_of_mut!(TCP_BUFFERS) };
    let socket = tcp::Socket::new(
        tcp::SocketBuffer::new(&mut tcp_buffers.rx[..]),
        tcp::SocketBuffer::new(&mut tcp_buffers.tx[..]),
    );
    let server = net.sockets_mut().add(socket);

    let mut ticks = 0u32;
    let mut echoed = 0u32;
    let mut last_state = tcp::State::Closed;
    loop {
        // Move every frame that is waiting and run the stack's timers. A short
        // busy wait keeps the loop cheap without parking the core (see the
        // module documentation).
        net.poll(&mut emac);
        delay.delay_ms(1);

        // Serve whatever the echo socket currently has room for. The borrow of
        // the socket ends before the status report below.
        {
            let socket = net.sockets_mut().get_mut::<tcp::Socket>(server);

            let mut buf = [0u8; 512];
            while socket.can_recv() {
                let Ok(n) = socket.recv_slice(&mut buf) else {
                    break;
                };
                if socket.send_slice(&buf[..n]).is_err() {
                    break;
                }
                echoed += n as u32;
                defmt::info!("tcp: echoed {} bytes ({} total)", n, echoed);
            }

            let state = socket.state();

            // A peer that closed leaves the socket half-closed: without our own
            // FIN it stays in CloseWait and the next client gets a reset.
            if state == tcp::State::CloseWait {
                socket.close();
            }

            // Closed and TIME-WAIT both accept a fresh `listen`, and asking
            // while already listening succeeds without doing anything.
            if matches!(state, tcp::State::Closed | tcp::State::TimeWait)
                && socket.listen(ECHO_PORT).is_ok()
            {
                echoed = 0;
            }

            let state = socket.state();
            if state != last_state {
                defmt::info!("tcp: {} after {} bytes", state, echoed);
                last_state = state;
            }
        }

        // Report the link once in a while so a cable pull is visible.
        if ticks.is_multiple_of(50) {
            let link = emac.poll_link();
            let ip = net.ipv4_addr().map(|addr| addr.octets());
            let state = net.sockets().get::<tcp::Socket>(server).state();
            defmt::info!(
                "tick {} at {} ms: link {}, ip {:?}, tcp {:?}",
                ticks,
                delay.now_us() / 1000,
                link.up,
                ip,
                state
            );
        }
        ticks = ticks.wrapping_add(1);
    }
}
