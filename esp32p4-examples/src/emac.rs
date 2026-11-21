//! Bring up the Ethernet MAC over RMII and report what the PHY says.
//!
//! The pin map below is the ESP32-P4 function board: an IP101GRI answers at
//! MDIO address 1 (`PHY_AD0` strapped high, `PHY_AD3` strapped low). Its reset
//! is wired to GPIO51 — the PHY stays off MDIO (every read returns `0xffff`)
//! until that pad is driven high.
//!
//! Everything is logged with `defmt`. That matters on the USB Serial/JTAG
//! port: the `defmt-usb` transport drops frames while the host is not reading,
//! whereas a raw `writeln!` into `UsbSerialJtag` spins until the FIFO drains
//! and stalls the whole example when no monitor is attached.
//!
//! The example scans MDIO, dumps the clause-22 registers, restarts
//! auto-negotiation, then polls the link while sending broadcast ARP requests
//! and printing every frame that arrives.
//!
//! ```text
//! cargo run --bin emac
//! ```

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::delay;
use esp32p4_hal::emac::{Emac, EmacBuffers, EmacConfig, EmacResources, RmiiPins};
use esp32p4_hal::gpio::Pins;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// MDIO address the IP101GRI is strapped to: `PHY_AD0` high, `PHY_AD3` low.
const PHY_ADDRESS: u8 = 1;
/// Active-low PHY reset on the function board.
const PHY_RESET_PIN: u32 = 51;

/// Eight receive slots keep a burst of frames while the loop is busy printing.
static mut BUFFERS: EmacBuffers<8, 4> = EmacBuffers::new();

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    // The USB Serial/JTAG logger silently drops everything until its clock
    // gate is open, so the port is opened before the first log statement.
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);

    // Reset the PHY before touching the MAC: assert nRST, then give the
    // oscillator time to start.
    let mut phy_reset = pins.pin::<PHY_RESET_PIN>();
    phy_reset.into_output().set_low();
    delay::delay_ms(100);
    phy_reset.set_high();
    delay::delay_ms(300);

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

    // SAFETY: `BUFFERS` is only used here, for the whole lifetime of the driver.
    let buffers = unsafe { &mut *core::ptr::addr_of_mut!(BUFFERS) };

    let mut emac = Emac::init(
        dp.EMAC,
        rmii,
        EmacConfig {
            phy_address: PHY_ADDRESS,
            ..EmacConfig::default()
        },
        buffers,
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
        "EMAC up, MAC {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0],
        mac[1],
        mac[2],
        mac[3],
        mac[4],
        mac[5]
    );

    // A PHY that never answers on MDIO shows up as 0xffff for every address;
    // the scan also catches a wrong strapping assumption.
    for address in 0..=31u8 {
        let id_hi = emac.mdio_read(address, 2);
        let id_lo = emac.mdio_read(address, 3);
        if id_hi != 0 && id_hi != 0xffff {
            defmt::info!("MDIO {}: id {:04x}:{:04x}", address, id_hi, id_lo);
        }
    }

    dump_phy(&emac, PHY_ADDRESS);

    let link = emac.autonegotiate();
    if link.up {
        defmt::info!("link up: {} {}", link.speed, link.duplex);
    } else {
        defmt::warn!("no link after auto-negotiation");
        dump_phy(&emac, PHY_ADDRESS);
    }

    let mut ticks = 0u32;
    let mut rx_frames = 0u32;
    let mut tx_frames = 0u32;
    loop {
        while let Some(frame) = emac.receive() {
            let len = frame.len();
            let mut header = [0u8; 14];
            header.copy_from_slice(&frame[..14]);
            // An ARP exchange is the cheapest proof that the peer saw our MAC:
            // every request we send should come back as a unicast reply.
            let is_arp = header[12..14] == [0x08, 0x06];
            let arp = if is_arp && len >= 42 {
                let mut body = [0u8; 42];
                body.copy_from_slice(&frame[..42]);
                Some(body)
            } else {
                None
            };
            emac.release_frame();

            rx_frames += 1;
            defmt::info!(
                "rx {} bytes, dst {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x} ethertype {:02x}{:02x}",
                len,
                header[0],
                header[1],
                header[2],
                header[3],
                header[4],
                header[5],
                header[12],
                header[13]
            );

            if let Some(arp) = arp {
                match u16::from_be_bytes([arp[20], arp[21]]) {
                    1 => defmt::info!(
                        "  ARP request: who has {}.{}.{}.{}? tell {}.{}.{}.{}",
                        arp[38],
                        arp[39],
                        arp[40],
                        arp[41],
                        arp[28],
                        arp[29],
                        arp[30],
                        arp[31]
                    ),
                    2 => defmt::info!(
                        "  ARP reply: {}.{}.{}.{} is at {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                        arp[28],
                        arp[29],
                        arp[30],
                        arp[31],
                        arp[22],
                        arp[23],
                        arp[24],
                        arp[25],
                        arp[26],
                        arp[27]
                    ),
                    operation => defmt::info!("  ARP operation {}", operation),
                }
            }
        }

        if ticks.is_multiple_of(5) {
            let link = emac.poll_link();

            // Broadcast ARP request for 10.0.0.1, so the transmit path and the
            // ring bookkeeping get exercised without a network stack.
            if link.up && emac.transmit_ready() {
                match emac.transmit(&arp_request(emac.mac_address())) {
                    Ok(()) => tx_frames += 1,
                    Err(error) => defmt::warn!("tx failed: {}", error),
                }
            }

            defmt::info!(
                "tick {}: link {} ({} {}), rx {} tx {}, dma {:#010x}",
                ticks,
                link.up,
                link.speed,
                link.duplex,
                rx_frames,
                tx_frames,
                emac.dma_status()
            );
        }

        ticks = ticks.wrapping_add(1);
        delay::delay_ms(200);
    }
}

