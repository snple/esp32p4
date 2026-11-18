//! I3C_MST
//! I3C Controller (Master)

#![allow(dead_code)]

pub const BASE: u32 = 0x500da000;

/// DEVICE_CTRL Register
///
/// DEVICE_CTRL register controls the transfer properties and disposition of controllers capabilities.
///
/// Address offset: 0x0000
/// Reset value: 0x00001020
///
/// ## Fields
///
/// - **REG_MEM_CLK_FORCE_ON** `[17]` (RW)
///
///   1: dev characteristic and address table memory clk date force on . 0 : clock gating by rd/wr.
///
/// - **REG_DATA_BYTE_CNT_UNLATCH** `[16]` (RW)
///
///   1: read current real-time updated value 0: read latch data byte cnt value
///
/// - **REG_SDA_PP_WR_PULLUP_EN** `[15]` (RW)
///
///   NA
///
/// - **REG_SDA_RD_TBIT_HLVL_PULLUP_EN** `[14]` (RW)
///
///   NA
///
/// - **REG_SDA_PP_RD_PULLUP_EN** `[13]` (RW)
///
///   NA
///
/// - **REG_SCL_OE_FORCE_EN** `[12]` (RW)
///
///   This bit is used to force scl_oe
///
/// - **REG_SCL_PULLUP_FORCE_EN** `[11]` (RW)
///
///   This bit is used to force scl_pullup_en
///
/// - **REG_RX_BYTE_ORDER** `[10]` (RW)
///
///   NA
///
/// - **REG_RX_BIT_ORDER** `[9]` (RW)
///
///   0: rx low byte fist, 1: rx high byte first
///
/// - **REG_MULTI_SLV_SINGLE_CCC_EN** `[8]` (RW)
///
///   0: rx high bit first, 1: rx low bit first
///
/// - **REG_DMA_TX_EN** `[7]` (RW)
///
///   NA
///
/// - **REG_DMA_RX_EN** `[6]` (RW)
///
///   NA
///
/// - **REG_AUTO_DIS_IBI_EN** `[5]` (RW)
///
///   NA
///
/// - **REG_IBI_RSTART_TRANS_EN** `[4]` (RW)
///
///   NA
///
/// - **REG_CLK_EN** `[3]` (RW)
///
///   NA
///
/// - **REG_TRANS_START** `[2]` (RW)
///
///   Transfer Start
///
/// - **REG_BA_INCLUDE** `[1]` (RW)
///
///   This bit is used to include I3C broadcast address(0x7E) for private transfer.(If I3C broadcast address is not include for the private transfer, In-Band Interrupts driven from Slaves may not win address arbitration. Hence IBIs will get delayed)
#[repr(transparent)]
pub struct DeviceCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DeviceCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DeviceCtrl {
    // ======== Field Constants ========

    /// **REG_MEM_CLK_FORCE_ON**
    ///
    /// 1: dev characteristic and address table memory clk date force on . 0 : clock gating by rd/wr.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const REG_MEM_CLK_FORCE_ON_OFFSET: u32 = 17;
    pub const REG_MEM_CLK_FORCE_ON_MASK: u32 = 0x00020000;
    pub const REG_MEM_CLK_FORCE_ON: u32 = 1 << 17;

    /// **REG_DATA_BYTE_CNT_UNLATCH**
    ///
    /// 1: read current real-time updated value 0: read latch data byte cnt value
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const REG_DATA_BYTE_CNT_UNLATCH_OFFSET: u32 = 16;
    pub const REG_DATA_BYTE_CNT_UNLATCH_MASK: u32 = 0x00010000;
    pub const REG_DATA_BYTE_CNT_UNLATCH: u32 = 1 << 16;

    /// **REG_SDA_PP_WR_PULLUP_EN**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const REG_SDA_PP_WR_PULLUP_EN_OFFSET: u32 = 15;
    pub const REG_SDA_PP_WR_PULLUP_EN_MASK: u32 = 0x00008000;
    pub const REG_SDA_PP_WR_PULLUP_EN: u32 = 1 << 15;

    /// **REG_SDA_RD_TBIT_HLVL_PULLUP_EN**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const REG_SDA_RD_TBIT_HLVL_PULLUP_EN_OFFSET: u32 = 14;
    pub const REG_SDA_RD_TBIT_HLVL_PULLUP_EN_MASK: u32 = 0x00004000;
    pub const REG_SDA_RD_TBIT_HLVL_PULLUP_EN: u32 = 1 << 14;

    /// **REG_SDA_PP_RD_PULLUP_EN**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const REG_SDA_PP_RD_PULLUP_EN_OFFSET: u32 = 13;
    pub const REG_SDA_PP_RD_PULLUP_EN_MASK: u32 = 0x00002000;
    pub const REG_SDA_PP_RD_PULLUP_EN: u32 = 1 << 13;

    /// **REG_SCL_OE_FORCE_EN**
    ///
    /// This bit is used to force scl_oe
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const REG_SCL_OE_FORCE_EN_OFFSET: u32 = 12;
    pub const REG_SCL_OE_FORCE_EN_MASK: u32 = 0x00001000;
    pub const REG_SCL_OE_FORCE_EN: u32 = 1 << 12;

    /// **REG_SCL_PULLUP_FORCE_EN**
    ///
    /// This bit is used to force scl_pullup_en
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_SCL_PULLUP_FORCE_EN_OFFSET: u32 = 11;
    pub const REG_SCL_PULLUP_FORCE_EN_MASK: u32 = 0x00000800;
    pub const REG_SCL_PULLUP_FORCE_EN: u32 = 1 << 11;

    /// **REG_RX_BYTE_ORDER**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_RX_BYTE_ORDER_OFFSET: u32 = 10;
    pub const REG_RX_BYTE_ORDER_MASK: u32 = 0x00000400;
    pub const REG_RX_BYTE_ORDER: u32 = 1 << 10;

    /// **REG_RX_BIT_ORDER**
    ///
    /// 0: rx low byte fist, 1: rx high byte first
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_RX_BIT_ORDER_OFFSET: u32 = 9;
    pub const REG_RX_BIT_ORDER_MASK: u32 = 0x00000200;
    pub const REG_RX_BIT_ORDER: u32 = 1 << 9;

    /// **REG_MULTI_SLV_SINGLE_CCC_EN**
    ///
    /// 0: rx high bit first, 1: rx low bit first
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_MULTI_SLV_SINGLE_CCC_EN_OFFSET: u32 = 8;
    pub const REG_MULTI_SLV_SINGLE_CCC_EN_MASK: u32 = 0x00000100;
    pub const REG_MULTI_SLV_SINGLE_CCC_EN: u32 = 1 << 8;

    /// **REG_DMA_TX_EN**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_DMA_TX_EN_OFFSET: u32 = 7;
    pub const REG_DMA_TX_EN_MASK: u32 = 0x00000080;
    pub const REG_DMA_TX_EN: u32 = 1 << 7;

    /// **REG_DMA_RX_EN**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const REG_DMA_RX_EN_OFFSET: u32 = 6;
    pub const REG_DMA_RX_EN_MASK: u32 = 0x00000040;
    pub const REG_DMA_RX_EN: u32 = 1 << 6;

    /// **REG_AUTO_DIS_IBI_EN**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_AUTO_DIS_IBI_EN_OFFSET: u32 = 5;
    pub const REG_AUTO_DIS_IBI_EN_MASK: u32 = 0x00000020;
    pub const REG_AUTO_DIS_IBI_EN: u32 = 1 << 5;

    /// **REG_IBI_RSTART_TRANS_EN**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_IBI_RSTART_TRANS_EN_OFFSET: u32 = 4;
    pub const REG_IBI_RSTART_TRANS_EN_MASK: u32 = 0x00000010;
    pub const REG_IBI_RSTART_TRANS_EN: u32 = 1 << 4;

    /// **REG_CLK_EN**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_CLK_EN_OFFSET: u32 = 3;
    pub const REG_CLK_EN_MASK: u32 = 0x00000008;
    pub const REG_CLK_EN: u32 = 1 << 3;

    /// **REG_TRANS_START**
    ///
    /// Transfer Start
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_TRANS_START_OFFSET: u32 = 2;
    pub const REG_TRANS_START_MASK: u32 = 0x00000004;
    pub const REG_TRANS_START: u32 = 1 << 2;

    /// **REG_BA_INCLUDE**
    ///
    /// This bit is used to include I3C broadcast address(0x7E) for private transfer.(If I3C broadcast address is not include for the private transfer, In-Band Interrupts driven from Slaves may not win address arbitration. Hence IBIs will get delayed)
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_BA_INCLUDE_OFFSET: u32 = 1;
    pub const REG_BA_INCLUDE_MASK: u32 = 0x00000002;
    pub const REG_BA_INCLUDE: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **REG_MEM_CLK_FORCE_ON** `[17]` (RW)
    ///
    /// 1: dev characteristic and address table memory clk date force on . 0 : clock gating by rd/wr.
    #[inline(always)]
    pub fn reg_mem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::REG_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set REG_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_MEM_CLK_FORCE_ON_MASK);
    }

    /// Clear REG_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_MEM_CLK_FORCE_ON_MASK);
    }

    /// **REG_DATA_BYTE_CNT_UNLATCH** `[16]` (RW)
    ///
    /// 1: read current real-time updated value 0: read latch data byte cnt value
    #[inline(always)]
    pub fn reg_data_byte_cnt_unlatch(&self) -> bool {
        (self.value.read_bits() & Self::REG_DATA_BYTE_CNT_UNLATCH_MASK) != 0
    }

    /// Set REG_DATA_BYTE_CNT_UNLATCH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_data_byte_cnt_unlatch(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_DATA_BYTE_CNT_UNLATCH_MASK);
    }

    /// Clear REG_DATA_BYTE_CNT_UNLATCH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_data_byte_cnt_unlatch(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_DATA_BYTE_CNT_UNLATCH_MASK);
    }

    /// **REG_SDA_PP_WR_PULLUP_EN** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_sda_pp_wr_pullup_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_SDA_PP_WR_PULLUP_EN_MASK) != 0
    }

    /// Set REG_SDA_PP_WR_PULLUP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_sda_pp_wr_pullup_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_SDA_PP_WR_PULLUP_EN_MASK);
    }

    /// Clear REG_SDA_PP_WR_PULLUP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_sda_pp_wr_pullup_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_SDA_PP_WR_PULLUP_EN_MASK);
    }

    /// **REG_SDA_RD_TBIT_HLVL_PULLUP_EN** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_sda_rd_tbit_hlvl_pullup_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_SDA_RD_TBIT_HLVL_PULLUP_EN_MASK) != 0
    }

    /// Set REG_SDA_RD_TBIT_HLVL_PULLUP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_sda_rd_tbit_hlvl_pullup_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_SDA_RD_TBIT_HLVL_PULLUP_EN_MASK);
    }

    /// Clear REG_SDA_RD_TBIT_HLVL_PULLUP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_sda_rd_tbit_hlvl_pullup_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_SDA_RD_TBIT_HLVL_PULLUP_EN_MASK);
    }

    /// **REG_SDA_PP_RD_PULLUP_EN** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_sda_pp_rd_pullup_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_SDA_PP_RD_PULLUP_EN_MASK) != 0
    }

    /// Set REG_SDA_PP_RD_PULLUP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_sda_pp_rd_pullup_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_SDA_PP_RD_PULLUP_EN_MASK);
    }

    /// Clear REG_SDA_PP_RD_PULLUP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_sda_pp_rd_pullup_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_SDA_PP_RD_PULLUP_EN_MASK);
    }

    /// **REG_SCL_OE_FORCE_EN** `[12]` (RW)
    ///
    /// This bit is used to force scl_oe
    #[inline(always)]
    pub fn reg_scl_oe_force_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_SCL_OE_FORCE_EN_MASK) != 0
    }

    /// Set REG_SCL_OE_FORCE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_scl_oe_force_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_SCL_OE_FORCE_EN_MASK);
    }

    /// Clear REG_SCL_OE_FORCE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_scl_oe_force_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_SCL_OE_FORCE_EN_MASK);
    }

    /// **REG_SCL_PULLUP_FORCE_EN** `[11]` (RW)
    ///
    /// This bit is used to force scl_pullup_en
    #[inline(always)]
    pub fn reg_scl_pullup_force_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_SCL_PULLUP_FORCE_EN_MASK) != 0
    }

    /// Set REG_SCL_PULLUP_FORCE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_scl_pullup_force_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_SCL_PULLUP_FORCE_EN_MASK);
    }

    /// Clear REG_SCL_PULLUP_FORCE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_scl_pullup_force_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_SCL_PULLUP_FORCE_EN_MASK);
    }

    /// **REG_RX_BYTE_ORDER** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_rx_byte_order(&self) -> bool {
        (self.value.read_bits() & Self::REG_RX_BYTE_ORDER_MASK) != 0
    }

    /// Set REG_RX_BYTE_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_rx_byte_order(&self) {
        self.value.modify_bits(|r| r | Self::REG_RX_BYTE_ORDER_MASK);
    }

    /// Clear REG_RX_BYTE_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_rx_byte_order(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_RX_BYTE_ORDER_MASK);
    }

    /// **REG_RX_BIT_ORDER** `[9]` (RW)
    ///
    /// 0: rx low byte fist, 1: rx high byte first
    #[inline(always)]
    pub fn reg_rx_bit_order(&self) -> bool {
        (self.value.read_bits() & Self::REG_RX_BIT_ORDER_MASK) != 0
    }

    /// Set REG_RX_BIT_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_rx_bit_order(&self) {
        self.value.modify_bits(|r| r | Self::REG_RX_BIT_ORDER_MASK);
    }

    /// Clear REG_RX_BIT_ORDER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_rx_bit_order(&self) {
        self.value.modify_bits(|r| r & !Self::REG_RX_BIT_ORDER_MASK);
    }

    /// **REG_MULTI_SLV_SINGLE_CCC_EN** `[8]` (RW)
    ///
    /// 0: rx high bit first, 1: rx low bit first
    #[inline(always)]
    pub fn reg_multi_slv_single_ccc_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_MULTI_SLV_SINGLE_CCC_EN_MASK) != 0
    }

    /// Set REG_MULTI_SLV_SINGLE_CCC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_multi_slv_single_ccc_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_MULTI_SLV_SINGLE_CCC_EN_MASK);
    }

    /// Clear REG_MULTI_SLV_SINGLE_CCC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_multi_slv_single_ccc_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_MULTI_SLV_SINGLE_CCC_EN_MASK);
    }

    /// **REG_DMA_TX_EN** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dma_tx_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_DMA_TX_EN_MASK) != 0
    }

    /// Set REG_DMA_TX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_dma_tx_en(&self) {
        self.value.modify_bits(|r| r | Self::REG_DMA_TX_EN_MASK);
    }

    /// Clear REG_DMA_TX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_dma_tx_en(&self) {
        self.value.modify_bits(|r| r & !Self::REG_DMA_TX_EN_MASK);
    }

    /// **REG_DMA_RX_EN** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dma_rx_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_DMA_RX_EN_MASK) != 0
    }

    /// Set REG_DMA_RX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_dma_rx_en(&self) {
        self.value.modify_bits(|r| r | Self::REG_DMA_RX_EN_MASK);
    }

    /// Clear REG_DMA_RX_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_dma_rx_en(&self) {
        self.value.modify_bits(|r| r & !Self::REG_DMA_RX_EN_MASK);
    }

    /// **REG_AUTO_DIS_IBI_EN** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_auto_dis_ibi_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_AUTO_DIS_IBI_EN_MASK) != 0
    }

    /// Set REG_AUTO_DIS_IBI_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_auto_dis_ibi_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_AUTO_DIS_IBI_EN_MASK);
    }

    /// Clear REG_AUTO_DIS_IBI_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_auto_dis_ibi_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_AUTO_DIS_IBI_EN_MASK);
    }

    /// **REG_IBI_RSTART_TRANS_EN** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_ibi_rstart_trans_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_IBI_RSTART_TRANS_EN_MASK) != 0
    }

    /// Set REG_IBI_RSTART_TRANS_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_ibi_rstart_trans_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_IBI_RSTART_TRANS_EN_MASK);
    }

    /// Clear REG_IBI_RSTART_TRANS_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_ibi_rstart_trans_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_IBI_RSTART_TRANS_EN_MASK);
    }

    /// **REG_CLK_EN** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_CLK_EN_MASK) != 0
    }

    /// Set REG_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::REG_CLK_EN_MASK);
    }

    /// Clear REG_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::REG_CLK_EN_MASK);
    }

    /// **REG_TRANS_START** `[2]` (RW)
    ///
    /// Transfer Start
    #[inline(always)]
    pub fn reg_trans_start(&self) -> bool {
        (self.value.read_bits() & Self::REG_TRANS_START_MASK) != 0
    }

    /// Set REG_TRANS_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_trans_start(&self) {
        self.value.modify_bits(|r| r | Self::REG_TRANS_START_MASK);
    }

    /// Clear REG_TRANS_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_trans_start(&self) {
        self.value.modify_bits(|r| r & !Self::REG_TRANS_START_MASK);
    }

    /// **REG_BA_INCLUDE** `[1]` (RW)
    ///
    /// This bit is used to include I3C broadcast address(0x7E) for private transfer.(If I3C broadcast address is not include for the private transfer, In-Band Interrupts driven from Slaves may not win address arbitration. Hence IBIs will get delayed)
    #[inline(always)]
    pub fn reg_ba_include(&self) -> bool {
        (self.value.read_bits() & Self::REG_BA_INCLUDE_MASK) != 0
    }

    /// Set REG_BA_INCLUDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_ba_include(&self) {
        self.value.modify_bits(|r| r | Self::REG_BA_INCLUDE_MASK);
    }

    /// Clear REG_BA_INCLUDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_ba_include(&self) {
        self.value.modify_bits(|r| r & !Self::REG_BA_INCLUDE_MASK);
    }
}

/// DEVICE_CTRL value type for batch read/modify/write operations
///
/// Use `devicectrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devicectrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DeviceCtrlVal {
    bits: u32,
}

impl DeviceCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00001020 }
    }

    /// Read **REG_MEM_CLK_FORCE_ON** `[17:17]`
    ///
    /// 1: dev characteristic and address table memory clk date force on . 0 : clock gating by rd/wr.
    #[inline(always)]
    pub const fn reg_mem_clk_force_on(&self) -> bool {
        (self.bits & DeviceCtrl::REG_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **REG_DATA_BYTE_CNT_UNLATCH** `[16:16]`
    ///
    /// 1: read current real-time updated value 0: read latch data byte cnt value
    #[inline(always)]
    pub const fn reg_data_byte_cnt_unlatch(&self) -> bool {
        (self.bits & DeviceCtrl::REG_DATA_BYTE_CNT_UNLATCH_MASK) != 0
    }

    /// Read **REG_SDA_PP_WR_PULLUP_EN** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_sda_pp_wr_pullup_en(&self) -> bool {
        (self.bits & DeviceCtrl::REG_SDA_PP_WR_PULLUP_EN_MASK) != 0
    }

    /// Read **REG_SDA_RD_TBIT_HLVL_PULLUP_EN** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_sda_rd_tbit_hlvl_pullup_en(&self) -> bool {
        (self.bits & DeviceCtrl::REG_SDA_RD_TBIT_HLVL_PULLUP_EN_MASK) != 0
    }

    /// Read **REG_SDA_PP_RD_PULLUP_EN** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_sda_pp_rd_pullup_en(&self) -> bool {
        (self.bits & DeviceCtrl::REG_SDA_PP_RD_PULLUP_EN_MASK) != 0
    }

    /// Read **REG_SCL_OE_FORCE_EN** `[12:12]`
    ///
    /// This bit is used to force scl_oe
    #[inline(always)]
    pub const fn reg_scl_oe_force_en(&self) -> bool {
        (self.bits & DeviceCtrl::REG_SCL_OE_FORCE_EN_MASK) != 0
    }

    /// Read **REG_SCL_PULLUP_FORCE_EN** `[11:11]`
    ///
    /// This bit is used to force scl_pullup_en
    #[inline(always)]
    pub const fn reg_scl_pullup_force_en(&self) -> bool {
        (self.bits & DeviceCtrl::REG_SCL_PULLUP_FORCE_EN_MASK) != 0
    }

    /// Read **REG_RX_BYTE_ORDER** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_rx_byte_order(&self) -> bool {
        (self.bits & DeviceCtrl::REG_RX_BYTE_ORDER_MASK) != 0
    }

    /// Read **REG_RX_BIT_ORDER** `[9:9]`
    ///
    /// 0: rx low byte fist, 1: rx high byte first
    #[inline(always)]
    pub const fn reg_rx_bit_order(&self) -> bool {
        (self.bits & DeviceCtrl::REG_RX_BIT_ORDER_MASK) != 0
    }

    /// Read **REG_MULTI_SLV_SINGLE_CCC_EN** `[8:8]`
    ///
    /// 0: rx high bit first, 1: rx low bit first
    #[inline(always)]
    pub const fn reg_multi_slv_single_ccc_en(&self) -> bool {
        (self.bits & DeviceCtrl::REG_MULTI_SLV_SINGLE_CCC_EN_MASK) != 0
    }

    /// Read **REG_DMA_TX_EN** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dma_tx_en(&self) -> bool {
        (self.bits & DeviceCtrl::REG_DMA_TX_EN_MASK) != 0
    }

    /// Read **REG_DMA_RX_EN** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dma_rx_en(&self) -> bool {
        (self.bits & DeviceCtrl::REG_DMA_RX_EN_MASK) != 0
    }

    /// Read **REG_AUTO_DIS_IBI_EN** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_auto_dis_ibi_en(&self) -> bool {
        (self.bits & DeviceCtrl::REG_AUTO_DIS_IBI_EN_MASK) != 0
    }

    /// Read **REG_IBI_RSTART_TRANS_EN** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_ibi_rstart_trans_en(&self) -> bool {
        (self.bits & DeviceCtrl::REG_IBI_RSTART_TRANS_EN_MASK) != 0
    }

    /// Read **REG_CLK_EN** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_clk_en(&self) -> bool {
        (self.bits & DeviceCtrl::REG_CLK_EN_MASK) != 0
    }

    /// Read **REG_TRANS_START** `[2:2]`
    ///
    /// Transfer Start
    #[inline(always)]
    pub const fn reg_trans_start(&self) -> bool {
        (self.bits & DeviceCtrl::REG_TRANS_START_MASK) != 0
    }

    /// Read **REG_BA_INCLUDE** `[1:1]`
    ///
    /// This bit is used to include I3C broadcast address(0x7E) for private transfer.(If I3C broadcast address is not include for the private transfer, In-Band Interrupts driven from Slaves may not win address arbitration. Hence IBIs will get delayed)
    #[inline(always)]
    pub const fn reg_ba_include(&self) -> bool {
        (self.bits & DeviceCtrl::REG_BA_INCLUDE_MASK) != 0
    }

    /// Set **REG_MEM_CLK_FORCE_ON** `[17:17]`
    ///
    /// 1: dev characteristic and address table memory clk date force on . 0 : clock gating by rd/wr.
    #[inline(always)]
    pub const fn with_reg_mem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_MEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !DeviceCtrl::REG_MEM_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **REG_DATA_BYTE_CNT_UNLATCH** `[16:16]`
    ///
    /// 1: read current real-time updated value 0: read latch data byte cnt value
    #[inline(always)]
    pub const fn with_reg_data_byte_cnt_unlatch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_DATA_BYTE_CNT_UNLATCH_MASK
            } else {
                self.bits & !DeviceCtrl::REG_DATA_BYTE_CNT_UNLATCH_MASK
            },
        }
    }

    /// Set **REG_SDA_PP_WR_PULLUP_EN** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_sda_pp_wr_pullup_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_SDA_PP_WR_PULLUP_EN_MASK
            } else {
                self.bits & !DeviceCtrl::REG_SDA_PP_WR_PULLUP_EN_MASK
            },
        }
    }

    /// Set **REG_SDA_RD_TBIT_HLVL_PULLUP_EN** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_sda_rd_tbit_hlvl_pullup_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_SDA_RD_TBIT_HLVL_PULLUP_EN_MASK
            } else {
                self.bits & !DeviceCtrl::REG_SDA_RD_TBIT_HLVL_PULLUP_EN_MASK
            },
        }
    }

    /// Set **REG_SDA_PP_RD_PULLUP_EN** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_sda_pp_rd_pullup_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_SDA_PP_RD_PULLUP_EN_MASK
            } else {
                self.bits & !DeviceCtrl::REG_SDA_PP_RD_PULLUP_EN_MASK
            },
        }
    }

    /// Set **REG_SCL_OE_FORCE_EN** `[12:12]`
    ///
    /// This bit is used to force scl_oe
    #[inline(always)]
    pub const fn with_reg_scl_oe_force_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_SCL_OE_FORCE_EN_MASK
            } else {
                self.bits & !DeviceCtrl::REG_SCL_OE_FORCE_EN_MASK
            },
        }
    }

    /// Set **REG_SCL_PULLUP_FORCE_EN** `[11:11]`
    ///
    /// This bit is used to force scl_pullup_en
    #[inline(always)]
    pub const fn with_reg_scl_pullup_force_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_SCL_PULLUP_FORCE_EN_MASK
            } else {
                self.bits & !DeviceCtrl::REG_SCL_PULLUP_FORCE_EN_MASK
            },
        }
    }

    /// Set **REG_RX_BYTE_ORDER** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_rx_byte_order(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_RX_BYTE_ORDER_MASK
            } else {
                self.bits & !DeviceCtrl::REG_RX_BYTE_ORDER_MASK
            },
        }
    }

    /// Set **REG_RX_BIT_ORDER** `[9:9]`
    ///
    /// 0: rx low byte fist, 1: rx high byte first
    #[inline(always)]
    pub const fn with_reg_rx_bit_order(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_RX_BIT_ORDER_MASK
            } else {
                self.bits & !DeviceCtrl::REG_RX_BIT_ORDER_MASK
            },
        }
    }

    /// Set **REG_MULTI_SLV_SINGLE_CCC_EN** `[8:8]`
    ///
    /// 0: rx high bit first, 1: rx low bit first
    #[inline(always)]
    pub const fn with_reg_multi_slv_single_ccc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_MULTI_SLV_SINGLE_CCC_EN_MASK
            } else {
                self.bits & !DeviceCtrl::REG_MULTI_SLV_SINGLE_CCC_EN_MASK
            },
        }
    }

    /// Set **REG_DMA_TX_EN** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dma_tx_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_DMA_TX_EN_MASK
            } else {
                self.bits & !DeviceCtrl::REG_DMA_TX_EN_MASK
            },
        }
    }

    /// Set **REG_DMA_RX_EN** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dma_rx_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_DMA_RX_EN_MASK
            } else {
                self.bits & !DeviceCtrl::REG_DMA_RX_EN_MASK
            },
        }
    }

    /// Set **REG_AUTO_DIS_IBI_EN** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_auto_dis_ibi_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_AUTO_DIS_IBI_EN_MASK
            } else {
                self.bits & !DeviceCtrl::REG_AUTO_DIS_IBI_EN_MASK
            },
        }
    }

    /// Set **REG_IBI_RSTART_TRANS_EN** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_ibi_rstart_trans_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_IBI_RSTART_TRANS_EN_MASK
            } else {
                self.bits & !DeviceCtrl::REG_IBI_RSTART_TRANS_EN_MASK
            },
        }
    }

    /// Set **REG_CLK_EN** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_CLK_EN_MASK
            } else {
                self.bits & !DeviceCtrl::REG_CLK_EN_MASK
            },
        }
    }

    /// Set **REG_TRANS_START** `[2:2]`
    ///
    /// Transfer Start
    #[inline(always)]
    pub const fn with_reg_trans_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_TRANS_START_MASK
            } else {
                self.bits & !DeviceCtrl::REG_TRANS_START_MASK
            },
        }
    }

    /// Set **REG_BA_INCLUDE** `[1:1]`
    ///
    /// This bit is used to include I3C broadcast address(0x7E) for private transfer.(If I3C broadcast address is not include for the private transfer, In-Band Interrupts driven from Slaves may not win address arbitration. Hence IBIs will get delayed)
    #[inline(always)]
    pub const fn with_reg_ba_include(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DeviceCtrl::REG_BA_INCLUDE_MASK
            } else {
                self.bits & !DeviceCtrl::REG_BA_INCLUDE_MASK
            },
        }
    }
}

impl Default for DeviceCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DeviceCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DeviceCtrlVal {
        DeviceCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DeviceCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DeviceCtrlVal) -> DeviceCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BUFFER_THLD_CTRL Register
///
/// In-Band Interrupt Status Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI status entries in the IBI buffer that trigger the IBI_STATUS_THLD_STAT interrupt.
///
/// Address offset: 0x001c
/// Reset value: 0x00041041
///
/// ## Fields
///
/// - **REG_IBI_STATUS_BUF_THLD** `[20:18]` (RW)
///
///   NA
///
/// - **REG_IBI_DATA_BUF_THLD** `[14:12]` (RW)
///
///   In-Band Interrupt Data Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI data entries in the IBI buffer that trigger the IBI_DATA_THLD_STAT interrupt.
///
/// - **REG_RESP_BUF_THLD** `[8:6]` (RW)
///
///   Response Buffer Threshold Value is used to control the number of entries in the Response Buffer that trigger the RESP_READY_STAT_INTR.
///
/// - **REG_CMD_BUF_EMPTY_THLD** `[3:0]` (RW)
///
///   Command Buffer Empty Threshold Value is used to control the number of empty locations(or greater) in the Command Buffer that trigger CMD_BUFFER_READY_STAT interrupt.
#[repr(transparent)]
pub struct BufferThldCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BufferThldCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BufferThldCtrl {
    // ======== Field Constants ========

    /// **REG_IBI_STATUS_BUF_THLD**
    ///
    /// NA
    ///
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const REG_IBI_STATUS_BUF_THLD_OFFSET: u32 = 18;
    pub const REG_IBI_STATUS_BUF_THLD_MASK: u32 = 0x001c0000;

    /// **REG_IBI_DATA_BUF_THLD**
    ///
    /// In-Band Interrupt Data Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI data entries in the IBI buffer that trigger the IBI_DATA_THLD_STAT interrupt.
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const REG_IBI_DATA_BUF_THLD_OFFSET: u32 = 12;
    pub const REG_IBI_DATA_BUF_THLD_MASK: u32 = 0x00007000;

    /// **REG_RESP_BUF_THLD**
    ///
    /// Response Buffer Threshold Value is used to control the number of entries in the Response Buffer that trigger the RESP_READY_STAT_INTR.
    ///
    /// Bits `[8:6]`, mask `0x000001c0`
    pub const REG_RESP_BUF_THLD_OFFSET: u32 = 6;
    pub const REG_RESP_BUF_THLD_MASK: u32 = 0x000001c0;

