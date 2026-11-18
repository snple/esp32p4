//! LP_INTR
//! Low-power Interrupt Controller

#![allow(dead_code)]

pub const BASE: u32 = 0x5012c000;

/// SW_INT_RAW Register
///
/// need_des
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **LP_SW_INT_RAW** `[31]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct SwIntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SwIntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SwIntRaw {
    // ======== Field Constants ========

    /// **LP_SW_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_SW_INT_RAW_OFFSET: u32 = 31;
    pub const LP_SW_INT_RAW_MASK: u32 = 0x80000000;
    pub const LP_SW_INT_RAW: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **LP_SW_INT_RAW** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_sw_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::LP_SW_INT_RAW_MASK) != 0
    }

    /// Set LP_SW_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_sw_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::LP_SW_INT_RAW_MASK);
    }

    /// Clear LP_SW_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_sw_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::LP_SW_INT_RAW_MASK);
    }
}

/// SW_INT_RAW value type for batch read/modify/write operations
///
/// Use `swintraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swintraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwIntRawVal {
    bits: u32,
}

impl SwIntRawVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SW_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_sw_int_raw(&self) -> bool {
        (self.bits & SwIntRaw::LP_SW_INT_RAW_MASK) != 0
    }

    /// Set **LP_SW_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_sw_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwIntRaw::LP_SW_INT_RAW_MASK
            } else {
                self.bits & !SwIntRaw::LP_SW_INT_RAW_MASK
            },
        }
    }
}

impl Default for SwIntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SwIntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SwIntRawVal {
        SwIntRawVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SwIntRawVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SwIntRawVal) -> SwIntRawVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SW_INT_ST Register
///
/// need_des
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **LP_SW_INT_ST** `[31]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct SwIntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SwIntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SwIntSt {
    // ======== Field Constants ========

    /// **LP_SW_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_SW_INT_ST_OFFSET: u32 = 31;
    pub const LP_SW_INT_ST_MASK: u32 = 0x80000000;
    pub const LP_SW_INT_ST: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **LP_SW_INT_ST** `[31]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_sw_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_SW_INT_ST_MASK) != 0
    }
}

/// SW_INT_ST value type for batch read/modify/write operations
///
/// Use `swintst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swintst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwIntStVal {
    bits: u32,
}

impl SwIntStVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SW_INT_ST** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_sw_int_st(&self) -> bool {
        (self.bits & SwIntSt::LP_SW_INT_ST_MASK) != 0
    }
}

impl Default for SwIntStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SwIntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SwIntStVal {
        SwIntStVal::from_bits(self.value.read_bits())
    }
}

/// SW_INT_ENA Register
///
/// need_des
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **LP_SW_INT_ENA** `[31]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct SwIntEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SwIntEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SwIntEna {
    // ======== Field Constants ========

    /// **LP_SW_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_SW_INT_ENA_OFFSET: u32 = 31;
    pub const LP_SW_INT_ENA_MASK: u32 = 0x80000000;
    pub const LP_SW_INT_ENA: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **LP_SW_INT_ENA** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_sw_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_SW_INT_ENA_MASK) != 0
    }

    /// Set LP_SW_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_sw_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::LP_SW_INT_ENA_MASK);
    }

    /// Clear LP_SW_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_sw_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::LP_SW_INT_ENA_MASK);
    }
}

/// SW_INT_ENA value type for batch read/modify/write operations
///
/// Use `swintena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swintena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwIntEnaVal {
    bits: u32,
}

impl SwIntEnaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SW_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_sw_int_ena(&self) -> bool {
        (self.bits & SwIntEna::LP_SW_INT_ENA_MASK) != 0
    }

    /// Set **LP_SW_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_sw_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwIntEna::LP_SW_INT_ENA_MASK
            } else {
                self.bits & !SwIntEna::LP_SW_INT_ENA_MASK
            },
        }
    }
}

impl Default for SwIntEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SwIntEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SwIntEnaVal {
        SwIntEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SwIntEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SwIntEnaVal) -> SwIntEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SW_INT_CLR Register
///
/// need_des
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **LP_SW_INT_CLR** `[31]` (WO)
///
///   need_des
#[repr(transparent)]
pub struct SwIntClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SwIntClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SwIntClr {
    // ======== Field Constants ========

