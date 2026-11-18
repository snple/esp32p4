//! JPEG
//! JPEG Codec

#![allow(dead_code)]

pub const BASE: u32 = 0x50086000;

/// CONFIG Register
///
/// Control and configuration registers
///
/// Address offset: 0x0000
/// Reset value: 0x00408958
///
/// ## Fields
///
/// - **MODE** `[31]` (RW)
///
///   0:encoder mode, 1: decoder mode
///
/// - **MEM_FORCE_PU** `[30]` (RW)
///
///   0: no operation,1:force jpeg memory to power up
///
/// - **MEM_FORCE_PD** `[29]` (RW)
///
///   0: no operation,1:force jpeg memory to power down
///
/// - **PAUSE_EN** `[28]` (RW)
///
///   set this bit to pause jpeg encoding
///
/// - **TAILER_EN** `[27]` (RW)
///
///   set this bit to add EOI of "0xffd9" at the end of bitstream
///
/// - **PIXEL_REV** `[26]` (RW)
///
///   reverse the source color pixel
///
/// - **FIFO_RST** `[25]` (RW)
///
///   fifo reset
///
/// - **SOFT_RST** `[24]` (RW)
///
///   when set to 1, soft reset JPEG module except jpeg_reg module
///
/// - **DECODE_TIMEOUT_TASK_SEL** `[23]` (RW)
///
///   0: software use reset to abort decode process ,1: decoder abort decode process by itself
///
/// - **JFIF_VER** `[22:17]` (RW)
///
///   decode pause period to trigger decode_timeout int, the timeout periods =2 power (reg_decode_timeout_thres) -1
///
/// - **MEM_CLK_FORCE_ON** `[16]` (RW)
///
///   force memory's clock enabled
///
/// - **DHT_FIFO_EN** `[15]` (RW)
///
///   0:use non-fifo way to write dht len_total/codemin/value table,1:use fifo way to write dht len_total/codemin/value table. Reading dht len_total/codemin/value table only has nonfifo way
///
/// - **COLOR_SPACE** `[14:13]` (RW)
///
///   configure picture's color space:0-rb888,1-yuv422,2-rgb565, 3-gray
///
/// - **CQNR_TBL_SEL** `[12:11]` (RW)
///
///   choose chrominance quntization table id (TBD)
///
/// - **LQNR_TBL_SEL** `[10:9]` (RW)
///
///   choose luminance quntization table id(TBD)
///
/// - **GRAY_SEL** `[8]` (RW)
///
///   0:use non-fifo way to access qnr ram,1:use fifo way to access qnr ram
///
/// - **DEBUG_DIRECT_OUT_EN** `[7]` (RW)
///
///   0:normal mode,1:debug mode for direct output from input
///
/// - **DMA_LINKLIST_MODE** `[6]` (RO)
///
///   1:use linklist to configure dma
///
/// - **SAMPLE_SEL** `[5:4]` (RW)
///
///   0:yuv444,1:yuv422, 2:yuv420
///
/// - **FF_CHECK_EN** `[3]` (RW)
///
///   enable whether to add "00" after "ff"
///
/// - **QNR_PRESITION** `[2]` (RW)
///
///   0:8bit qnr,1:12bit qnr(TBD)
///
/// - **JPEG_START** `[1]` (WO)
///
///   start to compress a new pic(in dma reg mode)
///
/// - **FSM_RST** `[0]` (WO)
///
///   fsm reset
#[repr(transparent)]
pub struct Config {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Config {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Config {
    // ======== Field Constants ========

    /// **MODE**
    ///
    /// 0:encoder mode, 1: decoder mode
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const MODE_OFFSET: u32 = 31;
    pub const MODE_MASK: u32 = 0x80000000;
    pub const MODE: u32 = 1 << 31;

    /// **MEM_FORCE_PU**
    ///
    /// 0: no operation,1:force jpeg memory to power up
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const MEM_FORCE_PU_OFFSET: u32 = 30;
    pub const MEM_FORCE_PU_MASK: u32 = 0x40000000;
    pub const MEM_FORCE_PU: u32 = 1 << 30;

    /// **MEM_FORCE_PD**
    ///
    /// 0: no operation,1:force jpeg memory to power down
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const MEM_FORCE_PD_OFFSET: u32 = 29;
    pub const MEM_FORCE_PD_MASK: u32 = 0x20000000;
    pub const MEM_FORCE_PD: u32 = 1 << 29;

    /// **PAUSE_EN**
    ///
    /// set this bit to pause jpeg encoding
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const PAUSE_EN_OFFSET: u32 = 28;
    pub const PAUSE_EN_MASK: u32 = 0x10000000;
    pub const PAUSE_EN: u32 = 1 << 28;

    /// **TAILER_EN**
    ///
    /// set this bit to add EOI of "0xffd9" at the end of bitstream
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const TAILER_EN_OFFSET: u32 = 27;
    pub const TAILER_EN_MASK: u32 = 0x08000000;
    pub const TAILER_EN: u32 = 1 << 27;

    /// **PIXEL_REV**
    ///
    /// reverse the source color pixel
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const PIXEL_REV_OFFSET: u32 = 26;
    pub const PIXEL_REV_MASK: u32 = 0x04000000;
    pub const PIXEL_REV: u32 = 1 << 26;

    /// **FIFO_RST**
    ///
    /// fifo reset
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const FIFO_RST_OFFSET: u32 = 25;
    pub const FIFO_RST_MASK: u32 = 0x02000000;
    pub const FIFO_RST: u32 = 1 << 25;

    /// **SOFT_RST**
    ///
    /// when set to 1, soft reset JPEG module except jpeg_reg module
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SOFT_RST_OFFSET: u32 = 24;
    pub const SOFT_RST_MASK: u32 = 0x01000000;
    pub const SOFT_RST: u32 = 1 << 24;

    /// **DECODE_TIMEOUT_TASK_SEL**
    ///
    /// 0: software use reset to abort decode process ,1: decoder abort decode process by itself
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const DECODE_TIMEOUT_TASK_SEL_OFFSET: u32 = 23;
    pub const DECODE_TIMEOUT_TASK_SEL_MASK: u32 = 0x00800000;
    pub const DECODE_TIMEOUT_TASK_SEL: u32 = 1 << 23;

    /// **JFIF_VER**
    ///
    /// decode pause period to trigger decode_timeout int, the timeout periods =2 power (reg_decode_timeout_thres) -1
    ///
    /// Bits `[22:17]`, mask `0x007e0000`
    pub const JFIF_VER_OFFSET: u32 = 17;
    pub const JFIF_VER_MASK: u32 = 0x007e0000;

    /// **MEM_CLK_FORCE_ON**
    ///
    /// force memory's clock enabled
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const MEM_CLK_FORCE_ON_OFFSET: u32 = 16;
    pub const MEM_CLK_FORCE_ON_MASK: u32 = 0x00010000;
    pub const MEM_CLK_FORCE_ON: u32 = 1 << 16;

    /// **DHT_FIFO_EN**
    ///
    /// 0:use non-fifo way to write dht len_total/codemin/value table,1:use fifo way to write dht len_total/codemin/value table. Reading dht len_total/codemin/value table only has nonfifo way
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DHT_FIFO_EN_OFFSET: u32 = 15;
    pub const DHT_FIFO_EN_MASK: u32 = 0x00008000;
    pub const DHT_FIFO_EN: u32 = 1 << 15;

    /// **COLOR_SPACE**
    ///
    /// configure picture's color space:0-rb888,1-yuv422,2-rgb565, 3-gray
    ///
    /// Bits `[14:13]`, mask `0x00006000`
    pub const COLOR_SPACE_OFFSET: u32 = 13;
    pub const COLOR_SPACE_MASK: u32 = 0x00006000;

    /// **CQNR_TBL_SEL**
    ///
    /// choose chrominance quntization table id (TBD)
    ///
    /// Bits `[12:11]`, mask `0x00001800`
    pub const CQNR_TBL_SEL_OFFSET: u32 = 11;
    pub const CQNR_TBL_SEL_MASK: u32 = 0x00001800;

    /// **LQNR_TBL_SEL**
    ///
    /// choose luminance quntization table id(TBD)
    ///
    /// Bits `[10:9]`, mask `0x00000600`
    pub const LQNR_TBL_SEL_OFFSET: u32 = 9;
    pub const LQNR_TBL_SEL_MASK: u32 = 0x00000600;

    /// **GRAY_SEL**
    ///
    /// 0:use non-fifo way to access qnr ram,1:use fifo way to access qnr ram
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const GRAY_SEL_OFFSET: u32 = 8;
    pub const GRAY_SEL_MASK: u32 = 0x00000100;
    pub const GRAY_SEL: u32 = 1 << 8;

    /// **DEBUG_DIRECT_OUT_EN**
    ///
    /// 0:normal mode,1:debug mode for direct output from input
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DEBUG_DIRECT_OUT_EN_OFFSET: u32 = 7;
    pub const DEBUG_DIRECT_OUT_EN_MASK: u32 = 0x00000080;
    pub const DEBUG_DIRECT_OUT_EN: u32 = 1 << 7;

    /// **DMA_LINKLIST_MODE**
    ///
    /// 1:use linklist to configure dma
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMA_LINKLIST_MODE_OFFSET: u32 = 6;
    pub const DMA_LINKLIST_MODE_MASK: u32 = 0x00000040;
    pub const DMA_LINKLIST_MODE: u32 = 1 << 6;

    /// **SAMPLE_SEL**
    ///
    /// 0:yuv444,1:yuv422, 2:yuv420
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const SAMPLE_SEL_OFFSET: u32 = 4;
    pub const SAMPLE_SEL_MASK: u32 = 0x00000030;

    /// **FF_CHECK_EN**
    ///
    /// enable whether to add "00" after "ff"
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const FF_CHECK_EN_OFFSET: u32 = 3;
    pub const FF_CHECK_EN_MASK: u32 = 0x00000008;
    pub const FF_CHECK_EN: u32 = 1 << 3;

    /// **QNR_PRESITION**
    ///
    /// 0:8bit qnr,1:12bit qnr(TBD)
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const QNR_PRESITION_OFFSET: u32 = 2;
    pub const QNR_PRESITION_MASK: u32 = 0x00000004;
    pub const QNR_PRESITION: u32 = 1 << 2;

    /// **JPEG_START**
    ///
    /// start to compress a new pic(in dma reg mode)
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const JPEG_START_OFFSET: u32 = 1;
    pub const JPEG_START_MASK: u32 = 0x00000002;
    pub const JPEG_START: u32 = 1 << 1;

    /// **FSM_RST**
    ///
    /// fsm reset
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FSM_RST_OFFSET: u32 = 0;
    pub const FSM_RST_MASK: u32 = 0x00000001;
    pub const FSM_RST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MODE** `[31]` (RW)
    ///
    /// 0:encoder mode, 1: decoder mode
    #[inline(always)]
    pub fn mode(&self) -> bool {
        (self.value.read_bits() & Self::MODE_MASK) != 0
    }

    /// Set MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mode(&self) {
        self.value.modify_bits(|r| r | Self::MODE_MASK);
    }

    /// Clear MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mode(&self) {
        self.value.modify_bits(|r| r & !Self::MODE_MASK);
    }

    /// **MEM_FORCE_PU** `[30]` (RW)
    ///
    /// 0: no operation,1:force jpeg memory to power up
    #[inline(always)]
    pub fn mem_force_pu(&self) -> bool {
        (self.value.read_bits() & Self::MEM_FORCE_PU_MASK) != 0
    }

    /// Set MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_force_pu(&self) {
        self.value.modify_bits(|r| r | Self::MEM_FORCE_PU_MASK);
    }

    /// Clear MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_force_pu(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_FORCE_PU_MASK);
    }

    /// **MEM_FORCE_PD** `[29]` (RW)
    ///
    /// 0: no operation,1:force jpeg memory to power down
    #[inline(always)]
    pub fn mem_force_pd(&self) -> bool {
        (self.value.read_bits() & Self::MEM_FORCE_PD_MASK) != 0
    }

    /// Set MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_force_pd(&self) {
        self.value.modify_bits(|r| r | Self::MEM_FORCE_PD_MASK);
    }

    /// Clear MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_force_pd(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_FORCE_PD_MASK);
    }

    /// **PAUSE_EN** `[28]` (RW)
    ///
    /// set this bit to pause jpeg encoding
    #[inline(always)]
    pub fn pause_en(&self) -> bool {
        (self.value.read_bits() & Self::PAUSE_EN_MASK) != 0
    }

    /// Set PAUSE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pause_en(&self) {
        self.value.modify_bits(|r| r | Self::PAUSE_EN_MASK);
    }

    /// Clear PAUSE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pause_en(&self) {
        self.value.modify_bits(|r| r & !Self::PAUSE_EN_MASK);
    }

    /// **TAILER_EN** `[27]` (RW)
    ///
    /// set this bit to add EOI of "0xffd9" at the end of bitstream
    #[inline(always)]
    pub fn tailer_en(&self) -> bool {
        (self.value.read_bits() & Self::TAILER_EN_MASK) != 0
    }

    /// Set TAILER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tailer_en(&self) {
        self.value.modify_bits(|r| r | Self::TAILER_EN_MASK);
    }

    /// Clear TAILER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tailer_en(&self) {
        self.value.modify_bits(|r| r & !Self::TAILER_EN_MASK);
    }

    /// **PIXEL_REV** `[26]` (RW)
    ///
    /// reverse the source color pixel
    #[inline(always)]
    pub fn pixel_rev(&self) -> bool {
        (self.value.read_bits() & Self::PIXEL_REV_MASK) != 0
    }

    /// Set PIXEL_REV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pixel_rev(&self) {
        self.value.modify_bits(|r| r | Self::PIXEL_REV_MASK);
    }

    /// Clear PIXEL_REV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pixel_rev(&self) {
        self.value.modify_bits(|r| r & !Self::PIXEL_REV_MASK);
    }

    /// **FIFO_RST** `[25]` (RW)
    ///
    /// fifo reset
    #[inline(always)]
    pub fn fifo_rst(&self) -> bool {
        (self.value.read_bits() & Self::FIFO_RST_MASK) != 0
    }

    /// Set FIFO_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fifo_rst(&self) {
        self.value.modify_bits(|r| r | Self::FIFO_RST_MASK);
    }

    /// Clear FIFO_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fifo_rst(&self) {
        self.value.modify_bits(|r| r & !Self::FIFO_RST_MASK);
    }

    /// **SOFT_RST** `[24]` (RW)
    ///
    /// when set to 1, soft reset JPEG module except jpeg_reg module
    #[inline(always)]
    pub fn soft_rst(&self) -> bool {
        (self.value.read_bits() & Self::SOFT_RST_MASK) != 0
    }

    /// Set SOFT_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_soft_rst(&self) {
        self.value.modify_bits(|r| r | Self::SOFT_RST_MASK);
    }

    /// Clear SOFT_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_soft_rst(&self) {
        self.value.modify_bits(|r| r & !Self::SOFT_RST_MASK);
    }

    /// **DECODE_TIMEOUT_TASK_SEL** `[23]` (RW)
    ///
    /// 0: software use reset to abort decode process ,1: decoder abort decode process by itself
    #[inline(always)]
    pub fn decode_timeout_task_sel(&self) -> bool {
        (self.value.read_bits() & Self::DECODE_TIMEOUT_TASK_SEL_MASK) != 0
    }

    /// Set DECODE_TIMEOUT_TASK_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_decode_timeout_task_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::DECODE_TIMEOUT_TASK_SEL_MASK);
    }

    /// Clear DECODE_TIMEOUT_TASK_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_decode_timeout_task_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::DECODE_TIMEOUT_TASK_SEL_MASK);
    }

    /// **JFIF_VER** `[22:17]` (RW)
    ///
    /// decode pause period to trigger decode_timeout int, the timeout periods =2 power (reg_decode_timeout_thres) -1
    #[inline(always)]
    pub fn jfif_ver(&self) -> u32 {
        (self.value.read_bits() & Self::JFIF_VER_MASK) >> Self::JFIF_VER_OFFSET
    }

    /// Write **JFIF_VER** `[22:17]` (RW)
    ///
    /// decode pause period to trigger decode_timeout int, the timeout periods =2 power (reg_decode_timeout_thres) -1
    #[inline(always)]
    pub fn write_jfif_ver(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::JFIF_VER_MASK) | ((value << Self::JFIF_VER_OFFSET) & Self::JFIF_VER_MASK)
        });
    }

    /// **MEM_CLK_FORCE_ON** `[16]` (RW)
    ///
    /// force memory's clock enabled
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_clk_force_on(&self) {
        self.value.modify_bits(|r| r | Self::MEM_CLK_FORCE_ON_MASK);
    }

    /// Clear MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_clk_force_on(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_CLK_FORCE_ON_MASK);
    }

    /// **DHT_FIFO_EN** `[15]` (RW)
    ///
    /// 0:use non-fifo way to write dht len_total/codemin/value table,1:use fifo way to write dht len_total/codemin/value table. Reading dht len_total/codemin/value table only has nonfifo way
    #[inline(always)]
    pub fn dht_fifo_en(&self) -> bool {
        (self.value.read_bits() & Self::DHT_FIFO_EN_MASK) != 0
    }

    /// Set DHT_FIFO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dht_fifo_en(&self) {
        self.value.modify_bits(|r| r | Self::DHT_FIFO_EN_MASK);
    }

    /// Clear DHT_FIFO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dht_fifo_en(&self) {
        self.value.modify_bits(|r| r & !Self::DHT_FIFO_EN_MASK);
    }

    /// **COLOR_SPACE** `[14:13]` (RW)
    ///
    /// configure picture's color space:0-rb888,1-yuv422,2-rgb565, 3-gray
    #[inline(always)]
    pub fn color_space(&self) -> u32 {
        (self.value.read_bits() & Self::COLOR_SPACE_MASK) >> Self::COLOR_SPACE_OFFSET
    }

    /// Write **COLOR_SPACE** `[14:13]` (RW)
    ///
    /// configure picture's color space:0-rb888,1-yuv422,2-rgb565, 3-gray
    #[inline(always)]
    pub fn write_color_space(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COLOR_SPACE_MASK)
                | ((value << Self::COLOR_SPACE_OFFSET) & Self::COLOR_SPACE_MASK)
        });
    }

    /// **CQNR_TBL_SEL** `[12:11]` (RW)
    ///
    /// choose chrominance quntization table id (TBD)
    #[inline(always)]
    pub fn cqnr_tbl_sel(&self) -> u32 {
        (self.value.read_bits() & Self::CQNR_TBL_SEL_MASK) >> Self::CQNR_TBL_SEL_OFFSET
    }

    /// Write **CQNR_TBL_SEL** `[12:11]` (RW)
    ///
    /// choose chrominance quntization table id (TBD)
    #[inline(always)]
    pub fn write_cqnr_tbl_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CQNR_TBL_SEL_MASK)
                | ((value << Self::CQNR_TBL_SEL_OFFSET) & Self::CQNR_TBL_SEL_MASK)
        });
    }

    /// **LQNR_TBL_SEL** `[10:9]` (RW)
    ///
    /// choose luminance quntization table id(TBD)
    #[inline(always)]
    pub fn lqnr_tbl_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LQNR_TBL_SEL_MASK) >> Self::LQNR_TBL_SEL_OFFSET
    }

    /// Write **LQNR_TBL_SEL** `[10:9]` (RW)
    ///
    /// choose luminance quntization table id(TBD)
    #[inline(always)]
    pub fn write_lqnr_tbl_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LQNR_TBL_SEL_MASK)
                | ((value << Self::LQNR_TBL_SEL_OFFSET) & Self::LQNR_TBL_SEL_MASK)
        });
    }

    /// **GRAY_SEL** `[8]` (RW)
    ///
    /// 0:use non-fifo way to access qnr ram,1:use fifo way to access qnr ram
    #[inline(always)]
    pub fn gray_sel(&self) -> bool {
        (self.value.read_bits() & Self::GRAY_SEL_MASK) != 0
    }

    /// Set GRAY_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gray_sel(&self) {
        self.value.modify_bits(|r| r | Self::GRAY_SEL_MASK);
    }

    /// Clear GRAY_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gray_sel(&self) {
        self.value.modify_bits(|r| r & !Self::GRAY_SEL_MASK);
    }

    /// **DEBUG_DIRECT_OUT_EN** `[7]` (RW)
    ///
    /// 0:normal mode,1:debug mode for direct output from input
    #[inline(always)]
    pub fn debug_direct_out_en(&self) -> bool {
        (self.value.read_bits() & Self::DEBUG_DIRECT_OUT_EN_MASK) != 0
    }

    /// Set DEBUG_DIRECT_OUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_debug_direct_out_en(&self) {
        self.value
            .modify_bits(|r| r | Self::DEBUG_DIRECT_OUT_EN_MASK);
    }

    /// Clear DEBUG_DIRECT_OUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_debug_direct_out_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::DEBUG_DIRECT_OUT_EN_MASK);
    }

    /// **DMA_LINKLIST_MODE** `[6]` (RO)
    ///
    /// 1:use linklist to configure dma
    #[inline(always)]
    pub fn dma_linklist_mode(&self) -> bool {
        (self.value.read_bits() & Self::DMA_LINKLIST_MODE_MASK) != 0
    }

    /// **SAMPLE_SEL** `[5:4]` (RW)
    ///
    /// 0:yuv444,1:yuv422, 2:yuv420
    #[inline(always)]
    pub fn sample_sel(&self) -> u32 {
        (self.value.read_bits() & Self::SAMPLE_SEL_MASK) >> Self::SAMPLE_SEL_OFFSET
    }

    /// Write **SAMPLE_SEL** `[5:4]` (RW)
    ///
    /// 0:yuv444,1:yuv422, 2:yuv420
    #[inline(always)]
    pub fn write_sample_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAMPLE_SEL_MASK)
                | ((value << Self::SAMPLE_SEL_OFFSET) & Self::SAMPLE_SEL_MASK)
        });
    }

    /// **FF_CHECK_EN** `[3]` (RW)
    ///
    /// enable whether to add "00" after "ff"
    #[inline(always)]
    pub fn ff_check_en(&self) -> bool {
        (self.value.read_bits() & Self::FF_CHECK_EN_MASK) != 0
    }

    /// Set FF_CHECK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ff_check_en(&self) {
        self.value.modify_bits(|r| r | Self::FF_CHECK_EN_MASK);
    }

    /// Clear FF_CHECK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ff_check_en(&self) {
        self.value.modify_bits(|r| r & !Self::FF_CHECK_EN_MASK);
    }

    /// **QNR_PRESITION** `[2]` (RW)
    ///
    /// 0:8bit qnr,1:12bit qnr(TBD)
    #[inline(always)]
    pub fn qnr_presition(&self) -> bool {
        (self.value.read_bits() & Self::QNR_PRESITION_MASK) != 0
    }

    /// Set QNR_PRESITION bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_qnr_presition(&self) {
        self.value.modify_bits(|r| r | Self::QNR_PRESITION_MASK);
    }

    /// Clear QNR_PRESITION bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_qnr_presition(&self) {
        self.value.modify_bits(|r| r & !Self::QNR_PRESITION_MASK);
    }

    /// Trigger **JPEG_START** (WO)
    ///
    /// start to compress a new pic(in dma reg mode)
    #[inline(always)]
    pub fn trigger_jpeg_start(&self) {
        self.value.modify_bits(|r| r | Self::JPEG_START_MASK);
    }

    /// Trigger **FSM_RST** (WO)
    ///
    /// fsm reset
    #[inline(always)]
    pub fn trigger_fsm_rst(&self) {
        self.value.modify_bits(|r| r | Self::FSM_RST_MASK);
    }
}

/// CONFIG value type for batch read/modify/write operations
///
/// Use `config.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `config.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ConfigVal {
    bits: u32,
}

impl ConfigVal {
    /// Create from raw bits
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
        Self { bits: 0x00408958 }
    }

    /// Read **MODE** `[31:31]`
    ///
    /// 0:encoder mode, 1: decoder mode
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        (self.bits & Config::MODE_MASK) != 0
    }

    /// Read **MEM_FORCE_PU** `[30:30]`
    ///
    /// 0: no operation,1:force jpeg memory to power up
    #[inline(always)]
    pub const fn mem_force_pu(&self) -> bool {
        (self.bits & Config::MEM_FORCE_PU_MASK) != 0
    }

    /// Read **MEM_FORCE_PD** `[29:29]`
    ///
    /// 0: no operation,1:force jpeg memory to power down
    #[inline(always)]
    pub const fn mem_force_pd(&self) -> bool {
        (self.bits & Config::MEM_FORCE_PD_MASK) != 0
    }

    /// Read **PAUSE_EN** `[28:28]`
    ///
    /// set this bit to pause jpeg encoding
    #[inline(always)]
    pub const fn pause_en(&self) -> bool {
        (self.bits & Config::PAUSE_EN_MASK) != 0
    }

    /// Read **TAILER_EN** `[27:27]`
    ///
    /// set this bit to add EOI of "0xffd9" at the end of bitstream
    #[inline(always)]
    pub const fn tailer_en(&self) -> bool {
        (self.bits & Config::TAILER_EN_MASK) != 0
    }

    /// Read **PIXEL_REV** `[26:26]`
    ///
    /// reverse the source color pixel
    #[inline(always)]
    pub const fn pixel_rev(&self) -> bool {
        (self.bits & Config::PIXEL_REV_MASK) != 0
    }

    /// Read **FIFO_RST** `[25:25]`
    ///
    /// fifo reset
    #[inline(always)]
    pub const fn fifo_rst(&self) -> bool {
        (self.bits & Config::FIFO_RST_MASK) != 0
    }

    /// Read **SOFT_RST** `[24:24]`
    ///
    /// when set to 1, soft reset JPEG module except jpeg_reg module
    #[inline(always)]
    pub const fn soft_rst(&self) -> bool {
        (self.bits & Config::SOFT_RST_MASK) != 0
    }

    /// Read **DECODE_TIMEOUT_TASK_SEL** `[23:23]`
    ///
    /// 0: software use reset to abort decode process ,1: decoder abort decode process by itself
    #[inline(always)]
    pub const fn decode_timeout_task_sel(&self) -> bool {
        (self.bits & Config::DECODE_TIMEOUT_TASK_SEL_MASK) != 0
    }

    /// Read **JFIF_VER** `[22:17]`
    ///
    /// decode pause period to trigger decode_timeout int, the timeout periods =2 power (reg_decode_timeout_thres) -1
    #[inline(always)]
    pub const fn jfif_ver(&self) -> u32 {
        (self.bits & Config::JFIF_VER_MASK) >> Config::JFIF_VER_OFFSET
    }

    /// Read **MEM_CLK_FORCE_ON** `[16:16]`
    ///
    /// force memory's clock enabled
    #[inline(always)]
    pub const fn mem_clk_force_on(&self) -> bool {
        (self.bits & Config::MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **DHT_FIFO_EN** `[15:15]`
    ///
    /// 0:use non-fifo way to write dht len_total/codemin/value table,1:use fifo way to write dht len_total/codemin/value table. Reading dht len_total/codemin/value table only has nonfifo way
    #[inline(always)]
    pub const fn dht_fifo_en(&self) -> bool {
        (self.bits & Config::DHT_FIFO_EN_MASK) != 0
    }

    /// Read **COLOR_SPACE** `[14:13]`
    ///
    /// configure picture's color space:0-rb888,1-yuv422,2-rgb565, 3-gray
    #[inline(always)]
    pub const fn color_space(&self) -> u32 {
        (self.bits & Config::COLOR_SPACE_MASK) >> Config::COLOR_SPACE_OFFSET
    }

    /// Read **CQNR_TBL_SEL** `[12:11]`
    ///
    /// choose chrominance quntization table id (TBD)
    #[inline(always)]
    pub const fn cqnr_tbl_sel(&self) -> u32 {
        (self.bits & Config::CQNR_TBL_SEL_MASK) >> Config::CQNR_TBL_SEL_OFFSET
    }

    /// Read **LQNR_TBL_SEL** `[10:9]`
    ///
    /// choose luminance quntization table id(TBD)
    #[inline(always)]
    pub const fn lqnr_tbl_sel(&self) -> u32 {
        (self.bits & Config::LQNR_TBL_SEL_MASK) >> Config::LQNR_TBL_SEL_OFFSET
    }

    /// Read **GRAY_SEL** `[8:8]`
    ///
    /// 0:use non-fifo way to access qnr ram,1:use fifo way to access qnr ram
    #[inline(always)]
    pub const fn gray_sel(&self) -> bool {
        (self.bits & Config::GRAY_SEL_MASK) != 0
    }

    /// Read **DEBUG_DIRECT_OUT_EN** `[7:7]`
    ///
    /// 0:normal mode,1:debug mode for direct output from input
    #[inline(always)]
    pub const fn debug_direct_out_en(&self) -> bool {
        (self.bits & Config::DEBUG_DIRECT_OUT_EN_MASK) != 0
    }

    /// Read **DMA_LINKLIST_MODE** `[6:6]`
    ///
    /// 1:use linklist to configure dma
    #[inline(always)]
    pub const fn dma_linklist_mode(&self) -> bool {
        (self.bits & Config::DMA_LINKLIST_MODE_MASK) != 0
    }

    /// Read **SAMPLE_SEL** `[5:4]`
    ///
    /// 0:yuv444,1:yuv422, 2:yuv420
    #[inline(always)]
    pub const fn sample_sel(&self) -> u32 {
        (self.bits & Config::SAMPLE_SEL_MASK) >> Config::SAMPLE_SEL_OFFSET
    }

    /// Read **FF_CHECK_EN** `[3:3]`
    ///
    /// enable whether to add "00" after "ff"
    #[inline(always)]
    pub const fn ff_check_en(&self) -> bool {
        (self.bits & Config::FF_CHECK_EN_MASK) != 0
    }

    /// Read **QNR_PRESITION** `[2:2]`
    ///
    /// 0:8bit qnr,1:12bit qnr(TBD)
    #[inline(always)]
    pub const fn qnr_presition(&self) -> bool {
        (self.bits & Config::QNR_PRESITION_MASK) != 0
    }

    /// Set **MODE** `[31:31]`
    ///
    /// 0:encoder mode, 1: decoder mode
    #[inline(always)]
    pub const fn with_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::MODE_MASK
            } else {
                self.bits & !Config::MODE_MASK
            },
        }
    }

    /// Set **MEM_FORCE_PU** `[30:30]`
    ///
    /// 0: no operation,1:force jpeg memory to power up
    #[inline(always)]
    pub const fn with_mem_force_pu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::MEM_FORCE_PU_MASK
            } else {
                self.bits & !Config::MEM_FORCE_PU_MASK
            },
        }
    }

    /// Set **MEM_FORCE_PD** `[29:29]`
    ///
    /// 0: no operation,1:force jpeg memory to power down
    #[inline(always)]
    pub const fn with_mem_force_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::MEM_FORCE_PD_MASK
            } else {
                self.bits & !Config::MEM_FORCE_PD_MASK
            },
        }
    }

    /// Set **PAUSE_EN** `[28:28]`
    ///
    /// set this bit to pause jpeg encoding
    #[inline(always)]
    pub const fn with_pause_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::PAUSE_EN_MASK
            } else {
                self.bits & !Config::PAUSE_EN_MASK
            },
        }
    }

    /// Set **TAILER_EN** `[27:27]`
    ///
    /// set this bit to add EOI of "0xffd9" at the end of bitstream
    #[inline(always)]
    pub const fn with_tailer_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::TAILER_EN_MASK
            } else {
                self.bits & !Config::TAILER_EN_MASK
            },
        }
    }

    /// Set **PIXEL_REV** `[26:26]`
    ///
    /// reverse the source color pixel
    #[inline(always)]
    pub const fn with_pixel_rev(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::PIXEL_REV_MASK
            } else {
                self.bits & !Config::PIXEL_REV_MASK
            },
        }
    }

    /// Set **FIFO_RST** `[25:25]`
    ///
    /// fifo reset
    #[inline(always)]
    pub const fn with_fifo_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::FIFO_RST_MASK
            } else {
                self.bits & !Config::FIFO_RST_MASK
            },
        }
    }

    /// Set **SOFT_RST** `[24:24]`
    ///
    /// when set to 1, soft reset JPEG module except jpeg_reg module
    #[inline(always)]
    pub const fn with_soft_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::SOFT_RST_MASK
            } else {
                self.bits & !Config::SOFT_RST_MASK
            },
        }
    }

    /// Set **DECODE_TIMEOUT_TASK_SEL** `[23:23]`
    ///
    /// 0: software use reset to abort decode process ,1: decoder abort decode process by itself
    #[inline(always)]
    pub const fn with_decode_timeout_task_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::DECODE_TIMEOUT_TASK_SEL_MASK
            } else {
                self.bits & !Config::DECODE_TIMEOUT_TASK_SEL_MASK
            },
        }
    }

    /// Set **JFIF_VER** `[22:17]`
    ///
    /// decode pause period to trigger decode_timeout int, the timeout periods =2 power (reg_decode_timeout_thres) -1
    #[inline(always)]
    pub const fn with_jfif_ver(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config::JFIF_VER_MASK)
                | ((value << Config::JFIF_VER_OFFSET) & Config::JFIF_VER_MASK),
        }
    }

    /// Set **MEM_CLK_FORCE_ON** `[16:16]`
    ///
    /// force memory's clock enabled
    #[inline(always)]
    pub const fn with_mem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::MEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !Config::MEM_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **DHT_FIFO_EN** `[15:15]`
    ///
    /// 0:use non-fifo way to write dht len_total/codemin/value table,1:use fifo way to write dht len_total/codemin/value table. Reading dht len_total/codemin/value table only has nonfifo way
    #[inline(always)]
    pub const fn with_dht_fifo_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::DHT_FIFO_EN_MASK
            } else {
                self.bits & !Config::DHT_FIFO_EN_MASK
            },
        }
    }

    /// Set **COLOR_SPACE** `[14:13]`
    ///
    /// configure picture's color space:0-rb888,1-yuv422,2-rgb565, 3-gray
    #[inline(always)]
    pub const fn with_color_space(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config::COLOR_SPACE_MASK)
                | ((value << Config::COLOR_SPACE_OFFSET) & Config::COLOR_SPACE_MASK),
        }
    }

    /// Set **CQNR_TBL_SEL** `[12:11]`
    ///
    /// choose chrominance quntization table id (TBD)
    #[inline(always)]
    pub const fn with_cqnr_tbl_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config::CQNR_TBL_SEL_MASK)
                | ((value << Config::CQNR_TBL_SEL_OFFSET) & Config::CQNR_TBL_SEL_MASK),
        }
    }

    /// Set **LQNR_TBL_SEL** `[10:9]`
    ///
    /// choose luminance quntization table id(TBD)
    #[inline(always)]
    pub const fn with_lqnr_tbl_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config::LQNR_TBL_SEL_MASK)
                | ((value << Config::LQNR_TBL_SEL_OFFSET) & Config::LQNR_TBL_SEL_MASK),
        }
    }

    /// Set **GRAY_SEL** `[8:8]`
    ///
    /// 0:use non-fifo way to access qnr ram,1:use fifo way to access qnr ram
    #[inline(always)]
    pub const fn with_gray_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::GRAY_SEL_MASK
            } else {
                self.bits & !Config::GRAY_SEL_MASK
            },
        }
    }

    /// Set **DEBUG_DIRECT_OUT_EN** `[7:7]`
    ///
    /// 0:normal mode,1:debug mode for direct output from input
    #[inline(always)]
    pub const fn with_debug_direct_out_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::DEBUG_DIRECT_OUT_EN_MASK
            } else {
                self.bits & !Config::DEBUG_DIRECT_OUT_EN_MASK
            },
        }
    }

    /// Set **SAMPLE_SEL** `[5:4]`
    ///
    /// 0:yuv444,1:yuv422, 2:yuv420
    #[inline(always)]
    pub const fn with_sample_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Config::SAMPLE_SEL_MASK)
                | ((value << Config::SAMPLE_SEL_OFFSET) & Config::SAMPLE_SEL_MASK),
        }
    }

    /// Set **FF_CHECK_EN** `[3:3]`
    ///
    /// enable whether to add "00" after "ff"
    #[inline(always)]
    pub const fn with_ff_check_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::FF_CHECK_EN_MASK
            } else {
                self.bits & !Config::FF_CHECK_EN_MASK
            },
        }
    }

    /// Set **QNR_PRESITION** `[2:2]`
    ///
    /// 0:8bit qnr,1:12bit qnr(TBD)
    #[inline(always)]
    pub const fn with_qnr_presition(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::QNR_PRESITION_MASK
            } else {
                self.bits & !Config::QNR_PRESITION_MASK
            },
        }
    }

    /// Set **JPEG_START** `[1:1]`
    ///
    /// start to compress a new pic(in dma reg mode)
    #[inline(always)]
    pub const fn with_jpeg_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::JPEG_START_MASK
            } else {
                self.bits & !Config::JPEG_START_MASK
            },
        }
    }

    /// Set **FSM_RST** `[0:0]`
    ///
    /// fsm reset
    #[inline(always)]
    pub const fn with_fsm_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Config::FSM_RST_MASK
            } else {
                self.bits & !Config::FSM_RST_MASK
            },
        }
    }
}

impl Default for ConfigVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Config {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ConfigVal {
        ConfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ConfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ConfigVal) -> ConfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DQT_INFO Register
///
/// Control and configuration registers
///
/// Address offset: 0x0004
/// Reset value: 0x03020100
///
/// ## Fields
///
/// - **T3_DQT_INFO** `[31:24]` (RW)
///
///   Configure dqt table3's quantization coefficient precision in bit[7:4], configure dqt table3's table id in bit[3:0]
///
/// - **T2_DQT_INFO** `[23:16]` (RW)
///
///   Configure dqt table2's quantization coefficient precision in bit[7:4], configure dqt table2's table id in bit[3:0]
///
/// - **T1_DQT_INFO** `[15:8]` (RW)
///
///   Configure dqt table1's quantization coefficient precision in bit[7:4], configure dqt table1's table id in bit[3:0]
///
/// - **T0_DQT_INFO** `[7:0]` (RW)
///
///   Configure dqt table0's quantization coefficient precision in bit[7:4], configure dqt table0's table id in bit[3:0]
#[repr(transparent)]
pub struct DqtInfo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DqtInfo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DqtInfo {
    // ======== Field Constants ========

    /// **T3_DQT_INFO**
    ///
    /// Configure dqt table3's quantization coefficient precision in bit[7:4], configure dqt table3's table id in bit[3:0]
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const T3_DQT_INFO_OFFSET: u32 = 24;
    pub const T3_DQT_INFO_MASK: u32 = 0xff000000;

    /// **T2_DQT_INFO**
    ///
    /// Configure dqt table2's quantization coefficient precision in bit[7:4], configure dqt table2's table id in bit[3:0]
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const T2_DQT_INFO_OFFSET: u32 = 16;
    pub const T2_DQT_INFO_MASK: u32 = 0x00ff0000;

