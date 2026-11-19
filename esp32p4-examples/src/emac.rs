//! Bring up the Ethernet MAC over RMII and report the link.
//!
//! The board needs an RMII PHY wired to the pins below (the ESP32-P4 function
//! board uses a LAN8720 on GPIO28..GPIO36). Without a PHY the driver still
//! runs: auto-negotiation times out and the loop keeps polling.
//!
//! ```text
//! cargo run --bin emac
//! ```

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

use esp32p4::delay::Delay;
use esp32p4::emac::{Emac, EmacBuffers, EmacConfig, EmacResources, RmiiPins};
use esp32p4::gpio::Pins;
use esp32p4::main;
use esp32p4::pac::Peripherals;
use esp32p4::usb_serial_jtag::UsbSerialJtag;

/// Eight receive slots keep a burst of frames while the loop is busy printing.
static mut BUFFERS: EmacBuffers<8, 4> = EmacBuffers::new();

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

    // SAFETY: `BUFFERS` is only used here, for the whole lifetime of the driver.
    let buffers = unsafe { &mut *core::ptr::addr_of_mut!(BUFFERS) };

    let mut emac = Emac::init(
        dp.EMAC,
        rmii,
        EmacConfig::default(),
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
    writeln!(
        serial,
        "\r\nEMAC up, MAC {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
    .ok();

    // The PHY id tells us whether MDIO works at all; a busy-wait timeout shows
    // up as 0xffff/0xffff.
    let id_hi = emac.mdio_read(0, 2);
    let id_lo = emac.mdio_read(0, 3);
    writeln!(serial, "PHY id {:04x}:{:04x}", id_hi, id_lo).ok();

    let link = emac.autonegotiate();
    if link.up {
        writeln!(serial, "link up: {:?} {:?}", link.speed, link.duplex).ok();
    } else {
        writeln!(serial, "no link (auto-negotiation timed out)").ok();
    }

    let mut ticks = 0u32;
    let mut frames = 0u32;
    loop {
        while let Some(frame) = emac.receive() {
            let len = frame.len();
            let mut header = [0u8; 14];
            header.copy_from_slice(&frame[..14]);
            emac.release_frame();

            frames += 1;
            writeln!(
                serial,
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
            )
            .ok();
        }

        if ticks.is_multiple_of(5) {
            let link = emac.poll_link();

            // Broadcast ARP request for 10.0.0.1, so the transmit path and the
            // ring bookkeeping get exercised without a network stack.
            if link.up && emac.transmit_ready() {
                let mac = emac.mac_address();
                let mut frame = [0u8; 42];
                frame[0..6].copy_from_slice(&[0xff; 6]); // dst: broadcast
                frame[6..12].copy_from_slice(&mac); // src
                frame[12..14].copy_from_slice(&[0x08, 0x06]); // ARP
                frame[14..16].copy_from_slice(&[0x00, 0x01]); // Ethernet/IPv4
                frame[16..18].copy_from_slice(&[0x08, 0x00]); // 6-byte addresses
                frame[18..20].copy_from_slice(&[0x06, 0x04]); // request
                frame[22..28].copy_from_slice(&mac); // sender MAC
                frame[28..32].copy_from_slice(&[10, 0, 0, 2]); // sender IP
                frame[38..42].copy_from_slice(&[10, 0, 0, 1]); // target IP
                let _ = emac.transmit(&frame);
            }

            writeln!(
                serial,
                "tick {ticks}: link {}, frames {frames}, tx ready {}",
                link.up,
                emac.transmit_ready(),
            )
            .ok();
        }

        ticks = ticks.wrapping_add(1);
        delay.delay_ms(200);
    }
}
