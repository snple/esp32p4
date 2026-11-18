//! AXI_ICM
//! AXI_ICM Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x500a4000;

/// VERID_FILEDS Register
///
/// NA
///
/// Address offset: 0x0000
/// Reset value: 0x3430342a
///
/// ## Fields
///
/// - **ICM_REG_VERID** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct VeridFileds {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VeridFileds {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VeridFileds {
    // ======== Field Constants ========

    /// **ICM_REG_VERID**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ICM_REG_VERID_OFFSET: u32 = 0;
    pub const ICM_REG_VERID_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ICM_REG_VERID** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_verid(&self) -> u32 {
        (self.value.read_bits() & Self::ICM_REG_VERID_MASK) >> Self::ICM_REG_VERID_OFFSET
    }
}

/// VERID_FILEDS value type for batch read/modify/write operations
///
/// Use `veridfileds.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `veridfileds.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VeridFiledsVal {
    bits: u32,
}

impl VeridFiledsVal {
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
        Self { bits: 0x3430342a }
    }

    /// Read **ICM_REG_VERID** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_verid(&self) -> u32 {
        (self.bits & VeridFileds::ICM_REG_VERID_MASK) >> VeridFileds::ICM_REG_VERID_OFFSET
    }
}

impl Default for VeridFiledsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VeridFileds {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VeridFiledsVal {
        VeridFiledsVal::from_bits(self.value.read_bits())
    }
}

