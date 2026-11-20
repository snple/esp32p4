//! ADC report: factory calibration, then a sweep over every ADC pad.
//!
//! Prints whether the chip carries eFuse calibration data, then samples each
//! ADC1 pad (GPIO16-GPIO23) and ADC2 pad (GPIO49-GPIO53) at every attenuation
//! and prints the raw code and the millivolt estimate. Nothing needs to be
//! wired up: a floating pad reads noise, and grounding one pad should pin its
//! reading to 0 (plus the per-channel offset).
//!
//! `read_raw` starts one conversion, polls for the done flag and reads the
//! result, so the numbers also prove the one-shot controller works.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::adc::{Adc, AdcInput, Attenuation};
use esp32p4_hal::gpio::{PinOps, Pins};
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let adc = Adc::init(dp.ADC, dp.LP_ADC, &dp.HP_SYS_CLKRST, &dp.LP_PERI);

    defmt::info!(
        "ADC: factory calibration {}",
        if adc.calibration().is_available() {
            "present"
        } else {
            "missing, using the nominal full scale"
        }
    );

    // `Pin<N>` is a distinct type per pad, so the sweep is spelled out.
    report(&adc, &mut pins.pin::<16>(), "ADC1");
    report(&adc, &mut pins.pin::<17>(), "ADC1");
    report(&adc, &mut pins.pin::<18>(), "ADC1");
    report(&adc, &mut pins.pin::<19>(), "ADC1");
    report(&adc, &mut pins.pin::<20>(), "ADC1");
    report(&adc, &mut pins.pin::<21>(), "ADC1");
    report(&adc, &mut pins.pin::<22>(), "ADC1");
    report(&adc, &mut pins.pin::<23>(), "ADC1");
    report(&adc, &mut pins.pin::<49>(), "ADC2");
    report(&adc, &mut pins.pin::<50>(), "ADC2");
    report(&adc, &mut pins.pin::<51>(), "ADC2");
    report(&adc, &mut pins.pin::<52>(), "ADC2");
    report(&adc, &mut pins.pin::<53>(), "ADC2");

    defmt::info!("done");
    loop {
        esp32p4_hal::delay::sleep_ms(1000);
    }
}

/// Sample one pad at every attenuation, twice, so a stale result shows up.
fn report(adc: &Adc, pin: &mut impl PinOps, unit: &str) {
    for attenuation in Attenuation::ALL {
        let input = AdcInput::new(pin, attenuation);
        let raw = adc.read_raw(&input);
        let millivolts = adc.read_millivolts(&input);
        let again = adc.read_raw(&input);
        defmt::info!(
            "{} GPIO{} ch{} {}: raw {} = {} mV (again {})",
            unit,
            pin.number(),
            input.channel(),
            attenuation,
            raw,
            millivolts,
            again
        );
    }
}
