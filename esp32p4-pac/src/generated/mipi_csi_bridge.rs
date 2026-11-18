//! MIPI_CSI_BRIDGE
//! MIPI Camera Interface Bridge

#![allow(dead_code)]

pub const BASE: u32 = 0x5009f800;

/// CLK_EN Register
///
/// csi bridge register mapping unit clock gating.
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **CLK_EN** `[0]` (RW)
///
///   0: enable clock gating. 1: disable clock gating, clock always on.
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
    /// 0: enable clock gating. 1: disable clock gating, clock always on.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_EN_OFFSET: u32 = 0;
    pub const CLK_EN_MASK: u32 = 0x00000001;
    pub const CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[0]` (RW)
    ///
    /// 0: enable clock gating. 1: disable clock gating, clock always on.
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
    /// 0: enable clock gating. 1: disable clock gating, clock always on.
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & ClkEn::CLK_EN_MASK) != 0
    }

    /// Set **CLK_EN** `[0:0]`
    ///
    /// 0: enable clock gating. 1: disable clock gating, clock always on.
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

/// CSI_EN Register
///
/// csi bridge enable.
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **CSI_BRIG_EN** `[0]` (RW)
///
///   0: disable csi bridge. 1: enable csi bridge.
#[repr(transparent)]
pub struct CsiEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CsiEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CsiEn {
    // ======== Field Constants ========

    /// **CSI_BRIG_EN**
    ///
    /// 0: disable csi bridge. 1: enable csi bridge.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CSI_BRIG_EN_OFFSET: u32 = 0;
    pub const CSI_BRIG_EN_MASK: u32 = 0x00000001;
    pub const CSI_BRIG_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CSI_BRIG_EN** `[0]` (RW)
    ///
    /// 0: disable csi bridge. 1: enable csi bridge.
    #[inline(always)]
    pub fn csi_brig_en(&self) -> bool {
        (self.value.read_bits() & Self::CSI_BRIG_EN_MASK) != 0
    }

    /// Set CSI_BRIG_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_csi_brig_en(&self) {
        self.value.modify_bits(|r| r | Self::CSI_BRIG_EN_MASK);
    }

    /// Clear CSI_BRIG_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_csi_brig_en(&self) {
        self.value.modify_bits(|r| r & !Self::CSI_BRIG_EN_MASK);
    }
}

/// CSI_EN value type for batch read/modify/write operations
///
/// Use `csien.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `csien.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CsiEnVal {
    bits: u32,
}

impl CsiEnVal {
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

    /// Read **CSI_BRIG_EN** `[0:0]`
    ///
    /// 0: disable csi bridge. 1: enable csi bridge.
    #[inline(always)]
    pub const fn csi_brig_en(&self) -> bool {
        (self.bits & CsiEn::CSI_BRIG_EN_MASK) != 0
    }

    /// Set **CSI_BRIG_EN** `[0:0]`
    ///
    /// 0: disable csi bridge. 1: enable csi bridge.
    #[inline(always)]
    pub const fn with_csi_brig_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CsiEn::CSI_BRIG_EN_MASK
            } else {
                self.bits & !CsiEn::CSI_BRIG_EN_MASK
            },
        }
    }
}

impl Default for CsiEnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CsiEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CsiEnVal {
        CsiEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CsiEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CsiEnVal) -> CsiEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA_REQ_CFG Register
///
/// dma request configuration.
///
/// Address offset: 0x0008
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **DMA_FORCE_RD_STATUS** `[16]` (RW)
///
///   1: mask dma request when reading frame info. 0: disable mask.
///
/// - **DMA_CFG_UPD_BY_BLK** `[12]` (RW)
///
///   1: reg_dma_burst_len & reg_dma_burst_len will be updated by dma block finish. 0: updated by frame.
///
/// - **DMA_BURST_LEN** `[11:0]` (RW)
///
///   DMA burst length.
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

    /// **DMA_FORCE_RD_STATUS**
    ///
    /// 1: mask dma request when reading frame info. 0: disable mask.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const DMA_FORCE_RD_STATUS_OFFSET: u32 = 16;
    pub const DMA_FORCE_RD_STATUS_MASK: u32 = 0x00010000;
    pub const DMA_FORCE_RD_STATUS: u32 = 1 << 16;

    /// **DMA_CFG_UPD_BY_BLK**
    ///
    /// 1: reg_dma_burst_len & reg_dma_burst_len will be updated by dma block finish. 0: updated by frame.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const DMA_CFG_UPD_BY_BLK_OFFSET: u32 = 12;
    pub const DMA_CFG_UPD_BY_BLK_MASK: u32 = 0x00001000;
    pub const DMA_CFG_UPD_BY_BLK: u32 = 1 << 12;

    /// **DMA_BURST_LEN**
    ///
    /// DMA burst length.
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const DMA_BURST_LEN_OFFSET: u32 = 0;
    pub const DMA_BURST_LEN_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **DMA_FORCE_RD_STATUS** `[16]` (RW)
    ///
    /// 1: mask dma request when reading frame info. 0: disable mask.
    #[inline(always)]
    pub fn dma_force_rd_status(&self) -> bool {
        (self.value.read_bits() & Self::DMA_FORCE_RD_STATUS_MASK) != 0
    }

    /// Set DMA_FORCE_RD_STATUS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma_force_rd_status(&self) {
        self.value
            .modify_bits(|r| r | Self::DMA_FORCE_RD_STATUS_MASK);
    }

    /// Clear DMA_FORCE_RD_STATUS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma_force_rd_status(&self) {
        self.value
            .modify_bits(|r| r & !Self::DMA_FORCE_RD_STATUS_MASK);
    }

    /// **DMA_CFG_UPD_BY_BLK** `[12]` (RW)
    ///
    /// 1: reg_dma_burst_len & reg_dma_burst_len will be updated by dma block finish. 0: updated by frame.
    #[inline(always)]
    pub fn dma_cfg_upd_by_blk(&self) -> bool {
        (self.value.read_bits() & Self::DMA_CFG_UPD_BY_BLK_MASK) != 0
    }

    /// Set DMA_CFG_UPD_BY_BLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma_cfg_upd_by_blk(&self) {
        self.value
            .modify_bits(|r| r | Self::DMA_CFG_UPD_BY_BLK_MASK);
    }

    /// Clear DMA_CFG_UPD_BY_BLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma_cfg_upd_by_blk(&self) {
        self.value
            .modify_bits(|r| r & !Self::DMA_CFG_UPD_BY_BLK_MASK);
    }

    /// **DMA_BURST_LEN** `[11:0]` (RW)
    ///
    /// DMA burst length.
    #[inline(always)]
    pub fn dma_burst_len(&self) -> u32 {
        (self.value.read_bits() & Self::DMA_BURST_LEN_MASK) >> Self::DMA_BURST_LEN_OFFSET
    }

    /// Write **DMA_BURST_LEN** `[11:0]` (RW)
    ///
    /// DMA burst length.
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

    /// Read **DMA_FORCE_RD_STATUS** `[16:16]`
    ///
    /// 1: mask dma request when reading frame info. 0: disable mask.
    #[inline(always)]
    pub const fn dma_force_rd_status(&self) -> bool {
        (self.bits & DmaReqCfg::DMA_FORCE_RD_STATUS_MASK) != 0
    }

    /// Read **DMA_CFG_UPD_BY_BLK** `[12:12]`
    ///
    /// 1: reg_dma_burst_len & reg_dma_burst_len will be updated by dma block finish. 0: updated by frame.
    #[inline(always)]
    pub const fn dma_cfg_upd_by_blk(&self) -> bool {
        (self.bits & DmaReqCfg::DMA_CFG_UPD_BY_BLK_MASK) != 0
    }

    /// Read **DMA_BURST_LEN** `[11:0]`
    ///
    /// DMA burst length.
    #[inline(always)]
    pub const fn dma_burst_len(&self) -> u32 {
        (self.bits & DmaReqCfg::DMA_BURST_LEN_MASK) >> DmaReqCfg::DMA_BURST_LEN_OFFSET
    }

    /// Set **DMA_FORCE_RD_STATUS** `[16:16]`
    ///
    /// 1: mask dma request when reading frame info. 0: disable mask.
    #[inline(always)]
    pub const fn with_dma_force_rd_status(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaReqCfg::DMA_FORCE_RD_STATUS_MASK
            } else {
                self.bits & !DmaReqCfg::DMA_FORCE_RD_STATUS_MASK
            },
        }
    }

    /// Set **DMA_CFG_UPD_BY_BLK** `[12:12]`
    ///
    /// 1: reg_dma_burst_len & reg_dma_burst_len will be updated by dma block finish. 0: updated by frame.
    #[inline(always)]
    pub const fn with_dma_cfg_upd_by_blk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaReqCfg::DMA_CFG_UPD_BY_BLK_MASK
            } else {
                self.bits & !DmaReqCfg::DMA_CFG_UPD_BY_BLK_MASK
            },
        }
    }

    /// Set **DMA_BURST_LEN** `[11:0]`
    ///
    /// DMA burst length.
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

/// BUF_FLOW_CTL Register
///
/// csi bridge buffer control.
///
/// Address offset: 0x000c
/// Reset value: 0x000007f8
///
/// ## Fields
///
/// - **CSI_BUF_DEPTH** `[29:16]` (RO)
///
///   buffer data count.
///
/// - **CSI_BUF_AFULL_THRD** `[13:0]` (RW)
///
///   buffer almost full threshold.
#[repr(transparent)]
pub struct BufFlowCtl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BufFlowCtl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BufFlowCtl {
    // ======== Field Constants ========

    /// **CSI_BUF_DEPTH**
    ///
    /// buffer data count.
    ///
    /// Bits `[29:16]`, mask `0x3fff0000`
    pub const CSI_BUF_DEPTH_OFFSET: u32 = 16;
    pub const CSI_BUF_DEPTH_MASK: u32 = 0x3fff0000;

    /// **CSI_BUF_AFULL_THRD**
    ///
    /// buffer almost full threshold.
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const CSI_BUF_AFULL_THRD_OFFSET: u32 = 0;
    pub const CSI_BUF_AFULL_THRD_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **CSI_BUF_DEPTH** `[29:16]` (RO)
    ///
    /// buffer data count.
    #[inline(always)]
    pub fn csi_buf_depth(&self) -> u32 {
        (self.value.read_bits() & Self::CSI_BUF_DEPTH_MASK) >> Self::CSI_BUF_DEPTH_OFFSET
    }

    /// **CSI_BUF_AFULL_THRD** `[13:0]` (RW)
    ///
    /// buffer almost full threshold.
    #[inline(always)]
    pub fn csi_buf_afull_thrd(&self) -> u32 {
        (self.value.read_bits() & Self::CSI_BUF_AFULL_THRD_MASK) >> Self::CSI_BUF_AFULL_THRD_OFFSET
    }

    /// Write **CSI_BUF_AFULL_THRD** `[13:0]` (RW)
    ///
    /// buffer almost full threshold.
    #[inline(always)]
    pub fn write_csi_buf_afull_thrd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CSI_BUF_AFULL_THRD_MASK)
                | ((value << Self::CSI_BUF_AFULL_THRD_OFFSET) & Self::CSI_BUF_AFULL_THRD_MASK)
        });
    }
}

/// BUF_FLOW_CTL value type for batch read/modify/write operations
///
/// Use `bufflowctl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bufflowctl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BufFlowCtlVal {
    bits: u32,
}

