//! Log over the USB Serial/JTAG port with `defmt`.
//!
//! The HAL owns the global logger (`defmt-usb-logger`), so unlike the RTT
//! example this needs nothing but the USB cable `espflash` already uses:
//!
//! ```text
//! cargo run --bin defmt_usb --features defmt-usb
//! ```

#![no_std]
#![no_main]

defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

use esp32p4::delay::Delay;
use esp32p4::main;
use esp32p4::pac::Peripherals;
use esp32p4::usb_serial_jtag::UsbSerialJtag;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    // Nothing is logged before this line: the logger drops frames while the
    // USB clock gate is still closed.
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let delay = Delay::init(dp.SYSTIMER);

    defmt::info!("hello from ESP32-P4 over USB Serial/JTAG");

    let mut ticks = 0u32;
    loop {
        defmt::debug!("tick {}", ticks);
        ticks = ticks.wrapping_add(1);
        delay.delay_ms(1000);
    }
}
