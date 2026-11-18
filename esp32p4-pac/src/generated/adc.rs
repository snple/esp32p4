//! ADC
//! ADC (Analog to Digital Converter)

#![allow(dead_code)]

pub const BASE: u32 = 0x500de000;

/// CTRL Register
///
/// Register
///
/// Address offset: 0x0000
/// Reset value: 0x403fc120
///
/// ## Fields
///
/// - **WAIT_ARB_CYCLE** `[31:30]` (RW)
///
///   wait arbit signal stable after sar_done
///
/// - **XPD_SAR2_FORCE** `[29:28]` (RW)
///
///   force option to xpd sar2 blocks
///
/// - **XPD_SAR1_FORCE** `[27:26]` (RW)
///
///   force option to xpd sar1 blocks
///
/// - **DATA_TO_I2S** `[25]` (RW)
///
///   1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix
///
/// - **DATA_SAR_SEL** `[24]` (RW)
///
///   1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits.
///
/// - **SAR2_PATT_P_CLEAR** `[23]` (RW)
///
///   clear the pointer of pattern table for DIG ADC2 CTRL
///
/// - **SAR1_PATT_P_CLEAR** `[22]` (RW)
///
///   clear the pointer of pattern table for DIG ADC1 CTRL
///
/// - **SAR2_PATT_LEN** `[21:18]` (RW)
///
///   0 ~ 15 means length 1 ~ 16
///
/// - **SAR1_PATT_LEN** `[17:14]` (RW)
///
///   0 ~ 15 means length 1 ~ 16
///
/// - **SAR_CLK_DIV** `[13:6]` (RW)
///
///   SAR clock divider
///
/// - **SAR_CLK_GATED** `[5]` (RW)
///
///   need_des
///
/// - **SAR_SEL** `[4]` (RW)
///
///   0: SAR1, 1: SAR2, only work for single SAR mode
///
/// - **WORK_MODE** `[3:2]` (RW)
///
///   0: single mode, 1: double mode, 2: alternate mode
///
/// - **START** `[1]` (RW)
///
///   need_des
///
/// - **START_FORCE** `[0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Ctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctrl {
    // ======== Field Constants ========

    /// **WAIT_ARB_CYCLE**
    ///
    /// wait arbit signal stable after sar_done
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const WAIT_ARB_CYCLE_OFFSET: u32 = 30;
    pub const WAIT_ARB_CYCLE_MASK: u32 = 0xc0000000;

    /// **XPD_SAR2_FORCE**
    ///
    /// force option to xpd sar2 blocks
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const XPD_SAR2_FORCE_OFFSET: u32 = 28;
    pub const XPD_SAR2_FORCE_MASK: u32 = 0x30000000;

    /// **XPD_SAR1_FORCE**
    ///
    /// force option to xpd sar1 blocks
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const XPD_SAR1_FORCE_OFFSET: u32 = 26;
    pub const XPD_SAR1_FORCE_MASK: u32 = 0x0c000000;

    /// **DATA_TO_I2S**
    ///
    /// 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const DATA_TO_I2S_OFFSET: u32 = 25;
    pub const DATA_TO_I2S_MASK: u32 = 0x02000000;
    pub const DATA_TO_I2S: u32 = 1 << 25;

    /// **DATA_SAR_SEL**
    ///
    /// 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const DATA_SAR_SEL_OFFSET: u32 = 24;
    pub const DATA_SAR_SEL_MASK: u32 = 0x01000000;
    pub const DATA_SAR_SEL: u32 = 1 << 24;

    /// **SAR2_PATT_P_CLEAR**
    ///
    /// clear the pointer of pattern table for DIG ADC2 CTRL
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const SAR2_PATT_P_CLEAR_OFFSET: u32 = 23;
    pub const SAR2_PATT_P_CLEAR_MASK: u32 = 0x00800000;
    pub const SAR2_PATT_P_CLEAR: u32 = 1 << 23;

    /// **SAR1_PATT_P_CLEAR**
    ///
    /// clear the pointer of pattern table for DIG ADC1 CTRL
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const SAR1_PATT_P_CLEAR_OFFSET: u32 = 22;
    pub const SAR1_PATT_P_CLEAR_MASK: u32 = 0x00400000;
    pub const SAR1_PATT_P_CLEAR: u32 = 1 << 22;

    /// **SAR2_PATT_LEN**
    ///
    /// 0 ~ 15 means length 1 ~ 16
    ///
    /// Bits `[21:18]`, mask `0x003c0000`
    pub const SAR2_PATT_LEN_OFFSET: u32 = 18;
    pub const SAR2_PATT_LEN_MASK: u32 = 0x003c0000;

    /// **SAR1_PATT_LEN**
    ///
    /// 0 ~ 15 means length 1 ~ 16
    ///
    /// Bits `[17:14]`, mask `0x0003c000`
    pub const SAR1_PATT_LEN_OFFSET: u32 = 14;
    pub const SAR1_PATT_LEN_MASK: u32 = 0x0003c000;

    /// **SAR_CLK_DIV**
    ///
    /// SAR clock divider
    ///
    /// Bits `[13:6]`, mask `0x00003fc0`
    pub const SAR_CLK_DIV_OFFSET: u32 = 6;
    pub const SAR_CLK_DIV_MASK: u32 = 0x00003fc0;

    /// **SAR_CLK_GATED**
    ///
    /// need_des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SAR_CLK_GATED_OFFSET: u32 = 5;
    pub const SAR_CLK_GATED_MASK: u32 = 0x00000020;
    pub const SAR_CLK_GATED: u32 = 1 << 5;

    /// **SAR_SEL**
    ///
    /// 0: SAR1, 1: SAR2, only work for single SAR mode
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SAR_SEL_OFFSET: u32 = 4;
    pub const SAR_SEL_MASK: u32 = 0x00000010;
    pub const SAR_SEL: u32 = 1 << 4;

    /// **WORK_MODE**
    ///
    /// 0: single mode, 1: double mode, 2: alternate mode
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const WORK_MODE_OFFSET: u32 = 2;
    pub const WORK_MODE_MASK: u32 = 0x0000000c;

    /// **START**
    ///
    /// need_des
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const START_OFFSET: u32 = 1;
    pub const START_MASK: u32 = 0x00000002;
    pub const START: u32 = 1 << 1;

    /// **START_FORCE**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const START_FORCE_OFFSET: u32 = 0;
    pub const START_FORCE_MASK: u32 = 0x00000001;
    pub const START_FORCE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **WAIT_ARB_CYCLE** `[31:30]` (RW)
    ///
    /// wait arbit signal stable after sar_done
    #[inline(always)]
    pub fn wait_arb_cycle(&self) -> u32 {
        (self.value.read_bits() & Self::WAIT_ARB_CYCLE_MASK) >> Self::WAIT_ARB_CYCLE_OFFSET
    }

    /// Write **WAIT_ARB_CYCLE** `[31:30]` (RW)
    ///
    /// wait arbit signal stable after sar_done
    #[inline(always)]
    pub fn write_wait_arb_cycle(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WAIT_ARB_CYCLE_MASK)
                | ((value << Self::WAIT_ARB_CYCLE_OFFSET) & Self::WAIT_ARB_CYCLE_MASK)
        });
    }

    /// **XPD_SAR2_FORCE** `[29:28]` (RW)
    ///
    /// force option to xpd sar2 blocks
    #[inline(always)]
    pub fn xpd_sar2_force(&self) -> u32 {
        (self.value.read_bits() & Self::XPD_SAR2_FORCE_MASK) >> Self::XPD_SAR2_FORCE_OFFSET
    }

    /// Write **XPD_SAR2_FORCE** `[29:28]` (RW)
    ///
    /// force option to xpd sar2 blocks
    #[inline(always)]
    pub fn write_xpd_sar2_force(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::XPD_SAR2_FORCE_MASK)
                | ((value << Self::XPD_SAR2_FORCE_OFFSET) & Self::XPD_SAR2_FORCE_MASK)
        });
    }

    /// **XPD_SAR1_FORCE** `[27:26]` (RW)
    ///
    /// force option to xpd sar1 blocks
    #[inline(always)]
    pub fn xpd_sar1_force(&self) -> u32 {
        (self.value.read_bits() & Self::XPD_SAR1_FORCE_MASK) >> Self::XPD_SAR1_FORCE_OFFSET
    }

    /// Write **XPD_SAR1_FORCE** `[27:26]` (RW)
    ///
    /// force option to xpd sar1 blocks
    #[inline(always)]
    pub fn write_xpd_sar1_force(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::XPD_SAR1_FORCE_MASK)
                | ((value << Self::XPD_SAR1_FORCE_OFFSET) & Self::XPD_SAR1_FORCE_MASK)
        });
    }

    /// **DATA_TO_I2S** `[25]` (RW)
    ///
    /// 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix
    #[inline(always)]
    pub fn data_to_i2s(&self) -> bool {
        (self.value.read_bits() & Self::DATA_TO_I2S_MASK) != 0
    }

    /// Set DATA_TO_I2S bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_data_to_i2s(&self) {
        self.value.modify_bits(|r| r | Self::DATA_TO_I2S_MASK);
    }

    /// Clear DATA_TO_I2S bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_data_to_i2s(&self) {
        self.value.modify_bits(|r| r & !Self::DATA_TO_I2S_MASK);
    }

    /// **DATA_SAR_SEL** `[24]` (RW)
    ///
    /// 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits.
    #[inline(always)]
    pub fn data_sar_sel(&self) -> bool {
        (self.value.read_bits() & Self::DATA_SAR_SEL_MASK) != 0
    }

    /// Set DATA_SAR_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_data_sar_sel(&self) {
        self.value.modify_bits(|r| r | Self::DATA_SAR_SEL_MASK);
    }

    /// Clear DATA_SAR_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_data_sar_sel(&self) {
        self.value.modify_bits(|r| r & !Self::DATA_SAR_SEL_MASK);
    }

    /// **SAR2_PATT_P_CLEAR** `[23]` (RW)
    ///
    /// clear the pointer of pattern table for DIG ADC2 CTRL
    #[inline(always)]
    pub fn sar2_patt_p_clear(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_PATT_P_CLEAR_MASK) != 0
    }

    /// Set SAR2_PATT_P_CLEAR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_patt_p_clear(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_PATT_P_CLEAR_MASK);
    }

    /// Clear SAR2_PATT_P_CLEAR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_patt_p_clear(&self) {
        self.value
            .modify_bits(|r| r & !Self::SAR2_PATT_P_CLEAR_MASK);
    }

    /// **SAR1_PATT_P_CLEAR** `[22]` (RW)
    ///
    /// clear the pointer of pattern table for DIG ADC1 CTRL
    #[inline(always)]
    pub fn sar1_patt_p_clear(&self) -> bool {
        (self.value.read_bits() & Self::SAR1_PATT_P_CLEAR_MASK) != 0
    }

    /// Set SAR1_PATT_P_CLEAR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar1_patt_p_clear(&self) {
        self.value.modify_bits(|r| r | Self::SAR1_PATT_P_CLEAR_MASK);
    }

    /// Clear SAR1_PATT_P_CLEAR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar1_patt_p_clear(&self) {
        self.value
            .modify_bits(|r| r & !Self::SAR1_PATT_P_CLEAR_MASK);
    }

    /// **SAR2_PATT_LEN** `[21:18]` (RW)
    ///
    /// 0 ~ 15 means length 1 ~ 16
    #[inline(always)]
    pub fn sar2_patt_len(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_PATT_LEN_MASK) >> Self::SAR2_PATT_LEN_OFFSET
    }

    /// Write **SAR2_PATT_LEN** `[21:18]` (RW)
    ///
    /// 0 ~ 15 means length 1 ~ 16
    #[inline(always)]
    pub fn write_sar2_patt_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_PATT_LEN_MASK)
                | ((value << Self::SAR2_PATT_LEN_OFFSET) & Self::SAR2_PATT_LEN_MASK)
        });
    }

    /// **SAR1_PATT_LEN** `[17:14]` (RW)
    ///
    /// 0 ~ 15 means length 1 ~ 16
    #[inline(always)]
    pub fn sar1_patt_len(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_PATT_LEN_MASK) >> Self::SAR1_PATT_LEN_OFFSET
    }

    /// Write **SAR1_PATT_LEN** `[17:14]` (RW)
    ///
    /// 0 ~ 15 means length 1 ~ 16
    #[inline(always)]
    pub fn write_sar1_patt_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR1_PATT_LEN_MASK)
                | ((value << Self::SAR1_PATT_LEN_OFFSET) & Self::SAR1_PATT_LEN_MASK)
        });
    }

    /// **SAR_CLK_DIV** `[13:6]` (RW)
    ///
    /// SAR clock divider
    #[inline(always)]
    pub fn sar_clk_div(&self) -> u32 {
        (self.value.read_bits() & Self::SAR_CLK_DIV_MASK) >> Self::SAR_CLK_DIV_OFFSET
    }

    /// Write **SAR_CLK_DIV** `[13:6]` (RW)
    ///
    /// SAR clock divider
    #[inline(always)]
    pub fn write_sar_clk_div(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR_CLK_DIV_MASK)
                | ((value << Self::SAR_CLK_DIV_OFFSET) & Self::SAR_CLK_DIV_MASK)
        });
    }

    /// **SAR_CLK_GATED** `[5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn sar_clk_gated(&self) -> bool {
        (self.value.read_bits() & Self::SAR_CLK_GATED_MASK) != 0
    }

    /// Set SAR_CLK_GATED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar_clk_gated(&self) {
        self.value.modify_bits(|r| r | Self::SAR_CLK_GATED_MASK);
    }

    /// Clear SAR_CLK_GATED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar_clk_gated(&self) {
        self.value.modify_bits(|r| r & !Self::SAR_CLK_GATED_MASK);
    }

    /// **SAR_SEL** `[4]` (RW)
    ///
    /// 0: SAR1, 1: SAR2, only work for single SAR mode
    #[inline(always)]
    pub fn sar_sel(&self) -> bool {
        (self.value.read_bits() & Self::SAR_SEL_MASK) != 0
    }

    /// Set SAR_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar_sel(&self) {
        self.value.modify_bits(|r| r | Self::SAR_SEL_MASK);
    }

    /// Clear SAR_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar_sel(&self) {
        self.value.modify_bits(|r| r & !Self::SAR_SEL_MASK);
    }

    /// **WORK_MODE** `[3:2]` (RW)
    ///
    /// 0: single mode, 1: double mode, 2: alternate mode
    #[inline(always)]
    pub fn work_mode(&self) -> u32 {
        (self.value.read_bits() & Self::WORK_MODE_MASK) >> Self::WORK_MODE_OFFSET
    }

    /// Write **WORK_MODE** `[3:2]` (RW)
    ///
    /// 0: single mode, 1: double mode, 2: alternate mode
    #[inline(always)]
    pub fn write_work_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WORK_MODE_MASK) | ((value << Self::WORK_MODE_OFFSET) & Self::WORK_MODE_MASK)
        });
    }

    /// **START** `[1]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn start(&self) -> bool {
        (self.value.read_bits() & Self::START_MASK) != 0
    }

    /// Set START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_start(&self) {
        self.value.modify_bits(|r| r | Self::START_MASK);
    }

    /// Clear START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_start(&self) {
        self.value.modify_bits(|r| r & !Self::START_MASK);
    }

    /// **START_FORCE** `[0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn start_force(&self) -> bool {
        (self.value.read_bits() & Self::START_FORCE_MASK) != 0
    }

    /// Set START_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_start_force(&self) {
        self.value.modify_bits(|r| r | Self::START_FORCE_MASK);
    }

    /// Clear START_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_start_force(&self) {
        self.value.modify_bits(|r| r & !Self::START_FORCE_MASK);
    }
}

/// CTRL value type for batch read/modify/write operations
///
/// Use `ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CtrlVal {
    bits: u32,
}