    /// **LP_SW_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_SW_INT_CLR_OFFSET: u32 = 31;
    pub const LP_SW_INT_CLR_MASK: u32 = 0x80000000;
    pub const LP_SW_INT_CLR: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// Trigger **LP_SW_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_sw_int_clr(&self) {
        self.value.write_bits(Self::LP_SW_INT_CLR_MASK);
    }
}

/// SW_INT_CLR value type for batch read/modify/write operations
///
/// Use `swintclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `swintclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SwIntClrVal {
    bits: u32,
}

impl SwIntClrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **LP_SW_INT_CLR** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_sw_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SwIntClr::LP_SW_INT_CLR_MASK
            } else {
                self.bits & !SwIntClr::LP_SW_INT_CLR_MASK
            },
        }
    }
}

impl Default for SwIntClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SwIntClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SwIntClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// STATUS Register
///
/// need_des
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **HP_INTR_ST** `[31]` (RO)
///
///   need_des
///
/// - **LP_RTC_INTR_ST** `[30]` (RO)
///
///   need_des
///
/// - **LP_WDT_INTR_ST** `[29]` (RO)
///
///   need_des
///
/// - **LP_TIMER_REG_0_INTR_ST** `[28]` (RO)
///
///   need_des
///
/// - **LP_TIMER_REG_1_INTR_ST** `[27]` (RO)
///
///   need_des
///
/// - **MB_HP_INTR_ST** `[26]` (RO)
///
///   need_des
///
/// - **MB_LP_INTR_ST** `[25]` (RO)
///
///   need_des
///
/// - **PMU_REG_0_INTR_ST** `[24]` (RO)
///
///   need_des
///
/// - **PMU_REG_1_INTR_ST** `[23]` (RO)
///
///   need_des
///
/// - **ANAPERI_INTR_ST** `[22]` (RO)
///
///   need_des
///
/// - **LP_ADC_INTR_ST** `[21]` (RO)
///
///   need_des
///
/// - **LP_GPIO_INTR_ST** `[20]` (RO)
///
///   need_des
///
/// - **LP_I2C_INTR_ST** `[19]` (RO)
///
///   need_des
///
/// - **LP_I2S_INTR_ST** `[18]` (RO)
///
///   need_des
///
/// - **LP_SPI_INTR_ST** `[17]` (RO)
///
///   need_des
///
/// - **LP_TOUCH_INTR_ST** `[16]` (RO)
///
///   need_des
///
/// - **LP_TSENS_INTR_ST** `[15]` (RO)
///
///   need_des
///
/// - **LP_UART_INTR_ST** `[14]` (RO)
///
///   need_des
///
/// - **LP_EFUSE_INTR_ST** `[13]` (RO)
///
///   need_des
///
/// - **LP_SW_INTR_ST** `[12]` (RO)
///
///   need_des
///
/// - **SYSREG_INTR_ST** `[11]` (RO)
///
///   need_des
///
/// - **LP_HUK_INTR_ST** `[10]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status {
    // ======== Field Constants ========

    /// **HP_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const HP_INTR_ST_OFFSET: u32 = 31;
    pub const HP_INTR_ST_MASK: u32 = 0x80000000;
    pub const HP_INTR_ST: u32 = 1 << 31;

    /// **LP_RTC_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_RTC_INTR_ST_OFFSET: u32 = 30;
    pub const LP_RTC_INTR_ST_MASK: u32 = 0x40000000;
    pub const LP_RTC_INTR_ST: u32 = 1 << 30;

    /// **LP_WDT_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_WDT_INTR_ST_OFFSET: u32 = 29;
    pub const LP_WDT_INTR_ST_MASK: u32 = 0x20000000;
    pub const LP_WDT_INTR_ST: u32 = 1 << 29;

    /// **LP_TIMER_REG_0_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_TIMER_REG_0_INTR_ST_OFFSET: u32 = 28;
    pub const LP_TIMER_REG_0_INTR_ST_MASK: u32 = 0x10000000;
    pub const LP_TIMER_REG_0_INTR_ST: u32 = 1 << 28;

    /// **LP_TIMER_REG_1_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LP_TIMER_REG_1_INTR_ST_OFFSET: u32 = 27;
    pub const LP_TIMER_REG_1_INTR_ST_MASK: u32 = 0x08000000;
    pub const LP_TIMER_REG_1_INTR_ST: u32 = 1 << 27;

    /// **MB_HP_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const MB_HP_INTR_ST_OFFSET: u32 = 26;
    pub const MB_HP_INTR_ST_MASK: u32 = 0x04000000;
    pub const MB_HP_INTR_ST: u32 = 1 << 26;

    /// **MB_LP_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const MB_LP_INTR_ST_OFFSET: u32 = 25;
    pub const MB_LP_INTR_ST_MASK: u32 = 0x02000000;
    pub const MB_LP_INTR_ST: u32 = 1 << 25;

    /// **PMU_REG_0_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const PMU_REG_0_INTR_ST_OFFSET: u32 = 24;
    pub const PMU_REG_0_INTR_ST_MASK: u32 = 0x01000000;
    pub const PMU_REG_0_INTR_ST: u32 = 1 << 24;

    /// **PMU_REG_1_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const PMU_REG_1_INTR_ST_OFFSET: u32 = 23;
    pub const PMU_REG_1_INTR_ST_MASK: u32 = 0x00800000;
    pub const PMU_REG_1_INTR_ST: u32 = 1 << 23;

    /// **ANAPERI_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const ANAPERI_INTR_ST_OFFSET: u32 = 22;
    pub const ANAPERI_INTR_ST_MASK: u32 = 0x00400000;
    pub const ANAPERI_INTR_ST: u32 = 1 << 22;

    /// **LP_ADC_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const LP_ADC_INTR_ST_OFFSET: u32 = 21;
    pub const LP_ADC_INTR_ST_MASK: u32 = 0x00200000;
    pub const LP_ADC_INTR_ST: u32 = 1 << 21;

    /// **LP_GPIO_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const LP_GPIO_INTR_ST_OFFSET: u32 = 20;
    pub const LP_GPIO_INTR_ST_MASK: u32 = 0x00100000;
    pub const LP_GPIO_INTR_ST: u32 = 1 << 20;

    /// **LP_I2C_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const LP_I2C_INTR_ST_OFFSET: u32 = 19;
    pub const LP_I2C_INTR_ST_MASK: u32 = 0x00080000;
    pub const LP_I2C_INTR_ST: u32 = 1 << 19;

    /// **LP_I2S_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const LP_I2S_INTR_ST_OFFSET: u32 = 18;
    pub const LP_I2S_INTR_ST_MASK: u32 = 0x00040000;
    pub const LP_I2S_INTR_ST: u32 = 1 << 18;

    /// **LP_SPI_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const LP_SPI_INTR_ST_OFFSET: u32 = 17;
    pub const LP_SPI_INTR_ST_MASK: u32 = 0x00020000;
    pub const LP_SPI_INTR_ST: u32 = 1 << 17;

    /// **LP_TOUCH_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const LP_TOUCH_INTR_ST_OFFSET: u32 = 16;
    pub const LP_TOUCH_INTR_ST_MASK: u32 = 0x00010000;
    pub const LP_TOUCH_INTR_ST: u32 = 1 << 16;

    /// **LP_TSENS_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const LP_TSENS_INTR_ST_OFFSET: u32 = 15;
    pub const LP_TSENS_INTR_ST_MASK: u32 = 0x00008000;
    pub const LP_TSENS_INTR_ST: u32 = 1 << 15;

    /// **LP_UART_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const LP_UART_INTR_ST_OFFSET: u32 = 14;
    pub const LP_UART_INTR_ST_MASK: u32 = 0x00004000;
    pub const LP_UART_INTR_ST: u32 = 1 << 14;

    /// **LP_EFUSE_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const LP_EFUSE_INTR_ST_OFFSET: u32 = 13;
    pub const LP_EFUSE_INTR_ST_MASK: u32 = 0x00002000;
    pub const LP_EFUSE_INTR_ST: u32 = 1 << 13;

    /// **LP_SW_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const LP_SW_INTR_ST_OFFSET: u32 = 12;
    pub const LP_SW_INTR_ST_MASK: u32 = 0x00001000;
    pub const LP_SW_INTR_ST: u32 = 1 << 12;

    /// **SYSREG_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const SYSREG_INTR_ST_OFFSET: u32 = 11;
    pub const SYSREG_INTR_ST_MASK: u32 = 0x00000800;
    pub const SYSREG_INTR_ST: u32 = 1 << 11;

    /// **LP_HUK_INTR_ST**
    ///
    /// need_des
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const LP_HUK_INTR_ST_OFFSET: u32 = 10;
    pub const LP_HUK_INTR_ST_MASK: u32 = 0x00000400;
    pub const LP_HUK_INTR_ST: u32 = 1 << 10;

    // ======== Field Accessors ========

    /// **HP_INTR_ST** `[31]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn hp_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::HP_INTR_ST_MASK) != 0
    }

    /// **LP_RTC_INTR_ST** `[30]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_rtc_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_RTC_INTR_ST_MASK) != 0
    }

    /// **LP_WDT_INTR_ST** `[29]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_wdt_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_WDT_INTR_ST_MASK) != 0
    }

    /// **LP_TIMER_REG_0_INTR_ST** `[28]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_timer_reg_0_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_TIMER_REG_0_INTR_ST_MASK) != 0
    }

    /// **LP_TIMER_REG_1_INTR_ST** `[27]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_timer_reg_1_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_TIMER_REG_1_INTR_ST_MASK) != 0
    }

    /// **MB_HP_INTR_ST** `[26]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn mb_hp_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::MB_HP_INTR_ST_MASK) != 0
    }

    /// **MB_LP_INTR_ST** `[25]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn mb_lp_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::MB_LP_INTR_ST_MASK) != 0
    }

    /// **PMU_REG_0_INTR_ST** `[24]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pmu_reg_0_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::PMU_REG_0_INTR_ST_MASK) != 0
    }

    /// **PMU_REG_1_INTR_ST** `[23]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pmu_reg_1_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::PMU_REG_1_INTR_ST_MASK) != 0
    }

    /// **ANAPERI_INTR_ST** `[22]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn anaperi_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::ANAPERI_INTR_ST_MASK) != 0
    }

    /// **LP_ADC_INTR_ST** `[21]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_adc_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_ADC_INTR_ST_MASK) != 0
    }

    /// **LP_GPIO_INTR_ST** `[20]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_gpio_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_GPIO_INTR_ST_MASK) != 0
    }

    /// **LP_I2C_INTR_ST** `[19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2c_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_I2C_INTR_ST_MASK) != 0
    }

    /// **LP_I2S_INTR_ST** `[18]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_i2s_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_I2S_INTR_ST_MASK) != 0
    }

    /// **LP_SPI_INTR_ST** `[17]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_spi_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_SPI_INTR_ST_MASK) != 0
    }

    /// **LP_TOUCH_INTR_ST** `[16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_touch_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_TOUCH_INTR_ST_MASK) != 0
    }

    /// **LP_TSENS_INTR_ST** `[15]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_tsens_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_TSENS_INTR_ST_MASK) != 0
    }

    /// **LP_UART_INTR_ST** `[14]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_uart_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_UART_INTR_ST_MASK) != 0
    }

    /// **LP_EFUSE_INTR_ST** `[13]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_efuse_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_EFUSE_INTR_ST_MASK) != 0
    }

    /// **LP_SW_INTR_ST** `[12]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_sw_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_SW_INTR_ST_MASK) != 0
    }

    /// **SYSREG_INTR_ST** `[11]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn sysreg_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::SYSREG_INTR_ST_MASK) != 0
    }

    /// **LP_HUK_INTR_ST** `[10]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_huk_intr_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_HUK_INTR_ST_MASK) != 0
    }
}

