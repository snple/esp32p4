//! LP_ADC
//! Low-power Analog to Digital Converter

#![allow(dead_code)]

pub const BASE: u32 = 0x50127000;

/// READER1_CTRL Register
///
/// Control the read operation of ADC1.
///
/// Address offset: 0x0000
/// Reset value: 0x20040002
///
/// ## Fields
///
/// - **SAR1_EN_PAD_FORCE_ENABLE** `[31:30]` (RW)
///
///   Force enable adc en_pad to analog circuit 2'b11: force enable .
///
/// - **SAR1_INT_EN** `[29]` (RW)
///
///   Enable saradc1 to send out interrupt.
///
/// - **SAR1_DATA_INV** `[28]` (RW)
///
///   Invert SAR ADC1 data.
///
/// - **SAR1_SAMPLE_NUM** `[26:19]` (RW)
///
///   N/A
///
/// - **SAR1_CLK_GATED** `[18]` (RW)
///
///   N/A
///
/// - **SAR1_CLK_DIV** `[7:0]` (RW)
///
///   Clock divider.
#[repr(transparent)]
pub struct Reader1Ctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Reader1Ctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Reader1Ctrl {
    // ======== Field Constants ========

    /// **SAR1_EN_PAD_FORCE_ENABLE**
    ///
    /// Force enable adc en_pad to analog circuit 2'b11: force enable .
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const SAR1_EN_PAD_FORCE_ENABLE_OFFSET: u32 = 30;
    pub const SAR1_EN_PAD_FORCE_ENABLE_MASK: u32 = 0xc0000000;

    /// **SAR1_INT_EN**
    ///
    /// Enable saradc1 to send out interrupt.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SAR1_INT_EN_OFFSET: u32 = 29;
    pub const SAR1_INT_EN_MASK: u32 = 0x20000000;
    pub const SAR1_INT_EN: u32 = 1 << 29;

    /// **SAR1_DATA_INV**
    ///
    /// Invert SAR ADC1 data.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const SAR1_DATA_INV_OFFSET: u32 = 28;
    pub const SAR1_DATA_INV_MASK: u32 = 0x10000000;
    pub const SAR1_DATA_INV: u32 = 1 << 28;

    /// **SAR1_SAMPLE_NUM**
    ///
    /// N/A
    ///
    /// Bits `[26:19]`, mask `0x07f80000`
    pub const SAR1_SAMPLE_NUM_OFFSET: u32 = 19;
    pub const SAR1_SAMPLE_NUM_MASK: u32 = 0x07f80000;

    /// **SAR1_CLK_GATED**
    ///
    /// N/A
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SAR1_CLK_GATED_OFFSET: u32 = 18;
    pub const SAR1_CLK_GATED_MASK: u32 = 0x00040000;
    pub const SAR1_CLK_GATED: u32 = 1 << 18;

    /// **SAR1_CLK_DIV**
    ///
    /// Clock divider.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SAR1_CLK_DIV_OFFSET: u32 = 0;
    pub const SAR1_CLK_DIV_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SAR1_EN_PAD_FORCE_ENABLE** `[31:30]` (RW)
    ///
    /// Force enable adc en_pad to analog circuit 2'b11: force enable .
    #[inline(always)]
    pub fn sar1_en_pad_force_enable(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_EN_PAD_FORCE_ENABLE_MASK)
            >> Self::SAR1_EN_PAD_FORCE_ENABLE_OFFSET
    }

    /// Write **SAR1_EN_PAD_FORCE_ENABLE** `[31:30]` (RW)
    ///
    /// Force enable adc en_pad to analog circuit 2'b11: force enable .
    #[inline(always)]
    pub fn write_sar1_en_pad_force_enable(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR1_EN_PAD_FORCE_ENABLE_MASK)
                | ((value << Self::SAR1_EN_PAD_FORCE_ENABLE_OFFSET)
                    & Self::SAR1_EN_PAD_FORCE_ENABLE_MASK)
        });
    }

    /// **SAR1_INT_EN** `[29]` (RW)
    ///
    /// Enable saradc1 to send out interrupt.
    #[inline(always)]
    pub fn sar1_int_en(&self) -> bool {
        (self.value.read_bits() & Self::SAR1_INT_EN_MASK) != 0
    }

    /// Set SAR1_INT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar1_int_en(&self) {
        self.value.modify_bits(|r| r | Self::SAR1_INT_EN_MASK);
    }

    /// Clear SAR1_INT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar1_int_en(&self) {
        self.value.modify_bits(|r| r & !Self::SAR1_INT_EN_MASK);
    }

    /// **SAR1_DATA_INV** `[28]` (RW)
    ///
    /// Invert SAR ADC1 data.
    #[inline(always)]
    pub fn sar1_data_inv(&self) -> bool {
        (self.value.read_bits() & Self::SAR1_DATA_INV_MASK) != 0
    }

    /// Set SAR1_DATA_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar1_data_inv(&self) {
        self.value.modify_bits(|r| r | Self::SAR1_DATA_INV_MASK);
    }

    /// Clear SAR1_DATA_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar1_data_inv(&self) {
        self.value.modify_bits(|r| r & !Self::SAR1_DATA_INV_MASK);
    }

    /// **SAR1_SAMPLE_NUM** `[26:19]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar1_sample_num(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_SAMPLE_NUM_MASK) >> Self::SAR1_SAMPLE_NUM_OFFSET
    }

    /// Write **SAR1_SAMPLE_NUM** `[26:19]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_sar1_sample_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR1_SAMPLE_NUM_MASK)
                | ((value << Self::SAR1_SAMPLE_NUM_OFFSET) & Self::SAR1_SAMPLE_NUM_MASK)
        });
    }

    /// **SAR1_CLK_GATED** `[18]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar1_clk_gated(&self) -> bool {
        (self.value.read_bits() & Self::SAR1_CLK_GATED_MASK) != 0
    }

    /// Set SAR1_CLK_GATED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar1_clk_gated(&self) {
        self.value.modify_bits(|r| r | Self::SAR1_CLK_GATED_MASK);
    }

    /// Clear SAR1_CLK_GATED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar1_clk_gated(&self) {
        self.value.modify_bits(|r| r & !Self::SAR1_CLK_GATED_MASK);
    }

    /// **SAR1_CLK_DIV** `[7:0]` (RW)
    ///
    /// Clock divider.
    #[inline(always)]
    pub fn sar1_clk_div(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_CLK_DIV_MASK) >> Self::SAR1_CLK_DIV_OFFSET
    }

    /// Write **SAR1_CLK_DIV** `[7:0]` (RW)
    ///
    /// Clock divider.
    #[inline(always)]
    pub fn write_sar1_clk_div(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR1_CLK_DIV_MASK)
                | ((value << Self::SAR1_CLK_DIV_OFFSET) & Self::SAR1_CLK_DIV_MASK)
        });
    }
}

/// READER1_CTRL value type for batch read/modify/write operations
///
/// Use `reader1ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `reader1ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Reader1CtrlVal {
    bits: u32,
}

impl Reader1CtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x20040002 }
    }

    /// Read **SAR1_EN_PAD_FORCE_ENABLE** `[31:30]`
    ///
    /// Force enable adc en_pad to analog circuit 2'b11: force enable .
    #[inline(always)]
    pub const fn sar1_en_pad_force_enable(&self) -> u32 {
        (self.bits & Reader1Ctrl::SAR1_EN_PAD_FORCE_ENABLE_MASK)
            >> Reader1Ctrl::SAR1_EN_PAD_FORCE_ENABLE_OFFSET
    }

    /// Read **SAR1_INT_EN** `[29:29]`
    ///
    /// Enable saradc1 to send out interrupt.
    #[inline(always)]
    pub const fn sar1_int_en(&self) -> bool {
        (self.bits & Reader1Ctrl::SAR1_INT_EN_MASK) != 0
    }

    /// Read **SAR1_DATA_INV** `[28:28]`
    ///
    /// Invert SAR ADC1 data.
    #[inline(always)]
    pub const fn sar1_data_inv(&self) -> bool {
        (self.bits & Reader1Ctrl::SAR1_DATA_INV_MASK) != 0
    }

    /// Read **SAR1_SAMPLE_NUM** `[26:19]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar1_sample_num(&self) -> u32 {
        (self.bits & Reader1Ctrl::SAR1_SAMPLE_NUM_MASK) >> Reader1Ctrl::SAR1_SAMPLE_NUM_OFFSET
    }

    /// Read **SAR1_CLK_GATED** `[18:18]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar1_clk_gated(&self) -> bool {
        (self.bits & Reader1Ctrl::SAR1_CLK_GATED_MASK) != 0
    }

    /// Read **SAR1_CLK_DIV** `[7:0]`
    ///
    /// Clock divider.
    #[inline(always)]
    pub const fn sar1_clk_div(&self) -> u32 {
        (self.bits & Reader1Ctrl::SAR1_CLK_DIV_MASK) >> Reader1Ctrl::SAR1_CLK_DIV_OFFSET
    }

    /// Set **SAR1_EN_PAD_FORCE_ENABLE** `[31:30]`
    ///
    /// Force enable adc en_pad to analog circuit 2'b11: force enable .
    #[inline(always)]
    pub const fn with_sar1_en_pad_force_enable(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Reader1Ctrl::SAR1_EN_PAD_FORCE_ENABLE_MASK)
                | ((value << Reader1Ctrl::SAR1_EN_PAD_FORCE_ENABLE_OFFSET)
                    & Reader1Ctrl::SAR1_EN_PAD_FORCE_ENABLE_MASK),
        }
    }

    /// Set **SAR1_INT_EN** `[29:29]`
    ///
    /// Enable saradc1 to send out interrupt.
    #[inline(always)]
    pub const fn with_sar1_int_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reader1Ctrl::SAR1_INT_EN_MASK
            } else {
                self.bits & !Reader1Ctrl::SAR1_INT_EN_MASK
            },
        }
    }

    /// Set **SAR1_DATA_INV** `[28:28]`
    ///
    /// Invert SAR ADC1 data.
    #[inline(always)]
    pub const fn with_sar1_data_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reader1Ctrl::SAR1_DATA_INV_MASK
            } else {
                self.bits & !Reader1Ctrl::SAR1_DATA_INV_MASK
            },
        }
    }

    /// Set **SAR1_SAMPLE_NUM** `[26:19]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar1_sample_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Reader1Ctrl::SAR1_SAMPLE_NUM_MASK)
                | ((value << Reader1Ctrl::SAR1_SAMPLE_NUM_OFFSET)
                    & Reader1Ctrl::SAR1_SAMPLE_NUM_MASK),
        }
    }

    /// Set **SAR1_CLK_GATED** `[18:18]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar1_clk_gated(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reader1Ctrl::SAR1_CLK_GATED_MASK
            } else {
                self.bits & !Reader1Ctrl::SAR1_CLK_GATED_MASK
            },
        }
    }

    /// Set **SAR1_CLK_DIV** `[7:0]`
    ///
    /// Clock divider.
    #[inline(always)]
    pub const fn with_sar1_clk_div(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Reader1Ctrl::SAR1_CLK_DIV_MASK)
                | ((value << Reader1Ctrl::SAR1_CLK_DIV_OFFSET) & Reader1Ctrl::SAR1_CLK_DIV_MASK),
        }
    }
}

impl Default for Reader1CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Reader1Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Reader1CtrlVal {
        Reader1CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Reader1CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Reader1CtrlVal) -> Reader1CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// READER1_STATUS Register
///
/// N/A
///
/// Address offset: 0x0004
/// Reset value: 0x20000000
///
/// ## Fields
///
/// - **SAR1_READER_STATUS** `[31:0]` (RO)
///
///   N/A
#[repr(transparent)]
pub struct Reader1Status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Reader1Status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Reader1Status {
    // ======== Field Constants ========

    /// **SAR1_READER_STATUS**
    ///
    /// N/A
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SAR1_READER_STATUS_OFFSET: u32 = 0;
    pub const SAR1_READER_STATUS_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SAR1_READER_STATUS** `[31:0]` (RO)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar1_reader_status(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_READER_STATUS_MASK) >> Self::SAR1_READER_STATUS_OFFSET
    }
}

/// READER1_STATUS value type for batch read/modify/write operations
///
/// Use `reader1status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `reader1status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Reader1StatusVal {
    bits: u32,
}

impl Reader1StatusVal {
    /// Create from raw bits
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
        Self { bits: 0x20000000 }
    }

    /// Read **SAR1_READER_STATUS** `[31:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar1_reader_status(&self) -> u32 {
        (self.bits & Reader1Status::SAR1_READER_STATUS_MASK)
            >> Reader1Status::SAR1_READER_STATUS_OFFSET
    }
}

impl Default for Reader1StatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Reader1Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Reader1StatusVal {
        Reader1StatusVal::from_bits(self.value.read_bits())
    }
}

/// MEAS1_CTRL1 Register
///
/// N/A
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **AMP_SHORT_REF_GND_FORCE** `[31:30]` (RW)
///
///   N/A
///
/// - **AMP_SHORT_REF_FORCE** `[29:28]` (RW)
///
///   N/A
///
/// - **AMP_RST_FB_FORCE** `[27:26]` (RW)
///
///   N/A
///
/// - **FORCE_XPD_AMP** `[25:24]` (RW)
///
///   N/A
#[repr(transparent)]
pub struct Meas1Ctrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Meas1Ctrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Meas1Ctrl1 {
    // ======== Field Constants ========

    /// **AMP_SHORT_REF_GND_FORCE**
    ///
    /// N/A
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const AMP_SHORT_REF_GND_FORCE_OFFSET: u32 = 30;
    pub const AMP_SHORT_REF_GND_FORCE_MASK: u32 = 0xc0000000;

    /// **AMP_SHORT_REF_FORCE**
    ///
    /// N/A
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const AMP_SHORT_REF_FORCE_OFFSET: u32 = 28;
    pub const AMP_SHORT_REF_FORCE_MASK: u32 = 0x30000000;

    /// **AMP_RST_FB_FORCE**
    ///
    /// N/A
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const AMP_RST_FB_FORCE_OFFSET: u32 = 26;
    pub const AMP_RST_FB_FORCE_MASK: u32 = 0x0c000000;

    /// **FORCE_XPD_AMP**
    ///
    /// N/A
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const FORCE_XPD_AMP_OFFSET: u32 = 24;
    pub const FORCE_XPD_AMP_MASK: u32 = 0x03000000;

    // ======== Field Accessors ========

    /// **AMP_SHORT_REF_GND_FORCE** `[31:30]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn amp_short_ref_gnd_force(&self) -> u32 {
        (self.value.read_bits() & Self::AMP_SHORT_REF_GND_FORCE_MASK)
            >> Self::AMP_SHORT_REF_GND_FORCE_OFFSET
    }

    /// Write **AMP_SHORT_REF_GND_FORCE** `[31:30]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_amp_short_ref_gnd_force(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AMP_SHORT_REF_GND_FORCE_MASK)
                | ((value << Self::AMP_SHORT_REF_GND_FORCE_OFFSET)
                    & Self::AMP_SHORT_REF_GND_FORCE_MASK)
        });
    }

    /// **AMP_SHORT_REF_FORCE** `[29:28]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn amp_short_ref_force(&self) -> u32 {
        (self.value.read_bits() & Self::AMP_SHORT_REF_FORCE_MASK)
            >> Self::AMP_SHORT_REF_FORCE_OFFSET
    }

    /// Write **AMP_SHORT_REF_FORCE** `[29:28]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_amp_short_ref_force(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AMP_SHORT_REF_FORCE_MASK)
                | ((value << Self::AMP_SHORT_REF_FORCE_OFFSET) & Self::AMP_SHORT_REF_FORCE_MASK)
        });
    }

    /// **AMP_RST_FB_FORCE** `[27:26]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn amp_rst_fb_force(&self) -> u32 {
        (self.value.read_bits() & Self::AMP_RST_FB_FORCE_MASK) >> Self::AMP_RST_FB_FORCE_OFFSET
    }

    /// Write **AMP_RST_FB_FORCE** `[27:26]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_amp_rst_fb_force(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AMP_RST_FB_FORCE_MASK)
                | ((value << Self::AMP_RST_FB_FORCE_OFFSET) & Self::AMP_RST_FB_FORCE_MASK)
        });
    }

    /// **FORCE_XPD_AMP** `[25:24]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn force_xpd_amp(&self) -> u32 {
        (self.value.read_bits() & Self::FORCE_XPD_AMP_MASK) >> Self::FORCE_XPD_AMP_OFFSET
    }

    /// Write **FORCE_XPD_AMP** `[25:24]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_force_xpd_amp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FORCE_XPD_AMP_MASK)
                | ((value << Self::FORCE_XPD_AMP_OFFSET) & Self::FORCE_XPD_AMP_MASK)
        });
    }
}

/// MEAS1_CTRL1 value type for batch read/modify/write operations
///
/// Use `meas1ctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `meas1ctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Meas1Ctrl1Val {
    bits: u32,
}

impl Meas1Ctrl1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **AMP_SHORT_REF_GND_FORCE** `[31:30]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn amp_short_ref_gnd_force(&self) -> u32 {
        (self.bits & Meas1Ctrl1::AMP_SHORT_REF_GND_FORCE_MASK)
            >> Meas1Ctrl1::AMP_SHORT_REF_GND_FORCE_OFFSET
    }

    /// Read **AMP_SHORT_REF_FORCE** `[29:28]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn amp_short_ref_force(&self) -> u32 {
        (self.bits & Meas1Ctrl1::AMP_SHORT_REF_FORCE_MASK) >> Meas1Ctrl1::AMP_SHORT_REF_FORCE_OFFSET
    }

    /// Read **AMP_RST_FB_FORCE** `[27:26]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn amp_rst_fb_force(&self) -> u32 {
        (self.bits & Meas1Ctrl1::AMP_RST_FB_FORCE_MASK) >> Meas1Ctrl1::AMP_RST_FB_FORCE_OFFSET
    }

    /// Read **FORCE_XPD_AMP** `[25:24]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn force_xpd_amp(&self) -> u32 {
        (self.bits & Meas1Ctrl1::FORCE_XPD_AMP_MASK) >> Meas1Ctrl1::FORCE_XPD_AMP_OFFSET
    }

    /// Set **AMP_SHORT_REF_GND_FORCE** `[31:30]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_amp_short_ref_gnd_force(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Meas1Ctrl1::AMP_SHORT_REF_GND_FORCE_MASK)
                | ((value << Meas1Ctrl1::AMP_SHORT_REF_GND_FORCE_OFFSET)
                    & Meas1Ctrl1::AMP_SHORT_REF_GND_FORCE_MASK),
        }
    }

    /// Set **AMP_SHORT_REF_FORCE** `[29:28]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_amp_short_ref_force(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Meas1Ctrl1::AMP_SHORT_REF_FORCE_MASK)
                | ((value << Meas1Ctrl1::AMP_SHORT_REF_FORCE_OFFSET)
                    & Meas1Ctrl1::AMP_SHORT_REF_FORCE_MASK),
        }
    }

    /// Set **AMP_RST_FB_FORCE** `[27:26]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_amp_rst_fb_force(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Meas1Ctrl1::AMP_RST_FB_FORCE_MASK)
                | ((value << Meas1Ctrl1::AMP_RST_FB_FORCE_OFFSET)
                    & Meas1Ctrl1::AMP_RST_FB_FORCE_MASK),
        }
    }

    /// Set **FORCE_XPD_AMP** `[25:24]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_force_xpd_amp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Meas1Ctrl1::FORCE_XPD_AMP_MASK)
                | ((value << Meas1Ctrl1::FORCE_XPD_AMP_OFFSET) & Meas1Ctrl1::FORCE_XPD_AMP_MASK),
        }
    }
}

impl Default for Meas1Ctrl1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Meas1Ctrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Meas1Ctrl1Val {
        Meas1Ctrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Meas1Ctrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Meas1Ctrl1Val) -> Meas1Ctrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MEAS1_CTRL2 Register
///
/// ADC1 configuration registers.
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **SAR1_EN_PAD_FORCE** `[31]` (RW)
///
///   1: SAR ADC1 pad enable bitmap is controlled by SW.
///
/// - **SAR1_EN_PAD** `[30:19]` (RW)
///
///   SAR ADC1 pad enable bitmap.
///
/// - **MEAS1_START_FORCE** `[18]` (RW)
///
///   1: SAR ADC1 controller (in RTC) is started by SW.
///
/// - **MEAS1_START_SAR** `[17]` (RW)
///
///   SAR ADC1 controller (in RTC) starts conversion.
///
/// - **MEAS1_DONE_SAR** `[16]` (RO)
///
///   SAR ADC1 conversion done indication.
///
/// - **MEAS1_DATA_SAR** `[15:0]` (RO)
///
///   SAR ADC1 data.
#[repr(transparent)]
pub struct Meas1Ctrl2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Meas1Ctrl2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Meas1Ctrl2 {
    // ======== Field Constants ========

    /// **SAR1_EN_PAD_FORCE**
    ///
    /// 1: SAR ADC1 pad enable bitmap is controlled by SW.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SAR1_EN_PAD_FORCE_OFFSET: u32 = 31;
    pub const SAR1_EN_PAD_FORCE_MASK: u32 = 0x80000000;
    pub const SAR1_EN_PAD_FORCE: u32 = 1 << 31;

    /// **SAR1_EN_PAD**
    ///
    /// SAR ADC1 pad enable bitmap.
    ///
    /// Bits `[30:19]`, mask `0x7ff80000`
    pub const SAR1_EN_PAD_OFFSET: u32 = 19;
    pub const SAR1_EN_PAD_MASK: u32 = 0x7ff80000;

    /// **MEAS1_START_FORCE**
    ///
    /// 1: SAR ADC1 controller (in RTC) is started by SW.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const MEAS1_START_FORCE_OFFSET: u32 = 18;
    pub const MEAS1_START_FORCE_MASK: u32 = 0x00040000;
    pub const MEAS1_START_FORCE: u32 = 1 << 18;

    /// **MEAS1_START_SAR**
    ///
    /// SAR ADC1 controller (in RTC) starts conversion.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const MEAS1_START_SAR_OFFSET: u32 = 17;
    pub const MEAS1_START_SAR_MASK: u32 = 0x00020000;
    pub const MEAS1_START_SAR: u32 = 1 << 17;

    /// **MEAS1_DONE_SAR**
    ///
    /// SAR ADC1 conversion done indication.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const MEAS1_DONE_SAR_OFFSET: u32 = 16;
    pub const MEAS1_DONE_SAR_MASK: u32 = 0x00010000;
    pub const MEAS1_DONE_SAR: u32 = 1 << 16;

    /// **MEAS1_DATA_SAR**
    ///
    /// SAR ADC1 data.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MEAS1_DATA_SAR_OFFSET: u32 = 0;
    pub const MEAS1_DATA_SAR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SAR1_EN_PAD_FORCE** `[31]` (RW)
    ///
    /// 1: SAR ADC1 pad enable bitmap is controlled by SW.
    #[inline(always)]
    pub fn sar1_en_pad_force(&self) -> bool {
        (self.value.read_bits() & Self::SAR1_EN_PAD_FORCE_MASK) != 0
    }

    /// Set SAR1_EN_PAD_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar1_en_pad_force(&self) {
        self.value.modify_bits(|r| r | Self::SAR1_EN_PAD_FORCE_MASK);
    }

    /// Clear SAR1_EN_PAD_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar1_en_pad_force(&self) {
        self.value
            .modify_bits(|r| r & !Self::SAR1_EN_PAD_FORCE_MASK);
    }

    /// **SAR1_EN_PAD** `[30:19]` (RW)
    ///
    /// SAR ADC1 pad enable bitmap.
    #[inline(always)]
    pub fn sar1_en_pad(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_EN_PAD_MASK) >> Self::SAR1_EN_PAD_OFFSET
    }

    /// Write **SAR1_EN_PAD** `[30:19]` (RW)
    ///
    /// SAR ADC1 pad enable bitmap.
    #[inline(always)]
    pub fn write_sar1_en_pad(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR1_EN_PAD_MASK)
                | ((value << Self::SAR1_EN_PAD_OFFSET) & Self::SAR1_EN_PAD_MASK)
        });
    }

    /// **MEAS1_START_FORCE** `[18]` (RW)
    ///
    /// 1: SAR ADC1 controller (in RTC) is started by SW.
    #[inline(always)]
    pub fn meas1_start_force(&self) -> bool {
        (self.value.read_bits() & Self::MEAS1_START_FORCE_MASK) != 0
    }

    /// Set MEAS1_START_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_meas1_start_force(&self) {
        self.value.modify_bits(|r| r | Self::MEAS1_START_FORCE_MASK);
    }

    /// Clear MEAS1_START_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_meas1_start_force(&self) {
        self.value
            .modify_bits(|r| r & !Self::MEAS1_START_FORCE_MASK);
    }

    /// **MEAS1_START_SAR** `[17]` (RW)
    ///
    /// SAR ADC1 controller (in RTC) starts conversion.
    #[inline(always)]
    pub fn meas1_start_sar(&self) -> bool {
        (self.value.read_bits() & Self::MEAS1_START_SAR_MASK) != 0
    }

    /// Set MEAS1_START_SAR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_meas1_start_sar(&self) {
        self.value.modify_bits(|r| r | Self::MEAS1_START_SAR_MASK);
    }

    /// Clear MEAS1_START_SAR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_meas1_start_sar(&self) {
        self.value.modify_bits(|r| r & !Self::MEAS1_START_SAR_MASK);
    }

    /// **MEAS1_DONE_SAR** `[16]` (RO)
    ///
    /// SAR ADC1 conversion done indication.
    #[inline(always)]
    pub fn meas1_done_sar(&self) -> bool {
        (self.value.read_bits() & Self::MEAS1_DONE_SAR_MASK) != 0
    }

    /// **MEAS1_DATA_SAR** `[15:0]` (RO)
    ///
    /// SAR ADC1 data.
    #[inline(always)]
    pub fn meas1_data_sar(&self) -> u32 {
        (self.value.read_bits() & Self::MEAS1_DATA_SAR_MASK) >> Self::MEAS1_DATA_SAR_OFFSET
    }
}

