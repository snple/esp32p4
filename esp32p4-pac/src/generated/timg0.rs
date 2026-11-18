//! TIMG0
//! Timer Group 0

#![allow(dead_code)]

pub const BASE: u32 = 0x500c2000;

/// T0CONFIG Register
///
/// Timer %s configuration register
///
/// Address offset: 0x0000
/// Reset value: 0x60002000
///
/// ## Fields
///
/// - **EN** `[31]` (RW)
///
///   When set, the timer %s time-base counter is enabled.
///
/// - **INCREASE** `[30]` (RW)
///
///   When set, the timer %s time-base counter will increment every clock tick. When
///   cleared, the timer %s time-base counter will decrement.
///
/// - **AUTORELOAD** `[29]` (RW)
///
///   When set, timer %s auto-reload at alarm is enabled.
///
/// - **DIVIDER** `[28:13]` (RW)
///
///   Timer %s clock (T%s_clk) prescaler value.
///
/// - **DIVCNT_RST** `[12]` (WO)
///
///   When set, Timer %s 's clock divider counter will be reset.
///
/// - **ALARM_EN** `[10]` (RW)
///
///   When set, the alarm is enabled. This bit is automatically cleared once an
///   alarm occurs.
///
/// - **USE_XTAL** `[9]` (RW)
///
///   1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group.
#[repr(transparent)]
pub struct T0config {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T0config {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T0config {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// When set, the timer %s time-base counter is enabled.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const EN_OFFSET: u32 = 31;
    pub const EN_MASK: u32 = 0x80000000;
    pub const EN: u32 = 1 << 31;

    /// **INCREASE**
    ///
    /// When set, the timer %s time-base counter will increment every clock tick. When
    /// cleared, the timer %s time-base counter will decrement.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const INCREASE_OFFSET: u32 = 30;
    pub const INCREASE_MASK: u32 = 0x40000000;
    pub const INCREASE: u32 = 1 << 30;

    /// **AUTORELOAD**
    ///
    /// When set, timer %s auto-reload at alarm is enabled.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const AUTORELOAD_OFFSET: u32 = 29;
    pub const AUTORELOAD_MASK: u32 = 0x20000000;
    pub const AUTORELOAD: u32 = 1 << 29;

    /// **DIVIDER**
    ///
    /// Timer %s clock (T%s_clk) prescaler value.
    ///
    /// Bits `[28:13]`, mask `0x1fffe000`
    pub const DIVIDER_OFFSET: u32 = 13;
    pub const DIVIDER_MASK: u32 = 0x1fffe000;

    /// **DIVCNT_RST**
    ///
    /// When set, Timer %s 's clock divider counter will be reset.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const DIVCNT_RST_OFFSET: u32 = 12;
    pub const DIVCNT_RST_MASK: u32 = 0x00001000;
    pub const DIVCNT_RST: u32 = 1 << 12;

    /// **ALARM_EN**
    ///
    /// When set, the alarm is enabled. This bit is automatically cleared once an
    /// alarm occurs.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ALARM_EN_OFFSET: u32 = 10;
    pub const ALARM_EN_MASK: u32 = 0x00000400;
    pub const ALARM_EN: u32 = 1 << 10;

    /// **USE_XTAL**
    ///
    /// 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const USE_XTAL_OFFSET: u32 = 9;
    pub const USE_XTAL_MASK: u32 = 0x00000200;
    pub const USE_XTAL: u32 = 1 << 9;

    // ======== Field Accessors ========

    /// **EN** `[31]` (RW)
    ///
    /// When set, the timer %s time-base counter is enabled.
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

    /// **INCREASE** `[30]` (RW)
    ///
    /// When set, the timer %s time-base counter will increment every clock tick. When
    /// cleared, the timer %s time-base counter will decrement.
    #[inline(always)]
    pub fn increase(&self) -> bool {
        (self.value.read_bits() & Self::INCREASE_MASK) != 0
    }

    /// Set INCREASE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_increase(&self) {
        self.value.modify_bits(|r| r | Self::INCREASE_MASK);
    }

    /// Clear INCREASE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_increase(&self) {
        self.value.modify_bits(|r| r & !Self::INCREASE_MASK);
    }

    /// **AUTORELOAD** `[29]` (RW)
    ///
    /// When set, timer %s auto-reload at alarm is enabled.
    #[inline(always)]
    pub fn autoreload(&self) -> bool {
        (self.value.read_bits() & Self::AUTORELOAD_MASK) != 0
    }

    /// Set AUTORELOAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_autoreload(&self) {
        self.value.modify_bits(|r| r | Self::AUTORELOAD_MASK);
    }

    /// Clear AUTORELOAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_autoreload(&self) {
        self.value.modify_bits(|r| r & !Self::AUTORELOAD_MASK);
    }

    /// **DIVIDER** `[28:13]` (RW)
    ///
    /// Timer %s clock (T%s_clk) prescaler value.
    #[inline(always)]
    pub fn divider(&self) -> u32 {
        (self.value.read_bits() & Self::DIVIDER_MASK) >> Self::DIVIDER_OFFSET
    }

    /// Write **DIVIDER** `[28:13]` (RW)
    ///
    /// Timer %s clock (T%s_clk) prescaler value.
    #[inline(always)]
    pub fn write_divider(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DIVIDER_MASK) | ((value << Self::DIVIDER_OFFSET) & Self::DIVIDER_MASK)
        });
    }

    /// Trigger **DIVCNT_RST** (WO)
    ///
    /// When set, Timer %s 's clock divider counter will be reset.
    #[inline(always)]
    pub fn trigger_divcnt_rst(&self) {
        self.value.modify_bits(|r| r | Self::DIVCNT_RST_MASK);
    }

    /// **ALARM_EN** `[10]` (RW)
    ///
    /// When set, the alarm is enabled. This bit is automatically cleared once an
    /// alarm occurs.
    #[inline(always)]
    pub fn alarm_en(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_EN_MASK) != 0
    }

    /// Set ALARM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_alarm_en(&self) {
        self.value.modify_bits(|r| r | Self::ALARM_EN_MASK);
    }

    /// Clear ALARM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_alarm_en(&self) {
        self.value.modify_bits(|r| r & !Self::ALARM_EN_MASK);
    }

    /// **USE_XTAL** `[9]` (RW)
    ///
    /// 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group.
    #[inline(always)]
    pub fn use_xtal(&self) -> bool {
        (self.value.read_bits() & Self::USE_XTAL_MASK) != 0
    }

    /// Set USE_XTAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_use_xtal(&self) {
        self.value.modify_bits(|r| r | Self::USE_XTAL_MASK);
    }

    /// Clear USE_XTAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_use_xtal(&self) {
        self.value.modify_bits(|r| r & !Self::USE_XTAL_MASK);
    }
}

/// T0CONFIG value type for batch read/modify/write operations
///
/// Use `t0config.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t0config.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T0configVal {
    bits: u32,
}

impl T0configVal {
    /// Create from raw bits
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
        Self { bits: 0x60002000 }
    }

    /// Read **EN** `[31:31]`
    ///
    /// When set, the timer %s time-base counter is enabled.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & T0config::EN_MASK) != 0
    }

    /// Read **INCREASE** `[30:30]`
    ///
    /// When set, the timer %s time-base counter will increment every clock tick. When
    /// cleared, the timer %s time-base counter will decrement.
    #[inline(always)]
    pub const fn increase(&self) -> bool {
        (self.bits & T0config::INCREASE_MASK) != 0
    }

    /// Read **AUTORELOAD** `[29:29]`
    ///
    /// When set, timer %s auto-reload at alarm is enabled.
    #[inline(always)]
    pub const fn autoreload(&self) -> bool {
        (self.bits & T0config::AUTORELOAD_MASK) != 0
    }

    /// Read **DIVIDER** `[28:13]`
    ///
    /// Timer %s clock (T%s_clk) prescaler value.
    #[inline(always)]
    pub const fn divider(&self) -> u32 {
        (self.bits & T0config::DIVIDER_MASK) >> T0config::DIVIDER_OFFSET
    }

    /// Read **ALARM_EN** `[10:10]`
    ///
    /// When set, the alarm is enabled. This bit is automatically cleared once an
    /// alarm occurs.
    #[inline(always)]
    pub const fn alarm_en(&self) -> bool {
        (self.bits & T0config::ALARM_EN_MASK) != 0
    }

    /// Read **USE_XTAL** `[9:9]`
    ///
    /// 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group.
    #[inline(always)]
    pub const fn use_xtal(&self) -> bool {
        (self.bits & T0config::USE_XTAL_MASK) != 0
    }

    /// Set **EN** `[31:31]`
    ///
    /// When set, the timer %s time-base counter is enabled.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | T0config::EN_MASK
            } else {
                self.bits & !T0config::EN_MASK
            },
        }
    }

    /// Set **INCREASE** `[30:30]`
    ///
    /// When set, the timer %s time-base counter will increment every clock tick. When
    /// cleared, the timer %s time-base counter will decrement.
    #[inline(always)]
    pub const fn with_increase(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | T0config::INCREASE_MASK
            } else {
                self.bits & !T0config::INCREASE_MASK
            },
        }
    }

    /// Set **AUTORELOAD** `[29:29]`
    ///
    /// When set, timer %s auto-reload at alarm is enabled.
    #[inline(always)]
    pub const fn with_autoreload(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | T0config::AUTORELOAD_MASK
            } else {
                self.bits & !T0config::AUTORELOAD_MASK
            },
        }
    }

    /// Set **DIVIDER** `[28:13]`
    ///
    /// Timer %s clock (T%s_clk) prescaler value.
    #[inline(always)]
    pub const fn with_divider(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !T0config::DIVIDER_MASK)
                | ((value << T0config::DIVIDER_OFFSET) & T0config::DIVIDER_MASK),
        }
    }

    /// Set **DIVCNT_RST** `[12:12]`
    ///
    /// When set, Timer %s 's clock divider counter will be reset.
    #[inline(always)]
    pub const fn with_divcnt_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | T0config::DIVCNT_RST_MASK
            } else {
                self.bits & !T0config::DIVCNT_RST_MASK
            },
        }
    }

    /// Set **ALARM_EN** `[10:10]`
    ///
    /// When set, the alarm is enabled. This bit is automatically cleared once an
    /// alarm occurs.
    #[inline(always)]
    pub const fn with_alarm_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | T0config::ALARM_EN_MASK
            } else {
                self.bits & !T0config::ALARM_EN_MASK
            },
        }
    }

    /// Set **USE_XTAL** `[9:9]`
    ///
    /// 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group.
    #[inline(always)]
    pub const fn with_use_xtal(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | T0config::USE_XTAL_MASK
            } else {
                self.bits & !T0config::USE_XTAL_MASK
            },
        }
    }
}

impl Default for T0configVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl T0config {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T0configVal {
        T0configVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: T0configVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(T0configVal) -> T0configVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T0LO Register
///
/// Timer %s current value, low 32 bits
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **LO** `[31:0]` (RO)
///
///   After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter
///   of timer %s can be read here.
#[repr(transparent)]
pub struct T0lo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T0lo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T0lo {
    // ======== Field Constants ========

    /// **LO**
    ///
    /// After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter
    /// of timer %s can be read here.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LO_OFFSET: u32 = 0;
    pub const LO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LO** `[31:0]` (RO)
    ///
    /// After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter
    /// of timer %s can be read here.
    #[inline(always)]
    pub fn lo(&self) -> u32 {
        (self.value.read_bits() & Self::LO_MASK) >> Self::LO_OFFSET
    }
}

/// T0LO value type for batch read/modify/write operations
///
/// Use `t0lo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t0lo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T0loVal {
    bits: u32,
}

impl T0loVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LO** `[31:0]`
    ///
    /// After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter
    /// of timer %s can be read here.
    #[inline(always)]
    pub const fn lo(&self) -> u32 {
        (self.bits & T0lo::LO_MASK) >> T0lo::LO_OFFSET
    }
}

impl Default for T0loVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T0lo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T0loVal {
        T0loVal::from_bits(self.value.read_bits())
    }
}

/// T0HI Register
///
/// Timer %s current value, high 22 bits
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **HI** `[21:0]` (RO)
///
///   After writing to TIMG_T%sUPDATE_REG, the high 22 bits of the time-base counter
///   of timer %s can be read here.
#[repr(transparent)]
pub struct T0hi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T0hi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T0hi {
    // ======== Field Constants ========

    /// **HI**
    ///
    /// After writing to TIMG_T%sUPDATE_REG, the high 22 bits of the time-base counter
    /// of timer %s can be read here.
    ///
    /// Bits `[21:0]`, mask `0x003fffff`
    pub const HI_OFFSET: u32 = 0;
    pub const HI_MASK: u32 = 0x003fffff;

    // ======== Field Accessors ========

    /// **HI** `[21:0]` (RO)
    ///
    /// After writing to TIMG_T%sUPDATE_REG, the high 22 bits of the time-base counter
    /// of timer %s can be read here.
    #[inline(always)]
    pub fn hi(&self) -> u32 {
        (self.value.read_bits() & Self::HI_MASK) >> Self::HI_OFFSET
    }
}

/// T0HI value type for batch read/modify/write operations
///
/// Use `t0hi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t0hi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T0hiVal {
    bits: u32,
}

impl T0hiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HI** `[21:0]`
    ///
    /// After writing to TIMG_T%sUPDATE_REG, the high 22 bits of the time-base counter
    /// of timer %s can be read here.
    #[inline(always)]
    pub const fn hi(&self) -> u32 {
        (self.bits & T0hi::HI_MASK) >> T0hi::HI_OFFSET
    }
}

impl Default for T0hiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T0hi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T0hiVal {
        T0hiVal::from_bits(self.value.read_bits())
    }
}

/// T0UPDATE Register
///
/// Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **UPDATE** `[31]` (RW)
///
///   After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched.
#[repr(transparent)]
pub struct T0update {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T0update {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T0update {
    // ======== Field Constants ========

    /// **UPDATE**
    ///
    /// After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const UPDATE_OFFSET: u32 = 31;
    pub const UPDATE_MASK: u32 = 0x80000000;
    pub const UPDATE: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **UPDATE** `[31]` (RW)
    ///
    /// After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched.
    #[inline(always)]
    pub fn update(&self) -> bool {
        (self.value.read_bits() & Self::UPDATE_MASK) != 0
    }

    /// Set UPDATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_update(&self) {
        self.value.modify_bits(|r| r | Self::UPDATE_MASK);
    }

    /// Clear UPDATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_update(&self) {
        self.value.modify_bits(|r| r & !Self::UPDATE_MASK);
    }
}

/// T0UPDATE value type for batch read/modify/write operations
///
/// Use `t0update.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t0update.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T0updateVal {
    bits: u32,
}

impl T0updateVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **UPDATE** `[31:31]`
    ///
    /// After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched.
    #[inline(always)]
    pub const fn update(&self) -> bool {
        (self.bits & T0update::UPDATE_MASK) != 0
    }

    /// Set **UPDATE** `[31:31]`
    ///
    /// After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched.
    #[inline(always)]
    pub const fn with_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | T0update::UPDATE_MASK
            } else {
                self.bits & !T0update::UPDATE_MASK
            },
        }
    }
}

impl Default for T0updateVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T0update {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T0updateVal {
        T0updateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: T0updateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(T0updateVal) -> T0updateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T0ALARMLO Register
///
/// Timer %s alarm value, low 32 bits
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **ALARM_LO** `[31:0]` (RW)
///
///   Timer %s alarm trigger time-base counter value, low 32 bits.
#[repr(transparent)]
pub struct T0alarmlo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T0alarmlo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T0alarmlo {
    // ======== Field Constants ========

    /// **ALARM_LO**
    ///
    /// Timer %s alarm trigger time-base counter value, low 32 bits.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ALARM_LO_OFFSET: u32 = 0;
    pub const ALARM_LO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ALARM_LO** `[31:0]` (RW)
    ///
    /// Timer %s alarm trigger time-base counter value, low 32 bits.
    #[inline(always)]
    pub fn alarm_lo(&self) -> u32 {
        (self.value.read_bits() & Self::ALARM_LO_MASK) >> Self::ALARM_LO_OFFSET
    }

    /// Write **ALARM_LO** `[31:0]` (RW)
    ///
    /// Timer %s alarm trigger time-base counter value, low 32 bits.
    #[inline(always)]
    pub fn write_alarm_lo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ALARM_LO_MASK) | ((value << Self::ALARM_LO_OFFSET) & Self::ALARM_LO_MASK)
        });
    }
}

/// T0ALARMLO value type for batch read/modify/write operations
///
/// Use `t0alarmlo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t0alarmlo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T0alarmloVal {
    bits: u32,
}

impl T0alarmloVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ALARM_LO** `[31:0]`
    ///
    /// Timer %s alarm trigger time-base counter value, low 32 bits.
    #[inline(always)]
    pub const fn alarm_lo(&self) -> u32 {
        (self.bits & T0alarmlo::ALARM_LO_MASK) >> T0alarmlo::ALARM_LO_OFFSET
    }

    /// Set **ALARM_LO** `[31:0]`
    ///
    /// Timer %s alarm trigger time-base counter value, low 32 bits.
    #[inline(always)]
    pub const fn with_alarm_lo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !T0alarmlo::ALARM_LO_MASK)
                | ((value << T0alarmlo::ALARM_LO_OFFSET) & T0alarmlo::ALARM_LO_MASK),
        }
    }
}