    /// **T1_DQT_INFO**
    ///
    /// Configure dqt table1's quantization coefficient precision in bit[7:4], configure dqt table1's table id in bit[3:0]
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const T1_DQT_INFO_OFFSET: u32 = 8;
    pub const T1_DQT_INFO_MASK: u32 = 0x0000ff00;

    /// **T0_DQT_INFO**
    ///
    /// Configure dqt table0's quantization coefficient precision in bit[7:4], configure dqt table0's table id in bit[3:0]
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const T0_DQT_INFO_OFFSET: u32 = 0;
    pub const T0_DQT_INFO_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **T3_DQT_INFO** `[31:24]` (RW)
    ///
    /// Configure dqt table3's quantization coefficient precision in bit[7:4], configure dqt table3's table id in bit[3:0]
    #[inline(always)]
    pub fn t3_dqt_info(&self) -> u32 {
        (self.value.read_bits() & Self::T3_DQT_INFO_MASK) >> Self::T3_DQT_INFO_OFFSET
    }

    /// Write **T3_DQT_INFO** `[31:24]` (RW)
    ///
    /// Configure dqt table3's quantization coefficient precision in bit[7:4], configure dqt table3's table id in bit[3:0]
    #[inline(always)]
    pub fn write_t3_dqt_info(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::T3_DQT_INFO_MASK)
                | ((value << Self::T3_DQT_INFO_OFFSET) & Self::T3_DQT_INFO_MASK)
        });
    }

    /// **T2_DQT_INFO** `[23:16]` (RW)
    ///
    /// Configure dqt table2's quantization coefficient precision in bit[7:4], configure dqt table2's table id in bit[3:0]
    #[inline(always)]
    pub fn t2_dqt_info(&self) -> u32 {
        (self.value.read_bits() & Self::T2_DQT_INFO_MASK) >> Self::T2_DQT_INFO_OFFSET
    }

    /// Write **T2_DQT_INFO** `[23:16]` (RW)
    ///
    /// Configure dqt table2's quantization coefficient precision in bit[7:4], configure dqt table2's table id in bit[3:0]
    #[inline(always)]
    pub fn write_t2_dqt_info(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::T2_DQT_INFO_MASK)
                | ((value << Self::T2_DQT_INFO_OFFSET) & Self::T2_DQT_INFO_MASK)
        });
    }

    /// **T1_DQT_INFO** `[15:8]` (RW)
    ///
    /// Configure dqt table1's quantization coefficient precision in bit[7:4], configure dqt table1's table id in bit[3:0]
    #[inline(always)]
    pub fn t1_dqt_info(&self) -> u32 {
        (self.value.read_bits() & Self::T1_DQT_INFO_MASK) >> Self::T1_DQT_INFO_OFFSET
    }

    /// Write **T1_DQT_INFO** `[15:8]` (RW)
    ///
    /// Configure dqt table1's quantization coefficient precision in bit[7:4], configure dqt table1's table id in bit[3:0]
    #[inline(always)]
    pub fn write_t1_dqt_info(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::T1_DQT_INFO_MASK)
                | ((value << Self::T1_DQT_INFO_OFFSET) & Self::T1_DQT_INFO_MASK)
        });
    }

    /// **T0_DQT_INFO** `[7:0]` (RW)
    ///
    /// Configure dqt table0's quantization coefficient precision in bit[7:4], configure dqt table0's table id in bit[3:0]
    #[inline(always)]
    pub fn t0_dqt_info(&self) -> u32 {
        (self.value.read_bits() & Self::T0_DQT_INFO_MASK) >> Self::T0_DQT_INFO_OFFSET
    }

    /// Write **T0_DQT_INFO** `[7:0]` (RW)
    ///
    /// Configure dqt table0's quantization coefficient precision in bit[7:4], configure dqt table0's table id in bit[3:0]
    #[inline(always)]
    pub fn write_t0_dqt_info(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::T0_DQT_INFO_MASK)
                | ((value << Self::T0_DQT_INFO_OFFSET) & Self::T0_DQT_INFO_MASK)
        });
    }
}

/// DQT_INFO value type for batch read/modify/write operations
///
/// Use `dqtinfo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dqtinfo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DqtInfoVal {
    bits: u32,
}

impl DqtInfoVal {
    /// Create from raw bits
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
        Self { bits: 0x03020100 }
    }

    /// Read **T3_DQT_INFO** `[31:24]`
    ///
    /// Configure dqt table3's quantization coefficient precision in bit[7:4], configure dqt table3's table id in bit[3:0]
    #[inline(always)]
    pub const fn t3_dqt_info(&self) -> u32 {
        (self.bits & DqtInfo::T3_DQT_INFO_MASK) >> DqtInfo::T3_DQT_INFO_OFFSET
    }

    /// Read **T2_DQT_INFO** `[23:16]`
    ///
    /// Configure dqt table2's quantization coefficient precision in bit[7:4], configure dqt table2's table id in bit[3:0]
    #[inline(always)]
    pub const fn t2_dqt_info(&self) -> u32 {
        (self.bits & DqtInfo::T2_DQT_INFO_MASK) >> DqtInfo::T2_DQT_INFO_OFFSET
    }

    /// Read **T1_DQT_INFO** `[15:8]`
    ///
    /// Configure dqt table1's quantization coefficient precision in bit[7:4], configure dqt table1's table id in bit[3:0]
    #[inline(always)]
    pub const fn t1_dqt_info(&self) -> u32 {
        (self.bits & DqtInfo::T1_DQT_INFO_MASK) >> DqtInfo::T1_DQT_INFO_OFFSET
    }

    /// Read **T0_DQT_INFO** `[7:0]`
    ///
    /// Configure dqt table0's quantization coefficient precision in bit[7:4], configure dqt table0's table id in bit[3:0]
    #[inline(always)]
    pub const fn t0_dqt_info(&self) -> u32 {
        (self.bits & DqtInfo::T0_DQT_INFO_MASK) >> DqtInfo::T0_DQT_INFO_OFFSET
    }

    /// Set **T3_DQT_INFO** `[31:24]`
    ///
    /// Configure dqt table3's quantization coefficient precision in bit[7:4], configure dqt table3's table id in bit[3:0]
    #[inline(always)]
    pub const fn with_t3_dqt_info(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DqtInfo::T3_DQT_INFO_MASK)
                | ((value << DqtInfo::T3_DQT_INFO_OFFSET) & DqtInfo::T3_DQT_INFO_MASK),
        }
    }

    /// Set **T2_DQT_INFO** `[23:16]`
    ///
    /// Configure dqt table2's quantization coefficient precision in bit[7:4], configure dqt table2's table id in bit[3:0]
    #[inline(always)]
    pub const fn with_t2_dqt_info(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DqtInfo::T2_DQT_INFO_MASK)
                | ((value << DqtInfo::T2_DQT_INFO_OFFSET) & DqtInfo::T2_DQT_INFO_MASK),
        }
    }

    /// Set **T1_DQT_INFO** `[15:8]`
    ///
    /// Configure dqt table1's quantization coefficient precision in bit[7:4], configure dqt table1's table id in bit[3:0]
    #[inline(always)]
    pub const fn with_t1_dqt_info(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DqtInfo::T1_DQT_INFO_MASK)
                | ((value << DqtInfo::T1_DQT_INFO_OFFSET) & DqtInfo::T1_DQT_INFO_MASK),
        }
    }

    /// Set **T0_DQT_INFO** `[7:0]`
    ///
    /// Configure dqt table0's quantization coefficient precision in bit[7:4], configure dqt table0's table id in bit[3:0]
    #[inline(always)]
    pub const fn with_t0_dqt_info(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DqtInfo::T0_DQT_INFO_MASK)
                | ((value << DqtInfo::T0_DQT_INFO_OFFSET) & DqtInfo::T0_DQT_INFO_MASK),
        }
    }
}

impl Default for DqtInfoVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DqtInfo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DqtInfoVal {
        DqtInfoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DqtInfoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DqtInfoVal) -> DqtInfoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIC_SIZE Register
///
/// Control and configuration registers
///
/// Address offset: 0x0008
/// Reset value: 0x028001e0
///
/// ## Fields
///
/// - **HA** `[31:16]` (RW)
///
///   configure picture's width. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
///
/// - **VA** `[15:0]` (RW)
///
///   configure picture's height. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
#[repr(transparent)]
pub struct PicSize {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PicSize {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PicSize {
    // ======== Field Constants ========

    /// **HA**
    ///
    /// configure picture's width. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const HA_OFFSET: u32 = 16;
    pub const HA_MASK: u32 = 0xffff0000;

    /// **VA**
    ///
    /// configure picture's height. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const VA_OFFSET: u32 = 0;
    pub const VA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **HA** `[31:16]` (RW)
    ///
    /// configure picture's width. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
    #[inline(always)]
    pub fn ha(&self) -> u32 {
        (self.value.read_bits() & Self::HA_MASK) >> Self::HA_OFFSET
    }

    /// Write **HA** `[31:16]` (RW)
    ///
    /// configure picture's width. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
    #[inline(always)]
    pub fn write_ha(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::HA_MASK) | ((value << Self::HA_OFFSET) & Self::HA_MASK));
    }

    /// **VA** `[15:0]` (RW)
    ///
    /// configure picture's height. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
    #[inline(always)]
    pub fn va(&self) -> u32 {
        (self.value.read_bits() & Self::VA_MASK) >> Self::VA_OFFSET
    }

    /// Write **VA** `[15:0]` (RW)
    ///
    /// configure picture's height. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
    #[inline(always)]
    pub fn write_va(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::VA_MASK) | ((value << Self::VA_OFFSET) & Self::VA_MASK));
    }
}

/// PIC_SIZE value type for batch read/modify/write operations
///
/// Use `picsize.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `picsize.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PicSizeVal {
    bits: u32,
}

impl PicSizeVal {
    /// Create from raw bits
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
        Self { bits: 0x028001e0 }
    }

    /// Read **HA** `[31:16]`
    ///
    /// configure picture's width. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
    #[inline(always)]
    pub const fn ha(&self) -> u32 {
        (self.bits & PicSize::HA_MASK) >> PicSize::HA_OFFSET
    }

    /// Read **VA** `[15:0]`
    ///
    /// configure picture's height. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
    #[inline(always)]
    pub const fn va(&self) -> u32 {
        (self.bits & PicSize::VA_MASK) >> PicSize::VA_OFFSET
    }

    /// Set **HA** `[31:16]`
    ///
    /// configure picture's width. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
    #[inline(always)]
    pub const fn with_ha(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PicSize::HA_MASK)
                | ((value << PicSize::HA_OFFSET) & PicSize::HA_MASK),
        }
    }

    /// Set **VA** `[15:0]`
    ///
    /// configure picture's height. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
    #[inline(always)]
    pub const fn with_va(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PicSize::VA_MASK)
                | ((value << PicSize::VA_OFFSET) & PicSize::VA_MASK),
        }
    }
}

impl Default for PicSizeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PicSize {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PicSizeVal {
        PicSizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PicSizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PicSizeVal) -> PicSizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T0QNR Register
///
/// Control and configuration registers
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **T0_QNR_VAL** `[31:0]` (RO)
///
///   write this reg to configure 64 quantization coefficient in t0 table
#[repr(transparent)]
pub struct T0qnr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T0qnr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T0qnr {
    // ======== Field Constants ========

    /// **T0_QNR_VAL**
    ///
    /// write this reg to configure 64 quantization coefficient in t0 table
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const T0_QNR_VAL_OFFSET: u32 = 0;
    pub const T0_QNR_VAL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **T0_QNR_VAL** `[31:0]` (RO)
    ///
    /// write this reg to configure 64 quantization coefficient in t0 table
    #[inline(always)]
    pub fn t0_qnr_val(&self) -> u32 {
        (self.value.read_bits() & Self::T0_QNR_VAL_MASK) >> Self::T0_QNR_VAL_OFFSET
    }
}

/// T0QNR value type for batch read/modify/write operations
///
/// Use `t0qnr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t0qnr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T0qnrVal {
    bits: u32,
}

impl T0qnrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **T0_QNR_VAL** `[31:0]`
    ///
    /// write this reg to configure 64 quantization coefficient in t0 table
    #[inline(always)]
    pub const fn t0_qnr_val(&self) -> u32 {
        (self.bits & T0qnr::T0_QNR_VAL_MASK) >> T0qnr::T0_QNR_VAL_OFFSET
    }
}

impl Default for T0qnrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T0qnr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T0qnrVal {
        T0qnrVal::from_bits(self.value.read_bits())
    }
}

/// T1QNR Register
///
/// Control and configuration registers
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **CHROMINANCE_QNR_VAL** `[31:0]` (RO)
///
///   write this reg to configure 64 quantization coefficient in t1 table
#[repr(transparent)]
pub struct T1qnr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T1qnr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T1qnr {
    // ======== Field Constants ========

    /// **CHROMINANCE_QNR_VAL**
    ///
    /// write this reg to configure 64 quantization coefficient in t1 table
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CHROMINANCE_QNR_VAL_OFFSET: u32 = 0;
    pub const CHROMINANCE_QNR_VAL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CHROMINANCE_QNR_VAL** `[31:0]` (RO)
    ///
    /// write this reg to configure 64 quantization coefficient in t1 table
    #[inline(always)]
    pub fn chrominance_qnr_val(&self) -> u32 {
        (self.value.read_bits() & Self::CHROMINANCE_QNR_VAL_MASK)
            >> Self::CHROMINANCE_QNR_VAL_OFFSET
    }
}

/// T1QNR value type for batch read/modify/write operations
///
/// Use `t1qnr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t1qnr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T1qnrVal {
    bits: u32,
}

impl T1qnrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CHROMINANCE_QNR_VAL** `[31:0]`
    ///
    /// write this reg to configure 64 quantization coefficient in t1 table
    #[inline(always)]
    pub const fn chrominance_qnr_val(&self) -> u32 {
        (self.bits & T1qnr::CHROMINANCE_QNR_VAL_MASK) >> T1qnr::CHROMINANCE_QNR_VAL_OFFSET
    }
}

impl Default for T1qnrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T1qnr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T1qnrVal {
        T1qnrVal::from_bits(self.value.read_bits())
    }
}

/// T2QNR Register
///
/// Control and configuration registers
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **T2_QNR_VAL** `[31:0]` (RO)
///
///   write this reg to configure 64 quantization coefficient in t2 table
#[repr(transparent)]
pub struct T2qnr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T2qnr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T2qnr {
    // ======== Field Constants ========

    /// **T2_QNR_VAL**
    ///
    /// write this reg to configure 64 quantization coefficient in t2 table
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const T2_QNR_VAL_OFFSET: u32 = 0;
    pub const T2_QNR_VAL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **T2_QNR_VAL** `[31:0]` (RO)
    ///
    /// write this reg to configure 64 quantization coefficient in t2 table
    #[inline(always)]
    pub fn t2_qnr_val(&self) -> u32 {
        (self.value.read_bits() & Self::T2_QNR_VAL_MASK) >> Self::T2_QNR_VAL_OFFSET
    }
}

/// T2QNR value type for batch read/modify/write operations
///
/// Use `t2qnr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t2qnr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T2qnrVal {
    bits: u32,
}

impl T2qnrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **T2_QNR_VAL** `[31:0]`
    ///
    /// write this reg to configure 64 quantization coefficient in t2 table
    #[inline(always)]
    pub const fn t2_qnr_val(&self) -> u32 {
        (self.bits & T2qnr::T2_QNR_VAL_MASK) >> T2qnr::T2_QNR_VAL_OFFSET
    }
}

impl Default for T2qnrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T2qnr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T2qnrVal {
        T2qnrVal::from_bits(self.value.read_bits())
    }
}

/// T3QNR Register
///
/// Control and configuration registers
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **T3_QNR_VAL** `[31:0]` (RO)
///
///   write this reg to configure 64 quantization coefficient in t3 table
#[repr(transparent)]
pub struct T3qnr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T3qnr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T3qnr {
    // ======== Field Constants ========

    /// **T3_QNR_VAL**
    ///
    /// write this reg to configure 64 quantization coefficient in t3 table
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const T3_QNR_VAL_OFFSET: u32 = 0;
    pub const T3_QNR_VAL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **T3_QNR_VAL** `[31:0]` (RO)
    ///
    /// write this reg to configure 64 quantization coefficient in t3 table
    #[inline(always)]
    pub fn t3_qnr_val(&self) -> u32 {
        (self.value.read_bits() & Self::T3_QNR_VAL_MASK) >> Self::T3_QNR_VAL_OFFSET
    }
}

/// T3QNR value type for batch read/modify/write operations
///
/// Use `t3qnr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t3qnr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T3qnrVal {
    bits: u32,
}

impl T3qnrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **T3_QNR_VAL** `[31:0]`
    ///
    /// write this reg to configure 64 quantization coefficient in t3 table
    #[inline(always)]
    pub const fn t3_qnr_val(&self) -> u32 {
        (self.bits & T3qnr::T3_QNR_VAL_MASK) >> T3qnr::T3_QNR_VAL_OFFSET
    }
}

impl Default for T3qnrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T3qnr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T3qnrVal {
        T3qnrVal::from_bits(self.value.read_bits())
    }
}

/// DECODE_CONF Register
///
/// Control and configuration registers
///
/// Address offset: 0x0020
/// Reset value: 0x5f030000
///
/// ## Fields
///
/// - **DEZIGZAG_READY_CTL** `[30]` (RW)
///
///   reserved for decoder
///
/// - **MULTI_SCAN_ERR_CHECK** `[29]` (RW)
///
///   reserved for decoder
///
/// - **RST_CHECK_BYTE_NUM** `[28:27]` (RW)
///
///   Configure the byte number to check next rst marker after one rst interval is decoded down. The real check number is reg_rst_check_byte_num+1
///
/// - **SOS_CHECK_BYTE_NUM** `[26:25]` (RW)
///
///   Configure the byte number to check next sos marker in the multi-scan picture after one scan is decoded down. The real check number is reg_sos_check_byte_num+1
///
/// - **SW_DHT_EN** `[24]` (RO)
///
///   software decode dht table enable
///
/// - **COMPONENT_NUM** `[23:16]` (RW)
///
///   configure number of components in frame when decode
///
/// - **RESTART_INTERVAL** `[15:0]` (RW)
///
///   configure restart interval in DRI marker when decode
#[repr(transparent)]
pub struct DecodeConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DecodeConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DecodeConf {
    // ======== Field Constants ========

    /// **DEZIGZAG_READY_CTL**
    ///
    /// reserved for decoder
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const DEZIGZAG_READY_CTL_OFFSET: u32 = 30;
    pub const DEZIGZAG_READY_CTL_MASK: u32 = 0x40000000;
    pub const DEZIGZAG_READY_CTL: u32 = 1 << 30;

    /// **MULTI_SCAN_ERR_CHECK**
    ///
    /// reserved for decoder
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const MULTI_SCAN_ERR_CHECK_OFFSET: u32 = 29;
    pub const MULTI_SCAN_ERR_CHECK_MASK: u32 = 0x20000000;
    pub const MULTI_SCAN_ERR_CHECK: u32 = 1 << 29;

    /// **RST_CHECK_BYTE_NUM**
    ///
    /// Configure the byte number to check next rst marker after one rst interval is decoded down. The real check number is reg_rst_check_byte_num+1
    ///
    /// Bits `[28:27]`, mask `0x18000000`
    pub const RST_CHECK_BYTE_NUM_OFFSET: u32 = 27;
    pub const RST_CHECK_BYTE_NUM_MASK: u32 = 0x18000000;

    /// **SOS_CHECK_BYTE_NUM**
    ///
    /// Configure the byte number to check next sos marker in the multi-scan picture after one scan is decoded down. The real check number is reg_sos_check_byte_num+1
    ///
    /// Bits `[26:25]`, mask `0x06000000`
    pub const SOS_CHECK_BYTE_NUM_OFFSET: u32 = 25;
    pub const SOS_CHECK_BYTE_NUM_MASK: u32 = 0x06000000;

    /// **SW_DHT_EN**
    ///
    /// software decode dht table enable
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const SW_DHT_EN_OFFSET: u32 = 24;
    pub const SW_DHT_EN_MASK: u32 = 0x01000000;
    pub const SW_DHT_EN: u32 = 1 << 24;

    /// **COMPONENT_NUM**
    ///
    /// configure number of components in frame when decode
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const COMPONENT_NUM_OFFSET: u32 = 16;
    pub const COMPONENT_NUM_MASK: u32 = 0x00ff0000;

    /// **RESTART_INTERVAL**
    ///
    /// configure restart interval in DRI marker when decode
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const RESTART_INTERVAL_OFFSET: u32 = 0;
    pub const RESTART_INTERVAL_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **DEZIGZAG_READY_CTL** `[30]` (RW)
    ///
    /// reserved for decoder
    #[inline(always)]
    pub fn dezigzag_ready_ctl(&self) -> bool {
        (self.value.read_bits() & Self::DEZIGZAG_READY_CTL_MASK) != 0
    }

    /// Set DEZIGZAG_READY_CTL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dezigzag_ready_ctl(&self) {
        self.value
            .modify_bits(|r| r | Self::DEZIGZAG_READY_CTL_MASK);
    }

    /// Clear DEZIGZAG_READY_CTL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dezigzag_ready_ctl(&self) {
        self.value
            .modify_bits(|r| r & !Self::DEZIGZAG_READY_CTL_MASK);
    }

    /// **MULTI_SCAN_ERR_CHECK** `[29]` (RW)
    ///
    /// reserved for decoder
    #[inline(always)]
    pub fn multi_scan_err_check(&self) -> bool {
        (self.value.read_bits() & Self::MULTI_SCAN_ERR_CHECK_MASK) != 0
    }

    /// Set MULTI_SCAN_ERR_CHECK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_multi_scan_err_check(&self) {
        self.value
            .modify_bits(|r| r | Self::MULTI_SCAN_ERR_CHECK_MASK);
    }

    /// Clear MULTI_SCAN_ERR_CHECK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_multi_scan_err_check(&self) {
        self.value
            .modify_bits(|r| r & !Self::MULTI_SCAN_ERR_CHECK_MASK);
    }

    /// **RST_CHECK_BYTE_NUM** `[28:27]` (RW)
    ///
    /// Configure the byte number to check next rst marker after one rst interval is decoded down. The real check number is reg_rst_check_byte_num+1
    #[inline(always)]
    pub fn rst_check_byte_num(&self) -> u32 {
        (self.value.read_bits() & Self::RST_CHECK_BYTE_NUM_MASK) >> Self::RST_CHECK_BYTE_NUM_OFFSET
    }

    /// Write **RST_CHECK_BYTE_NUM** `[28:27]` (RW)
    ///
    /// Configure the byte number to check next rst marker after one rst interval is decoded down. The real check number is reg_rst_check_byte_num+1
    #[inline(always)]
    pub fn write_rst_check_byte_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RST_CHECK_BYTE_NUM_MASK)
                | ((value << Self::RST_CHECK_BYTE_NUM_OFFSET) & Self::RST_CHECK_BYTE_NUM_MASK)
        });
    }

    /// **SOS_CHECK_BYTE_NUM** `[26:25]` (RW)
    ///
    /// Configure the byte number to check next sos marker in the multi-scan picture after one scan is decoded down. The real check number is reg_sos_check_byte_num+1
    #[inline(always)]
    pub fn sos_check_byte_num(&self) -> u32 {
        (self.value.read_bits() & Self::SOS_CHECK_BYTE_NUM_MASK) >> Self::SOS_CHECK_BYTE_NUM_OFFSET
    }

    /// Write **SOS_CHECK_BYTE_NUM** `[26:25]` (RW)
    ///
    /// Configure the byte number to check next sos marker in the multi-scan picture after one scan is decoded down. The real check number is reg_sos_check_byte_num+1
    #[inline(always)]
    pub fn write_sos_check_byte_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SOS_CHECK_BYTE_NUM_MASK)
                | ((value << Self::SOS_CHECK_BYTE_NUM_OFFSET) & Self::SOS_CHECK_BYTE_NUM_MASK)
        });
    }

    /// **SW_DHT_EN** `[24]` (RO)
    ///
    /// software decode dht table enable
    #[inline(always)]
    pub fn sw_dht_en(&self) -> bool {
        (self.value.read_bits() & Self::SW_DHT_EN_MASK) != 0
    }

    /// **COMPONENT_NUM** `[23:16]` (RW)
    ///
    /// configure number of components in frame when decode
    #[inline(always)]
    pub fn component_num(&self) -> u32 {
        (self.value.read_bits() & Self::COMPONENT_NUM_MASK) >> Self::COMPONENT_NUM_OFFSET
    }

    /// Write **COMPONENT_NUM** `[23:16]` (RW)
    ///
    /// configure number of components in frame when decode
    #[inline(always)]
    pub fn write_component_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::COMPONENT_NUM_MASK)
                | ((value << Self::COMPONENT_NUM_OFFSET) & Self::COMPONENT_NUM_MASK)
        });
    }

    /// **RESTART_INTERVAL** `[15:0]` (RW)
    ///
    /// configure restart interval in DRI marker when decode
    #[inline(always)]
    pub fn restart_interval(&self) -> u32 {
        (self.value.read_bits() & Self::RESTART_INTERVAL_MASK) >> Self::RESTART_INTERVAL_OFFSET
    }

    /// Write **RESTART_INTERVAL** `[15:0]` (RW)
    ///
    /// configure restart interval in DRI marker when decode
    #[inline(always)]
    pub fn write_restart_interval(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RESTART_INTERVAL_MASK)
                | ((value << Self::RESTART_INTERVAL_OFFSET) & Self::RESTART_INTERVAL_MASK)
        });
    }
}

/// DECODE_CONF value type for batch read/modify/write operations
///
/// Use `decodeconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `decodeconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DecodeConfVal {
    bits: u32,
}

impl DecodeConfVal {
    /// Create from raw bits
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
        Self { bits: 0x5f030000 }
    }

    /// Read **DEZIGZAG_READY_CTL** `[30:30]`
    ///
    /// reserved for decoder
    #[inline(always)]
    pub const fn dezigzag_ready_ctl(&self) -> bool {
        (self.bits & DecodeConf::DEZIGZAG_READY_CTL_MASK) != 0
    }

    /// Read **MULTI_SCAN_ERR_CHECK** `[29:29]`
    ///
    /// reserved for decoder
    #[inline(always)]
    pub const fn multi_scan_err_check(&self) -> bool {
        (self.bits & DecodeConf::MULTI_SCAN_ERR_CHECK_MASK) != 0
    }

    /// Read **RST_CHECK_BYTE_NUM** `[28:27]`
    ///
    /// Configure the byte number to check next rst marker after one rst interval is decoded down. The real check number is reg_rst_check_byte_num+1
    #[inline(always)]
    pub const fn rst_check_byte_num(&self) -> u32 {
        (self.bits & DecodeConf::RST_CHECK_BYTE_NUM_MASK) >> DecodeConf::RST_CHECK_BYTE_NUM_OFFSET
    }

    /// Read **SOS_CHECK_BYTE_NUM** `[26:25]`
    ///
    /// Configure the byte number to check next sos marker in the multi-scan picture after one scan is decoded down. The real check number is reg_sos_check_byte_num+1
    #[inline(always)]
    pub const fn sos_check_byte_num(&self) -> u32 {
        (self.bits & DecodeConf::SOS_CHECK_BYTE_NUM_MASK) >> DecodeConf::SOS_CHECK_BYTE_NUM_OFFSET
    }

    /// Read **SW_DHT_EN** `[24:24]`
    ///
    /// software decode dht table enable
    #[inline(always)]
    pub const fn sw_dht_en(&self) -> bool {
        (self.bits & DecodeConf::SW_DHT_EN_MASK) != 0
    }

    /// Read **COMPONENT_NUM** `[23:16]`
    ///
    /// configure number of components in frame when decode
    #[inline(always)]
    pub const fn component_num(&self) -> u32 {
        (self.bits & DecodeConf::COMPONENT_NUM_MASK) >> DecodeConf::COMPONENT_NUM_OFFSET
    }

    /// Read **RESTART_INTERVAL** `[15:0]`
    ///
    /// configure restart interval in DRI marker when decode
    #[inline(always)]
    pub const fn restart_interval(&self) -> u32 {
        (self.bits & DecodeConf::RESTART_INTERVAL_MASK) >> DecodeConf::RESTART_INTERVAL_OFFSET
    }

    /// Set **DEZIGZAG_READY_CTL** `[30:30]`
    ///
    /// reserved for decoder
    #[inline(always)]
    pub const fn with_dezigzag_ready_ctl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DecodeConf::DEZIGZAG_READY_CTL_MASK
            } else {
                self.bits & !DecodeConf::DEZIGZAG_READY_CTL_MASK
            },
        }
    }

    /// Set **MULTI_SCAN_ERR_CHECK** `[29:29]`
    ///
    /// reserved for decoder
    #[inline(always)]
    pub const fn with_multi_scan_err_check(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DecodeConf::MULTI_SCAN_ERR_CHECK_MASK
            } else {
                self.bits & !DecodeConf::MULTI_SCAN_ERR_CHECK_MASK
            },
        }
    }

    /// Set **RST_CHECK_BYTE_NUM** `[28:27]`
    ///
    /// Configure the byte number to check next rst marker after one rst interval is decoded down. The real check number is reg_rst_check_byte_num+1
    #[inline(always)]
    pub const fn with_rst_check_byte_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DecodeConf::RST_CHECK_BYTE_NUM_MASK)
                | ((value << DecodeConf::RST_CHECK_BYTE_NUM_OFFSET)
                    & DecodeConf::RST_CHECK_BYTE_NUM_MASK),
        }
    }

    /// Set **SOS_CHECK_BYTE_NUM** `[26:25]`
    ///
    /// Configure the byte number to check next sos marker in the multi-scan picture after one scan is decoded down. The real check number is reg_sos_check_byte_num+1
    #[inline(always)]
    pub const fn with_sos_check_byte_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DecodeConf::SOS_CHECK_BYTE_NUM_MASK)
                | ((value << DecodeConf::SOS_CHECK_BYTE_NUM_OFFSET)
                    & DecodeConf::SOS_CHECK_BYTE_NUM_MASK),
        }
    }

    /// Set **COMPONENT_NUM** `[23:16]`
    ///
    /// configure number of components in frame when decode
    #[inline(always)]
    pub const fn with_component_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DecodeConf::COMPONENT_NUM_MASK)
                | ((value << DecodeConf::COMPONENT_NUM_OFFSET) & DecodeConf::COMPONENT_NUM_MASK),
        }
    }

    /// Set **RESTART_INTERVAL** `[15:0]`
    ///
    /// configure restart interval in DRI marker when decode
    #[inline(always)]
    pub const fn with_restart_interval(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DecodeConf::RESTART_INTERVAL_MASK)
                | ((value << DecodeConf::RESTART_INTERVAL_OFFSET)
                    & DecodeConf::RESTART_INTERVAL_MASK),
        }
    }
}

impl Default for DecodeConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DecodeConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DecodeConfVal {
        DecodeConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DecodeConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DecodeConfVal) -> DecodeConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// C0 Register
///
/// Control and configuration registers
///
/// Address offset: 0x0024
/// Reset value: 0x00001100
///
/// ## Fields
///
/// - **ID** `[23:16]` (RW)
///
///   the identifier of c0
///
/// - **X_FACTOR** `[15:12]` (RW)
///
///   horizontal sampling factor of c0
///
/// - **Y_FACTOR** `[11:8]` (RW)
///
///   vertical sampling factor of c0
///
/// - **DQT_TBL_SEL** `[7:0]` (RW)
///
///   choose c0 quntization table id (TBD)
#[repr(transparent)]
pub struct C0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for C0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl C0 {
    // ======== Field Constants ========

    /// **ID**
    ///
    /// the identifier of c0
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const ID_OFFSET: u32 = 16;
    pub const ID_MASK: u32 = 0x00ff0000;

    /// **X_FACTOR**
    ///
    /// horizontal sampling factor of c0
    ///
    /// Bits `[15:12]`, mask `0x0000f000`
    pub const X_FACTOR_OFFSET: u32 = 12;
    pub const X_FACTOR_MASK: u32 = 0x0000f000;

    /// **Y_FACTOR**
    ///
    /// vertical sampling factor of c0
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const Y_FACTOR_OFFSET: u32 = 8;
    pub const Y_FACTOR_MASK: u32 = 0x00000f00;

    /// **DQT_TBL_SEL**
    ///
    /// choose c0 quntization table id (TBD)
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DQT_TBL_SEL_OFFSET: u32 = 0;
    pub const DQT_TBL_SEL_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **ID** `[23:16]` (RW)
    ///
    /// the identifier of c0
    #[inline(always)]
    pub fn id(&self) -> u32 {
        (self.value.read_bits() & Self::ID_MASK) >> Self::ID_OFFSET
    }

    /// Write **ID** `[23:16]` (RW)
    ///
    /// the identifier of c0
    #[inline(always)]
    pub fn write_id(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::ID_MASK) | ((value << Self::ID_OFFSET) & Self::ID_MASK));
    }

    /// **X_FACTOR** `[15:12]` (RW)
    ///
    /// horizontal sampling factor of c0
    #[inline(always)]
    pub fn x_factor(&self) -> u32 {
        (self.value.read_bits() & Self::X_FACTOR_MASK) >> Self::X_FACTOR_OFFSET
    }

    /// Write **X_FACTOR** `[15:12]` (RW)
    ///
    /// horizontal sampling factor of c0
    #[inline(always)]
    pub fn write_x_factor(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_FACTOR_MASK) | ((value << Self::X_FACTOR_OFFSET) & Self::X_FACTOR_MASK)
        });
    }

    /// **Y_FACTOR** `[11:8]` (RW)
    ///
    /// vertical sampling factor of c0
    #[inline(always)]
    pub fn y_factor(&self) -> u32 {
        (self.value.read_bits() & Self::Y_FACTOR_MASK) >> Self::Y_FACTOR_OFFSET
    }

    /// Write **Y_FACTOR** `[11:8]` (RW)
    ///
    /// vertical sampling factor of c0
    #[inline(always)]
    pub fn write_y_factor(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_FACTOR_MASK) | ((value << Self::Y_FACTOR_OFFSET) & Self::Y_FACTOR_MASK)
        });
    }

    /// **DQT_TBL_SEL** `[7:0]` (RW)
    ///
    /// choose c0 quntization table id (TBD)
    #[inline(always)]
    pub fn dqt_tbl_sel(&self) -> u32 {
        (self.value.read_bits() & Self::DQT_TBL_SEL_MASK) >> Self::DQT_TBL_SEL_OFFSET
    }

    /// Write **DQT_TBL_SEL** `[7:0]` (RW)
    ///
    /// choose c0 quntization table id (TBD)
    #[inline(always)]
    pub fn write_dqt_tbl_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DQT_TBL_SEL_MASK)
                | ((value << Self::DQT_TBL_SEL_OFFSET) & Self::DQT_TBL_SEL_MASK)
        });
    }
}

/// C0 value type for batch read/modify/write operations
///
/// Use `c0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `c0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct C0Val {
    bits: u32,
}

impl C0Val {
    /// Create from raw bits
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
        Self { bits: 0x00001100 }
    }

    /// Read **ID** `[23:16]`
    ///
    /// the identifier of c0
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        (self.bits & C0::ID_MASK) >> C0::ID_OFFSET
    }

    /// Read **X_FACTOR** `[15:12]`
    ///
    /// horizontal sampling factor of c0
    #[inline(always)]
    pub const fn x_factor(&self) -> u32 {
        (self.bits & C0::X_FACTOR_MASK) >> C0::X_FACTOR_OFFSET
    }

    /// Read **Y_FACTOR** `[11:8]`
    ///
    /// vertical sampling factor of c0
    #[inline(always)]
    pub const fn y_factor(&self) -> u32 {
        (self.bits & C0::Y_FACTOR_MASK) >> C0::Y_FACTOR_OFFSET
    }

    /// Read **DQT_TBL_SEL** `[7:0]`
    ///
    /// choose c0 quntization table id (TBD)
    #[inline(always)]
    pub const fn dqt_tbl_sel(&self) -> u32 {
        (self.bits & C0::DQT_TBL_SEL_MASK) >> C0::DQT_TBL_SEL_OFFSET
    }

    /// Set **ID** `[23:16]`
    ///
    /// the identifier of c0
    #[inline(always)]
    pub const fn with_id(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C0::ID_MASK) | ((value << C0::ID_OFFSET) & C0::ID_MASK),
        }
    }

    /// Set **X_FACTOR** `[15:12]`
    ///
    /// horizontal sampling factor of c0
    #[inline(always)]
    pub const fn with_x_factor(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C0::X_FACTOR_MASK)
                | ((value << C0::X_FACTOR_OFFSET) & C0::X_FACTOR_MASK),
        }
    }

    /// Set **Y_FACTOR** `[11:8]`
    ///
    /// vertical sampling factor of c0
    #[inline(always)]
    pub const fn with_y_factor(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C0::Y_FACTOR_MASK)
                | ((value << C0::Y_FACTOR_OFFSET) & C0::Y_FACTOR_MASK),
        }
    }

    /// Set **DQT_TBL_SEL** `[7:0]`
    ///
    /// choose c0 quntization table id (TBD)
    #[inline(always)]
    pub const fn with_dqt_tbl_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C0::DQT_TBL_SEL_MASK)
                | ((value << C0::DQT_TBL_SEL_OFFSET) & C0::DQT_TBL_SEL_MASK),
        }
    }
}

impl Default for C0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl C0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> C0Val {
        C0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: C0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(C0Val) -> C0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// C1 Register
///
/// Control and configuration registers
///
/// Address offset: 0x0028
/// Reset value: 0x00001100
///
/// ## Fields
///
/// - **ID** `[23:16]` (RW)
///
///   the identifier of c1
///
/// - **X_FACTOR** `[15:12]` (RW)
///
///   horizontal sampling factor of c1
///
/// - **Y_FACTOR** `[11:8]` (RW)
///
///   vertical sampling factor of c1
///
/// - **DQT_TBL_SEL** `[7:0]` (RW)
///
///   choose c1 quntization table id (TBD)
#[repr(transparent)]
pub struct C1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for C1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl C1 {
    // ======== Field Constants ========

    /// **ID**
    ///
    /// the identifier of c1
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const ID_OFFSET: u32 = 16;
    pub const ID_MASK: u32 = 0x00ff0000;

    /// **X_FACTOR**
    ///
    /// horizontal sampling factor of c1
    ///
    /// Bits `[15:12]`, mask `0x0000f000`
    pub const X_FACTOR_OFFSET: u32 = 12;
    pub const X_FACTOR_MASK: u32 = 0x0000f000;

    /// **Y_FACTOR**
    ///
    /// vertical sampling factor of c1
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const Y_FACTOR_OFFSET: u32 = 8;
    pub const Y_FACTOR_MASK: u32 = 0x00000f00;