/// MEAS1_CTRL2 value type for batch read/modify/write operations
///
/// Use `meas1ctrl2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `meas1ctrl2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Meas1Ctrl2Val {
    bits: u32,
}

impl Meas1Ctrl2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR1_EN_PAD_FORCE** `[31:31]`
    ///
    /// 1: SAR ADC1 pad enable bitmap is controlled by SW.
    #[inline(always)]
    pub const fn sar1_en_pad_force(&self) -> bool {
        (self.bits & Meas1Ctrl2::SAR1_EN_PAD_FORCE_MASK) != 0
    }

    /// Read **SAR1_EN_PAD** `[30:19]`
    ///
    /// SAR ADC1 pad enable bitmap.
    #[inline(always)]
    pub const fn sar1_en_pad(&self) -> u32 {
        (self.bits & Meas1Ctrl2::SAR1_EN_PAD_MASK) >> Meas1Ctrl2::SAR1_EN_PAD_OFFSET
    }

    /// Read **MEAS1_START_FORCE** `[18:18]`
    ///
    /// 1: SAR ADC1 controller (in RTC) is started by SW.
    #[inline(always)]
    pub const fn meas1_start_force(&self) -> bool {
        (self.bits & Meas1Ctrl2::MEAS1_START_FORCE_MASK) != 0
    }

    /// Read **MEAS1_START_SAR** `[17:17]`
    ///
    /// SAR ADC1 controller (in RTC) starts conversion.
    #[inline(always)]
    pub const fn meas1_start_sar(&self) -> bool {
        (self.bits & Meas1Ctrl2::MEAS1_START_SAR_MASK) != 0
    }

    /// Read **MEAS1_DONE_SAR** `[16:16]`
    ///
    /// SAR ADC1 conversion done indication.
    #[inline(always)]
    pub const fn meas1_done_sar(&self) -> bool {
        (self.bits & Meas1Ctrl2::MEAS1_DONE_SAR_MASK) != 0
    }

    /// Read **MEAS1_DATA_SAR** `[15:0]`
    ///
    /// SAR ADC1 data.
    #[inline(always)]
    pub const fn meas1_data_sar(&self) -> u32 {
        (self.bits & Meas1Ctrl2::MEAS1_DATA_SAR_MASK) >> Meas1Ctrl2::MEAS1_DATA_SAR_OFFSET
    }

    /// Set **SAR1_EN_PAD_FORCE** `[31:31]`
    ///
    /// 1: SAR ADC1 pad enable bitmap is controlled by SW.
    #[inline(always)]
    pub const fn with_sar1_en_pad_force(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Meas1Ctrl2::SAR1_EN_PAD_FORCE_MASK
            } else {
                self.bits & !Meas1Ctrl2::SAR1_EN_PAD_FORCE_MASK
            },
        }
    }

    /// Set **SAR1_EN_PAD** `[30:19]`
    ///
    /// SAR ADC1 pad enable bitmap.
    #[inline(always)]
    pub const fn with_sar1_en_pad(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Meas1Ctrl2::SAR1_EN_PAD_MASK)
                | ((value << Meas1Ctrl2::SAR1_EN_PAD_OFFSET) & Meas1Ctrl2::SAR1_EN_PAD_MASK),
        }
    }

    /// Set **MEAS1_START_FORCE** `[18:18]`
    ///
    /// 1: SAR ADC1 controller (in RTC) is started by SW.
    #[inline(always)]
    pub const fn with_meas1_start_force(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Meas1Ctrl2::MEAS1_START_FORCE_MASK
            } else {
                self.bits & !Meas1Ctrl2::MEAS1_START_FORCE_MASK
            },
        }
    }

    /// Set **MEAS1_START_SAR** `[17:17]`
    ///
    /// SAR ADC1 controller (in RTC) starts conversion.
    #[inline(always)]
    pub const fn with_meas1_start_sar(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Meas1Ctrl2::MEAS1_START_SAR_MASK
            } else {
                self.bits & !Meas1Ctrl2::MEAS1_START_SAR_MASK
            },
        }
    }
}

impl Default for Meas1Ctrl2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Meas1Ctrl2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Meas1Ctrl2Val {
        Meas1Ctrl2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Meas1Ctrl2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Meas1Ctrl2Val) -> Meas1Ctrl2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MEAS1_MUX Register
///
/// SAR ADC1 MUX register.
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **SAR1_DIG_FORCE** `[31]` (RW)
///
///   1: SAR ADC1 controlled by DIG ADC1 CTRL.
#[repr(transparent)]
pub struct Meas1Mux {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Meas1Mux {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Meas1Mux {
    // ======== Field Constants ========

    /// **SAR1_DIG_FORCE**
    ///
    /// 1: SAR ADC1 controlled by DIG ADC1 CTRL.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SAR1_DIG_FORCE_OFFSET: u32 = 31;
    pub const SAR1_DIG_FORCE_MASK: u32 = 0x80000000;
    pub const SAR1_DIG_FORCE: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **SAR1_DIG_FORCE** `[31]` (RW)
    ///
    /// 1: SAR ADC1 controlled by DIG ADC1 CTRL.
    #[inline(always)]
    pub fn sar1_dig_force(&self) -> bool {
        (self.value.read_bits() & Self::SAR1_DIG_FORCE_MASK) != 0
    }

    /// Set SAR1_DIG_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar1_dig_force(&self) {
        self.value.modify_bits(|r| r | Self::SAR1_DIG_FORCE_MASK);
    }

    /// Clear SAR1_DIG_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar1_dig_force(&self) {
        self.value.modify_bits(|r| r & !Self::SAR1_DIG_FORCE_MASK);
    }
}

/// MEAS1_MUX value type for batch read/modify/write operations
///
/// Use `meas1mux.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `meas1mux.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Meas1MuxVal {
    bits: u32,
}

impl Meas1MuxVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR1_DIG_FORCE** `[31:31]`
    ///
    /// 1: SAR ADC1 controlled by DIG ADC1 CTRL.
    #[inline(always)]
    pub const fn sar1_dig_force(&self) -> bool {
        (self.bits & Meas1Mux::SAR1_DIG_FORCE_MASK) != 0
    }

    /// Set **SAR1_DIG_FORCE** `[31:31]`
    ///
    /// 1: SAR ADC1 controlled by DIG ADC1 CTRL.
    #[inline(always)]
    pub const fn with_sar1_dig_force(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Meas1Mux::SAR1_DIG_FORCE_MASK
            } else {
                self.bits & !Meas1Mux::SAR1_DIG_FORCE_MASK
            },
        }
    }
}

impl Default for Meas1MuxVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Meas1Mux {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Meas1MuxVal {
        Meas1MuxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Meas1MuxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Meas1MuxVal) -> Meas1MuxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ATTEN1 Register
///
/// ADC1 attenuation registers.
///
/// Address offset: 0x0014
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **SAR1_ATTEN** `[31:0]` (RW)
///
///   2-bit attenuation for each pad.
#[repr(transparent)]
pub struct Atten1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Atten1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Atten1 {
    // ======== Field Constants ========

    /// **SAR1_ATTEN**
    ///
    /// 2-bit attenuation for each pad.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SAR1_ATTEN_OFFSET: u32 = 0;
    pub const SAR1_ATTEN_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SAR1_ATTEN** `[31:0]` (RW)
    ///
    /// 2-bit attenuation for each pad.
    #[inline(always)]
    pub fn sar1_atten(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_ATTEN_MASK) >> Self::SAR1_ATTEN_OFFSET
    }

    /// Write **SAR1_ATTEN** `[31:0]` (RW)
    ///
    /// 2-bit attenuation for each pad.
    #[inline(always)]
    pub fn write_sar1_atten(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR1_ATTEN_MASK)
                | ((value << Self::SAR1_ATTEN_OFFSET) & Self::SAR1_ATTEN_MASK)
        });
    }
}

/// ATTEN1 value type for batch read/modify/write operations
///
/// Use `atten1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atten1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Atten1Val {
    bits: u32,
}

impl Atten1Val {
    /// Create from raw bits
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

    /// Read **SAR1_ATTEN** `[31:0]`
    ///
    /// 2-bit attenuation for each pad.
    #[inline(always)]
    pub const fn sar1_atten(&self) -> u32 {
        (self.bits & Atten1::SAR1_ATTEN_MASK) >> Atten1::SAR1_ATTEN_OFFSET
    }

    /// Set **SAR1_ATTEN** `[31:0]`
    ///
    /// 2-bit attenuation for each pad.
    #[inline(always)]
    pub const fn with_sar1_atten(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atten1::SAR1_ATTEN_MASK)
                | ((value << Atten1::SAR1_ATTEN_OFFSET) & Atten1::SAR1_ATTEN_MASK),
        }
    }
}

impl Default for Atten1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Atten1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Atten1Val {
        Atten1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Atten1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Atten1Val) -> Atten1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AMP_CTRL1 Register
///
/// N/A
///
/// Address offset: 0x0018
/// Reset value: 0x000a000a
///
/// ## Fields
///
/// - **SAR_AMP_WAIT2** `[31:16]` (RW)
///
///   N/A
///
/// - **SAR_AMP_WAIT1** `[15:0]` (RW)
///
///   N/A
#[repr(transparent)]
pub struct AmpCtrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AmpCtrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AmpCtrl1 {
    // ======== Field Constants ========

    /// **SAR_AMP_WAIT2**
    ///
    /// N/A
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const SAR_AMP_WAIT2_OFFSET: u32 = 16;
    pub const SAR_AMP_WAIT2_MASK: u32 = 0xffff0000;

    /// **SAR_AMP_WAIT1**
    ///
    /// N/A
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SAR_AMP_WAIT1_OFFSET: u32 = 0;
    pub const SAR_AMP_WAIT1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SAR_AMP_WAIT2** `[31:16]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar_amp_wait2(&self) -> u32 {
        (self.value.read_bits() & Self::SAR_AMP_WAIT2_MASK) >> Self::SAR_AMP_WAIT2_OFFSET
    }

    /// Write **SAR_AMP_WAIT2** `[31:16]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_sar_amp_wait2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR_AMP_WAIT2_MASK)
                | ((value << Self::SAR_AMP_WAIT2_OFFSET) & Self::SAR_AMP_WAIT2_MASK)
        });
    }

    /// **SAR_AMP_WAIT1** `[15:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar_amp_wait1(&self) -> u32 {
        (self.value.read_bits() & Self::SAR_AMP_WAIT1_MASK) >> Self::SAR_AMP_WAIT1_OFFSET
    }

    /// Write **SAR_AMP_WAIT1** `[15:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_sar_amp_wait1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR_AMP_WAIT1_MASK)
                | ((value << Self::SAR_AMP_WAIT1_OFFSET) & Self::SAR_AMP_WAIT1_MASK)
        });
    }
}

/// AMP_CTRL1 value type for batch read/modify/write operations
///
/// Use `ampctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ampctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AmpCtrl1Val {
    bits: u32,
}

impl AmpCtrl1Val {
    /// Create from raw bits
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
        Self { bits: 0x000a000a }
    }

    /// Read **SAR_AMP_WAIT2** `[31:16]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar_amp_wait2(&self) -> u32 {
        (self.bits & AmpCtrl1::SAR_AMP_WAIT2_MASK) >> AmpCtrl1::SAR_AMP_WAIT2_OFFSET
    }

    /// Read **SAR_AMP_WAIT1** `[15:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar_amp_wait1(&self) -> u32 {
        (self.bits & AmpCtrl1::SAR_AMP_WAIT1_MASK) >> AmpCtrl1::SAR_AMP_WAIT1_OFFSET
    }

    /// Set **SAR_AMP_WAIT2** `[31:16]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar_amp_wait2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AmpCtrl1::SAR_AMP_WAIT2_MASK)
                | ((value << AmpCtrl1::SAR_AMP_WAIT2_OFFSET) & AmpCtrl1::SAR_AMP_WAIT2_MASK),
        }
    }

    /// Set **SAR_AMP_WAIT1** `[15:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar_amp_wait1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AmpCtrl1::SAR_AMP_WAIT1_MASK)
                | ((value << AmpCtrl1::SAR_AMP_WAIT1_OFFSET) & AmpCtrl1::SAR_AMP_WAIT1_MASK),
        }
    }
}

impl Default for AmpCtrl1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AmpCtrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AmpCtrl1Val {
        AmpCtrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AmpCtrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AmpCtrl1Val) -> AmpCtrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AMP_CTRL2 Register
///
/// N/A
///
/// Address offset: 0x001c
/// Reset value: 0x000a0000
///
/// ## Fields
///
/// - **SAR_AMP_WAIT3** `[31:16]` (RW)
///
///   N/A
///
/// - **SAR_RSTB_FSM_IDLE** `[6]` (RW)
///
///   N/A
///
/// - **XPD_SAR_FSM_IDLE** `[5]` (RW)
///
///   N/A
///
/// - **AMP_SHORT_REF_GND_FSM_IDLE** `[4]` (RW)
///
///   N/A
///
/// - **AMP_SHORT_REF_FSM_IDLE** `[3]` (RW)
///
///   N/A
///
/// - **AMP_RST_FB_FSM_IDLE** `[2]` (RW)
///
///   N/A
///
/// - **XPD_SAR_AMP_FSM_IDLE** `[1]` (RW)
///
///   N/A
///
/// - **SAR1_DAC_XPD_FSM_IDLE** `[0]` (RW)
///
///   N/A
#[repr(transparent)]
pub struct AmpCtrl2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AmpCtrl2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AmpCtrl2 {
    // ======== Field Constants ========

    /// **SAR_AMP_WAIT3**
    ///
    /// N/A
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const SAR_AMP_WAIT3_OFFSET: u32 = 16;
    pub const SAR_AMP_WAIT3_MASK: u32 = 0xffff0000;

    /// **SAR_RSTB_FSM_IDLE**
    ///
    /// N/A
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SAR_RSTB_FSM_IDLE_OFFSET: u32 = 6;
    pub const SAR_RSTB_FSM_IDLE_MASK: u32 = 0x00000040;
    pub const SAR_RSTB_FSM_IDLE: u32 = 1 << 6;

    /// **XPD_SAR_FSM_IDLE**
    ///
    /// N/A
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const XPD_SAR_FSM_IDLE_OFFSET: u32 = 5;
    pub const XPD_SAR_FSM_IDLE_MASK: u32 = 0x00000020;
    pub const XPD_SAR_FSM_IDLE: u32 = 1 << 5;

    /// **AMP_SHORT_REF_GND_FSM_IDLE**
    ///
    /// N/A
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const AMP_SHORT_REF_GND_FSM_IDLE_OFFSET: u32 = 4;
    pub const AMP_SHORT_REF_GND_FSM_IDLE_MASK: u32 = 0x00000010;
    pub const AMP_SHORT_REF_GND_FSM_IDLE: u32 = 1 << 4;

    /// **AMP_SHORT_REF_FSM_IDLE**
    ///
    /// N/A
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const AMP_SHORT_REF_FSM_IDLE_OFFSET: u32 = 3;
    pub const AMP_SHORT_REF_FSM_IDLE_MASK: u32 = 0x00000008;
    pub const AMP_SHORT_REF_FSM_IDLE: u32 = 1 << 3;

    /// **AMP_RST_FB_FSM_IDLE**
    ///
    /// N/A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const AMP_RST_FB_FSM_IDLE_OFFSET: u32 = 2;
    pub const AMP_RST_FB_FSM_IDLE_MASK: u32 = 0x00000004;
    pub const AMP_RST_FB_FSM_IDLE: u32 = 1 << 2;

    /// **XPD_SAR_AMP_FSM_IDLE**
    ///
    /// N/A
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const XPD_SAR_AMP_FSM_IDLE_OFFSET: u32 = 1;
    pub const XPD_SAR_AMP_FSM_IDLE_MASK: u32 = 0x00000002;
    pub const XPD_SAR_AMP_FSM_IDLE: u32 = 1 << 1;

    /// **SAR1_DAC_XPD_FSM_IDLE**
    ///
    /// N/A
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SAR1_DAC_XPD_FSM_IDLE_OFFSET: u32 = 0;
    pub const SAR1_DAC_XPD_FSM_IDLE_MASK: u32 = 0x00000001;
    pub const SAR1_DAC_XPD_FSM_IDLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SAR_AMP_WAIT3** `[31:16]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar_amp_wait3(&self) -> u32 {
        (self.value.read_bits() & Self::SAR_AMP_WAIT3_MASK) >> Self::SAR_AMP_WAIT3_OFFSET
    }

    /// Write **SAR_AMP_WAIT3** `[31:16]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_sar_amp_wait3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR_AMP_WAIT3_MASK)
                | ((value << Self::SAR_AMP_WAIT3_OFFSET) & Self::SAR_AMP_WAIT3_MASK)
        });
    }

    /// **SAR_RSTB_FSM_IDLE** `[6]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&self) -> bool {
        (self.value.read_bits() & Self::SAR_RSTB_FSM_IDLE_MASK) != 0
    }

    /// Set SAR_RSTB_FSM_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar_rstb_fsm_idle(&self) {
        self.value.modify_bits(|r| r | Self::SAR_RSTB_FSM_IDLE_MASK);
    }

    /// Clear SAR_RSTB_FSM_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar_rstb_fsm_idle(&self) {
        self.value
            .modify_bits(|r| r & !Self::SAR_RSTB_FSM_IDLE_MASK);
    }

    /// **XPD_SAR_FSM_IDLE** `[5]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&self) -> bool {
        (self.value.read_bits() & Self::XPD_SAR_FSM_IDLE_MASK) != 0
    }

    /// Set XPD_SAR_FSM_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_xpd_sar_fsm_idle(&self) {
        self.value.modify_bits(|r| r | Self::XPD_SAR_FSM_IDLE_MASK);
    }

    /// Clear XPD_SAR_FSM_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_xpd_sar_fsm_idle(&self) {
        self.value.modify_bits(|r| r & !Self::XPD_SAR_FSM_IDLE_MASK);
    }

    /// **AMP_SHORT_REF_GND_FSM_IDLE** `[4]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&self) -> bool {
        (self.value.read_bits() & Self::AMP_SHORT_REF_GND_FSM_IDLE_MASK) != 0
    }

    /// Set AMP_SHORT_REF_GND_FSM_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_amp_short_ref_gnd_fsm_idle(&self) {
        self.value
            .modify_bits(|r| r | Self::AMP_SHORT_REF_GND_FSM_IDLE_MASK);
    }

    /// Clear AMP_SHORT_REF_GND_FSM_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_amp_short_ref_gnd_fsm_idle(&self) {
        self.value
            .modify_bits(|r| r & !Self::AMP_SHORT_REF_GND_FSM_IDLE_MASK);
    }

    /// **AMP_SHORT_REF_FSM_IDLE** `[3]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&self) -> bool {
        (self.value.read_bits() & Self::AMP_SHORT_REF_FSM_IDLE_MASK) != 0
    }

    /// Set AMP_SHORT_REF_FSM_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_amp_short_ref_fsm_idle(&self) {
        self.value
            .modify_bits(|r| r | Self::AMP_SHORT_REF_FSM_IDLE_MASK);
    }

    /// Clear AMP_SHORT_REF_FSM_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_amp_short_ref_fsm_idle(&self) {
        self.value
            .modify_bits(|r| r & !Self::AMP_SHORT_REF_FSM_IDLE_MASK);
    }

    /// **AMP_RST_FB_FSM_IDLE** `[2]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&self) -> bool {
        (self.value.read_bits() & Self::AMP_RST_FB_FSM_IDLE_MASK) != 0
    }

    /// Set AMP_RST_FB_FSM_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_amp_rst_fb_fsm_idle(&self) {
        self.value
            .modify_bits(|r| r | Self::AMP_RST_FB_FSM_IDLE_MASK);
    }

    /// Clear AMP_RST_FB_FSM_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_amp_rst_fb_fsm_idle(&self) {
        self.value
            .modify_bits(|r| r & !Self::AMP_RST_FB_FSM_IDLE_MASK);
    }

    /// **XPD_SAR_AMP_FSM_IDLE** `[1]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&self) -> bool {
        (self.value.read_bits() & Self::XPD_SAR_AMP_FSM_IDLE_MASK) != 0
    }

    /// Set XPD_SAR_AMP_FSM_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_xpd_sar_amp_fsm_idle(&self) {
        self.value
            .modify_bits(|r| r | Self::XPD_SAR_AMP_FSM_IDLE_MASK);
    }

    /// Clear XPD_SAR_AMP_FSM_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_xpd_sar_amp_fsm_idle(&self) {
        self.value
            .modify_bits(|r| r & !Self::XPD_SAR_AMP_FSM_IDLE_MASK);
    }

    /// **SAR1_DAC_XPD_FSM_IDLE** `[0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&self) -> bool {
        (self.value.read_bits() & Self::SAR1_DAC_XPD_FSM_IDLE_MASK) != 0
    }

    /// Set SAR1_DAC_XPD_FSM_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar1_dac_xpd_fsm_idle(&self) {
        self.value
            .modify_bits(|r| r | Self::SAR1_DAC_XPD_FSM_IDLE_MASK);
    }

    /// Clear SAR1_DAC_XPD_FSM_IDLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar1_dac_xpd_fsm_idle(&self) {
        self.value
            .modify_bits(|r| r & !Self::SAR1_DAC_XPD_FSM_IDLE_MASK);
    }
}

/// AMP_CTRL2 value type for batch read/modify/write operations
///
/// Use `ampctrl2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ampctrl2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AmpCtrl2Val {
    bits: u32,
}

impl AmpCtrl2Val {
    /// Create from raw bits
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
        Self { bits: 0x000a0000 }
    }

    /// Read **SAR_AMP_WAIT3** `[31:16]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar_amp_wait3(&self) -> u32 {
        (self.bits & AmpCtrl2::SAR_AMP_WAIT3_MASK) >> AmpCtrl2::SAR_AMP_WAIT3_OFFSET
    }

    /// Read **SAR_RSTB_FSM_IDLE** `[6:6]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar_rstb_fsm_idle(&self) -> bool {
        (self.bits & AmpCtrl2::SAR_RSTB_FSM_IDLE_MASK) != 0
    }

    /// Read **XPD_SAR_FSM_IDLE** `[5:5]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn xpd_sar_fsm_idle(&self) -> bool {
        (self.bits & AmpCtrl2::XPD_SAR_FSM_IDLE_MASK) != 0
    }

    /// Read **AMP_SHORT_REF_GND_FSM_IDLE** `[4:4]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn amp_short_ref_gnd_fsm_idle(&self) -> bool {
        (self.bits & AmpCtrl2::AMP_SHORT_REF_GND_FSM_IDLE_MASK) != 0
    }

    /// Read **AMP_SHORT_REF_FSM_IDLE** `[3:3]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn amp_short_ref_fsm_idle(&self) -> bool {
        (self.bits & AmpCtrl2::AMP_SHORT_REF_FSM_IDLE_MASK) != 0
    }

    /// Read **AMP_RST_FB_FSM_IDLE** `[2:2]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn amp_rst_fb_fsm_idle(&self) -> bool {
        (self.bits & AmpCtrl2::AMP_RST_FB_FSM_IDLE_MASK) != 0
    }

    /// Read **XPD_SAR_AMP_FSM_IDLE** `[1:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn xpd_sar_amp_fsm_idle(&self) -> bool {
        (self.bits & AmpCtrl2::XPD_SAR_AMP_FSM_IDLE_MASK) != 0
    }

    /// Read **SAR1_DAC_XPD_FSM_IDLE** `[0:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar1_dac_xpd_fsm_idle(&self) -> bool {
        (self.bits & AmpCtrl2::SAR1_DAC_XPD_FSM_IDLE_MASK) != 0
    }

    /// Set **SAR_AMP_WAIT3** `[31:16]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar_amp_wait3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AmpCtrl2::SAR_AMP_WAIT3_MASK)
                | ((value << AmpCtrl2::SAR_AMP_WAIT3_OFFSET) & AmpCtrl2::SAR_AMP_WAIT3_MASK),
        }
    }

    /// Set **SAR_RSTB_FSM_IDLE** `[6:6]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar_rstb_fsm_idle(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AmpCtrl2::SAR_RSTB_FSM_IDLE_MASK
            } else {
                self.bits & !AmpCtrl2::SAR_RSTB_FSM_IDLE_MASK
            },
        }
    }

    /// Set **XPD_SAR_FSM_IDLE** `[5:5]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_xpd_sar_fsm_idle(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AmpCtrl2::XPD_SAR_FSM_IDLE_MASK
            } else {
                self.bits & !AmpCtrl2::XPD_SAR_FSM_IDLE_MASK
            },
        }
    }

    /// Set **AMP_SHORT_REF_GND_FSM_IDLE** `[4:4]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_amp_short_ref_gnd_fsm_idle(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AmpCtrl2::AMP_SHORT_REF_GND_FSM_IDLE_MASK
            } else {
                self.bits & !AmpCtrl2::AMP_SHORT_REF_GND_FSM_IDLE_MASK
            },
        }
    }

    /// Set **AMP_SHORT_REF_FSM_IDLE** `[3:3]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_amp_short_ref_fsm_idle(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AmpCtrl2::AMP_SHORT_REF_FSM_IDLE_MASK
            } else {
                self.bits & !AmpCtrl2::AMP_SHORT_REF_FSM_IDLE_MASK
            },
        }
    }

    /// Set **AMP_RST_FB_FSM_IDLE** `[2:2]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_amp_rst_fb_fsm_idle(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AmpCtrl2::AMP_RST_FB_FSM_IDLE_MASK
            } else {
                self.bits & !AmpCtrl2::AMP_RST_FB_FSM_IDLE_MASK
            },
        }
    }

    /// Set **XPD_SAR_AMP_FSM_IDLE** `[1:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_xpd_sar_amp_fsm_idle(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AmpCtrl2::XPD_SAR_AMP_FSM_IDLE_MASK
            } else {
                self.bits & !AmpCtrl2::XPD_SAR_AMP_FSM_IDLE_MASK
            },
        }
    }

    /// Set **SAR1_DAC_XPD_FSM_IDLE** `[0:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar1_dac_xpd_fsm_idle(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AmpCtrl2::SAR1_DAC_XPD_FSM_IDLE_MASK
            } else {
                self.bits & !AmpCtrl2::SAR1_DAC_XPD_FSM_IDLE_MASK
            },
        }
    }
}

