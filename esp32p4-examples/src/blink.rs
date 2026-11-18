//! Blink an LED on GPIO2.
//!
//! Build and flash from this directory:
//!
//! ```text
//! cargo run --bin blink
//! ```

#![no_std]
#![no_main]

// `--features defmt` turns on the HAL's log statements for every binary, so
// each of them needs a logger and a timestamp source of its own.
#[cfg(feature = "defmt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

use esp32p4::delay::Delay;
use esp32p4::gpio::Pins;
use esp32p4::main;
use esp32p4::pac::Peripherals;

const LED_PIN: u32 = 2;
const BLINK_MS: u32 = 500;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, dp.HP_SYS_CLKRST);
    let mut led = pins.pin::<LED_PIN>();
    led.into_output();
    let delay = Delay::init(dp.SYSTIMER);

    loop {
        led.toggle();
        delay.delay_ms(BLINK_MS);
    }
}
