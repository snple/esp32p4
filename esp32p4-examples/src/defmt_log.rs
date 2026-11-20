//! Logs over RTT with `defmt`.
//!
//! This is the one example wired for RTT; every other example takes the
//! default `defmt-usb` transport. The HAL emits its own `debug`/`info` messages
//! on the same channel, so its bring-up steps appear alongside the
//! application's output.
//!
//! Requires a debugger attached over JTAG (`probe-rs run --chip esp32p4 ...`);
//! RTT data is not visible over the UART used by `espflash monitor`.
//!
//! Build it with `--no-default-features --features defmt-rtt`:
//!
//! ```text
//! cargo run --bin defmt_log --no-default-features --features defmt-rtt
//! ```

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;

defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::delay::Delay;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    defmt::info!("hello from ESP32-P4");

    let delay = Delay::init(dp.SYSTIMER);
    let mut ticks = 0u32;
    loop {
        defmt::debug!("tick {}", ticks);
        ticks = ticks.wrapping_add(1);
        delay.delay_ms(1000);
    }
}
