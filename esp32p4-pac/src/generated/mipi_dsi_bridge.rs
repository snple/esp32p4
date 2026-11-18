//! MIPI_DSI_BRIDGE
//! MIPI Camera Interface Bridge

#![allow(dead_code)]

pub const BASE: u32 = 0x500a0800;

/// CLK_EN Register
///
/// dsi bridge clk control register
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **CLK_EN** `[0]` (RW)
///
///   this bit configures force_on of dsi_bridge register clock gate
#[repr(transparent)]
pub struct ClkEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClkEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkEn {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// this bit configures force_on of dsi_bridge register clock gate
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_EN_OFFSET: u32 = 0;
    pub const CLK_EN_MASK: u32 = 0x00000001;
    pub const CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[0]` (RW)
    ///
    /// this bit configures force_on of dsi_bridge register clock gate
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

/// CLK_EN value type for batch read/modify/write operations
///
/// Use `clken.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clken.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkEnVal {
    bits: u32,
}

impl ClkEnVal {
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
    /// this bit configures force_on of dsi_bridge register clock gate
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & ClkEn::CLK_EN_MASK) != 0
    }

    /// Set **CLK_EN** `[0:0]`
    ///
    /// this bit configures force_on of dsi_bridge register clock gate
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CLK_EN_MASK
            } else {
                self.bits & !ClkEn::CLK_EN_MASK
            },
        }
    }
}

impl Default for ClkEnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ClkEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkEnVal {
        ClkEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkEnVal) -> ClkEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EN Register
///
/// dsi bridge en register
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **DSI_EN** `[0]` (RW)
///
///   this bit configures module enable of dsi_bridge. 0: disable, 1: enable
#[repr(transparent)]
pub struct En {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for En {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl En {
    // ======== Field Constants ========

    /// **DSI_EN**
    ///
    /// this bit configures module enable of dsi_bridge. 0: disable, 1: enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DSI_EN_OFFSET: u32 = 0;
    pub const DSI_EN_MASK: u32 = 0x00000001;
    pub const DSI_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DSI_EN** `[0]` (RW)
    ///
    /// this bit configures module enable of dsi_bridge. 0: disable, 1: enable
    #[inline(always)]
    pub fn dsi_en(&self) -> bool {
        (self.value.read_bits() & Self::DSI_EN_MASK) != 0
    }

    /// Set DSI_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dsi_en(&self) {
        self.value.modify_bits(|r| r | Self::DSI_EN_MASK);
    }

    /// Clear DSI_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dsi_en(&self) {
        self.value.modify_bits(|r| r & !Self::DSI_EN_MASK);
    }
}

/// EN value type for batch read/modify/write operations
///
/// Use `en.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `en.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EnVal {
    bits: u32,
}

impl EnVal {
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

    /// Read **DSI_EN** `[0:0]`
    ///
    /// this bit configures module enable of dsi_bridge. 0: disable, 1: enable
    #[inline(always)]
    pub const fn dsi_en(&self) -> bool {
        (self.bits & En::DSI_EN_MASK) != 0
    }

    /// Set **DSI_EN** `[0:0]`
    ///
    /// this bit configures module enable of dsi_bridge. 0: disable, 1: enable
    #[inline(always)]
    pub const fn with_dsi_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | En::DSI_EN_MASK
            } else {
                self.bits & !En::DSI_EN_MASK
            },
        }
    }
}

impl Default for EnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl En {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EnVal {
        EnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EnVal) -> EnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA_REQ_CFG Register
///
/// dsi bridge dma burst len register
///
/// Address offset: 0x0008
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **DMA_BURST_LEN** `[11:0]` (RW)
///
///   this field configures the num of 64-bit in one dma burst transfer, valid only when dsi_bridge as flow controller
#[repr(transparent)]
pub struct DmaReqCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmaReqCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmaReqCfg {
    // ======== Field Constants ========

    /// **DMA_BURST_LEN**
    ///
    /// this field configures the num of 64-bit in one dma burst transfer, valid only when dsi_bridge as flow controller
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const DMA_BURST_LEN_OFFSET: u32 = 0;
    pub const DMA_BURST_LEN_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **DMA_BURST_LEN** `[11:0]` (RW)
    ///
    /// this field configures the num of 64-bit in one dma burst transfer, valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub fn dma_burst_len(&self) -> u32 {
        (self.value.read_bits() & Self::DMA_BURST_LEN_MASK) >> Self::DMA_BURST_LEN_OFFSET
    }

    /// Write **DMA_BURST_LEN** `[11:0]` (RW)
    ///
    /// this field configures the num of 64-bit in one dma burst transfer, valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub fn write_dma_burst_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DMA_BURST_LEN_MASK)
                | ((value << Self::DMA_BURST_LEN_OFFSET) & Self::DMA_BURST_LEN_MASK)
        });
    }
}

/// DMA_REQ_CFG value type for batch read/modify/write operations
///
/// Use `dmareqcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmareqcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaReqCfgVal {
    bits: u32,
}

impl DmaReqCfgVal {
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

    /// Read **DMA_BURST_LEN** `[11:0]`
    ///
    /// this field configures the num of 64-bit in one dma burst transfer, valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub const fn dma_burst_len(&self) -> u32 {
        (self.bits & DmaReqCfg::DMA_BURST_LEN_MASK) >> DmaReqCfg::DMA_BURST_LEN_OFFSET
    }

    /// Set **DMA_BURST_LEN** `[11:0]`
    ///
    /// this field configures the num of 64-bit in one dma burst transfer, valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub const fn with_dma_burst_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DmaReqCfg::DMA_BURST_LEN_MASK)
                | ((value << DmaReqCfg::DMA_BURST_LEN_OFFSET) & DmaReqCfg::DMA_BURST_LEN_MASK),
        }
    }
}

impl Default for DmaReqCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DmaReqCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaReqCfgVal {
        DmaReqCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaReqCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaReqCfgVal) -> DmaReqCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RAW_NUM_CFG Register
///
/// dsi bridge raw number control register
///
/// Address offset: 0x000c
/// Reset value: 0x00038400
///
/// ## Fields
///
/// - **RAW_NUM_TOTAL_SET** `[31]` (WO)
///
///   this bit configures enable of reload reg_raw_num_total to internal cnt. 0: disable, 1: enable. valid only when dsi_bridge as flow controller
///
/// - **UNALIGN_64BIT_EN** `[22]` (RW)
///
///   this field configures whether the total pix bits is a multiple of 64bits. 0: align to 64-bit, 1: unalign to 64-bit
///
/// - **RAW_NUM_TOTAL** `[21:0]` (RW)
///
///   this field configures number of total pix bits/64
#[repr(transparent)]
pub struct RawNumCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RawNumCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RawNumCfg {
    // ======== Field Constants ========

    /// **RAW_NUM_TOTAL_SET**
    ///
    /// this bit configures enable of reload reg_raw_num_total to internal cnt. 0: disable, 1: enable. valid only when dsi_bridge as flow controller
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const RAW_NUM_TOTAL_SET_OFFSET: u32 = 31;
    pub const RAW_NUM_TOTAL_SET_MASK: u32 = 0x80000000;
    pub const RAW_NUM_TOTAL_SET: u32 = 1 << 31;

    /// **UNALIGN_64BIT_EN**
    ///
    /// this field configures whether the total pix bits is a multiple of 64bits. 0: align to 64-bit, 1: unalign to 64-bit
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const UNALIGN_64BIT_EN_OFFSET: u32 = 22;
    pub const UNALIGN_64BIT_EN_MASK: u32 = 0x00400000;
    pub const UNALIGN_64BIT_EN: u32 = 1 << 22;

    /// **RAW_NUM_TOTAL**
    ///
    /// this field configures number of total pix bits/64
    ///
    /// Bits `[21:0]`, mask `0x003fffff`
    pub const RAW_NUM_TOTAL_OFFSET: u32 = 0;
    pub const RAW_NUM_TOTAL_MASK: u32 = 0x003fffff;

    // ======== Field Accessors ========

    /// Trigger **RAW_NUM_TOTAL_SET** (WO)
    ///
    /// this bit configures enable of reload reg_raw_num_total to internal cnt. 0: disable, 1: enable. valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub fn trigger_raw_num_total_set(&self) {
        self.value.modify_bits(|r| r | Self::RAW_NUM_TOTAL_SET_MASK);
    }

    /// **UNALIGN_64BIT_EN** `[22]` (RW)
    ///
    /// this field configures whether the total pix bits is a multiple of 64bits. 0: align to 64-bit, 1: unalign to 64-bit
    #[inline(always)]
    pub fn unalign_64bit_en(&self) -> bool {
        (self.value.read_bits() & Self::UNALIGN_64BIT_EN_MASK) != 0
    }

    /// Set UNALIGN_64BIT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_unalign_64bit_en(&self) {
        self.value.modify_bits(|r| r | Self::UNALIGN_64BIT_EN_MASK);
    }

    /// Clear UNALIGN_64BIT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_unalign_64bit_en(&self) {
        self.value.modify_bits(|r| r & !Self::UNALIGN_64BIT_EN_MASK);
    }

    /// **RAW_NUM_TOTAL** `[21:0]` (RW)
    ///
    /// this field configures number of total pix bits/64
    #[inline(always)]
    pub fn raw_num_total(&self) -> u32 {
        (self.value.read_bits() & Self::RAW_NUM_TOTAL_MASK) >> Self::RAW_NUM_TOTAL_OFFSET
    }

    /// Write **RAW_NUM_TOTAL** `[21:0]` (RW)
    ///
    /// this field configures number of total pix bits/64
    #[inline(always)]
    pub fn write_raw_num_total(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RAW_NUM_TOTAL_MASK)
                | ((value << Self::RAW_NUM_TOTAL_OFFSET) & Self::RAW_NUM_TOTAL_MASK)
        });
    }
}

/// RAW_NUM_CFG value type for batch read/modify/write operations
///
/// Use `rawnumcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rawnumcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RawNumCfgVal {
    bits: u32,
}

impl RawNumCfgVal {
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
        Self { bits: 0x00038400 }
    }

    /// Read **UNALIGN_64BIT_EN** `[22:22]`
    ///
    /// this field configures whether the total pix bits is a multiple of 64bits. 0: align to 64-bit, 1: unalign to 64-bit
    #[inline(always)]
    pub const fn unalign_64bit_en(&self) -> bool {
        (self.bits & RawNumCfg::UNALIGN_64BIT_EN_MASK) != 0
    }

    /// Read **RAW_NUM_TOTAL** `[21:0]`
    ///
    /// this field configures number of total pix bits/64
    #[inline(always)]
    pub const fn raw_num_total(&self) -> u32 {
        (self.bits & RawNumCfg::RAW_NUM_TOTAL_MASK) >> RawNumCfg::RAW_NUM_TOTAL_OFFSET
    }

    /// Set **RAW_NUM_TOTAL_SET** `[31:31]`
    ///
    /// this bit configures enable of reload reg_raw_num_total to internal cnt. 0: disable, 1: enable. valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub const fn with_raw_num_total_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RawNumCfg::RAW_NUM_TOTAL_SET_MASK
            } else {
                self.bits & !RawNumCfg::RAW_NUM_TOTAL_SET_MASK
            },
        }
    }

    /// Set **UNALIGN_64BIT_EN** `[22:22]`
    ///
    /// this field configures whether the total pix bits is a multiple of 64bits. 0: align to 64-bit, 1: unalign to 64-bit
    #[inline(always)]
    pub const fn with_unalign_64bit_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RawNumCfg::UNALIGN_64BIT_EN_MASK
            } else {
                self.bits & !RawNumCfg::UNALIGN_64BIT_EN_MASK
            },
        }
    }

    /// Set **RAW_NUM_TOTAL** `[21:0]`
    ///
    /// this field configures number of total pix bits/64
    #[inline(always)]
    pub const fn with_raw_num_total(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RawNumCfg::RAW_NUM_TOTAL_MASK)
                | ((value << RawNumCfg::RAW_NUM_TOTAL_OFFSET) & RawNumCfg::RAW_NUM_TOTAL_MASK),
        }
    }
}

impl Default for RawNumCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RawNumCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RawNumCfgVal {
        RawNumCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RawNumCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RawNumCfgVal) -> RawNumCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RAW_BUF_CREDIT_CTL Register
///
/// dsi bridge credit register
///
/// Address offset: 0x0010
/// Reset value: 0x03200400
///
/// ## Fields
///
/// - **CREDIT_RESET** `[31]` (RW)
///
///   this bit configures internal credit cnt clear, 0: non, 1: reset. valid only when dsi_bridge as flow controller
///
/// - **CREDIT_BURST_THRD** `[30:16]` (RW)
///
///   this field configures the threshold whether dsi_bridge fifo can receive one more dma burst, valid only when dsi_bridge as flow controller
///
/// - **CREDIT_THRD** `[14:0]` (RW)
///
///   this field configures the threshold whether dsi_bridge fifo can receive one more 64-bit, valid only when dsi_bridge as flow controller
#[repr(transparent)]
pub struct RawBufCreditCtl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RawBufCreditCtl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RawBufCreditCtl {
    // ======== Field Constants ========

    /// **CREDIT_RESET**
    ///
    /// this bit configures internal credit cnt clear, 0: non, 1: reset. valid only when dsi_bridge as flow controller
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CREDIT_RESET_OFFSET: u32 = 31;
    pub const CREDIT_RESET_MASK: u32 = 0x80000000;
    pub const CREDIT_RESET: u32 = 1 << 31;

    /// **CREDIT_BURST_THRD**
    ///
    /// this field configures the threshold whether dsi_bridge fifo can receive one more dma burst, valid only when dsi_bridge as flow controller
    ///
    /// Bits `[30:16]`, mask `0x7fff0000`
    pub const CREDIT_BURST_THRD_OFFSET: u32 = 16;
    pub const CREDIT_BURST_THRD_MASK: u32 = 0x7fff0000;

    /// **CREDIT_THRD**
    ///
    /// this field configures the threshold whether dsi_bridge fifo can receive one more 64-bit, valid only when dsi_bridge as flow controller
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const CREDIT_THRD_OFFSET: u32 = 0;
    pub const CREDIT_THRD_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **CREDIT_RESET** `[31]` (RW)
    ///
    /// this bit configures internal credit cnt clear, 0: non, 1: reset. valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub fn credit_reset(&self) -> bool {
        (self.value.read_bits() & Self::CREDIT_RESET_MASK) != 0
    }

    /// Set CREDIT_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_credit_reset(&self) {
        self.value.modify_bits(|r| r | Self::CREDIT_RESET_MASK);
    }

    /// Clear CREDIT_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_credit_reset(&self) {
        self.value.modify_bits(|r| r & !Self::CREDIT_RESET_MASK);
    }

    /// **CREDIT_BURST_THRD** `[30:16]` (RW)
    ///
    /// this field configures the threshold whether dsi_bridge fifo can receive one more dma burst, valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub fn credit_burst_thrd(&self) -> u32 {
        (self.value.read_bits() & Self::CREDIT_BURST_THRD_MASK) >> Self::CREDIT_BURST_THRD_OFFSET
    }

    /// Write **CREDIT_BURST_THRD** `[30:16]` (RW)
    ///
    /// this field configures the threshold whether dsi_bridge fifo can receive one more dma burst, valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub fn write_credit_burst_thrd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CREDIT_BURST_THRD_MASK)
                | ((value << Self::CREDIT_BURST_THRD_OFFSET) & Self::CREDIT_BURST_THRD_MASK)
        });
    }

    /// **CREDIT_THRD** `[14:0]` (RW)
    ///
    /// this field configures the threshold whether dsi_bridge fifo can receive one more 64-bit, valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub fn credit_thrd(&self) -> u32 {
        (self.value.read_bits() & Self::CREDIT_THRD_MASK) >> Self::CREDIT_THRD_OFFSET
    }

    /// Write **CREDIT_THRD** `[14:0]` (RW)
    ///
    /// this field configures the threshold whether dsi_bridge fifo can receive one more 64-bit, valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub fn write_credit_thrd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CREDIT_THRD_MASK)
                | ((value << Self::CREDIT_THRD_OFFSET) & Self::CREDIT_THRD_MASK)
        });
    }
}

/// RAW_BUF_CREDIT_CTL value type for batch read/modify/write operations
///
/// Use `rawbufcreditctl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rawbufcreditctl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RawBufCreditCtlVal {
    bits: u32,
}

impl RawBufCreditCtlVal {
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
        Self { bits: 0x03200400 }
    }

    /// Read **CREDIT_RESET** `[31:31]`
    ///
    /// this bit configures internal credit cnt clear, 0: non, 1: reset. valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub const fn credit_reset(&self) -> bool {
        (self.bits & RawBufCreditCtl::CREDIT_RESET_MASK) != 0
    }

    /// Read **CREDIT_BURST_THRD** `[30:16]`
    ///
    /// this field configures the threshold whether dsi_bridge fifo can receive one more dma burst, valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub const fn credit_burst_thrd(&self) -> u32 {
        (self.bits & RawBufCreditCtl::CREDIT_BURST_THRD_MASK)
            >> RawBufCreditCtl::CREDIT_BURST_THRD_OFFSET
    }

    /// Read **CREDIT_THRD** `[14:0]`
    ///
    /// this field configures the threshold whether dsi_bridge fifo can receive one more 64-bit, valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub const fn credit_thrd(&self) -> u32 {
        (self.bits & RawBufCreditCtl::CREDIT_THRD_MASK) >> RawBufCreditCtl::CREDIT_THRD_OFFSET
    }

    /// Set **CREDIT_RESET** `[31:31]`
    ///
    /// this bit configures internal credit cnt clear, 0: non, 1: reset. valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub const fn with_credit_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RawBufCreditCtl::CREDIT_RESET_MASK
            } else {
                self.bits & !RawBufCreditCtl::CREDIT_RESET_MASK
            },
        }
    }

    /// Set **CREDIT_BURST_THRD** `[30:16]`
    ///
    /// this field configures the threshold whether dsi_bridge fifo can receive one more dma burst, valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub const fn with_credit_burst_thrd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RawBufCreditCtl::CREDIT_BURST_THRD_MASK)
                | ((value << RawBufCreditCtl::CREDIT_BURST_THRD_OFFSET)
                    & RawBufCreditCtl::CREDIT_BURST_THRD_MASK),
        }
    }

    /// Set **CREDIT_THRD** `[14:0]`
    ///
    /// this field configures the threshold whether dsi_bridge fifo can receive one more 64-bit, valid only when dsi_bridge as flow controller
    #[inline(always)]
    pub const fn with_credit_thrd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RawBufCreditCtl::CREDIT_THRD_MASK)
                | ((value << RawBufCreditCtl::CREDIT_THRD_OFFSET)
                    & RawBufCreditCtl::CREDIT_THRD_MASK),
        }
    }
}

impl Default for RawBufCreditCtlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RawBufCreditCtl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RawBufCreditCtlVal {
        RawBufCreditCtlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RawBufCreditCtlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RawBufCreditCtlVal) -> RawBufCreditCtlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FIFO_FLOW_STATUS Register
///
/// dsi bridge raw buffer depth register
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **RAW_BUF_DEPTH** `[13:0]` (RO)
///
///   this field configures the depth of dsi_bridge fifo depth
#[repr(transparent)]
pub struct FifoFlowStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FifoFlowStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FifoFlowStatus {
    // ======== Field Constants ========

    /// **RAW_BUF_DEPTH**
    ///
    /// this field configures the depth of dsi_bridge fifo depth
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const RAW_BUF_DEPTH_OFFSET: u32 = 0;
    pub const RAW_BUF_DEPTH_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **RAW_BUF_DEPTH** `[13:0]` (RO)
    ///
    /// this field configures the depth of dsi_bridge fifo depth
    #[inline(always)]
    pub fn raw_buf_depth(&self) -> u32 {
        (self.value.read_bits() & Self::RAW_BUF_DEPTH_MASK) >> Self::RAW_BUF_DEPTH_OFFSET
    }
}

/// FIFO_FLOW_STATUS value type for batch read/modify/write operations
///
/// Use `fifoflowstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fifoflowstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FifoFlowStatusVal {
    bits: u32,
}

impl FifoFlowStatusVal {
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

    /// Read **RAW_BUF_DEPTH** `[13:0]`
    ///
    /// this field configures the depth of dsi_bridge fifo depth
    #[inline(always)]
    pub const fn raw_buf_depth(&self) -> u32 {
        (self.bits & FifoFlowStatus::RAW_BUF_DEPTH_MASK) >> FifoFlowStatus::RAW_BUF_DEPTH_OFFSET
    }
}

impl Default for FifoFlowStatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FifoFlowStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FifoFlowStatusVal {
        FifoFlowStatusVal::from_bits(self.value.read_bits())
    }
}

/// PIXEL_TYPE Register
///
/// dsi bridge dpi type control register
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **DATA_IN_TYPE** `[6]` (RW)
///
///   input data type, 0: rgb, 1: yuv
///
/// - **DPI_CONFIG** `[5:4]` (RW)
///
///   this field configures the pixel arrange type of dpi interface
///
/// - **RAW_TYPE** `[3:0]` (RW)
///
///   this field configures the pixel type. 0: rgb888, 1:rgb666, 2:rgb565
#[repr(transparent)]
pub struct PixelType {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PixelType {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PixelType {
    // ======== Field Constants ========

    /// **DATA_IN_TYPE**
    ///
    /// input data type, 0: rgb, 1: yuv
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DATA_IN_TYPE_OFFSET: u32 = 6;
    pub const DATA_IN_TYPE_MASK: u32 = 0x00000040;
    pub const DATA_IN_TYPE: u32 = 1 << 6;

    /// **DPI_CONFIG**
    ///
    /// this field configures the pixel arrange type of dpi interface
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const DPI_CONFIG_OFFSET: u32 = 4;
    pub const DPI_CONFIG_MASK: u32 = 0x00000030;

    /// **RAW_TYPE**
    ///
    /// this field configures the pixel type. 0: rgb888, 1:rgb666, 2:rgb565
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const RAW_TYPE_OFFSET: u32 = 0;
    pub const RAW_TYPE_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **DATA_IN_TYPE** `[6]` (RW)
    ///
    /// input data type, 0: rgb, 1: yuv
    #[inline(always)]
    pub fn data_in_type(&self) -> bool {
        (self.value.read_bits() & Self::DATA_IN_TYPE_MASK) != 0
    }

    /// Set DATA_IN_TYPE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_data_in_type(&self) {
        self.value.modify_bits(|r| r | Self::DATA_IN_TYPE_MASK);
    }

    /// Clear DATA_IN_TYPE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_data_in_type(&self) {
        self.value.modify_bits(|r| r & !Self::DATA_IN_TYPE_MASK);
    }

    /// **DPI_CONFIG** `[5:4]` (RW)
    ///
    /// this field configures the pixel arrange type of dpi interface
    #[inline(always)]
    pub fn dpi_config(&self) -> u32 {
        (self.value.read_bits() & Self::DPI_CONFIG_MASK) >> Self::DPI_CONFIG_OFFSET
    }

