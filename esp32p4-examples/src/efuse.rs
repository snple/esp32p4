//! Prints the chip version, factory MAC and flash/PSRAM parameters read from
//! eFuse over the USB Serial/JTAG port.
//!
//! ```text
//! cargo run --bin efuse
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

use esp32p4_hal::clock::ClockTree;
use esp32p4_hal::efuse::Efuse;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let efuse = Efuse::init(dp.EFUSE);

    let version = efuse.chip_version();
    let block = efuse.block_version();
    defmt::info!(
        "wafer v{}.{} (rev {}), cpll {} MHz, block v{}.{}, pkg {}",
        version.major(),
        version.minor(),
        version.revision(),
        version.cpll_hz() / 1_000_000,
        block.major(),
        block.minor(),
        efuse.package_version(),
    );
    defmt::info!(
        "mac {=[u8]:02x}, flash {:?}, page-size code {}, ecc {}, tpuw {} ms",
        efuse.mac(),
        efuse.flash_type(),
        efuse.flash_page_size_code(),
        efuse.flash_ecc_enabled(),
        efuse.flash_power_up_delay_ms(),
    );
    defmt::info!(
        "psram cap code {}, vendor {}, temp code {}",
        efuse.psram_capacity_code(),
        efuse.psram_vendor(),
        efuse.temperature_code(),
    );

    // The eFuse version decides whether the CPLL runs at 360 or 400 MHz, so the
    // tree no longer needs a hand-written `Sources` table.
    let clocks = ClockTree::init_from_efuse(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST, &efuse);
    defmt::info!(
        "root {} MHz, cpu {} MHz, apb {} MHz",
        clocks.root_hz() / 1_000_000,
        clocks.cpu_hz() / 1_000_000,
        clocks.apb_hz() / 1_000_000,
    );

    loop {
        core::hint::spin_loop();
    }
}
