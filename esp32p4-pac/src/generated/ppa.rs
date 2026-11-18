//! PPA
//! PPA Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x50087000;

/// BLEND0_CLUT_DATA Register
///
/// CLUT sram data read/write register in background plane of blender
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **RDWR_WORD_BLEND0_CLUT** `[31:0]` (RW)
///
///   Write and read data to/from CLUT RAM in background plane of blender engine through this field in fifo mode.
#[repr(transparent)]
pub struct Blend0ClutData {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Blend0ClutData {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Blend0ClutData {
    // ======== Field Constants ========

    /// **RDWR_WORD_BLEND0_CLUT**
    ///
    /// Write and read data to/from CLUT RAM in background plane of blender engine through this field in fifo mode.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RDWR_WORD_BLEND0_CLUT_OFFSET: u32 = 0;
    pub const RDWR_WORD_BLEND0_CLUT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RDWR_WORD_BLEND0_CLUT** `[31:0]` (RW)
    ///
    /// Write and read data to/from CLUT RAM in background plane of blender engine through this field in fifo mode.
    #[inline(always)]
    pub fn rdwr_word_blend0_clut(&self) -> u32 {
        (self.value.read_bits() & Self::RDWR_WORD_BLEND0_CLUT_MASK)
            >> Self::RDWR_WORD_BLEND0_CLUT_OFFSET
    }

    /// Write **RDWR_WORD_BLEND0_CLUT** `[31:0]` (RW)
    ///
    /// Write and read data to/from CLUT RAM in background plane of blender engine through this field in fifo mode.
    #[inline(always)]
    pub fn write_rdwr_word_blend0_clut(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RDWR_WORD_BLEND0_CLUT_MASK)
                | ((value << Self::RDWR_WORD_BLEND0_CLUT_OFFSET) & Self::RDWR_WORD_BLEND0_CLUT_MASK)
        });
    }
}

/// BLEND0_CLUT_DATA value type for batch read/modify/write operations
///
/// Use `blend0clutdata.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `blend0clutdata.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Blend0ClutDataVal {
    bits: u32,
}

impl Blend0ClutDataVal {
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

    /// Read **RDWR_WORD_BLEND0_CLUT** `[31:0]`
    ///
    /// Write and read data to/from CLUT RAM in background plane of blender engine through this field in fifo mode.
    #[inline(always)]
    pub const fn rdwr_word_blend0_clut(&self) -> u32 {
        (self.bits & Blend0ClutData::RDWR_WORD_BLEND0_CLUT_MASK)
            >> Blend0ClutData::RDWR_WORD_BLEND0_CLUT_OFFSET
    }

    /// Set **RDWR_WORD_BLEND0_CLUT** `[31:0]`
    ///
    /// Write and read data to/from CLUT RAM in background plane of blender engine through this field in fifo mode.
    #[inline(always)]
    pub const fn with_rdwr_word_blend0_clut(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Blend0ClutData::RDWR_WORD_BLEND0_CLUT_MASK)
                | ((value << Blend0ClutData::RDWR_WORD_BLEND0_CLUT_OFFSET)
                    & Blend0ClutData::RDWR_WORD_BLEND0_CLUT_MASK),
        }
    }
}

impl Default for Blend0ClutDataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Blend0ClutData {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Blend0ClutDataVal {
        Blend0ClutDataVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Blend0ClutDataVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Blend0ClutDataVal) -> Blend0ClutDataVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BLEND1_CLUT_DATA Register
///
/// CLUT sram data read/write register in foreground plane of blender
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **RDWR_WORD_BLEND1_CLUT** `[31:0]` (RW)
///
///   Write and read data to/from CLUT RAM in foreground plane of blender engine through this field in fifo mode.
#[repr(transparent)]
pub struct Blend1ClutData {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Blend1ClutData {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Blend1ClutData {
    // ======== Field Constants ========

    /// **RDWR_WORD_BLEND1_CLUT**
    ///
    /// Write and read data to/from CLUT RAM in foreground plane of blender engine through this field in fifo mode.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RDWR_WORD_BLEND1_CLUT_OFFSET: u32 = 0;
    pub const RDWR_WORD_BLEND1_CLUT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RDWR_WORD_BLEND1_CLUT** `[31:0]` (RW)
    ///
    /// Write and read data to/from CLUT RAM in foreground plane of blender engine through this field in fifo mode.
    #[inline(always)]
    pub fn rdwr_word_blend1_clut(&self) -> u32 {
        (self.value.read_bits() & Self::RDWR_WORD_BLEND1_CLUT_MASK)
            >> Self::RDWR_WORD_BLEND1_CLUT_OFFSET
    }

    /// Write **RDWR_WORD_BLEND1_CLUT** `[31:0]` (RW)
    ///
    /// Write and read data to/from CLUT RAM in foreground plane of blender engine through this field in fifo mode.
    #[inline(always)]
    pub fn write_rdwr_word_blend1_clut(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RDWR_WORD_BLEND1_CLUT_MASK)
                | ((value << Self::RDWR_WORD_BLEND1_CLUT_OFFSET) & Self::RDWR_WORD_BLEND1_CLUT_MASK)
        });
    }
}

/// BLEND1_CLUT_DATA value type for batch read/modify/write operations
///
/// Use `blend1clutdata.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `blend1clutdata.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Blend1ClutDataVal {
    bits: u32,
}

impl Blend1ClutDataVal {
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

    /// Read **RDWR_WORD_BLEND1_CLUT** `[31:0]`
    ///
    /// Write and read data to/from CLUT RAM in foreground plane of blender engine through this field in fifo mode.
    #[inline(always)]
    pub const fn rdwr_word_blend1_clut(&self) -> u32 {
        (self.bits & Blend1ClutData::RDWR_WORD_BLEND1_CLUT_MASK)
            >> Blend1ClutData::RDWR_WORD_BLEND1_CLUT_OFFSET
    }

    /// Set **RDWR_WORD_BLEND1_CLUT** `[31:0]`
    ///
    /// Write and read data to/from CLUT RAM in foreground plane of blender engine through this field in fifo mode.
    #[inline(always)]
    pub const fn with_rdwr_word_blend1_clut(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Blend1ClutData::RDWR_WORD_BLEND1_CLUT_MASK)
                | ((value << Blend1ClutData::RDWR_WORD_BLEND1_CLUT_OFFSET)
                    & Blend1ClutData::RDWR_WORD_BLEND1_CLUT_MASK),
        }
    }
}

impl Default for Blend1ClutDataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Blend1ClutData {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Blend1ClutDataVal {
        Blend1ClutDataVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Blend1ClutDataVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Blend1ClutDataVal) -> Blend1ClutDataVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLUT_CONF Register
///
/// CLUT configure register
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **BLEND0_CLUT_MEM_CLK_ENA** `[7]` (RW)
///
///   1: Force clock on for BLEND CLUT memory.
///
/// - **BLEND0_CLUT_MEM_FORCE_PU** `[6]` (RW)
///
///   1: force power up BLEND CLUT memory.
///
/// - **BLEND0_CLUT_MEM_FORCE_PD** `[5]` (RW)
///
///   1: force power down BLEND CLUT memory.
///
/// - **BLEND1_CLUT_MEM_RDADDR_RST** `[4]` (RW)
///
///   Write 1 then write 0 to reset the read address of BLEND1 CLUT in fifo mode.
///
/// - **BLEND0_CLUT_MEM_RDADDR_RST** `[3]` (RW)
///
///   Write 1 then write 0 to reset the read address of BLEND0 CLUT in fifo mode.
///
/// - **BLEND1_CLUT_MEM_RST** `[2]` (RW)
///
///   Write 1 then write 0 to this bit to reset BLEND1 CLUT.
///
/// - **BLEND0_CLUT_MEM_RST** `[1]` (RW)
///
///   Write 1 then write 0 to this bit to reset BLEND0 CLUT.
///
/// - **APB_FIFO_MASK** `[0]` (RW)
///
///   1'b0: fifo mode to wr/rd clut0/clut1 RAM through register PPA_SR_CLUT_DATA_REG/PPA_BLEND0_CLUT_DATA_REG/PPA_BLEND1_CLUT_DATA_REG. 1'b1: memory mode to wr/rd sr/blend0/blend1 clut RAM. The bit 11 and 10 of the waddr should be 01 to access sr clut and should be 10 to access blend0 clut and should be 11 to access blend 1 clut in memory mode.
#[repr(transparent)]
pub struct ClutConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClutConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClutConf {
    // ======== Field Constants ========

    /// **BLEND0_CLUT_MEM_CLK_ENA**
    ///
    /// 1: Force clock on for BLEND CLUT memory.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const BLEND0_CLUT_MEM_CLK_ENA_OFFSET: u32 = 7;
    pub const BLEND0_CLUT_MEM_CLK_ENA_MASK: u32 = 0x00000080;
    pub const BLEND0_CLUT_MEM_CLK_ENA: u32 = 1 << 7;

    /// **BLEND0_CLUT_MEM_FORCE_PU**
    ///
    /// 1: force power up BLEND CLUT memory.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const BLEND0_CLUT_MEM_FORCE_PU_OFFSET: u32 = 6;
    pub const BLEND0_CLUT_MEM_FORCE_PU_MASK: u32 = 0x00000040;
    pub const BLEND0_CLUT_MEM_FORCE_PU: u32 = 1 << 6;

    /// **BLEND0_CLUT_MEM_FORCE_PD**
    ///
    /// 1: force power down BLEND CLUT memory.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const BLEND0_CLUT_MEM_FORCE_PD_OFFSET: u32 = 5;
    pub const BLEND0_CLUT_MEM_FORCE_PD_MASK: u32 = 0x00000020;
    pub const BLEND0_CLUT_MEM_FORCE_PD: u32 = 1 << 5;

    /// **BLEND1_CLUT_MEM_RDADDR_RST**
    ///
    /// Write 1 then write 0 to reset the read address of BLEND1 CLUT in fifo mode.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BLEND1_CLUT_MEM_RDADDR_RST_OFFSET: u32 = 4;
    pub const BLEND1_CLUT_MEM_RDADDR_RST_MASK: u32 = 0x00000010;
    pub const BLEND1_CLUT_MEM_RDADDR_RST: u32 = 1 << 4;

    /// **BLEND0_CLUT_MEM_RDADDR_RST**
    ///
    /// Write 1 then write 0 to reset the read address of BLEND0 CLUT in fifo mode.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const BLEND0_CLUT_MEM_RDADDR_RST_OFFSET: u32 = 3;
    pub const BLEND0_CLUT_MEM_RDADDR_RST_MASK: u32 = 0x00000008;
    pub const BLEND0_CLUT_MEM_RDADDR_RST: u32 = 1 << 3;

    /// **BLEND1_CLUT_MEM_RST**
    ///
    /// Write 1 then write 0 to this bit to reset BLEND1 CLUT.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const BLEND1_CLUT_MEM_RST_OFFSET: u32 = 2;
    pub const BLEND1_CLUT_MEM_RST_MASK: u32 = 0x00000004;
    pub const BLEND1_CLUT_MEM_RST: u32 = 1 << 2;

    /// **BLEND0_CLUT_MEM_RST**
    ///
    /// Write 1 then write 0 to this bit to reset BLEND0 CLUT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BLEND0_CLUT_MEM_RST_OFFSET: u32 = 1;
    pub const BLEND0_CLUT_MEM_RST_MASK: u32 = 0x00000002;
    pub const BLEND0_CLUT_MEM_RST: u32 = 1 << 1;

    /// **APB_FIFO_MASK**
    ///
    /// 1'b0: fifo mode to wr/rd clut0/clut1 RAM through register PPA_SR_CLUT_DATA_REG/PPA_BLEND0_CLUT_DATA_REG/PPA_BLEND1_CLUT_DATA_REG. 1'b1: memory mode to wr/rd sr/blend0/blend1 clut RAM. The bit 11 and 10 of the waddr should be 01 to access sr clut and should be 10 to access blend0 clut and should be 11 to access blend 1 clut in memory mode.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const APB_FIFO_MASK_OFFSET: u32 = 0;
    pub const APB_FIFO_MASK_MASK: u32 = 0x00000001;
    pub const APB_FIFO_MASK: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BLEND0_CLUT_MEM_CLK_ENA** `[7]` (RW)
    ///
    /// 1: Force clock on for BLEND CLUT memory.
    #[inline(always)]
    pub fn blend0_clut_mem_clk_ena(&self) -> bool {
        (self.value.read_bits() & Self::BLEND0_CLUT_MEM_CLK_ENA_MASK) != 0
    }

    /// Set BLEND0_CLUT_MEM_CLK_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend0_clut_mem_clk_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::BLEND0_CLUT_MEM_CLK_ENA_MASK);
    }

    /// Clear BLEND0_CLUT_MEM_CLK_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend0_clut_mem_clk_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND0_CLUT_MEM_CLK_ENA_MASK);
    }

    /// **BLEND0_CLUT_MEM_FORCE_PU** `[6]` (RW)
    ///
    /// 1: force power up BLEND CLUT memory.
    #[inline(always)]
    pub fn blend0_clut_mem_force_pu(&self) -> bool {
        (self.value.read_bits() & Self::BLEND0_CLUT_MEM_FORCE_PU_MASK) != 0
    }

    /// Set BLEND0_CLUT_MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend0_clut_mem_force_pu(&self) {
        self.value
            .modify_bits(|r| r | Self::BLEND0_CLUT_MEM_FORCE_PU_MASK);
    }

    /// Clear BLEND0_CLUT_MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend0_clut_mem_force_pu(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND0_CLUT_MEM_FORCE_PU_MASK);
    }

    /// **BLEND0_CLUT_MEM_FORCE_PD** `[5]` (RW)
    ///
    /// 1: force power down BLEND CLUT memory.
    #[inline(always)]
    pub fn blend0_clut_mem_force_pd(&self) -> bool {
        (self.value.read_bits() & Self::BLEND0_CLUT_MEM_FORCE_PD_MASK) != 0
    }

    /// Set BLEND0_CLUT_MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend0_clut_mem_force_pd(&self) {
        self.value
            .modify_bits(|r| r | Self::BLEND0_CLUT_MEM_FORCE_PD_MASK);
    }

    /// Clear BLEND0_CLUT_MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend0_clut_mem_force_pd(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND0_CLUT_MEM_FORCE_PD_MASK);
    }

    /// **BLEND1_CLUT_MEM_RDADDR_RST** `[4]` (RW)
    ///
    /// Write 1 then write 0 to reset the read address of BLEND1 CLUT in fifo mode.
    #[inline(always)]
    pub fn blend1_clut_mem_rdaddr_rst(&self) -> bool {
        (self.value.read_bits() & Self::BLEND1_CLUT_MEM_RDADDR_RST_MASK) != 0
    }

    /// Set BLEND1_CLUT_MEM_RDADDR_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend1_clut_mem_rdaddr_rst(&self) {
        self.value
            .modify_bits(|r| r | Self::BLEND1_CLUT_MEM_RDADDR_RST_MASK);
    }

    /// Clear BLEND1_CLUT_MEM_RDADDR_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend1_clut_mem_rdaddr_rst(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND1_CLUT_MEM_RDADDR_RST_MASK);
    }

    /// **BLEND0_CLUT_MEM_RDADDR_RST** `[3]` (RW)
    ///
    /// Write 1 then write 0 to reset the read address of BLEND0 CLUT in fifo mode.
    #[inline(always)]
    pub fn blend0_clut_mem_rdaddr_rst(&self) -> bool {
        (self.value.read_bits() & Self::BLEND0_CLUT_MEM_RDADDR_RST_MASK) != 0
    }

    /// Set BLEND0_CLUT_MEM_RDADDR_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend0_clut_mem_rdaddr_rst(&self) {
        self.value
            .modify_bits(|r| r | Self::BLEND0_CLUT_MEM_RDADDR_RST_MASK);
    }

    /// Clear BLEND0_CLUT_MEM_RDADDR_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend0_clut_mem_rdaddr_rst(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND0_CLUT_MEM_RDADDR_RST_MASK);
    }

    /// **BLEND1_CLUT_MEM_RST** `[2]` (RW)
    ///
    /// Write 1 then write 0 to this bit to reset BLEND1 CLUT.
    #[inline(always)]
    pub fn blend1_clut_mem_rst(&self) -> bool {
        (self.value.read_bits() & Self::BLEND1_CLUT_MEM_RST_MASK) != 0
    }

    /// Set BLEND1_CLUT_MEM_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend1_clut_mem_rst(&self) {
        self.value
            .modify_bits(|r| r | Self::BLEND1_CLUT_MEM_RST_MASK);
    }

    /// Clear BLEND1_CLUT_MEM_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend1_clut_mem_rst(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND1_CLUT_MEM_RST_MASK);
    }

    /// **BLEND0_CLUT_MEM_RST** `[1]` (RW)
    ///
    /// Write 1 then write 0 to this bit to reset BLEND0 CLUT.
    #[inline(always)]
    pub fn blend0_clut_mem_rst(&self) -> bool {
        (self.value.read_bits() & Self::BLEND0_CLUT_MEM_RST_MASK) != 0
    }

    /// Set BLEND0_CLUT_MEM_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend0_clut_mem_rst(&self) {
        self.value
            .modify_bits(|r| r | Self::BLEND0_CLUT_MEM_RST_MASK);
    }

    /// Clear BLEND0_CLUT_MEM_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend0_clut_mem_rst(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND0_CLUT_MEM_RST_MASK);
    }

    /// **APB_FIFO_MASK** `[0]` (RW)
    ///
    /// 1'b0: fifo mode to wr/rd clut0/clut1 RAM through register PPA_SR_CLUT_DATA_REG/PPA_BLEND0_CLUT_DATA_REG/PPA_BLEND1_CLUT_DATA_REG. 1'b1: memory mode to wr/rd sr/blend0/blend1 clut RAM. The bit 11 and 10 of the waddr should be 01 to access sr clut and should be 10 to access blend0 clut and should be 11 to access blend 1 clut in memory mode.
    #[inline(always)]
    pub fn apb_fifo_mask(&self) -> bool {
        (self.value.read_bits() & Self::APB_FIFO_MASK_MASK) != 0
    }

    /// Set APB_FIFO_MASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_apb_fifo_mask(&self) {
        self.value.modify_bits(|r| r | Self::APB_FIFO_MASK_MASK);
    }

    /// Clear APB_FIFO_MASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_apb_fifo_mask(&self) {
        self.value.modify_bits(|r| r & !Self::APB_FIFO_MASK_MASK);
    }
}

/// CLUT_CONF value type for batch read/modify/write operations
///
/// Use `clutconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clutconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClutConfVal {
    bits: u32,
}

impl ClutConfVal {
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

    /// Read **BLEND0_CLUT_MEM_CLK_ENA** `[7:7]`
    ///
    /// 1: Force clock on for BLEND CLUT memory.
    #[inline(always)]
    pub const fn blend0_clut_mem_clk_ena(&self) -> bool {
        (self.bits & ClutConf::BLEND0_CLUT_MEM_CLK_ENA_MASK) != 0
    }

    /// Read **BLEND0_CLUT_MEM_FORCE_PU** `[6:6]`
    ///
    /// 1: force power up BLEND CLUT memory.
    #[inline(always)]
    pub const fn blend0_clut_mem_force_pu(&self) -> bool {
        (self.bits & ClutConf::BLEND0_CLUT_MEM_FORCE_PU_MASK) != 0
    }

    /// Read **BLEND0_CLUT_MEM_FORCE_PD** `[5:5]`
    ///
    /// 1: force power down BLEND CLUT memory.
    #[inline(always)]
    pub const fn blend0_clut_mem_force_pd(&self) -> bool {
        (self.bits & ClutConf::BLEND0_CLUT_MEM_FORCE_PD_MASK) != 0
    }

    /// Read **BLEND1_CLUT_MEM_RDADDR_RST** `[4:4]`
    ///
    /// Write 1 then write 0 to reset the read address of BLEND1 CLUT in fifo mode.
    #[inline(always)]
    pub const fn blend1_clut_mem_rdaddr_rst(&self) -> bool {
        (self.bits & ClutConf::BLEND1_CLUT_MEM_RDADDR_RST_MASK) != 0
    }

    /// Read **BLEND0_CLUT_MEM_RDADDR_RST** `[3:3]`
    ///
    /// Write 1 then write 0 to reset the read address of BLEND0 CLUT in fifo mode.
    #[inline(always)]
    pub const fn blend0_clut_mem_rdaddr_rst(&self) -> bool {
        (self.bits & ClutConf::BLEND0_CLUT_MEM_RDADDR_RST_MASK) != 0
    }

    /// Read **BLEND1_CLUT_MEM_RST** `[2:2]`
    ///
    /// Write 1 then write 0 to this bit to reset BLEND1 CLUT.
    #[inline(always)]
    pub const fn blend1_clut_mem_rst(&self) -> bool {
        (self.bits & ClutConf::BLEND1_CLUT_MEM_RST_MASK) != 0
    }

    /// Read **BLEND0_CLUT_MEM_RST** `[1:1]`
    ///
    /// Write 1 then write 0 to this bit to reset BLEND0 CLUT.
    #[inline(always)]
    pub const fn blend0_clut_mem_rst(&self) -> bool {
        (self.bits & ClutConf::BLEND0_CLUT_MEM_RST_MASK) != 0
    }

    /// Read **APB_FIFO_MASK** `[0:0]`
    ///
    /// 1'b0: fifo mode to wr/rd clut0/clut1 RAM through register PPA_SR_CLUT_DATA_REG/PPA_BLEND0_CLUT_DATA_REG/PPA_BLEND1_CLUT_DATA_REG. 1'b1: memory mode to wr/rd sr/blend0/blend1 clut RAM. The bit 11 and 10 of the waddr should be 01 to access sr clut and should be 10 to access blend0 clut and should be 11 to access blend 1 clut in memory mode.
    #[inline(always)]
    pub const fn apb_fifo_mask(&self) -> bool {
        (self.bits & ClutConf::APB_FIFO_MASK_MASK) != 0
    }

    /// Set **BLEND0_CLUT_MEM_CLK_ENA** `[7:7]`
    ///
    /// 1: Force clock on for BLEND CLUT memory.
    #[inline(always)]
    pub const fn with_blend0_clut_mem_clk_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClutConf::BLEND0_CLUT_MEM_CLK_ENA_MASK
            } else {
                self.bits & !ClutConf::BLEND0_CLUT_MEM_CLK_ENA_MASK
            },
        }
    }

    /// Set **BLEND0_CLUT_MEM_FORCE_PU** `[6:6]`
    ///
    /// 1: force power up BLEND CLUT memory.
    #[inline(always)]
    pub const fn with_blend0_clut_mem_force_pu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClutConf::BLEND0_CLUT_MEM_FORCE_PU_MASK
            } else {
                self.bits & !ClutConf::BLEND0_CLUT_MEM_FORCE_PU_MASK
            },
        }
    }

    /// Set **BLEND0_CLUT_MEM_FORCE_PD** `[5:5]`
    ///
    /// 1: force power down BLEND CLUT memory.
    #[inline(always)]
    pub const fn with_blend0_clut_mem_force_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClutConf::BLEND0_CLUT_MEM_FORCE_PD_MASK
            } else {
                self.bits & !ClutConf::BLEND0_CLUT_MEM_FORCE_PD_MASK
            },
        }
    }

    /// Set **BLEND1_CLUT_MEM_RDADDR_RST** `[4:4]`
    ///
    /// Write 1 then write 0 to reset the read address of BLEND1 CLUT in fifo mode.
    #[inline(always)]
    pub const fn with_blend1_clut_mem_rdaddr_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClutConf::BLEND1_CLUT_MEM_RDADDR_RST_MASK
            } else {
                self.bits & !ClutConf::BLEND1_CLUT_MEM_RDADDR_RST_MASK
            },
        }
    }

    /// Set **BLEND0_CLUT_MEM_RDADDR_RST** `[3:3]`
    ///
    /// Write 1 then write 0 to reset the read address of BLEND0 CLUT in fifo mode.
    #[inline(always)]
    pub const fn with_blend0_clut_mem_rdaddr_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClutConf::BLEND0_CLUT_MEM_RDADDR_RST_MASK
            } else {
                self.bits & !ClutConf::BLEND0_CLUT_MEM_RDADDR_RST_MASK
            },
        }
    }

    /// Set **BLEND1_CLUT_MEM_RST** `[2:2]`
    ///
    /// Write 1 then write 0 to this bit to reset BLEND1 CLUT.
    #[inline(always)]
    pub const fn with_blend1_clut_mem_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClutConf::BLEND1_CLUT_MEM_RST_MASK
            } else {
                self.bits & !ClutConf::BLEND1_CLUT_MEM_RST_MASK
            },
        }
    }

    /// Set **BLEND0_CLUT_MEM_RST** `[1:1]`
    ///
    /// Write 1 then write 0 to this bit to reset BLEND0 CLUT.
    #[inline(always)]
    pub const fn with_blend0_clut_mem_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClutConf::BLEND0_CLUT_MEM_RST_MASK
            } else {
                self.bits & !ClutConf::BLEND0_CLUT_MEM_RST_MASK
            },
        }
    }

    /// Set **APB_FIFO_MASK** `[0:0]`
    ///
    /// 1'b0: fifo mode to wr/rd clut0/clut1 RAM through register PPA_SR_CLUT_DATA_REG/PPA_BLEND0_CLUT_DATA_REG/PPA_BLEND1_CLUT_DATA_REG. 1'b1: memory mode to wr/rd sr/blend0/blend1 clut RAM. The bit 11 and 10 of the waddr should be 01 to access sr clut and should be 10 to access blend0 clut and should be 11 to access blend 1 clut in memory mode.
    #[inline(always)]
    pub const fn with_apb_fifo_mask(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClutConf::APB_FIFO_MASK_MASK
            } else {
                self.bits & !ClutConf::APB_FIFO_MASK_MASK
            },
        }
    }
}

impl Default for ClutConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ClutConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClutConfVal {
        ClutConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClutConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClutConfVal) -> ClutConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_RAW Register