impl Default for T0alarmloVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T0alarmlo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T0alarmloVal {
        T0alarmloVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: T0alarmloVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(T0alarmloVal) -> T0alarmloVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T0ALARMHI Register
///
/// Timer %s alarm value, high bits
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **ALARM_HI** `[21:0]` (RW)
///
///   Timer %s alarm trigger time-base counter value, high 22 bits.
#[repr(transparent)]
pub struct T0alarmhi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T0alarmhi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T0alarmhi {
    // ======== Field Constants ========

    /// **ALARM_HI**
    ///
    /// Timer %s alarm trigger time-base counter value, high 22 bits.
    ///
    /// Bits `[21:0]`, mask `0x003fffff`
    pub const ALARM_HI_OFFSET: u32 = 0;
    pub const ALARM_HI_MASK: u32 = 0x003fffff;

    // ======== Field Accessors ========

    /// **ALARM_HI** `[21:0]` (RW)
    ///
    /// Timer %s alarm trigger time-base counter value, high 22 bits.
    #[inline(always)]
    pub fn alarm_hi(&self) -> u32 {
        (self.value.read_bits() & Self::ALARM_HI_MASK) >> Self::ALARM_HI_OFFSET
    }

    /// Write **ALARM_HI** `[21:0]` (RW)
    ///
    /// Timer %s alarm trigger time-base counter value, high 22 bits.
    #[inline(always)]
    pub fn write_alarm_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ALARM_HI_MASK) | ((value << Self::ALARM_HI_OFFSET) & Self::ALARM_HI_MASK)
        });
    }
}

/// T0ALARMHI value type for batch read/modify/write operations
///
/// Use `t0alarmhi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t0alarmhi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T0alarmhiVal {
    bits: u32,
}

impl T0alarmhiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ALARM_HI** `[21:0]`
    ///
    /// Timer %s alarm trigger time-base counter value, high 22 bits.
    #[inline(always)]
    pub const fn alarm_hi(&self) -> u32 {
        (self.bits & T0alarmhi::ALARM_HI_MASK) >> T0alarmhi::ALARM_HI_OFFSET
    }

    /// Set **ALARM_HI** `[21:0]`
    ///
    /// Timer %s alarm trigger time-base counter value, high 22 bits.
    #[inline(always)]
    pub const fn with_alarm_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !T0alarmhi::ALARM_HI_MASK)
                | ((value << T0alarmhi::ALARM_HI_OFFSET) & T0alarmhi::ALARM_HI_MASK),
        }
    }
}

impl Default for T0alarmhiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T0alarmhi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T0alarmhiVal {
        T0alarmhiVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: T0alarmhiVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(T0alarmhiVal) -> T0alarmhiVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T0LOADLO Register
///
/// Timer %s reload value, low 32 bits
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **LOAD_LO** `[31:0]` (RW)
///
///   Low 32 bits of the value that a reload will load onto timer %s time-base
///   Counter.
#[repr(transparent)]
pub struct T0loadlo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T0loadlo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T0loadlo {
    // ======== Field Constants ========

    /// **LOAD_LO**
    ///
    /// Low 32 bits of the value that a reload will load onto timer %s time-base
    /// Counter.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LOAD_LO_OFFSET: u32 = 0;
    pub const LOAD_LO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LOAD_LO** `[31:0]` (RW)
    ///
    /// Low 32 bits of the value that a reload will load onto timer %s time-base
    /// Counter.
    #[inline(always)]
    pub fn load_lo(&self) -> u32 {
        (self.value.read_bits() & Self::LOAD_LO_MASK) >> Self::LOAD_LO_OFFSET
    }

    /// Write **LOAD_LO** `[31:0]` (RW)
    ///
    /// Low 32 bits of the value that a reload will load onto timer %s time-base
    /// Counter.
    #[inline(always)]
    pub fn write_load_lo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LOAD_LO_MASK) | ((value << Self::LOAD_LO_OFFSET) & Self::LOAD_LO_MASK)
        });
    }
}

/// T0LOADLO value type for batch read/modify/write operations
///
/// Use `t0loadlo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t0loadlo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T0loadloVal {
    bits: u32,
}

impl T0loadloVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LOAD_LO** `[31:0]`
    ///
    /// Low 32 bits of the value that a reload will load onto timer %s time-base
    /// Counter.
    #[inline(always)]
    pub const fn load_lo(&self) -> u32 {
        (self.bits & T0loadlo::LOAD_LO_MASK) >> T0loadlo::LOAD_LO_OFFSET
    }

    /// Set **LOAD_LO** `[31:0]`
    ///
    /// Low 32 bits of the value that a reload will load onto timer %s time-base
    /// Counter.
    #[inline(always)]
    pub const fn with_load_lo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !T0loadlo::LOAD_LO_MASK)
                | ((value << T0loadlo::LOAD_LO_OFFSET) & T0loadlo::LOAD_LO_MASK),
        }
    }
}

impl Default for T0loadloVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T0loadlo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T0loadloVal {
        T0loadloVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: T0loadloVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(T0loadloVal) -> T0loadloVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T0LOADHI Register
///
/// Timer %s reload value, high 22 bits
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **LOAD_HI** `[21:0]` (RW)
///
///   High 22 bits of the value that a reload will load onto timer %s time-base
///   counter.
#[repr(transparent)]
pub struct T0loadhi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T0loadhi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T0loadhi {
    // ======== Field Constants ========

    /// **LOAD_HI**
    ///
    /// High 22 bits of the value that a reload will load onto timer %s time-base
    /// counter.
    ///
    /// Bits `[21:0]`, mask `0x003fffff`
    pub const LOAD_HI_OFFSET: u32 = 0;
    pub const LOAD_HI_MASK: u32 = 0x003fffff;

    // ======== Field Accessors ========

    /// **LOAD_HI** `[21:0]` (RW)
    ///
    /// High 22 bits of the value that a reload will load onto timer %s time-base
    /// counter.
    #[inline(always)]
    pub fn load_hi(&self) -> u32 {
        (self.value.read_bits() & Self::LOAD_HI_MASK) >> Self::LOAD_HI_OFFSET
    }

    /// Write **LOAD_HI** `[21:0]` (RW)
    ///
    /// High 22 bits of the value that a reload will load onto timer %s time-base
    /// counter.
    #[inline(always)]
    pub fn write_load_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LOAD_HI_MASK) | ((value << Self::LOAD_HI_OFFSET) & Self::LOAD_HI_MASK)
        });
    }
}

/// T0LOADHI value type for batch read/modify/write operations
///
/// Use `t0loadhi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t0loadhi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T0loadhiVal {
    bits: u32,
}

impl T0loadhiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LOAD_HI** `[21:0]`
    ///
    /// High 22 bits of the value that a reload will load onto timer %s time-base
    /// counter.
    #[inline(always)]
    pub const fn load_hi(&self) -> u32 {
        (self.bits & T0loadhi::LOAD_HI_MASK) >> T0loadhi::LOAD_HI_OFFSET
    }

    /// Set **LOAD_HI** `[21:0]`
    ///
    /// High 22 bits of the value that a reload will load onto timer %s time-base
    /// counter.
    #[inline(always)]
    pub const fn with_load_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !T0loadhi::LOAD_HI_MASK)
                | ((value << T0loadhi::LOAD_HI_OFFSET) & T0loadhi::LOAD_HI_MASK),
        }
    }
}

impl Default for T0loadhiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T0loadhi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T0loadhiVal {
        T0loadhiVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: T0loadhiVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(T0loadhiVal) -> T0loadhiVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T0LOAD Register
///
/// Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **LOAD** `[31:0]` (WO)
///
///   Write any value to trigger a timer %s time-base counter reload.
#[repr(transparent)]
pub struct T0load {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T0load {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T0load {
    // ======== Field Constants ========

    /// **LOAD**
    ///
    /// Write any value to trigger a timer %s time-base counter reload.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LOAD_OFFSET: u32 = 0;
    pub const LOAD_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// Write **LOAD** `[31:0]` (WO)
    ///
    /// Write any value to trigger a timer %s time-base counter reload.
    #[inline(always)]
    pub fn write_load(&self, value: u32) {
        self.value
            .write_bits((value << Self::LOAD_OFFSET) & Self::LOAD_MASK);
    }
}

/// T0LOAD value type for batch read/modify/write operations
///
/// Use `t0load.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t0load.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T0loadVal {
    bits: u32,
}

impl T0loadVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **LOAD** `[31:0]`
    ///
    /// Write any value to trigger a timer %s time-base counter reload.
    #[inline(always)]
    pub const fn with_load(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !T0load::LOAD_MASK)
                | ((value << T0load::LOAD_OFFSET) & T0load::LOAD_MASK),
        }
    }
}

impl Default for T0loadVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T0load {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: T0loadVal) {
        self.value.write_bits(val.bits())
    }
}

/// T1CONFIG Register
///
/// Timer %s configuration register
///
/// Address offset: 0x0024
/// Reset value: 0x60002000
///
/// ## Fields
///
/// - **EN** `[31]` (RW)
///
///   When set, the timer %s time-base counter is enabled.
///
/// - **INCREASE** `[30]` (RW)
///
///   When set, the timer %s time-base counter will increment every clock tick. When
///   cleared, the timer %s time-base counter will decrement.
///
/// - **AUTORELOAD** `[29]` (RW)
///
///   When set, timer %s auto-reload at alarm is enabled.
///
/// - **DIVIDER** `[28:13]` (RW)
///
///   Timer %s clock (T%s_clk) prescaler value.
///
/// - **DIVCNT_RST** `[12]` (WO)
///
///   When set, Timer %s 's clock divider counter will be reset.
///
/// - **ALARM_EN** `[10]` (RW)
///
///   When set, the alarm is enabled. This bit is automatically cleared once an
///   alarm occurs.
///
/// - **USE_XTAL** `[9]` (RW)
///
///   1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group.
#[repr(transparent)]
pub struct T1config {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T1config {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T1config {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// When set, the timer %s time-base counter is enabled.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const EN_OFFSET: u32 = 31;
    pub const EN_MASK: u32 = 0x80000000;
    pub const EN: u32 = 1 << 31;

    /// **INCREASE**
    ///
    /// When set, the timer %s time-base counter will increment every clock tick. When
    /// cleared, the timer %s time-base counter will decrement.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const INCREASE_OFFSET: u32 = 30;
    pub const INCREASE_MASK: u32 = 0x40000000;
    pub const INCREASE: u32 = 1 << 30;

    /// **AUTORELOAD**
    ///
    /// When set, timer %s auto-reload at alarm is enabled.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const AUTORELOAD_OFFSET: u32 = 29;
    pub const AUTORELOAD_MASK: u32 = 0x20000000;
    pub const AUTORELOAD: u32 = 1 << 29;

    /// **DIVIDER**
    ///
    /// Timer %s clock (T%s_clk) prescaler value.
    ///
    /// Bits `[28:13]`, mask `0x1fffe000`
    pub const DIVIDER_OFFSET: u32 = 13;
    pub const DIVIDER_MASK: u32 = 0x1fffe000;

    /// **DIVCNT_RST**
    ///
    /// When set, Timer %s 's clock divider counter will be reset.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const DIVCNT_RST_OFFSET: u32 = 12;
    pub const DIVCNT_RST_MASK: u32 = 0x00001000;
    pub const DIVCNT_RST: u32 = 1 << 12;

    /// **ALARM_EN**
    ///
    /// When set, the alarm is enabled. This bit is automatically cleared once an
    /// alarm occurs.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const ALARM_EN_OFFSET: u32 = 10;
    pub const ALARM_EN_MASK: u32 = 0x00000400;
    pub const ALARM_EN: u32 = 1 << 10;

    /// **USE_XTAL**
    ///
    /// 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const USE_XTAL_OFFSET: u32 = 9;
    pub const USE_XTAL_MASK: u32 = 0x00000200;
    pub const USE_XTAL: u32 = 1 << 9;

    // ======== Field Accessors ========

    /// **EN** `[31]` (RW)
    ///
    /// When set, the timer %s time-base counter is enabled.
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

    /// **INCREASE** `[30]` (RW)
    ///
    /// When set, the timer %s time-base counter will increment every clock tick. When
    /// cleared, the timer %s time-base counter will decrement.
    #[inline(always)]
    pub fn increase(&self) -> bool {
        (self.value.read_bits() & Self::INCREASE_MASK) != 0
    }

    /// Set INCREASE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_increase(&self) {
        self.value.modify_bits(|r| r | Self::INCREASE_MASK);
    }

    /// Clear INCREASE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_increase(&self) {
        self.value.modify_bits(|r| r & !Self::INCREASE_MASK);
    }

    /// **AUTORELOAD** `[29]` (RW)
    ///
    /// When set, timer %s auto-reload at alarm is enabled.
    #[inline(always)]
    pub fn autoreload(&self) -> bool {
        (self.value.read_bits() & Self::AUTORELOAD_MASK) != 0
    }

    /// Set AUTORELOAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_autoreload(&self) {
        self.value.modify_bits(|r| r | Self::AUTORELOAD_MASK);
    }

    /// Clear AUTORELOAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_autoreload(&self) {
        self.value.modify_bits(|r| r & !Self::AUTORELOAD_MASK);
    }

    /// **DIVIDER** `[28:13]` (RW)
    ///
    /// Timer %s clock (T%s_clk) prescaler value.
    #[inline(always)]
    pub fn divider(&self) -> u32 {
        (self.value.read_bits() & Self::DIVIDER_MASK) >> Self::DIVIDER_OFFSET
    }

    /// Write **DIVIDER** `[28:13]` (RW)
    ///
    /// Timer %s clock (T%s_clk) prescaler value.
    #[inline(always)]
    pub fn write_divider(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DIVIDER_MASK) | ((value << Self::DIVIDER_OFFSET) & Self::DIVIDER_MASK)
        });
    }

    /// Trigger **DIVCNT_RST** (WO)
    ///
    /// When set, Timer %s 's clock divider counter will be reset.
    #[inline(always)]
    pub fn trigger_divcnt_rst(&self) {
        self.value.modify_bits(|r| r | Self::DIVCNT_RST_MASK);
    }

    /// **ALARM_EN** `[10]` (RW)
    ///
    /// When set, the alarm is enabled. This bit is automatically cleared once an
    /// alarm occurs.
    #[inline(always)]
    pub fn alarm_en(&self) -> bool {
        (self.value.read_bits() & Self::ALARM_EN_MASK) != 0
    }

    /// Set ALARM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_alarm_en(&self) {
        self.value.modify_bits(|r| r | Self::ALARM_EN_MASK);
    }

    /// Clear ALARM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_alarm_en(&self) {
        self.value.modify_bits(|r| r & !Self::ALARM_EN_MASK);
    }

    /// **USE_XTAL** `[9]` (RW)
    ///
    /// 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group.
    #[inline(always)]
    pub fn use_xtal(&self) -> bool {
        (self.value.read_bits() & Self::USE_XTAL_MASK) != 0
    }

    /// Set USE_XTAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_use_xtal(&self) {
        self.value.modify_bits(|r| r | Self::USE_XTAL_MASK);
    }

    /// Clear USE_XTAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_use_xtal(&self) {
        self.value.modify_bits(|r| r & !Self::USE_XTAL_MASK);
    }
}

/// T1CONFIG value type for batch read/modify/write operations
///
/// Use `t1config.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t1config.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T1configVal {
    bits: u32,
}

impl T1configVal {
    /// Create from raw bits
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
        Self { bits: 0x60002000 }
    }

    /// Read **EN** `[31:31]`
    ///
    /// When set, the timer %s time-base counter is enabled.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & T1config::EN_MASK) != 0
    }

    /// Read **INCREASE** `[30:30]`
    ///
    /// When set, the timer %s time-base counter will increment every clock tick. When
    /// cleared, the timer %s time-base counter will decrement.
    #[inline(always)]
    pub const fn increase(&self) -> bool {
        (self.bits & T1config::INCREASE_MASK) != 0
    }

    /// Read **AUTORELOAD** `[29:29]`
    ///
    /// When set, timer %s auto-reload at alarm is enabled.
    #[inline(always)]
    pub const fn autoreload(&self) -> bool {
        (self.bits & T1config::AUTORELOAD_MASK) != 0
    }

    /// Read **DIVIDER** `[28:13]`
    ///
    /// Timer %s clock (T%s_clk) prescaler value.
    #[inline(always)]
    pub const fn divider(&self) -> u32 {
        (self.bits & T1config::DIVIDER_MASK) >> T1config::DIVIDER_OFFSET
    }

    /// Read **ALARM_EN** `[10:10]`
    ///
    /// When set, the alarm is enabled. This bit is automatically cleared once an
    /// alarm occurs.
    #[inline(always)]
    pub const fn alarm_en(&self) -> bool {
        (self.bits & T1config::ALARM_EN_MASK) != 0
    }

    /// Read **USE_XTAL** `[9:9]`
    ///
    /// 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group.
    #[inline(always)]
    pub const fn use_xtal(&self) -> bool {
        (self.bits & T1config::USE_XTAL_MASK) != 0
    }

    /// Set **EN** `[31:31]`
    ///
    /// When set, the timer %s time-base counter is enabled.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | T1config::EN_MASK
            } else {
                self.bits & !T1config::EN_MASK
            },
        }
    }

    /// Set **INCREASE** `[30:30]`
    ///
    /// When set, the timer %s time-base counter will increment every clock tick. When
    /// cleared, the timer %s time-base counter will decrement.
    #[inline(always)]
    pub const fn with_increase(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | T1config::INCREASE_MASK
            } else {
                self.bits & !T1config::INCREASE_MASK
            },
        }
    }

    /// Set **AUTORELOAD** `[29:29]`
    ///
    /// When set, timer %s auto-reload at alarm is enabled.
    #[inline(always)]
    pub const fn with_autoreload(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | T1config::AUTORELOAD_MASK
            } else {
                self.bits & !T1config::AUTORELOAD_MASK
            },
        }
    }

    /// Set **DIVIDER** `[28:13]`
    ///
    /// Timer %s clock (T%s_clk) prescaler value.
    #[inline(always)]
    pub const fn with_divider(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !T1config::DIVIDER_MASK)
                | ((value << T1config::DIVIDER_OFFSET) & T1config::DIVIDER_MASK),
        }
    }

    /// Set **DIVCNT_RST** `[12:12]`
    ///
    /// When set, Timer %s 's clock divider counter will be reset.
    #[inline(always)]
    pub const fn with_divcnt_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | T1config::DIVCNT_RST_MASK
            } else {
                self.bits & !T1config::DIVCNT_RST_MASK
            },
        }
    }

    /// Set **ALARM_EN** `[10:10]`
    ///
    /// When set, the alarm is enabled. This bit is automatically cleared once an
    /// alarm occurs.
    #[inline(always)]
    pub const fn with_alarm_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | T1config::ALARM_EN_MASK
            } else {
                self.bits & !T1config::ALARM_EN_MASK
            },
        }
    }

    /// Set **USE_XTAL** `[9:9]`
    ///
    /// 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group.
    #[inline(always)]
    pub const fn with_use_xtal(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | T1config::USE_XTAL_MASK
            } else {
                self.bits & !T1config::USE_XTAL_MASK
            },
        }
    }
}

