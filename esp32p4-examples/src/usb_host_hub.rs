//! Keyboard behind a USB hub on the full-speed OTG port.
//!
//! Plug a powered USB 2.0 hub into the OTG port (D+ on GPIO27, D− on GPIO26),
//! put a boot-protocol keyboard on one of its downstream ports, and press
//! keys. The stack enumerates the hub, powers its ports, resets the port the
//! keyboard is on and enumerates the keyboard.
//!
//! The chip has no VBUS switch, so the board has to feed 5 V to the connector;
//! a self-powered hub is the easiest way to get there.
//!
//! For the high-speed port, replace `UsbHostFs`/`FsResources` with
//! `UsbHostHs::new_fsls_only`/`HsResources`: the high-speed core then runs as a
//! FS/LS-only host, which is what lets a hub and its FS/LS devices work without
//! split transactions.
//!
//! ```text
//! cargo run --bin usb_host_hub
//! ```

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::delay;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;
use esp32p4_usb::host::class::hid::{Keyboard, Report};
use esp32p4_usb::host::class::hub::Hub;
use esp32p4_usb::host::error::HostError;
use esp32p4_usb::host::{AddressPool, Host};
use esp32p4_usb::{FsResources, UsbHostFs};

/// Downstream port the keyboard is expected on.
const PORT: u8 = 1;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);

    let controller = UsbHostFs::new(FsResources {
        clkrst: &dp.HP_SYS_CLKRST,
        lp_aon_clkrst: &dp.LP_AON_CLKRST,
        usb_wrap: &dp.USB_WRAP,
        gpio: &dp.GPIO,
        io_mux: &dp.IO_MUX,
    });
    let host = Host::new(&controller);
    let mut addresses = AddressPool::new();

    defmt::info!("waiting for a hub with a keyboard on port {}", PORT);

    loop {
        let hub_device = match host.enumerate() {
            Ok(device) => device,
            Err(error) => {
                defmt::debug!("no device: {:?}", error);
                delay::sleep_ms(200);
                continue;
            }
        };
        addresses.reserve(hub_device.address());

        let hub = match Hub::open(&hub_device) {
            Ok(hub) => hub,
            Err(error) => {
                defmt::warn!(
                    "device {:04x}:{:04x} is not a hub: {:?}",
                    hub_device.descriptor().vendor_id,
                    hub_device.descriptor().product_id,
                    error
                );
                delay::sleep_ms(500);
                continue;
            }
        };
        defmt::info!(
            "hub {:04x}:{:04x} with {} ports at address {}",
            hub_device.descriptor().vendor_id,
            hub_device.descriptor().product_id,
            hub.ports(),
            hub_device.address().raw()
        );

        if let Err(error) = hub.power_ports() {
            defmt::warn!("powering the ports failed: {:?}", error);
            delay::sleep_ms(500);
            continue;
        }

        if let Err(error) = hub.wait_for_connection(PORT, 5_000) {
            defmt::debug!("nothing on port {}: {:?}", PORT, error);
            delay::sleep_ms(200);
            continue;
        }

        if let Err(error) = hub.reset_port(PORT) {
            defmt::warn!("resetting port {} failed: {:?}", PORT, error);
            delay::sleep_ms(200);
            continue;
        }

        let address = match addresses.take() {
            Some(address) => address,
            None => {
                defmt::warn!("no free USB address");
                delay::sleep_ms(500);
                continue;
            }
        };
        let child = match hub.enumerate_child(PORT, address) {
            Ok(child) => child,
            Err(error) => {
                defmt::warn!("the device on port {} did not enumerate: {:?}", PORT, error);
                addresses.release(address);
                delay::sleep_ms(200);
                continue;
            }
        };

        defmt::info!(
            "device {:04x}:{:04x} at address {} ({:?})",
            child.descriptor().vendor_id,
            child.descriptor().product_id,
            child.address().raw(),
            child.speed()
        );

        if let Err(error) = child.set_configuration(1) {
            defmt::warn!("SET_CONFIGURATION failed: {:?}", error);
            addresses.release(address);
            delay::sleep_ms(500);
            continue;
        }

        let keyboard = match Keyboard::open(&child) {
            Ok(keyboard) => keyboard,
            Err(error) => {
                defmt::info!("not a boot keyboard: {:?}", error);
                addresses.release(address);
                delay::sleep_ms(500);
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
                Err(HostError::Timeout) => {}
                Err(error) => {
                    defmt::warn!("keyboard went away: {:?}", error);
                    break;
                }
            }
        }

        addresses.release(address);
        delay::sleep_ms(200);
    }
}