///
/// Raw status interrupt
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **SR_PARAM_CFG_ERR_INT_RAW** `[2]` (RW)
///
///   The raw interrupt bit turns to high level when the configured scaling and rotating coefficient is wrong. User can check the reasons through register PPA_SR_PARAM_ERR_ST_REG.
///
/// - **BLEND_EOF_INT_RAW** `[1]` (RW)
///
///   The raw interrupt bit turns to high level when blending engine calculate one frame image.
///
/// - **SR_EOF_INT_RAW** `[0]` (RW)
///
///   The raw interrupt bit turns to high level when scaling and rotating engine calculate one frame image.
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

    /// **SR_PARAM_CFG_ERR_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when the configured scaling and rotating coefficient is wrong. User can check the reasons through register PPA_SR_PARAM_ERR_ST_REG.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SR_PARAM_CFG_ERR_INT_RAW_OFFSET: u32 = 2;
    pub const SR_PARAM_CFG_ERR_INT_RAW_MASK: u32 = 0x00000004;
    pub const SR_PARAM_CFG_ERR_INT_RAW: u32 = 1 << 2;

    /// **BLEND_EOF_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when blending engine calculate one frame image.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BLEND_EOF_INT_RAW_OFFSET: u32 = 1;
    pub const BLEND_EOF_INT_RAW_MASK: u32 = 0x00000002;
    pub const BLEND_EOF_INT_RAW: u32 = 1 << 1;

    /// **SR_EOF_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when scaling and rotating engine calculate one frame image.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SR_EOF_INT_RAW_OFFSET: u32 = 0;
    pub const SR_EOF_INT_RAW_MASK: u32 = 0x00000001;
    pub const SR_EOF_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SR_PARAM_CFG_ERR_INT_RAW** `[2]` (RW)
    ///
    /// The raw interrupt bit turns to high level when the configured scaling and rotating coefficient is wrong. User can check the reasons through register PPA_SR_PARAM_ERR_ST_REG.
    #[inline(always)]
    pub fn sr_param_cfg_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SR_PARAM_CFG_ERR_INT_RAW_MASK) != 0
    }

    /// Set SR_PARAM_CFG_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_param_cfg_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SR_PARAM_CFG_ERR_INT_RAW_MASK);
    }

    /// Clear SR_PARAM_CFG_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_param_cfg_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SR_PARAM_CFG_ERR_INT_RAW_MASK);
    }

    /// **BLEND_EOF_INT_RAW** `[1]` (RW)
    ///
    /// The raw interrupt bit turns to high level when blending engine calculate one frame image.
    #[inline(always)]
    pub fn blend_eof_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::BLEND_EOF_INT_RAW_MASK) != 0
    }

    /// Set BLEND_EOF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend_eof_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::BLEND_EOF_INT_RAW_MASK);
    }

    /// Clear BLEND_EOF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend_eof_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND_EOF_INT_RAW_MASK);
    }

    /// **SR_EOF_INT_RAW** `[0]` (RW)
    ///
    /// The raw interrupt bit turns to high level when scaling and rotating engine calculate one frame image.
    #[inline(always)]
    pub fn sr_eof_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SR_EOF_INT_RAW_MASK) != 0
    }

    /// Set SR_EOF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_eof_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::SR_EOF_INT_RAW_MASK);
    }

    /// Clear SR_EOF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_eof_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::SR_EOF_INT_RAW_MASK);
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

    /// Read **SR_PARAM_CFG_ERR_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when the configured scaling and rotating coefficient is wrong. User can check the reasons through register PPA_SR_PARAM_ERR_ST_REG.
    #[inline(always)]
    pub const fn sr_param_cfg_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::SR_PARAM_CFG_ERR_INT_RAW_MASK) != 0
    }

    /// Read **BLEND_EOF_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when blending engine calculate one frame image.
    #[inline(always)]
    pub const fn blend_eof_int_raw(&self) -> bool {
        (self.bits & IntRaw::BLEND_EOF_INT_RAW_MASK) != 0
    }

    /// Read **SR_EOF_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when scaling and rotating engine calculate one frame image.
    #[inline(always)]
    pub const fn sr_eof_int_raw(&self) -> bool {
        (self.bits & IntRaw::SR_EOF_INT_RAW_MASK) != 0
    }

    /// Set **SR_PARAM_CFG_ERR_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit turns to high level when the configured scaling and rotating coefficient is wrong. User can check the reasons through register PPA_SR_PARAM_ERR_ST_REG.
    #[inline(always)]
    pub const fn with_sr_param_cfg_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SR_PARAM_CFG_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SR_PARAM_CFG_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **BLEND_EOF_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit turns to high level when blending engine calculate one frame image.
    #[inline(always)]
    pub const fn with_blend_eof_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::BLEND_EOF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::BLEND_EOF_INT_RAW_MASK
            },
        }
    }

    /// Set **SR_EOF_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt bit turns to high level when scaling and rotating engine calculate one frame image.
    #[inline(always)]
    pub const fn with_sr_eof_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SR_EOF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SR_EOF_INT_RAW_MASK
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

/// INT_ST Register
///
/// Masked interrupt
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **SR_PARAM_CFG_ERR_INT_ST** `[2]` (RO)
///
///   The raw interrupt status bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt.
///
/// - **BLEND_EOF_INT_ST** `[1]` (RO)
///
///   The raw interrupt status bit for the PPA_BLEND_EOF_INT interrupt.
///
/// - **SR_EOF_INT_ST** `[0]` (RO)
///
///   The raw interrupt status bit for the PPA_SR_EOF_INT interrupt.
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

    /// **SR_PARAM_CFG_ERR_INT_ST**
    ///
    /// The raw interrupt status bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SR_PARAM_CFG_ERR_INT_ST_OFFSET: u32 = 2;
    pub const SR_PARAM_CFG_ERR_INT_ST_MASK: u32 = 0x00000004;
    pub const SR_PARAM_CFG_ERR_INT_ST: u32 = 1 << 2;

    /// **BLEND_EOF_INT_ST**
    ///
    /// The raw interrupt status bit for the PPA_BLEND_EOF_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BLEND_EOF_INT_ST_OFFSET: u32 = 1;
    pub const BLEND_EOF_INT_ST_MASK: u32 = 0x00000002;
    pub const BLEND_EOF_INT_ST: u32 = 1 << 1;

    /// **SR_EOF_INT_ST**
    ///
    /// The raw interrupt status bit for the PPA_SR_EOF_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SR_EOF_INT_ST_OFFSET: u32 = 0;
    pub const SR_EOF_INT_ST_MASK: u32 = 0x00000001;
    pub const SR_EOF_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SR_PARAM_CFG_ERR_INT_ST** `[2]` (RO)
    ///
    /// The raw interrupt status bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt.
    #[inline(always)]
    pub fn sr_param_cfg_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SR_PARAM_CFG_ERR_INT_ST_MASK) != 0
    }

    /// **BLEND_EOF_INT_ST** `[1]` (RO)
    ///
    /// The raw interrupt status bit for the PPA_BLEND_EOF_INT interrupt.
    #[inline(always)]
    pub fn blend_eof_int_st(&self) -> bool {
        (self.value.read_bits() & Self::BLEND_EOF_INT_ST_MASK) != 0
    }

    /// **SR_EOF_INT_ST** `[0]` (RO)
    ///
    /// The raw interrupt status bit for the PPA_SR_EOF_INT interrupt.
    #[inline(always)]
    pub fn sr_eof_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SR_EOF_INT_ST_MASK) != 0
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

    /// Read **SR_PARAM_CFG_ERR_INT_ST** `[2:2]`
    ///
    /// The raw interrupt status bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt.
    #[inline(always)]
    pub const fn sr_param_cfg_err_int_st(&self) -> bool {
        (self.bits & IntSt::SR_PARAM_CFG_ERR_INT_ST_MASK) != 0
    }

    /// Read **BLEND_EOF_INT_ST** `[1:1]`
    ///
    /// The raw interrupt status bit for the PPA_BLEND_EOF_INT interrupt.
    #[inline(always)]
    pub const fn blend_eof_int_st(&self) -> bool {
        (self.bits & IntSt::BLEND_EOF_INT_ST_MASK) != 0
    }

    /// Read **SR_EOF_INT_ST** `[0:0]`
    ///
    /// The raw interrupt status bit for the PPA_SR_EOF_INT interrupt.
    #[inline(always)]
    pub const fn sr_eof_int_st(&self) -> bool {
        (self.bits & IntSt::SR_EOF_INT_ST_MASK) != 0
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

/// INT_ENA Register
///
/// Interrupt enable bits
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **SR_PARAM_CFG_ERR_INT_ENA** `[2]` (RW)
///
///   The interrupt enable bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt.
///
/// - **BLEND_EOF_INT_ENA** `[1]` (RW)
///
///   The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt.
///
/// - **SR_EOF_INT_ENA** `[0]` (RW)
///
///   The interrupt enable bit for the PPA_SR_EOF_INT interrupt.
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

    /// **SR_PARAM_CFG_ERR_INT_ENA**
    ///
    /// The interrupt enable bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SR_PARAM_CFG_ERR_INT_ENA_OFFSET: u32 = 2;
    pub const SR_PARAM_CFG_ERR_INT_ENA_MASK: u32 = 0x00000004;
    pub const SR_PARAM_CFG_ERR_INT_ENA: u32 = 1 << 2;

    /// **BLEND_EOF_INT_ENA**
    ///
    /// The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BLEND_EOF_INT_ENA_OFFSET: u32 = 1;
    pub const BLEND_EOF_INT_ENA_MASK: u32 = 0x00000002;
    pub const BLEND_EOF_INT_ENA: u32 = 1 << 1;

    /// **SR_EOF_INT_ENA**
    ///
    /// The interrupt enable bit for the PPA_SR_EOF_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SR_EOF_INT_ENA_OFFSET: u32 = 0;
    pub const SR_EOF_INT_ENA_MASK: u32 = 0x00000001;
    pub const SR_EOF_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SR_PARAM_CFG_ERR_INT_ENA** `[2]` (RW)
    ///
    /// The interrupt enable bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt.
    #[inline(always)]
    pub fn sr_param_cfg_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SR_PARAM_CFG_ERR_INT_ENA_MASK) != 0
    }

    /// Set SR_PARAM_CFG_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_param_cfg_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SR_PARAM_CFG_ERR_INT_ENA_MASK);
    }

    /// Clear SR_PARAM_CFG_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_param_cfg_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SR_PARAM_CFG_ERR_INT_ENA_MASK);
    }

    /// **BLEND_EOF_INT_ENA** `[1]` (RW)
    ///
    /// The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt.
    #[inline(always)]
    pub fn blend_eof_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::BLEND_EOF_INT_ENA_MASK) != 0
    }

    /// Set BLEND_EOF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend_eof_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::BLEND_EOF_INT_ENA_MASK);
    }

    /// Clear BLEND_EOF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend_eof_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND_EOF_INT_ENA_MASK);
    }

    /// **SR_EOF_INT_ENA** `[0]` (RW)
    ///
    /// The interrupt enable bit for the PPA_SR_EOF_INT interrupt.
    #[inline(always)]
    pub fn sr_eof_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SR_EOF_INT_ENA_MASK) != 0
    }

    /// Set SR_EOF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_eof_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::SR_EOF_INT_ENA_MASK);
    }

    /// Clear SR_EOF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_eof_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::SR_EOF_INT_ENA_MASK);
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

    /// Read **SR_PARAM_CFG_ERR_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt.
    #[inline(always)]
    pub const fn sr_param_cfg_err_int_ena(&self) -> bool {
        (self.bits & IntEna::SR_PARAM_CFG_ERR_INT_ENA_MASK) != 0
    }

    /// Read **BLEND_EOF_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt.
    #[inline(always)]
    pub const fn blend_eof_int_ena(&self) -> bool {
        (self.bits & IntEna::BLEND_EOF_INT_ENA_MASK) != 0
    }

    /// Read **SR_EOF_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the PPA_SR_EOF_INT interrupt.
    #[inline(always)]
    pub const fn sr_eof_int_ena(&self) -> bool {
        (self.bits & IntEna::SR_EOF_INT_ENA_MASK) != 0
    }

    /// Set **SR_PARAM_CFG_ERR_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_sr_param_cfg_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SR_PARAM_CFG_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SR_PARAM_CFG_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **BLEND_EOF_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt.
    #[inline(always)]
    pub const fn with_blend_eof_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::BLEND_EOF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::BLEND_EOF_INT_ENA_MASK
            },
        }
    }

    /// Set **SR_EOF_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the PPA_SR_EOF_INT interrupt.
    #[inline(always)]
    pub const fn with_sr_eof_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SR_EOF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SR_EOF_INT_ENA_MASK
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

/// INT_CLR Register
///
/// Interrupt clear bits
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **SR_PARAM_CFG_ERR_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear the PPA_SR_RX_YSCAL_ERR_INT interrupt.
///
/// - **BLEND_EOF_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear the PPA_BLEND_EOF_INT interrupt.
///
/// - **SR_EOF_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the PPA_SR_EOF_INT interrupt.
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

    /// **SR_PARAM_CFG_ERR_INT_CLR**
    ///
    /// Set this bit to clear the PPA_SR_RX_YSCAL_ERR_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SR_PARAM_CFG_ERR_INT_CLR_OFFSET: u32 = 2;
    pub const SR_PARAM_CFG_ERR_INT_CLR_MASK: u32 = 0x00000004;
    pub const SR_PARAM_CFG_ERR_INT_CLR: u32 = 1 << 2;

    /// **BLEND_EOF_INT_CLR**
    ///
    /// Set this bit to clear the PPA_BLEND_EOF_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BLEND_EOF_INT_CLR_OFFSET: u32 = 1;
    pub const BLEND_EOF_INT_CLR_MASK: u32 = 0x00000002;
    pub const BLEND_EOF_INT_CLR: u32 = 1 << 1;

    /// **SR_EOF_INT_CLR**
    ///
    /// Set this bit to clear the PPA_SR_EOF_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SR_EOF_INT_CLR_OFFSET: u32 = 0;
    pub const SR_EOF_INT_CLR_MASK: u32 = 0x00000001;
    pub const SR_EOF_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SR_PARAM_CFG_ERR_INT_CLR** (WO)
    ///
    /// Set this bit to clear the PPA_SR_RX_YSCAL_ERR_INT interrupt.
    #[inline(always)]
    pub fn trigger_sr_param_cfg_err_int_clr(&self) {
        self.value.write_bits(Self::SR_PARAM_CFG_ERR_INT_CLR_MASK);
    }

    /// Trigger **BLEND_EOF_INT_CLR** (WO)
    ///
    /// Set this bit to clear the PPA_BLEND_EOF_INT interrupt.
    #[inline(always)]
    pub fn trigger_blend_eof_int_clr(&self) {
        self.value.write_bits(Self::BLEND_EOF_INT_CLR_MASK);
    }

    /// Trigger **SR_EOF_INT_CLR** (WO)
    ///
    /// Set this bit to clear the PPA_SR_EOF_INT interrupt.
    #[inline(always)]
    pub fn trigger_sr_eof_int_clr(&self) {
        self.value.write_bits(Self::SR_EOF_INT_CLR_MASK);
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

    /// Set **SR_PARAM_CFG_ERR_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear the PPA_SR_RX_YSCAL_ERR_INT interrupt.
    #[inline(always)]
    pub const fn with_sr_param_cfg_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SR_PARAM_CFG_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SR_PARAM_CFG_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **BLEND_EOF_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear the PPA_BLEND_EOF_INT interrupt.
    #[inline(always)]
    pub const fn with_blend_eof_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::BLEND_EOF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::BLEND_EOF_INT_CLR_MASK
            },
        }
    }

    /// Set **SR_EOF_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the PPA_SR_EOF_INT interrupt.
    #[inline(always)]
    pub const fn with_sr_eof_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SR_EOF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SR_EOF_INT_CLR_MASK
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

/// SR_COLOR_MODE Register
///
/// Scaling and rotating engine color mode register
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **RGB2YUV_PROTOCAL** `[11]` (RW)
///
///   RGB to YUV protocal when reg_sr_tx_cm is 4'd8. 0: BT601. 1: BT709
///
/// - **YUV2RGB_PROTOCAL** `[10]` (RW)
///
///   YUV to RGB protocal when reg_sr_rx_cm is 4'd8. 0: BT601. 1: BT709
///
/// - **YUV_TX_RANGE** `[9]` (RW)
///
///   YUV output range when reg_sr_tx_cm is 4'd8. 0: limit range. 1: full range
///
/// - **YUV_RX_RANGE** `[8]` (RW)
///
///   YUV input range when reg_sr_rx_cm is 4'd8. 0: limit range. 1: full range
///
/// - **SR_TX_CM** `[7:4]` (RW)
///
///   The destination image color mode for Scaling and Rotating engine Tx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved.
///
/// - **SR_RX_CM** `[3:0]` (RW)
///
///   The source image color mode for Scaling and Rotating engine Rx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved.
#[repr(transparent)]
pub struct SrColorMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SrColorMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SrColorMode {
    // ======== Field Constants ========

    /// **RGB2YUV_PROTOCAL**
    ///
    /// RGB to YUV protocal when reg_sr_tx_cm is 4'd8. 0: BT601. 1: BT709
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const RGB2YUV_PROTOCAL_OFFSET: u32 = 11;
    pub const RGB2YUV_PROTOCAL_MASK: u32 = 0x00000800;
    pub const RGB2YUV_PROTOCAL: u32 = 1 << 11;

    /// **YUV2RGB_PROTOCAL**
    ///
    /// YUV to RGB protocal when reg_sr_rx_cm is 4'd8. 0: BT601. 1: BT709
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const YUV2RGB_PROTOCAL_OFFSET: u32 = 10;
    pub const YUV2RGB_PROTOCAL_MASK: u32 = 0x00000400;
    pub const YUV2RGB_PROTOCAL: u32 = 1 << 10;

    /// **YUV_TX_RANGE**
    ///
    /// YUV output range when reg_sr_tx_cm is 4'd8. 0: limit range. 1: full range
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const YUV_TX_RANGE_OFFSET: u32 = 9;
    pub const YUV_TX_RANGE_MASK: u32 = 0x00000200;
    pub const YUV_TX_RANGE: u32 = 1 << 9;

    /// **YUV_RX_RANGE**
    ///
    /// YUV input range when reg_sr_rx_cm is 4'd8. 0: limit range. 1: full range
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const YUV_RX_RANGE_OFFSET: u32 = 8;
    pub const YUV_RX_RANGE_MASK: u32 = 0x00000100;
    pub const YUV_RX_RANGE: u32 = 1 << 8;

    /// **SR_TX_CM**
    ///
    /// The destination image color mode for Scaling and Rotating engine Tx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved.
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const SR_TX_CM_OFFSET: u32 = 4;
    pub const SR_TX_CM_MASK: u32 = 0x000000f0;

    /// **SR_RX_CM**
    ///
    /// The source image color mode for Scaling and Rotating engine Rx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const SR_RX_CM_OFFSET: u32 = 0;
    pub const SR_RX_CM_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **RGB2YUV_PROTOCAL** `[11]` (RW)
    ///
    /// RGB to YUV protocal when reg_sr_tx_cm is 4'd8. 0: BT601. 1: BT709
    #[inline(always)]
    pub fn rgb2yuv_protocal(&self) -> bool {
        (self.value.read_bits() & Self::RGB2YUV_PROTOCAL_MASK) != 0
    }

    /// Set RGB2YUV_PROTOCAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rgb2yuv_protocal(&self) {
        self.value.modify_bits(|r| r | Self::RGB2YUV_PROTOCAL_MASK);
    }

    /// Clear RGB2YUV_PROTOCAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rgb2yuv_protocal(&self) {
        self.value.modify_bits(|r| r & !Self::RGB2YUV_PROTOCAL_MASK);
    }

    /// **YUV2RGB_PROTOCAL** `[10]` (RW)
    ///
    /// YUV to RGB protocal when reg_sr_rx_cm is 4'd8. 0: BT601. 1: BT709
    #[inline(always)]
    pub fn yuv2rgb_protocal(&self) -> bool {
        (self.value.read_bits() & Self::YUV2RGB_PROTOCAL_MASK) != 0
    }

    /// Set YUV2RGB_PROTOCAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_yuv2rgb_protocal(&self) {
        self.value.modify_bits(|r| r | Self::YUV2RGB_PROTOCAL_MASK);
    }

    /// Clear YUV2RGB_PROTOCAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_yuv2rgb_protocal(&self) {
        self.value.modify_bits(|r| r & !Self::YUV2RGB_PROTOCAL_MASK);
    }

    /// **YUV_TX_RANGE** `[9]` (RW)
    ///
    /// YUV output range when reg_sr_tx_cm is 4'd8. 0: limit range. 1: full range
    #[inline(always)]
    pub fn yuv_tx_range(&self) -> bool {
        (self.value.read_bits() & Self::YUV_TX_RANGE_MASK) != 0
    }

    /// Set YUV_TX_RANGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_yuv_tx_range(&self) {
        self.value.modify_bits(|r| r | Self::YUV_TX_RANGE_MASK);
    }

    /// Clear YUV_TX_RANGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_yuv_tx_range(&self) {
        self.value.modify_bits(|r| r & !Self::YUV_TX_RANGE_MASK);
    }

    /// **YUV_RX_RANGE** `[8]` (RW)
    ///
    /// YUV input range when reg_sr_rx_cm is 4'd8. 0: limit range. 1: full range
    #[inline(always)]
    pub fn yuv_rx_range(&self) -> bool {
        (self.value.read_bits() & Self::YUV_RX_RANGE_MASK) != 0
    }

    /// Set YUV_RX_RANGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_yuv_rx_range(&self) {
        self.value.modify_bits(|r| r | Self::YUV_RX_RANGE_MASK);
    }

    /// Clear YUV_RX_RANGE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_yuv_rx_range(&self) {
        self.value.modify_bits(|r| r & !Self::YUV_RX_RANGE_MASK);
    }

    /// **SR_TX_CM** `[7:4]` (RW)
    ///
    /// The destination image color mode for Scaling and Rotating engine Tx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved.
    #[inline(always)]
    pub fn sr_tx_cm(&self) -> u32 {
        (self.value.read_bits() & Self::SR_TX_CM_MASK) >> Self::SR_TX_CM_OFFSET
    }

    /// Write **SR_TX_CM** `[7:4]` (RW)
    ///
    /// The destination image color mode for Scaling and Rotating engine Tx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved.
    #[inline(always)]
    pub fn write_sr_tx_cm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SR_TX_CM_MASK) | ((value << Self::SR_TX_CM_OFFSET) & Self::SR_TX_CM_MASK)
        });
    }

    /// **SR_RX_CM** `[3:0]` (RW)
    ///
    /// The source image color mode for Scaling and Rotating engine Rx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved.
    #[inline(always)]
    pub fn sr_rx_cm(&self) -> u32 {
        (self.value.read_bits() & Self::SR_RX_CM_MASK) >> Self::SR_RX_CM_OFFSET
    }

    /// Write **SR_RX_CM** `[3:0]` (RW)
    ///
    /// The source image color mode for Scaling and Rotating engine Rx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved.
    #[inline(always)]
    pub fn write_sr_rx_cm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SR_RX_CM_MASK) | ((value << Self::SR_RX_CM_OFFSET) & Self::SR_RX_CM_MASK)
        });
    }
}

/// SR_COLOR_MODE value type for batch read/modify/write operations
///
/// Use `srcolormode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `srcolormode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SrColorModeVal {
    bits: u32,
}

impl SrColorModeVal {
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

    /// Read **RGB2YUV_PROTOCAL** `[11:11]`
    ///
    /// RGB to YUV protocal when reg_sr_tx_cm is 4'd8. 0: BT601. 1: BT709
    #[inline(always)]
    pub const fn rgb2yuv_protocal(&self) -> bool {
        (self.bits & SrColorMode::RGB2YUV_PROTOCAL_MASK) != 0
    }

    /// Read **YUV2RGB_PROTOCAL** `[10:10]`
    ///
    /// YUV to RGB protocal when reg_sr_rx_cm is 4'd8. 0: BT601. 1: BT709
    #[inline(always)]
    pub const fn yuv2rgb_protocal(&self) -> bool {
        (self.bits & SrColorMode::YUV2RGB_PROTOCAL_MASK) != 0
    }

    /// Read **YUV_TX_RANGE** `[9:9]`
    ///
    /// YUV output range when reg_sr_tx_cm is 4'd8. 0: limit range. 1: full range
    #[inline(always)]
    pub const fn yuv_tx_range(&self) -> bool {
        (self.bits & SrColorMode::YUV_TX_RANGE_MASK) != 0
    }

    /// Read **YUV_RX_RANGE** `[8:8]`
    ///
    /// YUV input range when reg_sr_rx_cm is 4'd8. 0: limit range. 1: full range
    #[inline(always)]
    pub const fn yuv_rx_range(&self) -> bool {
        (self.bits & SrColorMode::YUV_RX_RANGE_MASK) != 0
    }

    /// Read **SR_TX_CM** `[7:4]`
    ///
    /// The destination image color mode for Scaling and Rotating engine Tx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved.
    #[inline(always)]
    pub const fn sr_tx_cm(&self) -> u32 {
        (self.bits & SrColorMode::SR_TX_CM_MASK) >> SrColorMode::SR_TX_CM_OFFSET
    }

    /// Read **SR_RX_CM** `[3:0]`
    ///
    /// The source image color mode for Scaling and Rotating engine Rx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved.
    #[inline(always)]
    pub const fn sr_rx_cm(&self) -> u32 {
        (self.bits & SrColorMode::SR_RX_CM_MASK) >> SrColorMode::SR_RX_CM_OFFSET
    }

    /// Set **RGB2YUV_PROTOCAL** `[11:11]`
    ///
    /// RGB to YUV protocal when reg_sr_tx_cm is 4'd8. 0: BT601. 1: BT709
    #[inline(always)]
    pub const fn with_rgb2yuv_protocal(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrColorMode::RGB2YUV_PROTOCAL_MASK
            } else {
                self.bits & !SrColorMode::RGB2YUV_PROTOCAL_MASK
            },
        }
    }

    /// Set **YUV2RGB_PROTOCAL** `[10:10]`
    ///
    /// YUV to RGB protocal when reg_sr_rx_cm is 4'd8. 0: BT601. 1: BT709
    #[inline(always)]
    pub const fn with_yuv2rgb_protocal(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrColorMode::YUV2RGB_PROTOCAL_MASK
            } else {
                self.bits & !SrColorMode::YUV2RGB_PROTOCAL_MASK
            },
        }
    }

    /// Set **YUV_TX_RANGE** `[9:9]`
    ///
    /// YUV output range when reg_sr_tx_cm is 4'd8. 0: limit range. 1: full range
    #[inline(always)]
    pub const fn with_yuv_tx_range(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrColorMode::YUV_TX_RANGE_MASK
            } else {
                self.bits & !SrColorMode::YUV_TX_RANGE_MASK
            },
        }
    }

    /// Set **YUV_RX_RANGE** `[8:8]`
    ///
    /// YUV input range when reg_sr_rx_cm is 4'd8. 0: limit range. 1: full range
    #[inline(always)]
    pub const fn with_yuv_rx_range(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrColorMode::YUV_RX_RANGE_MASK
            } else {
                self.bits & !SrColorMode::YUV_RX_RANGE_MASK
            },
        }
    }

    /// Set **SR_TX_CM** `[7:4]`
    ///
    /// The destination image color mode for Scaling and Rotating engine Tx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved.
    #[inline(always)]
    pub const fn with_sr_tx_cm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SrColorMode::SR_TX_CM_MASK)
                | ((value << SrColorMode::SR_TX_CM_OFFSET) & SrColorMode::SR_TX_CM_MASK),
        }
    }

    /// Set **SR_RX_CM** `[3:0]`
    ///
    /// The source image color mode for Scaling and Rotating engine Rx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. others: Reserved.
    #[inline(always)]
    pub const fn with_sr_rx_cm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SrColorMode::SR_RX_CM_MASK)
                | ((value << SrColorMode::SR_RX_CM_OFFSET) & SrColorMode::SR_RX_CM_MASK),
        }
    }
}

impl Default for SrColorModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SrColorMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SrColorModeVal {
        SrColorModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SrColorModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SrColorModeVal) -> SrColorModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BLEND_COLOR_MODE Register