impl CtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x403fc120 }
    }

    /// Read **WAIT_ARB_CYCLE** `[31:30]`
    ///
    /// wait arbit signal stable after sar_done
    #[inline(always)]
    pub const fn wait_arb_cycle(&self) -> u32 {
        (self.bits & Ctrl::WAIT_ARB_CYCLE_MASK) >> Ctrl::WAIT_ARB_CYCLE_OFFSET
    }

    /// Read **XPD_SAR2_FORCE** `[29:28]`
    ///
    /// force option to xpd sar2 blocks
    #[inline(always)]
    pub const fn xpd_sar2_force(&self) -> u32 {
        (self.bits & Ctrl::XPD_SAR2_FORCE_MASK) >> Ctrl::XPD_SAR2_FORCE_OFFSET
    }

    /// Read **XPD_SAR1_FORCE** `[27:26]`
    ///
    /// force option to xpd sar1 blocks
    #[inline(always)]
    pub const fn xpd_sar1_force(&self) -> u32 {
        (self.bits & Ctrl::XPD_SAR1_FORCE_MASK) >> Ctrl::XPD_SAR1_FORCE_OFFSET
    }

    /// Read **DATA_TO_I2S** `[25:25]`
    ///
    /// 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix
    #[inline(always)]
    pub const fn data_to_i2s(&self) -> bool {
        (self.bits & Ctrl::DATA_TO_I2S_MASK) != 0
    }

    /// Read **DATA_SAR_SEL** `[24:24]`
    ///
    /// 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits.
    #[inline(always)]
    pub const fn data_sar_sel(&self) -> bool {
        (self.bits & Ctrl::DATA_SAR_SEL_MASK) != 0
    }

    /// Read **SAR2_PATT_P_CLEAR** `[23:23]`
    ///
    /// clear the pointer of pattern table for DIG ADC2 CTRL
    #[inline(always)]
    pub const fn sar2_patt_p_clear(&self) -> bool {
        (self.bits & Ctrl::SAR2_PATT_P_CLEAR_MASK) != 0
    }

    /// Read **SAR1_PATT_P_CLEAR** `[22:22]`
    ///
    /// clear the pointer of pattern table for DIG ADC1 CTRL
    #[inline(always)]
    pub const fn sar1_patt_p_clear(&self) -> bool {
        (self.bits & Ctrl::SAR1_PATT_P_CLEAR_MASK) != 0
    }

    /// Read **SAR2_PATT_LEN** `[21:18]`
    ///
    /// 0 ~ 15 means length 1 ~ 16
    #[inline(always)]
    pub const fn sar2_patt_len(&self) -> u32 {
        (self.bits & Ctrl::SAR2_PATT_LEN_MASK) >> Ctrl::SAR2_PATT_LEN_OFFSET
    }

    /// Read **SAR1_PATT_LEN** `[17:14]`
    ///
    /// 0 ~ 15 means length 1 ~ 16
    #[inline(always)]
    pub const fn sar1_patt_len(&self) -> u32 {
        (self.bits & Ctrl::SAR1_PATT_LEN_MASK) >> Ctrl::SAR1_PATT_LEN_OFFSET
    }

    /// Read **SAR_CLK_DIV** `[13:6]`
    ///
    /// SAR clock divider
    #[inline(always)]
    pub const fn sar_clk_div(&self) -> u32 {
        (self.bits & Ctrl::SAR_CLK_DIV_MASK) >> Ctrl::SAR_CLK_DIV_OFFSET
    }

    /// Read **SAR_CLK_GATED** `[5:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn sar_clk_gated(&self) -> bool {
        (self.bits & Ctrl::SAR_CLK_GATED_MASK) != 0
    }

    /// Read **SAR_SEL** `[4:4]`
    ///
    /// 0: SAR1, 1: SAR2, only work for single SAR mode
    #[inline(always)]
    pub const fn sar_sel(&self) -> bool {
        (self.bits & Ctrl::SAR_SEL_MASK) != 0
    }

    /// Read **WORK_MODE** `[3:2]`
    ///
    /// 0: single mode, 1: double mode, 2: alternate mode
    #[inline(always)]
    pub const fn work_mode(&self) -> u32 {
        (self.bits & Ctrl::WORK_MODE_MASK) >> Ctrl::WORK_MODE_OFFSET
    }

    /// Read **START** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn start(&self) -> bool {
        (self.bits & Ctrl::START_MASK) != 0
    }

    /// Read **START_FORCE** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn start_force(&self) -> bool {
        (self.bits & Ctrl::START_FORCE_MASK) != 0
    }

    /// Set **WAIT_ARB_CYCLE** `[31:30]`
    ///
    /// wait arbit signal stable after sar_done
    #[inline(always)]
    pub const fn with_wait_arb_cycle(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::WAIT_ARB_CYCLE_MASK)
                | ((value << Ctrl::WAIT_ARB_CYCLE_OFFSET) & Ctrl::WAIT_ARB_CYCLE_MASK),
        }
    }

    /// Set **XPD_SAR2_FORCE** `[29:28]`
    ///
    /// force option to xpd sar2 blocks
    #[inline(always)]
    pub const fn with_xpd_sar2_force(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::XPD_SAR2_FORCE_MASK)
                | ((value << Ctrl::XPD_SAR2_FORCE_OFFSET) & Ctrl::XPD_SAR2_FORCE_MASK),
        }
    }

    /// Set **XPD_SAR1_FORCE** `[27:26]`
    ///
    /// force option to xpd sar1 blocks
    #[inline(always)]
    pub const fn with_xpd_sar1_force(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::XPD_SAR1_FORCE_MASK)
                | ((value << Ctrl::XPD_SAR1_FORCE_OFFSET) & Ctrl::XPD_SAR1_FORCE_MASK),
        }
    }

    /// Set **DATA_TO_I2S** `[25:25]`
    ///
    /// 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix
    #[inline(always)]
    pub const fn with_data_to_i2s(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::DATA_TO_I2S_MASK
            } else {
                self.bits & !Ctrl::DATA_TO_I2S_MASK
            },
        }
    }

    /// Set **DATA_SAR_SEL** `[24:24]`
    ///
    /// 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits.
    #[inline(always)]
    pub const fn with_data_sar_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::DATA_SAR_SEL_MASK
            } else {
                self.bits & !Ctrl::DATA_SAR_SEL_MASK
            },
        }
    }

    /// Set **SAR2_PATT_P_CLEAR** `[23:23]`
    ///
    /// clear the pointer of pattern table for DIG ADC2 CTRL
    #[inline(always)]
    pub const fn with_sar2_patt_p_clear(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::SAR2_PATT_P_CLEAR_MASK
            } else {
                self.bits & !Ctrl::SAR2_PATT_P_CLEAR_MASK
            },
        }
    }

    /// Set **SAR1_PATT_P_CLEAR** `[22:22]`
    ///
    /// clear the pointer of pattern table for DIG ADC1 CTRL
    #[inline(always)]
    pub const fn with_sar1_patt_p_clear(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::SAR1_PATT_P_CLEAR_MASK
            } else {
                self.bits & !Ctrl::SAR1_PATT_P_CLEAR_MASK
            },
        }
    }

    /// Set **SAR2_PATT_LEN** `[21:18]`
    ///
    /// 0 ~ 15 means length 1 ~ 16
    #[inline(always)]
    pub const fn with_sar2_patt_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::SAR2_PATT_LEN_MASK)
                | ((value << Ctrl::SAR2_PATT_LEN_OFFSET) & Ctrl::SAR2_PATT_LEN_MASK),
        }
    }

    /// Set **SAR1_PATT_LEN** `[17:14]`
    ///
    /// 0 ~ 15 means length 1 ~ 16
    #[inline(always)]
    pub const fn with_sar1_patt_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::SAR1_PATT_LEN_MASK)
                | ((value << Ctrl::SAR1_PATT_LEN_OFFSET) & Ctrl::SAR1_PATT_LEN_MASK),
        }
    }

    /// Set **SAR_CLK_DIV** `[13:6]`
    ///
    /// SAR clock divider
    #[inline(always)]
    pub const fn with_sar_clk_div(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::SAR_CLK_DIV_MASK)
                | ((value << Ctrl::SAR_CLK_DIV_OFFSET) & Ctrl::SAR_CLK_DIV_MASK),
        }
    }

    /// Set **SAR_CLK_GATED** `[5:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_sar_clk_gated(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::SAR_CLK_GATED_MASK
            } else {
                self.bits & !Ctrl::SAR_CLK_GATED_MASK
            },
        }
    }

    /// Set **SAR_SEL** `[4:4]`
    ///
    /// 0: SAR1, 1: SAR2, only work for single SAR mode
    #[inline(always)]
    pub const fn with_sar_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::SAR_SEL_MASK
            } else {
                self.bits & !Ctrl::SAR_SEL_MASK
            },
        }
    }

    /// Set **WORK_MODE** `[3:2]`
    ///
    /// 0: single mode, 1: double mode, 2: alternate mode
    #[inline(always)]
    pub const fn with_work_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::WORK_MODE_MASK)
                | ((value << Ctrl::WORK_MODE_OFFSET) & Ctrl::WORK_MODE_MASK),
        }
    }

    /// Set **START** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::START_MASK
            } else {
                self.bits & !Ctrl::START_MASK
            },
        }
    }

    /// Set **START_FORCE** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_start_force(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::START_FORCE_MASK
            } else {
                self.bits & !Ctrl::START_FORCE_MASK
            },
        }
    }
}

impl Default for CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CtrlVal {
        CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CtrlVal) -> CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CTRL2 Register
///
/// Register
///
/// Address offset: 0x0004
/// Reset value: 0x0000a1fe
///
/// ## Fields
///
/// - **TIMER_EN** `[24]` (RW)
///
///   to enable saradc timer trigger
///
/// - **TIMER_TARGET** `[23:12]` (RW)
///
///   to set saradc timer target
///
/// - **TIMER_SEL** `[11]` (RW)
///
///   1: select saradc timer 0: i2s_ws trigger
///
/// - **SAR2_INV** `[10]` (RW)
///
///   1: data to DIG ADC2 CTRL is inverted, otherwise not
///
/// - **SAR1_INV** `[9]` (RW)
///
///   1: data to DIG ADC1 CTRL is inverted, otherwise not
///
/// - **MAX_MEAS_NUM** `[8:1]` (RW)
///
///   max conversion number
///
/// - **MEAS_NUM_LIMIT** `[0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Ctrl2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ctrl2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctrl2 {
    // ======== Field Constants ========

    /// **TIMER_EN**
    ///
    /// to enable saradc timer trigger
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TIMER_EN_OFFSET: u32 = 24;
    pub const TIMER_EN_MASK: u32 = 0x01000000;
    pub const TIMER_EN: u32 = 1 << 24;

    /// **TIMER_TARGET**
    ///
    /// to set saradc timer target
    ///
    /// Bits `[23:12]`, mask `0x00fff000`
    pub const TIMER_TARGET_OFFSET: u32 = 12;
    pub const TIMER_TARGET_MASK: u32 = 0x00fff000;

    /// **TIMER_SEL**
    ///
    /// 1: select saradc timer 0: i2s_ws trigger
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TIMER_SEL_OFFSET: u32 = 11;
    pub const TIMER_SEL_MASK: u32 = 0x00000800;
    pub const TIMER_SEL: u32 = 1 << 11;

    /// **SAR2_INV**
    ///
    /// 1: data to DIG ADC2 CTRL is inverted, otherwise not
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const SAR2_INV_OFFSET: u32 = 10;
    pub const SAR2_INV_MASK: u32 = 0x00000400;
    pub const SAR2_INV: u32 = 1 << 10;

    /// **SAR1_INV**
    ///
    /// 1: data to DIG ADC1 CTRL is inverted, otherwise not
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SAR1_INV_OFFSET: u32 = 9;
    pub const SAR1_INV_MASK: u32 = 0x00000200;
    pub const SAR1_INV: u32 = 1 << 9;

    /// **MAX_MEAS_NUM**
    ///
    /// max conversion number
    ///
    /// Bits `[8:1]`, mask `0x000001fe`
    pub const MAX_MEAS_NUM_OFFSET: u32 = 1;
    pub const MAX_MEAS_NUM_MASK: u32 = 0x000001fe;

    /// **MEAS_NUM_LIMIT**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MEAS_NUM_LIMIT_OFFSET: u32 = 0;
    pub const MEAS_NUM_LIMIT_MASK: u32 = 0x00000001;
    pub const MEAS_NUM_LIMIT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMER_EN** `[24]` (RW)
    ///
    /// to enable saradc timer trigger
    #[inline(always)]
    pub fn timer_en(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_EN_MASK) != 0
    }

    /// Set TIMER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_en(&self) {
        self.value.modify_bits(|r| r | Self::TIMER_EN_MASK);
    }

    /// Clear TIMER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_en(&self) {
        self.value.modify_bits(|r| r & !Self::TIMER_EN_MASK);
    }

    /// **TIMER_TARGET** `[23:12]` (RW)
    ///
    /// to set saradc timer target
    #[inline(always)]
    pub fn timer_target(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_TARGET_MASK) >> Self::TIMER_TARGET_OFFSET
    }

    /// Write **TIMER_TARGET** `[23:12]` (RW)
    ///
    /// to set saradc timer target
    #[inline(always)]
    pub fn write_timer_target(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_TARGET_MASK)
                | ((value << Self::TIMER_TARGET_OFFSET) & Self::TIMER_TARGET_MASK)
        });
    }

    /// **TIMER_SEL** `[11]` (RW)
    ///
    /// 1: select saradc timer 0: i2s_ws trigger
    #[inline(always)]
    pub fn timer_sel(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_SEL_MASK) != 0
    }

    /// Set TIMER_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_sel(&self) {
        self.value.modify_bits(|r| r | Self::TIMER_SEL_MASK);
    }

    /// Clear TIMER_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_sel(&self) {
        self.value.modify_bits(|r| r & !Self::TIMER_SEL_MASK);
    }

    /// **SAR2_INV** `[10]` (RW)
    ///
    /// 1: data to DIG ADC2 CTRL is inverted, otherwise not
    #[inline(always)]
    pub fn sar2_inv(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_INV_MASK) != 0
    }

    /// Set SAR2_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_inv(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_INV_MASK);
    }

    /// Clear SAR2_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_inv(&self) {
        self.value.modify_bits(|r| r & !Self::SAR2_INV_MASK);
    }

    /// **SAR1_INV** `[9]` (RW)
    ///
    /// 1: data to DIG ADC1 CTRL is inverted, otherwise not
    #[inline(always)]
    pub fn sar1_inv(&self) -> bool {
        (self.value.read_bits() & Self::SAR1_INV_MASK) != 0
    }

    /// Set SAR1_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar1_inv(&self) {
        self.value.modify_bits(|r| r | Self::SAR1_INV_MASK);
    }

    /// Clear SAR1_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar1_inv(&self) {
        self.value.modify_bits(|r| r & !Self::SAR1_INV_MASK);
    }

    /// **MAX_MEAS_NUM** `[8:1]` (RW)
    ///
    /// max conversion number
    #[inline(always)]
    pub fn max_meas_num(&self) -> u32 {
        (self.value.read_bits() & Self::MAX_MEAS_NUM_MASK) >> Self::MAX_MEAS_NUM_OFFSET
    }

    /// Write **MAX_MEAS_NUM** `[8:1]` (RW)
    ///
    /// max conversion number
    #[inline(always)]
    pub fn write_max_meas_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAX_MEAS_NUM_MASK)
                | ((value << Self::MAX_MEAS_NUM_OFFSET) & Self::MAX_MEAS_NUM_MASK)
        });
    }

    /// **MEAS_NUM_LIMIT** `[0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn meas_num_limit(&self) -> bool {
        (self.value.read_bits() & Self::MEAS_NUM_LIMIT_MASK) != 0
    }

    /// Set MEAS_NUM_LIMIT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_meas_num_limit(&self) {
        self.value.modify_bits(|r| r | Self::MEAS_NUM_LIMIT_MASK);
    }

    /// Clear MEAS_NUM_LIMIT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_meas_num_limit(&self) {
        self.value.modify_bits(|r| r & !Self::MEAS_NUM_LIMIT_MASK);
    }
}

/// CTRL2 value type for batch read/modify/write operations
///
/// Use `ctrl2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctrl2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ctrl2Val {
    bits: u32,
}

impl Ctrl2Val {
    /// Create from raw bits
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
        Self { bits: 0x0000a1fe }
    }

    /// Read **TIMER_EN** `[24:24]`
    ///
    /// to enable saradc timer trigger
    #[inline(always)]
    pub const fn timer_en(&self) -> bool {
        (self.bits & Ctrl2::TIMER_EN_MASK) != 0
    }

    /// Read **TIMER_TARGET** `[23:12]`
    ///
    /// to set saradc timer target
    #[inline(always)]
    pub const fn timer_target(&self) -> u32 {
        (self.bits & Ctrl2::TIMER_TARGET_MASK) >> Ctrl2::TIMER_TARGET_OFFSET
    }

    /// Read **TIMER_SEL** `[11:11]`
    ///
    /// 1: select saradc timer 0: i2s_ws trigger
    #[inline(always)]
    pub const fn timer_sel(&self) -> bool {
        (self.bits & Ctrl2::TIMER_SEL_MASK) != 0
    }

    /// Read **SAR2_INV** `[10:10]`
    ///
    /// 1: data to DIG ADC2 CTRL is inverted, otherwise not
    #[inline(always)]
    pub const fn sar2_inv(&self) -> bool {
        (self.bits & Ctrl2::SAR2_INV_MASK) != 0
    }

    /// Read **SAR1_INV** `[9:9]`
    ///
    /// 1: data to DIG ADC1 CTRL is inverted, otherwise not
    #[inline(always)]
    pub const fn sar1_inv(&self) -> bool {
        (self.bits & Ctrl2::SAR1_INV_MASK) != 0
    }

    /// Read **MAX_MEAS_NUM** `[8:1]`
    ///
    /// max conversion number
    #[inline(always)]
    pub const fn max_meas_num(&self) -> u32 {
        (self.bits & Ctrl2::MAX_MEAS_NUM_MASK) >> Ctrl2::MAX_MEAS_NUM_OFFSET
    }

    /// Read **MEAS_NUM_LIMIT** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn meas_num_limit(&self) -> bool {
        (self.bits & Ctrl2::MEAS_NUM_LIMIT_MASK) != 0
    }

    /// Set **TIMER_EN** `[24:24]`
    ///
    /// to enable saradc timer trigger
    #[inline(always)]
    pub const fn with_timer_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl2::TIMER_EN_MASK
            } else {
                self.bits & !Ctrl2::TIMER_EN_MASK
            },
        }
    }

    /// Set **TIMER_TARGET** `[23:12]`
    ///
    /// to set saradc timer target
    #[inline(always)]
    pub const fn with_timer_target(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl2::TIMER_TARGET_MASK)
                | ((value << Ctrl2::TIMER_TARGET_OFFSET) & Ctrl2::TIMER_TARGET_MASK),
        }
    }

    /// Set **TIMER_SEL** `[11:11]`
    ///
    /// 1: select saradc timer 0: i2s_ws trigger
    #[inline(always)]
    pub const fn with_timer_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl2::TIMER_SEL_MASK
            } else {
                self.bits & !Ctrl2::TIMER_SEL_MASK
            },
        }
    }

    /// Set **SAR2_INV** `[10:10]`
    ///
    /// 1: data to DIG ADC2 CTRL is inverted, otherwise not
    #[inline(always)]
    pub const fn with_sar2_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl2::SAR2_INV_MASK
            } else {
                self.bits & !Ctrl2::SAR2_INV_MASK
            },
        }
    }

    /// Set **SAR1_INV** `[9:9]`
    ///
    /// 1: data to DIG ADC1 CTRL is inverted, otherwise not
    #[inline(always)]
    pub const fn with_sar1_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl2::SAR1_INV_MASK
            } else {
                self.bits & !Ctrl2::SAR1_INV_MASK
            },
        }
    }

    /// Set **MAX_MEAS_NUM** `[8:1]`
    ///
    /// max conversion number
    #[inline(always)]
    pub const fn with_max_meas_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl2::MAX_MEAS_NUM_MASK)
                | ((value << Ctrl2::MAX_MEAS_NUM_OFFSET) & Ctrl2::MAX_MEAS_NUM_MASK),
        }
    }

    /// Set **MEAS_NUM_LIMIT** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_meas_num_limit(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl2::MEAS_NUM_LIMIT_MASK
            } else {
                self.bits & !Ctrl2::MEAS_NUM_LIMIT_MASK
            },
        }
    }
}

