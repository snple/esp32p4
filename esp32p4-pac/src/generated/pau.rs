//! PAU
//! PAU Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x60093000;

/// REGDMA_CONF Register
///
/// Peri backup control register
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **SEL_MAC** `[9]` (RW)
///
///   mac hw/sw select
///
/// - **TO_MEM_MAC** `[8]` (RW)
///
///   mac sw backup direction(reg to mem / mem to reg)
///
/// - **START_MAC** `[7]` (WO)
///
///   mac sw backup start signal
///
/// - **LINK_SEL** `[6:5]` (RW)
///
///   Link select
///
/// - **TO_MEM** `[4]` (RW)
///
///   backup direction(reg to mem / mem to reg)
///
/// - **START** `[3]` (WO)
///
///   backup start signal
///
/// - **FLOW_ERR** `[2:0]` (RO)
///
///   backup error type
#[repr(transparent)]
pub struct RegdmaConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegdmaConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegdmaConf {
    // ======== Field Constants ========

    /// **SEL_MAC**
    ///
    /// mac hw/sw select
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SEL_MAC_OFFSET: u32 = 9;
    pub const SEL_MAC_MASK: u32 = 0x00000200;
    pub const SEL_MAC: u32 = 1 << 9;

    /// **TO_MEM_MAC**
    ///
    /// mac sw backup direction(reg to mem / mem to reg)
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TO_MEM_MAC_OFFSET: u32 = 8;
    pub const TO_MEM_MAC_MASK: u32 = 0x00000100;
    pub const TO_MEM_MAC: u32 = 1 << 8;

    /// **START_MAC**
    ///
    /// mac sw backup start signal
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const START_MAC_OFFSET: u32 = 7;
    pub const START_MAC_MASK: u32 = 0x00000080;
    pub const START_MAC: u32 = 1 << 7;

    /// **LINK_SEL**
    ///
    /// Link select
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const LINK_SEL_OFFSET: u32 = 5;
    pub const LINK_SEL_MASK: u32 = 0x00000060;

    /// **TO_MEM**
    ///
    /// backup direction(reg to mem / mem to reg)
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TO_MEM_OFFSET: u32 = 4;
    pub const TO_MEM_MASK: u32 = 0x00000010;
    pub const TO_MEM: u32 = 1 << 4;

    /// **START**
    ///
    /// backup start signal
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const START_OFFSET: u32 = 3;
    pub const START_MASK: u32 = 0x00000008;
    pub const START: u32 = 1 << 3;

    /// **FLOW_ERR**
    ///
    /// backup error type
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const FLOW_ERR_OFFSET: u32 = 0;
    pub const FLOW_ERR_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **SEL_MAC** `[9]` (RW)
    ///
    /// mac hw/sw select
    #[inline(always)]
    pub fn sel_mac(&self) -> bool {
        (self.value.read_bits() & Self::SEL_MAC_MASK) != 0
    }

    /// Set SEL_MAC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sel_mac(&self) {
        self.value.modify_bits(|r| r | Self::SEL_MAC_MASK);
    }

    /// Clear SEL_MAC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sel_mac(&self) {
        self.value.modify_bits(|r| r & !Self::SEL_MAC_MASK);
    }

    /// **TO_MEM_MAC** `[8]` (RW)
    ///
    /// mac sw backup direction(reg to mem / mem to reg)
    #[inline(always)]
    pub fn to_mem_mac(&self) -> bool {
        (self.value.read_bits() & Self::TO_MEM_MAC_MASK) != 0
    }

    /// Set TO_MEM_MAC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_to_mem_mac(&self) {
        self.value.modify_bits(|r| r | Self::TO_MEM_MAC_MASK);
    }

    /// Clear TO_MEM_MAC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_to_mem_mac(&self) {
        self.value.modify_bits(|r| r & !Self::TO_MEM_MAC_MASK);
    }

    /// Trigger **START_MAC** (WO)
    ///
    /// mac sw backup start signal
    #[inline(always)]
    pub fn trigger_start_mac(&self) {
        self.value.modify_bits(|r| r | Self::START_MAC_MASK);
    }

    /// **LINK_SEL** `[6:5]` (RW)
    ///
    /// Link select
    #[inline(always)]
    pub fn link_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LINK_SEL_MASK) >> Self::LINK_SEL_OFFSET
    }

    /// Write **LINK_SEL** `[6:5]` (RW)
    ///
    /// Link select
    #[inline(always)]
    pub fn write_link_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LINK_SEL_MASK) | ((value << Self::LINK_SEL_OFFSET) & Self::LINK_SEL_MASK)
        });
    }

    /// **TO_MEM** `[4]` (RW)
    ///
    /// backup direction(reg to mem / mem to reg)
    #[inline(always)]
    pub fn to_mem(&self) -> bool {
        (self.value.read_bits() & Self::TO_MEM_MASK) != 0
    }

    /// Set TO_MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_to_mem(&self) {
        self.value.modify_bits(|r| r | Self::TO_MEM_MASK);
    }

    /// Clear TO_MEM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_to_mem(&self) {
        self.value.modify_bits(|r| r & !Self::TO_MEM_MASK);
    }

    /// Trigger **START** (WO)
    ///
    /// backup start signal
    #[inline(always)]
    pub fn trigger_start(&self) {
        self.value.modify_bits(|r| r | Self::START_MASK);
    }

    /// **FLOW_ERR** `[2:0]` (RO)
    ///
    /// backup error type
    #[inline(always)]
    pub fn flow_err(&self) -> u32 {
        (self.value.read_bits() & Self::FLOW_ERR_MASK) >> Self::FLOW_ERR_OFFSET
    }
}

/// REGDMA_CONF value type for batch read/modify/write operations
///
/// Use `regdmaconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regdmaconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegdmaConfVal {
    bits: u32,
}

impl RegdmaConfVal {
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

    /// Read **SEL_MAC** `[9:9]`
    ///
    /// mac hw/sw select
    #[inline(always)]
    pub const fn sel_mac(&self) -> bool {
        (self.bits & RegdmaConf::SEL_MAC_MASK) != 0
    }

    /// Read **TO_MEM_MAC** `[8:8]`
    ///
    /// mac sw backup direction(reg to mem / mem to reg)
    #[inline(always)]
    pub const fn to_mem_mac(&self) -> bool {
        (self.bits & RegdmaConf::TO_MEM_MAC_MASK) != 0
    }

    /// Read **LINK_SEL** `[6:5]`
    ///
    /// Link select
    #[inline(always)]
    pub const fn link_sel(&self) -> u32 {
        (self.bits & RegdmaConf::LINK_SEL_MASK) >> RegdmaConf::LINK_SEL_OFFSET
    }