impl BufFlowCtlVal {
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
        Self { bits: 0x000007f8 }
    }

    /// Read **CSI_BUF_DEPTH** `[29:16]`
    ///
    /// buffer data count.
    #[inline(always)]
    pub const fn csi_buf_depth(&self) -> u32 {
        (self.bits & BufFlowCtl::CSI_BUF_DEPTH_MASK) >> BufFlowCtl::CSI_BUF_DEPTH_OFFSET
    }

    /// Read **CSI_BUF_AFULL_THRD** `[13:0]`
    ///
    /// buffer almost full threshold.
    #[inline(always)]
    pub const fn csi_buf_afull_thrd(&self) -> u32 {
        (self.bits & BufFlowCtl::CSI_BUF_AFULL_THRD_MASK) >> BufFlowCtl::CSI_BUF_AFULL_THRD_OFFSET
    }

    /// Set **CSI_BUF_AFULL_THRD** `[13:0]`
    ///
    /// buffer almost full threshold.
    #[inline(always)]
    pub const fn with_csi_buf_afull_thrd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BufFlowCtl::CSI_BUF_AFULL_THRD_MASK)
                | ((value << BufFlowCtl::CSI_BUF_AFULL_THRD_OFFSET)
                    & BufFlowCtl::CSI_BUF_AFULL_THRD_MASK),
        }
    }
}

impl Default for BufFlowCtlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BufFlowCtl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BufFlowCtlVal {
        BufFlowCtlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BufFlowCtlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BufFlowCtlVal) -> BufFlowCtlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_TYPE_CFG Register
///
/// pixel data type configuration.
///
/// Address offset: 0x0010
/// Reset value: 0x00002f18
///
/// ## Fields
///
/// - **DATA_TYPE_MAX** `[13:8]` (RW)
///
///   the max value of data type used for pixel filter.
///
/// - **DATA_TYPE_MIN** `[5:0]` (RW)
///
///   the min value of data type used for pixel filter.
#[repr(transparent)]
pub struct DataTypeCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DataTypeCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DataTypeCfg {
    // ======== Field Constants ========

    /// **DATA_TYPE_MAX**
    ///
    /// the max value of data type used for pixel filter.
    ///
    /// Bits `[13:8]`, mask `0x00003f00`
    pub const DATA_TYPE_MAX_OFFSET: u32 = 8;
    pub const DATA_TYPE_MAX_MASK: u32 = 0x00003f00;

    /// **DATA_TYPE_MIN**
    ///
    /// the min value of data type used for pixel filter.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const DATA_TYPE_MIN_OFFSET: u32 = 0;
    pub const DATA_TYPE_MIN_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **DATA_TYPE_MAX** `[13:8]` (RW)
    ///
    /// the max value of data type used for pixel filter.
    #[inline(always)]
    pub fn data_type_max(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_TYPE_MAX_MASK) >> Self::DATA_TYPE_MAX_OFFSET
    }

    /// Write **DATA_TYPE_MAX** `[13:8]` (RW)
    ///
    /// the max value of data type used for pixel filter.
    #[inline(always)]
    pub fn write_data_type_max(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_TYPE_MAX_MASK)
                | ((value << Self::DATA_TYPE_MAX_OFFSET) & Self::DATA_TYPE_MAX_MASK)
        });
    }

    /// **DATA_TYPE_MIN** `[5:0]` (RW)
    ///
    /// the min value of data type used for pixel filter.
    #[inline(always)]
    pub fn data_type_min(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_TYPE_MIN_MASK) >> Self::DATA_TYPE_MIN_OFFSET
    }

    /// Write **DATA_TYPE_MIN** `[5:0]` (RW)
    ///
    /// the min value of data type used for pixel filter.
    #[inline(always)]
    pub fn write_data_type_min(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATA_TYPE_MIN_MASK)
                | ((value << Self::DATA_TYPE_MIN_OFFSET) & Self::DATA_TYPE_MIN_MASK)
        });
    }
}

/// DATA_TYPE_CFG value type for batch read/modify/write operations
///
/// Use `datatypecfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `datatypecfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DataTypeCfgVal {
    bits: u32,
}

impl DataTypeCfgVal {
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
        Self { bits: 0x00002f18 }
    }

    /// Read **DATA_TYPE_MAX** `[13:8]`
    ///
    /// the max value of data type used for pixel filter.
    #[inline(always)]
    pub const fn data_type_max(&self) -> u32 {
        (self.bits & DataTypeCfg::DATA_TYPE_MAX_MASK) >> DataTypeCfg::DATA_TYPE_MAX_OFFSET
    }

    /// Read **DATA_TYPE_MIN** `[5:0]`
    ///
    /// the min value of data type used for pixel filter.
    #[inline(always)]
    pub const fn data_type_min(&self) -> u32 {
        (self.bits & DataTypeCfg::DATA_TYPE_MIN_MASK) >> DataTypeCfg::DATA_TYPE_MIN_OFFSET
    }

    /// Set **DATA_TYPE_MAX** `[13:8]`
    ///
    /// the max value of data type used for pixel filter.
    #[inline(always)]
    pub const fn with_data_type_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DataTypeCfg::DATA_TYPE_MAX_MASK)
                | ((value << DataTypeCfg::DATA_TYPE_MAX_OFFSET) & DataTypeCfg::DATA_TYPE_MAX_MASK),
        }
    }

    /// Set **DATA_TYPE_MIN** `[5:0]`
    ///
    /// the min value of data type used for pixel filter.
    #[inline(always)]
    pub const fn with_data_type_min(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DataTypeCfg::DATA_TYPE_MIN_MASK)
                | ((value << DataTypeCfg::DATA_TYPE_MIN_OFFSET) & DataTypeCfg::DATA_TYPE_MIN_MASK),
        }
    }
}

impl Default for DataTypeCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DataTypeCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DataTypeCfgVal {
        DataTypeCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DataTypeCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DataTypeCfgVal) -> DataTypeCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FRAME_CFG Register
///
/// frame configuration.
///
/// Address offset: 0x0014
/// Reset value: 0x011e01e0
///
/// ## Fields
///
/// - **VADR_NUM_CHECK** `[25]` (RW)
///
///   0: disable vadr check. 1: enable vadr check.
///
/// - **HAS_HSYNC_E** `[24]` (RW)
///
///   0: frame data doesn't contain hsync. 1: frame data contains hsync.
///
/// - **HADR_NUM** `[23:12]` (RW)
///
///   hadr of frame data.
///
/// - **VADR_NUM** `[11:0]` (RW)
///
///   vadr of frame data.
#[repr(transparent)]
pub struct FrameCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FrameCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FrameCfg {
    // ======== Field Constants ========

    /// **VADR_NUM_CHECK**
    ///
    /// 0: disable vadr check. 1: enable vadr check.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const VADR_NUM_CHECK_OFFSET: u32 = 25;
    pub const VADR_NUM_CHECK_MASK: u32 = 0x02000000;
    pub const VADR_NUM_CHECK: u32 = 1 << 25;

    /// **HAS_HSYNC_E**
    ///
    /// 0: frame data doesn't contain hsync. 1: frame data contains hsync.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const HAS_HSYNC_E_OFFSET: u32 = 24;
    pub const HAS_HSYNC_E_MASK: u32 = 0x01000000;
    pub const HAS_HSYNC_E: u32 = 1 << 24;

    /// **HADR_NUM**
    ///
    /// hadr of frame data.
    ///
    /// Bits `[23:12]`, mask `0x00fff000`
    pub const HADR_NUM_OFFSET: u32 = 12;
    pub const HADR_NUM_MASK: u32 = 0x00fff000;

    /// **VADR_NUM**
    ///
    /// vadr of frame data.
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const VADR_NUM_OFFSET: u32 = 0;
    pub const VADR_NUM_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **VADR_NUM_CHECK** `[25]` (RW)
    ///
    /// 0: disable vadr check. 1: enable vadr check.
    #[inline(always)]
    pub fn vadr_num_check(&self) -> bool {
        (self.value.read_bits() & Self::VADR_NUM_CHECK_MASK) != 0
    }

    /// Set VADR_NUM_CHECK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vadr_num_check(&self) {
        self.value.modify_bits(|r| r | Self::VADR_NUM_CHECK_MASK);
    }

    /// Clear VADR_NUM_CHECK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vadr_num_check(&self) {
        self.value.modify_bits(|r| r & !Self::VADR_NUM_CHECK_MASK);
    }

    /// **HAS_HSYNC_E** `[24]` (RW)
    ///
    /// 0: frame data doesn't contain hsync. 1: frame data contains hsync.
    #[inline(always)]
    pub fn has_hsync_e(&self) -> bool {
        (self.value.read_bits() & Self::HAS_HSYNC_E_MASK) != 0
    }

    /// Set HAS_HSYNC_E bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_has_hsync_e(&self) {
        self.value.modify_bits(|r| r | Self::HAS_HSYNC_E_MASK);
    }

    /// Clear HAS_HSYNC_E bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_has_hsync_e(&self) {
        self.value.modify_bits(|r| r & !Self::HAS_HSYNC_E_MASK);
    }

    /// **HADR_NUM** `[23:12]` (RW)
    ///
    /// hadr of frame data.
    #[inline(always)]
    pub fn hadr_num(&self) -> u32 {
        (self.value.read_bits() & Self::HADR_NUM_MASK) >> Self::HADR_NUM_OFFSET
    }

    /// Write **HADR_NUM** `[23:12]` (RW)
    ///
    /// hadr of frame data.
    #[inline(always)]
    pub fn write_hadr_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HADR_NUM_MASK) | ((value << Self::HADR_NUM_OFFSET) & Self::HADR_NUM_MASK)
        });
    }

    /// **VADR_NUM** `[11:0]` (RW)
    ///
    /// vadr of frame data.
    #[inline(always)]
    pub fn vadr_num(&self) -> u32 {
        (self.value.read_bits() & Self::VADR_NUM_MASK) >> Self::VADR_NUM_OFFSET
    }

    /// Write **VADR_NUM** `[11:0]` (RW)
    ///
    /// vadr of frame data.
    #[inline(always)]
    pub fn write_vadr_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VADR_NUM_MASK) | ((value << Self::VADR_NUM_OFFSET) & Self::VADR_NUM_MASK)
        });
    }
}