    /// **DQT_TBL_SEL**
    ///
    /// choose c1 quntization table id (TBD)
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DQT_TBL_SEL_OFFSET: u32 = 0;
    pub const DQT_TBL_SEL_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **ID** `[23:16]` (RW)
    ///
    /// the identifier of c1
    #[inline(always)]
    pub fn id(&self) -> u32 {
        (self.value.read_bits() & Self::ID_MASK) >> Self::ID_OFFSET
    }

    /// Write **ID** `[23:16]` (RW)
    ///
    /// the identifier of c1
    #[inline(always)]
    pub fn write_id(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::ID_MASK) | ((value << Self::ID_OFFSET) & Self::ID_MASK));
    }

    /// **X_FACTOR** `[15:12]` (RW)
    ///
    /// horizontal sampling factor of c1
    #[inline(always)]
    pub fn x_factor(&self) -> u32 {
        (self.value.read_bits() & Self::X_FACTOR_MASK) >> Self::X_FACTOR_OFFSET
    }

    /// Write **X_FACTOR** `[15:12]` (RW)
    ///
    /// horizontal sampling factor of c1
    #[inline(always)]
    pub fn write_x_factor(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_FACTOR_MASK) | ((value << Self::X_FACTOR_OFFSET) & Self::X_FACTOR_MASK)
        });
    }

    /// **Y_FACTOR** `[11:8]` (RW)
    ///
    /// vertical sampling factor of c1
    #[inline(always)]
    pub fn y_factor(&self) -> u32 {
        (self.value.read_bits() & Self::Y_FACTOR_MASK) >> Self::Y_FACTOR_OFFSET
    }

    /// Write **Y_FACTOR** `[11:8]` (RW)
    ///
    /// vertical sampling factor of c1
    #[inline(always)]
    pub fn write_y_factor(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_FACTOR_MASK) | ((value << Self::Y_FACTOR_OFFSET) & Self::Y_FACTOR_MASK)
        });
    }

    /// **DQT_TBL_SEL** `[7:0]` (RW)
    ///
    /// choose c1 quntization table id (TBD)
    #[inline(always)]
    pub fn dqt_tbl_sel(&self) -> u32 {
        (self.value.read_bits() & Self::DQT_TBL_SEL_MASK) >> Self::DQT_TBL_SEL_OFFSET
    }

    /// Write **DQT_TBL_SEL** `[7:0]` (RW)
    ///
    /// choose c1 quntization table id (TBD)
    #[inline(always)]
    pub fn write_dqt_tbl_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DQT_TBL_SEL_MASK)
                | ((value << Self::DQT_TBL_SEL_OFFSET) & Self::DQT_TBL_SEL_MASK)
        });
    }
}

/// C1 value type for batch read/modify/write operations
///
/// Use `c1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `c1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct C1Val {
    bits: u32,
}

impl C1Val {
    /// Create from raw bits
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
        Self { bits: 0x00001100 }
    }

    /// Read **ID** `[23:16]`
    ///
    /// the identifier of c1
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        (self.bits & C1::ID_MASK) >> C1::ID_OFFSET
    }

    /// Read **X_FACTOR** `[15:12]`
    ///
    /// horizontal sampling factor of c1
    #[inline(always)]
    pub const fn x_factor(&self) -> u32 {
        (self.bits & C1::X_FACTOR_MASK) >> C1::X_FACTOR_OFFSET
    }

    /// Read **Y_FACTOR** `[11:8]`
    ///
    /// vertical sampling factor of c1
    #[inline(always)]
    pub const fn y_factor(&self) -> u32 {
        (self.bits & C1::Y_FACTOR_MASK) >> C1::Y_FACTOR_OFFSET
    }

    /// Read **DQT_TBL_SEL** `[7:0]`
    ///
    /// choose c1 quntization table id (TBD)
    #[inline(always)]
    pub const fn dqt_tbl_sel(&self) -> u32 {
        (self.bits & C1::DQT_TBL_SEL_MASK) >> C1::DQT_TBL_SEL_OFFSET
    }

    /// Set **ID** `[23:16]`
    ///
    /// the identifier of c1
    #[inline(always)]
    pub const fn with_id(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C1::ID_MASK) | ((value << C1::ID_OFFSET) & C1::ID_MASK),
        }
    }

    /// Set **X_FACTOR** `[15:12]`
    ///
    /// horizontal sampling factor of c1
    #[inline(always)]
    pub const fn with_x_factor(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C1::X_FACTOR_MASK)
                | ((value << C1::X_FACTOR_OFFSET) & C1::X_FACTOR_MASK),
        }
    }

    /// Set **Y_FACTOR** `[11:8]`
    ///
    /// vertical sampling factor of c1
    #[inline(always)]
    pub const fn with_y_factor(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C1::Y_FACTOR_MASK)
                | ((value << C1::Y_FACTOR_OFFSET) & C1::Y_FACTOR_MASK),
        }
    }

    /// Set **DQT_TBL_SEL** `[7:0]`
    ///
    /// choose c1 quntization table id (TBD)
    #[inline(always)]
    pub const fn with_dqt_tbl_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C1::DQT_TBL_SEL_MASK)
                | ((value << C1::DQT_TBL_SEL_OFFSET) & C1::DQT_TBL_SEL_MASK),
        }
    }
}

impl Default for C1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl C1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> C1Val {
        C1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: C1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(C1Val) -> C1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// C2 Register
///
/// Control and configuration registers
///
/// Address offset: 0x002c
/// Reset value: 0x00001100
///
/// ## Fields
///
/// - **ID** `[23:16]` (RW)
///
///   the identifier of c2
///
/// - **X_FACTOR** `[15:12]` (RW)
///
///   horizontal sampling factor of c2
///
/// - **Y_FACTOR** `[11:8]` (RW)
///
///   vertical sampling factor of c2
///
/// - **DQT_TBL_SEL** `[7:0]` (RW)
///
///   choose c2 quntization table id (TBD)
#[repr(transparent)]
pub struct C2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for C2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl C2 {
    // ======== Field Constants ========

    /// **ID**
    ///
    /// the identifier of c2
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const ID_OFFSET: u32 = 16;
    pub const ID_MASK: u32 = 0x00ff0000;

    /// **X_FACTOR**
    ///
    /// horizontal sampling factor of c2
    ///
    /// Bits `[15:12]`, mask `0x0000f000`
    pub const X_FACTOR_OFFSET: u32 = 12;
    pub const X_FACTOR_MASK: u32 = 0x0000f000;

    /// **Y_FACTOR**
    ///
    /// vertical sampling factor of c2
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const Y_FACTOR_OFFSET: u32 = 8;
    pub const Y_FACTOR_MASK: u32 = 0x00000f00;

    /// **DQT_TBL_SEL**
    ///
    /// choose c2 quntization table id (TBD)
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DQT_TBL_SEL_OFFSET: u32 = 0;
    pub const DQT_TBL_SEL_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **ID** `[23:16]` (RW)
    ///
    /// the identifier of c2
    #[inline(always)]
    pub fn id(&self) -> u32 {
        (self.value.read_bits() & Self::ID_MASK) >> Self::ID_OFFSET
    }

    /// Write **ID** `[23:16]` (RW)
    ///
    /// the identifier of c2
    #[inline(always)]
    pub fn write_id(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::ID_MASK) | ((value << Self::ID_OFFSET) & Self::ID_MASK));
    }

    /// **X_FACTOR** `[15:12]` (RW)
    ///
    /// horizontal sampling factor of c2
    #[inline(always)]
    pub fn x_factor(&self) -> u32 {
        (self.value.read_bits() & Self::X_FACTOR_MASK) >> Self::X_FACTOR_OFFSET
    }

    /// Write **X_FACTOR** `[15:12]` (RW)
    ///
    /// horizontal sampling factor of c2
    #[inline(always)]
    pub fn write_x_factor(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_FACTOR_MASK) | ((value << Self::X_FACTOR_OFFSET) & Self::X_FACTOR_MASK)
        });
    }

    /// **Y_FACTOR** `[11:8]` (RW)
    ///
    /// vertical sampling factor of c2
    #[inline(always)]
    pub fn y_factor(&self) -> u32 {
        (self.value.read_bits() & Self::Y_FACTOR_MASK) >> Self::Y_FACTOR_OFFSET
    }

    /// Write **Y_FACTOR** `[11:8]` (RW)
    ///
    /// vertical sampling factor of c2
    #[inline(always)]
    pub fn write_y_factor(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_FACTOR_MASK) | ((value << Self::Y_FACTOR_OFFSET) & Self::Y_FACTOR_MASK)
        });
    }

    /// **DQT_TBL_SEL** `[7:0]` (RW)
    ///
    /// choose c2 quntization table id (TBD)
    #[inline(always)]
    pub fn dqt_tbl_sel(&self) -> u32 {
        (self.value.read_bits() & Self::DQT_TBL_SEL_MASK) >> Self::DQT_TBL_SEL_OFFSET
    }

    /// Write **DQT_TBL_SEL** `[7:0]` (RW)
    ///
    /// choose c2 quntization table id (TBD)
    #[inline(always)]
    pub fn write_dqt_tbl_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DQT_TBL_SEL_MASK)
                | ((value << Self::DQT_TBL_SEL_OFFSET) & Self::DQT_TBL_SEL_MASK)
        });
    }
}

/// C2 value type for batch read/modify/write operations
///
/// Use `c2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `c2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct C2Val {
    bits: u32,
}

impl C2Val {
    /// Create from raw bits
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
        Self { bits: 0x00001100 }
    }

    /// Read **ID** `[23:16]`
    ///
    /// the identifier of c2
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        (self.bits & C2::ID_MASK) >> C2::ID_OFFSET
    }

    /// Read **X_FACTOR** `[15:12]`
    ///
    /// horizontal sampling factor of c2
    #[inline(always)]
    pub const fn x_factor(&self) -> u32 {
        (self.bits & C2::X_FACTOR_MASK) >> C2::X_FACTOR_OFFSET
    }

    /// Read **Y_FACTOR** `[11:8]`
    ///
    /// vertical sampling factor of c2
    #[inline(always)]
    pub const fn y_factor(&self) -> u32 {
        (self.bits & C2::Y_FACTOR_MASK) >> C2::Y_FACTOR_OFFSET
    }

    /// Read **DQT_TBL_SEL** `[7:0]`
    ///
    /// choose c2 quntization table id (TBD)
    #[inline(always)]
    pub const fn dqt_tbl_sel(&self) -> u32 {
        (self.bits & C2::DQT_TBL_SEL_MASK) >> C2::DQT_TBL_SEL_OFFSET
    }

    /// Set **ID** `[23:16]`
    ///
    /// the identifier of c2
    #[inline(always)]
    pub const fn with_id(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C2::ID_MASK) | ((value << C2::ID_OFFSET) & C2::ID_MASK),
        }
    }

    /// Set **X_FACTOR** `[15:12]`
    ///
    /// horizontal sampling factor of c2
    #[inline(always)]
    pub const fn with_x_factor(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C2::X_FACTOR_MASK)
                | ((value << C2::X_FACTOR_OFFSET) & C2::X_FACTOR_MASK),
        }
    }

    /// Set **Y_FACTOR** `[11:8]`
    ///
    /// vertical sampling factor of c2
    #[inline(always)]
    pub const fn with_y_factor(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C2::Y_FACTOR_MASK)
                | ((value << C2::Y_FACTOR_OFFSET) & C2::Y_FACTOR_MASK),
        }
    }

    /// Set **DQT_TBL_SEL** `[7:0]`
    ///
    /// choose c2 quntization table id (TBD)
    #[inline(always)]
    pub const fn with_dqt_tbl_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C2::DQT_TBL_SEL_MASK)
                | ((value << C2::DQT_TBL_SEL_OFFSET) & C2::DQT_TBL_SEL_MASK),
        }
    }
}

impl Default for C2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl C2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> C2Val {
        C2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: C2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(C2Val) -> C2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// C3 Register
///
/// Control and configuration registers
///
/// Address offset: 0x0030
/// Reset value: 0x00001100
///
/// ## Fields
///
/// - **ID** `[23:16]` (RW)
///
///   the identifier of c3
///
/// - **X_FACTOR** `[15:12]` (RW)
///
///   horizontal sampling factor of c3
///
/// - **Y_FACTOR** `[11:8]` (RW)
///
///   vertical sampling factor of c3
///
/// - **DQT_TBL_SEL** `[7:0]` (RW)
///
///   choose c3 quntization table id (TBD)
#[repr(transparent)]
pub struct C3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for C3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl C3 {
    // ======== Field Constants ========

    /// **ID**
    ///
    /// the identifier of c3
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const ID_OFFSET: u32 = 16;
    pub const ID_MASK: u32 = 0x00ff0000;

    /// **X_FACTOR**
    ///
    /// horizontal sampling factor of c3
    ///
    /// Bits `[15:12]`, mask `0x0000f000`
    pub const X_FACTOR_OFFSET: u32 = 12;
    pub const X_FACTOR_MASK: u32 = 0x0000f000;

    /// **Y_FACTOR**
    ///
    /// vertical sampling factor of c3
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const Y_FACTOR_OFFSET: u32 = 8;
    pub const Y_FACTOR_MASK: u32 = 0x00000f00;

    /// **DQT_TBL_SEL**
    ///
    /// choose c3 quntization table id (TBD)
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DQT_TBL_SEL_OFFSET: u32 = 0;
    pub const DQT_TBL_SEL_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **ID** `[23:16]` (RW)
    ///
    /// the identifier of c3
    #[inline(always)]
    pub fn id(&self) -> u32 {
        (self.value.read_bits() & Self::ID_MASK) >> Self::ID_OFFSET
    }

    /// Write **ID** `[23:16]` (RW)
    ///
    /// the identifier of c3
    #[inline(always)]
    pub fn write_id(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::ID_MASK) | ((value << Self::ID_OFFSET) & Self::ID_MASK));
    }

    /// **X_FACTOR** `[15:12]` (RW)
    ///
    /// horizontal sampling factor of c3
    #[inline(always)]
    pub fn x_factor(&self) -> u32 {
        (self.value.read_bits() & Self::X_FACTOR_MASK) >> Self::X_FACTOR_OFFSET
    }

    /// Write **X_FACTOR** `[15:12]` (RW)
    ///
    /// horizontal sampling factor of c3
    #[inline(always)]
    pub fn write_x_factor(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::X_FACTOR_MASK) | ((value << Self::X_FACTOR_OFFSET) & Self::X_FACTOR_MASK)
        });
    }

    /// **Y_FACTOR** `[11:8]` (RW)
    ///
    /// vertical sampling factor of c3
    #[inline(always)]
    pub fn y_factor(&self) -> u32 {
        (self.value.read_bits() & Self::Y_FACTOR_MASK) >> Self::Y_FACTOR_OFFSET
    }

    /// Write **Y_FACTOR** `[11:8]` (RW)
    ///
    /// vertical sampling factor of c3
    #[inline(always)]
    pub fn write_y_factor(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::Y_FACTOR_MASK) | ((value << Self::Y_FACTOR_OFFSET) & Self::Y_FACTOR_MASK)
        });
    }

    /// **DQT_TBL_SEL** `[7:0]` (RW)
    ///
    /// choose c3 quntization table id (TBD)
    #[inline(always)]
    pub fn dqt_tbl_sel(&self) -> u32 {
        (self.value.read_bits() & Self::DQT_TBL_SEL_MASK) >> Self::DQT_TBL_SEL_OFFSET
    }

    /// Write **DQT_TBL_SEL** `[7:0]` (RW)
    ///
    /// choose c3 quntization table id (TBD)
    #[inline(always)]
    pub fn write_dqt_tbl_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DQT_TBL_SEL_MASK)
                | ((value << Self::DQT_TBL_SEL_OFFSET) & Self::DQT_TBL_SEL_MASK)
        });
    }
}

/// C3 value type for batch read/modify/write operations
///
/// Use `c3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `c3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct C3Val {
    bits: u32,
}

impl C3Val {
    /// Create from raw bits
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
        Self { bits: 0x00001100 }
    }

    /// Read **ID** `[23:16]`
    ///
    /// the identifier of c3
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        (self.bits & C3::ID_MASK) >> C3::ID_OFFSET
    }

    /// Read **X_FACTOR** `[15:12]`
    ///
    /// horizontal sampling factor of c3
    #[inline(always)]
    pub const fn x_factor(&self) -> u32 {
        (self.bits & C3::X_FACTOR_MASK) >> C3::X_FACTOR_OFFSET
    }

    /// Read **Y_FACTOR** `[11:8]`
    ///
    /// vertical sampling factor of c3
    #[inline(always)]
    pub const fn y_factor(&self) -> u32 {
        (self.bits & C3::Y_FACTOR_MASK) >> C3::Y_FACTOR_OFFSET
    }

    /// Read **DQT_TBL_SEL** `[7:0]`
    ///
    /// choose c3 quntization table id (TBD)
    #[inline(always)]
    pub const fn dqt_tbl_sel(&self) -> u32 {
        (self.bits & C3::DQT_TBL_SEL_MASK) >> C3::DQT_TBL_SEL_OFFSET
    }

    /// Set **ID** `[23:16]`
    ///
    /// the identifier of c3
    #[inline(always)]
    pub const fn with_id(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C3::ID_MASK) | ((value << C3::ID_OFFSET) & C3::ID_MASK),
        }
    }

    /// Set **X_FACTOR** `[15:12]`
    ///
    /// horizontal sampling factor of c3
    #[inline(always)]
    pub const fn with_x_factor(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C3::X_FACTOR_MASK)
                | ((value << C3::X_FACTOR_OFFSET) & C3::X_FACTOR_MASK),
        }
    }

    /// Set **Y_FACTOR** `[11:8]`
    ///
    /// vertical sampling factor of c3
    #[inline(always)]
    pub const fn with_y_factor(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C3::Y_FACTOR_MASK)
                | ((value << C3::Y_FACTOR_OFFSET) & C3::Y_FACTOR_MASK),
        }
    }

    /// Set **DQT_TBL_SEL** `[7:0]`
    ///
    /// choose c3 quntization table id (TBD)
    #[inline(always)]
    pub const fn with_dqt_tbl_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !C3::DQT_TBL_SEL_MASK)
                | ((value << C3::DQT_TBL_SEL_OFFSET) & C3::DQT_TBL_SEL_MASK),
        }
    }
}

impl Default for C3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl C3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> C3Val {
        C3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: C3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(C3Val) -> C3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DHT_INFO Register
///
/// Control and configuration registers
///
/// Address offset: 0x0034
/// Reset value: 0x00001010
///
/// ## Fields
///
/// - **AC1_DHT_ID** `[15:12]` (RW)
///
///   configure dht ac table 1 id
///
/// - **AC0_DHT_ID** `[11:8]` (RW)
///
///   configure dht ac table 0 id
///
/// - **DC1_DHT_ID** `[7:4]` (RW)
///
///   configure dht dc table 1 id
///
/// - **DC0_DHT_ID** `[3:0]` (RW)
///
///   configure dht dc table 0 id
#[repr(transparent)]
pub struct DhtInfo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DhtInfo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DhtInfo {
    // ======== Field Constants ========

    /// **AC1_DHT_ID**
    ///
    /// configure dht ac table 1 id
    ///
    /// Bits `[15:12]`, mask `0x0000f000`
    pub const AC1_DHT_ID_OFFSET: u32 = 12;
    pub const AC1_DHT_ID_MASK: u32 = 0x0000f000;

    /// **AC0_DHT_ID**
    ///
    /// configure dht ac table 0 id
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const AC0_DHT_ID_OFFSET: u32 = 8;
    pub const AC0_DHT_ID_MASK: u32 = 0x00000f00;

    /// **DC1_DHT_ID**
    ///
    /// configure dht dc table 1 id
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const DC1_DHT_ID_OFFSET: u32 = 4;
    pub const DC1_DHT_ID_MASK: u32 = 0x000000f0;

    /// **DC0_DHT_ID**
    ///
    /// configure dht dc table 0 id
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const DC0_DHT_ID_OFFSET: u32 = 0;
    pub const DC0_DHT_ID_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **AC1_DHT_ID** `[15:12]` (RW)
    ///
    /// configure dht ac table 1 id
    #[inline(always)]
    pub fn ac1_dht_id(&self) -> u32 {
        (self.value.read_bits() & Self::AC1_DHT_ID_MASK) >> Self::AC1_DHT_ID_OFFSET
    }

    /// Write **AC1_DHT_ID** `[15:12]` (RW)
    ///
    /// configure dht ac table 1 id
    #[inline(always)]
    pub fn write_ac1_dht_id(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AC1_DHT_ID_MASK)
                | ((value << Self::AC1_DHT_ID_OFFSET) & Self::AC1_DHT_ID_MASK)
        });
    }

    /// **AC0_DHT_ID** `[11:8]` (RW)
    ///
    /// configure dht ac table 0 id
    #[inline(always)]
    pub fn ac0_dht_id(&self) -> u32 {
        (self.value.read_bits() & Self::AC0_DHT_ID_MASK) >> Self::AC0_DHT_ID_OFFSET
    }

    /// Write **AC0_DHT_ID** `[11:8]` (RW)
    ///
    /// configure dht ac table 0 id
    #[inline(always)]
    pub fn write_ac0_dht_id(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AC0_DHT_ID_MASK)
                | ((value << Self::AC0_DHT_ID_OFFSET) & Self::AC0_DHT_ID_MASK)
        });
    }

    /// **DC1_DHT_ID** `[7:4]` (RW)
    ///
    /// configure dht dc table 1 id
    #[inline(always)]
    pub fn dc1_dht_id(&self) -> u32 {
        (self.value.read_bits() & Self::DC1_DHT_ID_MASK) >> Self::DC1_DHT_ID_OFFSET
    }

    /// Write **DC1_DHT_ID** `[7:4]` (RW)
    ///
    /// configure dht dc table 1 id
    #[inline(always)]
    pub fn write_dc1_dht_id(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DC1_DHT_ID_MASK)
                | ((value << Self::DC1_DHT_ID_OFFSET) & Self::DC1_DHT_ID_MASK)
        });
    }

    /// **DC0_DHT_ID** `[3:0]` (RW)
    ///
    /// configure dht dc table 0 id
    #[inline(always)]
    pub fn dc0_dht_id(&self) -> u32 {
        (self.value.read_bits() & Self::DC0_DHT_ID_MASK) >> Self::DC0_DHT_ID_OFFSET
    }

    /// Write **DC0_DHT_ID** `[3:0]` (RW)
    ///
    /// configure dht dc table 0 id
    #[inline(always)]
    pub fn write_dc0_dht_id(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DC0_DHT_ID_MASK)
                | ((value << Self::DC0_DHT_ID_OFFSET) & Self::DC0_DHT_ID_MASK)
        });
    }
}

/// DHT_INFO value type for batch read/modify/write operations
///
/// Use `dhtinfo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dhtinfo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DhtInfoVal {
    bits: u32,
}

impl DhtInfoVal {
    /// Create from raw bits
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
        Self { bits: 0x00001010 }
    }

    /// Read **AC1_DHT_ID** `[15:12]`
    ///
    /// configure dht ac table 1 id
    #[inline(always)]
    pub const fn ac1_dht_id(&self) -> u32 {
        (self.bits & DhtInfo::AC1_DHT_ID_MASK) >> DhtInfo::AC1_DHT_ID_OFFSET
    }

    /// Read **AC0_DHT_ID** `[11:8]`
    ///
    /// configure dht ac table 0 id
    #[inline(always)]
    pub const fn ac0_dht_id(&self) -> u32 {
        (self.bits & DhtInfo::AC0_DHT_ID_MASK) >> DhtInfo::AC0_DHT_ID_OFFSET
    }

    /// Read **DC1_DHT_ID** `[7:4]`
    ///
    /// configure dht dc table 1 id
    #[inline(always)]
    pub const fn dc1_dht_id(&self) -> u32 {
        (self.bits & DhtInfo::DC1_DHT_ID_MASK) >> DhtInfo::DC1_DHT_ID_OFFSET
    }

    /// Read **DC0_DHT_ID** `[3:0]`
    ///
    /// configure dht dc table 0 id
    #[inline(always)]
    pub const fn dc0_dht_id(&self) -> u32 {
        (self.bits & DhtInfo::DC0_DHT_ID_MASK) >> DhtInfo::DC0_DHT_ID_OFFSET
    }

    /// Set **AC1_DHT_ID** `[15:12]`
    ///
    /// configure dht ac table 1 id
    #[inline(always)]
    pub const fn with_ac1_dht_id(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DhtInfo::AC1_DHT_ID_MASK)
                | ((value << DhtInfo::AC1_DHT_ID_OFFSET) & DhtInfo::AC1_DHT_ID_MASK),
        }
    }

    /// Set **AC0_DHT_ID** `[11:8]`
    ///
    /// configure dht ac table 0 id
    #[inline(always)]
    pub const fn with_ac0_dht_id(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DhtInfo::AC0_DHT_ID_MASK)
                | ((value << DhtInfo::AC0_DHT_ID_OFFSET) & DhtInfo::AC0_DHT_ID_MASK),
        }
    }

    /// Set **DC1_DHT_ID** `[7:4]`
    ///
    /// configure dht dc table 1 id
    #[inline(always)]
    pub const fn with_dc1_dht_id(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DhtInfo::DC1_DHT_ID_MASK)
                | ((value << DhtInfo::DC1_DHT_ID_OFFSET) & DhtInfo::DC1_DHT_ID_MASK),
        }
    }

    /// Set **DC0_DHT_ID** `[3:0]`
    ///
    /// configure dht dc table 0 id
    #[inline(always)]
    pub const fn with_dc0_dht_id(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DhtInfo::DC0_DHT_ID_MASK)
                | ((value << DhtInfo::DC0_DHT_ID_OFFSET) & DhtInfo::DC0_DHT_ID_MASK),
        }
    }
}

impl Default for DhtInfoVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DhtInfo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DhtInfoVal {
        DhtInfoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DhtInfoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DhtInfoVal) -> DhtInfoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_RAW Register