    /// Read **TO_MEM** `[4:4]`
    ///
    /// backup direction(reg to mem / mem to reg)
    #[inline(always)]
    pub const fn to_mem(&self) -> bool {
        (self.bits & RegdmaConf::TO_MEM_MASK) != 0
    }

    /// Read **FLOW_ERR** `[2:0]`
    ///
    /// backup error type
    #[inline(always)]
    pub const fn flow_err(&self) -> u32 {
        (self.bits & RegdmaConf::FLOW_ERR_MASK) >> RegdmaConf::FLOW_ERR_OFFSET
    }

    /// Set **SEL_MAC** `[9:9]`
    ///
    /// mac hw/sw select
    #[inline(always)]
    pub const fn with_sel_mac(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RegdmaConf::SEL_MAC_MASK
            } else {
                self.bits & !RegdmaConf::SEL_MAC_MASK
            },
        }
    }

    /// Set **TO_MEM_MAC** `[8:8]`
    ///
    /// mac sw backup direction(reg to mem / mem to reg)
    #[inline(always)]
    pub const fn with_to_mem_mac(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RegdmaConf::TO_MEM_MAC_MASK
            } else {
                self.bits & !RegdmaConf::TO_MEM_MAC_MASK
            },
        }
    }

    /// Set **START_MAC** `[7:7]`
    ///
    /// mac sw backup start signal
    #[inline(always)]
    pub const fn with_start_mac(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RegdmaConf::START_MAC_MASK
            } else {
                self.bits & !RegdmaConf::START_MAC_MASK
            },
        }
    }

    /// Set **LINK_SEL** `[6:5]`
    ///
    /// Link select
    #[inline(always)]
    pub const fn with_link_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegdmaConf::LINK_SEL_MASK)
                | ((value << RegdmaConf::LINK_SEL_OFFSET) & RegdmaConf::LINK_SEL_MASK),
        }
    }

    /// Set **TO_MEM** `[4:4]`
    ///
    /// backup direction(reg to mem / mem to reg)
    #[inline(always)]
    pub const fn with_to_mem(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RegdmaConf::TO_MEM_MASK
            } else {
                self.bits & !RegdmaConf::TO_MEM_MASK
            },
        }
    }

    /// Set **START** `[3:3]`
    ///
    /// backup start signal
    #[inline(always)]
    pub const fn with_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RegdmaConf::START_MASK
            } else {
                self.bits & !RegdmaConf::START_MASK
            },
        }
    }
}

impl Default for RegdmaConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegdmaConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegdmaConfVal {
        RegdmaConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegdmaConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegdmaConfVal) -> RegdmaConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REGDMA_CLK_CONF Register
///
/// Clock control register
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **CLK_EN** `[0]` (RW)
///
///   clock enable
#[repr(transparent)]
pub struct RegdmaClkConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegdmaClkConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegdmaClkConf {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// clock enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_EN_OFFSET: u32 = 0;
    pub const CLK_EN_MASK: u32 = 0x00000001;
    pub const CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[0]` (RW)
    ///
    /// clock enable
    #[inline(always)]
    pub fn clk_en(&self) -> bool {
        (self.value.read_bits() & Self::CLK_EN_MASK) != 0
    }

    /// Set CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::CLK_EN_MASK);
    }

    /// Clear CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::CLK_EN_MASK);
    }
}

/// REGDMA_CLK_CONF value type for batch read/modify/write operations
///
/// Use `regdmaclkconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regdmaclkconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegdmaClkConfVal {
    bits: u32,
}

impl RegdmaClkConfVal {
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

    /// Read **CLK_EN** `[0:0]`
    ///
    /// clock enable
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & RegdmaClkConf::CLK_EN_MASK) != 0
    }

    /// Set **CLK_EN** `[0:0]`
    ///
    /// clock enable
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RegdmaClkConf::CLK_EN_MASK
            } else {
                self.bits & !RegdmaClkConf::CLK_EN_MASK
            },
        }
    }
}

impl Default for RegdmaClkConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegdmaClkConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegdmaClkConfVal {
        RegdmaClkConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegdmaClkConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegdmaClkConfVal) -> RegdmaClkConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REGDMA_ETM_CTRL Register
///
/// ETM start ctrl reg
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **ETM_START_3** `[3]` (WO)
///
///   etm_start_3 reg
///
/// - **ETM_START_2** `[2]` (WO)
///
///   etm_start_2 reg
///
/// - **ETM_START_1** `[1]` (WO)
///
///   etm_start_1 reg
///
/// - **ETM_START_0** `[0]` (WO)
///
///   etm_start_0 reg
#[repr(transparent)]
pub struct RegdmaEtmCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegdmaEtmCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegdmaEtmCtrl {
    // ======== Field Constants ========

    /// **ETM_START_3**
    ///
    /// etm_start_3 reg
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ETM_START_3_OFFSET: u32 = 3;
    pub const ETM_START_3_MASK: u32 = 0x00000008;
    pub const ETM_START_3: u32 = 1 << 3;

    /// **ETM_START_2**
    ///
    /// etm_start_2 reg
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ETM_START_2_OFFSET: u32 = 2;
    pub const ETM_START_2_MASK: u32 = 0x00000004;
    pub const ETM_START_2: u32 = 1 << 2;

    /// **ETM_START_1**
    ///
    /// etm_start_1 reg
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ETM_START_1_OFFSET: u32 = 1;
    pub const ETM_START_1_MASK: u32 = 0x00000002;
    pub const ETM_START_1: u32 = 1 << 1;

    /// **ETM_START_0**
    ///
    /// etm_start_0 reg
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_START_0_OFFSET: u32 = 0;
    pub const ETM_START_0_MASK: u32 = 0x00000001;
    pub const ETM_START_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **ETM_START_3** (WO)
    ///
    /// etm_start_3 reg
    #[inline(always)]
    pub fn trigger_etm_start_3(&self) {
        self.value.write_bits(Self::ETM_START_3_MASK);
    }

    /// Trigger **ETM_START_2** (WO)
    ///
    /// etm_start_2 reg
    #[inline(always)]
    pub fn trigger_etm_start_2(&self) {
        self.value.write_bits(Self::ETM_START_2_MASK);
    }

    /// Trigger **ETM_START_1** (WO)
    ///
    /// etm_start_1 reg
    #[inline(always)]
    pub fn trigger_etm_start_1(&self) {
        self.value.write_bits(Self::ETM_START_1_MASK);
    }

