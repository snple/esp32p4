//! MIPI_DSI_HOST
//! MIPI Display Interface Host

#![allow(dead_code)]

pub const BASE: u32 = 0x500a0000;

/// VERSION Register
///
/// NA
///
/// Address offset: 0x0000
/// Reset value: 0x3134312a
///
/// ## Fields
///
/// - **VERSION** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct Version {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Version {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Version {
    // ======== Field Constants ========

    /// **VERSION**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const VERSION_OFFSET: u32 = 0;
    pub const VERSION_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **VERSION** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn version(&self) -> u32 {
        (self.value.read_bits() & Self::VERSION_MASK) >> Self::VERSION_OFFSET
    }
}

/// VERSION value type for batch read/modify/write operations
///
/// Use `version.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `version.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VersionVal {
    bits: u32,
}

impl VersionVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x3134312a }
    }

    /// Read **VERSION** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn version(&self) -> u32 {
        (self.bits & Version::VERSION_MASK) >> Version::VERSION_OFFSET
    }
}

impl Default for VersionVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Version {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VersionVal {
        VersionVal::from_bits(self.value.read_bits())
    }
}

/// PWR_UP Register
///
/// NA
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **SHUTDOWNZ** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PwrUp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PwrUp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PwrUp {
    // ======== Field Constants ========

    /// **SHUTDOWNZ**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SHUTDOWNZ_OFFSET: u32 = 0;
    pub const SHUTDOWNZ_MASK: u32 = 0x00000001;
    pub const SHUTDOWNZ: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SHUTDOWNZ** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn shutdownz(&self) -> bool {
        (self.value.read_bits() & Self::SHUTDOWNZ_MASK) != 0
    }

    /// Set SHUTDOWNZ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_shutdownz(&self) {
        self.value.modify_bits(|r| r | Self::SHUTDOWNZ_MASK);
    }

    /// Clear SHUTDOWNZ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_shutdownz(&self) {
        self.value.modify_bits(|r| r & !Self::SHUTDOWNZ_MASK);
    }
}

/// PWR_UP value type for batch read/modify/write operations
///
/// Use `pwrup.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pwrup.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PwrUpVal {
    bits: u32,
}

impl PwrUpVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SHUTDOWNZ** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn shutdownz(&self) -> bool {
        (self.bits & PwrUp::SHUTDOWNZ_MASK) != 0
    }

    /// Set **SHUTDOWNZ** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_shutdownz(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PwrUp::SHUTDOWNZ_MASK
            } else {
                self.bits & !PwrUp::SHUTDOWNZ_MASK
            },
        }
    }
}

impl Default for PwrUpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PwrUp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PwrUpVal {
        PwrUpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PwrUpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PwrUpVal) -> PwrUpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLKMGR_CFG Register
///
/// NA
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **TO_CLK_DIVISION** `[15:8]` (RW)
///
///   NA
///
/// - **TX_ESC_CLK_DIVISION** `[7:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct ClkmgrCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClkmgrCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkmgrCfg {
    // ======== Field Constants ========

    /// **TO_CLK_DIVISION**
    ///
    /// NA
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const TO_CLK_DIVISION_OFFSET: u32 = 8;
    pub const TO_CLK_DIVISION_MASK: u32 = 0x0000ff00;

    /// **TX_ESC_CLK_DIVISION**
    ///
    /// NA
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const TX_ESC_CLK_DIVISION_OFFSET: u32 = 0;
    pub const TX_ESC_CLK_DIVISION_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **TO_CLK_DIVISION** `[15:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn to_clk_division(&self) -> u32 {
        (self.value.read_bits() & Self::TO_CLK_DIVISION_MASK) >> Self::TO_CLK_DIVISION_OFFSET
    }

    /// Write **TO_CLK_DIVISION** `[15:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_to_clk_division(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TO_CLK_DIVISION_MASK)
                | ((value << Self::TO_CLK_DIVISION_OFFSET) & Self::TO_CLK_DIVISION_MASK)
        });
    }

    /// **TX_ESC_CLK_DIVISION** `[7:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn tx_esc_clk_division(&self) -> u32 {
        (self.value.read_bits() & Self::TX_ESC_CLK_DIVISION_MASK)
            >> Self::TX_ESC_CLK_DIVISION_OFFSET
    }

    /// Write **TX_ESC_CLK_DIVISION** `[7:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_tx_esc_clk_division(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_ESC_CLK_DIVISION_MASK)
                | ((value << Self::TX_ESC_CLK_DIVISION_OFFSET) & Self::TX_ESC_CLK_DIVISION_MASK)
        });
    }
}

/// CLKMGR_CFG value type for batch read/modify/write operations
///
/// Use `clkmgrcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkmgrcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkmgrCfgVal {
    bits: u32,
}

impl ClkmgrCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TO_CLK_DIVISION** `[15:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn to_clk_division(&self) -> u32 {
        (self.bits & ClkmgrCfg::TO_CLK_DIVISION_MASK) >> ClkmgrCfg::TO_CLK_DIVISION_OFFSET
    }

    /// Read **TX_ESC_CLK_DIVISION** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn tx_esc_clk_division(&self) -> u32 {
        (self.bits & ClkmgrCfg::TX_ESC_CLK_DIVISION_MASK) >> ClkmgrCfg::TX_ESC_CLK_DIVISION_OFFSET
    }

    /// Set **TO_CLK_DIVISION** `[15:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_to_clk_division(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkmgrCfg::TO_CLK_DIVISION_MASK)
                | ((value << ClkmgrCfg::TO_CLK_DIVISION_OFFSET) & ClkmgrCfg::TO_CLK_DIVISION_MASK),
        }
    }

    /// Set **TX_ESC_CLK_DIVISION** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_tx_esc_clk_division(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkmgrCfg::TX_ESC_CLK_DIVISION_MASK)
                | ((value << ClkmgrCfg::TX_ESC_CLK_DIVISION_OFFSET)
                    & ClkmgrCfg::TX_ESC_CLK_DIVISION_MASK),
        }
    }
}

impl Default for ClkmgrCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ClkmgrCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkmgrCfgVal {
        ClkmgrCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkmgrCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkmgrCfgVal) -> ClkmgrCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DPI_VCID Register
///
/// NA
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **DPI_VCID** `[1:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DpiVcid {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiVcid {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiVcid {
    // ======== Field Constants ========

    /// **DPI_VCID**
    ///
    /// NA
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const DPI_VCID_OFFSET: u32 = 0;
    pub const DPI_VCID_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **DPI_VCID** `[1:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn dpi_vcid(&self) -> u32 {
        (self.value.read_bits() & Self::DPI_VCID_MASK) >> Self::DPI_VCID_OFFSET
    }

    /// Write **DPI_VCID** `[1:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_dpi_vcid(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DPI_VCID_MASK) | ((value << Self::DPI_VCID_OFFSET) & Self::DPI_VCID_MASK)
        });
    }
}

/// DPI_VCID value type for batch read/modify/write operations
///
/// Use `dpivcid.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpivcid.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiVcidVal {
    bits: u32,
}

impl DpiVcidVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DPI_VCID** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dpi_vcid(&self) -> u32 {
        (self.bits & DpiVcid::DPI_VCID_MASK) >> DpiVcid::DPI_VCID_OFFSET
    }

    /// Set **DPI_VCID** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_dpi_vcid(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DpiVcid::DPI_VCID_MASK)
                | ((value << DpiVcid::DPI_VCID_OFFSET) & DpiVcid::DPI_VCID_MASK),
        }
    }
}

impl Default for DpiVcidVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DpiVcid {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DpiVcidVal {
        DpiVcidVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DpiVcidVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DpiVcidVal) -> DpiVcidVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DPI_COLOR_CODING Register
///
/// NA
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **LOOSELY18_EN** `[8]` (RW)
///
///   NA
///
/// - **DPI_COLOR_CODING** `[3:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DpiColorCoding {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiColorCoding {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiColorCoding {
    // ======== Field Constants ========

    /// **LOOSELY18_EN**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const LOOSELY18_EN_OFFSET: u32 = 8;
    pub const LOOSELY18_EN_MASK: u32 = 0x00000100;
    pub const LOOSELY18_EN: u32 = 1 << 8;

    /// **DPI_COLOR_CODING**
    ///
    /// NA
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const DPI_COLOR_CODING_OFFSET: u32 = 0;
    pub const DPI_COLOR_CODING_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **LOOSELY18_EN** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn loosely18_en(&self) -> bool {
        (self.value.read_bits() & Self::LOOSELY18_EN_MASK) != 0
    }

    /// Set LOOSELY18_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_loosely18_en(&self) {
        self.value.modify_bits(|r| r | Self::LOOSELY18_EN_MASK);
    }

    /// Clear LOOSELY18_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_loosely18_en(&self) {
        self.value.modify_bits(|r| r & !Self::LOOSELY18_EN_MASK);
    }

    /// **DPI_COLOR_CODING** `[3:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn dpi_color_coding(&self) -> u32 {
        (self.value.read_bits() & Self::DPI_COLOR_CODING_MASK) >> Self::DPI_COLOR_CODING_OFFSET
    }

    /// Write **DPI_COLOR_CODING** `[3:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_dpi_color_coding(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DPI_COLOR_CODING_MASK)
                | ((value << Self::DPI_COLOR_CODING_OFFSET) & Self::DPI_COLOR_CODING_MASK)
        });
    }
}

/// DPI_COLOR_CODING value type for batch read/modify/write operations
///
/// Use `dpicolorcoding.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpicolorcoding.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiColorCodingVal {
    bits: u32,
}

impl DpiColorCodingVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LOOSELY18_EN** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn loosely18_en(&self) -> bool {
        (self.bits & DpiColorCoding::LOOSELY18_EN_MASK) != 0
    }

    /// Read **DPI_COLOR_CODING** `[3:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dpi_color_coding(&self) -> u32 {
        (self.bits & DpiColorCoding::DPI_COLOR_CODING_MASK)
            >> DpiColorCoding::DPI_COLOR_CODING_OFFSET
    }

    /// Set **LOOSELY18_EN** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_loosely18_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DpiColorCoding::LOOSELY18_EN_MASK
            } else {
                self.bits & !DpiColorCoding::LOOSELY18_EN_MASK
            },
        }
    }

    /// Set **DPI_COLOR_CODING** `[3:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_dpi_color_coding(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DpiColorCoding::DPI_COLOR_CODING_MASK)
                | ((value << DpiColorCoding::DPI_COLOR_CODING_OFFSET)
                    & DpiColorCoding::DPI_COLOR_CODING_MASK),
        }
    }
}

impl Default for DpiColorCodingVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DpiColorCoding {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DpiColorCodingVal {
        DpiColorCodingVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DpiColorCodingVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DpiColorCodingVal) -> DpiColorCodingVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DPI_CFG_POL Register
///
/// NA
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **COLORM_ACTIVE_LOW** `[4]` (RW)
///
///   NA
///
/// - **SHUTD_ACTIVE_LOW** `[3]` (RW)
///
///   NA
///
/// - **HSYNC_ACTIVE_LOW** `[2]` (RW)
///
///   NA
///
/// - **VSYNC_ACTIVE_LOW** `[1]` (RW)
///
///   NA
///
/// - **DATAEN_ACTIVE_LOW** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DpiCfgPol {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiCfgPol {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiCfgPol {
    // ======== Field Constants ========

    /// **COLORM_ACTIVE_LOW**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const COLORM_ACTIVE_LOW_OFFSET: u32 = 4;
    pub const COLORM_ACTIVE_LOW_MASK: u32 = 0x00000010;
    pub const COLORM_ACTIVE_LOW: u32 = 1 << 4;

    /// **SHUTD_ACTIVE_LOW**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SHUTD_ACTIVE_LOW_OFFSET: u32 = 3;
    pub const SHUTD_ACTIVE_LOW_MASK: u32 = 0x00000008;
    pub const SHUTD_ACTIVE_LOW: u32 = 1 << 3;

    /// **HSYNC_ACTIVE_LOW**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const HSYNC_ACTIVE_LOW_OFFSET: u32 = 2;
    pub const HSYNC_ACTIVE_LOW_MASK: u32 = 0x00000004;
    pub const HSYNC_ACTIVE_LOW: u32 = 1 << 2;

    /// **VSYNC_ACTIVE_LOW**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const VSYNC_ACTIVE_LOW_OFFSET: u32 = 1;
    pub const VSYNC_ACTIVE_LOW_MASK: u32 = 0x00000002;
    pub const VSYNC_ACTIVE_LOW: u32 = 1 << 1;

    /// **DATAEN_ACTIVE_LOW**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DATAEN_ACTIVE_LOW_OFFSET: u32 = 0;
    pub const DATAEN_ACTIVE_LOW_MASK: u32 = 0x00000001;
    pub const DATAEN_ACTIVE_LOW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **COLORM_ACTIVE_LOW** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn colorm_active_low(&self) -> bool {
        (self.value.read_bits() & Self::COLORM_ACTIVE_LOW_MASK) != 0
    }

    /// Set COLORM_ACTIVE_LOW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_colorm_active_low(&self) {
        self.value.modify_bits(|r| r | Self::COLORM_ACTIVE_LOW_MASK);
    }

    /// Clear COLORM_ACTIVE_LOW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_colorm_active_low(&self) {
        self.value
            .modify_bits(|r| r & !Self::COLORM_ACTIVE_LOW_MASK);
    }

    /// **SHUTD_ACTIVE_LOW** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn shutd_active_low(&self) -> bool {
        (self.value.read_bits() & Self::SHUTD_ACTIVE_LOW_MASK) != 0
    }

    /// Set SHUTD_ACTIVE_LOW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_shutd_active_low(&self) {
        self.value.modify_bits(|r| r | Self::SHUTD_ACTIVE_LOW_MASK);
    }

    /// Clear SHUTD_ACTIVE_LOW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_shutd_active_low(&self) {
        self.value.modify_bits(|r| r & !Self::SHUTD_ACTIVE_LOW_MASK);
    }

    /// **HSYNC_ACTIVE_LOW** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn hsync_active_low(&self) -> bool {
        (self.value.read_bits() & Self::HSYNC_ACTIVE_LOW_MASK) != 0
    }

    /// Set HSYNC_ACTIVE_LOW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_hsync_active_low(&self) {
        self.value.modify_bits(|r| r | Self::HSYNC_ACTIVE_LOW_MASK);
    }

    /// Clear HSYNC_ACTIVE_LOW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_hsync_active_low(&self) {
        self.value.modify_bits(|r| r & !Self::HSYNC_ACTIVE_LOW_MASK);
    }

    /// **VSYNC_ACTIVE_LOW** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vsync_active_low(&self) -> bool {
        (self.value.read_bits() & Self::VSYNC_ACTIVE_LOW_MASK) != 0
    }

    /// Set VSYNC_ACTIVE_LOW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vsync_active_low(&self) {
        self.value.modify_bits(|r| r | Self::VSYNC_ACTIVE_LOW_MASK);
    }

    /// Clear VSYNC_ACTIVE_LOW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vsync_active_low(&self) {
        self.value.modify_bits(|r| r & !Self::VSYNC_ACTIVE_LOW_MASK);
    }

    /// **DATAEN_ACTIVE_LOW** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn dataen_active_low(&self) -> bool {
        (self.value.read_bits() & Self::DATAEN_ACTIVE_LOW_MASK) != 0
    }

    /// Set DATAEN_ACTIVE_LOW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dataen_active_low(&self) {
        self.value.modify_bits(|r| r | Self::DATAEN_ACTIVE_LOW_MASK);
    }

    /// Clear DATAEN_ACTIVE_LOW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dataen_active_low(&self) {
        self.value
            .modify_bits(|r| r & !Self::DATAEN_ACTIVE_LOW_MASK);
    }
}

/// DPI_CFG_POL value type for batch read/modify/write operations
///
/// Use `dpicfgpol.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpicfgpol.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiCfgPolVal {
    bits: u32,
}

impl DpiCfgPolVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **COLORM_ACTIVE_LOW** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn colorm_active_low(&self) -> bool {
        (self.bits & DpiCfgPol::COLORM_ACTIVE_LOW_MASK) != 0
    }

    /// Read **SHUTD_ACTIVE_LOW** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn shutd_active_low(&self) -> bool {
        (self.bits & DpiCfgPol::SHUTD_ACTIVE_LOW_MASK) != 0
    }

    /// Read **HSYNC_ACTIVE_LOW** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn hsync_active_low(&self) -> bool {
        (self.bits & DpiCfgPol::HSYNC_ACTIVE_LOW_MASK) != 0
    }

    /// Read **VSYNC_ACTIVE_LOW** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vsync_active_low(&self) -> bool {
        (self.bits & DpiCfgPol::VSYNC_ACTIVE_LOW_MASK) != 0
    }

    /// Read **DATAEN_ACTIVE_LOW** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dataen_active_low(&self) -> bool {
        (self.bits & DpiCfgPol::DATAEN_ACTIVE_LOW_MASK) != 0
    }

    /// Set **COLORM_ACTIVE_LOW** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_colorm_active_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DpiCfgPol::COLORM_ACTIVE_LOW_MASK
            } else {
                self.bits & !DpiCfgPol::COLORM_ACTIVE_LOW_MASK
            },
        }
    }

    /// Set **SHUTD_ACTIVE_LOW** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_shutd_active_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DpiCfgPol::SHUTD_ACTIVE_LOW_MASK
            } else {
                self.bits & !DpiCfgPol::SHUTD_ACTIVE_LOW_MASK
            },
        }
    }

    /// Set **HSYNC_ACTIVE_LOW** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_hsync_active_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DpiCfgPol::HSYNC_ACTIVE_LOW_MASK
            } else {
                self.bits & !DpiCfgPol::HSYNC_ACTIVE_LOW_MASK
            },
        }
    }

    /// Set **VSYNC_ACTIVE_LOW** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vsync_active_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DpiCfgPol::VSYNC_ACTIVE_LOW_MASK
            } else {
                self.bits & !DpiCfgPol::VSYNC_ACTIVE_LOW_MASK
            },
        }
    }

    /// Set **DATAEN_ACTIVE_LOW** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_dataen_active_low(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DpiCfgPol::DATAEN_ACTIVE_LOW_MASK
            } else {
                self.bits & !DpiCfgPol::DATAEN_ACTIVE_LOW_MASK
            },
        }
    }
}

impl Default for DpiCfgPolVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DpiCfgPol {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DpiCfgPolVal {
        DpiCfgPolVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DpiCfgPolVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DpiCfgPolVal) -> DpiCfgPolVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DPI_LP_CMD_TIM Register
///
/// NA
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **OUTVACT_LPCMD_TIME** `[23:16]` (RW)
///
///   NA
///
/// - **INVACT_LPCMD_TIME** `[7:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DpiLpCmdTim {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiLpCmdTim {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiLpCmdTim {
    // ======== Field Constants ========

    /// **OUTVACT_LPCMD_TIME**
    ///
    /// NA
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const OUTVACT_LPCMD_TIME_OFFSET: u32 = 16;
    pub const OUTVACT_LPCMD_TIME_MASK: u32 = 0x00ff0000;

    /// **INVACT_LPCMD_TIME**
    ///
    /// NA
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const INVACT_LPCMD_TIME_OFFSET: u32 = 0;
    pub const INVACT_LPCMD_TIME_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **OUTVACT_LPCMD_TIME** `[23:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn outvact_lpcmd_time(&self) -> u32 {
        (self.value.read_bits() & Self::OUTVACT_LPCMD_TIME_MASK) >> Self::OUTVACT_LPCMD_TIME_OFFSET
    }

    /// Write **OUTVACT_LPCMD_TIME** `[23:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_outvact_lpcmd_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OUTVACT_LPCMD_TIME_MASK)
                | ((value << Self::OUTVACT_LPCMD_TIME_OFFSET) & Self::OUTVACT_LPCMD_TIME_MASK)
        });
    }

    /// **INVACT_LPCMD_TIME** `[7:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn invact_lpcmd_time(&self) -> u32 {
        (self.value.read_bits() & Self::INVACT_LPCMD_TIME_MASK) >> Self::INVACT_LPCMD_TIME_OFFSET
    }

    /// Write **INVACT_LPCMD_TIME** `[7:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_invact_lpcmd_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::INVACT_LPCMD_TIME_MASK)
                | ((value << Self::INVACT_LPCMD_TIME_OFFSET) & Self::INVACT_LPCMD_TIME_MASK)
        });
    }
}

/// DPI_LP_CMD_TIM value type for batch read/modify/write operations
///
/// Use `dpilpcmdtim.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpilpcmdtim.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiLpCmdTimVal {
    bits: u32,
}

impl DpiLpCmdTimVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **OUTVACT_LPCMD_TIME** `[23:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn outvact_lpcmd_time(&self) -> u32 {
        (self.bits & DpiLpCmdTim::OUTVACT_LPCMD_TIME_MASK) >> DpiLpCmdTim::OUTVACT_LPCMD_TIME_OFFSET
    }

    /// Read **INVACT_LPCMD_TIME** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn invact_lpcmd_time(&self) -> u32 {
        (self.bits & DpiLpCmdTim::INVACT_LPCMD_TIME_MASK) >> DpiLpCmdTim::INVACT_LPCMD_TIME_OFFSET
    }

    /// Set **OUTVACT_LPCMD_TIME** `[23:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_outvact_lpcmd_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DpiLpCmdTim::OUTVACT_LPCMD_TIME_MASK)
                | ((value << DpiLpCmdTim::OUTVACT_LPCMD_TIME_OFFSET)
                    & DpiLpCmdTim::OUTVACT_LPCMD_TIME_MASK),
        }
    }

    /// Set **INVACT_LPCMD_TIME** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_invact_lpcmd_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DpiLpCmdTim::INVACT_LPCMD_TIME_MASK)
                | ((value << DpiLpCmdTim::INVACT_LPCMD_TIME_OFFSET)
                    & DpiLpCmdTim::INVACT_LPCMD_TIME_MASK),
        }
    }
}

impl Default for DpiLpCmdTimVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DpiLpCmdTim {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DpiLpCmdTimVal {
        DpiLpCmdTimVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DpiLpCmdTimVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DpiLpCmdTimVal) -> DpiLpCmdTimVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBI_VCID Register
///
/// NA
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **DBI_VCID** `[1:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DbiVcid {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiVcid {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiVcid {
    // ======== Field Constants ========

    /// **DBI_VCID**
    ///
    /// NA
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const DBI_VCID_OFFSET: u32 = 0;
    pub const DBI_VCID_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **DBI_VCID** `[1:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn dbi_vcid(&self) -> u32 {
        (self.value.read_bits() & Self::DBI_VCID_MASK) >> Self::DBI_VCID_OFFSET
    }

    /// Write **DBI_VCID** `[1:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_dbi_vcid(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBI_VCID_MASK) | ((value << Self::DBI_VCID_OFFSET) & Self::DBI_VCID_MASK)
        });
    }
}

/// DBI_VCID value type for batch read/modify/write operations
///
/// Use `dbivcid.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbivcid.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiVcidVal {
    bits: u32,
}

impl DbiVcidVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DBI_VCID** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dbi_vcid(&self) -> u32 {
        (self.bits & DbiVcid::DBI_VCID_MASK) >> DbiVcid::DBI_VCID_OFFSET
    }

    /// Set **DBI_VCID** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_dbi_vcid(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiVcid::DBI_VCID_MASK)
                | ((value << DbiVcid::DBI_VCID_OFFSET) & DbiVcid::DBI_VCID_MASK),
        }
    }
}

impl Default for DbiVcidVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DbiVcid {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiVcidVal {
        DbiVcidVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiVcidVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiVcidVal) -> DbiVcidVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBI_CFG Register
///
/// NA
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **LUT_SIZE_CONF** `[17:16]` (RW)
///
///   NA
///
/// - **OUT_DBI_CONF** `[11:8]` (RW)
///
///   NA
///
/// - **IN_DBI_CONF** `[3:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DbiCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiCfg {
    // ======== Field Constants ========

    /// **LUT_SIZE_CONF**
    ///
    /// NA
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const LUT_SIZE_CONF_OFFSET: u32 = 16;
    pub const LUT_SIZE_CONF_MASK: u32 = 0x00030000;

    /// **OUT_DBI_CONF**
    ///
    /// NA
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const OUT_DBI_CONF_OFFSET: u32 = 8;
    pub const OUT_DBI_CONF_MASK: u32 = 0x00000f00;

    /// **IN_DBI_CONF**
    ///
    /// NA
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const IN_DBI_CONF_OFFSET: u32 = 0;
    pub const IN_DBI_CONF_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **LUT_SIZE_CONF** `[17:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn lut_size_conf(&self) -> u32 {
        (self.value.read_bits() & Self::LUT_SIZE_CONF_MASK) >> Self::LUT_SIZE_CONF_OFFSET
    }

    /// Write **LUT_SIZE_CONF** `[17:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_lut_size_conf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LUT_SIZE_CONF_MASK)
                | ((value << Self::LUT_SIZE_CONF_OFFSET) & Self::LUT_SIZE_CONF_MASK)
        });
    }

    /// **OUT_DBI_CONF** `[11:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn out_dbi_conf(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_DBI_CONF_MASK) >> Self::OUT_DBI_CONF_OFFSET
    }

    /// Write **OUT_DBI_CONF** `[11:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_out_dbi_conf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OUT_DBI_CONF_MASK)
                | ((value << Self::OUT_DBI_CONF_OFFSET) & Self::OUT_DBI_CONF_MASK)
        });
    }

    /// **IN_DBI_CONF** `[3:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn in_dbi_conf(&self) -> u32 {
        (self.value.read_bits() & Self::IN_DBI_CONF_MASK) >> Self::IN_DBI_CONF_OFFSET
    }

    /// Write **IN_DBI_CONF** `[3:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_in_dbi_conf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::IN_DBI_CONF_MASK)
                | ((value << Self::IN_DBI_CONF_OFFSET) & Self::IN_DBI_CONF_MASK)
        });
    }
}

/// DBI_CFG value type for batch read/modify/write operations
///
/// Use `dbicfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbicfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiCfgVal {
    bits: u32,
}

impl DbiCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LUT_SIZE_CONF** `[17:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lut_size_conf(&self) -> u32 {
        (self.bits & DbiCfg::LUT_SIZE_CONF_MASK) >> DbiCfg::LUT_SIZE_CONF_OFFSET
    }

    /// Read **OUT_DBI_CONF** `[11:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn out_dbi_conf(&self) -> u32 {
        (self.bits & DbiCfg::OUT_DBI_CONF_MASK) >> DbiCfg::OUT_DBI_CONF_OFFSET
    }

    /// Read **IN_DBI_CONF** `[3:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn in_dbi_conf(&self) -> u32 {
        (self.bits & DbiCfg::IN_DBI_CONF_MASK) >> DbiCfg::IN_DBI_CONF_OFFSET
    }

    /// Set **LUT_SIZE_CONF** `[17:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_lut_size_conf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiCfg::LUT_SIZE_CONF_MASK)
                | ((value << DbiCfg::LUT_SIZE_CONF_OFFSET) & DbiCfg::LUT_SIZE_CONF_MASK),
        }
    }

    /// Set **OUT_DBI_CONF** `[11:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_out_dbi_conf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiCfg::OUT_DBI_CONF_MASK)
                | ((value << DbiCfg::OUT_DBI_CONF_OFFSET) & DbiCfg::OUT_DBI_CONF_MASK),
        }
    }

    /// Set **IN_DBI_CONF** `[3:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_in_dbi_conf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiCfg::IN_DBI_CONF_MASK)
                | ((value << DbiCfg::IN_DBI_CONF_OFFSET) & DbiCfg::IN_DBI_CONF_MASK),
        }
    }
}

impl Default for DbiCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DbiCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiCfgVal {
        DbiCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiCfgVal) -> DbiCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBI_PARTITIONING_EN Register
///
/// NA
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **PARTITIONING_EN** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DbiPartitioningEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiPartitioningEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiPartitioningEn {
    // ======== Field Constants ========

    /// **PARTITIONING_EN**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PARTITIONING_EN_OFFSET: u32 = 0;
    pub const PARTITIONING_EN_MASK: u32 = 0x00000001;
    pub const PARTITIONING_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PARTITIONING_EN** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn partitioning_en(&self) -> bool {
        (self.value.read_bits() & Self::PARTITIONING_EN_MASK) != 0
    }

    /// Set PARTITIONING_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_partitioning_en(&self) {
        self.value.modify_bits(|r| r | Self::PARTITIONING_EN_MASK);
    }

    /// Clear PARTITIONING_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_partitioning_en(&self) {
        self.value.modify_bits(|r| r & !Self::PARTITIONING_EN_MASK);
    }
}

/// DBI_PARTITIONING_EN value type for batch read/modify/write operations
///
/// Use `dbipartitioningen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbipartitioningen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiPartitioningEnVal {
    bits: u32,
}

impl DbiPartitioningEnVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PARTITIONING_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn partitioning_en(&self) -> bool {
        (self.bits & DbiPartitioningEn::PARTITIONING_EN_MASK) != 0
    }

    /// Set **PARTITIONING_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_partitioning_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DbiPartitioningEn::PARTITIONING_EN_MASK
            } else {
                self.bits & !DbiPartitioningEn::PARTITIONING_EN_MASK
            },
        }
    }
}

impl Default for DbiPartitioningEnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DbiPartitioningEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiPartitioningEnVal {
        DbiPartitioningEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiPartitioningEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiPartitioningEnVal) -> DbiPartitioningEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBI_CMDSIZE Register
///
/// NA
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **ALLOWED_CMD_SIZE** `[31:16]` (RW)
///
///   NA
///
/// - **WR_CMD_SIZE** `[15:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DbiCmdsize {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiCmdsize {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiCmdsize {
    // ======== Field Constants ========

    /// **ALLOWED_CMD_SIZE**
    ///
    /// NA
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const ALLOWED_CMD_SIZE_OFFSET: u32 = 16;
    pub const ALLOWED_CMD_SIZE_MASK: u32 = 0xffff0000;

    /// **WR_CMD_SIZE**
    ///
    /// NA
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const WR_CMD_SIZE_OFFSET: u32 = 0;
    pub const WR_CMD_SIZE_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ALLOWED_CMD_SIZE** `[31:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn allowed_cmd_size(&self) -> u32 {
        (self.value.read_bits() & Self::ALLOWED_CMD_SIZE_MASK) >> Self::ALLOWED_CMD_SIZE_OFFSET
    }

    /// Write **ALLOWED_CMD_SIZE** `[31:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_allowed_cmd_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ALLOWED_CMD_SIZE_MASK)
                | ((value << Self::ALLOWED_CMD_SIZE_OFFSET) & Self::ALLOWED_CMD_SIZE_MASK)
        });
    }

    /// **WR_CMD_SIZE** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn wr_cmd_size(&self) -> u32 {
        (self.value.read_bits() & Self::WR_CMD_SIZE_MASK) >> Self::WR_CMD_SIZE_OFFSET
    }

    /// Write **WR_CMD_SIZE** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_wr_cmd_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WR_CMD_SIZE_MASK)
                | ((value << Self::WR_CMD_SIZE_OFFSET) & Self::WR_CMD_SIZE_MASK)
        });
    }
}

/// DBI_CMDSIZE value type for batch read/modify/write operations
///
/// Use `dbicmdsize.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbicmdsize.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiCmdsizeVal {
    bits: u32,
}

impl DbiCmdsizeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ALLOWED_CMD_SIZE** `[31:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn allowed_cmd_size(&self) -> u32 {
        (self.bits & DbiCmdsize::ALLOWED_CMD_SIZE_MASK) >> DbiCmdsize::ALLOWED_CMD_SIZE_OFFSET
    }

    /// Read **WR_CMD_SIZE** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn wr_cmd_size(&self) -> u32 {
        (self.bits & DbiCmdsize::WR_CMD_SIZE_MASK) >> DbiCmdsize::WR_CMD_SIZE_OFFSET
    }

    /// Set **ALLOWED_CMD_SIZE** `[31:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_allowed_cmd_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiCmdsize::ALLOWED_CMD_SIZE_MASK)
                | ((value << DbiCmdsize::ALLOWED_CMD_SIZE_OFFSET)
                    & DbiCmdsize::ALLOWED_CMD_SIZE_MASK),
        }
    }

    /// Set **WR_CMD_SIZE** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_wr_cmd_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiCmdsize::WR_CMD_SIZE_MASK)
                | ((value << DbiCmdsize::WR_CMD_SIZE_OFFSET) & DbiCmdsize::WR_CMD_SIZE_MASK),
        }
    }
}

impl Default for DbiCmdsizeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DbiCmdsize {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiCmdsizeVal {
        DbiCmdsizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiCmdsizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiCmdsizeVal) -> DbiCmdsizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PCKHDL_CFG Register
///
/// NA
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **EOTP_TX_LP_EN** `[5]` (RW)
///
///   NA
///
/// - **CRC_RX_EN** `[4]` (RW)
///
///   NA
///
/// - **ECC_RX_EN** `[3]` (RW)
///
///   NA
///
/// - **BTA_EN** `[2]` (RW)
///
///   NA
///
/// - **EOTP_RX_EN** `[1]` (RW)
///
///   NA
///
/// - **EOTP_TX_EN** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PckhdlCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PckhdlCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PckhdlCfg {
    // ======== Field Constants ========

    /// **EOTP_TX_LP_EN**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const EOTP_TX_LP_EN_OFFSET: u32 = 5;
    pub const EOTP_TX_LP_EN_MASK: u32 = 0x00000020;
    pub const EOTP_TX_LP_EN: u32 = 1 << 5;

    /// **CRC_RX_EN**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CRC_RX_EN_OFFSET: u32 = 4;
    pub const CRC_RX_EN_MASK: u32 = 0x00000010;
    pub const CRC_RX_EN: u32 = 1 << 4;

    /// **ECC_RX_EN**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ECC_RX_EN_OFFSET: u32 = 3;
    pub const ECC_RX_EN_MASK: u32 = 0x00000008;
    pub const ECC_RX_EN: u32 = 1 << 3;

    /// **BTA_EN**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const BTA_EN_OFFSET: u32 = 2;
    pub const BTA_EN_MASK: u32 = 0x00000004;
    pub const BTA_EN: u32 = 1 << 2;

    /// **EOTP_RX_EN**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const EOTP_RX_EN_OFFSET: u32 = 1;
    pub const EOTP_RX_EN_MASK: u32 = 0x00000002;
    pub const EOTP_RX_EN: u32 = 1 << 1;

    /// **EOTP_TX_EN**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EOTP_TX_EN_OFFSET: u32 = 0;
    pub const EOTP_TX_EN_MASK: u32 = 0x00000001;
    pub const EOTP_TX_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EOTP_TX_LP_EN** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn eotp_tx_lp_en(&self) -> bool {
        (self.value.read_bits() & Self::EOTP_TX_LP_EN_MASK) != 0
    }

    /// Set EOTP_TX_LP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_eotp_tx_lp_en(&self) {
        self.value.modify_bits(|r| r | Self::EOTP_TX_LP_EN_MASK);
    }

    /// Clear EOTP_TX_LP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_eotp_tx_lp_en(&self) {
        self.value.modify_bits(|r| r & !Self::EOTP_TX_LP_EN_MASK);
    }

    /// **CRC_RX_EN** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn crc_rx_en(&self) -> bool {
        (self.value.read_bits() & Self::CRC_RX_EN_MASK) != 0
    }

    /// Set CRC_RX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_crc_rx_en(&self) {
        self.value.modify_bits(|r| r | Self::CRC_RX_EN_MASK);
    }

    /// Clear CRC_RX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_crc_rx_en(&self) {
        self.value.modify_bits(|r| r & !Self::CRC_RX_EN_MASK);
    }

    /// **ECC_RX_EN** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn ecc_rx_en(&self) -> bool {
        (self.value.read_bits() & Self::ECC_RX_EN_MASK) != 0
    }

    /// Set ECC_RX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ecc_rx_en(&self) {
        self.value.modify_bits(|r| r | Self::ECC_RX_EN_MASK);
    }

    /// Clear ECC_RX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ecc_rx_en(&self) {
        self.value.modify_bits(|r| r & !Self::ECC_RX_EN_MASK);
    }

    /// **BTA_EN** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn bta_en(&self) -> bool {
        (self.value.read_bits() & Self::BTA_EN_MASK) != 0
    }

    /// Set BTA_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bta_en(&self) {
        self.value.modify_bits(|r| r | Self::BTA_EN_MASK);
    }

    /// Clear BTA_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bta_en(&self) {
        self.value.modify_bits(|r| r & !Self::BTA_EN_MASK);
    }

    /// **EOTP_RX_EN** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn eotp_rx_en(&self) -> bool {
        (self.value.read_bits() & Self::EOTP_RX_EN_MASK) != 0
    }

    /// Set EOTP_RX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_eotp_rx_en(&self) {
        self.value.modify_bits(|r| r | Self::EOTP_RX_EN_MASK);
    }

    /// Clear EOTP_RX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_eotp_rx_en(&self) {
        self.value.modify_bits(|r| r & !Self::EOTP_RX_EN_MASK);
    }

    /// **EOTP_TX_EN** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn eotp_tx_en(&self) -> bool {
        (self.value.read_bits() & Self::EOTP_TX_EN_MASK) != 0
    }

    /// Set EOTP_TX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_eotp_tx_en(&self) {
        self.value.modify_bits(|r| r | Self::EOTP_TX_EN_MASK);
    }

    /// Clear EOTP_TX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_eotp_tx_en(&self) {
        self.value.modify_bits(|r| r & !Self::EOTP_TX_EN_MASK);
    }
}

/// PCKHDL_CFG value type for batch read/modify/write operations
///
/// Use `pckhdlcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pckhdlcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PckhdlCfgVal {
    bits: u32,
}

impl PckhdlCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **EOTP_TX_LP_EN** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn eotp_tx_lp_en(&self) -> bool {
        (self.bits & PckhdlCfg::EOTP_TX_LP_EN_MASK) != 0
    }

    /// Read **CRC_RX_EN** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn crc_rx_en(&self) -> bool {
        (self.bits & PckhdlCfg::CRC_RX_EN_MASK) != 0
    }

    /// Read **ECC_RX_EN** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ecc_rx_en(&self) -> bool {
        (self.bits & PckhdlCfg::ECC_RX_EN_MASK) != 0
    }

    /// Read **BTA_EN** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn bta_en(&self) -> bool {
        (self.bits & PckhdlCfg::BTA_EN_MASK) != 0
    }

    /// Read **EOTP_RX_EN** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn eotp_rx_en(&self) -> bool {
        (self.bits & PckhdlCfg::EOTP_RX_EN_MASK) != 0
    }

    /// Read **EOTP_TX_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn eotp_tx_en(&self) -> bool {
        (self.bits & PckhdlCfg::EOTP_TX_EN_MASK) != 0
    }

    /// Set **EOTP_TX_LP_EN** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_eotp_tx_lp_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PckhdlCfg::EOTP_TX_LP_EN_MASK
            } else {
                self.bits & !PckhdlCfg::EOTP_TX_LP_EN_MASK
            },
        }
    }

    /// Set **CRC_RX_EN** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_crc_rx_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PckhdlCfg::CRC_RX_EN_MASK
            } else {
                self.bits & !PckhdlCfg::CRC_RX_EN_MASK
            },
        }
    }

    /// Set **ECC_RX_EN** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ecc_rx_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PckhdlCfg::ECC_RX_EN_MASK
            } else {
                self.bits & !PckhdlCfg::ECC_RX_EN_MASK
            },
        }
    }

    /// Set **BTA_EN** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_bta_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PckhdlCfg::BTA_EN_MASK
            } else {
                self.bits & !PckhdlCfg::BTA_EN_MASK
            },
        }
    }

    /// Set **EOTP_RX_EN** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_eotp_rx_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PckhdlCfg::EOTP_RX_EN_MASK
            } else {
                self.bits & !PckhdlCfg::EOTP_RX_EN_MASK
            },
        }
    }

    /// Set **EOTP_TX_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_eotp_tx_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PckhdlCfg::EOTP_TX_EN_MASK
            } else {
                self.bits & !PckhdlCfg::EOTP_TX_EN_MASK
            },
        }
    }
}

impl Default for PckhdlCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PckhdlCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PckhdlCfgVal {
        PckhdlCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PckhdlCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PckhdlCfgVal) -> PckhdlCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GEN_VCID Register
///
/// NA
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **TX_AUTO** `[17:16]` (RW)
///
///   NA
///
/// - **TEAR_AUTO** `[9:8]` (RW)
///
///   NA
///
/// - **RX** `[1:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct GenVcid {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GenVcid {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GenVcid {
    // ======== Field Constants ========

    /// **TX_AUTO**
    ///
    /// NA
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const TX_AUTO_OFFSET: u32 = 16;
    pub const TX_AUTO_MASK: u32 = 0x00030000;

    /// **TEAR_AUTO**
    ///
    /// NA
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const TEAR_AUTO_OFFSET: u32 = 8;
    pub const TEAR_AUTO_MASK: u32 = 0x00000300;

    /// **RX**
    ///
    /// NA
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const RX_OFFSET: u32 = 0;
    pub const RX_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TX_AUTO** `[17:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn tx_auto(&self) -> u32 {
        (self.value.read_bits() & Self::TX_AUTO_MASK) >> Self::TX_AUTO_OFFSET
    }

    /// Write **TX_AUTO** `[17:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_tx_auto(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_AUTO_MASK) | ((value << Self::TX_AUTO_OFFSET) & Self::TX_AUTO_MASK)
        });
    }

    /// **TEAR_AUTO** `[9:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn tear_auto(&self) -> u32 {
        (self.value.read_bits() & Self::TEAR_AUTO_MASK) >> Self::TEAR_AUTO_OFFSET
    }

    /// Write **TEAR_AUTO** `[9:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_tear_auto(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TEAR_AUTO_MASK) | ((value << Self::TEAR_AUTO_OFFSET) & Self::TEAR_AUTO_MASK)
        });
    }

    /// **RX** `[1:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn rx(&self) -> u32 {
        (self.value.read_bits() & Self::RX_MASK) >> Self::RX_OFFSET
    }

    /// Write **RX** `[1:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_rx(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::RX_MASK) | ((value << Self::RX_OFFSET) & Self::RX_MASK));
    }
}

/// GEN_VCID value type for batch read/modify/write operations
///
/// Use `genvcid.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `genvcid.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GenVcidVal {
    bits: u32,
}

impl GenVcidVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TX_AUTO** `[17:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn tx_auto(&self) -> u32 {
        (self.bits & GenVcid::TX_AUTO_MASK) >> GenVcid::TX_AUTO_OFFSET
    }

    /// Read **TEAR_AUTO** `[9:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn tear_auto(&self) -> u32 {
        (self.bits & GenVcid::TEAR_AUTO_MASK) >> GenVcid::TEAR_AUTO_OFFSET
    }

    /// Read **RX** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn rx(&self) -> u32 {
        (self.bits & GenVcid::RX_MASK) >> GenVcid::RX_OFFSET
    }

    /// Set **TX_AUTO** `[17:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_tx_auto(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GenVcid::TX_AUTO_MASK)
                | ((value << GenVcid::TX_AUTO_OFFSET) & GenVcid::TX_AUTO_MASK),
        }
    }

    /// Set **TEAR_AUTO** `[9:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_tear_auto(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GenVcid::TEAR_AUTO_MASK)
                | ((value << GenVcid::TEAR_AUTO_OFFSET) & GenVcid::TEAR_AUTO_MASK),
        }
    }

    /// Set **RX** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_rx(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GenVcid::RX_MASK)
                | ((value << GenVcid::RX_OFFSET) & GenVcid::RX_MASK),
        }
    }
}

impl Default for GenVcidVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GenVcid {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GenVcidVal {
        GenVcidVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GenVcidVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GenVcidVal) -> GenVcidVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MODE_CFG Register
///
/// NA
///
/// Address offset: 0x0034
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CMD_VIDEO_MODE** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct ModeCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ModeCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ModeCfg {
    // ======== Field Constants ========

    /// **CMD_VIDEO_MODE**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CMD_VIDEO_MODE_OFFSET: u32 = 0;
    pub const CMD_VIDEO_MODE_MASK: u32 = 0x00000001;
    pub const CMD_VIDEO_MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CMD_VIDEO_MODE** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn cmd_video_mode(&self) -> bool {
        (self.value.read_bits() & Self::CMD_VIDEO_MODE_MASK) != 0
    }

    /// Set CMD_VIDEO_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cmd_video_mode(&self) {
        self.value.modify_bits(|r| r | Self::CMD_VIDEO_MODE_MASK);
    }

    /// Clear CMD_VIDEO_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cmd_video_mode(&self) {
        self.value.modify_bits(|r| r & !Self::CMD_VIDEO_MODE_MASK);
    }
}

/// MODE_CFG value type for batch read/modify/write operations
///
/// Use `modecfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `modecfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ModeCfgVal {
    bits: u32,
}

impl ModeCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000001 }
    }

    /// Read **CMD_VIDEO_MODE** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn cmd_video_mode(&self) -> bool {
        (self.bits & ModeCfg::CMD_VIDEO_MODE_MASK) != 0
    }

    /// Set **CMD_VIDEO_MODE** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_cmd_video_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ModeCfg::CMD_VIDEO_MODE_MASK
            } else {
                self.bits & !ModeCfg::CMD_VIDEO_MODE_MASK
            },
        }
    }
}

impl Default for ModeCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ModeCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ModeCfgVal {
        ModeCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ModeCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ModeCfgVal) -> ModeCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VID_MODE_CFG Register
///
/// NA
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **VPG_ORIENTATION** `[24]` (RW)
///
///   NA
///
/// - **VPG_MODE** `[20]` (RW)
///
///   NA
///
/// - **VPG_EN** `[16]` (RW)
///
///   NA
///
/// - **LP_CMD_EN** `[15]` (RW)
///
///   NA
///
/// - **FRAME_BTA_ACK_EN** `[14]` (RW)
///
///   NA
///
/// - **LP_HFP_EN** `[13]` (RW)
///
///   NA
///
/// - **LP_HBP_EN** `[12]` (RW)
///
///   NA
///
/// - **LP_VACT_EN** `[11]` (RW)
///
///   NA
///
/// - **LP_VFP_EN** `[10]` (RW)
///
///   NA
///
/// - **LP_VBP_EN** `[9]` (RW)
///
///   NA
///
/// - **LP_VSA_EN** `[8]` (RW)
///
///   NA
///
/// - **VID_MODE_TYPE** `[1:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct VidModeCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidModeCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidModeCfg {
    // ======== Field Constants ========

    /// **VPG_ORIENTATION**
    ///
    /// NA
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const VPG_ORIENTATION_OFFSET: u32 = 24;
    pub const VPG_ORIENTATION_MASK: u32 = 0x01000000;
    pub const VPG_ORIENTATION: u32 = 1 << 24;

    /// **VPG_MODE**
    ///
    /// NA
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const VPG_MODE_OFFSET: u32 = 20;
    pub const VPG_MODE_MASK: u32 = 0x00100000;
    pub const VPG_MODE: u32 = 1 << 20;

    /// **VPG_EN**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const VPG_EN_OFFSET: u32 = 16;
    pub const VPG_EN_MASK: u32 = 0x00010000;
    pub const VPG_EN: u32 = 1 << 16;

    /// **LP_CMD_EN**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const LP_CMD_EN_OFFSET: u32 = 15;
    pub const LP_CMD_EN_MASK: u32 = 0x00008000;
    pub const LP_CMD_EN: u32 = 1 << 15;

    /// **FRAME_BTA_ACK_EN**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const FRAME_BTA_ACK_EN_OFFSET: u32 = 14;
    pub const FRAME_BTA_ACK_EN_MASK: u32 = 0x00004000;
    pub const FRAME_BTA_ACK_EN: u32 = 1 << 14;

    /// **LP_HFP_EN**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const LP_HFP_EN_OFFSET: u32 = 13;
    pub const LP_HFP_EN_MASK: u32 = 0x00002000;
    pub const LP_HFP_EN: u32 = 1 << 13;

    /// **LP_HBP_EN**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const LP_HBP_EN_OFFSET: u32 = 12;
    pub const LP_HBP_EN_MASK: u32 = 0x00001000;
    pub const LP_HBP_EN: u32 = 1 << 12;

    /// **LP_VACT_EN**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const LP_VACT_EN_OFFSET: u32 = 11;
    pub const LP_VACT_EN_MASK: u32 = 0x00000800;
    pub const LP_VACT_EN: u32 = 1 << 11;

    /// **LP_VFP_EN**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const LP_VFP_EN_OFFSET: u32 = 10;
    pub const LP_VFP_EN_MASK: u32 = 0x00000400;
    pub const LP_VFP_EN: u32 = 1 << 10;

    /// **LP_VBP_EN**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const LP_VBP_EN_OFFSET: u32 = 9;
    pub const LP_VBP_EN_MASK: u32 = 0x00000200;
    pub const LP_VBP_EN: u32 = 1 << 9;

    /// **LP_VSA_EN**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const LP_VSA_EN_OFFSET: u32 = 8;
    pub const LP_VSA_EN_MASK: u32 = 0x00000100;
    pub const LP_VSA_EN: u32 = 1 << 8;

    /// **VID_MODE_TYPE**
    ///
    /// NA
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const VID_MODE_TYPE_OFFSET: u32 = 0;
    pub const VID_MODE_TYPE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **VPG_ORIENTATION** `[24]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vpg_orientation(&self) -> bool {
        (self.value.read_bits() & Self::VPG_ORIENTATION_MASK) != 0
    }

    /// Set VPG_ORIENTATION bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vpg_orientation(&self) {
        self.value.modify_bits(|r| r | Self::VPG_ORIENTATION_MASK);
    }

    /// Clear VPG_ORIENTATION bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vpg_orientation(&self) {
        self.value.modify_bits(|r| r & !Self::VPG_ORIENTATION_MASK);
    }

    /// **VPG_MODE** `[20]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vpg_mode(&self) -> bool {
        (self.value.read_bits() & Self::VPG_MODE_MASK) != 0
    }

    /// Set VPG_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vpg_mode(&self) {
        self.value.modify_bits(|r| r | Self::VPG_MODE_MASK);
    }

    /// Clear VPG_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vpg_mode(&self) {
        self.value.modify_bits(|r| r & !Self::VPG_MODE_MASK);
    }

    /// **VPG_EN** `[16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vpg_en(&self) -> bool {
        (self.value.read_bits() & Self::VPG_EN_MASK) != 0
    }

    /// Set VPG_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vpg_en(&self) {
        self.value.modify_bits(|r| r | Self::VPG_EN_MASK);
    }

    /// Clear VPG_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vpg_en(&self) {
        self.value.modify_bits(|r| r & !Self::VPG_EN_MASK);
    }

    /// **LP_CMD_EN** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_cmd_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_CMD_EN_MASK) != 0
    }

    /// Set LP_CMD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_cmd_en(&self) {
        self.value.modify_bits(|r| r | Self::LP_CMD_EN_MASK);
    }

    /// Clear LP_CMD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_cmd_en(&self) {
        self.value.modify_bits(|r| r & !Self::LP_CMD_EN_MASK);
    }

    /// **FRAME_BTA_ACK_EN** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn frame_bta_ack_en(&self) -> bool {
        (self.value.read_bits() & Self::FRAME_BTA_ACK_EN_MASK) != 0
    }

    /// Set FRAME_BTA_ACK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_frame_bta_ack_en(&self) {
        self.value.modify_bits(|r| r | Self::FRAME_BTA_ACK_EN_MASK);
    }

    /// Clear FRAME_BTA_ACK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_frame_bta_ack_en(&self) {
        self.value.modify_bits(|r| r & !Self::FRAME_BTA_ACK_EN_MASK);
    }

    /// **LP_HFP_EN** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_hfp_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_HFP_EN_MASK) != 0
    }

    /// Set LP_HFP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_hfp_en(&self) {
        self.value.modify_bits(|r| r | Self::LP_HFP_EN_MASK);
    }

    /// Clear LP_HFP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_hfp_en(&self) {
        self.value.modify_bits(|r| r & !Self::LP_HFP_EN_MASK);
    }

    /// **LP_HBP_EN** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_hbp_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_HBP_EN_MASK) != 0
    }

    /// Set LP_HBP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_hbp_en(&self) {
        self.value.modify_bits(|r| r | Self::LP_HBP_EN_MASK);
    }

    /// Clear LP_HBP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_hbp_en(&self) {
        self.value.modify_bits(|r| r & !Self::LP_HBP_EN_MASK);
    }

    /// **LP_VACT_EN** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_vact_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_VACT_EN_MASK) != 0
    }

    /// Set LP_VACT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_vact_en(&self) {
        self.value.modify_bits(|r| r | Self::LP_VACT_EN_MASK);
    }

    /// Clear LP_VACT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_vact_en(&self) {
        self.value.modify_bits(|r| r & !Self::LP_VACT_EN_MASK);
    }

    /// **LP_VFP_EN** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_vfp_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_VFP_EN_MASK) != 0
    }

    /// Set LP_VFP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_vfp_en(&self) {
        self.value.modify_bits(|r| r | Self::LP_VFP_EN_MASK);
    }

    /// Clear LP_VFP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_vfp_en(&self) {
        self.value.modify_bits(|r| r & !Self::LP_VFP_EN_MASK);
    }

    /// **LP_VBP_EN** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_vbp_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_VBP_EN_MASK) != 0
    }

    /// Set LP_VBP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_vbp_en(&self) {
        self.value.modify_bits(|r| r | Self::LP_VBP_EN_MASK);
    }

    /// Clear LP_VBP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_vbp_en(&self) {
        self.value.modify_bits(|r| r & !Self::LP_VBP_EN_MASK);
    }

    /// **LP_VSA_EN** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_vsa_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_VSA_EN_MASK) != 0
    }

    /// Set LP_VSA_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_vsa_en(&self) {
        self.value.modify_bits(|r| r | Self::LP_VSA_EN_MASK);
    }

    /// Clear LP_VSA_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_vsa_en(&self) {
        self.value.modify_bits(|r| r & !Self::LP_VSA_EN_MASK);
    }

    /// **VID_MODE_TYPE** `[1:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_mode_type(&self) -> u32 {
        (self.value.read_bits() & Self::VID_MODE_TYPE_MASK) >> Self::VID_MODE_TYPE_OFFSET
    }

    /// Write **VID_MODE_TYPE** `[1:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_vid_mode_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VID_MODE_TYPE_MASK)
                | ((value << Self::VID_MODE_TYPE_OFFSET) & Self::VID_MODE_TYPE_MASK)
        });
    }
}

/// VID_MODE_CFG value type for batch read/modify/write operations
///
/// Use `vidmodecfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidmodecfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidModeCfgVal {
    bits: u32,
}

impl VidModeCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VPG_ORIENTATION** `[24:24]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vpg_orientation(&self) -> bool {
        (self.bits & VidModeCfg::VPG_ORIENTATION_MASK) != 0
    }

    /// Read **VPG_MODE** `[20:20]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vpg_mode(&self) -> bool {
        (self.bits & VidModeCfg::VPG_MODE_MASK) != 0
    }

    /// Read **VPG_EN** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vpg_en(&self) -> bool {
        (self.bits & VidModeCfg::VPG_EN_MASK) != 0
    }

    /// Read **LP_CMD_EN** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_cmd_en(&self) -> bool {
        (self.bits & VidModeCfg::LP_CMD_EN_MASK) != 0
    }

    /// Read **FRAME_BTA_ACK_EN** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn frame_bta_ack_en(&self) -> bool {
        (self.bits & VidModeCfg::FRAME_BTA_ACK_EN_MASK) != 0
    }

    /// Read **LP_HFP_EN** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_hfp_en(&self) -> bool {
        (self.bits & VidModeCfg::LP_HFP_EN_MASK) != 0
    }

    /// Read **LP_HBP_EN** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_hbp_en(&self) -> bool {
        (self.bits & VidModeCfg::LP_HBP_EN_MASK) != 0
    }

    /// Read **LP_VACT_EN** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_vact_en(&self) -> bool {
        (self.bits & VidModeCfg::LP_VACT_EN_MASK) != 0
    }

    /// Read **LP_VFP_EN** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_vfp_en(&self) -> bool {
        (self.bits & VidModeCfg::LP_VFP_EN_MASK) != 0
    }

    /// Read **LP_VBP_EN** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_vbp_en(&self) -> bool {
        (self.bits & VidModeCfg::LP_VBP_EN_MASK) != 0
    }

    /// Read **LP_VSA_EN** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_vsa_en(&self) -> bool {
        (self.bits & VidModeCfg::LP_VSA_EN_MASK) != 0
    }

    /// Read **VID_MODE_TYPE** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_mode_type(&self) -> u32 {
        (self.bits & VidModeCfg::VID_MODE_TYPE_MASK) >> VidModeCfg::VID_MODE_TYPE_OFFSET
    }

    /// Set **VPG_ORIENTATION** `[24:24]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vpg_orientation(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VidModeCfg::VPG_ORIENTATION_MASK
            } else {
                self.bits & !VidModeCfg::VPG_ORIENTATION_MASK
            },
        }
    }

    /// Set **VPG_MODE** `[20:20]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vpg_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VidModeCfg::VPG_MODE_MASK
            } else {
                self.bits & !VidModeCfg::VPG_MODE_MASK
            },
        }
    }

    /// Set **VPG_EN** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vpg_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VidModeCfg::VPG_EN_MASK
            } else {
                self.bits & !VidModeCfg::VPG_EN_MASK
            },
        }
    }

    /// Set **LP_CMD_EN** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_lp_cmd_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VidModeCfg::LP_CMD_EN_MASK
            } else {
                self.bits & !VidModeCfg::LP_CMD_EN_MASK
            },
        }
    }

    /// Set **FRAME_BTA_ACK_EN** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_frame_bta_ack_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VidModeCfg::FRAME_BTA_ACK_EN_MASK
            } else {
                self.bits & !VidModeCfg::FRAME_BTA_ACK_EN_MASK
            },
        }
    }

    /// Set **LP_HFP_EN** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_lp_hfp_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VidModeCfg::LP_HFP_EN_MASK
            } else {
                self.bits & !VidModeCfg::LP_HFP_EN_MASK
            },
        }
    }

    /// Set **LP_HBP_EN** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_lp_hbp_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VidModeCfg::LP_HBP_EN_MASK
            } else {
                self.bits & !VidModeCfg::LP_HBP_EN_MASK
            },
        }
    }

    /// Set **LP_VACT_EN** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_lp_vact_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VidModeCfg::LP_VACT_EN_MASK
            } else {
                self.bits & !VidModeCfg::LP_VACT_EN_MASK
            },
        }
    }

    /// Set **LP_VFP_EN** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_lp_vfp_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VidModeCfg::LP_VFP_EN_MASK
            } else {
                self.bits & !VidModeCfg::LP_VFP_EN_MASK
            },
        }
    }

    /// Set **LP_VBP_EN** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_lp_vbp_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VidModeCfg::LP_VBP_EN_MASK
            } else {
                self.bits & !VidModeCfg::LP_VBP_EN_MASK
            },
        }
    }

    /// Set **LP_VSA_EN** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_lp_vsa_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VidModeCfg::LP_VSA_EN_MASK
            } else {
                self.bits & !VidModeCfg::LP_VSA_EN_MASK
            },
        }
    }

    /// Set **VID_MODE_TYPE** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vid_mode_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VidModeCfg::VID_MODE_TYPE_MASK)
                | ((value << VidModeCfg::VID_MODE_TYPE_OFFSET) & VidModeCfg::VID_MODE_TYPE_MASK),
        }
    }
}

impl Default for VidModeCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidModeCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidModeCfgVal {
        VidModeCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VidModeCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VidModeCfgVal) -> VidModeCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VID_PKT_SIZE Register
///
/// NA
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **VID_PKT_SIZE** `[13:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct VidPktSize {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidPktSize {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidPktSize {
    // ======== Field Constants ========

    /// **VID_PKT_SIZE**
    ///
    /// NA
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const VID_PKT_SIZE_OFFSET: u32 = 0;
    pub const VID_PKT_SIZE_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **VID_PKT_SIZE** `[13:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_pkt_size(&self) -> u32 {
        (self.value.read_bits() & Self::VID_PKT_SIZE_MASK) >> Self::VID_PKT_SIZE_OFFSET
    }

    /// Write **VID_PKT_SIZE** `[13:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_vid_pkt_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VID_PKT_SIZE_MASK)
                | ((value << Self::VID_PKT_SIZE_OFFSET) & Self::VID_PKT_SIZE_MASK)
        });
    }
}

/// VID_PKT_SIZE value type for batch read/modify/write operations
///
/// Use `vidpktsize.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidpktsize.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidPktSizeVal {
    bits: u32,
}

impl VidPktSizeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VID_PKT_SIZE** `[13:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_pkt_size(&self) -> u32 {
        (self.bits & VidPktSize::VID_PKT_SIZE_MASK) >> VidPktSize::VID_PKT_SIZE_OFFSET
    }

    /// Set **VID_PKT_SIZE** `[13:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vid_pkt_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VidPktSize::VID_PKT_SIZE_MASK)
                | ((value << VidPktSize::VID_PKT_SIZE_OFFSET) & VidPktSize::VID_PKT_SIZE_MASK),
        }
    }
}

impl Default for VidPktSizeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidPktSize {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidPktSizeVal {
        VidPktSizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VidPktSizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VidPktSizeVal) -> VidPktSizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VID_NUM_CHUNKS Register
///
/// NA
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **VID_NUM_CHUNKS** `[12:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct VidNumChunks {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidNumChunks {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidNumChunks {
    // ======== Field Constants ========

    /// **VID_NUM_CHUNKS**
    ///
    /// NA
    ///
    /// Bits `[12:0]`, mask `0x00001fff`
    pub const VID_NUM_CHUNKS_OFFSET: u32 = 0;
    pub const VID_NUM_CHUNKS_MASK: u32 = 0x00001fff;

    // ======== Field Accessors ========

    /// **VID_NUM_CHUNKS** `[12:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_num_chunks(&self) -> u32 {
        (self.value.read_bits() & Self::VID_NUM_CHUNKS_MASK) >> Self::VID_NUM_CHUNKS_OFFSET
    }

    /// Write **VID_NUM_CHUNKS** `[12:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_vid_num_chunks(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VID_NUM_CHUNKS_MASK)
                | ((value << Self::VID_NUM_CHUNKS_OFFSET) & Self::VID_NUM_CHUNKS_MASK)
        });
    }
}

/// VID_NUM_CHUNKS value type for batch read/modify/write operations
///
/// Use `vidnumchunks.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidnumchunks.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidNumChunksVal {
    bits: u32,
}

impl VidNumChunksVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VID_NUM_CHUNKS** `[12:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_num_chunks(&self) -> u32 {
        (self.bits & VidNumChunks::VID_NUM_CHUNKS_MASK) >> VidNumChunks::VID_NUM_CHUNKS_OFFSET
    }

    /// Set **VID_NUM_CHUNKS** `[12:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vid_num_chunks(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VidNumChunks::VID_NUM_CHUNKS_MASK)
                | ((value << VidNumChunks::VID_NUM_CHUNKS_OFFSET)
                    & VidNumChunks::VID_NUM_CHUNKS_MASK),
        }
    }
}

impl Default for VidNumChunksVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidNumChunks {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidNumChunksVal {
        VidNumChunksVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VidNumChunksVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VidNumChunksVal) -> VidNumChunksVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VID_NULL_SIZE Register
///
/// NA
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **VID_NULL_SIZE** `[12:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct VidNullSize {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidNullSize {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidNullSize {
    // ======== Field Constants ========

    /// **VID_NULL_SIZE**
    ///
    /// NA
    ///
    /// Bits `[12:0]`, mask `0x00001fff`
    pub const VID_NULL_SIZE_OFFSET: u32 = 0;
    pub const VID_NULL_SIZE_MASK: u32 = 0x00001fff;

    // ======== Field Accessors ========

    /// **VID_NULL_SIZE** `[12:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_null_size(&self) -> u32 {
        (self.value.read_bits() & Self::VID_NULL_SIZE_MASK) >> Self::VID_NULL_SIZE_OFFSET
    }

    /// Write **VID_NULL_SIZE** `[12:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_vid_null_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VID_NULL_SIZE_MASK)
                | ((value << Self::VID_NULL_SIZE_OFFSET) & Self::VID_NULL_SIZE_MASK)
        });
    }
}

/// VID_NULL_SIZE value type for batch read/modify/write operations
///
/// Use `vidnullsize.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidnullsize.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidNullSizeVal {
    bits: u32,
}

impl VidNullSizeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VID_NULL_SIZE** `[12:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_null_size(&self) -> u32 {
        (self.bits & VidNullSize::VID_NULL_SIZE_MASK) >> VidNullSize::VID_NULL_SIZE_OFFSET
    }

    /// Set **VID_NULL_SIZE** `[12:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vid_null_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VidNullSize::VID_NULL_SIZE_MASK)
                | ((value << VidNullSize::VID_NULL_SIZE_OFFSET) & VidNullSize::VID_NULL_SIZE_MASK),
        }
    }
}

impl Default for VidNullSizeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidNullSize {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidNullSizeVal {
        VidNullSizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VidNullSizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VidNullSizeVal) -> VidNullSizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VID_HSA_TIME Register
///
/// NA
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **VID_HSA_TIME** `[11:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct VidHsaTime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidHsaTime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidHsaTime {
    // ======== Field Constants ========

    /// **VID_HSA_TIME**
    ///
    /// NA
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const VID_HSA_TIME_OFFSET: u32 = 0;
    pub const VID_HSA_TIME_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **VID_HSA_TIME** `[11:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_hsa_time(&self) -> u32 {
        (self.value.read_bits() & Self::VID_HSA_TIME_MASK) >> Self::VID_HSA_TIME_OFFSET
    }

    /// Write **VID_HSA_TIME** `[11:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_vid_hsa_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VID_HSA_TIME_MASK)
                | ((value << Self::VID_HSA_TIME_OFFSET) & Self::VID_HSA_TIME_MASK)
        });
    }
}

/// VID_HSA_TIME value type for batch read/modify/write operations
///
/// Use `vidhsatime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidhsatime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidHsaTimeVal {
    bits: u32,
}

impl VidHsaTimeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VID_HSA_TIME** `[11:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_hsa_time(&self) -> u32 {
        (self.bits & VidHsaTime::VID_HSA_TIME_MASK) >> VidHsaTime::VID_HSA_TIME_OFFSET
    }

    /// Set **VID_HSA_TIME** `[11:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vid_hsa_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VidHsaTime::VID_HSA_TIME_MASK)
                | ((value << VidHsaTime::VID_HSA_TIME_OFFSET) & VidHsaTime::VID_HSA_TIME_MASK),
        }
    }
}

impl Default for VidHsaTimeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidHsaTime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidHsaTimeVal {
        VidHsaTimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VidHsaTimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VidHsaTimeVal) -> VidHsaTimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VID_HBP_TIME Register
///
/// NA
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **VID_HBP_TIME** `[11:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct VidHbpTime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidHbpTime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidHbpTime {
    // ======== Field Constants ========

    /// **VID_HBP_TIME**
    ///
    /// NA
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const VID_HBP_TIME_OFFSET: u32 = 0;
    pub const VID_HBP_TIME_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **VID_HBP_TIME** `[11:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_hbp_time(&self) -> u32 {
        (self.value.read_bits() & Self::VID_HBP_TIME_MASK) >> Self::VID_HBP_TIME_OFFSET
    }

    /// Write **VID_HBP_TIME** `[11:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_vid_hbp_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VID_HBP_TIME_MASK)
                | ((value << Self::VID_HBP_TIME_OFFSET) & Self::VID_HBP_TIME_MASK)
        });
    }
}

/// VID_HBP_TIME value type for batch read/modify/write operations
///
/// Use `vidhbptime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidhbptime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidHbpTimeVal {
    bits: u32,
}

impl VidHbpTimeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VID_HBP_TIME** `[11:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_hbp_time(&self) -> u32 {
        (self.bits & VidHbpTime::VID_HBP_TIME_MASK) >> VidHbpTime::VID_HBP_TIME_OFFSET
    }

    /// Set **VID_HBP_TIME** `[11:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vid_hbp_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VidHbpTime::VID_HBP_TIME_MASK)
                | ((value << VidHbpTime::VID_HBP_TIME_OFFSET) & VidHbpTime::VID_HBP_TIME_MASK),
        }
    }
}

impl Default for VidHbpTimeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidHbpTime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidHbpTimeVal {
        VidHbpTimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VidHbpTimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VidHbpTimeVal) -> VidHbpTimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VID_HLINE_TIME Register
///
/// NA
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **VID_HLINE_TIME** `[14:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct VidHlineTime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidHlineTime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidHlineTime {
    // ======== Field Constants ========

    /// **VID_HLINE_TIME**
    ///
    /// NA
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const VID_HLINE_TIME_OFFSET: u32 = 0;
    pub const VID_HLINE_TIME_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **VID_HLINE_TIME** `[14:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_hline_time(&self) -> u32 {
        (self.value.read_bits() & Self::VID_HLINE_TIME_MASK) >> Self::VID_HLINE_TIME_OFFSET
    }

    /// Write **VID_HLINE_TIME** `[14:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_vid_hline_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VID_HLINE_TIME_MASK)
                | ((value << Self::VID_HLINE_TIME_OFFSET) & Self::VID_HLINE_TIME_MASK)
        });
    }
}

/// VID_HLINE_TIME value type for batch read/modify/write operations
///
/// Use `vidhlinetime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidhlinetime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidHlineTimeVal {
    bits: u32,
}

impl VidHlineTimeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VID_HLINE_TIME** `[14:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_hline_time(&self) -> u32 {
        (self.bits & VidHlineTime::VID_HLINE_TIME_MASK) >> VidHlineTime::VID_HLINE_TIME_OFFSET
    }

    /// Set **VID_HLINE_TIME** `[14:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vid_hline_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VidHlineTime::VID_HLINE_TIME_MASK)
                | ((value << VidHlineTime::VID_HLINE_TIME_OFFSET)
                    & VidHlineTime::VID_HLINE_TIME_MASK),
        }
    }
}

impl Default for VidHlineTimeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidHlineTime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidHlineTimeVal {
        VidHlineTimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VidHlineTimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VidHlineTimeVal) -> VidHlineTimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VID_VSA_LINES Register
///
/// NA
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **VSA_LINES** `[9:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct VidVsaLines {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidVsaLines {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidVsaLines {
    // ======== Field Constants ========

    /// **VSA_LINES**
    ///
    /// NA
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const VSA_LINES_OFFSET: u32 = 0;
    pub const VSA_LINES_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **VSA_LINES** `[9:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vsa_lines(&self) -> u32 {
        (self.value.read_bits() & Self::VSA_LINES_MASK) >> Self::VSA_LINES_OFFSET
    }

    /// Write **VSA_LINES** `[9:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_vsa_lines(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VSA_LINES_MASK) | ((value << Self::VSA_LINES_OFFSET) & Self::VSA_LINES_MASK)
        });
    }
}

/// VID_VSA_LINES value type for batch read/modify/write operations
///
/// Use `vidvsalines.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidvsalines.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidVsaLinesVal {
    bits: u32,
}

impl VidVsaLinesVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VSA_LINES** `[9:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vsa_lines(&self) -> u32 {
        (self.bits & VidVsaLines::VSA_LINES_MASK) >> VidVsaLines::VSA_LINES_OFFSET
    }

    /// Set **VSA_LINES** `[9:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vsa_lines(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VidVsaLines::VSA_LINES_MASK)
                | ((value << VidVsaLines::VSA_LINES_OFFSET) & VidVsaLines::VSA_LINES_MASK),
        }
    }
}

impl Default for VidVsaLinesVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidVsaLines {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidVsaLinesVal {
        VidVsaLinesVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VidVsaLinesVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VidVsaLinesVal) -> VidVsaLinesVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VID_VBP_LINES Register
///
/// NA
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **VBP_LINES** `[9:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct VidVbpLines {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidVbpLines {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidVbpLines {
    // ======== Field Constants ========

    /// **VBP_LINES**
    ///
    /// NA
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const VBP_LINES_OFFSET: u32 = 0;
    pub const VBP_LINES_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **VBP_LINES** `[9:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vbp_lines(&self) -> u32 {
        (self.value.read_bits() & Self::VBP_LINES_MASK) >> Self::VBP_LINES_OFFSET
    }

    /// Write **VBP_LINES** `[9:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_vbp_lines(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VBP_LINES_MASK) | ((value << Self::VBP_LINES_OFFSET) & Self::VBP_LINES_MASK)
        });
    }
}

/// VID_VBP_LINES value type for batch read/modify/write operations
///
/// Use `vidvbplines.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidvbplines.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidVbpLinesVal {
    bits: u32,
}

impl VidVbpLinesVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VBP_LINES** `[9:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vbp_lines(&self) -> u32 {
        (self.bits & VidVbpLines::VBP_LINES_MASK) >> VidVbpLines::VBP_LINES_OFFSET
    }

    /// Set **VBP_LINES** `[9:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vbp_lines(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VidVbpLines::VBP_LINES_MASK)
                | ((value << VidVbpLines::VBP_LINES_OFFSET) & VidVbpLines::VBP_LINES_MASK),
        }
    }
}

impl Default for VidVbpLinesVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidVbpLines {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidVbpLinesVal {
        VidVbpLinesVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VidVbpLinesVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VidVbpLinesVal) -> VidVbpLinesVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VID_VFP_LINES Register
///
/// NA
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **VFP_LINES** `[9:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct VidVfpLines {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidVfpLines {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidVfpLines {
    // ======== Field Constants ========

    /// **VFP_LINES**
    ///
    /// NA
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const VFP_LINES_OFFSET: u32 = 0;
    pub const VFP_LINES_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **VFP_LINES** `[9:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vfp_lines(&self) -> u32 {
        (self.value.read_bits() & Self::VFP_LINES_MASK) >> Self::VFP_LINES_OFFSET
    }

    /// Write **VFP_LINES** `[9:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_vfp_lines(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VFP_LINES_MASK) | ((value << Self::VFP_LINES_OFFSET) & Self::VFP_LINES_MASK)
        });
    }
}

/// VID_VFP_LINES value type for batch read/modify/write operations
///
/// Use `vidvfplines.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidvfplines.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidVfpLinesVal {
    bits: u32,
}

impl VidVfpLinesVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VFP_LINES** `[9:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vfp_lines(&self) -> u32 {
        (self.bits & VidVfpLines::VFP_LINES_MASK) >> VidVfpLines::VFP_LINES_OFFSET
    }

    /// Set **VFP_LINES** `[9:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vfp_lines(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VidVfpLines::VFP_LINES_MASK)
                | ((value << VidVfpLines::VFP_LINES_OFFSET) & VidVfpLines::VFP_LINES_MASK),
        }
    }
}

impl Default for VidVfpLinesVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidVfpLines {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidVfpLinesVal {
        VidVfpLinesVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VidVfpLinesVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VidVfpLinesVal) -> VidVfpLinesVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VID_VACTIVE_LINES Register
///
/// NA
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **V_ACTIVE_LINES** `[13:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct VidVactiveLines {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidVactiveLines {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidVactiveLines {
    // ======== Field Constants ========

    /// **V_ACTIVE_LINES**
    ///
    /// NA
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const V_ACTIVE_LINES_OFFSET: u32 = 0;
    pub const V_ACTIVE_LINES_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **V_ACTIVE_LINES** `[13:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn v_active_lines(&self) -> u32 {
        (self.value.read_bits() & Self::V_ACTIVE_LINES_MASK) >> Self::V_ACTIVE_LINES_OFFSET
    }

    /// Write **V_ACTIVE_LINES** `[13:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_v_active_lines(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::V_ACTIVE_LINES_MASK)
                | ((value << Self::V_ACTIVE_LINES_OFFSET) & Self::V_ACTIVE_LINES_MASK)
        });
    }
}

/// VID_VACTIVE_LINES value type for batch read/modify/write operations
///
/// Use `vidvactivelines.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidvactivelines.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidVactiveLinesVal {
    bits: u32,
}

impl VidVactiveLinesVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **V_ACTIVE_LINES** `[13:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn v_active_lines(&self) -> u32 {
        (self.bits & VidVactiveLines::V_ACTIVE_LINES_MASK) >> VidVactiveLines::V_ACTIVE_LINES_OFFSET
    }

    /// Set **V_ACTIVE_LINES** `[13:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_v_active_lines(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VidVactiveLines::V_ACTIVE_LINES_MASK)
                | ((value << VidVactiveLines::V_ACTIVE_LINES_OFFSET)
                    & VidVactiveLines::V_ACTIVE_LINES_MASK),
        }
    }
}

impl Default for VidVactiveLinesVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidVactiveLines {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidVactiveLinesVal {
        VidVactiveLinesVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VidVactiveLinesVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VidVactiveLinesVal) -> VidVactiveLinesVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EDPI_CMD_SIZE Register
///
/// NA
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **EDPI_ALLOWED_CMD_SIZE** `[15:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct EdpiCmdSize {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EdpiCmdSize {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EdpiCmdSize {
    // ======== Field Constants ========

    /// **EDPI_ALLOWED_CMD_SIZE**
    ///
    /// NA
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const EDPI_ALLOWED_CMD_SIZE_OFFSET: u32 = 0;
    pub const EDPI_ALLOWED_CMD_SIZE_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **EDPI_ALLOWED_CMD_SIZE** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn edpi_allowed_cmd_size(&self) -> u32 {
        (self.value.read_bits() & Self::EDPI_ALLOWED_CMD_SIZE_MASK)
            >> Self::EDPI_ALLOWED_CMD_SIZE_OFFSET
    }

    /// Write **EDPI_ALLOWED_CMD_SIZE** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_edpi_allowed_cmd_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EDPI_ALLOWED_CMD_SIZE_MASK)
                | ((value << Self::EDPI_ALLOWED_CMD_SIZE_OFFSET) & Self::EDPI_ALLOWED_CMD_SIZE_MASK)
        });
    }
}

/// EDPI_CMD_SIZE value type for batch read/modify/write operations
///
/// Use `edpicmdsize.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `edpicmdsize.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EdpiCmdSizeVal {
    bits: u32,
}

impl EdpiCmdSizeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **EDPI_ALLOWED_CMD_SIZE** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn edpi_allowed_cmd_size(&self) -> u32 {
        (self.bits & EdpiCmdSize::EDPI_ALLOWED_CMD_SIZE_MASK)
            >> EdpiCmdSize::EDPI_ALLOWED_CMD_SIZE_OFFSET
    }

    /// Set **EDPI_ALLOWED_CMD_SIZE** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_edpi_allowed_cmd_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EdpiCmdSize::EDPI_ALLOWED_CMD_SIZE_MASK)
                | ((value << EdpiCmdSize::EDPI_ALLOWED_CMD_SIZE_OFFSET)
                    & EdpiCmdSize::EDPI_ALLOWED_CMD_SIZE_MASK),
        }
    }
}

impl Default for EdpiCmdSizeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EdpiCmdSize {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EdpiCmdSizeVal {
        EdpiCmdSizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EdpiCmdSizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EdpiCmdSizeVal) -> EdpiCmdSizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CMD_MODE_CFG Register
///
/// NA
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **MAX_RD_PKT_SIZE** `[24]` (RW)
///
///   NA
///
/// - **DCS_LW_TX** `[19]` (RW)
///
///   NA
///
/// - **DCS_SR_0P_TX** `[18]` (RW)
///
///   NA
///
/// - **DCS_SW_1P_TX** `[17]` (RW)
///
///   NA
///
/// - **DCS_SW_0P_TX** `[16]` (RW)
///
///   NA
///
/// - **GEN_LW_TX** `[14]` (RW)
///
///   NA
///
/// - **GEN_SR_2P_TX** `[13]` (RW)
///
///   NA
///
/// - **GEN_SR_1P_TX** `[12]` (RW)
///
///   NA
///
/// - **GEN_SR_0P_TX** `[11]` (RW)
///
///   NA
///
/// - **GEN_SW_2P_TX** `[10]` (RW)
///
///   NA
///
/// - **GEN_SW_1P_TX** `[9]` (RW)
///
///   NA
///
/// - **GEN_SW_0P_TX** `[8]` (RW)
///
///   NA
///
/// - **ACK_RQST_EN** `[1]` (RW)
///
///   NA
///
/// - **TEAR_FX_EN** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct CmdModeCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CmdModeCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CmdModeCfg {
    // ======== Field Constants ========

    /// **MAX_RD_PKT_SIZE**
    ///
    /// NA
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const MAX_RD_PKT_SIZE_OFFSET: u32 = 24;
    pub const MAX_RD_PKT_SIZE_MASK: u32 = 0x01000000;
    pub const MAX_RD_PKT_SIZE: u32 = 1 << 24;

    /// **DCS_LW_TX**
    ///
    /// NA
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const DCS_LW_TX_OFFSET: u32 = 19;
    pub const DCS_LW_TX_MASK: u32 = 0x00080000;
    pub const DCS_LW_TX: u32 = 1 << 19;

    /// **DCS_SR_0P_TX**
    ///
    /// NA
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const DCS_SR_0P_TX_OFFSET: u32 = 18;
    pub const DCS_SR_0P_TX_MASK: u32 = 0x00040000;
    pub const DCS_SR_0P_TX: u32 = 1 << 18;

    /// **DCS_SW_1P_TX**
    ///
    /// NA
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const DCS_SW_1P_TX_OFFSET: u32 = 17;
    pub const DCS_SW_1P_TX_MASK: u32 = 0x00020000;
    pub const DCS_SW_1P_TX: u32 = 1 << 17;

    /// **DCS_SW_0P_TX**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const DCS_SW_0P_TX_OFFSET: u32 = 16;
    pub const DCS_SW_0P_TX_MASK: u32 = 0x00010000;
    pub const DCS_SW_0P_TX: u32 = 1 << 16;

    /// **GEN_LW_TX**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const GEN_LW_TX_OFFSET: u32 = 14;
    pub const GEN_LW_TX_MASK: u32 = 0x00004000;
    pub const GEN_LW_TX: u32 = 1 << 14;

    /// **GEN_SR_2P_TX**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const GEN_SR_2P_TX_OFFSET: u32 = 13;
    pub const GEN_SR_2P_TX_MASK: u32 = 0x00002000;
    pub const GEN_SR_2P_TX: u32 = 1 << 13;

    /// **GEN_SR_1P_TX**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const GEN_SR_1P_TX_OFFSET: u32 = 12;
    pub const GEN_SR_1P_TX_MASK: u32 = 0x00001000;
    pub const GEN_SR_1P_TX: u32 = 1 << 12;

    /// **GEN_SR_0P_TX**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const GEN_SR_0P_TX_OFFSET: u32 = 11;
    pub const GEN_SR_0P_TX_MASK: u32 = 0x00000800;
    pub const GEN_SR_0P_TX: u32 = 1 << 11;

    /// **GEN_SW_2P_TX**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const GEN_SW_2P_TX_OFFSET: u32 = 10;
    pub const GEN_SW_2P_TX_MASK: u32 = 0x00000400;
    pub const GEN_SW_2P_TX: u32 = 1 << 10;

    /// **GEN_SW_1P_TX**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const GEN_SW_1P_TX_OFFSET: u32 = 9;
    pub const GEN_SW_1P_TX_MASK: u32 = 0x00000200;
    pub const GEN_SW_1P_TX: u32 = 1 << 9;

    /// **GEN_SW_0P_TX**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const GEN_SW_0P_TX_OFFSET: u32 = 8;
    pub const GEN_SW_0P_TX_MASK: u32 = 0x00000100;
    pub const GEN_SW_0P_TX: u32 = 1 << 8;

    /// **ACK_RQST_EN**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ACK_RQST_EN_OFFSET: u32 = 1;
    pub const ACK_RQST_EN_MASK: u32 = 0x00000002;
    pub const ACK_RQST_EN: u32 = 1 << 1;

    /// **TEAR_FX_EN**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TEAR_FX_EN_OFFSET: u32 = 0;
    pub const TEAR_FX_EN_MASK: u32 = 0x00000001;
    pub const TEAR_FX_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MAX_RD_PKT_SIZE** `[24]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn max_rd_pkt_size(&self) -> bool {
        (self.value.read_bits() & Self::MAX_RD_PKT_SIZE_MASK) != 0
    }

    /// Set MAX_RD_PKT_SIZE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_max_rd_pkt_size(&self) {
        self.value.modify_bits(|r| r | Self::MAX_RD_PKT_SIZE_MASK);
    }

    /// Clear MAX_RD_PKT_SIZE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_max_rd_pkt_size(&self) {
        self.value.modify_bits(|r| r & !Self::MAX_RD_PKT_SIZE_MASK);
    }

    /// **DCS_LW_TX** `[19]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn dcs_lw_tx(&self) -> bool {
        (self.value.read_bits() & Self::DCS_LW_TX_MASK) != 0
    }

    /// Set DCS_LW_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dcs_lw_tx(&self) {
        self.value.modify_bits(|r| r | Self::DCS_LW_TX_MASK);
    }

    /// Clear DCS_LW_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dcs_lw_tx(&self) {
        self.value.modify_bits(|r| r & !Self::DCS_LW_TX_MASK);
    }

    /// **DCS_SR_0P_TX** `[18]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn dcs_sr_0p_tx(&self) -> bool {
        (self.value.read_bits() & Self::DCS_SR_0P_TX_MASK) != 0
    }

    /// Set DCS_SR_0P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dcs_sr_0p_tx(&self) {
        self.value.modify_bits(|r| r | Self::DCS_SR_0P_TX_MASK);
    }

    /// Clear DCS_SR_0P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dcs_sr_0p_tx(&self) {
        self.value.modify_bits(|r| r & !Self::DCS_SR_0P_TX_MASK);
    }

    /// **DCS_SW_1P_TX** `[17]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn dcs_sw_1p_tx(&self) -> bool {
        (self.value.read_bits() & Self::DCS_SW_1P_TX_MASK) != 0
    }

    /// Set DCS_SW_1P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dcs_sw_1p_tx(&self) {
        self.value.modify_bits(|r| r | Self::DCS_SW_1P_TX_MASK);
    }

    /// Clear DCS_SW_1P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dcs_sw_1p_tx(&self) {
        self.value.modify_bits(|r| r & !Self::DCS_SW_1P_TX_MASK);
    }

    /// **DCS_SW_0P_TX** `[16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn dcs_sw_0p_tx(&self) -> bool {
        (self.value.read_bits() & Self::DCS_SW_0P_TX_MASK) != 0
    }

    /// Set DCS_SW_0P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dcs_sw_0p_tx(&self) {
        self.value.modify_bits(|r| r | Self::DCS_SW_0P_TX_MASK);
    }

    /// Clear DCS_SW_0P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dcs_sw_0p_tx(&self) {
        self.value.modify_bits(|r| r & !Self::DCS_SW_0P_TX_MASK);
    }

    /// **GEN_LW_TX** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_lw_tx(&self) -> bool {
        (self.value.read_bits() & Self::GEN_LW_TX_MASK) != 0
    }

    /// Set GEN_LW_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gen_lw_tx(&self) {
        self.value.modify_bits(|r| r | Self::GEN_LW_TX_MASK);
    }

    /// Clear GEN_LW_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gen_lw_tx(&self) {
        self.value.modify_bits(|r| r & !Self::GEN_LW_TX_MASK);
    }

    /// **GEN_SR_2P_TX** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_sr_2p_tx(&self) -> bool {
        (self.value.read_bits() & Self::GEN_SR_2P_TX_MASK) != 0
    }

    /// Set GEN_SR_2P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gen_sr_2p_tx(&self) {
        self.value.modify_bits(|r| r | Self::GEN_SR_2P_TX_MASK);
    }

    /// Clear GEN_SR_2P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gen_sr_2p_tx(&self) {
        self.value.modify_bits(|r| r & !Self::GEN_SR_2P_TX_MASK);
    }

    /// **GEN_SR_1P_TX** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_sr_1p_tx(&self) -> bool {
        (self.value.read_bits() & Self::GEN_SR_1P_TX_MASK) != 0
    }

    /// Set GEN_SR_1P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gen_sr_1p_tx(&self) {
        self.value.modify_bits(|r| r | Self::GEN_SR_1P_TX_MASK);
    }

    /// Clear GEN_SR_1P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gen_sr_1p_tx(&self) {
        self.value.modify_bits(|r| r & !Self::GEN_SR_1P_TX_MASK);
    }

    /// **GEN_SR_0P_TX** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_sr_0p_tx(&self) -> bool {
        (self.value.read_bits() & Self::GEN_SR_0P_TX_MASK) != 0
    }

    /// Set GEN_SR_0P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gen_sr_0p_tx(&self) {
        self.value.modify_bits(|r| r | Self::GEN_SR_0P_TX_MASK);
    }

    /// Clear GEN_SR_0P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gen_sr_0p_tx(&self) {
        self.value.modify_bits(|r| r & !Self::GEN_SR_0P_TX_MASK);
    }

    /// **GEN_SW_2P_TX** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_sw_2p_tx(&self) -> bool {
        (self.value.read_bits() & Self::GEN_SW_2P_TX_MASK) != 0
    }

    /// Set GEN_SW_2P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gen_sw_2p_tx(&self) {
        self.value.modify_bits(|r| r | Self::GEN_SW_2P_TX_MASK);
    }

    /// Clear GEN_SW_2P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gen_sw_2p_tx(&self) {
        self.value.modify_bits(|r| r & !Self::GEN_SW_2P_TX_MASK);
    }

    /// **GEN_SW_1P_TX** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_sw_1p_tx(&self) -> bool {
        (self.value.read_bits() & Self::GEN_SW_1P_TX_MASK) != 0
    }

    /// Set GEN_SW_1P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gen_sw_1p_tx(&self) {
        self.value.modify_bits(|r| r | Self::GEN_SW_1P_TX_MASK);
    }

    /// Clear GEN_SW_1P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gen_sw_1p_tx(&self) {
        self.value.modify_bits(|r| r & !Self::GEN_SW_1P_TX_MASK);
    }

    /// **GEN_SW_0P_TX** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_sw_0p_tx(&self) -> bool {
        (self.value.read_bits() & Self::GEN_SW_0P_TX_MASK) != 0
    }

    /// Set GEN_SW_0P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gen_sw_0p_tx(&self) {
        self.value.modify_bits(|r| r | Self::GEN_SW_0P_TX_MASK);
    }

    /// Clear GEN_SW_0P_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gen_sw_0p_tx(&self) {
        self.value.modify_bits(|r| r & !Self::GEN_SW_0P_TX_MASK);
    }

    /// **ACK_RQST_EN** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_rqst_en(&self) -> bool {
        (self.value.read_bits() & Self::ACK_RQST_EN_MASK) != 0
    }

    /// Set ACK_RQST_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ack_rqst_en(&self) {
        self.value.modify_bits(|r| r | Self::ACK_RQST_EN_MASK);
    }

    /// Clear ACK_RQST_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ack_rqst_en(&self) {
        self.value.modify_bits(|r| r & !Self::ACK_RQST_EN_MASK);
    }

    /// **TEAR_FX_EN** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn tear_fx_en(&self) -> bool {
        (self.value.read_bits() & Self::TEAR_FX_EN_MASK) != 0
    }

    /// Set TEAR_FX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tear_fx_en(&self) {
        self.value.modify_bits(|r| r | Self::TEAR_FX_EN_MASK);
    }

    /// Clear TEAR_FX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tear_fx_en(&self) {
        self.value.modify_bits(|r| r & !Self::TEAR_FX_EN_MASK);
    }
}

/// CMD_MODE_CFG value type for batch read/modify/write operations
///
/// Use `cmdmodecfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cmdmodecfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CmdModeCfgVal {
    bits: u32,
}

impl CmdModeCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAX_RD_PKT_SIZE** `[24:24]`
    ///
    /// NA
    #[inline(always)]
    pub const fn max_rd_pkt_size(&self) -> bool {
        (self.bits & CmdModeCfg::MAX_RD_PKT_SIZE_MASK) != 0
    }

    /// Read **DCS_LW_TX** `[19:19]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dcs_lw_tx(&self) -> bool {
        (self.bits & CmdModeCfg::DCS_LW_TX_MASK) != 0
    }

    /// Read **DCS_SR_0P_TX** `[18:18]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dcs_sr_0p_tx(&self) -> bool {
        (self.bits & CmdModeCfg::DCS_SR_0P_TX_MASK) != 0
    }

    /// Read **DCS_SW_1P_TX** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dcs_sw_1p_tx(&self) -> bool {
        (self.bits & CmdModeCfg::DCS_SW_1P_TX_MASK) != 0
    }

    /// Read **DCS_SW_0P_TX** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dcs_sw_0p_tx(&self) -> bool {
        (self.bits & CmdModeCfg::DCS_SW_0P_TX_MASK) != 0
    }

    /// Read **GEN_LW_TX** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_lw_tx(&self) -> bool {
        (self.bits & CmdModeCfg::GEN_LW_TX_MASK) != 0
    }

    /// Read **GEN_SR_2P_TX** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_sr_2p_tx(&self) -> bool {
        (self.bits & CmdModeCfg::GEN_SR_2P_TX_MASK) != 0
    }

    /// Read **GEN_SR_1P_TX** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_sr_1p_tx(&self) -> bool {
        (self.bits & CmdModeCfg::GEN_SR_1P_TX_MASK) != 0
    }

    /// Read **GEN_SR_0P_TX** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_sr_0p_tx(&self) -> bool {
        (self.bits & CmdModeCfg::GEN_SR_0P_TX_MASK) != 0
    }

    /// Read **GEN_SW_2P_TX** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_sw_2p_tx(&self) -> bool {
        (self.bits & CmdModeCfg::GEN_SW_2P_TX_MASK) != 0
    }

    /// Read **GEN_SW_1P_TX** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_sw_1p_tx(&self) -> bool {
        (self.bits & CmdModeCfg::GEN_SW_1P_TX_MASK) != 0
    }

    /// Read **GEN_SW_0P_TX** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_sw_0p_tx(&self) -> bool {
        (self.bits & CmdModeCfg::GEN_SW_0P_TX_MASK) != 0
    }

    /// Read **ACK_RQST_EN** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_rqst_en(&self) -> bool {
        (self.bits & CmdModeCfg::ACK_RQST_EN_MASK) != 0
    }

    /// Read **TEAR_FX_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn tear_fx_en(&self) -> bool {
        (self.bits & CmdModeCfg::TEAR_FX_EN_MASK) != 0
    }

    /// Set **MAX_RD_PKT_SIZE** `[24:24]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_max_rd_pkt_size(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CmdModeCfg::MAX_RD_PKT_SIZE_MASK
            } else {
                self.bits & !CmdModeCfg::MAX_RD_PKT_SIZE_MASK
            },
        }
    }

    /// Set **DCS_LW_TX** `[19:19]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_dcs_lw_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CmdModeCfg::DCS_LW_TX_MASK
            } else {
                self.bits & !CmdModeCfg::DCS_LW_TX_MASK
            },
        }
    }

    /// Set **DCS_SR_0P_TX** `[18:18]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_dcs_sr_0p_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CmdModeCfg::DCS_SR_0P_TX_MASK
            } else {
                self.bits & !CmdModeCfg::DCS_SR_0P_TX_MASK
            },
        }
    }

    /// Set **DCS_SW_1P_TX** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_dcs_sw_1p_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CmdModeCfg::DCS_SW_1P_TX_MASK
            } else {
                self.bits & !CmdModeCfg::DCS_SW_1P_TX_MASK
            },
        }
    }

    /// Set **DCS_SW_0P_TX** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_dcs_sw_0p_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CmdModeCfg::DCS_SW_0P_TX_MASK
            } else {
                self.bits & !CmdModeCfg::DCS_SW_0P_TX_MASK
            },
        }
    }

    /// Set **GEN_LW_TX** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_lw_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CmdModeCfg::GEN_LW_TX_MASK
            } else {
                self.bits & !CmdModeCfg::GEN_LW_TX_MASK
            },
        }
    }

    /// Set **GEN_SR_2P_TX** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_sr_2p_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CmdModeCfg::GEN_SR_2P_TX_MASK
            } else {
                self.bits & !CmdModeCfg::GEN_SR_2P_TX_MASK
            },
        }
    }

    /// Set **GEN_SR_1P_TX** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_sr_1p_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CmdModeCfg::GEN_SR_1P_TX_MASK
            } else {
                self.bits & !CmdModeCfg::GEN_SR_1P_TX_MASK
            },
        }
    }

    /// Set **GEN_SR_0P_TX** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_sr_0p_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CmdModeCfg::GEN_SR_0P_TX_MASK
            } else {
                self.bits & !CmdModeCfg::GEN_SR_0P_TX_MASK
            },
        }
    }

    /// Set **GEN_SW_2P_TX** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_sw_2p_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CmdModeCfg::GEN_SW_2P_TX_MASK
            } else {
                self.bits & !CmdModeCfg::GEN_SW_2P_TX_MASK
            },
        }
    }

    /// Set **GEN_SW_1P_TX** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_sw_1p_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CmdModeCfg::GEN_SW_1P_TX_MASK
            } else {
                self.bits & !CmdModeCfg::GEN_SW_1P_TX_MASK
            },
        }
    }

    /// Set **GEN_SW_0P_TX** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_sw_0p_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CmdModeCfg::GEN_SW_0P_TX_MASK
            } else {
                self.bits & !CmdModeCfg::GEN_SW_0P_TX_MASK
            },
        }
    }

    /// Set **ACK_RQST_EN** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ack_rqst_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CmdModeCfg::ACK_RQST_EN_MASK
            } else {
                self.bits & !CmdModeCfg::ACK_RQST_EN_MASK
            },
        }
    }

    /// Set **TEAR_FX_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_tear_fx_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CmdModeCfg::TEAR_FX_EN_MASK
            } else {
                self.bits & !CmdModeCfg::TEAR_FX_EN_MASK
            },
        }
    }
}

impl Default for CmdModeCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CmdModeCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CmdModeCfgVal {
        CmdModeCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CmdModeCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CmdModeCfgVal) -> CmdModeCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GEN_HDR Register
///
/// NA
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **GEN_WC_MSBYTE** `[23:16]` (RW)
///
///   NA
///
/// - **GEN_WC_LSBYTE** `[15:8]` (RW)
///
///   NA
///
/// - **GEN_VC** `[7:6]` (RW)
///
///   NA
///
/// - **GEN_DT** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct GenHdr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GenHdr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GenHdr {
    // ======== Field Constants ========

    /// **GEN_WC_MSBYTE**
    ///
    /// NA
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const GEN_WC_MSBYTE_OFFSET: u32 = 16;
    pub const GEN_WC_MSBYTE_MASK: u32 = 0x00ff0000;

    /// **GEN_WC_LSBYTE**
    ///
    /// NA
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const GEN_WC_LSBYTE_OFFSET: u32 = 8;
    pub const GEN_WC_LSBYTE_MASK: u32 = 0x0000ff00;

    /// **GEN_VC**
    ///
    /// NA
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const GEN_VC_OFFSET: u32 = 6;
    pub const GEN_VC_MASK: u32 = 0x000000c0;

    /// **GEN_DT**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const GEN_DT_OFFSET: u32 = 0;
    pub const GEN_DT_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **GEN_WC_MSBYTE** `[23:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_wc_msbyte(&self) -> u32 {
        (self.value.read_bits() & Self::GEN_WC_MSBYTE_MASK) >> Self::GEN_WC_MSBYTE_OFFSET
    }

    /// Write **GEN_WC_MSBYTE** `[23:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_gen_wc_msbyte(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::GEN_WC_MSBYTE_MASK)
                | ((value << Self::GEN_WC_MSBYTE_OFFSET) & Self::GEN_WC_MSBYTE_MASK)
        });
    }

    /// **GEN_WC_LSBYTE** `[15:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_wc_lsbyte(&self) -> u32 {
        (self.value.read_bits() & Self::GEN_WC_LSBYTE_MASK) >> Self::GEN_WC_LSBYTE_OFFSET
    }

    /// Write **GEN_WC_LSBYTE** `[15:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_gen_wc_lsbyte(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::GEN_WC_LSBYTE_MASK)
                | ((value << Self::GEN_WC_LSBYTE_OFFSET) & Self::GEN_WC_LSBYTE_MASK)
        });
    }

    /// **GEN_VC** `[7:6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_vc(&self) -> u32 {
        (self.value.read_bits() & Self::GEN_VC_MASK) >> Self::GEN_VC_OFFSET
    }

    /// Write **GEN_VC** `[7:6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_gen_vc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::GEN_VC_MASK) | ((value << Self::GEN_VC_OFFSET) & Self::GEN_VC_MASK)
        });
    }

    /// **GEN_DT** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_dt(&self) -> u32 {
        (self.value.read_bits() & Self::GEN_DT_MASK) >> Self::GEN_DT_OFFSET
    }

    /// Write **GEN_DT** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_gen_dt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::GEN_DT_MASK) | ((value << Self::GEN_DT_OFFSET) & Self::GEN_DT_MASK)
        });
    }
}

/// GEN_HDR value type for batch read/modify/write operations
///
/// Use `genhdr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `genhdr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GenHdrVal {
    bits: u32,
}

impl GenHdrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **GEN_WC_MSBYTE** `[23:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_wc_msbyte(&self) -> u32 {
        (self.bits & GenHdr::GEN_WC_MSBYTE_MASK) >> GenHdr::GEN_WC_MSBYTE_OFFSET
    }

    /// Read **GEN_WC_LSBYTE** `[15:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_wc_lsbyte(&self) -> u32 {
        (self.bits & GenHdr::GEN_WC_LSBYTE_MASK) >> GenHdr::GEN_WC_LSBYTE_OFFSET
    }

    /// Read **GEN_VC** `[7:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_vc(&self) -> u32 {
        (self.bits & GenHdr::GEN_VC_MASK) >> GenHdr::GEN_VC_OFFSET
    }

    /// Read **GEN_DT** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_dt(&self) -> u32 {
        (self.bits & GenHdr::GEN_DT_MASK) >> GenHdr::GEN_DT_OFFSET
    }

    /// Set **GEN_WC_MSBYTE** `[23:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_wc_msbyte(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GenHdr::GEN_WC_MSBYTE_MASK)
                | ((value << GenHdr::GEN_WC_MSBYTE_OFFSET) & GenHdr::GEN_WC_MSBYTE_MASK),
        }
    }

    /// Set **GEN_WC_LSBYTE** `[15:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_wc_lsbyte(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GenHdr::GEN_WC_LSBYTE_MASK)
                | ((value << GenHdr::GEN_WC_LSBYTE_OFFSET) & GenHdr::GEN_WC_LSBYTE_MASK),
        }
    }

    /// Set **GEN_VC** `[7:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_vc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GenHdr::GEN_VC_MASK)
                | ((value << GenHdr::GEN_VC_OFFSET) & GenHdr::GEN_VC_MASK),
        }
    }

    /// Set **GEN_DT** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_dt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GenHdr::GEN_DT_MASK)
                | ((value << GenHdr::GEN_DT_OFFSET) & GenHdr::GEN_DT_MASK),
        }
    }
}

impl Default for GenHdrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GenHdr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GenHdrVal {
        GenHdrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GenHdrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GenHdrVal) -> GenHdrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GEN_PLD_DATA Register
///
/// NA
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **GEN_PLD_B4** `[31:24]` (RW)
///
///   NA
///
/// - **GEN_PLD_B3** `[23:16]` (RW)
///
///   NA
///
/// - **GEN_PLD_B2** `[15:8]` (RW)
///
///   NA
///
/// - **GEN_PLD_B1** `[7:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct GenPldData {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GenPldData {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GenPldData {
    // ======== Field Constants ========

    /// **GEN_PLD_B4**
    ///
    /// NA
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const GEN_PLD_B4_OFFSET: u32 = 24;
    pub const GEN_PLD_B4_MASK: u32 = 0xff000000;

    /// **GEN_PLD_B3**
    ///
    /// NA
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const GEN_PLD_B3_OFFSET: u32 = 16;
    pub const GEN_PLD_B3_MASK: u32 = 0x00ff0000;

    /// **GEN_PLD_B2**
    ///
    /// NA
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const GEN_PLD_B2_OFFSET: u32 = 8;
    pub const GEN_PLD_B2_MASK: u32 = 0x0000ff00;

    /// **GEN_PLD_B1**
    ///
    /// NA
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const GEN_PLD_B1_OFFSET: u32 = 0;
    pub const GEN_PLD_B1_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **GEN_PLD_B4** `[31:24]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_pld_b4(&self) -> u32 {
        (self.value.read_bits() & Self::GEN_PLD_B4_MASK) >> Self::GEN_PLD_B4_OFFSET
    }

    /// Write **GEN_PLD_B4** `[31:24]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_gen_pld_b4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::GEN_PLD_B4_MASK)
                | ((value << Self::GEN_PLD_B4_OFFSET) & Self::GEN_PLD_B4_MASK)
        });
    }

    /// **GEN_PLD_B3** `[23:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_pld_b3(&self) -> u32 {
        (self.value.read_bits() & Self::GEN_PLD_B3_MASK) >> Self::GEN_PLD_B3_OFFSET
    }

    /// Write **GEN_PLD_B3** `[23:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_gen_pld_b3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::GEN_PLD_B3_MASK)
                | ((value << Self::GEN_PLD_B3_OFFSET) & Self::GEN_PLD_B3_MASK)
        });
    }

    /// **GEN_PLD_B2** `[15:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_pld_b2(&self) -> u32 {
        (self.value.read_bits() & Self::GEN_PLD_B2_MASK) >> Self::GEN_PLD_B2_OFFSET
    }

    /// Write **GEN_PLD_B2** `[15:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_gen_pld_b2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::GEN_PLD_B2_MASK)
                | ((value << Self::GEN_PLD_B2_OFFSET) & Self::GEN_PLD_B2_MASK)
        });
    }

    /// **GEN_PLD_B1** `[7:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_pld_b1(&self) -> u32 {
        (self.value.read_bits() & Self::GEN_PLD_B1_MASK) >> Self::GEN_PLD_B1_OFFSET
    }

    /// Write **GEN_PLD_B1** `[7:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_gen_pld_b1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::GEN_PLD_B1_MASK)
                | ((value << Self::GEN_PLD_B1_OFFSET) & Self::GEN_PLD_B1_MASK)
        });
    }
}

/// GEN_PLD_DATA value type for batch read/modify/write operations
///
/// Use `genplddata.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `genplddata.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GenPldDataVal {
    bits: u32,
}

impl GenPldDataVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **GEN_PLD_B4** `[31:24]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_pld_b4(&self) -> u32 {
        (self.bits & GenPldData::GEN_PLD_B4_MASK) >> GenPldData::GEN_PLD_B4_OFFSET
    }

    /// Read **GEN_PLD_B3** `[23:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_pld_b3(&self) -> u32 {
        (self.bits & GenPldData::GEN_PLD_B3_MASK) >> GenPldData::GEN_PLD_B3_OFFSET
    }

    /// Read **GEN_PLD_B2** `[15:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_pld_b2(&self) -> u32 {
        (self.bits & GenPldData::GEN_PLD_B2_MASK) >> GenPldData::GEN_PLD_B2_OFFSET
    }

    /// Read **GEN_PLD_B1** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_pld_b1(&self) -> u32 {
        (self.bits & GenPldData::GEN_PLD_B1_MASK) >> GenPldData::GEN_PLD_B1_OFFSET
    }

    /// Set **GEN_PLD_B4** `[31:24]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_pld_b4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GenPldData::GEN_PLD_B4_MASK)
                | ((value << GenPldData::GEN_PLD_B4_OFFSET) & GenPldData::GEN_PLD_B4_MASK),
        }
    }

    /// Set **GEN_PLD_B3** `[23:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_pld_b3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GenPldData::GEN_PLD_B3_MASK)
                | ((value << GenPldData::GEN_PLD_B3_OFFSET) & GenPldData::GEN_PLD_B3_MASK),
        }
    }

    /// Set **GEN_PLD_B2** `[15:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_pld_b2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GenPldData::GEN_PLD_B2_MASK)
                | ((value << GenPldData::GEN_PLD_B2_OFFSET) & GenPldData::GEN_PLD_B2_MASK),
        }
    }

    /// Set **GEN_PLD_B1** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_gen_pld_b1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GenPldData::GEN_PLD_B1_MASK)
                | ((value << GenPldData::GEN_PLD_B1_OFFSET) & GenPldData::GEN_PLD_B1_MASK),
        }
    }
}

impl Default for GenPldDataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GenPldData {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GenPldDataVal {
        GenPldDataVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GenPldDataVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GenPldDataVal) -> GenPldDataVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CMD_PKT_STATUS Register
///
/// NA
///
/// Address offset: 0x0074
/// Reset value: 0x00050015
///
/// ## Fields
///
/// - **GEN_BUFF_PLD_FULL** `[19]` (RO)
///
///   NA
///
/// - **GEN_BUFF_PLD_EMPTY** `[18]` (RO)
///
///   NA
///
/// - **GEN_BUFF_CMD_FULL** `[17]` (RO)
///
///   NA
///
/// - **GEN_BUFF_CMD_EMPTY** `[16]` (RO)
///
///   NA
///
/// - **GEN_RD_CMD_BUSY** `[6]` (RO)
///
///   NA
///
/// - **GEN_PLD_R_FULL** `[5]` (RO)
///
///   NA
///
/// - **GEN_PLD_R_EMPTY** `[4]` (RO)
///
///   NA
///
/// - **GEN_PLD_W_FULL** `[3]` (RO)
///
///   NA
///
/// - **GEN_PLD_W_EMPTY** `[2]` (RO)
///
///   NA
///
/// - **GEN_CMD_FULL** `[1]` (RO)
///
///   NA
///
/// - **GEN_CMD_EMPTY** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct CmdPktStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CmdPktStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CmdPktStatus {
    // ======== Field Constants ========

    /// **GEN_BUFF_PLD_FULL**
    ///
    /// NA
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const GEN_BUFF_PLD_FULL_OFFSET: u32 = 19;
    pub const GEN_BUFF_PLD_FULL_MASK: u32 = 0x00080000;
    pub const GEN_BUFF_PLD_FULL: u32 = 1 << 19;

    /// **GEN_BUFF_PLD_EMPTY**
    ///
    /// NA
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const GEN_BUFF_PLD_EMPTY_OFFSET: u32 = 18;
    pub const GEN_BUFF_PLD_EMPTY_MASK: u32 = 0x00040000;
    pub const GEN_BUFF_PLD_EMPTY: u32 = 1 << 18;

    /// **GEN_BUFF_CMD_FULL**
    ///
    /// NA
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const GEN_BUFF_CMD_FULL_OFFSET: u32 = 17;
    pub const GEN_BUFF_CMD_FULL_MASK: u32 = 0x00020000;
    pub const GEN_BUFF_CMD_FULL: u32 = 1 << 17;

    /// **GEN_BUFF_CMD_EMPTY**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const GEN_BUFF_CMD_EMPTY_OFFSET: u32 = 16;
    pub const GEN_BUFF_CMD_EMPTY_MASK: u32 = 0x00010000;
    pub const GEN_BUFF_CMD_EMPTY: u32 = 1 << 16;

    /// **GEN_RD_CMD_BUSY**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const GEN_RD_CMD_BUSY_OFFSET: u32 = 6;
    pub const GEN_RD_CMD_BUSY_MASK: u32 = 0x00000040;
    pub const GEN_RD_CMD_BUSY: u32 = 1 << 6;

    /// **GEN_PLD_R_FULL**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const GEN_PLD_R_FULL_OFFSET: u32 = 5;
    pub const GEN_PLD_R_FULL_MASK: u32 = 0x00000020;
    pub const GEN_PLD_R_FULL: u32 = 1 << 5;

    /// **GEN_PLD_R_EMPTY**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const GEN_PLD_R_EMPTY_OFFSET: u32 = 4;
    pub const GEN_PLD_R_EMPTY_MASK: u32 = 0x00000010;
    pub const GEN_PLD_R_EMPTY: u32 = 1 << 4;

    /// **GEN_PLD_W_FULL**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const GEN_PLD_W_FULL_OFFSET: u32 = 3;
    pub const GEN_PLD_W_FULL_MASK: u32 = 0x00000008;
    pub const GEN_PLD_W_FULL: u32 = 1 << 3;

    /// **GEN_PLD_W_EMPTY**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const GEN_PLD_W_EMPTY_OFFSET: u32 = 2;
    pub const GEN_PLD_W_EMPTY_MASK: u32 = 0x00000004;
    pub const GEN_PLD_W_EMPTY: u32 = 1 << 2;

    /// **GEN_CMD_FULL**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const GEN_CMD_FULL_OFFSET: u32 = 1;
    pub const GEN_CMD_FULL_MASK: u32 = 0x00000002;
    pub const GEN_CMD_FULL: u32 = 1 << 1;

    /// **GEN_CMD_EMPTY**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const GEN_CMD_EMPTY_OFFSET: u32 = 0;
    pub const GEN_CMD_EMPTY_MASK: u32 = 0x00000001;
    pub const GEN_CMD_EMPTY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **GEN_BUFF_PLD_FULL** `[19]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_buff_pld_full(&self) -> bool {
        (self.value.read_bits() & Self::GEN_BUFF_PLD_FULL_MASK) != 0
    }

    /// **GEN_BUFF_PLD_EMPTY** `[18]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_buff_pld_empty(&self) -> bool {
        (self.value.read_bits() & Self::GEN_BUFF_PLD_EMPTY_MASK) != 0
    }

    /// **GEN_BUFF_CMD_FULL** `[17]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_buff_cmd_full(&self) -> bool {
        (self.value.read_bits() & Self::GEN_BUFF_CMD_FULL_MASK) != 0
    }

    /// **GEN_BUFF_CMD_EMPTY** `[16]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_buff_cmd_empty(&self) -> bool {
        (self.value.read_bits() & Self::GEN_BUFF_CMD_EMPTY_MASK) != 0
    }

    /// **GEN_RD_CMD_BUSY** `[6]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_rd_cmd_busy(&self) -> bool {
        (self.value.read_bits() & Self::GEN_RD_CMD_BUSY_MASK) != 0
    }

    /// **GEN_PLD_R_FULL** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_pld_r_full(&self) -> bool {
        (self.value.read_bits() & Self::GEN_PLD_R_FULL_MASK) != 0
    }

    /// **GEN_PLD_R_EMPTY** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_pld_r_empty(&self) -> bool {
        (self.value.read_bits() & Self::GEN_PLD_R_EMPTY_MASK) != 0
    }

    /// **GEN_PLD_W_FULL** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_pld_w_full(&self) -> bool {
        (self.value.read_bits() & Self::GEN_PLD_W_FULL_MASK) != 0
    }

    /// **GEN_PLD_W_EMPTY** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_pld_w_empty(&self) -> bool {
        (self.value.read_bits() & Self::GEN_PLD_W_EMPTY_MASK) != 0
    }

    /// **GEN_CMD_FULL** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_cmd_full(&self) -> bool {
        (self.value.read_bits() & Self::GEN_CMD_FULL_MASK) != 0
    }

    /// **GEN_CMD_EMPTY** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_cmd_empty(&self) -> bool {
        (self.value.read_bits() & Self::GEN_CMD_EMPTY_MASK) != 0
    }
}

/// CMD_PKT_STATUS value type for batch read/modify/write operations
///
/// Use `cmdpktstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cmdpktstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CmdPktStatusVal {
    bits: u32,
}

impl CmdPktStatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00050015 }
    }

    /// Read **GEN_BUFF_PLD_FULL** `[19:19]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_buff_pld_full(&self) -> bool {
        (self.bits & CmdPktStatus::GEN_BUFF_PLD_FULL_MASK) != 0
    }

    /// Read **GEN_BUFF_PLD_EMPTY** `[18:18]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_buff_pld_empty(&self) -> bool {
        (self.bits & CmdPktStatus::GEN_BUFF_PLD_EMPTY_MASK) != 0
    }

    /// Read **GEN_BUFF_CMD_FULL** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_buff_cmd_full(&self) -> bool {
        (self.bits & CmdPktStatus::GEN_BUFF_CMD_FULL_MASK) != 0
    }

    /// Read **GEN_BUFF_CMD_EMPTY** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_buff_cmd_empty(&self) -> bool {
        (self.bits & CmdPktStatus::GEN_BUFF_CMD_EMPTY_MASK) != 0
    }

    /// Read **GEN_RD_CMD_BUSY** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_rd_cmd_busy(&self) -> bool {
        (self.bits & CmdPktStatus::GEN_RD_CMD_BUSY_MASK) != 0
    }

    /// Read **GEN_PLD_R_FULL** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_pld_r_full(&self) -> bool {
        (self.bits & CmdPktStatus::GEN_PLD_R_FULL_MASK) != 0
    }

    /// Read **GEN_PLD_R_EMPTY** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_pld_r_empty(&self) -> bool {
        (self.bits & CmdPktStatus::GEN_PLD_R_EMPTY_MASK) != 0
    }

    /// Read **GEN_PLD_W_FULL** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_pld_w_full(&self) -> bool {
        (self.bits & CmdPktStatus::GEN_PLD_W_FULL_MASK) != 0
    }

    /// Read **GEN_PLD_W_EMPTY** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_pld_w_empty(&self) -> bool {
        (self.bits & CmdPktStatus::GEN_PLD_W_EMPTY_MASK) != 0
    }

    /// Read **GEN_CMD_FULL** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_cmd_full(&self) -> bool {
        (self.bits & CmdPktStatus::GEN_CMD_FULL_MASK) != 0
    }

    /// Read **GEN_CMD_EMPTY** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_cmd_empty(&self) -> bool {
        (self.bits & CmdPktStatus::GEN_CMD_EMPTY_MASK) != 0
    }
}

impl Default for CmdPktStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CmdPktStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CmdPktStatusVal {
        CmdPktStatusVal::from_bits(self.value.read_bits())
    }
}

/// TO_CNT_CFG Register
///
/// NA
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **HSTX_TO_CNT** `[31:16]` (RW)
///
///   NA
///
/// - **LPRX_TO_CNT** `[15:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct ToCntCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ToCntCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ToCntCfg {
    // ======== Field Constants ========

    /// **HSTX_TO_CNT**
    ///
    /// NA
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const HSTX_TO_CNT_OFFSET: u32 = 16;
    pub const HSTX_TO_CNT_MASK: u32 = 0xffff0000;

    /// **LPRX_TO_CNT**
    ///
    /// NA
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const LPRX_TO_CNT_OFFSET: u32 = 0;
    pub const LPRX_TO_CNT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **HSTX_TO_CNT** `[31:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn hstx_to_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::HSTX_TO_CNT_MASK) >> Self::HSTX_TO_CNT_OFFSET
    }

    /// Write **HSTX_TO_CNT** `[31:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_hstx_to_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HSTX_TO_CNT_MASK)
                | ((value << Self::HSTX_TO_CNT_OFFSET) & Self::HSTX_TO_CNT_MASK)
        });
    }

    /// **LPRX_TO_CNT** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn lprx_to_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::LPRX_TO_CNT_MASK) >> Self::LPRX_TO_CNT_OFFSET
    }

    /// Write **LPRX_TO_CNT** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_lprx_to_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LPRX_TO_CNT_MASK)
                | ((value << Self::LPRX_TO_CNT_OFFSET) & Self::LPRX_TO_CNT_MASK)
        });
    }
}

/// TO_CNT_CFG value type for batch read/modify/write operations
///
/// Use `tocntcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tocntcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ToCntCfgVal {
    bits: u32,
}

impl ToCntCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HSTX_TO_CNT** `[31:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn hstx_to_cnt(&self) -> u32 {
        (self.bits & ToCntCfg::HSTX_TO_CNT_MASK) >> ToCntCfg::HSTX_TO_CNT_OFFSET
    }

    /// Read **LPRX_TO_CNT** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lprx_to_cnt(&self) -> u32 {
        (self.bits & ToCntCfg::LPRX_TO_CNT_MASK) >> ToCntCfg::LPRX_TO_CNT_OFFSET
    }

    /// Set **HSTX_TO_CNT** `[31:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_hstx_to_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ToCntCfg::HSTX_TO_CNT_MASK)
                | ((value << ToCntCfg::HSTX_TO_CNT_OFFSET) & ToCntCfg::HSTX_TO_CNT_MASK),
        }
    }

    /// Set **LPRX_TO_CNT** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_lprx_to_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ToCntCfg::LPRX_TO_CNT_MASK)
                | ((value << ToCntCfg::LPRX_TO_CNT_OFFSET) & ToCntCfg::LPRX_TO_CNT_MASK),
        }
    }
}

impl Default for ToCntCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ToCntCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ToCntCfgVal {
        ToCntCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ToCntCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ToCntCfgVal) -> ToCntCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HS_RD_TO_CNT Register
///
/// NA
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **HS_RD_TO_CNT** `[15:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct HsRdToCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HsRdToCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HsRdToCnt {
    // ======== Field Constants ========

    /// **HS_RD_TO_CNT**
    ///
    /// NA
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const HS_RD_TO_CNT_OFFSET: u32 = 0;
    pub const HS_RD_TO_CNT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **HS_RD_TO_CNT** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn hs_rd_to_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::HS_RD_TO_CNT_MASK) >> Self::HS_RD_TO_CNT_OFFSET
    }

    /// Write **HS_RD_TO_CNT** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_hs_rd_to_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HS_RD_TO_CNT_MASK)
                | ((value << Self::HS_RD_TO_CNT_OFFSET) & Self::HS_RD_TO_CNT_MASK)
        });
    }
}

/// HS_RD_TO_CNT value type for batch read/modify/write operations
///
/// Use `hsrdtocnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hsrdtocnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HsRdToCntVal {
    bits: u32,
}

impl HsRdToCntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HS_RD_TO_CNT** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn hs_rd_to_cnt(&self) -> u32 {
        (self.bits & HsRdToCnt::HS_RD_TO_CNT_MASK) >> HsRdToCnt::HS_RD_TO_CNT_OFFSET
    }

    /// Set **HS_RD_TO_CNT** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_hs_rd_to_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HsRdToCnt::HS_RD_TO_CNT_MASK)
                | ((value << HsRdToCnt::HS_RD_TO_CNT_OFFSET) & HsRdToCnt::HS_RD_TO_CNT_MASK),
        }
    }
}

impl Default for HsRdToCntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl HsRdToCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HsRdToCntVal {
        HsRdToCntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HsRdToCntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HsRdToCntVal) -> HsRdToCntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_RD_TO_CNT Register
///
/// NA
///
/// Address offset: 0x0080
///
/// ## Fields
///
/// - **LP_RD_TO_CNT** `[15:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpRdToCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpRdToCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpRdToCnt {
    // ======== Field Constants ========

    /// **LP_RD_TO_CNT**
    ///
    /// NA
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const LP_RD_TO_CNT_OFFSET: u32 = 0;
    pub const LP_RD_TO_CNT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **LP_RD_TO_CNT** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_rd_to_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::LP_RD_TO_CNT_MASK) >> Self::LP_RD_TO_CNT_OFFSET
    }

    /// Write **LP_RD_TO_CNT** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_lp_rd_to_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_RD_TO_CNT_MASK)
                | ((value << Self::LP_RD_TO_CNT_OFFSET) & Self::LP_RD_TO_CNT_MASK)
        });
    }
}

/// LP_RD_TO_CNT value type for batch read/modify/write operations
///
/// Use `lprdtocnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lprdtocnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpRdToCntVal {
    bits: u32,
}

impl LpRdToCntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_RD_TO_CNT** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_rd_to_cnt(&self) -> u32 {
        (self.bits & LpRdToCnt::LP_RD_TO_CNT_MASK) >> LpRdToCnt::LP_RD_TO_CNT_OFFSET
    }

    /// Set **LP_RD_TO_CNT** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_lp_rd_to_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpRdToCnt::LP_RD_TO_CNT_MASK)
                | ((value << LpRdToCnt::LP_RD_TO_CNT_OFFSET) & LpRdToCnt::LP_RD_TO_CNT_MASK),
        }
    }
}

impl Default for LpRdToCntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpRdToCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpRdToCntVal {
        LpRdToCntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpRdToCntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpRdToCntVal) -> LpRdToCntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HS_WR_TO_CNT Register
///
/// NA
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **HS_WR_TO_CNT** `[15:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct HsWrToCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HsWrToCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HsWrToCnt {
    // ======== Field Constants ========

    /// **HS_WR_TO_CNT**
    ///
    /// NA
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const HS_WR_TO_CNT_OFFSET: u32 = 0;
    pub const HS_WR_TO_CNT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **HS_WR_TO_CNT** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn hs_wr_to_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::HS_WR_TO_CNT_MASK) >> Self::HS_WR_TO_CNT_OFFSET
    }

    /// Write **HS_WR_TO_CNT** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_hs_wr_to_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HS_WR_TO_CNT_MASK)
                | ((value << Self::HS_WR_TO_CNT_OFFSET) & Self::HS_WR_TO_CNT_MASK)
        });
    }
}

/// HS_WR_TO_CNT value type for batch read/modify/write operations
///
/// Use `hswrtocnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hswrtocnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HsWrToCntVal {
    bits: u32,
}

impl HsWrToCntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HS_WR_TO_CNT** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn hs_wr_to_cnt(&self) -> u32 {
        (self.bits & HsWrToCnt::HS_WR_TO_CNT_MASK) >> HsWrToCnt::HS_WR_TO_CNT_OFFSET
    }

    /// Set **HS_WR_TO_CNT** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_hs_wr_to_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HsWrToCnt::HS_WR_TO_CNT_MASK)
                | ((value << HsWrToCnt::HS_WR_TO_CNT_OFFSET) & HsWrToCnt::HS_WR_TO_CNT_MASK),
        }
    }
}

impl Default for HsWrToCntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl HsWrToCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HsWrToCntVal {
        HsWrToCntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HsWrToCntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HsWrToCntVal) -> HsWrToCntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_WR_TO_CNT Register
///
/// NA
///
/// Address offset: 0x0088
///
/// ## Fields
///
/// - **LP_WR_TO_CNT** `[15:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpWrToCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpWrToCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpWrToCnt {
    // ======== Field Constants ========

    /// **LP_WR_TO_CNT**
    ///
    /// NA
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const LP_WR_TO_CNT_OFFSET: u32 = 0;
    pub const LP_WR_TO_CNT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **LP_WR_TO_CNT** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_wr_to_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::LP_WR_TO_CNT_MASK) >> Self::LP_WR_TO_CNT_OFFSET
    }

    /// Write **LP_WR_TO_CNT** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_lp_wr_to_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_WR_TO_CNT_MASK)
                | ((value << Self::LP_WR_TO_CNT_OFFSET) & Self::LP_WR_TO_CNT_MASK)
        });
    }
}

/// LP_WR_TO_CNT value type for batch read/modify/write operations
///
/// Use `lpwrtocnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpwrtocnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpWrToCntVal {
    bits: u32,
}

impl LpWrToCntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_WR_TO_CNT** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_wr_to_cnt(&self) -> u32 {
        (self.bits & LpWrToCnt::LP_WR_TO_CNT_MASK) >> LpWrToCnt::LP_WR_TO_CNT_OFFSET
    }

    /// Set **LP_WR_TO_CNT** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_lp_wr_to_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpWrToCnt::LP_WR_TO_CNT_MASK)
                | ((value << LpWrToCnt::LP_WR_TO_CNT_OFFSET) & LpWrToCnt::LP_WR_TO_CNT_MASK),
        }
    }
}

impl Default for LpWrToCntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpWrToCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpWrToCntVal {
        LpWrToCntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpWrToCntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpWrToCntVal) -> LpWrToCntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BTA_TO_CNT Register
///
/// NA
///
/// Address offset: 0x008c
///
/// ## Fields
///
/// - **BTA_TO_CNT** `[15:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct BtaToCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BtaToCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BtaToCnt {
    // ======== Field Constants ========

    /// **BTA_TO_CNT**
    ///
    /// NA
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const BTA_TO_CNT_OFFSET: u32 = 0;
    pub const BTA_TO_CNT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **BTA_TO_CNT** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn bta_to_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::BTA_TO_CNT_MASK) >> Self::BTA_TO_CNT_OFFSET
    }

    /// Write **BTA_TO_CNT** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_bta_to_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BTA_TO_CNT_MASK)
                | ((value << Self::BTA_TO_CNT_OFFSET) & Self::BTA_TO_CNT_MASK)
        });
    }
}

/// BTA_TO_CNT value type for batch read/modify/write operations
///
/// Use `btatocnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `btatocnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BtaToCntVal {
    bits: u32,
}

impl BtaToCntVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BTA_TO_CNT** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn bta_to_cnt(&self) -> u32 {
        (self.bits & BtaToCnt::BTA_TO_CNT_MASK) >> BtaToCnt::BTA_TO_CNT_OFFSET
    }

    /// Set **BTA_TO_CNT** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_bta_to_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BtaToCnt::BTA_TO_CNT_MASK)
                | ((value << BtaToCnt::BTA_TO_CNT_OFFSET) & BtaToCnt::BTA_TO_CNT_MASK),
        }
    }
}

impl Default for BtaToCntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BtaToCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BtaToCntVal {
        BtaToCntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BtaToCntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BtaToCntVal) -> BtaToCntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SDF_3D Register
///
/// NA
///
/// Address offset: 0x0090
///
/// ## Fields
///
/// - **SEND_3D_CFG** `[16]` (RW)
///
///   NA
///
/// - **RIGHT_FIRST** `[5]` (RW)
///
///   NA
///
/// - **SECOND_VSYNC** `[4]` (RW)
///
///   NA
///
/// - **FORMAT_3D** `[3:2]` (RW)
///
///   NA
///
/// - **MODE_3D** `[1:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Sdf3d {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sdf3d {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sdf3d {
    // ======== Field Constants ========

    /// **SEND_3D_CFG**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SEND_3D_CFG_OFFSET: u32 = 16;
    pub const SEND_3D_CFG_MASK: u32 = 0x00010000;
    pub const SEND_3D_CFG: u32 = 1 << 16;

    /// **RIGHT_FIRST**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RIGHT_FIRST_OFFSET: u32 = 5;
    pub const RIGHT_FIRST_MASK: u32 = 0x00000020;
    pub const RIGHT_FIRST: u32 = 1 << 5;

    /// **SECOND_VSYNC**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SECOND_VSYNC_OFFSET: u32 = 4;
    pub const SECOND_VSYNC_MASK: u32 = 0x00000010;
    pub const SECOND_VSYNC: u32 = 1 << 4;

    /// **FORMAT_3D**
    ///
    /// NA
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const FORMAT_3D_OFFSET: u32 = 2;
    pub const FORMAT_3D_MASK: u32 = 0x0000000c;

    /// **MODE_3D**
    ///
    /// NA
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MODE_3D_OFFSET: u32 = 0;
    pub const MODE_3D_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SEND_3D_CFG** `[16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn send_3d_cfg(&self) -> bool {
        (self.value.read_bits() & Self::SEND_3D_CFG_MASK) != 0
    }

    /// Set SEND_3D_CFG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_send_3d_cfg(&self) {
        self.value.modify_bits(|r| r | Self::SEND_3D_CFG_MASK);
    }

    /// Clear SEND_3D_CFG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_send_3d_cfg(&self) {
        self.value.modify_bits(|r| r & !Self::SEND_3D_CFG_MASK);
    }

    /// **RIGHT_FIRST** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn right_first(&self) -> bool {
        (self.value.read_bits() & Self::RIGHT_FIRST_MASK) != 0
    }

    /// Set RIGHT_FIRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_right_first(&self) {
        self.value.modify_bits(|r| r | Self::RIGHT_FIRST_MASK);
    }

    /// Clear RIGHT_FIRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_right_first(&self) {
        self.value.modify_bits(|r| r & !Self::RIGHT_FIRST_MASK);
    }

    /// **SECOND_VSYNC** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn second_vsync(&self) -> bool {
        (self.value.read_bits() & Self::SECOND_VSYNC_MASK) != 0
    }

    /// Set SECOND_VSYNC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_second_vsync(&self) {
        self.value.modify_bits(|r| r | Self::SECOND_VSYNC_MASK);
    }

    /// Clear SECOND_VSYNC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_second_vsync(&self) {
        self.value.modify_bits(|r| r & !Self::SECOND_VSYNC_MASK);
    }

    /// **FORMAT_3D** `[3:2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn format_3d(&self) -> u32 {
        (self.value.read_bits() & Self::FORMAT_3D_MASK) >> Self::FORMAT_3D_OFFSET
    }

    /// Write **FORMAT_3D** `[3:2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_format_3d(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FORMAT_3D_MASK) | ((value << Self::FORMAT_3D_OFFSET) & Self::FORMAT_3D_MASK)
        });
    }

    /// **MODE_3D** `[1:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mode_3d(&self) -> u32 {
        (self.value.read_bits() & Self::MODE_3D_MASK) >> Self::MODE_3D_OFFSET
    }

    /// Write **MODE_3D** `[1:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_mode_3d(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE_3D_MASK) | ((value << Self::MODE_3D_OFFSET) & Self::MODE_3D_MASK)
        });
    }
}

/// SDF_3D value type for batch read/modify/write operations
///
/// Use `sdf3d.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sdf3d.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sdf3dVal {
    bits: u32,
}

impl Sdf3dVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_3D_CFG** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn send_3d_cfg(&self) -> bool {
        (self.bits & Sdf3d::SEND_3D_CFG_MASK) != 0
    }

    /// Read **RIGHT_FIRST** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn right_first(&self) -> bool {
        (self.bits & Sdf3d::RIGHT_FIRST_MASK) != 0
    }

    /// Read **SECOND_VSYNC** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn second_vsync(&self) -> bool {
        (self.bits & Sdf3d::SECOND_VSYNC_MASK) != 0
    }

    /// Read **FORMAT_3D** `[3:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn format_3d(&self) -> u32 {
        (self.bits & Sdf3d::FORMAT_3D_MASK) >> Sdf3d::FORMAT_3D_OFFSET
    }

    /// Read **MODE_3D** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mode_3d(&self) -> u32 {
        (self.bits & Sdf3d::MODE_3D_MASK) >> Sdf3d::MODE_3D_OFFSET
    }

    /// Set **SEND_3D_CFG** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_send_3d_cfg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sdf3d::SEND_3D_CFG_MASK
            } else {
                self.bits & !Sdf3d::SEND_3D_CFG_MASK
            },
        }
    }

    /// Set **RIGHT_FIRST** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_right_first(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sdf3d::RIGHT_FIRST_MASK
            } else {
                self.bits & !Sdf3d::RIGHT_FIRST_MASK
            },
        }
    }

    /// Set **SECOND_VSYNC** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_second_vsync(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sdf3d::SECOND_VSYNC_MASK
            } else {
                self.bits & !Sdf3d::SECOND_VSYNC_MASK
            },
        }
    }

    /// Set **FORMAT_3D** `[3:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_format_3d(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sdf3d::FORMAT_3D_MASK)
                | ((value << Sdf3d::FORMAT_3D_OFFSET) & Sdf3d::FORMAT_3D_MASK),
        }
    }

    /// Set **MODE_3D** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mode_3d(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sdf3d::MODE_3D_MASK)
                | ((value << Sdf3d::MODE_3D_OFFSET) & Sdf3d::MODE_3D_MASK),
        }
    }
}

impl Default for Sdf3dVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sdf3d {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sdf3dVal {
        Sdf3dVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sdf3dVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sdf3dVal) -> Sdf3dVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LPCLK_CTRL Register
///
/// NA
///
/// Address offset: 0x0094
///
/// ## Fields
///
/// - **AUTO_CLKLANE_CTRL** `[1]` (RW)
///
///   NA
///
/// - **PHY_TXREQUESTCLKHS** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpclkCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpclkCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpclkCtrl {
    // ======== Field Constants ========

    /// **AUTO_CLKLANE_CTRL**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const AUTO_CLKLANE_CTRL_OFFSET: u32 = 1;
    pub const AUTO_CLKLANE_CTRL_MASK: u32 = 0x00000002;
    pub const AUTO_CLKLANE_CTRL: u32 = 1 << 1;

    /// **PHY_TXREQUESTCLKHS**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PHY_TXREQUESTCLKHS_OFFSET: u32 = 0;
    pub const PHY_TXREQUESTCLKHS_MASK: u32 = 0x00000001;
    pub const PHY_TXREQUESTCLKHS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **AUTO_CLKLANE_CTRL** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn auto_clklane_ctrl(&self) -> bool {
        (self.value.read_bits() & Self::AUTO_CLKLANE_CTRL_MASK) != 0
    }

    /// Set AUTO_CLKLANE_CTRL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_auto_clklane_ctrl(&self) {
        self.value.modify_bits(|r| r | Self::AUTO_CLKLANE_CTRL_MASK);
    }

    /// Clear AUTO_CLKLANE_CTRL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_auto_clklane_ctrl(&self) {
        self.value
            .modify_bits(|r| r & !Self::AUTO_CLKLANE_CTRL_MASK);
    }

    /// **PHY_TXREQUESTCLKHS** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_txrequestclkhs(&self) -> bool {
        (self.value.read_bits() & Self::PHY_TXREQUESTCLKHS_MASK) != 0
    }

    /// Set PHY_TXREQUESTCLKHS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_txrequestclkhs(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_TXREQUESTCLKHS_MASK);
    }

    /// Clear PHY_TXREQUESTCLKHS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_txrequestclkhs(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_TXREQUESTCLKHS_MASK);
    }
}

/// LPCLK_CTRL value type for batch read/modify/write operations
///
/// Use `lpclkctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpclkctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpclkCtrlVal {
    bits: u32,
}

impl LpclkCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **AUTO_CLKLANE_CTRL** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn auto_clklane_ctrl(&self) -> bool {
        (self.bits & LpclkCtrl::AUTO_CLKLANE_CTRL_MASK) != 0
    }

    /// Read **PHY_TXREQUESTCLKHS** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_txrequestclkhs(&self) -> bool {
        (self.bits & LpclkCtrl::PHY_TXREQUESTCLKHS_MASK) != 0
    }

    /// Set **AUTO_CLKLANE_CTRL** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_auto_clklane_ctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpclkCtrl::AUTO_CLKLANE_CTRL_MASK
            } else {
                self.bits & !LpclkCtrl::AUTO_CLKLANE_CTRL_MASK
            },
        }
    }

    /// Set **PHY_TXREQUESTCLKHS** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_txrequestclkhs(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpclkCtrl::PHY_TXREQUESTCLKHS_MASK
            } else {
                self.bits & !LpclkCtrl::PHY_TXREQUESTCLKHS_MASK
            },
        }
    }
}

impl Default for LpclkCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpclkCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpclkCtrlVal {
        LpclkCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpclkCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpclkCtrlVal) -> LpclkCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_TMR_LPCLK_CFG Register
///
/// NA
///
/// Address offset: 0x0098
///
/// ## Fields
///
/// - **PHY_CLKHS2LP_TIME** `[25:16]` (RW)
///
///   NA
///
/// - **PHY_CLKLP2HS_TIME** `[9:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PhyTmrLpclkCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyTmrLpclkCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyTmrLpclkCfg {
    // ======== Field Constants ========

    /// **PHY_CLKHS2LP_TIME**
    ///
    /// NA
    ///
    /// Bits `[25:16]`, mask `0x03ff0000`
    pub const PHY_CLKHS2LP_TIME_OFFSET: u32 = 16;
    pub const PHY_CLKHS2LP_TIME_MASK: u32 = 0x03ff0000;

    /// **PHY_CLKLP2HS_TIME**
    ///
    /// NA
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const PHY_CLKLP2HS_TIME_OFFSET: u32 = 0;
    pub const PHY_CLKLP2HS_TIME_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **PHY_CLKHS2LP_TIME** `[25:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_clkhs2lp_time(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_CLKHS2LP_TIME_MASK) >> Self::PHY_CLKHS2LP_TIME_OFFSET
    }

    /// Write **PHY_CLKHS2LP_TIME** `[25:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_phy_clkhs2lp_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PHY_CLKHS2LP_TIME_MASK)
                | ((value << Self::PHY_CLKHS2LP_TIME_OFFSET) & Self::PHY_CLKHS2LP_TIME_MASK)
        });
    }

    /// **PHY_CLKLP2HS_TIME** `[9:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_clklp2hs_time(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_CLKLP2HS_TIME_MASK) >> Self::PHY_CLKLP2HS_TIME_OFFSET
    }

    /// Write **PHY_CLKLP2HS_TIME** `[9:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_phy_clklp2hs_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PHY_CLKLP2HS_TIME_MASK)
                | ((value << Self::PHY_CLKLP2HS_TIME_OFFSET) & Self::PHY_CLKLP2HS_TIME_MASK)
        });
    }
}

/// PHY_TMR_LPCLK_CFG value type for batch read/modify/write operations
///
/// Use `phytmrlpclkcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phytmrlpclkcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyTmrLpclkCfgVal {
    bits: u32,
}

impl PhyTmrLpclkCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PHY_CLKHS2LP_TIME** `[25:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_clkhs2lp_time(&self) -> u32 {
        (self.bits & PhyTmrLpclkCfg::PHY_CLKHS2LP_TIME_MASK)
            >> PhyTmrLpclkCfg::PHY_CLKHS2LP_TIME_OFFSET
    }

    /// Read **PHY_CLKLP2HS_TIME** `[9:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_clklp2hs_time(&self) -> u32 {
        (self.bits & PhyTmrLpclkCfg::PHY_CLKLP2HS_TIME_MASK)
            >> PhyTmrLpclkCfg::PHY_CLKLP2HS_TIME_OFFSET
    }

    /// Set **PHY_CLKHS2LP_TIME** `[25:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_clkhs2lp_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyTmrLpclkCfg::PHY_CLKHS2LP_TIME_MASK)
                | ((value << PhyTmrLpclkCfg::PHY_CLKHS2LP_TIME_OFFSET)
                    & PhyTmrLpclkCfg::PHY_CLKHS2LP_TIME_MASK),
        }
    }

    /// Set **PHY_CLKLP2HS_TIME** `[9:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_clklp2hs_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyTmrLpclkCfg::PHY_CLKLP2HS_TIME_MASK)
                | ((value << PhyTmrLpclkCfg::PHY_CLKLP2HS_TIME_OFFSET)
                    & PhyTmrLpclkCfg::PHY_CLKLP2HS_TIME_MASK),
        }
    }
}

impl Default for PhyTmrLpclkCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PhyTmrLpclkCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyTmrLpclkCfgVal {
        PhyTmrLpclkCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyTmrLpclkCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyTmrLpclkCfgVal) -> PhyTmrLpclkCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_TMR_CFG Register
///
/// NA
///
/// Address offset: 0x009c
///
/// ## Fields
///
/// - **PHY_HS2LP_TIME** `[25:16]` (RW)
///
///   NA
///
/// - **PHY_LP2HS_TIME** `[9:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PhyTmrCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyTmrCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyTmrCfg {
    // ======== Field Constants ========

    /// **PHY_HS2LP_TIME**
    ///
    /// NA
    ///
    /// Bits `[25:16]`, mask `0x03ff0000`
    pub const PHY_HS2LP_TIME_OFFSET: u32 = 16;
    pub const PHY_HS2LP_TIME_MASK: u32 = 0x03ff0000;

    /// **PHY_LP2HS_TIME**
    ///
    /// NA
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const PHY_LP2HS_TIME_OFFSET: u32 = 0;
    pub const PHY_LP2HS_TIME_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **PHY_HS2LP_TIME** `[25:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_hs2lp_time(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_HS2LP_TIME_MASK) >> Self::PHY_HS2LP_TIME_OFFSET
    }

    /// Write **PHY_HS2LP_TIME** `[25:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_phy_hs2lp_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PHY_HS2LP_TIME_MASK)
                | ((value << Self::PHY_HS2LP_TIME_OFFSET) & Self::PHY_HS2LP_TIME_MASK)
        });
    }

    /// **PHY_LP2HS_TIME** `[9:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_lp2hs_time(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_LP2HS_TIME_MASK) >> Self::PHY_LP2HS_TIME_OFFSET
    }

    /// Write **PHY_LP2HS_TIME** `[9:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_phy_lp2hs_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PHY_LP2HS_TIME_MASK)
                | ((value << Self::PHY_LP2HS_TIME_OFFSET) & Self::PHY_LP2HS_TIME_MASK)
        });
    }
}

/// PHY_TMR_CFG value type for batch read/modify/write operations
///
/// Use `phytmrcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phytmrcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyTmrCfgVal {
    bits: u32,
}

impl PhyTmrCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PHY_HS2LP_TIME** `[25:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_hs2lp_time(&self) -> u32 {
        (self.bits & PhyTmrCfg::PHY_HS2LP_TIME_MASK) >> PhyTmrCfg::PHY_HS2LP_TIME_OFFSET
    }

    /// Read **PHY_LP2HS_TIME** `[9:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_lp2hs_time(&self) -> u32 {
        (self.bits & PhyTmrCfg::PHY_LP2HS_TIME_MASK) >> PhyTmrCfg::PHY_LP2HS_TIME_OFFSET
    }

    /// Set **PHY_HS2LP_TIME** `[25:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_hs2lp_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyTmrCfg::PHY_HS2LP_TIME_MASK)
                | ((value << PhyTmrCfg::PHY_HS2LP_TIME_OFFSET) & PhyTmrCfg::PHY_HS2LP_TIME_MASK),
        }
    }

    /// Set **PHY_LP2HS_TIME** `[9:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_lp2hs_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyTmrCfg::PHY_LP2HS_TIME_MASK)
                | ((value << PhyTmrCfg::PHY_LP2HS_TIME_OFFSET) & PhyTmrCfg::PHY_LP2HS_TIME_MASK),
        }
    }
}

impl Default for PhyTmrCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PhyTmrCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyTmrCfgVal {
        PhyTmrCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyTmrCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyTmrCfgVal) -> PhyTmrCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_RSTZ Register
///
/// NA
///
/// Address offset: 0x00a0
///
/// ## Fields
///
/// - **PHY_FORCEPLL** `[3]` (RW)
///
///   NA
///
/// - **PHY_ENABLECLK** `[2]` (RW)
///
///   NA
///
/// - **PHY_RSTZ** `[1]` (RW)
///
///   NA
///
/// - **PHY_SHUTDOWNZ** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PhyRstz {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyRstz {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyRstz {
    // ======== Field Constants ========

    /// **PHY_FORCEPLL**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PHY_FORCEPLL_OFFSET: u32 = 3;
    pub const PHY_FORCEPLL_MASK: u32 = 0x00000008;
    pub const PHY_FORCEPLL: u32 = 1 << 3;

    /// **PHY_ENABLECLK**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PHY_ENABLECLK_OFFSET: u32 = 2;
    pub const PHY_ENABLECLK_MASK: u32 = 0x00000004;
    pub const PHY_ENABLECLK: u32 = 1 << 2;

    /// **PHY_RSTZ**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PHY_RSTZ_OFFSET: u32 = 1;
    pub const PHY_RSTZ_MASK: u32 = 0x00000002;
    pub const PHY_RSTZ: u32 = 1 << 1;

    /// **PHY_SHUTDOWNZ**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PHY_SHUTDOWNZ_OFFSET: u32 = 0;
    pub const PHY_SHUTDOWNZ_MASK: u32 = 0x00000001;
    pub const PHY_SHUTDOWNZ: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PHY_FORCEPLL** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_forcepll(&self) -> bool {
        (self.value.read_bits() & Self::PHY_FORCEPLL_MASK) != 0
    }

    /// Set PHY_FORCEPLL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_forcepll(&self) {
        self.value.modify_bits(|r| r | Self::PHY_FORCEPLL_MASK);
    }

    /// Clear PHY_FORCEPLL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_forcepll(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_FORCEPLL_MASK);
    }

    /// **PHY_ENABLECLK** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_enableclk(&self) -> bool {
        (self.value.read_bits() & Self::PHY_ENABLECLK_MASK) != 0
    }

    /// Set PHY_ENABLECLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_enableclk(&self) {
        self.value.modify_bits(|r| r | Self::PHY_ENABLECLK_MASK);
    }

    /// Clear PHY_ENABLECLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_enableclk(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_ENABLECLK_MASK);
    }

    /// **PHY_RSTZ** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_rstz(&self) -> bool {
        (self.value.read_bits() & Self::PHY_RSTZ_MASK) != 0
    }

    /// Set PHY_RSTZ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_rstz(&self) {
        self.value.modify_bits(|r| r | Self::PHY_RSTZ_MASK);
    }

    /// Clear PHY_RSTZ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_rstz(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_RSTZ_MASK);
    }

    /// **PHY_SHUTDOWNZ** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_shutdownz(&self) -> bool {
        (self.value.read_bits() & Self::PHY_SHUTDOWNZ_MASK) != 0
    }

    /// Set PHY_SHUTDOWNZ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_shutdownz(&self) {
        self.value.modify_bits(|r| r | Self::PHY_SHUTDOWNZ_MASK);
    }

    /// Clear PHY_SHUTDOWNZ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_shutdownz(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_SHUTDOWNZ_MASK);
    }
}

/// PHY_RSTZ value type for batch read/modify/write operations
///
/// Use `phyrstz.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phyrstz.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyRstzVal {
    bits: u32,
}

impl PhyRstzVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PHY_FORCEPLL** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_forcepll(&self) -> bool {
        (self.bits & PhyRstz::PHY_FORCEPLL_MASK) != 0
    }

    /// Read **PHY_ENABLECLK** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_enableclk(&self) -> bool {
        (self.bits & PhyRstz::PHY_ENABLECLK_MASK) != 0
    }

    /// Read **PHY_RSTZ** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_rstz(&self) -> bool {
        (self.bits & PhyRstz::PHY_RSTZ_MASK) != 0
    }

    /// Read **PHY_SHUTDOWNZ** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_shutdownz(&self) -> bool {
        (self.bits & PhyRstz::PHY_SHUTDOWNZ_MASK) != 0
    }

    /// Set **PHY_FORCEPLL** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_forcepll(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyRstz::PHY_FORCEPLL_MASK
            } else {
                self.bits & !PhyRstz::PHY_FORCEPLL_MASK
            },
        }
    }

    /// Set **PHY_ENABLECLK** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_enableclk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyRstz::PHY_ENABLECLK_MASK
            } else {
                self.bits & !PhyRstz::PHY_ENABLECLK_MASK
            },
        }
    }

    /// Set **PHY_RSTZ** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_rstz(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyRstz::PHY_RSTZ_MASK
            } else {
                self.bits & !PhyRstz::PHY_RSTZ_MASK
            },
        }
    }

    /// Set **PHY_SHUTDOWNZ** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_shutdownz(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyRstz::PHY_SHUTDOWNZ_MASK
            } else {
                self.bits & !PhyRstz::PHY_SHUTDOWNZ_MASK
            },
        }
    }
}

impl Default for PhyRstzVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PhyRstz {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyRstzVal {
        PhyRstzVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyRstzVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyRstzVal) -> PhyRstzVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_IF_CFG Register
///
/// NA
///
/// Address offset: 0x00a4
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **PHY_STOP_WAIT_TIME** `[15:8]` (RW)
///
///   NA
///
/// - **N_LANES** `[1:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PhyIfCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyIfCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyIfCfg {
    // ======== Field Constants ========

    /// **PHY_STOP_WAIT_TIME**
    ///
    /// NA
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const PHY_STOP_WAIT_TIME_OFFSET: u32 = 8;
    pub const PHY_STOP_WAIT_TIME_MASK: u32 = 0x0000ff00;

    /// **N_LANES**
    ///
    /// NA
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const N_LANES_OFFSET: u32 = 0;
    pub const N_LANES_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **PHY_STOP_WAIT_TIME** `[15:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_stop_wait_time(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_STOP_WAIT_TIME_MASK) >> Self::PHY_STOP_WAIT_TIME_OFFSET
    }

    /// Write **PHY_STOP_WAIT_TIME** `[15:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_phy_stop_wait_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PHY_STOP_WAIT_TIME_MASK)
                | ((value << Self::PHY_STOP_WAIT_TIME_OFFSET) & Self::PHY_STOP_WAIT_TIME_MASK)
        });
    }

    /// **N_LANES** `[1:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn n_lanes(&self) -> u32 {
        (self.value.read_bits() & Self::N_LANES_MASK) >> Self::N_LANES_OFFSET
    }

    /// Write **N_LANES** `[1:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_n_lanes(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::N_LANES_MASK) | ((value << Self::N_LANES_OFFSET) & Self::N_LANES_MASK)
        });
    }
}

/// PHY_IF_CFG value type for batch read/modify/write operations
///
/// Use `phyifcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phyifcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyIfCfgVal {
    bits: u32,
}

impl PhyIfCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000001 }
    }

    /// Read **PHY_STOP_WAIT_TIME** `[15:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_stop_wait_time(&self) -> u32 {
        (self.bits & PhyIfCfg::PHY_STOP_WAIT_TIME_MASK) >> PhyIfCfg::PHY_STOP_WAIT_TIME_OFFSET
    }

    /// Read **N_LANES** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn n_lanes(&self) -> u32 {
        (self.bits & PhyIfCfg::N_LANES_MASK) >> PhyIfCfg::N_LANES_OFFSET
    }

    /// Set **PHY_STOP_WAIT_TIME** `[15:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_stop_wait_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyIfCfg::PHY_STOP_WAIT_TIME_MASK)
                | ((value << PhyIfCfg::PHY_STOP_WAIT_TIME_OFFSET)
                    & PhyIfCfg::PHY_STOP_WAIT_TIME_MASK),
        }
    }

    /// Set **N_LANES** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_n_lanes(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyIfCfg::N_LANES_MASK)
                | ((value << PhyIfCfg::N_LANES_OFFSET) & PhyIfCfg::N_LANES_MASK),
        }
    }
}

impl Default for PhyIfCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PhyIfCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyIfCfgVal {
        PhyIfCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyIfCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyIfCfgVal) -> PhyIfCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_ULPS_CTRL Register
///
/// NA
///
/// Address offset: 0x00a8
///
/// ## Fields
///
/// - **PHY_TXEXITULPSLAN** `[3]` (RW)
///
///   NA
///
/// - **PHY_TXREQULPSLAN** `[2]` (RW)
///
///   NA
///
/// - **PHY_TXEXITULPSCLK** `[1]` (RW)
///
///   NA
///
/// - **PHY_TXREQULPSCLK** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PhyUlpsCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyUlpsCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyUlpsCtrl {
    // ======== Field Constants ========

    /// **PHY_TXEXITULPSLAN**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PHY_TXEXITULPSLAN_OFFSET: u32 = 3;
    pub const PHY_TXEXITULPSLAN_MASK: u32 = 0x00000008;
    pub const PHY_TXEXITULPSLAN: u32 = 1 << 3;

    /// **PHY_TXREQULPSLAN**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PHY_TXREQULPSLAN_OFFSET: u32 = 2;
    pub const PHY_TXREQULPSLAN_MASK: u32 = 0x00000004;
    pub const PHY_TXREQULPSLAN: u32 = 1 << 2;

    /// **PHY_TXEXITULPSCLK**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PHY_TXEXITULPSCLK_OFFSET: u32 = 1;
    pub const PHY_TXEXITULPSCLK_MASK: u32 = 0x00000002;
    pub const PHY_TXEXITULPSCLK: u32 = 1 << 1;

    /// **PHY_TXREQULPSCLK**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PHY_TXREQULPSCLK_OFFSET: u32 = 0;
    pub const PHY_TXREQULPSCLK_MASK: u32 = 0x00000001;
    pub const PHY_TXREQULPSCLK: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PHY_TXEXITULPSLAN** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_txexitulpslan(&self) -> bool {
        (self.value.read_bits() & Self::PHY_TXEXITULPSLAN_MASK) != 0
    }

    /// Set PHY_TXEXITULPSLAN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_txexitulpslan(&self) {
        self.value.modify_bits(|r| r | Self::PHY_TXEXITULPSLAN_MASK);
    }

    /// Clear PHY_TXEXITULPSLAN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_txexitulpslan(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_TXEXITULPSLAN_MASK);
    }

    /// **PHY_TXREQULPSLAN** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_txrequlpslan(&self) -> bool {
        (self.value.read_bits() & Self::PHY_TXREQULPSLAN_MASK) != 0
    }

    /// Set PHY_TXREQULPSLAN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_txrequlpslan(&self) {
        self.value.modify_bits(|r| r | Self::PHY_TXREQULPSLAN_MASK);
    }

    /// Clear PHY_TXREQULPSLAN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_txrequlpslan(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_TXREQULPSLAN_MASK);
    }

    /// **PHY_TXEXITULPSCLK** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_txexitulpsclk(&self) -> bool {
        (self.value.read_bits() & Self::PHY_TXEXITULPSCLK_MASK) != 0
    }

    /// Set PHY_TXEXITULPSCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_txexitulpsclk(&self) {
        self.value.modify_bits(|r| r | Self::PHY_TXEXITULPSCLK_MASK);
    }

    /// Clear PHY_TXEXITULPSCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_txexitulpsclk(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_TXEXITULPSCLK_MASK);
    }

    /// **PHY_TXREQULPSCLK** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_txrequlpsclk(&self) -> bool {
        (self.value.read_bits() & Self::PHY_TXREQULPSCLK_MASK) != 0
    }

    /// Set PHY_TXREQULPSCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_txrequlpsclk(&self) {
        self.value.modify_bits(|r| r | Self::PHY_TXREQULPSCLK_MASK);
    }

    /// Clear PHY_TXREQULPSCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_txrequlpsclk(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_TXREQULPSCLK_MASK);
    }
}

/// PHY_ULPS_CTRL value type for batch read/modify/write operations
///
/// Use `phyulpsctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phyulpsctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyUlpsCtrlVal {
    bits: u32,
}

impl PhyUlpsCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PHY_TXEXITULPSLAN** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_txexitulpslan(&self) -> bool {
        (self.bits & PhyUlpsCtrl::PHY_TXEXITULPSLAN_MASK) != 0
    }

    /// Read **PHY_TXREQULPSLAN** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_txrequlpslan(&self) -> bool {
        (self.bits & PhyUlpsCtrl::PHY_TXREQULPSLAN_MASK) != 0
    }

    /// Read **PHY_TXEXITULPSCLK** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_txexitulpsclk(&self) -> bool {
        (self.bits & PhyUlpsCtrl::PHY_TXEXITULPSCLK_MASK) != 0
    }

    /// Read **PHY_TXREQULPSCLK** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_txrequlpsclk(&self) -> bool {
        (self.bits & PhyUlpsCtrl::PHY_TXREQULPSCLK_MASK) != 0
    }

    /// Set **PHY_TXEXITULPSLAN** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_txexitulpslan(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyUlpsCtrl::PHY_TXEXITULPSLAN_MASK
            } else {
                self.bits & !PhyUlpsCtrl::PHY_TXEXITULPSLAN_MASK
            },
        }
    }

    /// Set **PHY_TXREQULPSLAN** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_txrequlpslan(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyUlpsCtrl::PHY_TXREQULPSLAN_MASK
            } else {
                self.bits & !PhyUlpsCtrl::PHY_TXREQULPSLAN_MASK
            },
        }
    }

    /// Set **PHY_TXEXITULPSCLK** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_txexitulpsclk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyUlpsCtrl::PHY_TXEXITULPSCLK_MASK
            } else {
                self.bits & !PhyUlpsCtrl::PHY_TXEXITULPSCLK_MASK
            },
        }
    }

    /// Set **PHY_TXREQULPSCLK** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_txrequlpsclk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyUlpsCtrl::PHY_TXREQULPSCLK_MASK
            } else {
                self.bits & !PhyUlpsCtrl::PHY_TXREQULPSCLK_MASK
            },
        }
    }
}

impl Default for PhyUlpsCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PhyUlpsCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyUlpsCtrlVal {
        PhyUlpsCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyUlpsCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyUlpsCtrlVal) -> PhyUlpsCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_TX_TRIGGERS Register
///
/// NA
///
/// Address offset: 0x00ac
///
/// ## Fields
///
/// - **PHY_TX_TRIGGERS** `[3:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PhyTxTriggers {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyTxTriggers {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyTxTriggers {
    // ======== Field Constants ========

    /// **PHY_TX_TRIGGERS**
    ///
    /// NA
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const PHY_TX_TRIGGERS_OFFSET: u32 = 0;
    pub const PHY_TX_TRIGGERS_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **PHY_TX_TRIGGERS** `[3:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_tx_triggers(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_TX_TRIGGERS_MASK) >> Self::PHY_TX_TRIGGERS_OFFSET
    }

    /// Write **PHY_TX_TRIGGERS** `[3:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_phy_tx_triggers(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PHY_TX_TRIGGERS_MASK)
                | ((value << Self::PHY_TX_TRIGGERS_OFFSET) & Self::PHY_TX_TRIGGERS_MASK)
        });
    }
}

/// PHY_TX_TRIGGERS value type for batch read/modify/write operations
///
/// Use `phytxtriggers.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phytxtriggers.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyTxTriggersVal {
    bits: u32,
}

impl PhyTxTriggersVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PHY_TX_TRIGGERS** `[3:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_tx_triggers(&self) -> u32 {
        (self.bits & PhyTxTriggers::PHY_TX_TRIGGERS_MASK) >> PhyTxTriggers::PHY_TX_TRIGGERS_OFFSET
    }

    /// Set **PHY_TX_TRIGGERS** `[3:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_tx_triggers(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyTxTriggers::PHY_TX_TRIGGERS_MASK)
                | ((value << PhyTxTriggers::PHY_TX_TRIGGERS_OFFSET)
                    & PhyTxTriggers::PHY_TX_TRIGGERS_MASK),
        }
    }
}

impl Default for PhyTxTriggersVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PhyTxTriggers {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyTxTriggersVal {
        PhyTxTriggersVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyTxTriggersVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyTxTriggersVal) -> PhyTxTriggersVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_STATUS Register
///
/// NA
///
/// Address offset: 0x00b0
/// Reset value: 0x00000140
///
/// ## Fields
///
/// - **PHY_ULPSACTIVENOT1LANE** `[8]` (RO)
///
///   NA
///
/// - **PHY_STOPSTATE1LANE** `[7]` (RO)
///
///   NA
///
/// - **PHY_RXULPSESC0LANE** `[6]` (RO)
///
///   NA
///
/// - **PHY_ULPSACTIVENOT0LANE** `[5]` (RO)
///
///   NA
///
/// - **PHY_STOPSTATE0LANE** `[4]` (RO)
///
///   NA
///
/// - **PHY_ULPSACTIVENOTCLK** `[3]` (RO)
///
///   NA
///
/// - **PHY_STOPSTATECLKLANE** `[2]` (RO)
///
///   NA
///
/// - **PHY_DIRECTION** `[1]` (RO)
///
///   NA
///
/// - **PHY_LOCK** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct PhyStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyStatus {
    // ======== Field Constants ========

    /// **PHY_ULPSACTIVENOT1LANE**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const PHY_ULPSACTIVENOT1LANE_OFFSET: u32 = 8;
    pub const PHY_ULPSACTIVENOT1LANE_MASK: u32 = 0x00000100;
    pub const PHY_ULPSACTIVENOT1LANE: u32 = 1 << 8;

    /// **PHY_STOPSTATE1LANE**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const PHY_STOPSTATE1LANE_OFFSET: u32 = 7;
    pub const PHY_STOPSTATE1LANE_MASK: u32 = 0x00000080;
    pub const PHY_STOPSTATE1LANE: u32 = 1 << 7;

    /// **PHY_RXULPSESC0LANE**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const PHY_RXULPSESC0LANE_OFFSET: u32 = 6;
    pub const PHY_RXULPSESC0LANE_MASK: u32 = 0x00000040;
    pub const PHY_RXULPSESC0LANE: u32 = 1 << 6;

    /// **PHY_ULPSACTIVENOT0LANE**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const PHY_ULPSACTIVENOT0LANE_OFFSET: u32 = 5;
    pub const PHY_ULPSACTIVENOT0LANE_MASK: u32 = 0x00000020;
    pub const PHY_ULPSACTIVENOT0LANE: u32 = 1 << 5;

    /// **PHY_STOPSTATE0LANE**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const PHY_STOPSTATE0LANE_OFFSET: u32 = 4;
    pub const PHY_STOPSTATE0LANE_MASK: u32 = 0x00000010;
    pub const PHY_STOPSTATE0LANE: u32 = 1 << 4;

    /// **PHY_ULPSACTIVENOTCLK**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PHY_ULPSACTIVENOTCLK_OFFSET: u32 = 3;
    pub const PHY_ULPSACTIVENOTCLK_MASK: u32 = 0x00000008;
    pub const PHY_ULPSACTIVENOTCLK: u32 = 1 << 3;

    /// **PHY_STOPSTATECLKLANE**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PHY_STOPSTATECLKLANE_OFFSET: u32 = 2;
    pub const PHY_STOPSTATECLKLANE_MASK: u32 = 0x00000004;
    pub const PHY_STOPSTATECLKLANE: u32 = 1 << 2;

    /// **PHY_DIRECTION**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PHY_DIRECTION_OFFSET: u32 = 1;
    pub const PHY_DIRECTION_MASK: u32 = 0x00000002;
    pub const PHY_DIRECTION: u32 = 1 << 1;

    /// **PHY_LOCK**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PHY_LOCK_OFFSET: u32 = 0;
    pub const PHY_LOCK_MASK: u32 = 0x00000001;
    pub const PHY_LOCK: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PHY_ULPSACTIVENOT1LANE** `[8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_ulpsactivenot1lane(&self) -> bool {
        (self.value.read_bits() & Self::PHY_ULPSACTIVENOT1LANE_MASK) != 0
    }

    /// **PHY_STOPSTATE1LANE** `[7]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_stopstate1lane(&self) -> bool {
        (self.value.read_bits() & Self::PHY_STOPSTATE1LANE_MASK) != 0
    }

    /// **PHY_RXULPSESC0LANE** `[6]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_rxulpsesc0lane(&self) -> bool {
        (self.value.read_bits() & Self::PHY_RXULPSESC0LANE_MASK) != 0
    }

    /// **PHY_ULPSACTIVENOT0LANE** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_ulpsactivenot0lane(&self) -> bool {
        (self.value.read_bits() & Self::PHY_ULPSACTIVENOT0LANE_MASK) != 0
    }

    /// **PHY_STOPSTATE0LANE** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_stopstate0lane(&self) -> bool {
        (self.value.read_bits() & Self::PHY_STOPSTATE0LANE_MASK) != 0
    }

    /// **PHY_ULPSACTIVENOTCLK** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_ulpsactivenotclk(&self) -> bool {
        (self.value.read_bits() & Self::PHY_ULPSACTIVENOTCLK_MASK) != 0
    }

    /// **PHY_STOPSTATECLKLANE** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_stopstateclklane(&self) -> bool {
        (self.value.read_bits() & Self::PHY_STOPSTATECLKLANE_MASK) != 0
    }

    /// **PHY_DIRECTION** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_direction(&self) -> bool {
        (self.value.read_bits() & Self::PHY_DIRECTION_MASK) != 0
    }

    /// **PHY_LOCK** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_lock(&self) -> bool {
        (self.value.read_bits() & Self::PHY_LOCK_MASK) != 0
    }
}

/// PHY_STATUS value type for batch read/modify/write operations
///
/// Use `phystatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phystatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyStatusVal {
    bits: u32,
}

impl PhyStatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000140 }
    }

    /// Read **PHY_ULPSACTIVENOT1LANE** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_ulpsactivenot1lane(&self) -> bool {
        (self.bits & PhyStatus::PHY_ULPSACTIVENOT1LANE_MASK) != 0
    }

    /// Read **PHY_STOPSTATE1LANE** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_stopstate1lane(&self) -> bool {
        (self.bits & PhyStatus::PHY_STOPSTATE1LANE_MASK) != 0
    }

    /// Read **PHY_RXULPSESC0LANE** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_rxulpsesc0lane(&self) -> bool {
        (self.bits & PhyStatus::PHY_RXULPSESC0LANE_MASK) != 0
    }

    /// Read **PHY_ULPSACTIVENOT0LANE** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_ulpsactivenot0lane(&self) -> bool {
        (self.bits & PhyStatus::PHY_ULPSACTIVENOT0LANE_MASK) != 0
    }

    /// Read **PHY_STOPSTATE0LANE** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_stopstate0lane(&self) -> bool {
        (self.bits & PhyStatus::PHY_STOPSTATE0LANE_MASK) != 0
    }

    /// Read **PHY_ULPSACTIVENOTCLK** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_ulpsactivenotclk(&self) -> bool {
        (self.bits & PhyStatus::PHY_ULPSACTIVENOTCLK_MASK) != 0
    }

    /// Read **PHY_STOPSTATECLKLANE** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_stopstateclklane(&self) -> bool {
        (self.bits & PhyStatus::PHY_STOPSTATECLKLANE_MASK) != 0
    }

    /// Read **PHY_DIRECTION** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_direction(&self) -> bool {
        (self.bits & PhyStatus::PHY_DIRECTION_MASK) != 0
    }

    /// Read **PHY_LOCK** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_lock(&self) -> bool {
        (self.bits & PhyStatus::PHY_LOCK_MASK) != 0
    }
}

impl Default for PhyStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PhyStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyStatusVal {
        PhyStatusVal::from_bits(self.value.read_bits())
    }
}

/// PHY_TST_CTRL0 Register
///
/// NA
///
/// Address offset: 0x00b4
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **PHY_TESTCLK** `[1]` (RW)
///
///   NA
///
/// - **PHY_TESTCLR** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PhyTstCtrl0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyTstCtrl0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyTstCtrl0 {
    // ======== Field Constants ========

    /// **PHY_TESTCLK**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PHY_TESTCLK_OFFSET: u32 = 1;
    pub const PHY_TESTCLK_MASK: u32 = 0x00000002;
    pub const PHY_TESTCLK: u32 = 1 << 1;

    /// **PHY_TESTCLR**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PHY_TESTCLR_OFFSET: u32 = 0;
    pub const PHY_TESTCLR_MASK: u32 = 0x00000001;
    pub const PHY_TESTCLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PHY_TESTCLK** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_testclk(&self) -> bool {
        (self.value.read_bits() & Self::PHY_TESTCLK_MASK) != 0
    }

    /// Set PHY_TESTCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_testclk(&self) {
        self.value.modify_bits(|r| r | Self::PHY_TESTCLK_MASK);
    }

    /// Clear PHY_TESTCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_testclk(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_TESTCLK_MASK);
    }

    /// **PHY_TESTCLR** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_testclr(&self) -> bool {
        (self.value.read_bits() & Self::PHY_TESTCLR_MASK) != 0
    }

    /// Set PHY_TESTCLR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_testclr(&self) {
        self.value.modify_bits(|r| r | Self::PHY_TESTCLR_MASK);
    }

    /// Clear PHY_TESTCLR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_testclr(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_TESTCLR_MASK);
    }
}

/// PHY_TST_CTRL0 value type for batch read/modify/write operations
///
/// Use `phytstctrl0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phytstctrl0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyTstCtrl0Val {
    bits: u32,
}

impl PhyTstCtrl0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000001 }
    }

    /// Read **PHY_TESTCLK** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_testclk(&self) -> bool {
        (self.bits & PhyTstCtrl0::PHY_TESTCLK_MASK) != 0
    }

    /// Read **PHY_TESTCLR** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_testclr(&self) -> bool {
        (self.bits & PhyTstCtrl0::PHY_TESTCLR_MASK) != 0
    }

    /// Set **PHY_TESTCLK** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_testclk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyTstCtrl0::PHY_TESTCLK_MASK
            } else {
                self.bits & !PhyTstCtrl0::PHY_TESTCLK_MASK
            },
        }
    }

    /// Set **PHY_TESTCLR** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_testclr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyTstCtrl0::PHY_TESTCLR_MASK
            } else {
                self.bits & !PhyTstCtrl0::PHY_TESTCLR_MASK
            },
        }
    }
}

impl Default for PhyTstCtrl0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PhyTstCtrl0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyTstCtrl0Val {
        PhyTstCtrl0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyTstCtrl0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyTstCtrl0Val) -> PhyTstCtrl0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_TST_CTRL1 Register
///
/// NA
///
/// Address offset: 0x00b8
///
/// ## Fields
///
/// - **PHY_TESTEN** `[16]` (RW)
///
///   NA
///
/// - **PHT_TESTDOUT** `[15:8]` (RO)
///
///   NA
///
/// - **PHY_TESTDIN** `[7:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PhyTstCtrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyTstCtrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyTstCtrl1 {
    // ======== Field Constants ========

    /// **PHY_TESTEN**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const PHY_TESTEN_OFFSET: u32 = 16;
    pub const PHY_TESTEN_MASK: u32 = 0x00010000;
    pub const PHY_TESTEN: u32 = 1 << 16;

    /// **PHT_TESTDOUT**
    ///
    /// NA
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const PHT_TESTDOUT_OFFSET: u32 = 8;
    pub const PHT_TESTDOUT_MASK: u32 = 0x0000ff00;

    /// **PHY_TESTDIN**
    ///
    /// NA
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const PHY_TESTDIN_OFFSET: u32 = 0;
    pub const PHY_TESTDIN_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **PHY_TESTEN** `[16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_testen(&self) -> bool {
        (self.value.read_bits() & Self::PHY_TESTEN_MASK) != 0
    }

    /// Set PHY_TESTEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_testen(&self) {
        self.value.modify_bits(|r| r | Self::PHY_TESTEN_MASK);
    }

    /// Clear PHY_TESTEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_testen(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_TESTEN_MASK);
    }

    /// **PHT_TESTDOUT** `[15:8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn pht_testdout(&self) -> u32 {
        (self.value.read_bits() & Self::PHT_TESTDOUT_MASK) >> Self::PHT_TESTDOUT_OFFSET
    }

    /// **PHY_TESTDIN** `[7:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_testdin(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_TESTDIN_MASK) >> Self::PHY_TESTDIN_OFFSET
    }

    /// Write **PHY_TESTDIN** `[7:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_phy_testdin(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PHY_TESTDIN_MASK)
                | ((value << Self::PHY_TESTDIN_OFFSET) & Self::PHY_TESTDIN_MASK)
        });
    }
}

/// PHY_TST_CTRL1 value type for batch read/modify/write operations
///
/// Use `phytstctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phytstctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyTstCtrl1Val {
    bits: u32,
}

impl PhyTstCtrl1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PHY_TESTEN** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_testen(&self) -> bool {
        (self.bits & PhyTstCtrl1::PHY_TESTEN_MASK) != 0
    }

    /// Read **PHT_TESTDOUT** `[15:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn pht_testdout(&self) -> u32 {
        (self.bits & PhyTstCtrl1::PHT_TESTDOUT_MASK) >> PhyTstCtrl1::PHT_TESTDOUT_OFFSET
    }

    /// Read **PHY_TESTDIN** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_testdin(&self) -> u32 {
        (self.bits & PhyTstCtrl1::PHY_TESTDIN_MASK) >> PhyTstCtrl1::PHY_TESTDIN_OFFSET
    }

    /// Set **PHY_TESTEN** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_testen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyTstCtrl1::PHY_TESTEN_MASK
            } else {
                self.bits & !PhyTstCtrl1::PHY_TESTEN_MASK
            },
        }
    }

    /// Set **PHY_TESTDIN** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_testdin(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyTstCtrl1::PHY_TESTDIN_MASK)
                | ((value << PhyTstCtrl1::PHY_TESTDIN_OFFSET) & PhyTstCtrl1::PHY_TESTDIN_MASK),
        }
    }
}

impl Default for PhyTstCtrl1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PhyTstCtrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyTstCtrl1Val {
        PhyTstCtrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyTstCtrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyTstCtrl1Val) -> PhyTstCtrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ST0 Register
///
/// NA
///
/// Address offset: 0x00bc
///
/// ## Fields
///
/// - **DPHY_ERRORS_4** `[20]` (RO)
///
///   NA
///
/// - **DPHY_ERRORS_3** `[19]` (RO)
///
///   NA
///
/// - **DPHY_ERRORS_2** `[18]` (RO)
///
///   NA
///
/// - **DPHY_ERRORS_1** `[17]` (RO)
///
///   NA
///
/// - **DPHY_ERRORS_0** `[16]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_15** `[15]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_14** `[14]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_13** `[13]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_12** `[12]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_11** `[11]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_10** `[10]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_9** `[9]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_8** `[8]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_7** `[7]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_6** `[6]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_5** `[5]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_4** `[4]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_3** `[3]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_2** `[2]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_1** `[1]` (RO)
///
///   NA
///
/// - **ACK_WITH_ERR_0** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntSt0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntSt0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntSt0 {
    // ======== Field Constants ========

    /// **DPHY_ERRORS_4**
    ///
    /// NA
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const DPHY_ERRORS_4_OFFSET: u32 = 20;
    pub const DPHY_ERRORS_4_MASK: u32 = 0x00100000;
    pub const DPHY_ERRORS_4: u32 = 1 << 20;

    /// **DPHY_ERRORS_3**
    ///
    /// NA
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const DPHY_ERRORS_3_OFFSET: u32 = 19;
    pub const DPHY_ERRORS_3_MASK: u32 = 0x00080000;
    pub const DPHY_ERRORS_3: u32 = 1 << 19;

    /// **DPHY_ERRORS_2**
    ///
    /// NA
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const DPHY_ERRORS_2_OFFSET: u32 = 18;
    pub const DPHY_ERRORS_2_MASK: u32 = 0x00040000;
    pub const DPHY_ERRORS_2: u32 = 1 << 18;

    /// **DPHY_ERRORS_1**
    ///
    /// NA
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const DPHY_ERRORS_1_OFFSET: u32 = 17;
    pub const DPHY_ERRORS_1_MASK: u32 = 0x00020000;
    pub const DPHY_ERRORS_1: u32 = 1 << 17;

    /// **DPHY_ERRORS_0**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const DPHY_ERRORS_0_OFFSET: u32 = 16;
    pub const DPHY_ERRORS_0_MASK: u32 = 0x00010000;
    pub const DPHY_ERRORS_0: u32 = 1 << 16;

    /// **ACK_WITH_ERR_15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const ACK_WITH_ERR_15_OFFSET: u32 = 15;
    pub const ACK_WITH_ERR_15_MASK: u32 = 0x00008000;
    pub const ACK_WITH_ERR_15: u32 = 1 << 15;

    /// **ACK_WITH_ERR_14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const ACK_WITH_ERR_14_OFFSET: u32 = 14;
    pub const ACK_WITH_ERR_14_MASK: u32 = 0x00004000;
    pub const ACK_WITH_ERR_14: u32 = 1 << 14;

    /// **ACK_WITH_ERR_13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const ACK_WITH_ERR_13_OFFSET: u32 = 13;
    pub const ACK_WITH_ERR_13_MASK: u32 = 0x00002000;
    pub const ACK_WITH_ERR_13: u32 = 1 << 13;

    /// **ACK_WITH_ERR_12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const ACK_WITH_ERR_12_OFFSET: u32 = 12;
    pub const ACK_WITH_ERR_12_MASK: u32 = 0x00001000;
    pub const ACK_WITH_ERR_12: u32 = 1 << 12;

    /// **ACK_WITH_ERR_11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ACK_WITH_ERR_11_OFFSET: u32 = 11;
    pub const ACK_WITH_ERR_11_MASK: u32 = 0x00000800;
    pub const ACK_WITH_ERR_11: u32 = 1 << 11;

    /// **ACK_WITH_ERR_10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ACK_WITH_ERR_10_OFFSET: u32 = 10;
    pub const ACK_WITH_ERR_10_MASK: u32 = 0x00000400;
    pub const ACK_WITH_ERR_10: u32 = 1 << 10;

    /// **ACK_WITH_ERR_9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ACK_WITH_ERR_9_OFFSET: u32 = 9;
    pub const ACK_WITH_ERR_9_MASK: u32 = 0x00000200;
    pub const ACK_WITH_ERR_9: u32 = 1 << 9;

    /// **ACK_WITH_ERR_8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ACK_WITH_ERR_8_OFFSET: u32 = 8;
    pub const ACK_WITH_ERR_8_MASK: u32 = 0x00000100;
    pub const ACK_WITH_ERR_8: u32 = 1 << 8;

    /// **ACK_WITH_ERR_7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ACK_WITH_ERR_7_OFFSET: u32 = 7;
    pub const ACK_WITH_ERR_7_MASK: u32 = 0x00000080;
    pub const ACK_WITH_ERR_7: u32 = 1 << 7;

    /// **ACK_WITH_ERR_6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ACK_WITH_ERR_6_OFFSET: u32 = 6;
    pub const ACK_WITH_ERR_6_MASK: u32 = 0x00000040;
    pub const ACK_WITH_ERR_6: u32 = 1 << 6;

    /// **ACK_WITH_ERR_5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ACK_WITH_ERR_5_OFFSET: u32 = 5;
    pub const ACK_WITH_ERR_5_MASK: u32 = 0x00000020;
    pub const ACK_WITH_ERR_5: u32 = 1 << 5;

    /// **ACK_WITH_ERR_4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ACK_WITH_ERR_4_OFFSET: u32 = 4;
    pub const ACK_WITH_ERR_4_MASK: u32 = 0x00000010;
    pub const ACK_WITH_ERR_4: u32 = 1 << 4;

    /// **ACK_WITH_ERR_3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ACK_WITH_ERR_3_OFFSET: u32 = 3;
    pub const ACK_WITH_ERR_3_MASK: u32 = 0x00000008;
    pub const ACK_WITH_ERR_3: u32 = 1 << 3;

    /// **ACK_WITH_ERR_2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ACK_WITH_ERR_2_OFFSET: u32 = 2;
    pub const ACK_WITH_ERR_2_MASK: u32 = 0x00000004;
    pub const ACK_WITH_ERR_2: u32 = 1 << 2;

    /// **ACK_WITH_ERR_1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ACK_WITH_ERR_1_OFFSET: u32 = 1;
    pub const ACK_WITH_ERR_1_MASK: u32 = 0x00000002;
    pub const ACK_WITH_ERR_1: u32 = 1 << 1;

    /// **ACK_WITH_ERR_0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ACK_WITH_ERR_0_OFFSET: u32 = 0;
    pub const ACK_WITH_ERR_0_MASK: u32 = 0x00000001;
    pub const ACK_WITH_ERR_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DPHY_ERRORS_4** `[20]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dphy_errors_4(&self) -> bool {
        (self.value.read_bits() & Self::DPHY_ERRORS_4_MASK) != 0
    }

    /// **DPHY_ERRORS_3** `[19]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dphy_errors_3(&self) -> bool {
        (self.value.read_bits() & Self::DPHY_ERRORS_3_MASK) != 0
    }

    /// **DPHY_ERRORS_2** `[18]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dphy_errors_2(&self) -> bool {
        (self.value.read_bits() & Self::DPHY_ERRORS_2_MASK) != 0
    }

    /// **DPHY_ERRORS_1** `[17]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dphy_errors_1(&self) -> bool {
        (self.value.read_bits() & Self::DPHY_ERRORS_1_MASK) != 0
    }

    /// **DPHY_ERRORS_0** `[16]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dphy_errors_0(&self) -> bool {
        (self.value.read_bits() & Self::DPHY_ERRORS_0_MASK) != 0
    }

    /// **ACK_WITH_ERR_15** `[15]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_15(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_15_MASK) != 0
    }

    /// **ACK_WITH_ERR_14** `[14]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_14(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_14_MASK) != 0
    }

    /// **ACK_WITH_ERR_13** `[13]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_13(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_13_MASK) != 0
    }

    /// **ACK_WITH_ERR_12** `[12]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_12(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_12_MASK) != 0
    }

    /// **ACK_WITH_ERR_11** `[11]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_11(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_11_MASK) != 0
    }

    /// **ACK_WITH_ERR_10** `[10]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_10(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_10_MASK) != 0
    }

    /// **ACK_WITH_ERR_9** `[9]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_9(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_9_MASK) != 0
    }

    /// **ACK_WITH_ERR_8** `[8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_8(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_8_MASK) != 0
    }

    /// **ACK_WITH_ERR_7** `[7]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_7(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_7_MASK) != 0
    }

    /// **ACK_WITH_ERR_6** `[6]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_6(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_6_MASK) != 0
    }

    /// **ACK_WITH_ERR_5** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_5(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_5_MASK) != 0
    }

    /// **ACK_WITH_ERR_4** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_4(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_4_MASK) != 0
    }

    /// **ACK_WITH_ERR_3** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_3(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_3_MASK) != 0
    }

    /// **ACK_WITH_ERR_2** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_2(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_2_MASK) != 0
    }

    /// **ACK_WITH_ERR_1** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_1(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_1_MASK) != 0
    }

    /// **ACK_WITH_ERR_0** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ack_with_err_0(&self) -> bool {
        (self.value.read_bits() & Self::ACK_WITH_ERR_0_MASK) != 0
    }
}

/// INT_ST0 value type for batch read/modify/write operations
///
/// Use `intst0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intst0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntSt0Val {
    bits: u32,
}

impl IntSt0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DPHY_ERRORS_4** `[20:20]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dphy_errors_4(&self) -> bool {
        (self.bits & IntSt0::DPHY_ERRORS_4_MASK) != 0
    }

    /// Read **DPHY_ERRORS_3** `[19:19]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dphy_errors_3(&self) -> bool {
        (self.bits & IntSt0::DPHY_ERRORS_3_MASK) != 0
    }

    /// Read **DPHY_ERRORS_2** `[18:18]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dphy_errors_2(&self) -> bool {
        (self.bits & IntSt0::DPHY_ERRORS_2_MASK) != 0
    }

    /// Read **DPHY_ERRORS_1** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dphy_errors_1(&self) -> bool {
        (self.bits & IntSt0::DPHY_ERRORS_1_MASK) != 0
    }

    /// Read **DPHY_ERRORS_0** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dphy_errors_0(&self) -> bool {
        (self.bits & IntSt0::DPHY_ERRORS_0_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_15(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_15_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_14(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_14_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_13(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_13_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_12(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_12_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_11(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_11_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_10(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_10_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_9(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_9_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_8(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_8_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_7(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_7_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_6(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_6_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_5(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_5_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_4(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_4_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_3(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_3_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_2(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_2_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_1(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_1_MASK) != 0
    }

    /// Read **ACK_WITH_ERR_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ack_with_err_0(&self) -> bool {
        (self.bits & IntSt0::ACK_WITH_ERR_0_MASK) != 0
    }
}

impl Default for IntSt0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntSt0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntSt0Val {
        IntSt0Val::from_bits(self.value.read_bits())
    }
}

/// INT_ST1 Register
///
/// NA
///
/// Address offset: 0x00c0
///
/// ## Fields
///
/// - **DPI_BUFF_PLD_UNDER** `[19]` (RO)
///
///   NA
///
/// - **GEN_PLD_RECEV_ERR** `[12]` (RO)
///
///   NA
///
/// - **GEN_PLD_RD_ERR** `[11]` (RO)
///
///   NA
///
/// - **GEN_PLD_SEND_ERR** `[10]` (RO)
///
///   NA
///
/// - **GEN_PLD_WR_ERR** `[9]` (RO)
///
///   NA
///
/// - **GEN_CMD_WR_ERR** `[8]` (RO)
///
///   NA
///
/// - **DPI_PLD_WR_ERR** `[7]` (RO)
///
///   NA
///
/// - **EOPT_ERR** `[6]` (RO)
///
///   NA
///
/// - **PKT_SIZE_ERR** `[5]` (RO)
///
///   NA
///
/// - **CRC_ERR** `[4]` (RO)
///
///   NA
///
/// - **ECC_MILTI_ERR** `[3]` (RO)
///
///   NA
///
/// - **ECC_SINGLE_ERR** `[2]` (RO)
///
///   NA
///
/// - **TO_LP_RX** `[1]` (RO)
///
///   NA
///
/// - **TO_HS_TX** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntSt1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntSt1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntSt1 {
    // ======== Field Constants ========

    /// **DPI_BUFF_PLD_UNDER**
    ///
    /// NA
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const DPI_BUFF_PLD_UNDER_OFFSET: u32 = 19;
    pub const DPI_BUFF_PLD_UNDER_MASK: u32 = 0x00080000;
    pub const DPI_BUFF_PLD_UNDER: u32 = 1 << 19;

    /// **GEN_PLD_RECEV_ERR**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const GEN_PLD_RECEV_ERR_OFFSET: u32 = 12;
    pub const GEN_PLD_RECEV_ERR_MASK: u32 = 0x00001000;
    pub const GEN_PLD_RECEV_ERR: u32 = 1 << 12;

    /// **GEN_PLD_RD_ERR**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const GEN_PLD_RD_ERR_OFFSET: u32 = 11;
    pub const GEN_PLD_RD_ERR_MASK: u32 = 0x00000800;
    pub const GEN_PLD_RD_ERR: u32 = 1 << 11;

    /// **GEN_PLD_SEND_ERR**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const GEN_PLD_SEND_ERR_OFFSET: u32 = 10;
    pub const GEN_PLD_SEND_ERR_MASK: u32 = 0x00000400;
    pub const GEN_PLD_SEND_ERR: u32 = 1 << 10;

    /// **GEN_PLD_WR_ERR**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const GEN_PLD_WR_ERR_OFFSET: u32 = 9;
    pub const GEN_PLD_WR_ERR_MASK: u32 = 0x00000200;
    pub const GEN_PLD_WR_ERR: u32 = 1 << 9;

    /// **GEN_CMD_WR_ERR**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const GEN_CMD_WR_ERR_OFFSET: u32 = 8;
    pub const GEN_CMD_WR_ERR_MASK: u32 = 0x00000100;
    pub const GEN_CMD_WR_ERR: u32 = 1 << 8;

    /// **DPI_PLD_WR_ERR**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DPI_PLD_WR_ERR_OFFSET: u32 = 7;
    pub const DPI_PLD_WR_ERR_MASK: u32 = 0x00000080;
    pub const DPI_PLD_WR_ERR: u32 = 1 << 7;

    /// **EOPT_ERR**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EOPT_ERR_OFFSET: u32 = 6;
    pub const EOPT_ERR_MASK: u32 = 0x00000040;
    pub const EOPT_ERR: u32 = 1 << 6;

    /// **PKT_SIZE_ERR**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const PKT_SIZE_ERR_OFFSET: u32 = 5;
    pub const PKT_SIZE_ERR_MASK: u32 = 0x00000020;
    pub const PKT_SIZE_ERR: u32 = 1 << 5;

    /// **CRC_ERR**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CRC_ERR_OFFSET: u32 = 4;
    pub const CRC_ERR_MASK: u32 = 0x00000010;
    pub const CRC_ERR: u32 = 1 << 4;

    /// **ECC_MILTI_ERR**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ECC_MILTI_ERR_OFFSET: u32 = 3;
    pub const ECC_MILTI_ERR_MASK: u32 = 0x00000008;
    pub const ECC_MILTI_ERR: u32 = 1 << 3;

    /// **ECC_SINGLE_ERR**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ECC_SINGLE_ERR_OFFSET: u32 = 2;
    pub const ECC_SINGLE_ERR_MASK: u32 = 0x00000004;
    pub const ECC_SINGLE_ERR: u32 = 1 << 2;

    /// **TO_LP_RX**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TO_LP_RX_OFFSET: u32 = 1;
    pub const TO_LP_RX_MASK: u32 = 0x00000002;
    pub const TO_LP_RX: u32 = 1 << 1;

    /// **TO_HS_TX**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TO_HS_TX_OFFSET: u32 = 0;
    pub const TO_HS_TX_MASK: u32 = 0x00000001;
    pub const TO_HS_TX: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DPI_BUFF_PLD_UNDER** `[19]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dpi_buff_pld_under(&self) -> bool {
        (self.value.read_bits() & Self::DPI_BUFF_PLD_UNDER_MASK) != 0
    }

    /// **GEN_PLD_RECEV_ERR** `[12]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_pld_recev_err(&self) -> bool {
        (self.value.read_bits() & Self::GEN_PLD_RECEV_ERR_MASK) != 0
    }

    /// **GEN_PLD_RD_ERR** `[11]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_pld_rd_err(&self) -> bool {
        (self.value.read_bits() & Self::GEN_PLD_RD_ERR_MASK) != 0
    }

    /// **GEN_PLD_SEND_ERR** `[10]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_pld_send_err(&self) -> bool {
        (self.value.read_bits() & Self::GEN_PLD_SEND_ERR_MASK) != 0
    }

    /// **GEN_PLD_WR_ERR** `[9]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_pld_wr_err(&self) -> bool {
        (self.value.read_bits() & Self::GEN_PLD_WR_ERR_MASK) != 0
    }

    /// **GEN_CMD_WR_ERR** `[8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn gen_cmd_wr_err(&self) -> bool {
        (self.value.read_bits() & Self::GEN_CMD_WR_ERR_MASK) != 0
    }

    /// **DPI_PLD_WR_ERR** `[7]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dpi_pld_wr_err(&self) -> bool {
        (self.value.read_bits() & Self::DPI_PLD_WR_ERR_MASK) != 0
    }

    /// **EOPT_ERR** `[6]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn eopt_err(&self) -> bool {
        (self.value.read_bits() & Self::EOPT_ERR_MASK) != 0
    }

    /// **PKT_SIZE_ERR** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn pkt_size_err(&self) -> bool {
        (self.value.read_bits() & Self::PKT_SIZE_ERR_MASK) != 0
    }

    /// **CRC_ERR** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn crc_err(&self) -> bool {
        (self.value.read_bits() & Self::CRC_ERR_MASK) != 0
    }

    /// **ECC_MILTI_ERR** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ecc_milti_err(&self) -> bool {
        (self.value.read_bits() & Self::ECC_MILTI_ERR_MASK) != 0
    }

    /// **ECC_SINGLE_ERR** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ecc_single_err(&self) -> bool {
        (self.value.read_bits() & Self::ECC_SINGLE_ERR_MASK) != 0
    }

    /// **TO_LP_RX** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn to_lp_rx(&self) -> bool {
        (self.value.read_bits() & Self::TO_LP_RX_MASK) != 0
    }

    /// **TO_HS_TX** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn to_hs_tx(&self) -> bool {
        (self.value.read_bits() & Self::TO_HS_TX_MASK) != 0
    }
}

/// INT_ST1 value type for batch read/modify/write operations
///
/// Use `intst1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intst1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntSt1Val {
    bits: u32,
}

impl IntSt1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DPI_BUFF_PLD_UNDER** `[19:19]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dpi_buff_pld_under(&self) -> bool {
        (self.bits & IntSt1::DPI_BUFF_PLD_UNDER_MASK) != 0
    }

    /// Read **GEN_PLD_RECEV_ERR** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_pld_recev_err(&self) -> bool {
        (self.bits & IntSt1::GEN_PLD_RECEV_ERR_MASK) != 0
    }

    /// Read **GEN_PLD_RD_ERR** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_pld_rd_err(&self) -> bool {
        (self.bits & IntSt1::GEN_PLD_RD_ERR_MASK) != 0
    }

    /// Read **GEN_PLD_SEND_ERR** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_pld_send_err(&self) -> bool {
        (self.bits & IntSt1::GEN_PLD_SEND_ERR_MASK) != 0
    }

    /// Read **GEN_PLD_WR_ERR** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_pld_wr_err(&self) -> bool {
        (self.bits & IntSt1::GEN_PLD_WR_ERR_MASK) != 0
    }

    /// Read **GEN_CMD_WR_ERR** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn gen_cmd_wr_err(&self) -> bool {
        (self.bits & IntSt1::GEN_CMD_WR_ERR_MASK) != 0
    }

    /// Read **DPI_PLD_WR_ERR** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dpi_pld_wr_err(&self) -> bool {
        (self.bits & IntSt1::DPI_PLD_WR_ERR_MASK) != 0
    }

    /// Read **EOPT_ERR** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn eopt_err(&self) -> bool {
        (self.bits & IntSt1::EOPT_ERR_MASK) != 0
    }

    /// Read **PKT_SIZE_ERR** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn pkt_size_err(&self) -> bool {
        (self.bits & IntSt1::PKT_SIZE_ERR_MASK) != 0
    }

    /// Read **CRC_ERR** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn crc_err(&self) -> bool {
        (self.bits & IntSt1::CRC_ERR_MASK) != 0
    }

    /// Read **ECC_MILTI_ERR** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ecc_milti_err(&self) -> bool {
        (self.bits & IntSt1::ECC_MILTI_ERR_MASK) != 0
    }

    /// Read **ECC_SINGLE_ERR** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ecc_single_err(&self) -> bool {
        (self.bits & IntSt1::ECC_SINGLE_ERR_MASK) != 0
    }

    /// Read **TO_LP_RX** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn to_lp_rx(&self) -> bool {
        (self.bits & IntSt1::TO_LP_RX_MASK) != 0
    }

    /// Read **TO_HS_TX** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn to_hs_tx(&self) -> bool {
        (self.bits & IntSt1::TO_HS_TX_MASK) != 0
    }
}

impl Default for IntSt1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntSt1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntSt1Val {
        IntSt1Val::from_bits(self.value.read_bits())
    }
}

/// INT_MSK0 Register
///
/// NA
///
/// Address offset: 0x00c4
///
/// ## Fields
///
/// - **MASK_DPHY_ERRORS_4** `[20]` (RW)
///
///   NA
///
/// - **MASK_DPHY_ERRORS_3** `[19]` (RW)
///
///   NA
///
/// - **MASK_DPHY_ERRORS_2** `[18]` (RW)
///
///   NA
///
/// - **MASK_DPHY_ERRORS_1** `[17]` (RW)
///
///   NA
///
/// - **MASK_DPHY_ERRORS_0** `[16]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_15** `[15]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_14** `[14]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_13** `[13]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_12** `[12]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_11** `[11]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_10** `[10]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_9** `[9]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_8** `[8]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_7** `[7]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_6** `[6]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_5** `[5]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_4** `[4]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_3** `[3]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_2** `[2]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_1** `[1]` (RW)
///
///   NA
///
/// - **MASK_ACK_WITH_ERR_0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntMsk0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntMsk0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntMsk0 {
    // ======== Field Constants ========

    /// **MASK_DPHY_ERRORS_4**
    ///
    /// NA
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const MASK_DPHY_ERRORS_4_OFFSET: u32 = 20;
    pub const MASK_DPHY_ERRORS_4_MASK: u32 = 0x00100000;
    pub const MASK_DPHY_ERRORS_4: u32 = 1 << 20;

    /// **MASK_DPHY_ERRORS_3**
    ///
    /// NA
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const MASK_DPHY_ERRORS_3_OFFSET: u32 = 19;
    pub const MASK_DPHY_ERRORS_3_MASK: u32 = 0x00080000;
    pub const MASK_DPHY_ERRORS_3: u32 = 1 << 19;

    /// **MASK_DPHY_ERRORS_2**
    ///
    /// NA
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const MASK_DPHY_ERRORS_2_OFFSET: u32 = 18;
    pub const MASK_DPHY_ERRORS_2_MASK: u32 = 0x00040000;
    pub const MASK_DPHY_ERRORS_2: u32 = 1 << 18;

    /// **MASK_DPHY_ERRORS_1**
    ///
    /// NA
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const MASK_DPHY_ERRORS_1_OFFSET: u32 = 17;
    pub const MASK_DPHY_ERRORS_1_MASK: u32 = 0x00020000;
    pub const MASK_DPHY_ERRORS_1: u32 = 1 << 17;

    /// **MASK_DPHY_ERRORS_0**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const MASK_DPHY_ERRORS_0_OFFSET: u32 = 16;
    pub const MASK_DPHY_ERRORS_0_MASK: u32 = 0x00010000;
    pub const MASK_DPHY_ERRORS_0: u32 = 1 << 16;

    /// **MASK_ACK_WITH_ERR_15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const MASK_ACK_WITH_ERR_15_OFFSET: u32 = 15;
    pub const MASK_ACK_WITH_ERR_15_MASK: u32 = 0x00008000;
    pub const MASK_ACK_WITH_ERR_15: u32 = 1 << 15;

    /// **MASK_ACK_WITH_ERR_14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MASK_ACK_WITH_ERR_14_OFFSET: u32 = 14;
    pub const MASK_ACK_WITH_ERR_14_MASK: u32 = 0x00004000;
    pub const MASK_ACK_WITH_ERR_14: u32 = 1 << 14;

    /// **MASK_ACK_WITH_ERR_13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const MASK_ACK_WITH_ERR_13_OFFSET: u32 = 13;
    pub const MASK_ACK_WITH_ERR_13_MASK: u32 = 0x00002000;
    pub const MASK_ACK_WITH_ERR_13: u32 = 1 << 13;

    /// **MASK_ACK_WITH_ERR_12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const MASK_ACK_WITH_ERR_12_OFFSET: u32 = 12;
    pub const MASK_ACK_WITH_ERR_12_MASK: u32 = 0x00001000;
    pub const MASK_ACK_WITH_ERR_12: u32 = 1 << 12;

    /// **MASK_ACK_WITH_ERR_11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const MASK_ACK_WITH_ERR_11_OFFSET: u32 = 11;
    pub const MASK_ACK_WITH_ERR_11_MASK: u32 = 0x00000800;
    pub const MASK_ACK_WITH_ERR_11: u32 = 1 << 11;

    /// **MASK_ACK_WITH_ERR_10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const MASK_ACK_WITH_ERR_10_OFFSET: u32 = 10;
    pub const MASK_ACK_WITH_ERR_10_MASK: u32 = 0x00000400;
    pub const MASK_ACK_WITH_ERR_10: u32 = 1 << 10;

    /// **MASK_ACK_WITH_ERR_9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const MASK_ACK_WITH_ERR_9_OFFSET: u32 = 9;
    pub const MASK_ACK_WITH_ERR_9_MASK: u32 = 0x00000200;
    pub const MASK_ACK_WITH_ERR_9: u32 = 1 << 9;

    /// **MASK_ACK_WITH_ERR_8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MASK_ACK_WITH_ERR_8_OFFSET: u32 = 8;
    pub const MASK_ACK_WITH_ERR_8_MASK: u32 = 0x00000100;
    pub const MASK_ACK_WITH_ERR_8: u32 = 1 << 8;

    /// **MASK_ACK_WITH_ERR_7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MASK_ACK_WITH_ERR_7_OFFSET: u32 = 7;
    pub const MASK_ACK_WITH_ERR_7_MASK: u32 = 0x00000080;
    pub const MASK_ACK_WITH_ERR_7: u32 = 1 << 7;

    /// **MASK_ACK_WITH_ERR_6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MASK_ACK_WITH_ERR_6_OFFSET: u32 = 6;
    pub const MASK_ACK_WITH_ERR_6_MASK: u32 = 0x00000040;
    pub const MASK_ACK_WITH_ERR_6: u32 = 1 << 6;

    /// **MASK_ACK_WITH_ERR_5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const MASK_ACK_WITH_ERR_5_OFFSET: u32 = 5;
    pub const MASK_ACK_WITH_ERR_5_MASK: u32 = 0x00000020;
    pub const MASK_ACK_WITH_ERR_5: u32 = 1 << 5;

    /// **MASK_ACK_WITH_ERR_4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MASK_ACK_WITH_ERR_4_OFFSET: u32 = 4;
    pub const MASK_ACK_WITH_ERR_4_MASK: u32 = 0x00000010;
    pub const MASK_ACK_WITH_ERR_4: u32 = 1 << 4;

    /// **MASK_ACK_WITH_ERR_3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MASK_ACK_WITH_ERR_3_OFFSET: u32 = 3;
    pub const MASK_ACK_WITH_ERR_3_MASK: u32 = 0x00000008;
    pub const MASK_ACK_WITH_ERR_3: u32 = 1 << 3;

    /// **MASK_ACK_WITH_ERR_2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MASK_ACK_WITH_ERR_2_OFFSET: u32 = 2;
    pub const MASK_ACK_WITH_ERR_2_MASK: u32 = 0x00000004;
    pub const MASK_ACK_WITH_ERR_2: u32 = 1 << 2;

    /// **MASK_ACK_WITH_ERR_1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MASK_ACK_WITH_ERR_1_OFFSET: u32 = 1;
    pub const MASK_ACK_WITH_ERR_1_MASK: u32 = 0x00000002;
    pub const MASK_ACK_WITH_ERR_1: u32 = 1 << 1;

    /// **MASK_ACK_WITH_ERR_0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MASK_ACK_WITH_ERR_0_OFFSET: u32 = 0;
    pub const MASK_ACK_WITH_ERR_0_MASK: u32 = 0x00000001;
    pub const MASK_ACK_WITH_ERR_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MASK_DPHY_ERRORS_4** `[20]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_dphy_errors_4(&self) -> bool {
        (self.value.read_bits() & Self::MASK_DPHY_ERRORS_4_MASK) != 0
    }

    /// Set MASK_DPHY_ERRORS_4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_dphy_errors_4(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_DPHY_ERRORS_4_MASK);
    }

    /// Clear MASK_DPHY_ERRORS_4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_dphy_errors_4(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_DPHY_ERRORS_4_MASK);
    }

    /// **MASK_DPHY_ERRORS_3** `[19]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_dphy_errors_3(&self) -> bool {
        (self.value.read_bits() & Self::MASK_DPHY_ERRORS_3_MASK) != 0
    }

    /// Set MASK_DPHY_ERRORS_3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_dphy_errors_3(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_DPHY_ERRORS_3_MASK);
    }

    /// Clear MASK_DPHY_ERRORS_3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_dphy_errors_3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_DPHY_ERRORS_3_MASK);
    }

    /// **MASK_DPHY_ERRORS_2** `[18]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_dphy_errors_2(&self) -> bool {
        (self.value.read_bits() & Self::MASK_DPHY_ERRORS_2_MASK) != 0
    }

    /// Set MASK_DPHY_ERRORS_2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_dphy_errors_2(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_DPHY_ERRORS_2_MASK);
    }

    /// Clear MASK_DPHY_ERRORS_2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_dphy_errors_2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_DPHY_ERRORS_2_MASK);
    }

    /// **MASK_DPHY_ERRORS_1** `[17]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_dphy_errors_1(&self) -> bool {
        (self.value.read_bits() & Self::MASK_DPHY_ERRORS_1_MASK) != 0
    }

    /// Set MASK_DPHY_ERRORS_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_dphy_errors_1(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_DPHY_ERRORS_1_MASK);
    }

    /// Clear MASK_DPHY_ERRORS_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_dphy_errors_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_DPHY_ERRORS_1_MASK);
    }

    /// **MASK_DPHY_ERRORS_0** `[16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_dphy_errors_0(&self) -> bool {
        (self.value.read_bits() & Self::MASK_DPHY_ERRORS_0_MASK) != 0
    }

    /// Set MASK_DPHY_ERRORS_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_dphy_errors_0(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_DPHY_ERRORS_0_MASK);
    }

    /// Clear MASK_DPHY_ERRORS_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_dphy_errors_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_DPHY_ERRORS_0_MASK);
    }

    /// **MASK_ACK_WITH_ERR_15** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_15(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_15_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_15(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_15_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_15(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_15_MASK);
    }

    /// **MASK_ACK_WITH_ERR_14** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_14(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_14_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_14(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_14_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_14(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_14_MASK);
    }

    /// **MASK_ACK_WITH_ERR_13** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_13(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_13_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_13(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_13_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_13(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_13_MASK);
    }

    /// **MASK_ACK_WITH_ERR_12** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_12(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_12_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_12(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_12_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_12(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_12_MASK);
    }

    /// **MASK_ACK_WITH_ERR_11** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_11(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_11_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_11(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_11_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_11(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_11_MASK);
    }

    /// **MASK_ACK_WITH_ERR_10** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_10(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_10_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_10(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_10_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_10(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_10_MASK);
    }

    /// **MASK_ACK_WITH_ERR_9** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_9(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_9_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_9(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_9_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_9(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_9_MASK);
    }

    /// **MASK_ACK_WITH_ERR_8** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_8(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_8_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_8(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_8_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_8(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_8_MASK);
    }

    /// **MASK_ACK_WITH_ERR_7** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_7(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_7_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_7(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_7_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_7(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_7_MASK);
    }

    /// **MASK_ACK_WITH_ERR_6** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_6(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_6_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_6(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_6_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_6(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_6_MASK);
    }

    /// **MASK_ACK_WITH_ERR_5** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_5(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_5_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_5(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_5_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_5(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_5_MASK);
    }

    /// **MASK_ACK_WITH_ERR_4** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_4(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_4_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_4(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_4_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_4(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_4_MASK);
    }

    /// **MASK_ACK_WITH_ERR_3** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_3(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_3_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_3(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_3_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_3_MASK);
    }

    /// **MASK_ACK_WITH_ERR_2** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_2(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_2_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_2(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_2_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_2_MASK);
    }

    /// **MASK_ACK_WITH_ERR_1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_1(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_1_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_1(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_1_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_1_MASK);
    }

    /// **MASK_ACK_WITH_ERR_0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ack_with_err_0(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ACK_WITH_ERR_0_MASK) != 0
    }

    /// Set MASK_ACK_WITH_ERR_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ack_with_err_0(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ACK_WITH_ERR_0_MASK);
    }

    /// Clear MASK_ACK_WITH_ERR_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ack_with_err_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ACK_WITH_ERR_0_MASK);
    }
}

/// INT_MSK0 value type for batch read/modify/write operations
///
/// Use `intmsk0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intmsk0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntMsk0Val {
    bits: u32,
}

impl IntMsk0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MASK_DPHY_ERRORS_4** `[20:20]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_dphy_errors_4(&self) -> bool {
        (self.bits & IntMsk0::MASK_DPHY_ERRORS_4_MASK) != 0
    }

    /// Read **MASK_DPHY_ERRORS_3** `[19:19]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_dphy_errors_3(&self) -> bool {
        (self.bits & IntMsk0::MASK_DPHY_ERRORS_3_MASK) != 0
    }

    /// Read **MASK_DPHY_ERRORS_2** `[18:18]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_dphy_errors_2(&self) -> bool {
        (self.bits & IntMsk0::MASK_DPHY_ERRORS_2_MASK) != 0
    }

    /// Read **MASK_DPHY_ERRORS_1** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_dphy_errors_1(&self) -> bool {
        (self.bits & IntMsk0::MASK_DPHY_ERRORS_1_MASK) != 0
    }

    /// Read **MASK_DPHY_ERRORS_0** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_dphy_errors_0(&self) -> bool {
        (self.bits & IntMsk0::MASK_DPHY_ERRORS_0_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_15(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_15_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_14(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_14_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_13(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_13_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_12(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_12_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_11(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_11_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_10(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_10_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_9(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_9_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_8(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_8_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_7(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_7_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_6(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_6_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_5(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_5_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_4(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_4_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_3(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_3_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_2(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_2_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_1(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_1_MASK) != 0
    }

    /// Read **MASK_ACK_WITH_ERR_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ack_with_err_0(&self) -> bool {
        (self.bits & IntMsk0::MASK_ACK_WITH_ERR_0_MASK) != 0
    }

    /// Set **MASK_DPHY_ERRORS_4** `[20:20]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_dphy_errors_4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_DPHY_ERRORS_4_MASK
            } else {
                self.bits & !IntMsk0::MASK_DPHY_ERRORS_4_MASK
            },
        }
    }

    /// Set **MASK_DPHY_ERRORS_3** `[19:19]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_dphy_errors_3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_DPHY_ERRORS_3_MASK
            } else {
                self.bits & !IntMsk0::MASK_DPHY_ERRORS_3_MASK
            },
        }
    }

    /// Set **MASK_DPHY_ERRORS_2** `[18:18]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_dphy_errors_2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_DPHY_ERRORS_2_MASK
            } else {
                self.bits & !IntMsk0::MASK_DPHY_ERRORS_2_MASK
            },
        }
    }

    /// Set **MASK_DPHY_ERRORS_1** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_dphy_errors_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_DPHY_ERRORS_1_MASK
            } else {
                self.bits & !IntMsk0::MASK_DPHY_ERRORS_1_MASK
            },
        }
    }

    /// Set **MASK_DPHY_ERRORS_0** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_dphy_errors_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_DPHY_ERRORS_0_MASK
            } else {
                self.bits & !IntMsk0::MASK_DPHY_ERRORS_0_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_15_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_15_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_14_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_14_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_13_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_13_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_12_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_12_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_11_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_11_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_10_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_10_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_9_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_9_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_8_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_8_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_7_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_7_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_6_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_6_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_5_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_5_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_4_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_4_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_3_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_3_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_2_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_2_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_1_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_1_MASK
            },
        }
    }

    /// Set **MASK_ACK_WITH_ERR_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ack_with_err_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk0::MASK_ACK_WITH_ERR_0_MASK
            } else {
                self.bits & !IntMsk0::MASK_ACK_WITH_ERR_0_MASK
            },
        }
    }
}

impl Default for IntMsk0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntMsk0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntMsk0Val {
        IntMsk0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntMsk0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntMsk0Val) -> IntMsk0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_MSK1 Register
///
/// NA
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **MASK_DPI_BUFF_PLD_UNDER** `[19]` (RW)
///
///   NA
///
/// - **MASK_GEN_PLD_RECEV_ERR** `[12]` (RW)
///
///   NA
///
/// - **MASK_GEN_PLD_RD_ERR** `[11]` (RW)
///
///   NA
///
/// - **MASK_GEN_PLD_SEND_ERR** `[10]` (RW)
///
///   NA
///
/// - **MASK_GEN_PLD_WR_ERR** `[9]` (RW)
///
///   NA
///
/// - **MASK_GEN_CMD_WR_ERR** `[8]` (RW)
///
///   NA
///
/// - **MASK_DPI_PLD_WR_ERR** `[7]` (RW)
///
///   NA
///
/// - **MASK_EOPT_ERR** `[6]` (RW)
///
///   NA
///
/// - **MASK_PKT_SIZE_ERR** `[5]` (RW)
///
///   NA
///
/// - **MASK_CRC_ERR** `[4]` (RW)
///
///   NA
///
/// - **MASK_ECC_MILTI_ERR** `[3]` (RW)
///
///   NA
///
/// - **MASK_ECC_SINGLE_ERR** `[2]` (RW)
///
///   NA
///
/// - **MASK_TO_LP_RX** `[1]` (RW)
///
///   NA
///
/// - **MASK_TO_HS_TX** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntMsk1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntMsk1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntMsk1 {
    // ======== Field Constants ========

    /// **MASK_DPI_BUFF_PLD_UNDER**
    ///
    /// NA
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const MASK_DPI_BUFF_PLD_UNDER_OFFSET: u32 = 19;
    pub const MASK_DPI_BUFF_PLD_UNDER_MASK: u32 = 0x00080000;
    pub const MASK_DPI_BUFF_PLD_UNDER: u32 = 1 << 19;

    /// **MASK_GEN_PLD_RECEV_ERR**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const MASK_GEN_PLD_RECEV_ERR_OFFSET: u32 = 12;
    pub const MASK_GEN_PLD_RECEV_ERR_MASK: u32 = 0x00001000;
    pub const MASK_GEN_PLD_RECEV_ERR: u32 = 1 << 12;

    /// **MASK_GEN_PLD_RD_ERR**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const MASK_GEN_PLD_RD_ERR_OFFSET: u32 = 11;
    pub const MASK_GEN_PLD_RD_ERR_MASK: u32 = 0x00000800;
    pub const MASK_GEN_PLD_RD_ERR: u32 = 1 << 11;

    /// **MASK_GEN_PLD_SEND_ERR**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const MASK_GEN_PLD_SEND_ERR_OFFSET: u32 = 10;
    pub const MASK_GEN_PLD_SEND_ERR_MASK: u32 = 0x00000400;
    pub const MASK_GEN_PLD_SEND_ERR: u32 = 1 << 10;

    /// **MASK_GEN_PLD_WR_ERR**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const MASK_GEN_PLD_WR_ERR_OFFSET: u32 = 9;
    pub const MASK_GEN_PLD_WR_ERR_MASK: u32 = 0x00000200;
    pub const MASK_GEN_PLD_WR_ERR: u32 = 1 << 9;

    /// **MASK_GEN_CMD_WR_ERR**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MASK_GEN_CMD_WR_ERR_OFFSET: u32 = 8;
    pub const MASK_GEN_CMD_WR_ERR_MASK: u32 = 0x00000100;
    pub const MASK_GEN_CMD_WR_ERR: u32 = 1 << 8;

    /// **MASK_DPI_PLD_WR_ERR**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MASK_DPI_PLD_WR_ERR_OFFSET: u32 = 7;
    pub const MASK_DPI_PLD_WR_ERR_MASK: u32 = 0x00000080;
    pub const MASK_DPI_PLD_WR_ERR: u32 = 1 << 7;

    /// **MASK_EOPT_ERR**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MASK_EOPT_ERR_OFFSET: u32 = 6;
    pub const MASK_EOPT_ERR_MASK: u32 = 0x00000040;
    pub const MASK_EOPT_ERR: u32 = 1 << 6;

    /// **MASK_PKT_SIZE_ERR**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const MASK_PKT_SIZE_ERR_OFFSET: u32 = 5;
    pub const MASK_PKT_SIZE_ERR_MASK: u32 = 0x00000020;
    pub const MASK_PKT_SIZE_ERR: u32 = 1 << 5;

    /// **MASK_CRC_ERR**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MASK_CRC_ERR_OFFSET: u32 = 4;
    pub const MASK_CRC_ERR_MASK: u32 = 0x00000010;
    pub const MASK_CRC_ERR: u32 = 1 << 4;

    /// **MASK_ECC_MILTI_ERR**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MASK_ECC_MILTI_ERR_OFFSET: u32 = 3;
    pub const MASK_ECC_MILTI_ERR_MASK: u32 = 0x00000008;
    pub const MASK_ECC_MILTI_ERR: u32 = 1 << 3;

    /// **MASK_ECC_SINGLE_ERR**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MASK_ECC_SINGLE_ERR_OFFSET: u32 = 2;
    pub const MASK_ECC_SINGLE_ERR_MASK: u32 = 0x00000004;
    pub const MASK_ECC_SINGLE_ERR: u32 = 1 << 2;

    /// **MASK_TO_LP_RX**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MASK_TO_LP_RX_OFFSET: u32 = 1;
    pub const MASK_TO_LP_RX_MASK: u32 = 0x00000002;
    pub const MASK_TO_LP_RX: u32 = 1 << 1;

    /// **MASK_TO_HS_TX**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MASK_TO_HS_TX_OFFSET: u32 = 0;
    pub const MASK_TO_HS_TX_MASK: u32 = 0x00000001;
    pub const MASK_TO_HS_TX: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MASK_DPI_BUFF_PLD_UNDER** `[19]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_dpi_buff_pld_under(&self) -> bool {
        (self.value.read_bits() & Self::MASK_DPI_BUFF_PLD_UNDER_MASK) != 0
    }

    /// Set MASK_DPI_BUFF_PLD_UNDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_dpi_buff_pld_under(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_DPI_BUFF_PLD_UNDER_MASK);
    }

    /// Clear MASK_DPI_BUFF_PLD_UNDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_dpi_buff_pld_under(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_DPI_BUFF_PLD_UNDER_MASK);
    }

    /// **MASK_GEN_PLD_RECEV_ERR** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_gen_pld_recev_err(&self) -> bool {
        (self.value.read_bits() & Self::MASK_GEN_PLD_RECEV_ERR_MASK) != 0
    }

    /// Set MASK_GEN_PLD_RECEV_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_gen_pld_recev_err(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_GEN_PLD_RECEV_ERR_MASK);
    }

    /// Clear MASK_GEN_PLD_RECEV_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_gen_pld_recev_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_GEN_PLD_RECEV_ERR_MASK);
    }

    /// **MASK_GEN_PLD_RD_ERR** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_gen_pld_rd_err(&self) -> bool {
        (self.value.read_bits() & Self::MASK_GEN_PLD_RD_ERR_MASK) != 0
    }

    /// Set MASK_GEN_PLD_RD_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_gen_pld_rd_err(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_GEN_PLD_RD_ERR_MASK);
    }

    /// Clear MASK_GEN_PLD_RD_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_gen_pld_rd_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_GEN_PLD_RD_ERR_MASK);
    }

    /// **MASK_GEN_PLD_SEND_ERR** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_gen_pld_send_err(&self) -> bool {
        (self.value.read_bits() & Self::MASK_GEN_PLD_SEND_ERR_MASK) != 0
    }

    /// Set MASK_GEN_PLD_SEND_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_gen_pld_send_err(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_GEN_PLD_SEND_ERR_MASK);
    }

    /// Clear MASK_GEN_PLD_SEND_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_gen_pld_send_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_GEN_PLD_SEND_ERR_MASK);
    }

    /// **MASK_GEN_PLD_WR_ERR** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_gen_pld_wr_err(&self) -> bool {
        (self.value.read_bits() & Self::MASK_GEN_PLD_WR_ERR_MASK) != 0
    }

    /// Set MASK_GEN_PLD_WR_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_gen_pld_wr_err(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_GEN_PLD_WR_ERR_MASK);
    }

    /// Clear MASK_GEN_PLD_WR_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_gen_pld_wr_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_GEN_PLD_WR_ERR_MASK);
    }

    /// **MASK_GEN_CMD_WR_ERR** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_gen_cmd_wr_err(&self) -> bool {
        (self.value.read_bits() & Self::MASK_GEN_CMD_WR_ERR_MASK) != 0
    }

    /// Set MASK_GEN_CMD_WR_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_gen_cmd_wr_err(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_GEN_CMD_WR_ERR_MASK);
    }

    /// Clear MASK_GEN_CMD_WR_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_gen_cmd_wr_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_GEN_CMD_WR_ERR_MASK);
    }

    /// **MASK_DPI_PLD_WR_ERR** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_dpi_pld_wr_err(&self) -> bool {
        (self.value.read_bits() & Self::MASK_DPI_PLD_WR_ERR_MASK) != 0
    }

    /// Set MASK_DPI_PLD_WR_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_dpi_pld_wr_err(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_DPI_PLD_WR_ERR_MASK);
    }

    /// Clear MASK_DPI_PLD_WR_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_dpi_pld_wr_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_DPI_PLD_WR_ERR_MASK);
    }

    /// **MASK_EOPT_ERR** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_eopt_err(&self) -> bool {
        (self.value.read_bits() & Self::MASK_EOPT_ERR_MASK) != 0
    }

    /// Set MASK_EOPT_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_eopt_err(&self) {
        self.value.modify_bits(|r| r | Self::MASK_EOPT_ERR_MASK);
    }

    /// Clear MASK_EOPT_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_eopt_err(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_EOPT_ERR_MASK);
    }

    /// **MASK_PKT_SIZE_ERR** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_pkt_size_err(&self) -> bool {
        (self.value.read_bits() & Self::MASK_PKT_SIZE_ERR_MASK) != 0
    }

    /// Set MASK_PKT_SIZE_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_pkt_size_err(&self) {
        self.value.modify_bits(|r| r | Self::MASK_PKT_SIZE_ERR_MASK);
    }

    /// Clear MASK_PKT_SIZE_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_pkt_size_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_PKT_SIZE_ERR_MASK);
    }

    /// **MASK_CRC_ERR** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_crc_err(&self) -> bool {
        (self.value.read_bits() & Self::MASK_CRC_ERR_MASK) != 0
    }

    /// Set MASK_CRC_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_crc_err(&self) {
        self.value.modify_bits(|r| r | Self::MASK_CRC_ERR_MASK);
    }

    /// Clear MASK_CRC_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_crc_err(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_CRC_ERR_MASK);
    }

    /// **MASK_ECC_MILTI_ERR** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ecc_milti_err(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ECC_MILTI_ERR_MASK) != 0
    }

    /// Set MASK_ECC_MILTI_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ecc_milti_err(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ECC_MILTI_ERR_MASK);
    }

    /// Clear MASK_ECC_MILTI_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ecc_milti_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ECC_MILTI_ERR_MASK);
    }

    /// **MASK_ECC_SINGLE_ERR** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_ecc_single_err(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ECC_SINGLE_ERR_MASK) != 0
    }

    /// Set MASK_ECC_SINGLE_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_ecc_single_err(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ECC_SINGLE_ERR_MASK);
    }

    /// Clear MASK_ECC_SINGLE_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_ecc_single_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ECC_SINGLE_ERR_MASK);
    }

    /// **MASK_TO_LP_RX** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_to_lp_rx(&self) -> bool {
        (self.value.read_bits() & Self::MASK_TO_LP_RX_MASK) != 0
    }

    /// Set MASK_TO_LP_RX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_to_lp_rx(&self) {
        self.value.modify_bits(|r| r | Self::MASK_TO_LP_RX_MASK);
    }

    /// Clear MASK_TO_LP_RX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_to_lp_rx(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_TO_LP_RX_MASK);
    }

    /// **MASK_TO_HS_TX** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_to_hs_tx(&self) -> bool {
        (self.value.read_bits() & Self::MASK_TO_HS_TX_MASK) != 0
    }

    /// Set MASK_TO_HS_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_to_hs_tx(&self) {
        self.value.modify_bits(|r| r | Self::MASK_TO_HS_TX_MASK);
    }

    /// Clear MASK_TO_HS_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_to_hs_tx(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_TO_HS_TX_MASK);
    }
}

/// INT_MSK1 value type for batch read/modify/write operations
///
/// Use `intmsk1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intmsk1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntMsk1Val {
    bits: u32,
}

impl IntMsk1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MASK_DPI_BUFF_PLD_UNDER** `[19:19]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_dpi_buff_pld_under(&self) -> bool {
        (self.bits & IntMsk1::MASK_DPI_BUFF_PLD_UNDER_MASK) != 0
    }

    /// Read **MASK_GEN_PLD_RECEV_ERR** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_gen_pld_recev_err(&self) -> bool {
        (self.bits & IntMsk1::MASK_GEN_PLD_RECEV_ERR_MASK) != 0
    }

    /// Read **MASK_GEN_PLD_RD_ERR** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_gen_pld_rd_err(&self) -> bool {
        (self.bits & IntMsk1::MASK_GEN_PLD_RD_ERR_MASK) != 0
    }

    /// Read **MASK_GEN_PLD_SEND_ERR** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_gen_pld_send_err(&self) -> bool {
        (self.bits & IntMsk1::MASK_GEN_PLD_SEND_ERR_MASK) != 0
    }

    /// Read **MASK_GEN_PLD_WR_ERR** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_gen_pld_wr_err(&self) -> bool {
        (self.bits & IntMsk1::MASK_GEN_PLD_WR_ERR_MASK) != 0
    }

    /// Read **MASK_GEN_CMD_WR_ERR** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_gen_cmd_wr_err(&self) -> bool {
        (self.bits & IntMsk1::MASK_GEN_CMD_WR_ERR_MASK) != 0
    }

    /// Read **MASK_DPI_PLD_WR_ERR** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_dpi_pld_wr_err(&self) -> bool {
        (self.bits & IntMsk1::MASK_DPI_PLD_WR_ERR_MASK) != 0
    }

    /// Read **MASK_EOPT_ERR** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_eopt_err(&self) -> bool {
        (self.bits & IntMsk1::MASK_EOPT_ERR_MASK) != 0
    }

    /// Read **MASK_PKT_SIZE_ERR** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_pkt_size_err(&self) -> bool {
        (self.bits & IntMsk1::MASK_PKT_SIZE_ERR_MASK) != 0
    }

    /// Read **MASK_CRC_ERR** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_crc_err(&self) -> bool {
        (self.bits & IntMsk1::MASK_CRC_ERR_MASK) != 0
    }

    /// Read **MASK_ECC_MILTI_ERR** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ecc_milti_err(&self) -> bool {
        (self.bits & IntMsk1::MASK_ECC_MILTI_ERR_MASK) != 0
    }

    /// Read **MASK_ECC_SINGLE_ERR** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_ecc_single_err(&self) -> bool {
        (self.bits & IntMsk1::MASK_ECC_SINGLE_ERR_MASK) != 0
    }

    /// Read **MASK_TO_LP_RX** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_to_lp_rx(&self) -> bool {
        (self.bits & IntMsk1::MASK_TO_LP_RX_MASK) != 0
    }

    /// Read **MASK_TO_HS_TX** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_to_hs_tx(&self) -> bool {
        (self.bits & IntMsk1::MASK_TO_HS_TX_MASK) != 0
    }

    /// Set **MASK_DPI_BUFF_PLD_UNDER** `[19:19]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_dpi_buff_pld_under(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk1::MASK_DPI_BUFF_PLD_UNDER_MASK
            } else {
                self.bits & !IntMsk1::MASK_DPI_BUFF_PLD_UNDER_MASK
            },
        }
    }

    /// Set **MASK_GEN_PLD_RECEV_ERR** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_gen_pld_recev_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk1::MASK_GEN_PLD_RECEV_ERR_MASK
            } else {
                self.bits & !IntMsk1::MASK_GEN_PLD_RECEV_ERR_MASK
            },
        }
    }

    /// Set **MASK_GEN_PLD_RD_ERR** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_gen_pld_rd_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk1::MASK_GEN_PLD_RD_ERR_MASK
            } else {
                self.bits & !IntMsk1::MASK_GEN_PLD_RD_ERR_MASK
            },
        }
    }

    /// Set **MASK_GEN_PLD_SEND_ERR** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_gen_pld_send_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk1::MASK_GEN_PLD_SEND_ERR_MASK
            } else {
                self.bits & !IntMsk1::MASK_GEN_PLD_SEND_ERR_MASK
            },
        }
    }

    /// Set **MASK_GEN_PLD_WR_ERR** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_gen_pld_wr_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk1::MASK_GEN_PLD_WR_ERR_MASK
            } else {
                self.bits & !IntMsk1::MASK_GEN_PLD_WR_ERR_MASK
            },
        }
    }

    /// Set **MASK_GEN_CMD_WR_ERR** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_gen_cmd_wr_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk1::MASK_GEN_CMD_WR_ERR_MASK
            } else {
                self.bits & !IntMsk1::MASK_GEN_CMD_WR_ERR_MASK
            },
        }
    }

    /// Set **MASK_DPI_PLD_WR_ERR** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_dpi_pld_wr_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk1::MASK_DPI_PLD_WR_ERR_MASK
            } else {
                self.bits & !IntMsk1::MASK_DPI_PLD_WR_ERR_MASK
            },
        }
    }

    /// Set **MASK_EOPT_ERR** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_eopt_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk1::MASK_EOPT_ERR_MASK
            } else {
                self.bits & !IntMsk1::MASK_EOPT_ERR_MASK
            },
        }
    }

    /// Set **MASK_PKT_SIZE_ERR** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_pkt_size_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk1::MASK_PKT_SIZE_ERR_MASK
            } else {
                self.bits & !IntMsk1::MASK_PKT_SIZE_ERR_MASK
            },
        }
    }

    /// Set **MASK_CRC_ERR** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_crc_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk1::MASK_CRC_ERR_MASK
            } else {
                self.bits & !IntMsk1::MASK_CRC_ERR_MASK
            },
        }
    }

    /// Set **MASK_ECC_MILTI_ERR** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ecc_milti_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk1::MASK_ECC_MILTI_ERR_MASK
            } else {
                self.bits & !IntMsk1::MASK_ECC_MILTI_ERR_MASK
            },
        }
    }

    /// Set **MASK_ECC_SINGLE_ERR** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_ecc_single_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk1::MASK_ECC_SINGLE_ERR_MASK
            } else {
                self.bits & !IntMsk1::MASK_ECC_SINGLE_ERR_MASK
            },
        }
    }

    /// Set **MASK_TO_LP_RX** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_to_lp_rx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk1::MASK_TO_LP_RX_MASK
            } else {
                self.bits & !IntMsk1::MASK_TO_LP_RX_MASK
            },
        }
    }

    /// Set **MASK_TO_HS_TX** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_to_hs_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMsk1::MASK_TO_HS_TX_MASK
            } else {
                self.bits & !IntMsk1::MASK_TO_HS_TX_MASK
            },
        }
    }
}

impl Default for IntMsk1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntMsk1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntMsk1Val {
        IntMsk1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntMsk1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntMsk1Val) -> IntMsk1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_CAL Register
///
/// NA
///
/// Address offset: 0x00cc
///
/// ## Fields
///
/// - **TXSKEWCALHS** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PhyCal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyCal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyCal {
    // ======== Field Constants ========

    /// **TXSKEWCALHS**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TXSKEWCALHS_OFFSET: u32 = 0;
    pub const TXSKEWCALHS_MASK: u32 = 0x00000001;
    pub const TXSKEWCALHS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TXSKEWCALHS** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn txskewcalhs(&self) -> bool {
        (self.value.read_bits() & Self::TXSKEWCALHS_MASK) != 0
    }

    /// Set TXSKEWCALHS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_txskewcalhs(&self) {
        self.value.modify_bits(|r| r | Self::TXSKEWCALHS_MASK);
    }

    /// Clear TXSKEWCALHS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_txskewcalhs(&self) {
        self.value.modify_bits(|r| r & !Self::TXSKEWCALHS_MASK);
    }
}

/// PHY_CAL value type for batch read/modify/write operations
///
/// Use `phycal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phycal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyCalVal {
    bits: u32,
}

impl PhyCalVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TXSKEWCALHS** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn txskewcalhs(&self) -> bool {
        (self.bits & PhyCal::TXSKEWCALHS_MASK) != 0
    }

    /// Set **TXSKEWCALHS** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_txskewcalhs(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyCal::TXSKEWCALHS_MASK
            } else {
                self.bits & !PhyCal::TXSKEWCALHS_MASK
            },
        }
    }
}

impl Default for PhyCalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PhyCal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyCalVal {
        PhyCalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyCalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyCalVal) -> PhyCalVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_FORCE0 Register
///
/// NA
///
/// Address offset: 0x00d8
///
/// ## Fields
///
/// - **FORCE_DPHY_ERRORS_4** `[20]` (RW)
///
///   NA
///
/// - **FORCE_DPHY_ERRORS_3** `[19]` (RW)
///
///   NA
///
/// - **FORCE_DPHY_ERRORS_2** `[18]` (RW)
///
///   NA
///
/// - **FORCE_DPHY_ERRORS_1** `[17]` (RW)
///
///   NA
///
/// - **FORCE_DPHY_ERRORS_0** `[16]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_15** `[15]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_14** `[14]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_13** `[13]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_12** `[12]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_11** `[11]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_10** `[10]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_9** `[9]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_8** `[8]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_7** `[7]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_6** `[6]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_5** `[5]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_4** `[4]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_3** `[3]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_2** `[2]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_1** `[1]` (RW)
///
///   NA
///
/// - **FORCE_ACK_WITH_ERR_0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntForce0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntForce0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntForce0 {
    // ======== Field Constants ========

    /// **FORCE_DPHY_ERRORS_4**
    ///
    /// NA
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const FORCE_DPHY_ERRORS_4_OFFSET: u32 = 20;
    pub const FORCE_DPHY_ERRORS_4_MASK: u32 = 0x00100000;
    pub const FORCE_DPHY_ERRORS_4: u32 = 1 << 20;

    /// **FORCE_DPHY_ERRORS_3**
    ///
    /// NA
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const FORCE_DPHY_ERRORS_3_OFFSET: u32 = 19;
    pub const FORCE_DPHY_ERRORS_3_MASK: u32 = 0x00080000;
    pub const FORCE_DPHY_ERRORS_3: u32 = 1 << 19;

    /// **FORCE_DPHY_ERRORS_2**
    ///
    /// NA
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const FORCE_DPHY_ERRORS_2_OFFSET: u32 = 18;
    pub const FORCE_DPHY_ERRORS_2_MASK: u32 = 0x00040000;
    pub const FORCE_DPHY_ERRORS_2: u32 = 1 << 18;

    /// **FORCE_DPHY_ERRORS_1**
    ///
    /// NA
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const FORCE_DPHY_ERRORS_1_OFFSET: u32 = 17;
    pub const FORCE_DPHY_ERRORS_1_MASK: u32 = 0x00020000;
    pub const FORCE_DPHY_ERRORS_1: u32 = 1 << 17;

    /// **FORCE_DPHY_ERRORS_0**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const FORCE_DPHY_ERRORS_0_OFFSET: u32 = 16;
    pub const FORCE_DPHY_ERRORS_0_MASK: u32 = 0x00010000;
    pub const FORCE_DPHY_ERRORS_0: u32 = 1 << 16;

    /// **FORCE_ACK_WITH_ERR_15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const FORCE_ACK_WITH_ERR_15_OFFSET: u32 = 15;
    pub const FORCE_ACK_WITH_ERR_15_MASK: u32 = 0x00008000;
    pub const FORCE_ACK_WITH_ERR_15: u32 = 1 << 15;

    /// **FORCE_ACK_WITH_ERR_14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const FORCE_ACK_WITH_ERR_14_OFFSET: u32 = 14;
    pub const FORCE_ACK_WITH_ERR_14_MASK: u32 = 0x00004000;
    pub const FORCE_ACK_WITH_ERR_14: u32 = 1 << 14;

    /// **FORCE_ACK_WITH_ERR_13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const FORCE_ACK_WITH_ERR_13_OFFSET: u32 = 13;
    pub const FORCE_ACK_WITH_ERR_13_MASK: u32 = 0x00002000;
    pub const FORCE_ACK_WITH_ERR_13: u32 = 1 << 13;

    /// **FORCE_ACK_WITH_ERR_12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const FORCE_ACK_WITH_ERR_12_OFFSET: u32 = 12;
    pub const FORCE_ACK_WITH_ERR_12_MASK: u32 = 0x00001000;
    pub const FORCE_ACK_WITH_ERR_12: u32 = 1 << 12;

    /// **FORCE_ACK_WITH_ERR_11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const FORCE_ACK_WITH_ERR_11_OFFSET: u32 = 11;
    pub const FORCE_ACK_WITH_ERR_11_MASK: u32 = 0x00000800;
    pub const FORCE_ACK_WITH_ERR_11: u32 = 1 << 11;

    /// **FORCE_ACK_WITH_ERR_10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const FORCE_ACK_WITH_ERR_10_OFFSET: u32 = 10;
    pub const FORCE_ACK_WITH_ERR_10_MASK: u32 = 0x00000400;
    pub const FORCE_ACK_WITH_ERR_10: u32 = 1 << 10;

    /// **FORCE_ACK_WITH_ERR_9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const FORCE_ACK_WITH_ERR_9_OFFSET: u32 = 9;
    pub const FORCE_ACK_WITH_ERR_9_MASK: u32 = 0x00000200;
    pub const FORCE_ACK_WITH_ERR_9: u32 = 1 << 9;

    /// **FORCE_ACK_WITH_ERR_8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const FORCE_ACK_WITH_ERR_8_OFFSET: u32 = 8;
    pub const FORCE_ACK_WITH_ERR_8_MASK: u32 = 0x00000100;
    pub const FORCE_ACK_WITH_ERR_8: u32 = 1 << 8;

    /// **FORCE_ACK_WITH_ERR_7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const FORCE_ACK_WITH_ERR_7_OFFSET: u32 = 7;
    pub const FORCE_ACK_WITH_ERR_7_MASK: u32 = 0x00000080;
    pub const FORCE_ACK_WITH_ERR_7: u32 = 1 << 7;

    /// **FORCE_ACK_WITH_ERR_6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const FORCE_ACK_WITH_ERR_6_OFFSET: u32 = 6;
    pub const FORCE_ACK_WITH_ERR_6_MASK: u32 = 0x00000040;
    pub const FORCE_ACK_WITH_ERR_6: u32 = 1 << 6;

    /// **FORCE_ACK_WITH_ERR_5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const FORCE_ACK_WITH_ERR_5_OFFSET: u32 = 5;
    pub const FORCE_ACK_WITH_ERR_5_MASK: u32 = 0x00000020;
    pub const FORCE_ACK_WITH_ERR_5: u32 = 1 << 5;

    /// **FORCE_ACK_WITH_ERR_4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const FORCE_ACK_WITH_ERR_4_OFFSET: u32 = 4;
    pub const FORCE_ACK_WITH_ERR_4_MASK: u32 = 0x00000010;
    pub const FORCE_ACK_WITH_ERR_4: u32 = 1 << 4;

    /// **FORCE_ACK_WITH_ERR_3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const FORCE_ACK_WITH_ERR_3_OFFSET: u32 = 3;
    pub const FORCE_ACK_WITH_ERR_3_MASK: u32 = 0x00000008;
    pub const FORCE_ACK_WITH_ERR_3: u32 = 1 << 3;

    /// **FORCE_ACK_WITH_ERR_2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FORCE_ACK_WITH_ERR_2_OFFSET: u32 = 2;
    pub const FORCE_ACK_WITH_ERR_2_MASK: u32 = 0x00000004;
    pub const FORCE_ACK_WITH_ERR_2: u32 = 1 << 2;

    /// **FORCE_ACK_WITH_ERR_1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FORCE_ACK_WITH_ERR_1_OFFSET: u32 = 1;
    pub const FORCE_ACK_WITH_ERR_1_MASK: u32 = 0x00000002;
    pub const FORCE_ACK_WITH_ERR_1: u32 = 1 << 1;

    /// **FORCE_ACK_WITH_ERR_0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FORCE_ACK_WITH_ERR_0_OFFSET: u32 = 0;
    pub const FORCE_ACK_WITH_ERR_0_MASK: u32 = 0x00000001;
    pub const FORCE_ACK_WITH_ERR_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FORCE_DPHY_ERRORS_4** `[20]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_dphy_errors_4(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_DPHY_ERRORS_4_MASK) != 0
    }

    /// Set FORCE_DPHY_ERRORS_4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_dphy_errors_4(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_DPHY_ERRORS_4_MASK);
    }

    /// Clear FORCE_DPHY_ERRORS_4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_dphy_errors_4(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_DPHY_ERRORS_4_MASK);
    }

    /// **FORCE_DPHY_ERRORS_3** `[19]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_dphy_errors_3(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_DPHY_ERRORS_3_MASK) != 0
    }

    /// Set FORCE_DPHY_ERRORS_3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_dphy_errors_3(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_DPHY_ERRORS_3_MASK);
    }

    /// Clear FORCE_DPHY_ERRORS_3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_dphy_errors_3(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_DPHY_ERRORS_3_MASK);
    }

    /// **FORCE_DPHY_ERRORS_2** `[18]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_dphy_errors_2(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_DPHY_ERRORS_2_MASK) != 0
    }

    /// Set FORCE_DPHY_ERRORS_2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_dphy_errors_2(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_DPHY_ERRORS_2_MASK);
    }

    /// Clear FORCE_DPHY_ERRORS_2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_dphy_errors_2(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_DPHY_ERRORS_2_MASK);
    }

    /// **FORCE_DPHY_ERRORS_1** `[17]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_dphy_errors_1(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_DPHY_ERRORS_1_MASK) != 0
    }

    /// Set FORCE_DPHY_ERRORS_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_dphy_errors_1(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_DPHY_ERRORS_1_MASK);
    }

    /// Clear FORCE_DPHY_ERRORS_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_dphy_errors_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_DPHY_ERRORS_1_MASK);
    }

    /// **FORCE_DPHY_ERRORS_0** `[16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_dphy_errors_0(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_DPHY_ERRORS_0_MASK) != 0
    }

    /// Set FORCE_DPHY_ERRORS_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_dphy_errors_0(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_DPHY_ERRORS_0_MASK);
    }

    /// Clear FORCE_DPHY_ERRORS_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_dphy_errors_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_DPHY_ERRORS_0_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_15** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_15(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_15_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_15(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_15_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_15(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_15_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_14** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_14(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_14_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_14(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_14_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_14(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_14_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_13** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_13(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_13_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_13(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_13_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_13(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_13_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_12** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_12(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_12_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_12(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_12_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_12(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_12_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_11** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_11(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_11_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_11(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_11_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_11(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_11_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_10** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_10(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_10_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_10(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_10_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_10(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_10_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_9** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_9(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_9_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_9(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_9_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_9(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_9_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_8** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_8(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_8_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_8(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_8_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_8(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_8_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_7** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_7(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_7_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_7(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_7_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_7(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_7_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_6** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_6(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_6_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_6(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_6_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_6(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_6_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_5** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_5(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_5_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_5(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_5_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_5(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_5_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_4** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_4(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_4_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_4(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_4_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_4(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_4_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_3** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_3(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_3_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_3(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_3_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_3(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_3_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_2** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_2(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_2_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_2(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_2_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_2(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_2_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_1(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_1_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_1(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_1_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_1_MASK);
    }

    /// **FORCE_ACK_WITH_ERR_0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ack_with_err_0(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ACK_WITH_ERR_0_MASK) != 0
    }

    /// Set FORCE_ACK_WITH_ERR_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ack_with_err_0(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ACK_WITH_ERR_0_MASK);
    }

    /// Clear FORCE_ACK_WITH_ERR_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ack_with_err_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ACK_WITH_ERR_0_MASK);
    }
}

/// INT_FORCE0 value type for batch read/modify/write operations
///
/// Use `intforce0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intforce0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntForce0Val {
    bits: u32,
}

impl IntForce0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FORCE_DPHY_ERRORS_4** `[20:20]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_dphy_errors_4(&self) -> bool {
        (self.bits & IntForce0::FORCE_DPHY_ERRORS_4_MASK) != 0
    }

    /// Read **FORCE_DPHY_ERRORS_3** `[19:19]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_dphy_errors_3(&self) -> bool {
        (self.bits & IntForce0::FORCE_DPHY_ERRORS_3_MASK) != 0
    }

    /// Read **FORCE_DPHY_ERRORS_2** `[18:18]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_dphy_errors_2(&self) -> bool {
        (self.bits & IntForce0::FORCE_DPHY_ERRORS_2_MASK) != 0
    }

    /// Read **FORCE_DPHY_ERRORS_1** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_dphy_errors_1(&self) -> bool {
        (self.bits & IntForce0::FORCE_DPHY_ERRORS_1_MASK) != 0
    }

    /// Read **FORCE_DPHY_ERRORS_0** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_dphy_errors_0(&self) -> bool {
        (self.bits & IntForce0::FORCE_DPHY_ERRORS_0_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_15(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_15_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_14(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_14_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_13(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_13_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_12(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_12_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_11(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_11_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_10(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_10_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_9(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_9_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_8(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_8_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_7(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_7_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_6(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_6_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_5(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_5_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_4(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_4_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_3(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_3_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_2(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_2_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_1(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_1_MASK) != 0
    }

    /// Read **FORCE_ACK_WITH_ERR_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ack_with_err_0(&self) -> bool {
        (self.bits & IntForce0::FORCE_ACK_WITH_ERR_0_MASK) != 0
    }

    /// Set **FORCE_DPHY_ERRORS_4** `[20:20]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_dphy_errors_4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_DPHY_ERRORS_4_MASK
            } else {
                self.bits & !IntForce0::FORCE_DPHY_ERRORS_4_MASK
            },
        }
    }

    /// Set **FORCE_DPHY_ERRORS_3** `[19:19]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_dphy_errors_3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_DPHY_ERRORS_3_MASK
            } else {
                self.bits & !IntForce0::FORCE_DPHY_ERRORS_3_MASK
            },
        }
    }

    /// Set **FORCE_DPHY_ERRORS_2** `[18:18]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_dphy_errors_2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_DPHY_ERRORS_2_MASK
            } else {
                self.bits & !IntForce0::FORCE_DPHY_ERRORS_2_MASK
            },
        }
    }

    /// Set **FORCE_DPHY_ERRORS_1** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_dphy_errors_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_DPHY_ERRORS_1_MASK
            } else {
                self.bits & !IntForce0::FORCE_DPHY_ERRORS_1_MASK
            },
        }
    }

    /// Set **FORCE_DPHY_ERRORS_0** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_dphy_errors_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_DPHY_ERRORS_0_MASK
            } else {
                self.bits & !IntForce0::FORCE_DPHY_ERRORS_0_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_15_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_15_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_14_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_14_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_13_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_13_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_12_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_12_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_11_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_11_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_10_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_10_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_9_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_9_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_8_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_8_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_7_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_7_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_6_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_6_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_5_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_5_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_4_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_4_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_3_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_3_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_2_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_2_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_1_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_1_MASK
            },
        }
    }

    /// Set **FORCE_ACK_WITH_ERR_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ack_with_err_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce0::FORCE_ACK_WITH_ERR_0_MASK
            } else {
                self.bits & !IntForce0::FORCE_ACK_WITH_ERR_0_MASK
            },
        }
    }
}

impl Default for IntForce0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntForce0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntForce0Val {
        IntForce0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntForce0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntForce0Val) -> IntForce0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_FORCE1 Register
///
/// NA
///
/// Address offset: 0x00dc
///
/// ## Fields
///
/// - **FORCE_DPI_BUFF_PLD_UNDER** `[19]` (RW)
///
///   NA
///
/// - **FORCE_GEN_PLD_RECEV_ERR** `[12]` (RW)
///
///   NA
///
/// - **FORCE_GEN_PLD_RD_ERR** `[11]` (RW)
///
///   NA
///
/// - **FORCE_GEN_PLD_SEND_ERR** `[10]` (RW)
///
///   NA
///
/// - **FORCE_GEN_PLD_WR_ERR** `[9]` (RW)
///
///   NA
///
/// - **FORCE_GEN_CMD_WR_ERR** `[8]` (RW)
///
///   NA
///
/// - **FORCE_DPI_PLD_WR_ERR** `[7]` (RW)
///
///   NA
///
/// - **FORCE_EOPT_ERR** `[6]` (RW)
///
///   NA
///
/// - **FORCE_PKT_SIZE_ERR** `[5]` (RW)
///
///   NA
///
/// - **FORCE_CRC_ERR** `[4]` (RW)
///
///   NA
///
/// - **FORCE_ECC_MILTI_ERR** `[3]` (RW)
///
///   NA
///
/// - **FORCE_ECC_SINGLE_ERR** `[2]` (RW)
///
///   NA
///
/// - **FORCE_TO_LP_RX** `[1]` (RW)
///
///   NA
///
/// - **FORCE_TO_HS_TX** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntForce1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntForce1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntForce1 {
    // ======== Field Constants ========

    /// **FORCE_DPI_BUFF_PLD_UNDER**
    ///
    /// NA
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const FORCE_DPI_BUFF_PLD_UNDER_OFFSET: u32 = 19;
    pub const FORCE_DPI_BUFF_PLD_UNDER_MASK: u32 = 0x00080000;
    pub const FORCE_DPI_BUFF_PLD_UNDER: u32 = 1 << 19;

    /// **FORCE_GEN_PLD_RECEV_ERR**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const FORCE_GEN_PLD_RECEV_ERR_OFFSET: u32 = 12;
    pub const FORCE_GEN_PLD_RECEV_ERR_MASK: u32 = 0x00001000;
    pub const FORCE_GEN_PLD_RECEV_ERR: u32 = 1 << 12;

    /// **FORCE_GEN_PLD_RD_ERR**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const FORCE_GEN_PLD_RD_ERR_OFFSET: u32 = 11;
    pub const FORCE_GEN_PLD_RD_ERR_MASK: u32 = 0x00000800;
    pub const FORCE_GEN_PLD_RD_ERR: u32 = 1 << 11;

    /// **FORCE_GEN_PLD_SEND_ERR**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const FORCE_GEN_PLD_SEND_ERR_OFFSET: u32 = 10;
    pub const FORCE_GEN_PLD_SEND_ERR_MASK: u32 = 0x00000400;
    pub const FORCE_GEN_PLD_SEND_ERR: u32 = 1 << 10;

    /// **FORCE_GEN_PLD_WR_ERR**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const FORCE_GEN_PLD_WR_ERR_OFFSET: u32 = 9;
    pub const FORCE_GEN_PLD_WR_ERR_MASK: u32 = 0x00000200;
    pub const FORCE_GEN_PLD_WR_ERR: u32 = 1 << 9;

    /// **FORCE_GEN_CMD_WR_ERR**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const FORCE_GEN_CMD_WR_ERR_OFFSET: u32 = 8;
    pub const FORCE_GEN_CMD_WR_ERR_MASK: u32 = 0x00000100;
    pub const FORCE_GEN_CMD_WR_ERR: u32 = 1 << 8;

    /// **FORCE_DPI_PLD_WR_ERR**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const FORCE_DPI_PLD_WR_ERR_OFFSET: u32 = 7;
    pub const FORCE_DPI_PLD_WR_ERR_MASK: u32 = 0x00000080;
    pub const FORCE_DPI_PLD_WR_ERR: u32 = 1 << 7;

    /// **FORCE_EOPT_ERR**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const FORCE_EOPT_ERR_OFFSET: u32 = 6;
    pub const FORCE_EOPT_ERR_MASK: u32 = 0x00000040;
    pub const FORCE_EOPT_ERR: u32 = 1 << 6;

    /// **FORCE_PKT_SIZE_ERR**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const FORCE_PKT_SIZE_ERR_OFFSET: u32 = 5;
    pub const FORCE_PKT_SIZE_ERR_MASK: u32 = 0x00000020;
    pub const FORCE_PKT_SIZE_ERR: u32 = 1 << 5;

    /// **FORCE_CRC_ERR**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const FORCE_CRC_ERR_OFFSET: u32 = 4;
    pub const FORCE_CRC_ERR_MASK: u32 = 0x00000010;
    pub const FORCE_CRC_ERR: u32 = 1 << 4;

    /// **FORCE_ECC_MILTI_ERR**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const FORCE_ECC_MILTI_ERR_OFFSET: u32 = 3;
    pub const FORCE_ECC_MILTI_ERR_MASK: u32 = 0x00000008;
    pub const FORCE_ECC_MILTI_ERR: u32 = 1 << 3;

    /// **FORCE_ECC_SINGLE_ERR**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FORCE_ECC_SINGLE_ERR_OFFSET: u32 = 2;
    pub const FORCE_ECC_SINGLE_ERR_MASK: u32 = 0x00000004;
    pub const FORCE_ECC_SINGLE_ERR: u32 = 1 << 2;

    /// **FORCE_TO_LP_RX**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FORCE_TO_LP_RX_OFFSET: u32 = 1;
    pub const FORCE_TO_LP_RX_MASK: u32 = 0x00000002;
    pub const FORCE_TO_LP_RX: u32 = 1 << 1;

    /// **FORCE_TO_HS_TX**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FORCE_TO_HS_TX_OFFSET: u32 = 0;
    pub const FORCE_TO_HS_TX_MASK: u32 = 0x00000001;
    pub const FORCE_TO_HS_TX: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FORCE_DPI_BUFF_PLD_UNDER** `[19]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_dpi_buff_pld_under(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_DPI_BUFF_PLD_UNDER_MASK) != 0
    }

    /// Set FORCE_DPI_BUFF_PLD_UNDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_dpi_buff_pld_under(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_DPI_BUFF_PLD_UNDER_MASK);
    }

    /// Clear FORCE_DPI_BUFF_PLD_UNDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_dpi_buff_pld_under(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_DPI_BUFF_PLD_UNDER_MASK);
    }

    /// **FORCE_GEN_PLD_RECEV_ERR** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_gen_pld_recev_err(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_GEN_PLD_RECEV_ERR_MASK) != 0
    }

    /// Set FORCE_GEN_PLD_RECEV_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_gen_pld_recev_err(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_GEN_PLD_RECEV_ERR_MASK);
    }

    /// Clear FORCE_GEN_PLD_RECEV_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_gen_pld_recev_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_GEN_PLD_RECEV_ERR_MASK);
    }

    /// **FORCE_GEN_PLD_RD_ERR** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_gen_pld_rd_err(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_GEN_PLD_RD_ERR_MASK) != 0
    }

    /// Set FORCE_GEN_PLD_RD_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_gen_pld_rd_err(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_GEN_PLD_RD_ERR_MASK);
    }

    /// Clear FORCE_GEN_PLD_RD_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_gen_pld_rd_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_GEN_PLD_RD_ERR_MASK);
    }

    /// **FORCE_GEN_PLD_SEND_ERR** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_gen_pld_send_err(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_GEN_PLD_SEND_ERR_MASK) != 0
    }

    /// Set FORCE_GEN_PLD_SEND_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_gen_pld_send_err(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_GEN_PLD_SEND_ERR_MASK);
    }

    /// Clear FORCE_GEN_PLD_SEND_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_gen_pld_send_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_GEN_PLD_SEND_ERR_MASK);
    }

    /// **FORCE_GEN_PLD_WR_ERR** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_gen_pld_wr_err(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_GEN_PLD_WR_ERR_MASK) != 0
    }

    /// Set FORCE_GEN_PLD_WR_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_gen_pld_wr_err(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_GEN_PLD_WR_ERR_MASK);
    }

    /// Clear FORCE_GEN_PLD_WR_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_gen_pld_wr_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_GEN_PLD_WR_ERR_MASK);
    }

    /// **FORCE_GEN_CMD_WR_ERR** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_gen_cmd_wr_err(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_GEN_CMD_WR_ERR_MASK) != 0
    }

    /// Set FORCE_GEN_CMD_WR_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_gen_cmd_wr_err(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_GEN_CMD_WR_ERR_MASK);
    }

    /// Clear FORCE_GEN_CMD_WR_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_gen_cmd_wr_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_GEN_CMD_WR_ERR_MASK);
    }

    /// **FORCE_DPI_PLD_WR_ERR** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_dpi_pld_wr_err(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_DPI_PLD_WR_ERR_MASK) != 0
    }

    /// Set FORCE_DPI_PLD_WR_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_dpi_pld_wr_err(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_DPI_PLD_WR_ERR_MASK);
    }

    /// Clear FORCE_DPI_PLD_WR_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_dpi_pld_wr_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_DPI_PLD_WR_ERR_MASK);
    }

    /// **FORCE_EOPT_ERR** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_eopt_err(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_EOPT_ERR_MASK) != 0
    }

    /// Set FORCE_EOPT_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_eopt_err(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_EOPT_ERR_MASK);
    }

    /// Clear FORCE_EOPT_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_eopt_err(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_EOPT_ERR_MASK);
    }

    /// **FORCE_PKT_SIZE_ERR** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_pkt_size_err(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_PKT_SIZE_ERR_MASK) != 0
    }

    /// Set FORCE_PKT_SIZE_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_pkt_size_err(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_PKT_SIZE_ERR_MASK);
    }

    /// Clear FORCE_PKT_SIZE_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_pkt_size_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_PKT_SIZE_ERR_MASK);
    }

    /// **FORCE_CRC_ERR** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_crc_err(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_CRC_ERR_MASK) != 0
    }

    /// Set FORCE_CRC_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_crc_err(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_CRC_ERR_MASK);
    }

    /// Clear FORCE_CRC_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_crc_err(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_CRC_ERR_MASK);
    }

    /// **FORCE_ECC_MILTI_ERR** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ecc_milti_err(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ECC_MILTI_ERR_MASK) != 0
    }

    /// Set FORCE_ECC_MILTI_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ecc_milti_err(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ECC_MILTI_ERR_MASK);
    }

    /// Clear FORCE_ECC_MILTI_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ecc_milti_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ECC_MILTI_ERR_MASK);
    }

    /// **FORCE_ECC_SINGLE_ERR** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_ecc_single_err(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ECC_SINGLE_ERR_MASK) != 0
    }

    /// Set FORCE_ECC_SINGLE_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_ecc_single_err(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ECC_SINGLE_ERR_MASK);
    }

    /// Clear FORCE_ECC_SINGLE_ERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_ecc_single_err(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ECC_SINGLE_ERR_MASK);
    }

    /// **FORCE_TO_LP_RX** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_to_lp_rx(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_TO_LP_RX_MASK) != 0
    }

    /// Set FORCE_TO_LP_RX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_to_lp_rx(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_TO_LP_RX_MASK);
    }

    /// Clear FORCE_TO_LP_RX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_to_lp_rx(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_TO_LP_RX_MASK);
    }

    /// **FORCE_TO_HS_TX** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_to_hs_tx(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_TO_HS_TX_MASK) != 0
    }

    /// Set FORCE_TO_HS_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_to_hs_tx(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_TO_HS_TX_MASK);
    }

    /// Clear FORCE_TO_HS_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_to_hs_tx(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_TO_HS_TX_MASK);
    }
}

/// INT_FORCE1 value type for batch read/modify/write operations
///
/// Use `intforce1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intforce1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntForce1Val {
    bits: u32,
}

impl IntForce1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FORCE_DPI_BUFF_PLD_UNDER** `[19:19]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_dpi_buff_pld_under(&self) -> bool {
        (self.bits & IntForce1::FORCE_DPI_BUFF_PLD_UNDER_MASK) != 0
    }

    /// Read **FORCE_GEN_PLD_RECEV_ERR** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_gen_pld_recev_err(&self) -> bool {
        (self.bits & IntForce1::FORCE_GEN_PLD_RECEV_ERR_MASK) != 0
    }

    /// Read **FORCE_GEN_PLD_RD_ERR** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_gen_pld_rd_err(&self) -> bool {
        (self.bits & IntForce1::FORCE_GEN_PLD_RD_ERR_MASK) != 0
    }

    /// Read **FORCE_GEN_PLD_SEND_ERR** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_gen_pld_send_err(&self) -> bool {
        (self.bits & IntForce1::FORCE_GEN_PLD_SEND_ERR_MASK) != 0
    }

    /// Read **FORCE_GEN_PLD_WR_ERR** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_gen_pld_wr_err(&self) -> bool {
        (self.bits & IntForce1::FORCE_GEN_PLD_WR_ERR_MASK) != 0
    }

    /// Read **FORCE_GEN_CMD_WR_ERR** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_gen_cmd_wr_err(&self) -> bool {
        (self.bits & IntForce1::FORCE_GEN_CMD_WR_ERR_MASK) != 0
    }

    /// Read **FORCE_DPI_PLD_WR_ERR** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_dpi_pld_wr_err(&self) -> bool {
        (self.bits & IntForce1::FORCE_DPI_PLD_WR_ERR_MASK) != 0
    }

    /// Read **FORCE_EOPT_ERR** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_eopt_err(&self) -> bool {
        (self.bits & IntForce1::FORCE_EOPT_ERR_MASK) != 0
    }

    /// Read **FORCE_PKT_SIZE_ERR** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_pkt_size_err(&self) -> bool {
        (self.bits & IntForce1::FORCE_PKT_SIZE_ERR_MASK) != 0
    }

    /// Read **FORCE_CRC_ERR** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_crc_err(&self) -> bool {
        (self.bits & IntForce1::FORCE_CRC_ERR_MASK) != 0
    }

    /// Read **FORCE_ECC_MILTI_ERR** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ecc_milti_err(&self) -> bool {
        (self.bits & IntForce1::FORCE_ECC_MILTI_ERR_MASK) != 0
    }

    /// Read **FORCE_ECC_SINGLE_ERR** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_ecc_single_err(&self) -> bool {
        (self.bits & IntForce1::FORCE_ECC_SINGLE_ERR_MASK) != 0
    }

    /// Read **FORCE_TO_LP_RX** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_to_lp_rx(&self) -> bool {
        (self.bits & IntForce1::FORCE_TO_LP_RX_MASK) != 0
    }

    /// Read **FORCE_TO_HS_TX** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_to_hs_tx(&self) -> bool {
        (self.bits & IntForce1::FORCE_TO_HS_TX_MASK) != 0
    }

    /// Set **FORCE_DPI_BUFF_PLD_UNDER** `[19:19]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_dpi_buff_pld_under(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce1::FORCE_DPI_BUFF_PLD_UNDER_MASK
            } else {
                self.bits & !IntForce1::FORCE_DPI_BUFF_PLD_UNDER_MASK
            },
        }
    }

    /// Set **FORCE_GEN_PLD_RECEV_ERR** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_gen_pld_recev_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce1::FORCE_GEN_PLD_RECEV_ERR_MASK
            } else {
                self.bits & !IntForce1::FORCE_GEN_PLD_RECEV_ERR_MASK
            },
        }
    }

    /// Set **FORCE_GEN_PLD_RD_ERR** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_gen_pld_rd_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce1::FORCE_GEN_PLD_RD_ERR_MASK
            } else {
                self.bits & !IntForce1::FORCE_GEN_PLD_RD_ERR_MASK
            },
        }
    }

    /// Set **FORCE_GEN_PLD_SEND_ERR** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_gen_pld_send_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce1::FORCE_GEN_PLD_SEND_ERR_MASK
            } else {
                self.bits & !IntForce1::FORCE_GEN_PLD_SEND_ERR_MASK
            },
        }
    }

    /// Set **FORCE_GEN_PLD_WR_ERR** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_gen_pld_wr_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce1::FORCE_GEN_PLD_WR_ERR_MASK
            } else {
                self.bits & !IntForce1::FORCE_GEN_PLD_WR_ERR_MASK
            },
        }
    }

    /// Set **FORCE_GEN_CMD_WR_ERR** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_gen_cmd_wr_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce1::FORCE_GEN_CMD_WR_ERR_MASK
            } else {
                self.bits & !IntForce1::FORCE_GEN_CMD_WR_ERR_MASK
            },
        }
    }

    /// Set **FORCE_DPI_PLD_WR_ERR** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_dpi_pld_wr_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce1::FORCE_DPI_PLD_WR_ERR_MASK
            } else {
                self.bits & !IntForce1::FORCE_DPI_PLD_WR_ERR_MASK
            },
        }
    }

    /// Set **FORCE_EOPT_ERR** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_eopt_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce1::FORCE_EOPT_ERR_MASK
            } else {
                self.bits & !IntForce1::FORCE_EOPT_ERR_MASK
            },
        }
    }

    /// Set **FORCE_PKT_SIZE_ERR** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_pkt_size_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce1::FORCE_PKT_SIZE_ERR_MASK
            } else {
                self.bits & !IntForce1::FORCE_PKT_SIZE_ERR_MASK
            },
        }
    }

    /// Set **FORCE_CRC_ERR** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_crc_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce1::FORCE_CRC_ERR_MASK
            } else {
                self.bits & !IntForce1::FORCE_CRC_ERR_MASK
            },
        }
    }

    /// Set **FORCE_ECC_MILTI_ERR** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ecc_milti_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce1::FORCE_ECC_MILTI_ERR_MASK
            } else {
                self.bits & !IntForce1::FORCE_ECC_MILTI_ERR_MASK
            },
        }
    }

    /// Set **FORCE_ECC_SINGLE_ERR** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_ecc_single_err(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce1::FORCE_ECC_SINGLE_ERR_MASK
            } else {
                self.bits & !IntForce1::FORCE_ECC_SINGLE_ERR_MASK
            },
        }
    }

    /// Set **FORCE_TO_LP_RX** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_to_lp_rx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce1::FORCE_TO_LP_RX_MASK
            } else {
                self.bits & !IntForce1::FORCE_TO_LP_RX_MASK
            },
        }
    }

    /// Set **FORCE_TO_HS_TX** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_to_hs_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForce1::FORCE_TO_HS_TX_MASK
            } else {
                self.bits & !IntForce1::FORCE_TO_HS_TX_MASK
            },
        }
    }
}

impl Default for IntForce1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntForce1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntForce1Val {
        IntForce1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntForce1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntForce1Val) -> IntForce1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DSC_PARAMETER Register
///
/// NA
///
/// Address offset: 0x00f0
///
/// ## Fields
///
/// - **PPS_SEL** `[17:16]` (RW)
///
///   NA
///
/// - **COMPRESS_ALGO** `[9:8]` (RW)
///
///   NA
///
/// - **COMPRESSION_MODE** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DscParameter {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DscParameter {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DscParameter {
    // ======== Field Constants ========

    /// **PPS_SEL**
    ///
    /// NA
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const PPS_SEL_OFFSET: u32 = 16;
    pub const PPS_SEL_MASK: u32 = 0x00030000;

    /// **COMPRESS_ALGO**
    ///
    /// NA
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const COMPRESS_ALGO_OFFSET: u32 = 8;
    pub const COMPRESS_ALGO_MASK: u32 = 0x00000300;

    /// **COMPRESSION_MODE**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const COMPRESSION_MODE_OFFSET: u32 = 0;
    pub const COMPRESSION_MODE_MASK: u32 = 0x00000001;
    pub const COMPRESSION_MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PPS_SEL** `[17:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn pps_sel(&self) -> u32 {
        (self.value.read_bits() & Self::PPS_SEL_MASK) >> Self::PPS_SEL_OFFSET
    }

    /// Write **PPS_SEL** `[17:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_pps_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PPS_SEL_MASK) | ((value << Self::PPS_SEL_OFFSET) & Self::PPS_SEL_MASK)
        });
    }

    /// **COMPRESS_ALGO** `[9:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn compress_algo(&self) -> u32 {
        (self.value.read_bits() & Self::COMPRESS_ALGO_MASK) >> Self::COMPRESS_ALGO_OFFSET
    }

    /// Write **COMPRESS_ALGO** `[9:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_compress_algo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COMPRESS_ALGO_MASK)
                | ((value << Self::COMPRESS_ALGO_OFFSET) & Self::COMPRESS_ALGO_MASK)
        });
    }

    /// **COMPRESSION_MODE** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn compression_mode(&self) -> bool {
        (self.value.read_bits() & Self::COMPRESSION_MODE_MASK) != 0
    }

    /// Set COMPRESSION_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_compression_mode(&self) {
        self.value.modify_bits(|r| r | Self::COMPRESSION_MODE_MASK);
    }

    /// Clear COMPRESSION_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_compression_mode(&self) {
        self.value.modify_bits(|r| r & !Self::COMPRESSION_MODE_MASK);
    }
}

/// DSC_PARAMETER value type for batch read/modify/write operations
///
/// Use `dscparameter.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dscparameter.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DscParameterVal {
    bits: u32,
}

impl DscParameterVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PPS_SEL** `[17:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn pps_sel(&self) -> u32 {
        (self.bits & DscParameter::PPS_SEL_MASK) >> DscParameter::PPS_SEL_OFFSET
    }

    /// Read **COMPRESS_ALGO** `[9:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn compress_algo(&self) -> u32 {
        (self.bits & DscParameter::COMPRESS_ALGO_MASK) >> DscParameter::COMPRESS_ALGO_OFFSET
    }

    /// Read **COMPRESSION_MODE** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn compression_mode(&self) -> bool {
        (self.bits & DscParameter::COMPRESSION_MODE_MASK) != 0
    }

    /// Set **PPS_SEL** `[17:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_pps_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DscParameter::PPS_SEL_MASK)
                | ((value << DscParameter::PPS_SEL_OFFSET) & DscParameter::PPS_SEL_MASK),
        }
    }

    /// Set **COMPRESS_ALGO** `[9:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_compress_algo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DscParameter::COMPRESS_ALGO_MASK)
                | ((value << DscParameter::COMPRESS_ALGO_OFFSET)
                    & DscParameter::COMPRESS_ALGO_MASK),
        }
    }

    /// Set **COMPRESSION_MODE** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_compression_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DscParameter::COMPRESSION_MODE_MASK
            } else {
                self.bits & !DscParameter::COMPRESSION_MODE_MASK
            },
        }
    }
}

impl Default for DscParameterVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DscParameter {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DscParameterVal {
        DscParameterVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DscParameterVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DscParameterVal) -> DscParameterVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_TMR_RD_CFG Register
///
/// NA
///
/// Address offset: 0x00f4
///
/// ## Fields
///
/// - **MAX_RD_TIME** `[14:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PhyTmrRdCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyTmrRdCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyTmrRdCfg {
    // ======== Field Constants ========

    /// **MAX_RD_TIME**
    ///
    /// NA
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const MAX_RD_TIME_OFFSET: u32 = 0;
    pub const MAX_RD_TIME_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **MAX_RD_TIME** `[14:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn max_rd_time(&self) -> u32 {
        (self.value.read_bits() & Self::MAX_RD_TIME_MASK) >> Self::MAX_RD_TIME_OFFSET
    }

    /// Write **MAX_RD_TIME** `[14:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_max_rd_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAX_RD_TIME_MASK)
                | ((value << Self::MAX_RD_TIME_OFFSET) & Self::MAX_RD_TIME_MASK)
        });
    }
}

/// PHY_TMR_RD_CFG value type for batch read/modify/write operations
///
/// Use `phytmrrdcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phytmrrdcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyTmrRdCfgVal {
    bits: u32,
}

impl PhyTmrRdCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAX_RD_TIME** `[14:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn max_rd_time(&self) -> u32 {
        (self.bits & PhyTmrRdCfg::MAX_RD_TIME_MASK) >> PhyTmrRdCfg::MAX_RD_TIME_OFFSET
    }

    /// Set **MAX_RD_TIME** `[14:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_max_rd_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyTmrRdCfg::MAX_RD_TIME_MASK)
                | ((value << PhyTmrRdCfg::MAX_RD_TIME_OFFSET) & PhyTmrRdCfg::MAX_RD_TIME_MASK),
        }
    }
}

impl Default for PhyTmrRdCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PhyTmrRdCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyTmrRdCfgVal {
        PhyTmrRdCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyTmrRdCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyTmrRdCfgVal) -> PhyTmrRdCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VID_SHADOW_CTRL Register
///
/// NA
///
/// Address offset: 0x0100
///
/// ## Fields
///
/// - **VID_SHADOW_PIN_REQ** `[16]` (RW)
///
///   NA
///
/// - **VID_SHADOW_REQ** `[8]` (RW)
///
///   NA
///
/// - **VID_SHADOW_EN** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct VidShadowCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidShadowCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidShadowCtrl {
    // ======== Field Constants ========

    /// **VID_SHADOW_PIN_REQ**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const VID_SHADOW_PIN_REQ_OFFSET: u32 = 16;
    pub const VID_SHADOW_PIN_REQ_MASK: u32 = 0x00010000;
    pub const VID_SHADOW_PIN_REQ: u32 = 1 << 16;

    /// **VID_SHADOW_REQ**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const VID_SHADOW_REQ_OFFSET: u32 = 8;
    pub const VID_SHADOW_REQ_MASK: u32 = 0x00000100;
    pub const VID_SHADOW_REQ: u32 = 1 << 8;

    /// **VID_SHADOW_EN**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VID_SHADOW_EN_OFFSET: u32 = 0;
    pub const VID_SHADOW_EN_MASK: u32 = 0x00000001;
    pub const VID_SHADOW_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **VID_SHADOW_PIN_REQ** `[16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_shadow_pin_req(&self) -> bool {
        (self.value.read_bits() & Self::VID_SHADOW_PIN_REQ_MASK) != 0
    }

    /// Set VID_SHADOW_PIN_REQ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vid_shadow_pin_req(&self) {
        self.value
            .modify_bits(|r| r | Self::VID_SHADOW_PIN_REQ_MASK);
    }

    /// Clear VID_SHADOW_PIN_REQ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vid_shadow_pin_req(&self) {
        self.value
            .modify_bits(|r| r & !Self::VID_SHADOW_PIN_REQ_MASK);
    }

    /// **VID_SHADOW_REQ** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_shadow_req(&self) -> bool {
        (self.value.read_bits() & Self::VID_SHADOW_REQ_MASK) != 0
    }

    /// Set VID_SHADOW_REQ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vid_shadow_req(&self) {
        self.value.modify_bits(|r| r | Self::VID_SHADOW_REQ_MASK);
    }

    /// Clear VID_SHADOW_REQ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vid_shadow_req(&self) {
        self.value.modify_bits(|r| r & !Self::VID_SHADOW_REQ_MASK);
    }

    /// **VID_SHADOW_EN** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_shadow_en(&self) -> bool {
        (self.value.read_bits() & Self::VID_SHADOW_EN_MASK) != 0
    }

    /// Set VID_SHADOW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vid_shadow_en(&self) {
        self.value.modify_bits(|r| r | Self::VID_SHADOW_EN_MASK);
    }

    /// Clear VID_SHADOW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vid_shadow_en(&self) {
        self.value.modify_bits(|r| r & !Self::VID_SHADOW_EN_MASK);
    }
}

/// VID_SHADOW_CTRL value type for batch read/modify/write operations
///
/// Use `vidshadowctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidshadowctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidShadowCtrlVal {
    bits: u32,
}

impl VidShadowCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VID_SHADOW_PIN_REQ** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_shadow_pin_req(&self) -> bool {
        (self.bits & VidShadowCtrl::VID_SHADOW_PIN_REQ_MASK) != 0
    }

    /// Read **VID_SHADOW_REQ** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_shadow_req(&self) -> bool {
        (self.bits & VidShadowCtrl::VID_SHADOW_REQ_MASK) != 0
    }

    /// Read **VID_SHADOW_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_shadow_en(&self) -> bool {
        (self.bits & VidShadowCtrl::VID_SHADOW_EN_MASK) != 0
    }

    /// Set **VID_SHADOW_PIN_REQ** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vid_shadow_pin_req(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VidShadowCtrl::VID_SHADOW_PIN_REQ_MASK
            } else {
                self.bits & !VidShadowCtrl::VID_SHADOW_PIN_REQ_MASK
            },
        }
    }

    /// Set **VID_SHADOW_REQ** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vid_shadow_req(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VidShadowCtrl::VID_SHADOW_REQ_MASK
            } else {
                self.bits & !VidShadowCtrl::VID_SHADOW_REQ_MASK
            },
        }
    }

    /// Set **VID_SHADOW_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vid_shadow_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VidShadowCtrl::VID_SHADOW_EN_MASK
            } else {
                self.bits & !VidShadowCtrl::VID_SHADOW_EN_MASK
            },
        }
    }
}

impl Default for VidShadowCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidShadowCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidShadowCtrlVal {
        VidShadowCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VidShadowCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VidShadowCtrlVal) -> VidShadowCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DPI_VCID_ACT Register
///
/// NA
///
/// Address offset: 0x010c
///
/// ## Fields
///
/// - **DPI_VCID_ACT** `[1:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DpiVcidAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiVcidAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiVcidAct {
    // ======== Field Constants ========

    /// **DPI_VCID_ACT**
    ///
    /// NA
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const DPI_VCID_ACT_OFFSET: u32 = 0;
    pub const DPI_VCID_ACT_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **DPI_VCID_ACT** `[1:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dpi_vcid_act(&self) -> u32 {
        (self.value.read_bits() & Self::DPI_VCID_ACT_MASK) >> Self::DPI_VCID_ACT_OFFSET
    }
}

/// DPI_VCID_ACT value type for batch read/modify/write operations
///
/// Use `dpivcidact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpivcidact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiVcidActVal {
    bits: u32,
}

impl DpiVcidActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DPI_VCID_ACT** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dpi_vcid_act(&self) -> u32 {
        (self.bits & DpiVcidAct::DPI_VCID_ACT_MASK) >> DpiVcidAct::DPI_VCID_ACT_OFFSET
    }
}

impl Default for DpiVcidActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DpiVcidAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DpiVcidActVal {
        DpiVcidActVal::from_bits(self.value.read_bits())
    }
}

/// DPI_COLOR_CODING_ACT Register
///
/// NA
///
/// Address offset: 0x0110
///
/// ## Fields
///
/// - **LOOSELY18_EN_ACT** `[8]` (RO)
///
///   NA
///
/// - **DPI_COLOR_CODING_ACT** `[3:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DpiColorCodingAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiColorCodingAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiColorCodingAct {
    // ======== Field Constants ========

    /// **LOOSELY18_EN_ACT**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const LOOSELY18_EN_ACT_OFFSET: u32 = 8;
    pub const LOOSELY18_EN_ACT_MASK: u32 = 0x00000100;
    pub const LOOSELY18_EN_ACT: u32 = 1 << 8;

    /// **DPI_COLOR_CODING_ACT**
    ///
    /// NA
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const DPI_COLOR_CODING_ACT_OFFSET: u32 = 0;
    pub const DPI_COLOR_CODING_ACT_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **LOOSELY18_EN_ACT** `[8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn loosely18_en_act(&self) -> bool {
        (self.value.read_bits() & Self::LOOSELY18_EN_ACT_MASK) != 0
    }

    /// **DPI_COLOR_CODING_ACT** `[3:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dpi_color_coding_act(&self) -> u32 {
        (self.value.read_bits() & Self::DPI_COLOR_CODING_ACT_MASK)
            >> Self::DPI_COLOR_CODING_ACT_OFFSET
    }
}

/// DPI_COLOR_CODING_ACT value type for batch read/modify/write operations
///
/// Use `dpicolorcodingact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpicolorcodingact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiColorCodingActVal {
    bits: u32,
}

impl DpiColorCodingActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LOOSELY18_EN_ACT** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn loosely18_en_act(&self) -> bool {
        (self.bits & DpiColorCodingAct::LOOSELY18_EN_ACT_MASK) != 0
    }

    /// Read **DPI_COLOR_CODING_ACT** `[3:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dpi_color_coding_act(&self) -> u32 {
        (self.bits & DpiColorCodingAct::DPI_COLOR_CODING_ACT_MASK)
            >> DpiColorCodingAct::DPI_COLOR_CODING_ACT_OFFSET
    }
}

impl Default for DpiColorCodingActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DpiColorCodingAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DpiColorCodingActVal {
        DpiColorCodingActVal::from_bits(self.value.read_bits())
    }
}

/// DPI_LP_CMD_TIM_ACT Register
///
/// NA
///
/// Address offset: 0x0118
///
/// ## Fields
///
/// - **OUTVACT_LPCMD_TIME_ACT** `[23:16]` (RO)
///
///   NA
///
/// - **INVACT_LPCMD_TIME_ACT** `[7:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct DpiLpCmdTimAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiLpCmdTimAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiLpCmdTimAct {
    // ======== Field Constants ========

    /// **OUTVACT_LPCMD_TIME_ACT**
    ///
    /// NA
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const OUTVACT_LPCMD_TIME_ACT_OFFSET: u32 = 16;
    pub const OUTVACT_LPCMD_TIME_ACT_MASK: u32 = 0x00ff0000;

    /// **INVACT_LPCMD_TIME_ACT**
    ///
    /// NA
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const INVACT_LPCMD_TIME_ACT_OFFSET: u32 = 0;
    pub const INVACT_LPCMD_TIME_ACT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **OUTVACT_LPCMD_TIME_ACT** `[23:16]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn outvact_lpcmd_time_act(&self) -> u32 {
        (self.value.read_bits() & Self::OUTVACT_LPCMD_TIME_ACT_MASK)
            >> Self::OUTVACT_LPCMD_TIME_ACT_OFFSET
    }

    /// **INVACT_LPCMD_TIME_ACT** `[7:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn invact_lpcmd_time_act(&self) -> u32 {
        (self.value.read_bits() & Self::INVACT_LPCMD_TIME_ACT_MASK)
            >> Self::INVACT_LPCMD_TIME_ACT_OFFSET
    }
}

/// DPI_LP_CMD_TIM_ACT value type for batch read/modify/write operations
///
/// Use `dpilpcmdtimact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpilpcmdtimact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiLpCmdTimActVal {
    bits: u32,
}

impl DpiLpCmdTimActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **OUTVACT_LPCMD_TIME_ACT** `[23:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn outvact_lpcmd_time_act(&self) -> u32 {
        (self.bits & DpiLpCmdTimAct::OUTVACT_LPCMD_TIME_ACT_MASK)
            >> DpiLpCmdTimAct::OUTVACT_LPCMD_TIME_ACT_OFFSET
    }

    /// Read **INVACT_LPCMD_TIME_ACT** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn invact_lpcmd_time_act(&self) -> u32 {
        (self.bits & DpiLpCmdTimAct::INVACT_LPCMD_TIME_ACT_MASK)
            >> DpiLpCmdTimAct::INVACT_LPCMD_TIME_ACT_OFFSET
    }
}

impl Default for DpiLpCmdTimActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DpiLpCmdTimAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DpiLpCmdTimActVal {
        DpiLpCmdTimActVal::from_bits(self.value.read_bits())
    }
}

/// EDPI_TE_HW_CFG Register
///
/// NA
///
/// Address offset: 0x011c
///
/// ## Fields
///
/// - **SCAN_LINE_PARAMETER** `[31:16]` (RW)
///
///   NA
///
/// - **HW_SET_SCAN_LINE** `[4]` (RW)
///
///   NA
///
/// - **HW_TEAR_EFFECT_GEN** `[1]` (RW)
///
///   NA
///
/// - **HW_TEAR_EFFECT_ON** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct EdpiTeHwCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EdpiTeHwCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EdpiTeHwCfg {
    // ======== Field Constants ========

    /// **SCAN_LINE_PARAMETER**
    ///
    /// NA
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const SCAN_LINE_PARAMETER_OFFSET: u32 = 16;
    pub const SCAN_LINE_PARAMETER_MASK: u32 = 0xffff0000;

    /// **HW_SET_SCAN_LINE**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const HW_SET_SCAN_LINE_OFFSET: u32 = 4;
    pub const HW_SET_SCAN_LINE_MASK: u32 = 0x00000010;
    pub const HW_SET_SCAN_LINE: u32 = 1 << 4;

    /// **HW_TEAR_EFFECT_GEN**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const HW_TEAR_EFFECT_GEN_OFFSET: u32 = 1;
    pub const HW_TEAR_EFFECT_GEN_MASK: u32 = 0x00000002;
    pub const HW_TEAR_EFFECT_GEN: u32 = 1 << 1;

    /// **HW_TEAR_EFFECT_ON**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const HW_TEAR_EFFECT_ON_OFFSET: u32 = 0;
    pub const HW_TEAR_EFFECT_ON_MASK: u32 = 0x00000001;
    pub const HW_TEAR_EFFECT_ON: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SCAN_LINE_PARAMETER** `[31:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn scan_line_parameter(&self) -> u32 {
        (self.value.read_bits() & Self::SCAN_LINE_PARAMETER_MASK)
            >> Self::SCAN_LINE_PARAMETER_OFFSET
    }

    /// Write **SCAN_LINE_PARAMETER** `[31:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_scan_line_parameter(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCAN_LINE_PARAMETER_MASK)
                | ((value << Self::SCAN_LINE_PARAMETER_OFFSET) & Self::SCAN_LINE_PARAMETER_MASK)
        });
    }

    /// **HW_SET_SCAN_LINE** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn hw_set_scan_line(&self) -> bool {
        (self.value.read_bits() & Self::HW_SET_SCAN_LINE_MASK) != 0
    }

    /// Set HW_SET_SCAN_LINE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_hw_set_scan_line(&self) {
        self.value.modify_bits(|r| r | Self::HW_SET_SCAN_LINE_MASK);
    }

    /// Clear HW_SET_SCAN_LINE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_hw_set_scan_line(&self) {
        self.value.modify_bits(|r| r & !Self::HW_SET_SCAN_LINE_MASK);
    }

    /// **HW_TEAR_EFFECT_GEN** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn hw_tear_effect_gen(&self) -> bool {
        (self.value.read_bits() & Self::HW_TEAR_EFFECT_GEN_MASK) != 0
    }

    /// Set HW_TEAR_EFFECT_GEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_hw_tear_effect_gen(&self) {
        self.value
            .modify_bits(|r| r | Self::HW_TEAR_EFFECT_GEN_MASK);
    }

    /// Clear HW_TEAR_EFFECT_GEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_hw_tear_effect_gen(&self) {
        self.value
            .modify_bits(|r| r & !Self::HW_TEAR_EFFECT_GEN_MASK);
    }

    /// **HW_TEAR_EFFECT_ON** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn hw_tear_effect_on(&self) -> bool {
        (self.value.read_bits() & Self::HW_TEAR_EFFECT_ON_MASK) != 0
    }

    /// Set HW_TEAR_EFFECT_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_hw_tear_effect_on(&self) {
        self.value.modify_bits(|r| r | Self::HW_TEAR_EFFECT_ON_MASK);
    }

    /// Clear HW_TEAR_EFFECT_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_hw_tear_effect_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::HW_TEAR_EFFECT_ON_MASK);
    }
}

/// EDPI_TE_HW_CFG value type for batch read/modify/write operations
///
/// Use `edpitehwcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `edpitehwcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EdpiTeHwCfgVal {
    bits: u32,
}

impl EdpiTeHwCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SCAN_LINE_PARAMETER** `[31:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn scan_line_parameter(&self) -> u32 {
        (self.bits & EdpiTeHwCfg::SCAN_LINE_PARAMETER_MASK)
            >> EdpiTeHwCfg::SCAN_LINE_PARAMETER_OFFSET
    }

    /// Read **HW_SET_SCAN_LINE** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn hw_set_scan_line(&self) -> bool {
        (self.bits & EdpiTeHwCfg::HW_SET_SCAN_LINE_MASK) != 0
    }

    /// Read **HW_TEAR_EFFECT_GEN** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn hw_tear_effect_gen(&self) -> bool {
        (self.bits & EdpiTeHwCfg::HW_TEAR_EFFECT_GEN_MASK) != 0
    }

    /// Read **HW_TEAR_EFFECT_ON** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn hw_tear_effect_on(&self) -> bool {
        (self.bits & EdpiTeHwCfg::HW_TEAR_EFFECT_ON_MASK) != 0
    }

    /// Set **SCAN_LINE_PARAMETER** `[31:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_scan_line_parameter(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EdpiTeHwCfg::SCAN_LINE_PARAMETER_MASK)
                | ((value << EdpiTeHwCfg::SCAN_LINE_PARAMETER_OFFSET)
                    & EdpiTeHwCfg::SCAN_LINE_PARAMETER_MASK),
        }
    }

    /// Set **HW_SET_SCAN_LINE** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_hw_set_scan_line(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EdpiTeHwCfg::HW_SET_SCAN_LINE_MASK
            } else {
                self.bits & !EdpiTeHwCfg::HW_SET_SCAN_LINE_MASK
            },
        }
    }

    /// Set **HW_TEAR_EFFECT_GEN** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_hw_tear_effect_gen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EdpiTeHwCfg::HW_TEAR_EFFECT_GEN_MASK
            } else {
                self.bits & !EdpiTeHwCfg::HW_TEAR_EFFECT_GEN_MASK
            },
        }
    }

    /// Set **HW_TEAR_EFFECT_ON** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_hw_tear_effect_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EdpiTeHwCfg::HW_TEAR_EFFECT_ON_MASK
            } else {
                self.bits & !EdpiTeHwCfg::HW_TEAR_EFFECT_ON_MASK
            },
        }
    }
}

impl Default for EdpiTeHwCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EdpiTeHwCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EdpiTeHwCfgVal {
        EdpiTeHwCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EdpiTeHwCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EdpiTeHwCfgVal) -> EdpiTeHwCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VID_MODE_CFG_ACT Register
///
/// NA
///
/// Address offset: 0x0138
///
/// ## Fields
///
/// - **LP_CMD_EN_ACT** `[9]` (RO)
///
///   NA
///
/// - **FRAME_BTA_ACK_EN_ACT** `[8]` (RO)
///
///   NA
///
/// - **LP_HFP_EN_ACT** `[7]` (RO)
///
///   NA
///
/// - **LP_HBP_EN_ACT** `[6]` (RO)
///
///   NA
///
/// - **LP_VACT_EN_ACT** `[5]` (RO)
///
///   NA
///
/// - **LP_VFP_EN_ACT** `[4]` (RO)
///
///   NA
///
/// - **LP_VBP_EN_ACT** `[3]` (RO)
///
///   NA
///
/// - **LP_VSA_EN_ACT** `[2]` (RO)
///
///   NA
///
/// - **VID_MODE_TYPE_ACT** `[1:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct VidModeCfgAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidModeCfgAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidModeCfgAct {
    // ======== Field Constants ========

    /// **LP_CMD_EN_ACT**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const LP_CMD_EN_ACT_OFFSET: u32 = 9;
    pub const LP_CMD_EN_ACT_MASK: u32 = 0x00000200;
    pub const LP_CMD_EN_ACT: u32 = 1 << 9;

    /// **FRAME_BTA_ACK_EN_ACT**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const FRAME_BTA_ACK_EN_ACT_OFFSET: u32 = 8;
    pub const FRAME_BTA_ACK_EN_ACT_MASK: u32 = 0x00000100;
    pub const FRAME_BTA_ACK_EN_ACT: u32 = 1 << 8;

    /// **LP_HFP_EN_ACT**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const LP_HFP_EN_ACT_OFFSET: u32 = 7;
    pub const LP_HFP_EN_ACT_MASK: u32 = 0x00000080;
    pub const LP_HFP_EN_ACT: u32 = 1 << 7;

    /// **LP_HBP_EN_ACT**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const LP_HBP_EN_ACT_OFFSET: u32 = 6;
    pub const LP_HBP_EN_ACT_MASK: u32 = 0x00000040;
    pub const LP_HBP_EN_ACT: u32 = 1 << 6;

    /// **LP_VACT_EN_ACT**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const LP_VACT_EN_ACT_OFFSET: u32 = 5;
    pub const LP_VACT_EN_ACT_MASK: u32 = 0x00000020;
    pub const LP_VACT_EN_ACT: u32 = 1 << 5;

    /// **LP_VFP_EN_ACT**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_VFP_EN_ACT_OFFSET: u32 = 4;
    pub const LP_VFP_EN_ACT_MASK: u32 = 0x00000010;
    pub const LP_VFP_EN_ACT: u32 = 1 << 4;

    /// **LP_VBP_EN_ACT**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LP_VBP_EN_ACT_OFFSET: u32 = 3;
    pub const LP_VBP_EN_ACT_MASK: u32 = 0x00000008;
    pub const LP_VBP_EN_ACT: u32 = 1 << 3;

    /// **LP_VSA_EN_ACT**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const LP_VSA_EN_ACT_OFFSET: u32 = 2;
    pub const LP_VSA_EN_ACT_MASK: u32 = 0x00000004;
    pub const LP_VSA_EN_ACT: u32 = 1 << 2;

    /// **VID_MODE_TYPE_ACT**
    ///
    /// NA
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const VID_MODE_TYPE_ACT_OFFSET: u32 = 0;
    pub const VID_MODE_TYPE_ACT_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **LP_CMD_EN_ACT** `[9]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_cmd_en_act(&self) -> bool {
        (self.value.read_bits() & Self::LP_CMD_EN_ACT_MASK) != 0
    }

    /// **FRAME_BTA_ACK_EN_ACT** `[8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn frame_bta_ack_en_act(&self) -> bool {
        (self.value.read_bits() & Self::FRAME_BTA_ACK_EN_ACT_MASK) != 0
    }

    /// **LP_HFP_EN_ACT** `[7]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_hfp_en_act(&self) -> bool {
        (self.value.read_bits() & Self::LP_HFP_EN_ACT_MASK) != 0
    }

    /// **LP_HBP_EN_ACT** `[6]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_hbp_en_act(&self) -> bool {
        (self.value.read_bits() & Self::LP_HBP_EN_ACT_MASK) != 0
    }

    /// **LP_VACT_EN_ACT** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_vact_en_act(&self) -> bool {
        (self.value.read_bits() & Self::LP_VACT_EN_ACT_MASK) != 0
    }

    /// **LP_VFP_EN_ACT** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_vfp_en_act(&self) -> bool {
        (self.value.read_bits() & Self::LP_VFP_EN_ACT_MASK) != 0
    }

    /// **LP_VBP_EN_ACT** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_vbp_en_act(&self) -> bool {
        (self.value.read_bits() & Self::LP_VBP_EN_ACT_MASK) != 0
    }

    /// **LP_VSA_EN_ACT** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn lp_vsa_en_act(&self) -> bool {
        (self.value.read_bits() & Self::LP_VSA_EN_ACT_MASK) != 0
    }

    /// **VID_MODE_TYPE_ACT** `[1:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_mode_type_act(&self) -> u32 {
        (self.value.read_bits() & Self::VID_MODE_TYPE_ACT_MASK) >> Self::VID_MODE_TYPE_ACT_OFFSET
    }
}

/// VID_MODE_CFG_ACT value type for batch read/modify/write operations
///
/// Use `vidmodecfgact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidmodecfgact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidModeCfgActVal {
    bits: u32,
}

impl VidModeCfgActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_CMD_EN_ACT** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_cmd_en_act(&self) -> bool {
        (self.bits & VidModeCfgAct::LP_CMD_EN_ACT_MASK) != 0
    }

    /// Read **FRAME_BTA_ACK_EN_ACT** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn frame_bta_ack_en_act(&self) -> bool {
        (self.bits & VidModeCfgAct::FRAME_BTA_ACK_EN_ACT_MASK) != 0
    }

    /// Read **LP_HFP_EN_ACT** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_hfp_en_act(&self) -> bool {
        (self.bits & VidModeCfgAct::LP_HFP_EN_ACT_MASK) != 0
    }

    /// Read **LP_HBP_EN_ACT** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_hbp_en_act(&self) -> bool {
        (self.bits & VidModeCfgAct::LP_HBP_EN_ACT_MASK) != 0
    }

    /// Read **LP_VACT_EN_ACT** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_vact_en_act(&self) -> bool {
        (self.bits & VidModeCfgAct::LP_VACT_EN_ACT_MASK) != 0
    }

    /// Read **LP_VFP_EN_ACT** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_vfp_en_act(&self) -> bool {
        (self.bits & VidModeCfgAct::LP_VFP_EN_ACT_MASK) != 0
    }

    /// Read **LP_VBP_EN_ACT** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_vbp_en_act(&self) -> bool {
        (self.bits & VidModeCfgAct::LP_VBP_EN_ACT_MASK) != 0
    }

    /// Read **LP_VSA_EN_ACT** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn lp_vsa_en_act(&self) -> bool {
        (self.bits & VidModeCfgAct::LP_VSA_EN_ACT_MASK) != 0
    }

    /// Read **VID_MODE_TYPE_ACT** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_mode_type_act(&self) -> u32 {
        (self.bits & VidModeCfgAct::VID_MODE_TYPE_ACT_MASK)
            >> VidModeCfgAct::VID_MODE_TYPE_ACT_OFFSET
    }
}

impl Default for VidModeCfgActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidModeCfgAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidModeCfgActVal {
        VidModeCfgActVal::from_bits(self.value.read_bits())
    }
}

/// VID_PKT_SIZE_ACT Register
///
/// NA
///
/// Address offset: 0x013c
///
/// ## Fields
///
/// - **VID_PKT_SIZE_ACT** `[13:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct VidPktSizeAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidPktSizeAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidPktSizeAct {
    // ======== Field Constants ========

    /// **VID_PKT_SIZE_ACT**
    ///
    /// NA
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const VID_PKT_SIZE_ACT_OFFSET: u32 = 0;
    pub const VID_PKT_SIZE_ACT_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **VID_PKT_SIZE_ACT** `[13:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_pkt_size_act(&self) -> u32 {
        (self.value.read_bits() & Self::VID_PKT_SIZE_ACT_MASK) >> Self::VID_PKT_SIZE_ACT_OFFSET
    }
}

/// VID_PKT_SIZE_ACT value type for batch read/modify/write operations
///
/// Use `vidpktsizeact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidpktsizeact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidPktSizeActVal {
    bits: u32,
}

impl VidPktSizeActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VID_PKT_SIZE_ACT** `[13:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_pkt_size_act(&self) -> u32 {
        (self.bits & VidPktSizeAct::VID_PKT_SIZE_ACT_MASK) >> VidPktSizeAct::VID_PKT_SIZE_ACT_OFFSET
    }
}

impl Default for VidPktSizeActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidPktSizeAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidPktSizeActVal {
        VidPktSizeActVal::from_bits(self.value.read_bits())
    }
}

/// VID_NUM_CHUNKS_ACT Register
///
/// NA
///
/// Address offset: 0x0140
///
/// ## Fields
///
/// - **VID_NUM_CHUNKS_ACT** `[12:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct VidNumChunksAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidNumChunksAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidNumChunksAct {
    // ======== Field Constants ========

    /// **VID_NUM_CHUNKS_ACT**
    ///
    /// NA
    ///
    /// Bits `[12:0]`, mask `0x00001fff`
    pub const VID_NUM_CHUNKS_ACT_OFFSET: u32 = 0;
    pub const VID_NUM_CHUNKS_ACT_MASK: u32 = 0x00001fff;

    // ======== Field Accessors ========

    /// **VID_NUM_CHUNKS_ACT** `[12:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_num_chunks_act(&self) -> u32 {
        (self.value.read_bits() & Self::VID_NUM_CHUNKS_ACT_MASK) >> Self::VID_NUM_CHUNKS_ACT_OFFSET
    }
}

/// VID_NUM_CHUNKS_ACT value type for batch read/modify/write operations
///
/// Use `vidnumchunksact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidnumchunksact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidNumChunksActVal {
    bits: u32,
}

impl VidNumChunksActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VID_NUM_CHUNKS_ACT** `[12:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_num_chunks_act(&self) -> u32 {
        (self.bits & VidNumChunksAct::VID_NUM_CHUNKS_ACT_MASK)
            >> VidNumChunksAct::VID_NUM_CHUNKS_ACT_OFFSET
    }
}

impl Default for VidNumChunksActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidNumChunksAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidNumChunksActVal {
        VidNumChunksActVal::from_bits(self.value.read_bits())
    }
}

/// VID_NULL_SIZE_ACT Register
///
/// NA
///
/// Address offset: 0x0144
///
/// ## Fields
///
/// - **VID_NULL_SIZE_ACT** `[12:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct VidNullSizeAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidNullSizeAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidNullSizeAct {
    // ======== Field Constants ========

    /// **VID_NULL_SIZE_ACT**
    ///
    /// NA
    ///
    /// Bits `[12:0]`, mask `0x00001fff`
    pub const VID_NULL_SIZE_ACT_OFFSET: u32 = 0;
    pub const VID_NULL_SIZE_ACT_MASK: u32 = 0x00001fff;

    // ======== Field Accessors ========

    /// **VID_NULL_SIZE_ACT** `[12:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_null_size_act(&self) -> u32 {
        (self.value.read_bits() & Self::VID_NULL_SIZE_ACT_MASK) >> Self::VID_NULL_SIZE_ACT_OFFSET
    }
}

/// VID_NULL_SIZE_ACT value type for batch read/modify/write operations
///
/// Use `vidnullsizeact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidnullsizeact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidNullSizeActVal {
    bits: u32,
}

impl VidNullSizeActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VID_NULL_SIZE_ACT** `[12:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_null_size_act(&self) -> u32 {
        (self.bits & VidNullSizeAct::VID_NULL_SIZE_ACT_MASK)
            >> VidNullSizeAct::VID_NULL_SIZE_ACT_OFFSET
    }
}

impl Default for VidNullSizeActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidNullSizeAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidNullSizeActVal {
        VidNullSizeActVal::from_bits(self.value.read_bits())
    }
}

/// VID_HSA_TIME_ACT Register
///
/// NA
///
/// Address offset: 0x0148
///
/// ## Fields
///
/// - **VID_HSA_TIME_ACT** `[11:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct VidHsaTimeAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidHsaTimeAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidHsaTimeAct {
    // ======== Field Constants ========

    /// **VID_HSA_TIME_ACT**
    ///
    /// NA
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const VID_HSA_TIME_ACT_OFFSET: u32 = 0;
    pub const VID_HSA_TIME_ACT_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **VID_HSA_TIME_ACT** `[11:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_hsa_time_act(&self) -> u32 {
        (self.value.read_bits() & Self::VID_HSA_TIME_ACT_MASK) >> Self::VID_HSA_TIME_ACT_OFFSET
    }
}

/// VID_HSA_TIME_ACT value type for batch read/modify/write operations
///
/// Use `vidhsatimeact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidhsatimeact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidHsaTimeActVal {
    bits: u32,
}

impl VidHsaTimeActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VID_HSA_TIME_ACT** `[11:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_hsa_time_act(&self) -> u32 {
        (self.bits & VidHsaTimeAct::VID_HSA_TIME_ACT_MASK) >> VidHsaTimeAct::VID_HSA_TIME_ACT_OFFSET
    }
}

impl Default for VidHsaTimeActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidHsaTimeAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidHsaTimeActVal {
        VidHsaTimeActVal::from_bits(self.value.read_bits())
    }
}

/// VID_HBP_TIME_ACT Register
///
/// NA
///
/// Address offset: 0x014c
///
/// ## Fields
///
/// - **VID_HBP_TIME_ACT** `[11:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct VidHbpTimeAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidHbpTimeAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidHbpTimeAct {
    // ======== Field Constants ========

    /// **VID_HBP_TIME_ACT**
    ///
    /// NA
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const VID_HBP_TIME_ACT_OFFSET: u32 = 0;
    pub const VID_HBP_TIME_ACT_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **VID_HBP_TIME_ACT** `[11:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_hbp_time_act(&self) -> u32 {
        (self.value.read_bits() & Self::VID_HBP_TIME_ACT_MASK) >> Self::VID_HBP_TIME_ACT_OFFSET
    }
}

/// VID_HBP_TIME_ACT value type for batch read/modify/write operations
///
/// Use `vidhbptimeact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidhbptimeact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidHbpTimeActVal {
    bits: u32,
}

impl VidHbpTimeActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VID_HBP_TIME_ACT** `[11:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_hbp_time_act(&self) -> u32 {
        (self.bits & VidHbpTimeAct::VID_HBP_TIME_ACT_MASK) >> VidHbpTimeAct::VID_HBP_TIME_ACT_OFFSET
    }
}

impl Default for VidHbpTimeActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidHbpTimeAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidHbpTimeActVal {
        VidHbpTimeActVal::from_bits(self.value.read_bits())
    }
}

/// VID_HLINE_TIME_ACT Register
///
/// NA
///
/// Address offset: 0x0150
///
/// ## Fields
///
/// - **VID_HLINE_TIME_ACT** `[14:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct VidHlineTimeAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidHlineTimeAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidHlineTimeAct {
    // ======== Field Constants ========

    /// **VID_HLINE_TIME_ACT**
    ///
    /// NA
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const VID_HLINE_TIME_ACT_OFFSET: u32 = 0;
    pub const VID_HLINE_TIME_ACT_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **VID_HLINE_TIME_ACT** `[14:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn vid_hline_time_act(&self) -> u32 {
        (self.value.read_bits() & Self::VID_HLINE_TIME_ACT_MASK) >> Self::VID_HLINE_TIME_ACT_OFFSET
    }
}

/// VID_HLINE_TIME_ACT value type for batch read/modify/write operations
///
/// Use `vidhlinetimeact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidhlinetimeact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidHlineTimeActVal {
    bits: u32,
}

impl VidHlineTimeActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VID_HLINE_TIME_ACT** `[14:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vid_hline_time_act(&self) -> u32 {
        (self.bits & VidHlineTimeAct::VID_HLINE_TIME_ACT_MASK)
            >> VidHlineTimeAct::VID_HLINE_TIME_ACT_OFFSET
    }
}

impl Default for VidHlineTimeActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidHlineTimeAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidHlineTimeActVal {
        VidHlineTimeActVal::from_bits(self.value.read_bits())
    }
}

/// VID_VSA_LINES_ACT Register
///
/// NA
///
/// Address offset: 0x0154
///
/// ## Fields
///
/// - **VSA_LINES_ACT** `[9:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct VidVsaLinesAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidVsaLinesAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidVsaLinesAct {
    // ======== Field Constants ========

    /// **VSA_LINES_ACT**
    ///
    /// NA
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const VSA_LINES_ACT_OFFSET: u32 = 0;
    pub const VSA_LINES_ACT_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **VSA_LINES_ACT** `[9:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn vsa_lines_act(&self) -> u32 {
        (self.value.read_bits() & Self::VSA_LINES_ACT_MASK) >> Self::VSA_LINES_ACT_OFFSET
    }
}

/// VID_VSA_LINES_ACT value type for batch read/modify/write operations
///
/// Use `vidvsalinesact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidvsalinesact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidVsaLinesActVal {
    bits: u32,
}

impl VidVsaLinesActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VSA_LINES_ACT** `[9:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vsa_lines_act(&self) -> u32 {
        (self.bits & VidVsaLinesAct::VSA_LINES_ACT_MASK) >> VidVsaLinesAct::VSA_LINES_ACT_OFFSET
    }
}

impl Default for VidVsaLinesActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidVsaLinesAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidVsaLinesActVal {
        VidVsaLinesActVal::from_bits(self.value.read_bits())
    }
}

/// VID_VBP_LINES_ACT Register
///
/// NA
///
/// Address offset: 0x0158
///
/// ## Fields
///
/// - **VBP_LINES_ACT** `[9:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct VidVbpLinesAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidVbpLinesAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidVbpLinesAct {
    // ======== Field Constants ========

    /// **VBP_LINES_ACT**
    ///
    /// NA
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const VBP_LINES_ACT_OFFSET: u32 = 0;
    pub const VBP_LINES_ACT_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **VBP_LINES_ACT** `[9:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn vbp_lines_act(&self) -> u32 {
        (self.value.read_bits() & Self::VBP_LINES_ACT_MASK) >> Self::VBP_LINES_ACT_OFFSET
    }
}

/// VID_VBP_LINES_ACT value type for batch read/modify/write operations
///
/// Use `vidvbplinesact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidvbplinesact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidVbpLinesActVal {
    bits: u32,
}

impl VidVbpLinesActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VBP_LINES_ACT** `[9:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vbp_lines_act(&self) -> u32 {
        (self.bits & VidVbpLinesAct::VBP_LINES_ACT_MASK) >> VidVbpLinesAct::VBP_LINES_ACT_OFFSET
    }
}

impl Default for VidVbpLinesActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidVbpLinesAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidVbpLinesActVal {
        VidVbpLinesActVal::from_bits(self.value.read_bits())
    }
}

/// VID_VFP_LINES_ACT Register
///
/// NA
///
/// Address offset: 0x015c
///
/// ## Fields
///
/// - **VFP_LINES_ACT** `[9:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct VidVfpLinesAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidVfpLinesAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidVfpLinesAct {
    // ======== Field Constants ========

    /// **VFP_LINES_ACT**
    ///
    /// NA
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const VFP_LINES_ACT_OFFSET: u32 = 0;
    pub const VFP_LINES_ACT_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **VFP_LINES_ACT** `[9:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn vfp_lines_act(&self) -> u32 {
        (self.value.read_bits() & Self::VFP_LINES_ACT_MASK) >> Self::VFP_LINES_ACT_OFFSET
    }
}

/// VID_VFP_LINES_ACT value type for batch read/modify/write operations
///
/// Use `vidvfplinesact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidvfplinesact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidVfpLinesActVal {
    bits: u32,
}

impl VidVfpLinesActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **VFP_LINES_ACT** `[9:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vfp_lines_act(&self) -> u32 {
        (self.bits & VidVfpLinesAct::VFP_LINES_ACT_MASK) >> VidVfpLinesAct::VFP_LINES_ACT_OFFSET
    }
}

impl Default for VidVfpLinesActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidVfpLinesAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidVfpLinesActVal {
        VidVfpLinesActVal::from_bits(self.value.read_bits())
    }
}

/// VID_VACTIVE_LINES_ACT Register
///
/// NA
///
/// Address offset: 0x0160
///
/// ## Fields
///
/// - **V_ACTIVE_LINES_ACT** `[13:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct VidVactiveLinesAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidVactiveLinesAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidVactiveLinesAct {
    // ======== Field Constants ========

    /// **V_ACTIVE_LINES_ACT**
    ///
    /// NA
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const V_ACTIVE_LINES_ACT_OFFSET: u32 = 0;
    pub const V_ACTIVE_LINES_ACT_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **V_ACTIVE_LINES_ACT** `[13:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn v_active_lines_act(&self) -> u32 {
        (self.value.read_bits() & Self::V_ACTIVE_LINES_ACT_MASK) >> Self::V_ACTIVE_LINES_ACT_OFFSET
    }
}

/// VID_VACTIVE_LINES_ACT value type for batch read/modify/write operations
///
/// Use `vidvactivelinesact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidvactivelinesact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidVactiveLinesActVal {
    bits: u32,
}

impl VidVactiveLinesActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **V_ACTIVE_LINES_ACT** `[13:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn v_active_lines_act(&self) -> u32 {
        (self.bits & VidVactiveLinesAct::V_ACTIVE_LINES_ACT_MASK)
            >> VidVactiveLinesAct::V_ACTIVE_LINES_ACT_OFFSET
    }
}

impl Default for VidVactiveLinesActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VidVactiveLinesAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidVactiveLinesActVal {
        VidVactiveLinesActVal::from_bits(self.value.read_bits())
    }
}

/// VID_PKT_STATUS Register
///
/// NA
///
/// Address offset: 0x0168
/// Reset value: 0x00010005
///
/// ## Fields
///
/// - **DPI_BUFF_PLD_FULL** `[17]` (RO)
///
///   NA
///
/// - **DPI_BUFF_PLD_EMPTY** `[16]` (RO)
///
///   NA
///
/// - **DPI_PLD_W_FULL** `[3]` (RO)
///
///   NA
///
/// - **DPI_PLD_W_EMPTY** `[2]` (RO)
///
///   NA
///
/// - **DPI_CMD_W_FULL** `[1]` (RO)
///
///   NA
///
/// - **DPI_CMD_W_EMPTY** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct VidPktStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VidPktStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VidPktStatus {
    // ======== Field Constants ========

    /// **DPI_BUFF_PLD_FULL**
    ///
    /// NA
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const DPI_BUFF_PLD_FULL_OFFSET: u32 = 17;
    pub const DPI_BUFF_PLD_FULL_MASK: u32 = 0x00020000;
    pub const DPI_BUFF_PLD_FULL: u32 = 1 << 17;

    /// **DPI_BUFF_PLD_EMPTY**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const DPI_BUFF_PLD_EMPTY_OFFSET: u32 = 16;
    pub const DPI_BUFF_PLD_EMPTY_MASK: u32 = 0x00010000;
    pub const DPI_BUFF_PLD_EMPTY: u32 = 1 << 16;

    /// **DPI_PLD_W_FULL**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DPI_PLD_W_FULL_OFFSET: u32 = 3;
    pub const DPI_PLD_W_FULL_MASK: u32 = 0x00000008;
    pub const DPI_PLD_W_FULL: u32 = 1 << 3;

    /// **DPI_PLD_W_EMPTY**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DPI_PLD_W_EMPTY_OFFSET: u32 = 2;
    pub const DPI_PLD_W_EMPTY_MASK: u32 = 0x00000004;
    pub const DPI_PLD_W_EMPTY: u32 = 1 << 2;

    /// **DPI_CMD_W_FULL**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DPI_CMD_W_FULL_OFFSET: u32 = 1;
    pub const DPI_CMD_W_FULL_MASK: u32 = 0x00000002;
    pub const DPI_CMD_W_FULL: u32 = 1 << 1;

    /// **DPI_CMD_W_EMPTY**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DPI_CMD_W_EMPTY_OFFSET: u32 = 0;
    pub const DPI_CMD_W_EMPTY_MASK: u32 = 0x00000001;
    pub const DPI_CMD_W_EMPTY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DPI_BUFF_PLD_FULL** `[17]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dpi_buff_pld_full(&self) -> bool {
        (self.value.read_bits() & Self::DPI_BUFF_PLD_FULL_MASK) != 0
    }

    /// **DPI_BUFF_PLD_EMPTY** `[16]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dpi_buff_pld_empty(&self) -> bool {
        (self.value.read_bits() & Self::DPI_BUFF_PLD_EMPTY_MASK) != 0
    }

    /// **DPI_PLD_W_FULL** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dpi_pld_w_full(&self) -> bool {
        (self.value.read_bits() & Self::DPI_PLD_W_FULL_MASK) != 0
    }

    /// **DPI_PLD_W_EMPTY** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dpi_pld_w_empty(&self) -> bool {
        (self.value.read_bits() & Self::DPI_PLD_W_EMPTY_MASK) != 0
    }

    /// **DPI_CMD_W_FULL** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dpi_cmd_w_full(&self) -> bool {
        (self.value.read_bits() & Self::DPI_CMD_W_FULL_MASK) != 0
    }

    /// **DPI_CMD_W_EMPTY** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn dpi_cmd_w_empty(&self) -> bool {
        (self.value.read_bits() & Self::DPI_CMD_W_EMPTY_MASK) != 0
    }
}

/// VID_PKT_STATUS value type for batch read/modify/write operations
///
/// Use `vidpktstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vidpktstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VidPktStatusVal {
    bits: u32,
}

impl VidPktStatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00010005 }
    }

    /// Read **DPI_BUFF_PLD_FULL** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dpi_buff_pld_full(&self) -> bool {
        (self.bits & VidPktStatus::DPI_BUFF_PLD_FULL_MASK) != 0
    }

    /// Read **DPI_BUFF_PLD_EMPTY** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dpi_buff_pld_empty(&self) -> bool {
        (self.bits & VidPktStatus::DPI_BUFF_PLD_EMPTY_MASK) != 0
    }

    /// Read **DPI_PLD_W_FULL** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dpi_pld_w_full(&self) -> bool {
        (self.bits & VidPktStatus::DPI_PLD_W_FULL_MASK) != 0
    }

    /// Read **DPI_PLD_W_EMPTY** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dpi_pld_w_empty(&self) -> bool {
        (self.bits & VidPktStatus::DPI_PLD_W_EMPTY_MASK) != 0
    }

    /// Read **DPI_CMD_W_FULL** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dpi_cmd_w_full(&self) -> bool {
        (self.bits & VidPktStatus::DPI_CMD_W_FULL_MASK) != 0
    }

    /// Read **DPI_CMD_W_EMPTY** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dpi_cmd_w_empty(&self) -> bool {
        (self.bits & VidPktStatus::DPI_CMD_W_EMPTY_MASK) != 0
    }
}

impl Default for VidPktStatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VidPktStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VidPktStatusVal {
        VidPktStatusVal::from_bits(self.value.read_bits())
    }
}

/// SDF_3D_ACT Register
///
/// NA
///
/// Address offset: 0x0190
///
/// ## Fields
///
/// - **SEND_3D_CFG_ACT** `[16]` (RO)
///
///   NA
///
/// - **RIGHT_FIRST_ACT** `[5]` (RO)
///
///   NA
///
/// - **SECOND_VSYNC_ACT** `[4]` (RO)
///
///   NA
///
/// - **FORMAT_3D_ACT** `[3:2]` (RO)
///
///   NA
///
/// - **MODE_3D_ACT** `[1:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct Sdf3dAct {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sdf3dAct {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sdf3dAct {
    // ======== Field Constants ========

    /// **SEND_3D_CFG_ACT**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const SEND_3D_CFG_ACT_OFFSET: u32 = 16;
    pub const SEND_3D_CFG_ACT_MASK: u32 = 0x00010000;
    pub const SEND_3D_CFG_ACT: u32 = 1 << 16;

    /// **RIGHT_FIRST_ACT**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const RIGHT_FIRST_ACT_OFFSET: u32 = 5;
    pub const RIGHT_FIRST_ACT_MASK: u32 = 0x00000020;
    pub const RIGHT_FIRST_ACT: u32 = 1 << 5;

    /// **SECOND_VSYNC_ACT**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SECOND_VSYNC_ACT_OFFSET: u32 = 4;
    pub const SECOND_VSYNC_ACT_MASK: u32 = 0x00000010;
    pub const SECOND_VSYNC_ACT: u32 = 1 << 4;

    /// **FORMAT_3D_ACT**
    ///
    /// NA
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const FORMAT_3D_ACT_OFFSET: u32 = 2;
    pub const FORMAT_3D_ACT_MASK: u32 = 0x0000000c;

    /// **MODE_3D_ACT**
    ///
    /// NA
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MODE_3D_ACT_OFFSET: u32 = 0;
    pub const MODE_3D_ACT_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SEND_3D_CFG_ACT** `[16]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn send_3d_cfg_act(&self) -> bool {
        (self.value.read_bits() & Self::SEND_3D_CFG_ACT_MASK) != 0
    }

    /// **RIGHT_FIRST_ACT** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn right_first_act(&self) -> bool {
        (self.value.read_bits() & Self::RIGHT_FIRST_ACT_MASK) != 0
    }

    /// **SECOND_VSYNC_ACT** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn second_vsync_act(&self) -> bool {
        (self.value.read_bits() & Self::SECOND_VSYNC_ACT_MASK) != 0
    }

    /// **FORMAT_3D_ACT** `[3:2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn format_3d_act(&self) -> u32 {
        (self.value.read_bits() & Self::FORMAT_3D_ACT_MASK) >> Self::FORMAT_3D_ACT_OFFSET
    }

    /// **MODE_3D_ACT** `[1:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn mode_3d_act(&self) -> u32 {
        (self.value.read_bits() & Self::MODE_3D_ACT_MASK) >> Self::MODE_3D_ACT_OFFSET
    }
}

/// SDF_3D_ACT value type for batch read/modify/write operations
///
/// Use `sdf3dact.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sdf3dact.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sdf3dActVal {
    bits: u32,
}

impl Sdf3dActVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEND_3D_CFG_ACT** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn send_3d_cfg_act(&self) -> bool {
        (self.bits & Sdf3dAct::SEND_3D_CFG_ACT_MASK) != 0
    }

    /// Read **RIGHT_FIRST_ACT** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn right_first_act(&self) -> bool {
        (self.bits & Sdf3dAct::RIGHT_FIRST_ACT_MASK) != 0
    }

    /// Read **SECOND_VSYNC_ACT** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn second_vsync_act(&self) -> bool {
        (self.bits & Sdf3dAct::SECOND_VSYNC_ACT_MASK) != 0
    }

    /// Read **FORMAT_3D_ACT** `[3:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn format_3d_act(&self) -> u32 {
        (self.bits & Sdf3dAct::FORMAT_3D_ACT_MASK) >> Sdf3dAct::FORMAT_3D_ACT_OFFSET
    }

    /// Read **MODE_3D_ACT** `[1:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mode_3d_act(&self) -> u32 {
        (self.bits & Sdf3dAct::MODE_3D_ACT_MASK) >> Sdf3dAct::MODE_3D_ACT_OFFSET
    }
}

impl Default for Sdf3dActVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sdf3dAct {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sdf3dActVal {
        Sdf3dActVal::from_bits(self.value.read_bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// NA
    pub version: Version,
    /// 0x0004
    /// NA
    pub pwr_up: PwrUp,
    /// 0x0008
    /// NA
    pub clkmgr_cfg: ClkmgrCfg,
    /// 0x000c
    /// NA
    pub dpi_vcid: DpiVcid,
    /// 0x0010
    /// NA
    pub dpi_color_coding: DpiColorCoding,
    /// 0x0014
    /// NA
    pub dpi_cfg_pol: DpiCfgPol,
    /// 0x0018
    /// NA
    pub dpi_lp_cmd_tim: DpiLpCmdTim,
    /// 0x001c
    /// NA
    pub dbi_vcid: DbiVcid,
    /// 0x0020
    /// NA
    pub dbi_cfg: DbiCfg,
    /// 0x0024
    /// NA
    pub dbi_partitioning_en: DbiPartitioningEn,
    /// 0x0028
    /// NA
    pub dbi_cmdsize: DbiCmdsize,
    /// 0x002c
    /// NA
    pub pckhdl_cfg: PckhdlCfg,
    /// 0x0030
    /// NA
    pub gen_vcid: GenVcid,
    /// 0x0034
    /// NA
    pub mode_cfg: ModeCfg,
    /// 0x0038
    /// NA
    pub vid_mode_cfg: VidModeCfg,
    /// 0x003c
    /// NA
    pub vid_pkt_size: VidPktSize,
    /// 0x0040
    /// NA
    pub vid_num_chunks: VidNumChunks,
    /// 0x0044
    /// NA
    pub vid_null_size: VidNullSize,
    /// 0x0048
    /// NA
    pub vid_hsa_time: VidHsaTime,
    /// 0x004c
    /// NA
    pub vid_hbp_time: VidHbpTime,
    /// 0x0050
    /// NA
    pub vid_hline_time: VidHlineTime,
    /// 0x0054
    /// NA
    pub vid_vsa_lines: VidVsaLines,
    /// 0x0058
    /// NA
    pub vid_vbp_lines: VidVbpLines,
    /// 0x005c
    /// NA
    pub vid_vfp_lines: VidVfpLines,
    /// 0x0060
    /// NA
    pub vid_vactive_lines: VidVactiveLines,
    /// 0x0064
    /// NA
    pub edpi_cmd_size: EdpiCmdSize,
    /// 0x0068
    /// NA
    pub cmd_mode_cfg: CmdModeCfg,
    /// 0x006c
    /// NA
    pub gen_hdr: GenHdr,
    /// 0x0070
    /// NA
    pub gen_pld_data: GenPldData,
    /// 0x0074
    /// NA
    pub cmd_pkt_status: CmdPktStatus,
    /// 0x0078
    /// NA
    pub to_cnt_cfg: ToCntCfg,
    /// 0x007c
    /// NA
    pub hs_rd_to_cnt: HsRdToCnt,
    /// 0x0080
    /// NA
    pub lp_rd_to_cnt: LpRdToCnt,
    /// 0x0084
    /// NA
    pub hs_wr_to_cnt: HsWrToCnt,
    /// 0x0088
    /// NA
    pub lp_wr_to_cnt: LpWrToCnt,
    /// 0x008c
    /// NA
    pub bta_to_cnt: BtaToCnt,
    /// 0x0090
    /// NA
    pub sdf3d: Sdf3d,
    /// 0x0094
    /// NA
    pub lpclk_ctrl: LpclkCtrl,
    /// 0x0098
    /// NA
    pub phy_tmr_lpclk_cfg: PhyTmrLpclkCfg,
    /// 0x009c
    /// NA
    pub phy_tmr_cfg: PhyTmrCfg,
    /// 0x00a0
    /// NA
    pub phy_rstz: PhyRstz,
    /// 0x00a4
    /// NA
    pub phy_if_cfg: PhyIfCfg,
    /// 0x00a8
    /// NA
    pub phy_ulps_ctrl: PhyUlpsCtrl,
    /// 0x00ac
    /// NA
    pub phy_tx_triggers: PhyTxTriggers,
    /// 0x00b0
    /// NA
    pub phy_status: PhyStatus,
    /// 0x00b4
    /// NA
    pub phy_tst_ctrl0: PhyTstCtrl0,
    /// 0x00b8
    /// NA
    pub phy_tst_ctrl1: PhyTstCtrl1,
    /// 0x00bc
    /// NA
    pub int_st0: IntSt0,
    /// 0x00c0
    /// NA
    pub int_st1: IntSt1,
    /// 0x00c4
    /// NA
    pub int_msk0: IntMsk0,
    /// 0x00c8
    /// NA
    pub int_msk1: IntMsk1,
    /// 0x00cc
    /// NA
    pub phy_cal: PhyCal,
    _reserved0: [u8; 8],
    /// 0x00d8
    /// NA
    pub int_force0: IntForce0,
    /// 0x00dc
    /// NA
    pub int_force1: IntForce1,
    _reserved1: [u8; 16],
    /// 0x00f0
    /// NA
    pub dsc_parameter: DscParameter,
    /// 0x00f4
    /// NA
    pub phy_tmr_rd_cfg: PhyTmrRdCfg,
    _reserved2: [u8; 8],
    /// 0x0100
    /// NA
    pub vid_shadow_ctrl: VidShadowCtrl,
    _reserved3: [u8; 8],
    /// 0x010c
    /// NA
    pub dpi_vcid_act: DpiVcidAct,
    /// 0x0110
    /// NA
    pub dpi_color_coding_act: DpiColorCodingAct,
    _reserved4: [u8; 4],
    /// 0x0118
    /// NA
    pub dpi_lp_cmd_tim_act: DpiLpCmdTimAct,
    /// 0x011c
    /// NA
    pub edpi_te_hw_cfg: EdpiTeHwCfg,
    _reserved5: [u8; 24],
    /// 0x0138
    /// NA
    pub vid_mode_cfg_act: VidModeCfgAct,
    /// 0x013c
    /// NA
    pub vid_pkt_size_act: VidPktSizeAct,
    /// 0x0140
    /// NA
    pub vid_num_chunks_act: VidNumChunksAct,
    /// 0x0144
    /// NA
    pub vid_null_size_act: VidNullSizeAct,
    /// 0x0148
    /// NA
    pub vid_hsa_time_act: VidHsaTimeAct,
    /// 0x014c
    /// NA
    pub vid_hbp_time_act: VidHbpTimeAct,
    /// 0x0150
    /// NA
    pub vid_hline_time_act: VidHlineTimeAct,
    /// 0x0154
    /// NA
    pub vid_vsa_lines_act: VidVsaLinesAct,
    /// 0x0158
    /// NA
    pub vid_vbp_lines_act: VidVbpLinesAct,
    /// 0x015c
    /// NA
    pub vid_vfp_lines_act: VidVfpLinesAct,
    /// 0x0160
    /// NA
    pub vid_vactive_lines_act: VidVactiveLinesAct,
    _reserved6: [u8; 4],
    /// 0x0168
    /// NA
    pub vid_pkt_status: VidPktStatus,
    _reserved7: [u8; 36],
    /// 0x0190
    /// NA
    pub sdf3d_act: Sdf3dAct,
}
