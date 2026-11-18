//! Blink an LED on GPIO2.
//!
//! Build and flash from this directory:
//!
//! ```text
//! cargo run --bin blink
//! ```

#![no_std]
#![no_main]

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
