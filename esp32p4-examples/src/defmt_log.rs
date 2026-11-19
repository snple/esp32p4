//! Logs over RTT with `defmt`.
//!
//! The HAL emits its own `debug`/`info` messages on the same channel, so the
//! PSRAM bring-up can be watched step by step.
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

defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

use esp32p4::delay::Delay;
use esp32p4::main;
use esp32p4::pac::Peripherals;
use esp32p4::psram::{Psram, PsramConfig};

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    defmt::info!("hello from ESP32-P4");

    match Psram::init(PsramConfig::default()) {
        Ok(psram) => defmt::info!("PSRAM: {} bytes available", psram.size()),
        Err(err) => defmt::error!("PSRAM: init failed: {}", err),
    }

    let delay = Delay::init(dp.SYSTIMER);
    let mut ticks = 0u32;
    loop {
        defmt::debug!("tick {}", ticks);
        ticks = ticks.wrapping_add(1);
        delay.delay_ms(1000);
    }
}