///
/// Interrupt raw registers
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **DECODE_TIMEOUT_INT_RAW** `[24]` (RW)
///
///   The raw interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
///
/// - **UNDET_INT_RAW** `[23]` (RW)
///
///   The raw interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
///
/// - **MARKER_ERR_OTHER_SCAN_INT_RAW** `[22]` (RW)
///
///   The raw interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
///
/// - **MARKER_ERR_FST_SCAN_INT_RAW** `[21]` (RW)
///
///   The raw interrupt bit to sign that the first scan has header marker error when decoding.
///
/// - **SOS_UNMATCH_ERR_INT_RAW** `[20]` (RW)
///
///   The raw interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
///
/// - **DE_FRAME_EOF_LACK_INT_RAW** `[19]` (RW)
///
///   The raw interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
///
/// - **DE_FRAME_EOF_ERR_INT_RAW** `[18]` (RW)
///
///   The raw interrupt bit to sign that decoded blocks are smaller than expected when decoding.
///
/// - **EN_FRAME_EOF_LACK_INT_RAW** `[17]` (RW)
///
///   The raw interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
///
/// - **EN_FRAME_EOF_ERR_INT_RAW** `[16]` (RW)
///
///   The raw interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
///
/// - **UXP_DET_INT_RAW** `[15]` (RW)
///
///   The raw interrupt bit to sign that unsupported header marker is detected when decoding.
///
/// - **SCAN_CHECK_POS_ERR_INT_RAW** `[14]` (RW)
///
///   The raw interrupt bit to sign that SOS header marker position wrong when decoding.
///
/// - **SCAN_CHECK_NONE_ERR_INT_RAW** `[13]` (RW)
///
///   The raw interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
///
/// - **BS_LAST_BLOCK_EOF_INT_RAW** `[12]` (RW)
///
///   The raw interrupt bit to sign that the coding process for last block is finished.
///
/// - **DCT_DONE_INT_RAW** `[11]` (RW)
///
///   The raw interrupt bit to sign that one dct calculation is finished.
///
/// - **SR_COLOR_MODE_ERR_INT_RAW** `[10]` (RW)
///
///   The raw interrupt bit to sign that the selected source color mode is not supported.
///
/// - **OUT_EOF_INT_RAW** `[9]` (RW)
///
///   The raw interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
///
/// - **RST_CHECK_POS_ERR_INT_RAW** `[8]` (RW)
///
///   The raw interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
///
/// - **RST_CHECK_NONE_ERR_INT_RAW** `[7]` (RW)
///
///   The raw interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
///
/// - **RST_UXP_ERR_INT_RAW** `[6]` (RW)
///
///   The raw interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
///
/// - **C_DQT_ID_ERR_INT_RAW** `[5]` (RW)
///
///   The raw interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
///
/// - **C_DHT_AC_ID_ERR_INT_RAW** `[4]` (RW)
///
///   The raw interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
///
/// - **C_DHT_DC_ID_ERR_INT_RAW** `[3]` (RW)
///
///   The raw interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
///
/// - **CID_ERR_INT_RAW** `[2]` (RW)
///
///   The raw interrupt bit to sign that scan id check with component fails when decoding.
///
/// - **RLE_PARALLEL_ERR_INT_RAW** `[1]` (RW)
///
///   The raw interrupt bit to sign that rle parallel error when decoding.
///
/// - **DONE_INT_RAW** `[0]` (RW)
///
///   This raw interrupt bit turns to high level when JPEG finishes encoding a picture..
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

    /// **DECODE_TIMEOUT_INT_RAW**
    ///
    /// The raw interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const DECODE_TIMEOUT_INT_RAW_OFFSET: u32 = 24;
    pub const DECODE_TIMEOUT_INT_RAW_MASK: u32 = 0x01000000;
    pub const DECODE_TIMEOUT_INT_RAW: u32 = 1 << 24;

    /// **UNDET_INT_RAW**
    ///
    /// The raw interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const UNDET_INT_RAW_OFFSET: u32 = 23;
    pub const UNDET_INT_RAW_MASK: u32 = 0x00800000;
    pub const UNDET_INT_RAW: u32 = 1 << 23;

    /// **MARKER_ERR_OTHER_SCAN_INT_RAW**
    ///
    /// The raw interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const MARKER_ERR_OTHER_SCAN_INT_RAW_OFFSET: u32 = 22;
    pub const MARKER_ERR_OTHER_SCAN_INT_RAW_MASK: u32 = 0x00400000;
    pub const MARKER_ERR_OTHER_SCAN_INT_RAW: u32 = 1 << 22;

    /// **MARKER_ERR_FST_SCAN_INT_RAW**
    ///
    /// The raw interrupt bit to sign that the first scan has header marker error when decoding.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const MARKER_ERR_FST_SCAN_INT_RAW_OFFSET: u32 = 21;
    pub const MARKER_ERR_FST_SCAN_INT_RAW_MASK: u32 = 0x00200000;
    pub const MARKER_ERR_FST_SCAN_INT_RAW: u32 = 1 << 21;

    /// **SOS_UNMATCH_ERR_INT_RAW**
    ///
    /// The raw interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SOS_UNMATCH_ERR_INT_RAW_OFFSET: u32 = 20;
    pub const SOS_UNMATCH_ERR_INT_RAW_MASK: u32 = 0x00100000;
    pub const SOS_UNMATCH_ERR_INT_RAW: u32 = 1 << 20;

    /// **DE_FRAME_EOF_LACK_INT_RAW**
    ///
    /// The raw interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const DE_FRAME_EOF_LACK_INT_RAW_OFFSET: u32 = 19;
    pub const DE_FRAME_EOF_LACK_INT_RAW_MASK: u32 = 0x00080000;
    pub const DE_FRAME_EOF_LACK_INT_RAW: u32 = 1 << 19;

    /// **DE_FRAME_EOF_ERR_INT_RAW**
    ///
    /// The raw interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const DE_FRAME_EOF_ERR_INT_RAW_OFFSET: u32 = 18;
    pub const DE_FRAME_EOF_ERR_INT_RAW_MASK: u32 = 0x00040000;
    pub const DE_FRAME_EOF_ERR_INT_RAW: u32 = 1 << 18;

    /// **EN_FRAME_EOF_LACK_INT_RAW**
    ///
    /// The raw interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const EN_FRAME_EOF_LACK_INT_RAW_OFFSET: u32 = 17;
    pub const EN_FRAME_EOF_LACK_INT_RAW_MASK: u32 = 0x00020000;
    pub const EN_FRAME_EOF_LACK_INT_RAW: u32 = 1 << 17;

    /// **EN_FRAME_EOF_ERR_INT_RAW**
    ///
    /// The raw interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const EN_FRAME_EOF_ERR_INT_RAW_OFFSET: u32 = 16;
    pub const EN_FRAME_EOF_ERR_INT_RAW_MASK: u32 = 0x00010000;
    pub const EN_FRAME_EOF_ERR_INT_RAW: u32 = 1 << 16;

    /// **UXP_DET_INT_RAW**
    ///
    /// The raw interrupt bit to sign that unsupported header marker is detected when decoding.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const UXP_DET_INT_RAW_OFFSET: u32 = 15;
    pub const UXP_DET_INT_RAW_MASK: u32 = 0x00008000;
    pub const UXP_DET_INT_RAW: u32 = 1 << 15;

    /// **SCAN_CHECK_POS_ERR_INT_RAW**
    ///
    /// The raw interrupt bit to sign that SOS header marker position wrong when decoding.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SCAN_CHECK_POS_ERR_INT_RAW_OFFSET: u32 = 14;
    pub const SCAN_CHECK_POS_ERR_INT_RAW_MASK: u32 = 0x00004000;
    pub const SCAN_CHECK_POS_ERR_INT_RAW: u32 = 1 << 14;

    /// **SCAN_CHECK_NONE_ERR_INT_RAW**
    ///
    /// The raw interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SCAN_CHECK_NONE_ERR_INT_RAW_OFFSET: u32 = 13;
    pub const SCAN_CHECK_NONE_ERR_INT_RAW_MASK: u32 = 0x00002000;
    pub const SCAN_CHECK_NONE_ERR_INT_RAW: u32 = 1 << 13;

    /// **BS_LAST_BLOCK_EOF_INT_RAW**
    ///
    /// The raw interrupt bit to sign that the coding process for last block is finished.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const BS_LAST_BLOCK_EOF_INT_RAW_OFFSET: u32 = 12;
    pub const BS_LAST_BLOCK_EOF_INT_RAW_MASK: u32 = 0x00001000;
    pub const BS_LAST_BLOCK_EOF_INT_RAW: u32 = 1 << 12;

    /// **DCT_DONE_INT_RAW**
    ///
    /// The raw interrupt bit to sign that one dct calculation is finished.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const DCT_DONE_INT_RAW_OFFSET: u32 = 11;
    pub const DCT_DONE_INT_RAW_MASK: u32 = 0x00000800;
    pub const DCT_DONE_INT_RAW: u32 = 1 << 11;

    /// **SR_COLOR_MODE_ERR_INT_RAW**
    ///
    /// The raw interrupt bit to sign that the selected source color mode is not supported.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SR_COLOR_MODE_ERR_INT_RAW_OFFSET: u32 = 10;
    pub const SR_COLOR_MODE_ERR_INT_RAW_MASK: u32 = 0x00000400;
    pub const SR_COLOR_MODE_ERR_INT_RAW: u32 = 1 << 10;

    /// **OUT_EOF_INT_RAW**
    ///
    /// The raw interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const OUT_EOF_INT_RAW_OFFSET: u32 = 9;
    pub const OUT_EOF_INT_RAW_MASK: u32 = 0x00000200;
    pub const OUT_EOF_INT_RAW: u32 = 1 << 9;

    /// **RST_CHECK_POS_ERR_INT_RAW**
    ///
    /// The raw interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RST_CHECK_POS_ERR_INT_RAW_OFFSET: u32 = 8;
    pub const RST_CHECK_POS_ERR_INT_RAW_MASK: u32 = 0x00000100;
    pub const RST_CHECK_POS_ERR_INT_RAW: u32 = 1 << 8;

    /// **RST_CHECK_NONE_ERR_INT_RAW**
    ///
    /// The raw interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RST_CHECK_NONE_ERR_INT_RAW_OFFSET: u32 = 7;
    pub const RST_CHECK_NONE_ERR_INT_RAW_MASK: u32 = 0x00000080;
    pub const RST_CHECK_NONE_ERR_INT_RAW: u32 = 1 << 7;

    /// **RST_UXP_ERR_INT_RAW**
    ///
    /// The raw interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RST_UXP_ERR_INT_RAW_OFFSET: u32 = 6;
    pub const RST_UXP_ERR_INT_RAW_MASK: u32 = 0x00000040;
    pub const RST_UXP_ERR_INT_RAW: u32 = 1 << 6;

    /// **C_DQT_ID_ERR_INT_RAW**
    ///
    /// The raw interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const C_DQT_ID_ERR_INT_RAW_OFFSET: u32 = 5;
    pub const C_DQT_ID_ERR_INT_RAW_MASK: u32 = 0x00000020;
    pub const C_DQT_ID_ERR_INT_RAW: u32 = 1 << 5;

    /// **C_DHT_AC_ID_ERR_INT_RAW**
    ///
    /// The raw interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const C_DHT_AC_ID_ERR_INT_RAW_OFFSET: u32 = 4;
    pub const C_DHT_AC_ID_ERR_INT_RAW_MASK: u32 = 0x00000010;
    pub const C_DHT_AC_ID_ERR_INT_RAW: u32 = 1 << 4;

    /// **C_DHT_DC_ID_ERR_INT_RAW**
    ///
    /// The raw interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const C_DHT_DC_ID_ERR_INT_RAW_OFFSET: u32 = 3;
    pub const C_DHT_DC_ID_ERR_INT_RAW_MASK: u32 = 0x00000008;
    pub const C_DHT_DC_ID_ERR_INT_RAW: u32 = 1 << 3;

    /// **CID_ERR_INT_RAW**
    ///
    /// The raw interrupt bit to sign that scan id check with component fails when decoding.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CID_ERR_INT_RAW_OFFSET: u32 = 2;
    pub const CID_ERR_INT_RAW_MASK: u32 = 0x00000004;
    pub const CID_ERR_INT_RAW: u32 = 1 << 2;

    /// **RLE_PARALLEL_ERR_INT_RAW**
    ///
    /// The raw interrupt bit to sign that rle parallel error when decoding.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RLE_PARALLEL_ERR_INT_RAW_OFFSET: u32 = 1;
    pub const RLE_PARALLEL_ERR_INT_RAW_MASK: u32 = 0x00000002;
    pub const RLE_PARALLEL_ERR_INT_RAW: u32 = 1 << 1;

    /// **DONE_INT_RAW**
    ///
    /// This raw interrupt bit turns to high level when JPEG finishes encoding a picture..
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DONE_INT_RAW_OFFSET: u32 = 0;
    pub const DONE_INT_RAW_MASK: u32 = 0x00000001;
    pub const DONE_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DECODE_TIMEOUT_INT_RAW** `[24]` (RW)
    ///
    /// The raw interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    #[inline(always)]
    pub fn decode_timeout_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DECODE_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Set DECODE_TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_decode_timeout_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DECODE_TIMEOUT_INT_RAW_MASK);
    }

    /// Clear DECODE_TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_decode_timeout_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DECODE_TIMEOUT_INT_RAW_MASK);
    }

    /// **UNDET_INT_RAW** `[23]` (RW)
    ///
    /// The raw interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    #[inline(always)]
    pub fn undet_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::UNDET_INT_RAW_MASK) != 0
    }

    /// Set UNDET_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_undet_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::UNDET_INT_RAW_MASK);
    }

    /// Clear UNDET_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_undet_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::UNDET_INT_RAW_MASK);
    }

    /// **MARKER_ERR_OTHER_SCAN_INT_RAW** `[22]` (RW)
    ///
    /// The raw interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    #[inline(always)]
    pub fn marker_err_other_scan_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::MARKER_ERR_OTHER_SCAN_INT_RAW_MASK) != 0
    }

    /// Set MARKER_ERR_OTHER_SCAN_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_marker_err_other_scan_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::MARKER_ERR_OTHER_SCAN_INT_RAW_MASK);
    }

    /// Clear MARKER_ERR_OTHER_SCAN_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_marker_err_other_scan_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::MARKER_ERR_OTHER_SCAN_INT_RAW_MASK);
    }

    /// **MARKER_ERR_FST_SCAN_INT_RAW** `[21]` (RW)
    ///
    /// The raw interrupt bit to sign that the first scan has header marker error when decoding.
    #[inline(always)]
    pub fn marker_err_fst_scan_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::MARKER_ERR_FST_SCAN_INT_RAW_MASK) != 0
    }

    /// Set MARKER_ERR_FST_SCAN_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_marker_err_fst_scan_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::MARKER_ERR_FST_SCAN_INT_RAW_MASK);
    }

    /// Clear MARKER_ERR_FST_SCAN_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_marker_err_fst_scan_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::MARKER_ERR_FST_SCAN_INT_RAW_MASK);
    }

    /// **SOS_UNMATCH_ERR_INT_RAW** `[20]` (RW)
    ///
    /// The raw interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    #[inline(always)]
    pub fn sos_unmatch_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SOS_UNMATCH_ERR_INT_RAW_MASK) != 0
    }

    /// Set SOS_UNMATCH_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sos_unmatch_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SOS_UNMATCH_ERR_INT_RAW_MASK);
    }

    /// Clear SOS_UNMATCH_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sos_unmatch_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SOS_UNMATCH_ERR_INT_RAW_MASK);
    }

    /// **DE_FRAME_EOF_LACK_INT_RAW** `[19]` (RW)
    ///
    /// The raw interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    #[inline(always)]
    pub fn de_frame_eof_lack_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DE_FRAME_EOF_LACK_INT_RAW_MASK) != 0
    }

    /// Set DE_FRAME_EOF_LACK_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_de_frame_eof_lack_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DE_FRAME_EOF_LACK_INT_RAW_MASK);
    }

    /// Clear DE_FRAME_EOF_LACK_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_de_frame_eof_lack_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DE_FRAME_EOF_LACK_INT_RAW_MASK);
    }

    /// **DE_FRAME_EOF_ERR_INT_RAW** `[18]` (RW)
    ///
    /// The raw interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    #[inline(always)]
    pub fn de_frame_eof_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DE_FRAME_EOF_ERR_INT_RAW_MASK) != 0
    }

    /// Set DE_FRAME_EOF_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_de_frame_eof_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DE_FRAME_EOF_ERR_INT_RAW_MASK);
    }

    /// Clear DE_FRAME_EOF_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_de_frame_eof_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DE_FRAME_EOF_ERR_INT_RAW_MASK);
    }

    /// **EN_FRAME_EOF_LACK_INT_RAW** `[17]` (RW)
    ///
    /// The raw interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    #[inline(always)]
    pub fn en_frame_eof_lack_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::EN_FRAME_EOF_LACK_INT_RAW_MASK) != 0
    }

    /// Set EN_FRAME_EOF_LACK_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en_frame_eof_lack_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::EN_FRAME_EOF_LACK_INT_RAW_MASK);
    }

    /// Clear EN_FRAME_EOF_LACK_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en_frame_eof_lack_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::EN_FRAME_EOF_LACK_INT_RAW_MASK);
    }

    /// **EN_FRAME_EOF_ERR_INT_RAW** `[16]` (RW)
    ///
    /// The raw interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    #[inline(always)]
    pub fn en_frame_eof_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::EN_FRAME_EOF_ERR_INT_RAW_MASK) != 0
    }

    /// Set EN_FRAME_EOF_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en_frame_eof_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::EN_FRAME_EOF_ERR_INT_RAW_MASK);
    }

    /// Clear EN_FRAME_EOF_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en_frame_eof_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::EN_FRAME_EOF_ERR_INT_RAW_MASK);
    }

    /// **UXP_DET_INT_RAW** `[15]` (RW)
    ///
    /// The raw interrupt bit to sign that unsupported header marker is detected when decoding.
    #[inline(always)]
    pub fn uxp_det_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::UXP_DET_INT_RAW_MASK) != 0
    }

    /// Set UXP_DET_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_uxp_det_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::UXP_DET_INT_RAW_MASK);
    }

    /// Clear UXP_DET_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_uxp_det_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::UXP_DET_INT_RAW_MASK);
    }

    /// **SCAN_CHECK_POS_ERR_INT_RAW** `[14]` (RW)
    ///
    /// The raw interrupt bit to sign that SOS header marker position wrong when decoding.
    #[inline(always)]
    pub fn scan_check_pos_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SCAN_CHECK_POS_ERR_INT_RAW_MASK) != 0
    }

    /// Set SCAN_CHECK_POS_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scan_check_pos_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SCAN_CHECK_POS_ERR_INT_RAW_MASK);
    }

    /// Clear SCAN_CHECK_POS_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scan_check_pos_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SCAN_CHECK_POS_ERR_INT_RAW_MASK);
    }

    /// **SCAN_CHECK_NONE_ERR_INT_RAW** `[13]` (RW)
    ///
    /// The raw interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    #[inline(always)]
    pub fn scan_check_none_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SCAN_CHECK_NONE_ERR_INT_RAW_MASK) != 0
    }

    /// Set SCAN_CHECK_NONE_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scan_check_none_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SCAN_CHECK_NONE_ERR_INT_RAW_MASK);
    }

    /// Clear SCAN_CHECK_NONE_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scan_check_none_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SCAN_CHECK_NONE_ERR_INT_RAW_MASK);
    }

    /// **BS_LAST_BLOCK_EOF_INT_RAW** `[12]` (RW)
    ///
    /// The raw interrupt bit to sign that the coding process for last block is finished.
    #[inline(always)]
    pub fn bs_last_block_eof_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::BS_LAST_BLOCK_EOF_INT_RAW_MASK) != 0
    }

    /// Set BS_LAST_BLOCK_EOF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bs_last_block_eof_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::BS_LAST_BLOCK_EOF_INT_RAW_MASK);
    }

    /// Clear BS_LAST_BLOCK_EOF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bs_last_block_eof_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::BS_LAST_BLOCK_EOF_INT_RAW_MASK);
    }

    /// **DCT_DONE_INT_RAW** `[11]` (RW)
    ///
    /// The raw interrupt bit to sign that one dct calculation is finished.
    #[inline(always)]
    pub fn dct_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DCT_DONE_INT_RAW_MASK) != 0
    }

    /// Set DCT_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dct_done_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::DCT_DONE_INT_RAW_MASK);
    }

    /// Clear DCT_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dct_done_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::DCT_DONE_INT_RAW_MASK);
    }

    /// **SR_COLOR_MODE_ERR_INT_RAW** `[10]` (RW)
    ///
    /// The raw interrupt bit to sign that the selected source color mode is not supported.
    #[inline(always)]
    pub fn sr_color_mode_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SR_COLOR_MODE_ERR_INT_RAW_MASK) != 0
    }

    /// Set SR_COLOR_MODE_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_color_mode_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SR_COLOR_MODE_ERR_INT_RAW_MASK);
    }

    /// Clear SR_COLOR_MODE_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_color_mode_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SR_COLOR_MODE_ERR_INT_RAW_MASK);
    }

    /// **OUT_EOF_INT_RAW** `[9]` (RW)
    ///
    /// The raw interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    #[inline(always)]
    pub fn out_eof_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_INT_RAW_MASK) != 0
    }

    /// Set OUT_EOF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_eof_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::OUT_EOF_INT_RAW_MASK);
    }

    /// Clear OUT_EOF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_eof_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_EOF_INT_RAW_MASK);
    }

    /// **RST_CHECK_POS_ERR_INT_RAW** `[8]` (RW)
    ///
    /// The raw interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    #[inline(always)]
    pub fn rst_check_pos_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RST_CHECK_POS_ERR_INT_RAW_MASK) != 0
    }

    /// Set RST_CHECK_POS_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_check_pos_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RST_CHECK_POS_ERR_INT_RAW_MASK);
    }

    /// Clear RST_CHECK_POS_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_check_pos_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RST_CHECK_POS_ERR_INT_RAW_MASK);
    }

    /// **RST_CHECK_NONE_ERR_INT_RAW** `[7]` (RW)
    ///
    /// The raw interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    #[inline(always)]
    pub fn rst_check_none_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RST_CHECK_NONE_ERR_INT_RAW_MASK) != 0
    }

    /// Set RST_CHECK_NONE_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_check_none_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RST_CHECK_NONE_ERR_INT_RAW_MASK);
    }

    /// Clear RST_CHECK_NONE_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_check_none_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RST_CHECK_NONE_ERR_INT_RAW_MASK);
    }

    /// **RST_UXP_ERR_INT_RAW** `[6]` (RW)
    ///
    /// The raw interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    #[inline(always)]
    pub fn rst_uxp_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RST_UXP_ERR_INT_RAW_MASK) != 0
    }

    /// Set RST_UXP_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_uxp_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RST_UXP_ERR_INT_RAW_MASK);
    }

    /// Clear RST_UXP_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_uxp_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RST_UXP_ERR_INT_RAW_MASK);
    }

    /// **C_DQT_ID_ERR_INT_RAW** `[5]` (RW)
    ///
    /// The raw interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    #[inline(always)]
    pub fn c_dqt_id_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::C_DQT_ID_ERR_INT_RAW_MASK) != 0
    }

    /// Set C_DQT_ID_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_c_dqt_id_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::C_DQT_ID_ERR_INT_RAW_MASK);
    }

    /// Clear C_DQT_ID_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_c_dqt_id_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::C_DQT_ID_ERR_INT_RAW_MASK);
    }

    /// **C_DHT_AC_ID_ERR_INT_RAW** `[4]` (RW)
    ///
    /// The raw interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    #[inline(always)]
    pub fn c_dht_ac_id_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::C_DHT_AC_ID_ERR_INT_RAW_MASK) != 0
    }

    /// Set C_DHT_AC_ID_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_c_dht_ac_id_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::C_DHT_AC_ID_ERR_INT_RAW_MASK);
    }

    /// Clear C_DHT_AC_ID_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_c_dht_ac_id_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::C_DHT_AC_ID_ERR_INT_RAW_MASK);
    }

    /// **C_DHT_DC_ID_ERR_INT_RAW** `[3]` (RW)
    ///
    /// The raw interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    #[inline(always)]
    pub fn c_dht_dc_id_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::C_DHT_DC_ID_ERR_INT_RAW_MASK) != 0
    }

    /// Set C_DHT_DC_ID_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_c_dht_dc_id_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::C_DHT_DC_ID_ERR_INT_RAW_MASK);
    }

    /// Clear C_DHT_DC_ID_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_c_dht_dc_id_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::C_DHT_DC_ID_ERR_INT_RAW_MASK);
    }

    /// **CID_ERR_INT_RAW** `[2]` (RW)
    ///
    /// The raw interrupt bit to sign that scan id check with component fails when decoding.
    #[inline(always)]
    pub fn cid_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CID_ERR_INT_RAW_MASK) != 0
    }

    /// Set CID_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cid_err_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::CID_ERR_INT_RAW_MASK);
    }

    /// Clear CID_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cid_err_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::CID_ERR_INT_RAW_MASK);
    }

    /// **RLE_PARALLEL_ERR_INT_RAW** `[1]` (RW)
    ///
    /// The raw interrupt bit to sign that rle parallel error when decoding.
    #[inline(always)]
    pub fn rle_parallel_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RLE_PARALLEL_ERR_INT_RAW_MASK) != 0
    }

    /// Set RLE_PARALLEL_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rle_parallel_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RLE_PARALLEL_ERR_INT_RAW_MASK);
    }

    /// Clear RLE_PARALLEL_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rle_parallel_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RLE_PARALLEL_ERR_INT_RAW_MASK);
    }

    /// **DONE_INT_RAW** `[0]` (RW)
    ///
    /// This raw interrupt bit turns to high level when JPEG finishes encoding a picture..
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

    /// Read **DECODE_TIMEOUT_INT_RAW** `[24:24]`
    ///
    /// The raw interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    #[inline(always)]
    pub const fn decode_timeout_int_raw(&self) -> bool {
        (self.bits & IntRaw::DECODE_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Read **UNDET_INT_RAW** `[23:23]`
    ///
    /// The raw interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    #[inline(always)]
    pub const fn undet_int_raw(&self) -> bool {
        (self.bits & IntRaw::UNDET_INT_RAW_MASK) != 0
    }

    /// Read **MARKER_ERR_OTHER_SCAN_INT_RAW** `[22:22]`
    ///
    /// The raw interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    #[inline(always)]
    pub const fn marker_err_other_scan_int_raw(&self) -> bool {
        (self.bits & IntRaw::MARKER_ERR_OTHER_SCAN_INT_RAW_MASK) != 0
    }

    /// Read **MARKER_ERR_FST_SCAN_INT_RAW** `[21:21]`
    ///
    /// The raw interrupt bit to sign that the first scan has header marker error when decoding.
    #[inline(always)]
    pub const fn marker_err_fst_scan_int_raw(&self) -> bool {
        (self.bits & IntRaw::MARKER_ERR_FST_SCAN_INT_RAW_MASK) != 0
    }

    /// Read **SOS_UNMATCH_ERR_INT_RAW** `[20:20]`
    ///
    /// The raw interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    #[inline(always)]
    pub const fn sos_unmatch_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::SOS_UNMATCH_ERR_INT_RAW_MASK) != 0
    }

    /// Read **DE_FRAME_EOF_LACK_INT_RAW** `[19:19]`
    ///
    /// The raw interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    #[inline(always)]
    pub const fn de_frame_eof_lack_int_raw(&self) -> bool {
        (self.bits & IntRaw::DE_FRAME_EOF_LACK_INT_RAW_MASK) != 0
    }

    /// Read **DE_FRAME_EOF_ERR_INT_RAW** `[18:18]`
    ///
    /// The raw interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    #[inline(always)]
    pub const fn de_frame_eof_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::DE_FRAME_EOF_ERR_INT_RAW_MASK) != 0
    }

    /// Read **EN_FRAME_EOF_LACK_INT_RAW** `[17:17]`
    ///
    /// The raw interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    #[inline(always)]
    pub const fn en_frame_eof_lack_int_raw(&self) -> bool {
        (self.bits & IntRaw::EN_FRAME_EOF_LACK_INT_RAW_MASK) != 0
    }

    /// Read **EN_FRAME_EOF_ERR_INT_RAW** `[16:16]`
    ///
    /// The raw interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    #[inline(always)]
    pub const fn en_frame_eof_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::EN_FRAME_EOF_ERR_INT_RAW_MASK) != 0
    }

    /// Read **UXP_DET_INT_RAW** `[15:15]`
    ///
    /// The raw interrupt bit to sign that unsupported header marker is detected when decoding.
    #[inline(always)]
    pub const fn uxp_det_int_raw(&self) -> bool {
        (self.bits & IntRaw::UXP_DET_INT_RAW_MASK) != 0
    }

    /// Read **SCAN_CHECK_POS_ERR_INT_RAW** `[14:14]`
    ///
    /// The raw interrupt bit to sign that SOS header marker position wrong when decoding.
    #[inline(always)]
    pub const fn scan_check_pos_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::SCAN_CHECK_POS_ERR_INT_RAW_MASK) != 0
    }

    /// Read **SCAN_CHECK_NONE_ERR_INT_RAW** `[13:13]`
    ///
    /// The raw interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    #[inline(always)]
    pub const fn scan_check_none_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::SCAN_CHECK_NONE_ERR_INT_RAW_MASK) != 0
    }

    /// Read **BS_LAST_BLOCK_EOF_INT_RAW** `[12:12]`
    ///
    /// The raw interrupt bit to sign that the coding process for last block is finished.
    #[inline(always)]
    pub const fn bs_last_block_eof_int_raw(&self) -> bool {
        (self.bits & IntRaw::BS_LAST_BLOCK_EOF_INT_RAW_MASK) != 0
    }

    /// Read **DCT_DONE_INT_RAW** `[11:11]`
    ///
    /// The raw interrupt bit to sign that one dct calculation is finished.
    #[inline(always)]
    pub const fn dct_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::DCT_DONE_INT_RAW_MASK) != 0
    }

    /// Read **SR_COLOR_MODE_ERR_INT_RAW** `[10:10]`
    ///
    /// The raw interrupt bit to sign that the selected source color mode is not supported.
    #[inline(always)]
    pub const fn sr_color_mode_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::SR_COLOR_MODE_ERR_INT_RAW_MASK) != 0
    }

    /// Read **OUT_EOF_INT_RAW** `[9:9]`
    ///
    /// The raw interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    #[inline(always)]
    pub const fn out_eof_int_raw(&self) -> bool {
        (self.bits & IntRaw::OUT_EOF_INT_RAW_MASK) != 0
    }

    /// Read **RST_CHECK_POS_ERR_INT_RAW** `[8:8]`
    ///
    /// The raw interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    #[inline(always)]
    pub const fn rst_check_pos_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::RST_CHECK_POS_ERR_INT_RAW_MASK) != 0
    }

    /// Read **RST_CHECK_NONE_ERR_INT_RAW** `[7:7]`
    ///
    /// The raw interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    #[inline(always)]
    pub const fn rst_check_none_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::RST_CHECK_NONE_ERR_INT_RAW_MASK) != 0
    }

    /// Read **RST_UXP_ERR_INT_RAW** `[6:6]`
    ///
    /// The raw interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    #[inline(always)]
    pub const fn rst_uxp_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::RST_UXP_ERR_INT_RAW_MASK) != 0
    }

    /// Read **C_DQT_ID_ERR_INT_RAW** `[5:5]`
    ///
    /// The raw interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    #[inline(always)]
    pub const fn c_dqt_id_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::C_DQT_ID_ERR_INT_RAW_MASK) != 0
    }

    /// Read **C_DHT_AC_ID_ERR_INT_RAW** `[4:4]`
    ///
    /// The raw interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    #[inline(always)]
    pub const fn c_dht_ac_id_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::C_DHT_AC_ID_ERR_INT_RAW_MASK) != 0
    }

    /// Read **C_DHT_DC_ID_ERR_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    #[inline(always)]
    pub const fn c_dht_dc_id_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::C_DHT_DC_ID_ERR_INT_RAW_MASK) != 0
    }

    /// Read **CID_ERR_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit to sign that scan id check with component fails when decoding.
    #[inline(always)]
    pub const fn cid_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::CID_ERR_INT_RAW_MASK) != 0
    }

    /// Read **RLE_PARALLEL_ERR_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit to sign that rle parallel error when decoding.
    #[inline(always)]
    pub const fn rle_parallel_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::RLE_PARALLEL_ERR_INT_RAW_MASK) != 0
    }

    /// Read **DONE_INT_RAW** `[0:0]`
    ///
    /// This raw interrupt bit turns to high level when JPEG finishes encoding a picture..
    #[inline(always)]
    pub const fn done_int_raw(&self) -> bool {
        (self.bits & IntRaw::DONE_INT_RAW_MASK) != 0
    }

    /// Set **DECODE_TIMEOUT_INT_RAW** `[24:24]`
    ///
    /// The raw interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    #[inline(always)]
    pub const fn with_decode_timeout_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DECODE_TIMEOUT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DECODE_TIMEOUT_INT_RAW_MASK
            },
        }
    }

    /// Set **UNDET_INT_RAW** `[23:23]`
    ///
    /// The raw interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    #[inline(always)]
    pub const fn with_undet_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::UNDET_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::UNDET_INT_RAW_MASK
            },
        }
    }

    /// Set **MARKER_ERR_OTHER_SCAN_INT_RAW** `[22:22]`
    ///
    /// The raw interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    #[inline(always)]
    pub const fn with_marker_err_other_scan_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::MARKER_ERR_OTHER_SCAN_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::MARKER_ERR_OTHER_SCAN_INT_RAW_MASK
            },
        }
    }

    /// Set **MARKER_ERR_FST_SCAN_INT_RAW** `[21:21]`
    ///
    /// The raw interrupt bit to sign that the first scan has header marker error when decoding.
    #[inline(always)]
    pub const fn with_marker_err_fst_scan_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::MARKER_ERR_FST_SCAN_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::MARKER_ERR_FST_SCAN_INT_RAW_MASK
            },
        }
    }

    /// Set **SOS_UNMATCH_ERR_INT_RAW** `[20:20]`
    ///
    /// The raw interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    #[inline(always)]
    pub const fn with_sos_unmatch_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SOS_UNMATCH_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SOS_UNMATCH_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **DE_FRAME_EOF_LACK_INT_RAW** `[19:19]`
    ///
    /// The raw interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    #[inline(always)]
    pub const fn with_de_frame_eof_lack_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DE_FRAME_EOF_LACK_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DE_FRAME_EOF_LACK_INT_RAW_MASK
            },
        }
    }

    /// Set **DE_FRAME_EOF_ERR_INT_RAW** `[18:18]`
    ///
    /// The raw interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    #[inline(always)]
    pub const fn with_de_frame_eof_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DE_FRAME_EOF_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DE_FRAME_EOF_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **EN_FRAME_EOF_LACK_INT_RAW** `[17:17]`
    ///
    /// The raw interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    #[inline(always)]
    pub const fn with_en_frame_eof_lack_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::EN_FRAME_EOF_LACK_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::EN_FRAME_EOF_LACK_INT_RAW_MASK
            },
        }
    }

    /// Set **EN_FRAME_EOF_ERR_INT_RAW** `[16:16]`
    ///
    /// The raw interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    #[inline(always)]
    pub const fn with_en_frame_eof_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::EN_FRAME_EOF_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::EN_FRAME_EOF_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **UXP_DET_INT_RAW** `[15:15]`
    ///
    /// The raw interrupt bit to sign that unsupported header marker is detected when decoding.
    #[inline(always)]
    pub const fn with_uxp_det_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::UXP_DET_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::UXP_DET_INT_RAW_MASK
            },
        }
    }

    /// Set **SCAN_CHECK_POS_ERR_INT_RAW** `[14:14]`
    ///
    /// The raw interrupt bit to sign that SOS header marker position wrong when decoding.
    #[inline(always)]
    pub const fn with_scan_check_pos_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SCAN_CHECK_POS_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SCAN_CHECK_POS_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **SCAN_CHECK_NONE_ERR_INT_RAW** `[13:13]`
    ///
    /// The raw interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    #[inline(always)]
    pub const fn with_scan_check_none_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SCAN_CHECK_NONE_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SCAN_CHECK_NONE_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **BS_LAST_BLOCK_EOF_INT_RAW** `[12:12]`
    ///
    /// The raw interrupt bit to sign that the coding process for last block is finished.
    #[inline(always)]
    pub const fn with_bs_last_block_eof_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::BS_LAST_BLOCK_EOF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::BS_LAST_BLOCK_EOF_INT_RAW_MASK
            },
        }
    }

    /// Set **DCT_DONE_INT_RAW** `[11:11]`
    ///
    /// The raw interrupt bit to sign that one dct calculation is finished.
    #[inline(always)]
    pub const fn with_dct_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DCT_DONE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DCT_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **SR_COLOR_MODE_ERR_INT_RAW** `[10:10]`
    ///
    /// The raw interrupt bit to sign that the selected source color mode is not supported.
    #[inline(always)]
    pub const fn with_sr_color_mode_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SR_COLOR_MODE_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SR_COLOR_MODE_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **OUT_EOF_INT_RAW** `[9:9]`
    ///
    /// The raw interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    #[inline(always)]
    pub const fn with_out_eof_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::OUT_EOF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::OUT_EOF_INT_RAW_MASK
            },
        }
    }

    /// Set **RST_CHECK_POS_ERR_INT_RAW** `[8:8]`
    ///
    /// The raw interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    #[inline(always)]
    pub const fn with_rst_check_pos_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RST_CHECK_POS_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RST_CHECK_POS_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **RST_CHECK_NONE_ERR_INT_RAW** `[7:7]`
    ///
    /// The raw interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    #[inline(always)]
    pub const fn with_rst_check_none_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RST_CHECK_NONE_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RST_CHECK_NONE_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **RST_UXP_ERR_INT_RAW** `[6:6]`
    ///
    /// The raw interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    #[inline(always)]
    pub const fn with_rst_uxp_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RST_UXP_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RST_UXP_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **C_DQT_ID_ERR_INT_RAW** `[5:5]`
    ///
    /// The raw interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    #[inline(always)]
    pub const fn with_c_dqt_id_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::C_DQT_ID_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::C_DQT_ID_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **C_DHT_AC_ID_ERR_INT_RAW** `[4:4]`
    ///
    /// The raw interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    #[inline(always)]
    pub const fn with_c_dht_ac_id_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::C_DHT_AC_ID_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::C_DHT_AC_ID_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **C_DHT_DC_ID_ERR_INT_RAW** `[3:3]`
    ///
    /// The raw interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    #[inline(always)]
    pub const fn with_c_dht_dc_id_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::C_DHT_DC_ID_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::C_DHT_DC_ID_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **CID_ERR_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt bit to sign that scan id check with component fails when decoding.
    #[inline(always)]
    pub const fn with_cid_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CID_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CID_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **RLE_PARALLEL_ERR_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt bit to sign that rle parallel error when decoding.
    #[inline(always)]
    pub const fn with_rle_parallel_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RLE_PARALLEL_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RLE_PARALLEL_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **DONE_INT_RAW** `[0:0]`
    ///
    /// This raw interrupt bit turns to high level when JPEG finishes encoding a picture..
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