    /// Write **DPI_CONFIG** `[5:4]` (RW)
    ///
    /// this field configures the pixel arrange type of dpi interface
    #[inline(always)]
    pub fn write_dpi_config(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DPI_CONFIG_MASK)
                | ((value << Self::DPI_CONFIG_OFFSET) & Self::DPI_CONFIG_MASK)
        });
    }

    /// **RAW_TYPE** `[3:0]` (RW)
    ///
    /// this field configures the pixel type. 0: rgb888, 1:rgb666, 2:rgb565
    #[inline(always)]
    pub fn raw_type(&self) -> u32 {
        (self.value.read_bits() & Self::RAW_TYPE_MASK) >> Self::RAW_TYPE_OFFSET
    }

    /// Write **RAW_TYPE** `[3:0]` (RW)
    ///
    /// this field configures the pixel type. 0: rgb888, 1:rgb666, 2:rgb565
    #[inline(always)]
    pub fn write_raw_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RAW_TYPE_MASK) | ((value << Self::RAW_TYPE_OFFSET) & Self::RAW_TYPE_MASK)
        });
    }
}

/// PIXEL_TYPE value type for batch read/modify/write operations
///
/// Use `pixeltype.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pixeltype.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PixelTypeVal {
    bits: u32,
}

impl PixelTypeVal {
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

    /// Read **DATA_IN_TYPE** `[6:6]`
    ///
    /// input data type, 0: rgb, 1: yuv
    #[inline(always)]
    pub const fn data_in_type(&self) -> bool {
        (self.bits & PixelType::DATA_IN_TYPE_MASK) != 0
    }

    /// Read **DPI_CONFIG** `[5:4]`
    ///
    /// this field configures the pixel arrange type of dpi interface
    #[inline(always)]
    pub const fn dpi_config(&self) -> u32 {
        (self.bits & PixelType::DPI_CONFIG_MASK) >> PixelType::DPI_CONFIG_OFFSET
    }

    /// Read **RAW_TYPE** `[3:0]`
    ///
    /// this field configures the pixel type. 0: rgb888, 1:rgb666, 2:rgb565
    #[inline(always)]
    pub const fn raw_type(&self) -> u32 {
        (self.bits & PixelType::RAW_TYPE_MASK) >> PixelType::RAW_TYPE_OFFSET
    }

    /// Set **DATA_IN_TYPE** `[6:6]`
    ///
    /// input data type, 0: rgb, 1: yuv
    #[inline(always)]
    pub const fn with_data_in_type(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PixelType::DATA_IN_TYPE_MASK
            } else {
                self.bits & !PixelType::DATA_IN_TYPE_MASK
            },
        }
    }

    /// Set **DPI_CONFIG** `[5:4]`
    ///
    /// this field configures the pixel arrange type of dpi interface
    #[inline(always)]
    pub const fn with_dpi_config(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PixelType::DPI_CONFIG_MASK)
                | ((value << PixelType::DPI_CONFIG_OFFSET) & PixelType::DPI_CONFIG_MASK),
        }
    }

    /// Set **RAW_TYPE** `[3:0]`
    ///
    /// this field configures the pixel type. 0: rgb888, 1:rgb666, 2:rgb565
    #[inline(always)]
    pub const fn with_raw_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PixelType::RAW_TYPE_MASK)
                | ((value << PixelType::RAW_TYPE_OFFSET) & PixelType::RAW_TYPE_MASK),
        }
    }
}

impl Default for PixelTypeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PixelType {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PixelTypeVal {
        PixelTypeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PixelTypeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PixelTypeVal) -> PixelTypeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA_BLOCK_INTERVAL Register
///
/// dsi bridge dma block interval control register
///
/// Address offset: 0x001c
/// Reset value: 0x30002409
///
/// ## Fields
///
/// - **EN** `[29]` (RW)
///
///   this bit configures enable of interval between dma block transfer, 0: disable, 1: enable
///
/// - **RAW_NUM_TOTAL_AUTO_RELOAD** `[28]` (RW)
///
///   this bit configures enable of auto reload reg_raw_num_total, 0: disable, 1: enable
///
/// - **DMA_BLOCK_INTERVAL** `[27:10]` (RW)
///
///   this field configures the max block_interval_cnt, block_interval_cnt increased by 1 when block_slot_cnt if full
///
/// - **DMA_BLOCK_SLOT** `[9:0]` (RW)
///
///   this field configures the max block_slot_cnt
#[repr(transparent)]
pub struct DmaBlockInterval {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmaBlockInterval {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmaBlockInterval {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// this bit configures enable of interval between dma block transfer, 0: disable, 1: enable
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const EN_OFFSET: u32 = 29;
    pub const EN_MASK: u32 = 0x20000000;
    pub const EN: u32 = 1 << 29;

    /// **RAW_NUM_TOTAL_AUTO_RELOAD**
    ///
    /// this bit configures enable of auto reload reg_raw_num_total, 0: disable, 1: enable
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const RAW_NUM_TOTAL_AUTO_RELOAD_OFFSET: u32 = 28;
    pub const RAW_NUM_TOTAL_AUTO_RELOAD_MASK: u32 = 0x10000000;
    pub const RAW_NUM_TOTAL_AUTO_RELOAD: u32 = 1 << 28;

    /// **DMA_BLOCK_INTERVAL**
    ///
    /// this field configures the max block_interval_cnt, block_interval_cnt increased by 1 when block_slot_cnt if full
    ///
    /// Bits `[27:10]`, mask `0x0ffffc00`
    pub const DMA_BLOCK_INTERVAL_OFFSET: u32 = 10;
    pub const DMA_BLOCK_INTERVAL_MASK: u32 = 0x0ffffc00;

    /// **DMA_BLOCK_SLOT**
    ///
    /// this field configures the max block_slot_cnt
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const DMA_BLOCK_SLOT_OFFSET: u32 = 0;
    pub const DMA_BLOCK_SLOT_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **EN** `[29]` (RW)
    ///
    /// this bit configures enable of interval between dma block transfer, 0: disable, 1: enable
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.modify_bits(|r| r | Self::EN_MASK);
    }

    /// Clear EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.modify_bits(|r| r & !Self::EN_MASK);
    }

    /// **RAW_NUM_TOTAL_AUTO_RELOAD** `[28]` (RW)
    ///
    /// this bit configures enable of auto reload reg_raw_num_total, 0: disable, 1: enable
    #[inline(always)]
    pub fn raw_num_total_auto_reload(&self) -> bool {
        (self.value.read_bits() & Self::RAW_NUM_TOTAL_AUTO_RELOAD_MASK) != 0
    }

    /// Set RAW_NUM_TOTAL_AUTO_RELOAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_raw_num_total_auto_reload(&self) {
        self.value
            .modify_bits(|r| r | Self::RAW_NUM_TOTAL_AUTO_RELOAD_MASK);
    }

    /// Clear RAW_NUM_TOTAL_AUTO_RELOAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_raw_num_total_auto_reload(&self) {
        self.value
            .modify_bits(|r| r & !Self::RAW_NUM_TOTAL_AUTO_RELOAD_MASK);
    }

    /// **DMA_BLOCK_INTERVAL** `[27:10]` (RW)
    ///
    /// this field configures the max block_interval_cnt, block_interval_cnt increased by 1 when block_slot_cnt if full
    #[inline(always)]
    pub fn dma_block_interval(&self) -> u32 {
        (self.value.read_bits() & Self::DMA_BLOCK_INTERVAL_MASK) >> Self::DMA_BLOCK_INTERVAL_OFFSET
    }

    /// Write **DMA_BLOCK_INTERVAL** `[27:10]` (RW)
    ///
    /// this field configures the max block_interval_cnt, block_interval_cnt increased by 1 when block_slot_cnt if full
    #[inline(always)]
    pub fn write_dma_block_interval(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DMA_BLOCK_INTERVAL_MASK)
                | ((value << Self::DMA_BLOCK_INTERVAL_OFFSET) & Self::DMA_BLOCK_INTERVAL_MASK)
        });
    }

    /// **DMA_BLOCK_SLOT** `[9:0]` (RW)
    ///
    /// this field configures the max block_slot_cnt
    #[inline(always)]
    pub fn dma_block_slot(&self) -> u32 {
        (self.value.read_bits() & Self::DMA_BLOCK_SLOT_MASK) >> Self::DMA_BLOCK_SLOT_OFFSET
    }

    /// Write **DMA_BLOCK_SLOT** `[9:0]` (RW)
    ///
    /// this field configures the max block_slot_cnt
    #[inline(always)]
    pub fn write_dma_block_slot(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DMA_BLOCK_SLOT_MASK)
                | ((value << Self::DMA_BLOCK_SLOT_OFFSET) & Self::DMA_BLOCK_SLOT_MASK)
        });
    }
}

/// DMA_BLOCK_INTERVAL value type for batch read/modify/write operations
///
/// Use `dmablockinterval.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmablockinterval.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaBlockIntervalVal {
    bits: u32,
}

impl DmaBlockIntervalVal {
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
        Self { bits: 0x30002409 }
    }

    /// Read **EN** `[29:29]`
    ///
    /// this bit configures enable of interval between dma block transfer, 0: disable, 1: enable
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & DmaBlockInterval::EN_MASK) != 0
    }

    /// Read **RAW_NUM_TOTAL_AUTO_RELOAD** `[28:28]`
    ///
    /// this bit configures enable of auto reload reg_raw_num_total, 0: disable, 1: enable
    #[inline(always)]
    pub const fn raw_num_total_auto_reload(&self) -> bool {
        (self.bits & DmaBlockInterval::RAW_NUM_TOTAL_AUTO_RELOAD_MASK) != 0
    }

    /// Read **DMA_BLOCK_INTERVAL** `[27:10]`
    ///
    /// this field configures the max block_interval_cnt, block_interval_cnt increased by 1 when block_slot_cnt if full
    #[inline(always)]
    pub const fn dma_block_interval(&self) -> u32 {
        (self.bits & DmaBlockInterval::DMA_BLOCK_INTERVAL_MASK)
            >> DmaBlockInterval::DMA_BLOCK_INTERVAL_OFFSET
    }

    /// Read **DMA_BLOCK_SLOT** `[9:0]`
    ///
    /// this field configures the max block_slot_cnt
    #[inline(always)]
    pub const fn dma_block_slot(&self) -> u32 {
        (self.bits & DmaBlockInterval::DMA_BLOCK_SLOT_MASK)
            >> DmaBlockInterval::DMA_BLOCK_SLOT_OFFSET
    }

    /// Set **EN** `[29:29]`
    ///
    /// this bit configures enable of interval between dma block transfer, 0: disable, 1: enable
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaBlockInterval::EN_MASK
            } else {
                self.bits & !DmaBlockInterval::EN_MASK
            },
        }
    }

    /// Set **RAW_NUM_TOTAL_AUTO_RELOAD** `[28:28]`
    ///
    /// this bit configures enable of auto reload reg_raw_num_total, 0: disable, 1: enable
    #[inline(always)]
    pub const fn with_raw_num_total_auto_reload(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaBlockInterval::RAW_NUM_TOTAL_AUTO_RELOAD_MASK
            } else {
                self.bits & !DmaBlockInterval::RAW_NUM_TOTAL_AUTO_RELOAD_MASK
            },
        }
    }

    /// Set **DMA_BLOCK_INTERVAL** `[27:10]`
    ///
    /// this field configures the max block_interval_cnt, block_interval_cnt increased by 1 when block_slot_cnt if full
    #[inline(always)]
    pub const fn with_dma_block_interval(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DmaBlockInterval::DMA_BLOCK_INTERVAL_MASK)
                | ((value << DmaBlockInterval::DMA_BLOCK_INTERVAL_OFFSET)
                    & DmaBlockInterval::DMA_BLOCK_INTERVAL_MASK),
        }
    }

    /// Set **DMA_BLOCK_SLOT** `[9:0]`
    ///
    /// this field configures the max block_slot_cnt
    #[inline(always)]
    pub const fn with_dma_block_slot(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DmaBlockInterval::DMA_BLOCK_SLOT_MASK)
                | ((value << DmaBlockInterval::DMA_BLOCK_SLOT_OFFSET)
                    & DmaBlockInterval::DMA_BLOCK_SLOT_MASK),
        }
    }
}

impl Default for DmaBlockIntervalVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DmaBlockInterval {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaBlockIntervalVal {
        DmaBlockIntervalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaBlockIntervalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaBlockIntervalVal) -> DmaBlockIntervalVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA_REQ_INTERVAL Register
///
/// dsi bridge dma req interval control register
///
/// Address offset: 0x0020
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **DMA_REQ_INTERVAL** `[15:0]` (RW)
///
///   this field configures the interval between dma req events
#[repr(transparent)]
pub struct DmaReqInterval {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmaReqInterval {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmaReqInterval {
    // ======== Field Constants ========

    /// **DMA_REQ_INTERVAL**
    ///
    /// this field configures the interval between dma req events
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const DMA_REQ_INTERVAL_OFFSET: u32 = 0;
    pub const DMA_REQ_INTERVAL_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **DMA_REQ_INTERVAL** `[15:0]` (RW)
    ///
    /// this field configures the interval between dma req events
    #[inline(always)]
    pub fn dma_req_interval(&self) -> u32 {
        (self.value.read_bits() & Self::DMA_REQ_INTERVAL_MASK) >> Self::DMA_REQ_INTERVAL_OFFSET
    }

    /// Write **DMA_REQ_INTERVAL** `[15:0]` (RW)
    ///
    /// this field configures the interval between dma req events
    #[inline(always)]
    pub fn write_dma_req_interval(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DMA_REQ_INTERVAL_MASK)
                | ((value << Self::DMA_REQ_INTERVAL_OFFSET) & Self::DMA_REQ_INTERVAL_MASK)
        });
    }
}

/// DMA_REQ_INTERVAL value type for batch read/modify/write operations
///
/// Use `dmareqinterval.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmareqinterval.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaReqIntervalVal {
    bits: u32,
}

impl DmaReqIntervalVal {
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

    /// Read **DMA_REQ_INTERVAL** `[15:0]`
    ///
    /// this field configures the interval between dma req events
    #[inline(always)]
    pub const fn dma_req_interval(&self) -> u32 {
        (self.bits & DmaReqInterval::DMA_REQ_INTERVAL_MASK)
            >> DmaReqInterval::DMA_REQ_INTERVAL_OFFSET
    }

    /// Set **DMA_REQ_INTERVAL** `[15:0]`
    ///
    /// this field configures the interval between dma req events
    #[inline(always)]
    pub const fn with_dma_req_interval(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DmaReqInterval::DMA_REQ_INTERVAL_MASK)
                | ((value << DmaReqInterval::DMA_REQ_INTERVAL_OFFSET)
                    & DmaReqInterval::DMA_REQ_INTERVAL_MASK),
        }
    }
}

impl Default for DmaReqIntervalVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DmaReqInterval {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaReqIntervalVal {
        DmaReqIntervalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaReqIntervalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaReqIntervalVal) -> DmaReqIntervalVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DPI_LCD_CTL Register
///
/// dsi bridge dpi signal control register
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **DPIUPDATECFG** `[2]` (RW)
///
///   this bit configures dpiupdatecfg signal in dpi interface
///
/// - **DPICOLORM** `[1]` (RW)
///
///   this bit configures dpicolorm signal in dpi interface
///
/// - **DPISHUTDN** `[0]` (RW)
///
///   this bit configures dpishutdn signal in dpi interface
#[repr(transparent)]
pub struct DpiLcdCtl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiLcdCtl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiLcdCtl {
    // ======== Field Constants ========

    /// **DPIUPDATECFG**
    ///
    /// this bit configures dpiupdatecfg signal in dpi interface
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DPIUPDATECFG_OFFSET: u32 = 2;
    pub const DPIUPDATECFG_MASK: u32 = 0x00000004;
    pub const DPIUPDATECFG: u32 = 1 << 2;

    /// **DPICOLORM**
    ///
    /// this bit configures dpicolorm signal in dpi interface
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DPICOLORM_OFFSET: u32 = 1;
    pub const DPICOLORM_MASK: u32 = 0x00000002;
    pub const DPICOLORM: u32 = 1 << 1;

    /// **DPISHUTDN**
    ///
    /// this bit configures dpishutdn signal in dpi interface
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DPISHUTDN_OFFSET: u32 = 0;
    pub const DPISHUTDN_MASK: u32 = 0x00000001;
    pub const DPISHUTDN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DPIUPDATECFG** `[2]` (RW)
    ///
    /// this bit configures dpiupdatecfg signal in dpi interface
    #[inline(always)]
    pub fn dpiupdatecfg(&self) -> bool {
        (self.value.read_bits() & Self::DPIUPDATECFG_MASK) != 0
    }

    /// Set DPIUPDATECFG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dpiupdatecfg(&self) {
        self.value.modify_bits(|r| r | Self::DPIUPDATECFG_MASK);
    }

    /// Clear DPIUPDATECFG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dpiupdatecfg(&self) {
        self.value.modify_bits(|r| r & !Self::DPIUPDATECFG_MASK);
    }

    /// **DPICOLORM** `[1]` (RW)
    ///
    /// this bit configures dpicolorm signal in dpi interface
    #[inline(always)]
    pub fn dpicolorm(&self) -> bool {
        (self.value.read_bits() & Self::DPICOLORM_MASK) != 0
    }

    /// Set DPICOLORM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dpicolorm(&self) {
        self.value.modify_bits(|r| r | Self::DPICOLORM_MASK);
    }

    /// Clear DPICOLORM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dpicolorm(&self) {
        self.value.modify_bits(|r| r & !Self::DPICOLORM_MASK);
    }

    /// **DPISHUTDN** `[0]` (RW)
    ///
    /// this bit configures dpishutdn signal in dpi interface
    #[inline(always)]
    pub fn dpishutdn(&self) -> bool {
        (self.value.read_bits() & Self::DPISHUTDN_MASK) != 0
    }

    /// Set DPISHUTDN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dpishutdn(&self) {
        self.value.modify_bits(|r| r | Self::DPISHUTDN_MASK);
    }

    /// Clear DPISHUTDN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dpishutdn(&self) {
        self.value.modify_bits(|r| r & !Self::DPISHUTDN_MASK);
    }
}

/// DPI_LCD_CTL value type for batch read/modify/write operations
///
/// Use `dpilcdctl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpilcdctl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiLcdCtlVal {
    bits: u32,
}

impl DpiLcdCtlVal {
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

    /// Read **DPIUPDATECFG** `[2:2]`
    ///
    /// this bit configures dpiupdatecfg signal in dpi interface
    #[inline(always)]
    pub const fn dpiupdatecfg(&self) -> bool {
        (self.bits & DpiLcdCtl::DPIUPDATECFG_MASK) != 0
    }

    /// Read **DPICOLORM** `[1:1]`
    ///
    /// this bit configures dpicolorm signal in dpi interface
    #[inline(always)]
    pub const fn dpicolorm(&self) -> bool {
        (self.bits & DpiLcdCtl::DPICOLORM_MASK) != 0
    }

    /// Read **DPISHUTDN** `[0:0]`
    ///
    /// this bit configures dpishutdn signal in dpi interface
    #[inline(always)]
    pub const fn dpishutdn(&self) -> bool {
        (self.bits & DpiLcdCtl::DPISHUTDN_MASK) != 0
    }

    /// Set **DPIUPDATECFG** `[2:2]`
    ///
    /// this bit configures dpiupdatecfg signal in dpi interface
    #[inline(always)]
    pub const fn with_dpiupdatecfg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DpiLcdCtl::DPIUPDATECFG_MASK
            } else {
                self.bits & !DpiLcdCtl::DPIUPDATECFG_MASK
            },
        }
    }

    /// Set **DPICOLORM** `[1:1]`
    ///
    /// this bit configures dpicolorm signal in dpi interface
    #[inline(always)]
    pub const fn with_dpicolorm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DpiLcdCtl::DPICOLORM_MASK
            } else {
                self.bits & !DpiLcdCtl::DPICOLORM_MASK
            },
        }
    }

    /// Set **DPISHUTDN** `[0:0]`
    ///
    /// this bit configures dpishutdn signal in dpi interface
    #[inline(always)]
    pub const fn with_dpishutdn(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DpiLcdCtl::DPISHUTDN_MASK
            } else {
                self.bits & !DpiLcdCtl::DPISHUTDN_MASK
            },
        }
    }
}

impl Default for DpiLcdCtlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DpiLcdCtl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DpiLcdCtlVal {
        DpiLcdCtlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DpiLcdCtlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DpiLcdCtlVal) -> DpiLcdCtlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DPI_RSV_DPI_DATA Register
///
/// dsi bridge dpi reserved data register
///
/// Address offset: 0x0028
/// Reset value: 0x00003fff
///
/// ## Fields
///
/// - **DPI_RSV_DATA** `[29:0]` (RW)
///
///   this field controls the pixel data sent to dsi_host when dsi_bridge fifo underflow
#[repr(transparent)]
pub struct DpiRsvDpiData {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiRsvDpiData {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiRsvDpiData {
    // ======== Field Constants ========

    /// **DPI_RSV_DATA**
    ///
    /// this field controls the pixel data sent to dsi_host when dsi_bridge fifo underflow
    ///
    /// Bits `[29:0]`, mask `0x3fffffff`
    pub const DPI_RSV_DATA_OFFSET: u32 = 0;
    pub const DPI_RSV_DATA_MASK: u32 = 0x3fffffff;

    // ======== Field Accessors ========

    /// **DPI_RSV_DATA** `[29:0]` (RW)
    ///
    /// this field controls the pixel data sent to dsi_host when dsi_bridge fifo underflow
    #[inline(always)]
    pub fn dpi_rsv_data(&self) -> u32 {
        (self.value.read_bits() & Self::DPI_RSV_DATA_MASK) >> Self::DPI_RSV_DATA_OFFSET
    }

    /// Write **DPI_RSV_DATA** `[29:0]` (RW)
    ///
    /// this field controls the pixel data sent to dsi_host when dsi_bridge fifo underflow
    #[inline(always)]
    pub fn write_dpi_rsv_data(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DPI_RSV_DATA_MASK)
                | ((value << Self::DPI_RSV_DATA_OFFSET) & Self::DPI_RSV_DATA_MASK)
        });
    }
}

/// DPI_RSV_DPI_DATA value type for batch read/modify/write operations
///
/// Use `dpirsvdpidata.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpirsvdpidata.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiRsvDpiDataVal {
    bits: u32,
}

impl DpiRsvDpiDataVal {
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
        Self { bits: 0x00003fff }
    }

    /// Read **DPI_RSV_DATA** `[29:0]`
    ///
    /// this field controls the pixel data sent to dsi_host when dsi_bridge fifo underflow
    #[inline(always)]
    pub const fn dpi_rsv_data(&self) -> u32 {
        (self.bits & DpiRsvDpiData::DPI_RSV_DATA_MASK) >> DpiRsvDpiData::DPI_RSV_DATA_OFFSET
    }

    /// Set **DPI_RSV_DATA** `[29:0]`
    ///
    /// this field controls the pixel data sent to dsi_host when dsi_bridge fifo underflow
    #[inline(always)]
    pub const fn with_dpi_rsv_data(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DpiRsvDpiData::DPI_RSV_DATA_MASK)
                | ((value << DpiRsvDpiData::DPI_RSV_DATA_OFFSET)
                    & DpiRsvDpiData::DPI_RSV_DATA_MASK),
        }
    }
}