/// HW_CFG Register
///
/// NA
///
/// Address offset: 0x0004
/// Reset value: 0x0070d151
///
/// ## Fields
///
/// - **ICM_REG_AXI_HWCFG_AXI_NUM_SLAVES** `[24:20]` (RO)
///
///   NA
///
/// - **ICM_REG_AXI_HWCFG_AXI_NUM_MASTERS** `[16:12]` (RO)
///
///   NA
///
/// - **ICM_REG_AXI_HWCFG_LOW_POWER_INF_EN** `[8]` (RO)
///
///   NA
///
/// - **ICM_REG_AXI_HWCFG_BI_DIR_CMD_EN** `[7]` (RO)
///
///   NA
///
/// - **ICM_REG_AXI_HWCFG_REMAP_EN** `[6]` (RO)
///
///   NA
///
/// - **ICM_REG_AXI_HWCFG_DECODER_TYPE** `[5]` (RO)
///
///   NA
///
/// - **ICM_REG_AXI_HWCFG_TRUST_ZONE_EN** `[4]` (RO)
///
///   NA
///
/// - **ICM_REG_AXI_HWCFG_LOCK_EN** `[3]` (RO)
///
///   NA
///
/// - **ICM_REG_AXI_HWCFG_AXI4_SUPPORT** `[2]` (RO)
///
///   NA
///
/// - **ICM_REG_AXI_HWCFG_APB3_SUPPORT** `[1]` (RO)
///
///   NA
///
/// - **ICM_REG_AXI_HWCFG_QOS_SUPPORT** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct HwCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HwCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HwCfg {
    // ======== Field Constants ========

    /// **ICM_REG_AXI_HWCFG_AXI_NUM_SLAVES**
    ///
    /// NA
    ///
    /// Bits `[24:20]`, mask `0x01f00000`
    pub const ICM_REG_AXI_HWCFG_AXI_NUM_SLAVES_OFFSET: u32 = 20;
    pub const ICM_REG_AXI_HWCFG_AXI_NUM_SLAVES_MASK: u32 = 0x01f00000;

    /// **ICM_REG_AXI_HWCFG_AXI_NUM_MASTERS**
    ///
    /// NA
    ///
    /// Bits `[16:12]`, mask `0x0001f000`
    pub const ICM_REG_AXI_HWCFG_AXI_NUM_MASTERS_OFFSET: u32 = 12;
    pub const ICM_REG_AXI_HWCFG_AXI_NUM_MASTERS_MASK: u32 = 0x0001f000;

    /// **ICM_REG_AXI_HWCFG_LOW_POWER_INF_EN**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ICM_REG_AXI_HWCFG_LOW_POWER_INF_EN_OFFSET: u32 = 8;
    pub const ICM_REG_AXI_HWCFG_LOW_POWER_INF_EN_MASK: u32 = 0x00000100;
    pub const ICM_REG_AXI_HWCFG_LOW_POWER_INF_EN: u32 = 1 << 8;

    /// **ICM_REG_AXI_HWCFG_BI_DIR_CMD_EN**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ICM_REG_AXI_HWCFG_BI_DIR_CMD_EN_OFFSET: u32 = 7;
    pub const ICM_REG_AXI_HWCFG_BI_DIR_CMD_EN_MASK: u32 = 0x00000080;
    pub const ICM_REG_AXI_HWCFG_BI_DIR_CMD_EN: u32 = 1 << 7;

    /// **ICM_REG_AXI_HWCFG_REMAP_EN**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const ICM_REG_AXI_HWCFG_REMAP_EN_OFFSET: u32 = 6;
    pub const ICM_REG_AXI_HWCFG_REMAP_EN_MASK: u32 = 0x00000040;
    pub const ICM_REG_AXI_HWCFG_REMAP_EN: u32 = 1 << 6;

    /// **ICM_REG_AXI_HWCFG_DECODER_TYPE**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ICM_REG_AXI_HWCFG_DECODER_TYPE_OFFSET: u32 = 5;
    pub const ICM_REG_AXI_HWCFG_DECODER_TYPE_MASK: u32 = 0x00000020;
    pub const ICM_REG_AXI_HWCFG_DECODER_TYPE: u32 = 1 << 5;

    /// **ICM_REG_AXI_HWCFG_TRUST_ZONE_EN**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ICM_REG_AXI_HWCFG_TRUST_ZONE_EN_OFFSET: u32 = 4;
    pub const ICM_REG_AXI_HWCFG_TRUST_ZONE_EN_MASK: u32 = 0x00000010;
    pub const ICM_REG_AXI_HWCFG_TRUST_ZONE_EN: u32 = 1 << 4;

    /// **ICM_REG_AXI_HWCFG_LOCK_EN**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ICM_REG_AXI_HWCFG_LOCK_EN_OFFSET: u32 = 3;
    pub const ICM_REG_AXI_HWCFG_LOCK_EN_MASK: u32 = 0x00000008;
    pub const ICM_REG_AXI_HWCFG_LOCK_EN: u32 = 1 << 3;

    /// **ICM_REG_AXI_HWCFG_AXI4_SUPPORT**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ICM_REG_AXI_HWCFG_AXI4_SUPPORT_OFFSET: u32 = 2;
    pub const ICM_REG_AXI_HWCFG_AXI4_SUPPORT_MASK: u32 = 0x00000004;
    pub const ICM_REG_AXI_HWCFG_AXI4_SUPPORT: u32 = 1 << 2;

    /// **ICM_REG_AXI_HWCFG_APB3_SUPPORT**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ICM_REG_AXI_HWCFG_APB3_SUPPORT_OFFSET: u32 = 1;
    pub const ICM_REG_AXI_HWCFG_APB3_SUPPORT_MASK: u32 = 0x00000002;
    pub const ICM_REG_AXI_HWCFG_APB3_SUPPORT: u32 = 1 << 1;

    /// **ICM_REG_AXI_HWCFG_QOS_SUPPORT**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ICM_REG_AXI_HWCFG_QOS_SUPPORT_OFFSET: u32 = 0;
    pub const ICM_REG_AXI_HWCFG_QOS_SUPPORT_MASK: u32 = 0x00000001;
    pub const ICM_REG_AXI_HWCFG_QOS_SUPPORT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ICM_REG_AXI_HWCFG_AXI_NUM_SLAVES** `[24:20]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_axi_num_slaves(&self) -> u32 {
        (self.value.read_bits() & Self::ICM_REG_AXI_HWCFG_AXI_NUM_SLAVES_MASK)
            >> Self::ICM_REG_AXI_HWCFG_AXI_NUM_SLAVES_OFFSET
    }

    /// **ICM_REG_AXI_HWCFG_AXI_NUM_MASTERS** `[16:12]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_axi_num_masters(&self) -> u32 {
        (self.value.read_bits() & Self::ICM_REG_AXI_HWCFG_AXI_NUM_MASTERS_MASK)
            >> Self::ICM_REG_AXI_HWCFG_AXI_NUM_MASTERS_OFFSET
    }

    /// **ICM_REG_AXI_HWCFG_LOW_POWER_INF_EN** `[8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_low_power_inf_en(&self) -> bool {
        (self.value.read_bits() & Self::ICM_REG_AXI_HWCFG_LOW_POWER_INF_EN_MASK) != 0
    }

    /// **ICM_REG_AXI_HWCFG_BI_DIR_CMD_EN** `[7]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_bi_dir_cmd_en(&self) -> bool {
        (self.value.read_bits() & Self::ICM_REG_AXI_HWCFG_BI_DIR_CMD_EN_MASK) != 0
    }

    /// **ICM_REG_AXI_HWCFG_REMAP_EN** `[6]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_remap_en(&self) -> bool {
        (self.value.read_bits() & Self::ICM_REG_AXI_HWCFG_REMAP_EN_MASK) != 0
    }

    /// **ICM_REG_AXI_HWCFG_DECODER_TYPE** `[5]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_decoder_type(&self) -> bool {
        (self.value.read_bits() & Self::ICM_REG_AXI_HWCFG_DECODER_TYPE_MASK) != 0
    }

    /// **ICM_REG_AXI_HWCFG_TRUST_ZONE_EN** `[4]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_trust_zone_en(&self) -> bool {
        (self.value.read_bits() & Self::ICM_REG_AXI_HWCFG_TRUST_ZONE_EN_MASK) != 0
    }

    /// **ICM_REG_AXI_HWCFG_LOCK_EN** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_lock_en(&self) -> bool {
        (self.value.read_bits() & Self::ICM_REG_AXI_HWCFG_LOCK_EN_MASK) != 0
    }

    /// **ICM_REG_AXI_HWCFG_AXI4_SUPPORT** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_axi4_support(&self) -> bool {
        (self.value.read_bits() & Self::ICM_REG_AXI_HWCFG_AXI4_SUPPORT_MASK) != 0
    }

    /// **ICM_REG_AXI_HWCFG_APB3_SUPPORT** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_apb3_support(&self) -> bool {
        (self.value.read_bits() & Self::ICM_REG_AXI_HWCFG_APB3_SUPPORT_MASK) != 0
    }

    /// **ICM_REG_AXI_HWCFG_QOS_SUPPORT** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_qos_support(&self) -> bool {
        (self.value.read_bits() & Self::ICM_REG_AXI_HWCFG_QOS_SUPPORT_MASK) != 0
    }
}