impl Default for AmpCtrl2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AmpCtrl2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AmpCtrl2Val {
        AmpCtrl2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AmpCtrl2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AmpCtrl2Val) -> AmpCtrl2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AMP_CTRL3 Register
///
/// N/A
///
/// Address offset: 0x0020
/// Reset value: 0x007338f3
///
/// ## Fields
///
/// - **SAR_RSTB_FSM** `[27:24]` (RW)
///
///   N/A
///
/// - **XPD_SAR_FSM** `[23:20]` (RW)
///
///   N/A
///
/// - **AMP_SHORT_REF_GND_FSM** `[19:16]` (RW)
///
///   N/A
///
/// - **AMP_SHORT_REF_FSM** `[15:12]` (RW)
///
///   N/A
///
/// - **AMP_RST_FB_FSM** `[11:8]` (RW)
///
///   N/A
///
/// - **XPD_SAR_AMP_FSM** `[7:4]` (RW)
///
///   N/A
///
/// - **SAR1_DAC_XPD_FSM** `[3:0]` (RW)
///
///   N/A
#[repr(transparent)]
pub struct AmpCtrl3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AmpCtrl3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AmpCtrl3 {
    // ======== Field Constants ========

    /// **SAR_RSTB_FSM**
    ///
    /// N/A
    ///
    /// Bits `[27:24]`, mask `0x0f000000`
    pub const SAR_RSTB_FSM_OFFSET: u32 = 24;
    pub const SAR_RSTB_FSM_MASK: u32 = 0x0f000000;

    /// **XPD_SAR_FSM**
    ///
    /// N/A
    ///
    /// Bits `[23:20]`, mask `0x00f00000`
    pub const XPD_SAR_FSM_OFFSET: u32 = 20;
    pub const XPD_SAR_FSM_MASK: u32 = 0x00f00000;

    /// **AMP_SHORT_REF_GND_FSM**
    ///
    /// N/A
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const AMP_SHORT_REF_GND_FSM_OFFSET: u32 = 16;
    pub const AMP_SHORT_REF_GND_FSM_MASK: u32 = 0x000f0000;

    /// **AMP_SHORT_REF_FSM**
    ///
    /// N/A
    ///
    /// Bits `[15:12]`, mask `0x0000f000`
    pub const AMP_SHORT_REF_FSM_OFFSET: u32 = 12;
    pub const AMP_SHORT_REF_FSM_MASK: u32 = 0x0000f000;

    /// **AMP_RST_FB_FSM**
    ///
    /// N/A
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const AMP_RST_FB_FSM_OFFSET: u32 = 8;
    pub const AMP_RST_FB_FSM_MASK: u32 = 0x00000f00;

    /// **XPD_SAR_AMP_FSM**
    ///
    /// N/A
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const XPD_SAR_AMP_FSM_OFFSET: u32 = 4;
    pub const XPD_SAR_AMP_FSM_MASK: u32 = 0x000000f0;

    /// **SAR1_DAC_XPD_FSM**
    ///
    /// N/A
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const SAR1_DAC_XPD_FSM_OFFSET: u32 = 0;
    pub const SAR1_DAC_XPD_FSM_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **SAR_RSTB_FSM** `[27:24]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar_rstb_fsm(&self) -> u32 {
        (self.value.read_bits() & Self::SAR_RSTB_FSM_MASK) >> Self::SAR_RSTB_FSM_OFFSET
    }

    /// Write **SAR_RSTB_FSM** `[27:24]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_sar_rstb_fsm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR_RSTB_FSM_MASK)
                | ((value << Self::SAR_RSTB_FSM_OFFSET) & Self::SAR_RSTB_FSM_MASK)
        });
    }

    /// **XPD_SAR_FSM** `[23:20]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn xpd_sar_fsm(&self) -> u32 {
        (self.value.read_bits() & Self::XPD_SAR_FSM_MASK) >> Self::XPD_SAR_FSM_OFFSET
    }

    /// Write **XPD_SAR_FSM** `[23:20]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_xpd_sar_fsm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::XPD_SAR_FSM_MASK)
                | ((value << Self::XPD_SAR_FSM_OFFSET) & Self::XPD_SAR_FSM_MASK)
        });
    }

    /// **AMP_SHORT_REF_GND_FSM** `[19:16]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm(&self) -> u32 {
        (self.value.read_bits() & Self::AMP_SHORT_REF_GND_FSM_MASK)
            >> Self::AMP_SHORT_REF_GND_FSM_OFFSET
    }

    /// Write **AMP_SHORT_REF_GND_FSM** `[19:16]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_amp_short_ref_gnd_fsm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AMP_SHORT_REF_GND_FSM_MASK)
                | ((value << Self::AMP_SHORT_REF_GND_FSM_OFFSET) & Self::AMP_SHORT_REF_GND_FSM_MASK)
        });
    }

    /// **AMP_SHORT_REF_FSM** `[15:12]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn amp_short_ref_fsm(&self) -> u32 {
        (self.value.read_bits() & Self::AMP_SHORT_REF_FSM_MASK) >> Self::AMP_SHORT_REF_FSM_OFFSET
    }

    /// Write **AMP_SHORT_REF_FSM** `[15:12]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_amp_short_ref_fsm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AMP_SHORT_REF_FSM_MASK)
                | ((value << Self::AMP_SHORT_REF_FSM_OFFSET) & Self::AMP_SHORT_REF_FSM_MASK)
        });
    }

    /// **AMP_RST_FB_FSM** `[11:8]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn amp_rst_fb_fsm(&self) -> u32 {
        (self.value.read_bits() & Self::AMP_RST_FB_FSM_MASK) >> Self::AMP_RST_FB_FSM_OFFSET
    }

    /// Write **AMP_RST_FB_FSM** `[11:8]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_amp_rst_fb_fsm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AMP_RST_FB_FSM_MASK)
                | ((value << Self::AMP_RST_FB_FSM_OFFSET) & Self::AMP_RST_FB_FSM_MASK)
        });
    }

    /// **XPD_SAR_AMP_FSM** `[7:4]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn xpd_sar_amp_fsm(&self) -> u32 {
        (self.value.read_bits() & Self::XPD_SAR_AMP_FSM_MASK) >> Self::XPD_SAR_AMP_FSM_OFFSET
    }

    /// Write **XPD_SAR_AMP_FSM** `[7:4]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_xpd_sar_amp_fsm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::XPD_SAR_AMP_FSM_MASK)
                | ((value << Self::XPD_SAR_AMP_FSM_OFFSET) & Self::XPD_SAR_AMP_FSM_MASK)
        });
    }

    /// **SAR1_DAC_XPD_FSM** `[3:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_DAC_XPD_FSM_MASK) >> Self::SAR1_DAC_XPD_FSM_OFFSET
    }

    /// Write **SAR1_DAC_XPD_FSM** `[3:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_sar1_dac_xpd_fsm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR1_DAC_XPD_FSM_MASK)
                | ((value << Self::SAR1_DAC_XPD_FSM_OFFSET) & Self::SAR1_DAC_XPD_FSM_MASK)
        });
    }
}

/// AMP_CTRL3 value type for batch read/modify/write operations
///
/// Use `ampctrl3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ampctrl3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AmpCtrl3Val {
    bits: u32,
}

impl AmpCtrl3Val {
    /// Create from raw bits
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
        Self { bits: 0x007338f3 }
    }

    /// Read **SAR_RSTB_FSM** `[27:24]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar_rstb_fsm(&self) -> u32 {
        (self.bits & AmpCtrl3::SAR_RSTB_FSM_MASK) >> AmpCtrl3::SAR_RSTB_FSM_OFFSET
    }

    /// Read **XPD_SAR_FSM** `[23:20]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn xpd_sar_fsm(&self) -> u32 {
        (self.bits & AmpCtrl3::XPD_SAR_FSM_MASK) >> AmpCtrl3::XPD_SAR_FSM_OFFSET
    }

    /// Read **AMP_SHORT_REF_GND_FSM** `[19:16]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn amp_short_ref_gnd_fsm(&self) -> u32 {
        (self.bits & AmpCtrl3::AMP_SHORT_REF_GND_FSM_MASK) >> AmpCtrl3::AMP_SHORT_REF_GND_FSM_OFFSET
    }

    /// Read **AMP_SHORT_REF_FSM** `[15:12]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn amp_short_ref_fsm(&self) -> u32 {
        (self.bits & AmpCtrl3::AMP_SHORT_REF_FSM_MASK) >> AmpCtrl3::AMP_SHORT_REF_FSM_OFFSET
    }

    /// Read **AMP_RST_FB_FSM** `[11:8]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn amp_rst_fb_fsm(&self) -> u32 {
        (self.bits & AmpCtrl3::AMP_RST_FB_FSM_MASK) >> AmpCtrl3::AMP_RST_FB_FSM_OFFSET
    }

    /// Read **XPD_SAR_AMP_FSM** `[7:4]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn xpd_sar_amp_fsm(&self) -> u32 {
        (self.bits & AmpCtrl3::XPD_SAR_AMP_FSM_MASK) >> AmpCtrl3::XPD_SAR_AMP_FSM_OFFSET
    }

    /// Read **SAR1_DAC_XPD_FSM** `[3:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar1_dac_xpd_fsm(&self) -> u32 {
        (self.bits & AmpCtrl3::SAR1_DAC_XPD_FSM_MASK) >> AmpCtrl3::SAR1_DAC_XPD_FSM_OFFSET
    }

    /// Set **SAR_RSTB_FSM** `[27:24]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar_rstb_fsm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AmpCtrl3::SAR_RSTB_FSM_MASK)
                | ((value << AmpCtrl3::SAR_RSTB_FSM_OFFSET) & AmpCtrl3::SAR_RSTB_FSM_MASK),
        }
    }

    /// Set **XPD_SAR_FSM** `[23:20]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_xpd_sar_fsm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AmpCtrl3::XPD_SAR_FSM_MASK)
                | ((value << AmpCtrl3::XPD_SAR_FSM_OFFSET) & AmpCtrl3::XPD_SAR_FSM_MASK),
        }
    }

    /// Set **AMP_SHORT_REF_GND_FSM** `[19:16]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_amp_short_ref_gnd_fsm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AmpCtrl3::AMP_SHORT_REF_GND_FSM_MASK)
                | ((value << AmpCtrl3::AMP_SHORT_REF_GND_FSM_OFFSET)
                    & AmpCtrl3::AMP_SHORT_REF_GND_FSM_MASK),
        }
    }

    /// Set **AMP_SHORT_REF_FSM** `[15:12]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_amp_short_ref_fsm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AmpCtrl3::AMP_SHORT_REF_FSM_MASK)
                | ((value << AmpCtrl3::AMP_SHORT_REF_FSM_OFFSET)
                    & AmpCtrl3::AMP_SHORT_REF_FSM_MASK),
        }
    }

    /// Set **AMP_RST_FB_FSM** `[11:8]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_amp_rst_fb_fsm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AmpCtrl3::AMP_RST_FB_FSM_MASK)
                | ((value << AmpCtrl3::AMP_RST_FB_FSM_OFFSET) & AmpCtrl3::AMP_RST_FB_FSM_MASK),
        }
    }

    /// Set **XPD_SAR_AMP_FSM** `[7:4]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_xpd_sar_amp_fsm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AmpCtrl3::XPD_SAR_AMP_FSM_MASK)
                | ((value << AmpCtrl3::XPD_SAR_AMP_FSM_OFFSET) & AmpCtrl3::XPD_SAR_AMP_FSM_MASK),
        }
    }

    /// Set **SAR1_DAC_XPD_FSM** `[3:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar1_dac_xpd_fsm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AmpCtrl3::SAR1_DAC_XPD_FSM_MASK)
                | ((value << AmpCtrl3::SAR1_DAC_XPD_FSM_OFFSET) & AmpCtrl3::SAR1_DAC_XPD_FSM_MASK),
        }
    }
}

impl Default for AmpCtrl3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AmpCtrl3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AmpCtrl3Val {
        AmpCtrl3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AmpCtrl3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AmpCtrl3Val) -> AmpCtrl3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// READER2_CTRL Register
///
/// Control the read operation of ADC2.
///
/// Address offset: 0x0024
/// Reset value: 0x40050002
///
/// ## Fields
///
/// - **SAR2_INT_EN** `[30]` (RW)
///
///   Enable saradc2 to send out interrupt.
///
/// - **SAR2_DATA_INV** `[29]` (RW)
///
///   Invert SAR ADC2 data.
///
/// - **SAR2_EN_PAD_FORCE_ENABLE** `[28:27]` (RW)
///
///   Force enable adc en_pad to analog circuit 2'b11: force enable .
///
/// - **SAR2_SAMPLE_NUM** `[26:19]` (RW)
///
///   N/A
///
/// - **SAR2_CLK_GATED** `[18]` (RW)
///
///   N/A
///
/// - **SAR2_WAIT_ARB_CYCLE** `[17:16]` (RW)
///
///   Wait arbit stable after sar_done.
///
/// - **SAR2_CLK_DIV** `[7:0]` (RW)
///
///   Clock divider.
#[repr(transparent)]
pub struct Reader2Ctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Reader2Ctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Reader2Ctrl {
    // ======== Field Constants ========

    /// **SAR2_INT_EN**
    ///
    /// Enable saradc2 to send out interrupt.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SAR2_INT_EN_OFFSET: u32 = 30;
    pub const SAR2_INT_EN_MASK: u32 = 0x40000000;
    pub const SAR2_INT_EN: u32 = 1 << 30;

    /// **SAR2_DATA_INV**
    ///
    /// Invert SAR ADC2 data.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SAR2_DATA_INV_OFFSET: u32 = 29;
    pub const SAR2_DATA_INV_MASK: u32 = 0x20000000;
    pub const SAR2_DATA_INV: u32 = 1 << 29;

    /// **SAR2_EN_PAD_FORCE_ENABLE**
    ///
    /// Force enable adc en_pad to analog circuit 2'b11: force enable .
    ///
    /// Bits `[28:27]`, mask `0x18000000`
    pub const SAR2_EN_PAD_FORCE_ENABLE_OFFSET: u32 = 27;
    pub const SAR2_EN_PAD_FORCE_ENABLE_MASK: u32 = 0x18000000;

    /// **SAR2_SAMPLE_NUM**
    ///
    /// N/A
    ///
    /// Bits `[26:19]`, mask `0x07f80000`
    pub const SAR2_SAMPLE_NUM_OFFSET: u32 = 19;
    pub const SAR2_SAMPLE_NUM_MASK: u32 = 0x07f80000;

    /// **SAR2_CLK_GATED**
    ///
    /// N/A
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const SAR2_CLK_GATED_OFFSET: u32 = 18;
    pub const SAR2_CLK_GATED_MASK: u32 = 0x00040000;
    pub const SAR2_CLK_GATED: u32 = 1 << 18;

    /// **SAR2_WAIT_ARB_CYCLE**
    ///
    /// Wait arbit stable after sar_done.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const SAR2_WAIT_ARB_CYCLE_OFFSET: u32 = 16;
    pub const SAR2_WAIT_ARB_CYCLE_MASK: u32 = 0x00030000;

    /// **SAR2_CLK_DIV**
    ///
    /// Clock divider.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SAR2_CLK_DIV_OFFSET: u32 = 0;
    pub const SAR2_CLK_DIV_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SAR2_INT_EN** `[30]` (RW)
    ///
    /// Enable saradc2 to send out interrupt.
    #[inline(always)]
    pub fn sar2_int_en(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_INT_EN_MASK) != 0
    }

    /// Set SAR2_INT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_int_en(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_INT_EN_MASK);
    }

    /// Clear SAR2_INT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_int_en(&self) {
        self.value.modify_bits(|r| r & !Self::SAR2_INT_EN_MASK);
    }

    /// **SAR2_DATA_INV** `[29]` (RW)
    ///
    /// Invert SAR ADC2 data.
    #[inline(always)]
    pub fn sar2_data_inv(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_DATA_INV_MASK) != 0
    }

    /// Set SAR2_DATA_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_data_inv(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_DATA_INV_MASK);
    }

    /// Clear SAR2_DATA_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_data_inv(&self) {
        self.value.modify_bits(|r| r & !Self::SAR2_DATA_INV_MASK);
    }

    /// **SAR2_EN_PAD_FORCE_ENABLE** `[28:27]` (RW)
    ///
    /// Force enable adc en_pad to analog circuit 2'b11: force enable .
    #[inline(always)]
    pub fn sar2_en_pad_force_enable(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_EN_PAD_FORCE_ENABLE_MASK)
            >> Self::SAR2_EN_PAD_FORCE_ENABLE_OFFSET
    }

    /// Write **SAR2_EN_PAD_FORCE_ENABLE** `[28:27]` (RW)
    ///
    /// Force enable adc en_pad to analog circuit 2'b11: force enable .
    #[inline(always)]
    pub fn write_sar2_en_pad_force_enable(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_EN_PAD_FORCE_ENABLE_MASK)
                | ((value << Self::SAR2_EN_PAD_FORCE_ENABLE_OFFSET)
                    & Self::SAR2_EN_PAD_FORCE_ENABLE_MASK)
        });
    }

    /// **SAR2_SAMPLE_NUM** `[26:19]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar2_sample_num(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_SAMPLE_NUM_MASK) >> Self::SAR2_SAMPLE_NUM_OFFSET
    }

    /// Write **SAR2_SAMPLE_NUM** `[26:19]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_sar2_sample_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_SAMPLE_NUM_MASK)
                | ((value << Self::SAR2_SAMPLE_NUM_OFFSET) & Self::SAR2_SAMPLE_NUM_MASK)
        });
    }

    /// **SAR2_CLK_GATED** `[18]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar2_clk_gated(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_CLK_GATED_MASK) != 0
    }

    /// Set SAR2_CLK_GATED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_clk_gated(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_CLK_GATED_MASK);
    }

    /// Clear SAR2_CLK_GATED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_clk_gated(&self) {
        self.value.modify_bits(|r| r & !Self::SAR2_CLK_GATED_MASK);
    }

    /// **SAR2_WAIT_ARB_CYCLE** `[17:16]` (RW)
    ///
    /// Wait arbit stable after sar_done.
    #[inline(always)]
    pub fn sar2_wait_arb_cycle(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_WAIT_ARB_CYCLE_MASK)
            >> Self::SAR2_WAIT_ARB_CYCLE_OFFSET
    }

    /// Write **SAR2_WAIT_ARB_CYCLE** `[17:16]` (RW)
    ///
    /// Wait arbit stable after sar_done.
    #[inline(always)]
    pub fn write_sar2_wait_arb_cycle(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_WAIT_ARB_CYCLE_MASK)
                | ((value << Self::SAR2_WAIT_ARB_CYCLE_OFFSET) & Self::SAR2_WAIT_ARB_CYCLE_MASK)
        });
    }

    /// **SAR2_CLK_DIV** `[7:0]` (RW)
    ///
    /// Clock divider.
    #[inline(always)]
    pub fn sar2_clk_div(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_CLK_DIV_MASK) >> Self::SAR2_CLK_DIV_OFFSET
    }

    /// Write **SAR2_CLK_DIV** `[7:0]` (RW)
    ///
    /// Clock divider.
    #[inline(always)]
    pub fn write_sar2_clk_div(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_CLK_DIV_MASK)
                | ((value << Self::SAR2_CLK_DIV_OFFSET) & Self::SAR2_CLK_DIV_MASK)
        });
    }
}

/// READER2_CTRL value type for batch read/modify/write operations
///
/// Use `reader2ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `reader2ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Reader2CtrlVal {
    bits: u32,
}

impl Reader2CtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x40050002 }
    }

    /// Read **SAR2_INT_EN** `[30:30]`
    ///
    /// Enable saradc2 to send out interrupt.
    #[inline(always)]
    pub const fn sar2_int_en(&self) -> bool {
        (self.bits & Reader2Ctrl::SAR2_INT_EN_MASK) != 0
    }

    /// Read **SAR2_DATA_INV** `[29:29]`
    ///
    /// Invert SAR ADC2 data.
    #[inline(always)]
    pub const fn sar2_data_inv(&self) -> bool {
        (self.bits & Reader2Ctrl::SAR2_DATA_INV_MASK) != 0
    }

    /// Read **SAR2_EN_PAD_FORCE_ENABLE** `[28:27]`
    ///
    /// Force enable adc en_pad to analog circuit 2'b11: force enable .
    #[inline(always)]
    pub const fn sar2_en_pad_force_enable(&self) -> u32 {
        (self.bits & Reader2Ctrl::SAR2_EN_PAD_FORCE_ENABLE_MASK)
            >> Reader2Ctrl::SAR2_EN_PAD_FORCE_ENABLE_OFFSET
    }

    /// Read **SAR2_SAMPLE_NUM** `[26:19]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar2_sample_num(&self) -> u32 {
        (self.bits & Reader2Ctrl::SAR2_SAMPLE_NUM_MASK) >> Reader2Ctrl::SAR2_SAMPLE_NUM_OFFSET
    }

    /// Read **SAR2_CLK_GATED** `[18:18]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar2_clk_gated(&self) -> bool {
        (self.bits & Reader2Ctrl::SAR2_CLK_GATED_MASK) != 0
    }

    /// Read **SAR2_WAIT_ARB_CYCLE** `[17:16]`
    ///
    /// Wait arbit stable after sar_done.
    #[inline(always)]
    pub const fn sar2_wait_arb_cycle(&self) -> u32 {
        (self.bits & Reader2Ctrl::SAR2_WAIT_ARB_CYCLE_MASK)
            >> Reader2Ctrl::SAR2_WAIT_ARB_CYCLE_OFFSET
    }

    /// Read **SAR2_CLK_DIV** `[7:0]`
    ///
    /// Clock divider.
    #[inline(always)]
    pub const fn sar2_clk_div(&self) -> u32 {
        (self.bits & Reader2Ctrl::SAR2_CLK_DIV_MASK) >> Reader2Ctrl::SAR2_CLK_DIV_OFFSET
    }

    /// Set **SAR2_INT_EN** `[30:30]`
    ///
    /// Enable saradc2 to send out interrupt.
    #[inline(always)]
    pub const fn with_sar2_int_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reader2Ctrl::SAR2_INT_EN_MASK
            } else {
                self.bits & !Reader2Ctrl::SAR2_INT_EN_MASK
            },
        }
    }

    /// Set **SAR2_DATA_INV** `[29:29]`
    ///
    /// Invert SAR ADC2 data.
    #[inline(always)]
    pub const fn with_sar2_data_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reader2Ctrl::SAR2_DATA_INV_MASK
            } else {
                self.bits & !Reader2Ctrl::SAR2_DATA_INV_MASK
            },
        }
    }

    /// Set **SAR2_EN_PAD_FORCE_ENABLE** `[28:27]`
    ///
    /// Force enable adc en_pad to analog circuit 2'b11: force enable .
    #[inline(always)]
    pub const fn with_sar2_en_pad_force_enable(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Reader2Ctrl::SAR2_EN_PAD_FORCE_ENABLE_MASK)
                | ((value << Reader2Ctrl::SAR2_EN_PAD_FORCE_ENABLE_OFFSET)
                    & Reader2Ctrl::SAR2_EN_PAD_FORCE_ENABLE_MASK),
        }
    }

    /// Set **SAR2_SAMPLE_NUM** `[26:19]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar2_sample_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Reader2Ctrl::SAR2_SAMPLE_NUM_MASK)
                | ((value << Reader2Ctrl::SAR2_SAMPLE_NUM_OFFSET)
                    & Reader2Ctrl::SAR2_SAMPLE_NUM_MASK),
        }
    }

    /// Set **SAR2_CLK_GATED** `[18:18]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar2_clk_gated(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Reader2Ctrl::SAR2_CLK_GATED_MASK
            } else {
                self.bits & !Reader2Ctrl::SAR2_CLK_GATED_MASK
            },
        }
    }

    /// Set **SAR2_WAIT_ARB_CYCLE** `[17:16]`
    ///
    /// Wait arbit stable after sar_done.
    #[inline(always)]
    pub const fn with_sar2_wait_arb_cycle(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Reader2Ctrl::SAR2_WAIT_ARB_CYCLE_MASK)
                | ((value << Reader2Ctrl::SAR2_WAIT_ARB_CYCLE_OFFSET)
                    & Reader2Ctrl::SAR2_WAIT_ARB_CYCLE_MASK),
        }
    }

    /// Set **SAR2_CLK_DIV** `[7:0]`
    ///
    /// Clock divider.
    #[inline(always)]
    pub const fn with_sar2_clk_div(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Reader2Ctrl::SAR2_CLK_DIV_MASK)
                | ((value << Reader2Ctrl::SAR2_CLK_DIV_OFFSET) & Reader2Ctrl::SAR2_CLK_DIV_MASK),
        }
    }
}

impl Default for Reader2CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Reader2Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Reader2CtrlVal {
        Reader2CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Reader2CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Reader2CtrlVal) -> Reader2CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// READER2_STATUS Register
///
/// N/A
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **SAR2_READER_STATUS** `[31:0]` (RO)
///
///   N/A
#[repr(transparent)]
pub struct Reader2Status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Reader2Status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Reader2Status {
    // ======== Field Constants ========

