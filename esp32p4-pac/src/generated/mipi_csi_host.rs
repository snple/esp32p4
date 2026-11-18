//! MIPI_CSI_HOST
//! MIPI Camera Interface Host

#![allow(dead_code)]

pub const BASE: u32 = 0x5009f000;

/// VERSION Register
///
/// NA
///
/// Address offset: 0x0000
/// Reset value: 0x3135302a
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
        Self { bits: 0x3135302a }
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

/// N_LANES Register
///
/// NA
///
/// Address offset: 0x0004
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **N_LANES** `[2:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct NLanes {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for NLanes {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl NLanes {
    // ======== Field Constants ========

    /// **N_LANES**
    ///
    /// NA
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const N_LANES_OFFSET: u32 = 0;
    pub const N_LANES_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **N_LANES** `[2:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn n_lanes(&self) -> u32 {
        (self.value.read_bits() & Self::N_LANES_MASK) >> Self::N_LANES_OFFSET
    }

    /// Write **N_LANES** `[2:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_n_lanes(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::N_LANES_MASK) | ((value << Self::N_LANES_OFFSET) & Self::N_LANES_MASK)
        });
    }
}

/// N_LANES value type for batch read/modify/write operations
///
/// Use `nlanes.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `nlanes.write_val(val)`.
#[derive(Clone, Copy)]
pub struct NLanesVal {
    bits: u32,
}

impl NLanesVal {
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

    /// Read **N_LANES** `[2:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn n_lanes(&self) -> u32 {
        (self.bits & NLanes::N_LANES_MASK) >> NLanes::N_LANES_OFFSET
    }

    /// Set **N_LANES** `[2:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_n_lanes(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !NLanes::N_LANES_MASK)
                | ((value << NLanes::N_LANES_OFFSET) & NLanes::N_LANES_MASK),
        }
    }
}

impl Default for NLanesVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl NLanes {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> NLanesVal {
        NLanesVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: NLanesVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(NLanesVal) -> NLanesVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CSI2_RESETN Register
///
/// NA
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **CSI2_RESETN** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Csi2Resetn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Csi2Resetn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Csi2Resetn {
    // ======== Field Constants ========

    /// **CSI2_RESETN**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CSI2_RESETN_OFFSET: u32 = 0;
    pub const CSI2_RESETN_MASK: u32 = 0x00000001;
    pub const CSI2_RESETN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CSI2_RESETN** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn csi2_resetn(&self) -> bool {
        (self.value.read_bits() & Self::CSI2_RESETN_MASK) != 0
    }

    /// Set CSI2_RESETN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_csi2_resetn(&self) {
        self.value.modify_bits(|r| r | Self::CSI2_RESETN_MASK);
    }

    /// Clear CSI2_RESETN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_csi2_resetn(&self) {
        self.value.modify_bits(|r| r & !Self::CSI2_RESETN_MASK);
    }
}

/// CSI2_RESETN value type for batch read/modify/write operations
///
/// Use `csi2resetn.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `csi2resetn.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Csi2ResetnVal {
    bits: u32,
}

impl Csi2ResetnVal {
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

    /// Read **CSI2_RESETN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn csi2_resetn(&self) -> bool {
        (self.bits & Csi2Resetn::CSI2_RESETN_MASK) != 0
    }

    /// Set **CSI2_RESETN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_csi2_resetn(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Csi2Resetn::CSI2_RESETN_MASK
            } else {
                self.bits & !Csi2Resetn::CSI2_RESETN_MASK
            },
        }
    }
}

impl Default for Csi2ResetnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Csi2Resetn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Csi2ResetnVal {
        Csi2ResetnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Csi2ResetnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Csi2ResetnVal) -> Csi2ResetnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ST_MAIN Register
///
/// NA
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **ST_STATUS_INT_PHY** `[16]` (RO)
///
///   NA
///
/// - **ST_STATUS_INT_ECC_CORRECTED** `[7]` (RO)
///
///   NA
///
/// - **ST_STATUS_INT_DATA_ID** `[6]` (RO)
///
///   NA
///
/// - **ST_STATUS_INT_PLD_CRC_FATAL** `[5]` (RO)
///
///   NA
///
/// - **ST_STATUS_INT_CRC_FRAME_FATAL** `[4]` (RO)
///
///   NA
///
/// - **ST_STATUS_INT_SEQ_FRAME_FATAL** `[3]` (RO)
///
///   NA
///
/// - **ST_STATUS_INT_BNDRY_FRAME_FATAL** `[2]` (RO)
///
///   NA
///
/// - **ST_STATUS_INT_PKT_FATAL** `[1]` (RO)
///
///   NA
///
/// - **ST_STATUS_INT_PHY_FATAL** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntStMain {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntStMain {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntStMain {
    // ======== Field Constants ========

    /// **ST_STATUS_INT_PHY**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ST_STATUS_INT_PHY_OFFSET: u32 = 16;
    pub const ST_STATUS_INT_PHY_MASK: u32 = 0x00010000;
    pub const ST_STATUS_INT_PHY: u32 = 1 << 16;

    /// **ST_STATUS_INT_ECC_CORRECTED**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ST_STATUS_INT_ECC_CORRECTED_OFFSET: u32 = 7;
    pub const ST_STATUS_INT_ECC_CORRECTED_MASK: u32 = 0x00000080;
    pub const ST_STATUS_INT_ECC_CORRECTED: u32 = 1 << 7;

    /// **ST_STATUS_INT_DATA_ID**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ST_STATUS_INT_DATA_ID_OFFSET: u32 = 6;
    pub const ST_STATUS_INT_DATA_ID_MASK: u32 = 0x00000040;
    pub const ST_STATUS_INT_DATA_ID: u32 = 1 << 6;

    /// **ST_STATUS_INT_PLD_CRC_FATAL**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ST_STATUS_INT_PLD_CRC_FATAL_OFFSET: u32 = 5;
    pub const ST_STATUS_INT_PLD_CRC_FATAL_MASK: u32 = 0x00000020;
    pub const ST_STATUS_INT_PLD_CRC_FATAL: u32 = 1 << 5;

    /// **ST_STATUS_INT_CRC_FRAME_FATAL**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ST_STATUS_INT_CRC_FRAME_FATAL_OFFSET: u32 = 4;
    pub const ST_STATUS_INT_CRC_FRAME_FATAL_MASK: u32 = 0x00000010;
    pub const ST_STATUS_INT_CRC_FRAME_FATAL: u32 = 1 << 4;

    /// **ST_STATUS_INT_SEQ_FRAME_FATAL**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ST_STATUS_INT_SEQ_FRAME_FATAL_OFFSET: u32 = 3;
    pub const ST_STATUS_INT_SEQ_FRAME_FATAL_MASK: u32 = 0x00000008;
    pub const ST_STATUS_INT_SEQ_FRAME_FATAL: u32 = 1 << 3;

    /// **ST_STATUS_INT_BNDRY_FRAME_FATAL**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ST_STATUS_INT_BNDRY_FRAME_FATAL_OFFSET: u32 = 2;
    pub const ST_STATUS_INT_BNDRY_FRAME_FATAL_MASK: u32 = 0x00000004;
    pub const ST_STATUS_INT_BNDRY_FRAME_FATAL: u32 = 1 << 2;

    /// **ST_STATUS_INT_PKT_FATAL**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ST_STATUS_INT_PKT_FATAL_OFFSET: u32 = 1;
    pub const ST_STATUS_INT_PKT_FATAL_MASK: u32 = 0x00000002;
    pub const ST_STATUS_INT_PKT_FATAL: u32 = 1 << 1;

    /// **ST_STATUS_INT_PHY_FATAL**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ST_STATUS_INT_PHY_FATAL_OFFSET: u32 = 0;
    pub const ST_STATUS_INT_PHY_FATAL_MASK: u32 = 0x00000001;
    pub const ST_STATUS_INT_PHY_FATAL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ST_STATUS_INT_PHY** `[16]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_status_int_phy(&self) -> bool {
        (self.value.read_bits() & Self::ST_STATUS_INT_PHY_MASK) != 0
    }

    /// **ST_STATUS_INT_ECC_CORRECTED** `[7]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_status_int_ecc_corrected(&self) -> bool {
        (self.value.read_bits() & Self::ST_STATUS_INT_ECC_CORRECTED_MASK) != 0
    }

    /// **ST_STATUS_INT_DATA_ID** `[6]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_status_int_data_id(&self) -> bool {
        (self.value.read_bits() & Self::ST_STATUS_INT_DATA_ID_MASK) != 0
    }

    /// **ST_STATUS_INT_PLD_CRC_FATAL** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_status_int_pld_crc_fatal(&self) -> bool {
        (self.value.read_bits() & Self::ST_STATUS_INT_PLD_CRC_FATAL_MASK) != 0
    }

    /// **ST_STATUS_INT_CRC_FRAME_FATAL** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_status_int_crc_frame_fatal(&self) -> bool {
        (self.value.read_bits() & Self::ST_STATUS_INT_CRC_FRAME_FATAL_MASK) != 0
    }

    /// **ST_STATUS_INT_SEQ_FRAME_FATAL** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_status_int_seq_frame_fatal(&self) -> bool {
        (self.value.read_bits() & Self::ST_STATUS_INT_SEQ_FRAME_FATAL_MASK) != 0
    }

    /// **ST_STATUS_INT_BNDRY_FRAME_FATAL** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_status_int_bndry_frame_fatal(&self) -> bool {
        (self.value.read_bits() & Self::ST_STATUS_INT_BNDRY_FRAME_FATAL_MASK) != 0
    }

    /// **ST_STATUS_INT_PKT_FATAL** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_status_int_pkt_fatal(&self) -> bool {
        (self.value.read_bits() & Self::ST_STATUS_INT_PKT_FATAL_MASK) != 0
    }

    /// **ST_STATUS_INT_PHY_FATAL** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_status_int_phy_fatal(&self) -> bool {
        (self.value.read_bits() & Self::ST_STATUS_INT_PHY_FATAL_MASK) != 0
    }
}

/// INT_ST_MAIN value type for batch read/modify/write operations
///
/// Use `intstmain.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intstmain.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStMainVal {
    bits: u32,
}

impl IntStMainVal {
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

    /// Read **ST_STATUS_INT_PHY** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_status_int_phy(&self) -> bool {
        (self.bits & IntStMain::ST_STATUS_INT_PHY_MASK) != 0
    }

    /// Read **ST_STATUS_INT_ECC_CORRECTED** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_status_int_ecc_corrected(&self) -> bool {
        (self.bits & IntStMain::ST_STATUS_INT_ECC_CORRECTED_MASK) != 0
    }

    /// Read **ST_STATUS_INT_DATA_ID** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_status_int_data_id(&self) -> bool {
        (self.bits & IntStMain::ST_STATUS_INT_DATA_ID_MASK) != 0
    }

    /// Read **ST_STATUS_INT_PLD_CRC_FATAL** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_status_int_pld_crc_fatal(&self) -> bool {
        (self.bits & IntStMain::ST_STATUS_INT_PLD_CRC_FATAL_MASK) != 0
    }

    /// Read **ST_STATUS_INT_CRC_FRAME_FATAL** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_status_int_crc_frame_fatal(&self) -> bool {
        (self.bits & IntStMain::ST_STATUS_INT_CRC_FRAME_FATAL_MASK) != 0
    }

    /// Read **ST_STATUS_INT_SEQ_FRAME_FATAL** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_status_int_seq_frame_fatal(&self) -> bool {
        (self.bits & IntStMain::ST_STATUS_INT_SEQ_FRAME_FATAL_MASK) != 0
    }

    /// Read **ST_STATUS_INT_BNDRY_FRAME_FATAL** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_status_int_bndry_frame_fatal(&self) -> bool {
        (self.bits & IntStMain::ST_STATUS_INT_BNDRY_FRAME_FATAL_MASK) != 0
    }

    /// Read **ST_STATUS_INT_PKT_FATAL** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_status_int_pkt_fatal(&self) -> bool {
        (self.bits & IntStMain::ST_STATUS_INT_PKT_FATAL_MASK) != 0
    }

    /// Read **ST_STATUS_INT_PHY_FATAL** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_status_int_phy_fatal(&self) -> bool {
        (self.bits & IntStMain::ST_STATUS_INT_PHY_FATAL_MASK) != 0
    }
}

impl Default for IntStMainVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntStMain {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStMainVal {
        IntStMainVal::from_bits(self.value.read_bits())
    }
}

/// PHY_SHUTDOWNZ Register
///
/// NA
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **PHY_SHUTDOWNZ** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PhyShutdownz {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyShutdownz {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyShutdownz {
    // ======== Field Constants ========

    /// **PHY_SHUTDOWNZ**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PHY_SHUTDOWNZ_OFFSET: u32 = 0;
    pub const PHY_SHUTDOWNZ_MASK: u32 = 0x00000001;
    pub const PHY_SHUTDOWNZ: u32 = 1 << 0;

    // ======== Field Accessors ========

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

/// PHY_SHUTDOWNZ value type for batch read/modify/write operations
///
/// Use `physhutdownz.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `physhutdownz.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyShutdownzVal {
    bits: u32,
}

impl PhyShutdownzVal {
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

    /// Read **PHY_SHUTDOWNZ** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_shutdownz(&self) -> bool {
        (self.bits & PhyShutdownz::PHY_SHUTDOWNZ_MASK) != 0
    }

    /// Set **PHY_SHUTDOWNZ** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_shutdownz(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyShutdownz::PHY_SHUTDOWNZ_MASK
            } else {
                self.bits & !PhyShutdownz::PHY_SHUTDOWNZ_MASK
            },
        }
    }
}

impl Default for PhyShutdownzVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PhyShutdownz {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyShutdownzVal {
        PhyShutdownzVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyShutdownzVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyShutdownzVal) -> PhyShutdownzVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DPHY_RSTZ Register
///
/// NA
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **DPHY_RSTZ** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DphyRstz {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DphyRstz {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DphyRstz {
    // ======== Field Constants ========

    /// **DPHY_RSTZ**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DPHY_RSTZ_OFFSET: u32 = 0;
    pub const DPHY_RSTZ_MASK: u32 = 0x00000001;
    pub const DPHY_RSTZ: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DPHY_RSTZ** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn dphy_rstz(&self) -> bool {
        (self.value.read_bits() & Self::DPHY_RSTZ_MASK) != 0
    }

    /// Set DPHY_RSTZ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dphy_rstz(&self) {
        self.value.modify_bits(|r| r | Self::DPHY_RSTZ_MASK);
    }

    /// Clear DPHY_RSTZ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dphy_rstz(&self) {
        self.value.modify_bits(|r| r & !Self::DPHY_RSTZ_MASK);
    }
}

/// DPHY_RSTZ value type for batch read/modify/write operations
///
/// Use `dphyrstz.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dphyrstz.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DphyRstzVal {
    bits: u32,
}

impl DphyRstzVal {
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

    /// Read **DPHY_RSTZ** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn dphy_rstz(&self) -> bool {
        (self.bits & DphyRstz::DPHY_RSTZ_MASK) != 0
    }

    /// Set **DPHY_RSTZ** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_dphy_rstz(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DphyRstz::DPHY_RSTZ_MASK
            } else {
                self.bits & !DphyRstz::DPHY_RSTZ_MASK
            },
        }
    }
}

impl Default for DphyRstzVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DphyRstz {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DphyRstzVal {
        DphyRstzVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DphyRstzVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DphyRstzVal) -> DphyRstzVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_RX Register
///
/// NA
///
/// Address offset: 0x0048
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **PHY_RXCLKACTIVEHS** `[17]` (RO)
///
///   NA
///
/// - **PHY_RXULPSCLKNOT** `[16]` (RO)
///
///   NA
///
/// - **PHY_RXULPSESC_1** `[1]` (RO)
///
///   NA
///
/// - **PHY_RXULPSESC_0** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct PhyRx {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyRx {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyRx {
    // ======== Field Constants ========

    /// **PHY_RXCLKACTIVEHS**
    ///
    /// NA
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const PHY_RXCLKACTIVEHS_OFFSET: u32 = 17;
    pub const PHY_RXCLKACTIVEHS_MASK: u32 = 0x00020000;
    pub const PHY_RXCLKACTIVEHS: u32 = 1 << 17;

    /// **PHY_RXULPSCLKNOT**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const PHY_RXULPSCLKNOT_OFFSET: u32 = 16;
    pub const PHY_RXULPSCLKNOT_MASK: u32 = 0x00010000;
    pub const PHY_RXULPSCLKNOT: u32 = 1 << 16;

    /// **PHY_RXULPSESC_1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PHY_RXULPSESC_1_OFFSET: u32 = 1;
    pub const PHY_RXULPSESC_1_MASK: u32 = 0x00000002;
    pub const PHY_RXULPSESC_1: u32 = 1 << 1;

    /// **PHY_RXULPSESC_0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PHY_RXULPSESC_0_OFFSET: u32 = 0;
    pub const PHY_RXULPSESC_0_MASK: u32 = 0x00000001;
    pub const PHY_RXULPSESC_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PHY_RXCLKACTIVEHS** `[17]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_rxclkactivehs(&self) -> bool {
        (self.value.read_bits() & Self::PHY_RXCLKACTIVEHS_MASK) != 0
    }

    /// **PHY_RXULPSCLKNOT** `[16]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_rxulpsclknot(&self) -> bool {
        (self.value.read_bits() & Self::PHY_RXULPSCLKNOT_MASK) != 0
    }

    /// **PHY_RXULPSESC_1** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_rxulpsesc_1(&self) -> bool {
        (self.value.read_bits() & Self::PHY_RXULPSESC_1_MASK) != 0
    }

    /// **PHY_RXULPSESC_0** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_rxulpsesc_0(&self) -> bool {
        (self.value.read_bits() & Self::PHY_RXULPSESC_0_MASK) != 0
    }
}

/// PHY_RX value type for batch read/modify/write operations
///
/// Use `phyrx.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phyrx.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyRxVal {
    bits: u32,
}

impl PhyRxVal {
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
        Self { bits: 0x00010000 }
    }

    /// Read **PHY_RXCLKACTIVEHS** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_rxclkactivehs(&self) -> bool {
        (self.bits & PhyRx::PHY_RXCLKACTIVEHS_MASK) != 0
    }

    /// Read **PHY_RXULPSCLKNOT** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_rxulpsclknot(&self) -> bool {
        (self.bits & PhyRx::PHY_RXULPSCLKNOT_MASK) != 0
    }

    /// Read **PHY_RXULPSESC_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_rxulpsesc_1(&self) -> bool {
        (self.bits & PhyRx::PHY_RXULPSESC_1_MASK) != 0
    }

    /// Read **PHY_RXULPSESC_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_rxulpsesc_0(&self) -> bool {
        (self.bits & PhyRx::PHY_RXULPSESC_0_MASK) != 0
    }
}

impl Default for PhyRxVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PhyRx {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyRxVal {
        PhyRxVal::from_bits(self.value.read_bits())
    }
}

/// PHY_STOPSTATE Register
///
/// NA
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **PHY_STOPSTATECLK** `[16]` (RO)
///
///   NA
///
/// - **PHY_STOPSTATEDATA_1** `[1]` (RO)
///
///   NA
///
/// - **PHY_STOPSTATEDATA_0** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct PhyStopstate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyStopstate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyStopstate {
    // ======== Field Constants ========

    /// **PHY_STOPSTATECLK**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const PHY_STOPSTATECLK_OFFSET: u32 = 16;
    pub const PHY_STOPSTATECLK_MASK: u32 = 0x00010000;
    pub const PHY_STOPSTATECLK: u32 = 1 << 16;

    /// **PHY_STOPSTATEDATA_1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PHY_STOPSTATEDATA_1_OFFSET: u32 = 1;
    pub const PHY_STOPSTATEDATA_1_MASK: u32 = 0x00000002;
    pub const PHY_STOPSTATEDATA_1: u32 = 1 << 1;

    /// **PHY_STOPSTATEDATA_0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PHY_STOPSTATEDATA_0_OFFSET: u32 = 0;
    pub const PHY_STOPSTATEDATA_0_MASK: u32 = 0x00000001;
    pub const PHY_STOPSTATEDATA_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PHY_STOPSTATECLK** `[16]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_stopstateclk(&self) -> bool {
        (self.value.read_bits() & Self::PHY_STOPSTATECLK_MASK) != 0
    }

    /// **PHY_STOPSTATEDATA_1** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_stopstatedata_1(&self) -> bool {
        (self.value.read_bits() & Self::PHY_STOPSTATEDATA_1_MASK) != 0
    }

    /// **PHY_STOPSTATEDATA_0** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_stopstatedata_0(&self) -> bool {
        (self.value.read_bits() & Self::PHY_STOPSTATEDATA_0_MASK) != 0
    }
}

/// PHY_STOPSTATE value type for batch read/modify/write operations
///
/// Use `phystopstate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phystopstate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyStopstateVal {
    bits: u32,
}

impl PhyStopstateVal {
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

    /// Read **PHY_STOPSTATECLK** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_stopstateclk(&self) -> bool {
        (self.bits & PhyStopstate::PHY_STOPSTATECLK_MASK) != 0
    }

    /// Read **PHY_STOPSTATEDATA_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_stopstatedata_1(&self) -> bool {
        (self.bits & PhyStopstate::PHY_STOPSTATEDATA_1_MASK) != 0
    }

    /// Read **PHY_STOPSTATEDATA_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_stopstatedata_0(&self) -> bool {
        (self.bits & PhyStopstate::PHY_STOPSTATEDATA_0_MASK) != 0
    }
}

impl Default for PhyStopstateVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PhyStopstate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyStopstateVal {
        PhyStopstateVal::from_bits(self.value.read_bits())
    }
}

/// PHY_TEST_CTRL0 Register
///
/// NA
///
/// Address offset: 0x0050
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
pub struct PhyTestCtrl0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyTestCtrl0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyTestCtrl0 {
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

/// PHY_TEST_CTRL0 value type for batch read/modify/write operations
///
/// Use `phytestctrl0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phytestctrl0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyTestCtrl0Val {
    bits: u32,
}

impl PhyTestCtrl0Val {
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
        (self.bits & PhyTestCtrl0::PHY_TESTCLK_MASK) != 0
    }

    /// Read **PHY_TESTCLR** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_testclr(&self) -> bool {
        (self.bits & PhyTestCtrl0::PHY_TESTCLR_MASK) != 0
    }

    /// Set **PHY_TESTCLK** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_testclk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyTestCtrl0::PHY_TESTCLK_MASK
            } else {
                self.bits & !PhyTestCtrl0::PHY_TESTCLK_MASK
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
                self.bits | PhyTestCtrl0::PHY_TESTCLR_MASK
            } else {
                self.bits & !PhyTestCtrl0::PHY_TESTCLR_MASK
            },
        }
    }
}

impl Default for PhyTestCtrl0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PhyTestCtrl0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyTestCtrl0Val {
        PhyTestCtrl0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyTestCtrl0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyTestCtrl0Val) -> PhyTestCtrl0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_TEST_CTRL1 Register
///
/// NA
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **PHY_TESTEN** `[16]` (RW)
///
///   NA
///
/// - **PHY_TESTDOUT** `[15:8]` (RO)
///
///   NA
///
/// - **PHY_TESTDIN** `[7:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PhyTestCtrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyTestCtrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyTestCtrl1 {
    // ======== Field Constants ========

    /// **PHY_TESTEN**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const PHY_TESTEN_OFFSET: u32 = 16;
    pub const PHY_TESTEN_MASK: u32 = 0x00010000;
    pub const PHY_TESTEN: u32 = 1 << 16;

    /// **PHY_TESTDOUT**
    ///
    /// NA
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const PHY_TESTDOUT_OFFSET: u32 = 8;
    pub const PHY_TESTDOUT_MASK: u32 = 0x0000ff00;

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

    /// **PHY_TESTDOUT** `[15:8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn phy_testdout(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_TESTDOUT_MASK) >> Self::PHY_TESTDOUT_OFFSET
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

/// PHY_TEST_CTRL1 value type for batch read/modify/write operations
///
/// Use `phytestctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phytestctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyTestCtrl1Val {
    bits: u32,
}

impl PhyTestCtrl1Val {
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
        (self.bits & PhyTestCtrl1::PHY_TESTEN_MASK) != 0
    }

    /// Read **PHY_TESTDOUT** `[15:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_testdout(&self) -> u32 {
        (self.bits & PhyTestCtrl1::PHY_TESTDOUT_MASK) >> PhyTestCtrl1::PHY_TESTDOUT_OFFSET
    }

    /// Read **PHY_TESTDIN** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn phy_testdin(&self) -> u32 {
        (self.bits & PhyTestCtrl1::PHY_TESTDIN_MASK) >> PhyTestCtrl1::PHY_TESTDIN_OFFSET
    }

    /// Set **PHY_TESTEN** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_testen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyTestCtrl1::PHY_TESTEN_MASK
            } else {
                self.bits & !PhyTestCtrl1::PHY_TESTEN_MASK
            },
        }
    }

    /// Set **PHY_TESTDIN** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_phy_testdin(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyTestCtrl1::PHY_TESTDIN_MASK)
                | ((value << PhyTestCtrl1::PHY_TESTDIN_OFFSET) & PhyTestCtrl1::PHY_TESTDIN_MASK),
        }
    }
}

impl Default for PhyTestCtrl1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PhyTestCtrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyTestCtrl1Val {
        PhyTestCtrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyTestCtrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyTestCtrl1Val) -> PhyTestCtrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VC_EXTENSION Register
///
/// NA
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **VCX** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct VcExtension {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VcExtension {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VcExtension {
    // ======== Field Constants ========

    /// **VCX**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VCX_OFFSET: u32 = 0;
    pub const VCX_MASK: u32 = 0x00000001;
    pub const VCX: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **VCX** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn vcx(&self) -> bool {
        (self.value.read_bits() & Self::VCX_MASK) != 0
    }

    /// Set VCX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vcx(&self) {
        self.value.modify_bits(|r| r | Self::VCX_MASK);
    }

    /// Clear VCX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vcx(&self) {
        self.value.modify_bits(|r| r & !Self::VCX_MASK);
    }
}

/// VC_EXTENSION value type for batch read/modify/write operations
///
/// Use `vcextension.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vcextension.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VcExtensionVal {
    bits: u32,
}

impl VcExtensionVal {
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

    /// Read **VCX** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn vcx(&self) -> bool {
        (self.bits & VcExtension::VCX_MASK) != 0
    }

    /// Set **VCX** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_vcx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VcExtension::VCX_MASK
            } else {
                self.bits & !VcExtension::VCX_MASK
            },
        }
    }
}

impl Default for VcExtensionVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VcExtension {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VcExtensionVal {
        VcExtensionVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VcExtensionVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VcExtensionVal) -> VcExtensionVal>(&self, f: F) {
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
/// - **RXSKEWCALHS** `[0]` (RO)
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

    /// **RXSKEWCALHS**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RXSKEWCALHS_OFFSET: u32 = 0;
    pub const RXSKEWCALHS_MASK: u32 = 0x00000001;
    pub const RXSKEWCALHS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RXSKEWCALHS** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn rxskewcalhs(&self) -> bool {
        (self.value.read_bits() & Self::RXSKEWCALHS_MASK) != 0
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

    /// Read **RXSKEWCALHS** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn rxskewcalhs(&self) -> bool {
        (self.bits & PhyCal::RXSKEWCALHS_MASK) != 0
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
}

/// INT_ST_PHY_FATAL Register
///
/// NA
///
/// Address offset: 0x00e0
///
/// ## Fields
///
/// - **ST_PHY_ERRSOTSYNCHS_1** `[1]` (RO)
///
///   NA
///
/// - **ST_PHY_ERRSOTSYNCHS_0** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntStPhyFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntStPhyFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntStPhyFatal {
    // ======== Field Constants ========

    /// **ST_PHY_ERRSOTSYNCHS_1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ST_PHY_ERRSOTSYNCHS_1_OFFSET: u32 = 1;
    pub const ST_PHY_ERRSOTSYNCHS_1_MASK: u32 = 0x00000002;
    pub const ST_PHY_ERRSOTSYNCHS_1: u32 = 1 << 1;

    /// **ST_PHY_ERRSOTSYNCHS_0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ST_PHY_ERRSOTSYNCHS_0_OFFSET: u32 = 0;
    pub const ST_PHY_ERRSOTSYNCHS_0_MASK: u32 = 0x00000001;
    pub const ST_PHY_ERRSOTSYNCHS_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ST_PHY_ERRSOTSYNCHS_1** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_phy_errsotsynchs_1(&self) -> bool {
        (self.value.read_bits() & Self::ST_PHY_ERRSOTSYNCHS_1_MASK) != 0
    }

    /// **ST_PHY_ERRSOTSYNCHS_0** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_phy_errsotsynchs_0(&self) -> bool {
        (self.value.read_bits() & Self::ST_PHY_ERRSOTSYNCHS_0_MASK) != 0
    }
}

