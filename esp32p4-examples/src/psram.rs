//! Initializes the in-package PSRAM and uses it as a heap.
//!
//! Success is indicated by a slow blink; the PSRAM bring-up is logged over the
//! USB Serial/JTAG port. If the read-back check fails, the panic handler parks
//! the core with the LED off.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

extern crate alloc;

use alloc::vec::Vec;

use esp32p4_hal::delay::Delay;
use esp32p4_hal::efuse::Efuse;
use esp32p4_hal::gpio::Pins;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::psram::{Psram, PsramAllocator, PsramConfig};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// LED pin (GPIO2).
const LED_PIN: u32 = 2;

#[global_allocator]
static HEAP: PsramAllocator = PsramAllocator::empty();

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let mut led = pins.pin::<LED_PIN>();
    led.into_output();

    let efuse = Efuse::init(dp.EFUSE);
    let psram = Psram::init(PsramConfig::from_efuse(&efuse)).expect("PSRAM init failed");
    HEAP.init(&psram);

    // Allocate from PSRAM to prove the heap works end to end.
    let mut values: Vec<u32> = Vec::with_capacity(1024);
    for i in 0..1024 {
        values.push(i);
    }
    let sum: u32 = values.iter().sum();
    assert_eq!(sum, (0..1024).sum(), "PSRAM heap corrupted");

    // `Allocator` is stable since Rust 1.100, so a collection can also be
    // pinned to PSRAM explicitly.
    let mut explicit: Vec<u64, &PsramAllocator> = Vec::new_in(&HEAP);
    explicit.extend(0..128u64);
    assert_eq!(explicit.len(), 128);

    let delay = Delay::init(dp.SYSTIMER);
    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