/// HW_CFG value type for batch read/modify/write operations
///
/// Use `hwcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hwcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HwCfgVal {
    bits: u32,
}

impl HwCfgVal {
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
        Self { bits: 0x0070d151 }
    }

    /// Read **ICM_REG_AXI_HWCFG_AXI_NUM_SLAVES** `[24:20]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_hwcfg_axi_num_slaves(&self) -> u32 {
        (self.bits & HwCfg::ICM_REG_AXI_HWCFG_AXI_NUM_SLAVES_MASK)
            >> HwCfg::ICM_REG_AXI_HWCFG_AXI_NUM_SLAVES_OFFSET
    }

    /// Read **ICM_REG_AXI_HWCFG_AXI_NUM_MASTERS** `[16:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_hwcfg_axi_num_masters(&self) -> u32 {
        (self.bits & HwCfg::ICM_REG_AXI_HWCFG_AXI_NUM_MASTERS_MASK)
            >> HwCfg::ICM_REG_AXI_HWCFG_AXI_NUM_MASTERS_OFFSET
    }

    /// Read **ICM_REG_AXI_HWCFG_LOW_POWER_INF_EN** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_hwcfg_low_power_inf_en(&self) -> bool {
        (self.bits & HwCfg::ICM_REG_AXI_HWCFG_LOW_POWER_INF_EN_MASK) != 0
    }

    /// Read **ICM_REG_AXI_HWCFG_BI_DIR_CMD_EN** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_hwcfg_bi_dir_cmd_en(&self) -> bool {
        (self.bits & HwCfg::ICM_REG_AXI_HWCFG_BI_DIR_CMD_EN_MASK) != 0
    }

    /// Read **ICM_REG_AXI_HWCFG_REMAP_EN** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_hwcfg_remap_en(&self) -> bool {
        (self.bits & HwCfg::ICM_REG_AXI_HWCFG_REMAP_EN_MASK) != 0
    }

    /// Read **ICM_REG_AXI_HWCFG_DECODER_TYPE** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_hwcfg_decoder_type(&self) -> bool {
        (self.bits & HwCfg::ICM_REG_AXI_HWCFG_DECODER_TYPE_MASK) != 0
    }

    /// Read **ICM_REG_AXI_HWCFG_TRUST_ZONE_EN** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_hwcfg_trust_zone_en(&self) -> bool {
        (self.bits & HwCfg::ICM_REG_AXI_HWCFG_TRUST_ZONE_EN_MASK) != 0
    }

    /// Read **ICM_REG_AXI_HWCFG_LOCK_EN** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_hwcfg_lock_en(&self) -> bool {
        (self.bits & HwCfg::ICM_REG_AXI_HWCFG_LOCK_EN_MASK) != 0
    }

    /// Read **ICM_REG_AXI_HWCFG_AXI4_SUPPORT** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_hwcfg_axi4_support(&self) -> bool {
        (self.bits & HwCfg::ICM_REG_AXI_HWCFG_AXI4_SUPPORT_MASK) != 0
    }

    /// Read **ICM_REG_AXI_HWCFG_APB3_SUPPORT** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_hwcfg_apb3_support(&self) -> bool {
        (self.bits & HwCfg::ICM_REG_AXI_HWCFG_APB3_SUPPORT_MASK) != 0
    }

    /// Read **ICM_REG_AXI_HWCFG_QOS_SUPPORT** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_hwcfg_qos_support(&self) -> bool {
        (self.bits & HwCfg::ICM_REG_AXI_HWCFG_QOS_SUPPORT_MASK) != 0
    }
}