/// INT_ST_PHY_FATAL value type for batch read/modify/write operations
///
/// Use `intstphyfatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intstphyfatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStPhyFatalVal {
    bits: u32,
}

impl IntStPhyFatalVal {
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

    /// Read **ST_PHY_ERRSOTSYNCHS_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_phy_errsotsynchs_1(&self) -> bool {
        (self.bits & IntStPhyFatal::ST_PHY_ERRSOTSYNCHS_1_MASK) != 0
    }

    /// Read **ST_PHY_ERRSOTSYNCHS_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_phy_errsotsynchs_0(&self) -> bool {
        (self.bits & IntStPhyFatal::ST_PHY_ERRSOTSYNCHS_0_MASK) != 0
    }
}

impl Default for IntStPhyFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntStPhyFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStPhyFatalVal {
        IntStPhyFatalVal::from_bits(self.value.read_bits())
    }
}

/// INT_MSK_PHY_FATAL Register
///
/// NA
///
/// Address offset: 0x00e4
///
/// ## Fields
///
/// - **MASK_PHY_ERRSOTSYNCHS_1** `[1]` (RW)
///
///   NA
///
/// - **MASK_PHY_ERRSOTSYNCHS_0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntMskPhyFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntMskPhyFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntMskPhyFatal {
    // ======== Field Constants ========

    /// **MASK_PHY_ERRSOTSYNCHS_1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MASK_PHY_ERRSOTSYNCHS_1_OFFSET: u32 = 1;
    pub const MASK_PHY_ERRSOTSYNCHS_1_MASK: u32 = 0x00000002;
    pub const MASK_PHY_ERRSOTSYNCHS_1: u32 = 1 << 1;

    /// **MASK_PHY_ERRSOTSYNCHS_0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MASK_PHY_ERRSOTSYNCHS_0_OFFSET: u32 = 0;
    pub const MASK_PHY_ERRSOTSYNCHS_0_MASK: u32 = 0x00000001;
    pub const MASK_PHY_ERRSOTSYNCHS_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MASK_PHY_ERRSOTSYNCHS_1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_phy_errsotsynchs_1(&self) -> bool {
        (self.value.read_bits() & Self::MASK_PHY_ERRSOTSYNCHS_1_MASK) != 0
    }

    /// Set MASK_PHY_ERRSOTSYNCHS_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_phy_errsotsynchs_1(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_PHY_ERRSOTSYNCHS_1_MASK);
    }

    /// Clear MASK_PHY_ERRSOTSYNCHS_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_phy_errsotsynchs_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_PHY_ERRSOTSYNCHS_1_MASK);
    }

    /// **MASK_PHY_ERRSOTSYNCHS_0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_phy_errsotsynchs_0(&self) -> bool {
        (self.value.read_bits() & Self::MASK_PHY_ERRSOTSYNCHS_0_MASK) != 0
    }

    /// Set MASK_PHY_ERRSOTSYNCHS_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_phy_errsotsynchs_0(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_PHY_ERRSOTSYNCHS_0_MASK);
    }

    /// Clear MASK_PHY_ERRSOTSYNCHS_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_phy_errsotsynchs_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_PHY_ERRSOTSYNCHS_0_MASK);
    }
}

/// INT_MSK_PHY_FATAL value type for batch read/modify/write operations
///
/// Use `intmskphyfatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intmskphyfatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntMskPhyFatalVal {
    bits: u32,
}

impl IntMskPhyFatalVal {
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

    /// Read **MASK_PHY_ERRSOTSYNCHS_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_phy_errsotsynchs_1(&self) -> bool {
        (self.bits & IntMskPhyFatal::MASK_PHY_ERRSOTSYNCHS_1_MASK) != 0
    }

    /// Read **MASK_PHY_ERRSOTSYNCHS_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_phy_errsotsynchs_0(&self) -> bool {
        (self.bits & IntMskPhyFatal::MASK_PHY_ERRSOTSYNCHS_0_MASK) != 0
    }

    /// Set **MASK_PHY_ERRSOTSYNCHS_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_phy_errsotsynchs_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPhyFatal::MASK_PHY_ERRSOTSYNCHS_1_MASK
            } else {
                self.bits & !IntMskPhyFatal::MASK_PHY_ERRSOTSYNCHS_1_MASK
            },
        }
    }

    /// Set **MASK_PHY_ERRSOTSYNCHS_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_phy_errsotsynchs_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPhyFatal::MASK_PHY_ERRSOTSYNCHS_0_MASK
            } else {
                self.bits & !IntMskPhyFatal::MASK_PHY_ERRSOTSYNCHS_0_MASK
            },
        }
    }
}

impl Default for IntMskPhyFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntMskPhyFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntMskPhyFatalVal {
        IntMskPhyFatalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntMskPhyFatalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntMskPhyFatalVal) -> IntMskPhyFatalVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_FORCE_PHY_FATAL Register
///
/// NA
///
/// Address offset: 0x00e8
///
/// ## Fields
///
/// - **FORCE_PHY_ERRSOTSYNCHS_1** `[1]` (RW)
///
///   NA
///
/// - **FORCE_PHY_ERRSOTSYNCHS_0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntForcePhyFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntForcePhyFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntForcePhyFatal {
    // ======== Field Constants ========

    /// **FORCE_PHY_ERRSOTSYNCHS_1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FORCE_PHY_ERRSOTSYNCHS_1_OFFSET: u32 = 1;
    pub const FORCE_PHY_ERRSOTSYNCHS_1_MASK: u32 = 0x00000002;
    pub const FORCE_PHY_ERRSOTSYNCHS_1: u32 = 1 << 1;

    /// **FORCE_PHY_ERRSOTSYNCHS_0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FORCE_PHY_ERRSOTSYNCHS_0_OFFSET: u32 = 0;
    pub const FORCE_PHY_ERRSOTSYNCHS_0_MASK: u32 = 0x00000001;
    pub const FORCE_PHY_ERRSOTSYNCHS_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FORCE_PHY_ERRSOTSYNCHS_1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_phy_errsotsynchs_1(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_PHY_ERRSOTSYNCHS_1_MASK) != 0
    }

    /// Set FORCE_PHY_ERRSOTSYNCHS_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_phy_errsotsynchs_1(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_PHY_ERRSOTSYNCHS_1_MASK);
    }

    /// Clear FORCE_PHY_ERRSOTSYNCHS_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_phy_errsotsynchs_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_PHY_ERRSOTSYNCHS_1_MASK);
    }

    /// **FORCE_PHY_ERRSOTSYNCHS_0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_phy_errsotsynchs_0(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_PHY_ERRSOTSYNCHS_0_MASK) != 0
    }

    /// Set FORCE_PHY_ERRSOTSYNCHS_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_phy_errsotsynchs_0(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_PHY_ERRSOTSYNCHS_0_MASK);
    }

    /// Clear FORCE_PHY_ERRSOTSYNCHS_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_phy_errsotsynchs_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_PHY_ERRSOTSYNCHS_0_MASK);
    }
}

/// INT_FORCE_PHY_FATAL value type for batch read/modify/write operations
///
/// Use `intforcephyfatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intforcephyfatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntForcePhyFatalVal {
    bits: u32,
}

impl IntForcePhyFatalVal {
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

    /// Read **FORCE_PHY_ERRSOTSYNCHS_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_phy_errsotsynchs_1(&self) -> bool {
        (self.bits & IntForcePhyFatal::FORCE_PHY_ERRSOTSYNCHS_1_MASK) != 0
    }

    /// Read **FORCE_PHY_ERRSOTSYNCHS_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_phy_errsotsynchs_0(&self) -> bool {
        (self.bits & IntForcePhyFatal::FORCE_PHY_ERRSOTSYNCHS_0_MASK) != 0
    }

    /// Set **FORCE_PHY_ERRSOTSYNCHS_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_phy_errsotsynchs_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePhyFatal::FORCE_PHY_ERRSOTSYNCHS_1_MASK
            } else {
                self.bits & !IntForcePhyFatal::FORCE_PHY_ERRSOTSYNCHS_1_MASK
            },
        }
    }

    /// Set **FORCE_PHY_ERRSOTSYNCHS_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_phy_errsotsynchs_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePhyFatal::FORCE_PHY_ERRSOTSYNCHS_0_MASK
            } else {
                self.bits & !IntForcePhyFatal::FORCE_PHY_ERRSOTSYNCHS_0_MASK
            },
        }
    }
}

impl Default for IntForcePhyFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntForcePhyFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntForcePhyFatalVal {
        IntForcePhyFatalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntForcePhyFatalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntForcePhyFatalVal) -> IntForcePhyFatalVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ST_PKT_FATAL Register
///
/// NA
///
/// Address offset: 0x00f0
///
/// ## Fields
///
/// - **ST_SHORTER_PAYLOAD** `[1]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_DOUBLE** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntStPktFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntStPktFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntStPktFatal {
    // ======== Field Constants ========

    /// **ST_SHORTER_PAYLOAD**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ST_SHORTER_PAYLOAD_OFFSET: u32 = 1;
    pub const ST_SHORTER_PAYLOAD_MASK: u32 = 0x00000002;
    pub const ST_SHORTER_PAYLOAD: u32 = 1 << 1;

    /// **ST_ERR_ECC_DOUBLE**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ST_ERR_ECC_DOUBLE_OFFSET: u32 = 0;
    pub const ST_ERR_ECC_DOUBLE_MASK: u32 = 0x00000001;
    pub const ST_ERR_ECC_DOUBLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ST_SHORTER_PAYLOAD** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_shorter_payload(&self) -> bool {
        (self.value.read_bits() & Self::ST_SHORTER_PAYLOAD_MASK) != 0
    }

    /// **ST_ERR_ECC_DOUBLE** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_double(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_DOUBLE_MASK) != 0
    }
}

/// INT_ST_PKT_FATAL value type for batch read/modify/write operations
///
/// Use `intstpktfatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intstpktfatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStPktFatalVal {
    bits: u32,
}

impl IntStPktFatalVal {
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

    /// Read **ST_SHORTER_PAYLOAD** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_shorter_payload(&self) -> bool {
        (self.bits & IntStPktFatal::ST_SHORTER_PAYLOAD_MASK) != 0
    }

    /// Read **ST_ERR_ECC_DOUBLE** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_double(&self) -> bool {
        (self.bits & IntStPktFatal::ST_ERR_ECC_DOUBLE_MASK) != 0
    }
}

impl Default for IntStPktFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntStPktFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStPktFatalVal {
        IntStPktFatalVal::from_bits(self.value.read_bits())
    }
}

/// INT_MSK_PKT_FATAL Register
///
/// NA
///
/// Address offset: 0x00f4
///
/// ## Fields
///
/// - **MASK_SHORTER_PAYLOAD** `[1]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_DOUBLE** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntMskPktFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntMskPktFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntMskPktFatal {
    // ======== Field Constants ========

    /// **MASK_SHORTER_PAYLOAD**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MASK_SHORTER_PAYLOAD_OFFSET: u32 = 1;
    pub const MASK_SHORTER_PAYLOAD_MASK: u32 = 0x00000002;
    pub const MASK_SHORTER_PAYLOAD: u32 = 1 << 1;

    /// **MASK_ERR_ECC_DOUBLE**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MASK_ERR_ECC_DOUBLE_OFFSET: u32 = 0;
    pub const MASK_ERR_ECC_DOUBLE_MASK: u32 = 0x00000001;
    pub const MASK_ERR_ECC_DOUBLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MASK_SHORTER_PAYLOAD** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_shorter_payload(&self) -> bool {
        (self.value.read_bits() & Self::MASK_SHORTER_PAYLOAD_MASK) != 0
    }

    /// Set MASK_SHORTER_PAYLOAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_shorter_payload(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_SHORTER_PAYLOAD_MASK);
    }

    /// Clear MASK_SHORTER_PAYLOAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_shorter_payload(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_SHORTER_PAYLOAD_MASK);
    }

    /// **MASK_ERR_ECC_DOUBLE** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_double(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_DOUBLE_MASK) != 0
    }

    /// Set MASK_ERR_ECC_DOUBLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_double(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_DOUBLE_MASK);
    }

    /// Clear MASK_ERR_ECC_DOUBLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_double(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_DOUBLE_MASK);
    }
}

/// INT_MSK_PKT_FATAL value type for batch read/modify/write operations
///
/// Use `intmskpktfatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intmskpktfatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntMskPktFatalVal {
    bits: u32,
}

impl IntMskPktFatalVal {
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

    /// Read **MASK_SHORTER_PAYLOAD** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_shorter_payload(&self) -> bool {
        (self.bits & IntMskPktFatal::MASK_SHORTER_PAYLOAD_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_DOUBLE** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_double(&self) -> bool {
        (self.bits & IntMskPktFatal::MASK_ERR_ECC_DOUBLE_MASK) != 0
    }

    /// Set **MASK_SHORTER_PAYLOAD** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_shorter_payload(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPktFatal::MASK_SHORTER_PAYLOAD_MASK
            } else {
                self.bits & !IntMskPktFatal::MASK_SHORTER_PAYLOAD_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_DOUBLE** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_double(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPktFatal::MASK_ERR_ECC_DOUBLE_MASK
            } else {
                self.bits & !IntMskPktFatal::MASK_ERR_ECC_DOUBLE_MASK
            },
        }
    }
}

impl Default for IntMskPktFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntMskPktFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntMskPktFatalVal {
        IntMskPktFatalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntMskPktFatalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntMskPktFatalVal) -> IntMskPktFatalVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_FORCE_PKT_FATAL Register
///
/// NA
///
/// Address offset: 0x00f8
///
/// ## Fields
///
/// - **FORCE_SHORTER_PAYLOAD** `[1]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_DOUBLE** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntForcePktFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntForcePktFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntForcePktFatal {
    // ======== Field Constants ========

    /// **FORCE_SHORTER_PAYLOAD**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FORCE_SHORTER_PAYLOAD_OFFSET: u32 = 1;
    pub const FORCE_SHORTER_PAYLOAD_MASK: u32 = 0x00000002;
    pub const FORCE_SHORTER_PAYLOAD: u32 = 1 << 1;

    /// **FORCE_ERR_ECC_DOUBLE**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FORCE_ERR_ECC_DOUBLE_OFFSET: u32 = 0;
    pub const FORCE_ERR_ECC_DOUBLE_MASK: u32 = 0x00000001;
    pub const FORCE_ERR_ECC_DOUBLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FORCE_SHORTER_PAYLOAD** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_shorter_payload(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_SHORTER_PAYLOAD_MASK) != 0
    }

    /// Set FORCE_SHORTER_PAYLOAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_shorter_payload(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_SHORTER_PAYLOAD_MASK);
    }

    /// Clear FORCE_SHORTER_PAYLOAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_shorter_payload(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_SHORTER_PAYLOAD_MASK);
    }

    /// **FORCE_ERR_ECC_DOUBLE** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_double(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_DOUBLE_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_DOUBLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_double(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_DOUBLE_MASK);
    }

    /// Clear FORCE_ERR_ECC_DOUBLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_double(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_DOUBLE_MASK);
    }
}

/// INT_FORCE_PKT_FATAL value type for batch read/modify/write operations
///
/// Use `intforcepktfatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intforcepktfatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntForcePktFatalVal {
    bits: u32,
}

impl IntForcePktFatalVal {
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

    /// Read **FORCE_SHORTER_PAYLOAD** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_shorter_payload(&self) -> bool {
        (self.bits & IntForcePktFatal::FORCE_SHORTER_PAYLOAD_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_DOUBLE** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_double(&self) -> bool {
        (self.bits & IntForcePktFatal::FORCE_ERR_ECC_DOUBLE_MASK) != 0
    }

    /// Set **FORCE_SHORTER_PAYLOAD** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_shorter_payload(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePktFatal::FORCE_SHORTER_PAYLOAD_MASK
            } else {
                self.bits & !IntForcePktFatal::FORCE_SHORTER_PAYLOAD_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_DOUBLE** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_double(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePktFatal::FORCE_ERR_ECC_DOUBLE_MASK
            } else {
                self.bits & !IntForcePktFatal::FORCE_ERR_ECC_DOUBLE_MASK
            },
        }
    }
}

impl Default for IntForcePktFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntForcePktFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntForcePktFatalVal {
        IntForcePktFatalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntForcePktFatalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntForcePktFatalVal) -> IntForcePktFatalVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ST_PHY Register
///
/// NA
///
/// Address offset: 0x0110
///
/// ## Fields
///
/// - **ST_PHY_ERRESC_1** `[17]` (RO)
///
///   NA
///
/// - **ST_PHY_ERRESC_0** `[16]` (RO)
///
///   NA
///
/// - **ST_PHY_ERRSOTHS_1** `[1]` (RO)
///
///   NA
///
/// - **ST_PHY_ERRSOTHS_0** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntStPhy {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntStPhy {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntStPhy {
    // ======== Field Constants ========

    /// **ST_PHY_ERRESC_1**
    ///
    /// NA
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const ST_PHY_ERRESC_1_OFFSET: u32 = 17;
    pub const ST_PHY_ERRESC_1_MASK: u32 = 0x00020000;
    pub const ST_PHY_ERRESC_1: u32 = 1 << 17;

    /// **ST_PHY_ERRESC_0**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ST_PHY_ERRESC_0_OFFSET: u32 = 16;
    pub const ST_PHY_ERRESC_0_MASK: u32 = 0x00010000;
    pub const ST_PHY_ERRESC_0: u32 = 1 << 16;

    /// **ST_PHY_ERRSOTHS_1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ST_PHY_ERRSOTHS_1_OFFSET: u32 = 1;
    pub const ST_PHY_ERRSOTHS_1_MASK: u32 = 0x00000002;
    pub const ST_PHY_ERRSOTHS_1: u32 = 1 << 1;

    /// **ST_PHY_ERRSOTHS_0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ST_PHY_ERRSOTHS_0_OFFSET: u32 = 0;
    pub const ST_PHY_ERRSOTHS_0_MASK: u32 = 0x00000001;
    pub const ST_PHY_ERRSOTHS_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ST_PHY_ERRESC_1** `[17]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_phy_erresc_1(&self) -> bool {
        (self.value.read_bits() & Self::ST_PHY_ERRESC_1_MASK) != 0
    }

    /// **ST_PHY_ERRESC_0** `[16]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_phy_erresc_0(&self) -> bool {
        (self.value.read_bits() & Self::ST_PHY_ERRESC_0_MASK) != 0
    }

    /// **ST_PHY_ERRSOTHS_1** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_phy_errsoths_1(&self) -> bool {
        (self.value.read_bits() & Self::ST_PHY_ERRSOTHS_1_MASK) != 0
    }

    /// **ST_PHY_ERRSOTHS_0** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_phy_errsoths_0(&self) -> bool {
        (self.value.read_bits() & Self::ST_PHY_ERRSOTHS_0_MASK) != 0
    }
}

/// INT_ST_PHY value type for batch read/modify/write operations
///
/// Use `intstphy.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intstphy.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStPhyVal {
    bits: u32,
}

impl IntStPhyVal {
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

    /// Read **ST_PHY_ERRESC_1** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_phy_erresc_1(&self) -> bool {
        (self.bits & IntStPhy::ST_PHY_ERRESC_1_MASK) != 0
    }

    /// Read **ST_PHY_ERRESC_0** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_phy_erresc_0(&self) -> bool {
        (self.bits & IntStPhy::ST_PHY_ERRESC_0_MASK) != 0
    }

    /// Read **ST_PHY_ERRSOTHS_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_phy_errsoths_1(&self) -> bool {
        (self.bits & IntStPhy::ST_PHY_ERRSOTHS_1_MASK) != 0
    }

    /// Read **ST_PHY_ERRSOTHS_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_phy_errsoths_0(&self) -> bool {
        (self.bits & IntStPhy::ST_PHY_ERRSOTHS_0_MASK) != 0
    }
}

impl Default for IntStPhyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntStPhy {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStPhyVal {
        IntStPhyVal::from_bits(self.value.read_bits())
    }
}

/// INT_MSK_PHY Register
///
/// NA
///
/// Address offset: 0x0114
///
/// ## Fields
///
/// - **MASK_PHY_ERRESC_1** `[17]` (RW)
///
///   NA
///
/// - **MASK_PHY_ERRESC_0** `[16]` (RW)
///
///   NA
///
/// - **MASK_PHY_ERRSOTHS_1** `[1]` (RW)
///
///   NA
///
/// - **MASK_PHY_ERRSOTHS_0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntMskPhy {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntMskPhy {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntMskPhy {
    // ======== Field Constants ========

    /// **MASK_PHY_ERRESC_1**
    ///
    /// NA
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const MASK_PHY_ERRESC_1_OFFSET: u32 = 17;
    pub const MASK_PHY_ERRESC_1_MASK: u32 = 0x00020000;
    pub const MASK_PHY_ERRESC_1: u32 = 1 << 17;

    /// **MASK_PHY_ERRESC_0**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const MASK_PHY_ERRESC_0_OFFSET: u32 = 16;
    pub const MASK_PHY_ERRESC_0_MASK: u32 = 0x00010000;
    pub const MASK_PHY_ERRESC_0: u32 = 1 << 16;

    /// **MASK_PHY_ERRSOTHS_1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MASK_PHY_ERRSOTHS_1_OFFSET: u32 = 1;
    pub const MASK_PHY_ERRSOTHS_1_MASK: u32 = 0x00000002;
    pub const MASK_PHY_ERRSOTHS_1: u32 = 1 << 1;

    /// **MASK_PHY_ERRSOTHS_0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MASK_PHY_ERRSOTHS_0_OFFSET: u32 = 0;
    pub const MASK_PHY_ERRSOTHS_0_MASK: u32 = 0x00000001;
    pub const MASK_PHY_ERRSOTHS_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MASK_PHY_ERRESC_1** `[17]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_phy_erresc_1(&self) -> bool {
        (self.value.read_bits() & Self::MASK_PHY_ERRESC_1_MASK) != 0
    }

    /// Set MASK_PHY_ERRESC_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_phy_erresc_1(&self) {
        self.value.modify_bits(|r| r | Self::MASK_PHY_ERRESC_1_MASK);
    }

    /// Clear MASK_PHY_ERRESC_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_phy_erresc_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_PHY_ERRESC_1_MASK);
    }

    /// **MASK_PHY_ERRESC_0** `[16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_phy_erresc_0(&self) -> bool {
        (self.value.read_bits() & Self::MASK_PHY_ERRESC_0_MASK) != 0
    }

    /// Set MASK_PHY_ERRESC_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_phy_erresc_0(&self) {
        self.value.modify_bits(|r| r | Self::MASK_PHY_ERRESC_0_MASK);
    }

    /// Clear MASK_PHY_ERRESC_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_phy_erresc_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_PHY_ERRESC_0_MASK);
    }

    /// **MASK_PHY_ERRSOTHS_1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_phy_errsoths_1(&self) -> bool {
        (self.value.read_bits() & Self::MASK_PHY_ERRSOTHS_1_MASK) != 0
    }

    /// Set MASK_PHY_ERRSOTHS_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_phy_errsoths_1(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_PHY_ERRSOTHS_1_MASK);
    }

    /// Clear MASK_PHY_ERRSOTHS_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_phy_errsoths_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_PHY_ERRSOTHS_1_MASK);
    }

    /// **MASK_PHY_ERRSOTHS_0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_phy_errsoths_0(&self) -> bool {
        (self.value.read_bits() & Self::MASK_PHY_ERRSOTHS_0_MASK) != 0
    }

    /// Set MASK_PHY_ERRSOTHS_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_phy_errsoths_0(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_PHY_ERRSOTHS_0_MASK);
    }

    /// Clear MASK_PHY_ERRSOTHS_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_phy_errsoths_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_PHY_ERRSOTHS_0_MASK);
    }
}

/// INT_MSK_PHY value type for batch read/modify/write operations
///
/// Use `intmskphy.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intmskphy.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntMskPhyVal {
    bits: u32,
}

impl IntMskPhyVal {
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

    /// Read **MASK_PHY_ERRESC_1** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_phy_erresc_1(&self) -> bool {
        (self.bits & IntMskPhy::MASK_PHY_ERRESC_1_MASK) != 0
    }

    /// Read **MASK_PHY_ERRESC_0** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_phy_erresc_0(&self) -> bool {
        (self.bits & IntMskPhy::MASK_PHY_ERRESC_0_MASK) != 0
    }

    /// Read **MASK_PHY_ERRSOTHS_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_phy_errsoths_1(&self) -> bool {
        (self.bits & IntMskPhy::MASK_PHY_ERRSOTHS_1_MASK) != 0
    }

    /// Read **MASK_PHY_ERRSOTHS_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_phy_errsoths_0(&self) -> bool {
        (self.bits & IntMskPhy::MASK_PHY_ERRSOTHS_0_MASK) != 0
    }

    /// Set **MASK_PHY_ERRESC_1** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_phy_erresc_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPhy::MASK_PHY_ERRESC_1_MASK
            } else {
                self.bits & !IntMskPhy::MASK_PHY_ERRESC_1_MASK
            },
        }
    }

    /// Set **MASK_PHY_ERRESC_0** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_phy_erresc_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPhy::MASK_PHY_ERRESC_0_MASK
            } else {
                self.bits & !IntMskPhy::MASK_PHY_ERRESC_0_MASK
            },
        }
    }

    /// Set **MASK_PHY_ERRSOTHS_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_phy_errsoths_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPhy::MASK_PHY_ERRSOTHS_1_MASK
            } else {
                self.bits & !IntMskPhy::MASK_PHY_ERRSOTHS_1_MASK
            },
        }
    }

    /// Set **MASK_PHY_ERRSOTHS_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_phy_errsoths_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPhy::MASK_PHY_ERRSOTHS_0_MASK
            } else {
                self.bits & !IntMskPhy::MASK_PHY_ERRSOTHS_0_MASK
            },
        }
    }
}

impl Default for IntMskPhyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntMskPhy {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntMskPhyVal {
        IntMskPhyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntMskPhyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntMskPhyVal) -> IntMskPhyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_FORCE_PHY Register
///
/// NA
///
/// Address offset: 0x0118
///
/// ## Fields
///
/// - **FORCE_PHY_ERRESC_1** `[17]` (RW)
///
///   NA
///
/// - **FORCE_PHY_ERRESC_0** `[16]` (RW)
///
///   NA
///
/// - **FORCE_PHY_ERRSOTHS_1** `[1]` (RW)
///
///   NA
///
/// - **FORCE_PHY_ERRSOTHS_0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntForcePhy {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntForcePhy {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntForcePhy {
    // ======== Field Constants ========

    /// **FORCE_PHY_ERRESC_1**
    ///
    /// NA
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const FORCE_PHY_ERRESC_1_OFFSET: u32 = 17;
    pub const FORCE_PHY_ERRESC_1_MASK: u32 = 0x00020000;
    pub const FORCE_PHY_ERRESC_1: u32 = 1 << 17;

    /// **FORCE_PHY_ERRESC_0**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const FORCE_PHY_ERRESC_0_OFFSET: u32 = 16;
    pub const FORCE_PHY_ERRESC_0_MASK: u32 = 0x00010000;
    pub const FORCE_PHY_ERRESC_0: u32 = 1 << 16;

    /// **FORCE_PHY_ERRSOTHS_1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FORCE_PHY_ERRSOTHS_1_OFFSET: u32 = 1;
    pub const FORCE_PHY_ERRSOTHS_1_MASK: u32 = 0x00000002;
    pub const FORCE_PHY_ERRSOTHS_1: u32 = 1 << 1;

    /// **FORCE_PHY_ERRSOTHS_0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FORCE_PHY_ERRSOTHS_0_OFFSET: u32 = 0;
    pub const FORCE_PHY_ERRSOTHS_0_MASK: u32 = 0x00000001;
    pub const FORCE_PHY_ERRSOTHS_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FORCE_PHY_ERRESC_1** `[17]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_phy_erresc_1(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_PHY_ERRESC_1_MASK) != 0
    }

    /// Set FORCE_PHY_ERRESC_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_phy_erresc_1(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_PHY_ERRESC_1_MASK);
    }

    /// Clear FORCE_PHY_ERRESC_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_phy_erresc_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_PHY_ERRESC_1_MASK);
    }

    /// **FORCE_PHY_ERRESC_0** `[16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_phy_erresc_0(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_PHY_ERRESC_0_MASK) != 0
    }