    /// **REG_CMD_BUF_EMPTY_THLD**
    ///
    /// Command Buffer Empty Threshold Value is used to control the number of empty locations(or greater) in the Command Buffer that trigger CMD_BUFFER_READY_STAT interrupt.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const REG_CMD_BUF_EMPTY_THLD_OFFSET: u32 = 0;
    pub const REG_CMD_BUF_EMPTY_THLD_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **REG_IBI_STATUS_BUF_THLD** `[20:18]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_ibi_status_buf_thld(&self) -> u32 {
        (self.value.read_bits() & Self::REG_IBI_STATUS_BUF_THLD_MASK)
            >> Self::REG_IBI_STATUS_BUF_THLD_OFFSET
    }

    /// Write **REG_IBI_STATUS_BUF_THLD** `[20:18]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_ibi_status_buf_thld(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_IBI_STATUS_BUF_THLD_MASK)
                | ((value << Self::REG_IBI_STATUS_BUF_THLD_OFFSET)
                    & Self::REG_IBI_STATUS_BUF_THLD_MASK)
        });
    }

    /// **REG_IBI_DATA_BUF_THLD** `[14:12]` (RW)
    ///
    /// In-Band Interrupt Data Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI data entries in the IBI buffer that trigger the IBI_DATA_THLD_STAT interrupt.
    #[inline(always)]
    pub fn reg_ibi_data_buf_thld(&self) -> u32 {
        (self.value.read_bits() & Self::REG_IBI_DATA_BUF_THLD_MASK)
            >> Self::REG_IBI_DATA_BUF_THLD_OFFSET
    }

    /// Write **REG_IBI_DATA_BUF_THLD** `[14:12]` (RW)
    ///
    /// In-Band Interrupt Data Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI data entries in the IBI buffer that trigger the IBI_DATA_THLD_STAT interrupt.
    #[inline(always)]
    pub fn write_reg_ibi_data_buf_thld(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_IBI_DATA_BUF_THLD_MASK)
                | ((value << Self::REG_IBI_DATA_BUF_THLD_OFFSET) & Self::REG_IBI_DATA_BUF_THLD_MASK)
        });
    }

    /// **REG_RESP_BUF_THLD** `[8:6]` (RW)
    ///
    /// Response Buffer Threshold Value is used to control the number of entries in the Response Buffer that trigger the RESP_READY_STAT_INTR.
    #[inline(always)]
    pub fn reg_resp_buf_thld(&self) -> u32 {
        (self.value.read_bits() & Self::REG_RESP_BUF_THLD_MASK) >> Self::REG_RESP_BUF_THLD_OFFSET
    }

    /// Write **REG_RESP_BUF_THLD** `[8:6]` (RW)
    ///
    /// Response Buffer Threshold Value is used to control the number of entries in the Response Buffer that trigger the RESP_READY_STAT_INTR.
    #[inline(always)]
    pub fn write_reg_resp_buf_thld(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_RESP_BUF_THLD_MASK)
                | ((value << Self::REG_RESP_BUF_THLD_OFFSET) & Self::REG_RESP_BUF_THLD_MASK)
        });
    }

    /// **REG_CMD_BUF_EMPTY_THLD** `[3:0]` (RW)
    ///
    /// Command Buffer Empty Threshold Value is used to control the number of empty locations(or greater) in the Command Buffer that trigger CMD_BUFFER_READY_STAT interrupt.
    #[inline(always)]
    pub fn reg_cmd_buf_empty_thld(&self) -> u32 {
        (self.value.read_bits() & Self::REG_CMD_BUF_EMPTY_THLD_MASK)
            >> Self::REG_CMD_BUF_EMPTY_THLD_OFFSET
    }

    /// Write **REG_CMD_BUF_EMPTY_THLD** `[3:0]` (RW)
    ///
    /// Command Buffer Empty Threshold Value is used to control the number of empty locations(or greater) in the Command Buffer that trigger CMD_BUFFER_READY_STAT interrupt.
    #[inline(always)]
    pub fn write_reg_cmd_buf_empty_thld(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_CMD_BUF_EMPTY_THLD_MASK)
                | ((value << Self::REG_CMD_BUF_EMPTY_THLD_OFFSET)
                    & Self::REG_CMD_BUF_EMPTY_THLD_MASK)
        });
    }
}

/// BUFFER_THLD_CTRL value type for batch read/modify/write operations
///
/// Use `bufferthldctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bufferthldctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BufferThldCtrlVal {
    bits: u32,
}

impl BufferThldCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00041041 }
    }

    /// Read **REG_IBI_STATUS_BUF_THLD** `[20:18]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_ibi_status_buf_thld(&self) -> u32 {
        (self.bits & BufferThldCtrl::REG_IBI_STATUS_BUF_THLD_MASK)
            >> BufferThldCtrl::REG_IBI_STATUS_BUF_THLD_OFFSET
    }

    /// Read **REG_IBI_DATA_BUF_THLD** `[14:12]`
    ///
    /// In-Band Interrupt Data Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI data entries in the IBI buffer that trigger the IBI_DATA_THLD_STAT interrupt.
    #[inline(always)]
    pub const fn reg_ibi_data_buf_thld(&self) -> u32 {
        (self.bits & BufferThldCtrl::REG_IBI_DATA_BUF_THLD_MASK)
            >> BufferThldCtrl::REG_IBI_DATA_BUF_THLD_OFFSET
    }

    /// Read **REG_RESP_BUF_THLD** `[8:6]`
    ///
    /// Response Buffer Threshold Value is used to control the number of entries in the Response Buffer that trigger the RESP_READY_STAT_INTR.
    #[inline(always)]
    pub const fn reg_resp_buf_thld(&self) -> u32 {
        (self.bits & BufferThldCtrl::REG_RESP_BUF_THLD_MASK)
            >> BufferThldCtrl::REG_RESP_BUF_THLD_OFFSET
    }

    /// Read **REG_CMD_BUF_EMPTY_THLD** `[3:0]`
    ///
    /// Command Buffer Empty Threshold Value is used to control the number of empty locations(or greater) in the Command Buffer that trigger CMD_BUFFER_READY_STAT interrupt.
    #[inline(always)]
    pub const fn reg_cmd_buf_empty_thld(&self) -> u32 {
        (self.bits & BufferThldCtrl::REG_CMD_BUF_EMPTY_THLD_MASK)
            >> BufferThldCtrl::REG_CMD_BUF_EMPTY_THLD_OFFSET
    }

    /// Set **REG_IBI_STATUS_BUF_THLD** `[20:18]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_ibi_status_buf_thld(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BufferThldCtrl::REG_IBI_STATUS_BUF_THLD_MASK)
                | ((value << BufferThldCtrl::REG_IBI_STATUS_BUF_THLD_OFFSET)
                    & BufferThldCtrl::REG_IBI_STATUS_BUF_THLD_MASK),
        }
    }

    /// Set **REG_IBI_DATA_BUF_THLD** `[14:12]`
    ///
    /// In-Band Interrupt Data Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI data entries in the IBI buffer that trigger the IBI_DATA_THLD_STAT interrupt.
    #[inline(always)]
    pub const fn with_reg_ibi_data_buf_thld(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BufferThldCtrl::REG_IBI_DATA_BUF_THLD_MASK)
                | ((value << BufferThldCtrl::REG_IBI_DATA_BUF_THLD_OFFSET)
                    & BufferThldCtrl::REG_IBI_DATA_BUF_THLD_MASK),
        }
    }

    /// Set **REG_RESP_BUF_THLD** `[8:6]`
    ///
    /// Response Buffer Threshold Value is used to control the number of entries in the Response Buffer that trigger the RESP_READY_STAT_INTR.
    #[inline(always)]
    pub const fn with_reg_resp_buf_thld(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BufferThldCtrl::REG_RESP_BUF_THLD_MASK)
                | ((value << BufferThldCtrl::REG_RESP_BUF_THLD_OFFSET)
                    & BufferThldCtrl::REG_RESP_BUF_THLD_MASK),
        }
    }

    /// Set **REG_CMD_BUF_EMPTY_THLD** `[3:0]`
    ///
    /// Command Buffer Empty Threshold Value is used to control the number of empty locations(or greater) in the Command Buffer that trigger CMD_BUFFER_READY_STAT interrupt.
    #[inline(always)]
    pub const fn with_reg_cmd_buf_empty_thld(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BufferThldCtrl::REG_CMD_BUF_EMPTY_THLD_MASK)
                | ((value << BufferThldCtrl::REG_CMD_BUF_EMPTY_THLD_OFFSET)
                    & BufferThldCtrl::REG_CMD_BUF_EMPTY_THLD_MASK),
        }
    }
}

impl Default for BufferThldCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BufferThldCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BufferThldCtrlVal {
        BufferThldCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BufferThldCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BufferThldCtrlVal) -> BufferThldCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATA_BUFFER_THLD_CTRL Register
///
/// NA
///
/// Address offset: 0x0020
/// Reset value: 0x00000009
///
/// ## Fields
///
/// - **REG_RX_DATA_BUF_THLD** `[5:3]` (RW)
///
///   Receive Buffer Threshold Value. This field controls the number of empty locations in the Receive FIFO that trigger the RX_THLD_STAT interrupt. Supports: 000:2 001:4 010:8 011:16 100:31, else:31
///
/// - **REG_TX_DATA_BUF_THLD** `[2:0]` (RW)
///
///   Transmit Buffer Threshold Value. This field controls the number of empty locations in the Transmit FIFO that trigger the TX_THLD_STAT interrupt. Supports values: 000:2 001:4 010:8 011:16 100:31, else:31
#[repr(transparent)]
pub struct DataBufferThldCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DataBufferThldCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DataBufferThldCtrl {
    // ======== Field Constants ========

    /// **REG_RX_DATA_BUF_THLD**
    ///
    /// Receive Buffer Threshold Value. This field controls the number of empty locations in the Receive FIFO that trigger the RX_THLD_STAT interrupt. Supports: 000:2 001:4 010:8 011:16 100:31, else:31
    ///
    /// Bits `[5:3]`, mask `0x00000038`
    pub const REG_RX_DATA_BUF_THLD_OFFSET: u32 = 3;
    pub const REG_RX_DATA_BUF_THLD_MASK: u32 = 0x00000038;

    /// **REG_TX_DATA_BUF_THLD**
    ///
    /// Transmit Buffer Threshold Value. This field controls the number of empty locations in the Transmit FIFO that trigger the TX_THLD_STAT interrupt. Supports values: 000:2 001:4 010:8 011:16 100:31, else:31
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const REG_TX_DATA_BUF_THLD_OFFSET: u32 = 0;
    pub const REG_TX_DATA_BUF_THLD_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **REG_RX_DATA_BUF_THLD** `[5:3]` (RW)
    ///
    /// Receive Buffer Threshold Value. This field controls the number of empty locations in the Receive FIFO that trigger the RX_THLD_STAT interrupt. Supports: 000:2 001:4 010:8 011:16 100:31, else:31
    #[inline(always)]
    pub fn reg_rx_data_buf_thld(&self) -> u32 {
        (self.value.read_bits() & Self::REG_RX_DATA_BUF_THLD_MASK)
            >> Self::REG_RX_DATA_BUF_THLD_OFFSET
    }

    /// Write **REG_RX_DATA_BUF_THLD** `[5:3]` (RW)
    ///
    /// Receive Buffer Threshold Value. This field controls the number of empty locations in the Receive FIFO that trigger the RX_THLD_STAT interrupt. Supports: 000:2 001:4 010:8 011:16 100:31, else:31
    #[inline(always)]
    pub fn write_reg_rx_data_buf_thld(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_RX_DATA_BUF_THLD_MASK)
                | ((value << Self::REG_RX_DATA_BUF_THLD_OFFSET) & Self::REG_RX_DATA_BUF_THLD_MASK)
        });
    }

    /// **REG_TX_DATA_BUF_THLD** `[2:0]` (RW)
    ///
    /// Transmit Buffer Threshold Value. This field controls the number of empty locations in the Transmit FIFO that trigger the TX_THLD_STAT interrupt. Supports values: 000:2 001:4 010:8 011:16 100:31, else:31
    #[inline(always)]
    pub fn reg_tx_data_buf_thld(&self) -> u32 {
        (self.value.read_bits() & Self::REG_TX_DATA_BUF_THLD_MASK)
            >> Self::REG_TX_DATA_BUF_THLD_OFFSET
    }

    /// Write **REG_TX_DATA_BUF_THLD** `[2:0]` (RW)
    ///
    /// Transmit Buffer Threshold Value. This field controls the number of empty locations in the Transmit FIFO that trigger the TX_THLD_STAT interrupt. Supports values: 000:2 001:4 010:8 011:16 100:31, else:31
    #[inline(always)]
    pub fn write_reg_tx_data_buf_thld(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_TX_DATA_BUF_THLD_MASK)
                | ((value << Self::REG_TX_DATA_BUF_THLD_OFFSET) & Self::REG_TX_DATA_BUF_THLD_MASK)
        });
    }
}

/// DATA_BUFFER_THLD_CTRL value type for batch read/modify/write operations
///
/// Use `databufferthldctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `databufferthldctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DataBufferThldCtrlVal {
    bits: u32,
}

impl DataBufferThldCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00000009 }
    }

    /// Read **REG_RX_DATA_BUF_THLD** `[5:3]`
    ///
    /// Receive Buffer Threshold Value. This field controls the number of empty locations in the Receive FIFO that trigger the RX_THLD_STAT interrupt. Supports: 000:2 001:4 010:8 011:16 100:31, else:31
    #[inline(always)]
    pub const fn reg_rx_data_buf_thld(&self) -> u32 {
        (self.bits & DataBufferThldCtrl::REG_RX_DATA_BUF_THLD_MASK)
            >> DataBufferThldCtrl::REG_RX_DATA_BUF_THLD_OFFSET
    }

    /// Read **REG_TX_DATA_BUF_THLD** `[2:0]`
    ///
    /// Transmit Buffer Threshold Value. This field controls the number of empty locations in the Transmit FIFO that trigger the TX_THLD_STAT interrupt. Supports values: 000:2 001:4 010:8 011:16 100:31, else:31
    #[inline(always)]
    pub const fn reg_tx_data_buf_thld(&self) -> u32 {
        (self.bits & DataBufferThldCtrl::REG_TX_DATA_BUF_THLD_MASK)
            >> DataBufferThldCtrl::REG_TX_DATA_BUF_THLD_OFFSET
    }

    /// Set **REG_RX_DATA_BUF_THLD** `[5:3]`
    ///
    /// Receive Buffer Threshold Value. This field controls the number of empty locations in the Receive FIFO that trigger the RX_THLD_STAT interrupt. Supports: 000:2 001:4 010:8 011:16 100:31, else:31
    #[inline(always)]
    pub const fn with_reg_rx_data_buf_thld(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DataBufferThldCtrl::REG_RX_DATA_BUF_THLD_MASK)
                | ((value << DataBufferThldCtrl::REG_RX_DATA_BUF_THLD_OFFSET)
                    & DataBufferThldCtrl::REG_RX_DATA_BUF_THLD_MASK),
        }
    }

    /// Set **REG_TX_DATA_BUF_THLD** `[2:0]`
    ///
    /// Transmit Buffer Threshold Value. This field controls the number of empty locations in the Transmit FIFO that trigger the TX_THLD_STAT interrupt. Supports values: 000:2 001:4 010:8 011:16 100:31, else:31
    #[inline(always)]
    pub const fn with_reg_tx_data_buf_thld(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DataBufferThldCtrl::REG_TX_DATA_BUF_THLD_MASK)
                | ((value << DataBufferThldCtrl::REG_TX_DATA_BUF_THLD_OFFSET)
                    & DataBufferThldCtrl::REG_TX_DATA_BUF_THLD_MASK),
        }
    }
}

impl Default for DataBufferThldCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DataBufferThldCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DataBufferThldCtrlVal {
        DataBufferThldCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DataBufferThldCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DataBufferThldCtrlVal) -> DataBufferThldCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IBI_NOTIFY_CTRL Register
///
/// NA
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **REG_NOTIFY_SIR_REJECTED** `[2]` (RW)
///
///   Notify Rejected Slave Interrupt Request Control. This bit is used to suppress reporting to the application about Slave Interrupt Request. 0:Suppress passing the IBI Status to the IBI FIFO(hence not notifying the application) when a SIR request is NACKed and auto-disabled base on the IBI_SIR_REQ_REJECT register. 1: Writes IBI Status to the IBI FIFO(hence notifying the application) when SIR request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT registerl.
#[repr(transparent)]
pub struct IbiNotifyCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IbiNotifyCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IbiNotifyCtrl {
    // ======== Field Constants ========

    /// **REG_NOTIFY_SIR_REJECTED**
    ///
    /// Notify Rejected Slave Interrupt Request Control. This bit is used to suppress reporting to the application about Slave Interrupt Request. 0:Suppress passing the IBI Status to the IBI FIFO(hence not notifying the application) when a SIR request is NACKed and auto-disabled base on the IBI_SIR_REQ_REJECT register. 1: Writes IBI Status to the IBI FIFO(hence notifying the application) when SIR request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT registerl.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_NOTIFY_SIR_REJECTED_OFFSET: u32 = 2;
    pub const REG_NOTIFY_SIR_REJECTED_MASK: u32 = 0x00000004;
    pub const REG_NOTIFY_SIR_REJECTED: u32 = 1 << 2;

    // ======== Field Accessors ========

    /// **REG_NOTIFY_SIR_REJECTED** `[2]` (RW)
    ///
    /// Notify Rejected Slave Interrupt Request Control. This bit is used to suppress reporting to the application about Slave Interrupt Request. 0:Suppress passing the IBI Status to the IBI FIFO(hence not notifying the application) when a SIR request is NACKed and auto-disabled base on the IBI_SIR_REQ_REJECT register. 1: Writes IBI Status to the IBI FIFO(hence notifying the application) when SIR request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT registerl.
    #[inline(always)]
    pub fn reg_notify_sir_rejected(&self) -> bool {
        (self.value.read_bits() & Self::REG_NOTIFY_SIR_REJECTED_MASK) != 0
    }

    /// Set REG_NOTIFY_SIR_REJECTED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_notify_sir_rejected(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_NOTIFY_SIR_REJECTED_MASK);
    }

    /// Clear REG_NOTIFY_SIR_REJECTED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_notify_sir_rejected(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_NOTIFY_SIR_REJECTED_MASK);
    }
}

/// IBI_NOTIFY_CTRL value type for batch read/modify/write operations
///
/// Use `ibinotifyctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ibinotifyctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IbiNotifyCtrlVal {
    bits: u32,
}

impl IbiNotifyCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_NOTIFY_SIR_REJECTED** `[2:2]`
    ///
    /// Notify Rejected Slave Interrupt Request Control. This bit is used to suppress reporting to the application about Slave Interrupt Request. 0:Suppress passing the IBI Status to the IBI FIFO(hence not notifying the application) when a SIR request is NACKed and auto-disabled base on the IBI_SIR_REQ_REJECT register. 1: Writes IBI Status to the IBI FIFO(hence notifying the application) when SIR request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT registerl.
    #[inline(always)]
    pub const fn reg_notify_sir_rejected(&self) -> bool {
        (self.bits & IbiNotifyCtrl::REG_NOTIFY_SIR_REJECTED_MASK) != 0
    }

    /// Set **REG_NOTIFY_SIR_REJECTED** `[2:2]`
    ///
    /// Notify Rejected Slave Interrupt Request Control. This bit is used to suppress reporting to the application about Slave Interrupt Request. 0:Suppress passing the IBI Status to the IBI FIFO(hence not notifying the application) when a SIR request is NACKed and auto-disabled base on the IBI_SIR_REQ_REJECT register. 1: Writes IBI Status to the IBI FIFO(hence notifying the application) when SIR request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT registerl.
    #[inline(always)]
    pub const fn with_reg_notify_sir_rejected(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IbiNotifyCtrl::REG_NOTIFY_SIR_REJECTED_MASK
            } else {
                self.bits & !IbiNotifyCtrl::REG_NOTIFY_SIR_REJECTED_MASK
            },
        }
    }
}

impl Default for IbiNotifyCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IbiNotifyCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IbiNotifyCtrlVal {
        IbiNotifyCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IbiNotifyCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IbiNotifyCtrlVal) -> IbiNotifyCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IBI_SIR_REQ_PAYLOAD Register
///
/// NA
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **REG_SIR_REQ_PAYLOAD** `[31:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IbiSirReqPayload {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IbiSirReqPayload {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IbiSirReqPayload {
    // ======== Field Constants ========

    /// **REG_SIR_REQ_PAYLOAD**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_SIR_REQ_PAYLOAD_OFFSET: u32 = 0;
    pub const REG_SIR_REQ_PAYLOAD_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_SIR_REQ_PAYLOAD** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_sir_req_payload(&self) -> u32 {
        (self.value.read_bits() & Self::REG_SIR_REQ_PAYLOAD_MASK)
            >> Self::REG_SIR_REQ_PAYLOAD_OFFSET
    }

    /// Write **REG_SIR_REQ_PAYLOAD** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_sir_req_payload(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_SIR_REQ_PAYLOAD_MASK)
                | ((value << Self::REG_SIR_REQ_PAYLOAD_OFFSET) & Self::REG_SIR_REQ_PAYLOAD_MASK)
        });
    }
}

/// IBI_SIR_REQ_PAYLOAD value type for batch read/modify/write operations
///
/// Use `ibisirreqpayload.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ibisirreqpayload.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IbiSirReqPayloadVal {
    bits: u32,
}

impl IbiSirReqPayloadVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_SIR_REQ_PAYLOAD** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_sir_req_payload(&self) -> u32 {
        (self.bits & IbiSirReqPayload::REG_SIR_REQ_PAYLOAD_MASK)
            >> IbiSirReqPayload::REG_SIR_REQ_PAYLOAD_OFFSET
    }

    /// Set **REG_SIR_REQ_PAYLOAD** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_sir_req_payload(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IbiSirReqPayload::REG_SIR_REQ_PAYLOAD_MASK)
                | ((value << IbiSirReqPayload::REG_SIR_REQ_PAYLOAD_OFFSET)
                    & IbiSirReqPayload::REG_SIR_REQ_PAYLOAD_MASK),
        }
    }
}

impl Default for IbiSirReqPayloadVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IbiSirReqPayload {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IbiSirReqPayloadVal {
        IbiSirReqPayloadVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IbiSirReqPayloadVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IbiSirReqPayloadVal) -> IbiSirReqPayloadVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// IBI_SIR_REQ_REJECT Register
///
/// NA
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **REG_SIR_REQ_REJECT** `[31:0]` (RW)
///
///   The application of controller can decide whether to send ACK or NACK for Slave request received from any I3C device. A device specific response control bit is provided to select the response option, Master will ACK/NACK the Master Request based on programming of control bit, corresponding to the interrupting device. 0:ACK the SIR Request 1:NACK and send direct auto disable CCC
#[repr(transparent)]
pub struct IbiSirReqReject {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IbiSirReqReject {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IbiSirReqReject {
    // ======== Field Constants ========

    /// **REG_SIR_REQ_REJECT**
    ///
    /// The application of controller can decide whether to send ACK or NACK for Slave request received from any I3C device. A device specific response control bit is provided to select the response option, Master will ACK/NACK the Master Request based on programming of control bit, corresponding to the interrupting device. 0:ACK the SIR Request 1:NACK and send direct auto disable CCC
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_SIR_REQ_REJECT_OFFSET: u32 = 0;
    pub const REG_SIR_REQ_REJECT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_SIR_REQ_REJECT** `[31:0]` (RW)
    ///
    /// The application of controller can decide whether to send ACK or NACK for Slave request received from any I3C device. A device specific response control bit is provided to select the response option, Master will ACK/NACK the Master Request based on programming of control bit, corresponding to the interrupting device. 0:ACK the SIR Request 1:NACK and send direct auto disable CCC
    #[inline(always)]
    pub fn reg_sir_req_reject(&self) -> u32 {
        (self.value.read_bits() & Self::REG_SIR_REQ_REJECT_MASK) >> Self::REG_SIR_REQ_REJECT_OFFSET
    }

    /// Write **REG_SIR_REQ_REJECT** `[31:0]` (RW)
    ///
    /// The application of controller can decide whether to send ACK or NACK for Slave request received from any I3C device. A device specific response control bit is provided to select the response option, Master will ACK/NACK the Master Request based on programming of control bit, corresponding to the interrupting device. 0:ACK the SIR Request 1:NACK and send direct auto disable CCC
    #[inline(always)]
    pub fn write_reg_sir_req_reject(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_SIR_REQ_REJECT_MASK)
                | ((value << Self::REG_SIR_REQ_REJECT_OFFSET) & Self::REG_SIR_REQ_REJECT_MASK)
        });
    }
}

/// IBI_SIR_REQ_REJECT value type for batch read/modify/write operations
///
/// Use `ibisirreqreject.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ibisirreqreject.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IbiSirReqRejectVal {
    bits: u32,
}

impl IbiSirReqRejectVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_SIR_REQ_REJECT** `[31:0]`
    ///
    /// The application of controller can decide whether to send ACK or NACK for Slave request received from any I3C device. A device specific response control bit is provided to select the response option, Master will ACK/NACK the Master Request based on programming of control bit, corresponding to the interrupting device. 0:ACK the SIR Request 1:NACK and send direct auto disable CCC
    #[inline(always)]
    pub const fn reg_sir_req_reject(&self) -> u32 {
        (self.bits & IbiSirReqReject::REG_SIR_REQ_REJECT_MASK)
            >> IbiSirReqReject::REG_SIR_REQ_REJECT_OFFSET
    }

    /// Set **REG_SIR_REQ_REJECT** `[31:0]`
    ///
    /// The application of controller can decide whether to send ACK or NACK for Slave request received from any I3C device. A device specific response control bit is provided to select the response option, Master will ACK/NACK the Master Request based on programming of control bit, corresponding to the interrupting device. 0:ACK the SIR Request 1:NACK and send direct auto disable CCC
    #[inline(always)]
    pub const fn with_reg_sir_req_reject(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IbiSirReqReject::REG_SIR_REQ_REJECT_MASK)
                | ((value << IbiSirReqReject::REG_SIR_REQ_REJECT_OFFSET)
                    & IbiSirReqReject::REG_SIR_REQ_REJECT_MASK),
        }
    }
}

impl Default for IbiSirReqRejectVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IbiSirReqReject {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IbiSirReqRejectVal {
        IbiSirReqRejectVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IbiSirReqRejectVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IbiSirReqRejectVal) -> IbiSirReqRejectVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_CLR Register
///
/// NA
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **CMD_CCC_MISMATCH_INT_CLR** `[15]` (WO)
///
///   NA
///
/// - **IBI_DETECT_INT_CLR** `[14]` (WO)
///
///   NA
///
/// - **IBI_HANDLE_DONE_INT_CLR** `[13]` (WO)
///
///   NA
///
/// - **IBI_STATUS_BUF_OVF_INT_CLR** `[12]` (WO)
///
///   NA
///
/// - **IBI_DATA_BUF_OVF_INT_CLR** `[11]` (WO)
///
///   NA
///
/// - **RESP_BUF_OVF_INT_CLR** `[10]` (WO)
///
///   NA
///
/// - **DETECT_START_INT_CLR** `[9]` (WO)
///
///   NA
///
/// - **COMMAND_DONE_INT_CLR** `[8]` (WO)
///
///   NA
///
/// - **TRANSFER_COMPLETE_INT_CLR** `[7]` (WO)
///
///   NA
///
/// - **TRANSFER_ERR_INT_CLR** `[6]` (WO)
///
///   NA
///
/// - **NXT_CMD_REQ_ERR_INT_CLR** `[5]` (WO)
///
///   NA
///
/// - **RESP_READY_INT_CLR** `[4]` (WO)
///
///   NA
///
/// - **CMD_BUF_EMPTY_THLD_INT_CLR** `[3]` (WO)
///
///   NA
///
/// - **IBI_STATUS_THLD_INT_CLR** `[2]` (WO)
///
///   NA
///
/// - **RX_DATA_BUF_THLD_INT_CLR** `[1]` (WO)
///
///   NA
///
/// - **TX_DATA_BUF_THLD_INT_CLR** `[0]` (WO)
///
///   NA
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

    /// **CMD_CCC_MISMATCH_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CMD_CCC_MISMATCH_INT_CLR_OFFSET: u32 = 15;
    pub const CMD_CCC_MISMATCH_INT_CLR_MASK: u32 = 0x00008000;
    pub const CMD_CCC_MISMATCH_INT_CLR: u32 = 1 << 15;

    /// **IBI_DETECT_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const IBI_DETECT_INT_CLR_OFFSET: u32 = 14;
    pub const IBI_DETECT_INT_CLR_MASK: u32 = 0x00004000;
    pub const IBI_DETECT_INT_CLR: u32 = 1 << 14;

    /// **IBI_HANDLE_DONE_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const IBI_HANDLE_DONE_INT_CLR_OFFSET: u32 = 13;
    pub const IBI_HANDLE_DONE_INT_CLR_MASK: u32 = 0x00002000;
    pub const IBI_HANDLE_DONE_INT_CLR: u32 = 1 << 13;

    /// **IBI_STATUS_BUF_OVF_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const IBI_STATUS_BUF_OVF_INT_CLR_OFFSET: u32 = 12;
    pub const IBI_STATUS_BUF_OVF_INT_CLR_MASK: u32 = 0x00001000;
    pub const IBI_STATUS_BUF_OVF_INT_CLR: u32 = 1 << 12;

    /// **IBI_DATA_BUF_OVF_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const IBI_DATA_BUF_OVF_INT_CLR_OFFSET: u32 = 11;
    pub const IBI_DATA_BUF_OVF_INT_CLR_MASK: u32 = 0x00000800;
    pub const IBI_DATA_BUF_OVF_INT_CLR: u32 = 1 << 11;

    /// **RESP_BUF_OVF_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const RESP_BUF_OVF_INT_CLR_OFFSET: u32 = 10;
    pub const RESP_BUF_OVF_INT_CLR_MASK: u32 = 0x00000400;
    pub const RESP_BUF_OVF_INT_CLR: u32 = 1 << 10;

    /// **DETECT_START_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DETECT_START_INT_CLR_OFFSET: u32 = 9;
    pub const DETECT_START_INT_CLR_MASK: u32 = 0x00000200;
    pub const DETECT_START_INT_CLR: u32 = 1 << 9;

    /// **COMMAND_DONE_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const COMMAND_DONE_INT_CLR_OFFSET: u32 = 8;
    pub const COMMAND_DONE_INT_CLR_MASK: u32 = 0x00000100;
    pub const COMMAND_DONE_INT_CLR: u32 = 1 << 8;

    /// **TRANSFER_COMPLETE_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TRANSFER_COMPLETE_INT_CLR_OFFSET: u32 = 7;
    pub const TRANSFER_COMPLETE_INT_CLR_MASK: u32 = 0x00000080;
    pub const TRANSFER_COMPLETE_INT_CLR: u32 = 1 << 7;

    /// **TRANSFER_ERR_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TRANSFER_ERR_INT_CLR_OFFSET: u32 = 6;
    pub const TRANSFER_ERR_INT_CLR_MASK: u32 = 0x00000040;
    pub const TRANSFER_ERR_INT_CLR: u32 = 1 << 6;

    /// **NXT_CMD_REQ_ERR_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const NXT_CMD_REQ_ERR_INT_CLR_OFFSET: u32 = 5;
    pub const NXT_CMD_REQ_ERR_INT_CLR_MASK: u32 = 0x00000020;
    pub const NXT_CMD_REQ_ERR_INT_CLR: u32 = 1 << 5;

    /// **RESP_READY_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RESP_READY_INT_CLR_OFFSET: u32 = 4;
    pub const RESP_READY_INT_CLR_MASK: u32 = 0x00000010;
    pub const RESP_READY_INT_CLR: u32 = 1 << 4;

    /// **CMD_BUF_EMPTY_THLD_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CMD_BUF_EMPTY_THLD_INT_CLR_OFFSET: u32 = 3;
    pub const CMD_BUF_EMPTY_THLD_INT_CLR_MASK: u32 = 0x00000008;
    pub const CMD_BUF_EMPTY_THLD_INT_CLR: u32 = 1 << 3;

    /// **IBI_STATUS_THLD_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IBI_STATUS_THLD_INT_CLR_OFFSET: u32 = 2;
    pub const IBI_STATUS_THLD_INT_CLR_MASK: u32 = 0x00000004;
    pub const IBI_STATUS_THLD_INT_CLR: u32 = 1 << 2;

    /// **RX_DATA_BUF_THLD_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_DATA_BUF_THLD_INT_CLR_OFFSET: u32 = 1;
    pub const RX_DATA_BUF_THLD_INT_CLR_MASK: u32 = 0x00000002;
    pub const RX_DATA_BUF_THLD_INT_CLR: u32 = 1 << 1;

    /// **TX_DATA_BUF_THLD_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_DATA_BUF_THLD_INT_CLR_OFFSET: u32 = 0;
    pub const TX_DATA_BUF_THLD_INT_CLR_MASK: u32 = 0x00000001;
    pub const TX_DATA_BUF_THLD_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CMD_CCC_MISMATCH_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_cmd_ccc_mismatch_int_clr(&self) {
        self.value.write_bits(Self::CMD_CCC_MISMATCH_INT_CLR_MASK);
    }