/// INT_ENA Register
///
/// Interrupt enable registers
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **DECODE_TIMEOUT_INT_ENA** `[24]` (RW)
///
///   The enable interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
///
/// - **UNDET_INT_ENA** `[23]` (RW)
///
///   The enable interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
///
/// - **MARKER_ERR_OTHER_SCAN_INT_ENA** `[22]` (RW)
///
///   The enable interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
///
/// - **MARKER_ERR_FST_SCAN_INT_ENA** `[21]` (RW)
///
///   The enable interrupt bit to sign that the first scan has header marker error when decoding.
///
/// - **SOS_UNMATCH_ERR_INT_ENA** `[20]` (RW)
///
///   The enable interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
///
/// - **DE_FRAME_EOF_LACK_INT_ENA** `[19]` (RW)
///
///   The enable interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
///
/// - **DE_FRAME_EOF_ERR_INT_ENA** `[18]` (RW)
///
///   The enable interrupt bit to sign that decoded blocks are smaller than expected when decoding.
///
/// - **EN_FRAME_EOF_LACK_INT_ENA** `[17]` (RW)
///
///   The enable interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
///
/// - **EN_FRAME_EOF_ERR_INT_ENA** `[16]` (RW)
///
///   The enable interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
///
/// - **UXP_DET_INT_ENA** `[15]` (RW)
///
///   The enable interrupt bit to sign that unsupported header marker is detected when decoding.
///
/// - **SCAN_CHECK_POS_ERR_INT_ENA** `[14]` (RW)
///
///   The enable interrupt bit to sign that SOS header marker position wrong when decoding.
///
/// - **SCAN_CHECK_NONE_ERR_INT_ENA** `[13]` (RW)
///
///   The enable interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
///
/// - **BS_LAST_BLOCK_EOF_INT_ENA** `[12]` (RW)
///
///   The enable interrupt bit to sign that the coding process for last block is finished.
///
/// - **DCT_DONE_INT_ENA** `[11]` (RW)
///
///   The enable interrupt bit to sign that one dct calculation is finished.
///
/// - **SR_COLOR_MODE_ERR_INT_ENA** `[10]` (RW)
///
///   The enable interrupt bit to sign that the selected source color mode is not supported.
///
/// - **OUT_EOF_INT_ENA** `[9]` (RW)
///
///   The enable interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
///
/// - **RST_CHECK_POS_ERR_INT_ENA** `[8]` (RW)
///
///   The enable interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
///
/// - **RST_CHECK_NONE_ERR_INT_ENA** `[7]` (RW)
///
///   The enable interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
///
/// - **RST_UXP_ERR_INT_ENA** `[6]` (RW)
///
///   The enable interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
///
/// - **C_DQT_ID_ERR_INT_ENA** `[5]` (RW)
///
///   The enable interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
///
/// - **C_DHT_AC_ID_ERR_INT_ENA** `[4]` (RW)
///
///   The enable interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
///
/// - **C_DHT_DC_ID_ERR_INT_ENA** `[3]` (RW)
///
///   The enable interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
///
/// - **CID_ERR_INT_ENA** `[2]` (RW)
///
///   The enable interrupt bit to sign that scan id check with component fails when decoding.
///
/// - **RLE_PARALLEL_ERR_INT_ENA** `[1]` (RW)
///
///   The enable interrupt bit to sign that rle parallel error when decoding.
///
/// - **DONE_INT_ENA** `[0]` (RW)
///
///   This enable interrupt bit turns to high level when JPEG finishes encoding a picture..
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

    /// **DECODE_TIMEOUT_INT_ENA**
    ///
    /// The enable interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const DECODE_TIMEOUT_INT_ENA_OFFSET: u32 = 24;
    pub const DECODE_TIMEOUT_INT_ENA_MASK: u32 = 0x01000000;
    pub const DECODE_TIMEOUT_INT_ENA: u32 = 1 << 24;

    /// **UNDET_INT_ENA**
    ///
    /// The enable interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const UNDET_INT_ENA_OFFSET: u32 = 23;
    pub const UNDET_INT_ENA_MASK: u32 = 0x00800000;
    pub const UNDET_INT_ENA: u32 = 1 << 23;

    /// **MARKER_ERR_OTHER_SCAN_INT_ENA**
    ///
    /// The enable interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const MARKER_ERR_OTHER_SCAN_INT_ENA_OFFSET: u32 = 22;
    pub const MARKER_ERR_OTHER_SCAN_INT_ENA_MASK: u32 = 0x00400000;
    pub const MARKER_ERR_OTHER_SCAN_INT_ENA: u32 = 1 << 22;

    /// **MARKER_ERR_FST_SCAN_INT_ENA**
    ///
    /// The enable interrupt bit to sign that the first scan has header marker error when decoding.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const MARKER_ERR_FST_SCAN_INT_ENA_OFFSET: u32 = 21;
    pub const MARKER_ERR_FST_SCAN_INT_ENA_MASK: u32 = 0x00200000;
    pub const MARKER_ERR_FST_SCAN_INT_ENA: u32 = 1 << 21;

    /// **SOS_UNMATCH_ERR_INT_ENA**
    ///
    /// The enable interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SOS_UNMATCH_ERR_INT_ENA_OFFSET: u32 = 20;
    pub const SOS_UNMATCH_ERR_INT_ENA_MASK: u32 = 0x00100000;
    pub const SOS_UNMATCH_ERR_INT_ENA: u32 = 1 << 20;

    /// **DE_FRAME_EOF_LACK_INT_ENA**
    ///
    /// The enable interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const DE_FRAME_EOF_LACK_INT_ENA_OFFSET: u32 = 19;
    pub const DE_FRAME_EOF_LACK_INT_ENA_MASK: u32 = 0x00080000;
    pub const DE_FRAME_EOF_LACK_INT_ENA: u32 = 1 << 19;

    /// **DE_FRAME_EOF_ERR_INT_ENA**
    ///
    /// The enable interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const DE_FRAME_EOF_ERR_INT_ENA_OFFSET: u32 = 18;
    pub const DE_FRAME_EOF_ERR_INT_ENA_MASK: u32 = 0x00040000;
    pub const DE_FRAME_EOF_ERR_INT_ENA: u32 = 1 << 18;

    /// **EN_FRAME_EOF_LACK_INT_ENA**
    ///
    /// The enable interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const EN_FRAME_EOF_LACK_INT_ENA_OFFSET: u32 = 17;
    pub const EN_FRAME_EOF_LACK_INT_ENA_MASK: u32 = 0x00020000;
    pub const EN_FRAME_EOF_LACK_INT_ENA: u32 = 1 << 17;

    /// **EN_FRAME_EOF_ERR_INT_ENA**
    ///
    /// The enable interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const EN_FRAME_EOF_ERR_INT_ENA_OFFSET: u32 = 16;
    pub const EN_FRAME_EOF_ERR_INT_ENA_MASK: u32 = 0x00010000;
    pub const EN_FRAME_EOF_ERR_INT_ENA: u32 = 1 << 16;

    /// **UXP_DET_INT_ENA**
    ///
    /// The enable interrupt bit to sign that unsupported header marker is detected when decoding.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const UXP_DET_INT_ENA_OFFSET: u32 = 15;
    pub const UXP_DET_INT_ENA_MASK: u32 = 0x00008000;
    pub const UXP_DET_INT_ENA: u32 = 1 << 15;

    /// **SCAN_CHECK_POS_ERR_INT_ENA**
    ///
    /// The enable interrupt bit to sign that SOS header marker position wrong when decoding.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SCAN_CHECK_POS_ERR_INT_ENA_OFFSET: u32 = 14;
    pub const SCAN_CHECK_POS_ERR_INT_ENA_MASK: u32 = 0x00004000;
    pub const SCAN_CHECK_POS_ERR_INT_ENA: u32 = 1 << 14;

    /// **SCAN_CHECK_NONE_ERR_INT_ENA**
    ///
    /// The enable interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SCAN_CHECK_NONE_ERR_INT_ENA_OFFSET: u32 = 13;
    pub const SCAN_CHECK_NONE_ERR_INT_ENA_MASK: u32 = 0x00002000;
    pub const SCAN_CHECK_NONE_ERR_INT_ENA: u32 = 1 << 13;

    /// **BS_LAST_BLOCK_EOF_INT_ENA**
    ///
    /// The enable interrupt bit to sign that the coding process for last block is finished.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const BS_LAST_BLOCK_EOF_INT_ENA_OFFSET: u32 = 12;
    pub const BS_LAST_BLOCK_EOF_INT_ENA_MASK: u32 = 0x00001000;
    pub const BS_LAST_BLOCK_EOF_INT_ENA: u32 = 1 << 12;

    /// **DCT_DONE_INT_ENA**
    ///
    /// The enable interrupt bit to sign that one dct calculation is finished.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const DCT_DONE_INT_ENA_OFFSET: u32 = 11;
    pub const DCT_DONE_INT_ENA_MASK: u32 = 0x00000800;
    pub const DCT_DONE_INT_ENA: u32 = 1 << 11;

    /// **SR_COLOR_MODE_ERR_INT_ENA**
    ///
    /// The enable interrupt bit to sign that the selected source color mode is not supported.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SR_COLOR_MODE_ERR_INT_ENA_OFFSET: u32 = 10;
    pub const SR_COLOR_MODE_ERR_INT_ENA_MASK: u32 = 0x00000400;
    pub const SR_COLOR_MODE_ERR_INT_ENA: u32 = 1 << 10;

    /// **OUT_EOF_INT_ENA**
    ///
    /// The enable interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const OUT_EOF_INT_ENA_OFFSET: u32 = 9;
    pub const OUT_EOF_INT_ENA_MASK: u32 = 0x00000200;
    pub const OUT_EOF_INT_ENA: u32 = 1 << 9;

    /// **RST_CHECK_POS_ERR_INT_ENA**
    ///
    /// The enable interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RST_CHECK_POS_ERR_INT_ENA_OFFSET: u32 = 8;
    pub const RST_CHECK_POS_ERR_INT_ENA_MASK: u32 = 0x00000100;
    pub const RST_CHECK_POS_ERR_INT_ENA: u32 = 1 << 8;

    /// **RST_CHECK_NONE_ERR_INT_ENA**
    ///
    /// The enable interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RST_CHECK_NONE_ERR_INT_ENA_OFFSET: u32 = 7;
    pub const RST_CHECK_NONE_ERR_INT_ENA_MASK: u32 = 0x00000080;
    pub const RST_CHECK_NONE_ERR_INT_ENA: u32 = 1 << 7;

    /// **RST_UXP_ERR_INT_ENA**
    ///
    /// The enable interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RST_UXP_ERR_INT_ENA_OFFSET: u32 = 6;
    pub const RST_UXP_ERR_INT_ENA_MASK: u32 = 0x00000040;
    pub const RST_UXP_ERR_INT_ENA: u32 = 1 << 6;

    /// **C_DQT_ID_ERR_INT_ENA**
    ///
    /// The enable interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const C_DQT_ID_ERR_INT_ENA_OFFSET: u32 = 5;
    pub const C_DQT_ID_ERR_INT_ENA_MASK: u32 = 0x00000020;
    pub const C_DQT_ID_ERR_INT_ENA: u32 = 1 << 5;

    /// **C_DHT_AC_ID_ERR_INT_ENA**
    ///
    /// The enable interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const C_DHT_AC_ID_ERR_INT_ENA_OFFSET: u32 = 4;
    pub const C_DHT_AC_ID_ERR_INT_ENA_MASK: u32 = 0x00000010;
    pub const C_DHT_AC_ID_ERR_INT_ENA: u32 = 1 << 4;

    /// **C_DHT_DC_ID_ERR_INT_ENA**
    ///
    /// The enable interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const C_DHT_DC_ID_ERR_INT_ENA_OFFSET: u32 = 3;
    pub const C_DHT_DC_ID_ERR_INT_ENA_MASK: u32 = 0x00000008;
    pub const C_DHT_DC_ID_ERR_INT_ENA: u32 = 1 << 3;

    /// **CID_ERR_INT_ENA**
    ///
    /// The enable interrupt bit to sign that scan id check with component fails when decoding.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CID_ERR_INT_ENA_OFFSET: u32 = 2;
    pub const CID_ERR_INT_ENA_MASK: u32 = 0x00000004;
    pub const CID_ERR_INT_ENA: u32 = 1 << 2;

    /// **RLE_PARALLEL_ERR_INT_ENA**
    ///
    /// The enable interrupt bit to sign that rle parallel error when decoding.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RLE_PARALLEL_ERR_INT_ENA_OFFSET: u32 = 1;
    pub const RLE_PARALLEL_ERR_INT_ENA_MASK: u32 = 0x00000002;
    pub const RLE_PARALLEL_ERR_INT_ENA: u32 = 1 << 1;

    /// **DONE_INT_ENA**
    ///
    /// This enable interrupt bit turns to high level when JPEG finishes encoding a picture..
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DONE_INT_ENA_OFFSET: u32 = 0;
    pub const DONE_INT_ENA_MASK: u32 = 0x00000001;
    pub const DONE_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DECODE_TIMEOUT_INT_ENA** `[24]` (RW)
    ///
    /// The enable interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    #[inline(always)]
    pub fn decode_timeout_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DECODE_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Set DECODE_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_decode_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DECODE_TIMEOUT_INT_ENA_MASK);
    }

    /// Clear DECODE_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_decode_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DECODE_TIMEOUT_INT_ENA_MASK);
    }

    /// **UNDET_INT_ENA** `[23]` (RW)
    ///
    /// The enable interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    #[inline(always)]
    pub fn undet_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::UNDET_INT_ENA_MASK) != 0
    }

    /// Set UNDET_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_undet_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::UNDET_INT_ENA_MASK);
    }

    /// Clear UNDET_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_undet_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::UNDET_INT_ENA_MASK);
    }

    /// **MARKER_ERR_OTHER_SCAN_INT_ENA** `[22]` (RW)
    ///
    /// The enable interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    #[inline(always)]
    pub fn marker_err_other_scan_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::MARKER_ERR_OTHER_SCAN_INT_ENA_MASK) != 0
    }

    /// Set MARKER_ERR_OTHER_SCAN_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_marker_err_other_scan_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::MARKER_ERR_OTHER_SCAN_INT_ENA_MASK);
    }

    /// Clear MARKER_ERR_OTHER_SCAN_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_marker_err_other_scan_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::MARKER_ERR_OTHER_SCAN_INT_ENA_MASK);
    }

    /// **MARKER_ERR_FST_SCAN_INT_ENA** `[21]` (RW)
    ///
    /// The enable interrupt bit to sign that the first scan has header marker error when decoding.
    #[inline(always)]
    pub fn marker_err_fst_scan_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::MARKER_ERR_FST_SCAN_INT_ENA_MASK) != 0
    }

    /// Set MARKER_ERR_FST_SCAN_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_marker_err_fst_scan_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::MARKER_ERR_FST_SCAN_INT_ENA_MASK);
    }

    /// Clear MARKER_ERR_FST_SCAN_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_marker_err_fst_scan_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::MARKER_ERR_FST_SCAN_INT_ENA_MASK);
    }

    /// **SOS_UNMATCH_ERR_INT_ENA** `[20]` (RW)
    ///
    /// The enable interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    #[inline(always)]
    pub fn sos_unmatch_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SOS_UNMATCH_ERR_INT_ENA_MASK) != 0
    }

    /// Set SOS_UNMATCH_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sos_unmatch_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SOS_UNMATCH_ERR_INT_ENA_MASK);
    }

    /// Clear SOS_UNMATCH_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sos_unmatch_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SOS_UNMATCH_ERR_INT_ENA_MASK);
    }

    /// **DE_FRAME_EOF_LACK_INT_ENA** `[19]` (RW)
    ///
    /// The enable interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    #[inline(always)]
    pub fn de_frame_eof_lack_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DE_FRAME_EOF_LACK_INT_ENA_MASK) != 0
    }

    /// Set DE_FRAME_EOF_LACK_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_de_frame_eof_lack_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DE_FRAME_EOF_LACK_INT_ENA_MASK);
    }

    /// Clear DE_FRAME_EOF_LACK_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_de_frame_eof_lack_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DE_FRAME_EOF_LACK_INT_ENA_MASK);
    }

    /// **DE_FRAME_EOF_ERR_INT_ENA** `[18]` (RW)
    ///
    /// The enable interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    #[inline(always)]
    pub fn de_frame_eof_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DE_FRAME_EOF_ERR_INT_ENA_MASK) != 0
    }

    /// Set DE_FRAME_EOF_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_de_frame_eof_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DE_FRAME_EOF_ERR_INT_ENA_MASK);
    }

    /// Clear DE_FRAME_EOF_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_de_frame_eof_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DE_FRAME_EOF_ERR_INT_ENA_MASK);
    }

    /// **EN_FRAME_EOF_LACK_INT_ENA** `[17]` (RW)
    ///
    /// The enable interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    #[inline(always)]
    pub fn en_frame_eof_lack_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::EN_FRAME_EOF_LACK_INT_ENA_MASK) != 0
    }

    /// Set EN_FRAME_EOF_LACK_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en_frame_eof_lack_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::EN_FRAME_EOF_LACK_INT_ENA_MASK);
    }

    /// Clear EN_FRAME_EOF_LACK_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en_frame_eof_lack_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::EN_FRAME_EOF_LACK_INT_ENA_MASK);
    }

    /// **EN_FRAME_EOF_ERR_INT_ENA** `[16]` (RW)
    ///
    /// The enable interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    #[inline(always)]
    pub fn en_frame_eof_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::EN_FRAME_EOF_ERR_INT_ENA_MASK) != 0
    }

    /// Set EN_FRAME_EOF_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en_frame_eof_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::EN_FRAME_EOF_ERR_INT_ENA_MASK);
    }

    /// Clear EN_FRAME_EOF_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en_frame_eof_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::EN_FRAME_EOF_ERR_INT_ENA_MASK);
    }

    /// **UXP_DET_INT_ENA** `[15]` (RW)
    ///
    /// The enable interrupt bit to sign that unsupported header marker is detected when decoding.
    #[inline(always)]
    pub fn uxp_det_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::UXP_DET_INT_ENA_MASK) != 0
    }

    /// Set UXP_DET_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_uxp_det_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::UXP_DET_INT_ENA_MASK);
    }

    /// Clear UXP_DET_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_uxp_det_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::UXP_DET_INT_ENA_MASK);
    }

    /// **SCAN_CHECK_POS_ERR_INT_ENA** `[14]` (RW)
    ///
    /// The enable interrupt bit to sign that SOS header marker position wrong when decoding.
    #[inline(always)]
    pub fn scan_check_pos_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SCAN_CHECK_POS_ERR_INT_ENA_MASK) != 0
    }

    /// Set SCAN_CHECK_POS_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scan_check_pos_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SCAN_CHECK_POS_ERR_INT_ENA_MASK);
    }

    /// Clear SCAN_CHECK_POS_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scan_check_pos_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SCAN_CHECK_POS_ERR_INT_ENA_MASK);
    }

    /// **SCAN_CHECK_NONE_ERR_INT_ENA** `[13]` (RW)
    ///
    /// The enable interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    #[inline(always)]
    pub fn scan_check_none_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SCAN_CHECK_NONE_ERR_INT_ENA_MASK) != 0
    }

    /// Set SCAN_CHECK_NONE_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scan_check_none_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SCAN_CHECK_NONE_ERR_INT_ENA_MASK);
    }

    /// Clear SCAN_CHECK_NONE_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scan_check_none_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SCAN_CHECK_NONE_ERR_INT_ENA_MASK);
    }

    /// **BS_LAST_BLOCK_EOF_INT_ENA** `[12]` (RW)
    ///
    /// The enable interrupt bit to sign that the coding process for last block is finished.
    #[inline(always)]
    pub fn bs_last_block_eof_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::BS_LAST_BLOCK_EOF_INT_ENA_MASK) != 0
    }

    /// Set BS_LAST_BLOCK_EOF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bs_last_block_eof_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::BS_LAST_BLOCK_EOF_INT_ENA_MASK);
    }

    /// Clear BS_LAST_BLOCK_EOF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bs_last_block_eof_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::BS_LAST_BLOCK_EOF_INT_ENA_MASK);
    }

    /// **DCT_DONE_INT_ENA** `[11]` (RW)
    ///
    /// The enable interrupt bit to sign that one dct calculation is finished.
    #[inline(always)]
    pub fn dct_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DCT_DONE_INT_ENA_MASK) != 0
    }

    /// Set DCT_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dct_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::DCT_DONE_INT_ENA_MASK);
    }

    /// Clear DCT_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dct_done_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::DCT_DONE_INT_ENA_MASK);
    }

    /// **SR_COLOR_MODE_ERR_INT_ENA** `[10]` (RW)
    ///
    /// The enable interrupt bit to sign that the selected source color mode is not supported.
    #[inline(always)]
    pub fn sr_color_mode_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SR_COLOR_MODE_ERR_INT_ENA_MASK) != 0
    }

    /// Set SR_COLOR_MODE_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sr_color_mode_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SR_COLOR_MODE_ERR_INT_ENA_MASK);
    }

    /// Clear SR_COLOR_MODE_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sr_color_mode_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SR_COLOR_MODE_ERR_INT_ENA_MASK);
    }

    /// **OUT_EOF_INT_ENA** `[9]` (RW)
    ///
    /// The enable interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    #[inline(always)]
    pub fn out_eof_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_INT_ENA_MASK) != 0
    }

    /// Set OUT_EOF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_out_eof_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::OUT_EOF_INT_ENA_MASK);
    }

    /// Clear OUT_EOF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_out_eof_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::OUT_EOF_INT_ENA_MASK);
    }

    /// **RST_CHECK_POS_ERR_INT_ENA** `[8]` (RW)
    ///
    /// The enable interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    #[inline(always)]
    pub fn rst_check_pos_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RST_CHECK_POS_ERR_INT_ENA_MASK) != 0
    }

    /// Set RST_CHECK_POS_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_check_pos_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RST_CHECK_POS_ERR_INT_ENA_MASK);
    }

    /// Clear RST_CHECK_POS_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_check_pos_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RST_CHECK_POS_ERR_INT_ENA_MASK);
    }

    /// **RST_CHECK_NONE_ERR_INT_ENA** `[7]` (RW)
    ///
    /// The enable interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    #[inline(always)]
    pub fn rst_check_none_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RST_CHECK_NONE_ERR_INT_ENA_MASK) != 0
    }

    /// Set RST_CHECK_NONE_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_check_none_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RST_CHECK_NONE_ERR_INT_ENA_MASK);
    }

    /// Clear RST_CHECK_NONE_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_check_none_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RST_CHECK_NONE_ERR_INT_ENA_MASK);
    }

    /// **RST_UXP_ERR_INT_ENA** `[6]` (RW)
    ///
    /// The enable interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    #[inline(always)]
    pub fn rst_uxp_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RST_UXP_ERR_INT_ENA_MASK) != 0
    }

    /// Set RST_UXP_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_uxp_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RST_UXP_ERR_INT_ENA_MASK);
    }

    /// Clear RST_UXP_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_uxp_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RST_UXP_ERR_INT_ENA_MASK);
    }

    /// **C_DQT_ID_ERR_INT_ENA** `[5]` (RW)
    ///
    /// The enable interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    #[inline(always)]
    pub fn c_dqt_id_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::C_DQT_ID_ERR_INT_ENA_MASK) != 0
    }

    /// Set C_DQT_ID_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_c_dqt_id_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::C_DQT_ID_ERR_INT_ENA_MASK);
    }

    /// Clear C_DQT_ID_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_c_dqt_id_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::C_DQT_ID_ERR_INT_ENA_MASK);
    }

    /// **C_DHT_AC_ID_ERR_INT_ENA** `[4]` (RW)
    ///
    /// The enable interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    #[inline(always)]
    pub fn c_dht_ac_id_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::C_DHT_AC_ID_ERR_INT_ENA_MASK) != 0
    }

    /// Set C_DHT_AC_ID_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_c_dht_ac_id_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::C_DHT_AC_ID_ERR_INT_ENA_MASK);
    }

    /// Clear C_DHT_AC_ID_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_c_dht_ac_id_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::C_DHT_AC_ID_ERR_INT_ENA_MASK);
    }

    /// **C_DHT_DC_ID_ERR_INT_ENA** `[3]` (RW)
    ///
    /// The enable interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    #[inline(always)]
    pub fn c_dht_dc_id_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::C_DHT_DC_ID_ERR_INT_ENA_MASK) != 0
    }

    /// Set C_DHT_DC_ID_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_c_dht_dc_id_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::C_DHT_DC_ID_ERR_INT_ENA_MASK);
    }

    /// Clear C_DHT_DC_ID_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_c_dht_dc_id_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::C_DHT_DC_ID_ERR_INT_ENA_MASK);
    }

    /// **CID_ERR_INT_ENA** `[2]` (RW)
    ///
    /// The enable interrupt bit to sign that scan id check with component fails when decoding.
    #[inline(always)]
    pub fn cid_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CID_ERR_INT_ENA_MASK) != 0
    }

    /// Set CID_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cid_err_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::CID_ERR_INT_ENA_MASK);
    }

    /// Clear CID_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cid_err_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::CID_ERR_INT_ENA_MASK);
    }

    /// **RLE_PARALLEL_ERR_INT_ENA** `[1]` (RW)
    ///
    /// The enable interrupt bit to sign that rle parallel error when decoding.
    #[inline(always)]
    pub fn rle_parallel_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RLE_PARALLEL_ERR_INT_ENA_MASK) != 0
    }

    /// Set RLE_PARALLEL_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rle_parallel_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RLE_PARALLEL_ERR_INT_ENA_MASK);
    }

    /// Clear RLE_PARALLEL_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rle_parallel_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RLE_PARALLEL_ERR_INT_ENA_MASK);
    }

    /// **DONE_INT_ENA** `[0]` (RW)
    ///
    /// This enable interrupt bit turns to high level when JPEG finishes encoding a picture..
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

    /// Read **DECODE_TIMEOUT_INT_ENA** `[24:24]`
    ///
    /// The enable interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    #[inline(always)]
    pub const fn decode_timeout_int_ena(&self) -> bool {
        (self.bits & IntEna::DECODE_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Read **UNDET_INT_ENA** `[23:23]`
    ///
    /// The enable interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    #[inline(always)]
    pub const fn undet_int_ena(&self) -> bool {
        (self.bits & IntEna::UNDET_INT_ENA_MASK) != 0
    }

    /// Read **MARKER_ERR_OTHER_SCAN_INT_ENA** `[22:22]`
    ///
    /// The enable interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    #[inline(always)]
    pub const fn marker_err_other_scan_int_ena(&self) -> bool {
        (self.bits & IntEna::MARKER_ERR_OTHER_SCAN_INT_ENA_MASK) != 0
    }

    /// Read **MARKER_ERR_FST_SCAN_INT_ENA** `[21:21]`
    ///
    /// The enable interrupt bit to sign that the first scan has header marker error when decoding.
    #[inline(always)]
    pub const fn marker_err_fst_scan_int_ena(&self) -> bool {
        (self.bits & IntEna::MARKER_ERR_FST_SCAN_INT_ENA_MASK) != 0
    }

    /// Read **SOS_UNMATCH_ERR_INT_ENA** `[20:20]`
    ///
    /// The enable interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    #[inline(always)]
    pub const fn sos_unmatch_err_int_ena(&self) -> bool {
        (self.bits & IntEna::SOS_UNMATCH_ERR_INT_ENA_MASK) != 0
    }

    /// Read **DE_FRAME_EOF_LACK_INT_ENA** `[19:19]`
    ///
    /// The enable interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    #[inline(always)]
    pub const fn de_frame_eof_lack_int_ena(&self) -> bool {
        (self.bits & IntEna::DE_FRAME_EOF_LACK_INT_ENA_MASK) != 0
    }

    /// Read **DE_FRAME_EOF_ERR_INT_ENA** `[18:18]`
    ///
    /// The enable interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    #[inline(always)]
    pub const fn de_frame_eof_err_int_ena(&self) -> bool {
        (self.bits & IntEna::DE_FRAME_EOF_ERR_INT_ENA_MASK) != 0
    }

    /// Read **EN_FRAME_EOF_LACK_INT_ENA** `[17:17]`
    ///
    /// The enable interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    #[inline(always)]
    pub const fn en_frame_eof_lack_int_ena(&self) -> bool {
        (self.bits & IntEna::EN_FRAME_EOF_LACK_INT_ENA_MASK) != 0
    }

    /// Read **EN_FRAME_EOF_ERR_INT_ENA** `[16:16]`
    ///
    /// The enable interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    #[inline(always)]
    pub const fn en_frame_eof_err_int_ena(&self) -> bool {
        (self.bits & IntEna::EN_FRAME_EOF_ERR_INT_ENA_MASK) != 0
    }

    /// Read **UXP_DET_INT_ENA** `[15:15]`
    ///
    /// The enable interrupt bit to sign that unsupported header marker is detected when decoding.
    #[inline(always)]
    pub const fn uxp_det_int_ena(&self) -> bool {
        (self.bits & IntEna::UXP_DET_INT_ENA_MASK) != 0
    }

    /// Read **SCAN_CHECK_POS_ERR_INT_ENA** `[14:14]`
    ///
    /// The enable interrupt bit to sign that SOS header marker position wrong when decoding.
    #[inline(always)]
    pub const fn scan_check_pos_err_int_ena(&self) -> bool {
        (self.bits & IntEna::SCAN_CHECK_POS_ERR_INT_ENA_MASK) != 0
    }

    /// Read **SCAN_CHECK_NONE_ERR_INT_ENA** `[13:13]`
    ///
    /// The enable interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    #[inline(always)]
    pub const fn scan_check_none_err_int_ena(&self) -> bool {
        (self.bits & IntEna::SCAN_CHECK_NONE_ERR_INT_ENA_MASK) != 0
    }

    /// Read **BS_LAST_BLOCK_EOF_INT_ENA** `[12:12]`
    ///
    /// The enable interrupt bit to sign that the coding process for last block is finished.
    #[inline(always)]
    pub const fn bs_last_block_eof_int_ena(&self) -> bool {
        (self.bits & IntEna::BS_LAST_BLOCK_EOF_INT_ENA_MASK) != 0
    }

    /// Read **DCT_DONE_INT_ENA** `[11:11]`
    ///
    /// The enable interrupt bit to sign that one dct calculation is finished.
    #[inline(always)]
    pub const fn dct_done_int_ena(&self) -> bool {
        (self.bits & IntEna::DCT_DONE_INT_ENA_MASK) != 0
    }

    /// Read **SR_COLOR_MODE_ERR_INT_ENA** `[10:10]`
    ///
    /// The enable interrupt bit to sign that the selected source color mode is not supported.
    #[inline(always)]
    pub const fn sr_color_mode_err_int_ena(&self) -> bool {
        (self.bits & IntEna::SR_COLOR_MODE_ERR_INT_ENA_MASK) != 0
    }

    /// Read **OUT_EOF_INT_ENA** `[9:9]`
    ///
    /// The enable interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    #[inline(always)]
    pub const fn out_eof_int_ena(&self) -> bool {
        (self.bits & IntEna::OUT_EOF_INT_ENA_MASK) != 0
    }

    /// Read **RST_CHECK_POS_ERR_INT_ENA** `[8:8]`
    ///
    /// The enable interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    #[inline(always)]
    pub const fn rst_check_pos_err_int_ena(&self) -> bool {
        (self.bits & IntEna::RST_CHECK_POS_ERR_INT_ENA_MASK) != 0
    }

    /// Read **RST_CHECK_NONE_ERR_INT_ENA** `[7:7]`
    ///
    /// The enable interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    #[inline(always)]
    pub const fn rst_check_none_err_int_ena(&self) -> bool {
        (self.bits & IntEna::RST_CHECK_NONE_ERR_INT_ENA_MASK) != 0
    }

    /// Read **RST_UXP_ERR_INT_ENA** `[6:6]`
    ///
    /// The enable interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    #[inline(always)]
    pub const fn rst_uxp_err_int_ena(&self) -> bool {
        (self.bits & IntEna::RST_UXP_ERR_INT_ENA_MASK) != 0
    }

    /// Read **C_DQT_ID_ERR_INT_ENA** `[5:5]`
    ///
    /// The enable interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    #[inline(always)]
    pub const fn c_dqt_id_err_int_ena(&self) -> bool {
        (self.bits & IntEna::C_DQT_ID_ERR_INT_ENA_MASK) != 0
    }

    /// Read **C_DHT_AC_ID_ERR_INT_ENA** `[4:4]`
    ///
    /// The enable interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    #[inline(always)]
    pub const fn c_dht_ac_id_err_int_ena(&self) -> bool {
        (self.bits & IntEna::C_DHT_AC_ID_ERR_INT_ENA_MASK) != 0
    }

    /// Read **C_DHT_DC_ID_ERR_INT_ENA** `[3:3]`
    ///
    /// The enable interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    #[inline(always)]
    pub const fn c_dht_dc_id_err_int_ena(&self) -> bool {
        (self.bits & IntEna::C_DHT_DC_ID_ERR_INT_ENA_MASK) != 0
    }

    /// Read **CID_ERR_INT_ENA** `[2:2]`
    ///
    /// The enable interrupt bit to sign that scan id check with component fails when decoding.
    #[inline(always)]
    pub const fn cid_err_int_ena(&self) -> bool {
        (self.bits & IntEna::CID_ERR_INT_ENA_MASK) != 0
    }

    /// Read **RLE_PARALLEL_ERR_INT_ENA** `[1:1]`
    ///
    /// The enable interrupt bit to sign that rle parallel error when decoding.
    #[inline(always)]
    pub const fn rle_parallel_err_int_ena(&self) -> bool {
        (self.bits & IntEna::RLE_PARALLEL_ERR_INT_ENA_MASK) != 0
    }

    /// Read **DONE_INT_ENA** `[0:0]`
    ///
    /// This enable interrupt bit turns to high level when JPEG finishes encoding a picture..
    #[inline(always)]
    pub const fn done_int_ena(&self) -> bool {
        (self.bits & IntEna::DONE_INT_ENA_MASK) != 0
    }

    /// Set **DECODE_TIMEOUT_INT_ENA** `[24:24]`
    ///
    /// The enable interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    #[inline(always)]
    pub const fn with_decode_timeout_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DECODE_TIMEOUT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DECODE_TIMEOUT_INT_ENA_MASK
            },
        }
    }

    /// Set **UNDET_INT_ENA** `[23:23]`
    ///
    /// The enable interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    #[inline(always)]
    pub const fn with_undet_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::UNDET_INT_ENA_MASK
            } else {
                self.bits & !IntEna::UNDET_INT_ENA_MASK
            },
        }
    }

    /// Set **MARKER_ERR_OTHER_SCAN_INT_ENA** `[22:22]`
    ///
    /// The enable interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    #[inline(always)]
    pub const fn with_marker_err_other_scan_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::MARKER_ERR_OTHER_SCAN_INT_ENA_MASK
            } else {
                self.bits & !IntEna::MARKER_ERR_OTHER_SCAN_INT_ENA_MASK
            },
        }
    }

    /// Set **MARKER_ERR_FST_SCAN_INT_ENA** `[21:21]`
    ///
    /// The enable interrupt bit to sign that the first scan has header marker error when decoding.
    #[inline(always)]
    pub const fn with_marker_err_fst_scan_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::MARKER_ERR_FST_SCAN_INT_ENA_MASK
            } else {
                self.bits & !IntEna::MARKER_ERR_FST_SCAN_INT_ENA_MASK
            },
        }
    }

    /// Set **SOS_UNMATCH_ERR_INT_ENA** `[20:20]`
    ///
    /// The enable interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    #[inline(always)]
    pub const fn with_sos_unmatch_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SOS_UNMATCH_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SOS_UNMATCH_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **DE_FRAME_EOF_LACK_INT_ENA** `[19:19]`
    ///
    /// The enable interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    #[inline(always)]
    pub const fn with_de_frame_eof_lack_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DE_FRAME_EOF_LACK_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DE_FRAME_EOF_LACK_INT_ENA_MASK
            },
        }
    }

    /// Set **DE_FRAME_EOF_ERR_INT_ENA** `[18:18]`
    ///
    /// The enable interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    #[inline(always)]
    pub const fn with_de_frame_eof_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DE_FRAME_EOF_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DE_FRAME_EOF_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **EN_FRAME_EOF_LACK_INT_ENA** `[17:17]`
    ///
    /// The enable interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    #[inline(always)]
    pub const fn with_en_frame_eof_lack_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::EN_FRAME_EOF_LACK_INT_ENA_MASK
            } else {
                self.bits & !IntEna::EN_FRAME_EOF_LACK_INT_ENA_MASK
            },
        }
    }

    /// Set **EN_FRAME_EOF_ERR_INT_ENA** `[16:16]`
    ///
    /// The enable interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    #[inline(always)]
    pub const fn with_en_frame_eof_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::EN_FRAME_EOF_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::EN_FRAME_EOF_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **UXP_DET_INT_ENA** `[15:15]`
    ///
    /// The enable interrupt bit to sign that unsupported header marker is detected when decoding.
    #[inline(always)]
    pub const fn with_uxp_det_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::UXP_DET_INT_ENA_MASK
            } else {
                self.bits & !IntEna::UXP_DET_INT_ENA_MASK
            },
        }
    }

    /// Set **SCAN_CHECK_POS_ERR_INT_ENA** `[14:14]`
    ///
    /// The enable interrupt bit to sign that SOS header marker position wrong when decoding.
    #[inline(always)]
    pub const fn with_scan_check_pos_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SCAN_CHECK_POS_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SCAN_CHECK_POS_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **SCAN_CHECK_NONE_ERR_INT_ENA** `[13:13]`
    ///
    /// The enable interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    #[inline(always)]
    pub const fn with_scan_check_none_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SCAN_CHECK_NONE_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SCAN_CHECK_NONE_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **BS_LAST_BLOCK_EOF_INT_ENA** `[12:12]`
    ///
    /// The enable interrupt bit to sign that the coding process for last block is finished.
    #[inline(always)]
    pub const fn with_bs_last_block_eof_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::BS_LAST_BLOCK_EOF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::BS_LAST_BLOCK_EOF_INT_ENA_MASK
            },
        }
    }

    /// Set **DCT_DONE_INT_ENA** `[11:11]`
    ///
    /// The enable interrupt bit to sign that one dct calculation is finished.
    #[inline(always)]
    pub const fn with_dct_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DCT_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DCT_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **SR_COLOR_MODE_ERR_INT_ENA** `[10:10]`
    ///
    /// The enable interrupt bit to sign that the selected source color mode is not supported.
    #[inline(always)]
    pub const fn with_sr_color_mode_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SR_COLOR_MODE_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SR_COLOR_MODE_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **OUT_EOF_INT_ENA** `[9:9]`
    ///
    /// The enable interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    #[inline(always)]
    pub const fn with_out_eof_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::OUT_EOF_INT_ENA_MASK
            } else {
                self.bits & !IntEna::OUT_EOF_INT_ENA_MASK
            },
        }
    }

    /// Set **RST_CHECK_POS_ERR_INT_ENA** `[8:8]`
    ///
    /// The enable interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    #[inline(always)]
    pub const fn with_rst_check_pos_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RST_CHECK_POS_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RST_CHECK_POS_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **RST_CHECK_NONE_ERR_INT_ENA** `[7:7]`
    ///
    /// The enable interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    #[inline(always)]
    pub const fn with_rst_check_none_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RST_CHECK_NONE_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RST_CHECK_NONE_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **RST_UXP_ERR_INT_ENA** `[6:6]`
    ///
    /// The enable interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    #[inline(always)]
    pub const fn with_rst_uxp_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RST_UXP_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RST_UXP_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **C_DQT_ID_ERR_INT_ENA** `[5:5]`
    ///
    /// The enable interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    #[inline(always)]
    pub const fn with_c_dqt_id_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::C_DQT_ID_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::C_DQT_ID_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **C_DHT_AC_ID_ERR_INT_ENA** `[4:4]`
    ///
    /// The enable interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    #[inline(always)]
    pub const fn with_c_dht_ac_id_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::C_DHT_AC_ID_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::C_DHT_AC_ID_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **C_DHT_DC_ID_ERR_INT_ENA** `[3:3]`
    ///
    /// The enable interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    #[inline(always)]
    pub const fn with_c_dht_dc_id_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::C_DHT_DC_ID_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::C_DHT_DC_ID_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **CID_ERR_INT_ENA** `[2:2]`
    ///
    /// The enable interrupt bit to sign that scan id check with component fails when decoding.
    #[inline(always)]
    pub const fn with_cid_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::CID_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::CID_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **RLE_PARALLEL_ERR_INT_ENA** `[1:1]`
    ///
    /// The enable interrupt bit to sign that rle parallel error when decoding.
    #[inline(always)]
    pub const fn with_rle_parallel_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::RLE_PARALLEL_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::RLE_PARALLEL_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **DONE_INT_ENA** `[0:0]`
    ///
    /// This enable interrupt bit turns to high level when JPEG finishes encoding a picture..
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