///
/// blending engine color mode register
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **BLEND_TX_CM** `[11:8]` (RW)
///
///   The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved..
///
/// - **BLEND1_RX_CM** `[7:4]` (RW)
///
///   The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4.
///
/// - **BLEND0_RX_CM** `[3:0]` (RW)
///
///   The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4.
#[repr(transparent)]
pub struct BlendColorMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BlendColorMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BlendColorMode {
    // ======== Field Constants ========

    /// **BLEND_TX_CM**
    ///
    /// The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved..
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const BLEND_TX_CM_OFFSET: u32 = 8;
    pub const BLEND_TX_CM_MASK: u32 = 0x00000f00;

    /// **BLEND1_RX_CM**
    ///
    /// The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4.
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const BLEND1_RX_CM_OFFSET: u32 = 4;
    pub const BLEND1_RX_CM_MASK: u32 = 0x000000f0;

    /// **BLEND0_RX_CM**
    ///
    /// The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const BLEND0_RX_CM_OFFSET: u32 = 0;
    pub const BLEND0_RX_CM_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **BLEND_TX_CM** `[11:8]` (RW)
    ///
    /// The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved..
    #[inline(always)]
    pub fn blend_tx_cm(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND_TX_CM_MASK) >> Self::BLEND_TX_CM_OFFSET
    }

    /// Write **BLEND_TX_CM** `[11:8]` (RW)
    ///
    /// The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved..
    #[inline(always)]
    pub fn write_blend_tx_cm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLEND_TX_CM_MASK)
                | ((value << Self::BLEND_TX_CM_OFFSET) & Self::BLEND_TX_CM_MASK)
        });
    }

    /// **BLEND1_RX_CM** `[7:4]` (RW)
    ///
    /// The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4.
    #[inline(always)]
    pub fn blend1_rx_cm(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND1_RX_CM_MASK) >> Self::BLEND1_RX_CM_OFFSET
    }

    /// Write **BLEND1_RX_CM** `[7:4]` (RW)
    ///
    /// The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4.
    #[inline(always)]
    pub fn write_blend1_rx_cm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLEND1_RX_CM_MASK)
                | ((value << Self::BLEND1_RX_CM_OFFSET) & Self::BLEND1_RX_CM_MASK)
        });
    }

    /// **BLEND0_RX_CM** `[3:0]` (RW)
    ///
    /// The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4.
    #[inline(always)]
    pub fn blend0_rx_cm(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND0_RX_CM_MASK) >> Self::BLEND0_RX_CM_OFFSET
    }

    /// Write **BLEND0_RX_CM** `[3:0]` (RW)
    ///
    /// The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4.
    #[inline(always)]
    pub fn write_blend0_rx_cm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLEND0_RX_CM_MASK)
                | ((value << Self::BLEND0_RX_CM_OFFSET) & Self::BLEND0_RX_CM_MASK)
        });
    }
}

/// BLEND_COLOR_MODE value type for batch read/modify/write operations
///
/// Use `blendcolormode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `blendcolormode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BlendColorModeVal {
    bits: u32,
}

impl BlendColorModeVal {
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

    /// Read **BLEND_TX_CM** `[11:8]`
    ///
    /// The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved..
    #[inline(always)]
    pub const fn blend_tx_cm(&self) -> u32 {
        (self.bits & BlendColorMode::BLEND_TX_CM_MASK) >> BlendColorMode::BLEND_TX_CM_OFFSET
    }

    /// Read **BLEND1_RX_CM** `[7:4]`
    ///
    /// The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4.
    #[inline(always)]
    pub const fn blend1_rx_cm(&self) -> u32 {
        (self.bits & BlendColorMode::BLEND1_RX_CM_MASK) >> BlendColorMode::BLEND1_RX_CM_OFFSET
    }

    /// Read **BLEND0_RX_CM** `[3:0]`
    ///
    /// The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4.
    #[inline(always)]
    pub const fn blend0_rx_cm(&self) -> u32 {
        (self.bits & BlendColorMode::BLEND0_RX_CM_MASK) >> BlendColorMode::BLEND0_RX_CM_OFFSET
    }

    /// Set **BLEND_TX_CM** `[11:8]`
    ///
    /// The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved..
    #[inline(always)]
    pub const fn with_blend_tx_cm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlendColorMode::BLEND_TX_CM_MASK)
                | ((value << BlendColorMode::BLEND_TX_CM_OFFSET)
                    & BlendColorMode::BLEND_TX_CM_MASK),
        }
    }

    /// Set **BLEND1_RX_CM** `[7:4]`
    ///
    /// The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4.
    #[inline(always)]
    pub const fn with_blend1_rx_cm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlendColorMode::BLEND1_RX_CM_MASK)
                | ((value << BlendColorMode::BLEND1_RX_CM_OFFSET)
                    & BlendColorMode::BLEND1_RX_CM_MASK),
        }
    }

    /// Set **BLEND0_RX_CM** `[3:0]`
    ///
    /// The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4.
    #[inline(always)]
    pub const fn with_blend0_rx_cm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlendColorMode::BLEND0_RX_CM_MASK)
                | ((value << BlendColorMode::BLEND0_RX_CM_OFFSET)
                    & BlendColorMode::BLEND0_RX_CM_MASK),
        }
    }
}

impl Default for BlendColorModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BlendColorMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BlendColorModeVal {
        BlendColorModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BlendColorModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BlendColorModeVal) -> BlendColorModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SR_BYTE_ORDER Register
///
/// Scaling and rotating engine byte order register
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **SR_MACRO_BK_RO_BYPASS** `[2]` (RW)
///
///   Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory.
///
/// - **SR_RX_RGB_SWAP_EN** `[1]` (RW)
///
///   Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
///
/// - **SR_RX_BYTE_SWAP_EN** `[0]` (RW)
///
///   Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
#[repr(transparent)]
pub struct SrByteOrder {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SrByteOrder {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SrByteOrder {
    // ======== Field Constants ========

    /// **SR_MACRO_BK_RO_BYPASS**
    ///
    /// Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SR_MACRO_BK_RO_BYPASS_OFFSET: u32 = 2;
    pub const SR_MACRO_BK_RO_BYPASS_MASK: u32 = 0x00000004;
    pub const SR_MACRO_BK_RO_BYPASS: u32 = 1 << 2;

    /// **SR_RX_RGB_SWAP_EN**
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SR_RX_RGB_SWAP_EN_OFFSET: u32 = 1;
    pub const SR_RX_RGB_SWAP_EN_MASK: u32 = 0x00000002;
    pub const SR_RX_RGB_SWAP_EN: u32 = 1 << 1;

    /// **SR_RX_BYTE_SWAP_EN**
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SR_RX_BYTE_SWAP_EN_OFFSET: u32 = 0;
    pub const SR_RX_BYTE_SWAP_EN_MASK: u32 = 0x00000001;
    pub const SR_RX_BYTE_SWAP_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SR_MACRO_BK_RO_BYPASS** `[2]` (RW)
    ///
    /// Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory.
    #[inline(always)]
    pub fn sr_macro_bk_ro_bypass(&self) -> bool {
        (self.value.read_bits() & Self::SR_MACRO_BK_RO_BYPASS_MASK) != 0
    }

    /// Set SR_MACRO_BK_RO_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_macro_bk_ro_bypass(&self) {
        self.value
            .modify_bits(|r| r | Self::SR_MACRO_BK_RO_BYPASS_MASK);
    }

    /// Clear SR_MACRO_BK_RO_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_macro_bk_ro_bypass(&self) {
        self.value
            .modify_bits(|r| r & !Self::SR_MACRO_BK_RO_BYPASS_MASK);
    }

    /// **SR_RX_RGB_SWAP_EN** `[1]` (RW)
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    #[inline(always)]
    pub fn sr_rx_rgb_swap_en(&self) -> bool {
        (self.value.read_bits() & Self::SR_RX_RGB_SWAP_EN_MASK) != 0
    }

    /// Set SR_RX_RGB_SWAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_rx_rgb_swap_en(&self) {
        self.value.modify_bits(|r| r | Self::SR_RX_RGB_SWAP_EN_MASK);
    }

    /// Clear SR_RX_RGB_SWAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_rx_rgb_swap_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SR_RX_RGB_SWAP_EN_MASK);
    }

    /// **SR_RX_BYTE_SWAP_EN** `[0]` (RW)
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    #[inline(always)]
    pub fn sr_rx_byte_swap_en(&self) -> bool {
        (self.value.read_bits() & Self::SR_RX_BYTE_SWAP_EN_MASK) != 0
    }

    /// Set SR_RX_BYTE_SWAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_rx_byte_swap_en(&self) {
        self.value
            .modify_bits(|r| r | Self::SR_RX_BYTE_SWAP_EN_MASK);
    }

    /// Clear SR_RX_BYTE_SWAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_rx_byte_swap_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SR_RX_BYTE_SWAP_EN_MASK);
    }
}

/// SR_BYTE_ORDER value type for batch read/modify/write operations
///
/// Use `srbyteorder.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `srbyteorder.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SrByteOrderVal {
    bits: u32,
}

impl SrByteOrderVal {
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

    /// Read **SR_MACRO_BK_RO_BYPASS** `[2:2]`
    ///
    /// Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory.
    #[inline(always)]
    pub const fn sr_macro_bk_ro_bypass(&self) -> bool {
        (self.bits & SrByteOrder::SR_MACRO_BK_RO_BYPASS_MASK) != 0
    }

    /// Read **SR_RX_RGB_SWAP_EN** `[1:1]`
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    #[inline(always)]
    pub const fn sr_rx_rgb_swap_en(&self) -> bool {
        (self.bits & SrByteOrder::SR_RX_RGB_SWAP_EN_MASK) != 0
    }

    /// Read **SR_RX_BYTE_SWAP_EN** `[0:0]`
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    #[inline(always)]
    pub const fn sr_rx_byte_swap_en(&self) -> bool {
        (self.bits & SrByteOrder::SR_RX_BYTE_SWAP_EN_MASK) != 0
    }

    /// Set **SR_MACRO_BK_RO_BYPASS** `[2:2]`
    ///
    /// Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory.
    #[inline(always)]
    pub const fn with_sr_macro_bk_ro_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrByteOrder::SR_MACRO_BK_RO_BYPASS_MASK
            } else {
                self.bits & !SrByteOrder::SR_MACRO_BK_RO_BYPASS_MASK
            },
        }
    }

    /// Set **SR_RX_RGB_SWAP_EN** `[1:1]`
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    #[inline(always)]
    pub const fn with_sr_rx_rgb_swap_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrByteOrder::SR_RX_RGB_SWAP_EN_MASK
            } else {
                self.bits & !SrByteOrder::SR_RX_RGB_SWAP_EN_MASK
            },
        }
    }

    /// Set **SR_RX_BYTE_SWAP_EN** `[0:0]`
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    #[inline(always)]
    pub const fn with_sr_rx_byte_swap_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrByteOrder::SR_RX_BYTE_SWAP_EN_MASK
            } else {
                self.bits & !SrByteOrder::SR_RX_BYTE_SWAP_EN_MASK
            },
        }
    }
}

impl Default for SrByteOrderVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SrByteOrder {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SrByteOrderVal {
        SrByteOrderVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SrByteOrderVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SrByteOrderVal) -> SrByteOrderVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BLEND_BYTE_ORDER Register
///
/// Blending engine byte order register
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **BLEND1_RX_RGB_SWAP_EN** `[3]` (RW)
///
///   Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
///
/// - **BLEND0_RX_RGB_SWAP_EN** `[2]` (RW)
///
///   Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
///
/// - **BLEND1_RX_BYTE_SWAP_EN** `[1]` (RW)
///
///   Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
///
/// - **BLEND0_RX_BYTE_SWAP_EN** `[0]` (RW)
///
///   Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
#[repr(transparent)]
pub struct BlendByteOrder {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BlendByteOrder {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BlendByteOrder {
    // ======== Field Constants ========

    /// **BLEND1_RX_RGB_SWAP_EN**
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const BLEND1_RX_RGB_SWAP_EN_OFFSET: u32 = 3;
    pub const BLEND1_RX_RGB_SWAP_EN_MASK: u32 = 0x00000008;
    pub const BLEND1_RX_RGB_SWAP_EN: u32 = 1 << 3;

    /// **BLEND0_RX_RGB_SWAP_EN**
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const BLEND0_RX_RGB_SWAP_EN_OFFSET: u32 = 2;
    pub const BLEND0_RX_RGB_SWAP_EN_MASK: u32 = 0x00000004;
    pub const BLEND0_RX_RGB_SWAP_EN: u32 = 1 << 2;

    /// **BLEND1_RX_BYTE_SWAP_EN**
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BLEND1_RX_BYTE_SWAP_EN_OFFSET: u32 = 1;
    pub const BLEND1_RX_BYTE_SWAP_EN_MASK: u32 = 0x00000002;
    pub const BLEND1_RX_BYTE_SWAP_EN: u32 = 1 << 1;

    /// **BLEND0_RX_BYTE_SWAP_EN**
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const BLEND0_RX_BYTE_SWAP_EN_OFFSET: u32 = 0;
    pub const BLEND0_RX_BYTE_SWAP_EN_MASK: u32 = 0x00000001;
    pub const BLEND0_RX_BYTE_SWAP_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BLEND1_RX_RGB_SWAP_EN** `[3]` (RW)
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    #[inline(always)]
    pub fn blend1_rx_rgb_swap_en(&self) -> bool {
        (self.value.read_bits() & Self::BLEND1_RX_RGB_SWAP_EN_MASK) != 0
    }

    /// Set BLEND1_RX_RGB_SWAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend1_rx_rgb_swap_en(&self) {
        self.value
            .modify_bits(|r| r | Self::BLEND1_RX_RGB_SWAP_EN_MASK);
    }

    /// Clear BLEND1_RX_RGB_SWAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend1_rx_rgb_swap_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND1_RX_RGB_SWAP_EN_MASK);
    }

    /// **BLEND0_RX_RGB_SWAP_EN** `[2]` (RW)
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    #[inline(always)]
    pub fn blend0_rx_rgb_swap_en(&self) -> bool {
        (self.value.read_bits() & Self::BLEND0_RX_RGB_SWAP_EN_MASK) != 0
    }

    /// Set BLEND0_RX_RGB_SWAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend0_rx_rgb_swap_en(&self) {
        self.value
            .modify_bits(|r| r | Self::BLEND0_RX_RGB_SWAP_EN_MASK);
    }

    /// Clear BLEND0_RX_RGB_SWAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend0_rx_rgb_swap_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND0_RX_RGB_SWAP_EN_MASK);
    }

    /// **BLEND1_RX_BYTE_SWAP_EN** `[1]` (RW)
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    #[inline(always)]
    pub fn blend1_rx_byte_swap_en(&self) -> bool {
        (self.value.read_bits() & Self::BLEND1_RX_BYTE_SWAP_EN_MASK) != 0
    }

    /// Set BLEND1_RX_BYTE_SWAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend1_rx_byte_swap_en(&self) {
        self.value
            .modify_bits(|r| r | Self::BLEND1_RX_BYTE_SWAP_EN_MASK);
    }

    /// Clear BLEND1_RX_BYTE_SWAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend1_rx_byte_swap_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND1_RX_BYTE_SWAP_EN_MASK);
    }

    /// **BLEND0_RX_BYTE_SWAP_EN** `[0]` (RW)
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    #[inline(always)]
    pub fn blend0_rx_byte_swap_en(&self) -> bool {
        (self.value.read_bits() & Self::BLEND0_RX_BYTE_SWAP_EN_MASK) != 0
    }

    /// Set BLEND0_RX_BYTE_SWAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend0_rx_byte_swap_en(&self) {
        self.value
            .modify_bits(|r| r | Self::BLEND0_RX_BYTE_SWAP_EN_MASK);
    }

    /// Clear BLEND0_RX_BYTE_SWAP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend0_rx_byte_swap_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND0_RX_BYTE_SWAP_EN_MASK);
    }
}

/// BLEND_BYTE_ORDER value type for batch read/modify/write operations
///
/// Use `blendbyteorder.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `blendbyteorder.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BlendByteOrderVal {
    bits: u32,
}

impl BlendByteOrderVal {
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

    /// Read **BLEND1_RX_RGB_SWAP_EN** `[3:3]`
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    #[inline(always)]
    pub const fn blend1_rx_rgb_swap_en(&self) -> bool {
        (self.bits & BlendByteOrder::BLEND1_RX_RGB_SWAP_EN_MASK) != 0
    }

    /// Read **BLEND0_RX_RGB_SWAP_EN** `[2:2]`
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    #[inline(always)]
    pub const fn blend0_rx_rgb_swap_en(&self) -> bool {
        (self.bits & BlendByteOrder::BLEND0_RX_RGB_SWAP_EN_MASK) != 0
    }

    /// Read **BLEND1_RX_BYTE_SWAP_EN** `[1:1]`
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    #[inline(always)]
    pub const fn blend1_rx_byte_swap_en(&self) -> bool {
        (self.bits & BlendByteOrder::BLEND1_RX_BYTE_SWAP_EN_MASK) != 0
    }

    /// Read **BLEND0_RX_BYTE_SWAP_EN** `[0:0]`
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    #[inline(always)]
    pub const fn blend0_rx_byte_swap_en(&self) -> bool {
        (self.bits & BlendByteOrder::BLEND0_RX_BYTE_SWAP_EN_MASK) != 0
    }

    /// Set **BLEND1_RX_RGB_SWAP_EN** `[3:3]`
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    #[inline(always)]
    pub const fn with_blend1_rx_rgb_swap_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BlendByteOrder::BLEND1_RX_RGB_SWAP_EN_MASK
            } else {
                self.bits & !BlendByteOrder::BLEND1_RX_RGB_SWAP_EN_MASK
            },
        }
    }

    /// Set **BLEND0_RX_RGB_SWAP_EN** `[2:2]`
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    #[inline(always)]
    pub const fn with_blend0_rx_rgb_swap_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BlendByteOrder::BLEND0_RX_RGB_SWAP_EN_MASK
            } else {
                self.bits & !BlendByteOrder::BLEND0_RX_RGB_SWAP_EN_MASK
            },
        }
    }

    /// Set **BLEND1_RX_BYTE_SWAP_EN** `[1:1]`
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    #[inline(always)]
    pub const fn with_blend1_rx_byte_swap_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BlendByteOrder::BLEND1_RX_BYTE_SWAP_EN_MASK
            } else {
                self.bits & !BlendByteOrder::BLEND1_RX_BYTE_SWAP_EN_MASK
            },
        }
    }

    /// Set **BLEND0_RX_BYTE_SWAP_EN** `[0:0]`
    ///
    /// Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    #[inline(always)]
    pub const fn with_blend0_rx_byte_swap_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BlendByteOrder::BLEND0_RX_BYTE_SWAP_EN_MASK
            } else {
                self.bits & !BlendByteOrder::BLEND0_RX_BYTE_SWAP_EN_MASK
            },
        }
    }
}

impl Default for BlendByteOrderVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BlendByteOrder {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BlendByteOrderVal {
        BlendByteOrderVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BlendByteOrderVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BlendByteOrderVal) -> BlendByteOrderVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BLEND_TRANS_MODE Register
///
/// Blending engine mode configure register
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **BLEND_RST** `[4]` (RW)
///
///   write 1 then write 0 to reset blending engine.
///
/// - **UPDATE** `[3]` (WO)
///
///   Set this bit to update the transfer mode. Only the bit is set the transfer mode is valid.
///
/// - **BLEND_FIX_PIXEL_FILL_EN** `[2]` (RW)
///
///   This bit is used to enable fix pixel filling. When this mode is enable only Tx channel is work and the output pixel is configured by PPA_OUT_FIX_PIXEL.
///
/// - **BLEND_BYPASS** `[1]` (RW)
///
///   Set this bit to bypass blender. Then background date would be output.
///
/// - **BLEND_EN** `[0]` (RW)
///
///   Set this bit to enable alpha blending.
#[repr(transparent)]
pub struct BlendTransMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BlendTransMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BlendTransMode {
    // ======== Field Constants ========

    /// **BLEND_RST**
    ///
    /// write 1 then write 0 to reset blending engine.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const BLEND_RST_OFFSET: u32 = 4;
    pub const BLEND_RST_MASK: u32 = 0x00000010;
    pub const BLEND_RST: u32 = 1 << 4;

    /// **UPDATE**
    ///
    /// Set this bit to update the transfer mode. Only the bit is set the transfer mode is valid.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const UPDATE_OFFSET: u32 = 3;
    pub const UPDATE_MASK: u32 = 0x00000008;
    pub const UPDATE: u32 = 1 << 3;

    /// **BLEND_FIX_PIXEL_FILL_EN**
    ///
    /// This bit is used to enable fix pixel filling. When this mode is enable only Tx channel is work and the output pixel is configured by PPA_OUT_FIX_PIXEL.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const BLEND_FIX_PIXEL_FILL_EN_OFFSET: u32 = 2;
    pub const BLEND_FIX_PIXEL_FILL_EN_MASK: u32 = 0x00000004;
    pub const BLEND_FIX_PIXEL_FILL_EN: u32 = 1 << 2;

    /// **BLEND_BYPASS**
    ///
    /// Set this bit to bypass blender. Then background date would be output.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BLEND_BYPASS_OFFSET: u32 = 1;
    pub const BLEND_BYPASS_MASK: u32 = 0x00000002;
    pub const BLEND_BYPASS: u32 = 1 << 1;

    /// **BLEND_EN**
    ///
    /// Set this bit to enable alpha blending.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const BLEND_EN_OFFSET: u32 = 0;
    pub const BLEND_EN_MASK: u32 = 0x00000001;
    pub const BLEND_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BLEND_RST** `[4]` (RW)
    ///
    /// write 1 then write 0 to reset blending engine.
    #[inline(always)]
    pub fn blend_rst(&self) -> bool {
        (self.value.read_bits() & Self::BLEND_RST_MASK) != 0
    }

    /// Set BLEND_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend_rst(&self) {
        self.value.modify_bits(|r| r | Self::BLEND_RST_MASK);
    }

    /// Clear BLEND_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend_rst(&self) {
        self.value.modify_bits(|r| r & !Self::BLEND_RST_MASK);
    }

    /// Trigger **UPDATE** (WO)
    ///
    /// Set this bit to update the transfer mode. Only the bit is set the transfer mode is valid.
    #[inline(always)]
    pub fn trigger_update(&self) {
        self.value.modify_bits(|r| r | Self::UPDATE_MASK);
    }

    /// **BLEND_FIX_PIXEL_FILL_EN** `[2]` (RW)
    ///
    /// This bit is used to enable fix pixel filling. When this mode is enable only Tx channel is work and the output pixel is configured by PPA_OUT_FIX_PIXEL.
    #[inline(always)]
    pub fn blend_fix_pixel_fill_en(&self) -> bool {
        (self.value.read_bits() & Self::BLEND_FIX_PIXEL_FILL_EN_MASK) != 0
    }

    /// Set BLEND_FIX_PIXEL_FILL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend_fix_pixel_fill_en(&self) {
        self.value
            .modify_bits(|r| r | Self::BLEND_FIX_PIXEL_FILL_EN_MASK);
    }

    /// Clear BLEND_FIX_PIXEL_FILL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend_fix_pixel_fill_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND_FIX_PIXEL_FILL_EN_MASK);
    }

    /// **BLEND_BYPASS** `[1]` (RW)
    ///
    /// Set this bit to bypass blender. Then background date would be output.
    #[inline(always)]
    pub fn blend_bypass(&self) -> bool {
        (self.value.read_bits() & Self::BLEND_BYPASS_MASK) != 0
    }

    /// Set BLEND_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend_bypass(&self) {
        self.value.modify_bits(|r| r | Self::BLEND_BYPASS_MASK);
    }

    /// Clear BLEND_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend_bypass(&self) {
        self.value.modify_bits(|r| r & !Self::BLEND_BYPASS_MASK);
    }

    /// **BLEND_EN** `[0]` (RW)
    ///
    /// Set this bit to enable alpha blending.
    #[inline(always)]
    pub fn blend_en(&self) -> bool {
        (self.value.read_bits() & Self::BLEND_EN_MASK) != 0
    }

    /// Set BLEND_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend_en(&self) {
        self.value.modify_bits(|r| r | Self::BLEND_EN_MASK);
    }

    /// Clear BLEND_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend_en(&self) {
        self.value.modify_bits(|r| r & !Self::BLEND_EN_MASK);
    }
}

/// BLEND_TRANS_MODE value type for batch read/modify/write operations
///
/// Use `blendtransmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `blendtransmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BlendTransModeVal {
    bits: u32,
}

impl BlendTransModeVal {
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

    /// Read **BLEND_RST** `[4:4]`
    ///
    /// write 1 then write 0 to reset blending engine.
    #[inline(always)]
    pub const fn blend_rst(&self) -> bool {
        (self.bits & BlendTransMode::BLEND_RST_MASK) != 0
    }

    /// Read **BLEND_FIX_PIXEL_FILL_EN** `[2:2]`
    ///
    /// This bit is used to enable fix pixel filling. When this mode is enable only Tx channel is work and the output pixel is configured by PPA_OUT_FIX_PIXEL.
    #[inline(always)]
    pub const fn blend_fix_pixel_fill_en(&self) -> bool {
        (self.bits & BlendTransMode::BLEND_FIX_PIXEL_FILL_EN_MASK) != 0
    }

    /// Read **BLEND_BYPASS** `[1:1]`
    ///
    /// Set this bit to bypass blender. Then background date would be output.
    #[inline(always)]
    pub const fn blend_bypass(&self) -> bool {
        (self.bits & BlendTransMode::BLEND_BYPASS_MASK) != 0
    }

    /// Read **BLEND_EN** `[0:0]`
    ///
    /// Set this bit to enable alpha blending.
    #[inline(always)]
    pub const fn blend_en(&self) -> bool {
        (self.bits & BlendTransMode::BLEND_EN_MASK) != 0
    }

    /// Set **BLEND_RST** `[4:4]`
    ///
    /// write 1 then write 0 to reset blending engine.
    #[inline(always)]
    pub const fn with_blend_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BlendTransMode::BLEND_RST_MASK
            } else {
                self.bits & !BlendTransMode::BLEND_RST_MASK
            },
        }
    }

    /// Set **UPDATE** `[3:3]`
    ///
    /// Set this bit to update the transfer mode. Only the bit is set the transfer mode is valid.
    #[inline(always)]
    pub const fn with_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BlendTransMode::UPDATE_MASK
            } else {
                self.bits & !BlendTransMode::UPDATE_MASK
            },
        }
    }

    /// Set **BLEND_FIX_PIXEL_FILL_EN** `[2:2]`
    ///
    /// This bit is used to enable fix pixel filling. When this mode is enable only Tx channel is work and the output pixel is configured by PPA_OUT_FIX_PIXEL.
    #[inline(always)]
    pub const fn with_blend_fix_pixel_fill_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BlendTransMode::BLEND_FIX_PIXEL_FILL_EN_MASK
            } else {
                self.bits & !BlendTransMode::BLEND_FIX_PIXEL_FILL_EN_MASK
            },
        }
    }

    /// Set **BLEND_BYPASS** `[1:1]`
    ///
    /// Set this bit to bypass blender. Then background date would be output.
    #[inline(always)]
    pub const fn with_blend_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BlendTransMode::BLEND_BYPASS_MASK
            } else {
                self.bits & !BlendTransMode::BLEND_BYPASS_MASK
            },
        }
    }

    /// Set **BLEND_EN** `[0:0]`
    ///
    /// Set this bit to enable alpha blending.
    #[inline(always)]
    pub const fn with_blend_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BlendTransMode::BLEND_EN_MASK
            } else {
                self.bits & !BlendTransMode::BLEND_EN_MASK
            },
        }
    }
}