    /// Trigger **ETM_START_0** (WO)
    ///
    /// etm_start_0 reg
    #[inline(always)]
    pub fn trigger_etm_start_0(&self) {
        self.value.write_bits(Self::ETM_START_0_MASK);
    }
}

/// REGDMA_ETM_CTRL value type for batch read/modify/write operations
///
/// Use `regdmaetmctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regdmaetmctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegdmaEtmCtrlVal {
    bits: u32,
}

impl RegdmaEtmCtrlVal {
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

    /// Set **ETM_START_3** `[3:3]`
    ///
    /// etm_start_3 reg
    #[inline(always)]
    pub const fn with_etm_start_3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RegdmaEtmCtrl::ETM_START_3_MASK
            } else {
                self.bits & !RegdmaEtmCtrl::ETM_START_3_MASK
            },
        }
    }

    /// Set **ETM_START_2** `[2:2]`
    ///
    /// etm_start_2 reg
    #[inline(always)]
    pub const fn with_etm_start_2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RegdmaEtmCtrl::ETM_START_2_MASK
            } else {
                self.bits & !RegdmaEtmCtrl::ETM_START_2_MASK
            },
        }
    }

    /// Set **ETM_START_1** `[1:1]`
    ///
    /// etm_start_1 reg
    #[inline(always)]
    pub const fn with_etm_start_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RegdmaEtmCtrl::ETM_START_1_MASK
            } else {
                self.bits & !RegdmaEtmCtrl::ETM_START_1_MASK
            },
        }
    }

    /// Set **ETM_START_0** `[0:0]`
    ///
    /// etm_start_0 reg
    #[inline(always)]
    pub const fn with_etm_start_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RegdmaEtmCtrl::ETM_START_0_MASK
            } else {
                self.bits & !RegdmaEtmCtrl::ETM_START_0_MASK
            },
        }
    }
}

impl Default for RegdmaEtmCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegdmaEtmCtrl {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegdmaEtmCtrlVal) {
        self.value.write_bits(val.bits())
    }
}

/// REGDMA_LINK_0_ADDR Register
///
/// link_0_addr
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **LINK_ADDR_0** `[31:0]` (RW)
///
///   link_0_addr reg
#[repr(transparent)]
pub struct RegdmaLink0Addr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegdmaLink0Addr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegdmaLink0Addr {
    // ======== Field Constants ========

    /// **LINK_ADDR_0**
    ///
    /// link_0_addr reg
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LINK_ADDR_0_OFFSET: u32 = 0;
    pub const LINK_ADDR_0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LINK_ADDR_0** `[31:0]` (RW)
    ///
    /// link_0_addr reg
    #[inline(always)]
    pub fn link_addr_0(&self) -> u32 {
        (self.value.read_bits() & Self::LINK_ADDR_0_MASK) >> Self::LINK_ADDR_0_OFFSET
    }

    /// Write **LINK_ADDR_0** `[31:0]` (RW)
    ///
    /// link_0_addr reg
    #[inline(always)]
    pub fn write_link_addr_0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LINK_ADDR_0_MASK)
                | ((value << Self::LINK_ADDR_0_OFFSET) & Self::LINK_ADDR_0_MASK)
        });
    }
}

/// REGDMA_LINK_0_ADDR value type for batch read/modify/write operations
///
/// Use `regdmalink0addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regdmalink0addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegdmaLink0AddrVal {
    bits: u32,
}

impl RegdmaLink0AddrVal {
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

    /// Read **LINK_ADDR_0** `[31:0]`
    ///
    /// link_0_addr reg
    #[inline(always)]
    pub const fn link_addr_0(&self) -> u32 {
        (self.bits & RegdmaLink0Addr::LINK_ADDR_0_MASK) >> RegdmaLink0Addr::LINK_ADDR_0_OFFSET
    }

    /// Set **LINK_ADDR_0** `[31:0]`
    ///
    /// link_0_addr reg
    #[inline(always)]
    pub const fn with_link_addr_0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegdmaLink0Addr::LINK_ADDR_0_MASK)
                | ((value << RegdmaLink0Addr::LINK_ADDR_0_OFFSET)
                    & RegdmaLink0Addr::LINK_ADDR_0_MASK),
        }
    }
}

impl Default for RegdmaLink0AddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegdmaLink0Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegdmaLink0AddrVal {
        RegdmaLink0AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegdmaLink0AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegdmaLink0AddrVal) -> RegdmaLink0AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REGDMA_LINK_1_ADDR Register
///
/// Link_1_addr
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **LINK_ADDR_1** `[31:0]` (RW)
///
///   Link_1_addr reg
#[repr(transparent)]
pub struct RegdmaLink1Addr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegdmaLink1Addr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegdmaLink1Addr {
    // ======== Field Constants ========

    /// **LINK_ADDR_1**
    ///
    /// Link_1_addr reg
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LINK_ADDR_1_OFFSET: u32 = 0;
    pub const LINK_ADDR_1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LINK_ADDR_1** `[31:0]` (RW)
    ///
    /// Link_1_addr reg
    #[inline(always)]
    pub fn link_addr_1(&self) -> u32 {
        (self.value.read_bits() & Self::LINK_ADDR_1_MASK) >> Self::LINK_ADDR_1_OFFSET
    }

    /// Write **LINK_ADDR_1** `[31:0]` (RW)
    ///
    /// Link_1_addr reg
    #[inline(always)]
    pub fn write_link_addr_1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LINK_ADDR_1_MASK)
                | ((value << Self::LINK_ADDR_1_OFFSET) & Self::LINK_ADDR_1_MASK)
        });
    }
}

/// REGDMA_LINK_1_ADDR value type for batch read/modify/write operations
///
/// Use `regdmalink1addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regdmalink1addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegdmaLink1AddrVal {
    bits: u32,
}

impl RegdmaLink1AddrVal {
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

    /// Read **LINK_ADDR_1** `[31:0]`
    ///
    /// Link_1_addr reg
    #[inline(always)]
    pub const fn link_addr_1(&self) -> u32 {
        (self.bits & RegdmaLink1Addr::LINK_ADDR_1_MASK) >> RegdmaLink1Addr::LINK_ADDR_1_OFFSET
    }

    /// Set **LINK_ADDR_1** `[31:0]`
    ///
    /// Link_1_addr reg
    #[inline(always)]
    pub const fn with_link_addr_1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegdmaLink1Addr::LINK_ADDR_1_MASK)
                | ((value << RegdmaLink1Addr::LINK_ADDR_1_OFFSET)
                    & RegdmaLink1Addr::LINK_ADDR_1_MASK),
        }
    }
}

impl Default for RegdmaLink1AddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegdmaLink1Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegdmaLink1AddrVal {
        RegdmaLink1AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegdmaLink1AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegdmaLink1AddrVal) -> RegdmaLink1AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REGDMA_LINK_2_ADDR Register