    /// Set FORCE_PHY_ERRESC_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_phy_erresc_0(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_PHY_ERRESC_0_MASK);
    }

    /// Clear FORCE_PHY_ERRESC_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_phy_erresc_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_PHY_ERRESC_0_MASK);
    }

    /// **FORCE_PHY_ERRSOTHS_1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_phy_errsoths_1(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_PHY_ERRSOTHS_1_MASK) != 0
    }

    /// Set FORCE_PHY_ERRSOTHS_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_phy_errsoths_1(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_PHY_ERRSOTHS_1_MASK);
    }

    /// Clear FORCE_PHY_ERRSOTHS_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_phy_errsoths_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_PHY_ERRSOTHS_1_MASK);
    }

    /// **FORCE_PHY_ERRSOTHS_0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_phy_errsoths_0(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_PHY_ERRSOTHS_0_MASK) != 0
    }

    /// Set FORCE_PHY_ERRSOTHS_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_phy_errsoths_0(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_PHY_ERRSOTHS_0_MASK);
    }

    /// Clear FORCE_PHY_ERRSOTHS_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_phy_errsoths_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_PHY_ERRSOTHS_0_MASK);
    }
}

/// INT_FORCE_PHY value type for batch read/modify/write operations
///
/// Use `intforcephy.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intforcephy.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntForcePhyVal {
    bits: u32,
}

impl IntForcePhyVal {
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

    /// Read **FORCE_PHY_ERRESC_1** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_phy_erresc_1(&self) -> bool {
        (self.bits & IntForcePhy::FORCE_PHY_ERRESC_1_MASK) != 0
    }

    /// Read **FORCE_PHY_ERRESC_0** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_phy_erresc_0(&self) -> bool {
        (self.bits & IntForcePhy::FORCE_PHY_ERRESC_0_MASK) != 0
    }

    /// Read **FORCE_PHY_ERRSOTHS_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_phy_errsoths_1(&self) -> bool {
        (self.bits & IntForcePhy::FORCE_PHY_ERRSOTHS_1_MASK) != 0
    }

    /// Read **FORCE_PHY_ERRSOTHS_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_phy_errsoths_0(&self) -> bool {
        (self.bits & IntForcePhy::FORCE_PHY_ERRSOTHS_0_MASK) != 0
    }

    /// Set **FORCE_PHY_ERRESC_1** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_phy_erresc_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePhy::FORCE_PHY_ERRESC_1_MASK
            } else {
                self.bits & !IntForcePhy::FORCE_PHY_ERRESC_1_MASK
            },
        }
    }

    /// Set **FORCE_PHY_ERRESC_0** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_phy_erresc_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePhy::FORCE_PHY_ERRESC_0_MASK
            } else {
                self.bits & !IntForcePhy::FORCE_PHY_ERRESC_0_MASK
            },
        }
    }

    /// Set **FORCE_PHY_ERRSOTHS_1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_phy_errsoths_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePhy::FORCE_PHY_ERRSOTHS_1_MASK
            } else {
                self.bits & !IntForcePhy::FORCE_PHY_ERRSOTHS_1_MASK
            },
        }
    }

    /// Set **FORCE_PHY_ERRSOTHS_0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_phy_errsoths_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePhy::FORCE_PHY_ERRSOTHS_0_MASK
            } else {
                self.bits & !IntForcePhy::FORCE_PHY_ERRSOTHS_0_MASK
            },
        }
    }
}

impl Default for IntForcePhyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntForcePhy {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntForcePhyVal {
        IntForcePhyVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntForcePhyVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntForcePhyVal) -> IntForcePhyVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ST_BNDRY_FRAME_FATAL Register
///
/// NA
///
/// Address offset: 0x0280
///
/// ## Fields
///
/// - **ST_ERR_F_BNDRY_MATCH_VC15** `[15]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC14** `[14]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC13** `[13]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC12** `[12]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC11** `[11]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC10** `[10]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC9** `[9]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC8** `[8]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC7** `[7]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC6** `[6]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC5** `[5]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC4** `[4]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC3** `[3]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC2** `[2]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC1** `[1]` (RO)
///
///   NA
///
/// - **ST_ERR_F_BNDRY_MATCH_VC0** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntStBndryFrameFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntStBndryFrameFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntStBndryFrameFatal {
    // ======== Field Constants ========

    /// **ST_ERR_F_BNDRY_MATCH_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const ST_ERR_F_BNDRY_MATCH_VC15_OFFSET: u32 = 15;
    pub const ST_ERR_F_BNDRY_MATCH_VC15_MASK: u32 = 0x00008000;
    pub const ST_ERR_F_BNDRY_MATCH_VC15: u32 = 1 << 15;

    /// **ST_ERR_F_BNDRY_MATCH_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const ST_ERR_F_BNDRY_MATCH_VC14_OFFSET: u32 = 14;
    pub const ST_ERR_F_BNDRY_MATCH_VC14_MASK: u32 = 0x00004000;
    pub const ST_ERR_F_BNDRY_MATCH_VC14: u32 = 1 << 14;

    /// **ST_ERR_F_BNDRY_MATCH_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const ST_ERR_F_BNDRY_MATCH_VC13_OFFSET: u32 = 13;
    pub const ST_ERR_F_BNDRY_MATCH_VC13_MASK: u32 = 0x00002000;
    pub const ST_ERR_F_BNDRY_MATCH_VC13: u32 = 1 << 13;

    /// **ST_ERR_F_BNDRY_MATCH_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const ST_ERR_F_BNDRY_MATCH_VC12_OFFSET: u32 = 12;
    pub const ST_ERR_F_BNDRY_MATCH_VC12_MASK: u32 = 0x00001000;
    pub const ST_ERR_F_BNDRY_MATCH_VC12: u32 = 1 << 12;

    /// **ST_ERR_F_BNDRY_MATCH_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ST_ERR_F_BNDRY_MATCH_VC11_OFFSET: u32 = 11;
    pub const ST_ERR_F_BNDRY_MATCH_VC11_MASK: u32 = 0x00000800;
    pub const ST_ERR_F_BNDRY_MATCH_VC11: u32 = 1 << 11;

    /// **ST_ERR_F_BNDRY_MATCH_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ST_ERR_F_BNDRY_MATCH_VC10_OFFSET: u32 = 10;
    pub const ST_ERR_F_BNDRY_MATCH_VC10_MASK: u32 = 0x00000400;
    pub const ST_ERR_F_BNDRY_MATCH_VC10: u32 = 1 << 10;

    /// **ST_ERR_F_BNDRY_MATCH_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ST_ERR_F_BNDRY_MATCH_VC9_OFFSET: u32 = 9;
    pub const ST_ERR_F_BNDRY_MATCH_VC9_MASK: u32 = 0x00000200;
    pub const ST_ERR_F_BNDRY_MATCH_VC9: u32 = 1 << 9;

    /// **ST_ERR_F_BNDRY_MATCH_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ST_ERR_F_BNDRY_MATCH_VC8_OFFSET: u32 = 8;
    pub const ST_ERR_F_BNDRY_MATCH_VC8_MASK: u32 = 0x00000100;
    pub const ST_ERR_F_BNDRY_MATCH_VC8: u32 = 1 << 8;

    /// **ST_ERR_F_BNDRY_MATCH_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ST_ERR_F_BNDRY_MATCH_VC7_OFFSET: u32 = 7;
    pub const ST_ERR_F_BNDRY_MATCH_VC7_MASK: u32 = 0x00000080;
    pub const ST_ERR_F_BNDRY_MATCH_VC7: u32 = 1 << 7;

    /// **ST_ERR_F_BNDRY_MATCH_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ST_ERR_F_BNDRY_MATCH_VC6_OFFSET: u32 = 6;
    pub const ST_ERR_F_BNDRY_MATCH_VC6_MASK: u32 = 0x00000040;
    pub const ST_ERR_F_BNDRY_MATCH_VC6: u32 = 1 << 6;

    /// **ST_ERR_F_BNDRY_MATCH_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ST_ERR_F_BNDRY_MATCH_VC5_OFFSET: u32 = 5;
    pub const ST_ERR_F_BNDRY_MATCH_VC5_MASK: u32 = 0x00000020;
    pub const ST_ERR_F_BNDRY_MATCH_VC5: u32 = 1 << 5;

    /// **ST_ERR_F_BNDRY_MATCH_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ST_ERR_F_BNDRY_MATCH_VC4_OFFSET: u32 = 4;
    pub const ST_ERR_F_BNDRY_MATCH_VC4_MASK: u32 = 0x00000010;
    pub const ST_ERR_F_BNDRY_MATCH_VC4: u32 = 1 << 4;

    /// **ST_ERR_F_BNDRY_MATCH_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ST_ERR_F_BNDRY_MATCH_VC3_OFFSET: u32 = 3;
    pub const ST_ERR_F_BNDRY_MATCH_VC3_MASK: u32 = 0x00000008;
    pub const ST_ERR_F_BNDRY_MATCH_VC3: u32 = 1 << 3;

    /// **ST_ERR_F_BNDRY_MATCH_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ST_ERR_F_BNDRY_MATCH_VC2_OFFSET: u32 = 2;
    pub const ST_ERR_F_BNDRY_MATCH_VC2_MASK: u32 = 0x00000004;
    pub const ST_ERR_F_BNDRY_MATCH_VC2: u32 = 1 << 2;

    /// **ST_ERR_F_BNDRY_MATCH_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ST_ERR_F_BNDRY_MATCH_VC1_OFFSET: u32 = 1;
    pub const ST_ERR_F_BNDRY_MATCH_VC1_MASK: u32 = 0x00000002;
    pub const ST_ERR_F_BNDRY_MATCH_VC1: u32 = 1 << 1;