impl Default for DpiRsvDpiDataVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DpiRsvDpiData {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DpiRsvDpiDataVal {
        DpiRsvDpiDataVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DpiRsvDpiDataVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DpiRsvDpiDataVal) -> DpiRsvDpiDataVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DPI_V_CFG0 Register
///
/// dsi bridge dpi v config register 0
///
/// Address offset: 0x0030
/// Reset value: 0x01e0020d
///
/// ## Fields
///
/// - **VDISP** `[27:16]` (RW)
///
///   this field configures the length of valid line (by line) for dpi output
///
/// - **VTOTAL** `[11:0]` (RW)
///
///   this field configures the total length of one frame (by line) for dpi output, must meet: reg_vtotal > reg_vdisp+reg_vsync+reg_vbank
#[repr(transparent)]
pub struct DpiVCfg0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiVCfg0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiVCfg0 {
    // ======== Field Constants ========

    /// **VDISP**
    ///
    /// this field configures the length of valid line (by line) for dpi output
    ///
    /// Bits `[27:16]`, mask `0x0fff0000`
    pub const VDISP_OFFSET: u32 = 16;
    pub const VDISP_MASK: u32 = 0x0fff0000;

    /// **VTOTAL**
    ///
    /// this field configures the total length of one frame (by line) for dpi output, must meet: reg_vtotal > reg_vdisp+reg_vsync+reg_vbank
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const VTOTAL_OFFSET: u32 = 0;
    pub const VTOTAL_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **VDISP** `[27:16]` (RW)
    ///
    /// this field configures the length of valid line (by line) for dpi output
    #[inline(always)]
    pub fn vdisp(&self) -> u32 {
        (self.value.read_bits() & Self::VDISP_MASK) >> Self::VDISP_OFFSET
    }

    /// Write **VDISP** `[27:16]` (RW)
    ///
    /// this field configures the length of valid line (by line) for dpi output
    #[inline(always)]
    pub fn write_vdisp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VDISP_MASK) | ((value << Self::VDISP_OFFSET) & Self::VDISP_MASK)
        });
    }

    /// **VTOTAL** `[11:0]` (RW)
    ///
    /// this field configures the total length of one frame (by line) for dpi output, must meet: reg_vtotal > reg_vdisp+reg_vsync+reg_vbank
    #[inline(always)]
    pub fn vtotal(&self) -> u32 {
        (self.value.read_bits() & Self::VTOTAL_MASK) >> Self::VTOTAL_OFFSET
    }

    /// Write **VTOTAL** `[11:0]` (RW)
    ///
    /// this field configures the total length of one frame (by line) for dpi output, must meet: reg_vtotal > reg_vdisp+reg_vsync+reg_vbank
    #[inline(always)]
    pub fn write_vtotal(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VTOTAL_MASK) | ((value << Self::VTOTAL_OFFSET) & Self::VTOTAL_MASK)
        });
    }
}

/// DPI_V_CFG0 value type for batch read/modify/write operations
///
/// Use `dpivcfg0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpivcfg0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiVCfg0Val {
    bits: u32,
}

impl DpiVCfg0Val {
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
        Self { bits: 0x01e0020d }
    }

    /// Read **VDISP** `[27:16]`
    ///
    /// this field configures the length of valid line (by line) for dpi output
    #[inline(always)]
    pub const fn vdisp(&self) -> u32 {
        (self.bits & DpiVCfg0::VDISP_MASK) >> DpiVCfg0::VDISP_OFFSET
    }

    /// Read **VTOTAL** `[11:0]`
    ///
    /// this field configures the total length of one frame (by line) for dpi output, must meet: reg_vtotal > reg_vdisp+reg_vsync+reg_vbank
    #[inline(always)]
    pub const fn vtotal(&self) -> u32 {
        (self.bits & DpiVCfg0::VTOTAL_MASK) >> DpiVCfg0::VTOTAL_OFFSET
    }

    /// Set **VDISP** `[27:16]`
    ///
    /// this field configures the length of valid line (by line) for dpi output
    #[inline(always)]
    pub const fn with_vdisp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DpiVCfg0::VDISP_MASK)
                | ((value << DpiVCfg0::VDISP_OFFSET) & DpiVCfg0::VDISP_MASK),
        }
    }

    /// Set **VTOTAL** `[11:0]`
    ///
    /// this field configures the total length of one frame (by line) for dpi output, must meet: reg_vtotal > reg_vdisp+reg_vsync+reg_vbank
    #[inline(always)]
    pub const fn with_vtotal(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DpiVCfg0::VTOTAL_MASK)
                | ((value << DpiVCfg0::VTOTAL_OFFSET) & DpiVCfg0::VTOTAL_MASK),
        }
    }
}

impl Default for DpiVCfg0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DpiVCfg0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DpiVCfg0Val {
        DpiVCfg0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DpiVCfg0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DpiVCfg0Val) -> DpiVCfg0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DPI_V_CFG1 Register
///
/// dsi bridge dpi v config register 1
///
/// Address offset: 0x0034
/// Reset value: 0x00020021
///
/// ## Fields
///
/// - **VSYNC** `[27:16]` (RW)
///
///   this field configures the length of vsync (by line) for dpi output
///
/// - **VBANK** `[11:0]` (RW)
///
///   this field configures the length between vsync and valid line (by line) for dpi output
#[repr(transparent)]
pub struct DpiVCfg1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiVCfg1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiVCfg1 {
    // ======== Field Constants ========

    /// **VSYNC**
    ///
    /// this field configures the length of vsync (by line) for dpi output
    ///
    /// Bits `[27:16]`, mask `0x0fff0000`
    pub const VSYNC_OFFSET: u32 = 16;
    pub const VSYNC_MASK: u32 = 0x0fff0000;

    /// **VBANK**
    ///
    /// this field configures the length between vsync and valid line (by line) for dpi output
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const VBANK_OFFSET: u32 = 0;
    pub const VBANK_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **VSYNC** `[27:16]` (RW)
    ///
    /// this field configures the length of vsync (by line) for dpi output
    #[inline(always)]
    pub fn vsync(&self) -> u32 {
        (self.value.read_bits() & Self::VSYNC_MASK) >> Self::VSYNC_OFFSET
    }

    /// Write **VSYNC** `[27:16]` (RW)
    ///
    /// this field configures the length of vsync (by line) for dpi output
    #[inline(always)]
    pub fn write_vsync(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VSYNC_MASK) | ((value << Self::VSYNC_OFFSET) & Self::VSYNC_MASK)
        });
    }

    /// **VBANK** `[11:0]` (RW)
    ///
    /// this field configures the length between vsync and valid line (by line) for dpi output
    #[inline(always)]
    pub fn vbank(&self) -> u32 {
        (self.value.read_bits() & Self::VBANK_MASK) >> Self::VBANK_OFFSET
    }

    /// Write **VBANK** `[11:0]` (RW)
    ///
    /// this field configures the length between vsync and valid line (by line) for dpi output
    #[inline(always)]
    pub fn write_vbank(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VBANK_MASK) | ((value << Self::VBANK_OFFSET) & Self::VBANK_MASK)
        });
    }
}

/// DPI_V_CFG1 value type for batch read/modify/write operations
///
/// Use `dpivcfg1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpivcfg1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiVCfg1Val {
    bits: u32,
}

impl DpiVCfg1Val {
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
        Self { bits: 0x00020021 }
    }

    /// Read **VSYNC** `[27:16]`
    ///
    /// this field configures the length of vsync (by line) for dpi output
    #[inline(always)]
    pub const fn vsync(&self) -> u32 {
        (self.bits & DpiVCfg1::VSYNC_MASK) >> DpiVCfg1::VSYNC_OFFSET
    }

    /// Read **VBANK** `[11:0]`
    ///
    /// this field configures the length between vsync and valid line (by line) for dpi output
    #[inline(always)]
    pub const fn vbank(&self) -> u32 {
        (self.bits & DpiVCfg1::VBANK_MASK) >> DpiVCfg1::VBANK_OFFSET
    }

    /// Set **VSYNC** `[27:16]`
    ///
    /// this field configures the length of vsync (by line) for dpi output
    #[inline(always)]
    pub const fn with_vsync(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DpiVCfg1::VSYNC_MASK)
                | ((value << DpiVCfg1::VSYNC_OFFSET) & DpiVCfg1::VSYNC_MASK),
        }
    }

    /// Set **VBANK** `[11:0]`
    ///
    /// this field configures the length between vsync and valid line (by line) for dpi output
    #[inline(always)]
    pub const fn with_vbank(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DpiVCfg1::VBANK_MASK)
                | ((value << DpiVCfg1::VBANK_OFFSET) & DpiVCfg1::VBANK_MASK),
        }
    }
}

impl Default for DpiVCfg1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DpiVCfg1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DpiVCfg1Val {
        DpiVCfg1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DpiVCfg1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DpiVCfg1Val) -> DpiVCfg1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DPI_H_CFG0 Register
///
/// dsi bridge dpi h config register 0
///
/// Address offset: 0x0038
/// Reset value: 0x02800320
///
/// ## Fields
///
/// - **HDISP** `[27:16]` (RW)
///
///   this field configures the length of valid pixel data (by pixel num) for dpi output
///
/// - **HTOTAL** `[11:0]` (RW)
///
///   this field configures the total length of one line (by pixel num) for dpi output, must meet: reg_htotal > reg_hdisp+reg_hsync+reg_hbank
#[repr(transparent)]
pub struct DpiHCfg0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiHCfg0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiHCfg0 {
    // ======== Field Constants ========

    /// **HDISP**
    ///
    /// this field configures the length of valid pixel data (by pixel num) for dpi output
    ///
    /// Bits `[27:16]`, mask `0x0fff0000`
    pub const HDISP_OFFSET: u32 = 16;
    pub const HDISP_MASK: u32 = 0x0fff0000;

    /// **HTOTAL**
    ///
    /// this field configures the total length of one line (by pixel num) for dpi output, must meet: reg_htotal > reg_hdisp+reg_hsync+reg_hbank
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const HTOTAL_OFFSET: u32 = 0;
    pub const HTOTAL_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **HDISP** `[27:16]` (RW)
    ///
    /// this field configures the length of valid pixel data (by pixel num) for dpi output
    #[inline(always)]
    pub fn hdisp(&self) -> u32 {
        (self.value.read_bits() & Self::HDISP_MASK) >> Self::HDISP_OFFSET
    }

    /// Write **HDISP** `[27:16]` (RW)
    ///
    /// this field configures the length of valid pixel data (by pixel num) for dpi output
    #[inline(always)]
    pub fn write_hdisp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HDISP_MASK) | ((value << Self::HDISP_OFFSET) & Self::HDISP_MASK)
        });
    }

    /// **HTOTAL** `[11:0]` (RW)
    ///
    /// this field configures the total length of one line (by pixel num) for dpi output, must meet: reg_htotal > reg_hdisp+reg_hsync+reg_hbank
    #[inline(always)]
    pub fn htotal(&self) -> u32 {
        (self.value.read_bits() & Self::HTOTAL_MASK) >> Self::HTOTAL_OFFSET
    }

    /// Write **HTOTAL** `[11:0]` (RW)
    ///
    /// this field configures the total length of one line (by pixel num) for dpi output, must meet: reg_htotal > reg_hdisp+reg_hsync+reg_hbank
    #[inline(always)]
    pub fn write_htotal(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HTOTAL_MASK) | ((value << Self::HTOTAL_OFFSET) & Self::HTOTAL_MASK)
        });
    }
}

/// DPI_H_CFG0 value type for batch read/modify/write operations
///
/// Use `dpihcfg0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpihcfg0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiHCfg0Val {
    bits: u32,
}

impl DpiHCfg0Val {
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
        Self { bits: 0x02800320 }
    }

    /// Read **HDISP** `[27:16]`
    ///
    /// this field configures the length of valid pixel data (by pixel num) for dpi output
    #[inline(always)]
    pub const fn hdisp(&self) -> u32 {
        (self.bits & DpiHCfg0::HDISP_MASK) >> DpiHCfg0::HDISP_OFFSET
    }

    /// Read **HTOTAL** `[11:0]`
    ///
    /// this field configures the total length of one line (by pixel num) for dpi output, must meet: reg_htotal > reg_hdisp+reg_hsync+reg_hbank
    #[inline(always)]
    pub const fn htotal(&self) -> u32 {
        (self.bits & DpiHCfg0::HTOTAL_MASK) >> DpiHCfg0::HTOTAL_OFFSET
    }

    /// Set **HDISP** `[27:16]`
    ///
    /// this field configures the length of valid pixel data (by pixel num) for dpi output
    #[inline(always)]
    pub const fn with_hdisp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DpiHCfg0::HDISP_MASK)
                | ((value << DpiHCfg0::HDISP_OFFSET) & DpiHCfg0::HDISP_MASK),
        }
    }

    /// Set **HTOTAL** `[11:0]`
    ///
    /// this field configures the total length of one line (by pixel num) for dpi output, must meet: reg_htotal > reg_hdisp+reg_hsync+reg_hbank
    #[inline(always)]
    pub const fn with_htotal(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DpiHCfg0::HTOTAL_MASK)
                | ((value << DpiHCfg0::HTOTAL_OFFSET) & DpiHCfg0::HTOTAL_MASK),
        }
    }
}

impl Default for DpiHCfg0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DpiHCfg0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DpiHCfg0Val {
        DpiHCfg0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DpiHCfg0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DpiHCfg0Val) -> DpiHCfg0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DPI_H_CFG1 Register
///
/// dsi bridge dpi h config register 1
///
/// Address offset: 0x003c
/// Reset value: 0x00600030
///
/// ## Fields
///
/// - **HSYNC** `[27:16]` (RW)
///
///   this field configures the length of hsync (by pixel num) for dpi output
///
/// - **HBANK** `[11:0]` (RW)
///
///   this field configures the length between hsync and pixel data valid (by pixel num) for dpi output
#[repr(transparent)]
pub struct DpiHCfg1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiHCfg1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiHCfg1 {
    // ======== Field Constants ========

    /// **HSYNC**
    ///
    /// this field configures the length of hsync (by pixel num) for dpi output
    ///
    /// Bits `[27:16]`, mask `0x0fff0000`
    pub const HSYNC_OFFSET: u32 = 16;
    pub const HSYNC_MASK: u32 = 0x0fff0000;

    /// **HBANK**
    ///
    /// this field configures the length between hsync and pixel data valid (by pixel num) for dpi output
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const HBANK_OFFSET: u32 = 0;
    pub const HBANK_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **HSYNC** `[27:16]` (RW)
    ///
    /// this field configures the length of hsync (by pixel num) for dpi output
    #[inline(always)]
    pub fn hsync(&self) -> u32 {
        (self.value.read_bits() & Self::HSYNC_MASK) >> Self::HSYNC_OFFSET
    }

    /// Write **HSYNC** `[27:16]` (RW)
    ///
    /// this field configures the length of hsync (by pixel num) for dpi output
    #[inline(always)]
    pub fn write_hsync(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HSYNC_MASK) | ((value << Self::HSYNC_OFFSET) & Self::HSYNC_MASK)
        });
    }

    /// **HBANK** `[11:0]` (RW)
    ///
    /// this field configures the length between hsync and pixel data valid (by pixel num) for dpi output
    #[inline(always)]
    pub fn hbank(&self) -> u32 {
        (self.value.read_bits() & Self::HBANK_MASK) >> Self::HBANK_OFFSET
    }

    /// Write **HBANK** `[11:0]` (RW)
    ///
    /// this field configures the length between hsync and pixel data valid (by pixel num) for dpi output
    #[inline(always)]
    pub fn write_hbank(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HBANK_MASK) | ((value << Self::HBANK_OFFSET) & Self::HBANK_MASK)
        });
    }
}

/// DPI_H_CFG1 value type for batch read/modify/write operations
///
/// Use `dpihcfg1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpihcfg1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiHCfg1Val {
    bits: u32,
}

impl DpiHCfg1Val {
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
        Self { bits: 0x00600030 }
    }

    /// Read **HSYNC** `[27:16]`
    ///
    /// this field configures the length of hsync (by pixel num) for dpi output
    #[inline(always)]
    pub const fn hsync(&self) -> u32 {
        (self.bits & DpiHCfg1::HSYNC_MASK) >> DpiHCfg1::HSYNC_OFFSET
    }

    /// Read **HBANK** `[11:0]`
    ///
    /// this field configures the length between hsync and pixel data valid (by pixel num) for dpi output
    #[inline(always)]
    pub const fn hbank(&self) -> u32 {
        (self.bits & DpiHCfg1::HBANK_MASK) >> DpiHCfg1::HBANK_OFFSET
    }

    /// Set **HSYNC** `[27:16]`
    ///
    /// this field configures the length of hsync (by pixel num) for dpi output
    #[inline(always)]
    pub const fn with_hsync(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DpiHCfg1::HSYNC_MASK)
                | ((value << DpiHCfg1::HSYNC_OFFSET) & DpiHCfg1::HSYNC_MASK),
        }
    }

    /// Set **HBANK** `[11:0]`
    ///
    /// this field configures the length between hsync and pixel data valid (by pixel num) for dpi output
    #[inline(always)]
    pub const fn with_hbank(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DpiHCfg1::HBANK_MASK)
                | ((value << DpiHCfg1::HBANK_OFFSET) & DpiHCfg1::HBANK_MASK),
        }
    }
}

impl Default for DpiHCfg1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DpiHCfg1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DpiHCfg1Val {
        DpiHCfg1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DpiHCfg1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DpiHCfg1Val) -> DpiHCfg1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DPI_MISC_CONFIG Register
///
/// dsi_bridge dpi misc config register
///
/// Address offset: 0x0040
/// Reset value: 0x000019d0
///
/// ## Fields
///
/// - **FIFO_UNDERRUN_DISCARD_VCNT** `[15:4]` (RW)
///
///   this field configures the underrun interrupt musk, when underrun occurs and line cnt is less then this field
///
/// - **DPI_EN** `[0]` (RW)
///
///   this bit configures enable of dpi output, 0: disable, 1: enable
#[repr(transparent)]
pub struct DpiMiscConfig {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiMiscConfig {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiMiscConfig {
    // ======== Field Constants ========

    /// **FIFO_UNDERRUN_DISCARD_VCNT**
    ///
    /// this field configures the underrun interrupt musk, when underrun occurs and line cnt is less then this field
    ///
    /// Bits `[15:4]`, mask `0x0000fff0`
    pub const FIFO_UNDERRUN_DISCARD_VCNT_OFFSET: u32 = 4;
    pub const FIFO_UNDERRUN_DISCARD_VCNT_MASK: u32 = 0x0000fff0;

    /// **DPI_EN**
    ///
    /// this bit configures enable of dpi output, 0: disable, 1: enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DPI_EN_OFFSET: u32 = 0;
    pub const DPI_EN_MASK: u32 = 0x00000001;
    pub const DPI_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FIFO_UNDERRUN_DISCARD_VCNT** `[15:4]` (RW)
    ///
    /// this field configures the underrun interrupt musk, when underrun occurs and line cnt is less then this field
    #[inline(always)]
    pub fn fifo_underrun_discard_vcnt(&self) -> u32 {
        (self.value.read_bits() & Self::FIFO_UNDERRUN_DISCARD_VCNT_MASK)
            >> Self::FIFO_UNDERRUN_DISCARD_VCNT_OFFSET
    }

    /// Write **FIFO_UNDERRUN_DISCARD_VCNT** `[15:4]` (RW)
    ///
    /// this field configures the underrun interrupt musk, when underrun occurs and line cnt is less then this field
    #[inline(always)]
    pub fn write_fifo_underrun_discard_vcnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FIFO_UNDERRUN_DISCARD_VCNT_MASK)
                | ((value << Self::FIFO_UNDERRUN_DISCARD_VCNT_OFFSET)
                    & Self::FIFO_UNDERRUN_DISCARD_VCNT_MASK)
        });
    }

    /// **DPI_EN** `[0]` (RW)
    ///
    /// this bit configures enable of dpi output, 0: disable, 1: enable
    #[inline(always)]
    pub fn dpi_en(&self) -> bool {
        (self.value.read_bits() & Self::DPI_EN_MASK) != 0
    }

    /// Set DPI_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dpi_en(&self) {
        self.value.modify_bits(|r| r | Self::DPI_EN_MASK);
    }

    /// Clear DPI_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dpi_en(&self) {
        self.value.modify_bits(|r| r & !Self::DPI_EN_MASK);
    }
}

/// DPI_MISC_CONFIG value type for batch read/modify/write operations
///
/// Use `dpimiscconfig.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpimiscconfig.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiMiscConfigVal {
    bits: u32,
}

impl DpiMiscConfigVal {
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
        Self { bits: 0x000019d0 }
    }

    /// Read **FIFO_UNDERRUN_DISCARD_VCNT** `[15:4]`
    ///
    /// this field configures the underrun interrupt musk, when underrun occurs and line cnt is less then this field
    #[inline(always)]
    pub const fn fifo_underrun_discard_vcnt(&self) -> u32 {
        (self.bits & DpiMiscConfig::FIFO_UNDERRUN_DISCARD_VCNT_MASK)
            >> DpiMiscConfig::FIFO_UNDERRUN_DISCARD_VCNT_OFFSET
    }

    /// Read **DPI_EN** `[0:0]`
    ///
    /// this bit configures enable of dpi output, 0: disable, 1: enable
    #[inline(always)]
    pub const fn dpi_en(&self) -> bool {
        (self.bits & DpiMiscConfig::DPI_EN_MASK) != 0
    }

    /// Set **FIFO_UNDERRUN_DISCARD_VCNT** `[15:4]`
    ///
    /// this field configures the underrun interrupt musk, when underrun occurs and line cnt is less then this field
    #[inline(always)]
    pub const fn with_fifo_underrun_discard_vcnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DpiMiscConfig::FIFO_UNDERRUN_DISCARD_VCNT_MASK)
                | ((value << DpiMiscConfig::FIFO_UNDERRUN_DISCARD_VCNT_OFFSET)
                    & DpiMiscConfig::FIFO_UNDERRUN_DISCARD_VCNT_MASK),
        }
    }

    /// Set **DPI_EN** `[0:0]`
    ///
    /// this bit configures enable of dpi output, 0: disable, 1: enable
    #[inline(always)]
    pub const fn with_dpi_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DpiMiscConfig::DPI_EN_MASK
            } else {
                self.bits & !DpiMiscConfig::DPI_EN_MASK
            },
        }
    }
}

impl Default for DpiMiscConfigVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DpiMiscConfig {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DpiMiscConfigVal {
        DpiMiscConfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DpiMiscConfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DpiMiscConfigVal) -> DpiMiscConfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DPI_CONFIG_UPDATE Register
///
/// dsi_bridge dpi config update register
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **DPI_CONFIG_UPDATE** `[0]` (WO)
///
///   write 1 to this bit to update dpi config register MIPI_DSI_BRG_DPI_*
#[repr(transparent)]
pub struct DpiConfigUpdate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DpiConfigUpdate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DpiConfigUpdate {
    // ======== Field Constants ========

