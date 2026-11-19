//! eFuse: chip version, factory MAC and flash/PSRAM calibration data.
//!
//! The eFuse controller latches BLOCK0..BLOCK10 into `EFUSE_RD_*` registers
//! during reset, and the 2nd-stage bootloader leaves them readable. This module
//! is read-only: it never writes the programming registers, so it cannot brick
//! a part by accident.
//!
//! # Features
//!
//! - [`ChipVersion`] with the wafer version, block version and the CPLL
//!   frequency that goes with the die (360 MHz before v3.0, 400 MHz from v3.0)
//! - Factory MAC address
//! - Flash type/page size/ECC/power-up delay (BLOCK0)
//! - PSRAM capacity/vendor and package/temperature codes (BLOCK1)
//! - [`ClockTree::init_from_efuse`](crate::clock::ClockTree::init_from_efuse) and
//!   [`PsramConfig::from_efuse`](crate::psram::PsramConfig::from_efuse) use the
//!   version to pick their defaults
//!
//! # Example
//!
//! ```ignore
//! use esp32p4::efuse::Efuse;
//!
//! let dp = Peripherals::take().unwrap();
//! let efuse = Efuse::init(dp.EFUSE);
//!
//! let version = efuse.chip_version();
//! defmt::info!(
//!     "chip v{}.{} (rev {}), cpll {} MHz",
//!     version.major(),
//!     version.minor(),
//!     version.revision(),
//!     version.cpll_hz() / 1_000_000,
//! );
//! defmt::info!("mac {:02x}", efuse.mac());
//! ```

use esp32p4_pac::{EFUSE, efuse};

use crate::clock::{CPLL_360M_HZ, CPLL_400M_HZ};

// BLOCK1 word 2 (`EFUSE_RD_MAC_SYS2_REG`). The TRM marks the word reserved and
// the SVD therefore only exposes it as one value; the layout below is the one
// ESP-IDF's eFuse table uses for every ESP32-P4 revision.
const WAFER_VERSION_MINOR_SHIFT: u32 = 0;
const WAFER_VERSION_MINOR_MASK: u32 = 0x0000_000f;
const WAFER_VERSION_MAJOR_LO_SHIFT: u32 = 4;
const WAFER_VERSION_MAJOR_LO_MASK: u32 = 0x0000_0003;
const DISABLE_WAFER_VERSION_MAJOR_SHIFT: u32 = 6;
const BLK_VERSION_MINOR_SHIFT: u32 = 8;
const BLK_VERSION_MINOR_MASK: u32 = 0x0000_0007;
const BLK_VERSION_MAJOR_SHIFT: u32 = 11;
const BLK_VERSION_MAJOR_MASK: u32 = 0x0000_0003;
const PSRAM_CAP_SHIFT: u32 = 13;
const PSRAM_CAP_MASK: u32 = 0x0000_0007;
const TEMP_SHIFT: u32 = 16;
const TEMP_MASK: u32 = 0x0000_0003;
const PSRAM_VENDOR_SHIFT: u32 = 18;
const PSRAM_VENDOR_MASK: u32 = 0x0000_0003;
const PKG_VERSION_SHIFT: u32 = 20;
const PKG_VERSION_MASK: u32 = 0x0000_0007;
const WAFER_VERSION_MAJOR_HI_SHIFT: u32 = 23;
const WAFER_VERSION_MAJOR_HI_MASK: u32 = 0x0000_0001;

/// A chip or block version, major/minor.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ChipVersion {
    major: u8,
    minor: u8,
}

impl ChipVersion {
    /// A version from its parts.
    #[inline]
    pub const fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }

    /// The major version (3 for a v3.x die).
    #[inline]
    pub const fn major(self) -> u8 {
        self.major
    }

    /// The minor version (0 for a v3.0 die).
    #[inline]
    pub const fn minor(self) -> u8 {
        self.minor
    }

    /// `major * 100 + minor`, the encoding used by ESP-IDF (`300` for v3.0).
    #[inline]
    pub const fn revision(self) -> u16 {
        self.major as u16 * 100 + self.minor as u16
    }

    /// The CPLL frequency this revision runs at.
    ///
    /// Dies before v3.0 top out at 360 MHz; v3.0 and later configure the CPLL
    /// for 400 MHz.
    #[inline]
    pub const fn cpll_hz(self) -> u32 {
        if self.major >= 3 {
            CPLL_400M_HZ
        } else {
            CPLL_360M_HZ
        }
    }
}

/// Flash interface width, from `EFUSE_FLASH_TYPE`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashType {
    /// Four data lines (quad).
    FourLine,
    /// Eight data lines (oct).
    EightLine,
}

/// The eFuse controller, owning the PAC peripheral.
///
/// Create once with [`Efuse::init`]; all methods take `&self`, so a stolen
/// handle can also read the latched registers from an interrupt.
pub struct Efuse {
    efuse: EFUSE,
}

impl Efuse {
    /// Take ownership of the eFuse controller.
    ///
    /// The registers are already loaded by reset/bootloader, so there is no
    /// clock or timing setup to do here.
    #[inline]
    pub fn init(efuse: EFUSE) -> Self {
        Self { efuse }
    }