    /// Trigger **IBI_DETECT_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_ibi_detect_int_clr(&self) {
        self.value.write_bits(Self::IBI_DETECT_INT_CLR_MASK);
    }

    /// Trigger **IBI_HANDLE_DONE_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_ibi_handle_done_int_clr(&self) {
        self.value.write_bits(Self::IBI_HANDLE_DONE_INT_CLR_MASK);
    }

    /// Trigger **IBI_STATUS_BUF_OVF_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_ibi_status_buf_ovf_int_clr(&self) {
        self.value.write_bits(Self::IBI_STATUS_BUF_OVF_INT_CLR_MASK);
    }

    /// Trigger **IBI_DATA_BUF_OVF_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_ibi_data_buf_ovf_int_clr(&self) {
        self.value.write_bits(Self::IBI_DATA_BUF_OVF_INT_CLR_MASK);
    }

    /// Trigger **RESP_BUF_OVF_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_resp_buf_ovf_int_clr(&self) {
        self.value.write_bits(Self::RESP_BUF_OVF_INT_CLR_MASK);
    }

    /// Trigger **DETECT_START_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_detect_start_int_clr(&self) {
        self.value.write_bits(Self::DETECT_START_INT_CLR_MASK);
    }

    /// Trigger **COMMAND_DONE_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_command_done_int_clr(&self) {
        self.value.write_bits(Self::COMMAND_DONE_INT_CLR_MASK);
    }

    /// Trigger **TRANSFER_COMPLETE_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_transfer_complete_int_clr(&self) {
        self.value.write_bits(Self::TRANSFER_COMPLETE_INT_CLR_MASK);
    }

    /// Trigger **TRANSFER_ERR_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_transfer_err_int_clr(&self) {
        self.value.write_bits(Self::TRANSFER_ERR_INT_CLR_MASK);
    }

    /// Trigger **NXT_CMD_REQ_ERR_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_nxt_cmd_req_err_int_clr(&self) {
        self.value.write_bits(Self::NXT_CMD_REQ_ERR_INT_CLR_MASK);
    }

    /// Trigger **RESP_READY_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_resp_ready_int_clr(&self) {
        self.value.write_bits(Self::RESP_READY_INT_CLR_MASK);
    }

    /// Trigger **CMD_BUF_EMPTY_THLD_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_cmd_buf_empty_thld_int_clr(&self) {
        self.value.write_bits(Self::CMD_BUF_EMPTY_THLD_INT_CLR_MASK);
    }

    /// Trigger **IBI_STATUS_THLD_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_ibi_status_thld_int_clr(&self) {
        self.value.write_bits(Self::IBI_STATUS_THLD_INT_CLR_MASK);
    }

    /// Trigger **RX_DATA_BUF_THLD_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_rx_data_buf_thld_int_clr(&self) {
        self.value.write_bits(Self::RX_DATA_BUF_THLD_INT_CLR_MASK);
    }

    /// Trigger **TX_DATA_BUF_THLD_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_tx_data_buf_thld_int_clr(&self) {
        self.value.write_bits(Self::TX_DATA_BUF_THLD_INT_CLR_MASK);
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

    /// Set **CMD_CCC_MISMATCH_INT_CLR** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_cmd_ccc_mismatch_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CMD_CCC_MISMATCH_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CMD_CCC_MISMATCH_INT_CLR_MASK
            },
        }
    }

    /// Set **IBI_DETECT_INT_CLR** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ibi_detect_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::IBI_DETECT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::IBI_DETECT_INT_CLR_MASK
            },
        }
    }

    /// Set **IBI_HANDLE_DONE_INT_CLR** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ibi_handle_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::IBI_HANDLE_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::IBI_HANDLE_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **IBI_STATUS_BUF_OVF_INT_CLR** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ibi_status_buf_ovf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::IBI_STATUS_BUF_OVF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::IBI_STATUS_BUF_OVF_INT_CLR_MASK
            },
        }
    }

    /// Set **IBI_DATA_BUF_OVF_INT_CLR** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ibi_data_buf_ovf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::IBI_DATA_BUF_OVF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::IBI_DATA_BUF_OVF_INT_CLR_MASK
            },
        }
    }

    /// Set **RESP_BUF_OVF_INT_CLR** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_resp_buf_ovf_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RESP_BUF_OVF_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RESP_BUF_OVF_INT_CLR_MASK
            },
        }
    }

    /// Set **DETECT_START_INT_CLR** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_detect_start_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DETECT_START_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DETECT_START_INT_CLR_MASK
            },
        }
    }

    /// Set **COMMAND_DONE_INT_CLR** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_command_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::COMMAND_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::COMMAND_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **TRANSFER_COMPLETE_INT_CLR** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_transfer_complete_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TRANSFER_COMPLETE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TRANSFER_COMPLETE_INT_CLR_MASK
            },
        }
    }

    /// Set **TRANSFER_ERR_INT_CLR** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_transfer_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TRANSFER_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TRANSFER_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **NXT_CMD_REQ_ERR_INT_CLR** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_nxt_cmd_req_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::NXT_CMD_REQ_ERR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::NXT_CMD_REQ_ERR_INT_CLR_MASK
            },
        }
    }

    /// Set **RESP_READY_INT_CLR** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_resp_ready_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RESP_READY_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RESP_READY_INT_CLR_MASK
            },
        }
    }

    /// Set **CMD_BUF_EMPTY_THLD_INT_CLR** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_cmd_buf_empty_thld_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CMD_BUF_EMPTY_THLD_INT_CLR_MASK
            } else {
                self.bits & !IntClr::CMD_BUF_EMPTY_THLD_INT_CLR_MASK
            },
        }
    }

    /// Set **IBI_STATUS_THLD_INT_CLR** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ibi_status_thld_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::IBI_STATUS_THLD_INT_CLR_MASK
            } else {
                self.bits & !IntClr::IBI_STATUS_THLD_INT_CLR_MASK
            },
        }
    }

    /// Set **RX_DATA_BUF_THLD_INT_CLR** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_rx_data_buf_thld_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::RX_DATA_BUF_THLD_INT_CLR_MASK
            } else {
                self.bits & !IntClr::RX_DATA_BUF_THLD_INT_CLR_MASK
            },
        }
    }

    /// Set **TX_DATA_BUF_THLD_INT_CLR** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_tx_data_buf_thld_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TX_DATA_BUF_THLD_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TX_DATA_BUF_THLD_INT_CLR_MASK
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
/// NA
///
/// Address offset: 0x0034
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **CMD_CCC_MISMATCH_INT_RAW** `[15]` (RW)
///
///   NA
///
/// - **IBI_DETECT_INT_RAW** `[14]` (RW)
///
///   NA
///
/// - **IBI_HANDLE_DONE_INT_RAW** `[13]` (RW)
///
///   NA
///
/// - **IBI_STATUS_BUF_OVF_INT_RAW** `[12]` (RW)
///
///   NA
///
/// - **IBI_DATA_BUF_OVF_INT_RAW** `[11]` (RW)
///
///   NA
///
/// - **RESP_BUF_OVF_INT_RAW** `[10]` (RW)
///
///   NA
///
/// - **DETECT_START_INT_RAW** `[9]` (RW)
///
///   NA
///
/// - **COMMAND_DONE_INT_RAW** `[8]` (RW)
///
///   NA
///
/// - **TRANSFER_COMPLETE_INT_RAW** `[7]` (RW)
///
///   NA
///
/// - **TRANSFER_ERR_INT_RAW** `[6]` (RW)
///
///   NA
///
/// - **NXT_CMD_REQ_ERR_INT_RAW** `[5]` (RW)
///
///   NA
///
/// - **RESP_READY_INT_RAW** `[4]` (RW)
///
///   NA
///
/// - **CMD_BUF_EMPTY_THLD_INT_RAW** `[3]` (RW)
///
///   NA
///
/// - **IBI_STATUS_THLD_INT_RAW** `[2]` (RW)
///
///   NA
///
/// - **RX_DATA_BUF_THLD_INT_RAW** `[1]` (RW)
///
///   NA
///
/// - **TX_DATA_BUF_THLD_INT_RAW** `[0]` (RW)
///
///   NA
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

    /// **CMD_CCC_MISMATCH_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CMD_CCC_MISMATCH_INT_RAW_OFFSET: u32 = 15;
    pub const CMD_CCC_MISMATCH_INT_RAW_MASK: u32 = 0x00008000;
    pub const CMD_CCC_MISMATCH_INT_RAW: u32 = 1 << 15;

    /// **IBI_DETECT_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const IBI_DETECT_INT_RAW_OFFSET: u32 = 14;
    pub const IBI_DETECT_INT_RAW_MASK: u32 = 0x00004000;
    pub const IBI_DETECT_INT_RAW: u32 = 1 << 14;

    /// **IBI_HANDLE_DONE_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const IBI_HANDLE_DONE_INT_RAW_OFFSET: u32 = 13;
    pub const IBI_HANDLE_DONE_INT_RAW_MASK: u32 = 0x00002000;
    pub const IBI_HANDLE_DONE_INT_RAW: u32 = 1 << 13;

    /// **IBI_STATUS_BUF_OVF_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const IBI_STATUS_BUF_OVF_INT_RAW_OFFSET: u32 = 12;
    pub const IBI_STATUS_BUF_OVF_INT_RAW_MASK: u32 = 0x00001000;
    pub const IBI_STATUS_BUF_OVF_INT_RAW: u32 = 1 << 12;

    /// **IBI_DATA_BUF_OVF_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const IBI_DATA_BUF_OVF_INT_RAW_OFFSET: u32 = 11;
    pub const IBI_DATA_BUF_OVF_INT_RAW_MASK: u32 = 0x00000800;
    pub const IBI_DATA_BUF_OVF_INT_RAW: u32 = 1 << 11;

    /// **RESP_BUF_OVF_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const RESP_BUF_OVF_INT_RAW_OFFSET: u32 = 10;
    pub const RESP_BUF_OVF_INT_RAW_MASK: u32 = 0x00000400;
    pub const RESP_BUF_OVF_INT_RAW: u32 = 1 << 10;

    /// **DETECT_START_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DETECT_START_INT_RAW_OFFSET: u32 = 9;
    pub const DETECT_START_INT_RAW_MASK: u32 = 0x00000200;
    pub const DETECT_START_INT_RAW: u32 = 1 << 9;

    /// **COMMAND_DONE_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const COMMAND_DONE_INT_RAW_OFFSET: u32 = 8;
    pub const COMMAND_DONE_INT_RAW_MASK: u32 = 0x00000100;
    pub const COMMAND_DONE_INT_RAW: u32 = 1 << 8;

    /// **TRANSFER_COMPLETE_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TRANSFER_COMPLETE_INT_RAW_OFFSET: u32 = 7;
    pub const TRANSFER_COMPLETE_INT_RAW_MASK: u32 = 0x00000080;
    pub const TRANSFER_COMPLETE_INT_RAW: u32 = 1 << 7;

    /// **TRANSFER_ERR_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TRANSFER_ERR_INT_RAW_OFFSET: u32 = 6;
    pub const TRANSFER_ERR_INT_RAW_MASK: u32 = 0x00000040;
    pub const TRANSFER_ERR_INT_RAW: u32 = 1 << 6;

    /// **NXT_CMD_REQ_ERR_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const NXT_CMD_REQ_ERR_INT_RAW_OFFSET: u32 = 5;
    pub const NXT_CMD_REQ_ERR_INT_RAW_MASK: u32 = 0x00000020;
    pub const NXT_CMD_REQ_ERR_INT_RAW: u32 = 1 << 5;

    /// **RESP_READY_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RESP_READY_INT_RAW_OFFSET: u32 = 4;
    pub const RESP_READY_INT_RAW_MASK: u32 = 0x00000010;
    pub const RESP_READY_INT_RAW: u32 = 1 << 4;

    /// **CMD_BUF_EMPTY_THLD_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CMD_BUF_EMPTY_THLD_INT_RAW_OFFSET: u32 = 3;
    pub const CMD_BUF_EMPTY_THLD_INT_RAW_MASK: u32 = 0x00000008;
    pub const CMD_BUF_EMPTY_THLD_INT_RAW: u32 = 1 << 3;

    /// **IBI_STATUS_THLD_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IBI_STATUS_THLD_INT_RAW_OFFSET: u32 = 2;
    pub const IBI_STATUS_THLD_INT_RAW_MASK: u32 = 0x00000004;
    pub const IBI_STATUS_THLD_INT_RAW: u32 = 1 << 2;

    /// **RX_DATA_BUF_THLD_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_DATA_BUF_THLD_INT_RAW_OFFSET: u32 = 1;
    pub const RX_DATA_BUF_THLD_INT_RAW_MASK: u32 = 0x00000002;
    pub const RX_DATA_BUF_THLD_INT_RAW: u32 = 1 << 1;

    /// **TX_DATA_BUF_THLD_INT_RAW**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_DATA_BUF_THLD_INT_RAW_OFFSET: u32 = 0;
    pub const TX_DATA_BUF_THLD_INT_RAW_MASK: u32 = 0x00000001;
    pub const TX_DATA_BUF_THLD_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CMD_CCC_MISMATCH_INT_RAW** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn cmd_ccc_mismatch_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CMD_CCC_MISMATCH_INT_RAW_MASK) != 0
    }

    /// Set CMD_CCC_MISMATCH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cmd_ccc_mismatch_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CMD_CCC_MISMATCH_INT_RAW_MASK);
    }

    /// Clear CMD_CCC_MISMATCH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cmd_ccc_mismatch_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CMD_CCC_MISMATCH_INT_RAW_MASK);
    }

    /// **IBI_DETECT_INT_RAW** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_detect_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IBI_DETECT_INT_RAW_MASK) != 0
    }

    /// Set IBI_DETECT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ibi_detect_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IBI_DETECT_INT_RAW_MASK);
    }

    /// Clear IBI_DETECT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ibi_detect_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IBI_DETECT_INT_RAW_MASK);
    }

    /// **IBI_HANDLE_DONE_INT_RAW** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_handle_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IBI_HANDLE_DONE_INT_RAW_MASK) != 0
    }

    /// Set IBI_HANDLE_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ibi_handle_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IBI_HANDLE_DONE_INT_RAW_MASK);
    }

    /// Clear IBI_HANDLE_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ibi_handle_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IBI_HANDLE_DONE_INT_RAW_MASK);
    }

    /// **IBI_STATUS_BUF_OVF_INT_RAW** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_status_buf_ovf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IBI_STATUS_BUF_OVF_INT_RAW_MASK) != 0
    }

    /// Set IBI_STATUS_BUF_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ibi_status_buf_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IBI_STATUS_BUF_OVF_INT_RAW_MASK);
    }

    /// Clear IBI_STATUS_BUF_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ibi_status_buf_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IBI_STATUS_BUF_OVF_INT_RAW_MASK);
    }

    /// **IBI_DATA_BUF_OVF_INT_RAW** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_data_buf_ovf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IBI_DATA_BUF_OVF_INT_RAW_MASK) != 0
    }

    /// Set IBI_DATA_BUF_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ibi_data_buf_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IBI_DATA_BUF_OVF_INT_RAW_MASK);
    }

    /// Clear IBI_DATA_BUF_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ibi_data_buf_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IBI_DATA_BUF_OVF_INT_RAW_MASK);
    }

    /// **RESP_BUF_OVF_INT_RAW** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn resp_buf_ovf_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RESP_BUF_OVF_INT_RAW_MASK) != 0
    }

    /// Set RESP_BUF_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_resp_buf_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RESP_BUF_OVF_INT_RAW_MASK);
    }

    /// Clear RESP_BUF_OVF_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_resp_buf_ovf_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RESP_BUF_OVF_INT_RAW_MASK);
    }

    /// **DETECT_START_INT_RAW** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn detect_start_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DETECT_START_INT_RAW_MASK) != 0
    }

    /// Set DETECT_START_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_detect_start_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::DETECT_START_INT_RAW_MASK);
    }

    /// Clear DETECT_START_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_detect_start_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::DETECT_START_INT_RAW_MASK);
    }

    /// **COMMAND_DONE_INT_RAW** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn command_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::COMMAND_DONE_INT_RAW_MASK) != 0
    }

    /// Set COMMAND_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_command_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::COMMAND_DONE_INT_RAW_MASK);
    }

    /// Clear COMMAND_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_command_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::COMMAND_DONE_INT_RAW_MASK);
    }

    /// **TRANSFER_COMPLETE_INT_RAW** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn transfer_complete_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TRANSFER_COMPLETE_INT_RAW_MASK) != 0
    }

    /// Set TRANSFER_COMPLETE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_transfer_complete_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TRANSFER_COMPLETE_INT_RAW_MASK);
    }

    /// Clear TRANSFER_COMPLETE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_transfer_complete_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TRANSFER_COMPLETE_INT_RAW_MASK);
    }

    /// **TRANSFER_ERR_INT_RAW** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn transfer_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TRANSFER_ERR_INT_RAW_MASK) != 0
    }

    /// Set TRANSFER_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_transfer_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TRANSFER_ERR_INT_RAW_MASK);
    }

    /// Clear TRANSFER_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_transfer_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TRANSFER_ERR_INT_RAW_MASK);
    }

    /// **NXT_CMD_REQ_ERR_INT_RAW** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn nxt_cmd_req_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::NXT_CMD_REQ_ERR_INT_RAW_MASK) != 0
    }

    /// Set NXT_CMD_REQ_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_nxt_cmd_req_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::NXT_CMD_REQ_ERR_INT_RAW_MASK);
    }

    /// Clear NXT_CMD_REQ_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_nxt_cmd_req_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::NXT_CMD_REQ_ERR_INT_RAW_MASK);
    }

    /// **RESP_READY_INT_RAW** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn resp_ready_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RESP_READY_INT_RAW_MASK) != 0
    }

    /// Set RESP_READY_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_resp_ready_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RESP_READY_INT_RAW_MASK);
    }

    /// Clear RESP_READY_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_resp_ready_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RESP_READY_INT_RAW_MASK);
    }

    /// **CMD_BUF_EMPTY_THLD_INT_RAW** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn cmd_buf_empty_thld_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CMD_BUF_EMPTY_THLD_INT_RAW_MASK) != 0
    }

    /// Set CMD_BUF_EMPTY_THLD_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cmd_buf_empty_thld_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CMD_BUF_EMPTY_THLD_INT_RAW_MASK);
    }

    /// Clear CMD_BUF_EMPTY_THLD_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cmd_buf_empty_thld_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CMD_BUF_EMPTY_THLD_INT_RAW_MASK);
    }

    /// **IBI_STATUS_THLD_INT_RAW** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_status_thld_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IBI_STATUS_THLD_INT_RAW_MASK) != 0
    }

    /// Set IBI_STATUS_THLD_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ibi_status_thld_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::IBI_STATUS_THLD_INT_RAW_MASK);
    }

    /// Clear IBI_STATUS_THLD_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ibi_status_thld_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::IBI_STATUS_THLD_INT_RAW_MASK);
    }

    /// **RX_DATA_BUF_THLD_INT_RAW** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn rx_data_buf_thld_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::RX_DATA_BUF_THLD_INT_RAW_MASK) != 0
    }

    /// Set RX_DATA_BUF_THLD_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_data_buf_thld_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_DATA_BUF_THLD_INT_RAW_MASK);
    }

    /// Clear RX_DATA_BUF_THLD_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_data_buf_thld_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_DATA_BUF_THLD_INT_RAW_MASK);
    }

    /// **TX_DATA_BUF_THLD_INT_RAW** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn tx_data_buf_thld_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TX_DATA_BUF_THLD_INT_RAW_MASK) != 0
    }

    /// Set TX_DATA_BUF_THLD_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_data_buf_thld_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_DATA_BUF_THLD_INT_RAW_MASK);
    }

    /// Clear TX_DATA_BUF_THLD_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_data_buf_thld_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_DATA_BUF_THLD_INT_RAW_MASK);
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

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000008 }
    }

    /// Read **CMD_CCC_MISMATCH_INT_RAW** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn cmd_ccc_mismatch_int_raw(&self) -> bool {
        (self.bits & IntRaw::CMD_CCC_MISMATCH_INT_RAW_MASK) != 0
    }

    /// Read **IBI_DETECT_INT_RAW** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_detect_int_raw(&self) -> bool {
        (self.bits & IntRaw::IBI_DETECT_INT_RAW_MASK) != 0
    }

    /// Read **IBI_HANDLE_DONE_INT_RAW** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_handle_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::IBI_HANDLE_DONE_INT_RAW_MASK) != 0
    }

    /// Read **IBI_STATUS_BUF_OVF_INT_RAW** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_status_buf_ovf_int_raw(&self) -> bool {
        (self.bits & IntRaw::IBI_STATUS_BUF_OVF_INT_RAW_MASK) != 0
    }

    /// Read **IBI_DATA_BUF_OVF_INT_RAW** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_data_buf_ovf_int_raw(&self) -> bool {
        (self.bits & IntRaw::IBI_DATA_BUF_OVF_INT_RAW_MASK) != 0
    }

    /// Read **RESP_BUF_OVF_INT_RAW** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn resp_buf_ovf_int_raw(&self) -> bool {
        (self.bits & IntRaw::RESP_BUF_OVF_INT_RAW_MASK) != 0
    }

    /// Read **DETECT_START_INT_RAW** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn detect_start_int_raw(&self) -> bool {
        (self.bits & IntRaw::DETECT_START_INT_RAW_MASK) != 0
    }

    /// Read **COMMAND_DONE_INT_RAW** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn command_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::COMMAND_DONE_INT_RAW_MASK) != 0
    }

    /// Read **TRANSFER_COMPLETE_INT_RAW** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn transfer_complete_int_raw(&self) -> bool {
        (self.bits & IntRaw::TRANSFER_COMPLETE_INT_RAW_MASK) != 0
    }

    /// Read **TRANSFER_ERR_INT_RAW** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn transfer_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::TRANSFER_ERR_INT_RAW_MASK) != 0
    }

    /// Read **NXT_CMD_REQ_ERR_INT_RAW** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn nxt_cmd_req_err_int_raw(&self) -> bool {
        (self.bits & IntRaw::NXT_CMD_REQ_ERR_INT_RAW_MASK) != 0
    }

    /// Read **RESP_READY_INT_RAW** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn resp_ready_int_raw(&self) -> bool {
        (self.bits & IntRaw::RESP_READY_INT_RAW_MASK) != 0
    }

    /// Read **CMD_BUF_EMPTY_THLD_INT_RAW** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn cmd_buf_empty_thld_int_raw(&self) -> bool {
        (self.bits & IntRaw::CMD_BUF_EMPTY_THLD_INT_RAW_MASK) != 0
    }

    /// Read **IBI_STATUS_THLD_INT_RAW** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_status_thld_int_raw(&self) -> bool {
        (self.bits & IntRaw::IBI_STATUS_THLD_INT_RAW_MASK) != 0
    }

    /// Read **RX_DATA_BUF_THLD_INT_RAW** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn rx_data_buf_thld_int_raw(&self) -> bool {
        (self.bits & IntRaw::RX_DATA_BUF_THLD_INT_RAW_MASK) != 0
    }

    /// Read **TX_DATA_BUF_THLD_INT_RAW** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn tx_data_buf_thld_int_raw(&self) -> bool {
        (self.bits & IntRaw::TX_DATA_BUF_THLD_INT_RAW_MASK) != 0
    }

    /// Set **CMD_CCC_MISMATCH_INT_RAW** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_cmd_ccc_mismatch_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CMD_CCC_MISMATCH_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CMD_CCC_MISMATCH_INT_RAW_MASK
            },
        }
    }

    /// Set **IBI_DETECT_INT_RAW** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ibi_detect_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::IBI_DETECT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::IBI_DETECT_INT_RAW_MASK
            },
        }
    }

    /// Set **IBI_HANDLE_DONE_INT_RAW** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ibi_handle_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::IBI_HANDLE_DONE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::IBI_HANDLE_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **IBI_STATUS_BUF_OVF_INT_RAW** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ibi_status_buf_ovf_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::IBI_STATUS_BUF_OVF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::IBI_STATUS_BUF_OVF_INT_RAW_MASK
            },
        }
    }

    /// Set **IBI_DATA_BUF_OVF_INT_RAW** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ibi_data_buf_ovf_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::IBI_DATA_BUF_OVF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::IBI_DATA_BUF_OVF_INT_RAW_MASK
            },
        }
    }

    /// Set **RESP_BUF_OVF_INT_RAW** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_resp_buf_ovf_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RESP_BUF_OVF_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RESP_BUF_OVF_INT_RAW_MASK
            },
        }
    }

    /// Set **DETECT_START_INT_RAW** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_detect_start_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DETECT_START_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DETECT_START_INT_RAW_MASK
            },
        }
    }

    /// Set **COMMAND_DONE_INT_RAW** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_command_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::COMMAND_DONE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::COMMAND_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **TRANSFER_COMPLETE_INT_RAW** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_transfer_complete_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TRANSFER_COMPLETE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TRANSFER_COMPLETE_INT_RAW_MASK
            },
        }
    }

    /// Set **TRANSFER_ERR_INT_RAW** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_transfer_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TRANSFER_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TRANSFER_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **NXT_CMD_REQ_ERR_INT_RAW** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_nxt_cmd_req_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::NXT_CMD_REQ_ERR_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::NXT_CMD_REQ_ERR_INT_RAW_MASK
            },
        }
    }

    /// Set **RESP_READY_INT_RAW** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_resp_ready_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RESP_READY_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RESP_READY_INT_RAW_MASK
            },
        }
    }

    /// Set **CMD_BUF_EMPTY_THLD_INT_RAW** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_cmd_buf_empty_thld_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::CMD_BUF_EMPTY_THLD_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::CMD_BUF_EMPTY_THLD_INT_RAW_MASK
            },
        }
    }

    /// Set **IBI_STATUS_THLD_INT_RAW** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ibi_status_thld_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::IBI_STATUS_THLD_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::IBI_STATUS_THLD_INT_RAW_MASK
            },
        }
    }

    /// Set **RX_DATA_BUF_THLD_INT_RAW** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_rx_data_buf_thld_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::RX_DATA_BUF_THLD_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::RX_DATA_BUF_THLD_INT_RAW_MASK
            },
        }
    }

    /// Set **TX_DATA_BUF_THLD_INT_RAW** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_tx_data_buf_thld_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TX_DATA_BUF_THLD_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TX_DATA_BUF_THLD_INT_RAW_MASK
            },
        }
    }
}