    /// **DPI_CONFIG_UPDATE**
    ///
    /// write 1 to this bit to update dpi config register MIPI_DSI_BRG_DPI_*
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DPI_CONFIG_UPDATE_OFFSET: u32 = 0;
    pub const DPI_CONFIG_UPDATE_MASK: u32 = 0x00000001;
    pub const DPI_CONFIG_UPDATE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **DPI_CONFIG_UPDATE** (WO)
    ///
    /// write 1 to this bit to update dpi config register MIPI_DSI_BRG_DPI_*
    #[inline(always)]
    pub fn trigger_dpi_config_update(&self) {
        self.value.write_bits(Self::DPI_CONFIG_UPDATE_MASK);
    }
}

/// DPI_CONFIG_UPDATE value type for batch read/modify/write operations
///
/// Use `dpiconfigupdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dpiconfigupdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DpiConfigUpdateVal {
    bits: u32,
}

impl DpiConfigUpdateVal {
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

    /// Set **DPI_CONFIG_UPDATE** `[0:0]`
    ///
    /// write 1 to this bit to update dpi config register MIPI_DSI_BRG_DPI_*
    #[inline(always)]
    pub const fn with_dpi_config_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DpiConfigUpdate::DPI_CONFIG_UPDATE_MASK
            } else {
                self.bits & !DpiConfigUpdate::DPI_CONFIG_UPDATE_MASK
            },
        }
    }
}

impl Default for DpiConfigUpdateVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DpiConfigUpdate {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DpiConfigUpdateVal) {
        self.value.write_bits(val.bits())
    }
}

/// INT_ENA Register
///
/// dsi_bridge interrupt enable register
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **UNDERRUN_INT_ENA** `[0]` (RW)
///
///   write 1 to enables dpi_underrun_int_st field of MIPI_DSI_BRG_INT_ST_REG controlled by dpi_underrun interrupt signal
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

    /// **UNDERRUN_INT_ENA**
    ///
    /// write 1 to enables dpi_underrun_int_st field of MIPI_DSI_BRG_INT_ST_REG controlled by dpi_underrun interrupt signal
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const UNDERRUN_INT_ENA_OFFSET: u32 = 0;
    pub const UNDERRUN_INT_ENA_MASK: u32 = 0x00000001;
    pub const UNDERRUN_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **UNDERRUN_INT_ENA** `[0]` (RW)
    ///
    /// write 1 to enables dpi_underrun_int_st field of MIPI_DSI_BRG_INT_ST_REG controlled by dpi_underrun interrupt signal
    #[inline(always)]
    pub fn underrun_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::UNDERRUN_INT_ENA_MASK) != 0
    }

    /// Set UNDERRUN_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_underrun_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::UNDERRUN_INT_ENA_MASK);
    }

    /// Clear UNDERRUN_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_underrun_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::UNDERRUN_INT_ENA_MASK);
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

    /// Read **UNDERRUN_INT_ENA** `[0:0]`
    ///
    /// write 1 to enables dpi_underrun_int_st field of MIPI_DSI_BRG_INT_ST_REG controlled by dpi_underrun interrupt signal
    #[inline(always)]
    pub const fn underrun_int_ena(&self) -> bool {
        (self.bits & IntEna::UNDERRUN_INT_ENA_MASK) != 0
    }

    /// Set **UNDERRUN_INT_ENA** `[0:0]`
    ///
    /// write 1 to enables dpi_underrun_int_st field of MIPI_DSI_BRG_INT_ST_REG controlled by dpi_underrun interrupt signal
    #[inline(always)]
    pub const fn with_underrun_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::UNDERRUN_INT_ENA_MASK
            } else {
                self.bits & !IntEna::UNDERRUN_INT_ENA_MASK
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
/// dsi_bridge interrupt clear register
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **UNDERRUN_INT_CLR** `[0]` (WO)
///
///   write 1 to this bit to clear dpi_underrun_int_raw field of MIPI_DSI_BRG_INT_RAW_REG
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

    /// **UNDERRUN_INT_CLR**
    ///
    /// write 1 to this bit to clear dpi_underrun_int_raw field of MIPI_DSI_BRG_INT_RAW_REG
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const UNDERRUN_INT_CLR_OFFSET: u32 = 0;
    pub const UNDERRUN_INT_CLR_MASK: u32 = 0x00000001;
    pub const UNDERRUN_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **UNDERRUN_INT_CLR** (WO)
    ///
    /// write 1 to this bit to clear dpi_underrun_int_raw field of MIPI_DSI_BRG_INT_RAW_REG
    #[inline(always)]
    pub fn trigger_underrun_int_clr(&self) {
        self.value.write_bits(Self::UNDERRUN_INT_CLR_MASK);
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

    /// Set **UNDERRUN_INT_CLR** `[0:0]`
    ///
    /// write 1 to this bit to clear dpi_underrun_int_raw field of MIPI_DSI_BRG_INT_RAW_REG
    #[inline(always)]
    pub const fn with_underrun_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::UNDERRUN_INT_CLR_MASK
            } else {
                self.bits & !IntClr::UNDERRUN_INT_CLR_MASK
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

/// INT_RAW Register
///
/// dsi_bridge raw interrupt register
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **UNDERRUN_INT_RAW** `[0]` (RW)
///
///   the raw interrupt status of dpi_underrun
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

    /// **UNDERRUN_INT_RAW**
    ///
    /// the raw interrupt status of dpi_underrun
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const UNDERRUN_INT_RAW_OFFSET: u32 = 0;
    pub const UNDERRUN_INT_RAW_MASK: u32 = 0x00000001;
    pub const UNDERRUN_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **UNDERRUN_INT_RAW** `[0]` (RW)
    ///
    /// the raw interrupt status of dpi_underrun
    #[inline(always)]
    pub fn underrun_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::UNDERRUN_INT_RAW_MASK) != 0
    }

    /// Set UNDERRUN_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_underrun_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::UNDERRUN_INT_RAW_MASK);
    }

    /// Clear UNDERRUN_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_underrun_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::UNDERRUN_INT_RAW_MASK);
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

    /// Read **UNDERRUN_INT_RAW** `[0:0]`
    ///
    /// the raw interrupt status of dpi_underrun
    #[inline(always)]
    pub const fn underrun_int_raw(&self) -> bool {
        (self.bits & IntRaw::UNDERRUN_INT_RAW_MASK) != 0
    }

    /// Set **UNDERRUN_INT_RAW** `[0:0]`
    ///
    /// the raw interrupt status of dpi_underrun
    #[inline(always)]
    pub const fn with_underrun_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::UNDERRUN_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::UNDERRUN_INT_RAW_MASK
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
/// dsi_bridge masked interrupt register
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **UNDERRUN_INT_ST** `[0]` (RO)
///
///   the masked interrupt status of dpi_underrun
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

    /// **UNDERRUN_INT_ST**
    ///
    /// the masked interrupt status of dpi_underrun
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const UNDERRUN_INT_ST_OFFSET: u32 = 0;
    pub const UNDERRUN_INT_ST_MASK: u32 = 0x00000001;
    pub const UNDERRUN_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **UNDERRUN_INT_ST** `[0]` (RO)
    ///
    /// the masked interrupt status of dpi_underrun
    #[inline(always)]
    pub fn underrun_int_st(&self) -> bool {
        (self.value.read_bits() & Self::UNDERRUN_INT_ST_MASK) != 0
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

    /// Read **UNDERRUN_INT_ST** `[0:0]`
    ///
    /// the masked interrupt status of dpi_underrun
    #[inline(always)]
    pub const fn underrun_int_st(&self) -> bool {
        (self.bits & IntSt::UNDERRUN_INT_ST_MASK) != 0
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

/// HOST_BIST_CTL Register
///
/// dsi_bridge host bist control register
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **BISTON** `[1]` (RW)
///
///   biston
///
/// - **BISTOK** `[0]` (RO)
///
///   bistok
#[repr(transparent)]
pub struct HostBistCtl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HostBistCtl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HostBistCtl {
    // ======== Field Constants ========

    /// **BISTON**
    ///
    /// biston
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BISTON_OFFSET: u32 = 1;
    pub const BISTON_MASK: u32 = 0x00000002;
    pub const BISTON: u32 = 1 << 1;

    /// **BISTOK**
    ///
    /// bistok
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const BISTOK_OFFSET: u32 = 0;
    pub const BISTOK_MASK: u32 = 0x00000001;
    pub const BISTOK: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BISTON** `[1]` (RW)
    ///
    /// biston
    #[inline(always)]
    pub fn biston(&self) -> bool {
        (self.value.read_bits() & Self::BISTON_MASK) != 0
    }

    /// Set BISTON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_biston(&self) {
        self.value.modify_bits(|r| r | Self::BISTON_MASK);
    }

    /// Clear BISTON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_biston(&self) {
        self.value.modify_bits(|r| r & !Self::BISTON_MASK);
    }

    /// **BISTOK** `[0]` (RO)
    ///
    /// bistok
    #[inline(always)]
    pub fn bistok(&self) -> bool {
        (self.value.read_bits() & Self::BISTOK_MASK) != 0
    }
}

/// HOST_BIST_CTL value type for batch read/modify/write operations
///
/// Use `hostbistctl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hostbistctl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HostBistCtlVal {
    bits: u32,
}

impl HostBistCtlVal {
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

    /// Read **BISTON** `[1:1]`
    ///
    /// biston
    #[inline(always)]
    pub const fn biston(&self) -> bool {
        (self.bits & HostBistCtl::BISTON_MASK) != 0
    }

    /// Read **BISTOK** `[0:0]`
    ///
    /// bistok
    #[inline(always)]
    pub const fn bistok(&self) -> bool {
        (self.bits & HostBistCtl::BISTOK_MASK) != 0
    }

    /// Set **BISTON** `[1:1]`
    ///
    /// biston
    #[inline(always)]
    pub const fn with_biston(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HostBistCtl::BISTON_MASK
            } else {
                self.bits & !HostBistCtl::BISTON_MASK
            },
        }
    }
}

impl Default for HostBistCtlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl HostBistCtl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HostBistCtlVal {
        HostBistCtlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HostBistCtlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HostBistCtlVal) -> HostBistCtlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HOST_TRIGGER_REV Register
///
/// dsi_bridge host trigger reverse control register
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **RX_TRIGGER_REV_EN** `[1]` (RW)
///
///   rx_trigger reverse. 0: disable, 1: enable
///
/// - **TX_TRIGGER_REV_EN** `[0]` (RW)
///
///   tx_trigger reverse. 0: disable, 1: enable
#[repr(transparent)]
pub struct HostTriggerRev {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HostTriggerRev {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HostTriggerRev {
    // ======== Field Constants ========

    /// **RX_TRIGGER_REV_EN**
    ///
    /// rx_trigger reverse. 0: disable, 1: enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_TRIGGER_REV_EN_OFFSET: u32 = 1;
    pub const RX_TRIGGER_REV_EN_MASK: u32 = 0x00000002;
    pub const RX_TRIGGER_REV_EN: u32 = 1 << 1;

    /// **TX_TRIGGER_REV_EN**
    ///
    /// tx_trigger reverse. 0: disable, 1: enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_TRIGGER_REV_EN_OFFSET: u32 = 0;
    pub const TX_TRIGGER_REV_EN_MASK: u32 = 0x00000001;
    pub const TX_TRIGGER_REV_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RX_TRIGGER_REV_EN** `[1]` (RW)
    ///
    /// rx_trigger reverse. 0: disable, 1: enable
    #[inline(always)]
    pub fn rx_trigger_rev_en(&self) -> bool {
        (self.value.read_bits() & Self::RX_TRIGGER_REV_EN_MASK) != 0
    }

    /// Set RX_TRIGGER_REV_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_trigger_rev_en(&self) {
        self.value.modify_bits(|r| r | Self::RX_TRIGGER_REV_EN_MASK);
    }

    /// Clear RX_TRIGGER_REV_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_trigger_rev_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_TRIGGER_REV_EN_MASK);
    }

    /// **TX_TRIGGER_REV_EN** `[0]` (RW)
    ///
    /// tx_trigger reverse. 0: disable, 1: enable
    #[inline(always)]
    pub fn tx_trigger_rev_en(&self) -> bool {
        (self.value.read_bits() & Self::TX_TRIGGER_REV_EN_MASK) != 0
    }

    /// Set TX_TRIGGER_REV_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_trigger_rev_en(&self) {
        self.value.modify_bits(|r| r | Self::TX_TRIGGER_REV_EN_MASK);
    }

    /// Clear TX_TRIGGER_REV_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_trigger_rev_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_TRIGGER_REV_EN_MASK);
    }
}

/// HOST_TRIGGER_REV value type for batch read/modify/write operations
///
/// Use `hosttriggerrev.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hosttriggerrev.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HostTriggerRevVal {
    bits: u32,
}

impl HostTriggerRevVal {
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

    /// Read **RX_TRIGGER_REV_EN** `[1:1]`
    ///
    /// rx_trigger reverse. 0: disable, 1: enable
    #[inline(always)]
    pub const fn rx_trigger_rev_en(&self) -> bool {
        (self.bits & HostTriggerRev::RX_TRIGGER_REV_EN_MASK) != 0
    }

    /// Read **TX_TRIGGER_REV_EN** `[0:0]`
    ///
    /// tx_trigger reverse. 0: disable, 1: enable
    #[inline(always)]
    pub const fn tx_trigger_rev_en(&self) -> bool {
        (self.bits & HostTriggerRev::TX_TRIGGER_REV_EN_MASK) != 0
    }

    /// Set **RX_TRIGGER_REV_EN** `[1:1]`
    ///
    /// rx_trigger reverse. 0: disable, 1: enable
    #[inline(always)]
    pub const fn with_rx_trigger_rev_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HostTriggerRev::RX_TRIGGER_REV_EN_MASK
            } else {
                self.bits & !HostTriggerRev::RX_TRIGGER_REV_EN_MASK
            },
        }
    }

    /// Set **TX_TRIGGER_REV_EN** `[0:0]`
    ///
    /// tx_trigger reverse. 0: disable, 1: enable
    #[inline(always)]
    pub const fn with_tx_trigger_rev_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HostTriggerRev::TX_TRIGGER_REV_EN_MASK
            } else {
                self.bits & !HostTriggerRev::TX_TRIGGER_REV_EN_MASK
            },
        }
    }
}

impl Default for HostTriggerRevVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl HostTriggerRev {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HostTriggerRevVal {
        HostTriggerRevVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HostTriggerRevVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HostTriggerRevVal) -> HostTriggerRevVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BLK_RAW_NUM_CFG Register
///
/// dsi_bridge block raw number control register
///
/// Address offset: 0x0068
/// Reset value: 0x00038400
///
/// ## Fields
///
/// - **BLK_RAW_NUM_TOTAL_SET** `[31]` (WO)
///
///   write 1 to reload reg_blk_raw_num_total to internal cnt
///
/// - **BLK_RAW_NUM_TOTAL** `[21:0]` (RW)
///
///   this field configures number of total block pix bits/64
#[repr(transparent)]
pub struct BlkRawNumCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BlkRawNumCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BlkRawNumCfg {
    // ======== Field Constants ========

    /// **BLK_RAW_NUM_TOTAL_SET**
    ///
    /// write 1 to reload reg_blk_raw_num_total to internal cnt
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const BLK_RAW_NUM_TOTAL_SET_OFFSET: u32 = 31;
    pub const BLK_RAW_NUM_TOTAL_SET_MASK: u32 = 0x80000000;
    pub const BLK_RAW_NUM_TOTAL_SET: u32 = 1 << 31;

    /// **BLK_RAW_NUM_TOTAL**
    ///
    /// this field configures number of total block pix bits/64
    ///
    /// Bits `[21:0]`, mask `0x003fffff`
    pub const BLK_RAW_NUM_TOTAL_OFFSET: u32 = 0;
    pub const BLK_RAW_NUM_TOTAL_MASK: u32 = 0x003fffff;

    // ======== Field Accessors ========

    /// Trigger **BLK_RAW_NUM_TOTAL_SET** (WO)
    ///
    /// write 1 to reload reg_blk_raw_num_total to internal cnt
    #[inline(always)]
    pub fn trigger_blk_raw_num_total_set(&self) {
        self.value
            .modify_bits(|r| r | Self::BLK_RAW_NUM_TOTAL_SET_MASK);
    }

    /// **BLK_RAW_NUM_TOTAL** `[21:0]` (RW)
    ///
    /// this field configures number of total block pix bits/64
    #[inline(always)]
    pub fn blk_raw_num_total(&self) -> u32 {
        (self.value.read_bits() & Self::BLK_RAW_NUM_TOTAL_MASK) >> Self::BLK_RAW_NUM_TOTAL_OFFSET
    }

    /// Write **BLK_RAW_NUM_TOTAL** `[21:0]` (RW)
    ///
    /// this field configures number of total block pix bits/64
    #[inline(always)]
    pub fn write_blk_raw_num_total(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLK_RAW_NUM_TOTAL_MASK)
                | ((value << Self::BLK_RAW_NUM_TOTAL_OFFSET) & Self::BLK_RAW_NUM_TOTAL_MASK)
        });
    }
}

/// BLK_RAW_NUM_CFG value type for batch read/modify/write operations
///
/// Use `blkrawnumcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `blkrawnumcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BlkRawNumCfgVal {
    bits: u32,
}

impl BlkRawNumCfgVal {
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
        Self { bits: 0x00038400 }
    }

    /// Read **BLK_RAW_NUM_TOTAL** `[21:0]`
    ///
    /// this field configures number of total block pix bits/64
    #[inline(always)]
    pub const fn blk_raw_num_total(&self) -> u32 {
        (self.bits & BlkRawNumCfg::BLK_RAW_NUM_TOTAL_MASK) >> BlkRawNumCfg::BLK_RAW_NUM_TOTAL_OFFSET
    }

    /// Set **BLK_RAW_NUM_TOTAL_SET** `[31:31]`
    ///
    /// write 1 to reload reg_blk_raw_num_total to internal cnt
    #[inline(always)]
    pub const fn with_blk_raw_num_total_set(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BlkRawNumCfg::BLK_RAW_NUM_TOTAL_SET_MASK
            } else {
                self.bits & !BlkRawNumCfg::BLK_RAW_NUM_TOTAL_SET_MASK
            },
        }
    }

    /// Set **BLK_RAW_NUM_TOTAL** `[21:0]`
    ///
    /// this field configures number of total block pix bits/64
    #[inline(always)]
    pub const fn with_blk_raw_num_total(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlkRawNumCfg::BLK_RAW_NUM_TOTAL_MASK)
                | ((value << BlkRawNumCfg::BLK_RAW_NUM_TOTAL_OFFSET)
                    & BlkRawNumCfg::BLK_RAW_NUM_TOTAL_MASK),
        }
    }
}

impl Default for BlkRawNumCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BlkRawNumCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BlkRawNumCfgVal {
        BlkRawNumCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BlkRawNumCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BlkRawNumCfgVal) -> BlkRawNumCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA_FRAME_INTERVAL Register
///
/// dsi_bridge dam frame interval control register
///
/// Address offset: 0x006c
/// Reset value: 0x20002409
///
/// ## Fields
///
/// - **EN** `[29]` (RW)
///
///   this bit configures enable interval between frame transfer, 0: disable, 1: enable
///
/// - **DMA_MULTIBLK_EN** `[28]` (RW)
///
///   this bit configures enable multi-blk transfer, 0: disable, 1: enable
///
/// - **DMA_FRAME_INTERVAL** `[27:10]` (RW)
///
///   this field configures the max frame_interval_cnt, frame_interval_cnt increased by 1 when frame_slot_cnt if full
///
/// - **DMA_FRAME_SLOT** `[9:0]` (RW)
///
///   this field configures the max frame_slot_cnt
#[repr(transparent)]
pub struct DmaFrameInterval {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmaFrameInterval {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmaFrameInterval {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// this bit configures enable interval between frame transfer, 0: disable, 1: enable
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const EN_OFFSET: u32 = 29;
    pub const EN_MASK: u32 = 0x20000000;
    pub const EN: u32 = 1 << 29;

    /// **DMA_MULTIBLK_EN**
    ///
    /// this bit configures enable multi-blk transfer, 0: disable, 1: enable
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const DMA_MULTIBLK_EN_OFFSET: u32 = 28;
    pub const DMA_MULTIBLK_EN_MASK: u32 = 0x10000000;
    pub const DMA_MULTIBLK_EN: u32 = 1 << 28;

    /// **DMA_FRAME_INTERVAL**
    ///
    /// this field configures the max frame_interval_cnt, frame_interval_cnt increased by 1 when frame_slot_cnt if full
    ///
    /// Bits `[27:10]`, mask `0x0ffffc00`
    pub const DMA_FRAME_INTERVAL_OFFSET: u32 = 10;
    pub const DMA_FRAME_INTERVAL_MASK: u32 = 0x0ffffc00;

    /// **DMA_FRAME_SLOT**
    ///
    /// this field configures the max frame_slot_cnt
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const DMA_FRAME_SLOT_OFFSET: u32 = 0;
    pub const DMA_FRAME_SLOT_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **EN** `[29]` (RW)
    ///
    /// this bit configures enable interval between frame transfer, 0: disable, 1: enable
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.modify_bits(|r| r | Self::EN_MASK);
    }

    /// Clear EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.modify_bits(|r| r & !Self::EN_MASK);
    }

    /// **DMA_MULTIBLK_EN** `[28]` (RW)
    ///
    /// this bit configures enable multi-blk transfer, 0: disable, 1: enable
    #[inline(always)]
    pub fn dma_multiblk_en(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MULTIBLK_EN_MASK) != 0
    }

    /// Set DMA_MULTIBLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma_multiblk_en(&self) {
        self.value.modify_bits(|r| r | Self::DMA_MULTIBLK_EN_MASK);
    }

    /// Clear DMA_MULTIBLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma_multiblk_en(&self) {
        self.value.modify_bits(|r| r & !Self::DMA_MULTIBLK_EN_MASK);
    }

    /// **DMA_FRAME_INTERVAL** `[27:10]` (RW)
    ///
    /// this field configures the max frame_interval_cnt, frame_interval_cnt increased by 1 when frame_slot_cnt if full
    #[inline(always)]
    pub fn dma_frame_interval(&self) -> u32 {
        (self.value.read_bits() & Self::DMA_FRAME_INTERVAL_MASK) >> Self::DMA_FRAME_INTERVAL_OFFSET
    }

    /// Write **DMA_FRAME_INTERVAL** `[27:10]` (RW)
    ///
    /// this field configures the max frame_interval_cnt, frame_interval_cnt increased by 1 when frame_slot_cnt if full
    #[inline(always)]
    pub fn write_dma_frame_interval(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DMA_FRAME_INTERVAL_MASK)
                | ((value << Self::DMA_FRAME_INTERVAL_OFFSET) & Self::DMA_FRAME_INTERVAL_MASK)
        });
    }

    /// **DMA_FRAME_SLOT** `[9:0]` (RW)
    ///
    /// this field configures the max frame_slot_cnt
    #[inline(always)]
    pub fn dma_frame_slot(&self) -> u32 {
        (self.value.read_bits() & Self::DMA_FRAME_SLOT_MASK) >> Self::DMA_FRAME_SLOT_OFFSET
    }

    /// Write **DMA_FRAME_SLOT** `[9:0]` (RW)
    ///
    /// this field configures the max frame_slot_cnt
    #[inline(always)]
    pub fn write_dma_frame_slot(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DMA_FRAME_SLOT_MASK)
                | ((value << Self::DMA_FRAME_SLOT_OFFSET) & Self::DMA_FRAME_SLOT_MASK)
        });
    }
}

