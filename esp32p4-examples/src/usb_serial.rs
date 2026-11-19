//! Print over the USB Serial/JTAG port and echo what the host sends.
//!
//! This is the same port `espflash` uses to flash the chip, so `espflash
//! monitor` — or any terminal on the resulting `/dev/ttyACM*` — works without
//! extra wiring. The LED on GPIO2 keeps blinking, which also shows that the
//! GPIO and USB drivers share `HP_SYS_CLKRST` through a borrow.
//!
//! ```text
//! cargo run --bin usb_serial
//! ```

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

use core::fmt::Write;

use esp32p4::delay::Delay;
use esp32p4::gpio::Pins;
use esp32p4::main;
use esp32p4::pac::Peripherals;
use esp32p4::usb_serial_jtag::UsbSerialJtag;

const LED_PIN: u32 = 2;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let mut led = pins.pin::<LED_PIN>();
    led.into_output();
    let delay = Delay::init(dp.SYSTIMER);

    writeln!(serial, "\r\nESP32-P4 USB Serial/JTAG ready").ok();

    let mut ticks = 0u32;
    let mut buffer = [0u8; 64];
    loop {
        // Echo whatever the host sent. Without a host this returns 0 at once,
        // and the writes below simply go nowhere.
        let read = serial.read(&mut buffer);
        if read > 0 {
            serial.write_str("echo: ");
            serial.write(&buffer[..read]);
            serial.write_str("\r\n");
            serial.flush();
        }

        led.toggle();
        writeln!(serial, "tick {ticks}").ok();
        ticks = ticks.wrapping_add(1);
        delay.delay_ms(1000);
    }
}