impl Default for IntRawVal {
    fn default() -> Self {
        Self::reset_val()
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
/// NA
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **CMD_CCC_MISMATCH_INT_ST** `[15]` (RO)
///
///   NA
///
/// - **IBI_DETECT_INT_ST** `[14]` (RO)
///
///   NA
///
/// - **IBI_HANDLE_DONE_INT_ST** `[13]` (RO)
///
///   NA
///
/// - **IBI_STATUS_BUF_OVF_INT_ST** `[12]` (RO)
///
///   NA
///
/// - **IBI_DATA_BUF_OVF_INT_ST** `[11]` (RO)
///
///   NA
///
/// - **RESP_BUF_OVF_INT_ST** `[10]` (RO)
///
///   NA
///
/// - **DETECT_START_INT_ST** `[9]` (RO)
///
///   NA
///
/// - **COMMAND_DONE_INT_ST** `[8]` (RO)
///
///   NA
///
/// - **TRANSFER_COMPLETE_INT_ST** `[7]` (RO)
///
///   NA
///
/// - **TRANSFER_ERR_INT_ST** `[6]` (RO)
///
///   This interrupt is generated if any error occurs during transfer. The error type will be specified in the response packet associated with the command (in ERR_STATUS field of RESPONSE_BUFFER_PORT register). This bit can be cleared by writing 1'h1.
///
/// - **NXT_CMD_REQ_ERR_INT_ST** `[5]` (RO)
///
///   This interrupt is generated if toc is 0(master will restart next command), but command buf is empty.
///
/// - **RESP_READY_INT_ST** `[4]` (RO)
///
///   This interrupt is generated when number of entries in response buffer is greater than or equal to threshold value specified by RESP_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in response buffer is less than threshold value.
///
/// - **CMD_BUF_EMPTY_THLD_INT_ST** `[3]` (RO)
///
///   This interrupt is generated when number of empty locations in command buffer is greater than or equal to threshold value specified by CMD_EMPTY_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in command buffer is less than threshold value.
///
/// - **IBI_STATUS_THLD_INT_ST** `[2]` (RO)
///
///   Only used in master mode. This interrupt is generated when number of entries in IBI buffer is greater than or equal to threshold value specified by IBI_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in IBI buffer is less than threshold value.
///
/// - **RX_DATA_BUF_THLD_INT_ST** `[1]` (RO)
///
///   This interrupt is generated when number of entries in receive buffer is greater than or equal to threshold value specified by RX_BUF_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in receive buffer is less than threshold value.
///
/// - **TX_DATA_BUF_THLD_INT_ST** `[0]` (RO)
///
///   This interrupt is generated when number of empty locations in transmit buffer is greater than or equal to threshold value specified by TX_EMPTY_BUS_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in transmit buffer is less than threshold value.
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

    /// **CMD_CCC_MISMATCH_INT_ST**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CMD_CCC_MISMATCH_INT_ST_OFFSET: u32 = 15;
    pub const CMD_CCC_MISMATCH_INT_ST_MASK: u32 = 0x00008000;
    pub const CMD_CCC_MISMATCH_INT_ST: u32 = 1 << 15;

    /// **IBI_DETECT_INT_ST**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const IBI_DETECT_INT_ST_OFFSET: u32 = 14;
    pub const IBI_DETECT_INT_ST_MASK: u32 = 0x00004000;
    pub const IBI_DETECT_INT_ST: u32 = 1 << 14;

    /// **IBI_HANDLE_DONE_INT_ST**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const IBI_HANDLE_DONE_INT_ST_OFFSET: u32 = 13;
    pub const IBI_HANDLE_DONE_INT_ST_MASK: u32 = 0x00002000;
    pub const IBI_HANDLE_DONE_INT_ST: u32 = 1 << 13;

    /// **IBI_STATUS_BUF_OVF_INT_ST**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const IBI_STATUS_BUF_OVF_INT_ST_OFFSET: u32 = 12;
    pub const IBI_STATUS_BUF_OVF_INT_ST_MASK: u32 = 0x00001000;
    pub const IBI_STATUS_BUF_OVF_INT_ST: u32 = 1 << 12;

    /// **IBI_DATA_BUF_OVF_INT_ST**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const IBI_DATA_BUF_OVF_INT_ST_OFFSET: u32 = 11;
    pub const IBI_DATA_BUF_OVF_INT_ST_MASK: u32 = 0x00000800;
    pub const IBI_DATA_BUF_OVF_INT_ST: u32 = 1 << 11;

    /// **RESP_BUF_OVF_INT_ST**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const RESP_BUF_OVF_INT_ST_OFFSET: u32 = 10;
    pub const RESP_BUF_OVF_INT_ST_MASK: u32 = 0x00000400;
    pub const RESP_BUF_OVF_INT_ST: u32 = 1 << 10;

    /// **DETECT_START_INT_ST**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DETECT_START_INT_ST_OFFSET: u32 = 9;
    pub const DETECT_START_INT_ST_MASK: u32 = 0x00000200;
    pub const DETECT_START_INT_ST: u32 = 1 << 9;

    /// **COMMAND_DONE_INT_ST**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const COMMAND_DONE_INT_ST_OFFSET: u32 = 8;
    pub const COMMAND_DONE_INT_ST_MASK: u32 = 0x00000100;
    pub const COMMAND_DONE_INT_ST: u32 = 1 << 8;

    /// **TRANSFER_COMPLETE_INT_ST**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TRANSFER_COMPLETE_INT_ST_OFFSET: u32 = 7;
    pub const TRANSFER_COMPLETE_INT_ST_MASK: u32 = 0x00000080;
    pub const TRANSFER_COMPLETE_INT_ST: u32 = 1 << 7;

    /// **TRANSFER_ERR_INT_ST**
    ///
    /// This interrupt is generated if any error occurs during transfer. The error type will be specified in the response packet associated with the command (in ERR_STATUS field of RESPONSE_BUFFER_PORT register). This bit can be cleared by writing 1'h1.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TRANSFER_ERR_INT_ST_OFFSET: u32 = 6;
    pub const TRANSFER_ERR_INT_ST_MASK: u32 = 0x00000040;
    pub const TRANSFER_ERR_INT_ST: u32 = 1 << 6;

    /// **NXT_CMD_REQ_ERR_INT_ST**
    ///
    /// This interrupt is generated if toc is 0(master will restart next command), but command buf is empty.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const NXT_CMD_REQ_ERR_INT_ST_OFFSET: u32 = 5;
    pub const NXT_CMD_REQ_ERR_INT_ST_MASK: u32 = 0x00000020;
    pub const NXT_CMD_REQ_ERR_INT_ST: u32 = 1 << 5;

    /// **RESP_READY_INT_ST**
    ///
    /// This interrupt is generated when number of entries in response buffer is greater than or equal to threshold value specified by RESP_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in response buffer is less than threshold value.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RESP_READY_INT_ST_OFFSET: u32 = 4;
    pub const RESP_READY_INT_ST_MASK: u32 = 0x00000010;
    pub const RESP_READY_INT_ST: u32 = 1 << 4;

    /// **CMD_BUF_EMPTY_THLD_INT_ST**
    ///
    /// This interrupt is generated when number of empty locations in command buffer is greater than or equal to threshold value specified by CMD_EMPTY_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in command buffer is less than threshold value.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CMD_BUF_EMPTY_THLD_INT_ST_OFFSET: u32 = 3;
    pub const CMD_BUF_EMPTY_THLD_INT_ST_MASK: u32 = 0x00000008;
    pub const CMD_BUF_EMPTY_THLD_INT_ST: u32 = 1 << 3;

    /// **IBI_STATUS_THLD_INT_ST**
    ///
    /// Only used in master mode. This interrupt is generated when number of entries in IBI buffer is greater than or equal to threshold value specified by IBI_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in IBI buffer is less than threshold value.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IBI_STATUS_THLD_INT_ST_OFFSET: u32 = 2;
    pub const IBI_STATUS_THLD_INT_ST_MASK: u32 = 0x00000004;
    pub const IBI_STATUS_THLD_INT_ST: u32 = 1 << 2;

    /// **RX_DATA_BUF_THLD_INT_ST**
    ///
    /// This interrupt is generated when number of entries in receive buffer is greater than or equal to threshold value specified by RX_BUF_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in receive buffer is less than threshold value.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_DATA_BUF_THLD_INT_ST_OFFSET: u32 = 1;
    pub const RX_DATA_BUF_THLD_INT_ST_MASK: u32 = 0x00000002;
    pub const RX_DATA_BUF_THLD_INT_ST: u32 = 1 << 1;

    /// **TX_DATA_BUF_THLD_INT_ST**
    ///
    /// This interrupt is generated when number of empty locations in transmit buffer is greater than or equal to threshold value specified by TX_EMPTY_BUS_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in transmit buffer is less than threshold value.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_DATA_BUF_THLD_INT_ST_OFFSET: u32 = 0;
    pub const TX_DATA_BUF_THLD_INT_ST_MASK: u32 = 0x00000001;
    pub const TX_DATA_BUF_THLD_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CMD_CCC_MISMATCH_INT_ST** `[15]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn cmd_ccc_mismatch_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CMD_CCC_MISMATCH_INT_ST_MASK) != 0
    }

    /// **IBI_DETECT_INT_ST** `[14]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_detect_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IBI_DETECT_INT_ST_MASK) != 0
    }

    /// **IBI_HANDLE_DONE_INT_ST** `[13]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_handle_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IBI_HANDLE_DONE_INT_ST_MASK) != 0
    }

    /// **IBI_STATUS_BUF_OVF_INT_ST** `[12]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_status_buf_ovf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IBI_STATUS_BUF_OVF_INT_ST_MASK) != 0
    }

    /// **IBI_DATA_BUF_OVF_INT_ST** `[11]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_data_buf_ovf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IBI_DATA_BUF_OVF_INT_ST_MASK) != 0
    }

    /// **RESP_BUF_OVF_INT_ST** `[10]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn resp_buf_ovf_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RESP_BUF_OVF_INT_ST_MASK) != 0
    }

    /// **DETECT_START_INT_ST** `[9]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn detect_start_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DETECT_START_INT_ST_MASK) != 0
    }

    /// **COMMAND_DONE_INT_ST** `[8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn command_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::COMMAND_DONE_INT_ST_MASK) != 0
    }

    /// **TRANSFER_COMPLETE_INT_ST** `[7]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn transfer_complete_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TRANSFER_COMPLETE_INT_ST_MASK) != 0
    }

    /// **TRANSFER_ERR_INT_ST** `[6]` (RO)
    ///
    /// This interrupt is generated if any error occurs during transfer. The error type will be specified in the response packet associated with the command (in ERR_STATUS field of RESPONSE_BUFFER_PORT register). This bit can be cleared by writing 1'h1.
    #[inline(always)]
    pub fn transfer_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TRANSFER_ERR_INT_ST_MASK) != 0
    }

    /// **NXT_CMD_REQ_ERR_INT_ST** `[5]` (RO)
    ///
    /// This interrupt is generated if toc is 0(master will restart next command), but command buf is empty.
    #[inline(always)]
    pub fn nxt_cmd_req_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::NXT_CMD_REQ_ERR_INT_ST_MASK) != 0
    }

    /// **RESP_READY_INT_ST** `[4]` (RO)
    ///
    /// This interrupt is generated when number of entries in response buffer is greater than or equal to threshold value specified by RESP_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in response buffer is less than threshold value.
    #[inline(always)]
    pub fn resp_ready_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RESP_READY_INT_ST_MASK) != 0
    }

    /// **CMD_BUF_EMPTY_THLD_INT_ST** `[3]` (RO)
    ///
    /// This interrupt is generated when number of empty locations in command buffer is greater than or equal to threshold value specified by CMD_EMPTY_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in command buffer is less than threshold value.
    #[inline(always)]
    pub fn cmd_buf_empty_thld_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CMD_BUF_EMPTY_THLD_INT_ST_MASK) != 0
    }

    /// **IBI_STATUS_THLD_INT_ST** `[2]` (RO)
    ///
    /// Only used in master mode. This interrupt is generated when number of entries in IBI buffer is greater than or equal to threshold value specified by IBI_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in IBI buffer is less than threshold value.
    #[inline(always)]
    pub fn ibi_status_thld_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IBI_STATUS_THLD_INT_ST_MASK) != 0
    }

    /// **RX_DATA_BUF_THLD_INT_ST** `[1]` (RO)
    ///
    /// This interrupt is generated when number of entries in receive buffer is greater than or equal to threshold value specified by RX_BUF_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in receive buffer is less than threshold value.
    #[inline(always)]
    pub fn rx_data_buf_thld_int_st(&self) -> bool {
        (self.value.read_bits() & Self::RX_DATA_BUF_THLD_INT_ST_MASK) != 0
    }

    /// **TX_DATA_BUF_THLD_INT_ST** `[0]` (RO)
    ///
    /// This interrupt is generated when number of empty locations in transmit buffer is greater than or equal to threshold value specified by TX_EMPTY_BUS_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in transmit buffer is less than threshold value.
    #[inline(always)]
    pub fn tx_data_buf_thld_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TX_DATA_BUF_THLD_INT_ST_MASK) != 0
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

    /// Read **CMD_CCC_MISMATCH_INT_ST** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn cmd_ccc_mismatch_int_st(&self) -> bool {
        (self.bits & IntSt::CMD_CCC_MISMATCH_INT_ST_MASK) != 0
    }

    /// Read **IBI_DETECT_INT_ST** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_detect_int_st(&self) -> bool {
        (self.bits & IntSt::IBI_DETECT_INT_ST_MASK) != 0
    }

    /// Read **IBI_HANDLE_DONE_INT_ST** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_handle_done_int_st(&self) -> bool {
        (self.bits & IntSt::IBI_HANDLE_DONE_INT_ST_MASK) != 0
    }

    /// Read **IBI_STATUS_BUF_OVF_INT_ST** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_status_buf_ovf_int_st(&self) -> bool {
        (self.bits & IntSt::IBI_STATUS_BUF_OVF_INT_ST_MASK) != 0
    }

    /// Read **IBI_DATA_BUF_OVF_INT_ST** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_data_buf_ovf_int_st(&self) -> bool {
        (self.bits & IntSt::IBI_DATA_BUF_OVF_INT_ST_MASK) != 0
    }

    /// Read **RESP_BUF_OVF_INT_ST** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn resp_buf_ovf_int_st(&self) -> bool {
        (self.bits & IntSt::RESP_BUF_OVF_INT_ST_MASK) != 0
    }

    /// Read **DETECT_START_INT_ST** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn detect_start_int_st(&self) -> bool {
        (self.bits & IntSt::DETECT_START_INT_ST_MASK) != 0
    }

    /// Read **COMMAND_DONE_INT_ST** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn command_done_int_st(&self) -> bool {
        (self.bits & IntSt::COMMAND_DONE_INT_ST_MASK) != 0
    }

    /// Read **TRANSFER_COMPLETE_INT_ST** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn transfer_complete_int_st(&self) -> bool {
        (self.bits & IntSt::TRANSFER_COMPLETE_INT_ST_MASK) != 0
    }

    /// Read **TRANSFER_ERR_INT_ST** `[6:6]`
    ///
    /// This interrupt is generated if any error occurs during transfer. The error type will be specified in the response packet associated with the command (in ERR_STATUS field of RESPONSE_BUFFER_PORT register). This bit can be cleared by writing 1'h1.
    #[inline(always)]
    pub const fn transfer_err_int_st(&self) -> bool {
        (self.bits & IntSt::TRANSFER_ERR_INT_ST_MASK) != 0
    }

    /// Read **NXT_CMD_REQ_ERR_INT_ST** `[5:5]`
    ///
    /// This interrupt is generated if toc is 0(master will restart next command), but command buf is empty.
    #[inline(always)]
    pub const fn nxt_cmd_req_err_int_st(&self) -> bool {
        (self.bits & IntSt::NXT_CMD_REQ_ERR_INT_ST_MASK) != 0
    }

    /// Read **RESP_READY_INT_ST** `[4:4]`
    ///
    /// This interrupt is generated when number of entries in response buffer is greater than or equal to threshold value specified by RESP_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in response buffer is less than threshold value.
    #[inline(always)]
    pub const fn resp_ready_int_st(&self) -> bool {
        (self.bits & IntSt::RESP_READY_INT_ST_MASK) != 0
    }

    /// Read **CMD_BUF_EMPTY_THLD_INT_ST** `[3:3]`
    ///
    /// This interrupt is generated when number of empty locations in command buffer is greater than or equal to threshold value specified by CMD_EMPTY_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in command buffer is less than threshold value.
    #[inline(always)]
    pub const fn cmd_buf_empty_thld_int_st(&self) -> bool {
        (self.bits & IntSt::CMD_BUF_EMPTY_THLD_INT_ST_MASK) != 0
    }

    /// Read **IBI_STATUS_THLD_INT_ST** `[2:2]`
    ///
    /// Only used in master mode. This interrupt is generated when number of entries in IBI buffer is greater than or equal to threshold value specified by IBI_BUF_THLD field in BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in IBI buffer is less than threshold value.
    #[inline(always)]
    pub const fn ibi_status_thld_int_st(&self) -> bool {
        (self.bits & IntSt::IBI_STATUS_THLD_INT_ST_MASK) != 0
    }

    /// Read **RX_DATA_BUF_THLD_INT_ST** `[1:1]`
    ///
    /// This interrupt is generated when number of entries in receive buffer is greater than or equal to threshold value specified by RX_BUF_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of entries in receive buffer is less than threshold value.
    #[inline(always)]
    pub const fn rx_data_buf_thld_int_st(&self) -> bool {
        (self.bits & IntSt::RX_DATA_BUF_THLD_INT_ST_MASK) != 0
    }

    /// Read **TX_DATA_BUF_THLD_INT_ST** `[0:0]`
    ///
    /// This interrupt is generated when number of empty locations in transmit buffer is greater than or equal to threshold value specified by TX_EMPTY_BUS_THLD field in DATA_BUFFER_THLD_CTRL register. This interrupt will be cleared automatically when number of empty locations in transmit buffer is less than threshold value.
    #[inline(always)]
    pub const fn tx_data_buf_thld_int_st(&self) -> bool {
        (self.bits & IntSt::TX_DATA_BUF_THLD_INT_ST_MASK) != 0
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

/// INT_ST_ENA Register
///
/// The Interrupt status will be updated in INTR_STATUS register if corresponding Status Enable bit set.
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **CMD_CCC_MISMATCH_INT_ENA** `[15]` (RW)
///
///   NA
///
/// - **IBI_DETECT_INT_ENA** `[14]` (RW)
///
///   NA
///
/// - **IBI_HANDLE_DONE_INT_ENA** `[13]` (RW)
///
///   NA
///
/// - **IBI_STATUS_BUF_OVF_INT_ENA** `[12]` (RW)
///
///   NA
///
/// - **IBI_DATA_BUF_OVF_INT_ENA** `[11]` (RW)
///
///   NA
///
/// - **RESP_BUF_OVF_INT_ENA** `[10]` (RW)
///
///   NA
///
/// - **DETECT_START_INT_ENA** `[9]` (RW)
///
///   NA
///
/// - **COMMAND_DONE_INT_ENA** `[8]` (RW)
///
///   NA
///
/// - **TRANSFER_COMPLETE_INT_ENA** `[7]` (RW)
///
///   NA
///
/// - **TRANSFER_ERR_INT_ENA** `[6]` (RW)
///
///   Transfer error status enable
///
/// - **NXT_CMD_REQ_ERR_INT_ENA** `[5]` (RW)
///
///   next command request error status enable
///
/// - **RESP_READY_INT_ENA** `[4]` (RW)
///
///   Response buffer ready status enable.
///
/// - **CMD_BUF_EMPTY_THLD_INT_ENA** `[3]` (RW)
///
///   Command buffer ready status enable.
///
/// - **IBI_STATUS_THLD_INT_ENA** `[2]` (RW)
///
///   Only used in master mode. IBI Buffer threshold status enable.
///
/// - **RX_DATA_BUF_THLD_INT_ENA** `[1]` (RW)
///
///   Receive Buffer threshold status enable.
///
/// - **TX_DATA_BUF_THLD_INT_ENA** `[0]` (RW)
///
///   Transmit Buffer threshold status enable.
#[repr(transparent)]
pub struct IntStEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntStEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntStEna {
    // ======== Field Constants ========

    /// **CMD_CCC_MISMATCH_INT_ENA**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const CMD_CCC_MISMATCH_INT_ENA_OFFSET: u32 = 15;
    pub const CMD_CCC_MISMATCH_INT_ENA_MASK: u32 = 0x00008000;
    pub const CMD_CCC_MISMATCH_INT_ENA: u32 = 1 << 15;

    /// **IBI_DETECT_INT_ENA**
    ///
    /// NA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const IBI_DETECT_INT_ENA_OFFSET: u32 = 14;
    pub const IBI_DETECT_INT_ENA_MASK: u32 = 0x00004000;
    pub const IBI_DETECT_INT_ENA: u32 = 1 << 14;

    /// **IBI_HANDLE_DONE_INT_ENA**
    ///
    /// NA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const IBI_HANDLE_DONE_INT_ENA_OFFSET: u32 = 13;
    pub const IBI_HANDLE_DONE_INT_ENA_MASK: u32 = 0x00002000;
    pub const IBI_HANDLE_DONE_INT_ENA: u32 = 1 << 13;

    /// **IBI_STATUS_BUF_OVF_INT_ENA**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const IBI_STATUS_BUF_OVF_INT_ENA_OFFSET: u32 = 12;
    pub const IBI_STATUS_BUF_OVF_INT_ENA_MASK: u32 = 0x00001000;
    pub const IBI_STATUS_BUF_OVF_INT_ENA: u32 = 1 << 12;

    /// **IBI_DATA_BUF_OVF_INT_ENA**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const IBI_DATA_BUF_OVF_INT_ENA_OFFSET: u32 = 11;
    pub const IBI_DATA_BUF_OVF_INT_ENA_MASK: u32 = 0x00000800;
    pub const IBI_DATA_BUF_OVF_INT_ENA: u32 = 1 << 11;

    /// **RESP_BUF_OVF_INT_ENA**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const RESP_BUF_OVF_INT_ENA_OFFSET: u32 = 10;
    pub const RESP_BUF_OVF_INT_ENA_MASK: u32 = 0x00000400;
    pub const RESP_BUF_OVF_INT_ENA: u32 = 1 << 10;

    /// **DETECT_START_INT_ENA**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DETECT_START_INT_ENA_OFFSET: u32 = 9;
    pub const DETECT_START_INT_ENA_MASK: u32 = 0x00000200;
    pub const DETECT_START_INT_ENA: u32 = 1 << 9;

    /// **COMMAND_DONE_INT_ENA**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const COMMAND_DONE_INT_ENA_OFFSET: u32 = 8;
    pub const COMMAND_DONE_INT_ENA_MASK: u32 = 0x00000100;
    pub const COMMAND_DONE_INT_ENA: u32 = 1 << 8;

    /// **TRANSFER_COMPLETE_INT_ENA**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const TRANSFER_COMPLETE_INT_ENA_OFFSET: u32 = 7;
    pub const TRANSFER_COMPLETE_INT_ENA_MASK: u32 = 0x00000080;
    pub const TRANSFER_COMPLETE_INT_ENA: u32 = 1 << 7;

    /// **TRANSFER_ERR_INT_ENA**
    ///
    /// Transfer error status enable
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TRANSFER_ERR_INT_ENA_OFFSET: u32 = 6;
    pub const TRANSFER_ERR_INT_ENA_MASK: u32 = 0x00000040;
    pub const TRANSFER_ERR_INT_ENA: u32 = 1 << 6;

    /// **NXT_CMD_REQ_ERR_INT_ENA**
    ///
    /// next command request error status enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const NXT_CMD_REQ_ERR_INT_ENA_OFFSET: u32 = 5;
    pub const NXT_CMD_REQ_ERR_INT_ENA_MASK: u32 = 0x00000020;
    pub const NXT_CMD_REQ_ERR_INT_ENA: u32 = 1 << 5;

    /// **RESP_READY_INT_ENA**
    ///
    /// Response buffer ready status enable.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RESP_READY_INT_ENA_OFFSET: u32 = 4;
    pub const RESP_READY_INT_ENA_MASK: u32 = 0x00000010;
    pub const RESP_READY_INT_ENA: u32 = 1 << 4;

    /// **CMD_BUF_EMPTY_THLD_INT_ENA**
    ///
    /// Command buffer ready status enable.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CMD_BUF_EMPTY_THLD_INT_ENA_OFFSET: u32 = 3;
    pub const CMD_BUF_EMPTY_THLD_INT_ENA_MASK: u32 = 0x00000008;
    pub const CMD_BUF_EMPTY_THLD_INT_ENA: u32 = 1 << 3;

    /// **IBI_STATUS_THLD_INT_ENA**
    ///
    /// Only used in master mode. IBI Buffer threshold status enable.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const IBI_STATUS_THLD_INT_ENA_OFFSET: u32 = 2;
    pub const IBI_STATUS_THLD_INT_ENA_MASK: u32 = 0x00000004;
    pub const IBI_STATUS_THLD_INT_ENA: u32 = 1 << 2;

    /// **RX_DATA_BUF_THLD_INT_ENA**
    ///
    /// Receive Buffer threshold status enable.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RX_DATA_BUF_THLD_INT_ENA_OFFSET: u32 = 1;
    pub const RX_DATA_BUF_THLD_INT_ENA_MASK: u32 = 0x00000002;
    pub const RX_DATA_BUF_THLD_INT_ENA: u32 = 1 << 1;

    /// **TX_DATA_BUF_THLD_INT_ENA**
    ///
    /// Transmit Buffer threshold status enable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TX_DATA_BUF_THLD_INT_ENA_OFFSET: u32 = 0;
    pub const TX_DATA_BUF_THLD_INT_ENA_MASK: u32 = 0x00000001;
    pub const TX_DATA_BUF_THLD_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CMD_CCC_MISMATCH_INT_ENA** `[15]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn cmd_ccc_mismatch_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CMD_CCC_MISMATCH_INT_ENA_MASK) != 0
    }

    /// Set CMD_CCC_MISMATCH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cmd_ccc_mismatch_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CMD_CCC_MISMATCH_INT_ENA_MASK);
    }

    /// Clear CMD_CCC_MISMATCH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cmd_ccc_mismatch_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CMD_CCC_MISMATCH_INT_ENA_MASK);
    }

    /// **IBI_DETECT_INT_ENA** `[14]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_detect_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IBI_DETECT_INT_ENA_MASK) != 0
    }

    /// Set IBI_DETECT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ibi_detect_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IBI_DETECT_INT_ENA_MASK);
    }

    /// Clear IBI_DETECT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ibi_detect_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IBI_DETECT_INT_ENA_MASK);
    }

    /// **IBI_HANDLE_DONE_INT_ENA** `[13]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_handle_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IBI_HANDLE_DONE_INT_ENA_MASK) != 0
    }

    /// Set IBI_HANDLE_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ibi_handle_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IBI_HANDLE_DONE_INT_ENA_MASK);
    }

    /// Clear IBI_HANDLE_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ibi_handle_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IBI_HANDLE_DONE_INT_ENA_MASK);
    }

    /// **IBI_STATUS_BUF_OVF_INT_ENA** `[12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_status_buf_ovf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IBI_STATUS_BUF_OVF_INT_ENA_MASK) != 0
    }

    /// Set IBI_STATUS_BUF_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ibi_status_buf_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IBI_STATUS_BUF_OVF_INT_ENA_MASK);
    }

    /// Clear IBI_STATUS_BUF_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ibi_status_buf_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IBI_STATUS_BUF_OVF_INT_ENA_MASK);
    }

    /// **IBI_DATA_BUF_OVF_INT_ENA** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_data_buf_ovf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IBI_DATA_BUF_OVF_INT_ENA_MASK) != 0
    }

    /// Set IBI_DATA_BUF_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ibi_data_buf_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IBI_DATA_BUF_OVF_INT_ENA_MASK);
    }

    /// Clear IBI_DATA_BUF_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ibi_data_buf_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IBI_DATA_BUF_OVF_INT_ENA_MASK);
    }

    /// **RESP_BUF_OVF_INT_ENA** `[10]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn resp_buf_ovf_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RESP_BUF_OVF_INT_ENA_MASK) != 0
    }

    /// Set RESP_BUF_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_resp_buf_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RESP_BUF_OVF_INT_ENA_MASK);
    }

    /// Clear RESP_BUF_OVF_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_resp_buf_ovf_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RESP_BUF_OVF_INT_ENA_MASK);
    }

    /// **DETECT_START_INT_ENA** `[9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn detect_start_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DETECT_START_INT_ENA_MASK) != 0
    }

    /// Set DETECT_START_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_detect_start_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::DETECT_START_INT_ENA_MASK);
    }

    /// Clear DETECT_START_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_detect_start_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::DETECT_START_INT_ENA_MASK);
    }

    /// **COMMAND_DONE_INT_ENA** `[8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn command_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::COMMAND_DONE_INT_ENA_MASK) != 0
    }

    /// Set COMMAND_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_command_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::COMMAND_DONE_INT_ENA_MASK);
    }

    /// Clear COMMAND_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_command_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::COMMAND_DONE_INT_ENA_MASK);
    }

    /// **TRANSFER_COMPLETE_INT_ENA** `[7]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn transfer_complete_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TRANSFER_COMPLETE_INT_ENA_MASK) != 0
    }

    /// Set TRANSFER_COMPLETE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_transfer_complete_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TRANSFER_COMPLETE_INT_ENA_MASK);
    }

    /// Clear TRANSFER_COMPLETE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_transfer_complete_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TRANSFER_COMPLETE_INT_ENA_MASK);
    }

    /// **TRANSFER_ERR_INT_ENA** `[6]` (RW)
    ///
    /// Transfer error status enable
    #[inline(always)]
    pub fn transfer_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TRANSFER_ERR_INT_ENA_MASK) != 0
    }

    /// Set TRANSFER_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_transfer_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TRANSFER_ERR_INT_ENA_MASK);
    }

    /// Clear TRANSFER_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_transfer_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TRANSFER_ERR_INT_ENA_MASK);
    }

    /// **NXT_CMD_REQ_ERR_INT_ENA** `[5]` (RW)
    ///
    /// next command request error status enable
    #[inline(always)]
    pub fn nxt_cmd_req_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::NXT_CMD_REQ_ERR_INT_ENA_MASK) != 0
    }

    /// Set NXT_CMD_REQ_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_nxt_cmd_req_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::NXT_CMD_REQ_ERR_INT_ENA_MASK);
    }

    /// Clear NXT_CMD_REQ_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_nxt_cmd_req_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::NXT_CMD_REQ_ERR_INT_ENA_MASK);
    }

    /// **RESP_READY_INT_ENA** `[4]` (RW)
    ///
    /// Response buffer ready status enable.
    #[inline(always)]
    pub fn resp_ready_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RESP_READY_INT_ENA_MASK) != 0
    }

    /// Set RESP_READY_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_resp_ready_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RESP_READY_INT_ENA_MASK);
    }

    /// Clear RESP_READY_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_resp_ready_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RESP_READY_INT_ENA_MASK);
    }

    /// **CMD_BUF_EMPTY_THLD_INT_ENA** `[3]` (RW)
    ///
    /// Command buffer ready status enable.
    #[inline(always)]
    pub fn cmd_buf_empty_thld_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CMD_BUF_EMPTY_THLD_INT_ENA_MASK) != 0
    }

    /// Set CMD_BUF_EMPTY_THLD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cmd_buf_empty_thld_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CMD_BUF_EMPTY_THLD_INT_ENA_MASK);
    }

    /// Clear CMD_BUF_EMPTY_THLD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cmd_buf_empty_thld_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CMD_BUF_EMPTY_THLD_INT_ENA_MASK);
    }

    /// **IBI_STATUS_THLD_INT_ENA** `[2]` (RW)
    ///
    /// Only used in master mode. IBI Buffer threshold status enable.
    #[inline(always)]
    pub fn ibi_status_thld_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IBI_STATUS_THLD_INT_ENA_MASK) != 0
    }

    /// Set IBI_STATUS_THLD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ibi_status_thld_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IBI_STATUS_THLD_INT_ENA_MASK);
    }

    /// Clear IBI_STATUS_THLD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ibi_status_thld_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IBI_STATUS_THLD_INT_ENA_MASK);
    }

    /// **RX_DATA_BUF_THLD_INT_ENA** `[1]` (RW)
    ///
    /// Receive Buffer threshold status enable.
    #[inline(always)]
    pub fn rx_data_buf_thld_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::RX_DATA_BUF_THLD_INT_ENA_MASK) != 0
    }

    /// Set RX_DATA_BUF_THLD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rx_data_buf_thld_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::RX_DATA_BUF_THLD_INT_ENA_MASK);
    }

    /// Clear RX_DATA_BUF_THLD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rx_data_buf_thld_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::RX_DATA_BUF_THLD_INT_ENA_MASK);
    }

    /// **TX_DATA_BUF_THLD_INT_ENA** `[0]` (RW)
    ///
    /// Transmit Buffer threshold status enable.
    #[inline(always)]
    pub fn tx_data_buf_thld_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TX_DATA_BUF_THLD_INT_ENA_MASK) != 0
    }

    /// Set TX_DATA_BUF_THLD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tx_data_buf_thld_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TX_DATA_BUF_THLD_INT_ENA_MASK);
    }

    /// Clear TX_DATA_BUF_THLD_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tx_data_buf_thld_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TX_DATA_BUF_THLD_INT_ENA_MASK);
    }
}

/// INT_ST_ENA value type for batch read/modify/write operations
///
/// Use `intstena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intstena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStEnaVal {
    bits: u32,
}

impl IntStEnaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CMD_CCC_MISMATCH_INT_ENA** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn cmd_ccc_mismatch_int_ena(&self) -> bool {
        (self.bits & IntStEna::CMD_CCC_MISMATCH_INT_ENA_MASK) != 0
    }

    /// Read **IBI_DETECT_INT_ENA** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_detect_int_ena(&self) -> bool {
        (self.bits & IntStEna::IBI_DETECT_INT_ENA_MASK) != 0
    }

    /// Read **IBI_HANDLE_DONE_INT_ENA** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_handle_done_int_ena(&self) -> bool {
        (self.bits & IntStEna::IBI_HANDLE_DONE_INT_ENA_MASK) != 0
    }

    /// Read **IBI_STATUS_BUF_OVF_INT_ENA** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_status_buf_ovf_int_ena(&self) -> bool {
        (self.bits & IntStEna::IBI_STATUS_BUF_OVF_INT_ENA_MASK) != 0
    }

    /// Read **IBI_DATA_BUF_OVF_INT_ENA** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_data_buf_ovf_int_ena(&self) -> bool {
        (self.bits & IntStEna::IBI_DATA_BUF_OVF_INT_ENA_MASK) != 0
    }

    /// Read **RESP_BUF_OVF_INT_ENA** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn resp_buf_ovf_int_ena(&self) -> bool {
        (self.bits & IntStEna::RESP_BUF_OVF_INT_ENA_MASK) != 0
    }

    /// Read **DETECT_START_INT_ENA** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn detect_start_int_ena(&self) -> bool {
        (self.bits & IntStEna::DETECT_START_INT_ENA_MASK) != 0
    }

    /// Read **COMMAND_DONE_INT_ENA** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn command_done_int_ena(&self) -> bool {
        (self.bits & IntStEna::COMMAND_DONE_INT_ENA_MASK) != 0
    }

    /// Read **TRANSFER_COMPLETE_INT_ENA** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn transfer_complete_int_ena(&self) -> bool {
        (self.bits & IntStEna::TRANSFER_COMPLETE_INT_ENA_MASK) != 0
    }

    /// Read **TRANSFER_ERR_INT_ENA** `[6:6]`
    ///
    /// Transfer error status enable
    #[inline(always)]
    pub const fn transfer_err_int_ena(&self) -> bool {
        (self.bits & IntStEna::TRANSFER_ERR_INT_ENA_MASK) != 0
    }

    /// Read **NXT_CMD_REQ_ERR_INT_ENA** `[5:5]`
    ///
    /// next command request error status enable
    #[inline(always)]
    pub const fn nxt_cmd_req_err_int_ena(&self) -> bool {
        (self.bits & IntStEna::NXT_CMD_REQ_ERR_INT_ENA_MASK) != 0
    }

    /// Read **RESP_READY_INT_ENA** `[4:4]`
    ///
    /// Response buffer ready status enable.
    #[inline(always)]
    pub const fn resp_ready_int_ena(&self) -> bool {
        (self.bits & IntStEna::RESP_READY_INT_ENA_MASK) != 0
    }

    /// Read **CMD_BUF_EMPTY_THLD_INT_ENA** `[3:3]`
    ///
    /// Command buffer ready status enable.
    #[inline(always)]
    pub const fn cmd_buf_empty_thld_int_ena(&self) -> bool {
        (self.bits & IntStEna::CMD_BUF_EMPTY_THLD_INT_ENA_MASK) != 0
    }

    /// Read **IBI_STATUS_THLD_INT_ENA** `[2:2]`
    ///
    /// Only used in master mode. IBI Buffer threshold status enable.
    #[inline(always)]
    pub const fn ibi_status_thld_int_ena(&self) -> bool {
        (self.bits & IntStEna::IBI_STATUS_THLD_INT_ENA_MASK) != 0
    }

    /// Read **RX_DATA_BUF_THLD_INT_ENA** `[1:1]`
    ///
    /// Receive Buffer threshold status enable.
    #[inline(always)]
    pub const fn rx_data_buf_thld_int_ena(&self) -> bool {
        (self.bits & IntStEna::RX_DATA_BUF_THLD_INT_ENA_MASK) != 0
    }

    /// Read **TX_DATA_BUF_THLD_INT_ENA** `[0:0]`
    ///
    /// Transmit Buffer threshold status enable.
    #[inline(always)]
    pub const fn tx_data_buf_thld_int_ena(&self) -> bool {
        (self.bits & IntStEna::TX_DATA_BUF_THLD_INT_ENA_MASK) != 0
    }

    /// Set **CMD_CCC_MISMATCH_INT_ENA** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_cmd_ccc_mismatch_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::CMD_CCC_MISMATCH_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::CMD_CCC_MISMATCH_INT_ENA_MASK
            },
        }
    }

    /// Set **IBI_DETECT_INT_ENA** `[14:14]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ibi_detect_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::IBI_DETECT_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::IBI_DETECT_INT_ENA_MASK
            },
        }
    }

    /// Set **IBI_HANDLE_DONE_INT_ENA** `[13:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ibi_handle_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::IBI_HANDLE_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::IBI_HANDLE_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **IBI_STATUS_BUF_OVF_INT_ENA** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ibi_status_buf_ovf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::IBI_STATUS_BUF_OVF_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::IBI_STATUS_BUF_OVF_INT_ENA_MASK
            },
        }
    }

    /// Set **IBI_DATA_BUF_OVF_INT_ENA** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_ibi_data_buf_ovf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::IBI_DATA_BUF_OVF_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::IBI_DATA_BUF_OVF_INT_ENA_MASK
            },
        }
    }

    /// Set **RESP_BUF_OVF_INT_ENA** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_resp_buf_ovf_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::RESP_BUF_OVF_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::RESP_BUF_OVF_INT_ENA_MASK
            },
        }
    }

    /// Set **DETECT_START_INT_ENA** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_detect_start_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::DETECT_START_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::DETECT_START_INT_ENA_MASK
            },
        }
    }

    /// Set **COMMAND_DONE_INT_ENA** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_command_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::COMMAND_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::COMMAND_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **TRANSFER_COMPLETE_INT_ENA** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_transfer_complete_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::TRANSFER_COMPLETE_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::TRANSFER_COMPLETE_INT_ENA_MASK
            },
        }
    }

    /// Set **TRANSFER_ERR_INT_ENA** `[6:6]`
    ///
    /// Transfer error status enable
    #[inline(always)]
    pub const fn with_transfer_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::TRANSFER_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::TRANSFER_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **NXT_CMD_REQ_ERR_INT_ENA** `[5:5]`
    ///
    /// next command request error status enable
    #[inline(always)]
    pub const fn with_nxt_cmd_req_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::NXT_CMD_REQ_ERR_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::NXT_CMD_REQ_ERR_INT_ENA_MASK
            },
        }
    }

    /// Set **RESP_READY_INT_ENA** `[4:4]`
    ///
    /// Response buffer ready status enable.
    #[inline(always)]
    pub const fn with_resp_ready_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::RESP_READY_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::RESP_READY_INT_ENA_MASK
            },
        }
    }

    /// Set **CMD_BUF_EMPTY_THLD_INT_ENA** `[3:3]`
    ///
    /// Command buffer ready status enable.
    #[inline(always)]
    pub const fn with_cmd_buf_empty_thld_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::CMD_BUF_EMPTY_THLD_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::CMD_BUF_EMPTY_THLD_INT_ENA_MASK
            },
        }
    }

    /// Set **IBI_STATUS_THLD_INT_ENA** `[2:2]`
    ///
    /// Only used in master mode. IBI Buffer threshold status enable.
    #[inline(always)]
    pub const fn with_ibi_status_thld_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::IBI_STATUS_THLD_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::IBI_STATUS_THLD_INT_ENA_MASK
            },
        }
    }

    /// Set **RX_DATA_BUF_THLD_INT_ENA** `[1:1]`
    ///
    /// Receive Buffer threshold status enable.
    #[inline(always)]
    pub const fn with_rx_data_buf_thld_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::RX_DATA_BUF_THLD_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::RX_DATA_BUF_THLD_INT_ENA_MASK
            },
        }
    }

    /// Set **TX_DATA_BUF_THLD_INT_ENA** `[0:0]`
    ///
    /// Transmit Buffer threshold status enable.
    #[inline(always)]
    pub const fn with_tx_data_buf_thld_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntStEna::TX_DATA_BUF_THLD_INT_ENA_MASK
            } else {
                self.bits & !IntStEna::TX_DATA_BUF_THLD_INT_ENA_MASK
            },
        }
    }
}