impl Default for T1configVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl T1config {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T1configVal {
        T1configVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: T1configVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(T1configVal) -> T1configVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T1LO Register
///
/// Timer %s current value, low 32 bits
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **LO** `[31:0]` (RO)
///
///   After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter
///   of timer %s can be read here.
#[repr(transparent)]
pub struct T1lo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T1lo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T1lo {
    // ======== Field Constants ========

    /// **LO**
    ///
    /// After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter
    /// of timer %s can be read here.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LO_OFFSET: u32 = 0;
    pub const LO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LO** `[31:0]` (RO)
    ///
    /// After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter
    /// of timer %s can be read here.
    #[inline(always)]
    pub fn lo(&self) -> u32 {
        (self.value.read_bits() & Self::LO_MASK) >> Self::LO_OFFSET
    }
}

/// T1LO value type for batch read/modify/write operations
///
/// Use `t1lo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t1lo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T1loVal {
    bits: u32,
}

impl T1loVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LO** `[31:0]`
    ///
    /// After writing to TIMG_T%sUPDATE_REG, the low 32 bits of the time-base counter
    /// of timer %s can be read here.
    #[inline(always)]
    pub const fn lo(&self) -> u32 {
        (self.bits & T1lo::LO_MASK) >> T1lo::LO_OFFSET
    }
}

impl Default for T1loVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T1lo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T1loVal {
        T1loVal::from_bits(self.value.read_bits())
    }
}

/// T1HI Register
///
/// Timer %s current value, high 22 bits
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **HI** `[21:0]` (RO)
///
///   After writing to TIMG_T%sUPDATE_REG, the high 22 bits of the time-base counter
///   of timer %s can be read here.
#[repr(transparent)]
pub struct T1hi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T1hi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T1hi {
    // ======== Field Constants ========

    /// **HI**
    ///
    /// After writing to TIMG_T%sUPDATE_REG, the high 22 bits of the time-base counter
    /// of timer %s can be read here.
    ///
    /// Bits `[21:0]`, mask `0x003fffff`
    pub const HI_OFFSET: u32 = 0;
    pub const HI_MASK: u32 = 0x003fffff;

    // ======== Field Accessors ========

    /// **HI** `[21:0]` (RO)
    ///
    /// After writing to TIMG_T%sUPDATE_REG, the high 22 bits of the time-base counter
    /// of timer %s can be read here.
    #[inline(always)]
    pub fn hi(&self) -> u32 {
        (self.value.read_bits() & Self::HI_MASK) >> Self::HI_OFFSET
    }
}

/// T1HI value type for batch read/modify/write operations
///
/// Use `t1hi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t1hi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T1hiVal {
    bits: u32,
}

impl T1hiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HI** `[21:0]`
    ///
    /// After writing to TIMG_T%sUPDATE_REG, the high 22 bits of the time-base counter
    /// of timer %s can be read here.
    #[inline(always)]
    pub const fn hi(&self) -> u32 {
        (self.bits & T1hi::HI_MASK) >> T1hi::HI_OFFSET
    }
}

impl Default for T1hiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T1hi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T1hiVal {
        T1hiVal::from_bits(self.value.read_bits())
    }
}

/// T1UPDATE Register
///
/// Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **UPDATE** `[31]` (RW)
///
///   After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched.
#[repr(transparent)]
pub struct T1update {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T1update {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T1update {
    // ======== Field Constants ========

    /// **UPDATE**
    ///
    /// After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const UPDATE_OFFSET: u32 = 31;
    pub const UPDATE_MASK: u32 = 0x80000000;
    pub const UPDATE: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **UPDATE** `[31]` (RW)
    ///
    /// After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched.
    #[inline(always)]
    pub fn update(&self) -> bool {
        (self.value.read_bits() & Self::UPDATE_MASK) != 0
    }

    /// Set UPDATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_update(&self) {
        self.value.modify_bits(|r| r | Self::UPDATE_MASK);
    }

    /// Clear UPDATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_update(&self) {
        self.value.modify_bits(|r| r & !Self::UPDATE_MASK);
    }
}

/// T1UPDATE value type for batch read/modify/write operations
///
/// Use `t1update.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t1update.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T1updateVal {
    bits: u32,
}

impl T1updateVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **UPDATE** `[31:31]`
    ///
    /// After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched.
    #[inline(always)]
    pub const fn update(&self) -> bool {
        (self.bits & T1update::UPDATE_MASK) != 0
    }

    /// Set **UPDATE** `[31:31]`
    ///
    /// After writing 0 or 1 to TIMG_T%sUPDATE_REG, the counter value is latched.
    #[inline(always)]
    pub const fn with_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | T1update::UPDATE_MASK
            } else {
                self.bits & !T1update::UPDATE_MASK
            },
        }
    }
}

impl Default for T1updateVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T1update {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T1updateVal {
        T1updateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: T1updateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(T1updateVal) -> T1updateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T1ALARMLO Register
///
/// Timer %s alarm value, low 32 bits
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **ALARM_LO** `[31:0]` (RW)
///
///   Timer %s alarm trigger time-base counter value, low 32 bits.
#[repr(transparent)]
pub struct T1alarmlo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T1alarmlo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T1alarmlo {
    // ======== Field Constants ========

    /// **ALARM_LO**
    ///
    /// Timer %s alarm trigger time-base counter value, low 32 bits.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ALARM_LO_OFFSET: u32 = 0;
    pub const ALARM_LO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ALARM_LO** `[31:0]` (RW)
    ///
    /// Timer %s alarm trigger time-base counter value, low 32 bits.
    #[inline(always)]
    pub fn alarm_lo(&self) -> u32 {
        (self.value.read_bits() & Self::ALARM_LO_MASK) >> Self::ALARM_LO_OFFSET
    }

    /// Write **ALARM_LO** `[31:0]` (RW)
    ///
    /// Timer %s alarm trigger time-base counter value, low 32 bits.
    #[inline(always)]
    pub fn write_alarm_lo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ALARM_LO_MASK) | ((value << Self::ALARM_LO_OFFSET) & Self::ALARM_LO_MASK)
        });
    }
}

/// T1ALARMLO value type for batch read/modify/write operations
///
/// Use `t1alarmlo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t1alarmlo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T1alarmloVal {
    bits: u32,
}

impl T1alarmloVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ALARM_LO** `[31:0]`
    ///
    /// Timer %s alarm trigger time-base counter value, low 32 bits.
    #[inline(always)]
    pub const fn alarm_lo(&self) -> u32 {
        (self.bits & T1alarmlo::ALARM_LO_MASK) >> T1alarmlo::ALARM_LO_OFFSET
    }

    /// Set **ALARM_LO** `[31:0]`
    ///
    /// Timer %s alarm trigger time-base counter value, low 32 bits.
    #[inline(always)]
    pub const fn with_alarm_lo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !T1alarmlo::ALARM_LO_MASK)
                | ((value << T1alarmlo::ALARM_LO_OFFSET) & T1alarmlo::ALARM_LO_MASK),
        }
    }
}

impl Default for T1alarmloVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T1alarmlo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T1alarmloVal {
        T1alarmloVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: T1alarmloVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(T1alarmloVal) -> T1alarmloVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T1ALARMHI Register
///
/// Timer %s alarm value, high bits
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **ALARM_HI** `[21:0]` (RW)
///
///   Timer %s alarm trigger time-base counter value, high 22 bits.
#[repr(transparent)]
pub struct T1alarmhi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T1alarmhi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T1alarmhi {
    // ======== Field Constants ========

    /// **ALARM_HI**
    ///
    /// Timer %s alarm trigger time-base counter value, high 22 bits.
    ///
    /// Bits `[21:0]`, mask `0x003fffff`
    pub const ALARM_HI_OFFSET: u32 = 0;
    pub const ALARM_HI_MASK: u32 = 0x003fffff;

    // ======== Field Accessors ========

    /// **ALARM_HI** `[21:0]` (RW)
    ///
    /// Timer %s alarm trigger time-base counter value, high 22 bits.
    #[inline(always)]
    pub fn alarm_hi(&self) -> u32 {
        (self.value.read_bits() & Self::ALARM_HI_MASK) >> Self::ALARM_HI_OFFSET
    }

    /// Write **ALARM_HI** `[21:0]` (RW)
    ///
    /// Timer %s alarm trigger time-base counter value, high 22 bits.
    #[inline(always)]
    pub fn write_alarm_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ALARM_HI_MASK) | ((value << Self::ALARM_HI_OFFSET) & Self::ALARM_HI_MASK)
        });
    }
}

/// T1ALARMHI value type for batch read/modify/write operations
///
/// Use `t1alarmhi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t1alarmhi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T1alarmhiVal {
    bits: u32,
}

impl T1alarmhiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ALARM_HI** `[21:0]`
    ///
    /// Timer %s alarm trigger time-base counter value, high 22 bits.
    #[inline(always)]
    pub const fn alarm_hi(&self) -> u32 {
        (self.bits & T1alarmhi::ALARM_HI_MASK) >> T1alarmhi::ALARM_HI_OFFSET
    }

    /// Set **ALARM_HI** `[21:0]`
    ///
    /// Timer %s alarm trigger time-base counter value, high 22 bits.
    #[inline(always)]
    pub const fn with_alarm_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !T1alarmhi::ALARM_HI_MASK)
                | ((value << T1alarmhi::ALARM_HI_OFFSET) & T1alarmhi::ALARM_HI_MASK),
        }
    }
}

impl Default for T1alarmhiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T1alarmhi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T1alarmhiVal {
        T1alarmhiVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: T1alarmhiVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(T1alarmhiVal) -> T1alarmhiVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T1LOADLO Register
///
/// Timer %s reload value, low 32 bits
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **LOAD_LO** `[31:0]` (RW)
///
///   Low 32 bits of the value that a reload will load onto timer %s time-base
///   Counter.
#[repr(transparent)]
pub struct T1loadlo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T1loadlo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T1loadlo {
    // ======== Field Constants ========

    /// **LOAD_LO**
    ///
    /// Low 32 bits of the value that a reload will load onto timer %s time-base
    /// Counter.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LOAD_LO_OFFSET: u32 = 0;
    pub const LOAD_LO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LOAD_LO** `[31:0]` (RW)
    ///
    /// Low 32 bits of the value that a reload will load onto timer %s time-base
    /// Counter.
    #[inline(always)]
    pub fn load_lo(&self) -> u32 {
        (self.value.read_bits() & Self::LOAD_LO_MASK) >> Self::LOAD_LO_OFFSET
    }

    /// Write **LOAD_LO** `[31:0]` (RW)
    ///
    /// Low 32 bits of the value that a reload will load onto timer %s time-base
    /// Counter.
    #[inline(always)]
    pub fn write_load_lo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LOAD_LO_MASK) | ((value << Self::LOAD_LO_OFFSET) & Self::LOAD_LO_MASK)
        });
    }
}

/// T1LOADLO value type for batch read/modify/write operations
///
/// Use `t1loadlo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t1loadlo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T1loadloVal {
    bits: u32,
}

impl T1loadloVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LOAD_LO** `[31:0]`
    ///
    /// Low 32 bits of the value that a reload will load onto timer %s time-base
    /// Counter.
    #[inline(always)]
    pub const fn load_lo(&self) -> u32 {
        (self.bits & T1loadlo::LOAD_LO_MASK) >> T1loadlo::LOAD_LO_OFFSET
    }

    /// Set **LOAD_LO** `[31:0]`
    ///
    /// Low 32 bits of the value that a reload will load onto timer %s time-base
    /// Counter.
    #[inline(always)]
    pub const fn with_load_lo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !T1loadlo::LOAD_LO_MASK)
                | ((value << T1loadlo::LOAD_LO_OFFSET) & T1loadlo::LOAD_LO_MASK),
        }
    }
}

impl Default for T1loadloVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T1loadlo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T1loadloVal {
        T1loadloVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: T1loadloVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(T1loadloVal) -> T1loadloVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T1LOADHI Register
///
/// Timer %s reload value, high 22 bits
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **LOAD_HI** `[21:0]` (RW)
///
///   High 22 bits of the value that a reload will load onto timer %s time-base
///   counter.
#[repr(transparent)]
pub struct T1loadhi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T1loadhi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T1loadhi {
    // ======== Field Constants ========

    /// **LOAD_HI**
    ///
    /// High 22 bits of the value that a reload will load onto timer %s time-base
    /// counter.
    ///
    /// Bits `[21:0]`, mask `0x003fffff`
    pub const LOAD_HI_OFFSET: u32 = 0;
    pub const LOAD_HI_MASK: u32 = 0x003fffff;

    // ======== Field Accessors ========

    /// **LOAD_HI** `[21:0]` (RW)
    ///
    /// High 22 bits of the value that a reload will load onto timer %s time-base
    /// counter.
    #[inline(always)]
    pub fn load_hi(&self) -> u32 {
        (self.value.read_bits() & Self::LOAD_HI_MASK) >> Self::LOAD_HI_OFFSET
    }

    /// Write **LOAD_HI** `[21:0]` (RW)
    ///
    /// High 22 bits of the value that a reload will load onto timer %s time-base
    /// counter.
    #[inline(always)]
    pub fn write_load_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LOAD_HI_MASK) | ((value << Self::LOAD_HI_OFFSET) & Self::LOAD_HI_MASK)
        });
    }
}

/// T1LOADHI value type for batch read/modify/write operations
///
/// Use `t1loadhi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t1loadhi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T1loadhiVal {
    bits: u32,
}

impl T1loadhiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LOAD_HI** `[21:0]`
    ///
    /// High 22 bits of the value that a reload will load onto timer %s time-base
    /// counter.
    #[inline(always)]
    pub const fn load_hi(&self) -> u32 {
        (self.bits & T1loadhi::LOAD_HI_MASK) >> T1loadhi::LOAD_HI_OFFSET
    }

    /// Set **LOAD_HI** `[21:0]`
    ///
    /// High 22 bits of the value that a reload will load onto timer %s time-base
    /// counter.
    #[inline(always)]
    pub const fn with_load_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !T1loadhi::LOAD_HI_MASK)
                | ((value << T1loadhi::LOAD_HI_OFFSET) & T1loadhi::LOAD_HI_MASK),
        }
    }
}

impl Default for T1loadhiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T1loadhi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> T1loadhiVal {
        T1loadhiVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: T1loadhiVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(T1loadhiVal) -> T1loadhiVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// T1LOAD Register
///
/// Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **LOAD** `[31:0]` (WO)
///
///   Write any value to trigger a timer %s time-base counter reload.
#[repr(transparent)]
pub struct T1load {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T1load {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T1load {
    // ======== Field Constants ========

    /// **LOAD**
    ///
    /// Write any value to trigger a timer %s time-base counter reload.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LOAD_OFFSET: u32 = 0;
    pub const LOAD_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// Write **LOAD** `[31:0]` (WO)
    ///
    /// Write any value to trigger a timer %s time-base counter reload.
    #[inline(always)]
    pub fn write_load(&self, value: u32) {
        self.value
            .write_bits((value << Self::LOAD_OFFSET) & Self::LOAD_MASK);
    }
}

/// T1LOAD value type for batch read/modify/write operations
///
/// Use `t1load.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `t1load.write_val(val)`.
#[derive(Clone, Copy)]
pub struct T1loadVal {
    bits: u32,
}

impl T1loadVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **LOAD** `[31:0]`
    ///
    /// Write any value to trigger a timer %s time-base counter reload.
    #[inline(always)]
    pub const fn with_load(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !T1load::LOAD_MASK)
                | ((value << T1load::LOAD_OFFSET) & T1load::LOAD_MASK),
        }
    }
}

