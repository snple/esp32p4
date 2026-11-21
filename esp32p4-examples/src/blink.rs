//! Blink an LED on GPIO2.
//!
//! Build and flash from this directory:
//!
//! ```text
//! cargo run --bin blink
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
use esp32p4_hal::gpio::Pins;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

const LED_PIN: u32 = 2;
const BLINK_MS: u32 = 1000;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let mut led = pins.pin::<LED_PIN>();
    led.into_output();

    defmt::info!("hello from ESP32-P4 over USB Serial/JTAG");

    let mut ticks = 0u32;
    loop {
        defmt::debug!("tick {}", ticks);
        ticks = ticks.wrapping_add(1);
        led.toggle();
        delay::delay_ms(BLINK_MS);
    }
}