impl Default for IntStEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntStEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStEnaVal {
        IntStEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntStEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntStEnaVal) -> IntStEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RESET_CTRL Register
///
/// NA
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **REG_IBI_STATUS_BUF_RST** `[6]` (RW)
///
///   NA
///
/// - **REG_IBI_DATA_BUF_RST** `[5]` (RW)
///
///   NA
///
/// - **REG_RX_DATA_BUF_RST** `[4]` (RW)
///
///   NA
///
/// - **REG_TX_DATA_BUF_BUF_RST** `[3]` (RW)
///
///   NA
///
/// - **REG_RESP_BUF_RST** `[2]` (RW)
///
///   NA
///
/// - **REG_CMD_BUF_RST** `[1]` (RW)
///
///   NA
///
/// - **REG_CORE_SOFT_RST** `[0]` (WO)
///
///   NA
#[repr(transparent)]
pub struct ResetCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ResetCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ResetCtrl {
    // ======== Field Constants ========

    /// **REG_IBI_STATUS_BUF_RST**
    ///
    /// NA
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const REG_IBI_STATUS_BUF_RST_OFFSET: u32 = 6;
    pub const REG_IBI_STATUS_BUF_RST_MASK: u32 = 0x00000040;
    pub const REG_IBI_STATUS_BUF_RST: u32 = 1 << 6;

    /// **REG_IBI_DATA_BUF_RST**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_IBI_DATA_BUF_RST_OFFSET: u32 = 5;
    pub const REG_IBI_DATA_BUF_RST_MASK: u32 = 0x00000020;
    pub const REG_IBI_DATA_BUF_RST: u32 = 1 << 5;

    /// **REG_RX_DATA_BUF_RST**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_RX_DATA_BUF_RST_OFFSET: u32 = 4;
    pub const REG_RX_DATA_BUF_RST_MASK: u32 = 0x00000010;
    pub const REG_RX_DATA_BUF_RST: u32 = 1 << 4;

    /// **REG_TX_DATA_BUF_BUF_RST**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_TX_DATA_BUF_BUF_RST_OFFSET: u32 = 3;
    pub const REG_TX_DATA_BUF_BUF_RST_MASK: u32 = 0x00000008;
    pub const REG_TX_DATA_BUF_BUF_RST: u32 = 1 << 3;

    /// **REG_RESP_BUF_RST**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_RESP_BUF_RST_OFFSET: u32 = 2;
    pub const REG_RESP_BUF_RST_MASK: u32 = 0x00000004;
    pub const REG_RESP_BUF_RST: u32 = 1 << 2;

    /// **REG_CMD_BUF_RST**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_CMD_BUF_RST_OFFSET: u32 = 1;
    pub const REG_CMD_BUF_RST_MASK: u32 = 0x00000002;
    pub const REG_CMD_BUF_RST: u32 = 1 << 1;

    /// **REG_CORE_SOFT_RST**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_CORE_SOFT_RST_OFFSET: u32 = 0;
    pub const REG_CORE_SOFT_RST_MASK: u32 = 0x00000001;
    pub const REG_CORE_SOFT_RST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_IBI_STATUS_BUF_RST** `[6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_ibi_status_buf_rst(&self) -> bool {
        (self.value.read_bits() & Self::REG_IBI_STATUS_BUF_RST_MASK) != 0
    }

    /// Set REG_IBI_STATUS_BUF_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_ibi_status_buf_rst(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_IBI_STATUS_BUF_RST_MASK);
    }

    /// Clear REG_IBI_STATUS_BUF_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_ibi_status_buf_rst(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_IBI_STATUS_BUF_RST_MASK);
    }

    /// **REG_IBI_DATA_BUF_RST** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_ibi_data_buf_rst(&self) -> bool {
        (self.value.read_bits() & Self::REG_IBI_DATA_BUF_RST_MASK) != 0
    }

    /// Set REG_IBI_DATA_BUF_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_ibi_data_buf_rst(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_IBI_DATA_BUF_RST_MASK);
    }

    /// Clear REG_IBI_DATA_BUF_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_ibi_data_buf_rst(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_IBI_DATA_BUF_RST_MASK);
    }

    /// **REG_RX_DATA_BUF_RST** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_rx_data_buf_rst(&self) -> bool {
        (self.value.read_bits() & Self::REG_RX_DATA_BUF_RST_MASK) != 0
    }

    /// Set REG_RX_DATA_BUF_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_rx_data_buf_rst(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_RX_DATA_BUF_RST_MASK);
    }

    /// Clear REG_RX_DATA_BUF_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_rx_data_buf_rst(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_RX_DATA_BUF_RST_MASK);
    }

    /// **REG_TX_DATA_BUF_BUF_RST** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_tx_data_buf_buf_rst(&self) -> bool {
        (self.value.read_bits() & Self::REG_TX_DATA_BUF_BUF_RST_MASK) != 0
    }

    /// Set REG_TX_DATA_BUF_BUF_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_tx_data_buf_buf_rst(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_TX_DATA_BUF_BUF_RST_MASK);
    }

    /// Clear REG_TX_DATA_BUF_BUF_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_tx_data_buf_buf_rst(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_TX_DATA_BUF_BUF_RST_MASK);
    }

    /// **REG_RESP_BUF_RST** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_resp_buf_rst(&self) -> bool {
        (self.value.read_bits() & Self::REG_RESP_BUF_RST_MASK) != 0
    }

    /// Set REG_RESP_BUF_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_resp_buf_rst(&self) {
        self.value.modify_bits(|r| r | Self::REG_RESP_BUF_RST_MASK);
    }

    /// Clear REG_RESP_BUF_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_resp_buf_rst(&self) {
        self.value.modify_bits(|r| r & !Self::REG_RESP_BUF_RST_MASK);
    }

    /// **REG_CMD_BUF_RST** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_cmd_buf_rst(&self) -> bool {
        (self.value.read_bits() & Self::REG_CMD_BUF_RST_MASK) != 0
    }

    /// Set REG_CMD_BUF_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_cmd_buf_rst(&self) {
        self.value.modify_bits(|r| r | Self::REG_CMD_BUF_RST_MASK);
    }

    /// Clear REG_CMD_BUF_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_cmd_buf_rst(&self) {
        self.value.modify_bits(|r| r & !Self::REG_CMD_BUF_RST_MASK);
    }

    /// Trigger **REG_CORE_SOFT_RST** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_reg_core_soft_rst(&self) {
        self.value.modify_bits(|r| r | Self::REG_CORE_SOFT_RST_MASK);
    }
}

/// RESET_CTRL value type for batch read/modify/write operations
///
/// Use `resetctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `resetctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ResetCtrlVal {
    bits: u32,
}

impl ResetCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_IBI_STATUS_BUF_RST** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_ibi_status_buf_rst(&self) -> bool {
        (self.bits & ResetCtrl::REG_IBI_STATUS_BUF_RST_MASK) != 0
    }

    /// Read **REG_IBI_DATA_BUF_RST** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_ibi_data_buf_rst(&self) -> bool {
        (self.bits & ResetCtrl::REG_IBI_DATA_BUF_RST_MASK) != 0
    }

    /// Read **REG_RX_DATA_BUF_RST** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_rx_data_buf_rst(&self) -> bool {
        (self.bits & ResetCtrl::REG_RX_DATA_BUF_RST_MASK) != 0
    }

    /// Read **REG_TX_DATA_BUF_BUF_RST** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_tx_data_buf_buf_rst(&self) -> bool {
        (self.bits & ResetCtrl::REG_TX_DATA_BUF_BUF_RST_MASK) != 0
    }

    /// Read **REG_RESP_BUF_RST** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_resp_buf_rst(&self) -> bool {
        (self.bits & ResetCtrl::REG_RESP_BUF_RST_MASK) != 0
    }

    /// Read **REG_CMD_BUF_RST** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_cmd_buf_rst(&self) -> bool {
        (self.bits & ResetCtrl::REG_CMD_BUF_RST_MASK) != 0
    }

    /// Set **REG_IBI_STATUS_BUF_RST** `[6:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_ibi_status_buf_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetCtrl::REG_IBI_STATUS_BUF_RST_MASK
            } else {
                self.bits & !ResetCtrl::REG_IBI_STATUS_BUF_RST_MASK
            },
        }
    }

    /// Set **REG_IBI_DATA_BUF_RST** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_ibi_data_buf_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetCtrl::REG_IBI_DATA_BUF_RST_MASK
            } else {
                self.bits & !ResetCtrl::REG_IBI_DATA_BUF_RST_MASK
            },
        }
    }

    /// Set **REG_RX_DATA_BUF_RST** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_rx_data_buf_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetCtrl::REG_RX_DATA_BUF_RST_MASK
            } else {
                self.bits & !ResetCtrl::REG_RX_DATA_BUF_RST_MASK
            },
        }
    }

    /// Set **REG_TX_DATA_BUF_BUF_RST** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_tx_data_buf_buf_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetCtrl::REG_TX_DATA_BUF_BUF_RST_MASK
            } else {
                self.bits & !ResetCtrl::REG_TX_DATA_BUF_BUF_RST_MASK
            },
        }
    }

    /// Set **REG_RESP_BUF_RST** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_resp_buf_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetCtrl::REG_RESP_BUF_RST_MASK
            } else {
                self.bits & !ResetCtrl::REG_RESP_BUF_RST_MASK
            },
        }
    }

    /// Set **REG_CMD_BUF_RST** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_cmd_buf_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetCtrl::REG_CMD_BUF_RST_MASK
            } else {
                self.bits & !ResetCtrl::REG_CMD_BUF_RST_MASK
            },
        }
    }

    /// Set **REG_CORE_SOFT_RST** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_core_soft_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetCtrl::REG_CORE_SOFT_RST_MASK
            } else {
                self.bits & !ResetCtrl::REG_CORE_SOFT_RST_MASK
            },
        }
    }
}

impl Default for ResetCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ResetCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ResetCtrlVal {
        ResetCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ResetCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ResetCtrlVal) -> ResetCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BUFFER_STATUS_LEVEL Register
///
/// BUFFER_STATUS_LEVEL reflects the status level of Buffers in the controller.
///
/// Address offset: 0x0048
/// Reset value: 0x00000010
///
/// ## Fields
///
/// - **IBI_STATUS_BUF_CNT** `[27:24]` (RO)
///
///   IBI Buffer Status Count contains the number of IBI status entries in the IBI Buffer. This field is used in master mode.
///
/// - **IBI_DATA_BUF_CNT** `[19:16]` (RO)
///
///   IBI Buffer Level Value contains the number of valid entries in the IBI Buffer. This is field is used in master mode.
///
/// - **RESP_BUF_CNT** `[11:8]` (RO)
///
///   Response Buffer Level Value contains the number of valid data entries in the response buffer.
///
/// - **CMD_BUF_EMPTY_CNT** `[4:0]` (RO)
///
///   Command Buffer Empty Locations contains the number of empty locations in the command buffer.
#[repr(transparent)]
pub struct BufferStatusLevel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BufferStatusLevel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BufferStatusLevel {
    // ======== Field Constants ========

    /// **IBI_STATUS_BUF_CNT**
    ///
    /// IBI Buffer Status Count contains the number of IBI status entries in the IBI Buffer. This field is used in master mode.
    ///
    /// Bits `[27:24]`, mask `0x0f000000`
    pub const IBI_STATUS_BUF_CNT_OFFSET: u32 = 24;
    pub const IBI_STATUS_BUF_CNT_MASK: u32 = 0x0f000000;

    /// **IBI_DATA_BUF_CNT**
    ///
    /// IBI Buffer Level Value contains the number of valid entries in the IBI Buffer. This is field is used in master mode.
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const IBI_DATA_BUF_CNT_OFFSET: u32 = 16;
    pub const IBI_DATA_BUF_CNT_MASK: u32 = 0x000f0000;

    /// **RESP_BUF_CNT**
    ///
    /// Response Buffer Level Value contains the number of valid data entries in the response buffer.
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const RESP_BUF_CNT_OFFSET: u32 = 8;
    pub const RESP_BUF_CNT_MASK: u32 = 0x00000f00;

    /// **CMD_BUF_EMPTY_CNT**
    ///
    /// Command Buffer Empty Locations contains the number of empty locations in the command buffer.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const CMD_BUF_EMPTY_CNT_OFFSET: u32 = 0;
    pub const CMD_BUF_EMPTY_CNT_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **IBI_STATUS_BUF_CNT** `[27:24]` (RO)
    ///
    /// IBI Buffer Status Count contains the number of IBI status entries in the IBI Buffer. This field is used in master mode.
    #[inline(always)]
    pub fn ibi_status_buf_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::IBI_STATUS_BUF_CNT_MASK) >> Self::IBI_STATUS_BUF_CNT_OFFSET
    }

    /// **IBI_DATA_BUF_CNT** `[19:16]` (RO)
    ///
    /// IBI Buffer Level Value contains the number of valid entries in the IBI Buffer. This is field is used in master mode.
    #[inline(always)]
    pub fn ibi_data_buf_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::IBI_DATA_BUF_CNT_MASK) >> Self::IBI_DATA_BUF_CNT_OFFSET
    }

    /// **RESP_BUF_CNT** `[11:8]` (RO)
    ///
    /// Response Buffer Level Value contains the number of valid data entries in the response buffer.
    #[inline(always)]
    pub fn resp_buf_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::RESP_BUF_CNT_MASK) >> Self::RESP_BUF_CNT_OFFSET
    }

    /// **CMD_BUF_EMPTY_CNT** `[4:0]` (RO)
    ///
    /// Command Buffer Empty Locations contains the number of empty locations in the command buffer.
    #[inline(always)]
    pub fn cmd_buf_empty_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::CMD_BUF_EMPTY_CNT_MASK) >> Self::CMD_BUF_EMPTY_CNT_OFFSET
    }
}

/// BUFFER_STATUS_LEVEL value type for batch read/modify/write operations
///
/// Use `bufferstatuslevel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bufferstatuslevel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BufferStatusLevelVal {
    bits: u32,
}

impl BufferStatusLevelVal {
    /// Create from raw bits
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
        Self { bits: 0x00000010 }
    }

    /// Read **IBI_STATUS_BUF_CNT** `[27:24]`
    ///
    /// IBI Buffer Status Count contains the number of IBI status entries in the IBI Buffer. This field is used in master mode.
    #[inline(always)]
    pub const fn ibi_status_buf_cnt(&self) -> u32 {
        (self.bits & BufferStatusLevel::IBI_STATUS_BUF_CNT_MASK)
            >> BufferStatusLevel::IBI_STATUS_BUF_CNT_OFFSET
    }

    /// Read **IBI_DATA_BUF_CNT** `[19:16]`
    ///
    /// IBI Buffer Level Value contains the number of valid entries in the IBI Buffer. This is field is used in master mode.
    #[inline(always)]
    pub const fn ibi_data_buf_cnt(&self) -> u32 {
        (self.bits & BufferStatusLevel::IBI_DATA_BUF_CNT_MASK)
            >> BufferStatusLevel::IBI_DATA_BUF_CNT_OFFSET
    }

    /// Read **RESP_BUF_CNT** `[11:8]`
    ///
    /// Response Buffer Level Value contains the number of valid data entries in the response buffer.
    #[inline(always)]
    pub const fn resp_buf_cnt(&self) -> u32 {
        (self.bits & BufferStatusLevel::RESP_BUF_CNT_MASK) >> BufferStatusLevel::RESP_BUF_CNT_OFFSET
    }

    /// Read **CMD_BUF_EMPTY_CNT** `[4:0]`
    ///
    /// Command Buffer Empty Locations contains the number of empty locations in the command buffer.
    #[inline(always)]
    pub const fn cmd_buf_empty_cnt(&self) -> u32 {
        (self.bits & BufferStatusLevel::CMD_BUF_EMPTY_CNT_MASK)
            >> BufferStatusLevel::CMD_BUF_EMPTY_CNT_OFFSET
    }
}

impl Default for BufferStatusLevelVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BufferStatusLevel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BufferStatusLevelVal {
        BufferStatusLevelVal::from_bits(self.value.read_bits())
    }
}

/// DATA_BUFFER_STATUS_LEVEL Register
///
/// DATA_BUFFER_STATUS_LEVEL reflects the status level of the Buffers in the controller.
///
/// Address offset: 0x004c
/// Reset value: 0x00000020
///
/// ## Fields
///
/// - **RX_DATA_BUF_CNT** `[21:16]` (RO)
///
///   Receive Buffer Level value contains the number of valid data entries in the receive buffer.
///
/// - **TX_DATA_BUF_EMPTY_CNT** `[5:0]` (RO)
///
///   Transmit Buffer Empty Level Value contains the number of empty locations in the transmit Buffer.
#[repr(transparent)]
pub struct DataBufferStatusLevel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DataBufferStatusLevel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DataBufferStatusLevel {
    // ======== Field Constants ========

    /// **RX_DATA_BUF_CNT**
    ///
    /// Receive Buffer Level value contains the number of valid data entries in the receive buffer.
    ///
    /// Bits `[21:16]`, mask `0x003f0000`
    pub const RX_DATA_BUF_CNT_OFFSET: u32 = 16;
    pub const RX_DATA_BUF_CNT_MASK: u32 = 0x003f0000;

    /// **TX_DATA_BUF_EMPTY_CNT**
    ///
    /// Transmit Buffer Empty Level Value contains the number of empty locations in the transmit Buffer.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const TX_DATA_BUF_EMPTY_CNT_OFFSET: u32 = 0;
    pub const TX_DATA_BUF_EMPTY_CNT_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **RX_DATA_BUF_CNT** `[21:16]` (RO)
    ///
    /// Receive Buffer Level value contains the number of valid data entries in the receive buffer.
    #[inline(always)]
    pub fn rx_data_buf_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::RX_DATA_BUF_CNT_MASK) >> Self::RX_DATA_BUF_CNT_OFFSET
    }

    /// **TX_DATA_BUF_EMPTY_CNT** `[5:0]` (RO)
    ///
    /// Transmit Buffer Empty Level Value contains the number of empty locations in the transmit Buffer.
    #[inline(always)]
    pub fn tx_data_buf_empty_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::TX_DATA_BUF_EMPTY_CNT_MASK)
            >> Self::TX_DATA_BUF_EMPTY_CNT_OFFSET
    }
}

/// DATA_BUFFER_STATUS_LEVEL value type for batch read/modify/write operations
///
/// Use `databufferstatuslevel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `databufferstatuslevel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DataBufferStatusLevelVal {
    bits: u32,
}

impl DataBufferStatusLevelVal {
    /// Create from raw bits
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
        Self { bits: 0x00000020 }
    }

    /// Read **RX_DATA_BUF_CNT** `[21:16]`
    ///
    /// Receive Buffer Level value contains the number of valid data entries in the receive buffer.
    #[inline(always)]
    pub const fn rx_data_buf_cnt(&self) -> u32 {
        (self.bits & DataBufferStatusLevel::RX_DATA_BUF_CNT_MASK)
            >> DataBufferStatusLevel::RX_DATA_BUF_CNT_OFFSET
    }

    /// Read **TX_DATA_BUF_EMPTY_CNT** `[5:0]`
    ///
    /// Transmit Buffer Empty Level Value contains the number of empty locations in the transmit Buffer.
    #[inline(always)]
    pub const fn tx_data_buf_empty_cnt(&self) -> u32 {
        (self.bits & DataBufferStatusLevel::TX_DATA_BUF_EMPTY_CNT_MASK)
            >> DataBufferStatusLevel::TX_DATA_BUF_EMPTY_CNT_OFFSET
    }
}

impl Default for DataBufferStatusLevelVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DataBufferStatusLevel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DataBufferStatusLevelVal {
        DataBufferStatusLevelVal::from_bits(self.value.read_bits())
    }
}

/// PRESENT_STATE0 Register
///
/// NA
///
/// Address offset: 0x0050
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **MAIN_FSM_STATE** `[31:29]` (RO)
///
///   NA
///
/// - **DAA_MODE_FSM_STATE** `[28:26]` (RO)
///
///   Reflects whether the Master Controller is in IDLE or not. This bit will be set when all the buffer(Command, Response, IBI, Transmit, Receive) are empty along with the Master State machine is in idle state. 0X0: not in idle 0x1: in idle
///
/// - **SDR_MODE_FSM_STATE** `[25:22]` (RO)
///
///   NA
///
/// - **I2C_MODE_FSM_STATE** `[21:19]` (RO)
///
///   NA
///
/// - **IBI_EV_HANDLE_FSM_STATE** `[18:16]` (RO)
///
///   NA
///
/// - **SCL_GEN_FSM_STATE** `[15:13]` (RO)
///
///   NA
///
/// - **CMD_TID** `[12:9]` (RO)
///
///   NA
///
/// - **BUS_FREE** `[3]` (RO)
///
///   NA
///
/// - **BUS_BUSY** `[2]` (RO)
///
///   NA
///
/// - **SCL_LVL** `[1]` (RO)
///
///   This bit is used to check the SDA line level to recover from error and for debugging. This bit reflects the value of synchronized sda_in_a.
///
/// - **SDA_LVL** `[0]` (RO)
///
///   This bit is used to check the SCL line level to recover from error and for debugging. This bit reflects the value of synchronized scl_in_a.
#[repr(transparent)]
pub struct PresentState0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PresentState0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PresentState0 {
    // ======== Field Constants ========

    /// **MAIN_FSM_STATE**
    ///
    /// NA
    ///
    /// Bits `[31:29]`, mask `0xe0000000`
    pub const MAIN_FSM_STATE_OFFSET: u32 = 29;
    pub const MAIN_FSM_STATE_MASK: u32 = 0xe0000000;

    /// **DAA_MODE_FSM_STATE**
    ///
    /// Reflects whether the Master Controller is in IDLE or not. This bit will be set when all the buffer(Command, Response, IBI, Transmit, Receive) are empty along with the Master State machine is in idle state. 0X0: not in idle 0x1: in idle
    ///
    /// Bits `[28:26]`, mask `0x1c000000`
    pub const DAA_MODE_FSM_STATE_OFFSET: u32 = 26;
    pub const DAA_MODE_FSM_STATE_MASK: u32 = 0x1c000000;

    /// **SDR_MODE_FSM_STATE**
    ///
    /// NA
    ///
    /// Bits `[25:22]`, mask `0x03c00000`
    pub const SDR_MODE_FSM_STATE_OFFSET: u32 = 22;
    pub const SDR_MODE_FSM_STATE_MASK: u32 = 0x03c00000;

    /// **I2C_MODE_FSM_STATE**
    ///
    /// NA
    ///
    /// Bits `[21:19]`, mask `0x00380000`
    pub const I2C_MODE_FSM_STATE_OFFSET: u32 = 19;
    pub const I2C_MODE_FSM_STATE_MASK: u32 = 0x00380000;

    /// **IBI_EV_HANDLE_FSM_STATE**
    ///
    /// NA
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const IBI_EV_HANDLE_FSM_STATE_OFFSET: u32 = 16;
    pub const IBI_EV_HANDLE_FSM_STATE_MASK: u32 = 0x00070000;

    /// **SCL_GEN_FSM_STATE**
    ///
    /// NA
    ///
    /// Bits `[15:13]`, mask `0x0000e000`
    pub const SCL_GEN_FSM_STATE_OFFSET: u32 = 13;
    pub const SCL_GEN_FSM_STATE_MASK: u32 = 0x0000e000;

    /// **CMD_TID**
    ///
    /// NA
    ///
    /// Bits `[12:9]`, mask `0x00001e00`
    pub const CMD_TID_OFFSET: u32 = 9;
    pub const CMD_TID_MASK: u32 = 0x00001e00;

    /// **BUS_FREE**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const BUS_FREE_OFFSET: u32 = 3;
    pub const BUS_FREE_MASK: u32 = 0x00000008;
    pub const BUS_FREE: u32 = 1 << 3;

    /// **BUS_BUSY**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const BUS_BUSY_OFFSET: u32 = 2;
    pub const BUS_BUSY_MASK: u32 = 0x00000004;
    pub const BUS_BUSY: u32 = 1 << 2;

    /// **SCL_LVL**
    ///
    /// This bit is used to check the SDA line level to recover from error and for debugging. This bit reflects the value of synchronized sda_in_a.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SCL_LVL_OFFSET: u32 = 1;
    pub const SCL_LVL_MASK: u32 = 0x00000002;
    pub const SCL_LVL: u32 = 1 << 1;

    /// **SDA_LVL**
    ///
    /// This bit is used to check the SCL line level to recover from error and for debugging. This bit reflects the value of synchronized scl_in_a.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SDA_LVL_OFFSET: u32 = 0;
    pub const SDA_LVL_MASK: u32 = 0x00000001;
    pub const SDA_LVL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MAIN_FSM_STATE** `[31:29]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn main_fsm_state(&self) -> u32 {
        (self.value.read_bits() & Self::MAIN_FSM_STATE_MASK) >> Self::MAIN_FSM_STATE_OFFSET
    }

    /// **DAA_MODE_FSM_STATE** `[28:26]` (RO)
    ///
    /// Reflects whether the Master Controller is in IDLE or not. This bit will be set when all the buffer(Command, Response, IBI, Transmit, Receive) are empty along with the Master State machine is in idle state. 0X0: not in idle 0x1: in idle
    #[inline(always)]
    pub fn daa_mode_fsm_state(&self) -> u32 {
        (self.value.read_bits() & Self::DAA_MODE_FSM_STATE_MASK) >> Self::DAA_MODE_FSM_STATE_OFFSET
    }

    /// **SDR_MODE_FSM_STATE** `[25:22]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn sdr_mode_fsm_state(&self) -> u32 {
        (self.value.read_bits() & Self::SDR_MODE_FSM_STATE_MASK) >> Self::SDR_MODE_FSM_STATE_OFFSET
    }

    /// **I2C_MODE_FSM_STATE** `[21:19]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn i2c_mode_fsm_state(&self) -> u32 {
        (self.value.read_bits() & Self::I2C_MODE_FSM_STATE_MASK) >> Self::I2C_MODE_FSM_STATE_OFFSET
    }

    /// **IBI_EV_HANDLE_FSM_STATE** `[18:16]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn ibi_ev_handle_fsm_state(&self) -> u32 {
        (self.value.read_bits() & Self::IBI_EV_HANDLE_FSM_STATE_MASK)
            >> Self::IBI_EV_HANDLE_FSM_STATE_OFFSET
    }

    /// **SCL_GEN_FSM_STATE** `[15:13]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn scl_gen_fsm_state(&self) -> u32 {
        (self.value.read_bits() & Self::SCL_GEN_FSM_STATE_MASK) >> Self::SCL_GEN_FSM_STATE_OFFSET
    }

    /// **CMD_TID** `[12:9]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn cmd_tid(&self) -> u32 {
        (self.value.read_bits() & Self::CMD_TID_MASK) >> Self::CMD_TID_OFFSET
    }

    /// **BUS_FREE** `[3]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn bus_free(&self) -> bool {
        (self.value.read_bits() & Self::BUS_FREE_MASK) != 0
    }

    /// **BUS_BUSY** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn bus_busy(&self) -> bool {
        (self.value.read_bits() & Self::BUS_BUSY_MASK) != 0
    }

    /// **SCL_LVL** `[1]` (RO)
    ///
    /// This bit is used to check the SDA line level to recover from error and for debugging. This bit reflects the value of synchronized sda_in_a.
    #[inline(always)]
    pub fn scl_lvl(&self) -> bool {
        (self.value.read_bits() & Self::SCL_LVL_MASK) != 0
    }

    /// **SDA_LVL** `[0]` (RO)
    ///
    /// This bit is used to check the SCL line level to recover from error and for debugging. This bit reflects the value of synchronized scl_in_a.
    #[inline(always)]
    pub fn sda_lvl(&self) -> bool {
        (self.value.read_bits() & Self::SDA_LVL_MASK) != 0
    }
}

/// PRESENT_STATE0 value type for batch read/modify/write operations
///
/// Use `presentstate0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `presentstate0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PresentState0Val {
    bits: u32,
}

impl PresentState0Val {
    /// Create from raw bits
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

    /// Read **MAIN_FSM_STATE** `[31:29]`
    ///
    /// NA
    #[inline(always)]
    pub const fn main_fsm_state(&self) -> u32 {
        (self.bits & PresentState0::MAIN_FSM_STATE_MASK) >> PresentState0::MAIN_FSM_STATE_OFFSET
    }

    /// Read **DAA_MODE_FSM_STATE** `[28:26]`
    ///
    /// Reflects whether the Master Controller is in IDLE or not. This bit will be set when all the buffer(Command, Response, IBI, Transmit, Receive) are empty along with the Master State machine is in idle state. 0X0: not in idle 0x1: in idle
    #[inline(always)]
    pub const fn daa_mode_fsm_state(&self) -> u32 {
        (self.bits & PresentState0::DAA_MODE_FSM_STATE_MASK)
            >> PresentState0::DAA_MODE_FSM_STATE_OFFSET
    }

    /// Read **SDR_MODE_FSM_STATE** `[25:22]`
    ///
    /// NA
    #[inline(always)]
    pub const fn sdr_mode_fsm_state(&self) -> u32 {
        (self.bits & PresentState0::SDR_MODE_FSM_STATE_MASK)
            >> PresentState0::SDR_MODE_FSM_STATE_OFFSET
    }

    /// Read **I2C_MODE_FSM_STATE** `[21:19]`
    ///
    /// NA
    #[inline(always)]
    pub const fn i2c_mode_fsm_state(&self) -> u32 {
        (self.bits & PresentState0::I2C_MODE_FSM_STATE_MASK)
            >> PresentState0::I2C_MODE_FSM_STATE_OFFSET
    }

