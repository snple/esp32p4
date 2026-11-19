//! Runs code from the 8 KB HP scratchpad and from SRAM.
//!
//! `#[tcmfunc]` and `#[ramfunc]` place a function in a section the 2nd-stage
//! bootloader copies into RAM; the printout shows where each one ended up.
//!
//! Requires a debugger attached over JTAG (`probe-rs run --chip esp32p4 ...`);
//! RTT data is not visible over the UART used by `espflash monitor`.

#![no_std]
#![no_main]

use defmt_rtt as _;

defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

use esp32p4::delay::Delay;
use esp32p4::main;
use esp32p4::pac::Peripherals;
use esp32p4::tcm;
use esp32p4::{ramfunc, tcmdata, tcmfunc};

/// Runs from the scratchpad at 0x3010_0000.
#[tcmfunc]
fn tcm_work(rounds: u32) -> u32 {
    let mut acc = 0u32;
    for i in 0..rounds {
        acc = acc.wrapping_mul(31).wrapping_add(i);
    }
    acc
}

/// Runs from the SRAM region, so it survives a flash cache suspension.
#[ramfunc]
fn ram_work(rounds: u32) -> u32 {
    let mut acc = 1u32;
    for i in 0..rounds {
        acc = acc.wrapping_mul(17).wrapping_add(i);
    }
    acc
}

/// A table the scratchpad code reads: it lives next to the code, not in flash.
#[tcmdata]
static LUT: [u32; 8] = [0, 1, 4, 9, 16, 25, 36, 49];

/// A normal function, for comparison: this one stays in flash.
fn flash_work(rounds: u32) -> u32 {
    let mut acc = 2u32;
    for i in 0..rounds {
        acc = acc.wrapping_mul(13).wrapping_add(i);
    }
    acc
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    defmt::info!(
        "tcm_work at {=usize:#x} (in scratchpad: {})",
        tcm_work as *const () as usize,
        tcm::contains(tcm_work as *const () as usize)
    );
    defmt::info!(
        "ram_work at {=usize:#x} (in scratchpad: {})",
        ram_work as *const () as usize,
        tcm::contains(ram_work as *const () as usize)
    );
    defmt::info!(
        "flash_work at {=usize:#x} (in scratchpad: {})",
        flash_work as *const () as usize,
        tcm::contains(flash_work as *const () as usize)
    );

    defmt::info!(
        "LUT at {=usize:#x} (in scratchpad: {})",
        &raw const LUT as usize,
        tcm::contains(&raw const LUT as usize)
    );

    let used = tcm::used_range();
    defmt::info!(
        "scratchpad in use: {=usize:#x}..{=usize:#x} of {=usize:#x}..{=usize:#x}",
        used.start,
        used.end,
        tcm::BASE,
        tcm::BASE + tcm::SIZE
    );

    tcm::init_parity();
    defmt::info!("scratchpad parity check on: {}", tcm::parity_enabled());

    let delay = Delay::init(dp.SYSTIMER);
    let mut round = 0u32;
    loop {
        defmt::info!(
            "{}: tcm {} ram {} flash {}",
            round,
            tcm_work(1000),
            ram_work(1000),
            flash_work(1000)
        );
        round = round.wrapping_add(1);
        delay.delay_ms(1000);
    }
}