    /// Obtain a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other handle to the same peripheral is used
    /// concurrently. Reads are side-effect free, so this is safe to call from an
    /// ISR once [`Efuse::init`] has run.
    #[inline]
    pub const unsafe fn steal() -> crate::Steal<Self> {
        crate::Steal(Self {
            efuse: unsafe { EFUSE::steal() },
        })
    }

    /// The underlying register block.
    #[inline]
    pub fn regs(&self) -> &efuse::RegisterBlock {
        &self.efuse
    }

    /// Give the PAC peripheral back.
    #[inline]
    pub fn release(self) -> EFUSE {
        self.efuse
    }

    /// The wafer version of the die.
    #[inline]
    pub fn chip_version(&self) -> ChipVersion {
        let raw = self.efuse.rd_mac_sys2.read_bits();
        let major_lo = (raw >> WAFER_VERSION_MAJOR_LO_SHIFT) & WAFER_VERSION_MAJOR_LO_MASK;
        let major_hi = (raw >> WAFER_VERSION_MAJOR_HI_SHIFT) & WAFER_VERSION_MAJOR_HI_MASK;
        let minor = (raw >> WAFER_VERSION_MINOR_SHIFT) & WAFER_VERSION_MINOR_MASK;
        ChipVersion::new(((major_hi << 2) | major_lo) as u8, minor as u8)
    }

    /// Whether software may ignore the major version check.
    #[inline]
    pub fn disable_chip_version_check(&self) -> bool {
        (self.efuse.rd_mac_sys2.read_bits() >> DISABLE_WAFER_VERSION_MAJOR_SHIFT) & 1 != 0
    }

    /// The BLOCK2 version, i.e. the layout version of the system-data block.
    #[inline]
    pub fn block_version(&self) -> ChipVersion {
        let raw = self.efuse.rd_mac_sys2.read_bits();
        let major = (raw >> BLK_VERSION_MAJOR_SHIFT) & BLK_VERSION_MAJOR_MASK;
        let minor = (raw >> BLK_VERSION_MINOR_SHIFT) & BLK_VERSION_MINOR_MASK;
        ChipVersion::new(major as u8, minor as u8)
    }

    /// The factory MAC address, in transmission order.
    ///
    /// `RD_MAC_SYS1.MAC_1` holds the first two octets and `RD_MAC_SYS0.MAC_0`
    /// the last four, both with the first octet in the most significant byte —
    /// the layout ESP-IDF's `esp_efuse_mac_get_default` swizzles the same way.
    /// Reading the words as little-endian yields the address reversed.
    #[inline]
    pub fn mac(&self) -> [u8; 6] {
        let high = self.efuse.rd_mac_sys1.mac_1();
        let low = self.efuse.rd_mac_sys0.mac_0();
        [
            (high >> 8) as u8,
            high as u8,
            (low >> 24) as u8,
            (low >> 16) as u8,
            (low >> 8) as u8,
            low as u8,
        ]
    }

    /// Package version code (`EFUSE_PKG_VERSION`).
    #[inline]
    pub fn package_version(&self) -> u32 {
        (self.efuse.rd_mac_sys2.read_bits() >> PKG_VERSION_SHIFT) & PKG_VERSION_MASK
    }

    /// Raw PSRAM capacity code.
    ///
    /// The 3-bit encoding depends on the PSRAM vendor; the TRM does not publish
    /// the table, so this returns the code rather than a byte count.
    #[inline]
    pub fn psram_capacity_code(&self) -> u32 {
        (self.efuse.rd_mac_sys2.read_bits() >> PSRAM_CAP_SHIFT) & PSRAM_CAP_MASK
    }

    /// Raw PSRAM vendor code.
    #[inline]
    pub fn psram_vendor(&self) -> u32 {
        (self.efuse.rd_mac_sys2.read_bits() >> PSRAM_VENDOR_SHIFT) & PSRAM_VENDOR_MASK
    }

    /// Raw temperature-range code.
    #[inline]
    pub fn temperature_code(&self) -> u32 {
        (self.efuse.rd_mac_sys2.read_bits() >> TEMP_SHIFT) & TEMP_MASK
    }

    /// Flash interface width.
    #[inline]
    pub fn flash_type(&self) -> FlashType {
        if self.efuse.rd_repeat_data2.flash_type() {
            FlashType::EightLine
        } else {
            FlashType::FourLine
        }
    }

    /// Raw flash page-size code (`EFUSE_FLASH_PAGE_SIZE`).
    #[inline]
    pub fn flash_page_size_code(&self) -> u32 {
        self.efuse.rd_repeat_data2.flash_page_size()
    }

    /// Whether the bootloader uses ECC when reading the flash.
    #[inline]
    pub fn flash_ecc_enabled(&self) -> bool {
        self.efuse.rd_repeat_data2.flash_ecc_en()
    }

    /// Flash power-up wait time in milliseconds.
    ///
    /// Values below 15 are used as programmed; 15 and above mean the fixed
    /// 30 ms wait.
    #[inline]
    pub fn flash_power_up_delay_ms(&self) -> u32 {
        let code = self.efuse.rd_repeat_data2.flash_tpuw();
        if code < 15 { code } else { 30 }
    }
}
