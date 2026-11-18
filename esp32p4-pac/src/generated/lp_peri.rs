//! LP_PERI
//! LP_PERI Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x50120000;

/// CLK_EN Register
///
/// need_des
///
/// Address offset: 0x0000
/// Reset value: 0x7fff0000
///
/// ## Fields
///
/// - **CK_EN_LP_CORE** `[31]` (RW)
///
///   write 1 to force on lp_core clk
///
/// - **CK_EN_LP_INTR** `[30]` (RW)
///
///   need_des
///
/// - **CK_EN_LP_UART** `[29]` (RW)
///
///   need_des
///
/// - **CK_EN_LP_I2C** `[28]` (RW)
///
///   need_des
///
/// - **CK_EN_LP_I2CMST** `[27]` (RW)
///
///   need_des
///
/// - **CK_EN_LP_I2S** `[26]` (RW)
///
///   need_des
///
/// - **CK_EN_LP_I2S_RX** `[25]` (RW)
///
///   need_des
///
/// - **CK_EN_LP_I2S_TX** `[24]` (RW)
///
///   need_des
///
/// - **CK_EN_LP_ADC** `[23]` (RW)
///
///   need_des
///
/// - **CK_EN_LP_SPI** `[22]` (RW)
///
///   need_des
///
/// - **CK_EN_LP_TOUCH** `[21]` (RW)
///
///   need_des
///
/// - **CK_EN_LP_IOMUX** `[20]` (RW)
///
///   need_des
///
/// - **CK_EN_LP_EFUSE** `[19]` (RW)
///
///   need_des
///
/// - **CK_EN_LP_PMS** `[18]` (RW)
///
///   need_des
///
/// - **CK_EN_LP_TSENS** `[17]` (RW)
///
///   need_des
///
/// - **CK_EN_RNG** `[16]` (RW)
///
///   need_des
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

    /// **CK_EN_LP_CORE**
    ///
    /// write 1 to force on lp_core clk
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CK_EN_LP_CORE_OFFSET: u32 = 31;
    pub const CK_EN_LP_CORE_MASK: u32 = 0x80000000;
    pub const CK_EN_LP_CORE: u32 = 1 << 31;

    /// **CK_EN_LP_INTR**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const CK_EN_LP_INTR_OFFSET: u32 = 30;
    pub const CK_EN_LP_INTR_MASK: u32 = 0x40000000;
    pub const CK_EN_LP_INTR: u32 = 1 << 30;

    /// **CK_EN_LP_UART**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const CK_EN_LP_UART_OFFSET: u32 = 29;
    pub const CK_EN_LP_UART_MASK: u32 = 0x20000000;
    pub const CK_EN_LP_UART: u32 = 1 << 29;

    /// **CK_EN_LP_I2C**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const CK_EN_LP_I2C_OFFSET: u32 = 28;
    pub const CK_EN_LP_I2C_MASK: u32 = 0x10000000;
    pub const CK_EN_LP_I2C: u32 = 1 << 28;

    /// **CK_EN_LP_I2CMST**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const CK_EN_LP_I2CMST_OFFSET: u32 = 27;
    pub const CK_EN_LP_I2CMST_MASK: u32 = 0x08000000;
    pub const CK_EN_LP_I2CMST: u32 = 1 << 27;

    /// **CK_EN_LP_I2S**
    ///
    /// need_des
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const CK_EN_LP_I2S_OFFSET: u32 = 26;
    pub const CK_EN_LP_I2S_MASK: u32 = 0x04000000;
    pub const CK_EN_LP_I2S: u32 = 1 << 26;

    /// **CK_EN_LP_I2S_RX**
    ///
    /// need_des
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const CK_EN_LP_I2S_RX_OFFSET: u32 = 25;
    pub const CK_EN_LP_I2S_RX_MASK: u32 = 0x02000000;
    pub const CK_EN_LP_I2S_RX: u32 = 1 << 25;

    /// **CK_EN_LP_I2S_TX**
    ///
    /// need_des
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const CK_EN_LP_I2S_TX_OFFSET: u32 = 24;
    pub const CK_EN_LP_I2S_TX_MASK: u32 = 0x01000000;
    pub const CK_EN_LP_I2S_TX: u32 = 1 << 24;

    /// **CK_EN_LP_ADC**
    ///
    /// need_des
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const CK_EN_LP_ADC_OFFSET: u32 = 23;
    pub const CK_EN_LP_ADC_MASK: u32 = 0x00800000;
    pub const CK_EN_LP_ADC: u32 = 1 << 23;

    /// **CK_EN_LP_SPI**
    ///
    /// need_des
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const CK_EN_LP_SPI_OFFSET: u32 = 22;
    pub const CK_EN_LP_SPI_MASK: u32 = 0x00400000;
    pub const CK_EN_LP_SPI: u32 = 1 << 22;

    /// **CK_EN_LP_TOUCH**
    ///
    /// need_des
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const CK_EN_LP_TOUCH_OFFSET: u32 = 21;
    pub const CK_EN_LP_TOUCH_MASK: u32 = 0x00200000;
    pub const CK_EN_LP_TOUCH: u32 = 1 << 21;

    /// **CK_EN_LP_IOMUX**
    ///
    /// need_des
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const CK_EN_LP_IOMUX_OFFSET: u32 = 20;
    pub const CK_EN_LP_IOMUX_MASK: u32 = 0x00100000;
    pub const CK_EN_LP_IOMUX: u32 = 1 << 20;

    /// **CK_EN_LP_EFUSE**
    ///
    /// need_des
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CK_EN_LP_EFUSE_OFFSET: u32 = 19;
    pub const CK_EN_LP_EFUSE_MASK: u32 = 0x00080000;
    pub const CK_EN_LP_EFUSE: u32 = 1 << 19;

    /// **CK_EN_LP_PMS**
    ///
    /// need_des
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const CK_EN_LP_PMS_OFFSET: u32 = 18;
    pub const CK_EN_LP_PMS_MASK: u32 = 0x00040000;
    pub const CK_EN_LP_PMS: u32 = 1 << 18;

    /// **CK_EN_LP_TSENS**
    ///
    /// need_des
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const CK_EN_LP_TSENS_OFFSET: u32 = 17;
    pub const CK_EN_LP_TSENS_MASK: u32 = 0x00020000;
    pub const CK_EN_LP_TSENS: u32 = 1 << 17;

    /// **CK_EN_RNG**
    ///
    /// need_des
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const CK_EN_RNG_OFFSET: u32 = 16;
    pub const CK_EN_RNG_MASK: u32 = 0x00010000;
    pub const CK_EN_RNG: u32 = 1 << 16;

    // ======== Field Accessors ========

    /// **CK_EN_LP_CORE** `[31]` (RW)
    ///
    /// write 1 to force on lp_core clk
    #[inline(always)]
    pub fn ck_en_lp_core(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_CORE_MASK) != 0
    }

    /// Set CK_EN_LP_CORE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_core(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_CORE_MASK);
    }

    /// Clear CK_EN_LP_CORE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_core(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_CORE_MASK);
    }

    /// **CK_EN_LP_INTR** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_lp_intr(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_INTR_MASK) != 0
    }

    /// Set CK_EN_LP_INTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_intr(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_INTR_MASK);
    }

    /// Clear CK_EN_LP_INTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_intr(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_INTR_MASK);
    }

    /// **CK_EN_LP_UART** `[29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_lp_uart(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_UART_MASK) != 0
    }

    /// Set CK_EN_LP_UART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_uart(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_UART_MASK);
    }

    /// Clear CK_EN_LP_UART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_uart(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_UART_MASK);
    }

    /// **CK_EN_LP_I2C** `[28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_lp_i2c(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_I2C_MASK) != 0
    }

    /// Set CK_EN_LP_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_i2c(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_I2C_MASK);
    }

    /// Clear CK_EN_LP_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_i2c(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_I2C_MASK);
    }

    /// **CK_EN_LP_I2CMST** `[27]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_lp_i2cmst(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_I2CMST_MASK) != 0
    }

    /// Set CK_EN_LP_I2CMST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_i2cmst(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_I2CMST_MASK);
    }

    /// Clear CK_EN_LP_I2CMST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_i2cmst(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_I2CMST_MASK);
    }

    /// **CK_EN_LP_I2S** `[26]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_lp_i2s(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_I2S_MASK) != 0
    }

    /// Set CK_EN_LP_I2S bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_i2s(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_I2S_MASK);
    }

    /// Clear CK_EN_LP_I2S bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_i2s(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_I2S_MASK);
    }

    /// **CK_EN_LP_I2S_RX** `[25]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_lp_i2s_rx(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_I2S_RX_MASK) != 0
    }

    /// Set CK_EN_LP_I2S_RX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_i2s_rx(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_I2S_RX_MASK);
    }

    /// Clear CK_EN_LP_I2S_RX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_i2s_rx(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_I2S_RX_MASK);
    }

    /// **CK_EN_LP_I2S_TX** `[24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_lp_i2s_tx(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_I2S_TX_MASK) != 0
    }

    /// Set CK_EN_LP_I2S_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_i2s_tx(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_I2S_TX_MASK);
    }

    /// Clear CK_EN_LP_I2S_TX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_i2s_tx(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_I2S_TX_MASK);
    }

    /// **CK_EN_LP_ADC** `[23]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_lp_adc(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_ADC_MASK) != 0
    }

    /// Set CK_EN_LP_ADC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_adc(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_ADC_MASK);
    }

    /// Clear CK_EN_LP_ADC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_adc(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_ADC_MASK);
    }

    /// **CK_EN_LP_SPI** `[22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_lp_spi(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_SPI_MASK) != 0
    }

    /// Set CK_EN_LP_SPI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_spi(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_SPI_MASK);
    }

    /// Clear CK_EN_LP_SPI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_spi(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_SPI_MASK);
    }

    /// **CK_EN_LP_TOUCH** `[21]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_lp_touch(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_TOUCH_MASK) != 0
    }

    /// Set CK_EN_LP_TOUCH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_touch(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_TOUCH_MASK);
    }

    /// Clear CK_EN_LP_TOUCH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_touch(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_TOUCH_MASK);
    }

    /// **CK_EN_LP_IOMUX** `[20]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_lp_iomux(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_IOMUX_MASK) != 0
    }

    /// Set CK_EN_LP_IOMUX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_iomux(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_IOMUX_MASK);
    }

    /// Clear CK_EN_LP_IOMUX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_iomux(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_IOMUX_MASK);
    }

    /// **CK_EN_LP_EFUSE** `[19]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_lp_efuse(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_EFUSE_MASK) != 0
    }

    /// Set CK_EN_LP_EFUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_efuse(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_EFUSE_MASK);
    }

    /// Clear CK_EN_LP_EFUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_efuse(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_EFUSE_MASK);
    }

    /// **CK_EN_LP_PMS** `[18]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_lp_pms(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_PMS_MASK) != 0
    }

    /// Set CK_EN_LP_PMS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_pms(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_PMS_MASK);
    }

    /// Clear CK_EN_LP_PMS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_pms(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_PMS_MASK);
    }

    /// **CK_EN_LP_TSENS** `[17]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_lp_tsens(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_LP_TSENS_MASK) != 0
    }

    /// Set CK_EN_LP_TSENS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_lp_tsens(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_LP_TSENS_MASK);
    }

    /// Clear CK_EN_LP_TSENS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_lp_tsens(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_LP_TSENS_MASK);
    }

    /// **CK_EN_RNG** `[16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ck_en_rng(&self) -> bool {
        (self.value.read_bits() & Self::CK_EN_RNG_MASK) != 0
    }

    /// Set CK_EN_RNG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ck_en_rng(&self) {
        self.value.modify_bits(|r| r | Self::CK_EN_RNG_MASK);
    }

    /// Clear CK_EN_RNG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ck_en_rng(&self) {
        self.value.modify_bits(|r| r & !Self::CK_EN_RNG_MASK);
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

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x7fff0000 }
    }

    /// Read **CK_EN_LP_CORE** `[31:31]`
    ///
    /// write 1 to force on lp_core clk
    #[inline(always)]
    pub const fn ck_en_lp_core(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_CORE_MASK) != 0
    }

    /// Read **CK_EN_LP_INTR** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_lp_intr(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_INTR_MASK) != 0
    }

    /// Read **CK_EN_LP_UART** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_lp_uart(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_UART_MASK) != 0
    }

    /// Read **CK_EN_LP_I2C** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_lp_i2c(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_I2C_MASK) != 0
    }

    /// Read **CK_EN_LP_I2CMST** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_lp_i2cmst(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_I2CMST_MASK) != 0
    }

    /// Read **CK_EN_LP_I2S** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_lp_i2s(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_I2S_MASK) != 0
    }

    /// Read **CK_EN_LP_I2S_RX** `[25:25]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_lp_i2s_rx(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_I2S_RX_MASK) != 0
    }

    /// Read **CK_EN_LP_I2S_TX** `[24:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_lp_i2s_tx(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_I2S_TX_MASK) != 0
    }

    /// Read **CK_EN_LP_ADC** `[23:23]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_lp_adc(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_ADC_MASK) != 0
    }

    /// Read **CK_EN_LP_SPI** `[22:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_lp_spi(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_SPI_MASK) != 0
    }

    /// Read **CK_EN_LP_TOUCH** `[21:21]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_lp_touch(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_TOUCH_MASK) != 0
    }

    /// Read **CK_EN_LP_IOMUX** `[20:20]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_lp_iomux(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_IOMUX_MASK) != 0
    }

    /// Read **CK_EN_LP_EFUSE** `[19:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_lp_efuse(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_EFUSE_MASK) != 0
    }

    /// Read **CK_EN_LP_PMS** `[18:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_lp_pms(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_PMS_MASK) != 0
    }

    /// Read **CK_EN_LP_TSENS** `[17:17]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_lp_tsens(&self) -> bool {
        (self.bits & ClkEn::CK_EN_LP_TSENS_MASK) != 0
    }

    /// Read **CK_EN_RNG** `[16:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ck_en_rng(&self) -> bool {
        (self.bits & ClkEn::CK_EN_RNG_MASK) != 0
    }

    /// Set **CK_EN_LP_CORE** `[31:31]`
    ///
    /// write 1 to force on lp_core clk
    #[inline(always)]
    pub const fn with_ck_en_lp_core(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_CORE_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_CORE_MASK
            },
        }
    }

    /// Set **CK_EN_LP_INTR** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_lp_intr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_INTR_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_INTR_MASK
            },
        }
    }

    /// Set **CK_EN_LP_UART** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_lp_uart(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_UART_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_UART_MASK
            },
        }
    }

    /// Set **CK_EN_LP_I2C** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_lp_i2c(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_I2C_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_I2C_MASK
            },
        }
    }

    /// Set **CK_EN_LP_I2CMST** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_lp_i2cmst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_I2CMST_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_I2CMST_MASK
            },
        }
    }

    /// Set **CK_EN_LP_I2S** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_lp_i2s(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_I2S_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_I2S_MASK
            },
        }
    }

    /// Set **CK_EN_LP_I2S_RX** `[25:25]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_lp_i2s_rx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_I2S_RX_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_I2S_RX_MASK
            },
        }
    }

    /// Set **CK_EN_LP_I2S_TX** `[24:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_lp_i2s_tx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_I2S_TX_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_I2S_TX_MASK
            },
        }
    }

    /// Set **CK_EN_LP_ADC** `[23:23]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_lp_adc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_ADC_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_ADC_MASK
            },
        }
    }

    /// Set **CK_EN_LP_SPI** `[22:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_lp_spi(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_SPI_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_SPI_MASK
            },
        }
    }

    /// Set **CK_EN_LP_TOUCH** `[21:21]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_lp_touch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_TOUCH_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_TOUCH_MASK
            },
        }
    }

    /// Set **CK_EN_LP_IOMUX** `[20:20]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_lp_iomux(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_IOMUX_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_IOMUX_MASK
            },
        }
    }

    /// Set **CK_EN_LP_EFUSE** `[19:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_lp_efuse(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_EFUSE_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_EFUSE_MASK
            },
        }
    }

    /// Set **CK_EN_LP_PMS** `[18:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_lp_pms(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_PMS_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_PMS_MASK
            },
        }
    }

    /// Set **CK_EN_LP_TSENS** `[17:17]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_lp_tsens(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_LP_TSENS_MASK
            } else {
                self.bits & !ClkEn::CK_EN_LP_TSENS_MASK
            },
        }
    }

    /// Set **CK_EN_RNG** `[16:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ck_en_rng(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::CK_EN_RNG_MASK
            } else {
                self.bits & !ClkEn::CK_EN_RNG_MASK
            },
        }
    }
}