impl Default for T1loadVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl T1load {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: T1loadVal) {
        self.value.write_bits(val.bits())
    }
}

/// WDTCONFIG0 Register
///
/// Watchdog timer configuration register
///
/// Address offset: 0x0048
/// Reset value: 0x0004c000
///
/// ## Fields
///
/// - **WDT_EN** `[31]` (RW)
///
///   When set, MWDT is enabled.
///
/// - **WDT_STG0** `[30:29]` (RW)
///
///   Stage 0 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
///
/// - **WDT_STG1** `[28:27]` (RW)
///
///   Stage 1 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
///
/// - **WDT_STG2** `[26:25]` (RW)
///
///   Stage 2 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
///
/// - **WDT_STG3** `[24:23]` (RW)
///
///   Stage 3 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
///
/// - **WDT_CONF_UPDATE_EN** `[22]` (WO)
///
///   update the WDT configuration registers
///
/// - **WDT_USE_XTAL** `[21]` (RW)
///
///   choose WDT clock:0-apb_clk, 1-xtal_clk.
///
/// - **WDT_CPU_RESET_LENGTH** `[20:18]` (RW)
///
///   CPU reset signal length selection. 0: 100 ns, 1: 200 ns,
///   2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us.
///
/// - **WDT_SYS_RESET_LENGTH** `[17:15]` (RW)
///
///   System reset signal length selection. 0: 100 ns, 1: 200 ns,
///   2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us.
///
/// - **WDT_FLASHBOOT_MOD_EN** `[14]` (RW)
///
///   When set, Flash boot protection is enabled.
///
/// - **WDT_PROCPU_RESET_EN** `[13]` (RW)
///
///   WDT reset CPU enable.
///
/// - **WDT_APPCPU_RESET_EN** `[12]` (RW)
///
///   WDT reset CPU enable.
#[repr(transparent)]
pub struct Wdtconfig0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wdtconfig0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdtconfig0 {
    // ======== Field Constants ========

    /// **WDT_EN**
    ///
    /// When set, MWDT is enabled.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const WDT_EN_OFFSET: u32 = 31;
    pub const WDT_EN_MASK: u32 = 0x80000000;
    pub const WDT_EN: u32 = 1 << 31;

    /// **WDT_STG0**
    ///
    /// Stage 0 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    ///
    /// Bits `[30:29]`, mask `0x60000000`
    pub const WDT_STG0_OFFSET: u32 = 29;
    pub const WDT_STG0_MASK: u32 = 0x60000000;

    /// **WDT_STG1**
    ///
    /// Stage 1 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    ///
    /// Bits `[28:27]`, mask `0x18000000`
    pub const WDT_STG1_OFFSET: u32 = 27;
    pub const WDT_STG1_MASK: u32 = 0x18000000;

    /// **WDT_STG2**
    ///
    /// Stage 2 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    ///
    /// Bits `[26:25]`, mask `0x06000000`
    pub const WDT_STG2_OFFSET: u32 = 25;
    pub const WDT_STG2_MASK: u32 = 0x06000000;

    /// **WDT_STG3**
    ///
    /// Stage 3 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    ///
    /// Bits `[24:23]`, mask `0x01800000`
    pub const WDT_STG3_OFFSET: u32 = 23;
    pub const WDT_STG3_MASK: u32 = 0x01800000;

    /// **WDT_CONF_UPDATE_EN**
    ///
    /// update the WDT configuration registers
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const WDT_CONF_UPDATE_EN_OFFSET: u32 = 22;
    pub const WDT_CONF_UPDATE_EN_MASK: u32 = 0x00400000;
    pub const WDT_CONF_UPDATE_EN: u32 = 1 << 22;

    /// **WDT_USE_XTAL**
    ///
    /// choose WDT clock:0-apb_clk, 1-xtal_clk.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const WDT_USE_XTAL_OFFSET: u32 = 21;
    pub const WDT_USE_XTAL_MASK: u32 = 0x00200000;
    pub const WDT_USE_XTAL: u32 = 1 << 21;

    /// **WDT_CPU_RESET_LENGTH**
    ///
    /// CPU reset signal length selection. 0: 100 ns, 1: 200 ns,
    /// 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us.
    ///
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const WDT_CPU_RESET_LENGTH_OFFSET: u32 = 18;
    pub const WDT_CPU_RESET_LENGTH_MASK: u32 = 0x001c0000;

    /// **WDT_SYS_RESET_LENGTH**
    ///
    /// System reset signal length selection. 0: 100 ns, 1: 200 ns,
    /// 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us.
    ///
    /// Bits `[17:15]`, mask `0x00038000`
    pub const WDT_SYS_RESET_LENGTH_OFFSET: u32 = 15;
    pub const WDT_SYS_RESET_LENGTH_MASK: u32 = 0x00038000;

    /// **WDT_FLASHBOOT_MOD_EN**
    ///
    /// When set, Flash boot protection is enabled.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const WDT_FLASHBOOT_MOD_EN_OFFSET: u32 = 14;
    pub const WDT_FLASHBOOT_MOD_EN_MASK: u32 = 0x00004000;
    pub const WDT_FLASHBOOT_MOD_EN: u32 = 1 << 14;

    /// **WDT_PROCPU_RESET_EN**
    ///
    /// WDT reset CPU enable.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const WDT_PROCPU_RESET_EN_OFFSET: u32 = 13;
    pub const WDT_PROCPU_RESET_EN_MASK: u32 = 0x00002000;
    pub const WDT_PROCPU_RESET_EN: u32 = 1 << 13;

    /// **WDT_APPCPU_RESET_EN**
    ///
    /// WDT reset CPU enable.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const WDT_APPCPU_RESET_EN_OFFSET: u32 = 12;
    pub const WDT_APPCPU_RESET_EN_MASK: u32 = 0x00001000;
    pub const WDT_APPCPU_RESET_EN: u32 = 1 << 12;

    // ======== Field Accessors ========

    /// **WDT_EN** `[31]` (RW)
    ///
    /// When set, MWDT is enabled.
    #[inline(always)]
    pub fn wdt_en(&self) -> bool {
        (self.value.read_bits() & Self::WDT_EN_MASK) != 0
    }

    /// Set WDT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdt_en(&self) {
        self.value.modify_bits(|r| r | Self::WDT_EN_MASK);
    }

    /// Clear WDT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdt_en(&self) {
        self.value.modify_bits(|r| r & !Self::WDT_EN_MASK);
    }

    /// **WDT_STG0** `[30:29]` (RW)
    ///
    /// Stage 0 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub fn wdt_stg0(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG0_MASK) >> Self::WDT_STG0_OFFSET
    }

    /// Write **WDT_STG0** `[30:29]` (RW)
    ///
    /// Stage 0 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub fn write_wdt_stg0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG0_MASK) | ((value << Self::WDT_STG0_OFFSET) & Self::WDT_STG0_MASK)
        });
    }

    /// **WDT_STG1** `[28:27]` (RW)
    ///
    /// Stage 1 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub fn wdt_stg1(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG1_MASK) >> Self::WDT_STG1_OFFSET
    }

    /// Write **WDT_STG1** `[28:27]` (RW)
    ///
    /// Stage 1 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub fn write_wdt_stg1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG1_MASK) | ((value << Self::WDT_STG1_OFFSET) & Self::WDT_STG1_MASK)
        });
    }

    /// **WDT_STG2** `[26:25]` (RW)
    ///
    /// Stage 2 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub fn wdt_stg2(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG2_MASK) >> Self::WDT_STG2_OFFSET
    }

    /// Write **WDT_STG2** `[26:25]` (RW)
    ///
    /// Stage 2 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub fn write_wdt_stg2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG2_MASK) | ((value << Self::WDT_STG2_OFFSET) & Self::WDT_STG2_MASK)
        });
    }

    /// **WDT_STG3** `[24:23]` (RW)
    ///
    /// Stage 3 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub fn wdt_stg3(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG3_MASK) >> Self::WDT_STG3_OFFSET
    }

    /// Write **WDT_STG3** `[24:23]` (RW)
    ///
    /// Stage 3 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub fn write_wdt_stg3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG3_MASK) | ((value << Self::WDT_STG3_OFFSET) & Self::WDT_STG3_MASK)
        });
    }

    /// Trigger **WDT_CONF_UPDATE_EN** (WO)
    ///
    /// update the WDT configuration registers
    #[inline(always)]
    pub fn trigger_wdt_conf_update_en(&self) {
        self.value
            .modify_bits(|r| r | Self::WDT_CONF_UPDATE_EN_MASK);
    }

    /// **WDT_USE_XTAL** `[21]` (RW)
    ///
    /// choose WDT clock:0-apb_clk, 1-xtal_clk.
    #[inline(always)]
    pub fn wdt_use_xtal(&self) -> bool {
        (self.value.read_bits() & Self::WDT_USE_XTAL_MASK) != 0
    }

    /// Set WDT_USE_XTAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdt_use_xtal(&self) {
        self.value.modify_bits(|r| r | Self::WDT_USE_XTAL_MASK);
    }

    /// Clear WDT_USE_XTAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdt_use_xtal(&self) {
        self.value.modify_bits(|r| r & !Self::WDT_USE_XTAL_MASK);
    }

    /// **WDT_CPU_RESET_LENGTH** `[20:18]` (RW)
    ///
    /// CPU reset signal length selection. 0: 100 ns, 1: 200 ns,
    /// 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us.
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_CPU_RESET_LENGTH_MASK)
            >> Self::WDT_CPU_RESET_LENGTH_OFFSET
    }

    /// Write **WDT_CPU_RESET_LENGTH** `[20:18]` (RW)
    ///
    /// CPU reset signal length selection. 0: 100 ns, 1: 200 ns,
    /// 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us.
    #[inline(always)]
    pub fn write_wdt_cpu_reset_length(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_CPU_RESET_LENGTH_MASK)
                | ((value << Self::WDT_CPU_RESET_LENGTH_OFFSET) & Self::WDT_CPU_RESET_LENGTH_MASK)
        });
    }

    /// **WDT_SYS_RESET_LENGTH** `[17:15]` (RW)
    ///
    /// System reset signal length selection. 0: 100 ns, 1: 200 ns,
    /// 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us.
    #[inline(always)]
    pub fn wdt_sys_reset_length(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_SYS_RESET_LENGTH_MASK)
            >> Self::WDT_SYS_RESET_LENGTH_OFFSET
    }

    /// Write **WDT_SYS_RESET_LENGTH** `[17:15]` (RW)
    ///
    /// System reset signal length selection. 0: 100 ns, 1: 200 ns,
    /// 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us.
    #[inline(always)]
    pub fn write_wdt_sys_reset_length(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_SYS_RESET_LENGTH_MASK)
                | ((value << Self::WDT_SYS_RESET_LENGTH_OFFSET) & Self::WDT_SYS_RESET_LENGTH_MASK)
        });
    }

    /// **WDT_FLASHBOOT_MOD_EN** `[14]` (RW)
    ///
    /// When set, Flash boot protection is enabled.
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&self) -> bool {
        (self.value.read_bits() & Self::WDT_FLASHBOOT_MOD_EN_MASK) != 0
    }

    /// Set WDT_FLASHBOOT_MOD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdt_flashboot_mod_en(&self) {
        self.value
            .modify_bits(|r| r | Self::WDT_FLASHBOOT_MOD_EN_MASK);
    }

    /// Clear WDT_FLASHBOOT_MOD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdt_flashboot_mod_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::WDT_FLASHBOOT_MOD_EN_MASK);
    }

    /// **WDT_PROCPU_RESET_EN** `[13]` (RW)
    ///
    /// WDT reset CPU enable.
    #[inline(always)]
    pub fn wdt_procpu_reset_en(&self) -> bool {
        (self.value.read_bits() & Self::WDT_PROCPU_RESET_EN_MASK) != 0
    }

    /// Set WDT_PROCPU_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdt_procpu_reset_en(&self) {
        self.value
            .modify_bits(|r| r | Self::WDT_PROCPU_RESET_EN_MASK);
    }

    /// Clear WDT_PROCPU_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdt_procpu_reset_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::WDT_PROCPU_RESET_EN_MASK);
    }

    /// **WDT_APPCPU_RESET_EN** `[12]` (RW)
    ///
    /// WDT reset CPU enable.
    #[inline(always)]
    pub fn wdt_appcpu_reset_en(&self) -> bool {
        (self.value.read_bits() & Self::WDT_APPCPU_RESET_EN_MASK) != 0
    }

    /// Set WDT_APPCPU_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdt_appcpu_reset_en(&self) {
        self.value
            .modify_bits(|r| r | Self::WDT_APPCPU_RESET_EN_MASK);
    }

    /// Clear WDT_APPCPU_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdt_appcpu_reset_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::WDT_APPCPU_RESET_EN_MASK);
    }
}

/// WDTCONFIG0 value type for batch read/modify/write operations
///
/// Use `wdtconfig0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdtconfig0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Wdtconfig0Val {
    bits: u32,
}

impl Wdtconfig0Val {
    /// Create from raw bits
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
        Self { bits: 0x0004c000 }
    }

    /// Read **WDT_EN** `[31:31]`
    ///
    /// When set, MWDT is enabled.
    #[inline(always)]
    pub const fn wdt_en(&self) -> bool {
        (self.bits & Wdtconfig0::WDT_EN_MASK) != 0
    }

    /// Read **WDT_STG0** `[30:29]`
    ///
    /// Stage 0 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub const fn wdt_stg0(&self) -> u32 {
        (self.bits & Wdtconfig0::WDT_STG0_MASK) >> Wdtconfig0::WDT_STG0_OFFSET
    }

    /// Read **WDT_STG1** `[28:27]`
    ///
    /// Stage 1 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub const fn wdt_stg1(&self) -> u32 {
        (self.bits & Wdtconfig0::WDT_STG1_MASK) >> Wdtconfig0::WDT_STG1_OFFSET
    }

    /// Read **WDT_STG2** `[26:25]`
    ///
    /// Stage 2 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub const fn wdt_stg2(&self) -> u32 {
        (self.bits & Wdtconfig0::WDT_STG2_MASK) >> Wdtconfig0::WDT_STG2_OFFSET
    }

    /// Read **WDT_STG3** `[24:23]`
    ///
    /// Stage 3 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub const fn wdt_stg3(&self) -> u32 {
        (self.bits & Wdtconfig0::WDT_STG3_MASK) >> Wdtconfig0::WDT_STG3_OFFSET
    }

    /// Read **WDT_USE_XTAL** `[21:21]`
    ///
    /// choose WDT clock:0-apb_clk, 1-xtal_clk.
    #[inline(always)]
    pub const fn wdt_use_xtal(&self) -> bool {
        (self.bits & Wdtconfig0::WDT_USE_XTAL_MASK) != 0
    }

    /// Read **WDT_CPU_RESET_LENGTH** `[20:18]`
    ///
    /// CPU reset signal length selection. 0: 100 ns, 1: 200 ns,
    /// 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us.
    #[inline(always)]
    pub const fn wdt_cpu_reset_length(&self) -> u32 {
        (self.bits & Wdtconfig0::WDT_CPU_RESET_LENGTH_MASK)
            >> Wdtconfig0::WDT_CPU_RESET_LENGTH_OFFSET
    }

    /// Read **WDT_SYS_RESET_LENGTH** `[17:15]`
    ///
    /// System reset signal length selection. 0: 100 ns, 1: 200 ns,
    /// 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us.
    #[inline(always)]
    pub const fn wdt_sys_reset_length(&self) -> u32 {
        (self.bits & Wdtconfig0::WDT_SYS_RESET_LENGTH_MASK)
            >> Wdtconfig0::WDT_SYS_RESET_LENGTH_OFFSET
    }

    /// Read **WDT_FLASHBOOT_MOD_EN** `[14:14]`
    ///
    /// When set, Flash boot protection is enabled.
    #[inline(always)]
    pub const fn wdt_flashboot_mod_en(&self) -> bool {
        (self.bits & Wdtconfig0::WDT_FLASHBOOT_MOD_EN_MASK) != 0
    }

    /// Read **WDT_PROCPU_RESET_EN** `[13:13]`
    ///
    /// WDT reset CPU enable.
    #[inline(always)]
    pub const fn wdt_procpu_reset_en(&self) -> bool {
        (self.bits & Wdtconfig0::WDT_PROCPU_RESET_EN_MASK) != 0
    }

    /// Read **WDT_APPCPU_RESET_EN** `[12:12]`
    ///
    /// WDT reset CPU enable.
    #[inline(always)]
    pub const fn wdt_appcpu_reset_en(&self) -> bool {
        (self.bits & Wdtconfig0::WDT_APPCPU_RESET_EN_MASK) != 0
    }

    /// Set **WDT_EN** `[31:31]`
    ///
    /// When set, MWDT is enabled.
    #[inline(always)]
    pub const fn with_wdt_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdtconfig0::WDT_EN_MASK
            } else {
                self.bits & !Wdtconfig0::WDT_EN_MASK
            },
        }
    }

    /// Set **WDT_STG0** `[30:29]`
    ///
    /// Stage 0 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub const fn with_wdt_stg0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdtconfig0::WDT_STG0_MASK)
                | ((value << Wdtconfig0::WDT_STG0_OFFSET) & Wdtconfig0::WDT_STG0_MASK),
        }
    }

    /// Set **WDT_STG1** `[28:27]`
    ///
    /// Stage 1 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub const fn with_wdt_stg1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdtconfig0::WDT_STG1_MASK)
                | ((value << Wdtconfig0::WDT_STG1_OFFSET) & Wdtconfig0::WDT_STG1_MASK),
        }
    }

    /// Set **WDT_STG2** `[26:25]`
    ///
    /// Stage 2 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub const fn with_wdt_stg2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdtconfig0::WDT_STG2_MASK)
                | ((value << Wdtconfig0::WDT_STG2_OFFSET) & Wdtconfig0::WDT_STG2_MASK),
        }
    }

    /// Set **WDT_STG3** `[24:23]`
    ///
    /// Stage 3 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system.
    #[inline(always)]
    pub const fn with_wdt_stg3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdtconfig0::WDT_STG3_MASK)
                | ((value << Wdtconfig0::WDT_STG3_OFFSET) & Wdtconfig0::WDT_STG3_MASK),
        }
    }

    /// Set **WDT_CONF_UPDATE_EN** `[22:22]`
    ///
    /// update the WDT configuration registers
    #[inline(always)]
    pub const fn with_wdt_conf_update_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdtconfig0::WDT_CONF_UPDATE_EN_MASK
            } else {
                self.bits & !Wdtconfig0::WDT_CONF_UPDATE_EN_MASK
            },
        }
    }

    /// Set **WDT_USE_XTAL** `[21:21]`
    ///
    /// choose WDT clock:0-apb_clk, 1-xtal_clk.
    #[inline(always)]
    pub const fn with_wdt_use_xtal(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdtconfig0::WDT_USE_XTAL_MASK
            } else {
                self.bits & !Wdtconfig0::WDT_USE_XTAL_MASK
            },
        }
    }

    /// Set **WDT_CPU_RESET_LENGTH** `[20:18]`
    ///
    /// CPU reset signal length selection. 0: 100 ns, 1: 200 ns,
    /// 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us.
    #[inline(always)]
    pub const fn with_wdt_cpu_reset_length(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdtconfig0::WDT_CPU_RESET_LENGTH_MASK)
                | ((value << Wdtconfig0::WDT_CPU_RESET_LENGTH_OFFSET)
                    & Wdtconfig0::WDT_CPU_RESET_LENGTH_MASK),
        }
    }

    /// Set **WDT_SYS_RESET_LENGTH** `[17:15]`
    ///
    /// System reset signal length selection. 0: 100 ns, 1: 200 ns,
    /// 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us.
    #[inline(always)]
    pub const fn with_wdt_sys_reset_length(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdtconfig0::WDT_SYS_RESET_LENGTH_MASK)
                | ((value << Wdtconfig0::WDT_SYS_RESET_LENGTH_OFFSET)
                    & Wdtconfig0::WDT_SYS_RESET_LENGTH_MASK),
        }
    }

    /// Set **WDT_FLASHBOOT_MOD_EN** `[14:14]`
    ///
    /// When set, Flash boot protection is enabled.
    #[inline(always)]
    pub const fn with_wdt_flashboot_mod_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdtconfig0::WDT_FLASHBOOT_MOD_EN_MASK
            } else {
                self.bits & !Wdtconfig0::WDT_FLASHBOOT_MOD_EN_MASK
            },
        }
    }

    /// Set **WDT_PROCPU_RESET_EN** `[13:13]`
    ///
    /// WDT reset CPU enable.
    #[inline(always)]
    pub const fn with_wdt_procpu_reset_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdtconfig0::WDT_PROCPU_RESET_EN_MASK
            } else {
                self.bits & !Wdtconfig0::WDT_PROCPU_RESET_EN_MASK
            },
        }
    }

    /// Set **WDT_APPCPU_RESET_EN** `[12:12]`
    ///
    /// WDT reset CPU enable.
    #[inline(always)]
    pub const fn with_wdt_appcpu_reset_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdtconfig0::WDT_APPCPU_RESET_EN_MASK
            } else {
                self.bits & !Wdtconfig0::WDT_APPCPU_RESET_EN_MASK
            },
        }
    }
}