/// FRAME_CFG value type for batch read/modify/write operations
///
/// Use `framecfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `framecfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FrameCfgVal {
    bits: u32,
}

impl FrameCfgVal {
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
        Self { bits: 0x011e01e0 }
    }

    /// Read **VADR_NUM_CHECK** `[25:25]`
    ///
    /// 0: disable vadr check. 1: enable vadr check.
    #[inline(always)]
    pub const fn vadr_num_check(&self) -> bool {
        (self.bits & FrameCfg::VADR_NUM_CHECK_MASK) != 0
    }

    /// Read **HAS_HSYNC_E** `[24:24]`
    ///
    /// 0: frame data doesn't contain hsync. 1: frame data contains hsync.
    #[inline(always)]
    pub const fn has_hsync_e(&self) -> bool {
        (self.bits & FrameCfg::HAS_HSYNC_E_MASK) != 0
    }

    /// Read **HADR_NUM** `[23:12]`
    ///
    /// hadr of frame data.
    #[inline(always)]
    pub const fn hadr_num(&self) -> u32 {
        (self.bits & FrameCfg::HADR_NUM_MASK) >> FrameCfg::HADR_NUM_OFFSET
    }

    /// Read **VADR_NUM** `[11:0]`
    ///
    /// vadr of frame data.
    #[inline(always)]
    pub const fn vadr_num(&self) -> u32 {
        (self.bits & FrameCfg::VADR_NUM_MASK) >> FrameCfg::VADR_NUM_OFFSET
    }

    /// Set **VADR_NUM_CHECK** `[25:25]`
    ///
    /// 0: disable vadr check. 1: enable vadr check.
    #[inline(always)]
    pub const fn with_vadr_num_check(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrameCfg::VADR_NUM_CHECK_MASK
            } else {
                self.bits & !FrameCfg::VADR_NUM_CHECK_MASK
            },
        }
    }

    /// Set **HAS_HSYNC_E** `[24:24]`
    ///
    /// 0: frame data doesn't contain hsync. 1: frame data contains hsync.
    #[inline(always)]
    pub const fn with_has_hsync_e(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FrameCfg::HAS_HSYNC_E_MASK
            } else {
                self.bits & !FrameCfg::HAS_HSYNC_E_MASK
            },
        }
    }

    /// Set **HADR_NUM** `[23:12]`
    ///
    /// hadr of frame data.
    #[inline(always)]
    pub const fn with_hadr_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FrameCfg::HADR_NUM_MASK)
                | ((value << FrameCfg::HADR_NUM_OFFSET) & FrameCfg::HADR_NUM_MASK),
        }
    }

    /// Set **VADR_NUM** `[11:0]`
    ///
    /// vadr of frame data.
    #[inline(always)]
    pub const fn with_vadr_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FrameCfg::VADR_NUM_MASK)
                | ((value << FrameCfg::VADR_NUM_OFFSET) & FrameCfg::VADR_NUM_MASK),
        }
    }
}

impl Default for FrameCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl FrameCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FrameCfgVal {
        FrameCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FrameCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FrameCfgVal) -> FrameCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ENDIAN_MODE Register
///
/// data endianness order configuration.
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **BIT_ENDIAN_ORDER** `[1]` (RW)
///
///   N/A
///
/// - **BYTE_ENDIAN_ORDER** `[0]` (RW)
///
///   endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed.
#[repr(transparent)]
pub struct EndianMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EndianMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EndianMode {
    // ======== Field Constants ========

    /// **BIT_ENDIAN_ORDER**
    ///
    /// N/A
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BIT_ENDIAN_ORDER_OFFSET: u32 = 1;
    pub const BIT_ENDIAN_ORDER_MASK: u32 = 0x00000002;
    pub const BIT_ENDIAN_ORDER: u32 = 1 << 1;

    /// **BYTE_ENDIAN_ORDER**
    ///
    /// endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const BYTE_ENDIAN_ORDER_OFFSET: u32 = 0;
    pub const BYTE_ENDIAN_ORDER_MASK: u32 = 0x00000001;
    pub const BYTE_ENDIAN_ORDER: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BIT_ENDIAN_ORDER** `[1]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn bit_endian_order(&self) -> bool {
        (self.value.read_bits() & Self::BIT_ENDIAN_ORDER_MASK) != 0
    }

    /// Set BIT_ENDIAN_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bit_endian_order(&self) {
        self.value.modify_bits(|r| r | Self::BIT_ENDIAN_ORDER_MASK);
    }

    /// Clear BIT_ENDIAN_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bit_endian_order(&self) {
        self.value.modify_bits(|r| r & !Self::BIT_ENDIAN_ORDER_MASK);
    }

    /// **BYTE_ENDIAN_ORDER** `[0]` (RW)
    ///
    /// endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed.
    #[inline(always)]
    pub fn byte_endian_order(&self) -> bool {
        (self.value.read_bits() & Self::BYTE_ENDIAN_ORDER_MASK) != 0
    }

    /// Set BYTE_ENDIAN_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_byte_endian_order(&self) {
        self.value.modify_bits(|r| r | Self::BYTE_ENDIAN_ORDER_MASK);
    }

    /// Clear BYTE_ENDIAN_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_byte_endian_order(&self) {
        self.value
            .modify_bits(|r| r & !Self::BYTE_ENDIAN_ORDER_MASK);
    }
}

/// ENDIAN_MODE value type for batch read/modify/write operations
///
/// Use `endianmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `endianmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EndianModeVal {
    bits: u32,
}

impl EndianModeVal {
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

    /// Read **BIT_ENDIAN_ORDER** `[1:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn bit_endian_order(&self) -> bool {
        (self.bits & EndianMode::BIT_ENDIAN_ORDER_MASK) != 0
    }

    /// Read **BYTE_ENDIAN_ORDER** `[0:0]`
    ///
    /// endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed.
    #[inline(always)]
    pub const fn byte_endian_order(&self) -> bool {
        (self.bits & EndianMode::BYTE_ENDIAN_ORDER_MASK) != 0
    }

    /// Set **BIT_ENDIAN_ORDER** `[1:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_bit_endian_order(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EndianMode::BIT_ENDIAN_ORDER_MASK
            } else {
                self.bits & !EndianMode::BIT_ENDIAN_ORDER_MASK
            },
        }
    }

    /// Set **BYTE_ENDIAN_ORDER** `[0:0]`
    ///
    /// endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed.
    #[inline(always)]
    pub const fn with_byte_endian_order(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EndianMode::BYTE_ENDIAN_ORDER_MASK
            } else {
                self.bits & !EndianMode::BYTE_ENDIAN_ORDER_MASK
            },
        }
    }
}

impl Default for EndianModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EndianMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EndianModeVal {
        EndianModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EndianModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EndianModeVal) -> EndianModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_RAW Register