impl Default for BlendTransModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BlendTransMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BlendTransModeVal {
        BlendTransModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BlendTransModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BlendTransModeVal) -> BlendTransModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SR_FIX_ALPHA Register
///
/// Scaling and rotating engine alpha override register
///
/// Address offset: 0x0038
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **SR_RX_ALPHA_INV** `[10]` (RW)
///
///   Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
///
/// - **SR_RX_ALPHA_MOD** `[9:8]` (RW)
///
///   Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
///
/// - **SR_RX_FIX_ALPHA** `[7:0]` (RW)
///
///   The value would replace the alpha value in received pixel for Scaling and Rotating engine when PPA_SR_RX_ALPHA_CONF_EN is enabled.
#[repr(transparent)]
pub struct SrFixAlpha {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SrFixAlpha {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SrFixAlpha {
    // ======== Field Constants ========

    /// **SR_RX_ALPHA_INV**
    ///
    /// Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SR_RX_ALPHA_INV_OFFSET: u32 = 10;
    pub const SR_RX_ALPHA_INV_MASK: u32 = 0x00000400;
    pub const SR_RX_ALPHA_INV: u32 = 1 << 10;

    /// **SR_RX_ALPHA_MOD**
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const SR_RX_ALPHA_MOD_OFFSET: u32 = 8;
    pub const SR_RX_ALPHA_MOD_MASK: u32 = 0x00000300;

    /// **SR_RX_FIX_ALPHA**
    ///
    /// The value would replace the alpha value in received pixel for Scaling and Rotating engine when PPA_SR_RX_ALPHA_CONF_EN is enabled.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SR_RX_FIX_ALPHA_OFFSET: u32 = 0;
    pub const SR_RX_FIX_ALPHA_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SR_RX_ALPHA_INV** `[10]` (RW)
    ///
    /// Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
    #[inline(always)]
    pub fn sr_rx_alpha_inv(&self) -> bool {
        (self.value.read_bits() & Self::SR_RX_ALPHA_INV_MASK) != 0
    }

    /// Set SR_RX_ALPHA_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_rx_alpha_inv(&self) {
        self.value.modify_bits(|r| r | Self::SR_RX_ALPHA_INV_MASK);
    }

    /// Clear SR_RX_ALPHA_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_rx_alpha_inv(&self) {
        self.value.modify_bits(|r| r & !Self::SR_RX_ALPHA_INV_MASK);
    }

    /// **SR_RX_ALPHA_MOD** `[9:8]` (RW)
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    #[inline(always)]
    pub fn sr_rx_alpha_mod(&self) -> u32 {
        (self.value.read_bits() & Self::SR_RX_ALPHA_MOD_MASK) >> Self::SR_RX_ALPHA_MOD_OFFSET
    }

    /// Write **SR_RX_ALPHA_MOD** `[9:8]` (RW)
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    #[inline(always)]
    pub fn write_sr_rx_alpha_mod(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SR_RX_ALPHA_MOD_MASK)
                | ((value << Self::SR_RX_ALPHA_MOD_OFFSET) & Self::SR_RX_ALPHA_MOD_MASK)
        });
    }

    /// **SR_RX_FIX_ALPHA** `[7:0]` (RW)
    ///
    /// The value would replace the alpha value in received pixel for Scaling and Rotating engine when PPA_SR_RX_ALPHA_CONF_EN is enabled.
    #[inline(always)]
    pub fn sr_rx_fix_alpha(&self) -> u32 {
        (self.value.read_bits() & Self::SR_RX_FIX_ALPHA_MASK) >> Self::SR_RX_FIX_ALPHA_OFFSET
    }

    /// Write **SR_RX_FIX_ALPHA** `[7:0]` (RW)
    ///
    /// The value would replace the alpha value in received pixel for Scaling and Rotating engine when PPA_SR_RX_ALPHA_CONF_EN is enabled.
    #[inline(always)]
    pub fn write_sr_rx_fix_alpha(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SR_RX_FIX_ALPHA_MASK)
                | ((value << Self::SR_RX_FIX_ALPHA_OFFSET) & Self::SR_RX_FIX_ALPHA_MASK)
        });
    }
}

/// SR_FIX_ALPHA value type for batch read/modify/write operations
///
/// Use `srfixalpha.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `srfixalpha.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SrFixAlphaVal {
    bits: u32,
}

impl SrFixAlphaVal {
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
        Self { bits: 0x00000080 }
    }

    /// Read **SR_RX_ALPHA_INV** `[10:10]`
    ///
    /// Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
    #[inline(always)]
    pub const fn sr_rx_alpha_inv(&self) -> bool {
        (self.bits & SrFixAlpha::SR_RX_ALPHA_INV_MASK) != 0
    }

    /// Read **SR_RX_ALPHA_MOD** `[9:8]`
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    #[inline(always)]
    pub const fn sr_rx_alpha_mod(&self) -> u32 {
        (self.bits & SrFixAlpha::SR_RX_ALPHA_MOD_MASK) >> SrFixAlpha::SR_RX_ALPHA_MOD_OFFSET
    }

    /// Read **SR_RX_FIX_ALPHA** `[7:0]`
    ///
    /// The value would replace the alpha value in received pixel for Scaling and Rotating engine when PPA_SR_RX_ALPHA_CONF_EN is enabled.
    #[inline(always)]
    pub const fn sr_rx_fix_alpha(&self) -> u32 {
        (self.bits & SrFixAlpha::SR_RX_FIX_ALPHA_MASK) >> SrFixAlpha::SR_RX_FIX_ALPHA_OFFSET
    }

    /// Set **SR_RX_ALPHA_INV** `[10:10]`
    ///
    /// Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
    #[inline(always)]
    pub const fn with_sr_rx_alpha_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrFixAlpha::SR_RX_ALPHA_INV_MASK
            } else {
                self.bits & !SrFixAlpha::SR_RX_ALPHA_INV_MASK
            },
        }
    }

    /// Set **SR_RX_ALPHA_MOD** `[9:8]`
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    #[inline(always)]
    pub const fn with_sr_rx_alpha_mod(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SrFixAlpha::SR_RX_ALPHA_MOD_MASK)
                | ((value << SrFixAlpha::SR_RX_ALPHA_MOD_OFFSET)
                    & SrFixAlpha::SR_RX_ALPHA_MOD_MASK),
        }
    }

    /// Set **SR_RX_FIX_ALPHA** `[7:0]`
    ///
    /// The value would replace the alpha value in received pixel for Scaling and Rotating engine when PPA_SR_RX_ALPHA_CONF_EN is enabled.
    #[inline(always)]
    pub const fn with_sr_rx_fix_alpha(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SrFixAlpha::SR_RX_FIX_ALPHA_MASK)
                | ((value << SrFixAlpha::SR_RX_FIX_ALPHA_OFFSET)
                    & SrFixAlpha::SR_RX_FIX_ALPHA_MASK),
        }
    }
}

impl Default for SrFixAlphaVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SrFixAlpha {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SrFixAlphaVal {
        SrFixAlphaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SrFixAlphaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SrFixAlphaVal) -> SrFixAlphaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BLEND_TX_SIZE Register
///
/// Fix pixel filling mode image size register
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **BLEND_VB** `[27:14]` (RW)
///
///   The vertical width of image block that would be filled in fix pixel filling mode. The unit is pixel
///
/// - **BLEND_HB** `[13:0]` (RW)
///
///   The horizontal width of image block that would be filled in fix pixel filling mode. The unit is pixel
#[repr(transparent)]
pub struct BlendTxSize {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BlendTxSize {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BlendTxSize {
    // ======== Field Constants ========

    /// **BLEND_VB**
    ///
    /// The vertical width of image block that would be filled in fix pixel filling mode. The unit is pixel
    ///
    /// Bits `[27:14]`, mask `0x0fffc000`
    pub const BLEND_VB_OFFSET: u32 = 14;
    pub const BLEND_VB_MASK: u32 = 0x0fffc000;

    /// **BLEND_HB**
    ///
    /// The horizontal width of image block that would be filled in fix pixel filling mode. The unit is pixel
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const BLEND_HB_OFFSET: u32 = 0;
    pub const BLEND_HB_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **BLEND_VB** `[27:14]` (RW)
    ///
    /// The vertical width of image block that would be filled in fix pixel filling mode. The unit is pixel
    #[inline(always)]
    pub fn blend_vb(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND_VB_MASK) >> Self::BLEND_VB_OFFSET
    }

    /// Write **BLEND_VB** `[27:14]` (RW)
    ///
    /// The vertical width of image block that would be filled in fix pixel filling mode. The unit is pixel
    #[inline(always)]
    pub fn write_blend_vb(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLEND_VB_MASK) | ((value << Self::BLEND_VB_OFFSET) & Self::BLEND_VB_MASK)
        });
    }

    /// **BLEND_HB** `[13:0]` (RW)
    ///
    /// The horizontal width of image block that would be filled in fix pixel filling mode. The unit is pixel
    #[inline(always)]
    pub fn blend_hb(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND_HB_MASK) >> Self::BLEND_HB_OFFSET
    }

    /// Write **BLEND_HB** `[13:0]` (RW)
    ///
    /// The horizontal width of image block that would be filled in fix pixel filling mode. The unit is pixel
    #[inline(always)]
    pub fn write_blend_hb(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLEND_HB_MASK) | ((value << Self::BLEND_HB_OFFSET) & Self::BLEND_HB_MASK)
        });
    }
}

/// BLEND_TX_SIZE value type for batch read/modify/write operations
///
/// Use `blendtxsize.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `blendtxsize.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BlendTxSizeVal {
    bits: u32,
}

impl BlendTxSizeVal {
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

    /// Read **BLEND_VB** `[27:14]`
    ///
    /// The vertical width of image block that would be filled in fix pixel filling mode. The unit is pixel
    #[inline(always)]
    pub const fn blend_vb(&self) -> u32 {
        (self.bits & BlendTxSize::BLEND_VB_MASK) >> BlendTxSize::BLEND_VB_OFFSET
    }

    /// Read **BLEND_HB** `[13:0]`
    ///
    /// The horizontal width of image block that would be filled in fix pixel filling mode. The unit is pixel
    #[inline(always)]
    pub const fn blend_hb(&self) -> u32 {
        (self.bits & BlendTxSize::BLEND_HB_MASK) >> BlendTxSize::BLEND_HB_OFFSET
    }

    /// Set **BLEND_VB** `[27:14]`
    ///
    /// The vertical width of image block that would be filled in fix pixel filling mode. The unit is pixel
    #[inline(always)]
    pub const fn with_blend_vb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlendTxSize::BLEND_VB_MASK)
                | ((value << BlendTxSize::BLEND_VB_OFFSET) & BlendTxSize::BLEND_VB_MASK),
        }
    }

    /// Set **BLEND_HB** `[13:0]`
    ///
    /// The horizontal width of image block that would be filled in fix pixel filling mode. The unit is pixel
    #[inline(always)]
    pub const fn with_blend_hb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlendTxSize::BLEND_HB_MASK)
                | ((value << BlendTxSize::BLEND_HB_OFFSET) & BlendTxSize::BLEND_HB_MASK),
        }
    }
}

impl Default for BlendTxSizeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BlendTxSize {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BlendTxSizeVal {
        BlendTxSizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BlendTxSizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BlendTxSizeVal) -> BlendTxSizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BLEND_FIX_ALPHA Register
///
/// Blending engine alpha override register
///
/// Address offset: 0x0040
/// Reset value: 0x00008080
///
/// ## Fields
///
/// - **BLEND1_RX_ALPHA_INV** `[21]` (RW)
///
///   Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
///
/// - **BLEND0_RX_ALPHA_INV** `[20]` (RW)
///
///   Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
///
/// - **BLEND1_RX_ALPHA_MOD** `[19:18]` (RW)
///
///   Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
///
/// - **BLEND0_RX_ALPHA_MOD** `[17:16]` (RW)
///
///   Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
///
/// - **BLEND1_RX_FIX_ALPHA** `[15:8]` (RW)
///
///   The value would replace the alpha value in received pixel for foreground plane of blender when PPA_BLEND1_RX_ALPHA_CONF_EN is enabled.
///
/// - **BLEND0_RX_FIX_ALPHA** `[7:0]` (RW)
///
///   The value would replace the alpha value in received pixel for background plane of blender when PPA_BLEND0_RX_ALPHA_CONF_EN is enabled.
#[repr(transparent)]
pub struct BlendFixAlpha {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BlendFixAlpha {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BlendFixAlpha {
    // ======== Field Constants ========

    /// **BLEND1_RX_ALPHA_INV**
    ///
    /// Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const BLEND1_RX_ALPHA_INV_OFFSET: u32 = 21;
    pub const BLEND1_RX_ALPHA_INV_MASK: u32 = 0x00200000;
    pub const BLEND1_RX_ALPHA_INV: u32 = 1 << 21;

    /// **BLEND0_RX_ALPHA_INV**
    ///
    /// Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const BLEND0_RX_ALPHA_INV_OFFSET: u32 = 20;
    pub const BLEND0_RX_ALPHA_INV_MASK: u32 = 0x00100000;
    pub const BLEND0_RX_ALPHA_INV: u32 = 1 << 20;

    /// **BLEND1_RX_ALPHA_MOD**
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    ///
    /// Bits `[19:18]`, mask `0x000c0000`
    pub const BLEND1_RX_ALPHA_MOD_OFFSET: u32 = 18;
    pub const BLEND1_RX_ALPHA_MOD_MASK: u32 = 0x000c0000;

    /// **BLEND0_RX_ALPHA_MOD**
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const BLEND0_RX_ALPHA_MOD_OFFSET: u32 = 16;
    pub const BLEND0_RX_ALPHA_MOD_MASK: u32 = 0x00030000;

    /// **BLEND1_RX_FIX_ALPHA**
    ///
    /// The value would replace the alpha value in received pixel for foreground plane of blender when PPA_BLEND1_RX_ALPHA_CONF_EN is enabled.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const BLEND1_RX_FIX_ALPHA_OFFSET: u32 = 8;
    pub const BLEND1_RX_FIX_ALPHA_MASK: u32 = 0x0000ff00;

    /// **BLEND0_RX_FIX_ALPHA**
    ///
    /// The value would replace the alpha value in received pixel for background plane of blender when PPA_BLEND0_RX_ALPHA_CONF_EN is enabled.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const BLEND0_RX_FIX_ALPHA_OFFSET: u32 = 0;
    pub const BLEND0_RX_FIX_ALPHA_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **BLEND1_RX_ALPHA_INV** `[21]` (RW)
    ///
    /// Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
    #[inline(always)]
    pub fn blend1_rx_alpha_inv(&self) -> bool {
        (self.value.read_bits() & Self::BLEND1_RX_ALPHA_INV_MASK) != 0
    }

    /// Set BLEND1_RX_ALPHA_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend1_rx_alpha_inv(&self) {
        self.value
            .modify_bits(|r| r | Self::BLEND1_RX_ALPHA_INV_MASK);
    }

    /// Clear BLEND1_RX_ALPHA_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend1_rx_alpha_inv(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND1_RX_ALPHA_INV_MASK);
    }

    /// **BLEND0_RX_ALPHA_INV** `[20]` (RW)
    ///
    /// Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
    #[inline(always)]
    pub fn blend0_rx_alpha_inv(&self) -> bool {
        (self.value.read_bits() & Self::BLEND0_RX_ALPHA_INV_MASK) != 0
    }

    /// Set BLEND0_RX_ALPHA_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_blend0_rx_alpha_inv(&self) {
        self.value
            .modify_bits(|r| r | Self::BLEND0_RX_ALPHA_INV_MASK);
    }

    /// Clear BLEND0_RX_ALPHA_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_blend0_rx_alpha_inv(&self) {
        self.value
            .modify_bits(|r| r & !Self::BLEND0_RX_ALPHA_INV_MASK);
    }

    /// **BLEND1_RX_ALPHA_MOD** `[19:18]` (RW)
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    #[inline(always)]
    pub fn blend1_rx_alpha_mod(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND1_RX_ALPHA_MOD_MASK)
            >> Self::BLEND1_RX_ALPHA_MOD_OFFSET
    }

    /// Write **BLEND1_RX_ALPHA_MOD** `[19:18]` (RW)
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    #[inline(always)]
    pub fn write_blend1_rx_alpha_mod(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLEND1_RX_ALPHA_MOD_MASK)
                | ((value << Self::BLEND1_RX_ALPHA_MOD_OFFSET) & Self::BLEND1_RX_ALPHA_MOD_MASK)
        });
    }

    /// **BLEND0_RX_ALPHA_MOD** `[17:16]` (RW)
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    #[inline(always)]
    pub fn blend0_rx_alpha_mod(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND0_RX_ALPHA_MOD_MASK)
            >> Self::BLEND0_RX_ALPHA_MOD_OFFSET
    }

    /// Write **BLEND0_RX_ALPHA_MOD** `[17:16]` (RW)
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    #[inline(always)]
    pub fn write_blend0_rx_alpha_mod(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLEND0_RX_ALPHA_MOD_MASK)
                | ((value << Self::BLEND0_RX_ALPHA_MOD_OFFSET) & Self::BLEND0_RX_ALPHA_MOD_MASK)
        });
    }

    /// **BLEND1_RX_FIX_ALPHA** `[15:8]` (RW)
    ///
    /// The value would replace the alpha value in received pixel for foreground plane of blender when PPA_BLEND1_RX_ALPHA_CONF_EN is enabled.
    #[inline(always)]
    pub fn blend1_rx_fix_alpha(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND1_RX_FIX_ALPHA_MASK)
            >> Self::BLEND1_RX_FIX_ALPHA_OFFSET
    }

    /// Write **BLEND1_RX_FIX_ALPHA** `[15:8]` (RW)
    ///
    /// The value would replace the alpha value in received pixel for foreground plane of blender when PPA_BLEND1_RX_ALPHA_CONF_EN is enabled.
    #[inline(always)]
    pub fn write_blend1_rx_fix_alpha(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLEND1_RX_FIX_ALPHA_MASK)
                | ((value << Self::BLEND1_RX_FIX_ALPHA_OFFSET) & Self::BLEND1_RX_FIX_ALPHA_MASK)
        });
    }

    /// **BLEND0_RX_FIX_ALPHA** `[7:0]` (RW)
    ///
    /// The value would replace the alpha value in received pixel for background plane of blender when PPA_BLEND0_RX_ALPHA_CONF_EN is enabled.
    #[inline(always)]
    pub fn blend0_rx_fix_alpha(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND0_RX_FIX_ALPHA_MASK)
            >> Self::BLEND0_RX_FIX_ALPHA_OFFSET
    }

    /// Write **BLEND0_RX_FIX_ALPHA** `[7:0]` (RW)
    ///
    /// The value would replace the alpha value in received pixel for background plane of blender when PPA_BLEND0_RX_ALPHA_CONF_EN is enabled.
    #[inline(always)]
    pub fn write_blend0_rx_fix_alpha(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLEND0_RX_FIX_ALPHA_MASK)
                | ((value << Self::BLEND0_RX_FIX_ALPHA_OFFSET) & Self::BLEND0_RX_FIX_ALPHA_MASK)
        });
    }
}

/// BLEND_FIX_ALPHA value type for batch read/modify/write operations
///
/// Use `blendfixalpha.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `blendfixalpha.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BlendFixAlphaVal {
    bits: u32,
}

impl BlendFixAlphaVal {
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
        Self { bits: 0x00008080 }
    }

    /// Read **BLEND1_RX_ALPHA_INV** `[21:21]`
    ///
    /// Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
    #[inline(always)]
    pub const fn blend1_rx_alpha_inv(&self) -> bool {
        (self.bits & BlendFixAlpha::BLEND1_RX_ALPHA_INV_MASK) != 0
    }

    /// Read **BLEND0_RX_ALPHA_INV** `[20:20]`
    ///
    /// Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
    #[inline(always)]
    pub const fn blend0_rx_alpha_inv(&self) -> bool {
        (self.bits & BlendFixAlpha::BLEND0_RX_ALPHA_INV_MASK) != 0
    }

    /// Read **BLEND1_RX_ALPHA_MOD** `[19:18]`
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    #[inline(always)]
    pub const fn blend1_rx_alpha_mod(&self) -> u32 {
        (self.bits & BlendFixAlpha::BLEND1_RX_ALPHA_MOD_MASK)
            >> BlendFixAlpha::BLEND1_RX_ALPHA_MOD_OFFSET
    }

    /// Read **BLEND0_RX_ALPHA_MOD** `[17:16]`
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    #[inline(always)]
    pub const fn blend0_rx_alpha_mod(&self) -> u32 {
        (self.bits & BlendFixAlpha::BLEND0_RX_ALPHA_MOD_MASK)
            >> BlendFixAlpha::BLEND0_RX_ALPHA_MOD_OFFSET
    }

    /// Read **BLEND1_RX_FIX_ALPHA** `[15:8]`
    ///
    /// The value would replace the alpha value in received pixel for foreground plane of blender when PPA_BLEND1_RX_ALPHA_CONF_EN is enabled.
    #[inline(always)]
    pub const fn blend1_rx_fix_alpha(&self) -> u32 {
        (self.bits & BlendFixAlpha::BLEND1_RX_FIX_ALPHA_MASK)
            >> BlendFixAlpha::BLEND1_RX_FIX_ALPHA_OFFSET
    }

    /// Read **BLEND0_RX_FIX_ALPHA** `[7:0]`
    ///
    /// The value would replace the alpha value in received pixel for background plane of blender when PPA_BLEND0_RX_ALPHA_CONF_EN is enabled.
    #[inline(always)]
    pub const fn blend0_rx_fix_alpha(&self) -> u32 {
        (self.bits & BlendFixAlpha::BLEND0_RX_FIX_ALPHA_MASK)
            >> BlendFixAlpha::BLEND0_RX_FIX_ALPHA_OFFSET
    }

    /// Set **BLEND1_RX_ALPHA_INV** `[21:21]`
    ///
    /// Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
    #[inline(always)]
    pub const fn with_blend1_rx_alpha_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BlendFixAlpha::BLEND1_RX_ALPHA_INV_MASK
            } else {
                self.bits & !BlendFixAlpha::BLEND1_RX_ALPHA_INV_MASK
            },
        }
    }

    /// Set **BLEND0_RX_ALPHA_INV** `[20:20]`
    ///
    /// Set this bit to invert the original alpha value. When RX color mode is RGB565/RGB88. The original alpha value is 255.
    #[inline(always)]
    pub const fn with_blend0_rx_alpha_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BlendFixAlpha::BLEND0_RX_ALPHA_INV_MASK
            } else {
                self.bits & !BlendFixAlpha::BLEND0_RX_ALPHA_INV_MASK
            },
        }
    }

    /// Set **BLEND1_RX_ALPHA_MOD** `[19:18]`
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    #[inline(always)]
    pub const fn with_blend1_rx_alpha_mod(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlendFixAlpha::BLEND1_RX_ALPHA_MOD_MASK)
                | ((value << BlendFixAlpha::BLEND1_RX_ALPHA_MOD_OFFSET)
                    & BlendFixAlpha::BLEND1_RX_ALPHA_MOD_MASK),
        }
    }

    /// Set **BLEND0_RX_ALPHA_MOD** `[17:16]`
    ///
    /// Alpha mode. 0/3: not replace alpha. 1: replace alpha with PPA_SR_FIX_ALPHA. 2: Original alpha multiply with PPA_SR_FIX_ALPHA/256.
    #[inline(always)]
    pub const fn with_blend0_rx_alpha_mod(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlendFixAlpha::BLEND0_RX_ALPHA_MOD_MASK)
                | ((value << BlendFixAlpha::BLEND0_RX_ALPHA_MOD_OFFSET)
                    & BlendFixAlpha::BLEND0_RX_ALPHA_MOD_MASK),
        }
    }

    /// Set **BLEND1_RX_FIX_ALPHA** `[15:8]`
    ///
    /// The value would replace the alpha value in received pixel for foreground plane of blender when PPA_BLEND1_RX_ALPHA_CONF_EN is enabled.
    #[inline(always)]
    pub const fn with_blend1_rx_fix_alpha(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlendFixAlpha::BLEND1_RX_FIX_ALPHA_MASK)
                | ((value << BlendFixAlpha::BLEND1_RX_FIX_ALPHA_OFFSET)
                    & BlendFixAlpha::BLEND1_RX_FIX_ALPHA_MASK),
        }
    }

    /// Set **BLEND0_RX_FIX_ALPHA** `[7:0]`
    ///
    /// The value would replace the alpha value in received pixel for background plane of blender when PPA_BLEND0_RX_ALPHA_CONF_EN is enabled.
    #[inline(always)]
    pub const fn with_blend0_rx_fix_alpha(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlendFixAlpha::BLEND0_RX_FIX_ALPHA_MASK)
                | ((value << BlendFixAlpha::BLEND0_RX_FIX_ALPHA_OFFSET)
                    & BlendFixAlpha::BLEND0_RX_FIX_ALPHA_MASK),
        }
    }
}

impl Default for BlendFixAlphaVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BlendFixAlpha {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BlendFixAlphaVal {
        BlendFixAlphaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BlendFixAlphaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BlendFixAlphaVal) -> BlendFixAlphaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BLEND_RGB Register
///
/// RGB color register
///
/// Address offset: 0x0048
/// Reset value: 0x00808080
///
/// ## Fields
///
/// - **BLEND1_RX_R** `[23:16]` (RW)
///
///   red color for A4/A8 mode.
///
/// - **BLEND1_RX_G** `[15:8]` (RW)
///
///   green color for A4/A8 mode.
///
/// - **BLEND1_RX_B** `[7:0]` (RW)
///
///   blue color for A4/A8 mode.
#[repr(transparent)]
pub struct BlendRgb {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BlendRgb {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BlendRgb {
    // ======== Field Constants ========

    /// **BLEND1_RX_R**
    ///
    /// red color for A4/A8 mode.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const BLEND1_RX_R_OFFSET: u32 = 16;
    pub const BLEND1_RX_R_MASK: u32 = 0x00ff0000;

    /// **BLEND1_RX_G**
    ///
    /// green color for A4/A8 mode.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const BLEND1_RX_G_OFFSET: u32 = 8;
    pub const BLEND1_RX_G_MASK: u32 = 0x0000ff00;

    /// **BLEND1_RX_B**
    ///
    /// blue color for A4/A8 mode.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const BLEND1_RX_B_OFFSET: u32 = 0;
    pub const BLEND1_RX_B_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **BLEND1_RX_R** `[23:16]` (RW)
    ///
    /// red color for A4/A8 mode.
    #[inline(always)]
    pub fn blend1_rx_r(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND1_RX_R_MASK) >> Self::BLEND1_RX_R_OFFSET
    }

