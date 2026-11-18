//! LP_GPIO
//! Low-power General Purpose Input/Output

#![allow(dead_code)]

pub const BASE: u32 = 0x5012a000;

/// CLK_EN Register
///
/// Reserved
///
/// Address offset: 0x0000
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **REG_CLK_EN** `[0]` (RW)
///
///   Reserved
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

    /// **REG_CLK_EN**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_CLK_EN_OFFSET: u32 = 0;
    pub const REG_CLK_EN_MASK: u32 = 0x00000001;
    pub const REG_CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_CLK_EN** `[0]` (RW)
    ///
    /// Reserved
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
        Self { bits: 0x00000001 }
    }

    /// Read **REG_CLK_EN** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_clk_en(&self) -> bool {
        (self.bits & ClkEn::REG_CLK_EN_MASK) != 0
    }

    /// Set **REG_CLK_EN** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::REG_CLK_EN_MASK
            } else {
                self.bits & !ClkEn::REG_CLK_EN_MASK
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

/// VER_DATE Register
///
/// Reserved
///
/// Address offset: 0x0004
/// Reset value: 0x00230323
///
/// ## Fields
///
/// - **REG_VER_DATE** `[27:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct VerDate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VerDate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VerDate {
    // ======== Field Constants ========

    /// **REG_VER_DATE**
    ///
    /// Reserved
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const REG_VER_DATE_OFFSET: u32 = 0;
    pub const REG_VER_DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **REG_VER_DATE** `[27:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_ver_date(&self) -> u32 {
        (self.value.read_bits() & Self::REG_VER_DATE_MASK) >> Self::REG_VER_DATE_OFFSET
    }

    /// Write **REG_VER_DATE** `[27:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_ver_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_VER_DATE_MASK)
                | ((value << Self::REG_VER_DATE_OFFSET) & Self::REG_VER_DATE_MASK)
        });
    }
}

/// VER_DATE value type for batch read/modify/write operations
///
/// Use `verdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `verdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VerDateVal {
    bits: u32,
}

impl VerDateVal {
    /// Create from raw bits
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
        Self { bits: 0x00230323 }
    }

    /// Read **REG_VER_DATE** `[27:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_ver_date(&self) -> u32 {
        (self.bits & VerDate::REG_VER_DATE_MASK) >> VerDate::REG_VER_DATE_OFFSET
    }

    /// Set **REG_VER_DATE** `[27:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_ver_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VerDate::REG_VER_DATE_MASK)
                | ((value << VerDate::REG_VER_DATE_OFFSET) & VerDate::REG_VER_DATE_MASK),
        }
    }
}

impl Default for VerDateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VerDate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VerDateVal {
        VerDateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VerDateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VerDateVal) -> VerDateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT Register
///
/// Reserved
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **REG_GPIO_OUT_DATA** `[15:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Out {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Out {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Out {
    // ======== Field Constants ========

    /// **REG_GPIO_OUT_DATA**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_GPIO_OUT_DATA_OFFSET: u32 = 0;
    pub const REG_GPIO_OUT_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **REG_GPIO_OUT_DATA** `[15:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_out_data(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_OUT_DATA_MASK) >> Self::REG_GPIO_OUT_DATA_OFFSET
    }

    /// Write **REG_GPIO_OUT_DATA** `[15:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_out_data(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_OUT_DATA_MASK)
                | ((value << Self::REG_GPIO_OUT_DATA_OFFSET) & Self::REG_GPIO_OUT_DATA_MASK)
        });
    }
}

/// OUT value type for batch read/modify/write operations
///
/// Use `out.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `out.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutVal {
    bits: u32,
}

impl OutVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_OUT_DATA** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_out_data(&self) -> u32 {
        (self.bits & Out::REG_GPIO_OUT_DATA_MASK) >> Out::REG_GPIO_OUT_DATA_OFFSET
    }

    /// Set **REG_GPIO_OUT_DATA** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_out_data(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Out::REG_GPIO_OUT_DATA_MASK)
                | ((value << Out::REG_GPIO_OUT_DATA_OFFSET) & Out::REG_GPIO_OUT_DATA_MASK),
        }
    }
}

impl Default for OutVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Out {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OutVal {
        OutVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OutVal) -> OutVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// OUT_W1TS Register
///
/// Reserved
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **REG_GPIO_OUT_DATA_W1TS** `[15:0]` (WO)
///
///   Reserved
#[repr(transparent)]
pub struct OutW1ts {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutW1ts {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutW1ts {
    // ======== Field Constants ========

    /// **REG_GPIO_OUT_DATA_W1TS**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_GPIO_OUT_DATA_W1TS_OFFSET: u32 = 0;
    pub const REG_GPIO_OUT_DATA_W1TS_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **REG_GPIO_OUT_DATA_W1TS** `[15:0]` (WO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_out_data_w1ts(&self, value: u32) {
        self.value.write_bits(
            (value << Self::REG_GPIO_OUT_DATA_W1TS_OFFSET) & Self::REG_GPIO_OUT_DATA_W1TS_MASK,
        );
    }
}

/// OUT_W1TS value type for batch read/modify/write operations
///
/// Use `outw1ts.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outw1ts.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutW1tsVal {
    bits: u32,
}

impl OutW1tsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **REG_GPIO_OUT_DATA_W1TS** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_out_data_w1ts(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutW1ts::REG_GPIO_OUT_DATA_W1TS_MASK)
                | ((value << OutW1ts::REG_GPIO_OUT_DATA_W1TS_OFFSET)
                    & OutW1ts::REG_GPIO_OUT_DATA_W1TS_MASK),
        }
    }
}

impl Default for OutW1tsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutW1ts {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutW1tsVal) {
        self.value.write_bits(val.bits())
    }
}

/// OUT_W1TC Register
///
/// Reserved
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **REG_GPIO_OUT_DATA_W1TC** `[15:0]` (WO)
///
///   Reserved
#[repr(transparent)]
pub struct OutW1tc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OutW1tc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OutW1tc {
    // ======== Field Constants ========

    /// **REG_GPIO_OUT_DATA_W1TC**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_GPIO_OUT_DATA_W1TC_OFFSET: u32 = 0;
    pub const REG_GPIO_OUT_DATA_W1TC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **REG_GPIO_OUT_DATA_W1TC** `[15:0]` (WO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_out_data_w1tc(&self, value: u32) {
        self.value.write_bits(
            (value << Self::REG_GPIO_OUT_DATA_W1TC_OFFSET) & Self::REG_GPIO_OUT_DATA_W1TC_MASK,
        );
    }
}

/// OUT_W1TC value type for batch read/modify/write operations
///
/// Use `outw1tc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `outw1tc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OutW1tcVal {
    bits: u32,
}

impl OutW1tcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **REG_GPIO_OUT_DATA_W1TC** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_out_data_w1tc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OutW1tc::REG_GPIO_OUT_DATA_W1TC_MASK)
                | ((value << OutW1tc::REG_GPIO_OUT_DATA_W1TC_OFFSET)
                    & OutW1tc::REG_GPIO_OUT_DATA_W1TC_MASK),
        }
    }
}

impl Default for OutW1tcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OutW1tc {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OutW1tcVal) {
        self.value.write_bits(val.bits())
    }
}

/// ENABLE Register
///
/// Reserved
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **REG_GPIO_ENABLE_DATA** `[15:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Enable {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Enable {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Enable {
    // ======== Field Constants ========

    /// **REG_GPIO_ENABLE_DATA**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_GPIO_ENABLE_DATA_OFFSET: u32 = 0;
    pub const REG_GPIO_ENABLE_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **REG_GPIO_ENABLE_DATA** `[15:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_enable_data(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_ENABLE_DATA_MASK)
            >> Self::REG_GPIO_ENABLE_DATA_OFFSET
    }

    /// Write **REG_GPIO_ENABLE_DATA** `[15:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_enable_data(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_ENABLE_DATA_MASK)
                | ((value << Self::REG_GPIO_ENABLE_DATA_OFFSET) & Self::REG_GPIO_ENABLE_DATA_MASK)
        });
    }
}

/// ENABLE value type for batch read/modify/write operations
///
/// Use `enable.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `enable.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EnableVal {
    bits: u32,
}

impl EnableVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_ENABLE_DATA** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_enable_data(&self) -> u32 {
        (self.bits & Enable::REG_GPIO_ENABLE_DATA_MASK) >> Enable::REG_GPIO_ENABLE_DATA_OFFSET
    }

    /// Set **REG_GPIO_ENABLE_DATA** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_enable_data(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Enable::REG_GPIO_ENABLE_DATA_MASK)
                | ((value << Enable::REG_GPIO_ENABLE_DATA_OFFSET)
                    & Enable::REG_GPIO_ENABLE_DATA_MASK),
        }
    }
}

impl Default for EnableVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Enable {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EnableVal {
        EnableVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EnableVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EnableVal) -> EnableVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ENABLE_W1TS Register
///
/// Reserved
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **REG_GPIO_ENABLE_DATA_W1TS** `[15:0]` (WO)
///
///   Reserved
#[repr(transparent)]
pub struct EnableW1ts {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EnableW1ts {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EnableW1ts {
    // ======== Field Constants ========

    /// **REG_GPIO_ENABLE_DATA_W1TS**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_GPIO_ENABLE_DATA_W1TS_OFFSET: u32 = 0;
    pub const REG_GPIO_ENABLE_DATA_W1TS_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **REG_GPIO_ENABLE_DATA_W1TS** `[15:0]` (WO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_enable_data_w1ts(&self, value: u32) {
        self.value.write_bits(
            (value << Self::REG_GPIO_ENABLE_DATA_W1TS_OFFSET)
                & Self::REG_GPIO_ENABLE_DATA_W1TS_MASK,
        );
    }
}

/// ENABLE_W1TS value type for batch read/modify/write operations
///
/// Use `enablew1ts.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `enablew1ts.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EnableW1tsVal {
    bits: u32,
}

impl EnableW1tsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **REG_GPIO_ENABLE_DATA_W1TS** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_enable_data_w1ts(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EnableW1ts::REG_GPIO_ENABLE_DATA_W1TS_MASK)
                | ((value << EnableW1ts::REG_GPIO_ENABLE_DATA_W1TS_OFFSET)
                    & EnableW1ts::REG_GPIO_ENABLE_DATA_W1TS_MASK),
        }
    }
}

impl Default for EnableW1tsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EnableW1ts {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EnableW1tsVal) {
        self.value.write_bits(val.bits())
    }
}

/// ENABLE_W1TC Register
///
/// Reserved
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **REG_GPIO_ENABLE_DATA_W1TC** `[15:0]` (WO)
///
///   Reserved
#[repr(transparent)]
pub struct EnableW1tc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EnableW1tc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EnableW1tc {
    // ======== Field Constants ========

    /// **REG_GPIO_ENABLE_DATA_W1TC**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_GPIO_ENABLE_DATA_W1TC_OFFSET: u32 = 0;
    pub const REG_GPIO_ENABLE_DATA_W1TC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **REG_GPIO_ENABLE_DATA_W1TC** `[15:0]` (WO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_enable_data_w1tc(&self, value: u32) {
        self.value.write_bits(
            (value << Self::REG_GPIO_ENABLE_DATA_W1TC_OFFSET)
                & Self::REG_GPIO_ENABLE_DATA_W1TC_MASK,
        );
    }
}

/// ENABLE_W1TC value type for batch read/modify/write operations
///
/// Use `enablew1tc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `enablew1tc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EnableW1tcVal {
    bits: u32,
}

impl EnableW1tcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **REG_GPIO_ENABLE_DATA_W1TC** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_enable_data_w1tc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EnableW1tc::REG_GPIO_ENABLE_DATA_W1TC_MASK)
                | ((value << EnableW1tc::REG_GPIO_ENABLE_DATA_W1TC_OFFSET)
                    & EnableW1tc::REG_GPIO_ENABLE_DATA_W1TC_MASK),
        }
    }
}

impl Default for EnableW1tcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EnableW1tc {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EnableW1tcVal) {
        self.value.write_bits(val.bits())
    }
}

/// STATUS Register
///
/// Reserved
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **REG_GPIO_STATUS_DATA** `[15:0]` (RW)
///
///   Reserved
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

    /// **REG_GPIO_STATUS_DATA**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_GPIO_STATUS_DATA_OFFSET: u32 = 0;
    pub const REG_GPIO_STATUS_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **REG_GPIO_STATUS_DATA** `[15:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_status_data(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_STATUS_DATA_MASK)
            >> Self::REG_GPIO_STATUS_DATA_OFFSET
    }

    /// Write **REG_GPIO_STATUS_DATA** `[15:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_status_data(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_STATUS_DATA_MASK)
                | ((value << Self::REG_GPIO_STATUS_DATA_OFFSET) & Self::REG_GPIO_STATUS_DATA_MASK)
        });
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

    /// Read **REG_GPIO_STATUS_DATA** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_status_data(&self) -> u32 {
        (self.bits & Status::REG_GPIO_STATUS_DATA_MASK) >> Status::REG_GPIO_STATUS_DATA_OFFSET
    }

    /// Set **REG_GPIO_STATUS_DATA** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_status_data(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Status::REG_GPIO_STATUS_DATA_MASK)
                | ((value << Status::REG_GPIO_STATUS_DATA_OFFSET)
                    & Status::REG_GPIO_STATUS_DATA_MASK),
        }
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

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: StatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(StatusVal) -> StatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// STATUS_W1TS Register
///
/// Reserved
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **REG_GPIO_STATUS_DATA_W1TS** `[15:0]` (WO)
///
///   Reserved
#[repr(transparent)]
pub struct StatusW1ts {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for StatusW1ts {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl StatusW1ts {
    // ======== Field Constants ========

    /// **REG_GPIO_STATUS_DATA_W1TS**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_GPIO_STATUS_DATA_W1TS_OFFSET: u32 = 0;
    pub const REG_GPIO_STATUS_DATA_W1TS_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **REG_GPIO_STATUS_DATA_W1TS** `[15:0]` (WO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_status_data_w1ts(&self, value: u32) {
        self.value.write_bits(
            (value << Self::REG_GPIO_STATUS_DATA_W1TS_OFFSET)
                & Self::REG_GPIO_STATUS_DATA_W1TS_MASK,
        );
    }
}

/// STATUS_W1TS value type for batch read/modify/write operations
///
/// Use `statusw1ts.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `statusw1ts.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StatusW1tsVal {
    bits: u32,
}

impl StatusW1tsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **REG_GPIO_STATUS_DATA_W1TS** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_status_data_w1ts(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !StatusW1ts::REG_GPIO_STATUS_DATA_W1TS_MASK)
                | ((value << StatusW1ts::REG_GPIO_STATUS_DATA_W1TS_OFFSET)
                    & StatusW1ts::REG_GPIO_STATUS_DATA_W1TS_MASK),
        }
    }
}

impl Default for StatusW1tsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl StatusW1ts {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: StatusW1tsVal) {
        self.value.write_bits(val.bits())
    }
}

/// STATUS_W1TC Register
///
/// Reserved
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **REG_GPIO_STATUS_DATA_W1TC** `[15:0]` (WO)
///
///   Reserved
#[repr(transparent)]
pub struct StatusW1tc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for StatusW1tc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl StatusW1tc {
    // ======== Field Constants ========

    /// **REG_GPIO_STATUS_DATA_W1TC**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_GPIO_STATUS_DATA_W1TC_OFFSET: u32 = 0;
    pub const REG_GPIO_STATUS_DATA_W1TC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Write **REG_GPIO_STATUS_DATA_W1TC** `[15:0]` (WO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_status_data_w1tc(&self, value: u32) {
        self.value.write_bits(
            (value << Self::REG_GPIO_STATUS_DATA_W1TC_OFFSET)
                & Self::REG_GPIO_STATUS_DATA_W1TC_MASK,
        );
    }
}

/// STATUS_W1TC value type for batch read/modify/write operations
///
/// Use `statusw1tc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `statusw1tc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StatusW1tcVal {
    bits: u32,
}

impl StatusW1tcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **REG_GPIO_STATUS_DATA_W1TC** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_status_data_w1tc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !StatusW1tc::REG_GPIO_STATUS_DATA_W1TC_MASK)
                | ((value << StatusW1tc::REG_GPIO_STATUS_DATA_W1TC_OFFSET)
                    & StatusW1tc::REG_GPIO_STATUS_DATA_W1TC_MASK),
        }
    }
}

impl Default for StatusW1tcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl StatusW1tc {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: StatusW1tcVal) {
        self.value.write_bits(val.bits())
    }
}

/// STATUS_NEXT Register
///
/// Reserved
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **REG_GPIO_STATUS_INTERRUPT_NEXT** `[15:0]` (RO)
///
///   Reserved
#[repr(transparent)]
pub struct StatusNext {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for StatusNext {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl StatusNext {
    // ======== Field Constants ========

    /// **REG_GPIO_STATUS_INTERRUPT_NEXT**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_GPIO_STATUS_INTERRUPT_NEXT_OFFSET: u32 = 0;
    pub const REG_GPIO_STATUS_INTERRUPT_NEXT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **REG_GPIO_STATUS_INTERRUPT_NEXT** `[15:0]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_status_interrupt_next(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_STATUS_INTERRUPT_NEXT_MASK)
            >> Self::REG_GPIO_STATUS_INTERRUPT_NEXT_OFFSET
    }
}

/// STATUS_NEXT value type for batch read/modify/write operations
///
/// Use `statusnext.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `statusnext.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StatusNextVal {
    bits: u32,
}

impl StatusNextVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_STATUS_INTERRUPT_NEXT** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_status_interrupt_next(&self) -> u32 {
        (self.bits & StatusNext::REG_GPIO_STATUS_INTERRUPT_NEXT_MASK)
            >> StatusNext::REG_GPIO_STATUS_INTERRUPT_NEXT_OFFSET
    }
}

impl Default for StatusNextVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl StatusNext {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StatusNextVal {
        StatusNextVal::from_bits(self.value.read_bits())
    }
}

/// IN Register
///
/// Reserved
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **REG_GPIO_IN_DATA_NEXT** `[15:0]` (RO)
///
///   Reserved
#[repr(transparent)]
pub struct In {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for In {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl In {
    // ======== Field Constants ========

    /// **REG_GPIO_IN_DATA_NEXT**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_GPIO_IN_DATA_NEXT_OFFSET: u32 = 0;
    pub const REG_GPIO_IN_DATA_NEXT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **REG_GPIO_IN_DATA_NEXT** `[15:0]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_in_data_next(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_IN_DATA_NEXT_MASK)
            >> Self::REG_GPIO_IN_DATA_NEXT_OFFSET
    }
}

/// IN value type for batch read/modify/write operations
///
/// Use `in.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `in.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InVal {
    bits: u32,
}