/// INT_ST Register
///
/// Interrupt status registers
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **DECODE_TIMEOUT_INT_ST** `[24]` (RO)
///
///   The status interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
///
/// - **UNDET_INT_ST** `[23]` (RO)
///
///   The status interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
///
/// - **MARKER_ERR_OTHER_SCAN_INT_ST** `[22]` (RO)
///
///   The status interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
///
/// - **MARKER_ERR_FST_SCAN_INT_ST** `[21]` (RO)
///
///   The status interrupt bit to sign that the first scan has header marker error when decoding.
///
/// - **SOS_UNMATCH_ERR_INT_ST** `[20]` (RO)
///
///   The status interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
///
/// - **DE_FRAME_EOF_LACK_INT_ST** `[19]` (RO)
///
///   The status interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
///
/// - **DE_FRAME_EOF_ERR_INT_ST** `[18]` (RO)
///
///   The status interrupt bit to sign that decoded blocks are smaller than expected when decoding.
///
/// - **EN_FRAME_EOF_LACK_INT_ST** `[17]` (RO)
///
///   The status interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
///
/// - **EN_FRAME_EOF_ERR_INT_ST** `[16]` (RO)
///
///   The status interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
///
/// - **UXP_DET_INT_ST** `[15]` (RO)
///
///   The status interrupt bit to sign that unsupported header marker is detected when decoding.
///
/// - **SCAN_CHECK_POS_ERR_INT_ST** `[14]` (RO)
///
///   The status interrupt bit to sign that SOS header marker position wrong when decoding.
///
/// - **SCAN_CHECK_NONE_ERR_INT_ST** `[13]` (RO)
///
///   The status interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
///
/// - **BS_LAST_BLOCK_EOF_INT_ST** `[12]` (RO)
///
///   The status interrupt bit to sign that the coding process for last block is finished.
///
/// - **DCT_DONE_INT_ST** `[11]` (RO)
///
///   The status interrupt bit to sign that one dct calculation is finished.
///
/// - **SR_COLOR_MODE_ERR_INT_ST** `[10]` (RO)
///
///   The status interrupt bit to sign that the selected source color mode is not supported.
///
/// - **OUT_EOF_INT_ST** `[9]` (RO)
///
///   The status interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
///
/// - **RST_CHECK_POS_ERR_INT_ST** `[8]` (RO)
///
///   The status interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
///
/// - **RST_CHECK_NONE_ERR_INT_ST** `[7]` (RO)
///
///   The status interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
///
/// - **RST_UXP_ERR_INT_ST** `[6]` (RO)
///
///   The status interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
///
/// - **C_DQT_ID_ERR_INT_ST** `[5]` (RO)
///
///   The status interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
///
/// - **C_DHT_AC_ID_ERR_INT_ST** `[4]` (RO)
///
///   The status interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
///
/// - **C_DHT_DC_ID_ERR_INT_ST** `[3]` (RO)
///
///   The status interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
///
/// - **CID_ERR_INT_ST** `[2]` (RO)
///
///   The status interrupt bit to sign that scan id check with component fails when decoding.
///
/// - **RLE_PARALLEL_ERR_INT_ST** `[1]` (RO)
///
///   The status interrupt bit to sign that rle parallel error when decoding.
///
/// - **DONE_INT_ST** `[0]` (RO)
///
///   This status interrupt bit turns to high level when JPEG finishes encoding a picture..
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

    /// **DECODE_TIMEOUT_INT_ST**
    ///
    /// The status interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const DECODE_TIMEOUT_INT_ST_OFFSET: u32 = 24;
    pub const DECODE_TIMEOUT_INT_ST_MASK: u32 = 0x01000000;
    pub const DECODE_TIMEOUT_INT_ST: u32 = 1 << 24;

    /// **UNDET_INT_ST**
    ///
    /// The status interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const UNDET_INT_ST_OFFSET: u32 = 23;
    pub const UNDET_INT_ST_MASK: u32 = 0x00800000;
    pub const UNDET_INT_ST: u32 = 1 << 23;

    /// **MARKER_ERR_OTHER_SCAN_INT_ST**
    ///
    /// The status interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const MARKER_ERR_OTHER_SCAN_INT_ST_OFFSET: u32 = 22;
    pub const MARKER_ERR_OTHER_SCAN_INT_ST_MASK: u32 = 0x00400000;
    pub const MARKER_ERR_OTHER_SCAN_INT_ST: u32 = 1 << 22;

    /// **MARKER_ERR_FST_SCAN_INT_ST**
    ///
    /// The status interrupt bit to sign that the first scan has header marker error when decoding.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const MARKER_ERR_FST_SCAN_INT_ST_OFFSET: u32 = 21;
    pub const MARKER_ERR_FST_SCAN_INT_ST_MASK: u32 = 0x00200000;
    pub const MARKER_ERR_FST_SCAN_INT_ST: u32 = 1 << 21;

    /// **SOS_UNMATCH_ERR_INT_ST**
    ///
    /// The status interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SOS_UNMATCH_ERR_INT_ST_OFFSET: u32 = 20;
    pub const SOS_UNMATCH_ERR_INT_ST_MASK: u32 = 0x00100000;
    pub const SOS_UNMATCH_ERR_INT_ST: u32 = 1 << 20;

    /// **DE_FRAME_EOF_LACK_INT_ST**
    ///
    /// The status interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const DE_FRAME_EOF_LACK_INT_ST_OFFSET: u32 = 19;
    pub const DE_FRAME_EOF_LACK_INT_ST_MASK: u32 = 0x00080000;
    pub const DE_FRAME_EOF_LACK_INT_ST: u32 = 1 << 19;

    /// **DE_FRAME_EOF_ERR_INT_ST**
    ///
    /// The status interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const DE_FRAME_EOF_ERR_INT_ST_OFFSET: u32 = 18;
    pub const DE_FRAME_EOF_ERR_INT_ST_MASK: u32 = 0x00040000;
    pub const DE_FRAME_EOF_ERR_INT_ST: u32 = 1 << 18;

    /// **EN_FRAME_EOF_LACK_INT_ST**
    ///
    /// The status interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const EN_FRAME_EOF_LACK_INT_ST_OFFSET: u32 = 17;
    pub const EN_FRAME_EOF_LACK_INT_ST_MASK: u32 = 0x00020000;
    pub const EN_FRAME_EOF_LACK_INT_ST: u32 = 1 << 17;

    /// **EN_FRAME_EOF_ERR_INT_ST**
    ///
    /// The status interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const EN_FRAME_EOF_ERR_INT_ST_OFFSET: u32 = 16;
    pub const EN_FRAME_EOF_ERR_INT_ST_MASK: u32 = 0x00010000;
    pub const EN_FRAME_EOF_ERR_INT_ST: u32 = 1 << 16;

    /// **UXP_DET_INT_ST**
    ///
    /// The status interrupt bit to sign that unsupported header marker is detected when decoding.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const UXP_DET_INT_ST_OFFSET: u32 = 15;
    pub const UXP_DET_INT_ST_MASK: u32 = 0x00008000;
    pub const UXP_DET_INT_ST: u32 = 1 << 15;

    /// **SCAN_CHECK_POS_ERR_INT_ST**
    ///
    /// The status interrupt bit to sign that SOS header marker position wrong when decoding.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SCAN_CHECK_POS_ERR_INT_ST_OFFSET: u32 = 14;
    pub const SCAN_CHECK_POS_ERR_INT_ST_MASK: u32 = 0x00004000;
    pub const SCAN_CHECK_POS_ERR_INT_ST: u32 = 1 << 14;

    /// **SCAN_CHECK_NONE_ERR_INT_ST**
    ///
    /// The status interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SCAN_CHECK_NONE_ERR_INT_ST_OFFSET: u32 = 13;
    pub const SCAN_CHECK_NONE_ERR_INT_ST_MASK: u32 = 0x00002000;
    pub const SCAN_CHECK_NONE_ERR_INT_ST: u32 = 1 << 13;

    /// **BS_LAST_BLOCK_EOF_INT_ST**
    ///
    /// The status interrupt bit to sign that the coding process for last block is finished.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const BS_LAST_BLOCK_EOF_INT_ST_OFFSET: u32 = 12;
    pub const BS_LAST_BLOCK_EOF_INT_ST_MASK: u32 = 0x00001000;
    pub const BS_LAST_BLOCK_EOF_INT_ST: u32 = 1 << 12;

    /// **DCT_DONE_INT_ST**
    ///
    /// The status interrupt bit to sign that one dct calculation is finished.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const DCT_DONE_INT_ST_OFFSET: u32 = 11;
    pub const DCT_DONE_INT_ST_MASK: u32 = 0x00000800;
    pub const DCT_DONE_INT_ST: u32 = 1 << 11;

    /// **SR_COLOR_MODE_ERR_INT_ST**
    ///
    /// The status interrupt bit to sign that the selected source color mode is not supported.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SR_COLOR_MODE_ERR_INT_ST_OFFSET: u32 = 10;
    pub const SR_COLOR_MODE_ERR_INT_ST_MASK: u32 = 0x00000400;
    pub const SR_COLOR_MODE_ERR_INT_ST: u32 = 1 << 10;

    /// **OUT_EOF_INT_ST**
    ///
    /// The status interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const OUT_EOF_INT_ST_OFFSET: u32 = 9;
    pub const OUT_EOF_INT_ST_MASK: u32 = 0x00000200;
    pub const OUT_EOF_INT_ST: u32 = 1 << 9;

    /// **RST_CHECK_POS_ERR_INT_ST**
    ///
    /// The status interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RST_CHECK_POS_ERR_INT_ST_OFFSET: u32 = 8;
    pub const RST_CHECK_POS_ERR_INT_ST_MASK: u32 = 0x00000100;
    pub const RST_CHECK_POS_ERR_INT_ST: u32 = 1 << 8;

    /// **RST_CHECK_NONE_ERR_INT_ST**
    ///
    /// The status interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RST_CHECK_NONE_ERR_INT_ST_OFFSET: u32 = 7;
    pub const RST_CHECK_NONE_ERR_INT_ST_MASK: u32 = 0x00000080;
    pub const RST_CHECK_NONE_ERR_INT_ST: u32 = 1 << 7;

    /// **RST_UXP_ERR_INT_ST**
    ///
    /// The status interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RST_UXP_ERR_INT_ST_OFFSET: u32 = 6;
    pub const RST_UXP_ERR_INT_ST_MASK: u32 = 0x00000040;
    pub const RST_UXP_ERR_INT_ST: u32 = 1 << 6;

    /// **C_DQT_ID_ERR_INT_ST**
    ///
    /// The status interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const C_DQT_ID_ERR_INT_ST_OFFSET: u32 = 5;
    pub const C_DQT_ID_ERR_INT_ST_MASK: u32 = 0x00000020;
    pub const C_DQT_ID_ERR_INT_ST: u32 = 1 << 5;

    /// **C_DHT_AC_ID_ERR_INT_ST**
    ///
    /// The status interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const C_DHT_AC_ID_ERR_INT_ST_OFFSET: u32 = 4;
    pub const C_DHT_AC_ID_ERR_INT_ST_MASK: u32 = 0x00000010;
    pub const C_DHT_AC_ID_ERR_INT_ST: u32 = 1 << 4;

    /// **C_DHT_DC_ID_ERR_INT_ST**
    ///
    /// The status interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const C_DHT_DC_ID_ERR_INT_ST_OFFSET: u32 = 3;
    pub const C_DHT_DC_ID_ERR_INT_ST_MASK: u32 = 0x00000008;
    pub const C_DHT_DC_ID_ERR_INT_ST: u32 = 1 << 3;

    /// **CID_ERR_INT_ST**
    ///
    /// The status interrupt bit to sign that scan id check with component fails when decoding.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CID_ERR_INT_ST_OFFSET: u32 = 2;
    pub const CID_ERR_INT_ST_MASK: u32 = 0x00000004;
    pub const CID_ERR_INT_ST: u32 = 1 << 2;

    /// **RLE_PARALLEL_ERR_INT_ST**
    ///
    /// The status interrupt bit to sign that rle parallel error when decoding.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RLE_PARALLEL_ERR_INT_ST_OFFSET: u32 = 1;
    pub const RLE_PARALLEL_ERR_INT_ST_MASK: u32 = 0x00000002;
    pub const RLE_PARALLEL_ERR_INT_ST: u32 = 1 << 1;

    /// **DONE_INT_ST**
    ///
    /// This status interrupt bit turns to high level when JPEG finishes encoding a picture..
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DONE_INT_ST_OFFSET: u32 = 0;
    pub const DONE_INT_ST_MASK: u32 = 0x00000001;
    pub const DONE_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DECODE_TIMEOUT_INT_ST** `[24]` (RO)
    ///
    /// The status interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    #[inline(always)]
    pub fn decode_timeout_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DECODE_TIMEOUT_INT_ST_MASK) != 0
    }

    /// **UNDET_INT_ST** `[23]` (RO)
    ///
    /// The status interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    #[inline(always)]
    pub fn undet_int_st(&self) -> bool {
        (self.value.read_bits() & Self::UNDET_INT_ST_MASK) != 0
    }

    /// **MARKER_ERR_OTHER_SCAN_INT_ST** `[22]` (RO)
    ///
    /// The status interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    #[inline(always)]
    pub fn marker_err_other_scan_int_st(&self) -> bool {
        (self.value.read_bits() & Self::MARKER_ERR_OTHER_SCAN_INT_ST_MASK) != 0
    }

    /// **MARKER_ERR_FST_SCAN_INT_ST** `[21]` (RO)
    ///
    /// The status interrupt bit to sign that the first scan has header marker error when decoding.
    #[inline(always)]
    pub fn marker_err_fst_scan_int_st(&self) -> bool {
        (self.value.read_bits() & Self::MARKER_ERR_FST_SCAN_INT_ST_MASK) != 0
    }

    /// **SOS_UNMATCH_ERR_INT_ST** `[20]` (RO)
    ///
    /// The status interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    #[inline(always)]
    pub fn sos_unmatch_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SOS_UNMATCH_ERR_INT_ST_MASK) != 0
    }

    /// **DE_FRAME_EOF_LACK_INT_ST** `[19]` (RO)
    ///
    /// The status interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    #[inline(always)]
    pub fn de_frame_eof_lack_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DE_FRAME_EOF_LACK_INT_ST_MASK) != 0
    }

    /// **DE_FRAME_EOF_ERR_INT_ST** `[18]` (RO)
    ///
    /// The status interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    #[inline(always)]
    pub fn de_frame_eof_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DE_FRAME_EOF_ERR_INT_ST_MASK) != 0
    }

    /// **EN_FRAME_EOF_LACK_INT_ST** `[17]` (RO)
    ///
    /// The status interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    #[inline(always)]
    pub fn en_frame_eof_lack_int_st(&self) -> bool {
        (self.value.read_bits() & Self::EN_FRAME_EOF_LACK_INT_ST_MASK) != 0
    }

    /// **EN_FRAME_EOF_ERR_INT_ST** `[16]` (RO)
    ///
    /// The status interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    #[inline(always)]
    pub fn en_frame_eof_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::EN_FRAME_EOF_ERR_INT_ST_MASK) != 0
    }

    /// **UXP_DET_INT_ST** `[15]` (RO)
    ///
    /// The status interrupt bit to sign that unsupported header marker is detected when decoding.
    #[inline(always)]
    pub fn uxp_det_int_st(&self) -> bool {
        (self.value.read_bits() & Self::UXP_DET_INT_ST_MASK) != 0
    }

    /// **SCAN_CHECK_POS_ERR_INT_ST** `[14]` (RO)
    ///
    /// The status interrupt bit to sign that SOS header marker position wrong when decoding.
    #[inline(always)]
    pub fn scan_check_pos_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SCAN_CHECK_POS_ERR_INT_ST_MASK) != 0
    }

    /// **SCAN_CHECK_NONE_ERR_INT_ST** `[13]` (RO)
    ///
    /// The status interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    #[inline(always)]
    pub fn scan_check_none_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SCAN_CHECK_NONE_ERR_INT_ST_MASK) != 0
    }

    /// **BS_LAST_BLOCK_EOF_INT_ST** `[12]` (RO)
    ///
    /// The status interrupt bit to sign that the coding process for last block is finished.
    #[inline(always)]
    pub fn bs_last_block_eof_int_st(&self) -> bool {
        (self.value.read_bits() & Self::BS_LAST_BLOCK_EOF_INT_ST_MASK) != 0
    }

    /// **DCT_DONE_INT_ST** `[11]` (RO)
    ///
    /// The status interrupt bit to sign that one dct calculation is finished.
    #[inline(always)]
    pub fn dct_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DCT_DONE_INT_ST_MASK) != 0
    }

    /// **SR_COLOR_MODE_ERR_INT_ST** `[10]` (RO)
    ///
    /// The status interrupt bit to sign that the selected source color mode is not supported.
    #[inline(always)]
    pub fn sr_color_mode_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SR_COLOR_MODE_ERR_INT_ST_MASK) != 0
    }

    /// **OUT_EOF_INT_ST** `[9]` (RO)
    ///
    /// The status interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    #[inline(always)]
    pub fn out_eof_int_st(&self) -> bool {
        (self.value.read_bits() & Self::OUT_EOF_INT_ST_MASK) != 0
    }

    /// **RST_CHECK_POS_ERR_INT_ST** `[8]` (RO)
    ///
    /// The status interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    #[inline(always)]
    pub fn rst_check_pos_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RST_CHECK_POS_ERR_INT_ST_MASK) != 0
    }

    /// **RST_CHECK_NONE_ERR_INT_ST** `[7]` (RO)
    ///
    /// The status interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    #[inline(always)]
    pub fn rst_check_none_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RST_CHECK_NONE_ERR_INT_ST_MASK) != 0
    }

    /// **RST_UXP_ERR_INT_ST** `[6]` (RO)
    ///
    /// The status interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    #[inline(always)]
    pub fn rst_uxp_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RST_UXP_ERR_INT_ST_MASK) != 0
    }

    /// **C_DQT_ID_ERR_INT_ST** `[5]` (RO)
    ///
    /// The status interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    #[inline(always)]
    pub fn c_dqt_id_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::C_DQT_ID_ERR_INT_ST_MASK) != 0
    }

    /// **C_DHT_AC_ID_ERR_INT_ST** `[4]` (RO)
    ///
    /// The status interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    #[inline(always)]
    pub fn c_dht_ac_id_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::C_DHT_AC_ID_ERR_INT_ST_MASK) != 0
    }

    /// **C_DHT_DC_ID_ERR_INT_ST** `[3]` (RO)
    ///
    /// The status interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    #[inline(always)]
    pub fn c_dht_dc_id_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::C_DHT_DC_ID_ERR_INT_ST_MASK) != 0
    }

    /// **CID_ERR_INT_ST** `[2]` (RO)
    ///
    /// The status interrupt bit to sign that scan id check with component fails when decoding.
    #[inline(always)]
    pub fn cid_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CID_ERR_INT_ST_MASK) != 0
    }

    /// **RLE_PARALLEL_ERR_INT_ST** `[1]` (RO)
    ///
    /// The status interrupt bit to sign that rle parallel error when decoding.
    #[inline(always)]
    pub fn rle_parallel_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RLE_PARALLEL_ERR_INT_ST_MASK) != 0
    }

    /// **DONE_INT_ST** `[0]` (RO)
    ///
    /// This status interrupt bit turns to high level when JPEG finishes encoding a picture..
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

    /// Read **DECODE_TIMEOUT_INT_ST** `[24:24]`
    ///
    /// The status interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    #[inline(always)]
    pub const fn decode_timeout_int_st(&self) -> bool {
        (self.bits & IntSt::DECODE_TIMEOUT_INT_ST_MASK) != 0
    }

    /// Read **UNDET_INT_ST** `[23:23]`
    ///
    /// The status interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    #[inline(always)]
    pub const fn undet_int_st(&self) -> bool {
        (self.bits & IntSt::UNDET_INT_ST_MASK) != 0
    }

    /// Read **MARKER_ERR_OTHER_SCAN_INT_ST** `[22:22]`
    ///
    /// The status interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    #[inline(always)]
    pub const fn marker_err_other_scan_int_st(&self) -> bool {
        (self.bits & IntSt::MARKER_ERR_OTHER_SCAN_INT_ST_MASK) != 0
    }

    /// Read **MARKER_ERR_FST_SCAN_INT_ST** `[21:21]`
    ///
    /// The status interrupt bit to sign that the first scan has header marker error when decoding.
    #[inline(always)]
    pub const fn marker_err_fst_scan_int_st(&self) -> bool {
        (self.bits & IntSt::MARKER_ERR_FST_SCAN_INT_ST_MASK) != 0
    }

    /// Read **SOS_UNMATCH_ERR_INT_ST** `[20:20]`
    ///
    /// The status interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    #[inline(always)]
    pub const fn sos_unmatch_err_int_st(&self) -> bool {
        (self.bits & IntSt::SOS_UNMATCH_ERR_INT_ST_MASK) != 0
    }

    /// Read **DE_FRAME_EOF_LACK_INT_ST** `[19:19]`
    ///
    /// The status interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    #[inline(always)]
    pub const fn de_frame_eof_lack_int_st(&self) -> bool {
        (self.bits & IntSt::DE_FRAME_EOF_LACK_INT_ST_MASK) != 0
    }

    /// Read **DE_FRAME_EOF_ERR_INT_ST** `[18:18]`
    ///
    /// The status interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    #[inline(always)]
    pub const fn de_frame_eof_err_int_st(&self) -> bool {
        (self.bits & IntSt::DE_FRAME_EOF_ERR_INT_ST_MASK) != 0
    }

    /// Read **EN_FRAME_EOF_LACK_INT_ST** `[17:17]`
    ///
    /// The status interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    #[inline(always)]
    pub const fn en_frame_eof_lack_int_st(&self) -> bool {
        (self.bits & IntSt::EN_FRAME_EOF_LACK_INT_ST_MASK) != 0
    }

    /// Read **EN_FRAME_EOF_ERR_INT_ST** `[16:16]`
    ///
    /// The status interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    #[inline(always)]
    pub const fn en_frame_eof_err_int_st(&self) -> bool {
        (self.bits & IntSt::EN_FRAME_EOF_ERR_INT_ST_MASK) != 0
    }

    /// Read **UXP_DET_INT_ST** `[15:15]`
    ///
    /// The status interrupt bit to sign that unsupported header marker is detected when decoding.
    #[inline(always)]
    pub const fn uxp_det_int_st(&self) -> bool {
        (self.bits & IntSt::UXP_DET_INT_ST_MASK) != 0
    }

    /// Read **SCAN_CHECK_POS_ERR_INT_ST** `[14:14]`
    ///
    /// The status interrupt bit to sign that SOS header marker position wrong when decoding.
    #[inline(always)]
    pub const fn scan_check_pos_err_int_st(&self) -> bool {
        (self.bits & IntSt::SCAN_CHECK_POS_ERR_INT_ST_MASK) != 0
    }

    /// Read **SCAN_CHECK_NONE_ERR_INT_ST** `[13:13]`
    ///
    /// The status interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    #[inline(always)]
    pub const fn scan_check_none_err_int_st(&self) -> bool {
        (self.bits & IntSt::SCAN_CHECK_NONE_ERR_INT_ST_MASK) != 0
    }

    /// Read **BS_LAST_BLOCK_EOF_INT_ST** `[12:12]`
    ///
    /// The status interrupt bit to sign that the coding process for last block is finished.
    #[inline(always)]
    pub const fn bs_last_block_eof_int_st(&self) -> bool {
        (self.bits & IntSt::BS_LAST_BLOCK_EOF_INT_ST_MASK) != 0
    }

    /// Read **DCT_DONE_INT_ST** `[11:11]`
    ///
    /// The status interrupt bit to sign that one dct calculation is finished.
    #[inline(always)]
    pub const fn dct_done_int_st(&self) -> bool {
        (self.bits & IntSt::DCT_DONE_INT_ST_MASK) != 0
    }

    /// Read **SR_COLOR_MODE_ERR_INT_ST** `[10:10]`
    ///
    /// The status interrupt bit to sign that the selected source color mode is not supported.
    #[inline(always)]
    pub const fn sr_color_mode_err_int_st(&self) -> bool {
        (self.bits & IntSt::SR_COLOR_MODE_ERR_INT_ST_MASK) != 0
    }

    /// Read **OUT_EOF_INT_ST** `[9:9]`
    ///
    /// The status interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    #[inline(always)]
    pub const fn out_eof_int_st(&self) -> bool {
        (self.bits & IntSt::OUT_EOF_INT_ST_MASK) != 0
    }

    /// Read **RST_CHECK_POS_ERR_INT_ST** `[8:8]`
    ///
    /// The status interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    #[inline(always)]
    pub const fn rst_check_pos_err_int_st(&self) -> bool {
        (self.bits & IntSt::RST_CHECK_POS_ERR_INT_ST_MASK) != 0
    }

    /// Read **RST_CHECK_NONE_ERR_INT_ST** `[7:7]`
    ///
    /// The status interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    #[inline(always)]
    pub const fn rst_check_none_err_int_st(&self) -> bool {
        (self.bits & IntSt::RST_CHECK_NONE_ERR_INT_ST_MASK) != 0
    }

    /// Read **RST_UXP_ERR_INT_ST** `[6:6]`
    ///
    /// The status interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    #[inline(always)]
    pub const fn rst_uxp_err_int_st(&self) -> bool {
        (self.bits & IntSt::RST_UXP_ERR_INT_ST_MASK) != 0
    }

    /// Read **C_DQT_ID_ERR_INT_ST** `[5:5]`
    ///
    /// The status interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    #[inline(always)]
    pub const fn c_dqt_id_err_int_st(&self) -> bool {
        (self.bits & IntSt::C_DQT_ID_ERR_INT_ST_MASK) != 0
    }

    /// Read **C_DHT_AC_ID_ERR_INT_ST** `[4:4]`
    ///
    /// The status interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    #[inline(always)]
    pub const fn c_dht_ac_id_err_int_st(&self) -> bool {
        (self.bits & IntSt::C_DHT_AC_ID_ERR_INT_ST_MASK) != 0
    }

    /// Read **C_DHT_DC_ID_ERR_INT_ST** `[3:3]`
    ///
    /// The status interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    #[inline(always)]
    pub const fn c_dht_dc_id_err_int_st(&self) -> bool {
        (self.bits & IntSt::C_DHT_DC_ID_ERR_INT_ST_MASK) != 0
    }

    /// Read **CID_ERR_INT_ST** `[2:2]`
    ///
    /// The status interrupt bit to sign that scan id check with component fails when decoding.
    #[inline(always)]
    pub const fn cid_err_int_st(&self) -> bool {
        (self.bits & IntSt::CID_ERR_INT_ST_MASK) != 0
    }

    /// Read **RLE_PARALLEL_ERR_INT_ST** `[1:1]`
    ///
    /// The status interrupt bit to sign that rle parallel error when decoding.
    #[inline(always)]
    pub const fn rle_parallel_err_int_st(&self) -> bool {
        (self.bits & IntSt::RLE_PARALLEL_ERR_INT_ST_MASK) != 0
    }

    /// Read **DONE_INT_ST** `[0:0]`
    ///
    /// This status interrupt bit turns to high level when JPEG finishes encoding a picture..
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

/// INT_CLR Register
///
/// Interrupt clear registers
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **DECODE_TIMEOUT_INT_CLR** `[24]` (WO)
///
///   The clear interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
///
/// - **UNDET_INT_CLR** `[23]` (WO)
///
///   The clear interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
///
/// - **MARKER_ERR_OTHER_SCAN_INT_CLR** `[22]` (WO)
///
///   The clear interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
///
/// - **MARKER_ERR_FST_SCAN_INT_CLR** `[21]` (WO)
///
///   The clear interrupt bit to sign that the first scan has header marker error when decoding.
///
/// - **SOS_UNMATCH_ERR_INT_CLR** `[20]` (WO)
///
///   The clear interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
///
/// - **DE_FRAME_EOF_LACK_INT_CLR** `[19]` (WO)
///
///   The clear interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
///
/// - **DE_FRAME_EOF_ERR_INT_CLR** `[18]` (WO)
///
///   The clear interrupt bit to sign that decoded blocks are smaller than expected when decoding.
///
/// - **EN_FRAME_EOF_LACK_INT_CLR** `[17]` (WO)
///
///   The clear interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
///
/// - **EN_FRAME_EOF_ERR_INT_CLR** `[16]` (WO)
///
///   The clear interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
///
/// - **UXP_DET_INT_CLR** `[15]` (WO)
///
///   The clear interrupt bit to sign that unsupported header marker is detected when decoding.
///
/// - **SCAN_CHECK_POS_ERR_INT_CLR** `[14]` (WO)
///
///   The clear interrupt bit to sign that SOS header marker position wrong when decoding.
///
/// - **SCAN_CHECK_NONE_ERR_INT_CLR** `[13]` (WO)
///
///   The clear interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
///
/// - **BS_LAST_BLOCK_EOF_INT_CLR** `[12]` (WO)
///
///   The clear interrupt bit to sign that the coding process for last block is finished.
///
/// - **DCT_DONE_INT_CLR** `[11]` (WO)
///
///   The clear interrupt bit to sign that one dct calculation is finished.
///
/// - **SR_COLOR_MODE_ERR_INT_CLR** `[10]` (WO)
///
///   The clear interrupt bit to sign that the selected source color mode is not supported.
///
/// - **OUT_EOF_INT_CLR** `[9]` (WO)
///
///   The clear interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
///
/// - **RST_CHECK_POS_ERR_INT_CLR** `[8]` (WO)
///
///   The clear interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
///
/// - **RST_CHECK_NONE_ERR_INT_CLR** `[7]` (WO)
///
///   The clear interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
///
/// - **RST_UXP_ERR_INT_CLR** `[6]` (WO)
///
///   The clear interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
///
/// - **C_DQT_ID_ERR_INT_CLR** `[5]` (WO)
///
///   The clear interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
///
/// - **C_DHT_AC_ID_ERR_INT_CLR** `[4]` (WO)
///
///   The clear interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
///
/// - **C_DHT_DC_ID_ERR_INT_CLR** `[3]` (WO)
///
///   The clear interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
///
/// - **CID_ERR_INT_CLR** `[2]` (WO)
///
///   The clear interrupt bit to sign that scan id check with component fails when decoding.
///
/// - **RLE_PARALLEL_ERR_INT_CLR** `[1]` (WO)
///
///   The clear interrupt bit to sign that rle parallel error when decoding.
///
/// - **DONE_INT_CLR** `[0]` (WO)
///
///   This clear interrupt bit turns to high level when JPEG finishes encoding a picture..
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

    /// **DECODE_TIMEOUT_INT_CLR**
    ///
    /// The clear interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const DECODE_TIMEOUT_INT_CLR_OFFSET: u32 = 24;
    pub const DECODE_TIMEOUT_INT_CLR_MASK: u32 = 0x01000000;
    pub const DECODE_TIMEOUT_INT_CLR: u32 = 1 << 24;

    /// **UNDET_INT_CLR**
    ///
    /// The clear interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const UNDET_INT_CLR_OFFSET: u32 = 23;
    pub const UNDET_INT_CLR_MASK: u32 = 0x00800000;
    pub const UNDET_INT_CLR: u32 = 1 << 23;

    /// **MARKER_ERR_OTHER_SCAN_INT_CLR**
    ///
    /// The clear interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const MARKER_ERR_OTHER_SCAN_INT_CLR_OFFSET: u32 = 22;
    pub const MARKER_ERR_OTHER_SCAN_INT_CLR_MASK: u32 = 0x00400000;
    pub const MARKER_ERR_OTHER_SCAN_INT_CLR: u32 = 1 << 22;

    /// **MARKER_ERR_FST_SCAN_INT_CLR**
    ///
    /// The clear interrupt bit to sign that the first scan has header marker error when decoding.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const MARKER_ERR_FST_SCAN_INT_CLR_OFFSET: u32 = 21;
    pub const MARKER_ERR_FST_SCAN_INT_CLR_MASK: u32 = 0x00200000;
    pub const MARKER_ERR_FST_SCAN_INT_CLR: u32 = 1 << 21;

    /// **SOS_UNMATCH_ERR_INT_CLR**
    ///
    /// The clear interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SOS_UNMATCH_ERR_INT_CLR_OFFSET: u32 = 20;
    pub const SOS_UNMATCH_ERR_INT_CLR_MASK: u32 = 0x00100000;
    pub const SOS_UNMATCH_ERR_INT_CLR: u32 = 1 << 20;

    /// **DE_FRAME_EOF_LACK_INT_CLR**
    ///
    /// The clear interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const DE_FRAME_EOF_LACK_INT_CLR_OFFSET: u32 = 19;
    pub const DE_FRAME_EOF_LACK_INT_CLR_MASK: u32 = 0x00080000;
    pub const DE_FRAME_EOF_LACK_INT_CLR: u32 = 1 << 19;

    /// **DE_FRAME_EOF_ERR_INT_CLR**
    ///
    /// The clear interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const DE_FRAME_EOF_ERR_INT_CLR_OFFSET: u32 = 18;
    pub const DE_FRAME_EOF_ERR_INT_CLR_MASK: u32 = 0x00040000;
    pub const DE_FRAME_EOF_ERR_INT_CLR: u32 = 1 << 18;

    /// **EN_FRAME_EOF_LACK_INT_CLR**
    ///
    /// The clear interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const EN_FRAME_EOF_LACK_INT_CLR_OFFSET: u32 = 17;
    pub const EN_FRAME_EOF_LACK_INT_CLR_MASK: u32 = 0x00020000;
    pub const EN_FRAME_EOF_LACK_INT_CLR: u32 = 1 << 17;

    /// **EN_FRAME_EOF_ERR_INT_CLR**
    ///
    /// The clear interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const EN_FRAME_EOF_ERR_INT_CLR_OFFSET: u32 = 16;
    pub const EN_FRAME_EOF_ERR_INT_CLR_MASK: u32 = 0x00010000;
    pub const EN_FRAME_EOF_ERR_INT_CLR: u32 = 1 << 16;

    /// **UXP_DET_INT_CLR**
    ///
    /// The clear interrupt bit to sign that unsupported header marker is detected when decoding.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const UXP_DET_INT_CLR_OFFSET: u32 = 15;
    pub const UXP_DET_INT_CLR_MASK: u32 = 0x00008000;
    pub const UXP_DET_INT_CLR: u32 = 1 << 15;

    /// **SCAN_CHECK_POS_ERR_INT_CLR**
    ///
    /// The clear interrupt bit to sign that SOS header marker position wrong when decoding.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const SCAN_CHECK_POS_ERR_INT_CLR_OFFSET: u32 = 14;
    pub const SCAN_CHECK_POS_ERR_INT_CLR_MASK: u32 = 0x00004000;
    pub const SCAN_CHECK_POS_ERR_INT_CLR: u32 = 1 << 14;

    /// **SCAN_CHECK_NONE_ERR_INT_CLR**
    ///
    /// The clear interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SCAN_CHECK_NONE_ERR_INT_CLR_OFFSET: u32 = 13;
    pub const SCAN_CHECK_NONE_ERR_INT_CLR_MASK: u32 = 0x00002000;
    pub const SCAN_CHECK_NONE_ERR_INT_CLR: u32 = 1 << 13;

    /// **BS_LAST_BLOCK_EOF_INT_CLR**
    ///
    /// The clear interrupt bit to sign that the coding process for last block is finished.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const BS_LAST_BLOCK_EOF_INT_CLR_OFFSET: u32 = 12;
    pub const BS_LAST_BLOCK_EOF_INT_CLR_MASK: u32 = 0x00001000;
    pub const BS_LAST_BLOCK_EOF_INT_CLR: u32 = 1 << 12;

    /// **DCT_DONE_INT_CLR**
    ///
    /// The clear interrupt bit to sign that one dct calculation is finished.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const DCT_DONE_INT_CLR_OFFSET: u32 = 11;
    pub const DCT_DONE_INT_CLR_MASK: u32 = 0x00000800;
    pub const DCT_DONE_INT_CLR: u32 = 1 << 11;

    /// **SR_COLOR_MODE_ERR_INT_CLR**
    ///
    /// The clear interrupt bit to sign that the selected source color mode is not supported.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SR_COLOR_MODE_ERR_INT_CLR_OFFSET: u32 = 10;
    pub const SR_COLOR_MODE_ERR_INT_CLR_MASK: u32 = 0x00000400;
    pub const SR_COLOR_MODE_ERR_INT_CLR: u32 = 1 << 10;

    /// **OUT_EOF_INT_CLR**
    ///
    /// The clear interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const OUT_EOF_INT_CLR_OFFSET: u32 = 9;
    pub const OUT_EOF_INT_CLR_MASK: u32 = 0x00000200;
    pub const OUT_EOF_INT_CLR: u32 = 1 << 9;

    /// **RST_CHECK_POS_ERR_INT_CLR**
    ///
    /// The clear interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RST_CHECK_POS_ERR_INT_CLR_OFFSET: u32 = 8;
    pub const RST_CHECK_POS_ERR_INT_CLR_MASK: u32 = 0x00000100;
    pub const RST_CHECK_POS_ERR_INT_CLR: u32 = 1 << 8;

    /// **RST_CHECK_NONE_ERR_INT_CLR**
    ///
    /// The clear interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RST_CHECK_NONE_ERR_INT_CLR_OFFSET: u32 = 7;
    pub const RST_CHECK_NONE_ERR_INT_CLR_MASK: u32 = 0x00000080;
    pub const RST_CHECK_NONE_ERR_INT_CLR: u32 = 1 << 7;

    /// **RST_UXP_ERR_INT_CLR**
    ///
    /// The clear interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RST_UXP_ERR_INT_CLR_OFFSET: u32 = 6;
    pub const RST_UXP_ERR_INT_CLR_MASK: u32 = 0x00000040;
    pub const RST_UXP_ERR_INT_CLR: u32 = 1 << 6;

    /// **C_DQT_ID_ERR_INT_CLR**
    ///
    /// The clear interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const C_DQT_ID_ERR_INT_CLR_OFFSET: u32 = 5;
    pub const C_DQT_ID_ERR_INT_CLR_MASK: u32 = 0x00000020;
    pub const C_DQT_ID_ERR_INT_CLR: u32 = 1 << 5;

    /// **C_DHT_AC_ID_ERR_INT_CLR**
    ///
    /// The clear interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const C_DHT_AC_ID_ERR_INT_CLR_OFFSET: u32 = 4;
    pub const C_DHT_AC_ID_ERR_INT_CLR_MASK: u32 = 0x00000010;
    pub const C_DHT_AC_ID_ERR_INT_CLR: u32 = 1 << 4;

    /// **C_DHT_DC_ID_ERR_INT_CLR**
    ///
    /// The clear interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const C_DHT_DC_ID_ERR_INT_CLR_OFFSET: u32 = 3;
    pub const C_DHT_DC_ID_ERR_INT_CLR_MASK: u32 = 0x00000008;
    pub const C_DHT_DC_ID_ERR_INT_CLR: u32 = 1 << 3;

    /// **CID_ERR_INT_CLR**
    ///
    /// The clear interrupt bit to sign that scan id check with component fails when decoding.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CID_ERR_INT_CLR_OFFSET: u32 = 2;
    pub const CID_ERR_INT_CLR_MASK: u32 = 0x00000004;
    pub const CID_ERR_INT_CLR: u32 = 1 << 2;

    /// **RLE_PARALLEL_ERR_INT_CLR**
    ///
    /// The clear interrupt bit to sign that rle parallel error when decoding.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RLE_PARALLEL_ERR_INT_CLR_OFFSET: u32 = 1;
    pub const RLE_PARALLEL_ERR_INT_CLR_MASK: u32 = 0x00000002;
    pub const RLE_PARALLEL_ERR_INT_CLR: u32 = 1 << 1;

    /// **DONE_INT_CLR**
    ///
    /// This clear interrupt bit turns to high level when JPEG finishes encoding a picture..
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DONE_INT_CLR_OFFSET: u32 = 0;
    pub const DONE_INT_CLR_MASK: u32 = 0x00000001;
    pub const DONE_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **DECODE_TIMEOUT_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    #[inline(always)]
    pub fn trigger_decode_timeout_int_clr(&self) {
        self.value.write_bits(Self::DECODE_TIMEOUT_INT_CLR_MASK);
    }

    /// Trigger **UNDET_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    #[inline(always)]
    pub fn trigger_undet_int_clr(&self) {
        self.value.write_bits(Self::UNDET_INT_CLR_MASK);
    }

    /// Trigger **MARKER_ERR_OTHER_SCAN_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    #[inline(always)]
    pub fn trigger_marker_err_other_scan_int_clr(&self) {
        self.value
            .write_bits(Self::MARKER_ERR_OTHER_SCAN_INT_CLR_MASK);
    }

    /// Trigger **MARKER_ERR_FST_SCAN_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that the first scan has header marker error when decoding.
    #[inline(always)]
    pub fn trigger_marker_err_fst_scan_int_clr(&self) {
        self.value
            .write_bits(Self::MARKER_ERR_FST_SCAN_INT_CLR_MASK);
    }

    /// Trigger **SOS_UNMATCH_ERR_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    #[inline(always)]
    pub fn trigger_sos_unmatch_err_int_clr(&self) {
        self.value.write_bits(Self::SOS_UNMATCH_ERR_INT_CLR_MASK);
    }

    /// Trigger **DE_FRAME_EOF_LACK_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    #[inline(always)]
    pub fn trigger_de_frame_eof_lack_int_clr(&self) {
        self.value.write_bits(Self::DE_FRAME_EOF_LACK_INT_CLR_MASK);
    }

    /// Trigger **DE_FRAME_EOF_ERR_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    #[inline(always)]
    pub fn trigger_de_frame_eof_err_int_clr(&self) {
        self.value.write_bits(Self::DE_FRAME_EOF_ERR_INT_CLR_MASK);
    }

    /// Trigger **EN_FRAME_EOF_LACK_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    #[inline(always)]
    pub fn trigger_en_frame_eof_lack_int_clr(&self) {
        self.value.write_bits(Self::EN_FRAME_EOF_LACK_INT_CLR_MASK);
    }

    /// Trigger **EN_FRAME_EOF_ERR_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    #[inline(always)]
    pub fn trigger_en_frame_eof_err_int_clr(&self) {
        self.value.write_bits(Self::EN_FRAME_EOF_ERR_INT_CLR_MASK);
    }

    /// Trigger **UXP_DET_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that unsupported header marker is detected when decoding.
    #[inline(always)]
    pub fn trigger_uxp_det_int_clr(&self) {
        self.value.write_bits(Self::UXP_DET_INT_CLR_MASK);
    }

    /// Trigger **SCAN_CHECK_POS_ERR_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that SOS header marker position wrong when decoding.
    #[inline(always)]
    pub fn trigger_scan_check_pos_err_int_clr(&self) {
        self.value.write_bits(Self::SCAN_CHECK_POS_ERR_INT_CLR_MASK);
    }

    /// Trigger **SCAN_CHECK_NONE_ERR_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    #[inline(always)]
    pub fn trigger_scan_check_none_err_int_clr(&self) {
        self.value
            .write_bits(Self::SCAN_CHECK_NONE_ERR_INT_CLR_MASK);
    }

    /// Trigger **BS_LAST_BLOCK_EOF_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that the coding process for last block is finished.
    #[inline(always)]
    pub fn trigger_bs_last_block_eof_int_clr(&self) {
        self.value.write_bits(Self::BS_LAST_BLOCK_EOF_INT_CLR_MASK);
    }

    /// Trigger **DCT_DONE_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that one dct calculation is finished.
    #[inline(always)]
    pub fn trigger_dct_done_int_clr(&self) {
        self.value.write_bits(Self::DCT_DONE_INT_CLR_MASK);
    }

    /// Trigger **SR_COLOR_MODE_ERR_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that the selected source color mode is not supported.
    #[inline(always)]
    pub fn trigger_sr_color_mode_err_int_clr(&self) {
        self.value.write_bits(Self::SR_COLOR_MODE_ERR_INT_CLR_MASK);
    }

    /// Trigger **OUT_EOF_INT_CLR** (WO)
    ///
    /// The clear interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    #[inline(always)]
    pub fn trigger_out_eof_int_clr(&self) {
        self.value.write_bits(Self::OUT_EOF_INT_CLR_MASK);
    }

    /// Trigger **RST_CHECK_POS_ERR_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    #[inline(always)]
    pub fn trigger_rst_check_pos_err_int_clr(&self) {
        self.value.write_bits(Self::RST_CHECK_POS_ERR_INT_CLR_MASK);
    }

    /// Trigger **RST_CHECK_NONE_ERR_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    #[inline(always)]
    pub fn trigger_rst_check_none_err_int_clr(&self) {
        self.value.write_bits(Self::RST_CHECK_NONE_ERR_INT_CLR_MASK);
    }

    /// Trigger **RST_UXP_ERR_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    #[inline(always)]
    pub fn trigger_rst_uxp_err_int_clr(&self) {
        self.value.write_bits(Self::RST_UXP_ERR_INT_CLR_MASK);
    }

    /// Trigger **C_DQT_ID_ERR_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    #[inline(always)]
    pub fn trigger_c_dqt_id_err_int_clr(&self) {
        self.value.write_bits(Self::C_DQT_ID_ERR_INT_CLR_MASK);
    }

    /// Trigger **C_DHT_AC_ID_ERR_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    #[inline(always)]
    pub fn trigger_c_dht_ac_id_err_int_clr(&self) {
        self.value.write_bits(Self::C_DHT_AC_ID_ERR_INT_CLR_MASK);
    }

    /// Trigger **C_DHT_DC_ID_ERR_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    #[inline(always)]
    pub fn trigger_c_dht_dc_id_err_int_clr(&self) {
        self.value.write_bits(Self::C_DHT_DC_ID_ERR_INT_CLR_MASK);
    }

    /// Trigger **CID_ERR_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that scan id check with component fails when decoding.
    #[inline(always)]
    pub fn trigger_cid_err_int_clr(&self) {
        self.value.write_bits(Self::CID_ERR_INT_CLR_MASK);
    }

    /// Trigger **RLE_PARALLEL_ERR_INT_CLR** (WO)
    ///
    /// The clear interrupt bit to sign that rle parallel error when decoding.
    #[inline(always)]
    pub fn trigger_rle_parallel_err_int_clr(&self) {
        self.value.write_bits(Self::RLE_PARALLEL_ERR_INT_CLR_MASK);
    }

    /// Trigger **DONE_INT_CLR** (WO)
    ///
    /// This clear interrupt bit turns to high level when JPEG finishes encoding a picture..
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

    /// Set **DECODE_TIMEOUT_INT_CLR** `[24:24]`
    ///
    /// The clear interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding.
    #[inline(always)]
    pub const fn with_decode_timeout_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DECODE_TIMEOUT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DECODE_TIMEOUT_INT_CLR_MASK
            },
        }
    }

    /// Set **UNDET_INT_CLR** `[23:23]`
    ///
    /// The clear interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding.
    #[inline(always)]
    pub const fn with_undet_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::UNDET_INT_CLR_MASK
            } else {
                self.bits & !IntClr::UNDET_INT_CLR_MASK
            },
        }
    }

    /// Set **MARKER_ERR_OTHER_SCAN_INT_CLR** `[22:22]`
    ///
    /// The clear interrupt bit to sign that the following scans but not the first scan have header marker error when decoding.
    #[inline(always)]
    pub const fn with_marker_err_other_scan_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::MARKER_ERR_OTHER_SCAN_INT_CLR_MASK
            } else {
                self.bits & !IntClr::MARKER_ERR_OTHER_SCAN_INT_CLR_MASK
            },
        }
    }

    /// Set **MARKER_ERR_FST_SCAN_INT_CLR** `[21:21]`
    ///
    /// The clear interrupt bit to sign that the first scan has header marker error when decoding.
    #[inline(always)]
    pub const fn with_marker_err_fst_scan_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::MARKER_ERR_FST_SCAN_INT_CLR_MASK
            } else {
                self.bits & !IntClr::MARKER_ERR_FST_SCAN_INT_CLR_MASK
            },
        }
    }

    /// Set **SOS_UNMATCH_ERR_INT_CLR** `[20:20]`
    ///
    /// The clear interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding.
    #[inline(always)]
    pub const fn with_sos_unmatch_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SOS_UNMATCH_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SOS_UNMATCH_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **DE_FRAME_EOF_LACK_INT_CLR** `[19:19]`
    ///
    /// The clear interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough.
    #[inline(always)]
    pub const fn with_de_frame_eof_lack_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DE_FRAME_EOF_LACK_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DE_FRAME_EOF_LACK_INT_CLR_MASK
            },
        }
    }

    /// Set **DE_FRAME_EOF_ERR_INT_CLR** `[18:18]`
    ///
    /// The clear interrupt bit to sign that decoded blocks are smaller than expected when decoding.
    #[inline(always)]
    pub const fn with_de_frame_eof_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DE_FRAME_EOF_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DE_FRAME_EOF_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **EN_FRAME_EOF_LACK_INT_CLR** `[17:17]`
    ///
    /// The clear interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough.
    #[inline(always)]
    pub const fn with_en_frame_eof_lack_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::EN_FRAME_EOF_LACK_INT_CLR_MASK
            } else {
                self.bits & !IntClr::EN_FRAME_EOF_LACK_INT_CLR_MASK
            },
        }
    }

    /// Set **EN_FRAME_EOF_ERR_INT_CLR** `[16:16]`
    ///
    /// The clear interrupt bit to sign that received pixel blocks are smaller than expected when encoding.
    #[inline(always)]
    pub const fn with_en_frame_eof_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::EN_FRAME_EOF_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::EN_FRAME_EOF_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **UXP_DET_INT_CLR** `[15:15]`
    ///
    /// The clear interrupt bit to sign that unsupported header marker is detected when decoding.
    #[inline(always)]
    pub const fn with_uxp_det_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::UXP_DET_INT_CLR_MASK
            } else {
                self.bits & !IntClr::UXP_DET_INT_CLR_MASK
            },
        }
    }

    /// Set **SCAN_CHECK_POS_ERR_INT_CLR** `[14:14]`
    ///
    /// The clear interrupt bit to sign that SOS header marker position wrong when decoding.
    #[inline(always)]
    pub const fn with_scan_check_pos_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SCAN_CHECK_POS_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SCAN_CHECK_POS_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **SCAN_CHECK_NONE_ERR_INT_CLR** `[13:13]`
    ///
    /// The clear interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded.
    #[inline(always)]
    pub const fn with_scan_check_none_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SCAN_CHECK_NONE_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SCAN_CHECK_NONE_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **BS_LAST_BLOCK_EOF_INT_CLR** `[12:12]`
    ///
    /// The clear interrupt bit to sign that the coding process for last block is finished.
    #[inline(always)]
    pub const fn with_bs_last_block_eof_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::BS_LAST_BLOCK_EOF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::BS_LAST_BLOCK_EOF_INT_CLR_MASK
            },
        }
    }

    /// Set **DCT_DONE_INT_CLR** `[11:11]`
    ///
    /// The clear interrupt bit to sign that one dct calculation is finished.
    #[inline(always)]
    pub const fn with_dct_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DCT_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DCT_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **SR_COLOR_MODE_ERR_INT_CLR** `[10:10]`
    ///
    /// The clear interrupt bit to sign that the selected source color mode is not supported.
    #[inline(always)]
    pub const fn with_sr_color_mode_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SR_COLOR_MODE_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SR_COLOR_MODE_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **OUT_EOF_INT_CLR** `[9:9]`
    ///
    /// The clear interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel.
    #[inline(always)]
    pub const fn with_out_eof_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::OUT_EOF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::OUT_EOF_INT_CLR_MASK
            },
        }
    }

    /// Set **RST_CHECK_POS_ERR_INT_CLR** `[8:8]`
    ///
    /// The clear interrupt bit to sign that RST header marker position mismatches with restart interval when decoding.
    #[inline(always)]
    pub const fn with_rst_check_pos_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RST_CHECK_POS_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RST_CHECK_POS_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **RST_CHECK_NONE_ERR_INT_CLR** `[7:7]`
    ///
    /// The clear interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding.
    #[inline(always)]
    pub const fn with_rst_check_none_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RST_CHECK_NONE_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RST_CHECK_NONE_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **RST_UXP_ERR_INT_CLR** `[6:6]`
    ///
    /// The clear interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding.
    #[inline(always)]
    pub const fn with_rst_uxp_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RST_UXP_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RST_UXP_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **C_DQT_ID_ERR_INT_CLR** `[5:5]`
    ///
    /// The clear interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding.
    #[inline(always)]
    pub const fn with_c_dqt_id_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::C_DQT_ID_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::C_DQT_ID_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **C_DHT_AC_ID_ERR_INT_CLR** `[4:4]`
    ///
    /// The clear interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding.
    #[inline(always)]
    pub const fn with_c_dht_ac_id_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::C_DHT_AC_ID_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::C_DHT_AC_ID_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **C_DHT_DC_ID_ERR_INT_CLR** `[3:3]`
    ///
    /// The clear interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding.
    #[inline(always)]
    pub const fn with_c_dht_dc_id_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::C_DHT_DC_ID_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::C_DHT_DC_ID_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **CID_ERR_INT_CLR** `[2:2]`
    ///
    /// The clear interrupt bit to sign that scan id check with component fails when decoding.
    #[inline(always)]
    pub const fn with_cid_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CID_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CID_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **RLE_PARALLEL_ERR_INT_CLR** `[1:1]`
    ///
    /// The clear interrupt bit to sign that rle parallel error when decoding.
    #[inline(always)]
    pub const fn with_rle_parallel_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RLE_PARALLEL_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RLE_PARALLEL_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **DONE_INT_CLR** `[0:0]`
    ///
    /// This clear interrupt bit turns to high level when JPEG finishes encoding a picture..
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