impl Default for ClkEnVal {
    fn default() -> Self {
        Self::reset_val()
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

/// CORE_CLK_SEL Register
///
/// need_des
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **LP_UART_CLK_SEL** `[31:30]` (RW)
///
///   need_des
///
/// - **LP_I2C_CLK_SEL** `[29:28]` (RW)
///
///   need_des
///
/// - **LP_I2S_RX_CLK_SEL** `[27:26]` (RW)
///
///   need_des
///
/// - **LP_I2S_TX_CLK_SEL** `[25:24]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct CoreClkSel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CoreClkSel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoreClkSel {
    // ======== Field Constants ========

    /// **LP_UART_CLK_SEL**
    ///
    /// need_des
    ///
    /// Bits `[31:30]`, mask `0xc0000000`
    pub const LP_UART_CLK_SEL_OFFSET: u32 = 30;
    pub const LP_UART_CLK_SEL_MASK: u32 = 0xc0000000;

    /// **LP_I2C_CLK_SEL**
    ///
    /// need_des
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const LP_I2C_CLK_SEL_OFFSET: u32 = 28;
    pub const LP_I2C_CLK_SEL_MASK: u32 = 0x30000000;

    /// **LP_I2S_RX_CLK_SEL**
    ///
    /// need_des
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const LP_I2S_RX_CLK_SEL_OFFSET: u32 = 26;
    pub const LP_I2S_RX_CLK_SEL_MASK: u32 = 0x0c000000;

    /// **LP_I2S_TX_CLK_SEL**
    ///
    /// need_des
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const LP_I2S_TX_CLK_SEL_OFFSET: u32 = 24;
    pub const LP_I2S_TX_CLK_SEL_MASK: u32 = 0x03000000;

    // ======== Field Accessors ========

    /// **LP_UART_CLK_SEL** `[31:30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_uart_clk_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LP_UART_CLK_SEL_MASK) >> Self::LP_UART_CLK_SEL_OFFSET
    }

    /// Write **LP_UART_CLK_SEL** `[31:30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_uart_clk_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_UART_CLK_SEL_MASK)
                | ((value << Self::LP_UART_CLK_SEL_OFFSET) & Self::LP_UART_CLK_SEL_MASK)
        });
    }

    /// **LP_I2C_CLK_SEL** `[29:28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2c_clk_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LP_I2C_CLK_SEL_MASK) >> Self::LP_I2C_CLK_SEL_OFFSET
    }

    /// Write **LP_I2C_CLK_SEL** `[29:28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_i2c_clk_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_I2C_CLK_SEL_MASK)
                | ((value << Self::LP_I2C_CLK_SEL_OFFSET) & Self::LP_I2C_CLK_SEL_MASK)
        });
    }

    /// **LP_I2S_RX_CLK_SEL** `[27:26]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2s_rx_clk_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LP_I2S_RX_CLK_SEL_MASK) >> Self::LP_I2S_RX_CLK_SEL_OFFSET
    }

    /// Write **LP_I2S_RX_CLK_SEL** `[27:26]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_i2s_rx_clk_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_I2S_RX_CLK_SEL_MASK)
                | ((value << Self::LP_I2S_RX_CLK_SEL_OFFSET) & Self::LP_I2S_RX_CLK_SEL_MASK)
        });
    }

    /// **LP_I2S_TX_CLK_SEL** `[25:24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2s_tx_clk_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LP_I2S_TX_CLK_SEL_MASK) >> Self::LP_I2S_TX_CLK_SEL_OFFSET
    }

    /// Write **LP_I2S_TX_CLK_SEL** `[25:24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_i2s_tx_clk_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_I2S_TX_CLK_SEL_MASK)
                | ((value << Self::LP_I2S_TX_CLK_SEL_OFFSET) & Self::LP_I2S_TX_CLK_SEL_MASK)
        });
    }
}