    /// **ST_ERR_F_BNDRY_MATCH_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ST_ERR_F_BNDRY_MATCH_VC0_OFFSET: u32 = 0;
    pub const ST_ERR_F_BNDRY_MATCH_VC0_MASK: u32 = 0x00000001;
    pub const ST_ERR_F_BNDRY_MATCH_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ST_ERR_F_BNDRY_MATCH_VC15** `[15]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc15(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC15_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC14** `[14]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc14(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC14_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC13** `[13]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc13(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC13_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC12** `[12]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc12(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC12_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC11** `[11]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc11(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC11_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC10** `[10]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc10(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC10_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC9** `[9]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc9(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC9_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC8** `[8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc8(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC8_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC7** `[7]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc7(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC7_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC6** `[6]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc6(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC6_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC5** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc5(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC5_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC4** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc4(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC4_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC3** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc3(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC3_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC2** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc2(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC2_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC1** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc1(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC1_MASK) != 0
    }

    /// **ST_ERR_F_BNDRY_MATCH_VC0** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_bndry_match_vc0(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_BNDRY_MATCH_VC0_MASK) != 0
    }
}

/// INT_ST_BNDRY_FRAME_FATAL value type for batch read/modify/write operations
///
/// Use `intstbndryframefatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intstbndryframefatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStBndryFrameFatalVal {
    bits: u32,
}

impl IntStBndryFrameFatalVal {
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

    /// Read **ST_ERR_F_BNDRY_MATCH_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc15(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC15_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc14(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC14_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc13(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC13_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc12(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC12_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc11(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC11_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc10(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC10_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc9(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC9_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc8(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC8_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc7(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC7_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc6(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC6_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc5(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC5_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc4(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC4_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc3(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC3_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc2(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC2_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc1(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC1_MASK) != 0
    }

    /// Read **ST_ERR_F_BNDRY_MATCH_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_bndry_match_vc0(&self) -> bool {
        (self.bits & IntStBndryFrameFatal::ST_ERR_F_BNDRY_MATCH_VC0_MASK) != 0
    }
}

impl Default for IntStBndryFrameFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntStBndryFrameFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStBndryFrameFatalVal {
        IntStBndryFrameFatalVal::from_bits(self.value.read_bits())
    }
}

/// INT_MSK_BNDRY_FRAME_FATAL Register
///
/// NA
///
/// Address offset: 0x0284
///
/// ## Fields
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC15** `[15]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC14** `[14]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC13** `[13]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC12** `[12]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC11** `[11]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC10** `[10]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC9** `[9]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC8** `[8]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC7** `[7]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC6** `[6]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC5** `[5]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC4** `[4]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC3** `[3]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC2** `[2]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC1** `[1]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_BNDRY_MATCH_VC0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntMskBndryFrameFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntMskBndryFrameFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntMskBndryFrameFatal {
    // ======== Field Constants ========

    /// **MASK_ERR_F_BNDRY_MATCH_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const MASK_ERR_F_BNDRY_MATCH_VC15_OFFSET: u32 = 15;
    pub const MASK_ERR_F_BNDRY_MATCH_VC15_MASK: u32 = 0x00008000;
    pub const MASK_ERR_F_BNDRY_MATCH_VC15: u32 = 1 << 15;

    /// **MASK_ERR_F_BNDRY_MATCH_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MASK_ERR_F_BNDRY_MATCH_VC14_OFFSET: u32 = 14;
    pub const MASK_ERR_F_BNDRY_MATCH_VC14_MASK: u32 = 0x00004000;
    pub const MASK_ERR_F_BNDRY_MATCH_VC14: u32 = 1 << 14;

    /// **MASK_ERR_F_BNDRY_MATCH_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const MASK_ERR_F_BNDRY_MATCH_VC13_OFFSET: u32 = 13;
    pub const MASK_ERR_F_BNDRY_MATCH_VC13_MASK: u32 = 0x00002000;
    pub const MASK_ERR_F_BNDRY_MATCH_VC13: u32 = 1 << 13;

    /// **MASK_ERR_F_BNDRY_MATCH_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const MASK_ERR_F_BNDRY_MATCH_VC12_OFFSET: u32 = 12;
    pub const MASK_ERR_F_BNDRY_MATCH_VC12_MASK: u32 = 0x00001000;
    pub const MASK_ERR_F_BNDRY_MATCH_VC12: u32 = 1 << 12;

    /// **MASK_ERR_F_BNDRY_MATCH_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const MASK_ERR_F_BNDRY_MATCH_VC11_OFFSET: u32 = 11;
    pub const MASK_ERR_F_BNDRY_MATCH_VC11_MASK: u32 = 0x00000800;
    pub const MASK_ERR_F_BNDRY_MATCH_VC11: u32 = 1 << 11;

    /// **MASK_ERR_F_BNDRY_MATCH_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const MASK_ERR_F_BNDRY_MATCH_VC10_OFFSET: u32 = 10;
    pub const MASK_ERR_F_BNDRY_MATCH_VC10_MASK: u32 = 0x00000400;
    pub const MASK_ERR_F_BNDRY_MATCH_VC10: u32 = 1 << 10;

    /// **MASK_ERR_F_BNDRY_MATCH_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const MASK_ERR_F_BNDRY_MATCH_VC9_OFFSET: u32 = 9;
    pub const MASK_ERR_F_BNDRY_MATCH_VC9_MASK: u32 = 0x00000200;
    pub const MASK_ERR_F_BNDRY_MATCH_VC9: u32 = 1 << 9;

    /// **MASK_ERR_F_BNDRY_MATCH_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MASK_ERR_F_BNDRY_MATCH_VC8_OFFSET: u32 = 8;
    pub const MASK_ERR_F_BNDRY_MATCH_VC8_MASK: u32 = 0x00000100;
    pub const MASK_ERR_F_BNDRY_MATCH_VC8: u32 = 1 << 8;

    /// **MASK_ERR_F_BNDRY_MATCH_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MASK_ERR_F_BNDRY_MATCH_VC7_OFFSET: u32 = 7;
    pub const MASK_ERR_F_BNDRY_MATCH_VC7_MASK: u32 = 0x00000080;
    pub const MASK_ERR_F_BNDRY_MATCH_VC7: u32 = 1 << 7;

    /// **MASK_ERR_F_BNDRY_MATCH_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MASK_ERR_F_BNDRY_MATCH_VC6_OFFSET: u32 = 6;
    pub const MASK_ERR_F_BNDRY_MATCH_VC6_MASK: u32 = 0x00000040;
    pub const MASK_ERR_F_BNDRY_MATCH_VC6: u32 = 1 << 6;

    /// **MASK_ERR_F_BNDRY_MATCH_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const MASK_ERR_F_BNDRY_MATCH_VC5_OFFSET: u32 = 5;
    pub const MASK_ERR_F_BNDRY_MATCH_VC5_MASK: u32 = 0x00000020;
    pub const MASK_ERR_F_BNDRY_MATCH_VC5: u32 = 1 << 5;

    /// **MASK_ERR_F_BNDRY_MATCH_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MASK_ERR_F_BNDRY_MATCH_VC4_OFFSET: u32 = 4;
    pub const MASK_ERR_F_BNDRY_MATCH_VC4_MASK: u32 = 0x00000010;
    pub const MASK_ERR_F_BNDRY_MATCH_VC4: u32 = 1 << 4;

    /// **MASK_ERR_F_BNDRY_MATCH_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MASK_ERR_F_BNDRY_MATCH_VC3_OFFSET: u32 = 3;
    pub const MASK_ERR_F_BNDRY_MATCH_VC3_MASK: u32 = 0x00000008;
    pub const MASK_ERR_F_BNDRY_MATCH_VC3: u32 = 1 << 3;

    /// **MASK_ERR_F_BNDRY_MATCH_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MASK_ERR_F_BNDRY_MATCH_VC2_OFFSET: u32 = 2;
    pub const MASK_ERR_F_BNDRY_MATCH_VC2_MASK: u32 = 0x00000004;
    pub const MASK_ERR_F_BNDRY_MATCH_VC2: u32 = 1 << 2;

    /// **MASK_ERR_F_BNDRY_MATCH_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MASK_ERR_F_BNDRY_MATCH_VC1_OFFSET: u32 = 1;
    pub const MASK_ERR_F_BNDRY_MATCH_VC1_MASK: u32 = 0x00000002;
    pub const MASK_ERR_F_BNDRY_MATCH_VC1: u32 = 1 << 1;

    /// **MASK_ERR_F_BNDRY_MATCH_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MASK_ERR_F_BNDRY_MATCH_VC0_OFFSET: u32 = 0;
    pub const MASK_ERR_F_BNDRY_MATCH_VC0_MASK: u32 = 0x00000001;
    pub const MASK_ERR_F_BNDRY_MATCH_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MASK_ERR_F_BNDRY_MATCH_VC15** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc15(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC15_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc15(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC15_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc15(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC15_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC14** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc14(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC14_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc14(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC14_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc14(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC14_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC13** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc13(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC13_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc13(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC13_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc13(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC13_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC12** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc12(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC12_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc12(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC12_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc12(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC12_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC11** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc11(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC11_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc11(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC11_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc11(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC11_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC10** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc10(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC10_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc10(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC10_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc10(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC10_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC9** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc9(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC9_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc9(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC9_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc9(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC9_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC8** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc8(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC8_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc8(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC8_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc8(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC8_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC7** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc7(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC7_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc7(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC7_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc7(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC7_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC6** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc6(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC6_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc6(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC6_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc6(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC6_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC5** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc5(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC5_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc5(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC5_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc5(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC5_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC4** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc4(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC4_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc4(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC4_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc4(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC4_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC3** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc3(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC3_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc3(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC3_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC3_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC2** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc2(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC2_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc2(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC2_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC2_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc1(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC1_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc1(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC1_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC1_MASK);
    }

    /// **MASK_ERR_F_BNDRY_MATCH_VC0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_bndry_match_vc0(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_BNDRY_MATCH_VC0_MASK) != 0
    }

    /// Set MASK_ERR_F_BNDRY_MATCH_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_bndry_match_vc0(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_BNDRY_MATCH_VC0_MASK);
    }

    /// Clear MASK_ERR_F_BNDRY_MATCH_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_bndry_match_vc0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_BNDRY_MATCH_VC0_MASK);
    }
}

/// INT_MSK_BNDRY_FRAME_FATAL value type for batch read/modify/write operations
///
/// Use `intmskbndryframefatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intmskbndryframefatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntMskBndryFrameFatalVal {
    bits: u32,
}

impl IntMskBndryFrameFatalVal {
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

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc15(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC15_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc14(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC14_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc13(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC13_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc12(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC12_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc11(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC11_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc10(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC10_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc9(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC9_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc8(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC8_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc7(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC7_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc6(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC6_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc5(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC5_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc4(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC4_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc3(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC3_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc2(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC2_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc1(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC1_MASK) != 0
    }

    /// Read **MASK_ERR_F_BNDRY_MATCH_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_bndry_match_vc0(&self) -> bool {
        (self.bits & IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC0_MASK) != 0
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC15_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC15_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC14_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC14_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC13_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC13_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC12_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC12_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC11_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC11_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC10_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC10_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC9_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC9_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC8_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC8_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC7_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC7_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC6_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC6_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC5_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC5_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC4_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC4_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC3_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC3_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC2_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC2_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC1_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC1_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_BNDRY_MATCH_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_bndry_match_vc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC0_MASK
            } else {
                self.bits & !IntMskBndryFrameFatal::MASK_ERR_F_BNDRY_MATCH_VC0_MASK
            },
        }
    }
}

impl Default for IntMskBndryFrameFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntMskBndryFrameFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntMskBndryFrameFatalVal {
        IntMskBndryFrameFatalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntMskBndryFrameFatalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntMskBndryFrameFatalVal) -> IntMskBndryFrameFatalVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_FORCE_BNDRY_FRAME_FATAL Register
///
/// NA
///
/// Address offset: 0x0288
///
/// ## Fields
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC15** `[15]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC14** `[14]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC13** `[13]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC12** `[12]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC11** `[11]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC10** `[10]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC9** `[9]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC8** `[8]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC7** `[7]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC6** `[6]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC5** `[5]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC4** `[4]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC3** `[3]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC2** `[2]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC1** `[1]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_BNDRY_MATCH_VC0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntForceBndryFrameFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntForceBndryFrameFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntForceBndryFrameFatal {
    // ======== Field Constants ========

    /// **FORCE_ERR_F_BNDRY_MATCH_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC15_OFFSET: u32 = 15;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC15_MASK: u32 = 0x00008000;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC15: u32 = 1 << 15;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC14_OFFSET: u32 = 14;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC14_MASK: u32 = 0x00004000;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC14: u32 = 1 << 14;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC13_OFFSET: u32 = 13;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC13_MASK: u32 = 0x00002000;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC13: u32 = 1 << 13;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC12_OFFSET: u32 = 12;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC12_MASK: u32 = 0x00001000;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC12: u32 = 1 << 12;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC11_OFFSET: u32 = 11;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC11_MASK: u32 = 0x00000800;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC11: u32 = 1 << 11;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC10_OFFSET: u32 = 10;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC10_MASK: u32 = 0x00000400;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC10: u32 = 1 << 10;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC9_OFFSET: u32 = 9;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC9_MASK: u32 = 0x00000200;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC9: u32 = 1 << 9;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC8_OFFSET: u32 = 8;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC8_MASK: u32 = 0x00000100;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC8: u32 = 1 << 8;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC7_OFFSET: u32 = 7;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC7_MASK: u32 = 0x00000080;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC7: u32 = 1 << 7;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC6_OFFSET: u32 = 6;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC6_MASK: u32 = 0x00000040;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC6: u32 = 1 << 6;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC5_OFFSET: u32 = 5;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC5_MASK: u32 = 0x00000020;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC5: u32 = 1 << 5;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC4_OFFSET: u32 = 4;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC4_MASK: u32 = 0x00000010;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC4: u32 = 1 << 4;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC3_OFFSET: u32 = 3;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC3_MASK: u32 = 0x00000008;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC3: u32 = 1 << 3;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC2_OFFSET: u32 = 2;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC2_MASK: u32 = 0x00000004;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC2: u32 = 1 << 2;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC1_OFFSET: u32 = 1;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC1_MASK: u32 = 0x00000002;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC1: u32 = 1 << 1;

    /// **FORCE_ERR_F_BNDRY_MATCH_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FORCE_ERR_F_BNDRY_MATCH_VC0_OFFSET: u32 = 0;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC0_MASK: u32 = 0x00000001;
    pub const FORCE_ERR_F_BNDRY_MATCH_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FORCE_ERR_F_BNDRY_MATCH_VC15** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc15(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC15_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc15(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC15_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc15(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC15_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC14** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc14(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC14_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc14(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC14_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc14(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC14_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC13** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc13(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC13_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc13(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC13_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc13(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC13_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC12** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc12(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC12_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc12(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC12_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc12(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC12_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC11** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc11(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC11_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc11(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC11_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc11(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC11_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC10** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc10(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC10_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc10(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC10_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc10(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC10_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC9** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc9(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC9_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc9(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC9_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc9(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC9_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC8** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc8(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC8_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc8(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC8_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc8(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC8_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC7** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc7(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC7_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc7(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC7_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc7(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC7_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC6** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc6(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC6_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc6(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC6_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc6(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC6_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC5** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc5(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC5_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc5(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC5_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc5(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC5_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC4** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc4(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC4_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc4(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC4_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc4(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC4_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC3** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc3(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC3_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc3(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC3_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc3(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC3_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC2** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc2(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC2_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc2(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC2_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc2(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC2_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc1(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC1_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc1(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC1_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc1(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC1_MASK);
    }

    /// **FORCE_ERR_F_BNDRY_MATCH_VC0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_bndry_match_vc0(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_BNDRY_MATCH_VC0_MASK) != 0
    }

    /// Set FORCE_ERR_F_BNDRY_MATCH_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_bndry_match_vc0(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_BNDRY_MATCH_VC0_MASK);
    }

    /// Clear FORCE_ERR_F_BNDRY_MATCH_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_bndry_match_vc0(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_BNDRY_MATCH_VC0_MASK);
    }
}

/// INT_FORCE_BNDRY_FRAME_FATAL value type for batch read/modify/write operations
///
/// Use `intforcebndryframefatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intforcebndryframefatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntForceBndryFrameFatalVal {
    bits: u32,
}

impl IntForceBndryFrameFatalVal {
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

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc15(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC15_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc14(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC14_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc13(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC13_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc12(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC12_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc11(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC11_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc10(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC10_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc9(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC9_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc8(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC8_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc7(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC7_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc6(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC6_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc5(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC5_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc4(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC4_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc3(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC3_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc2(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC2_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc1(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC1_MASK) != 0
    }

    /// Read **FORCE_ERR_F_BNDRY_MATCH_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_bndry_match_vc0(&self) -> bool {
        (self.bits & IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC0_MASK) != 0
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC15_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC15_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC14_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC14_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC13_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC13_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC12_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC12_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC11_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC11_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC10_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC10_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC9_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC9_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC8_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC8_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC7_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC7_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC6_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC6_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC5_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC5_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC4_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC4_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC3_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC3_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC2_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC2_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC1_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC1_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_BNDRY_MATCH_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_bndry_match_vc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC0_MASK
            } else {
                self.bits & !IntForceBndryFrameFatal::FORCE_ERR_F_BNDRY_MATCH_VC0_MASK
            },
        }
    }
}

impl Default for IntForceBndryFrameFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntForceBndryFrameFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntForceBndryFrameFatalVal {
        IntForceBndryFrameFatalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntForceBndryFrameFatalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntForceBndryFrameFatalVal) -> IntForceBndryFrameFatalVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ST_SEQ_FRAME_FATAL Register
///
/// NA
///
/// Address offset: 0x0290
///
/// ## Fields
///
/// - **ST_ERR_F_SEQ_VC15** `[15]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC14** `[14]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC13** `[13]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC12** `[12]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC11** `[11]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC10** `[10]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC9** `[9]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC8** `[8]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC7** `[7]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC6** `[6]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC5** `[5]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC4** `[4]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC3** `[3]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC2** `[2]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC1** `[1]` (RO)
///
///   NA
///
/// - **ST_ERR_F_SEQ_VC0** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntStSeqFrameFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntStSeqFrameFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntStSeqFrameFatal {
    // ======== Field Constants ========

    /// **ST_ERR_F_SEQ_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const ST_ERR_F_SEQ_VC15_OFFSET: u32 = 15;
    pub const ST_ERR_F_SEQ_VC15_MASK: u32 = 0x00008000;
    pub const ST_ERR_F_SEQ_VC15: u32 = 1 << 15;

    /// **ST_ERR_F_SEQ_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const ST_ERR_F_SEQ_VC14_OFFSET: u32 = 14;
    pub const ST_ERR_F_SEQ_VC14_MASK: u32 = 0x00004000;
    pub const ST_ERR_F_SEQ_VC14: u32 = 1 << 14;

    /// **ST_ERR_F_SEQ_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const ST_ERR_F_SEQ_VC13_OFFSET: u32 = 13;
    pub const ST_ERR_F_SEQ_VC13_MASK: u32 = 0x00002000;
    pub const ST_ERR_F_SEQ_VC13: u32 = 1 << 13;

    /// **ST_ERR_F_SEQ_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const ST_ERR_F_SEQ_VC12_OFFSET: u32 = 12;
    pub const ST_ERR_F_SEQ_VC12_MASK: u32 = 0x00001000;
    pub const ST_ERR_F_SEQ_VC12: u32 = 1 << 12;

    /// **ST_ERR_F_SEQ_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ST_ERR_F_SEQ_VC11_OFFSET: u32 = 11;
    pub const ST_ERR_F_SEQ_VC11_MASK: u32 = 0x00000800;
    pub const ST_ERR_F_SEQ_VC11: u32 = 1 << 11;

    /// **ST_ERR_F_SEQ_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ST_ERR_F_SEQ_VC10_OFFSET: u32 = 10;
    pub const ST_ERR_F_SEQ_VC10_MASK: u32 = 0x00000400;
    pub const ST_ERR_F_SEQ_VC10: u32 = 1 << 10;

    /// **ST_ERR_F_SEQ_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ST_ERR_F_SEQ_VC9_OFFSET: u32 = 9;
    pub const ST_ERR_F_SEQ_VC9_MASK: u32 = 0x00000200;
    pub const ST_ERR_F_SEQ_VC9: u32 = 1 << 9;

    /// **ST_ERR_F_SEQ_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ST_ERR_F_SEQ_VC8_OFFSET: u32 = 8;
    pub const ST_ERR_F_SEQ_VC8_MASK: u32 = 0x00000100;
    pub const ST_ERR_F_SEQ_VC8: u32 = 1 << 8;

    /// **ST_ERR_F_SEQ_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ST_ERR_F_SEQ_VC7_OFFSET: u32 = 7;
    pub const ST_ERR_F_SEQ_VC7_MASK: u32 = 0x00000080;
    pub const ST_ERR_F_SEQ_VC7: u32 = 1 << 7;

    /// **ST_ERR_F_SEQ_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ST_ERR_F_SEQ_VC6_OFFSET: u32 = 6;
    pub const ST_ERR_F_SEQ_VC6_MASK: u32 = 0x00000040;
    pub const ST_ERR_F_SEQ_VC6: u32 = 1 << 6;

    /// **ST_ERR_F_SEQ_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ST_ERR_F_SEQ_VC5_OFFSET: u32 = 5;
    pub const ST_ERR_F_SEQ_VC5_MASK: u32 = 0x00000020;
    pub const ST_ERR_F_SEQ_VC5: u32 = 1 << 5;

    /// **ST_ERR_F_SEQ_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ST_ERR_F_SEQ_VC4_OFFSET: u32 = 4;
    pub const ST_ERR_F_SEQ_VC4_MASK: u32 = 0x00000010;
    pub const ST_ERR_F_SEQ_VC4: u32 = 1 << 4;

    /// **ST_ERR_F_SEQ_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ST_ERR_F_SEQ_VC3_OFFSET: u32 = 3;
    pub const ST_ERR_F_SEQ_VC3_MASK: u32 = 0x00000008;
    pub const ST_ERR_F_SEQ_VC3: u32 = 1 << 3;

    /// **ST_ERR_F_SEQ_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ST_ERR_F_SEQ_VC2_OFFSET: u32 = 2;
    pub const ST_ERR_F_SEQ_VC2_MASK: u32 = 0x00000004;
    pub const ST_ERR_F_SEQ_VC2: u32 = 1 << 2;

    /// **ST_ERR_F_SEQ_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ST_ERR_F_SEQ_VC1_OFFSET: u32 = 1;
    pub const ST_ERR_F_SEQ_VC1_MASK: u32 = 0x00000002;
    pub const ST_ERR_F_SEQ_VC1: u32 = 1 << 1;

    /// **ST_ERR_F_SEQ_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ST_ERR_F_SEQ_VC0_OFFSET: u32 = 0;
    pub const ST_ERR_F_SEQ_VC0_MASK: u32 = 0x00000001;
    pub const ST_ERR_F_SEQ_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ST_ERR_F_SEQ_VC15** `[15]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc15(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC15_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC14** `[14]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc14(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC14_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC13** `[13]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc13(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC13_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC12** `[12]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc12(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC12_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC11** `[11]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc11(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC11_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC10** `[10]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc10(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC10_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC9** `[9]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc9(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC9_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC8** `[8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc8(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC8_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC7** `[7]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc7(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC7_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC6** `[6]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc6(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC6_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC5** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc5(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC5_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC4** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc4(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC4_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC3** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc3(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC3_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC2** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc2(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC2_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC1** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc1(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC1_MASK) != 0
    }

    /// **ST_ERR_F_SEQ_VC0** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_f_seq_vc0(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_F_SEQ_VC0_MASK) != 0
    }
}

/// INT_ST_SEQ_FRAME_FATAL value type for batch read/modify/write operations
///
/// Use `intstseqframefatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intstseqframefatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStSeqFrameFatalVal {
    bits: u32,
}

impl IntStSeqFrameFatalVal {
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

    /// Read **ST_ERR_F_SEQ_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc15(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC15_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc14(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC14_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc13(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC13_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc12(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC12_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc11(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC11_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc10(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC10_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc9(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC9_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc8(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC8_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc7(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC7_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc6(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC6_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc5(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC5_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc4(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC4_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc3(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC3_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc2(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC2_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc1(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC1_MASK) != 0
    }

    /// Read **ST_ERR_F_SEQ_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_f_seq_vc0(&self) -> bool {
        (self.bits & IntStSeqFrameFatal::ST_ERR_F_SEQ_VC0_MASK) != 0
    }
}

impl Default for IntStSeqFrameFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntStSeqFrameFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStSeqFrameFatalVal {
        IntStSeqFrameFatalVal::from_bits(self.value.read_bits())
    }
}

/// INT_MSK_SEQ_FRAME_FATAL Register
///
/// NA
///
/// Address offset: 0x0294
///
/// ## Fields
///
/// - **MASK_ERR_F_SEQ_VC15** `[15]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC14** `[14]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC13** `[13]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC12** `[12]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC11** `[11]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC10** `[10]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC9** `[9]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC8** `[8]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC7** `[7]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC6** `[6]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC5** `[5]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC4** `[4]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC3** `[3]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC2** `[2]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC1** `[1]` (RW)
///
///   NA
///
/// - **MASK_ERR_F_SEQ_VC0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntMskSeqFrameFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntMskSeqFrameFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntMskSeqFrameFatal {
    // ======== Field Constants ========

    /// **MASK_ERR_F_SEQ_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const MASK_ERR_F_SEQ_VC15_OFFSET: u32 = 15;
    pub const MASK_ERR_F_SEQ_VC15_MASK: u32 = 0x00008000;
    pub const MASK_ERR_F_SEQ_VC15: u32 = 1 << 15;

    /// **MASK_ERR_F_SEQ_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MASK_ERR_F_SEQ_VC14_OFFSET: u32 = 14;
    pub const MASK_ERR_F_SEQ_VC14_MASK: u32 = 0x00004000;
    pub const MASK_ERR_F_SEQ_VC14: u32 = 1 << 14;

    /// **MASK_ERR_F_SEQ_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const MASK_ERR_F_SEQ_VC13_OFFSET: u32 = 13;
    pub const MASK_ERR_F_SEQ_VC13_MASK: u32 = 0x00002000;
    pub const MASK_ERR_F_SEQ_VC13: u32 = 1 << 13;

    /// **MASK_ERR_F_SEQ_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const MASK_ERR_F_SEQ_VC12_OFFSET: u32 = 12;
    pub const MASK_ERR_F_SEQ_VC12_MASK: u32 = 0x00001000;
    pub const MASK_ERR_F_SEQ_VC12: u32 = 1 << 12;

    /// **MASK_ERR_F_SEQ_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const MASK_ERR_F_SEQ_VC11_OFFSET: u32 = 11;
    pub const MASK_ERR_F_SEQ_VC11_MASK: u32 = 0x00000800;
    pub const MASK_ERR_F_SEQ_VC11: u32 = 1 << 11;

    /// **MASK_ERR_F_SEQ_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const MASK_ERR_F_SEQ_VC10_OFFSET: u32 = 10;
    pub const MASK_ERR_F_SEQ_VC10_MASK: u32 = 0x00000400;
    pub const MASK_ERR_F_SEQ_VC10: u32 = 1 << 10;

    /// **MASK_ERR_F_SEQ_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const MASK_ERR_F_SEQ_VC9_OFFSET: u32 = 9;
    pub const MASK_ERR_F_SEQ_VC9_MASK: u32 = 0x00000200;
    pub const MASK_ERR_F_SEQ_VC9: u32 = 1 << 9;

    /// **MASK_ERR_F_SEQ_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MASK_ERR_F_SEQ_VC8_OFFSET: u32 = 8;
    pub const MASK_ERR_F_SEQ_VC8_MASK: u32 = 0x00000100;
    pub const MASK_ERR_F_SEQ_VC8: u32 = 1 << 8;

    /// **MASK_ERR_F_SEQ_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MASK_ERR_F_SEQ_VC7_OFFSET: u32 = 7;
    pub const MASK_ERR_F_SEQ_VC7_MASK: u32 = 0x00000080;
    pub const MASK_ERR_F_SEQ_VC7: u32 = 1 << 7;

    /// **MASK_ERR_F_SEQ_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MASK_ERR_F_SEQ_VC6_OFFSET: u32 = 6;
    pub const MASK_ERR_F_SEQ_VC6_MASK: u32 = 0x00000040;
    pub const MASK_ERR_F_SEQ_VC6: u32 = 1 << 6;

    /// **MASK_ERR_F_SEQ_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const MASK_ERR_F_SEQ_VC5_OFFSET: u32 = 5;
    pub const MASK_ERR_F_SEQ_VC5_MASK: u32 = 0x00000020;
    pub const MASK_ERR_F_SEQ_VC5: u32 = 1 << 5;

    /// **MASK_ERR_F_SEQ_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MASK_ERR_F_SEQ_VC4_OFFSET: u32 = 4;
    pub const MASK_ERR_F_SEQ_VC4_MASK: u32 = 0x00000010;
    pub const MASK_ERR_F_SEQ_VC4: u32 = 1 << 4;

    /// **MASK_ERR_F_SEQ_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MASK_ERR_F_SEQ_VC3_OFFSET: u32 = 3;
    pub const MASK_ERR_F_SEQ_VC3_MASK: u32 = 0x00000008;
    pub const MASK_ERR_F_SEQ_VC3: u32 = 1 << 3;

    /// **MASK_ERR_F_SEQ_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MASK_ERR_F_SEQ_VC2_OFFSET: u32 = 2;
    pub const MASK_ERR_F_SEQ_VC2_MASK: u32 = 0x00000004;
    pub const MASK_ERR_F_SEQ_VC2: u32 = 1 << 2;

    /// **MASK_ERR_F_SEQ_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MASK_ERR_F_SEQ_VC1_OFFSET: u32 = 1;
    pub const MASK_ERR_F_SEQ_VC1_MASK: u32 = 0x00000002;
    pub const MASK_ERR_F_SEQ_VC1: u32 = 1 << 1;

    /// **MASK_ERR_F_SEQ_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MASK_ERR_F_SEQ_VC0_OFFSET: u32 = 0;
    pub const MASK_ERR_F_SEQ_VC0_MASK: u32 = 0x00000001;
    pub const MASK_ERR_F_SEQ_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MASK_ERR_F_SEQ_VC15** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc15(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC15_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc15(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC15_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc15(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC15_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC14** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc14(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC14_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc14(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC14_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc14(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC14_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC13** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc13(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC13_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc13(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC13_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc13(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC13_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC12** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc12(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC12_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc12(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC12_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc12(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC12_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC11** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc11(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC11_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc11(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC11_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc11(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC11_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC10** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc10(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC10_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc10(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC10_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc10(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC10_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC9** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc9(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC9_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc9(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC9_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc9(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC9_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC8** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc8(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC8_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc8(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC8_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc8(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC8_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC7** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc7(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC7_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc7(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC7_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc7(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC7_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC6** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc6(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC6_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc6(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC6_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc6(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC6_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC5** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc5(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC5_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc5(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC5_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc5(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC5_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC4** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc4(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC4_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc4(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC4_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc4(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC4_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC3** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc3(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC3_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc3(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC3_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC3_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC2** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc2(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC2_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc2(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC2_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC2_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc1(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC1_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc1(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC1_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC1_MASK);
    }

    /// **MASK_ERR_F_SEQ_VC0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_f_seq_vc0(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_F_SEQ_VC0_MASK) != 0
    }

    /// Set MASK_ERR_F_SEQ_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_f_seq_vc0(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_F_SEQ_VC0_MASK);
    }

    /// Clear MASK_ERR_F_SEQ_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_f_seq_vc0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_F_SEQ_VC0_MASK);
    }
}

/// INT_MSK_SEQ_FRAME_FATAL value type for batch read/modify/write operations
///
/// Use `intmskseqframefatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intmskseqframefatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntMskSeqFrameFatalVal {
    bits: u32,
}

impl IntMskSeqFrameFatalVal {
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

    /// Read **MASK_ERR_F_SEQ_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc15(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC15_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc14(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC14_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc13(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC13_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc12(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC12_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc11(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC11_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc10(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC10_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc9(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC9_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc8(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC8_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc7(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC7_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc6(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC6_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc5(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC5_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc4(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC4_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc3(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC3_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc2(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC2_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc1(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC1_MASK) != 0
    }

    /// Read **MASK_ERR_F_SEQ_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_f_seq_vc0(&self) -> bool {
        (self.bits & IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC0_MASK) != 0
    }

    /// Set **MASK_ERR_F_SEQ_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC15_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC15_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC14_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC14_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC13_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC13_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC12_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC12_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC11_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC11_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC10_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC10_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC9_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC9_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC8_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC8_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC7_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC7_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC6_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC6_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC5_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC5_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC4_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC4_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC3_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC3_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC2_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC2_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC1_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC1_MASK
            },
        }
    }

    /// Set **MASK_ERR_F_SEQ_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_f_seq_vc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC0_MASK
            } else {
                self.bits & !IntMskSeqFrameFatal::MASK_ERR_F_SEQ_VC0_MASK
            },
        }
    }
}

impl Default for IntMskSeqFrameFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntMskSeqFrameFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntMskSeqFrameFatalVal {
        IntMskSeqFrameFatalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntMskSeqFrameFatalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntMskSeqFrameFatalVal) -> IntMskSeqFrameFatalVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_FORCE_SEQ_FRAME_FATAL Register
///
/// NA
///
/// Address offset: 0x0298
///
/// ## Fields
///
/// - **FORCE_ERR_F_SEQ_VC15** `[15]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC14** `[14]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC13** `[13]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC12** `[12]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC11** `[11]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC10** `[10]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC9** `[9]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC8** `[8]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC7** `[7]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC6** `[6]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC5** `[5]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC4** `[4]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC3** `[3]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC2** `[2]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC1** `[1]` (RW)
///
///   NA
///
/// - **FORCE_ERR_F_SEQ_VC0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntForceSeqFrameFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntForceSeqFrameFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntForceSeqFrameFatal {
    // ======== Field Constants ========

    /// **FORCE_ERR_F_SEQ_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const FORCE_ERR_F_SEQ_VC15_OFFSET: u32 = 15;
    pub const FORCE_ERR_F_SEQ_VC15_MASK: u32 = 0x00008000;
    pub const FORCE_ERR_F_SEQ_VC15: u32 = 1 << 15;

    /// **FORCE_ERR_F_SEQ_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const FORCE_ERR_F_SEQ_VC14_OFFSET: u32 = 14;
    pub const FORCE_ERR_F_SEQ_VC14_MASK: u32 = 0x00004000;
    pub const FORCE_ERR_F_SEQ_VC14: u32 = 1 << 14;

    /// **FORCE_ERR_F_SEQ_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const FORCE_ERR_F_SEQ_VC13_OFFSET: u32 = 13;
    pub const FORCE_ERR_F_SEQ_VC13_MASK: u32 = 0x00002000;
    pub const FORCE_ERR_F_SEQ_VC13: u32 = 1 << 13;

    /// **FORCE_ERR_F_SEQ_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const FORCE_ERR_F_SEQ_VC12_OFFSET: u32 = 12;
    pub const FORCE_ERR_F_SEQ_VC12_MASK: u32 = 0x00001000;
    pub const FORCE_ERR_F_SEQ_VC12: u32 = 1 << 12;

    /// **FORCE_ERR_F_SEQ_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const FORCE_ERR_F_SEQ_VC11_OFFSET: u32 = 11;
    pub const FORCE_ERR_F_SEQ_VC11_MASK: u32 = 0x00000800;
    pub const FORCE_ERR_F_SEQ_VC11: u32 = 1 << 11;

    /// **FORCE_ERR_F_SEQ_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const FORCE_ERR_F_SEQ_VC10_OFFSET: u32 = 10;
    pub const FORCE_ERR_F_SEQ_VC10_MASK: u32 = 0x00000400;
    pub const FORCE_ERR_F_SEQ_VC10: u32 = 1 << 10;

    /// **FORCE_ERR_F_SEQ_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const FORCE_ERR_F_SEQ_VC9_OFFSET: u32 = 9;
    pub const FORCE_ERR_F_SEQ_VC9_MASK: u32 = 0x00000200;
    pub const FORCE_ERR_F_SEQ_VC9: u32 = 1 << 9;

    /// **FORCE_ERR_F_SEQ_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const FORCE_ERR_F_SEQ_VC8_OFFSET: u32 = 8;
    pub const FORCE_ERR_F_SEQ_VC8_MASK: u32 = 0x00000100;
    pub const FORCE_ERR_F_SEQ_VC8: u32 = 1 << 8;

    /// **FORCE_ERR_F_SEQ_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const FORCE_ERR_F_SEQ_VC7_OFFSET: u32 = 7;
    pub const FORCE_ERR_F_SEQ_VC7_MASK: u32 = 0x00000080;
    pub const FORCE_ERR_F_SEQ_VC7: u32 = 1 << 7;

    /// **FORCE_ERR_F_SEQ_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const FORCE_ERR_F_SEQ_VC6_OFFSET: u32 = 6;
    pub const FORCE_ERR_F_SEQ_VC6_MASK: u32 = 0x00000040;
    pub const FORCE_ERR_F_SEQ_VC6: u32 = 1 << 6;

    /// **FORCE_ERR_F_SEQ_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const FORCE_ERR_F_SEQ_VC5_OFFSET: u32 = 5;
    pub const FORCE_ERR_F_SEQ_VC5_MASK: u32 = 0x00000020;
    pub const FORCE_ERR_F_SEQ_VC5: u32 = 1 << 5;

    /// **FORCE_ERR_F_SEQ_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const FORCE_ERR_F_SEQ_VC4_OFFSET: u32 = 4;
    pub const FORCE_ERR_F_SEQ_VC4_MASK: u32 = 0x00000010;
    pub const FORCE_ERR_F_SEQ_VC4: u32 = 1 << 4;

    /// **FORCE_ERR_F_SEQ_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const FORCE_ERR_F_SEQ_VC3_OFFSET: u32 = 3;
    pub const FORCE_ERR_F_SEQ_VC3_MASK: u32 = 0x00000008;
    pub const FORCE_ERR_F_SEQ_VC3: u32 = 1 << 3;

    /// **FORCE_ERR_F_SEQ_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FORCE_ERR_F_SEQ_VC2_OFFSET: u32 = 2;
    pub const FORCE_ERR_F_SEQ_VC2_MASK: u32 = 0x00000004;
    pub const FORCE_ERR_F_SEQ_VC2: u32 = 1 << 2;

    /// **FORCE_ERR_F_SEQ_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FORCE_ERR_F_SEQ_VC1_OFFSET: u32 = 1;
    pub const FORCE_ERR_F_SEQ_VC1_MASK: u32 = 0x00000002;
    pub const FORCE_ERR_F_SEQ_VC1: u32 = 1 << 1;

    /// **FORCE_ERR_F_SEQ_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FORCE_ERR_F_SEQ_VC0_OFFSET: u32 = 0;
    pub const FORCE_ERR_F_SEQ_VC0_MASK: u32 = 0x00000001;
    pub const FORCE_ERR_F_SEQ_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FORCE_ERR_F_SEQ_VC15** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc15(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC15_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc15(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC15_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc15(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC15_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC14** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc14(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC14_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc14(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC14_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc14(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC14_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC13** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc13(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC13_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc13(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC13_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc13(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC13_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC12** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc12(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC12_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc12(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC12_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc12(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC12_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC11** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc11(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC11_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc11(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC11_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc11(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC11_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC10** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc10(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC10_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc10(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC10_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc10(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC10_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC9** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc9(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC9_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc9(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC9_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc9(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC9_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC8** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc8(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC8_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc8(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC8_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc8(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC8_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC7** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc7(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC7_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc7(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC7_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc7(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC7_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC6** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc6(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC6_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc6(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC6_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc6(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC6_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC5** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc5(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC5_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc5(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC5_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc5(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC5_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC4** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc4(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC4_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc4(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC4_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc4(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC4_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC3** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc3(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC3_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc3(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC3_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc3(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC3_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC2** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc2(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC2_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc2(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC2_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc2(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC2_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc1(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC1_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc1(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC1_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc1(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC1_MASK);
    }

    /// **FORCE_ERR_F_SEQ_VC0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_f_seq_vc0(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_F_SEQ_VC0_MASK) != 0
    }

    /// Set FORCE_ERR_F_SEQ_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_f_seq_vc0(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_F_SEQ_VC0_MASK);
    }

    /// Clear FORCE_ERR_F_SEQ_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_f_seq_vc0(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_F_SEQ_VC0_MASK);
    }
}

/// INT_FORCE_SEQ_FRAME_FATAL value type for batch read/modify/write operations
///
/// Use `intforceseqframefatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intforceseqframefatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntForceSeqFrameFatalVal {
    bits: u32,
}

impl IntForceSeqFrameFatalVal {
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

    /// Read **FORCE_ERR_F_SEQ_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc15(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC15_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc14(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC14_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc13(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC13_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc12(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC12_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc11(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC11_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc10(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC10_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc9(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC9_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc8(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC8_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc7(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC7_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc6(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC6_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc5(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC5_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc4(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC4_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc3(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC3_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc2(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC2_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc1(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC1_MASK) != 0
    }

    /// Read **FORCE_ERR_F_SEQ_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_f_seq_vc0(&self) -> bool {
        (self.bits & IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC0_MASK) != 0
    }

    /// Set **FORCE_ERR_F_SEQ_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC15_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC15_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC14_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC14_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC13_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC13_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC12_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC12_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC11_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC11_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC10_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC10_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC9_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC9_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC8_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC8_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC7_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC7_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC6_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC6_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC5_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC5_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC4_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC4_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC3_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC3_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC2_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC2_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC1_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC1_MASK
            },
        }
    }

    /// Set **FORCE_ERR_F_SEQ_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_f_seq_vc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC0_MASK
            } else {
                self.bits & !IntForceSeqFrameFatal::FORCE_ERR_F_SEQ_VC0_MASK
            },
        }
    }
}

impl Default for IntForceSeqFrameFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntForceSeqFrameFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntForceSeqFrameFatalVal {
        IntForceSeqFrameFatalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntForceSeqFrameFatalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntForceSeqFrameFatalVal) -> IntForceSeqFrameFatalVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ST_CRC_FRAME_FATAL Register
///
/// NA
///
/// Address offset: 0x02a0
///
/// ## Fields
///
/// - **ST_ERR_FRAME_DATA_VC15** `[15]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC14** `[14]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC13** `[13]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC12** `[12]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC11** `[11]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC10** `[10]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC9** `[9]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC8** `[8]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC7** `[7]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC6** `[6]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC5** `[5]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC4** `[4]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC3** `[3]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC2** `[2]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC1** `[1]` (RO)
///
///   NA
///
/// - **ST_ERR_FRAME_DATA_VC0** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntStCrcFrameFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntStCrcFrameFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntStCrcFrameFatal {
    // ======== Field Constants ========

    /// **ST_ERR_FRAME_DATA_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const ST_ERR_FRAME_DATA_VC15_OFFSET: u32 = 15;
    pub const ST_ERR_FRAME_DATA_VC15_MASK: u32 = 0x00008000;
    pub const ST_ERR_FRAME_DATA_VC15: u32 = 1 << 15;

    /// **ST_ERR_FRAME_DATA_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const ST_ERR_FRAME_DATA_VC14_OFFSET: u32 = 14;
    pub const ST_ERR_FRAME_DATA_VC14_MASK: u32 = 0x00004000;
    pub const ST_ERR_FRAME_DATA_VC14: u32 = 1 << 14;

    /// **ST_ERR_FRAME_DATA_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const ST_ERR_FRAME_DATA_VC13_OFFSET: u32 = 13;
    pub const ST_ERR_FRAME_DATA_VC13_MASK: u32 = 0x00002000;
    pub const ST_ERR_FRAME_DATA_VC13: u32 = 1 << 13;

    /// **ST_ERR_FRAME_DATA_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const ST_ERR_FRAME_DATA_VC12_OFFSET: u32 = 12;
    pub const ST_ERR_FRAME_DATA_VC12_MASK: u32 = 0x00001000;
    pub const ST_ERR_FRAME_DATA_VC12: u32 = 1 << 12;

    /// **ST_ERR_FRAME_DATA_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ST_ERR_FRAME_DATA_VC11_OFFSET: u32 = 11;
    pub const ST_ERR_FRAME_DATA_VC11_MASK: u32 = 0x00000800;
    pub const ST_ERR_FRAME_DATA_VC11: u32 = 1 << 11;

    /// **ST_ERR_FRAME_DATA_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ST_ERR_FRAME_DATA_VC10_OFFSET: u32 = 10;
    pub const ST_ERR_FRAME_DATA_VC10_MASK: u32 = 0x00000400;
    pub const ST_ERR_FRAME_DATA_VC10: u32 = 1 << 10;

    /// **ST_ERR_FRAME_DATA_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ST_ERR_FRAME_DATA_VC9_OFFSET: u32 = 9;
    pub const ST_ERR_FRAME_DATA_VC9_MASK: u32 = 0x00000200;
    pub const ST_ERR_FRAME_DATA_VC9: u32 = 1 << 9;

    /// **ST_ERR_FRAME_DATA_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ST_ERR_FRAME_DATA_VC8_OFFSET: u32 = 8;
    pub const ST_ERR_FRAME_DATA_VC8_MASK: u32 = 0x00000100;
    pub const ST_ERR_FRAME_DATA_VC8: u32 = 1 << 8;

    /// **ST_ERR_FRAME_DATA_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ST_ERR_FRAME_DATA_VC7_OFFSET: u32 = 7;
    pub const ST_ERR_FRAME_DATA_VC7_MASK: u32 = 0x00000080;
    pub const ST_ERR_FRAME_DATA_VC7: u32 = 1 << 7;

    /// **ST_ERR_FRAME_DATA_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ST_ERR_FRAME_DATA_VC6_OFFSET: u32 = 6;
    pub const ST_ERR_FRAME_DATA_VC6_MASK: u32 = 0x00000040;
    pub const ST_ERR_FRAME_DATA_VC6: u32 = 1 << 6;

    /// **ST_ERR_FRAME_DATA_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ST_ERR_FRAME_DATA_VC5_OFFSET: u32 = 5;
    pub const ST_ERR_FRAME_DATA_VC5_MASK: u32 = 0x00000020;
    pub const ST_ERR_FRAME_DATA_VC5: u32 = 1 << 5;

    /// **ST_ERR_FRAME_DATA_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ST_ERR_FRAME_DATA_VC4_OFFSET: u32 = 4;
    pub const ST_ERR_FRAME_DATA_VC4_MASK: u32 = 0x00000010;
    pub const ST_ERR_FRAME_DATA_VC4: u32 = 1 << 4;

    /// **ST_ERR_FRAME_DATA_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ST_ERR_FRAME_DATA_VC3_OFFSET: u32 = 3;
    pub const ST_ERR_FRAME_DATA_VC3_MASK: u32 = 0x00000008;
    pub const ST_ERR_FRAME_DATA_VC3: u32 = 1 << 3;

    /// **ST_ERR_FRAME_DATA_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ST_ERR_FRAME_DATA_VC2_OFFSET: u32 = 2;
    pub const ST_ERR_FRAME_DATA_VC2_MASK: u32 = 0x00000004;
    pub const ST_ERR_FRAME_DATA_VC2: u32 = 1 << 2;

    /// **ST_ERR_FRAME_DATA_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ST_ERR_FRAME_DATA_VC1_OFFSET: u32 = 1;
    pub const ST_ERR_FRAME_DATA_VC1_MASK: u32 = 0x00000002;
    pub const ST_ERR_FRAME_DATA_VC1: u32 = 1 << 1;

    /// **ST_ERR_FRAME_DATA_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ST_ERR_FRAME_DATA_VC0_OFFSET: u32 = 0;
    pub const ST_ERR_FRAME_DATA_VC0_MASK: u32 = 0x00000001;
    pub const ST_ERR_FRAME_DATA_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ST_ERR_FRAME_DATA_VC15** `[15]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc15(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC15_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC14** `[14]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc14(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC14_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC13** `[13]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc13(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC13_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC12** `[12]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc12(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC12_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC11** `[11]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc11(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC11_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC10** `[10]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc10(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC10_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC9** `[9]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc9(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC9_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC8** `[8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc8(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC8_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC7** `[7]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc7(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC7_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC6** `[6]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc6(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC6_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC5** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc5(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC5_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC4** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc4(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC4_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC3** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc3(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC3_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC2** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc2(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC2_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC1** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc1(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC1_MASK) != 0
    }

    /// **ST_ERR_FRAME_DATA_VC0** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_frame_data_vc0(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_FRAME_DATA_VC0_MASK) != 0
    }
}

/// INT_ST_CRC_FRAME_FATAL value type for batch read/modify/write operations
///
/// Use `intstcrcframefatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intstcrcframefatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStCrcFrameFatalVal {
    bits: u32,
}

impl IntStCrcFrameFatalVal {
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

    /// Read **ST_ERR_FRAME_DATA_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc15(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC15_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc14(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC14_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc13(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC13_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc12(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC12_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc11(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC11_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc10(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC10_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc9(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC9_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc8(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC8_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc7(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC7_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc6(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC6_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc5(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC5_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc4(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC4_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc3(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC3_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc2(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC2_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc1(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC1_MASK) != 0
    }

    /// Read **ST_ERR_FRAME_DATA_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_frame_data_vc0(&self) -> bool {
        (self.bits & IntStCrcFrameFatal::ST_ERR_FRAME_DATA_VC0_MASK) != 0
    }
}

impl Default for IntStCrcFrameFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntStCrcFrameFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStCrcFrameFatalVal {
        IntStCrcFrameFatalVal::from_bits(self.value.read_bits())
    }
}

/// INT_MSK_CRC_FRAME_FATAL Register
///
/// NA
///
/// Address offset: 0x02a4
///
/// ## Fields
///
/// - **MASK_ERR_FRAME_DATA_VC15** `[15]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC14** `[14]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC13** `[13]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC12** `[12]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC11** `[11]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC10** `[10]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC9** `[9]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC8** `[8]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC7** `[7]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC6** `[6]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC5** `[5]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC4** `[4]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC3** `[3]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC2** `[2]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC1** `[1]` (RW)
///
///   NA
///
/// - **MASK_ERR_FRAME_DATA_VC0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntMskCrcFrameFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntMskCrcFrameFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntMskCrcFrameFatal {
    // ======== Field Constants ========

    /// **MASK_ERR_FRAME_DATA_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const MASK_ERR_FRAME_DATA_VC15_OFFSET: u32 = 15;
    pub const MASK_ERR_FRAME_DATA_VC15_MASK: u32 = 0x00008000;
    pub const MASK_ERR_FRAME_DATA_VC15: u32 = 1 << 15;

    /// **MASK_ERR_FRAME_DATA_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MASK_ERR_FRAME_DATA_VC14_OFFSET: u32 = 14;
    pub const MASK_ERR_FRAME_DATA_VC14_MASK: u32 = 0x00004000;
    pub const MASK_ERR_FRAME_DATA_VC14: u32 = 1 << 14;

    /// **MASK_ERR_FRAME_DATA_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const MASK_ERR_FRAME_DATA_VC13_OFFSET: u32 = 13;
    pub const MASK_ERR_FRAME_DATA_VC13_MASK: u32 = 0x00002000;
    pub const MASK_ERR_FRAME_DATA_VC13: u32 = 1 << 13;

    /// **MASK_ERR_FRAME_DATA_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const MASK_ERR_FRAME_DATA_VC12_OFFSET: u32 = 12;
    pub const MASK_ERR_FRAME_DATA_VC12_MASK: u32 = 0x00001000;
    pub const MASK_ERR_FRAME_DATA_VC12: u32 = 1 << 12;

    /// **MASK_ERR_FRAME_DATA_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const MASK_ERR_FRAME_DATA_VC11_OFFSET: u32 = 11;
    pub const MASK_ERR_FRAME_DATA_VC11_MASK: u32 = 0x00000800;
    pub const MASK_ERR_FRAME_DATA_VC11: u32 = 1 << 11;

    /// **MASK_ERR_FRAME_DATA_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const MASK_ERR_FRAME_DATA_VC10_OFFSET: u32 = 10;
    pub const MASK_ERR_FRAME_DATA_VC10_MASK: u32 = 0x00000400;
    pub const MASK_ERR_FRAME_DATA_VC10: u32 = 1 << 10;

    /// **MASK_ERR_FRAME_DATA_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const MASK_ERR_FRAME_DATA_VC9_OFFSET: u32 = 9;
    pub const MASK_ERR_FRAME_DATA_VC9_MASK: u32 = 0x00000200;
    pub const MASK_ERR_FRAME_DATA_VC9: u32 = 1 << 9;

    /// **MASK_ERR_FRAME_DATA_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MASK_ERR_FRAME_DATA_VC8_OFFSET: u32 = 8;
    pub const MASK_ERR_FRAME_DATA_VC8_MASK: u32 = 0x00000100;
    pub const MASK_ERR_FRAME_DATA_VC8: u32 = 1 << 8;

    /// **MASK_ERR_FRAME_DATA_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MASK_ERR_FRAME_DATA_VC7_OFFSET: u32 = 7;
    pub const MASK_ERR_FRAME_DATA_VC7_MASK: u32 = 0x00000080;
    pub const MASK_ERR_FRAME_DATA_VC7: u32 = 1 << 7;

    /// **MASK_ERR_FRAME_DATA_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MASK_ERR_FRAME_DATA_VC6_OFFSET: u32 = 6;
    pub const MASK_ERR_FRAME_DATA_VC6_MASK: u32 = 0x00000040;
    pub const MASK_ERR_FRAME_DATA_VC6: u32 = 1 << 6;

    /// **MASK_ERR_FRAME_DATA_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const MASK_ERR_FRAME_DATA_VC5_OFFSET: u32 = 5;
    pub const MASK_ERR_FRAME_DATA_VC5_MASK: u32 = 0x00000020;
    pub const MASK_ERR_FRAME_DATA_VC5: u32 = 1 << 5;

    /// **MASK_ERR_FRAME_DATA_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MASK_ERR_FRAME_DATA_VC4_OFFSET: u32 = 4;
    pub const MASK_ERR_FRAME_DATA_VC4_MASK: u32 = 0x00000010;
    pub const MASK_ERR_FRAME_DATA_VC4: u32 = 1 << 4;

    /// **MASK_ERR_FRAME_DATA_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MASK_ERR_FRAME_DATA_VC3_OFFSET: u32 = 3;
    pub const MASK_ERR_FRAME_DATA_VC3_MASK: u32 = 0x00000008;
    pub const MASK_ERR_FRAME_DATA_VC3: u32 = 1 << 3;

    /// **MASK_ERR_FRAME_DATA_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MASK_ERR_FRAME_DATA_VC2_OFFSET: u32 = 2;
    pub const MASK_ERR_FRAME_DATA_VC2_MASK: u32 = 0x00000004;
    pub const MASK_ERR_FRAME_DATA_VC2: u32 = 1 << 2;

    /// **MASK_ERR_FRAME_DATA_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MASK_ERR_FRAME_DATA_VC1_OFFSET: u32 = 1;
    pub const MASK_ERR_FRAME_DATA_VC1_MASK: u32 = 0x00000002;
    pub const MASK_ERR_FRAME_DATA_VC1: u32 = 1 << 1;

    /// **MASK_ERR_FRAME_DATA_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MASK_ERR_FRAME_DATA_VC0_OFFSET: u32 = 0;
    pub const MASK_ERR_FRAME_DATA_VC0_MASK: u32 = 0x00000001;
    pub const MASK_ERR_FRAME_DATA_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MASK_ERR_FRAME_DATA_VC15** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc15(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC15_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc15(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC15_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc15(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC15_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC14** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc14(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC14_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc14(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC14_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc14(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC14_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC13** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc13(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC13_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc13(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC13_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc13(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC13_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC12** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc12(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC12_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc12(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC12_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc12(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC12_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC11** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc11(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC11_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc11(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC11_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc11(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC11_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC10** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc10(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC10_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc10(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC10_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc10(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC10_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC9** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc9(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC9_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc9(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC9_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc9(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC9_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC8** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc8(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC8_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc8(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC8_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc8(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC8_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC7** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc7(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC7_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc7(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC7_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc7(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC7_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC6** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc6(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC6_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc6(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC6_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc6(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC6_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC5** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc5(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC5_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc5(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC5_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc5(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC5_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC4** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc4(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC4_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc4(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC4_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc4(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC4_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC3** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc3(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC3_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc3(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC3_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC3_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC2** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc2(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC2_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc2(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC2_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC2_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc1(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC1_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc1(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC1_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC1_MASK);
    }

    /// **MASK_ERR_FRAME_DATA_VC0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_frame_data_vc0(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_FRAME_DATA_VC0_MASK) != 0
    }

    /// Set MASK_ERR_FRAME_DATA_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_frame_data_vc0(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_FRAME_DATA_VC0_MASK);
    }

    /// Clear MASK_ERR_FRAME_DATA_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_frame_data_vc0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_FRAME_DATA_VC0_MASK);
    }
}

/// INT_MSK_CRC_FRAME_FATAL value type for batch read/modify/write operations
///
/// Use `intmskcrcframefatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intmskcrcframefatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntMskCrcFrameFatalVal {
    bits: u32,
}

impl IntMskCrcFrameFatalVal {
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

    /// Read **MASK_ERR_FRAME_DATA_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc15(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC15_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc14(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC14_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc13(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC13_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc12(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC12_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc11(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC11_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc10(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC10_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc9(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC9_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc8(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC8_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc7(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC7_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc6(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC6_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc5(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC5_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc4(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC4_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc3(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC3_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc2(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC2_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc1(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC1_MASK) != 0
    }

    /// Read **MASK_ERR_FRAME_DATA_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_frame_data_vc0(&self) -> bool {
        (self.bits & IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC0_MASK) != 0
    }

    /// Set **MASK_ERR_FRAME_DATA_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC15_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC15_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC14_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC14_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC13_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC13_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC12_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC12_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC11_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC11_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC10_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC10_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC9_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC9_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC8_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC8_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC7_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC7_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC6_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC6_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC5_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC5_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC4_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC4_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC3_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC3_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC2_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC2_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC1_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC1_MASK
            },
        }
    }

    /// Set **MASK_ERR_FRAME_DATA_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_frame_data_vc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC0_MASK
            } else {
                self.bits & !IntMskCrcFrameFatal::MASK_ERR_FRAME_DATA_VC0_MASK
            },
        }
    }
}

impl Default for IntMskCrcFrameFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntMskCrcFrameFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntMskCrcFrameFatalVal {
        IntMskCrcFrameFatalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntMskCrcFrameFatalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntMskCrcFrameFatalVal) -> IntMskCrcFrameFatalVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_FORCE_CRC_FRAME_FATAL Register
///
/// NA
///
/// Address offset: 0x02a8
///
/// ## Fields
///
/// - **FORCE_ERR_FRAME_DATA_VC15** `[15]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC14** `[14]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC13** `[13]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC12** `[12]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC11** `[11]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC10** `[10]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC9** `[9]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC8** `[8]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC7** `[7]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC6** `[6]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC5** `[5]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC4** `[4]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC3** `[3]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC2** `[2]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC1** `[1]` (RW)
///
///   NA
///
/// - **FORCE_ERR_FRAME_DATA_VC0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntForceCrcFrameFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntForceCrcFrameFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntForceCrcFrameFatal {
    // ======== Field Constants ========

    /// **FORCE_ERR_FRAME_DATA_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const FORCE_ERR_FRAME_DATA_VC15_OFFSET: u32 = 15;
    pub const FORCE_ERR_FRAME_DATA_VC15_MASK: u32 = 0x00008000;
    pub const FORCE_ERR_FRAME_DATA_VC15: u32 = 1 << 15;

    /// **FORCE_ERR_FRAME_DATA_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const FORCE_ERR_FRAME_DATA_VC14_OFFSET: u32 = 14;
    pub const FORCE_ERR_FRAME_DATA_VC14_MASK: u32 = 0x00004000;
    pub const FORCE_ERR_FRAME_DATA_VC14: u32 = 1 << 14;

    /// **FORCE_ERR_FRAME_DATA_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const FORCE_ERR_FRAME_DATA_VC13_OFFSET: u32 = 13;
    pub const FORCE_ERR_FRAME_DATA_VC13_MASK: u32 = 0x00002000;
    pub const FORCE_ERR_FRAME_DATA_VC13: u32 = 1 << 13;

    /// **FORCE_ERR_FRAME_DATA_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const FORCE_ERR_FRAME_DATA_VC12_OFFSET: u32 = 12;
    pub const FORCE_ERR_FRAME_DATA_VC12_MASK: u32 = 0x00001000;
    pub const FORCE_ERR_FRAME_DATA_VC12: u32 = 1 << 12;

    /// **FORCE_ERR_FRAME_DATA_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const FORCE_ERR_FRAME_DATA_VC11_OFFSET: u32 = 11;
    pub const FORCE_ERR_FRAME_DATA_VC11_MASK: u32 = 0x00000800;
    pub const FORCE_ERR_FRAME_DATA_VC11: u32 = 1 << 11;

    /// **FORCE_ERR_FRAME_DATA_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const FORCE_ERR_FRAME_DATA_VC10_OFFSET: u32 = 10;
    pub const FORCE_ERR_FRAME_DATA_VC10_MASK: u32 = 0x00000400;
    pub const FORCE_ERR_FRAME_DATA_VC10: u32 = 1 << 10;

    /// **FORCE_ERR_FRAME_DATA_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const FORCE_ERR_FRAME_DATA_VC9_OFFSET: u32 = 9;
    pub const FORCE_ERR_FRAME_DATA_VC9_MASK: u32 = 0x00000200;
    pub const FORCE_ERR_FRAME_DATA_VC9: u32 = 1 << 9;

    /// **FORCE_ERR_FRAME_DATA_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const FORCE_ERR_FRAME_DATA_VC8_OFFSET: u32 = 8;
    pub const FORCE_ERR_FRAME_DATA_VC8_MASK: u32 = 0x00000100;
    pub const FORCE_ERR_FRAME_DATA_VC8: u32 = 1 << 8;

    /// **FORCE_ERR_FRAME_DATA_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const FORCE_ERR_FRAME_DATA_VC7_OFFSET: u32 = 7;
    pub const FORCE_ERR_FRAME_DATA_VC7_MASK: u32 = 0x00000080;
    pub const FORCE_ERR_FRAME_DATA_VC7: u32 = 1 << 7;

    /// **FORCE_ERR_FRAME_DATA_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const FORCE_ERR_FRAME_DATA_VC6_OFFSET: u32 = 6;
    pub const FORCE_ERR_FRAME_DATA_VC6_MASK: u32 = 0x00000040;
    pub const FORCE_ERR_FRAME_DATA_VC6: u32 = 1 << 6;

    /// **FORCE_ERR_FRAME_DATA_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const FORCE_ERR_FRAME_DATA_VC5_OFFSET: u32 = 5;
    pub const FORCE_ERR_FRAME_DATA_VC5_MASK: u32 = 0x00000020;
    pub const FORCE_ERR_FRAME_DATA_VC5: u32 = 1 << 5;

    /// **FORCE_ERR_FRAME_DATA_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const FORCE_ERR_FRAME_DATA_VC4_OFFSET: u32 = 4;
    pub const FORCE_ERR_FRAME_DATA_VC4_MASK: u32 = 0x00000010;
    pub const FORCE_ERR_FRAME_DATA_VC4: u32 = 1 << 4;

    /// **FORCE_ERR_FRAME_DATA_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const FORCE_ERR_FRAME_DATA_VC3_OFFSET: u32 = 3;
    pub const FORCE_ERR_FRAME_DATA_VC3_MASK: u32 = 0x00000008;
    pub const FORCE_ERR_FRAME_DATA_VC3: u32 = 1 << 3;

    /// **FORCE_ERR_FRAME_DATA_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FORCE_ERR_FRAME_DATA_VC2_OFFSET: u32 = 2;
    pub const FORCE_ERR_FRAME_DATA_VC2_MASK: u32 = 0x00000004;
    pub const FORCE_ERR_FRAME_DATA_VC2: u32 = 1 << 2;

    /// **FORCE_ERR_FRAME_DATA_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FORCE_ERR_FRAME_DATA_VC1_OFFSET: u32 = 1;
    pub const FORCE_ERR_FRAME_DATA_VC1_MASK: u32 = 0x00000002;
    pub const FORCE_ERR_FRAME_DATA_VC1: u32 = 1 << 1;

    /// **FORCE_ERR_FRAME_DATA_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FORCE_ERR_FRAME_DATA_VC0_OFFSET: u32 = 0;
    pub const FORCE_ERR_FRAME_DATA_VC0_MASK: u32 = 0x00000001;
    pub const FORCE_ERR_FRAME_DATA_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FORCE_ERR_FRAME_DATA_VC15** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc15(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC15_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc15(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC15_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc15(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC15_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC14** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc14(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC14_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc14(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC14_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc14(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC14_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC13** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc13(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC13_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc13(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC13_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc13(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC13_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC12** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc12(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC12_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc12(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC12_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc12(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC12_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC11** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc11(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC11_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc11(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC11_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc11(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC11_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC10** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc10(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC10_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc10(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC10_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc10(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC10_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC9** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc9(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC9_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc9(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC9_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc9(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC9_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC8** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc8(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC8_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc8(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC8_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc8(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC8_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC7** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc7(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC7_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc7(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC7_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc7(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC7_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC6** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc6(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC6_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc6(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC6_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc6(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC6_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC5** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc5(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC5_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc5(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC5_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc5(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC5_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC4** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc4(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC4_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc4(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC4_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc4(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC4_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC3** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc3(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC3_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc3(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC3_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc3(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC3_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC2** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc2(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC2_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc2(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC2_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc2(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC2_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc1(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC1_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc1(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC1_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc1(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC1_MASK);
    }

    /// **FORCE_ERR_FRAME_DATA_VC0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_frame_data_vc0(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_FRAME_DATA_VC0_MASK) != 0
    }

    /// Set FORCE_ERR_FRAME_DATA_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_frame_data_vc0(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_FRAME_DATA_VC0_MASK);
    }

    /// Clear FORCE_ERR_FRAME_DATA_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_frame_data_vc0(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_FRAME_DATA_VC0_MASK);
    }
}

/// INT_FORCE_CRC_FRAME_FATAL value type for batch read/modify/write operations
///
/// Use `intforcecrcframefatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intforcecrcframefatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntForceCrcFrameFatalVal {
    bits: u32,
}

impl IntForceCrcFrameFatalVal {
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

    /// Read **FORCE_ERR_FRAME_DATA_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc15(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC15_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc14(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC14_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc13(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC13_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc12(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC12_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc11(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC11_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc10(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC10_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc9(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC9_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc8(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC8_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc7(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC7_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc6(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC6_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc5(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC5_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc4(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC4_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc3(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC3_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc2(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC2_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc1(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC1_MASK) != 0
    }

    /// Read **FORCE_ERR_FRAME_DATA_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_frame_data_vc0(&self) -> bool {
        (self.bits & IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC0_MASK) != 0
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC15_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC15_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC14_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC14_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC13_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC13_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC12_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC12_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC11_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC11_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC10_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC10_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC9_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC9_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC8_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC8_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC7_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC7_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC6_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC6_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC5_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC5_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC4_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC4_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC3_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC3_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC2_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC2_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC1_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC1_MASK
            },
        }
    }

    /// Set **FORCE_ERR_FRAME_DATA_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_frame_data_vc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC0_MASK
            } else {
                self.bits & !IntForceCrcFrameFatal::FORCE_ERR_FRAME_DATA_VC0_MASK
            },
        }
    }
}

impl Default for IntForceCrcFrameFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntForceCrcFrameFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntForceCrcFrameFatalVal {
        IntForceCrcFrameFatalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntForceCrcFrameFatalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntForceCrcFrameFatalVal) -> IntForceCrcFrameFatalVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ST_PLD_CRC_FATAL Register
///
/// NA
///
/// Address offset: 0x02b0
///
/// ## Fields
///
/// - **ST_ERR_CRC_VC15** `[15]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC14** `[14]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC13** `[13]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC12** `[12]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC11** `[11]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC10** `[10]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC9** `[9]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC8** `[8]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC7** `[7]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC6** `[6]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC5** `[5]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC4** `[4]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC3** `[3]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC2** `[2]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC1** `[1]` (RO)
///
///   NA
///
/// - **ST_ERR_CRC_VC0** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntStPldCrcFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntStPldCrcFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntStPldCrcFatal {
    // ======== Field Constants ========

    /// **ST_ERR_CRC_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const ST_ERR_CRC_VC15_OFFSET: u32 = 15;
    pub const ST_ERR_CRC_VC15_MASK: u32 = 0x00008000;
    pub const ST_ERR_CRC_VC15: u32 = 1 << 15;

    /// **ST_ERR_CRC_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const ST_ERR_CRC_VC14_OFFSET: u32 = 14;
    pub const ST_ERR_CRC_VC14_MASK: u32 = 0x00004000;
    pub const ST_ERR_CRC_VC14: u32 = 1 << 14;

    /// **ST_ERR_CRC_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const ST_ERR_CRC_VC13_OFFSET: u32 = 13;
    pub const ST_ERR_CRC_VC13_MASK: u32 = 0x00002000;
    pub const ST_ERR_CRC_VC13: u32 = 1 << 13;

    /// **ST_ERR_CRC_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const ST_ERR_CRC_VC12_OFFSET: u32 = 12;
    pub const ST_ERR_CRC_VC12_MASK: u32 = 0x00001000;
    pub const ST_ERR_CRC_VC12: u32 = 1 << 12;

    /// **ST_ERR_CRC_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ST_ERR_CRC_VC11_OFFSET: u32 = 11;
    pub const ST_ERR_CRC_VC11_MASK: u32 = 0x00000800;
    pub const ST_ERR_CRC_VC11: u32 = 1 << 11;

    /// **ST_ERR_CRC_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ST_ERR_CRC_VC10_OFFSET: u32 = 10;
    pub const ST_ERR_CRC_VC10_MASK: u32 = 0x00000400;
    pub const ST_ERR_CRC_VC10: u32 = 1 << 10;

    /// **ST_ERR_CRC_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ST_ERR_CRC_VC9_OFFSET: u32 = 9;
    pub const ST_ERR_CRC_VC9_MASK: u32 = 0x00000200;
    pub const ST_ERR_CRC_VC9: u32 = 1 << 9;

    /// **ST_ERR_CRC_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ST_ERR_CRC_VC8_OFFSET: u32 = 8;
    pub const ST_ERR_CRC_VC8_MASK: u32 = 0x00000100;
    pub const ST_ERR_CRC_VC8: u32 = 1 << 8;

    /// **ST_ERR_CRC_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ST_ERR_CRC_VC7_OFFSET: u32 = 7;
    pub const ST_ERR_CRC_VC7_MASK: u32 = 0x00000080;
    pub const ST_ERR_CRC_VC7: u32 = 1 << 7;

    /// **ST_ERR_CRC_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ST_ERR_CRC_VC6_OFFSET: u32 = 6;
    pub const ST_ERR_CRC_VC6_MASK: u32 = 0x00000040;
    pub const ST_ERR_CRC_VC6: u32 = 1 << 6;

    /// **ST_ERR_CRC_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ST_ERR_CRC_VC5_OFFSET: u32 = 5;
    pub const ST_ERR_CRC_VC5_MASK: u32 = 0x00000020;
    pub const ST_ERR_CRC_VC5: u32 = 1 << 5;

    /// **ST_ERR_CRC_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ST_ERR_CRC_VC4_OFFSET: u32 = 4;
    pub const ST_ERR_CRC_VC4_MASK: u32 = 0x00000010;
    pub const ST_ERR_CRC_VC4: u32 = 1 << 4;

    /// **ST_ERR_CRC_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ST_ERR_CRC_VC3_OFFSET: u32 = 3;
    pub const ST_ERR_CRC_VC3_MASK: u32 = 0x00000008;
    pub const ST_ERR_CRC_VC3: u32 = 1 << 3;

    /// **ST_ERR_CRC_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ST_ERR_CRC_VC2_OFFSET: u32 = 2;
    pub const ST_ERR_CRC_VC2_MASK: u32 = 0x00000004;
    pub const ST_ERR_CRC_VC2: u32 = 1 << 2;

    /// **ST_ERR_CRC_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ST_ERR_CRC_VC1_OFFSET: u32 = 1;
    pub const ST_ERR_CRC_VC1_MASK: u32 = 0x00000002;
    pub const ST_ERR_CRC_VC1: u32 = 1 << 1;

    /// **ST_ERR_CRC_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ST_ERR_CRC_VC0_OFFSET: u32 = 0;
    pub const ST_ERR_CRC_VC0_MASK: u32 = 0x00000001;
    pub const ST_ERR_CRC_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ST_ERR_CRC_VC15** `[15]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc15(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC15_MASK) != 0
    }

    /// **ST_ERR_CRC_VC14** `[14]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc14(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC14_MASK) != 0
    }

    /// **ST_ERR_CRC_VC13** `[13]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc13(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC13_MASK) != 0
    }

    /// **ST_ERR_CRC_VC12** `[12]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc12(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC12_MASK) != 0
    }

    /// **ST_ERR_CRC_VC11** `[11]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc11(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC11_MASK) != 0
    }

    /// **ST_ERR_CRC_VC10** `[10]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc10(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC10_MASK) != 0
    }

    /// **ST_ERR_CRC_VC9** `[9]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc9(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC9_MASK) != 0
    }

    /// **ST_ERR_CRC_VC8** `[8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc8(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC8_MASK) != 0
    }

    /// **ST_ERR_CRC_VC7** `[7]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc7(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC7_MASK) != 0
    }

    /// **ST_ERR_CRC_VC6** `[6]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc6(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC6_MASK) != 0
    }

    /// **ST_ERR_CRC_VC5** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc5(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC5_MASK) != 0
    }

    /// **ST_ERR_CRC_VC4** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc4(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC4_MASK) != 0
    }

    /// **ST_ERR_CRC_VC3** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc3(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC3_MASK) != 0
    }

    /// **ST_ERR_CRC_VC2** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc2(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC2_MASK) != 0
    }

    /// **ST_ERR_CRC_VC1** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc1(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC1_MASK) != 0
    }

    /// **ST_ERR_CRC_VC0** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_crc_vc0(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_CRC_VC0_MASK) != 0
    }
}

/// INT_ST_PLD_CRC_FATAL value type for batch read/modify/write operations
///
/// Use `intstpldcrcfatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intstpldcrcfatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStPldCrcFatalVal {
    bits: u32,
}

impl IntStPldCrcFatalVal {
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

    /// Read **ST_ERR_CRC_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc15(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC15_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc14(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC14_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc13(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC13_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc12(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC12_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc11(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC11_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc10(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC10_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc9(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC9_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc8(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC8_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc7(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC7_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc6(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC6_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc5(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC5_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc4(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC4_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc3(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC3_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc2(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC2_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc1(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC1_MASK) != 0
    }

    /// Read **ST_ERR_CRC_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_crc_vc0(&self) -> bool {
        (self.bits & IntStPldCrcFatal::ST_ERR_CRC_VC0_MASK) != 0
    }
}

impl Default for IntStPldCrcFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntStPldCrcFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStPldCrcFatalVal {
        IntStPldCrcFatalVal::from_bits(self.value.read_bits())
    }
}

/// INT_MSK_PLD_CRC_FATAL Register
///
/// NA
///
/// Address offset: 0x02b4
///
/// ## Fields
///
/// - **MASK_ERR_CRC_VC15** `[15]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC14** `[14]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC13** `[13]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC12** `[12]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC11** `[11]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC10** `[10]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC9** `[9]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC8** `[8]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC7** `[7]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC6** `[6]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC5** `[5]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC4** `[4]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC3** `[3]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC2** `[2]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC1** `[1]` (RW)
///
///   NA
///
/// - **MASK_ERR_CRC_VC0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntMskPldCrcFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntMskPldCrcFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntMskPldCrcFatal {
    // ======== Field Constants ========

    /// **MASK_ERR_CRC_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const MASK_ERR_CRC_VC15_OFFSET: u32 = 15;
    pub const MASK_ERR_CRC_VC15_MASK: u32 = 0x00008000;
    pub const MASK_ERR_CRC_VC15: u32 = 1 << 15;

    /// **MASK_ERR_CRC_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MASK_ERR_CRC_VC14_OFFSET: u32 = 14;
    pub const MASK_ERR_CRC_VC14_MASK: u32 = 0x00004000;
    pub const MASK_ERR_CRC_VC14: u32 = 1 << 14;

    /// **MASK_ERR_CRC_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const MASK_ERR_CRC_VC13_OFFSET: u32 = 13;
    pub const MASK_ERR_CRC_VC13_MASK: u32 = 0x00002000;
    pub const MASK_ERR_CRC_VC13: u32 = 1 << 13;

    /// **MASK_ERR_CRC_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const MASK_ERR_CRC_VC12_OFFSET: u32 = 12;
    pub const MASK_ERR_CRC_VC12_MASK: u32 = 0x00001000;
    pub const MASK_ERR_CRC_VC12: u32 = 1 << 12;

    /// **MASK_ERR_CRC_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const MASK_ERR_CRC_VC11_OFFSET: u32 = 11;
    pub const MASK_ERR_CRC_VC11_MASK: u32 = 0x00000800;
    pub const MASK_ERR_CRC_VC11: u32 = 1 << 11;

    /// **MASK_ERR_CRC_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const MASK_ERR_CRC_VC10_OFFSET: u32 = 10;
    pub const MASK_ERR_CRC_VC10_MASK: u32 = 0x00000400;
    pub const MASK_ERR_CRC_VC10: u32 = 1 << 10;

    /// **MASK_ERR_CRC_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const MASK_ERR_CRC_VC9_OFFSET: u32 = 9;
    pub const MASK_ERR_CRC_VC9_MASK: u32 = 0x00000200;
    pub const MASK_ERR_CRC_VC9: u32 = 1 << 9;

    /// **MASK_ERR_CRC_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MASK_ERR_CRC_VC8_OFFSET: u32 = 8;
    pub const MASK_ERR_CRC_VC8_MASK: u32 = 0x00000100;
    pub const MASK_ERR_CRC_VC8: u32 = 1 << 8;

    /// **MASK_ERR_CRC_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MASK_ERR_CRC_VC7_OFFSET: u32 = 7;
    pub const MASK_ERR_CRC_VC7_MASK: u32 = 0x00000080;
    pub const MASK_ERR_CRC_VC7: u32 = 1 << 7;

    /// **MASK_ERR_CRC_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MASK_ERR_CRC_VC6_OFFSET: u32 = 6;
    pub const MASK_ERR_CRC_VC6_MASK: u32 = 0x00000040;
    pub const MASK_ERR_CRC_VC6: u32 = 1 << 6;

    /// **MASK_ERR_CRC_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const MASK_ERR_CRC_VC5_OFFSET: u32 = 5;
    pub const MASK_ERR_CRC_VC5_MASK: u32 = 0x00000020;
    pub const MASK_ERR_CRC_VC5: u32 = 1 << 5;

    /// **MASK_ERR_CRC_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MASK_ERR_CRC_VC4_OFFSET: u32 = 4;
    pub const MASK_ERR_CRC_VC4_MASK: u32 = 0x00000010;
    pub const MASK_ERR_CRC_VC4: u32 = 1 << 4;

    /// **MASK_ERR_CRC_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MASK_ERR_CRC_VC3_OFFSET: u32 = 3;
    pub const MASK_ERR_CRC_VC3_MASK: u32 = 0x00000008;
    pub const MASK_ERR_CRC_VC3: u32 = 1 << 3;

    /// **MASK_ERR_CRC_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MASK_ERR_CRC_VC2_OFFSET: u32 = 2;
    pub const MASK_ERR_CRC_VC2_MASK: u32 = 0x00000004;
    pub const MASK_ERR_CRC_VC2: u32 = 1 << 2;

    /// **MASK_ERR_CRC_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MASK_ERR_CRC_VC1_OFFSET: u32 = 1;
    pub const MASK_ERR_CRC_VC1_MASK: u32 = 0x00000002;
    pub const MASK_ERR_CRC_VC1: u32 = 1 << 1;

    /// **MASK_ERR_CRC_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MASK_ERR_CRC_VC0_OFFSET: u32 = 0;
    pub const MASK_ERR_CRC_VC0_MASK: u32 = 0x00000001;
    pub const MASK_ERR_CRC_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MASK_ERR_CRC_VC15** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc15(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC15_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc15(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC15_MASK);
    }

    /// Clear MASK_ERR_CRC_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc15(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_CRC_VC15_MASK);
    }

    /// **MASK_ERR_CRC_VC14** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc14(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC14_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc14(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC14_MASK);
    }

    /// Clear MASK_ERR_CRC_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc14(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_CRC_VC14_MASK);
    }

    /// **MASK_ERR_CRC_VC13** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc13(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC13_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc13(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC13_MASK);
    }

    /// Clear MASK_ERR_CRC_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc13(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_CRC_VC13_MASK);
    }

    /// **MASK_ERR_CRC_VC12** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc12(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC12_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc12(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC12_MASK);
    }

    /// Clear MASK_ERR_CRC_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc12(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_CRC_VC12_MASK);
    }

    /// **MASK_ERR_CRC_VC11** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc11(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC11_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc11(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC11_MASK);
    }

    /// Clear MASK_ERR_CRC_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc11(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_CRC_VC11_MASK);
    }

    /// **MASK_ERR_CRC_VC10** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc10(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC10_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc10(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC10_MASK);
    }

    /// Clear MASK_ERR_CRC_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc10(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_CRC_VC10_MASK);
    }

    /// **MASK_ERR_CRC_VC9** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc9(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC9_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc9(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC9_MASK);
    }

    /// Clear MASK_ERR_CRC_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc9(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_CRC_VC9_MASK);
    }

    /// **MASK_ERR_CRC_VC8** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc8(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC8_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc8(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC8_MASK);
    }

    /// Clear MASK_ERR_CRC_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc8(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_CRC_VC8_MASK);
    }

    /// **MASK_ERR_CRC_VC7** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc7(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC7_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc7(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC7_MASK);
    }

    /// Clear MASK_ERR_CRC_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc7(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_CRC_VC7_MASK);
    }

    /// **MASK_ERR_CRC_VC6** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc6(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC6_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc6(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC6_MASK);
    }

    /// Clear MASK_ERR_CRC_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc6(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_CRC_VC6_MASK);
    }

    /// **MASK_ERR_CRC_VC5** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc5(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC5_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc5(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC5_MASK);
    }

    /// Clear MASK_ERR_CRC_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc5(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_CRC_VC5_MASK);
    }

    /// **MASK_ERR_CRC_VC4** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc4(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC4_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc4(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC4_MASK);
    }

    /// Clear MASK_ERR_CRC_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc4(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_CRC_VC4_MASK);
    }

    /// **MASK_ERR_CRC_VC3** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc3(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC3_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc3(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC3_MASK);
    }

    /// Clear MASK_ERR_CRC_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc3(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_CRC_VC3_MASK);
    }

    /// **MASK_ERR_CRC_VC2** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc2(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC2_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc2(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC2_MASK);
    }

    /// Clear MASK_ERR_CRC_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc2(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_CRC_VC2_MASK);
    }

    /// **MASK_ERR_CRC_VC1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc1(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC1_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc1(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC1_MASK);
    }

    /// Clear MASK_ERR_CRC_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc1(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_CRC_VC1_MASK);
    }

    /// **MASK_ERR_CRC_VC0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_crc_vc0(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_CRC_VC0_MASK) != 0
    }

    /// Set MASK_ERR_CRC_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_crc_vc0(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_CRC_VC0_MASK);
    }

    /// Clear MASK_ERR_CRC_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_crc_vc0(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_CRC_VC0_MASK);
    }
}

/// INT_MSK_PLD_CRC_FATAL value type for batch read/modify/write operations
///
/// Use `intmskpldcrcfatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intmskpldcrcfatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntMskPldCrcFatalVal {
    bits: u32,
}

impl IntMskPldCrcFatalVal {
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

    /// Read **MASK_ERR_CRC_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc15(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC15_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc14(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC14_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc13(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC13_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc12(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC12_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc11(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC11_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc10(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC10_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc9(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC9_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc8(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC8_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc7(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC7_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc6(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC6_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc5(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC5_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc4(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC4_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc3(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC3_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc2(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC2_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc1(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC1_MASK) != 0
    }

    /// Read **MASK_ERR_CRC_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_crc_vc0(&self) -> bool {
        (self.bits & IntMskPldCrcFatal::MASK_ERR_CRC_VC0_MASK) != 0
    }

    /// Set **MASK_ERR_CRC_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC15_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC15_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC14_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC14_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC13_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC13_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC12_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC12_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC11_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC11_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC10_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC10_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC9_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC9_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC8_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC8_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC7_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC7_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC6_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC6_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC5_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC5_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC4_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC4_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC3_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC3_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC2_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC2_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC1_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC1_MASK
            },
        }
    }

    /// Set **MASK_ERR_CRC_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_crc_vc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskPldCrcFatal::MASK_ERR_CRC_VC0_MASK
            } else {
                self.bits & !IntMskPldCrcFatal::MASK_ERR_CRC_VC0_MASK
            },
        }
    }
}

impl Default for IntMskPldCrcFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntMskPldCrcFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntMskPldCrcFatalVal {
        IntMskPldCrcFatalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntMskPldCrcFatalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntMskPldCrcFatalVal) -> IntMskPldCrcFatalVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_FORCE_PLD_CRC_FATAL Register
///
/// NA
///
/// Address offset: 0x02b8
///
/// ## Fields
///
/// - **FORCE_ERR_CRC_VC15** `[15]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC14** `[14]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC13** `[13]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC12** `[12]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC11** `[11]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC10** `[10]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC9** `[9]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC8** `[8]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC7** `[7]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC6** `[6]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC5** `[5]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC4** `[4]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC3** `[3]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC2** `[2]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC1** `[1]` (RW)
///
///   NA
///
/// - **FORCE_ERR_CRC_VC0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntForcePldCrcFatal {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntForcePldCrcFatal {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntForcePldCrcFatal {
    // ======== Field Constants ========

    /// **FORCE_ERR_CRC_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const FORCE_ERR_CRC_VC15_OFFSET: u32 = 15;
    pub const FORCE_ERR_CRC_VC15_MASK: u32 = 0x00008000;
    pub const FORCE_ERR_CRC_VC15: u32 = 1 << 15;

    /// **FORCE_ERR_CRC_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const FORCE_ERR_CRC_VC14_OFFSET: u32 = 14;
    pub const FORCE_ERR_CRC_VC14_MASK: u32 = 0x00004000;
    pub const FORCE_ERR_CRC_VC14: u32 = 1 << 14;

    /// **FORCE_ERR_CRC_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const FORCE_ERR_CRC_VC13_OFFSET: u32 = 13;
    pub const FORCE_ERR_CRC_VC13_MASK: u32 = 0x00002000;
    pub const FORCE_ERR_CRC_VC13: u32 = 1 << 13;

    /// **FORCE_ERR_CRC_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const FORCE_ERR_CRC_VC12_OFFSET: u32 = 12;
    pub const FORCE_ERR_CRC_VC12_MASK: u32 = 0x00001000;
    pub const FORCE_ERR_CRC_VC12: u32 = 1 << 12;

    /// **FORCE_ERR_CRC_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const FORCE_ERR_CRC_VC11_OFFSET: u32 = 11;
    pub const FORCE_ERR_CRC_VC11_MASK: u32 = 0x00000800;
    pub const FORCE_ERR_CRC_VC11: u32 = 1 << 11;

    /// **FORCE_ERR_CRC_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const FORCE_ERR_CRC_VC10_OFFSET: u32 = 10;
    pub const FORCE_ERR_CRC_VC10_MASK: u32 = 0x00000400;
    pub const FORCE_ERR_CRC_VC10: u32 = 1 << 10;

    /// **FORCE_ERR_CRC_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const FORCE_ERR_CRC_VC9_OFFSET: u32 = 9;
    pub const FORCE_ERR_CRC_VC9_MASK: u32 = 0x00000200;
    pub const FORCE_ERR_CRC_VC9: u32 = 1 << 9;

    /// **FORCE_ERR_CRC_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const FORCE_ERR_CRC_VC8_OFFSET: u32 = 8;
    pub const FORCE_ERR_CRC_VC8_MASK: u32 = 0x00000100;
    pub const FORCE_ERR_CRC_VC8: u32 = 1 << 8;

    /// **FORCE_ERR_CRC_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const FORCE_ERR_CRC_VC7_OFFSET: u32 = 7;
    pub const FORCE_ERR_CRC_VC7_MASK: u32 = 0x00000080;
    pub const FORCE_ERR_CRC_VC7: u32 = 1 << 7;

    /// **FORCE_ERR_CRC_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const FORCE_ERR_CRC_VC6_OFFSET: u32 = 6;
    pub const FORCE_ERR_CRC_VC6_MASK: u32 = 0x00000040;
    pub const FORCE_ERR_CRC_VC6: u32 = 1 << 6;

    /// **FORCE_ERR_CRC_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const FORCE_ERR_CRC_VC5_OFFSET: u32 = 5;
    pub const FORCE_ERR_CRC_VC5_MASK: u32 = 0x00000020;
    pub const FORCE_ERR_CRC_VC5: u32 = 1 << 5;

    /// **FORCE_ERR_CRC_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const FORCE_ERR_CRC_VC4_OFFSET: u32 = 4;
    pub const FORCE_ERR_CRC_VC4_MASK: u32 = 0x00000010;
    pub const FORCE_ERR_CRC_VC4: u32 = 1 << 4;

    /// **FORCE_ERR_CRC_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const FORCE_ERR_CRC_VC3_OFFSET: u32 = 3;
    pub const FORCE_ERR_CRC_VC3_MASK: u32 = 0x00000008;
    pub const FORCE_ERR_CRC_VC3: u32 = 1 << 3;

    /// **FORCE_ERR_CRC_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FORCE_ERR_CRC_VC2_OFFSET: u32 = 2;
    pub const FORCE_ERR_CRC_VC2_MASK: u32 = 0x00000004;
    pub const FORCE_ERR_CRC_VC2: u32 = 1 << 2;

    /// **FORCE_ERR_CRC_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FORCE_ERR_CRC_VC1_OFFSET: u32 = 1;
    pub const FORCE_ERR_CRC_VC1_MASK: u32 = 0x00000002;
    pub const FORCE_ERR_CRC_VC1: u32 = 1 << 1;

    /// **FORCE_ERR_CRC_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FORCE_ERR_CRC_VC0_OFFSET: u32 = 0;
    pub const FORCE_ERR_CRC_VC0_MASK: u32 = 0x00000001;
    pub const FORCE_ERR_CRC_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FORCE_ERR_CRC_VC15** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc15(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC15_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc15(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_CRC_VC15_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc15(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC15_MASK);
    }

    /// **FORCE_ERR_CRC_VC14** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc14(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC14_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc14(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_CRC_VC14_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc14(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC14_MASK);
    }

    /// **FORCE_ERR_CRC_VC13** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc13(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC13_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc13(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_CRC_VC13_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc13(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC13_MASK);
    }

    /// **FORCE_ERR_CRC_VC12** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc12(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC12_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc12(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_CRC_VC12_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc12(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC12_MASK);
    }

    /// **FORCE_ERR_CRC_VC11** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc11(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC11_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc11(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_CRC_VC11_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc11(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC11_MASK);
    }

    /// **FORCE_ERR_CRC_VC10** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc10(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC10_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc10(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_CRC_VC10_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc10(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC10_MASK);
    }

    /// **FORCE_ERR_CRC_VC9** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc9(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC9_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc9(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_CRC_VC9_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc9(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC9_MASK);
    }

    /// **FORCE_ERR_CRC_VC8** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc8(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC8_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc8(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_CRC_VC8_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc8(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC8_MASK);
    }

    /// **FORCE_ERR_CRC_VC7** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc7(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC7_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc7(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_CRC_VC7_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc7(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC7_MASK);
    }

    /// **FORCE_ERR_CRC_VC6** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc6(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC6_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc6(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_CRC_VC6_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc6(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC6_MASK);
    }

    /// **FORCE_ERR_CRC_VC5** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc5(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC5_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc5(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_CRC_VC5_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc5(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC5_MASK);
    }

    /// **FORCE_ERR_CRC_VC4** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc4(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC4_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc4(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_CRC_VC4_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc4(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC4_MASK);
    }

    /// **FORCE_ERR_CRC_VC3** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc3(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC3_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc3(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_CRC_VC3_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc3(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC3_MASK);
    }

    /// **FORCE_ERR_CRC_VC2** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc2(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC2_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc2(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_CRC_VC2_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc2(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC2_MASK);
    }

    /// **FORCE_ERR_CRC_VC1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc1(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC1_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc1(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_CRC_VC1_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc1(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC1_MASK);
    }

    /// **FORCE_ERR_CRC_VC0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_crc_vc0(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_CRC_VC0_MASK) != 0
    }

    /// Set FORCE_ERR_CRC_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_crc_vc0(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_CRC_VC0_MASK);
    }

    /// Clear FORCE_ERR_CRC_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_crc_vc0(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_CRC_VC0_MASK);
    }
}

/// INT_FORCE_PLD_CRC_FATAL value type for batch read/modify/write operations
///
/// Use `intforcepldcrcfatal.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intforcepldcrcfatal.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntForcePldCrcFatalVal {
    bits: u32,
}

impl IntForcePldCrcFatalVal {
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

    /// Read **FORCE_ERR_CRC_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc15(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC15_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc14(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC14_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc13(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC13_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc12(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC12_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc11(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC11_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc10(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC10_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc9(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC9_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc8(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC8_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc7(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC7_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc6(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC6_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc5(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC5_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc4(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC4_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc3(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC3_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc2(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC2_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc1(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC1_MASK) != 0
    }

    /// Read **FORCE_ERR_CRC_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_crc_vc0(&self) -> bool {
        (self.bits & IntForcePldCrcFatal::FORCE_ERR_CRC_VC0_MASK) != 0
    }

    /// Set **FORCE_ERR_CRC_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC15_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC15_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC14_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC14_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC13_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC13_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC12_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC12_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC11_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC11_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC10_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC10_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC9_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC9_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC8_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC8_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC7_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC7_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC6_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC6_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC5_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC5_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC4_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC4_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC3_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC3_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC2_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC2_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC1_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC1_MASK
            },
        }
    }

    /// Set **FORCE_ERR_CRC_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_crc_vc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForcePldCrcFatal::FORCE_ERR_CRC_VC0_MASK
            } else {
                self.bits & !IntForcePldCrcFatal::FORCE_ERR_CRC_VC0_MASK
            },
        }
    }
}

impl Default for IntForcePldCrcFatalVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntForcePldCrcFatal {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntForcePldCrcFatalVal {
        IntForcePldCrcFatalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntForcePldCrcFatalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntForcePldCrcFatalVal) -> IntForcePldCrcFatalVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ST_DATA_ID Register
///
/// NA
///
/// Address offset: 0x02c0
///
/// ## Fields
///
/// - **ST_ERR_ID_VC15** `[15]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC14** `[14]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC13** `[13]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC12** `[12]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC11** `[11]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC10** `[10]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC9** `[9]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC8** `[8]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC7** `[7]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC6** `[6]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC5** `[5]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC4** `[4]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC3** `[3]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC2** `[2]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC1** `[1]` (RO)
///
///   NA
///
/// - **ST_ERR_ID_VC0** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntStDataId {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntStDataId {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntStDataId {
    // ======== Field Constants ========

    /// **ST_ERR_ID_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const ST_ERR_ID_VC15_OFFSET: u32 = 15;
    pub const ST_ERR_ID_VC15_MASK: u32 = 0x00008000;
    pub const ST_ERR_ID_VC15: u32 = 1 << 15;

    /// **ST_ERR_ID_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const ST_ERR_ID_VC14_OFFSET: u32 = 14;
    pub const ST_ERR_ID_VC14_MASK: u32 = 0x00004000;
    pub const ST_ERR_ID_VC14: u32 = 1 << 14;

    /// **ST_ERR_ID_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const ST_ERR_ID_VC13_OFFSET: u32 = 13;
    pub const ST_ERR_ID_VC13_MASK: u32 = 0x00002000;
    pub const ST_ERR_ID_VC13: u32 = 1 << 13;

    /// **ST_ERR_ID_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const ST_ERR_ID_VC12_OFFSET: u32 = 12;
    pub const ST_ERR_ID_VC12_MASK: u32 = 0x00001000;
    pub const ST_ERR_ID_VC12: u32 = 1 << 12;

    /// **ST_ERR_ID_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ST_ERR_ID_VC11_OFFSET: u32 = 11;
    pub const ST_ERR_ID_VC11_MASK: u32 = 0x00000800;
    pub const ST_ERR_ID_VC11: u32 = 1 << 11;

    /// **ST_ERR_ID_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ST_ERR_ID_VC10_OFFSET: u32 = 10;
    pub const ST_ERR_ID_VC10_MASK: u32 = 0x00000400;
    pub const ST_ERR_ID_VC10: u32 = 1 << 10;

    /// **ST_ERR_ID_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ST_ERR_ID_VC9_OFFSET: u32 = 9;
    pub const ST_ERR_ID_VC9_MASK: u32 = 0x00000200;
    pub const ST_ERR_ID_VC9: u32 = 1 << 9;

    /// **ST_ERR_ID_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ST_ERR_ID_VC8_OFFSET: u32 = 8;
    pub const ST_ERR_ID_VC8_MASK: u32 = 0x00000100;
    pub const ST_ERR_ID_VC8: u32 = 1 << 8;

    /// **ST_ERR_ID_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ST_ERR_ID_VC7_OFFSET: u32 = 7;
    pub const ST_ERR_ID_VC7_MASK: u32 = 0x00000080;
    pub const ST_ERR_ID_VC7: u32 = 1 << 7;

    /// **ST_ERR_ID_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ST_ERR_ID_VC6_OFFSET: u32 = 6;
    pub const ST_ERR_ID_VC6_MASK: u32 = 0x00000040;
    pub const ST_ERR_ID_VC6: u32 = 1 << 6;

    /// **ST_ERR_ID_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ST_ERR_ID_VC5_OFFSET: u32 = 5;
    pub const ST_ERR_ID_VC5_MASK: u32 = 0x00000020;
    pub const ST_ERR_ID_VC5: u32 = 1 << 5;

    /// **ST_ERR_ID_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ST_ERR_ID_VC4_OFFSET: u32 = 4;
    pub const ST_ERR_ID_VC4_MASK: u32 = 0x00000010;
    pub const ST_ERR_ID_VC4: u32 = 1 << 4;

    /// **ST_ERR_ID_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ST_ERR_ID_VC3_OFFSET: u32 = 3;
    pub const ST_ERR_ID_VC3_MASK: u32 = 0x00000008;
    pub const ST_ERR_ID_VC3: u32 = 1 << 3;

    /// **ST_ERR_ID_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ST_ERR_ID_VC2_OFFSET: u32 = 2;
    pub const ST_ERR_ID_VC2_MASK: u32 = 0x00000004;
    pub const ST_ERR_ID_VC2: u32 = 1 << 2;

    /// **ST_ERR_ID_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ST_ERR_ID_VC1_OFFSET: u32 = 1;
    pub const ST_ERR_ID_VC1_MASK: u32 = 0x00000002;
    pub const ST_ERR_ID_VC1: u32 = 1 << 1;

    /// **ST_ERR_ID_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ST_ERR_ID_VC0_OFFSET: u32 = 0;
    pub const ST_ERR_ID_VC0_MASK: u32 = 0x00000001;
    pub const ST_ERR_ID_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ST_ERR_ID_VC15** `[15]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc15(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC15_MASK) != 0
    }

    /// **ST_ERR_ID_VC14** `[14]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc14(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC14_MASK) != 0
    }

    /// **ST_ERR_ID_VC13** `[13]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc13(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC13_MASK) != 0
    }

    /// **ST_ERR_ID_VC12** `[12]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc12(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC12_MASK) != 0
    }

    /// **ST_ERR_ID_VC11** `[11]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc11(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC11_MASK) != 0
    }

    /// **ST_ERR_ID_VC10** `[10]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc10(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC10_MASK) != 0
    }

    /// **ST_ERR_ID_VC9** `[9]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc9(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC9_MASK) != 0
    }

    /// **ST_ERR_ID_VC8** `[8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc8(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC8_MASK) != 0
    }

    /// **ST_ERR_ID_VC7** `[7]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc7(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC7_MASK) != 0
    }

    /// **ST_ERR_ID_VC6** `[6]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc6(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC6_MASK) != 0
    }

    /// **ST_ERR_ID_VC5** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc5(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC5_MASK) != 0
    }

    /// **ST_ERR_ID_VC4** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc4(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC4_MASK) != 0
    }

    /// **ST_ERR_ID_VC3** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc3(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC3_MASK) != 0
    }

    /// **ST_ERR_ID_VC2** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc2(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC2_MASK) != 0
    }

    /// **ST_ERR_ID_VC1** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc1(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC1_MASK) != 0
    }

    /// **ST_ERR_ID_VC0** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_id_vc0(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ID_VC0_MASK) != 0
    }
}

/// INT_ST_DATA_ID value type for batch read/modify/write operations
///
/// Use `intstdataid.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intstdataid.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStDataIdVal {
    bits: u32,
}

impl IntStDataIdVal {
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

    /// Read **ST_ERR_ID_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc15(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC15_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc14(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC14_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc13(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC13_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc12(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC12_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc11(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC11_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc10(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC10_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc9(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC9_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc8(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC8_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc7(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC7_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc6(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC6_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc5(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC5_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc4(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC4_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc3(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC3_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc2(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC2_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc1(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC1_MASK) != 0
    }

    /// Read **ST_ERR_ID_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_id_vc0(&self) -> bool {
        (self.bits & IntStDataId::ST_ERR_ID_VC0_MASK) != 0
    }
}

impl Default for IntStDataIdVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntStDataId {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStDataIdVal {
        IntStDataIdVal::from_bits(self.value.read_bits())
    }
}

/// INT_MSK_DATA_ID Register
///
/// NA
///
/// Address offset: 0x02c4
///
/// ## Fields
///
/// - **MASK_ERR_ID_VC15** `[15]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC14** `[14]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC13** `[13]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC12** `[12]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC11** `[11]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC10** `[10]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC9** `[9]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC8** `[8]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC7** `[7]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC6** `[6]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC5** `[5]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC4** `[4]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC3** `[3]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC2** `[2]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC1** `[1]` (RW)
///
///   NA
///
/// - **MASK_ERR_ID_VC0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntMskDataId {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntMskDataId {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntMskDataId {
    // ======== Field Constants ========

    /// **MASK_ERR_ID_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const MASK_ERR_ID_VC15_OFFSET: u32 = 15;
    pub const MASK_ERR_ID_VC15_MASK: u32 = 0x00008000;
    pub const MASK_ERR_ID_VC15: u32 = 1 << 15;

    /// **MASK_ERR_ID_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MASK_ERR_ID_VC14_OFFSET: u32 = 14;
    pub const MASK_ERR_ID_VC14_MASK: u32 = 0x00004000;
    pub const MASK_ERR_ID_VC14: u32 = 1 << 14;

    /// **MASK_ERR_ID_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const MASK_ERR_ID_VC13_OFFSET: u32 = 13;
    pub const MASK_ERR_ID_VC13_MASK: u32 = 0x00002000;
    pub const MASK_ERR_ID_VC13: u32 = 1 << 13;

    /// **MASK_ERR_ID_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const MASK_ERR_ID_VC12_OFFSET: u32 = 12;
    pub const MASK_ERR_ID_VC12_MASK: u32 = 0x00001000;
    pub const MASK_ERR_ID_VC12: u32 = 1 << 12;

    /// **MASK_ERR_ID_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const MASK_ERR_ID_VC11_OFFSET: u32 = 11;
    pub const MASK_ERR_ID_VC11_MASK: u32 = 0x00000800;
    pub const MASK_ERR_ID_VC11: u32 = 1 << 11;

    /// **MASK_ERR_ID_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const MASK_ERR_ID_VC10_OFFSET: u32 = 10;
    pub const MASK_ERR_ID_VC10_MASK: u32 = 0x00000400;
    pub const MASK_ERR_ID_VC10: u32 = 1 << 10;

    /// **MASK_ERR_ID_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const MASK_ERR_ID_VC9_OFFSET: u32 = 9;
    pub const MASK_ERR_ID_VC9_MASK: u32 = 0x00000200;
    pub const MASK_ERR_ID_VC9: u32 = 1 << 9;

    /// **MASK_ERR_ID_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MASK_ERR_ID_VC8_OFFSET: u32 = 8;
    pub const MASK_ERR_ID_VC8_MASK: u32 = 0x00000100;
    pub const MASK_ERR_ID_VC8: u32 = 1 << 8;

    /// **MASK_ERR_ID_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MASK_ERR_ID_VC7_OFFSET: u32 = 7;
    pub const MASK_ERR_ID_VC7_MASK: u32 = 0x00000080;
    pub const MASK_ERR_ID_VC7: u32 = 1 << 7;

    /// **MASK_ERR_ID_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MASK_ERR_ID_VC6_OFFSET: u32 = 6;
    pub const MASK_ERR_ID_VC6_MASK: u32 = 0x00000040;
    pub const MASK_ERR_ID_VC6: u32 = 1 << 6;

    /// **MASK_ERR_ID_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const MASK_ERR_ID_VC5_OFFSET: u32 = 5;
    pub const MASK_ERR_ID_VC5_MASK: u32 = 0x00000020;
    pub const MASK_ERR_ID_VC5: u32 = 1 << 5;

    /// **MASK_ERR_ID_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MASK_ERR_ID_VC4_OFFSET: u32 = 4;
    pub const MASK_ERR_ID_VC4_MASK: u32 = 0x00000010;
    pub const MASK_ERR_ID_VC4: u32 = 1 << 4;

    /// **MASK_ERR_ID_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MASK_ERR_ID_VC3_OFFSET: u32 = 3;
    pub const MASK_ERR_ID_VC3_MASK: u32 = 0x00000008;
    pub const MASK_ERR_ID_VC3: u32 = 1 << 3;

    /// **MASK_ERR_ID_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MASK_ERR_ID_VC2_OFFSET: u32 = 2;
    pub const MASK_ERR_ID_VC2_MASK: u32 = 0x00000004;
    pub const MASK_ERR_ID_VC2: u32 = 1 << 2;

    /// **MASK_ERR_ID_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MASK_ERR_ID_VC1_OFFSET: u32 = 1;
    pub const MASK_ERR_ID_VC1_MASK: u32 = 0x00000002;
    pub const MASK_ERR_ID_VC1: u32 = 1 << 1;

    /// **MASK_ERR_ID_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MASK_ERR_ID_VC0_OFFSET: u32 = 0;
    pub const MASK_ERR_ID_VC0_MASK: u32 = 0x00000001;
    pub const MASK_ERR_ID_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MASK_ERR_ID_VC15** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc15(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC15_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc15(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC15_MASK);
    }

    /// Clear MASK_ERR_ID_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc15(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC15_MASK);
    }

    /// **MASK_ERR_ID_VC14** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc14(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC14_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc14(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC14_MASK);
    }

    /// Clear MASK_ERR_ID_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc14(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC14_MASK);
    }

    /// **MASK_ERR_ID_VC13** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc13(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC13_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc13(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC13_MASK);
    }

    /// Clear MASK_ERR_ID_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc13(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC13_MASK);
    }

    /// **MASK_ERR_ID_VC12** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc12(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC12_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc12(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC12_MASK);
    }

    /// Clear MASK_ERR_ID_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc12(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC12_MASK);
    }

    /// **MASK_ERR_ID_VC11** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc11(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC11_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc11(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC11_MASK);
    }

    /// Clear MASK_ERR_ID_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc11(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC11_MASK);
    }

    /// **MASK_ERR_ID_VC10** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc10(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC10_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc10(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC10_MASK);
    }

    /// Clear MASK_ERR_ID_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc10(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC10_MASK);
    }

    /// **MASK_ERR_ID_VC9** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc9(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC9_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc9(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC9_MASK);
    }

    /// Clear MASK_ERR_ID_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc9(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC9_MASK);
    }

    /// **MASK_ERR_ID_VC8** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc8(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC8_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc8(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC8_MASK);
    }

    /// Clear MASK_ERR_ID_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc8(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC8_MASK);
    }

    /// **MASK_ERR_ID_VC7** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc7(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC7_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc7(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC7_MASK);
    }

    /// Clear MASK_ERR_ID_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc7(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC7_MASK);
    }

    /// **MASK_ERR_ID_VC6** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc6(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC6_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc6(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC6_MASK);
    }

    /// Clear MASK_ERR_ID_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc6(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC6_MASK);
    }

    /// **MASK_ERR_ID_VC5** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc5(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC5_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc5(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC5_MASK);
    }

    /// Clear MASK_ERR_ID_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc5(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC5_MASK);
    }

    /// **MASK_ERR_ID_VC4** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc4(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC4_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc4(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC4_MASK);
    }

    /// Clear MASK_ERR_ID_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc4(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC4_MASK);
    }

    /// **MASK_ERR_ID_VC3** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc3(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC3_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc3(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC3_MASK);
    }

    /// Clear MASK_ERR_ID_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc3(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC3_MASK);
    }

    /// **MASK_ERR_ID_VC2** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc2(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC2_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc2(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC2_MASK);
    }

    /// Clear MASK_ERR_ID_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc2(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC2_MASK);
    }

    /// **MASK_ERR_ID_VC1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc1(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC1_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc1(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC1_MASK);
    }

    /// Clear MASK_ERR_ID_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc1(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC1_MASK);
    }

    /// **MASK_ERR_ID_VC0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_id_vc0(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ID_VC0_MASK) != 0
    }

    /// Set MASK_ERR_ID_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_id_vc0(&self) {
        self.value.modify_bits(|r| r | Self::MASK_ERR_ID_VC0_MASK);
    }

    /// Clear MASK_ERR_ID_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_id_vc0(&self) {
        self.value.modify_bits(|r| r & !Self::MASK_ERR_ID_VC0_MASK);
    }
}

/// INT_MSK_DATA_ID value type for batch read/modify/write operations
///
/// Use `intmskdataid.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intmskdataid.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntMskDataIdVal {
    bits: u32,
}

impl IntMskDataIdVal {
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

    /// Read **MASK_ERR_ID_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc15(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC15_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc14(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC14_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc13(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC13_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc12(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC12_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc11(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC11_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc10(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC10_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc9(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC9_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc8(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC8_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc7(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC7_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc6(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC6_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc5(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC5_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc4(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC4_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc3(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC3_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc2(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC2_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc1(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC1_MASK) != 0
    }

    /// Read **MASK_ERR_ID_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_id_vc0(&self) -> bool {
        (self.bits & IntMskDataId::MASK_ERR_ID_VC0_MASK) != 0
    }

    /// Set **MASK_ERR_ID_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC15_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC15_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC14_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC14_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC13_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC13_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC12_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC12_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC11_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC11_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC10_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC10_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC9_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC9_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC8_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC8_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC7_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC7_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC6_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC6_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC5_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC5_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC4_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC4_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC3_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC3_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC2_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC2_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC1_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC1_MASK
            },
        }
    }