impl Default for Wdtconfig0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wdtconfig0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Wdtconfig0Val {
        Wdtconfig0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Wdtconfig0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Wdtconfig0Val) -> Wdtconfig0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WDTCONFIG1 Register
///
/// Watchdog timer prescaler register
///
/// Address offset: 0x004c
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **WDT_CLK_PRESCALE** `[31:16]` (RW)
///
///   MWDT clock prescaler value. MWDT clock period = 12.5 ns *
///   TIMG_WDT_CLK_PRESCALE.
///
/// - **WDT_DIVCNT_RST** `[0]` (WO)
///
///   When set, WDT 's clock divider counter will be reset.
#[repr(transparent)]
pub struct Wdtconfig1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wdtconfig1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdtconfig1 {
    // ======== Field Constants ========

    /// **WDT_CLK_PRESCALE**
    ///
    /// MWDT clock prescaler value. MWDT clock period = 12.5 ns *
    /// TIMG_WDT_CLK_PRESCALE.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const WDT_CLK_PRESCALE_OFFSET: u32 = 16;
    pub const WDT_CLK_PRESCALE_MASK: u32 = 0xffff0000;

    /// **WDT_DIVCNT_RST**
    ///
    /// When set, WDT 's clock divider counter will be reset.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const WDT_DIVCNT_RST_OFFSET: u32 = 0;
    pub const WDT_DIVCNT_RST_MASK: u32 = 0x00000001;
    pub const WDT_DIVCNT_RST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **WDT_CLK_PRESCALE** `[31:16]` (RW)
    ///
    /// MWDT clock prescaler value. MWDT clock period = 12.5 ns *
    /// TIMG_WDT_CLK_PRESCALE.
    #[inline(always)]
    pub fn wdt_clk_prescale(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_CLK_PRESCALE_MASK) >> Self::WDT_CLK_PRESCALE_OFFSET
    }

    /// Write **WDT_CLK_PRESCALE** `[31:16]` (RW)
    ///
    /// MWDT clock prescaler value. MWDT clock period = 12.5 ns *
    /// TIMG_WDT_CLK_PRESCALE.
    #[inline(always)]
    pub fn write_wdt_clk_prescale(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_CLK_PRESCALE_MASK)
                | ((value << Self::WDT_CLK_PRESCALE_OFFSET) & Self::WDT_CLK_PRESCALE_MASK)
        });
    }

    /// Trigger **WDT_DIVCNT_RST** (WO)
    ///
    /// When set, WDT 's clock divider counter will be reset.
    #[inline(always)]
    pub fn trigger_wdt_divcnt_rst(&self) {
        self.value.modify_bits(|r| r | Self::WDT_DIVCNT_RST_MASK);
    }
}

/// WDTCONFIG1 value type for batch read/modify/write operations
///
/// Use `wdtconfig1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdtconfig1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Wdtconfig1Val {
    bits: u32,
}

impl Wdtconfig1Val {
    /// Create from raw bits
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
        Self { bits: 0x00010000 }
    }

    /// Read **WDT_CLK_PRESCALE** `[31:16]`
    ///
    /// MWDT clock prescaler value. MWDT clock period = 12.5 ns *
    /// TIMG_WDT_CLK_PRESCALE.
    #[inline(always)]
    pub const fn wdt_clk_prescale(&self) -> u32 {
        (self.bits & Wdtconfig1::WDT_CLK_PRESCALE_MASK) >> Wdtconfig1::WDT_CLK_PRESCALE_OFFSET
    }

    /// Set **WDT_CLK_PRESCALE** `[31:16]`
    ///
    /// MWDT clock prescaler value. MWDT clock period = 12.5 ns *
    /// TIMG_WDT_CLK_PRESCALE.
    #[inline(always)]
    pub const fn with_wdt_clk_prescale(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdtconfig1::WDT_CLK_PRESCALE_MASK)
                | ((value << Wdtconfig1::WDT_CLK_PRESCALE_OFFSET)
                    & Wdtconfig1::WDT_CLK_PRESCALE_MASK),
        }
    }

    /// Set **WDT_DIVCNT_RST** `[0:0]`
    ///
    /// When set, WDT 's clock divider counter will be reset.
    #[inline(always)]
    pub const fn with_wdt_divcnt_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Wdtconfig1::WDT_DIVCNT_RST_MASK
            } else {
                self.bits & !Wdtconfig1::WDT_DIVCNT_RST_MASK
            },
        }
    }
}

impl Default for Wdtconfig1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wdtconfig1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Wdtconfig1Val {
        Wdtconfig1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Wdtconfig1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Wdtconfig1Val) -> Wdtconfig1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WDTCONFIG2 Register
///
/// Watchdog timer stage 0 timeout value
///
/// Address offset: 0x0050
/// Reset value: 0x018cba80
///
/// ## Fields
///
/// - **WDT_STG0_HOLD** `[31:0]` (RW)
///
///   Stage 0 timeout value, in MWDT clock cycles.
#[repr(transparent)]
pub struct Wdtconfig2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wdtconfig2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdtconfig2 {
    // ======== Field Constants ========

    /// **WDT_STG0_HOLD**
    ///
    /// Stage 0 timeout value, in MWDT clock cycles.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WDT_STG0_HOLD_OFFSET: u32 = 0;
    pub const WDT_STG0_HOLD_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **WDT_STG0_HOLD** `[31:0]` (RW)
    ///
    /// Stage 0 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub fn wdt_stg0_hold(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG0_HOLD_MASK) >> Self::WDT_STG0_HOLD_OFFSET
    }

    /// Write **WDT_STG0_HOLD** `[31:0]` (RW)
    ///
    /// Stage 0 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub fn write_wdt_stg0_hold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG0_HOLD_MASK)
                | ((value << Self::WDT_STG0_HOLD_OFFSET) & Self::WDT_STG0_HOLD_MASK)
        });
    }
}

/// WDTCONFIG2 value type for batch read/modify/write operations
///
/// Use `wdtconfig2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdtconfig2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Wdtconfig2Val {
    bits: u32,
}

impl Wdtconfig2Val {
    /// Create from raw bits
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
        Self { bits: 0x018cba80 }
    }

    /// Read **WDT_STG0_HOLD** `[31:0]`
    ///
    /// Stage 0 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub const fn wdt_stg0_hold(&self) -> u32 {
        (self.bits & Wdtconfig2::WDT_STG0_HOLD_MASK) >> Wdtconfig2::WDT_STG0_HOLD_OFFSET
    }

    /// Set **WDT_STG0_HOLD** `[31:0]`
    ///
    /// Stage 0 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub const fn with_wdt_stg0_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdtconfig2::WDT_STG0_HOLD_MASK)
                | ((value << Wdtconfig2::WDT_STG0_HOLD_OFFSET) & Wdtconfig2::WDT_STG0_HOLD_MASK),
        }
    }
}

impl Default for Wdtconfig2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wdtconfig2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Wdtconfig2Val {
        Wdtconfig2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Wdtconfig2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Wdtconfig2Val) -> Wdtconfig2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WDTCONFIG3 Register
///
/// Watchdog timer stage 1 timeout value
///
/// Address offset: 0x0054
/// Reset value: 0x07ffffff
///
/// ## Fields
///
/// - **WDT_STG1_HOLD** `[31:0]` (RW)
///
///   Stage 1 timeout value, in MWDT clock cycles.
#[repr(transparent)]
pub struct Wdtconfig3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wdtconfig3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdtconfig3 {
    // ======== Field Constants ========

    /// **WDT_STG1_HOLD**
    ///
    /// Stage 1 timeout value, in MWDT clock cycles.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WDT_STG1_HOLD_OFFSET: u32 = 0;
    pub const WDT_STG1_HOLD_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **WDT_STG1_HOLD** `[31:0]` (RW)
    ///
    /// Stage 1 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub fn wdt_stg1_hold(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG1_HOLD_MASK) >> Self::WDT_STG1_HOLD_OFFSET
    }

    /// Write **WDT_STG1_HOLD** `[31:0]` (RW)
    ///
    /// Stage 1 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub fn write_wdt_stg1_hold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG1_HOLD_MASK)
                | ((value << Self::WDT_STG1_HOLD_OFFSET) & Self::WDT_STG1_HOLD_MASK)
        });
    }
}

/// WDTCONFIG3 value type for batch read/modify/write operations
///
/// Use `wdtconfig3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdtconfig3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Wdtconfig3Val {
    bits: u32,
}

impl Wdtconfig3Val {
    /// Create from raw bits
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
        Self { bits: 0x07ffffff }
    }

    /// Read **WDT_STG1_HOLD** `[31:0]`
    ///
    /// Stage 1 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub const fn wdt_stg1_hold(&self) -> u32 {
        (self.bits & Wdtconfig3::WDT_STG1_HOLD_MASK) >> Wdtconfig3::WDT_STG1_HOLD_OFFSET
    }

    /// Set **WDT_STG1_HOLD** `[31:0]`
    ///
    /// Stage 1 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub const fn with_wdt_stg1_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdtconfig3::WDT_STG1_HOLD_MASK)
                | ((value << Wdtconfig3::WDT_STG1_HOLD_OFFSET) & Wdtconfig3::WDT_STG1_HOLD_MASK),
        }
    }
}

impl Default for Wdtconfig3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wdtconfig3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Wdtconfig3Val {
        Wdtconfig3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Wdtconfig3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Wdtconfig3Val) -> Wdtconfig3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WDTCONFIG4 Register
///
/// Watchdog timer stage 2 timeout value
///
/// Address offset: 0x0058
/// Reset value: 0x000fffff
///
/// ## Fields
///
/// - **WDT_STG2_HOLD** `[31:0]` (RW)
///
///   Stage 2 timeout value, in MWDT clock cycles.
#[repr(transparent)]
pub struct Wdtconfig4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wdtconfig4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdtconfig4 {
    // ======== Field Constants ========

    /// **WDT_STG2_HOLD**
    ///
    /// Stage 2 timeout value, in MWDT clock cycles.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WDT_STG2_HOLD_OFFSET: u32 = 0;
    pub const WDT_STG2_HOLD_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **WDT_STG2_HOLD** `[31:0]` (RW)
    ///
    /// Stage 2 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub fn wdt_stg2_hold(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG2_HOLD_MASK) >> Self::WDT_STG2_HOLD_OFFSET
    }

    /// Write **WDT_STG2_HOLD** `[31:0]` (RW)
    ///
    /// Stage 2 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub fn write_wdt_stg2_hold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG2_HOLD_MASK)
                | ((value << Self::WDT_STG2_HOLD_OFFSET) & Self::WDT_STG2_HOLD_MASK)
        });
    }
}

/// WDTCONFIG4 value type for batch read/modify/write operations
///
/// Use `wdtconfig4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdtconfig4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Wdtconfig4Val {
    bits: u32,
}

impl Wdtconfig4Val {
    /// Create from raw bits
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
        Self { bits: 0x000fffff }
    }

    /// Read **WDT_STG2_HOLD** `[31:0]`
    ///
    /// Stage 2 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub const fn wdt_stg2_hold(&self) -> u32 {
        (self.bits & Wdtconfig4::WDT_STG2_HOLD_MASK) >> Wdtconfig4::WDT_STG2_HOLD_OFFSET
    }

    /// Set **WDT_STG2_HOLD** `[31:0]`
    ///
    /// Stage 2 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub const fn with_wdt_stg2_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdtconfig4::WDT_STG2_HOLD_MASK)
                | ((value << Wdtconfig4::WDT_STG2_HOLD_OFFSET) & Wdtconfig4::WDT_STG2_HOLD_MASK),
        }
    }
}

impl Default for Wdtconfig4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wdtconfig4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Wdtconfig4Val {
        Wdtconfig4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Wdtconfig4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Wdtconfig4Val) -> Wdtconfig4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WDTCONFIG5 Register
///
/// Watchdog timer stage 3 timeout value
///
/// Address offset: 0x005c
/// Reset value: 0x000fffff
///
/// ## Fields
///
/// - **WDT_STG3_HOLD** `[31:0]` (RW)
///
///   Stage 3 timeout value, in MWDT clock cycles.
#[repr(transparent)]
pub struct Wdtconfig5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wdtconfig5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdtconfig5 {
    // ======== Field Constants ========

    /// **WDT_STG3_HOLD**
    ///
    /// Stage 3 timeout value, in MWDT clock cycles.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WDT_STG3_HOLD_OFFSET: u32 = 0;
    pub const WDT_STG3_HOLD_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **WDT_STG3_HOLD** `[31:0]` (RW)
    ///
    /// Stage 3 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub fn wdt_stg3_hold(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_STG3_HOLD_MASK) >> Self::WDT_STG3_HOLD_OFFSET
    }

    /// Write **WDT_STG3_HOLD** `[31:0]` (RW)
    ///
    /// Stage 3 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub fn write_wdt_stg3_hold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_STG3_HOLD_MASK)
                | ((value << Self::WDT_STG3_HOLD_OFFSET) & Self::WDT_STG3_HOLD_MASK)
        });
    }
}

/// WDTCONFIG5 value type for batch read/modify/write operations
///
/// Use `wdtconfig5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdtconfig5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Wdtconfig5Val {
    bits: u32,
}

