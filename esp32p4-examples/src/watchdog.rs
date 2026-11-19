//! Let the main system watchdog reset the board when the loop stalls.
//!
//! Stage 0 of `TIMG0`'s MWDT is set to one second and to "reset the system".
//! The loop feeds it every 100 ms while blinking; delete the `feed()` call and
//! the chip resets after a second.
//!
//! ```text
//! cargo run --bin watchdog
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

use esp32p4::delay::Delay;
use esp32p4::gpio::Pins;
use esp32p4::main;
use esp32p4::pac::Peripherals;
use esp32p4::time::Duration;
use esp32p4::timer::{MwdtStage, MwdtStageAction, TimerGroup0};

const LED_PIN: u32 = 2;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let mut led = pins.pin::<LED_PIN>();
    led.into_output();
    let delay = Delay::init(dp.SYSTIMER);

    let group = TimerGroup0::init(dp.TIMG0, &dp.HP_SYS_CLKRST);

    // One second at 40 MHz. Anything slower than that resets the chip.
    let wdt = &group.wdt;
    wdt.set_stage_action(MwdtStage::Stage0, MwdtStageAction::ResetSystem);
    wdt.set_timeout(MwdtStage::Stage0, Duration::from_secs(1));
    wdt.enable();

    loop {
        led.toggle();
        wdt.feed();
        delay.delay_ms(100);
    }
}