///
/// csi bridge interrupt raw.
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **DMA_CFG_HAS_UPDATED_INT_RAW** `[5]` (RW)
///
///   dma configuration update complete interrupt raw.
///
/// - **CSI_ASYNC_FIFO_OVF_INT_RAW** `[4]` (RW)
///
///   buffer overflow interrupt raw.
///
/// - **CSI_BUF_OVERRUN_INT_RAW** `[3]` (RW)
///
///   buffer overrun interrupt raw.
///
/// - **DISCARD_INT_RAW** `[2]` (RW)
///
///   an incomplete frame of data was sent interrupt raw.
///
/// - **VADR_NUM_LT_INT_RAW** `[1]` (RW)
///
///   reg_vadr_num is less than real interrupt raw.
///
/// - **VADR_NUM_GT_INT_RAW** `[0]` (RW)
///
///   reg_vadr_num is greater than real interrupt raw.
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

    /// **DMA_CFG_HAS_UPDATED_INT_RAW**
    ///
    /// dma configuration update complete interrupt raw.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DMA_CFG_HAS_UPDATED_INT_RAW_OFFSET: u32 = 5;
    pub const DMA_CFG_HAS_UPDATED_INT_RAW_MASK: u32 = 0x00000020;
    pub const DMA_CFG_HAS_UPDATED_INT_RAW: u32 = 1 << 5;

    /// **CSI_ASYNC_FIFO_OVF_INT_RAW**
    ///
    /// buffer overflow interrupt raw.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CSI_ASYNC_FIFO_OVF_INT_RAW_OFFSET: u32 = 4;
    pub const CSI_ASYNC_FIFO_OVF_INT_RAW_MASK: u32 = 0x00000010;
    pub const CSI_ASYNC_FIFO_OVF_INT_RAW: u32 = 1 << 4;

    /// **CSI_BUF_OVERRUN_INT_RAW**
    ///
    /// buffer overrun interrupt raw.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CSI_BUF_OVERRUN_INT_RAW_OFFSET: u32 = 3;
    pub const CSI_BUF_OVERRUN_INT_RAW_MASK: u32 = 0x00000008;
    pub const CSI_BUF_OVERRUN_INT_RAW: u32 = 1 << 3;

    /// **DISCARD_INT_RAW**
    ///
    /// an incomplete frame of data was sent interrupt raw.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DISCARD_INT_RAW_OFFSET: u32 = 2;
    pub const DISCARD_INT_RAW_MASK: u32 = 0x00000004;
    pub const DISCARD_INT_RAW: u32 = 1 << 2;

    /// **VADR_NUM_LT_INT_RAW**
    ///
    /// reg_vadr_num is less than real interrupt raw.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const VADR_NUM_LT_INT_RAW_OFFSET: u32 = 1;
    pub const VADR_NUM_LT_INT_RAW_MASK: u32 = 0x00000002;
    pub const VADR_NUM_LT_INT_RAW: u32 = 1 << 1;

    /// **VADR_NUM_GT_INT_RAW**
    ///
    /// reg_vadr_num is greater than real interrupt raw.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VADR_NUM_GT_INT_RAW_OFFSET: u32 = 0;
    pub const VADR_NUM_GT_INT_RAW_MASK: u32 = 0x00000001;
    pub const VADR_NUM_GT_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DMA_CFG_HAS_UPDATED_INT_RAW** `[5]` (RW)
    ///
    /// dma configuration update complete interrupt raw.
    #[inline(always)]
    pub fn dma_cfg_has_updated_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DMA_CFG_HAS_UPDATED_INT_RAW_MASK) != 0
    }

    /// Set DMA_CFG_HAS_UPDATED_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma_cfg_has_updated_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DMA_CFG_HAS_UPDATED_INT_RAW_MASK);
    }

    /// Clear DMA_CFG_HAS_UPDATED_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma_cfg_has_updated_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DMA_CFG_HAS_UPDATED_INT_RAW_MASK);
    }

    /// **CSI_ASYNC_FIFO_OVF_INT_RAW** `[4]` (RW)
    ///
    /// buffer overflow interrupt raw.
    #[inline(always)]
    pub fn csi_async_fifo_ovf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CSI_ASYNC_FIFO_OVF_INT_RAW_MASK) != 0
    }

    /// Set CSI_ASYNC_FIFO_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_csi_async_fifo_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CSI_ASYNC_FIFO_OVF_INT_RAW_MASK);
    }

    /// Clear CSI_ASYNC_FIFO_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_csi_async_fifo_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CSI_ASYNC_FIFO_OVF_INT_RAW_MASK);
    }

    /// **CSI_BUF_OVERRUN_INT_RAW** `[3]` (RW)
    ///
    /// buffer overrun interrupt raw.
    #[inline(always)]
    pub fn csi_buf_overrun_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CSI_BUF_OVERRUN_INT_RAW_MASK) != 0
    }

    /// Set CSI_BUF_OVERRUN_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_csi_buf_overrun_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CSI_BUF_OVERRUN_INT_RAW_MASK);
    }

    /// Clear CSI_BUF_OVERRUN_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_csi_buf_overrun_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CSI_BUF_OVERRUN_INT_RAW_MASK);
    }

    /// **DISCARD_INT_RAW** `[2]` (RW)
    ///
    /// an incomplete frame of data was sent interrupt raw.
    #[inline(always)]
    pub fn discard_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DISCARD_INT_RAW_MASK) != 0
    }

    /// Set DISCARD_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_discard_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::DISCARD_INT_RAW_MASK);
    }

    /// Clear DISCARD_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_discard_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::DISCARD_INT_RAW_MASK);
    }

    /// **VADR_NUM_LT_INT_RAW** `[1]` (RW)
    ///
    /// reg_vadr_num is less than real interrupt raw.
    #[inline(always)]
    pub fn vadr_num_lt_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::VADR_NUM_LT_INT_RAW_MASK) != 0
    }

    /// Set VADR_NUM_LT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vadr_num_lt_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::VADR_NUM_LT_INT_RAW_MASK);
    }

    /// Clear VADR_NUM_LT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vadr_num_lt_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::VADR_NUM_LT_INT_RAW_MASK);
    }

    /// **VADR_NUM_GT_INT_RAW** `[0]` (RW)
    ///
    /// reg_vadr_num is greater than real interrupt raw.
    #[inline(always)]
    pub fn vadr_num_gt_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::VADR_NUM_GT_INT_RAW_MASK) != 0
    }

    /// Set VADR_NUM_GT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vadr_num_gt_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::VADR_NUM_GT_INT_RAW_MASK);
    }

    /// Clear VADR_NUM_GT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vadr_num_gt_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::VADR_NUM_GT_INT_RAW_MASK);
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

    /// Read **DMA_CFG_HAS_UPDATED_INT_RAW** `[5:5]`
    ///
    /// dma configuration update complete interrupt raw.
    #[inline(always)]
    pub const fn dma_cfg_has_updated_int_raw(&self) -> bool {
        (self.bits & IntRaw::DMA_CFG_HAS_UPDATED_INT_RAW_MASK) != 0
    }

    /// Read **CSI_ASYNC_FIFO_OVF_INT_RAW** `[4:4]`
    ///
    /// buffer overflow interrupt raw.
    #[inline(always)]
    pub const fn csi_async_fifo_ovf_int_raw(&self) -> bool {
        (self.bits & IntRaw::CSI_ASYNC_FIFO_OVF_INT_RAW_MASK) != 0
    }

    /// Read **CSI_BUF_OVERRUN_INT_RAW** `[3:3]`
    ///
    /// buffer overrun interrupt raw.
    #[inline(always)]
    pub const fn csi_buf_overrun_int_raw(&self) -> bool {
        (self.bits & IntRaw::CSI_BUF_OVERRUN_INT_RAW_MASK) != 0
    }

    /// Read **DISCARD_INT_RAW** `[2:2]`
    ///
    /// an incomplete frame of data was sent interrupt raw.
    #[inline(always)]
    pub const fn discard_int_raw(&self) -> bool {
        (self.bits & IntRaw::DISCARD_INT_RAW_MASK) != 0
    }

    /// Read **VADR_NUM_LT_INT_RAW** `[1:1]`
    ///
    /// reg_vadr_num is less than real interrupt raw.
    #[inline(always)]
    pub const fn vadr_num_lt_int_raw(&self) -> bool {
        (self.bits & IntRaw::VADR_NUM_LT_INT_RAW_MASK) != 0
    }

    /// Read **VADR_NUM_GT_INT_RAW** `[0:0]`
    ///
    /// reg_vadr_num is greater than real interrupt raw.
    #[inline(always)]
    pub const fn vadr_num_gt_int_raw(&self) -> bool {
        (self.bits & IntRaw::VADR_NUM_GT_INT_RAW_MASK) != 0
    }

    /// Set **DMA_CFG_HAS_UPDATED_INT_RAW** `[5:5]`
    ///
    /// dma configuration update complete interrupt raw.
    #[inline(always)]
    pub const fn with_dma_cfg_has_updated_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DMA_CFG_HAS_UPDATED_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DMA_CFG_HAS_UPDATED_INT_RAW_MASK
            },
        }
    }

    /// Set **CSI_ASYNC_FIFO_OVF_INT_RAW** `[4:4]`
    ///
    /// buffer overflow interrupt raw.
    #[inline(always)]
    pub const fn with_csi_async_fifo_ovf_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CSI_ASYNC_FIFO_OVF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CSI_ASYNC_FIFO_OVF_INT_RAW_MASK
            },
        }
    }

    /// Set **CSI_BUF_OVERRUN_INT_RAW** `[3:3]`
    ///
    /// buffer overrun interrupt raw.
    #[inline(always)]
    pub const fn with_csi_buf_overrun_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CSI_BUF_OVERRUN_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CSI_BUF_OVERRUN_INT_RAW_MASK
            },
        }
    }

    /// Set **DISCARD_INT_RAW** `[2:2]`
    ///
    /// an incomplete frame of data was sent interrupt raw.
    #[inline(always)]
    pub const fn with_discard_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DISCARD_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DISCARD_INT_RAW_MASK
            },
        }
    }

    /// Set **VADR_NUM_LT_INT_RAW** `[1:1]`
    ///
    /// reg_vadr_num is less than real interrupt raw.
    #[inline(always)]
    pub const fn with_vadr_num_lt_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::VADR_NUM_LT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::VADR_NUM_LT_INT_RAW_MASK
            },
        }
    }

    /// Set **VADR_NUM_GT_INT_RAW** `[0:0]`
    ///
    /// reg_vadr_num is greater than real interrupt raw.
    #[inline(always)]
    pub const fn with_vadr_num_gt_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::VADR_NUM_GT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::VADR_NUM_GT_INT_RAW_MASK
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
/// csi bridge interrupt clr.
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **DMA_CFG_HAS_UPDATED_INT_CLR** `[5]` (WO)
///
///   dma configuration update complete interrupt clr.
///
/// - **CSI_ASYNC_FIFO_OVF_INT_CLR** `[4]` (WO)
///
///   buffer overflow interrupt clr.
///
/// - **CSI_BUF_OVERRUN_INT_CLR** `[3]` (WO)
///
///   buffer overrun interrupt clr.
///
/// - **DISCARD_INT_CLR** `[2]` (WO)
///
///   an incomplete frame of data was sent interrupt clr.
///
/// - **VADR_NUM_LT_REAL_INT_CLR** `[1]` (WO)
///
///   reg_vadr_num is less than real interrupt clr.
///
/// - **VADR_NUM_GT_REAL_INT_CLR** `[0]` (WO)
///
///   reg_vadr_num is greater than real interrupt clr.
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

    /// **DMA_CFG_HAS_UPDATED_INT_CLR**
    ///
    /// dma configuration update complete interrupt clr.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DMA_CFG_HAS_UPDATED_INT_CLR_OFFSET: u32 = 5;
    pub const DMA_CFG_HAS_UPDATED_INT_CLR_MASK: u32 = 0x00000020;
    pub const DMA_CFG_HAS_UPDATED_INT_CLR: u32 = 1 << 5;

    /// **CSI_ASYNC_FIFO_OVF_INT_CLR**
    ///
    /// buffer overflow interrupt clr.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CSI_ASYNC_FIFO_OVF_INT_CLR_OFFSET: u32 = 4;
    pub const CSI_ASYNC_FIFO_OVF_INT_CLR_MASK: u32 = 0x00000010;
    pub const CSI_ASYNC_FIFO_OVF_INT_CLR: u32 = 1 << 4;

    /// **CSI_BUF_OVERRUN_INT_CLR**
    ///
    /// buffer overrun interrupt clr.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CSI_BUF_OVERRUN_INT_CLR_OFFSET: u32 = 3;
    pub const CSI_BUF_OVERRUN_INT_CLR_MASK: u32 = 0x00000008;
    pub const CSI_BUF_OVERRUN_INT_CLR: u32 = 1 << 3;

    /// **DISCARD_INT_CLR**
    ///
    /// an incomplete frame of data was sent interrupt clr.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DISCARD_INT_CLR_OFFSET: u32 = 2;
    pub const DISCARD_INT_CLR_MASK: u32 = 0x00000004;
    pub const DISCARD_INT_CLR: u32 = 1 << 2;

    /// **VADR_NUM_LT_REAL_INT_CLR**
    ///
    /// reg_vadr_num is less than real interrupt clr.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const VADR_NUM_LT_REAL_INT_CLR_OFFSET: u32 = 1;
    pub const VADR_NUM_LT_REAL_INT_CLR_MASK: u32 = 0x00000002;
    pub const VADR_NUM_LT_REAL_INT_CLR: u32 = 1 << 1;

    /// **VADR_NUM_GT_REAL_INT_CLR**
    ///
    /// reg_vadr_num is greater than real interrupt clr.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VADR_NUM_GT_REAL_INT_CLR_OFFSET: u32 = 0;
    pub const VADR_NUM_GT_REAL_INT_CLR_MASK: u32 = 0x00000001;
    pub const VADR_NUM_GT_REAL_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **DMA_CFG_HAS_UPDATED_INT_CLR** (WO)
    ///
    /// dma configuration update complete interrupt clr.
    #[inline(always)]
    pub fn trigger_dma_cfg_has_updated_int_clr(&self) {
        self.value
            .write_bits(Self::DMA_CFG_HAS_UPDATED_INT_CLR_MASK);
    }

    /// Trigger **CSI_ASYNC_FIFO_OVF_INT_CLR** (WO)
    ///
    /// buffer overflow interrupt clr.
    #[inline(always)]
    pub fn trigger_csi_async_fifo_ovf_int_clr(&self) {
        self.value.write_bits(Self::CSI_ASYNC_FIFO_OVF_INT_CLR_MASK);
    }

    /// Trigger **CSI_BUF_OVERRUN_INT_CLR** (WO)
    ///
    /// buffer overrun interrupt clr.
    #[inline(always)]
    pub fn trigger_csi_buf_overrun_int_clr(&self) {
        self.value.write_bits(Self::CSI_BUF_OVERRUN_INT_CLR_MASK);
    }

    /// Trigger **DISCARD_INT_CLR** (WO)
    ///
    /// an incomplete frame of data was sent interrupt clr.
    #[inline(always)]
    pub fn trigger_discard_int_clr(&self) {
        self.value.write_bits(Self::DISCARD_INT_CLR_MASK);
    }

    /// Trigger **VADR_NUM_LT_REAL_INT_CLR** (WO)
    ///
    /// reg_vadr_num is less than real interrupt clr.
    #[inline(always)]
    pub fn trigger_vadr_num_lt_real_int_clr(&self) {
        self.value.write_bits(Self::VADR_NUM_LT_REAL_INT_CLR_MASK);
    }

    /// Trigger **VADR_NUM_GT_REAL_INT_CLR** (WO)
    ///
    /// reg_vadr_num is greater than real interrupt clr.
    #[inline(always)]
    pub fn trigger_vadr_num_gt_real_int_clr(&self) {
        self.value.write_bits(Self::VADR_NUM_GT_REAL_INT_CLR_MASK);
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

    /// Set **DMA_CFG_HAS_UPDATED_INT_CLR** `[5:5]`
    ///
    /// dma configuration update complete interrupt clr.
    #[inline(always)]
    pub const fn with_dma_cfg_has_updated_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DMA_CFG_HAS_UPDATED_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DMA_CFG_HAS_UPDATED_INT_CLR_MASK
            },
        }
    }

    /// Set **CSI_ASYNC_FIFO_OVF_INT_CLR** `[4:4]`
    ///
    /// buffer overflow interrupt clr.
    #[inline(always)]
    pub const fn with_csi_async_fifo_ovf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CSI_ASYNC_FIFO_OVF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CSI_ASYNC_FIFO_OVF_INT_CLR_MASK
            },
        }
    }

    /// Set **CSI_BUF_OVERRUN_INT_CLR** `[3:3]`
    ///
    /// buffer overrun interrupt clr.
    #[inline(always)]
    pub const fn with_csi_buf_overrun_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CSI_BUF_OVERRUN_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CSI_BUF_OVERRUN_INT_CLR_MASK
            },
        }
    }

    /// Set **DISCARD_INT_CLR** `[2:2]`
    ///
    /// an incomplete frame of data was sent interrupt clr.
    #[inline(always)]
    pub const fn with_discard_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DISCARD_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DISCARD_INT_CLR_MASK
            },
        }
    }

    /// Set **VADR_NUM_LT_REAL_INT_CLR** `[1:1]`
    ///
    /// reg_vadr_num is less than real interrupt clr.
    #[inline(always)]
    pub const fn with_vadr_num_lt_real_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::VADR_NUM_LT_REAL_INT_CLR_MASK
            } else {
                self.bits & !IntClr::VADR_NUM_LT_REAL_INT_CLR_MASK
            },
        }
    }

    /// Set **VADR_NUM_GT_REAL_INT_CLR** `[0:0]`
    ///
    /// reg_vadr_num is greater than real interrupt clr.
    #[inline(always)]
    pub const fn with_vadr_num_gt_real_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::VADR_NUM_GT_REAL_INT_CLR_MASK
            } else {
                self.bits & !IntClr::VADR_NUM_GT_REAL_INT_CLR_MASK
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
/// csi bridge interrupt st.
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **DMA_CFG_HAS_UPDATED_INT_ST** `[5]` (RO)
///
///   dma configuration update complete interrupt st.
///
/// - **CSI_ASYNC_FIFO_OVF_INT_ST** `[4]` (RO)
///
///   buffer overflow interrupt st.
///
/// - **CSI_BUF_OVERRUN_INT_ST** `[3]` (RO)
///
///   buffer overrun interrupt st.
///
/// - **DISCARD_INT_ST** `[2]` (RO)
///
///   an incomplete frame of data was sent interrupt st.
///
/// - **VADR_NUM_LT_INT_ST** `[1]` (RO)
///
///   reg_vadr_num is less than real interrupt st.
///
/// - **VADR_NUM_GT_INT_ST** `[0]` (RO)
///
///   reg_vadr_num is greater than real interrupt st.
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

    /// **DMA_CFG_HAS_UPDATED_INT_ST**
    ///
    /// dma configuration update complete interrupt st.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DMA_CFG_HAS_UPDATED_INT_ST_OFFSET: u32 = 5;
    pub const DMA_CFG_HAS_UPDATED_INT_ST_MASK: u32 = 0x00000020;
    pub const DMA_CFG_HAS_UPDATED_INT_ST: u32 = 1 << 5;

    /// **CSI_ASYNC_FIFO_OVF_INT_ST**
    ///
    /// buffer overflow interrupt st.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CSI_ASYNC_FIFO_OVF_INT_ST_OFFSET: u32 = 4;
    pub const CSI_ASYNC_FIFO_OVF_INT_ST_MASK: u32 = 0x00000010;
    pub const CSI_ASYNC_FIFO_OVF_INT_ST: u32 = 1 << 4;

    /// **CSI_BUF_OVERRUN_INT_ST**
    ///
    /// buffer overrun interrupt st.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CSI_BUF_OVERRUN_INT_ST_OFFSET: u32 = 3;
    pub const CSI_BUF_OVERRUN_INT_ST_MASK: u32 = 0x00000008;
    pub const CSI_BUF_OVERRUN_INT_ST: u32 = 1 << 3;

    /// **DISCARD_INT_ST**
    ///
    /// an incomplete frame of data was sent interrupt st.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DISCARD_INT_ST_OFFSET: u32 = 2;
    pub const DISCARD_INT_ST_MASK: u32 = 0x00000004;
    pub const DISCARD_INT_ST: u32 = 1 << 2;

    /// **VADR_NUM_LT_INT_ST**
    ///
    /// reg_vadr_num is less than real interrupt st.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const VADR_NUM_LT_INT_ST_OFFSET: u32 = 1;
    pub const VADR_NUM_LT_INT_ST_MASK: u32 = 0x00000002;
    pub const VADR_NUM_LT_INT_ST: u32 = 1 << 1;

    /// **VADR_NUM_GT_INT_ST**
    ///
    /// reg_vadr_num is greater than real interrupt st.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VADR_NUM_GT_INT_ST_OFFSET: u32 = 0;
    pub const VADR_NUM_GT_INT_ST_MASK: u32 = 0x00000001;
    pub const VADR_NUM_GT_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DMA_CFG_HAS_UPDATED_INT_ST** `[5]` (RO)
    ///
    /// dma configuration update complete interrupt st.
    #[inline(always)]
    pub fn dma_cfg_has_updated_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DMA_CFG_HAS_UPDATED_INT_ST_MASK) != 0
    }

    /// **CSI_ASYNC_FIFO_OVF_INT_ST** `[4]` (RO)
    ///
    /// buffer overflow interrupt st.
    #[inline(always)]
    pub fn csi_async_fifo_ovf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CSI_ASYNC_FIFO_OVF_INT_ST_MASK) != 0
    }

    /// **CSI_BUF_OVERRUN_INT_ST** `[3]` (RO)
    ///
    /// buffer overrun interrupt st.
    #[inline(always)]
    pub fn csi_buf_overrun_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CSI_BUF_OVERRUN_INT_ST_MASK) != 0
    }

    /// **DISCARD_INT_ST** `[2]` (RO)
    ///
    /// an incomplete frame of data was sent interrupt st.
    #[inline(always)]
    pub fn discard_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DISCARD_INT_ST_MASK) != 0
    }

    /// **VADR_NUM_LT_INT_ST** `[1]` (RO)
    ///
    /// reg_vadr_num is less than real interrupt st.
    #[inline(always)]
    pub fn vadr_num_lt_int_st(&self) -> bool {
        (self.value.read_bits() & Self::VADR_NUM_LT_INT_ST_MASK) != 0
    }

    /// **VADR_NUM_GT_INT_ST** `[0]` (RO)
    ///
    /// reg_vadr_num is greater than real interrupt st.
    #[inline(always)]
    pub fn vadr_num_gt_int_st(&self) -> bool {
        (self.value.read_bits() & Self::VADR_NUM_GT_INT_ST_MASK) != 0
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

    /// Read **DMA_CFG_HAS_UPDATED_INT_ST** `[5:5]`
    ///
    /// dma configuration update complete interrupt st.
    #[inline(always)]
    pub const fn dma_cfg_has_updated_int_st(&self) -> bool {
        (self.bits & IntSt::DMA_CFG_HAS_UPDATED_INT_ST_MASK) != 0
    }

    /// Read **CSI_ASYNC_FIFO_OVF_INT_ST** `[4:4]`
    ///
    /// buffer overflow interrupt st.
    #[inline(always)]
    pub const fn csi_async_fifo_ovf_int_st(&self) -> bool {
        (self.bits & IntSt::CSI_ASYNC_FIFO_OVF_INT_ST_MASK) != 0
    }

    /// Read **CSI_BUF_OVERRUN_INT_ST** `[3:3]`
    ///
    /// buffer overrun interrupt st.
    #[inline(always)]
    pub const fn csi_buf_overrun_int_st(&self) -> bool {
        (self.bits & IntSt::CSI_BUF_OVERRUN_INT_ST_MASK) != 0
    }

    /// Read **DISCARD_INT_ST** `[2:2]`
    ///
    /// an incomplete frame of data was sent interrupt st.
    #[inline(always)]
    pub const fn discard_int_st(&self) -> bool {
        (self.bits & IntSt::DISCARD_INT_ST_MASK) != 0
    }

    /// Read **VADR_NUM_LT_INT_ST** `[1:1]`
    ///
    /// reg_vadr_num is less than real interrupt st.
    #[inline(always)]
    pub const fn vadr_num_lt_int_st(&self) -> bool {
        (self.bits & IntSt::VADR_NUM_LT_INT_ST_MASK) != 0
    }

    /// Read **VADR_NUM_GT_INT_ST** `[0:0]`
    ///
    /// reg_vadr_num is greater than real interrupt st.
    #[inline(always)]
    pub const fn vadr_num_gt_int_st(&self) -> bool {
        (self.bits & IntSt::VADR_NUM_GT_INT_ST_MASK) != 0
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
/// csi bridge interrupt enable.
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **DMA_CFG_HAS_UPDATED_INT_ENA** `[5]` (RW)
///
///   dma configuration update complete interrupt enable.
///
/// - **CSI_ASYNC_FIFO_OVF_INT_ENA** `[4]` (RW)
///
///   buffer overflow interrupt enable.
///
/// - **CSI_BUF_OVERRUN_INT_ENA** `[3]` (RW)
///
///   buffer overrun interrupt enable.
///
/// - **DISCARD_INT_ENA** `[2]` (RW)
///
///   an incomplete frame of data was sent interrupt enable.
///
/// - **VADR_NUM_LT_INT_ENA** `[1]` (RW)
///
///   reg_vadr_num is less than real interrupt enable.
///
/// - **VADR_NUM_GT_INT_ENA** `[0]` (RW)
///
///   reg_vadr_num is greater than real interrupt enable.
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

    /// **DMA_CFG_HAS_UPDATED_INT_ENA**
    ///
    /// dma configuration update complete interrupt enable.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DMA_CFG_HAS_UPDATED_INT_ENA_OFFSET: u32 = 5;
    pub const DMA_CFG_HAS_UPDATED_INT_ENA_MASK: u32 = 0x00000020;
    pub const DMA_CFG_HAS_UPDATED_INT_ENA: u32 = 1 << 5;

    /// **CSI_ASYNC_FIFO_OVF_INT_ENA**
    ///
    /// buffer overflow interrupt enable.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CSI_ASYNC_FIFO_OVF_INT_ENA_OFFSET: u32 = 4;
    pub const CSI_ASYNC_FIFO_OVF_INT_ENA_MASK: u32 = 0x00000010;
    pub const CSI_ASYNC_FIFO_OVF_INT_ENA: u32 = 1 << 4;

    /// **CSI_BUF_OVERRUN_INT_ENA**
    ///
    /// buffer overrun interrupt enable.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CSI_BUF_OVERRUN_INT_ENA_OFFSET: u32 = 3;
    pub const CSI_BUF_OVERRUN_INT_ENA_MASK: u32 = 0x00000008;
    pub const CSI_BUF_OVERRUN_INT_ENA: u32 = 1 << 3;

    /// **DISCARD_INT_ENA**
    ///
    /// an incomplete frame of data was sent interrupt enable.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DISCARD_INT_ENA_OFFSET: u32 = 2;
    pub const DISCARD_INT_ENA_MASK: u32 = 0x00000004;
    pub const DISCARD_INT_ENA: u32 = 1 << 2;

    /// **VADR_NUM_LT_INT_ENA**
    ///
    /// reg_vadr_num is less than real interrupt enable.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const VADR_NUM_LT_INT_ENA_OFFSET: u32 = 1;
    pub const VADR_NUM_LT_INT_ENA_MASK: u32 = 0x00000002;
    pub const VADR_NUM_LT_INT_ENA: u32 = 1 << 1;

    /// **VADR_NUM_GT_INT_ENA**
    ///
    /// reg_vadr_num is greater than real interrupt enable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VADR_NUM_GT_INT_ENA_OFFSET: u32 = 0;
    pub const VADR_NUM_GT_INT_ENA_MASK: u32 = 0x00000001;
    pub const VADR_NUM_GT_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DMA_CFG_HAS_UPDATED_INT_ENA** `[5]` (RW)
    ///
    /// dma configuration update complete interrupt enable.
    #[inline(always)]
    pub fn dma_cfg_has_updated_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DMA_CFG_HAS_UPDATED_INT_ENA_MASK) != 0
    }

    /// Set DMA_CFG_HAS_UPDATED_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma_cfg_has_updated_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DMA_CFG_HAS_UPDATED_INT_ENA_MASK);
    }

    /// Clear DMA_CFG_HAS_UPDATED_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma_cfg_has_updated_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DMA_CFG_HAS_UPDATED_INT_ENA_MASK);
    }

    /// **CSI_ASYNC_FIFO_OVF_INT_ENA** `[4]` (RW)
    ///
    /// buffer overflow interrupt enable.
    #[inline(always)]
    pub fn csi_async_fifo_ovf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CSI_ASYNC_FIFO_OVF_INT_ENA_MASK) != 0
    }

    /// Set CSI_ASYNC_FIFO_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_csi_async_fifo_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CSI_ASYNC_FIFO_OVF_INT_ENA_MASK);
    }

    /// Clear CSI_ASYNC_FIFO_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_csi_async_fifo_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CSI_ASYNC_FIFO_OVF_INT_ENA_MASK);
    }

    /// **CSI_BUF_OVERRUN_INT_ENA** `[3]` (RW)
    ///
    /// buffer overrun interrupt enable.
    #[inline(always)]
    pub fn csi_buf_overrun_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CSI_BUF_OVERRUN_INT_ENA_MASK) != 0
    }

    /// Set CSI_BUF_OVERRUN_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_csi_buf_overrun_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CSI_BUF_OVERRUN_INT_ENA_MASK);
    }

    /// Clear CSI_BUF_OVERRUN_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_csi_buf_overrun_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CSI_BUF_OVERRUN_INT_ENA_MASK);
    }

    /// **DISCARD_INT_ENA** `[2]` (RW)
    ///
    /// an incomplete frame of data was sent interrupt enable.
    #[inline(always)]
    pub fn discard_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DISCARD_INT_ENA_MASK) != 0
    }

    /// Set DISCARD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_discard_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::DISCARD_INT_ENA_MASK);
    }

    /// Clear DISCARD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_discard_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::DISCARD_INT_ENA_MASK);
    }

    /// **VADR_NUM_LT_INT_ENA** `[1]` (RW)
    ///
    /// reg_vadr_num is less than real interrupt enable.
    #[inline(always)]
    pub fn vadr_num_lt_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::VADR_NUM_LT_INT_ENA_MASK) != 0
    }

    /// Set VADR_NUM_LT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vadr_num_lt_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::VADR_NUM_LT_INT_ENA_MASK);
    }

    /// Clear VADR_NUM_LT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vadr_num_lt_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::VADR_NUM_LT_INT_ENA_MASK);
    }

    /// **VADR_NUM_GT_INT_ENA** `[0]` (RW)
    ///
    /// reg_vadr_num is greater than real interrupt enable.
    #[inline(always)]
    pub fn vadr_num_gt_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::VADR_NUM_GT_INT_ENA_MASK) != 0
    }

    /// Set VADR_NUM_GT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vadr_num_gt_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::VADR_NUM_GT_INT_ENA_MASK);
    }

    /// Clear VADR_NUM_GT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vadr_num_gt_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::VADR_NUM_GT_INT_ENA_MASK);
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

    /// Read **DMA_CFG_HAS_UPDATED_INT_ENA** `[5:5]`
    ///
    /// dma configuration update complete interrupt enable.
    #[inline(always)]
    pub const fn dma_cfg_has_updated_int_ena(&self) -> bool {
        (self.bits & IntEna::DMA_CFG_HAS_UPDATED_INT_ENA_MASK) != 0
    }

    /// Read **CSI_ASYNC_FIFO_OVF_INT_ENA** `[4:4]`
    ///
    /// buffer overflow interrupt enable.
    #[inline(always)]
    pub const fn csi_async_fifo_ovf_int_ena(&self) -> bool {
        (self.bits & IntEna::CSI_ASYNC_FIFO_OVF_INT_ENA_MASK) != 0
    }

    /// Read **CSI_BUF_OVERRUN_INT_ENA** `[3:3]`
    ///
    /// buffer overrun interrupt enable.
    #[inline(always)]
    pub const fn csi_buf_overrun_int_ena(&self) -> bool {
        (self.bits & IntEna::CSI_BUF_OVERRUN_INT_ENA_MASK) != 0
    }

    /// Read **DISCARD_INT_ENA** `[2:2]`
    ///
    /// an incomplete frame of data was sent interrupt enable.
    #[inline(always)]
    pub const fn discard_int_ena(&self) -> bool {
        (self.bits & IntEna::DISCARD_INT_ENA_MASK) != 0
    }

    /// Read **VADR_NUM_LT_INT_ENA** `[1:1]`
    ///
    /// reg_vadr_num is less than real interrupt enable.
    #[inline(always)]
    pub const fn vadr_num_lt_int_ena(&self) -> bool {
        (self.bits & IntEna::VADR_NUM_LT_INT_ENA_MASK) != 0
    }

    /// Read **VADR_NUM_GT_INT_ENA** `[0:0]`
    ///
    /// reg_vadr_num is greater than real interrupt enable.
    #[inline(always)]
    pub const fn vadr_num_gt_int_ena(&self) -> bool {
        (self.bits & IntEna::VADR_NUM_GT_INT_ENA_MASK) != 0
    }

    /// Set **DMA_CFG_HAS_UPDATED_INT_ENA** `[5:5]`
    ///
    /// dma configuration update complete interrupt enable.
    #[inline(always)]
    pub const fn with_dma_cfg_has_updated_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DMA_CFG_HAS_UPDATED_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DMA_CFG_HAS_UPDATED_INT_ENA_MASK
            },
        }
    }

    /// Set **CSI_ASYNC_FIFO_OVF_INT_ENA** `[4:4]`
    ///
    /// buffer overflow interrupt enable.
    #[inline(always)]
    pub const fn with_csi_async_fifo_ovf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CSI_ASYNC_FIFO_OVF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CSI_ASYNC_FIFO_OVF_INT_ENA_MASK
            },
        }
    }

    /// Set **CSI_BUF_OVERRUN_INT_ENA** `[3:3]`
    ///
    /// buffer overrun interrupt enable.
    #[inline(always)]
    pub const fn with_csi_buf_overrun_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CSI_BUF_OVERRUN_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CSI_BUF_OVERRUN_INT_ENA_MASK
            },
        }
    }

    /// Set **DISCARD_INT_ENA** `[2:2]`
    ///
    /// an incomplete frame of data was sent interrupt enable.
    #[inline(always)]
    pub const fn with_discard_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DISCARD_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DISCARD_INT_ENA_MASK
            },
        }
    }

    /// Set **VADR_NUM_LT_INT_ENA** `[1:1]`
    ///
    /// reg_vadr_num is less than real interrupt enable.
    #[inline(always)]
    pub const fn with_vadr_num_lt_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::VADR_NUM_LT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::VADR_NUM_LT_INT_ENA_MASK
            },
        }
    }

    /// Set **VADR_NUM_GT_INT_ENA** `[0:0]`
    ///
    /// reg_vadr_num is greater than real interrupt enable.
    #[inline(always)]
    pub const fn with_vadr_num_gt_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::VADR_NUM_GT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::VADR_NUM_GT_INT_ENA_MASK
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