/// CORE_CLK_SEL value type for batch read/modify/write operations
///
/// Use `coreclksel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `coreclksel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoreClkSelVal {
    bits: u32,
}

impl CoreClkSelVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_UART_CLK_SEL** `[31:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_uart_clk_sel(&self) -> u32 {
        (self.bits & CoreClkSel::LP_UART_CLK_SEL_MASK) >> CoreClkSel::LP_UART_CLK_SEL_OFFSET
    }

    /// Read **LP_I2C_CLK_SEL** `[29:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2c_clk_sel(&self) -> u32 {
        (self.bits & CoreClkSel::LP_I2C_CLK_SEL_MASK) >> CoreClkSel::LP_I2C_CLK_SEL_OFFSET
    }

    /// Read **LP_I2S_RX_CLK_SEL** `[27:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2s_rx_clk_sel(&self) -> u32 {
        (self.bits & CoreClkSel::LP_I2S_RX_CLK_SEL_MASK) >> CoreClkSel::LP_I2S_RX_CLK_SEL_OFFSET
    }

    /// Read **LP_I2S_TX_CLK_SEL** `[25:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2s_tx_clk_sel(&self) -> u32 {
        (self.bits & CoreClkSel::LP_I2S_TX_CLK_SEL_MASK) >> CoreClkSel::LP_I2S_TX_CLK_SEL_OFFSET
    }

    /// Set **LP_UART_CLK_SEL** `[31:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_uart_clk_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CoreClkSel::LP_UART_CLK_SEL_MASK)
                | ((value << CoreClkSel::LP_UART_CLK_SEL_OFFSET)
                    & CoreClkSel::LP_UART_CLK_SEL_MASK),
        }
    }

    /// Set **LP_I2C_CLK_SEL** `[29:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_i2c_clk_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CoreClkSel::LP_I2C_CLK_SEL_MASK)
                | ((value << CoreClkSel::LP_I2C_CLK_SEL_OFFSET) & CoreClkSel::LP_I2C_CLK_SEL_MASK),
        }
    }

    /// Set **LP_I2S_RX_CLK_SEL** `[27:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_i2s_rx_clk_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CoreClkSel::LP_I2S_RX_CLK_SEL_MASK)
                | ((value << CoreClkSel::LP_I2S_RX_CLK_SEL_OFFSET)
                    & CoreClkSel::LP_I2S_RX_CLK_SEL_MASK),
        }
    }

    /// Set **LP_I2S_TX_CLK_SEL** `[25:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_i2s_tx_clk_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CoreClkSel::LP_I2S_TX_CLK_SEL_MASK)
                | ((value << CoreClkSel::LP_I2S_TX_CLK_SEL_OFFSET)
                    & CoreClkSel::LP_I2S_TX_CLK_SEL_MASK),
        }
    }
}

impl Default for CoreClkSelVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CoreClkSel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CoreClkSelVal {
        CoreClkSelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CoreClkSelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CoreClkSelVal) -> CoreClkSelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RESET_EN Register
///
/// need_des
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **RST_EN_LP_CORE** `[31]` (WO)
///
///   need_des
///
/// - **RST_EN_LP_ROM** `[30]` (RW)
///
///   need_des
///
/// - **RST_EN_LP_INTR** `[29]` (RW)
///
///   need_des
///
/// - **RST_EN_LP_UART** `[28]` (RW)
///
///   need_des
///
/// - **RST_EN_LP_I2C** `[27]` (RW)
///
///   need_des
///
/// - **RST_EN_LP_I2CMST** `[26]` (RW)
///
///   need_des
///
/// - **RST_EN_LP_I2S** `[25]` (RW)
///
///   need_des
///
/// - **RST_EN_LP_ADC** `[24]` (RW)
///
///   need_des
///
/// - **RST_EN_LP_SPI** `[23]` (RW)
///
///   need_des
///
/// - **RST_EN_LP_TOUCH** `[22]` (RW)
///
///   need_des
///
/// - **RST_EN_LP_IOMUX** `[21]` (RW)
///
///   need_des
///
/// - **RST_EN_LP_EFUSE** `[20]` (RW)
///
///   need_des
///
/// - **RST_EN_LP_PMS** `[19]` (RW)
///
///   need_des
///
/// - **RST_EN_LP_TSENS** `[18]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct ResetEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ResetEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ResetEn {
    // ======== Field Constants ========

    /// **RST_EN_LP_CORE**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const RST_EN_LP_CORE_OFFSET: u32 = 31;
    pub const RST_EN_LP_CORE_MASK: u32 = 0x80000000;
    pub const RST_EN_LP_CORE: u32 = 1 << 31;

    /// **RST_EN_LP_ROM**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const RST_EN_LP_ROM_OFFSET: u32 = 30;
    pub const RST_EN_LP_ROM_MASK: u32 = 0x40000000;
    pub const RST_EN_LP_ROM: u32 = 1 << 30;

    /// **RST_EN_LP_INTR**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const RST_EN_LP_INTR_OFFSET: u32 = 29;
    pub const RST_EN_LP_INTR_MASK: u32 = 0x20000000;
    pub const RST_EN_LP_INTR: u32 = 1 << 29;

    /// **RST_EN_LP_UART**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const RST_EN_LP_UART_OFFSET: u32 = 28;
    pub const RST_EN_LP_UART_MASK: u32 = 0x10000000;
    pub const RST_EN_LP_UART: u32 = 1 << 28;

    /// **RST_EN_LP_I2C**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const RST_EN_LP_I2C_OFFSET: u32 = 27;
    pub const RST_EN_LP_I2C_MASK: u32 = 0x08000000;
    pub const RST_EN_LP_I2C: u32 = 1 << 27;

    /// **RST_EN_LP_I2CMST**
    ///
    /// need_des
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const RST_EN_LP_I2CMST_OFFSET: u32 = 26;
    pub const RST_EN_LP_I2CMST_MASK: u32 = 0x04000000;
    pub const RST_EN_LP_I2CMST: u32 = 1 << 26;

    /// **RST_EN_LP_I2S**
    ///
    /// need_des
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const RST_EN_LP_I2S_OFFSET: u32 = 25;
    pub const RST_EN_LP_I2S_MASK: u32 = 0x02000000;
    pub const RST_EN_LP_I2S: u32 = 1 << 25;

    /// **RST_EN_LP_ADC**
    ///
    /// need_des
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const RST_EN_LP_ADC_OFFSET: u32 = 24;
    pub const RST_EN_LP_ADC_MASK: u32 = 0x01000000;
    pub const RST_EN_LP_ADC: u32 = 1 << 24;

    /// **RST_EN_LP_SPI**
    ///
    /// need_des
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const RST_EN_LP_SPI_OFFSET: u32 = 23;
    pub const RST_EN_LP_SPI_MASK: u32 = 0x00800000;
    pub const RST_EN_LP_SPI: u32 = 1 << 23;

    /// **RST_EN_LP_TOUCH**
    ///
    /// need_des
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const RST_EN_LP_TOUCH_OFFSET: u32 = 22;
    pub const RST_EN_LP_TOUCH_MASK: u32 = 0x00400000;
    pub const RST_EN_LP_TOUCH: u32 = 1 << 22;

    /// **RST_EN_LP_IOMUX**
    ///
    /// need_des
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const RST_EN_LP_IOMUX_OFFSET: u32 = 21;
    pub const RST_EN_LP_IOMUX_MASK: u32 = 0x00200000;
    pub const RST_EN_LP_IOMUX: u32 = 1 << 21;

    /// **RST_EN_LP_EFUSE**
    ///
    /// need_des
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const RST_EN_LP_EFUSE_OFFSET: u32 = 20;
    pub const RST_EN_LP_EFUSE_MASK: u32 = 0x00100000;
    pub const RST_EN_LP_EFUSE: u32 = 1 << 20;

    /// **RST_EN_LP_PMS**
    ///
    /// need_des
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const RST_EN_LP_PMS_OFFSET: u32 = 19;
    pub const RST_EN_LP_PMS_MASK: u32 = 0x00080000;
    pub const RST_EN_LP_PMS: u32 = 1 << 19;

    /// **RST_EN_LP_TSENS**
    ///
    /// need_des
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const RST_EN_LP_TSENS_OFFSET: u32 = 18;
    pub const RST_EN_LP_TSENS_MASK: u32 = 0x00040000;
    pub const RST_EN_LP_TSENS: u32 = 1 << 18;

    // ======== Field Accessors ========

    /// Trigger **RST_EN_LP_CORE** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_rst_en_lp_core(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN_LP_CORE_MASK);
    }

    /// **RST_EN_LP_ROM** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rst_en_lp_rom(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN_LP_ROM_MASK) != 0
    }

    /// Set RST_EN_LP_ROM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en_lp_rom(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN_LP_ROM_MASK);
    }

    /// Clear RST_EN_LP_ROM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en_lp_rom(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN_LP_ROM_MASK);
    }

    /// **RST_EN_LP_INTR** `[29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rst_en_lp_intr(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN_LP_INTR_MASK) != 0
    }

    /// Set RST_EN_LP_INTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en_lp_intr(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN_LP_INTR_MASK);
    }

    /// Clear RST_EN_LP_INTR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en_lp_intr(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN_LP_INTR_MASK);
    }

    /// **RST_EN_LP_UART** `[28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rst_en_lp_uart(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN_LP_UART_MASK) != 0
    }

    /// Set RST_EN_LP_UART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en_lp_uart(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN_LP_UART_MASK);
    }

    /// Clear RST_EN_LP_UART bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en_lp_uart(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN_LP_UART_MASK);
    }

    /// **RST_EN_LP_I2C** `[27]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rst_en_lp_i2c(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN_LP_I2C_MASK) != 0
    }

    /// Set RST_EN_LP_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en_lp_i2c(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN_LP_I2C_MASK);
    }

    /// Clear RST_EN_LP_I2C bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en_lp_i2c(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN_LP_I2C_MASK);
    }

    /// **RST_EN_LP_I2CMST** `[26]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rst_en_lp_i2cmst(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN_LP_I2CMST_MASK) != 0
    }

    /// Set RST_EN_LP_I2CMST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en_lp_i2cmst(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN_LP_I2CMST_MASK);
    }

    /// Clear RST_EN_LP_I2CMST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en_lp_i2cmst(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN_LP_I2CMST_MASK);
    }

    /// **RST_EN_LP_I2S** `[25]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rst_en_lp_i2s(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN_LP_I2S_MASK) != 0
    }

    /// Set RST_EN_LP_I2S bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en_lp_i2s(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN_LP_I2S_MASK);
    }

    /// Clear RST_EN_LP_I2S bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en_lp_i2s(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN_LP_I2S_MASK);
    }

    /// **RST_EN_LP_ADC** `[24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rst_en_lp_adc(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN_LP_ADC_MASK) != 0
    }

    /// Set RST_EN_LP_ADC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en_lp_adc(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN_LP_ADC_MASK);
    }

    /// Clear RST_EN_LP_ADC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en_lp_adc(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN_LP_ADC_MASK);
    }

    /// **RST_EN_LP_SPI** `[23]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rst_en_lp_spi(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN_LP_SPI_MASK) != 0
    }

    /// Set RST_EN_LP_SPI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en_lp_spi(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN_LP_SPI_MASK);
    }

    /// Clear RST_EN_LP_SPI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en_lp_spi(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN_LP_SPI_MASK);
    }

    /// **RST_EN_LP_TOUCH** `[22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rst_en_lp_touch(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN_LP_TOUCH_MASK) != 0
    }

    /// Set RST_EN_LP_TOUCH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en_lp_touch(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN_LP_TOUCH_MASK);
    }

    /// Clear RST_EN_LP_TOUCH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en_lp_touch(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN_LP_TOUCH_MASK);
    }

    /// **RST_EN_LP_IOMUX** `[21]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rst_en_lp_iomux(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN_LP_IOMUX_MASK) != 0
    }

    /// Set RST_EN_LP_IOMUX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en_lp_iomux(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN_LP_IOMUX_MASK);
    }

    /// Clear RST_EN_LP_IOMUX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en_lp_iomux(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN_LP_IOMUX_MASK);
    }

    /// **RST_EN_LP_EFUSE** `[20]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rst_en_lp_efuse(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN_LP_EFUSE_MASK) != 0
    }

    /// Set RST_EN_LP_EFUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en_lp_efuse(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN_LP_EFUSE_MASK);
    }

    /// Clear RST_EN_LP_EFUSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en_lp_efuse(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN_LP_EFUSE_MASK);
    }

    /// **RST_EN_LP_PMS** `[19]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rst_en_lp_pms(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN_LP_PMS_MASK) != 0
    }

    /// Set RST_EN_LP_PMS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en_lp_pms(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN_LP_PMS_MASK);
    }

    /// Clear RST_EN_LP_PMS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en_lp_pms(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN_LP_PMS_MASK);
    }

    /// **RST_EN_LP_TSENS** `[18]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rst_en_lp_tsens(&self) -> bool {
        (self.value.read_bits() & Self::RST_EN_LP_TSENS_MASK) != 0
    }

    /// Set RST_EN_LP_TSENS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rst_en_lp_tsens(&self) {
        self.value.modify_bits(|r| r | Self::RST_EN_LP_TSENS_MASK);
    }

    /// Clear RST_EN_LP_TSENS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rst_en_lp_tsens(&self) {
        self.value.modify_bits(|r| r & !Self::RST_EN_LP_TSENS_MASK);
    }
}

