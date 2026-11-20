//! Prints the chip's die temperature once per second.
//!
//! The sensor picks its measurement range from the reading, so the log also
//! shows which range the current value was taken in: the narrower the range,
//! the better the accuracy.

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
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::tsens::TemperatureSensor;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let delay = Delay::init(dp.SYSTIMER);

    let mut sensor = TemperatureSensor::init(dp.LP_TSENS, &dp.LP_PERI);

    loop {
        let millidegrees = sensor.update();
        defmt::info!(
            "{}.{:03} °C ({} m°C, range {:?})",
            millidegrees.div_euclid(1_000),
            millidegrees.rem_euclid(1_000),
            millidegrees,
            sensor.range()
        );
        delay.delay_ms(1_000);
    }
}