impl Default for Ctrl2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ctrl2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ctrl2Val {
        Ctrl2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ctrl2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ctrl2Val) -> Ctrl2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FILTER_CTRL1 Register
///
/// Register
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **FILTER_FACTOR0** `[31:29]` (RW)
///
///   need_des
///
/// - **FILTER_FACTOR1** `[28:26]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct FilterCtrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FilterCtrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FilterCtrl1 {
    // ======== Field Constants ========

    /// **FILTER_FACTOR0**
    ///
    /// need_des
    ///
    /// Bits `[31:29]`, mask `0xe0000000`
    pub const FILTER_FACTOR0_OFFSET: u32 = 29;
    pub const FILTER_FACTOR0_MASK: u32 = 0xe0000000;

    /// **FILTER_FACTOR1**
    ///
    /// need_des
    ///
    /// Bits `[28:26]`, mask `0x1c000000`
    pub const FILTER_FACTOR1_OFFSET: u32 = 26;
    pub const FILTER_FACTOR1_MASK: u32 = 0x1c000000;

    // ======== Field Accessors ========

    /// **FILTER_FACTOR0** `[31:29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn filter_factor0(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_FACTOR0_MASK) >> Self::FILTER_FACTOR0_OFFSET
    }

    /// Write **FILTER_FACTOR0** `[31:29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_filter_factor0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_FACTOR0_MASK)
                | ((value << Self::FILTER_FACTOR0_OFFSET) & Self::FILTER_FACTOR0_MASK)
        });
    }

    /// **FILTER_FACTOR1** `[28:26]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn filter_factor1(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_FACTOR1_MASK) >> Self::FILTER_FACTOR1_OFFSET
    }

    /// Write **FILTER_FACTOR1** `[28:26]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_filter_factor1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_FACTOR1_MASK)
                | ((value << Self::FILTER_FACTOR1_OFFSET) & Self::FILTER_FACTOR1_MASK)
        });
    }
}

/// FILTER_CTRL1 value type for batch read/modify/write operations
///
/// Use `filterctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `filterctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FilterCtrl1Val {
    bits: u32,
}

impl FilterCtrl1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FILTER_FACTOR0** `[31:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn filter_factor0(&self) -> u32 {
        (self.bits & FilterCtrl1::FILTER_FACTOR0_MASK) >> FilterCtrl1::FILTER_FACTOR0_OFFSET
    }

    /// Read **FILTER_FACTOR1** `[28:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn filter_factor1(&self) -> u32 {
        (self.bits & FilterCtrl1::FILTER_FACTOR1_MASK) >> FilterCtrl1::FILTER_FACTOR1_OFFSET
    }

    /// Set **FILTER_FACTOR0** `[31:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_filter_factor0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FilterCtrl1::FILTER_FACTOR0_MASK)
                | ((value << FilterCtrl1::FILTER_FACTOR0_OFFSET)
                    & FilterCtrl1::FILTER_FACTOR0_MASK),
        }
    }

    /// Set **FILTER_FACTOR1** `[28:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_filter_factor1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FilterCtrl1::FILTER_FACTOR1_MASK)
                | ((value << FilterCtrl1::FILTER_FACTOR1_OFFSET)
                    & FilterCtrl1::FILTER_FACTOR1_MASK),
        }
    }
}

impl Default for FilterCtrl1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FilterCtrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FilterCtrl1Val {
        FilterCtrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FilterCtrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FilterCtrl1Val) -> FilterCtrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FSM_WAIT Register
///
/// Register
///
/// Address offset: 0x000c
/// Reset value: 0x00ff0808
///
/// ## Fields
///
/// - **STANDBY_WAIT** `[23:16]` (RW)
///
///   need_des
///
/// - **RSTB_WAIT** `[15:8]` (RW)
///
///   need_des
///
/// - **XPD_WAIT** `[7:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct FsmWait {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FsmWait {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FsmWait {
    // ======== Field Constants ========

    /// **STANDBY_WAIT**
    ///
    /// need_des
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const STANDBY_WAIT_OFFSET: u32 = 16;
    pub const STANDBY_WAIT_MASK: u32 = 0x00ff0000;

    /// **RSTB_WAIT**
    ///
    /// need_des
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const RSTB_WAIT_OFFSET: u32 = 8;
    pub const RSTB_WAIT_MASK: u32 = 0x0000ff00;

    /// **XPD_WAIT**
    ///
    /// need_des
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const XPD_WAIT_OFFSET: u32 = 0;
    pub const XPD_WAIT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **STANDBY_WAIT** `[23:16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn standby_wait(&self) -> u32 {
        (self.value.read_bits() & Self::STANDBY_WAIT_MASK) >> Self::STANDBY_WAIT_OFFSET
    }

    /// Write **STANDBY_WAIT** `[23:16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_standby_wait(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::STANDBY_WAIT_MASK)
                | ((value << Self::STANDBY_WAIT_OFFSET) & Self::STANDBY_WAIT_MASK)
        });
    }

    /// **RSTB_WAIT** `[15:8]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rstb_wait(&self) -> u32 {
        (self.value.read_bits() & Self::RSTB_WAIT_MASK) >> Self::RSTB_WAIT_OFFSET
    }

    /// Write **RSTB_WAIT** `[15:8]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_rstb_wait(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RSTB_WAIT_MASK) | ((value << Self::RSTB_WAIT_OFFSET) & Self::RSTB_WAIT_MASK)
        });
    }

    /// **XPD_WAIT** `[7:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn xpd_wait(&self) -> u32 {
        (self.value.read_bits() & Self::XPD_WAIT_MASK) >> Self::XPD_WAIT_OFFSET
    }

    /// Write **XPD_WAIT** `[7:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_xpd_wait(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::XPD_WAIT_MASK) | ((value << Self::XPD_WAIT_OFFSET) & Self::XPD_WAIT_MASK)
        });
    }
}

/// FSM_WAIT value type for batch read/modify/write operations
///
/// Use `fsmwait.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `fsmwait.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FsmWaitVal {
    bits: u32,
}

impl FsmWaitVal {
    /// Create from raw bits
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
        Self { bits: 0x00ff0808 }
    }

    /// Read **STANDBY_WAIT** `[23:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn standby_wait(&self) -> u32 {
        (self.bits & FsmWait::STANDBY_WAIT_MASK) >> FsmWait::STANDBY_WAIT_OFFSET
    }

    /// Read **RSTB_WAIT** `[15:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rstb_wait(&self) -> u32 {
        (self.bits & FsmWait::RSTB_WAIT_MASK) >> FsmWait::RSTB_WAIT_OFFSET
    }

    /// Read **XPD_WAIT** `[7:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn xpd_wait(&self) -> u32 {
        (self.bits & FsmWait::XPD_WAIT_MASK) >> FsmWait::XPD_WAIT_OFFSET
    }

    /// Set **STANDBY_WAIT** `[23:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_standby_wait(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FsmWait::STANDBY_WAIT_MASK)
                | ((value << FsmWait::STANDBY_WAIT_OFFSET) & FsmWait::STANDBY_WAIT_MASK),
        }
    }

    /// Set **RSTB_WAIT** `[15:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rstb_wait(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FsmWait::RSTB_WAIT_MASK)
                | ((value << FsmWait::RSTB_WAIT_OFFSET) & FsmWait::RSTB_WAIT_MASK),
        }
    }

    /// Set **XPD_WAIT** `[7:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_xpd_wait(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FsmWait::XPD_WAIT_MASK)
                | ((value << FsmWait::XPD_WAIT_OFFSET) & FsmWait::XPD_WAIT_MASK),
        }
    }
}

impl Default for FsmWaitVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl FsmWait {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FsmWaitVal {
        FsmWaitVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FsmWaitVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FsmWaitVal) -> FsmWaitVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SAR1_STATUS Register
///
/// Register
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **SAR1_STATUS** `[31:0]` (RO)
#[repr(transparent)]
pub struct Sar1Status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sar1Status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sar1Status {
    // ======== Field Constants ========

    /// **SAR1_STATUS**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SAR1_STATUS_OFFSET: u32 = 0;
    pub const SAR1_STATUS_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SAR1_STATUS** `[31:0]` (RO)
    #[inline(always)]
    pub fn sar1_status(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_STATUS_MASK) >> Self::SAR1_STATUS_OFFSET
    }
}

/// SAR1_STATUS value type for batch read/modify/write operations
///
/// Use `sar1status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sar1status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sar1StatusVal {
    bits: u32,
}

impl Sar1StatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR1_STATUS** `[31:0]`
    #[inline(always)]
    pub const fn sar1_status(&self) -> u32 {
        (self.bits & Sar1Status::SAR1_STATUS_MASK) >> Sar1Status::SAR1_STATUS_OFFSET
    }
}

impl Default for Sar1StatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sar1Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sar1StatusVal {
        Sar1StatusVal::from_bits(self.value.read_bits())
    }
}

/// SAR2_STATUS Register
///
/// Register
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **SAR2_STATUS** `[31:0]` (RO)
#[repr(transparent)]
pub struct Sar2Status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sar2Status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sar2Status {
    // ======== Field Constants ========

    /// **SAR2_STATUS**
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SAR2_STATUS_OFFSET: u32 = 0;
    pub const SAR2_STATUS_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SAR2_STATUS** `[31:0]` (RO)
    #[inline(always)]
    pub fn sar2_status(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_STATUS_MASK) >> Self::SAR2_STATUS_OFFSET
    }
}

/// SAR2_STATUS value type for batch read/modify/write operations
///
/// Use `sar2status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sar2status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sar2StatusVal {
    bits: u32,
}

impl Sar2StatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR2_STATUS** `[31:0]`
    #[inline(always)]
    pub const fn sar2_status(&self) -> u32 {
        (self.bits & Sar2Status::SAR2_STATUS_MASK) >> Sar2Status::SAR2_STATUS_OFFSET
    }
}

impl Default for Sar2StatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sar2Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sar2StatusVal {
        Sar2StatusVal::from_bits(self.value.read_bits())
    }
}

/// SAR1_PATT_TAB1 Register
///
/// Register
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **SAR1_PATT_TAB1** `[23:0]` (RW)
///
///   item 0 ~ 3 for pattern table 1 (each item one byte)
#[repr(transparent)]
pub struct Sar1PattTab1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sar1PattTab1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sar1PattTab1 {
    // ======== Field Constants ========

    /// **SAR1_PATT_TAB1**
    ///
    /// item 0 ~ 3 for pattern table 1 (each item one byte)
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const SAR1_PATT_TAB1_OFFSET: u32 = 0;
    pub const SAR1_PATT_TAB1_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **SAR1_PATT_TAB1** `[23:0]` (RW)
    ///
    /// item 0 ~ 3 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub fn sar1_patt_tab1(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_PATT_TAB1_MASK) >> Self::SAR1_PATT_TAB1_OFFSET
    }

    /// Write **SAR1_PATT_TAB1** `[23:0]` (RW)
    ///
    /// item 0 ~ 3 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub fn write_sar1_patt_tab1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR1_PATT_TAB1_MASK)
                | ((value << Self::SAR1_PATT_TAB1_OFFSET) & Self::SAR1_PATT_TAB1_MASK)
        });
    }
}

/// SAR1_PATT_TAB1 value type for batch read/modify/write operations
///
/// Use `sar1patttab1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sar1patttab1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sar1PattTab1Val {
    bits: u32,
}

impl Sar1PattTab1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR1_PATT_TAB1** `[23:0]`
    ///
    /// item 0 ~ 3 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub const fn sar1_patt_tab1(&self) -> u32 {
        (self.bits & Sar1PattTab1::SAR1_PATT_TAB1_MASK) >> Sar1PattTab1::SAR1_PATT_TAB1_OFFSET
    }

    /// Set **SAR1_PATT_TAB1** `[23:0]`
    ///
    /// item 0 ~ 3 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub const fn with_sar1_patt_tab1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sar1PattTab1::SAR1_PATT_TAB1_MASK)
                | ((value << Sar1PattTab1::SAR1_PATT_TAB1_OFFSET)
                    & Sar1PattTab1::SAR1_PATT_TAB1_MASK),
        }
    }
}

impl Default for Sar1PattTab1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sar1PattTab1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sar1PattTab1Val {
        Sar1PattTab1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sar1PattTab1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sar1PattTab1Val) -> Sar1PattTab1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SAR1_PATT_TAB2 Register
///
/// Register
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **SAR1_PATT_TAB2** `[23:0]` (RW)
///
///   Item 4 ~ 7 for pattern table 1 (each item one byte)
#[repr(transparent)]
pub struct Sar1PattTab2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sar1PattTab2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sar1PattTab2 {
    // ======== Field Constants ========

    /// **SAR1_PATT_TAB2**
    ///
    /// Item 4 ~ 7 for pattern table 1 (each item one byte)
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const SAR1_PATT_TAB2_OFFSET: u32 = 0;
    pub const SAR1_PATT_TAB2_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **SAR1_PATT_TAB2** `[23:0]` (RW)
    ///
    /// Item 4 ~ 7 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub fn sar1_patt_tab2(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_PATT_TAB2_MASK) >> Self::SAR1_PATT_TAB2_OFFSET
    }

    /// Write **SAR1_PATT_TAB2** `[23:0]` (RW)
    ///
    /// Item 4 ~ 7 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub fn write_sar1_patt_tab2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR1_PATT_TAB2_MASK)
                | ((value << Self::SAR1_PATT_TAB2_OFFSET) & Self::SAR1_PATT_TAB2_MASK)
        });
    }
}

/// SAR1_PATT_TAB2 value type for batch read/modify/write operations
///
/// Use `sar1patttab2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sar1patttab2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sar1PattTab2Val {
    bits: u32,
}

impl Sar1PattTab2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR1_PATT_TAB2** `[23:0]`
    ///
    /// Item 4 ~ 7 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub const fn sar1_patt_tab2(&self) -> u32 {
        (self.bits & Sar1PattTab2::SAR1_PATT_TAB2_MASK) >> Sar1PattTab2::SAR1_PATT_TAB2_OFFSET
    }

    /// Set **SAR1_PATT_TAB2** `[23:0]`
    ///
    /// Item 4 ~ 7 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub const fn with_sar1_patt_tab2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sar1PattTab2::SAR1_PATT_TAB2_MASK)
                | ((value << Sar1PattTab2::SAR1_PATT_TAB2_OFFSET)
                    & Sar1PattTab2::SAR1_PATT_TAB2_MASK),
        }
    }
}

impl Default for Sar1PattTab2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sar1PattTab2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sar1PattTab2Val {
        Sar1PattTab2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sar1PattTab2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sar1PattTab2Val) -> Sar1PattTab2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SAR1_PATT_TAB3 Register
///
/// Register
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **SAR1_PATT_TAB3** `[23:0]` (RW)
///
///   Item 8 ~ 11 for pattern table 1 (each item one byte)
#[repr(transparent)]
pub struct Sar1PattTab3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sar1PattTab3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sar1PattTab3 {
    // ======== Field Constants ========

    /// **SAR1_PATT_TAB3**
    ///
    /// Item 8 ~ 11 for pattern table 1 (each item one byte)
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const SAR1_PATT_TAB3_OFFSET: u32 = 0;
    pub const SAR1_PATT_TAB3_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **SAR1_PATT_TAB3** `[23:0]` (RW)
    ///
    /// Item 8 ~ 11 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub fn sar1_patt_tab3(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_PATT_TAB3_MASK) >> Self::SAR1_PATT_TAB3_OFFSET
    }

    /// Write **SAR1_PATT_TAB3** `[23:0]` (RW)
    ///
    /// Item 8 ~ 11 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub fn write_sar1_patt_tab3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR1_PATT_TAB3_MASK)
                | ((value << Self::SAR1_PATT_TAB3_OFFSET) & Self::SAR1_PATT_TAB3_MASK)
        });
    }
}

/// SAR1_PATT_TAB3 value type for batch read/modify/write operations
///
/// Use `sar1patttab3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sar1patttab3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sar1PattTab3Val {
    bits: u32,
}

impl Sar1PattTab3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR1_PATT_TAB3** `[23:0]`
    ///
    /// Item 8 ~ 11 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub const fn sar1_patt_tab3(&self) -> u32 {
        (self.bits & Sar1PattTab3::SAR1_PATT_TAB3_MASK) >> Sar1PattTab3::SAR1_PATT_TAB3_OFFSET
    }

    /// Set **SAR1_PATT_TAB3** `[23:0]`
    ///
    /// Item 8 ~ 11 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub const fn with_sar1_patt_tab3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sar1PattTab3::SAR1_PATT_TAB3_MASK)
                | ((value << Sar1PattTab3::SAR1_PATT_TAB3_OFFSET)
                    & Sar1PattTab3::SAR1_PATT_TAB3_MASK),
        }
    }
}

impl Default for Sar1PattTab3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sar1PattTab3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sar1PattTab3Val {
        Sar1PattTab3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sar1PattTab3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sar1PattTab3Val) -> Sar1PattTab3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SAR1_PATT_TAB4 Register
///
/// Register
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **SAR1_PATT_TAB4** `[23:0]` (RW)
///
///   Item 12 ~ 15 for pattern table 1 (each item one byte)
#[repr(transparent)]
pub struct Sar1PattTab4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sar1PattTab4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sar1PattTab4 {
    // ======== Field Constants ========

    /// **SAR1_PATT_TAB4**
    ///
    /// Item 12 ~ 15 for pattern table 1 (each item one byte)
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const SAR1_PATT_TAB4_OFFSET: u32 = 0;
    pub const SAR1_PATT_TAB4_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **SAR1_PATT_TAB4** `[23:0]` (RW)
    ///
    /// Item 12 ~ 15 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub fn sar1_patt_tab4(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_PATT_TAB4_MASK) >> Self::SAR1_PATT_TAB4_OFFSET
    }

    /// Write **SAR1_PATT_TAB4** `[23:0]` (RW)
    ///
    /// Item 12 ~ 15 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub fn write_sar1_patt_tab4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR1_PATT_TAB4_MASK)
                | ((value << Self::SAR1_PATT_TAB4_OFFSET) & Self::SAR1_PATT_TAB4_MASK)
        });
    }
}