/// STATUS0 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **REG_STATE_YUV** `[31:29]` (RO)
///
///   the state of jpeg fsm
///
/// - **QNRVAL_ZZSCAN_ADDR** `[28:23]` (RO)
///
///   the zig-zag read addr from qnrval_ram
///
/// - **DCTOUT_ZZSCAN_ADDR** `[22:17]` (RO)
///
///   the zig-zag read addr from dctout_ram
///
/// - **BITSTREAM_EOF_VLD_CNT** `[16:11]` (RO)
///
///   the valid bit count for last bitstream
#[repr(transparent)]
pub struct Status0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status0 {
    // ======== Field Constants ========

    /// **REG_STATE_YUV**
    ///
    /// the state of jpeg fsm
    ///
    /// Bits `[31:29]`, mask `0xe0000000`
    pub const REG_STATE_YUV_OFFSET: u32 = 29;
    pub const REG_STATE_YUV_MASK: u32 = 0xe0000000;

    /// **QNRVAL_ZZSCAN_ADDR**
    ///
    /// the zig-zag read addr from qnrval_ram
    ///
    /// Bits `[28:23]`, mask `0x1f800000`
    pub const QNRVAL_ZZSCAN_ADDR_OFFSET: u32 = 23;
    pub const QNRVAL_ZZSCAN_ADDR_MASK: u32 = 0x1f800000;

    /// **DCTOUT_ZZSCAN_ADDR**
    ///
    /// the zig-zag read addr from dctout_ram
    ///
    /// Bits `[22:17]`, mask `0x007e0000`
    pub const DCTOUT_ZZSCAN_ADDR_OFFSET: u32 = 17;
    pub const DCTOUT_ZZSCAN_ADDR_MASK: u32 = 0x007e0000;

    /// **BITSTREAM_EOF_VLD_CNT**
    ///
    /// the valid bit count for last bitstream
    ///
    /// Bits `[16:11]`, mask `0x0001f800`
    pub const BITSTREAM_EOF_VLD_CNT_OFFSET: u32 = 11;
    pub const BITSTREAM_EOF_VLD_CNT_MASK: u32 = 0x0001f800;

    // ======== Field Accessors ========

    /// **REG_STATE_YUV** `[31:29]` (RO)
    ///
    /// the state of jpeg fsm
    #[inline(always)]
    pub fn reg_state_yuv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_STATE_YUV_MASK) >> Self::REG_STATE_YUV_OFFSET
    }

    /// **QNRVAL_ZZSCAN_ADDR** `[28:23]` (RO)
    ///
    /// the zig-zag read addr from qnrval_ram
    #[inline(always)]
    pub fn qnrval_zzscan_addr(&self) -> u32 {
        (self.value.read_bits() & Self::QNRVAL_ZZSCAN_ADDR_MASK) >> Self::QNRVAL_ZZSCAN_ADDR_OFFSET
    }

    /// **DCTOUT_ZZSCAN_ADDR** `[22:17]` (RO)
    ///
    /// the zig-zag read addr from dctout_ram
    #[inline(always)]
    pub fn dctout_zzscan_addr(&self) -> u32 {
        (self.value.read_bits() & Self::DCTOUT_ZZSCAN_ADDR_MASK) >> Self::DCTOUT_ZZSCAN_ADDR_OFFSET
    }

    /// **BITSTREAM_EOF_VLD_CNT** `[16:11]` (RO)
    ///
    /// the valid bit count for last bitstream
    #[inline(always)]
    pub fn bitstream_eof_vld_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::BITSTREAM_EOF_VLD_CNT_MASK)
            >> Self::BITSTREAM_EOF_VLD_CNT_OFFSET
    }
}

/// STATUS0 value type for batch read/modify/write operations
///
/// Use `status0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status0Val {
    bits: u32,
}

impl Status0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_STATE_YUV** `[31:29]`
    ///
    /// the state of jpeg fsm
    #[inline(always)]
    pub const fn reg_state_yuv(&self) -> u32 {
        (self.bits & Status0::REG_STATE_YUV_MASK) >> Status0::REG_STATE_YUV_OFFSET
    }

    /// Read **QNRVAL_ZZSCAN_ADDR** `[28:23]`
    ///
    /// the zig-zag read addr from qnrval_ram
    #[inline(always)]
    pub const fn qnrval_zzscan_addr(&self) -> u32 {
        (self.bits & Status0::QNRVAL_ZZSCAN_ADDR_MASK) >> Status0::QNRVAL_ZZSCAN_ADDR_OFFSET
    }

    /// Read **DCTOUT_ZZSCAN_ADDR** `[22:17]`
    ///
    /// the zig-zag read addr from dctout_ram
    #[inline(always)]
    pub const fn dctout_zzscan_addr(&self) -> u32 {
        (self.bits & Status0::DCTOUT_ZZSCAN_ADDR_MASK) >> Status0::DCTOUT_ZZSCAN_ADDR_OFFSET
    }

    /// Read **BITSTREAM_EOF_VLD_CNT** `[16:11]`
    ///
    /// the valid bit count for last bitstream
    #[inline(always)]
    pub const fn bitstream_eof_vld_cnt(&self) -> u32 {
        (self.bits & Status0::BITSTREAM_EOF_VLD_CNT_MASK) >> Status0::BITSTREAM_EOF_VLD_CNT_OFFSET
    }
}

impl Default for Status0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status0Val {
        Status0Val::from_bits(self.value.read_bits())
    }
}

/// STATUS2 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x004c
/// Reset value: 0x08000000
///
/// ## Fields
///
/// - **PACKFIFO_READY** `[27]` (RO)
///
///   the jpeg pack_fifo ready signal, high active
///
/// - **LAST_DC** `[26]` (RO)
///
///   indicate the encoding process is at the header of the last block of the picture
///
/// - **LAST_MCU** `[25]` (RO)
///
///   indicate the encoding process for the last block of the picture
///
/// - **LAST_BLOCK** `[24]` (RO)
///
///   indicate the encoding process for the last mcu of the picture
///
/// - **SOURCE_PIXEL** `[23:0]` (RO)
///
///   source pixels fetched from dma
#[repr(transparent)]
pub struct Status2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status2 {
    // ======== Field Constants ========

    /// **PACKFIFO_READY**
    ///
    /// the jpeg pack_fifo ready signal, high active
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const PACKFIFO_READY_OFFSET: u32 = 27;
    pub const PACKFIFO_READY_MASK: u32 = 0x08000000;
    pub const PACKFIFO_READY: u32 = 1 << 27;

    /// **LAST_DC**
    ///
    /// indicate the encoding process is at the header of the last block of the picture
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const LAST_DC_OFFSET: u32 = 26;
    pub const LAST_DC_MASK: u32 = 0x04000000;
    pub const LAST_DC: u32 = 1 << 26;

    /// **LAST_MCU**
    ///
    /// indicate the encoding process for the last block of the picture
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const LAST_MCU_OFFSET: u32 = 25;
    pub const LAST_MCU_MASK: u32 = 0x02000000;
    pub const LAST_MCU: u32 = 1 << 25;

    /// **LAST_BLOCK**
    ///
    /// indicate the encoding process for the last mcu of the picture
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const LAST_BLOCK_OFFSET: u32 = 24;
    pub const LAST_BLOCK_MASK: u32 = 0x01000000;
    pub const LAST_BLOCK: u32 = 1 << 24;

    /// **SOURCE_PIXEL**
    ///
    /// source pixels fetched from dma
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const SOURCE_PIXEL_OFFSET: u32 = 0;
    pub const SOURCE_PIXEL_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **PACKFIFO_READY** `[27]` (RO)
    ///
    /// the jpeg pack_fifo ready signal, high active
    #[inline(always)]
    pub fn packfifo_ready(&self) -> bool {
        (self.value.read_bits() & Self::PACKFIFO_READY_MASK) != 0
    }

    /// **LAST_DC** `[26]` (RO)
    ///
    /// indicate the encoding process is at the header of the last block of the picture
    #[inline(always)]
    pub fn last_dc(&self) -> bool {
        (self.value.read_bits() & Self::LAST_DC_MASK) != 0
    }

    /// **LAST_MCU** `[25]` (RO)
    ///
    /// indicate the encoding process for the last block of the picture
    #[inline(always)]
    pub fn last_mcu(&self) -> bool {
        (self.value.read_bits() & Self::LAST_MCU_MASK) != 0
    }

    /// **LAST_BLOCK** `[24]` (RO)
    ///
    /// indicate the encoding process for the last mcu of the picture
    #[inline(always)]
    pub fn last_block(&self) -> bool {
        (self.value.read_bits() & Self::LAST_BLOCK_MASK) != 0
    }

    /// **SOURCE_PIXEL** `[23:0]` (RO)
    ///
    /// source pixels fetched from dma
    #[inline(always)]
    pub fn source_pixel(&self) -> u32 {
        (self.value.read_bits() & Self::SOURCE_PIXEL_MASK) >> Self::SOURCE_PIXEL_OFFSET
    }
}

/// STATUS2 value type for batch read/modify/write operations
///
/// Use `status2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status2Val {
    bits: u32,
}

impl Status2Val {
    /// Create from raw bits
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
        Self { bits: 0x08000000 }
    }

    /// Read **PACKFIFO_READY** `[27:27]`
    ///
    /// the jpeg pack_fifo ready signal, high active
    #[inline(always)]
    pub const fn packfifo_ready(&self) -> bool {
        (self.bits & Status2::PACKFIFO_READY_MASK) != 0
    }

    /// Read **LAST_DC** `[26:26]`
    ///
    /// indicate the encoding process is at the header of the last block of the picture
    #[inline(always)]
    pub const fn last_dc(&self) -> bool {
        (self.bits & Status2::LAST_DC_MASK) != 0
    }

    /// Read **LAST_MCU** `[25:25]`
    ///
    /// indicate the encoding process for the last block of the picture
    #[inline(always)]
    pub const fn last_mcu(&self) -> bool {
        (self.bits & Status2::LAST_MCU_MASK) != 0
    }

    /// Read **LAST_BLOCK** `[24:24]`
    ///
    /// indicate the encoding process for the last mcu of the picture
    #[inline(always)]
    pub const fn last_block(&self) -> bool {
        (self.bits & Status2::LAST_BLOCK_MASK) != 0
    }

    /// Read **SOURCE_PIXEL** `[23:0]`
    ///
    /// source pixels fetched from dma
    #[inline(always)]
    pub const fn source_pixel(&self) -> u32 {
        (self.bits & Status2::SOURCE_PIXEL_MASK) >> Status2::SOURCE_PIXEL_OFFSET
    }
}

impl Default for Status2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Status2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status2Val {
        Status2Val::from_bits(self.value.read_bits())
    }
}

/// STATUS3 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **CR_READY** `[29]` (RO)
///
///   component cr valid signal, high active
///
/// - **CRO** `[28:20]` (RO)
///
///   component cr transferred from rgb input
///
/// - **CB_READY** `[19]` (RO)
///
///   component cb valid signal, high active
///
/// - **CBO** `[18:10]` (RO)
///
///   component cb transferred from rgb input
///
/// - **Y_READY** `[9]` (RO)
///
///   component y valid signal, high active
///
/// - **YO** `[8:0]` (RO)
///
///   component y transferred from rgb input
#[repr(transparent)]
pub struct Status3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status3 {
    // ======== Field Constants ========

    /// **CR_READY**
    ///
    /// component cr valid signal, high active
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CR_READY_OFFSET: u32 = 29;
    pub const CR_READY_MASK: u32 = 0x20000000;
    pub const CR_READY: u32 = 1 << 29;

    /// **CRO**
    ///
    /// component cr transferred from rgb input
    ///
    /// Bits `[28:20]`, mask `0x1ff00000`
    pub const CRO_OFFSET: u32 = 20;
    pub const CRO_MASK: u32 = 0x1ff00000;

    /// **CB_READY**
    ///
    /// component cb valid signal, high active
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CB_READY_OFFSET: u32 = 19;
    pub const CB_READY_MASK: u32 = 0x00080000;
    pub const CB_READY: u32 = 1 << 19;

    /// **CBO**
    ///
    /// component cb transferred from rgb input
    ///
    /// Bits `[18:10]`, mask `0x0007fc00`
    pub const CBO_OFFSET: u32 = 10;
    pub const CBO_MASK: u32 = 0x0007fc00;

    /// **Y_READY**
    ///
    /// component y valid signal, high active
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const Y_READY_OFFSET: u32 = 9;
    pub const Y_READY_MASK: u32 = 0x00000200;
    pub const Y_READY: u32 = 1 << 9;

    /// **YO**
    ///
    /// component y transferred from rgb input
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const YO_OFFSET: u32 = 0;
    pub const YO_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **CR_READY** `[29]` (RO)
    ///
    /// component cr valid signal, high active
    #[inline(always)]
    pub fn cr_ready(&self) -> bool {
        (self.value.read_bits() & Self::CR_READY_MASK) != 0
    }

    /// **CRO** `[28:20]` (RO)
    ///
    /// component cr transferred from rgb input
    #[inline(always)]
    pub fn cro(&self) -> u32 {
        (self.value.read_bits() & Self::CRO_MASK) >> Self::CRO_OFFSET
    }

    /// **CB_READY** `[19]` (RO)
    ///
    /// component cb valid signal, high active
    #[inline(always)]
    pub fn cb_ready(&self) -> bool {
        (self.value.read_bits() & Self::CB_READY_MASK) != 0
    }

    /// **CBO** `[18:10]` (RO)
    ///
    /// component cb transferred from rgb input
    #[inline(always)]
    pub fn cbo(&self) -> u32 {
        (self.value.read_bits() & Self::CBO_MASK) >> Self::CBO_OFFSET
    }

    /// **Y_READY** `[9]` (RO)
    ///
    /// component y valid signal, high active
    #[inline(always)]
    pub fn y_ready(&self) -> bool {
        (self.value.read_bits() & Self::Y_READY_MASK) != 0
    }

    /// **YO** `[8:0]` (RO)
    ///
    /// component y transferred from rgb input
    #[inline(always)]
    pub fn yo(&self) -> u32 {
        (self.value.read_bits() & Self::YO_MASK) >> Self::YO_OFFSET
    }
}

/// STATUS3 value type for batch read/modify/write operations
///
/// Use `status3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status3Val {
    bits: u32,
}

impl Status3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CR_READY** `[29:29]`
    ///
    /// component cr valid signal, high active
    #[inline(always)]
    pub const fn cr_ready(&self) -> bool {
        (self.bits & Status3::CR_READY_MASK) != 0
    }

    /// Read **CRO** `[28:20]`
    ///
    /// component cr transferred from rgb input
    #[inline(always)]
    pub const fn cro(&self) -> u32 {
        (self.bits & Status3::CRO_MASK) >> Status3::CRO_OFFSET
    }

    /// Read **CB_READY** `[19:19]`
    ///
    /// component cb valid signal, high active
    #[inline(always)]
    pub const fn cb_ready(&self) -> bool {
        (self.bits & Status3::CB_READY_MASK) != 0
    }

    /// Read **CBO** `[18:10]`
    ///
    /// component cb transferred from rgb input
    #[inline(always)]
    pub const fn cbo(&self) -> u32 {
        (self.bits & Status3::CBO_MASK) >> Status3::CBO_OFFSET
    }

    /// Read **Y_READY** `[9:9]`
    ///
    /// component y valid signal, high active
    #[inline(always)]
    pub const fn y_ready(&self) -> bool {
        (self.bits & Status3::Y_READY_MASK) != 0
    }

    /// Read **YO** `[8:0]`
    ///
    /// component y transferred from rgb input
    #[inline(always)]
    pub const fn yo(&self) -> u32 {
        (self.bits & Status3::YO_MASK) >> Status3::YO_OFFSET
    }
}

impl Default for Status3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status3Val {
        Status3Val::from_bits(self.value.read_bits())
    }
}

/// STATUS4 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **HFM_BITSTREAM** `[31:0]` (RO)
///
///   the hufman bitstream during encoding process
#[repr(transparent)]
pub struct Status4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status4 {
    // ======== Field Constants ========

    /// **HFM_BITSTREAM**
    ///
    /// the hufman bitstream during encoding process
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const HFM_BITSTREAM_OFFSET: u32 = 0;
    pub const HFM_BITSTREAM_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **HFM_BITSTREAM** `[31:0]` (RO)
    ///
    /// the hufman bitstream during encoding process
    #[inline(always)]
    pub fn hfm_bitstream(&self) -> u32 {
        (self.value.read_bits() & Self::HFM_BITSTREAM_MASK) >> Self::HFM_BITSTREAM_OFFSET
    }
}

/// STATUS4 value type for batch read/modify/write operations
///
/// Use `status4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status4Val {
    bits: u32,
}

impl Status4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HFM_BITSTREAM** `[31:0]`
    ///
    /// the hufman bitstream during encoding process
    #[inline(always)]
    pub const fn hfm_bitstream(&self) -> u32 {
        (self.bits & Status4::HFM_BITSTREAM_MASK) >> Status4::HFM_BITSTREAM_OFFSET
    }
}

impl Default for Status4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status4Val {
        Status4Val::from_bits(self.value.read_bits())
    }
}

/// DHT_TOTLEN_DC0 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **DHT_TOTLEN_DC0** `[31:0]` (RO)
///
///   write the numbers of 1~n codeword length sum from 1~16 of dc0 table
#[repr(transparent)]
pub struct DhtTotlenDc0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DhtTotlenDc0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DhtTotlenDc0 {
    // ======== Field Constants ========

    /// **DHT_TOTLEN_DC0**
    ///
    /// write the numbers of 1~n codeword length sum from 1~16 of dc0 table
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DHT_TOTLEN_DC0_OFFSET: u32 = 0;
    pub const DHT_TOTLEN_DC0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DHT_TOTLEN_DC0** `[31:0]` (RO)
    ///
    /// write the numbers of 1~n codeword length sum from 1~16 of dc0 table
    #[inline(always)]
    pub fn dht_totlen_dc0(&self) -> u32 {
        (self.value.read_bits() & Self::DHT_TOTLEN_DC0_MASK) >> Self::DHT_TOTLEN_DC0_OFFSET
    }
}

/// DHT_TOTLEN_DC0 value type for batch read/modify/write operations
///
/// Use `dhttotlendc0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dhttotlendc0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DhtTotlenDc0Val {
    bits: u32,
}

impl DhtTotlenDc0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DHT_TOTLEN_DC0** `[31:0]`
    ///
    /// write the numbers of 1~n codeword length sum from 1~16 of dc0 table
    #[inline(always)]
    pub const fn dht_totlen_dc0(&self) -> u32 {
        (self.bits & DhtTotlenDc0::DHT_TOTLEN_DC0_MASK) >> DhtTotlenDc0::DHT_TOTLEN_DC0_OFFSET
    }
}

impl Default for DhtTotlenDc0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DhtTotlenDc0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DhtTotlenDc0Val {
        DhtTotlenDc0Val::from_bits(self.value.read_bits())
    }
}

/// DHT_VAl_DC0 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **DHT_VAL_DC0** `[31:0]` (RO)
///
///   write codeword corresponding huffman values of dc0 table
#[repr(transparent)]
pub struct DhtVAlDc0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DhtVAlDc0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DhtVAlDc0 {
    // ======== Field Constants ========

    /// **DHT_VAL_DC0**
    ///
    /// write codeword corresponding huffman values of dc0 table
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DHT_VAL_DC0_OFFSET: u32 = 0;
    pub const DHT_VAL_DC0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DHT_VAL_DC0** `[31:0]` (RO)
    ///
    /// write codeword corresponding huffman values of dc0 table
    #[inline(always)]
    pub fn dht_val_dc0(&self) -> u32 {
        (self.value.read_bits() & Self::DHT_VAL_DC0_MASK) >> Self::DHT_VAL_DC0_OFFSET
    }
}

/// DHT_VAl_DC0 value type for batch read/modify/write operations
///
/// Use `dhtvaldc0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dhtvaldc0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DhtVAlDc0Val {
    bits: u32,
}

impl DhtVAlDc0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DHT_VAL_DC0** `[31:0]`
    ///
    /// write codeword corresponding huffman values of dc0 table
    #[inline(always)]
    pub const fn dht_val_dc0(&self) -> u32 {
        (self.bits & DhtVAlDc0::DHT_VAL_DC0_MASK) >> DhtVAlDc0::DHT_VAL_DC0_OFFSET
    }
}

impl Default for DhtVAlDc0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DhtVAlDc0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DhtVAlDc0Val {
        DhtVAlDc0Val::from_bits(self.value.read_bits())
    }
}

/// DHT_TOTLEN_AC0 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **DHT_TOTLEN_AC0** `[31:0]` (RO)
///
///   write the numbers of 1~n codeword length sum from 1~16 of ac0 table
#[repr(transparent)]
pub struct DhtTotlenAc0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DhtTotlenAc0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DhtTotlenAc0 {
    // ======== Field Constants ========

    /// **DHT_TOTLEN_AC0**
    ///
    /// write the numbers of 1~n codeword length sum from 1~16 of ac0 table
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DHT_TOTLEN_AC0_OFFSET: u32 = 0;
    pub const DHT_TOTLEN_AC0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DHT_TOTLEN_AC0** `[31:0]` (RO)
    ///
    /// write the numbers of 1~n codeword length sum from 1~16 of ac0 table
    #[inline(always)]
    pub fn dht_totlen_ac0(&self) -> u32 {
        (self.value.read_bits() & Self::DHT_TOTLEN_AC0_MASK) >> Self::DHT_TOTLEN_AC0_OFFSET
    }
}

/// DHT_TOTLEN_AC0 value type for batch read/modify/write operations
///
/// Use `dhttotlenac0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dhttotlenac0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DhtTotlenAc0Val {
    bits: u32,
}

impl DhtTotlenAc0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DHT_TOTLEN_AC0** `[31:0]`
    ///
    /// write the numbers of 1~n codeword length sum from 1~16 of ac0 table
    #[inline(always)]
    pub const fn dht_totlen_ac0(&self) -> u32 {
        (self.bits & DhtTotlenAc0::DHT_TOTLEN_AC0_MASK) >> DhtTotlenAc0::DHT_TOTLEN_AC0_OFFSET
    }
}

impl Default for DhtTotlenAc0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DhtTotlenAc0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DhtTotlenAc0Val {
        DhtTotlenAc0Val::from_bits(self.value.read_bits())
    }
}

/// DHT_VAl_AC0 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **DHT_VAL_AC0** `[31:0]` (RO)
///
///   write codeword corresponding huffman values of ac0 table
#[repr(transparent)]
pub struct DhtVAlAc0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DhtVAlAc0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DhtVAlAc0 {
    // ======== Field Constants ========

    /// **DHT_VAL_AC0**
    ///
    /// write codeword corresponding huffman values of ac0 table
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DHT_VAL_AC0_OFFSET: u32 = 0;
    pub const DHT_VAL_AC0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DHT_VAL_AC0** `[31:0]` (RO)
    ///
    /// write codeword corresponding huffman values of ac0 table
    #[inline(always)]
    pub fn dht_val_ac0(&self) -> u32 {
        (self.value.read_bits() & Self::DHT_VAL_AC0_MASK) >> Self::DHT_VAL_AC0_OFFSET
    }
}

/// DHT_VAl_AC0 value type for batch read/modify/write operations
///
/// Use `dhtvalac0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dhtvalac0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DhtVAlAc0Val {
    bits: u32,
}

impl DhtVAlAc0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DHT_VAL_AC0** `[31:0]`
    ///
    /// write codeword corresponding huffman values of ac0 table
    #[inline(always)]
    pub const fn dht_val_ac0(&self) -> u32 {
        (self.bits & DhtVAlAc0::DHT_VAL_AC0_MASK) >> DhtVAlAc0::DHT_VAL_AC0_OFFSET
    }
}

impl Default for DhtVAlAc0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DhtVAlAc0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DhtVAlAc0Val {
        DhtVAlAc0Val::from_bits(self.value.read_bits())
    }
}

/// DHT_TOTLEN_DC1 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **DHT_TOTLEN_DC1** `[31:0]` (RO)
///
///   write the numbers of 1~n codeword length sum from 1~16 of dc1 table
#[repr(transparent)]
pub struct DhtTotlenDc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DhtTotlenDc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DhtTotlenDc1 {
    // ======== Field Constants ========

    /// **DHT_TOTLEN_DC1**
    ///
    /// write the numbers of 1~n codeword length sum from 1~16 of dc1 table
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DHT_TOTLEN_DC1_OFFSET: u32 = 0;
    pub const DHT_TOTLEN_DC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DHT_TOTLEN_DC1** `[31:0]` (RO)
    ///
    /// write the numbers of 1~n codeword length sum from 1~16 of dc1 table
    #[inline(always)]
    pub fn dht_totlen_dc1(&self) -> u32 {
        (self.value.read_bits() & Self::DHT_TOTLEN_DC1_MASK) >> Self::DHT_TOTLEN_DC1_OFFSET
    }
}

/// DHT_TOTLEN_DC1 value type for batch read/modify/write operations
///
/// Use `dhttotlendc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dhttotlendc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DhtTotlenDc1Val {
    bits: u32,
}

impl DhtTotlenDc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DHT_TOTLEN_DC1** `[31:0]`
    ///
    /// write the numbers of 1~n codeword length sum from 1~16 of dc1 table
    #[inline(always)]
    pub const fn dht_totlen_dc1(&self) -> u32 {
        (self.bits & DhtTotlenDc1::DHT_TOTLEN_DC1_MASK) >> DhtTotlenDc1::DHT_TOTLEN_DC1_OFFSET
    }
}

impl Default for DhtTotlenDc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DhtTotlenDc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DhtTotlenDc1Val {
        DhtTotlenDc1Val::from_bits(self.value.read_bits())
    }
}

/// DHT_VAl_DC1 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **DHT_VAL_DC1** `[31:0]` (RO)
///
///   write codeword corresponding huffman values of dc1 table
#[repr(transparent)]
pub struct DhtVAlDc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DhtVAlDc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DhtVAlDc1 {
    // ======== Field Constants ========

    /// **DHT_VAL_DC1**
    ///
    /// write codeword corresponding huffman values of dc1 table
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DHT_VAL_DC1_OFFSET: u32 = 0;
    pub const DHT_VAL_DC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DHT_VAL_DC1** `[31:0]` (RO)
    ///
    /// write codeword corresponding huffman values of dc1 table
    #[inline(always)]
    pub fn dht_val_dc1(&self) -> u32 {
        (self.value.read_bits() & Self::DHT_VAL_DC1_MASK) >> Self::DHT_VAL_DC1_OFFSET
    }
}

/// DHT_VAl_DC1 value type for batch read/modify/write operations
///
/// Use `dhtvaldc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dhtvaldc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DhtVAlDc1Val {
    bits: u32,
}

impl DhtVAlDc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DHT_VAL_DC1** `[31:0]`
    ///
    /// write codeword corresponding huffman values of dc1 table
    #[inline(always)]
    pub const fn dht_val_dc1(&self) -> u32 {
        (self.bits & DhtVAlDc1::DHT_VAL_DC1_MASK) >> DhtVAlDc1::DHT_VAL_DC1_OFFSET
    }
}

impl Default for DhtVAlDc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DhtVAlDc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DhtVAlDc1Val {
        DhtVAlDc1Val::from_bits(self.value.read_bits())
    }
}

/// DHT_TOTLEN_AC1 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **DHT_TOTLEN_AC1** `[31:0]` (RO)
///
///   write the numbers of 1~n codeword length sum from 1~16 of ac1 table
#[repr(transparent)]
pub struct DhtTotlenAc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DhtTotlenAc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DhtTotlenAc1 {
    // ======== Field Constants ========

    /// **DHT_TOTLEN_AC1**
    ///
    /// write the numbers of 1~n codeword length sum from 1~16 of ac1 table
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DHT_TOTLEN_AC1_OFFSET: u32 = 0;
    pub const DHT_TOTLEN_AC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DHT_TOTLEN_AC1** `[31:0]` (RO)
    ///
    /// write the numbers of 1~n codeword length sum from 1~16 of ac1 table
    #[inline(always)]
    pub fn dht_totlen_ac1(&self) -> u32 {
        (self.value.read_bits() & Self::DHT_TOTLEN_AC1_MASK) >> Self::DHT_TOTLEN_AC1_OFFSET
    }
}

/// DHT_TOTLEN_AC1 value type for batch read/modify/write operations
///
/// Use `dhttotlenac1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dhttotlenac1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DhtTotlenAc1Val {
    bits: u32,
}

impl DhtTotlenAc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DHT_TOTLEN_AC1** `[31:0]`
    ///
    /// write the numbers of 1~n codeword length sum from 1~16 of ac1 table
    #[inline(always)]
    pub const fn dht_totlen_ac1(&self) -> u32 {
        (self.bits & DhtTotlenAc1::DHT_TOTLEN_AC1_MASK) >> DhtTotlenAc1::DHT_TOTLEN_AC1_OFFSET
    }
}

impl Default for DhtTotlenAc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DhtTotlenAc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DhtTotlenAc1Val {
        DhtTotlenAc1Val::from_bits(self.value.read_bits())
    }
}

/// DHT_VAl_AC1 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **DHT_VAL_AC1** `[31:0]` (RO)
///
///   write codeword corresponding huffman values of ac1 table
#[repr(transparent)]
pub struct DhtVAlAc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DhtVAlAc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DhtVAlAc1 {
    // ======== Field Constants ========

    /// **DHT_VAL_AC1**
    ///
    /// write codeword corresponding huffman values of ac1 table
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DHT_VAL_AC1_OFFSET: u32 = 0;
    pub const DHT_VAL_AC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DHT_VAL_AC1** `[31:0]` (RO)
    ///
    /// write codeword corresponding huffman values of ac1 table
    #[inline(always)]
    pub fn dht_val_ac1(&self) -> u32 {
        (self.value.read_bits() & Self::DHT_VAL_AC1_MASK) >> Self::DHT_VAL_AC1_OFFSET
    }
}

/// DHT_VAl_AC1 value type for batch read/modify/write operations
///
/// Use `dhtvalac1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dhtvalac1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DhtVAlAc1Val {
    bits: u32,
}

impl DhtVAlAc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DHT_VAL_AC1** `[31:0]`
    ///
    /// write codeword corresponding huffman values of ac1 table
    #[inline(always)]
    pub const fn dht_val_ac1(&self) -> u32 {
        (self.bits & DhtVAlAc1::DHT_VAL_AC1_MASK) >> DhtVAlAc1::DHT_VAL_AC1_OFFSET
    }
}

impl Default for DhtVAlAc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DhtVAlAc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DhtVAlAc1Val {
        DhtVAlAc1Val::from_bits(self.value.read_bits())
    }
}

/// DHT_CODEMIN_DC0 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **DHT_CODEMIN_DC0** `[31:0]` (RO)
///
///   write the minimum codeword of code length from 1~16 of dc0 table. The codeword is left shifted to the MSB position of a 16bit word
#[repr(transparent)]
pub struct DhtCodeminDc0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DhtCodeminDc0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DhtCodeminDc0 {
    // ======== Field Constants ========

    /// **DHT_CODEMIN_DC0**
    ///
    /// write the minimum codeword of code length from 1~16 of dc0 table. The codeword is left shifted to the MSB position of a 16bit word
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DHT_CODEMIN_DC0_OFFSET: u32 = 0;
    pub const DHT_CODEMIN_DC0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DHT_CODEMIN_DC0** `[31:0]` (RO)
    ///
    /// write the minimum codeword of code length from 1~16 of dc0 table. The codeword is left shifted to the MSB position of a 16bit word
    #[inline(always)]
    pub fn dht_codemin_dc0(&self) -> u32 {
        (self.value.read_bits() & Self::DHT_CODEMIN_DC0_MASK) >> Self::DHT_CODEMIN_DC0_OFFSET
    }
}

/// DHT_CODEMIN_DC0 value type for batch read/modify/write operations
///
/// Use `dhtcodemindc0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dhtcodemindc0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DhtCodeminDc0Val {
    bits: u32,
}

impl DhtCodeminDc0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DHT_CODEMIN_DC0** `[31:0]`
    ///
    /// write the minimum codeword of code length from 1~16 of dc0 table. The codeword is left shifted to the MSB position of a 16bit word
    #[inline(always)]
    pub const fn dht_codemin_dc0(&self) -> u32 {
        (self.bits & DhtCodeminDc0::DHT_CODEMIN_DC0_MASK) >> DhtCodeminDc0::DHT_CODEMIN_DC0_OFFSET
    }
}

impl Default for DhtCodeminDc0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DhtCodeminDc0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DhtCodeminDc0Val {
        DhtCodeminDc0Val::from_bits(self.value.read_bits())
    }
}

/// DHT_CODEMIN_AC0 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **DHT_CODEMIN_AC0** `[31:0]` (RO)
///
///   write the minimum codeword of code length from 1~16 of ac0 table. The codeword is left shifted to the MSB position of a 16bit word
#[repr(transparent)]
pub struct DhtCodeminAc0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DhtCodeminAc0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DhtCodeminAc0 {
    // ======== Field Constants ========

    /// **DHT_CODEMIN_AC0**
    ///
    /// write the minimum codeword of code length from 1~16 of ac0 table. The codeword is left shifted to the MSB position of a 16bit word
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DHT_CODEMIN_AC0_OFFSET: u32 = 0;
    pub const DHT_CODEMIN_AC0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DHT_CODEMIN_AC0** `[31:0]` (RO)
    ///
    /// write the minimum codeword of code length from 1~16 of ac0 table. The codeword is left shifted to the MSB position of a 16bit word
    #[inline(always)]
    pub fn dht_codemin_ac0(&self) -> u32 {
        (self.value.read_bits() & Self::DHT_CODEMIN_AC0_MASK) >> Self::DHT_CODEMIN_AC0_OFFSET
    }
}