    /// **SAR2_READER_STATUS**
    ///
    /// N/A
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SAR2_READER_STATUS_OFFSET: u32 = 0;
    pub const SAR2_READER_STATUS_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SAR2_READER_STATUS** `[31:0]` (RO)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar2_reader_status(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_READER_STATUS_MASK) >> Self::SAR2_READER_STATUS_OFFSET
    }
}

/// READER2_STATUS value type for batch read/modify/write operations
///
/// Use `reader2status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `reader2status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Reader2StatusVal {
    bits: u32,
}

impl Reader2StatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR2_READER_STATUS** `[31:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar2_reader_status(&self) -> u32 {
        (self.bits & Reader2Status::SAR2_READER_STATUS_MASK)
            >> Reader2Status::SAR2_READER_STATUS_OFFSET
    }
}

impl Default for Reader2StatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Reader2Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Reader2StatusVal {
        Reader2StatusVal::from_bits(self.value.read_bits())
    }
}

/// MEAS2_CTRL1 Register
///
/// ADC2 configuration registers.
///
/// Address offset: 0x002c
/// Reset value: 0x07020200
///
/// ## Fields
///
/// - **SAR2_XPD_WAIT** `[31:24]` (RW)
///
///   N/A
///
/// - **SAR2_RSTB_WAIT** `[23:16]` (RW)
///
///   N/A
///
/// - **SAR2_STANDBY_WAIT** `[15:8]` (RW)
///
///   N/A
///
/// - **SAR2_RSTB_FORCE** `[7:6]` (RW)
///
///   N/A
///
/// - **SAR2_EN_TEST** `[5]` (RW)
///
///   SAR2_EN_TEST.
///
/// - **SAR2_PKDET_CAL_EN** `[4]` (RW)
///
///   RTC control pkdet enable.
///
/// - **SAR2_PWDET_CAL_EN** `[3]` (RW)
///
///   RTC control pwdet enable.
///
/// - **SAR2_CNTL_STATE** `[2:0]` (RO)
///
///   saradc2_cntl_fsm.
#[repr(transparent)]
pub struct Meas2Ctrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Meas2Ctrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Meas2Ctrl1 {
    // ======== Field Constants ========

    /// **SAR2_XPD_WAIT**
    ///
    /// N/A
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const SAR2_XPD_WAIT_OFFSET: u32 = 24;
    pub const SAR2_XPD_WAIT_MASK: u32 = 0xff000000;

    /// **SAR2_RSTB_WAIT**
    ///
    /// N/A
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const SAR2_RSTB_WAIT_OFFSET: u32 = 16;
    pub const SAR2_RSTB_WAIT_MASK: u32 = 0x00ff0000;

    /// **SAR2_STANDBY_WAIT**
    ///
    /// N/A
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SAR2_STANDBY_WAIT_OFFSET: u32 = 8;
    pub const SAR2_STANDBY_WAIT_MASK: u32 = 0x0000ff00;

    /// **SAR2_RSTB_FORCE**
    ///
    /// N/A
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const SAR2_RSTB_FORCE_OFFSET: u32 = 6;
    pub const SAR2_RSTB_FORCE_MASK: u32 = 0x000000c0;

    /// **SAR2_EN_TEST**
    ///
    /// SAR2_EN_TEST.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const SAR2_EN_TEST_OFFSET: u32 = 5;
    pub const SAR2_EN_TEST_MASK: u32 = 0x00000020;
    pub const SAR2_EN_TEST: u32 = 1 << 5;

    /// **SAR2_PKDET_CAL_EN**
    ///
    /// RTC control pkdet enable.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const SAR2_PKDET_CAL_EN_OFFSET: u32 = 4;
    pub const SAR2_PKDET_CAL_EN_MASK: u32 = 0x00000010;
    pub const SAR2_PKDET_CAL_EN: u32 = 1 << 4;

    /// **SAR2_PWDET_CAL_EN**
    ///
    /// RTC control pwdet enable.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const SAR2_PWDET_CAL_EN_OFFSET: u32 = 3;
    pub const SAR2_PWDET_CAL_EN_MASK: u32 = 0x00000008;
    pub const SAR2_PWDET_CAL_EN: u32 = 1 << 3;

    /// **SAR2_CNTL_STATE**
    ///
    /// saradc2_cntl_fsm.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const SAR2_CNTL_STATE_OFFSET: u32 = 0;
    pub const SAR2_CNTL_STATE_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **SAR2_XPD_WAIT** `[31:24]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar2_xpd_wait(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_XPD_WAIT_MASK) >> Self::SAR2_XPD_WAIT_OFFSET
    }

    /// Write **SAR2_XPD_WAIT** `[31:24]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_sar2_xpd_wait(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_XPD_WAIT_MASK)
                | ((value << Self::SAR2_XPD_WAIT_OFFSET) & Self::SAR2_XPD_WAIT_MASK)
        });
    }

    /// **SAR2_RSTB_WAIT** `[23:16]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar2_rstb_wait(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_RSTB_WAIT_MASK) >> Self::SAR2_RSTB_WAIT_OFFSET
    }

    /// Write **SAR2_RSTB_WAIT** `[23:16]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_sar2_rstb_wait(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_RSTB_WAIT_MASK)
                | ((value << Self::SAR2_RSTB_WAIT_OFFSET) & Self::SAR2_RSTB_WAIT_MASK)
        });
    }

    /// **SAR2_STANDBY_WAIT** `[15:8]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar2_standby_wait(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_STANDBY_WAIT_MASK) >> Self::SAR2_STANDBY_WAIT_OFFSET
    }

    /// Write **SAR2_STANDBY_WAIT** `[15:8]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_sar2_standby_wait(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_STANDBY_WAIT_MASK)
                | ((value << Self::SAR2_STANDBY_WAIT_OFFSET) & Self::SAR2_STANDBY_WAIT_MASK)
        });
    }

    /// **SAR2_RSTB_FORCE** `[7:6]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sar2_rstb_force(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_RSTB_FORCE_MASK) >> Self::SAR2_RSTB_FORCE_OFFSET
    }

    /// Write **SAR2_RSTB_FORCE** `[7:6]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_sar2_rstb_force(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_RSTB_FORCE_MASK)
                | ((value << Self::SAR2_RSTB_FORCE_OFFSET) & Self::SAR2_RSTB_FORCE_MASK)
        });
    }

    /// **SAR2_EN_TEST** `[5]` (RW)
    ///
    /// SAR2_EN_TEST.
    #[inline(always)]
    pub fn sar2_en_test(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_EN_TEST_MASK) != 0
    }

    /// Set SAR2_EN_TEST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_en_test(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_EN_TEST_MASK);
    }

    /// Clear SAR2_EN_TEST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_en_test(&self) {
        self.value.modify_bits(|r| r & !Self::SAR2_EN_TEST_MASK);
    }

    /// **SAR2_PKDET_CAL_EN** `[4]` (RW)
    ///
    /// RTC control pkdet enable.
    #[inline(always)]
    pub fn sar2_pkdet_cal_en(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_PKDET_CAL_EN_MASK) != 0
    }

    /// Set SAR2_PKDET_CAL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_pkdet_cal_en(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_PKDET_CAL_EN_MASK);
    }

    /// Clear SAR2_PKDET_CAL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_pkdet_cal_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SAR2_PKDET_CAL_EN_MASK);
    }

    /// **SAR2_PWDET_CAL_EN** `[3]` (RW)
    ///
    /// RTC control pwdet enable.
    #[inline(always)]
    pub fn sar2_pwdet_cal_en(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_PWDET_CAL_EN_MASK) != 0
    }

    /// Set SAR2_PWDET_CAL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_pwdet_cal_en(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_PWDET_CAL_EN_MASK);
    }

    /// Clear SAR2_PWDET_CAL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_pwdet_cal_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::SAR2_PWDET_CAL_EN_MASK);
    }

    /// **SAR2_CNTL_STATE** `[2:0]` (RO)
    ///
    /// saradc2_cntl_fsm.
    #[inline(always)]
    pub fn sar2_cntl_state(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_CNTL_STATE_MASK) >> Self::SAR2_CNTL_STATE_OFFSET
    }
}

/// MEAS2_CTRL1 value type for batch read/modify/write operations
///
/// Use `meas2ctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `meas2ctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Meas2Ctrl1Val {
    bits: u32,
}

impl Meas2Ctrl1Val {
    /// Create from raw bits
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
        Self { bits: 0x07020200 }
    }

    /// Read **SAR2_XPD_WAIT** `[31:24]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar2_xpd_wait(&self) -> u32 {
        (self.bits & Meas2Ctrl1::SAR2_XPD_WAIT_MASK) >> Meas2Ctrl1::SAR2_XPD_WAIT_OFFSET
    }

    /// Read **SAR2_RSTB_WAIT** `[23:16]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar2_rstb_wait(&self) -> u32 {
        (self.bits & Meas2Ctrl1::SAR2_RSTB_WAIT_MASK) >> Meas2Ctrl1::SAR2_RSTB_WAIT_OFFSET
    }

    /// Read **SAR2_STANDBY_WAIT** `[15:8]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar2_standby_wait(&self) -> u32 {
        (self.bits & Meas2Ctrl1::SAR2_STANDBY_WAIT_MASK) >> Meas2Ctrl1::SAR2_STANDBY_WAIT_OFFSET
    }

    /// Read **SAR2_RSTB_FORCE** `[7:6]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sar2_rstb_force(&self) -> u32 {
        (self.bits & Meas2Ctrl1::SAR2_RSTB_FORCE_MASK) >> Meas2Ctrl1::SAR2_RSTB_FORCE_OFFSET
    }

    /// Read **SAR2_EN_TEST** `[5:5]`
    ///
    /// SAR2_EN_TEST.
    #[inline(always)]
    pub const fn sar2_en_test(&self) -> bool {
        (self.bits & Meas2Ctrl1::SAR2_EN_TEST_MASK) != 0
    }

    /// Read **SAR2_PKDET_CAL_EN** `[4:4]`
    ///
    /// RTC control pkdet enable.
    #[inline(always)]
    pub const fn sar2_pkdet_cal_en(&self) -> bool {
        (self.bits & Meas2Ctrl1::SAR2_PKDET_CAL_EN_MASK) != 0
    }

    /// Read **SAR2_PWDET_CAL_EN** `[3:3]`
    ///
    /// RTC control pwdet enable.
    #[inline(always)]
    pub const fn sar2_pwdet_cal_en(&self) -> bool {
        (self.bits & Meas2Ctrl1::SAR2_PWDET_CAL_EN_MASK) != 0
    }

    /// Read **SAR2_CNTL_STATE** `[2:0]`
    ///
    /// saradc2_cntl_fsm.
    #[inline(always)]
    pub const fn sar2_cntl_state(&self) -> u32 {
        (self.bits & Meas2Ctrl1::SAR2_CNTL_STATE_MASK) >> Meas2Ctrl1::SAR2_CNTL_STATE_OFFSET
    }

    /// Set **SAR2_XPD_WAIT** `[31:24]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar2_xpd_wait(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Meas2Ctrl1::SAR2_XPD_WAIT_MASK)
                | ((value << Meas2Ctrl1::SAR2_XPD_WAIT_OFFSET) & Meas2Ctrl1::SAR2_XPD_WAIT_MASK),
        }
    }

    /// Set **SAR2_RSTB_WAIT** `[23:16]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar2_rstb_wait(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Meas2Ctrl1::SAR2_RSTB_WAIT_MASK)
                | ((value << Meas2Ctrl1::SAR2_RSTB_WAIT_OFFSET) & Meas2Ctrl1::SAR2_RSTB_WAIT_MASK),
        }
    }

    /// Set **SAR2_STANDBY_WAIT** `[15:8]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar2_standby_wait(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Meas2Ctrl1::SAR2_STANDBY_WAIT_MASK)
                | ((value << Meas2Ctrl1::SAR2_STANDBY_WAIT_OFFSET)
                    & Meas2Ctrl1::SAR2_STANDBY_WAIT_MASK),
        }
    }

    /// Set **SAR2_RSTB_FORCE** `[7:6]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sar2_rstb_force(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Meas2Ctrl1::SAR2_RSTB_FORCE_MASK)
                | ((value << Meas2Ctrl1::SAR2_RSTB_FORCE_OFFSET)
                    & Meas2Ctrl1::SAR2_RSTB_FORCE_MASK),
        }
    }

    /// Set **SAR2_EN_TEST** `[5:5]`
    ///
    /// SAR2_EN_TEST.
    #[inline(always)]
    pub const fn with_sar2_en_test(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Meas2Ctrl1::SAR2_EN_TEST_MASK
            } else {
                self.bits & !Meas2Ctrl1::SAR2_EN_TEST_MASK
            },
        }
    }

    /// Set **SAR2_PKDET_CAL_EN** `[4:4]`
    ///
    /// RTC control pkdet enable.
    #[inline(always)]
    pub const fn with_sar2_pkdet_cal_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Meas2Ctrl1::SAR2_PKDET_CAL_EN_MASK
            } else {
                self.bits & !Meas2Ctrl1::SAR2_PKDET_CAL_EN_MASK
            },
        }
    }

    /// Set **SAR2_PWDET_CAL_EN** `[3:3]`
    ///
    /// RTC control pwdet enable.
    #[inline(always)]
    pub const fn with_sar2_pwdet_cal_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Meas2Ctrl1::SAR2_PWDET_CAL_EN_MASK
            } else {
                self.bits & !Meas2Ctrl1::SAR2_PWDET_CAL_EN_MASK
            },
        }
    }
}

impl Default for Meas2Ctrl1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Meas2Ctrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Meas2Ctrl1Val {
        Meas2Ctrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Meas2Ctrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Meas2Ctrl1Val) -> Meas2Ctrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MEAS2_CTRL2 Register
///
/// ADC2 configuration registers.
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **SAR2_EN_PAD_FORCE** `[31]` (RW)
///
///   1: SAR ADC2 pad enable bitmap is controlled by SW.
///
/// - **SAR2_EN_PAD** `[30:19]` (RW)
///
///   SAR ADC2 pad enable bitmap.
///
/// - **MEAS2_START_FORCE** `[18]` (RW)
///
///   1: SAR ADC2 controller (in RTC) is started by SW.
///
/// - **MEAS2_START_SAR** `[17]` (RW)
///
///   SAR ADC2 controller (in RTC) starts conversion.
///
/// - **MEAS2_DONE_SAR** `[16]` (RO)
///
///   SAR ADC2 conversion done indication.
///
/// - **MEAS2_DATA_SAR** `[15:0]` (RO)
///
///   SAR ADC2 data.
#[repr(transparent)]
pub struct Meas2Ctrl2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Meas2Ctrl2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Meas2Ctrl2 {
    // ======== Field Constants ========

    /// **SAR2_EN_PAD_FORCE**
    ///
    /// 1: SAR ADC2 pad enable bitmap is controlled by SW.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SAR2_EN_PAD_FORCE_OFFSET: u32 = 31;
    pub const SAR2_EN_PAD_FORCE_MASK: u32 = 0x80000000;
    pub const SAR2_EN_PAD_FORCE: u32 = 1 << 31;

    /// **SAR2_EN_PAD**
    ///
    /// SAR ADC2 pad enable bitmap.
    ///
    /// Bits `[30:19]`, mask `0x7ff80000`
    pub const SAR2_EN_PAD_OFFSET: u32 = 19;
    pub const SAR2_EN_PAD_MASK: u32 = 0x7ff80000;

    /// **MEAS2_START_FORCE**
    ///
    /// 1: SAR ADC2 controller (in RTC) is started by SW.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const MEAS2_START_FORCE_OFFSET: u32 = 18;
    pub const MEAS2_START_FORCE_MASK: u32 = 0x00040000;
    pub const MEAS2_START_FORCE: u32 = 1 << 18;

    /// **MEAS2_START_SAR**
    ///
    /// SAR ADC2 controller (in RTC) starts conversion.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const MEAS2_START_SAR_OFFSET: u32 = 17;
    pub const MEAS2_START_SAR_MASK: u32 = 0x00020000;
    pub const MEAS2_START_SAR: u32 = 1 << 17;

    /// **MEAS2_DONE_SAR**
    ///
    /// SAR ADC2 conversion done indication.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const MEAS2_DONE_SAR_OFFSET: u32 = 16;
    pub const MEAS2_DONE_SAR_MASK: u32 = 0x00010000;
    pub const MEAS2_DONE_SAR: u32 = 1 << 16;

    /// **MEAS2_DATA_SAR**
    ///
    /// SAR ADC2 data.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MEAS2_DATA_SAR_OFFSET: u32 = 0;
    pub const MEAS2_DATA_SAR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SAR2_EN_PAD_FORCE** `[31]` (RW)
    ///
    /// 1: SAR ADC2 pad enable bitmap is controlled by SW.
    #[inline(always)]
    pub fn sar2_en_pad_force(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_EN_PAD_FORCE_MASK) != 0
    }

    /// Set SAR2_EN_PAD_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_en_pad_force(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_EN_PAD_FORCE_MASK);
    }

    /// Clear SAR2_EN_PAD_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_en_pad_force(&self) {
        self.value
            .modify_bits(|r| r & !Self::SAR2_EN_PAD_FORCE_MASK);
    }

    /// **SAR2_EN_PAD** `[30:19]` (RW)
    ///
    /// SAR ADC2 pad enable bitmap.
    #[inline(always)]
    pub fn sar2_en_pad(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_EN_PAD_MASK) >> Self::SAR2_EN_PAD_OFFSET
    }

    /// Write **SAR2_EN_PAD** `[30:19]` (RW)
    ///
    /// SAR ADC2 pad enable bitmap.
    #[inline(always)]
    pub fn write_sar2_en_pad(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_EN_PAD_MASK)
                | ((value << Self::SAR2_EN_PAD_OFFSET) & Self::SAR2_EN_PAD_MASK)
        });
    }

    /// **MEAS2_START_FORCE** `[18]` (RW)
    ///
    /// 1: SAR ADC2 controller (in RTC) is started by SW.
    #[inline(always)]
    pub fn meas2_start_force(&self) -> bool {
        (self.value.read_bits() & Self::MEAS2_START_FORCE_MASK) != 0
    }

    /// Set MEAS2_START_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_meas2_start_force(&self) {
        self.value.modify_bits(|r| r | Self::MEAS2_START_FORCE_MASK);
    }

    /// Clear MEAS2_START_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_meas2_start_force(&self) {
        self.value
            .modify_bits(|r| r & !Self::MEAS2_START_FORCE_MASK);
    }

    /// **MEAS2_START_SAR** `[17]` (RW)
    ///
    /// SAR ADC2 controller (in RTC) starts conversion.
    #[inline(always)]
    pub fn meas2_start_sar(&self) -> bool {
        (self.value.read_bits() & Self::MEAS2_START_SAR_MASK) != 0
    }

    /// Set MEAS2_START_SAR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_meas2_start_sar(&self) {
        self.value.modify_bits(|r| r | Self::MEAS2_START_SAR_MASK);
    }

    /// Clear MEAS2_START_SAR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_meas2_start_sar(&self) {
        self.value.modify_bits(|r| r & !Self::MEAS2_START_SAR_MASK);
    }

    /// **MEAS2_DONE_SAR** `[16]` (RO)
    ///
    /// SAR ADC2 conversion done indication.
    #[inline(always)]
    pub fn meas2_done_sar(&self) -> bool {
        (self.value.read_bits() & Self::MEAS2_DONE_SAR_MASK) != 0
    }

    /// **MEAS2_DATA_SAR** `[15:0]` (RO)
    ///
    /// SAR ADC2 data.
    #[inline(always)]
    pub fn meas2_data_sar(&self) -> u32 {
        (self.value.read_bits() & Self::MEAS2_DATA_SAR_MASK) >> Self::MEAS2_DATA_SAR_OFFSET
    }
}

/// MEAS2_CTRL2 value type for batch read/modify/write operations
///
/// Use `meas2ctrl2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `meas2ctrl2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Meas2Ctrl2Val {
    bits: u32,
}

impl Meas2Ctrl2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR2_EN_PAD_FORCE** `[31:31]`
    ///
    /// 1: SAR ADC2 pad enable bitmap is controlled by SW.
    #[inline(always)]
    pub const fn sar2_en_pad_force(&self) -> bool {
        (self.bits & Meas2Ctrl2::SAR2_EN_PAD_FORCE_MASK) != 0
    }

    /// Read **SAR2_EN_PAD** `[30:19]`
    ///
    /// SAR ADC2 pad enable bitmap.
    #[inline(always)]
    pub const fn sar2_en_pad(&self) -> u32 {
        (self.bits & Meas2Ctrl2::SAR2_EN_PAD_MASK) >> Meas2Ctrl2::SAR2_EN_PAD_OFFSET
    }

    /// Read **MEAS2_START_FORCE** `[18:18]`
    ///
    /// 1: SAR ADC2 controller (in RTC) is started by SW.
    #[inline(always)]
    pub const fn meas2_start_force(&self) -> bool {
        (self.bits & Meas2Ctrl2::MEAS2_START_FORCE_MASK) != 0
    }

    /// Read **MEAS2_START_SAR** `[17:17]`
    ///
    /// SAR ADC2 controller (in RTC) starts conversion.
    #[inline(always)]
    pub const fn meas2_start_sar(&self) -> bool {
        (self.bits & Meas2Ctrl2::MEAS2_START_SAR_MASK) != 0
    }

    /// Read **MEAS2_DONE_SAR** `[16:16]`
    ///
    /// SAR ADC2 conversion done indication.
    #[inline(always)]
    pub const fn meas2_done_sar(&self) -> bool {
        (self.bits & Meas2Ctrl2::MEAS2_DONE_SAR_MASK) != 0
    }

    /// Read **MEAS2_DATA_SAR** `[15:0]`
    ///
    /// SAR ADC2 data.
    #[inline(always)]
    pub const fn meas2_data_sar(&self) -> u32 {
        (self.bits & Meas2Ctrl2::MEAS2_DATA_SAR_MASK) >> Meas2Ctrl2::MEAS2_DATA_SAR_OFFSET
    }

    /// Set **SAR2_EN_PAD_FORCE** `[31:31]`
    ///
    /// 1: SAR ADC2 pad enable bitmap is controlled by SW.
    #[inline(always)]
    pub const fn with_sar2_en_pad_force(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Meas2Ctrl2::SAR2_EN_PAD_FORCE_MASK
            } else {
                self.bits & !Meas2Ctrl2::SAR2_EN_PAD_FORCE_MASK
            },
        }
    }

    /// Set **SAR2_EN_PAD** `[30:19]`
    ///
    /// SAR ADC2 pad enable bitmap.
    #[inline(always)]
    pub const fn with_sar2_en_pad(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Meas2Ctrl2::SAR2_EN_PAD_MASK)
                | ((value << Meas2Ctrl2::SAR2_EN_PAD_OFFSET) & Meas2Ctrl2::SAR2_EN_PAD_MASK),
        }
    }

    /// Set **MEAS2_START_FORCE** `[18:18]`
    ///
    /// 1: SAR ADC2 controller (in RTC) is started by SW.
    #[inline(always)]
    pub const fn with_meas2_start_force(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Meas2Ctrl2::MEAS2_START_FORCE_MASK
            } else {
                self.bits & !Meas2Ctrl2::MEAS2_START_FORCE_MASK
            },
        }
    }

    /// Set **MEAS2_START_SAR** `[17:17]`
    ///
    /// SAR ADC2 controller (in RTC) starts conversion.
    #[inline(always)]
    pub const fn with_meas2_start_sar(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Meas2Ctrl2::MEAS2_START_SAR_MASK
            } else {
                self.bits & !Meas2Ctrl2::MEAS2_START_SAR_MASK
            },
        }
    }
}

impl Default for Meas2Ctrl2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Meas2Ctrl2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Meas2Ctrl2Val {
        Meas2Ctrl2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Meas2Ctrl2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Meas2Ctrl2Val) -> Meas2Ctrl2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MEAS2_MUX Register
///
/// SAR ADC2 MUX register.
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **SAR2_RTC_FORCE** `[31]` (RW)
///
///   In sleep, force to use rtc to control ADC.
///
/// - **SAR2_PWDET_CCT** `[30:28]` (RW)
///
///   SAR2_PWDET_CCT.
#[repr(transparent)]
pub struct Meas2Mux {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Meas2Mux {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Meas2Mux {
    // ======== Field Constants ========

    /// **SAR2_RTC_FORCE**
    ///
    /// In sleep, force to use rtc to control ADC.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SAR2_RTC_FORCE_OFFSET: u32 = 31;
    pub const SAR2_RTC_FORCE_MASK: u32 = 0x80000000;
    pub const SAR2_RTC_FORCE: u32 = 1 << 31;

    /// **SAR2_PWDET_CCT**
    ///
    /// SAR2_PWDET_CCT.
    ///
    /// Bits `[30:28]`, mask `0x70000000`
    pub const SAR2_PWDET_CCT_OFFSET: u32 = 28;
    pub const SAR2_PWDET_CCT_MASK: u32 = 0x70000000;

    // ======== Field Accessors ========

    /// **SAR2_RTC_FORCE** `[31]` (RW)
    ///
    /// In sleep, force to use rtc to control ADC.
    #[inline(always)]
    pub fn sar2_rtc_force(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_RTC_FORCE_MASK) != 0
    }

    /// Set SAR2_RTC_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_rtc_force(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_RTC_FORCE_MASK);
    }

    /// Clear SAR2_RTC_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_rtc_force(&self) {
        self.value.modify_bits(|r| r & !Self::SAR2_RTC_FORCE_MASK);
    }

    /// **SAR2_PWDET_CCT** `[30:28]` (RW)
    ///
    /// SAR2_PWDET_CCT.
    #[inline(always)]
    pub fn sar2_pwdet_cct(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_PWDET_CCT_MASK) >> Self::SAR2_PWDET_CCT_OFFSET
    }

    /// Write **SAR2_PWDET_CCT** `[30:28]` (RW)
    ///
    /// SAR2_PWDET_CCT.
    #[inline(always)]
    pub fn write_sar2_pwdet_cct(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_PWDET_CCT_MASK)
                | ((value << Self::SAR2_PWDET_CCT_OFFSET) & Self::SAR2_PWDET_CCT_MASK)
        });
    }
}