    /// Read **IBI_EV_HANDLE_FSM_STATE** `[18:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn ibi_ev_handle_fsm_state(&self) -> u32 {
        (self.bits & PresentState0::IBI_EV_HANDLE_FSM_STATE_MASK)
            >> PresentState0::IBI_EV_HANDLE_FSM_STATE_OFFSET
    }

    /// Read **SCL_GEN_FSM_STATE** `[15:13]`
    ///
    /// NA
    #[inline(always)]
    pub const fn scl_gen_fsm_state(&self) -> u32 {
        (self.bits & PresentState0::SCL_GEN_FSM_STATE_MASK)
            >> PresentState0::SCL_GEN_FSM_STATE_OFFSET
    }

    /// Read **CMD_TID** `[12:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn cmd_tid(&self) -> u32 {
        (self.bits & PresentState0::CMD_TID_MASK) >> PresentState0::CMD_TID_OFFSET
    }

    /// Read **BUS_FREE** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn bus_free(&self) -> bool {
        (self.bits & PresentState0::BUS_FREE_MASK) != 0
    }

    /// Read **BUS_BUSY** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn bus_busy(&self) -> bool {
        (self.bits & PresentState0::BUS_BUSY_MASK) != 0
    }

    /// Read **SCL_LVL** `[1:1]`
    ///
    /// This bit is used to check the SDA line level to recover from error and for debugging. This bit reflects the value of synchronized sda_in_a.
    #[inline(always)]
    pub const fn scl_lvl(&self) -> bool {
        (self.bits & PresentState0::SCL_LVL_MASK) != 0
    }

    /// Read **SDA_LVL** `[0:0]`
    ///
    /// This bit is used to check the SCL line level to recover from error and for debugging. This bit reflects the value of synchronized scl_in_a.
    #[inline(always)]
    pub const fn sda_lvl(&self) -> bool {
        (self.bits & PresentState0::SDA_LVL_MASK) != 0
    }
}

impl Default for PresentState0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PresentState0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PresentState0Val {
        PresentState0Val::from_bits(self.value.read_bits())
    }
}

/// PRESENT_STATE1 Register
///
/// NA
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **DATA_BYTE_CNT** `[15:0]` (RO)
///
///   Present transfer data byte cnt: tx data byte cnt if write rx data byte cnt if read ibi data byte cnt if IBI handle.
#[repr(transparent)]
pub struct PresentState1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PresentState1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PresentState1 {
    // ======== Field Constants ========

    /// **DATA_BYTE_CNT**
    ///
    /// Present transfer data byte cnt: tx data byte cnt if write rx data byte cnt if read ibi data byte cnt if IBI handle.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const DATA_BYTE_CNT_OFFSET: u32 = 0;
    pub const DATA_BYTE_CNT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **DATA_BYTE_CNT** `[15:0]` (RO)
    ///
    /// Present transfer data byte cnt: tx data byte cnt if write rx data byte cnt if read ibi data byte cnt if IBI handle.
    #[inline(always)]
    pub fn data_byte_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::DATA_BYTE_CNT_MASK) >> Self::DATA_BYTE_CNT_OFFSET
    }
}

/// PRESENT_STATE1 value type for batch read/modify/write operations
///
/// Use `presentstate1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `presentstate1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PresentState1Val {
    bits: u32,
}

impl PresentState1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DATA_BYTE_CNT** `[15:0]`
    ///
    /// Present transfer data byte cnt: tx data byte cnt if write rx data byte cnt if read ibi data byte cnt if IBI handle.
    #[inline(always)]
    pub const fn data_byte_cnt(&self) -> u32 {
        (self.bits & PresentState1::DATA_BYTE_CNT_MASK) >> PresentState1::DATA_BYTE_CNT_OFFSET
    }
}

impl Default for PresentState1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PresentState1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PresentState1Val {
        PresentState1Val::from_bits(self.value.read_bits())
    }
}

/// DEVICE_TABLE Register
///
/// Pointer for Device Address Table
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **PRESENT_DAT_INDEX** `[15:12]` (RO)
///
///   NA
///
/// - **PRESENT_DCT_INDEX** `[11:8]` (RO)
///
///   NA
///
/// - **REG_DAT_DAA_INIT_INDEX** `[7:4]` (RW)
///
///   NA
///
/// - **REG_DCT_DAA_INIT_INDEX** `[3:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct DeviceTable {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DeviceTable {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DeviceTable {
    // ======== Field Constants ========

    /// **PRESENT_DAT_INDEX**
    ///
    /// NA
    ///
    /// Bits `[15:12]`, mask `0x0000f000`
    pub const PRESENT_DAT_INDEX_OFFSET: u32 = 12;
    pub const PRESENT_DAT_INDEX_MASK: u32 = 0x0000f000;

    /// **PRESENT_DCT_INDEX**
    ///
    /// NA
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const PRESENT_DCT_INDEX_OFFSET: u32 = 8;
    pub const PRESENT_DCT_INDEX_MASK: u32 = 0x00000f00;

    /// **REG_DAT_DAA_INIT_INDEX**
    ///
    /// NA
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const REG_DAT_DAA_INIT_INDEX_OFFSET: u32 = 4;
    pub const REG_DAT_DAA_INIT_INDEX_MASK: u32 = 0x000000f0;

    /// **REG_DCT_DAA_INIT_INDEX**
    ///
    /// Reserved
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const REG_DCT_DAA_INIT_INDEX_OFFSET: u32 = 0;
    pub const REG_DCT_DAA_INIT_INDEX_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **PRESENT_DAT_INDEX** `[15:12]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn present_dat_index(&self) -> u32 {
        (self.value.read_bits() & Self::PRESENT_DAT_INDEX_MASK) >> Self::PRESENT_DAT_INDEX_OFFSET
    }

    /// **PRESENT_DCT_INDEX** `[11:8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn present_dct_index(&self) -> u32 {
        (self.value.read_bits() & Self::PRESENT_DCT_INDEX_MASK) >> Self::PRESENT_DCT_INDEX_OFFSET
    }

    /// **REG_DAT_DAA_INIT_INDEX** `[7:4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_dat_daa_init_index(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DAT_DAA_INIT_INDEX_MASK)
            >> Self::REG_DAT_DAA_INIT_INDEX_OFFSET
    }

    /// Write **REG_DAT_DAA_INIT_INDEX** `[7:4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_dat_daa_init_index(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DAT_DAA_INIT_INDEX_MASK)
                | ((value << Self::REG_DAT_DAA_INIT_INDEX_OFFSET)
                    & Self::REG_DAT_DAA_INIT_INDEX_MASK)
        });
    }

    /// **REG_DCT_DAA_INIT_INDEX** `[3:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_dct_daa_init_index(&self) -> u32 {
        (self.value.read_bits() & Self::REG_DCT_DAA_INIT_INDEX_MASK)
            >> Self::REG_DCT_DAA_INIT_INDEX_OFFSET
    }

    /// Write **REG_DCT_DAA_INIT_INDEX** `[3:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_dct_daa_init_index(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_DCT_DAA_INIT_INDEX_MASK)
                | ((value << Self::REG_DCT_DAA_INIT_INDEX_OFFSET)
                    & Self::REG_DCT_DAA_INIT_INDEX_MASK)
        });
    }
}

/// DEVICE_TABLE value type for batch read/modify/write operations
///
/// Use `devicetable.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `devicetable.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DeviceTableVal {
    bits: u32,
}

impl DeviceTableVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PRESENT_DAT_INDEX** `[15:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn present_dat_index(&self) -> u32 {
        (self.bits & DeviceTable::PRESENT_DAT_INDEX_MASK) >> DeviceTable::PRESENT_DAT_INDEX_OFFSET
    }

    /// Read **PRESENT_DCT_INDEX** `[11:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn present_dct_index(&self) -> u32 {
        (self.bits & DeviceTable::PRESENT_DCT_INDEX_MASK) >> DeviceTable::PRESENT_DCT_INDEX_OFFSET
    }

    /// Read **REG_DAT_DAA_INIT_INDEX** `[7:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_dat_daa_init_index(&self) -> u32 {
        (self.bits & DeviceTable::REG_DAT_DAA_INIT_INDEX_MASK)
            >> DeviceTable::REG_DAT_DAA_INIT_INDEX_OFFSET
    }

    /// Read **REG_DCT_DAA_INIT_INDEX** `[3:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_dct_daa_init_index(&self) -> u32 {
        (self.bits & DeviceTable::REG_DCT_DAA_INIT_INDEX_MASK)
            >> DeviceTable::REG_DCT_DAA_INIT_INDEX_OFFSET
    }

    /// Set **REG_DAT_DAA_INIT_INDEX** `[7:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_dat_daa_init_index(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DeviceTable::REG_DAT_DAA_INIT_INDEX_MASK)
                | ((value << DeviceTable::REG_DAT_DAA_INIT_INDEX_OFFSET)
                    & DeviceTable::REG_DAT_DAA_INIT_INDEX_MASK),
        }
    }

    /// Set **REG_DCT_DAA_INIT_INDEX** `[3:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_dct_daa_init_index(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DeviceTable::REG_DCT_DAA_INIT_INDEX_MASK)
                | ((value << DeviceTable::REG_DCT_DAA_INIT_INDEX_OFFSET)
                    & DeviceTable::REG_DCT_DAA_INIT_INDEX_MASK),
        }
    }
}

impl Default for DeviceTableVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DeviceTable {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DeviceTableVal {
        DeviceTableVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DeviceTableVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DeviceTableVal) -> DeviceTableVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIME_OUT_VALUE Register
///
/// NA
///
/// Address offset: 0x005c
/// Reset value: 0x00410410
///
/// ## Fields
///
/// - **REG_RX_DATA_BUF_TO_EN** `[23]` (RW)
///
///   NA
///
/// - **REG_RX_DATA_BUF_TO_VALUE** `[22:18]` (RW)
///
///   NA
///
/// - **REG_IBI_STATUS_BUF_TO_EN** `[17]` (RW)
///
///   NA
///
/// - **REG_IBI_STATUS_BUF_TO_VALUE** `[16:12]` (RW)
///
///   NA
///
/// - **REG_IBI_DATA_BUF_TO_EN** `[11]` (RW)
///
///   NA
///
/// - **REG_IBI_DATA_BUF_TO_VALUE** `[10:6]` (RW)
///
///   NA
///
/// - **REG_RESP_BUF_TO_EN** `[5]` (RW)
///
///   NA
///
/// - **REG_RESP_BUF_TO_VALUE** `[4:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct TimeOutValue {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TimeOutValue {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TimeOutValue {
    // ======== Field Constants ========

    /// **REG_RX_DATA_BUF_TO_EN**
    ///
    /// NA
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const REG_RX_DATA_BUF_TO_EN_OFFSET: u32 = 23;
    pub const REG_RX_DATA_BUF_TO_EN_MASK: u32 = 0x00800000;
    pub const REG_RX_DATA_BUF_TO_EN: u32 = 1 << 23;

    /// **REG_RX_DATA_BUF_TO_VALUE**
    ///
    /// NA
    ///
    /// Bits `[22:18]`, mask `0x007c0000`
    pub const REG_RX_DATA_BUF_TO_VALUE_OFFSET: u32 = 18;
    pub const REG_RX_DATA_BUF_TO_VALUE_MASK: u32 = 0x007c0000;

    /// **REG_IBI_STATUS_BUF_TO_EN**
    ///
    /// NA
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const REG_IBI_STATUS_BUF_TO_EN_OFFSET: u32 = 17;
    pub const REG_IBI_STATUS_BUF_TO_EN_MASK: u32 = 0x00020000;
    pub const REG_IBI_STATUS_BUF_TO_EN: u32 = 1 << 17;

    /// **REG_IBI_STATUS_BUF_TO_VALUE**
    ///
    /// NA
    ///
    /// Bits `[16:12]`, mask `0x0001f000`
    pub const REG_IBI_STATUS_BUF_TO_VALUE_OFFSET: u32 = 12;
    pub const REG_IBI_STATUS_BUF_TO_VALUE_MASK: u32 = 0x0001f000;

    /// **REG_IBI_DATA_BUF_TO_EN**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_IBI_DATA_BUF_TO_EN_OFFSET: u32 = 11;
    pub const REG_IBI_DATA_BUF_TO_EN_MASK: u32 = 0x00000800;
    pub const REG_IBI_DATA_BUF_TO_EN: u32 = 1 << 11;

    /// **REG_IBI_DATA_BUF_TO_VALUE**
    ///
    /// NA
    ///
    /// Bits `[10:6]`, mask `0x000007c0`
    pub const REG_IBI_DATA_BUF_TO_VALUE_OFFSET: u32 = 6;
    pub const REG_IBI_DATA_BUF_TO_VALUE_MASK: u32 = 0x000007c0;

    /// **REG_RESP_BUF_TO_EN**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_RESP_BUF_TO_EN_OFFSET: u32 = 5;
    pub const REG_RESP_BUF_TO_EN_MASK: u32 = 0x00000020;
    pub const REG_RESP_BUF_TO_EN: u32 = 1 << 5;

    /// **REG_RESP_BUF_TO_VALUE**
    ///
    /// NA
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const REG_RESP_BUF_TO_VALUE_OFFSET: u32 = 0;
    pub const REG_RESP_BUF_TO_VALUE_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **REG_RX_DATA_BUF_TO_EN** `[23]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_rx_data_buf_to_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_RX_DATA_BUF_TO_EN_MASK) != 0
    }

    /// Set REG_RX_DATA_BUF_TO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_rx_data_buf_to_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_RX_DATA_BUF_TO_EN_MASK);
    }

    /// Clear REG_RX_DATA_BUF_TO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_rx_data_buf_to_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_RX_DATA_BUF_TO_EN_MASK);
    }

    /// **REG_RX_DATA_BUF_TO_VALUE** `[22:18]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_rx_data_buf_to_value(&self) -> u32 {
        (self.value.read_bits() & Self::REG_RX_DATA_BUF_TO_VALUE_MASK)
            >> Self::REG_RX_DATA_BUF_TO_VALUE_OFFSET
    }

    /// Write **REG_RX_DATA_BUF_TO_VALUE** `[22:18]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_rx_data_buf_to_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_RX_DATA_BUF_TO_VALUE_MASK)
                | ((value << Self::REG_RX_DATA_BUF_TO_VALUE_OFFSET)
                    & Self::REG_RX_DATA_BUF_TO_VALUE_MASK)
        });
    }

    /// **REG_IBI_STATUS_BUF_TO_EN** `[17]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_ibi_status_buf_to_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_IBI_STATUS_BUF_TO_EN_MASK) != 0
    }

    /// Set REG_IBI_STATUS_BUF_TO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_ibi_status_buf_to_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_IBI_STATUS_BUF_TO_EN_MASK);
    }

    /// Clear REG_IBI_STATUS_BUF_TO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_ibi_status_buf_to_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_IBI_STATUS_BUF_TO_EN_MASK);
    }

    /// **REG_IBI_STATUS_BUF_TO_VALUE** `[16:12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_ibi_status_buf_to_value(&self) -> u32 {
        (self.value.read_bits() & Self::REG_IBI_STATUS_BUF_TO_VALUE_MASK)
            >> Self::REG_IBI_STATUS_BUF_TO_VALUE_OFFSET
    }

    /// Write **REG_IBI_STATUS_BUF_TO_VALUE** `[16:12]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_ibi_status_buf_to_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_IBI_STATUS_BUF_TO_VALUE_MASK)
                | ((value << Self::REG_IBI_STATUS_BUF_TO_VALUE_OFFSET)
                    & Self::REG_IBI_STATUS_BUF_TO_VALUE_MASK)
        });
    }

    /// **REG_IBI_DATA_BUF_TO_EN** `[11]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_ibi_data_buf_to_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_IBI_DATA_BUF_TO_EN_MASK) != 0
    }

    /// Set REG_IBI_DATA_BUF_TO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_ibi_data_buf_to_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_IBI_DATA_BUF_TO_EN_MASK);
    }

    /// Clear REG_IBI_DATA_BUF_TO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_ibi_data_buf_to_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_IBI_DATA_BUF_TO_EN_MASK);
    }

    /// **REG_IBI_DATA_BUF_TO_VALUE** `[10:6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_ibi_data_buf_to_value(&self) -> u32 {
        (self.value.read_bits() & Self::REG_IBI_DATA_BUF_TO_VALUE_MASK)
            >> Self::REG_IBI_DATA_BUF_TO_VALUE_OFFSET
    }

    /// Write **REG_IBI_DATA_BUF_TO_VALUE** `[10:6]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_ibi_data_buf_to_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_IBI_DATA_BUF_TO_VALUE_MASK)
                | ((value << Self::REG_IBI_DATA_BUF_TO_VALUE_OFFSET)
                    & Self::REG_IBI_DATA_BUF_TO_VALUE_MASK)
        });
    }

    /// **REG_RESP_BUF_TO_EN** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_resp_buf_to_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_RESP_BUF_TO_EN_MASK) != 0
    }

    /// Set REG_RESP_BUF_TO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_resp_buf_to_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_RESP_BUF_TO_EN_MASK);
    }

    /// Clear REG_RESP_BUF_TO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_resp_buf_to_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_RESP_BUF_TO_EN_MASK);
    }

    /// **REG_RESP_BUF_TO_VALUE** `[4:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_resp_buf_to_value(&self) -> u32 {
        (self.value.read_bits() & Self::REG_RESP_BUF_TO_VALUE_MASK)
            >> Self::REG_RESP_BUF_TO_VALUE_OFFSET
    }

    /// Write **REG_RESP_BUF_TO_VALUE** `[4:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_resp_buf_to_value(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_RESP_BUF_TO_VALUE_MASK)
                | ((value << Self::REG_RESP_BUF_TO_VALUE_OFFSET) & Self::REG_RESP_BUF_TO_VALUE_MASK)
        });
    }
}

/// TIME_OUT_VALUE value type for batch read/modify/write operations
///
/// Use `timeoutvalue.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timeoutvalue.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TimeOutValueVal {
    bits: u32,
}

impl TimeOutValueVal {
    /// Create from raw bits
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
        Self { bits: 0x00410410 }
    }

    /// Read **REG_RX_DATA_BUF_TO_EN** `[23:23]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_rx_data_buf_to_en(&self) -> bool {
        (self.bits & TimeOutValue::REG_RX_DATA_BUF_TO_EN_MASK) != 0
    }

    /// Read **REG_RX_DATA_BUF_TO_VALUE** `[22:18]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_rx_data_buf_to_value(&self) -> u32 {
        (self.bits & TimeOutValue::REG_RX_DATA_BUF_TO_VALUE_MASK)
            >> TimeOutValue::REG_RX_DATA_BUF_TO_VALUE_OFFSET
    }

    /// Read **REG_IBI_STATUS_BUF_TO_EN** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_ibi_status_buf_to_en(&self) -> bool {
        (self.bits & TimeOutValue::REG_IBI_STATUS_BUF_TO_EN_MASK) != 0
    }

    /// Read **REG_IBI_STATUS_BUF_TO_VALUE** `[16:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_ibi_status_buf_to_value(&self) -> u32 {
        (self.bits & TimeOutValue::REG_IBI_STATUS_BUF_TO_VALUE_MASK)
            >> TimeOutValue::REG_IBI_STATUS_BUF_TO_VALUE_OFFSET
    }

    /// Read **REG_IBI_DATA_BUF_TO_EN** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_ibi_data_buf_to_en(&self) -> bool {
        (self.bits & TimeOutValue::REG_IBI_DATA_BUF_TO_EN_MASK) != 0
    }

    /// Read **REG_IBI_DATA_BUF_TO_VALUE** `[10:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_ibi_data_buf_to_value(&self) -> u32 {
        (self.bits & TimeOutValue::REG_IBI_DATA_BUF_TO_VALUE_MASK)
            >> TimeOutValue::REG_IBI_DATA_BUF_TO_VALUE_OFFSET
    }

    /// Read **REG_RESP_BUF_TO_EN** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_resp_buf_to_en(&self) -> bool {
        (self.bits & TimeOutValue::REG_RESP_BUF_TO_EN_MASK) != 0
    }

    /// Read **REG_RESP_BUF_TO_VALUE** `[4:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_resp_buf_to_value(&self) -> u32 {
        (self.bits & TimeOutValue::REG_RESP_BUF_TO_VALUE_MASK)
            >> TimeOutValue::REG_RESP_BUF_TO_VALUE_OFFSET
    }

    /// Set **REG_RX_DATA_BUF_TO_EN** `[23:23]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_rx_data_buf_to_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TimeOutValue::REG_RX_DATA_BUF_TO_EN_MASK
            } else {
                self.bits & !TimeOutValue::REG_RX_DATA_BUF_TO_EN_MASK
            },
        }
    }

    /// Set **REG_RX_DATA_BUF_TO_VALUE** `[22:18]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_rx_data_buf_to_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TimeOutValue::REG_RX_DATA_BUF_TO_VALUE_MASK)
                | ((value << TimeOutValue::REG_RX_DATA_BUF_TO_VALUE_OFFSET)
                    & TimeOutValue::REG_RX_DATA_BUF_TO_VALUE_MASK),
        }
    }

    /// Set **REG_IBI_STATUS_BUF_TO_EN** `[17:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_ibi_status_buf_to_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TimeOutValue::REG_IBI_STATUS_BUF_TO_EN_MASK
            } else {
                self.bits & !TimeOutValue::REG_IBI_STATUS_BUF_TO_EN_MASK
            },
        }
    }

    /// Set **REG_IBI_STATUS_BUF_TO_VALUE** `[16:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_ibi_status_buf_to_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TimeOutValue::REG_IBI_STATUS_BUF_TO_VALUE_MASK)
                | ((value << TimeOutValue::REG_IBI_STATUS_BUF_TO_VALUE_OFFSET)
                    & TimeOutValue::REG_IBI_STATUS_BUF_TO_VALUE_MASK),
        }
    }

    /// Set **REG_IBI_DATA_BUF_TO_EN** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_ibi_data_buf_to_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TimeOutValue::REG_IBI_DATA_BUF_TO_EN_MASK
            } else {
                self.bits & !TimeOutValue::REG_IBI_DATA_BUF_TO_EN_MASK
            },
        }
    }

    /// Set **REG_IBI_DATA_BUF_TO_VALUE** `[10:6]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_ibi_data_buf_to_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TimeOutValue::REG_IBI_DATA_BUF_TO_VALUE_MASK)
                | ((value << TimeOutValue::REG_IBI_DATA_BUF_TO_VALUE_OFFSET)
                    & TimeOutValue::REG_IBI_DATA_BUF_TO_VALUE_MASK),
        }
    }

    /// Set **REG_RESP_BUF_TO_EN** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_resp_buf_to_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TimeOutValue::REG_RESP_BUF_TO_EN_MASK
            } else {
                self.bits & !TimeOutValue::REG_RESP_BUF_TO_EN_MASK
            },
        }
    }

    /// Set **REG_RESP_BUF_TO_VALUE** `[4:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_resp_buf_to_value(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TimeOutValue::REG_RESP_BUF_TO_VALUE_MASK)
                | ((value << TimeOutValue::REG_RESP_BUF_TO_VALUE_OFFSET)
                    & TimeOutValue::REG_RESP_BUF_TO_VALUE_MASK),
        }
    }
}

impl Default for TimeOutValueVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TimeOutValue {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TimeOutValueVal {
        TimeOutValueVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TimeOutValueVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TimeOutValueVal) -> TimeOutValueVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_I3C_MST_OD_TIME Register
///
/// NA
///
/// Address offset: 0x0060
/// Reset value: 0x00050019
///
/// ## Fields
///
/// - **REG_I3C_MST_OD_HIGH_PERIOD** `[31:16]` (RW)
///
///   SCL Open-Drain High count for I3C transfers targeted to I3C devices.
///
/// - **REG_I3C_MST_OD_LOW_PERIOD** `[15:0]` (RW)
///
///   SCL Open-Drain low count for I3C transfers targeted to I3C devices.
#[repr(transparent)]
pub struct SclI3cMstOdTime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclI3cMstOdTime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclI3cMstOdTime {
    // ======== Field Constants ========

    /// **REG_I3C_MST_OD_HIGH_PERIOD**
    ///
    /// SCL Open-Drain High count for I3C transfers targeted to I3C devices.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const REG_I3C_MST_OD_HIGH_PERIOD_OFFSET: u32 = 16;
    pub const REG_I3C_MST_OD_HIGH_PERIOD_MASK: u32 = 0xffff0000;

    /// **REG_I3C_MST_OD_LOW_PERIOD**
    ///
    /// SCL Open-Drain low count for I3C transfers targeted to I3C devices.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_I3C_MST_OD_LOW_PERIOD_OFFSET: u32 = 0;
    pub const REG_I3C_MST_OD_LOW_PERIOD_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **REG_I3C_MST_OD_HIGH_PERIOD** `[31:16]` (RW)
    ///
    /// SCL Open-Drain High count for I3C transfers targeted to I3C devices.
    #[inline(always)]
    pub fn reg_i3c_mst_od_high_period(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I3C_MST_OD_HIGH_PERIOD_MASK)
            >> Self::REG_I3C_MST_OD_HIGH_PERIOD_OFFSET
    }

    /// Write **REG_I3C_MST_OD_HIGH_PERIOD** `[31:16]` (RW)
    ///
    /// SCL Open-Drain High count for I3C transfers targeted to I3C devices.
    #[inline(always)]
    pub fn write_reg_i3c_mst_od_high_period(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I3C_MST_OD_HIGH_PERIOD_MASK)
                | ((value << Self::REG_I3C_MST_OD_HIGH_PERIOD_OFFSET)
                    & Self::REG_I3C_MST_OD_HIGH_PERIOD_MASK)
        });
    }

    /// **REG_I3C_MST_OD_LOW_PERIOD** `[15:0]` (RW)
    ///
    /// SCL Open-Drain low count for I3C transfers targeted to I3C devices.
    #[inline(always)]
    pub fn reg_i3c_mst_od_low_period(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I3C_MST_OD_LOW_PERIOD_MASK)
            >> Self::REG_I3C_MST_OD_LOW_PERIOD_OFFSET
    }

    /// Write **REG_I3C_MST_OD_LOW_PERIOD** `[15:0]` (RW)
    ///
    /// SCL Open-Drain low count for I3C transfers targeted to I3C devices.
    #[inline(always)]
    pub fn write_reg_i3c_mst_od_low_period(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I3C_MST_OD_LOW_PERIOD_MASK)
                | ((value << Self::REG_I3C_MST_OD_LOW_PERIOD_OFFSET)
                    & Self::REG_I3C_MST_OD_LOW_PERIOD_MASK)
        });
    }
}

/// SCL_I3C_MST_OD_TIME value type for batch read/modify/write operations
///
/// Use `scli3cmstodtime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scli3cmstodtime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclI3cMstOdTimeVal {
    bits: u32,
}

impl SclI3cMstOdTimeVal {
    /// Create from raw bits
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
        Self { bits: 0x00050019 }
    }

    /// Read **REG_I3C_MST_OD_HIGH_PERIOD** `[31:16]`
    ///
    /// SCL Open-Drain High count for I3C transfers targeted to I3C devices.
    #[inline(always)]
    pub const fn reg_i3c_mst_od_high_period(&self) -> u32 {
        (self.bits & SclI3cMstOdTime::REG_I3C_MST_OD_HIGH_PERIOD_MASK)
            >> SclI3cMstOdTime::REG_I3C_MST_OD_HIGH_PERIOD_OFFSET
    }

    /// Read **REG_I3C_MST_OD_LOW_PERIOD** `[15:0]`
    ///
    /// SCL Open-Drain low count for I3C transfers targeted to I3C devices.
    #[inline(always)]
    pub const fn reg_i3c_mst_od_low_period(&self) -> u32 {
        (self.bits & SclI3cMstOdTime::REG_I3C_MST_OD_LOW_PERIOD_MASK)
            >> SclI3cMstOdTime::REG_I3C_MST_OD_LOW_PERIOD_OFFSET
    }

    /// Set **REG_I3C_MST_OD_HIGH_PERIOD** `[31:16]`
    ///
    /// SCL Open-Drain High count for I3C transfers targeted to I3C devices.
    #[inline(always)]
    pub const fn with_reg_i3c_mst_od_high_period(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclI3cMstOdTime::REG_I3C_MST_OD_HIGH_PERIOD_MASK)
                | ((value << SclI3cMstOdTime::REG_I3C_MST_OD_HIGH_PERIOD_OFFSET)
                    & SclI3cMstOdTime::REG_I3C_MST_OD_HIGH_PERIOD_MASK),
        }
    }

    /// Set **REG_I3C_MST_OD_LOW_PERIOD** `[15:0]`
    ///
    /// SCL Open-Drain low count for I3C transfers targeted to I3C devices.
    #[inline(always)]
    pub const fn with_reg_i3c_mst_od_low_period(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclI3cMstOdTime::REG_I3C_MST_OD_LOW_PERIOD_MASK)
                | ((value << SclI3cMstOdTime::REG_I3C_MST_OD_LOW_PERIOD_OFFSET)
                    & SclI3cMstOdTime::REG_I3C_MST_OD_LOW_PERIOD_MASK),
        }
    }
}

impl Default for SclI3cMstOdTimeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclI3cMstOdTime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclI3cMstOdTimeVal {
        SclI3cMstOdTimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclI3cMstOdTimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclI3cMstOdTimeVal) -> SclI3cMstOdTimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_I3C_MST_PP_TIME Register
///
/// NA
///
/// Address offset: 0x0064
/// Reset value: 0x00050005
///
/// ## Fields
///
/// - **REG_I3C_MST_PP_HIGH_PERIOD** `[23:16]` (RW)
///
///   NA
///
/// - **REG_I3C_MST_PP_LOW_PERIOD** `[7:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct SclI3cMstPpTime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclI3cMstPpTime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclI3cMstPpTime {
    // ======== Field Constants ========

    /// **REG_I3C_MST_PP_HIGH_PERIOD**
    ///
    /// NA
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_I3C_MST_PP_HIGH_PERIOD_OFFSET: u32 = 16;
    pub const REG_I3C_MST_PP_HIGH_PERIOD_MASK: u32 = 0x00ff0000;

    /// **REG_I3C_MST_PP_LOW_PERIOD**
    ///
    /// NA
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const REG_I3C_MST_PP_LOW_PERIOD_OFFSET: u32 = 0;
    pub const REG_I3C_MST_PP_LOW_PERIOD_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **REG_I3C_MST_PP_HIGH_PERIOD** `[23:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_i3c_mst_pp_high_period(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I3C_MST_PP_HIGH_PERIOD_MASK)
            >> Self::REG_I3C_MST_PP_HIGH_PERIOD_OFFSET
    }

    /// Write **REG_I3C_MST_PP_HIGH_PERIOD** `[23:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_i3c_mst_pp_high_period(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I3C_MST_PP_HIGH_PERIOD_MASK)
                | ((value << Self::REG_I3C_MST_PP_HIGH_PERIOD_OFFSET)
                    & Self::REG_I3C_MST_PP_HIGH_PERIOD_MASK)
        });
    }

    /// **REG_I3C_MST_PP_LOW_PERIOD** `[7:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_i3c_mst_pp_low_period(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I3C_MST_PP_LOW_PERIOD_MASK)
            >> Self::REG_I3C_MST_PP_LOW_PERIOD_OFFSET
    }

    /// Write **REG_I3C_MST_PP_LOW_PERIOD** `[7:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_i3c_mst_pp_low_period(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I3C_MST_PP_LOW_PERIOD_MASK)
                | ((value << Self::REG_I3C_MST_PP_LOW_PERIOD_OFFSET)
                    & Self::REG_I3C_MST_PP_LOW_PERIOD_MASK)
        });
    }
}

/// SCL_I3C_MST_PP_TIME value type for batch read/modify/write operations
///
/// Use `scli3cmstpptime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scli3cmstpptime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclI3cMstPpTimeVal {
    bits: u32,
}

impl SclI3cMstPpTimeVal {
    /// Create from raw bits
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
        Self { bits: 0x00050005 }
    }

    /// Read **REG_I3C_MST_PP_HIGH_PERIOD** `[23:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_i3c_mst_pp_high_period(&self) -> u32 {
        (self.bits & SclI3cMstPpTime::REG_I3C_MST_PP_HIGH_PERIOD_MASK)
            >> SclI3cMstPpTime::REG_I3C_MST_PP_HIGH_PERIOD_OFFSET
    }

    /// Read **REG_I3C_MST_PP_LOW_PERIOD** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_i3c_mst_pp_low_period(&self) -> u32 {
        (self.bits & SclI3cMstPpTime::REG_I3C_MST_PP_LOW_PERIOD_MASK)
            >> SclI3cMstPpTime::REG_I3C_MST_PP_LOW_PERIOD_OFFSET
    }

    /// Set **REG_I3C_MST_PP_HIGH_PERIOD** `[23:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_i3c_mst_pp_high_period(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclI3cMstPpTime::REG_I3C_MST_PP_HIGH_PERIOD_MASK)
                | ((value << SclI3cMstPpTime::REG_I3C_MST_PP_HIGH_PERIOD_OFFSET)
                    & SclI3cMstPpTime::REG_I3C_MST_PP_HIGH_PERIOD_MASK),
        }
    }

    /// Set **REG_I3C_MST_PP_LOW_PERIOD** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_i3c_mst_pp_low_period(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclI3cMstPpTime::REG_I3C_MST_PP_LOW_PERIOD_MASK)
                | ((value << SclI3cMstPpTime::REG_I3C_MST_PP_LOW_PERIOD_OFFSET)
                    & SclI3cMstPpTime::REG_I3C_MST_PP_LOW_PERIOD_MASK),
        }
    }
}