impl Wdtconfig5Val {
    /// Create from raw bits
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
        Self { bits: 0x000fffff }
    }

    /// Read **WDT_STG3_HOLD** `[31:0]`
    ///
    /// Stage 3 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub const fn wdt_stg3_hold(&self) -> u32 {
        (self.bits & Wdtconfig5::WDT_STG3_HOLD_MASK) >> Wdtconfig5::WDT_STG3_HOLD_OFFSET
    }

    /// Set **WDT_STG3_HOLD** `[31:0]`
    ///
    /// Stage 3 timeout value, in MWDT clock cycles.
    #[inline(always)]
    pub const fn with_wdt_stg3_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdtconfig5::WDT_STG3_HOLD_MASK)
                | ((value << Wdtconfig5::WDT_STG3_HOLD_OFFSET) & Wdtconfig5::WDT_STG3_HOLD_MASK),
        }
    }
}

impl Default for Wdtconfig5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wdtconfig5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Wdtconfig5Val {
        Wdtconfig5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Wdtconfig5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Wdtconfig5Val) -> Wdtconfig5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WDTFEED Register
///
/// Write to feed the watchdog timer
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **WDT_FEED** `[31:0]` (WO)
///
///   Write any value to feed the MWDT. (WO)
#[repr(transparent)]
pub struct Wdtfeed {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wdtfeed {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdtfeed {
    // ======== Field Constants ========

    /// **WDT_FEED**
    ///
    /// Write any value to feed the MWDT. (WO)
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WDT_FEED_OFFSET: u32 = 0;
    pub const WDT_FEED_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// Write **WDT_FEED** `[31:0]` (WO)
    ///
    /// Write any value to feed the MWDT. (WO)
    #[inline(always)]
    pub fn write_wdt_feed(&self, value: u32) {
        self.value
            .write_bits((value << Self::WDT_FEED_OFFSET) & Self::WDT_FEED_MASK);
    }
}

/// WDTFEED value type for batch read/modify/write operations
///
/// Use `wdtfeed.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdtfeed.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WdtfeedVal {
    bits: u32,
}

impl WdtfeedVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **WDT_FEED** `[31:0]`
    ///
    /// Write any value to feed the MWDT. (WO)
    #[inline(always)]
    pub const fn with_wdt_feed(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdtfeed::WDT_FEED_MASK)
                | ((value << Wdtfeed::WDT_FEED_OFFSET) & Wdtfeed::WDT_FEED_MASK),
        }
    }
}

impl Default for WdtfeedVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Wdtfeed {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WdtfeedVal) {
        self.value.write_bits(val.bits())
    }
}

/// WDTWPROTECT Register
///
/// Watchdog write protect register
///
/// Address offset: 0x0064
/// Reset value: 0x50d83aa1
///
/// ## Fields
///
/// - **WDT_WKEY** `[31:0]` (RW)
///
///   If the register contains a different value than its reset value, write
///   protection is enabled.
#[repr(transparent)]
pub struct Wdtwprotect {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Wdtwprotect {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Wdtwprotect {
    // ======== Field Constants ========

    /// **WDT_WKEY**
    ///
    /// If the register contains a different value than its reset value, write
    /// protection is enabled.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WDT_WKEY_OFFSET: u32 = 0;
    pub const WDT_WKEY_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **WDT_WKEY** `[31:0]` (RW)
    ///
    /// If the register contains a different value than its reset value, write
    /// protection is enabled.
    #[inline(always)]
    pub fn wdt_wkey(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_WKEY_MASK) >> Self::WDT_WKEY_OFFSET
    }

    /// Write **WDT_WKEY** `[31:0]` (RW)
    ///
    /// If the register contains a different value than its reset value, write
    /// protection is enabled.
    #[inline(always)]
    pub fn write_wdt_wkey(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDT_WKEY_MASK) | ((value << Self::WDT_WKEY_OFFSET) & Self::WDT_WKEY_MASK)
        });
    }
}

/// WDTWPROTECT value type for batch read/modify/write operations
///
/// Use `wdtwprotect.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wdtwprotect.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WdtwprotectVal {
    bits: u32,
}

impl WdtwprotectVal {
    /// Create from raw bits
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
        Self { bits: 0x50d83aa1 }
    }

    /// Read **WDT_WKEY** `[31:0]`
    ///
    /// If the register contains a different value than its reset value, write
    /// protection is enabled.
    #[inline(always)]
    pub const fn wdt_wkey(&self) -> u32 {
        (self.bits & Wdtwprotect::WDT_WKEY_MASK) >> Wdtwprotect::WDT_WKEY_OFFSET
    }

    /// Set **WDT_WKEY** `[31:0]`
    ///
    /// If the register contains a different value than its reset value, write
    /// protection is enabled.
    #[inline(always)]
    pub const fn with_wdt_wkey(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Wdtwprotect::WDT_WKEY_MASK)
                | ((value << Wdtwprotect::WDT_WKEY_OFFSET) & Wdtwprotect::WDT_WKEY_MASK),
        }
    }
}

impl Default for WdtwprotectVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Wdtwprotect {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WdtwprotectVal {
        WdtwprotectVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WdtwprotectVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WdtwprotectVal) -> WdtwprotectVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RTCCALICFG Register
///
/// RTC calibration configure register
///
/// Address offset: 0x0068
/// Reset value: 0x00011000
///
/// ## Fields
///
/// - **RTC_CALI_START** `[31]` (RW)
///
///   Set this bit to start one-shot frequency calculation.
///
/// - **RTC_CALI_MAX** `[30:16]` (RW)
///
///   Configure the time to calculate RTC slow clock's frequency.
///
/// - **RTC_CALI_RDY** `[15]` (RO)
///
///   indicate one-shot frequency calculation is done.
///
/// - **RTC_CALI_CLK_SEL** `[14:13]` (RW)
///
///   0:rtc slow clock. 1:clk_8m, 2:xtal_32k.
///
/// - **RTC_CALI_START_CYCLING** `[12]` (RW)
///
///   0: one-shot frequency calculation,1: periodic frequency calculation,
#[repr(transparent)]
pub struct Rtccalicfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Rtccalicfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rtccalicfg {
    // ======== Field Constants ========

    /// **RTC_CALI_START**
    ///
    /// Set this bit to start one-shot frequency calculation.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const RTC_CALI_START_OFFSET: u32 = 31;
    pub const RTC_CALI_START_MASK: u32 = 0x80000000;
    pub const RTC_CALI_START: u32 = 1 << 31;

    /// **RTC_CALI_MAX**
    ///
    /// Configure the time to calculate RTC slow clock's frequency.
    ///
    /// Bits `[30:16]`, mask `0x7fff0000`
    pub const RTC_CALI_MAX_OFFSET: u32 = 16;
    pub const RTC_CALI_MAX_MASK: u32 = 0x7fff0000;

    /// **RTC_CALI_RDY**
    ///
    /// indicate one-shot frequency calculation is done.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const RTC_CALI_RDY_OFFSET: u32 = 15;
    pub const RTC_CALI_RDY_MASK: u32 = 0x00008000;
    pub const RTC_CALI_RDY: u32 = 1 << 15;

    /// **RTC_CALI_CLK_SEL**
    ///
    /// 0:rtc slow clock. 1:clk_8m, 2:xtal_32k.
    ///
    /// Bits `[14:13]`, mask `0x00006000`
    pub const RTC_CALI_CLK_SEL_OFFSET: u32 = 13;
    pub const RTC_CALI_CLK_SEL_MASK: u32 = 0x00006000;

    /// **RTC_CALI_START_CYCLING**
    ///
    /// 0: one-shot frequency calculation,1: periodic frequency calculation,
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const RTC_CALI_START_CYCLING_OFFSET: u32 = 12;
    pub const RTC_CALI_START_CYCLING_MASK: u32 = 0x00001000;
    pub const RTC_CALI_START_CYCLING: u32 = 1 << 12;

    // ======== Field Accessors ========

    /// **RTC_CALI_START** `[31]` (RW)
    ///
    /// Set this bit to start one-shot frequency calculation.
    #[inline(always)]
    pub fn rtc_cali_start(&self) -> bool {
        (self.value.read_bits() & Self::RTC_CALI_START_MASK) != 0
    }

    /// Set RTC_CALI_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rtc_cali_start(&self) {
        self.value.modify_bits(|r| r | Self::RTC_CALI_START_MASK);
    }

    /// Clear RTC_CALI_START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rtc_cali_start(&self) {
        self.value.modify_bits(|r| r & !Self::RTC_CALI_START_MASK);
    }

    /// **RTC_CALI_MAX** `[30:16]` (RW)
    ///
    /// Configure the time to calculate RTC slow clock's frequency.
    #[inline(always)]
    pub fn rtc_cali_max(&self) -> u32 {
        (self.value.read_bits() & Self::RTC_CALI_MAX_MASK) >> Self::RTC_CALI_MAX_OFFSET
    }

    /// Write **RTC_CALI_MAX** `[30:16]` (RW)
    ///
    /// Configure the time to calculate RTC slow clock's frequency.
    #[inline(always)]
    pub fn write_rtc_cali_max(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RTC_CALI_MAX_MASK)
                | ((value << Self::RTC_CALI_MAX_OFFSET) & Self::RTC_CALI_MAX_MASK)
        });
    }

    /// **RTC_CALI_RDY** `[15]` (RO)
    ///
    /// indicate one-shot frequency calculation is done.
    #[inline(always)]
    pub fn rtc_cali_rdy(&self) -> bool {
        (self.value.read_bits() & Self::RTC_CALI_RDY_MASK) != 0
    }

    /// **RTC_CALI_CLK_SEL** `[14:13]` (RW)
    ///
    /// 0:rtc slow clock. 1:clk_8m, 2:xtal_32k.
    #[inline(always)]
    pub fn rtc_cali_clk_sel(&self) -> u32 {
        (self.value.read_bits() & Self::RTC_CALI_CLK_SEL_MASK) >> Self::RTC_CALI_CLK_SEL_OFFSET
    }

    /// Write **RTC_CALI_CLK_SEL** `[14:13]` (RW)
    ///
    /// 0:rtc slow clock. 1:clk_8m, 2:xtal_32k.
    #[inline(always)]
    pub fn write_rtc_cali_clk_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RTC_CALI_CLK_SEL_MASK)
                | ((value << Self::RTC_CALI_CLK_SEL_OFFSET) & Self::RTC_CALI_CLK_SEL_MASK)
        });
    }

    /// **RTC_CALI_START_CYCLING** `[12]` (RW)
    ///
    /// 0: one-shot frequency calculation,1: periodic frequency calculation,
    #[inline(always)]
    pub fn rtc_cali_start_cycling(&self) -> bool {
        (self.value.read_bits() & Self::RTC_CALI_START_CYCLING_MASK) != 0
    }

    /// Set RTC_CALI_START_CYCLING bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rtc_cali_start_cycling(&self) {
        self.value
            .modify_bits(|r| r | Self::RTC_CALI_START_CYCLING_MASK);
    }

    /// Clear RTC_CALI_START_CYCLING bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rtc_cali_start_cycling(&self) {
        self.value
            .modify_bits(|r| r & !Self::RTC_CALI_START_CYCLING_MASK);
    }
}

/// RTCCALICFG value type for batch read/modify/write operations
///
/// Use `rtccalicfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rtccalicfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RtccalicfgVal {
    bits: u32,
}

impl RtccalicfgVal {
    /// Create from raw bits
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
        Self { bits: 0x00011000 }
    }

    /// Read **RTC_CALI_START** `[31:31]`
    ///
    /// Set this bit to start one-shot frequency calculation.
    #[inline(always)]
    pub const fn rtc_cali_start(&self) -> bool {
        (self.bits & Rtccalicfg::RTC_CALI_START_MASK) != 0
    }

    /// Read **RTC_CALI_MAX** `[30:16]`
    ///
    /// Configure the time to calculate RTC slow clock's frequency.
    #[inline(always)]
    pub const fn rtc_cali_max(&self) -> u32 {
        (self.bits & Rtccalicfg::RTC_CALI_MAX_MASK) >> Rtccalicfg::RTC_CALI_MAX_OFFSET
    }

    /// Read **RTC_CALI_RDY** `[15:15]`
    ///
    /// indicate one-shot frequency calculation is done.
    #[inline(always)]
    pub const fn rtc_cali_rdy(&self) -> bool {
        (self.bits & Rtccalicfg::RTC_CALI_RDY_MASK) != 0
    }

    /// Read **RTC_CALI_CLK_SEL** `[14:13]`
    ///
    /// 0:rtc slow clock. 1:clk_8m, 2:xtal_32k.
    #[inline(always)]
    pub const fn rtc_cali_clk_sel(&self) -> u32 {
        (self.bits & Rtccalicfg::RTC_CALI_CLK_SEL_MASK) >> Rtccalicfg::RTC_CALI_CLK_SEL_OFFSET
    }

    /// Read **RTC_CALI_START_CYCLING** `[12:12]`
    ///
    /// 0: one-shot frequency calculation,1: periodic frequency calculation,
    #[inline(always)]
    pub const fn rtc_cali_start_cycling(&self) -> bool {
        (self.bits & Rtccalicfg::RTC_CALI_START_CYCLING_MASK) != 0
    }

    /// Set **RTC_CALI_START** `[31:31]`
    ///
    /// Set this bit to start one-shot frequency calculation.
    #[inline(always)]
    pub const fn with_rtc_cali_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtccalicfg::RTC_CALI_START_MASK
            } else {
                self.bits & !Rtccalicfg::RTC_CALI_START_MASK
            },
        }
    }

    /// Set **RTC_CALI_MAX** `[30:16]`
    ///
    /// Configure the time to calculate RTC slow clock's frequency.
    #[inline(always)]
    pub const fn with_rtc_cali_max(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rtccalicfg::RTC_CALI_MAX_MASK)
                | ((value << Rtccalicfg::RTC_CALI_MAX_OFFSET) & Rtccalicfg::RTC_CALI_MAX_MASK),
        }
    }

    /// Set **RTC_CALI_CLK_SEL** `[14:13]`
    ///
    /// 0:rtc slow clock. 1:clk_8m, 2:xtal_32k.
    #[inline(always)]
    pub const fn with_rtc_cali_clk_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rtccalicfg::RTC_CALI_CLK_SEL_MASK)
                | ((value << Rtccalicfg::RTC_CALI_CLK_SEL_OFFSET)
                    & Rtccalicfg::RTC_CALI_CLK_SEL_MASK),
        }
    }

    /// Set **RTC_CALI_START_CYCLING** `[12:12]`
    ///
    /// 0: one-shot frequency calculation,1: periodic frequency calculation,
    #[inline(always)]
    pub const fn with_rtc_cali_start_cycling(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Rtccalicfg::RTC_CALI_START_CYCLING_MASK
            } else {
                self.bits & !Rtccalicfg::RTC_CALI_START_CYCLING_MASK
            },
        }
    }
}

impl Default for RtccalicfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rtccalicfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RtccalicfgVal {
        RtccalicfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RtccalicfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RtccalicfgVal) -> RtccalicfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RTCCALICFG1 Register
///
/// RTC calibration configure1 register
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **RTC_CALI_VALUE** `[31:7]` (RO)
///
///   When one-shot or periodic frequency calculation is done, read this value to calculate RTC slow clock's frequency.
///
/// - **RTC_CALI_CYCLING_DATA_VLD** `[0]` (RO)
///
///   indicate periodic frequency calculation is done.
#[repr(transparent)]
pub struct Rtccalicfg1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Rtccalicfg1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rtccalicfg1 {
    // ======== Field Constants ========

    /// **RTC_CALI_VALUE**
    ///
    /// When one-shot or periodic frequency calculation is done, read this value to calculate RTC slow clock's frequency.
    ///
    /// Bits `[31:7]`, mask `0xffffff80`
    pub const RTC_CALI_VALUE_OFFSET: u32 = 7;
    pub const RTC_CALI_VALUE_MASK: u32 = 0xffffff80;

    /// **RTC_CALI_CYCLING_DATA_VLD**
    ///
    /// indicate periodic frequency calculation is done.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RTC_CALI_CYCLING_DATA_VLD_OFFSET: u32 = 0;
    pub const RTC_CALI_CYCLING_DATA_VLD_MASK: u32 = 0x00000001;
    pub const RTC_CALI_CYCLING_DATA_VLD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RTC_CALI_VALUE** `[31:7]` (RO)
    ///
    /// When one-shot or periodic frequency calculation is done, read this value to calculate RTC slow clock's frequency.
    #[inline(always)]
    pub fn rtc_cali_value(&self) -> u32 {
        (self.value.read_bits() & Self::RTC_CALI_VALUE_MASK) >> Self::RTC_CALI_VALUE_OFFSET
    }

    /// **RTC_CALI_CYCLING_DATA_VLD** `[0]` (RO)
    ///
    /// indicate periodic frequency calculation is done.
    #[inline(always)]
    pub fn rtc_cali_cycling_data_vld(&self) -> bool {
        (self.value.read_bits() & Self::RTC_CALI_CYCLING_DATA_VLD_MASK) != 0
    }
}

/// RTCCALICFG1 value type for batch read/modify/write operations
///
/// Use `rtccalicfg1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rtccalicfg1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rtccalicfg1Val {
    bits: u32,
}

impl Rtccalicfg1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RTC_CALI_VALUE** `[31:7]`
    ///
    /// When one-shot or periodic frequency calculation is done, read this value to calculate RTC slow clock's frequency.
    #[inline(always)]
    pub const fn rtc_cali_value(&self) -> u32 {
        (self.bits & Rtccalicfg1::RTC_CALI_VALUE_MASK) >> Rtccalicfg1::RTC_CALI_VALUE_OFFSET
    }

    /// Read **RTC_CALI_CYCLING_DATA_VLD** `[0:0]`
    ///
    /// indicate periodic frequency calculation is done.
    #[inline(always)]
    pub const fn rtc_cali_cycling_data_vld(&self) -> bool {
        (self.bits & Rtccalicfg1::RTC_CALI_CYCLING_DATA_VLD_MASK) != 0
    }
}

