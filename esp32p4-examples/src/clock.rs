//! Prints the clock tree over RTT.
//!
//! Requires a debugger attached over JTAG (`probe-rs run --chip esp32p4 ...`);
//! RTT data is not visible over the UART used by `espflash monitor`.

#![no_std]
#![no_main]

use defmt_rtt as _;

defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

use esp32p4::clock::{ClockTree, RefClock};
use esp32p4::delay::Delay;
use esp32p4::main;
use esp32p4::pac::Peripherals;
use esp32p4::timer::TimerGroup0;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
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

    let delay = Delay::init(dp.SYSTIMER);
    loop {
        delay.delay_ms(1000);
        defmt::debug!("cpu {} MHz", clocks.cpu_hz() / 1_000_000);
    }
}