/// SAR1_PATT_TAB4 value type for batch read/modify/write operations
///
/// Use `sar1patttab4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sar1patttab4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sar1PattTab4Val {
    bits: u32,
}

impl Sar1PattTab4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR1_PATT_TAB4** `[23:0]`
    ///
    /// Item 12 ~ 15 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub const fn sar1_patt_tab4(&self) -> u32 {
        (self.bits & Sar1PattTab4::SAR1_PATT_TAB4_MASK) >> Sar1PattTab4::SAR1_PATT_TAB4_OFFSET
    }

    /// Set **SAR1_PATT_TAB4** `[23:0]`
    ///
    /// Item 12 ~ 15 for pattern table 1 (each item one byte)
    #[inline(always)]
    pub const fn with_sar1_patt_tab4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sar1PattTab4::SAR1_PATT_TAB4_MASK)
                | ((value << Sar1PattTab4::SAR1_PATT_TAB4_OFFSET)
                    & Sar1PattTab4::SAR1_PATT_TAB4_MASK),
        }
    }
}

impl Default for Sar1PattTab4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sar1PattTab4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sar1PattTab4Val {
        Sar1PattTab4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sar1PattTab4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sar1PattTab4Val) -> Sar1PattTab4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SAR2_PATT_TAB1 Register
///
/// Register
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **SAR2_PATT_TAB1** `[23:0]` (RW)
///
///   item 0 ~ 3 for pattern table 2 (each item one byte)
#[repr(transparent)]
pub struct Sar2PattTab1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sar2PattTab1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sar2PattTab1 {
    // ======== Field Constants ========

    /// **SAR2_PATT_TAB1**
    ///
    /// item 0 ~ 3 for pattern table 2 (each item one byte)
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const SAR2_PATT_TAB1_OFFSET: u32 = 0;
    pub const SAR2_PATT_TAB1_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **SAR2_PATT_TAB1** `[23:0]` (RW)
    ///
    /// item 0 ~ 3 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub fn sar2_patt_tab1(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_PATT_TAB1_MASK) >> Self::SAR2_PATT_TAB1_OFFSET
    }

    /// Write **SAR2_PATT_TAB1** `[23:0]` (RW)
    ///
    /// item 0 ~ 3 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub fn write_sar2_patt_tab1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_PATT_TAB1_MASK)
                | ((value << Self::SAR2_PATT_TAB1_OFFSET) & Self::SAR2_PATT_TAB1_MASK)
        });
    }
}

/// SAR2_PATT_TAB1 value type for batch read/modify/write operations
///
/// Use `sar2patttab1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sar2patttab1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sar2PattTab1Val {
    bits: u32,
}

impl Sar2PattTab1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR2_PATT_TAB1** `[23:0]`
    ///
    /// item 0 ~ 3 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub const fn sar2_patt_tab1(&self) -> u32 {
        (self.bits & Sar2PattTab1::SAR2_PATT_TAB1_MASK) >> Sar2PattTab1::SAR2_PATT_TAB1_OFFSET
    }

    /// Set **SAR2_PATT_TAB1** `[23:0]`
    ///
    /// item 0 ~ 3 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub const fn with_sar2_patt_tab1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sar2PattTab1::SAR2_PATT_TAB1_MASK)
                | ((value << Sar2PattTab1::SAR2_PATT_TAB1_OFFSET)
                    & Sar2PattTab1::SAR2_PATT_TAB1_MASK),
        }
    }
}

impl Default for Sar2PattTab1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sar2PattTab1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sar2PattTab1Val {
        Sar2PattTab1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sar2PattTab1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sar2PattTab1Val) -> Sar2PattTab1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SAR2_PATT_TAB2 Register
///
/// Register
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **SAR2_PATT_TAB2** `[23:0]` (RW)
///
///   Item 4 ~ 7 for pattern table 2 (each item one byte)
#[repr(transparent)]
pub struct Sar2PattTab2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sar2PattTab2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sar2PattTab2 {
    // ======== Field Constants ========

    /// **SAR2_PATT_TAB2**
    ///
    /// Item 4 ~ 7 for pattern table 2 (each item one byte)
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const SAR2_PATT_TAB2_OFFSET: u32 = 0;
    pub const SAR2_PATT_TAB2_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **SAR2_PATT_TAB2** `[23:0]` (RW)
    ///
    /// Item 4 ~ 7 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub fn sar2_patt_tab2(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_PATT_TAB2_MASK) >> Self::SAR2_PATT_TAB2_OFFSET
    }

    /// Write **SAR2_PATT_TAB2** `[23:0]` (RW)
    ///
    /// Item 4 ~ 7 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub fn write_sar2_patt_tab2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_PATT_TAB2_MASK)
                | ((value << Self::SAR2_PATT_TAB2_OFFSET) & Self::SAR2_PATT_TAB2_MASK)
        });
    }
}

/// SAR2_PATT_TAB2 value type for batch read/modify/write operations
///
/// Use `sar2patttab2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sar2patttab2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sar2PattTab2Val {
    bits: u32,
}

impl Sar2PattTab2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR2_PATT_TAB2** `[23:0]`
    ///
    /// Item 4 ~ 7 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub const fn sar2_patt_tab2(&self) -> u32 {
        (self.bits & Sar2PattTab2::SAR2_PATT_TAB2_MASK) >> Sar2PattTab2::SAR2_PATT_TAB2_OFFSET
    }

    /// Set **SAR2_PATT_TAB2** `[23:0]`
    ///
    /// Item 4 ~ 7 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub const fn with_sar2_patt_tab2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sar2PattTab2::SAR2_PATT_TAB2_MASK)
                | ((value << Sar2PattTab2::SAR2_PATT_TAB2_OFFSET)
                    & Sar2PattTab2::SAR2_PATT_TAB2_MASK),
        }
    }
}

impl Default for Sar2PattTab2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sar2PattTab2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sar2PattTab2Val {
        Sar2PattTab2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sar2PattTab2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sar2PattTab2Val) -> Sar2PattTab2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SAR2_PATT_TAB3 Register
///
/// Register
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **SAR2_PATT_TAB3** `[23:0]` (RW)
///
///   Item 8 ~ 11 for pattern table 2 (each item one byte)
#[repr(transparent)]
pub struct Sar2PattTab3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sar2PattTab3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sar2PattTab3 {
    // ======== Field Constants ========

    /// **SAR2_PATT_TAB3**
    ///
    /// Item 8 ~ 11 for pattern table 2 (each item one byte)
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const SAR2_PATT_TAB3_OFFSET: u32 = 0;
    pub const SAR2_PATT_TAB3_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **SAR2_PATT_TAB3** `[23:0]` (RW)
    ///
    /// Item 8 ~ 11 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub fn sar2_patt_tab3(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_PATT_TAB3_MASK) >> Self::SAR2_PATT_TAB3_OFFSET
    }

    /// Write **SAR2_PATT_TAB3** `[23:0]` (RW)
    ///
    /// Item 8 ~ 11 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub fn write_sar2_patt_tab3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_PATT_TAB3_MASK)
                | ((value << Self::SAR2_PATT_TAB3_OFFSET) & Self::SAR2_PATT_TAB3_MASK)
        });
    }
}

/// SAR2_PATT_TAB3 value type for batch read/modify/write operations
///
/// Use `sar2patttab3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sar2patttab3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sar2PattTab3Val {
    bits: u32,
}

impl Sar2PattTab3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR2_PATT_TAB3** `[23:0]`
    ///
    /// Item 8 ~ 11 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub const fn sar2_patt_tab3(&self) -> u32 {
        (self.bits & Sar2PattTab3::SAR2_PATT_TAB3_MASK) >> Sar2PattTab3::SAR2_PATT_TAB3_OFFSET
    }

    /// Set **SAR2_PATT_TAB3** `[23:0]`
    ///
    /// Item 8 ~ 11 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub const fn with_sar2_patt_tab3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sar2PattTab3::SAR2_PATT_TAB3_MASK)
                | ((value << Sar2PattTab3::SAR2_PATT_TAB3_OFFSET)
                    & Sar2PattTab3::SAR2_PATT_TAB3_MASK),
        }
    }
}

impl Default for Sar2PattTab3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sar2PattTab3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sar2PattTab3Val {
        Sar2PattTab3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sar2PattTab3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sar2PattTab3Val) -> Sar2PattTab3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SAR2_PATT_TAB4 Register
///
/// Register
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **SAR2_PATT_TAB4** `[23:0]` (RW)
///
///   Item 12 ~ 15 for pattern table 2 (each item one byte)
#[repr(transparent)]
pub struct Sar2PattTab4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sar2PattTab4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sar2PattTab4 {
    // ======== Field Constants ========

    /// **SAR2_PATT_TAB4**
    ///
    /// Item 12 ~ 15 for pattern table 2 (each item one byte)
    ///
    /// Bits `[23:0]`, mask `0x00ffffff`
    pub const SAR2_PATT_TAB4_OFFSET: u32 = 0;
    pub const SAR2_PATT_TAB4_MASK: u32 = 0x00ffffff;

    // ======== Field Accessors ========

    /// **SAR2_PATT_TAB4** `[23:0]` (RW)
    ///
    /// Item 12 ~ 15 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub fn sar2_patt_tab4(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_PATT_TAB4_MASK) >> Self::SAR2_PATT_TAB4_OFFSET
    }

    /// Write **SAR2_PATT_TAB4** `[23:0]` (RW)
    ///
    /// Item 12 ~ 15 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub fn write_sar2_patt_tab4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_PATT_TAB4_MASK)
                | ((value << Self::SAR2_PATT_TAB4_OFFSET) & Self::SAR2_PATT_TAB4_MASK)
        });
    }
}

/// SAR2_PATT_TAB4 value type for batch read/modify/write operations
///
/// Use `sar2patttab4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sar2patttab4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sar2PattTab4Val {
    bits: u32,
}

impl Sar2PattTab4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR2_PATT_TAB4** `[23:0]`
    ///
    /// Item 12 ~ 15 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub const fn sar2_patt_tab4(&self) -> u32 {
        (self.bits & Sar2PattTab4::SAR2_PATT_TAB4_MASK) >> Sar2PattTab4::SAR2_PATT_TAB4_OFFSET
    }

    /// Set **SAR2_PATT_TAB4** `[23:0]`
    ///
    /// Item 12 ~ 15 for pattern table 2 (each item one byte)
    #[inline(always)]
    pub const fn with_sar2_patt_tab4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sar2PattTab4::SAR2_PATT_TAB4_MASK)
                | ((value << Sar2PattTab4::SAR2_PATT_TAB4_OFFSET)
                    & Sar2PattTab4::SAR2_PATT_TAB4_MASK),
        }
    }
}

impl Default for Sar2PattTab4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sar2PattTab4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sar2PattTab4Val {
        Sar2PattTab4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sar2PattTab4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sar2PattTab4Val) -> Sar2PattTab4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ARB_CTRL Register
///
/// Register
///
/// Address offset: 0x0038
/// Reset value: 0x00000900
///
/// ## Fields
///
/// - **ARB_FIX_PRIORITY** `[12]` (RW)
///
///   adc2 arbiter uses fixed priority
///
/// - **ARB_WIFI_PRIORITY** `[11:10]` (RW)
///
///   Set adc2 arbiter wifi priority
///
/// - **ARB_RTC_PRIORITY** `[9:8]` (RW)
///
///   Set adc2 arbiter rtc priority
///
/// - **ARB_APB_PRIORITY** `[7:6]` (RW)
///
///   Set adc2 arbiterapb priority
///
/// - **ARB_GRANT_FORCE** `[5]` (RW)
///
///   adc2 arbiter force grant
///
/// - **ARB_WIFI_FORCE** `[4]` (RW)
///
///   adc2 arbiter force to enable wifi controller
///
/// - **ARB_RTC_FORCE** `[3]` (RW)
///
///   adc2 arbiter force to enable rtc controller
///
/// - **ARB_APB_FORCE** `[2]` (RW)
///
///   adc2 arbiter force to enableapb controller
#[repr(transparent)]
pub struct ArbCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ArbCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ArbCtrl {
    // ======== Field Constants ========

    /// **ARB_FIX_PRIORITY**
    ///
    /// adc2 arbiter uses fixed priority
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const ARB_FIX_PRIORITY_OFFSET: u32 = 12;
    pub const ARB_FIX_PRIORITY_MASK: u32 = 0x00001000;
    pub const ARB_FIX_PRIORITY: u32 = 1 << 12;

    /// **ARB_WIFI_PRIORITY**
    ///
    /// Set adc2 arbiter wifi priority
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const ARB_WIFI_PRIORITY_OFFSET: u32 = 10;
    pub const ARB_WIFI_PRIORITY_MASK: u32 = 0x00000c00;

    /// **ARB_RTC_PRIORITY**
    ///
    /// Set adc2 arbiter rtc priority
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const ARB_RTC_PRIORITY_OFFSET: u32 = 8;
    pub const ARB_RTC_PRIORITY_MASK: u32 = 0x00000300;

    /// **ARB_APB_PRIORITY**
    ///
    /// Set adc2 arbiterapb priority
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const ARB_APB_PRIORITY_OFFSET: u32 = 6;
    pub const ARB_APB_PRIORITY_MASK: u32 = 0x000000c0;

    /// **ARB_GRANT_FORCE**
    ///
    /// adc2 arbiter force grant
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ARB_GRANT_FORCE_OFFSET: u32 = 5;
    pub const ARB_GRANT_FORCE_MASK: u32 = 0x00000020;
    pub const ARB_GRANT_FORCE: u32 = 1 << 5;

    /// **ARB_WIFI_FORCE**
    ///
    /// adc2 arbiter force to enable wifi controller
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const ARB_WIFI_FORCE_OFFSET: u32 = 4;
    pub const ARB_WIFI_FORCE_MASK: u32 = 0x00000010;
    pub const ARB_WIFI_FORCE: u32 = 1 << 4;

    /// **ARB_RTC_FORCE**
    ///
    /// adc2 arbiter force to enable rtc controller
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const ARB_RTC_FORCE_OFFSET: u32 = 3;
    pub const ARB_RTC_FORCE_MASK: u32 = 0x00000008;
    pub const ARB_RTC_FORCE: u32 = 1 << 3;

    /// **ARB_APB_FORCE**
    ///
    /// adc2 arbiter force to enableapb controller
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ARB_APB_FORCE_OFFSET: u32 = 2;
    pub const ARB_APB_FORCE_MASK: u32 = 0x00000004;
    pub const ARB_APB_FORCE: u32 = 1 << 2;

    // ======== Field Accessors ========

    /// **ARB_FIX_PRIORITY** `[12]` (RW)
    ///
    /// adc2 arbiter uses fixed priority
    #[inline(always)]
    pub fn arb_fix_priority(&self) -> bool {
        (self.value.read_bits() & Self::ARB_FIX_PRIORITY_MASK) != 0
    }

    /// Set ARB_FIX_PRIORITY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_arb_fix_priority(&self) {
        self.value.modify_bits(|r| r | Self::ARB_FIX_PRIORITY_MASK);
    }

    /// Clear ARB_FIX_PRIORITY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_arb_fix_priority(&self) {
        self.value.modify_bits(|r| r & !Self::ARB_FIX_PRIORITY_MASK);
    }

    /// **ARB_WIFI_PRIORITY** `[11:10]` (RW)
    ///
    /// Set adc2 arbiter wifi priority
    #[inline(always)]
    pub fn arb_wifi_priority(&self) -> u32 {
        (self.value.read_bits() & Self::ARB_WIFI_PRIORITY_MASK) >> Self::ARB_WIFI_PRIORITY_OFFSET
    }

    /// Write **ARB_WIFI_PRIORITY** `[11:10]` (RW)
    ///
    /// Set adc2 arbiter wifi priority
    #[inline(always)]
    pub fn write_arb_wifi_priority(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ARB_WIFI_PRIORITY_MASK)
                | ((value << Self::ARB_WIFI_PRIORITY_OFFSET) & Self::ARB_WIFI_PRIORITY_MASK)
        });
    }

    /// **ARB_RTC_PRIORITY** `[9:8]` (RW)
    ///
    /// Set adc2 arbiter rtc priority
    #[inline(always)]
    pub fn arb_rtc_priority(&self) -> u32 {
        (self.value.read_bits() & Self::ARB_RTC_PRIORITY_MASK) >> Self::ARB_RTC_PRIORITY_OFFSET
    }

    /// Write **ARB_RTC_PRIORITY** `[9:8]` (RW)
    ///
    /// Set adc2 arbiter rtc priority
    #[inline(always)]
    pub fn write_arb_rtc_priority(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ARB_RTC_PRIORITY_MASK)
                | ((value << Self::ARB_RTC_PRIORITY_OFFSET) & Self::ARB_RTC_PRIORITY_MASK)
        });
    }

    /// **ARB_APB_PRIORITY** `[7:6]` (RW)
    ///
    /// Set adc2 arbiterapb priority
    #[inline(always)]
    pub fn arb_apb_priority(&self) -> u32 {
        (self.value.read_bits() & Self::ARB_APB_PRIORITY_MASK) >> Self::ARB_APB_PRIORITY_OFFSET
    }

    /// Write **ARB_APB_PRIORITY** `[7:6]` (RW)
    ///
    /// Set adc2 arbiterapb priority
    #[inline(always)]
    pub fn write_arb_apb_priority(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ARB_APB_PRIORITY_MASK)
                | ((value << Self::ARB_APB_PRIORITY_OFFSET) & Self::ARB_APB_PRIORITY_MASK)
        });
    }

    /// **ARB_GRANT_FORCE** `[5]` (RW)
    ///
    /// adc2 arbiter force grant
    #[inline(always)]
    pub fn arb_grant_force(&self) -> bool {
        (self.value.read_bits() & Self::ARB_GRANT_FORCE_MASK) != 0
    }

    /// Set ARB_GRANT_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_arb_grant_force(&self) {
        self.value.modify_bits(|r| r | Self::ARB_GRANT_FORCE_MASK);
    }

    /// Clear ARB_GRANT_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_arb_grant_force(&self) {
        self.value.modify_bits(|r| r & !Self::ARB_GRANT_FORCE_MASK);
    }

    /// **ARB_WIFI_FORCE** `[4]` (RW)
    ///
    /// adc2 arbiter force to enable wifi controller
    #[inline(always)]
    pub fn arb_wifi_force(&self) -> bool {
        (self.value.read_bits() & Self::ARB_WIFI_FORCE_MASK) != 0
    }

    /// Set ARB_WIFI_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_arb_wifi_force(&self) {
        self.value.modify_bits(|r| r | Self::ARB_WIFI_FORCE_MASK);
    }

    /// Clear ARB_WIFI_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_arb_wifi_force(&self) {
        self.value.modify_bits(|r| r & !Self::ARB_WIFI_FORCE_MASK);
    }

    /// **ARB_RTC_FORCE** `[3]` (RW)
    ///
    /// adc2 arbiter force to enable rtc controller
    #[inline(always)]
    pub fn arb_rtc_force(&self) -> bool {
        (self.value.read_bits() & Self::ARB_RTC_FORCE_MASK) != 0
    }

    /// Set ARB_RTC_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_arb_rtc_force(&self) {
        self.value.modify_bits(|r| r | Self::ARB_RTC_FORCE_MASK);
    }

    /// Clear ARB_RTC_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_arb_rtc_force(&self) {
        self.value.modify_bits(|r| r & !Self::ARB_RTC_FORCE_MASK);
    }

    /// **ARB_APB_FORCE** `[2]` (RW)
    ///
    /// adc2 arbiter force to enableapb controller
    #[inline(always)]
    pub fn arb_apb_force(&self) -> bool {
        (self.value.read_bits() & Self::ARB_APB_FORCE_MASK) != 0
    }

    /// Set ARB_APB_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_arb_apb_force(&self) {
        self.value.modify_bits(|r| r | Self::ARB_APB_FORCE_MASK);
    }

    /// Clear ARB_APB_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_arb_apb_force(&self) {
        self.value.modify_bits(|r| r & !Self::ARB_APB_FORCE_MASK);
    }
}