/// DMA_REQ_INTERVAL Register
///
/// DMA interval configuration.
///
/// Address offset: 0x002c
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **DMA_REQ_INTERVAL** `[15:0]` (RW)
///
///   16'b1: 1 cycle. 16'b11: 2 cycle. ... ... 16'hFFFF: 16 cycle.
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
    /// 16'b1: 1 cycle. 16'b11: 2 cycle. ... ... 16'hFFFF: 16 cycle.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const DMA_REQ_INTERVAL_OFFSET: u32 = 0;
    pub const DMA_REQ_INTERVAL_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **DMA_REQ_INTERVAL** `[15:0]` (RW)
    ///
    /// 16'b1: 1 cycle. 16'b11: 2 cycle. ... ... 16'hFFFF: 16 cycle.
    #[inline(always)]
    pub fn dma_req_interval(&self) -> u32 {
        (self.value.read_bits() & Self::DMA_REQ_INTERVAL_MASK) >> Self::DMA_REQ_INTERVAL_OFFSET
    }

    /// Write **DMA_REQ_INTERVAL** `[15:0]` (RW)
    ///
    /// 16'b1: 1 cycle. 16'b11: 2 cycle. ... ... 16'hFFFF: 16 cycle.
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
    /// 16'b1: 1 cycle. 16'b11: 2 cycle. ... ... 16'hFFFF: 16 cycle.
    #[inline(always)]
    pub const fn dma_req_interval(&self) -> u32 {
        (self.bits & DmaReqInterval::DMA_REQ_INTERVAL_MASK)
            >> DmaReqInterval::DMA_REQ_INTERVAL_OFFSET
    }

    /// Set **DMA_REQ_INTERVAL** `[15:0]`
    ///
    /// 16'b1: 1 cycle. 16'b11: 2 cycle. ... ... 16'hFFFF: 16 cycle.
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

