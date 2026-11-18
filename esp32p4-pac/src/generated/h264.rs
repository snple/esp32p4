//! H264
//! H264 Encoder (Core)

#![allow(dead_code)]

pub const BASE: u32 = 0x50084000;

/// SYS_CTRL Register
///
/// H264 system level control register.
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **SYS_RST_PULSE** `[3]` (WO)
///
///   Configures whether or not to reset H264 ip.\\0: Invalid. No effect\\1: Reset H264 ip
///
/// - **FRAME_MODE** `[2]` (RW)
///
///   Configures H264 running mode. When field H264_DUAL_STREAM_MODE is set to 1, this field must be set to 1 too.\\0: GOP mode. Before every GOP first frame start, need reconfig reference frame DMA\\1: Frame mode. Before every frame start, need reconfig reference frame DMA
///
/// - **DMA_MOVE_START** `[1]` (WO)
///
///   Configures whether or not to start moving reference data from external mem.\\0: Invalid. No effect\\1: H264 start moving two MB lines of reference frame from external mem to internal mem
///
/// - **FRAME_START** `[0]` (WO)
///
///   Configures whether or not to start encoding one frame.\\0: Invalid. No effect\\1: Start encoding one frame
#[repr(transparent)]
pub struct SysCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SysCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SysCtrl {
    // ======== Field Constants ========

    /// **SYS_RST_PULSE**
    ///
    /// Configures whether or not to reset H264 ip.\\0: Invalid. No effect\\1: Reset H264 ip
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SYS_RST_PULSE_OFFSET: u32 = 3;
    pub const SYS_RST_PULSE_MASK: u32 = 0x00000008;
    pub const SYS_RST_PULSE: u32 = 1 << 3;

    /// **FRAME_MODE**
    ///
    /// Configures H264 running mode. When field H264_DUAL_STREAM_MODE is set to 1, this field must be set to 1 too.\\0: GOP mode. Before every GOP first frame start, need reconfig reference frame DMA\\1: Frame mode. Before every frame start, need reconfig reference frame DMA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FRAME_MODE_OFFSET: u32 = 2;
    pub const FRAME_MODE_MASK: u32 = 0x00000004;
    pub const FRAME_MODE: u32 = 1 << 2;

    /// **DMA_MOVE_START**
    ///
    /// Configures whether or not to start moving reference data from external mem.\\0: Invalid. No effect\\1: H264 start moving two MB lines of reference frame from external mem to internal mem
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DMA_MOVE_START_OFFSET: u32 = 1;
    pub const DMA_MOVE_START_MASK: u32 = 0x00000002;
    pub const DMA_MOVE_START: u32 = 1 << 1;

    /// **FRAME_START**
    ///
    /// Configures whether or not to start encoding one frame.\\0: Invalid. No effect\\1: Start encoding one frame
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FRAME_START_OFFSET: u32 = 0;
    pub const FRAME_START_MASK: u32 = 0x00000001;
    pub const FRAME_START: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SYS_RST_PULSE** (WO)
    ///
    /// Configures whether or not to reset H264 ip.\\0: Invalid. No effect\\1: Reset H264 ip
    #[inline(always)]
    pub fn trigger_sys_rst_pulse(&self) {
        self.value.modify_bits(|r| r | Self::SYS_RST_PULSE_MASK);
    }

    /// **FRAME_MODE** `[2]` (RW)
    ///
    /// Configures H264 running mode. When field H264_DUAL_STREAM_MODE is set to 1, this field must be set to 1 too.\\0: GOP mode. Before every GOP first frame start, need reconfig reference frame DMA\\1: Frame mode. Before every frame start, need reconfig reference frame DMA
    #[inline(always)]
    pub fn frame_mode(&self) -> bool {
        (self.value.read_bits() & Self::FRAME_MODE_MASK) != 0
    }

    /// Set FRAME_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_frame_mode(&self) {
        self.value.modify_bits(|r| r | Self::FRAME_MODE_MASK);
    }

    /// Clear FRAME_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_frame_mode(&self) {
        self.value.modify_bits(|r| r & !Self::FRAME_MODE_MASK);
    }

    /// Trigger **DMA_MOVE_START** (WO)
    ///
    /// Configures whether or not to start moving reference data from external mem.\\0: Invalid. No effect\\1: H264 start moving two MB lines of reference frame from external mem to internal mem
    #[inline(always)]
    pub fn trigger_dma_move_start(&self) {
        self.value.modify_bits(|r| r | Self::DMA_MOVE_START_MASK);
    }

    /// Trigger **FRAME_START** (WO)
    ///
    /// Configures whether or not to start encoding one frame.\\0: Invalid. No effect\\1: Start encoding one frame
    #[inline(always)]
    pub fn trigger_frame_start(&self) {
        self.value.modify_bits(|r| r | Self::FRAME_START_MASK);
    }
}

/// SYS_CTRL value type for batch read/modify/write operations
///
/// Use `sysctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sysctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SysCtrlVal {
    bits: u32,
}

impl SysCtrlVal {
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

    /// Read **FRAME_MODE** `[2:2]`
    ///
    /// Configures H264 running mode. When field H264_DUAL_STREAM_MODE is set to 1, this field must be set to 1 too.\\0: GOP mode. Before every GOP first frame start, need reconfig reference frame DMA\\1: Frame mode. Before every frame start, need reconfig reference frame DMA
    #[inline(always)]
    pub const fn frame_mode(&self) -> bool {
        (self.bits & SysCtrl::FRAME_MODE_MASK) != 0
    }

    /// Set **SYS_RST_PULSE** `[3:3]`
    ///
    /// Configures whether or not to reset H264 ip.\\0: Invalid. No effect\\1: Reset H264 ip
    #[inline(always)]
    pub const fn with_sys_rst_pulse(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysCtrl::SYS_RST_PULSE_MASK
            } else {
                self.bits & !SysCtrl::SYS_RST_PULSE_MASK
            },
        }
    }

    /// Set **FRAME_MODE** `[2:2]`
    ///
    /// Configures H264 running mode. When field H264_DUAL_STREAM_MODE is set to 1, this field must be set to 1 too.\\0: GOP mode. Before every GOP first frame start, need reconfig reference frame DMA\\1: Frame mode. Before every frame start, need reconfig reference frame DMA
    #[inline(always)]
    pub const fn with_frame_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysCtrl::FRAME_MODE_MASK
            } else {
                self.bits & !SysCtrl::FRAME_MODE_MASK
            },
        }
    }

    /// Set **DMA_MOVE_START** `[1:1]`
    ///
    /// Configures whether or not to start moving reference data from external mem.\\0: Invalid. No effect\\1: H264 start moving two MB lines of reference frame from external mem to internal mem
    #[inline(always)]
    pub const fn with_dma_move_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysCtrl::DMA_MOVE_START_MASK
            } else {
                self.bits & !SysCtrl::DMA_MOVE_START_MASK
            },
        }
    }

    /// Set **FRAME_START** `[0:0]`
    ///
    /// Configures whether or not to start encoding one frame.\\0: Invalid. No effect\\1: Start encoding one frame
    #[inline(always)]
    pub const fn with_frame_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysCtrl::FRAME_START_MASK
            } else {
                self.bits & !SysCtrl::FRAME_START_MASK
            },
        }
    }
}

impl Default for SysCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SysCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SysCtrlVal {
        SysCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SysCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SysCtrlVal) -> SysCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GOP_CONF Register
///
/// GOP related configuration register.
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **GOP_NUM** `[8:1]` (RW)
///
///   Configures the frame number of one GOP.\\0: The frame number of one GOP is infinite\\Others: Actual frame number of one GOP
///
/// - **DUAL_STREAM_MODE** `[0]` (RW)
///
///   Configures whether or not to enable dual stream mode. When this field is set to 1, H264_FRAME_MODE field must be set to 1 too.\\0: Normal mode\\1: Dual stream mode
#[repr(transparent)]
pub struct GopConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GopConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GopConf {
    // ======== Field Constants ========

    /// **GOP_NUM**
    ///
    /// Configures the frame number of one GOP.\\0: The frame number of one GOP is infinite\\Others: Actual frame number of one GOP
    ///
    /// Bits `[8:1]`, mask `0x000001fe`
    pub const GOP_NUM_OFFSET: u32 = 1;
    pub const GOP_NUM_MASK: u32 = 0x000001fe;

    /// **DUAL_STREAM_MODE**
    ///
    /// Configures whether or not to enable dual stream mode. When this field is set to 1, H264_FRAME_MODE field must be set to 1 too.\\0: Normal mode\\1: Dual stream mode
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DUAL_STREAM_MODE_OFFSET: u32 = 0;
    pub const DUAL_STREAM_MODE_MASK: u32 = 0x00000001;
    pub const DUAL_STREAM_MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **GOP_NUM** `[8:1]` (RW)
    ///
    /// Configures the frame number of one GOP.\\0: The frame number of one GOP is infinite\\Others: Actual frame number of one GOP
    #[inline(always)]
    pub fn gop_num(&self) -> u32 {
        (self.value.read_bits() & Self::GOP_NUM_MASK) >> Self::GOP_NUM_OFFSET
    }

    /// Write **GOP_NUM** `[8:1]` (RW)
    ///
    /// Configures the frame number of one GOP.\\0: The frame number of one GOP is infinite\\Others: Actual frame number of one GOP
    #[inline(always)]
    pub fn write_gop_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::GOP_NUM_MASK) | ((value << Self::GOP_NUM_OFFSET) & Self::GOP_NUM_MASK)
        });
    }

    /// **DUAL_STREAM_MODE** `[0]` (RW)
    ///
    /// Configures whether or not to enable dual stream mode. When this field is set to 1, H264_FRAME_MODE field must be set to 1 too.\\0: Normal mode\\1: Dual stream mode
    #[inline(always)]
    pub fn dual_stream_mode(&self) -> bool {
        (self.value.read_bits() & Self::DUAL_STREAM_MODE_MASK) != 0
    }

    /// Set DUAL_STREAM_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dual_stream_mode(&self) {
        self.value.modify_bits(|r| r | Self::DUAL_STREAM_MODE_MASK);
    }

    /// Clear DUAL_STREAM_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dual_stream_mode(&self) {
        self.value.modify_bits(|r| r & !Self::DUAL_STREAM_MODE_MASK);
    }
}

/// GOP_CONF value type for batch read/modify/write operations
///
/// Use `gopconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gopconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GopConfVal {
    bits: u32,
}

impl GopConfVal {
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

    /// Read **GOP_NUM** `[8:1]`
    ///
    /// Configures the frame number of one GOP.\\0: The frame number of one GOP is infinite\\Others: Actual frame number of one GOP
    #[inline(always)]
    pub const fn gop_num(&self) -> u32 {
        (self.bits & GopConf::GOP_NUM_MASK) >> GopConf::GOP_NUM_OFFSET
    }

    /// Read **DUAL_STREAM_MODE** `[0:0]`
    ///
    /// Configures whether or not to enable dual stream mode. When this field is set to 1, H264_FRAME_MODE field must be set to 1 too.\\0: Normal mode\\1: Dual stream mode
    #[inline(always)]
    pub const fn dual_stream_mode(&self) -> bool {
        (self.bits & GopConf::DUAL_STREAM_MODE_MASK) != 0
    }

    /// Set **GOP_NUM** `[8:1]`
    ///
    /// Configures the frame number of one GOP.\\0: The frame number of one GOP is infinite\\Others: Actual frame number of one GOP
    #[inline(always)]
    pub const fn with_gop_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GopConf::GOP_NUM_MASK)
                | ((value << GopConf::GOP_NUM_OFFSET) & GopConf::GOP_NUM_MASK),
        }
    }

    /// Set **DUAL_STREAM_MODE** `[0:0]`
    ///
    /// Configures whether or not to enable dual stream mode. When this field is set to 1, H264_FRAME_MODE field must be set to 1 too.\\0: Normal mode\\1: Dual stream mode
    #[inline(always)]
    pub const fn with_dual_stream_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GopConf::DUAL_STREAM_MODE_MASK
            } else {
                self.bits & !GopConf::DUAL_STREAM_MODE_MASK
            },
        }
    }
}

impl Default for GopConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GopConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GopConfVal {
        GopConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GopConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GopConfVal) -> GopConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_SYS_MB_RES Register
///
/// Video A horizontal and vertical MB resolution register.
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **A_SYS_TOTAL_MB_X** `[13:7]` (RW)
///
///   Configures video A horizontal MB resolution.
///
/// - **A_SYS_TOTAL_MB_Y** `[6:0]` (RW)
///
///   Configures video A vertical MB resolution.
#[repr(transparent)]
pub struct ASysMbRes {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ASysMbRes {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ASysMbRes {
    // ======== Field Constants ========

    /// **A_SYS_TOTAL_MB_X**
    ///
    /// Configures video A horizontal MB resolution.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const A_SYS_TOTAL_MB_X_OFFSET: u32 = 7;
    pub const A_SYS_TOTAL_MB_X_MASK: u32 = 0x00003f80;

    /// **A_SYS_TOTAL_MB_Y**
    ///
    /// Configures video A vertical MB resolution.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const A_SYS_TOTAL_MB_Y_OFFSET: u32 = 0;
    pub const A_SYS_TOTAL_MB_Y_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **A_SYS_TOTAL_MB_X** `[13:7]` (RW)
    ///
    /// Configures video A horizontal MB resolution.
    #[inline(always)]
    pub fn a_sys_total_mb_x(&self) -> u32 {
        (self.value.read_bits() & Self::A_SYS_TOTAL_MB_X_MASK) >> Self::A_SYS_TOTAL_MB_X_OFFSET
    }

    /// Write **A_SYS_TOTAL_MB_X** `[13:7]` (RW)
    ///
    /// Configures video A horizontal MB resolution.
    #[inline(always)]
    pub fn write_a_sys_total_mb_x(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_SYS_TOTAL_MB_X_MASK)
                | ((value << Self::A_SYS_TOTAL_MB_X_OFFSET) & Self::A_SYS_TOTAL_MB_X_MASK)
        });
    }

    /// **A_SYS_TOTAL_MB_Y** `[6:0]` (RW)
    ///
    /// Configures video A vertical MB resolution.
    #[inline(always)]
    pub fn a_sys_total_mb_y(&self) -> u32 {
        (self.value.read_bits() & Self::A_SYS_TOTAL_MB_Y_MASK) >> Self::A_SYS_TOTAL_MB_Y_OFFSET
    }

    /// Write **A_SYS_TOTAL_MB_Y** `[6:0]` (RW)
    ///
    /// Configures video A vertical MB resolution.
    #[inline(always)]
    pub fn write_a_sys_total_mb_y(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_SYS_TOTAL_MB_Y_MASK)
                | ((value << Self::A_SYS_TOTAL_MB_Y_OFFSET) & Self::A_SYS_TOTAL_MB_Y_MASK)
        });
    }
}

/// A_SYS_MB_RES value type for batch read/modify/write operations
///
/// Use `asysmbres.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `asysmbres.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ASysMbResVal {
    bits: u32,
}

impl ASysMbResVal {
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

    /// Read **A_SYS_TOTAL_MB_X** `[13:7]`
    ///
    /// Configures video A horizontal MB resolution.
    #[inline(always)]
    pub const fn a_sys_total_mb_x(&self) -> u32 {
        (self.bits & ASysMbRes::A_SYS_TOTAL_MB_X_MASK) >> ASysMbRes::A_SYS_TOTAL_MB_X_OFFSET
    }

    /// Read **A_SYS_TOTAL_MB_Y** `[6:0]`
    ///
    /// Configures video A vertical MB resolution.
    #[inline(always)]
    pub const fn a_sys_total_mb_y(&self) -> u32 {
        (self.bits & ASysMbRes::A_SYS_TOTAL_MB_Y_MASK) >> ASysMbRes::A_SYS_TOTAL_MB_Y_OFFSET
    }

    /// Set **A_SYS_TOTAL_MB_X** `[13:7]`
    ///
    /// Configures video A horizontal MB resolution.
    #[inline(always)]
    pub const fn with_a_sys_total_mb_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ASysMbRes::A_SYS_TOTAL_MB_X_MASK)
                | ((value << ASysMbRes::A_SYS_TOTAL_MB_X_OFFSET)
                    & ASysMbRes::A_SYS_TOTAL_MB_X_MASK),
        }
    }

    /// Set **A_SYS_TOTAL_MB_Y** `[6:0]`
    ///
    /// Configures video A vertical MB resolution.
    #[inline(always)]
    pub const fn with_a_sys_total_mb_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ASysMbRes::A_SYS_TOTAL_MB_Y_MASK)
                | ((value << ASysMbRes::A_SYS_TOTAL_MB_Y_OFFSET)
                    & ASysMbRes::A_SYS_TOTAL_MB_Y_MASK),
        }
    }
}

impl Default for ASysMbResVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ASysMbRes {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ASysMbResVal {
        ASysMbResVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ASysMbResVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ASysMbResVal) -> ASysMbResVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_SYS_CONF Register
///
/// Video A system level configuration register.
///
/// Address offset: 0x000c
/// Reset value: 0x00000203
///
/// ## Fields
///
/// - **A_INTRA_COST_CMP_OFFSET** `[29:14]` (RW)
///
///   Configures video A intra cost offset when I MB compared with P MB.
///
/// - **A_REC_READY_TRIGGER_MB_LINES** `[13:7]` (RW)
///
///   Configures when to trigger video A H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4.
///
/// - **A_DB_TMP_READY_TRIGGER_MB_NUM** `[6:0]` (RW)
///
///   Configures when to trigger video A H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3.
#[repr(transparent)]
pub struct ASysConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ASysConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ASysConf {
    // ======== Field Constants ========

    /// **A_INTRA_COST_CMP_OFFSET**
    ///
    /// Configures video A intra cost offset when I MB compared with P MB.
    ///
    /// Bits `[29:14]`, mask `0x3fffc000`
    pub const A_INTRA_COST_CMP_OFFSET_OFFSET: u32 = 14;
    pub const A_INTRA_COST_CMP_OFFSET_MASK: u32 = 0x3fffc000;

    /// **A_REC_READY_TRIGGER_MB_LINES**
    ///
    /// Configures when to trigger video A H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const A_REC_READY_TRIGGER_MB_LINES_OFFSET: u32 = 7;
    pub const A_REC_READY_TRIGGER_MB_LINES_MASK: u32 = 0x00003f80;

    /// **A_DB_TMP_READY_TRIGGER_MB_NUM**
    ///
    /// Configures when to trigger video A H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const A_DB_TMP_READY_TRIGGER_MB_NUM_OFFSET: u32 = 0;
    pub const A_DB_TMP_READY_TRIGGER_MB_NUM_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **A_INTRA_COST_CMP_OFFSET** `[29:14]` (RW)
    ///
    /// Configures video A intra cost offset when I MB compared with P MB.
    #[inline(always)]
    pub fn a_intra_cost_cmp_offset(&self) -> u32 {
        (self.value.read_bits() & Self::A_INTRA_COST_CMP_OFFSET_MASK)
            >> Self::A_INTRA_COST_CMP_OFFSET_OFFSET
    }

    /// Write **A_INTRA_COST_CMP_OFFSET** `[29:14]` (RW)
    ///
    /// Configures video A intra cost offset when I MB compared with P MB.
    #[inline(always)]
    pub fn write_a_intra_cost_cmp_offset(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_INTRA_COST_CMP_OFFSET_MASK)
                | ((value << Self::A_INTRA_COST_CMP_OFFSET_OFFSET)
                    & Self::A_INTRA_COST_CMP_OFFSET_MASK)
        });
    }

    /// **A_REC_READY_TRIGGER_MB_LINES** `[13:7]` (RW)
    ///
    /// Configures when to trigger video A H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4.
    #[inline(always)]
    pub fn a_rec_ready_trigger_mb_lines(&self) -> u32 {
        (self.value.read_bits() & Self::A_REC_READY_TRIGGER_MB_LINES_MASK)
            >> Self::A_REC_READY_TRIGGER_MB_LINES_OFFSET
    }

    /// Write **A_REC_READY_TRIGGER_MB_LINES** `[13:7]` (RW)
    ///
    /// Configures when to trigger video A H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4.
    #[inline(always)]
    pub fn write_a_rec_ready_trigger_mb_lines(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_REC_READY_TRIGGER_MB_LINES_MASK)
                | ((value << Self::A_REC_READY_TRIGGER_MB_LINES_OFFSET)
                    & Self::A_REC_READY_TRIGGER_MB_LINES_MASK)
        });
    }

    /// **A_DB_TMP_READY_TRIGGER_MB_NUM** `[6:0]` (RW)
    ///
    /// Configures when to trigger video A H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3.
    #[inline(always)]
    pub fn a_db_tmp_ready_trigger_mb_num(&self) -> u32 {
        (self.value.read_bits() & Self::A_DB_TMP_READY_TRIGGER_MB_NUM_MASK)
            >> Self::A_DB_TMP_READY_TRIGGER_MB_NUM_OFFSET
    }

    /// Write **A_DB_TMP_READY_TRIGGER_MB_NUM** `[6:0]` (RW)
    ///
    /// Configures when to trigger video A H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3.
    #[inline(always)]
    pub fn write_a_db_tmp_ready_trigger_mb_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_DB_TMP_READY_TRIGGER_MB_NUM_MASK)
                | ((value << Self::A_DB_TMP_READY_TRIGGER_MB_NUM_OFFSET)
                    & Self::A_DB_TMP_READY_TRIGGER_MB_NUM_MASK)
        });
    }
}

/// A_SYS_CONF value type for batch read/modify/write operations
///
/// Use `asysconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `asysconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ASysConfVal {
    bits: u32,
}

impl ASysConfVal {
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
        Self { bits: 0x00000203 }
    }

    /// Read **A_INTRA_COST_CMP_OFFSET** `[29:14]`
    ///
    /// Configures video A intra cost offset when I MB compared with P MB.
    #[inline(always)]
    pub const fn a_intra_cost_cmp_offset(&self) -> u32 {
        (self.bits & ASysConf::A_INTRA_COST_CMP_OFFSET_MASK)
            >> ASysConf::A_INTRA_COST_CMP_OFFSET_OFFSET
    }

    /// Read **A_REC_READY_TRIGGER_MB_LINES** `[13:7]`
    ///
    /// Configures when to trigger video A H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4.
    #[inline(always)]
    pub const fn a_rec_ready_trigger_mb_lines(&self) -> u32 {
        (self.bits & ASysConf::A_REC_READY_TRIGGER_MB_LINES_MASK)
            >> ASysConf::A_REC_READY_TRIGGER_MB_LINES_OFFSET
    }

    /// Read **A_DB_TMP_READY_TRIGGER_MB_NUM** `[6:0]`
    ///
    /// Configures when to trigger video A H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3.
    #[inline(always)]
    pub const fn a_db_tmp_ready_trigger_mb_num(&self) -> u32 {
        (self.bits & ASysConf::A_DB_TMP_READY_TRIGGER_MB_NUM_MASK)
            >> ASysConf::A_DB_TMP_READY_TRIGGER_MB_NUM_OFFSET
    }

    /// Set **A_INTRA_COST_CMP_OFFSET** `[29:14]`
    ///
    /// Configures video A intra cost offset when I MB compared with P MB.
    #[inline(always)]
    pub const fn with_a_intra_cost_cmp_offset(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ASysConf::A_INTRA_COST_CMP_OFFSET_MASK)
                | ((value << ASysConf::A_INTRA_COST_CMP_OFFSET_OFFSET)
                    & ASysConf::A_INTRA_COST_CMP_OFFSET_MASK),
        }
    }

    /// Set **A_REC_READY_TRIGGER_MB_LINES** `[13:7]`
    ///
    /// Configures when to trigger video A H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4.
    #[inline(always)]
    pub const fn with_a_rec_ready_trigger_mb_lines(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ASysConf::A_REC_READY_TRIGGER_MB_LINES_MASK)
                | ((value << ASysConf::A_REC_READY_TRIGGER_MB_LINES_OFFSET)
                    & ASysConf::A_REC_READY_TRIGGER_MB_LINES_MASK),
        }
    }

    /// Set **A_DB_TMP_READY_TRIGGER_MB_NUM** `[6:0]`
    ///
    /// Configures when to trigger video A H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3.
    #[inline(always)]
    pub const fn with_a_db_tmp_ready_trigger_mb_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ASysConf::A_DB_TMP_READY_TRIGGER_MB_NUM_MASK)
                | ((value << ASysConf::A_DB_TMP_READY_TRIGGER_MB_NUM_OFFSET)
                    & ASysConf::A_DB_TMP_READY_TRIGGER_MB_NUM_MASK),
        }
    }
}

impl Default for ASysConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ASysConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ASysConfVal {
        ASysConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ASysConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ASysConfVal) -> ASysConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_DECI_SCORE Register
///
/// Video A luma and chroma MB decimate score Register.
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **A_L_DECI_SCORE** `[19:10]` (RW)
///
///   Configures video A luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
///
/// - **A_C_DECI_SCORE** `[9:0]` (RW)
///
///   Configures video A chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
#[repr(transparent)]
pub struct ADeciScore {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ADeciScore {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ADeciScore {
    // ======== Field Constants ========

    /// **A_L_DECI_SCORE**
    ///
    /// Configures video A luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
    ///
    /// Bits `[19:10]`, mask `0x000ffc00`
    pub const A_L_DECI_SCORE_OFFSET: u32 = 10;
    pub const A_L_DECI_SCORE_MASK: u32 = 0x000ffc00;

    /// **A_C_DECI_SCORE**
    ///
    /// Configures video A chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const A_C_DECI_SCORE_OFFSET: u32 = 0;
    pub const A_C_DECI_SCORE_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **A_L_DECI_SCORE** `[19:10]` (RW)
    ///
    /// Configures video A luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
    #[inline(always)]
    pub fn a_l_deci_score(&self) -> u32 {
        (self.value.read_bits() & Self::A_L_DECI_SCORE_MASK) >> Self::A_L_DECI_SCORE_OFFSET
    }

    /// Write **A_L_DECI_SCORE** `[19:10]` (RW)
    ///
    /// Configures video A luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
    #[inline(always)]
    pub fn write_a_l_deci_score(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_L_DECI_SCORE_MASK)
                | ((value << Self::A_L_DECI_SCORE_OFFSET) & Self::A_L_DECI_SCORE_MASK)
        });
    }

    /// **A_C_DECI_SCORE** `[9:0]` (RW)
    ///
    /// Configures video A chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
    #[inline(always)]
    pub fn a_c_deci_score(&self) -> u32 {
        (self.value.read_bits() & Self::A_C_DECI_SCORE_MASK) >> Self::A_C_DECI_SCORE_OFFSET
    }

    /// Write **A_C_DECI_SCORE** `[9:0]` (RW)
    ///
    /// Configures video A chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
    #[inline(always)]
    pub fn write_a_c_deci_score(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_C_DECI_SCORE_MASK)
                | ((value << Self::A_C_DECI_SCORE_OFFSET) & Self::A_C_DECI_SCORE_MASK)
        });
    }
}

/// A_DECI_SCORE value type for batch read/modify/write operations
///
/// Use `adeciscore.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `adeciscore.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ADeciScoreVal {
    bits: u32,
}

impl ADeciScoreVal {
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

    /// Read **A_L_DECI_SCORE** `[19:10]`
    ///
    /// Configures video A luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
    #[inline(always)]
    pub const fn a_l_deci_score(&self) -> u32 {
        (self.bits & ADeciScore::A_L_DECI_SCORE_MASK) >> ADeciScore::A_L_DECI_SCORE_OFFSET
    }

    /// Read **A_C_DECI_SCORE** `[9:0]`
    ///
    /// Configures video A chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
    #[inline(always)]
    pub const fn a_c_deci_score(&self) -> u32 {
        (self.bits & ADeciScore::A_C_DECI_SCORE_MASK) >> ADeciScore::A_C_DECI_SCORE_OFFSET
    }

    /// Set **A_L_DECI_SCORE** `[19:10]`
    ///
    /// Configures video A luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
    #[inline(always)]
    pub const fn with_a_l_deci_score(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ADeciScore::A_L_DECI_SCORE_MASK)
                | ((value << ADeciScore::A_L_DECI_SCORE_OFFSET) & ADeciScore::A_L_DECI_SCORE_MASK),
        }
    }

    /// Set **A_C_DECI_SCORE** `[9:0]`
    ///
    /// Configures video A chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
    #[inline(always)]
    pub const fn with_a_c_deci_score(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ADeciScore::A_C_DECI_SCORE_MASK)
                | ((value << ADeciScore::A_C_DECI_SCORE_OFFSET) & ADeciScore::A_C_DECI_SCORE_MASK),
        }
    }
}

impl Default for ADeciScoreVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ADeciScore {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ADeciScoreVal {
        ADeciScoreVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ADeciScoreVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ADeciScoreVal) -> ADeciScoreVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_DECI_SCORE_OFFSET Register
///
/// Video A luma and chroma MB decimate score offset Register.
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **A_P_CHROMA_DECI_SCORE_OFFSET** `[23:18]` (RW)
///
///   Configures video A p chroma MB decimate score offset. This offset will be added to p chroma MB score.
///
/// - **A_P16X16_DECI_SCORE_OFFSET** `[17:12]` (RW)
///
///   Configures video A p16x16 MB decimate score offset. This offset will be added to p16x16 MB score.
///
/// - **A_I_CHROMA_DECI_SCORE_OFFSET** `[11:6]` (RW)
///
///   Configures video A I chroma MB decimate score offset. This offset will be added to I chroma MB score.
///
/// - **A_I16X16_DECI_SCORE_OFFSET** `[5:0]` (RW)
///
///   Configures video A i16x16 MB decimate score offset. This offset will be added to i16x16 MB score.
#[repr(transparent)]
pub struct ADeciScoreOffset {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ADeciScoreOffset {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ADeciScoreOffset {
    // ======== Field Constants ========

    /// **A_P_CHROMA_DECI_SCORE_OFFSET**
    ///
    /// Configures video A p chroma MB decimate score offset. This offset will be added to p chroma MB score.
    ///
    /// Bits `[23:18]`, mask `0x00fc0000`
    pub const A_P_CHROMA_DECI_SCORE_OFFSET_OFFSET: u32 = 18;
    pub const A_P_CHROMA_DECI_SCORE_OFFSET_MASK: u32 = 0x00fc0000;

    /// **A_P16X16_DECI_SCORE_OFFSET**
    ///
    /// Configures video A p16x16 MB decimate score offset. This offset will be added to p16x16 MB score.
    ///
    /// Bits `[17:12]`, mask `0x0003f000`
    pub const A_P16X16_DECI_SCORE_OFFSET_OFFSET: u32 = 12;
    pub const A_P16X16_DECI_SCORE_OFFSET_MASK: u32 = 0x0003f000;

    /// **A_I_CHROMA_DECI_SCORE_OFFSET**
    ///
    /// Configures video A I chroma MB decimate score offset. This offset will be added to I chroma MB score.
    ///
    /// Bits `[11:6]`, mask `0x00000fc0`
    pub const A_I_CHROMA_DECI_SCORE_OFFSET_OFFSET: u32 = 6;
    pub const A_I_CHROMA_DECI_SCORE_OFFSET_MASK: u32 = 0x00000fc0;

    /// **A_I16X16_DECI_SCORE_OFFSET**
    ///
    /// Configures video A i16x16 MB decimate score offset. This offset will be added to i16x16 MB score.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const A_I16X16_DECI_SCORE_OFFSET_OFFSET: u32 = 0;
    pub const A_I16X16_DECI_SCORE_OFFSET_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **A_P_CHROMA_DECI_SCORE_OFFSET** `[23:18]` (RW)
    ///
    /// Configures video A p chroma MB decimate score offset. This offset will be added to p chroma MB score.
    #[inline(always)]
    pub fn a_p_chroma_deci_score_offset(&self) -> u32 {
        (self.value.read_bits() & Self::A_P_CHROMA_DECI_SCORE_OFFSET_MASK)
            >> Self::A_P_CHROMA_DECI_SCORE_OFFSET_OFFSET
    }

    /// Write **A_P_CHROMA_DECI_SCORE_OFFSET** `[23:18]` (RW)
    ///
    /// Configures video A p chroma MB decimate score offset. This offset will be added to p chroma MB score.
    #[inline(always)]
    pub fn write_a_p_chroma_deci_score_offset(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_P_CHROMA_DECI_SCORE_OFFSET_MASK)
                | ((value << Self::A_P_CHROMA_DECI_SCORE_OFFSET_OFFSET)
                    & Self::A_P_CHROMA_DECI_SCORE_OFFSET_MASK)
        });
    }

    /// **A_P16X16_DECI_SCORE_OFFSET** `[17:12]` (RW)
    ///
    /// Configures video A p16x16 MB decimate score offset. This offset will be added to p16x16 MB score.
    #[inline(always)]
    pub fn a_p16x16_deci_score_offset(&self) -> u32 {
        (self.value.read_bits() & Self::A_P16X16_DECI_SCORE_OFFSET_MASK)
            >> Self::A_P16X16_DECI_SCORE_OFFSET_OFFSET
    }

    /// Write **A_P16X16_DECI_SCORE_OFFSET** `[17:12]` (RW)
    ///
    /// Configures video A p16x16 MB decimate score offset. This offset will be added to p16x16 MB score.
    #[inline(always)]
    pub fn write_a_p16x16_deci_score_offset(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_P16X16_DECI_SCORE_OFFSET_MASK)
                | ((value << Self::A_P16X16_DECI_SCORE_OFFSET_OFFSET)
                    & Self::A_P16X16_DECI_SCORE_OFFSET_MASK)
        });
    }

    /// **A_I_CHROMA_DECI_SCORE_OFFSET** `[11:6]` (RW)
    ///
    /// Configures video A I chroma MB decimate score offset. This offset will be added to I chroma MB score.
    #[inline(always)]
    pub fn a_i_chroma_deci_score_offset(&self) -> u32 {
        (self.value.read_bits() & Self::A_I_CHROMA_DECI_SCORE_OFFSET_MASK)
            >> Self::A_I_CHROMA_DECI_SCORE_OFFSET_OFFSET
    }

    /// Write **A_I_CHROMA_DECI_SCORE_OFFSET** `[11:6]` (RW)
    ///
    /// Configures video A I chroma MB decimate score offset. This offset will be added to I chroma MB score.
    #[inline(always)]
    pub fn write_a_i_chroma_deci_score_offset(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_I_CHROMA_DECI_SCORE_OFFSET_MASK)
                | ((value << Self::A_I_CHROMA_DECI_SCORE_OFFSET_OFFSET)
                    & Self::A_I_CHROMA_DECI_SCORE_OFFSET_MASK)
        });
    }

    /// **A_I16X16_DECI_SCORE_OFFSET** `[5:0]` (RW)
    ///
    /// Configures video A i16x16 MB decimate score offset. This offset will be added to i16x16 MB score.
    #[inline(always)]
    pub fn a_i16x16_deci_score_offset(&self) -> u32 {
        (self.value.read_bits() & Self::A_I16X16_DECI_SCORE_OFFSET_MASK)
            >> Self::A_I16X16_DECI_SCORE_OFFSET_OFFSET
    }

    /// Write **A_I16X16_DECI_SCORE_OFFSET** `[5:0]` (RW)
    ///
    /// Configures video A i16x16 MB decimate score offset. This offset will be added to i16x16 MB score.
    #[inline(always)]
    pub fn write_a_i16x16_deci_score_offset(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_I16X16_DECI_SCORE_OFFSET_MASK)
                | ((value << Self::A_I16X16_DECI_SCORE_OFFSET_OFFSET)
                    & Self::A_I16X16_DECI_SCORE_OFFSET_MASK)
        });
    }
}

/// A_DECI_SCORE_OFFSET value type for batch read/modify/write operations
///
/// Use `adeciscoreoffset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `adeciscoreoffset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ADeciScoreOffsetVal {
    bits: u32,
}

impl ADeciScoreOffsetVal {
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

    /// Read **A_P_CHROMA_DECI_SCORE_OFFSET** `[23:18]`
    ///
    /// Configures video A p chroma MB decimate score offset. This offset will be added to p chroma MB score.
    #[inline(always)]
    pub const fn a_p_chroma_deci_score_offset(&self) -> u32 {
        (self.bits & ADeciScoreOffset::A_P_CHROMA_DECI_SCORE_OFFSET_MASK)
            >> ADeciScoreOffset::A_P_CHROMA_DECI_SCORE_OFFSET_OFFSET
    }

    /// Read **A_P16X16_DECI_SCORE_OFFSET** `[17:12]`
    ///
    /// Configures video A p16x16 MB decimate score offset. This offset will be added to p16x16 MB score.
    #[inline(always)]
    pub const fn a_p16x16_deci_score_offset(&self) -> u32 {
        (self.bits & ADeciScoreOffset::A_P16X16_DECI_SCORE_OFFSET_MASK)
            >> ADeciScoreOffset::A_P16X16_DECI_SCORE_OFFSET_OFFSET
    }

    /// Read **A_I_CHROMA_DECI_SCORE_OFFSET** `[11:6]`
    ///
    /// Configures video A I chroma MB decimate score offset. This offset will be added to I chroma MB score.
    #[inline(always)]
    pub const fn a_i_chroma_deci_score_offset(&self) -> u32 {
        (self.bits & ADeciScoreOffset::A_I_CHROMA_DECI_SCORE_OFFSET_MASK)
            >> ADeciScoreOffset::A_I_CHROMA_DECI_SCORE_OFFSET_OFFSET
    }

    /// Read **A_I16X16_DECI_SCORE_OFFSET** `[5:0]`
    ///
    /// Configures video A i16x16 MB decimate score offset. This offset will be added to i16x16 MB score.
    #[inline(always)]
    pub const fn a_i16x16_deci_score_offset(&self) -> u32 {
        (self.bits & ADeciScoreOffset::A_I16X16_DECI_SCORE_OFFSET_MASK)
            >> ADeciScoreOffset::A_I16X16_DECI_SCORE_OFFSET_OFFSET
    }

    /// Set **A_P_CHROMA_DECI_SCORE_OFFSET** `[23:18]`
    ///
    /// Configures video A p chroma MB decimate score offset. This offset will be added to p chroma MB score.
    #[inline(always)]
    pub const fn with_a_p_chroma_deci_score_offset(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ADeciScoreOffset::A_P_CHROMA_DECI_SCORE_OFFSET_MASK)
                | ((value << ADeciScoreOffset::A_P_CHROMA_DECI_SCORE_OFFSET_OFFSET)
                    & ADeciScoreOffset::A_P_CHROMA_DECI_SCORE_OFFSET_MASK),
        }
    }

    /// Set **A_P16X16_DECI_SCORE_OFFSET** `[17:12]`
    ///
    /// Configures video A p16x16 MB decimate score offset. This offset will be added to p16x16 MB score.
    #[inline(always)]
    pub const fn with_a_p16x16_deci_score_offset(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ADeciScoreOffset::A_P16X16_DECI_SCORE_OFFSET_MASK)
                | ((value << ADeciScoreOffset::A_P16X16_DECI_SCORE_OFFSET_OFFSET)
                    & ADeciScoreOffset::A_P16X16_DECI_SCORE_OFFSET_MASK),
        }
    }

    /// Set **A_I_CHROMA_DECI_SCORE_OFFSET** `[11:6]`
    ///
    /// Configures video A I chroma MB decimate score offset. This offset will be added to I chroma MB score.
    #[inline(always)]
    pub const fn with_a_i_chroma_deci_score_offset(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ADeciScoreOffset::A_I_CHROMA_DECI_SCORE_OFFSET_MASK)
                | ((value << ADeciScoreOffset::A_I_CHROMA_DECI_SCORE_OFFSET_OFFSET)
                    & ADeciScoreOffset::A_I_CHROMA_DECI_SCORE_OFFSET_MASK),
        }
    }

    /// Set **A_I16X16_DECI_SCORE_OFFSET** `[5:0]`
    ///
    /// Configures video A i16x16 MB decimate score offset. This offset will be added to i16x16 MB score.
    #[inline(always)]
    pub const fn with_a_i16x16_deci_score_offset(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ADeciScoreOffset::A_I16X16_DECI_SCORE_OFFSET_MASK)
                | ((value << ADeciScoreOffset::A_I16X16_DECI_SCORE_OFFSET_OFFSET)
                    & ADeciScoreOffset::A_I16X16_DECI_SCORE_OFFSET_MASK),
        }
    }
}

impl Default for ADeciScoreOffsetVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ADeciScoreOffset {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ADeciScoreOffsetVal {
        ADeciScoreOffsetVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ADeciScoreOffsetVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ADeciScoreOffsetVal) -> ADeciScoreOffsetVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_RC_CONF0 Register
///
/// Video A rate control configuration register0.
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **A_MB_RATE_CTRL_EN** `[22]` (RW)
///
///   Configures video A whether or not to open macro block rate ctrl.\\1:Open the macro block rate ctrl\\1:Close the macro block rate ctrl.
///
/// - **A_RATE_CTRL_U** `[21:6]` (RW)
///
///   Configures video A parameter U value. U = int((float) u << 8).
///
/// - **A_QP** `[5:0]` (RW)
///
///   Configures video A frame level initial luma QP value.
#[repr(transparent)]
pub struct ARcConf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ARcConf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ARcConf0 {
    // ======== Field Constants ========

    /// **A_MB_RATE_CTRL_EN**
    ///
    /// Configures video A whether or not to open macro block rate ctrl.\\1:Open the macro block rate ctrl\\1:Close the macro block rate ctrl.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const A_MB_RATE_CTRL_EN_OFFSET: u32 = 22;
    pub const A_MB_RATE_CTRL_EN_MASK: u32 = 0x00400000;
    pub const A_MB_RATE_CTRL_EN: u32 = 1 << 22;

    /// **A_RATE_CTRL_U**
    ///
    /// Configures video A parameter U value. U = int((float) u << 8).
    ///
    /// Bits `[21:6]`, mask `0x003fffc0`
    pub const A_RATE_CTRL_U_OFFSET: u32 = 6;
    pub const A_RATE_CTRL_U_MASK: u32 = 0x003fffc0;

    /// **A_QP**
    ///
    /// Configures video A frame level initial luma QP value.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const A_QP_OFFSET: u32 = 0;
    pub const A_QP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **A_MB_RATE_CTRL_EN** `[22]` (RW)
    ///
    /// Configures video A whether or not to open macro block rate ctrl.\\1:Open the macro block rate ctrl\\1:Close the macro block rate ctrl.
    #[inline(always)]
    pub fn a_mb_rate_ctrl_en(&self) -> bool {
        (self.value.read_bits() & Self::A_MB_RATE_CTRL_EN_MASK) != 0
    }

    /// Set A_MB_RATE_CTRL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_a_mb_rate_ctrl_en(&self) {
        self.value.modify_bits(|r| r | Self::A_MB_RATE_CTRL_EN_MASK);
    }

    /// Clear A_MB_RATE_CTRL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_a_mb_rate_ctrl_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::A_MB_RATE_CTRL_EN_MASK);
    }

    /// **A_RATE_CTRL_U** `[21:6]` (RW)
    ///
    /// Configures video A parameter U value. U = int((float) u << 8).
    #[inline(always)]
    pub fn a_rate_ctrl_u(&self) -> u32 {
        (self.value.read_bits() & Self::A_RATE_CTRL_U_MASK) >> Self::A_RATE_CTRL_U_OFFSET
    }

    /// Write **A_RATE_CTRL_U** `[21:6]` (RW)
    ///
    /// Configures video A parameter U value. U = int((float) u << 8).
    #[inline(always)]
    pub fn write_a_rate_ctrl_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_RATE_CTRL_U_MASK)
                | ((value << Self::A_RATE_CTRL_U_OFFSET) & Self::A_RATE_CTRL_U_MASK)
        });
    }

    /// **A_QP** `[5:0]` (RW)
    ///
    /// Configures video A frame level initial luma QP value.
    #[inline(always)]
    pub fn a_qp(&self) -> u32 {
        (self.value.read_bits() & Self::A_QP_MASK) >> Self::A_QP_OFFSET
    }

    /// Write **A_QP** `[5:0]` (RW)
    ///
    /// Configures video A frame level initial luma QP value.
    #[inline(always)]
    pub fn write_a_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_QP_MASK) | ((value << Self::A_QP_OFFSET) & Self::A_QP_MASK)
        });
    }
}

/// A_RC_CONF0 value type for batch read/modify/write operations
///
/// Use `arcconf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `arcconf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ARcConf0Val {
    bits: u32,
}

impl ARcConf0Val {
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

    /// Read **A_MB_RATE_CTRL_EN** `[22:22]`
    ///
    /// Configures video A whether or not to open macro block rate ctrl.\\1:Open the macro block rate ctrl\\1:Close the macro block rate ctrl.
    #[inline(always)]
    pub const fn a_mb_rate_ctrl_en(&self) -> bool {
        (self.bits & ARcConf0::A_MB_RATE_CTRL_EN_MASK) != 0
    }

    /// Read **A_RATE_CTRL_U** `[21:6]`
    ///
    /// Configures video A parameter U value. U = int((float) u << 8).
    #[inline(always)]
    pub const fn a_rate_ctrl_u(&self) -> u32 {
        (self.bits & ARcConf0::A_RATE_CTRL_U_MASK) >> ARcConf0::A_RATE_CTRL_U_OFFSET
    }

    /// Read **A_QP** `[5:0]`
    ///
    /// Configures video A frame level initial luma QP value.
    #[inline(always)]
    pub const fn a_qp(&self) -> u32 {
        (self.bits & ARcConf0::A_QP_MASK) >> ARcConf0::A_QP_OFFSET
    }

    /// Set **A_MB_RATE_CTRL_EN** `[22:22]`
    ///
    /// Configures video A whether or not to open macro block rate ctrl.\\1:Open the macro block rate ctrl\\1:Close the macro block rate ctrl.
    #[inline(always)]
    pub const fn with_a_mb_rate_ctrl_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ARcConf0::A_MB_RATE_CTRL_EN_MASK
            } else {
                self.bits & !ARcConf0::A_MB_RATE_CTRL_EN_MASK
            },
        }
    }

    /// Set **A_RATE_CTRL_U** `[21:6]`
    ///
    /// Configures video A parameter U value. U = int((float) u << 8).
    #[inline(always)]
    pub const fn with_a_rate_ctrl_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARcConf0::A_RATE_CTRL_U_MASK)
                | ((value << ARcConf0::A_RATE_CTRL_U_OFFSET) & ARcConf0::A_RATE_CTRL_U_MASK),
        }
    }

    /// Set **A_QP** `[5:0]`
    ///
    /// Configures video A frame level initial luma QP value.
    #[inline(always)]
    pub const fn with_a_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARcConf0::A_QP_MASK)
                | ((value << ARcConf0::A_QP_OFFSET) & ARcConf0::A_QP_MASK),
        }
    }
}

impl Default for ARcConf0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ARcConf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ARcConf0Val {
        ARcConf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ARcConf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ARcConf0Val) -> ARcConf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_RC_CONF1 Register
///
/// Video A rate control configuration register1.
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **A_MAD_FRAME_PRED** `[30:19]` (RW)
///
///   Configures vdieo A frame level predicted MB MAD value.
///
/// - **A_QP_MAX** `[18:13]` (RW)
///
///   Configures video A allowed luma QP max value.
///
/// - **A_QP_MIN** `[12:7]` (RW)
///
///   Configures video A allowed luma QP min value.
///
/// - **A_CHROMA_QP_DELTA** `[6:3]` (RW)
///
///   Configures video A chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta.
///
/// - **A_CHROMA_DC_QP_DELTA** `[2:0]` (RW)
///
///   Configures video A chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta.
#[repr(transparent)]
pub struct ARcConf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ARcConf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ARcConf1 {
    // ======== Field Constants ========

    /// **A_MAD_FRAME_PRED**
    ///
    /// Configures vdieo A frame level predicted MB MAD value.
    ///
    /// Bits `[30:19]`, mask `0x7ff80000`
    pub const A_MAD_FRAME_PRED_OFFSET: u32 = 19;
    pub const A_MAD_FRAME_PRED_MASK: u32 = 0x7ff80000;

    /// **A_QP_MAX**
    ///
    /// Configures video A allowed luma QP max value.
    ///
    /// Bits `[18:13]`, mask `0x0007e000`
    pub const A_QP_MAX_OFFSET: u32 = 13;
    pub const A_QP_MAX_MASK: u32 = 0x0007e000;

    /// **A_QP_MIN**
    ///
    /// Configures video A allowed luma QP min value.
    ///
    /// Bits `[12:7]`, mask `0x00001f80`
    pub const A_QP_MIN_OFFSET: u32 = 7;
    pub const A_QP_MIN_MASK: u32 = 0x00001f80;

    /// **A_CHROMA_QP_DELTA**
    ///
    /// Configures video A chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta.
    ///
    /// Bits `[6:3]`, mask `0x00000078`
    pub const A_CHROMA_QP_DELTA_OFFSET: u32 = 3;
    pub const A_CHROMA_QP_DELTA_MASK: u32 = 0x00000078;

    /// **A_CHROMA_DC_QP_DELTA**
    ///
    /// Configures video A chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const A_CHROMA_DC_QP_DELTA_OFFSET: u32 = 0;
    pub const A_CHROMA_DC_QP_DELTA_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **A_MAD_FRAME_PRED** `[30:19]` (RW)
    ///
    /// Configures vdieo A frame level predicted MB MAD value.
    #[inline(always)]
    pub fn a_mad_frame_pred(&self) -> u32 {
        (self.value.read_bits() & Self::A_MAD_FRAME_PRED_MASK) >> Self::A_MAD_FRAME_PRED_OFFSET
    }

    /// Write **A_MAD_FRAME_PRED** `[30:19]` (RW)
    ///
    /// Configures vdieo A frame level predicted MB MAD value.
    #[inline(always)]
    pub fn write_a_mad_frame_pred(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_MAD_FRAME_PRED_MASK)
                | ((value << Self::A_MAD_FRAME_PRED_OFFSET) & Self::A_MAD_FRAME_PRED_MASK)
        });
    }

    /// **A_QP_MAX** `[18:13]` (RW)
    ///
    /// Configures video A allowed luma QP max value.
    #[inline(always)]
    pub fn a_qp_max(&self) -> u32 {
        (self.value.read_bits() & Self::A_QP_MAX_MASK) >> Self::A_QP_MAX_OFFSET
    }

    /// Write **A_QP_MAX** `[18:13]` (RW)
    ///
    /// Configures video A allowed luma QP max value.
    #[inline(always)]
    pub fn write_a_qp_max(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_QP_MAX_MASK) | ((value << Self::A_QP_MAX_OFFSET) & Self::A_QP_MAX_MASK)
        });
    }

    /// **A_QP_MIN** `[12:7]` (RW)
    ///
    /// Configures video A allowed luma QP min value.
    #[inline(always)]
    pub fn a_qp_min(&self) -> u32 {
        (self.value.read_bits() & Self::A_QP_MIN_MASK) >> Self::A_QP_MIN_OFFSET
    }

    /// Write **A_QP_MIN** `[12:7]` (RW)
    ///
    /// Configures video A allowed luma QP min value.
    #[inline(always)]
    pub fn write_a_qp_min(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_QP_MIN_MASK) | ((value << Self::A_QP_MIN_OFFSET) & Self::A_QP_MIN_MASK)
        });
    }

    /// **A_CHROMA_QP_DELTA** `[6:3]` (RW)
    ///
    /// Configures video A chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta.
    #[inline(always)]
    pub fn a_chroma_qp_delta(&self) -> u32 {
        (self.value.read_bits() & Self::A_CHROMA_QP_DELTA_MASK) >> Self::A_CHROMA_QP_DELTA_OFFSET
    }

    /// Write **A_CHROMA_QP_DELTA** `[6:3]` (RW)
    ///
    /// Configures video A chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta.
    #[inline(always)]
    pub fn write_a_chroma_qp_delta(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_CHROMA_QP_DELTA_MASK)
                | ((value << Self::A_CHROMA_QP_DELTA_OFFSET) & Self::A_CHROMA_QP_DELTA_MASK)
        });
    }

    /// **A_CHROMA_DC_QP_DELTA** `[2:0]` (RW)
    ///
    /// Configures video A chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta.
    #[inline(always)]
    pub fn a_chroma_dc_qp_delta(&self) -> u32 {
        (self.value.read_bits() & Self::A_CHROMA_DC_QP_DELTA_MASK)
            >> Self::A_CHROMA_DC_QP_DELTA_OFFSET
    }

    /// Write **A_CHROMA_DC_QP_DELTA** `[2:0]` (RW)
    ///
    /// Configures video A chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta.
    #[inline(always)]
    pub fn write_a_chroma_dc_qp_delta(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_CHROMA_DC_QP_DELTA_MASK)
                | ((value << Self::A_CHROMA_DC_QP_DELTA_OFFSET) & Self::A_CHROMA_DC_QP_DELTA_MASK)
        });
    }
}

/// A_RC_CONF1 value type for batch read/modify/write operations
///
/// Use `arcconf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `arcconf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ARcConf1Val {
    bits: u32,
}

impl ARcConf1Val {
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

    /// Read **A_MAD_FRAME_PRED** `[30:19]`
    ///
    /// Configures vdieo A frame level predicted MB MAD value.
    #[inline(always)]
    pub const fn a_mad_frame_pred(&self) -> u32 {
        (self.bits & ARcConf1::A_MAD_FRAME_PRED_MASK) >> ARcConf1::A_MAD_FRAME_PRED_OFFSET
    }

    /// Read **A_QP_MAX** `[18:13]`
    ///
    /// Configures video A allowed luma QP max value.
    #[inline(always)]
    pub const fn a_qp_max(&self) -> u32 {
        (self.bits & ARcConf1::A_QP_MAX_MASK) >> ARcConf1::A_QP_MAX_OFFSET
    }

    /// Read **A_QP_MIN** `[12:7]`
    ///
    /// Configures video A allowed luma QP min value.
    #[inline(always)]
    pub const fn a_qp_min(&self) -> u32 {
        (self.bits & ARcConf1::A_QP_MIN_MASK) >> ARcConf1::A_QP_MIN_OFFSET
    }

    /// Read **A_CHROMA_QP_DELTA** `[6:3]`
    ///
    /// Configures video A chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta.
    #[inline(always)]
    pub const fn a_chroma_qp_delta(&self) -> u32 {
        (self.bits & ARcConf1::A_CHROMA_QP_DELTA_MASK) >> ARcConf1::A_CHROMA_QP_DELTA_OFFSET
    }

    /// Read **A_CHROMA_DC_QP_DELTA** `[2:0]`
    ///
    /// Configures video A chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta.
    #[inline(always)]
    pub const fn a_chroma_dc_qp_delta(&self) -> u32 {
        (self.bits & ARcConf1::A_CHROMA_DC_QP_DELTA_MASK) >> ARcConf1::A_CHROMA_DC_QP_DELTA_OFFSET
    }

    /// Set **A_MAD_FRAME_PRED** `[30:19]`
    ///
    /// Configures vdieo A frame level predicted MB MAD value.
    #[inline(always)]
    pub const fn with_a_mad_frame_pred(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARcConf1::A_MAD_FRAME_PRED_MASK)
                | ((value << ARcConf1::A_MAD_FRAME_PRED_OFFSET) & ARcConf1::A_MAD_FRAME_PRED_MASK),
        }
    }

    /// Set **A_QP_MAX** `[18:13]`
    ///
    /// Configures video A allowed luma QP max value.
    #[inline(always)]
    pub const fn with_a_qp_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARcConf1::A_QP_MAX_MASK)
                | ((value << ARcConf1::A_QP_MAX_OFFSET) & ARcConf1::A_QP_MAX_MASK),
        }
    }

    /// Set **A_QP_MIN** `[12:7]`
    ///
    /// Configures video A allowed luma QP min value.
    #[inline(always)]
    pub const fn with_a_qp_min(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARcConf1::A_QP_MIN_MASK)
                | ((value << ARcConf1::A_QP_MIN_OFFSET) & ARcConf1::A_QP_MIN_MASK),
        }
    }

    /// Set **A_CHROMA_QP_DELTA** `[6:3]`
    ///
    /// Configures video A chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta.
    #[inline(always)]
    pub const fn with_a_chroma_qp_delta(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARcConf1::A_CHROMA_QP_DELTA_MASK)
                | ((value << ARcConf1::A_CHROMA_QP_DELTA_OFFSET)
                    & ARcConf1::A_CHROMA_QP_DELTA_MASK),
        }
    }

    /// Set **A_CHROMA_DC_QP_DELTA** `[2:0]`
    ///
    /// Configures video A chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta.
    #[inline(always)]
    pub const fn with_a_chroma_dc_qp_delta(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARcConf1::A_CHROMA_DC_QP_DELTA_MASK)
                | ((value << ARcConf1::A_CHROMA_DC_QP_DELTA_OFFSET)
                    & ARcConf1::A_CHROMA_DC_QP_DELTA_MASK),
        }
    }
}

impl Default for ARcConf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ARcConf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ARcConf1Val {
        ARcConf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ARcConf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ARcConf1Val) -> ARcConf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_DB_BYPASS Register
///
/// Video A Deblocking bypass register
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **A_BYPASS_DB_FILTER** `[0]` (RW)
///
///   Configures whether or not to bypass video A deblcoking filter. \\0: Open the deblock filter\\1: Close the deblock filter
#[repr(transparent)]
pub struct ADbBypass {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ADbBypass {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ADbBypass {
    // ======== Field Constants ========

    /// **A_BYPASS_DB_FILTER**
    ///
    /// Configures whether or not to bypass video A deblcoking filter. \\0: Open the deblock filter\\1: Close the deblock filter
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const A_BYPASS_DB_FILTER_OFFSET: u32 = 0;
    pub const A_BYPASS_DB_FILTER_MASK: u32 = 0x00000001;
    pub const A_BYPASS_DB_FILTER: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **A_BYPASS_DB_FILTER** `[0]` (RW)
    ///
    /// Configures whether or not to bypass video A deblcoking filter. \\0: Open the deblock filter\\1: Close the deblock filter
    #[inline(always)]
    pub fn a_bypass_db_filter(&self) -> bool {
        (self.value.read_bits() & Self::A_BYPASS_DB_FILTER_MASK) != 0
    }

    /// Set A_BYPASS_DB_FILTER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_a_bypass_db_filter(&self) {
        self.value
            .modify_bits(|r| r | Self::A_BYPASS_DB_FILTER_MASK);
    }

    /// Clear A_BYPASS_DB_FILTER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_a_bypass_db_filter(&self) {
        self.value
            .modify_bits(|r| r & !Self::A_BYPASS_DB_FILTER_MASK);
    }
}

/// A_DB_BYPASS value type for batch read/modify/write operations
///
/// Use `adbbypass.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `adbbypass.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ADbBypassVal {
    bits: u32,
}

impl ADbBypassVal {
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

    /// Read **A_BYPASS_DB_FILTER** `[0:0]`
    ///
    /// Configures whether or not to bypass video A deblcoking filter. \\0: Open the deblock filter\\1: Close the deblock filter
    #[inline(always)]
    pub const fn a_bypass_db_filter(&self) -> bool {
        (self.bits & ADbBypass::A_BYPASS_DB_FILTER_MASK) != 0
    }

    /// Set **A_BYPASS_DB_FILTER** `[0:0]`
    ///
    /// Configures whether or not to bypass video A deblcoking filter. \\0: Open the deblock filter\\1: Close the deblock filter
    #[inline(always)]
    pub const fn with_a_bypass_db_filter(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ADbBypass::A_BYPASS_DB_FILTER_MASK
            } else {
                self.bits & !ADbBypass::A_BYPASS_DB_FILTER_MASK
            },
        }
    }
}

impl Default for ADbBypassVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ADbBypass {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ADbBypassVal {
        ADbBypassVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ADbBypassVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ADbBypassVal) -> ADbBypassVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_ROI_REGION0 Register
///
/// Video A H264 ROI region0 range configure register.
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video A ROI of region 0 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 0 in Video A.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 0 in Video A.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 0 in Video A.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontal start macroblocks of region 0 in Video A.
#[repr(transparent)]
pub struct ARoiRegion0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ARoiRegion0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ARoiRegion0 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video A ROI of region 0 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 0 in Video A.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 0 in Video A.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 0 in Video A.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontal start macroblocks of region 0 in Video A.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video A ROI of region 0 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 0 in Video A.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 0 in Video A.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 0 in Video A.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 0 in Video A.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 0 in Video A.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 0 in Video A.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 0 in Video A.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 0 in Video A.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// A_ROI_REGION0 value type for batch read/modify/write operations
///
/// Use `aroiregion0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `aroiregion0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ARoiRegion0Val {
    bits: u32,
}

impl ARoiRegion0Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 0 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & ARoiRegion0::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 0 in Video A.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & ARoiRegion0::Y_LEN_MASK) >> ARoiRegion0::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 0 in Video A.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & ARoiRegion0::X_LEN_MASK) >> ARoiRegion0::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 0 in Video A.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & ARoiRegion0::Y_MASK) >> ARoiRegion0::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 0 in Video A.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & ARoiRegion0::X_MASK) >> ARoiRegion0::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 0 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ARoiRegion0::EN_MASK
            } else {
                self.bits & !ARoiRegion0::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 0 in Video A.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion0::Y_LEN_MASK)
                | ((value << ARoiRegion0::Y_LEN_OFFSET) & ARoiRegion0::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 0 in Video A.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion0::X_LEN_MASK)
                | ((value << ARoiRegion0::X_LEN_OFFSET) & ARoiRegion0::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 0 in Video A.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion0::Y_MASK)
                | ((value << ARoiRegion0::Y_OFFSET) & ARoiRegion0::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 0 in Video A.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion0::X_MASK)
                | ((value << ARoiRegion0::X_OFFSET) & ARoiRegion0::X_MASK),
        }
    }
}

impl Default for ARoiRegion0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ARoiRegion0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ARoiRegion0Val {
        ARoiRegion0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ARoiRegion0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ARoiRegion0Val) -> ARoiRegion0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_ROI_REGION1 Register
///
/// Video A H264 ROI region1 range configure register.
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video A ROI of region 1 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 1 in Video A.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 1 in Video A.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 1 in Video A.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontal start macroblocks of region 1 in Video A.
#[repr(transparent)]
pub struct ARoiRegion1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ARoiRegion1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ARoiRegion1 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video A ROI of region 1 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 1 in Video A.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 1 in Video A.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 1 in Video A.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontal start macroblocks of region 1 in Video A.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video A ROI of region 1 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 1 in Video A.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 1 in Video A.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 1 in Video A.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 1 in Video A.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 1 in Video A.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 1 in Video A.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 1 in Video A.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 1 in Video A.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// A_ROI_REGION1 value type for batch read/modify/write operations
///
/// Use `aroiregion1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `aroiregion1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ARoiRegion1Val {
    bits: u32,
}

impl ARoiRegion1Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 1 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & ARoiRegion1::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 1 in Video A.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & ARoiRegion1::Y_LEN_MASK) >> ARoiRegion1::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 1 in Video A.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & ARoiRegion1::X_LEN_MASK) >> ARoiRegion1::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 1 in Video A.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & ARoiRegion1::Y_MASK) >> ARoiRegion1::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 1 in Video A.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & ARoiRegion1::X_MASK) >> ARoiRegion1::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 1 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ARoiRegion1::EN_MASK
            } else {
                self.bits & !ARoiRegion1::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 1 in Video A.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion1::Y_LEN_MASK)
                | ((value << ARoiRegion1::Y_LEN_OFFSET) & ARoiRegion1::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 1 in Video A.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion1::X_LEN_MASK)
                | ((value << ARoiRegion1::X_LEN_OFFSET) & ARoiRegion1::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 1 in Video A.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion1::Y_MASK)
                | ((value << ARoiRegion1::Y_OFFSET) & ARoiRegion1::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 1 in Video A.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion1::X_MASK)
                | ((value << ARoiRegion1::X_OFFSET) & ARoiRegion1::X_MASK),
        }
    }
}

impl Default for ARoiRegion1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ARoiRegion1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ARoiRegion1Val {
        ARoiRegion1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ARoiRegion1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ARoiRegion1Val) -> ARoiRegion1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_ROI_REGION2 Register
///
/// Video A H264 ROI region2 range configure register.
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video A ROI of region 2 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 2 in Video A.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 2 in Video A.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 2 in Video A.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontal start macroblocks of region 2 in Video A.
#[repr(transparent)]
pub struct ARoiRegion2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ARoiRegion2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ARoiRegion2 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video A ROI of region 2 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 2 in Video A.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 2 in Video A.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 2 in Video A.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontal start macroblocks of region 2 in Video A.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video A ROI of region 2 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 2 in Video A.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 2 in Video A.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 2 in Video A.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 2 in Video A.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 2 in Video A.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 2 in Video A.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 2 in Video A.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 2 in Video A.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// A_ROI_REGION2 value type for batch read/modify/write operations
///
/// Use `aroiregion2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `aroiregion2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ARoiRegion2Val {
    bits: u32,
}

impl ARoiRegion2Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 2 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & ARoiRegion2::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 2 in Video A.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & ARoiRegion2::Y_LEN_MASK) >> ARoiRegion2::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 2 in Video A.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & ARoiRegion2::X_LEN_MASK) >> ARoiRegion2::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 2 in Video A.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & ARoiRegion2::Y_MASK) >> ARoiRegion2::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 2 in Video A.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & ARoiRegion2::X_MASK) >> ARoiRegion2::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 2 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ARoiRegion2::EN_MASK
            } else {
                self.bits & !ARoiRegion2::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 2 in Video A.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion2::Y_LEN_MASK)
                | ((value << ARoiRegion2::Y_LEN_OFFSET) & ARoiRegion2::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 2 in Video A.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion2::X_LEN_MASK)
                | ((value << ARoiRegion2::X_LEN_OFFSET) & ARoiRegion2::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 2 in Video A.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion2::Y_MASK)
                | ((value << ARoiRegion2::Y_OFFSET) & ARoiRegion2::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 2 in Video A.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion2::X_MASK)
                | ((value << ARoiRegion2::X_OFFSET) & ARoiRegion2::X_MASK),
        }
    }
}

impl Default for ARoiRegion2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ARoiRegion2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ARoiRegion2Val {
        ARoiRegion2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ARoiRegion2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ARoiRegion2Val) -> ARoiRegion2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_ROI_REGION3 Register
///
/// Video A H264 ROI region3 range configure register.
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video A ROI of region 3 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 3 in video A.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 3 in video A.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 3 in Video A.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontal start macroblocks of region 3 in Video A.
#[repr(transparent)]
pub struct ARoiRegion3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ARoiRegion3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ARoiRegion3 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video A ROI of region 3 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 3 in video A.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 3 in video A.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 3 in Video A.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontal start macroblocks of region 3 in Video A.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video A ROI of region 3 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 3 in video A.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 3 in video A.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 3 in video A.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 3 in video A.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 3 in Video A.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 3 in Video A.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 3 in Video A.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 3 in Video A.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// A_ROI_REGION3 value type for batch read/modify/write operations
///
/// Use `aroiregion3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `aroiregion3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ARoiRegion3Val {
    bits: u32,
}

impl ARoiRegion3Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 3 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & ARoiRegion3::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 3 in video A.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & ARoiRegion3::Y_LEN_MASK) >> ARoiRegion3::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 3 in video A.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & ARoiRegion3::X_LEN_MASK) >> ARoiRegion3::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 3 in Video A.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & ARoiRegion3::Y_MASK) >> ARoiRegion3::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 3 in Video A.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & ARoiRegion3::X_MASK) >> ARoiRegion3::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 3 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ARoiRegion3::EN_MASK
            } else {
                self.bits & !ARoiRegion3::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 3 in video A.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion3::Y_LEN_MASK)
                | ((value << ARoiRegion3::Y_LEN_OFFSET) & ARoiRegion3::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 3 in video A.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion3::X_LEN_MASK)
                | ((value << ARoiRegion3::X_LEN_OFFSET) & ARoiRegion3::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 3 in Video A.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion3::Y_MASK)
                | ((value << ARoiRegion3::Y_OFFSET) & ARoiRegion3::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 3 in Video A.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion3::X_MASK)
                | ((value << ARoiRegion3::X_OFFSET) & ARoiRegion3::X_MASK),
        }
    }
}

impl Default for ARoiRegion3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ARoiRegion3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ARoiRegion3Val {
        ARoiRegion3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ARoiRegion3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ARoiRegion3Val) -> ARoiRegion3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_ROI_REGION4 Register
///
/// Video A H264 ROI region4 range configure register.
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video A ROI of region 4 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 4 in video A.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 4 in video A.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 4 in Video A.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontal start macroblocks of region 4 in Video A.
#[repr(transparent)]
pub struct ARoiRegion4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ARoiRegion4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ARoiRegion4 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video A ROI of region 4 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 4 in video A.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 4 in video A.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 4 in Video A.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontal start macroblocks of region 4 in Video A.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video A ROI of region 4 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 4 in video A.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 4 in video A.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 4 in video A.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 4 in video A.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 4 in Video A.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 4 in Video A.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 4 in Video A.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 4 in Video A.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// A_ROI_REGION4 value type for batch read/modify/write operations
///
/// Use `aroiregion4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `aroiregion4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ARoiRegion4Val {
    bits: u32,
}

impl ARoiRegion4Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 4 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & ARoiRegion4::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 4 in video A.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & ARoiRegion4::Y_LEN_MASK) >> ARoiRegion4::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 4 in video A.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & ARoiRegion4::X_LEN_MASK) >> ARoiRegion4::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 4 in Video A.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & ARoiRegion4::Y_MASK) >> ARoiRegion4::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 4 in Video A.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & ARoiRegion4::X_MASK) >> ARoiRegion4::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 4 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ARoiRegion4::EN_MASK
            } else {
                self.bits & !ARoiRegion4::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 4 in video A.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion4::Y_LEN_MASK)
                | ((value << ARoiRegion4::Y_LEN_OFFSET) & ARoiRegion4::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 4 in video A.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion4::X_LEN_MASK)
                | ((value << ARoiRegion4::X_LEN_OFFSET) & ARoiRegion4::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 4 in Video A.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion4::Y_MASK)
                | ((value << ARoiRegion4::Y_OFFSET) & ARoiRegion4::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 4 in Video A.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion4::X_MASK)
                | ((value << ARoiRegion4::X_OFFSET) & ARoiRegion4::X_MASK),
        }
    }
}

impl Default for ARoiRegion4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ARoiRegion4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ARoiRegion4Val {
        ARoiRegion4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ARoiRegion4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ARoiRegion4Val) -> ARoiRegion4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_ROI_REGION5 Register
///
/// Video A H264 ROI region5 range configure register.
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video A ROI of region 5 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 5 in video A.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 5 video A.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 5 video A.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontial start macroblocks of region 5 video A.
#[repr(transparent)]
pub struct ARoiRegion5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ARoiRegion5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ARoiRegion5 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video A ROI of region 5 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 5 in video A.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 5 video A.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 5 video A.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontial start macroblocks of region 5 video A.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video A ROI of region 5 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 5 in video A.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 5 in video A.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 5 video A.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 5 video A.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 5 video A.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 5 video A.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontial start macroblocks of region 5 video A.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontial start macroblocks of region 5 video A.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// A_ROI_REGION5 value type for batch read/modify/write operations
///
/// Use `aroiregion5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `aroiregion5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ARoiRegion5Val {
    bits: u32,
}

impl ARoiRegion5Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 5 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & ARoiRegion5::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 5 in video A.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & ARoiRegion5::Y_LEN_MASK) >> ARoiRegion5::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 5 video A.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & ARoiRegion5::X_LEN_MASK) >> ARoiRegion5::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 5 video A.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & ARoiRegion5::Y_MASK) >> ARoiRegion5::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontial start macroblocks of region 5 video A.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & ARoiRegion5::X_MASK) >> ARoiRegion5::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 5 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ARoiRegion5::EN_MASK
            } else {
                self.bits & !ARoiRegion5::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 5 in video A.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion5::Y_LEN_MASK)
                | ((value << ARoiRegion5::Y_LEN_OFFSET) & ARoiRegion5::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 5 video A.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion5::X_LEN_MASK)
                | ((value << ARoiRegion5::X_LEN_OFFSET) & ARoiRegion5::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 5 video A.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion5::Y_MASK)
                | ((value << ARoiRegion5::Y_OFFSET) & ARoiRegion5::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontial start macroblocks of region 5 video A.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion5::X_MASK)
                | ((value << ARoiRegion5::X_OFFSET) & ARoiRegion5::X_MASK),
        }
    }
}

impl Default for ARoiRegion5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ARoiRegion5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ARoiRegion5Val {
        ARoiRegion5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ARoiRegion5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ARoiRegion5Val) -> ARoiRegion5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_ROI_REGION6 Register
///
/// Video A H264 ROI region6 range configure register.
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video A ROI of region 6 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 6 in video A.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 6 in video A.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 6 in video A.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontial start macroblocks of region 6 video A.
#[repr(transparent)]
pub struct ARoiRegion6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ARoiRegion6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ARoiRegion6 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video A ROI of region 6 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 6 in video A.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 6 in video A.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 6 in video A.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontial start macroblocks of region 6 video A.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video A ROI of region 6 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 6 in video A.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 6 in video A.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 6 in video A.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 6 in video A.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 6 in video A.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 6 in video A.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontial start macroblocks of region 6 video A.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontial start macroblocks of region 6 video A.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// A_ROI_REGION6 value type for batch read/modify/write operations
///
/// Use `aroiregion6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `aroiregion6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ARoiRegion6Val {
    bits: u32,
}

impl ARoiRegion6Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 6 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & ARoiRegion6::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 6 in video A.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & ARoiRegion6::Y_LEN_MASK) >> ARoiRegion6::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 6 in video A.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & ARoiRegion6::X_LEN_MASK) >> ARoiRegion6::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 6 in video A.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & ARoiRegion6::Y_MASK) >> ARoiRegion6::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontial start macroblocks of region 6 video A.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & ARoiRegion6::X_MASK) >> ARoiRegion6::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 6 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ARoiRegion6::EN_MASK
            } else {
                self.bits & !ARoiRegion6::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 6 in video A.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion6::Y_LEN_MASK)
                | ((value << ARoiRegion6::Y_LEN_OFFSET) & ARoiRegion6::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 6 in video A.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion6::X_LEN_MASK)
                | ((value << ARoiRegion6::X_LEN_OFFSET) & ARoiRegion6::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 6 in video A.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion6::Y_MASK)
                | ((value << ARoiRegion6::Y_OFFSET) & ARoiRegion6::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontial start macroblocks of region 6 video A.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion6::X_MASK)
                | ((value << ARoiRegion6::X_OFFSET) & ARoiRegion6::X_MASK),
        }
    }
}

impl Default for ARoiRegion6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ARoiRegion6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ARoiRegion6Val {
        ARoiRegion6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ARoiRegion6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ARoiRegion6Val) -> ARoiRegion6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_ROI_REGION7 Register
///
/// Video A H264 ROI region7 range configure register.
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video A ROI of region 7 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 7 in video A.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 7 in video A.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 7 in video A.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontal start macroblocks of region 7 in video A.
#[repr(transparent)]
pub struct ARoiRegion7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ARoiRegion7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ARoiRegion7 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video A ROI of region 7 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 7 in video A.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 7 in video A.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 7 in video A.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontal start macroblocks of region 7 in video A.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video A ROI of region 7 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 7 in video A.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 7 in video A.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 7 in video A.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 7 in video A.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 7 in video A.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 7 in video A.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 7 in video A.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 7 in video A.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// A_ROI_REGION7 value type for batch read/modify/write operations
///
/// Use `aroiregion7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `aroiregion7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ARoiRegion7Val {
    bits: u32,
}

impl ARoiRegion7Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 7 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & ARoiRegion7::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 7 in video A.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & ARoiRegion7::Y_LEN_MASK) >> ARoiRegion7::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 7 in video A.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & ARoiRegion7::X_LEN_MASK) >> ARoiRegion7::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 7 in video A.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & ARoiRegion7::Y_MASK) >> ARoiRegion7::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 7 in video A.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & ARoiRegion7::X_MASK) >> ARoiRegion7::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video A ROI of region 7 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ARoiRegion7::EN_MASK
            } else {
                self.bits & !ARoiRegion7::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 7 in video A.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion7::Y_LEN_MASK)
                | ((value << ARoiRegion7::Y_LEN_OFFSET) & ARoiRegion7::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 7 in video A.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion7::X_LEN_MASK)
                | ((value << ARoiRegion7::X_LEN_OFFSET) & ARoiRegion7::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 7 in video A.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion7::Y_MASK)
                | ((value << ARoiRegion7::Y_OFFSET) & ARoiRegion7::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 7 in video A.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion7::X_MASK)
                | ((value << ARoiRegion7::X_OFFSET) & ARoiRegion7::X_MASK),
        }
    }
}

impl Default for ARoiRegion7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ARoiRegion7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ARoiRegion7Val {
        ARoiRegion7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ARoiRegion7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ARoiRegion7Val) -> ARoiRegion7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_ROI_REGION0_3_QP Register
///
/// Video A H264 ROI region0, region1,region2,region3 QP register.
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **A_ROI_REGION3_QP** `[27:21]` (RW)
///
///   Configure H264 ROI region3 qp in video A,fixed qp or delta qp.
///
/// - **A_ROI_REGION2_QP** `[20:14]` (RW)
///
///   Configure H264 ROI region2 qp in video A,fixed qp or delta qp.
///
/// - **A_ROI_REGION1_QP** `[13:7]` (RW)
///
///   Configure H264 ROI region1 qp in video A,fixed qp or delta qp.
///
/// - **A_ROI_REGION0_QP** `[6:0]` (RW)
///
///   Configure H264 ROI region0 qp in video A,fixed qp or delta qp.
#[repr(transparent)]
pub struct ARoiRegion03Qp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ARoiRegion03Qp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ARoiRegion03Qp {
    // ======== Field Constants ========

    /// **A_ROI_REGION3_QP**
    ///
    /// Configure H264 ROI region3 qp in video A,fixed qp or delta qp.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const A_ROI_REGION3_QP_OFFSET: u32 = 21;
    pub const A_ROI_REGION3_QP_MASK: u32 = 0x0fe00000;

    /// **A_ROI_REGION2_QP**
    ///
    /// Configure H264 ROI region2 qp in video A,fixed qp or delta qp.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const A_ROI_REGION2_QP_OFFSET: u32 = 14;
    pub const A_ROI_REGION2_QP_MASK: u32 = 0x001fc000;

    /// **A_ROI_REGION1_QP**
    ///
    /// Configure H264 ROI region1 qp in video A,fixed qp or delta qp.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const A_ROI_REGION1_QP_OFFSET: u32 = 7;
    pub const A_ROI_REGION1_QP_MASK: u32 = 0x00003f80;

    /// **A_ROI_REGION0_QP**
    ///
    /// Configure H264 ROI region0 qp in video A,fixed qp or delta qp.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const A_ROI_REGION0_QP_OFFSET: u32 = 0;
    pub const A_ROI_REGION0_QP_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **A_ROI_REGION3_QP** `[27:21]` (RW)
    ///
    /// Configure H264 ROI region3 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn a_roi_region3_qp(&self) -> u32 {
        (self.value.read_bits() & Self::A_ROI_REGION3_QP_MASK) >> Self::A_ROI_REGION3_QP_OFFSET
    }

    /// Write **A_ROI_REGION3_QP** `[27:21]` (RW)
    ///
    /// Configure H264 ROI region3 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_a_roi_region3_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_ROI_REGION3_QP_MASK)
                | ((value << Self::A_ROI_REGION3_QP_OFFSET) & Self::A_ROI_REGION3_QP_MASK)
        });
    }

    /// **A_ROI_REGION2_QP** `[20:14]` (RW)
    ///
    /// Configure H264 ROI region2 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn a_roi_region2_qp(&self) -> u32 {
        (self.value.read_bits() & Self::A_ROI_REGION2_QP_MASK) >> Self::A_ROI_REGION2_QP_OFFSET
    }

    /// Write **A_ROI_REGION2_QP** `[20:14]` (RW)
    ///
    /// Configure H264 ROI region2 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_a_roi_region2_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_ROI_REGION2_QP_MASK)
                | ((value << Self::A_ROI_REGION2_QP_OFFSET) & Self::A_ROI_REGION2_QP_MASK)
        });
    }

    /// **A_ROI_REGION1_QP** `[13:7]` (RW)
    ///
    /// Configure H264 ROI region1 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn a_roi_region1_qp(&self) -> u32 {
        (self.value.read_bits() & Self::A_ROI_REGION1_QP_MASK) >> Self::A_ROI_REGION1_QP_OFFSET
    }

    /// Write **A_ROI_REGION1_QP** `[13:7]` (RW)
    ///
    /// Configure H264 ROI region1 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_a_roi_region1_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_ROI_REGION1_QP_MASK)
                | ((value << Self::A_ROI_REGION1_QP_OFFSET) & Self::A_ROI_REGION1_QP_MASK)
        });
    }

    /// **A_ROI_REGION0_QP** `[6:0]` (RW)
    ///
    /// Configure H264 ROI region0 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn a_roi_region0_qp(&self) -> u32 {
        (self.value.read_bits() & Self::A_ROI_REGION0_QP_MASK) >> Self::A_ROI_REGION0_QP_OFFSET
    }

    /// Write **A_ROI_REGION0_QP** `[6:0]` (RW)
    ///
    /// Configure H264 ROI region0 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_a_roi_region0_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_ROI_REGION0_QP_MASK)
                | ((value << Self::A_ROI_REGION0_QP_OFFSET) & Self::A_ROI_REGION0_QP_MASK)
        });
    }
}

/// A_ROI_REGION0_3_QP value type for batch read/modify/write operations
///
/// Use `aroiregion03qp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `aroiregion03qp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ARoiRegion03QpVal {
    bits: u32,
}

impl ARoiRegion03QpVal {
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

    /// Read **A_ROI_REGION3_QP** `[27:21]`
    ///
    /// Configure H264 ROI region3 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn a_roi_region3_qp(&self) -> u32 {
        (self.bits & ARoiRegion03Qp::A_ROI_REGION3_QP_MASK)
            >> ARoiRegion03Qp::A_ROI_REGION3_QP_OFFSET
    }

    /// Read **A_ROI_REGION2_QP** `[20:14]`
    ///
    /// Configure H264 ROI region2 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn a_roi_region2_qp(&self) -> u32 {
        (self.bits & ARoiRegion03Qp::A_ROI_REGION2_QP_MASK)
            >> ARoiRegion03Qp::A_ROI_REGION2_QP_OFFSET
    }

    /// Read **A_ROI_REGION1_QP** `[13:7]`
    ///
    /// Configure H264 ROI region1 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn a_roi_region1_qp(&self) -> u32 {
        (self.bits & ARoiRegion03Qp::A_ROI_REGION1_QP_MASK)
            >> ARoiRegion03Qp::A_ROI_REGION1_QP_OFFSET
    }

    /// Read **A_ROI_REGION0_QP** `[6:0]`
    ///
    /// Configure H264 ROI region0 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn a_roi_region0_qp(&self) -> u32 {
        (self.bits & ARoiRegion03Qp::A_ROI_REGION0_QP_MASK)
            >> ARoiRegion03Qp::A_ROI_REGION0_QP_OFFSET
    }

    /// Set **A_ROI_REGION3_QP** `[27:21]`
    ///
    /// Configure H264 ROI region3 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_a_roi_region3_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion03Qp::A_ROI_REGION3_QP_MASK)
                | ((value << ARoiRegion03Qp::A_ROI_REGION3_QP_OFFSET)
                    & ARoiRegion03Qp::A_ROI_REGION3_QP_MASK),
        }
    }

    /// Set **A_ROI_REGION2_QP** `[20:14]`
    ///
    /// Configure H264 ROI region2 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_a_roi_region2_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion03Qp::A_ROI_REGION2_QP_MASK)
                | ((value << ARoiRegion03Qp::A_ROI_REGION2_QP_OFFSET)
                    & ARoiRegion03Qp::A_ROI_REGION2_QP_MASK),
        }
    }

    /// Set **A_ROI_REGION1_QP** `[13:7]`
    ///
    /// Configure H264 ROI region1 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_a_roi_region1_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion03Qp::A_ROI_REGION1_QP_MASK)
                | ((value << ARoiRegion03Qp::A_ROI_REGION1_QP_OFFSET)
                    & ARoiRegion03Qp::A_ROI_REGION1_QP_MASK),
        }
    }

    /// Set **A_ROI_REGION0_QP** `[6:0]`
    ///
    /// Configure H264 ROI region0 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_a_roi_region0_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion03Qp::A_ROI_REGION0_QP_MASK)
                | ((value << ARoiRegion03Qp::A_ROI_REGION0_QP_OFFSET)
                    & ARoiRegion03Qp::A_ROI_REGION0_QP_MASK),
        }
    }
}

impl Default for ARoiRegion03QpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ARoiRegion03Qp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ARoiRegion03QpVal {
        ARoiRegion03QpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ARoiRegion03QpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ARoiRegion03QpVal) -> ARoiRegion03QpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_ROI_REGION4_7_QP Register
///
/// Video A H264 ROI region4, region5,region6,region7 QP register.
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **A_ROI_REGION7_QP** `[27:21]` (RW)
///
///   Configure H264 ROI region7 qp in video A,fixed qp or delta qp.
///
/// - **A_ROI_REGION6_QP** `[20:14]` (RW)
///
///   Configure H264 ROI region6 qp in video A,fixed qp or delta qp.
///
/// - **A_ROI_REGION5_QP** `[13:7]` (RW)
///
///   Configure H264 ROI region5 qp in video A,fixed qp or delta qp.
///
/// - **A_ROI_REGION4_QP** `[6:0]` (RW)
///
///   Configure H264 ROI region4 qp in video A,fixed qp or delta qp.
#[repr(transparent)]
pub struct ARoiRegion47Qp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ARoiRegion47Qp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ARoiRegion47Qp {
    // ======== Field Constants ========

    /// **A_ROI_REGION7_QP**
    ///
    /// Configure H264 ROI region7 qp in video A,fixed qp or delta qp.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const A_ROI_REGION7_QP_OFFSET: u32 = 21;
    pub const A_ROI_REGION7_QP_MASK: u32 = 0x0fe00000;

    /// **A_ROI_REGION6_QP**
    ///
    /// Configure H264 ROI region6 qp in video A,fixed qp or delta qp.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const A_ROI_REGION6_QP_OFFSET: u32 = 14;
    pub const A_ROI_REGION6_QP_MASK: u32 = 0x001fc000;

    /// **A_ROI_REGION5_QP**
    ///
    /// Configure H264 ROI region5 qp in video A,fixed qp or delta qp.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const A_ROI_REGION5_QP_OFFSET: u32 = 7;
    pub const A_ROI_REGION5_QP_MASK: u32 = 0x00003f80;

    /// **A_ROI_REGION4_QP**
    ///
    /// Configure H264 ROI region4 qp in video A,fixed qp or delta qp.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const A_ROI_REGION4_QP_OFFSET: u32 = 0;
    pub const A_ROI_REGION4_QP_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **A_ROI_REGION7_QP** `[27:21]` (RW)
    ///
    /// Configure H264 ROI region7 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn a_roi_region7_qp(&self) -> u32 {
        (self.value.read_bits() & Self::A_ROI_REGION7_QP_MASK) >> Self::A_ROI_REGION7_QP_OFFSET
    }

    /// Write **A_ROI_REGION7_QP** `[27:21]` (RW)
    ///
    /// Configure H264 ROI region7 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_a_roi_region7_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_ROI_REGION7_QP_MASK)
                | ((value << Self::A_ROI_REGION7_QP_OFFSET) & Self::A_ROI_REGION7_QP_MASK)
        });
    }

    /// **A_ROI_REGION6_QP** `[20:14]` (RW)
    ///
    /// Configure H264 ROI region6 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn a_roi_region6_qp(&self) -> u32 {
        (self.value.read_bits() & Self::A_ROI_REGION6_QP_MASK) >> Self::A_ROI_REGION6_QP_OFFSET
    }

    /// Write **A_ROI_REGION6_QP** `[20:14]` (RW)
    ///
    /// Configure H264 ROI region6 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_a_roi_region6_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_ROI_REGION6_QP_MASK)
                | ((value << Self::A_ROI_REGION6_QP_OFFSET) & Self::A_ROI_REGION6_QP_MASK)
        });
    }

    /// **A_ROI_REGION5_QP** `[13:7]` (RW)
    ///
    /// Configure H264 ROI region5 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn a_roi_region5_qp(&self) -> u32 {
        (self.value.read_bits() & Self::A_ROI_REGION5_QP_MASK) >> Self::A_ROI_REGION5_QP_OFFSET
    }

    /// Write **A_ROI_REGION5_QP** `[13:7]` (RW)
    ///
    /// Configure H264 ROI region5 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_a_roi_region5_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_ROI_REGION5_QP_MASK)
                | ((value << Self::A_ROI_REGION5_QP_OFFSET) & Self::A_ROI_REGION5_QP_MASK)
        });
    }

    /// **A_ROI_REGION4_QP** `[6:0]` (RW)
    ///
    /// Configure H264 ROI region4 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn a_roi_region4_qp(&self) -> u32 {
        (self.value.read_bits() & Self::A_ROI_REGION4_QP_MASK) >> Self::A_ROI_REGION4_QP_OFFSET
    }

    /// Write **A_ROI_REGION4_QP** `[6:0]` (RW)
    ///
    /// Configure H264 ROI region4 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_a_roi_region4_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_ROI_REGION4_QP_MASK)
                | ((value << Self::A_ROI_REGION4_QP_OFFSET) & Self::A_ROI_REGION4_QP_MASK)
        });
    }
}

/// A_ROI_REGION4_7_QP value type for batch read/modify/write operations
///
/// Use `aroiregion47qp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `aroiregion47qp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ARoiRegion47QpVal {
    bits: u32,
}

impl ARoiRegion47QpVal {
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

    /// Read **A_ROI_REGION7_QP** `[27:21]`
    ///
    /// Configure H264 ROI region7 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn a_roi_region7_qp(&self) -> u32 {
        (self.bits & ARoiRegion47Qp::A_ROI_REGION7_QP_MASK)
            >> ARoiRegion47Qp::A_ROI_REGION7_QP_OFFSET
    }

    /// Read **A_ROI_REGION6_QP** `[20:14]`
    ///
    /// Configure H264 ROI region6 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn a_roi_region6_qp(&self) -> u32 {
        (self.bits & ARoiRegion47Qp::A_ROI_REGION6_QP_MASK)
            >> ARoiRegion47Qp::A_ROI_REGION6_QP_OFFSET
    }

    /// Read **A_ROI_REGION5_QP** `[13:7]`
    ///
    /// Configure H264 ROI region5 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn a_roi_region5_qp(&self) -> u32 {
        (self.bits & ARoiRegion47Qp::A_ROI_REGION5_QP_MASK)
            >> ARoiRegion47Qp::A_ROI_REGION5_QP_OFFSET
    }

    /// Read **A_ROI_REGION4_QP** `[6:0]`
    ///
    /// Configure H264 ROI region4 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn a_roi_region4_qp(&self) -> u32 {
        (self.bits & ARoiRegion47Qp::A_ROI_REGION4_QP_MASK)
            >> ARoiRegion47Qp::A_ROI_REGION4_QP_OFFSET
    }

    /// Set **A_ROI_REGION7_QP** `[27:21]`
    ///
    /// Configure H264 ROI region7 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_a_roi_region7_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion47Qp::A_ROI_REGION7_QP_MASK)
                | ((value << ARoiRegion47Qp::A_ROI_REGION7_QP_OFFSET)
                    & ARoiRegion47Qp::A_ROI_REGION7_QP_MASK),
        }
    }

    /// Set **A_ROI_REGION6_QP** `[20:14]`
    ///
    /// Configure H264 ROI region6 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_a_roi_region6_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion47Qp::A_ROI_REGION6_QP_MASK)
                | ((value << ARoiRegion47Qp::A_ROI_REGION6_QP_OFFSET)
                    & ARoiRegion47Qp::A_ROI_REGION6_QP_MASK),
        }
    }

    /// Set **A_ROI_REGION5_QP** `[13:7]`
    ///
    /// Configure H264 ROI region5 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_a_roi_region5_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion47Qp::A_ROI_REGION5_QP_MASK)
                | ((value << ARoiRegion47Qp::A_ROI_REGION5_QP_OFFSET)
                    & ARoiRegion47Qp::A_ROI_REGION5_QP_MASK),
        }
    }

    /// Set **A_ROI_REGION4_QP** `[6:0]`
    ///
    /// Configure H264 ROI region4 qp in video A,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_a_roi_region4_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ARoiRegion47Qp::A_ROI_REGION4_QP_MASK)
                | ((value << ARoiRegion47Qp::A_ROI_REGION4_QP_OFFSET)
                    & ARoiRegion47Qp::A_ROI_REGION4_QP_MASK),
        }
    }
}

impl Default for ARoiRegion47QpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ARoiRegion47Qp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ARoiRegion47QpVal {
        ARoiRegion47QpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ARoiRegion47QpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ARoiRegion47QpVal) -> ARoiRegion47QpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_NO_ROI_REGION_QP_OFFSET Register
///
/// Video A H264 no roi region QP register.
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **A_NO_ROI_REGION_QP** `[6:0]` (RW)
///
///   Configure H264 no region qp in video A, delta qp.
#[repr(transparent)]
pub struct ANoRoiRegionQpOffset {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ANoRoiRegionQpOffset {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ANoRoiRegionQpOffset {
    // ======== Field Constants ========

    /// **A_NO_ROI_REGION_QP**
    ///
    /// Configure H264 no region qp in video A, delta qp.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const A_NO_ROI_REGION_QP_OFFSET: u32 = 0;
    pub const A_NO_ROI_REGION_QP_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **A_NO_ROI_REGION_QP** `[6:0]` (RW)
    ///
    /// Configure H264 no region qp in video A, delta qp.
    #[inline(always)]
    pub fn a_no_roi_region_qp(&self) -> u32 {
        (self.value.read_bits() & Self::A_NO_ROI_REGION_QP_MASK) >> Self::A_NO_ROI_REGION_QP_OFFSET
    }

    /// Write **A_NO_ROI_REGION_QP** `[6:0]` (RW)
    ///
    /// Configure H264 no region qp in video A, delta qp.
    #[inline(always)]
    pub fn write_a_no_roi_region_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::A_NO_ROI_REGION_QP_MASK)
                | ((value << Self::A_NO_ROI_REGION_QP_OFFSET) & Self::A_NO_ROI_REGION_QP_MASK)
        });
    }
}

/// A_NO_ROI_REGION_QP_OFFSET value type for batch read/modify/write operations
///
/// Use `anoroiregionqpoffset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `anoroiregionqpoffset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ANoRoiRegionQpOffsetVal {
    bits: u32,
}

impl ANoRoiRegionQpOffsetVal {
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

    /// Read **A_NO_ROI_REGION_QP** `[6:0]`
    ///
    /// Configure H264 no region qp in video A, delta qp.
    #[inline(always)]
    pub const fn a_no_roi_region_qp(&self) -> u32 {
        (self.bits & ANoRoiRegionQpOffset::A_NO_ROI_REGION_QP_MASK)
            >> ANoRoiRegionQpOffset::A_NO_ROI_REGION_QP_OFFSET
    }

    /// Set **A_NO_ROI_REGION_QP** `[6:0]`
    ///
    /// Configure H264 no region qp in video A, delta qp.
    #[inline(always)]
    pub const fn with_a_no_roi_region_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ANoRoiRegionQpOffset::A_NO_ROI_REGION_QP_MASK)
                | ((value << ANoRoiRegionQpOffset::A_NO_ROI_REGION_QP_OFFSET)
                    & ANoRoiRegionQpOffset::A_NO_ROI_REGION_QP_MASK),
        }
    }
}

impl Default for ANoRoiRegionQpOffsetVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ANoRoiRegionQpOffset {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ANoRoiRegionQpOffsetVal {
        ANoRoiRegionQpOffsetVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ANoRoiRegionQpOffsetVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ANoRoiRegionQpOffsetVal) -> ANoRoiRegionQpOffsetVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// A_ROI_CONFIG Register
///
/// Video A H264 ROI configure register.
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **A_ROI_MODE** `[1]` (RW)
///
///   Configure the mode of ROI in video A.\\0:fixed qp\\1:delta qp.
///
/// - **A_ROI_EN** `[0]` (RW)
///
///   Configure whether or not to enable ROI in video A.\\0:not enable ROI\\1:enable ROI.
#[repr(transparent)]
pub struct ARoiConfig {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ARoiConfig {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ARoiConfig {
    // ======== Field Constants ========

    /// **A_ROI_MODE**
    ///
    /// Configure the mode of ROI in video A.\\0:fixed qp\\1:delta qp.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const A_ROI_MODE_OFFSET: u32 = 1;
    pub const A_ROI_MODE_MASK: u32 = 0x00000002;
    pub const A_ROI_MODE: u32 = 1 << 1;

    /// **A_ROI_EN**
    ///
    /// Configure whether or not to enable ROI in video A.\\0:not enable ROI\\1:enable ROI.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const A_ROI_EN_OFFSET: u32 = 0;
    pub const A_ROI_EN_MASK: u32 = 0x00000001;
    pub const A_ROI_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **A_ROI_MODE** `[1]` (RW)
    ///
    /// Configure the mode of ROI in video A.\\0:fixed qp\\1:delta qp.
    #[inline(always)]
    pub fn a_roi_mode(&self) -> bool {
        (self.value.read_bits() & Self::A_ROI_MODE_MASK) != 0
    }

    /// Set A_ROI_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_a_roi_mode(&self) {
        self.value.modify_bits(|r| r | Self::A_ROI_MODE_MASK);
    }

    /// Clear A_ROI_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_a_roi_mode(&self) {
        self.value.modify_bits(|r| r & !Self::A_ROI_MODE_MASK);
    }

    /// **A_ROI_EN** `[0]` (RW)
    ///
    /// Configure whether or not to enable ROI in video A.\\0:not enable ROI\\1:enable ROI.
    #[inline(always)]
    pub fn a_roi_en(&self) -> bool {
        (self.value.read_bits() & Self::A_ROI_EN_MASK) != 0
    }

    /// Set A_ROI_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_a_roi_en(&self) {
        self.value.modify_bits(|r| r | Self::A_ROI_EN_MASK);
    }

    /// Clear A_ROI_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_a_roi_en(&self) {
        self.value.modify_bits(|r| r & !Self::A_ROI_EN_MASK);
    }
}

/// A_ROI_CONFIG value type for batch read/modify/write operations
///
/// Use `aroiconfig.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `aroiconfig.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ARoiConfigVal {
    bits: u32,
}

impl ARoiConfigVal {
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

    /// Read **A_ROI_MODE** `[1:1]`
    ///
    /// Configure the mode of ROI in video A.\\0:fixed qp\\1:delta qp.
    #[inline(always)]
    pub const fn a_roi_mode(&self) -> bool {
        (self.bits & ARoiConfig::A_ROI_MODE_MASK) != 0
    }

    /// Read **A_ROI_EN** `[0:0]`
    ///
    /// Configure whether or not to enable ROI in video A.\\0:not enable ROI\\1:enable ROI.
    #[inline(always)]
    pub const fn a_roi_en(&self) -> bool {
        (self.bits & ARoiConfig::A_ROI_EN_MASK) != 0
    }

    /// Set **A_ROI_MODE** `[1:1]`
    ///
    /// Configure the mode of ROI in video A.\\0:fixed qp\\1:delta qp.
    #[inline(always)]
    pub const fn with_a_roi_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ARoiConfig::A_ROI_MODE_MASK
            } else {
                self.bits & !ARoiConfig::A_ROI_MODE_MASK
            },
        }
    }

    /// Set **A_ROI_EN** `[0:0]`
    ///
    /// Configure whether or not to enable ROI in video A.\\0:not enable ROI\\1:enable ROI.
    #[inline(always)]
    pub const fn with_a_roi_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ARoiConfig::A_ROI_EN_MASK
            } else {
                self.bits & !ARoiConfig::A_ROI_EN_MASK
            },
        }
    }
}

impl Default for ARoiConfigVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ARoiConfig {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ARoiConfigVal {
        ARoiConfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ARoiConfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ARoiConfigVal) -> ARoiConfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_SYS_MB_RES Register
///
/// Video B horizontal and vertical MB resolution register.
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **B_SYS_TOTAL_MB_X** `[13:7]` (RW)
///
///   Configures video B horizontal MB resolution.
///
/// - **B_SYS_TOTAL_MB_Y** `[6:0]` (RW)
///
///   Configures video B vertical MB resolution.
#[repr(transparent)]
pub struct BSysMbRes {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BSysMbRes {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BSysMbRes {
    // ======== Field Constants ========

    /// **B_SYS_TOTAL_MB_X**
    ///
    /// Configures video B horizontal MB resolution.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const B_SYS_TOTAL_MB_X_OFFSET: u32 = 7;
    pub const B_SYS_TOTAL_MB_X_MASK: u32 = 0x00003f80;

    /// **B_SYS_TOTAL_MB_Y**
    ///
    /// Configures video B vertical MB resolution.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const B_SYS_TOTAL_MB_Y_OFFSET: u32 = 0;
    pub const B_SYS_TOTAL_MB_Y_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **B_SYS_TOTAL_MB_X** `[13:7]` (RW)
    ///
    /// Configures video B horizontal MB resolution.
    #[inline(always)]
    pub fn b_sys_total_mb_x(&self) -> u32 {
        (self.value.read_bits() & Self::B_SYS_TOTAL_MB_X_MASK) >> Self::B_SYS_TOTAL_MB_X_OFFSET
    }

    /// Write **B_SYS_TOTAL_MB_X** `[13:7]` (RW)
    ///
    /// Configures video B horizontal MB resolution.
    #[inline(always)]
    pub fn write_b_sys_total_mb_x(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_SYS_TOTAL_MB_X_MASK)
                | ((value << Self::B_SYS_TOTAL_MB_X_OFFSET) & Self::B_SYS_TOTAL_MB_X_MASK)
        });
    }

    /// **B_SYS_TOTAL_MB_Y** `[6:0]` (RW)
    ///
    /// Configures video B vertical MB resolution.
    #[inline(always)]
    pub fn b_sys_total_mb_y(&self) -> u32 {
        (self.value.read_bits() & Self::B_SYS_TOTAL_MB_Y_MASK) >> Self::B_SYS_TOTAL_MB_Y_OFFSET
    }

    /// Write **B_SYS_TOTAL_MB_Y** `[6:0]` (RW)
    ///
    /// Configures video B vertical MB resolution.
    #[inline(always)]
    pub fn write_b_sys_total_mb_y(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_SYS_TOTAL_MB_Y_MASK)
                | ((value << Self::B_SYS_TOTAL_MB_Y_OFFSET) & Self::B_SYS_TOTAL_MB_Y_MASK)
        });
    }
}

/// B_SYS_MB_RES value type for batch read/modify/write operations
///
/// Use `bsysmbres.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bsysmbres.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BSysMbResVal {
    bits: u32,
}

impl BSysMbResVal {
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

    /// Read **B_SYS_TOTAL_MB_X** `[13:7]`
    ///
    /// Configures video B horizontal MB resolution.
    #[inline(always)]
    pub const fn b_sys_total_mb_x(&self) -> u32 {
        (self.bits & BSysMbRes::B_SYS_TOTAL_MB_X_MASK) >> BSysMbRes::B_SYS_TOTAL_MB_X_OFFSET
    }

    /// Read **B_SYS_TOTAL_MB_Y** `[6:0]`
    ///
    /// Configures video B vertical MB resolution.
    #[inline(always)]
    pub const fn b_sys_total_mb_y(&self) -> u32 {
        (self.bits & BSysMbRes::B_SYS_TOTAL_MB_Y_MASK) >> BSysMbRes::B_SYS_TOTAL_MB_Y_OFFSET
    }

    /// Set **B_SYS_TOTAL_MB_X** `[13:7]`
    ///
    /// Configures video B horizontal MB resolution.
    #[inline(always)]
    pub const fn with_b_sys_total_mb_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BSysMbRes::B_SYS_TOTAL_MB_X_MASK)
                | ((value << BSysMbRes::B_SYS_TOTAL_MB_X_OFFSET)
                    & BSysMbRes::B_SYS_TOTAL_MB_X_MASK),
        }
    }

    /// Set **B_SYS_TOTAL_MB_Y** `[6:0]`
    ///
    /// Configures video B vertical MB resolution.
    #[inline(always)]
    pub const fn with_b_sys_total_mb_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BSysMbRes::B_SYS_TOTAL_MB_Y_MASK)
                | ((value << BSysMbRes::B_SYS_TOTAL_MB_Y_OFFSET)
                    & BSysMbRes::B_SYS_TOTAL_MB_Y_MASK),
        }
    }
}

impl Default for BSysMbResVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BSysMbRes {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BSysMbResVal {
        BSysMbResVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BSysMbResVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BSysMbResVal) -> BSysMbResVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_SYS_CONF Register
///
/// Video B system level configuration register.
///
/// Address offset: 0x0058
/// Reset value: 0x00000203
///
/// ## Fields
///
/// - **B_INTRA_COST_CMP_OFFSET** `[29:14]` (RW)
///
///   Configures video B intra cost offset when I MB compared with P MB.
///
/// - **B_REC_READY_TRIGGER_MB_LINES** `[13:7]` (RW)
///
///   Configures when to trigger video B H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4.
///
/// - **B_DB_TMP_READY_TRIGGER_MB_NUM** `[6:0]` (RW)
///
///   Configures when to trigger video B H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3.
#[repr(transparent)]
pub struct BSysConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BSysConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BSysConf {
    // ======== Field Constants ========

    /// **B_INTRA_COST_CMP_OFFSET**
    ///
    /// Configures video B intra cost offset when I MB compared with P MB.
    ///
    /// Bits `[29:14]`, mask `0x3fffc000`
    pub const B_INTRA_COST_CMP_OFFSET_OFFSET: u32 = 14;
    pub const B_INTRA_COST_CMP_OFFSET_MASK: u32 = 0x3fffc000;

    /// **B_REC_READY_TRIGGER_MB_LINES**
    ///
    /// Configures when to trigger video B H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const B_REC_READY_TRIGGER_MB_LINES_OFFSET: u32 = 7;
    pub const B_REC_READY_TRIGGER_MB_LINES_MASK: u32 = 0x00003f80;

    /// **B_DB_TMP_READY_TRIGGER_MB_NUM**
    ///
    /// Configures when to trigger video B H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const B_DB_TMP_READY_TRIGGER_MB_NUM_OFFSET: u32 = 0;
    pub const B_DB_TMP_READY_TRIGGER_MB_NUM_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **B_INTRA_COST_CMP_OFFSET** `[29:14]` (RW)
    ///
    /// Configures video B intra cost offset when I MB compared with P MB.
    #[inline(always)]
    pub fn b_intra_cost_cmp_offset(&self) -> u32 {
        (self.value.read_bits() & Self::B_INTRA_COST_CMP_OFFSET_MASK)
            >> Self::B_INTRA_COST_CMP_OFFSET_OFFSET
    }

    /// Write **B_INTRA_COST_CMP_OFFSET** `[29:14]` (RW)
    ///
    /// Configures video B intra cost offset when I MB compared with P MB.
    #[inline(always)]
    pub fn write_b_intra_cost_cmp_offset(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_INTRA_COST_CMP_OFFSET_MASK)
                | ((value << Self::B_INTRA_COST_CMP_OFFSET_OFFSET)
                    & Self::B_INTRA_COST_CMP_OFFSET_MASK)
        });
    }

    /// **B_REC_READY_TRIGGER_MB_LINES** `[13:7]` (RW)
    ///
    /// Configures when to trigger video B H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4.
    #[inline(always)]
    pub fn b_rec_ready_trigger_mb_lines(&self) -> u32 {
        (self.value.read_bits() & Self::B_REC_READY_TRIGGER_MB_LINES_MASK)
            >> Self::B_REC_READY_TRIGGER_MB_LINES_OFFSET
    }

    /// Write **B_REC_READY_TRIGGER_MB_LINES** `[13:7]` (RW)
    ///
    /// Configures when to trigger video B H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4.
    #[inline(always)]
    pub fn write_b_rec_ready_trigger_mb_lines(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_REC_READY_TRIGGER_MB_LINES_MASK)
                | ((value << Self::B_REC_READY_TRIGGER_MB_LINES_OFFSET)
                    & Self::B_REC_READY_TRIGGER_MB_LINES_MASK)
        });
    }

    /// **B_DB_TMP_READY_TRIGGER_MB_NUM** `[6:0]` (RW)
    ///
    /// Configures when to trigger video B H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3.
    #[inline(always)]
    pub fn b_db_tmp_ready_trigger_mb_num(&self) -> u32 {
        (self.value.read_bits() & Self::B_DB_TMP_READY_TRIGGER_MB_NUM_MASK)
            >> Self::B_DB_TMP_READY_TRIGGER_MB_NUM_OFFSET
    }

    /// Write **B_DB_TMP_READY_TRIGGER_MB_NUM** `[6:0]` (RW)
    ///
    /// Configures when to trigger video B H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3.
    #[inline(always)]
    pub fn write_b_db_tmp_ready_trigger_mb_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_DB_TMP_READY_TRIGGER_MB_NUM_MASK)
                | ((value << Self::B_DB_TMP_READY_TRIGGER_MB_NUM_OFFSET)
                    & Self::B_DB_TMP_READY_TRIGGER_MB_NUM_MASK)
        });
    }
}

/// B_SYS_CONF value type for batch read/modify/write operations
///
/// Use `bsysconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bsysconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BSysConfVal {
    bits: u32,
}

impl BSysConfVal {
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
        Self { bits: 0x00000203 }
    }

    /// Read **B_INTRA_COST_CMP_OFFSET** `[29:14]`
    ///
    /// Configures video B intra cost offset when I MB compared with P MB.
    #[inline(always)]
    pub const fn b_intra_cost_cmp_offset(&self) -> u32 {
        (self.bits & BSysConf::B_INTRA_COST_CMP_OFFSET_MASK)
            >> BSysConf::B_INTRA_COST_CMP_OFFSET_OFFSET
    }

    /// Read **B_REC_READY_TRIGGER_MB_LINES** `[13:7]`
    ///
    /// Configures when to trigger video B H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4.
    #[inline(always)]
    pub const fn b_rec_ready_trigger_mb_lines(&self) -> u32 {
        (self.bits & BSysConf::B_REC_READY_TRIGGER_MB_LINES_MASK)
            >> BSysConf::B_REC_READY_TRIGGER_MB_LINES_OFFSET
    }

    /// Read **B_DB_TMP_READY_TRIGGER_MB_NUM** `[6:0]`
    ///
    /// Configures when to trigger video B H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3.
    #[inline(always)]
    pub const fn b_db_tmp_ready_trigger_mb_num(&self) -> u32 {
        (self.bits & BSysConf::B_DB_TMP_READY_TRIGGER_MB_NUM_MASK)
            >> BSysConf::B_DB_TMP_READY_TRIGGER_MB_NUM_OFFSET
    }

    /// Set **B_INTRA_COST_CMP_OFFSET** `[29:14]`
    ///
    /// Configures video B intra cost offset when I MB compared with P MB.
    #[inline(always)]
    pub const fn with_b_intra_cost_cmp_offset(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BSysConf::B_INTRA_COST_CMP_OFFSET_MASK)
                | ((value << BSysConf::B_INTRA_COST_CMP_OFFSET_OFFSET)
                    & BSysConf::B_INTRA_COST_CMP_OFFSET_MASK),
        }
    }

    /// Set **B_REC_READY_TRIGGER_MB_LINES** `[13:7]`
    ///
    /// Configures when to trigger video B H264_REC_READY_INT. When the MB line number of generated reconstruct pixel is greater than this filed, trigger H264_REC_READY_INT. Min is 4.
    #[inline(always)]
    pub const fn with_b_rec_ready_trigger_mb_lines(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BSysConf::B_REC_READY_TRIGGER_MB_LINES_MASK)
                | ((value << BSysConf::B_REC_READY_TRIGGER_MB_LINES_OFFSET)
                    & BSysConf::B_REC_READY_TRIGGER_MB_LINES_MASK),
        }
    }

    /// Set **B_DB_TMP_READY_TRIGGER_MB_NUM** `[6:0]`
    ///
    /// Configures when to trigger video B H264_DB_TMP_READY_INT. When the (MB number of written db temp+1) is greater than this filed in first MB line, trigger H264_DB_TMP_READY_INT. Min is 3.
    #[inline(always)]
    pub const fn with_b_db_tmp_ready_trigger_mb_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BSysConf::B_DB_TMP_READY_TRIGGER_MB_NUM_MASK)
                | ((value << BSysConf::B_DB_TMP_READY_TRIGGER_MB_NUM_OFFSET)
                    & BSysConf::B_DB_TMP_READY_TRIGGER_MB_NUM_MASK),
        }
    }
}

impl Default for BSysConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BSysConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BSysConfVal {
        BSysConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BSysConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BSysConfVal) -> BSysConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_DECI_SCORE Register
///
/// Video B luma and chroma MB decimate score Register.
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **B_L_DECI_SCORE** `[19:10]` (RW)
///
///   Configures video B luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
///
/// - **B_C_DECI_SCORE** `[9:0]` (RW)
///
///   Configures video B chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
#[repr(transparent)]
pub struct BDeciScore {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BDeciScore {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BDeciScore {
    // ======== Field Constants ========

    /// **B_L_DECI_SCORE**
    ///
    /// Configures video B luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
    ///
    /// Bits `[19:10]`, mask `0x000ffc00`
    pub const B_L_DECI_SCORE_OFFSET: u32 = 10;
    pub const B_L_DECI_SCORE_MASK: u32 = 0x000ffc00;

    /// **B_C_DECI_SCORE**
    ///
    /// Configures video B chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const B_C_DECI_SCORE_OFFSET: u32 = 0;
    pub const B_C_DECI_SCORE_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **B_L_DECI_SCORE** `[19:10]` (RW)
    ///
    /// Configures video B luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
    #[inline(always)]
    pub fn b_l_deci_score(&self) -> u32 {
        (self.value.read_bits() & Self::B_L_DECI_SCORE_MASK) >> Self::B_L_DECI_SCORE_OFFSET
    }

    /// Write **B_L_DECI_SCORE** `[19:10]` (RW)
    ///
    /// Configures video B luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
    #[inline(always)]
    pub fn write_b_l_deci_score(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_L_DECI_SCORE_MASK)
                | ((value << Self::B_L_DECI_SCORE_OFFSET) & Self::B_L_DECI_SCORE_MASK)
        });
    }

    /// **B_C_DECI_SCORE** `[9:0]` (RW)
    ///
    /// Configures video B chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
    #[inline(always)]
    pub fn b_c_deci_score(&self) -> u32 {
        (self.value.read_bits() & Self::B_C_DECI_SCORE_MASK) >> Self::B_C_DECI_SCORE_OFFSET
    }

    /// Write **B_C_DECI_SCORE** `[9:0]` (RW)
    ///
    /// Configures video B chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
    #[inline(always)]
    pub fn write_b_c_deci_score(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_C_DECI_SCORE_MASK)
                | ((value << Self::B_C_DECI_SCORE_OFFSET) & Self::B_C_DECI_SCORE_MASK)
        });
    }
}

/// B_DECI_SCORE value type for batch read/modify/write operations
///
/// Use `bdeciscore.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bdeciscore.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BDeciScoreVal {
    bits: u32,
}

impl BDeciScoreVal {
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

    /// Read **B_L_DECI_SCORE** `[19:10]`
    ///
    /// Configures video B luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
    #[inline(always)]
    pub const fn b_l_deci_score(&self) -> u32 {
        (self.bits & BDeciScore::B_L_DECI_SCORE_MASK) >> BDeciScore::B_L_DECI_SCORE_OFFSET
    }

    /// Read **B_C_DECI_SCORE** `[9:0]`
    ///
    /// Configures video B chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
    #[inline(always)]
    pub const fn b_c_deci_score(&self) -> u32 {
        (self.bits & BDeciScore::B_C_DECI_SCORE_MASK) >> BDeciScore::B_C_DECI_SCORE_OFFSET
    }

    /// Set **B_L_DECI_SCORE** `[19:10]`
    ///
    /// Configures video B luma MB decimate score. When luma score is smaller than it, luma decimate will be enable.
    #[inline(always)]
    pub const fn with_b_l_deci_score(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BDeciScore::B_L_DECI_SCORE_MASK)
                | ((value << BDeciScore::B_L_DECI_SCORE_OFFSET) & BDeciScore::B_L_DECI_SCORE_MASK),
        }
    }

    /// Set **B_C_DECI_SCORE** `[9:0]`
    ///
    /// Configures video B chroma MB decimate score. When chroma score is smaller than it, chroma decimate will be enable.
    #[inline(always)]
    pub const fn with_b_c_deci_score(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BDeciScore::B_C_DECI_SCORE_MASK)
                | ((value << BDeciScore::B_C_DECI_SCORE_OFFSET) & BDeciScore::B_C_DECI_SCORE_MASK),
        }
    }
}

impl Default for BDeciScoreVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BDeciScore {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BDeciScoreVal {
        BDeciScoreVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BDeciScoreVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BDeciScoreVal) -> BDeciScoreVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_DECI_SCORE_OFFSET Register
///
/// Video B luma and chroma MB decimate score offset Register.
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **B_P_CHROMA_DECI_SCORE_OFFSET** `[23:18]` (RW)
///
///   Configures video B p chroma MB decimate score offset. This offset will be added to p chroma MB score.
///
/// - **B_P16X16_DECI_SCORE_OFFSET** `[17:12]` (RW)
///
///   Configures video B p16x16 MB decimate score offset. This offset will be added to p16x16 MB score.
///
/// - **B_I_CHROMA_DECI_SCORE_OFFSET** `[11:6]` (RW)
///
///   Configures video B I chroma MB decimate score offset. This offset will be added to I chroma MB score.
///
/// - **B_I16X16_DECI_SCORE_OFFSET** `[5:0]` (RW)
///
///   Configures video B i16x16 MB decimate score offset. This offset will be added to i16x16 MB score.
#[repr(transparent)]
pub struct BDeciScoreOffset {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BDeciScoreOffset {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BDeciScoreOffset {
    // ======== Field Constants ========

    /// **B_P_CHROMA_DECI_SCORE_OFFSET**
    ///
    /// Configures video B p chroma MB decimate score offset. This offset will be added to p chroma MB score.
    ///
    /// Bits `[23:18]`, mask `0x00fc0000`
    pub const B_P_CHROMA_DECI_SCORE_OFFSET_OFFSET: u32 = 18;
    pub const B_P_CHROMA_DECI_SCORE_OFFSET_MASK: u32 = 0x00fc0000;

    /// **B_P16X16_DECI_SCORE_OFFSET**
    ///
    /// Configures video B p16x16 MB decimate score offset. This offset will be added to p16x16 MB score.
    ///
    /// Bits `[17:12]`, mask `0x0003f000`
    pub const B_P16X16_DECI_SCORE_OFFSET_OFFSET: u32 = 12;
    pub const B_P16X16_DECI_SCORE_OFFSET_MASK: u32 = 0x0003f000;

    /// **B_I_CHROMA_DECI_SCORE_OFFSET**
    ///
    /// Configures video B I chroma MB decimate score offset. This offset will be added to I chroma MB score.
    ///
    /// Bits `[11:6]`, mask `0x00000fc0`
    pub const B_I_CHROMA_DECI_SCORE_OFFSET_OFFSET: u32 = 6;
    pub const B_I_CHROMA_DECI_SCORE_OFFSET_MASK: u32 = 0x00000fc0;

    /// **B_I16X16_DECI_SCORE_OFFSET**
    ///
    /// Configures video B i16x16 MB decimate score offset. This offset will be added to i16x16 MB score.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const B_I16X16_DECI_SCORE_OFFSET_OFFSET: u32 = 0;
    pub const B_I16X16_DECI_SCORE_OFFSET_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **B_P_CHROMA_DECI_SCORE_OFFSET** `[23:18]` (RW)
    ///
    /// Configures video B p chroma MB decimate score offset. This offset will be added to p chroma MB score.
    #[inline(always)]
    pub fn b_p_chroma_deci_score_offset(&self) -> u32 {
        (self.value.read_bits() & Self::B_P_CHROMA_DECI_SCORE_OFFSET_MASK)
            >> Self::B_P_CHROMA_DECI_SCORE_OFFSET_OFFSET
    }

    /// Write **B_P_CHROMA_DECI_SCORE_OFFSET** `[23:18]` (RW)
    ///
    /// Configures video B p chroma MB decimate score offset. This offset will be added to p chroma MB score.
    #[inline(always)]
    pub fn write_b_p_chroma_deci_score_offset(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_P_CHROMA_DECI_SCORE_OFFSET_MASK)
                | ((value << Self::B_P_CHROMA_DECI_SCORE_OFFSET_OFFSET)
                    & Self::B_P_CHROMA_DECI_SCORE_OFFSET_MASK)
        });
    }

    /// **B_P16X16_DECI_SCORE_OFFSET** `[17:12]` (RW)
    ///
    /// Configures video B p16x16 MB decimate score offset. This offset will be added to p16x16 MB score.
    #[inline(always)]
    pub fn b_p16x16_deci_score_offset(&self) -> u32 {
        (self.value.read_bits() & Self::B_P16X16_DECI_SCORE_OFFSET_MASK)
            >> Self::B_P16X16_DECI_SCORE_OFFSET_OFFSET
    }

    /// Write **B_P16X16_DECI_SCORE_OFFSET** `[17:12]` (RW)
    ///
    /// Configures video B p16x16 MB decimate score offset. This offset will be added to p16x16 MB score.
    #[inline(always)]
    pub fn write_b_p16x16_deci_score_offset(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_P16X16_DECI_SCORE_OFFSET_MASK)
                | ((value << Self::B_P16X16_DECI_SCORE_OFFSET_OFFSET)
                    & Self::B_P16X16_DECI_SCORE_OFFSET_MASK)
        });
    }

    /// **B_I_CHROMA_DECI_SCORE_OFFSET** `[11:6]` (RW)
    ///
    /// Configures video B I chroma MB decimate score offset. This offset will be added to I chroma MB score.
    #[inline(always)]
    pub fn b_i_chroma_deci_score_offset(&self) -> u32 {
        (self.value.read_bits() & Self::B_I_CHROMA_DECI_SCORE_OFFSET_MASK)
            >> Self::B_I_CHROMA_DECI_SCORE_OFFSET_OFFSET
    }

    /// Write **B_I_CHROMA_DECI_SCORE_OFFSET** `[11:6]` (RW)
    ///
    /// Configures video B I chroma MB decimate score offset. This offset will be added to I chroma MB score.
    #[inline(always)]
    pub fn write_b_i_chroma_deci_score_offset(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_I_CHROMA_DECI_SCORE_OFFSET_MASK)
                | ((value << Self::B_I_CHROMA_DECI_SCORE_OFFSET_OFFSET)
                    & Self::B_I_CHROMA_DECI_SCORE_OFFSET_MASK)
        });
    }

    /// **B_I16X16_DECI_SCORE_OFFSET** `[5:0]` (RW)
    ///
    /// Configures video B i16x16 MB decimate score offset. This offset will be added to i16x16 MB score.
    #[inline(always)]
    pub fn b_i16x16_deci_score_offset(&self) -> u32 {
        (self.value.read_bits() & Self::B_I16X16_DECI_SCORE_OFFSET_MASK)
            >> Self::B_I16X16_DECI_SCORE_OFFSET_OFFSET
    }

    /// Write **B_I16X16_DECI_SCORE_OFFSET** `[5:0]` (RW)
    ///
    /// Configures video B i16x16 MB decimate score offset. This offset will be added to i16x16 MB score.
    #[inline(always)]
    pub fn write_b_i16x16_deci_score_offset(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_I16X16_DECI_SCORE_OFFSET_MASK)
                | ((value << Self::B_I16X16_DECI_SCORE_OFFSET_OFFSET)
                    & Self::B_I16X16_DECI_SCORE_OFFSET_MASK)
        });
    }
}

/// B_DECI_SCORE_OFFSET value type for batch read/modify/write operations
///
/// Use `bdeciscoreoffset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bdeciscoreoffset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BDeciScoreOffsetVal {
    bits: u32,
}

impl BDeciScoreOffsetVal {
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

    /// Read **B_P_CHROMA_DECI_SCORE_OFFSET** `[23:18]`
    ///
    /// Configures video B p chroma MB decimate score offset. This offset will be added to p chroma MB score.
    #[inline(always)]
    pub const fn b_p_chroma_deci_score_offset(&self) -> u32 {
        (self.bits & BDeciScoreOffset::B_P_CHROMA_DECI_SCORE_OFFSET_MASK)
            >> BDeciScoreOffset::B_P_CHROMA_DECI_SCORE_OFFSET_OFFSET
    }

    /// Read **B_P16X16_DECI_SCORE_OFFSET** `[17:12]`
    ///
    /// Configures video B p16x16 MB decimate score offset. This offset will be added to p16x16 MB score.
    #[inline(always)]
    pub const fn b_p16x16_deci_score_offset(&self) -> u32 {
        (self.bits & BDeciScoreOffset::B_P16X16_DECI_SCORE_OFFSET_MASK)
            >> BDeciScoreOffset::B_P16X16_DECI_SCORE_OFFSET_OFFSET
    }

    /// Read **B_I_CHROMA_DECI_SCORE_OFFSET** `[11:6]`
    ///
    /// Configures video B I chroma MB decimate score offset. This offset will be added to I chroma MB score.
    #[inline(always)]
    pub const fn b_i_chroma_deci_score_offset(&self) -> u32 {
        (self.bits & BDeciScoreOffset::B_I_CHROMA_DECI_SCORE_OFFSET_MASK)
            >> BDeciScoreOffset::B_I_CHROMA_DECI_SCORE_OFFSET_OFFSET
    }

    /// Read **B_I16X16_DECI_SCORE_OFFSET** `[5:0]`
    ///
    /// Configures video B i16x16 MB decimate score offset. This offset will be added to i16x16 MB score.
    #[inline(always)]
    pub const fn b_i16x16_deci_score_offset(&self) -> u32 {
        (self.bits & BDeciScoreOffset::B_I16X16_DECI_SCORE_OFFSET_MASK)
            >> BDeciScoreOffset::B_I16X16_DECI_SCORE_OFFSET_OFFSET
    }

    /// Set **B_P_CHROMA_DECI_SCORE_OFFSET** `[23:18]`
    ///
    /// Configures video B p chroma MB decimate score offset. This offset will be added to p chroma MB score.
    #[inline(always)]
    pub const fn with_b_p_chroma_deci_score_offset(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BDeciScoreOffset::B_P_CHROMA_DECI_SCORE_OFFSET_MASK)
                | ((value << BDeciScoreOffset::B_P_CHROMA_DECI_SCORE_OFFSET_OFFSET)
                    & BDeciScoreOffset::B_P_CHROMA_DECI_SCORE_OFFSET_MASK),
        }
    }

    /// Set **B_P16X16_DECI_SCORE_OFFSET** `[17:12]`
    ///
    /// Configures video B p16x16 MB decimate score offset. This offset will be added to p16x16 MB score.
    #[inline(always)]
    pub const fn with_b_p16x16_deci_score_offset(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BDeciScoreOffset::B_P16X16_DECI_SCORE_OFFSET_MASK)
                | ((value << BDeciScoreOffset::B_P16X16_DECI_SCORE_OFFSET_OFFSET)
                    & BDeciScoreOffset::B_P16X16_DECI_SCORE_OFFSET_MASK),
        }
    }

    /// Set **B_I_CHROMA_DECI_SCORE_OFFSET** `[11:6]`
    ///
    /// Configures video B I chroma MB decimate score offset. This offset will be added to I chroma MB score.
    #[inline(always)]
    pub const fn with_b_i_chroma_deci_score_offset(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BDeciScoreOffset::B_I_CHROMA_DECI_SCORE_OFFSET_MASK)
                | ((value << BDeciScoreOffset::B_I_CHROMA_DECI_SCORE_OFFSET_OFFSET)
                    & BDeciScoreOffset::B_I_CHROMA_DECI_SCORE_OFFSET_MASK),
        }
    }

    /// Set **B_I16X16_DECI_SCORE_OFFSET** `[5:0]`
    ///
    /// Configures video B i16x16 MB decimate score offset. This offset will be added to i16x16 MB score.
    #[inline(always)]
    pub const fn with_b_i16x16_deci_score_offset(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BDeciScoreOffset::B_I16X16_DECI_SCORE_OFFSET_MASK)
                | ((value << BDeciScoreOffset::B_I16X16_DECI_SCORE_OFFSET_OFFSET)
                    & BDeciScoreOffset::B_I16X16_DECI_SCORE_OFFSET_MASK),
        }
    }
}

impl Default for BDeciScoreOffsetVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BDeciScoreOffset {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BDeciScoreOffsetVal {
        BDeciScoreOffsetVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BDeciScoreOffsetVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BDeciScoreOffsetVal) -> BDeciScoreOffsetVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_RC_CONF0 Register
///
/// Video B rate control configuration register0.
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **B_MB_RATE_CTRL_EN** `[22]` (RW)
///
///   Configures video A whether or not to open macro block rate ctrl.\\1:Open the macro block rate ctrl\\1:Close the macro block rate ctrl.
///
/// - **B_RATE_CTRL_U** `[21:6]` (RW)
///
///   Configures video B parameter U value. U = int((float) u << 8).
///
/// - **B_QP** `[5:0]` (RW)
///
///   Configures video B frame level initial luma QP value.
#[repr(transparent)]
pub struct BRcConf0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BRcConf0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BRcConf0 {
    // ======== Field Constants ========

    /// **B_MB_RATE_CTRL_EN**
    ///
    /// Configures video A whether or not to open macro block rate ctrl.\\1:Open the macro block rate ctrl\\1:Close the macro block rate ctrl.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const B_MB_RATE_CTRL_EN_OFFSET: u32 = 22;
    pub const B_MB_RATE_CTRL_EN_MASK: u32 = 0x00400000;
    pub const B_MB_RATE_CTRL_EN: u32 = 1 << 22;

    /// **B_RATE_CTRL_U**
    ///
    /// Configures video B parameter U value. U = int((float) u << 8).
    ///
    /// Bits `[21:6]`, mask `0x003fffc0`
    pub const B_RATE_CTRL_U_OFFSET: u32 = 6;
    pub const B_RATE_CTRL_U_MASK: u32 = 0x003fffc0;

    /// **B_QP**
    ///
    /// Configures video B frame level initial luma QP value.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const B_QP_OFFSET: u32 = 0;
    pub const B_QP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **B_MB_RATE_CTRL_EN** `[22]` (RW)
    ///
    /// Configures video A whether or not to open macro block rate ctrl.\\1:Open the macro block rate ctrl\\1:Close the macro block rate ctrl.
    #[inline(always)]
    pub fn b_mb_rate_ctrl_en(&self) -> bool {
        (self.value.read_bits() & Self::B_MB_RATE_CTRL_EN_MASK) != 0
    }

    /// Set B_MB_RATE_CTRL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_b_mb_rate_ctrl_en(&self) {
        self.value.modify_bits(|r| r | Self::B_MB_RATE_CTRL_EN_MASK);
    }

    /// Clear B_MB_RATE_CTRL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_b_mb_rate_ctrl_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::B_MB_RATE_CTRL_EN_MASK);
    }

    /// **B_RATE_CTRL_U** `[21:6]` (RW)
    ///
    /// Configures video B parameter U value. U = int((float) u << 8).
    #[inline(always)]
    pub fn b_rate_ctrl_u(&self) -> u32 {
        (self.value.read_bits() & Self::B_RATE_CTRL_U_MASK) >> Self::B_RATE_CTRL_U_OFFSET
    }

    /// Write **B_RATE_CTRL_U** `[21:6]` (RW)
    ///
    /// Configures video B parameter U value. U = int((float) u << 8).
    #[inline(always)]
    pub fn write_b_rate_ctrl_u(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_RATE_CTRL_U_MASK)
                | ((value << Self::B_RATE_CTRL_U_OFFSET) & Self::B_RATE_CTRL_U_MASK)
        });
    }

    /// **B_QP** `[5:0]` (RW)
    ///
    /// Configures video B frame level initial luma QP value.
    #[inline(always)]
    pub fn b_qp(&self) -> u32 {
        (self.value.read_bits() & Self::B_QP_MASK) >> Self::B_QP_OFFSET
    }

    /// Write **B_QP** `[5:0]` (RW)
    ///
    /// Configures video B frame level initial luma QP value.
    #[inline(always)]
    pub fn write_b_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_QP_MASK) | ((value << Self::B_QP_OFFSET) & Self::B_QP_MASK)
        });
    }
}

/// B_RC_CONF0 value type for batch read/modify/write operations
///
/// Use `brcconf0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `brcconf0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BRcConf0Val {
    bits: u32,
}

impl BRcConf0Val {
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

    /// Read **B_MB_RATE_CTRL_EN** `[22:22]`
    ///
    /// Configures video A whether or not to open macro block rate ctrl.\\1:Open the macro block rate ctrl\\1:Close the macro block rate ctrl.
    #[inline(always)]
    pub const fn b_mb_rate_ctrl_en(&self) -> bool {
        (self.bits & BRcConf0::B_MB_RATE_CTRL_EN_MASK) != 0
    }

    /// Read **B_RATE_CTRL_U** `[21:6]`
    ///
    /// Configures video B parameter U value. U = int((float) u << 8).
    #[inline(always)]
    pub const fn b_rate_ctrl_u(&self) -> u32 {
        (self.bits & BRcConf0::B_RATE_CTRL_U_MASK) >> BRcConf0::B_RATE_CTRL_U_OFFSET
    }

    /// Read **B_QP** `[5:0]`
    ///
    /// Configures video B frame level initial luma QP value.
    #[inline(always)]
    pub const fn b_qp(&self) -> u32 {
        (self.bits & BRcConf0::B_QP_MASK) >> BRcConf0::B_QP_OFFSET
    }

    /// Set **B_MB_RATE_CTRL_EN** `[22:22]`
    ///
    /// Configures video A whether or not to open macro block rate ctrl.\\1:Open the macro block rate ctrl\\1:Close the macro block rate ctrl.
    #[inline(always)]
    pub const fn with_b_mb_rate_ctrl_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BRcConf0::B_MB_RATE_CTRL_EN_MASK
            } else {
                self.bits & !BRcConf0::B_MB_RATE_CTRL_EN_MASK
            },
        }
    }

    /// Set **B_RATE_CTRL_U** `[21:6]`
    ///
    /// Configures video B parameter U value. U = int((float) u << 8).
    #[inline(always)]
    pub const fn with_b_rate_ctrl_u(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRcConf0::B_RATE_CTRL_U_MASK)
                | ((value << BRcConf0::B_RATE_CTRL_U_OFFSET) & BRcConf0::B_RATE_CTRL_U_MASK),
        }
    }

    /// Set **B_QP** `[5:0]`
    ///
    /// Configures video B frame level initial luma QP value.
    #[inline(always)]
    pub const fn with_b_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRcConf0::B_QP_MASK)
                | ((value << BRcConf0::B_QP_OFFSET) & BRcConf0::B_QP_MASK),
        }
    }
}

impl Default for BRcConf0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BRcConf0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BRcConf0Val {
        BRcConf0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BRcConf0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BRcConf0Val) -> BRcConf0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_RC_CONF1 Register
///
/// Video B rate control configuration register1.
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **B_MAD_FRAME_PRED** `[30:19]` (RW)
///
///   Configures vdieo B frame level predicted MB MAD value.
///
/// - **B_QP_MAX** `[18:13]` (RW)
///
///   Configures video B allowed luma QP max value.
///
/// - **B_QP_MIN** `[12:7]` (RW)
///
///   Configures video B allowed luma QP min value.
///
/// - **B_CHROMA_QP_DELTA** `[6:3]` (RW)
///
///   Configures video B chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta.
///
/// - **B_CHROMA_DC_QP_DELTA** `[2:0]` (RW)
///
///   Configures video B chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta.
#[repr(transparent)]
pub struct BRcConf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BRcConf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BRcConf1 {
    // ======== Field Constants ========

    /// **B_MAD_FRAME_PRED**
    ///
    /// Configures vdieo B frame level predicted MB MAD value.
    ///
    /// Bits `[30:19]`, mask `0x7ff80000`
    pub const B_MAD_FRAME_PRED_OFFSET: u32 = 19;
    pub const B_MAD_FRAME_PRED_MASK: u32 = 0x7ff80000;

    /// **B_QP_MAX**
    ///
    /// Configures video B allowed luma QP max value.
    ///
    /// Bits `[18:13]`, mask `0x0007e000`
    pub const B_QP_MAX_OFFSET: u32 = 13;
    pub const B_QP_MAX_MASK: u32 = 0x0007e000;

    /// **B_QP_MIN**
    ///
    /// Configures video B allowed luma QP min value.
    ///
    /// Bits `[12:7]`, mask `0x00001f80`
    pub const B_QP_MIN_OFFSET: u32 = 7;
    pub const B_QP_MIN_MASK: u32 = 0x00001f80;

    /// **B_CHROMA_QP_DELTA**
    ///
    /// Configures video B chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta.
    ///
    /// Bits `[6:3]`, mask `0x00000078`
    pub const B_CHROMA_QP_DELTA_OFFSET: u32 = 3;
    pub const B_CHROMA_QP_DELTA_MASK: u32 = 0x00000078;

    /// **B_CHROMA_DC_QP_DELTA**
    ///
    /// Configures video B chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const B_CHROMA_DC_QP_DELTA_OFFSET: u32 = 0;
    pub const B_CHROMA_DC_QP_DELTA_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **B_MAD_FRAME_PRED** `[30:19]` (RW)
    ///
    /// Configures vdieo B frame level predicted MB MAD value.
    #[inline(always)]
    pub fn b_mad_frame_pred(&self) -> u32 {
        (self.value.read_bits() & Self::B_MAD_FRAME_PRED_MASK) >> Self::B_MAD_FRAME_PRED_OFFSET
    }

    /// Write **B_MAD_FRAME_PRED** `[30:19]` (RW)
    ///
    /// Configures vdieo B frame level predicted MB MAD value.
    #[inline(always)]
    pub fn write_b_mad_frame_pred(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_MAD_FRAME_PRED_MASK)
                | ((value << Self::B_MAD_FRAME_PRED_OFFSET) & Self::B_MAD_FRAME_PRED_MASK)
        });
    }

    /// **B_QP_MAX** `[18:13]` (RW)
    ///
    /// Configures video B allowed luma QP max value.
    #[inline(always)]
    pub fn b_qp_max(&self) -> u32 {
        (self.value.read_bits() & Self::B_QP_MAX_MASK) >> Self::B_QP_MAX_OFFSET
    }

    /// Write **B_QP_MAX** `[18:13]` (RW)
    ///
    /// Configures video B allowed luma QP max value.
    #[inline(always)]
    pub fn write_b_qp_max(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_QP_MAX_MASK) | ((value << Self::B_QP_MAX_OFFSET) & Self::B_QP_MAX_MASK)
        });
    }

    /// **B_QP_MIN** `[12:7]` (RW)
    ///
    /// Configures video B allowed luma QP min value.
    #[inline(always)]
    pub fn b_qp_min(&self) -> u32 {
        (self.value.read_bits() & Self::B_QP_MIN_MASK) >> Self::B_QP_MIN_OFFSET
    }

    /// Write **B_QP_MIN** `[12:7]` (RW)
    ///
    /// Configures video B allowed luma QP min value.
    #[inline(always)]
    pub fn write_b_qp_min(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_QP_MIN_MASK) | ((value << Self::B_QP_MIN_OFFSET) & Self::B_QP_MIN_MASK)
        });
    }

    /// **B_CHROMA_QP_DELTA** `[6:3]` (RW)
    ///
    /// Configures video B chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta.
    #[inline(always)]
    pub fn b_chroma_qp_delta(&self) -> u32 {
        (self.value.read_bits() & Self::B_CHROMA_QP_DELTA_MASK) >> Self::B_CHROMA_QP_DELTA_OFFSET
    }

    /// Write **B_CHROMA_QP_DELTA** `[6:3]` (RW)
    ///
    /// Configures video B chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta.
    #[inline(always)]
    pub fn write_b_chroma_qp_delta(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_CHROMA_QP_DELTA_MASK)
                | ((value << Self::B_CHROMA_QP_DELTA_OFFSET) & Self::B_CHROMA_QP_DELTA_MASK)
        });
    }

    /// **B_CHROMA_DC_QP_DELTA** `[2:0]` (RW)
    ///
    /// Configures video B chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta.
    #[inline(always)]
    pub fn b_chroma_dc_qp_delta(&self) -> u32 {
        (self.value.read_bits() & Self::B_CHROMA_DC_QP_DELTA_MASK)
            >> Self::B_CHROMA_DC_QP_DELTA_OFFSET
    }

    /// Write **B_CHROMA_DC_QP_DELTA** `[2:0]` (RW)
    ///
    /// Configures video B chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta.
    #[inline(always)]
    pub fn write_b_chroma_dc_qp_delta(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_CHROMA_DC_QP_DELTA_MASK)
                | ((value << Self::B_CHROMA_DC_QP_DELTA_OFFSET) & Self::B_CHROMA_DC_QP_DELTA_MASK)
        });
    }
}

/// B_RC_CONF1 value type for batch read/modify/write operations
///
/// Use `brcconf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `brcconf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BRcConf1Val {
    bits: u32,
}

impl BRcConf1Val {
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

    /// Read **B_MAD_FRAME_PRED** `[30:19]`
    ///
    /// Configures vdieo B frame level predicted MB MAD value.
    #[inline(always)]
    pub const fn b_mad_frame_pred(&self) -> u32 {
        (self.bits & BRcConf1::B_MAD_FRAME_PRED_MASK) >> BRcConf1::B_MAD_FRAME_PRED_OFFSET
    }

    /// Read **B_QP_MAX** `[18:13]`
    ///
    /// Configures video B allowed luma QP max value.
    #[inline(always)]
    pub const fn b_qp_max(&self) -> u32 {
        (self.bits & BRcConf1::B_QP_MAX_MASK) >> BRcConf1::B_QP_MAX_OFFSET
    }

    /// Read **B_QP_MIN** `[12:7]`
    ///
    /// Configures video B allowed luma QP min value.
    #[inline(always)]
    pub const fn b_qp_min(&self) -> u32 {
        (self.bits & BRcConf1::B_QP_MIN_MASK) >> BRcConf1::B_QP_MIN_OFFSET
    }

    /// Read **B_CHROMA_QP_DELTA** `[6:3]`
    ///
    /// Configures video B chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta.
    #[inline(always)]
    pub const fn b_chroma_qp_delta(&self) -> u32 {
        (self.bits & BRcConf1::B_CHROMA_QP_DELTA_MASK) >> BRcConf1::B_CHROMA_QP_DELTA_OFFSET
    }

    /// Read **B_CHROMA_DC_QP_DELTA** `[2:0]`
    ///
    /// Configures video B chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta.
    #[inline(always)]
    pub const fn b_chroma_dc_qp_delta(&self) -> u32 {
        (self.bits & BRcConf1::B_CHROMA_DC_QP_DELTA_MASK) >> BRcConf1::B_CHROMA_DC_QP_DELTA_OFFSET
    }

    /// Set **B_MAD_FRAME_PRED** `[30:19]`
    ///
    /// Configures vdieo B frame level predicted MB MAD value.
    #[inline(always)]
    pub const fn with_b_mad_frame_pred(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRcConf1::B_MAD_FRAME_PRED_MASK)
                | ((value << BRcConf1::B_MAD_FRAME_PRED_OFFSET) & BRcConf1::B_MAD_FRAME_PRED_MASK),
        }
    }

    /// Set **B_QP_MAX** `[18:13]`
    ///
    /// Configures video B allowed luma QP max value.
    #[inline(always)]
    pub const fn with_b_qp_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRcConf1::B_QP_MAX_MASK)
                | ((value << BRcConf1::B_QP_MAX_OFFSET) & BRcConf1::B_QP_MAX_MASK),
        }
    }

    /// Set **B_QP_MIN** `[12:7]`
    ///
    /// Configures video B allowed luma QP min value.
    #[inline(always)]
    pub const fn with_b_qp_min(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRcConf1::B_QP_MIN_MASK)
                | ((value << BRcConf1::B_QP_MIN_OFFSET) & BRcConf1::B_QP_MIN_MASK),
        }
    }

    /// Set **B_CHROMA_QP_DELTA** `[6:3]`
    ///
    /// Configures video B chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta.
    #[inline(always)]
    pub const fn with_b_chroma_qp_delta(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRcConf1::B_CHROMA_QP_DELTA_MASK)
                | ((value << BRcConf1::B_CHROMA_QP_DELTA_OFFSET)
                    & BRcConf1::B_CHROMA_QP_DELTA_MASK),
        }
    }

    /// Set **B_CHROMA_DC_QP_DELTA** `[2:0]`
    ///
    /// Configures video B chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta.
    #[inline(always)]
    pub const fn with_b_chroma_dc_qp_delta(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRcConf1::B_CHROMA_DC_QP_DELTA_MASK)
                | ((value << BRcConf1::B_CHROMA_DC_QP_DELTA_OFFSET)
                    & BRcConf1::B_CHROMA_DC_QP_DELTA_MASK),
        }
    }
}

impl Default for BRcConf1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BRcConf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BRcConf1Val {
        BRcConf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BRcConf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BRcConf1Val) -> BRcConf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_DB_BYPASS Register
///
/// Video B Deblocking bypass register
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **B_BYPASS_DB_FILTER** `[0]` (RW)
///
///   Configures whether or not to bypass video B deblcoking filter. \\0: Open the deblock filter\\1: Close the deblock filter
#[repr(transparent)]
pub struct BDbBypass {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BDbBypass {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BDbBypass {
    // ======== Field Constants ========

    /// **B_BYPASS_DB_FILTER**
    ///
    /// Configures whether or not to bypass video B deblcoking filter. \\0: Open the deblock filter\\1: Close the deblock filter
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const B_BYPASS_DB_FILTER_OFFSET: u32 = 0;
    pub const B_BYPASS_DB_FILTER_MASK: u32 = 0x00000001;
    pub const B_BYPASS_DB_FILTER: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **B_BYPASS_DB_FILTER** `[0]` (RW)
    ///
    /// Configures whether or not to bypass video B deblcoking filter. \\0: Open the deblock filter\\1: Close the deblock filter
    #[inline(always)]
    pub fn b_bypass_db_filter(&self) -> bool {
        (self.value.read_bits() & Self::B_BYPASS_DB_FILTER_MASK) != 0
    }

    /// Set B_BYPASS_DB_FILTER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_b_bypass_db_filter(&self) {
        self.value
            .modify_bits(|r| r | Self::B_BYPASS_DB_FILTER_MASK);
    }

    /// Clear B_BYPASS_DB_FILTER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_b_bypass_db_filter(&self) {
        self.value
            .modify_bits(|r| r & !Self::B_BYPASS_DB_FILTER_MASK);
    }
}

/// B_DB_BYPASS value type for batch read/modify/write operations
///
/// Use `bdbbypass.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bdbbypass.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BDbBypassVal {
    bits: u32,
}

impl BDbBypassVal {
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

    /// Read **B_BYPASS_DB_FILTER** `[0:0]`
    ///
    /// Configures whether or not to bypass video B deblcoking filter. \\0: Open the deblock filter\\1: Close the deblock filter
    #[inline(always)]
    pub const fn b_bypass_db_filter(&self) -> bool {
        (self.bits & BDbBypass::B_BYPASS_DB_FILTER_MASK) != 0
    }

    /// Set **B_BYPASS_DB_FILTER** `[0:0]`
    ///
    /// Configures whether or not to bypass video B deblcoking filter. \\0: Open the deblock filter\\1: Close the deblock filter
    #[inline(always)]
    pub const fn with_b_bypass_db_filter(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BDbBypass::B_BYPASS_DB_FILTER_MASK
            } else {
                self.bits & !BDbBypass::B_BYPASS_DB_FILTER_MASK
            },
        }
    }
}

impl Default for BDbBypassVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BDbBypass {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BDbBypassVal {
        BDbBypassVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BDbBypassVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BDbBypassVal) -> BDbBypassVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_ROI_REGION0 Register
///
/// Video B H264 ROI region0 range configure register.
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video B ROI of region 0 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 0 in Video B.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 0 in Video B.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 0 in Video B.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontal start macroblocks of region 0 in Video B.
#[repr(transparent)]
pub struct BRoiRegion0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BRoiRegion0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BRoiRegion0 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video B ROI of region 0 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 0 in Video B.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 0 in Video B.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 0 in Video B.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontal start macroblocks of region 0 in Video B.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video B ROI of region 0 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 0 in Video B.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 0 in Video B.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 0 in Video B.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 0 in Video B.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 0 in Video B.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 0 in Video B.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 0 in Video B.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 0 in Video B.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// B_ROI_REGION0 value type for batch read/modify/write operations
///
/// Use `broiregion0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `broiregion0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BRoiRegion0Val {
    bits: u32,
}

impl BRoiRegion0Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 0 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & BRoiRegion0::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 0 in Video B.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & BRoiRegion0::Y_LEN_MASK) >> BRoiRegion0::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 0 in Video B.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & BRoiRegion0::X_LEN_MASK) >> BRoiRegion0::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 0 in Video B.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & BRoiRegion0::Y_MASK) >> BRoiRegion0::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 0 in Video B.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & BRoiRegion0::X_MASK) >> BRoiRegion0::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 0 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BRoiRegion0::EN_MASK
            } else {
                self.bits & !BRoiRegion0::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 0 in Video B.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion0::Y_LEN_MASK)
                | ((value << BRoiRegion0::Y_LEN_OFFSET) & BRoiRegion0::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 0 in Video B.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion0::X_LEN_MASK)
                | ((value << BRoiRegion0::X_LEN_OFFSET) & BRoiRegion0::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 0 in Video B.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion0::Y_MASK)
                | ((value << BRoiRegion0::Y_OFFSET) & BRoiRegion0::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 0 in Video B.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion0::X_MASK)
                | ((value << BRoiRegion0::X_OFFSET) & BRoiRegion0::X_MASK),
        }
    }
}

impl Default for BRoiRegion0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BRoiRegion0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BRoiRegion0Val {
        BRoiRegion0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BRoiRegion0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BRoiRegion0Val) -> BRoiRegion0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_ROI_REGION1 Register
///
/// Video B H264 ROI region1 range configure register.
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video B ROI of region 1 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 1 in Video B.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 1 in Video B.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 1 in Video B.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontal start macroblocks of region 1 in Video B.
#[repr(transparent)]
pub struct BRoiRegion1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BRoiRegion1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BRoiRegion1 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video B ROI of region 1 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 1 in Video B.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 1 in Video B.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 1 in Video B.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontal start macroblocks of region 1 in Video B.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video B ROI of region 1 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 1 in Video B.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 1 in Video B.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 1 in Video B.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 1 in Video B.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 1 in Video B.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 1 in Video B.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 1 in Video B.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 1 in Video B.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// B_ROI_REGION1 value type for batch read/modify/write operations
///
/// Use `broiregion1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `broiregion1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BRoiRegion1Val {
    bits: u32,
}

impl BRoiRegion1Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 1 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & BRoiRegion1::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 1 in Video B.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & BRoiRegion1::Y_LEN_MASK) >> BRoiRegion1::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 1 in Video B.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & BRoiRegion1::X_LEN_MASK) >> BRoiRegion1::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 1 in Video B.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & BRoiRegion1::Y_MASK) >> BRoiRegion1::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 1 in Video B.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & BRoiRegion1::X_MASK) >> BRoiRegion1::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 1 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BRoiRegion1::EN_MASK
            } else {
                self.bits & !BRoiRegion1::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 1 in Video B.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion1::Y_LEN_MASK)
                | ((value << BRoiRegion1::Y_LEN_OFFSET) & BRoiRegion1::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 1 in Video B.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion1::X_LEN_MASK)
                | ((value << BRoiRegion1::X_LEN_OFFSET) & BRoiRegion1::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 1 in Video B.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion1::Y_MASK)
                | ((value << BRoiRegion1::Y_OFFSET) & BRoiRegion1::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 1 in Video B.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion1::X_MASK)
                | ((value << BRoiRegion1::X_OFFSET) & BRoiRegion1::X_MASK),
        }
    }
}

impl Default for BRoiRegion1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BRoiRegion1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BRoiRegion1Val {
        BRoiRegion1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BRoiRegion1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BRoiRegion1Val) -> BRoiRegion1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_ROI_REGION2 Register
///
/// Video B H264 ROI region2 range configure register.
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video B ROI of region 2 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 2 in Video B.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 2 in Video B.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 2 in Video B.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontal start macroblocks of region 2 in Video B.
#[repr(transparent)]
pub struct BRoiRegion2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BRoiRegion2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BRoiRegion2 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video B ROI of region 2 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 2 in Video B.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 2 in Video B.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 2 in Video B.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontal start macroblocks of region 2 in Video B.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video B ROI of region 2 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 2 in Video B.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 2 in Video B.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 2 in Video B.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 2 in Video B.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 2 in Video B.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 2 in Video B.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 2 in Video B.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 2 in Video B.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// B_ROI_REGION2 value type for batch read/modify/write operations
///
/// Use `broiregion2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `broiregion2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BRoiRegion2Val {
    bits: u32,
}

impl BRoiRegion2Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 2 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & BRoiRegion2::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 2 in Video B.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & BRoiRegion2::Y_LEN_MASK) >> BRoiRegion2::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 2 in Video B.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & BRoiRegion2::X_LEN_MASK) >> BRoiRegion2::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 2 in Video B.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & BRoiRegion2::Y_MASK) >> BRoiRegion2::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 2 in Video B.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & BRoiRegion2::X_MASK) >> BRoiRegion2::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 2 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BRoiRegion2::EN_MASK
            } else {
                self.bits & !BRoiRegion2::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 2 in Video B.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion2::Y_LEN_MASK)
                | ((value << BRoiRegion2::Y_LEN_OFFSET) & BRoiRegion2::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 2 in Video B.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion2::X_LEN_MASK)
                | ((value << BRoiRegion2::X_LEN_OFFSET) & BRoiRegion2::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 2 in Video B.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion2::Y_MASK)
                | ((value << BRoiRegion2::Y_OFFSET) & BRoiRegion2::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 2 in Video B.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion2::X_MASK)
                | ((value << BRoiRegion2::X_OFFSET) & BRoiRegion2::X_MASK),
        }
    }
}

impl Default for BRoiRegion2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BRoiRegion2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BRoiRegion2Val {
        BRoiRegion2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BRoiRegion2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BRoiRegion2Val) -> BRoiRegion2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_ROI_REGION3 Register
///
/// Video B H264 ROI region3 range configure register.
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video B ROI of region 3 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 3 in video B.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 3 in video B.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 3 in Video B.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontal start macroblocks of region 3 in Video B.
#[repr(transparent)]
pub struct BRoiRegion3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BRoiRegion3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BRoiRegion3 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video B ROI of region 3 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 3 in video B.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 3 in video B.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 3 in Video B.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontal start macroblocks of region 3 in Video B.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video B ROI of region 3 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 3 in video B.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 3 in video B.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 3 in video B.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 3 in video B.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 3 in Video B.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 3 in Video B.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 3 in Video B.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 3 in Video B.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// B_ROI_REGION3 value type for batch read/modify/write operations
///
/// Use `broiregion3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `broiregion3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BRoiRegion3Val {
    bits: u32,
}

impl BRoiRegion3Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 3 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & BRoiRegion3::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 3 in video B.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & BRoiRegion3::Y_LEN_MASK) >> BRoiRegion3::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 3 in video B.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & BRoiRegion3::X_LEN_MASK) >> BRoiRegion3::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 3 in Video B.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & BRoiRegion3::Y_MASK) >> BRoiRegion3::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 3 in Video B.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & BRoiRegion3::X_MASK) >> BRoiRegion3::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 3 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BRoiRegion3::EN_MASK
            } else {
                self.bits & !BRoiRegion3::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 3 in video B.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion3::Y_LEN_MASK)
                | ((value << BRoiRegion3::Y_LEN_OFFSET) & BRoiRegion3::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 3 in video B.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion3::X_LEN_MASK)
                | ((value << BRoiRegion3::X_LEN_OFFSET) & BRoiRegion3::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 3 in Video B.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion3::Y_MASK)
                | ((value << BRoiRegion3::Y_OFFSET) & BRoiRegion3::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 3 in Video B.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion3::X_MASK)
                | ((value << BRoiRegion3::X_OFFSET) & BRoiRegion3::X_MASK),
        }
    }
}

impl Default for BRoiRegion3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BRoiRegion3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BRoiRegion3Val {
        BRoiRegion3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BRoiRegion3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BRoiRegion3Val) -> BRoiRegion3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_ROI_REGION4 Register
///
/// Video B H264 ROI region4 range configure register.
///
/// Address offset: 0x0080
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video B ROI of region 4 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 4 in video B.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 4 in video B.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 4 in Video B.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontal start macroblocks of region 4 in Video B.
#[repr(transparent)]
pub struct BRoiRegion4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BRoiRegion4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BRoiRegion4 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video B ROI of region 4 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 4 in video B.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 4 in video B.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 4 in Video B.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontal start macroblocks of region 4 in Video B.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video B ROI of region 4 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 4 in video B.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 4 in video B.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 4 in video B.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 4 in video B.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 4 in Video B.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 4 in Video B.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 4 in Video B.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 4 in Video B.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// B_ROI_REGION4 value type for batch read/modify/write operations
///
/// Use `broiregion4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `broiregion4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BRoiRegion4Val {
    bits: u32,
}

impl BRoiRegion4Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 4 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & BRoiRegion4::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 4 in video B.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & BRoiRegion4::Y_LEN_MASK) >> BRoiRegion4::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 4 in video B.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & BRoiRegion4::X_LEN_MASK) >> BRoiRegion4::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 4 in Video B.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & BRoiRegion4::Y_MASK) >> BRoiRegion4::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 4 in Video B.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & BRoiRegion4::X_MASK) >> BRoiRegion4::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 4 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BRoiRegion4::EN_MASK
            } else {
                self.bits & !BRoiRegion4::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 4 in video B.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion4::Y_LEN_MASK)
                | ((value << BRoiRegion4::Y_LEN_OFFSET) & BRoiRegion4::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 4 in video B.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion4::X_LEN_MASK)
                | ((value << BRoiRegion4::X_LEN_OFFSET) & BRoiRegion4::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 4 in Video B.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion4::Y_MASK)
                | ((value << BRoiRegion4::Y_OFFSET) & BRoiRegion4::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 4 in Video B.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion4::X_MASK)
                | ((value << BRoiRegion4::X_OFFSET) & BRoiRegion4::X_MASK),
        }
    }
}

impl Default for BRoiRegion4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BRoiRegion4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BRoiRegion4Val {
        BRoiRegion4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BRoiRegion4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BRoiRegion4Val) -> BRoiRegion4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_ROI_REGION5 Register
///
/// Video B H264 ROI region5 range configure register.
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video B ROI of region 5 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 5 in video B.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 5 video B.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 5 video B.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontial start macroblocks of region 5 video B.
#[repr(transparent)]
pub struct BRoiRegion5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BRoiRegion5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BRoiRegion5 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video B ROI of region 5 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 5 in video B.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 5 video B.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 5 video B.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontial start macroblocks of region 5 video B.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video B ROI of region 5 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 5 in video B.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 5 in video B.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 5 video B.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 5 video B.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 5 video B.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 5 video B.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontial start macroblocks of region 5 video B.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontial start macroblocks of region 5 video B.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// B_ROI_REGION5 value type for batch read/modify/write operations
///
/// Use `broiregion5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `broiregion5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BRoiRegion5Val {
    bits: u32,
}

impl BRoiRegion5Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 5 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & BRoiRegion5::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 5 in video B.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & BRoiRegion5::Y_LEN_MASK) >> BRoiRegion5::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 5 video B.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & BRoiRegion5::X_LEN_MASK) >> BRoiRegion5::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 5 video B.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & BRoiRegion5::Y_MASK) >> BRoiRegion5::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontial start macroblocks of region 5 video B.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & BRoiRegion5::X_MASK) >> BRoiRegion5::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 5 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BRoiRegion5::EN_MASK
            } else {
                self.bits & !BRoiRegion5::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 5 in video B.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion5::Y_LEN_MASK)
                | ((value << BRoiRegion5::Y_LEN_OFFSET) & BRoiRegion5::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 5 video B.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion5::X_LEN_MASK)
                | ((value << BRoiRegion5::X_LEN_OFFSET) & BRoiRegion5::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 5 video B.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion5::Y_MASK)
                | ((value << BRoiRegion5::Y_OFFSET) & BRoiRegion5::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontial start macroblocks of region 5 video B.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion5::X_MASK)
                | ((value << BRoiRegion5::X_OFFSET) & BRoiRegion5::X_MASK),
        }
    }
}

impl Default for BRoiRegion5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BRoiRegion5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BRoiRegion5Val {
        BRoiRegion5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BRoiRegion5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BRoiRegion5Val) -> BRoiRegion5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_ROI_REGION6 Register
///
/// Video B H264 ROI region6 range configure register.
///
/// Address offset: 0x0088
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video B ROI of region 6 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 6 in video B.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 6 in video B.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 6 in video B.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontial start macroblocks of region 6 video B.
#[repr(transparent)]
pub struct BRoiRegion6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BRoiRegion6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BRoiRegion6 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video B ROI of region 6 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 6 in video B.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 6 in video B.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 6 in video B.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontial start macroblocks of region 6 video B.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video B ROI of region 6 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 6 in video B.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 6 in video B.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 6 in video B.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 6 in video B.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 6 in video B.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 6 in video B.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontial start macroblocks of region 6 video B.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontial start macroblocks of region 6 video B.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// B_ROI_REGION6 value type for batch read/modify/write operations
///
/// Use `broiregion6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `broiregion6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BRoiRegion6Val {
    bits: u32,
}

impl BRoiRegion6Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 6 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & BRoiRegion6::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 6 in video B.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & BRoiRegion6::Y_LEN_MASK) >> BRoiRegion6::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 6 in video B.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & BRoiRegion6::X_LEN_MASK) >> BRoiRegion6::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 6 in video B.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & BRoiRegion6::Y_MASK) >> BRoiRegion6::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontial start macroblocks of region 6 video B.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & BRoiRegion6::X_MASK) >> BRoiRegion6::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 6 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BRoiRegion6::EN_MASK
            } else {
                self.bits & !BRoiRegion6::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 6 in video B.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion6::Y_LEN_MASK)
                | ((value << BRoiRegion6::Y_LEN_OFFSET) & BRoiRegion6::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 6 in video B.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion6::X_LEN_MASK)
                | ((value << BRoiRegion6::X_LEN_OFFSET) & BRoiRegion6::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 6 in video B.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion6::Y_MASK)
                | ((value << BRoiRegion6::Y_OFFSET) & BRoiRegion6::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontial start macroblocks of region 6 video B.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion6::X_MASK)
                | ((value << BRoiRegion6::X_OFFSET) & BRoiRegion6::X_MASK),
        }
    }
}

impl Default for BRoiRegion6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BRoiRegion6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BRoiRegion6Val {
        BRoiRegion6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BRoiRegion6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BRoiRegion6Val) -> BRoiRegion6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_ROI_REGION7 Register
///
/// Video B H264 ROI region7 range configure register.
///
/// Address offset: 0x008c
///
/// ## Fields
///
/// - **EN** `[28]` (RW)
///
///   Configures whether or not to open Video B ROI of region 7 .\\0:Close ROI\\1:Open ROI.
///
/// - **Y_LEN** `[27:21]` (RW)
///
///   Configures the number of macroblocks in vertical direction of the region 7 in video B.
///
/// - **X_LEN** `[20:14]` (RW)
///
///   Configures the number of macroblocks in horizontal direction of the region 7 in video B.
///
/// - **Y** `[13:7]` (RW)
///
///   Configures the vertical start macroblocks of region 7 in video B.
///
/// - **X** `[6:0]` (RW)
///
///   Configures the horizontal start macroblocks of region 7 in video B.
#[repr(transparent)]
pub struct BRoiRegion7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BRoiRegion7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BRoiRegion7 {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Configures whether or not to open Video B ROI of region 7 .\\0:Close ROI\\1:Open ROI.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EN_OFFSET: u32 = 28;
    pub const EN_MASK: u32 = 0x10000000;
    pub const EN: u32 = 1 << 28;

    /// **Y_LEN**
    ///
    /// Configures the number of macroblocks in vertical direction of the region 7 in video B.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const Y_LEN_OFFSET: u32 = 21;
    pub const Y_LEN_MASK: u32 = 0x0fe00000;

    /// **X_LEN**
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 7 in video B.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const X_LEN_OFFSET: u32 = 14;
    pub const X_LEN_MASK: u32 = 0x001fc000;

    /// **Y**
    ///
    /// Configures the vertical start macroblocks of region 7 in video B.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const Y_OFFSET: u32 = 7;
    pub const Y_MASK: u32 = 0x00003f80;

    /// **X**
    ///
    /// Configures the horizontal start macroblocks of region 7 in video B.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const X_OFFSET: u32 = 0;
    pub const X_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **EN** `[28]` (RW)
    ///
    /// Configures whether or not to open Video B ROI of region 7 .\\0:Close ROI\\1:Open ROI.
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

    /// **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 7 in video B.
    #[inline(always)]
    pub fn y_len(&self) -> u32 {
        (self.value.read_bits() & Self::Y_LEN_MASK) >> Self::Y_LEN_OFFSET
    }

    /// Write **Y_LEN** `[27:21]` (RW)
    ///
    /// Configures the number of macroblocks in vertical direction of the region 7 in video B.
    #[inline(always)]
    pub fn write_y_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_LEN_MASK) | ((value << Self::Y_LEN_OFFSET) & Self::Y_LEN_MASK)
        });
    }

    /// **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 7 in video B.
    #[inline(always)]
    pub fn x_len(&self) -> u32 {
        (self.value.read_bits() & Self::X_LEN_MASK) >> Self::X_LEN_OFFSET
    }

    /// Write **X_LEN** `[20:14]` (RW)
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 7 in video B.
    #[inline(always)]
    pub fn write_x_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_LEN_MASK) | ((value << Self::X_LEN_OFFSET) & Self::X_LEN_MASK)
        });
    }

    /// **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 7 in video B.
    #[inline(always)]
    pub fn y(&self) -> u32 {
        (self.value.read_bits() & Self::Y_MASK) >> Self::Y_OFFSET
    }

    /// Write **Y** `[13:7]` (RW)
    ///
    /// Configures the vertical start macroblocks of region 7 in video B.
    #[inline(always)]
    pub fn write_y(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::Y_MASK) | ((value << Self::Y_OFFSET) & Self::Y_MASK));
    }

    /// **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 7 in video B.
    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.value.read_bits() & Self::X_MASK) >> Self::X_OFFSET
    }

    /// Write **X** `[6:0]` (RW)
    ///
    /// Configures the horizontal start macroblocks of region 7 in video B.
    #[inline(always)]
    pub fn write_x(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::X_MASK) | ((value << Self::X_OFFSET) & Self::X_MASK));
    }
}

/// B_ROI_REGION7 value type for batch read/modify/write operations
///
/// Use `broiregion7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `broiregion7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BRoiRegion7Val {
    bits: u32,
}

impl BRoiRegion7Val {
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

    /// Read **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 7 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & BRoiRegion7::EN_MASK) != 0
    }

    /// Read **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 7 in video B.
    #[inline(always)]
    pub const fn y_len(&self) -> u32 {
        (self.bits & BRoiRegion7::Y_LEN_MASK) >> BRoiRegion7::Y_LEN_OFFSET
    }

    /// Read **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 7 in video B.
    #[inline(always)]
    pub const fn x_len(&self) -> u32 {
        (self.bits & BRoiRegion7::X_LEN_MASK) >> BRoiRegion7::X_LEN_OFFSET
    }

    /// Read **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 7 in video B.
    #[inline(always)]
    pub const fn y(&self) -> u32 {
        (self.bits & BRoiRegion7::Y_MASK) >> BRoiRegion7::Y_OFFSET
    }

    /// Read **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 7 in video B.
    #[inline(always)]
    pub const fn x(&self) -> u32 {
        (self.bits & BRoiRegion7::X_MASK) >> BRoiRegion7::X_OFFSET
    }

    /// Set **EN** `[28:28]`
    ///
    /// Configures whether or not to open Video B ROI of region 7 .\\0:Close ROI\\1:Open ROI.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BRoiRegion7::EN_MASK
            } else {
                self.bits & !BRoiRegion7::EN_MASK
            },
        }
    }

    /// Set **Y_LEN** `[27:21]`
    ///
    /// Configures the number of macroblocks in vertical direction of the region 7 in video B.
    #[inline(always)]
    pub const fn with_y_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion7::Y_LEN_MASK)
                | ((value << BRoiRegion7::Y_LEN_OFFSET) & BRoiRegion7::Y_LEN_MASK),
        }
    }

    /// Set **X_LEN** `[20:14]`
    ///
    /// Configures the number of macroblocks in horizontal direction of the region 7 in video B.
    #[inline(always)]
    pub const fn with_x_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion7::X_LEN_MASK)
                | ((value << BRoiRegion7::X_LEN_OFFSET) & BRoiRegion7::X_LEN_MASK),
        }
    }

    /// Set **Y** `[13:7]`
    ///
    /// Configures the vertical start macroblocks of region 7 in video B.
    #[inline(always)]
    pub const fn with_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion7::Y_MASK)
                | ((value << BRoiRegion7::Y_OFFSET) & BRoiRegion7::Y_MASK),
        }
    }

    /// Set **X** `[6:0]`
    ///
    /// Configures the horizontal start macroblocks of region 7 in video B.
    #[inline(always)]
    pub const fn with_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion7::X_MASK)
                | ((value << BRoiRegion7::X_OFFSET) & BRoiRegion7::X_MASK),
        }
    }
}

impl Default for BRoiRegion7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BRoiRegion7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BRoiRegion7Val {
        BRoiRegion7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BRoiRegion7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BRoiRegion7Val) -> BRoiRegion7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_ROI_REGION0_3_QP Register
///
/// Video B H264 ROI region0, region1,region2,region3 QP register.
///
/// Address offset: 0x0090
///
/// ## Fields
///
/// - **B_ROI_REGION3_QP** `[27:21]` (RW)
///
///   Configure H264 ROI region3 qp in video B,fixed qp or delta qp.
///
/// - **B_ROI_REGION2_QP** `[20:14]` (RW)
///
///   Configure H264 ROI region2 qp in video B,fixed qp or delta qp.
///
/// - **B_ROI_REGION1_QP** `[13:7]` (RW)
///
///   Configure H264 ROI region1 qp in video B,fixed qp or delta qp.
///
/// - **B_ROI_REGION0_QP** `[6:0]` (RW)
///
///   Configure H264 ROI region0 qp in video B,fixed qp or delta qp.
#[repr(transparent)]
pub struct BRoiRegion03Qp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BRoiRegion03Qp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BRoiRegion03Qp {
    // ======== Field Constants ========

    /// **B_ROI_REGION3_QP**
    ///
    /// Configure H264 ROI region3 qp in video B,fixed qp or delta qp.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const B_ROI_REGION3_QP_OFFSET: u32 = 21;
    pub const B_ROI_REGION3_QP_MASK: u32 = 0x0fe00000;

    /// **B_ROI_REGION2_QP**
    ///
    /// Configure H264 ROI region2 qp in video B,fixed qp or delta qp.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const B_ROI_REGION2_QP_OFFSET: u32 = 14;
    pub const B_ROI_REGION2_QP_MASK: u32 = 0x001fc000;

    /// **B_ROI_REGION1_QP**
    ///
    /// Configure H264 ROI region1 qp in video B,fixed qp or delta qp.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const B_ROI_REGION1_QP_OFFSET: u32 = 7;
    pub const B_ROI_REGION1_QP_MASK: u32 = 0x00003f80;

    /// **B_ROI_REGION0_QP**
    ///
    /// Configure H264 ROI region0 qp in video B,fixed qp or delta qp.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const B_ROI_REGION0_QP_OFFSET: u32 = 0;
    pub const B_ROI_REGION0_QP_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **B_ROI_REGION3_QP** `[27:21]` (RW)
    ///
    /// Configure H264 ROI region3 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn b_roi_region3_qp(&self) -> u32 {
        (self.value.read_bits() & Self::B_ROI_REGION3_QP_MASK) >> Self::B_ROI_REGION3_QP_OFFSET
    }

    /// Write **B_ROI_REGION3_QP** `[27:21]` (RW)
    ///
    /// Configure H264 ROI region3 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_b_roi_region3_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_ROI_REGION3_QP_MASK)
                | ((value << Self::B_ROI_REGION3_QP_OFFSET) & Self::B_ROI_REGION3_QP_MASK)
        });
    }

    /// **B_ROI_REGION2_QP** `[20:14]` (RW)
    ///
    /// Configure H264 ROI region2 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn b_roi_region2_qp(&self) -> u32 {
        (self.value.read_bits() & Self::B_ROI_REGION2_QP_MASK) >> Self::B_ROI_REGION2_QP_OFFSET
    }

    /// Write **B_ROI_REGION2_QP** `[20:14]` (RW)
    ///
    /// Configure H264 ROI region2 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_b_roi_region2_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_ROI_REGION2_QP_MASK)
                | ((value << Self::B_ROI_REGION2_QP_OFFSET) & Self::B_ROI_REGION2_QP_MASK)
        });
    }

    /// **B_ROI_REGION1_QP** `[13:7]` (RW)
    ///
    /// Configure H264 ROI region1 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn b_roi_region1_qp(&self) -> u32 {
        (self.value.read_bits() & Self::B_ROI_REGION1_QP_MASK) >> Self::B_ROI_REGION1_QP_OFFSET
    }

    /// Write **B_ROI_REGION1_QP** `[13:7]` (RW)
    ///
    /// Configure H264 ROI region1 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_b_roi_region1_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_ROI_REGION1_QP_MASK)
                | ((value << Self::B_ROI_REGION1_QP_OFFSET) & Self::B_ROI_REGION1_QP_MASK)
        });
    }

    /// **B_ROI_REGION0_QP** `[6:0]` (RW)
    ///
    /// Configure H264 ROI region0 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn b_roi_region0_qp(&self) -> u32 {
        (self.value.read_bits() & Self::B_ROI_REGION0_QP_MASK) >> Self::B_ROI_REGION0_QP_OFFSET
    }

    /// Write **B_ROI_REGION0_QP** `[6:0]` (RW)
    ///
    /// Configure H264 ROI region0 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_b_roi_region0_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_ROI_REGION0_QP_MASK)
                | ((value << Self::B_ROI_REGION0_QP_OFFSET) & Self::B_ROI_REGION0_QP_MASK)
        });
    }
}

/// B_ROI_REGION0_3_QP value type for batch read/modify/write operations
///
/// Use `broiregion03qp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `broiregion03qp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BRoiRegion03QpVal {
    bits: u32,
}

impl BRoiRegion03QpVal {
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

    /// Read **B_ROI_REGION3_QP** `[27:21]`
    ///
    /// Configure H264 ROI region3 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn b_roi_region3_qp(&self) -> u32 {
        (self.bits & BRoiRegion03Qp::B_ROI_REGION3_QP_MASK)
            >> BRoiRegion03Qp::B_ROI_REGION3_QP_OFFSET
    }

    /// Read **B_ROI_REGION2_QP** `[20:14]`
    ///
    /// Configure H264 ROI region2 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn b_roi_region2_qp(&self) -> u32 {
        (self.bits & BRoiRegion03Qp::B_ROI_REGION2_QP_MASK)
            >> BRoiRegion03Qp::B_ROI_REGION2_QP_OFFSET
    }

    /// Read **B_ROI_REGION1_QP** `[13:7]`
    ///
    /// Configure H264 ROI region1 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn b_roi_region1_qp(&self) -> u32 {
        (self.bits & BRoiRegion03Qp::B_ROI_REGION1_QP_MASK)
            >> BRoiRegion03Qp::B_ROI_REGION1_QP_OFFSET
    }

    /// Read **B_ROI_REGION0_QP** `[6:0]`
    ///
    /// Configure H264 ROI region0 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn b_roi_region0_qp(&self) -> u32 {
        (self.bits & BRoiRegion03Qp::B_ROI_REGION0_QP_MASK)
            >> BRoiRegion03Qp::B_ROI_REGION0_QP_OFFSET
    }

    /// Set **B_ROI_REGION3_QP** `[27:21]`
    ///
    /// Configure H264 ROI region3 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_b_roi_region3_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion03Qp::B_ROI_REGION3_QP_MASK)
                | ((value << BRoiRegion03Qp::B_ROI_REGION3_QP_OFFSET)
                    & BRoiRegion03Qp::B_ROI_REGION3_QP_MASK),
        }
    }

    /// Set **B_ROI_REGION2_QP** `[20:14]`
    ///
    /// Configure H264 ROI region2 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_b_roi_region2_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion03Qp::B_ROI_REGION2_QP_MASK)
                | ((value << BRoiRegion03Qp::B_ROI_REGION2_QP_OFFSET)
                    & BRoiRegion03Qp::B_ROI_REGION2_QP_MASK),
        }
    }

    /// Set **B_ROI_REGION1_QP** `[13:7]`
    ///
    /// Configure H264 ROI region1 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_b_roi_region1_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion03Qp::B_ROI_REGION1_QP_MASK)
                | ((value << BRoiRegion03Qp::B_ROI_REGION1_QP_OFFSET)
                    & BRoiRegion03Qp::B_ROI_REGION1_QP_MASK),
        }
    }

    /// Set **B_ROI_REGION0_QP** `[6:0]`
    ///
    /// Configure H264 ROI region0 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_b_roi_region0_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion03Qp::B_ROI_REGION0_QP_MASK)
                | ((value << BRoiRegion03Qp::B_ROI_REGION0_QP_OFFSET)
                    & BRoiRegion03Qp::B_ROI_REGION0_QP_MASK),
        }
    }
}

impl Default for BRoiRegion03QpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BRoiRegion03Qp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BRoiRegion03QpVal {
        BRoiRegion03QpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BRoiRegion03QpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BRoiRegion03QpVal) -> BRoiRegion03QpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_ROI_REGION4_7_QP Register
///
/// Video B H264 ROI region4, region5,region6,region7 QP register.
///
/// Address offset: 0x0094
///
/// ## Fields
///
/// - **B_ROI_REGION7_QP** `[27:21]` (RW)
///
///   Configure H264 ROI region7 qp in video B,fixed qp or delta qp.
///
/// - **B_ROI_REGION6_QP** `[20:14]` (RW)
///
///   Configure H264 ROI region6 qp in video B,fixed qp or delta qp.
///
/// - **B_ROI_REGION5_QP** `[13:7]` (RW)
///
///   Configure H264 ROI region5 qp in video B,fixed qp or delta qp.
///
/// - **B_ROI_REGION4_QP** `[6:0]` (RW)
///
///   Configure H264 ROI region4 qp in video B,fixed qp or delta qp.
#[repr(transparent)]
pub struct BRoiRegion47Qp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BRoiRegion47Qp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BRoiRegion47Qp {
    // ======== Field Constants ========

    /// **B_ROI_REGION7_QP**
    ///
    /// Configure H264 ROI region7 qp in video B,fixed qp or delta qp.
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const B_ROI_REGION7_QP_OFFSET: u32 = 21;
    pub const B_ROI_REGION7_QP_MASK: u32 = 0x0fe00000;

    /// **B_ROI_REGION6_QP**
    ///
    /// Configure H264 ROI region6 qp in video B,fixed qp or delta qp.
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const B_ROI_REGION6_QP_OFFSET: u32 = 14;
    pub const B_ROI_REGION6_QP_MASK: u32 = 0x001fc000;

    /// **B_ROI_REGION5_QP**
    ///
    /// Configure H264 ROI region5 qp in video B,fixed qp or delta qp.
    ///
    /// Bits `[13:7]`, mask `0x00003f80`
    pub const B_ROI_REGION5_QP_OFFSET: u32 = 7;
    pub const B_ROI_REGION5_QP_MASK: u32 = 0x00003f80;

    /// **B_ROI_REGION4_QP**
    ///
    /// Configure H264 ROI region4 qp in video B,fixed qp or delta qp.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const B_ROI_REGION4_QP_OFFSET: u32 = 0;
    pub const B_ROI_REGION4_QP_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **B_ROI_REGION7_QP** `[27:21]` (RW)
    ///
    /// Configure H264 ROI region7 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn b_roi_region7_qp(&self) -> u32 {
        (self.value.read_bits() & Self::B_ROI_REGION7_QP_MASK) >> Self::B_ROI_REGION7_QP_OFFSET
    }

    /// Write **B_ROI_REGION7_QP** `[27:21]` (RW)
    ///
    /// Configure H264 ROI region7 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_b_roi_region7_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_ROI_REGION7_QP_MASK)
                | ((value << Self::B_ROI_REGION7_QP_OFFSET) & Self::B_ROI_REGION7_QP_MASK)
        });
    }

    /// **B_ROI_REGION6_QP** `[20:14]` (RW)
    ///
    /// Configure H264 ROI region6 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn b_roi_region6_qp(&self) -> u32 {
        (self.value.read_bits() & Self::B_ROI_REGION6_QP_MASK) >> Self::B_ROI_REGION6_QP_OFFSET
    }

    /// Write **B_ROI_REGION6_QP** `[20:14]` (RW)
    ///
    /// Configure H264 ROI region6 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_b_roi_region6_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_ROI_REGION6_QP_MASK)
                | ((value << Self::B_ROI_REGION6_QP_OFFSET) & Self::B_ROI_REGION6_QP_MASK)
        });
    }

    /// **B_ROI_REGION5_QP** `[13:7]` (RW)
    ///
    /// Configure H264 ROI region5 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn b_roi_region5_qp(&self) -> u32 {
        (self.value.read_bits() & Self::B_ROI_REGION5_QP_MASK) >> Self::B_ROI_REGION5_QP_OFFSET
    }

    /// Write **B_ROI_REGION5_QP** `[13:7]` (RW)
    ///
    /// Configure H264 ROI region5 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_b_roi_region5_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_ROI_REGION5_QP_MASK)
                | ((value << Self::B_ROI_REGION5_QP_OFFSET) & Self::B_ROI_REGION5_QP_MASK)
        });
    }

    /// **B_ROI_REGION4_QP** `[6:0]` (RW)
    ///
    /// Configure H264 ROI region4 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn b_roi_region4_qp(&self) -> u32 {
        (self.value.read_bits() & Self::B_ROI_REGION4_QP_MASK) >> Self::B_ROI_REGION4_QP_OFFSET
    }

    /// Write **B_ROI_REGION4_QP** `[6:0]` (RW)
    ///
    /// Configure H264 ROI region4 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub fn write_b_roi_region4_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_ROI_REGION4_QP_MASK)
                | ((value << Self::B_ROI_REGION4_QP_OFFSET) & Self::B_ROI_REGION4_QP_MASK)
        });
    }
}

/// B_ROI_REGION4_7_QP value type for batch read/modify/write operations
///
/// Use `broiregion47qp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `broiregion47qp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BRoiRegion47QpVal {
    bits: u32,
}

impl BRoiRegion47QpVal {
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

    /// Read **B_ROI_REGION7_QP** `[27:21]`
    ///
    /// Configure H264 ROI region7 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn b_roi_region7_qp(&self) -> u32 {
        (self.bits & BRoiRegion47Qp::B_ROI_REGION7_QP_MASK)
            >> BRoiRegion47Qp::B_ROI_REGION7_QP_OFFSET
    }

    /// Read **B_ROI_REGION6_QP** `[20:14]`
    ///
    /// Configure H264 ROI region6 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn b_roi_region6_qp(&self) -> u32 {
        (self.bits & BRoiRegion47Qp::B_ROI_REGION6_QP_MASK)
            >> BRoiRegion47Qp::B_ROI_REGION6_QP_OFFSET
    }

    /// Read **B_ROI_REGION5_QP** `[13:7]`
    ///
    /// Configure H264 ROI region5 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn b_roi_region5_qp(&self) -> u32 {
        (self.bits & BRoiRegion47Qp::B_ROI_REGION5_QP_MASK)
            >> BRoiRegion47Qp::B_ROI_REGION5_QP_OFFSET
    }

    /// Read **B_ROI_REGION4_QP** `[6:0]`
    ///
    /// Configure H264 ROI region4 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn b_roi_region4_qp(&self) -> u32 {
        (self.bits & BRoiRegion47Qp::B_ROI_REGION4_QP_MASK)
            >> BRoiRegion47Qp::B_ROI_REGION4_QP_OFFSET
    }

    /// Set **B_ROI_REGION7_QP** `[27:21]`
    ///
    /// Configure H264 ROI region7 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_b_roi_region7_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion47Qp::B_ROI_REGION7_QP_MASK)
                | ((value << BRoiRegion47Qp::B_ROI_REGION7_QP_OFFSET)
                    & BRoiRegion47Qp::B_ROI_REGION7_QP_MASK),
        }
    }

    /// Set **B_ROI_REGION6_QP** `[20:14]`
    ///
    /// Configure H264 ROI region6 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_b_roi_region6_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion47Qp::B_ROI_REGION6_QP_MASK)
                | ((value << BRoiRegion47Qp::B_ROI_REGION6_QP_OFFSET)
                    & BRoiRegion47Qp::B_ROI_REGION6_QP_MASK),
        }
    }

    /// Set **B_ROI_REGION5_QP** `[13:7]`
    ///
    /// Configure H264 ROI region5 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_b_roi_region5_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion47Qp::B_ROI_REGION5_QP_MASK)
                | ((value << BRoiRegion47Qp::B_ROI_REGION5_QP_OFFSET)
                    & BRoiRegion47Qp::B_ROI_REGION5_QP_MASK),
        }
    }

    /// Set **B_ROI_REGION4_QP** `[6:0]`
    ///
    /// Configure H264 ROI region4 qp in video B,fixed qp or delta qp.
    #[inline(always)]
    pub const fn with_b_roi_region4_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BRoiRegion47Qp::B_ROI_REGION4_QP_MASK)
                | ((value << BRoiRegion47Qp::B_ROI_REGION4_QP_OFFSET)
                    & BRoiRegion47Qp::B_ROI_REGION4_QP_MASK),
        }
    }
}

impl Default for BRoiRegion47QpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BRoiRegion47Qp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BRoiRegion47QpVal {
        BRoiRegion47QpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BRoiRegion47QpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BRoiRegion47QpVal) -> BRoiRegion47QpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_NO_ROI_REGION_QP_OFFSET Register
///
/// Video B H264 no roi region QP register.
///
/// Address offset: 0x0098
///
/// ## Fields
///
/// - **B_NO_ROI_REGION_QP** `[6:0]` (RW)
///
///   Configure H264 no region qp in video B, delta qp.
#[repr(transparent)]
pub struct BNoRoiRegionQpOffset {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BNoRoiRegionQpOffset {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BNoRoiRegionQpOffset {
    // ======== Field Constants ========

    /// **B_NO_ROI_REGION_QP**
    ///
    /// Configure H264 no region qp in video B, delta qp.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const B_NO_ROI_REGION_QP_OFFSET: u32 = 0;
    pub const B_NO_ROI_REGION_QP_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **B_NO_ROI_REGION_QP** `[6:0]` (RW)
    ///
    /// Configure H264 no region qp in video B, delta qp.
    #[inline(always)]
    pub fn b_no_roi_region_qp(&self) -> u32 {
        (self.value.read_bits() & Self::B_NO_ROI_REGION_QP_MASK) >> Self::B_NO_ROI_REGION_QP_OFFSET
    }

    /// Write **B_NO_ROI_REGION_QP** `[6:0]` (RW)
    ///
    /// Configure H264 no region qp in video B, delta qp.
    #[inline(always)]
    pub fn write_b_no_roi_region_qp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::B_NO_ROI_REGION_QP_MASK)
                | ((value << Self::B_NO_ROI_REGION_QP_OFFSET) & Self::B_NO_ROI_REGION_QP_MASK)
        });
    }
}

/// B_NO_ROI_REGION_QP_OFFSET value type for batch read/modify/write operations
///
/// Use `bnoroiregionqpoffset.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bnoroiregionqpoffset.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BNoRoiRegionQpOffsetVal {
    bits: u32,
}

impl BNoRoiRegionQpOffsetVal {
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

    /// Read **B_NO_ROI_REGION_QP** `[6:0]`
    ///
    /// Configure H264 no region qp in video B, delta qp.
    #[inline(always)]
    pub const fn b_no_roi_region_qp(&self) -> u32 {
        (self.bits & BNoRoiRegionQpOffset::B_NO_ROI_REGION_QP_MASK)
            >> BNoRoiRegionQpOffset::B_NO_ROI_REGION_QP_OFFSET
    }

    /// Set **B_NO_ROI_REGION_QP** `[6:0]`
    ///
    /// Configure H264 no region qp in video B, delta qp.
    #[inline(always)]
    pub const fn with_b_no_roi_region_qp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BNoRoiRegionQpOffset::B_NO_ROI_REGION_QP_MASK)
                | ((value << BNoRoiRegionQpOffset::B_NO_ROI_REGION_QP_OFFSET)
                    & BNoRoiRegionQpOffset::B_NO_ROI_REGION_QP_MASK),
        }
    }
}

impl Default for BNoRoiRegionQpOffsetVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BNoRoiRegionQpOffset {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BNoRoiRegionQpOffsetVal {
        BNoRoiRegionQpOffsetVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BNoRoiRegionQpOffsetVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BNoRoiRegionQpOffsetVal) -> BNoRoiRegionQpOffsetVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// B_ROI_CONFIG Register
///
/// Video B H264 ROI configure register.
///
/// Address offset: 0x009c
///
/// ## Fields
///
/// - **B_ROI_MODE** `[1]` (RW)
///
///   Configure the mode of ROI in video B.\\0:fixed qp\\1:delta qp.
///
/// - **B_ROI_EN** `[0]` (RW)
///
///   Configure whether or not to enable ROI in video B.\\0:not enable ROI\\1:enable ROI.
#[repr(transparent)]
pub struct BRoiConfig {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BRoiConfig {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BRoiConfig {
    // ======== Field Constants ========

    /// **B_ROI_MODE**
    ///
    /// Configure the mode of ROI in video B.\\0:fixed qp\\1:delta qp.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const B_ROI_MODE_OFFSET: u32 = 1;
    pub const B_ROI_MODE_MASK: u32 = 0x00000002;
    pub const B_ROI_MODE: u32 = 1 << 1;

    /// **B_ROI_EN**
    ///
    /// Configure whether or not to enable ROI in video B.\\0:not enable ROI\\1:enable ROI.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const B_ROI_EN_OFFSET: u32 = 0;
    pub const B_ROI_EN_MASK: u32 = 0x00000001;
    pub const B_ROI_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **B_ROI_MODE** `[1]` (RW)
    ///
    /// Configure the mode of ROI in video B.\\0:fixed qp\\1:delta qp.
    #[inline(always)]
    pub fn b_roi_mode(&self) -> bool {
        (self.value.read_bits() & Self::B_ROI_MODE_MASK) != 0
    }

    /// Set B_ROI_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_b_roi_mode(&self) {
        self.value.modify_bits(|r| r | Self::B_ROI_MODE_MASK);
    }

    /// Clear B_ROI_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_b_roi_mode(&self) {
        self.value.modify_bits(|r| r & !Self::B_ROI_MODE_MASK);
    }

    /// **B_ROI_EN** `[0]` (RW)
    ///
    /// Configure whether or not to enable ROI in video B.\\0:not enable ROI\\1:enable ROI.
    #[inline(always)]
    pub fn b_roi_en(&self) -> bool {
        (self.value.read_bits() & Self::B_ROI_EN_MASK) != 0
    }

    /// Set B_ROI_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_b_roi_en(&self) {
        self.value.modify_bits(|r| r | Self::B_ROI_EN_MASK);
    }

    /// Clear B_ROI_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_b_roi_en(&self) {
        self.value.modify_bits(|r| r & !Self::B_ROI_EN_MASK);
    }
}

/// B_ROI_CONFIG value type for batch read/modify/write operations
///
/// Use `broiconfig.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `broiconfig.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BRoiConfigVal {
    bits: u32,
}

impl BRoiConfigVal {
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

    /// Read **B_ROI_MODE** `[1:1]`
    ///
    /// Configure the mode of ROI in video B.\\0:fixed qp\\1:delta qp.
    #[inline(always)]
    pub const fn b_roi_mode(&self) -> bool {
        (self.bits & BRoiConfig::B_ROI_MODE_MASK) != 0
    }

    /// Read **B_ROI_EN** `[0:0]`
    ///
    /// Configure whether or not to enable ROI in video B.\\0:not enable ROI\\1:enable ROI.
    #[inline(always)]
    pub const fn b_roi_en(&self) -> bool {
        (self.bits & BRoiConfig::B_ROI_EN_MASK) != 0
    }

    /// Set **B_ROI_MODE** `[1:1]`
    ///
    /// Configure the mode of ROI in video B.\\0:fixed qp\\1:delta qp.
    #[inline(always)]
    pub const fn with_b_roi_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BRoiConfig::B_ROI_MODE_MASK
            } else {
                self.bits & !BRoiConfig::B_ROI_MODE_MASK
            },
        }
    }

    /// Set **B_ROI_EN** `[0:0]`
    ///
    /// Configure whether or not to enable ROI in video B.\\0:not enable ROI\\1:enable ROI.
    #[inline(always)]
    pub const fn with_b_roi_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BRoiConfig::B_ROI_EN_MASK
            } else {
                self.bits & !BRoiConfig::B_ROI_EN_MASK
            },
        }
    }
}

impl Default for BRoiConfigVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BRoiConfig {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BRoiConfigVal {
        BRoiConfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BRoiConfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BRoiConfigVal) -> BRoiConfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RC_STATUS0 Register
///
/// Rate control status register0.
///
/// Address offset: 0x00a0
///
/// ## Fields
///
/// - **FRAME_MAD_SUM** `[20:0]` (RO)
///
///   Represents all MB actual MAD sum value of one frame.
#[repr(transparent)]
pub struct RcStatus0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RcStatus0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RcStatus0 {
    // ======== Field Constants ========

    /// **FRAME_MAD_SUM**
    ///
    /// Represents all MB actual MAD sum value of one frame.
    ///
    /// Bits `[20:0]`, mask `0x001fffff`
    pub const FRAME_MAD_SUM_OFFSET: u32 = 0;
    pub const FRAME_MAD_SUM_MASK: u32 = 0x001fffff;

    // ======== Field Accessors ========

    /// **FRAME_MAD_SUM** `[20:0]` (RO)
    ///
    /// Represents all MB actual MAD sum value of one frame.
    #[inline(always)]
    pub fn frame_mad_sum(&self) -> u32 {
        (self.value.read_bits() & Self::FRAME_MAD_SUM_MASK) >> Self::FRAME_MAD_SUM_OFFSET
    }
}

/// RC_STATUS0 value type for batch read/modify/write operations
///
/// Use `rcstatus0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rcstatus0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RcStatus0Val {
    bits: u32,
}

impl RcStatus0Val {
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

    /// Read **FRAME_MAD_SUM** `[20:0]`
    ///
    /// Represents all MB actual MAD sum value of one frame.
    #[inline(always)]
    pub const fn frame_mad_sum(&self) -> u32 {
        (self.bits & RcStatus0::FRAME_MAD_SUM_MASK) >> RcStatus0::FRAME_MAD_SUM_OFFSET
    }
}

impl Default for RcStatus0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RcStatus0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RcStatus0Val {
        RcStatus0Val::from_bits(self.value.read_bits())
    }
}

/// RC_STATUS1 Register
///
/// Rate control status register1.
///
/// Address offset: 0x00a4
///
/// ## Fields
///
/// - **FRAME_ENC_BITS** `[26:0]` (RO)
///
///   Represents all MB actual encoding bits sum value of one frame.
#[repr(transparent)]
pub struct RcStatus1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RcStatus1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RcStatus1 {
    // ======== Field Constants ========

    /// **FRAME_ENC_BITS**
    ///
    /// Represents all MB actual encoding bits sum value of one frame.
    ///
    /// Bits `[26:0]`, mask `0x07ffffff`
    pub const FRAME_ENC_BITS_OFFSET: u32 = 0;
    pub const FRAME_ENC_BITS_MASK: u32 = 0x07ffffff;

    // ======== Field Accessors ========

    /// **FRAME_ENC_BITS** `[26:0]` (RO)
    ///
    /// Represents all MB actual encoding bits sum value of one frame.
    #[inline(always)]
    pub fn frame_enc_bits(&self) -> u32 {
        (self.value.read_bits() & Self::FRAME_ENC_BITS_MASK) >> Self::FRAME_ENC_BITS_OFFSET
    }
}

/// RC_STATUS1 value type for batch read/modify/write operations
///
/// Use `rcstatus1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rcstatus1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RcStatus1Val {
    bits: u32,
}

impl RcStatus1Val {
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

    /// Read **FRAME_ENC_BITS** `[26:0]`
    ///
    /// Represents all MB actual encoding bits sum value of one frame.
    #[inline(always)]
    pub const fn frame_enc_bits(&self) -> u32 {
        (self.bits & RcStatus1::FRAME_ENC_BITS_MASK) >> RcStatus1::FRAME_ENC_BITS_OFFSET
    }
}

impl Default for RcStatus1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RcStatus1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RcStatus1Val {
        RcStatus1Val::from_bits(self.value.read_bits())
    }
}

/// RC_STATUS2 Register
///
/// Rate control status register2.
///
/// Address offset: 0x00a8
///
/// ## Fields
///
/// - **FRAME_QP_SUM** `[18:0]` (RO)
///
///   Represents all MB actual luma QP sum value of one frame.
#[repr(transparent)]
pub struct RcStatus2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RcStatus2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RcStatus2 {
    // ======== Field Constants ========

    /// **FRAME_QP_SUM**
    ///
    /// Represents all MB actual luma QP sum value of one frame.
    ///
    /// Bits `[18:0]`, mask `0x0007ffff`
    pub const FRAME_QP_SUM_OFFSET: u32 = 0;
    pub const FRAME_QP_SUM_MASK: u32 = 0x0007ffff;

    // ======== Field Accessors ========

    /// **FRAME_QP_SUM** `[18:0]` (RO)
    ///
    /// Represents all MB actual luma QP sum value of one frame.
    #[inline(always)]
    pub fn frame_qp_sum(&self) -> u32 {
        (self.value.read_bits() & Self::FRAME_QP_SUM_MASK) >> Self::FRAME_QP_SUM_OFFSET
    }
}

/// RC_STATUS2 value type for batch read/modify/write operations
///
/// Use `rcstatus2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rcstatus2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RcStatus2Val {
    bits: u32,
}

impl RcStatus2Val {
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

    /// Read **FRAME_QP_SUM** `[18:0]`
    ///
    /// Represents all MB actual luma QP sum value of one frame.
    #[inline(always)]
    pub const fn frame_qp_sum(&self) -> u32 {
        (self.bits & RcStatus2::FRAME_QP_SUM_MASK) >> RcStatus2::FRAME_QP_SUM_OFFSET
    }
}

impl Default for RcStatus2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RcStatus2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RcStatus2Val {
        RcStatus2Val::from_bits(self.value.read_bits())
    }
}

/// SLICE_HEADER_REMAIN Register
///
/// Frame Slice Header remain bit register.
///
/// Address offset: 0x00ac
///
/// ## Fields
///
/// - **SLICE_REMAIN_BIT** `[10:3]` (RW)
///
///   Configures Slice Header remain bit
///
/// - **SLICE_REMAIN_BITLENGTH** `[2:0]` (RW)
///
///   Configures Slice Header remain bit number
#[repr(transparent)]
pub struct SliceHeaderRemain {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SliceHeaderRemain {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SliceHeaderRemain {
    // ======== Field Constants ========

    /// **SLICE_REMAIN_BIT**
    ///
    /// Configures Slice Header remain bit
    ///
    /// Bits `[10:3]`, mask `0x000007f8`
    pub const SLICE_REMAIN_BIT_OFFSET: u32 = 3;
    pub const SLICE_REMAIN_BIT_MASK: u32 = 0x000007f8;

    /// **SLICE_REMAIN_BITLENGTH**
    ///
    /// Configures Slice Header remain bit number
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const SLICE_REMAIN_BITLENGTH_OFFSET: u32 = 0;
    pub const SLICE_REMAIN_BITLENGTH_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **SLICE_REMAIN_BIT** `[10:3]` (RW)
    ///
    /// Configures Slice Header remain bit
    #[inline(always)]
    pub fn slice_remain_bit(&self) -> u32 {
        (self.value.read_bits() & Self::SLICE_REMAIN_BIT_MASK) >> Self::SLICE_REMAIN_BIT_OFFSET
    }

    /// Write **SLICE_REMAIN_BIT** `[10:3]` (RW)
    ///
    /// Configures Slice Header remain bit
    #[inline(always)]
    pub fn write_slice_remain_bit(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SLICE_REMAIN_BIT_MASK)
                | ((value << Self::SLICE_REMAIN_BIT_OFFSET) & Self::SLICE_REMAIN_BIT_MASK)
        });
    }

    /// **SLICE_REMAIN_BITLENGTH** `[2:0]` (RW)
    ///
    /// Configures Slice Header remain bit number
    #[inline(always)]
    pub fn slice_remain_bitlength(&self) -> u32 {
        (self.value.read_bits() & Self::SLICE_REMAIN_BITLENGTH_MASK)
            >> Self::SLICE_REMAIN_BITLENGTH_OFFSET
    }

    /// Write **SLICE_REMAIN_BITLENGTH** `[2:0]` (RW)
    ///
    /// Configures Slice Header remain bit number
    #[inline(always)]
    pub fn write_slice_remain_bitlength(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SLICE_REMAIN_BITLENGTH_MASK)
                | ((value << Self::SLICE_REMAIN_BITLENGTH_OFFSET)
                    & Self::SLICE_REMAIN_BITLENGTH_MASK)
        });
    }
}

/// SLICE_HEADER_REMAIN value type for batch read/modify/write operations
///
/// Use `sliceheaderremain.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sliceheaderremain.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SliceHeaderRemainVal {
    bits: u32,
}

impl SliceHeaderRemainVal {
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

    /// Read **SLICE_REMAIN_BIT** `[10:3]`
    ///
    /// Configures Slice Header remain bit
    #[inline(always)]
    pub const fn slice_remain_bit(&self) -> u32 {
        (self.bits & SliceHeaderRemain::SLICE_REMAIN_BIT_MASK)
            >> SliceHeaderRemain::SLICE_REMAIN_BIT_OFFSET
    }

    /// Read **SLICE_REMAIN_BITLENGTH** `[2:0]`
    ///
    /// Configures Slice Header remain bit number
    #[inline(always)]
    pub const fn slice_remain_bitlength(&self) -> u32 {
        (self.bits & SliceHeaderRemain::SLICE_REMAIN_BITLENGTH_MASK)
            >> SliceHeaderRemain::SLICE_REMAIN_BITLENGTH_OFFSET
    }

    /// Set **SLICE_REMAIN_BIT** `[10:3]`
    ///
    /// Configures Slice Header remain bit
    #[inline(always)]
    pub const fn with_slice_remain_bit(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SliceHeaderRemain::SLICE_REMAIN_BIT_MASK)
                | ((value << SliceHeaderRemain::SLICE_REMAIN_BIT_OFFSET)
                    & SliceHeaderRemain::SLICE_REMAIN_BIT_MASK),
        }
    }

    /// Set **SLICE_REMAIN_BITLENGTH** `[2:0]`
    ///
    /// Configures Slice Header remain bit number
    #[inline(always)]
    pub const fn with_slice_remain_bitlength(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SliceHeaderRemain::SLICE_REMAIN_BITLENGTH_MASK)
                | ((value << SliceHeaderRemain::SLICE_REMAIN_BITLENGTH_OFFSET)
                    & SliceHeaderRemain::SLICE_REMAIN_BITLENGTH_MASK),
        }
    }
}

impl Default for SliceHeaderRemainVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SliceHeaderRemain {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SliceHeaderRemainVal {
        SliceHeaderRemainVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SliceHeaderRemainVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SliceHeaderRemainVal) -> SliceHeaderRemainVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SLICE_HEADER_BYTE_LENGTH Register
///
/// Frame Slice Header byte length register.
///
/// Address offset: 0x00b0
///
/// ## Fields
///
/// - **SLICE_BYTE_LENGTH** `[3:0]` (RW)
///
///   Configures Slice Header byte number
#[repr(transparent)]
pub struct SliceHeaderByteLength {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SliceHeaderByteLength {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SliceHeaderByteLength {
    // ======== Field Constants ========

    /// **SLICE_BYTE_LENGTH**
    ///
    /// Configures Slice Header byte number
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const SLICE_BYTE_LENGTH_OFFSET: u32 = 0;
    pub const SLICE_BYTE_LENGTH_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **SLICE_BYTE_LENGTH** `[3:0]` (RW)
    ///
    /// Configures Slice Header byte number
    #[inline(always)]
    pub fn slice_byte_length(&self) -> u32 {
        (self.value.read_bits() & Self::SLICE_BYTE_LENGTH_MASK) >> Self::SLICE_BYTE_LENGTH_OFFSET
    }

    /// Write **SLICE_BYTE_LENGTH** `[3:0]` (RW)
    ///
    /// Configures Slice Header byte number
    #[inline(always)]
    pub fn write_slice_byte_length(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SLICE_BYTE_LENGTH_MASK)
                | ((value << Self::SLICE_BYTE_LENGTH_OFFSET) & Self::SLICE_BYTE_LENGTH_MASK)
        });
    }
}

/// SLICE_HEADER_BYTE_LENGTH value type for batch read/modify/write operations
///
/// Use `sliceheaderbytelength.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sliceheaderbytelength.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SliceHeaderByteLengthVal {
    bits: u32,
}

impl SliceHeaderByteLengthVal {
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

    /// Read **SLICE_BYTE_LENGTH** `[3:0]`
    ///
    /// Configures Slice Header byte number
    #[inline(always)]
    pub const fn slice_byte_length(&self) -> u32 {
        (self.bits & SliceHeaderByteLength::SLICE_BYTE_LENGTH_MASK)
            >> SliceHeaderByteLength::SLICE_BYTE_LENGTH_OFFSET
    }

    /// Set **SLICE_BYTE_LENGTH** `[3:0]`
    ///
    /// Configures Slice Header byte number
    #[inline(always)]
    pub const fn with_slice_byte_length(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SliceHeaderByteLength::SLICE_BYTE_LENGTH_MASK)
                | ((value << SliceHeaderByteLength::SLICE_BYTE_LENGTH_OFFSET)
                    & SliceHeaderByteLength::SLICE_BYTE_LENGTH_MASK),
        }
    }
}

impl Default for SliceHeaderByteLengthVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SliceHeaderByteLength {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SliceHeaderByteLengthVal {
        SliceHeaderByteLengthVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SliceHeaderByteLengthVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SliceHeaderByteLengthVal) -> SliceHeaderByteLengthVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// BS_THRESHOLD Register
///
/// Bitstream buffer overflow threshold register
///
/// Address offset: 0x00b4
/// Reset value: 0x00000030
///
/// ## Fields
///
/// - **BS_BUFFER_THRESHOLD** `[6:0]` (RW)
///
///   Configures bitstream buffer overflow threshold. This value should be bigger than the encode bytes of one 4x4 submb.
#[repr(transparent)]
pub struct BsThreshold {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BsThreshold {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BsThreshold {
    // ======== Field Constants ========

    /// **BS_BUFFER_THRESHOLD**
    ///
    /// Configures bitstream buffer overflow threshold. This value should be bigger than the encode bytes of one 4x4 submb.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const BS_BUFFER_THRESHOLD_OFFSET: u32 = 0;
    pub const BS_BUFFER_THRESHOLD_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **BS_BUFFER_THRESHOLD** `[6:0]` (RW)
    ///
    /// Configures bitstream buffer overflow threshold. This value should be bigger than the encode bytes of one 4x4 submb.
    #[inline(always)]
    pub fn bs_buffer_threshold(&self) -> u32 {
        (self.value.read_bits() & Self::BS_BUFFER_THRESHOLD_MASK)
            >> Self::BS_BUFFER_THRESHOLD_OFFSET
    }

    /// Write **BS_BUFFER_THRESHOLD** `[6:0]` (RW)
    ///
    /// Configures bitstream buffer overflow threshold. This value should be bigger than the encode bytes of one 4x4 submb.
    #[inline(always)]
    pub fn write_bs_buffer_threshold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BS_BUFFER_THRESHOLD_MASK)
                | ((value << Self::BS_BUFFER_THRESHOLD_OFFSET) & Self::BS_BUFFER_THRESHOLD_MASK)
        });
    }
}

/// BS_THRESHOLD value type for batch read/modify/write operations
///
/// Use `bsthreshold.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bsthreshold.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BsThresholdVal {
    bits: u32,
}

impl BsThresholdVal {
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
        Self { bits: 0x00000030 }
    }

    /// Read **BS_BUFFER_THRESHOLD** `[6:0]`
    ///
    /// Configures bitstream buffer overflow threshold. This value should be bigger than the encode bytes of one 4x4 submb.
    #[inline(always)]
    pub const fn bs_buffer_threshold(&self) -> u32 {
        (self.bits & BsThreshold::BS_BUFFER_THRESHOLD_MASK)
            >> BsThreshold::BS_BUFFER_THRESHOLD_OFFSET
    }

    /// Set **BS_BUFFER_THRESHOLD** `[6:0]`
    ///
    /// Configures bitstream buffer overflow threshold. This value should be bigger than the encode bytes of one 4x4 submb.
    #[inline(always)]
    pub const fn with_bs_buffer_threshold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BsThreshold::BS_BUFFER_THRESHOLD_MASK)
                | ((value << BsThreshold::BS_BUFFER_THRESHOLD_OFFSET)
                    & BsThreshold::BS_BUFFER_THRESHOLD_MASK),
        }
    }
}

impl Default for BsThresholdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BsThreshold {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BsThresholdVal {
        BsThresholdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BsThresholdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BsThresholdVal) -> BsThresholdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SLICE_HEADER_BYTE0 Register
///
/// Frame Slice Header byte low 32 bit register.
///
/// Address offset: 0x00b8
///
/// ## Fields
///
/// - **SLICE_BYTE_LSB** `[31:0]` (RW)
///
///   Configures Slice Header low 32 bit
#[repr(transparent)]
pub struct SliceHeaderByte0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SliceHeaderByte0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SliceHeaderByte0 {
    // ======== Field Constants ========

    /// **SLICE_BYTE_LSB**
    ///
    /// Configures Slice Header low 32 bit
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SLICE_BYTE_LSB_OFFSET: u32 = 0;
    pub const SLICE_BYTE_LSB_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SLICE_BYTE_LSB** `[31:0]` (RW)
    ///
    /// Configures Slice Header low 32 bit
    #[inline(always)]
    pub fn slice_byte_lsb(&self) -> u32 {
        (self.value.read_bits() & Self::SLICE_BYTE_LSB_MASK) >> Self::SLICE_BYTE_LSB_OFFSET
    }

    /// Write **SLICE_BYTE_LSB** `[31:0]` (RW)
    ///
    /// Configures Slice Header low 32 bit
    #[inline(always)]
    pub fn write_slice_byte_lsb(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SLICE_BYTE_LSB_MASK)
                | ((value << Self::SLICE_BYTE_LSB_OFFSET) & Self::SLICE_BYTE_LSB_MASK)
        });
    }
}

/// SLICE_HEADER_BYTE0 value type for batch read/modify/write operations
///
/// Use `sliceheaderbyte0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sliceheaderbyte0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SliceHeaderByte0Val {
    bits: u32,
}

impl SliceHeaderByte0Val {
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

    /// Read **SLICE_BYTE_LSB** `[31:0]`
    ///
    /// Configures Slice Header low 32 bit
    #[inline(always)]
    pub const fn slice_byte_lsb(&self) -> u32 {
        (self.bits & SliceHeaderByte0::SLICE_BYTE_LSB_MASK)
            >> SliceHeaderByte0::SLICE_BYTE_LSB_OFFSET
    }

    /// Set **SLICE_BYTE_LSB** `[31:0]`
    ///
    /// Configures Slice Header low 32 bit
    #[inline(always)]
    pub const fn with_slice_byte_lsb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SliceHeaderByte0::SLICE_BYTE_LSB_MASK)
                | ((value << SliceHeaderByte0::SLICE_BYTE_LSB_OFFSET)
                    & SliceHeaderByte0::SLICE_BYTE_LSB_MASK),
        }
    }
}

impl Default for SliceHeaderByte0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SliceHeaderByte0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SliceHeaderByte0Val {
        SliceHeaderByte0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SliceHeaderByte0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SliceHeaderByte0Val) -> SliceHeaderByte0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SLICE_HEADER_BYTE1 Register
///
/// Frame Slice Header byte high 32 bit register.
///
/// Address offset: 0x00bc
///
/// ## Fields
///
/// - **SLICE_BYTE_MSB** `[31:0]` (RW)
///
///   Configures Slice Header high 32 bit
#[repr(transparent)]
pub struct SliceHeaderByte1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SliceHeaderByte1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SliceHeaderByte1 {
    // ======== Field Constants ========

    /// **SLICE_BYTE_MSB**
    ///
    /// Configures Slice Header high 32 bit
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SLICE_BYTE_MSB_OFFSET: u32 = 0;
    pub const SLICE_BYTE_MSB_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SLICE_BYTE_MSB** `[31:0]` (RW)
    ///
    /// Configures Slice Header high 32 bit
    #[inline(always)]
    pub fn slice_byte_msb(&self) -> u32 {
        (self.value.read_bits() & Self::SLICE_BYTE_MSB_MASK) >> Self::SLICE_BYTE_MSB_OFFSET
    }

    /// Write **SLICE_BYTE_MSB** `[31:0]` (RW)
    ///
    /// Configures Slice Header high 32 bit
    #[inline(always)]
    pub fn write_slice_byte_msb(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SLICE_BYTE_MSB_MASK)
                | ((value << Self::SLICE_BYTE_MSB_OFFSET) & Self::SLICE_BYTE_MSB_MASK)
        });
    }
}

/// SLICE_HEADER_BYTE1 value type for batch read/modify/write operations
///
/// Use `sliceheaderbyte1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sliceheaderbyte1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SliceHeaderByte1Val {
    bits: u32,
}

impl SliceHeaderByte1Val {
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

    /// Read **SLICE_BYTE_MSB** `[31:0]`
    ///
    /// Configures Slice Header high 32 bit
    #[inline(always)]
    pub const fn slice_byte_msb(&self) -> u32 {
        (self.bits & SliceHeaderByte1::SLICE_BYTE_MSB_MASK)
            >> SliceHeaderByte1::SLICE_BYTE_MSB_OFFSET
    }

    /// Set **SLICE_BYTE_MSB** `[31:0]`
    ///
    /// Configures Slice Header high 32 bit
    #[inline(always)]
    pub const fn with_slice_byte_msb(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SliceHeaderByte1::SLICE_BYTE_MSB_MASK)
                | ((value << SliceHeaderByte1::SLICE_BYTE_MSB_OFFSET)
                    & SliceHeaderByte1::SLICE_BYTE_MSB_MASK),
        }
    }
}

impl Default for SliceHeaderByte1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SliceHeaderByte1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SliceHeaderByte1Val {
        SliceHeaderByte1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SliceHeaderByte1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SliceHeaderByte1Val) -> SliceHeaderByte1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_RAW Register
///
/// Interrupt raw status register
///
/// Address offset: 0x00c0
///
/// ## Fields
///
/// - **DMA_MOVE_2MB_LINE_DONE_INT_RAW** `[3]` (RW)
///
///   Raw status bit: The raw interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Triggered when H264 move two MB lines of reference frame from external mem to internal mem done.
///
/// - **FRAME_DONE_INT_RAW** `[2]` (RW)
///
///   Raw status bit: The raw interrupt status of H264_FRAME_DONE_INT. Triggered when H264 encoding one frame done.
///
/// - **REC_READY_INT_RAW** `[1]` (RW)
///
///   Raw status bit: The raw interrupt status of H264_REC_READY_INT. Triggered when H264 encoding enough reconstruct pixel.
///
/// - **DB_TMP_READY_INT_RAW** `[0]` (RW)
///
///   Raw status bit: The raw interrupt status of H264_DB_TMP_READY_INT. Triggered when H264 written enough db tmp pixel.
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

    /// **DMA_MOVE_2MB_LINE_DONE_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Triggered when H264 move two MB lines of reference frame from external mem to internal mem done.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DMA_MOVE_2MB_LINE_DONE_INT_RAW_OFFSET: u32 = 3;
    pub const DMA_MOVE_2MB_LINE_DONE_INT_RAW_MASK: u32 = 0x00000008;
    pub const DMA_MOVE_2MB_LINE_DONE_INT_RAW: u32 = 1 << 3;

    /// **FRAME_DONE_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of H264_FRAME_DONE_INT. Triggered when H264 encoding one frame done.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FRAME_DONE_INT_RAW_OFFSET: u32 = 2;
    pub const FRAME_DONE_INT_RAW_MASK: u32 = 0x00000004;
    pub const FRAME_DONE_INT_RAW: u32 = 1 << 2;

    /// **REC_READY_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of H264_REC_READY_INT. Triggered when H264 encoding enough reconstruct pixel.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REC_READY_INT_RAW_OFFSET: u32 = 1;
    pub const REC_READY_INT_RAW_MASK: u32 = 0x00000002;
    pub const REC_READY_INT_RAW: u32 = 1 << 1;

    /// **DB_TMP_READY_INT_RAW**
    ///
    /// Raw status bit: The raw interrupt status of H264_DB_TMP_READY_INT. Triggered when H264 written enough db tmp pixel.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DB_TMP_READY_INT_RAW_OFFSET: u32 = 0;
    pub const DB_TMP_READY_INT_RAW_MASK: u32 = 0x00000001;
    pub const DB_TMP_READY_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DMA_MOVE_2MB_LINE_DONE_INT_RAW** `[3]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Triggered when H264 move two MB lines of reference frame from external mem to internal mem done.
    #[inline(always)]
    pub fn dma_move_2mb_line_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MOVE_2MB_LINE_DONE_INT_RAW_MASK) != 0
    }

    /// Set DMA_MOVE_2MB_LINE_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma_move_2mb_line_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DMA_MOVE_2MB_LINE_DONE_INT_RAW_MASK);
    }

    /// Clear DMA_MOVE_2MB_LINE_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma_move_2mb_line_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DMA_MOVE_2MB_LINE_DONE_INT_RAW_MASK);
    }

    /// **FRAME_DONE_INT_RAW** `[2]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of H264_FRAME_DONE_INT. Triggered when H264 encoding one frame done.
    #[inline(always)]
    pub fn frame_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::FRAME_DONE_INT_RAW_MASK) != 0
    }

    /// Set FRAME_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_frame_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::FRAME_DONE_INT_RAW_MASK);
    }

    /// Clear FRAME_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_frame_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::FRAME_DONE_INT_RAW_MASK);
    }

    /// **REC_READY_INT_RAW** `[1]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of H264_REC_READY_INT. Triggered when H264 encoding enough reconstruct pixel.
    #[inline(always)]
    pub fn rec_ready_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::REC_READY_INT_RAW_MASK) != 0
    }

    /// Set REC_READY_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rec_ready_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::REC_READY_INT_RAW_MASK);
    }

    /// Clear REC_READY_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rec_ready_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::REC_READY_INT_RAW_MASK);
    }

    /// **DB_TMP_READY_INT_RAW** `[0]` (RW)
    ///
    /// Raw status bit: The raw interrupt status of H264_DB_TMP_READY_INT. Triggered when H264 written enough db tmp pixel.
    #[inline(always)]
    pub fn db_tmp_ready_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DB_TMP_READY_INT_RAW_MASK) != 0
    }

    /// Set DB_TMP_READY_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_db_tmp_ready_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DB_TMP_READY_INT_RAW_MASK);
    }

    /// Clear DB_TMP_READY_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_db_tmp_ready_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DB_TMP_READY_INT_RAW_MASK);
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

    /// Read **DMA_MOVE_2MB_LINE_DONE_INT_RAW** `[3:3]`
    ///
    /// Raw status bit: The raw interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Triggered when H264 move two MB lines of reference frame from external mem to internal mem done.
    #[inline(always)]
    pub const fn dma_move_2mb_line_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::DMA_MOVE_2MB_LINE_DONE_INT_RAW_MASK) != 0
    }

    /// Read **FRAME_DONE_INT_RAW** `[2:2]`
    ///
    /// Raw status bit: The raw interrupt status of H264_FRAME_DONE_INT. Triggered when H264 encoding one frame done.
    #[inline(always)]
    pub const fn frame_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::FRAME_DONE_INT_RAW_MASK) != 0
    }

    /// Read **REC_READY_INT_RAW** `[1:1]`
    ///
    /// Raw status bit: The raw interrupt status of H264_REC_READY_INT. Triggered when H264 encoding enough reconstruct pixel.
    #[inline(always)]
    pub const fn rec_ready_int_raw(&self) -> bool {
        (self.bits & IntRaw::REC_READY_INT_RAW_MASK) != 0
    }

    /// Read **DB_TMP_READY_INT_RAW** `[0:0]`
    ///
    /// Raw status bit: The raw interrupt status of H264_DB_TMP_READY_INT. Triggered when H264 written enough db tmp pixel.
    #[inline(always)]
    pub const fn db_tmp_ready_int_raw(&self) -> bool {
        (self.bits & IntRaw::DB_TMP_READY_INT_RAW_MASK) != 0
    }

    /// Set **DMA_MOVE_2MB_LINE_DONE_INT_RAW** `[3:3]`
    ///
    /// Raw status bit: The raw interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Triggered when H264 move two MB lines of reference frame from external mem to internal mem done.
    #[inline(always)]
    pub const fn with_dma_move_2mb_line_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DMA_MOVE_2MB_LINE_DONE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DMA_MOVE_2MB_LINE_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **FRAME_DONE_INT_RAW** `[2:2]`
    ///
    /// Raw status bit: The raw interrupt status of H264_FRAME_DONE_INT. Triggered when H264 encoding one frame done.
    #[inline(always)]
    pub const fn with_frame_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::FRAME_DONE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::FRAME_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **REC_READY_INT_RAW** `[1:1]`
    ///
    /// Raw status bit: The raw interrupt status of H264_REC_READY_INT. Triggered when H264 encoding enough reconstruct pixel.
    #[inline(always)]
    pub const fn with_rec_ready_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::REC_READY_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::REC_READY_INT_RAW_MASK
            },
        }
    }

    /// Set **DB_TMP_READY_INT_RAW** `[0:0]`
    ///
    /// Raw status bit: The raw interrupt status of H264_DB_TMP_READY_INT. Triggered when H264 written enough db tmp pixel.
    #[inline(always)]
    pub const fn with_db_tmp_ready_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DB_TMP_READY_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DB_TMP_READY_INT_RAW_MASK
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
/// Interrupt masked status register
///
/// Address offset: 0x00c4
///
/// ## Fields
///
/// - **DMA_MOVE_2MB_LINE_DONE_INT_ST** `[3]` (RO)
///
///   Masked status bit: The masked interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Valid only when the H264_DMA_MOVE_2MB_LINE_DONE_INT_ENA is set to 1.
///
/// - **FRAME_DONE_INT_ST** `[2]` (RO)
///
///   The masked interrupt status of H264_FRAME_DONE_INT. Valid only when the H264_FRAME_DONE_INT_ENA is set to 1.
///
/// - **REC_READY_INT_ST** `[1]` (RO)
///
///   The masked interrupt status of H264_REC_READY_INT. Valid only when the H264_REC_READY_INT_ENA is set to 1.
///
/// - **DB_TMP_READY_INT_ST** `[0]` (RO)
///
///   The masked interrupt status of H264_DB_TMP_READY_INT. Valid only when the H264_DB_TMP_READY_INT_ENA is set to 1.
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

    /// **DMA_MOVE_2MB_LINE_DONE_INT_ST**
    ///
    /// Masked status bit: The masked interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Valid only when the H264_DMA_MOVE_2MB_LINE_DONE_INT_ENA is set to 1.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DMA_MOVE_2MB_LINE_DONE_INT_ST_OFFSET: u32 = 3;
    pub const DMA_MOVE_2MB_LINE_DONE_INT_ST_MASK: u32 = 0x00000008;
    pub const DMA_MOVE_2MB_LINE_DONE_INT_ST: u32 = 1 << 3;

    /// **FRAME_DONE_INT_ST**
    ///
    /// The masked interrupt status of H264_FRAME_DONE_INT. Valid only when the H264_FRAME_DONE_INT_ENA is set to 1.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FRAME_DONE_INT_ST_OFFSET: u32 = 2;
    pub const FRAME_DONE_INT_ST_MASK: u32 = 0x00000004;
    pub const FRAME_DONE_INT_ST: u32 = 1 << 2;

    /// **REC_READY_INT_ST**
    ///
    /// The masked interrupt status of H264_REC_READY_INT. Valid only when the H264_REC_READY_INT_ENA is set to 1.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REC_READY_INT_ST_OFFSET: u32 = 1;
    pub const REC_READY_INT_ST_MASK: u32 = 0x00000002;
    pub const REC_READY_INT_ST: u32 = 1 << 1;

    /// **DB_TMP_READY_INT_ST**
    ///
    /// The masked interrupt status of H264_DB_TMP_READY_INT. Valid only when the H264_DB_TMP_READY_INT_ENA is set to 1.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DB_TMP_READY_INT_ST_OFFSET: u32 = 0;
    pub const DB_TMP_READY_INT_ST_MASK: u32 = 0x00000001;
    pub const DB_TMP_READY_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DMA_MOVE_2MB_LINE_DONE_INT_ST** `[3]` (RO)
    ///
    /// Masked status bit: The masked interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Valid only when the H264_DMA_MOVE_2MB_LINE_DONE_INT_ENA is set to 1.
    #[inline(always)]
    pub fn dma_move_2mb_line_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MOVE_2MB_LINE_DONE_INT_ST_MASK) != 0
    }

    /// **FRAME_DONE_INT_ST** `[2]` (RO)
    ///
    /// The masked interrupt status of H264_FRAME_DONE_INT. Valid only when the H264_FRAME_DONE_INT_ENA is set to 1.
    #[inline(always)]
    pub fn frame_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::FRAME_DONE_INT_ST_MASK) != 0
    }

    /// **REC_READY_INT_ST** `[1]` (RO)
    ///
    /// The masked interrupt status of H264_REC_READY_INT. Valid only when the H264_REC_READY_INT_ENA is set to 1.
    #[inline(always)]
    pub fn rec_ready_int_st(&self) -> bool {
        (self.value.read_bits() & Self::REC_READY_INT_ST_MASK) != 0
    }

    /// **DB_TMP_READY_INT_ST** `[0]` (RO)
    ///
    /// The masked interrupt status of H264_DB_TMP_READY_INT. Valid only when the H264_DB_TMP_READY_INT_ENA is set to 1.
    #[inline(always)]
    pub fn db_tmp_ready_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DB_TMP_READY_INT_ST_MASK) != 0
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

    /// Read **DMA_MOVE_2MB_LINE_DONE_INT_ST** `[3:3]`
    ///
    /// Masked status bit: The masked interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Valid only when the H264_DMA_MOVE_2MB_LINE_DONE_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn dma_move_2mb_line_done_int_st(&self) -> bool {
        (self.bits & IntSt::DMA_MOVE_2MB_LINE_DONE_INT_ST_MASK) != 0
    }

    /// Read **FRAME_DONE_INT_ST** `[2:2]`
    ///
    /// The masked interrupt status of H264_FRAME_DONE_INT. Valid only when the H264_FRAME_DONE_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn frame_done_int_st(&self) -> bool {
        (self.bits & IntSt::FRAME_DONE_INT_ST_MASK) != 0
    }

    /// Read **REC_READY_INT_ST** `[1:1]`
    ///
    /// The masked interrupt status of H264_REC_READY_INT. Valid only when the H264_REC_READY_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn rec_ready_int_st(&self) -> bool {
        (self.bits & IntSt::REC_READY_INT_ST_MASK) != 0
    }

    /// Read **DB_TMP_READY_INT_ST** `[0:0]`
    ///
    /// The masked interrupt status of H264_DB_TMP_READY_INT. Valid only when the H264_DB_TMP_READY_INT_ENA is set to 1.
    #[inline(always)]
    pub const fn db_tmp_ready_int_st(&self) -> bool {
        (self.bits & IntSt::DB_TMP_READY_INT_ST_MASK) != 0
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
/// Interrupt enable register
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **DMA_MOVE_2MB_LINE_DONE_INT_ENA** `[3]` (RW)
///
///   Enable bit: Write 1 to enable H264_DMA_MOVE_2MB_LINE_DONE_INT.
///
/// - **FRAME_DONE_INT_ENA** `[2]` (RW)
///
///   Write 1 to enable H264_FRAME_DONE_INT.
///
/// - **REC_READY_INT_ENA** `[1]` (RW)
///
///   Write 1 to enable H264_REC_READY_INT.
///
/// - **DB_TMP_READY_INT_ENA** `[0]` (RW)
///
///   Write 1 to enable H264_DB_TMP_READY_INT.
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

    /// **DMA_MOVE_2MB_LINE_DONE_INT_ENA**
    ///
    /// Enable bit: Write 1 to enable H264_DMA_MOVE_2MB_LINE_DONE_INT.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DMA_MOVE_2MB_LINE_DONE_INT_ENA_OFFSET: u32 = 3;
    pub const DMA_MOVE_2MB_LINE_DONE_INT_ENA_MASK: u32 = 0x00000008;
    pub const DMA_MOVE_2MB_LINE_DONE_INT_ENA: u32 = 1 << 3;

    /// **FRAME_DONE_INT_ENA**
    ///
    /// Write 1 to enable H264_FRAME_DONE_INT.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FRAME_DONE_INT_ENA_OFFSET: u32 = 2;
    pub const FRAME_DONE_INT_ENA_MASK: u32 = 0x00000004;
    pub const FRAME_DONE_INT_ENA: u32 = 1 << 2;

    /// **REC_READY_INT_ENA**
    ///
    /// Write 1 to enable H264_REC_READY_INT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REC_READY_INT_ENA_OFFSET: u32 = 1;
    pub const REC_READY_INT_ENA_MASK: u32 = 0x00000002;
    pub const REC_READY_INT_ENA: u32 = 1 << 1;

    /// **DB_TMP_READY_INT_ENA**
    ///
    /// Write 1 to enable H264_DB_TMP_READY_INT.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DB_TMP_READY_INT_ENA_OFFSET: u32 = 0;
    pub const DB_TMP_READY_INT_ENA_MASK: u32 = 0x00000001;
    pub const DB_TMP_READY_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DMA_MOVE_2MB_LINE_DONE_INT_ENA** `[3]` (RW)
    ///
    /// Enable bit: Write 1 to enable H264_DMA_MOVE_2MB_LINE_DONE_INT.
    #[inline(always)]
    pub fn dma_move_2mb_line_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MOVE_2MB_LINE_DONE_INT_ENA_MASK) != 0
    }

    /// Set DMA_MOVE_2MB_LINE_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma_move_2mb_line_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DMA_MOVE_2MB_LINE_DONE_INT_ENA_MASK);
    }

    /// Clear DMA_MOVE_2MB_LINE_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma_move_2mb_line_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DMA_MOVE_2MB_LINE_DONE_INT_ENA_MASK);
    }

    /// **FRAME_DONE_INT_ENA** `[2]` (RW)
    ///
    /// Write 1 to enable H264_FRAME_DONE_INT.
    #[inline(always)]
    pub fn frame_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::FRAME_DONE_INT_ENA_MASK) != 0
    }

    /// Set FRAME_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_frame_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::FRAME_DONE_INT_ENA_MASK);
    }

    /// Clear FRAME_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_frame_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::FRAME_DONE_INT_ENA_MASK);
    }

    /// **REC_READY_INT_ENA** `[1]` (RW)
    ///
    /// Write 1 to enable H264_REC_READY_INT.
    #[inline(always)]
    pub fn rec_ready_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::REC_READY_INT_ENA_MASK) != 0
    }

    /// Set REC_READY_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rec_ready_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::REC_READY_INT_ENA_MASK);
    }

    /// Clear REC_READY_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rec_ready_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::REC_READY_INT_ENA_MASK);
    }

    /// **DB_TMP_READY_INT_ENA** `[0]` (RW)
    ///
    /// Write 1 to enable H264_DB_TMP_READY_INT.
    #[inline(always)]
    pub fn db_tmp_ready_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DB_TMP_READY_INT_ENA_MASK) != 0
    }

    /// Set DB_TMP_READY_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_db_tmp_ready_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DB_TMP_READY_INT_ENA_MASK);
    }

    /// Clear DB_TMP_READY_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_db_tmp_ready_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DB_TMP_READY_INT_ENA_MASK);
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

    /// Read **DMA_MOVE_2MB_LINE_DONE_INT_ENA** `[3:3]`
    ///
    /// Enable bit: Write 1 to enable H264_DMA_MOVE_2MB_LINE_DONE_INT.
    #[inline(always)]
    pub const fn dma_move_2mb_line_done_int_ena(&self) -> bool {
        (self.bits & IntEna::DMA_MOVE_2MB_LINE_DONE_INT_ENA_MASK) != 0
    }

    /// Read **FRAME_DONE_INT_ENA** `[2:2]`
    ///
    /// Write 1 to enable H264_FRAME_DONE_INT.
    #[inline(always)]
    pub const fn frame_done_int_ena(&self) -> bool {
        (self.bits & IntEna::FRAME_DONE_INT_ENA_MASK) != 0
    }

    /// Read **REC_READY_INT_ENA** `[1:1]`
    ///
    /// Write 1 to enable H264_REC_READY_INT.
    #[inline(always)]
    pub const fn rec_ready_int_ena(&self) -> bool {
        (self.bits & IntEna::REC_READY_INT_ENA_MASK) != 0
    }

    /// Read **DB_TMP_READY_INT_ENA** `[0:0]`
    ///
    /// Write 1 to enable H264_DB_TMP_READY_INT.
    #[inline(always)]
    pub const fn db_tmp_ready_int_ena(&self) -> bool {
        (self.bits & IntEna::DB_TMP_READY_INT_ENA_MASK) != 0
    }

    /// Set **DMA_MOVE_2MB_LINE_DONE_INT_ENA** `[3:3]`
    ///
    /// Enable bit: Write 1 to enable H264_DMA_MOVE_2MB_LINE_DONE_INT.
    #[inline(always)]
    pub const fn with_dma_move_2mb_line_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DMA_MOVE_2MB_LINE_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DMA_MOVE_2MB_LINE_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **FRAME_DONE_INT_ENA** `[2:2]`
    ///
    /// Write 1 to enable H264_FRAME_DONE_INT.
    #[inline(always)]
    pub const fn with_frame_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::FRAME_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::FRAME_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **REC_READY_INT_ENA** `[1:1]`
    ///
    /// Write 1 to enable H264_REC_READY_INT.
    #[inline(always)]
    pub const fn with_rec_ready_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::REC_READY_INT_ENA_MASK
            } else {
                self.bits & !IntEna::REC_READY_INT_ENA_MASK
            },
        }
    }

    /// Set **DB_TMP_READY_INT_ENA** `[0:0]`
    ///
    /// Write 1 to enable H264_DB_TMP_READY_INT.
    #[inline(always)]
    pub const fn with_db_tmp_ready_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DB_TMP_READY_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DB_TMP_READY_INT_ENA_MASK
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
/// Interrupt clear register
///
/// Address offset: 0x00cc
///
/// ## Fields
///
/// - **DMA_MOVE_2MB_LINE_DONE_INT_CLR** `[3]` (WO)
///
///   Clear bit: Write 1 to clear H264_DMA_MOVE_2MB_LINE_DONE_INT.
///
/// - **FRAME_DONE_INT_CLR** `[2]` (WO)
///
///   Write 1 to clear H264_FRAME_DONE_INT.
///
/// - **REC_READY_INT_CLR** `[1]` (WO)
///
///   Write 1 to clear H264_REC_READY_INT.
///
/// - **DB_TMP_READY_INT_CLR** `[0]` (WO)
///
///   Write 1 to clear H264_DB_TMP_READY_INT.
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

    /// **DMA_MOVE_2MB_LINE_DONE_INT_CLR**
    ///
    /// Clear bit: Write 1 to clear H264_DMA_MOVE_2MB_LINE_DONE_INT.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DMA_MOVE_2MB_LINE_DONE_INT_CLR_OFFSET: u32 = 3;
    pub const DMA_MOVE_2MB_LINE_DONE_INT_CLR_MASK: u32 = 0x00000008;
    pub const DMA_MOVE_2MB_LINE_DONE_INT_CLR: u32 = 1 << 3;

    /// **FRAME_DONE_INT_CLR**
    ///
    /// Write 1 to clear H264_FRAME_DONE_INT.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FRAME_DONE_INT_CLR_OFFSET: u32 = 2;
    pub const FRAME_DONE_INT_CLR_MASK: u32 = 0x00000004;
    pub const FRAME_DONE_INT_CLR: u32 = 1 << 2;

    /// **REC_READY_INT_CLR**
    ///
    /// Write 1 to clear H264_REC_READY_INT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REC_READY_INT_CLR_OFFSET: u32 = 1;
    pub const REC_READY_INT_CLR_MASK: u32 = 0x00000002;
    pub const REC_READY_INT_CLR: u32 = 1 << 1;

    /// **DB_TMP_READY_INT_CLR**
    ///
    /// Write 1 to clear H264_DB_TMP_READY_INT.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DB_TMP_READY_INT_CLR_OFFSET: u32 = 0;
    pub const DB_TMP_READY_INT_CLR_MASK: u32 = 0x00000001;
    pub const DB_TMP_READY_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **DMA_MOVE_2MB_LINE_DONE_INT_CLR** (WO)
    ///
    /// Clear bit: Write 1 to clear H264_DMA_MOVE_2MB_LINE_DONE_INT.
    #[inline(always)]
    pub fn trigger_dma_move_2mb_line_done_int_clr(&self) {
        self.value
            .write_bits(Self::DMA_MOVE_2MB_LINE_DONE_INT_CLR_MASK);
    }

    /// Trigger **FRAME_DONE_INT_CLR** (WO)
    ///
    /// Write 1 to clear H264_FRAME_DONE_INT.
    #[inline(always)]
    pub fn trigger_frame_done_int_clr(&self) {
        self.value.write_bits(Self::FRAME_DONE_INT_CLR_MASK);
    }

    /// Trigger **REC_READY_INT_CLR** (WO)
    ///
    /// Write 1 to clear H264_REC_READY_INT.
    #[inline(always)]
    pub fn trigger_rec_ready_int_clr(&self) {
        self.value.write_bits(Self::REC_READY_INT_CLR_MASK);
    }

    /// Trigger **DB_TMP_READY_INT_CLR** (WO)
    ///
    /// Write 1 to clear H264_DB_TMP_READY_INT.
    #[inline(always)]
    pub fn trigger_db_tmp_ready_int_clr(&self) {
        self.value.write_bits(Self::DB_TMP_READY_INT_CLR_MASK);
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

    /// Set **DMA_MOVE_2MB_LINE_DONE_INT_CLR** `[3:3]`
    ///
    /// Clear bit: Write 1 to clear H264_DMA_MOVE_2MB_LINE_DONE_INT.
    #[inline(always)]
    pub const fn with_dma_move_2mb_line_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DMA_MOVE_2MB_LINE_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DMA_MOVE_2MB_LINE_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **FRAME_DONE_INT_CLR** `[2:2]`
    ///
    /// Write 1 to clear H264_FRAME_DONE_INT.
    #[inline(always)]
    pub const fn with_frame_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::FRAME_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::FRAME_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **REC_READY_INT_CLR** `[1:1]`
    ///
    /// Write 1 to clear H264_REC_READY_INT.
    #[inline(always)]
    pub const fn with_rec_ready_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::REC_READY_INT_CLR_MASK
            } else {
                self.bits & !IntClr::REC_READY_INT_CLR_MASK
            },
        }
    }

    /// Set **DB_TMP_READY_INT_CLR** `[0:0]`
    ///
    /// Write 1 to clear H264_DB_TMP_READY_INT.
    #[inline(always)]
    pub const fn with_db_tmp_ready_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DB_TMP_READY_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DB_TMP_READY_INT_CLR_MASK
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

/// CONF Register
///
/// General configuration register.
///
/// Address offset: 0x00d0
///
/// ## Fields
///
/// - **MV_MERGE_CLK_EN** `[25]` (RW)
///
///   Configures whether or not to open the clock gate for mv merge.\\0: Open the clock gate only when mv merge work\\1: Force open the clock gate for mv merge
///
/// - **BS_CLK_EN** `[24]` (RW)
///
///   Configures whether or not to open the clock gate for bs buffer.\\0: Open the clock gate only when bs buffer work\\1: Force open the clock gate for bs buffer
///
/// - **DECI_CLK_EN** `[23]` (RW)
///
///   Configures whether or not to open the clock gate for decimate.\\0: Open the clock gate only when decimate work\\1: Force open the clock gate for decimate
///
/// - **INTRA_CLK_EN** `[22]` (RW)
///
///   Configures whether or not to open the clock gate for intra.\\0: Open the clock gate only when intra work\\1: Force open the clock gate for intra
///
/// - **CLAVLC_CLK_EN** `[21]` (RW)
///
///   Configures whether or not to open the clock gate for cavlc.\\0: Open the clock gate only when cavlc work\\1: Force open the clock gate for cavlc
///
/// - **DB_CLK_EN** `[20]` (RW)
///
///   Configures whether or not to open the clock gate for deblocking filter.\\0: Open the clock gate only when deblocking filter work\\1: Force open the clock gate for deblocking filter
///
/// - **INTERPOLATOR_CLK_EN** `[19]` (RW)
///
///   Configures whether or not to open the clock gate for interpolator.\\0: Open the clock gate only when interpolator work\\1: Force open the clock gate for interpolator
///
/// - **MC_CLK_EN** `[18]` (RW)
///
///   Configures whether or not to open the clock gate for mc.\\0: Open the clock gate only when mc work\\1: Force open the clock gate for mc
///
/// - **FME_CLK_EN** `[17]` (RW)
///
///   Configures whether or not to open the clock gate for fme.\\0: Open the clock gate only when fme work\\1: Force open the clock gate for fme
///
/// - **IME_CLK_EN** `[16]` (RW)
///
///   Configures whether or not to open the clock gate for ime.\\0: Open the clock gate only when ime work\\1: Force open the clock gate for ime
///
/// - **CAVLC_RAM_CLK_EN** `[15]` (RW)
///
///   Configures whether or not to open the clock gate for cavlc ram.\\0: Open the clock gate only when application writes or reads cavlc ram\\1: Force open the clock gate for cavlc ram
///
/// - **CUR_MB_RAM_CLK_EN** `[14]` (RW)
///
///   Configures whether or not to open the clock gate for cur_mb ram.\\0: Open the clock gate only when application writes or reads cur_mb ram\\1: Force open the clock gate for cur_mb ram
///
/// - **DB_RAM_CLK_EN** `[13]` (RW)
///
///   Configures whether or not to open the clock gate for db ram.\\0: Open the clock gate only when application writes or reads db ram\\1: Force open the clock gate for db ram
///
/// - **FETCH_RAM_CLK_EN** `[12]` (RW)
///
///   Configures whether or not to open the clock gate for fetch ram.\\0: Open the clock gate only when application writes or reads fetch ram\\1: Force open the clock gate for fetch ram
///
/// - **FME_RAM_CLK_EN** `[11]` (RW)
///
///   Configures whether or not to open the clock gate for fme ram.\\0: Open the clock gate only when application writes or readsfme ram\\1: Force open the clock gate for fme ram
///
/// - **IME_RAM_CLK_EN** `[10]` (RW)
///
///   Configures whether or not to open the clock gate for ime ram.\\0: Open the clock gate only when application writes or reads ime ram\\1: Force open the clock gate for ime ram
///
/// - **I4X4_REF_RAM_CLK_EN** `[9]` (RW)
///
///   Configures whether or not to open the clock gate for i4x4_mode ram.\\0: Open the clock gate only when application writes or reads i4x4_mode ram\\1: Force open the clock gate for i4x4_mode ram
///
/// - **REF_RAM_CLK_EN** `[8]` (RW)
///
///   Configures whether or not to open the clock gate for ref ram.\\0: Open the clock gate only when application writes or reads ref ram\\1: Force open the clock gate for ref ram
///
/// - **MC_RAM_CLK_EN** `[7]` (RW)
///
///   Configures whether or not to open the clock gate for mc ram.\\0: Open the clock gate only when application writes or reads mc ram\\1: Force open the clock gate for mc ram
///
/// - **MVD_RAM_CLK_EN** `[6]` (RW)
///
///   Configures whether or not to open the clock gate for mvd ram.\\0: Open the clock gate only when application writes or reads mvd ram\\1: Force open the clock gate for mvd ram
///
/// - **PRE_RAM_CLK_EN** `[5]` (RW)
///
///   Configures whether or not to open the clock gate for pre ram.\\0: Open the clock gate only when application writes or reads pre ram\\1: Force open the clock gate for pre ram
///
/// - **QUANT_RAM_CLK_EN1** `[4]` (RW)
///
///   Configures whether or not to open the clock gate for quant ram1.\\0: Open the clock gate only when application writes or reads quant ram1\\1: Force open the clock gate for quant ram1
///
/// - **QUANT_RAM_CLK_EN2** `[3]` (RW)
///
///   Configures whether or not to open the clock gate for quant ram2.\\0: Open the clock gate only when application writes or reads quant ram2\\1: Force open the clock gate for quant ram2
///
/// - **REC_RAM_CLK_EN1** `[2]` (RW)
///
///   Configures whether or not to open the clock gate for rec ram1.\\0: Open the clock gate only when application writes or reads rec ram1\\1: Force open the clock gate for rec ram1
///
/// - **REC_RAM_CLK_EN2** `[1]` (RW)
///
///   Configures whether or not to open the clock gate for rec ram2.\\0: Open the clock gate only when application writes or reads rec ram2\\1: Force open the clock gate for rec ram2
///
/// - **CLK_EN** `[0]` (RW)
///
///   Configures whether or not to open register clock gate.\\0: Open the clock gate only when application writes registers\\1: Force open the clock gate for register
#[repr(transparent)]
pub struct Conf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Conf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Conf {
    // ======== Field Constants ========

    /// **MV_MERGE_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for mv merge.\\0: Open the clock gate only when mv merge work\\1: Force open the clock gate for mv merge
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const MV_MERGE_CLK_EN_OFFSET: u32 = 25;
    pub const MV_MERGE_CLK_EN_MASK: u32 = 0x02000000;
    pub const MV_MERGE_CLK_EN: u32 = 1 << 25;

    /// **BS_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for bs buffer.\\0: Open the clock gate only when bs buffer work\\1: Force open the clock gate for bs buffer
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const BS_CLK_EN_OFFSET: u32 = 24;
    pub const BS_CLK_EN_MASK: u32 = 0x01000000;
    pub const BS_CLK_EN: u32 = 1 << 24;

    /// **DECI_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for decimate.\\0: Open the clock gate only when decimate work\\1: Force open the clock gate for decimate
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const DECI_CLK_EN_OFFSET: u32 = 23;
    pub const DECI_CLK_EN_MASK: u32 = 0x00800000;
    pub const DECI_CLK_EN: u32 = 1 << 23;

    /// **INTRA_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for intra.\\0: Open the clock gate only when intra work\\1: Force open the clock gate for intra
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const INTRA_CLK_EN_OFFSET: u32 = 22;
    pub const INTRA_CLK_EN_MASK: u32 = 0x00400000;
    pub const INTRA_CLK_EN: u32 = 1 << 22;

    /// **CLAVLC_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for cavlc.\\0: Open the clock gate only when cavlc work\\1: Force open the clock gate for cavlc
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CLAVLC_CLK_EN_OFFSET: u32 = 21;
    pub const CLAVLC_CLK_EN_MASK: u32 = 0x00200000;
    pub const CLAVLC_CLK_EN: u32 = 1 << 21;

    /// **DB_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for deblocking filter.\\0: Open the clock gate only when deblocking filter work\\1: Force open the clock gate for deblocking filter
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const DB_CLK_EN_OFFSET: u32 = 20;
    pub const DB_CLK_EN_MASK: u32 = 0x00100000;
    pub const DB_CLK_EN: u32 = 1 << 20;

    /// **INTERPOLATOR_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for interpolator.\\0: Open the clock gate only when interpolator work\\1: Force open the clock gate for interpolator
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const INTERPOLATOR_CLK_EN_OFFSET: u32 = 19;
    pub const INTERPOLATOR_CLK_EN_MASK: u32 = 0x00080000;
    pub const INTERPOLATOR_CLK_EN: u32 = 1 << 19;

    /// **MC_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for mc.\\0: Open the clock gate only when mc work\\1: Force open the clock gate for mc
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const MC_CLK_EN_OFFSET: u32 = 18;
    pub const MC_CLK_EN_MASK: u32 = 0x00040000;
    pub const MC_CLK_EN: u32 = 1 << 18;

    /// **FME_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for fme.\\0: Open the clock gate only when fme work\\1: Force open the clock gate for fme
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const FME_CLK_EN_OFFSET: u32 = 17;
    pub const FME_CLK_EN_MASK: u32 = 0x00020000;
    pub const FME_CLK_EN: u32 = 1 << 17;

    /// **IME_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for ime.\\0: Open the clock gate only when ime work\\1: Force open the clock gate for ime
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const IME_CLK_EN_OFFSET: u32 = 16;
    pub const IME_CLK_EN_MASK: u32 = 0x00010000;
    pub const IME_CLK_EN: u32 = 1 << 16;

    /// **CAVLC_RAM_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for cavlc ram.\\0: Open the clock gate only when application writes or reads cavlc ram\\1: Force open the clock gate for cavlc ram
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CAVLC_RAM_CLK_EN_OFFSET: u32 = 15;
    pub const CAVLC_RAM_CLK_EN_MASK: u32 = 0x00008000;
    pub const CAVLC_RAM_CLK_EN: u32 = 1 << 15;

    /// **CUR_MB_RAM_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for cur_mb ram.\\0: Open the clock gate only when application writes or reads cur_mb ram\\1: Force open the clock gate for cur_mb ram
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CUR_MB_RAM_CLK_EN_OFFSET: u32 = 14;
    pub const CUR_MB_RAM_CLK_EN_MASK: u32 = 0x00004000;
    pub const CUR_MB_RAM_CLK_EN: u32 = 1 << 14;

    /// **DB_RAM_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for db ram.\\0: Open the clock gate only when application writes or reads db ram\\1: Force open the clock gate for db ram
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const DB_RAM_CLK_EN_OFFSET: u32 = 13;
    pub const DB_RAM_CLK_EN_MASK: u32 = 0x00002000;
    pub const DB_RAM_CLK_EN: u32 = 1 << 13;

    /// **FETCH_RAM_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for fetch ram.\\0: Open the clock gate only when application writes or reads fetch ram\\1: Force open the clock gate for fetch ram
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const FETCH_RAM_CLK_EN_OFFSET: u32 = 12;
    pub const FETCH_RAM_CLK_EN_MASK: u32 = 0x00001000;
    pub const FETCH_RAM_CLK_EN: u32 = 1 << 12;

    /// **FME_RAM_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for fme ram.\\0: Open the clock gate only when application writes or readsfme ram\\1: Force open the clock gate for fme ram
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const FME_RAM_CLK_EN_OFFSET: u32 = 11;
    pub const FME_RAM_CLK_EN_MASK: u32 = 0x00000800;
    pub const FME_RAM_CLK_EN: u32 = 1 << 11;

    /// **IME_RAM_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for ime ram.\\0: Open the clock gate only when application writes or reads ime ram\\1: Force open the clock gate for ime ram
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const IME_RAM_CLK_EN_OFFSET: u32 = 10;
    pub const IME_RAM_CLK_EN_MASK: u32 = 0x00000400;
    pub const IME_RAM_CLK_EN: u32 = 1 << 10;

    /// **I4X4_REF_RAM_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for i4x4_mode ram.\\0: Open the clock gate only when application writes or reads i4x4_mode ram\\1: Force open the clock gate for i4x4_mode ram
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const I4X4_REF_RAM_CLK_EN_OFFSET: u32 = 9;
    pub const I4X4_REF_RAM_CLK_EN_MASK: u32 = 0x00000200;
    pub const I4X4_REF_RAM_CLK_EN: u32 = 1 << 9;

    /// **REF_RAM_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for ref ram.\\0: Open the clock gate only when application writes or reads ref ram\\1: Force open the clock gate for ref ram
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REF_RAM_CLK_EN_OFFSET: u32 = 8;
    pub const REF_RAM_CLK_EN_MASK: u32 = 0x00000100;
    pub const REF_RAM_CLK_EN: u32 = 1 << 8;

    /// **MC_RAM_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for mc ram.\\0: Open the clock gate only when application writes or reads mc ram\\1: Force open the clock gate for mc ram
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const MC_RAM_CLK_EN_OFFSET: u32 = 7;
    pub const MC_RAM_CLK_EN_MASK: u32 = 0x00000080;
    pub const MC_RAM_CLK_EN: u32 = 1 << 7;

    /// **MVD_RAM_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for mvd ram.\\0: Open the clock gate only when application writes or reads mvd ram\\1: Force open the clock gate for mvd ram
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const MVD_RAM_CLK_EN_OFFSET: u32 = 6;
    pub const MVD_RAM_CLK_EN_MASK: u32 = 0x00000040;
    pub const MVD_RAM_CLK_EN: u32 = 1 << 6;

    /// **PRE_RAM_CLK_EN**
    ///
    /// Configures whether or not to open the clock gate for pre ram.\\0: Open the clock gate only when application writes or reads pre ram\\1: Force open the clock gate for pre ram
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const PRE_RAM_CLK_EN_OFFSET: u32 = 5;
    pub const PRE_RAM_CLK_EN_MASK: u32 = 0x00000020;
    pub const PRE_RAM_CLK_EN: u32 = 1 << 5;

    /// **QUANT_RAM_CLK_EN1**
    ///
    /// Configures whether or not to open the clock gate for quant ram1.\\0: Open the clock gate only when application writes or reads quant ram1\\1: Force open the clock gate for quant ram1
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const QUANT_RAM_CLK_EN1_OFFSET: u32 = 4;
    pub const QUANT_RAM_CLK_EN1_MASK: u32 = 0x00000010;
    pub const QUANT_RAM_CLK_EN1: u32 = 1 << 4;

    /// **QUANT_RAM_CLK_EN2**
    ///
    /// Configures whether or not to open the clock gate for quant ram2.\\0: Open the clock gate only when application writes or reads quant ram2\\1: Force open the clock gate for quant ram2
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const QUANT_RAM_CLK_EN2_OFFSET: u32 = 3;
    pub const QUANT_RAM_CLK_EN2_MASK: u32 = 0x00000008;
    pub const QUANT_RAM_CLK_EN2: u32 = 1 << 3;

    /// **REC_RAM_CLK_EN1**
    ///
    /// Configures whether or not to open the clock gate for rec ram1.\\0: Open the clock gate only when application writes or reads rec ram1\\1: Force open the clock gate for rec ram1
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REC_RAM_CLK_EN1_OFFSET: u32 = 2;
    pub const REC_RAM_CLK_EN1_MASK: u32 = 0x00000004;
    pub const REC_RAM_CLK_EN1: u32 = 1 << 2;

    /// **REC_RAM_CLK_EN2**
    ///
    /// Configures whether or not to open the clock gate for rec ram2.\\0: Open the clock gate only when application writes or reads rec ram2\\1: Force open the clock gate for rec ram2
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REC_RAM_CLK_EN2_OFFSET: u32 = 1;
    pub const REC_RAM_CLK_EN2_MASK: u32 = 0x00000002;
    pub const REC_RAM_CLK_EN2: u32 = 1 << 1;

    /// **CLK_EN**
    ///
    /// Configures whether or not to open register clock gate.\\0: Open the clock gate only when application writes registers\\1: Force open the clock gate for register
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_EN_OFFSET: u32 = 0;
    pub const CLK_EN_MASK: u32 = 0x00000001;
    pub const CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MV_MERGE_CLK_EN** `[25]` (RW)
    ///
    /// Configures whether or not to open the clock gate for mv merge.\\0: Open the clock gate only when mv merge work\\1: Force open the clock gate for mv merge
    #[inline(always)]
    pub fn mv_merge_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::MV_MERGE_CLK_EN_MASK) != 0
    }

    /// Set MV_MERGE_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mv_merge_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::MV_MERGE_CLK_EN_MASK);
    }

    /// Clear MV_MERGE_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mv_merge_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::MV_MERGE_CLK_EN_MASK);
    }

    /// **BS_CLK_EN** `[24]` (RW)
    ///
    /// Configures whether or not to open the clock gate for bs buffer.\\0: Open the clock gate only when bs buffer work\\1: Force open the clock gate for bs buffer
    #[inline(always)]
    pub fn bs_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::BS_CLK_EN_MASK) != 0
    }

    /// Set BS_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bs_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::BS_CLK_EN_MASK);
    }

    /// Clear BS_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bs_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::BS_CLK_EN_MASK);
    }

    /// **DECI_CLK_EN** `[23]` (RW)
    ///
    /// Configures whether or not to open the clock gate for decimate.\\0: Open the clock gate only when decimate work\\1: Force open the clock gate for decimate
    #[inline(always)]
    pub fn deci_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::DECI_CLK_EN_MASK) != 0
    }

    /// Set DECI_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_deci_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::DECI_CLK_EN_MASK);
    }

    /// Clear DECI_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_deci_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::DECI_CLK_EN_MASK);
    }

    /// **INTRA_CLK_EN** `[22]` (RW)
    ///
    /// Configures whether or not to open the clock gate for intra.\\0: Open the clock gate only when intra work\\1: Force open the clock gate for intra
    #[inline(always)]
    pub fn intra_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::INTRA_CLK_EN_MASK) != 0
    }

    /// Set INTRA_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_intra_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::INTRA_CLK_EN_MASK);
    }

    /// Clear INTRA_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_intra_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::INTRA_CLK_EN_MASK);
    }

    /// **CLAVLC_CLK_EN** `[21]` (RW)
    ///
    /// Configures whether or not to open the clock gate for cavlc.\\0: Open the clock gate only when cavlc work\\1: Force open the clock gate for cavlc
    #[inline(always)]
    pub fn clavlc_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::CLAVLC_CLK_EN_MASK) != 0
    }

    /// Set CLAVLC_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_clavlc_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::CLAVLC_CLK_EN_MASK);
    }

    /// Clear CLAVLC_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_clavlc_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::CLAVLC_CLK_EN_MASK);
    }

    /// **DB_CLK_EN** `[20]` (RW)
    ///
    /// Configures whether or not to open the clock gate for deblocking filter.\\0: Open the clock gate only when deblocking filter work\\1: Force open the clock gate for deblocking filter
    #[inline(always)]
    pub fn db_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::DB_CLK_EN_MASK) != 0
    }

    /// Set DB_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_db_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::DB_CLK_EN_MASK);
    }

    /// Clear DB_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_db_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::DB_CLK_EN_MASK);
    }

    /// **INTERPOLATOR_CLK_EN** `[19]` (RW)
    ///
    /// Configures whether or not to open the clock gate for interpolator.\\0: Open the clock gate only when interpolator work\\1: Force open the clock gate for interpolator
    #[inline(always)]
    pub fn interpolator_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::INTERPOLATOR_CLK_EN_MASK) != 0
    }

    /// Set INTERPOLATOR_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_interpolator_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::INTERPOLATOR_CLK_EN_MASK);
    }

    /// Clear INTERPOLATOR_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_interpolator_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::INTERPOLATOR_CLK_EN_MASK);
    }

    /// **MC_CLK_EN** `[18]` (RW)
    ///
    /// Configures whether or not to open the clock gate for mc.\\0: Open the clock gate only when mc work\\1: Force open the clock gate for mc
    #[inline(always)]
    pub fn mc_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::MC_CLK_EN_MASK) != 0
    }

    /// Set MC_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mc_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::MC_CLK_EN_MASK);
    }

    /// Clear MC_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mc_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::MC_CLK_EN_MASK);
    }

    /// **FME_CLK_EN** `[17]` (RW)
    ///
    /// Configures whether or not to open the clock gate for fme.\\0: Open the clock gate only when fme work\\1: Force open the clock gate for fme
    #[inline(always)]
    pub fn fme_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::FME_CLK_EN_MASK) != 0
    }

    /// Set FME_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fme_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::FME_CLK_EN_MASK);
    }

    /// Clear FME_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fme_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::FME_CLK_EN_MASK);
    }

    /// **IME_CLK_EN** `[16]` (RW)
    ///
    /// Configures whether or not to open the clock gate for ime.\\0: Open the clock gate only when ime work\\1: Force open the clock gate for ime
    #[inline(always)]
    pub fn ime_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::IME_CLK_EN_MASK) != 0
    }

    /// Set IME_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ime_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::IME_CLK_EN_MASK);
    }

    /// Clear IME_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ime_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::IME_CLK_EN_MASK);
    }

    /// **CAVLC_RAM_CLK_EN** `[15]` (RW)
    ///
    /// Configures whether or not to open the clock gate for cavlc ram.\\0: Open the clock gate only when application writes or reads cavlc ram\\1: Force open the clock gate for cavlc ram
    #[inline(always)]
    pub fn cavlc_ram_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::CAVLC_RAM_CLK_EN_MASK) != 0
    }

    /// Set CAVLC_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cavlc_ram_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::CAVLC_RAM_CLK_EN_MASK);
    }

    /// Clear CAVLC_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cavlc_ram_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::CAVLC_RAM_CLK_EN_MASK);
    }

    /// **CUR_MB_RAM_CLK_EN** `[14]` (RW)
    ///
    /// Configures whether or not to open the clock gate for cur_mb ram.\\0: Open the clock gate only when application writes or reads cur_mb ram\\1: Force open the clock gate for cur_mb ram
    #[inline(always)]
    pub fn cur_mb_ram_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::CUR_MB_RAM_CLK_EN_MASK) != 0
    }

    /// Set CUR_MB_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cur_mb_ram_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::CUR_MB_RAM_CLK_EN_MASK);
    }

    /// Clear CUR_MB_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cur_mb_ram_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::CUR_MB_RAM_CLK_EN_MASK);
    }

    /// **DB_RAM_CLK_EN** `[13]` (RW)
    ///
    /// Configures whether or not to open the clock gate for db ram.\\0: Open the clock gate only when application writes or reads db ram\\1: Force open the clock gate for db ram
    #[inline(always)]
    pub fn db_ram_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::DB_RAM_CLK_EN_MASK) != 0
    }

    /// Set DB_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_db_ram_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::DB_RAM_CLK_EN_MASK);
    }

    /// Clear DB_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_db_ram_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::DB_RAM_CLK_EN_MASK);
    }

    /// **FETCH_RAM_CLK_EN** `[12]` (RW)
    ///
    /// Configures whether or not to open the clock gate for fetch ram.\\0: Open the clock gate only when application writes or reads fetch ram\\1: Force open the clock gate for fetch ram
    #[inline(always)]
    pub fn fetch_ram_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::FETCH_RAM_CLK_EN_MASK) != 0
    }

    /// Set FETCH_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fetch_ram_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::FETCH_RAM_CLK_EN_MASK);
    }

    /// Clear FETCH_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fetch_ram_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::FETCH_RAM_CLK_EN_MASK);
    }

    /// **FME_RAM_CLK_EN** `[11]` (RW)
    ///
    /// Configures whether or not to open the clock gate for fme ram.\\0: Open the clock gate only when application writes or readsfme ram\\1: Force open the clock gate for fme ram
    #[inline(always)]
    pub fn fme_ram_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::FME_RAM_CLK_EN_MASK) != 0
    }

    /// Set FME_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fme_ram_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::FME_RAM_CLK_EN_MASK);
    }

    /// Clear FME_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fme_ram_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::FME_RAM_CLK_EN_MASK);
    }

    /// **IME_RAM_CLK_EN** `[10]` (RW)
    ///
    /// Configures whether or not to open the clock gate for ime ram.\\0: Open the clock gate only when application writes or reads ime ram\\1: Force open the clock gate for ime ram
    #[inline(always)]
    pub fn ime_ram_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::IME_RAM_CLK_EN_MASK) != 0
    }

    /// Set IME_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ime_ram_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::IME_RAM_CLK_EN_MASK);
    }

    /// Clear IME_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ime_ram_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::IME_RAM_CLK_EN_MASK);
    }

    /// **I4X4_REF_RAM_CLK_EN** `[9]` (RW)
    ///
    /// Configures whether or not to open the clock gate for i4x4_mode ram.\\0: Open the clock gate only when application writes or reads i4x4_mode ram\\1: Force open the clock gate for i4x4_mode ram
    #[inline(always)]
    pub fn i4x4_ref_ram_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::I4X4_REF_RAM_CLK_EN_MASK) != 0
    }

    /// Set I4X4_REF_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_i4x4_ref_ram_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::I4X4_REF_RAM_CLK_EN_MASK);
    }

    /// Clear I4X4_REF_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_i4x4_ref_ram_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::I4X4_REF_RAM_CLK_EN_MASK);
    }

    /// **REF_RAM_CLK_EN** `[8]` (RW)
    ///
    /// Configures whether or not to open the clock gate for ref ram.\\0: Open the clock gate only when application writes or reads ref ram\\1: Force open the clock gate for ref ram
    #[inline(always)]
    pub fn ref_ram_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::REF_RAM_CLK_EN_MASK) != 0
    }

    /// Set REF_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ref_ram_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::REF_RAM_CLK_EN_MASK);
    }

    /// Clear REF_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ref_ram_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::REF_RAM_CLK_EN_MASK);
    }

    /// **MC_RAM_CLK_EN** `[7]` (RW)
    ///
    /// Configures whether or not to open the clock gate for mc ram.\\0: Open the clock gate only when application writes or reads mc ram\\1: Force open the clock gate for mc ram
    #[inline(always)]
    pub fn mc_ram_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::MC_RAM_CLK_EN_MASK) != 0
    }

    /// Set MC_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mc_ram_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::MC_RAM_CLK_EN_MASK);
    }

    /// Clear MC_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mc_ram_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::MC_RAM_CLK_EN_MASK);
    }

    /// **MVD_RAM_CLK_EN** `[6]` (RW)
    ///
    /// Configures whether or not to open the clock gate for mvd ram.\\0: Open the clock gate only when application writes or reads mvd ram\\1: Force open the clock gate for mvd ram
    #[inline(always)]
    pub fn mvd_ram_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::MVD_RAM_CLK_EN_MASK) != 0
    }

    /// Set MVD_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mvd_ram_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::MVD_RAM_CLK_EN_MASK);
    }

    /// Clear MVD_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mvd_ram_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::MVD_RAM_CLK_EN_MASK);
    }

    /// **PRE_RAM_CLK_EN** `[5]` (RW)
    ///
    /// Configures whether or not to open the clock gate for pre ram.\\0: Open the clock gate only when application writes or reads pre ram\\1: Force open the clock gate for pre ram
    #[inline(always)]
    pub fn pre_ram_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::PRE_RAM_CLK_EN_MASK) != 0
    }

    /// Set PRE_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pre_ram_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::PRE_RAM_CLK_EN_MASK);
    }

    /// Clear PRE_RAM_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pre_ram_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::PRE_RAM_CLK_EN_MASK);
    }

    /// **QUANT_RAM_CLK_EN1** `[4]` (RW)
    ///
    /// Configures whether or not to open the clock gate for quant ram1.\\0: Open the clock gate only when application writes or reads quant ram1\\1: Force open the clock gate for quant ram1
    #[inline(always)]
    pub fn quant_ram_clk_en1(&self) -> bool {
        (self.value.read_bits() & Self::QUANT_RAM_CLK_EN1_MASK) != 0
    }

    /// Set QUANT_RAM_CLK_EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_quant_ram_clk_en1(&self) {
        self.value.modify_bits(|r| r | Self::QUANT_RAM_CLK_EN1_MASK);
    }

    /// Clear QUANT_RAM_CLK_EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_quant_ram_clk_en1(&self) {
        self.value
            .modify_bits(|r| r & !Self::QUANT_RAM_CLK_EN1_MASK);
    }

    /// **QUANT_RAM_CLK_EN2** `[3]` (RW)
    ///
    /// Configures whether or not to open the clock gate for quant ram2.\\0: Open the clock gate only when application writes or reads quant ram2\\1: Force open the clock gate for quant ram2
    #[inline(always)]
    pub fn quant_ram_clk_en2(&self) -> bool {
        (self.value.read_bits() & Self::QUANT_RAM_CLK_EN2_MASK) != 0
    }

    /// Set QUANT_RAM_CLK_EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_quant_ram_clk_en2(&self) {
        self.value.modify_bits(|r| r | Self::QUANT_RAM_CLK_EN2_MASK);
    }

    /// Clear QUANT_RAM_CLK_EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_quant_ram_clk_en2(&self) {
        self.value
            .modify_bits(|r| r & !Self::QUANT_RAM_CLK_EN2_MASK);
    }

    /// **REC_RAM_CLK_EN1** `[2]` (RW)
    ///
    /// Configures whether or not to open the clock gate for rec ram1.\\0: Open the clock gate only when application writes or reads rec ram1\\1: Force open the clock gate for rec ram1
    #[inline(always)]
    pub fn rec_ram_clk_en1(&self) -> bool {
        (self.value.read_bits() & Self::REC_RAM_CLK_EN1_MASK) != 0
    }

    /// Set REC_RAM_CLK_EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rec_ram_clk_en1(&self) {
        self.value.modify_bits(|r| r | Self::REC_RAM_CLK_EN1_MASK);
    }

    /// Clear REC_RAM_CLK_EN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rec_ram_clk_en1(&self) {
        self.value.modify_bits(|r| r & !Self::REC_RAM_CLK_EN1_MASK);
    }

    /// **REC_RAM_CLK_EN2** `[1]` (RW)
    ///
    /// Configures whether or not to open the clock gate for rec ram2.\\0: Open the clock gate only when application writes or reads rec ram2\\1: Force open the clock gate for rec ram2
    #[inline(always)]
    pub fn rec_ram_clk_en2(&self) -> bool {
        (self.value.read_bits() & Self::REC_RAM_CLK_EN2_MASK) != 0
    }

    /// Set REC_RAM_CLK_EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rec_ram_clk_en2(&self) {
        self.value.modify_bits(|r| r | Self::REC_RAM_CLK_EN2_MASK);
    }

    /// Clear REC_RAM_CLK_EN2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rec_ram_clk_en2(&self) {
        self.value.modify_bits(|r| r & !Self::REC_RAM_CLK_EN2_MASK);
    }

    /// **CLK_EN** `[0]` (RW)
    ///
    /// Configures whether or not to open register clock gate.\\0: Open the clock gate only when application writes registers\\1: Force open the clock gate for register
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

/// CONF value type for batch read/modify/write operations
///
/// Use `conf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `conf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ConfVal {
    bits: u32,
}

impl ConfVal {
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

    /// Read **MV_MERGE_CLK_EN** `[25:25]`
    ///
    /// Configures whether or not to open the clock gate for mv merge.\\0: Open the clock gate only when mv merge work\\1: Force open the clock gate for mv merge
    #[inline(always)]
    pub const fn mv_merge_clk_en(&self) -> bool {
        (self.bits & Conf::MV_MERGE_CLK_EN_MASK) != 0
    }

    /// Read **BS_CLK_EN** `[24:24]`
    ///
    /// Configures whether or not to open the clock gate for bs buffer.\\0: Open the clock gate only when bs buffer work\\1: Force open the clock gate for bs buffer
    #[inline(always)]
    pub const fn bs_clk_en(&self) -> bool {
        (self.bits & Conf::BS_CLK_EN_MASK) != 0
    }

    /// Read **DECI_CLK_EN** `[23:23]`
    ///
    /// Configures whether or not to open the clock gate for decimate.\\0: Open the clock gate only when decimate work\\1: Force open the clock gate for decimate
    #[inline(always)]
    pub const fn deci_clk_en(&self) -> bool {
        (self.bits & Conf::DECI_CLK_EN_MASK) != 0
    }

    /// Read **INTRA_CLK_EN** `[22:22]`
    ///
    /// Configures whether or not to open the clock gate for intra.\\0: Open the clock gate only when intra work\\1: Force open the clock gate for intra
    #[inline(always)]
    pub const fn intra_clk_en(&self) -> bool {
        (self.bits & Conf::INTRA_CLK_EN_MASK) != 0
    }

    /// Read **CLAVLC_CLK_EN** `[21:21]`
    ///
    /// Configures whether or not to open the clock gate for cavlc.\\0: Open the clock gate only when cavlc work\\1: Force open the clock gate for cavlc
    #[inline(always)]
    pub const fn clavlc_clk_en(&self) -> bool {
        (self.bits & Conf::CLAVLC_CLK_EN_MASK) != 0
    }

    /// Read **DB_CLK_EN** `[20:20]`
    ///
    /// Configures whether or not to open the clock gate for deblocking filter.\\0: Open the clock gate only when deblocking filter work\\1: Force open the clock gate for deblocking filter
    #[inline(always)]
    pub const fn db_clk_en(&self) -> bool {
        (self.bits & Conf::DB_CLK_EN_MASK) != 0
    }

    /// Read **INTERPOLATOR_CLK_EN** `[19:19]`
    ///
    /// Configures whether or not to open the clock gate for interpolator.\\0: Open the clock gate only when interpolator work\\1: Force open the clock gate for interpolator
    #[inline(always)]
    pub const fn interpolator_clk_en(&self) -> bool {
        (self.bits & Conf::INTERPOLATOR_CLK_EN_MASK) != 0
    }

    /// Read **MC_CLK_EN** `[18:18]`
    ///
    /// Configures whether or not to open the clock gate for mc.\\0: Open the clock gate only when mc work\\1: Force open the clock gate for mc
    #[inline(always)]
    pub const fn mc_clk_en(&self) -> bool {
        (self.bits & Conf::MC_CLK_EN_MASK) != 0
    }

    /// Read **FME_CLK_EN** `[17:17]`
    ///
    /// Configures whether or not to open the clock gate for fme.\\0: Open the clock gate only when fme work\\1: Force open the clock gate for fme
    #[inline(always)]
    pub const fn fme_clk_en(&self) -> bool {
        (self.bits & Conf::FME_CLK_EN_MASK) != 0
    }

    /// Read **IME_CLK_EN** `[16:16]`
    ///
    /// Configures whether or not to open the clock gate for ime.\\0: Open the clock gate only when ime work\\1: Force open the clock gate for ime
    #[inline(always)]
    pub const fn ime_clk_en(&self) -> bool {
        (self.bits & Conf::IME_CLK_EN_MASK) != 0
    }

    /// Read **CAVLC_RAM_CLK_EN** `[15:15]`
    ///
    /// Configures whether or not to open the clock gate for cavlc ram.\\0: Open the clock gate only when application writes or reads cavlc ram\\1: Force open the clock gate for cavlc ram
    #[inline(always)]
    pub const fn cavlc_ram_clk_en(&self) -> bool {
        (self.bits & Conf::CAVLC_RAM_CLK_EN_MASK) != 0
    }

    /// Read **CUR_MB_RAM_CLK_EN** `[14:14]`
    ///
    /// Configures whether or not to open the clock gate for cur_mb ram.\\0: Open the clock gate only when application writes or reads cur_mb ram\\1: Force open the clock gate for cur_mb ram
    #[inline(always)]
    pub const fn cur_mb_ram_clk_en(&self) -> bool {
        (self.bits & Conf::CUR_MB_RAM_CLK_EN_MASK) != 0
    }

    /// Read **DB_RAM_CLK_EN** `[13:13]`
    ///
    /// Configures whether or not to open the clock gate for db ram.\\0: Open the clock gate only when application writes or reads db ram\\1: Force open the clock gate for db ram
    #[inline(always)]
    pub const fn db_ram_clk_en(&self) -> bool {
        (self.bits & Conf::DB_RAM_CLK_EN_MASK) != 0
    }

    /// Read **FETCH_RAM_CLK_EN** `[12:12]`
    ///
    /// Configures whether or not to open the clock gate for fetch ram.\\0: Open the clock gate only when application writes or reads fetch ram\\1: Force open the clock gate for fetch ram
    #[inline(always)]
    pub const fn fetch_ram_clk_en(&self) -> bool {
        (self.bits & Conf::FETCH_RAM_CLK_EN_MASK) != 0
    }

    /// Read **FME_RAM_CLK_EN** `[11:11]`
    ///
    /// Configures whether or not to open the clock gate for fme ram.\\0: Open the clock gate only when application writes or readsfme ram\\1: Force open the clock gate for fme ram
    #[inline(always)]
    pub const fn fme_ram_clk_en(&self) -> bool {
        (self.bits & Conf::FME_RAM_CLK_EN_MASK) != 0
    }

    /// Read **IME_RAM_CLK_EN** `[10:10]`
    ///
    /// Configures whether or not to open the clock gate for ime ram.\\0: Open the clock gate only when application writes or reads ime ram\\1: Force open the clock gate for ime ram
    #[inline(always)]
    pub const fn ime_ram_clk_en(&self) -> bool {
        (self.bits & Conf::IME_RAM_CLK_EN_MASK) != 0
    }

    /// Read **I4X4_REF_RAM_CLK_EN** `[9:9]`
    ///
    /// Configures whether or not to open the clock gate for i4x4_mode ram.\\0: Open the clock gate only when application writes or reads i4x4_mode ram\\1: Force open the clock gate for i4x4_mode ram
    #[inline(always)]
    pub const fn i4x4_ref_ram_clk_en(&self) -> bool {
        (self.bits & Conf::I4X4_REF_RAM_CLK_EN_MASK) != 0
    }

    /// Read **REF_RAM_CLK_EN** `[8:8]`
    ///
    /// Configures whether or not to open the clock gate for ref ram.\\0: Open the clock gate only when application writes or reads ref ram\\1: Force open the clock gate for ref ram
    #[inline(always)]
    pub const fn ref_ram_clk_en(&self) -> bool {
        (self.bits & Conf::REF_RAM_CLK_EN_MASK) != 0
    }

    /// Read **MC_RAM_CLK_EN** `[7:7]`
    ///
    /// Configures whether or not to open the clock gate for mc ram.\\0: Open the clock gate only when application writes or reads mc ram\\1: Force open the clock gate for mc ram
    #[inline(always)]
    pub const fn mc_ram_clk_en(&self) -> bool {
        (self.bits & Conf::MC_RAM_CLK_EN_MASK) != 0
    }

    /// Read **MVD_RAM_CLK_EN** `[6:6]`
    ///
    /// Configures whether or not to open the clock gate for mvd ram.\\0: Open the clock gate only when application writes or reads mvd ram\\1: Force open the clock gate for mvd ram
    #[inline(always)]
    pub const fn mvd_ram_clk_en(&self) -> bool {
        (self.bits & Conf::MVD_RAM_CLK_EN_MASK) != 0
    }

    /// Read **PRE_RAM_CLK_EN** `[5:5]`
    ///
    /// Configures whether or not to open the clock gate for pre ram.\\0: Open the clock gate only when application writes or reads pre ram\\1: Force open the clock gate for pre ram
    #[inline(always)]
    pub const fn pre_ram_clk_en(&self) -> bool {
        (self.bits & Conf::PRE_RAM_CLK_EN_MASK) != 0
    }

    /// Read **QUANT_RAM_CLK_EN1** `[4:4]`
    ///
    /// Configures whether or not to open the clock gate for quant ram1.\\0: Open the clock gate only when application writes or reads quant ram1\\1: Force open the clock gate for quant ram1
    #[inline(always)]
    pub const fn quant_ram_clk_en1(&self) -> bool {
        (self.bits & Conf::QUANT_RAM_CLK_EN1_MASK) != 0
    }

    /// Read **QUANT_RAM_CLK_EN2** `[3:3]`
    ///
    /// Configures whether or not to open the clock gate for quant ram2.\\0: Open the clock gate only when application writes or reads quant ram2\\1: Force open the clock gate for quant ram2
    #[inline(always)]
    pub const fn quant_ram_clk_en2(&self) -> bool {
        (self.bits & Conf::QUANT_RAM_CLK_EN2_MASK) != 0
    }

    /// Read **REC_RAM_CLK_EN1** `[2:2]`
    ///
    /// Configures whether or not to open the clock gate for rec ram1.\\0: Open the clock gate only when application writes or reads rec ram1\\1: Force open the clock gate for rec ram1
    #[inline(always)]
    pub const fn rec_ram_clk_en1(&self) -> bool {
        (self.bits & Conf::REC_RAM_CLK_EN1_MASK) != 0
    }

    /// Read **REC_RAM_CLK_EN2** `[1:1]`
    ///
    /// Configures whether or not to open the clock gate for rec ram2.\\0: Open the clock gate only when application writes or reads rec ram2\\1: Force open the clock gate for rec ram2
    #[inline(always)]
    pub const fn rec_ram_clk_en2(&self) -> bool {
        (self.bits & Conf::REC_RAM_CLK_EN2_MASK) != 0
    }

    /// Read **CLK_EN** `[0:0]`
    ///
    /// Configures whether or not to open register clock gate.\\0: Open the clock gate only when application writes registers\\1: Force open the clock gate for register
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & Conf::CLK_EN_MASK) != 0
    }

    /// Set **MV_MERGE_CLK_EN** `[25:25]`
    ///
    /// Configures whether or not to open the clock gate for mv merge.\\0: Open the clock gate only when mv merge work\\1: Force open the clock gate for mv merge
    #[inline(always)]
    pub const fn with_mv_merge_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::MV_MERGE_CLK_EN_MASK
            } else {
                self.bits & !Conf::MV_MERGE_CLK_EN_MASK
            },
        }
    }

    /// Set **BS_CLK_EN** `[24:24]`
    ///
    /// Configures whether or not to open the clock gate for bs buffer.\\0: Open the clock gate only when bs buffer work\\1: Force open the clock gate for bs buffer
    #[inline(always)]
    pub const fn with_bs_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::BS_CLK_EN_MASK
            } else {
                self.bits & !Conf::BS_CLK_EN_MASK
            },
        }
    }

    /// Set **DECI_CLK_EN** `[23:23]`
    ///
    /// Configures whether or not to open the clock gate for decimate.\\0: Open the clock gate only when decimate work\\1: Force open the clock gate for decimate
    #[inline(always)]
    pub const fn with_deci_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::DECI_CLK_EN_MASK
            } else {
                self.bits & !Conf::DECI_CLK_EN_MASK
            },
        }
    }

    /// Set **INTRA_CLK_EN** `[22:22]`
    ///
    /// Configures whether or not to open the clock gate for intra.\\0: Open the clock gate only when intra work\\1: Force open the clock gate for intra
    #[inline(always)]
    pub const fn with_intra_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::INTRA_CLK_EN_MASK
            } else {
                self.bits & !Conf::INTRA_CLK_EN_MASK
            },
        }
    }

    /// Set **CLAVLC_CLK_EN** `[21:21]`
    ///
    /// Configures whether or not to open the clock gate for cavlc.\\0: Open the clock gate only when cavlc work\\1: Force open the clock gate for cavlc
    #[inline(always)]
    pub const fn with_clavlc_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::CLAVLC_CLK_EN_MASK
            } else {
                self.bits & !Conf::CLAVLC_CLK_EN_MASK
            },
        }
    }

    /// Set **DB_CLK_EN** `[20:20]`
    ///
    /// Configures whether or not to open the clock gate for deblocking filter.\\0: Open the clock gate only when deblocking filter work\\1: Force open the clock gate for deblocking filter
    #[inline(always)]
    pub const fn with_db_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::DB_CLK_EN_MASK
            } else {
                self.bits & !Conf::DB_CLK_EN_MASK
            },
        }
    }

    /// Set **INTERPOLATOR_CLK_EN** `[19:19]`
    ///
    /// Configures whether or not to open the clock gate for interpolator.\\0: Open the clock gate only when interpolator work\\1: Force open the clock gate for interpolator
    #[inline(always)]
    pub const fn with_interpolator_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::INTERPOLATOR_CLK_EN_MASK
            } else {
                self.bits & !Conf::INTERPOLATOR_CLK_EN_MASK
            },
        }
    }

    /// Set **MC_CLK_EN** `[18:18]`
    ///
    /// Configures whether or not to open the clock gate for mc.\\0: Open the clock gate only when mc work\\1: Force open the clock gate for mc
    #[inline(always)]
    pub const fn with_mc_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::MC_CLK_EN_MASK
            } else {
                self.bits & !Conf::MC_CLK_EN_MASK
            },
        }
    }

    /// Set **FME_CLK_EN** `[17:17]`
    ///
    /// Configures whether or not to open the clock gate for fme.\\0: Open the clock gate only when fme work\\1: Force open the clock gate for fme
    #[inline(always)]
    pub const fn with_fme_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::FME_CLK_EN_MASK
            } else {
                self.bits & !Conf::FME_CLK_EN_MASK
            },
        }
    }

    /// Set **IME_CLK_EN** `[16:16]`
    ///
    /// Configures whether or not to open the clock gate for ime.\\0: Open the clock gate only when ime work\\1: Force open the clock gate for ime
    #[inline(always)]
    pub const fn with_ime_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::IME_CLK_EN_MASK
            } else {
                self.bits & !Conf::IME_CLK_EN_MASK
            },
        }
    }

    /// Set **CAVLC_RAM_CLK_EN** `[15:15]`
    ///
    /// Configures whether or not to open the clock gate for cavlc ram.\\0: Open the clock gate only when application writes or reads cavlc ram\\1: Force open the clock gate for cavlc ram
    #[inline(always)]
    pub const fn with_cavlc_ram_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::CAVLC_RAM_CLK_EN_MASK
            } else {
                self.bits & !Conf::CAVLC_RAM_CLK_EN_MASK
            },
        }
    }

    /// Set **CUR_MB_RAM_CLK_EN** `[14:14]`
    ///
    /// Configures whether or not to open the clock gate for cur_mb ram.\\0: Open the clock gate only when application writes or reads cur_mb ram\\1: Force open the clock gate for cur_mb ram
    #[inline(always)]
    pub const fn with_cur_mb_ram_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::CUR_MB_RAM_CLK_EN_MASK
            } else {
                self.bits & !Conf::CUR_MB_RAM_CLK_EN_MASK
            },
        }
    }

    /// Set **DB_RAM_CLK_EN** `[13:13]`
    ///
    /// Configures whether or not to open the clock gate for db ram.\\0: Open the clock gate only when application writes or reads db ram\\1: Force open the clock gate for db ram
    #[inline(always)]
    pub const fn with_db_ram_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::DB_RAM_CLK_EN_MASK
            } else {
                self.bits & !Conf::DB_RAM_CLK_EN_MASK
            },
        }
    }

    /// Set **FETCH_RAM_CLK_EN** `[12:12]`
    ///
    /// Configures whether or not to open the clock gate for fetch ram.\\0: Open the clock gate only when application writes or reads fetch ram\\1: Force open the clock gate for fetch ram
    #[inline(always)]
    pub const fn with_fetch_ram_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::FETCH_RAM_CLK_EN_MASK
            } else {
                self.bits & !Conf::FETCH_RAM_CLK_EN_MASK
            },
        }
    }

    /// Set **FME_RAM_CLK_EN** `[11:11]`
    ///
    /// Configures whether or not to open the clock gate for fme ram.\\0: Open the clock gate only when application writes or readsfme ram\\1: Force open the clock gate for fme ram
    #[inline(always)]
    pub const fn with_fme_ram_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::FME_RAM_CLK_EN_MASK
            } else {
                self.bits & !Conf::FME_RAM_CLK_EN_MASK
            },
        }
    }

    /// Set **IME_RAM_CLK_EN** `[10:10]`
    ///
    /// Configures whether or not to open the clock gate for ime ram.\\0: Open the clock gate only when application writes or reads ime ram\\1: Force open the clock gate for ime ram
    #[inline(always)]
    pub const fn with_ime_ram_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::IME_RAM_CLK_EN_MASK
            } else {
                self.bits & !Conf::IME_RAM_CLK_EN_MASK
            },
        }
    }

    /// Set **I4X4_REF_RAM_CLK_EN** `[9:9]`
    ///
    /// Configures whether or not to open the clock gate for i4x4_mode ram.\\0: Open the clock gate only when application writes or reads i4x4_mode ram\\1: Force open the clock gate for i4x4_mode ram
    #[inline(always)]
    pub const fn with_i4x4_ref_ram_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::I4X4_REF_RAM_CLK_EN_MASK
            } else {
                self.bits & !Conf::I4X4_REF_RAM_CLK_EN_MASK
            },
        }
    }

    /// Set **REF_RAM_CLK_EN** `[8:8]`
    ///
    /// Configures whether or not to open the clock gate for ref ram.\\0: Open the clock gate only when application writes or reads ref ram\\1: Force open the clock gate for ref ram
    #[inline(always)]
    pub const fn with_ref_ram_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::REF_RAM_CLK_EN_MASK
            } else {
                self.bits & !Conf::REF_RAM_CLK_EN_MASK
            },
        }
    }

    /// Set **MC_RAM_CLK_EN** `[7:7]`
    ///
    /// Configures whether or not to open the clock gate for mc ram.\\0: Open the clock gate only when application writes or reads mc ram\\1: Force open the clock gate for mc ram
    #[inline(always)]
    pub const fn with_mc_ram_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::MC_RAM_CLK_EN_MASK
            } else {
                self.bits & !Conf::MC_RAM_CLK_EN_MASK
            },
        }
    }

    /// Set **MVD_RAM_CLK_EN** `[6:6]`
    ///
    /// Configures whether or not to open the clock gate for mvd ram.\\0: Open the clock gate only when application writes or reads mvd ram\\1: Force open the clock gate for mvd ram
    #[inline(always)]
    pub const fn with_mvd_ram_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::MVD_RAM_CLK_EN_MASK
            } else {
                self.bits & !Conf::MVD_RAM_CLK_EN_MASK
            },
        }
    }

    /// Set **PRE_RAM_CLK_EN** `[5:5]`
    ///
    /// Configures whether or not to open the clock gate for pre ram.\\0: Open the clock gate only when application writes or reads pre ram\\1: Force open the clock gate for pre ram
    #[inline(always)]
    pub const fn with_pre_ram_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::PRE_RAM_CLK_EN_MASK
            } else {
                self.bits & !Conf::PRE_RAM_CLK_EN_MASK
            },
        }
    }

    /// Set **QUANT_RAM_CLK_EN1** `[4:4]`
    ///
    /// Configures whether or not to open the clock gate for quant ram1.\\0: Open the clock gate only when application writes or reads quant ram1\\1: Force open the clock gate for quant ram1
    #[inline(always)]
    pub const fn with_quant_ram_clk_en1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::QUANT_RAM_CLK_EN1_MASK
            } else {
                self.bits & !Conf::QUANT_RAM_CLK_EN1_MASK
            },
        }
    }

    /// Set **QUANT_RAM_CLK_EN2** `[3:3]`
    ///
    /// Configures whether or not to open the clock gate for quant ram2.\\0: Open the clock gate only when application writes or reads quant ram2\\1: Force open the clock gate for quant ram2
    #[inline(always)]
    pub const fn with_quant_ram_clk_en2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::QUANT_RAM_CLK_EN2_MASK
            } else {
                self.bits & !Conf::QUANT_RAM_CLK_EN2_MASK
            },
        }
    }

    /// Set **REC_RAM_CLK_EN1** `[2:2]`
    ///
    /// Configures whether or not to open the clock gate for rec ram1.\\0: Open the clock gate only when application writes or reads rec ram1\\1: Force open the clock gate for rec ram1
    #[inline(always)]
    pub const fn with_rec_ram_clk_en1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::REC_RAM_CLK_EN1_MASK
            } else {
                self.bits & !Conf::REC_RAM_CLK_EN1_MASK
            },
        }
    }

    /// Set **REC_RAM_CLK_EN2** `[1:1]`
    ///
    /// Configures whether or not to open the clock gate for rec ram2.\\0: Open the clock gate only when application writes or reads rec ram2\\1: Force open the clock gate for rec ram2
    #[inline(always)]
    pub const fn with_rec_ram_clk_en2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::REC_RAM_CLK_EN2_MASK
            } else {
                self.bits & !Conf::REC_RAM_CLK_EN2_MASK
            },
        }
    }

    /// Set **CLK_EN** `[0:0]`
    ///
    /// Configures whether or not to open register clock gate.\\0: Open the clock gate only when application writes registers\\1: Force open the clock gate for register
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::CLK_EN_MASK
            } else {
                self.bits & !Conf::CLK_EN_MASK
            },
        }
    }
}

impl Default for ConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Conf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ConfVal {
        ConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ConfVal) -> ConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MV_MERGE_CONFIG Register
///
/// Mv merge configuration register.
///
/// Address offset: 0x00d4
///
/// ## Fields
///
/// - **MB_VALID_NUM** `[17:5]` (RO)
///
///   Represents the valid mb number of mv merge output.
///
/// - **B_MV_MERGE_EN** `[4]` (RW)
///
///   Configure whether or not to enable video B mv merge.\\0: disable\\1: enable.
///
/// - **A_MV_MERGE_EN** `[3]` (RW)
///
///   Configure whether or not to enable video A mv merge.\\0: disable\\1: enable.
///
/// - **INT_MV_OUT_EN** `[2]` (RW)
///
///   Configure mv merge output integer part not zero mv or all part not zero mv.\\0: output all part not zero mv\\1: output integer part not zero mv.
///
/// - **MV_MERGE_TYPE** `[1:0]` (RW)
///
///   Configure mv merge type.\\0: merge p16x16 mv\\1: merge min mv\\2: merge max mv\\3: not valid.
#[repr(transparent)]
pub struct MvMergeConfig {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MvMergeConfig {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MvMergeConfig {
    // ======== Field Constants ========

    /// **MB_VALID_NUM**
    ///
    /// Represents the valid mb number of mv merge output.
    ///
    /// Bits `[17:5]`, mask `0x0003ffe0`
    pub const MB_VALID_NUM_OFFSET: u32 = 5;
    pub const MB_VALID_NUM_MASK: u32 = 0x0003ffe0;

    /// **B_MV_MERGE_EN**
    ///
    /// Configure whether or not to enable video B mv merge.\\0: disable\\1: enable.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const B_MV_MERGE_EN_OFFSET: u32 = 4;
    pub const B_MV_MERGE_EN_MASK: u32 = 0x00000010;
    pub const B_MV_MERGE_EN: u32 = 1 << 4;

    /// **A_MV_MERGE_EN**
    ///
    /// Configure whether or not to enable video A mv merge.\\0: disable\\1: enable.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const A_MV_MERGE_EN_OFFSET: u32 = 3;
    pub const A_MV_MERGE_EN_MASK: u32 = 0x00000008;
    pub const A_MV_MERGE_EN: u32 = 1 << 3;

    /// **INT_MV_OUT_EN**
    ///
    /// Configure mv merge output integer part not zero mv or all part not zero mv.\\0: output all part not zero mv\\1: output integer part not zero mv.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const INT_MV_OUT_EN_OFFSET: u32 = 2;
    pub const INT_MV_OUT_EN_MASK: u32 = 0x00000004;
    pub const INT_MV_OUT_EN: u32 = 1 << 2;

    /// **MV_MERGE_TYPE**
    ///
    /// Configure mv merge type.\\0: merge p16x16 mv\\1: merge min mv\\2: merge max mv\\3: not valid.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MV_MERGE_TYPE_OFFSET: u32 = 0;
    pub const MV_MERGE_TYPE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **MB_VALID_NUM** `[17:5]` (RO)
    ///
    /// Represents the valid mb number of mv merge output.
    #[inline(always)]
    pub fn mb_valid_num(&self) -> u32 {
        (self.value.read_bits() & Self::MB_VALID_NUM_MASK) >> Self::MB_VALID_NUM_OFFSET
    }

    /// **B_MV_MERGE_EN** `[4]` (RW)
    ///
    /// Configure whether or not to enable video B mv merge.\\0: disable\\1: enable.
    #[inline(always)]
    pub fn b_mv_merge_en(&self) -> bool {
        (self.value.read_bits() & Self::B_MV_MERGE_EN_MASK) != 0
    }

    /// Set B_MV_MERGE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_b_mv_merge_en(&self) {
        self.value.modify_bits(|r| r | Self::B_MV_MERGE_EN_MASK);
    }

    /// Clear B_MV_MERGE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_b_mv_merge_en(&self) {
        self.value.modify_bits(|r| r & !Self::B_MV_MERGE_EN_MASK);
    }

    /// **A_MV_MERGE_EN** `[3]` (RW)
    ///
    /// Configure whether or not to enable video A mv merge.\\0: disable\\1: enable.
    #[inline(always)]
    pub fn a_mv_merge_en(&self) -> bool {
        (self.value.read_bits() & Self::A_MV_MERGE_EN_MASK) != 0
    }

    /// Set A_MV_MERGE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_a_mv_merge_en(&self) {
        self.value.modify_bits(|r| r | Self::A_MV_MERGE_EN_MASK);
    }

    /// Clear A_MV_MERGE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_a_mv_merge_en(&self) {
        self.value.modify_bits(|r| r & !Self::A_MV_MERGE_EN_MASK);
    }

    /// **INT_MV_OUT_EN** `[2]` (RW)
    ///
    /// Configure mv merge output integer part not zero mv or all part not zero mv.\\0: output all part not zero mv\\1: output integer part not zero mv.
    #[inline(always)]
    pub fn int_mv_out_en(&self) -> bool {
        (self.value.read_bits() & Self::INT_MV_OUT_EN_MASK) != 0
    }

    /// Set INT_MV_OUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_int_mv_out_en(&self) {
        self.value.modify_bits(|r| r | Self::INT_MV_OUT_EN_MASK);
    }

    /// Clear INT_MV_OUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_int_mv_out_en(&self) {
        self.value.modify_bits(|r| r & !Self::INT_MV_OUT_EN_MASK);
    }

    /// **MV_MERGE_TYPE** `[1:0]` (RW)
    ///
    /// Configure mv merge type.\\0: merge p16x16 mv\\1: merge min mv\\2: merge max mv\\3: not valid.
    #[inline(always)]
    pub fn mv_merge_type(&self) -> u32 {
        (self.value.read_bits() & Self::MV_MERGE_TYPE_MASK) >> Self::MV_MERGE_TYPE_OFFSET
    }

    /// Write **MV_MERGE_TYPE** `[1:0]` (RW)
    ///
    /// Configure mv merge type.\\0: merge p16x16 mv\\1: merge min mv\\2: merge max mv\\3: not valid.
    #[inline(always)]
    pub fn write_mv_merge_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MV_MERGE_TYPE_MASK)
                | ((value << Self::MV_MERGE_TYPE_OFFSET) & Self::MV_MERGE_TYPE_MASK)
        });
    }
}

/// MV_MERGE_CONFIG value type for batch read/modify/write operations
///
/// Use `mvmergeconfig.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mvmergeconfig.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MvMergeConfigVal {
    bits: u32,
}

impl MvMergeConfigVal {
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

    /// Read **MB_VALID_NUM** `[17:5]`
    ///
    /// Represents the valid mb number of mv merge output.
    #[inline(always)]
    pub const fn mb_valid_num(&self) -> u32 {
        (self.bits & MvMergeConfig::MB_VALID_NUM_MASK) >> MvMergeConfig::MB_VALID_NUM_OFFSET
    }

    /// Read **B_MV_MERGE_EN** `[4:4]`
    ///
    /// Configure whether or not to enable video B mv merge.\\0: disable\\1: enable.
    #[inline(always)]
    pub const fn b_mv_merge_en(&self) -> bool {
        (self.bits & MvMergeConfig::B_MV_MERGE_EN_MASK) != 0
    }

    /// Read **A_MV_MERGE_EN** `[3:3]`
    ///
    /// Configure whether or not to enable video A mv merge.\\0: disable\\1: enable.
    #[inline(always)]
    pub const fn a_mv_merge_en(&self) -> bool {
        (self.bits & MvMergeConfig::A_MV_MERGE_EN_MASK) != 0
    }

    /// Read **INT_MV_OUT_EN** `[2:2]`
    ///
    /// Configure mv merge output integer part not zero mv or all part not zero mv.\\0: output all part not zero mv\\1: output integer part not zero mv.
    #[inline(always)]
    pub const fn int_mv_out_en(&self) -> bool {
        (self.bits & MvMergeConfig::INT_MV_OUT_EN_MASK) != 0
    }

    /// Read **MV_MERGE_TYPE** `[1:0]`
    ///
    /// Configure mv merge type.\\0: merge p16x16 mv\\1: merge min mv\\2: merge max mv\\3: not valid.
    #[inline(always)]
    pub const fn mv_merge_type(&self) -> u32 {
        (self.bits & MvMergeConfig::MV_MERGE_TYPE_MASK) >> MvMergeConfig::MV_MERGE_TYPE_OFFSET
    }

    /// Set **B_MV_MERGE_EN** `[4:4]`
    ///
    /// Configure whether or not to enable video B mv merge.\\0: disable\\1: enable.
    #[inline(always)]
    pub const fn with_b_mv_merge_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MvMergeConfig::B_MV_MERGE_EN_MASK
            } else {
                self.bits & !MvMergeConfig::B_MV_MERGE_EN_MASK
            },
        }
    }

    /// Set **A_MV_MERGE_EN** `[3:3]`
    ///
    /// Configure whether or not to enable video A mv merge.\\0: disable\\1: enable.
    #[inline(always)]
    pub const fn with_a_mv_merge_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MvMergeConfig::A_MV_MERGE_EN_MASK
            } else {
                self.bits & !MvMergeConfig::A_MV_MERGE_EN_MASK
            },
        }
    }

    /// Set **INT_MV_OUT_EN** `[2:2]`
    ///
    /// Configure mv merge output integer part not zero mv or all part not zero mv.\\0: output all part not zero mv\\1: output integer part not zero mv.
    #[inline(always)]
    pub const fn with_int_mv_out_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MvMergeConfig::INT_MV_OUT_EN_MASK
            } else {
                self.bits & !MvMergeConfig::INT_MV_OUT_EN_MASK
            },
        }
    }

    /// Set **MV_MERGE_TYPE** `[1:0]`
    ///
    /// Configure mv merge type.\\0: merge p16x16 mv\\1: merge min mv\\2: merge max mv\\3: not valid.
    #[inline(always)]
    pub const fn with_mv_merge_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !MvMergeConfig::MV_MERGE_TYPE_MASK)
                | ((value << MvMergeConfig::MV_MERGE_TYPE_OFFSET)
                    & MvMergeConfig::MV_MERGE_TYPE_MASK),
        }
    }
}

impl Default for MvMergeConfigVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MvMergeConfig {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MvMergeConfigVal {
        MvMergeConfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MvMergeConfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MvMergeConfigVal) -> MvMergeConfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DEBUG_DMA_SEL Register
///
/// Debug H264 DMA select register
///
/// Address offset: 0x00d8
///
/// ## Fields
///
/// - **DBG_DMA_SEL** `[7:0]` (RW)
///
///   Every bit represents a dma in h264
#[repr(transparent)]
pub struct DebugDmaSel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DebugDmaSel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DebugDmaSel {
    // ======== Field Constants ========

    /// **DBG_DMA_SEL**
    ///
    /// Every bit represents a dma in h264
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DBG_DMA_SEL_OFFSET: u32 = 0;
    pub const DBG_DMA_SEL_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **DBG_DMA_SEL** `[7:0]` (RW)
    ///
    /// Every bit represents a dma in h264
    #[inline(always)]
    pub fn dbg_dma_sel(&self) -> u32 {
        (self.value.read_bits() & Self::DBG_DMA_SEL_MASK) >> Self::DBG_DMA_SEL_OFFSET
    }

    /// Write **DBG_DMA_SEL** `[7:0]` (RW)
    ///
    /// Every bit represents a dma in h264
    #[inline(always)]
    pub fn write_dbg_dma_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBG_DMA_SEL_MASK)
                | ((value << Self::DBG_DMA_SEL_OFFSET) & Self::DBG_DMA_SEL_MASK)
        });
    }
}

/// DEBUG_DMA_SEL value type for batch read/modify/write operations
///
/// Use `debugdmasel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `debugdmasel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DebugDmaSelVal {
    bits: u32,
}

impl DebugDmaSelVal {
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

    /// Read **DBG_DMA_SEL** `[7:0]`
    ///
    /// Every bit represents a dma in h264
    #[inline(always)]
    pub const fn dbg_dma_sel(&self) -> u32 {
        (self.bits & DebugDmaSel::DBG_DMA_SEL_MASK) >> DebugDmaSel::DBG_DMA_SEL_OFFSET
    }

    /// Set **DBG_DMA_SEL** `[7:0]`
    ///
    /// Every bit represents a dma in h264
    #[inline(always)]
    pub const fn with_dbg_dma_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DebugDmaSel::DBG_DMA_SEL_MASK)
                | ((value << DebugDmaSel::DBG_DMA_SEL_OFFSET) & DebugDmaSel::DBG_DMA_SEL_MASK),
        }
    }
}

impl Default for DebugDmaSelVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DebugDmaSel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DebugDmaSelVal {
        DebugDmaSelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DebugDmaSelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DebugDmaSelVal) -> DebugDmaSelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SYS_STATUS Register
///
/// System status register.
///
/// Address offset: 0x00dc
///
/// ## Fields
///
/// - **INTRA_FLAG** `[10]` (RO)
///
///   Represents the type of current encoding frame.\\0: P frame\\1: I frame.
///
/// - **DUAL_STREAM_SEL** `[9]` (RO)
///
///   Represents which register group is used for cur frame.\\0: Register group A is used\\1: Register group B is used.
///
/// - **FRAME_NUM** `[8:0]` (RO)
///
///   Represents current frame number.
#[repr(transparent)]
pub struct SysStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SysStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SysStatus {
    // ======== Field Constants ========

    /// **INTRA_FLAG**
    ///
    /// Represents the type of current encoding frame.\\0: P frame\\1: I frame.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const INTRA_FLAG_OFFSET: u32 = 10;
    pub const INTRA_FLAG_MASK: u32 = 0x00000400;
    pub const INTRA_FLAG: u32 = 1 << 10;

    /// **DUAL_STREAM_SEL**
    ///
    /// Represents which register group is used for cur frame.\\0: Register group A is used\\1: Register group B is used.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DUAL_STREAM_SEL_OFFSET: u32 = 9;
    pub const DUAL_STREAM_SEL_MASK: u32 = 0x00000200;
    pub const DUAL_STREAM_SEL: u32 = 1 << 9;

    /// **FRAME_NUM**
    ///
    /// Represents current frame number.
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const FRAME_NUM_OFFSET: u32 = 0;
    pub const FRAME_NUM_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **INTRA_FLAG** `[10]` (RO)
    ///
    /// Represents the type of current encoding frame.\\0: P frame\\1: I frame.
    #[inline(always)]
    pub fn intra_flag(&self) -> bool {
        (self.value.read_bits() & Self::INTRA_FLAG_MASK) != 0
    }

    /// **DUAL_STREAM_SEL** `[9]` (RO)
    ///
    /// Represents which register group is used for cur frame.\\0: Register group A is used\\1: Register group B is used.
    #[inline(always)]
    pub fn dual_stream_sel(&self) -> bool {
        (self.value.read_bits() & Self::DUAL_STREAM_SEL_MASK) != 0
    }

    /// **FRAME_NUM** `[8:0]` (RO)
    ///
    /// Represents current frame number.
    #[inline(always)]
    pub fn frame_num(&self) -> u32 {
        (self.value.read_bits() & Self::FRAME_NUM_MASK) >> Self::FRAME_NUM_OFFSET
    }
}

/// SYS_STATUS value type for batch read/modify/write operations
///
/// Use `sysstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sysstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SysStatusVal {
    bits: u32,
}

impl SysStatusVal {
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

    /// Read **INTRA_FLAG** `[10:10]`
    ///
    /// Represents the type of current encoding frame.\\0: P frame\\1: I frame.
    #[inline(always)]
    pub const fn intra_flag(&self) -> bool {
        (self.bits & SysStatus::INTRA_FLAG_MASK) != 0
    }

    /// Read **DUAL_STREAM_SEL** `[9:9]`
    ///
    /// Represents which register group is used for cur frame.\\0: Register group A is used\\1: Register group B is used.
    #[inline(always)]
    pub const fn dual_stream_sel(&self) -> bool {
        (self.bits & SysStatus::DUAL_STREAM_SEL_MASK) != 0
    }

    /// Read **FRAME_NUM** `[8:0]`
    ///
    /// Represents current frame number.
    #[inline(always)]
    pub const fn frame_num(&self) -> u32 {
        (self.bits & SysStatus::FRAME_NUM_MASK) >> SysStatus::FRAME_NUM_OFFSET
    }
}

impl Default for SysStatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SysStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SysStatusVal {
        SysStatusVal::from_bits(self.value.read_bits())
    }
}

/// FRAME_CODE_LENGTH Register
///
/// Frame code byte length register.
///
/// Address offset: 0x00e0
///
/// ## Fields
///
/// - **FRAME_CODE_LENGTH** `[23:0]` (RO)
///
///   Represents current frame code byte length.
#[repr(transparent)]
pub struct FrameCodeLength {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FrameCodeLength {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FrameCodeLength {
    // ======== Field Constants ========

    /// **FRAME_CODE_LENGTH**
    ///
    /// Represents current frame code byte length.
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const FRAME_CODE_LENGTH_OFFSET: u32 = 0;
    pub const FRAME_CODE_LENGTH_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **FRAME_CODE_LENGTH** `[23:0]` (RO)
    ///
    /// Represents current frame code byte length.
    #[inline(always)]
    pub fn frame_code_length(&self) -> u32 {
        (self.value.read_bits() & Self::FRAME_CODE_LENGTH_MASK) >> Self::FRAME_CODE_LENGTH_OFFSET
    }
}

/// FRAME_CODE_LENGTH value type for batch read/modify/write operations
///
/// Use `framecodelength.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `framecodelength.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FrameCodeLengthVal {
    bits: u32,
}

impl FrameCodeLengthVal {
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

    /// Read **FRAME_CODE_LENGTH** `[23:0]`
    ///
    /// Represents current frame code byte length.
    #[inline(always)]
    pub const fn frame_code_length(&self) -> u32 {
        (self.bits & FrameCodeLength::FRAME_CODE_LENGTH_MASK)
            >> FrameCodeLength::FRAME_CODE_LENGTH_OFFSET
    }
}

impl Default for FrameCodeLengthVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FrameCodeLength {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FrameCodeLengthVal {
        FrameCodeLengthVal::from_bits(self.value.read_bits())
    }
}

/// DEBUG_INFO0 Register
///
/// Debug information register0.
///
/// Address offset: 0x00e4
///
/// ## Fields
///
/// - **IME_CTRL_DEBUG_STATE** `[27:25]` (RO)
///
///   Represents ime_ctrl module FSM info.
///
/// - **INTRA_TOP_CTRL_DEBUG_STATE** `[24:22]` (RO)
///
///   Represents intra_top_ctrl module FSM info.
///
/// - **INTRA_4X4_CTRL_DEBUG_STATE** `[21:18]` (RO)
///
///   Represents intra_4x4_ctrl module FSM info.
///
/// - **INTRA_16X16_CHROMA_CTRL_DEBUG_STATE** `[17:14]` (RO)
///
///   Represents intra_16x16_chroma_ctrl module FSM info.
///
/// - **MC_CHROMA_IP_DEBUG_STATE** `[13]` (RO)
///
///   Represents mc_chroma_ip module FSM info.
///
/// - **MVD_DEBUG_STATE** `[12:10]` (RO)
///
///   Represents mvd module FSM info.
///
/// - **P_I_CMP_DEBUG_STATE** `[9:7]` (RO)
///
///   Represents p_i_cmp module FSM info.
///
/// - **TOP_CTRL_INTRA_DEBUG_STATE** `[6:4]` (RO)
///
///   Represents top_ctrl_intra module FSM info.
///
/// - **TOP_CTRL_INTER_DEBUG_STATE** `[3:0]` (RO)
///
///   Represents top_ctrl_inter module FSM info.
#[repr(transparent)]
pub struct DebugInfo0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DebugInfo0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DebugInfo0 {
    // ======== Field Constants ========

    /// **IME_CTRL_DEBUG_STATE**
    ///
    /// Represents ime_ctrl module FSM info.
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const IME_CTRL_DEBUG_STATE_OFFSET: u32 = 25;
    pub const IME_CTRL_DEBUG_STATE_MASK: u32 = 0x0e000000;

    /// **INTRA_TOP_CTRL_DEBUG_STATE**
    ///
    /// Represents intra_top_ctrl module FSM info.
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const INTRA_TOP_CTRL_DEBUG_STATE_OFFSET: u32 = 22;
    pub const INTRA_TOP_CTRL_DEBUG_STATE_MASK: u32 = 0x01c00000;

    /// **INTRA_4X4_CTRL_DEBUG_STATE**
    ///
    /// Represents intra_4x4_ctrl module FSM info.
    ///
    /// Bits `[21:18]`, mask `0x003c0000`
    pub const INTRA_4X4_CTRL_DEBUG_STATE_OFFSET: u32 = 18;
    pub const INTRA_4X4_CTRL_DEBUG_STATE_MASK: u32 = 0x003c0000;

    /// **INTRA_16X16_CHROMA_CTRL_DEBUG_STATE**
    ///
    /// Represents intra_16x16_chroma_ctrl module FSM info.
    ///
    /// Bits `[17:14]`, mask `0x0003c000`
    pub const INTRA_16X16_CHROMA_CTRL_DEBUG_STATE_OFFSET: u32 = 14;
    pub const INTRA_16X16_CHROMA_CTRL_DEBUG_STATE_MASK: u32 = 0x0003c000;

    /// **MC_CHROMA_IP_DEBUG_STATE**
    ///
    /// Represents mc_chroma_ip module FSM info.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const MC_CHROMA_IP_DEBUG_STATE_OFFSET: u32 = 13;
    pub const MC_CHROMA_IP_DEBUG_STATE_MASK: u32 = 0x00002000;
    pub const MC_CHROMA_IP_DEBUG_STATE: u32 = 1 << 13;

    /// **MVD_DEBUG_STATE**
    ///
    /// Represents mvd module FSM info.
    ///
    /// Bits `[12:10]`, mask `0x00001c00`
    pub const MVD_DEBUG_STATE_OFFSET: u32 = 10;
    pub const MVD_DEBUG_STATE_MASK: u32 = 0x00001c00;

    /// **P_I_CMP_DEBUG_STATE**
    ///
    /// Represents p_i_cmp module FSM info.
    ///
    /// Bits `[9:7]`, mask `0x00000380`
    pub const P_I_CMP_DEBUG_STATE_OFFSET: u32 = 7;
    pub const P_I_CMP_DEBUG_STATE_MASK: u32 = 0x00000380;

    /// **TOP_CTRL_INTRA_DEBUG_STATE**
    ///
    /// Represents top_ctrl_intra module FSM info.
    ///
    /// Bits `[6:4]`, mask `0x00000070`
    pub const TOP_CTRL_INTRA_DEBUG_STATE_OFFSET: u32 = 4;
    pub const TOP_CTRL_INTRA_DEBUG_STATE_MASK: u32 = 0x00000070;

    /// **TOP_CTRL_INTER_DEBUG_STATE**
    ///
    /// Represents top_ctrl_inter module FSM info.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const TOP_CTRL_INTER_DEBUG_STATE_OFFSET: u32 = 0;
    pub const TOP_CTRL_INTER_DEBUG_STATE_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **IME_CTRL_DEBUG_STATE** `[27:25]` (RO)
    ///
    /// Represents ime_ctrl module FSM info.
    #[inline(always)]
    pub fn ime_ctrl_debug_state(&self) -> u32 {
        (self.value.read_bits() & Self::IME_CTRL_DEBUG_STATE_MASK)
            >> Self::IME_CTRL_DEBUG_STATE_OFFSET
    }

    /// **INTRA_TOP_CTRL_DEBUG_STATE** `[24:22]` (RO)
    ///
    /// Represents intra_top_ctrl module FSM info.
    #[inline(always)]
    pub fn intra_top_ctrl_debug_state(&self) -> u32 {
        (self.value.read_bits() & Self::INTRA_TOP_CTRL_DEBUG_STATE_MASK)
            >> Self::INTRA_TOP_CTRL_DEBUG_STATE_OFFSET
    }

    /// **INTRA_4X4_CTRL_DEBUG_STATE** `[21:18]` (RO)
    ///
    /// Represents intra_4x4_ctrl module FSM info.
    #[inline(always)]
    pub fn intra_4x4_ctrl_debug_state(&self) -> u32 {
        (self.value.read_bits() & Self::INTRA_4X4_CTRL_DEBUG_STATE_MASK)
            >> Self::INTRA_4X4_CTRL_DEBUG_STATE_OFFSET
    }

    /// **INTRA_16X16_CHROMA_CTRL_DEBUG_STATE** `[17:14]` (RO)
    ///
    /// Represents intra_16x16_chroma_ctrl module FSM info.
    #[inline(always)]
    pub fn intra_16x16_chroma_ctrl_debug_state(&self) -> u32 {
        (self.value.read_bits() & Self::INTRA_16X16_CHROMA_CTRL_DEBUG_STATE_MASK)
            >> Self::INTRA_16X16_CHROMA_CTRL_DEBUG_STATE_OFFSET
    }

    /// **MC_CHROMA_IP_DEBUG_STATE** `[13]` (RO)
    ///
    /// Represents mc_chroma_ip module FSM info.
    #[inline(always)]
    pub fn mc_chroma_ip_debug_state(&self) -> bool {
        (self.value.read_bits() & Self::MC_CHROMA_IP_DEBUG_STATE_MASK) != 0
    }

    /// **MVD_DEBUG_STATE** `[12:10]` (RO)
    ///
    /// Represents mvd module FSM info.
    #[inline(always)]
    pub fn mvd_debug_state(&self) -> u32 {
        (self.value.read_bits() & Self::MVD_DEBUG_STATE_MASK) >> Self::MVD_DEBUG_STATE_OFFSET
    }

    /// **P_I_CMP_DEBUG_STATE** `[9:7]` (RO)
    ///
    /// Represents p_i_cmp module FSM info.
    #[inline(always)]
    pub fn p_i_cmp_debug_state(&self) -> u32 {
        (self.value.read_bits() & Self::P_I_CMP_DEBUG_STATE_MASK)
            >> Self::P_I_CMP_DEBUG_STATE_OFFSET
    }

    /// **TOP_CTRL_INTRA_DEBUG_STATE** `[6:4]` (RO)
    ///
    /// Represents top_ctrl_intra module FSM info.
    #[inline(always)]
    pub fn top_ctrl_intra_debug_state(&self) -> u32 {
        (self.value.read_bits() & Self::TOP_CTRL_INTRA_DEBUG_STATE_MASK)
            >> Self::TOP_CTRL_INTRA_DEBUG_STATE_OFFSET
    }

    /// **TOP_CTRL_INTER_DEBUG_STATE** `[3:0]` (RO)
    ///
    /// Represents top_ctrl_inter module FSM info.
    #[inline(always)]
    pub fn top_ctrl_inter_debug_state(&self) -> u32 {
        (self.value.read_bits() & Self::TOP_CTRL_INTER_DEBUG_STATE_MASK)
            >> Self::TOP_CTRL_INTER_DEBUG_STATE_OFFSET
    }
}

/// DEBUG_INFO0 value type for batch read/modify/write operations
///
/// Use `debuginfo0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `debuginfo0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DebugInfo0Val {
    bits: u32,
}

impl DebugInfo0Val {
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

    /// Read **IME_CTRL_DEBUG_STATE** `[27:25]`
    ///
    /// Represents ime_ctrl module FSM info.
    #[inline(always)]
    pub const fn ime_ctrl_debug_state(&self) -> u32 {
        (self.bits & DebugInfo0::IME_CTRL_DEBUG_STATE_MASK)
            >> DebugInfo0::IME_CTRL_DEBUG_STATE_OFFSET
    }

    /// Read **INTRA_TOP_CTRL_DEBUG_STATE** `[24:22]`
    ///
    /// Represents intra_top_ctrl module FSM info.
    #[inline(always)]
    pub const fn intra_top_ctrl_debug_state(&self) -> u32 {
        (self.bits & DebugInfo0::INTRA_TOP_CTRL_DEBUG_STATE_MASK)
            >> DebugInfo0::INTRA_TOP_CTRL_DEBUG_STATE_OFFSET
    }

    /// Read **INTRA_4X4_CTRL_DEBUG_STATE** `[21:18]`
    ///
    /// Represents intra_4x4_ctrl module FSM info.
    #[inline(always)]
    pub const fn intra_4x4_ctrl_debug_state(&self) -> u32 {
        (self.bits & DebugInfo0::INTRA_4X4_CTRL_DEBUG_STATE_MASK)
            >> DebugInfo0::INTRA_4X4_CTRL_DEBUG_STATE_OFFSET
    }

    /// Read **INTRA_16X16_CHROMA_CTRL_DEBUG_STATE** `[17:14]`
    ///
    /// Represents intra_16x16_chroma_ctrl module FSM info.
    #[inline(always)]
    pub const fn intra_16x16_chroma_ctrl_debug_state(&self) -> u32 {
        (self.bits & DebugInfo0::INTRA_16X16_CHROMA_CTRL_DEBUG_STATE_MASK)
            >> DebugInfo0::INTRA_16X16_CHROMA_CTRL_DEBUG_STATE_OFFSET
    }

    /// Read **MC_CHROMA_IP_DEBUG_STATE** `[13:13]`
    ///
    /// Represents mc_chroma_ip module FSM info.
    #[inline(always)]
    pub const fn mc_chroma_ip_debug_state(&self) -> bool {
        (self.bits & DebugInfo0::MC_CHROMA_IP_DEBUG_STATE_MASK) != 0
    }

    /// Read **MVD_DEBUG_STATE** `[12:10]`
    ///
    /// Represents mvd module FSM info.
    #[inline(always)]
    pub const fn mvd_debug_state(&self) -> u32 {
        (self.bits & DebugInfo0::MVD_DEBUG_STATE_MASK) >> DebugInfo0::MVD_DEBUG_STATE_OFFSET
    }

    /// Read **P_I_CMP_DEBUG_STATE** `[9:7]`
    ///
    /// Represents p_i_cmp module FSM info.
    #[inline(always)]
    pub const fn p_i_cmp_debug_state(&self) -> u32 {
        (self.bits & DebugInfo0::P_I_CMP_DEBUG_STATE_MASK) >> DebugInfo0::P_I_CMP_DEBUG_STATE_OFFSET
    }

    /// Read **TOP_CTRL_INTRA_DEBUG_STATE** `[6:4]`
    ///
    /// Represents top_ctrl_intra module FSM info.
    #[inline(always)]
    pub const fn top_ctrl_intra_debug_state(&self) -> u32 {
        (self.bits & DebugInfo0::TOP_CTRL_INTRA_DEBUG_STATE_MASK)
            >> DebugInfo0::TOP_CTRL_INTRA_DEBUG_STATE_OFFSET
    }

    /// Read **TOP_CTRL_INTER_DEBUG_STATE** `[3:0]`
    ///
    /// Represents top_ctrl_inter module FSM info.
    #[inline(always)]
    pub const fn top_ctrl_inter_debug_state(&self) -> u32 {
        (self.bits & DebugInfo0::TOP_CTRL_INTER_DEBUG_STATE_MASK)
            >> DebugInfo0::TOP_CTRL_INTER_DEBUG_STATE_OFFSET
    }
}

impl Default for DebugInfo0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DebugInfo0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DebugInfo0Val {
        DebugInfo0Val::from_bits(self.value.read_bits())
    }
}

/// DEBUG_INFO1 Register
///
/// Debug information register1.
///
/// Address offset: 0x00e8
///
/// ## Fields
///
/// - **BS_BUFFER_DEBUG_STATE** `[18]` (RO)
///
///   Represents bs buffer overflow info.
///
/// - **CAVLC_CTRL_DEBUG_STATE** `[17:16]` (RO)
///
///   Represents cavlc module ctrl FSM info.
///
/// - **CAVLC_SCAN_DEBUG_STATE** `[15:12]` (RO)
///
///   Represents cavlc module scan FSM info.
///
/// - **CAVLC_ENC_DEBUG_STATE** `[11:8]` (RO)
///
///   Represents cavlc module enc FSM info.
///
/// - **DB_DEBUG_STATE** `[7:5]` (RO)
///
///   Represents db module FSM info.
///
/// - **DECI_CALC_DEBUG_STATE** `[4:3]` (RO)
///
///   Represents deci_calc module's FSM info. DEV use only.
///
/// - **FME_CTRL_DEBUG_STATE** `[2:0]` (RO)
///
///   Represents fme_ctrl module FSM info.
#[repr(transparent)]
pub struct DebugInfo1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DebugInfo1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DebugInfo1 {
    // ======== Field Constants ========

    /// **BS_BUFFER_DEBUG_STATE**
    ///
    /// Represents bs buffer overflow info.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const BS_BUFFER_DEBUG_STATE_OFFSET: u32 = 18;
    pub const BS_BUFFER_DEBUG_STATE_MASK: u32 = 0x00040000;
    pub const BS_BUFFER_DEBUG_STATE: u32 = 1 << 18;

    /// **CAVLC_CTRL_DEBUG_STATE**
    ///
    /// Represents cavlc module ctrl FSM info.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const CAVLC_CTRL_DEBUG_STATE_OFFSET: u32 = 16;
    pub const CAVLC_CTRL_DEBUG_STATE_MASK: u32 = 0x00030000;

    /// **CAVLC_SCAN_DEBUG_STATE**
    ///
    /// Represents cavlc module scan FSM info.
    ///
    /// Bits `[15:12]`, mask `0x0000f000`
    pub const CAVLC_SCAN_DEBUG_STATE_OFFSET: u32 = 12;
    pub const CAVLC_SCAN_DEBUG_STATE_MASK: u32 = 0x0000f000;

    /// **CAVLC_ENC_DEBUG_STATE**
    ///
    /// Represents cavlc module enc FSM info.
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const CAVLC_ENC_DEBUG_STATE_OFFSET: u32 = 8;
    pub const CAVLC_ENC_DEBUG_STATE_MASK: u32 = 0x00000f00;

    /// **DB_DEBUG_STATE**
    ///
    /// Represents db module FSM info.
    ///
    /// Bits `[7:5]`, mask `0x000000e0`
    pub const DB_DEBUG_STATE_OFFSET: u32 = 5;
    pub const DB_DEBUG_STATE_MASK: u32 = 0x000000e0;

    /// **DECI_CALC_DEBUG_STATE**
    ///
    /// Represents deci_calc module's FSM info. DEV use only.
    ///
    /// Bits `[4:3]`, mask `0x00000018`
    pub const DECI_CALC_DEBUG_STATE_OFFSET: u32 = 3;
    pub const DECI_CALC_DEBUG_STATE_MASK: u32 = 0x00000018;

    /// **FME_CTRL_DEBUG_STATE**
    ///
    /// Represents fme_ctrl module FSM info.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const FME_CTRL_DEBUG_STATE_OFFSET: u32 = 0;
    pub const FME_CTRL_DEBUG_STATE_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **BS_BUFFER_DEBUG_STATE** `[18]` (RO)
    ///
    /// Represents bs buffer overflow info.
    #[inline(always)]
    pub fn bs_buffer_debug_state(&self) -> bool {
        (self.value.read_bits() & Self::BS_BUFFER_DEBUG_STATE_MASK) != 0
    }

    /// **CAVLC_CTRL_DEBUG_STATE** `[17:16]` (RO)
    ///
    /// Represents cavlc module ctrl FSM info.
    #[inline(always)]
    pub fn cavlc_ctrl_debug_state(&self) -> u32 {
        (self.value.read_bits() & Self::CAVLC_CTRL_DEBUG_STATE_MASK)
            >> Self::CAVLC_CTRL_DEBUG_STATE_OFFSET
    }

    /// **CAVLC_SCAN_DEBUG_STATE** `[15:12]` (RO)
    ///
    /// Represents cavlc module scan FSM info.
    #[inline(always)]
    pub fn cavlc_scan_debug_state(&self) -> u32 {
        (self.value.read_bits() & Self::CAVLC_SCAN_DEBUG_STATE_MASK)
            >> Self::CAVLC_SCAN_DEBUG_STATE_OFFSET
    }

    /// **CAVLC_ENC_DEBUG_STATE** `[11:8]` (RO)
    ///
    /// Represents cavlc module enc FSM info.
    #[inline(always)]
    pub fn cavlc_enc_debug_state(&self) -> u32 {
        (self.value.read_bits() & Self::CAVLC_ENC_DEBUG_STATE_MASK)
            >> Self::CAVLC_ENC_DEBUG_STATE_OFFSET
    }

    /// **DB_DEBUG_STATE** `[7:5]` (RO)
    ///
    /// Represents db module FSM info.
    #[inline(always)]
    pub fn db_debug_state(&self) -> u32 {
        (self.value.read_bits() & Self::DB_DEBUG_STATE_MASK) >> Self::DB_DEBUG_STATE_OFFSET
    }

    /// **DECI_CALC_DEBUG_STATE** `[4:3]` (RO)
    ///
    /// Represents deci_calc module's FSM info. DEV use only.
    #[inline(always)]
    pub fn deci_calc_debug_state(&self) -> u32 {
        (self.value.read_bits() & Self::DECI_CALC_DEBUG_STATE_MASK)
            >> Self::DECI_CALC_DEBUG_STATE_OFFSET
    }

    /// **FME_CTRL_DEBUG_STATE** `[2:0]` (RO)
    ///
    /// Represents fme_ctrl module FSM info.
    #[inline(always)]
    pub fn fme_ctrl_debug_state(&self) -> u32 {
        (self.value.read_bits() & Self::FME_CTRL_DEBUG_STATE_MASK)
            >> Self::FME_CTRL_DEBUG_STATE_OFFSET
    }
}

/// DEBUG_INFO1 value type for batch read/modify/write operations
///
/// Use `debuginfo1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `debuginfo1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DebugInfo1Val {
    bits: u32,
}

impl DebugInfo1Val {
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

    /// Read **BS_BUFFER_DEBUG_STATE** `[18:18]`
    ///
    /// Represents bs buffer overflow info.
    #[inline(always)]
    pub const fn bs_buffer_debug_state(&self) -> bool {
        (self.bits & DebugInfo1::BS_BUFFER_DEBUG_STATE_MASK) != 0
    }

    /// Read **CAVLC_CTRL_DEBUG_STATE** `[17:16]`
    ///
    /// Represents cavlc module ctrl FSM info.
    #[inline(always)]
    pub const fn cavlc_ctrl_debug_state(&self) -> u32 {
        (self.bits & DebugInfo1::CAVLC_CTRL_DEBUG_STATE_MASK)
            >> DebugInfo1::CAVLC_CTRL_DEBUG_STATE_OFFSET
    }

    /// Read **CAVLC_SCAN_DEBUG_STATE** `[15:12]`
    ///
    /// Represents cavlc module scan FSM info.
    #[inline(always)]
    pub const fn cavlc_scan_debug_state(&self) -> u32 {
        (self.bits & DebugInfo1::CAVLC_SCAN_DEBUG_STATE_MASK)
            >> DebugInfo1::CAVLC_SCAN_DEBUG_STATE_OFFSET
    }

    /// Read **CAVLC_ENC_DEBUG_STATE** `[11:8]`
    ///
    /// Represents cavlc module enc FSM info.
    #[inline(always)]
    pub const fn cavlc_enc_debug_state(&self) -> u32 {
        (self.bits & DebugInfo1::CAVLC_ENC_DEBUG_STATE_MASK)
            >> DebugInfo1::CAVLC_ENC_DEBUG_STATE_OFFSET
    }

    /// Read **DB_DEBUG_STATE** `[7:5]`
    ///
    /// Represents db module FSM info.
    #[inline(always)]
    pub const fn db_debug_state(&self) -> u32 {
        (self.bits & DebugInfo1::DB_DEBUG_STATE_MASK) >> DebugInfo1::DB_DEBUG_STATE_OFFSET
    }

    /// Read **DECI_CALC_DEBUG_STATE** `[4:3]`
    ///
    /// Represents deci_calc module's FSM info. DEV use only.
    #[inline(always)]
    pub const fn deci_calc_debug_state(&self) -> u32 {
        (self.bits & DebugInfo1::DECI_CALC_DEBUG_STATE_MASK)
            >> DebugInfo1::DECI_CALC_DEBUG_STATE_OFFSET
    }

    /// Read **FME_CTRL_DEBUG_STATE** `[2:0]`
    ///
    /// Represents fme_ctrl module FSM info.
    #[inline(always)]
    pub const fn fme_ctrl_debug_state(&self) -> u32 {
        (self.bits & DebugInfo1::FME_CTRL_DEBUG_STATE_MASK)
            >> DebugInfo1::FME_CTRL_DEBUG_STATE_OFFSET
    }
}

impl Default for DebugInfo1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DebugInfo1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DebugInfo1Val {
        DebugInfo1Val::from_bits(self.value.read_bits())
    }
}

/// DEBUG_INFO2 Register
///
/// Debug information register2.
///
/// Address offset: 0x00ec
///
/// ## Fields
///
/// - **I_BS_BUF_DONE_DEBUG_FLAG** `[17]` (RO)
///
///   Represents I bitstream buffer done status.\\0: not done\\1: done.
///
/// - **I_DB_DONE_DEBUG_FLAG** `[16]` (RO)
///
///   Represents I deblocking done status.\\0: not done\\1: done.
///
/// - **I_EC_DONE_DEBUG_FLAG** `[15]` (RO)
///
///   Represents I encoder done status.\\0: not done\\1: done.
///
/// - **I_GET_ORI_DONE_DEBUG_FLAG** `[14]` (RO)
///
///   Represents I get origin done status.\\0: not done\\1: done.
///
/// - **I_MOVE_ORI_DONE_DEBUG_FLAG** `[13]` (RO)
///
///   Represents I move origin done status.\\0: not done\\1: done.
///
/// - **I_P_I_CMP_DONE_DEBUG_FLAG** `[12]` (RO)
///
///   Represents I p_i_cmp done status.\\0: not done\\1: done.
///
/// - **REF_MOVE_2MB_LINE_DONE_DEBUG_FLAG** `[11]` (RO)
///
///   Represents dma move 2 ref mb line done status.\\0: not done\\1: done.
///
/// - **P_BS_BUF_DONE_DEBUG_FLAG** `[10]` (RO)
///
///   Represents p bitstream buffer done status.\\0: not done\\1: done.
///
/// - **P_DB_DONE_DEBUG_FLAG** `[9]` (RO)
///
///   Represents p deblocking done status.\\0: not done\\1: done.
///
/// - **P_FETCH_DONE_DEBUG_FLAG** `[8]` (RO)
///
///   Represents p fetch done status.\\0: not done\\1: done.
///
/// - **P_FME_DONE_DEBUG_FLAG** `[7]` (RO)
///
///   Represents p fme done status.\\0: not done\\1: done.
///
/// - **P_GET_ORI_DONE_DEBUG_FLAG** `[6]` (RO)
///
///   Represents p get origin done status.\\0: not done\\1: done.
///
/// - **P_IME_DONE_DEBUG_FLAG** `[5]` (RO)
///
///   Represents p ime done status.\\0: not done\\1: done.
///
/// - **P_MC_DONE_DEBUG_FLAG** `[4]` (RO)
///
///   Represents p mc done status.\\0: not done\\1: done.
///
/// - **P_MOVE_ORI_DONE_DEBUG_FLAG** `[3]` (RO)
///
///   Represents p move origin done status.\\0: not done\\1: done.
///
/// - **P_MV_MERGE_DONE_DEBUG_FLAG** `[2]` (RO)
///
///   Represents p mv merge done status.\\0: not done\\1: done.
///
/// - **P_P_I_CMP_DONE_DEBUG_FLAG** `[1]` (RO)
///
///   Represents p p_i_cmp done status.\\0: not done\\1: done.
///
/// - **P_RC_DONE_DEBUG_FLAG** `[0]` (RO)
///
///   Represents p rate ctrl done status.\\0: not done\\1: done.
#[repr(transparent)]
pub struct DebugInfo2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DebugInfo2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DebugInfo2 {
    // ======== Field Constants ========

    /// **I_BS_BUF_DONE_DEBUG_FLAG**
    ///
    /// Represents I bitstream buffer done status.\\0: not done\\1: done.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const I_BS_BUF_DONE_DEBUG_FLAG_OFFSET: u32 = 17;
    pub const I_BS_BUF_DONE_DEBUG_FLAG_MASK: u32 = 0x00020000;
    pub const I_BS_BUF_DONE_DEBUG_FLAG: u32 = 1 << 17;

    /// **I_DB_DONE_DEBUG_FLAG**
    ///
    /// Represents I deblocking done status.\\0: not done\\1: done.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const I_DB_DONE_DEBUG_FLAG_OFFSET: u32 = 16;
    pub const I_DB_DONE_DEBUG_FLAG_MASK: u32 = 0x00010000;
    pub const I_DB_DONE_DEBUG_FLAG: u32 = 1 << 16;

    /// **I_EC_DONE_DEBUG_FLAG**
    ///
    /// Represents I encoder done status.\\0: not done\\1: done.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const I_EC_DONE_DEBUG_FLAG_OFFSET: u32 = 15;
    pub const I_EC_DONE_DEBUG_FLAG_MASK: u32 = 0x00008000;
    pub const I_EC_DONE_DEBUG_FLAG: u32 = 1 << 15;

    /// **I_GET_ORI_DONE_DEBUG_FLAG**
    ///
    /// Represents I get origin done status.\\0: not done\\1: done.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const I_GET_ORI_DONE_DEBUG_FLAG_OFFSET: u32 = 14;
    pub const I_GET_ORI_DONE_DEBUG_FLAG_MASK: u32 = 0x00004000;
    pub const I_GET_ORI_DONE_DEBUG_FLAG: u32 = 1 << 14;

    /// **I_MOVE_ORI_DONE_DEBUG_FLAG**
    ///
    /// Represents I move origin done status.\\0: not done\\1: done.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const I_MOVE_ORI_DONE_DEBUG_FLAG_OFFSET: u32 = 13;
    pub const I_MOVE_ORI_DONE_DEBUG_FLAG_MASK: u32 = 0x00002000;
    pub const I_MOVE_ORI_DONE_DEBUG_FLAG: u32 = 1 << 13;

    /// **I_P_I_CMP_DONE_DEBUG_FLAG**
    ///
    /// Represents I p_i_cmp done status.\\0: not done\\1: done.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const I_P_I_CMP_DONE_DEBUG_FLAG_OFFSET: u32 = 12;
    pub const I_P_I_CMP_DONE_DEBUG_FLAG_MASK: u32 = 0x00001000;
    pub const I_P_I_CMP_DONE_DEBUG_FLAG: u32 = 1 << 12;

    /// **REF_MOVE_2MB_LINE_DONE_DEBUG_FLAG**
    ///
    /// Represents dma move 2 ref mb line done status.\\0: not done\\1: done.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REF_MOVE_2MB_LINE_DONE_DEBUG_FLAG_OFFSET: u32 = 11;
    pub const REF_MOVE_2MB_LINE_DONE_DEBUG_FLAG_MASK: u32 = 0x00000800;
    pub const REF_MOVE_2MB_LINE_DONE_DEBUG_FLAG: u32 = 1 << 11;

    /// **P_BS_BUF_DONE_DEBUG_FLAG**
    ///
    /// Represents p bitstream buffer done status.\\0: not done\\1: done.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const P_BS_BUF_DONE_DEBUG_FLAG_OFFSET: u32 = 10;
    pub const P_BS_BUF_DONE_DEBUG_FLAG_MASK: u32 = 0x00000400;
    pub const P_BS_BUF_DONE_DEBUG_FLAG: u32 = 1 << 10;

    /// **P_DB_DONE_DEBUG_FLAG**
    ///
    /// Represents p deblocking done status.\\0: not done\\1: done.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const P_DB_DONE_DEBUG_FLAG_OFFSET: u32 = 9;
    pub const P_DB_DONE_DEBUG_FLAG_MASK: u32 = 0x00000200;
    pub const P_DB_DONE_DEBUG_FLAG: u32 = 1 << 9;

    /// **P_FETCH_DONE_DEBUG_FLAG**
    ///
    /// Represents p fetch done status.\\0: not done\\1: done.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const P_FETCH_DONE_DEBUG_FLAG_OFFSET: u32 = 8;
    pub const P_FETCH_DONE_DEBUG_FLAG_MASK: u32 = 0x00000100;
    pub const P_FETCH_DONE_DEBUG_FLAG: u32 = 1 << 8;

    /// **P_FME_DONE_DEBUG_FLAG**
    ///
    /// Represents p fme done status.\\0: not done\\1: done.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const P_FME_DONE_DEBUG_FLAG_OFFSET: u32 = 7;
    pub const P_FME_DONE_DEBUG_FLAG_MASK: u32 = 0x00000080;
    pub const P_FME_DONE_DEBUG_FLAG: u32 = 1 << 7;

    /// **P_GET_ORI_DONE_DEBUG_FLAG**
    ///
    /// Represents p get origin done status.\\0: not done\\1: done.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const P_GET_ORI_DONE_DEBUG_FLAG_OFFSET: u32 = 6;
    pub const P_GET_ORI_DONE_DEBUG_FLAG_MASK: u32 = 0x00000040;
    pub const P_GET_ORI_DONE_DEBUG_FLAG: u32 = 1 << 6;

    /// **P_IME_DONE_DEBUG_FLAG**
    ///
    /// Represents p ime done status.\\0: not done\\1: done.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const P_IME_DONE_DEBUG_FLAG_OFFSET: u32 = 5;
    pub const P_IME_DONE_DEBUG_FLAG_MASK: u32 = 0x00000020;
    pub const P_IME_DONE_DEBUG_FLAG: u32 = 1 << 5;

    /// **P_MC_DONE_DEBUG_FLAG**
    ///
    /// Represents p mc done status.\\0: not done\\1: done.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const P_MC_DONE_DEBUG_FLAG_OFFSET: u32 = 4;
    pub const P_MC_DONE_DEBUG_FLAG_MASK: u32 = 0x00000010;
    pub const P_MC_DONE_DEBUG_FLAG: u32 = 1 << 4;

    /// **P_MOVE_ORI_DONE_DEBUG_FLAG**
    ///
    /// Represents p move origin done status.\\0: not done\\1: done.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const P_MOVE_ORI_DONE_DEBUG_FLAG_OFFSET: u32 = 3;
    pub const P_MOVE_ORI_DONE_DEBUG_FLAG_MASK: u32 = 0x00000008;
    pub const P_MOVE_ORI_DONE_DEBUG_FLAG: u32 = 1 << 3;

    /// **P_MV_MERGE_DONE_DEBUG_FLAG**
    ///
    /// Represents p mv merge done status.\\0: not done\\1: done.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const P_MV_MERGE_DONE_DEBUG_FLAG_OFFSET: u32 = 2;
    pub const P_MV_MERGE_DONE_DEBUG_FLAG_MASK: u32 = 0x00000004;
    pub const P_MV_MERGE_DONE_DEBUG_FLAG: u32 = 1 << 2;

    /// **P_P_I_CMP_DONE_DEBUG_FLAG**
    ///
    /// Represents p p_i_cmp done status.\\0: not done\\1: done.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const P_P_I_CMP_DONE_DEBUG_FLAG_OFFSET: u32 = 1;
    pub const P_P_I_CMP_DONE_DEBUG_FLAG_MASK: u32 = 0x00000002;
    pub const P_P_I_CMP_DONE_DEBUG_FLAG: u32 = 1 << 1;

    /// **P_RC_DONE_DEBUG_FLAG**
    ///
    /// Represents p rate ctrl done status.\\0: not done\\1: done.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const P_RC_DONE_DEBUG_FLAG_OFFSET: u32 = 0;
    pub const P_RC_DONE_DEBUG_FLAG_MASK: u32 = 0x00000001;
    pub const P_RC_DONE_DEBUG_FLAG: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **I_BS_BUF_DONE_DEBUG_FLAG** `[17]` (RO)
    ///
    /// Represents I bitstream buffer done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn i_bs_buf_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::I_BS_BUF_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **I_DB_DONE_DEBUG_FLAG** `[16]` (RO)
    ///
    /// Represents I deblocking done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn i_db_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::I_DB_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **I_EC_DONE_DEBUG_FLAG** `[15]` (RO)
    ///
    /// Represents I encoder done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn i_ec_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::I_EC_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **I_GET_ORI_DONE_DEBUG_FLAG** `[14]` (RO)
    ///
    /// Represents I get origin done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn i_get_ori_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::I_GET_ORI_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **I_MOVE_ORI_DONE_DEBUG_FLAG** `[13]` (RO)
    ///
    /// Represents I move origin done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn i_move_ori_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::I_MOVE_ORI_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **I_P_I_CMP_DONE_DEBUG_FLAG** `[12]` (RO)
    ///
    /// Represents I p_i_cmp done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn i_p_i_cmp_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::I_P_I_CMP_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **REF_MOVE_2MB_LINE_DONE_DEBUG_FLAG** `[11]` (RO)
    ///
    /// Represents dma move 2 ref mb line done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn ref_move_2mb_line_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::REF_MOVE_2MB_LINE_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **P_BS_BUF_DONE_DEBUG_FLAG** `[10]` (RO)
    ///
    /// Represents p bitstream buffer done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn p_bs_buf_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::P_BS_BUF_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **P_DB_DONE_DEBUG_FLAG** `[9]` (RO)
    ///
    /// Represents p deblocking done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn p_db_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::P_DB_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **P_FETCH_DONE_DEBUG_FLAG** `[8]` (RO)
    ///
    /// Represents p fetch done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn p_fetch_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::P_FETCH_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **P_FME_DONE_DEBUG_FLAG** `[7]` (RO)
    ///
    /// Represents p fme done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn p_fme_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::P_FME_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **P_GET_ORI_DONE_DEBUG_FLAG** `[6]` (RO)
    ///
    /// Represents p get origin done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn p_get_ori_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::P_GET_ORI_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **P_IME_DONE_DEBUG_FLAG** `[5]` (RO)
    ///
    /// Represents p ime done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn p_ime_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::P_IME_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **P_MC_DONE_DEBUG_FLAG** `[4]` (RO)
    ///
    /// Represents p mc done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn p_mc_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::P_MC_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **P_MOVE_ORI_DONE_DEBUG_FLAG** `[3]` (RO)
    ///
    /// Represents p move origin done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn p_move_ori_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::P_MOVE_ORI_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **P_MV_MERGE_DONE_DEBUG_FLAG** `[2]` (RO)
    ///
    /// Represents p mv merge done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn p_mv_merge_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::P_MV_MERGE_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **P_P_I_CMP_DONE_DEBUG_FLAG** `[1]` (RO)
    ///
    /// Represents p p_i_cmp done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn p_p_i_cmp_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::P_P_I_CMP_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// **P_RC_DONE_DEBUG_FLAG** `[0]` (RO)
    ///
    /// Represents p rate ctrl done status.\\0: not done\\1: done.
    #[inline(always)]
    pub fn p_rc_done_debug_flag(&self) -> bool {
        (self.value.read_bits() & Self::P_RC_DONE_DEBUG_FLAG_MASK) != 0
    }
}

/// DEBUG_INFO2 value type for batch read/modify/write operations
///
/// Use `debuginfo2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `debuginfo2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DebugInfo2Val {
    bits: u32,
}

impl DebugInfo2Val {
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

    /// Read **I_BS_BUF_DONE_DEBUG_FLAG** `[17:17]`
    ///
    /// Represents I bitstream buffer done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn i_bs_buf_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::I_BS_BUF_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **I_DB_DONE_DEBUG_FLAG** `[16:16]`
    ///
    /// Represents I deblocking done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn i_db_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::I_DB_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **I_EC_DONE_DEBUG_FLAG** `[15:15]`
    ///
    /// Represents I encoder done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn i_ec_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::I_EC_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **I_GET_ORI_DONE_DEBUG_FLAG** `[14:14]`
    ///
    /// Represents I get origin done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn i_get_ori_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::I_GET_ORI_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **I_MOVE_ORI_DONE_DEBUG_FLAG** `[13:13]`
    ///
    /// Represents I move origin done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn i_move_ori_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::I_MOVE_ORI_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **I_P_I_CMP_DONE_DEBUG_FLAG** `[12:12]`
    ///
    /// Represents I p_i_cmp done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn i_p_i_cmp_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::I_P_I_CMP_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **REF_MOVE_2MB_LINE_DONE_DEBUG_FLAG** `[11:11]`
    ///
    /// Represents dma move 2 ref mb line done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn ref_move_2mb_line_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::REF_MOVE_2MB_LINE_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **P_BS_BUF_DONE_DEBUG_FLAG** `[10:10]`
    ///
    /// Represents p bitstream buffer done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn p_bs_buf_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::P_BS_BUF_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **P_DB_DONE_DEBUG_FLAG** `[9:9]`
    ///
    /// Represents p deblocking done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn p_db_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::P_DB_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **P_FETCH_DONE_DEBUG_FLAG** `[8:8]`
    ///
    /// Represents p fetch done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn p_fetch_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::P_FETCH_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **P_FME_DONE_DEBUG_FLAG** `[7:7]`
    ///
    /// Represents p fme done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn p_fme_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::P_FME_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **P_GET_ORI_DONE_DEBUG_FLAG** `[6:6]`
    ///
    /// Represents p get origin done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn p_get_ori_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::P_GET_ORI_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **P_IME_DONE_DEBUG_FLAG** `[5:5]`
    ///
    /// Represents p ime done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn p_ime_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::P_IME_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **P_MC_DONE_DEBUG_FLAG** `[4:4]`
    ///
    /// Represents p mc done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn p_mc_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::P_MC_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **P_MOVE_ORI_DONE_DEBUG_FLAG** `[3:3]`
    ///
    /// Represents p move origin done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn p_move_ori_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::P_MOVE_ORI_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **P_MV_MERGE_DONE_DEBUG_FLAG** `[2:2]`
    ///
    /// Represents p mv merge done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn p_mv_merge_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::P_MV_MERGE_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **P_P_I_CMP_DONE_DEBUG_FLAG** `[1:1]`
    ///
    /// Represents p p_i_cmp done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn p_p_i_cmp_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::P_P_I_CMP_DONE_DEBUG_FLAG_MASK) != 0
    }

    /// Read **P_RC_DONE_DEBUG_FLAG** `[0:0]`
    ///
    /// Represents p rate ctrl done status.\\0: not done\\1: done.
    #[inline(always)]
    pub const fn p_rc_done_debug_flag(&self) -> bool {
        (self.bits & DebugInfo2::P_RC_DONE_DEBUG_FLAG_MASK) != 0
    }
}

impl Default for DebugInfo2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DebugInfo2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DebugInfo2Val {
        DebugInfo2Val::from_bits(self.value.read_bits())
    }
}

/// DATE Register
///
/// Version control register
///
/// Address offset: 0x00f0
/// Reset value: 0x02304240
///
/// ## Fields
///
/// - **LEDC_DATE** `[27:0]` (RW)
///
///   Configures the version.
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

    /// **LEDC_DATE**
    ///
    /// Configures the version.
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const LEDC_DATE_OFFSET: u32 = 0;
    pub const LEDC_DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **LEDC_DATE** `[27:0]` (RW)
    ///
    /// Configures the version.
    #[inline(always)]
    pub fn ledc_date(&self) -> u32 {
        (self.value.read_bits() & Self::LEDC_DATE_MASK) >> Self::LEDC_DATE_OFFSET
    }

    /// Write **LEDC_DATE** `[27:0]` (RW)
    ///
    /// Configures the version.
    #[inline(always)]
    pub fn write_ledc_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LEDC_DATE_MASK) | ((value << Self::LEDC_DATE_OFFSET) & Self::LEDC_DATE_MASK)
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
        Self { bits: 0x02304240 }
    }

    /// Read **LEDC_DATE** `[27:0]`
    ///
    /// Configures the version.
    #[inline(always)]
    pub const fn ledc_date(&self) -> u32 {
        (self.bits & Date::LEDC_DATE_MASK) >> Date::LEDC_DATE_OFFSET
    }

    /// Set **LEDC_DATE** `[27:0]`
    ///
    /// Configures the version.
    #[inline(always)]
    pub const fn with_ledc_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Date::LEDC_DATE_MASK)
                | ((value << Date::LEDC_DATE_OFFSET) & Date::LEDC_DATE_MASK),
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
    /// H264 system level control register.
    pub sys_ctrl: SysCtrl,
    /// 0x0004
    /// GOP related configuration register.
    pub gop_conf: GopConf,
    /// 0x0008
    /// Video A horizontal and vertical MB resolution register.
    pub a_sys_mb_res: ASysMbRes,
    /// 0x000c
    /// Video A system level configuration register.
    pub a_sys_conf: ASysConf,
    /// 0x0010
    /// Video A luma and chroma MB decimate score Register.
    pub a_deci_score: ADeciScore,
    /// 0x0014
    /// Video A luma and chroma MB decimate score offset Register.
    pub a_deci_score_offset: ADeciScoreOffset,
    /// 0x0018
    /// Video A rate control configuration register0.
    pub a_rc_conf0: ARcConf0,
    /// 0x001c
    /// Video A rate control configuration register1.
    pub a_rc_conf1: ARcConf1,
    /// 0x0020
    /// Video A Deblocking bypass register
    pub a_db_bypass: ADbBypass,
    /// 0x0024
    /// Video A H264 ROI region0 range configure register.
    pub a_roi_region0: ARoiRegion0,
    /// 0x0028
    /// Video A H264 ROI region1 range configure register.
    pub a_roi_region1: ARoiRegion1,
    /// 0x002c
    /// Video A H264 ROI region2 range configure register.
    pub a_roi_region2: ARoiRegion2,
    /// 0x0030
    /// Video A H264 ROI region3 range configure register.
    pub a_roi_region3: ARoiRegion3,
    /// 0x0034
    /// Video A H264 ROI region4 range configure register.
    pub a_roi_region4: ARoiRegion4,
    /// 0x0038
    /// Video A H264 ROI region5 range configure register.
    pub a_roi_region5: ARoiRegion5,
    /// 0x003c
    /// Video A H264 ROI region6 range configure register.
    pub a_roi_region6: ARoiRegion6,
    /// 0x0040
    /// Video A H264 ROI region7 range configure register.
    pub a_roi_region7: ARoiRegion7,
    /// 0x0044
    /// Video A H264 ROI region0, region1,region2,region3 QP register.
    pub a_roi_region03_qp: ARoiRegion03Qp,
    /// 0x0048
    /// Video A H264 ROI region4, region5,region6,region7 QP register.
    pub a_roi_region47_qp: ARoiRegion47Qp,
    /// 0x004c
    /// Video A H264 no roi region QP register.
    pub a_no_roi_region_qp_offset: ANoRoiRegionQpOffset,
    /// 0x0050
    /// Video A H264 ROI configure register.
    pub a_roi_config: ARoiConfig,
    /// 0x0054
    /// Video B horizontal and vertical MB resolution register.
    pub b_sys_mb_res: BSysMbRes,
    /// 0x0058
    /// Video B system level configuration register.
    pub b_sys_conf: BSysConf,
    /// 0x005c
    /// Video B luma and chroma MB decimate score Register.
    pub b_deci_score: BDeciScore,
    /// 0x0060
    /// Video B luma and chroma MB decimate score offset Register.
    pub b_deci_score_offset: BDeciScoreOffset,
    /// 0x0064
    /// Video B rate control configuration register0.
    pub b_rc_conf0: BRcConf0,
    /// 0x0068
    /// Video B rate control configuration register1.
    pub b_rc_conf1: BRcConf1,
    /// 0x006c
    /// Video B Deblocking bypass register
    pub b_db_bypass: BDbBypass,
    /// 0x0070
    /// Video B H264 ROI region0 range configure register.
    pub b_roi_region0: BRoiRegion0,
    /// 0x0074
    /// Video B H264 ROI region1 range configure register.
    pub b_roi_region1: BRoiRegion1,
    /// 0x0078
    /// Video B H264 ROI region2 range configure register.
    pub b_roi_region2: BRoiRegion2,
    /// 0x007c
    /// Video B H264 ROI region3 range configure register.
    pub b_roi_region3: BRoiRegion3,
    /// 0x0080
    /// Video B H264 ROI region4 range configure register.
    pub b_roi_region4: BRoiRegion4,
    /// 0x0084
    /// Video B H264 ROI region5 range configure register.
    pub b_roi_region5: BRoiRegion5,
    /// 0x0088
    /// Video B H264 ROI region6 range configure register.
    pub b_roi_region6: BRoiRegion6,
    /// 0x008c
    /// Video B H264 ROI region7 range configure register.
    pub b_roi_region7: BRoiRegion7,
    /// 0x0090
    /// Video B H264 ROI region0, region1,region2,region3 QP register.
    pub b_roi_region03_qp: BRoiRegion03Qp,
    /// 0x0094
    /// Video B H264 ROI region4, region5,region6,region7 QP register.
    pub b_roi_region47_qp: BRoiRegion47Qp,
    /// 0x0098
    /// Video B H264 no roi region QP register.
    pub b_no_roi_region_qp_offset: BNoRoiRegionQpOffset,
    /// 0x009c
    /// Video B H264 ROI configure register.
    pub b_roi_config: BRoiConfig,
    /// 0x00a0
    /// Rate control status register0.
    pub rc_status0: RcStatus0,
    /// 0x00a4
    /// Rate control status register1.
    pub rc_status1: RcStatus1,
    /// 0x00a8
    /// Rate control status register2.
    pub rc_status2: RcStatus2,
    /// 0x00ac
    /// Frame Slice Header remain bit register.
    pub slice_header_remain: SliceHeaderRemain,
    /// 0x00b0
    /// Frame Slice Header byte length register.
    pub slice_header_byte_length: SliceHeaderByteLength,
    /// 0x00b4
    /// Bitstream buffer overflow threshold register
    pub bs_threshold: BsThreshold,
    /// 0x00b8
    /// Frame Slice Header byte low 32 bit register.
    pub slice_header_byte0: SliceHeaderByte0,
    /// 0x00bc
    /// Frame Slice Header byte high 32 bit register.
    pub slice_header_byte1: SliceHeaderByte1,
    /// 0x00c0
    /// Interrupt raw status register
    pub int_raw: IntRaw,
    /// 0x00c4
    /// Interrupt masked status register
    pub int_st: IntSt,
    /// 0x00c8
    /// Interrupt enable register
    pub int_ena: IntEna,
    /// 0x00cc
    /// Interrupt clear register
    pub int_clr: IntClr,
    /// 0x00d0
    /// General configuration register.
    pub conf: Conf,
    /// 0x00d4
    /// Mv merge configuration register.
    pub mv_merge_config: MvMergeConfig,
    /// 0x00d8
    /// Debug H264 DMA select register
    pub debug_dma_sel: DebugDmaSel,
    /// 0x00dc
    /// System status register.
    pub sys_status: SysStatus,
    /// 0x00e0
    /// Frame code byte length register.
    pub frame_code_length: FrameCodeLength,
    /// 0x00e4
    /// Debug information register0.
    pub debug_info0: DebugInfo0,
    /// 0x00e8
    /// Debug information register1.
    pub debug_info1: DebugInfo1,
    /// 0x00ec
    /// Debug information register2.
    pub debug_info2: DebugInfo2,
    /// 0x00f0
    /// Version control register
    pub date: Date,
}