    /// Write **BLEND1_RX_R** `[23:16]` (RW)
    ///
    /// red color for A4/A8 mode.
    #[inline(always)]
    pub fn write_blend1_rx_r(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLEND1_RX_R_MASK)
                | ((value << Self::BLEND1_RX_R_OFFSET) & Self::BLEND1_RX_R_MASK)
        });
    }

    /// **BLEND1_RX_G** `[15:8]` (RW)
    ///
    /// green color for A4/A8 mode.
    #[inline(always)]
    pub fn blend1_rx_g(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND1_RX_G_MASK) >> Self::BLEND1_RX_G_OFFSET
    }

    /// Write **BLEND1_RX_G** `[15:8]` (RW)
    ///
    /// green color for A4/A8 mode.
    #[inline(always)]
    pub fn write_blend1_rx_g(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLEND1_RX_G_MASK)
                | ((value << Self::BLEND1_RX_G_OFFSET) & Self::BLEND1_RX_G_MASK)
        });
    }

    /// **BLEND1_RX_B** `[7:0]` (RW)
    ///
    /// blue color for A4/A8 mode.
    #[inline(always)]
    pub fn blend1_rx_b(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND1_RX_B_MASK) >> Self::BLEND1_RX_B_OFFSET
    }

    /// Write **BLEND1_RX_B** `[7:0]` (RW)
    ///
    /// blue color for A4/A8 mode.
    #[inline(always)]
    pub fn write_blend1_rx_b(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLEND1_RX_B_MASK)
                | ((value << Self::BLEND1_RX_B_OFFSET) & Self::BLEND1_RX_B_MASK)
        });
    }
}

/// BLEND_RGB value type for batch read/modify/write operations
///
/// Use `blendrgb.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `blendrgb.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BlendRgbVal {
    bits: u32,
}

impl BlendRgbVal {
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
        Self { bits: 0x00808080 }
    }

    /// Read **BLEND1_RX_R** `[23:16]`
    ///
    /// red color for A4/A8 mode.
    #[inline(always)]
    pub const fn blend1_rx_r(&self) -> u32 {
        (self.bits & BlendRgb::BLEND1_RX_R_MASK) >> BlendRgb::BLEND1_RX_R_OFFSET
    }

    /// Read **BLEND1_RX_G** `[15:8]`
    ///
    /// green color for A4/A8 mode.
    #[inline(always)]
    pub const fn blend1_rx_g(&self) -> u32 {
        (self.bits & BlendRgb::BLEND1_RX_G_MASK) >> BlendRgb::BLEND1_RX_G_OFFSET
    }

    /// Read **BLEND1_RX_B** `[7:0]`
    ///
    /// blue color for A4/A8 mode.
    #[inline(always)]
    pub const fn blend1_rx_b(&self) -> u32 {
        (self.bits & BlendRgb::BLEND1_RX_B_MASK) >> BlendRgb::BLEND1_RX_B_OFFSET
    }

    /// Set **BLEND1_RX_R** `[23:16]`
    ///
    /// red color for A4/A8 mode.
    #[inline(always)]
    pub const fn with_blend1_rx_r(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlendRgb::BLEND1_RX_R_MASK)
                | ((value << BlendRgb::BLEND1_RX_R_OFFSET) & BlendRgb::BLEND1_RX_R_MASK),
        }
    }

    /// Set **BLEND1_RX_G** `[15:8]`
    ///
    /// green color for A4/A8 mode.
    #[inline(always)]
    pub const fn with_blend1_rx_g(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlendRgb::BLEND1_RX_G_MASK)
                | ((value << BlendRgb::BLEND1_RX_G_OFFSET) & BlendRgb::BLEND1_RX_G_MASK),
        }
    }

    /// Set **BLEND1_RX_B** `[7:0]`
    ///
    /// blue color for A4/A8 mode.
    #[inline(always)]
    pub const fn with_blend1_rx_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlendRgb::BLEND1_RX_B_MASK)
                | ((value << BlendRgb::BLEND1_RX_B_OFFSET) & BlendRgb::BLEND1_RX_B_MASK),
        }
    }
}

impl Default for BlendRgbVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BlendRgb {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BlendRgbVal {
        BlendRgbVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BlendRgbVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BlendRgbVal) -> BlendRgbVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BLEND_FIX_PIXEL Register
///
/// Blending engine fix pixel register
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **BLEND_TX_FIX_PIXEL** `[31:0]` (RW)
///
///   The configure fix pixel in fix pixel filling mode for blender engine.
#[repr(transparent)]
pub struct BlendFixPixel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BlendFixPixel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BlendFixPixel {
    // ======== Field Constants ========

    /// **BLEND_TX_FIX_PIXEL**
    ///
    /// The configure fix pixel in fix pixel filling mode for blender engine.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const BLEND_TX_FIX_PIXEL_OFFSET: u32 = 0;
    pub const BLEND_TX_FIX_PIXEL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **BLEND_TX_FIX_PIXEL** `[31:0]` (RW)
    ///
    /// The configure fix pixel in fix pixel filling mode for blender engine.
    #[inline(always)]
    pub fn blend_tx_fix_pixel(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND_TX_FIX_PIXEL_MASK) >> Self::BLEND_TX_FIX_PIXEL_OFFSET
    }

    /// Write **BLEND_TX_FIX_PIXEL** `[31:0]` (RW)
    ///
    /// The configure fix pixel in fix pixel filling mode for blender engine.
    #[inline(always)]
    pub fn write_blend_tx_fix_pixel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLEND_TX_FIX_PIXEL_MASK)
                | ((value << Self::BLEND_TX_FIX_PIXEL_OFFSET) & Self::BLEND_TX_FIX_PIXEL_MASK)
        });
    }
}

/// BLEND_FIX_PIXEL value type for batch read/modify/write operations
///
/// Use `blendfixpixel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `blendfixpixel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BlendFixPixelVal {
    bits: u32,
}

impl BlendFixPixelVal {
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

    /// Read **BLEND_TX_FIX_PIXEL** `[31:0]`
    ///
    /// The configure fix pixel in fix pixel filling mode for blender engine.
    #[inline(always)]
    pub const fn blend_tx_fix_pixel(&self) -> u32 {
        (self.bits & BlendFixPixel::BLEND_TX_FIX_PIXEL_MASK)
            >> BlendFixPixel::BLEND_TX_FIX_PIXEL_OFFSET
    }

    /// Set **BLEND_TX_FIX_PIXEL** `[31:0]`
    ///
    /// The configure fix pixel in fix pixel filling mode for blender engine.
    #[inline(always)]
    pub const fn with_blend_tx_fix_pixel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlendFixPixel::BLEND_TX_FIX_PIXEL_MASK)
                | ((value << BlendFixPixel::BLEND_TX_FIX_PIXEL_OFFSET)
                    & BlendFixPixel::BLEND_TX_FIX_PIXEL_MASK),
        }
    }
}

impl Default for BlendFixPixelVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BlendFixPixel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BlendFixPixelVal {
        BlendFixPixelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BlendFixPixelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BlendFixPixelVal) -> BlendFixPixelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CK_FG_LOW Register
///
/// foreground color key lower threshold
///
/// Address offset: 0x0050
/// Reset value: 0x00ffffff
///
/// ## Fields
///
/// - **COLORKEY_FG_R_LOW** `[23:16]` (RW)
///
///   color key lower threshold of foreground r channel
///
/// - **COLORKEY_FG_G_LOW** `[15:8]` (RW)
///
///   color key lower threshold of foreground g channel
///
/// - **COLORKEY_FG_B_LOW** `[7:0]` (RW)
///
///   color key lower threshold of foreground b channel
#[repr(transparent)]
pub struct CkFgLow {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CkFgLow {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CkFgLow {
    // ======== Field Constants ========

    /// **COLORKEY_FG_R_LOW**
    ///
    /// color key lower threshold of foreground r channel
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const COLORKEY_FG_R_LOW_OFFSET: u32 = 16;
    pub const COLORKEY_FG_R_LOW_MASK: u32 = 0x00ff0000;

    /// **COLORKEY_FG_G_LOW**
    ///
    /// color key lower threshold of foreground g channel
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const COLORKEY_FG_G_LOW_OFFSET: u32 = 8;
    pub const COLORKEY_FG_G_LOW_MASK: u32 = 0x0000ff00;

    /// **COLORKEY_FG_B_LOW**
    ///
    /// color key lower threshold of foreground b channel
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const COLORKEY_FG_B_LOW_OFFSET: u32 = 0;
    pub const COLORKEY_FG_B_LOW_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **COLORKEY_FG_R_LOW** `[23:16]` (RW)
    ///
    /// color key lower threshold of foreground r channel
    #[inline(always)]
    pub fn colorkey_fg_r_low(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_FG_R_LOW_MASK) >> Self::COLORKEY_FG_R_LOW_OFFSET
    }

    /// Write **COLORKEY_FG_R_LOW** `[23:16]` (RW)
    ///
    /// color key lower threshold of foreground r channel
    #[inline(always)]
    pub fn write_colorkey_fg_r_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_FG_R_LOW_MASK)
                | ((value << Self::COLORKEY_FG_R_LOW_OFFSET) & Self::COLORKEY_FG_R_LOW_MASK)
        });
    }

    /// **COLORKEY_FG_G_LOW** `[15:8]` (RW)
    ///
    /// color key lower threshold of foreground g channel
    #[inline(always)]
    pub fn colorkey_fg_g_low(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_FG_G_LOW_MASK) >> Self::COLORKEY_FG_G_LOW_OFFSET
    }

    /// Write **COLORKEY_FG_G_LOW** `[15:8]` (RW)
    ///
    /// color key lower threshold of foreground g channel
    #[inline(always)]
    pub fn write_colorkey_fg_g_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_FG_G_LOW_MASK)
                | ((value << Self::COLORKEY_FG_G_LOW_OFFSET) & Self::COLORKEY_FG_G_LOW_MASK)
        });
    }

    /// **COLORKEY_FG_B_LOW** `[7:0]` (RW)
    ///
    /// color key lower threshold of foreground b channel
    #[inline(always)]
    pub fn colorkey_fg_b_low(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_FG_B_LOW_MASK) >> Self::COLORKEY_FG_B_LOW_OFFSET
    }

    /// Write **COLORKEY_FG_B_LOW** `[7:0]` (RW)
    ///
    /// color key lower threshold of foreground b channel
    #[inline(always)]
    pub fn write_colorkey_fg_b_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_FG_B_LOW_MASK)
                | ((value << Self::COLORKEY_FG_B_LOW_OFFSET) & Self::COLORKEY_FG_B_LOW_MASK)
        });
    }
}

/// CK_FG_LOW value type for batch read/modify/write operations
///
/// Use `ckfglow.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ckfglow.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CkFgLowVal {
    bits: u32,
}

impl CkFgLowVal {
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
        Self { bits: 0x00ffffff }
    }

    /// Read **COLORKEY_FG_R_LOW** `[23:16]`
    ///
    /// color key lower threshold of foreground r channel
    #[inline(always)]
    pub const fn colorkey_fg_r_low(&self) -> u32 {
        (self.bits & CkFgLow::COLORKEY_FG_R_LOW_MASK) >> CkFgLow::COLORKEY_FG_R_LOW_OFFSET
    }

    /// Read **COLORKEY_FG_G_LOW** `[15:8]`
    ///
    /// color key lower threshold of foreground g channel
    #[inline(always)]
    pub const fn colorkey_fg_g_low(&self) -> u32 {
        (self.bits & CkFgLow::COLORKEY_FG_G_LOW_MASK) >> CkFgLow::COLORKEY_FG_G_LOW_OFFSET
    }

    /// Read **COLORKEY_FG_B_LOW** `[7:0]`
    ///
    /// color key lower threshold of foreground b channel
    #[inline(always)]
    pub const fn colorkey_fg_b_low(&self) -> u32 {
        (self.bits & CkFgLow::COLORKEY_FG_B_LOW_MASK) >> CkFgLow::COLORKEY_FG_B_LOW_OFFSET
    }

    /// Set **COLORKEY_FG_R_LOW** `[23:16]`
    ///
    /// color key lower threshold of foreground r channel
    #[inline(always)]
    pub const fn with_colorkey_fg_r_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkFgLow::COLORKEY_FG_R_LOW_MASK)
                | ((value << CkFgLow::COLORKEY_FG_R_LOW_OFFSET) & CkFgLow::COLORKEY_FG_R_LOW_MASK),
        }
    }

    /// Set **COLORKEY_FG_G_LOW** `[15:8]`
    ///
    /// color key lower threshold of foreground g channel
    #[inline(always)]
    pub const fn with_colorkey_fg_g_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkFgLow::COLORKEY_FG_G_LOW_MASK)
                | ((value << CkFgLow::COLORKEY_FG_G_LOW_OFFSET) & CkFgLow::COLORKEY_FG_G_LOW_MASK),
        }
    }

    /// Set **COLORKEY_FG_B_LOW** `[7:0]`
    ///
    /// color key lower threshold of foreground b channel
    #[inline(always)]
    pub const fn with_colorkey_fg_b_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkFgLow::COLORKEY_FG_B_LOW_MASK)
                | ((value << CkFgLow::COLORKEY_FG_B_LOW_OFFSET) & CkFgLow::COLORKEY_FG_B_LOW_MASK),
        }
    }
}

impl Default for CkFgLowVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CkFgLow {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CkFgLowVal {
        CkFgLowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CkFgLowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CkFgLowVal) -> CkFgLowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CK_FG_HIGH Register
///
/// foreground color key higher threshold
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **COLORKEY_FG_R_HIGH** `[23:16]` (RW)
///
///   color key higher threshold of foreground r channel
///
/// - **COLORKEY_FG_G_HIGH** `[15:8]` (RW)
///
///   color key higher threshold of foreground g channel
///
/// - **COLORKEY_FG_B_HIGH** `[7:0]` (RW)
///
///   color key higher threshold of foreground b channel
#[repr(transparent)]
pub struct CkFgHigh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CkFgHigh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CkFgHigh {
    // ======== Field Constants ========

    /// **COLORKEY_FG_R_HIGH**
    ///
    /// color key higher threshold of foreground r channel
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const COLORKEY_FG_R_HIGH_OFFSET: u32 = 16;
    pub const COLORKEY_FG_R_HIGH_MASK: u32 = 0x00ff0000;

    /// **COLORKEY_FG_G_HIGH**
    ///
    /// color key higher threshold of foreground g channel
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const COLORKEY_FG_G_HIGH_OFFSET: u32 = 8;
    pub const COLORKEY_FG_G_HIGH_MASK: u32 = 0x0000ff00;

    /// **COLORKEY_FG_B_HIGH**
    ///
    /// color key higher threshold of foreground b channel
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const COLORKEY_FG_B_HIGH_OFFSET: u32 = 0;
    pub const COLORKEY_FG_B_HIGH_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **COLORKEY_FG_R_HIGH** `[23:16]` (RW)
    ///
    /// color key higher threshold of foreground r channel
    #[inline(always)]
    pub fn colorkey_fg_r_high(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_FG_R_HIGH_MASK) >> Self::COLORKEY_FG_R_HIGH_OFFSET
    }

    /// Write **COLORKEY_FG_R_HIGH** `[23:16]` (RW)
    ///
    /// color key higher threshold of foreground r channel
    #[inline(always)]
    pub fn write_colorkey_fg_r_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_FG_R_HIGH_MASK)
                | ((value << Self::COLORKEY_FG_R_HIGH_OFFSET) & Self::COLORKEY_FG_R_HIGH_MASK)
        });
    }

    /// **COLORKEY_FG_G_HIGH** `[15:8]` (RW)
    ///
    /// color key higher threshold of foreground g channel
    #[inline(always)]
    pub fn colorkey_fg_g_high(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_FG_G_HIGH_MASK) >> Self::COLORKEY_FG_G_HIGH_OFFSET
    }

    /// Write **COLORKEY_FG_G_HIGH** `[15:8]` (RW)
    ///
    /// color key higher threshold of foreground g channel
    #[inline(always)]
    pub fn write_colorkey_fg_g_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_FG_G_HIGH_MASK)
                | ((value << Self::COLORKEY_FG_G_HIGH_OFFSET) & Self::COLORKEY_FG_G_HIGH_MASK)
        });
    }

    /// **COLORKEY_FG_B_HIGH** `[7:0]` (RW)
    ///
    /// color key higher threshold of foreground b channel
    #[inline(always)]
    pub fn colorkey_fg_b_high(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_FG_B_HIGH_MASK) >> Self::COLORKEY_FG_B_HIGH_OFFSET
    }

    /// Write **COLORKEY_FG_B_HIGH** `[7:0]` (RW)
    ///
    /// color key higher threshold of foreground b channel
    #[inline(always)]
    pub fn write_colorkey_fg_b_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_FG_B_HIGH_MASK)
                | ((value << Self::COLORKEY_FG_B_HIGH_OFFSET) & Self::COLORKEY_FG_B_HIGH_MASK)
        });
    }
}

/// CK_FG_HIGH value type for batch read/modify/write operations
///
/// Use `ckfghigh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ckfghigh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CkFgHighVal {
    bits: u32,
}

impl CkFgHighVal {
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

    /// Read **COLORKEY_FG_R_HIGH** `[23:16]`
    ///
    /// color key higher threshold of foreground r channel
    #[inline(always)]
    pub const fn colorkey_fg_r_high(&self) -> u32 {
        (self.bits & CkFgHigh::COLORKEY_FG_R_HIGH_MASK) >> CkFgHigh::COLORKEY_FG_R_HIGH_OFFSET
    }

    /// Read **COLORKEY_FG_G_HIGH** `[15:8]`
    ///
    /// color key higher threshold of foreground g channel
    #[inline(always)]
    pub const fn colorkey_fg_g_high(&self) -> u32 {
        (self.bits & CkFgHigh::COLORKEY_FG_G_HIGH_MASK) >> CkFgHigh::COLORKEY_FG_G_HIGH_OFFSET
    }

    /// Read **COLORKEY_FG_B_HIGH** `[7:0]`
    ///
    /// color key higher threshold of foreground b channel
    #[inline(always)]
    pub const fn colorkey_fg_b_high(&self) -> u32 {
        (self.bits & CkFgHigh::COLORKEY_FG_B_HIGH_MASK) >> CkFgHigh::COLORKEY_FG_B_HIGH_OFFSET
    }

    /// Set **COLORKEY_FG_R_HIGH** `[23:16]`
    ///
    /// color key higher threshold of foreground r channel
    #[inline(always)]
    pub const fn with_colorkey_fg_r_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkFgHigh::COLORKEY_FG_R_HIGH_MASK)
                | ((value << CkFgHigh::COLORKEY_FG_R_HIGH_OFFSET)
                    & CkFgHigh::COLORKEY_FG_R_HIGH_MASK),
        }
    }

    /// Set **COLORKEY_FG_G_HIGH** `[15:8]`
    ///
    /// color key higher threshold of foreground g channel
    #[inline(always)]
    pub const fn with_colorkey_fg_g_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkFgHigh::COLORKEY_FG_G_HIGH_MASK)
                | ((value << CkFgHigh::COLORKEY_FG_G_HIGH_OFFSET)
                    & CkFgHigh::COLORKEY_FG_G_HIGH_MASK),
        }
    }

    /// Set **COLORKEY_FG_B_HIGH** `[7:0]`
    ///
    /// color key higher threshold of foreground b channel
    #[inline(always)]
    pub const fn with_colorkey_fg_b_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkFgHigh::COLORKEY_FG_B_HIGH_MASK)
                | ((value << CkFgHigh::COLORKEY_FG_B_HIGH_OFFSET)
                    & CkFgHigh::COLORKEY_FG_B_HIGH_MASK),
        }
    }
}

impl Default for CkFgHighVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CkFgHigh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CkFgHighVal {
        CkFgHighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CkFgHighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CkFgHighVal) -> CkFgHighVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CK_BG_LOW Register
///
/// background color key lower threshold
///
/// Address offset: 0x0058
/// Reset value: 0x00ffffff
///
/// ## Fields
///
/// - **COLORKEY_BG_R_LOW** `[23:16]` (RW)
///
///   color key lower threshold of background r channel
///
/// - **COLORKEY_BG_G_LOW** `[15:8]` (RW)
///
///   color key lower threshold of background g channel
///
/// - **COLORKEY_BG_B_LOW** `[7:0]` (RW)
///
///   color key lower threshold of background b channel
#[repr(transparent)]
pub struct CkBgLow {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CkBgLow {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CkBgLow {
    // ======== Field Constants ========

    /// **COLORKEY_BG_R_LOW**
    ///
    /// color key lower threshold of background r channel
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const COLORKEY_BG_R_LOW_OFFSET: u32 = 16;
    pub const COLORKEY_BG_R_LOW_MASK: u32 = 0x00ff0000;

    /// **COLORKEY_BG_G_LOW**
    ///
    /// color key lower threshold of background g channel
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const COLORKEY_BG_G_LOW_OFFSET: u32 = 8;
    pub const COLORKEY_BG_G_LOW_MASK: u32 = 0x0000ff00;

    /// **COLORKEY_BG_B_LOW**
    ///
    /// color key lower threshold of background b channel
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const COLORKEY_BG_B_LOW_OFFSET: u32 = 0;
    pub const COLORKEY_BG_B_LOW_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **COLORKEY_BG_R_LOW** `[23:16]` (RW)
    ///
    /// color key lower threshold of background r channel
    #[inline(always)]
    pub fn colorkey_bg_r_low(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_BG_R_LOW_MASK) >> Self::COLORKEY_BG_R_LOW_OFFSET
    }

    /// Write **COLORKEY_BG_R_LOW** `[23:16]` (RW)
    ///
    /// color key lower threshold of background r channel
    #[inline(always)]
    pub fn write_colorkey_bg_r_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_BG_R_LOW_MASK)
                | ((value << Self::COLORKEY_BG_R_LOW_OFFSET) & Self::COLORKEY_BG_R_LOW_MASK)
        });
    }

    /// **COLORKEY_BG_G_LOW** `[15:8]` (RW)
    ///
    /// color key lower threshold of background g channel
    #[inline(always)]
    pub fn colorkey_bg_g_low(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_BG_G_LOW_MASK) >> Self::COLORKEY_BG_G_LOW_OFFSET
    }

    /// Write **COLORKEY_BG_G_LOW** `[15:8]` (RW)
    ///
    /// color key lower threshold of background g channel
    #[inline(always)]
    pub fn write_colorkey_bg_g_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_BG_G_LOW_MASK)
                | ((value << Self::COLORKEY_BG_G_LOW_OFFSET) & Self::COLORKEY_BG_G_LOW_MASK)
        });
    }

    /// **COLORKEY_BG_B_LOW** `[7:0]` (RW)
    ///
    /// color key lower threshold of background b channel
    #[inline(always)]
    pub fn colorkey_bg_b_low(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_BG_B_LOW_MASK) >> Self::COLORKEY_BG_B_LOW_OFFSET
    }

    /// Write **COLORKEY_BG_B_LOW** `[7:0]` (RW)
    ///
    /// color key lower threshold of background b channel
    #[inline(always)]
    pub fn write_colorkey_bg_b_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_BG_B_LOW_MASK)
                | ((value << Self::COLORKEY_BG_B_LOW_OFFSET) & Self::COLORKEY_BG_B_LOW_MASK)
        });
    }
}

/// CK_BG_LOW value type for batch read/modify/write operations
///
/// Use `ckbglow.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ckbglow.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CkBgLowVal {
    bits: u32,
}

impl CkBgLowVal {
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
        Self { bits: 0x00ffffff }
    }

    /// Read **COLORKEY_BG_R_LOW** `[23:16]`
    ///
    /// color key lower threshold of background r channel
    #[inline(always)]
    pub const fn colorkey_bg_r_low(&self) -> u32 {
        (self.bits & CkBgLow::COLORKEY_BG_R_LOW_MASK) >> CkBgLow::COLORKEY_BG_R_LOW_OFFSET
    }

    /// Read **COLORKEY_BG_G_LOW** `[15:8]`
    ///
    /// color key lower threshold of background g channel
    #[inline(always)]
    pub const fn colorkey_bg_g_low(&self) -> u32 {
        (self.bits & CkBgLow::COLORKEY_BG_G_LOW_MASK) >> CkBgLow::COLORKEY_BG_G_LOW_OFFSET
    }

    /// Read **COLORKEY_BG_B_LOW** `[7:0]`
    ///
    /// color key lower threshold of background b channel
    #[inline(always)]
    pub const fn colorkey_bg_b_low(&self) -> u32 {
        (self.bits & CkBgLow::COLORKEY_BG_B_LOW_MASK) >> CkBgLow::COLORKEY_BG_B_LOW_OFFSET
    }

    /// Set **COLORKEY_BG_R_LOW** `[23:16]`
    ///
    /// color key lower threshold of background r channel
    #[inline(always)]
    pub const fn with_colorkey_bg_r_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkBgLow::COLORKEY_BG_R_LOW_MASK)
                | ((value << CkBgLow::COLORKEY_BG_R_LOW_OFFSET) & CkBgLow::COLORKEY_BG_R_LOW_MASK),
        }
    }

    /// Set **COLORKEY_BG_G_LOW** `[15:8]`
    ///
    /// color key lower threshold of background g channel
    #[inline(always)]
    pub const fn with_colorkey_bg_g_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkBgLow::COLORKEY_BG_G_LOW_MASK)
                | ((value << CkBgLow::COLORKEY_BG_G_LOW_OFFSET) & CkBgLow::COLORKEY_BG_G_LOW_MASK),
        }
    }

    /// Set **COLORKEY_BG_B_LOW** `[7:0]`
    ///
    /// color key lower threshold of background b channel
    #[inline(always)]
    pub const fn with_colorkey_bg_b_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkBgLow::COLORKEY_BG_B_LOW_MASK)
                | ((value << CkBgLow::COLORKEY_BG_B_LOW_OFFSET) & CkBgLow::COLORKEY_BG_B_LOW_MASK),
        }
    }
}

impl Default for CkBgLowVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CkBgLow {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CkBgLowVal {
        CkBgLowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CkBgLowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CkBgLowVal) -> CkBgLowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CK_BG_HIGH Register
///
/// background color key higher threshold
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **COLORKEY_BG_R_HIGH** `[23:16]` (RW)
///
///   color key higher threshold of background r channel
///
/// - **COLORKEY_BG_G_HIGH** `[15:8]` (RW)
///
///   color key higher threshold of background g channel
///
/// - **COLORKEY_BG_B_HIGH** `[7:0]` (RW)
///
///   color key higher threshold of background b channel
#[repr(transparent)]
pub struct CkBgHigh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CkBgHigh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CkBgHigh {
    // ======== Field Constants ========

    /// **COLORKEY_BG_R_HIGH**
    ///
    /// color key higher threshold of background r channel
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const COLORKEY_BG_R_HIGH_OFFSET: u32 = 16;
    pub const COLORKEY_BG_R_HIGH_MASK: u32 = 0x00ff0000;

    /// **COLORKEY_BG_G_HIGH**
    ///
    /// color key higher threshold of background g channel
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const COLORKEY_BG_G_HIGH_OFFSET: u32 = 8;
    pub const COLORKEY_BG_G_HIGH_MASK: u32 = 0x0000ff00;