/// DMABLK_SIZE Register
///
/// DMA block size configuration.
///
/// Address offset: 0x0030
/// Reset value: 0x00001fff
///
/// ## Fields
///
/// - **DMABLK_SIZE** `[12:0]` (RW)
///
///   the number of reg_dma_burst_len in a block
#[repr(transparent)]
pub struct DmablkSize {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmablkSize {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmablkSize {
    // ======== Field Constants ========

    /// **DMABLK_SIZE**
    ///
    /// the number of reg_dma_burst_len in a block
    ///
    /// Bits `[12:0]`, mask `0x00001fff`
    pub const DMABLK_SIZE_OFFSET: u32 = 0;
    pub const DMABLK_SIZE_MASK: u32 = 0x00001fff;

    // ======== Field Accessors ========

    /// **DMABLK_SIZE** `[12:0]` (RW)
    ///
    /// the number of reg_dma_burst_len in a block
    #[inline(always)]
    pub fn dmablk_size(&self) -> u32 {
        (self.value.read_bits() & Self::DMABLK_SIZE_MASK) >> Self::DMABLK_SIZE_OFFSET
    }

    /// Write **DMABLK_SIZE** `[12:0]` (RW)
    ///
    /// the number of reg_dma_burst_len in a block
    #[inline(always)]
    pub fn write_dmablk_size(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DMABLK_SIZE_MASK)
                | ((value << Self::DMABLK_SIZE_OFFSET) & Self::DMABLK_SIZE_MASK)
        });
    }
}