/// DMA_FRAME_INTERVAL value type for batch read/modify/write operations
///
/// Use `dmaframeinterval.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmaframeinterval.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaFrameIntervalVal {
    bits: u32,
}

impl DmaFrameIntervalVal {
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
        Self { bits: 0x20002409 }
    }

    /// Read **EN** `[29:29]`
    ///
    /// this bit configures enable interval between frame transfer, 0: disable, 1: enable
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & DmaFrameInterval::EN_MASK) != 0
    }

    /// Read **DMA_MULTIBLK_EN** `[28:28]`
    ///
    /// this bit configures enable multi-blk transfer, 0: disable, 1: enable
    #[inline(always)]
    pub const fn dma_multiblk_en(&self) -> bool {
        (self.bits & DmaFrameInterval::DMA_MULTIBLK_EN_MASK) != 0
    }

    /// Read **DMA_FRAME_INTERVAL** `[27:10]`
    ///
    /// this field configures the max frame_interval_cnt, frame_interval_cnt increased by 1 when frame_slot_cnt if full
    #[inline(always)]
    pub const fn dma_frame_interval(&self) -> u32 {
        (self.bits & DmaFrameInterval::DMA_FRAME_INTERVAL_MASK)
            >> DmaFrameInterval::DMA_FRAME_INTERVAL_OFFSET
    }

    /// Read **DMA_FRAME_SLOT** `[9:0]`
    ///
    /// this field configures the max frame_slot_cnt
    #[inline(always)]
    pub const fn dma_frame_slot(&self) -> u32 {
        (self.bits & DmaFrameInterval::DMA_FRAME_SLOT_MASK)
            >> DmaFrameInterval::DMA_FRAME_SLOT_OFFSET
    }

    /// Set **EN** `[29:29]`
    ///
    /// this bit configures enable interval between frame transfer, 0: disable, 1: enable
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaFrameInterval::EN_MASK
            } else {
                self.bits & !DmaFrameInterval::EN_MASK
            },
        }
    }

    /// Set **DMA_MULTIBLK_EN** `[28:28]`
    ///
    /// this bit configures enable multi-blk transfer, 0: disable, 1: enable
    #[inline(always)]
    pub const fn with_dma_multiblk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaFrameInterval::DMA_MULTIBLK_EN_MASK
            } else {
                self.bits & !DmaFrameInterval::DMA_MULTIBLK_EN_MASK
            },
        }
    }

    /// Set **DMA_FRAME_INTERVAL** `[27:10]`
    ///
    /// this field configures the max frame_interval_cnt, frame_interval_cnt increased by 1 when frame_slot_cnt if full
    #[inline(always)]
    pub const fn with_dma_frame_interval(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DmaFrameInterval::DMA_FRAME_INTERVAL_MASK)
                | ((value << DmaFrameInterval::DMA_FRAME_INTERVAL_OFFSET)
                    & DmaFrameInterval::DMA_FRAME_INTERVAL_MASK),
        }
    }

    /// Set **DMA_FRAME_SLOT** `[9:0]`
    ///
    /// this field configures the max frame_slot_cnt
    #[inline(always)]
    pub const fn with_dma_frame_slot(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DmaFrameInterval::DMA_FRAME_SLOT_MASK)
                | ((value << DmaFrameInterval::DMA_FRAME_SLOT_OFFSET)
                    & DmaFrameInterval::DMA_FRAME_SLOT_MASK),
        }
    }
}

impl Default for DmaFrameIntervalVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DmaFrameInterval {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaFrameIntervalVal {
        DmaFrameIntervalVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaFrameIntervalVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaFrameIntervalVal) -> DmaFrameIntervalVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MEM_AUX_CTRL Register
///
/// dsi_bridge mem aux control register
///
/// Address offset: 0x0070
/// Reset value: 0x00001320
///
/// ## Fields
///
/// - **DSI_MEM_AUX_CTRL** `[13:0]` (RW)
///
///   this field configures dsi_bridge fifo memory aux ctrl
#[repr(transparent)]
pub struct MemAuxCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MemAuxCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MemAuxCtrl {
    // ======== Field Constants ========

    /// **DSI_MEM_AUX_CTRL**
    ///
    /// this field configures dsi_bridge fifo memory aux ctrl
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const DSI_MEM_AUX_CTRL_OFFSET: u32 = 0;
    pub const DSI_MEM_AUX_CTRL_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **DSI_MEM_AUX_CTRL** `[13:0]` (RW)
    ///
    /// this field configures dsi_bridge fifo memory aux ctrl
    #[inline(always)]
    pub fn dsi_mem_aux_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::DSI_MEM_AUX_CTRL_MASK) >> Self::DSI_MEM_AUX_CTRL_OFFSET
    }

    /// Write **DSI_MEM_AUX_CTRL** `[13:0]` (RW)
    ///
    /// this field configures dsi_bridge fifo memory aux ctrl
    #[inline(always)]
    pub fn write_dsi_mem_aux_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DSI_MEM_AUX_CTRL_MASK)
                | ((value << Self::DSI_MEM_AUX_CTRL_OFFSET) & Self::DSI_MEM_AUX_CTRL_MASK)
        });
    }
}

/// MEM_AUX_CTRL value type for batch read/modify/write operations
///
/// Use `memauxctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `memauxctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MemAuxCtrlVal {
    bits: u32,
}

impl MemAuxCtrlVal {
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

    /// Read **DSI_MEM_AUX_CTRL** `[13:0]`
    ///
    /// this field configures dsi_bridge fifo memory aux ctrl
    #[inline(always)]
    pub const fn dsi_mem_aux_ctrl(&self) -> u32 {
        (self.bits & MemAuxCtrl::DSI_MEM_AUX_CTRL_MASK) >> MemAuxCtrl::DSI_MEM_AUX_CTRL_OFFSET
    }

    /// Set **DSI_MEM_AUX_CTRL** `[13:0]`
    ///
    /// this field configures dsi_bridge fifo memory aux ctrl
    #[inline(always)]
    pub const fn with_dsi_mem_aux_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !MemAuxCtrl::DSI_MEM_AUX_CTRL_MASK)
                | ((value << MemAuxCtrl::DSI_MEM_AUX_CTRL_OFFSET)
                    & MemAuxCtrl::DSI_MEM_AUX_CTRL_MASK),
        }
    }
}

impl Default for MemAuxCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl MemAuxCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MemAuxCtrlVal {
        MemAuxCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MemAuxCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MemAuxCtrlVal) -> MemAuxCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RDN_ECO_CS Register
///
/// dsi_bridge rdn eco cs register
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **RDN_ECO_RESULT** `[1]` (RO)
///
///   rdn_eco_result
///
/// - **RDN_ECO_EN** `[0]` (RW)
///
///   rdn_eco_en
#[repr(transparent)]
pub struct RdnEcoCs {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdnEcoCs {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdnEcoCs {
    // ======== Field Constants ========

    /// **RDN_ECO_RESULT**
    ///
    /// rdn_eco_result
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RDN_ECO_RESULT_OFFSET: u32 = 1;
    pub const RDN_ECO_RESULT_MASK: u32 = 0x00000002;
    pub const RDN_ECO_RESULT: u32 = 1 << 1;

    /// **RDN_ECO_EN**
    ///
    /// rdn_eco_en
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RDN_ECO_EN_OFFSET: u32 = 0;
    pub const RDN_ECO_EN_MASK: u32 = 0x00000001;
    pub const RDN_ECO_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RDN_ECO_RESULT** `[1]` (RO)
    ///
    /// rdn_eco_result
    #[inline(always)]
    pub fn rdn_eco_result(&self) -> bool {
        (self.value.read_bits() & Self::RDN_ECO_RESULT_MASK) != 0
    }

    /// **RDN_ECO_EN** `[0]` (RW)
    ///
    /// rdn_eco_en
    #[inline(always)]
    pub fn rdn_eco_en(&self) -> bool {
        (self.value.read_bits() & Self::RDN_ECO_EN_MASK) != 0
    }

    /// Set RDN_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rdn_eco_en(&self) {
        self.value.modify_bits(|r| r | Self::RDN_ECO_EN_MASK);
    }

    /// Clear RDN_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rdn_eco_en(&self) {
        self.value.modify_bits(|r| r & !Self::RDN_ECO_EN_MASK);
    }
}

/// RDN_ECO_CS value type for batch read/modify/write operations
///
/// Use `rdnecocs.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdnecocs.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdnEcoCsVal {
    bits: u32,
}

impl RdnEcoCsVal {
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

    /// Read **RDN_ECO_RESULT** `[1:1]`
    ///
    /// rdn_eco_result
    #[inline(always)]
    pub const fn rdn_eco_result(&self) -> bool {
        (self.bits & RdnEcoCs::RDN_ECO_RESULT_MASK) != 0
    }

    /// Read **RDN_ECO_EN** `[0:0]`
    ///
    /// rdn_eco_en
    #[inline(always)]
    pub const fn rdn_eco_en(&self) -> bool {
        (self.bits & RdnEcoCs::RDN_ECO_EN_MASK) != 0
    }

    /// Set **RDN_ECO_EN** `[0:0]`
    ///
    /// rdn_eco_en
    #[inline(always)]
    pub const fn with_rdn_eco_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RdnEcoCs::RDN_ECO_EN_MASK
            } else {
                self.bits & !RdnEcoCs::RDN_ECO_EN_MASK
            },
        }
    }
}

impl Default for RdnEcoCsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdnEcoCs {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdnEcoCsVal {
        RdnEcoCsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RdnEcoCsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RdnEcoCsVal) -> RdnEcoCsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RDN_ECO_LOW Register
///
/// dsi_bridge rdn eco all low register
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **RDN_ECO_LOW** `[31:0]` (RW)
///
///   rdn_eco_low
#[repr(transparent)]
pub struct RdnEcoLow {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdnEcoLow {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdnEcoLow {
    // ======== Field Constants ========

    /// **RDN_ECO_LOW**
    ///
    /// rdn_eco_low
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RDN_ECO_LOW_OFFSET: u32 = 0;
    pub const RDN_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// rdn_eco_low
    #[inline(always)]
    pub fn rdn_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::RDN_ECO_LOW_MASK) >> Self::RDN_ECO_LOW_OFFSET
    }

    /// Write **RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// rdn_eco_low
    #[inline(always)]
    pub fn write_rdn_eco_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RDN_ECO_LOW_MASK)
                | ((value << Self::RDN_ECO_LOW_OFFSET) & Self::RDN_ECO_LOW_MASK)
        });
    }
}

/// RDN_ECO_LOW value type for batch read/modify/write operations
///
/// Use `rdnecolow.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdnecolow.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdnEcoLowVal {
    bits: u32,
}

impl RdnEcoLowVal {
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

    /// Read **RDN_ECO_LOW** `[31:0]`
    ///
    /// rdn_eco_low
    #[inline(always)]
    pub const fn rdn_eco_low(&self) -> u32 {
        (self.bits & RdnEcoLow::RDN_ECO_LOW_MASK) >> RdnEcoLow::RDN_ECO_LOW_OFFSET
    }

    /// Set **RDN_ECO_LOW** `[31:0]`
    ///
    /// rdn_eco_low
    #[inline(always)]
    pub const fn with_rdn_eco_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RdnEcoLow::RDN_ECO_LOW_MASK)
                | ((value << RdnEcoLow::RDN_ECO_LOW_OFFSET) & RdnEcoLow::RDN_ECO_LOW_MASK),
        }
    }
}

impl Default for RdnEcoLowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdnEcoLow {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdnEcoLowVal {
        RdnEcoLowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RdnEcoLowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RdnEcoLowVal) -> RdnEcoLowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RDN_ECO_HIGH Register
///
/// dsi_bridge rdn eco all high register
///
/// Address offset: 0x007c
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **RDN_ECO_HIGH** `[31:0]` (RW)
///
///   rdn_eco_high
#[repr(transparent)]
pub struct RdnEcoHigh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdnEcoHigh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdnEcoHigh {
    // ======== Field Constants ========

    /// **RDN_ECO_HIGH**
    ///
    /// rdn_eco_high
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RDN_ECO_HIGH_OFFSET: u32 = 0;
    pub const RDN_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// rdn_eco_high
    #[inline(always)]
    pub fn rdn_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::RDN_ECO_HIGH_MASK) >> Self::RDN_ECO_HIGH_OFFSET
    }

    /// Write **RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// rdn_eco_high
    #[inline(always)]
    pub fn write_rdn_eco_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RDN_ECO_HIGH_MASK)
                | ((value << Self::RDN_ECO_HIGH_OFFSET) & Self::RDN_ECO_HIGH_MASK)
        });
    }
}

/// RDN_ECO_HIGH value type for batch read/modify/write operations
///
/// Use `rdnecohigh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdnecohigh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdnEcoHighVal {
    bits: u32,
}

impl RdnEcoHighVal {
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

    /// Read **RDN_ECO_HIGH** `[31:0]`
    ///
    /// rdn_eco_high
    #[inline(always)]
    pub const fn rdn_eco_high(&self) -> u32 {
        (self.bits & RdnEcoHigh::RDN_ECO_HIGH_MASK) >> RdnEcoHigh::RDN_ECO_HIGH_OFFSET
    }

    /// Set **RDN_ECO_HIGH** `[31:0]`
    ///
    /// rdn_eco_high
    #[inline(always)]
    pub const fn with_rdn_eco_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RdnEcoHigh::RDN_ECO_HIGH_MASK)
                | ((value << RdnEcoHigh::RDN_ECO_HIGH_OFFSET) & RdnEcoHigh::RDN_ECO_HIGH_MASK),
        }
    }
}

impl Default for RdnEcoHighVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RdnEcoHigh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdnEcoHighVal {
        RdnEcoHighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RdnEcoHighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RdnEcoHighVal) -> RdnEcoHighVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HOST_CTRL Register
///
/// dsi_bridge host control register
///
/// Address offset: 0x0080
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **DSI_CFG_REF_CLK_EN** `[0]` (RW)
///
///   this bit configures the clk enable refclk and cfg_clk of dsi_host. 0: disable, 1: enable
#[repr(transparent)]
pub struct HostCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HostCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HostCtrl {
    // ======== Field Constants ========

    /// **DSI_CFG_REF_CLK_EN**
    ///
    /// this bit configures the clk enable refclk and cfg_clk of dsi_host. 0: disable, 1: enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DSI_CFG_REF_CLK_EN_OFFSET: u32 = 0;
    pub const DSI_CFG_REF_CLK_EN_MASK: u32 = 0x00000001;
    pub const DSI_CFG_REF_CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DSI_CFG_REF_CLK_EN** `[0]` (RW)
    ///
    /// this bit configures the clk enable refclk and cfg_clk of dsi_host. 0: disable, 1: enable
    #[inline(always)]
    pub fn dsi_cfg_ref_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::DSI_CFG_REF_CLK_EN_MASK) != 0
    }

    /// Set DSI_CFG_REF_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dsi_cfg_ref_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::DSI_CFG_REF_CLK_EN_MASK);
    }

    /// Clear DSI_CFG_REF_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dsi_cfg_ref_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::DSI_CFG_REF_CLK_EN_MASK);
    }
}

/// HOST_CTRL value type for batch read/modify/write operations
///
/// Use `hostctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hostctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HostCtrlVal {
    bits: u32,
}

impl HostCtrlVal {
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

    /// Read **DSI_CFG_REF_CLK_EN** `[0:0]`
    ///
    /// this bit configures the clk enable refclk and cfg_clk of dsi_host. 0: disable, 1: enable
    #[inline(always)]
    pub const fn dsi_cfg_ref_clk_en(&self) -> bool {
        (self.bits & HostCtrl::DSI_CFG_REF_CLK_EN_MASK) != 0
    }

    /// Set **DSI_CFG_REF_CLK_EN** `[0:0]`
    ///
    /// this bit configures the clk enable refclk and cfg_clk of dsi_host. 0: disable, 1: enable
    #[inline(always)]
    pub const fn with_dsi_cfg_ref_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HostCtrl::DSI_CFG_REF_CLK_EN_MASK
            } else {
                self.bits & !HostCtrl::DSI_CFG_REF_CLK_EN_MASK
            },
        }
    }
}

impl Default for HostCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl HostCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HostCtrlVal {
        HostCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HostCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HostCtrlVal) -> HostCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MEM_CLK_CTRL Register
///
/// dsi_bridge mem force on control register
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **DSI_MEM_CLK_FORCE_ON** `[1]` (RW)
///
///   this bit configures the clock force on of dpi fifo memory. 0: disable, 1: force on
///
/// - **DSI_BRIDGE_MEM_CLK_FORCE_ON** `[0]` (RW)
///
///   this bit configures the clock force on of dsi_bridge fifo memory. 0: disable, 1: force on
#[repr(transparent)]
pub struct MemClkCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MemClkCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MemClkCtrl {
    // ======== Field Constants ========

    /// **DSI_MEM_CLK_FORCE_ON**
    ///
    /// this bit configures the clock force on of dpi fifo memory. 0: disable, 1: force on
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DSI_MEM_CLK_FORCE_ON_OFFSET: u32 = 1;
    pub const DSI_MEM_CLK_FORCE_ON_MASK: u32 = 0x00000002;
    pub const DSI_MEM_CLK_FORCE_ON: u32 = 1 << 1;

    /// **DSI_BRIDGE_MEM_CLK_FORCE_ON**
    ///
    /// this bit configures the clock force on of dsi_bridge fifo memory. 0: disable, 1: force on
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DSI_BRIDGE_MEM_CLK_FORCE_ON_OFFSET: u32 = 0;
    pub const DSI_BRIDGE_MEM_CLK_FORCE_ON_MASK: u32 = 0x00000001;
    pub const DSI_BRIDGE_MEM_CLK_FORCE_ON: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DSI_MEM_CLK_FORCE_ON** `[1]` (RW)
    ///
    /// this bit configures the clock force on of dpi fifo memory. 0: disable, 1: force on
    #[inline(always)]
    pub fn dsi_mem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::DSI_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set DSI_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dsi_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::DSI_MEM_CLK_FORCE_ON_MASK);
    }

    /// Clear DSI_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dsi_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::DSI_MEM_CLK_FORCE_ON_MASK);
    }

    /// **DSI_BRIDGE_MEM_CLK_FORCE_ON** `[0]` (RW)
    ///
    /// this bit configures the clock force on of dsi_bridge fifo memory. 0: disable, 1: force on
    #[inline(always)]
    pub fn dsi_bridge_mem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::DSI_BRIDGE_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set DSI_BRIDGE_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dsi_bridge_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::DSI_BRIDGE_MEM_CLK_FORCE_ON_MASK);
    }

    /// Clear DSI_BRIDGE_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dsi_bridge_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::DSI_BRIDGE_MEM_CLK_FORCE_ON_MASK);
    }
}

/// MEM_CLK_CTRL value type for batch read/modify/write operations
///
/// Use `memclkctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `memclkctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MemClkCtrlVal {
    bits: u32,
}

impl MemClkCtrlVal {
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

    /// Read **DSI_MEM_CLK_FORCE_ON** `[1:1]`
    ///
    /// this bit configures the clock force on of dpi fifo memory. 0: disable, 1: force on
    #[inline(always)]
    pub const fn dsi_mem_clk_force_on(&self) -> bool {
        (self.bits & MemClkCtrl::DSI_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **DSI_BRIDGE_MEM_CLK_FORCE_ON** `[0:0]`
    ///
    /// this bit configures the clock force on of dsi_bridge fifo memory. 0: disable, 1: force on
    #[inline(always)]
    pub const fn dsi_bridge_mem_clk_force_on(&self) -> bool {
        (self.bits & MemClkCtrl::DSI_BRIDGE_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set **DSI_MEM_CLK_FORCE_ON** `[1:1]`
    ///
    /// this bit configures the clock force on of dpi fifo memory. 0: disable, 1: force on
    #[inline(always)]
    pub const fn with_dsi_mem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MemClkCtrl::DSI_MEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !MemClkCtrl::DSI_MEM_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **DSI_BRIDGE_MEM_CLK_FORCE_ON** `[0:0]`
    ///
    /// this bit configures the clock force on of dsi_bridge fifo memory. 0: disable, 1: force on
    #[inline(always)]
    pub const fn with_dsi_bridge_mem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MemClkCtrl::DSI_BRIDGE_MEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !MemClkCtrl::DSI_BRIDGE_MEM_CLK_FORCE_ON_MASK
            },
        }
    }
}

impl Default for MemClkCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MemClkCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MemClkCtrlVal {
        MemClkCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MemClkCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MemClkCtrlVal) -> MemClkCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA_FLOW_CTRL Register
///
/// dsi_bridge dma flow controller register
///
/// Address offset: 0x0088
/// Reset value: 0x00000011
///
/// ## Fields
///
/// - **DMA_FLOW_MULTIBLK_NUM** `[7:4]` (RW)
///
///   this field configures the num of blocks when multi-blk is enable and dmac as flow controller
///
/// - **DSI_DMA_FLOW_CONTROLLER** `[0]` (RW)
///
///   this bit configures the flow controller, 0: dmac as flow controller, 1:dsi_bridge as flow controller
#[repr(transparent)]
pub struct DmaFlowCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmaFlowCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmaFlowCtrl {
    // ======== Field Constants ========

    /// **DMA_FLOW_MULTIBLK_NUM**
    ///
    /// this field configures the num of blocks when multi-blk is enable and dmac as flow controller
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const DMA_FLOW_MULTIBLK_NUM_OFFSET: u32 = 4;
    pub const DMA_FLOW_MULTIBLK_NUM_MASK: u32 = 0x000000f0;

    /// **DSI_DMA_FLOW_CONTROLLER**
    ///
    /// this bit configures the flow controller, 0: dmac as flow controller, 1:dsi_bridge as flow controller
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DSI_DMA_FLOW_CONTROLLER_OFFSET: u32 = 0;
    pub const DSI_DMA_FLOW_CONTROLLER_MASK: u32 = 0x00000001;
    pub const DSI_DMA_FLOW_CONTROLLER: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DMA_FLOW_MULTIBLK_NUM** `[7:4]` (RW)
    ///
    /// this field configures the num of blocks when multi-blk is enable and dmac as flow controller
    #[inline(always)]
    pub fn dma_flow_multiblk_num(&self) -> u32 {
        (self.value.read_bits() & Self::DMA_FLOW_MULTIBLK_NUM_MASK)
            >> Self::DMA_FLOW_MULTIBLK_NUM_OFFSET
    }