///
/// Link_2_addr
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **LINK_ADDR_2** `[31:0]` (RW)
///
///   Link_2_addr reg
#[repr(transparent)]
pub struct RegdmaLink2Addr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegdmaLink2Addr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegdmaLink2Addr {
    // ======== Field Constants ========

    /// **LINK_ADDR_2**
    ///
    /// Link_2_addr reg
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LINK_ADDR_2_OFFSET: u32 = 0;
    pub const LINK_ADDR_2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LINK_ADDR_2** `[31:0]` (RW)
    ///
    /// Link_2_addr reg
    #[inline(always)]
    pub fn link_addr_2(&self) -> u32 {
        (self.value.read_bits() & Self::LINK_ADDR_2_MASK) >> Self::LINK_ADDR_2_OFFSET
    }

    /// Write **LINK_ADDR_2** `[31:0]` (RW)
    ///
    /// Link_2_addr reg
    #[inline(always)]
    pub fn write_link_addr_2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LINK_ADDR_2_MASK)
                | ((value << Self::LINK_ADDR_2_OFFSET) & Self::LINK_ADDR_2_MASK)
        });
    }
}

/// REGDMA_LINK_2_ADDR value type for batch read/modify/write operations
///
/// Use `regdmalink2addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regdmalink2addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegdmaLink2AddrVal {
    bits: u32,
}

impl RegdmaLink2AddrVal {
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

    /// Read **LINK_ADDR_2** `[31:0]`
    ///
    /// Link_2_addr reg
    #[inline(always)]
    pub const fn link_addr_2(&self) -> u32 {
        (self.bits & RegdmaLink2Addr::LINK_ADDR_2_MASK) >> RegdmaLink2Addr::LINK_ADDR_2_OFFSET
    }

    /// Set **LINK_ADDR_2** `[31:0]`
    ///
    /// Link_2_addr reg
    #[inline(always)]
    pub const fn with_link_addr_2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegdmaLink2Addr::LINK_ADDR_2_MASK)
                | ((value << RegdmaLink2Addr::LINK_ADDR_2_OFFSET)
                    & RegdmaLink2Addr::LINK_ADDR_2_MASK),
        }
    }
}

impl Default for RegdmaLink2AddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegdmaLink2Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegdmaLink2AddrVal {
        RegdmaLink2AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegdmaLink2AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegdmaLink2AddrVal) -> RegdmaLink2AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REGDMA_LINK_3_ADDR Register
///
/// Link_3_addr
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **LINK_ADDR_3** `[31:0]` (RW)
///
///   Link_3_addr reg
#[repr(transparent)]
pub struct RegdmaLink3Addr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegdmaLink3Addr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegdmaLink3Addr {
    // ======== Field Constants ========

    /// **LINK_ADDR_3**
    ///
    /// Link_3_addr reg
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LINK_ADDR_3_OFFSET: u32 = 0;
    pub const LINK_ADDR_3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LINK_ADDR_3** `[31:0]` (RW)
    ///
    /// Link_3_addr reg
    #[inline(always)]
    pub fn link_addr_3(&self) -> u32 {
        (self.value.read_bits() & Self::LINK_ADDR_3_MASK) >> Self::LINK_ADDR_3_OFFSET
    }

    /// Write **LINK_ADDR_3** `[31:0]` (RW)
    ///
    /// Link_3_addr reg
    #[inline(always)]
    pub fn write_link_addr_3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LINK_ADDR_3_MASK)
                | ((value << Self::LINK_ADDR_3_OFFSET) & Self::LINK_ADDR_3_MASK)
        });
    }
}

/// REGDMA_LINK_3_ADDR value type for batch read/modify/write operations
///
/// Use `regdmalink3addr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regdmalink3addr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegdmaLink3AddrVal {
    bits: u32,
}

impl RegdmaLink3AddrVal {
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

    /// Read **LINK_ADDR_3** `[31:0]`
    ///
    /// Link_3_addr reg
    #[inline(always)]
    pub const fn link_addr_3(&self) -> u32 {
        (self.bits & RegdmaLink3Addr::LINK_ADDR_3_MASK) >> RegdmaLink3Addr::LINK_ADDR_3_OFFSET
    }

    /// Set **LINK_ADDR_3** `[31:0]`
    ///
    /// Link_3_addr reg
    #[inline(always)]
    pub const fn with_link_addr_3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegdmaLink3Addr::LINK_ADDR_3_MASK)
                | ((value << RegdmaLink3Addr::LINK_ADDR_3_OFFSET)
                    & RegdmaLink3Addr::LINK_ADDR_3_MASK),
        }
    }
}

impl Default for RegdmaLink3AddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegdmaLink3Addr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegdmaLink3AddrVal {
        RegdmaLink3AddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegdmaLink3AddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegdmaLink3AddrVal) -> RegdmaLink3AddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REGDMA_LINK_MAC_ADDR Register
///
/// Link_mac_addr
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **LINK_ADDR_MAC** `[31:0]` (RW)
///
///   Link_mac_addr reg
#[repr(transparent)]
pub struct RegdmaLinkMacAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegdmaLinkMacAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegdmaLinkMacAddr {
    // ======== Field Constants ========

    /// **LINK_ADDR_MAC**
    ///
    /// Link_mac_addr reg
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LINK_ADDR_MAC_OFFSET: u32 = 0;
    pub const LINK_ADDR_MAC_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LINK_ADDR_MAC** `[31:0]` (RW)
    ///
    /// Link_mac_addr reg
    #[inline(always)]
    pub fn link_addr_mac(&self) -> u32 {
        (self.value.read_bits() & Self::LINK_ADDR_MAC_MASK) >> Self::LINK_ADDR_MAC_OFFSET
    }

    /// Write **LINK_ADDR_MAC** `[31:0]` (RW)
    ///
    /// Link_mac_addr reg
    #[inline(always)]
    pub fn write_link_addr_mac(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LINK_ADDR_MAC_MASK)
                | ((value << Self::LINK_ADDR_MAC_OFFSET) & Self::LINK_ADDR_MAC_MASK)
        });
    }
}

/// REGDMA_LINK_MAC_ADDR value type for batch read/modify/write operations
///
/// Use `regdmalinkmacaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regdmalinkmacaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegdmaLinkMacAddrVal {
    bits: u32,
}

impl RegdmaLinkMacAddrVal {
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

    /// Read **LINK_ADDR_MAC** `[31:0]`
    ///
    /// Link_mac_addr reg
    #[inline(always)]
    pub const fn link_addr_mac(&self) -> u32 {
        (self.bits & RegdmaLinkMacAddr::LINK_ADDR_MAC_MASK)
            >> RegdmaLinkMacAddr::LINK_ADDR_MAC_OFFSET
    }