impl InVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_IN_DATA_NEXT** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_in_data_next(&self) -> u32 {
        (self.bits & In::REG_GPIO_IN_DATA_NEXT_MASK) >> In::REG_GPIO_IN_DATA_NEXT_OFFSET
    }
}

impl Default for InVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl In {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InVal {
        InVal::from_bits(self.value.read_bits())
    }
}

/// PIN0 Register
///
/// Reserved
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **REG_GPIO_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN0_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN0_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN0_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin0 {
    // ======== Field Constants ========

    /// **REG_GPIO_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPIO_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPIO_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPIO_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN0_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN0_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN0_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN0_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN0_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN0_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN0_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN0_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN0_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN0_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN0_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPIO_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpio_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN0_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin0_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN0_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN0_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin0_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN0_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN0_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin0_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN0_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN0_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin0_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN0_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN0_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN0_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin0_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN0_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN0_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN0_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN0_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin0_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN0_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN0_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin0_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN0_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN0_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin0_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN0_WAKEUP_ENABLE_MASK);
    }
}

/// PIN0 value type for batch read/modify/write operations
///
/// Use `pin0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin0Val {
    bits: u32,
}

impl Pin0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN0_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin0_pad_driver(&self) -> bool {
        (self.bits & Pin0::REG_GPIO_PIN0_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN0_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin0_int_type(&self) -> u32 {
        (self.bits & Pin0::REG_GPIO_PIN0_INT_TYPE_MASK) >> Pin0::REG_GPIO_PIN0_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN0_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin0_wakeup_enable(&self) -> bool {
        (self.bits & Pin0::REG_GPIO_PIN0_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPIO_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpio_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin0::REG_GPIO_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin0::REG_GPIO_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN0_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin0_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin0::REG_GPIO_PIN0_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin0::REG_GPIO_PIN0_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN0_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin0_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin0::REG_GPIO_PIN0_INT_TYPE_MASK)
                | ((value << Pin0::REG_GPIO_PIN0_INT_TYPE_OFFSET)
                    & Pin0::REG_GPIO_PIN0_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN0_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin0_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin0::REG_GPIO_PIN0_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin0::REG_GPIO_PIN0_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin0Val {
        Pin0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin0Val) -> Pin0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN1 Register
///
/// Reserved
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **REG_GPI1_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN1_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN1_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN1_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin1 {
    // ======== Field Constants ========

    /// **REG_GPI1_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI1_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI1_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI1_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN1_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN1_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN1_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN1_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN1_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN1_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN1_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN1_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN1_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN1_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN1_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI1_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi1_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI1_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN1_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin1_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN1_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN1_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin1_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN1_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN1_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin1_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN1_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN1_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin1_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN1_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN1_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN1_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin1_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN1_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN1_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN1_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN1_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin1_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN1_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN1_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin1_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN1_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN1_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin1_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN1_WAKEUP_ENABLE_MASK);
    }
}

/// PIN1 value type for batch read/modify/write operations
///
/// Use `pin1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin1Val {
    bits: u32,
}

impl Pin1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN1_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin1_pad_driver(&self) -> bool {
        (self.bits & Pin1::REG_GPIO_PIN1_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN1_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin1_int_type(&self) -> u32 {
        (self.bits & Pin1::REG_GPIO_PIN1_INT_TYPE_MASK) >> Pin1::REG_GPIO_PIN1_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN1_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin1_wakeup_enable(&self) -> bool {
        (self.bits & Pin1::REG_GPIO_PIN1_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI1_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi1_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin1::REG_GPI1_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin1::REG_GPI1_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN1_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin1_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin1::REG_GPIO_PIN1_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin1::REG_GPIO_PIN1_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN1_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin1_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin1::REG_GPIO_PIN1_INT_TYPE_MASK)
                | ((value << Pin1::REG_GPIO_PIN1_INT_TYPE_OFFSET)
                    & Pin1::REG_GPIO_PIN1_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN1_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin1_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin1::REG_GPIO_PIN1_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin1::REG_GPIO_PIN1_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin1Val {
        Pin1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin1Val) -> Pin1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN2 Register
///
/// Reserved
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **REG_GPI2_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN2_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN2_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN2_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin2 {
    // ======== Field Constants ========

    /// **REG_GPI2_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI2_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI2_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI2_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN2_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN2_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN2_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN2_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN2_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN2_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN2_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN2_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN2_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN2_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN2_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI2_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi2_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI2_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN2_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin2_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN2_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN2_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin2_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN2_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN2_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin2_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN2_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN2_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin2_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN2_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN2_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN2_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin2_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN2_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN2_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN2_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN2_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin2_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN2_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN2_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin2_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN2_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN2_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin2_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN2_WAKEUP_ENABLE_MASK);
    }
}

/// PIN2 value type for batch read/modify/write operations
///
/// Use `pin2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin2Val {
    bits: u32,
}

impl Pin2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN2_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin2_pad_driver(&self) -> bool {
        (self.bits & Pin2::REG_GPIO_PIN2_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN2_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin2_int_type(&self) -> u32 {
        (self.bits & Pin2::REG_GPIO_PIN2_INT_TYPE_MASK) >> Pin2::REG_GPIO_PIN2_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN2_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin2_wakeup_enable(&self) -> bool {
        (self.bits & Pin2::REG_GPIO_PIN2_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI2_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi2_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin2::REG_GPI2_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin2::REG_GPI2_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN2_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin2_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin2::REG_GPIO_PIN2_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin2::REG_GPIO_PIN2_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN2_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin2_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin2::REG_GPIO_PIN2_INT_TYPE_MASK)
                | ((value << Pin2::REG_GPIO_PIN2_INT_TYPE_OFFSET)
                    & Pin2::REG_GPIO_PIN2_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN2_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin2_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin2::REG_GPIO_PIN2_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin2::REG_GPIO_PIN2_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin2Val {
        Pin2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin2Val) -> Pin2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN3 Register
///
/// Reserved
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **REG_GPI3_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN3_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN3_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN3_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin3 {
    // ======== Field Constants ========

    /// **REG_GPI3_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI3_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI3_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI3_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN3_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN3_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN3_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN3_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN3_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN3_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN3_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN3_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN3_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN3_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN3_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI3_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi3_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI3_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN3_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin3_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN3_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN3_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin3_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN3_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN3_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin3_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN3_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN3_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin3_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN3_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN3_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN3_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin3_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN3_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN3_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN3_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN3_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin3_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN3_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN3_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin3_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN3_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN3_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin3_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN3_WAKEUP_ENABLE_MASK);
    }
}

/// PIN3 value type for batch read/modify/write operations
///
/// Use `pin3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin3Val {
    bits: u32,
}

impl Pin3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN3_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin3_pad_driver(&self) -> bool {
        (self.bits & Pin3::REG_GPIO_PIN3_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN3_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin3_int_type(&self) -> u32 {
        (self.bits & Pin3::REG_GPIO_PIN3_INT_TYPE_MASK) >> Pin3::REG_GPIO_PIN3_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN3_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin3_wakeup_enable(&self) -> bool {
        (self.bits & Pin3::REG_GPIO_PIN3_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI3_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi3_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin3::REG_GPI3_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin3::REG_GPI3_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN3_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin3_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin3::REG_GPIO_PIN3_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin3::REG_GPIO_PIN3_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN3_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin3_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin3::REG_GPIO_PIN3_INT_TYPE_MASK)
                | ((value << Pin3::REG_GPIO_PIN3_INT_TYPE_OFFSET)
                    & Pin3::REG_GPIO_PIN3_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN3_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin3_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin3::REG_GPIO_PIN3_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin3::REG_GPIO_PIN3_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin3Val {
        Pin3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin3Val) -> Pin3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN4 Register
///
/// Reserved
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **REG_GPI4_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN4_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN4_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN4_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin4 {
    // ======== Field Constants ========

    /// **REG_GPI4_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI4_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI4_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI4_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN4_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN4_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN4_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN4_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN4_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN4_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN4_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN4_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN4_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN4_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN4_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI4_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi4_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI4_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN4_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin4_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN4_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN4_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin4_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN4_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN4_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin4_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN4_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN4_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin4_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN4_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN4_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN4_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin4_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN4_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN4_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN4_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN4_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin4_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN4_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN4_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin4_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN4_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN4_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin4_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN4_WAKEUP_ENABLE_MASK);
    }
}

/// PIN4 value type for batch read/modify/write operations
///
/// Use `pin4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin4Val {
    bits: u32,
}

impl Pin4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN4_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin4_pad_driver(&self) -> bool {
        (self.bits & Pin4::REG_GPIO_PIN4_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN4_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin4_int_type(&self) -> u32 {
        (self.bits & Pin4::REG_GPIO_PIN4_INT_TYPE_MASK) >> Pin4::REG_GPIO_PIN4_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN4_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin4_wakeup_enable(&self) -> bool {
        (self.bits & Pin4::REG_GPIO_PIN4_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI4_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi4_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin4::REG_GPI4_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin4::REG_GPI4_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN4_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin4_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin4::REG_GPIO_PIN4_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin4::REG_GPIO_PIN4_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN4_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin4_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin4::REG_GPIO_PIN4_INT_TYPE_MASK)
                | ((value << Pin4::REG_GPIO_PIN4_INT_TYPE_OFFSET)
                    & Pin4::REG_GPIO_PIN4_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN4_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin4_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin4::REG_GPIO_PIN4_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin4::REG_GPIO_PIN4_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin4Val {
        Pin4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin4Val) -> Pin4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN5 Register
///
/// Reserved
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **REG_GPI5_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN5_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN5_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN5_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin5 {
    // ======== Field Constants ========

    /// **REG_GPI5_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI5_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI5_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI5_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN5_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN5_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN5_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN5_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN5_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN5_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN5_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN5_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN5_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN5_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN5_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI5_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi5_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI5_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN5_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin5_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN5_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN5_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin5_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN5_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN5_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin5_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN5_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN5_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin5_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN5_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN5_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN5_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin5_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN5_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN5_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN5_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN5_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin5_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN5_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN5_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin5_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN5_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN5_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin5_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN5_WAKEUP_ENABLE_MASK);
    }
}

/// PIN5 value type for batch read/modify/write operations
///
/// Use `pin5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin5Val {
    bits: u32,
}

impl Pin5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN5_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin5_pad_driver(&self) -> bool {
        (self.bits & Pin5::REG_GPIO_PIN5_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN5_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin5_int_type(&self) -> u32 {
        (self.bits & Pin5::REG_GPIO_PIN5_INT_TYPE_MASK) >> Pin5::REG_GPIO_PIN5_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN5_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin5_wakeup_enable(&self) -> bool {
        (self.bits & Pin5::REG_GPIO_PIN5_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI5_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi5_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin5::REG_GPI5_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin5::REG_GPI5_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN5_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin5_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin5::REG_GPIO_PIN5_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin5::REG_GPIO_PIN5_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN5_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin5_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin5::REG_GPIO_PIN5_INT_TYPE_MASK)
                | ((value << Pin5::REG_GPIO_PIN5_INT_TYPE_OFFSET)
                    & Pin5::REG_GPIO_PIN5_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN5_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin5_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin5::REG_GPIO_PIN5_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin5::REG_GPIO_PIN5_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin5Val {
        Pin5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin5Val) -> Pin5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN6 Register
///
/// Reserved
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **REG_GPI6_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN6_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN6_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN6_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin6 {
    // ======== Field Constants ========

    /// **REG_GPI6_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI6_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI6_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI6_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN6_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN6_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN6_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN6_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN6_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN6_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN6_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN6_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN6_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN6_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN6_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI6_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi6_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI6_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN6_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin6_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN6_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN6_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin6_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN6_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN6_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin6_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN6_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN6_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin6_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN6_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN6_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN6_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin6_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN6_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN6_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN6_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN6_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin6_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN6_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN6_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin6_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN6_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN6_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin6_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN6_WAKEUP_ENABLE_MASK);
    }
}

/// PIN6 value type for batch read/modify/write operations
///
/// Use `pin6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin6Val {
    bits: u32,
}

impl Pin6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN6_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin6_pad_driver(&self) -> bool {
        (self.bits & Pin6::REG_GPIO_PIN6_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN6_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin6_int_type(&self) -> u32 {
        (self.bits & Pin6::REG_GPIO_PIN6_INT_TYPE_MASK) >> Pin6::REG_GPIO_PIN6_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN6_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin6_wakeup_enable(&self) -> bool {
        (self.bits & Pin6::REG_GPIO_PIN6_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI6_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi6_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin6::REG_GPI6_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin6::REG_GPI6_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN6_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin6_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin6::REG_GPIO_PIN6_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin6::REG_GPIO_PIN6_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN6_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin6_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin6::REG_GPIO_PIN6_INT_TYPE_MASK)
                | ((value << Pin6::REG_GPIO_PIN6_INT_TYPE_OFFSET)
                    & Pin6::REG_GPIO_PIN6_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN6_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin6_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin6::REG_GPIO_PIN6_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin6::REG_GPIO_PIN6_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin6Val {
        Pin6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin6Val) -> Pin6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN7 Register
///
/// Reserved
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **REG_GPI7_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN7_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN7_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN7_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin7 {
    // ======== Field Constants ========

    /// **REG_GPI7_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI7_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI7_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI7_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN7_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN7_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN7_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN7_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN7_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN7_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN7_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN7_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN7_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN7_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN7_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI7_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi7_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI7_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN7_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin7_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN7_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN7_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin7_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN7_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN7_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin7_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN7_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN7_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin7_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN7_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN7_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN7_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin7_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN7_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN7_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN7_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN7_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin7_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN7_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN7_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin7_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN7_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN7_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin7_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN7_WAKEUP_ENABLE_MASK);
    }
}

/// PIN7 value type for batch read/modify/write operations
///
/// Use `pin7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin7Val {
    bits: u32,
}

impl Pin7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN7_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin7_pad_driver(&self) -> bool {
        (self.bits & Pin7::REG_GPIO_PIN7_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN7_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin7_int_type(&self) -> u32 {
        (self.bits & Pin7::REG_GPIO_PIN7_INT_TYPE_MASK) >> Pin7::REG_GPIO_PIN7_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN7_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin7_wakeup_enable(&self) -> bool {
        (self.bits & Pin7::REG_GPIO_PIN7_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI7_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi7_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin7::REG_GPI7_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin7::REG_GPI7_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN7_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin7_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin7::REG_GPIO_PIN7_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin7::REG_GPIO_PIN7_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN7_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin7_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin7::REG_GPIO_PIN7_INT_TYPE_MASK)
                | ((value << Pin7::REG_GPIO_PIN7_INT_TYPE_OFFSET)
                    & Pin7::REG_GPIO_PIN7_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN7_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin7_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin7::REG_GPIO_PIN7_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin7::REG_GPIO_PIN7_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin7Val {
        Pin7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin7Val) -> Pin7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN8 Register
///
/// Reserved
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **REG_GPI8_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN8_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN8_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN8_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin8 {
    // ======== Field Constants ========

    /// **REG_GPI8_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI8_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI8_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI8_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN8_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN8_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN8_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN8_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN8_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN8_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN8_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN8_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN8_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN8_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN8_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI8_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi8_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI8_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN8_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin8_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN8_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN8_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin8_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN8_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN8_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin8_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN8_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN8_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin8_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN8_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN8_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN8_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin8_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN8_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN8_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN8_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN8_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin8_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN8_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN8_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin8_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN8_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN8_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin8_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN8_WAKEUP_ENABLE_MASK);
    }
}

/// PIN8 value type for batch read/modify/write operations
///
/// Use `pin8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin8Val {
    bits: u32,
}

impl Pin8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN8_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin8_pad_driver(&self) -> bool {
        (self.bits & Pin8::REG_GPIO_PIN8_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN8_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin8_int_type(&self) -> u32 {
        (self.bits & Pin8::REG_GPIO_PIN8_INT_TYPE_MASK) >> Pin8::REG_GPIO_PIN8_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN8_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin8_wakeup_enable(&self) -> bool {
        (self.bits & Pin8::REG_GPIO_PIN8_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI8_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi8_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin8::REG_GPI8_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin8::REG_GPI8_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN8_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin8_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin8::REG_GPIO_PIN8_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin8::REG_GPIO_PIN8_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN8_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin8_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin8::REG_GPIO_PIN8_INT_TYPE_MASK)
                | ((value << Pin8::REG_GPIO_PIN8_INT_TYPE_OFFSET)
                    & Pin8::REG_GPIO_PIN8_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN8_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin8_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin8::REG_GPIO_PIN8_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin8::REG_GPIO_PIN8_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin8Val {
        Pin8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin8Val) -> Pin8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN9 Register
///
/// Reserved
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **REG_GPI9_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN9_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN9_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN9_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin9 {
    // ======== Field Constants ========

    /// **REG_GPI9_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI9_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI9_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI9_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN9_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN9_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN9_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN9_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN9_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN9_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN9_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN9_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN9_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN9_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN9_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI9_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi9_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI9_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN9_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin9_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN9_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN9_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin9_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN9_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN9_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin9_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN9_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN9_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin9_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN9_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN9_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN9_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin9_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN9_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN9_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN9_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN9_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin9_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN9_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN9_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin9_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN9_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN9_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin9_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN9_WAKEUP_ENABLE_MASK);
    }
}

/// PIN9 value type for batch read/modify/write operations
///
/// Use `pin9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin9Val {
    bits: u32,
}

impl Pin9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN9_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin9_pad_driver(&self) -> bool {
        (self.bits & Pin9::REG_GPIO_PIN9_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN9_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin9_int_type(&self) -> u32 {
        (self.bits & Pin9::REG_GPIO_PIN9_INT_TYPE_MASK) >> Pin9::REG_GPIO_PIN9_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN9_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin9_wakeup_enable(&self) -> bool {
        (self.bits & Pin9::REG_GPIO_PIN9_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI9_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi9_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin9::REG_GPI9_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin9::REG_GPI9_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN9_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin9_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin9::REG_GPIO_PIN9_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin9::REG_GPIO_PIN9_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN9_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin9_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin9::REG_GPIO_PIN9_INT_TYPE_MASK)
                | ((value << Pin9::REG_GPIO_PIN9_INT_TYPE_OFFSET)
                    & Pin9::REG_GPIO_PIN9_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN9_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin9_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin9::REG_GPIO_PIN9_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin9::REG_GPIO_PIN9_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin9Val {
        Pin9Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin9Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin9Val) -> Pin9Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN10 Register
///
/// Reserved
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **REG_GPI10_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN10_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN10_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN10_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin10 {
    // ======== Field Constants ========

    /// **REG_GPI10_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI10_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI10_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI10_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN10_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN10_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN10_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN10_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN10_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN10_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN10_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN10_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN10_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN10_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN10_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI10_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi10_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI10_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN10_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin10_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN10_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN10_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin10_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN10_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN10_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin10_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN10_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN10_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin10_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN10_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN10_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN10_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin10_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN10_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN10_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN10_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN10_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin10_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN10_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN10_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin10_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN10_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN10_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin10_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN10_WAKEUP_ENABLE_MASK);
    }
}

/// PIN10 value type for batch read/modify/write operations
///
/// Use `pin10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin10Val {
    bits: u32,
}

impl Pin10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN10_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin10_pad_driver(&self) -> bool {
        (self.bits & Pin10::REG_GPIO_PIN10_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN10_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin10_int_type(&self) -> u32 {
        (self.bits & Pin10::REG_GPIO_PIN10_INT_TYPE_MASK) >> Pin10::REG_GPIO_PIN10_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN10_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin10_wakeup_enable(&self) -> bool {
        (self.bits & Pin10::REG_GPIO_PIN10_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI10_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi10_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin10::REG_GPI10_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin10::REG_GPI10_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN10_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin10_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin10::REG_GPIO_PIN10_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin10::REG_GPIO_PIN10_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN10_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin10_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin10::REG_GPIO_PIN10_INT_TYPE_MASK)
                | ((value << Pin10::REG_GPIO_PIN10_INT_TYPE_OFFSET)
                    & Pin10::REG_GPIO_PIN10_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN10_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin10_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin10::REG_GPIO_PIN10_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin10::REG_GPIO_PIN10_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin10Val {
        Pin10Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin10Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin10Val) -> Pin10Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN11 Register
///
/// Reserved
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **REG_GPI11_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN11_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN11_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN11_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin11 {
    // ======== Field Constants ========

    /// **REG_GPI11_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI11_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI11_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI11_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN11_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN11_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN11_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN11_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN11_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN11_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN11_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN11_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN11_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN11_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN11_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI11_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi11_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI11_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN11_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin11_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN11_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN11_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin11_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN11_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN11_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin11_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN11_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN11_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin11_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN11_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN11_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN11_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin11_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN11_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN11_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN11_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN11_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin11_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN11_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN11_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin11_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN11_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN11_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin11_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN11_WAKEUP_ENABLE_MASK);
    }
}

/// PIN11 value type for batch read/modify/write operations
///
/// Use `pin11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin11Val {
    bits: u32,
}

impl Pin11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN11_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin11_pad_driver(&self) -> bool {
        (self.bits & Pin11::REG_GPIO_PIN11_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN11_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin11_int_type(&self) -> u32 {
        (self.bits & Pin11::REG_GPIO_PIN11_INT_TYPE_MASK) >> Pin11::REG_GPIO_PIN11_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN11_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin11_wakeup_enable(&self) -> bool {
        (self.bits & Pin11::REG_GPIO_PIN11_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI11_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi11_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin11::REG_GPI11_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin11::REG_GPI11_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN11_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin11_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin11::REG_GPIO_PIN11_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin11::REG_GPIO_PIN11_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN11_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin11_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin11::REG_GPIO_PIN11_INT_TYPE_MASK)
                | ((value << Pin11::REG_GPIO_PIN11_INT_TYPE_OFFSET)
                    & Pin11::REG_GPIO_PIN11_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN11_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin11_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin11::REG_GPIO_PIN11_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin11::REG_GPIO_PIN11_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin11Val {
        Pin11Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin11Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin11Val) -> Pin11Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN12 Register
///
/// Reserved
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **REG_GPI12_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN12_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN12_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN12_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin12 {
    // ======== Field Constants ========

    /// **REG_GPI12_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI12_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI12_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI12_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN12_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN12_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN12_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN12_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN12_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN12_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN12_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN12_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN12_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN12_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN12_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI12_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi12_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI12_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN12_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin12_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN12_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN12_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin12_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN12_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN12_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin12_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN12_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN12_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin12_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN12_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN12_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN12_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin12_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN12_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN12_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN12_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN12_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin12_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN12_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN12_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin12_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN12_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN12_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin12_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN12_WAKEUP_ENABLE_MASK);
    }
}

/// PIN12 value type for batch read/modify/write operations
///
/// Use `pin12.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin12.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin12Val {
    bits: u32,
}

impl Pin12Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN12_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin12_pad_driver(&self) -> bool {
        (self.bits & Pin12::REG_GPIO_PIN12_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN12_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin12_int_type(&self) -> u32 {
        (self.bits & Pin12::REG_GPIO_PIN12_INT_TYPE_MASK) >> Pin12::REG_GPIO_PIN12_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN12_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin12_wakeup_enable(&self) -> bool {
        (self.bits & Pin12::REG_GPIO_PIN12_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI12_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi12_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin12::REG_GPI12_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin12::REG_GPI12_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN12_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin12_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin12::REG_GPIO_PIN12_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin12::REG_GPIO_PIN12_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN12_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin12_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin12::REG_GPIO_PIN12_INT_TYPE_MASK)
                | ((value << Pin12::REG_GPIO_PIN12_INT_TYPE_OFFSET)
                    & Pin12::REG_GPIO_PIN12_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN12_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin12_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin12::REG_GPIO_PIN12_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin12::REG_GPIO_PIN12_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin12Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin12 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin12Val {
        Pin12Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin12Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin12Val) -> Pin12Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN13 Register
///
/// Reserved
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **REG_GPI13_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN13_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN13_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN13_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin13 {
    // ======== Field Constants ========

    /// **REG_GPI13_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI13_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI13_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI13_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN13_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN13_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN13_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN13_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN13_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN13_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN13_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN13_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN13_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN13_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN13_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI13_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi13_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI13_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN13_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin13_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN13_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN13_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin13_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN13_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN13_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin13_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN13_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN13_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin13_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN13_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN13_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN13_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin13_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN13_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN13_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN13_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN13_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin13_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN13_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN13_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin13_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN13_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN13_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin13_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN13_WAKEUP_ENABLE_MASK);
    }
}

/// PIN13 value type for batch read/modify/write operations
///
/// Use `pin13.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin13.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin13Val {
    bits: u32,
}

impl Pin13Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN13_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin13_pad_driver(&self) -> bool {
        (self.bits & Pin13::REG_GPIO_PIN13_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN13_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin13_int_type(&self) -> u32 {
        (self.bits & Pin13::REG_GPIO_PIN13_INT_TYPE_MASK) >> Pin13::REG_GPIO_PIN13_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN13_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin13_wakeup_enable(&self) -> bool {
        (self.bits & Pin13::REG_GPIO_PIN13_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI13_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi13_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin13::REG_GPI13_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin13::REG_GPI13_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN13_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin13_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin13::REG_GPIO_PIN13_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin13::REG_GPIO_PIN13_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN13_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin13_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin13::REG_GPIO_PIN13_INT_TYPE_MASK)
                | ((value << Pin13::REG_GPIO_PIN13_INT_TYPE_OFFSET)
                    & Pin13::REG_GPIO_PIN13_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN13_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin13_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin13::REG_GPIO_PIN13_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin13::REG_GPIO_PIN13_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin13Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin13 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin13Val {
        Pin13Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin13Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin13Val) -> Pin13Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN14 Register
///
/// Reserved
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **REG_GPI14_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN14_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN14_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN14_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin14 {
    // ======== Field Constants ========

    /// **REG_GPI14_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI14_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI14_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI14_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN14_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN14_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN14_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN14_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN14_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN14_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN14_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN14_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN14_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN14_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN14_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI14_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi14_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI14_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN14_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin14_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN14_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN14_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin14_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN14_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN14_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin14_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN14_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN14_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin14_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN14_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN14_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN14_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin14_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN14_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN14_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN14_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN14_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin14_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN14_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN14_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin14_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN14_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN14_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin14_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN14_WAKEUP_ENABLE_MASK);
    }
}

/// PIN14 value type for batch read/modify/write operations
///
/// Use `pin14.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin14.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin14Val {
    bits: u32,
}

impl Pin14Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN14_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin14_pad_driver(&self) -> bool {
        (self.bits & Pin14::REG_GPIO_PIN14_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN14_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin14_int_type(&self) -> u32 {
        (self.bits & Pin14::REG_GPIO_PIN14_INT_TYPE_MASK) >> Pin14::REG_GPIO_PIN14_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN14_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin14_wakeup_enable(&self) -> bool {
        (self.bits & Pin14::REG_GPIO_PIN14_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI14_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi14_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin14::REG_GPI14_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin14::REG_GPI14_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN14_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin14_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin14::REG_GPIO_PIN14_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin14::REG_GPIO_PIN14_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN14_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin14_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin14::REG_GPIO_PIN14_INT_TYPE_MASK)
                | ((value << Pin14::REG_GPIO_PIN14_INT_TYPE_OFFSET)
                    & Pin14::REG_GPIO_PIN14_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN14_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin14_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin14::REG_GPIO_PIN14_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin14::REG_GPIO_PIN14_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin14Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin14 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin14Val {
        Pin14Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin14Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin14Val) -> Pin14Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PIN15 Register
///
/// Reserved
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **REG_GPI15_PIN0_EDGE_WAKEUP_CLR** `[5]` (WO)
///
///   need des
///
/// - **REG_GPIO_PIN15_PAD_DRIVER** `[4]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN15_INT_TYPE** `[3:1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_PIN15_WAKEUP_ENABLE** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pin15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pin15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pin15 {
    // ======== Field Constants ========

    /// **REG_GPI15_PIN0_EDGE_WAKEUP_CLR**
    ///
    /// need des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_GPI15_PIN0_EDGE_WAKEUP_CLR_OFFSET: u32 = 5;
    pub const REG_GPI15_PIN0_EDGE_WAKEUP_CLR_MASK: u32 = 0x00000020;
    pub const REG_GPI15_PIN0_EDGE_WAKEUP_CLR: u32 = 1 << 5;

    /// **REG_GPIO_PIN15_PAD_DRIVER**
    ///
    /// Reserved
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_GPIO_PIN15_PAD_DRIVER_OFFSET: u32 = 4;
    pub const REG_GPIO_PIN15_PAD_DRIVER_MASK: u32 = 0x00000010;
    pub const REG_GPIO_PIN15_PAD_DRIVER: u32 = 1 << 4;

    /// **REG_GPIO_PIN15_INT_TYPE**
    ///
    /// Reserved
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const REG_GPIO_PIN15_INT_TYPE_OFFSET: u32 = 1;
    pub const REG_GPIO_PIN15_INT_TYPE_MASK: u32 = 0x0000000e;

    /// **REG_GPIO_PIN15_WAKEUP_ENABLE**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_PIN15_WAKEUP_ENABLE_OFFSET: u32 = 0;
    pub const REG_GPIO_PIN15_WAKEUP_ENABLE_MASK: u32 = 0x00000001;
    pub const REG_GPIO_PIN15_WAKEUP_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_GPI15_PIN0_EDGE_WAKEUP_CLR** (WO)
    ///
    /// need des
    #[inline(always)]
    pub fn trigger_reg_gpi15_pin0_edge_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPI15_PIN0_EDGE_WAKEUP_CLR_MASK);
    }

    /// **REG_GPIO_PIN15_PAD_DRIVER** `[4]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin15_pad_driver(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN15_PAD_DRIVER_MASK) != 0
    }

    /// Set REG_GPIO_PIN15_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin15_pad_driver(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN15_PAD_DRIVER_MASK);
    }

    /// Clear REG_GPIO_PIN15_PAD_DRIVER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin15_pad_driver(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN15_PAD_DRIVER_MASK);
    }

    /// **REG_GPIO_PIN15_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin15_int_type(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_PIN15_INT_TYPE_MASK)
            >> Self::REG_GPIO_PIN15_INT_TYPE_OFFSET
    }

    /// Write **REG_GPIO_PIN15_INT_TYPE** `[3:1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_pin15_int_type(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_PIN15_INT_TYPE_MASK)
                | ((value << Self::REG_GPIO_PIN15_INT_TYPE_OFFSET)
                    & Self::REG_GPIO_PIN15_INT_TYPE_MASK)
        });
    }

    /// **REG_GPIO_PIN15_WAKEUP_ENABLE** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_pin15_wakeup_enable(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_PIN15_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set REG_GPIO_PIN15_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_pin15_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_PIN15_WAKEUP_ENABLE_MASK);
    }

    /// Clear REG_GPIO_PIN15_WAKEUP_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_pin15_wakeup_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_PIN15_WAKEUP_ENABLE_MASK);
    }
}

/// PIN15 value type for batch read/modify/write operations
///
/// Use `pin15.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pin15.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pin15Val {
    bits: u32,
}

impl Pin15Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_GPIO_PIN15_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin15_pad_driver(&self) -> bool {
        (self.bits & Pin15::REG_GPIO_PIN15_PAD_DRIVER_MASK) != 0
    }

    /// Read **REG_GPIO_PIN15_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin15_int_type(&self) -> u32 {
        (self.bits & Pin15::REG_GPIO_PIN15_INT_TYPE_MASK) >> Pin15::REG_GPIO_PIN15_INT_TYPE_OFFSET
    }

    /// Read **REG_GPIO_PIN15_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_pin15_wakeup_enable(&self) -> bool {
        (self.bits & Pin15::REG_GPIO_PIN15_WAKEUP_ENABLE_MASK) != 0
    }

    /// Set **REG_GPI15_PIN0_EDGE_WAKEUP_CLR** `[5:5]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_gpi15_pin0_edge_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin15::REG_GPI15_PIN0_EDGE_WAKEUP_CLR_MASK
            } else {
                self.bits & !Pin15::REG_GPI15_PIN0_EDGE_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN15_PAD_DRIVER** `[4:4]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin15_pad_driver(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin15::REG_GPIO_PIN15_PAD_DRIVER_MASK
            } else {
                self.bits & !Pin15::REG_GPIO_PIN15_PAD_DRIVER_MASK
            },
        }
    }

    /// Set **REG_GPIO_PIN15_INT_TYPE** `[3:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin15_int_type(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pin15::REG_GPIO_PIN15_INT_TYPE_MASK)
                | ((value << Pin15::REG_GPIO_PIN15_INT_TYPE_OFFSET)
                    & Pin15::REG_GPIO_PIN15_INT_TYPE_MASK),
        }
    }

    /// Set **REG_GPIO_PIN15_WAKEUP_ENABLE** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_pin15_wakeup_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pin15::REG_GPIO_PIN15_WAKEUP_ENABLE_MASK
            } else {
                self.bits & !Pin15::REG_GPIO_PIN15_WAKEUP_ENABLE_MASK
            },
        }
    }
}

impl Default for Pin15Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pin15 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pin15Val {
        Pin15Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pin15Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pin15Val) -> Pin15Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC0_IN_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0074
/// Reset value: 0x000000c0
///
/// ## Fields
///
/// - **REG_GPIO_FUNC0_IN_SEL** `[7:2]` (RW)
///
///   reg_gpio_func0_in_sel[5:4]==2'b11->constant 1,reg_gpio_func0_in_sel[5:4]==2'b10->constant 0
///
/// - **REG_GPIO_SIG0_IN_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC0_IN_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func0InSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func0InSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func0InSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC0_IN_SEL**
    ///
    /// reg_gpio_func0_in_sel[5:4]==2'b11->constant 1,reg_gpio_func0_in_sel[5:4]==2'b10->constant 0
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const REG_GPIO_FUNC0_IN_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC0_IN_SEL_MASK: u32 = 0x000000fc;

    /// **REG_GPIO_SIG0_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_SIG0_IN_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_SIG0_IN_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_SIG0_IN_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC0_IN_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC0_IN_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC0_IN_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC0_IN_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC0_IN_SEL** `[7:2]` (RW)
    ///
    /// reg_gpio_func0_in_sel[5:4]==2'b11->constant 1,reg_gpio_func0_in_sel[5:4]==2'b10->constant 0
    #[inline(always)]
    pub fn reg_gpio_func0_in_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC0_IN_SEL_MASK)
            >> Self::REG_GPIO_FUNC0_IN_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC0_IN_SEL** `[7:2]` (RW)
    ///
    /// reg_gpio_func0_in_sel[5:4]==2'b11->constant 1,reg_gpio_func0_in_sel[5:4]==2'b10->constant 0
    #[inline(always)]
    pub fn write_reg_gpio_func0_in_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC0_IN_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC0_IN_SEL_OFFSET) & Self::REG_GPIO_FUNC0_IN_SEL_MASK)
        });
    }

    /// **REG_GPIO_SIG0_IN_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_sig0_in_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_SIG0_IN_SEL_MASK) != 0
    }

    /// Set REG_GPIO_SIG0_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_sig0_in_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_SIG0_IN_SEL_MASK);
    }

    /// Clear REG_GPIO_SIG0_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_sig0_in_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_SIG0_IN_SEL_MASK);
    }

    /// **REG_GPIO_FUNC0_IN_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func0_in_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC0_IN_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC0_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func0_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC0_IN_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC0_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func0_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC0_IN_INV_SEL_MASK);
    }
}

/// FUNC0_IN_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func0inselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func0inselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func0InSelCfgVal {
    bits: u32,
}

impl Func0InSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x000000c0 }
    }

    /// Read **REG_GPIO_FUNC0_IN_SEL** `[7:2]`
    ///
    /// reg_gpio_func0_in_sel[5:4]==2'b11->constant 1,reg_gpio_func0_in_sel[5:4]==2'b10->constant 0
    #[inline(always)]
    pub const fn reg_gpio_func0_in_sel(&self) -> u32 {
        (self.bits & Func0InSelCfg::REG_GPIO_FUNC0_IN_SEL_MASK)
            >> Func0InSelCfg::REG_GPIO_FUNC0_IN_SEL_OFFSET
    }

    /// Read **REG_GPIO_SIG0_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_sig0_in_sel(&self) -> bool {
        (self.bits & Func0InSelCfg::REG_GPIO_SIG0_IN_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC0_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func0_in_inv_sel(&self) -> bool {
        (self.bits & Func0InSelCfg::REG_GPIO_FUNC0_IN_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC0_IN_SEL** `[7:2]`
    ///
    /// reg_gpio_func0_in_sel[5:4]==2'b11->constant 1,reg_gpio_func0_in_sel[5:4]==2'b10->constant 0
    #[inline(always)]
    pub const fn with_reg_gpio_func0_in_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func0InSelCfg::REG_GPIO_FUNC0_IN_SEL_MASK)
                | ((value << Func0InSelCfg::REG_GPIO_FUNC0_IN_SEL_OFFSET)
                    & Func0InSelCfg::REG_GPIO_FUNC0_IN_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_SIG0_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_sig0_in_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func0InSelCfg::REG_GPIO_SIG0_IN_SEL_MASK
            } else {
                self.bits & !Func0InSelCfg::REG_GPIO_SIG0_IN_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC0_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func0_in_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func0InSelCfg::REG_GPIO_FUNC0_IN_INV_SEL_MASK
            } else {
                self.bits & !Func0InSelCfg::REG_GPIO_FUNC0_IN_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func0InSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func0InSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func0InSelCfgVal {
        Func0InSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func0InSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func0InSelCfgVal) -> Func0InSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC1_IN_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0078
/// Reset value: 0x000000c0
///
/// ## Fields
///
/// - **REG_GPIO_FUNC1_IN_SEL** `[7:2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_SIG1_IN_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC1_IN_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func1InSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func1InSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func1InSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC1_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const REG_GPIO_FUNC1_IN_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC1_IN_SEL_MASK: u32 = 0x000000fc;

    /// **REG_GPIO_SIG1_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_SIG1_IN_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_SIG1_IN_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_SIG1_IN_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC1_IN_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC1_IN_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC1_IN_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC1_IN_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC1_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func1_in_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC1_IN_SEL_MASK)
            >> Self::REG_GPIO_FUNC1_IN_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC1_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func1_in_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC1_IN_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC1_IN_SEL_OFFSET) & Self::REG_GPIO_FUNC1_IN_SEL_MASK)
        });
    }

    /// **REG_GPIO_SIG1_IN_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_sig1_in_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_SIG1_IN_SEL_MASK) != 0
    }

    /// Set REG_GPIO_SIG1_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_sig1_in_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_SIG1_IN_SEL_MASK);
    }

    /// Clear REG_GPIO_SIG1_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_sig1_in_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_SIG1_IN_SEL_MASK);
    }

    /// **REG_GPIO_FUNC1_IN_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func1_in_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC1_IN_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC1_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func1_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC1_IN_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC1_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func1_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC1_IN_INV_SEL_MASK);
    }
}

/// FUNC1_IN_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func1inselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func1inselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func1InSelCfgVal {
    bits: u32,
}

impl Func1InSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x000000c0 }
    }

    /// Read **REG_GPIO_FUNC1_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func1_in_sel(&self) -> u32 {
        (self.bits & Func1InSelCfg::REG_GPIO_FUNC1_IN_SEL_MASK)
            >> Func1InSelCfg::REG_GPIO_FUNC1_IN_SEL_OFFSET
    }

    /// Read **REG_GPIO_SIG1_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_sig1_in_sel(&self) -> bool {
        (self.bits & Func1InSelCfg::REG_GPIO_SIG1_IN_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC1_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func1_in_inv_sel(&self) -> bool {
        (self.bits & Func1InSelCfg::REG_GPIO_FUNC1_IN_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC1_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func1_in_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func1InSelCfg::REG_GPIO_FUNC1_IN_SEL_MASK)
                | ((value << Func1InSelCfg::REG_GPIO_FUNC1_IN_SEL_OFFSET)
                    & Func1InSelCfg::REG_GPIO_FUNC1_IN_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_SIG1_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_sig1_in_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func1InSelCfg::REG_GPIO_SIG1_IN_SEL_MASK
            } else {
                self.bits & !Func1InSelCfg::REG_GPIO_SIG1_IN_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC1_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func1_in_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func1InSelCfg::REG_GPIO_FUNC1_IN_INV_SEL_MASK
            } else {
                self.bits & !Func1InSelCfg::REG_GPIO_FUNC1_IN_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func1InSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func1InSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func1InSelCfgVal {
        Func1InSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func1InSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func1InSelCfgVal) -> Func1InSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC2_IN_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x007c
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **REG_GPIO_FUNC2_IN_SEL** `[7:2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_SIG2_IN_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC2_IN_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func2InSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func2InSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func2InSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC2_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const REG_GPIO_FUNC2_IN_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC2_IN_SEL_MASK: u32 = 0x000000fc;

    /// **REG_GPIO_SIG2_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_SIG2_IN_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_SIG2_IN_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_SIG2_IN_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC2_IN_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC2_IN_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC2_IN_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC2_IN_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC2_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func2_in_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC2_IN_SEL_MASK)
            >> Self::REG_GPIO_FUNC2_IN_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC2_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func2_in_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC2_IN_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC2_IN_SEL_OFFSET) & Self::REG_GPIO_FUNC2_IN_SEL_MASK)
        });
    }

    /// **REG_GPIO_SIG2_IN_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_sig2_in_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_SIG2_IN_SEL_MASK) != 0
    }

    /// Set REG_GPIO_SIG2_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_sig2_in_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_SIG2_IN_SEL_MASK);
    }

    /// Clear REG_GPIO_SIG2_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_sig2_in_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_SIG2_IN_SEL_MASK);
    }

    /// **REG_GPIO_FUNC2_IN_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func2_in_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC2_IN_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC2_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func2_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC2_IN_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC2_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func2_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC2_IN_INV_SEL_MASK);
    }
}

/// FUNC2_IN_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func2inselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func2inselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func2InSelCfgVal {
    bits: u32,
}

impl Func2InSelCfgVal {
    /// Create from raw bits
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

    /// Read **REG_GPIO_FUNC2_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func2_in_sel(&self) -> u32 {
        (self.bits & Func2InSelCfg::REG_GPIO_FUNC2_IN_SEL_MASK)
            >> Func2InSelCfg::REG_GPIO_FUNC2_IN_SEL_OFFSET
    }

    /// Read **REG_GPIO_SIG2_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_sig2_in_sel(&self) -> bool {
        (self.bits & Func2InSelCfg::REG_GPIO_SIG2_IN_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC2_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func2_in_inv_sel(&self) -> bool {
        (self.bits & Func2InSelCfg::REG_GPIO_FUNC2_IN_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC2_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func2_in_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func2InSelCfg::REG_GPIO_FUNC2_IN_SEL_MASK)
                | ((value << Func2InSelCfg::REG_GPIO_FUNC2_IN_SEL_OFFSET)
                    & Func2InSelCfg::REG_GPIO_FUNC2_IN_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_SIG2_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_sig2_in_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func2InSelCfg::REG_GPIO_SIG2_IN_SEL_MASK
            } else {
                self.bits & !Func2InSelCfg::REG_GPIO_SIG2_IN_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC2_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func2_in_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func2InSelCfg::REG_GPIO_FUNC2_IN_INV_SEL_MASK
            } else {
                self.bits & !Func2InSelCfg::REG_GPIO_FUNC2_IN_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func2InSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func2InSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func2InSelCfgVal {
        Func2InSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func2InSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func2InSelCfgVal) -> Func2InSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC3_IN_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0080
/// Reset value: 0x000000c0
///
/// ## Fields
///
/// - **REG_GPIO_FUNC3_IN_SEL** `[7:2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_SIG3_IN_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC3_IN_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func3InSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func3InSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func3InSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC3_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const REG_GPIO_FUNC3_IN_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC3_IN_SEL_MASK: u32 = 0x000000fc;

    /// **REG_GPIO_SIG3_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_SIG3_IN_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_SIG3_IN_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_SIG3_IN_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC3_IN_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC3_IN_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC3_IN_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC3_IN_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC3_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func3_in_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC3_IN_SEL_MASK)
            >> Self::REG_GPIO_FUNC3_IN_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC3_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func3_in_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC3_IN_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC3_IN_SEL_OFFSET) & Self::REG_GPIO_FUNC3_IN_SEL_MASK)
        });
    }

