//! Initializes the in-package PSRAM and uses it as a heap.
//!
//! Success is indicated by a slow blink; if the bring-up or the read-back
//! check fails, the panic handler parks the core with the LED off.

#![no_std]
#![no_main]

// `--features defmt` turns on the HAL's log statements for every binary, so
// each of them needs a logger and a timestamp source of its own: `defmt-rtt`
// installs the RTT one, `defmt-usb` the USB Serial/JTAG one.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

extern crate alloc;

use alloc::vec::Vec;

use esp32p4::delay::Delay;
use esp32p4::efuse::Efuse;
use esp32p4::gpio::Pins;
use esp32p4::main;
use esp32p4::pac::Peripherals;
use esp32p4::psram::{Psram, PsramAllocator, PsramConfig};

/// LED pin (GPIO2).
const LED_PIN: u32 = 2;

#[global_allocator]
static HEAP: PsramAllocator = PsramAllocator::empty();

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
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