/// DHT_CODEMIN_AC0 value type for batch read/modify/write operations
///
/// Use `dhtcodeminac0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dhtcodeminac0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DhtCodeminAc0Val {
    bits: u32,
}

impl DhtCodeminAc0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DHT_CODEMIN_AC0** `[31:0]`
    ///
    /// write the minimum codeword of code length from 1~16 of ac0 table. The codeword is left shifted to the MSB position of a 16bit word
    #[inline(always)]
    pub const fn dht_codemin_ac0(&self) -> u32 {
        (self.bits & DhtCodeminAc0::DHT_CODEMIN_AC0_MASK) >> DhtCodeminAc0::DHT_CODEMIN_AC0_OFFSET
    }
}

impl Default for DhtCodeminAc0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DhtCodeminAc0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DhtCodeminAc0Val {
        DhtCodeminAc0Val::from_bits(self.value.read_bits())
    }
}

/// DHT_CODEMIN_DC1 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0080
///
/// ## Fields
///
/// - **DHT_CODEMIN_DC1** `[31:0]` (RO)
///
///   write the minimum codeword of code length from 1~16 of dc1 table. The codeword is left shifted to the MSB position of a 16bit word
#[repr(transparent)]
pub struct DhtCodeminDc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DhtCodeminDc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DhtCodeminDc1 {
    // ======== Field Constants ========

    /// **DHT_CODEMIN_DC1**
    ///
    /// write the minimum codeword of code length from 1~16 of dc1 table. The codeword is left shifted to the MSB position of a 16bit word
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DHT_CODEMIN_DC1_OFFSET: u32 = 0;
    pub const DHT_CODEMIN_DC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DHT_CODEMIN_DC1** `[31:0]` (RO)
    ///
    /// write the minimum codeword of code length from 1~16 of dc1 table. The codeword is left shifted to the MSB position of a 16bit word
    #[inline(always)]
    pub fn dht_codemin_dc1(&self) -> u32 {
        (self.value.read_bits() & Self::DHT_CODEMIN_DC1_MASK) >> Self::DHT_CODEMIN_DC1_OFFSET
    }
}

/// DHT_CODEMIN_DC1 value type for batch read/modify/write operations
///
/// Use `dhtcodemindc1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dhtcodemindc1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DhtCodeminDc1Val {
    bits: u32,
}

impl DhtCodeminDc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DHT_CODEMIN_DC1** `[31:0]`
    ///
    /// write the minimum codeword of code length from 1~16 of dc1 table. The codeword is left shifted to the MSB position of a 16bit word
    #[inline(always)]
    pub const fn dht_codemin_dc1(&self) -> u32 {
        (self.bits & DhtCodeminDc1::DHT_CODEMIN_DC1_MASK) >> DhtCodeminDc1::DHT_CODEMIN_DC1_OFFSET
    }
}

impl Default for DhtCodeminDc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DhtCodeminDc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DhtCodeminDc1Val {
        DhtCodeminDc1Val::from_bits(self.value.read_bits())
    }
}

/// DHT_CODEMIN_AC1 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **DHT_CODEMIN_AC1** `[31:0]` (RO)
///
///   write the minimum codeword of code length from 1~16 of ac1 table. The codeword is left shifted to the MSB position of a 16bit word
#[repr(transparent)]
pub struct DhtCodeminAc1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DhtCodeminAc1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DhtCodeminAc1 {
    // ======== Field Constants ========

    /// **DHT_CODEMIN_AC1**
    ///
    /// write the minimum codeword of code length from 1~16 of ac1 table. The codeword is left shifted to the MSB position of a 16bit word
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DHT_CODEMIN_AC1_OFFSET: u32 = 0;
    pub const DHT_CODEMIN_AC1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DHT_CODEMIN_AC1** `[31:0]` (RO)
    ///
    /// write the minimum codeword of code length from 1~16 of ac1 table. The codeword is left shifted to the MSB position of a 16bit word
    #[inline(always)]
    pub fn dht_codemin_ac1(&self) -> u32 {
        (self.value.read_bits() & Self::DHT_CODEMIN_AC1_MASK) >> Self::DHT_CODEMIN_AC1_OFFSET
    }
}

/// DHT_CODEMIN_AC1 value type for batch read/modify/write operations
///
/// Use `dhtcodeminac1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dhtcodeminac1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DhtCodeminAc1Val {
    bits: u32,
}

impl DhtCodeminAc1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DHT_CODEMIN_AC1** `[31:0]`
    ///
    /// write the minimum codeword of code length from 1~16 of ac1 table. The codeword is left shifted to the MSB position of a 16bit word
    #[inline(always)]
    pub const fn dht_codemin_ac1(&self) -> u32 {
        (self.bits & DhtCodeminAc1::DHT_CODEMIN_AC1_MASK) >> DhtCodeminAc1::DHT_CODEMIN_AC1_OFFSET
    }
}

impl Default for DhtCodeminAc1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DhtCodeminAc1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DhtCodeminAc1Val {
        DhtCodeminAc1Val::from_bits(self.value.read_bits())
    }
}

/// DECODER_STATUS0 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0088
///
/// ## Fields
///
/// - **DECODE_SAMPLE_SEL** `[31:30]` (RO)
///
///   Reserved
///
/// - **HEADER_DEC_ST** `[29:26]` (RO)
///
///   Reserved
///
/// - **DECODE_BYTE_CNT** `[25:0]` (RO)
///
///   Reserved
#[repr(transparent)]
pub struct DecoderStatus0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DecoderStatus0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DecoderStatus0 {
    // ======== Field Constants ========

    /// **DECODE_SAMPLE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const DECODE_SAMPLE_SEL_OFFSET: u32 = 30;
    pub const DECODE_SAMPLE_SEL_MASK: u32 = 0xc0000000;

    /// **HEADER_DEC_ST**
    ///
    /// Reserved
    ///
    /// Bits `[29:26]`, mask `0x3c000000`
    pub const HEADER_DEC_ST_OFFSET: u32 = 26;
    pub const HEADER_DEC_ST_MASK: u32 = 0x3c000000;

    /// **DECODE_BYTE_CNT**
    ///
    /// Reserved
    ///
    /// Bits `[25:0]`, mask `0x03ffffff`
    pub const DECODE_BYTE_CNT_OFFSET: u32 = 0;
    pub const DECODE_BYTE_CNT_MASK: u32 = 0x03ffffff;

    // ======== Field Accessors ========

    /// **DECODE_SAMPLE_SEL** `[31:30]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn decode_sample_sel(&self) -> u32 {
        (self.value.read_bits() & Self::DECODE_SAMPLE_SEL_MASK) >> Self::DECODE_SAMPLE_SEL_OFFSET
    }

    /// **HEADER_DEC_ST** `[29:26]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn header_dec_st(&self) -> u32 {
        (self.value.read_bits() & Self::HEADER_DEC_ST_MASK) >> Self::HEADER_DEC_ST_OFFSET
    }

    /// **DECODE_BYTE_CNT** `[25:0]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn decode_byte_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::DECODE_BYTE_CNT_MASK) >> Self::DECODE_BYTE_CNT_OFFSET
    }
}

/// DECODER_STATUS0 value type for batch read/modify/write operations
///
/// Use `decoderstatus0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `decoderstatus0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DecoderStatus0Val {
    bits: u32,
}

impl DecoderStatus0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DECODE_SAMPLE_SEL** `[31:30]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn decode_sample_sel(&self) -> u32 {
        (self.bits & DecoderStatus0::DECODE_SAMPLE_SEL_MASK)
            >> DecoderStatus0::DECODE_SAMPLE_SEL_OFFSET
    }

    /// Read **HEADER_DEC_ST** `[29:26]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn header_dec_st(&self) -> u32 {
        (self.bits & DecoderStatus0::HEADER_DEC_ST_MASK) >> DecoderStatus0::HEADER_DEC_ST_OFFSET
    }

    /// Read **DECODE_BYTE_CNT** `[25:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn decode_byte_cnt(&self) -> u32 {
        (self.bits & DecoderStatus0::DECODE_BYTE_CNT_MASK) >> DecoderStatus0::DECODE_BYTE_CNT_OFFSET
    }
}

impl Default for DecoderStatus0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DecoderStatus0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DecoderStatus0Val {
        DecoderStatus0Val::from_bits(self.value.read_bits())
    }
}

/// DECODER_STATUS1 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x008c
///
/// ## Fields
///
/// - **DECODE_DATA** `[31:24]` (RO)
///
///   Reserved
///
/// - **MCU_FSM_READY** `[23]` (RO)
///
///   Reserved
///
/// - **COUNT_Q** `[22:16]` (RO)
///
///   Reserved
///
/// - **ENCODE_DATA** `[15:0]` (RO)
///
///   Reserved
#[repr(transparent)]
pub struct DecoderStatus1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DecoderStatus1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DecoderStatus1 {
    // ======== Field Constants ========

    /// **DECODE_DATA**
    ///
    /// Reserved
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const DECODE_DATA_OFFSET: u32 = 24;
    pub const DECODE_DATA_MASK: u32 = 0xff000000;

    /// **MCU_FSM_READY**
    ///
    /// Reserved
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const MCU_FSM_READY_OFFSET: u32 = 23;
    pub const MCU_FSM_READY_MASK: u32 = 0x00800000;
    pub const MCU_FSM_READY: u32 = 1 << 23;

    /// **COUNT_Q**
    ///
    /// Reserved
    ///
    /// Bits `[22:16]`, mask `0x007f0000`
    pub const COUNT_Q_OFFSET: u32 = 16;
    pub const COUNT_Q_MASK: u32 = 0x007f0000;

    /// **ENCODE_DATA**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const ENCODE_DATA_OFFSET: u32 = 0;
    pub const ENCODE_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **DECODE_DATA** `[31:24]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn decode_data(&self) -> u32 {
        (self.value.read_bits() & Self::DECODE_DATA_MASK) >> Self::DECODE_DATA_OFFSET
    }

    /// **MCU_FSM_READY** `[23]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn mcu_fsm_ready(&self) -> bool {
        (self.value.read_bits() & Self::MCU_FSM_READY_MASK) != 0
    }

    /// **COUNT_Q** `[22:16]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn count_q(&self) -> u32 {
        (self.value.read_bits() & Self::COUNT_Q_MASK) >> Self::COUNT_Q_OFFSET
    }

    /// **ENCODE_DATA** `[15:0]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn encode_data(&self) -> u32 {
        (self.value.read_bits() & Self::ENCODE_DATA_MASK) >> Self::ENCODE_DATA_OFFSET
    }
}

/// DECODER_STATUS1 value type for batch read/modify/write operations
///
/// Use `decoderstatus1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `decoderstatus1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DecoderStatus1Val {
    bits: u32,
}

impl DecoderStatus1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DECODE_DATA** `[31:24]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn decode_data(&self) -> u32 {
        (self.bits & DecoderStatus1::DECODE_DATA_MASK) >> DecoderStatus1::DECODE_DATA_OFFSET
    }

    /// Read **MCU_FSM_READY** `[23:23]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn mcu_fsm_ready(&self) -> bool {
        (self.bits & DecoderStatus1::MCU_FSM_READY_MASK) != 0
    }

    /// Read **COUNT_Q** `[22:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn count_q(&self) -> u32 {
        (self.bits & DecoderStatus1::COUNT_Q_MASK) >> DecoderStatus1::COUNT_Q_OFFSET
    }

    /// Read **ENCODE_DATA** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn encode_data(&self) -> u32 {
        (self.bits & DecoderStatus1::ENCODE_DATA_MASK) >> DecoderStatus1::ENCODE_DATA_OFFSET
    }
}

impl Default for DecoderStatus1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DecoderStatus1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DecoderStatus1Val {
        DecoderStatus1Val::from_bits(self.value.read_bits())
    }
}

/// DECODER_STATUS2 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0090
///
/// ## Fields
///
/// - **MCU_IN_PROC** `[31]` (RO)
///
///   Reserved
///
/// - **SCAN_CHECK_WAIT** `[30]` (RO)
///
///   Reserved
///
/// - **RST_CHECK_WAIT** `[29]` (RO)
///
///   Reserved
///
/// - **SCAN_NUM** `[28:26]` (RO)
///
///   Reserved
///
/// - **COMP_BLOCK_NUM** `[25:0]` (RO)
///
///   Reserved
#[repr(transparent)]
pub struct DecoderStatus2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DecoderStatus2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DecoderStatus2 {
    // ======== Field Constants ========

    /// **MCU_IN_PROC**
    ///
    /// Reserved
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const MCU_IN_PROC_OFFSET: u32 = 31;
    pub const MCU_IN_PROC_MASK: u32 = 0x80000000;
    pub const MCU_IN_PROC: u32 = 1 << 31;

    /// **SCAN_CHECK_WAIT**
    ///
    /// Reserved
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SCAN_CHECK_WAIT_OFFSET: u32 = 30;
    pub const SCAN_CHECK_WAIT_MASK: u32 = 0x40000000;
    pub const SCAN_CHECK_WAIT: u32 = 1 << 30;

    /// **RST_CHECK_WAIT**
    ///
    /// Reserved
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const RST_CHECK_WAIT_OFFSET: u32 = 29;
    pub const RST_CHECK_WAIT_MASK: u32 = 0x20000000;
    pub const RST_CHECK_WAIT: u32 = 1 << 29;

    /// **SCAN_NUM**
    ///
    /// Reserved
    ///
    /// Bits `[28:26]`, mask `0x1c000000`
    pub const SCAN_NUM_OFFSET: u32 = 26;
    pub const SCAN_NUM_MASK: u32 = 0x1c000000;

    /// **COMP_BLOCK_NUM**
    ///
    /// Reserved
    ///
    /// Bits `[25:0]`, mask `0x03ffffff`
    pub const COMP_BLOCK_NUM_OFFSET: u32 = 0;
    pub const COMP_BLOCK_NUM_MASK: u32 = 0x03ffffff;

    // ======== Field Accessors ========

    /// **MCU_IN_PROC** `[31]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn mcu_in_proc(&self) -> bool {
        (self.value.read_bits() & Self::MCU_IN_PROC_MASK) != 0
    }

    /// **SCAN_CHECK_WAIT** `[30]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn scan_check_wait(&self) -> bool {
        (self.value.read_bits() & Self::SCAN_CHECK_WAIT_MASK) != 0
    }

    /// **RST_CHECK_WAIT** `[29]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn rst_check_wait(&self) -> bool {
        (self.value.read_bits() & Self::RST_CHECK_WAIT_MASK) != 0
    }

    /// **SCAN_NUM** `[28:26]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn scan_num(&self) -> u32 {
        (self.value.read_bits() & Self::SCAN_NUM_MASK) >> Self::SCAN_NUM_OFFSET
    }

    /// **COMP_BLOCK_NUM** `[25:0]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn comp_block_num(&self) -> u32 {
        (self.value.read_bits() & Self::COMP_BLOCK_NUM_MASK) >> Self::COMP_BLOCK_NUM_OFFSET
    }
}

/// DECODER_STATUS2 value type for batch read/modify/write operations
///
/// Use `decoderstatus2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `decoderstatus2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DecoderStatus2Val {
    bits: u32,
}

impl DecoderStatus2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MCU_IN_PROC** `[31:31]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn mcu_in_proc(&self) -> bool {
        (self.bits & DecoderStatus2::MCU_IN_PROC_MASK) != 0
    }

    /// Read **SCAN_CHECK_WAIT** `[30:30]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn scan_check_wait(&self) -> bool {
        (self.bits & DecoderStatus2::SCAN_CHECK_WAIT_MASK) != 0
    }

    /// Read **RST_CHECK_WAIT** `[29:29]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn rst_check_wait(&self) -> bool {
        (self.bits & DecoderStatus2::RST_CHECK_WAIT_MASK) != 0
    }

    /// Read **SCAN_NUM** `[28:26]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn scan_num(&self) -> u32 {
        (self.bits & DecoderStatus2::SCAN_NUM_MASK) >> DecoderStatus2::SCAN_NUM_OFFSET
    }

    /// Read **COMP_BLOCK_NUM** `[25:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn comp_block_num(&self) -> u32 {
        (self.bits & DecoderStatus2::COMP_BLOCK_NUM_MASK) >> DecoderStatus2::COMP_BLOCK_NUM_OFFSET
    }
}

impl Default for DecoderStatus2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DecoderStatus2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DecoderStatus2Val {
        DecoderStatus2Val::from_bits(self.value.read_bits())
    }
}

/// DECODER_STATUS3 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0094
///
/// ## Fields
///
/// - **LOOKUP_DATA** `[31:0]` (RO)
///
///   Reserved
#[repr(transparent)]
pub struct DecoderStatus3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DecoderStatus3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DecoderStatus3 {
    // ======== Field Constants ========

    /// **LOOKUP_DATA**
    ///
    /// Reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LOOKUP_DATA_OFFSET: u32 = 0;
    pub const LOOKUP_DATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LOOKUP_DATA** `[31:0]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lookup_data(&self) -> u32 {
        (self.value.read_bits() & Self::LOOKUP_DATA_MASK) >> Self::LOOKUP_DATA_OFFSET
    }
}

/// DECODER_STATUS3 value type for batch read/modify/write operations
///
/// Use `decoderstatus3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `decoderstatus3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DecoderStatus3Val {
    bits: u32,
}

impl DecoderStatus3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LOOKUP_DATA** `[31:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lookup_data(&self) -> u32 {
        (self.bits & DecoderStatus3::LOOKUP_DATA_MASK) >> DecoderStatus3::LOOKUP_DATA_OFFSET
    }
}

impl Default for DecoderStatus3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DecoderStatus3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DecoderStatus3Val {
        DecoderStatus3Val::from_bits(self.value.read_bits())
    }
}

/// DECODER_STATUS4 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x0098
///
/// ## Fields
///
/// - **DE_DMA2D_IN_PUSH** `[28]` (RO)
///
///   Reserved
///
/// - **DE_FRAME_EOF_CHECK** `[27]` (RO)
///
///   Reserved
///
/// - **DEZIGZAG_READY** `[26]` (RO)
///
///   Reserved
///
/// - **BLOCK_EOF_CNT** `[25:0]` (RO)
///
///   Reserved
#[repr(transparent)]
pub struct DecoderStatus4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DecoderStatus4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DecoderStatus4 {
    // ======== Field Constants ========

    /// **DE_DMA2D_IN_PUSH**
    ///
    /// Reserved
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const DE_DMA2D_IN_PUSH_OFFSET: u32 = 28;
    pub const DE_DMA2D_IN_PUSH_MASK: u32 = 0x10000000;
    pub const DE_DMA2D_IN_PUSH: u32 = 1 << 28;

    /// **DE_FRAME_EOF_CHECK**
    ///
    /// Reserved
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const DE_FRAME_EOF_CHECK_OFFSET: u32 = 27;
    pub const DE_FRAME_EOF_CHECK_MASK: u32 = 0x08000000;
    pub const DE_FRAME_EOF_CHECK: u32 = 1 << 27;

    /// **DEZIGZAG_READY**
    ///
    /// Reserved
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const DEZIGZAG_READY_OFFSET: u32 = 26;
    pub const DEZIGZAG_READY_MASK: u32 = 0x04000000;
    pub const DEZIGZAG_READY: u32 = 1 << 26;

    /// **BLOCK_EOF_CNT**
    ///
    /// Reserved
    ///
    /// Bits `[25:0]`, mask `0x03ffffff`
    pub const BLOCK_EOF_CNT_OFFSET: u32 = 0;
    pub const BLOCK_EOF_CNT_MASK: u32 = 0x03ffffff;

    // ======== Field Accessors ========

    /// **DE_DMA2D_IN_PUSH** `[28]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn de_dma2d_in_push(&self) -> bool {
        (self.value.read_bits() & Self::DE_DMA2D_IN_PUSH_MASK) != 0
    }

    /// **DE_FRAME_EOF_CHECK** `[27]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn de_frame_eof_check(&self) -> bool {
        (self.value.read_bits() & Self::DE_FRAME_EOF_CHECK_MASK) != 0
    }

    /// **DEZIGZAG_READY** `[26]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn dezigzag_ready(&self) -> bool {
        (self.value.read_bits() & Self::DEZIGZAG_READY_MASK) != 0
    }

    /// **BLOCK_EOF_CNT** `[25:0]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn block_eof_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::BLOCK_EOF_CNT_MASK) >> Self::BLOCK_EOF_CNT_OFFSET
    }
}

/// DECODER_STATUS4 value type for batch read/modify/write operations
///
/// Use `decoderstatus4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `decoderstatus4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DecoderStatus4Val {
    bits: u32,
}

impl DecoderStatus4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DE_DMA2D_IN_PUSH** `[28:28]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn de_dma2d_in_push(&self) -> bool {
        (self.bits & DecoderStatus4::DE_DMA2D_IN_PUSH_MASK) != 0
    }

    /// Read **DE_FRAME_EOF_CHECK** `[27:27]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn de_frame_eof_check(&self) -> bool {
        (self.bits & DecoderStatus4::DE_FRAME_EOF_CHECK_MASK) != 0
    }

    /// Read **DEZIGZAG_READY** `[26:26]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn dezigzag_ready(&self) -> bool {
        (self.bits & DecoderStatus4::DEZIGZAG_READY_MASK) != 0
    }

    /// Read **BLOCK_EOF_CNT** `[25:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn block_eof_cnt(&self) -> u32 {
        (self.bits & DecoderStatus4::BLOCK_EOF_CNT_MASK) >> DecoderStatus4::BLOCK_EOF_CNT_OFFSET
    }
}

impl Default for DecoderStatus4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DecoderStatus4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DecoderStatus4Val {
        DecoderStatus4Val::from_bits(self.value.read_bits())
    }
}

/// DECODER_STATUS5 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x009c
///
/// ## Fields
///
/// - **RDN_ENA** `[30]` (RW)
///
///   redundant control registers for jpeg
///
/// - **RDN_RESULT** `[29]` (RO)
///
///   redundant registers for jpeg
///
/// - **DATA_LAST_O** `[28]` (RO)
///
///   Reserved
///
/// - **NS3** `[27:25]` (RO)
///
///   Reserved
///
/// - **NS2** `[24:22]` (RO)
///
///   Reserved
///
/// - **NS1** `[21:19]` (RO)
///
///   Reserved
///
/// - **NS0** `[18:16]` (RO)
///
///   Reserved
///
/// - **IDCT_HFM_DATA** `[15:0]` (RO)
///
///   Reserved
#[repr(transparent)]
pub struct DecoderStatus5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DecoderStatus5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DecoderStatus5 {
    // ======== Field Constants ========

    /// **RDN_ENA**
    ///
    /// redundant control registers for jpeg
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const RDN_ENA_OFFSET: u32 = 30;
    pub const RDN_ENA_MASK: u32 = 0x40000000;
    pub const RDN_ENA: u32 = 1 << 30;

    /// **RDN_RESULT**
    ///
    /// redundant registers for jpeg
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const RDN_RESULT_OFFSET: u32 = 29;
    pub const RDN_RESULT_MASK: u32 = 0x20000000;
    pub const RDN_RESULT: u32 = 1 << 29;

    /// **DATA_LAST_O**
    ///
    /// Reserved
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const DATA_LAST_O_OFFSET: u32 = 28;
    pub const DATA_LAST_O_MASK: u32 = 0x10000000;
    pub const DATA_LAST_O: u32 = 1 << 28;

    /// **NS3**
    ///
    /// Reserved
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const NS3_OFFSET: u32 = 25;
    pub const NS3_MASK: u32 = 0x0e000000;

    /// **NS2**
    ///
    /// Reserved
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const NS2_OFFSET: u32 = 22;
    pub const NS2_MASK: u32 = 0x01c00000;

    /// **NS1**
    ///
    /// Reserved
    ///
    /// Bits `[21:19]`, mask `0x00380000`
    pub const NS1_OFFSET: u32 = 19;
    pub const NS1_MASK: u32 = 0x00380000;

    /// **NS0**
    ///
    /// Reserved
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const NS0_OFFSET: u32 = 16;
    pub const NS0_MASK: u32 = 0x00070000;

    /// **IDCT_HFM_DATA**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const IDCT_HFM_DATA_OFFSET: u32 = 0;
    pub const IDCT_HFM_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **RDN_ENA** `[30]` (RW)
    ///
    /// redundant control registers for jpeg
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

    /// **RDN_RESULT** `[29]` (RO)
    ///
    /// redundant registers for jpeg
    #[inline(always)]
    pub fn rdn_result(&self) -> bool {
        (self.value.read_bits() & Self::RDN_RESULT_MASK) != 0
    }

    /// **DATA_LAST_O** `[28]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn data_last_o(&self) -> bool {
        (self.value.read_bits() & Self::DATA_LAST_O_MASK) != 0
    }

    /// **NS3** `[27:25]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn ns3(&self) -> u32 {
        (self.value.read_bits() & Self::NS3_MASK) >> Self::NS3_OFFSET
    }

    /// **NS2** `[24:22]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn ns2(&self) -> u32 {
        (self.value.read_bits() & Self::NS2_MASK) >> Self::NS2_OFFSET
    }

    /// **NS1** `[21:19]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn ns1(&self) -> u32 {
        (self.value.read_bits() & Self::NS1_MASK) >> Self::NS1_OFFSET
    }

    /// **NS0** `[18:16]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn ns0(&self) -> u32 {
        (self.value.read_bits() & Self::NS0_MASK) >> Self::NS0_OFFSET
    }

    /// **IDCT_HFM_DATA** `[15:0]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn idct_hfm_data(&self) -> u32 {
        (self.value.read_bits() & Self::IDCT_HFM_DATA_MASK) >> Self::IDCT_HFM_DATA_OFFSET
    }
}

/// DECODER_STATUS5 value type for batch read/modify/write operations
///
/// Use `decoderstatus5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `decoderstatus5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DecoderStatus5Val {
    bits: u32,
}

impl DecoderStatus5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RDN_ENA** `[30:30]`
    ///
    /// redundant control registers for jpeg
    #[inline(always)]
    pub const fn rdn_ena(&self) -> bool {
        (self.bits & DecoderStatus5::RDN_ENA_MASK) != 0
    }

    /// Read **RDN_RESULT** `[29:29]`
    ///
    /// redundant registers for jpeg
    #[inline(always)]
    pub const fn rdn_result(&self) -> bool {
        (self.bits & DecoderStatus5::RDN_RESULT_MASK) != 0
    }

    /// Read **DATA_LAST_O** `[28:28]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn data_last_o(&self) -> bool {
        (self.bits & DecoderStatus5::DATA_LAST_O_MASK) != 0
    }

    /// Read **NS3** `[27:25]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn ns3(&self) -> u32 {
        (self.bits & DecoderStatus5::NS3_MASK) >> DecoderStatus5::NS3_OFFSET
    }

    /// Read **NS2** `[24:22]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn ns2(&self) -> u32 {
        (self.bits & DecoderStatus5::NS2_MASK) >> DecoderStatus5::NS2_OFFSET
    }

    /// Read **NS1** `[21:19]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn ns1(&self) -> u32 {
        (self.bits & DecoderStatus5::NS1_MASK) >> DecoderStatus5::NS1_OFFSET
    }

    /// Read **NS0** `[18:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn ns0(&self) -> u32 {
        (self.bits & DecoderStatus5::NS0_MASK) >> DecoderStatus5::NS0_OFFSET
    }

    /// Read **IDCT_HFM_DATA** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn idct_hfm_data(&self) -> u32 {
        (self.bits & DecoderStatus5::IDCT_HFM_DATA_MASK) >> DecoderStatus5::IDCT_HFM_DATA_OFFSET
    }

    /// Set **RDN_ENA** `[30:30]`
    ///
    /// redundant control registers for jpeg
    #[inline(always)]
    pub const fn with_rdn_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DecoderStatus5::RDN_ENA_MASK
            } else {
                self.bits & !DecoderStatus5::RDN_ENA_MASK
            },
        }
    }
}

impl Default for DecoderStatus5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DecoderStatus5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DecoderStatus5Val {
        DecoderStatus5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DecoderStatus5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DecoderStatus5Val) -> DecoderStatus5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// STATUS5 Register
///
/// Trace and Debug registers
///
/// Address offset: 0x00a0
///
/// ## Fields
///
/// - **PIC_BLOCK_NUM** `[23:0]` (RO)
///
///   Reserved
#[repr(transparent)]
pub struct Status5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status5 {
    // ======== Field Constants ========

    /// **PIC_BLOCK_NUM**
    ///
    /// Reserved
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const PIC_BLOCK_NUM_OFFSET: u32 = 0;
    pub const PIC_BLOCK_NUM_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **PIC_BLOCK_NUM** `[23:0]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn pic_block_num(&self) -> u32 {
        (self.value.read_bits() & Self::PIC_BLOCK_NUM_MASK) >> Self::PIC_BLOCK_NUM_OFFSET
    }
}

/// STATUS5 value type for batch read/modify/write operations
///
/// Use `status5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status5Val {
    bits: u32,
}

impl Status5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PIC_BLOCK_NUM** `[23:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn pic_block_num(&self) -> u32 {
        (self.bits & Status5::PIC_BLOCK_NUM_MASK) >> Status5::PIC_BLOCK_NUM_OFFSET
    }
}

impl Default for Status5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status5Val {
        Status5Val::from_bits(self.value.read_bits())
    }
}

/// ECO_LOW Register
///
/// Trace and Debug registers
///
/// Address offset: 0x00a4
///
/// ## Fields
///
/// - **RDN_ECO_LOW** `[31:0]` (RW)
///
///   redundant registers for jpeg
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

    /// **RDN_ECO_LOW**
    ///
    /// redundant registers for jpeg
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RDN_ECO_LOW_OFFSET: u32 = 0;
    pub const RDN_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// redundant registers for jpeg
    #[inline(always)]
    pub fn rdn_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::RDN_ECO_LOW_MASK) >> Self::RDN_ECO_LOW_OFFSET
    }

    /// Write **RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// redundant registers for jpeg
    #[inline(always)]
    pub fn write_rdn_eco_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RDN_ECO_LOW_MASK)
                | ((value << Self::RDN_ECO_LOW_OFFSET) & Self::RDN_ECO_LOW_MASK)
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

    /// Read **RDN_ECO_LOW** `[31:0]`
    ///
    /// redundant registers for jpeg
    #[inline(always)]
    pub const fn rdn_eco_low(&self) -> u32 {
        (self.bits & EcoLow::RDN_ECO_LOW_MASK) >> EcoLow::RDN_ECO_LOW_OFFSET
    }

    /// Set **RDN_ECO_LOW** `[31:0]`
    ///
    /// redundant registers for jpeg
    #[inline(always)]
    pub const fn with_rdn_eco_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EcoLow::RDN_ECO_LOW_MASK)
                | ((value << EcoLow::RDN_ECO_LOW_OFFSET) & EcoLow::RDN_ECO_LOW_MASK),
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
/// Trace and Debug registers
///
/// Address offset: 0x00a8
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **RDN_ECO_HIGH** `[31:0]` (RW)
///
///   redundant registers for jpeg
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

    /// **RDN_ECO_HIGH**
    ///
    /// redundant registers for jpeg
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RDN_ECO_HIGH_OFFSET: u32 = 0;
    pub const RDN_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// redundant registers for jpeg
    #[inline(always)]
    pub fn rdn_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::RDN_ECO_HIGH_MASK) >> Self::RDN_ECO_HIGH_OFFSET
    }

    /// Write **RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// redundant registers for jpeg
    #[inline(always)]
    pub fn write_rdn_eco_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RDN_ECO_HIGH_MASK)
                | ((value << Self::RDN_ECO_HIGH_OFFSET) & Self::RDN_ECO_HIGH_MASK)
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

    /// Read **RDN_ECO_HIGH** `[31:0]`
    ///
    /// redundant registers for jpeg
    #[inline(always)]
    pub const fn rdn_eco_high(&self) -> u32 {
        (self.bits & EcoHigh::RDN_ECO_HIGH_MASK) >> EcoHigh::RDN_ECO_HIGH_OFFSET
    }

    /// Set **RDN_ECO_HIGH** `[31:0]`
    ///
    /// redundant registers for jpeg
    #[inline(always)]
    pub const fn with_rdn_eco_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EcoHigh::RDN_ECO_HIGH_MASK)
                | ((value << EcoHigh::RDN_ECO_HIGH_OFFSET) & EcoHigh::RDN_ECO_HIGH_MASK),
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

/// SYS Register
///
/// Trace and Debug registers
///
/// Address offset: 0x00f8
///
/// ## Fields
///
/// - **CLK_EN** `[31]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Sys {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sys {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sys {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// Reserved
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_EN_OFFSET: u32 = 31;
    pub const CLK_EN_MASK: u32 = 0x80000000;
    pub const CLK_EN: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **CLK_EN** `[31]` (RW)
    ///
    /// Reserved
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

/// SYS value type for batch read/modify/write operations
///
/// Use `sys.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sys.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SysVal {
    bits: u32,
}

impl SysVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CLK_EN** `[31:31]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & Sys::CLK_EN_MASK) != 0
    }

    /// Set **CLK_EN** `[31:31]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sys::CLK_EN_MASK
            } else {
                self.bits & !Sys::CLK_EN_MASK
            },
        }
    }
}

impl Default for SysVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sys {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SysVal {
        SysVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SysVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SysVal) -> SysVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VERSION Register
///
/// Trace and Debug registers
///
/// Address offset: 0x00fc
/// Reset value: 0x02111190
///
/// ## Fields
///
/// - **JPEG_VER** `[27:0]` (RW)
///
///   Reserved
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

    /// **JPEG_VER**
    ///
    /// Reserved
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const JPEG_VER_OFFSET: u32 = 0;
    pub const JPEG_VER_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **JPEG_VER** `[27:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn jpeg_ver(&self) -> u32 {
        (self.value.read_bits() & Self::JPEG_VER_MASK) >> Self::JPEG_VER_OFFSET
    }

    /// Write **JPEG_VER** `[27:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_jpeg_ver(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::JPEG_VER_MASK) | ((value << Self::JPEG_VER_OFFSET) & Self::JPEG_VER_MASK)
        });
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
        Self { bits: 0x02111190 }
    }

    /// Read **JPEG_VER** `[27:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn jpeg_ver(&self) -> u32 {
        (self.bits & Version::JPEG_VER_MASK) >> Version::JPEG_VER_OFFSET
    }

    /// Set **JPEG_VER** `[27:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_jpeg_ver(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Version::JPEG_VER_MASK)
                | ((value << Version::JPEG_VER_OFFSET) & Version::JPEG_VER_MASK),
        }
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

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VersionVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VersionVal) -> VersionVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Control and configuration registers
    pub config: Config,
    /// 0x0004
    /// Control and configuration registers
    pub dqt_info: DqtInfo,
    /// 0x0008
    /// Control and configuration registers
    pub pic_size: PicSize,
    _reserved0: [u8; 4],
    /// 0x0010
    /// Control and configuration registers
    pub t0qnr: T0qnr,
    /// 0x0014
    /// Control and configuration registers
    pub t1qnr: T1qnr,
    /// 0x0018
    /// Control and configuration registers
    pub t2qnr: T2qnr,
    /// 0x001c
    /// Control and configuration registers
    pub t3qnr: T3qnr,
    /// 0x0020
    /// Control and configuration registers
    pub decode_conf: DecodeConf,
    /// 0x0024
    /// Control and configuration registers
    pub c0: C0,
    /// 0x0028
    /// Control and configuration registers
    pub c1: C1,
    /// 0x002c
    /// Control and configuration registers
    pub c2: C2,
    /// 0x0030
    /// Control and configuration registers
    pub c3: C3,
    /// 0x0034
    /// Control and configuration registers
    pub dht_info: DhtInfo,
    /// 0x0038
    /// Interrupt raw registers
    pub int_raw: IntRaw,
    /// 0x003c
    /// Interrupt enable registers
    pub int_ena: IntEna,
    /// 0x0040
    /// Interrupt status registers
    pub int_st: IntSt,
    /// 0x0044
    /// Interrupt clear registers
    pub int_clr: IntClr,
    /// 0x0048
    /// Trace and Debug registers
    pub status0: Status0,
    /// 0x004c
    /// Trace and Debug registers
    pub status2: Status2,
    /// 0x0050
    /// Trace and Debug registers
    pub status3: Status3,
    /// 0x0054
    /// Trace and Debug registers
    pub status4: Status4,
    /// 0x0058
    /// Trace and Debug registers
    pub dht_totlen_dc0: DhtTotlenDc0,
    /// 0x005c
    /// Trace and Debug registers
    pub dht_v_al_dc0: DhtVAlDc0,
    /// 0x0060
    /// Trace and Debug registers
    pub dht_totlen_ac0: DhtTotlenAc0,
    /// 0x0064
    /// Trace and Debug registers
    pub dht_v_al_ac0: DhtVAlAc0,
    /// 0x0068
    /// Trace and Debug registers
    pub dht_totlen_dc1: DhtTotlenDc1,
    /// 0x006c
    /// Trace and Debug registers
    pub dht_v_al_dc1: DhtVAlDc1,
    /// 0x0070
    /// Trace and Debug registers
    pub dht_totlen_ac1: DhtTotlenAc1,
    /// 0x0074
    /// Trace and Debug registers
    pub dht_v_al_ac1: DhtVAlAc1,
    /// 0x0078
    /// Trace and Debug registers
    pub dht_codemin_dc0: DhtCodeminDc0,
    /// 0x007c
    /// Trace and Debug registers
    pub dht_codemin_ac0: DhtCodeminAc0,
    /// 0x0080
    /// Trace and Debug registers
    pub dht_codemin_dc1: DhtCodeminDc1,
    /// 0x0084
    /// Trace and Debug registers
    pub dht_codemin_ac1: DhtCodeminAc1,
    /// 0x0088
    /// Trace and Debug registers
    pub decoder_status0: DecoderStatus0,
    /// 0x008c
    /// Trace and Debug registers
    pub decoder_status1: DecoderStatus1,
    /// 0x0090
    /// Trace and Debug registers
    pub decoder_status2: DecoderStatus2,
    /// 0x0094
    /// Trace and Debug registers
    pub decoder_status3: DecoderStatus3,
    /// 0x0098
    /// Trace and Debug registers
    pub decoder_status4: DecoderStatus4,
    /// 0x009c
    /// Trace and Debug registers
    pub decoder_status5: DecoderStatus5,
    /// 0x00a0
    /// Trace and Debug registers
    pub status5: Status5,
    /// 0x00a4
    /// Trace and Debug registers
    pub eco_low: EcoLow,
    /// 0x00a8
    /// Trace and Debug registers
    pub eco_high: EcoHigh,
    _reserved1: [u8; 76],
    /// 0x00f8
    /// Trace and Debug registers
    pub sys: Sys,
    /// 0x00fc
    /// Trace and Debug registers
    pub version: Version,
}