impl Default for HwCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl HwCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HwCfgVal {
        HwCfgVal::from_bits(self.value.read_bits())
    }
}

/// CMD Register
///
/// NA
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **ICM_REG_AXI_CMD_EN** `[31]` (RW)
///
///   NA
///
/// - **ICM_REG_AXI_RD_WR_CMD** `[30]` (RW)
///
///   NA
///
/// - **ICM_REG_AXI_SOFT_RESET_BIT** `[29]` (RW)
///
///   NA
///
/// - **ICM_REG_AXI_ERR_BIT** `[28]` (RO)
///
///   NA
///
/// - **ICM_REG_AXI_MASTER_PORT** `[11:8]` (RW)
///
///   NA
///
/// - **ICM_REG_RD_WR_CHAN** `[7]` (RW)
///
///   NA
///
/// - **ICM_REG_AXI_CMD** `[2:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Cmd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cmd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cmd {
    // ======== Field Constants ========

    /// **ICM_REG_AXI_CMD_EN**
    ///
    /// NA
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const ICM_REG_AXI_CMD_EN_OFFSET: u32 = 31;
    pub const ICM_REG_AXI_CMD_EN_MASK: u32 = 0x80000000;
    pub const ICM_REG_AXI_CMD_EN: u32 = 1 << 31;

    /// **ICM_REG_AXI_RD_WR_CMD**
    ///
    /// NA
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const ICM_REG_AXI_RD_WR_CMD_OFFSET: u32 = 30;
    pub const ICM_REG_AXI_RD_WR_CMD_MASK: u32 = 0x40000000;
    pub const ICM_REG_AXI_RD_WR_CMD: u32 = 1 << 30;

    /// **ICM_REG_AXI_SOFT_RESET_BIT**
    ///
    /// NA
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const ICM_REG_AXI_SOFT_RESET_BIT_OFFSET: u32 = 29;
    pub const ICM_REG_AXI_SOFT_RESET_BIT_MASK: u32 = 0x20000000;
    pub const ICM_REG_AXI_SOFT_RESET_BIT: u32 = 1 << 29;

    /// **ICM_REG_AXI_ERR_BIT**
    ///
    /// NA
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const ICM_REG_AXI_ERR_BIT_OFFSET: u32 = 28;
    pub const ICM_REG_AXI_ERR_BIT_MASK: u32 = 0x10000000;
    pub const ICM_REG_AXI_ERR_BIT: u32 = 1 << 28;

    /// **ICM_REG_AXI_MASTER_PORT**
    ///
    /// NA
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const ICM_REG_AXI_MASTER_PORT_OFFSET: u32 = 8;
    pub const ICM_REG_AXI_MASTER_PORT_MASK: u32 = 0x00000f00;

    /// **ICM_REG_RD_WR_CHAN**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ICM_REG_RD_WR_CHAN_OFFSET: u32 = 7;
    pub const ICM_REG_RD_WR_CHAN_MASK: u32 = 0x00000080;
    pub const ICM_REG_RD_WR_CHAN: u32 = 1 << 7;

    /// **ICM_REG_AXI_CMD**
    ///
    /// NA
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const ICM_REG_AXI_CMD_OFFSET: u32 = 0;
    pub const ICM_REG_AXI_CMD_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **ICM_REG_AXI_CMD_EN** `[31]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_cmd_en(&self) -> bool {
        (self.value.read_bits() & Self::ICM_REG_AXI_CMD_EN_MASK) != 0
    }

    /// Set ICM_REG_AXI_CMD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_icm_reg_axi_cmd_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ICM_REG_AXI_CMD_EN_MASK);
    }

    /// Clear ICM_REG_AXI_CMD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_icm_reg_axi_cmd_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ICM_REG_AXI_CMD_EN_MASK);
    }

    /// **ICM_REG_AXI_RD_WR_CMD** `[30]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_rd_wr_cmd(&self) -> bool {
        (self.value.read_bits() & Self::ICM_REG_AXI_RD_WR_CMD_MASK) != 0
    }

    /// Set ICM_REG_AXI_RD_WR_CMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_icm_reg_axi_rd_wr_cmd(&self) {
        self.value
            .modify_bits(|r| r | Self::ICM_REG_AXI_RD_WR_CMD_MASK);
    }

    /// Clear ICM_REG_AXI_RD_WR_CMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_icm_reg_axi_rd_wr_cmd(&self) {
        self.value
            .modify_bits(|r| r & !Self::ICM_REG_AXI_RD_WR_CMD_MASK);
    }

    /// **ICM_REG_AXI_SOFT_RESET_BIT** `[29]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_soft_reset_bit(&self) -> bool {
        (self.value.read_bits() & Self::ICM_REG_AXI_SOFT_RESET_BIT_MASK) != 0
    }

    /// Set ICM_REG_AXI_SOFT_RESET_BIT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_icm_reg_axi_soft_reset_bit(&self) {
        self.value
            .modify_bits(|r| r | Self::ICM_REG_AXI_SOFT_RESET_BIT_MASK);
    }

    /// Clear ICM_REG_AXI_SOFT_RESET_BIT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_icm_reg_axi_soft_reset_bit(&self) {
        self.value
            .modify_bits(|r| r & !Self::ICM_REG_AXI_SOFT_RESET_BIT_MASK);
    }

    /// **ICM_REG_AXI_ERR_BIT** `[28]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_err_bit(&self) -> bool {
        (self.value.read_bits() & Self::ICM_REG_AXI_ERR_BIT_MASK) != 0
    }

    /// **ICM_REG_AXI_MASTER_PORT** `[11:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_master_port(&self) -> u32 {
        (self.value.read_bits() & Self::ICM_REG_AXI_MASTER_PORT_MASK)
            >> Self::ICM_REG_AXI_MASTER_PORT_OFFSET
    }

    /// Write **ICM_REG_AXI_MASTER_PORT** `[11:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_icm_reg_axi_master_port(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ICM_REG_AXI_MASTER_PORT_MASK)
                | ((value << Self::ICM_REG_AXI_MASTER_PORT_OFFSET)
                    & Self::ICM_REG_AXI_MASTER_PORT_MASK)
        });
    }

    /// **ICM_REG_RD_WR_CHAN** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_rd_wr_chan(&self) -> bool {
        (self.value.read_bits() & Self::ICM_REG_RD_WR_CHAN_MASK) != 0
    }

    /// Set ICM_REG_RD_WR_CHAN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_icm_reg_rd_wr_chan(&self) {
        self.value
            .modify_bits(|r| r | Self::ICM_REG_RD_WR_CHAN_MASK);
    }

    /// Clear ICM_REG_RD_WR_CHAN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_icm_reg_rd_wr_chan(&self) {
        self.value
            .modify_bits(|r| r & !Self::ICM_REG_RD_WR_CHAN_MASK);
    }

    /// **ICM_REG_AXI_CMD** `[2:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_axi_cmd(&self) -> u32 {
        (self.value.read_bits() & Self::ICM_REG_AXI_CMD_MASK) >> Self::ICM_REG_AXI_CMD_OFFSET
    }

    /// Write **ICM_REG_AXI_CMD** `[2:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_icm_reg_axi_cmd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ICM_REG_AXI_CMD_MASK)
                | ((value << Self::ICM_REG_AXI_CMD_OFFSET) & Self::ICM_REG_AXI_CMD_MASK)
        });
    }
}