impl Default for SclI3cMstPpTimeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclI3cMstPpTime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclI3cMstPpTimeVal {
        SclI3cMstPpTimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclI3cMstPpTimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclI3cMstPpTimeVal) -> SclI3cMstPpTimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_I2C_FM_TIME Register
///
/// NA
///
/// Address offset: 0x0068
/// Reset value: 0x004b00a3
///
/// ## Fields
///
/// - **REG_I2C_FM_HIGH_PERIOD** `[31:16]` (RW)
///
///   The SCL open-drain low count timing for I2C Fast Mode transfers.
///
/// - **REG_I2C_FM_LOW_PERIOD** `[15:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct SclI2cFmTime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclI2cFmTime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclI2cFmTime {
    // ======== Field Constants ========

    /// **REG_I2C_FM_HIGH_PERIOD**
    ///
    /// The SCL open-drain low count timing for I2C Fast Mode transfers.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const REG_I2C_FM_HIGH_PERIOD_OFFSET: u32 = 16;
    pub const REG_I2C_FM_HIGH_PERIOD_MASK: u32 = 0xffff0000;

    /// **REG_I2C_FM_LOW_PERIOD**
    ///
    /// NA
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_I2C_FM_LOW_PERIOD_OFFSET: u32 = 0;
    pub const REG_I2C_FM_LOW_PERIOD_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **REG_I2C_FM_HIGH_PERIOD** `[31:16]` (RW)
    ///
    /// The SCL open-drain low count timing for I2C Fast Mode transfers.
    #[inline(always)]
    pub fn reg_i2c_fm_high_period(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I2C_FM_HIGH_PERIOD_MASK)
            >> Self::REG_I2C_FM_HIGH_PERIOD_OFFSET
    }

    /// Write **REG_I2C_FM_HIGH_PERIOD** `[31:16]` (RW)
    ///
    /// The SCL open-drain low count timing for I2C Fast Mode transfers.
    #[inline(always)]
    pub fn write_reg_i2c_fm_high_period(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I2C_FM_HIGH_PERIOD_MASK)
                | ((value << Self::REG_I2C_FM_HIGH_PERIOD_OFFSET)
                    & Self::REG_I2C_FM_HIGH_PERIOD_MASK)
        });
    }

    /// **REG_I2C_FM_LOW_PERIOD** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_i2c_fm_low_period(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I2C_FM_LOW_PERIOD_MASK)
            >> Self::REG_I2C_FM_LOW_PERIOD_OFFSET
    }

    /// Write **REG_I2C_FM_LOW_PERIOD** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_i2c_fm_low_period(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I2C_FM_LOW_PERIOD_MASK)
                | ((value << Self::REG_I2C_FM_LOW_PERIOD_OFFSET) & Self::REG_I2C_FM_LOW_PERIOD_MASK)
        });
    }
}

/// SCL_I2C_FM_TIME value type for batch read/modify/write operations
///
/// Use `scli2cfmtime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scli2cfmtime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclI2cFmTimeVal {
    bits: u32,
}

impl SclI2cFmTimeVal {
    /// Create from raw bits
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
        Self { bits: 0x004b00a3 }
    }

    /// Read **REG_I2C_FM_HIGH_PERIOD** `[31:16]`
    ///
    /// The SCL open-drain low count timing for I2C Fast Mode transfers.
    #[inline(always)]
    pub const fn reg_i2c_fm_high_period(&self) -> u32 {
        (self.bits & SclI2cFmTime::REG_I2C_FM_HIGH_PERIOD_MASK)
            >> SclI2cFmTime::REG_I2C_FM_HIGH_PERIOD_OFFSET
    }

    /// Read **REG_I2C_FM_LOW_PERIOD** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_i2c_fm_low_period(&self) -> u32 {
        (self.bits & SclI2cFmTime::REG_I2C_FM_LOW_PERIOD_MASK)
            >> SclI2cFmTime::REG_I2C_FM_LOW_PERIOD_OFFSET
    }

    /// Set **REG_I2C_FM_HIGH_PERIOD** `[31:16]`
    ///
    /// The SCL open-drain low count timing for I2C Fast Mode transfers.
    #[inline(always)]
    pub const fn with_reg_i2c_fm_high_period(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclI2cFmTime::REG_I2C_FM_HIGH_PERIOD_MASK)
                | ((value << SclI2cFmTime::REG_I2C_FM_HIGH_PERIOD_OFFSET)
                    & SclI2cFmTime::REG_I2C_FM_HIGH_PERIOD_MASK),
        }
    }

    /// Set **REG_I2C_FM_LOW_PERIOD** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_i2c_fm_low_period(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclI2cFmTime::REG_I2C_FM_LOW_PERIOD_MASK)
                | ((value << SclI2cFmTime::REG_I2C_FM_LOW_PERIOD_OFFSET)
                    & SclI2cFmTime::REG_I2C_FM_LOW_PERIOD_MASK),
        }
    }
}

impl Default for SclI2cFmTimeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclI2cFmTime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclI2cFmTimeVal {
        SclI2cFmTimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclI2cFmTimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclI2cFmTimeVal) -> SclI2cFmTimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_I2C_FMP_TIME Register
///
/// NA
///
/// Address offset: 0x006c
/// Reset value: 0x0021003f
///
/// ## Fields
///
/// - **REG_I2C_FMP_HIGH_PERIOD** `[23:16]` (RW)
///
///   NA
///
/// - **REG_I2C_FMP_LOW_PERIOD** `[15:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct SclI2cFmpTime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclI2cFmpTime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclI2cFmpTime {
    // ======== Field Constants ========

    /// **REG_I2C_FMP_HIGH_PERIOD**
    ///
    /// NA
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_I2C_FMP_HIGH_PERIOD_OFFSET: u32 = 16;
    pub const REG_I2C_FMP_HIGH_PERIOD_MASK: u32 = 0x00ff0000;

    /// **REG_I2C_FMP_LOW_PERIOD**
    ///
    /// NA
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_I2C_FMP_LOW_PERIOD_OFFSET: u32 = 0;
    pub const REG_I2C_FMP_LOW_PERIOD_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **REG_I2C_FMP_HIGH_PERIOD** `[23:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_i2c_fmp_high_period(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I2C_FMP_HIGH_PERIOD_MASK)
            >> Self::REG_I2C_FMP_HIGH_PERIOD_OFFSET
    }

    /// Write **REG_I2C_FMP_HIGH_PERIOD** `[23:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_i2c_fmp_high_period(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I2C_FMP_HIGH_PERIOD_MASK)
                | ((value << Self::REG_I2C_FMP_HIGH_PERIOD_OFFSET)
                    & Self::REG_I2C_FMP_HIGH_PERIOD_MASK)
        });
    }

    /// **REG_I2C_FMP_LOW_PERIOD** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_i2c_fmp_low_period(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I2C_FMP_LOW_PERIOD_MASK)
            >> Self::REG_I2C_FMP_LOW_PERIOD_OFFSET
    }

    /// Write **REG_I2C_FMP_LOW_PERIOD** `[15:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_i2c_fmp_low_period(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I2C_FMP_LOW_PERIOD_MASK)
                | ((value << Self::REG_I2C_FMP_LOW_PERIOD_OFFSET)
                    & Self::REG_I2C_FMP_LOW_PERIOD_MASK)
        });
    }
}

/// SCL_I2C_FMP_TIME value type for batch read/modify/write operations
///
/// Use `scli2cfmptime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scli2cfmptime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclI2cFmpTimeVal {
    bits: u32,
}

impl SclI2cFmpTimeVal {
    /// Create from raw bits
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
        Self { bits: 0x0021003f }
    }

    /// Read **REG_I2C_FMP_HIGH_PERIOD** `[23:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_i2c_fmp_high_period(&self) -> u32 {
        (self.bits & SclI2cFmpTime::REG_I2C_FMP_HIGH_PERIOD_MASK)
            >> SclI2cFmpTime::REG_I2C_FMP_HIGH_PERIOD_OFFSET
    }

    /// Read **REG_I2C_FMP_LOW_PERIOD** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_i2c_fmp_low_period(&self) -> u32 {
        (self.bits & SclI2cFmpTime::REG_I2C_FMP_LOW_PERIOD_MASK)
            >> SclI2cFmpTime::REG_I2C_FMP_LOW_PERIOD_OFFSET
    }

    /// Set **REG_I2C_FMP_HIGH_PERIOD** `[23:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_i2c_fmp_high_period(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclI2cFmpTime::REG_I2C_FMP_HIGH_PERIOD_MASK)
                | ((value << SclI2cFmpTime::REG_I2C_FMP_HIGH_PERIOD_OFFSET)
                    & SclI2cFmpTime::REG_I2C_FMP_HIGH_PERIOD_MASK),
        }
    }

    /// Set **REG_I2C_FMP_LOW_PERIOD** `[15:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_i2c_fmp_low_period(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclI2cFmpTime::REG_I2C_FMP_LOW_PERIOD_MASK)
                | ((value << SclI2cFmpTime::REG_I2C_FMP_LOW_PERIOD_OFFSET)
                    & SclI2cFmpTime::REG_I2C_FMP_LOW_PERIOD_MASK),
        }
    }
}

impl Default for SclI2cFmpTimeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclI2cFmpTime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclI2cFmpTimeVal {
        SclI2cFmpTimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclI2cFmpTimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclI2cFmpTimeVal) -> SclI2cFmpTimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_EXT_LOW_TIME Register
///
/// NA
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **REG_I3C_MST_EXT_LOW_PERIOD4** `[31:24]` (RW)
///
///   NA
///
/// - **REG_I3C_MST_EXT_LOW_PERIOD3** `[23:16]` (RW)
///
///   NA
///
/// - **REG_I3C_MST_EXT_LOW_PERIOD2** `[15:8]` (RW)
///
///   NA
///
/// - **REG_I3C_MST_EXT_LOW_PERIOD1** `[7:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct SclExtLowTime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclExtLowTime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclExtLowTime {
    // ======== Field Constants ========

    /// **REG_I3C_MST_EXT_LOW_PERIOD4**
    ///
    /// NA
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const REG_I3C_MST_EXT_LOW_PERIOD4_OFFSET: u32 = 24;
    pub const REG_I3C_MST_EXT_LOW_PERIOD4_MASK: u32 = 0xff000000;

    /// **REG_I3C_MST_EXT_LOW_PERIOD3**
    ///
    /// NA
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_I3C_MST_EXT_LOW_PERIOD3_OFFSET: u32 = 16;
    pub const REG_I3C_MST_EXT_LOW_PERIOD3_MASK: u32 = 0x00ff0000;

    /// **REG_I3C_MST_EXT_LOW_PERIOD2**
    ///
    /// NA
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const REG_I3C_MST_EXT_LOW_PERIOD2_OFFSET: u32 = 8;
    pub const REG_I3C_MST_EXT_LOW_PERIOD2_MASK: u32 = 0x0000ff00;

    /// **REG_I3C_MST_EXT_LOW_PERIOD1**
    ///
    /// NA
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const REG_I3C_MST_EXT_LOW_PERIOD1_OFFSET: u32 = 0;
    pub const REG_I3C_MST_EXT_LOW_PERIOD1_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **REG_I3C_MST_EXT_LOW_PERIOD4** `[31:24]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period4(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I3C_MST_EXT_LOW_PERIOD4_MASK)
            >> Self::REG_I3C_MST_EXT_LOW_PERIOD4_OFFSET
    }

    /// Write **REG_I3C_MST_EXT_LOW_PERIOD4** `[31:24]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_i3c_mst_ext_low_period4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I3C_MST_EXT_LOW_PERIOD4_MASK)
                | ((value << Self::REG_I3C_MST_EXT_LOW_PERIOD4_OFFSET)
                    & Self::REG_I3C_MST_EXT_LOW_PERIOD4_MASK)
        });
    }

    /// **REG_I3C_MST_EXT_LOW_PERIOD3** `[23:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period3(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I3C_MST_EXT_LOW_PERIOD3_MASK)
            >> Self::REG_I3C_MST_EXT_LOW_PERIOD3_OFFSET
    }

    /// Write **REG_I3C_MST_EXT_LOW_PERIOD3** `[23:16]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_i3c_mst_ext_low_period3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I3C_MST_EXT_LOW_PERIOD3_MASK)
                | ((value << Self::REG_I3C_MST_EXT_LOW_PERIOD3_OFFSET)
                    & Self::REG_I3C_MST_EXT_LOW_PERIOD3_MASK)
        });
    }

    /// **REG_I3C_MST_EXT_LOW_PERIOD2** `[15:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period2(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I3C_MST_EXT_LOW_PERIOD2_MASK)
            >> Self::REG_I3C_MST_EXT_LOW_PERIOD2_OFFSET
    }

    /// Write **REG_I3C_MST_EXT_LOW_PERIOD2** `[15:8]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_i3c_mst_ext_low_period2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I3C_MST_EXT_LOW_PERIOD2_MASK)
                | ((value << Self::REG_I3C_MST_EXT_LOW_PERIOD2_OFFSET)
                    & Self::REG_I3C_MST_EXT_LOW_PERIOD2_MASK)
        });
    }

    /// **REG_I3C_MST_EXT_LOW_PERIOD1** `[7:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_i3c_mst_ext_low_period1(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I3C_MST_EXT_LOW_PERIOD1_MASK)
            >> Self::REG_I3C_MST_EXT_LOW_PERIOD1_OFFSET
    }

    /// Write **REG_I3C_MST_EXT_LOW_PERIOD1** `[7:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_i3c_mst_ext_low_period1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I3C_MST_EXT_LOW_PERIOD1_MASK)
                | ((value << Self::REG_I3C_MST_EXT_LOW_PERIOD1_OFFSET)
                    & Self::REG_I3C_MST_EXT_LOW_PERIOD1_MASK)
        });
    }
}

/// SCL_EXT_LOW_TIME value type for batch read/modify/write operations
///
/// Use `sclextlowtime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sclextlowtime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclExtLowTimeVal {
    bits: u32,
}

impl SclExtLowTimeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_I3C_MST_EXT_LOW_PERIOD4** `[31:24]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_i3c_mst_ext_low_period4(&self) -> u32 {
        (self.bits & SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD4_MASK)
            >> SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD4_OFFSET
    }

    /// Read **REG_I3C_MST_EXT_LOW_PERIOD3** `[23:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_i3c_mst_ext_low_period3(&self) -> u32 {
        (self.bits & SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD3_MASK)
            >> SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD3_OFFSET
    }

    /// Read **REG_I3C_MST_EXT_LOW_PERIOD2** `[15:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_i3c_mst_ext_low_period2(&self) -> u32 {
        (self.bits & SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD2_MASK)
            >> SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD2_OFFSET
    }

    /// Read **REG_I3C_MST_EXT_LOW_PERIOD1** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_i3c_mst_ext_low_period1(&self) -> u32 {
        (self.bits & SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD1_MASK)
            >> SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD1_OFFSET
    }

    /// Set **REG_I3C_MST_EXT_LOW_PERIOD4** `[31:24]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_i3c_mst_ext_low_period4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD4_MASK)
                | ((value << SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD4_OFFSET)
                    & SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD4_MASK),
        }
    }

    /// Set **REG_I3C_MST_EXT_LOW_PERIOD3** `[23:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_i3c_mst_ext_low_period3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD3_MASK)
                | ((value << SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD3_OFFSET)
                    & SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD3_MASK),
        }
    }

    /// Set **REG_I3C_MST_EXT_LOW_PERIOD2** `[15:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_i3c_mst_ext_low_period2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD2_MASK)
                | ((value << SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD2_OFFSET)
                    & SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD2_MASK),
        }
    }

    /// Set **REG_I3C_MST_EXT_LOW_PERIOD1** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_i3c_mst_ext_low_period1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD1_MASK)
                | ((value << SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD1_OFFSET)
                    & SclExtLowTime::REG_I3C_MST_EXT_LOW_PERIOD1_MASK),
        }
    }
}

impl Default for SclExtLowTimeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SclExtLowTime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclExtLowTimeVal {
        SclExtLowTimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclExtLowTimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclExtLowTimeVal) -> SclExtLowTimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SDA_SAMPLE_TIME Register
///
/// NA
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **REG_SDA_PP_SAMPLE_TIME** `[13:9]` (RW)
///
///   It is used to adjust sda sample point when scl high under push pull speed
///
/// - **REG_SDA_OD_SAMPLE_TIME** `[8:0]` (RW)
///
///   It is used to adjust sda sample point when scl high under open drain speed
#[repr(transparent)]
pub struct SdaSampleTime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SdaSampleTime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SdaSampleTime {
    // ======== Field Constants ========

    /// **REG_SDA_PP_SAMPLE_TIME**
    ///
    /// It is used to adjust sda sample point when scl high under push pull speed
    ///
    /// Bits `[13:9]`, mask `0x00003e00`
    pub const REG_SDA_PP_SAMPLE_TIME_OFFSET: u32 = 9;
    pub const REG_SDA_PP_SAMPLE_TIME_MASK: u32 = 0x00003e00;

    /// **REG_SDA_OD_SAMPLE_TIME**
    ///
    /// It is used to adjust sda sample point when scl high under open drain speed
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const REG_SDA_OD_SAMPLE_TIME_OFFSET: u32 = 0;
    pub const REG_SDA_OD_SAMPLE_TIME_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **REG_SDA_PP_SAMPLE_TIME** `[13:9]` (RW)
    ///
    /// It is used to adjust sda sample point when scl high under push pull speed
    #[inline(always)]
    pub fn reg_sda_pp_sample_time(&self) -> u32 {
        (self.value.read_bits() & Self::REG_SDA_PP_SAMPLE_TIME_MASK)
            >> Self::REG_SDA_PP_SAMPLE_TIME_OFFSET
    }

    /// Write **REG_SDA_PP_SAMPLE_TIME** `[13:9]` (RW)
    ///
    /// It is used to adjust sda sample point when scl high under push pull speed
    #[inline(always)]
    pub fn write_reg_sda_pp_sample_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_SDA_PP_SAMPLE_TIME_MASK)
                | ((value << Self::REG_SDA_PP_SAMPLE_TIME_OFFSET)
                    & Self::REG_SDA_PP_SAMPLE_TIME_MASK)
        });
    }

    /// **REG_SDA_OD_SAMPLE_TIME** `[8:0]` (RW)
    ///
    /// It is used to adjust sda sample point when scl high under open drain speed
    #[inline(always)]
    pub fn reg_sda_od_sample_time(&self) -> u32 {
        (self.value.read_bits() & Self::REG_SDA_OD_SAMPLE_TIME_MASK)
            >> Self::REG_SDA_OD_SAMPLE_TIME_OFFSET
    }

    /// Write **REG_SDA_OD_SAMPLE_TIME** `[8:0]` (RW)
    ///
    /// It is used to adjust sda sample point when scl high under open drain speed
    #[inline(always)]
    pub fn write_reg_sda_od_sample_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_SDA_OD_SAMPLE_TIME_MASK)
                | ((value << Self::REG_SDA_OD_SAMPLE_TIME_OFFSET)
                    & Self::REG_SDA_OD_SAMPLE_TIME_MASK)
        });
    }
}

/// SDA_SAMPLE_TIME value type for batch read/modify/write operations
///
/// Use `sdasampletime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sdasampletime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SdaSampleTimeVal {
    bits: u32,
}

impl SdaSampleTimeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_SDA_PP_SAMPLE_TIME** `[13:9]`
    ///
    /// It is used to adjust sda sample point when scl high under push pull speed
    #[inline(always)]
    pub const fn reg_sda_pp_sample_time(&self) -> u32 {
        (self.bits & SdaSampleTime::REG_SDA_PP_SAMPLE_TIME_MASK)
            >> SdaSampleTime::REG_SDA_PP_SAMPLE_TIME_OFFSET
    }

    /// Read **REG_SDA_OD_SAMPLE_TIME** `[8:0]`
    ///
    /// It is used to adjust sda sample point when scl high under open drain speed
    #[inline(always)]
    pub const fn reg_sda_od_sample_time(&self) -> u32 {
        (self.bits & SdaSampleTime::REG_SDA_OD_SAMPLE_TIME_MASK)
            >> SdaSampleTime::REG_SDA_OD_SAMPLE_TIME_OFFSET
    }

    /// Set **REG_SDA_PP_SAMPLE_TIME** `[13:9]`
    ///
    /// It is used to adjust sda sample point when scl high under push pull speed
    #[inline(always)]
    pub const fn with_reg_sda_pp_sample_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SdaSampleTime::REG_SDA_PP_SAMPLE_TIME_MASK)
                | ((value << SdaSampleTime::REG_SDA_PP_SAMPLE_TIME_OFFSET)
                    & SdaSampleTime::REG_SDA_PP_SAMPLE_TIME_MASK),
        }
    }

    /// Set **REG_SDA_OD_SAMPLE_TIME** `[8:0]`
    ///
    /// It is used to adjust sda sample point when scl high under open drain speed
    #[inline(always)]
    pub const fn with_reg_sda_od_sample_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SdaSampleTime::REG_SDA_OD_SAMPLE_TIME_MASK)
                | ((value << SdaSampleTime::REG_SDA_OD_SAMPLE_TIME_OFFSET)
                    & SdaSampleTime::REG_SDA_OD_SAMPLE_TIME_MASK),
        }
    }
}

impl Default for SdaSampleTimeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SdaSampleTime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SdaSampleTimeVal {
        SdaSampleTimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SdaSampleTimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SdaSampleTimeVal) -> SdaSampleTimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SDA_HOLD_TIME Register
///
/// NA
///
/// Address offset: 0x0078
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **REG_SDA_PP_TX_HOLD_TIME** `[13:9]` (RW)
///
///   It is used to adjust sda dirve point after scl neg under push pull speed
///
/// - **REG_SDA_OD_TX_HOLD_TIME** `[8:0]` (RW)
///
///   It is used to adjust sda drive point after scl neg under open drain speed
#[repr(transparent)]
pub struct SdaHoldTime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SdaHoldTime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SdaHoldTime {
    // ======== Field Constants ========

    /// **REG_SDA_PP_TX_HOLD_TIME**
    ///
    /// It is used to adjust sda dirve point after scl neg under push pull speed
    ///
    /// Bits `[13:9]`, mask `0x00003e00`
    pub const REG_SDA_PP_TX_HOLD_TIME_OFFSET: u32 = 9;
    pub const REG_SDA_PP_TX_HOLD_TIME_MASK: u32 = 0x00003e00;

    /// **REG_SDA_OD_TX_HOLD_TIME**
    ///
    /// It is used to adjust sda drive point after scl neg under open drain speed
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const REG_SDA_OD_TX_HOLD_TIME_OFFSET: u32 = 0;
    pub const REG_SDA_OD_TX_HOLD_TIME_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **REG_SDA_PP_TX_HOLD_TIME** `[13:9]` (RW)
    ///
    /// It is used to adjust sda dirve point after scl neg under push pull speed
    #[inline(always)]
    pub fn reg_sda_pp_tx_hold_time(&self) -> u32 {
        (self.value.read_bits() & Self::REG_SDA_PP_TX_HOLD_TIME_MASK)
            >> Self::REG_SDA_PP_TX_HOLD_TIME_OFFSET
    }

    /// Write **REG_SDA_PP_TX_HOLD_TIME** `[13:9]` (RW)
    ///
    /// It is used to adjust sda dirve point after scl neg under push pull speed
    #[inline(always)]
    pub fn write_reg_sda_pp_tx_hold_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_SDA_PP_TX_HOLD_TIME_MASK)
                | ((value << Self::REG_SDA_PP_TX_HOLD_TIME_OFFSET)
                    & Self::REG_SDA_PP_TX_HOLD_TIME_MASK)
        });
    }

    /// **REG_SDA_OD_TX_HOLD_TIME** `[8:0]` (RW)
    ///
    /// It is used to adjust sda drive point after scl neg under open drain speed
    #[inline(always)]
    pub fn reg_sda_od_tx_hold_time(&self) -> u32 {
        (self.value.read_bits() & Self::REG_SDA_OD_TX_HOLD_TIME_MASK)
            >> Self::REG_SDA_OD_TX_HOLD_TIME_OFFSET
    }

    /// Write **REG_SDA_OD_TX_HOLD_TIME** `[8:0]` (RW)
    ///
    /// It is used to adjust sda drive point after scl neg under open drain speed
    #[inline(always)]
    pub fn write_reg_sda_od_tx_hold_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_SDA_OD_TX_HOLD_TIME_MASK)
                | ((value << Self::REG_SDA_OD_TX_HOLD_TIME_OFFSET)
                    & Self::REG_SDA_OD_TX_HOLD_TIME_MASK)
        });
    }
}

/// SDA_HOLD_TIME value type for batch read/modify/write operations
///
/// Use `sdaholdtime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sdaholdtime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SdaHoldTimeVal {
    bits: u32,
}

impl SdaHoldTimeVal {
    /// Create from raw bits
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

    /// Read **REG_SDA_PP_TX_HOLD_TIME** `[13:9]`
    ///
    /// It is used to adjust sda dirve point after scl neg under push pull speed
    #[inline(always)]
    pub const fn reg_sda_pp_tx_hold_time(&self) -> u32 {
        (self.bits & SdaHoldTime::REG_SDA_PP_TX_HOLD_TIME_MASK)
            >> SdaHoldTime::REG_SDA_PP_TX_HOLD_TIME_OFFSET
    }

    /// Read **REG_SDA_OD_TX_HOLD_TIME** `[8:0]`
    ///
    /// It is used to adjust sda drive point after scl neg under open drain speed
    #[inline(always)]
    pub const fn reg_sda_od_tx_hold_time(&self) -> u32 {
        (self.bits & SdaHoldTime::REG_SDA_OD_TX_HOLD_TIME_MASK)
            >> SdaHoldTime::REG_SDA_OD_TX_HOLD_TIME_OFFSET
    }

    /// Set **REG_SDA_PP_TX_HOLD_TIME** `[13:9]`
    ///
    /// It is used to adjust sda dirve point after scl neg under push pull speed
    #[inline(always)]
    pub const fn with_reg_sda_pp_tx_hold_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SdaHoldTime::REG_SDA_PP_TX_HOLD_TIME_MASK)
                | ((value << SdaHoldTime::REG_SDA_PP_TX_HOLD_TIME_OFFSET)
                    & SdaHoldTime::REG_SDA_PP_TX_HOLD_TIME_MASK),
        }
    }

    /// Set **REG_SDA_OD_TX_HOLD_TIME** `[8:0]`
    ///
    /// It is used to adjust sda drive point after scl neg under open drain speed
    #[inline(always)]
    pub const fn with_reg_sda_od_tx_hold_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SdaHoldTime::REG_SDA_OD_TX_HOLD_TIME_MASK)
                | ((value << SdaHoldTime::REG_SDA_OD_TX_HOLD_TIME_OFFSET)
                    & SdaHoldTime::REG_SDA_OD_TX_HOLD_TIME_MASK),
        }
    }
}

impl Default for SdaHoldTimeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SdaHoldTime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SdaHoldTimeVal {
        SdaHoldTimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SdaHoldTimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SdaHoldTimeVal) -> SdaHoldTimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_START_HOLD Register
///
/// NA
///
/// Address offset: 0x007c
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **REG_START_DET_HOLD_TIME** `[10:9]` (RW)
///
///   NA
///
/// - **REG_SCL_START_HOLD_TIME** `[8:0]` (RW)
///
///   I2C_SCL_START_HOLD_TIME
#[repr(transparent)]
pub struct SclStartHold {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclStartHold {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclStartHold {
    // ======== Field Constants ========

    /// **REG_START_DET_HOLD_TIME**
    ///
    /// NA
    ///
    /// Bits `[10:9]`, mask `0x00000600`
    pub const REG_START_DET_HOLD_TIME_OFFSET: u32 = 9;
    pub const REG_START_DET_HOLD_TIME_MASK: u32 = 0x00000600;

    /// **REG_SCL_START_HOLD_TIME**
    ///
    /// I2C_SCL_START_HOLD_TIME
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const REG_SCL_START_HOLD_TIME_OFFSET: u32 = 0;
    pub const REG_SCL_START_HOLD_TIME_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **REG_START_DET_HOLD_TIME** `[10:9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_start_det_hold_time(&self) -> u32 {
        (self.value.read_bits() & Self::REG_START_DET_HOLD_TIME_MASK)
            >> Self::REG_START_DET_HOLD_TIME_OFFSET
    }

    /// Write **REG_START_DET_HOLD_TIME** `[10:9]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_start_det_hold_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_START_DET_HOLD_TIME_MASK)
                | ((value << Self::REG_START_DET_HOLD_TIME_OFFSET)
                    & Self::REG_START_DET_HOLD_TIME_MASK)
        });
    }

    /// **REG_SCL_START_HOLD_TIME** `[8:0]` (RW)
    ///
    /// I2C_SCL_START_HOLD_TIME
    #[inline(always)]
    pub fn reg_scl_start_hold_time(&self) -> u32 {
        (self.value.read_bits() & Self::REG_SCL_START_HOLD_TIME_MASK)
            >> Self::REG_SCL_START_HOLD_TIME_OFFSET
    }

    /// Write **REG_SCL_START_HOLD_TIME** `[8:0]` (RW)
    ///
    /// I2C_SCL_START_HOLD_TIME
    #[inline(always)]
    pub fn write_reg_scl_start_hold_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_SCL_START_HOLD_TIME_MASK)
                | ((value << Self::REG_SCL_START_HOLD_TIME_OFFSET)
                    & Self::REG_SCL_START_HOLD_TIME_MASK)
        });
    }
}

/// SCL_START_HOLD value type for batch read/modify/write operations
///
/// Use `sclstarthold.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sclstarthold.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclStartHoldVal {
    bits: u32,
}

impl SclStartHoldVal {
    /// Create from raw bits
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
        Self { bits: 0x00000008 }
    }

    /// Read **REG_START_DET_HOLD_TIME** `[10:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_start_det_hold_time(&self) -> u32 {
        (self.bits & SclStartHold::REG_START_DET_HOLD_TIME_MASK)
            >> SclStartHold::REG_START_DET_HOLD_TIME_OFFSET
    }

    /// Read **REG_SCL_START_HOLD_TIME** `[8:0]`
    ///
    /// I2C_SCL_START_HOLD_TIME
    #[inline(always)]
    pub const fn reg_scl_start_hold_time(&self) -> u32 {
        (self.bits & SclStartHold::REG_SCL_START_HOLD_TIME_MASK)
            >> SclStartHold::REG_SCL_START_HOLD_TIME_OFFSET
    }

    /// Set **REG_START_DET_HOLD_TIME** `[10:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_start_det_hold_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclStartHold::REG_START_DET_HOLD_TIME_MASK)
                | ((value << SclStartHold::REG_START_DET_HOLD_TIME_OFFSET)
                    & SclStartHold::REG_START_DET_HOLD_TIME_MASK),
        }
    }

    /// Set **REG_SCL_START_HOLD_TIME** `[8:0]`
    ///
    /// I2C_SCL_START_HOLD_TIME
    #[inline(always)]
    pub const fn with_reg_scl_start_hold_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclStartHold::REG_SCL_START_HOLD_TIME_MASK)
                | ((value << SclStartHold::REG_SCL_START_HOLD_TIME_OFFSET)
                    & SclStartHold::REG_SCL_START_HOLD_TIME_MASK),
        }
    }
}

impl Default for SclStartHoldVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclStartHold {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclStartHoldVal {
        SclStartHoldVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclStartHoldVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclStartHoldVal) -> SclStartHoldVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_RSTART_SETUP Register
///
/// NA
///
/// Address offset: 0x0080
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **REG_SCL_RSTART_SETUP_TIME** `[8:0]` (RW)
///
///   I2C_SCL_RSTART_SETUP_TIME
#[repr(transparent)]
pub struct SclRstartSetup {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclRstartSetup {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclRstartSetup {
    // ======== Field Constants ========

    /// **REG_SCL_RSTART_SETUP_TIME**
    ///
    /// I2C_SCL_RSTART_SETUP_TIME
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const REG_SCL_RSTART_SETUP_TIME_OFFSET: u32 = 0;
    pub const REG_SCL_RSTART_SETUP_TIME_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **REG_SCL_RSTART_SETUP_TIME** `[8:0]` (RW)
    ///
    /// I2C_SCL_RSTART_SETUP_TIME
    #[inline(always)]
    pub fn reg_scl_rstart_setup_time(&self) -> u32 {
        (self.value.read_bits() & Self::REG_SCL_RSTART_SETUP_TIME_MASK)
            >> Self::REG_SCL_RSTART_SETUP_TIME_OFFSET
    }

