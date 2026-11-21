//! Prints the clock tree over the USB Serial/JTAG port.
//!
//! ```text
//! cargo run --bin clock
//! ```

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::clock::{ClockTree, RefClock};
use esp32p4_hal::delay::Delay;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::timer::TimerGroup0;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let clocks = ClockTree::init(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST);

    // The drivers take the clock tree where they used to take the HP registers:
    // `ClockTree` derefs to `HP_SYS_CLKRST`'s register block.
    let _timer_group = TimerGroup0::init(dp.TIMG0, &clocks);

    defmt::info!(
        "root {:?} {} MHz: cpu {} MHz, mem {} MHz, sys {} MHz, apb {} MHz",
        clocks.root_source(),
        clocks.root_hz() / 1_000_000,
        clocks.cpu_hz() / 1_000_000,
        clocks.mem_hz() / 1_000_000,
        clocks.sys_hz() / 1_000_000,
        clocks.apb_hz() / 1_000_000,
    );
    defmt::info!(
        "lp slow {:?} {} Hz, lp fast {:?} {} Hz, lp peri {} Hz",
        clocks.lp_slow_source(),
        clocks.lp_slow_hz(),
        clocks.lp_fast_source(),
        clocks.lp_fast_hz(),
        clocks.lp_peri_hz(),
    );
    for clk in RefClock::ALL {
        defmt::info!(
            "{:?}: {} Hz (enabled: {})",
            clk,
            clocks.ref_hz(clk),
            clocks.ref_enabled(clk)
        );
    }

    let delay = Delay::init();
    loop {
        delay.delay_ms(1000);
        defmt::debug!("cpu {} MHz", clocks.cpu_hz() / 1_000_000);
    }
}
