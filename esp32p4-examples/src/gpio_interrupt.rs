//! Counts button presses on GPIO3 with a GPIO interrupt.
//!
//! The button is active-low (input with pull-up), so a falling edge is the
//! press. Each press toggles the LED on GPIO2.
//!
//! `GPIO_INTR0` is routed to CPU line 21; the ISR only acknowledges the pin and
//! sets a flag, and the main loop does the visible work.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

use core::sync::atomic::{AtomicBool, Ordering};

use esp32p4::delay::Delay;
use esp32p4::gpio::{InterruptLine, InterruptType, Pin, Pins};
use esp32p4::interrupt::{self, CpuInterrupt, Level};
use esp32p4::main;
use esp32p4::pac::Peripherals;
use esp32p4::usb_serial_jtag::UsbSerialJtag;

const LED_PIN: u32 = 2;
const BUTTON_PIN: u32 = 3;
const BUTTON_LINE: InterruptLine = InterruptLine::Line0;
const BUTTON_CPU_LINE: u8 = 21;

static PRESSED: AtomicBool = AtomicBool::new(false);

fn button_isr() {
    // SAFETY: `Pins::init` has run and the main loop never reconfigures the pin.
    let button = unsafe { Pin::<BUTTON_PIN>::steal() };
    if button.interrupt_status() {
        button.clear_interrupt();
        PRESSED.store(true, Ordering::Relaxed);
    }
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);

    let mut led = pins.pin::<LED_PIN>();
    led.into_output();

    let mut button = pins.pin::<BUTTON_PIN>();
    button.into_input_pullup();
    button.set_interrupt_type(InterruptType::NegativeEdge);
    button.enable_interrupt(BUTTON_LINE);

    // SAFETY: the pin status was cleared by `Pins::init`, so the first
    // interrupt can only come from a new edge.
    let handler = unsafe {
        interrupt::claim(
            BUTTON_LINE.source(),
            CpuInterrupt::new(BUTTON_CPU_LINE),
            Level::LOW,
            button_isr,
        )
    };
    handler.enable();
    // SAFETY: the GPIO line is claimed and its handler is installed.
    unsafe { interrupt::enable() };

    let delay = Delay::init(dp.SYSTIMER);
    loop {
        if PRESSED.swap(false, Ordering::Relaxed) {
            led.toggle();
        }
        delay.delay_ms(10);
    }
}