    /// Set **LINK_ADDR_MAC** `[31:0]`
    ///
    /// Link_mac_addr reg
    #[inline(always)]
    pub const fn with_link_addr_mac(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegdmaLinkMacAddr::LINK_ADDR_MAC_MASK)
                | ((value << RegdmaLinkMacAddr::LINK_ADDR_MAC_OFFSET)
                    & RegdmaLinkMacAddr::LINK_ADDR_MAC_MASK),
        }
    }
}

impl Default for RegdmaLinkMacAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegdmaLinkMacAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegdmaLinkMacAddrVal {
        RegdmaLinkMacAddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegdmaLinkMacAddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegdmaLinkMacAddrVal) -> RegdmaLinkMacAddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REGDMA_CURRENT_LINK_ADDR Register
///
/// current link addr
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **CURRENT_LINK_ADDR** `[31:0]` (RO)
///
///   current link addr reg
#[repr(transparent)]
pub struct RegdmaCurrentLinkAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegdmaCurrentLinkAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegdmaCurrentLinkAddr {
    // ======== Field Constants ========

    /// **CURRENT_LINK_ADDR**
    ///
    /// current link addr reg
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CURRENT_LINK_ADDR_OFFSET: u32 = 0;
    pub const CURRENT_LINK_ADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CURRENT_LINK_ADDR** `[31:0]` (RO)
    ///
    /// current link addr reg
    #[inline(always)]
    pub fn current_link_addr(&self) -> u32 {
        (self.value.read_bits() & Self::CURRENT_LINK_ADDR_MASK) >> Self::CURRENT_LINK_ADDR_OFFSET
    }
}

/// REGDMA_CURRENT_LINK_ADDR value type for batch read/modify/write operations
///
/// Use `regdmacurrentlinkaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regdmacurrentlinkaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegdmaCurrentLinkAddrVal {
    bits: u32,
}

impl RegdmaCurrentLinkAddrVal {
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

    /// Read **CURRENT_LINK_ADDR** `[31:0]`
    ///
    /// current link addr reg
    #[inline(always)]
    pub const fn current_link_addr(&self) -> u32 {
        (self.bits & RegdmaCurrentLinkAddr::CURRENT_LINK_ADDR_MASK)
            >> RegdmaCurrentLinkAddr::CURRENT_LINK_ADDR_OFFSET
    }
}

impl Default for RegdmaCurrentLinkAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegdmaCurrentLinkAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegdmaCurrentLinkAddrVal {
        RegdmaCurrentLinkAddrVal::from_bits(self.value.read_bits())
    }
}

/// REGDMA_BACKUP_ADDR Register
///
/// Backup addr
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **BACKUP_ADDR** `[31:0]` (RO)
///
///   backup addr reg
#[repr(transparent)]
pub struct RegdmaBackupAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegdmaBackupAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegdmaBackupAddr {
    // ======== Field Constants ========

    /// **BACKUP_ADDR**
    ///
    /// backup addr reg
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const BACKUP_ADDR_OFFSET: u32 = 0;
    pub const BACKUP_ADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **BACKUP_ADDR** `[31:0]` (RO)
    ///
    /// backup addr reg
    #[inline(always)]
    pub fn backup_addr(&self) -> u32 {
        (self.value.read_bits() & Self::BACKUP_ADDR_MASK) >> Self::BACKUP_ADDR_OFFSET
    }
}

/// REGDMA_BACKUP_ADDR value type for batch read/modify/write operations
///
/// Use `regdmabackupaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regdmabackupaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegdmaBackupAddrVal {
    bits: u32,
}

impl RegdmaBackupAddrVal {
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

    /// Read **BACKUP_ADDR** `[31:0]`
    ///
    /// backup addr reg
    #[inline(always)]
    pub const fn backup_addr(&self) -> u32 {
        (self.bits & RegdmaBackupAddr::BACKUP_ADDR_MASK) >> RegdmaBackupAddr::BACKUP_ADDR_OFFSET
    }
}

impl Default for RegdmaBackupAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegdmaBackupAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegdmaBackupAddrVal {
        RegdmaBackupAddrVal::from_bits(self.value.read_bits())
    }
}

/// REGDMA_MEM_ADDR Register
///
/// mem addr
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **MEM_ADDR** `[31:0]` (RO)
///
///   mem addr reg
#[repr(transparent)]
pub struct RegdmaMemAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegdmaMemAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegdmaMemAddr {
    // ======== Field Constants ========

    /// **MEM_ADDR**
    ///
    /// mem addr reg
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MEM_ADDR_OFFSET: u32 = 0;
    pub const MEM_ADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MEM_ADDR** `[31:0]` (RO)
    ///
    /// mem addr reg
    #[inline(always)]
    pub fn mem_addr(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_ADDR_MASK) >> Self::MEM_ADDR_OFFSET
    }
}

/// REGDMA_MEM_ADDR value type for batch read/modify/write operations
///
/// Use `regdmamemaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regdmamemaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegdmaMemAddrVal {
    bits: u32,
}

impl RegdmaMemAddrVal {
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

    /// Read **MEM_ADDR** `[31:0]`
    ///
    /// mem addr reg
    #[inline(always)]
    pub const fn mem_addr(&self) -> u32 {
        (self.bits & RegdmaMemAddr::MEM_ADDR_MASK) >> RegdmaMemAddr::MEM_ADDR_OFFSET
    }
}

impl Default for RegdmaMemAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegdmaMemAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegdmaMemAddrVal {
        RegdmaMemAddrVal::from_bits(self.value.read_bits())
    }
}

/// REGDMA_BKP_CONF Register
///
/// backup config
///
/// Address offset: 0x002c
/// Reset value: 0x7d101920
///
/// ## Fields
///
/// - **BACKUP_TOUT_THRES** `[31:22]` (RW)
///
///   Backup timeout threshold
///
/// - **BURST_LIMIT** `[21:17]` (RW)
///
///   burst limit
///
/// - **LINK_TOUT_THRES** `[16:7]` (RW)
///
///   link wait timeout threshold
///
/// - **READ_INTERVAL** `[6:0]` (RW)
///
///   Link read_interval
#[repr(transparent)]
pub struct RegdmaBkpConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegdmaBkpConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegdmaBkpConf {
    // ======== Field Constants ========

    /// **BACKUP_TOUT_THRES**
    ///
    /// Backup timeout threshold
    ///
    /// Bits `[31:22]`, mask `0xffc00000`
    pub const BACKUP_TOUT_THRES_OFFSET: u32 = 22;
    pub const BACKUP_TOUT_THRES_MASK: u32 = 0xffc00000;