/// MEAS2_MUX value type for batch read/modify/write operations
///
/// Use `meas2mux.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `meas2mux.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Meas2MuxVal {
    bits: u32,
}

impl Meas2MuxVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR2_RTC_FORCE** `[31:31]`
    ///
    /// In sleep, force to use rtc to control ADC.
    #[inline(always)]
    pub const fn sar2_rtc_force(&self) -> bool {
        (self.bits & Meas2Mux::SAR2_RTC_FORCE_MASK) != 0
    }

    /// Read **SAR2_PWDET_CCT** `[30:28]`
    ///
    /// SAR2_PWDET_CCT.
    #[inline(always)]
    pub const fn sar2_pwdet_cct(&self) -> u32 {
        (self.bits & Meas2Mux::SAR2_PWDET_CCT_MASK) >> Meas2Mux::SAR2_PWDET_CCT_OFFSET
    }

    /// Set **SAR2_RTC_FORCE** `[31:31]`
    ///
    /// In sleep, force to use rtc to control ADC.
    #[inline(always)]
    pub const fn with_sar2_rtc_force(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Meas2Mux::SAR2_RTC_FORCE_MASK
            } else {
                self.bits & !Meas2Mux::SAR2_RTC_FORCE_MASK
            },
        }
    }

    /// Set **SAR2_PWDET_CCT** `[30:28]`
    ///
    /// SAR2_PWDET_CCT.
    #[inline(always)]
    pub const fn with_sar2_pwdet_cct(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Meas2Mux::SAR2_PWDET_CCT_MASK)
                | ((value << Meas2Mux::SAR2_PWDET_CCT_OFFSET) & Meas2Mux::SAR2_PWDET_CCT_MASK),
        }
    }
}

impl Default for Meas2MuxVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Meas2Mux {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Meas2MuxVal {
        Meas2MuxVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Meas2MuxVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Meas2MuxVal) -> Meas2MuxVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ATTEN2 Register
///
/// ADC1 attenuation registers.
///
/// Address offset: 0x0038
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **SAR2_ATTEN** `[31:0]` (RW)
///
///   2-bit attenuation for each pad.
#[repr(transparent)]
pub struct Atten2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Atten2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Atten2 {
    // ======== Field Constants ========

    /// **SAR2_ATTEN**
    ///
    /// 2-bit attenuation for each pad.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SAR2_ATTEN_OFFSET: u32 = 0;
    pub const SAR2_ATTEN_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SAR2_ATTEN** `[31:0]` (RW)
    ///
    /// 2-bit attenuation for each pad.
    #[inline(always)]
    pub fn sar2_atten(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_ATTEN_MASK) >> Self::SAR2_ATTEN_OFFSET
    }

    /// Write **SAR2_ATTEN** `[31:0]` (RW)
    ///
    /// 2-bit attenuation for each pad.
    #[inline(always)]
    pub fn write_sar2_atten(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_ATTEN_MASK)
                | ((value << Self::SAR2_ATTEN_OFFSET) & Self::SAR2_ATTEN_MASK)
        });
    }
}

/// ATTEN2 value type for batch read/modify/write operations
///
/// Use `atten2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `atten2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Atten2Val {
    bits: u32,
}

impl Atten2Val {
    /// Create from raw bits
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

    /// Read **SAR2_ATTEN** `[31:0]`
    ///
    /// 2-bit attenuation for each pad.
    #[inline(always)]
    pub const fn sar2_atten(&self) -> u32 {
        (self.bits & Atten2::SAR2_ATTEN_MASK) >> Atten2::SAR2_ATTEN_OFFSET
    }

    /// Set **SAR2_ATTEN** `[31:0]`
    ///
    /// 2-bit attenuation for each pad.
    #[inline(always)]
    pub const fn with_sar2_atten(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Atten2::SAR2_ATTEN_MASK)
                | ((value << Atten2::SAR2_ATTEN_OFFSET) & Atten2::SAR2_ATTEN_MASK),
        }
    }
}

impl Default for Atten2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Atten2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Atten2Val {
        Atten2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Atten2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Atten2Val) -> Atten2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FORCE_WPD_SAR Register
///
/// In sleep, force to use rtc to control ADC
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **FORCE_XPD_SAR2** `[3:2]` (RW)
///
///   2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control.
///
/// - **FORCE_XPD_SAR1** `[1:0]` (RW)
///
///   2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control.
#[repr(transparent)]
pub struct ForceWpdSar {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ForceWpdSar {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ForceWpdSar {
    // ======== Field Constants ========

    /// **FORCE_XPD_SAR2**
    ///
    /// 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control.
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const FORCE_XPD_SAR2_OFFSET: u32 = 2;
    pub const FORCE_XPD_SAR2_MASK: u32 = 0x0000000c;

    /// **FORCE_XPD_SAR1**
    ///
    /// 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const FORCE_XPD_SAR1_OFFSET: u32 = 0;
    pub const FORCE_XPD_SAR1_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **FORCE_XPD_SAR2** `[3:2]` (RW)
    ///
    /// 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control.
    #[inline(always)]
    pub fn force_xpd_sar2(&self) -> u32 {
        (self.value.read_bits() & Self::FORCE_XPD_SAR2_MASK) >> Self::FORCE_XPD_SAR2_OFFSET
    }

    /// Write **FORCE_XPD_SAR2** `[3:2]` (RW)
    ///
    /// 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control.
    #[inline(always)]
    pub fn write_force_xpd_sar2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FORCE_XPD_SAR2_MASK)
                | ((value << Self::FORCE_XPD_SAR2_OFFSET) & Self::FORCE_XPD_SAR2_MASK)
        });
    }

    /// **FORCE_XPD_SAR1** `[1:0]` (RW)
    ///
    /// 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control.
    #[inline(always)]
    pub fn force_xpd_sar1(&self) -> u32 {
        (self.value.read_bits() & Self::FORCE_XPD_SAR1_MASK) >> Self::FORCE_XPD_SAR1_OFFSET
    }

    /// Write **FORCE_XPD_SAR1** `[1:0]` (RW)
    ///
    /// 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control.
    #[inline(always)]
    pub fn write_force_xpd_sar1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FORCE_XPD_SAR1_MASK)
                | ((value << Self::FORCE_XPD_SAR1_OFFSET) & Self::FORCE_XPD_SAR1_MASK)
        });
    }
}

/// FORCE_WPD_SAR value type for batch read/modify/write operations
///
/// Use `forcewpdsar.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `forcewpdsar.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ForceWpdSarVal {
    bits: u32,
}

impl ForceWpdSarVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FORCE_XPD_SAR2** `[3:2]`
    ///
    /// 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control.
    #[inline(always)]
    pub const fn force_xpd_sar2(&self) -> u32 {
        (self.bits & ForceWpdSar::FORCE_XPD_SAR2_MASK) >> ForceWpdSar::FORCE_XPD_SAR2_OFFSET
    }

    /// Read **FORCE_XPD_SAR1** `[1:0]`
    ///
    /// 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control.
    #[inline(always)]
    pub const fn force_xpd_sar1(&self) -> u32 {
        (self.bits & ForceWpdSar::FORCE_XPD_SAR1_MASK) >> ForceWpdSar::FORCE_XPD_SAR1_OFFSET
    }

    /// Set **FORCE_XPD_SAR2** `[3:2]`
    ///
    /// 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control.
    #[inline(always)]
    pub const fn with_force_xpd_sar2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ForceWpdSar::FORCE_XPD_SAR2_MASK)
                | ((value << ForceWpdSar::FORCE_XPD_SAR2_OFFSET)
                    & ForceWpdSar::FORCE_XPD_SAR2_MASK),
        }
    }

    /// Set **FORCE_XPD_SAR1** `[1:0]`
    ///
    /// 2'b11:software control, force on. 2'b10:software control, force off. 2'b0x:hardware control.
    #[inline(always)]
    pub const fn with_force_xpd_sar1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ForceWpdSar::FORCE_XPD_SAR1_MASK)
                | ((value << ForceWpdSar::FORCE_XPD_SAR1_OFFSET)
                    & ForceWpdSar::FORCE_XPD_SAR1_MASK),
        }
    }
}

impl Default for ForceWpdSarVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ForceWpdSar {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ForceWpdSarVal {
        ForceWpdSarVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ForceWpdSarVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ForceWpdSarVal) -> ForceWpdSarVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MEAS_STATUS Register
///
/// N/A
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **SARADC_MEAS_STATUS** `[7:0]` (RO)
///
///   N/A
#[repr(transparent)]
pub struct MeasStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MeasStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MeasStatus {
    // ======== Field Constants ========

    /// **SARADC_MEAS_STATUS**
    ///
    /// N/A
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SARADC_MEAS_STATUS_OFFSET: u32 = 0;
    pub const SARADC_MEAS_STATUS_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SARADC_MEAS_STATUS** `[7:0]` (RO)
    ///
    /// N/A
    #[inline(always)]
    pub fn saradc_meas_status(&self) -> u32 {
        (self.value.read_bits() & Self::SARADC_MEAS_STATUS_MASK) >> Self::SARADC_MEAS_STATUS_OFFSET
    }
}

/// MEAS_STATUS value type for batch read/modify/write operations
///
/// Use `measstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `measstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MeasStatusVal {
    bits: u32,
}

impl MeasStatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SARADC_MEAS_STATUS** `[7:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn saradc_meas_status(&self) -> u32 {
        (self.bits & MeasStatus::SARADC_MEAS_STATUS_MASK) >> MeasStatus::SARADC_MEAS_STATUS_OFFSET
    }
}

impl Default for MeasStatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MeasStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MeasStatusVal {
        MeasStatusVal::from_bits(self.value.read_bits())
    }
}

/// REG_CLKEN Register
///
/// N/A
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **CLK_EN** `[0]` (RW)
///
///   N/A
#[repr(transparent)]
pub struct RegClken {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RegClken {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RegClken {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// N/A
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_EN_OFFSET: u32 = 0;
    pub const CLK_EN_MASK: u32 = 0x00000001;
    pub const CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[0]` (RW)
    ///
    /// N/A
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

/// REG_CLKEN value type for batch read/modify/write operations
///
/// Use `regclken.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regclken.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegClkenVal {
    bits: u32,
}

impl RegClkenVal {
    /// Create from raw bits
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
    /// N/A
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & RegClken::CLK_EN_MASK) != 0
    }

    /// Set **CLK_EN** `[0:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RegClken::CLK_EN_MASK
            } else {
                self.bits & !RegClken::CLK_EN_MASK
            },
        }
    }
}

impl Default for RegClkenVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RegClken {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegClkenVal {
        RegClkenVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegClkenVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegClkenVal) -> RegClkenVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// COCPU_INT_RAW Register
///
/// Interrupt raw registers.
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **COCPU_SARADC2_WAKE_INT_RAW** `[5]` (RW)
///
///   A wakeup event is triggered from ADC2, int raw.
///
/// - **COCPU_SARADC1_WAKE_INT_RAW** `[4]` (RW)
///
///   A wakeup event is triggered from ADC1, int raw.
///
/// - **COCPU_SARADC2_ERROR_INT_RAW** `[3]` (RW)
///
///   An errro occurs from ADC2, int raw.
///
/// - **COCPU_SARADC1_ERROR_INT_RAW** `[2]` (RW)
///
///   An errro occurs from ADC1, int raw.
///
/// - **COCPU_SARADC2_INT_RAW** `[1]` (RW)
///
///   ADC2 Conversion is done, int raw.
///
/// - **COCPU_SARADC1_INT_RAW** `[0]` (RW)
///
///   ADC1 Conversion is done, int raw.
#[repr(transparent)]
pub struct CocpuIntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CocpuIntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CocpuIntRaw {
    // ======== Field Constants ========

    /// **COCPU_SARADC2_WAKE_INT_RAW**
    ///
    /// A wakeup event is triggered from ADC2, int raw.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const COCPU_SARADC2_WAKE_INT_RAW_OFFSET: u32 = 5;
    pub const COCPU_SARADC2_WAKE_INT_RAW_MASK: u32 = 0x00000020;
    pub const COCPU_SARADC2_WAKE_INT_RAW: u32 = 1 << 5;

    /// **COCPU_SARADC1_WAKE_INT_RAW**
    ///
    /// A wakeup event is triggered from ADC1, int raw.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const COCPU_SARADC1_WAKE_INT_RAW_OFFSET: u32 = 4;
    pub const COCPU_SARADC1_WAKE_INT_RAW_MASK: u32 = 0x00000010;
    pub const COCPU_SARADC1_WAKE_INT_RAW: u32 = 1 << 4;

    /// **COCPU_SARADC2_ERROR_INT_RAW**
    ///
    /// An errro occurs from ADC2, int raw.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const COCPU_SARADC2_ERROR_INT_RAW_OFFSET: u32 = 3;
    pub const COCPU_SARADC2_ERROR_INT_RAW_MASK: u32 = 0x00000008;
    pub const COCPU_SARADC2_ERROR_INT_RAW: u32 = 1 << 3;

    /// **COCPU_SARADC1_ERROR_INT_RAW**
    ///
    /// An errro occurs from ADC1, int raw.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const COCPU_SARADC1_ERROR_INT_RAW_OFFSET: u32 = 2;
    pub const COCPU_SARADC1_ERROR_INT_RAW_MASK: u32 = 0x00000004;
    pub const COCPU_SARADC1_ERROR_INT_RAW: u32 = 1 << 2;

    /// **COCPU_SARADC2_INT_RAW**
    ///
    /// ADC2 Conversion is done, int raw.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const COCPU_SARADC2_INT_RAW_OFFSET: u32 = 1;
    pub const COCPU_SARADC2_INT_RAW_MASK: u32 = 0x00000002;
    pub const COCPU_SARADC2_INT_RAW: u32 = 1 << 1;

    /// **COCPU_SARADC1_INT_RAW**
    ///
    /// ADC1 Conversion is done, int raw.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const COCPU_SARADC1_INT_RAW_OFFSET: u32 = 0;
    pub const COCPU_SARADC1_INT_RAW_MASK: u32 = 0x00000001;
    pub const COCPU_SARADC1_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **COCPU_SARADC2_WAKE_INT_RAW** `[5]` (RW)
    ///
    /// A wakeup event is triggered from ADC2, int raw.
    #[inline(always)]
    pub fn cocpu_saradc2_wake_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC2_WAKE_INT_RAW_MASK) != 0
    }

    /// Set COCPU_SARADC2_WAKE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cocpu_saradc2_wake_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::COCPU_SARADC2_WAKE_INT_RAW_MASK);
    }

    /// Clear COCPU_SARADC2_WAKE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cocpu_saradc2_wake_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::COCPU_SARADC2_WAKE_INT_RAW_MASK);
    }

    /// **COCPU_SARADC1_WAKE_INT_RAW** `[4]` (RW)
    ///
    /// A wakeup event is triggered from ADC1, int raw.
    #[inline(always)]
    pub fn cocpu_saradc1_wake_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC1_WAKE_INT_RAW_MASK) != 0
    }

    /// Set COCPU_SARADC1_WAKE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cocpu_saradc1_wake_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::COCPU_SARADC1_WAKE_INT_RAW_MASK);
    }

    /// Clear COCPU_SARADC1_WAKE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cocpu_saradc1_wake_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::COCPU_SARADC1_WAKE_INT_RAW_MASK);
    }

    /// **COCPU_SARADC2_ERROR_INT_RAW** `[3]` (RW)
    ///
    /// An errro occurs from ADC2, int raw.
    #[inline(always)]
    pub fn cocpu_saradc2_error_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC2_ERROR_INT_RAW_MASK) != 0
    }

    /// Set COCPU_SARADC2_ERROR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cocpu_saradc2_error_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::COCPU_SARADC2_ERROR_INT_RAW_MASK);
    }

    /// Clear COCPU_SARADC2_ERROR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cocpu_saradc2_error_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::COCPU_SARADC2_ERROR_INT_RAW_MASK);
    }

    /// **COCPU_SARADC1_ERROR_INT_RAW** `[2]` (RW)
    ///
    /// An errro occurs from ADC1, int raw.
    #[inline(always)]
    pub fn cocpu_saradc1_error_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC1_ERROR_INT_RAW_MASK) != 0
    }

    /// Set COCPU_SARADC1_ERROR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cocpu_saradc1_error_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::COCPU_SARADC1_ERROR_INT_RAW_MASK);
    }

    /// Clear COCPU_SARADC1_ERROR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cocpu_saradc1_error_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::COCPU_SARADC1_ERROR_INT_RAW_MASK);
    }

    /// **COCPU_SARADC2_INT_RAW** `[1]` (RW)
    ///
    /// ADC2 Conversion is done, int raw.
    #[inline(always)]
    pub fn cocpu_saradc2_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC2_INT_RAW_MASK) != 0
    }

    /// Set COCPU_SARADC2_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cocpu_saradc2_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::COCPU_SARADC2_INT_RAW_MASK);
    }

    /// Clear COCPU_SARADC2_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cocpu_saradc2_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::COCPU_SARADC2_INT_RAW_MASK);
    }

    /// **COCPU_SARADC1_INT_RAW** `[0]` (RW)
    ///
    /// ADC1 Conversion is done, int raw.
    #[inline(always)]
    pub fn cocpu_saradc1_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC1_INT_RAW_MASK) != 0
    }

    /// Set COCPU_SARADC1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cocpu_saradc1_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::COCPU_SARADC1_INT_RAW_MASK);
    }

    /// Clear COCPU_SARADC1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cocpu_saradc1_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::COCPU_SARADC1_INT_RAW_MASK);
    }
}

/// COCPU_INT_RAW value type for batch read/modify/write operations
///
/// Use `cocpuintraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cocpuintraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CocpuIntRawVal {
    bits: u32,
}

impl CocpuIntRawVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **COCPU_SARADC2_WAKE_INT_RAW** `[5:5]`
    ///
    /// A wakeup event is triggered from ADC2, int raw.
    #[inline(always)]
    pub const fn cocpu_saradc2_wake_int_raw(&self) -> bool {
        (self.bits & CocpuIntRaw::COCPU_SARADC2_WAKE_INT_RAW_MASK) != 0
    }

    /// Read **COCPU_SARADC1_WAKE_INT_RAW** `[4:4]`
    ///
    /// A wakeup event is triggered from ADC1, int raw.
    #[inline(always)]
    pub const fn cocpu_saradc1_wake_int_raw(&self) -> bool {
        (self.bits & CocpuIntRaw::COCPU_SARADC1_WAKE_INT_RAW_MASK) != 0
    }

    /// Read **COCPU_SARADC2_ERROR_INT_RAW** `[3:3]`
    ///
    /// An errro occurs from ADC2, int raw.
    #[inline(always)]
    pub const fn cocpu_saradc2_error_int_raw(&self) -> bool {
        (self.bits & CocpuIntRaw::COCPU_SARADC2_ERROR_INT_RAW_MASK) != 0
    }

    /// Read **COCPU_SARADC1_ERROR_INT_RAW** `[2:2]`
    ///
    /// An errro occurs from ADC1, int raw.
    #[inline(always)]
    pub const fn cocpu_saradc1_error_int_raw(&self) -> bool {
        (self.bits & CocpuIntRaw::COCPU_SARADC1_ERROR_INT_RAW_MASK) != 0
    }

    /// Read **COCPU_SARADC2_INT_RAW** `[1:1]`
    ///
    /// ADC2 Conversion is done, int raw.
    #[inline(always)]
    pub const fn cocpu_saradc2_int_raw(&self) -> bool {
        (self.bits & CocpuIntRaw::COCPU_SARADC2_INT_RAW_MASK) != 0
    }

    /// Read **COCPU_SARADC1_INT_RAW** `[0:0]`
    ///
    /// ADC1 Conversion is done, int raw.
    #[inline(always)]
    pub const fn cocpu_saradc1_int_raw(&self) -> bool {
        (self.bits & CocpuIntRaw::COCPU_SARADC1_INT_RAW_MASK) != 0
    }

    /// Set **COCPU_SARADC2_WAKE_INT_RAW** `[5:5]`
    ///
    /// A wakeup event is triggered from ADC2, int raw.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_wake_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CocpuIntRaw::COCPU_SARADC2_WAKE_INT_RAW_MASK
            } else {
                self.bits & !CocpuIntRaw::COCPU_SARADC2_WAKE_INT_RAW_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_WAKE_INT_RAW** `[4:4]`
    ///
    /// A wakeup event is triggered from ADC1, int raw.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_wake_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CocpuIntRaw::COCPU_SARADC1_WAKE_INT_RAW_MASK
            } else {
                self.bits & !CocpuIntRaw::COCPU_SARADC1_WAKE_INT_RAW_MASK
            },
        }
    }

    /// Set **COCPU_SARADC2_ERROR_INT_RAW** `[3:3]`
    ///
    /// An errro occurs from ADC2, int raw.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_error_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CocpuIntRaw::COCPU_SARADC2_ERROR_INT_RAW_MASK
            } else {
                self.bits & !CocpuIntRaw::COCPU_SARADC2_ERROR_INT_RAW_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_ERROR_INT_RAW** `[2:2]`
    ///
    /// An errro occurs from ADC1, int raw.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_error_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CocpuIntRaw::COCPU_SARADC1_ERROR_INT_RAW_MASK
            } else {
                self.bits & !CocpuIntRaw::COCPU_SARADC1_ERROR_INT_RAW_MASK
            },
        }
    }

    /// Set **COCPU_SARADC2_INT_RAW** `[1:1]`
    ///
    /// ADC2 Conversion is done, int raw.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CocpuIntRaw::COCPU_SARADC2_INT_RAW_MASK
            } else {
                self.bits & !CocpuIntRaw::COCPU_SARADC2_INT_RAW_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_INT_RAW** `[0:0]`
    ///
    /// ADC1 Conversion is done, int raw.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CocpuIntRaw::COCPU_SARADC1_INT_RAW_MASK
            } else {
                self.bits & !CocpuIntRaw::COCPU_SARADC1_INT_RAW_MASK
            },
        }
    }
}

impl Default for CocpuIntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CocpuIntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CocpuIntRawVal {
        CocpuIntRawVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CocpuIntRawVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CocpuIntRawVal) -> CocpuIntRawVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ENA Register