    /// Write **DMA_FLOW_MULTIBLK_NUM** `[7:4]` (RW)
    ///
    /// this field configures the num of blocks when multi-blk is enable and dmac as flow controller
    #[inline(always)]
    pub fn write_dma_flow_multiblk_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DMA_FLOW_MULTIBLK_NUM_MASK)
                | ((value << Self::DMA_FLOW_MULTIBLK_NUM_OFFSET) & Self::DMA_FLOW_MULTIBLK_NUM_MASK)
        });
    }

    /// **DSI_DMA_FLOW_CONTROLLER** `[0]` (RW)
    ///
    /// this bit configures the flow controller, 0: dmac as flow controller, 1:dsi_bridge as flow controller
    #[inline(always)]
    pub fn dsi_dma_flow_controller(&self) -> bool {
        (self.value.read_bits() & Self::DSI_DMA_FLOW_CONTROLLER_MASK) != 0
    }

    /// Set DSI_DMA_FLOW_CONTROLLER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dsi_dma_flow_controller(&self) {
        self.value
            .modify_bits(|r| r | Self::DSI_DMA_FLOW_CONTROLLER_MASK);
    }

    /// Clear DSI_DMA_FLOW_CONTROLLER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dsi_dma_flow_controller(&self) {
        self.value
            .modify_bits(|r| r & !Self::DSI_DMA_FLOW_CONTROLLER_MASK);
    }
}

/// DMA_FLOW_CTRL value type for batch read/modify/write operations
///
/// Use `dmaflowctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmaflowctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaFlowCtrlVal {
    bits: u32,
}

impl DmaFlowCtrlVal {
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
        Self { bits: 0x00000011 }
    }

    /// Read **DMA_FLOW_MULTIBLK_NUM** `[7:4]`
    ///
    /// this field configures the num of blocks when multi-blk is enable and dmac as flow controller
    #[inline(always)]
    pub const fn dma_flow_multiblk_num(&self) -> u32 {
        (self.bits & DmaFlowCtrl::DMA_FLOW_MULTIBLK_NUM_MASK)
            >> DmaFlowCtrl::DMA_FLOW_MULTIBLK_NUM_OFFSET
    }

    /// Read **DSI_DMA_FLOW_CONTROLLER** `[0:0]`
    ///
    /// this bit configures the flow controller, 0: dmac as flow controller, 1:dsi_bridge as flow controller
    #[inline(always)]
    pub const fn dsi_dma_flow_controller(&self) -> bool {
        (self.bits & DmaFlowCtrl::DSI_DMA_FLOW_CONTROLLER_MASK) != 0
    }

    /// Set **DMA_FLOW_MULTIBLK_NUM** `[7:4]`
    ///
    /// this field configures the num of blocks when multi-blk is enable and dmac as flow controller
    #[inline(always)]
    pub const fn with_dma_flow_multiblk_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DmaFlowCtrl::DMA_FLOW_MULTIBLK_NUM_MASK)
                | ((value << DmaFlowCtrl::DMA_FLOW_MULTIBLK_NUM_OFFSET)
                    & DmaFlowCtrl::DMA_FLOW_MULTIBLK_NUM_MASK),
        }
    }

    /// Set **DSI_DMA_FLOW_CONTROLLER** `[0:0]`
    ///
    /// this bit configures the flow controller, 0: dmac as flow controller, 1:dsi_bridge as flow controller
    #[inline(always)]
    pub const fn with_dsi_dma_flow_controller(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaFlowCtrl::DSI_DMA_FLOW_CONTROLLER_MASK
            } else {
                self.bits & !DmaFlowCtrl::DSI_DMA_FLOW_CONTROLLER_MASK
            },
        }
    }
}

impl Default for DmaFlowCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DmaFlowCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaFlowCtrlVal {
        DmaFlowCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaFlowCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaFlowCtrlVal) -> DmaFlowCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RAW_BUF_ALMOST_EMPTY_THRD Register
///
/// dsi_bridge buffer empty threshold register
///
/// Address offset: 0x008c
/// Reset value: 0x00000200
///
/// ## Fields
///
/// - **DSI_RAW_BUF_ALMOST_EMPTY_THRD** `[10:0]` (RW)
///
///   this field configures the fifo almost empty threshold, is valid only when dmac as flow controller
#[repr(transparent)]
pub struct RawBufAlmostEmptyThrd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RawBufAlmostEmptyThrd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RawBufAlmostEmptyThrd {
    // ======== Field Constants ========

    /// **DSI_RAW_BUF_ALMOST_EMPTY_THRD**
    ///
    /// this field configures the fifo almost empty threshold, is valid only when dmac as flow controller
    ///
    /// Bits `[10:0]`, mask `0x000007ff`
    pub const DSI_RAW_BUF_ALMOST_EMPTY_THRD_OFFSET: u32 = 0;
    pub const DSI_RAW_BUF_ALMOST_EMPTY_THRD_MASK: u32 = 0x000007ff;

    // ======== Field Accessors ========

    /// **DSI_RAW_BUF_ALMOST_EMPTY_THRD** `[10:0]` (RW)
    ///
    /// this field configures the fifo almost empty threshold, is valid only when dmac as flow controller
    #[inline(always)]
    pub fn dsi_raw_buf_almost_empty_thrd(&self) -> u32 {
        (self.value.read_bits() & Self::DSI_RAW_BUF_ALMOST_EMPTY_THRD_MASK)
            >> Self::DSI_RAW_BUF_ALMOST_EMPTY_THRD_OFFSET
    }

    /// Write **DSI_RAW_BUF_ALMOST_EMPTY_THRD** `[10:0]` (RW)
    ///
    /// this field configures the fifo almost empty threshold, is valid only when dmac as flow controller
    #[inline(always)]
    pub fn write_dsi_raw_buf_almost_empty_thrd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DSI_RAW_BUF_ALMOST_EMPTY_THRD_MASK)
                | ((value << Self::DSI_RAW_BUF_ALMOST_EMPTY_THRD_OFFSET)
                    & Self::DSI_RAW_BUF_ALMOST_EMPTY_THRD_MASK)
        });
    }
}

/// RAW_BUF_ALMOST_EMPTY_THRD value type for batch read/modify/write operations
///
/// Use `rawbufalmostemptythrd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rawbufalmostemptythrd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RawBufAlmostEmptyThrdVal {
    bits: u32,
}

impl RawBufAlmostEmptyThrdVal {
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
        Self { bits: 0x00000200 }
    }

    /// Read **DSI_RAW_BUF_ALMOST_EMPTY_THRD** `[10:0]`
    ///
    /// this field configures the fifo almost empty threshold, is valid only when dmac as flow controller
    #[inline(always)]
    pub const fn dsi_raw_buf_almost_empty_thrd(&self) -> u32 {
        (self.bits & RawBufAlmostEmptyThrd::DSI_RAW_BUF_ALMOST_EMPTY_THRD_MASK)
            >> RawBufAlmostEmptyThrd::DSI_RAW_BUF_ALMOST_EMPTY_THRD_OFFSET
    }

    /// Set **DSI_RAW_BUF_ALMOST_EMPTY_THRD** `[10:0]`
    ///
    /// this field configures the fifo almost empty threshold, is valid only when dmac as flow controller
    #[inline(always)]
    pub const fn with_dsi_raw_buf_almost_empty_thrd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RawBufAlmostEmptyThrd::DSI_RAW_BUF_ALMOST_EMPTY_THRD_MASK)
                | ((value << RawBufAlmostEmptyThrd::DSI_RAW_BUF_ALMOST_EMPTY_THRD_OFFSET)
                    & RawBufAlmostEmptyThrd::DSI_RAW_BUF_ALMOST_EMPTY_THRD_MASK),
        }
    }
}

impl Default for RawBufAlmostEmptyThrdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RawBufAlmostEmptyThrd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RawBufAlmostEmptyThrdVal {
        RawBufAlmostEmptyThrdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RawBufAlmostEmptyThrdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RawBufAlmostEmptyThrdVal) -> RawBufAlmostEmptyThrdVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// YUV_CFG Register
///
/// dsi_bridge yuv format config register
///
/// Address offset: 0x0090
///
/// ## Fields
///
/// - **YUV422_FORMAT** `[3:2]` (RW)
///
///   this field configures yuv422 store format, 0: yuyv, 1: yvyu, 2: uyvy, 3: vyuy
///
/// - **YUV_PIX_ENDIAN** `[1]` (RW)
///
///   this bit configures yuv pixel endian, 0: y0u0y1v1y2u2y3v3, 1: y3u3y2v2y1u1y0v0
///
/// - **PROTOCAL** `[0]` (RW)
///
///   this bit configures yuv protoocl, 0: bt.601, 1: bt.709
#[repr(transparent)]
pub struct YuvCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YuvCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YuvCfg {
    // ======== Field Constants ========

    /// **YUV422_FORMAT**
    ///
    /// this field configures yuv422 store format, 0: yuyv, 1: yvyu, 2: uyvy, 3: vyuy
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const YUV422_FORMAT_OFFSET: u32 = 2;
    pub const YUV422_FORMAT_MASK: u32 = 0x0000000c;

    /// **YUV_PIX_ENDIAN**
    ///
    /// this bit configures yuv pixel endian, 0: y0u0y1v1y2u2y3v3, 1: y3u3y2v2y1u1y0v0
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const YUV_PIX_ENDIAN_OFFSET: u32 = 1;
    pub const YUV_PIX_ENDIAN_MASK: u32 = 0x00000002;
    pub const YUV_PIX_ENDIAN: u32 = 1 << 1;

    /// **PROTOCAL**
    ///
    /// this bit configures yuv protoocl, 0: bt.601, 1: bt.709
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PROTOCAL_OFFSET: u32 = 0;
    pub const PROTOCAL_MASK: u32 = 0x00000001;
    pub const PROTOCAL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **YUV422_FORMAT** `[3:2]` (RW)
    ///
    /// this field configures yuv422 store format, 0: yuyv, 1: yvyu, 2: uyvy, 3: vyuy
    #[inline(always)]
    pub fn yuv422_format(&self) -> u32 {
        (self.value.read_bits() & Self::YUV422_FORMAT_MASK) >> Self::YUV422_FORMAT_OFFSET
    }

    /// Write **YUV422_FORMAT** `[3:2]` (RW)
    ///
    /// this field configures yuv422 store format, 0: yuyv, 1: yvyu, 2: uyvy, 3: vyuy
    #[inline(always)]
    pub fn write_yuv422_format(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::YUV422_FORMAT_MASK)
                | ((value << Self::YUV422_FORMAT_OFFSET) & Self::YUV422_FORMAT_MASK)
        });
    }

    /// **YUV_PIX_ENDIAN** `[1]` (RW)
    ///
    /// this bit configures yuv pixel endian, 0: y0u0y1v1y2u2y3v3, 1: y3u3y2v2y1u1y0v0
    #[inline(always)]
    pub fn yuv_pix_endian(&self) -> bool {
        (self.value.read_bits() & Self::YUV_PIX_ENDIAN_MASK) != 0
    }

    /// Set YUV_PIX_ENDIAN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_yuv_pix_endian(&self) {
        self.value.modify_bits(|r| r | Self::YUV_PIX_ENDIAN_MASK);
    }

    /// Clear YUV_PIX_ENDIAN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_yuv_pix_endian(&self) {
        self.value.modify_bits(|r| r & !Self::YUV_PIX_ENDIAN_MASK);
    }

    /// **PROTOCAL** `[0]` (RW)
    ///
    /// this bit configures yuv protoocl, 0: bt.601, 1: bt.709
    #[inline(always)]
    pub fn protocal(&self) -> bool {
        (self.value.read_bits() & Self::PROTOCAL_MASK) != 0
    }

    /// Set PROTOCAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_protocal(&self) {
        self.value.modify_bits(|r| r | Self::PROTOCAL_MASK);
    }

    /// Clear PROTOCAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_protocal(&self) {
        self.value.modify_bits(|r| r & !Self::PROTOCAL_MASK);
    }
}

/// YUV_CFG value type for batch read/modify/write operations
///
/// Use `yuvcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `yuvcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct YuvCfgVal {
    bits: u32,
}

impl YuvCfgVal {
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

    /// Read **YUV422_FORMAT** `[3:2]`
    ///
    /// this field configures yuv422 store format, 0: yuyv, 1: yvyu, 2: uyvy, 3: vyuy
    #[inline(always)]
    pub const fn yuv422_format(&self) -> u32 {
        (self.bits & YuvCfg::YUV422_FORMAT_MASK) >> YuvCfg::YUV422_FORMAT_OFFSET
    }

    /// Read **YUV_PIX_ENDIAN** `[1:1]`
    ///
    /// this bit configures yuv pixel endian, 0: y0u0y1v1y2u2y3v3, 1: y3u3y2v2y1u1y0v0
    #[inline(always)]
    pub const fn yuv_pix_endian(&self) -> bool {
        (self.bits & YuvCfg::YUV_PIX_ENDIAN_MASK) != 0
    }

    /// Read **PROTOCAL** `[0:0]`
    ///
    /// this bit configures yuv protoocl, 0: bt.601, 1: bt.709
    #[inline(always)]
    pub const fn protocal(&self) -> bool {
        (self.bits & YuvCfg::PROTOCAL_MASK) != 0
    }

    /// Set **YUV422_FORMAT** `[3:2]`
    ///
    /// this field configures yuv422 store format, 0: yuyv, 1: yvyu, 2: uyvy, 3: vyuy
    #[inline(always)]
    pub const fn with_yuv422_format(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !YuvCfg::YUV422_FORMAT_MASK)
                | ((value << YuvCfg::YUV422_FORMAT_OFFSET) & YuvCfg::YUV422_FORMAT_MASK),
        }
    }

    /// Set **YUV_PIX_ENDIAN** `[1:1]`
    ///
    /// this bit configures yuv pixel endian, 0: y0u0y1v1y2u2y3v3, 1: y3u3y2v2y1u1y0v0
    #[inline(always)]
    pub const fn with_yuv_pix_endian(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | YuvCfg::YUV_PIX_ENDIAN_MASK
            } else {
                self.bits & !YuvCfg::YUV_PIX_ENDIAN_MASK
            },
        }
    }

    /// Set **PROTOCAL** `[0:0]`
    ///
    /// this bit configures yuv protoocl, 0: bt.601, 1: bt.709
    #[inline(always)]
    pub const fn with_protocal(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | YuvCfg::PROTOCAL_MASK
            } else {
                self.bits & !YuvCfg::PROTOCAL_MASK
            },
        }
    }
}

impl Default for YuvCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl YuvCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> YuvCfgVal {
        YuvCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: YuvCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(YuvCfgVal) -> YuvCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_LP_LOOPBACK_CTRL Register
///
/// dsi phy lp_loopback test ctrl
///
/// Address offset: 0x0094
///
/// ## Fields
///
/// - **PHY_LP_LOOPBACK_OK** `[31]` (RO)
///
///   result of dsi phy lp_loopback test
///
/// - **PHY_LP_LOOPBACK_EN** `[30]` (RW)
///
///   dsi phy lp_loopback ctrl en
///
/// - **PHY_LP_LOOPBACK_CHECK_DONE** `[29]` (RO)
///
///   dsi phy lp_loopback test check done
///
/// - **PHY_LP_LOOPBACK_CHECK** `[28]` (WO)
///
///   dsi phy lp_loopback test start check
///
/// - **PHY_LP_BASEDIR_0** `[27]` (RW)
///
///   basedir_0 ctrl when enable dsi phy lp_loopback_test
///
/// - **PHY_LP_TXLPDTESC_0** `[26]` (RW)
///
///   txlpdtesc_0 ctrl when enable dsi phy lp_loopback_test
///
/// - **PHY_LP_TXVALIDESC_0** `[25]` (RW)
///
///   txvalidesc_0 ctrl when enable dsi phy lp_loopback_test
///
/// - **PHY_LP_TXREQUESTESC_0** `[24]` (RW)
///
///   txrequestesc_0 ctrl when enable dsi phy lp_loopback_test
///
/// - **PHY_LP_TXDATAESC_0** `[23:16]` (RW)
///
///   txdataesc_0 ctrl when enable dsi phy lp_loopback_test
///
/// - **PHY_LP_BASEDIR_1** `[11]` (RW)
///
///   basedir_1 ctrl when enable dsi phy lp_loopback_test
///
/// - **PHY_LP_TXLPDTESC_1** `[10]` (RW)
///
///   txlpdtesc_1 ctrl when enable dsi phy lp_loopback_test
///
/// - **PHY_LP_TXVALIDESC_1** `[9]` (RW)
///
///   txvalidesc_1 ctrl when enable dsi phy lp_loopback_test
///
/// - **PHY_LP_TXREQUESTESC_1** `[8]` (RW)
///
///   txrequestesc_1 ctrl when enable dsi phy lp_loopback_test
///
/// - **PHY_LP_TXDATAESC_1** `[7:0]` (RW)
///
///   txdataesc_1 ctrl when enable dsi phy lp_loopback_test
#[repr(transparent)]
pub struct PhyLpLoopbackCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyLpLoopbackCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyLpLoopbackCtrl {
    // ======== Field Constants ========

    /// **PHY_LP_LOOPBACK_OK**
    ///
    /// result of dsi phy lp_loopback test
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const PHY_LP_LOOPBACK_OK_OFFSET: u32 = 31;
    pub const PHY_LP_LOOPBACK_OK_MASK: u32 = 0x80000000;
    pub const PHY_LP_LOOPBACK_OK: u32 = 1 << 31;

    /// **PHY_LP_LOOPBACK_EN**
    ///
    /// dsi phy lp_loopback ctrl en
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const PHY_LP_LOOPBACK_EN_OFFSET: u32 = 30;
    pub const PHY_LP_LOOPBACK_EN_MASK: u32 = 0x40000000;
    pub const PHY_LP_LOOPBACK_EN: u32 = 1 << 30;

    /// **PHY_LP_LOOPBACK_CHECK_DONE**
    ///
    /// dsi phy lp_loopback test check done
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const PHY_LP_LOOPBACK_CHECK_DONE_OFFSET: u32 = 29;
    pub const PHY_LP_LOOPBACK_CHECK_DONE_MASK: u32 = 0x20000000;
    pub const PHY_LP_LOOPBACK_CHECK_DONE: u32 = 1 << 29;

    /// **PHY_LP_LOOPBACK_CHECK**
    ///
    /// dsi phy lp_loopback test start check
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const PHY_LP_LOOPBACK_CHECK_OFFSET: u32 = 28;
    pub const PHY_LP_LOOPBACK_CHECK_MASK: u32 = 0x10000000;
    pub const PHY_LP_LOOPBACK_CHECK: u32 = 1 << 28;

    /// **PHY_LP_BASEDIR_0**
    ///
    /// basedir_0 ctrl when enable dsi phy lp_loopback_test
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const PHY_LP_BASEDIR_0_OFFSET: u32 = 27;
    pub const PHY_LP_BASEDIR_0_MASK: u32 = 0x08000000;
    pub const PHY_LP_BASEDIR_0: u32 = 1 << 27;

    /// **PHY_LP_TXLPDTESC_0**
    ///
    /// txlpdtesc_0 ctrl when enable dsi phy lp_loopback_test
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const PHY_LP_TXLPDTESC_0_OFFSET: u32 = 26;
    pub const PHY_LP_TXLPDTESC_0_MASK: u32 = 0x04000000;
    pub const PHY_LP_TXLPDTESC_0: u32 = 1 << 26;

    /// **PHY_LP_TXVALIDESC_0**
    ///
    /// txvalidesc_0 ctrl when enable dsi phy lp_loopback_test
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const PHY_LP_TXVALIDESC_0_OFFSET: u32 = 25;
    pub const PHY_LP_TXVALIDESC_0_MASK: u32 = 0x02000000;
    pub const PHY_LP_TXVALIDESC_0: u32 = 1 << 25;

    /// **PHY_LP_TXREQUESTESC_0**
    ///
    /// txrequestesc_0 ctrl when enable dsi phy lp_loopback_test
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const PHY_LP_TXREQUESTESC_0_OFFSET: u32 = 24;
    pub const PHY_LP_TXREQUESTESC_0_MASK: u32 = 0x01000000;
    pub const PHY_LP_TXREQUESTESC_0: u32 = 1 << 24;

    /// **PHY_LP_TXDATAESC_0**
    ///
    /// txdataesc_0 ctrl when enable dsi phy lp_loopback_test
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const PHY_LP_TXDATAESC_0_OFFSET: u32 = 16;
    pub const PHY_LP_TXDATAESC_0_MASK: u32 = 0x00ff0000;

    /// **PHY_LP_BASEDIR_1**
    ///
    /// basedir_1 ctrl when enable dsi phy lp_loopback_test
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const PHY_LP_BASEDIR_1_OFFSET: u32 = 11;
    pub const PHY_LP_BASEDIR_1_MASK: u32 = 0x00000800;
    pub const PHY_LP_BASEDIR_1: u32 = 1 << 11;

    /// **PHY_LP_TXLPDTESC_1**
    ///
    /// txlpdtesc_1 ctrl when enable dsi phy lp_loopback_test
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const PHY_LP_TXLPDTESC_1_OFFSET: u32 = 10;
    pub const PHY_LP_TXLPDTESC_1_MASK: u32 = 0x00000400;
    pub const PHY_LP_TXLPDTESC_1: u32 = 1 << 10;

    /// **PHY_LP_TXVALIDESC_1**
    ///
    /// txvalidesc_1 ctrl when enable dsi phy lp_loopback_test
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const PHY_LP_TXVALIDESC_1_OFFSET: u32 = 9;
    pub const PHY_LP_TXVALIDESC_1_MASK: u32 = 0x00000200;
    pub const PHY_LP_TXVALIDESC_1: u32 = 1 << 9;

    /// **PHY_LP_TXREQUESTESC_1**
    ///
    /// txrequestesc_1 ctrl when enable dsi phy lp_loopback_test
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const PHY_LP_TXREQUESTESC_1_OFFSET: u32 = 8;
    pub const PHY_LP_TXREQUESTESC_1_MASK: u32 = 0x00000100;
    pub const PHY_LP_TXREQUESTESC_1: u32 = 1 << 8;

    /// **PHY_LP_TXDATAESC_1**
    ///
    /// txdataesc_1 ctrl when enable dsi phy lp_loopback_test
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const PHY_LP_TXDATAESC_1_OFFSET: u32 = 0;
    pub const PHY_LP_TXDATAESC_1_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **PHY_LP_LOOPBACK_OK** `[31]` (RO)
    ///
    /// result of dsi phy lp_loopback test
    #[inline(always)]
    pub fn phy_lp_loopback_ok(&self) -> bool {
        (self.value.read_bits() & Self::PHY_LP_LOOPBACK_OK_MASK) != 0
    }

    /// **PHY_LP_LOOPBACK_EN** `[30]` (RW)
    ///
    /// dsi phy lp_loopback ctrl en
    #[inline(always)]
    pub fn phy_lp_loopback_en(&self) -> bool {
        (self.value.read_bits() & Self::PHY_LP_LOOPBACK_EN_MASK) != 0
    }