/// RESET_EN value type for batch read/modify/write operations
///
/// Use `reseten.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `reseten.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ResetEnVal {
    bits: u32,
}

impl ResetEnVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RST_EN_LP_ROM** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rst_en_lp_rom(&self) -> bool {
        (self.bits & ResetEn::RST_EN_LP_ROM_MASK) != 0
    }

    /// Read **RST_EN_LP_INTR** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rst_en_lp_intr(&self) -> bool {
        (self.bits & ResetEn::RST_EN_LP_INTR_MASK) != 0
    }

    /// Read **RST_EN_LP_UART** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rst_en_lp_uart(&self) -> bool {
        (self.bits & ResetEn::RST_EN_LP_UART_MASK) != 0
    }

    /// Read **RST_EN_LP_I2C** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rst_en_lp_i2c(&self) -> bool {
        (self.bits & ResetEn::RST_EN_LP_I2C_MASK) != 0
    }

    /// Read **RST_EN_LP_I2CMST** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rst_en_lp_i2cmst(&self) -> bool {
        (self.bits & ResetEn::RST_EN_LP_I2CMST_MASK) != 0
    }

    /// Read **RST_EN_LP_I2S** `[25:25]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rst_en_lp_i2s(&self) -> bool {
        (self.bits & ResetEn::RST_EN_LP_I2S_MASK) != 0
    }

    /// Read **RST_EN_LP_ADC** `[24:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rst_en_lp_adc(&self) -> bool {
        (self.bits & ResetEn::RST_EN_LP_ADC_MASK) != 0
    }

    /// Read **RST_EN_LP_SPI** `[23:23]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rst_en_lp_spi(&self) -> bool {
        (self.bits & ResetEn::RST_EN_LP_SPI_MASK) != 0
    }

    /// Read **RST_EN_LP_TOUCH** `[22:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rst_en_lp_touch(&self) -> bool {
        (self.bits & ResetEn::RST_EN_LP_TOUCH_MASK) != 0
    }

    /// Read **RST_EN_LP_IOMUX** `[21:21]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rst_en_lp_iomux(&self) -> bool {
        (self.bits & ResetEn::RST_EN_LP_IOMUX_MASK) != 0
    }

    /// Read **RST_EN_LP_EFUSE** `[20:20]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rst_en_lp_efuse(&self) -> bool {
        (self.bits & ResetEn::RST_EN_LP_EFUSE_MASK) != 0
    }

    /// Read **RST_EN_LP_PMS** `[19:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rst_en_lp_pms(&self) -> bool {
        (self.bits & ResetEn::RST_EN_LP_PMS_MASK) != 0
    }

    /// Read **RST_EN_LP_TSENS** `[18:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rst_en_lp_tsens(&self) -> bool {
        (self.bits & ResetEn::RST_EN_LP_TSENS_MASK) != 0
    }

    /// Set **RST_EN_LP_CORE** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rst_en_lp_core(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetEn::RST_EN_LP_CORE_MASK
            } else {
                self.bits & !ResetEn::RST_EN_LP_CORE_MASK
            },
        }
    }

    /// Set **RST_EN_LP_ROM** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rst_en_lp_rom(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetEn::RST_EN_LP_ROM_MASK
            } else {
                self.bits & !ResetEn::RST_EN_LP_ROM_MASK
            },
        }
    }

    /// Set **RST_EN_LP_INTR** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rst_en_lp_intr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetEn::RST_EN_LP_INTR_MASK
            } else {
                self.bits & !ResetEn::RST_EN_LP_INTR_MASK
            },
        }
    }

    /// Set **RST_EN_LP_UART** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rst_en_lp_uart(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetEn::RST_EN_LP_UART_MASK
            } else {
                self.bits & !ResetEn::RST_EN_LP_UART_MASK
            },
        }
    }

    /// Set **RST_EN_LP_I2C** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rst_en_lp_i2c(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetEn::RST_EN_LP_I2C_MASK
            } else {
                self.bits & !ResetEn::RST_EN_LP_I2C_MASK
            },
        }
    }

    /// Set **RST_EN_LP_I2CMST** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rst_en_lp_i2cmst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetEn::RST_EN_LP_I2CMST_MASK
            } else {
                self.bits & !ResetEn::RST_EN_LP_I2CMST_MASK
            },
        }
    }

    /// Set **RST_EN_LP_I2S** `[25:25]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rst_en_lp_i2s(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetEn::RST_EN_LP_I2S_MASK
            } else {
                self.bits & !ResetEn::RST_EN_LP_I2S_MASK
            },
        }
    }

    /// Set **RST_EN_LP_ADC** `[24:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rst_en_lp_adc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetEn::RST_EN_LP_ADC_MASK
            } else {
                self.bits & !ResetEn::RST_EN_LP_ADC_MASK
            },
        }
    }

    /// Set **RST_EN_LP_SPI** `[23:23]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rst_en_lp_spi(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetEn::RST_EN_LP_SPI_MASK
            } else {
                self.bits & !ResetEn::RST_EN_LP_SPI_MASK
            },
        }
    }

    /// Set **RST_EN_LP_TOUCH** `[22:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rst_en_lp_touch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetEn::RST_EN_LP_TOUCH_MASK
            } else {
                self.bits & !ResetEn::RST_EN_LP_TOUCH_MASK
            },
        }
    }

    /// Set **RST_EN_LP_IOMUX** `[21:21]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rst_en_lp_iomux(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetEn::RST_EN_LP_IOMUX_MASK
            } else {
                self.bits & !ResetEn::RST_EN_LP_IOMUX_MASK
            },
        }
    }

    /// Set **RST_EN_LP_EFUSE** `[20:20]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rst_en_lp_efuse(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetEn::RST_EN_LP_EFUSE_MASK
            } else {
                self.bits & !ResetEn::RST_EN_LP_EFUSE_MASK
            },
        }
    }

    /// Set **RST_EN_LP_PMS** `[19:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rst_en_lp_pms(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetEn::RST_EN_LP_PMS_MASK
            } else {
                self.bits & !ResetEn::RST_EN_LP_PMS_MASK
            },
        }
    }

    /// Set **RST_EN_LP_TSENS** `[18:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rst_en_lp_tsens(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ResetEn::RST_EN_LP_TSENS_MASK
            } else {
                self.bits & !ResetEn::RST_EN_LP_TSENS_MASK
            },
        }
    }
}