/// ARB_CTRL value type for batch read/modify/write operations
///
/// Use `arbctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `arbctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ArbCtrlVal {
    bits: u32,
}

impl ArbCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00000900 }
    }

    /// Read **ARB_FIX_PRIORITY** `[12:12]`
    ///
    /// adc2 arbiter uses fixed priority
    #[inline(always)]
    pub const fn arb_fix_priority(&self) -> bool {
        (self.bits & ArbCtrl::ARB_FIX_PRIORITY_MASK) != 0
    }

    /// Read **ARB_WIFI_PRIORITY** `[11:10]`
    ///
    /// Set adc2 arbiter wifi priority
    #[inline(always)]
    pub const fn arb_wifi_priority(&self) -> u32 {
        (self.bits & ArbCtrl::ARB_WIFI_PRIORITY_MASK) >> ArbCtrl::ARB_WIFI_PRIORITY_OFFSET
    }

    /// Read **ARB_RTC_PRIORITY** `[9:8]`
    ///
    /// Set adc2 arbiter rtc priority
    #[inline(always)]
    pub const fn arb_rtc_priority(&self) -> u32 {
        (self.bits & ArbCtrl::ARB_RTC_PRIORITY_MASK) >> ArbCtrl::ARB_RTC_PRIORITY_OFFSET
    }

    /// Read **ARB_APB_PRIORITY** `[7:6]`
    ///
    /// Set adc2 arbiterapb priority
    #[inline(always)]
    pub const fn arb_apb_priority(&self) -> u32 {
        (self.bits & ArbCtrl::ARB_APB_PRIORITY_MASK) >> ArbCtrl::ARB_APB_PRIORITY_OFFSET
    }

    /// Read **ARB_GRANT_FORCE** `[5:5]`
    ///
    /// adc2 arbiter force grant
    #[inline(always)]
    pub const fn arb_grant_force(&self) -> bool {
        (self.bits & ArbCtrl::ARB_GRANT_FORCE_MASK) != 0
    }

    /// Read **ARB_WIFI_FORCE** `[4:4]`
    ///
    /// adc2 arbiter force to enable wifi controller
    #[inline(always)]
    pub const fn arb_wifi_force(&self) -> bool {
        (self.bits & ArbCtrl::ARB_WIFI_FORCE_MASK) != 0
    }

    /// Read **ARB_RTC_FORCE** `[3:3]`
    ///
    /// adc2 arbiter force to enable rtc controller
    #[inline(always)]
    pub const fn arb_rtc_force(&self) -> bool {
        (self.bits & ArbCtrl::ARB_RTC_FORCE_MASK) != 0
    }

    /// Read **ARB_APB_FORCE** `[2:2]`
    ///
    /// adc2 arbiter force to enableapb controller
    #[inline(always)]
    pub const fn arb_apb_force(&self) -> bool {
        (self.bits & ArbCtrl::ARB_APB_FORCE_MASK) != 0
    }

    /// Set **ARB_FIX_PRIORITY** `[12:12]`
    ///
    /// adc2 arbiter uses fixed priority
    #[inline(always)]
    pub const fn with_arb_fix_priority(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ArbCtrl::ARB_FIX_PRIORITY_MASK
            } else {
                self.bits & !ArbCtrl::ARB_FIX_PRIORITY_MASK
            },
        }
    }

    /// Set **ARB_WIFI_PRIORITY** `[11:10]`
    ///
    /// Set adc2 arbiter wifi priority
    #[inline(always)]
    pub const fn with_arb_wifi_priority(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ArbCtrl::ARB_WIFI_PRIORITY_MASK)
                | ((value << ArbCtrl::ARB_WIFI_PRIORITY_OFFSET) & ArbCtrl::ARB_WIFI_PRIORITY_MASK),
        }
    }

    /// Set **ARB_RTC_PRIORITY** `[9:8]`
    ///
    /// Set adc2 arbiter rtc priority
    #[inline(always)]
    pub const fn with_arb_rtc_priority(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ArbCtrl::ARB_RTC_PRIORITY_MASK)
                | ((value << ArbCtrl::ARB_RTC_PRIORITY_OFFSET) & ArbCtrl::ARB_RTC_PRIORITY_MASK),
        }
    }

    /// Set **ARB_APB_PRIORITY** `[7:6]`
    ///
    /// Set adc2 arbiterapb priority
    #[inline(always)]
    pub const fn with_arb_apb_priority(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ArbCtrl::ARB_APB_PRIORITY_MASK)
                | ((value << ArbCtrl::ARB_APB_PRIORITY_OFFSET) & ArbCtrl::ARB_APB_PRIORITY_MASK),
        }
    }

    /// Set **ARB_GRANT_FORCE** `[5:5]`
    ///
    /// adc2 arbiter force grant
    #[inline(always)]
    pub const fn with_arb_grant_force(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ArbCtrl::ARB_GRANT_FORCE_MASK
            } else {
                self.bits & !ArbCtrl::ARB_GRANT_FORCE_MASK
            },
        }
    }

    /// Set **ARB_WIFI_FORCE** `[4:4]`
    ///
    /// adc2 arbiter force to enable wifi controller
    #[inline(always)]
    pub const fn with_arb_wifi_force(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ArbCtrl::ARB_WIFI_FORCE_MASK
            } else {
                self.bits & !ArbCtrl::ARB_WIFI_FORCE_MASK
            },
        }
    }

    /// Set **ARB_RTC_FORCE** `[3:3]`
    ///
    /// adc2 arbiter force to enable rtc controller
    #[inline(always)]
    pub const fn with_arb_rtc_force(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ArbCtrl::ARB_RTC_FORCE_MASK
            } else {
                self.bits & !ArbCtrl::ARB_RTC_FORCE_MASK
            },
        }
    }

    /// Set **ARB_APB_FORCE** `[2:2]`
    ///
    /// adc2 arbiter force to enableapb controller
    #[inline(always)]
    pub const fn with_arb_apb_force(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ArbCtrl::ARB_APB_FORCE_MASK
            } else {
                self.bits & !ArbCtrl::ARB_APB_FORCE_MASK
            },
        }
    }
}

impl Default for ArbCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ArbCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ArbCtrlVal {
        ArbCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ArbCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ArbCtrlVal) -> ArbCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FILTER_CTRL0 Register
///
/// Register
///
/// Address offset: 0x003c
/// Reset value: 0x006b4000
///
/// ## Fields
///
/// - **FILTER_RESET** `[31]` (RW)
///
///   enable apb_adc1_filter
///
/// - **FILTER_CHANNEL0** `[23:19]` (RW)
///
///   apb_adc1_filter_factor
///
/// - **FILTER_CHANNEL1** `[18:14]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct FilterCtrl0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FilterCtrl0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FilterCtrl0 {
    // ======== Field Constants ========

    /// **FILTER_RESET**
    ///
    /// enable apb_adc1_filter
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const FILTER_RESET_OFFSET: u32 = 31;
    pub const FILTER_RESET_MASK: u32 = 0x80000000;
    pub const FILTER_RESET: u32 = 1 << 31;

    /// **FILTER_CHANNEL0**
    ///
    /// apb_adc1_filter_factor
    ///
    /// Bits `[23:19]`, mask `0x00f80000`
    pub const FILTER_CHANNEL0_OFFSET: u32 = 19;
    pub const FILTER_CHANNEL0_MASK: u32 = 0x00f80000;

    /// **FILTER_CHANNEL1**
    ///
    /// need_des
    ///
    /// Bits `[18:14]`, mask `0x0007c000`
    pub const FILTER_CHANNEL1_OFFSET: u32 = 14;
    pub const FILTER_CHANNEL1_MASK: u32 = 0x0007c000;

    // ======== Field Accessors ========

    /// **FILTER_RESET** `[31]` (RW)
    ///
    /// enable apb_adc1_filter
    #[inline(always)]
    pub fn filter_reset(&self) -> bool {
        (self.value.read_bits() & Self::FILTER_RESET_MASK) != 0
    }

    /// Set FILTER_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_filter_reset(&self) {
        self.value.modify_bits(|r| r | Self::FILTER_RESET_MASK);
    }

    /// Clear FILTER_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_filter_reset(&self) {
        self.value.modify_bits(|r| r & !Self::FILTER_RESET_MASK);
    }

    /// **FILTER_CHANNEL0** `[23:19]` (RW)
    ///
    /// apb_adc1_filter_factor
    #[inline(always)]
    pub fn filter_channel0(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CHANNEL0_MASK) >> Self::FILTER_CHANNEL0_OFFSET
    }

    /// Write **FILTER_CHANNEL0** `[23:19]` (RW)
    ///
    /// apb_adc1_filter_factor
    #[inline(always)]
    pub fn write_filter_channel0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CHANNEL0_MASK)
                | ((value << Self::FILTER_CHANNEL0_OFFSET) & Self::FILTER_CHANNEL0_MASK)
        });
    }

    /// **FILTER_CHANNEL1** `[18:14]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn filter_channel1(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CHANNEL1_MASK) >> Self::FILTER_CHANNEL1_OFFSET
    }

    /// Write **FILTER_CHANNEL1** `[18:14]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_filter_channel1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CHANNEL1_MASK)
                | ((value << Self::FILTER_CHANNEL1_OFFSET) & Self::FILTER_CHANNEL1_MASK)
        });
    }
}

/// FILTER_CTRL0 value type for batch read/modify/write operations
///
/// Use `filterctrl0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `filterctrl0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FilterCtrl0Val {
    bits: u32,
}

impl FilterCtrl0Val {
    /// Create from raw bits
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
        Self { bits: 0x006b4000 }
    }

    /// Read **FILTER_RESET** `[31:31]`
    ///
    /// enable apb_adc1_filter
    #[inline(always)]
    pub const fn filter_reset(&self) -> bool {
        (self.bits & FilterCtrl0::FILTER_RESET_MASK) != 0
    }

    /// Read **FILTER_CHANNEL0** `[23:19]`
    ///
    /// apb_adc1_filter_factor
    #[inline(always)]
    pub const fn filter_channel0(&self) -> u32 {
        (self.bits & FilterCtrl0::FILTER_CHANNEL0_MASK) >> FilterCtrl0::FILTER_CHANNEL0_OFFSET
    }

    /// Read **FILTER_CHANNEL1** `[18:14]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn filter_channel1(&self) -> u32 {
        (self.bits & FilterCtrl0::FILTER_CHANNEL1_MASK) >> FilterCtrl0::FILTER_CHANNEL1_OFFSET
    }

    /// Set **FILTER_RESET** `[31:31]`
    ///
    /// enable apb_adc1_filter
    #[inline(always)]
    pub const fn with_filter_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | FilterCtrl0::FILTER_RESET_MASK
            } else {
                self.bits & !FilterCtrl0::FILTER_RESET_MASK
            },
        }
    }

    /// Set **FILTER_CHANNEL0** `[23:19]`
    ///
    /// apb_adc1_filter_factor
    #[inline(always)]
    pub const fn with_filter_channel0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FilterCtrl0::FILTER_CHANNEL0_MASK)
                | ((value << FilterCtrl0::FILTER_CHANNEL0_OFFSET)
                    & FilterCtrl0::FILTER_CHANNEL0_MASK),
        }
    }

    /// Set **FILTER_CHANNEL1** `[18:14]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_filter_channel1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FilterCtrl0::FILTER_CHANNEL1_MASK)
                | ((value << FilterCtrl0::FILTER_CHANNEL1_OFFSET)
                    & FilterCtrl0::FILTER_CHANNEL1_MASK),
        }
    }
}

impl Default for FilterCtrl0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl FilterCtrl0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FilterCtrl0Val {
        FilterCtrl0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FilterCtrl0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FilterCtrl0Val) -> FilterCtrl0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SAR1_DATA_STATUS Register
///
/// Register
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **APB_SARADC1_DATA** `[16:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Sar1DataStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sar1DataStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sar1DataStatus {
    // ======== Field Constants ========

    /// **APB_SARADC1_DATA**
    ///
    /// need_des
    ///
    /// Bits `[16:0]`, mask `0x0001ffff`
    pub const APB_SARADC1_DATA_OFFSET: u32 = 0;
    pub const APB_SARADC1_DATA_MASK: u32 = 0x0001ffff;

    // ======== Field Accessors ========

    /// **APB_SARADC1_DATA** `[16:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn apb_saradc1_data(&self) -> u32 {
        (self.value.read_bits() & Self::APB_SARADC1_DATA_MASK) >> Self::APB_SARADC1_DATA_OFFSET
    }
}

/// SAR1_DATA_STATUS value type for batch read/modify/write operations
///
/// Use `sar1datastatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sar1datastatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sar1DataStatusVal {
    bits: u32,
}

impl Sar1DataStatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB_SARADC1_DATA** `[16:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn apb_saradc1_data(&self) -> u32 {
        (self.bits & Sar1DataStatus::APB_SARADC1_DATA_MASK)
            >> Sar1DataStatus::APB_SARADC1_DATA_OFFSET
    }
}

impl Default for Sar1DataStatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sar1DataStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sar1DataStatusVal {
        Sar1DataStatusVal::from_bits(self.value.read_bits())
    }
}

/// THRES0_CTRL Register
///
/// Register
///
/// Address offset: 0x0044
/// Reset value: 0x0003ffed
///
/// ## Fields
///
/// - **THRES0_LOW** `[30:18]` (RW)
///
///   saradc1's thres0 monitor thres
///
/// - **THRES0_HIGH** `[17:5]` (RW)
///
///   saradc1's thres0 monitor thres
///
/// - **THRES0_CHANNEL** `[4:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Thres0Ctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Thres0Ctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Thres0Ctrl {
    // ======== Field Constants ========

    /// **THRES0_LOW**
    ///
    /// saradc1's thres0 monitor thres
    ///
    /// Bits `[30:18]`, mask `0x7ffc0000`
    pub const THRES0_LOW_OFFSET: u32 = 18;
    pub const THRES0_LOW_MASK: u32 = 0x7ffc0000;

    /// **THRES0_HIGH**
    ///
    /// saradc1's thres0 monitor thres
    ///
    /// Bits `[17:5]`, mask `0x0003ffe0`
    pub const THRES0_HIGH_OFFSET: u32 = 5;
    pub const THRES0_HIGH_MASK: u32 = 0x0003ffe0;

    /// **THRES0_CHANNEL**
    ///
    /// need_des
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const THRES0_CHANNEL_OFFSET: u32 = 0;
    pub const THRES0_CHANNEL_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **THRES0_LOW** `[30:18]` (RW)
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub fn thres0_low(&self) -> u32 {
        (self.value.read_bits() & Self::THRES0_LOW_MASK) >> Self::THRES0_LOW_OFFSET
    }

    /// Write **THRES0_LOW** `[30:18]` (RW)
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub fn write_thres0_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::THRES0_LOW_MASK)
                | ((value << Self::THRES0_LOW_OFFSET) & Self::THRES0_LOW_MASK)
        });
    }

    /// **THRES0_HIGH** `[17:5]` (RW)
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub fn thres0_high(&self) -> u32 {
        (self.value.read_bits() & Self::THRES0_HIGH_MASK) >> Self::THRES0_HIGH_OFFSET
    }

    /// Write **THRES0_HIGH** `[17:5]` (RW)
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub fn write_thres0_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::THRES0_HIGH_MASK)
                | ((value << Self::THRES0_HIGH_OFFSET) & Self::THRES0_HIGH_MASK)
        });
    }

    /// **THRES0_CHANNEL** `[4:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres0_channel(&self) -> u32 {
        (self.value.read_bits() & Self::THRES0_CHANNEL_MASK) >> Self::THRES0_CHANNEL_OFFSET
    }

    /// Write **THRES0_CHANNEL** `[4:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_thres0_channel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::THRES0_CHANNEL_MASK)
                | ((value << Self::THRES0_CHANNEL_OFFSET) & Self::THRES0_CHANNEL_MASK)
        });
    }
}

/// THRES0_CTRL value type for batch read/modify/write operations
///
/// Use `thres0ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `thres0ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Thres0CtrlVal {
    bits: u32,
}