    /// Set PHY_LP_LOOPBACK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_lp_loopback_en(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_LP_LOOPBACK_EN_MASK);
    }

    /// Clear PHY_LP_LOOPBACK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_lp_loopback_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_LP_LOOPBACK_EN_MASK);
    }

    /// **PHY_LP_LOOPBACK_CHECK_DONE** `[29]` (RO)
    ///
    /// dsi phy lp_loopback test check done
    #[inline(always)]
    pub fn phy_lp_loopback_check_done(&self) -> bool {
        (self.value.read_bits() & Self::PHY_LP_LOOPBACK_CHECK_DONE_MASK) != 0
    }

    /// Trigger **PHY_LP_LOOPBACK_CHECK** (WO)
    ///
    /// dsi phy lp_loopback test start check
    #[inline(always)]
    pub fn trigger_phy_lp_loopback_check(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_LP_LOOPBACK_CHECK_MASK);
    }

    /// **PHY_LP_BASEDIR_0** `[27]` (RW)
    ///
    /// basedir_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub fn phy_lp_basedir_0(&self) -> bool {
        (self.value.read_bits() & Self::PHY_LP_BASEDIR_0_MASK) != 0
    }

    /// Set PHY_LP_BASEDIR_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_lp_basedir_0(&self) {
        self.value.modify_bits(|r| r | Self::PHY_LP_BASEDIR_0_MASK);
    }

    /// Clear PHY_LP_BASEDIR_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_lp_basedir_0(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_LP_BASEDIR_0_MASK);
    }

    /// **PHY_LP_TXLPDTESC_0** `[26]` (RW)
    ///
    /// txlpdtesc_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub fn phy_lp_txlpdtesc_0(&self) -> bool {
        (self.value.read_bits() & Self::PHY_LP_TXLPDTESC_0_MASK) != 0
    }

    /// Set PHY_LP_TXLPDTESC_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_lp_txlpdtesc_0(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_LP_TXLPDTESC_0_MASK);
    }

    /// Clear PHY_LP_TXLPDTESC_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_lp_txlpdtesc_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_LP_TXLPDTESC_0_MASK);
    }

    /// **PHY_LP_TXVALIDESC_0** `[25]` (RW)
    ///
    /// txvalidesc_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub fn phy_lp_txvalidesc_0(&self) -> bool {
        (self.value.read_bits() & Self::PHY_LP_TXVALIDESC_0_MASK) != 0
    }

    /// Set PHY_LP_TXVALIDESC_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_lp_txvalidesc_0(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_LP_TXVALIDESC_0_MASK);
    }

    /// Clear PHY_LP_TXVALIDESC_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_lp_txvalidesc_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_LP_TXVALIDESC_0_MASK);
    }

    /// **PHY_LP_TXREQUESTESC_0** `[24]` (RW)
    ///
    /// txrequestesc_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub fn phy_lp_txrequestesc_0(&self) -> bool {
        (self.value.read_bits() & Self::PHY_LP_TXREQUESTESC_0_MASK) != 0
    }

    /// Set PHY_LP_TXREQUESTESC_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_lp_txrequestesc_0(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_LP_TXREQUESTESC_0_MASK);
    }

    /// Clear PHY_LP_TXREQUESTESC_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_lp_txrequestesc_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_LP_TXREQUESTESC_0_MASK);
    }

    /// **PHY_LP_TXDATAESC_0** `[23:16]` (RW)
    ///
    /// txdataesc_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub fn phy_lp_txdataesc_0(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_LP_TXDATAESC_0_MASK) >> Self::PHY_LP_TXDATAESC_0_OFFSET
    }

    /// Write **PHY_LP_TXDATAESC_0** `[23:16]` (RW)
    ///
    /// txdataesc_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub fn write_phy_lp_txdataesc_0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PHY_LP_TXDATAESC_0_MASK)
                | ((value << Self::PHY_LP_TXDATAESC_0_OFFSET) & Self::PHY_LP_TXDATAESC_0_MASK)
        });
    }

    /// **PHY_LP_BASEDIR_1** `[11]` (RW)
    ///
    /// basedir_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub fn phy_lp_basedir_1(&self) -> bool {
        (self.value.read_bits() & Self::PHY_LP_BASEDIR_1_MASK) != 0
    }

    /// Set PHY_LP_BASEDIR_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_lp_basedir_1(&self) {
        self.value.modify_bits(|r| r | Self::PHY_LP_BASEDIR_1_MASK);
    }

    /// Clear PHY_LP_BASEDIR_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_lp_basedir_1(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_LP_BASEDIR_1_MASK);
    }

    /// **PHY_LP_TXLPDTESC_1** `[10]` (RW)
    ///
    /// txlpdtesc_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub fn phy_lp_txlpdtesc_1(&self) -> bool {
        (self.value.read_bits() & Self::PHY_LP_TXLPDTESC_1_MASK) != 0
    }

    /// Set PHY_LP_TXLPDTESC_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_lp_txlpdtesc_1(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_LP_TXLPDTESC_1_MASK);
    }

    /// Clear PHY_LP_TXLPDTESC_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_lp_txlpdtesc_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_LP_TXLPDTESC_1_MASK);
    }

    /// **PHY_LP_TXVALIDESC_1** `[9]` (RW)
    ///
    /// txvalidesc_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub fn phy_lp_txvalidesc_1(&self) -> bool {
        (self.value.read_bits() & Self::PHY_LP_TXVALIDESC_1_MASK) != 0
    }

    /// Set PHY_LP_TXVALIDESC_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_lp_txvalidesc_1(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_LP_TXVALIDESC_1_MASK);
    }

    /// Clear PHY_LP_TXVALIDESC_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_lp_txvalidesc_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_LP_TXVALIDESC_1_MASK);
    }

    /// **PHY_LP_TXREQUESTESC_1** `[8]` (RW)
    ///
    /// txrequestesc_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub fn phy_lp_txrequestesc_1(&self) -> bool {
        (self.value.read_bits() & Self::PHY_LP_TXREQUESTESC_1_MASK) != 0
    }

    /// Set PHY_LP_TXREQUESTESC_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_lp_txrequestesc_1(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_LP_TXREQUESTESC_1_MASK);
    }

    /// Clear PHY_LP_TXREQUESTESC_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_lp_txrequestesc_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_LP_TXREQUESTESC_1_MASK);
    }

    /// **PHY_LP_TXDATAESC_1** `[7:0]` (RW)
    ///
    /// txdataesc_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub fn phy_lp_txdataesc_1(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_LP_TXDATAESC_1_MASK) >> Self::PHY_LP_TXDATAESC_1_OFFSET
    }

    /// Write **PHY_LP_TXDATAESC_1** `[7:0]` (RW)
    ///
    /// txdataesc_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub fn write_phy_lp_txdataesc_1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PHY_LP_TXDATAESC_1_MASK)
                | ((value << Self::PHY_LP_TXDATAESC_1_OFFSET) & Self::PHY_LP_TXDATAESC_1_MASK)
        });
    }
}

/// PHY_LP_LOOPBACK_CTRL value type for batch read/modify/write operations
///
/// Use `phylploopbackctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phylploopbackctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyLpLoopbackCtrlVal {
    bits: u32,
}

impl PhyLpLoopbackCtrlVal {
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

    /// Read **PHY_LP_LOOPBACK_OK** `[31:31]`
    ///
    /// result of dsi phy lp_loopback test
    #[inline(always)]
    pub const fn phy_lp_loopback_ok(&self) -> bool {
        (self.bits & PhyLpLoopbackCtrl::PHY_LP_LOOPBACK_OK_MASK) != 0
    }

    /// Read **PHY_LP_LOOPBACK_EN** `[30:30]`
    ///
    /// dsi phy lp_loopback ctrl en
    #[inline(always)]
    pub const fn phy_lp_loopback_en(&self) -> bool {
        (self.bits & PhyLpLoopbackCtrl::PHY_LP_LOOPBACK_EN_MASK) != 0
    }

    /// Read **PHY_LP_LOOPBACK_CHECK_DONE** `[29:29]`
    ///
    /// dsi phy lp_loopback test check done
    #[inline(always)]
    pub const fn phy_lp_loopback_check_done(&self) -> bool {
        (self.bits & PhyLpLoopbackCtrl::PHY_LP_LOOPBACK_CHECK_DONE_MASK) != 0
    }

    /// Read **PHY_LP_BASEDIR_0** `[27:27]`
    ///
    /// basedir_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn phy_lp_basedir_0(&self) -> bool {
        (self.bits & PhyLpLoopbackCtrl::PHY_LP_BASEDIR_0_MASK) != 0
    }

    /// Read **PHY_LP_TXLPDTESC_0** `[26:26]`
    ///
    /// txlpdtesc_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn phy_lp_txlpdtesc_0(&self) -> bool {
        (self.bits & PhyLpLoopbackCtrl::PHY_LP_TXLPDTESC_0_MASK) != 0
    }

    /// Read **PHY_LP_TXVALIDESC_0** `[25:25]`
    ///
    /// txvalidesc_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn phy_lp_txvalidesc_0(&self) -> bool {
        (self.bits & PhyLpLoopbackCtrl::PHY_LP_TXVALIDESC_0_MASK) != 0
    }

    /// Read **PHY_LP_TXREQUESTESC_0** `[24:24]`
    ///
    /// txrequestesc_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn phy_lp_txrequestesc_0(&self) -> bool {
        (self.bits & PhyLpLoopbackCtrl::PHY_LP_TXREQUESTESC_0_MASK) != 0
    }

    /// Read **PHY_LP_TXDATAESC_0** `[23:16]`
    ///
    /// txdataesc_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn phy_lp_txdataesc_0(&self) -> u32 {
        (self.bits & PhyLpLoopbackCtrl::PHY_LP_TXDATAESC_0_MASK)
            >> PhyLpLoopbackCtrl::PHY_LP_TXDATAESC_0_OFFSET
    }

    /// Read **PHY_LP_BASEDIR_1** `[11:11]`
    ///
    /// basedir_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn phy_lp_basedir_1(&self) -> bool {
        (self.bits & PhyLpLoopbackCtrl::PHY_LP_BASEDIR_1_MASK) != 0
    }

    /// Read **PHY_LP_TXLPDTESC_1** `[10:10]`
    ///
    /// txlpdtesc_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn phy_lp_txlpdtesc_1(&self) -> bool {
        (self.bits & PhyLpLoopbackCtrl::PHY_LP_TXLPDTESC_1_MASK) != 0
    }

    /// Read **PHY_LP_TXVALIDESC_1** `[9:9]`
    ///
    /// txvalidesc_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn phy_lp_txvalidesc_1(&self) -> bool {
        (self.bits & PhyLpLoopbackCtrl::PHY_LP_TXVALIDESC_1_MASK) != 0
    }

    /// Read **PHY_LP_TXREQUESTESC_1** `[8:8]`
    ///
    /// txrequestesc_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn phy_lp_txrequestesc_1(&self) -> bool {
        (self.bits & PhyLpLoopbackCtrl::PHY_LP_TXREQUESTESC_1_MASK) != 0
    }

    /// Read **PHY_LP_TXDATAESC_1** `[7:0]`
    ///
    /// txdataesc_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn phy_lp_txdataesc_1(&self) -> u32 {
        (self.bits & PhyLpLoopbackCtrl::PHY_LP_TXDATAESC_1_MASK)
            >> PhyLpLoopbackCtrl::PHY_LP_TXDATAESC_1_OFFSET
    }

    /// Set **PHY_LP_LOOPBACK_EN** `[30:30]`
    ///
    /// dsi phy lp_loopback ctrl en
    #[inline(always)]
    pub const fn with_phy_lp_loopback_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyLpLoopbackCtrl::PHY_LP_LOOPBACK_EN_MASK
            } else {
                self.bits & !PhyLpLoopbackCtrl::PHY_LP_LOOPBACK_EN_MASK
            },
        }
    }

    /// Set **PHY_LP_LOOPBACK_CHECK** `[28:28]`
    ///
    /// dsi phy lp_loopback test start check
    #[inline(always)]
    pub const fn with_phy_lp_loopback_check(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyLpLoopbackCtrl::PHY_LP_LOOPBACK_CHECK_MASK
            } else {
                self.bits & !PhyLpLoopbackCtrl::PHY_LP_LOOPBACK_CHECK_MASK
            },
        }
    }

    /// Set **PHY_LP_BASEDIR_0** `[27:27]`
    ///
    /// basedir_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn with_phy_lp_basedir_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyLpLoopbackCtrl::PHY_LP_BASEDIR_0_MASK
            } else {
                self.bits & !PhyLpLoopbackCtrl::PHY_LP_BASEDIR_0_MASK
            },
        }
    }

    /// Set **PHY_LP_TXLPDTESC_0** `[26:26]`
    ///
    /// txlpdtesc_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn with_phy_lp_txlpdtesc_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyLpLoopbackCtrl::PHY_LP_TXLPDTESC_0_MASK
            } else {
                self.bits & !PhyLpLoopbackCtrl::PHY_LP_TXLPDTESC_0_MASK
            },
        }
    }

    /// Set **PHY_LP_TXVALIDESC_0** `[25:25]`
    ///
    /// txvalidesc_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn with_phy_lp_txvalidesc_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyLpLoopbackCtrl::PHY_LP_TXVALIDESC_0_MASK
            } else {
                self.bits & !PhyLpLoopbackCtrl::PHY_LP_TXVALIDESC_0_MASK
            },
        }
    }

    /// Set **PHY_LP_TXREQUESTESC_0** `[24:24]`
    ///
    /// txrequestesc_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn with_phy_lp_txrequestesc_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyLpLoopbackCtrl::PHY_LP_TXREQUESTESC_0_MASK
            } else {
                self.bits & !PhyLpLoopbackCtrl::PHY_LP_TXREQUESTESC_0_MASK
            },
        }
    }

    /// Set **PHY_LP_TXDATAESC_0** `[23:16]`
    ///
    /// txdataesc_0 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn with_phy_lp_txdataesc_0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyLpLoopbackCtrl::PHY_LP_TXDATAESC_0_MASK)
                | ((value << PhyLpLoopbackCtrl::PHY_LP_TXDATAESC_0_OFFSET)
                    & PhyLpLoopbackCtrl::PHY_LP_TXDATAESC_0_MASK),
        }
    }

    /// Set **PHY_LP_BASEDIR_1** `[11:11]`
    ///
    /// basedir_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn with_phy_lp_basedir_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyLpLoopbackCtrl::PHY_LP_BASEDIR_1_MASK
            } else {
                self.bits & !PhyLpLoopbackCtrl::PHY_LP_BASEDIR_1_MASK
            },
        }
    }

    /// Set **PHY_LP_TXLPDTESC_1** `[10:10]`
    ///
    /// txlpdtesc_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn with_phy_lp_txlpdtesc_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyLpLoopbackCtrl::PHY_LP_TXLPDTESC_1_MASK
            } else {
                self.bits & !PhyLpLoopbackCtrl::PHY_LP_TXLPDTESC_1_MASK
            },
        }
    }

    /// Set **PHY_LP_TXVALIDESC_1** `[9:9]`
    ///
    /// txvalidesc_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn with_phy_lp_txvalidesc_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyLpLoopbackCtrl::PHY_LP_TXVALIDESC_1_MASK
            } else {
                self.bits & !PhyLpLoopbackCtrl::PHY_LP_TXVALIDESC_1_MASK
            },
        }
    }

    /// Set **PHY_LP_TXREQUESTESC_1** `[8:8]`
    ///
    /// txrequestesc_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn with_phy_lp_txrequestesc_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyLpLoopbackCtrl::PHY_LP_TXREQUESTESC_1_MASK
            } else {
                self.bits & !PhyLpLoopbackCtrl::PHY_LP_TXREQUESTESC_1_MASK
            },
        }
    }

    /// Set **PHY_LP_TXDATAESC_1** `[7:0]`
    ///
    /// txdataesc_1 ctrl when enable dsi phy lp_loopback_test
    #[inline(always)]
    pub const fn with_phy_lp_txdataesc_1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyLpLoopbackCtrl::PHY_LP_TXDATAESC_1_MASK)
                | ((value << PhyLpLoopbackCtrl::PHY_LP_TXDATAESC_1_OFFSET)
                    & PhyLpLoopbackCtrl::PHY_LP_TXDATAESC_1_MASK),
        }
    }
}

impl Default for PhyLpLoopbackCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PhyLpLoopbackCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyLpLoopbackCtrlVal {
        PhyLpLoopbackCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyLpLoopbackCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyLpLoopbackCtrlVal) -> PhyLpLoopbackCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_HS_LOOPBACK_CTRL Register
///
/// dsi phy hp_loopback test ctrl
///
/// Address offset: 0x0098
/// Reset value: 0x00000200
///
/// ## Fields
///
/// - **PHY_HS_LOOPBACK_OK** `[31]` (RO)
///
///   result of dsi phy hs_loopback test
///
/// - **PHY_HS_LOOPBACK_EN** `[30]` (RW)
///
///   dsi phy hs_loopback ctrl en
///
/// - **PHY_HS_LOOPBACK_CHECK_DONE** `[29]` (RO)
///
///   dsi phy hs_loopback test check done
///
/// - **PHY_HS_LOOPBACK_CHECK** `[28]` (WO)
///
///   dsi phy hs_loopback test start check
///
/// - **PHY_HS_TXREQUESTHSCLK** `[27]` (RW)
///
///   txrequesthsclk when enable dsi phy hs_loopback_test
///
/// - **PHY_HS_BASEDIR_0** `[25]` (RW)
///
///   basedir_0 ctrl when enable dsi phy hs_loopback_test
///
/// - **PHY_HS_TXREQUESTDATAHS_0** `[24]` (RW)
///
///   txrequestdatahs_0 ctrl when enable dsi phy hs_loopback_test
///
/// - **PHY_HS_TXDATAHS_0** `[23:16]` (RW)
///
///   txdatahs_0 ctrl when enable dsi phy hs_loopback_test
///
/// - **PHY_HS_BASEDIR_1** `[9]` (RW)
///
///   basedir_1 ctrl when enable dsi phy hs_loopback_test
///
/// - **PHY_HS_TXREQUESTDATAHS_1** `[8]` (RW)
///
///   txrequestdatahs_1 ctrl when enable dsi phy hs_loopback_test
///
/// - **PHY_HS_TXDATAHS_1** `[7:0]` (RW)
///
///   txdatahs_1 ctrl when enable dsi phy hs_loopback_test
#[repr(transparent)]
pub struct PhyHsLoopbackCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyHsLoopbackCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyHsLoopbackCtrl {
    // ======== Field Constants ========

    /// **PHY_HS_LOOPBACK_OK**
    ///
    /// result of dsi phy hs_loopback test
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const PHY_HS_LOOPBACK_OK_OFFSET: u32 = 31;
    pub const PHY_HS_LOOPBACK_OK_MASK: u32 = 0x80000000;
    pub const PHY_HS_LOOPBACK_OK: u32 = 1 << 31;

    /// **PHY_HS_LOOPBACK_EN**
    ///
    /// dsi phy hs_loopback ctrl en
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const PHY_HS_LOOPBACK_EN_OFFSET: u32 = 30;
    pub const PHY_HS_LOOPBACK_EN_MASK: u32 = 0x40000000;
    pub const PHY_HS_LOOPBACK_EN: u32 = 1 << 30;

    /// **PHY_HS_LOOPBACK_CHECK_DONE**
    ///
    /// dsi phy hs_loopback test check done
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const PHY_HS_LOOPBACK_CHECK_DONE_OFFSET: u32 = 29;
    pub const PHY_HS_LOOPBACK_CHECK_DONE_MASK: u32 = 0x20000000;
    pub const PHY_HS_LOOPBACK_CHECK_DONE: u32 = 1 << 29;

    /// **PHY_HS_LOOPBACK_CHECK**
    ///
    /// dsi phy hs_loopback test start check
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const PHY_HS_LOOPBACK_CHECK_OFFSET: u32 = 28;
    pub const PHY_HS_LOOPBACK_CHECK_MASK: u32 = 0x10000000;
    pub const PHY_HS_LOOPBACK_CHECK: u32 = 1 << 28;

    /// **PHY_HS_TXREQUESTHSCLK**
    ///
    /// txrequesthsclk when enable dsi phy hs_loopback_test
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const PHY_HS_TXREQUESTHSCLK_OFFSET: u32 = 27;
    pub const PHY_HS_TXREQUESTHSCLK_MASK: u32 = 0x08000000;
    pub const PHY_HS_TXREQUESTHSCLK: u32 = 1 << 27;

    /// **PHY_HS_BASEDIR_0**
    ///
    /// basedir_0 ctrl when enable dsi phy hs_loopback_test
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const PHY_HS_BASEDIR_0_OFFSET: u32 = 25;
    pub const PHY_HS_BASEDIR_0_MASK: u32 = 0x02000000;
    pub const PHY_HS_BASEDIR_0: u32 = 1 << 25;

    /// **PHY_HS_TXREQUESTDATAHS_0**
    ///
    /// txrequestdatahs_0 ctrl when enable dsi phy hs_loopback_test
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const PHY_HS_TXREQUESTDATAHS_0_OFFSET: u32 = 24;
    pub const PHY_HS_TXREQUESTDATAHS_0_MASK: u32 = 0x01000000;
    pub const PHY_HS_TXREQUESTDATAHS_0: u32 = 1 << 24;

    /// **PHY_HS_TXDATAHS_0**
    ///
    /// txdatahs_0 ctrl when enable dsi phy hs_loopback_test
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const PHY_HS_TXDATAHS_0_OFFSET: u32 = 16;
    pub const PHY_HS_TXDATAHS_0_MASK: u32 = 0x00ff0000;

    /// **PHY_HS_BASEDIR_1**
    ///
    /// basedir_1 ctrl when enable dsi phy hs_loopback_test
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const PHY_HS_BASEDIR_1_OFFSET: u32 = 9;
    pub const PHY_HS_BASEDIR_1_MASK: u32 = 0x00000200;
    pub const PHY_HS_BASEDIR_1: u32 = 1 << 9;

    /// **PHY_HS_TXREQUESTDATAHS_1**
    ///
    /// txrequestdatahs_1 ctrl when enable dsi phy hs_loopback_test
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const PHY_HS_TXREQUESTDATAHS_1_OFFSET: u32 = 8;
    pub const PHY_HS_TXREQUESTDATAHS_1_MASK: u32 = 0x00000100;
    pub const PHY_HS_TXREQUESTDATAHS_1: u32 = 1 << 8;

    /// **PHY_HS_TXDATAHS_1**
    ///
    /// txdatahs_1 ctrl when enable dsi phy hs_loopback_test
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const PHY_HS_TXDATAHS_1_OFFSET: u32 = 0;
    pub const PHY_HS_TXDATAHS_1_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **PHY_HS_LOOPBACK_OK** `[31]` (RO)
    ///
    /// result of dsi phy hs_loopback test
    #[inline(always)]
    pub fn phy_hs_loopback_ok(&self) -> bool {
        (self.value.read_bits() & Self::PHY_HS_LOOPBACK_OK_MASK) != 0
    }