impl Default for ResetEnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ResetEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ResetEnVal {
        ResetEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ResetEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ResetEnVal) -> ResetEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CPU Register
///
/// need_des
///
/// Address offset: 0x000c
/// Reset value: 0x80000000
///
/// ## Fields
///
/// - **LPCORE_DBGM_UNAVAILABLE** `[31]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Cpu {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cpu {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cpu {
    // ======== Field Constants ========

    /// **LPCORE_DBGM_UNAVAILABLE**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LPCORE_DBGM_UNAVAILABLE_OFFSET: u32 = 31;
    pub const LPCORE_DBGM_UNAVAILABLE_MASK: u32 = 0x80000000;
    pub const LPCORE_DBGM_UNAVAILABLE: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **LPCORE_DBGM_UNAVAILABLE** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lpcore_dbgm_unavailable(&self) -> bool {
        (self.value.read_bits() & Self::LPCORE_DBGM_UNAVAILABLE_MASK) != 0
    }

    /// Set LPCORE_DBGM_UNAVAILABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lpcore_dbgm_unavailable(&self) {
        self.value
            .modify_bits(|r| r | Self::LPCORE_DBGM_UNAVAILABLE_MASK);
    }

    /// Clear LPCORE_DBGM_UNAVAILABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lpcore_dbgm_unavailable(&self) {
        self.value
            .modify_bits(|r| r & !Self::LPCORE_DBGM_UNAVAILABLE_MASK);
    }
}

/// CPU value type for batch read/modify/write operations
///
/// Use `cpu.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cpu.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CpuVal {
    bits: u32,
}

impl CpuVal {
    /// Create from raw bits
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
        Self { bits: 0x80000000 }
    }

    /// Read **LPCORE_DBGM_UNAVAILABLE** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lpcore_dbgm_unavailable(&self) -> bool {
        (self.bits & Cpu::LPCORE_DBGM_UNAVAILABLE_MASK) != 0
    }

    /// Set **LPCORE_DBGM_UNAVAILABLE** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lpcore_dbgm_unavailable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cpu::LPCORE_DBGM_UNAVAILABLE_MASK
            } else {
                self.bits & !Cpu::LPCORE_DBGM_UNAVAILABLE_MASK
            },
        }
    }
}

impl Default for CpuVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Cpu {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CpuVal {
        CpuVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CpuVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CpuVal) -> CpuVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MEM_CTRL Register
///
/// need_des
///
/// Address offset: 0x0028
/// Reset value: 0x80000000
///
/// ## Fields
///
/// - **LP_UART_MEM_FORCE_PU** `[31]` (RW)
///
///   need_des
///
/// - **LP_UART_MEM_FORCE_PD** `[30]` (RW)
///
///   need_des
///
/// - **LP_UART_WAKEUP_EN** `[29]` (RW)
///
///   need_des
///
/// - **LP_UART_WAKEUP_FLAG** `[1]` (RW)
///
///   need_des
///
/// - **LP_UART_WAKEUP_FLAG_CLR** `[0]` (WO)
///
///   need_des
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

    /// **LP_UART_MEM_FORCE_PU**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_UART_MEM_FORCE_PU_OFFSET: u32 = 31;
    pub const LP_UART_MEM_FORCE_PU_MASK: u32 = 0x80000000;
    pub const LP_UART_MEM_FORCE_PU: u32 = 1 << 31;

    /// **LP_UART_MEM_FORCE_PD**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_UART_MEM_FORCE_PD_OFFSET: u32 = 30;
    pub const LP_UART_MEM_FORCE_PD_MASK: u32 = 0x40000000;
    pub const LP_UART_MEM_FORCE_PD: u32 = 1 << 30;

    /// **LP_UART_WAKEUP_EN**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_UART_WAKEUP_EN_OFFSET: u32 = 29;
    pub const LP_UART_WAKEUP_EN_MASK: u32 = 0x20000000;
    pub const LP_UART_WAKEUP_EN: u32 = 1 << 29;

    /// **LP_UART_WAKEUP_FLAG**
    ///
    /// need_des
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const LP_UART_WAKEUP_FLAG_OFFSET: u32 = 1;
    pub const LP_UART_WAKEUP_FLAG_MASK: u32 = 0x00000002;
    pub const LP_UART_WAKEUP_FLAG: u32 = 1 << 1;

    /// **LP_UART_WAKEUP_FLAG_CLR**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_UART_WAKEUP_FLAG_CLR_OFFSET: u32 = 0;
    pub const LP_UART_WAKEUP_FLAG_CLR_MASK: u32 = 0x00000001;
    pub const LP_UART_WAKEUP_FLAG_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LP_UART_MEM_FORCE_PU** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_uart_mem_force_pu(&self) -> bool {
        (self.value.read_bits() & Self::LP_UART_MEM_FORCE_PU_MASK) != 0
    }

    /// Set LP_UART_MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_uart_mem_force_pu(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_UART_MEM_FORCE_PU_MASK);
    }

    /// Clear LP_UART_MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_uart_mem_force_pu(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_UART_MEM_FORCE_PU_MASK);
    }

    /// **LP_UART_MEM_FORCE_PD** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_uart_mem_force_pd(&self) -> bool {
        (self.value.read_bits() & Self::LP_UART_MEM_FORCE_PD_MASK) != 0
    }

    /// Set LP_UART_MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_uart_mem_force_pd(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_UART_MEM_FORCE_PD_MASK);
    }

    /// Clear LP_UART_MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_uart_mem_force_pd(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_UART_MEM_FORCE_PD_MASK);
    }

    /// **LP_UART_WAKEUP_EN** `[29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_uart_wakeup_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_UART_WAKEUP_EN_MASK) != 0
    }

    /// Set LP_UART_WAKEUP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_uart_wakeup_en(&self) {
        self.value.modify_bits(|r| r | Self::LP_UART_WAKEUP_EN_MASK);
    }

    /// Clear LP_UART_WAKEUP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_uart_wakeup_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_UART_WAKEUP_EN_MASK);
    }

    /// **LP_UART_WAKEUP_FLAG** `[1]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_uart_wakeup_flag(&self) -> bool {
        (self.value.read_bits() & Self::LP_UART_WAKEUP_FLAG_MASK) != 0
    }

    /// Set LP_UART_WAKEUP_FLAG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_uart_wakeup_flag(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_UART_WAKEUP_FLAG_MASK);
    }

    /// Clear LP_UART_WAKEUP_FLAG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_uart_wakeup_flag(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_UART_WAKEUP_FLAG_MASK);
    }

    /// Trigger **LP_UART_WAKEUP_FLAG_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_uart_wakeup_flag_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_UART_WAKEUP_FLAG_CLR_MASK);
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
        Self { bits: 0x80000000 }
    }

    /// Read **LP_UART_MEM_FORCE_PU** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_uart_mem_force_pu(&self) -> bool {
        (self.bits & MemCtrl::LP_UART_MEM_FORCE_PU_MASK) != 0
    }

    /// Read **LP_UART_MEM_FORCE_PD** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_uart_mem_force_pd(&self) -> bool {
        (self.bits & MemCtrl::LP_UART_MEM_FORCE_PD_MASK) != 0
    }

    /// Read **LP_UART_WAKEUP_EN** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_uart_wakeup_en(&self) -> bool {
        (self.bits & MemCtrl::LP_UART_WAKEUP_EN_MASK) != 0
    }

    /// Read **LP_UART_WAKEUP_FLAG** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_uart_wakeup_flag(&self) -> bool {
        (self.bits & MemCtrl::LP_UART_WAKEUP_FLAG_MASK) != 0
    }

    /// Set **LP_UART_MEM_FORCE_PU** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_uart_mem_force_pu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MemCtrl::LP_UART_MEM_FORCE_PU_MASK
            } else {
                self.bits & !MemCtrl::LP_UART_MEM_FORCE_PU_MASK
            },
        }
    }

    /// Set **LP_UART_MEM_FORCE_PD** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_uart_mem_force_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MemCtrl::LP_UART_MEM_FORCE_PD_MASK
            } else {
                self.bits & !MemCtrl::LP_UART_MEM_FORCE_PD_MASK
            },
        }
    }

    /// Set **LP_UART_WAKEUP_EN** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_uart_wakeup_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MemCtrl::LP_UART_WAKEUP_EN_MASK
            } else {
                self.bits & !MemCtrl::LP_UART_WAKEUP_EN_MASK
            },
        }
    }

    /// Set **LP_UART_WAKEUP_FLAG** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_uart_wakeup_flag(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MemCtrl::LP_UART_WAKEUP_FLAG_MASK
            } else {
                self.bits & !MemCtrl::LP_UART_WAKEUP_FLAG_MASK
            },
        }
    }

    /// Set **LP_UART_WAKEUP_FLAG_CLR** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_uart_wakeup_flag_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MemCtrl::LP_UART_WAKEUP_FLAG_CLR_MASK
            } else {
                self.bits & !MemCtrl::LP_UART_WAKEUP_FLAG_CLR_MASK
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

/// ADC_CTRL Register
///
/// need_des
///
/// Address offset: 0x002c
/// Reset value: 0x04040400
///
/// ## Fields
///
/// - **LPADC_SAR1_DIV_NUM** `[31:24]` (RW)
///
///   need_des
///
/// - **LPADC_SAR2_DIV_NUM** `[23:16]` (RW)
///
///   need_des
///
/// - **LPADC_FUNC_DIV_NUM** `[15:8]` (RW)
///
///   need_des
///
/// - **SAR1_CLK_FORCE_ON** `[7]` (RW)
///
///   need_des
///
/// - **SAR2_CLK_FORCE_ON** `[6]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct AdcCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AdcCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AdcCtrl {
    // ======== Field Constants ========

    /// **LPADC_SAR1_DIV_NUM**
    ///
    /// need_des
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const LPADC_SAR1_DIV_NUM_OFFSET: u32 = 24;
    pub const LPADC_SAR1_DIV_NUM_MASK: u32 = 0xff000000;

    /// **LPADC_SAR2_DIV_NUM**
    ///
    /// need_des
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const LPADC_SAR2_DIV_NUM_OFFSET: u32 = 16;
    pub const LPADC_SAR2_DIV_NUM_MASK: u32 = 0x00ff0000;

    /// **LPADC_FUNC_DIV_NUM**
    ///
    /// need_des
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const LPADC_FUNC_DIV_NUM_OFFSET: u32 = 8;
    pub const LPADC_FUNC_DIV_NUM_MASK: u32 = 0x0000ff00;

    /// **SAR1_CLK_FORCE_ON**
    ///
    /// need_des
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SAR1_CLK_FORCE_ON_OFFSET: u32 = 7;
    pub const SAR1_CLK_FORCE_ON_MASK: u32 = 0x00000080;
    pub const SAR1_CLK_FORCE_ON: u32 = 1 << 7;

    /// **SAR2_CLK_FORCE_ON**
    ///
    /// need_des
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SAR2_CLK_FORCE_ON_OFFSET: u32 = 6;
    pub const SAR2_CLK_FORCE_ON_MASK: u32 = 0x00000040;
    pub const SAR2_CLK_FORCE_ON: u32 = 1 << 6;

    // ======== Field Accessors ========

    /// **LPADC_SAR1_DIV_NUM** `[31:24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lpadc_sar1_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::LPADC_SAR1_DIV_NUM_MASK) >> Self::LPADC_SAR1_DIV_NUM_OFFSET
    }

    /// Write **LPADC_SAR1_DIV_NUM** `[31:24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lpadc_sar1_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LPADC_SAR1_DIV_NUM_MASK)
                | ((value << Self::LPADC_SAR1_DIV_NUM_OFFSET) & Self::LPADC_SAR1_DIV_NUM_MASK)
        });
    }

    /// **LPADC_SAR2_DIV_NUM** `[23:16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lpadc_sar2_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::LPADC_SAR2_DIV_NUM_MASK) >> Self::LPADC_SAR2_DIV_NUM_OFFSET
    }

    /// Write **LPADC_SAR2_DIV_NUM** `[23:16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lpadc_sar2_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LPADC_SAR2_DIV_NUM_MASK)
                | ((value << Self::LPADC_SAR2_DIV_NUM_OFFSET) & Self::LPADC_SAR2_DIV_NUM_MASK)
        });
    }

    /// **LPADC_FUNC_DIV_NUM** `[15:8]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lpadc_func_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::LPADC_FUNC_DIV_NUM_MASK) >> Self::LPADC_FUNC_DIV_NUM_OFFSET
    }

    /// Write **LPADC_FUNC_DIV_NUM** `[15:8]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lpadc_func_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LPADC_FUNC_DIV_NUM_MASK)
                | ((value << Self::LPADC_FUNC_DIV_NUM_OFFSET) & Self::LPADC_FUNC_DIV_NUM_MASK)
        });
    }

    /// **SAR1_CLK_FORCE_ON** `[7]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn sar1_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::SAR1_CLK_FORCE_ON_MASK) != 0
    }

    /// Set SAR1_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar1_clk_force_on(&self) {
        self.value.modify_bits(|r| r | Self::SAR1_CLK_FORCE_ON_MASK);
    }

    /// Clear SAR1_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar1_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::SAR1_CLK_FORCE_ON_MASK);
    }

    /// **SAR2_CLK_FORCE_ON** `[6]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn sar2_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::SAR2_CLK_FORCE_ON_MASK) != 0
    }

    /// Set SAR2_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sar2_clk_force_on(&self) {
        self.value.modify_bits(|r| r | Self::SAR2_CLK_FORCE_ON_MASK);
    }

    /// Clear SAR2_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sar2_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::SAR2_CLK_FORCE_ON_MASK);
    }
}