///
/// Interrupt enable registers.
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **COCPU_SARADC2_WAKE_INT_ENA** `[5]` (RW)
///
///   A wakeup event is triggered from ADC2, int enable.
///
/// - **COCPU_SARADC1_WAKE_INT_ENA** `[4]` (RW)
///
///   A wakeup event is triggered from ADC1, int enable.
///
/// - **COCPU_SARADC2_ERROR_INT_ENA** `[3]` (RW)
///
///   An errro occurs from ADC2, int enable.
///
/// - **COCPU_SARADC1_ERROR_INT_ENA** `[2]` (RW)
///
///   An errro occurs from ADC1, int enable.
///
/// - **COCPU_SARADC2_INT_ENA** `[1]` (RW)
///
///   ADC2 Conversion is done, int enable.
///
/// - **COCPU_SARADC1_INT_ENA** `[0]` (RW)
///
///   ADC1 Conversion is done, int enable.
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

    /// **COCPU_SARADC2_WAKE_INT_ENA**
    ///
    /// A wakeup event is triggered from ADC2, int enable.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const COCPU_SARADC2_WAKE_INT_ENA_OFFSET: u32 = 5;
    pub const COCPU_SARADC2_WAKE_INT_ENA_MASK: u32 = 0x00000020;
    pub const COCPU_SARADC2_WAKE_INT_ENA: u32 = 1 << 5;

    /// **COCPU_SARADC1_WAKE_INT_ENA**
    ///
    /// A wakeup event is triggered from ADC1, int enable.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const COCPU_SARADC1_WAKE_INT_ENA_OFFSET: u32 = 4;
    pub const COCPU_SARADC1_WAKE_INT_ENA_MASK: u32 = 0x00000010;
    pub const COCPU_SARADC1_WAKE_INT_ENA: u32 = 1 << 4;

    /// **COCPU_SARADC2_ERROR_INT_ENA**
    ///
    /// An errro occurs from ADC2, int enable.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const COCPU_SARADC2_ERROR_INT_ENA_OFFSET: u32 = 3;
    pub const COCPU_SARADC2_ERROR_INT_ENA_MASK: u32 = 0x00000008;
    pub const COCPU_SARADC2_ERROR_INT_ENA: u32 = 1 << 3;

    /// **COCPU_SARADC1_ERROR_INT_ENA**
    ///
    /// An errro occurs from ADC1, int enable.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const COCPU_SARADC1_ERROR_INT_ENA_OFFSET: u32 = 2;
    pub const COCPU_SARADC1_ERROR_INT_ENA_MASK: u32 = 0x00000004;
    pub const COCPU_SARADC1_ERROR_INT_ENA: u32 = 1 << 2;

    /// **COCPU_SARADC2_INT_ENA**
    ///
    /// ADC2 Conversion is done, int enable.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const COCPU_SARADC2_INT_ENA_OFFSET: u32 = 1;
    pub const COCPU_SARADC2_INT_ENA_MASK: u32 = 0x00000002;
    pub const COCPU_SARADC2_INT_ENA: u32 = 1 << 1;

    /// **COCPU_SARADC1_INT_ENA**
    ///
    /// ADC1 Conversion is done, int enable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const COCPU_SARADC1_INT_ENA_OFFSET: u32 = 0;
    pub const COCPU_SARADC1_INT_ENA_MASK: u32 = 0x00000001;
    pub const COCPU_SARADC1_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **COCPU_SARADC2_WAKE_INT_ENA** `[5]` (RW)
    ///
    /// A wakeup event is triggered from ADC2, int enable.
    #[inline(always)]
    pub fn cocpu_saradc2_wake_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC2_WAKE_INT_ENA_MASK) != 0
    }

    /// Set COCPU_SARADC2_WAKE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cocpu_saradc2_wake_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::COCPU_SARADC2_WAKE_INT_ENA_MASK);
    }

    /// Clear COCPU_SARADC2_WAKE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cocpu_saradc2_wake_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::COCPU_SARADC2_WAKE_INT_ENA_MASK);
    }

    /// **COCPU_SARADC1_WAKE_INT_ENA** `[4]` (RW)
    ///
    /// A wakeup event is triggered from ADC1, int enable.
    #[inline(always)]
    pub fn cocpu_saradc1_wake_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC1_WAKE_INT_ENA_MASK) != 0
    }

    /// Set COCPU_SARADC1_WAKE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cocpu_saradc1_wake_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::COCPU_SARADC1_WAKE_INT_ENA_MASK);
    }

    /// Clear COCPU_SARADC1_WAKE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cocpu_saradc1_wake_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::COCPU_SARADC1_WAKE_INT_ENA_MASK);
    }

    /// **COCPU_SARADC2_ERROR_INT_ENA** `[3]` (RW)
    ///
    /// An errro occurs from ADC2, int enable.
    #[inline(always)]
    pub fn cocpu_saradc2_error_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC2_ERROR_INT_ENA_MASK) != 0
    }

    /// Set COCPU_SARADC2_ERROR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cocpu_saradc2_error_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::COCPU_SARADC2_ERROR_INT_ENA_MASK);
    }

    /// Clear COCPU_SARADC2_ERROR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cocpu_saradc2_error_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::COCPU_SARADC2_ERROR_INT_ENA_MASK);
    }

    /// **COCPU_SARADC1_ERROR_INT_ENA** `[2]` (RW)
    ///
    /// An errro occurs from ADC1, int enable.
    #[inline(always)]
    pub fn cocpu_saradc1_error_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC1_ERROR_INT_ENA_MASK) != 0
    }

    /// Set COCPU_SARADC1_ERROR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cocpu_saradc1_error_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::COCPU_SARADC1_ERROR_INT_ENA_MASK);
    }

    /// Clear COCPU_SARADC1_ERROR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cocpu_saradc1_error_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::COCPU_SARADC1_ERROR_INT_ENA_MASK);
    }

    /// **COCPU_SARADC2_INT_ENA** `[1]` (RW)
    ///
    /// ADC2 Conversion is done, int enable.
    #[inline(always)]
    pub fn cocpu_saradc2_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC2_INT_ENA_MASK) != 0
    }

    /// Set COCPU_SARADC2_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cocpu_saradc2_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::COCPU_SARADC2_INT_ENA_MASK);
    }

    /// Clear COCPU_SARADC2_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cocpu_saradc2_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::COCPU_SARADC2_INT_ENA_MASK);
    }

    /// **COCPU_SARADC1_INT_ENA** `[0]` (RW)
    ///
    /// ADC1 Conversion is done, int enable.
    #[inline(always)]
    pub fn cocpu_saradc1_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC1_INT_ENA_MASK) != 0
    }

    /// Set COCPU_SARADC1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cocpu_saradc1_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::COCPU_SARADC1_INT_ENA_MASK);
    }

    /// Clear COCPU_SARADC1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cocpu_saradc1_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::COCPU_SARADC1_INT_ENA_MASK);
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

    /// Read **COCPU_SARADC2_WAKE_INT_ENA** `[5:5]`
    ///
    /// A wakeup event is triggered from ADC2, int enable.
    #[inline(always)]
    pub const fn cocpu_saradc2_wake_int_ena(&self) -> bool {
        (self.bits & IntEna::COCPU_SARADC2_WAKE_INT_ENA_MASK) != 0
    }

    /// Read **COCPU_SARADC1_WAKE_INT_ENA** `[4:4]`
    ///
    /// A wakeup event is triggered from ADC1, int enable.
    #[inline(always)]
    pub const fn cocpu_saradc1_wake_int_ena(&self) -> bool {
        (self.bits & IntEna::COCPU_SARADC1_WAKE_INT_ENA_MASK) != 0
    }

    /// Read **COCPU_SARADC2_ERROR_INT_ENA** `[3:3]`
    ///
    /// An errro occurs from ADC2, int enable.
    #[inline(always)]
    pub const fn cocpu_saradc2_error_int_ena(&self) -> bool {
        (self.bits & IntEna::COCPU_SARADC2_ERROR_INT_ENA_MASK) != 0
    }

    /// Read **COCPU_SARADC1_ERROR_INT_ENA** `[2:2]`
    ///
    /// An errro occurs from ADC1, int enable.
    #[inline(always)]
    pub const fn cocpu_saradc1_error_int_ena(&self) -> bool {
        (self.bits & IntEna::COCPU_SARADC1_ERROR_INT_ENA_MASK) != 0
    }

    /// Read **COCPU_SARADC2_INT_ENA** `[1:1]`
    ///
    /// ADC2 Conversion is done, int enable.
    #[inline(always)]
    pub const fn cocpu_saradc2_int_ena(&self) -> bool {
        (self.bits & IntEna::COCPU_SARADC2_INT_ENA_MASK) != 0
    }

    /// Read **COCPU_SARADC1_INT_ENA** `[0:0]`
    ///
    /// ADC1 Conversion is done, int enable.
    #[inline(always)]
    pub const fn cocpu_saradc1_int_ena(&self) -> bool {
        (self.bits & IntEna::COCPU_SARADC1_INT_ENA_MASK) != 0
    }

    /// Set **COCPU_SARADC2_WAKE_INT_ENA** `[5:5]`
    ///
    /// A wakeup event is triggered from ADC2, int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_wake_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::COCPU_SARADC2_WAKE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::COCPU_SARADC2_WAKE_INT_ENA_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_WAKE_INT_ENA** `[4:4]`
    ///
    /// A wakeup event is triggered from ADC1, int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_wake_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::COCPU_SARADC1_WAKE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::COCPU_SARADC1_WAKE_INT_ENA_MASK
            },
        }
    }

    /// Set **COCPU_SARADC2_ERROR_INT_ENA** `[3:3]`
    ///
    /// An errro occurs from ADC2, int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_error_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::COCPU_SARADC2_ERROR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::COCPU_SARADC2_ERROR_INT_ENA_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_ERROR_INT_ENA** `[2:2]`
    ///
    /// An errro occurs from ADC1, int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_error_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::COCPU_SARADC1_ERROR_INT_ENA_MASK
            } else {
                self.bits & !IntEna::COCPU_SARADC1_ERROR_INT_ENA_MASK
            },
        }
    }

    /// Set **COCPU_SARADC2_INT_ENA** `[1:1]`
    ///
    /// ADC2 Conversion is done, int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::COCPU_SARADC2_INT_ENA_MASK
            } else {
                self.bits & !IntEna::COCPU_SARADC2_INT_ENA_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_INT_ENA** `[0:0]`
    ///
    /// ADC1 Conversion is done, int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::COCPU_SARADC1_INT_ENA_MASK
            } else {
                self.bits & !IntEna::COCPU_SARADC1_INT_ENA_MASK
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
/// Interrupt status registers.
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **COCPU_SARADC2_WAKE_INT_ST** `[5]` (RO)
///
///   A wakeup event is triggered from ADC2, int status.
///
/// - **COCPU_SARADC1_WAKE_INT_ST** `[4]` (RO)
///
///   A wakeup event is triggered from ADC1, int status.
///
/// - **COCPU_SARADC2_ERROR_INT_ST** `[3]` (RO)
///
///   An errro occurs from ADC2, int status.
///
/// - **COCPU_SARADC1_ERROR_INT_ST** `[2]` (RO)
///
///   An errro occurs from ADC1, int status.
///
/// - **COCPU_SARADC2_INT_ST** `[1]` (RO)
///
///   ADC2 Conversion is done, int status.
///
/// - **COCPU_SARADC1_INT_ST** `[0]` (RO)
///
///   ADC1 Conversion is done, int status.
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

    /// **COCPU_SARADC2_WAKE_INT_ST**
    ///
    /// A wakeup event is triggered from ADC2, int status.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const COCPU_SARADC2_WAKE_INT_ST_OFFSET: u32 = 5;
    pub const COCPU_SARADC2_WAKE_INT_ST_MASK: u32 = 0x00000020;
    pub const COCPU_SARADC2_WAKE_INT_ST: u32 = 1 << 5;

    /// **COCPU_SARADC1_WAKE_INT_ST**
    ///
    /// A wakeup event is triggered from ADC1, int status.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const COCPU_SARADC1_WAKE_INT_ST_OFFSET: u32 = 4;
    pub const COCPU_SARADC1_WAKE_INT_ST_MASK: u32 = 0x00000010;
    pub const COCPU_SARADC1_WAKE_INT_ST: u32 = 1 << 4;

    /// **COCPU_SARADC2_ERROR_INT_ST**
    ///
    /// An errro occurs from ADC2, int status.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const COCPU_SARADC2_ERROR_INT_ST_OFFSET: u32 = 3;
    pub const COCPU_SARADC2_ERROR_INT_ST_MASK: u32 = 0x00000008;
    pub const COCPU_SARADC2_ERROR_INT_ST: u32 = 1 << 3;

    /// **COCPU_SARADC1_ERROR_INT_ST**
    ///
    /// An errro occurs from ADC1, int status.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const COCPU_SARADC1_ERROR_INT_ST_OFFSET: u32 = 2;
    pub const COCPU_SARADC1_ERROR_INT_ST_MASK: u32 = 0x00000004;
    pub const COCPU_SARADC1_ERROR_INT_ST: u32 = 1 << 2;

    /// **COCPU_SARADC2_INT_ST**
    ///
    /// ADC2 Conversion is done, int status.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const COCPU_SARADC2_INT_ST_OFFSET: u32 = 1;
    pub const COCPU_SARADC2_INT_ST_MASK: u32 = 0x00000002;
    pub const COCPU_SARADC2_INT_ST: u32 = 1 << 1;

    /// **COCPU_SARADC1_INT_ST**
    ///
    /// ADC1 Conversion is done, int status.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const COCPU_SARADC1_INT_ST_OFFSET: u32 = 0;
    pub const COCPU_SARADC1_INT_ST_MASK: u32 = 0x00000001;
    pub const COCPU_SARADC1_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **COCPU_SARADC2_WAKE_INT_ST** `[5]` (RO)
    ///
    /// A wakeup event is triggered from ADC2, int status.
    #[inline(always)]
    pub fn cocpu_saradc2_wake_int_st(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC2_WAKE_INT_ST_MASK) != 0
    }

    /// **COCPU_SARADC1_WAKE_INT_ST** `[4]` (RO)
    ///
    /// A wakeup event is triggered from ADC1, int status.
    #[inline(always)]
    pub fn cocpu_saradc1_wake_int_st(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC1_WAKE_INT_ST_MASK) != 0
    }

    /// **COCPU_SARADC2_ERROR_INT_ST** `[3]` (RO)
    ///
    /// An errro occurs from ADC2, int status.
    #[inline(always)]
    pub fn cocpu_saradc2_error_int_st(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC2_ERROR_INT_ST_MASK) != 0
    }

    /// **COCPU_SARADC1_ERROR_INT_ST** `[2]` (RO)
    ///
    /// An errro occurs from ADC1, int status.
    #[inline(always)]
    pub fn cocpu_saradc1_error_int_st(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC1_ERROR_INT_ST_MASK) != 0
    }

    /// **COCPU_SARADC2_INT_ST** `[1]` (RO)
    ///
    /// ADC2 Conversion is done, int status.
    #[inline(always)]
    pub fn cocpu_saradc2_int_st(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC2_INT_ST_MASK) != 0
    }

    /// **COCPU_SARADC1_INT_ST** `[0]` (RO)
    ///
    /// ADC1 Conversion is done, int status.
    #[inline(always)]
    pub fn cocpu_saradc1_int_st(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_SARADC1_INT_ST_MASK) != 0
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

    /// Read **COCPU_SARADC2_WAKE_INT_ST** `[5:5]`
    ///
    /// A wakeup event is triggered from ADC2, int status.
    #[inline(always)]
    pub const fn cocpu_saradc2_wake_int_st(&self) -> bool {
        (self.bits & IntSt::COCPU_SARADC2_WAKE_INT_ST_MASK) != 0
    }

    /// Read **COCPU_SARADC1_WAKE_INT_ST** `[4:4]`
    ///
    /// A wakeup event is triggered from ADC1, int status.
    #[inline(always)]
    pub const fn cocpu_saradc1_wake_int_st(&self) -> bool {
        (self.bits & IntSt::COCPU_SARADC1_WAKE_INT_ST_MASK) != 0
    }

    /// Read **COCPU_SARADC2_ERROR_INT_ST** `[3:3]`
    ///
    /// An errro occurs from ADC2, int status.
    #[inline(always)]
    pub const fn cocpu_saradc2_error_int_st(&self) -> bool {
        (self.bits & IntSt::COCPU_SARADC2_ERROR_INT_ST_MASK) != 0
    }

    /// Read **COCPU_SARADC1_ERROR_INT_ST** `[2:2]`
    ///
    /// An errro occurs from ADC1, int status.
    #[inline(always)]
    pub const fn cocpu_saradc1_error_int_st(&self) -> bool {
        (self.bits & IntSt::COCPU_SARADC1_ERROR_INT_ST_MASK) != 0
    }

    /// Read **COCPU_SARADC2_INT_ST** `[1:1]`
    ///
    /// ADC2 Conversion is done, int status.
    #[inline(always)]
    pub const fn cocpu_saradc2_int_st(&self) -> bool {
        (self.bits & IntSt::COCPU_SARADC2_INT_ST_MASK) != 0
    }

    /// Read **COCPU_SARADC1_INT_ST** `[0:0]`
    ///
    /// ADC1 Conversion is done, int status.
    #[inline(always)]
    pub const fn cocpu_saradc1_int_st(&self) -> bool {
        (self.bits & IntSt::COCPU_SARADC1_INT_ST_MASK) != 0
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
/// Interrupt clear registers.
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **COCPU_SARADC2_WAKE_INT_CLR** `[5]` (WO)
///
///   A wakeup event is triggered from ADC2, int clear.
///
/// - **COCPU_SARADC1_WAKE_INT_CLR** `[4]` (WO)
///
///   A wakeup event is triggered from ADC1, int clear.
///
/// - **COCPU_SARADC2_ERROR_INT_CLR** `[3]` (WO)
///
///   An errro occurs from ADC2, int clear.
///
/// - **COCPU_SARADC1_ERROR_INT_CLR** `[2]` (WO)
///
///   An errro occurs from ADC1, int clear.
///
/// - **COCPU_SARADC2_INT_CLR** `[1]` (WO)
///
///   ADC2 Conversion is done, int clear.
///
/// - **COCPU_SARADC1_INT_CLR** `[0]` (WO)
///
///   ADC1 Conversion is done, int clear.
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

    /// **COCPU_SARADC2_WAKE_INT_CLR**
    ///
    /// A wakeup event is triggered from ADC2, int clear.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const COCPU_SARADC2_WAKE_INT_CLR_OFFSET: u32 = 5;
    pub const COCPU_SARADC2_WAKE_INT_CLR_MASK: u32 = 0x00000020;
    pub const COCPU_SARADC2_WAKE_INT_CLR: u32 = 1 << 5;

    /// **COCPU_SARADC1_WAKE_INT_CLR**
    ///
    /// A wakeup event is triggered from ADC1, int clear.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const COCPU_SARADC1_WAKE_INT_CLR_OFFSET: u32 = 4;
    pub const COCPU_SARADC1_WAKE_INT_CLR_MASK: u32 = 0x00000010;
    pub const COCPU_SARADC1_WAKE_INT_CLR: u32 = 1 << 4;

    /// **COCPU_SARADC2_ERROR_INT_CLR**
    ///
    /// An errro occurs from ADC2, int clear.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const COCPU_SARADC2_ERROR_INT_CLR_OFFSET: u32 = 3;
    pub const COCPU_SARADC2_ERROR_INT_CLR_MASK: u32 = 0x00000008;
    pub const COCPU_SARADC2_ERROR_INT_CLR: u32 = 1 << 3;

    /// **COCPU_SARADC1_ERROR_INT_CLR**
    ///
    /// An errro occurs from ADC1, int clear.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const COCPU_SARADC1_ERROR_INT_CLR_OFFSET: u32 = 2;
    pub const COCPU_SARADC1_ERROR_INT_CLR_MASK: u32 = 0x00000004;
    pub const COCPU_SARADC1_ERROR_INT_CLR: u32 = 1 << 2;

    /// **COCPU_SARADC2_INT_CLR**
    ///
    /// ADC2 Conversion is done, int clear.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const COCPU_SARADC2_INT_CLR_OFFSET: u32 = 1;
    pub const COCPU_SARADC2_INT_CLR_MASK: u32 = 0x00000002;
    pub const COCPU_SARADC2_INT_CLR: u32 = 1 << 1;

    /// **COCPU_SARADC1_INT_CLR**
    ///
    /// ADC1 Conversion is done, int clear.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const COCPU_SARADC1_INT_CLR_OFFSET: u32 = 0;
    pub const COCPU_SARADC1_INT_CLR_MASK: u32 = 0x00000001;
    pub const COCPU_SARADC1_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **COCPU_SARADC2_WAKE_INT_CLR** (WO)
    ///
    /// A wakeup event is triggered from ADC2, int clear.
    #[inline(always)]
    pub fn trigger_cocpu_saradc2_wake_int_clr(&self) {
        self.value.write_bits(Self::COCPU_SARADC2_WAKE_INT_CLR_MASK);
    }

    /// Trigger **COCPU_SARADC1_WAKE_INT_CLR** (WO)
    ///
    /// A wakeup event is triggered from ADC1, int clear.
    #[inline(always)]
    pub fn trigger_cocpu_saradc1_wake_int_clr(&self) {
        self.value.write_bits(Self::COCPU_SARADC1_WAKE_INT_CLR_MASK);
    }

    /// Trigger **COCPU_SARADC2_ERROR_INT_CLR** (WO)
    ///
    /// An errro occurs from ADC2, int clear.
    #[inline(always)]
    pub fn trigger_cocpu_saradc2_error_int_clr(&self) {
        self.value
            .write_bits(Self::COCPU_SARADC2_ERROR_INT_CLR_MASK);
    }

    /// Trigger **COCPU_SARADC1_ERROR_INT_CLR** (WO)
    ///
    /// An errro occurs from ADC1, int clear.
    #[inline(always)]
    pub fn trigger_cocpu_saradc1_error_int_clr(&self) {
        self.value
            .write_bits(Self::COCPU_SARADC1_ERROR_INT_CLR_MASK);
    }

    /// Trigger **COCPU_SARADC2_INT_CLR** (WO)
    ///
    /// ADC2 Conversion is done, int clear.
    #[inline(always)]
    pub fn trigger_cocpu_saradc2_int_clr(&self) {
        self.value.write_bits(Self::COCPU_SARADC2_INT_CLR_MASK);
    }

    /// Trigger **COCPU_SARADC1_INT_CLR** (WO)
    ///
    /// ADC1 Conversion is done, int clear.
    #[inline(always)]
    pub fn trigger_cocpu_saradc1_int_clr(&self) {
        self.value.write_bits(Self::COCPU_SARADC1_INT_CLR_MASK);
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

    /// Set **COCPU_SARADC2_WAKE_INT_CLR** `[5:5]`
    ///
    /// A wakeup event is triggered from ADC2, int clear.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_wake_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::COCPU_SARADC2_WAKE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::COCPU_SARADC2_WAKE_INT_CLR_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_WAKE_INT_CLR** `[4:4]`
    ///
    /// A wakeup event is triggered from ADC1, int clear.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_wake_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::COCPU_SARADC1_WAKE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::COCPU_SARADC1_WAKE_INT_CLR_MASK
            },
        }
    }

    /// Set **COCPU_SARADC2_ERROR_INT_CLR** `[3:3]`
    ///
    /// An errro occurs from ADC2, int clear.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_error_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::COCPU_SARADC2_ERROR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::COCPU_SARADC2_ERROR_INT_CLR_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_ERROR_INT_CLR** `[2:2]`
    ///
    /// An errro occurs from ADC1, int clear.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_error_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::COCPU_SARADC1_ERROR_INT_CLR_MASK
            } else {
                self.bits & !IntClr::COCPU_SARADC1_ERROR_INT_CLR_MASK
            },
        }
    }

    /// Set **COCPU_SARADC2_INT_CLR** `[1:1]`
    ///
    /// ADC2 Conversion is done, int clear.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::COCPU_SARADC2_INT_CLR_MASK
            } else {
                self.bits & !IntClr::COCPU_SARADC2_INT_CLR_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_INT_CLR** `[0:0]`
    ///
    /// ADC1 Conversion is done, int clear.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::COCPU_SARADC1_INT_CLR_MASK
            } else {
                self.bits & !IntClr::COCPU_SARADC1_INT_CLR_MASK
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

/// INT_ENA_W1TS Register
///
/// Interrupt enable assert registers.
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **COCPU_SARADC2_WAKE_INT_ENA_W1TS** `[5]` (WO)
///
///   A wakeup event is triggered from ADC2, write 1 to assert int enable.
///
/// - **COCPU_SARADC1_WAKE_INT_ENA_W1TS** `[4]` (WO)
///
///   A wakeup event is triggered from ADC1, write 1 to assert int enable.
///
/// - **COCPU_SARADC2_ERROR_INT_ENA_W1TS** `[3]` (WO)
///
///   An errro occurs from ADC2, write 1 to assert int enable.
///
/// - **COCPU_SARADC1_ERROR_INT_ENA_W1TS** `[2]` (WO)
///
///   An errro occurs from ADC1, write 1 to assert int enable.
///
/// - **COCPU_SARADC2_INT_ENA_W1TS** `[1]` (WO)
///
///   ADC2 Conversion is done, write 1 to assert int enable.
///
/// - **COCPU_SARADC1_INT_ENA_W1TS** `[0]` (WO)
///
///   ADC1 Conversion is done, write 1 to assert int enable.
#[repr(transparent)]
pub struct IntEnaW1ts {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntEnaW1ts {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntEnaW1ts {
    // ======== Field Constants ========

    /// **COCPU_SARADC2_WAKE_INT_ENA_W1TS**
    ///
    /// A wakeup event is triggered from ADC2, write 1 to assert int enable.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const COCPU_SARADC2_WAKE_INT_ENA_W1TS_OFFSET: u32 = 5;
    pub const COCPU_SARADC2_WAKE_INT_ENA_W1TS_MASK: u32 = 0x00000020;
    pub const COCPU_SARADC2_WAKE_INT_ENA_W1TS: u32 = 1 << 5;

    /// **COCPU_SARADC1_WAKE_INT_ENA_W1TS**
    ///
    /// A wakeup event is triggered from ADC1, write 1 to assert int enable.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const COCPU_SARADC1_WAKE_INT_ENA_W1TS_OFFSET: u32 = 4;
    pub const COCPU_SARADC1_WAKE_INT_ENA_W1TS_MASK: u32 = 0x00000010;
    pub const COCPU_SARADC1_WAKE_INT_ENA_W1TS: u32 = 1 << 4;

    /// **COCPU_SARADC2_ERROR_INT_ENA_W1TS**
    ///
    /// An errro occurs from ADC2, write 1 to assert int enable.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const COCPU_SARADC2_ERROR_INT_ENA_W1TS_OFFSET: u32 = 3;
    pub const COCPU_SARADC2_ERROR_INT_ENA_W1TS_MASK: u32 = 0x00000008;
    pub const COCPU_SARADC2_ERROR_INT_ENA_W1TS: u32 = 1 << 3;

    /// **COCPU_SARADC1_ERROR_INT_ENA_W1TS**
    ///
    /// An errro occurs from ADC1, write 1 to assert int enable.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const COCPU_SARADC1_ERROR_INT_ENA_W1TS_OFFSET: u32 = 2;
    pub const COCPU_SARADC1_ERROR_INT_ENA_W1TS_MASK: u32 = 0x00000004;
    pub const COCPU_SARADC1_ERROR_INT_ENA_W1TS: u32 = 1 << 2;

    /// **COCPU_SARADC2_INT_ENA_W1TS**
    ///
    /// ADC2 Conversion is done, write 1 to assert int enable.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const COCPU_SARADC2_INT_ENA_W1TS_OFFSET: u32 = 1;
    pub const COCPU_SARADC2_INT_ENA_W1TS_MASK: u32 = 0x00000002;
    pub const COCPU_SARADC2_INT_ENA_W1TS: u32 = 1 << 1;

    /// **COCPU_SARADC1_INT_ENA_W1TS**
    ///
    /// ADC1 Conversion is done, write 1 to assert int enable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const COCPU_SARADC1_INT_ENA_W1TS_OFFSET: u32 = 0;
    pub const COCPU_SARADC1_INT_ENA_W1TS_MASK: u32 = 0x00000001;
    pub const COCPU_SARADC1_INT_ENA_W1TS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **COCPU_SARADC2_WAKE_INT_ENA_W1TS** (WO)
    ///
    /// A wakeup event is triggered from ADC2, write 1 to assert int enable.
    #[inline(always)]
    pub fn trigger_cocpu_saradc2_wake_int_ena_w1ts(&self) {
        self.value
            .write_bits(Self::COCPU_SARADC2_WAKE_INT_ENA_W1TS_MASK);
    }

    /// Trigger **COCPU_SARADC1_WAKE_INT_ENA_W1TS** (WO)
    ///
    /// A wakeup event is triggered from ADC1, write 1 to assert int enable.
    #[inline(always)]
    pub fn trigger_cocpu_saradc1_wake_int_ena_w1ts(&self) {
        self.value
            .write_bits(Self::COCPU_SARADC1_WAKE_INT_ENA_W1TS_MASK);
    }

    /// Trigger **COCPU_SARADC2_ERROR_INT_ENA_W1TS** (WO)
    ///
    /// An errro occurs from ADC2, write 1 to assert int enable.
    #[inline(always)]
    pub fn trigger_cocpu_saradc2_error_int_ena_w1ts(&self) {
        self.value
            .write_bits(Self::COCPU_SARADC2_ERROR_INT_ENA_W1TS_MASK);
    }

    /// Trigger **COCPU_SARADC1_ERROR_INT_ENA_W1TS** (WO)
    ///
    /// An errro occurs from ADC1, write 1 to assert int enable.
    #[inline(always)]
    pub fn trigger_cocpu_saradc1_error_int_ena_w1ts(&self) {
        self.value
            .write_bits(Self::COCPU_SARADC1_ERROR_INT_ENA_W1TS_MASK);
    }

    /// Trigger **COCPU_SARADC2_INT_ENA_W1TS** (WO)
    ///
    /// ADC2 Conversion is done, write 1 to assert int enable.
    #[inline(always)]
    pub fn trigger_cocpu_saradc2_int_ena_w1ts(&self) {
        self.value.write_bits(Self::COCPU_SARADC2_INT_ENA_W1TS_MASK);
    }

    /// Trigger **COCPU_SARADC1_INT_ENA_W1TS** (WO)
    ///
    /// ADC1 Conversion is done, write 1 to assert int enable.
    #[inline(always)]
    pub fn trigger_cocpu_saradc1_int_ena_w1ts(&self) {
        self.value.write_bits(Self::COCPU_SARADC1_INT_ENA_W1TS_MASK);
    }
}

/// INT_ENA_W1TS value type for batch read/modify/write operations
///
/// Use `intenaw1ts.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intenaw1ts.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntEnaW1tsVal {
    bits: u32,
}

impl IntEnaW1tsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **COCPU_SARADC2_WAKE_INT_ENA_W1TS** `[5:5]`
    ///
    /// A wakeup event is triggered from ADC2, write 1 to assert int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_wake_int_ena_w1ts(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaW1ts::COCPU_SARADC2_WAKE_INT_ENA_W1TS_MASK
            } else {
                self.bits & !IntEnaW1ts::COCPU_SARADC2_WAKE_INT_ENA_W1TS_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_WAKE_INT_ENA_W1TS** `[4:4]`
    ///
    /// A wakeup event is triggered from ADC1, write 1 to assert int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_wake_int_ena_w1ts(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaW1ts::COCPU_SARADC1_WAKE_INT_ENA_W1TS_MASK
            } else {
                self.bits & !IntEnaW1ts::COCPU_SARADC1_WAKE_INT_ENA_W1TS_MASK
            },
        }
    }

    /// Set **COCPU_SARADC2_ERROR_INT_ENA_W1TS** `[3:3]`
    ///
    /// An errro occurs from ADC2, write 1 to assert int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_error_int_ena_w1ts(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaW1ts::COCPU_SARADC2_ERROR_INT_ENA_W1TS_MASK
            } else {
                self.bits & !IntEnaW1ts::COCPU_SARADC2_ERROR_INT_ENA_W1TS_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_ERROR_INT_ENA_W1TS** `[2:2]`
    ///
    /// An errro occurs from ADC1, write 1 to assert int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_error_int_ena_w1ts(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaW1ts::COCPU_SARADC1_ERROR_INT_ENA_W1TS_MASK
            } else {
                self.bits & !IntEnaW1ts::COCPU_SARADC1_ERROR_INT_ENA_W1TS_MASK
            },
        }
    }

    /// Set **COCPU_SARADC2_INT_ENA_W1TS** `[1:1]`
    ///
    /// ADC2 Conversion is done, write 1 to assert int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_int_ena_w1ts(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaW1ts::COCPU_SARADC2_INT_ENA_W1TS_MASK
            } else {
                self.bits & !IntEnaW1ts::COCPU_SARADC2_INT_ENA_W1TS_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_INT_ENA_W1TS** `[0:0]`
    ///
    /// ADC1 Conversion is done, write 1 to assert int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_int_ena_w1ts(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaW1ts::COCPU_SARADC1_INT_ENA_W1TS_MASK
            } else {
                self.bits & !IntEnaW1ts::COCPU_SARADC1_INT_ENA_W1TS_MASK
            },
        }
    }
}