/// CMD value type for batch read/modify/write operations
///
/// Use `cmd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cmd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CmdVal {
    bits: u32,
}

impl CmdVal {
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

    /// Read **ICM_REG_AXI_CMD_EN** `[31:31]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_cmd_en(&self) -> bool {
        (self.bits & Cmd::ICM_REG_AXI_CMD_EN_MASK) != 0
    }

    /// Read **ICM_REG_AXI_RD_WR_CMD** `[30:30]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_rd_wr_cmd(&self) -> bool {
        (self.bits & Cmd::ICM_REG_AXI_RD_WR_CMD_MASK) != 0
    }

    /// Read **ICM_REG_AXI_SOFT_RESET_BIT** `[29:29]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_soft_reset_bit(&self) -> bool {
        (self.bits & Cmd::ICM_REG_AXI_SOFT_RESET_BIT_MASK) != 0
    }

    /// Read **ICM_REG_AXI_ERR_BIT** `[28:28]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_err_bit(&self) -> bool {
        (self.bits & Cmd::ICM_REG_AXI_ERR_BIT_MASK) != 0
    }

    /// Read **ICM_REG_AXI_MASTER_PORT** `[11:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_master_port(&self) -> u32 {
        (self.bits & Cmd::ICM_REG_AXI_MASTER_PORT_MASK) >> Cmd::ICM_REG_AXI_MASTER_PORT_OFFSET
    }

    /// Read **ICM_REG_RD_WR_CHAN** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_rd_wr_chan(&self) -> bool {
        (self.bits & Cmd::ICM_REG_RD_WR_CHAN_MASK) != 0
    }

    /// Read **ICM_REG_AXI_CMD** `[2:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_axi_cmd(&self) -> u32 {
        (self.bits & Cmd::ICM_REG_AXI_CMD_MASK) >> Cmd::ICM_REG_AXI_CMD_OFFSET
    }

    /// Set **ICM_REG_AXI_CMD_EN** `[31:31]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_icm_reg_axi_cmd_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::ICM_REG_AXI_CMD_EN_MASK
            } else {
                self.bits & !Cmd::ICM_REG_AXI_CMD_EN_MASK
            },
        }
    }

    /// Set **ICM_REG_AXI_RD_WR_CMD** `[30:30]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_icm_reg_axi_rd_wr_cmd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::ICM_REG_AXI_RD_WR_CMD_MASK
            } else {
                self.bits & !Cmd::ICM_REG_AXI_RD_WR_CMD_MASK
            },
        }
    }

    /// Set **ICM_REG_AXI_SOFT_RESET_BIT** `[29:29]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_icm_reg_axi_soft_reset_bit(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::ICM_REG_AXI_SOFT_RESET_BIT_MASK
            } else {
                self.bits & !Cmd::ICM_REG_AXI_SOFT_RESET_BIT_MASK
            },
        }
    }

    /// Set **ICM_REG_AXI_MASTER_PORT** `[11:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_icm_reg_axi_master_port(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cmd::ICM_REG_AXI_MASTER_PORT_MASK)
                | ((value << Cmd::ICM_REG_AXI_MASTER_PORT_OFFSET)
                    & Cmd::ICM_REG_AXI_MASTER_PORT_MASK),
        }
    }

    /// Set **ICM_REG_RD_WR_CHAN** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_icm_reg_rd_wr_chan(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::ICM_REG_RD_WR_CHAN_MASK
            } else {
                self.bits & !Cmd::ICM_REG_RD_WR_CHAN_MASK
            },
        }
    }

    /// Set **ICM_REG_AXI_CMD** `[2:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_icm_reg_axi_cmd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cmd::ICM_REG_AXI_CMD_MASK)
                | ((value << Cmd::ICM_REG_AXI_CMD_OFFSET) & Cmd::ICM_REG_AXI_CMD_MASK),
        }
    }
}