    /// **REG_GPIO_SIG3_IN_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_sig3_in_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_SIG3_IN_SEL_MASK) != 0
    }

    /// Set REG_GPIO_SIG3_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_sig3_in_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_SIG3_IN_SEL_MASK);
    }

    /// Clear REG_GPIO_SIG3_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_sig3_in_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_SIG3_IN_SEL_MASK);
    }

    /// **REG_GPIO_FUNC3_IN_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func3_in_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC3_IN_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC3_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func3_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC3_IN_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC3_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func3_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC3_IN_INV_SEL_MASK);
    }
}

/// FUNC3_IN_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func3inselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func3inselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func3InSelCfgVal {
    bits: u32,
}

impl Func3InSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x000000c0 }
    }

    /// Read **REG_GPIO_FUNC3_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func3_in_sel(&self) -> u32 {
        (self.bits & Func3InSelCfg::REG_GPIO_FUNC3_IN_SEL_MASK)
            >> Func3InSelCfg::REG_GPIO_FUNC3_IN_SEL_OFFSET
    }

    /// Read **REG_GPIO_SIG3_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_sig3_in_sel(&self) -> bool {
        (self.bits & Func3InSelCfg::REG_GPIO_SIG3_IN_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC3_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func3_in_inv_sel(&self) -> bool {
        (self.bits & Func3InSelCfg::REG_GPIO_FUNC3_IN_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC3_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func3_in_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func3InSelCfg::REG_GPIO_FUNC3_IN_SEL_MASK)
                | ((value << Func3InSelCfg::REG_GPIO_FUNC3_IN_SEL_OFFSET)
                    & Func3InSelCfg::REG_GPIO_FUNC3_IN_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_SIG3_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_sig3_in_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func3InSelCfg::REG_GPIO_SIG3_IN_SEL_MASK
            } else {
                self.bits & !Func3InSelCfg::REG_GPIO_SIG3_IN_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC3_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func3_in_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func3InSelCfg::REG_GPIO_FUNC3_IN_INV_SEL_MASK
            } else {
                self.bits & !Func3InSelCfg::REG_GPIO_FUNC3_IN_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func3InSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func3InSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func3InSelCfgVal {
        Func3InSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func3InSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func3InSelCfgVal) -> Func3InSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC4_IN_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0084
/// Reset value: 0x000000c0
///
/// ## Fields
///
/// - **REG_GPIO_FUNC4_IN_SEL** `[7:2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_SIG4_IN_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC4_IN_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func4InSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func4InSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func4InSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC4_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const REG_GPIO_FUNC4_IN_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC4_IN_SEL_MASK: u32 = 0x000000fc;

    /// **REG_GPIO_SIG4_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_SIG4_IN_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_SIG4_IN_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_SIG4_IN_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC4_IN_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC4_IN_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC4_IN_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC4_IN_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC4_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func4_in_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC4_IN_SEL_MASK)
            >> Self::REG_GPIO_FUNC4_IN_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC4_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func4_in_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC4_IN_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC4_IN_SEL_OFFSET) & Self::REG_GPIO_FUNC4_IN_SEL_MASK)
        });
    }

    /// **REG_GPIO_SIG4_IN_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_sig4_in_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_SIG4_IN_SEL_MASK) != 0
    }

    /// Set REG_GPIO_SIG4_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_sig4_in_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_SIG4_IN_SEL_MASK);
    }

    /// Clear REG_GPIO_SIG4_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_sig4_in_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_SIG4_IN_SEL_MASK);
    }

    /// **REG_GPIO_FUNC4_IN_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func4_in_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC4_IN_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC4_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func4_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC4_IN_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC4_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func4_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC4_IN_INV_SEL_MASK);
    }
}