    /// **COLORKEY_BG_B_HIGH**
    ///
    /// color key higher threshold of background b channel
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const COLORKEY_BG_B_HIGH_OFFSET: u32 = 0;
    pub const COLORKEY_BG_B_HIGH_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **COLORKEY_BG_R_HIGH** `[23:16]` (RW)
    ///
    /// color key higher threshold of background r channel
    #[inline(always)]
    pub fn colorkey_bg_r_high(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_BG_R_HIGH_MASK) >> Self::COLORKEY_BG_R_HIGH_OFFSET
    }

    /// Write **COLORKEY_BG_R_HIGH** `[23:16]` (RW)
    ///
    /// color key higher threshold of background r channel
    #[inline(always)]
    pub fn write_colorkey_bg_r_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_BG_R_HIGH_MASK)
                | ((value << Self::COLORKEY_BG_R_HIGH_OFFSET) & Self::COLORKEY_BG_R_HIGH_MASK)
        });
    }

    /// **COLORKEY_BG_G_HIGH** `[15:8]` (RW)
    ///
    /// color key higher threshold of background g channel
    #[inline(always)]
    pub fn colorkey_bg_g_high(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_BG_G_HIGH_MASK) >> Self::COLORKEY_BG_G_HIGH_OFFSET
    }

    /// Write **COLORKEY_BG_G_HIGH** `[15:8]` (RW)
    ///
    /// color key higher threshold of background g channel
    #[inline(always)]
    pub fn write_colorkey_bg_g_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_BG_G_HIGH_MASK)
                | ((value << Self::COLORKEY_BG_G_HIGH_OFFSET) & Self::COLORKEY_BG_G_HIGH_MASK)
        });
    }

    /// **COLORKEY_BG_B_HIGH** `[7:0]` (RW)
    ///
    /// color key higher threshold of background b channel
    #[inline(always)]
    pub fn colorkey_bg_b_high(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_BG_B_HIGH_MASK) >> Self::COLORKEY_BG_B_HIGH_OFFSET
    }

    /// Write **COLORKEY_BG_B_HIGH** `[7:0]` (RW)
    ///
    /// color key higher threshold of background b channel
    #[inline(always)]
    pub fn write_colorkey_bg_b_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_BG_B_HIGH_MASK)
                | ((value << Self::COLORKEY_BG_B_HIGH_OFFSET) & Self::COLORKEY_BG_B_HIGH_MASK)
        });
    }
}

/// CK_BG_HIGH value type for batch read/modify/write operations
///
/// Use `ckbghigh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ckbghigh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CkBgHighVal {
    bits: u32,
}

impl CkBgHighVal {
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

    /// Read **COLORKEY_BG_R_HIGH** `[23:16]`
    ///
    /// color key higher threshold of background r channel
    #[inline(always)]
    pub const fn colorkey_bg_r_high(&self) -> u32 {
        (self.bits & CkBgHigh::COLORKEY_BG_R_HIGH_MASK) >> CkBgHigh::COLORKEY_BG_R_HIGH_OFFSET
    }

    /// Read **COLORKEY_BG_G_HIGH** `[15:8]`
    ///
    /// color key higher threshold of background g channel
    #[inline(always)]
    pub const fn colorkey_bg_g_high(&self) -> u32 {
        (self.bits & CkBgHigh::COLORKEY_BG_G_HIGH_MASK) >> CkBgHigh::COLORKEY_BG_G_HIGH_OFFSET
    }

    /// Read **COLORKEY_BG_B_HIGH** `[7:0]`
    ///
    /// color key higher threshold of background b channel
    #[inline(always)]
    pub const fn colorkey_bg_b_high(&self) -> u32 {
        (self.bits & CkBgHigh::COLORKEY_BG_B_HIGH_MASK) >> CkBgHigh::COLORKEY_BG_B_HIGH_OFFSET
    }

    /// Set **COLORKEY_BG_R_HIGH** `[23:16]`
    ///
    /// color key higher threshold of background r channel
    #[inline(always)]
    pub const fn with_colorkey_bg_r_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkBgHigh::COLORKEY_BG_R_HIGH_MASK)
                | ((value << CkBgHigh::COLORKEY_BG_R_HIGH_OFFSET)
                    & CkBgHigh::COLORKEY_BG_R_HIGH_MASK),
        }
    }

    /// Set **COLORKEY_BG_G_HIGH** `[15:8]`
    ///
    /// color key higher threshold of background g channel
    #[inline(always)]
    pub const fn with_colorkey_bg_g_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkBgHigh::COLORKEY_BG_G_HIGH_MASK)
                | ((value << CkBgHigh::COLORKEY_BG_G_HIGH_OFFSET)
                    & CkBgHigh::COLORKEY_BG_G_HIGH_MASK),
        }
    }

    /// Set **COLORKEY_BG_B_HIGH** `[7:0]`
    ///
    /// color key higher threshold of background b channel
    #[inline(always)]
    pub const fn with_colorkey_bg_b_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkBgHigh::COLORKEY_BG_B_HIGH_MASK)
                | ((value << CkBgHigh::COLORKEY_BG_B_HIGH_OFFSET)
                    & CkBgHigh::COLORKEY_BG_B_HIGH_MASK),
        }
    }
}

impl Default for CkBgHighVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CkBgHigh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CkBgHighVal {
        CkBgHighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CkBgHighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CkBgHighVal) -> CkBgHighVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CK_DEFAULT Register
///
/// default value when foreground and background both in color key range
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **COLORKEY_FG_BG_REVERSE** `[24]` (RW)
///
///   when pixel in bg ck range but not in fg ck range, 0: the result is bg, 1: the result is fg
///
/// - **COLORKEY_DEFAULT_R** `[23:16]` (RW)
///
///   default R channle value of color key
///
/// - **COLORKEY_DEFAULT_G** `[15:8]` (RW)
///
///   default G channle value of color key
///
/// - **COLORKEY_DEFAULT_B** `[7:0]` (RW)
///
///   default B channle value of color key
#[repr(transparent)]
pub struct CkDefault {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CkDefault {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CkDefault {
    // ======== Field Constants ========

    /// **COLORKEY_FG_BG_REVERSE**
    ///
    /// when pixel in bg ck range but not in fg ck range, 0: the result is bg, 1: the result is fg
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const COLORKEY_FG_BG_REVERSE_OFFSET: u32 = 24;
    pub const COLORKEY_FG_BG_REVERSE_MASK: u32 = 0x01000000;
    pub const COLORKEY_FG_BG_REVERSE: u32 = 1 << 24;

    /// **COLORKEY_DEFAULT_R**
    ///
    /// default R channle value of color key
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const COLORKEY_DEFAULT_R_OFFSET: u32 = 16;
    pub const COLORKEY_DEFAULT_R_MASK: u32 = 0x00ff0000;

    /// **COLORKEY_DEFAULT_G**
    ///
    /// default G channle value of color key
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const COLORKEY_DEFAULT_G_OFFSET: u32 = 8;
    pub const COLORKEY_DEFAULT_G_MASK: u32 = 0x0000ff00;

    /// **COLORKEY_DEFAULT_B**
    ///
    /// default B channle value of color key
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const COLORKEY_DEFAULT_B_OFFSET: u32 = 0;
    pub const COLORKEY_DEFAULT_B_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **COLORKEY_FG_BG_REVERSE** `[24]` (RW)
    ///
    /// when pixel in bg ck range but not in fg ck range, 0: the result is bg, 1: the result is fg
    #[inline(always)]
    pub fn colorkey_fg_bg_reverse(&self) -> bool {
        (self.value.read_bits() & Self::COLORKEY_FG_BG_REVERSE_MASK) != 0
    }

    /// Set COLORKEY_FG_BG_REVERSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_colorkey_fg_bg_reverse(&self) {
        self.value
            .modify_bits(|r| r | Self::COLORKEY_FG_BG_REVERSE_MASK);
    }

    /// Clear COLORKEY_FG_BG_REVERSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_colorkey_fg_bg_reverse(&self) {
        self.value
            .modify_bits(|r| r & !Self::COLORKEY_FG_BG_REVERSE_MASK);
    }

    /// **COLORKEY_DEFAULT_R** `[23:16]` (RW)
    ///
    /// default R channle value of color key
    #[inline(always)]
    pub fn colorkey_default_r(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_DEFAULT_R_MASK) >> Self::COLORKEY_DEFAULT_R_OFFSET
    }

    /// Write **COLORKEY_DEFAULT_R** `[23:16]` (RW)
    ///
    /// default R channle value of color key
    #[inline(always)]
    pub fn write_colorkey_default_r(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_DEFAULT_R_MASK)
                | ((value << Self::COLORKEY_DEFAULT_R_OFFSET) & Self::COLORKEY_DEFAULT_R_MASK)
        });
    }

    /// **COLORKEY_DEFAULT_G** `[15:8]` (RW)
    ///
    /// default G channle value of color key
    #[inline(always)]
    pub fn colorkey_default_g(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_DEFAULT_G_MASK) >> Self::COLORKEY_DEFAULT_G_OFFSET
    }

    /// Write **COLORKEY_DEFAULT_G** `[15:8]` (RW)
    ///
    /// default G channle value of color key
    #[inline(always)]
    pub fn write_colorkey_default_g(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_DEFAULT_G_MASK)
                | ((value << Self::COLORKEY_DEFAULT_G_OFFSET) & Self::COLORKEY_DEFAULT_G_MASK)
        });
    }

    /// **COLORKEY_DEFAULT_B** `[7:0]` (RW)
    ///
    /// default B channle value of color key
    #[inline(always)]
    pub fn colorkey_default_b(&self) -> u32 {
        (self.value.read_bits() & Self::COLORKEY_DEFAULT_B_MASK) >> Self::COLORKEY_DEFAULT_B_OFFSET
    }

    /// Write **COLORKEY_DEFAULT_B** `[7:0]` (RW)
    ///
    /// default B channle value of color key
    #[inline(always)]
    pub fn write_colorkey_default_b(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLORKEY_DEFAULT_B_MASK)
                | ((value << Self::COLORKEY_DEFAULT_B_OFFSET) & Self::COLORKEY_DEFAULT_B_MASK)
        });
    }
}

/// CK_DEFAULT value type for batch read/modify/write operations
///
/// Use `ckdefault.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ckdefault.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CkDefaultVal {
    bits: u32,
}

impl CkDefaultVal {
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

    /// Read **COLORKEY_FG_BG_REVERSE** `[24:24]`
    ///
    /// when pixel in bg ck range but not in fg ck range, 0: the result is bg, 1: the result is fg
    #[inline(always)]
    pub const fn colorkey_fg_bg_reverse(&self) -> bool {
        (self.bits & CkDefault::COLORKEY_FG_BG_REVERSE_MASK) != 0
    }

    /// Read **COLORKEY_DEFAULT_R** `[23:16]`
    ///
    /// default R channle value of color key
    #[inline(always)]
    pub const fn colorkey_default_r(&self) -> u32 {
        (self.bits & CkDefault::COLORKEY_DEFAULT_R_MASK) >> CkDefault::COLORKEY_DEFAULT_R_OFFSET
    }

    /// Read **COLORKEY_DEFAULT_G** `[15:8]`
    ///
    /// default G channle value of color key
    #[inline(always)]
    pub const fn colorkey_default_g(&self) -> u32 {
        (self.bits & CkDefault::COLORKEY_DEFAULT_G_MASK) >> CkDefault::COLORKEY_DEFAULT_G_OFFSET
    }

    /// Read **COLORKEY_DEFAULT_B** `[7:0]`
    ///
    /// default B channle value of color key
    #[inline(always)]
    pub const fn colorkey_default_b(&self) -> u32 {
        (self.bits & CkDefault::COLORKEY_DEFAULT_B_MASK) >> CkDefault::COLORKEY_DEFAULT_B_OFFSET
    }

    /// Set **COLORKEY_FG_BG_REVERSE** `[24:24]`
    ///
    /// when pixel in bg ck range but not in fg ck range, 0: the result is bg, 1: the result is fg
    #[inline(always)]
    pub const fn with_colorkey_fg_bg_reverse(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CkDefault::COLORKEY_FG_BG_REVERSE_MASK
            } else {
                self.bits & !CkDefault::COLORKEY_FG_BG_REVERSE_MASK
            },
        }
    }

    /// Set **COLORKEY_DEFAULT_R** `[23:16]`
    ///
    /// default R channle value of color key
    #[inline(always)]
    pub const fn with_colorkey_default_r(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkDefault::COLORKEY_DEFAULT_R_MASK)
                | ((value << CkDefault::COLORKEY_DEFAULT_R_OFFSET)
                    & CkDefault::COLORKEY_DEFAULT_R_MASK),
        }
    }

    /// Set **COLORKEY_DEFAULT_G** `[15:8]`
    ///
    /// default G channle value of color key
    #[inline(always)]
    pub const fn with_colorkey_default_g(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkDefault::COLORKEY_DEFAULT_G_MASK)
                | ((value << CkDefault::COLORKEY_DEFAULT_G_OFFSET)
                    & CkDefault::COLORKEY_DEFAULT_G_MASK),
        }
    }

    /// Set **COLORKEY_DEFAULT_B** `[7:0]`
    ///
    /// default B channle value of color key
    #[inline(always)]
    pub const fn with_colorkey_default_b(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CkDefault::COLORKEY_DEFAULT_B_MASK)
                | ((value << CkDefault::COLORKEY_DEFAULT_B_OFFSET)
                    & CkDefault::COLORKEY_DEFAULT_B_MASK),
        }
    }
}

impl Default for CkDefaultVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CkDefault {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CkDefaultVal {
        CkDefaultVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CkDefaultVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CkDefaultVal) -> CkDefaultVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SR_SCAL_ROTATE Register
///
/// Scaling and rotating coefficient register
///
/// Address offset: 0x0064
/// Reset value: 0x00001001
///
/// ## Fields
///
/// - **SR_MIRROR_Y** `[29]` (RW)
///
///   Image mirror in Y direction. 0: disable, 1: enable
///
/// - **SR_MIRROR_X** `[28]` (RW)
///
///   Image mirror in X direction. 0: disable, 1: enable
///
/// - **SCAL_ROTATE_START** `[27]` (WO)
///
///   Write 1 to enable scaling and rotating engine after parameter is configured.
///
/// - **SCAL_ROTATE_RST** `[26]` (RW)
///
///   Write 1 then write 0 to this bit to reset scaling and rotating engine.
///
/// - **SR_ROTATE_ANGLE** `[25:24]` (RW)
///
///   The rotate angle. 0: 0 degree. 1: 90 degree. 2: 180 degree. 3: 270 degree.
///
/// - **SR_SCAL_Y_FRAG** `[23:20]` (RW)
///
///   The fragment part of scaling coefficient in Y direction.
///
/// - **SR_SCAL_Y_INT** `[19:12]` (RW)
///
///   The integrated part of scaling coefficient in Y direction.
///
/// - **SR_SCAL_X_FRAG** `[11:8]` (RW)
///
///   The fragment part of scaling coefficient in X direction.
///
/// - **SR_SCAL_X_INT** `[7:0]` (RW)
///
///   The integrated part of scaling coefficient in X direction.
#[repr(transparent)]
pub struct SrScalRotate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SrScalRotate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SrScalRotate {
    // ======== Field Constants ========

    /// **SR_MIRROR_Y**
    ///
    /// Image mirror in Y direction. 0: disable, 1: enable
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SR_MIRROR_Y_OFFSET: u32 = 29;
    pub const SR_MIRROR_Y_MASK: u32 = 0x20000000;
    pub const SR_MIRROR_Y: u32 = 1 << 29;

    /// **SR_MIRROR_X**
    ///
    /// Image mirror in X direction. 0: disable, 1: enable
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SR_MIRROR_X_OFFSET: u32 = 28;
    pub const SR_MIRROR_X_MASK: u32 = 0x10000000;
    pub const SR_MIRROR_X: u32 = 1 << 28;

    /// **SCAL_ROTATE_START**
    ///
    /// Write 1 to enable scaling and rotating engine after parameter is configured.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const SCAL_ROTATE_START_OFFSET: u32 = 27;
    pub const SCAL_ROTATE_START_MASK: u32 = 0x08000000;
    pub const SCAL_ROTATE_START: u32 = 1 << 27;

    /// **SCAL_ROTATE_RST**
    ///
    /// Write 1 then write 0 to this bit to reset scaling and rotating engine.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const SCAL_ROTATE_RST_OFFSET: u32 = 26;
    pub const SCAL_ROTATE_RST_MASK: u32 = 0x04000000;
    pub const SCAL_ROTATE_RST: u32 = 1 << 26;

    /// **SR_ROTATE_ANGLE**
    ///
    /// The rotate angle. 0: 0 degree. 1: 90 degree. 2: 180 degree. 3: 270 degree.
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const SR_ROTATE_ANGLE_OFFSET: u32 = 24;
    pub const SR_ROTATE_ANGLE_MASK: u32 = 0x03000000;

    /// **SR_SCAL_Y_FRAG**
    ///
    /// The fragment part of scaling coefficient in Y direction.
    ///
    /// Bits `[23:20]`, mask `0x00f00000`
    pub const SR_SCAL_Y_FRAG_OFFSET: u32 = 20;
    pub const SR_SCAL_Y_FRAG_MASK: u32 = 0x00f00000;

    /// **SR_SCAL_Y_INT**
    ///
    /// The integrated part of scaling coefficient in Y direction.
    ///
    /// Bits `[19:12]`, mask `0x000ff000`
    pub const SR_SCAL_Y_INT_OFFSET: u32 = 12;
    pub const SR_SCAL_Y_INT_MASK: u32 = 0x000ff000;

    /// **SR_SCAL_X_FRAG**
    ///
    /// The fragment part of scaling coefficient in X direction.
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const SR_SCAL_X_FRAG_OFFSET: u32 = 8;
    pub const SR_SCAL_X_FRAG_MASK: u32 = 0x00000f00;

    /// **SR_SCAL_X_INT**
    ///
    /// The integrated part of scaling coefficient in X direction.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SR_SCAL_X_INT_OFFSET: u32 = 0;
    pub const SR_SCAL_X_INT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SR_MIRROR_Y** `[29]` (RW)
    ///
    /// Image mirror in Y direction. 0: disable, 1: enable
    #[inline(always)]
    pub fn sr_mirror_y(&self) -> bool {
        (self.value.read_bits() & Self::SR_MIRROR_Y_MASK) != 0
    }

    /// Set SR_MIRROR_Y bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_mirror_y(&self) {
        self.value.modify_bits(|r| r | Self::SR_MIRROR_Y_MASK);
    }

    /// Clear SR_MIRROR_Y bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_mirror_y(&self) {
        self.value.modify_bits(|r| r & !Self::SR_MIRROR_Y_MASK);
    }

    /// **SR_MIRROR_X** `[28]` (RW)
    ///
    /// Image mirror in X direction. 0: disable, 1: enable
    #[inline(always)]
    pub fn sr_mirror_x(&self) -> bool {
        (self.value.read_bits() & Self::SR_MIRROR_X_MASK) != 0
    }

    /// Set SR_MIRROR_X bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_mirror_x(&self) {
        self.value.modify_bits(|r| r | Self::SR_MIRROR_X_MASK);
    }

    /// Clear SR_MIRROR_X bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_mirror_x(&self) {
        self.value.modify_bits(|r| r & !Self::SR_MIRROR_X_MASK);
    }

    /// Trigger **SCAL_ROTATE_START** (WO)
    ///
    /// Write 1 to enable scaling and rotating engine after parameter is configured.
    #[inline(always)]
    pub fn trigger_scal_rotate_start(&self) {
        self.value.modify_bits(|r| r | Self::SCAL_ROTATE_START_MASK);
    }

    /// **SCAL_ROTATE_RST** `[26]` (RW)
    ///
    /// Write 1 then write 0 to this bit to reset scaling and rotating engine.
    #[inline(always)]
    pub fn scal_rotate_rst(&self) -> bool {
        (self.value.read_bits() & Self::SCAL_ROTATE_RST_MASK) != 0
    }

    /// Set SCAL_ROTATE_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scal_rotate_rst(&self) {
        self.value.modify_bits(|r| r | Self::SCAL_ROTATE_RST_MASK);
    }

    /// Clear SCAL_ROTATE_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scal_rotate_rst(&self) {
        self.value.modify_bits(|r| r & !Self::SCAL_ROTATE_RST_MASK);
    }

    /// **SR_ROTATE_ANGLE** `[25:24]` (RW)
    ///
    /// The rotate angle. 0: 0 degree. 1: 90 degree. 2: 180 degree. 3: 270 degree.
    #[inline(always)]
    pub fn sr_rotate_angle(&self) -> u32 {
        (self.value.read_bits() & Self::SR_ROTATE_ANGLE_MASK) >> Self::SR_ROTATE_ANGLE_OFFSET
    }

    /// Write **SR_ROTATE_ANGLE** `[25:24]` (RW)
    ///
    /// The rotate angle. 0: 0 degree. 1: 90 degree. 2: 180 degree. 3: 270 degree.
    #[inline(always)]
    pub fn write_sr_rotate_angle(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SR_ROTATE_ANGLE_MASK)
                | ((value << Self::SR_ROTATE_ANGLE_OFFSET) & Self::SR_ROTATE_ANGLE_MASK)
        });
    }

    /// **SR_SCAL_Y_FRAG** `[23:20]` (RW)
    ///
    /// The fragment part of scaling coefficient in Y direction.
    #[inline(always)]
    pub fn sr_scal_y_frag(&self) -> u32 {
        (self.value.read_bits() & Self::SR_SCAL_Y_FRAG_MASK) >> Self::SR_SCAL_Y_FRAG_OFFSET
    }

    /// Write **SR_SCAL_Y_FRAG** `[23:20]` (RW)
    ///
    /// The fragment part of scaling coefficient in Y direction.
    #[inline(always)]
    pub fn write_sr_scal_y_frag(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SR_SCAL_Y_FRAG_MASK)
                | ((value << Self::SR_SCAL_Y_FRAG_OFFSET) & Self::SR_SCAL_Y_FRAG_MASK)
        });
    }

    /// **SR_SCAL_Y_INT** `[19:12]` (RW)
    ///
    /// The integrated part of scaling coefficient in Y direction.
    #[inline(always)]
    pub fn sr_scal_y_int(&self) -> u32 {
        (self.value.read_bits() & Self::SR_SCAL_Y_INT_MASK) >> Self::SR_SCAL_Y_INT_OFFSET
    }

    /// Write **SR_SCAL_Y_INT** `[19:12]` (RW)
    ///
    /// The integrated part of scaling coefficient in Y direction.
    #[inline(always)]
    pub fn write_sr_scal_y_int(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SR_SCAL_Y_INT_MASK)
                | ((value << Self::SR_SCAL_Y_INT_OFFSET) & Self::SR_SCAL_Y_INT_MASK)
        });
    }

    /// **SR_SCAL_X_FRAG** `[11:8]` (RW)
    ///
    /// The fragment part of scaling coefficient in X direction.
    #[inline(always)]
    pub fn sr_scal_x_frag(&self) -> u32 {
        (self.value.read_bits() & Self::SR_SCAL_X_FRAG_MASK) >> Self::SR_SCAL_X_FRAG_OFFSET
    }

    /// Write **SR_SCAL_X_FRAG** `[11:8]` (RW)
    ///
    /// The fragment part of scaling coefficient in X direction.
    #[inline(always)]
    pub fn write_sr_scal_x_frag(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SR_SCAL_X_FRAG_MASK)
                | ((value << Self::SR_SCAL_X_FRAG_OFFSET) & Self::SR_SCAL_X_FRAG_MASK)
        });
    }

    /// **SR_SCAL_X_INT** `[7:0]` (RW)
    ///
    /// The integrated part of scaling coefficient in X direction.
    #[inline(always)]
    pub fn sr_scal_x_int(&self) -> u32 {
        (self.value.read_bits() & Self::SR_SCAL_X_INT_MASK) >> Self::SR_SCAL_X_INT_OFFSET
    }

    /// Write **SR_SCAL_X_INT** `[7:0]` (RW)
    ///
    /// The integrated part of scaling coefficient in X direction.
    #[inline(always)]
    pub fn write_sr_scal_x_int(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SR_SCAL_X_INT_MASK)
                | ((value << Self::SR_SCAL_X_INT_OFFSET) & Self::SR_SCAL_X_INT_MASK)
        });
    }
}

/// SR_SCAL_ROTATE value type for batch read/modify/write operations
///
/// Use `srscalrotate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `srscalrotate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SrScalRotateVal {
    bits: u32,
}

