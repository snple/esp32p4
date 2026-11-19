//! Maps the same PSRAM page at a second MMU entry and reads it back.
//!
//! The PSRAM controller already maps physical pages 0..N at MMU entries 0..N.
//! This example asks [`Region::map`] for entry 512 (offset 32 MB, unused while
//! PSRAM is limited to 16 MB) so the same physical page is visible twice:
//!
//! ```text
//! physical page 0 ─┬─► entry 0   ─► 0x4800_0000 (Psram::base)
//!                  └─► entry 512 ─► 0x4A00_0000 (alias.cached_ptr)
//! ```
//!
//! Requires a debugger attached over JTAG (`probe-rs run --chip esp32p4 ...`).

#![no_std]
#![no_main]

use defmt_rtt as _;

defmt::timestamp!("{=u64:us}", esp32p4::delay::micros());

use esp32p4::efuse::Efuse;
use esp32p4::main;
use esp32p4::mmu::Region;
use esp32p4::pac::Peripherals;
use esp32p4::psram::{Psram, PsramConfig, PsramSize};

/// Keep the default PSRAM mapping below entry 512.
const MAPPED_PSRAM: usize = 16 * 1024 * 1024;
const ALIAS_ENTRY: u32 = 512;
const PATTERN: u32 = 0x5a5a_c3c3;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let efuse = Efuse::init(dp.EFUSE);
    let psram = Psram::init(PsramConfig {
        size: PsramSize::Bytes(MAPPED_PSRAM),
        ..PsramConfig::from_efuse(&efuse)
    })
    .expect("PSRAM init failed");

    // Write through the primary mapping, then clean the line so the alias sees
    // the value even if it bypasses the cache.
    // SAFETY: the window is mapped and at least four bytes long.
    unsafe { core::ptr::write_volatile(psram.base() as *mut u32, PATTERN) };
    esp32p4::cache::writeback(psram.base().cast(), 4);

    let alias = Region::map(0, ALIAS_ENTRY, 1).expect("MMU entry 512 must be free");
    defmt::info!(
        "psram {} pages, alias at {:#x} (uncached {:#x})",
        psram.region().pages(),
        alias.cached_addr(),
        alias.uncached_addr(),
    );

    // SAFETY: the alias points at mapped PSRAM.
    let read = unsafe { core::ptr::read_volatile(alias.uncached_ptr() as *const u32) };
    assert_eq!(read, PATTERN, "alias does not see the same physical page");
    defmt::info!("alias read {:#x}", read);

    alias.unmap();

    loop {
        core::hint::spin_loop();
    }
}