impl Default for IntEnaW1tsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntEnaW1ts {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntEnaW1tsVal) {
        self.value.write_bits(val.bits())
    }
}

/// INT_ENA_W1TC Register
///
/// Interrupt enable deassert registers.
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **COCPU_SARADC2_WAKE_INT_ENA_W1TC** `[5]` (WO)
///
///   A wakeup event is triggered from ADC2, write 1 to deassert int enable.
///
/// - **COCPU_SARADC1_WAKE_INT_ENA_W1TC** `[4]` (WO)
///
///   A wakeup event is triggered from ADC1, write 1 to deassert int enable.
///
/// - **COCPU_SARADC2_ERROR_INT_ENA_W1TC** `[3]` (WO)
///
///   An errro occurs from ADC2, write 1 to deassert int enable.
///
/// - **COCPU_SARADC1_ERROR_INT_ENA_W1TC** `[2]` (WO)
///
///   An errro occurs from ADC1, write 1 to deassert int enable.
///
/// - **COCPU_SARADC2_INT_ENA_W1TC** `[1]` (WO)
///
///   ADC2 Conversion is done, write 1 to deassert int enable.
///
/// - **COCPU_SARADC1_INT_ENA_W1TC** `[0]` (WO)
///
///   ADC1 Conversion is done, write 1 to deassert int enable.
#[repr(transparent)]
pub struct IntEnaW1tc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntEnaW1tc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntEnaW1tc {
    // ======== Field Constants ========

    /// **COCPU_SARADC2_WAKE_INT_ENA_W1TC**
    ///
    /// A wakeup event is triggered from ADC2, write 1 to deassert int enable.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const COCPU_SARADC2_WAKE_INT_ENA_W1TC_OFFSET: u32 = 5;
    pub const COCPU_SARADC2_WAKE_INT_ENA_W1TC_MASK: u32 = 0x00000020;
    pub const COCPU_SARADC2_WAKE_INT_ENA_W1TC: u32 = 1 << 5;

    /// **COCPU_SARADC1_WAKE_INT_ENA_W1TC**
    ///
    /// A wakeup event is triggered from ADC1, write 1 to deassert int enable.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const COCPU_SARADC1_WAKE_INT_ENA_W1TC_OFFSET: u32 = 4;
    pub const COCPU_SARADC1_WAKE_INT_ENA_W1TC_MASK: u32 = 0x00000010;
    pub const COCPU_SARADC1_WAKE_INT_ENA_W1TC: u32 = 1 << 4;

    /// **COCPU_SARADC2_ERROR_INT_ENA_W1TC**
    ///
    /// An errro occurs from ADC2, write 1 to deassert int enable.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const COCPU_SARADC2_ERROR_INT_ENA_W1TC_OFFSET: u32 = 3;
    pub const COCPU_SARADC2_ERROR_INT_ENA_W1TC_MASK: u32 = 0x00000008;
    pub const COCPU_SARADC2_ERROR_INT_ENA_W1TC: u32 = 1 << 3;

    /// **COCPU_SARADC1_ERROR_INT_ENA_W1TC**
    ///
    /// An errro occurs from ADC1, write 1 to deassert int enable.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const COCPU_SARADC1_ERROR_INT_ENA_W1TC_OFFSET: u32 = 2;
    pub const COCPU_SARADC1_ERROR_INT_ENA_W1TC_MASK: u32 = 0x00000004;
    pub const COCPU_SARADC1_ERROR_INT_ENA_W1TC: u32 = 1 << 2;

    /// **COCPU_SARADC2_INT_ENA_W1TC**
    ///
    /// ADC2 Conversion is done, write 1 to deassert int enable.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const COCPU_SARADC2_INT_ENA_W1TC_OFFSET: u32 = 1;
    pub const COCPU_SARADC2_INT_ENA_W1TC_MASK: u32 = 0x00000002;
    pub const COCPU_SARADC2_INT_ENA_W1TC: u32 = 1 << 1;

    /// **COCPU_SARADC1_INT_ENA_W1TC**
    ///
    /// ADC1 Conversion is done, write 1 to deassert int enable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const COCPU_SARADC1_INT_ENA_W1TC_OFFSET: u32 = 0;
    pub const COCPU_SARADC1_INT_ENA_W1TC_MASK: u32 = 0x00000001;
    pub const COCPU_SARADC1_INT_ENA_W1TC: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **COCPU_SARADC2_WAKE_INT_ENA_W1TC** (WO)
    ///
    /// A wakeup event is triggered from ADC2, write 1 to deassert int enable.
    #[inline(always)]
    pub fn trigger_cocpu_saradc2_wake_int_ena_w1tc(&self) {
        self.value
            .write_bits(Self::COCPU_SARADC2_WAKE_INT_ENA_W1TC_MASK);
    }

    /// Trigger **COCPU_SARADC1_WAKE_INT_ENA_W1TC** (WO)
    ///
    /// A wakeup event is triggered from ADC1, write 1 to deassert int enable.
    #[inline(always)]
    pub fn trigger_cocpu_saradc1_wake_int_ena_w1tc(&self) {
        self.value
            .write_bits(Self::COCPU_SARADC1_WAKE_INT_ENA_W1TC_MASK);
    }

    /// Trigger **COCPU_SARADC2_ERROR_INT_ENA_W1TC** (WO)
    ///
    /// An errro occurs from ADC2, write 1 to deassert int enable.
    #[inline(always)]
    pub fn trigger_cocpu_saradc2_error_int_ena_w1tc(&self) {
        self.value
            .write_bits(Self::COCPU_SARADC2_ERROR_INT_ENA_W1TC_MASK);
    }

    /// Trigger **COCPU_SARADC1_ERROR_INT_ENA_W1TC** (WO)
    ///
    /// An errro occurs from ADC1, write 1 to deassert int enable.
    #[inline(always)]
    pub fn trigger_cocpu_saradc1_error_int_ena_w1tc(&self) {
        self.value
            .write_bits(Self::COCPU_SARADC1_ERROR_INT_ENA_W1TC_MASK);
    }

    /// Trigger **COCPU_SARADC2_INT_ENA_W1TC** (WO)
    ///
    /// ADC2 Conversion is done, write 1 to deassert int enable.
    #[inline(always)]
    pub fn trigger_cocpu_saradc2_int_ena_w1tc(&self) {
        self.value.write_bits(Self::COCPU_SARADC2_INT_ENA_W1TC_MASK);
    }

    /// Trigger **COCPU_SARADC1_INT_ENA_W1TC** (WO)
    ///
    /// ADC1 Conversion is done, write 1 to deassert int enable.
    #[inline(always)]
    pub fn trigger_cocpu_saradc1_int_ena_w1tc(&self) {
        self.value.write_bits(Self::COCPU_SARADC1_INT_ENA_W1TC_MASK);
    }
}

/// INT_ENA_W1TC value type for batch read/modify/write operations
///
/// Use `intenaw1tc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intenaw1tc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntEnaW1tcVal {
    bits: u32,
}

impl IntEnaW1tcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **COCPU_SARADC2_WAKE_INT_ENA_W1TC** `[5:5]`
    ///
    /// A wakeup event is triggered from ADC2, write 1 to deassert int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_wake_int_ena_w1tc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaW1tc::COCPU_SARADC2_WAKE_INT_ENA_W1TC_MASK
            } else {
                self.bits & !IntEnaW1tc::COCPU_SARADC2_WAKE_INT_ENA_W1TC_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_WAKE_INT_ENA_W1TC** `[4:4]`
    ///
    /// A wakeup event is triggered from ADC1, write 1 to deassert int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_wake_int_ena_w1tc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaW1tc::COCPU_SARADC1_WAKE_INT_ENA_W1TC_MASK
            } else {
                self.bits & !IntEnaW1tc::COCPU_SARADC1_WAKE_INT_ENA_W1TC_MASK
            },
        }
    }

    /// Set **COCPU_SARADC2_ERROR_INT_ENA_W1TC** `[3:3]`
    ///
    /// An errro occurs from ADC2, write 1 to deassert int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_error_int_ena_w1tc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaW1tc::COCPU_SARADC2_ERROR_INT_ENA_W1TC_MASK
            } else {
                self.bits & !IntEnaW1tc::COCPU_SARADC2_ERROR_INT_ENA_W1TC_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_ERROR_INT_ENA_W1TC** `[2:2]`
    ///
    /// An errro occurs from ADC1, write 1 to deassert int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_error_int_ena_w1tc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaW1tc::COCPU_SARADC1_ERROR_INT_ENA_W1TC_MASK
            } else {
                self.bits & !IntEnaW1tc::COCPU_SARADC1_ERROR_INT_ENA_W1TC_MASK
            },
        }
    }

    /// Set **COCPU_SARADC2_INT_ENA_W1TC** `[1:1]`
    ///
    /// ADC2 Conversion is done, write 1 to deassert int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc2_int_ena_w1tc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaW1tc::COCPU_SARADC2_INT_ENA_W1TC_MASK
            } else {
                self.bits & !IntEnaW1tc::COCPU_SARADC2_INT_ENA_W1TC_MASK
            },
        }
    }

    /// Set **COCPU_SARADC1_INT_ENA_W1TC** `[0:0]`
    ///
    /// ADC1 Conversion is done, write 1 to deassert int enable.
    #[inline(always)]
    pub const fn with_cocpu_saradc1_int_ena_w1tc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaW1tc::COCPU_SARADC1_INT_ENA_W1TC_MASK
            } else {
                self.bits & !IntEnaW1tc::COCPU_SARADC1_INT_ENA_W1TC_MASK
            },
        }
    }
}

impl Default for IntEnaW1tcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntEnaW1tc {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntEnaW1tcVal) {
        self.value.write_bits(val.bits())
    }
}

/// WAKEUP1 Register
///
/// ADC1 wakeup configuration registers.
///
/// Address offset: 0x0060
/// Reset value: 0x03ffc000
///
/// ## Fields
///
/// - **SAR1_WAKEUP_MODE** `[31]` (RW)
///
///   0:absolute value comparison mode. 1: relative value comparison mode.
///
/// - **SAR1_WAKEUP_EN** `[30]` (RW)
///
///   Wakeup function enable.
///
/// - **SAR1_WAKEUP_OVER_UPPER_TH** `[29]` (RO)
///
///   Indicates that this wakeup event arose from exceeding upper threshold.
///
/// - **SAR1_WAKEUP_TH_HIGH** `[25:14]` (RW)
///
///   Upper threshold.
///
/// - **SAR1_WAKEUP_TH_LOW** `[11:0]` (RW)
///
///   Lower threshold.
#[repr(transparent)]
pub struct Wakeup1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wakeup1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wakeup1 {
    // ======== Field Constants ========

    /// **SAR1_WAKEUP_MODE**
    ///
    /// 0:absolute value comparison mode. 1: relative value comparison mode.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SAR1_WAKEUP_MODE_OFFSET: u32 = 31;
    pub const SAR1_WAKEUP_MODE_MASK: u32 = 0x80000000;
    pub const SAR1_WAKEUP_MODE: u32 = 1 << 31;

    /// **SAR1_WAKEUP_EN**
    ///
    /// Wakeup function enable.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SAR1_WAKEUP_EN_OFFSET: u32 = 30;
    pub const SAR1_WAKEUP_EN_MASK: u32 = 0x40000000;
    pub const SAR1_WAKEUP_EN: u32 = 1 << 30;

    /// **SAR1_WAKEUP_OVER_UPPER_TH**
    ///
    /// Indicates that this wakeup event arose from exceeding upper threshold.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SAR1_WAKEUP_OVER_UPPER_TH_OFFSET: u32 = 29;
    pub const SAR1_WAKEUP_OVER_UPPER_TH_MASK: u32 = 0x20000000;
    pub const SAR1_WAKEUP_OVER_UPPER_TH: u32 = 1 << 29;

    /// **SAR1_WAKEUP_TH_HIGH**
    ///
    /// Upper threshold.
    ///
    /// Bits `[25:14]`, mask `0x03ffc000`
    pub const SAR1_WAKEUP_TH_HIGH_OFFSET: u32 = 14;
    pub const SAR1_WAKEUP_TH_HIGH_MASK: u32 = 0x03ffc000;

    /// **SAR1_WAKEUP_TH_LOW**
    ///
    /// Lower threshold.
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const SAR1_WAKEUP_TH_LOW_OFFSET: u32 = 0;
    pub const SAR1_WAKEUP_TH_LOW_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **SAR1_WAKEUP_MODE** `[31]` (RW)
    ///
    /// 0:absolute value comparison mode. 1: relative value comparison mode.
    #[inline(always)]
    pub fn sar1_wakeup_mode(&self) -> bool {
        (self.value.read_bits() & Self::SAR1_WAKEUP_MODE_MASK) != 0
    }

    /// Set SAR1_WAKEUP_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar1_wakeup_mode(&self) {
        self.value.modify_bits(|r| r | Self::SAR1_WAKEUP_MODE_MASK);
    }

    /// Clear SAR1_WAKEUP_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar1_wakeup_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SAR1_WAKEUP_MODE_MASK);
    }

    /// **SAR1_WAKEUP_EN** `[30]` (RW)
    ///
    /// Wakeup function enable.
    #[inline(always)]
    pub fn sar1_wakeup_en(&self) -> bool {
        (self.value.read_bits() & Self::SAR1_WAKEUP_EN_MASK) != 0
    }

    /// Set SAR1_WAKEUP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar1_wakeup_en(&self) {
        self.value.modify_bits(|r| r | Self::SAR1_WAKEUP_EN_MASK);
    }

    /// Clear SAR1_WAKEUP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar1_wakeup_en(&self) {
        self.value.modify_bits(|r| r & !Self::SAR1_WAKEUP_EN_MASK);
    }

    /// **SAR1_WAKEUP_OVER_UPPER_TH** `[29]` (RO)
    ///
    /// Indicates that this wakeup event arose from exceeding upper threshold.
    #[inline(always)]
    pub fn sar1_wakeup_over_upper_th(&self) -> bool {
        (self.value.read_bits() & Self::SAR1_WAKEUP_OVER_UPPER_TH_MASK) != 0
    }

    /// **SAR1_WAKEUP_TH_HIGH** `[25:14]` (RW)
    ///
    /// Upper threshold.
    #[inline(always)]
    pub fn sar1_wakeup_th_high(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_WAKEUP_TH_HIGH_MASK)
            >> Self::SAR1_WAKEUP_TH_HIGH_OFFSET
    }

    /// Write **SAR1_WAKEUP_TH_HIGH** `[25:14]` (RW)
    ///
    /// Upper threshold.
    #[inline(always)]
    pub fn write_sar1_wakeup_th_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR1_WAKEUP_TH_HIGH_MASK)
                | ((value << Self::SAR1_WAKEUP_TH_HIGH_OFFSET) & Self::SAR1_WAKEUP_TH_HIGH_MASK)
        });
    }

    /// **SAR1_WAKEUP_TH_LOW** `[11:0]` (RW)
    ///
    /// Lower threshold.
    #[inline(always)]
    pub fn sar1_wakeup_th_low(&self) -> u32 {
        (self.value.read_bits() & Self::SAR1_WAKEUP_TH_LOW_MASK) >> Self::SAR1_WAKEUP_TH_LOW_OFFSET
    }

    /// Write **SAR1_WAKEUP_TH_LOW** `[11:0]` (RW)
    ///
    /// Lower threshold.
    #[inline(always)]
    pub fn write_sar1_wakeup_th_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR1_WAKEUP_TH_LOW_MASK)
                | ((value << Self::SAR1_WAKEUP_TH_LOW_OFFSET) & Self::SAR1_WAKEUP_TH_LOW_MASK)
        });
    }
}

/// WAKEUP1 value type for batch read/modify/write operations
///
/// Use `wakeup1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wakeup1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Wakeup1Val {
    bits: u32,
}

impl Wakeup1Val {
    /// Create from raw bits
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
        Self { bits: 0x03ffc000 }
    }

    /// Read **SAR1_WAKEUP_MODE** `[31:31]`
    ///
    /// 0:absolute value comparison mode. 1: relative value comparison mode.
    #[inline(always)]
    pub const fn sar1_wakeup_mode(&self) -> bool {
        (self.bits & Wakeup1::SAR1_WAKEUP_MODE_MASK) != 0
    }

    /// Read **SAR1_WAKEUP_EN** `[30:30]`
    ///
    /// Wakeup function enable.
    #[inline(always)]
    pub const fn sar1_wakeup_en(&self) -> bool {
        (self.bits & Wakeup1::SAR1_WAKEUP_EN_MASK) != 0
    }

    /// Read **SAR1_WAKEUP_OVER_UPPER_TH** `[29:29]`
    ///
    /// Indicates that this wakeup event arose from exceeding upper threshold.
    #[inline(always)]
    pub const fn sar1_wakeup_over_upper_th(&self) -> bool {
        (self.bits & Wakeup1::SAR1_WAKEUP_OVER_UPPER_TH_MASK) != 0
    }

    /// Read **SAR1_WAKEUP_TH_HIGH** `[25:14]`
    ///
    /// Upper threshold.
    #[inline(always)]
    pub const fn sar1_wakeup_th_high(&self) -> u32 {
        (self.bits & Wakeup1::SAR1_WAKEUP_TH_HIGH_MASK) >> Wakeup1::SAR1_WAKEUP_TH_HIGH_OFFSET
    }

    /// Read **SAR1_WAKEUP_TH_LOW** `[11:0]`
    ///
    /// Lower threshold.
    #[inline(always)]
    pub const fn sar1_wakeup_th_low(&self) -> u32 {
        (self.bits & Wakeup1::SAR1_WAKEUP_TH_LOW_MASK) >> Wakeup1::SAR1_WAKEUP_TH_LOW_OFFSET
    }

    /// Set **SAR1_WAKEUP_MODE** `[31:31]`
    ///
    /// 0:absolute value comparison mode. 1: relative value comparison mode.
    #[inline(always)]
    pub const fn with_sar1_wakeup_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wakeup1::SAR1_WAKEUP_MODE_MASK
            } else {
                self.bits & !Wakeup1::SAR1_WAKEUP_MODE_MASK
            },
        }
    }

    /// Set **SAR1_WAKEUP_EN** `[30:30]`
    ///
    /// Wakeup function enable.
    #[inline(always)]
    pub const fn with_sar1_wakeup_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wakeup1::SAR1_WAKEUP_EN_MASK
            } else {
                self.bits & !Wakeup1::SAR1_WAKEUP_EN_MASK
            },
        }
    }

    /// Set **SAR1_WAKEUP_TH_HIGH** `[25:14]`
    ///
    /// Upper threshold.
    #[inline(always)]
    pub const fn with_sar1_wakeup_th_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wakeup1::SAR1_WAKEUP_TH_HIGH_MASK)
                | ((value << Wakeup1::SAR1_WAKEUP_TH_HIGH_OFFSET)
                    & Wakeup1::SAR1_WAKEUP_TH_HIGH_MASK),
        }
    }

    /// Set **SAR1_WAKEUP_TH_LOW** `[11:0]`
    ///
    /// Lower threshold.
    #[inline(always)]
    pub const fn with_sar1_wakeup_th_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wakeup1::SAR1_WAKEUP_TH_LOW_MASK)
                | ((value << Wakeup1::SAR1_WAKEUP_TH_LOW_OFFSET)
                    & Wakeup1::SAR1_WAKEUP_TH_LOW_MASK),
        }
    }
}

impl Default for Wakeup1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wakeup1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Wakeup1Val {
        Wakeup1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Wakeup1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Wakeup1Val) -> Wakeup1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WAKEUP2 Register
///
/// ADC2 wakeup configuration registers.
///
/// Address offset: 0x0064
/// Reset value: 0x03ffc000
///
/// ## Fields
///
/// - **SAR2_WAKEUP_MODE** `[31]` (RW)
///
///   0:absolute value comparison mode. 1: relative value comparison mode.
///
/// - **SAR2_WAKEUP_EN** `[30]` (RW)
///
///   Wakeup function enable.
///
/// - **SAR2_WAKEUP_OVER_UPPER_TH** `[29]` (RO)
///
///   Indicates that this wakeup event arose from exceeding upper threshold.
///
/// - **SAR2_WAKEUP_TH_HIGH** `[25:14]` (RW)
///
///   Upper threshold.
///
/// - **SAR2_WAKEUP_TH_LOW** `[11:0]` (RW)
///
///   Lower threshold.
#[repr(transparent)]
pub struct Wakeup2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wakeup2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wakeup2 {
    // ======== Field Constants ========

    /// **SAR2_WAKEUP_MODE**
    ///
    /// 0:absolute value comparison mode. 1: relative value comparison mode.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SAR2_WAKEUP_MODE_OFFSET: u32 = 31;
    pub const SAR2_WAKEUP_MODE_MASK: u32 = 0x80000000;
    pub const SAR2_WAKEUP_MODE: u32 = 1 << 31;

    /// **SAR2_WAKEUP_EN**
    ///
    /// Wakeup function enable.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SAR2_WAKEUP_EN_OFFSET: u32 = 30;
    pub const SAR2_WAKEUP_EN_MASK: u32 = 0x40000000;
    pub const SAR2_WAKEUP_EN: u32 = 1 << 30;

    /// **SAR2_WAKEUP_OVER_UPPER_TH**
    ///
    /// Indicates that this wakeup event arose from exceeding upper threshold.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const SAR2_WAKEUP_OVER_UPPER_TH_OFFSET: u32 = 29;
    pub const SAR2_WAKEUP_OVER_UPPER_TH_MASK: u32 = 0x20000000;
    pub const SAR2_WAKEUP_OVER_UPPER_TH: u32 = 1 << 29;

    /// **SAR2_WAKEUP_TH_HIGH**
    ///
    /// Upper threshold.
    ///
    /// Bits `[25:14]`, mask `0x03ffc000`
    pub const SAR2_WAKEUP_TH_HIGH_OFFSET: u32 = 14;
    pub const SAR2_WAKEUP_TH_HIGH_MASK: u32 = 0x03ffc000;