impl SrScalRotateVal {
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
        Self { bits: 0x00001001 }
    }

    /// Read **SR_MIRROR_Y** `[29:29]`
    ///
    /// Image mirror in Y direction. 0: disable, 1: enable
    #[inline(always)]
    pub const fn sr_mirror_y(&self) -> bool {
        (self.bits & SrScalRotate::SR_MIRROR_Y_MASK) != 0
    }

    /// Read **SR_MIRROR_X** `[28:28]`
    ///
    /// Image mirror in X direction. 0: disable, 1: enable
    #[inline(always)]
    pub const fn sr_mirror_x(&self) -> bool {
        (self.bits & SrScalRotate::SR_MIRROR_X_MASK) != 0
    }

    /// Read **SCAL_ROTATE_RST** `[26:26]`
    ///
    /// Write 1 then write 0 to this bit to reset scaling and rotating engine.
    #[inline(always)]
    pub const fn scal_rotate_rst(&self) -> bool {
        (self.bits & SrScalRotate::SCAL_ROTATE_RST_MASK) != 0
    }

    /// Read **SR_ROTATE_ANGLE** `[25:24]`
    ///
    /// The rotate angle. 0: 0 degree. 1: 90 degree. 2: 180 degree. 3: 270 degree.
    #[inline(always)]
    pub const fn sr_rotate_angle(&self) -> u32 {
        (self.bits & SrScalRotate::SR_ROTATE_ANGLE_MASK) >> SrScalRotate::SR_ROTATE_ANGLE_OFFSET
    }

    /// Read **SR_SCAL_Y_FRAG** `[23:20]`
    ///
    /// The fragment part of scaling coefficient in Y direction.
    #[inline(always)]
    pub const fn sr_scal_y_frag(&self) -> u32 {
        (self.bits & SrScalRotate::SR_SCAL_Y_FRAG_MASK) >> SrScalRotate::SR_SCAL_Y_FRAG_OFFSET
    }

    /// Read **SR_SCAL_Y_INT** `[19:12]`
    ///
    /// The integrated part of scaling coefficient in Y direction.
    #[inline(always)]
    pub const fn sr_scal_y_int(&self) -> u32 {
        (self.bits & SrScalRotate::SR_SCAL_Y_INT_MASK) >> SrScalRotate::SR_SCAL_Y_INT_OFFSET
    }

    /// Read **SR_SCAL_X_FRAG** `[11:8]`
    ///
    /// The fragment part of scaling coefficient in X direction.
    #[inline(always)]
    pub const fn sr_scal_x_frag(&self) -> u32 {
        (self.bits & SrScalRotate::SR_SCAL_X_FRAG_MASK) >> SrScalRotate::SR_SCAL_X_FRAG_OFFSET
    }

    /// Read **SR_SCAL_X_INT** `[7:0]`
    ///
    /// The integrated part of scaling coefficient in X direction.
    #[inline(always)]
    pub const fn sr_scal_x_int(&self) -> u32 {
        (self.bits & SrScalRotate::SR_SCAL_X_INT_MASK) >> SrScalRotate::SR_SCAL_X_INT_OFFSET
    }

    /// Set **SR_MIRROR_Y** `[29:29]`
    ///
    /// Image mirror in Y direction. 0: disable, 1: enable
    #[inline(always)]
    pub const fn with_sr_mirror_y(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrScalRotate::SR_MIRROR_Y_MASK
            } else {
                self.bits & !SrScalRotate::SR_MIRROR_Y_MASK
            },
        }
    }

    /// Set **SR_MIRROR_X** `[28:28]`
    ///
    /// Image mirror in X direction. 0: disable, 1: enable
    #[inline(always)]
    pub const fn with_sr_mirror_x(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrScalRotate::SR_MIRROR_X_MASK
            } else {
                self.bits & !SrScalRotate::SR_MIRROR_X_MASK
            },
        }
    }

    /// Set **SCAL_ROTATE_START** `[27:27]`
    ///
    /// Write 1 to enable scaling and rotating engine after parameter is configured.
    #[inline(always)]
    pub const fn with_scal_rotate_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrScalRotate::SCAL_ROTATE_START_MASK
            } else {
                self.bits & !SrScalRotate::SCAL_ROTATE_START_MASK
            },
        }
    }

    /// Set **SCAL_ROTATE_RST** `[26:26]`
    ///
    /// Write 1 then write 0 to this bit to reset scaling and rotating engine.
    #[inline(always)]
    pub const fn with_scal_rotate_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrScalRotate::SCAL_ROTATE_RST_MASK
            } else {
                self.bits & !SrScalRotate::SCAL_ROTATE_RST_MASK
            },
        }
    }

    /// Set **SR_ROTATE_ANGLE** `[25:24]`
    ///
    /// The rotate angle. 0: 0 degree. 1: 90 degree. 2: 180 degree. 3: 270 degree.
    #[inline(always)]
    pub const fn with_sr_rotate_angle(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SrScalRotate::SR_ROTATE_ANGLE_MASK)
                | ((value << SrScalRotate::SR_ROTATE_ANGLE_OFFSET)
                    & SrScalRotate::SR_ROTATE_ANGLE_MASK),
        }
    }

    /// Set **SR_SCAL_Y_FRAG** `[23:20]`
    ///
    /// The fragment part of scaling coefficient in Y direction.
    #[inline(always)]
    pub const fn with_sr_scal_y_frag(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SrScalRotate::SR_SCAL_Y_FRAG_MASK)
                | ((value << SrScalRotate::SR_SCAL_Y_FRAG_OFFSET)
                    & SrScalRotate::SR_SCAL_Y_FRAG_MASK),
        }
    }

    /// Set **SR_SCAL_Y_INT** `[19:12]`
    ///
    /// The integrated part of scaling coefficient in Y direction.
    #[inline(always)]
    pub const fn with_sr_scal_y_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SrScalRotate::SR_SCAL_Y_INT_MASK)
                | ((value << SrScalRotate::SR_SCAL_Y_INT_OFFSET)
                    & SrScalRotate::SR_SCAL_Y_INT_MASK),
        }
    }

    /// Set **SR_SCAL_X_FRAG** `[11:8]`
    ///
    /// The fragment part of scaling coefficient in X direction.
    #[inline(always)]
    pub const fn with_sr_scal_x_frag(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SrScalRotate::SR_SCAL_X_FRAG_MASK)
                | ((value << SrScalRotate::SR_SCAL_X_FRAG_OFFSET)
                    & SrScalRotate::SR_SCAL_X_FRAG_MASK),
        }
    }

    /// Set **SR_SCAL_X_INT** `[7:0]`
    ///
    /// The integrated part of scaling coefficient in X direction.
    #[inline(always)]
    pub const fn with_sr_scal_x_int(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SrScalRotate::SR_SCAL_X_INT_MASK)
                | ((value << SrScalRotate::SR_SCAL_X_INT_OFFSET)
                    & SrScalRotate::SR_SCAL_X_INT_MASK),
        }
    }
}

impl Default for SrScalRotateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SrScalRotate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SrScalRotateVal {
        SrScalRotateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SrScalRotateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SrScalRotateVal) -> SrScalRotateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SR_MEM_PD Register
///
/// SR memory power done register
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **SR_MEM_FORCE_PU** `[2]` (RW)
///
///   Set this bit to force power up scaling and rotating engine's data memory.
///
/// - **SR_MEM_FORCE_PD** `[1]` (RW)
///
///   Set this bit to force power down scaling and rotating engine's data memory.
///
/// - **SR_MEM_CLK_ENA** `[0]` (RW)
///
///   Set this bit to force clock enable of scaling and rotating engine's data memory.
#[repr(transparent)]
pub struct SrMemPd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SrMemPd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SrMemPd {
    // ======== Field Constants ========

    /// **SR_MEM_FORCE_PU**
    ///
    /// Set this bit to force power up scaling and rotating engine's data memory.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const SR_MEM_FORCE_PU_OFFSET: u32 = 2;
    pub const SR_MEM_FORCE_PU_MASK: u32 = 0x00000004;
    pub const SR_MEM_FORCE_PU: u32 = 1 << 2;

    /// **SR_MEM_FORCE_PD**
    ///
    /// Set this bit to force power down scaling and rotating engine's data memory.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SR_MEM_FORCE_PD_OFFSET: u32 = 1;
    pub const SR_MEM_FORCE_PD_MASK: u32 = 0x00000002;
    pub const SR_MEM_FORCE_PD: u32 = 1 << 1;

    /// **SR_MEM_CLK_ENA**
    ///
    /// Set this bit to force clock enable of scaling and rotating engine's data memory.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SR_MEM_CLK_ENA_OFFSET: u32 = 0;
    pub const SR_MEM_CLK_ENA_MASK: u32 = 0x00000001;
    pub const SR_MEM_CLK_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SR_MEM_FORCE_PU** `[2]` (RW)
    ///
    /// Set this bit to force power up scaling and rotating engine's data memory.
    #[inline(always)]
    pub fn sr_mem_force_pu(&self) -> bool {
        (self.value.read_bits() & Self::SR_MEM_FORCE_PU_MASK) != 0
    }

    /// Set SR_MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_mem_force_pu(&self) {
        self.value.modify_bits(|r| r | Self::SR_MEM_FORCE_PU_MASK);
    }

    /// Clear SR_MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_mem_force_pu(&self) {
        self.value.modify_bits(|r| r & !Self::SR_MEM_FORCE_PU_MASK);
    }

    /// **SR_MEM_FORCE_PD** `[1]` (RW)
    ///
    /// Set this bit to force power down scaling and rotating engine's data memory.
    #[inline(always)]
    pub fn sr_mem_force_pd(&self) -> bool {
        (self.value.read_bits() & Self::SR_MEM_FORCE_PD_MASK) != 0
    }

    /// Set SR_MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_mem_force_pd(&self) {
        self.value.modify_bits(|r| r | Self::SR_MEM_FORCE_PD_MASK);
    }

    /// Clear SR_MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_mem_force_pd(&self) {
        self.value.modify_bits(|r| r & !Self::SR_MEM_FORCE_PD_MASK);
    }

    /// **SR_MEM_CLK_ENA** `[0]` (RW)
    ///
    /// Set this bit to force clock enable of scaling and rotating engine's data memory.
    #[inline(always)]
    pub fn sr_mem_clk_ena(&self) -> bool {
        (self.value.read_bits() & Self::SR_MEM_CLK_ENA_MASK) != 0
    }

    /// Set SR_MEM_CLK_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_mem_clk_ena(&self) {
        self.value.modify_bits(|r| r | Self::SR_MEM_CLK_ENA_MASK);
    }

    /// Clear SR_MEM_CLK_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_mem_clk_ena(&self) {
        self.value.modify_bits(|r| r & !Self::SR_MEM_CLK_ENA_MASK);
    }
}

/// SR_MEM_PD value type for batch read/modify/write operations
///
/// Use `srmempd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `srmempd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SrMemPdVal {
    bits: u32,
}

impl SrMemPdVal {
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

    /// Read **SR_MEM_FORCE_PU** `[2:2]`
    ///
    /// Set this bit to force power up scaling and rotating engine's data memory.
    #[inline(always)]
    pub const fn sr_mem_force_pu(&self) -> bool {
        (self.bits & SrMemPd::SR_MEM_FORCE_PU_MASK) != 0
    }

    /// Read **SR_MEM_FORCE_PD** `[1:1]`
    ///
    /// Set this bit to force power down scaling and rotating engine's data memory.
    #[inline(always)]
    pub const fn sr_mem_force_pd(&self) -> bool {
        (self.bits & SrMemPd::SR_MEM_FORCE_PD_MASK) != 0
    }

    /// Read **SR_MEM_CLK_ENA** `[0:0]`
    ///
    /// Set this bit to force clock enable of scaling and rotating engine's data memory.
    #[inline(always)]
    pub const fn sr_mem_clk_ena(&self) -> bool {
        (self.bits & SrMemPd::SR_MEM_CLK_ENA_MASK) != 0
    }

    /// Set **SR_MEM_FORCE_PU** `[2:2]`
    ///
    /// Set this bit to force power up scaling and rotating engine's data memory.
    #[inline(always)]
    pub const fn with_sr_mem_force_pu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrMemPd::SR_MEM_FORCE_PU_MASK
            } else {
                self.bits & !SrMemPd::SR_MEM_FORCE_PU_MASK
            },
        }
    }

    /// Set **SR_MEM_FORCE_PD** `[1:1]`
    ///
    /// Set this bit to force power down scaling and rotating engine's data memory.
    #[inline(always)]
    pub const fn with_sr_mem_force_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrMemPd::SR_MEM_FORCE_PD_MASK
            } else {
                self.bits & !SrMemPd::SR_MEM_FORCE_PD_MASK
            },
        }
    }

    /// Set **SR_MEM_CLK_ENA** `[0:0]`
    ///
    /// Set this bit to force clock enable of scaling and rotating engine's data memory.
    #[inline(always)]
    pub const fn with_sr_mem_clk_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SrMemPd::SR_MEM_CLK_ENA_MASK
            } else {
                self.bits & !SrMemPd::SR_MEM_CLK_ENA_MASK
            },
        }
    }
}

impl Default for SrMemPdVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SrMemPd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SrMemPdVal {
        SrMemPdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SrMemPdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SrMemPdVal) -> SrMemPdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REG_CONF Register
///
/// Register clock enable register
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **CLK_EN** `[0]` (RW)
///
///   PPA register clock gate enable signal.
#[repr(transparent)]
pub struct RegConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegConf {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// PPA register clock gate enable signal.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_EN_OFFSET: u32 = 0;
    pub const CLK_EN_MASK: u32 = 0x00000001;
    pub const CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[0]` (RW)
    ///
    /// PPA register clock gate enable signal.
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

/// REG_CONF value type for batch read/modify/write operations
///
/// Use `regconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegConfVal {
    bits: u32,
}

impl RegConfVal {
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
    /// PPA register clock gate enable signal.
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & RegConf::CLK_EN_MASK) != 0
    }

    /// Set **CLK_EN** `[0:0]`
    ///
    /// PPA register clock gate enable signal.
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RegConf::CLK_EN_MASK
            } else {
                self.bits & !RegConf::CLK_EN_MASK
            },
        }
    }
}

impl Default for RegConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegConfVal {
        RegConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegConfVal) -> RegConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLUT_CNT Register
///
/// BLEND CLUT write counter register
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **BLEND1_CLUT_CNT** `[17:9]` (RO)
///
///   The write data counter of BLEND1 CLUT in fifo mode.
///
/// - **BLEND0_CLUT_CNT** `[8:0]` (RO)
///
///   The write data counter of BLEND0 CLUT in fifo mode.
#[repr(transparent)]
pub struct ClutCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClutCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClutCnt {
    // ======== Field Constants ========

    /// **BLEND1_CLUT_CNT**
    ///
    /// The write data counter of BLEND1 CLUT in fifo mode.
    ///
    /// Bits `[17:9]`, mask `0x0003fe00`
    pub const BLEND1_CLUT_CNT_OFFSET: u32 = 9;
    pub const BLEND1_CLUT_CNT_MASK: u32 = 0x0003fe00;

    /// **BLEND0_CLUT_CNT**
    ///
    /// The write data counter of BLEND0 CLUT in fifo mode.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const BLEND0_CLUT_CNT_OFFSET: u32 = 0;
    pub const BLEND0_CLUT_CNT_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **BLEND1_CLUT_CNT** `[17:9]` (RO)
    ///
    /// The write data counter of BLEND1 CLUT in fifo mode.
    #[inline(always)]
    pub fn blend1_clut_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND1_CLUT_CNT_MASK) >> Self::BLEND1_CLUT_CNT_OFFSET
    }

    /// **BLEND0_CLUT_CNT** `[8:0]` (RO)
    ///
    /// The write data counter of BLEND0 CLUT in fifo mode.
    #[inline(always)]
    pub fn blend0_clut_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::BLEND0_CLUT_CNT_MASK) >> Self::BLEND0_CLUT_CNT_OFFSET
    }
}

/// CLUT_CNT value type for batch read/modify/write operations
///
/// Use `clutcnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clutcnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClutCntVal {
    bits: u32,
}

impl ClutCntVal {
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

    /// Read **BLEND1_CLUT_CNT** `[17:9]`
    ///
    /// The write data counter of BLEND1 CLUT in fifo mode.
    #[inline(always)]
    pub const fn blend1_clut_cnt(&self) -> u32 {
        (self.bits & ClutCnt::BLEND1_CLUT_CNT_MASK) >> ClutCnt::BLEND1_CLUT_CNT_OFFSET
    }

    /// Read **BLEND0_CLUT_CNT** `[8:0]`
    ///
    /// The write data counter of BLEND0 CLUT in fifo mode.
    #[inline(always)]
    pub const fn blend0_clut_cnt(&self) -> u32 {
        (self.bits & ClutCnt::BLEND0_CLUT_CNT_MASK) >> ClutCnt::BLEND0_CLUT_CNT_OFFSET
    }
}

impl Default for ClutCntVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ClutCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClutCntVal {
        ClutCntVal::from_bits(self.value.read_bits())
    }
}

/// BLEND_ST Register
///
/// Blending engine status register
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **BLEND_SIZE_DIFF_ST** `[0]` (RO)
///
///   1: indicate the size of two image is different.
#[repr(transparent)]
pub struct BlendSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BlendSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BlendSt {
    // ======== Field Constants ========

    /// **BLEND_SIZE_DIFF_ST**
    ///
    /// 1: indicate the size of two image is different.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const BLEND_SIZE_DIFF_ST_OFFSET: u32 = 0;
    pub const BLEND_SIZE_DIFF_ST_MASK: u32 = 0x00000001;
    pub const BLEND_SIZE_DIFF_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BLEND_SIZE_DIFF_ST** `[0]` (RO)
    ///
    /// 1: indicate the size of two image is different.
    #[inline(always)]
    pub fn blend_size_diff_st(&self) -> bool {
        (self.value.read_bits() & Self::BLEND_SIZE_DIFF_ST_MASK) != 0
    }
}

/// BLEND_ST value type for batch read/modify/write operations
///
/// Use `blendst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `blendst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BlendStVal {
    bits: u32,
}

impl BlendStVal {
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

    /// Read **BLEND_SIZE_DIFF_ST** `[0:0]`
    ///
    /// 1: indicate the size of two image is different.
    #[inline(always)]
    pub const fn blend_size_diff_st(&self) -> bool {
        (self.bits & BlendSt::BLEND_SIZE_DIFF_ST_MASK) != 0
    }
}

impl Default for BlendStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BlendSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BlendStVal {
        BlendStVal::from_bits(self.value.read_bits())
    }
}

/// SR_PARAM_ERR_ST Register
///
/// Scaling and rotating coefficient error register
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **Y_YUV420_TX_SCALE_ERR_ST** `[13]` (RO)
///
///   The error is that the va/vb/y param in dma2d descriptor is an odd num when enable yuv420 tx
///
/// - **X_YUV420_TX_SCALE_ERR_ST** `[12]` (RO)
///
///   The error is that the ha/hb/x param in dma2d descriptor is an odd num when enable yuv420 tx
///
/// - **Y_YUV420_RX_SCALE_ERR_ST** `[11]` (RO)
///
///   The error is that the va/vb/y param in dma2d descriptor is an odd num when enable yuv420 rx
///
/// - **X_YUV420_RX_SCALE_ERR_ST** `[10]` (RO)
///
///   The error is that the ha/hb/x param in dma2d descriptor is an odd num when enable yuv420 rx
///
/// - **XDST_LEN_TOO_LARGE_ERR_ST** `[9]` (RO)
///
///   The error is that the scaled image height is larger than (2^13 - 1).
///
/// - **XDST_LEN_TOO_SAMLL_ERR_ST** `[8]` (RO)
///
///   The error is that the scaled image height is 0. For example. when source height is 14. scaled value is 1/16. and no rotate operation. then scaled height would be 0 as the result would be floored.
///
/// - **RX_DSCR_HB_ERR_ST** `[7]` (RO)
///
///   The error is that the HB in 2DDMA transmit descriptor plus the offset of X coordinate in 2DDMA transmit descriptor is larger than HA in 2DDMA transmit descriptor.
///
/// - **X_RX_SCAL_EQUAL_0_ERR_ST** `[6]` (RO)
///
///   The error is that the scaled image height is 0.
///
/// - **YDST_LEN_TOO_LARGE_ERR_ST** `[5]` (RO)
///
///   The error is that the scaled width is larger than (2^13 - 1).
///
/// - **YDST_LEN_TOO_SAMLL_ERR_ST** `[4]` (RO)
///
///   The error is that the scaled image width is 0. For example. when source width is 14. scaled value is 1/16. and no rotate operation. then scaled width would be 0 as the result would be floored.
///
/// - **RX_DSCR_VB_ERR_ST** `[3]` (RO)
///
///   The error is that VB in 2DDMA receive descriptor plus the offset of Y coordinate in 2DDMA transmit descriptor is larger than VA in 2DDMA transmit descriptor
///
/// - **Y_RX_SCAL_EQUAL_0_ERR_ST** `[2]` (RO)
///
///   The error is that the PPA_SR_SCAL_Y_INT and PPA_SR_CAL_Y_FRAG both are 0.
///
/// - **TX_DSCR_HB_ERR_ST** `[1]` (RO)
///
///   The error is that the scaled HB plus the offset of X coordinate in 2DDMA receive descriptor is larger than HA in 2DDMA receive descriptor.
///
/// - **TX_DSCR_VB_ERR_ST** `[0]` (RO)
///
///   The error is that the scaled VB plus the offset of Y coordinate in 2DDMA receive descriptor is larger than VA in 2DDMA receive descriptor.
#[repr(transparent)]
pub struct SrParamErrSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SrParamErrSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SrParamErrSt {
    // ======== Field Constants ========

    /// **Y_YUV420_TX_SCALE_ERR_ST**
    ///
    /// The error is that the va/vb/y param in dma2d descriptor is an odd num when enable yuv420 tx
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const Y_YUV420_TX_SCALE_ERR_ST_OFFSET: u32 = 13;
    pub const Y_YUV420_TX_SCALE_ERR_ST_MASK: u32 = 0x00002000;
    pub const Y_YUV420_TX_SCALE_ERR_ST: u32 = 1 << 13;

    /// **X_YUV420_TX_SCALE_ERR_ST**
    ///
    /// The error is that the ha/hb/x param in dma2d descriptor is an odd num when enable yuv420 tx
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const X_YUV420_TX_SCALE_ERR_ST_OFFSET: u32 = 12;
    pub const X_YUV420_TX_SCALE_ERR_ST_MASK: u32 = 0x00001000;
    pub const X_YUV420_TX_SCALE_ERR_ST: u32 = 1 << 12;

    /// **Y_YUV420_RX_SCALE_ERR_ST**
    ///
    /// The error is that the va/vb/y param in dma2d descriptor is an odd num when enable yuv420 rx
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const Y_YUV420_RX_SCALE_ERR_ST_OFFSET: u32 = 11;
    pub const Y_YUV420_RX_SCALE_ERR_ST_MASK: u32 = 0x00000800;
    pub const Y_YUV420_RX_SCALE_ERR_ST: u32 = 1 << 11;

    /// **X_YUV420_RX_SCALE_ERR_ST**
    ///
    /// The error is that the ha/hb/x param in dma2d descriptor is an odd num when enable yuv420 rx
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const X_YUV420_RX_SCALE_ERR_ST_OFFSET: u32 = 10;
    pub const X_YUV420_RX_SCALE_ERR_ST_MASK: u32 = 0x00000400;
    pub const X_YUV420_RX_SCALE_ERR_ST: u32 = 1 << 10;

    /// **XDST_LEN_TOO_LARGE_ERR_ST**
    ///
    /// The error is that the scaled image height is larger than (2^13 - 1).
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const XDST_LEN_TOO_LARGE_ERR_ST_OFFSET: u32 = 9;
    pub const XDST_LEN_TOO_LARGE_ERR_ST_MASK: u32 = 0x00000200;
    pub const XDST_LEN_TOO_LARGE_ERR_ST: u32 = 1 << 9;

    /// **XDST_LEN_TOO_SAMLL_ERR_ST**
    ///
    /// The error is that the scaled image height is 0. For example. when source height is 14. scaled value is 1/16. and no rotate operation. then scaled height would be 0 as the result would be floored.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const XDST_LEN_TOO_SAMLL_ERR_ST_OFFSET: u32 = 8;
    pub const XDST_LEN_TOO_SAMLL_ERR_ST_MASK: u32 = 0x00000100;
    pub const XDST_LEN_TOO_SAMLL_ERR_ST: u32 = 1 << 8;

    /// **RX_DSCR_HB_ERR_ST**
    ///
    /// The error is that the HB in 2DDMA transmit descriptor plus the offset of X coordinate in 2DDMA transmit descriptor is larger than HA in 2DDMA transmit descriptor.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RX_DSCR_HB_ERR_ST_OFFSET: u32 = 7;
    pub const RX_DSCR_HB_ERR_ST_MASK: u32 = 0x00000080;
    pub const RX_DSCR_HB_ERR_ST: u32 = 1 << 7;

    /// **X_RX_SCAL_EQUAL_0_ERR_ST**
    ///
    /// The error is that the scaled image height is 0.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const X_RX_SCAL_EQUAL_0_ERR_ST_OFFSET: u32 = 6;
    pub const X_RX_SCAL_EQUAL_0_ERR_ST_MASK: u32 = 0x00000040;
    pub const X_RX_SCAL_EQUAL_0_ERR_ST: u32 = 1 << 6;

    /// **YDST_LEN_TOO_LARGE_ERR_ST**
    ///
    /// The error is that the scaled width is larger than (2^13 - 1).
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const YDST_LEN_TOO_LARGE_ERR_ST_OFFSET: u32 = 5;
    pub const YDST_LEN_TOO_LARGE_ERR_ST_MASK: u32 = 0x00000020;
    pub const YDST_LEN_TOO_LARGE_ERR_ST: u32 = 1 << 5;

    /// **YDST_LEN_TOO_SAMLL_ERR_ST**
    ///
    /// The error is that the scaled image width is 0. For example. when source width is 14. scaled value is 1/16. and no rotate operation. then scaled width would be 0 as the result would be floored.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const YDST_LEN_TOO_SAMLL_ERR_ST_OFFSET: u32 = 4;
    pub const YDST_LEN_TOO_SAMLL_ERR_ST_MASK: u32 = 0x00000010;
    pub const YDST_LEN_TOO_SAMLL_ERR_ST: u32 = 1 << 4;

    /// **RX_DSCR_VB_ERR_ST**
    ///
    /// The error is that VB in 2DDMA receive descriptor plus the offset of Y coordinate in 2DDMA transmit descriptor is larger than VA in 2DDMA transmit descriptor
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RX_DSCR_VB_ERR_ST_OFFSET: u32 = 3;
    pub const RX_DSCR_VB_ERR_ST_MASK: u32 = 0x00000008;
    pub const RX_DSCR_VB_ERR_ST: u32 = 1 << 3;

    /// **Y_RX_SCAL_EQUAL_0_ERR_ST**
    ///
    /// The error is that the PPA_SR_SCAL_Y_INT and PPA_SR_CAL_Y_FRAG both are 0.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const Y_RX_SCAL_EQUAL_0_ERR_ST_OFFSET: u32 = 2;
    pub const Y_RX_SCAL_EQUAL_0_ERR_ST_MASK: u32 = 0x00000004;
    pub const Y_RX_SCAL_EQUAL_0_ERR_ST: u32 = 1 << 2;

    /// **TX_DSCR_HB_ERR_ST**
    ///
    /// The error is that the scaled HB plus the offset of X coordinate in 2DDMA receive descriptor is larger than HA in 2DDMA receive descriptor.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TX_DSCR_HB_ERR_ST_OFFSET: u32 = 1;
    pub const TX_DSCR_HB_ERR_ST_MASK: u32 = 0x00000002;
    pub const TX_DSCR_HB_ERR_ST: u32 = 1 << 1;

    /// **TX_DSCR_VB_ERR_ST**
    ///
    /// The error is that the scaled VB plus the offset of Y coordinate in 2DDMA receive descriptor is larger than VA in 2DDMA receive descriptor.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_DSCR_VB_ERR_ST_OFFSET: u32 = 0;
    pub const TX_DSCR_VB_ERR_ST_MASK: u32 = 0x00000001;
    pub const TX_DSCR_VB_ERR_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **Y_YUV420_TX_SCALE_ERR_ST** `[13]` (RO)
    ///
    /// The error is that the va/vb/y param in dma2d descriptor is an odd num when enable yuv420 tx
    #[inline(always)]
    pub fn y_yuv420_tx_scale_err_st(&self) -> bool {
        (self.value.read_bits() & Self::Y_YUV420_TX_SCALE_ERR_ST_MASK) != 0
    }

    /// **X_YUV420_TX_SCALE_ERR_ST** `[12]` (RO)
    ///
    /// The error is that the ha/hb/x param in dma2d descriptor is an odd num when enable yuv420 tx
    #[inline(always)]
    pub fn x_yuv420_tx_scale_err_st(&self) -> bool {
        (self.value.read_bits() & Self::X_YUV420_TX_SCALE_ERR_ST_MASK) != 0
    }

    /// **Y_YUV420_RX_SCALE_ERR_ST** `[11]` (RO)
    ///
    /// The error is that the va/vb/y param in dma2d descriptor is an odd num when enable yuv420 rx
    #[inline(always)]
    pub fn y_yuv420_rx_scale_err_st(&self) -> bool {
        (self.value.read_bits() & Self::Y_YUV420_RX_SCALE_ERR_ST_MASK) != 0
    }

    /// **X_YUV420_RX_SCALE_ERR_ST** `[10]` (RO)
    ///
    /// The error is that the ha/hb/x param in dma2d descriptor is an odd num when enable yuv420 rx
    #[inline(always)]
    pub fn x_yuv420_rx_scale_err_st(&self) -> bool {
        (self.value.read_bits() & Self::X_YUV420_RX_SCALE_ERR_ST_MASK) != 0
    }