impl Thres0CtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x0003ffed }
    }

    /// Read **THRES0_LOW** `[30:18]`
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub const fn thres0_low(&self) -> u32 {
        (self.bits & Thres0Ctrl::THRES0_LOW_MASK) >> Thres0Ctrl::THRES0_LOW_OFFSET
    }

    /// Read **THRES0_HIGH** `[17:5]`
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub const fn thres0_high(&self) -> u32 {
        (self.bits & Thres0Ctrl::THRES0_HIGH_MASK) >> Thres0Ctrl::THRES0_HIGH_OFFSET
    }

    /// Read **THRES0_CHANNEL** `[4:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres0_channel(&self) -> u32 {
        (self.bits & Thres0Ctrl::THRES0_CHANNEL_MASK) >> Thres0Ctrl::THRES0_CHANNEL_OFFSET
    }

    /// Set **THRES0_LOW** `[30:18]`
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub const fn with_thres0_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Thres0Ctrl::THRES0_LOW_MASK)
                | ((value << Thres0Ctrl::THRES0_LOW_OFFSET) & Thres0Ctrl::THRES0_LOW_MASK),
        }
    }

    /// Set **THRES0_HIGH** `[17:5]`
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub const fn with_thres0_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Thres0Ctrl::THRES0_HIGH_MASK)
                | ((value << Thres0Ctrl::THRES0_HIGH_OFFSET) & Thres0Ctrl::THRES0_HIGH_MASK),
        }
    }

    /// Set **THRES0_CHANNEL** `[4:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres0_channel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Thres0Ctrl::THRES0_CHANNEL_MASK)
                | ((value << Thres0Ctrl::THRES0_CHANNEL_OFFSET) & Thres0Ctrl::THRES0_CHANNEL_MASK),
        }
    }
}

impl Default for Thres0CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Thres0Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Thres0CtrlVal {
        Thres0CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Thres0CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Thres0CtrlVal) -> Thres0CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// THRES1_CTRL Register
///
/// Register
///
/// Address offset: 0x0048
/// Reset value: 0x0003ffed
///
/// ## Fields
///
/// - **THRES1_LOW** `[30:18]` (RW)
///
///   saradc1's thres0 monitor thres
///
/// - **THRES1_HIGH** `[17:5]` (RW)
///
///   saradc1's thres0 monitor thres
///
/// - **THRES1_CHANNEL** `[4:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Thres1Ctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Thres1Ctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Thres1Ctrl {
    // ======== Field Constants ========

    /// **THRES1_LOW**
    ///
    /// saradc1's thres0 monitor thres
    ///
    /// Bits `[30:18]`, mask `0x7ffc0000`
    pub const THRES1_LOW_OFFSET: u32 = 18;
    pub const THRES1_LOW_MASK: u32 = 0x7ffc0000;

    /// **THRES1_HIGH**
    ///
    /// saradc1's thres0 monitor thres
    ///
    /// Bits `[17:5]`, mask `0x0003ffe0`
    pub const THRES1_HIGH_OFFSET: u32 = 5;
    pub const THRES1_HIGH_MASK: u32 = 0x0003ffe0;

    /// **THRES1_CHANNEL**
    ///
    /// need_des
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const THRES1_CHANNEL_OFFSET: u32 = 0;
    pub const THRES1_CHANNEL_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **THRES1_LOW** `[30:18]` (RW)
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub fn thres1_low(&self) -> u32 {
        (self.value.read_bits() & Self::THRES1_LOW_MASK) >> Self::THRES1_LOW_OFFSET
    }

    /// Write **THRES1_LOW** `[30:18]` (RW)
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub fn write_thres1_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::THRES1_LOW_MASK)
                | ((value << Self::THRES1_LOW_OFFSET) & Self::THRES1_LOW_MASK)
        });
    }

    /// **THRES1_HIGH** `[17:5]` (RW)
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub fn thres1_high(&self) -> u32 {
        (self.value.read_bits() & Self::THRES1_HIGH_MASK) >> Self::THRES1_HIGH_OFFSET
    }

    /// Write **THRES1_HIGH** `[17:5]` (RW)
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub fn write_thres1_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::THRES1_HIGH_MASK)
                | ((value << Self::THRES1_HIGH_OFFSET) & Self::THRES1_HIGH_MASK)
        });
    }

    /// **THRES1_CHANNEL** `[4:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres1_channel(&self) -> u32 {
        (self.value.read_bits() & Self::THRES1_CHANNEL_MASK) >> Self::THRES1_CHANNEL_OFFSET
    }

    /// Write **THRES1_CHANNEL** `[4:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_thres1_channel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::THRES1_CHANNEL_MASK)
                | ((value << Self::THRES1_CHANNEL_OFFSET) & Self::THRES1_CHANNEL_MASK)
        });
    }
}

/// THRES1_CTRL value type for batch read/modify/write operations
///
/// Use `thres1ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `thres1ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Thres1CtrlVal {
    bits: u32,
}

impl Thres1CtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x0003ffed }
    }

    /// Read **THRES1_LOW** `[30:18]`
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub const fn thres1_low(&self) -> u32 {
        (self.bits & Thres1Ctrl::THRES1_LOW_MASK) >> Thres1Ctrl::THRES1_LOW_OFFSET
    }

    /// Read **THRES1_HIGH** `[17:5]`
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub const fn thres1_high(&self) -> u32 {
        (self.bits & Thres1Ctrl::THRES1_HIGH_MASK) >> Thres1Ctrl::THRES1_HIGH_OFFSET
    }

    /// Read **THRES1_CHANNEL** `[4:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres1_channel(&self) -> u32 {
        (self.bits & Thres1Ctrl::THRES1_CHANNEL_MASK) >> Thres1Ctrl::THRES1_CHANNEL_OFFSET
    }

    /// Set **THRES1_LOW** `[30:18]`
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub const fn with_thres1_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Thres1Ctrl::THRES1_LOW_MASK)
                | ((value << Thres1Ctrl::THRES1_LOW_OFFSET) & Thres1Ctrl::THRES1_LOW_MASK),
        }
    }

    /// Set **THRES1_HIGH** `[17:5]`
    ///
    /// saradc1's thres0 monitor thres
    #[inline(always)]
    pub const fn with_thres1_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Thres1Ctrl::THRES1_HIGH_MASK)
                | ((value << Thres1Ctrl::THRES1_HIGH_OFFSET) & Thres1Ctrl::THRES1_HIGH_MASK),
        }
    }

    /// Set **THRES1_CHANNEL** `[4:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres1_channel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Thres1Ctrl::THRES1_CHANNEL_MASK)
                | ((value << Thres1Ctrl::THRES1_CHANNEL_OFFSET) & Thres1Ctrl::THRES1_CHANNEL_MASK),
        }
    }
}

impl Default for Thres1CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Thres1Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Thres1CtrlVal {
        Thres1CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Thres1CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Thres1CtrlVal) -> Thres1CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// THRES_CTRL Register
///
/// Register
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **THRES0_EN** `[31]` (RW)
///
///   need_des
///
/// - **THRES1_EN** `[30]` (RW)
///
///   need_des
///
/// - **THRES2_EN** `[29]` (RW)
///
///   need_des
///
/// - **THRES3_EN** `[28]` (RW)
///
///   need_des
///
/// - **THRES_ALL_EN** `[27]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct ThresCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ThresCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ThresCtrl {
    // ======== Field Constants ========

    /// **THRES0_EN**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const THRES0_EN_OFFSET: u32 = 31;
    pub const THRES0_EN_MASK: u32 = 0x80000000;
    pub const THRES0_EN: u32 = 1 << 31;

    /// **THRES1_EN**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const THRES1_EN_OFFSET: u32 = 30;
    pub const THRES1_EN_MASK: u32 = 0x40000000;
    pub const THRES1_EN: u32 = 1 << 30;

    /// **THRES2_EN**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const THRES2_EN_OFFSET: u32 = 29;
    pub const THRES2_EN_MASK: u32 = 0x20000000;
    pub const THRES2_EN: u32 = 1 << 29;

    /// **THRES3_EN**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const THRES3_EN_OFFSET: u32 = 28;
    pub const THRES3_EN_MASK: u32 = 0x10000000;
    pub const THRES3_EN: u32 = 1 << 28;

    /// **THRES_ALL_EN**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const THRES_ALL_EN_OFFSET: u32 = 27;
    pub const THRES_ALL_EN_MASK: u32 = 0x08000000;
    pub const THRES_ALL_EN: u32 = 1 << 27;

    // ======== Field Accessors ========

    /// **THRES0_EN** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres0_en(&self) -> bool {
        (self.value.read_bits() & Self::THRES0_EN_MASK) != 0
    }

    /// Set THRES0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thres0_en(&self) {
        self.value.modify_bits(|r| r | Self::THRES0_EN_MASK);
    }

    /// Clear THRES0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thres0_en(&self) {
        self.value.modify_bits(|r| r & !Self::THRES0_EN_MASK);
    }

    /// **THRES1_EN** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres1_en(&self) -> bool {
        (self.value.read_bits() & Self::THRES1_EN_MASK) != 0
    }

    /// Set THRES1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thres1_en(&self) {
        self.value.modify_bits(|r| r | Self::THRES1_EN_MASK);
    }

    /// Clear THRES1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thres1_en(&self) {
        self.value.modify_bits(|r| r & !Self::THRES1_EN_MASK);
    }

    /// **THRES2_EN** `[29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres2_en(&self) -> bool {
        (self.value.read_bits() & Self::THRES2_EN_MASK) != 0
    }

    /// Set THRES2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thres2_en(&self) {
        self.value.modify_bits(|r| r | Self::THRES2_EN_MASK);
    }

    /// Clear THRES2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thres2_en(&self) {
        self.value.modify_bits(|r| r & !Self::THRES2_EN_MASK);
    }

    /// **THRES3_EN** `[28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres3_en(&self) -> bool {
        (self.value.read_bits() & Self::THRES3_EN_MASK) != 0
    }

    /// Set THRES3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thres3_en(&self) {
        self.value.modify_bits(|r| r | Self::THRES3_EN_MASK);
    }

    /// Clear THRES3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thres3_en(&self) {
        self.value.modify_bits(|r| r & !Self::THRES3_EN_MASK);
    }

    /// **THRES_ALL_EN** `[27]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres_all_en(&self) -> bool {
        (self.value.read_bits() & Self::THRES_ALL_EN_MASK) != 0
    }

    /// Set THRES_ALL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thres_all_en(&self) {
        self.value.modify_bits(|r| r | Self::THRES_ALL_EN_MASK);
    }

    /// Clear THRES_ALL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thres_all_en(&self) {
        self.value.modify_bits(|r| r & !Self::THRES_ALL_EN_MASK);
    }
}

/// THRES_CTRL value type for batch read/modify/write operations
///
/// Use `thresctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `thresctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ThresCtrlVal {
    bits: u32,
}

impl ThresCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **THRES0_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres0_en(&self) -> bool {
        (self.bits & ThresCtrl::THRES0_EN_MASK) != 0
    }

    /// Read **THRES1_EN** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres1_en(&self) -> bool {
        (self.bits & ThresCtrl::THRES1_EN_MASK) != 0
    }

    /// Read **THRES2_EN** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres2_en(&self) -> bool {
        (self.bits & ThresCtrl::THRES2_EN_MASK) != 0
    }

    /// Read **THRES3_EN** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres3_en(&self) -> bool {
        (self.bits & ThresCtrl::THRES3_EN_MASK) != 0
    }

    /// Read **THRES_ALL_EN** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres_all_en(&self) -> bool {
        (self.bits & ThresCtrl::THRES_ALL_EN_MASK) != 0
    }

    /// Set **THRES0_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ThresCtrl::THRES0_EN_MASK
            } else {
                self.bits & !ThresCtrl::THRES0_EN_MASK
            },
        }
    }

    /// Set **THRES1_EN** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres1_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ThresCtrl::THRES1_EN_MASK
            } else {
                self.bits & !ThresCtrl::THRES1_EN_MASK
            },
        }
    }

    /// Set **THRES2_EN** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres2_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ThresCtrl::THRES2_EN_MASK
            } else {
                self.bits & !ThresCtrl::THRES2_EN_MASK
            },
        }
    }

    /// Set **THRES3_EN** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres3_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ThresCtrl::THRES3_EN_MASK
            } else {
                self.bits & !ThresCtrl::THRES3_EN_MASK
            },
        }
    }

    /// Set **THRES_ALL_EN** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres_all_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ThresCtrl::THRES_ALL_EN_MASK
            } else {
                self.bits & !ThresCtrl::THRES_ALL_EN_MASK
            },
        }
    }
}

impl Default for ThresCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ThresCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ThresCtrlVal {
        ThresCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ThresCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ThresCtrlVal) -> ThresCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ENA Register