    /// Set **MASK_ERR_ID_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_id_vc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskDataId::MASK_ERR_ID_VC0_MASK
            } else {
                self.bits & !IntMskDataId::MASK_ERR_ID_VC0_MASK
            },
        }
    }
}

impl Default for IntMskDataIdVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntMskDataId {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntMskDataIdVal {
        IntMskDataIdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntMskDataIdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntMskDataIdVal) -> IntMskDataIdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_FORCE_DATA_ID Register
///
/// NA
///
/// Address offset: 0x02c8
///
/// ## Fields
///
/// - **FORCE_ERR_ID_VC15** `[15]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC14** `[14]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC13** `[13]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC12** `[12]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC11** `[11]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC10** `[10]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC9** `[9]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC8** `[8]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC7** `[7]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC6** `[6]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC5** `[5]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC4** `[4]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC3** `[3]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC2** `[2]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC1** `[1]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ID_VC0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntForceDataId {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntForceDataId {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntForceDataId {
    // ======== Field Constants ========

    /// **FORCE_ERR_ID_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const FORCE_ERR_ID_VC15_OFFSET: u32 = 15;
    pub const FORCE_ERR_ID_VC15_MASK: u32 = 0x00008000;
    pub const FORCE_ERR_ID_VC15: u32 = 1 << 15;