    /// **PHY_HS_LOOPBACK_EN** `[30]` (RW)
    ///
    /// dsi phy hs_loopback ctrl en
    #[inline(always)]
    pub fn phy_hs_loopback_en(&self) -> bool {
        (self.value.read_bits() & Self::PHY_HS_LOOPBACK_EN_MASK) != 0
    }

    /// Set PHY_HS_LOOPBACK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_hs_loopback_en(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_HS_LOOPBACK_EN_MASK);
    }

    /// Clear PHY_HS_LOOPBACK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_hs_loopback_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_HS_LOOPBACK_EN_MASK);
    }

    /// **PHY_HS_LOOPBACK_CHECK_DONE** `[29]` (RO)
    ///
    /// dsi phy hs_loopback test check done
    #[inline(always)]
    pub fn phy_hs_loopback_check_done(&self) -> bool {
        (self.value.read_bits() & Self::PHY_HS_LOOPBACK_CHECK_DONE_MASK) != 0
    }

    /// Trigger **PHY_HS_LOOPBACK_CHECK** (WO)
    ///
    /// dsi phy hs_loopback test start check
    #[inline(always)]
    pub fn trigger_phy_hs_loopback_check(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_HS_LOOPBACK_CHECK_MASK);
    }

    /// **PHY_HS_TXREQUESTHSCLK** `[27]` (RW)
    ///
    /// txrequesthsclk when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub fn phy_hs_txrequesthsclk(&self) -> bool {
        (self.value.read_bits() & Self::PHY_HS_TXREQUESTHSCLK_MASK) != 0
    }

    /// Set PHY_HS_TXREQUESTHSCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_hs_txrequesthsclk(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_HS_TXREQUESTHSCLK_MASK);
    }

    /// Clear PHY_HS_TXREQUESTHSCLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_hs_txrequesthsclk(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_HS_TXREQUESTHSCLK_MASK);
    }

    /// **PHY_HS_BASEDIR_0** `[25]` (RW)
    ///
    /// basedir_0 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub fn phy_hs_basedir_0(&self) -> bool {
        (self.value.read_bits() & Self::PHY_HS_BASEDIR_0_MASK) != 0
    }

    /// Set PHY_HS_BASEDIR_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_hs_basedir_0(&self) {
        self.value.modify_bits(|r| r | Self::PHY_HS_BASEDIR_0_MASK);
    }

    /// Clear PHY_HS_BASEDIR_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_hs_basedir_0(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_HS_BASEDIR_0_MASK);
    }

    /// **PHY_HS_TXREQUESTDATAHS_0** `[24]` (RW)
    ///
    /// txrequestdatahs_0 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub fn phy_hs_txrequestdatahs_0(&self) -> bool {
        (self.value.read_bits() & Self::PHY_HS_TXREQUESTDATAHS_0_MASK) != 0
    }

    /// Set PHY_HS_TXREQUESTDATAHS_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_hs_txrequestdatahs_0(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_HS_TXREQUESTDATAHS_0_MASK);
    }

    /// Clear PHY_HS_TXREQUESTDATAHS_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_hs_txrequestdatahs_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_HS_TXREQUESTDATAHS_0_MASK);
    }

    /// **PHY_HS_TXDATAHS_0** `[23:16]` (RW)
    ///
    /// txdatahs_0 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub fn phy_hs_txdatahs_0(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_HS_TXDATAHS_0_MASK) >> Self::PHY_HS_TXDATAHS_0_OFFSET
    }

    /// Write **PHY_HS_TXDATAHS_0** `[23:16]` (RW)
    ///
    /// txdatahs_0 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub fn write_phy_hs_txdatahs_0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PHY_HS_TXDATAHS_0_MASK)
                | ((value << Self::PHY_HS_TXDATAHS_0_OFFSET) & Self::PHY_HS_TXDATAHS_0_MASK)
        });
    }

    /// **PHY_HS_BASEDIR_1** `[9]` (RW)
    ///
    /// basedir_1 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub fn phy_hs_basedir_1(&self) -> bool {
        (self.value.read_bits() & Self::PHY_HS_BASEDIR_1_MASK) != 0
    }

    /// Set PHY_HS_BASEDIR_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_hs_basedir_1(&self) {
        self.value.modify_bits(|r| r | Self::PHY_HS_BASEDIR_1_MASK);
    }

    /// Clear PHY_HS_BASEDIR_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_hs_basedir_1(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_HS_BASEDIR_1_MASK);
    }

    /// **PHY_HS_TXREQUESTDATAHS_1** `[8]` (RW)
    ///
    /// txrequestdatahs_1 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub fn phy_hs_txrequestdatahs_1(&self) -> bool {
        (self.value.read_bits() & Self::PHY_HS_TXREQUESTDATAHS_1_MASK) != 0
    }

    /// Set PHY_HS_TXREQUESTDATAHS_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_hs_txrequestdatahs_1(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_HS_TXREQUESTDATAHS_1_MASK);
    }

    /// Clear PHY_HS_TXREQUESTDATAHS_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_hs_txrequestdatahs_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_HS_TXREQUESTDATAHS_1_MASK);
    }

    /// **PHY_HS_TXDATAHS_1** `[7:0]` (RW)
    ///
    /// txdatahs_1 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub fn phy_hs_txdatahs_1(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_HS_TXDATAHS_1_MASK) >> Self::PHY_HS_TXDATAHS_1_OFFSET
    }

    /// Write **PHY_HS_TXDATAHS_1** `[7:0]` (RW)
    ///
    /// txdatahs_1 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub fn write_phy_hs_txdatahs_1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PHY_HS_TXDATAHS_1_MASK)
                | ((value << Self::PHY_HS_TXDATAHS_1_OFFSET) & Self::PHY_HS_TXDATAHS_1_MASK)
        });
    }
}

/// PHY_HS_LOOPBACK_CTRL value type for batch read/modify/write operations
///
/// Use `phyhsloopbackctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phyhsloopbackctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyHsLoopbackCtrlVal {
    bits: u32,
}

impl PhyHsLoopbackCtrlVal {
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
        Self { bits: 0x00000200 }
    }

    /// Read **PHY_HS_LOOPBACK_OK** `[31:31]`
    ///
    /// result of dsi phy hs_loopback test
    #[inline(always)]
    pub const fn phy_hs_loopback_ok(&self) -> bool {
        (self.bits & PhyHsLoopbackCtrl::PHY_HS_LOOPBACK_OK_MASK) != 0
    }

    /// Read **PHY_HS_LOOPBACK_EN** `[30:30]`
    ///
    /// dsi phy hs_loopback ctrl en
    #[inline(always)]
    pub const fn phy_hs_loopback_en(&self) -> bool {
        (self.bits & PhyHsLoopbackCtrl::PHY_HS_LOOPBACK_EN_MASK) != 0
    }

    /// Read **PHY_HS_LOOPBACK_CHECK_DONE** `[29:29]`
    ///
    /// dsi phy hs_loopback test check done
    #[inline(always)]
    pub const fn phy_hs_loopback_check_done(&self) -> bool {
        (self.bits & PhyHsLoopbackCtrl::PHY_HS_LOOPBACK_CHECK_DONE_MASK) != 0
    }

    /// Read **PHY_HS_TXREQUESTHSCLK** `[27:27]`
    ///
    /// txrequesthsclk when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub const fn phy_hs_txrequesthsclk(&self) -> bool {
        (self.bits & PhyHsLoopbackCtrl::PHY_HS_TXREQUESTHSCLK_MASK) != 0
    }

    /// Read **PHY_HS_BASEDIR_0** `[25:25]`
    ///
    /// basedir_0 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub const fn phy_hs_basedir_0(&self) -> bool {
        (self.bits & PhyHsLoopbackCtrl::PHY_HS_BASEDIR_0_MASK) != 0
    }

    /// Read **PHY_HS_TXREQUESTDATAHS_0** `[24:24]`
    ///
    /// txrequestdatahs_0 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub const fn phy_hs_txrequestdatahs_0(&self) -> bool {
        (self.bits & PhyHsLoopbackCtrl::PHY_HS_TXREQUESTDATAHS_0_MASK) != 0
    }

    /// Read **PHY_HS_TXDATAHS_0** `[23:16]`
    ///
    /// txdatahs_0 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub const fn phy_hs_txdatahs_0(&self) -> u32 {
        (self.bits & PhyHsLoopbackCtrl::PHY_HS_TXDATAHS_0_MASK)
            >> PhyHsLoopbackCtrl::PHY_HS_TXDATAHS_0_OFFSET
    }

    /// Read **PHY_HS_BASEDIR_1** `[9:9]`
    ///
    /// basedir_1 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub const fn phy_hs_basedir_1(&self) -> bool {
        (self.bits & PhyHsLoopbackCtrl::PHY_HS_BASEDIR_1_MASK) != 0
    }

    /// Read **PHY_HS_TXREQUESTDATAHS_1** `[8:8]`
    ///
    /// txrequestdatahs_1 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub const fn phy_hs_txrequestdatahs_1(&self) -> bool {
        (self.bits & PhyHsLoopbackCtrl::PHY_HS_TXREQUESTDATAHS_1_MASK) != 0
    }

    /// Read **PHY_HS_TXDATAHS_1** `[7:0]`
    ///
    /// txdatahs_1 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub const fn phy_hs_txdatahs_1(&self) -> u32 {
        (self.bits & PhyHsLoopbackCtrl::PHY_HS_TXDATAHS_1_MASK)
            >> PhyHsLoopbackCtrl::PHY_HS_TXDATAHS_1_OFFSET
    }

    /// Set **PHY_HS_LOOPBACK_EN** `[30:30]`
    ///
    /// dsi phy hs_loopback ctrl en
    #[inline(always)]
    pub const fn with_phy_hs_loopback_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyHsLoopbackCtrl::PHY_HS_LOOPBACK_EN_MASK
            } else {
                self.bits & !PhyHsLoopbackCtrl::PHY_HS_LOOPBACK_EN_MASK
            },
        }
    }

    /// Set **PHY_HS_LOOPBACK_CHECK** `[28:28]`
    ///
    /// dsi phy hs_loopback test start check
    #[inline(always)]
    pub const fn with_phy_hs_loopback_check(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyHsLoopbackCtrl::PHY_HS_LOOPBACK_CHECK_MASK
            } else {
                self.bits & !PhyHsLoopbackCtrl::PHY_HS_LOOPBACK_CHECK_MASK
            },
        }
    }

    /// Set **PHY_HS_TXREQUESTHSCLK** `[27:27]`
    ///
    /// txrequesthsclk when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub const fn with_phy_hs_txrequesthsclk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyHsLoopbackCtrl::PHY_HS_TXREQUESTHSCLK_MASK
            } else {
                self.bits & !PhyHsLoopbackCtrl::PHY_HS_TXREQUESTHSCLK_MASK
            },
        }
    }

    /// Set **PHY_HS_BASEDIR_0** `[25:25]`
    ///
    /// basedir_0 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub const fn with_phy_hs_basedir_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyHsLoopbackCtrl::PHY_HS_BASEDIR_0_MASK
            } else {
                self.bits & !PhyHsLoopbackCtrl::PHY_HS_BASEDIR_0_MASK
            },
        }
    }

    /// Set **PHY_HS_TXREQUESTDATAHS_0** `[24:24]`
    ///
    /// txrequestdatahs_0 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub const fn with_phy_hs_txrequestdatahs_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyHsLoopbackCtrl::PHY_HS_TXREQUESTDATAHS_0_MASK
            } else {
                self.bits & !PhyHsLoopbackCtrl::PHY_HS_TXREQUESTDATAHS_0_MASK
            },
        }
    }

    /// Set **PHY_HS_TXDATAHS_0** `[23:16]`
    ///
    /// txdatahs_0 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub const fn with_phy_hs_txdatahs_0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyHsLoopbackCtrl::PHY_HS_TXDATAHS_0_MASK)
                | ((value << PhyHsLoopbackCtrl::PHY_HS_TXDATAHS_0_OFFSET)
                    & PhyHsLoopbackCtrl::PHY_HS_TXDATAHS_0_MASK),
        }
    }

    /// Set **PHY_HS_BASEDIR_1** `[9:9]`
    ///
    /// basedir_1 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub const fn with_phy_hs_basedir_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyHsLoopbackCtrl::PHY_HS_BASEDIR_1_MASK
            } else {
                self.bits & !PhyHsLoopbackCtrl::PHY_HS_BASEDIR_1_MASK
            },
        }
    }

    /// Set **PHY_HS_TXREQUESTDATAHS_1** `[8:8]`
    ///
    /// txrequestdatahs_1 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub const fn with_phy_hs_txrequestdatahs_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PhyHsLoopbackCtrl::PHY_HS_TXREQUESTDATAHS_1_MASK
            } else {
                self.bits & !PhyHsLoopbackCtrl::PHY_HS_TXREQUESTDATAHS_1_MASK
            },
        }
    }

    /// Set **PHY_HS_TXDATAHS_1** `[7:0]`
    ///
    /// txdatahs_1 ctrl when enable dsi phy hs_loopback_test
    #[inline(always)]
    pub const fn with_phy_hs_txdatahs_1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyHsLoopbackCtrl::PHY_HS_TXDATAHS_1_MASK)
                | ((value << PhyHsLoopbackCtrl::PHY_HS_TXDATAHS_1_OFFSET)
                    & PhyHsLoopbackCtrl::PHY_HS_TXDATAHS_1_MASK),
        }
    }
}

impl Default for PhyHsLoopbackCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PhyHsLoopbackCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyHsLoopbackCtrlVal {
        PhyHsLoopbackCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyHsLoopbackCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyHsLoopbackCtrlVal) -> PhyHsLoopbackCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PHY_LOOPBACK_CNT Register
///
/// loopback test cnt
///
/// Address offset: 0x009c
/// Reset value: 0x00400040
///
/// ## Fields
///
/// - **PHY_LP_CHECK_CNT_TH** `[23:16]` (RW)
///
///   lp_loopback test check cnt
///
/// - **PHY_HS_CHECK_CNT_TH** `[7:0]` (RW)
///
///   hs_loopback test check cnt
#[repr(transparent)]
pub struct PhyLoopbackCnt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PhyLoopbackCnt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PhyLoopbackCnt {
    // ======== Field Constants ========

    /// **PHY_LP_CHECK_CNT_TH**
    ///
    /// lp_loopback test check cnt
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const PHY_LP_CHECK_CNT_TH_OFFSET: u32 = 16;
    pub const PHY_LP_CHECK_CNT_TH_MASK: u32 = 0x00ff0000;

    /// **PHY_HS_CHECK_CNT_TH**
    ///
    /// hs_loopback test check cnt
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const PHY_HS_CHECK_CNT_TH_OFFSET: u32 = 0;
    pub const PHY_HS_CHECK_CNT_TH_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **PHY_LP_CHECK_CNT_TH** `[23:16]` (RW)
    ///
    /// lp_loopback test check cnt
    #[inline(always)]
    pub fn phy_lp_check_cnt_th(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_LP_CHECK_CNT_TH_MASK)
            >> Self::PHY_LP_CHECK_CNT_TH_OFFSET
    }

    /// Write **PHY_LP_CHECK_CNT_TH** `[23:16]` (RW)
    ///
    /// lp_loopback test check cnt
    #[inline(always)]
    pub fn write_phy_lp_check_cnt_th(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PHY_LP_CHECK_CNT_TH_MASK)
                | ((value << Self::PHY_LP_CHECK_CNT_TH_OFFSET) & Self::PHY_LP_CHECK_CNT_TH_MASK)
        });
    }

    /// **PHY_HS_CHECK_CNT_TH** `[7:0]` (RW)
    ///
    /// hs_loopback test check cnt
    #[inline(always)]
    pub fn phy_hs_check_cnt_th(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_HS_CHECK_CNT_TH_MASK)
            >> Self::PHY_HS_CHECK_CNT_TH_OFFSET
    }

    /// Write **PHY_HS_CHECK_CNT_TH** `[7:0]` (RW)
    ///
    /// hs_loopback test check cnt
    #[inline(always)]
    pub fn write_phy_hs_check_cnt_th(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PHY_HS_CHECK_CNT_TH_MASK)
                | ((value << Self::PHY_HS_CHECK_CNT_TH_OFFSET) & Self::PHY_HS_CHECK_CNT_TH_MASK)
        });
    }
}

/// PHY_LOOPBACK_CNT value type for batch read/modify/write operations
///
/// Use `phyloopbackcnt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `phyloopbackcnt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PhyLoopbackCntVal {
    bits: u32,
}

impl PhyLoopbackCntVal {
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
        Self { bits: 0x00400040 }
    }

    /// Read **PHY_LP_CHECK_CNT_TH** `[23:16]`
    ///
    /// lp_loopback test check cnt
    #[inline(always)]
    pub const fn phy_lp_check_cnt_th(&self) -> u32 {
        (self.bits & PhyLoopbackCnt::PHY_LP_CHECK_CNT_TH_MASK)
            >> PhyLoopbackCnt::PHY_LP_CHECK_CNT_TH_OFFSET
    }

    /// Read **PHY_HS_CHECK_CNT_TH** `[7:0]`
    ///
    /// hs_loopback test check cnt
    #[inline(always)]
    pub const fn phy_hs_check_cnt_th(&self) -> u32 {
        (self.bits & PhyLoopbackCnt::PHY_HS_CHECK_CNT_TH_MASK)
            >> PhyLoopbackCnt::PHY_HS_CHECK_CNT_TH_OFFSET
    }

    /// Set **PHY_LP_CHECK_CNT_TH** `[23:16]`
    ///
    /// lp_loopback test check cnt
    #[inline(always)]
    pub const fn with_phy_lp_check_cnt_th(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyLoopbackCnt::PHY_LP_CHECK_CNT_TH_MASK)
                | ((value << PhyLoopbackCnt::PHY_LP_CHECK_CNT_TH_OFFSET)
                    & PhyLoopbackCnt::PHY_LP_CHECK_CNT_TH_MASK),
        }
    }

    /// Set **PHY_HS_CHECK_CNT_TH** `[7:0]`
    ///
    /// hs_loopback test check cnt
    #[inline(always)]
    pub const fn with_phy_hs_check_cnt_th(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PhyLoopbackCnt::PHY_HS_CHECK_CNT_TH_MASK)
                | ((value << PhyLoopbackCnt::PHY_HS_CHECK_CNT_TH_OFFSET)
                    & PhyLoopbackCnt::PHY_HS_CHECK_CNT_TH_MASK),
        }
    }
}

impl Default for PhyLoopbackCntVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PhyLoopbackCnt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PhyLoopbackCntVal {
        PhyLoopbackCntVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PhyLoopbackCntVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PhyLoopbackCntVal) -> PhyLoopbackCntVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// dsi bridge clk control register
    pub clk_en: ClkEn,
    /// 0x0004
    /// dsi bridge en register
    pub en: En,
    /// 0x0008
    /// dsi bridge dma burst len register
    pub dma_req_cfg: DmaReqCfg,
    /// 0x000c
    /// dsi bridge raw number control register
    pub raw_num_cfg: RawNumCfg,
    /// 0x0010
    /// dsi bridge credit register
    pub raw_buf_credit_ctl: RawBufCreditCtl,
    /// 0x0014
    /// dsi bridge raw buffer depth register
    pub fifo_flow_status: FifoFlowStatus,
    /// 0x0018
    /// dsi bridge dpi type control register
    pub pixel_type: PixelType,
    /// 0x001c
    /// dsi bridge dma block interval control register
    pub dma_block_interval: DmaBlockInterval,
    /// 0x0020
    /// dsi bridge dma req interval control register
    pub dma_req_interval: DmaReqInterval,
    /// 0x0024
    /// dsi bridge dpi signal control register
    pub dpi_lcd_ctl: DpiLcdCtl,
    /// 0x0028
    /// dsi bridge dpi reserved data register
    pub dpi_rsv_dpi_data: DpiRsvDpiData,
    _reserved0: [u8; 4],
    /// 0x0030
    /// dsi bridge dpi v config register 0
    pub dpi_v_cfg0: DpiVCfg0,
    /// 0x0034
    /// dsi bridge dpi v config register 1
    pub dpi_v_cfg1: DpiVCfg1,
    /// 0x0038
    /// dsi bridge dpi h config register 0
    pub dpi_h_cfg0: DpiHCfg0,
    /// 0x003c
    /// dsi bridge dpi h config register 1
    pub dpi_h_cfg1: DpiHCfg1,
    /// 0x0040
    /// dsi_bridge dpi misc config register
    pub dpi_misc_config: DpiMiscConfig,
    /// 0x0044
    /// dsi_bridge dpi config update register
    pub dpi_config_update: DpiConfigUpdate,
    _reserved1: [u8; 8],
    /// 0x0050
    /// dsi_bridge interrupt enable register
    pub int_ena: IntEna,
    /// 0x0054
    /// dsi_bridge interrupt clear register
    pub int_clr: IntClr,
    /// 0x0058
    /// dsi_bridge raw interrupt register
    pub int_raw: IntRaw,
    /// 0x005c
    /// dsi_bridge masked interrupt register
    pub int_st: IntSt,
    /// 0x0060
    /// dsi_bridge host bist control register
    pub host_bist_ctl: HostBistCtl,
    /// 0x0064
    /// dsi_bridge host trigger reverse control register
    pub host_trigger_rev: HostTriggerRev,
    /// 0x0068
    /// dsi_bridge block raw number control register
    pub blk_raw_num_cfg: BlkRawNumCfg,
    /// 0x006c
    /// dsi_bridge dam frame interval control register
    pub dma_frame_interval: DmaFrameInterval,
    /// 0x0070
    /// dsi_bridge mem aux control register
    pub mem_aux_ctrl: MemAuxCtrl,
    /// 0x0074
    /// dsi_bridge rdn eco cs register
    pub rdn_eco_cs: RdnEcoCs,
    /// 0x0078
    /// dsi_bridge rdn eco all low register
    pub rdn_eco_low: RdnEcoLow,
    /// 0x007c
    /// dsi_bridge rdn eco all high register
    pub rdn_eco_high: RdnEcoHigh,
    /// 0x0080
    /// dsi_bridge host control register
    pub host_ctrl: HostCtrl,
    /// 0x0084
    /// dsi_bridge mem force on control register
    pub mem_clk_ctrl: MemClkCtrl,
    /// 0x0088
    /// dsi_bridge dma flow controller register
    pub dma_flow_ctrl: DmaFlowCtrl,
    /// 0x008c
    /// dsi_bridge buffer empty threshold register
    pub raw_buf_almost_empty_thrd: RawBufAlmostEmptyThrd,
    /// 0x0090
    /// dsi_bridge yuv format config register
    pub yuv_cfg: YuvCfg,
    /// 0x0094
    /// dsi phy lp_loopback test ctrl
    pub phy_lp_loopback_ctrl: PhyLpLoopbackCtrl,
    /// 0x0098
    /// dsi phy hp_loopback test ctrl
    pub phy_hs_loopback_ctrl: PhyHsLoopbackCtrl,
    /// 0x009c
    /// loopback test cnt
    pub phy_loopback_cnt: PhyLoopbackCnt,
}