///
/// Register
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **SAR1_DONE_INT_ENA** `[31]` (RW)
///
///   need_des
///
/// - **SAR2_DONE_INT_ENA** `[30]` (RW)
///
///   need_des
///
/// - **THRES0_HIGH_INT_ENA** `[29]` (RW)
///
///   need_des
///
/// - **THRES1_HIGH_INT_ENA** `[28]` (RW)
///
///   need_des
///
/// - **THRES0_LOW_INT_ENA** `[27]` (RW)
///
///   need_des
///
/// - **THRES1_LOW_INT_ENA** `[26]` (RW)
///
///   need_des
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

    /// **SAR1_DONE_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SAR1_DONE_INT_ENA_OFFSET: u32 = 31;
    pub const SAR1_DONE_INT_ENA_MASK: u32 = 0x80000000;
    pub const SAR1_DONE_INT_ENA: u32 = 1 << 31;

    /// **SAR2_DONE_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SAR2_DONE_INT_ENA_OFFSET: u32 = 30;
    pub const SAR2_DONE_INT_ENA_MASK: u32 = 0x40000000;
    pub const SAR2_DONE_INT_ENA: u32 = 1 << 30;

    /// **THRES0_HIGH_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const THRES0_HIGH_INT_ENA_OFFSET: u32 = 29;
    pub const THRES0_HIGH_INT_ENA_MASK: u32 = 0x20000000;
    pub const THRES0_HIGH_INT_ENA: u32 = 1 << 29;

    /// **THRES1_HIGH_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const THRES1_HIGH_INT_ENA_OFFSET: u32 = 28;
    pub const THRES1_HIGH_INT_ENA_MASK: u32 = 0x10000000;
    pub const THRES1_HIGH_INT_ENA: u32 = 1 << 28;

    /// **THRES0_LOW_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const THRES0_LOW_INT_ENA_OFFSET: u32 = 27;
    pub const THRES0_LOW_INT_ENA_MASK: u32 = 0x08000000;
    pub const THRES0_LOW_INT_ENA: u32 = 1 << 27;

    /// **THRES1_LOW_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const THRES1_LOW_INT_ENA_OFFSET: u32 = 26;
    pub const THRES1_LOW_INT_ENA_MASK: u32 = 0x04000000;
    pub const THRES1_LOW_INT_ENA: u32 = 1 << 26;

    // ======== Field Accessors ========

    /// **SAR1_DONE_INT_ENA** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn sar1_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SAR1_DONE_INT_ENA_MASK) != 0
    }

    /// Set SAR1_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar1_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::SAR1_DONE_INT_ENA_MASK);
    }

    /// Clear SAR1_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar1_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SAR1_DONE_INT_ENA_MASK);
    }

    /// **SAR2_DONE_INT_ENA** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn sar2_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_DONE_INT_ENA_MASK) != 0
    }

    /// Set SAR2_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_DONE_INT_ENA_MASK);
    }

    /// Clear SAR2_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SAR2_DONE_INT_ENA_MASK);
    }

    /// **THRES0_HIGH_INT_ENA** `[29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres0_high_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::THRES0_HIGH_INT_ENA_MASK) != 0
    }

    /// Set THRES0_HIGH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thres0_high_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::THRES0_HIGH_INT_ENA_MASK);
    }

    /// Clear THRES0_HIGH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thres0_high_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::THRES0_HIGH_INT_ENA_MASK);
    }

    /// **THRES1_HIGH_INT_ENA** `[28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres1_high_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::THRES1_HIGH_INT_ENA_MASK) != 0
    }

    /// Set THRES1_HIGH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thres1_high_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::THRES1_HIGH_INT_ENA_MASK);
    }

    /// Clear THRES1_HIGH_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thres1_high_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::THRES1_HIGH_INT_ENA_MASK);
    }

    /// **THRES0_LOW_INT_ENA** `[27]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres0_low_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::THRES0_LOW_INT_ENA_MASK) != 0
    }

    /// Set THRES0_LOW_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thres0_low_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::THRES0_LOW_INT_ENA_MASK);
    }

    /// Clear THRES0_LOW_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thres0_low_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::THRES0_LOW_INT_ENA_MASK);
    }

    /// **THRES1_LOW_INT_ENA** `[26]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres1_low_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::THRES1_LOW_INT_ENA_MASK) != 0
    }

    /// Set THRES1_LOW_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thres1_low_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::THRES1_LOW_INT_ENA_MASK);
    }

    /// Clear THRES1_LOW_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thres1_low_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::THRES1_LOW_INT_ENA_MASK);
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

    /// Read **SAR1_DONE_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn sar1_done_int_ena(&self) -> bool {
        (self.bits & IntEna::SAR1_DONE_INT_ENA_MASK) != 0
    }

    /// Read **SAR2_DONE_INT_ENA** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn sar2_done_int_ena(&self) -> bool {
        (self.bits & IntEna::SAR2_DONE_INT_ENA_MASK) != 0
    }

    /// Read **THRES0_HIGH_INT_ENA** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres0_high_int_ena(&self) -> bool {
        (self.bits & IntEna::THRES0_HIGH_INT_ENA_MASK) != 0
    }

    /// Read **THRES1_HIGH_INT_ENA** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres1_high_int_ena(&self) -> bool {
        (self.bits & IntEna::THRES1_HIGH_INT_ENA_MASK) != 0
    }

    /// Read **THRES0_LOW_INT_ENA** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres0_low_int_ena(&self) -> bool {
        (self.bits & IntEna::THRES0_LOW_INT_ENA_MASK) != 0
    }

    /// Read **THRES1_LOW_INT_ENA** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres1_low_int_ena(&self) -> bool {
        (self.bits & IntEna::THRES1_LOW_INT_ENA_MASK) != 0
    }

    /// Set **SAR1_DONE_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_sar1_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SAR1_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SAR1_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **SAR2_DONE_INT_ENA** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_sar2_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SAR2_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SAR2_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **THRES0_HIGH_INT_ENA** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres0_high_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::THRES0_HIGH_INT_ENA_MASK
            } else {
                self.bits & !IntEna::THRES0_HIGH_INT_ENA_MASK
            },
        }
    }

    /// Set **THRES1_HIGH_INT_ENA** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres1_high_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::THRES1_HIGH_INT_ENA_MASK
            } else {
                self.bits & !IntEna::THRES1_HIGH_INT_ENA_MASK
            },
        }
    }

    /// Set **THRES0_LOW_INT_ENA** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres0_low_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::THRES0_LOW_INT_ENA_MASK
            } else {
                self.bits & !IntEna::THRES0_LOW_INT_ENA_MASK
            },
        }
    }

    /// Set **THRES1_LOW_INT_ENA** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres1_low_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::THRES1_LOW_INT_ENA_MASK
            } else {
                self.bits & !IntEna::THRES1_LOW_INT_ENA_MASK
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
/// Register
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **SAR1_DONE_INT_RAW** `[31]` (RW)
///
///   need_des
///
/// - **SAR2_DONE_INT_RAW** `[30]` (RW)
///
///   need_des
///
/// - **THRES0_HIGH_INT_RAW** `[29]` (RW)
///
///   need_des
///
/// - **THRES1_HIGH_INT_RAW** `[28]` (RW)
///
///   need_des
///
/// - **THRES0_LOW_INT_RAW** `[27]` (RW)
///
///   need_des
///
/// - **THRES1_LOW_INT_RAW** `[26]` (RW)
///
///   need_des
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

    /// **SAR1_DONE_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SAR1_DONE_INT_RAW_OFFSET: u32 = 31;
    pub const SAR1_DONE_INT_RAW_MASK: u32 = 0x80000000;
    pub const SAR1_DONE_INT_RAW: u32 = 1 << 31;

    /// **SAR2_DONE_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SAR2_DONE_INT_RAW_OFFSET: u32 = 30;
    pub const SAR2_DONE_INT_RAW_MASK: u32 = 0x40000000;
    pub const SAR2_DONE_INT_RAW: u32 = 1 << 30;

    /// **THRES0_HIGH_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const THRES0_HIGH_INT_RAW_OFFSET: u32 = 29;
    pub const THRES0_HIGH_INT_RAW_MASK: u32 = 0x20000000;
    pub const THRES0_HIGH_INT_RAW: u32 = 1 << 29;

    /// **THRES1_HIGH_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const THRES1_HIGH_INT_RAW_OFFSET: u32 = 28;
    pub const THRES1_HIGH_INT_RAW_MASK: u32 = 0x10000000;
    pub const THRES1_HIGH_INT_RAW: u32 = 1 << 28;

    /// **THRES0_LOW_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const THRES0_LOW_INT_RAW_OFFSET: u32 = 27;
    pub const THRES0_LOW_INT_RAW_MASK: u32 = 0x08000000;
    pub const THRES0_LOW_INT_RAW: u32 = 1 << 27;

    /// **THRES1_LOW_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const THRES1_LOW_INT_RAW_OFFSET: u32 = 26;
    pub const THRES1_LOW_INT_RAW_MASK: u32 = 0x04000000;
    pub const THRES1_LOW_INT_RAW: u32 = 1 << 26;

    // ======== Field Accessors ========

    /// **SAR1_DONE_INT_RAW** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn sar1_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SAR1_DONE_INT_RAW_MASK) != 0
    }

    /// Set SAR1_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar1_done_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::SAR1_DONE_INT_RAW_MASK);
    }

    /// Clear SAR1_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar1_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SAR1_DONE_INT_RAW_MASK);
    }

    /// **SAR2_DONE_INT_RAW** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn sar2_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_DONE_INT_RAW_MASK) != 0
    }

    /// Set SAR2_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_done_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_DONE_INT_RAW_MASK);
    }

    /// Clear SAR2_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SAR2_DONE_INT_RAW_MASK);
    }

    /// **THRES0_HIGH_INT_RAW** `[29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres0_high_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::THRES0_HIGH_INT_RAW_MASK) != 0
    }

    /// Set THRES0_HIGH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thres0_high_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::THRES0_HIGH_INT_RAW_MASK);
    }

    /// Clear THRES0_HIGH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thres0_high_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::THRES0_HIGH_INT_RAW_MASK);
    }

    /// **THRES1_HIGH_INT_RAW** `[28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres1_high_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::THRES1_HIGH_INT_RAW_MASK) != 0
    }

    /// Set THRES1_HIGH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thres1_high_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::THRES1_HIGH_INT_RAW_MASK);
    }

    /// Clear THRES1_HIGH_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thres1_high_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::THRES1_HIGH_INT_RAW_MASK);
    }

    /// **THRES0_LOW_INT_RAW** `[27]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres0_low_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::THRES0_LOW_INT_RAW_MASK) != 0
    }

    /// Set THRES0_LOW_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thres0_low_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::THRES0_LOW_INT_RAW_MASK);
    }

    /// Clear THRES0_LOW_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thres0_low_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::THRES0_LOW_INT_RAW_MASK);
    }

    /// **THRES1_LOW_INT_RAW** `[26]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres1_low_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::THRES1_LOW_INT_RAW_MASK) != 0
    }

    /// Set THRES1_LOW_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_thres1_low_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::THRES1_LOW_INT_RAW_MASK);
    }

    /// Clear THRES1_LOW_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_thres1_low_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::THRES1_LOW_INT_RAW_MASK);
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

    /// Read **SAR1_DONE_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn sar1_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::SAR1_DONE_INT_RAW_MASK) != 0
    }

    /// Read **SAR2_DONE_INT_RAW** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn sar2_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::SAR2_DONE_INT_RAW_MASK) != 0
    }

    /// Read **THRES0_HIGH_INT_RAW** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres0_high_int_raw(&self) -> bool {
        (self.bits & IntRaw::THRES0_HIGH_INT_RAW_MASK) != 0
    }

    /// Read **THRES1_HIGH_INT_RAW** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres1_high_int_raw(&self) -> bool {
        (self.bits & IntRaw::THRES1_HIGH_INT_RAW_MASK) != 0
    }

    /// Read **THRES0_LOW_INT_RAW** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres0_low_int_raw(&self) -> bool {
        (self.bits & IntRaw::THRES0_LOW_INT_RAW_MASK) != 0
    }

    /// Read **THRES1_LOW_INT_RAW** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres1_low_int_raw(&self) -> bool {
        (self.bits & IntRaw::THRES1_LOW_INT_RAW_MASK) != 0
    }

    /// Set **SAR1_DONE_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_sar1_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SAR1_DONE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SAR1_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **SAR2_DONE_INT_RAW** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_sar2_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SAR2_DONE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SAR2_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **THRES0_HIGH_INT_RAW** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres0_high_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::THRES0_HIGH_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::THRES0_HIGH_INT_RAW_MASK
            },
        }
    }

    /// Set **THRES1_HIGH_INT_RAW** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres1_high_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::THRES1_HIGH_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::THRES1_HIGH_INT_RAW_MASK
            },
        }
    }

    /// Set **THRES0_LOW_INT_RAW** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres0_low_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::THRES0_LOW_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::THRES0_LOW_INT_RAW_MASK
            },
        }
    }

    /// Set **THRES1_LOW_INT_RAW** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres1_low_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::THRES1_LOW_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::THRES1_LOW_INT_RAW_MASK
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
/// Register
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **APB_SARADC1_DONE_INT_ST** `[31]` (RO)
///
///   need_des
///
/// - **APB_SARADC2_DONE_INT_ST** `[30]` (RO)
///
///   need_des
///
/// - **THRES0_HIGH_INT_ST** `[29]` (RO)
///
///   need_des
///
/// - **THRES1_HIGH_INT_ST** `[28]` (RO)
///
///   need_des
///
/// - **THRES0_LOW_INT_ST** `[27]` (RO)
///
///   need_des
///
/// - **THRES1_LOW_INT_ST** `[26]` (RO)
///
///   need_des
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

    /// **APB_SARADC1_DONE_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const APB_SARADC1_DONE_INT_ST_OFFSET: u32 = 31;
    pub const APB_SARADC1_DONE_INT_ST_MASK: u32 = 0x80000000;
    pub const APB_SARADC1_DONE_INT_ST: u32 = 1 << 31;

    /// **APB_SARADC2_DONE_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const APB_SARADC2_DONE_INT_ST_OFFSET: u32 = 30;
    pub const APB_SARADC2_DONE_INT_ST_MASK: u32 = 0x40000000;
    pub const APB_SARADC2_DONE_INT_ST: u32 = 1 << 30;

    /// **THRES0_HIGH_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const THRES0_HIGH_INT_ST_OFFSET: u32 = 29;
    pub const THRES0_HIGH_INT_ST_MASK: u32 = 0x20000000;
    pub const THRES0_HIGH_INT_ST: u32 = 1 << 29;

    /// **THRES1_HIGH_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const THRES1_HIGH_INT_ST_OFFSET: u32 = 28;
    pub const THRES1_HIGH_INT_ST_MASK: u32 = 0x10000000;
    pub const THRES1_HIGH_INT_ST: u32 = 1 << 28;

    /// **THRES0_LOW_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const THRES0_LOW_INT_ST_OFFSET: u32 = 27;
    pub const THRES0_LOW_INT_ST_MASK: u32 = 0x08000000;
    pub const THRES0_LOW_INT_ST: u32 = 1 << 27;

    /// **THRES1_LOW_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const THRES1_LOW_INT_ST_OFFSET: u32 = 26;
    pub const THRES1_LOW_INT_ST_MASK: u32 = 0x04000000;
    pub const THRES1_LOW_INT_ST: u32 = 1 << 26;

    // ======== Field Accessors ========

    /// **APB_SARADC1_DONE_INT_ST** `[31]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn apb_saradc1_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::APB_SARADC1_DONE_INT_ST_MASK) != 0
    }

    /// **APB_SARADC2_DONE_INT_ST** `[30]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn apb_saradc2_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::APB_SARADC2_DONE_INT_ST_MASK) != 0
    }

    /// **THRES0_HIGH_INT_ST** `[29]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres0_high_int_st(&self) -> bool {
        (self.value.read_bits() & Self::THRES0_HIGH_INT_ST_MASK) != 0
    }

    /// **THRES1_HIGH_INT_ST** `[28]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres1_high_int_st(&self) -> bool {
        (self.value.read_bits() & Self::THRES1_HIGH_INT_ST_MASK) != 0
    }

    /// **THRES0_LOW_INT_ST** `[27]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres0_low_int_st(&self) -> bool {
        (self.value.read_bits() & Self::THRES0_LOW_INT_ST_MASK) != 0
    }

    /// **THRES1_LOW_INT_ST** `[26]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn thres1_low_int_st(&self) -> bool {
        (self.value.read_bits() & Self::THRES1_LOW_INT_ST_MASK) != 0
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

    /// Read **APB_SARADC1_DONE_INT_ST** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn apb_saradc1_done_int_st(&self) -> bool {
        (self.bits & IntSt::APB_SARADC1_DONE_INT_ST_MASK) != 0
    }

    /// Read **APB_SARADC2_DONE_INT_ST** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn apb_saradc2_done_int_st(&self) -> bool {
        (self.bits & IntSt::APB_SARADC2_DONE_INT_ST_MASK) != 0
    }

    /// Read **THRES0_HIGH_INT_ST** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres0_high_int_st(&self) -> bool {
        (self.bits & IntSt::THRES0_HIGH_INT_ST_MASK) != 0
    }

    /// Read **THRES1_HIGH_INT_ST** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres1_high_int_st(&self) -> bool {
        (self.bits & IntSt::THRES1_HIGH_INT_ST_MASK) != 0
    }

    /// Read **THRES0_LOW_INT_ST** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres0_low_int_st(&self) -> bool {
        (self.bits & IntSt::THRES0_LOW_INT_ST_MASK) != 0
    }

    /// Read **THRES1_LOW_INT_ST** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn thres1_low_int_st(&self) -> bool {
        (self.bits & IntSt::THRES1_LOW_INT_ST_MASK) != 0
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
/// Register
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **APB_SARADC1_DONE_INT_CLR** `[31]` (WO)
///
///   need_des
///
/// - **APB_SARADC2_DONE_INT_CLR** `[30]` (WO)
///
///   need_des
///
/// - **THRES0_HIGH_INT_CLR** `[29]` (WO)
///
///   need_des
///
/// - **THRES1_HIGH_INT_CLR** `[28]` (WO)
///
///   need_des
///
/// - **THRES0_LOW_INT_CLR** `[27]` (WO)
///
///   need_des
///
/// - **THRES1_LOW_INT_CLR** `[26]` (WO)
///
///   need_des
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

    /// **APB_SARADC1_DONE_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const APB_SARADC1_DONE_INT_CLR_OFFSET: u32 = 31;
    pub const APB_SARADC1_DONE_INT_CLR_MASK: u32 = 0x80000000;
    pub const APB_SARADC1_DONE_INT_CLR: u32 = 1 << 31;

    /// **APB_SARADC2_DONE_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const APB_SARADC2_DONE_INT_CLR_OFFSET: u32 = 30;
    pub const APB_SARADC2_DONE_INT_CLR_MASK: u32 = 0x40000000;
    pub const APB_SARADC2_DONE_INT_CLR: u32 = 1 << 30;

    /// **THRES0_HIGH_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const THRES0_HIGH_INT_CLR_OFFSET: u32 = 29;
    pub const THRES0_HIGH_INT_CLR_MASK: u32 = 0x20000000;
    pub const THRES0_HIGH_INT_CLR: u32 = 1 << 29;

    /// **THRES1_HIGH_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const THRES1_HIGH_INT_CLR_OFFSET: u32 = 28;
    pub const THRES1_HIGH_INT_CLR_MASK: u32 = 0x10000000;
    pub const THRES1_HIGH_INT_CLR: u32 = 1 << 28;

    /// **THRES0_LOW_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const THRES0_LOW_INT_CLR_OFFSET: u32 = 27;
    pub const THRES0_LOW_INT_CLR_MASK: u32 = 0x08000000;
    pub const THRES0_LOW_INT_CLR: u32 = 1 << 27;

    /// **THRES1_LOW_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const THRES1_LOW_INT_CLR_OFFSET: u32 = 26;
    pub const THRES1_LOW_INT_CLR_MASK: u32 = 0x04000000;
    pub const THRES1_LOW_INT_CLR: u32 = 1 << 26;

    // ======== Field Accessors ========

    /// Trigger **APB_SARADC1_DONE_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_apb_saradc1_done_int_clr(&self) {
        self.value.write_bits(Self::APB_SARADC1_DONE_INT_CLR_MASK);
    }

    /// Trigger **APB_SARADC2_DONE_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_apb_saradc2_done_int_clr(&self) {
        self.value.write_bits(Self::APB_SARADC2_DONE_INT_CLR_MASK);
    }

    /// Trigger **THRES0_HIGH_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_thres0_high_int_clr(&self) {
        self.value.write_bits(Self::THRES0_HIGH_INT_CLR_MASK);
    }

    /// Trigger **THRES1_HIGH_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_thres1_high_int_clr(&self) {
        self.value.write_bits(Self::THRES1_HIGH_INT_CLR_MASK);
    }

    /// Trigger **THRES0_LOW_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_thres0_low_int_clr(&self) {
        self.value.write_bits(Self::THRES0_LOW_INT_CLR_MASK);
    }

    /// Trigger **THRES1_LOW_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_thres1_low_int_clr(&self) {
        self.value.write_bits(Self::THRES1_LOW_INT_CLR_MASK);
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

    /// Set **APB_SARADC1_DONE_INT_CLR** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_apb_saradc1_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::APB_SARADC1_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::APB_SARADC1_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **APB_SARADC2_DONE_INT_CLR** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_apb_saradc2_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::APB_SARADC2_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::APB_SARADC2_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **THRES0_HIGH_INT_CLR** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres0_high_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::THRES0_HIGH_INT_CLR_MASK
            } else {
                self.bits & !IntClr::THRES0_HIGH_INT_CLR_MASK
            },
        }
    }

    /// Set **THRES1_HIGH_INT_CLR** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres1_high_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::THRES1_HIGH_INT_CLR_MASK
            } else {
                self.bits & !IntClr::THRES1_HIGH_INT_CLR_MASK
            },
        }
    }

    /// Set **THRES0_LOW_INT_CLR** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres0_low_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::THRES0_LOW_INT_CLR_MASK
            } else {
                self.bits & !IntClr::THRES0_LOW_INT_CLR_MASK
            },
        }
    }

    /// Set **THRES1_LOW_INT_CLR** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_thres1_low_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::THRES1_LOW_INT_CLR_MASK
            } else {
                self.bits & !IntClr::THRES1_LOW_INT_CLR_MASK
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

/// DMA_CONF Register
///
/// Register
///
/// Address offset: 0x0060
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **APB_ADC_TRANS** `[31]` (RW)
///
///   enable apb_adc use spi_dma
///
/// - **APB_ADC_RESET_FSM** `[30]` (RW)
///
///   reset_apb_adc_state
///
/// - **APB_ADC_EOF_NUM** `[15:0]` (RW)
///
///   the dma_in_suc_eof gen when sample cnt = spi_eof_num
#[repr(transparent)]
pub struct DmaConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmaConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmaConf {
    // ======== Field Constants ========

    /// **APB_ADC_TRANS**
    ///
    /// enable apb_adc use spi_dma
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const APB_ADC_TRANS_OFFSET: u32 = 31;
    pub const APB_ADC_TRANS_MASK: u32 = 0x80000000;
    pub const APB_ADC_TRANS: u32 = 1 << 31;

    /// **APB_ADC_RESET_FSM**
    ///
    /// reset_apb_adc_state
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const APB_ADC_RESET_FSM_OFFSET: u32 = 30;
    pub const APB_ADC_RESET_FSM_MASK: u32 = 0x40000000;
    pub const APB_ADC_RESET_FSM: u32 = 1 << 30;

    /// **APB_ADC_EOF_NUM**
    ///
    /// the dma_in_suc_eof gen when sample cnt = spi_eof_num
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const APB_ADC_EOF_NUM_OFFSET: u32 = 0;
    pub const APB_ADC_EOF_NUM_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **APB_ADC_TRANS** `[31]` (RW)
    ///
    /// enable apb_adc use spi_dma
    #[inline(always)]
    pub fn apb_adc_trans(&self) -> bool {
        (self.value.read_bits() & Self::APB_ADC_TRANS_MASK) != 0
    }

    /// Set APB_ADC_TRANS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_apb_adc_trans(&self) {
        self.value.modify_bits(|r| r | Self::APB_ADC_TRANS_MASK);
    }

    /// Clear APB_ADC_TRANS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_apb_adc_trans(&self) {
        self.value.modify_bits(|r| r & !Self::APB_ADC_TRANS_MASK);
    }

    /// **APB_ADC_RESET_FSM** `[30]` (RW)
    ///
    /// reset_apb_adc_state
    #[inline(always)]
    pub fn apb_adc_reset_fsm(&self) -> bool {
        (self.value.read_bits() & Self::APB_ADC_RESET_FSM_MASK) != 0
    }

    /// Set APB_ADC_RESET_FSM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_apb_adc_reset_fsm(&self) {
        self.value.modify_bits(|r| r | Self::APB_ADC_RESET_FSM_MASK);
    }

    /// Clear APB_ADC_RESET_FSM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_apb_adc_reset_fsm(&self) {
        self.value
            .modify_bits(|r| r & !Self::APB_ADC_RESET_FSM_MASK);
    }

    /// **APB_ADC_EOF_NUM** `[15:0]` (RW)
    ///
    /// the dma_in_suc_eof gen when sample cnt = spi_eof_num
    #[inline(always)]
    pub fn apb_adc_eof_num(&self) -> u32 {
        (self.value.read_bits() & Self::APB_ADC_EOF_NUM_MASK) >> Self::APB_ADC_EOF_NUM_OFFSET
    }

    /// Write **APB_ADC_EOF_NUM** `[15:0]` (RW)
    ///
    /// the dma_in_suc_eof gen when sample cnt = spi_eof_num
    #[inline(always)]
    pub fn write_apb_adc_eof_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::APB_ADC_EOF_NUM_MASK)
                | ((value << Self::APB_ADC_EOF_NUM_OFFSET) & Self::APB_ADC_EOF_NUM_MASK)
        });
    }
}