    /// **SAR2_WAKEUP_TH_LOW**
    ///
    /// Lower threshold.
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const SAR2_WAKEUP_TH_LOW_OFFSET: u32 = 0;
    pub const SAR2_WAKEUP_TH_LOW_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **SAR2_WAKEUP_MODE** `[31]` (RW)
    ///
    /// 0:absolute value comparison mode. 1: relative value comparison mode.
    #[inline(always)]
    pub fn sar2_wakeup_mode(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_WAKEUP_MODE_MASK) != 0
    }

    /// Set SAR2_WAKEUP_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_wakeup_mode(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_WAKEUP_MODE_MASK);
    }

    /// Clear SAR2_WAKEUP_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_wakeup_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SAR2_WAKEUP_MODE_MASK);
    }

    /// **SAR2_WAKEUP_EN** `[30]` (RW)
    ///
    /// Wakeup function enable.
    #[inline(always)]
    pub fn sar2_wakeup_en(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_WAKEUP_EN_MASK) != 0
    }

    /// Set SAR2_WAKEUP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_wakeup_en(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_WAKEUP_EN_MASK);
    }

    /// Clear SAR2_WAKEUP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_wakeup_en(&self) {
        self.value.modify_bits(|r| r & !Self::SAR2_WAKEUP_EN_MASK);
    }

    /// **SAR2_WAKEUP_OVER_UPPER_TH** `[29]` (RO)
    ///
    /// Indicates that this wakeup event arose from exceeding upper threshold.
    #[inline(always)]
    pub fn sar2_wakeup_over_upper_th(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_WAKEUP_OVER_UPPER_TH_MASK) != 0
    }

    /// **SAR2_WAKEUP_TH_HIGH** `[25:14]` (RW)
    ///
    /// Upper threshold.
    #[inline(always)]
    pub fn sar2_wakeup_th_high(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_WAKEUP_TH_HIGH_MASK)
            >> Self::SAR2_WAKEUP_TH_HIGH_OFFSET
    }

    /// Write **SAR2_WAKEUP_TH_HIGH** `[25:14]` (RW)
    ///
    /// Upper threshold.
    #[inline(always)]
    pub fn write_sar2_wakeup_th_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_WAKEUP_TH_HIGH_MASK)
                | ((value << Self::SAR2_WAKEUP_TH_HIGH_OFFSET) & Self::SAR2_WAKEUP_TH_HIGH_MASK)
        });
    }

    /// **SAR2_WAKEUP_TH_LOW** `[11:0]` (RW)
    ///
    /// Lower threshold.
    #[inline(always)]
    pub fn sar2_wakeup_th_low(&self) -> u32 {
        (self.value.read_bits() & Self::SAR2_WAKEUP_TH_LOW_MASK) >> Self::SAR2_WAKEUP_TH_LOW_OFFSET
    }

    /// Write **SAR2_WAKEUP_TH_LOW** `[11:0]` (RW)
    ///
    /// Lower threshold.
    #[inline(always)]
    pub fn write_sar2_wakeup_th_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAR2_WAKEUP_TH_LOW_MASK)
                | ((value << Self::SAR2_WAKEUP_TH_LOW_OFFSET) & Self::SAR2_WAKEUP_TH_LOW_MASK)
        });
    }
}

/// WAKEUP2 value type for batch read/modify/write operations
///
/// Use `wakeup2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wakeup2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Wakeup2Val {
    bits: u32,
}

impl Wakeup2Val {
    /// Create from raw bits
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
        Self { bits: 0x03ffc000 }
    }

    /// Read **SAR2_WAKEUP_MODE** `[31:31]`
    ///
    /// 0:absolute value comparison mode. 1: relative value comparison mode.
    #[inline(always)]
    pub const fn sar2_wakeup_mode(&self) -> bool {
        (self.bits & Wakeup2::SAR2_WAKEUP_MODE_MASK) != 0
    }

    /// Read **SAR2_WAKEUP_EN** `[30:30]`
    ///
    /// Wakeup function enable.
    #[inline(always)]
    pub const fn sar2_wakeup_en(&self) -> bool {
        (self.bits & Wakeup2::SAR2_WAKEUP_EN_MASK) != 0
    }

    /// Read **SAR2_WAKEUP_OVER_UPPER_TH** `[29:29]`
    ///
    /// Indicates that this wakeup event arose from exceeding upper threshold.
    #[inline(always)]
    pub const fn sar2_wakeup_over_upper_th(&self) -> bool {
        (self.bits & Wakeup2::SAR2_WAKEUP_OVER_UPPER_TH_MASK) != 0
    }

    /// Read **SAR2_WAKEUP_TH_HIGH** `[25:14]`
    ///
    /// Upper threshold.
    #[inline(always)]
    pub const fn sar2_wakeup_th_high(&self) -> u32 {
        (self.bits & Wakeup2::SAR2_WAKEUP_TH_HIGH_MASK) >> Wakeup2::SAR2_WAKEUP_TH_HIGH_OFFSET
    }

    /// Read **SAR2_WAKEUP_TH_LOW** `[11:0]`
    ///
    /// Lower threshold.
    #[inline(always)]
    pub const fn sar2_wakeup_th_low(&self) -> u32 {
        (self.bits & Wakeup2::SAR2_WAKEUP_TH_LOW_MASK) >> Wakeup2::SAR2_WAKEUP_TH_LOW_OFFSET
    }

    /// Set **SAR2_WAKEUP_MODE** `[31:31]`
    ///
    /// 0:absolute value comparison mode. 1: relative value comparison mode.
    #[inline(always)]
    pub const fn with_sar2_wakeup_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wakeup2::SAR2_WAKEUP_MODE_MASK
            } else {
                self.bits & !Wakeup2::SAR2_WAKEUP_MODE_MASK
            },
        }
    }

    /// Set **SAR2_WAKEUP_EN** `[30:30]`
    ///
    /// Wakeup function enable.
    #[inline(always)]
    pub const fn with_sar2_wakeup_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wakeup2::SAR2_WAKEUP_EN_MASK
            } else {
                self.bits & !Wakeup2::SAR2_WAKEUP_EN_MASK
            },
        }
    }

    /// Set **SAR2_WAKEUP_TH_HIGH** `[25:14]`
    ///
    /// Upper threshold.
    #[inline(always)]
    pub const fn with_sar2_wakeup_th_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wakeup2::SAR2_WAKEUP_TH_HIGH_MASK)
                | ((value << Wakeup2::SAR2_WAKEUP_TH_HIGH_OFFSET)
                    & Wakeup2::SAR2_WAKEUP_TH_HIGH_MASK),
        }
    }

    /// Set **SAR2_WAKEUP_TH_LOW** `[11:0]`
    ///
    /// Lower threshold.
    #[inline(always)]
    pub const fn with_sar2_wakeup_th_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wakeup2::SAR2_WAKEUP_TH_LOW_MASK)
                | ((value << Wakeup2::SAR2_WAKEUP_TH_LOW_OFFSET)
                    & Wakeup2::SAR2_WAKEUP_TH_LOW_MASK),
        }
    }
}

impl Default for Wakeup2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wakeup2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Wakeup2Val {
        Wakeup2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Wakeup2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Wakeup2Val) -> Wakeup2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WAKEUP_SEL Register
///
/// Wakeup source select register.
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **SAR_WAKEUP_SEL** `[0]` (RW)
///
///   0: ADC1. 1: ADC2.
#[repr(transparent)]
pub struct WakeupSel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WakeupSel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WakeupSel {
    // ======== Field Constants ========

    /// **SAR_WAKEUP_SEL**
    ///
    /// 0: ADC1. 1: ADC2.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SAR_WAKEUP_SEL_OFFSET: u32 = 0;
    pub const SAR_WAKEUP_SEL_MASK: u32 = 0x00000001;
    pub const SAR_WAKEUP_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SAR_WAKEUP_SEL** `[0]` (RW)
    ///
    /// 0: ADC1. 1: ADC2.
    #[inline(always)]
    pub fn sar_wakeup_sel(&self) -> bool {
        (self.value.read_bits() & Self::SAR_WAKEUP_SEL_MASK) != 0
    }

    /// Set SAR_WAKEUP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar_wakeup_sel(&self) {
        self.value.modify_bits(|r| r | Self::SAR_WAKEUP_SEL_MASK);
    }

    /// Clear SAR_WAKEUP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar_wakeup_sel(&self) {
        self.value.modify_bits(|r| r & !Self::SAR_WAKEUP_SEL_MASK);
    }
}

/// WAKEUP_SEL value type for batch read/modify/write operations
///
/// Use `wakeupsel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wakeupsel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WakeupSelVal {
    bits: u32,
}

impl WakeupSelVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SAR_WAKEUP_SEL** `[0:0]`
    ///
    /// 0: ADC1. 1: ADC2.
    #[inline(always)]
    pub const fn sar_wakeup_sel(&self) -> bool {
        (self.bits & WakeupSel::SAR_WAKEUP_SEL_MASK) != 0
    }

    /// Set **SAR_WAKEUP_SEL** `[0:0]`
    ///
    /// 0: ADC1. 1: ADC2.
    #[inline(always)]
    pub const fn with_sar_wakeup_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeupSel::SAR_WAKEUP_SEL_MASK
            } else {
                self.bits & !WakeupSel::SAR_WAKEUP_SEL_MASK
            },
        }
    }
}

impl Default for WakeupSelVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl WakeupSel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WakeupSelVal {
        WakeupSelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WakeupSelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WakeupSelVal) -> WakeupSelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SAR1_HW_WAKEUP Register
///
/// Hardware automatic sampling registers for wakeup function.
///
/// Address offset: 0x006c
/// Reset value: 0x000000c8
///
/// ## Fields
///
/// - **ADC1_HW_READ_RATE_I** `[16:1]` (RW)
///
///   Hardware automatic sampling rate.
///
/// - **ADC1_HW_READ_EN_I** `[0]` (RW)
///
///   Enable hardware automatic sampling.
#[repr(transparent)]
pub struct Sar1HwWakeup {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sar1HwWakeup {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sar1HwWakeup {
    // ======== Field Constants ========

    /// **ADC1_HW_READ_RATE_I**
    ///
    /// Hardware automatic sampling rate.
    ///
    /// Bits `[16:1]`, mask `0x0001fffe`
    pub const ADC1_HW_READ_RATE_I_OFFSET: u32 = 1;
    pub const ADC1_HW_READ_RATE_I_MASK: u32 = 0x0001fffe;

    /// **ADC1_HW_READ_EN_I**
    ///
    /// Enable hardware automatic sampling.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ADC1_HW_READ_EN_I_OFFSET: u32 = 0;
    pub const ADC1_HW_READ_EN_I_MASK: u32 = 0x00000001;
    pub const ADC1_HW_READ_EN_I: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ADC1_HW_READ_RATE_I** `[16:1]` (RW)
    ///
    /// Hardware automatic sampling rate.
    #[inline(always)]
    pub fn adc1_hw_read_rate_i(&self) -> u32 {
        (self.value.read_bits() & Self::ADC1_HW_READ_RATE_I_MASK)
            >> Self::ADC1_HW_READ_RATE_I_OFFSET
    }

    /// Write **ADC1_HW_READ_RATE_I** `[16:1]` (RW)
    ///
    /// Hardware automatic sampling rate.
    #[inline(always)]
    pub fn write_adc1_hw_read_rate_i(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADC1_HW_READ_RATE_I_MASK)
                | ((value << Self::ADC1_HW_READ_RATE_I_OFFSET) & Self::ADC1_HW_READ_RATE_I_MASK)
        });
    }

    /// **ADC1_HW_READ_EN_I** `[0]` (RW)
    ///
    /// Enable hardware automatic sampling.
    #[inline(always)]
    pub fn adc1_hw_read_en_i(&self) -> bool {
        (self.value.read_bits() & Self::ADC1_HW_READ_EN_I_MASK) != 0
    }

    /// Set ADC1_HW_READ_EN_I bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_adc1_hw_read_en_i(&self) {
        self.value.modify_bits(|r| r | Self::ADC1_HW_READ_EN_I_MASK);
    }

    /// Clear ADC1_HW_READ_EN_I bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_adc1_hw_read_en_i(&self) {
        self.value
            .modify_bits(|r| r & !Self::ADC1_HW_READ_EN_I_MASK);
    }
}

/// SAR1_HW_WAKEUP value type for batch read/modify/write operations
///
/// Use `sar1hwwakeup.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sar1hwwakeup.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sar1HwWakeupVal {
    bits: u32,
}

impl Sar1HwWakeupVal {
    /// Create from raw bits
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
        Self { bits: 0x000000c8 }
    }

    /// Read **ADC1_HW_READ_RATE_I** `[16:1]`
    ///
    /// Hardware automatic sampling rate.
    #[inline(always)]
    pub const fn adc1_hw_read_rate_i(&self) -> u32 {
        (self.bits & Sar1HwWakeup::ADC1_HW_READ_RATE_I_MASK)
            >> Sar1HwWakeup::ADC1_HW_READ_RATE_I_OFFSET
    }

    /// Read **ADC1_HW_READ_EN_I** `[0:0]`
    ///
    /// Enable hardware automatic sampling.
    #[inline(always)]
    pub const fn adc1_hw_read_en_i(&self) -> bool {
        (self.bits & Sar1HwWakeup::ADC1_HW_READ_EN_I_MASK) != 0
    }

    /// Set **ADC1_HW_READ_RATE_I** `[16:1]`
    ///
    /// Hardware automatic sampling rate.
    #[inline(always)]
    pub const fn with_adc1_hw_read_rate_i(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sar1HwWakeup::ADC1_HW_READ_RATE_I_MASK)
                | ((value << Sar1HwWakeup::ADC1_HW_READ_RATE_I_OFFSET)
                    & Sar1HwWakeup::ADC1_HW_READ_RATE_I_MASK),
        }
    }

    /// Set **ADC1_HW_READ_EN_I** `[0:0]`
    ///
    /// Enable hardware automatic sampling.
    #[inline(always)]
    pub const fn with_adc1_hw_read_en_i(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sar1HwWakeup::ADC1_HW_READ_EN_I_MASK
            } else {
                self.bits & !Sar1HwWakeup::ADC1_HW_READ_EN_I_MASK
            },
        }
    }
}

impl Default for Sar1HwWakeupVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sar1HwWakeup {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sar1HwWakeupVal {
        Sar1HwWakeupVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sar1HwWakeupVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sar1HwWakeupVal) -> Sar1HwWakeupVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SAR2_HW_WAKEUP Register
///
/// Hardware automatic sampling registers for wakeup function.
///
/// Address offset: 0x0070
/// Reset value: 0x000000c8
///
/// ## Fields
///
/// - **ADC2_HW_READ_RATE_I** `[16:1]` (RW)
///
///   Hardware automatic sampling rate.
///
/// - **ADC2_HW_READ_EN_I** `[0]` (RW)
///
///   Enable hardware automatic sampling.
#[repr(transparent)]
pub struct Sar2HwWakeup {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sar2HwWakeup {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sar2HwWakeup {
    // ======== Field Constants ========

    /// **ADC2_HW_READ_RATE_I**
    ///
    /// Hardware automatic sampling rate.
    ///
    /// Bits `[16:1]`, mask `0x0001fffe`
    pub const ADC2_HW_READ_RATE_I_OFFSET: u32 = 1;
    pub const ADC2_HW_READ_RATE_I_MASK: u32 = 0x0001fffe;

    /// **ADC2_HW_READ_EN_I**
    ///
    /// Enable hardware automatic sampling.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ADC2_HW_READ_EN_I_OFFSET: u32 = 0;
    pub const ADC2_HW_READ_EN_I_MASK: u32 = 0x00000001;
    pub const ADC2_HW_READ_EN_I: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ADC2_HW_READ_RATE_I** `[16:1]` (RW)
    ///
    /// Hardware automatic sampling rate.
    #[inline(always)]
    pub fn adc2_hw_read_rate_i(&self) -> u32 {
        (self.value.read_bits() & Self::ADC2_HW_READ_RATE_I_MASK)
            >> Self::ADC2_HW_READ_RATE_I_OFFSET
    }

    /// Write **ADC2_HW_READ_RATE_I** `[16:1]` (RW)
    ///
    /// Hardware automatic sampling rate.
    #[inline(always)]
    pub fn write_adc2_hw_read_rate_i(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADC2_HW_READ_RATE_I_MASK)
                | ((value << Self::ADC2_HW_READ_RATE_I_OFFSET) & Self::ADC2_HW_READ_RATE_I_MASK)
        });
    }

    /// **ADC2_HW_READ_EN_I** `[0]` (RW)
    ///
    /// Enable hardware automatic sampling.
    #[inline(always)]
    pub fn adc2_hw_read_en_i(&self) -> bool {
        (self.value.read_bits() & Self::ADC2_HW_READ_EN_I_MASK) != 0
    }

    /// Set ADC2_HW_READ_EN_I bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_adc2_hw_read_en_i(&self) {
        self.value.modify_bits(|r| r | Self::ADC2_HW_READ_EN_I_MASK);
    }

    /// Clear ADC2_HW_READ_EN_I bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_adc2_hw_read_en_i(&self) {
        self.value
            .modify_bits(|r| r & !Self::ADC2_HW_READ_EN_I_MASK);
    }
}

/// SAR2_HW_WAKEUP value type for batch read/modify/write operations
///
/// Use `sar2hwwakeup.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sar2hwwakeup.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sar2HwWakeupVal {
    bits: u32,
}

impl Sar2HwWakeupVal {
    /// Create from raw bits
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
        Self { bits: 0x000000c8 }
    }

    /// Read **ADC2_HW_READ_RATE_I** `[16:1]`
    ///
    /// Hardware automatic sampling rate.
    #[inline(always)]
    pub const fn adc2_hw_read_rate_i(&self) -> u32 {
        (self.bits & Sar2HwWakeup::ADC2_HW_READ_RATE_I_MASK)
            >> Sar2HwWakeup::ADC2_HW_READ_RATE_I_OFFSET
    }

    /// Read **ADC2_HW_READ_EN_I** `[0:0]`
    ///
    /// Enable hardware automatic sampling.
    #[inline(always)]
    pub const fn adc2_hw_read_en_i(&self) -> bool {
        (self.bits & Sar2HwWakeup::ADC2_HW_READ_EN_I_MASK) != 0
    }

    /// Set **ADC2_HW_READ_RATE_I** `[16:1]`
    ///
    /// Hardware automatic sampling rate.
    #[inline(always)]
    pub const fn with_adc2_hw_read_rate_i(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sar2HwWakeup::ADC2_HW_READ_RATE_I_MASK)
                | ((value << Sar2HwWakeup::ADC2_HW_READ_RATE_I_OFFSET)
                    & Sar2HwWakeup::ADC2_HW_READ_RATE_I_MASK),
        }
    }

    /// Set **ADC2_HW_READ_EN_I** `[0:0]`
    ///
    /// Enable hardware automatic sampling.
    #[inline(always)]
    pub const fn with_adc2_hw_read_en_i(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Sar2HwWakeup::ADC2_HW_READ_EN_I_MASK
            } else {
                self.bits & !Sar2HwWakeup::ADC2_HW_READ_EN_I_MASK
            },
        }
    }
}

impl Default for Sar2HwWakeupVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sar2HwWakeup {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sar2HwWakeupVal {
        Sar2HwWakeupVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sar2HwWakeupVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sar2HwWakeupVal) -> Sar2HwWakeupVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RND_ECO_LOW Register
///
/// N/A
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **RND_ECO_LOW** `[31:0]` (RW)
///
///   N/A
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
    /// N/A
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RND_ECO_LOW_OFFSET: u32 = 0;
    pub const RND_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RND_ECO_LOW** `[31:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn rnd_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::RND_ECO_LOW_MASK) >> Self::RND_ECO_LOW_OFFSET
    }

    /// Write **RND_ECO_LOW** `[31:0]` (RW)
    ///
    /// N/A
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
    /// N/A
    #[inline(always)]
    pub const fn rnd_eco_low(&self) -> u32 {
        (self.bits & RndEcoLow::RND_ECO_LOW_MASK) >> RndEcoLow::RND_ECO_LOW_OFFSET
    }

    /// Set **RND_ECO_LOW** `[31:0]`
    ///
    /// N/A
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
/// N/A
///
/// Address offset: 0x0078
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **RND_ECO_HIGH** `[31:0]` (RW)
///
///   N/A
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
    /// N/A
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RND_ECO_HIGH_OFFSET: u32 = 0;
    pub const RND_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RND_ECO_HIGH** `[31:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn rnd_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::RND_ECO_HIGH_MASK) >> Self::RND_ECO_HIGH_OFFSET
    }

    /// Write **RND_ECO_HIGH** `[31:0]` (RW)
    ///
    /// N/A
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
    /// N/A
    #[inline(always)]
    pub const fn rnd_eco_high(&self) -> u32 {
        (self.bits & RndEcoHigh::RND_ECO_HIGH_MASK) >> RndEcoHigh::RND_ECO_HIGH_OFFSET
    }

    /// Set **RND_ECO_HIGH** `[31:0]`
    ///
    /// N/A
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
/// N/A
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **RND_ECO_RESULT** `[1]` (RO)
///
///   N/A
///
/// - **RND_ECO_EN** `[0]` (RW)
///
///   N/A
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
    /// N/A
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RND_ECO_RESULT_OFFSET: u32 = 1;
    pub const RND_ECO_RESULT_MASK: u32 = 0x00000002;
    pub const RND_ECO_RESULT: u32 = 1 << 1;

    /// **RND_ECO_EN**
    ///
    /// N/A
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RND_ECO_EN_OFFSET: u32 = 0;
    pub const RND_ECO_EN_MASK: u32 = 0x00000001;
    pub const RND_ECO_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RND_ECO_RESULT** `[1]` (RO)
    ///
    /// N/A
    #[inline(always)]
    pub fn rnd_eco_result(&self) -> bool {
        (self.value.read_bits() & Self::RND_ECO_RESULT_MASK) != 0
    }

    /// **RND_ECO_EN** `[0]` (RW)
    ///
    /// N/A
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
    /// N/A
    #[inline(always)]
    pub const fn rnd_eco_result(&self) -> bool {
        (self.bits & RndEcoCs::RND_ECO_RESULT_MASK) != 0
    }

    /// Read **RND_ECO_EN** `[0:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn rnd_eco_en(&self) -> bool {
        (self.bits & RndEcoCs::RND_ECO_EN_MASK) != 0
    }

    /// Set **RND_ECO_EN** `[0:0]`
    ///
    /// N/A
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

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Control the read operation of ADC1.
    pub reader1_ctrl: Reader1Ctrl,
    /// 0x0004
    /// N/A
    pub reader1_status: Reader1Status,
    /// 0x0008
    /// N/A
    pub meas1_ctrl1: Meas1Ctrl1,
    /// 0x000c
    /// ADC1 configuration registers.
    pub meas1_ctrl2: Meas1Ctrl2,
    /// 0x0010
    /// SAR ADC1 MUX register.
    pub meas1_mux: Meas1Mux,
    /// 0x0014
    /// ADC1 attenuation registers.
    pub atten1: Atten1,
    /// 0x0018
    /// N/A
    pub amp_ctrl1: AmpCtrl1,
    /// 0x001c
    /// N/A
    pub amp_ctrl2: AmpCtrl2,
    /// 0x0020
    /// N/A
    pub amp_ctrl3: AmpCtrl3,
    /// 0x0024
    /// Control the read operation of ADC2.
    pub reader2_ctrl: Reader2Ctrl,
    /// 0x0028
    /// N/A
    pub reader2_status: Reader2Status,
    /// 0x002c
    /// ADC2 configuration registers.
    pub meas2_ctrl1: Meas2Ctrl1,
    /// 0x0030
    /// ADC2 configuration registers.
    pub meas2_ctrl2: Meas2Ctrl2,
    /// 0x0034
    /// SAR ADC2 MUX register.
    pub meas2_mux: Meas2Mux,
    /// 0x0038
    /// ADC1 attenuation registers.
    pub atten2: Atten2,
    /// 0x003c
    /// In sleep, force to use rtc to control ADC
    pub force_wpd_sar: ForceWpdSar,
    /// 0x0040
    /// N/A
    pub meas_status: MeasStatus,
    /// 0x0044
    /// N/A
    pub reg_clken: RegClken,
    /// 0x0048
    /// Interrupt raw registers.
    pub cocpu_int_raw: CocpuIntRaw,
    /// 0x004c
    /// Interrupt enable registers.
    pub int_ena: IntEna,
    /// 0x0050
    /// Interrupt status registers.
    pub int_st: IntSt,
    /// 0x0054
    /// Interrupt clear registers.
    pub int_clr: IntClr,
    /// 0x0058
    /// Interrupt enable assert registers.
    pub int_ena_w1ts: IntEnaW1ts,
    /// 0x005c
    /// Interrupt enable deassert registers.
    pub int_ena_w1tc: IntEnaW1tc,
    /// 0x0060
    /// ADC1 wakeup configuration registers.
    pub wakeup1: Wakeup1,
    /// 0x0064
    /// ADC2 wakeup configuration registers.
    pub wakeup2: Wakeup2,
    /// 0x0068
    /// Wakeup source select register.
    pub wakeup_sel: WakeupSel,
    /// 0x006c
    /// Hardware automatic sampling registers for wakeup function.
    pub sar1_hw_wakeup: Sar1HwWakeup,
    /// 0x0070
    /// Hardware automatic sampling registers for wakeup function.
    pub sar2_hw_wakeup: Sar2HwWakeup,
    /// 0x0074
    /// N/A
    pub rnd_eco_low: RndEcoLow,
    /// 0x0078
    /// N/A
    pub rnd_eco_high: RndEcoHigh,
    /// 0x007c
    /// N/A
    pub rnd_eco_cs: RndEcoCs,
}