    /// **XDST_LEN_TOO_LARGE_ERR_ST** `[9]` (RO)
    ///
    /// The error is that the scaled image height is larger than (2^13 - 1).
    #[inline(always)]
    pub fn xdst_len_too_large_err_st(&self) -> bool {
        (self.value.read_bits() & Self::XDST_LEN_TOO_LARGE_ERR_ST_MASK) != 0
    }

    /// **XDST_LEN_TOO_SAMLL_ERR_ST** `[8]` (RO)
    ///
    /// The error is that the scaled image height is 0. For example. when source height is 14. scaled value is 1/16. and no rotate operation. then scaled height would be 0 as the result would be floored.
    #[inline(always)]
    pub fn xdst_len_too_samll_err_st(&self) -> bool {
        (self.value.read_bits() & Self::XDST_LEN_TOO_SAMLL_ERR_ST_MASK) != 0
    }

    /// **RX_DSCR_HB_ERR_ST** `[7]` (RO)
    ///
    /// The error is that the HB in 2DDMA transmit descriptor plus the offset of X coordinate in 2DDMA transmit descriptor is larger than HA in 2DDMA transmit descriptor.
    #[inline(always)]
    pub fn rx_dscr_hb_err_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_DSCR_HB_ERR_ST_MASK) != 0
    }

    /// **X_RX_SCAL_EQUAL_0_ERR_ST** `[6]` (RO)
    ///
    /// The error is that the scaled image height is 0.
    #[inline(always)]
    pub fn x_rx_scal_equal_0_err_st(&self) -> bool {
        (self.value.read_bits() & Self::X_RX_SCAL_EQUAL_0_ERR_ST_MASK) != 0
    }

    /// **YDST_LEN_TOO_LARGE_ERR_ST** `[5]` (RO)
    ///
    /// The error is that the scaled width is larger than (2^13 - 1).
    #[inline(always)]
    pub fn ydst_len_too_large_err_st(&self) -> bool {
        (self.value.read_bits() & Self::YDST_LEN_TOO_LARGE_ERR_ST_MASK) != 0
    }

    /// **YDST_LEN_TOO_SAMLL_ERR_ST** `[4]` (RO)
    ///
    /// The error is that the scaled image width is 0. For example. when source width is 14. scaled value is 1/16. and no rotate operation. then scaled width would be 0 as the result would be floored.
    #[inline(always)]
    pub fn ydst_len_too_samll_err_st(&self) -> bool {
        (self.value.read_bits() & Self::YDST_LEN_TOO_SAMLL_ERR_ST_MASK) != 0
    }

    /// **RX_DSCR_VB_ERR_ST** `[3]` (RO)
    ///
    /// The error is that VB in 2DDMA receive descriptor plus the offset of Y coordinate in 2DDMA transmit descriptor is larger than VA in 2DDMA transmit descriptor
    #[inline(always)]
    pub fn rx_dscr_vb_err_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_DSCR_VB_ERR_ST_MASK) != 0
    }

    /// **Y_RX_SCAL_EQUAL_0_ERR_ST** `[2]` (RO)
    ///
    /// The error is that the PPA_SR_SCAL_Y_INT and PPA_SR_CAL_Y_FRAG both are 0.
    #[inline(always)]
    pub fn y_rx_scal_equal_0_err_st(&self) -> bool {
        (self.value.read_bits() & Self::Y_RX_SCAL_EQUAL_0_ERR_ST_MASK) != 0
    }

    /// **TX_DSCR_HB_ERR_ST** `[1]` (RO)
    ///
    /// The error is that the scaled HB plus the offset of X coordinate in 2DDMA receive descriptor is larger than HA in 2DDMA receive descriptor.
    #[inline(always)]
    pub fn tx_dscr_hb_err_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_DSCR_HB_ERR_ST_MASK) != 0
    }

    /// **TX_DSCR_VB_ERR_ST** `[0]` (RO)
    ///
    /// The error is that the scaled VB plus the offset of Y coordinate in 2DDMA receive descriptor is larger than VA in 2DDMA receive descriptor.
    #[inline(always)]
    pub fn tx_dscr_vb_err_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_DSCR_VB_ERR_ST_MASK) != 0
    }
}

/// SR_PARAM_ERR_ST value type for batch read/modify/write operations
///
/// Use `srparamerrst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `srparamerrst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SrParamErrStVal {
    bits: u32,
}

impl SrParamErrStVal {
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

    /// Read **Y_YUV420_TX_SCALE_ERR_ST** `[13:13]`
    ///
    /// The error is that the va/vb/y param in dma2d descriptor is an odd num when enable yuv420 tx
    #[inline(always)]
    pub const fn y_yuv420_tx_scale_err_st(&self) -> bool {
        (self.bits & SrParamErrSt::Y_YUV420_TX_SCALE_ERR_ST_MASK) != 0
    }

    /// Read **X_YUV420_TX_SCALE_ERR_ST** `[12:12]`
    ///
    /// The error is that the ha/hb/x param in dma2d descriptor is an odd num when enable yuv420 tx
    #[inline(always)]
    pub const fn x_yuv420_tx_scale_err_st(&self) -> bool {
        (self.bits & SrParamErrSt::X_YUV420_TX_SCALE_ERR_ST_MASK) != 0
    }

    /// Read **Y_YUV420_RX_SCALE_ERR_ST** `[11:11]`
    ///
    /// The error is that the va/vb/y param in dma2d descriptor is an odd num when enable yuv420 rx
    #[inline(always)]
    pub const fn y_yuv420_rx_scale_err_st(&self) -> bool {
        (self.bits & SrParamErrSt::Y_YUV420_RX_SCALE_ERR_ST_MASK) != 0
    }

    /// Read **X_YUV420_RX_SCALE_ERR_ST** `[10:10]`
    ///
    /// The error is that the ha/hb/x param in dma2d descriptor is an odd num when enable yuv420 rx
    #[inline(always)]
    pub const fn x_yuv420_rx_scale_err_st(&self) -> bool {
        (self.bits & SrParamErrSt::X_YUV420_RX_SCALE_ERR_ST_MASK) != 0
    }

    /// Read **XDST_LEN_TOO_LARGE_ERR_ST** `[9:9]`
    ///
    /// The error is that the scaled image height is larger than (2^13 - 1).
    #[inline(always)]
    pub const fn xdst_len_too_large_err_st(&self) -> bool {
        (self.bits & SrParamErrSt::XDST_LEN_TOO_LARGE_ERR_ST_MASK) != 0
    }

    /// Read **XDST_LEN_TOO_SAMLL_ERR_ST** `[8:8]`
    ///
    /// The error is that the scaled image height is 0. For example. when source height is 14. scaled value is 1/16. and no rotate operation. then scaled height would be 0 as the result would be floored.
    #[inline(always)]
    pub const fn xdst_len_too_samll_err_st(&self) -> bool {
        (self.bits & SrParamErrSt::XDST_LEN_TOO_SAMLL_ERR_ST_MASK) != 0
    }

    /// Read **RX_DSCR_HB_ERR_ST** `[7:7]`
    ///
    /// The error is that the HB in 2DDMA transmit descriptor plus the offset of X coordinate in 2DDMA transmit descriptor is larger than HA in 2DDMA transmit descriptor.
    #[inline(always)]
    pub const fn rx_dscr_hb_err_st(&self) -> bool {
        (self.bits & SrParamErrSt::RX_DSCR_HB_ERR_ST_MASK) != 0
    }

    /// Read **X_RX_SCAL_EQUAL_0_ERR_ST** `[6:6]`
    ///
    /// The error is that the scaled image height is 0.
    #[inline(always)]
    pub const fn x_rx_scal_equal_0_err_st(&self) -> bool {
        (self.bits & SrParamErrSt::X_RX_SCAL_EQUAL_0_ERR_ST_MASK) != 0
    }

    /// Read **YDST_LEN_TOO_LARGE_ERR_ST** `[5:5]`
    ///
    /// The error is that the scaled width is larger than (2^13 - 1).
    #[inline(always)]
    pub const fn ydst_len_too_large_err_st(&self) -> bool {
        (self.bits & SrParamErrSt::YDST_LEN_TOO_LARGE_ERR_ST_MASK) != 0
    }

    /// Read **YDST_LEN_TOO_SAMLL_ERR_ST** `[4:4]`
    ///
    /// The error is that the scaled image width is 0. For example. when source width is 14. scaled value is 1/16. and no rotate operation. then scaled width would be 0 as the result would be floored.
    #[inline(always)]
    pub const fn ydst_len_too_samll_err_st(&self) -> bool {
        (self.bits & SrParamErrSt::YDST_LEN_TOO_SAMLL_ERR_ST_MASK) != 0
    }

    /// Read **RX_DSCR_VB_ERR_ST** `[3:3]`
    ///
    /// The error is that VB in 2DDMA receive descriptor plus the offset of Y coordinate in 2DDMA transmit descriptor is larger than VA in 2DDMA transmit descriptor
    #[inline(always)]
    pub const fn rx_dscr_vb_err_st(&self) -> bool {
        (self.bits & SrParamErrSt::RX_DSCR_VB_ERR_ST_MASK) != 0
    }

    /// Read **Y_RX_SCAL_EQUAL_0_ERR_ST** `[2:2]`
    ///
    /// The error is that the PPA_SR_SCAL_Y_INT and PPA_SR_CAL_Y_FRAG both are 0.
    #[inline(always)]
    pub const fn y_rx_scal_equal_0_err_st(&self) -> bool {
        (self.bits & SrParamErrSt::Y_RX_SCAL_EQUAL_0_ERR_ST_MASK) != 0
    }

    /// Read **TX_DSCR_HB_ERR_ST** `[1:1]`
    ///
    /// The error is that the scaled HB plus the offset of X coordinate in 2DDMA receive descriptor is larger than HA in 2DDMA receive descriptor.
    #[inline(always)]
    pub const fn tx_dscr_hb_err_st(&self) -> bool {
        (self.bits & SrParamErrSt::TX_DSCR_HB_ERR_ST_MASK) != 0
    }

    /// Read **TX_DSCR_VB_ERR_ST** `[0:0]`
    ///
    /// The error is that the scaled VB plus the offset of Y coordinate in 2DDMA receive descriptor is larger than VA in 2DDMA receive descriptor.
    #[inline(always)]
    pub const fn tx_dscr_vb_err_st(&self) -> bool {
        (self.bits & SrParamErrSt::TX_DSCR_VB_ERR_ST_MASK) != 0
    }
}

impl Default for SrParamErrStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SrParamErrSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SrParamErrStVal {
        SrParamErrStVal::from_bits(self.value.read_bits())
    }
}

/// SR_STATUS Register
///
/// SR FSM register
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **SR_TX_SCAN_STATE** `[8:6]` (RO)
///
///   Reserved.
///
/// - **SR_TX_DSCR_SAMPLE_STATE** `[5:4]` (RO)
///
///   Reserved.
///
/// - **SR_RX_SCAN_STATE** `[3:2]` (RO)
///
///   Reserved.
///
/// - **SR_RX_DSCR_SAMPLE_STATE** `[1:0]` (RO)
///
///   Reserved.
#[repr(transparent)]
pub struct SrStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SrStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SrStatus {
    // ======== Field Constants ========

    /// **SR_TX_SCAN_STATE**
    ///
    /// Reserved.
    ///
    /// Bits `[8:6]`, mask `0x000001c0`
    pub const SR_TX_SCAN_STATE_OFFSET: u32 = 6;
    pub const SR_TX_SCAN_STATE_MASK: u32 = 0x000001c0;

    /// **SR_TX_DSCR_SAMPLE_STATE**
    ///
    /// Reserved.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const SR_TX_DSCR_SAMPLE_STATE_OFFSET: u32 = 4;
    pub const SR_TX_DSCR_SAMPLE_STATE_MASK: u32 = 0x00000030;

    /// **SR_RX_SCAN_STATE**
    ///
    /// Reserved.
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const SR_RX_SCAN_STATE_OFFSET: u32 = 2;
    pub const SR_RX_SCAN_STATE_MASK: u32 = 0x0000000c;

    /// **SR_RX_DSCR_SAMPLE_STATE**
    ///
    /// Reserved.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const SR_RX_DSCR_SAMPLE_STATE_OFFSET: u32 = 0;
    pub const SR_RX_DSCR_SAMPLE_STATE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SR_TX_SCAN_STATE** `[8:6]` (RO)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn sr_tx_scan_state(&self) -> u32 {
        (self.value.read_bits() & Self::SR_TX_SCAN_STATE_MASK) >> Self::SR_TX_SCAN_STATE_OFFSET
    }

    /// **SR_TX_DSCR_SAMPLE_STATE** `[5:4]` (RO)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn sr_tx_dscr_sample_state(&self) -> u32 {
        (self.value.read_bits() & Self::SR_TX_DSCR_SAMPLE_STATE_MASK)
            >> Self::SR_TX_DSCR_SAMPLE_STATE_OFFSET
    }

    /// **SR_RX_SCAN_STATE** `[3:2]` (RO)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn sr_rx_scan_state(&self) -> u32 {
        (self.value.read_bits() & Self::SR_RX_SCAN_STATE_MASK) >> Self::SR_RX_SCAN_STATE_OFFSET
    }

    /// **SR_RX_DSCR_SAMPLE_STATE** `[1:0]` (RO)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn sr_rx_dscr_sample_state(&self) -> u32 {
        (self.value.read_bits() & Self::SR_RX_DSCR_SAMPLE_STATE_MASK)
            >> Self::SR_RX_DSCR_SAMPLE_STATE_OFFSET
    }
}

/// SR_STATUS value type for batch read/modify/write operations
///
/// Use `srstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `srstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SrStatusVal {
    bits: u32,
}

impl SrStatusVal {
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

    /// Read **SR_TX_SCAN_STATE** `[8:6]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn sr_tx_scan_state(&self) -> u32 {
        (self.bits & SrStatus::SR_TX_SCAN_STATE_MASK) >> SrStatus::SR_TX_SCAN_STATE_OFFSET
    }

    /// Read **SR_TX_DSCR_SAMPLE_STATE** `[5:4]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn sr_tx_dscr_sample_state(&self) -> u32 {
        (self.bits & SrStatus::SR_TX_DSCR_SAMPLE_STATE_MASK)
            >> SrStatus::SR_TX_DSCR_SAMPLE_STATE_OFFSET
    }

    /// Read **SR_RX_SCAN_STATE** `[3:2]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn sr_rx_scan_state(&self) -> u32 {
        (self.bits & SrStatus::SR_RX_SCAN_STATE_MASK) >> SrStatus::SR_RX_SCAN_STATE_OFFSET
    }

    /// Read **SR_RX_DSCR_SAMPLE_STATE** `[1:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn sr_rx_dscr_sample_state(&self) -> u32 {
        (self.bits & SrStatus::SR_RX_DSCR_SAMPLE_STATE_MASK)
            >> SrStatus::SR_RX_DSCR_SAMPLE_STATE_OFFSET
    }
}

impl Default for SrStatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SrStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SrStatusVal {
        SrStatusVal::from_bits(self.value.read_bits())
    }
}

/// ECO_LOW Register
///
/// Reserved.
///
/// Address offset: 0x0080
///
/// ## Fields
///
/// - **RND_ECO_LOW** `[31:0]` (RW)
///
///   Reserved.
#[repr(transparent)]
pub struct EcoLow {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EcoLow {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EcoLow {
    // ======== Field Constants ========

    /// **RND_ECO_LOW**
    ///
    /// Reserved.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RND_ECO_LOW_OFFSET: u32 = 0;
    pub const RND_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RND_ECO_LOW** `[31:0]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn rnd_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::RND_ECO_LOW_MASK) >> Self::RND_ECO_LOW_OFFSET
    }

    /// Write **RND_ECO_LOW** `[31:0]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn write_rnd_eco_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RND_ECO_LOW_MASK)
                | ((value << Self::RND_ECO_LOW_OFFSET) & Self::RND_ECO_LOW_MASK)
        });
    }
}

/// ECO_LOW value type for batch read/modify/write operations
///
/// Use `ecolow.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ecolow.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EcoLowVal {
    bits: u32,
}

impl EcoLowVal {
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

    /// Read **RND_ECO_LOW** `[31:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn rnd_eco_low(&self) -> u32 {
        (self.bits & EcoLow::RND_ECO_LOW_MASK) >> EcoLow::RND_ECO_LOW_OFFSET
    }

    /// Set **RND_ECO_LOW** `[31:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn with_rnd_eco_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EcoLow::RND_ECO_LOW_MASK)
                | ((value << EcoLow::RND_ECO_LOW_OFFSET) & EcoLow::RND_ECO_LOW_MASK),
        }
    }
}

impl Default for EcoLowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EcoLow {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EcoLowVal {
        EcoLowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EcoLowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EcoLowVal) -> EcoLowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ECO_HIGH Register
///
/// Reserved.
///
/// Address offset: 0x0084
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **RND_ECO_HIGH** `[31:0]` (RW)
///
///   Reserved.
#[repr(transparent)]
pub struct EcoHigh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EcoHigh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EcoHigh {
    // ======== Field Constants ========

    /// **RND_ECO_HIGH**
    ///
    /// Reserved.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RND_ECO_HIGH_OFFSET: u32 = 0;
    pub const RND_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RND_ECO_HIGH** `[31:0]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn rnd_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::RND_ECO_HIGH_MASK) >> Self::RND_ECO_HIGH_OFFSET
    }

    /// Write **RND_ECO_HIGH** `[31:0]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn write_rnd_eco_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RND_ECO_HIGH_MASK)
                | ((value << Self::RND_ECO_HIGH_OFFSET) & Self::RND_ECO_HIGH_MASK)
        });
    }
}

/// ECO_HIGH value type for batch read/modify/write operations
///
/// Use `ecohigh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ecohigh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EcoHighVal {
    bits: u32,
}

impl EcoHighVal {
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
        Self { bits: 0xffffffff }
    }

    /// Read **RND_ECO_HIGH** `[31:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn rnd_eco_high(&self) -> u32 {
        (self.bits & EcoHigh::RND_ECO_HIGH_MASK) >> EcoHigh::RND_ECO_HIGH_OFFSET
    }

    /// Set **RND_ECO_HIGH** `[31:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn with_rnd_eco_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EcoHigh::RND_ECO_HIGH_MASK)
                | ((value << EcoHigh::RND_ECO_HIGH_OFFSET) & EcoHigh::RND_ECO_HIGH_MASK),
        }
    }
}

impl Default for EcoHighVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EcoHigh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EcoHighVal {
        EcoHighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EcoHighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EcoHighVal) -> EcoHighVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ECO_CELL_CTRL Register
///
/// Reserved.
///
/// Address offset: 0x0088
///
/// ## Fields
///
/// - **RDN_ENA** `[1]` (RW)
///
///   Reserved.
///
/// - **RDN_RESULT** `[0]` (RO)
///
///   Reserved.
#[repr(transparent)]
pub struct EcoCellCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EcoCellCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EcoCellCtrl {
    // ======== Field Constants ========

    /// **RDN_ENA**
    ///
    /// Reserved.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RDN_ENA_OFFSET: u32 = 1;
    pub const RDN_ENA_MASK: u32 = 0x00000002;
    pub const RDN_ENA: u32 = 1 << 1;

    /// **RDN_RESULT**
    ///
    /// Reserved.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RDN_RESULT_OFFSET: u32 = 0;
    pub const RDN_RESULT_MASK: u32 = 0x00000001;
    pub const RDN_RESULT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RDN_ENA** `[1]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn rdn_ena(&self) -> bool {
        (self.value.read_bits() & Self::RDN_ENA_MASK) != 0
    }

    /// Set RDN_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rdn_ena(&self) {
        self.value.modify_bits(|r| r | Self::RDN_ENA_MASK);
    }

    /// Clear RDN_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rdn_ena(&self) {
        self.value.modify_bits(|r| r & !Self::RDN_ENA_MASK);
    }

    /// **RDN_RESULT** `[0]` (RO)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn rdn_result(&self) -> bool {
        (self.value.read_bits() & Self::RDN_RESULT_MASK) != 0
    }
}

/// ECO_CELL_CTRL value type for batch read/modify/write operations
///
/// Use `ecocellctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ecocellctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EcoCellCtrlVal {
    bits: u32,
}

impl EcoCellCtrlVal {
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

    /// Read **RDN_ENA** `[1:1]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn rdn_ena(&self) -> bool {
        (self.bits & EcoCellCtrl::RDN_ENA_MASK) != 0
    }

    /// Read **RDN_RESULT** `[0:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn rdn_result(&self) -> bool {
        (self.bits & EcoCellCtrl::RDN_RESULT_MASK) != 0
    }

    /// Set **RDN_ENA** `[1:1]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn with_rdn_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EcoCellCtrl::RDN_ENA_MASK
            } else {
                self.bits & !EcoCellCtrl::RDN_ENA_MASK
            },
        }
    }
}

impl Default for EcoCellCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EcoCellCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EcoCellCtrlVal {
        EcoCellCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EcoCellCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EcoCellCtrlVal) -> EcoCellCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SRAM_CTRL Register
///
/// PPA SRAM Control Register
///
/// Address offset: 0x008c
/// Reset value: 0x00001320
///
/// ## Fields
///
/// - **MEM_AUX_CTRL** `[13:0]` (RW)
///
///   Control signals
#[repr(transparent)]
pub struct SramCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SramCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SramCtrl {
    // ======== Field Constants ========

    /// **MEM_AUX_CTRL**
    ///
    /// Control signals
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const MEM_AUX_CTRL_OFFSET: u32 = 0;
    pub const MEM_AUX_CTRL_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **MEM_AUX_CTRL** `[13:0]` (RW)
    ///
    /// Control signals
    #[inline(always)]
    pub fn mem_aux_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::MEM_AUX_CTRL_MASK) >> Self::MEM_AUX_CTRL_OFFSET
    }

    /// Write **MEM_AUX_CTRL** `[13:0]` (RW)
    ///
    /// Control signals
    #[inline(always)]
    pub fn write_mem_aux_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MEM_AUX_CTRL_MASK)
                | ((value << Self::MEM_AUX_CTRL_OFFSET) & Self::MEM_AUX_CTRL_MASK)
        });
    }
}

/// SRAM_CTRL value type for batch read/modify/write operations
///
/// Use `sramctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sramctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SramCtrlVal {
    bits: u32,
}

impl SramCtrlVal {
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
        Self { bits: 0x00001320 }
    }

    /// Read **MEM_AUX_CTRL** `[13:0]`
    ///
    /// Control signals
    #[inline(always)]
    pub const fn mem_aux_ctrl(&self) -> u32 {
        (self.bits & SramCtrl::MEM_AUX_CTRL_MASK) >> SramCtrl::MEM_AUX_CTRL_OFFSET
    }

    /// Set **MEM_AUX_CTRL** `[13:0]`
    ///
    /// Control signals
    #[inline(always)]
    pub const fn with_mem_aux_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SramCtrl::MEM_AUX_CTRL_MASK)
                | ((value << SramCtrl::MEM_AUX_CTRL_OFFSET) & SramCtrl::MEM_AUX_CTRL_MASK),
        }
    }
}

impl Default for SramCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SramCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SramCtrlVal {
        SramCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SramCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SramCtrlVal) -> SramCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// PPA Version register
///
/// Address offset: 0x0100
/// Reset value: 0x02304041
///
/// ## Fields
///
/// - **DATE** `[31:0]` (RW)
///
///   register version.
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
    /// register version.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DATE** `[31:0]` (RW)
    ///
    /// register version.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[31:0]` (RW)
    ///
    /// register version.
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
        Self { bits: 0x02304041 }
    }

    /// Read **DATE** `[31:0]`
    ///
    /// register version.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[31:0]`
    ///
    /// register version.
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
    /// CLUT sram data read/write register in background plane of blender
    pub blend0_clut_data: Blend0ClutData,
    /// 0x0004
    /// CLUT sram data read/write register in foreground plane of blender
    pub blend1_clut_data: Blend1ClutData,
    _reserved0: [u8; 4],
    /// 0x000c
    /// CLUT configure register
    pub clut_conf: ClutConf,
    /// 0x0010
    /// Raw status interrupt
    pub int_raw: IntRaw,
    /// 0x0014
    /// Masked interrupt
    pub int_st: IntSt,
    /// 0x0018
    /// Interrupt enable bits
    pub int_ena: IntEna,
    /// 0x001c
    /// Interrupt clear bits
    pub int_clr: IntClr,
    /// 0x0020
    /// Scaling and rotating engine color mode register
    pub sr_color_mode: SrColorMode,
    /// 0x0024
    /// blending engine color mode register
    pub blend_color_mode: BlendColorMode,
    /// 0x0028
    /// Scaling and rotating engine byte order register
    pub sr_byte_order: SrByteOrder,
    /// 0x002c
    /// Blending engine byte order register
    pub blend_byte_order: BlendByteOrder,
    _reserved1: [u8; 4],
    /// 0x0034
    /// Blending engine mode configure register
    pub blend_trans_mode: BlendTransMode,
    /// 0x0038
    /// Scaling and rotating engine alpha override register
    pub sr_fix_alpha: SrFixAlpha,
    /// 0x003c
    /// Fix pixel filling mode image size register
    pub blend_tx_size: BlendTxSize,
    /// 0x0040
    /// Blending engine alpha override register
    pub blend_fix_alpha: BlendFixAlpha,
    _reserved2: [u8; 4],
    /// 0x0048
    /// RGB color register
    pub blend_rgb: BlendRgb,
    /// 0x004c
    /// Blending engine fix pixel register
    pub blend_fix_pixel: BlendFixPixel,
    /// 0x0050
    /// foreground color key lower threshold
    pub ck_fg_low: CkFgLow,
    /// 0x0054
    /// foreground color key higher threshold
    pub ck_fg_high: CkFgHigh,
    /// 0x0058
    /// background color key lower threshold
    pub ck_bg_low: CkBgLow,
    /// 0x005c
    /// background color key higher threshold
    pub ck_bg_high: CkBgHigh,
    /// 0x0060
    /// default value when foreground and background both in color key range
    pub ck_default: CkDefault,
    /// 0x0064
    /// Scaling and rotating coefficient register
    pub sr_scal_rotate: SrScalRotate,
    /// 0x0068
    /// SR memory power done register
    pub sr_mem_pd: SrMemPd,
    /// 0x006c
    /// Register clock enable register
    pub reg_conf: RegConf,
    /// 0x0070
    /// BLEND CLUT write counter register
    pub clut_cnt: ClutCnt,
    /// 0x0074
    /// Blending engine status register
    pub blend_st: BlendSt,
    /// 0x0078
    /// Scaling and rotating coefficient error register
    pub sr_param_err_st: SrParamErrSt,
    /// 0x007c
    /// SR FSM register
    pub sr_status: SrStatus,
    /// 0x0080
    /// Reserved.
    pub eco_low: EcoLow,
    /// 0x0084
    /// Reserved.
    pub eco_high: EcoHigh,
    /// 0x0088
    /// Reserved.
    pub eco_cell_ctrl: EcoCellCtrl,
    /// 0x008c
    /// PPA SRAM Control Register
    pub sram_ctrl: SramCtrl,
    _reserved3: [u8; 112],
    /// 0x0100
    /// PPA Version register
    pub date: Date,
}