/// DMA_CONF value type for batch read/modify/write operations
///
/// Use `dmaconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmaconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaConfVal {
    bits: u32,
}

impl DmaConfVal {
    /// Create from raw bits
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
        Self { bits: 0x000000ff }
    }

    /// Read **APB_ADC_TRANS** `[31:31]`
    ///
    /// enable apb_adc use spi_dma
    #[inline(always)]
    pub const fn apb_adc_trans(&self) -> bool {
        (self.bits & DmaConf::APB_ADC_TRANS_MASK) != 0
    }

    /// Read **APB_ADC_RESET_FSM** `[30:30]`
    ///
    /// reset_apb_adc_state
    #[inline(always)]
    pub const fn apb_adc_reset_fsm(&self) -> bool {
        (self.bits & DmaConf::APB_ADC_RESET_FSM_MASK) != 0
    }

    /// Read **APB_ADC_EOF_NUM** `[15:0]`
    ///
    /// the dma_in_suc_eof gen when sample cnt = spi_eof_num
    #[inline(always)]
    pub const fn apb_adc_eof_num(&self) -> u32 {
        (self.bits & DmaConf::APB_ADC_EOF_NUM_MASK) >> DmaConf::APB_ADC_EOF_NUM_OFFSET
    }

    /// Set **APB_ADC_TRANS** `[31:31]`
    ///
    /// enable apb_adc use spi_dma
    #[inline(always)]
    pub const fn with_apb_adc_trans(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaConf::APB_ADC_TRANS_MASK
            } else {
                self.bits & !DmaConf::APB_ADC_TRANS_MASK
            },
        }
    }

    /// Set **APB_ADC_RESET_FSM** `[30:30]`
    ///
    /// reset_apb_adc_state
    #[inline(always)]
    pub const fn with_apb_adc_reset_fsm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaConf::APB_ADC_RESET_FSM_MASK
            } else {
                self.bits & !DmaConf::APB_ADC_RESET_FSM_MASK
            },
        }
    }

    /// Set **APB_ADC_EOF_NUM** `[15:0]`
    ///
    /// the dma_in_suc_eof gen when sample cnt = spi_eof_num
    #[inline(always)]
    pub const fn with_apb_adc_eof_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DmaConf::APB_ADC_EOF_NUM_MASK)
                | ((value << DmaConf::APB_ADC_EOF_NUM_OFFSET) & DmaConf::APB_ADC_EOF_NUM_MASK),
        }
    }
}

impl Default for DmaConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DmaConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaConfVal {
        DmaConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaConfVal) -> DmaConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SAR2_DATA_STATUS Register
///
/// Register
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **APB_SARADC2_DATA** `[16:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Sar2DataStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sar2DataStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sar2DataStatus {
    // ======== Field Constants ========

    /// **APB_SARADC2_DATA**
    ///
    /// need_des
    ///
    /// Bits `[16:0]`, mask `0x0001ffff`
    pub const APB_SARADC2_DATA_OFFSET: u32 = 0;
    pub const APB_SARADC2_DATA_MASK: u32 = 0x0001ffff;

    // ======== Field Accessors ========

    /// **APB_SARADC2_DATA** `[16:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn apb_saradc2_data(&self) -> u32 {
        (self.value.read_bits() & Self::APB_SARADC2_DATA_MASK) >> Self::APB_SARADC2_DATA_OFFSET
    }
}

/// SAR2_DATA_STATUS value type for batch read/modify/write operations
///
/// Use `sar2datastatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sar2datastatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sar2DataStatusVal {
    bits: u32,
}

impl Sar2DataStatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB_SARADC2_DATA** `[16:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn apb_saradc2_data(&self) -> u32 {
        (self.bits & Sar2DataStatus::APB_SARADC2_DATA_MASK)
            >> Sar2DataStatus::APB_SARADC2_DATA_OFFSET
    }
}

impl Default for Sar2DataStatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Sar2DataStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sar2DataStatusVal {
        Sar2DataStatusVal::from_bits(self.value.read_bits())
    }
}

/// CALI Register
///
/// Register
///
/// Address offset: 0x0068
/// Reset value: 0x00008000
///
/// ## Fields
///
/// - **CFG** `[16:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Cali {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cali {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cali {
    // ======== Field Constants ========

    /// **CFG**
    ///
    /// need_des
    ///
    /// Bits `[16:0]`, mask `0x0001ffff`
    pub const CFG_OFFSET: u32 = 0;
    pub const CFG_MASK: u32 = 0x0001ffff;

    // ======== Field Accessors ========

    /// **CFG** `[16:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn cfg(&self) -> u32 {
        (self.value.read_bits() & Self::CFG_MASK) >> Self::CFG_OFFSET
    }

    /// Write **CFG** `[16:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_cfg(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CFG_MASK) | ((value << Self::CFG_OFFSET) & Self::CFG_MASK)
        });
    }
}

/// CALI value type for batch read/modify/write operations
///
/// Use `cali.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cali.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CaliVal {
    bits: u32,
}

impl CaliVal {
    /// Create from raw bits
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
        Self { bits: 0x00008000 }
    }

    /// Read **CFG** `[16:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn cfg(&self) -> u32 {
        (self.bits & Cali::CFG_MASK) >> Cali::CFG_OFFSET
    }

    /// Set **CFG** `[16:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_cfg(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cali::CFG_MASK) | ((value << Cali::CFG_OFFSET) & Cali::CFG_MASK),
        }
    }
}

impl Default for CaliVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cali {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CaliVal {
        CaliVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CaliVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CaliVal) -> CaliVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RND_ECO_LOW Register
///
/// Register
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **RND_ECO_LOW** `[31:0]` (RW)
///
///   rnd eco low
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

    /// **RND_ECO_LOW**
    ///
    /// rnd eco low
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RND_ECO_LOW_OFFSET: u32 = 0;
    pub const RND_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RND_ECO_LOW** `[31:0]` (RW)
    ///
    /// rnd eco low
    #[inline(always)]
    pub fn rnd_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::RND_ECO_LOW_MASK) >> Self::RND_ECO_LOW_OFFSET
    }

    /// Write **RND_ECO_LOW** `[31:0]` (RW)
    ///
    /// rnd eco low
    #[inline(always)]
    pub fn write_rnd_eco_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RND_ECO_LOW_MASK)
                | ((value << Self::RND_ECO_LOW_OFFSET) & Self::RND_ECO_LOW_MASK)
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

    /// Read **RND_ECO_LOW** `[31:0]`
    ///
    /// rnd eco low
    #[inline(always)]
    pub const fn rnd_eco_low(&self) -> u32 {
        (self.bits & RndEcoLow::RND_ECO_LOW_MASK) >> RndEcoLow::RND_ECO_LOW_OFFSET
    }

    /// Set **RND_ECO_LOW** `[31:0]`
    ///
    /// rnd eco low
    #[inline(always)]
    pub const fn with_rnd_eco_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RndEcoLow::RND_ECO_LOW_MASK)
                | ((value << RndEcoLow::RND_ECO_LOW_OFFSET) & RndEcoLow::RND_ECO_LOW_MASK),
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
/// Register
///
/// Address offset: 0x0070
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **RND_ECO_HIGH** `[31:0]` (RW)
///
///   rnd eco high
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

    /// **RND_ECO_HIGH**
    ///
    /// rnd eco high
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RND_ECO_HIGH_OFFSET: u32 = 0;
    pub const RND_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RND_ECO_HIGH** `[31:0]` (RW)
    ///
    /// rnd eco high
    #[inline(always)]
    pub fn rnd_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::RND_ECO_HIGH_MASK) >> Self::RND_ECO_HIGH_OFFSET
    }

    /// Write **RND_ECO_HIGH** `[31:0]` (RW)
    ///
    /// rnd eco high
    #[inline(always)]
    pub fn write_rnd_eco_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RND_ECO_HIGH_MASK)
                | ((value << Self::RND_ECO_HIGH_OFFSET) & Self::RND_ECO_HIGH_MASK)
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
        Self { bits: 0xffffffff }
    }

    /// Read **RND_ECO_HIGH** `[31:0]`
    ///
    /// rnd eco high
    #[inline(always)]
    pub const fn rnd_eco_high(&self) -> u32 {
        (self.bits & RndEcoHigh::RND_ECO_HIGH_MASK) >> RndEcoHigh::RND_ECO_HIGH_OFFSET
    }

    /// Set **RND_ECO_HIGH** `[31:0]`
    ///
    /// rnd eco high
    #[inline(always)]
    pub const fn with_rnd_eco_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RndEcoHigh::RND_ECO_HIGH_MASK)
                | ((value << RndEcoHigh::RND_ECO_HIGH_OFFSET) & RndEcoHigh::RND_ECO_HIGH_MASK),
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

/// RND_ECO_CS Register
///
/// Register
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **RND_ECO_RESULT** `[1]` (RO)
///
///   need_des
///
/// - **RND_ECO_EN** `[0]` (RW)
///
///   need_des
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
    /// need_des
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RND_ECO_RESULT_OFFSET: u32 = 1;
    pub const RND_ECO_RESULT_MASK: u32 = 0x00000002;
    pub const RND_ECO_RESULT: u32 = 1 << 1;

    /// **RND_ECO_EN**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RND_ECO_EN_OFFSET: u32 = 0;
    pub const RND_ECO_EN_MASK: u32 = 0x00000001;
    pub const RND_ECO_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RND_ECO_RESULT** `[1]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn rnd_eco_result(&self) -> bool {
        (self.value.read_bits() & Self::RND_ECO_RESULT_MASK) != 0
    }

    /// **RND_ECO_EN** `[0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rnd_eco_en(&self) -> bool {
        (self.value.read_bits() & Self::RND_ECO_EN_MASK) != 0
    }

    /// Set RND_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rnd_eco_en(&self) {
        self.value.modify_bits(|r| r | Self::RND_ECO_EN_MASK);
    }

    /// Clear RND_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rnd_eco_en(&self) {
        self.value.modify_bits(|r| r & !Self::RND_ECO_EN_MASK);
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
    /// need_des
    #[inline(always)]
    pub const fn rnd_eco_result(&self) -> bool {
        (self.bits & RndEcoCs::RND_ECO_RESULT_MASK) != 0
    }

    /// Read **RND_ECO_EN** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rnd_eco_en(&self) -> bool {
        (self.bits & RndEcoCs::RND_ECO_EN_MASK) != 0
    }

    /// Set **RND_ECO_EN** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rnd_eco_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RndEcoCs::RND_ECO_EN_MASK
            } else {
                self.bits & !RndEcoCs::RND_ECO_EN_MASK
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

/// CTRL_DATE Register
///
/// Register
///
/// Address offset: 0x03fc
/// Reset value: 0x02212260
///
/// ## Fields
///
/// - **CLK_EN** `[31]` (RW)
///
///   need_des
///
/// - **CTRL_DATE** `[30:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct CtrlDate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CtrlDate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CtrlDate {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_EN_OFFSET: u32 = 31;
    pub const CLK_EN_MASK: u32 = 0x80000000;
    pub const CLK_EN: u32 = 1 << 31;

    /// **CTRL_DATE**
    ///
    /// need_des
    ///
    /// Bits `[30:0]`, mask `0x7fffffff`
    pub const CTRL_DATE_OFFSET: u32 = 0;
    pub const CTRL_DATE_MASK: u32 = 0x7fffffff;

    // ======== Field Accessors ========

    /// **CLK_EN** `[31]` (RW)
    ///
    /// need_des
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

    /// **CTRL_DATE** `[30:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ctrl_date(&self) -> u32 {
        (self.value.read_bits() & Self::CTRL_DATE_MASK) >> Self::CTRL_DATE_OFFSET
    }

    /// Write **CTRL_DATE** `[30:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_ctrl_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CTRL_DATE_MASK) | ((value << Self::CTRL_DATE_OFFSET) & Self::CTRL_DATE_MASK)
        });
    }
}

/// CTRL_DATE value type for batch read/modify/write operations
///
/// Use `ctrldate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctrldate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CtrlDateVal {
    bits: u32,
}

impl CtrlDateVal {
    /// Create from raw bits
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
        Self { bits: 0x02212260 }
    }

    /// Read **CLK_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & CtrlDate::CLK_EN_MASK) != 0
    }

    /// Read **CTRL_DATE** `[30:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ctrl_date(&self) -> u32 {
        (self.bits & CtrlDate::CTRL_DATE_MASK) >> CtrlDate::CTRL_DATE_OFFSET
    }

    /// Set **CLK_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CtrlDate::CLK_EN_MASK
            } else {
                self.bits & !CtrlDate::CLK_EN_MASK
            },
        }
    }

    /// Set **CTRL_DATE** `[30:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ctrl_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CtrlDate::CTRL_DATE_MASK)
                | ((value << CtrlDate::CTRL_DATE_OFFSET) & CtrlDate::CTRL_DATE_MASK),
        }
    }
}

impl Default for CtrlDateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CtrlDate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CtrlDateVal {
        CtrlDateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CtrlDateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CtrlDateVal) -> CtrlDateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Register
    pub ctrl: Ctrl,
    /// 0x0004
    /// Register
    pub ctrl2: Ctrl2,
    /// 0x0008
    /// Register
    pub filter_ctrl1: FilterCtrl1,
    /// 0x000c
    /// Register
    pub fsm_wait: FsmWait,
    /// 0x0010
    /// Register
    pub sar1_status: Sar1Status,
    /// 0x0014
    /// Register
    pub sar2_status: Sar2Status,
    /// 0x0018
    /// Register
    pub sar1_patt_tab1: Sar1PattTab1,
    /// 0x001c
    /// Register
    pub sar1_patt_tab2: Sar1PattTab2,
    /// 0x0020
    /// Register
    pub sar1_patt_tab3: Sar1PattTab3,
    /// 0x0024
    /// Register
    pub sar1_patt_tab4: Sar1PattTab4,
    /// 0x0028
    /// Register
    pub sar2_patt_tab1: Sar2PattTab1,
    /// 0x002c
    /// Register
    pub sar2_patt_tab2: Sar2PattTab2,
    /// 0x0030
    /// Register
    pub sar2_patt_tab3: Sar2PattTab3,
    /// 0x0034
    /// Register
    pub sar2_patt_tab4: Sar2PattTab4,
    /// 0x0038
    /// Register
    pub arb_ctrl: ArbCtrl,
    /// 0x003c
    /// Register
    pub filter_ctrl0: FilterCtrl0,
    /// 0x0040
    /// Register
    pub sar1_data_status: Sar1DataStatus,
    /// 0x0044
    /// Register
    pub thres0_ctrl: Thres0Ctrl,
    /// 0x0048
    /// Register
    pub thres1_ctrl: Thres1Ctrl,
    /// 0x004c
    /// Register
    pub thres_ctrl: ThresCtrl,
    /// 0x0050
    /// Register
    pub int_ena: IntEna,
    /// 0x0054
    /// Register
    pub int_raw: IntRaw,
    /// 0x0058
    /// Register
    pub int_st: IntSt,
    /// 0x005c
    /// Register
    pub int_clr: IntClr,
    /// 0x0060
    /// Register
    pub dma_conf: DmaConf,
    /// 0x0064
    /// Register
    pub sar2_data_status: Sar2DataStatus,
    /// 0x0068
    /// Register
    pub cali: Cali,
    /// 0x006c
    /// Register
    pub rnd_eco_low: RndEcoLow,
    /// 0x0070
    /// Register
    pub rnd_eco_high: RndEcoHigh,
    /// 0x0074
    /// Register
    pub rnd_eco_cs: RndEcoCs,
    _reserved0: [u8; 900],
    /// 0x03fc
    /// Register
    pub ctrl_date: CtrlDate,
}