/// FUNC4_IN_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func4inselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func4inselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func4InSelCfgVal {
    bits: u32,
}

impl Func4InSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x000000c0 }
    }

    /// Read **REG_GPIO_FUNC4_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func4_in_sel(&self) -> u32 {
        (self.bits & Func4InSelCfg::REG_GPIO_FUNC4_IN_SEL_MASK)
            >> Func4InSelCfg::REG_GPIO_FUNC4_IN_SEL_OFFSET
    }

    /// Read **REG_GPIO_SIG4_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_sig4_in_sel(&self) -> bool {
        (self.bits & Func4InSelCfg::REG_GPIO_SIG4_IN_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC4_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func4_in_inv_sel(&self) -> bool {
        (self.bits & Func4InSelCfg::REG_GPIO_FUNC4_IN_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC4_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func4_in_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func4InSelCfg::REG_GPIO_FUNC4_IN_SEL_MASK)
                | ((value << Func4InSelCfg::REG_GPIO_FUNC4_IN_SEL_OFFSET)
                    & Func4InSelCfg::REG_GPIO_FUNC4_IN_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_SIG4_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_sig4_in_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func4InSelCfg::REG_GPIO_SIG4_IN_SEL_MASK
            } else {
                self.bits & !Func4InSelCfg::REG_GPIO_SIG4_IN_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC4_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func4_in_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func4InSelCfg::REG_GPIO_FUNC4_IN_INV_SEL_MASK
            } else {
                self.bits & !Func4InSelCfg::REG_GPIO_FUNC4_IN_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func4InSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func4InSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func4InSelCfgVal {
        Func4InSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func4InSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func4InSelCfgVal) -> Func4InSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC5_IN_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0088
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **REG_GPIO_FUNC5_IN_SEL** `[7:2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_SIG5_IN_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC5_IN_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func5InSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func5InSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func5InSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC5_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const REG_GPIO_FUNC5_IN_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC5_IN_SEL_MASK: u32 = 0x000000fc;

    /// **REG_GPIO_SIG5_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_SIG5_IN_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_SIG5_IN_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_SIG5_IN_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC5_IN_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC5_IN_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC5_IN_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC5_IN_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC5_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func5_in_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC5_IN_SEL_MASK)
            >> Self::REG_GPIO_FUNC5_IN_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC5_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func5_in_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC5_IN_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC5_IN_SEL_OFFSET) & Self::REG_GPIO_FUNC5_IN_SEL_MASK)
        });
    }

    /// **REG_GPIO_SIG5_IN_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_sig5_in_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_SIG5_IN_SEL_MASK) != 0
    }

    /// Set REG_GPIO_SIG5_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_sig5_in_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_SIG5_IN_SEL_MASK);
    }

    /// Clear REG_GPIO_SIG5_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_sig5_in_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_SIG5_IN_SEL_MASK);
    }

    /// **REG_GPIO_FUNC5_IN_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func5_in_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC5_IN_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC5_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func5_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC5_IN_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC5_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func5_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC5_IN_INV_SEL_MASK);
    }
}

/// FUNC5_IN_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func5inselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func5inselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func5InSelCfgVal {
    bits: u32,
}

impl Func5InSelCfgVal {
    /// Create from raw bits
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

    /// Read **REG_GPIO_FUNC5_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func5_in_sel(&self) -> u32 {
        (self.bits & Func5InSelCfg::REG_GPIO_FUNC5_IN_SEL_MASK)
            >> Func5InSelCfg::REG_GPIO_FUNC5_IN_SEL_OFFSET
    }

    /// Read **REG_GPIO_SIG5_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_sig5_in_sel(&self) -> bool {
        (self.bits & Func5InSelCfg::REG_GPIO_SIG5_IN_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC5_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func5_in_inv_sel(&self) -> bool {
        (self.bits & Func5InSelCfg::REG_GPIO_FUNC5_IN_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC5_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func5_in_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func5InSelCfg::REG_GPIO_FUNC5_IN_SEL_MASK)
                | ((value << Func5InSelCfg::REG_GPIO_FUNC5_IN_SEL_OFFSET)
                    & Func5InSelCfg::REG_GPIO_FUNC5_IN_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_SIG5_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_sig5_in_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func5InSelCfg::REG_GPIO_SIG5_IN_SEL_MASK
            } else {
                self.bits & !Func5InSelCfg::REG_GPIO_SIG5_IN_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC5_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func5_in_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func5InSelCfg::REG_GPIO_FUNC5_IN_INV_SEL_MASK
            } else {
                self.bits & !Func5InSelCfg::REG_GPIO_FUNC5_IN_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func5InSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func5InSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func5InSelCfgVal {
        Func5InSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func5InSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func5InSelCfgVal) -> Func5InSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC6_IN_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x008c
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **REG_GPIO_FUNC6_IN_SEL** `[7:2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_SIG6_IN_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC6_IN_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func6InSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func6InSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func6InSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC6_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const REG_GPIO_FUNC6_IN_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC6_IN_SEL_MASK: u32 = 0x000000fc;

    /// **REG_GPIO_SIG6_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_SIG6_IN_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_SIG6_IN_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_SIG6_IN_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC6_IN_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC6_IN_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC6_IN_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC6_IN_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC6_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func6_in_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC6_IN_SEL_MASK)
            >> Self::REG_GPIO_FUNC6_IN_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC6_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func6_in_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC6_IN_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC6_IN_SEL_OFFSET) & Self::REG_GPIO_FUNC6_IN_SEL_MASK)
        });
    }

    /// **REG_GPIO_SIG6_IN_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_sig6_in_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_SIG6_IN_SEL_MASK) != 0
    }

    /// Set REG_GPIO_SIG6_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_sig6_in_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_SIG6_IN_SEL_MASK);
    }

    /// Clear REG_GPIO_SIG6_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_sig6_in_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_SIG6_IN_SEL_MASK);
    }

    /// **REG_GPIO_FUNC6_IN_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func6_in_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC6_IN_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC6_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func6_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC6_IN_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC6_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func6_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC6_IN_INV_SEL_MASK);
    }
}