/// DMABLK_SIZE value type for batch read/modify/write operations
///
/// Use `dmablksize.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmablksize.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmablkSizeVal {
    bits: u32,
}

impl DmablkSizeVal {
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
        Self { bits: 0x00001fff }
    }

    /// Read **DMABLK_SIZE** `[12:0]`
    ///
    /// the number of reg_dma_burst_len in a block
    #[inline(always)]
    pub const fn dmablk_size(&self) -> u32 {
        (self.bits & DmablkSize::DMABLK_SIZE_MASK) >> DmablkSize::DMABLK_SIZE_OFFSET
    }

    /// Set **DMABLK_SIZE** `[12:0]`
    ///
    /// the number of reg_dma_burst_len in a block
    #[inline(always)]
    pub const fn with_dmablk_size(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DmablkSize::DMABLK_SIZE_MASK)
                | ((value << DmablkSize::DMABLK_SIZE_OFFSET) & DmablkSize::DMABLK_SIZE_MASK),
        }
    }
}

impl Default for DmablkSizeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DmablkSize {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmablkSizeVal {
        DmablkSizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmablkSizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmablkSizeVal) -> DmablkSizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RDN_ECO_CS Register
///
/// N/A
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **RDN_ECO_RESULT** `[1]` (RO)
///
///   N/A
///
/// - **RDN_ECO_EN** `[0]` (RW)
///
///   N/A
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
    /// N/A
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RDN_ECO_RESULT_OFFSET: u32 = 1;
    pub const RDN_ECO_RESULT_MASK: u32 = 0x00000002;
    pub const RDN_ECO_RESULT: u32 = 1 << 1;

    /// **RDN_ECO_EN**
    ///
    /// N/A
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RDN_ECO_EN_OFFSET: u32 = 0;
    pub const RDN_ECO_EN_MASK: u32 = 0x00000001;
    pub const RDN_ECO_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RDN_ECO_RESULT** `[1]` (RO)
    ///
    /// N/A
    #[inline(always)]
    pub fn rdn_eco_result(&self) -> bool {
        (self.value.read_bits() & Self::RDN_ECO_RESULT_MASK) != 0
    }

    /// **RDN_ECO_EN** `[0]` (RW)
    ///
    /// N/A
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
    /// N/A
    #[inline(always)]
    pub const fn rdn_eco_result(&self) -> bool {
        (self.bits & RdnEcoCs::RDN_ECO_RESULT_MASK) != 0
    }