    /// **BURST_LIMIT**
    ///
    /// burst limit
    ///
    /// Bits `[21:17]`, mask `0x003e0000`
    pub const BURST_LIMIT_OFFSET: u32 = 17;
    pub const BURST_LIMIT_MASK: u32 = 0x003e0000;

    /// **LINK_TOUT_THRES**
    ///
    /// link wait timeout threshold
    ///
    /// Bits `[16:7]`, mask `0x0001ff80`
    pub const LINK_TOUT_THRES_OFFSET: u32 = 7;
    pub const LINK_TOUT_THRES_MASK: u32 = 0x0001ff80;

    /// **READ_INTERVAL**
    ///
    /// Link read_interval
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const READ_INTERVAL_OFFSET: u32 = 0;
    pub const READ_INTERVAL_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **BACKUP_TOUT_THRES** `[31:22]` (RW)
    ///
    /// Backup timeout threshold
    #[inline(always)]
    pub fn backup_tout_thres(&self) -> u32 {
        (self.value.read_bits() & Self::BACKUP_TOUT_THRES_MASK) >> Self::BACKUP_TOUT_THRES_OFFSET
    }

    /// Write **BACKUP_TOUT_THRES** `[31:22]` (RW)
    ///
    /// Backup timeout threshold
    #[inline(always)]
    pub fn write_backup_tout_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BACKUP_TOUT_THRES_MASK)
                | ((value << Self::BACKUP_TOUT_THRES_OFFSET) & Self::BACKUP_TOUT_THRES_MASK)
        });
    }

    /// **BURST_LIMIT** `[21:17]` (RW)
    ///
    /// burst limit
    #[inline(always)]
    pub fn burst_limit(&self) -> u32 {
        (self.value.read_bits() & Self::BURST_LIMIT_MASK) >> Self::BURST_LIMIT_OFFSET
    }

    /// Write **BURST_LIMIT** `[21:17]` (RW)
    ///
    /// burst limit
    #[inline(always)]
    pub fn write_burst_limit(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BURST_LIMIT_MASK)
                | ((value << Self::BURST_LIMIT_OFFSET) & Self::BURST_LIMIT_MASK)
        });
    }

    /// **LINK_TOUT_THRES** `[16:7]` (RW)
    ///
    /// link wait timeout threshold
    #[inline(always)]
    pub fn link_tout_thres(&self) -> u32 {
        (self.value.read_bits() & Self::LINK_TOUT_THRES_MASK) >> Self::LINK_TOUT_THRES_OFFSET
    }

    /// Write **LINK_TOUT_THRES** `[16:7]` (RW)
    ///
    /// link wait timeout threshold
    #[inline(always)]
    pub fn write_link_tout_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LINK_TOUT_THRES_MASK)
                | ((value << Self::LINK_TOUT_THRES_OFFSET) & Self::LINK_TOUT_THRES_MASK)
        });
    }

    /// **READ_INTERVAL** `[6:0]` (RW)
    ///
    /// Link read_interval
    #[inline(always)]
    pub fn read_interval(&self) -> u32 {
        (self.value.read_bits() & Self::READ_INTERVAL_MASK) >> Self::READ_INTERVAL_OFFSET
    }

    /// Write **READ_INTERVAL** `[6:0]` (RW)
    ///
    /// Link read_interval
    #[inline(always)]
    pub fn write_read_interval(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::READ_INTERVAL_MASK)
                | ((value << Self::READ_INTERVAL_OFFSET) & Self::READ_INTERVAL_MASK)
        });
    }
}

/// REGDMA_BKP_CONF value type for batch read/modify/write operations
///
/// Use `regdmabkpconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regdmabkpconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegdmaBkpConfVal {
    bits: u32,
}

impl RegdmaBkpConfVal {
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
        Self { bits: 0x7d101920 }
    }

    /// Read **BACKUP_TOUT_THRES** `[31:22]`
    ///
    /// Backup timeout threshold
    #[inline(always)]
    pub const fn backup_tout_thres(&self) -> u32 {
        (self.bits & RegdmaBkpConf::BACKUP_TOUT_THRES_MASK)
            >> RegdmaBkpConf::BACKUP_TOUT_THRES_OFFSET
    }

    /// Read **BURST_LIMIT** `[21:17]`
    ///
    /// burst limit
    #[inline(always)]
    pub const fn burst_limit(&self) -> u32 {
        (self.bits & RegdmaBkpConf::BURST_LIMIT_MASK) >> RegdmaBkpConf::BURST_LIMIT_OFFSET
    }

    /// Read **LINK_TOUT_THRES** `[16:7]`
    ///
    /// link wait timeout threshold
    #[inline(always)]
    pub const fn link_tout_thres(&self) -> u32 {
        (self.bits & RegdmaBkpConf::LINK_TOUT_THRES_MASK) >> RegdmaBkpConf::LINK_TOUT_THRES_OFFSET
    }

    /// Read **READ_INTERVAL** `[6:0]`
    ///
    /// Link read_interval
    #[inline(always)]
    pub const fn read_interval(&self) -> u32 {
        (self.bits & RegdmaBkpConf::READ_INTERVAL_MASK) >> RegdmaBkpConf::READ_INTERVAL_OFFSET
    }

    /// Set **BACKUP_TOUT_THRES** `[31:22]`
    ///
    /// Backup timeout threshold
    #[inline(always)]
    pub const fn with_backup_tout_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegdmaBkpConf::BACKUP_TOUT_THRES_MASK)
                | ((value << RegdmaBkpConf::BACKUP_TOUT_THRES_OFFSET)
                    & RegdmaBkpConf::BACKUP_TOUT_THRES_MASK),
        }
    }

    /// Set **BURST_LIMIT** `[21:17]`
    ///
    /// burst limit
    #[inline(always)]
    pub const fn with_burst_limit(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegdmaBkpConf::BURST_LIMIT_MASK)
                | ((value << RegdmaBkpConf::BURST_LIMIT_OFFSET) & RegdmaBkpConf::BURST_LIMIT_MASK),
        }
    }

    /// Set **LINK_TOUT_THRES** `[16:7]`
    ///
    /// link wait timeout threshold
    #[inline(always)]
    pub const fn with_link_tout_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegdmaBkpConf::LINK_TOUT_THRES_MASK)
                | ((value << RegdmaBkpConf::LINK_TOUT_THRES_OFFSET)
                    & RegdmaBkpConf::LINK_TOUT_THRES_MASK),
        }
    }

    /// Set **READ_INTERVAL** `[6:0]`
    ///
    /// Link read_interval
    #[inline(always)]
    pub const fn with_read_interval(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RegdmaBkpConf::READ_INTERVAL_MASK)
                | ((value << RegdmaBkpConf::READ_INTERVAL_OFFSET)
                    & RegdmaBkpConf::READ_INTERVAL_MASK),
        }
    }
}