impl Default for CmdVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Cmd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CmdVal {
        CmdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CmdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CmdVal) -> CmdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA Register
///
/// NA
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **ICM_REG_DATA** `[31:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Data {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Data {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Data {
    // ======== Field Constants ========

    /// **ICM_REG_DATA**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ICM_REG_DATA_OFFSET: u32 = 0;
    pub const ICM_REG_DATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ICM_REG_DATA** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn icm_reg_data(&self) -> u32 {
        (self.value.read_bits() & Self::ICM_REG_DATA_MASK) >> Self::ICM_REG_DATA_OFFSET
    }

    /// Write **ICM_REG_DATA** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_icm_reg_data(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ICM_REG_DATA_MASK)
                | ((value << Self::ICM_REG_DATA_OFFSET) & Self::ICM_REG_DATA_MASK)
        });
    }
}

/// DATA value type for batch read/modify/write operations
///
/// Use `data.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `data.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DataVal {
    bits: u32,
}

impl DataVal {
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

    /// Read **ICM_REG_DATA** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn icm_reg_data(&self) -> u32 {
        (self.bits & Data::ICM_REG_DATA_MASK) >> Data::ICM_REG_DATA_OFFSET
    }

    /// Set **ICM_REG_DATA** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_icm_reg_data(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Data::ICM_REG_DATA_MASK)
                | ((value << Data::ICM_REG_DATA_OFFSET) & Data::ICM_REG_DATA_MASK),
        }
    }
}

impl Default for DataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Data {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DataVal {
        DataVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DataVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DataVal) -> DataVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// NA
    pub verid_fileds: VeridFileds,
    /// 0x0004
    /// NA
    pub hw_cfg: HwCfg,
    /// 0x0008
    /// NA
    pub cmd: Cmd,
    /// 0x000c
    /// NA
    pub data: Data,
}