/// FUNC6_IN_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func6inselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func6inselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func6InSelCfgVal {
    bits: u32,
}

impl Func6InSelCfgVal {
    /// Create from raw bits
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

    /// Read **REG_GPIO_FUNC6_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func6_in_sel(&self) -> u32 {
        (self.bits & Func6InSelCfg::REG_GPIO_FUNC6_IN_SEL_MASK)
            >> Func6InSelCfg::REG_GPIO_FUNC6_IN_SEL_OFFSET
    }

    /// Read **REG_GPIO_SIG6_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_sig6_in_sel(&self) -> bool {
        (self.bits & Func6InSelCfg::REG_GPIO_SIG6_IN_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC6_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func6_in_inv_sel(&self) -> bool {
        (self.bits & Func6InSelCfg::REG_GPIO_FUNC6_IN_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC6_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func6_in_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func6InSelCfg::REG_GPIO_FUNC6_IN_SEL_MASK)
                | ((value << Func6InSelCfg::REG_GPIO_FUNC6_IN_SEL_OFFSET)
                    & Func6InSelCfg::REG_GPIO_FUNC6_IN_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_SIG6_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_sig6_in_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func6InSelCfg::REG_GPIO_SIG6_IN_SEL_MASK
            } else {
                self.bits & !Func6InSelCfg::REG_GPIO_SIG6_IN_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC6_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func6_in_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func6InSelCfg::REG_GPIO_FUNC6_IN_INV_SEL_MASK
            } else {
                self.bits & !Func6InSelCfg::REG_GPIO_FUNC6_IN_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func6InSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func6InSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func6InSelCfgVal {
        Func6InSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func6InSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func6InSelCfgVal) -> Func6InSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC7_IN_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0090
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **REG_GPIO_FUNC7_IN_SEL** `[7:2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_SIG7_IN_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC7_IN_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func7InSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func7InSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func7InSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC7_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const REG_GPIO_FUNC7_IN_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC7_IN_SEL_MASK: u32 = 0x000000fc;

    /// **REG_GPIO_SIG7_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_SIG7_IN_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_SIG7_IN_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_SIG7_IN_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC7_IN_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC7_IN_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC7_IN_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC7_IN_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC7_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func7_in_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC7_IN_SEL_MASK)
            >> Self::REG_GPIO_FUNC7_IN_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC7_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func7_in_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC7_IN_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC7_IN_SEL_OFFSET) & Self::REG_GPIO_FUNC7_IN_SEL_MASK)
        });
    }

    /// **REG_GPIO_SIG7_IN_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_sig7_in_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_SIG7_IN_SEL_MASK) != 0
    }

    /// Set REG_GPIO_SIG7_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_sig7_in_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_SIG7_IN_SEL_MASK);
    }

    /// Clear REG_GPIO_SIG7_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_sig7_in_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_SIG7_IN_SEL_MASK);
    }

    /// **REG_GPIO_FUNC7_IN_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func7_in_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC7_IN_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC7_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func7_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC7_IN_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC7_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func7_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC7_IN_INV_SEL_MASK);
    }
}

/// FUNC7_IN_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func7inselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func7inselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func7InSelCfgVal {
    bits: u32,
}

impl Func7InSelCfgVal {
    /// Create from raw bits
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

    /// Read **REG_GPIO_FUNC7_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func7_in_sel(&self) -> u32 {
        (self.bits & Func7InSelCfg::REG_GPIO_FUNC7_IN_SEL_MASK)
            >> Func7InSelCfg::REG_GPIO_FUNC7_IN_SEL_OFFSET
    }

    /// Read **REG_GPIO_SIG7_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_sig7_in_sel(&self) -> bool {
        (self.bits & Func7InSelCfg::REG_GPIO_SIG7_IN_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC7_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func7_in_inv_sel(&self) -> bool {
        (self.bits & Func7InSelCfg::REG_GPIO_FUNC7_IN_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC7_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func7_in_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func7InSelCfg::REG_GPIO_FUNC7_IN_SEL_MASK)
                | ((value << Func7InSelCfg::REG_GPIO_FUNC7_IN_SEL_OFFSET)
                    & Func7InSelCfg::REG_GPIO_FUNC7_IN_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_SIG7_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_sig7_in_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func7InSelCfg::REG_GPIO_SIG7_IN_SEL_MASK
            } else {
                self.bits & !Func7InSelCfg::REG_GPIO_SIG7_IN_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC7_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func7_in_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func7InSelCfg::REG_GPIO_FUNC7_IN_INV_SEL_MASK
            } else {
                self.bits & !Func7InSelCfg::REG_GPIO_FUNC7_IN_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func7InSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func7InSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func7InSelCfgVal {
        Func7InSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func7InSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func7InSelCfgVal) -> Func7InSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC8_IN_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0094
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **REG_GPIO_FUNC8_IN_SEL** `[7:2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_SIG8_IN_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC8_IN_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func8InSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func8InSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func8InSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC8_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const REG_GPIO_FUNC8_IN_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC8_IN_SEL_MASK: u32 = 0x000000fc;

    /// **REG_GPIO_SIG8_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_SIG8_IN_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_SIG8_IN_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_SIG8_IN_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC8_IN_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC8_IN_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC8_IN_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC8_IN_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC8_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func8_in_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC8_IN_SEL_MASK)
            >> Self::REG_GPIO_FUNC8_IN_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC8_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func8_in_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC8_IN_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC8_IN_SEL_OFFSET) & Self::REG_GPIO_FUNC8_IN_SEL_MASK)
        });
    }

    /// **REG_GPIO_SIG8_IN_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_sig8_in_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_SIG8_IN_SEL_MASK) != 0
    }

    /// Set REG_GPIO_SIG8_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_sig8_in_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_SIG8_IN_SEL_MASK);
    }

    /// Clear REG_GPIO_SIG8_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_sig8_in_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_SIG8_IN_SEL_MASK);
    }

    /// **REG_GPIO_FUNC8_IN_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func8_in_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC8_IN_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC8_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func8_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC8_IN_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC8_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func8_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC8_IN_INV_SEL_MASK);
    }
}

/// FUNC8_IN_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func8inselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func8inselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func8InSelCfgVal {
    bits: u32,
}

impl Func8InSelCfgVal {
    /// Create from raw bits
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

    /// Read **REG_GPIO_FUNC8_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func8_in_sel(&self) -> u32 {
        (self.bits & Func8InSelCfg::REG_GPIO_FUNC8_IN_SEL_MASK)
            >> Func8InSelCfg::REG_GPIO_FUNC8_IN_SEL_OFFSET
    }

    /// Read **REG_GPIO_SIG8_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_sig8_in_sel(&self) -> bool {
        (self.bits & Func8InSelCfg::REG_GPIO_SIG8_IN_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC8_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func8_in_inv_sel(&self) -> bool {
        (self.bits & Func8InSelCfg::REG_GPIO_FUNC8_IN_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC8_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func8_in_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func8InSelCfg::REG_GPIO_FUNC8_IN_SEL_MASK)
                | ((value << Func8InSelCfg::REG_GPIO_FUNC8_IN_SEL_OFFSET)
                    & Func8InSelCfg::REG_GPIO_FUNC8_IN_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_SIG8_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_sig8_in_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func8InSelCfg::REG_GPIO_SIG8_IN_SEL_MASK
            } else {
                self.bits & !Func8InSelCfg::REG_GPIO_SIG8_IN_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC8_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func8_in_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func8InSelCfg::REG_GPIO_FUNC8_IN_INV_SEL_MASK
            } else {
                self.bits & !Func8InSelCfg::REG_GPIO_FUNC8_IN_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func8InSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func8InSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func8InSelCfgVal {
        Func8InSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func8InSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func8InSelCfgVal) -> Func8InSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC9_IN_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0098
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **REG_GPIO_FUNC9_IN_SEL** `[7:2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_SIG9_IN_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC9_IN_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func9InSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func9InSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func9InSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC9_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const REG_GPIO_FUNC9_IN_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC9_IN_SEL_MASK: u32 = 0x000000fc;

    /// **REG_GPIO_SIG9_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_SIG9_IN_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_SIG9_IN_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_SIG9_IN_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC9_IN_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC9_IN_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC9_IN_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC9_IN_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC9_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func9_in_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC9_IN_SEL_MASK)
            >> Self::REG_GPIO_FUNC9_IN_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC9_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func9_in_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC9_IN_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC9_IN_SEL_OFFSET) & Self::REG_GPIO_FUNC9_IN_SEL_MASK)
        });
    }

    /// **REG_GPIO_SIG9_IN_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_sig9_in_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_SIG9_IN_SEL_MASK) != 0
    }

    /// Set REG_GPIO_SIG9_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_sig9_in_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_SIG9_IN_SEL_MASK);
    }

    /// Clear REG_GPIO_SIG9_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_sig9_in_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_SIG9_IN_SEL_MASK);
    }

    /// **REG_GPIO_FUNC9_IN_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func9_in_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC9_IN_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC9_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func9_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC9_IN_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC9_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func9_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC9_IN_INV_SEL_MASK);
    }
}

/// FUNC9_IN_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func9inselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func9inselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func9InSelCfgVal {
    bits: u32,
}

impl Func9InSelCfgVal {
    /// Create from raw bits
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

    /// Read **REG_GPIO_FUNC9_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func9_in_sel(&self) -> u32 {
        (self.bits & Func9InSelCfg::REG_GPIO_FUNC9_IN_SEL_MASK)
            >> Func9InSelCfg::REG_GPIO_FUNC9_IN_SEL_OFFSET
    }

    /// Read **REG_GPIO_SIG9_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_sig9_in_sel(&self) -> bool {
        (self.bits & Func9InSelCfg::REG_GPIO_SIG9_IN_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC9_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func9_in_inv_sel(&self) -> bool {
        (self.bits & Func9InSelCfg::REG_GPIO_FUNC9_IN_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC9_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func9_in_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func9InSelCfg::REG_GPIO_FUNC9_IN_SEL_MASK)
                | ((value << Func9InSelCfg::REG_GPIO_FUNC9_IN_SEL_OFFSET)
                    & Func9InSelCfg::REG_GPIO_FUNC9_IN_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_SIG9_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_sig9_in_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func9InSelCfg::REG_GPIO_SIG9_IN_SEL_MASK
            } else {
                self.bits & !Func9InSelCfg::REG_GPIO_SIG9_IN_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC9_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func9_in_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func9InSelCfg::REG_GPIO_FUNC9_IN_INV_SEL_MASK
            } else {
                self.bits & !Func9InSelCfg::REG_GPIO_FUNC9_IN_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func9InSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func9InSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func9InSelCfgVal {
        Func9InSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func9InSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func9InSelCfgVal) -> Func9InSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC10_IN_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x009c
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **REG_GPIO_FUNC10_IN_SEL** `[7:2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_SIG10_IN_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC10_IN_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func10InSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func10InSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func10InSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC10_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const REG_GPIO_FUNC10_IN_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC10_IN_SEL_MASK: u32 = 0x000000fc;

    /// **REG_GPIO_SIG10_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_SIG10_IN_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_SIG10_IN_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_SIG10_IN_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC10_IN_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC10_IN_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC10_IN_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC10_IN_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC10_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func10_in_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC10_IN_SEL_MASK)
            >> Self::REG_GPIO_FUNC10_IN_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC10_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func10_in_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC10_IN_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC10_IN_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC10_IN_SEL_MASK)
        });
    }

    /// **REG_GPIO_SIG10_IN_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_sig10_in_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_SIG10_IN_SEL_MASK) != 0
    }

    /// Set REG_GPIO_SIG10_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_sig10_in_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_SIG10_IN_SEL_MASK);
    }

    /// Clear REG_GPIO_SIG10_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_sig10_in_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_SIG10_IN_SEL_MASK);
    }

    /// **REG_GPIO_FUNC10_IN_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func10_in_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC10_IN_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC10_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func10_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC10_IN_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC10_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func10_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC10_IN_INV_SEL_MASK);
    }
}

/// FUNC10_IN_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func10inselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func10inselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func10InSelCfgVal {
    bits: u32,
}

impl Func10InSelCfgVal {
    /// Create from raw bits
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

    /// Read **REG_GPIO_FUNC10_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func10_in_sel(&self) -> u32 {
        (self.bits & Func10InSelCfg::REG_GPIO_FUNC10_IN_SEL_MASK)
            >> Func10InSelCfg::REG_GPIO_FUNC10_IN_SEL_OFFSET
    }

    /// Read **REG_GPIO_SIG10_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_sig10_in_sel(&self) -> bool {
        (self.bits & Func10InSelCfg::REG_GPIO_SIG10_IN_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC10_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func10_in_inv_sel(&self) -> bool {
        (self.bits & Func10InSelCfg::REG_GPIO_FUNC10_IN_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC10_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func10_in_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func10InSelCfg::REG_GPIO_FUNC10_IN_SEL_MASK)
                | ((value << Func10InSelCfg::REG_GPIO_FUNC10_IN_SEL_OFFSET)
                    & Func10InSelCfg::REG_GPIO_FUNC10_IN_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_SIG10_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_sig10_in_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func10InSelCfg::REG_GPIO_SIG10_IN_SEL_MASK
            } else {
                self.bits & !Func10InSelCfg::REG_GPIO_SIG10_IN_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC10_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func10_in_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func10InSelCfg::REG_GPIO_FUNC10_IN_INV_SEL_MASK
            } else {
                self.bits & !Func10InSelCfg::REG_GPIO_FUNC10_IN_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func10InSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func10InSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func10InSelCfgVal {
        Func10InSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func10InSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func10InSelCfgVal) -> Func10InSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC11_IN_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x00a0
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **REG_GPIO_FUNC11_IN_SEL** `[7:2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_SIG11_IN_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC11_IN_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func11InSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func11InSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func11InSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC11_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const REG_GPIO_FUNC11_IN_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC11_IN_SEL_MASK: u32 = 0x000000fc;

    /// **REG_GPIO_SIG11_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_SIG11_IN_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_SIG11_IN_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_SIG11_IN_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC11_IN_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC11_IN_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC11_IN_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC11_IN_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC11_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func11_in_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC11_IN_SEL_MASK)
            >> Self::REG_GPIO_FUNC11_IN_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC11_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func11_in_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC11_IN_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC11_IN_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC11_IN_SEL_MASK)
        });
    }

    /// **REG_GPIO_SIG11_IN_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_sig11_in_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_SIG11_IN_SEL_MASK) != 0
    }

    /// Set REG_GPIO_SIG11_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_sig11_in_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_SIG11_IN_SEL_MASK);
    }

    /// Clear REG_GPIO_SIG11_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_sig11_in_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_SIG11_IN_SEL_MASK);
    }

    /// **REG_GPIO_FUNC11_IN_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func11_in_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC11_IN_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC11_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func11_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC11_IN_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC11_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func11_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC11_IN_INV_SEL_MASK);
    }
}

/// FUNC11_IN_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func11inselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func11inselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func11InSelCfgVal {
    bits: u32,
}

impl Func11InSelCfgVal {
    /// Create from raw bits
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

    /// Read **REG_GPIO_FUNC11_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func11_in_sel(&self) -> u32 {
        (self.bits & Func11InSelCfg::REG_GPIO_FUNC11_IN_SEL_MASK)
            >> Func11InSelCfg::REG_GPIO_FUNC11_IN_SEL_OFFSET
    }

    /// Read **REG_GPIO_SIG11_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_sig11_in_sel(&self) -> bool {
        (self.bits & Func11InSelCfg::REG_GPIO_SIG11_IN_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC11_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func11_in_inv_sel(&self) -> bool {
        (self.bits & Func11InSelCfg::REG_GPIO_FUNC11_IN_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC11_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func11_in_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func11InSelCfg::REG_GPIO_FUNC11_IN_SEL_MASK)
                | ((value << Func11InSelCfg::REG_GPIO_FUNC11_IN_SEL_OFFSET)
                    & Func11InSelCfg::REG_GPIO_FUNC11_IN_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_SIG11_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_sig11_in_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func11InSelCfg::REG_GPIO_SIG11_IN_SEL_MASK
            } else {
                self.bits & !Func11InSelCfg::REG_GPIO_SIG11_IN_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC11_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func11_in_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func11InSelCfg::REG_GPIO_FUNC11_IN_INV_SEL_MASK
            } else {
                self.bits & !Func11InSelCfg::REG_GPIO_FUNC11_IN_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func11InSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func11InSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func11InSelCfgVal {
        Func11InSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func11InSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func11InSelCfgVal) -> Func11InSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC12_IN_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x00a4
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **REG_GPIO_FUNC12_IN_SEL** `[7:2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_SIG12_IN_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC12_IN_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func12InSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func12InSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func12InSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC12_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const REG_GPIO_FUNC12_IN_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC12_IN_SEL_MASK: u32 = 0x000000fc;

    /// **REG_GPIO_SIG12_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_SIG12_IN_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_SIG12_IN_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_SIG12_IN_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC12_IN_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC12_IN_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC12_IN_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC12_IN_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC12_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func12_in_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC12_IN_SEL_MASK)
            >> Self::REG_GPIO_FUNC12_IN_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC12_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func12_in_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC12_IN_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC12_IN_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC12_IN_SEL_MASK)
        });
    }

    /// **REG_GPIO_SIG12_IN_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_sig12_in_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_SIG12_IN_SEL_MASK) != 0
    }

    /// Set REG_GPIO_SIG12_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_sig12_in_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_SIG12_IN_SEL_MASK);
    }

    /// Clear REG_GPIO_SIG12_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_sig12_in_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_SIG12_IN_SEL_MASK);
    }

    /// **REG_GPIO_FUNC12_IN_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func12_in_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC12_IN_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC12_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func12_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC12_IN_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC12_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func12_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC12_IN_INV_SEL_MASK);
    }
}