    /// **FORCE_ERR_ID_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const FORCE_ERR_ID_VC14_OFFSET: u32 = 14;
    pub const FORCE_ERR_ID_VC14_MASK: u32 = 0x00004000;
    pub const FORCE_ERR_ID_VC14: u32 = 1 << 14;

    /// **FORCE_ERR_ID_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const FORCE_ERR_ID_VC13_OFFSET: u32 = 13;
    pub const FORCE_ERR_ID_VC13_MASK: u32 = 0x00002000;
    pub const FORCE_ERR_ID_VC13: u32 = 1 << 13;

    /// **FORCE_ERR_ID_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const FORCE_ERR_ID_VC12_OFFSET: u32 = 12;
    pub const FORCE_ERR_ID_VC12_MASK: u32 = 0x00001000;
    pub const FORCE_ERR_ID_VC12: u32 = 1 << 12;

    /// **FORCE_ERR_ID_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const FORCE_ERR_ID_VC11_OFFSET: u32 = 11;
    pub const FORCE_ERR_ID_VC11_MASK: u32 = 0x00000800;
    pub const FORCE_ERR_ID_VC11: u32 = 1 << 11;

    /// **FORCE_ERR_ID_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const FORCE_ERR_ID_VC10_OFFSET: u32 = 10;
    pub const FORCE_ERR_ID_VC10_MASK: u32 = 0x00000400;
    pub const FORCE_ERR_ID_VC10: u32 = 1 << 10;