impl Default for Rtccalicfg1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Rtccalicfg1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rtccalicfg1Val {
        Rtccalicfg1Val::from_bits(self.value.read_bits())
    }
}

/// INT_ENA_TIMERS Register
///
/// Interrupt enable bits
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **WDT_INT_ENA** `[2]` (RW)
///
///   The interrupt enable bit for the TIMG_WDT_INT interrupt.
///
/// - **T1_INT_ENA** `[1]` (RW)
///
///   The interrupt enable bit for the TIMG_T1_INT interrupt.
///
/// - **T0_INT_ENA** `[0]` (RW)
///
///   The interrupt enable bit for the TIMG_T0_INT interrupt.
#[repr(transparent)]
pub struct IntEnaTimers {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntEnaTimers {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntEnaTimers {
    // ======== Field Constants ========

    /// **WDT_INT_ENA**
    ///
    /// The interrupt enable bit for the TIMG_WDT_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const WDT_INT_ENA_OFFSET: u32 = 2;
    pub const WDT_INT_ENA_MASK: u32 = 0x00000004;
    pub const WDT_INT_ENA: u32 = 1 << 2;

    /// **T1_INT_ENA**
    ///
    /// The interrupt enable bit for the TIMG_T1_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const T1_INT_ENA_OFFSET: u32 = 1;
    pub const T1_INT_ENA_MASK: u32 = 0x00000002;
    pub const T1_INT_ENA: u32 = 1 << 1;

    /// **T0_INT_ENA**
    ///
    /// The interrupt enable bit for the TIMG_T0_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const T0_INT_ENA_OFFSET: u32 = 0;
    pub const T0_INT_ENA_MASK: u32 = 0x00000001;
    pub const T0_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **WDT_INT_ENA** `[2]` (RW)
    ///
    /// The interrupt enable bit for the TIMG_WDT_INT interrupt.
    #[inline(always)]
    pub fn wdt_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::WDT_INT_ENA_MASK) != 0
    }

    /// Set WDT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdt_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::WDT_INT_ENA_MASK);
    }

    /// Clear WDT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdt_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::WDT_INT_ENA_MASK);
    }

    /// **T1_INT_ENA** `[1]` (RW)
    ///
    /// The interrupt enable bit for the TIMG_T1_INT interrupt.
    #[inline(always)]
    pub fn t1_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::T1_INT_ENA_MASK) != 0
    }

    /// Set T1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_t1_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::T1_INT_ENA_MASK);
    }

    /// Clear T1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_t1_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::T1_INT_ENA_MASK);
    }

    /// **T0_INT_ENA** `[0]` (RW)
    ///
    /// The interrupt enable bit for the TIMG_T0_INT interrupt.
    #[inline(always)]
    pub fn t0_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::T0_INT_ENA_MASK) != 0
    }

    /// Set T0_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_t0_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::T0_INT_ENA_MASK);
    }

    /// Clear T0_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_t0_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::T0_INT_ENA_MASK);
    }
}

/// INT_ENA_TIMERS value type for batch read/modify/write operations
///
/// Use `intenatimers.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intenatimers.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntEnaTimersVal {
    bits: u32,
}

impl IntEnaTimersVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **WDT_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the TIMG_WDT_INT interrupt.
    #[inline(always)]
    pub const fn wdt_int_ena(&self) -> bool {
        (self.bits & IntEnaTimers::WDT_INT_ENA_MASK) != 0
    }

    /// Read **T1_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the TIMG_T1_INT interrupt.
    #[inline(always)]
    pub const fn t1_int_ena(&self) -> bool {
        (self.bits & IntEnaTimers::T1_INT_ENA_MASK) != 0
    }

    /// Read **T0_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the TIMG_T0_INT interrupt.
    #[inline(always)]
    pub const fn t0_int_ena(&self) -> bool {
        (self.bits & IntEnaTimers::T0_INT_ENA_MASK) != 0
    }

    /// Set **WDT_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the TIMG_WDT_INT interrupt.
    #[inline(always)]
    pub const fn with_wdt_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaTimers::WDT_INT_ENA_MASK
            } else {
                self.bits & !IntEnaTimers::WDT_INT_ENA_MASK
            },
        }
    }

    /// Set **T1_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the TIMG_T1_INT interrupt.
    #[inline(always)]
    pub const fn with_t1_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaTimers::T1_INT_ENA_MASK
            } else {
                self.bits & !IntEnaTimers::T1_INT_ENA_MASK
            },
        }
    }

    /// Set **T0_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the TIMG_T0_INT interrupt.
    #[inline(always)]
    pub const fn with_t0_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaTimers::T0_INT_ENA_MASK
            } else {
                self.bits & !IntEnaTimers::T0_INT_ENA_MASK
            },
        }
    }
}

impl Default for IntEnaTimersVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntEnaTimers {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntEnaTimersVal {
        IntEnaTimersVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntEnaTimersVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntEnaTimersVal) -> IntEnaTimersVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_RAW_TIMERS Register
///
/// Raw interrupt status
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **WDT_INT_RAW** `[2]` (RO)
///
///   The raw interrupt status bit for the TIMG_WDT_INT interrupt.
///
/// - **T1_INT_RAW** `[1]` (RO)
///
///   The raw interrupt status bit for the TIMG_T1_INT interrupt.
///
/// - **T0_INT_RAW** `[0]` (RO)
///
///   The raw interrupt status bit for the TIMG_T0_INT interrupt.
#[repr(transparent)]
pub struct IntRawTimers {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntRawTimers {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntRawTimers {
    // ======== Field Constants ========

    /// **WDT_INT_RAW**
    ///
    /// The raw interrupt status bit for the TIMG_WDT_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const WDT_INT_RAW_OFFSET: u32 = 2;
    pub const WDT_INT_RAW_MASK: u32 = 0x00000004;
    pub const WDT_INT_RAW: u32 = 1 << 2;

    /// **T1_INT_RAW**
    ///
    /// The raw interrupt status bit for the TIMG_T1_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const T1_INT_RAW_OFFSET: u32 = 1;
    pub const T1_INT_RAW_MASK: u32 = 0x00000002;
    pub const T1_INT_RAW: u32 = 1 << 1;

    /// **T0_INT_RAW**
    ///
    /// The raw interrupt status bit for the TIMG_T0_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const T0_INT_RAW_OFFSET: u32 = 0;
    pub const T0_INT_RAW_MASK: u32 = 0x00000001;
    pub const T0_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **WDT_INT_RAW** `[2]` (RO)
    ///
    /// The raw interrupt status bit for the TIMG_WDT_INT interrupt.
    #[inline(always)]
    pub fn wdt_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::WDT_INT_RAW_MASK) != 0
    }

    /// **T1_INT_RAW** `[1]` (RO)
    ///
    /// The raw interrupt status bit for the TIMG_T1_INT interrupt.
    #[inline(always)]
    pub fn t1_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::T1_INT_RAW_MASK) != 0
    }

    /// **T0_INT_RAW** `[0]` (RO)
    ///
    /// The raw interrupt status bit for the TIMG_T0_INT interrupt.
    #[inline(always)]
    pub fn t0_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::T0_INT_RAW_MASK) != 0
    }
}

/// INT_RAW_TIMERS value type for batch read/modify/write operations
///
/// Use `intrawtimers.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intrawtimers.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntRawTimersVal {
    bits: u32,
}

impl IntRawTimersVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **WDT_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt status bit for the TIMG_WDT_INT interrupt.
    #[inline(always)]
    pub const fn wdt_int_raw(&self) -> bool {
        (self.bits & IntRawTimers::WDT_INT_RAW_MASK) != 0
    }

    /// Read **T1_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt status bit for the TIMG_T1_INT interrupt.
    #[inline(always)]
    pub const fn t1_int_raw(&self) -> bool {
        (self.bits & IntRawTimers::T1_INT_RAW_MASK) != 0
    }

    /// Read **T0_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt status bit for the TIMG_T0_INT interrupt.
    #[inline(always)]
    pub const fn t0_int_raw(&self) -> bool {
        (self.bits & IntRawTimers::T0_INT_RAW_MASK) != 0
    }
}

impl Default for IntRawTimersVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntRawTimers {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntRawTimersVal {
        IntRawTimersVal::from_bits(self.value.read_bits())
    }
}

/// INT_ST_TIMERS Register
///
/// Masked interrupt status
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **WDT_INT_ST** `[2]` (RO)
///
///   The masked interrupt status bit for the TIMG_WDT_INT interrupt.
///
/// - **T1_INT_ST** `[1]` (RO)
///
///   The masked interrupt status bit for the TIMG_T1_INT interrupt.
///
/// - **T0_INT_ST** `[0]` (RO)
///
///   The masked interrupt status bit for the TIMG_T0_INT interrupt.
#[repr(transparent)]
pub struct IntStTimers {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntStTimers {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntStTimers {
    // ======== Field Constants ========

    /// **WDT_INT_ST**
    ///
    /// The masked interrupt status bit for the TIMG_WDT_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const WDT_INT_ST_OFFSET: u32 = 2;
    pub const WDT_INT_ST_MASK: u32 = 0x00000004;
    pub const WDT_INT_ST: u32 = 1 << 2;

    /// **T1_INT_ST**
    ///
    /// The masked interrupt status bit for the TIMG_T1_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const T1_INT_ST_OFFSET: u32 = 1;
    pub const T1_INT_ST_MASK: u32 = 0x00000002;
    pub const T1_INT_ST: u32 = 1 << 1;

    /// **T0_INT_ST**
    ///
    /// The masked interrupt status bit for the TIMG_T0_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const T0_INT_ST_OFFSET: u32 = 0;
    pub const T0_INT_ST_MASK: u32 = 0x00000001;
    pub const T0_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **WDT_INT_ST** `[2]` (RO)
    ///
    /// The masked interrupt status bit for the TIMG_WDT_INT interrupt.
    #[inline(always)]
    pub fn wdt_int_st(&self) -> bool {
        (self.value.read_bits() & Self::WDT_INT_ST_MASK) != 0
    }

    /// **T1_INT_ST** `[1]` (RO)
    ///
    /// The masked interrupt status bit for the TIMG_T1_INT interrupt.
    #[inline(always)]
    pub fn t1_int_st(&self) -> bool {
        (self.value.read_bits() & Self::T1_INT_ST_MASK) != 0
    }

    /// **T0_INT_ST** `[0]` (RO)
    ///
    /// The masked interrupt status bit for the TIMG_T0_INT interrupt.
    #[inline(always)]
    pub fn t0_int_st(&self) -> bool {
        (self.value.read_bits() & Self::T0_INT_ST_MASK) != 0
    }
}

/// INT_ST_TIMERS value type for batch read/modify/write operations
///
/// Use `intsttimers.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intsttimers.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntStTimersVal {
    bits: u32,
}

impl IntStTimersVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **WDT_INT_ST** `[2:2]`
    ///
    /// The masked interrupt status bit for the TIMG_WDT_INT interrupt.
    #[inline(always)]
    pub const fn wdt_int_st(&self) -> bool {
        (self.bits & IntStTimers::WDT_INT_ST_MASK) != 0
    }

    /// Read **T1_INT_ST** `[1:1]`
    ///
    /// The masked interrupt status bit for the TIMG_T1_INT interrupt.
    #[inline(always)]
    pub const fn t1_int_st(&self) -> bool {
        (self.bits & IntStTimers::T1_INT_ST_MASK) != 0
    }

    /// Read **T0_INT_ST** `[0:0]`
    ///
    /// The masked interrupt status bit for the TIMG_T0_INT interrupt.
    #[inline(always)]
    pub const fn t0_int_st(&self) -> bool {
        (self.bits & IntStTimers::T0_INT_ST_MASK) != 0
    }
}

impl Default for IntStTimersVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntStTimers {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntStTimersVal {
        IntStTimersVal::from_bits(self.value.read_bits())
    }
}

/// INT_CLR_TIMERS Register
///
/// Interrupt clear bits
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **WDT_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear the TIMG_WDT_INT interrupt.
///
/// - **T1_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear the TIMG_T1_INT interrupt.
///
/// - **T0_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the TIMG_T0_INT interrupt.
#[repr(transparent)]
pub struct IntClrTimers {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntClrTimers {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntClrTimers {
    // ======== Field Constants ========

    /// **WDT_INT_CLR**
    ///
    /// Set this bit to clear the TIMG_WDT_INT interrupt.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const WDT_INT_CLR_OFFSET: u32 = 2;
    pub const WDT_INT_CLR_MASK: u32 = 0x00000004;
    pub const WDT_INT_CLR: u32 = 1 << 2;

    /// **T1_INT_CLR**
    ///
    /// Set this bit to clear the TIMG_T1_INT interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const T1_INT_CLR_OFFSET: u32 = 1;
    pub const T1_INT_CLR_MASK: u32 = 0x00000002;
    pub const T1_INT_CLR: u32 = 1 << 1;

    /// **T0_INT_CLR**
    ///
    /// Set this bit to clear the TIMG_T0_INT interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const T0_INT_CLR_OFFSET: u32 = 0;
    pub const T0_INT_CLR_MASK: u32 = 0x00000001;
    pub const T0_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **WDT_INT_CLR** (WO)
    ///
    /// Set this bit to clear the TIMG_WDT_INT interrupt.
    #[inline(always)]
    pub fn trigger_wdt_int_clr(&self) {
        self.value.write_bits(Self::WDT_INT_CLR_MASK);
    }

    /// Trigger **T1_INT_CLR** (WO)
    ///
    /// Set this bit to clear the TIMG_T1_INT interrupt.
    #[inline(always)]
    pub fn trigger_t1_int_clr(&self) {
        self.value.write_bits(Self::T1_INT_CLR_MASK);
    }

    /// Trigger **T0_INT_CLR** (WO)
    ///
    /// Set this bit to clear the TIMG_T0_INT interrupt.
    #[inline(always)]
    pub fn trigger_t0_int_clr(&self) {
        self.value.write_bits(Self::T0_INT_CLR_MASK);
    }
}

/// INT_CLR_TIMERS value type for batch read/modify/write operations
///
/// Use `intclrtimers.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intclrtimers.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntClrTimersVal {
    bits: u32,
}

impl IntClrTimersVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **WDT_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear the TIMG_WDT_INT interrupt.
    #[inline(always)]
    pub const fn with_wdt_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClrTimers::WDT_INT_CLR_MASK
            } else {
                self.bits & !IntClrTimers::WDT_INT_CLR_MASK
            },
        }
    }

    /// Set **T1_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear the TIMG_T1_INT interrupt.
    #[inline(always)]
    pub const fn with_t1_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClrTimers::T1_INT_CLR_MASK
            } else {
                self.bits & !IntClrTimers::T1_INT_CLR_MASK
            },
        }
    }

    /// Set **T0_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the TIMG_T0_INT interrupt.
    #[inline(always)]
    pub const fn with_t0_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClrTimers::T0_INT_CLR_MASK
            } else {
                self.bits & !IntClrTimers::T0_INT_CLR_MASK
            },
        }
    }
}

impl Default for IntClrTimersVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntClrTimers {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntClrTimersVal) {
        self.value.write_bits(val.bits())
    }
}

/// RTCCALICFG2 Register
///
/// Timer group calibration register
///
/// Address offset: 0x0080
/// Reset value: 0xffffff98
///
/// ## Fields
///
/// - **RTC_CALI_TIMEOUT_THRES** `[31:7]` (RW)
///
///   Threshold value for the RTC calibration timer. If the calibration timer's value exceeds this threshold, a timeout is triggered.
///
/// - **RTC_CALI_TIMEOUT_RST_CNT** `[6:3]` (RW)
///
///   Cycles that release calibration timeout reset
///
/// - **RTC_CALI_TIMEOUT** `[0]` (RO)
///
///   RTC calibration timeout indicator
#[repr(transparent)]
pub struct Rtccalicfg2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Rtccalicfg2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Rtccalicfg2 {
    // ======== Field Constants ========

    /// **RTC_CALI_TIMEOUT_THRES**
    ///
    /// Threshold value for the RTC calibration timer. If the calibration timer's value exceeds this threshold, a timeout is triggered.
    ///
    /// Bits `[31:7]`, mask `0xffffff80`
    pub const RTC_CALI_TIMEOUT_THRES_OFFSET: u32 = 7;
    pub const RTC_CALI_TIMEOUT_THRES_MASK: u32 = 0xffffff80;

    /// **RTC_CALI_TIMEOUT_RST_CNT**
    ///
    /// Cycles that release calibration timeout reset
    ///
    /// Bits `[6:3]`, mask `0x00000078`
    pub const RTC_CALI_TIMEOUT_RST_CNT_OFFSET: u32 = 3;
    pub const RTC_CALI_TIMEOUT_RST_CNT_MASK: u32 = 0x00000078;