/// FUNC12_IN_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func12inselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func12inselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func12InSelCfgVal {
    bits: u32,
}

impl Func12InSelCfgVal {
    /// Create from raw bits
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

    /// Read **REG_GPIO_FUNC12_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func12_in_sel(&self) -> u32 {
        (self.bits & Func12InSelCfg::REG_GPIO_FUNC12_IN_SEL_MASK)
            >> Func12InSelCfg::REG_GPIO_FUNC12_IN_SEL_OFFSET
    }

    /// Read **REG_GPIO_SIG12_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_sig12_in_sel(&self) -> bool {
        (self.bits & Func12InSelCfg::REG_GPIO_SIG12_IN_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC12_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func12_in_inv_sel(&self) -> bool {
        (self.bits & Func12InSelCfg::REG_GPIO_FUNC12_IN_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC12_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func12_in_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func12InSelCfg::REG_GPIO_FUNC12_IN_SEL_MASK)
                | ((value << Func12InSelCfg::REG_GPIO_FUNC12_IN_SEL_OFFSET)
                    & Func12InSelCfg::REG_GPIO_FUNC12_IN_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_SIG12_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_sig12_in_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func12InSelCfg::REG_GPIO_SIG12_IN_SEL_MASK
            } else {
                self.bits & !Func12InSelCfg::REG_GPIO_SIG12_IN_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC12_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func12_in_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func12InSelCfg::REG_GPIO_FUNC12_IN_INV_SEL_MASK
            } else {
                self.bits & !Func12InSelCfg::REG_GPIO_FUNC12_IN_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func12InSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func12InSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func12InSelCfgVal {
        Func12InSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func12InSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func12InSelCfgVal) -> Func12InSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC13_IN_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x00a8
/// Reset value: 0x00000080
///
/// ## Fields
///
/// - **REG_GPIO_FUNC13_IN_SEL** `[7:2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_SIG13_IN_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC13_IN_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func13InSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func13InSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func13InSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC13_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[7:2]`, mask `0x000000fc`
    pub const REG_GPIO_FUNC13_IN_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC13_IN_SEL_MASK: u32 = 0x000000fc;

    /// **REG_GPIO_SIG13_IN_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_SIG13_IN_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_SIG13_IN_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_SIG13_IN_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC13_IN_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC13_IN_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC13_IN_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC13_IN_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC13_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func13_in_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC13_IN_SEL_MASK)
            >> Self::REG_GPIO_FUNC13_IN_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC13_IN_SEL** `[7:2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func13_in_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC13_IN_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC13_IN_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC13_IN_SEL_MASK)
        });
    }

    /// **REG_GPIO_SIG13_IN_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_sig13_in_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_SIG13_IN_SEL_MASK) != 0
    }

    /// Set REG_GPIO_SIG13_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_sig13_in_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_SIG13_IN_SEL_MASK);
    }

    /// Clear REG_GPIO_SIG13_IN_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_sig13_in_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_SIG13_IN_SEL_MASK);
    }

    /// **REG_GPIO_FUNC13_IN_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func13_in_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC13_IN_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC13_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func13_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC13_IN_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC13_IN_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func13_in_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC13_IN_INV_SEL_MASK);
    }
}

/// FUNC13_IN_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func13inselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func13inselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func13InSelCfgVal {
    bits: u32,
}

impl Func13InSelCfgVal {
    /// Create from raw bits
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

    /// Read **REG_GPIO_FUNC13_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func13_in_sel(&self) -> u32 {
        (self.bits & Func13InSelCfg::REG_GPIO_FUNC13_IN_SEL_MASK)
            >> Func13InSelCfg::REG_GPIO_FUNC13_IN_SEL_OFFSET
    }

    /// Read **REG_GPIO_SIG13_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_sig13_in_sel(&self) -> bool {
        (self.bits & Func13InSelCfg::REG_GPIO_SIG13_IN_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC13_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func13_in_inv_sel(&self) -> bool {
        (self.bits & Func13InSelCfg::REG_GPIO_FUNC13_IN_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC13_IN_SEL** `[7:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func13_in_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func13InSelCfg::REG_GPIO_FUNC13_IN_SEL_MASK)
                | ((value << Func13InSelCfg::REG_GPIO_FUNC13_IN_SEL_OFFSET)
                    & Func13InSelCfg::REG_GPIO_FUNC13_IN_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_SIG13_IN_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_sig13_in_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func13InSelCfg::REG_GPIO_SIG13_IN_SEL_MASK
            } else {
                self.bits & !Func13InSelCfg::REG_GPIO_SIG13_IN_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC13_IN_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func13_in_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func13InSelCfg::REG_GPIO_FUNC13_IN_INV_SEL_MASK
            } else {
                self.bits & !Func13InSelCfg::REG_GPIO_FUNC13_IN_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func13InSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func13InSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func13InSelCfgVal {
        Func13InSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func13InSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func13InSelCfgVal) -> Func13InSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC0_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x00f4
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC0_OUT_SEL** `[8:3]` (RW)
///
///   reg_gpio_func0_out_sel[5:1]==16 -> output gpio register value to pad
///
/// - **REG_GPIO_FUNC0_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC0_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC0_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func0OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func0OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func0OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC0_OUT_SEL**
    ///
    /// reg_gpio_func0_out_sel[5:1]==16 -> output gpio register value to pad
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC0_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC0_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC0_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC0_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC0_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC0_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC0_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC0_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC0_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC0_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC0_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC0_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC0_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC0_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC0_OUT_SEL** `[8:3]` (RW)
    ///
    /// reg_gpio_func0_out_sel[5:1]==16 -> output gpio register value to pad
    #[inline(always)]
    pub fn reg_gpio_func0_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC0_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC0_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC0_OUT_SEL** `[8:3]` (RW)
    ///
    /// reg_gpio_func0_out_sel[5:1]==16 -> output gpio register value to pad
    #[inline(always)]
    pub fn write_reg_gpio_func0_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC0_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC0_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC0_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC0_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func0_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC0_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC0_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func0_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC0_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC0_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func0_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC0_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC0_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func0_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC0_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC0_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func0_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC0_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC0_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func0_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC0_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC0_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func0_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC0_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC0_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func0_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC0_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC0_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func0_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC0_OE_INV_SEL_MASK);
    }
}

/// FUNC0_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func0outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func0outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func0OutSelCfgVal {
    bits: u32,
}