    /// **FORCE_ERR_ID_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const FORCE_ERR_ID_VC9_OFFSET: u32 = 9;
    pub const FORCE_ERR_ID_VC9_MASK: u32 = 0x00000200;
    pub const FORCE_ERR_ID_VC9: u32 = 1 << 9;

    /// **FORCE_ERR_ID_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const FORCE_ERR_ID_VC8_OFFSET: u32 = 8;
    pub const FORCE_ERR_ID_VC8_MASK: u32 = 0x00000100;
    pub const FORCE_ERR_ID_VC8: u32 = 1 << 8;

    /// **FORCE_ERR_ID_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const FORCE_ERR_ID_VC7_OFFSET: u32 = 7;
    pub const FORCE_ERR_ID_VC7_MASK: u32 = 0x00000080;
    pub const FORCE_ERR_ID_VC7: u32 = 1 << 7;

    /// **FORCE_ERR_ID_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const FORCE_ERR_ID_VC6_OFFSET: u32 = 6;
    pub const FORCE_ERR_ID_VC6_MASK: u32 = 0x00000040;
    pub const FORCE_ERR_ID_VC6: u32 = 1 << 6;

    /// **FORCE_ERR_ID_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const FORCE_ERR_ID_VC5_OFFSET: u32 = 5;
    pub const FORCE_ERR_ID_VC5_MASK: u32 = 0x00000020;
    pub const FORCE_ERR_ID_VC5: u32 = 1 << 5;

    /// **FORCE_ERR_ID_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const FORCE_ERR_ID_VC4_OFFSET: u32 = 4;
    pub const FORCE_ERR_ID_VC4_MASK: u32 = 0x00000010;
    pub const FORCE_ERR_ID_VC4: u32 = 1 << 4;

    /// **FORCE_ERR_ID_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const FORCE_ERR_ID_VC3_OFFSET: u32 = 3;
    pub const FORCE_ERR_ID_VC3_MASK: u32 = 0x00000008;
    pub const FORCE_ERR_ID_VC3: u32 = 1 << 3;

    /// **FORCE_ERR_ID_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FORCE_ERR_ID_VC2_OFFSET: u32 = 2;
    pub const FORCE_ERR_ID_VC2_MASK: u32 = 0x00000004;
    pub const FORCE_ERR_ID_VC2: u32 = 1 << 2;

    /// **FORCE_ERR_ID_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FORCE_ERR_ID_VC1_OFFSET: u32 = 1;
    pub const FORCE_ERR_ID_VC1_MASK: u32 = 0x00000002;
    pub const FORCE_ERR_ID_VC1: u32 = 1 << 1;

    /// **FORCE_ERR_ID_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FORCE_ERR_ID_VC0_OFFSET: u32 = 0;
    pub const FORCE_ERR_ID_VC0_MASK: u32 = 0x00000001;
    pub const FORCE_ERR_ID_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FORCE_ERR_ID_VC15** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc15(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC15_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc15(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC15_MASK);
    }

    /// Clear FORCE_ERR_ID_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc15(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ID_VC15_MASK);
    }

    /// **FORCE_ERR_ID_VC14** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc14(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC14_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc14(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC14_MASK);
    }

    /// Clear FORCE_ERR_ID_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc14(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ID_VC14_MASK);
    }

    /// **FORCE_ERR_ID_VC13** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc13(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC13_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc13(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC13_MASK);
    }

    /// Clear FORCE_ERR_ID_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc13(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ID_VC13_MASK);
    }

    /// **FORCE_ERR_ID_VC12** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc12(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC12_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc12(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC12_MASK);
    }

    /// Clear FORCE_ERR_ID_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc12(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ID_VC12_MASK);
    }

    /// **FORCE_ERR_ID_VC11** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc11(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC11_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc11(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC11_MASK);
    }

    /// Clear FORCE_ERR_ID_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc11(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ID_VC11_MASK);
    }

    /// **FORCE_ERR_ID_VC10** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc10(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC10_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc10(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC10_MASK);
    }

    /// Clear FORCE_ERR_ID_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc10(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ID_VC10_MASK);
    }

    /// **FORCE_ERR_ID_VC9** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc9(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC9_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc9(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC9_MASK);
    }

    /// Clear FORCE_ERR_ID_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc9(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_ERR_ID_VC9_MASK);
    }

    /// **FORCE_ERR_ID_VC8** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc8(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC8_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc8(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC8_MASK);
    }

    /// Clear FORCE_ERR_ID_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc8(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_ERR_ID_VC8_MASK);
    }

    /// **FORCE_ERR_ID_VC7** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc7(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC7_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc7(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC7_MASK);
    }

    /// Clear FORCE_ERR_ID_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc7(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_ERR_ID_VC7_MASK);
    }

    /// **FORCE_ERR_ID_VC6** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc6(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC6_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc6(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC6_MASK);
    }

    /// Clear FORCE_ERR_ID_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc6(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_ERR_ID_VC6_MASK);
    }

    /// **FORCE_ERR_ID_VC5** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc5(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC5_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc5(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC5_MASK);
    }

    /// Clear FORCE_ERR_ID_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc5(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_ERR_ID_VC5_MASK);
    }

    /// **FORCE_ERR_ID_VC4** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc4(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC4_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc4(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC4_MASK);
    }

    /// Clear FORCE_ERR_ID_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc4(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_ERR_ID_VC4_MASK);
    }

    /// **FORCE_ERR_ID_VC3** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc3(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC3_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc3(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC3_MASK);
    }

    /// Clear FORCE_ERR_ID_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc3(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_ERR_ID_VC3_MASK);
    }

    /// **FORCE_ERR_ID_VC2** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc2(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC2_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc2(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC2_MASK);
    }

    /// Clear FORCE_ERR_ID_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc2(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_ERR_ID_VC2_MASK);
    }

    /// **FORCE_ERR_ID_VC1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc1(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC1_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc1(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC1_MASK);
    }

    /// Clear FORCE_ERR_ID_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc1(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_ERR_ID_VC1_MASK);
    }

    /// **FORCE_ERR_ID_VC0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_id_vc0(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ID_VC0_MASK) != 0
    }

    /// Set FORCE_ERR_ID_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_id_vc0(&self) {
        self.value.modify_bits(|r| r | Self::FORCE_ERR_ID_VC0_MASK);
    }

    /// Clear FORCE_ERR_ID_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_id_vc0(&self) {
        self.value.modify_bits(|r| r & !Self::FORCE_ERR_ID_VC0_MASK);
    }
}

/// INT_FORCE_DATA_ID value type for batch read/modify/write operations
///
/// Use `intforcedataid.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intforcedataid.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntForceDataIdVal {
    bits: u32,
}

impl IntForceDataIdVal {
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

    /// Read **FORCE_ERR_ID_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc15(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC15_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc14(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC14_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc13(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC13_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc12(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC12_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc11(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC11_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc10(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC10_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc9(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC9_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc8(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC8_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc7(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC7_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc6(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC6_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc5(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC5_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc4(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC4_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc3(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC3_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc2(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC2_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc1(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC1_MASK) != 0
    }

    /// Read **FORCE_ERR_ID_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_id_vc0(&self) -> bool {
        (self.bits & IntForceDataId::FORCE_ERR_ID_VC0_MASK) != 0
    }

    /// Set **FORCE_ERR_ID_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC15_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC15_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC14_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC14_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC13_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC13_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC12_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC12_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC11_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC11_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC10_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC10_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC9_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC9_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC8_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC8_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC7_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC7_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC6_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC6_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC5_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC5_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC4_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC4_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC3_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC3_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC2_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC2_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC1_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC1_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ID_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_id_vc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceDataId::FORCE_ERR_ID_VC0_MASK
            } else {
                self.bits & !IntForceDataId::FORCE_ERR_ID_VC0_MASK
            },
        }
    }
}

impl Default for IntForceDataIdVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntForceDataId {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntForceDataIdVal {
        IntForceDataIdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntForceDataIdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntForceDataIdVal) -> IntForceDataIdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ST_ECC_CORRECTED Register
///
/// NA
///
/// Address offset: 0x02d0
///
/// ## Fields
///
/// - **ST_ERR_ECC_CORRECTED_VC15** `[15]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC14** `[14]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC13** `[13]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC12** `[12]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC11** `[11]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC10** `[10]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC9** `[9]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC8** `[8]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC7** `[7]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC6** `[6]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC5** `[5]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC4** `[4]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC3** `[3]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC2** `[2]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC1** `[1]` (RO)
///
///   NA
///
/// - **ST_ERR_ECC_CORRECTED_VC0** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntStEccCorrected {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntStEccCorrected {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntStEccCorrected {
    // ======== Field Constants ========

    /// **ST_ERR_ECC_CORRECTED_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const ST_ERR_ECC_CORRECTED_VC15_OFFSET: u32 = 15;
    pub const ST_ERR_ECC_CORRECTED_VC15_MASK: u32 = 0x00008000;
    pub const ST_ERR_ECC_CORRECTED_VC15: u32 = 1 << 15;

    /// **ST_ERR_ECC_CORRECTED_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const ST_ERR_ECC_CORRECTED_VC14_OFFSET: u32 = 14;
    pub const ST_ERR_ECC_CORRECTED_VC14_MASK: u32 = 0x00004000;
    pub const ST_ERR_ECC_CORRECTED_VC14: u32 = 1 << 14;

    /// **ST_ERR_ECC_CORRECTED_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const ST_ERR_ECC_CORRECTED_VC13_OFFSET: u32 = 13;
    pub const ST_ERR_ECC_CORRECTED_VC13_MASK: u32 = 0x00002000;
    pub const ST_ERR_ECC_CORRECTED_VC13: u32 = 1 << 13;

    /// **ST_ERR_ECC_CORRECTED_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const ST_ERR_ECC_CORRECTED_VC12_OFFSET: u32 = 12;
    pub const ST_ERR_ECC_CORRECTED_VC12_MASK: u32 = 0x00001000;
    pub const ST_ERR_ECC_CORRECTED_VC12: u32 = 1 << 12;

    /// **ST_ERR_ECC_CORRECTED_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const ST_ERR_ECC_CORRECTED_VC11_OFFSET: u32 = 11;
    pub const ST_ERR_ECC_CORRECTED_VC11_MASK: u32 = 0x00000800;
    pub const ST_ERR_ECC_CORRECTED_VC11: u32 = 1 << 11;

    /// **ST_ERR_ECC_CORRECTED_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ST_ERR_ECC_CORRECTED_VC10_OFFSET: u32 = 10;
    pub const ST_ERR_ECC_CORRECTED_VC10_MASK: u32 = 0x00000400;
    pub const ST_ERR_ECC_CORRECTED_VC10: u32 = 1 << 10;

    /// **ST_ERR_ECC_CORRECTED_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const ST_ERR_ECC_CORRECTED_VC9_OFFSET: u32 = 9;
    pub const ST_ERR_ECC_CORRECTED_VC9_MASK: u32 = 0x00000200;
    pub const ST_ERR_ECC_CORRECTED_VC9: u32 = 1 << 9;

    /// **ST_ERR_ECC_CORRECTED_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ST_ERR_ECC_CORRECTED_VC8_OFFSET: u32 = 8;
    pub const ST_ERR_ECC_CORRECTED_VC8_MASK: u32 = 0x00000100;
    pub const ST_ERR_ECC_CORRECTED_VC8: u32 = 1 << 8;

    /// **ST_ERR_ECC_CORRECTED_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ST_ERR_ECC_CORRECTED_VC7_OFFSET: u32 = 7;
    pub const ST_ERR_ECC_CORRECTED_VC7_MASK: u32 = 0x00000080;
    pub const ST_ERR_ECC_CORRECTED_VC7: u32 = 1 << 7;

    /// **ST_ERR_ECC_CORRECTED_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ST_ERR_ECC_CORRECTED_VC6_OFFSET: u32 = 6;
    pub const ST_ERR_ECC_CORRECTED_VC6_MASK: u32 = 0x00000040;
    pub const ST_ERR_ECC_CORRECTED_VC6: u32 = 1 << 6;

    /// **ST_ERR_ECC_CORRECTED_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ST_ERR_ECC_CORRECTED_VC5_OFFSET: u32 = 5;
    pub const ST_ERR_ECC_CORRECTED_VC5_MASK: u32 = 0x00000020;
    pub const ST_ERR_ECC_CORRECTED_VC5: u32 = 1 << 5;

    /// **ST_ERR_ECC_CORRECTED_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ST_ERR_ECC_CORRECTED_VC4_OFFSET: u32 = 4;
    pub const ST_ERR_ECC_CORRECTED_VC4_MASK: u32 = 0x00000010;
    pub const ST_ERR_ECC_CORRECTED_VC4: u32 = 1 << 4;

    /// **ST_ERR_ECC_CORRECTED_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ST_ERR_ECC_CORRECTED_VC3_OFFSET: u32 = 3;
    pub const ST_ERR_ECC_CORRECTED_VC3_MASK: u32 = 0x00000008;
    pub const ST_ERR_ECC_CORRECTED_VC3: u32 = 1 << 3;

    /// **ST_ERR_ECC_CORRECTED_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ST_ERR_ECC_CORRECTED_VC2_OFFSET: u32 = 2;
    pub const ST_ERR_ECC_CORRECTED_VC2_MASK: u32 = 0x00000004;
    pub const ST_ERR_ECC_CORRECTED_VC2: u32 = 1 << 2;

    /// **ST_ERR_ECC_CORRECTED_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ST_ERR_ECC_CORRECTED_VC1_OFFSET: u32 = 1;
    pub const ST_ERR_ECC_CORRECTED_VC1_MASK: u32 = 0x00000002;
    pub const ST_ERR_ECC_CORRECTED_VC1: u32 = 1 << 1;

