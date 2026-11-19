//! Scans the I2C0 bus for devices.
//!
//! SCL and SDA are routed to two GPIOs and pulled up by the internal
//! resistors, which is enough for a short bench wire but not for a real bus —
//! fit the usual 2.2-4.7 kΩ pull-ups for that. Every address in the 7-bit
//! user range is probed with an address-only write, and the ones that
//! acknowledge are logged; a scan over an empty bus prints nothing and
//! completes in well under a second.
//!
//! Set `I2C_SCL_PIN` / `I2C_SDA_PIN` to the pins of your board.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

use esp32p4::gpio::{PinOps, Pins};
use esp32p4::i2c::{I2c0, I2cConfig};
use esp32p4::main;
use esp32p4::pac::Peripherals;
use esp32p4::usb_serial_jtag::UsbSerialJtag;

const I2C_SCL_PIN: u32 = 8;
const I2C_SDA_PIN: u32 = 9;

/// 7-bit addresses that the specification reserves.
const RESERVED: core::ops::RangeInclusive<u8> = 0x78..=0x7f;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);

    let i2c = I2c0::init(dp.I2C0, &I2cConfig::default(), &dp.HP_SYS_CLKRST);

    let mut scl = pins.pin::<I2C_SCL_PIN>();
    let mut sda = pins.pin::<I2C_SDA_PIN>();
    scl.set_pull_up_enabled(true);
    sda.set_pull_up_enabled(true);
    i2c.set_pins(scl, sda);

    defmt::info!("I2C{}: scanning the 7-bit address space", I2c0::NUM);

    let mut found = 0u32;
    for address in 0x08..=0x77 {
        if i2c.probe(address).is_ok() {
            defmt::info!("device at {:#04x}", address);
            found += 1;
        }
    }

    match found {
        0 => defmt::info!(
            "no devices responded (reserved range starts at {:#04x})",
            RESERVED.start()
        ),
        _ => defmt::info!("{} device(s) found", found),
    }

    loop {
        esp32p4::delay::sleep_ms(1_000);
    }
}