    /// Read **RDN_ECO_EN** `[0:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn rdn_eco_en(&self) -> bool {
        (self.bits & RdnEcoCs::RDN_ECO_EN_MASK) != 0
    }

    /// Set **RDN_ECO_EN** `[0:0]`
    ///
    /// N/A
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
/// N/A
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **RDN_ECO_LOW** `[31:0]` (RW)
///
///   N/A
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
    /// N/A
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RDN_ECO_LOW_OFFSET: u32 = 0;
    pub const RDN_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn rdn_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::RDN_ECO_LOW_MASK) >> Self::RDN_ECO_LOW_OFFSET
    }

    /// Write **RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// N/A
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
    /// N/A
    #[inline(always)]
    pub const fn rdn_eco_low(&self) -> u32 {
        (self.bits & RdnEcoLow::RDN_ECO_LOW_MASK) >> RdnEcoLow::RDN_ECO_LOW_OFFSET
    }

    /// Set **RDN_ECO_LOW** `[31:0]`
    ///
    /// N/A
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
/// N/A
///
/// Address offset: 0x003c
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **RDN_ECO_HIGH** `[31:0]` (RW)
///
///   N/A
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
    /// N/A
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RDN_ECO_HIGH_OFFSET: u32 = 0;
    pub const RDN_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn rdn_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::RDN_ECO_HIGH_MASK) >> Self::RDN_ECO_HIGH_OFFSET
    }

    /// Write **RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// N/A
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
    /// N/A
    #[inline(always)]
    pub const fn rdn_eco_high(&self) -> u32 {
        (self.bits & RdnEcoHigh::RDN_ECO_HIGH_MASK) >> RdnEcoHigh::RDN_ECO_HIGH_OFFSET
    }

    /// Set **RDN_ECO_HIGH** `[31:0]`
    ///
    /// N/A
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
/// csi host control by csi bridge.
///
/// Address offset: 0x0040
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **LOOPBK_TEST_EN** `[2]` (RW)
///
///   for phy test by loopback dsi phy to csi phy.
///
/// - **CSI_CFG_CLK_EN** `[1]` (RW)
///
///   enable cfg_clk of csi host module.
///
/// - **CSI_ENABLECLK** `[0]` (RW)
///
///   enable clock lane module of csi phy.
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

    /// **LOOPBK_TEST_EN**
    ///
    /// for phy test by loopback dsi phy to csi phy.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const LOOPBK_TEST_EN_OFFSET: u32 = 2;
    pub const LOOPBK_TEST_EN_MASK: u32 = 0x00000004;
    pub const LOOPBK_TEST_EN: u32 = 1 << 2;

    /// **CSI_CFG_CLK_EN**
    ///
    /// enable cfg_clk of csi host module.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CSI_CFG_CLK_EN_OFFSET: u32 = 1;
    pub const CSI_CFG_CLK_EN_MASK: u32 = 0x00000002;
    pub const CSI_CFG_CLK_EN: u32 = 1 << 1;

    /// **CSI_ENABLECLK**
    ///
    /// enable clock lane module of csi phy.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CSI_ENABLECLK_OFFSET: u32 = 0;
    pub const CSI_ENABLECLK_MASK: u32 = 0x00000001;
    pub const CSI_ENABLECLK: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LOOPBK_TEST_EN** `[2]` (RW)
    ///
    /// for phy test by loopback dsi phy to csi phy.
    #[inline(always)]
    pub fn loopbk_test_en(&self) -> bool {
        (self.value.read_bits() & Self::LOOPBK_TEST_EN_MASK) != 0
    }

    /// Set LOOPBK_TEST_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_loopbk_test_en(&self) {
        self.value.modify_bits(|r| r | Self::LOOPBK_TEST_EN_MASK);
    }

    /// Clear LOOPBK_TEST_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_loopbk_test_en(&self) {
        self.value.modify_bits(|r| r & !Self::LOOPBK_TEST_EN_MASK);
    }

    /// **CSI_CFG_CLK_EN** `[1]` (RW)
    ///
    /// enable cfg_clk of csi host module.
    #[inline(always)]
    pub fn csi_cfg_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::CSI_CFG_CLK_EN_MASK) != 0
    }

    /// Set CSI_CFG_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_csi_cfg_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::CSI_CFG_CLK_EN_MASK);
    }

    /// Clear CSI_CFG_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_csi_cfg_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::CSI_CFG_CLK_EN_MASK);
    }

    /// **CSI_ENABLECLK** `[0]` (RW)
    ///
    /// enable clock lane module of csi phy.
    #[inline(always)]
    pub fn csi_enableclk(&self) -> bool {
        (self.value.read_bits() & Self::CSI_ENABLECLK_MASK) != 0
    }

    /// Set CSI_ENABLECLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_csi_enableclk(&self) {
        self.value.modify_bits(|r| r | Self::CSI_ENABLECLK_MASK);
    }

    /// Clear CSI_ENABLECLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_csi_enableclk(&self) {
        self.value.modify_bits(|r| r & !Self::CSI_ENABLECLK_MASK);
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
        Self { bits: 0x00000003 }
    }

    /// Read **LOOPBK_TEST_EN** `[2:2]`
    ///
    /// for phy test by loopback dsi phy to csi phy.
    #[inline(always)]
    pub const fn loopbk_test_en(&self) -> bool {
        (self.bits & HostCtrl::LOOPBK_TEST_EN_MASK) != 0
    }

    /// Read **CSI_CFG_CLK_EN** `[1:1]`
    ///
    /// enable cfg_clk of csi host module.
    #[inline(always)]
    pub const fn csi_cfg_clk_en(&self) -> bool {
        (self.bits & HostCtrl::CSI_CFG_CLK_EN_MASK) != 0
    }

    /// Read **CSI_ENABLECLK** `[0:0]`
    ///
    /// enable clock lane module of csi phy.
    #[inline(always)]
    pub const fn csi_enableclk(&self) -> bool {
        (self.bits & HostCtrl::CSI_ENABLECLK_MASK) != 0
    }

    /// Set **LOOPBK_TEST_EN** `[2:2]`
    ///
    /// for phy test by loopback dsi phy to csi phy.
    #[inline(always)]
    pub const fn with_loopbk_test_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HostCtrl::LOOPBK_TEST_EN_MASK
            } else {
                self.bits & !HostCtrl::LOOPBK_TEST_EN_MASK
            },
        }
    }

    /// Set **CSI_CFG_CLK_EN** `[1:1]`
    ///
    /// enable cfg_clk of csi host module.
    #[inline(always)]
    pub const fn with_csi_cfg_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HostCtrl::CSI_CFG_CLK_EN_MASK
            } else {
                self.bits & !HostCtrl::CSI_CFG_CLK_EN_MASK
            },
        }
    }

    /// Set **CSI_ENABLECLK** `[0:0]`
    ///
    /// enable clock lane module of csi phy.
    #[inline(always)]
    pub const fn with_csi_enableclk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HostCtrl::CSI_ENABLECLK_MASK
            } else {
                self.bits & !HostCtrl::CSI_ENABLECLK_MASK
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

/// MEM_CTRL Register
///
/// csi bridge buffer control.
///
/// Address offset: 0x0044
/// Reset value: 0x00002640
///
/// ## Fields
///
/// - **CSI_MEM_AUX_CTRL** `[14:1]` (RW)
///
///   N/A
///
/// - **CSI_BRIDGE_MEM_CLK_FORCE_ON** `[0]` (RW)
///
///   csi bridge memory clock gating force on.
#[repr(transparent)]
pub struct MemCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MemCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MemCtrl {
    // ======== Field Constants ========

    /// **CSI_MEM_AUX_CTRL**
    ///
    /// N/A
    ///
    /// Bits `[14:1]`, mask `0x00007ffe`
    pub const CSI_MEM_AUX_CTRL_OFFSET: u32 = 1;
    pub const CSI_MEM_AUX_CTRL_MASK: u32 = 0x00007ffe;

    /// **CSI_BRIDGE_MEM_CLK_FORCE_ON**
    ///
    /// csi bridge memory clock gating force on.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CSI_BRIDGE_MEM_CLK_FORCE_ON_OFFSET: u32 = 0;
    pub const CSI_BRIDGE_MEM_CLK_FORCE_ON_MASK: u32 = 0x00000001;
    pub const CSI_BRIDGE_MEM_CLK_FORCE_ON: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CSI_MEM_AUX_CTRL** `[14:1]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn csi_mem_aux_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::CSI_MEM_AUX_CTRL_MASK) >> Self::CSI_MEM_AUX_CTRL_OFFSET
    }

    /// Write **CSI_MEM_AUX_CTRL** `[14:1]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_csi_mem_aux_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CSI_MEM_AUX_CTRL_MASK)
                | ((value << Self::CSI_MEM_AUX_CTRL_OFFSET) & Self::CSI_MEM_AUX_CTRL_MASK)
        });
    }

    /// **CSI_BRIDGE_MEM_CLK_FORCE_ON** `[0]` (RW)
    ///
    /// csi bridge memory clock gating force on.
    #[inline(always)]
    pub fn csi_bridge_mem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::CSI_BRIDGE_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set CSI_BRIDGE_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_csi_bridge_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::CSI_BRIDGE_MEM_CLK_FORCE_ON_MASK);
    }

    /// Clear CSI_BRIDGE_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_csi_bridge_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::CSI_BRIDGE_MEM_CLK_FORCE_ON_MASK);
    }
}

/// MEM_CTRL value type for batch read/modify/write operations
///
/// Use `memctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `memctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MemCtrlVal {
    bits: u32,
}

impl MemCtrlVal {
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
        Self { bits: 0x00002640 }
    }

    /// Read **CSI_MEM_AUX_CTRL** `[14:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn csi_mem_aux_ctrl(&self) -> u32 {
        (self.bits & MemCtrl::CSI_MEM_AUX_CTRL_MASK) >> MemCtrl::CSI_MEM_AUX_CTRL_OFFSET
    }

    /// Read **CSI_BRIDGE_MEM_CLK_FORCE_ON** `[0:0]`
    ///
    /// csi bridge memory clock gating force on.
    #[inline(always)]
    pub const fn csi_bridge_mem_clk_force_on(&self) -> bool {
        (self.bits & MemCtrl::CSI_BRIDGE_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set **CSI_MEM_AUX_CTRL** `[14:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_csi_mem_aux_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !MemCtrl::CSI_MEM_AUX_CTRL_MASK)
                | ((value << MemCtrl::CSI_MEM_AUX_CTRL_OFFSET) & MemCtrl::CSI_MEM_AUX_CTRL_MASK),
        }
    }

    /// Set **CSI_BRIDGE_MEM_CLK_FORCE_ON** `[0:0]`
    ///
    /// csi bridge memory clock gating force on.
    #[inline(always)]
    pub const fn with_csi_bridge_mem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MemCtrl::CSI_BRIDGE_MEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !MemCtrl::CSI_BRIDGE_MEM_CLK_FORCE_ON_MASK
            },
        }
    }
}

impl Default for MemCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl MemCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MemCtrlVal {
        MemCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MemCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MemCtrlVal) -> MemCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// csi bridge register mapping unit clock gating.
    pub clk_en: ClkEn,
    /// 0x0004
    /// csi bridge enable.
    pub csi_en: CsiEn,
    /// 0x0008
    /// dma request configuration.
    pub dma_req_cfg: DmaReqCfg,
    /// 0x000c
    /// csi bridge buffer control.
    pub buf_flow_ctl: BufFlowCtl,
    /// 0x0010
    /// pixel data type configuration.
    pub data_type_cfg: DataTypeCfg,
    /// 0x0014
    /// frame configuration.
    pub frame_cfg: FrameCfg,
    /// 0x0018
    /// data endianness order configuration.
    pub endian_mode: EndianMode,
    /// 0x001c
    /// csi bridge interrupt raw.
    pub int_raw: IntRaw,
    /// 0x0020
    /// csi bridge interrupt clr.
    pub int_clr: IntClr,
    /// 0x0024
    /// csi bridge interrupt st.
    pub int_st: IntSt,
    /// 0x0028
    /// csi bridge interrupt enable.
    pub int_ena: IntEna,
    /// 0x002c
    /// DMA interval configuration.
    pub dma_req_interval: DmaReqInterval,
    /// 0x0030
    /// DMA block size configuration.
    pub dmablk_size: DmablkSize,
    /// 0x0034
    /// N/A
    pub rdn_eco_cs: RdnEcoCs,
    /// 0x0038
    /// N/A
    pub rdn_eco_low: RdnEcoLow,
    /// 0x003c
    /// N/A
    pub rdn_eco_high: RdnEcoHigh,
    /// 0x0040
    /// csi host control by csi bridge.
    pub host_ctrl: HostCtrl,
    /// 0x0044
    /// csi bridge buffer control.
    pub mem_ctrl: MemCtrl,
}