    /// **RTC_CALI_TIMEOUT**
    ///
    /// RTC calibration timeout indicator
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RTC_CALI_TIMEOUT_OFFSET: u32 = 0;
    pub const RTC_CALI_TIMEOUT_MASK: u32 = 0x00000001;
    pub const RTC_CALI_TIMEOUT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RTC_CALI_TIMEOUT_THRES** `[31:7]` (RW)
    ///
    /// Threshold value for the RTC calibration timer. If the calibration timer's value exceeds this threshold, a timeout is triggered.
    #[inline(always)]
    pub fn rtc_cali_timeout_thres(&self) -> u32 {
        (self.value.read_bits() & Self::RTC_CALI_TIMEOUT_THRES_MASK)
            >> Self::RTC_CALI_TIMEOUT_THRES_OFFSET
    }

    /// Write **RTC_CALI_TIMEOUT_THRES** `[31:7]` (RW)
    ///
    /// Threshold value for the RTC calibration timer. If the calibration timer's value exceeds this threshold, a timeout is triggered.
    #[inline(always)]
    pub fn write_rtc_cali_timeout_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RTC_CALI_TIMEOUT_THRES_MASK)
                | ((value << Self::RTC_CALI_TIMEOUT_THRES_OFFSET)
                    & Self::RTC_CALI_TIMEOUT_THRES_MASK)
        });
    }

    /// **RTC_CALI_TIMEOUT_RST_CNT** `[6:3]` (RW)
    ///
    /// Cycles that release calibration timeout reset
    #[inline(always)]
    pub fn rtc_cali_timeout_rst_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::RTC_CALI_TIMEOUT_RST_CNT_MASK)
            >> Self::RTC_CALI_TIMEOUT_RST_CNT_OFFSET
    }

    /// Write **RTC_CALI_TIMEOUT_RST_CNT** `[6:3]` (RW)
    ///
    /// Cycles that release calibration timeout reset
    #[inline(always)]
    pub fn write_rtc_cali_timeout_rst_cnt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RTC_CALI_TIMEOUT_RST_CNT_MASK)
                | ((value << Self::RTC_CALI_TIMEOUT_RST_CNT_OFFSET)
                    & Self::RTC_CALI_TIMEOUT_RST_CNT_MASK)
        });
    }

    /// **RTC_CALI_TIMEOUT** `[0]` (RO)
    ///
    /// RTC calibration timeout indicator
    #[inline(always)]
    pub fn rtc_cali_timeout(&self) -> bool {
        (self.value.read_bits() & Self::RTC_CALI_TIMEOUT_MASK) != 0
    }
}

/// RTCCALICFG2 value type for batch read/modify/write operations
///
/// Use `rtccalicfg2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rtccalicfg2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Rtccalicfg2Val {
    bits: u32,
}

impl Rtccalicfg2Val {
    /// Create from raw bits
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
        Self { bits: 0xffffff98 }
    }

    /// Read **RTC_CALI_TIMEOUT_THRES** `[31:7]`
    ///
    /// Threshold value for the RTC calibration timer. If the calibration timer's value exceeds this threshold, a timeout is triggered.
    #[inline(always)]
    pub const fn rtc_cali_timeout_thres(&self) -> u32 {
        (self.bits & Rtccalicfg2::RTC_CALI_TIMEOUT_THRES_MASK)
            >> Rtccalicfg2::RTC_CALI_TIMEOUT_THRES_OFFSET
    }

    /// Read **RTC_CALI_TIMEOUT_RST_CNT** `[6:3]`
    ///
    /// Cycles that release calibration timeout reset
    #[inline(always)]
    pub const fn rtc_cali_timeout_rst_cnt(&self) -> u32 {
        (self.bits & Rtccalicfg2::RTC_CALI_TIMEOUT_RST_CNT_MASK)
            >> Rtccalicfg2::RTC_CALI_TIMEOUT_RST_CNT_OFFSET
    }

    /// Read **RTC_CALI_TIMEOUT** `[0:0]`
    ///
    /// RTC calibration timeout indicator
    #[inline(always)]
    pub const fn rtc_cali_timeout(&self) -> bool {
        (self.bits & Rtccalicfg2::RTC_CALI_TIMEOUT_MASK) != 0
    }

    /// Set **RTC_CALI_TIMEOUT_THRES** `[31:7]`
    ///
    /// Threshold value for the RTC calibration timer. If the calibration timer's value exceeds this threshold, a timeout is triggered.
    #[inline(always)]
    pub const fn with_rtc_cali_timeout_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rtccalicfg2::RTC_CALI_TIMEOUT_THRES_MASK)
                | ((value << Rtccalicfg2::RTC_CALI_TIMEOUT_THRES_OFFSET)
                    & Rtccalicfg2::RTC_CALI_TIMEOUT_THRES_MASK),
        }
    }

    /// Set **RTC_CALI_TIMEOUT_RST_CNT** `[6:3]`
    ///
    /// Cycles that release calibration timeout reset
    #[inline(always)]
    pub const fn with_rtc_cali_timeout_rst_cnt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Rtccalicfg2::RTC_CALI_TIMEOUT_RST_CNT_MASK)
                | ((value << Rtccalicfg2::RTC_CALI_TIMEOUT_RST_CNT_OFFSET)
                    & Rtccalicfg2::RTC_CALI_TIMEOUT_RST_CNT_MASK),
        }
    }
}

impl Default for Rtccalicfg2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Rtccalicfg2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Rtccalicfg2Val {
        Rtccalicfg2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Rtccalicfg2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Rtccalicfg2Val) -> Rtccalicfg2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// NTIMERS_DATE Register
///
/// Timer version control register
///
/// Address offset: 0x00f8
/// Reset value: 0x02209142
///
/// ## Fields
///
/// - **NTIMGS_DATE** `[27:0]` (RW)
///
///   Timer version control register
#[repr(transparent)]
pub struct NtimersDate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for NtimersDate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl NtimersDate {
    // ======== Field Constants ========

    /// **NTIMGS_DATE**
    ///
    /// Timer version control register
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const NTIMGS_DATE_OFFSET: u32 = 0;
    pub const NTIMGS_DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **NTIMGS_DATE** `[27:0]` (RW)
    ///
    /// Timer version control register
    #[inline(always)]
    pub fn ntimgs_date(&self) -> u32 {
        (self.value.read_bits() & Self::NTIMGS_DATE_MASK) >> Self::NTIMGS_DATE_OFFSET
    }

    /// Write **NTIMGS_DATE** `[27:0]` (RW)
    ///
    /// Timer version control register
    #[inline(always)]
    pub fn write_ntimgs_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::NTIMGS_DATE_MASK)
                | ((value << Self::NTIMGS_DATE_OFFSET) & Self::NTIMGS_DATE_MASK)
        });
    }
}

/// NTIMERS_DATE value type for batch read/modify/write operations
///
/// Use `ntimersdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ntimersdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct NtimersDateVal {
    bits: u32,
}

impl NtimersDateVal {
    /// Create from raw bits
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
        Self { bits: 0x02209142 }
    }

    /// Read **NTIMGS_DATE** `[27:0]`
    ///
    /// Timer version control register
    #[inline(always)]
    pub const fn ntimgs_date(&self) -> u32 {
        (self.bits & NtimersDate::NTIMGS_DATE_MASK) >> NtimersDate::NTIMGS_DATE_OFFSET
    }

    /// Set **NTIMGS_DATE** `[27:0]`
    ///
    /// Timer version control register
    #[inline(always)]
    pub const fn with_ntimgs_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !NtimersDate::NTIMGS_DATE_MASK)
                | ((value << NtimersDate::NTIMGS_DATE_OFFSET) & NtimersDate::NTIMGS_DATE_MASK),
        }
    }
}

impl Default for NtimersDateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl NtimersDate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> NtimersDateVal {
        NtimersDateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: NtimersDateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(NtimersDateVal) -> NtimersDateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REGCLK Register
///
/// Timer group clock gate register
///
/// Address offset: 0x00fc
/// Reset value: 0x70000000
///
/// ## Fields
///
/// - **CLK_EN** `[31]` (RW)
///
///   Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software.
///
/// - **TIMER_CLK_IS_ACTIVE** `[30]` (RW)
///
///   enable Timer 30's clock
///
/// - **WDT_CLK_IS_ACTIVE** `[29]` (RW)
///
///   enable WDT's clock
///
/// - **ETM_EN** `[28]` (RW)
///
///   enable timer's etm task and event
#[repr(transparent)]
pub struct Regclk {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Regclk {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Regclk {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_EN_OFFSET: u32 = 31;
    pub const CLK_EN_MASK: u32 = 0x80000000;
    pub const CLK_EN: u32 = 1 << 31;

    /// **TIMER_CLK_IS_ACTIVE**
    ///
    /// enable Timer 30's clock
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const TIMER_CLK_IS_ACTIVE_OFFSET: u32 = 30;
    pub const TIMER_CLK_IS_ACTIVE_MASK: u32 = 0x40000000;
    pub const TIMER_CLK_IS_ACTIVE: u32 = 1 << 30;

    /// **WDT_CLK_IS_ACTIVE**
    ///
    /// enable WDT's clock
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const WDT_CLK_IS_ACTIVE_OFFSET: u32 = 29;
    pub const WDT_CLK_IS_ACTIVE_MASK: u32 = 0x20000000;
    pub const WDT_CLK_IS_ACTIVE: u32 = 1 << 29;

    /// **ETM_EN**
    ///
    /// enable timer's etm task and event
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const ETM_EN_OFFSET: u32 = 28;
    pub const ETM_EN_MASK: u32 = 0x10000000;
    pub const ETM_EN: u32 = 1 << 28;

    // ======== Field Accessors ========

    /// **CLK_EN** `[31]` (RW)
    ///
    /// Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software.
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

    /// **TIMER_CLK_IS_ACTIVE** `[30]` (RW)
    ///
    /// enable Timer 30's clock
    #[inline(always)]
    pub fn timer_clk_is_active(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_CLK_IS_ACTIVE_MASK) != 0
    }

    /// Set TIMER_CLK_IS_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_clk_is_active(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER_CLK_IS_ACTIVE_MASK);
    }

    /// Clear TIMER_CLK_IS_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_clk_is_active(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER_CLK_IS_ACTIVE_MASK);
    }

    /// **WDT_CLK_IS_ACTIVE** `[29]` (RW)
    ///
    /// enable WDT's clock
    #[inline(always)]
    pub fn wdt_clk_is_active(&self) -> bool {
        (self.value.read_bits() & Self::WDT_CLK_IS_ACTIVE_MASK) != 0
    }

    /// Set WDT_CLK_IS_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wdt_clk_is_active(&self) {
        self.value.modify_bits(|r| r | Self::WDT_CLK_IS_ACTIVE_MASK);
    }

    /// Clear WDT_CLK_IS_ACTIVE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wdt_clk_is_active(&self) {
        self.value
            .modify_bits(|r| r & !Self::WDT_CLK_IS_ACTIVE_MASK);
    }

    /// **ETM_EN** `[28]` (RW)
    ///
    /// enable timer's etm task and event
    #[inline(always)]
    pub fn etm_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_EN_MASK) != 0
    }

    /// Set ETM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_EN_MASK);
    }

    /// Clear ETM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_en(&self) {
        self.value.modify_bits(|r| r & !Self::ETM_EN_MASK);
    }
}

/// REGCLK value type for batch read/modify/write operations
///
/// Use `regclk.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `regclk.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RegclkVal {
    bits: u32,
}

impl RegclkVal {
    /// Create from raw bits
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
        Self { bits: 0x70000000 }
    }

    /// Read **CLK_EN** `[31:31]`
    ///
    /// Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software.
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & Regclk::CLK_EN_MASK) != 0
    }

    /// Read **TIMER_CLK_IS_ACTIVE** `[30:30]`
    ///
    /// enable Timer 30's clock
    #[inline(always)]
    pub const fn timer_clk_is_active(&self) -> bool {
        (self.bits & Regclk::TIMER_CLK_IS_ACTIVE_MASK) != 0
    }

    /// Read **WDT_CLK_IS_ACTIVE** `[29:29]`
    ///
    /// enable WDT's clock
    #[inline(always)]
    pub const fn wdt_clk_is_active(&self) -> bool {
        (self.bits & Regclk::WDT_CLK_IS_ACTIVE_MASK) != 0
    }

    /// Read **ETM_EN** `[28:28]`
    ///
    /// enable timer's etm task and event
    #[inline(always)]
    pub const fn etm_en(&self) -> bool {
        (self.bits & Regclk::ETM_EN_MASK) != 0
    }

    /// Set **CLK_EN** `[31:31]`
    ///
    /// Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software.
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Regclk::CLK_EN_MASK
            } else {
                self.bits & !Regclk::CLK_EN_MASK
            },
        }
    }

    /// Set **TIMER_CLK_IS_ACTIVE** `[30:30]`
    ///
    /// enable Timer 30's clock
    #[inline(always)]
    pub const fn with_timer_clk_is_active(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Regclk::TIMER_CLK_IS_ACTIVE_MASK
            } else {
                self.bits & !Regclk::TIMER_CLK_IS_ACTIVE_MASK
            },
        }
    }

    /// Set **WDT_CLK_IS_ACTIVE** `[29:29]`
    ///
    /// enable WDT's clock
    #[inline(always)]
    pub const fn with_wdt_clk_is_active(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Regclk::WDT_CLK_IS_ACTIVE_MASK
            } else {
                self.bits & !Regclk::WDT_CLK_IS_ACTIVE_MASK
            },
        }
    }

    /// Set **ETM_EN** `[28:28]`
    ///
    /// enable timer's etm task and event
    #[inline(always)]
    pub const fn with_etm_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Regclk::ETM_EN_MASK
            } else {
                self.bits & !Regclk::ETM_EN_MASK
            },
        }
    }
}

impl Default for RegclkVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Regclk {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RegclkVal {
        RegclkVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RegclkVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RegclkVal) -> RegclkVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Timer %s configuration register
    pub t0config: T0config,
    /// 0x0004
    /// Timer %s current value, low 32 bits
    pub t0lo: T0lo,
    /// 0x0008
    /// Timer %s current value, high 22 bits
    pub t0hi: T0hi,
    /// 0x000c
    /// Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG
    pub t0update: T0update,
    /// 0x0010
    /// Timer %s alarm value, low 32 bits
    pub t0alarmlo: T0alarmlo,
    /// 0x0014
    /// Timer %s alarm value, high bits
    pub t0alarmhi: T0alarmhi,
    /// 0x0018
    /// Timer %s reload value, low 32 bits
    pub t0loadlo: T0loadlo,
    /// 0x001c
    /// Timer %s reload value, high 22 bits
    pub t0loadhi: T0loadhi,
    /// 0x0020
    /// Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG
    pub t0load: T0load,
    /// 0x0024
    /// Timer %s configuration register
    pub t1config: T1config,
    /// 0x0028
    /// Timer %s current value, low 32 bits
    pub t1lo: T1lo,
    /// 0x002c
    /// Timer %s current value, high 22 bits
    pub t1hi: T1hi,
    /// 0x0030
    /// Write to copy current timer value to TIMGn_T%s_(LO/HI)_REG
    pub t1update: T1update,
    /// 0x0034
    /// Timer %s alarm value, low 32 bits
    pub t1alarmlo: T1alarmlo,
    /// 0x0038
    /// Timer %s alarm value, high bits
    pub t1alarmhi: T1alarmhi,
    /// 0x003c
    /// Timer %s reload value, low 32 bits
    pub t1loadlo: T1loadlo,
    /// 0x0040
    /// Timer %s reload value, high 22 bits
    pub t1loadhi: T1loadhi,
    /// 0x0044
    /// Write to reload timer from TIMG_T%s_(LOADLOLOADHI)_REG
    pub t1load: T1load,
    /// 0x0048
    /// Watchdog timer configuration register
    pub wdtconfig0: Wdtconfig0,
    /// 0x004c
    /// Watchdog timer prescaler register
    pub wdtconfig1: Wdtconfig1,
    /// 0x0050
    /// Watchdog timer stage 0 timeout value
    pub wdtconfig2: Wdtconfig2,
    /// 0x0054
    /// Watchdog timer stage 1 timeout value
    pub wdtconfig3: Wdtconfig3,
    /// 0x0058
    /// Watchdog timer stage 2 timeout value
    pub wdtconfig4: Wdtconfig4,
    /// 0x005c
    /// Watchdog timer stage 3 timeout value
    pub wdtconfig5: Wdtconfig5,
    /// 0x0060
    /// Write to feed the watchdog timer
    pub wdtfeed: Wdtfeed,
    /// 0x0064
    /// Watchdog write protect register
    pub wdtwprotect: Wdtwprotect,
    /// 0x0068
    /// RTC calibration configure register
    pub rtccalicfg: Rtccalicfg,
    /// 0x006c
    /// RTC calibration configure1 register
    pub rtccalicfg1: Rtccalicfg1,
    /// 0x0070
    /// Interrupt enable bits
    pub int_ena_timers: IntEnaTimers,
    /// 0x0074
    /// Raw interrupt status
    pub int_raw_timers: IntRawTimers,
    /// 0x0078
    /// Masked interrupt status
    pub int_st_timers: IntStTimers,
    /// 0x007c
    /// Interrupt clear bits
    pub int_clr_timers: IntClrTimers,
    /// 0x0080
    /// Timer group calibration register
    pub rtccalicfg2: Rtccalicfg2,
    _reserved0: [u8; 116],
    /// 0x00f8
    /// Timer version control register
    pub ntimers_date: NtimersDate,
    /// 0x00fc
    /// Timer group clock gate register
    pub regclk: Regclk,
}