/// ADC_CTRL value type for batch read/modify/write operations
///
/// Use `adcctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `adcctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AdcCtrlVal {
    bits: u32,
}

impl AdcCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x04040400 }
    }

    /// Read **LPADC_SAR1_DIV_NUM** `[31:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lpadc_sar1_div_num(&self) -> u32 {
        (self.bits & AdcCtrl::LPADC_SAR1_DIV_NUM_MASK) >> AdcCtrl::LPADC_SAR1_DIV_NUM_OFFSET
    }

    /// Read **LPADC_SAR2_DIV_NUM** `[23:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lpadc_sar2_div_num(&self) -> u32 {
        (self.bits & AdcCtrl::LPADC_SAR2_DIV_NUM_MASK) >> AdcCtrl::LPADC_SAR2_DIV_NUM_OFFSET
    }

    /// Read **LPADC_FUNC_DIV_NUM** `[15:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lpadc_func_div_num(&self) -> u32 {
        (self.bits & AdcCtrl::LPADC_FUNC_DIV_NUM_MASK) >> AdcCtrl::LPADC_FUNC_DIV_NUM_OFFSET
    }

    /// Read **SAR1_CLK_FORCE_ON** `[7:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn sar1_clk_force_on(&self) -> bool {
        (self.bits & AdcCtrl::SAR1_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **SAR2_CLK_FORCE_ON** `[6:6]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn sar2_clk_force_on(&self) -> bool {
        (self.bits & AdcCtrl::SAR2_CLK_FORCE_ON_MASK) != 0
    }

    /// Set **LPADC_SAR1_DIV_NUM** `[31:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lpadc_sar1_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AdcCtrl::LPADC_SAR1_DIV_NUM_MASK)
                | ((value << AdcCtrl::LPADC_SAR1_DIV_NUM_OFFSET)
                    & AdcCtrl::LPADC_SAR1_DIV_NUM_MASK),
        }
    }

    /// Set **LPADC_SAR2_DIV_NUM** `[23:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lpadc_sar2_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AdcCtrl::LPADC_SAR2_DIV_NUM_MASK)
                | ((value << AdcCtrl::LPADC_SAR2_DIV_NUM_OFFSET)
                    & AdcCtrl::LPADC_SAR2_DIV_NUM_MASK),
        }
    }

    /// Set **LPADC_FUNC_DIV_NUM** `[15:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lpadc_func_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AdcCtrl::LPADC_FUNC_DIV_NUM_MASK)
                | ((value << AdcCtrl::LPADC_FUNC_DIV_NUM_OFFSET)
                    & AdcCtrl::LPADC_FUNC_DIV_NUM_MASK),
        }
    }

    /// Set **SAR1_CLK_FORCE_ON** `[7:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_sar1_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AdcCtrl::SAR1_CLK_FORCE_ON_MASK
            } else {
                self.bits & !AdcCtrl::SAR1_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **SAR2_CLK_FORCE_ON** `[6:6]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_sar2_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | AdcCtrl::SAR2_CLK_FORCE_ON_MASK
            } else {
                self.bits & !AdcCtrl::SAR2_CLK_FORCE_ON_MASK
            },
        }
    }
}

impl Default for AdcCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AdcCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AdcCtrlVal {
        AdcCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AdcCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AdcCtrlVal) -> AdcCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_I2S_RXCLK_DIV_NUM Register
///
/// need_des
///
/// Address offset: 0x0030
/// Reset value: 0x02000000
///
/// ## Fields
///
/// - **LP_I2S_RX_CLKM_DIV_NUM** `[31:24]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpI2sRxclkDivNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpI2sRxclkDivNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpI2sRxclkDivNum {
    // ======== Field Constants ========

    /// **LP_I2S_RX_CLKM_DIV_NUM**
    ///
    /// need_des
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const LP_I2S_RX_CLKM_DIV_NUM_OFFSET: u32 = 24;
    pub const LP_I2S_RX_CLKM_DIV_NUM_MASK: u32 = 0xff000000;

    // ======== Field Accessors ========

    /// **LP_I2S_RX_CLKM_DIV_NUM** `[31:24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2s_rx_clkm_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::LP_I2S_RX_CLKM_DIV_NUM_MASK)
            >> Self::LP_I2S_RX_CLKM_DIV_NUM_OFFSET
    }

    /// Write **LP_I2S_RX_CLKM_DIV_NUM** `[31:24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_i2s_rx_clkm_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_I2S_RX_CLKM_DIV_NUM_MASK)
                | ((value << Self::LP_I2S_RX_CLKM_DIV_NUM_OFFSET)
                    & Self::LP_I2S_RX_CLKM_DIV_NUM_MASK)
        });
    }
}

/// LP_I2S_RXCLK_DIV_NUM value type for batch read/modify/write operations
///
/// Use `lpi2srxclkdivnum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpi2srxclkdivnum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpI2sRxclkDivNumVal {
    bits: u32,
}

impl LpI2sRxclkDivNumVal {
    /// Create from raw bits
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
        Self { bits: 0x02000000 }
    }

    /// Read **LP_I2S_RX_CLKM_DIV_NUM** `[31:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2s_rx_clkm_div_num(&self) -> u32 {
        (self.bits & LpI2sRxclkDivNum::LP_I2S_RX_CLKM_DIV_NUM_MASK)
            >> LpI2sRxclkDivNum::LP_I2S_RX_CLKM_DIV_NUM_OFFSET
    }

    /// Set **LP_I2S_RX_CLKM_DIV_NUM** `[31:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_i2s_rx_clkm_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpI2sRxclkDivNum::LP_I2S_RX_CLKM_DIV_NUM_MASK)
                | ((value << LpI2sRxclkDivNum::LP_I2S_RX_CLKM_DIV_NUM_OFFSET)
                    & LpI2sRxclkDivNum::LP_I2S_RX_CLKM_DIV_NUM_MASK),
        }
    }
}