impl Func0OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC0_OUT_SEL** `[8:3]`
    ///
    /// reg_gpio_func0_out_sel[5:1]==16 -> output gpio register value to pad
    #[inline(always)]
    pub const fn reg_gpio_func0_out_sel(&self) -> u32 {
        (self.bits & Func0OutSelCfg::REG_GPIO_FUNC0_OUT_SEL_MASK)
            >> Func0OutSelCfg::REG_GPIO_FUNC0_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC0_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func0_out_inv_sel(&self) -> bool {
        (self.bits & Func0OutSelCfg::REG_GPIO_FUNC0_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC0_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func0_oe_sel(&self) -> bool {
        (self.bits & Func0OutSelCfg::REG_GPIO_FUNC0_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC0_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func0_oe_inv_sel(&self) -> bool {
        (self.bits & Func0OutSelCfg::REG_GPIO_FUNC0_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC0_OUT_SEL** `[8:3]`
    ///
    /// reg_gpio_func0_out_sel[5:1]==16 -> output gpio register value to pad
    #[inline(always)]
    pub const fn with_reg_gpio_func0_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func0OutSelCfg::REG_GPIO_FUNC0_OUT_SEL_MASK)
                | ((value << Func0OutSelCfg::REG_GPIO_FUNC0_OUT_SEL_OFFSET)
                    & Func0OutSelCfg::REG_GPIO_FUNC0_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC0_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func0_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func0OutSelCfg::REG_GPIO_FUNC0_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func0OutSelCfg::REG_GPIO_FUNC0_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC0_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func0_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func0OutSelCfg::REG_GPIO_FUNC0_OE_SEL_MASK
            } else {
                self.bits & !Func0OutSelCfg::REG_GPIO_FUNC0_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC0_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func0_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func0OutSelCfg::REG_GPIO_FUNC0_OE_INV_SEL_MASK
            } else {
                self.bits & !Func0OutSelCfg::REG_GPIO_FUNC0_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func0OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func0OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func0OutSelCfgVal {
        Func0OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func0OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func0OutSelCfgVal) -> Func0OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC1_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x00f8
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC1_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC1_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC1_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC1_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func1OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func1OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func1OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC1_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC1_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC1_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC1_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC1_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC1_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC1_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC1_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC1_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC1_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC1_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC1_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC1_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC1_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC1_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC1_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func1_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC1_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC1_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC1_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func1_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC1_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC1_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC1_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC1_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func1_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC1_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC1_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func1_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC1_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC1_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func1_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC1_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC1_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func1_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC1_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC1_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func1_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC1_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC1_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func1_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC1_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC1_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func1_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC1_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC1_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func1_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC1_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC1_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func1_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC1_OE_INV_SEL_MASK);
    }
}

/// FUNC1_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func1outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func1outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func1OutSelCfgVal {
    bits: u32,
}

impl Func1OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC1_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func1_out_sel(&self) -> u32 {
        (self.bits & Func1OutSelCfg::REG_GPIO_FUNC1_OUT_SEL_MASK)
            >> Func1OutSelCfg::REG_GPIO_FUNC1_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC1_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func1_out_inv_sel(&self) -> bool {
        (self.bits & Func1OutSelCfg::REG_GPIO_FUNC1_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC1_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func1_oe_sel(&self) -> bool {
        (self.bits & Func1OutSelCfg::REG_GPIO_FUNC1_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC1_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func1_oe_inv_sel(&self) -> bool {
        (self.bits & Func1OutSelCfg::REG_GPIO_FUNC1_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC1_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func1_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func1OutSelCfg::REG_GPIO_FUNC1_OUT_SEL_MASK)
                | ((value << Func1OutSelCfg::REG_GPIO_FUNC1_OUT_SEL_OFFSET)
                    & Func1OutSelCfg::REG_GPIO_FUNC1_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC1_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func1_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func1OutSelCfg::REG_GPIO_FUNC1_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func1OutSelCfg::REG_GPIO_FUNC1_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC1_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func1_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func1OutSelCfg::REG_GPIO_FUNC1_OE_SEL_MASK
            } else {
                self.bits & !Func1OutSelCfg::REG_GPIO_FUNC1_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC1_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func1_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func1OutSelCfg::REG_GPIO_FUNC1_OE_INV_SEL_MASK
            } else {
                self.bits & !Func1OutSelCfg::REG_GPIO_FUNC1_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func1OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func1OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func1OutSelCfgVal {
        Func1OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func1OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func1OutSelCfgVal) -> Func1OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC2_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x00fc
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC2_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC2_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC2_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC2_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func2OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func2OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func2OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC2_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC2_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC2_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC2_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC2_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC2_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC2_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC2_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC2_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC2_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC2_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC2_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC2_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC2_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC2_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC2_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func2_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC2_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC2_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC2_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func2_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC2_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC2_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC2_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC2_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func2_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC2_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC2_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func2_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC2_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC2_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func2_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC2_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC2_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func2_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC2_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC2_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func2_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC2_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC2_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func2_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC2_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC2_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func2_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC2_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC2_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func2_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC2_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC2_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func2_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC2_OE_INV_SEL_MASK);
    }
}

/// FUNC2_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func2outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func2outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func2OutSelCfgVal {
    bits: u32,
}

impl Func2OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC2_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func2_out_sel(&self) -> u32 {
        (self.bits & Func2OutSelCfg::REG_GPIO_FUNC2_OUT_SEL_MASK)
            >> Func2OutSelCfg::REG_GPIO_FUNC2_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC2_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func2_out_inv_sel(&self) -> bool {
        (self.bits & Func2OutSelCfg::REG_GPIO_FUNC2_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC2_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func2_oe_sel(&self) -> bool {
        (self.bits & Func2OutSelCfg::REG_GPIO_FUNC2_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC2_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func2_oe_inv_sel(&self) -> bool {
        (self.bits & Func2OutSelCfg::REG_GPIO_FUNC2_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC2_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func2_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func2OutSelCfg::REG_GPIO_FUNC2_OUT_SEL_MASK)
                | ((value << Func2OutSelCfg::REG_GPIO_FUNC2_OUT_SEL_OFFSET)
                    & Func2OutSelCfg::REG_GPIO_FUNC2_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC2_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func2_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func2OutSelCfg::REG_GPIO_FUNC2_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func2OutSelCfg::REG_GPIO_FUNC2_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC2_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func2_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func2OutSelCfg::REG_GPIO_FUNC2_OE_SEL_MASK
            } else {
                self.bits & !Func2OutSelCfg::REG_GPIO_FUNC2_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC2_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func2_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func2OutSelCfg::REG_GPIO_FUNC2_OE_INV_SEL_MASK
            } else {
                self.bits & !Func2OutSelCfg::REG_GPIO_FUNC2_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func2OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func2OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func2OutSelCfgVal {
        Func2OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func2OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func2OutSelCfgVal) -> Func2OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC3_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0100
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC3_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC3_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC3_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC3_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func3OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func3OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func3OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC3_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC3_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC3_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC3_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC3_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC3_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC3_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC3_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC3_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC3_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC3_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC3_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC3_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC3_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC3_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC3_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func3_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC3_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC3_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC3_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func3_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC3_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC3_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC3_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC3_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func3_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC3_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC3_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func3_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC3_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC3_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func3_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC3_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC3_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func3_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC3_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC3_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func3_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC3_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC3_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func3_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC3_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC3_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func3_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC3_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC3_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func3_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC3_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC3_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func3_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC3_OE_INV_SEL_MASK);
    }
}

/// FUNC3_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func3outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func3outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func3OutSelCfgVal {
    bits: u32,
}

impl Func3OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC3_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func3_out_sel(&self) -> u32 {
        (self.bits & Func3OutSelCfg::REG_GPIO_FUNC3_OUT_SEL_MASK)
            >> Func3OutSelCfg::REG_GPIO_FUNC3_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC3_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func3_out_inv_sel(&self) -> bool {
        (self.bits & Func3OutSelCfg::REG_GPIO_FUNC3_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC3_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func3_oe_sel(&self) -> bool {
        (self.bits & Func3OutSelCfg::REG_GPIO_FUNC3_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC3_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func3_oe_inv_sel(&self) -> bool {
        (self.bits & Func3OutSelCfg::REG_GPIO_FUNC3_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC3_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func3_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func3OutSelCfg::REG_GPIO_FUNC3_OUT_SEL_MASK)
                | ((value << Func3OutSelCfg::REG_GPIO_FUNC3_OUT_SEL_OFFSET)
                    & Func3OutSelCfg::REG_GPIO_FUNC3_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC3_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func3_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func3OutSelCfg::REG_GPIO_FUNC3_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func3OutSelCfg::REG_GPIO_FUNC3_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC3_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func3_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func3OutSelCfg::REG_GPIO_FUNC3_OE_SEL_MASK
            } else {
                self.bits & !Func3OutSelCfg::REG_GPIO_FUNC3_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC3_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func3_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func3OutSelCfg::REG_GPIO_FUNC3_OE_INV_SEL_MASK
            } else {
                self.bits & !Func3OutSelCfg::REG_GPIO_FUNC3_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func3OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func3OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func3OutSelCfgVal {
        Func3OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func3OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func3OutSelCfgVal) -> Func3OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC4_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0104
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC4_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC4_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC4_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC4_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func4OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func4OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func4OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC4_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC4_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC4_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC4_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC4_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC4_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC4_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC4_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC4_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC4_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC4_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC4_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC4_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC4_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC4_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC4_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func4_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC4_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC4_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC4_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func4_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC4_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC4_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC4_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC4_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func4_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC4_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC4_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func4_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC4_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC4_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func4_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC4_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC4_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func4_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC4_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC4_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func4_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC4_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC4_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func4_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC4_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC4_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func4_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC4_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC4_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func4_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC4_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC4_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func4_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC4_OE_INV_SEL_MASK);
    }
}

/// FUNC4_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func4outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func4outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func4OutSelCfgVal {
    bits: u32,
}

impl Func4OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC4_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func4_out_sel(&self) -> u32 {
        (self.bits & Func4OutSelCfg::REG_GPIO_FUNC4_OUT_SEL_MASK)
            >> Func4OutSelCfg::REG_GPIO_FUNC4_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC4_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func4_out_inv_sel(&self) -> bool {
        (self.bits & Func4OutSelCfg::REG_GPIO_FUNC4_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC4_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func4_oe_sel(&self) -> bool {
        (self.bits & Func4OutSelCfg::REG_GPIO_FUNC4_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC4_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func4_oe_inv_sel(&self) -> bool {
        (self.bits & Func4OutSelCfg::REG_GPIO_FUNC4_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC4_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func4_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func4OutSelCfg::REG_GPIO_FUNC4_OUT_SEL_MASK)
                | ((value << Func4OutSelCfg::REG_GPIO_FUNC4_OUT_SEL_OFFSET)
                    & Func4OutSelCfg::REG_GPIO_FUNC4_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC4_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func4_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func4OutSelCfg::REG_GPIO_FUNC4_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func4OutSelCfg::REG_GPIO_FUNC4_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC4_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func4_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func4OutSelCfg::REG_GPIO_FUNC4_OE_SEL_MASK
            } else {
                self.bits & !Func4OutSelCfg::REG_GPIO_FUNC4_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC4_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func4_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func4OutSelCfg::REG_GPIO_FUNC4_OE_INV_SEL_MASK
            } else {
                self.bits & !Func4OutSelCfg::REG_GPIO_FUNC4_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func4OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func4OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func4OutSelCfgVal {
        Func4OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func4OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func4OutSelCfgVal) -> Func4OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC5_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0108
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC5_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC5_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC5_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC5_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func5OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func5OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func5OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC5_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC5_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC5_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC5_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC5_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC5_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC5_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC5_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC5_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC5_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC5_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC5_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC5_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC5_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC5_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC5_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func5_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC5_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC5_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC5_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func5_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC5_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC5_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC5_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC5_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func5_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC5_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC5_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func5_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC5_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC5_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func5_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC5_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC5_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func5_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC5_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC5_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func5_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC5_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC5_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func5_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC5_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC5_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func5_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC5_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC5_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func5_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC5_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC5_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func5_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC5_OE_INV_SEL_MASK);
    }
}

/// FUNC5_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func5outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func5outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func5OutSelCfgVal {
    bits: u32,
}

impl Func5OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC5_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func5_out_sel(&self) -> u32 {
        (self.bits & Func5OutSelCfg::REG_GPIO_FUNC5_OUT_SEL_MASK)
            >> Func5OutSelCfg::REG_GPIO_FUNC5_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC5_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func5_out_inv_sel(&self) -> bool {
        (self.bits & Func5OutSelCfg::REG_GPIO_FUNC5_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC5_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func5_oe_sel(&self) -> bool {
        (self.bits & Func5OutSelCfg::REG_GPIO_FUNC5_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC5_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func5_oe_inv_sel(&self) -> bool {
        (self.bits & Func5OutSelCfg::REG_GPIO_FUNC5_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC5_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func5_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func5OutSelCfg::REG_GPIO_FUNC5_OUT_SEL_MASK)
                | ((value << Func5OutSelCfg::REG_GPIO_FUNC5_OUT_SEL_OFFSET)
                    & Func5OutSelCfg::REG_GPIO_FUNC5_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC5_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func5_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func5OutSelCfg::REG_GPIO_FUNC5_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func5OutSelCfg::REG_GPIO_FUNC5_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC5_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func5_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func5OutSelCfg::REG_GPIO_FUNC5_OE_SEL_MASK
            } else {
                self.bits & !Func5OutSelCfg::REG_GPIO_FUNC5_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC5_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func5_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func5OutSelCfg::REG_GPIO_FUNC5_OE_INV_SEL_MASK
            } else {
                self.bits & !Func5OutSelCfg::REG_GPIO_FUNC5_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func5OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func5OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func5OutSelCfgVal {
        Func5OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func5OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func5OutSelCfgVal) -> Func5OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC6_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x010c
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC6_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC6_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC6_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC6_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func6OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func6OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func6OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC6_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC6_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC6_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC6_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC6_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC6_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC6_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC6_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC6_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC6_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC6_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC6_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC6_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC6_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC6_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC6_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func6_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC6_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC6_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC6_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func6_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC6_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC6_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC6_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC6_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func6_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC6_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC6_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func6_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC6_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC6_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func6_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC6_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC6_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func6_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC6_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC6_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func6_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC6_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC6_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func6_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC6_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC6_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func6_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC6_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC6_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func6_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC6_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC6_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func6_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC6_OE_INV_SEL_MASK);
    }
}

/// FUNC6_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func6outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func6outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func6OutSelCfgVal {
    bits: u32,
}

impl Func6OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC6_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func6_out_sel(&self) -> u32 {
        (self.bits & Func6OutSelCfg::REG_GPIO_FUNC6_OUT_SEL_MASK)
            >> Func6OutSelCfg::REG_GPIO_FUNC6_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC6_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func6_out_inv_sel(&self) -> bool {
        (self.bits & Func6OutSelCfg::REG_GPIO_FUNC6_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC6_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func6_oe_sel(&self) -> bool {
        (self.bits & Func6OutSelCfg::REG_GPIO_FUNC6_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC6_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func6_oe_inv_sel(&self) -> bool {
        (self.bits & Func6OutSelCfg::REG_GPIO_FUNC6_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC6_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func6_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func6OutSelCfg::REG_GPIO_FUNC6_OUT_SEL_MASK)
                | ((value << Func6OutSelCfg::REG_GPIO_FUNC6_OUT_SEL_OFFSET)
                    & Func6OutSelCfg::REG_GPIO_FUNC6_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC6_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func6_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func6OutSelCfg::REG_GPIO_FUNC6_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func6OutSelCfg::REG_GPIO_FUNC6_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC6_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func6_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func6OutSelCfg::REG_GPIO_FUNC6_OE_SEL_MASK
            } else {
                self.bits & !Func6OutSelCfg::REG_GPIO_FUNC6_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC6_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func6_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func6OutSelCfg::REG_GPIO_FUNC6_OE_INV_SEL_MASK
            } else {
                self.bits & !Func6OutSelCfg::REG_GPIO_FUNC6_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func6OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func6OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func6OutSelCfgVal {
        Func6OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func6OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func6OutSelCfgVal) -> Func6OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC7_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0110
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC7_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC7_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC7_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC7_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func7OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func7OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func7OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC7_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC7_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC7_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC7_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC7_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC7_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC7_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC7_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC7_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC7_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC7_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC7_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC7_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC7_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC7_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC7_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func7_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC7_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC7_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC7_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func7_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC7_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC7_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC7_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC7_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func7_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC7_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC7_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func7_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC7_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC7_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func7_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC7_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC7_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func7_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC7_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC7_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func7_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC7_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC7_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func7_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC7_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC7_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func7_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC7_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC7_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func7_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC7_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC7_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func7_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC7_OE_INV_SEL_MASK);
    }
}

/// FUNC7_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func7outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func7outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func7OutSelCfgVal {
    bits: u32,
}

impl Func7OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC7_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func7_out_sel(&self) -> u32 {
        (self.bits & Func7OutSelCfg::REG_GPIO_FUNC7_OUT_SEL_MASK)
            >> Func7OutSelCfg::REG_GPIO_FUNC7_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC7_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func7_out_inv_sel(&self) -> bool {
        (self.bits & Func7OutSelCfg::REG_GPIO_FUNC7_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC7_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func7_oe_sel(&self) -> bool {
        (self.bits & Func7OutSelCfg::REG_GPIO_FUNC7_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC7_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func7_oe_inv_sel(&self) -> bool {
        (self.bits & Func7OutSelCfg::REG_GPIO_FUNC7_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC7_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func7_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func7OutSelCfg::REG_GPIO_FUNC7_OUT_SEL_MASK)
                | ((value << Func7OutSelCfg::REG_GPIO_FUNC7_OUT_SEL_OFFSET)
                    & Func7OutSelCfg::REG_GPIO_FUNC7_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC7_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func7_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func7OutSelCfg::REG_GPIO_FUNC7_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func7OutSelCfg::REG_GPIO_FUNC7_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC7_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func7_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func7OutSelCfg::REG_GPIO_FUNC7_OE_SEL_MASK
            } else {
                self.bits & !Func7OutSelCfg::REG_GPIO_FUNC7_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC7_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func7_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func7OutSelCfg::REG_GPIO_FUNC7_OE_INV_SEL_MASK
            } else {
                self.bits & !Func7OutSelCfg::REG_GPIO_FUNC7_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func7OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func7OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func7OutSelCfgVal {
        Func7OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func7OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func7OutSelCfgVal) -> Func7OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC8_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0114
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC8_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC8_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC8_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC8_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func8OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func8OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func8OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC8_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC8_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC8_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC8_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC8_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC8_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC8_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC8_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC8_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC8_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC8_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC8_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC8_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC8_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC8_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC8_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func8_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC8_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC8_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC8_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func8_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC8_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC8_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC8_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC8_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func8_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC8_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC8_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func8_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC8_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC8_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func8_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC8_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC8_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func8_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC8_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC8_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func8_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC8_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC8_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func8_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC8_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC8_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func8_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC8_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC8_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func8_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC8_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC8_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func8_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC8_OE_INV_SEL_MASK);
    }
}

/// FUNC8_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func8outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func8outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func8OutSelCfgVal {
    bits: u32,
}

impl Func8OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC8_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func8_out_sel(&self) -> u32 {
        (self.bits & Func8OutSelCfg::REG_GPIO_FUNC8_OUT_SEL_MASK)
            >> Func8OutSelCfg::REG_GPIO_FUNC8_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC8_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func8_out_inv_sel(&self) -> bool {
        (self.bits & Func8OutSelCfg::REG_GPIO_FUNC8_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC8_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func8_oe_sel(&self) -> bool {
        (self.bits & Func8OutSelCfg::REG_GPIO_FUNC8_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC8_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func8_oe_inv_sel(&self) -> bool {
        (self.bits & Func8OutSelCfg::REG_GPIO_FUNC8_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC8_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func8_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func8OutSelCfg::REG_GPIO_FUNC8_OUT_SEL_MASK)
                | ((value << Func8OutSelCfg::REG_GPIO_FUNC8_OUT_SEL_OFFSET)
                    & Func8OutSelCfg::REG_GPIO_FUNC8_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC8_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func8_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func8OutSelCfg::REG_GPIO_FUNC8_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func8OutSelCfg::REG_GPIO_FUNC8_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC8_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func8_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func8OutSelCfg::REG_GPIO_FUNC8_OE_SEL_MASK
            } else {
                self.bits & !Func8OutSelCfg::REG_GPIO_FUNC8_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC8_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func8_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func8OutSelCfg::REG_GPIO_FUNC8_OE_INV_SEL_MASK
            } else {
                self.bits & !Func8OutSelCfg::REG_GPIO_FUNC8_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func8OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func8OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func8OutSelCfgVal {
        Func8OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func8OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func8OutSelCfgVal) -> Func8OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC9_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0118
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC9_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC9_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC9_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC9_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func9OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func9OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func9OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC9_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC9_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC9_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC9_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC9_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC9_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC9_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC9_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC9_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC9_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC9_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC9_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC9_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC9_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC9_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC9_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func9_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC9_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC9_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC9_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func9_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC9_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC9_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC9_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC9_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func9_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC9_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC9_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func9_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC9_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC9_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func9_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC9_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC9_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func9_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC9_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC9_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func9_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC9_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC9_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func9_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC9_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC9_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func9_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC9_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC9_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func9_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC9_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC9_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func9_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC9_OE_INV_SEL_MASK);
    }
}

/// FUNC9_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func9outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func9outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func9OutSelCfgVal {
    bits: u32,
}

impl Func9OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC9_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func9_out_sel(&self) -> u32 {
        (self.bits & Func9OutSelCfg::REG_GPIO_FUNC9_OUT_SEL_MASK)
            >> Func9OutSelCfg::REG_GPIO_FUNC9_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC9_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func9_out_inv_sel(&self) -> bool {
        (self.bits & Func9OutSelCfg::REG_GPIO_FUNC9_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC9_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func9_oe_sel(&self) -> bool {
        (self.bits & Func9OutSelCfg::REG_GPIO_FUNC9_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC9_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func9_oe_inv_sel(&self) -> bool {
        (self.bits & Func9OutSelCfg::REG_GPIO_FUNC9_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC9_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func9_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func9OutSelCfg::REG_GPIO_FUNC9_OUT_SEL_MASK)
                | ((value << Func9OutSelCfg::REG_GPIO_FUNC9_OUT_SEL_OFFSET)
                    & Func9OutSelCfg::REG_GPIO_FUNC9_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC9_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func9_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func9OutSelCfg::REG_GPIO_FUNC9_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func9OutSelCfg::REG_GPIO_FUNC9_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC9_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func9_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func9OutSelCfg::REG_GPIO_FUNC9_OE_SEL_MASK
            } else {
                self.bits & !Func9OutSelCfg::REG_GPIO_FUNC9_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC9_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func9_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func9OutSelCfg::REG_GPIO_FUNC9_OE_INV_SEL_MASK
            } else {
                self.bits & !Func9OutSelCfg::REG_GPIO_FUNC9_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func9OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func9OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func9OutSelCfgVal {
        Func9OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func9OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func9OutSelCfgVal) -> Func9OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC10_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x011c
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC10_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC10_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC10_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC10_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func10OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func10OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func10OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC10_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC10_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC10_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC10_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC10_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC10_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC10_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC10_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC10_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC10_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC10_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC10_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC10_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC10_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC10_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC10_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func10_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC10_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC10_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC10_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func10_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC10_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC10_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC10_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC10_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func10_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC10_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC10_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func10_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC10_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC10_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func10_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC10_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC10_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func10_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC10_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC10_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func10_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC10_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC10_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func10_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC10_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC10_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func10_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC10_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC10_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func10_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC10_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC10_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func10_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC10_OE_INV_SEL_MASK);
    }
}

/// FUNC10_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func10outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func10outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func10OutSelCfgVal {
    bits: u32,
}

impl Func10OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC10_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func10_out_sel(&self) -> u32 {
        (self.bits & Func10OutSelCfg::REG_GPIO_FUNC10_OUT_SEL_MASK)
            >> Func10OutSelCfg::REG_GPIO_FUNC10_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC10_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func10_out_inv_sel(&self) -> bool {
        (self.bits & Func10OutSelCfg::REG_GPIO_FUNC10_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC10_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func10_oe_sel(&self) -> bool {
        (self.bits & Func10OutSelCfg::REG_GPIO_FUNC10_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC10_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func10_oe_inv_sel(&self) -> bool {
        (self.bits & Func10OutSelCfg::REG_GPIO_FUNC10_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC10_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func10_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func10OutSelCfg::REG_GPIO_FUNC10_OUT_SEL_MASK)
                | ((value << Func10OutSelCfg::REG_GPIO_FUNC10_OUT_SEL_OFFSET)
                    & Func10OutSelCfg::REG_GPIO_FUNC10_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC10_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func10_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func10OutSelCfg::REG_GPIO_FUNC10_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func10OutSelCfg::REG_GPIO_FUNC10_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC10_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func10_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func10OutSelCfg::REG_GPIO_FUNC10_OE_SEL_MASK
            } else {
                self.bits & !Func10OutSelCfg::REG_GPIO_FUNC10_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC10_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func10_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func10OutSelCfg::REG_GPIO_FUNC10_OE_INV_SEL_MASK
            } else {
                self.bits & !Func10OutSelCfg::REG_GPIO_FUNC10_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func10OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func10OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func10OutSelCfgVal {
        Func10OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func10OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func10OutSelCfgVal) -> Func10OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC11_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0120
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC11_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC11_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC11_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC11_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func11OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func11OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func11OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC11_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC11_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC11_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC11_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC11_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC11_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC11_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC11_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC11_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC11_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC11_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC11_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC11_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC11_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC11_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC11_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func11_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC11_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC11_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC11_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func11_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC11_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC11_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC11_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC11_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func11_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC11_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC11_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func11_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC11_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC11_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func11_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC11_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC11_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func11_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC11_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC11_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func11_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC11_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC11_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func11_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC11_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC11_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func11_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC11_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC11_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func11_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC11_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC11_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func11_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC11_OE_INV_SEL_MASK);
    }
}

/// FUNC11_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func11outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func11outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func11OutSelCfgVal {
    bits: u32,
}

impl Func11OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC11_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func11_out_sel(&self) -> u32 {
        (self.bits & Func11OutSelCfg::REG_GPIO_FUNC11_OUT_SEL_MASK)
            >> Func11OutSelCfg::REG_GPIO_FUNC11_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC11_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func11_out_inv_sel(&self) -> bool {
        (self.bits & Func11OutSelCfg::REG_GPIO_FUNC11_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC11_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func11_oe_sel(&self) -> bool {
        (self.bits & Func11OutSelCfg::REG_GPIO_FUNC11_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC11_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func11_oe_inv_sel(&self) -> bool {
        (self.bits & Func11OutSelCfg::REG_GPIO_FUNC11_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC11_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func11_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func11OutSelCfg::REG_GPIO_FUNC11_OUT_SEL_MASK)
                | ((value << Func11OutSelCfg::REG_GPIO_FUNC11_OUT_SEL_OFFSET)
                    & Func11OutSelCfg::REG_GPIO_FUNC11_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC11_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func11_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func11OutSelCfg::REG_GPIO_FUNC11_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func11OutSelCfg::REG_GPIO_FUNC11_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC11_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func11_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func11OutSelCfg::REG_GPIO_FUNC11_OE_SEL_MASK
            } else {
                self.bits & !Func11OutSelCfg::REG_GPIO_FUNC11_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC11_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func11_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func11OutSelCfg::REG_GPIO_FUNC11_OE_INV_SEL_MASK
            } else {
                self.bits & !Func11OutSelCfg::REG_GPIO_FUNC11_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func11OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func11OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func11OutSelCfgVal {
        Func11OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func11OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func11OutSelCfgVal) -> Func11OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC12_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0124
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC12_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC12_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC12_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC12_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func12OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func12OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func12OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC12_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC12_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC12_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC12_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC12_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC12_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC12_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC12_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC12_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC12_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC12_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC12_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC12_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC12_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC12_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC12_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func12_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC12_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC12_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC12_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func12_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC12_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC12_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC12_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC12_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func12_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC12_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC12_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func12_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC12_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC12_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func12_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC12_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC12_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func12_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC12_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC12_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func12_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC12_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC12_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func12_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC12_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC12_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func12_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC12_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC12_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func12_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC12_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC12_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func12_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC12_OE_INV_SEL_MASK);
    }
}

/// FUNC12_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func12outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func12outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func12OutSelCfgVal {
    bits: u32,
}

impl Func12OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC12_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func12_out_sel(&self) -> u32 {
        (self.bits & Func12OutSelCfg::REG_GPIO_FUNC12_OUT_SEL_MASK)
            >> Func12OutSelCfg::REG_GPIO_FUNC12_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC12_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func12_out_inv_sel(&self) -> bool {
        (self.bits & Func12OutSelCfg::REG_GPIO_FUNC12_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC12_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func12_oe_sel(&self) -> bool {
        (self.bits & Func12OutSelCfg::REG_GPIO_FUNC12_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC12_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func12_oe_inv_sel(&self) -> bool {
        (self.bits & Func12OutSelCfg::REG_GPIO_FUNC12_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC12_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func12_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func12OutSelCfg::REG_GPIO_FUNC12_OUT_SEL_MASK)
                | ((value << Func12OutSelCfg::REG_GPIO_FUNC12_OUT_SEL_OFFSET)
                    & Func12OutSelCfg::REG_GPIO_FUNC12_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC12_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func12_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func12OutSelCfg::REG_GPIO_FUNC12_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func12OutSelCfg::REG_GPIO_FUNC12_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC12_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func12_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func12OutSelCfg::REG_GPIO_FUNC12_OE_SEL_MASK
            } else {
                self.bits & !Func12OutSelCfg::REG_GPIO_FUNC12_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC12_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func12_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func12OutSelCfg::REG_GPIO_FUNC12_OE_INV_SEL_MASK
            } else {
                self.bits & !Func12OutSelCfg::REG_GPIO_FUNC12_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func12OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func12OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func12OutSelCfgVal {
        Func12OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func12OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func12OutSelCfgVal) -> Func12OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC13_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0128
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC13_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC13_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC13_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC13_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func13OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func13OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func13OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC13_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC13_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC13_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC13_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC13_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC13_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC13_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC13_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC13_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC13_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC13_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC13_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC13_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC13_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC13_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC13_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func13_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC13_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC13_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC13_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func13_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC13_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC13_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC13_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC13_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func13_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC13_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC13_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func13_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC13_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC13_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func13_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC13_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC13_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func13_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC13_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC13_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func13_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC13_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC13_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func13_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC13_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC13_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func13_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC13_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC13_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func13_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC13_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC13_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func13_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC13_OE_INV_SEL_MASK);
    }
}

/// FUNC13_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func13outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func13outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func13OutSelCfgVal {
    bits: u32,
}

impl Func13OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC13_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func13_out_sel(&self) -> u32 {
        (self.bits & Func13OutSelCfg::REG_GPIO_FUNC13_OUT_SEL_MASK)
            >> Func13OutSelCfg::REG_GPIO_FUNC13_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC13_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func13_out_inv_sel(&self) -> bool {
        (self.bits & Func13OutSelCfg::REG_GPIO_FUNC13_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC13_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func13_oe_sel(&self) -> bool {
        (self.bits & Func13OutSelCfg::REG_GPIO_FUNC13_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC13_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func13_oe_inv_sel(&self) -> bool {
        (self.bits & Func13OutSelCfg::REG_GPIO_FUNC13_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC13_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func13_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func13OutSelCfg::REG_GPIO_FUNC13_OUT_SEL_MASK)
                | ((value << Func13OutSelCfg::REG_GPIO_FUNC13_OUT_SEL_OFFSET)
                    & Func13OutSelCfg::REG_GPIO_FUNC13_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC13_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func13_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func13OutSelCfg::REG_GPIO_FUNC13_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func13OutSelCfg::REG_GPIO_FUNC13_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC13_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func13_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func13OutSelCfg::REG_GPIO_FUNC13_OE_SEL_MASK
            } else {
                self.bits & !Func13OutSelCfg::REG_GPIO_FUNC13_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC13_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func13_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func13OutSelCfg::REG_GPIO_FUNC13_OE_INV_SEL_MASK
            } else {
                self.bits & !Func13OutSelCfg::REG_GPIO_FUNC13_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func13OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func13OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func13OutSelCfgVal {
        Func13OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func13OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func13OutSelCfgVal) -> Func13OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC14_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x012c
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC14_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC14_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC14_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC14_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func14OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func14OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func14OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC14_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC14_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC14_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC14_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC14_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC14_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC14_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC14_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC14_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC14_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC14_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC14_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC14_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC14_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC14_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC14_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func14_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC14_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC14_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC14_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func14_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC14_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC14_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC14_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC14_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func14_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC14_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC14_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func14_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC14_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC14_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func14_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC14_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC14_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func14_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC14_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC14_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func14_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC14_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC14_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func14_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC14_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC14_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func14_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC14_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC14_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func14_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC14_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC14_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func14_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC14_OE_INV_SEL_MASK);
    }
}

/// FUNC14_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func14outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func14outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func14OutSelCfgVal {
    bits: u32,
}

impl Func14OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC14_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func14_out_sel(&self) -> u32 {
        (self.bits & Func14OutSelCfg::REG_GPIO_FUNC14_OUT_SEL_MASK)
            >> Func14OutSelCfg::REG_GPIO_FUNC14_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC14_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func14_out_inv_sel(&self) -> bool {
        (self.bits & Func14OutSelCfg::REG_GPIO_FUNC14_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC14_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func14_oe_sel(&self) -> bool {
        (self.bits & Func14OutSelCfg::REG_GPIO_FUNC14_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC14_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func14_oe_inv_sel(&self) -> bool {
        (self.bits & Func14OutSelCfg::REG_GPIO_FUNC14_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC14_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func14_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func14OutSelCfg::REG_GPIO_FUNC14_OUT_SEL_MASK)
                | ((value << Func14OutSelCfg::REG_GPIO_FUNC14_OUT_SEL_OFFSET)
                    & Func14OutSelCfg::REG_GPIO_FUNC14_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC14_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func14_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func14OutSelCfg::REG_GPIO_FUNC14_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func14OutSelCfg::REG_GPIO_FUNC14_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC14_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func14_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func14OutSelCfg::REG_GPIO_FUNC14_OE_SEL_MASK
            } else {
                self.bits & !Func14OutSelCfg::REG_GPIO_FUNC14_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC14_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func14_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func14OutSelCfg::REG_GPIO_FUNC14_OE_INV_SEL_MASK
            } else {
                self.bits & !Func14OutSelCfg::REG_GPIO_FUNC14_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func14OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func14OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func14OutSelCfgVal {
        Func14OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func14OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func14OutSelCfgVal) -> Func14OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FUNC15_OUT_SEL_CFG Register
///
/// Reserved
///
/// Address offset: 0x0130
/// Reset value: 0x00000100
///
/// ## Fields
///
/// - **REG_GPIO_FUNC15_OUT_SEL** `[8:3]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC15_OUT_INV_SEL** `[2]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC15_OE_SEL** `[1]` (RW)
///
///   Reserved
///
/// - **REG_GPIO_FUNC15_OE_INV_SEL** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Func15OutSelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Func15OutSelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Func15OutSelCfg {
    // ======== Field Constants ========

    /// **REG_GPIO_FUNC15_OUT_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[8:3]`, mask `0x000001f8`
    pub const REG_GPIO_FUNC15_OUT_SEL_OFFSET: u32 = 3;
    pub const REG_GPIO_FUNC15_OUT_SEL_MASK: u32 = 0x000001f8;

    /// **REG_GPIO_FUNC15_OUT_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_GPIO_FUNC15_OUT_INV_SEL_OFFSET: u32 = 2;
    pub const REG_GPIO_FUNC15_OUT_INV_SEL_MASK: u32 = 0x00000004;
    pub const REG_GPIO_FUNC15_OUT_INV_SEL: u32 = 1 << 2;

    /// **REG_GPIO_FUNC15_OE_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_GPIO_FUNC15_OE_SEL_OFFSET: u32 = 1;
    pub const REG_GPIO_FUNC15_OE_SEL_MASK: u32 = 0x00000002;
    pub const REG_GPIO_FUNC15_OE_SEL: u32 = 1 << 1;

    /// **REG_GPIO_FUNC15_OE_INV_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_GPIO_FUNC15_OE_INV_SEL_OFFSET: u32 = 0;
    pub const REG_GPIO_FUNC15_OE_INV_SEL_MASK: u32 = 0x00000001;
    pub const REG_GPIO_FUNC15_OE_INV_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_GPIO_FUNC15_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func15_out_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_FUNC15_OUT_SEL_MASK)
            >> Self::REG_GPIO_FUNC15_OUT_SEL_OFFSET
    }

    /// Write **REG_GPIO_FUNC15_OUT_SEL** `[8:3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_gpio_func15_out_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_FUNC15_OUT_SEL_MASK)
                | ((value << Self::REG_GPIO_FUNC15_OUT_SEL_OFFSET)
                    & Self::REG_GPIO_FUNC15_OUT_SEL_MASK)
        });
    }

    /// **REG_GPIO_FUNC15_OUT_INV_SEL** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func15_out_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC15_OUT_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC15_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func15_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC15_OUT_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC15_OUT_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func15_out_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC15_OUT_INV_SEL_MASK);
    }

    /// **REG_GPIO_FUNC15_OE_SEL** `[1]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func15_oe_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC15_OE_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC15_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func15_oe_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC15_OE_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC15_OE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func15_oe_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC15_OE_SEL_MASK);
    }

    /// **REG_GPIO_FUNC15_OE_INV_SEL** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_gpio_func15_oe_inv_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_GPIO_FUNC15_OE_INV_SEL_MASK) != 0
    }

    /// Set REG_GPIO_FUNC15_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_gpio_func15_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_GPIO_FUNC15_OE_INV_SEL_MASK);
    }

    /// Clear REG_GPIO_FUNC15_OE_INV_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_gpio_func15_oe_inv_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_GPIO_FUNC15_OE_INV_SEL_MASK);
    }
}

/// FUNC15_OUT_SEL_CFG value type for batch read/modify/write operations
///
/// Use `func15outselcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `func15outselcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Func15OutSelCfgVal {
    bits: u32,
}

impl Func15OutSelCfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00000100 }
    }

    /// Read **REG_GPIO_FUNC15_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func15_out_sel(&self) -> u32 {
        (self.bits & Func15OutSelCfg::REG_GPIO_FUNC15_OUT_SEL_MASK)
            >> Func15OutSelCfg::REG_GPIO_FUNC15_OUT_SEL_OFFSET
    }

    /// Read **REG_GPIO_FUNC15_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func15_out_inv_sel(&self) -> bool {
        (self.bits & Func15OutSelCfg::REG_GPIO_FUNC15_OUT_INV_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC15_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func15_oe_sel(&self) -> bool {
        (self.bits & Func15OutSelCfg::REG_GPIO_FUNC15_OE_SEL_MASK) != 0
    }

    /// Read **REG_GPIO_FUNC15_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_gpio_func15_oe_inv_sel(&self) -> bool {
        (self.bits & Func15OutSelCfg::REG_GPIO_FUNC15_OE_INV_SEL_MASK) != 0
    }

    /// Set **REG_GPIO_FUNC15_OUT_SEL** `[8:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func15_out_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Func15OutSelCfg::REG_GPIO_FUNC15_OUT_SEL_MASK)
                | ((value << Func15OutSelCfg::REG_GPIO_FUNC15_OUT_SEL_OFFSET)
                    & Func15OutSelCfg::REG_GPIO_FUNC15_OUT_SEL_MASK),
        }
    }

    /// Set **REG_GPIO_FUNC15_OUT_INV_SEL** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func15_out_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func15OutSelCfg::REG_GPIO_FUNC15_OUT_INV_SEL_MASK
            } else {
                self.bits & !Func15OutSelCfg::REG_GPIO_FUNC15_OUT_INV_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC15_OE_SEL** `[1:1]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func15_oe_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func15OutSelCfg::REG_GPIO_FUNC15_OE_SEL_MASK
            } else {
                self.bits & !Func15OutSelCfg::REG_GPIO_FUNC15_OE_SEL_MASK
            },
        }
    }

    /// Set **REG_GPIO_FUNC15_OE_INV_SEL** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_gpio_func15_oe_inv_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Func15OutSelCfg::REG_GPIO_FUNC15_OE_INV_SEL_MASK
            } else {
                self.bits & !Func15OutSelCfg::REG_GPIO_FUNC15_OE_INV_SEL_MASK
            },
        }
    }
}

impl Default for Func15OutSelCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Func15OutSelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Func15OutSelCfgVal {
        Func15OutSelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Func15OutSelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Func15OutSelCfgVal) -> Func15OutSelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Reserved
    pub clk_en: ClkEn,
    /// 0x0004
    /// Reserved
    pub ver_date: VerDate,
    /// 0x0008
    /// Reserved
    pub out: Out,
    /// 0x000c
    /// Reserved
    pub out_w1ts: OutW1ts,
    /// 0x0010
    /// Reserved
    pub out_w1tc: OutW1tc,
    /// 0x0014
    /// Reserved
    pub enable: Enable,
    /// 0x0018
    /// Reserved
    pub enable_w1ts: EnableW1ts,
    /// 0x001c
    /// Reserved
    pub enable_w1tc: EnableW1tc,
    /// 0x0020
    /// Reserved
    pub status: Status,
    /// 0x0024
    /// Reserved
    pub status_w1ts: StatusW1ts,
    /// 0x0028
    /// Reserved
    pub status_w1tc: StatusW1tc,
    /// 0x002c
    /// Reserved
    pub status_next: StatusNext,
    /// 0x0030
    /// Reserved
    pub r#in: In,
    /// 0x0034
    /// Reserved
    pub pin0: Pin0,
    /// 0x0038
    /// Reserved
    pub pin1: Pin1,
    /// 0x003c
    /// Reserved
    pub pin2: Pin2,
    /// 0x0040
    /// Reserved
    pub pin3: Pin3,
    /// 0x0044
    /// Reserved
    pub pin4: Pin4,
    /// 0x0048
    /// Reserved
    pub pin5: Pin5,
    /// 0x004c
    /// Reserved
    pub pin6: Pin6,
    /// 0x0050
    /// Reserved
    pub pin7: Pin7,
    /// 0x0054
    /// Reserved
    pub pin8: Pin8,
    /// 0x0058
    /// Reserved
    pub pin9: Pin9,
    /// 0x005c
    /// Reserved
    pub pin10: Pin10,
    /// 0x0060
    /// Reserved
    pub pin11: Pin11,
    /// 0x0064
    /// Reserved
    pub pin12: Pin12,
    /// 0x0068
    /// Reserved
    pub pin13: Pin13,
    /// 0x006c
    /// Reserved
    pub pin14: Pin14,
    /// 0x0070
    /// Reserved
    pub pin15: Pin15,
    /// 0x0074
    /// Reserved
    pub func0_in_sel_cfg: Func0InSelCfg,
    /// 0x0078
    /// Reserved
    pub func1_in_sel_cfg: Func1InSelCfg,
    /// 0x007c
    /// Reserved
    pub func2_in_sel_cfg: Func2InSelCfg,
    /// 0x0080
    /// Reserved
    pub func3_in_sel_cfg: Func3InSelCfg,
    /// 0x0084
    /// Reserved
    pub func4_in_sel_cfg: Func4InSelCfg,
    /// 0x0088
    /// Reserved
    pub func5_in_sel_cfg: Func5InSelCfg,
    /// 0x008c
    /// Reserved
    pub func6_in_sel_cfg: Func6InSelCfg,
    /// 0x0090
    /// Reserved
    pub func7_in_sel_cfg: Func7InSelCfg,
    /// 0x0094
    /// Reserved
    pub func8_in_sel_cfg: Func8InSelCfg,
    /// 0x0098
    /// Reserved
    pub func9_in_sel_cfg: Func9InSelCfg,
    /// 0x009c
    /// Reserved
    pub func10_in_sel_cfg: Func10InSelCfg,
    /// 0x00a0
    /// Reserved
    pub func11_in_sel_cfg: Func11InSelCfg,
    /// 0x00a4
    /// Reserved
    pub func12_in_sel_cfg: Func12InSelCfg,
    /// 0x00a8
    /// Reserved
    pub func13_in_sel_cfg: Func13InSelCfg,
    _reserved0: [u8; 72],
    /// 0x00f4
    /// Reserved
    pub func0_out_sel_cfg: Func0OutSelCfg,
    /// 0x00f8
    /// Reserved
    pub func1_out_sel_cfg: Func1OutSelCfg,
    /// 0x00fc
    /// Reserved
    pub func2_out_sel_cfg: Func2OutSelCfg,
    /// 0x0100
    /// Reserved
    pub func3_out_sel_cfg: Func3OutSelCfg,
    /// 0x0104
    /// Reserved
    pub func4_out_sel_cfg: Func4OutSelCfg,
    /// 0x0108
    /// Reserved
    pub func5_out_sel_cfg: Func5OutSelCfg,
    /// 0x010c
    /// Reserved
    pub func6_out_sel_cfg: Func6OutSelCfg,
    /// 0x0110
    /// Reserved
    pub func7_out_sel_cfg: Func7OutSelCfg,
    /// 0x0114
    /// Reserved
    pub func8_out_sel_cfg: Func8OutSelCfg,
    /// 0x0118
    /// Reserved
    pub func9_out_sel_cfg: Func9OutSelCfg,
    /// 0x011c
    /// Reserved
    pub func10_out_sel_cfg: Func10OutSelCfg,
    /// 0x0120
    /// Reserved
    pub func11_out_sel_cfg: Func11OutSelCfg,
    /// 0x0124
    /// Reserved
    pub func12_out_sel_cfg: Func12OutSelCfg,
    /// 0x0128
    /// Reserved
    pub func13_out_sel_cfg: Func13OutSelCfg,
    /// 0x012c
    /// Reserved
    pub func14_out_sel_cfg: Func14OutSelCfg,
    /// 0x0130
    /// Reserved
    pub func15_out_sel_cfg: Func15OutSelCfg,
}
