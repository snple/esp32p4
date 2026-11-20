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

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::delay::Delay;
use esp32p4_hal::gpio::Pins;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::time::Duration;
use esp32p4_hal::timer::{MwdtStage, MwdtStageAction, TimerGroup0};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

const LED_PIN: u32 = 2;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
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
