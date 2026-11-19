//! Virtual serial port over the full-speed USB OTG core.
//!
//! The prompt appears as a new `/dev/ttyACM*` (Linux) or COM port (Windows)
//! after the device enumerates; whatever is typed there is echoed back.
//!
//! Use a second USB cable or a hub: the USB Serial/JTAG port that `espflash`
//! uses is a different controller (GPIO24/25), while this one drives the pads on
//! GPIO26/27.
//!
//! ```text
//! cargo run --bin usb_cdc
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

use esp32p4::main;
use esp32p4::pac::Peripherals;
use esp32p4_usb::class::cdc_acm::CdcAcm;
use esp32p4_usb::p4::{FsResources, UsbFs};
use esp32p4_usb::{UsbBusAllocator, UsbDeviceBuilder};

/// Bulk packet size of a full-speed endpoint.
const MAX_PACKET: u16 = 64;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    let allocator = UsbBusAllocator::new(UsbFs::new(FsResources {
        clkrst: &dp.HP_SYS_CLKRST,
        lp_aon_clkrst: &dp.LP_AON_CLKRST,
        usb_wrap: &dp.USB_WRAP,
        gpio: &dp.GPIO,
        io_mux: &dp.IO_MUX,
    }));

    let mut serial = CdcAcm::new(&allocator, MAX_PACKET);
    let mut device = UsbDeviceBuilder::new(&allocator, 0x303a, 0x1001)
        .manufacturer("Espressif")
        .product("ESP32-P4 CDC (full speed)")
        .serial_number("p4-fs")
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

        // Echo whatever the host sent. `read` returns `WouldBlock` when nothing
        // arrived, which is the common case.
        if let Ok(n) = serial.read(&mut buffer) {
            serial.write_all(&buffer[..n], 1_000_000);
        }
    }
}
