//! Virtual serial port over the high-speed USB OTG core.
//!
//! Same echo programme as `usb_cdc`, but on `USB_OTG20` and its internal UTMI+
//! PHY, so bulk transfers use 512-byte packets. The chip still has to be
//! connected to a high-speed capable host port for the 480 Mbit/s link; on a
//! full-speed port the controller falls back to 12 Mbit/s.
//!
//! ```text
//! cargo run --bin usb_cdc_hs
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

use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;
use esp32p4_usb::class::cdc_acm::CdcAcm;
use esp32p4_usb::p4::{HsResources, UsbHs};
use esp32p4_usb::{UsbBusAllocator, UsbDeviceBuilder};

/// Bulk packet size of a high-speed endpoint.
const MAX_PACKET: u16 = 512;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    let allocator = UsbBusAllocator::new(UsbHs::new(HsResources {
        clkrst: &dp.HP_SYS_CLKRST,
        lp_aon_clkrst: &dp.LP_AON_CLKRST,
        hp_sys: &dp.HP_SYS,
        lp_sys: &dp.LP_SYS,
    }));

    let mut serial = CdcAcm::new(&allocator, MAX_PACKET);
    let mut device = UsbDeviceBuilder::new(&allocator, 0x303a, 0x1002)
        .manufacturer("Espressif")
        .product("ESP32-P4 CDC (high speed)")
        .serial_number("p4-hs")
        .build();

    let mut was_configured = false;
    let mut buffer = [0u8; MAX_PACKET as usize];

    loop {
        device.poll(&mut [&mut serial]);

        let configured = device.is_configured();
        if configured && !was_configured {
            serial.write_all(b"\r\nESP32-P4 USB CDC ready\r\n", 1_000_000);
        }
        was_configured = configured;

        if let Ok(n) = serial.read(&mut buffer) {
            serial.write_all(&buffer[..n], 1_000_000);
        }
    }
}