/// STATUS value type for batch read/modify/write operations
///
/// Use `status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StatusVal {
    bits: u32,
}

impl StatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HP_INTR_ST** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn hp_intr_st(&self) -> bool {
        (self.bits & Status::HP_INTR_ST_MASK) != 0
    }

    /// Read **LP_RTC_INTR_ST** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_rtc_intr_st(&self) -> bool {
        (self.bits & Status::LP_RTC_INTR_ST_MASK) != 0
    }

    /// Read **LP_WDT_INTR_ST** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_wdt_intr_st(&self) -> bool {
        (self.bits & Status::LP_WDT_INTR_ST_MASK) != 0
    }

    /// Read **LP_TIMER_REG_0_INTR_ST** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_timer_reg_0_intr_st(&self) -> bool {
        (self.bits & Status::LP_TIMER_REG_0_INTR_ST_MASK) != 0
    }

    /// Read **LP_TIMER_REG_1_INTR_ST** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_timer_reg_1_intr_st(&self) -> bool {
        (self.bits & Status::LP_TIMER_REG_1_INTR_ST_MASK) != 0
    }

    /// Read **MB_HP_INTR_ST** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn mb_hp_intr_st(&self) -> bool {
        (self.bits & Status::MB_HP_INTR_ST_MASK) != 0
    }

    /// Read **MB_LP_INTR_ST** `[25:25]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn mb_lp_intr_st(&self) -> bool {
        (self.bits & Status::MB_LP_INTR_ST_MASK) != 0
    }

    /// Read **PMU_REG_0_INTR_ST** `[24:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pmu_reg_0_intr_st(&self) -> bool {
        (self.bits & Status::PMU_REG_0_INTR_ST_MASK) != 0
    }

    /// Read **PMU_REG_1_INTR_ST** `[23:23]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pmu_reg_1_intr_st(&self) -> bool {
        (self.bits & Status::PMU_REG_1_INTR_ST_MASK) != 0
    }

    /// Read **ANAPERI_INTR_ST** `[22:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn anaperi_intr_st(&self) -> bool {
        (self.bits & Status::ANAPERI_INTR_ST_MASK) != 0
    }

    /// Read **LP_ADC_INTR_ST** `[21:21]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_adc_intr_st(&self) -> bool {
        (self.bits & Status::LP_ADC_INTR_ST_MASK) != 0
    }

    /// Read **LP_GPIO_INTR_ST** `[20:20]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_gpio_intr_st(&self) -> bool {
        (self.bits & Status::LP_GPIO_INTR_ST_MASK) != 0
    }

    /// Read **LP_I2C_INTR_ST** `[19:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2c_intr_st(&self) -> bool {
        (self.bits & Status::LP_I2C_INTR_ST_MASK) != 0
    }

    /// Read **LP_I2S_INTR_ST** `[18:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_i2s_intr_st(&self) -> bool {
        (self.bits & Status::LP_I2S_INTR_ST_MASK) != 0
    }

    /// Read **LP_SPI_INTR_ST** `[17:17]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_spi_intr_st(&self) -> bool {
        (self.bits & Status::LP_SPI_INTR_ST_MASK) != 0
    }

    /// Read **LP_TOUCH_INTR_ST** `[16:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_touch_intr_st(&self) -> bool {
        (self.bits & Status::LP_TOUCH_INTR_ST_MASK) != 0
    }

    /// Read **LP_TSENS_INTR_ST** `[15:15]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_tsens_intr_st(&self) -> bool {
        (self.bits & Status::LP_TSENS_INTR_ST_MASK) != 0
    }

    /// Read **LP_UART_INTR_ST** `[14:14]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_uart_intr_st(&self) -> bool {
        (self.bits & Status::LP_UART_INTR_ST_MASK) != 0
    }

    /// Read **LP_EFUSE_INTR_ST** `[13:13]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_efuse_intr_st(&self) -> bool {
        (self.bits & Status::LP_EFUSE_INTR_ST_MASK) != 0
    }

    /// Read **LP_SW_INTR_ST** `[12:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_sw_intr_st(&self) -> bool {
        (self.bits & Status::LP_SW_INTR_ST_MASK) != 0
    }

    /// Read **SYSREG_INTR_ST** `[11:11]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn sysreg_intr_st(&self) -> bool {
        (self.bits & Status::SYSREG_INTR_ST_MASK) != 0
    }

    /// Read **LP_HUK_INTR_ST** `[10:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_huk_intr_st(&self) -> bool {
        (self.bits & Status::LP_HUK_INTR_ST_MASK) != 0
    }
}

impl Default for StatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StatusVal {
        StatusVal::from_bits(self.value.read_bits())
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
    pub sw_int_raw: SwIntRaw,
    /// 0x0004
    /// need_des
    pub sw_int_st: SwIntSt,
    /// 0x0008
    /// need_des
    pub sw_int_ena: SwIntEna,
    /// 0x000c
    /// need_des
    pub sw_int_clr: SwIntClr,
    /// 0x0010
    /// need_des
    pub status: Status,
    _reserved0: [u8; 1000],
    /// 0x03fc
    /// need_des
    pub date: Date,
}