    /// Write **REG_SCL_RSTART_SETUP_TIME** `[8:0]` (RW)
    ///
    /// I2C_SCL_RSTART_SETUP_TIME
    #[inline(always)]
    pub fn write_reg_scl_rstart_setup_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_SCL_RSTART_SETUP_TIME_MASK)
                | ((value << Self::REG_SCL_RSTART_SETUP_TIME_OFFSET)
                    & Self::REG_SCL_RSTART_SETUP_TIME_MASK)
        });
    }
}

/// SCL_RSTART_SETUP value type for batch read/modify/write operations
///
/// Use `sclrstartsetup.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sclrstartsetup.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclRstartSetupVal {
    bits: u32,
}

impl SclRstartSetupVal {
    /// Create from raw bits
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
        Self { bits: 0x00000008 }
    }

    /// Read **REG_SCL_RSTART_SETUP_TIME** `[8:0]`
    ///
    /// I2C_SCL_RSTART_SETUP_TIME
    #[inline(always)]
    pub const fn reg_scl_rstart_setup_time(&self) -> u32 {
        (self.bits & SclRstartSetup::REG_SCL_RSTART_SETUP_TIME_MASK)
            >> SclRstartSetup::REG_SCL_RSTART_SETUP_TIME_OFFSET
    }

    /// Set **REG_SCL_RSTART_SETUP_TIME** `[8:0]`
    ///
    /// I2C_SCL_RSTART_SETUP_TIME
    #[inline(always)]
    pub const fn with_reg_scl_rstart_setup_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclRstartSetup::REG_SCL_RSTART_SETUP_TIME_MASK)
                | ((value << SclRstartSetup::REG_SCL_RSTART_SETUP_TIME_OFFSET)
                    & SclRstartSetup::REG_SCL_RSTART_SETUP_TIME_MASK),
        }
    }
}

impl Default for SclRstartSetupVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclRstartSetup {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclRstartSetupVal {
        SclRstartSetupVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclRstartSetupVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclRstartSetupVal) -> SclRstartSetupVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_STOP_HOLD Register
///
/// NA
///
/// Address offset: 0x0084
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **REG_SCL_STOP_HOLD_TIME** `[8:0]` (RW)
///
///   I2C_SCL_STOP_HOLD_TIME
#[repr(transparent)]
pub struct SclStopHold {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclStopHold {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclStopHold {
    // ======== Field Constants ========

    /// **REG_SCL_STOP_HOLD_TIME**
    ///
    /// I2C_SCL_STOP_HOLD_TIME
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const REG_SCL_STOP_HOLD_TIME_OFFSET: u32 = 0;
    pub const REG_SCL_STOP_HOLD_TIME_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **REG_SCL_STOP_HOLD_TIME** `[8:0]` (RW)
    ///
    /// I2C_SCL_STOP_HOLD_TIME
    #[inline(always)]
    pub fn reg_scl_stop_hold_time(&self) -> u32 {
        (self.value.read_bits() & Self::REG_SCL_STOP_HOLD_TIME_MASK)
            >> Self::REG_SCL_STOP_HOLD_TIME_OFFSET
    }

    /// Write **REG_SCL_STOP_HOLD_TIME** `[8:0]` (RW)
    ///
    /// I2C_SCL_STOP_HOLD_TIME
    #[inline(always)]
    pub fn write_reg_scl_stop_hold_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_SCL_STOP_HOLD_TIME_MASK)
                | ((value << Self::REG_SCL_STOP_HOLD_TIME_OFFSET)
                    & Self::REG_SCL_STOP_HOLD_TIME_MASK)
        });
    }
}

/// SCL_STOP_HOLD value type for batch read/modify/write operations
///
/// Use `sclstophold.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sclstophold.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclStopHoldVal {
    bits: u32,
}

impl SclStopHoldVal {
    /// Create from raw bits
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
        Self { bits: 0x00000008 }
    }

    /// Read **REG_SCL_STOP_HOLD_TIME** `[8:0]`
    ///
    /// I2C_SCL_STOP_HOLD_TIME
    #[inline(always)]
    pub const fn reg_scl_stop_hold_time(&self) -> u32 {
        (self.bits & SclStopHold::REG_SCL_STOP_HOLD_TIME_MASK)
            >> SclStopHold::REG_SCL_STOP_HOLD_TIME_OFFSET
    }

    /// Set **REG_SCL_STOP_HOLD_TIME** `[8:0]`
    ///
    /// I2C_SCL_STOP_HOLD_TIME
    #[inline(always)]
    pub const fn with_reg_scl_stop_hold_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclStopHold::REG_SCL_STOP_HOLD_TIME_MASK)
                | ((value << SclStopHold::REG_SCL_STOP_HOLD_TIME_OFFSET)
                    & SclStopHold::REG_SCL_STOP_HOLD_TIME_MASK),
        }
    }
}

impl Default for SclStopHoldVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclStopHold {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclStopHoldVal {
        SclStopHoldVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclStopHoldVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclStopHoldVal) -> SclStopHoldVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_STOP_SETUP Register
///
/// NA
///
/// Address offset: 0x0088
/// Reset value: 0x00000008
///
/// ## Fields
///
/// - **REG_SCL_STOP_SETUP_TIME** `[8:0]` (RW)
///
///   I2C_SCL_STOP_SETUP_TIME
#[repr(transparent)]
pub struct SclStopSetup {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclStopSetup {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclStopSetup {
    // ======== Field Constants ========

    /// **REG_SCL_STOP_SETUP_TIME**
    ///
    /// I2C_SCL_STOP_SETUP_TIME
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const REG_SCL_STOP_SETUP_TIME_OFFSET: u32 = 0;
    pub const REG_SCL_STOP_SETUP_TIME_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **REG_SCL_STOP_SETUP_TIME** `[8:0]` (RW)
    ///
    /// I2C_SCL_STOP_SETUP_TIME
    #[inline(always)]
    pub fn reg_scl_stop_setup_time(&self) -> u32 {
        (self.value.read_bits() & Self::REG_SCL_STOP_SETUP_TIME_MASK)
            >> Self::REG_SCL_STOP_SETUP_TIME_OFFSET
    }

    /// Write **REG_SCL_STOP_SETUP_TIME** `[8:0]` (RW)
    ///
    /// I2C_SCL_STOP_SETUP_TIME
    #[inline(always)]
    pub fn write_reg_scl_stop_setup_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_SCL_STOP_SETUP_TIME_MASK)
                | ((value << Self::REG_SCL_STOP_SETUP_TIME_OFFSET)
                    & Self::REG_SCL_STOP_SETUP_TIME_MASK)
        });
    }
}

/// SCL_STOP_SETUP value type for batch read/modify/write operations
///
/// Use `sclstopsetup.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sclstopsetup.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclStopSetupVal {
    bits: u32,
}

impl SclStopSetupVal {
    /// Create from raw bits
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
        Self { bits: 0x00000008 }
    }

    /// Read **REG_SCL_STOP_SETUP_TIME** `[8:0]`
    ///
    /// I2C_SCL_STOP_SETUP_TIME
    #[inline(always)]
    pub const fn reg_scl_stop_setup_time(&self) -> u32 {
        (self.bits & SclStopSetup::REG_SCL_STOP_SETUP_TIME_MASK)
            >> SclStopSetup::REG_SCL_STOP_SETUP_TIME_OFFSET
    }

    /// Set **REG_SCL_STOP_SETUP_TIME** `[8:0]`
    ///
    /// I2C_SCL_STOP_SETUP_TIME
    #[inline(always)]
    pub const fn with_reg_scl_stop_setup_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclStopSetup::REG_SCL_STOP_SETUP_TIME_MASK)
                | ((value << SclStopSetup::REG_SCL_STOP_SETUP_TIME_OFFSET)
                    & SclStopSetup::REG_SCL_STOP_SETUP_TIME_MASK),
        }
    }
}

impl Default for SclStopSetupVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclStopSetup {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclStopSetupVal {
        SclStopSetupVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclStopSetupVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclStopSetupVal) -> SclStopSetupVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BUS_FREE_TIME Register
///
/// NA
///
/// Address offset: 0x0090
/// Reset value: 0x00000005
///
/// ## Fields
///
/// - **REG_BUS_FREE_TIME** `[15:0]` (RW)
///
///   I3C Bus Free Count Value. This field is used only in Master mode. In pure Bus System, this field represents tCAS. In Mixed Bus System, this field is expected to be programmed to tLOW of I2C Timing.
#[repr(transparent)]
pub struct BusFreeTime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BusFreeTime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BusFreeTime {
    // ======== Field Constants ========

    /// **REG_BUS_FREE_TIME**
    ///
    /// I3C Bus Free Count Value. This field is used only in Master mode. In pure Bus System, this field represents tCAS. In Mixed Bus System, this field is expected to be programmed to tLOW of I2C Timing.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_BUS_FREE_TIME_OFFSET: u32 = 0;
    pub const REG_BUS_FREE_TIME_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **REG_BUS_FREE_TIME** `[15:0]` (RW)
    ///
    /// I3C Bus Free Count Value. This field is used only in Master mode. In pure Bus System, this field represents tCAS. In Mixed Bus System, this field is expected to be programmed to tLOW of I2C Timing.
    #[inline(always)]
    pub fn reg_bus_free_time(&self) -> u32 {
        (self.value.read_bits() & Self::REG_BUS_FREE_TIME_MASK) >> Self::REG_BUS_FREE_TIME_OFFSET
    }

    /// Write **REG_BUS_FREE_TIME** `[15:0]` (RW)
    ///
    /// I3C Bus Free Count Value. This field is used only in Master mode. In pure Bus System, this field represents tCAS. In Mixed Bus System, this field is expected to be programmed to tLOW of I2C Timing.
    #[inline(always)]
    pub fn write_reg_bus_free_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_BUS_FREE_TIME_MASK)
                | ((value << Self::REG_BUS_FREE_TIME_OFFSET) & Self::REG_BUS_FREE_TIME_MASK)
        });
    }
}

/// BUS_FREE_TIME value type for batch read/modify/write operations
///
/// Use `busfreetime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `busfreetime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BusFreeTimeVal {
    bits: u32,
}

impl BusFreeTimeVal {
    /// Create from raw bits
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
        Self { bits: 0x00000005 }
    }

    /// Read **REG_BUS_FREE_TIME** `[15:0]`
    ///
    /// I3C Bus Free Count Value. This field is used only in Master mode. In pure Bus System, this field represents tCAS. In Mixed Bus System, this field is expected to be programmed to tLOW of I2C Timing.
    #[inline(always)]
    pub const fn reg_bus_free_time(&self) -> u32 {
        (self.bits & BusFreeTime::REG_BUS_FREE_TIME_MASK) >> BusFreeTime::REG_BUS_FREE_TIME_OFFSET
    }

    /// Set **REG_BUS_FREE_TIME** `[15:0]`
    ///
    /// I3C Bus Free Count Value. This field is used only in Master mode. In pure Bus System, this field represents tCAS. In Mixed Bus System, this field is expected to be programmed to tLOW of I2C Timing.
    #[inline(always)]
    pub const fn with_reg_bus_free_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BusFreeTime::REG_BUS_FREE_TIME_MASK)
                | ((value << BusFreeTime::REG_BUS_FREE_TIME_OFFSET)
                    & BusFreeTime::REG_BUS_FREE_TIME_MASK),
        }
    }
}

impl Default for BusFreeTimeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BusFreeTime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BusFreeTimeVal {
        BusFreeTimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BusFreeTimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BusFreeTimeVal) -> BusFreeTimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SCL_TERMN_T_EXT_LOW_TIME Register
///
/// NA
///
/// Address offset: 0x0094
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_I3C_MST_TERMN_T_EXT_LOW_TIME** `[7:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct SclTermnTExtLowTime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SclTermnTExtLowTime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SclTermnTExtLowTime {
    // ======== Field Constants ========

    /// **REG_I3C_MST_TERMN_T_EXT_LOW_TIME**
    ///
    /// NA
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const REG_I3C_MST_TERMN_T_EXT_LOW_TIME_OFFSET: u32 = 0;
    pub const REG_I3C_MST_TERMN_T_EXT_LOW_TIME_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **REG_I3C_MST_TERMN_T_EXT_LOW_TIME** `[7:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_i3c_mst_termn_t_ext_low_time(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I3C_MST_TERMN_T_EXT_LOW_TIME_MASK)
            >> Self::REG_I3C_MST_TERMN_T_EXT_LOW_TIME_OFFSET
    }

    /// Write **REG_I3C_MST_TERMN_T_EXT_LOW_TIME** `[7:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_i3c_mst_termn_t_ext_low_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I3C_MST_TERMN_T_EXT_LOW_TIME_MASK)
                | ((value << Self::REG_I3C_MST_TERMN_T_EXT_LOW_TIME_OFFSET)
                    & Self::REG_I3C_MST_TERMN_T_EXT_LOW_TIME_MASK)
        });
    }
}

/// SCL_TERMN_T_EXT_LOW_TIME value type for batch read/modify/write operations
///
/// Use `scltermntextlowtime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `scltermntextlowtime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SclTermnTExtLowTimeVal {
    bits: u32,
}

impl SclTermnTExtLowTimeVal {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_I3C_MST_TERMN_T_EXT_LOW_TIME** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_i3c_mst_termn_t_ext_low_time(&self) -> u32 {
        (self.bits & SclTermnTExtLowTime::REG_I3C_MST_TERMN_T_EXT_LOW_TIME_MASK)
            >> SclTermnTExtLowTime::REG_I3C_MST_TERMN_T_EXT_LOW_TIME_OFFSET
    }

    /// Set **REG_I3C_MST_TERMN_T_EXT_LOW_TIME** `[7:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_i3c_mst_termn_t_ext_low_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SclTermnTExtLowTime::REG_I3C_MST_TERMN_T_EXT_LOW_TIME_MASK)
                | ((value << SclTermnTExtLowTime::REG_I3C_MST_TERMN_T_EXT_LOW_TIME_OFFSET)
                    & SclTermnTExtLowTime::REG_I3C_MST_TERMN_T_EXT_LOW_TIME_MASK),
        }
    }
}

impl Default for SclTermnTExtLowTimeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SclTermnTExtLowTime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SclTermnTExtLowTimeVal {
        SclTermnTExtLowTimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SclTermnTExtLowTimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SclTermnTExtLowTimeVal) -> SclTermnTExtLowTimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VER_ID Register
///
/// NA
///
/// Address offset: 0x00a0
/// Reset value: 0x20230504
///
/// ## Fields
///
/// - **REG_I3C_MST_VER_ID** `[31:0]` (RW)
///
///   This field indicates the controller current release number that is read by an application.
#[repr(transparent)]
pub struct VerId {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VerId {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VerId {
    // ======== Field Constants ========

    /// **REG_I3C_MST_VER_ID**
    ///
    /// This field indicates the controller current release number that is read by an application.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_I3C_MST_VER_ID_OFFSET: u32 = 0;
    pub const REG_I3C_MST_VER_ID_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_I3C_MST_VER_ID** `[31:0]` (RW)
    ///
    /// This field indicates the controller current release number that is read by an application.
    #[inline(always)]
    pub fn reg_i3c_mst_ver_id(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I3C_MST_VER_ID_MASK) >> Self::REG_I3C_MST_VER_ID_OFFSET
    }

    /// Write **REG_I3C_MST_VER_ID** `[31:0]` (RW)
    ///
    /// This field indicates the controller current release number that is read by an application.
    #[inline(always)]
    pub fn write_reg_i3c_mst_ver_id(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I3C_MST_VER_ID_MASK)
                | ((value << Self::REG_I3C_MST_VER_ID_OFFSET) & Self::REG_I3C_MST_VER_ID_MASK)
        });
    }
}

/// VER_ID value type for batch read/modify/write operations
///
/// Use `verid.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `verid.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VerIdVal {
    bits: u32,
}

impl VerIdVal {
    /// Create from raw bits
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
        Self { bits: 0x20230504 }
    }

    /// Read **REG_I3C_MST_VER_ID** `[31:0]`
    ///
    /// This field indicates the controller current release number that is read by an application.
    #[inline(always)]
    pub const fn reg_i3c_mst_ver_id(&self) -> u32 {
        (self.bits & VerId::REG_I3C_MST_VER_ID_MASK) >> VerId::REG_I3C_MST_VER_ID_OFFSET
    }

    /// Set **REG_I3C_MST_VER_ID** `[31:0]`
    ///
    /// This field indicates the controller current release number that is read by an application.
    #[inline(always)]
    pub const fn with_reg_i3c_mst_ver_id(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VerId::REG_I3C_MST_VER_ID_MASK)
                | ((value << VerId::REG_I3C_MST_VER_ID_OFFSET) & VerId::REG_I3C_MST_VER_ID_MASK),
        }
    }
}

impl Default for VerIdVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VerId {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VerIdVal {
        VerIdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VerIdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VerIdVal) -> VerIdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VER_TYPE Register
///
/// NA
///
/// Address offset: 0x00a4
///
/// ## Fields
///
/// - **REG_I3C_MST_VER_TYPE** `[31:0]` (RW)
///
///   This field indicates the controller current release type that is read by an application.
#[repr(transparent)]
pub struct VerType {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VerType {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VerType {
    // ======== Field Constants ========

    /// **REG_I3C_MST_VER_TYPE**
    ///
    /// This field indicates the controller current release type that is read by an application.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_I3C_MST_VER_TYPE_OFFSET: u32 = 0;
    pub const REG_I3C_MST_VER_TYPE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_I3C_MST_VER_TYPE** `[31:0]` (RW)
    ///
    /// This field indicates the controller current release type that is read by an application.
    #[inline(always)]
    pub fn reg_i3c_mst_ver_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I3C_MST_VER_TYPE_MASK)
            >> Self::REG_I3C_MST_VER_TYPE_OFFSET
    }

    /// Write **REG_I3C_MST_VER_TYPE** `[31:0]` (RW)
    ///
    /// This field indicates the controller current release type that is read by an application.
    #[inline(always)]
    pub fn write_reg_i3c_mst_ver_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I3C_MST_VER_TYPE_MASK)
                | ((value << Self::REG_I3C_MST_VER_TYPE_OFFSET) & Self::REG_I3C_MST_VER_TYPE_MASK)
        });
    }
}

/// VER_TYPE value type for batch read/modify/write operations
///
/// Use `vertype.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vertype.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VerTypeVal {
    bits: u32,
}

impl VerTypeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_I3C_MST_VER_TYPE** `[31:0]`
    ///
    /// This field indicates the controller current release type that is read by an application.
    #[inline(always)]
    pub const fn reg_i3c_mst_ver_type(&self) -> u32 {
        (self.bits & VerType::REG_I3C_MST_VER_TYPE_MASK) >> VerType::REG_I3C_MST_VER_TYPE_OFFSET
    }

    /// Set **REG_I3C_MST_VER_TYPE** `[31:0]`
    ///
    /// This field indicates the controller current release type that is read by an application.
    #[inline(always)]
    pub const fn with_reg_i3c_mst_ver_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VerType::REG_I3C_MST_VER_TYPE_MASK)
                | ((value << VerType::REG_I3C_MST_VER_TYPE_OFFSET)
                    & VerType::REG_I3C_MST_VER_TYPE_MASK),
        }
    }
}

impl Default for VerTypeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VerType {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VerTypeVal {
        VerTypeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VerTypeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VerTypeVal) -> VerTypeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FPGA_DEBUG_PROBE Register
///
/// NA
///
/// Address offset: 0x00ac
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **REG_I3C_MST_FPGA_DEBUG_PROBE** `[31:0]` (RW)
///
///   For Debug Probe Test on FPGA
#[repr(transparent)]
pub struct FpgaDebugProbe {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FpgaDebugProbe {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FpgaDebugProbe {
    // ======== Field Constants ========

    /// **REG_I3C_MST_FPGA_DEBUG_PROBE**
    ///
    /// For Debug Probe Test on FPGA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_I3C_MST_FPGA_DEBUG_PROBE_OFFSET: u32 = 0;
    pub const REG_I3C_MST_FPGA_DEBUG_PROBE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_I3C_MST_FPGA_DEBUG_PROBE** `[31:0]` (RW)
    ///
    /// For Debug Probe Test on FPGA
    #[inline(always)]
    pub fn reg_i3c_mst_fpga_debug_probe(&self) -> u32 {
        (self.value.read_bits() & Self::REG_I3C_MST_FPGA_DEBUG_PROBE_MASK)
            >> Self::REG_I3C_MST_FPGA_DEBUG_PROBE_OFFSET
    }

    /// Write **REG_I3C_MST_FPGA_DEBUG_PROBE** `[31:0]` (RW)
    ///
    /// For Debug Probe Test on FPGA
    #[inline(always)]
    pub fn write_reg_i3c_mst_fpga_debug_probe(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_I3C_MST_FPGA_DEBUG_PROBE_MASK)
                | ((value << Self::REG_I3C_MST_FPGA_DEBUG_PROBE_OFFSET)
                    & Self::REG_I3C_MST_FPGA_DEBUG_PROBE_MASK)
        });
    }
}

/// FPGA_DEBUG_PROBE value type for batch read/modify/write operations
///
/// Use `fpgadebugprobe.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fpgadebugprobe.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FpgaDebugProbeVal {
    bits: u32,
}

impl FpgaDebugProbeVal {
    /// Create from raw bits
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

    /// Read **REG_I3C_MST_FPGA_DEBUG_PROBE** `[31:0]`
    ///
    /// For Debug Probe Test on FPGA
    #[inline(always)]
    pub const fn reg_i3c_mst_fpga_debug_probe(&self) -> u32 {
        (self.bits & FpgaDebugProbe::REG_I3C_MST_FPGA_DEBUG_PROBE_MASK)
            >> FpgaDebugProbe::REG_I3C_MST_FPGA_DEBUG_PROBE_OFFSET
    }

    /// Set **REG_I3C_MST_FPGA_DEBUG_PROBE** `[31:0]`
    ///
    /// For Debug Probe Test on FPGA
    #[inline(always)]
    pub const fn with_reg_i3c_mst_fpga_debug_probe(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FpgaDebugProbe::REG_I3C_MST_FPGA_DEBUG_PROBE_MASK)
                | ((value << FpgaDebugProbe::REG_I3C_MST_FPGA_DEBUG_PROBE_OFFSET)
                    & FpgaDebugProbe::REG_I3C_MST_FPGA_DEBUG_PROBE_MASK),
        }
    }
}

impl Default for FpgaDebugProbeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl FpgaDebugProbe {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FpgaDebugProbeVal {
        FpgaDebugProbeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FpgaDebugProbeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FpgaDebugProbeVal) -> FpgaDebugProbeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RND_ECO_CS Register
///
/// NA
///
/// Address offset: 0x00b0
///
/// ## Fields
///
/// - **RND_ECO_RESULT** `[1]` (RO)
///
///   NA
///
/// - **REG_RND_ECO_EN** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct RndEcoCs {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RndEcoCs {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RndEcoCs {
    // ======== Field Constants ========

    /// **RND_ECO_RESULT**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RND_ECO_RESULT_OFFSET: u32 = 1;
    pub const RND_ECO_RESULT_MASK: u32 = 0x00000002;
    pub const RND_ECO_RESULT: u32 = 1 << 1;

    /// **REG_RND_ECO_EN**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_RND_ECO_EN_OFFSET: u32 = 0;
    pub const REG_RND_ECO_EN_MASK: u32 = 0x00000001;
    pub const REG_RND_ECO_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RND_ECO_RESULT** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn rnd_eco_result(&self) -> bool {
        (self.value.read_bits() & Self::RND_ECO_RESULT_MASK) != 0
    }

    /// **REG_RND_ECO_EN** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_rnd_eco_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_RND_ECO_EN_MASK) != 0
    }

    /// Set REG_RND_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_rnd_eco_en(&self) {
        self.value.modify_bits(|r| r | Self::REG_RND_ECO_EN_MASK);
    }

    /// Clear REG_RND_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_rnd_eco_en(&self) {
        self.value.modify_bits(|r| r & !Self::REG_RND_ECO_EN_MASK);
    }
}

/// RND_ECO_CS value type for batch read/modify/write operations
///
/// Use `rndecocs.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rndecocs.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RndEcoCsVal {
    bits: u32,
}

impl RndEcoCsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RND_ECO_RESULT** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn rnd_eco_result(&self) -> bool {
        (self.bits & RndEcoCs::RND_ECO_RESULT_MASK) != 0
    }

    /// Read **REG_RND_ECO_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_rnd_eco_en(&self) -> bool {
        (self.bits & RndEcoCs::REG_RND_ECO_EN_MASK) != 0
    }

    /// Set **REG_RND_ECO_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_rnd_eco_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RndEcoCs::REG_RND_ECO_EN_MASK
            } else {
                self.bits & !RndEcoCs::REG_RND_ECO_EN_MASK
            },
        }
    }
}

impl Default for RndEcoCsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RndEcoCs {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RndEcoCsVal {
        RndEcoCsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RndEcoCsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RndEcoCsVal) -> RndEcoCsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RND_ECO_LOW Register
///
/// NA
///
/// Address offset: 0x00b4
///
/// ## Fields
///
/// - **REG_RND_ECO_LOW** `[31:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct RndEcoLow {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RndEcoLow {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RndEcoLow {
    // ======== Field Constants ========

    /// **REG_RND_ECO_LOW**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_RND_ECO_LOW_OFFSET: u32 = 0;
    pub const REG_RND_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_RND_ECO_LOW** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_rnd_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::REG_RND_ECO_LOW_MASK) >> Self::REG_RND_ECO_LOW_OFFSET
    }

    /// Write **REG_RND_ECO_LOW** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_rnd_eco_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_RND_ECO_LOW_MASK)
                | ((value << Self::REG_RND_ECO_LOW_OFFSET) & Self::REG_RND_ECO_LOW_MASK)
        });
    }
}

/// RND_ECO_LOW value type for batch read/modify/write operations
///
/// Use `rndecolow.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rndecolow.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RndEcoLowVal {
    bits: u32,
}

impl RndEcoLowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_RND_ECO_LOW** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_rnd_eco_low(&self) -> u32 {
        (self.bits & RndEcoLow::REG_RND_ECO_LOW_MASK) >> RndEcoLow::REG_RND_ECO_LOW_OFFSET
    }

    /// Set **REG_RND_ECO_LOW** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_rnd_eco_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RndEcoLow::REG_RND_ECO_LOW_MASK)
                | ((value << RndEcoLow::REG_RND_ECO_LOW_OFFSET) & RndEcoLow::REG_RND_ECO_LOW_MASK),
        }
    }
}

impl Default for RndEcoLowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RndEcoLow {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RndEcoLowVal {
        RndEcoLowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RndEcoLowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RndEcoLowVal) -> RndEcoLowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RND_ECO_HIGH Register
///
/// NA
///
/// Address offset: 0x00b8
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **REG_RND_ECO_HIGH** `[31:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct RndEcoHigh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RndEcoHigh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RndEcoHigh {
    // ======== Field Constants ========

    /// **REG_RND_ECO_HIGH**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_RND_ECO_HIGH_OFFSET: u32 = 0;
    pub const REG_RND_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_RND_ECO_HIGH** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_rnd_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::REG_RND_ECO_HIGH_MASK) >> Self::REG_RND_ECO_HIGH_OFFSET
    }

    /// Write **REG_RND_ECO_HIGH** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_rnd_eco_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_RND_ECO_HIGH_MASK)
                | ((value << Self::REG_RND_ECO_HIGH_OFFSET) & Self::REG_RND_ECO_HIGH_MASK)
        });
    }
}

/// RND_ECO_HIGH value type for batch read/modify/write operations
///
/// Use `rndecohigh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rndecohigh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RndEcoHighVal {
    bits: u32,
}

impl RndEcoHighVal {
    /// Create from raw bits
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
        Self { bits: 0x0000ffff }
    }

    /// Read **REG_RND_ECO_HIGH** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_rnd_eco_high(&self) -> u32 {
        (self.bits & RndEcoHigh::REG_RND_ECO_HIGH_MASK) >> RndEcoHigh::REG_RND_ECO_HIGH_OFFSET
    }

    /// Set **REG_RND_ECO_HIGH** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_rnd_eco_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RndEcoHigh::REG_RND_ECO_HIGH_MASK)
                | ((value << RndEcoHigh::REG_RND_ECO_HIGH_OFFSET)
                    & RndEcoHigh::REG_RND_ECO_HIGH_MASK),
        }
    }
}

impl Default for RndEcoHighVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RndEcoHigh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RndEcoHighVal {
        RndEcoHighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RndEcoHighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RndEcoHighVal) -> RndEcoHighVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// DEVICE_CTRL register controls the transfer properties and disposition of controllers capabilities.
    pub device_ctrl: DeviceCtrl,
    _reserved0: [u8; 24],
    /// 0x001c
    /// In-Band Interrupt Status Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI status entries in the IBI buffer that trigger the IBI_STATUS_THLD_STAT interrupt.
    pub buffer_thld_ctrl: BufferThldCtrl,
    /// 0x0020
    /// NA
    pub data_buffer_thld_ctrl: DataBufferThldCtrl,
    /// 0x0024
    /// NA
    pub ibi_notify_ctrl: IbiNotifyCtrl,
    /// 0x0028
    /// NA
    pub ibi_sir_req_payload: IbiSirReqPayload,
    /// 0x002c
    /// NA
    pub ibi_sir_req_reject: IbiSirReqReject,
    /// 0x0030
    /// NA
    pub int_clr: IntClr,
    /// 0x0034
    /// NA
    pub int_raw: IntRaw,
    /// 0x0038
    /// NA
    pub int_st: IntSt,
    /// 0x003c
    /// The Interrupt status will be updated in INTR_STATUS register if corresponding Status Enable bit set.
    pub int_st_ena: IntStEna,
    _reserved1: [u8; 4],
    /// 0x0044
    /// NA
    pub reset_ctrl: ResetCtrl,
    /// 0x0048
    /// BUFFER_STATUS_LEVEL reflects the status level of Buffers in the controller.
    pub buffer_status_level: BufferStatusLevel,
    /// 0x004c
    /// DATA_BUFFER_STATUS_LEVEL reflects the status level of the Buffers in the controller.
    pub data_buffer_status_level: DataBufferStatusLevel,
    /// 0x0050
    /// NA
    pub present_state0: PresentState0,
    /// 0x0054
    /// NA
    pub present_state1: PresentState1,
    /// 0x0058
    /// Pointer for Device Address Table
    pub device_table: DeviceTable,
    /// 0x005c
    /// NA
    pub time_out_value: TimeOutValue,
    /// 0x0060
    /// NA
    pub scl_i3c_mst_od_time: SclI3cMstOdTime,
    /// 0x0064
    /// NA
    pub scl_i3c_mst_pp_time: SclI3cMstPpTime,
    /// 0x0068
    /// NA
    pub scl_i2c_fm_time: SclI2cFmTime,
    /// 0x006c
    /// NA
    pub scl_i2c_fmp_time: SclI2cFmpTime,
    /// 0x0070
    /// NA
    pub scl_ext_low_time: SclExtLowTime,
    /// 0x0074
    /// NA
    pub sda_sample_time: SdaSampleTime,
    /// 0x0078
    /// NA
    pub sda_hold_time: SdaHoldTime,
    /// 0x007c
    /// NA
    pub scl_start_hold: SclStartHold,
    /// 0x0080
    /// NA
    pub scl_rstart_setup: SclRstartSetup,
    /// 0x0084
    /// NA
    pub scl_stop_hold: SclStopHold,
    /// 0x0088
    /// NA
    pub scl_stop_setup: SclStopSetup,
    _reserved2: [u8; 4],
    /// 0x0090
    /// NA
    pub bus_free_time: BusFreeTime,
    /// 0x0094
    /// NA
    pub scl_termn_t_ext_low_time: SclTermnTExtLowTime,
    _reserved3: [u8; 8],
    /// 0x00a0
    /// NA
    pub ver_id: VerId,
    /// 0x00a4
    /// NA
    pub ver_type: VerType,
    _reserved4: [u8; 4],
    /// 0x00ac
    /// NA
    pub fpga_debug_probe: FpgaDebugProbe,
    /// 0x00b0
    /// NA
    pub rnd_eco_cs: RndEcoCs,
    /// 0x00b4
    /// NA
    pub rnd_eco_low: RndEcoLow,
    /// 0x00b8
    /// NA
    pub rnd_eco_high: RndEcoHigh,
}