    /// **ST_ERR_ECC_CORRECTED_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ST_ERR_ECC_CORRECTED_VC0_OFFSET: u32 = 0;
    pub const ST_ERR_ECC_CORRECTED_VC0_MASK: u32 = 0x00000001;
    pub const ST_ERR_ECC_CORRECTED_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ST_ERR_ECC_CORRECTED_VC15** `[15]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc15(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC15_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC14** `[14]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc14(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC14_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC13** `[13]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc13(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC13_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC12** `[12]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc12(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC12_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC11** `[11]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc11(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC11_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC10** `[10]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc10(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC10_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC9** `[9]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc9(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC9_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC8** `[8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc8(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC8_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC7** `[7]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc7(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC7_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC6** `[6]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc6(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC6_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC5** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc5(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC5_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC4** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc4(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC4_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC3** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc3(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC3_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC2** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc2(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC2_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC1** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc1(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC1_MASK) != 0
    }

    /// **ST_ERR_ECC_CORRECTED_VC0** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn st_err_ecc_corrected_vc0(&self) -> bool {
        (self.value.read_bits() & Self::ST_ERR_ECC_CORRECTED_VC0_MASK) != 0
    }
}

/// INT_ST_ECC_CORRECTED value type for batch read/modify/write operations
///
/// Use `intstecccorrected.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intstecccorrected.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStEccCorrectedVal {
    bits: u32,
}

impl IntStEccCorrectedVal {
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

    /// Read **ST_ERR_ECC_CORRECTED_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc15(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC15_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc14(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC14_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc13(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC13_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc12(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC12_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc11(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC11_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc10(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC10_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc9(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC9_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc8(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC8_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc7(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC7_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc6(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC6_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc5(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC5_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc4(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC4_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc3(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC3_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc2(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC2_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc1(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC1_MASK) != 0
    }

    /// Read **ST_ERR_ECC_CORRECTED_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn st_err_ecc_corrected_vc0(&self) -> bool {
        (self.bits & IntStEccCorrected::ST_ERR_ECC_CORRECTED_VC0_MASK) != 0
    }
}

impl Default for IntStEccCorrectedVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntStEccCorrected {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStEccCorrectedVal {
        IntStEccCorrectedVal::from_bits(self.value.read_bits())
    }
}

/// INT_MSK_ECC_CORRECTED Register
///
/// NA
///
/// Address offset: 0x02d4
///
/// ## Fields
///
/// - **MASK_ERR_ECC_CORRECTED_VC15** `[15]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC14** `[14]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC13** `[13]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC12** `[12]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC11** `[11]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC10** `[10]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC9** `[9]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC8** `[8]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC7** `[7]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC6** `[6]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC5** `[5]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC4** `[4]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC3** `[3]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC2** `[2]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC1** `[1]` (RW)
///
///   NA
///
/// - **MASK_ERR_ECC_CORRECTED_VC0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntMskEccCorrected {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntMskEccCorrected {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntMskEccCorrected {
    // ======== Field Constants ========

    /// **MASK_ERR_ECC_CORRECTED_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const MASK_ERR_ECC_CORRECTED_VC15_OFFSET: u32 = 15;
    pub const MASK_ERR_ECC_CORRECTED_VC15_MASK: u32 = 0x00008000;
    pub const MASK_ERR_ECC_CORRECTED_VC15: u32 = 1 << 15;

    /// **MASK_ERR_ECC_CORRECTED_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const MASK_ERR_ECC_CORRECTED_VC14_OFFSET: u32 = 14;
    pub const MASK_ERR_ECC_CORRECTED_VC14_MASK: u32 = 0x00004000;
    pub const MASK_ERR_ECC_CORRECTED_VC14: u32 = 1 << 14;

    /// **MASK_ERR_ECC_CORRECTED_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const MASK_ERR_ECC_CORRECTED_VC13_OFFSET: u32 = 13;
    pub const MASK_ERR_ECC_CORRECTED_VC13_MASK: u32 = 0x00002000;
    pub const MASK_ERR_ECC_CORRECTED_VC13: u32 = 1 << 13;

    /// **MASK_ERR_ECC_CORRECTED_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const MASK_ERR_ECC_CORRECTED_VC12_OFFSET: u32 = 12;
    pub const MASK_ERR_ECC_CORRECTED_VC12_MASK: u32 = 0x00001000;
    pub const MASK_ERR_ECC_CORRECTED_VC12: u32 = 1 << 12;

    /// **MASK_ERR_ECC_CORRECTED_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const MASK_ERR_ECC_CORRECTED_VC11_OFFSET: u32 = 11;
    pub const MASK_ERR_ECC_CORRECTED_VC11_MASK: u32 = 0x00000800;
    pub const MASK_ERR_ECC_CORRECTED_VC11: u32 = 1 << 11;

    /// **MASK_ERR_ECC_CORRECTED_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const MASK_ERR_ECC_CORRECTED_VC10_OFFSET: u32 = 10;
    pub const MASK_ERR_ECC_CORRECTED_VC10_MASK: u32 = 0x00000400;
    pub const MASK_ERR_ECC_CORRECTED_VC10: u32 = 1 << 10;

    /// **MASK_ERR_ECC_CORRECTED_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const MASK_ERR_ECC_CORRECTED_VC9_OFFSET: u32 = 9;
    pub const MASK_ERR_ECC_CORRECTED_VC9_MASK: u32 = 0x00000200;
    pub const MASK_ERR_ECC_CORRECTED_VC9: u32 = 1 << 9;

    /// **MASK_ERR_ECC_CORRECTED_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MASK_ERR_ECC_CORRECTED_VC8_OFFSET: u32 = 8;
    pub const MASK_ERR_ECC_CORRECTED_VC8_MASK: u32 = 0x00000100;
    pub const MASK_ERR_ECC_CORRECTED_VC8: u32 = 1 << 8;

    /// **MASK_ERR_ECC_CORRECTED_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MASK_ERR_ECC_CORRECTED_VC7_OFFSET: u32 = 7;
    pub const MASK_ERR_ECC_CORRECTED_VC7_MASK: u32 = 0x00000080;
    pub const MASK_ERR_ECC_CORRECTED_VC7: u32 = 1 << 7;

    /// **MASK_ERR_ECC_CORRECTED_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MASK_ERR_ECC_CORRECTED_VC6_OFFSET: u32 = 6;
    pub const MASK_ERR_ECC_CORRECTED_VC6_MASK: u32 = 0x00000040;
    pub const MASK_ERR_ECC_CORRECTED_VC6: u32 = 1 << 6;

    /// **MASK_ERR_ECC_CORRECTED_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const MASK_ERR_ECC_CORRECTED_VC5_OFFSET: u32 = 5;
    pub const MASK_ERR_ECC_CORRECTED_VC5_MASK: u32 = 0x00000020;
    pub const MASK_ERR_ECC_CORRECTED_VC5: u32 = 1 << 5;

    /// **MASK_ERR_ECC_CORRECTED_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MASK_ERR_ECC_CORRECTED_VC4_OFFSET: u32 = 4;
    pub const MASK_ERR_ECC_CORRECTED_VC4_MASK: u32 = 0x00000010;
    pub const MASK_ERR_ECC_CORRECTED_VC4: u32 = 1 << 4;

    /// **MASK_ERR_ECC_CORRECTED_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MASK_ERR_ECC_CORRECTED_VC3_OFFSET: u32 = 3;
    pub const MASK_ERR_ECC_CORRECTED_VC3_MASK: u32 = 0x00000008;
    pub const MASK_ERR_ECC_CORRECTED_VC3: u32 = 1 << 3;

    /// **MASK_ERR_ECC_CORRECTED_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MASK_ERR_ECC_CORRECTED_VC2_OFFSET: u32 = 2;
    pub const MASK_ERR_ECC_CORRECTED_VC2_MASK: u32 = 0x00000004;
    pub const MASK_ERR_ECC_CORRECTED_VC2: u32 = 1 << 2;

    /// **MASK_ERR_ECC_CORRECTED_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MASK_ERR_ECC_CORRECTED_VC1_OFFSET: u32 = 1;
    pub const MASK_ERR_ECC_CORRECTED_VC1_MASK: u32 = 0x00000002;
    pub const MASK_ERR_ECC_CORRECTED_VC1: u32 = 1 << 1;

    /// **MASK_ERR_ECC_CORRECTED_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MASK_ERR_ECC_CORRECTED_VC0_OFFSET: u32 = 0;
    pub const MASK_ERR_ECC_CORRECTED_VC0_MASK: u32 = 0x00000001;
    pub const MASK_ERR_ECC_CORRECTED_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MASK_ERR_ECC_CORRECTED_VC15** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc15(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC15_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc15(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC15_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc15(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC15_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC14** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc14(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC14_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc14(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC14_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc14(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC14_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC13** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc13(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC13_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc13(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC13_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc13(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC13_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC12** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc12(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC12_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc12(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC12_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc12(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC12_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC11** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc11(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC11_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc11(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC11_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc11(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC11_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC10** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc10(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC10_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc10(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC10_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc10(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC10_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC9** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc9(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC9_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc9(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC9_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc9(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC9_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC8** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc8(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC8_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc8(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC8_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc8(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC8_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC7** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc7(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC7_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc7(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC7_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc7(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC7_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC6** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc6(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC6_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc6(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC6_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc6(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC6_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC5** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc5(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC5_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc5(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC5_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc5(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC5_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC4** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc4(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC4_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc4(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC4_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc4(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC4_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC3** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc3(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC3_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc3(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC3_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC3_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC2** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc2(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC2_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc2(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC2_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC2_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc1(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC1_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc1(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC1_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC1_MASK);
    }

    /// **MASK_ERR_ECC_CORRECTED_VC0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn mask_err_ecc_corrected_vc0(&self) -> bool {
        (self.value.read_bits() & Self::MASK_ERR_ECC_CORRECTED_VC0_MASK) != 0
    }

    /// Set MASK_ERR_ECC_CORRECTED_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mask_err_ecc_corrected_vc0(&self) {
        self.value
            .modify_bits(|r| r | Self::MASK_ERR_ECC_CORRECTED_VC0_MASK);
    }

    /// Clear MASK_ERR_ECC_CORRECTED_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mask_err_ecc_corrected_vc0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MASK_ERR_ECC_CORRECTED_VC0_MASK);
    }
}

/// INT_MSK_ECC_CORRECTED value type for batch read/modify/write operations
///
/// Use `intmskecccorrected.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intmskecccorrected.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntMskEccCorrectedVal {
    bits: u32,
}

impl IntMskEccCorrectedVal {
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

    /// Read **MASK_ERR_ECC_CORRECTED_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc15(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC15_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc14(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC14_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc13(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC13_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc12(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC12_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc11(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC11_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc10(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC10_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc9(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC9_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc8(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC8_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc7(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC7_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc6(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC6_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc5(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC5_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc4(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC4_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc3(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC3_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc2(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC2_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc1(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC1_MASK) != 0
    }

    /// Read **MASK_ERR_ECC_CORRECTED_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn mask_err_ecc_corrected_vc0(&self) -> bool {
        (self.bits & IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC0_MASK) != 0
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC15_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC15_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC14_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC14_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC13_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC13_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC12_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC12_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC11_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC11_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC10_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC10_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC9_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC9_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC8_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC8_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC7_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC7_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC6_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC6_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC5_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC5_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC4_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC4_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC3_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC3_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC2_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC2_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC1_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC1_MASK
            },
        }
    }

    /// Set **MASK_ERR_ECC_CORRECTED_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_mask_err_ecc_corrected_vc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC0_MASK
            } else {
                self.bits & !IntMskEccCorrected::MASK_ERR_ECC_CORRECTED_VC0_MASK
            },
        }
    }
}

impl Default for IntMskEccCorrectedVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntMskEccCorrected {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntMskEccCorrectedVal {
        IntMskEccCorrectedVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntMskEccCorrectedVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntMskEccCorrectedVal) -> IntMskEccCorrectedVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_FORCE_ECC_CORRECTED Register
///
/// NA
///
/// Address offset: 0x02d8
///
/// ## Fields
///
/// - **FORCE_ERR_ECC_CORRECTED_VC15** `[15]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC14** `[14]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC13** `[13]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC12** `[12]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC11** `[11]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC10** `[10]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC9** `[9]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC8** `[8]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC7** `[7]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC6** `[6]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC5** `[5]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC4** `[4]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC3** `[3]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC2** `[2]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC1** `[1]` (RW)
///
///   NA
///
/// - **FORCE_ERR_ECC_CORRECTED_VC0** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IntForceEccCorrected {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntForceEccCorrected {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntForceEccCorrected {
    // ======== Field Constants ========

    /// **FORCE_ERR_ECC_CORRECTED_VC15**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const FORCE_ERR_ECC_CORRECTED_VC15_OFFSET: u32 = 15;
    pub const FORCE_ERR_ECC_CORRECTED_VC15_MASK: u32 = 0x00008000;
    pub const FORCE_ERR_ECC_CORRECTED_VC15: u32 = 1 << 15;

    /// **FORCE_ERR_ECC_CORRECTED_VC14**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const FORCE_ERR_ECC_CORRECTED_VC14_OFFSET: u32 = 14;
    pub const FORCE_ERR_ECC_CORRECTED_VC14_MASK: u32 = 0x00004000;
    pub const FORCE_ERR_ECC_CORRECTED_VC14: u32 = 1 << 14;

    /// **FORCE_ERR_ECC_CORRECTED_VC13**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const FORCE_ERR_ECC_CORRECTED_VC13_OFFSET: u32 = 13;
    pub const FORCE_ERR_ECC_CORRECTED_VC13_MASK: u32 = 0x00002000;
    pub const FORCE_ERR_ECC_CORRECTED_VC13: u32 = 1 << 13;

    /// **FORCE_ERR_ECC_CORRECTED_VC12**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const FORCE_ERR_ECC_CORRECTED_VC12_OFFSET: u32 = 12;
    pub const FORCE_ERR_ECC_CORRECTED_VC12_MASK: u32 = 0x00001000;
    pub const FORCE_ERR_ECC_CORRECTED_VC12: u32 = 1 << 12;

    /// **FORCE_ERR_ECC_CORRECTED_VC11**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const FORCE_ERR_ECC_CORRECTED_VC11_OFFSET: u32 = 11;
    pub const FORCE_ERR_ECC_CORRECTED_VC11_MASK: u32 = 0x00000800;
    pub const FORCE_ERR_ECC_CORRECTED_VC11: u32 = 1 << 11;

    /// **FORCE_ERR_ECC_CORRECTED_VC10**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const FORCE_ERR_ECC_CORRECTED_VC10_OFFSET: u32 = 10;
    pub const FORCE_ERR_ECC_CORRECTED_VC10_MASK: u32 = 0x00000400;
    pub const FORCE_ERR_ECC_CORRECTED_VC10: u32 = 1 << 10;

    /// **FORCE_ERR_ECC_CORRECTED_VC9**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const FORCE_ERR_ECC_CORRECTED_VC9_OFFSET: u32 = 9;
    pub const FORCE_ERR_ECC_CORRECTED_VC9_MASK: u32 = 0x00000200;
    pub const FORCE_ERR_ECC_CORRECTED_VC9: u32 = 1 << 9;

    /// **FORCE_ERR_ECC_CORRECTED_VC8**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const FORCE_ERR_ECC_CORRECTED_VC8_OFFSET: u32 = 8;
    pub const FORCE_ERR_ECC_CORRECTED_VC8_MASK: u32 = 0x00000100;
    pub const FORCE_ERR_ECC_CORRECTED_VC8: u32 = 1 << 8;

    /// **FORCE_ERR_ECC_CORRECTED_VC7**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const FORCE_ERR_ECC_CORRECTED_VC7_OFFSET: u32 = 7;
    pub const FORCE_ERR_ECC_CORRECTED_VC7_MASK: u32 = 0x00000080;
    pub const FORCE_ERR_ECC_CORRECTED_VC7: u32 = 1 << 7;

    /// **FORCE_ERR_ECC_CORRECTED_VC6**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const FORCE_ERR_ECC_CORRECTED_VC6_OFFSET: u32 = 6;
    pub const FORCE_ERR_ECC_CORRECTED_VC6_MASK: u32 = 0x00000040;
    pub const FORCE_ERR_ECC_CORRECTED_VC6: u32 = 1 << 6;

    /// **FORCE_ERR_ECC_CORRECTED_VC5**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const FORCE_ERR_ECC_CORRECTED_VC5_OFFSET: u32 = 5;
    pub const FORCE_ERR_ECC_CORRECTED_VC5_MASK: u32 = 0x00000020;
    pub const FORCE_ERR_ECC_CORRECTED_VC5: u32 = 1 << 5;

    /// **FORCE_ERR_ECC_CORRECTED_VC4**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const FORCE_ERR_ECC_CORRECTED_VC4_OFFSET: u32 = 4;
    pub const FORCE_ERR_ECC_CORRECTED_VC4_MASK: u32 = 0x00000010;
    pub const FORCE_ERR_ECC_CORRECTED_VC4: u32 = 1 << 4;

    /// **FORCE_ERR_ECC_CORRECTED_VC3**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const FORCE_ERR_ECC_CORRECTED_VC3_OFFSET: u32 = 3;
    pub const FORCE_ERR_ECC_CORRECTED_VC3_MASK: u32 = 0x00000008;
    pub const FORCE_ERR_ECC_CORRECTED_VC3: u32 = 1 << 3;

    /// **FORCE_ERR_ECC_CORRECTED_VC2**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FORCE_ERR_ECC_CORRECTED_VC2_OFFSET: u32 = 2;
    pub const FORCE_ERR_ECC_CORRECTED_VC2_MASK: u32 = 0x00000004;
    pub const FORCE_ERR_ECC_CORRECTED_VC2: u32 = 1 << 2;

    /// **FORCE_ERR_ECC_CORRECTED_VC1**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const FORCE_ERR_ECC_CORRECTED_VC1_OFFSET: u32 = 1;
    pub const FORCE_ERR_ECC_CORRECTED_VC1_MASK: u32 = 0x00000002;
    pub const FORCE_ERR_ECC_CORRECTED_VC1: u32 = 1 << 1;

    /// **FORCE_ERR_ECC_CORRECTED_VC0**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FORCE_ERR_ECC_CORRECTED_VC0_OFFSET: u32 = 0;
    pub const FORCE_ERR_ECC_CORRECTED_VC0_MASK: u32 = 0x00000001;
    pub const FORCE_ERR_ECC_CORRECTED_VC0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FORCE_ERR_ECC_CORRECTED_VC15** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc15(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC15_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc15(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC15_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC15 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc15(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC15_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC14** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc14(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC14_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc14(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC14_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC14 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc14(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC14_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC13** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc13(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC13_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc13(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC13_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC13 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc13(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC13_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC12** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc12(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC12_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc12(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC12_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC12 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc12(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC12_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC11** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc11(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC11_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc11(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC11_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc11(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC11_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC10** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc10(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC10_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc10(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC10_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC10 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc10(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC10_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC9** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc9(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC9_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc9(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC9_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC9 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc9(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC9_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC8** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc8(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC8_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc8(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC8_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc8(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC8_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC7** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc7(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC7_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc7(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC7_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc7(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC7_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC6** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc6(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC6_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc6(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC6_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc6(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC6_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC5** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc5(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC5_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc5(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC5_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc5(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC5_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC4** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc4(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC4_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc4(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC4_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc4(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC4_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC3** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc3(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC3_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc3(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC3_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc3(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC3_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC2** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc2(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC2_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc2(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC2_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc2(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC2_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC1** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc1(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC1_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc1(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC1_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc1(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC1_MASK);
    }

    /// **FORCE_ERR_ECC_CORRECTED_VC0** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn force_err_ecc_corrected_vc0(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_ERR_ECC_CORRECTED_VC0_MASK) != 0
    }

    /// Set FORCE_ERR_ECC_CORRECTED_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_err_ecc_corrected_vc0(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_ERR_ECC_CORRECTED_VC0_MASK);
    }

    /// Clear FORCE_ERR_ECC_CORRECTED_VC0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_err_ecc_corrected_vc0(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_ERR_ECC_CORRECTED_VC0_MASK);
    }
}

/// INT_FORCE_ECC_CORRECTED value type for batch read/modify/write operations
///
/// Use `intforceecccorrected.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intforceecccorrected.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntForceEccCorrectedVal {
    bits: u32,
}

impl IntForceEccCorrectedVal {
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

    /// Read **FORCE_ERR_ECC_CORRECTED_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc15(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC15_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc14(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC14_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc13(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC13_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc12(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC12_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc11(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC11_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc10(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC10_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc9(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC9_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc8(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC8_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc7(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC7_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc6(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC6_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc5(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC5_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc4(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC4_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc3(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC3_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc2(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC2_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc1(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC1_MASK) != 0
    }

    /// Read **FORCE_ERR_ECC_CORRECTED_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn force_err_ecc_corrected_vc0(&self) -> bool {
        (self.bits & IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC0_MASK) != 0
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC15** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc15(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC15_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC15_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC14** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc14(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC14_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC14_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC13** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc13(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC13_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC13_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC12** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc12(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC12_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC12_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC11** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC11_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC11_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC10** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc10(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC10_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC10_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC9** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc9(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC9_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC9_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC8** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC8_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC8_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC7** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC7_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC7_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC6** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC6_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC6_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC5** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC5_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC5_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC4** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC4_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC4_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC3** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC3_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC3_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC2** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC2_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC2_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC1** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC1_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC1_MASK
            },
        }
    }

    /// Set **FORCE_ERR_ECC_CORRECTED_VC0** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_force_err_ecc_corrected_vc0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC0_MASK
            } else {
                self.bits & !IntForceEccCorrected::FORCE_ERR_ECC_CORRECTED_VC0_MASK
            },
        }
    }
}

impl Default for IntForceEccCorrectedVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntForceEccCorrected {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntForceEccCorrectedVal {
        IntForceEccCorrectedVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntForceEccCorrectedVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntForceEccCorrectedVal) -> IntForceEccCorrectedVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCRAMBLING Register
///
/// NA
///
/// Address offset: 0x0300
///
/// ## Fields
///
/// - **SCRAMBLE_ENABLE** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Scrambling {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Scrambling {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Scrambling {
    // ======== Field Constants ========

    /// **SCRAMBLE_ENABLE**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SCRAMBLE_ENABLE_OFFSET: u32 = 0;
    pub const SCRAMBLE_ENABLE_MASK: u32 = 0x00000001;
    pub const SCRAMBLE_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SCRAMBLE_ENABLE** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn scramble_enable(&self) -> bool {
        (self.value.read_bits() & Self::SCRAMBLE_ENABLE_MASK) != 0
    }

    /// Set SCRAMBLE_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scramble_enable(&self) {
        self.value.modify_bits(|r| r | Self::SCRAMBLE_ENABLE_MASK);
    }

    /// Clear SCRAMBLE_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scramble_enable(&self) {
        self.value.modify_bits(|r| r & !Self::SCRAMBLE_ENABLE_MASK);
    }
}

/// SCRAMBLING value type for batch read/modify/write operations
///
/// Use `scrambling.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scrambling.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ScramblingVal {
    bits: u32,
}

impl ScramblingVal {
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

    /// Read **SCRAMBLE_ENABLE** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn scramble_enable(&self) -> bool {
        (self.bits & Scrambling::SCRAMBLE_ENABLE_MASK) != 0
    }

    /// Set **SCRAMBLE_ENABLE** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_scramble_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Scrambling::SCRAMBLE_ENABLE_MASK
            } else {
                self.bits & !Scrambling::SCRAMBLE_ENABLE_MASK
            },
        }
    }
}

impl Default for ScramblingVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Scrambling {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ScramblingVal {
        ScramblingVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ScramblingVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ScramblingVal) -> ScramblingVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCRAMBLING_SEED1 Register
///
/// NA
///
/// Address offset: 0x0304
/// Reset value: 0x00001008
///
/// ## Fields
///
/// - **SCRAMBLE_SEED_LANE1** `[15:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct ScramblingSeed1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ScramblingSeed1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ScramblingSeed1 {
    // ======== Field Constants ========

    /// **SCRAMBLE_SEED_LANE1**
    ///
    /// NA
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SCRAMBLE_SEED_LANE1_OFFSET: u32 = 0;
    pub const SCRAMBLE_SEED_LANE1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SCRAMBLE_SEED_LANE1** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn scramble_seed_lane1(&self) -> u32 {
        (self.value.read_bits() & Self::SCRAMBLE_SEED_LANE1_MASK)
            >> Self::SCRAMBLE_SEED_LANE1_OFFSET
    }

    /// Write **SCRAMBLE_SEED_LANE1** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_scramble_seed_lane1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCRAMBLE_SEED_LANE1_MASK)
                | ((value << Self::SCRAMBLE_SEED_LANE1_OFFSET) & Self::SCRAMBLE_SEED_LANE1_MASK)
        });
    }
}

/// SCRAMBLING_SEED1 value type for batch read/modify/write operations
///
/// Use `scramblingseed1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scramblingseed1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ScramblingSeed1Val {
    bits: u32,
}

impl ScramblingSeed1Val {
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
        Self { bits: 0x00001008 }
    }

    /// Read **SCRAMBLE_SEED_LANE1** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn scramble_seed_lane1(&self) -> u32 {
        (self.bits & ScramblingSeed1::SCRAMBLE_SEED_LANE1_MASK)
            >> ScramblingSeed1::SCRAMBLE_SEED_LANE1_OFFSET
    }

    /// Set **SCRAMBLE_SEED_LANE1** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_scramble_seed_lane1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ScramblingSeed1::SCRAMBLE_SEED_LANE1_MASK)
                | ((value << ScramblingSeed1::SCRAMBLE_SEED_LANE1_OFFSET)
                    & ScramblingSeed1::SCRAMBLE_SEED_LANE1_MASK),
        }
    }
}

impl Default for ScramblingSeed1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ScramblingSeed1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ScramblingSeed1Val {
        ScramblingSeed1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ScramblingSeed1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ScramblingSeed1Val) -> ScramblingSeed1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCRAMBLING_SEED2 Register
///
/// NA
///
/// Address offset: 0x0308
/// Reset value: 0x00001188
///
/// ## Fields
///
/// - **SCRAMBLE_SEED_LANE2** `[15:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct ScramblingSeed2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ScramblingSeed2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ScramblingSeed2 {
    // ======== Field Constants ========

    /// **SCRAMBLE_SEED_LANE2**
    ///
    /// NA
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SCRAMBLE_SEED_LANE2_OFFSET: u32 = 0;
    pub const SCRAMBLE_SEED_LANE2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SCRAMBLE_SEED_LANE2** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn scramble_seed_lane2(&self) -> u32 {
        (self.value.read_bits() & Self::SCRAMBLE_SEED_LANE2_MASK)
            >> Self::SCRAMBLE_SEED_LANE2_OFFSET
    }

    /// Write **SCRAMBLE_SEED_LANE2** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_scramble_seed_lane2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SCRAMBLE_SEED_LANE2_MASK)
                | ((value << Self::SCRAMBLE_SEED_LANE2_OFFSET) & Self::SCRAMBLE_SEED_LANE2_MASK)
        });
    }
}

/// SCRAMBLING_SEED2 value type for batch read/modify/write operations
///
/// Use `scramblingseed2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scramblingseed2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ScramblingSeed2Val {
    bits: u32,
}

impl ScramblingSeed2Val {
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
        Self { bits: 0x00001188 }
    }

    /// Read **SCRAMBLE_SEED_LANE2** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn scramble_seed_lane2(&self) -> u32 {
        (self.bits & ScramblingSeed2::SCRAMBLE_SEED_LANE2_MASK)
            >> ScramblingSeed2::SCRAMBLE_SEED_LANE2_OFFSET
    }

    /// Set **SCRAMBLE_SEED_LANE2** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_scramble_seed_lane2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ScramblingSeed2::SCRAMBLE_SEED_LANE2_MASK)
                | ((value << ScramblingSeed2::SCRAMBLE_SEED_LANE2_OFFSET)
                    & ScramblingSeed2::SCRAMBLE_SEED_LANE2_MASK),
        }
    }
}

impl Default for ScramblingSeed2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ScramblingSeed2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ScramblingSeed2Val {
        ScramblingSeed2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ScramblingSeed2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ScramblingSeed2Val) -> ScramblingSeed2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
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
    pub n_lanes: NLanes,
    /// 0x0008
    /// NA
    pub csi2_resetn: Csi2Resetn,
    /// 0x000c
    /// NA
    pub int_st_main: IntStMain,
    _reserved0: [u8; 48],
    /// 0x0040
    /// NA
    pub phy_shutdownz: PhyShutdownz,
    /// 0x0044
    /// NA
    pub dphy_rstz: DphyRstz,
    /// 0x0048
    /// NA
    pub phy_rx: PhyRx,
    /// 0x004c
    /// NA
    pub phy_stopstate: PhyStopstate,
    /// 0x0050
    /// NA
    pub phy_test_ctrl0: PhyTestCtrl0,
    /// 0x0054
    /// NA
    pub phy_test_ctrl1: PhyTestCtrl1,
    _reserved1: [u8; 112],
    /// 0x00c8
    /// NA
    pub vc_extension: VcExtension,
    /// 0x00cc
    /// NA
    pub phy_cal: PhyCal,
    _reserved2: [u8; 16],
    /// 0x00e0
    /// NA
    pub int_st_phy_fatal: IntStPhyFatal,
    /// 0x00e4
    /// NA
    pub int_msk_phy_fatal: IntMskPhyFatal,
    /// 0x00e8
    /// NA
    pub int_force_phy_fatal: IntForcePhyFatal,
    _reserved3: [u8; 4],
    /// 0x00f0
    /// NA
    pub int_st_pkt_fatal: IntStPktFatal,
    /// 0x00f4
    /// NA
    pub int_msk_pkt_fatal: IntMskPktFatal,
    /// 0x00f8
    /// NA
    pub int_force_pkt_fatal: IntForcePktFatal,
    _reserved4: [u8; 20],
    /// 0x0110
    /// NA
    pub int_st_phy: IntStPhy,
    /// 0x0114
    /// NA
    pub int_msk_phy: IntMskPhy,
    /// 0x0118
    /// NA
    pub int_force_phy: IntForcePhy,
    _reserved5: [u8; 356],
    /// 0x0280
    /// NA
    pub int_st_bndry_frame_fatal: IntStBndryFrameFatal,
    /// 0x0284
    /// NA
    pub int_msk_bndry_frame_fatal: IntMskBndryFrameFatal,
    /// 0x0288
    /// NA
    pub int_force_bndry_frame_fatal: IntForceBndryFrameFatal,
    _reserved6: [u8; 4],
    /// 0x0290
    /// NA
    pub int_st_seq_frame_fatal: IntStSeqFrameFatal,
    /// 0x0294
    /// NA
    pub int_msk_seq_frame_fatal: IntMskSeqFrameFatal,
    /// 0x0298
    /// NA
    pub int_force_seq_frame_fatal: IntForceSeqFrameFatal,
    _reserved7: [u8; 4],
    /// 0x02a0
    /// NA
    pub int_st_crc_frame_fatal: IntStCrcFrameFatal,
    /// 0x02a4
    /// NA
    pub int_msk_crc_frame_fatal: IntMskCrcFrameFatal,
    /// 0x02a8
    /// NA
    pub int_force_crc_frame_fatal: IntForceCrcFrameFatal,
    _reserved8: [u8; 4],
    /// 0x02b0
    /// NA
    pub int_st_pld_crc_fatal: IntStPldCrcFatal,
    /// 0x02b4
    /// NA
    pub int_msk_pld_crc_fatal: IntMskPldCrcFatal,
    /// 0x02b8
    /// NA
    pub int_force_pld_crc_fatal: IntForcePldCrcFatal,
    _reserved9: [u8; 4],
    /// 0x02c0
    /// NA
    pub int_st_data_id: IntStDataId,
    /// 0x02c4
    /// NA
    pub int_msk_data_id: IntMskDataId,
    /// 0x02c8
    /// NA
    pub int_force_data_id: IntForceDataId,
    _reserved10: [u8; 4],
    /// 0x02d0
    /// NA
    pub int_st_ecc_corrected: IntStEccCorrected,
    /// 0x02d4
    /// NA
    pub int_msk_ecc_corrected: IntMskEccCorrected,
    /// 0x02d8
    /// NA
    pub int_force_ecc_corrected: IntForceEccCorrected,
    _reserved11: [u8; 36],
    /// 0x0300
    /// NA
    pub scrambling: Scrambling,
    /// 0x0304
    /// NA
    pub scrambling_seed1: ScramblingSeed1,
    /// 0x0308
    /// NA
    pub scrambling_seed2: ScramblingSeed2,
}