/// Print the standard clause-22 registers plus the link bit the PHY reports.
fn dump_phy<const RX: usize, const TX: usize>(emac: &Emac<'_, RX, TX>, address: u8) {
    let bmcr = emac.mdio_read(address, 0); // control
    let bmsr = emac.mdio_read(address, 1); // status
    let anar = emac.mdio_read(address, 4); // own abilities
    let anlpar = emac.mdio_read(address, 5); // partner abilities
    defmt::info!(
        "PHY {}: BMCR {:#06x} BMSR {:#06x} ANAR {:#06x} ANLPAR {:#06x} (link {}, aneg done {})",
        address,
        bmcr,
        bmsr,
        anar,
        anlpar,
        bmsr & (1 << 2) != 0,
        bmsr & (1 << 5) != 0
    );
}

/// "Who has 10.0.0.1? Tell 10.0.0.2" — 42 bytes, no FCS.
fn arp_request(mac: [u8; 6]) -> [u8; 42] {
    let mut frame = [0u8; 42];
    frame[0..6].copy_from_slice(&[0xff; 6]); // dst: broadcast
    frame[6..12].copy_from_slice(&mac); // src
    frame[12..14].copy_from_slice(&[0x08, 0x06]); // ARP
    frame[14..16].copy_from_slice(&[0x00, 0x01]); // hardware type: Ethernet
    frame[16..18].copy_from_slice(&[0x08, 0x00]); // protocol type: IPv4
    frame[18..20].copy_from_slice(&[0x06, 0x04]); // 6-byte hardware, 4-byte protocol
    frame[20..22].copy_from_slice(&[0x00, 0x01]); // operation: request
    frame[22..28].copy_from_slice(&mac); // sender MAC
    frame[28..32].copy_from_slice(&[10, 0, 0, 2]); // sender IP
    frame[38..42].copy_from_slice(&[10, 0, 0, 1]); // target IP
    frame
}
