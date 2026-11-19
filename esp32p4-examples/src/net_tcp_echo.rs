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
//! Wiring: an RMII PHY on GPIO28..GPIO36 plus MDC/MDIO on GPIO20/21, the same
//! layout as the `emac` example.

#![no_std]
#![no_main]

// `--features defmt` turns on the HAL's log statements for every binary, so
// each of them needs a logger and a timestamp source of its own: `defmt-rtt`
// installs the RTT one, `defmt-usb` the USB Serial/JTAG one.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

use core::fmt::Write;

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

const ECHO_PORT: u16 = 7;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let delay = Delay::init(dp.SYSTIMER);

    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let rmii = RmiiPins::new(
        pins.pin::<33>(), // TXEN
        pins.pin::<34>(), // TXD0
        pins.pin::<35>(), // TXD1
        pins.pin::<30>(), // RXD1
        pins.pin::<29>(), // RXD0
        pins.pin::<28>(), // CRS_DV
        pins.pin::<32>(), // REF_CLK (50 MHz from the PHY)
        pins.pin::<20>(), // MDC
        pins.pin::<21>(), // MDIO
    );
    let (gpio, io_mux) = pins.release();

    // SAFETY: the storages are only used by this function, for the whole run.
    let emac_buffers = unsafe { &mut *core::ptr::addr_of_mut!(EMAC_BUFFERS) };
    let mut emac = Emac::init(
        dp.EMAC,
        rmii,
        EmacConfig::default(),
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
    let link = emac.poll_link();
    writeln!(
        serial,
        "\r\nEMAC {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}, link {}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5], link.up
    )
    .ok();

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

    writeln!(serial, "waiting for DHCP…").ok();
    match net.wait_dhcp(&mut emac, 10_000) {
        Some(info) => {
            let addr = info.address.address().octets();
            let router = info.router.map(|r| r.octets()).unwrap_or([0; 4]);
            writeln!(
                serial,
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
            )
            .ok();
            writeln!(serial, "echo server on port {}", ECHO_PORT).ok();
        }
        None => {
            writeln!(serial, "DHCP timed out; falling back to 10.0.0.2/24").ok();
            net.set_ipv4(
                smoltcp::wire::Ipv4Cidr::new(smoltcp::wire::Ipv4Address::new(10, 0, 0, 2), 24),
                Some(smoltcp::wire::Ipv4Address::new(10, 0, 0, 1)),
            );
        }
    }

    // Interrupt-driven from here on: the MAC's DMA completion line and the
    // system timer's comparator both wake the core out of `wfi`.
    emac.enable_interrupts(esp32p4::interrupt::lines::EMAC);
    // SAFETY: both handlers are installed and the peripheral sources cleared.
    unsafe { esp32p4::interrupt::enable() };

    // SAFETY: handed to the socket once, below.
    let tcp_buffers = unsafe { &mut *core::ptr::addr_of_mut!(TCP_BUFFERS) };
    let socket = tcp::Socket::new(
        tcp::SocketBuffer::new(&mut tcp_buffers.rx[..]),
        tcp::SocketBuffer::new(&mut tcp_buffers.tx[..]),
    );
    let server = net.sockets_mut().add(socket);

    let mut ticks = 0u32;
    loop {
        // Park the core until the MAC or the stack's next timer has something
        // to do, then run the whole loop body.
        net.poll_wait(&mut emac, 20);

        // Serve whatever the echo socket currently has room for. The borrow of
        // the socket ends before the status report below.
        {
            let socket = net.sockets_mut().get_mut::<tcp::Socket>(server);
            if socket.is_open() {
                let mut buf = [0u8; 512];
                while socket.can_recv() {
                    let Ok(n) = socket.recv_slice(&mut buf) else {
                        break;
                    };
                    if socket.send_slice(&buf[..n]).is_err() {
                        break;
                    }
                }
            } else if !socket.is_listening() {
                // Closed by the peer (or idle): start listening again.
                socket.listen(ECHO_PORT).ok();
            }
        }

        // Report the link once in a while so a cable pull is visible.
        if ticks.is_multiple_of(50) {
            let link = emac.poll_link();
            let ip = net.ipv4_addr().map(|addr| addr.octets());
            let state = net.sockets().get::<tcp::Socket>(server).state();
            writeln!(
                serial,
                "tick {ticks} at {} ms: link {}, ip {:?}, tcp {:?}",
                delay.now_us() / 1000,
                link.up,
                ip,
                state
            )
            .ok();
        }
        ticks = ticks.wrapping_add(1);
    }
}