impl Default for LpI2sRxclkDivNumVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpI2sRxclkDivNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpI2sRxclkDivNumVal {
        LpI2sRxclkDivNumVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpI2sRxclkDivNumVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpI2sRxclkDivNumVal) -> LpI2sRxclkDivNumVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_I2S_RXCLK_DIV_XYZ Register
///
/// need_des
///
/// Address offset: 0x0034
/// Reset value: 0x00004000
///
/// ## Fields
///
/// - **LP_I2S_RX_CLKM_DIV_X** `[31:23]` (RW)
///
///   need_des
///
/// - **LP_I2S_RX_CLKM_DIV_Y** `[22:14]` (RW)
///
///   need_des
///
/// - **LP_I2S_RX_CLKM_DIV_Z** `[13:5]` (RW)
///
///   need_des
///
/// - **LP_I2S_RX_CLKM_DIV_YN1** `[4]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpI2sRxclkDivXyz {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpI2sRxclkDivXyz {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpI2sRxclkDivXyz {
    // ======== Field Constants ========

    /// **LP_I2S_RX_CLKM_DIV_X**
    ///
    /// need_des
    ///
    /// Bits `[31:23]`, mask `0xff800000`
    pub const LP_I2S_RX_CLKM_DIV_X_OFFSET: u32 = 23;
    pub const LP_I2S_RX_CLKM_DIV_X_MASK: u32 = 0xff800000;

    /// **LP_I2S_RX_CLKM_DIV_Y**
    ///
    /// need_des
    ///
    /// Bits `[22:14]`, mask `0x007fc000`
    pub const LP_I2S_RX_CLKM_DIV_Y_OFFSET: u32 = 14;
    pub const LP_I2S_RX_CLKM_DIV_Y_MASK: u32 = 0x007fc000;

    /// **LP_I2S_RX_CLKM_DIV_Z**
    ///
    /// need_des
    ///
    /// Bits `[13:5]`, mask `0x00003fe0`
    pub const LP_I2S_RX_CLKM_DIV_Z_OFFSET: u32 = 5;
    pub const LP_I2S_RX_CLKM_DIV_Z_MASK: u32 = 0x00003fe0;

    /// **LP_I2S_RX_CLKM_DIV_YN1**
    ///
    /// need_des
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_I2S_RX_CLKM_DIV_YN1_OFFSET: u32 = 4;
    pub const LP_I2S_RX_CLKM_DIV_YN1_MASK: u32 = 0x00000010;
    pub const LP_I2S_RX_CLKM_DIV_YN1: u32 = 1 << 4;

    // ======== Field Accessors ========

    /// **LP_I2S_RX_CLKM_DIV_X** `[31:23]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2s_rx_clkm_div_x(&self) -> u32 {
        (self.value.read_bits() & Self::LP_I2S_RX_CLKM_DIV_X_MASK)
            >> Self::LP_I2S_RX_CLKM_DIV_X_OFFSET
    }

    /// Write **LP_I2S_RX_CLKM_DIV_X** `[31:23]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_i2s_rx_clkm_div_x(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_I2S_RX_CLKM_DIV_X_MASK)
                | ((value << Self::LP_I2S_RX_CLKM_DIV_X_OFFSET) & Self::LP_I2S_RX_CLKM_DIV_X_MASK)
        });
    }

    /// **LP_I2S_RX_CLKM_DIV_Y** `[22:14]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2s_rx_clkm_div_y(&self) -> u32 {
        (self.value.read_bits() & Self::LP_I2S_RX_CLKM_DIV_Y_MASK)
            >> Self::LP_I2S_RX_CLKM_DIV_Y_OFFSET
    }

    /// Write **LP_I2S_RX_CLKM_DIV_Y** `[22:14]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_i2s_rx_clkm_div_y(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_I2S_RX_CLKM_DIV_Y_MASK)
                | ((value << Self::LP_I2S_RX_CLKM_DIV_Y_OFFSET) & Self::LP_I2S_RX_CLKM_DIV_Y_MASK)
        });
    }

    /// **LP_I2S_RX_CLKM_DIV_Z** `[13:5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2s_rx_clkm_div_z(&self) -> u32 {
        (self.value.read_bits() & Self::LP_I2S_RX_CLKM_DIV_Z_MASK)
            >> Self::LP_I2S_RX_CLKM_DIV_Z_OFFSET
    }

    /// Write **LP_I2S_RX_CLKM_DIV_Z** `[13:5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_i2s_rx_clkm_div_z(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_I2S_RX_CLKM_DIV_Z_MASK)
                | ((value << Self::LP_I2S_RX_CLKM_DIV_Z_OFFSET) & Self::LP_I2S_RX_CLKM_DIV_Z_MASK)
        });
    }

    /// **LP_I2S_RX_CLKM_DIV_YN1** `[4]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2s_rx_clkm_div_yn1(&self) -> bool {
        (self.value.read_bits() & Self::LP_I2S_RX_CLKM_DIV_YN1_MASK) != 0
    }

    /// Set LP_I2S_RX_CLKM_DIV_YN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_i2s_rx_clkm_div_yn1(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_I2S_RX_CLKM_DIV_YN1_MASK);
    }

    /// Clear LP_I2S_RX_CLKM_DIV_YN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_i2s_rx_clkm_div_yn1(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_I2S_RX_CLKM_DIV_YN1_MASK);
    }
}

/// LP_I2S_RXCLK_DIV_XYZ value type for batch read/modify/write operations
///
/// Use `lpi2srxclkdivxyz.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpi2srxclkdivxyz.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpI2sRxclkDivXyzVal {
    bits: u32,
}

impl LpI2sRxclkDivXyzVal {
    /// Create from raw bits
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
        Self { bits: 0x00004000 }
    }

    /// Read **LP_I2S_RX_CLKM_DIV_X** `[31:23]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2s_rx_clkm_div_x(&self) -> u32 {
        (self.bits & LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_X_MASK)
            >> LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_X_OFFSET
    }

    /// Read **LP_I2S_RX_CLKM_DIV_Y** `[22:14]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2s_rx_clkm_div_y(&self) -> u32 {
        (self.bits & LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_Y_MASK)
            >> LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_Y_OFFSET
    }

    /// Read **LP_I2S_RX_CLKM_DIV_Z** `[13:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2s_rx_clkm_div_z(&self) -> u32 {
        (self.bits & LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_Z_MASK)
            >> LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_Z_OFFSET
    }

    /// Read **LP_I2S_RX_CLKM_DIV_YN1** `[4:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2s_rx_clkm_div_yn1(&self) -> bool {
        (self.bits & LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_YN1_MASK) != 0
    }

    /// Set **LP_I2S_RX_CLKM_DIV_X** `[31:23]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_i2s_rx_clkm_div_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_X_MASK)
                | ((value << LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_X_OFFSET)
                    & LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_X_MASK),
        }
    }

    /// Set **LP_I2S_RX_CLKM_DIV_Y** `[22:14]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_i2s_rx_clkm_div_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_Y_MASK)
                | ((value << LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_Y_OFFSET)
                    & LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_Y_MASK),
        }
    }

    /// Set **LP_I2S_RX_CLKM_DIV_Z** `[13:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_i2s_rx_clkm_div_z(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_Z_MASK)
                | ((value << LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_Z_OFFSET)
                    & LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_Z_MASK),
        }
    }

    /// Set **LP_I2S_RX_CLKM_DIV_YN1** `[4:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_i2s_rx_clkm_div_yn1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_YN1_MASK
            } else {
                self.bits & !LpI2sRxclkDivXyz::LP_I2S_RX_CLKM_DIV_YN1_MASK
            },
        }
    }
}

impl Default for LpI2sRxclkDivXyzVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpI2sRxclkDivXyz {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpI2sRxclkDivXyzVal {
        LpI2sRxclkDivXyzVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpI2sRxclkDivXyzVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpI2sRxclkDivXyzVal) -> LpI2sRxclkDivXyzVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_I2S_TXCLK_DIV_NUM Register
///
/// need_des
///
/// Address offset: 0x0038
/// Reset value: 0x02000000
///
/// ## Fields
///
/// - **LP_I2S_TX_CLKM_DIV_NUM** `[31:24]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpI2sTxclkDivNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpI2sTxclkDivNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpI2sTxclkDivNum {
    // ======== Field Constants ========

    /// **LP_I2S_TX_CLKM_DIV_NUM**
    ///
    /// need_des
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const LP_I2S_TX_CLKM_DIV_NUM_OFFSET: u32 = 24;
    pub const LP_I2S_TX_CLKM_DIV_NUM_MASK: u32 = 0xff000000;

    // ======== Field Accessors ========

    /// **LP_I2S_TX_CLKM_DIV_NUM** `[31:24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2s_tx_clkm_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::LP_I2S_TX_CLKM_DIV_NUM_MASK)
            >> Self::LP_I2S_TX_CLKM_DIV_NUM_OFFSET
    }

    /// Write **LP_I2S_TX_CLKM_DIV_NUM** `[31:24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_i2s_tx_clkm_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_I2S_TX_CLKM_DIV_NUM_MASK)
                | ((value << Self::LP_I2S_TX_CLKM_DIV_NUM_OFFSET)
                    & Self::LP_I2S_TX_CLKM_DIV_NUM_MASK)
        });
    }
}

/// LP_I2S_TXCLK_DIV_NUM value type for batch read/modify/write operations
///
/// Use `lpi2stxclkdivnum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpi2stxclkdivnum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpI2sTxclkDivNumVal {
    bits: u32,
}

impl LpI2sTxclkDivNumVal {
    /// Create from raw bits
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
        Self { bits: 0x02000000 }
    }

    /// Read **LP_I2S_TX_CLKM_DIV_NUM** `[31:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2s_tx_clkm_div_num(&self) -> u32 {
        (self.bits & LpI2sTxclkDivNum::LP_I2S_TX_CLKM_DIV_NUM_MASK)
            >> LpI2sTxclkDivNum::LP_I2S_TX_CLKM_DIV_NUM_OFFSET
    }

    /// Set **LP_I2S_TX_CLKM_DIV_NUM** `[31:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_i2s_tx_clkm_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpI2sTxclkDivNum::LP_I2S_TX_CLKM_DIV_NUM_MASK)
                | ((value << LpI2sTxclkDivNum::LP_I2S_TX_CLKM_DIV_NUM_OFFSET)
                    & LpI2sTxclkDivNum::LP_I2S_TX_CLKM_DIV_NUM_MASK),
        }
    }
}

