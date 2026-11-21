//! Boot-protocol keyboard on the full-speed USB OTG port.
//!
//! Plug a keyboard into the USB OTG port (D+ on GPIO27, D− on GPIO26) and press
//! keys; the HID usage codes appear on the USB Serial/JTAG console. The controller
//! enumerates whatever is attached and waits for the device to be plugged in
//! again if it goes away.
//!
//! The port has no VBUS switch of its own: the board has to feed 5 V to the
//! connector. On a board without one, power the keyboard through a powered hub.
//!
//! ```text
//! cargo run --bin usb_host_hid
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
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;
use esp32p4_usb::FsResources;
use esp32p4_usb::UsbHostFs;
use esp32p4_usb::host::Host;
use esp32p4_usb::host::class::hid::{Keyboard, Report};
use esp32p4_usb::host::error::HostError;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    // The host may use the same GPIO26/27 pads as the device example did; the
    // driver configures them for the PHY, so only the clock and wrappers differ.
    let controller = UsbHostFs::new(FsResources {
        clkrst: &dp.HP_SYS_CLKRST,
        lp_aon_clkrst: &dp.LP_AON_CLKRST,
        usb_wrap: &dp.USB_WRAP,
        gpio: &dp.GPIO,
        io_mux: &dp.IO_MUX,
    });
    let host = Host::new(&controller);

    defmt::info!("waiting for a device on the full-speed OTG port");

    loop {
        let device = match host.enumerate() {
            Ok(device) => device,
            Err(error) => {
                // Nothing attached, or a device that does not enumerate.
                defmt::debug!("no device: {:?}", error);
                delay::delay_ms(100);
                continue;
            }
        };

        let descriptor = device.descriptor();
        defmt::info!(
            "device {:04x}:{:04x} at address {}",
            descriptor.vendor_id,
            descriptor.product_id,
            device.address().raw()
        );

        if let Err(error) = device.set_configuration(1) {
            defmt::warn!("SET_CONFIGURATION failed: {:?}", error);
            continue;
        }

        let keyboard = match Keyboard::open(&device) {
            Ok(keyboard) => keyboard,
            Err(error) => {
                defmt::info!("not a boot keyboard: {:?}", error);
                delay::delay_ms(500);
                continue;
            }
        };

        defmt::info!("typing on endpoint {}", keyboard.endpoint().raw());

        let mut previous = Report::default();
        loop {
            match keyboard.read() {
                Ok(report) => {
                    for key in report.new_keys(&previous) {
                        defmt::info!("key {} with modifiers {}", key, report.modifiers);
                    }
                    previous = report;
                }
                // Nothing changed since the last report.
                Err(HostError::Timeout) => {}
                Err(error) => {
                    defmt::warn!("keyboard went away: {:?}", error);
                    break;
                }
            }
        }
    }
}