impl Default for RegdmaBkpConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RegdmaBkpConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegdmaBkpConfVal {
        RegdmaBkpConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegdmaBkpConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegdmaBkpConfVal) -> RegdmaBkpConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ENA Register
///
/// Read only register for error and done
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **ERROR_INT_ENA** `[1]` (RW)
///
///   error flag
///
/// - **DONE_INT_ENA** `[0]` (RW)
///
///   backup done flag
#[repr(transparent)]
pub struct IntEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntEna {
    // ======== Field Constants ========

    /// **ERROR_INT_ENA**
    ///
    /// error flag
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ERROR_INT_ENA_OFFSET: u32 = 1;
    pub const ERROR_INT_ENA_MASK: u32 = 0x00000002;
    pub const ERROR_INT_ENA: u32 = 1 << 1;

    /// **DONE_INT_ENA**
    ///
    /// backup done flag
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DONE_INT_ENA_OFFSET: u32 = 0;
    pub const DONE_INT_ENA_MASK: u32 = 0x00000001;
    pub const DONE_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ERROR_INT_ENA** `[1]` (RW)
    ///
    /// error flag
    #[inline(always)]
    pub fn error_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_INT_ENA_MASK) != 0
    }

    /// Set ERROR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_error_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::ERROR_INT_ENA_MASK);
    }

    /// Clear ERROR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_error_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::ERROR_INT_ENA_MASK);
    }

    /// **DONE_INT_ENA** `[0]` (RW)
    ///
    /// backup done flag
    #[inline(always)]
    pub fn done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DONE_INT_ENA_MASK) != 0
    }

    /// Set DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::DONE_INT_ENA_MASK);
    }

    /// Clear DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_done_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::DONE_INT_ENA_MASK);
    }
}

/// INT_ENA value type for batch read/modify/write operations
///
/// Use `intena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntEnaVal {
    bits: u32,
}

impl IntEnaVal {
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

    /// Read **ERROR_INT_ENA** `[1:1]`
    ///
    /// error flag
    #[inline(always)]
    pub const fn error_int_ena(&self) -> bool {
        (self.bits & IntEna::ERROR_INT_ENA_MASK) != 0
    }

    /// Read **DONE_INT_ENA** `[0:0]`
    ///
    /// backup done flag
    #[inline(always)]
    pub const fn done_int_ena(&self) -> bool {
        (self.bits & IntEna::DONE_INT_ENA_MASK) != 0
    }

    /// Set **ERROR_INT_ENA** `[1:1]`
    ///
    /// error flag
    #[inline(always)]
    pub const fn with_error_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::ERROR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::ERROR_INT_ENA_MASK
            },
        }
    }

    /// Set **DONE_INT_ENA** `[0:0]`
    ///
    /// backup done flag
    #[inline(always)]
    pub const fn with_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DONE_INT_ENA_MASK
            },
        }
    }
}

impl Default for IntEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntEnaVal {
        IntEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntEnaVal) -> IntEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_RAW Register
///
/// Read only register for error and done
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **ERROR_INT_RAW** `[1]` (RW)
///
///   error flag
///
/// - **DONE_INT_RAW** `[0]` (RW)
///
///   backup done flag
#[repr(transparent)]
pub struct IntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntRaw {
    // ======== Field Constants ========

    /// **ERROR_INT_RAW**
    ///
    /// error flag
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ERROR_INT_RAW_OFFSET: u32 = 1;
    pub const ERROR_INT_RAW_MASK: u32 = 0x00000002;
    pub const ERROR_INT_RAW: u32 = 1 << 1;

    /// **DONE_INT_RAW**
    ///
    /// backup done flag
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DONE_INT_RAW_OFFSET: u32 = 0;
    pub const DONE_INT_RAW_MASK: u32 = 0x00000001;
    pub const DONE_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ERROR_INT_RAW** `[1]` (RW)
    ///
    /// error flag
    #[inline(always)]
    pub fn error_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_INT_RAW_MASK) != 0
    }

    /// Set ERROR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_error_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::ERROR_INT_RAW_MASK);
    }

    /// Clear ERROR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_error_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::ERROR_INT_RAW_MASK);
    }

    /// **DONE_INT_RAW** `[0]` (RW)
    ///
    /// backup done flag
    #[inline(always)]
    pub fn done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DONE_INT_RAW_MASK) != 0
    }

    /// Set DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_done_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::DONE_INT_RAW_MASK);
    }

    /// Clear DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_done_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::DONE_INT_RAW_MASK);
    }
}

/// INT_RAW value type for batch read/modify/write operations
///
/// Use `intraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntRawVal {
    bits: u32,
}

impl IntRawVal {
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

    /// Read **ERROR_INT_RAW** `[1:1]`
    ///
    /// error flag
    #[inline(always)]
    pub const fn error_int_raw(&self) -> bool {
        (self.bits & IntRaw::ERROR_INT_RAW_MASK) != 0
    }

    /// Read **DONE_INT_RAW** `[0:0]`
    ///
    /// backup done flag
    #[inline(always)]
    pub const fn done_int_raw(&self) -> bool {
        (self.bits & IntRaw::DONE_INT_RAW_MASK) != 0
    }

    /// Set **ERROR_INT_RAW** `[1:1]`
    ///
    /// error flag
    #[inline(always)]
    pub const fn with_error_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::ERROR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::ERROR_INT_RAW_MASK
            },
        }
    }

    /// Set **DONE_INT_RAW** `[0:0]`
    ///
    /// backup done flag
    #[inline(always)]
    pub const fn with_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DONE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DONE_INT_RAW_MASK
            },
        }
    }
}

impl Default for IntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntRawVal {
        IntRawVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntRawVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntRawVal) -> IntRawVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_CLR Register
///
/// Read only register for error and done
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **ERROR_INT_CLR** `[1]` (WO)
///
///   error flag
///
/// - **DONE_INT_CLR** `[0]` (WO)
///
///   backup done flag
#[repr(transparent)]
pub struct IntClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntClr {
    // ======== Field Constants ========

    /// **ERROR_INT_CLR**
    ///
    /// error flag
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ERROR_INT_CLR_OFFSET: u32 = 1;
    pub const ERROR_INT_CLR_MASK: u32 = 0x00000002;
    pub const ERROR_INT_CLR: u32 = 1 << 1;

