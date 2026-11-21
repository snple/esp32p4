//! Explores the per-pad IO MUX table.
//!
//! Every pad has up to four direct functions, selected by
//! `IO_MUX_GPIOx.MCU_SEL`. Function 1 is the GPIO function on all of them — the
//! path through the GPIO matrix — while the others are pad-specific: JTAG,
//! UART, SPI2 flash, SD1, the GMAC PHY signals and so on.
//!
//! The example prints what a few pads can do, then selects a direct function on
//! GPIO43 and reads it back, which shows that the write reaches the pad.

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
use esp32p4_hal::gpio::pins::pads_with;
use esp32p4_hal::gpio::{AnalogFunction, MuxFunction, PinOps, Pins};
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

const LED_PIN: u32 = 2;

/// Prints every direct function of one pad.
///
/// Generic over the pin rather than over its number: the per-pin register
/// access is generated for concrete pins only, so drivers and helpers take
/// `impl PinOps`.
fn describe(pin: impl PinOps) {
    let pad = pin.pad();
    defmt::info!(
        "GPIO{}: lp={} drv={:?} mux={}",
        pin.number(),
        pad.lp,
        pad.reset_drive_strength,
        pin.mux_function()
    );
    for (index, function) in pad.functions().iter().enumerate() {
        match function {
            Some(function) => defmt::info!("  function {}: {}", index, function.name()),
            None => defmt::info!("  function {}: -", index),
        }
    }
    for (index, analog) in pad.analog().iter().enumerate() {
        match analog {
            Some(AnalogFunction::Touch(_) | AnalogFunction::Adc1(_) | AnalogFunction::Adc2(_)) => {
                defmt::info!(
                    "  analog {}: {} {}",
                    index,
                    analog.unwrap().name(),
                    analog.unwrap().channel().unwrap()
                )
            }
            Some(analog) => defmt::info!("  analog {}: {}", index, analog.name()),
            None => defmt::info!("  analog {}: -", index),
        }
    }
    defmt::info!("  usb={} gmac={}", pad.is_usb_phy(), pad.is_gmac_phy());
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);

    // Pads with nothing but GPIO plus one special function.
    describe(pins.pin::<LED_PIN>());
    // UART1 RTS/CTS and SPI2 data lines share these pads.
    describe(pins.pin::<8>());
    // Only GPIO through the matrix.
    describe(pins.pin::<20>());
    // UART0 TX and RX have their own pads, so a console needs no matrix route.
    describe(pins.pin::<37>());
    // SD1 clock, shared with a GMAC PHY output.
    describe(pins.pin::<43>());
    // USB full-speed D-: one of the pads that no matrix route can replace.
    describe(pins.pin::<26>());
    // ADC1 channel 0 and an EMAC PHY pad, for the analog and GMAC cases.
    describe(pins.pin::<16>());
    describe(pins.pin::<33>());

    // Which pads can drive UART0 TX directly?
    for pad in pads_with(MuxFunction::Uart0Txd) {
        defmt::info!("direct UART0 TX on GPIO{}", pad.number);
    }

    // Select a direct function and read it back.
    let mut sd_clk = pins.pin::<43>();
    sd_clk.set_mux_function(MuxFunction::Sd1Cclk);
    defmt::info!(
        "GPIO43 now {} (MCU_SEL={})",
        sd_clk.mux_function().map(|f| f.name()),
        sd_clk.function()
    );

    // Back to the GPIO matrix, so the pad is a normal output again.
    sd_clk.set_mux_function(MuxFunction::Gpio);
    defmt::info!(
        "GPIO43 restored to {}",
        sd_clk.mux_function().map(|f| f.name())
    );

    let mut led = pins.pin::<LED_PIN>();
    led.into_output();
    defmt::info!("LED pad function: {}", led.mux_function().map(|f| f.name()));

    loop {
        led.toggle();
        delay::delay_ms(500);
    }
}