impl Default for LpI2sTxclkDivNumVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpI2sTxclkDivNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpI2sTxclkDivNumVal {
        LpI2sTxclkDivNumVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpI2sTxclkDivNumVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpI2sTxclkDivNumVal) -> LpI2sTxclkDivNumVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_I2S_TXCLK_DIV_XYZ Register
///
/// need_des
///
/// Address offset: 0x003c
/// Reset value: 0x00004000
///
/// ## Fields
///
/// - **LP_I2S_TX_CLKM_DIV_X** `[31:23]` (RW)
///
///   need_des
///
/// - **LP_I2S_TX_CLKM_DIV_Y** `[22:14]` (RW)
///
///   need_des
///
/// - **LP_I2S_TX_CLKM_DIV_Z** `[13:5]` (RW)
///
///   need_des
///
/// - **LP_I2S_TX_CLKM_DIV_YN1** `[4]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpI2sTxclkDivXyz {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpI2sTxclkDivXyz {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpI2sTxclkDivXyz {
    // ======== Field Constants ========

    /// **LP_I2S_TX_CLKM_DIV_X**
    ///
    /// need_des
    ///
    /// Bits `[31:23]`, mask `0xff800000`
    pub const LP_I2S_TX_CLKM_DIV_X_OFFSET: u32 = 23;
    pub const LP_I2S_TX_CLKM_DIV_X_MASK: u32 = 0xff800000;

    /// **LP_I2S_TX_CLKM_DIV_Y**
    ///
    /// need_des
    ///
    /// Bits `[22:14]`, mask `0x007fc000`
    pub const LP_I2S_TX_CLKM_DIV_Y_OFFSET: u32 = 14;
    pub const LP_I2S_TX_CLKM_DIV_Y_MASK: u32 = 0x007fc000;

    /// **LP_I2S_TX_CLKM_DIV_Z**
    ///
    /// need_des
    ///
    /// Bits `[13:5]`, mask `0x00003fe0`
    pub const LP_I2S_TX_CLKM_DIV_Z_OFFSET: u32 = 5;
    pub const LP_I2S_TX_CLKM_DIV_Z_MASK: u32 = 0x00003fe0;

    /// **LP_I2S_TX_CLKM_DIV_YN1**
    ///
    /// need_des
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_I2S_TX_CLKM_DIV_YN1_OFFSET: u32 = 4;
    pub const LP_I2S_TX_CLKM_DIV_YN1_MASK: u32 = 0x00000010;
    pub const LP_I2S_TX_CLKM_DIV_YN1: u32 = 1 << 4;

    // ======== Field Accessors ========

    /// **LP_I2S_TX_CLKM_DIV_X** `[31:23]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2s_tx_clkm_div_x(&self) -> u32 {
        (self.value.read_bits() & Self::LP_I2S_TX_CLKM_DIV_X_MASK)
            >> Self::LP_I2S_TX_CLKM_DIV_X_OFFSET
    }

    /// Write **LP_I2S_TX_CLKM_DIV_X** `[31:23]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_i2s_tx_clkm_div_x(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_I2S_TX_CLKM_DIV_X_MASK)
                | ((value << Self::LP_I2S_TX_CLKM_DIV_X_OFFSET) & Self::LP_I2S_TX_CLKM_DIV_X_MASK)
        });
    }

    /// **LP_I2S_TX_CLKM_DIV_Y** `[22:14]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2s_tx_clkm_div_y(&self) -> u32 {
        (self.value.read_bits() & Self::LP_I2S_TX_CLKM_DIV_Y_MASK)
            >> Self::LP_I2S_TX_CLKM_DIV_Y_OFFSET
    }

    /// Write **LP_I2S_TX_CLKM_DIV_Y** `[22:14]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_i2s_tx_clkm_div_y(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_I2S_TX_CLKM_DIV_Y_MASK)
                | ((value << Self::LP_I2S_TX_CLKM_DIV_Y_OFFSET) & Self::LP_I2S_TX_CLKM_DIV_Y_MASK)
        });
    }

    /// **LP_I2S_TX_CLKM_DIV_Z** `[13:5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2s_tx_clkm_div_z(&self) -> u32 {
        (self.value.read_bits() & Self::LP_I2S_TX_CLKM_DIV_Z_MASK)
            >> Self::LP_I2S_TX_CLKM_DIV_Z_OFFSET
    }

    /// Write **LP_I2S_TX_CLKM_DIV_Z** `[13:5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_i2s_tx_clkm_div_z(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_I2S_TX_CLKM_DIV_Z_MASK)
                | ((value << Self::LP_I2S_TX_CLKM_DIV_Z_OFFSET) & Self::LP_I2S_TX_CLKM_DIV_Z_MASK)
        });
    }

    /// **LP_I2S_TX_CLKM_DIV_YN1** `[4]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2s_tx_clkm_div_yn1(&self) -> bool {
        (self.value.read_bits() & Self::LP_I2S_TX_CLKM_DIV_YN1_MASK) != 0
    }

    /// Set LP_I2S_TX_CLKM_DIV_YN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_i2s_tx_clkm_div_yn1(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_I2S_TX_CLKM_DIV_YN1_MASK);
    }

    /// Clear LP_I2S_TX_CLKM_DIV_YN1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_i2s_tx_clkm_div_yn1(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_I2S_TX_CLKM_DIV_YN1_MASK);
    }
}

/// LP_I2S_TXCLK_DIV_XYZ value type for batch read/modify/write operations
///
/// Use `lpi2stxclkdivxyz.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpi2stxclkdivxyz.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpI2sTxclkDivXyzVal {
    bits: u32,
}

impl LpI2sTxclkDivXyzVal {
    /// Create from raw bits
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
        Self { bits: 0x00004000 }
    }

    /// Read **LP_I2S_TX_CLKM_DIV_X** `[31:23]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2s_tx_clkm_div_x(&self) -> u32 {
        (self.bits & LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_X_MASK)
            >> LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_X_OFFSET
    }

    /// Read **LP_I2S_TX_CLKM_DIV_Y** `[22:14]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2s_tx_clkm_div_y(&self) -> u32 {
        (self.bits & LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_Y_MASK)
            >> LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_Y_OFFSET
    }

    /// Read **LP_I2S_TX_CLKM_DIV_Z** `[13:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2s_tx_clkm_div_z(&self) -> u32 {
        (self.bits & LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_Z_MASK)
            >> LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_Z_OFFSET
    }

    /// Read **LP_I2S_TX_CLKM_DIV_YN1** `[4:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2s_tx_clkm_div_yn1(&self) -> bool {
        (self.bits & LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_YN1_MASK) != 0
    }

    /// Set **LP_I2S_TX_CLKM_DIV_X** `[31:23]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_i2s_tx_clkm_div_x(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_X_MASK)
                | ((value << LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_X_OFFSET)
                    & LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_X_MASK),
        }
    }

    /// Set **LP_I2S_TX_CLKM_DIV_Y** `[22:14]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_i2s_tx_clkm_div_y(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_Y_MASK)
                | ((value << LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_Y_OFFSET)
                    & LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_Y_MASK),
        }
    }

    /// Set **LP_I2S_TX_CLKM_DIV_Z** `[13:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_i2s_tx_clkm_div_z(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_Z_MASK)
                | ((value << LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_Z_OFFSET)
                    & LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_Z_MASK),
        }
    }

    /// Set **LP_I2S_TX_CLKM_DIV_YN1** `[4:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_i2s_tx_clkm_div_yn1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_YN1_MASK
            } else {
                self.bits & !LpI2sTxclkDivXyz::LP_I2S_TX_CLKM_DIV_YN1_MASK
            },
        }
    }
}

impl Default for LpI2sTxclkDivXyzVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpI2sTxclkDivXyz {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpI2sTxclkDivXyzVal {
        LpI2sTxclkDivXyzVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpI2sTxclkDivXyzVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpI2sTxclkDivXyzVal) -> LpI2sTxclkDivXyzVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// need_des
///
/// Address offset: 0x03fc
///
/// ## Fields
///
/// - **CLK_EN** `[31]` (RW)
///
///   need_des
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

    /// **CLK_EN**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_EN_OFFSET: u32 = 31;
    pub const CLK_EN_MASK: u32 = 0x80000000;
    pub const CLK_EN: u32 = 1 << 31;

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

    /// Read **CLK_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & Date::CLK_EN_MASK) != 0
    }

    /// Set **CLK_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Date::CLK_EN_MASK
            } else {
                self.bits & !Date::CLK_EN_MASK
            },
        }
    }
}

impl Default for DateVal {
    fn default() -> Self {
        Self { bits: 0 }
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
    /// need_des
    pub clk_en: ClkEn,
    /// 0x0004
    /// need_des
    pub core_clk_sel: CoreClkSel,
    /// 0x0008
    /// need_des
    pub reset_en: ResetEn,
    /// 0x000c
    /// need_des
    pub cpu: Cpu,
    _reserved0: [u8; 24],
    /// 0x0028
    /// need_des
    pub mem_ctrl: MemCtrl,
    /// 0x002c
    /// need_des
    pub adc_ctrl: AdcCtrl,
    /// 0x0030
    /// need_des
    pub lp_i2s_rxclk_div_num: LpI2sRxclkDivNum,
    /// 0x0034
    /// need_des
    pub lp_i2s_rxclk_div_xyz: LpI2sRxclkDivXyz,
    /// 0x0038
    /// need_des
    pub lp_i2s_txclk_div_num: LpI2sTxclkDivNum,
    /// 0x003c
    /// need_des
    pub lp_i2s_txclk_div_xyz: LpI2sTxclkDivXyz,
    _reserved1: [u8; 956],
    /// 0x03fc
    /// need_des
    pub date: Date,
}