    /// **DONE_INT_CLR**
    ///
    /// backup done flag
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DONE_INT_CLR_OFFSET: u32 = 0;
    pub const DONE_INT_CLR_MASK: u32 = 0x00000001;
    pub const DONE_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **ERROR_INT_CLR** (WO)
    ///
    /// error flag
    #[inline(always)]
    pub fn trigger_error_int_clr(&self) {
        self.value.write_bits(Self::ERROR_INT_CLR_MASK);
    }

    /// Trigger **DONE_INT_CLR** (WO)
    ///
    /// backup done flag
    #[inline(always)]
    pub fn trigger_done_int_clr(&self) {
        self.value.write_bits(Self::DONE_INT_CLR_MASK);
    }
}

/// INT_CLR value type for batch read/modify/write operations
///
/// Use `intclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntClrVal {
    bits: u32,
}

impl IntClrVal {
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

    /// Set **ERROR_INT_CLR** `[1:1]`
    ///
    /// error flag
    #[inline(always)]
    pub const fn with_error_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::ERROR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::ERROR_INT_CLR_MASK
            },
        }
    }

    /// Set **DONE_INT_CLR** `[0:0]`
    ///
    /// backup done flag
    #[inline(always)]
    pub const fn with_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DONE_INT_CLR_MASK
            },
        }
    }
}

impl Default for IntClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// INT_ST Register
///
/// Read only register for error and done
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **ERROR_INT_ST** `[1]` (RO)
///
///   error flag
///
/// - **DONE_INT_ST** `[0]` (RO)
///
///   backup done flag
#[repr(transparent)]
pub struct IntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntSt {
    // ======== Field Constants ========

    /// **ERROR_INT_ST**
    ///
    /// error flag
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ERROR_INT_ST_OFFSET: u32 = 1;
    pub const ERROR_INT_ST_MASK: u32 = 0x00000002;
    pub const ERROR_INT_ST: u32 = 1 << 1;

    /// **DONE_INT_ST**
    ///
    /// backup done flag
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DONE_INT_ST_OFFSET: u32 = 0;
    pub const DONE_INT_ST_MASK: u32 = 0x00000001;
    pub const DONE_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ERROR_INT_ST** `[1]` (RO)
    ///
    /// error flag
    #[inline(always)]
    pub fn error_int_st(&self) -> bool {
        (self.value.read_bits() & Self::ERROR_INT_ST_MASK) != 0
    }

    /// **DONE_INT_ST** `[0]` (RO)
    ///
    /// backup done flag
    #[inline(always)]
    pub fn done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DONE_INT_ST_MASK) != 0
    }
}

/// INT_ST value type for batch read/modify/write operations
///
/// Use `intst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStVal {
    bits: u32,
}

impl IntStVal {
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

    /// Read **ERROR_INT_ST** `[1:1]`
    ///
    /// error flag
    #[inline(always)]
    pub const fn error_int_st(&self) -> bool {
        (self.bits & IntSt::ERROR_INT_ST_MASK) != 0
    }

    /// Read **DONE_INT_ST** `[0:0]`
    ///
    /// backup done flag
    #[inline(always)]
    pub const fn done_int_st(&self) -> bool {
        (self.bits & IntSt::DONE_INT_ST_MASK) != 0
    }
}

impl Default for IntStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStVal {
        IntStVal::from_bits(self.value.read_bits())
    }
}

/// DATE Register
///
/// Date register.
///
/// Address offset: 0x03fc
/// Reset value: 0x02203070
///
/// ## Fields
///
/// - **DATE** `[27:0]` (RW)
///
///   REGDMA date information/ REGDMA version information.
#[repr(transparent)]
pub struct Date {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Date {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Date {
    // ======== Field Constants ========

    /// **DATE**
    ///
    /// REGDMA date information/ REGDMA version information.
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **DATE** `[27:0]` (RW)
    ///
    /// REGDMA date information/ REGDMA version information.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[27:0]` (RW)
    ///
    /// REGDMA date information/ REGDMA version information.
    #[inline(always)]
    pub fn write_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATE_MASK) | ((value << Self::DATE_OFFSET) & Self::DATE_MASK)
        });
    }
}

/// DATE value type for batch read/modify/write operations
///
/// Use `date.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `date.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DateVal {
    bits: u32,
}

impl DateVal {
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
        Self { bits: 0x02203070 }
    }

    /// Read **DATE** `[27:0]`
    ///
    /// REGDMA date information/ REGDMA version information.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[27:0]`
    ///
    /// REGDMA date information/ REGDMA version information.
    #[inline(always)]
    pub const fn with_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Date::DATE_MASK) | ((value << Date::DATE_OFFSET) & Date::DATE_MASK),
        }
    }
}

impl Default for DateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Date {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DateVal {
        DateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DateVal) -> DateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Peri backup control register
    pub regdma_conf: RegdmaConf,
    /// 0x0004
    /// Clock control register
    pub regdma_clk_conf: RegdmaClkConf,
    /// 0x0008
    /// ETM start ctrl reg
    pub regdma_etm_ctrl: RegdmaEtmCtrl,
    /// 0x000c
    /// link_0_addr
    pub regdma_link0_addr: RegdmaLink0Addr,
    /// 0x0010
    /// Link_1_addr
    pub regdma_link1_addr: RegdmaLink1Addr,
    /// 0x0014
    /// Link_2_addr
    pub regdma_link2_addr: RegdmaLink2Addr,
    /// 0x0018
    /// Link_3_addr
    pub regdma_link3_addr: RegdmaLink3Addr,
    /// 0x001c
    /// Link_mac_addr
    pub regdma_link_mac_addr: RegdmaLinkMacAddr,
    /// 0x0020
    /// current link addr
    pub regdma_current_link_addr: RegdmaCurrentLinkAddr,
    /// 0x0024
    /// Backup addr
    pub regdma_backup_addr: RegdmaBackupAddr,
    /// 0x0028
    /// mem addr
    pub regdma_mem_addr: RegdmaMemAddr,
    /// 0x002c
    /// backup config
    pub regdma_bkp_conf: RegdmaBkpConf,
    /// 0x0030
    /// Read only register for error and done
    pub int_ena: IntEna,
    /// 0x0034
    /// Read only register for error and done
    pub int_raw: IntRaw,
    /// 0x0038
    /// Read only register for error and done
    pub int_clr: IntClr,
    /// 0x003c
    /// Read only register for error and done
    pub int_st: IntSt,
    _reserved0: [u8; 956],
    /// 0x03fc
    /// Date register.
    pub date: Date,
}
