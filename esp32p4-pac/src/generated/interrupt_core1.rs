//! INTERRUPT_CORE1
//! Interrupt Controller (Core 1)

#![allow(dead_code)]

pub const BASE: u32 = 0x500d6800;

/// LP_RTC_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **CORE1_LP_RTC_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpRtcIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpRtcIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpRtcIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_RTC_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_RTC_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_RTC_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_RTC_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_rtc_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_RTC_INT_MAP_MASK)
            >> Self::CORE1_LP_RTC_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_RTC_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_rtc_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_RTC_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_RTC_INT_MAP_OFFSET) & Self::CORE1_LP_RTC_INT_MAP_MASK)
        });
    }
}

/// LP_RTC_INT_MAP value type for batch read/modify/write operations
///
/// Use `lprtcintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lprtcintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpRtcIntMapVal {
    bits: u32,
}

impl LpRtcIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_RTC_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_rtc_int_map(&self) -> u32 {
        (self.bits & LpRtcIntMap::CORE1_LP_RTC_INT_MAP_MASK)
            >> LpRtcIntMap::CORE1_LP_RTC_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_RTC_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_rtc_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpRtcIntMap::CORE1_LP_RTC_INT_MAP_MASK)
                | ((value << LpRtcIntMap::CORE1_LP_RTC_INT_MAP_OFFSET)
                    & LpRtcIntMap::CORE1_LP_RTC_INT_MAP_MASK),
        }
    }
}

impl Default for LpRtcIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpRtcIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpRtcIntMapVal {
        LpRtcIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpRtcIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpRtcIntMapVal) -> LpRtcIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_WDT_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **CORE1_LP_WDT_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpWdtIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpWdtIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpWdtIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_WDT_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_WDT_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_WDT_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_WDT_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_wdt_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_WDT_INT_MAP_MASK)
            >> Self::CORE1_LP_WDT_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_WDT_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_wdt_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_WDT_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_WDT_INT_MAP_OFFSET) & Self::CORE1_LP_WDT_INT_MAP_MASK)
        });
    }
}

/// LP_WDT_INT_MAP value type for batch read/modify/write operations
///
/// Use `lpwdtintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpwdtintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpWdtIntMapVal {
    bits: u32,
}

impl LpWdtIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_WDT_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_wdt_int_map(&self) -> u32 {
        (self.bits & LpWdtIntMap::CORE1_LP_WDT_INT_MAP_MASK)
            >> LpWdtIntMap::CORE1_LP_WDT_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_WDT_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_wdt_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpWdtIntMap::CORE1_LP_WDT_INT_MAP_MASK)
                | ((value << LpWdtIntMap::CORE1_LP_WDT_INT_MAP_OFFSET)
                    & LpWdtIntMap::CORE1_LP_WDT_INT_MAP_MASK),
        }
    }
}

impl Default for LpWdtIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpWdtIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpWdtIntMapVal {
        LpWdtIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpWdtIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpWdtIntMapVal) -> LpWdtIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_TIMER_REG_0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **CORE1_LP_TIMER_REG_0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpTimerReg0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpTimerReg0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpTimerReg0IntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_TIMER_REG_0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_TIMER_REG_0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_TIMER_REG_0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_TIMER_REG_0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_timer_reg_0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_TIMER_REG_0_INT_MAP_MASK)
            >> Self::CORE1_LP_TIMER_REG_0_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_TIMER_REG_0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_timer_reg_0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_TIMER_REG_0_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_TIMER_REG_0_INT_MAP_OFFSET)
                    & Self::CORE1_LP_TIMER_REG_0_INT_MAP_MASK)
        });
    }
}

/// LP_TIMER_REG_0_INT_MAP value type for batch read/modify/write operations
///
/// Use `lptimerreg0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lptimerreg0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpTimerReg0IntMapVal {
    bits: u32,
}

impl LpTimerReg0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_TIMER_REG_0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_timer_reg_0_int_map(&self) -> u32 {
        (self.bits & LpTimerReg0IntMap::CORE1_LP_TIMER_REG_0_INT_MAP_MASK)
            >> LpTimerReg0IntMap::CORE1_LP_TIMER_REG_0_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_TIMER_REG_0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_timer_reg_0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpTimerReg0IntMap::CORE1_LP_TIMER_REG_0_INT_MAP_MASK)
                | ((value << LpTimerReg0IntMap::CORE1_LP_TIMER_REG_0_INT_MAP_OFFSET)
                    & LpTimerReg0IntMap::CORE1_LP_TIMER_REG_0_INT_MAP_MASK),
        }
    }
}

impl Default for LpTimerReg0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpTimerReg0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpTimerReg0IntMapVal {
        LpTimerReg0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpTimerReg0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpTimerReg0IntMapVal) -> LpTimerReg0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_TIMER_REG_1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **CORE1_LP_TIMER_REG_1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpTimerReg1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpTimerReg1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpTimerReg1IntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_TIMER_REG_1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_TIMER_REG_1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_TIMER_REG_1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_TIMER_REG_1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_timer_reg_1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_TIMER_REG_1_INT_MAP_MASK)
            >> Self::CORE1_LP_TIMER_REG_1_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_TIMER_REG_1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_timer_reg_1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_TIMER_REG_1_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_TIMER_REG_1_INT_MAP_OFFSET)
                    & Self::CORE1_LP_TIMER_REG_1_INT_MAP_MASK)
        });
    }
}

/// LP_TIMER_REG_1_INT_MAP value type for batch read/modify/write operations
///
/// Use `lptimerreg1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lptimerreg1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpTimerReg1IntMapVal {
    bits: u32,
}

impl LpTimerReg1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_TIMER_REG_1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_timer_reg_1_int_map(&self) -> u32 {
        (self.bits & LpTimerReg1IntMap::CORE1_LP_TIMER_REG_1_INT_MAP_MASK)
            >> LpTimerReg1IntMap::CORE1_LP_TIMER_REG_1_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_TIMER_REG_1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_timer_reg_1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpTimerReg1IntMap::CORE1_LP_TIMER_REG_1_INT_MAP_MASK)
                | ((value << LpTimerReg1IntMap::CORE1_LP_TIMER_REG_1_INT_MAP_OFFSET)
                    & LpTimerReg1IntMap::CORE1_LP_TIMER_REG_1_INT_MAP_MASK),
        }
    }
}

impl Default for LpTimerReg1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpTimerReg1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpTimerReg1IntMapVal {
        LpTimerReg1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpTimerReg1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpTimerReg1IntMapVal) -> LpTimerReg1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MB_HP_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **CORE1_MB_HP_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct MbHpIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MbHpIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MbHpIntMap {
    // ======== Field Constants ========

    /// **CORE1_MB_HP_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_MB_HP_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_MB_HP_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_MB_HP_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_mb_hp_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_MB_HP_INT_MAP_MASK)
            >> Self::CORE1_MB_HP_INT_MAP_OFFSET
    }

    /// Write **CORE1_MB_HP_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_mb_hp_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_MB_HP_INT_MAP_MASK)
                | ((value << Self::CORE1_MB_HP_INT_MAP_OFFSET) & Self::CORE1_MB_HP_INT_MAP_MASK)
        });
    }
}

/// MB_HP_INT_MAP value type for batch read/modify/write operations
///
/// Use `mbhpintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mbhpintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MbHpIntMapVal {
    bits: u32,
}

impl MbHpIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_MB_HP_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_mb_hp_int_map(&self) -> u32 {
        (self.bits & MbHpIntMap::CORE1_MB_HP_INT_MAP_MASK) >> MbHpIntMap::CORE1_MB_HP_INT_MAP_OFFSET
    }

    /// Set **CORE1_MB_HP_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_mb_hp_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !MbHpIntMap::CORE1_MB_HP_INT_MAP_MASK)
                | ((value << MbHpIntMap::CORE1_MB_HP_INT_MAP_OFFSET)
                    & MbHpIntMap::CORE1_MB_HP_INT_MAP_MASK),
        }
    }
}

impl Default for MbHpIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MbHpIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MbHpIntMapVal {
        MbHpIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MbHpIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MbHpIntMapVal) -> MbHpIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MB_LP_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **CORE1_MB_LP_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct MbLpIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MbLpIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MbLpIntMap {
    // ======== Field Constants ========

    /// **CORE1_MB_LP_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_MB_LP_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_MB_LP_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_MB_LP_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_mb_lp_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_MB_LP_INT_MAP_MASK)
            >> Self::CORE1_MB_LP_INT_MAP_OFFSET
    }

    /// Write **CORE1_MB_LP_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_mb_lp_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_MB_LP_INT_MAP_MASK)
                | ((value << Self::CORE1_MB_LP_INT_MAP_OFFSET) & Self::CORE1_MB_LP_INT_MAP_MASK)
        });
    }
}

/// MB_LP_INT_MAP value type for batch read/modify/write operations
///
/// Use `mblpintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mblpintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MbLpIntMapVal {
    bits: u32,
}

impl MbLpIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_MB_LP_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_mb_lp_int_map(&self) -> u32 {
        (self.bits & MbLpIntMap::CORE1_MB_LP_INT_MAP_MASK) >> MbLpIntMap::CORE1_MB_LP_INT_MAP_OFFSET
    }

    /// Set **CORE1_MB_LP_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_mb_lp_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !MbLpIntMap::CORE1_MB_LP_INT_MAP_MASK)
                | ((value << MbLpIntMap::CORE1_MB_LP_INT_MAP_OFFSET)
                    & MbLpIntMap::CORE1_MB_LP_INT_MAP_MASK),
        }
    }
}

impl Default for MbLpIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MbLpIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MbLpIntMapVal {
        MbLpIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MbLpIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MbLpIntMapVal) -> MbLpIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMU_REG_0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **CORE1_PMU_REG_0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PmuReg0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmuReg0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmuReg0IntMap {
    // ======== Field Constants ========

    /// **CORE1_PMU_REG_0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_PMU_REG_0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_PMU_REG_0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_PMU_REG_0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_pmu_reg_0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_PMU_REG_0_INT_MAP_MASK)
            >> Self::CORE1_PMU_REG_0_INT_MAP_OFFSET
    }

    /// Write **CORE1_PMU_REG_0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_pmu_reg_0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_PMU_REG_0_INT_MAP_MASK)
                | ((value << Self::CORE1_PMU_REG_0_INT_MAP_OFFSET)
                    & Self::CORE1_PMU_REG_0_INT_MAP_MASK)
        });
    }
}

/// PMU_REG_0_INT_MAP value type for batch read/modify/write operations
///
/// Use `pmureg0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmureg0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmuReg0IntMapVal {
    bits: u32,
}

impl PmuReg0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_PMU_REG_0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_pmu_reg_0_int_map(&self) -> u32 {
        (self.bits & PmuReg0IntMap::CORE1_PMU_REG_0_INT_MAP_MASK)
            >> PmuReg0IntMap::CORE1_PMU_REG_0_INT_MAP_OFFSET
    }

    /// Set **CORE1_PMU_REG_0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_pmu_reg_0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmuReg0IntMap::CORE1_PMU_REG_0_INT_MAP_MASK)
                | ((value << PmuReg0IntMap::CORE1_PMU_REG_0_INT_MAP_OFFSET)
                    & PmuReg0IntMap::CORE1_PMU_REG_0_INT_MAP_MASK),
        }
    }
}

impl Default for PmuReg0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmuReg0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmuReg0IntMapVal {
        PmuReg0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmuReg0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmuReg0IntMapVal) -> PmuReg0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMU_REG_1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **CORE1_PMU_REG_1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PmuReg1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmuReg1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmuReg1IntMap {
    // ======== Field Constants ========

    /// **CORE1_PMU_REG_1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_PMU_REG_1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_PMU_REG_1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_PMU_REG_1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_pmu_reg_1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_PMU_REG_1_INT_MAP_MASK)
            >> Self::CORE1_PMU_REG_1_INT_MAP_OFFSET
    }

    /// Write **CORE1_PMU_REG_1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_pmu_reg_1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_PMU_REG_1_INT_MAP_MASK)
                | ((value << Self::CORE1_PMU_REG_1_INT_MAP_OFFSET)
                    & Self::CORE1_PMU_REG_1_INT_MAP_MASK)
        });
    }
}

/// PMU_REG_1_INT_MAP value type for batch read/modify/write operations
///
/// Use `pmureg1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmureg1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmuReg1IntMapVal {
    bits: u32,
}

impl PmuReg1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_PMU_REG_1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_pmu_reg_1_int_map(&self) -> u32 {
        (self.bits & PmuReg1IntMap::CORE1_PMU_REG_1_INT_MAP_MASK)
            >> PmuReg1IntMap::CORE1_PMU_REG_1_INT_MAP_OFFSET
    }

    /// Set **CORE1_PMU_REG_1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_pmu_reg_1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmuReg1IntMap::CORE1_PMU_REG_1_INT_MAP_MASK)
                | ((value << PmuReg1IntMap::CORE1_PMU_REG_1_INT_MAP_OFFSET)
                    & PmuReg1IntMap::CORE1_PMU_REG_1_INT_MAP_MASK),
        }
    }
}

impl Default for PmuReg1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmuReg1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmuReg1IntMapVal {
        PmuReg1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmuReg1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmuReg1IntMapVal) -> PmuReg1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANAPERI_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **CORE1_LP_ANAPERI_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpAnaperiIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaperiIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaperiIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_ANAPERI_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_ANAPERI_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_ANAPERI_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_ANAPERI_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_anaperi_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_ANAPERI_INT_MAP_MASK)
            >> Self::CORE1_LP_ANAPERI_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_ANAPERI_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_anaperi_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_ANAPERI_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_ANAPERI_INT_MAP_OFFSET)
                    & Self::CORE1_LP_ANAPERI_INT_MAP_MASK)
        });
    }
}

/// LP_ANAPERI_INT_MAP value type for batch read/modify/write operations
///
/// Use `lpanaperiintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanaperiintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaperiIntMapVal {
    bits: u32,
}

impl LpAnaperiIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_ANAPERI_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_anaperi_int_map(&self) -> u32 {
        (self.bits & LpAnaperiIntMap::CORE1_LP_ANAPERI_INT_MAP_MASK)
            >> LpAnaperiIntMap::CORE1_LP_ANAPERI_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_ANAPERI_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_anaperi_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaperiIntMap::CORE1_LP_ANAPERI_INT_MAP_MASK)
                | ((value << LpAnaperiIntMap::CORE1_LP_ANAPERI_INT_MAP_OFFSET)
                    & LpAnaperiIntMap::CORE1_LP_ANAPERI_INT_MAP_MASK),
        }
    }
}

impl Default for LpAnaperiIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaperiIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaperiIntMapVal {
        LpAnaperiIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaperiIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaperiIntMapVal) -> LpAnaperiIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ADC_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **CORE1_LP_ADC_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpAdcIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAdcIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAdcIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_ADC_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_ADC_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_ADC_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_ADC_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_adc_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_ADC_INT_MAP_MASK)
            >> Self::CORE1_LP_ADC_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_ADC_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_adc_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_ADC_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_ADC_INT_MAP_OFFSET) & Self::CORE1_LP_ADC_INT_MAP_MASK)
        });
    }
}

/// LP_ADC_INT_MAP value type for batch read/modify/write operations
///
/// Use `lpadcintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpadcintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAdcIntMapVal {
    bits: u32,
}

impl LpAdcIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_ADC_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_adc_int_map(&self) -> u32 {
        (self.bits & LpAdcIntMap::CORE1_LP_ADC_INT_MAP_MASK)
            >> LpAdcIntMap::CORE1_LP_ADC_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_ADC_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_adc_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAdcIntMap::CORE1_LP_ADC_INT_MAP_MASK)
                | ((value << LpAdcIntMap::CORE1_LP_ADC_INT_MAP_OFFSET)
                    & LpAdcIntMap::CORE1_LP_ADC_INT_MAP_MASK),
        }
    }
}

impl Default for LpAdcIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAdcIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAdcIntMapVal {
        LpAdcIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAdcIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAdcIntMapVal) -> LpAdcIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_GPIO_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **CORE1_LP_GPIO_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpGpioIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpGpioIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpGpioIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_GPIO_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_GPIO_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_GPIO_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_GPIO_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_gpio_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_GPIO_INT_MAP_MASK)
            >> Self::CORE1_LP_GPIO_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_GPIO_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_gpio_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_GPIO_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_GPIO_INT_MAP_OFFSET) & Self::CORE1_LP_GPIO_INT_MAP_MASK)
        });
    }
}

/// LP_GPIO_INT_MAP value type for batch read/modify/write operations
///
/// Use `lpgpiointmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpgpiointmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpGpioIntMapVal {
    bits: u32,
}

impl LpGpioIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_GPIO_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_gpio_int_map(&self) -> u32 {
        (self.bits & LpGpioIntMap::CORE1_LP_GPIO_INT_MAP_MASK)
            >> LpGpioIntMap::CORE1_LP_GPIO_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_GPIO_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_gpio_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpGpioIntMap::CORE1_LP_GPIO_INT_MAP_MASK)
                | ((value << LpGpioIntMap::CORE1_LP_GPIO_INT_MAP_OFFSET)
                    & LpGpioIntMap::CORE1_LP_GPIO_INT_MAP_MASK),
        }
    }
}

impl Default for LpGpioIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpGpioIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpGpioIntMapVal {
        LpGpioIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpGpioIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpGpioIntMapVal) -> LpGpioIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_I2C_INT_MAP Register
///
/// NA
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **CORE1_LP_I2C_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpI2cIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpI2cIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpI2cIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_I2C_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_I2C_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_I2C_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_I2C_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_i2c_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_I2C_INT_MAP_MASK)
            >> Self::CORE1_LP_I2C_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_I2C_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_i2c_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_I2C_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_I2C_INT_MAP_OFFSET) & Self::CORE1_LP_I2C_INT_MAP_MASK)
        });
    }
}

/// LP_I2C_INT_MAP value type for batch read/modify/write operations
///
/// Use `lpi2cintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpi2cintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpI2cIntMapVal {
    bits: u32,
}

impl LpI2cIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_I2C_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_i2c_int_map(&self) -> u32 {
        (self.bits & LpI2cIntMap::CORE1_LP_I2C_INT_MAP_MASK)
            >> LpI2cIntMap::CORE1_LP_I2C_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_I2C_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_i2c_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpI2cIntMap::CORE1_LP_I2C_INT_MAP_MASK)
                | ((value << LpI2cIntMap::CORE1_LP_I2C_INT_MAP_OFFSET)
                    & LpI2cIntMap::CORE1_LP_I2C_INT_MAP_MASK),
        }
    }
}

impl Default for LpI2cIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpI2cIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpI2cIntMapVal {
        LpI2cIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpI2cIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpI2cIntMapVal) -> LpI2cIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_I2S_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **CORE1_LP_I2S_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpI2sIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpI2sIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpI2sIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_I2S_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_I2S_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_I2S_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_I2S_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_i2s_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_I2S_INT_MAP_MASK)
            >> Self::CORE1_LP_I2S_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_I2S_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_i2s_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_I2S_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_I2S_INT_MAP_OFFSET) & Self::CORE1_LP_I2S_INT_MAP_MASK)
        });
    }
}

/// LP_I2S_INT_MAP value type for batch read/modify/write operations
///
/// Use `lpi2sintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpi2sintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpI2sIntMapVal {
    bits: u32,
}

impl LpI2sIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_I2S_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_i2s_int_map(&self) -> u32 {
        (self.bits & LpI2sIntMap::CORE1_LP_I2S_INT_MAP_MASK)
            >> LpI2sIntMap::CORE1_LP_I2S_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_I2S_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_i2s_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpI2sIntMap::CORE1_LP_I2S_INT_MAP_MASK)
                | ((value << LpI2sIntMap::CORE1_LP_I2S_INT_MAP_OFFSET)
                    & LpI2sIntMap::CORE1_LP_I2S_INT_MAP_MASK),
        }
    }
}

impl Default for LpI2sIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpI2sIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpI2sIntMapVal {
        LpI2sIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpI2sIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpI2sIntMapVal) -> LpI2sIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_SPI_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **CORE1_LP_SPI_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpSpiIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpSpiIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpSpiIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_SPI_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_SPI_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_SPI_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_SPI_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_spi_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_SPI_INT_MAP_MASK)
            >> Self::CORE1_LP_SPI_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_SPI_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_spi_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_SPI_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_SPI_INT_MAP_OFFSET) & Self::CORE1_LP_SPI_INT_MAP_MASK)
        });
    }
}

/// LP_SPI_INT_MAP value type for batch read/modify/write operations
///
/// Use `lpspiintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpspiintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpSpiIntMapVal {
    bits: u32,
}

impl LpSpiIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_SPI_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_spi_int_map(&self) -> u32 {
        (self.bits & LpSpiIntMap::CORE1_LP_SPI_INT_MAP_MASK)
            >> LpSpiIntMap::CORE1_LP_SPI_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_SPI_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_spi_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpSpiIntMap::CORE1_LP_SPI_INT_MAP_MASK)
                | ((value << LpSpiIntMap::CORE1_LP_SPI_INT_MAP_OFFSET)
                    & LpSpiIntMap::CORE1_LP_SPI_INT_MAP_MASK),
        }
    }
}

impl Default for LpSpiIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpSpiIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpSpiIntMapVal {
        LpSpiIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpSpiIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpSpiIntMapVal) -> LpSpiIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_TOUCH_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **CORE1_LP_TOUCH_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpTouchIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpTouchIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpTouchIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_TOUCH_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_TOUCH_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_TOUCH_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_TOUCH_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_touch_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_TOUCH_INT_MAP_MASK)
            >> Self::CORE1_LP_TOUCH_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_TOUCH_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_touch_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_TOUCH_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_TOUCH_INT_MAP_OFFSET)
                    & Self::CORE1_LP_TOUCH_INT_MAP_MASK)
        });
    }
}

/// LP_TOUCH_INT_MAP value type for batch read/modify/write operations
///
/// Use `lptouchintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lptouchintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpTouchIntMapVal {
    bits: u32,
}

impl LpTouchIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_TOUCH_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_touch_int_map(&self) -> u32 {
        (self.bits & LpTouchIntMap::CORE1_LP_TOUCH_INT_MAP_MASK)
            >> LpTouchIntMap::CORE1_LP_TOUCH_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_TOUCH_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_touch_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpTouchIntMap::CORE1_LP_TOUCH_INT_MAP_MASK)
                | ((value << LpTouchIntMap::CORE1_LP_TOUCH_INT_MAP_OFFSET)
                    & LpTouchIntMap::CORE1_LP_TOUCH_INT_MAP_MASK),
        }
    }
}

impl Default for LpTouchIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpTouchIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpTouchIntMapVal {
        LpTouchIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpTouchIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpTouchIntMapVal) -> LpTouchIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_TSENS_INT_MAP Register
///
/// NA
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **CORE1_LP_TSENS_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpTsensIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpTsensIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpTsensIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_TSENS_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_TSENS_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_TSENS_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_TSENS_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_tsens_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_TSENS_INT_MAP_MASK)
            >> Self::CORE1_LP_TSENS_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_TSENS_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_tsens_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_TSENS_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_TSENS_INT_MAP_OFFSET)
                    & Self::CORE1_LP_TSENS_INT_MAP_MASK)
        });
    }
}

/// LP_TSENS_INT_MAP value type for batch read/modify/write operations
///
/// Use `lptsensintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lptsensintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpTsensIntMapVal {
    bits: u32,
}

impl LpTsensIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_TSENS_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_tsens_int_map(&self) -> u32 {
        (self.bits & LpTsensIntMap::CORE1_LP_TSENS_INT_MAP_MASK)
            >> LpTsensIntMap::CORE1_LP_TSENS_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_TSENS_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_tsens_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpTsensIntMap::CORE1_LP_TSENS_INT_MAP_MASK)
                | ((value << LpTsensIntMap::CORE1_LP_TSENS_INT_MAP_OFFSET)
                    & LpTsensIntMap::CORE1_LP_TSENS_INT_MAP_MASK),
        }
    }
}

impl Default for LpTsensIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpTsensIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpTsensIntMapVal {
        LpTsensIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpTsensIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpTsensIntMapVal) -> LpTsensIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_UART_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **CORE1_LP_UART_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpUartIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpUartIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpUartIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_UART_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_UART_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_UART_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_UART_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_uart_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_UART_INT_MAP_MASK)
            >> Self::CORE1_LP_UART_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_UART_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_uart_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_UART_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_UART_INT_MAP_OFFSET) & Self::CORE1_LP_UART_INT_MAP_MASK)
        });
    }
}

/// LP_UART_INT_MAP value type for batch read/modify/write operations
///
/// Use `lpuartintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpuartintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpUartIntMapVal {
    bits: u32,
}

impl LpUartIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_UART_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_uart_int_map(&self) -> u32 {
        (self.bits & LpUartIntMap::CORE1_LP_UART_INT_MAP_MASK)
            >> LpUartIntMap::CORE1_LP_UART_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_UART_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_uart_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpUartIntMap::CORE1_LP_UART_INT_MAP_MASK)
                | ((value << LpUartIntMap::CORE1_LP_UART_INT_MAP_OFFSET)
                    & LpUartIntMap::CORE1_LP_UART_INT_MAP_MASK),
        }
    }
}

impl Default for LpUartIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpUartIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpUartIntMapVal {
        LpUartIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpUartIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpUartIntMapVal) -> LpUartIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_EFUSE_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **CORE1_LP_EFUSE_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpEfuseIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpEfuseIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpEfuseIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_EFUSE_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_EFUSE_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_EFUSE_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_EFUSE_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_efuse_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_EFUSE_INT_MAP_MASK)
            >> Self::CORE1_LP_EFUSE_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_EFUSE_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_efuse_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_EFUSE_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_EFUSE_INT_MAP_OFFSET)
                    & Self::CORE1_LP_EFUSE_INT_MAP_MASK)
        });
    }
}

/// LP_EFUSE_INT_MAP value type for batch read/modify/write operations
///
/// Use `lpefuseintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpefuseintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpEfuseIntMapVal {
    bits: u32,
}

impl LpEfuseIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_EFUSE_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_efuse_int_map(&self) -> u32 {
        (self.bits & LpEfuseIntMap::CORE1_LP_EFUSE_INT_MAP_MASK)
            >> LpEfuseIntMap::CORE1_LP_EFUSE_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_EFUSE_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_efuse_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpEfuseIntMap::CORE1_LP_EFUSE_INT_MAP_MASK)
                | ((value << LpEfuseIntMap::CORE1_LP_EFUSE_INT_MAP_OFFSET)
                    & LpEfuseIntMap::CORE1_LP_EFUSE_INT_MAP_MASK),
        }
    }
}

impl Default for LpEfuseIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpEfuseIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpEfuseIntMapVal {
        LpEfuseIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpEfuseIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpEfuseIntMapVal) -> LpEfuseIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_SW_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **CORE1_LP_SW_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpSwIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpSwIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpSwIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_SW_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_SW_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_SW_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_SW_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_sw_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_SW_INT_MAP_MASK)
            >> Self::CORE1_LP_SW_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_SW_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_sw_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_SW_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_SW_INT_MAP_OFFSET) & Self::CORE1_LP_SW_INT_MAP_MASK)
        });
    }
}

/// LP_SW_INT_MAP value type for batch read/modify/write operations
///
/// Use `lpswintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpswintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpSwIntMapVal {
    bits: u32,
}

impl LpSwIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_SW_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_sw_int_map(&self) -> u32 {
        (self.bits & LpSwIntMap::CORE1_LP_SW_INT_MAP_MASK) >> LpSwIntMap::CORE1_LP_SW_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_SW_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_sw_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpSwIntMap::CORE1_LP_SW_INT_MAP_MASK)
                | ((value << LpSwIntMap::CORE1_LP_SW_INT_MAP_OFFSET)
                    & LpSwIntMap::CORE1_LP_SW_INT_MAP_MASK),
        }
    }
}

impl Default for LpSwIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpSwIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpSwIntMapVal {
        LpSwIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpSwIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpSwIntMapVal) -> LpSwIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_SYSREG_INT_MAP Register
///
/// NA
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **CORE1_LP_SYSREG_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpSysregIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpSysregIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpSysregIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_SYSREG_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_SYSREG_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_SYSREG_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_SYSREG_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_sysreg_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_SYSREG_INT_MAP_MASK)
            >> Self::CORE1_LP_SYSREG_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_SYSREG_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_sysreg_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_SYSREG_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_SYSREG_INT_MAP_OFFSET)
                    & Self::CORE1_LP_SYSREG_INT_MAP_MASK)
        });
    }
}

/// LP_SYSREG_INT_MAP value type for batch read/modify/write operations
///
/// Use `lpsysregintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpsysregintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpSysregIntMapVal {
    bits: u32,
}

impl LpSysregIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_SYSREG_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_sysreg_int_map(&self) -> u32 {
        (self.bits & LpSysregIntMap::CORE1_LP_SYSREG_INT_MAP_MASK)
            >> LpSysregIntMap::CORE1_LP_SYSREG_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_SYSREG_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_sysreg_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpSysregIntMap::CORE1_LP_SYSREG_INT_MAP_MASK)
                | ((value << LpSysregIntMap::CORE1_LP_SYSREG_INT_MAP_OFFSET)
                    & LpSysregIntMap::CORE1_LP_SYSREG_INT_MAP_MASK),
        }
    }
}

impl Default for LpSysregIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpSysregIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpSysregIntMapVal {
        LpSysregIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpSysregIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpSysregIntMapVal) -> LpSysregIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_HUK_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **CORE1_LP_HUK_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpHukIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpHukIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpHukIntMap {
    // ======== Field Constants ========

    /// **CORE1_LP_HUK_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LP_HUK_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LP_HUK_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LP_HUK_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lp_huk_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LP_HUK_INT_MAP_MASK)
            >> Self::CORE1_LP_HUK_INT_MAP_OFFSET
    }

    /// Write **CORE1_LP_HUK_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lp_huk_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LP_HUK_INT_MAP_MASK)
                | ((value << Self::CORE1_LP_HUK_INT_MAP_OFFSET) & Self::CORE1_LP_HUK_INT_MAP_MASK)
        });
    }
}

/// LP_HUK_INT_MAP value type for batch read/modify/write operations
///
/// Use `lphukintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lphukintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpHukIntMapVal {
    bits: u32,
}

impl LpHukIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LP_HUK_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lp_huk_int_map(&self) -> u32 {
        (self.bits & LpHukIntMap::CORE1_LP_HUK_INT_MAP_MASK)
            >> LpHukIntMap::CORE1_LP_HUK_INT_MAP_OFFSET
    }

    /// Set **CORE1_LP_HUK_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lp_huk_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpHukIntMap::CORE1_LP_HUK_INT_MAP_MASK)
                | ((value << LpHukIntMap::CORE1_LP_HUK_INT_MAP_OFFSET)
                    & LpHukIntMap::CORE1_LP_HUK_INT_MAP_MASK),
        }
    }
}

impl Default for LpHukIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpHukIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpHukIntMapVal {
        LpHukIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpHukIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpHukIntMapVal) -> LpHukIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SYS_ICM_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **CORE1_SYS_ICM_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct SysIcmIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SysIcmIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SysIcmIntMap {
    // ======== Field Constants ========

    /// **CORE1_SYS_ICM_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_SYS_ICM_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_SYS_ICM_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_SYS_ICM_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_sys_icm_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_SYS_ICM_INT_MAP_MASK)
            >> Self::CORE1_SYS_ICM_INT_MAP_OFFSET
    }

    /// Write **CORE1_SYS_ICM_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_sys_icm_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_SYS_ICM_INT_MAP_MASK)
                | ((value << Self::CORE1_SYS_ICM_INT_MAP_OFFSET) & Self::CORE1_SYS_ICM_INT_MAP_MASK)
        });
    }
}

/// SYS_ICM_INT_MAP value type for batch read/modify/write operations
///
/// Use `sysicmintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sysicmintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SysIcmIntMapVal {
    bits: u32,
}

impl SysIcmIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_SYS_ICM_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_sys_icm_int_map(&self) -> u32 {
        (self.bits & SysIcmIntMap::CORE1_SYS_ICM_INT_MAP_MASK)
            >> SysIcmIntMap::CORE1_SYS_ICM_INT_MAP_OFFSET
    }

    /// Set **CORE1_SYS_ICM_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_sys_icm_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SysIcmIntMap::CORE1_SYS_ICM_INT_MAP_MASK)
                | ((value << SysIcmIntMap::CORE1_SYS_ICM_INT_MAP_OFFSET)
                    & SysIcmIntMap::CORE1_SYS_ICM_INT_MAP_MASK),
        }
    }
}

impl Default for SysIcmIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SysIcmIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SysIcmIntMapVal {
        SysIcmIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SysIcmIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SysIcmIntMapVal) -> SysIcmIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USB_DEVICE_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **CORE1_USB_DEVICE_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct UsbDeviceIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for UsbDeviceIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UsbDeviceIntMap {
    // ======== Field Constants ========

    /// **CORE1_USB_DEVICE_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_USB_DEVICE_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_USB_DEVICE_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_USB_DEVICE_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_usb_device_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_USB_DEVICE_INT_MAP_MASK)
            >> Self::CORE1_USB_DEVICE_INT_MAP_OFFSET
    }

    /// Write **CORE1_USB_DEVICE_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_usb_device_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_USB_DEVICE_INT_MAP_MASK)
                | ((value << Self::CORE1_USB_DEVICE_INT_MAP_OFFSET)
                    & Self::CORE1_USB_DEVICE_INT_MAP_MASK)
        });
    }
}

/// USB_DEVICE_INT_MAP value type for batch read/modify/write operations
///
/// Use `usbdeviceintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbdeviceintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbDeviceIntMapVal {
    bits: u32,
}

impl UsbDeviceIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_USB_DEVICE_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_usb_device_int_map(&self) -> u32 {
        (self.bits & UsbDeviceIntMap::CORE1_USB_DEVICE_INT_MAP_MASK)
            >> UsbDeviceIntMap::CORE1_USB_DEVICE_INT_MAP_OFFSET
    }

    /// Set **CORE1_USB_DEVICE_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_usb_device_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbDeviceIntMap::CORE1_USB_DEVICE_INT_MAP_MASK)
                | ((value << UsbDeviceIntMap::CORE1_USB_DEVICE_INT_MAP_OFFSET)
                    & UsbDeviceIntMap::CORE1_USB_DEVICE_INT_MAP_MASK),
        }
    }
}

impl Default for UsbDeviceIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl UsbDeviceIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UsbDeviceIntMapVal {
        UsbDeviceIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UsbDeviceIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UsbDeviceIntMapVal) -> UsbDeviceIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SDIO_HOST_INT_MAP Register
///
/// NA
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **CORE1_SDIO_HOST_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct SdioHostIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SdioHostIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SdioHostIntMap {
    // ======== Field Constants ========

    /// **CORE1_SDIO_HOST_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_SDIO_HOST_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_SDIO_HOST_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_SDIO_HOST_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_sdio_host_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_SDIO_HOST_INT_MAP_MASK)
            >> Self::CORE1_SDIO_HOST_INT_MAP_OFFSET
    }

    /// Write **CORE1_SDIO_HOST_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_sdio_host_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_SDIO_HOST_INT_MAP_MASK)
                | ((value << Self::CORE1_SDIO_HOST_INT_MAP_OFFSET)
                    & Self::CORE1_SDIO_HOST_INT_MAP_MASK)
        });
    }
}

/// SDIO_HOST_INT_MAP value type for batch read/modify/write operations
///
/// Use `sdiohostintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sdiohostintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SdioHostIntMapVal {
    bits: u32,
}

impl SdioHostIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_SDIO_HOST_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_sdio_host_int_map(&self) -> u32 {
        (self.bits & SdioHostIntMap::CORE1_SDIO_HOST_INT_MAP_MASK)
            >> SdioHostIntMap::CORE1_SDIO_HOST_INT_MAP_OFFSET
    }

    /// Set **CORE1_SDIO_HOST_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_sdio_host_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SdioHostIntMap::CORE1_SDIO_HOST_INT_MAP_MASK)
                | ((value << SdioHostIntMap::CORE1_SDIO_HOST_INT_MAP_OFFSET)
                    & SdioHostIntMap::CORE1_SDIO_HOST_INT_MAP_MASK),
        }
    }
}

impl Default for SdioHostIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SdioHostIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SdioHostIntMapVal {
        SdioHostIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SdioHostIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SdioHostIntMapVal) -> SdioHostIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GDMA_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **CORE1_GDMA_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct GdmaIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GdmaIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GdmaIntMap {
    // ======== Field Constants ========

    /// **CORE1_GDMA_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_GDMA_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_GDMA_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_GDMA_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_gdma_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_GDMA_INT_MAP_MASK) >> Self::CORE1_GDMA_INT_MAP_OFFSET
    }

    /// Write **CORE1_GDMA_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_gdma_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_GDMA_INT_MAP_MASK)
                | ((value << Self::CORE1_GDMA_INT_MAP_OFFSET) & Self::CORE1_GDMA_INT_MAP_MASK)
        });
    }
}

/// GDMA_INT_MAP value type for batch read/modify/write operations
///
/// Use `gdmaintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gdmaintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GdmaIntMapVal {
    bits: u32,
}

impl GdmaIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_GDMA_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_gdma_int_map(&self) -> u32 {
        (self.bits & GdmaIntMap::CORE1_GDMA_INT_MAP_MASK) >> GdmaIntMap::CORE1_GDMA_INT_MAP_OFFSET
    }

    /// Set **CORE1_GDMA_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_gdma_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GdmaIntMap::CORE1_GDMA_INT_MAP_MASK)
                | ((value << GdmaIntMap::CORE1_GDMA_INT_MAP_OFFSET)
                    & GdmaIntMap::CORE1_GDMA_INT_MAP_MASK),
        }
    }
}

impl Default for GdmaIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GdmaIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GdmaIntMapVal {
        GdmaIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GdmaIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GdmaIntMapVal) -> GdmaIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI2_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **CORE1_SPI2_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Spi2IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spi2IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spi2IntMap {
    // ======== Field Constants ========

    /// **CORE1_SPI2_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_SPI2_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_SPI2_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_SPI2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_spi2_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_SPI2_INT_MAP_MASK) >> Self::CORE1_SPI2_INT_MAP_OFFSET
    }

    /// Write **CORE1_SPI2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_spi2_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_SPI2_INT_MAP_MASK)
                | ((value << Self::CORE1_SPI2_INT_MAP_OFFSET) & Self::CORE1_SPI2_INT_MAP_MASK)
        });
    }
}

/// SPI2_INT_MAP value type for batch read/modify/write operations
///
/// Use `spi2intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spi2intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spi2IntMapVal {
    bits: u32,
}

impl Spi2IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_SPI2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_spi2_int_map(&self) -> u32 {
        (self.bits & Spi2IntMap::CORE1_SPI2_INT_MAP_MASK) >> Spi2IntMap::CORE1_SPI2_INT_MAP_OFFSET
    }

    /// Set **CORE1_SPI2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_spi2_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spi2IntMap::CORE1_SPI2_INT_MAP_MASK)
                | ((value << Spi2IntMap::CORE1_SPI2_INT_MAP_OFFSET)
                    & Spi2IntMap::CORE1_SPI2_INT_MAP_MASK),
        }
    }
}

impl Default for Spi2IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Spi2IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spi2IntMapVal {
        Spi2IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spi2IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spi2IntMapVal) -> Spi2IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SPI3_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **CORE1_SPI3_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Spi3IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Spi3IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Spi3IntMap {
    // ======== Field Constants ========

    /// **CORE1_SPI3_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_SPI3_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_SPI3_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_SPI3_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_spi3_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_SPI3_INT_MAP_MASK) >> Self::CORE1_SPI3_INT_MAP_OFFSET
    }

    /// Write **CORE1_SPI3_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_spi3_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_SPI3_INT_MAP_MASK)
                | ((value << Self::CORE1_SPI3_INT_MAP_OFFSET) & Self::CORE1_SPI3_INT_MAP_MASK)
        });
    }
}

/// SPI3_INT_MAP value type for batch read/modify/write operations
///
/// Use `spi3intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `spi3intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Spi3IntMapVal {
    bits: u32,
}

impl Spi3IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_SPI3_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_spi3_int_map(&self) -> u32 {
        (self.bits & Spi3IntMap::CORE1_SPI3_INT_MAP_MASK) >> Spi3IntMap::CORE1_SPI3_INT_MAP_OFFSET
    }

    /// Set **CORE1_SPI3_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_spi3_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Spi3IntMap::CORE1_SPI3_INT_MAP_MASK)
                | ((value << Spi3IntMap::CORE1_SPI3_INT_MAP_OFFSET)
                    & Spi3IntMap::CORE1_SPI3_INT_MAP_MASK),
        }
    }
}

impl Default for Spi3IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Spi3IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Spi3IntMapVal {
        Spi3IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Spi3IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Spi3IntMapVal) -> Spi3IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I2S0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **CORE1_I2S0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct I2s0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I2s0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2s0IntMap {
    // ======== Field Constants ========

    /// **CORE1_I2S0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_I2S0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_I2S0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_I2S0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_i2s0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_I2S0_INT_MAP_MASK) >> Self::CORE1_I2S0_INT_MAP_OFFSET
    }

    /// Write **CORE1_I2S0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_i2s0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_I2S0_INT_MAP_MASK)
                | ((value << Self::CORE1_I2S0_INT_MAP_OFFSET) & Self::CORE1_I2S0_INT_MAP_MASK)
        });
    }
}

/// I2S0_INT_MAP value type for batch read/modify/write operations
///
/// Use `i2s0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2s0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2s0IntMapVal {
    bits: u32,
}

impl I2s0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_I2S0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_i2s0_int_map(&self) -> u32 {
        (self.bits & I2s0IntMap::CORE1_I2S0_INT_MAP_MASK) >> I2s0IntMap::CORE1_I2S0_INT_MAP_OFFSET
    }

    /// Set **CORE1_I2S0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_i2s0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2s0IntMap::CORE1_I2S0_INT_MAP_MASK)
                | ((value << I2s0IntMap::CORE1_I2S0_INT_MAP_OFFSET)
                    & I2s0IntMap::CORE1_I2S0_INT_MAP_MASK),
        }
    }
}

impl Default for I2s0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl I2s0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2s0IntMapVal {
        I2s0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2s0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2s0IntMapVal) -> I2s0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I2S1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **CORE1_I2S1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct I2s1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I2s1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2s1IntMap {
    // ======== Field Constants ========

    /// **CORE1_I2S1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_I2S1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_I2S1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_I2S1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_i2s1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_I2S1_INT_MAP_MASK) >> Self::CORE1_I2S1_INT_MAP_OFFSET
    }

    /// Write **CORE1_I2S1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_i2s1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_I2S1_INT_MAP_MASK)
                | ((value << Self::CORE1_I2S1_INT_MAP_OFFSET) & Self::CORE1_I2S1_INT_MAP_MASK)
        });
    }
}

/// I2S1_INT_MAP value type for batch read/modify/write operations
///
/// Use `i2s1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2s1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2s1IntMapVal {
    bits: u32,
}

impl I2s1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_I2S1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_i2s1_int_map(&self) -> u32 {
        (self.bits & I2s1IntMap::CORE1_I2S1_INT_MAP_MASK) >> I2s1IntMap::CORE1_I2S1_INT_MAP_OFFSET
    }

    /// Set **CORE1_I2S1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_i2s1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2s1IntMap::CORE1_I2S1_INT_MAP_MASK)
                | ((value << I2s1IntMap::CORE1_I2S1_INT_MAP_OFFSET)
                    & I2s1IntMap::CORE1_I2S1_INT_MAP_MASK),
        }
    }
}

impl Default for I2s1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl I2s1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2s1IntMapVal {
        I2s1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2s1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2s1IntMapVal) -> I2s1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I2S2_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **CORE1_I2S2_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct I2s2IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I2s2IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2s2IntMap {
    // ======== Field Constants ========

    /// **CORE1_I2S2_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_I2S2_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_I2S2_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_I2S2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_i2s2_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_I2S2_INT_MAP_MASK) >> Self::CORE1_I2S2_INT_MAP_OFFSET
    }

    /// Write **CORE1_I2S2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_i2s2_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_I2S2_INT_MAP_MASK)
                | ((value << Self::CORE1_I2S2_INT_MAP_OFFSET) & Self::CORE1_I2S2_INT_MAP_MASK)
        });
    }
}

/// I2S2_INT_MAP value type for batch read/modify/write operations
///
/// Use `i2s2intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2s2intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2s2IntMapVal {
    bits: u32,
}

impl I2s2IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_I2S2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_i2s2_int_map(&self) -> u32 {
        (self.bits & I2s2IntMap::CORE1_I2S2_INT_MAP_MASK) >> I2s2IntMap::CORE1_I2S2_INT_MAP_OFFSET
    }

    /// Set **CORE1_I2S2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_i2s2_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2s2IntMap::CORE1_I2S2_INT_MAP_MASK)
                | ((value << I2s2IntMap::CORE1_I2S2_INT_MAP_OFFSET)
                    & I2s2IntMap::CORE1_I2S2_INT_MAP_MASK),
        }
    }
}

impl Default for I2s2IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl I2s2IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2s2IntMapVal {
        I2s2IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2s2IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2s2IntMapVal) -> I2s2IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UHCI0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **CORE1_UHCI0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Uhci0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uhci0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uhci0IntMap {
    // ======== Field Constants ========

    /// **CORE1_UHCI0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_UHCI0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_UHCI0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_UHCI0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_uhci0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_UHCI0_INT_MAP_MASK)
            >> Self::CORE1_UHCI0_INT_MAP_OFFSET
    }

    /// Write **CORE1_UHCI0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_uhci0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_UHCI0_INT_MAP_MASK)
                | ((value << Self::CORE1_UHCI0_INT_MAP_OFFSET) & Self::CORE1_UHCI0_INT_MAP_MASK)
        });
    }
}

/// UHCI0_INT_MAP value type for batch read/modify/write operations
///
/// Use `uhci0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uhci0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uhci0IntMapVal {
    bits: u32,
}

impl Uhci0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_UHCI0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_uhci0_int_map(&self) -> u32 {
        (self.bits & Uhci0IntMap::CORE1_UHCI0_INT_MAP_MASK)
            >> Uhci0IntMap::CORE1_UHCI0_INT_MAP_OFFSET
    }

    /// Set **CORE1_UHCI0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_uhci0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uhci0IntMap::CORE1_UHCI0_INT_MAP_MASK)
                | ((value << Uhci0IntMap::CORE1_UHCI0_INT_MAP_OFFSET)
                    & Uhci0IntMap::CORE1_UHCI0_INT_MAP_MASK),
        }
    }
}

impl Default for Uhci0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Uhci0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uhci0IntMapVal {
        Uhci0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uhci0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uhci0IntMapVal) -> Uhci0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UART0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **CORE1_UART0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Uart0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uart0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uart0IntMap {
    // ======== Field Constants ========

    /// **CORE1_UART0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_UART0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_UART0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_UART0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_uart0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_UART0_INT_MAP_MASK)
            >> Self::CORE1_UART0_INT_MAP_OFFSET
    }

    /// Write **CORE1_UART0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_uart0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_UART0_INT_MAP_MASK)
                | ((value << Self::CORE1_UART0_INT_MAP_OFFSET) & Self::CORE1_UART0_INT_MAP_MASK)
        });
    }
}

/// UART0_INT_MAP value type for batch read/modify/write operations
///
/// Use `uart0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uart0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uart0IntMapVal {
    bits: u32,
}

impl Uart0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_UART0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_uart0_int_map(&self) -> u32 {
        (self.bits & Uart0IntMap::CORE1_UART0_INT_MAP_MASK)
            >> Uart0IntMap::CORE1_UART0_INT_MAP_OFFSET
    }

    /// Set **CORE1_UART0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_uart0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uart0IntMap::CORE1_UART0_INT_MAP_MASK)
                | ((value << Uart0IntMap::CORE1_UART0_INT_MAP_OFFSET)
                    & Uart0IntMap::CORE1_UART0_INT_MAP_MASK),
        }
    }
}

impl Default for Uart0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Uart0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uart0IntMapVal {
        Uart0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uart0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uart0IntMapVal) -> Uart0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UART1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0080
///
/// ## Fields
///
/// - **CORE1_UART1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Uart1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uart1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uart1IntMap {
    // ======== Field Constants ========

    /// **CORE1_UART1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_UART1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_UART1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_UART1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_uart1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_UART1_INT_MAP_MASK)
            >> Self::CORE1_UART1_INT_MAP_OFFSET
    }

    /// Write **CORE1_UART1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_uart1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_UART1_INT_MAP_MASK)
                | ((value << Self::CORE1_UART1_INT_MAP_OFFSET) & Self::CORE1_UART1_INT_MAP_MASK)
        });
    }
}

/// UART1_INT_MAP value type for batch read/modify/write operations
///
/// Use `uart1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uart1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uart1IntMapVal {
    bits: u32,
}

impl Uart1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_UART1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_uart1_int_map(&self) -> u32 {
        (self.bits & Uart1IntMap::CORE1_UART1_INT_MAP_MASK)
            >> Uart1IntMap::CORE1_UART1_INT_MAP_OFFSET
    }

    /// Set **CORE1_UART1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_uart1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uart1IntMap::CORE1_UART1_INT_MAP_MASK)
                | ((value << Uart1IntMap::CORE1_UART1_INT_MAP_OFFSET)
                    & Uart1IntMap::CORE1_UART1_INT_MAP_MASK),
        }
    }
}

impl Default for Uart1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Uart1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uart1IntMapVal {
        Uart1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uart1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uart1IntMapVal) -> Uart1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UART2_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **CORE1_UART2_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Uart2IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uart2IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uart2IntMap {
    // ======== Field Constants ========

    /// **CORE1_UART2_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_UART2_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_UART2_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_UART2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_uart2_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_UART2_INT_MAP_MASK)
            >> Self::CORE1_UART2_INT_MAP_OFFSET
    }

    /// Write **CORE1_UART2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_uart2_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_UART2_INT_MAP_MASK)
                | ((value << Self::CORE1_UART2_INT_MAP_OFFSET) & Self::CORE1_UART2_INT_MAP_MASK)
        });
    }
}

/// UART2_INT_MAP value type for batch read/modify/write operations
///
/// Use `uart2intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uart2intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uart2IntMapVal {
    bits: u32,
}

impl Uart2IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_UART2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_uart2_int_map(&self) -> u32 {
        (self.bits & Uart2IntMap::CORE1_UART2_INT_MAP_MASK)
            >> Uart2IntMap::CORE1_UART2_INT_MAP_OFFSET
    }

    /// Set **CORE1_UART2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_uart2_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uart2IntMap::CORE1_UART2_INT_MAP_MASK)
                | ((value << Uart2IntMap::CORE1_UART2_INT_MAP_OFFSET)
                    & Uart2IntMap::CORE1_UART2_INT_MAP_MASK),
        }
    }
}

impl Default for Uart2IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Uart2IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uart2IntMapVal {
        Uart2IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uart2IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uart2IntMapVal) -> Uart2IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UART3_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0088
///
/// ## Fields
///
/// - **CORE1_UART3_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Uart3IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uart3IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uart3IntMap {
    // ======== Field Constants ========

    /// **CORE1_UART3_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_UART3_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_UART3_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_UART3_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_uart3_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_UART3_INT_MAP_MASK)
            >> Self::CORE1_UART3_INT_MAP_OFFSET
    }

    /// Write **CORE1_UART3_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_uart3_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_UART3_INT_MAP_MASK)
                | ((value << Self::CORE1_UART3_INT_MAP_OFFSET) & Self::CORE1_UART3_INT_MAP_MASK)
        });
    }
}

/// UART3_INT_MAP value type for batch read/modify/write operations
///
/// Use `uart3intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uart3intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uart3IntMapVal {
    bits: u32,
}

impl Uart3IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_UART3_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_uart3_int_map(&self) -> u32 {
        (self.bits & Uart3IntMap::CORE1_UART3_INT_MAP_MASK)
            >> Uart3IntMap::CORE1_UART3_INT_MAP_OFFSET
    }

    /// Set **CORE1_UART3_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_uart3_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uart3IntMap::CORE1_UART3_INT_MAP_MASK)
                | ((value << Uart3IntMap::CORE1_UART3_INT_MAP_OFFSET)
                    & Uart3IntMap::CORE1_UART3_INT_MAP_MASK),
        }
    }
}

impl Default for Uart3IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Uart3IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uart3IntMapVal {
        Uart3IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uart3IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uart3IntMapVal) -> Uart3IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UART4_INT_MAP Register
///
/// NA
///
/// Address offset: 0x008c
///
/// ## Fields
///
/// - **CORE1_UART4_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Uart4IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Uart4IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Uart4IntMap {
    // ======== Field Constants ========

    /// **CORE1_UART4_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_UART4_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_UART4_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_UART4_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_uart4_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_UART4_INT_MAP_MASK)
            >> Self::CORE1_UART4_INT_MAP_OFFSET
    }

    /// Write **CORE1_UART4_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_uart4_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_UART4_INT_MAP_MASK)
                | ((value << Self::CORE1_UART4_INT_MAP_OFFSET) & Self::CORE1_UART4_INT_MAP_MASK)
        });
    }
}

/// UART4_INT_MAP value type for batch read/modify/write operations
///
/// Use `uart4intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uart4intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Uart4IntMapVal {
    bits: u32,
}

impl Uart4IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_UART4_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_uart4_int_map(&self) -> u32 {
        (self.bits & Uart4IntMap::CORE1_UART4_INT_MAP_MASK)
            >> Uart4IntMap::CORE1_UART4_INT_MAP_OFFSET
    }

    /// Set **CORE1_UART4_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_uart4_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Uart4IntMap::CORE1_UART4_INT_MAP_MASK)
                | ((value << Uart4IntMap::CORE1_UART4_INT_MAP_OFFSET)
                    & Uart4IntMap::CORE1_UART4_INT_MAP_MASK),
        }
    }
}

impl Default for Uart4IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Uart4IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Uart4IntMapVal {
        Uart4IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Uart4IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Uart4IntMapVal) -> Uart4IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LCD_CAM_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0090
///
/// ## Fields
///
/// - **CORE1_LCD_CAM_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LcdCamIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LcdCamIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LcdCamIntMap {
    // ======== Field Constants ========

    /// **CORE1_LCD_CAM_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LCD_CAM_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LCD_CAM_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LCD_CAM_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lcd_cam_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LCD_CAM_INT_MAP_MASK)
            >> Self::CORE1_LCD_CAM_INT_MAP_OFFSET
    }

    /// Write **CORE1_LCD_CAM_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lcd_cam_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LCD_CAM_INT_MAP_MASK)
                | ((value << Self::CORE1_LCD_CAM_INT_MAP_OFFSET) & Self::CORE1_LCD_CAM_INT_MAP_MASK)
        });
    }
}

/// LCD_CAM_INT_MAP value type for batch read/modify/write operations
///
/// Use `lcdcamintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lcdcamintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LcdCamIntMapVal {
    bits: u32,
}

impl LcdCamIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LCD_CAM_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lcd_cam_int_map(&self) -> u32 {
        (self.bits & LcdCamIntMap::CORE1_LCD_CAM_INT_MAP_MASK)
            >> LcdCamIntMap::CORE1_LCD_CAM_INT_MAP_OFFSET
    }

    /// Set **CORE1_LCD_CAM_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lcd_cam_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LcdCamIntMap::CORE1_LCD_CAM_INT_MAP_MASK)
                | ((value << LcdCamIntMap::CORE1_LCD_CAM_INT_MAP_OFFSET)
                    & LcdCamIntMap::CORE1_LCD_CAM_INT_MAP_MASK),
        }
    }
}

impl Default for LcdCamIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LcdCamIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LcdCamIntMapVal {
        LcdCamIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LcdCamIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LcdCamIntMapVal) -> LcdCamIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ADC_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0094
///
/// ## Fields
///
/// - **CORE1_ADC_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AdcIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AdcIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AdcIntMap {
    // ======== Field Constants ========

    /// **CORE1_ADC_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_ADC_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_ADC_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_ADC_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_adc_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_ADC_INT_MAP_MASK) >> Self::CORE1_ADC_INT_MAP_OFFSET
    }

    /// Write **CORE1_ADC_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_adc_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_ADC_INT_MAP_MASK)
                | ((value << Self::CORE1_ADC_INT_MAP_OFFSET) & Self::CORE1_ADC_INT_MAP_MASK)
        });
    }
}

/// ADC_INT_MAP value type for batch read/modify/write operations
///
/// Use `adcintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `adcintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AdcIntMapVal {
    bits: u32,
}

impl AdcIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_ADC_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_adc_int_map(&self) -> u32 {
        (self.bits & AdcIntMap::CORE1_ADC_INT_MAP_MASK) >> AdcIntMap::CORE1_ADC_INT_MAP_OFFSET
    }

    /// Set **CORE1_ADC_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_adc_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AdcIntMap::CORE1_ADC_INT_MAP_MASK)
                | ((value << AdcIntMap::CORE1_ADC_INT_MAP_OFFSET)
                    & AdcIntMap::CORE1_ADC_INT_MAP_MASK),
        }
    }
}

impl Default for AdcIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AdcIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AdcIntMapVal {
        AdcIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AdcIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AdcIntMapVal) -> AdcIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PWM0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0098
///
/// ## Fields
///
/// - **CORE1_PWM0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Pwm0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pwm0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pwm0IntMap {
    // ======== Field Constants ========

    /// **CORE1_PWM0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_PWM0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_PWM0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_PWM0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_pwm0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_PWM0_INT_MAP_MASK) >> Self::CORE1_PWM0_INT_MAP_OFFSET
    }

    /// Write **CORE1_PWM0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_pwm0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_PWM0_INT_MAP_MASK)
                | ((value << Self::CORE1_PWM0_INT_MAP_OFFSET) & Self::CORE1_PWM0_INT_MAP_MASK)
        });
    }
}

/// PWM0_INT_MAP value type for batch read/modify/write operations
///
/// Use `pwm0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pwm0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pwm0IntMapVal {
    bits: u32,
}

impl Pwm0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_PWM0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_pwm0_int_map(&self) -> u32 {
        (self.bits & Pwm0IntMap::CORE1_PWM0_INT_MAP_MASK) >> Pwm0IntMap::CORE1_PWM0_INT_MAP_OFFSET
    }

    /// Set **CORE1_PWM0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_pwm0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pwm0IntMap::CORE1_PWM0_INT_MAP_MASK)
                | ((value << Pwm0IntMap::CORE1_PWM0_INT_MAP_OFFSET)
                    & Pwm0IntMap::CORE1_PWM0_INT_MAP_MASK),
        }
    }
}

impl Default for Pwm0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pwm0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pwm0IntMapVal {
        Pwm0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pwm0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pwm0IntMapVal) -> Pwm0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PWM1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x009c
///
/// ## Fields
///
/// - **CORE1_PWM1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Pwm1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pwm1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pwm1IntMap {
    // ======== Field Constants ========

    /// **CORE1_PWM1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_PWM1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_PWM1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_PWM1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_pwm1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_PWM1_INT_MAP_MASK) >> Self::CORE1_PWM1_INT_MAP_OFFSET
    }

    /// Write **CORE1_PWM1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_pwm1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_PWM1_INT_MAP_MASK)
                | ((value << Self::CORE1_PWM1_INT_MAP_OFFSET) & Self::CORE1_PWM1_INT_MAP_MASK)
        });
    }
}

/// PWM1_INT_MAP value type for batch read/modify/write operations
///
/// Use `pwm1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pwm1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pwm1IntMapVal {
    bits: u32,
}

impl Pwm1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_PWM1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_pwm1_int_map(&self) -> u32 {
        (self.bits & Pwm1IntMap::CORE1_PWM1_INT_MAP_MASK) >> Pwm1IntMap::CORE1_PWM1_INT_MAP_OFFSET
    }

    /// Set **CORE1_PWM1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_pwm1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pwm1IntMap::CORE1_PWM1_INT_MAP_MASK)
                | ((value << Pwm1IntMap::CORE1_PWM1_INT_MAP_OFFSET)
                    & Pwm1IntMap::CORE1_PWM1_INT_MAP_MASK),
        }
    }
}

impl Default for Pwm1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Pwm1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pwm1IntMapVal {
        Pwm1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pwm1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pwm1IntMapVal) -> Pwm1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CAN0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00a0
///
/// ## Fields
///
/// - **CORE1_CAN0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Can0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Can0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Can0IntMap {
    // ======== Field Constants ========

    /// **CORE1_CAN0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_CAN0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_CAN0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_CAN0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_can0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_CAN0_INT_MAP_MASK) >> Self::CORE1_CAN0_INT_MAP_OFFSET
    }

    /// Write **CORE1_CAN0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_can0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_CAN0_INT_MAP_MASK)
                | ((value << Self::CORE1_CAN0_INT_MAP_OFFSET) & Self::CORE1_CAN0_INT_MAP_MASK)
        });
    }
}

/// CAN0_INT_MAP value type for batch read/modify/write operations
///
/// Use `can0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `can0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Can0IntMapVal {
    bits: u32,
}

impl Can0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_CAN0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_can0_int_map(&self) -> u32 {
        (self.bits & Can0IntMap::CORE1_CAN0_INT_MAP_MASK) >> Can0IntMap::CORE1_CAN0_INT_MAP_OFFSET
    }

    /// Set **CORE1_CAN0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_can0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Can0IntMap::CORE1_CAN0_INT_MAP_MASK)
                | ((value << Can0IntMap::CORE1_CAN0_INT_MAP_OFFSET)
                    & Can0IntMap::CORE1_CAN0_INT_MAP_MASK),
        }
    }
}

impl Default for Can0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Can0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Can0IntMapVal {
        Can0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Can0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Can0IntMapVal) -> Can0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CAN1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00a4
///
/// ## Fields
///
/// - **CORE1_CAN1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Can1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Can1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Can1IntMap {
    // ======== Field Constants ========

    /// **CORE1_CAN1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_CAN1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_CAN1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_CAN1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_can1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_CAN1_INT_MAP_MASK) >> Self::CORE1_CAN1_INT_MAP_OFFSET
    }

    /// Write **CORE1_CAN1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_can1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_CAN1_INT_MAP_MASK)
                | ((value << Self::CORE1_CAN1_INT_MAP_OFFSET) & Self::CORE1_CAN1_INT_MAP_MASK)
        });
    }
}

/// CAN1_INT_MAP value type for batch read/modify/write operations
///
/// Use `can1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `can1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Can1IntMapVal {
    bits: u32,
}

impl Can1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_CAN1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_can1_int_map(&self) -> u32 {
        (self.bits & Can1IntMap::CORE1_CAN1_INT_MAP_MASK) >> Can1IntMap::CORE1_CAN1_INT_MAP_OFFSET
    }

    /// Set **CORE1_CAN1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_can1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Can1IntMap::CORE1_CAN1_INT_MAP_MASK)
                | ((value << Can1IntMap::CORE1_CAN1_INT_MAP_OFFSET)
                    & Can1IntMap::CORE1_CAN1_INT_MAP_MASK),
        }
    }
}

impl Default for Can1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Can1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Can1IntMapVal {
        Can1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Can1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Can1IntMapVal) -> Can1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CAN2_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00a8
///
/// ## Fields
///
/// - **CORE1_CAN2_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Can2IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Can2IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Can2IntMap {
    // ======== Field Constants ========

    /// **CORE1_CAN2_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_CAN2_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_CAN2_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_CAN2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_can2_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_CAN2_INT_MAP_MASK) >> Self::CORE1_CAN2_INT_MAP_OFFSET
    }

    /// Write **CORE1_CAN2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_can2_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_CAN2_INT_MAP_MASK)
                | ((value << Self::CORE1_CAN2_INT_MAP_OFFSET) & Self::CORE1_CAN2_INT_MAP_MASK)
        });
    }
}

/// CAN2_INT_MAP value type for batch read/modify/write operations
///
/// Use `can2intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `can2intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Can2IntMapVal {
    bits: u32,
}

impl Can2IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_CAN2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_can2_int_map(&self) -> u32 {
        (self.bits & Can2IntMap::CORE1_CAN2_INT_MAP_MASK) >> Can2IntMap::CORE1_CAN2_INT_MAP_OFFSET
    }

    /// Set **CORE1_CAN2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_can2_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Can2IntMap::CORE1_CAN2_INT_MAP_MASK)
                | ((value << Can2IntMap::CORE1_CAN2_INT_MAP_OFFSET)
                    & Can2IntMap::CORE1_CAN2_INT_MAP_MASK),
        }
    }
}

impl Default for Can2IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Can2IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Can2IntMapVal {
        Can2IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Can2IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Can2IntMapVal) -> Can2IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RMT_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00ac
///
/// ## Fields
///
/// - **CORE1_RMT_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct RmtIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RmtIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RmtIntMap {
    // ======== Field Constants ========

    /// **CORE1_RMT_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_RMT_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_RMT_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_RMT_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_rmt_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_RMT_INT_MAP_MASK) >> Self::CORE1_RMT_INT_MAP_OFFSET
    }

    /// Write **CORE1_RMT_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_rmt_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_RMT_INT_MAP_MASK)
                | ((value << Self::CORE1_RMT_INT_MAP_OFFSET) & Self::CORE1_RMT_INT_MAP_MASK)
        });
    }
}

/// RMT_INT_MAP value type for batch read/modify/write operations
///
/// Use `rmtintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rmtintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RmtIntMapVal {
    bits: u32,
}

impl RmtIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_RMT_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_rmt_int_map(&self) -> u32 {
        (self.bits & RmtIntMap::CORE1_RMT_INT_MAP_MASK) >> RmtIntMap::CORE1_RMT_INT_MAP_OFFSET
    }

    /// Set **CORE1_RMT_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_rmt_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RmtIntMap::CORE1_RMT_INT_MAP_MASK)
                | ((value << RmtIntMap::CORE1_RMT_INT_MAP_OFFSET)
                    & RmtIntMap::CORE1_RMT_INT_MAP_MASK),
        }
    }
}

impl Default for RmtIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RmtIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RmtIntMapVal {
        RmtIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RmtIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RmtIntMapVal) -> RmtIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I2C0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00b0
///
/// ## Fields
///
/// - **CORE1_I2C0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct I2c0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I2c0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2c0IntMap {
    // ======== Field Constants ========

    /// **CORE1_I2C0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_I2C0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_I2C0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_I2C0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_i2c0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_I2C0_INT_MAP_MASK) >> Self::CORE1_I2C0_INT_MAP_OFFSET
    }

    /// Write **CORE1_I2C0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_i2c0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_I2C0_INT_MAP_MASK)
                | ((value << Self::CORE1_I2C0_INT_MAP_OFFSET) & Self::CORE1_I2C0_INT_MAP_MASK)
        });
    }
}

/// I2C0_INT_MAP value type for batch read/modify/write operations
///
/// Use `i2c0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2c0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2c0IntMapVal {
    bits: u32,
}

impl I2c0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_I2C0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_i2c0_int_map(&self) -> u32 {
        (self.bits & I2c0IntMap::CORE1_I2C0_INT_MAP_MASK) >> I2c0IntMap::CORE1_I2C0_INT_MAP_OFFSET
    }

    /// Set **CORE1_I2C0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_i2c0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2c0IntMap::CORE1_I2C0_INT_MAP_MASK)
                | ((value << I2c0IntMap::CORE1_I2C0_INT_MAP_OFFSET)
                    & I2c0IntMap::CORE1_I2C0_INT_MAP_MASK),
        }
    }
}

impl Default for I2c0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl I2c0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2c0IntMapVal {
        I2c0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2c0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2c0IntMapVal) -> I2c0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I2C1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00b4
///
/// ## Fields
///
/// - **CORE1_I2C1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct I2c1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I2c1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I2c1IntMap {
    // ======== Field Constants ========

    /// **CORE1_I2C1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_I2C1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_I2C1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_I2C1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_i2c1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_I2C1_INT_MAP_MASK) >> Self::CORE1_I2C1_INT_MAP_OFFSET
    }

    /// Write **CORE1_I2C1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_i2c1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_I2C1_INT_MAP_MASK)
                | ((value << Self::CORE1_I2C1_INT_MAP_OFFSET) & Self::CORE1_I2C1_INT_MAP_MASK)
        });
    }
}

/// I2C1_INT_MAP value type for batch read/modify/write operations
///
/// Use `i2c1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i2c1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I2c1IntMapVal {
    bits: u32,
}

impl I2c1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_I2C1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_i2c1_int_map(&self) -> u32 {
        (self.bits & I2c1IntMap::CORE1_I2C1_INT_MAP_MASK) >> I2c1IntMap::CORE1_I2C1_INT_MAP_OFFSET
    }

    /// Set **CORE1_I2C1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_i2c1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I2c1IntMap::CORE1_I2C1_INT_MAP_MASK)
                | ((value << I2c1IntMap::CORE1_I2C1_INT_MAP_OFFSET)
                    & I2c1IntMap::CORE1_I2C1_INT_MAP_MASK),
        }
    }
}

impl Default for I2c1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl I2c1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I2c1IntMapVal {
        I2c1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I2c1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I2c1IntMapVal) -> I2c1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMERGRP0_T0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00b8
///
/// ## Fields
///
/// - **CORE1_TIMERGRP0_T0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Timergrp0T0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timergrp0T0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timergrp0T0IntMap {
    // ======== Field Constants ========

    /// **CORE1_TIMERGRP0_T0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_TIMERGRP0_T0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_TIMERGRP0_T0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_TIMERGRP0_T0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_timergrp0_t0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_TIMERGRP0_T0_INT_MAP_MASK)
            >> Self::CORE1_TIMERGRP0_T0_INT_MAP_OFFSET
    }

    /// Write **CORE1_TIMERGRP0_T0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_timergrp0_t0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_TIMERGRP0_T0_INT_MAP_MASK)
                | ((value << Self::CORE1_TIMERGRP0_T0_INT_MAP_OFFSET)
                    & Self::CORE1_TIMERGRP0_T0_INT_MAP_MASK)
        });
    }
}

/// TIMERGRP0_T0_INT_MAP value type for batch read/modify/write operations
///
/// Use `timergrp0t0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timergrp0t0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timergrp0T0IntMapVal {
    bits: u32,
}

impl Timergrp0T0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_TIMERGRP0_T0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_timergrp0_t0_int_map(&self) -> u32 {
        (self.bits & Timergrp0T0IntMap::CORE1_TIMERGRP0_T0_INT_MAP_MASK)
            >> Timergrp0T0IntMap::CORE1_TIMERGRP0_T0_INT_MAP_OFFSET
    }

    /// Set **CORE1_TIMERGRP0_T0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_timergrp0_t0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timergrp0T0IntMap::CORE1_TIMERGRP0_T0_INT_MAP_MASK)
                | ((value << Timergrp0T0IntMap::CORE1_TIMERGRP0_T0_INT_MAP_OFFSET)
                    & Timergrp0T0IntMap::CORE1_TIMERGRP0_T0_INT_MAP_MASK),
        }
    }
}

impl Default for Timergrp0T0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timergrp0T0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timergrp0T0IntMapVal {
        Timergrp0T0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timergrp0T0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timergrp0T0IntMapVal) -> Timergrp0T0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMERGRP0_T1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00bc
///
/// ## Fields
///
/// - **CORE1_TIMERGRP0_T1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Timergrp0T1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timergrp0T1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timergrp0T1IntMap {
    // ======== Field Constants ========

    /// **CORE1_TIMERGRP0_T1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_TIMERGRP0_T1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_TIMERGRP0_T1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_TIMERGRP0_T1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_timergrp0_t1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_TIMERGRP0_T1_INT_MAP_MASK)
            >> Self::CORE1_TIMERGRP0_T1_INT_MAP_OFFSET
    }

    /// Write **CORE1_TIMERGRP0_T1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_timergrp0_t1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_TIMERGRP0_T1_INT_MAP_MASK)
                | ((value << Self::CORE1_TIMERGRP0_T1_INT_MAP_OFFSET)
                    & Self::CORE1_TIMERGRP0_T1_INT_MAP_MASK)
        });
    }
}

/// TIMERGRP0_T1_INT_MAP value type for batch read/modify/write operations
///
/// Use `timergrp0t1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timergrp0t1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timergrp0T1IntMapVal {
    bits: u32,
}

impl Timergrp0T1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_TIMERGRP0_T1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_timergrp0_t1_int_map(&self) -> u32 {
        (self.bits & Timergrp0T1IntMap::CORE1_TIMERGRP0_T1_INT_MAP_MASK)
            >> Timergrp0T1IntMap::CORE1_TIMERGRP0_T1_INT_MAP_OFFSET
    }

    /// Set **CORE1_TIMERGRP0_T1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_timergrp0_t1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timergrp0T1IntMap::CORE1_TIMERGRP0_T1_INT_MAP_MASK)
                | ((value << Timergrp0T1IntMap::CORE1_TIMERGRP0_T1_INT_MAP_OFFSET)
                    & Timergrp0T1IntMap::CORE1_TIMERGRP0_T1_INT_MAP_MASK),
        }
    }
}

impl Default for Timergrp0T1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timergrp0T1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timergrp0T1IntMapVal {
        Timergrp0T1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timergrp0T1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timergrp0T1IntMapVal) -> Timergrp0T1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMERGRP0_WDT_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00c0
///
/// ## Fields
///
/// - **CORE1_TIMERGRP0_WDT_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Timergrp0WdtIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timergrp0WdtIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timergrp0WdtIntMap {
    // ======== Field Constants ========

    /// **CORE1_TIMERGRP0_WDT_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_TIMERGRP0_WDT_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_TIMERGRP0_WDT_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_TIMERGRP0_WDT_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_timergrp0_wdt_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_TIMERGRP0_WDT_INT_MAP_MASK)
            >> Self::CORE1_TIMERGRP0_WDT_INT_MAP_OFFSET
    }

    /// Write **CORE1_TIMERGRP0_WDT_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_timergrp0_wdt_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_TIMERGRP0_WDT_INT_MAP_MASK)
                | ((value << Self::CORE1_TIMERGRP0_WDT_INT_MAP_OFFSET)
                    & Self::CORE1_TIMERGRP0_WDT_INT_MAP_MASK)
        });
    }
}

/// TIMERGRP0_WDT_INT_MAP value type for batch read/modify/write operations
///
/// Use `timergrp0wdtintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timergrp0wdtintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timergrp0WdtIntMapVal {
    bits: u32,
}

impl Timergrp0WdtIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_TIMERGRP0_WDT_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_timergrp0_wdt_int_map(&self) -> u32 {
        (self.bits & Timergrp0WdtIntMap::CORE1_TIMERGRP0_WDT_INT_MAP_MASK)
            >> Timergrp0WdtIntMap::CORE1_TIMERGRP0_WDT_INT_MAP_OFFSET
    }

    /// Set **CORE1_TIMERGRP0_WDT_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_timergrp0_wdt_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timergrp0WdtIntMap::CORE1_TIMERGRP0_WDT_INT_MAP_MASK)
                | ((value << Timergrp0WdtIntMap::CORE1_TIMERGRP0_WDT_INT_MAP_OFFSET)
                    & Timergrp0WdtIntMap::CORE1_TIMERGRP0_WDT_INT_MAP_MASK),
        }
    }
}

impl Default for Timergrp0WdtIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timergrp0WdtIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timergrp0WdtIntMapVal {
        Timergrp0WdtIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timergrp0WdtIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timergrp0WdtIntMapVal) -> Timergrp0WdtIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMERGRP1_T0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00c4
///
/// ## Fields
///
/// - **CORE1_TIMERGRP1_T0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Timergrp1T0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timergrp1T0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timergrp1T0IntMap {
    // ======== Field Constants ========

    /// **CORE1_TIMERGRP1_T0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_TIMERGRP1_T0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_TIMERGRP1_T0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_TIMERGRP1_T0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_timergrp1_t0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_TIMERGRP1_T0_INT_MAP_MASK)
            >> Self::CORE1_TIMERGRP1_T0_INT_MAP_OFFSET
    }

    /// Write **CORE1_TIMERGRP1_T0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_timergrp1_t0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_TIMERGRP1_T0_INT_MAP_MASK)
                | ((value << Self::CORE1_TIMERGRP1_T0_INT_MAP_OFFSET)
                    & Self::CORE1_TIMERGRP1_T0_INT_MAP_MASK)
        });
    }
}

/// TIMERGRP1_T0_INT_MAP value type for batch read/modify/write operations
///
/// Use `timergrp1t0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timergrp1t0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timergrp1T0IntMapVal {
    bits: u32,
}

impl Timergrp1T0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_TIMERGRP1_T0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_timergrp1_t0_int_map(&self) -> u32 {
        (self.bits & Timergrp1T0IntMap::CORE1_TIMERGRP1_T0_INT_MAP_MASK)
            >> Timergrp1T0IntMap::CORE1_TIMERGRP1_T0_INT_MAP_OFFSET
    }

    /// Set **CORE1_TIMERGRP1_T0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_timergrp1_t0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timergrp1T0IntMap::CORE1_TIMERGRP1_T0_INT_MAP_MASK)
                | ((value << Timergrp1T0IntMap::CORE1_TIMERGRP1_T0_INT_MAP_OFFSET)
                    & Timergrp1T0IntMap::CORE1_TIMERGRP1_T0_INT_MAP_MASK),
        }
    }
}

impl Default for Timergrp1T0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timergrp1T0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timergrp1T0IntMapVal {
        Timergrp1T0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timergrp1T0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timergrp1T0IntMapVal) -> Timergrp1T0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMERGRP1_T1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **CORE1_TIMERGRP1_T1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Timergrp1T1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timergrp1T1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timergrp1T1IntMap {
    // ======== Field Constants ========

    /// **CORE1_TIMERGRP1_T1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_TIMERGRP1_T1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_TIMERGRP1_T1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_TIMERGRP1_T1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_timergrp1_t1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_TIMERGRP1_T1_INT_MAP_MASK)
            >> Self::CORE1_TIMERGRP1_T1_INT_MAP_OFFSET
    }

    /// Write **CORE1_TIMERGRP1_T1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_timergrp1_t1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_TIMERGRP1_T1_INT_MAP_MASK)
                | ((value << Self::CORE1_TIMERGRP1_T1_INT_MAP_OFFSET)
                    & Self::CORE1_TIMERGRP1_T1_INT_MAP_MASK)
        });
    }
}

/// TIMERGRP1_T1_INT_MAP value type for batch read/modify/write operations
///
/// Use `timergrp1t1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timergrp1t1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timergrp1T1IntMapVal {
    bits: u32,
}

impl Timergrp1T1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_TIMERGRP1_T1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_timergrp1_t1_int_map(&self) -> u32 {
        (self.bits & Timergrp1T1IntMap::CORE1_TIMERGRP1_T1_INT_MAP_MASK)
            >> Timergrp1T1IntMap::CORE1_TIMERGRP1_T1_INT_MAP_OFFSET
    }

    /// Set **CORE1_TIMERGRP1_T1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_timergrp1_t1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timergrp1T1IntMap::CORE1_TIMERGRP1_T1_INT_MAP_MASK)
                | ((value << Timergrp1T1IntMap::CORE1_TIMERGRP1_T1_INT_MAP_OFFSET)
                    & Timergrp1T1IntMap::CORE1_TIMERGRP1_T1_INT_MAP_MASK),
        }
    }
}

impl Default for Timergrp1T1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timergrp1T1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timergrp1T1IntMapVal {
        Timergrp1T1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timergrp1T1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timergrp1T1IntMapVal) -> Timergrp1T1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TIMERGRP1_WDT_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00cc
///
/// ## Fields
///
/// - **CORE1_TIMERGRP1_WDT_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Timergrp1WdtIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Timergrp1WdtIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Timergrp1WdtIntMap {
    // ======== Field Constants ========

    /// **CORE1_TIMERGRP1_WDT_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_TIMERGRP1_WDT_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_TIMERGRP1_WDT_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_TIMERGRP1_WDT_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_timergrp1_wdt_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_TIMERGRP1_WDT_INT_MAP_MASK)
            >> Self::CORE1_TIMERGRP1_WDT_INT_MAP_OFFSET
    }

    /// Write **CORE1_TIMERGRP1_WDT_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_timergrp1_wdt_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_TIMERGRP1_WDT_INT_MAP_MASK)
                | ((value << Self::CORE1_TIMERGRP1_WDT_INT_MAP_OFFSET)
                    & Self::CORE1_TIMERGRP1_WDT_INT_MAP_MASK)
        });
    }
}

/// TIMERGRP1_WDT_INT_MAP value type for batch read/modify/write operations
///
/// Use `timergrp1wdtintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `timergrp1wdtintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Timergrp1WdtIntMapVal {
    bits: u32,
}

impl Timergrp1WdtIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_TIMERGRP1_WDT_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_timergrp1_wdt_int_map(&self) -> u32 {
        (self.bits & Timergrp1WdtIntMap::CORE1_TIMERGRP1_WDT_INT_MAP_MASK)
            >> Timergrp1WdtIntMap::CORE1_TIMERGRP1_WDT_INT_MAP_OFFSET
    }

    /// Set **CORE1_TIMERGRP1_WDT_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_timergrp1_wdt_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Timergrp1WdtIntMap::CORE1_TIMERGRP1_WDT_INT_MAP_MASK)
                | ((value << Timergrp1WdtIntMap::CORE1_TIMERGRP1_WDT_INT_MAP_OFFSET)
                    & Timergrp1WdtIntMap::CORE1_TIMERGRP1_WDT_INT_MAP_MASK),
        }
    }
}

impl Default for Timergrp1WdtIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Timergrp1WdtIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Timergrp1WdtIntMapVal {
        Timergrp1WdtIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Timergrp1WdtIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Timergrp1WdtIntMapVal) -> Timergrp1WdtIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LEDC_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00d0
///
/// ## Fields
///
/// - **CORE1_LEDC_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LedcIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LedcIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LedcIntMap {
    // ======== Field Constants ========

    /// **CORE1_LEDC_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LEDC_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LEDC_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LEDC_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_ledc_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LEDC_INT_MAP_MASK) >> Self::CORE1_LEDC_INT_MAP_OFFSET
    }

    /// Write **CORE1_LEDC_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_ledc_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LEDC_INT_MAP_MASK)
                | ((value << Self::CORE1_LEDC_INT_MAP_OFFSET) & Self::CORE1_LEDC_INT_MAP_MASK)
        });
    }
}

/// LEDC_INT_MAP value type for batch read/modify/write operations
///
/// Use `ledcintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ledcintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LedcIntMapVal {
    bits: u32,
}

impl LedcIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LEDC_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_ledc_int_map(&self) -> u32 {
        (self.bits & LedcIntMap::CORE1_LEDC_INT_MAP_MASK) >> LedcIntMap::CORE1_LEDC_INT_MAP_OFFSET
    }

    /// Set **CORE1_LEDC_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_ledc_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LedcIntMap::CORE1_LEDC_INT_MAP_MASK)
                | ((value << LedcIntMap::CORE1_LEDC_INT_MAP_OFFSET)
                    & LedcIntMap::CORE1_LEDC_INT_MAP_MASK),
        }
    }
}

impl Default for LedcIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LedcIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LedcIntMapVal {
        LedcIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LedcIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LedcIntMapVal) -> LedcIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SYSTIMER_TARGET0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00d4
///
/// ## Fields
///
/// - **CORE1_SYSTIMER_TARGET0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct SystimerTarget0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SystimerTarget0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SystimerTarget0IntMap {
    // ======== Field Constants ========

    /// **CORE1_SYSTIMER_TARGET0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_SYSTIMER_TARGET0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_SYSTIMER_TARGET0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_SYSTIMER_TARGET0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_systimer_target0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_SYSTIMER_TARGET0_INT_MAP_MASK)
            >> Self::CORE1_SYSTIMER_TARGET0_INT_MAP_OFFSET
    }

    /// Write **CORE1_SYSTIMER_TARGET0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_systimer_target0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_SYSTIMER_TARGET0_INT_MAP_MASK)
                | ((value << Self::CORE1_SYSTIMER_TARGET0_INT_MAP_OFFSET)
                    & Self::CORE1_SYSTIMER_TARGET0_INT_MAP_MASK)
        });
    }
}

/// SYSTIMER_TARGET0_INT_MAP value type for batch read/modify/write operations
///
/// Use `systimertarget0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `systimertarget0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SystimerTarget0IntMapVal {
    bits: u32,
}

impl SystimerTarget0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_SYSTIMER_TARGET0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_systimer_target0_int_map(&self) -> u32 {
        (self.bits & SystimerTarget0IntMap::CORE1_SYSTIMER_TARGET0_INT_MAP_MASK)
            >> SystimerTarget0IntMap::CORE1_SYSTIMER_TARGET0_INT_MAP_OFFSET
    }

    /// Set **CORE1_SYSTIMER_TARGET0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_systimer_target0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SystimerTarget0IntMap::CORE1_SYSTIMER_TARGET0_INT_MAP_MASK)
                | ((value << SystimerTarget0IntMap::CORE1_SYSTIMER_TARGET0_INT_MAP_OFFSET)
                    & SystimerTarget0IntMap::CORE1_SYSTIMER_TARGET0_INT_MAP_MASK),
        }
    }
}

impl Default for SystimerTarget0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SystimerTarget0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SystimerTarget0IntMapVal {
        SystimerTarget0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SystimerTarget0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SystimerTarget0IntMapVal) -> SystimerTarget0IntMapVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// SYSTIMER_TARGET1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00d8
///
/// ## Fields
///
/// - **CORE1_SYSTIMER_TARGET1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct SystimerTarget1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SystimerTarget1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SystimerTarget1IntMap {
    // ======== Field Constants ========

    /// **CORE1_SYSTIMER_TARGET1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_SYSTIMER_TARGET1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_SYSTIMER_TARGET1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_SYSTIMER_TARGET1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_systimer_target1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_SYSTIMER_TARGET1_INT_MAP_MASK)
            >> Self::CORE1_SYSTIMER_TARGET1_INT_MAP_OFFSET
    }

    /// Write **CORE1_SYSTIMER_TARGET1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_systimer_target1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_SYSTIMER_TARGET1_INT_MAP_MASK)
                | ((value << Self::CORE1_SYSTIMER_TARGET1_INT_MAP_OFFSET)
                    & Self::CORE1_SYSTIMER_TARGET1_INT_MAP_MASK)
        });
    }
}

/// SYSTIMER_TARGET1_INT_MAP value type for batch read/modify/write operations
///
/// Use `systimertarget1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `systimertarget1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SystimerTarget1IntMapVal {
    bits: u32,
}

impl SystimerTarget1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_SYSTIMER_TARGET1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_systimer_target1_int_map(&self) -> u32 {
        (self.bits & SystimerTarget1IntMap::CORE1_SYSTIMER_TARGET1_INT_MAP_MASK)
            >> SystimerTarget1IntMap::CORE1_SYSTIMER_TARGET1_INT_MAP_OFFSET
    }

    /// Set **CORE1_SYSTIMER_TARGET1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_systimer_target1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SystimerTarget1IntMap::CORE1_SYSTIMER_TARGET1_INT_MAP_MASK)
                | ((value << SystimerTarget1IntMap::CORE1_SYSTIMER_TARGET1_INT_MAP_OFFSET)
                    & SystimerTarget1IntMap::CORE1_SYSTIMER_TARGET1_INT_MAP_MASK),
        }
    }
}

impl Default for SystimerTarget1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SystimerTarget1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SystimerTarget1IntMapVal {
        SystimerTarget1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SystimerTarget1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SystimerTarget1IntMapVal) -> SystimerTarget1IntMapVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// SYSTIMER_TARGET2_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00dc
///
/// ## Fields
///
/// - **CORE1_SYSTIMER_TARGET2_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct SystimerTarget2IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SystimerTarget2IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SystimerTarget2IntMap {
    // ======== Field Constants ========

    /// **CORE1_SYSTIMER_TARGET2_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_SYSTIMER_TARGET2_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_SYSTIMER_TARGET2_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_SYSTIMER_TARGET2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_systimer_target2_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_SYSTIMER_TARGET2_INT_MAP_MASK)
            >> Self::CORE1_SYSTIMER_TARGET2_INT_MAP_OFFSET
    }

    /// Write **CORE1_SYSTIMER_TARGET2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_systimer_target2_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_SYSTIMER_TARGET2_INT_MAP_MASK)
                | ((value << Self::CORE1_SYSTIMER_TARGET2_INT_MAP_OFFSET)
                    & Self::CORE1_SYSTIMER_TARGET2_INT_MAP_MASK)
        });
    }
}

/// SYSTIMER_TARGET2_INT_MAP value type for batch read/modify/write operations
///
/// Use `systimertarget2intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `systimertarget2intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SystimerTarget2IntMapVal {
    bits: u32,
}

impl SystimerTarget2IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_SYSTIMER_TARGET2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_systimer_target2_int_map(&self) -> u32 {
        (self.bits & SystimerTarget2IntMap::CORE1_SYSTIMER_TARGET2_INT_MAP_MASK)
            >> SystimerTarget2IntMap::CORE1_SYSTIMER_TARGET2_INT_MAP_OFFSET
    }

    /// Set **CORE1_SYSTIMER_TARGET2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_systimer_target2_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SystimerTarget2IntMap::CORE1_SYSTIMER_TARGET2_INT_MAP_MASK)
                | ((value << SystimerTarget2IntMap::CORE1_SYSTIMER_TARGET2_INT_MAP_OFFSET)
                    & SystimerTarget2IntMap::CORE1_SYSTIMER_TARGET2_INT_MAP_MASK),
        }
    }
}

impl Default for SystimerTarget2IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SystimerTarget2IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SystimerTarget2IntMapVal {
        SystimerTarget2IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SystimerTarget2IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SystimerTarget2IntMapVal) -> SystimerTarget2IntMapVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// AHB_PDMA_IN_CH0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00e0
///
/// ## Fields
///
/// - **CORE1_AHB_PDMA_IN_CH0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AhbPdmaInCh0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AhbPdmaInCh0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AhbPdmaInCh0IntMap {
    // ======== Field Constants ========

    /// **CORE1_AHB_PDMA_IN_CH0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_AHB_PDMA_IN_CH0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_AHB_PDMA_IN_CH0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_AHB_PDMA_IN_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_ahb_pdma_in_ch0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_AHB_PDMA_IN_CH0_INT_MAP_MASK)
            >> Self::CORE1_AHB_PDMA_IN_CH0_INT_MAP_OFFSET
    }

    /// Write **CORE1_AHB_PDMA_IN_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_ahb_pdma_in_ch0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_AHB_PDMA_IN_CH0_INT_MAP_MASK)
                | ((value << Self::CORE1_AHB_PDMA_IN_CH0_INT_MAP_OFFSET)
                    & Self::CORE1_AHB_PDMA_IN_CH0_INT_MAP_MASK)
        });
    }
}

/// AHB_PDMA_IN_CH0_INT_MAP value type for batch read/modify/write operations
///
/// Use `ahbpdmainch0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ahbpdmainch0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AhbPdmaInCh0IntMapVal {
    bits: u32,
}

impl AhbPdmaInCh0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_AHB_PDMA_IN_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_ahb_pdma_in_ch0_int_map(&self) -> u32 {
        (self.bits & AhbPdmaInCh0IntMap::CORE1_AHB_PDMA_IN_CH0_INT_MAP_MASK)
            >> AhbPdmaInCh0IntMap::CORE1_AHB_PDMA_IN_CH0_INT_MAP_OFFSET
    }

    /// Set **CORE1_AHB_PDMA_IN_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_ahb_pdma_in_ch0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AhbPdmaInCh0IntMap::CORE1_AHB_PDMA_IN_CH0_INT_MAP_MASK)
                | ((value << AhbPdmaInCh0IntMap::CORE1_AHB_PDMA_IN_CH0_INT_MAP_OFFSET)
                    & AhbPdmaInCh0IntMap::CORE1_AHB_PDMA_IN_CH0_INT_MAP_MASK),
        }
    }
}

impl Default for AhbPdmaInCh0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AhbPdmaInCh0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AhbPdmaInCh0IntMapVal {
        AhbPdmaInCh0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AhbPdmaInCh0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AhbPdmaInCh0IntMapVal) -> AhbPdmaInCh0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AHB_PDMA_IN_CH1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00e4
///
/// ## Fields
///
/// - **CORE1_AHB_PDMA_IN_CH1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AhbPdmaInCh1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AhbPdmaInCh1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AhbPdmaInCh1IntMap {
    // ======== Field Constants ========

    /// **CORE1_AHB_PDMA_IN_CH1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_AHB_PDMA_IN_CH1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_AHB_PDMA_IN_CH1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_AHB_PDMA_IN_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_ahb_pdma_in_ch1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_AHB_PDMA_IN_CH1_INT_MAP_MASK)
            >> Self::CORE1_AHB_PDMA_IN_CH1_INT_MAP_OFFSET
    }

    /// Write **CORE1_AHB_PDMA_IN_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_ahb_pdma_in_ch1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_AHB_PDMA_IN_CH1_INT_MAP_MASK)
                | ((value << Self::CORE1_AHB_PDMA_IN_CH1_INT_MAP_OFFSET)
                    & Self::CORE1_AHB_PDMA_IN_CH1_INT_MAP_MASK)
        });
    }
}

/// AHB_PDMA_IN_CH1_INT_MAP value type for batch read/modify/write operations
///
/// Use `ahbpdmainch1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ahbpdmainch1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AhbPdmaInCh1IntMapVal {
    bits: u32,
}

impl AhbPdmaInCh1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_AHB_PDMA_IN_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_ahb_pdma_in_ch1_int_map(&self) -> u32 {
        (self.bits & AhbPdmaInCh1IntMap::CORE1_AHB_PDMA_IN_CH1_INT_MAP_MASK)
            >> AhbPdmaInCh1IntMap::CORE1_AHB_PDMA_IN_CH1_INT_MAP_OFFSET
    }

    /// Set **CORE1_AHB_PDMA_IN_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_ahb_pdma_in_ch1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AhbPdmaInCh1IntMap::CORE1_AHB_PDMA_IN_CH1_INT_MAP_MASK)
                | ((value << AhbPdmaInCh1IntMap::CORE1_AHB_PDMA_IN_CH1_INT_MAP_OFFSET)
                    & AhbPdmaInCh1IntMap::CORE1_AHB_PDMA_IN_CH1_INT_MAP_MASK),
        }
    }
}

impl Default for AhbPdmaInCh1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AhbPdmaInCh1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AhbPdmaInCh1IntMapVal {
        AhbPdmaInCh1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AhbPdmaInCh1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AhbPdmaInCh1IntMapVal) -> AhbPdmaInCh1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AHB_PDMA_IN_CH2_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00e8
///
/// ## Fields
///
/// - **CORE1_AHB_PDMA_IN_CH2_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AhbPdmaInCh2IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AhbPdmaInCh2IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AhbPdmaInCh2IntMap {
    // ======== Field Constants ========

    /// **CORE1_AHB_PDMA_IN_CH2_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_AHB_PDMA_IN_CH2_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_AHB_PDMA_IN_CH2_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_AHB_PDMA_IN_CH2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_ahb_pdma_in_ch2_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_AHB_PDMA_IN_CH2_INT_MAP_MASK)
            >> Self::CORE1_AHB_PDMA_IN_CH2_INT_MAP_OFFSET
    }

    /// Write **CORE1_AHB_PDMA_IN_CH2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_ahb_pdma_in_ch2_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_AHB_PDMA_IN_CH2_INT_MAP_MASK)
                | ((value << Self::CORE1_AHB_PDMA_IN_CH2_INT_MAP_OFFSET)
                    & Self::CORE1_AHB_PDMA_IN_CH2_INT_MAP_MASK)
        });
    }
}

/// AHB_PDMA_IN_CH2_INT_MAP value type for batch read/modify/write operations
///
/// Use `ahbpdmainch2intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ahbpdmainch2intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AhbPdmaInCh2IntMapVal {
    bits: u32,
}

impl AhbPdmaInCh2IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_AHB_PDMA_IN_CH2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_ahb_pdma_in_ch2_int_map(&self) -> u32 {
        (self.bits & AhbPdmaInCh2IntMap::CORE1_AHB_PDMA_IN_CH2_INT_MAP_MASK)
            >> AhbPdmaInCh2IntMap::CORE1_AHB_PDMA_IN_CH2_INT_MAP_OFFSET
    }

    /// Set **CORE1_AHB_PDMA_IN_CH2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_ahb_pdma_in_ch2_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AhbPdmaInCh2IntMap::CORE1_AHB_PDMA_IN_CH2_INT_MAP_MASK)
                | ((value << AhbPdmaInCh2IntMap::CORE1_AHB_PDMA_IN_CH2_INT_MAP_OFFSET)
                    & AhbPdmaInCh2IntMap::CORE1_AHB_PDMA_IN_CH2_INT_MAP_MASK),
        }
    }
}

impl Default for AhbPdmaInCh2IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AhbPdmaInCh2IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AhbPdmaInCh2IntMapVal {
        AhbPdmaInCh2IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AhbPdmaInCh2IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AhbPdmaInCh2IntMapVal) -> AhbPdmaInCh2IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AHB_PDMA_OUT_CH0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00ec
///
/// ## Fields
///
/// - **CORE1_AHB_PDMA_OUT_CH0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AhbPdmaOutCh0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AhbPdmaOutCh0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AhbPdmaOutCh0IntMap {
    // ======== Field Constants ========

    /// **CORE1_AHB_PDMA_OUT_CH0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_AHB_PDMA_OUT_CH0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_AHB_PDMA_OUT_CH0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_AHB_PDMA_OUT_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_ahb_pdma_out_ch0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_AHB_PDMA_OUT_CH0_INT_MAP_MASK)
            >> Self::CORE1_AHB_PDMA_OUT_CH0_INT_MAP_OFFSET
    }

    /// Write **CORE1_AHB_PDMA_OUT_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_ahb_pdma_out_ch0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_AHB_PDMA_OUT_CH0_INT_MAP_MASK)
                | ((value << Self::CORE1_AHB_PDMA_OUT_CH0_INT_MAP_OFFSET)
                    & Self::CORE1_AHB_PDMA_OUT_CH0_INT_MAP_MASK)
        });
    }
}

/// AHB_PDMA_OUT_CH0_INT_MAP value type for batch read/modify/write operations
///
/// Use `ahbpdmaoutch0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ahbpdmaoutch0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AhbPdmaOutCh0IntMapVal {
    bits: u32,
}

impl AhbPdmaOutCh0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_AHB_PDMA_OUT_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_ahb_pdma_out_ch0_int_map(&self) -> u32 {
        (self.bits & AhbPdmaOutCh0IntMap::CORE1_AHB_PDMA_OUT_CH0_INT_MAP_MASK)
            >> AhbPdmaOutCh0IntMap::CORE1_AHB_PDMA_OUT_CH0_INT_MAP_OFFSET
    }

    /// Set **CORE1_AHB_PDMA_OUT_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_ahb_pdma_out_ch0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AhbPdmaOutCh0IntMap::CORE1_AHB_PDMA_OUT_CH0_INT_MAP_MASK)
                | ((value << AhbPdmaOutCh0IntMap::CORE1_AHB_PDMA_OUT_CH0_INT_MAP_OFFSET)
                    & AhbPdmaOutCh0IntMap::CORE1_AHB_PDMA_OUT_CH0_INT_MAP_MASK),
        }
    }
}

impl Default for AhbPdmaOutCh0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AhbPdmaOutCh0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AhbPdmaOutCh0IntMapVal {
        AhbPdmaOutCh0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AhbPdmaOutCh0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AhbPdmaOutCh0IntMapVal) -> AhbPdmaOutCh0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AHB_PDMA_OUT_CH1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00f0
///
/// ## Fields
///
/// - **CORE1_AHB_PDMA_OUT_CH1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AhbPdmaOutCh1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AhbPdmaOutCh1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AhbPdmaOutCh1IntMap {
    // ======== Field Constants ========

    /// **CORE1_AHB_PDMA_OUT_CH1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_AHB_PDMA_OUT_CH1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_AHB_PDMA_OUT_CH1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_AHB_PDMA_OUT_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_ahb_pdma_out_ch1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_AHB_PDMA_OUT_CH1_INT_MAP_MASK)
            >> Self::CORE1_AHB_PDMA_OUT_CH1_INT_MAP_OFFSET
    }

    /// Write **CORE1_AHB_PDMA_OUT_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_ahb_pdma_out_ch1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_AHB_PDMA_OUT_CH1_INT_MAP_MASK)
                | ((value << Self::CORE1_AHB_PDMA_OUT_CH1_INT_MAP_OFFSET)
                    & Self::CORE1_AHB_PDMA_OUT_CH1_INT_MAP_MASK)
        });
    }
}

/// AHB_PDMA_OUT_CH1_INT_MAP value type for batch read/modify/write operations
///
/// Use `ahbpdmaoutch1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ahbpdmaoutch1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AhbPdmaOutCh1IntMapVal {
    bits: u32,
}

impl AhbPdmaOutCh1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_AHB_PDMA_OUT_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_ahb_pdma_out_ch1_int_map(&self) -> u32 {
        (self.bits & AhbPdmaOutCh1IntMap::CORE1_AHB_PDMA_OUT_CH1_INT_MAP_MASK)
            >> AhbPdmaOutCh1IntMap::CORE1_AHB_PDMA_OUT_CH1_INT_MAP_OFFSET
    }

    /// Set **CORE1_AHB_PDMA_OUT_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_ahb_pdma_out_ch1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AhbPdmaOutCh1IntMap::CORE1_AHB_PDMA_OUT_CH1_INT_MAP_MASK)
                | ((value << AhbPdmaOutCh1IntMap::CORE1_AHB_PDMA_OUT_CH1_INT_MAP_OFFSET)
                    & AhbPdmaOutCh1IntMap::CORE1_AHB_PDMA_OUT_CH1_INT_MAP_MASK),
        }
    }
}

impl Default for AhbPdmaOutCh1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AhbPdmaOutCh1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AhbPdmaOutCh1IntMapVal {
        AhbPdmaOutCh1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AhbPdmaOutCh1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AhbPdmaOutCh1IntMapVal) -> AhbPdmaOutCh1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AHB_PDMA_OUT_CH2_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00f4
///
/// ## Fields
///
/// - **CORE1_AHB_PDMA_OUT_CH2_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AhbPdmaOutCh2IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AhbPdmaOutCh2IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AhbPdmaOutCh2IntMap {
    // ======== Field Constants ========

    /// **CORE1_AHB_PDMA_OUT_CH2_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_AHB_PDMA_OUT_CH2_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_AHB_PDMA_OUT_CH2_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_AHB_PDMA_OUT_CH2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_ahb_pdma_out_ch2_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_AHB_PDMA_OUT_CH2_INT_MAP_MASK)
            >> Self::CORE1_AHB_PDMA_OUT_CH2_INT_MAP_OFFSET
    }

    /// Write **CORE1_AHB_PDMA_OUT_CH2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_ahb_pdma_out_ch2_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_AHB_PDMA_OUT_CH2_INT_MAP_MASK)
                | ((value << Self::CORE1_AHB_PDMA_OUT_CH2_INT_MAP_OFFSET)
                    & Self::CORE1_AHB_PDMA_OUT_CH2_INT_MAP_MASK)
        });
    }
}

/// AHB_PDMA_OUT_CH2_INT_MAP value type for batch read/modify/write operations
///
/// Use `ahbpdmaoutch2intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ahbpdmaoutch2intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AhbPdmaOutCh2IntMapVal {
    bits: u32,
}

impl AhbPdmaOutCh2IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_AHB_PDMA_OUT_CH2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_ahb_pdma_out_ch2_int_map(&self) -> u32 {
        (self.bits & AhbPdmaOutCh2IntMap::CORE1_AHB_PDMA_OUT_CH2_INT_MAP_MASK)
            >> AhbPdmaOutCh2IntMap::CORE1_AHB_PDMA_OUT_CH2_INT_MAP_OFFSET
    }

    /// Set **CORE1_AHB_PDMA_OUT_CH2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_ahb_pdma_out_ch2_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AhbPdmaOutCh2IntMap::CORE1_AHB_PDMA_OUT_CH2_INT_MAP_MASK)
                | ((value << AhbPdmaOutCh2IntMap::CORE1_AHB_PDMA_OUT_CH2_INT_MAP_OFFSET)
                    & AhbPdmaOutCh2IntMap::CORE1_AHB_PDMA_OUT_CH2_INT_MAP_MASK),
        }
    }
}

impl Default for AhbPdmaOutCh2IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AhbPdmaOutCh2IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AhbPdmaOutCh2IntMapVal {
        AhbPdmaOutCh2IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AhbPdmaOutCh2IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AhbPdmaOutCh2IntMapVal) -> AhbPdmaOutCh2IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AXI_PDMA_IN_CH0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00f8
///
/// ## Fields
///
/// - **CORE1_AXI_PDMA_IN_CH0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AxiPdmaInCh0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AxiPdmaInCh0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AxiPdmaInCh0IntMap {
    // ======== Field Constants ========

    /// **CORE1_AXI_PDMA_IN_CH0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_AXI_PDMA_IN_CH0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_AXI_PDMA_IN_CH0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_AXI_PDMA_IN_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_axi_pdma_in_ch0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_AXI_PDMA_IN_CH0_INT_MAP_MASK)
            >> Self::CORE1_AXI_PDMA_IN_CH0_INT_MAP_OFFSET
    }

    /// Write **CORE1_AXI_PDMA_IN_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_axi_pdma_in_ch0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_AXI_PDMA_IN_CH0_INT_MAP_MASK)
                | ((value << Self::CORE1_AXI_PDMA_IN_CH0_INT_MAP_OFFSET)
                    & Self::CORE1_AXI_PDMA_IN_CH0_INT_MAP_MASK)
        });
    }
}

/// AXI_PDMA_IN_CH0_INT_MAP value type for batch read/modify/write operations
///
/// Use `axipdmainch0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `axipdmainch0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AxiPdmaInCh0IntMapVal {
    bits: u32,
}

impl AxiPdmaInCh0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_AXI_PDMA_IN_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_axi_pdma_in_ch0_int_map(&self) -> u32 {
        (self.bits & AxiPdmaInCh0IntMap::CORE1_AXI_PDMA_IN_CH0_INT_MAP_MASK)
            >> AxiPdmaInCh0IntMap::CORE1_AXI_PDMA_IN_CH0_INT_MAP_OFFSET
    }

    /// Set **CORE1_AXI_PDMA_IN_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_axi_pdma_in_ch0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AxiPdmaInCh0IntMap::CORE1_AXI_PDMA_IN_CH0_INT_MAP_MASK)
                | ((value << AxiPdmaInCh0IntMap::CORE1_AXI_PDMA_IN_CH0_INT_MAP_OFFSET)
                    & AxiPdmaInCh0IntMap::CORE1_AXI_PDMA_IN_CH0_INT_MAP_MASK),
        }
    }
}

impl Default for AxiPdmaInCh0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AxiPdmaInCh0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AxiPdmaInCh0IntMapVal {
        AxiPdmaInCh0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AxiPdmaInCh0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AxiPdmaInCh0IntMapVal) -> AxiPdmaInCh0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AXI_PDMA_IN_CH1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x00fc
///
/// ## Fields
///
/// - **CORE1_AXI_PDMA_IN_CH1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AxiPdmaInCh1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AxiPdmaInCh1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AxiPdmaInCh1IntMap {
    // ======== Field Constants ========

    /// **CORE1_AXI_PDMA_IN_CH1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_AXI_PDMA_IN_CH1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_AXI_PDMA_IN_CH1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_AXI_PDMA_IN_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_axi_pdma_in_ch1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_AXI_PDMA_IN_CH1_INT_MAP_MASK)
            >> Self::CORE1_AXI_PDMA_IN_CH1_INT_MAP_OFFSET
    }

    /// Write **CORE1_AXI_PDMA_IN_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_axi_pdma_in_ch1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_AXI_PDMA_IN_CH1_INT_MAP_MASK)
                | ((value << Self::CORE1_AXI_PDMA_IN_CH1_INT_MAP_OFFSET)
                    & Self::CORE1_AXI_PDMA_IN_CH1_INT_MAP_MASK)
        });
    }
}

/// AXI_PDMA_IN_CH1_INT_MAP value type for batch read/modify/write operations
///
/// Use `axipdmainch1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `axipdmainch1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AxiPdmaInCh1IntMapVal {
    bits: u32,
}

impl AxiPdmaInCh1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_AXI_PDMA_IN_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_axi_pdma_in_ch1_int_map(&self) -> u32 {
        (self.bits & AxiPdmaInCh1IntMap::CORE1_AXI_PDMA_IN_CH1_INT_MAP_MASK)
            >> AxiPdmaInCh1IntMap::CORE1_AXI_PDMA_IN_CH1_INT_MAP_OFFSET
    }

    /// Set **CORE1_AXI_PDMA_IN_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_axi_pdma_in_ch1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AxiPdmaInCh1IntMap::CORE1_AXI_PDMA_IN_CH1_INT_MAP_MASK)
                | ((value << AxiPdmaInCh1IntMap::CORE1_AXI_PDMA_IN_CH1_INT_MAP_OFFSET)
                    & AxiPdmaInCh1IntMap::CORE1_AXI_PDMA_IN_CH1_INT_MAP_MASK),
        }
    }
}

impl Default for AxiPdmaInCh1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AxiPdmaInCh1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AxiPdmaInCh1IntMapVal {
        AxiPdmaInCh1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AxiPdmaInCh1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AxiPdmaInCh1IntMapVal) -> AxiPdmaInCh1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AXI_PDMA_IN_CH2_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0100
///
/// ## Fields
///
/// - **CORE1_AXI_PDMA_IN_CH2_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AxiPdmaInCh2IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AxiPdmaInCh2IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AxiPdmaInCh2IntMap {
    // ======== Field Constants ========

    /// **CORE1_AXI_PDMA_IN_CH2_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_AXI_PDMA_IN_CH2_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_AXI_PDMA_IN_CH2_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_AXI_PDMA_IN_CH2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_axi_pdma_in_ch2_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_AXI_PDMA_IN_CH2_INT_MAP_MASK)
            >> Self::CORE1_AXI_PDMA_IN_CH2_INT_MAP_OFFSET
    }

    /// Write **CORE1_AXI_PDMA_IN_CH2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_axi_pdma_in_ch2_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_AXI_PDMA_IN_CH2_INT_MAP_MASK)
                | ((value << Self::CORE1_AXI_PDMA_IN_CH2_INT_MAP_OFFSET)
                    & Self::CORE1_AXI_PDMA_IN_CH2_INT_MAP_MASK)
        });
    }
}

/// AXI_PDMA_IN_CH2_INT_MAP value type for batch read/modify/write operations
///
/// Use `axipdmainch2intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `axipdmainch2intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AxiPdmaInCh2IntMapVal {
    bits: u32,
}

impl AxiPdmaInCh2IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_AXI_PDMA_IN_CH2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_axi_pdma_in_ch2_int_map(&self) -> u32 {
        (self.bits & AxiPdmaInCh2IntMap::CORE1_AXI_PDMA_IN_CH2_INT_MAP_MASK)
            >> AxiPdmaInCh2IntMap::CORE1_AXI_PDMA_IN_CH2_INT_MAP_OFFSET
    }

    /// Set **CORE1_AXI_PDMA_IN_CH2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_axi_pdma_in_ch2_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AxiPdmaInCh2IntMap::CORE1_AXI_PDMA_IN_CH2_INT_MAP_MASK)
                | ((value << AxiPdmaInCh2IntMap::CORE1_AXI_PDMA_IN_CH2_INT_MAP_OFFSET)
                    & AxiPdmaInCh2IntMap::CORE1_AXI_PDMA_IN_CH2_INT_MAP_MASK),
        }
    }
}

impl Default for AxiPdmaInCh2IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AxiPdmaInCh2IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AxiPdmaInCh2IntMapVal {
        AxiPdmaInCh2IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AxiPdmaInCh2IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AxiPdmaInCh2IntMapVal) -> AxiPdmaInCh2IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AXI_PDMA_OUT_CH0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0104
///
/// ## Fields
///
/// - **CORE1_AXI_PDMA_OUT_CH0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AxiPdmaOutCh0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AxiPdmaOutCh0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AxiPdmaOutCh0IntMap {
    // ======== Field Constants ========

    /// **CORE1_AXI_PDMA_OUT_CH0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_AXI_PDMA_OUT_CH0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_AXI_PDMA_OUT_CH0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_AXI_PDMA_OUT_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_axi_pdma_out_ch0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_AXI_PDMA_OUT_CH0_INT_MAP_MASK)
            >> Self::CORE1_AXI_PDMA_OUT_CH0_INT_MAP_OFFSET
    }

    /// Write **CORE1_AXI_PDMA_OUT_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_axi_pdma_out_ch0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_AXI_PDMA_OUT_CH0_INT_MAP_MASK)
                | ((value << Self::CORE1_AXI_PDMA_OUT_CH0_INT_MAP_OFFSET)
                    & Self::CORE1_AXI_PDMA_OUT_CH0_INT_MAP_MASK)
        });
    }
}

/// AXI_PDMA_OUT_CH0_INT_MAP value type for batch read/modify/write operations
///
/// Use `axipdmaoutch0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `axipdmaoutch0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AxiPdmaOutCh0IntMapVal {
    bits: u32,
}

impl AxiPdmaOutCh0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_AXI_PDMA_OUT_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_axi_pdma_out_ch0_int_map(&self) -> u32 {
        (self.bits & AxiPdmaOutCh0IntMap::CORE1_AXI_PDMA_OUT_CH0_INT_MAP_MASK)
            >> AxiPdmaOutCh0IntMap::CORE1_AXI_PDMA_OUT_CH0_INT_MAP_OFFSET
    }

    /// Set **CORE1_AXI_PDMA_OUT_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_axi_pdma_out_ch0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AxiPdmaOutCh0IntMap::CORE1_AXI_PDMA_OUT_CH0_INT_MAP_MASK)
                | ((value << AxiPdmaOutCh0IntMap::CORE1_AXI_PDMA_OUT_CH0_INT_MAP_OFFSET)
                    & AxiPdmaOutCh0IntMap::CORE1_AXI_PDMA_OUT_CH0_INT_MAP_MASK),
        }
    }
}

impl Default for AxiPdmaOutCh0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AxiPdmaOutCh0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AxiPdmaOutCh0IntMapVal {
        AxiPdmaOutCh0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AxiPdmaOutCh0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AxiPdmaOutCh0IntMapVal) -> AxiPdmaOutCh0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AXI_PDMA_OUT_CH1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0108
///
/// ## Fields
///
/// - **CORE1_AXI_PDMA_OUT_CH1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AxiPdmaOutCh1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AxiPdmaOutCh1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AxiPdmaOutCh1IntMap {
    // ======== Field Constants ========

    /// **CORE1_AXI_PDMA_OUT_CH1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_AXI_PDMA_OUT_CH1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_AXI_PDMA_OUT_CH1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_AXI_PDMA_OUT_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_axi_pdma_out_ch1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_AXI_PDMA_OUT_CH1_INT_MAP_MASK)
            >> Self::CORE1_AXI_PDMA_OUT_CH1_INT_MAP_OFFSET
    }

    /// Write **CORE1_AXI_PDMA_OUT_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_axi_pdma_out_ch1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_AXI_PDMA_OUT_CH1_INT_MAP_MASK)
                | ((value << Self::CORE1_AXI_PDMA_OUT_CH1_INT_MAP_OFFSET)
                    & Self::CORE1_AXI_PDMA_OUT_CH1_INT_MAP_MASK)
        });
    }
}

/// AXI_PDMA_OUT_CH1_INT_MAP value type for batch read/modify/write operations
///
/// Use `axipdmaoutch1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `axipdmaoutch1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AxiPdmaOutCh1IntMapVal {
    bits: u32,
}

impl AxiPdmaOutCh1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_AXI_PDMA_OUT_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_axi_pdma_out_ch1_int_map(&self) -> u32 {
        (self.bits & AxiPdmaOutCh1IntMap::CORE1_AXI_PDMA_OUT_CH1_INT_MAP_MASK)
            >> AxiPdmaOutCh1IntMap::CORE1_AXI_PDMA_OUT_CH1_INT_MAP_OFFSET
    }

    /// Set **CORE1_AXI_PDMA_OUT_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_axi_pdma_out_ch1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AxiPdmaOutCh1IntMap::CORE1_AXI_PDMA_OUT_CH1_INT_MAP_MASK)
                | ((value << AxiPdmaOutCh1IntMap::CORE1_AXI_PDMA_OUT_CH1_INT_MAP_OFFSET)
                    & AxiPdmaOutCh1IntMap::CORE1_AXI_PDMA_OUT_CH1_INT_MAP_MASK),
        }
    }
}

impl Default for AxiPdmaOutCh1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AxiPdmaOutCh1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AxiPdmaOutCh1IntMapVal {
        AxiPdmaOutCh1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AxiPdmaOutCh1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AxiPdmaOutCh1IntMapVal) -> AxiPdmaOutCh1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AXI_PDMA_OUT_CH2_INT_MAP Register
///
/// NA
///
/// Address offset: 0x010c
///
/// ## Fields
///
/// - **CORE1_AXI_PDMA_OUT_CH2_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AxiPdmaOutCh2IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AxiPdmaOutCh2IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AxiPdmaOutCh2IntMap {
    // ======== Field Constants ========

    /// **CORE1_AXI_PDMA_OUT_CH2_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_AXI_PDMA_OUT_CH2_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_AXI_PDMA_OUT_CH2_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_AXI_PDMA_OUT_CH2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_axi_pdma_out_ch2_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_AXI_PDMA_OUT_CH2_INT_MAP_MASK)
            >> Self::CORE1_AXI_PDMA_OUT_CH2_INT_MAP_OFFSET
    }

    /// Write **CORE1_AXI_PDMA_OUT_CH2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_axi_pdma_out_ch2_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_AXI_PDMA_OUT_CH2_INT_MAP_MASK)
                | ((value << Self::CORE1_AXI_PDMA_OUT_CH2_INT_MAP_OFFSET)
                    & Self::CORE1_AXI_PDMA_OUT_CH2_INT_MAP_MASK)
        });
    }
}

/// AXI_PDMA_OUT_CH2_INT_MAP value type for batch read/modify/write operations
///
/// Use `axipdmaoutch2intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `axipdmaoutch2intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AxiPdmaOutCh2IntMapVal {
    bits: u32,
}

impl AxiPdmaOutCh2IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_AXI_PDMA_OUT_CH2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_axi_pdma_out_ch2_int_map(&self) -> u32 {
        (self.bits & AxiPdmaOutCh2IntMap::CORE1_AXI_PDMA_OUT_CH2_INT_MAP_MASK)
            >> AxiPdmaOutCh2IntMap::CORE1_AXI_PDMA_OUT_CH2_INT_MAP_OFFSET
    }

    /// Set **CORE1_AXI_PDMA_OUT_CH2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_axi_pdma_out_ch2_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AxiPdmaOutCh2IntMap::CORE1_AXI_PDMA_OUT_CH2_INT_MAP_MASK)
                | ((value << AxiPdmaOutCh2IntMap::CORE1_AXI_PDMA_OUT_CH2_INT_MAP_OFFSET)
                    & AxiPdmaOutCh2IntMap::CORE1_AXI_PDMA_OUT_CH2_INT_MAP_MASK),
        }
    }
}

impl Default for AxiPdmaOutCh2IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AxiPdmaOutCh2IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AxiPdmaOutCh2IntMapVal {
        AxiPdmaOutCh2IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AxiPdmaOutCh2IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AxiPdmaOutCh2IntMapVal) -> AxiPdmaOutCh2IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RSA_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0110
///
/// ## Fields
///
/// - **CORE1_RSA_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct RsaIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RsaIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RsaIntMap {
    // ======== Field Constants ========

    /// **CORE1_RSA_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_RSA_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_RSA_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_RSA_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_rsa_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_RSA_INT_MAP_MASK) >> Self::CORE1_RSA_INT_MAP_OFFSET
    }

    /// Write **CORE1_RSA_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_rsa_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_RSA_INT_MAP_MASK)
                | ((value << Self::CORE1_RSA_INT_MAP_OFFSET) & Self::CORE1_RSA_INT_MAP_MASK)
        });
    }
}

/// RSA_INT_MAP value type for batch read/modify/write operations
///
/// Use `rsaintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rsaintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RsaIntMapVal {
    bits: u32,
}

impl RsaIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_RSA_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_rsa_int_map(&self) -> u32 {
        (self.bits & RsaIntMap::CORE1_RSA_INT_MAP_MASK) >> RsaIntMap::CORE1_RSA_INT_MAP_OFFSET
    }

    /// Set **CORE1_RSA_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_rsa_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RsaIntMap::CORE1_RSA_INT_MAP_MASK)
                | ((value << RsaIntMap::CORE1_RSA_INT_MAP_OFFSET)
                    & RsaIntMap::CORE1_RSA_INT_MAP_MASK),
        }
    }
}

impl Default for RsaIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RsaIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RsaIntMapVal {
        RsaIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RsaIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RsaIntMapVal) -> RsaIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AES_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0114
///
/// ## Fields
///
/// - **CORE1_AES_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AesIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AesIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AesIntMap {
    // ======== Field Constants ========

    /// **CORE1_AES_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_AES_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_AES_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_AES_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_aes_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_AES_INT_MAP_MASK) >> Self::CORE1_AES_INT_MAP_OFFSET
    }

    /// Write **CORE1_AES_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_aes_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_AES_INT_MAP_MASK)
                | ((value << Self::CORE1_AES_INT_MAP_OFFSET) & Self::CORE1_AES_INT_MAP_MASK)
        });
    }
}

/// AES_INT_MAP value type for batch read/modify/write operations
///
/// Use `aesintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `aesintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AesIntMapVal {
    bits: u32,
}

impl AesIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_AES_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_aes_int_map(&self) -> u32 {
        (self.bits & AesIntMap::CORE1_AES_INT_MAP_MASK) >> AesIntMap::CORE1_AES_INT_MAP_OFFSET
    }

    /// Set **CORE1_AES_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_aes_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AesIntMap::CORE1_AES_INT_MAP_MASK)
                | ((value << AesIntMap::CORE1_AES_INT_MAP_OFFSET)
                    & AesIntMap::CORE1_AES_INT_MAP_MASK),
        }
    }
}

impl Default for AesIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AesIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AesIntMapVal {
        AesIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AesIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AesIntMapVal) -> AesIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SHA_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0118
///
/// ## Fields
///
/// - **CORE1_SHA_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct ShaIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ShaIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ShaIntMap {
    // ======== Field Constants ========

    /// **CORE1_SHA_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_SHA_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_SHA_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_SHA_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_sha_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_SHA_INT_MAP_MASK) >> Self::CORE1_SHA_INT_MAP_OFFSET
    }

    /// Write **CORE1_SHA_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_sha_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_SHA_INT_MAP_MASK)
                | ((value << Self::CORE1_SHA_INT_MAP_OFFSET) & Self::CORE1_SHA_INT_MAP_MASK)
        });
    }
}

/// SHA_INT_MAP value type for batch read/modify/write operations
///
/// Use `shaintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `shaintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ShaIntMapVal {
    bits: u32,
}

impl ShaIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_SHA_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_sha_int_map(&self) -> u32 {
        (self.bits & ShaIntMap::CORE1_SHA_INT_MAP_MASK) >> ShaIntMap::CORE1_SHA_INT_MAP_OFFSET
    }

    /// Set **CORE1_SHA_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_sha_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ShaIntMap::CORE1_SHA_INT_MAP_MASK)
                | ((value << ShaIntMap::CORE1_SHA_INT_MAP_OFFSET)
                    & ShaIntMap::CORE1_SHA_INT_MAP_MASK),
        }
    }
}

impl Default for ShaIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ShaIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ShaIntMapVal {
        ShaIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ShaIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ShaIntMapVal) -> ShaIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ECC_INT_MAP Register
///
/// NA
///
/// Address offset: 0x011c
///
/// ## Fields
///
/// - **CORE1_ECC_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct EccIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EccIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EccIntMap {
    // ======== Field Constants ========

    /// **CORE1_ECC_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_ECC_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_ECC_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_ECC_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_ecc_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_ECC_INT_MAP_MASK) >> Self::CORE1_ECC_INT_MAP_OFFSET
    }

    /// Write **CORE1_ECC_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_ecc_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_ECC_INT_MAP_MASK)
                | ((value << Self::CORE1_ECC_INT_MAP_OFFSET) & Self::CORE1_ECC_INT_MAP_MASK)
        });
    }
}

/// ECC_INT_MAP value type for batch read/modify/write operations
///
/// Use `eccintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `eccintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EccIntMapVal {
    bits: u32,
}

impl EccIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_ECC_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_ecc_int_map(&self) -> u32 {
        (self.bits & EccIntMap::CORE1_ECC_INT_MAP_MASK) >> EccIntMap::CORE1_ECC_INT_MAP_OFFSET
    }

    /// Set **CORE1_ECC_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_ecc_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EccIntMap::CORE1_ECC_INT_MAP_MASK)
                | ((value << EccIntMap::CORE1_ECC_INT_MAP_OFFSET)
                    & EccIntMap::CORE1_ECC_INT_MAP_MASK),
        }
    }
}

impl Default for EccIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EccIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EccIntMapVal {
        EccIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EccIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EccIntMapVal) -> EccIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ECDSA_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0120
///
/// ## Fields
///
/// - **CORE1_ECDSA_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct EcdsaIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EcdsaIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EcdsaIntMap {
    // ======== Field Constants ========

    /// **CORE1_ECDSA_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_ECDSA_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_ECDSA_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_ECDSA_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_ecdsa_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_ECDSA_INT_MAP_MASK)
            >> Self::CORE1_ECDSA_INT_MAP_OFFSET
    }

    /// Write **CORE1_ECDSA_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_ecdsa_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_ECDSA_INT_MAP_MASK)
                | ((value << Self::CORE1_ECDSA_INT_MAP_OFFSET) & Self::CORE1_ECDSA_INT_MAP_MASK)
        });
    }
}

/// ECDSA_INT_MAP value type for batch read/modify/write operations
///
/// Use `ecdsaintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ecdsaintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EcdsaIntMapVal {
    bits: u32,
}

impl EcdsaIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_ECDSA_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_ecdsa_int_map(&self) -> u32 {
        (self.bits & EcdsaIntMap::CORE1_ECDSA_INT_MAP_MASK)
            >> EcdsaIntMap::CORE1_ECDSA_INT_MAP_OFFSET
    }

    /// Set **CORE1_ECDSA_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_ecdsa_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EcdsaIntMap::CORE1_ECDSA_INT_MAP_MASK)
                | ((value << EcdsaIntMap::CORE1_ECDSA_INT_MAP_OFFSET)
                    & EcdsaIntMap::CORE1_ECDSA_INT_MAP_MASK),
        }
    }
}

impl Default for EcdsaIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EcdsaIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EcdsaIntMapVal {
        EcdsaIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EcdsaIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EcdsaIntMapVal) -> EcdsaIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// KM_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0124
///
/// ## Fields
///
/// - **CORE1_KM_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct KmIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for KmIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl KmIntMap {
    // ======== Field Constants ========

    /// **CORE1_KM_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_KM_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_KM_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_KM_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_km_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_KM_INT_MAP_MASK) >> Self::CORE1_KM_INT_MAP_OFFSET
    }

    /// Write **CORE1_KM_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_km_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_KM_INT_MAP_MASK)
                | ((value << Self::CORE1_KM_INT_MAP_OFFSET) & Self::CORE1_KM_INT_MAP_MASK)
        });
    }
}

/// KM_INT_MAP value type for batch read/modify/write operations
///
/// Use `kmintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `kmintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct KmIntMapVal {
    bits: u32,
}

impl KmIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_KM_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_km_int_map(&self) -> u32 {
        (self.bits & KmIntMap::CORE1_KM_INT_MAP_MASK) >> KmIntMap::CORE1_KM_INT_MAP_OFFSET
    }

    /// Set **CORE1_KM_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_km_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !KmIntMap::CORE1_KM_INT_MAP_MASK)
                | ((value << KmIntMap::CORE1_KM_INT_MAP_OFFSET) & KmIntMap::CORE1_KM_INT_MAP_MASK),
        }
    }
}

impl Default for KmIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl KmIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> KmIntMapVal {
        KmIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: KmIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(KmIntMapVal) -> KmIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GPIO_INT0_MAP Register
///
/// NA
///
/// Address offset: 0x0128
///
/// ## Fields
///
/// - **CORE1_GPIO_INT0_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct GpioInt0Map {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioInt0Map {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioInt0Map {
    // ======== Field Constants ========

    /// **CORE1_GPIO_INT0_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_GPIO_INT0_MAP_OFFSET: u32 = 0;
    pub const CORE1_GPIO_INT0_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_GPIO_INT0_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_gpio_int0_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_GPIO_INT0_MAP_MASK)
            >> Self::CORE1_GPIO_INT0_MAP_OFFSET
    }

    /// Write **CORE1_GPIO_INT0_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_gpio_int0_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_GPIO_INT0_MAP_MASK)
                | ((value << Self::CORE1_GPIO_INT0_MAP_OFFSET) & Self::CORE1_GPIO_INT0_MAP_MASK)
        });
    }
}

/// GPIO_INT0_MAP value type for batch read/modify/write operations
///
/// Use `gpioint0map.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioint0map.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioInt0MapVal {
    bits: u32,
}

impl GpioInt0MapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_GPIO_INT0_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_gpio_int0_map(&self) -> u32 {
        (self.bits & GpioInt0Map::CORE1_GPIO_INT0_MAP_MASK)
            >> GpioInt0Map::CORE1_GPIO_INT0_MAP_OFFSET
    }

    /// Set **CORE1_GPIO_INT0_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_gpio_int0_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioInt0Map::CORE1_GPIO_INT0_MAP_MASK)
                | ((value << GpioInt0Map::CORE1_GPIO_INT0_MAP_OFFSET)
                    & GpioInt0Map::CORE1_GPIO_INT0_MAP_MASK),
        }
    }
}

impl Default for GpioInt0MapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GpioInt0Map {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioInt0MapVal {
        GpioInt0MapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioInt0MapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioInt0MapVal) -> GpioInt0MapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GPIO_INT1_MAP Register
///
/// NA
///
/// Address offset: 0x012c
///
/// ## Fields
///
/// - **CORE1_GPIO_INT1_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct GpioInt1Map {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioInt1Map {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioInt1Map {
    // ======== Field Constants ========

    /// **CORE1_GPIO_INT1_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_GPIO_INT1_MAP_OFFSET: u32 = 0;
    pub const CORE1_GPIO_INT1_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_GPIO_INT1_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_gpio_int1_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_GPIO_INT1_MAP_MASK)
            >> Self::CORE1_GPIO_INT1_MAP_OFFSET
    }

    /// Write **CORE1_GPIO_INT1_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_gpio_int1_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_GPIO_INT1_MAP_MASK)
                | ((value << Self::CORE1_GPIO_INT1_MAP_OFFSET) & Self::CORE1_GPIO_INT1_MAP_MASK)
        });
    }
}

/// GPIO_INT1_MAP value type for batch read/modify/write operations
///
/// Use `gpioint1map.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioint1map.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioInt1MapVal {
    bits: u32,
}

impl GpioInt1MapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_GPIO_INT1_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_gpio_int1_map(&self) -> u32 {
        (self.bits & GpioInt1Map::CORE1_GPIO_INT1_MAP_MASK)
            >> GpioInt1Map::CORE1_GPIO_INT1_MAP_OFFSET
    }

    /// Set **CORE1_GPIO_INT1_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_gpio_int1_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioInt1Map::CORE1_GPIO_INT1_MAP_MASK)
                | ((value << GpioInt1Map::CORE1_GPIO_INT1_MAP_OFFSET)
                    & GpioInt1Map::CORE1_GPIO_INT1_MAP_MASK),
        }
    }
}

impl Default for GpioInt1MapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GpioInt1Map {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioInt1MapVal {
        GpioInt1MapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioInt1MapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioInt1MapVal) -> GpioInt1MapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GPIO_INT2_MAP Register
///
/// NA
///
/// Address offset: 0x0130
///
/// ## Fields
///
/// - **CORE1_GPIO_INT2_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct GpioInt2Map {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioInt2Map {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioInt2Map {
    // ======== Field Constants ========

    /// **CORE1_GPIO_INT2_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_GPIO_INT2_MAP_OFFSET: u32 = 0;
    pub const CORE1_GPIO_INT2_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_GPIO_INT2_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_gpio_int2_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_GPIO_INT2_MAP_MASK)
            >> Self::CORE1_GPIO_INT2_MAP_OFFSET
    }

    /// Write **CORE1_GPIO_INT2_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_gpio_int2_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_GPIO_INT2_MAP_MASK)
                | ((value << Self::CORE1_GPIO_INT2_MAP_OFFSET) & Self::CORE1_GPIO_INT2_MAP_MASK)
        });
    }
}

/// GPIO_INT2_MAP value type for batch read/modify/write operations
///
/// Use `gpioint2map.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioint2map.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioInt2MapVal {
    bits: u32,
}

impl GpioInt2MapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_GPIO_INT2_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_gpio_int2_map(&self) -> u32 {
        (self.bits & GpioInt2Map::CORE1_GPIO_INT2_MAP_MASK)
            >> GpioInt2Map::CORE1_GPIO_INT2_MAP_OFFSET
    }

    /// Set **CORE1_GPIO_INT2_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_gpio_int2_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioInt2Map::CORE1_GPIO_INT2_MAP_MASK)
                | ((value << GpioInt2Map::CORE1_GPIO_INT2_MAP_OFFSET)
                    & GpioInt2Map::CORE1_GPIO_INT2_MAP_MASK),
        }
    }
}

impl Default for GpioInt2MapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GpioInt2Map {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioInt2MapVal {
        GpioInt2MapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioInt2MapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioInt2MapVal) -> GpioInt2MapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GPIO_INT3_MAP Register
///
/// NA
///
/// Address offset: 0x0134
///
/// ## Fields
///
/// - **CORE1_GPIO_INT3_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct GpioInt3Map {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioInt3Map {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioInt3Map {
    // ======== Field Constants ========

    /// **CORE1_GPIO_INT3_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_GPIO_INT3_MAP_OFFSET: u32 = 0;
    pub const CORE1_GPIO_INT3_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_GPIO_INT3_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_gpio_int3_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_GPIO_INT3_MAP_MASK)
            >> Self::CORE1_GPIO_INT3_MAP_OFFSET
    }

    /// Write **CORE1_GPIO_INT3_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_gpio_int3_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_GPIO_INT3_MAP_MASK)
                | ((value << Self::CORE1_GPIO_INT3_MAP_OFFSET) & Self::CORE1_GPIO_INT3_MAP_MASK)
        });
    }
}

/// GPIO_INT3_MAP value type for batch read/modify/write operations
///
/// Use `gpioint3map.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioint3map.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioInt3MapVal {
    bits: u32,
}

impl GpioInt3MapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_GPIO_INT3_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_gpio_int3_map(&self) -> u32 {
        (self.bits & GpioInt3Map::CORE1_GPIO_INT3_MAP_MASK)
            >> GpioInt3Map::CORE1_GPIO_INT3_MAP_OFFSET
    }

    /// Set **CORE1_GPIO_INT3_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_gpio_int3_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioInt3Map::CORE1_GPIO_INT3_MAP_MASK)
                | ((value << GpioInt3Map::CORE1_GPIO_INT3_MAP_OFFSET)
                    & GpioInt3Map::CORE1_GPIO_INT3_MAP_MASK),
        }
    }
}

impl Default for GpioInt3MapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GpioInt3Map {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioInt3MapVal {
        GpioInt3MapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioInt3MapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioInt3MapVal) -> GpioInt3MapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GPIO_PAD_COMP_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0138
///
/// ## Fields
///
/// - **CORE1_GPIO_PAD_COMP_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct GpioPadCompIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioPadCompIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioPadCompIntMap {
    // ======== Field Constants ========

    /// **CORE1_GPIO_PAD_COMP_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_GPIO_PAD_COMP_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_GPIO_PAD_COMP_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_GPIO_PAD_COMP_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_gpio_pad_comp_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_GPIO_PAD_COMP_INT_MAP_MASK)
            >> Self::CORE1_GPIO_PAD_COMP_INT_MAP_OFFSET
    }

    /// Write **CORE1_GPIO_PAD_COMP_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_gpio_pad_comp_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_GPIO_PAD_COMP_INT_MAP_MASK)
                | ((value << Self::CORE1_GPIO_PAD_COMP_INT_MAP_OFFSET)
                    & Self::CORE1_GPIO_PAD_COMP_INT_MAP_MASK)
        });
    }
}

/// GPIO_PAD_COMP_INT_MAP value type for batch read/modify/write operations
///
/// Use `gpiopadcompintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiopadcompintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioPadCompIntMapVal {
    bits: u32,
}

impl GpioPadCompIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_GPIO_PAD_COMP_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_gpio_pad_comp_int_map(&self) -> u32 {
        (self.bits & GpioPadCompIntMap::CORE1_GPIO_PAD_COMP_INT_MAP_MASK)
            >> GpioPadCompIntMap::CORE1_GPIO_PAD_COMP_INT_MAP_OFFSET
    }

    /// Set **CORE1_GPIO_PAD_COMP_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_gpio_pad_comp_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioPadCompIntMap::CORE1_GPIO_PAD_COMP_INT_MAP_MASK)
                | ((value << GpioPadCompIntMap::CORE1_GPIO_PAD_COMP_INT_MAP_OFFSET)
                    & GpioPadCompIntMap::CORE1_GPIO_PAD_COMP_INT_MAP_MASK),
        }
    }
}

impl Default for GpioPadCompIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GpioPadCompIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioPadCompIntMapVal {
        GpioPadCompIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioPadCompIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioPadCompIntMapVal) -> GpioPadCompIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CPU_INT_FROM_CPU_0_MAP Register
///
/// NA
///
/// Address offset: 0x013c
///
/// ## Fields
///
/// - **CORE1_CPU_INT_FROM_CPU_0_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct CpuIntFromCpu0Map {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CpuIntFromCpu0Map {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CpuIntFromCpu0Map {
    // ======== Field Constants ========

    /// **CORE1_CPU_INT_FROM_CPU_0_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_CPU_INT_FROM_CPU_0_MAP_OFFSET: u32 = 0;
    pub const CORE1_CPU_INT_FROM_CPU_0_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_CPU_INT_FROM_CPU_0_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_cpu_int_from_cpu_0_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_CPU_INT_FROM_CPU_0_MAP_MASK)
            >> Self::CORE1_CPU_INT_FROM_CPU_0_MAP_OFFSET
    }

    /// Write **CORE1_CPU_INT_FROM_CPU_0_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_cpu_int_from_cpu_0_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_CPU_INT_FROM_CPU_0_MAP_MASK)
                | ((value << Self::CORE1_CPU_INT_FROM_CPU_0_MAP_OFFSET)
                    & Self::CORE1_CPU_INT_FROM_CPU_0_MAP_MASK)
        });
    }
}

/// CPU_INT_FROM_CPU_0_MAP value type for batch read/modify/write operations
///
/// Use `cpuintfromcpu0map.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cpuintfromcpu0map.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CpuIntFromCpu0MapVal {
    bits: u32,
}

impl CpuIntFromCpu0MapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_CPU_INT_FROM_CPU_0_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_cpu_int_from_cpu_0_map(&self) -> u32 {
        (self.bits & CpuIntFromCpu0Map::CORE1_CPU_INT_FROM_CPU_0_MAP_MASK)
            >> CpuIntFromCpu0Map::CORE1_CPU_INT_FROM_CPU_0_MAP_OFFSET
    }

    /// Set **CORE1_CPU_INT_FROM_CPU_0_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_cpu_int_from_cpu_0_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CpuIntFromCpu0Map::CORE1_CPU_INT_FROM_CPU_0_MAP_MASK)
                | ((value << CpuIntFromCpu0Map::CORE1_CPU_INT_FROM_CPU_0_MAP_OFFSET)
                    & CpuIntFromCpu0Map::CORE1_CPU_INT_FROM_CPU_0_MAP_MASK),
        }
    }
}

impl Default for CpuIntFromCpu0MapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CpuIntFromCpu0Map {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CpuIntFromCpu0MapVal {
        CpuIntFromCpu0MapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CpuIntFromCpu0MapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CpuIntFromCpu0MapVal) -> CpuIntFromCpu0MapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CPU_INT_FROM_CPU_1_MAP Register
///
/// NA
///
/// Address offset: 0x0140
///
/// ## Fields
///
/// - **CORE1_CPU_INT_FROM_CPU_1_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct CpuIntFromCpu1Map {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CpuIntFromCpu1Map {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CpuIntFromCpu1Map {
    // ======== Field Constants ========

    /// **CORE1_CPU_INT_FROM_CPU_1_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_CPU_INT_FROM_CPU_1_MAP_OFFSET: u32 = 0;
    pub const CORE1_CPU_INT_FROM_CPU_1_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_CPU_INT_FROM_CPU_1_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_cpu_int_from_cpu_1_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_CPU_INT_FROM_CPU_1_MAP_MASK)
            >> Self::CORE1_CPU_INT_FROM_CPU_1_MAP_OFFSET
    }

    /// Write **CORE1_CPU_INT_FROM_CPU_1_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_cpu_int_from_cpu_1_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_CPU_INT_FROM_CPU_1_MAP_MASK)
                | ((value << Self::CORE1_CPU_INT_FROM_CPU_1_MAP_OFFSET)
                    & Self::CORE1_CPU_INT_FROM_CPU_1_MAP_MASK)
        });
    }
}

/// CPU_INT_FROM_CPU_1_MAP value type for batch read/modify/write operations
///
/// Use `cpuintfromcpu1map.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cpuintfromcpu1map.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CpuIntFromCpu1MapVal {
    bits: u32,
}

impl CpuIntFromCpu1MapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_CPU_INT_FROM_CPU_1_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_cpu_int_from_cpu_1_map(&self) -> u32 {
        (self.bits & CpuIntFromCpu1Map::CORE1_CPU_INT_FROM_CPU_1_MAP_MASK)
            >> CpuIntFromCpu1Map::CORE1_CPU_INT_FROM_CPU_1_MAP_OFFSET
    }

    /// Set **CORE1_CPU_INT_FROM_CPU_1_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_cpu_int_from_cpu_1_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CpuIntFromCpu1Map::CORE1_CPU_INT_FROM_CPU_1_MAP_MASK)
                | ((value << CpuIntFromCpu1Map::CORE1_CPU_INT_FROM_CPU_1_MAP_OFFSET)
                    & CpuIntFromCpu1Map::CORE1_CPU_INT_FROM_CPU_1_MAP_MASK),
        }
    }
}

impl Default for CpuIntFromCpu1MapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CpuIntFromCpu1Map {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CpuIntFromCpu1MapVal {
        CpuIntFromCpu1MapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CpuIntFromCpu1MapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CpuIntFromCpu1MapVal) -> CpuIntFromCpu1MapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CPU_INT_FROM_CPU_2_MAP Register
///
/// NA
///
/// Address offset: 0x0144
///
/// ## Fields
///
/// - **CORE1_CPU_INT_FROM_CPU_2_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct CpuIntFromCpu2Map {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CpuIntFromCpu2Map {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CpuIntFromCpu2Map {
    // ======== Field Constants ========

    /// **CORE1_CPU_INT_FROM_CPU_2_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_CPU_INT_FROM_CPU_2_MAP_OFFSET: u32 = 0;
    pub const CORE1_CPU_INT_FROM_CPU_2_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_CPU_INT_FROM_CPU_2_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_cpu_int_from_cpu_2_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_CPU_INT_FROM_CPU_2_MAP_MASK)
            >> Self::CORE1_CPU_INT_FROM_CPU_2_MAP_OFFSET
    }

    /// Write **CORE1_CPU_INT_FROM_CPU_2_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_cpu_int_from_cpu_2_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_CPU_INT_FROM_CPU_2_MAP_MASK)
                | ((value << Self::CORE1_CPU_INT_FROM_CPU_2_MAP_OFFSET)
                    & Self::CORE1_CPU_INT_FROM_CPU_2_MAP_MASK)
        });
    }
}

/// CPU_INT_FROM_CPU_2_MAP value type for batch read/modify/write operations
///
/// Use `cpuintfromcpu2map.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cpuintfromcpu2map.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CpuIntFromCpu2MapVal {
    bits: u32,
}

impl CpuIntFromCpu2MapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_CPU_INT_FROM_CPU_2_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_cpu_int_from_cpu_2_map(&self) -> u32 {
        (self.bits & CpuIntFromCpu2Map::CORE1_CPU_INT_FROM_CPU_2_MAP_MASK)
            >> CpuIntFromCpu2Map::CORE1_CPU_INT_FROM_CPU_2_MAP_OFFSET
    }

    /// Set **CORE1_CPU_INT_FROM_CPU_2_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_cpu_int_from_cpu_2_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CpuIntFromCpu2Map::CORE1_CPU_INT_FROM_CPU_2_MAP_MASK)
                | ((value << CpuIntFromCpu2Map::CORE1_CPU_INT_FROM_CPU_2_MAP_OFFSET)
                    & CpuIntFromCpu2Map::CORE1_CPU_INT_FROM_CPU_2_MAP_MASK),
        }
    }
}

impl Default for CpuIntFromCpu2MapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CpuIntFromCpu2Map {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CpuIntFromCpu2MapVal {
        CpuIntFromCpu2MapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CpuIntFromCpu2MapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CpuIntFromCpu2MapVal) -> CpuIntFromCpu2MapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CPU_INT_FROM_CPU_3_MAP Register
///
/// NA
///
/// Address offset: 0x0148
///
/// ## Fields
///
/// - **CORE1_CPU_INT_FROM_CPU_3_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct CpuIntFromCpu3Map {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CpuIntFromCpu3Map {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CpuIntFromCpu3Map {
    // ======== Field Constants ========

    /// **CORE1_CPU_INT_FROM_CPU_3_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_CPU_INT_FROM_CPU_3_MAP_OFFSET: u32 = 0;
    pub const CORE1_CPU_INT_FROM_CPU_3_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_CPU_INT_FROM_CPU_3_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_cpu_int_from_cpu_3_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_CPU_INT_FROM_CPU_3_MAP_MASK)
            >> Self::CORE1_CPU_INT_FROM_CPU_3_MAP_OFFSET
    }

    /// Write **CORE1_CPU_INT_FROM_CPU_3_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_cpu_int_from_cpu_3_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_CPU_INT_FROM_CPU_3_MAP_MASK)
                | ((value << Self::CORE1_CPU_INT_FROM_CPU_3_MAP_OFFSET)
                    & Self::CORE1_CPU_INT_FROM_CPU_3_MAP_MASK)
        });
    }
}

/// CPU_INT_FROM_CPU_3_MAP value type for batch read/modify/write operations
///
/// Use `cpuintfromcpu3map.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cpuintfromcpu3map.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CpuIntFromCpu3MapVal {
    bits: u32,
}

impl CpuIntFromCpu3MapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_CPU_INT_FROM_CPU_3_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_cpu_int_from_cpu_3_map(&self) -> u32 {
        (self.bits & CpuIntFromCpu3Map::CORE1_CPU_INT_FROM_CPU_3_MAP_MASK)
            >> CpuIntFromCpu3Map::CORE1_CPU_INT_FROM_CPU_3_MAP_OFFSET
    }

    /// Set **CORE1_CPU_INT_FROM_CPU_3_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_cpu_int_from_cpu_3_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CpuIntFromCpu3Map::CORE1_CPU_INT_FROM_CPU_3_MAP_MASK)
                | ((value << CpuIntFromCpu3Map::CORE1_CPU_INT_FROM_CPU_3_MAP_OFFSET)
                    & CpuIntFromCpu3Map::CORE1_CPU_INT_FROM_CPU_3_MAP_MASK),
        }
    }
}

impl Default for CpuIntFromCpu3MapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CpuIntFromCpu3Map {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CpuIntFromCpu3MapVal {
        CpuIntFromCpu3MapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CpuIntFromCpu3MapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CpuIntFromCpu3MapVal) -> CpuIntFromCpu3MapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CACHE_INT_MAP Register
///
/// NA
///
/// Address offset: 0x014c
///
/// ## Fields
///
/// - **CORE1_CACHE_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct CacheIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CacheIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CacheIntMap {
    // ======== Field Constants ========

    /// **CORE1_CACHE_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_CACHE_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_CACHE_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_CACHE_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_cache_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_CACHE_INT_MAP_MASK)
            >> Self::CORE1_CACHE_INT_MAP_OFFSET
    }

    /// Write **CORE1_CACHE_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_cache_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_CACHE_INT_MAP_MASK)
                | ((value << Self::CORE1_CACHE_INT_MAP_OFFSET) & Self::CORE1_CACHE_INT_MAP_MASK)
        });
    }
}

/// CACHE_INT_MAP value type for batch read/modify/write operations
///
/// Use `cacheintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cacheintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CacheIntMapVal {
    bits: u32,
}

impl CacheIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_CACHE_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_cache_int_map(&self) -> u32 {
        (self.bits & CacheIntMap::CORE1_CACHE_INT_MAP_MASK)
            >> CacheIntMap::CORE1_CACHE_INT_MAP_OFFSET
    }

    /// Set **CORE1_CACHE_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_cache_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CacheIntMap::CORE1_CACHE_INT_MAP_MASK)
                | ((value << CacheIntMap::CORE1_CACHE_INT_MAP_OFFSET)
                    & CacheIntMap::CORE1_CACHE_INT_MAP_MASK),
        }
    }
}

impl Default for CacheIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CacheIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CacheIntMapVal {
        CacheIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CacheIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CacheIntMapVal) -> CacheIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// FLASH_MSPI_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0150
///
/// ## Fields
///
/// - **CORE1_FLASH_MSPI_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct FlashMspiIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for FlashMspiIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl FlashMspiIntMap {
    // ======== Field Constants ========

    /// **CORE1_FLASH_MSPI_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_FLASH_MSPI_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_FLASH_MSPI_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_FLASH_MSPI_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_flash_mspi_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_FLASH_MSPI_INT_MAP_MASK)
            >> Self::CORE1_FLASH_MSPI_INT_MAP_OFFSET
    }

    /// Write **CORE1_FLASH_MSPI_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_flash_mspi_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_FLASH_MSPI_INT_MAP_MASK)
                | ((value << Self::CORE1_FLASH_MSPI_INT_MAP_OFFSET)
                    & Self::CORE1_FLASH_MSPI_INT_MAP_MASK)
        });
    }
}

/// FLASH_MSPI_INT_MAP value type for batch read/modify/write operations
///
/// Use `flashmspiintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `flashmspiintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct FlashMspiIntMapVal {
    bits: u32,
}

impl FlashMspiIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_FLASH_MSPI_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_flash_mspi_int_map(&self) -> u32 {
        (self.bits & FlashMspiIntMap::CORE1_FLASH_MSPI_INT_MAP_MASK)
            >> FlashMspiIntMap::CORE1_FLASH_MSPI_INT_MAP_OFFSET
    }

    /// Set **CORE1_FLASH_MSPI_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_flash_mspi_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !FlashMspiIntMap::CORE1_FLASH_MSPI_INT_MAP_MASK)
                | ((value << FlashMspiIntMap::CORE1_FLASH_MSPI_INT_MAP_OFFSET)
                    & FlashMspiIntMap::CORE1_FLASH_MSPI_INT_MAP_MASK),
        }
    }
}

impl Default for FlashMspiIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl FlashMspiIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> FlashMspiIntMapVal {
        FlashMspiIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: FlashMspiIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(FlashMspiIntMapVal) -> FlashMspiIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CSI_BRIDGE_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0154
///
/// ## Fields
///
/// - **CORE1_CSI_BRIDGE_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct CsiBridgeIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CsiBridgeIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CsiBridgeIntMap {
    // ======== Field Constants ========

    /// **CORE1_CSI_BRIDGE_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_CSI_BRIDGE_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_CSI_BRIDGE_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_CSI_BRIDGE_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_csi_bridge_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_CSI_BRIDGE_INT_MAP_MASK)
            >> Self::CORE1_CSI_BRIDGE_INT_MAP_OFFSET
    }

    /// Write **CORE1_CSI_BRIDGE_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_csi_bridge_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_CSI_BRIDGE_INT_MAP_MASK)
                | ((value << Self::CORE1_CSI_BRIDGE_INT_MAP_OFFSET)
                    & Self::CORE1_CSI_BRIDGE_INT_MAP_MASK)
        });
    }
}

/// CSI_BRIDGE_INT_MAP value type for batch read/modify/write operations
///
/// Use `csibridgeintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `csibridgeintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CsiBridgeIntMapVal {
    bits: u32,
}

impl CsiBridgeIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_CSI_BRIDGE_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_csi_bridge_int_map(&self) -> u32 {
        (self.bits & CsiBridgeIntMap::CORE1_CSI_BRIDGE_INT_MAP_MASK)
            >> CsiBridgeIntMap::CORE1_CSI_BRIDGE_INT_MAP_OFFSET
    }

    /// Set **CORE1_CSI_BRIDGE_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_csi_bridge_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CsiBridgeIntMap::CORE1_CSI_BRIDGE_INT_MAP_MASK)
                | ((value << CsiBridgeIntMap::CORE1_CSI_BRIDGE_INT_MAP_OFFSET)
                    & CsiBridgeIntMap::CORE1_CSI_BRIDGE_INT_MAP_MASK),
        }
    }
}

impl Default for CsiBridgeIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CsiBridgeIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CsiBridgeIntMapVal {
        CsiBridgeIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CsiBridgeIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CsiBridgeIntMapVal) -> CsiBridgeIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DSI_BRIDGE_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0158
///
/// ## Fields
///
/// - **CORE1_DSI_BRIDGE_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DsiBridgeIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DsiBridgeIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DsiBridgeIntMap {
    // ======== Field Constants ========

    /// **CORE1_DSI_BRIDGE_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_DSI_BRIDGE_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_DSI_BRIDGE_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_DSI_BRIDGE_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_dsi_bridge_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_DSI_BRIDGE_INT_MAP_MASK)
            >> Self::CORE1_DSI_BRIDGE_INT_MAP_OFFSET
    }

    /// Write **CORE1_DSI_BRIDGE_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_dsi_bridge_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_DSI_BRIDGE_INT_MAP_MASK)
                | ((value << Self::CORE1_DSI_BRIDGE_INT_MAP_OFFSET)
                    & Self::CORE1_DSI_BRIDGE_INT_MAP_MASK)
        });
    }
}

/// DSI_BRIDGE_INT_MAP value type for batch read/modify/write operations
///
/// Use `dsibridgeintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dsibridgeintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DsiBridgeIntMapVal {
    bits: u32,
}

impl DsiBridgeIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_DSI_BRIDGE_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_dsi_bridge_int_map(&self) -> u32 {
        (self.bits & DsiBridgeIntMap::CORE1_DSI_BRIDGE_INT_MAP_MASK)
            >> DsiBridgeIntMap::CORE1_DSI_BRIDGE_INT_MAP_OFFSET
    }

    /// Set **CORE1_DSI_BRIDGE_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_dsi_bridge_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DsiBridgeIntMap::CORE1_DSI_BRIDGE_INT_MAP_MASK)
                | ((value << DsiBridgeIntMap::CORE1_DSI_BRIDGE_INT_MAP_OFFSET)
                    & DsiBridgeIntMap::CORE1_DSI_BRIDGE_INT_MAP_MASK),
        }
    }
}

impl Default for DsiBridgeIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DsiBridgeIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DsiBridgeIntMapVal {
        DsiBridgeIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DsiBridgeIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DsiBridgeIntMapVal) -> DsiBridgeIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CSI_INT_MAP Register
///
/// NA
///
/// Address offset: 0x015c
///
/// ## Fields
///
/// - **CORE1_CSI_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct CsiIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CsiIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CsiIntMap {
    // ======== Field Constants ========

    /// **CORE1_CSI_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_CSI_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_CSI_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_CSI_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_csi_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_CSI_INT_MAP_MASK) >> Self::CORE1_CSI_INT_MAP_OFFSET
    }

    /// Write **CORE1_CSI_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_csi_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_CSI_INT_MAP_MASK)
                | ((value << Self::CORE1_CSI_INT_MAP_OFFSET) & Self::CORE1_CSI_INT_MAP_MASK)
        });
    }
}

/// CSI_INT_MAP value type for batch read/modify/write operations
///
/// Use `csiintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `csiintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CsiIntMapVal {
    bits: u32,
}

impl CsiIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_CSI_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_csi_int_map(&self) -> u32 {
        (self.bits & CsiIntMap::CORE1_CSI_INT_MAP_MASK) >> CsiIntMap::CORE1_CSI_INT_MAP_OFFSET
    }

    /// Set **CORE1_CSI_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_csi_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CsiIntMap::CORE1_CSI_INT_MAP_MASK)
                | ((value << CsiIntMap::CORE1_CSI_INT_MAP_OFFSET)
                    & CsiIntMap::CORE1_CSI_INT_MAP_MASK),
        }
    }
}

impl Default for CsiIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CsiIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CsiIntMapVal {
        CsiIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CsiIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CsiIntMapVal) -> CsiIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DSI_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0160
///
/// ## Fields
///
/// - **CORE1_DSI_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct DsiIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DsiIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DsiIntMap {
    // ======== Field Constants ========

    /// **CORE1_DSI_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_DSI_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_DSI_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_DSI_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_dsi_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_DSI_INT_MAP_MASK) >> Self::CORE1_DSI_INT_MAP_OFFSET
    }

    /// Write **CORE1_DSI_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_dsi_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_DSI_INT_MAP_MASK)
                | ((value << Self::CORE1_DSI_INT_MAP_OFFSET) & Self::CORE1_DSI_INT_MAP_MASK)
        });
    }
}

/// DSI_INT_MAP value type for batch read/modify/write operations
///
/// Use `dsiintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dsiintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DsiIntMapVal {
    bits: u32,
}

impl DsiIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_DSI_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_dsi_int_map(&self) -> u32 {
        (self.bits & DsiIntMap::CORE1_DSI_INT_MAP_MASK) >> DsiIntMap::CORE1_DSI_INT_MAP_OFFSET
    }

    /// Set **CORE1_DSI_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_dsi_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DsiIntMap::CORE1_DSI_INT_MAP_MASK)
                | ((value << DsiIntMap::CORE1_DSI_INT_MAP_OFFSET)
                    & DsiIntMap::CORE1_DSI_INT_MAP_MASK),
        }
    }
}

impl Default for DsiIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DsiIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DsiIntMapVal {
        DsiIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DsiIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DsiIntMapVal) -> DsiIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GMII_PHY_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0164
///
/// ## Fields
///
/// - **CORE1_GMII_PHY_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct GmiiPhyIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GmiiPhyIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GmiiPhyIntMap {
    // ======== Field Constants ========

    /// **CORE1_GMII_PHY_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_GMII_PHY_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_GMII_PHY_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_GMII_PHY_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_gmii_phy_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_GMII_PHY_INT_MAP_MASK)
            >> Self::CORE1_GMII_PHY_INT_MAP_OFFSET
    }

    /// Write **CORE1_GMII_PHY_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_gmii_phy_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_GMII_PHY_INT_MAP_MASK)
                | ((value << Self::CORE1_GMII_PHY_INT_MAP_OFFSET)
                    & Self::CORE1_GMII_PHY_INT_MAP_MASK)
        });
    }
}

/// GMII_PHY_INT_MAP value type for batch read/modify/write operations
///
/// Use `gmiiphyintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gmiiphyintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GmiiPhyIntMapVal {
    bits: u32,
}

impl GmiiPhyIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_GMII_PHY_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_gmii_phy_int_map(&self) -> u32 {
        (self.bits & GmiiPhyIntMap::CORE1_GMII_PHY_INT_MAP_MASK)
            >> GmiiPhyIntMap::CORE1_GMII_PHY_INT_MAP_OFFSET
    }

    /// Set **CORE1_GMII_PHY_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_gmii_phy_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GmiiPhyIntMap::CORE1_GMII_PHY_INT_MAP_MASK)
                | ((value << GmiiPhyIntMap::CORE1_GMII_PHY_INT_MAP_OFFSET)
                    & GmiiPhyIntMap::CORE1_GMII_PHY_INT_MAP_MASK),
        }
    }
}

impl Default for GmiiPhyIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GmiiPhyIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GmiiPhyIntMapVal {
        GmiiPhyIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GmiiPhyIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GmiiPhyIntMapVal) -> GmiiPhyIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LPI_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0168
///
/// ## Fields
///
/// - **CORE1_LPI_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct LpiIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpiIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpiIntMap {
    // ======== Field Constants ========

    /// **CORE1_LPI_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_LPI_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_LPI_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_LPI_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_lpi_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_LPI_INT_MAP_MASK) >> Self::CORE1_LPI_INT_MAP_OFFSET
    }

    /// Write **CORE1_LPI_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_lpi_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_LPI_INT_MAP_MASK)
                | ((value << Self::CORE1_LPI_INT_MAP_OFFSET) & Self::CORE1_LPI_INT_MAP_MASK)
        });
    }
}

/// LPI_INT_MAP value type for batch read/modify/write operations
///
/// Use `lpiintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpiintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpiIntMapVal {
    bits: u32,
}

impl LpiIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_LPI_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_lpi_int_map(&self) -> u32 {
        (self.bits & LpiIntMap::CORE1_LPI_INT_MAP_MASK) >> LpiIntMap::CORE1_LPI_INT_MAP_OFFSET
    }

    /// Set **CORE1_LPI_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_lpi_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpiIntMap::CORE1_LPI_INT_MAP_MASK)
                | ((value << LpiIntMap::CORE1_LPI_INT_MAP_OFFSET)
                    & LpiIntMap::CORE1_LPI_INT_MAP_MASK),
        }
    }
}

impl Default for LpiIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpiIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpiIntMapVal {
        LpiIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpiIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpiIntMapVal) -> LpiIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMT_INT_MAP Register
///
/// NA
///
/// Address offset: 0x016c
///
/// ## Fields
///
/// - **CORE1_PMT_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PmtIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmtIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmtIntMap {
    // ======== Field Constants ========

    /// **CORE1_PMT_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_PMT_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_PMT_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_PMT_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_pmt_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_PMT_INT_MAP_MASK) >> Self::CORE1_PMT_INT_MAP_OFFSET
    }

    /// Write **CORE1_PMT_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_pmt_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_PMT_INT_MAP_MASK)
                | ((value << Self::CORE1_PMT_INT_MAP_OFFSET) & Self::CORE1_PMT_INT_MAP_MASK)
        });
    }
}

/// PMT_INT_MAP value type for batch read/modify/write operations
///
/// Use `pmtintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmtintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmtIntMapVal {
    bits: u32,
}

impl PmtIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_PMT_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_pmt_int_map(&self) -> u32 {
        (self.bits & PmtIntMap::CORE1_PMT_INT_MAP_MASK) >> PmtIntMap::CORE1_PMT_INT_MAP_OFFSET
    }

    /// Set **CORE1_PMT_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_pmt_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmtIntMap::CORE1_PMT_INT_MAP_MASK)
                | ((value << PmtIntMap::CORE1_PMT_INT_MAP_OFFSET)
                    & PmtIntMap::CORE1_PMT_INT_MAP_MASK),
        }
    }
}

impl Default for PmtIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmtIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmtIntMapVal {
        PmtIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmtIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmtIntMapVal) -> PmtIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SBD_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0170
///
/// ## Fields
///
/// - **CORE1_SBD_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct SbdIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SbdIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SbdIntMap {
    // ======== Field Constants ========

    /// **CORE1_SBD_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_SBD_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_SBD_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_SBD_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_sbd_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_SBD_INT_MAP_MASK) >> Self::CORE1_SBD_INT_MAP_OFFSET
    }

    /// Write **CORE1_SBD_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_sbd_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_SBD_INT_MAP_MASK)
                | ((value << Self::CORE1_SBD_INT_MAP_OFFSET) & Self::CORE1_SBD_INT_MAP_MASK)
        });
    }
}

/// SBD_INT_MAP value type for batch read/modify/write operations
///
/// Use `sbdintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sbdintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SbdIntMapVal {
    bits: u32,
}

impl SbdIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_SBD_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_sbd_int_map(&self) -> u32 {
        (self.bits & SbdIntMap::CORE1_SBD_INT_MAP_MASK) >> SbdIntMap::CORE1_SBD_INT_MAP_OFFSET
    }

    /// Set **CORE1_SBD_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_sbd_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SbdIntMap::CORE1_SBD_INT_MAP_MASK)
                | ((value << SbdIntMap::CORE1_SBD_INT_MAP_OFFSET)
                    & SbdIntMap::CORE1_SBD_INT_MAP_MASK),
        }
    }
}

impl Default for SbdIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SbdIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SbdIntMapVal {
        SbdIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SbdIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SbdIntMapVal) -> SbdIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USB_OTG_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0174
///
/// ## Fields
///
/// - **CORE1_USB_OTG_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct UsbOtgIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for UsbOtgIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UsbOtgIntMap {
    // ======== Field Constants ========

    /// **CORE1_USB_OTG_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_USB_OTG_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_USB_OTG_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_USB_OTG_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_usb_otg_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_USB_OTG_INT_MAP_MASK)
            >> Self::CORE1_USB_OTG_INT_MAP_OFFSET
    }

    /// Write **CORE1_USB_OTG_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_usb_otg_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_USB_OTG_INT_MAP_MASK)
                | ((value << Self::CORE1_USB_OTG_INT_MAP_OFFSET) & Self::CORE1_USB_OTG_INT_MAP_MASK)
        });
    }
}

/// USB_OTG_INT_MAP value type for batch read/modify/write operations
///
/// Use `usbotgintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbotgintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbOtgIntMapVal {
    bits: u32,
}

impl UsbOtgIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_USB_OTG_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_usb_otg_int_map(&self) -> u32 {
        (self.bits & UsbOtgIntMap::CORE1_USB_OTG_INT_MAP_MASK)
            >> UsbOtgIntMap::CORE1_USB_OTG_INT_MAP_OFFSET
    }

    /// Set **CORE1_USB_OTG_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_usb_otg_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbOtgIntMap::CORE1_USB_OTG_INT_MAP_MASK)
                | ((value << UsbOtgIntMap::CORE1_USB_OTG_INT_MAP_OFFSET)
                    & UsbOtgIntMap::CORE1_USB_OTG_INT_MAP_MASK),
        }
    }
}

impl Default for UsbOtgIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl UsbOtgIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UsbOtgIntMapVal {
        UsbOtgIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UsbOtgIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UsbOtgIntMapVal) -> UsbOtgIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USB_OTG_ENDP_MULTI_PROC_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0178
///
/// ## Fields
///
/// - **CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct UsbOtgEndpMultiProcIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for UsbOtgEndpMultiProcIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UsbOtgEndpMultiProcIntMap {
    // ======== Field Constants ========

    /// **CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_usb_otg_endp_multi_proc_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP_MASK)
            >> Self::CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP_OFFSET
    }

    /// Write **CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_usb_otg_endp_multi_proc_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP_MASK)
                | ((value << Self::CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP_OFFSET)
                    & Self::CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP_MASK)
        });
    }
}

/// USB_OTG_ENDP_MULTI_PROC_INT_MAP value type for batch read/modify/write operations
///
/// Use `usbotgendpmultiprocintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbotgendpmultiprocintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbOtgEndpMultiProcIntMapVal {
    bits: u32,
}

impl UsbOtgEndpMultiProcIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_usb_otg_endp_multi_proc_int_map(&self) -> u32 {
        (self.bits & UsbOtgEndpMultiProcIntMap::CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP_MASK)
            >> UsbOtgEndpMultiProcIntMap::CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP_OFFSET
    }

    /// Set **CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_usb_otg_endp_multi_proc_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits
                & !UsbOtgEndpMultiProcIntMap::CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP_MASK)
                | ((value
                    << UsbOtgEndpMultiProcIntMap::CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP_OFFSET)
                    & UsbOtgEndpMultiProcIntMap::CORE1_USB_OTG_ENDP_MULTI_PROC_INT_MAP_MASK),
        }
    }
}

impl Default for UsbOtgEndpMultiProcIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl UsbOtgEndpMultiProcIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UsbOtgEndpMultiProcIntMapVal {
        UsbOtgEndpMultiProcIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UsbOtgEndpMultiProcIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UsbOtgEndpMultiProcIntMapVal) -> UsbOtgEndpMultiProcIntMapVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// JPEG_INT_MAP Register
///
/// NA
///
/// Address offset: 0x017c
///
/// ## Fields
///
/// - **CORE1_JPEG_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct JpegIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for JpegIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl JpegIntMap {
    // ======== Field Constants ========

    /// **CORE1_JPEG_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_JPEG_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_JPEG_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_JPEG_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_jpeg_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_JPEG_INT_MAP_MASK) >> Self::CORE1_JPEG_INT_MAP_OFFSET
    }

    /// Write **CORE1_JPEG_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_jpeg_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_JPEG_INT_MAP_MASK)
                | ((value << Self::CORE1_JPEG_INT_MAP_OFFSET) & Self::CORE1_JPEG_INT_MAP_MASK)
        });
    }
}

/// JPEG_INT_MAP value type for batch read/modify/write operations
///
/// Use `jpegintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `jpegintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct JpegIntMapVal {
    bits: u32,
}

impl JpegIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_JPEG_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_jpeg_int_map(&self) -> u32 {
        (self.bits & JpegIntMap::CORE1_JPEG_INT_MAP_MASK) >> JpegIntMap::CORE1_JPEG_INT_MAP_OFFSET
    }

    /// Set **CORE1_JPEG_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_jpeg_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !JpegIntMap::CORE1_JPEG_INT_MAP_MASK)
                | ((value << JpegIntMap::CORE1_JPEG_INT_MAP_OFFSET)
                    & JpegIntMap::CORE1_JPEG_INT_MAP_MASK),
        }
    }
}

impl Default for JpegIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl JpegIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> JpegIntMapVal {
        JpegIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: JpegIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(JpegIntMapVal) -> JpegIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PPA_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0180
///
/// ## Fields
///
/// - **CORE1_PPA_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PpaIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PpaIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PpaIntMap {
    // ======== Field Constants ========

    /// **CORE1_PPA_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_PPA_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_PPA_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_PPA_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_ppa_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_PPA_INT_MAP_MASK) >> Self::CORE1_PPA_INT_MAP_OFFSET
    }

    /// Write **CORE1_PPA_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_ppa_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_PPA_INT_MAP_MASK)
                | ((value << Self::CORE1_PPA_INT_MAP_OFFSET) & Self::CORE1_PPA_INT_MAP_MASK)
        });
    }
}

/// PPA_INT_MAP value type for batch read/modify/write operations
///
/// Use `ppaintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ppaintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PpaIntMapVal {
    bits: u32,
}

impl PpaIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_PPA_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_ppa_int_map(&self) -> u32 {
        (self.bits & PpaIntMap::CORE1_PPA_INT_MAP_MASK) >> PpaIntMap::CORE1_PPA_INT_MAP_OFFSET
    }

    /// Set **CORE1_PPA_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_ppa_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PpaIntMap::CORE1_PPA_INT_MAP_MASK)
                | ((value << PpaIntMap::CORE1_PPA_INT_MAP_OFFSET)
                    & PpaIntMap::CORE1_PPA_INT_MAP_MASK),
        }
    }
}

impl Default for PpaIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PpaIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PpaIntMapVal {
        PpaIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PpaIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PpaIntMapVal) -> PpaIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE0_TRACE_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0184
///
/// ## Fields
///
/// - **CORE1_CORE0_TRACE_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Core0TraceIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core0TraceIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core0TraceIntMap {
    // ======== Field Constants ========

    /// **CORE1_CORE0_TRACE_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_CORE0_TRACE_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_CORE0_TRACE_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_CORE0_TRACE_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_core0_trace_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_CORE0_TRACE_INT_MAP_MASK)
            >> Self::CORE1_CORE0_TRACE_INT_MAP_OFFSET
    }

    /// Write **CORE1_CORE0_TRACE_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_core0_trace_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_CORE0_TRACE_INT_MAP_MASK)
                | ((value << Self::CORE1_CORE0_TRACE_INT_MAP_OFFSET)
                    & Self::CORE1_CORE0_TRACE_INT_MAP_MASK)
        });
    }
}

/// CORE0_TRACE_INT_MAP value type for batch read/modify/write operations
///
/// Use `core0traceintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core0traceintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core0TraceIntMapVal {
    bits: u32,
}

impl Core0TraceIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_CORE0_TRACE_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_core0_trace_int_map(&self) -> u32 {
        (self.bits & Core0TraceIntMap::CORE1_CORE0_TRACE_INT_MAP_MASK)
            >> Core0TraceIntMap::CORE1_CORE0_TRACE_INT_MAP_OFFSET
    }

    /// Set **CORE1_CORE0_TRACE_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_core0_trace_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core0TraceIntMap::CORE1_CORE0_TRACE_INT_MAP_MASK)
                | ((value << Core0TraceIntMap::CORE1_CORE0_TRACE_INT_MAP_OFFSET)
                    & Core0TraceIntMap::CORE1_CORE0_TRACE_INT_MAP_MASK),
        }
    }
}

impl Default for Core0TraceIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core0TraceIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core0TraceIntMapVal {
        Core0TraceIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core0TraceIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core0TraceIntMapVal) -> Core0TraceIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE1_TRACE_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0188
///
/// ## Fields
///
/// - **CORE1_CORE1_TRACE_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Core1TraceIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Core1TraceIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Core1TraceIntMap {
    // ======== Field Constants ========

    /// **CORE1_CORE1_TRACE_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_CORE1_TRACE_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_CORE1_TRACE_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_CORE1_TRACE_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_core1_trace_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_CORE1_TRACE_INT_MAP_MASK)
            >> Self::CORE1_CORE1_TRACE_INT_MAP_OFFSET
    }

    /// Write **CORE1_CORE1_TRACE_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_core1_trace_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_CORE1_TRACE_INT_MAP_MASK)
                | ((value << Self::CORE1_CORE1_TRACE_INT_MAP_OFFSET)
                    & Self::CORE1_CORE1_TRACE_INT_MAP_MASK)
        });
    }
}

/// CORE1_TRACE_INT_MAP value type for batch read/modify/write operations
///
/// Use `core1traceintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `core1traceintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Core1TraceIntMapVal {
    bits: u32,
}

impl Core1TraceIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_CORE1_TRACE_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_core1_trace_int_map(&self) -> u32 {
        (self.bits & Core1TraceIntMap::CORE1_CORE1_TRACE_INT_MAP_MASK)
            >> Core1TraceIntMap::CORE1_CORE1_TRACE_INT_MAP_OFFSET
    }

    /// Set **CORE1_CORE1_TRACE_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_core1_trace_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Core1TraceIntMap::CORE1_CORE1_TRACE_INT_MAP_MASK)
                | ((value << Core1TraceIntMap::CORE1_CORE1_TRACE_INT_MAP_OFFSET)
                    & Core1TraceIntMap::CORE1_CORE1_TRACE_INT_MAP_MASK),
        }
    }
}

impl Default for Core1TraceIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Core1TraceIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Core1TraceIntMapVal {
        Core1TraceIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Core1TraceIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Core1TraceIntMapVal) -> Core1TraceIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HP_CORE_CTRL_INT_MAP Register
///
/// NA
///
/// Address offset: 0x018c
///
/// ## Fields
///
/// - **CORE1_HP_CORE_CTRL_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct HpCoreCtrlIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HpCoreCtrlIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HpCoreCtrlIntMap {
    // ======== Field Constants ========

    /// **CORE1_HP_CORE_CTRL_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_HP_CORE_CTRL_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_HP_CORE_CTRL_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_HP_CORE_CTRL_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_hp_core_ctrl_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_HP_CORE_CTRL_INT_MAP_MASK)
            >> Self::CORE1_HP_CORE_CTRL_INT_MAP_OFFSET
    }

    /// Write **CORE1_HP_CORE_CTRL_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_hp_core_ctrl_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_HP_CORE_CTRL_INT_MAP_MASK)
                | ((value << Self::CORE1_HP_CORE_CTRL_INT_MAP_OFFSET)
                    & Self::CORE1_HP_CORE_CTRL_INT_MAP_MASK)
        });
    }
}

/// HP_CORE_CTRL_INT_MAP value type for batch read/modify/write operations
///
/// Use `hpcorectrlintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hpcorectrlintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HpCoreCtrlIntMapVal {
    bits: u32,
}

impl HpCoreCtrlIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_HP_CORE_CTRL_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_hp_core_ctrl_int_map(&self) -> u32 {
        (self.bits & HpCoreCtrlIntMap::CORE1_HP_CORE_CTRL_INT_MAP_MASK)
            >> HpCoreCtrlIntMap::CORE1_HP_CORE_CTRL_INT_MAP_OFFSET
    }

    /// Set **CORE1_HP_CORE_CTRL_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_hp_core_ctrl_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HpCoreCtrlIntMap::CORE1_HP_CORE_CTRL_INT_MAP_MASK)
                | ((value << HpCoreCtrlIntMap::CORE1_HP_CORE_CTRL_INT_MAP_OFFSET)
                    & HpCoreCtrlIntMap::CORE1_HP_CORE_CTRL_INT_MAP_MASK),
        }
    }
}

impl Default for HpCoreCtrlIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl HpCoreCtrlIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HpCoreCtrlIntMapVal {
        HpCoreCtrlIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HpCoreCtrlIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HpCoreCtrlIntMapVal) -> HpCoreCtrlIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ISP_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0190
///
/// ## Fields
///
/// - **CORE1_ISP_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct IspIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IspIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IspIntMap {
    // ======== Field Constants ========

    /// **CORE1_ISP_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_ISP_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_ISP_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_ISP_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_isp_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_ISP_INT_MAP_MASK) >> Self::CORE1_ISP_INT_MAP_OFFSET
    }

    /// Write **CORE1_ISP_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_isp_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_ISP_INT_MAP_MASK)
                | ((value << Self::CORE1_ISP_INT_MAP_OFFSET) & Self::CORE1_ISP_INT_MAP_MASK)
        });
    }
}

/// ISP_INT_MAP value type for batch read/modify/write operations
///
/// Use `ispintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ispintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IspIntMapVal {
    bits: u32,
}

impl IspIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_ISP_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_isp_int_map(&self) -> u32 {
        (self.bits & IspIntMap::CORE1_ISP_INT_MAP_MASK) >> IspIntMap::CORE1_ISP_INT_MAP_OFFSET
    }

    /// Set **CORE1_ISP_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_isp_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !IspIntMap::CORE1_ISP_INT_MAP_MASK)
                | ((value << IspIntMap::CORE1_ISP_INT_MAP_OFFSET)
                    & IspIntMap::CORE1_ISP_INT_MAP_MASK),
        }
    }
}

impl Default for IspIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IspIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IspIntMapVal {
        IspIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IspIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IspIntMapVal) -> IspIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I3C_MST_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0194
///
/// ## Fields
///
/// - **CORE1_I3C_MST_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct I3cMstIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I3cMstIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I3cMstIntMap {
    // ======== Field Constants ========

    /// **CORE1_I3C_MST_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_I3C_MST_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_I3C_MST_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_I3C_MST_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_i3c_mst_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_I3C_MST_INT_MAP_MASK)
            >> Self::CORE1_I3C_MST_INT_MAP_OFFSET
    }

    /// Write **CORE1_I3C_MST_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_i3c_mst_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_I3C_MST_INT_MAP_MASK)
                | ((value << Self::CORE1_I3C_MST_INT_MAP_OFFSET) & Self::CORE1_I3C_MST_INT_MAP_MASK)
        });
    }
}

/// I3C_MST_INT_MAP value type for batch read/modify/write operations
///
/// Use `i3cmstintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i3cmstintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I3cMstIntMapVal {
    bits: u32,
}

impl I3cMstIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_I3C_MST_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_i3c_mst_int_map(&self) -> u32 {
        (self.bits & I3cMstIntMap::CORE1_I3C_MST_INT_MAP_MASK)
            >> I3cMstIntMap::CORE1_I3C_MST_INT_MAP_OFFSET
    }

    /// Set **CORE1_I3C_MST_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_i3c_mst_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I3cMstIntMap::CORE1_I3C_MST_INT_MAP_MASK)
                | ((value << I3cMstIntMap::CORE1_I3C_MST_INT_MAP_OFFSET)
                    & I3cMstIntMap::CORE1_I3C_MST_INT_MAP_MASK),
        }
    }
}

impl Default for I3cMstIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl I3cMstIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I3cMstIntMapVal {
        I3cMstIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I3cMstIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I3cMstIntMapVal) -> I3cMstIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// I3C_SLV_INT_MAP Register
///
/// NA
///
/// Address offset: 0x0198
///
/// ## Fields
///
/// - **CORE1_I3C_SLV_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct I3cSlvIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for I3cSlvIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl I3cSlvIntMap {
    // ======== Field Constants ========

    /// **CORE1_I3C_SLV_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_I3C_SLV_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_I3C_SLV_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_I3C_SLV_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_i3c_slv_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_I3C_SLV_INT_MAP_MASK)
            >> Self::CORE1_I3C_SLV_INT_MAP_OFFSET
    }

    /// Write **CORE1_I3C_SLV_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_i3c_slv_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_I3C_SLV_INT_MAP_MASK)
                | ((value << Self::CORE1_I3C_SLV_INT_MAP_OFFSET) & Self::CORE1_I3C_SLV_INT_MAP_MASK)
        });
    }
}

/// I3C_SLV_INT_MAP value type for batch read/modify/write operations
///
/// Use `i3cslvintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `i3cslvintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct I3cSlvIntMapVal {
    bits: u32,
}

impl I3cSlvIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_I3C_SLV_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_i3c_slv_int_map(&self) -> u32 {
        (self.bits & I3cSlvIntMap::CORE1_I3C_SLV_INT_MAP_MASK)
            >> I3cSlvIntMap::CORE1_I3C_SLV_INT_MAP_OFFSET
    }

    /// Set **CORE1_I3C_SLV_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_i3c_slv_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !I3cSlvIntMap::CORE1_I3C_SLV_INT_MAP_MASK)
                | ((value << I3cSlvIntMap::CORE1_I3C_SLV_INT_MAP_OFFSET)
                    & I3cSlvIntMap::CORE1_I3C_SLV_INT_MAP_MASK),
        }
    }
}

impl Default for I3cSlvIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl I3cSlvIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> I3cSlvIntMapVal {
        I3cSlvIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: I3cSlvIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(I3cSlvIntMapVal) -> I3cSlvIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USB_OTG11_INT_MAP Register
///
/// NA
///
/// Address offset: 0x019c
///
/// ## Fields
///
/// - **CORE1_USB_OTG11_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct UsbOtg11IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for UsbOtg11IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UsbOtg11IntMap {
    // ======== Field Constants ========

    /// **CORE1_USB_OTG11_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_USB_OTG11_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_USB_OTG11_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_USB_OTG11_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_usb_otg11_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_USB_OTG11_INT_MAP_MASK)
            >> Self::CORE1_USB_OTG11_INT_MAP_OFFSET
    }

    /// Write **CORE1_USB_OTG11_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_usb_otg11_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_USB_OTG11_INT_MAP_MASK)
                | ((value << Self::CORE1_USB_OTG11_INT_MAP_OFFSET)
                    & Self::CORE1_USB_OTG11_INT_MAP_MASK)
        });
    }
}

/// USB_OTG11_INT_MAP value type for batch read/modify/write operations
///
/// Use `usbotg11intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbotg11intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbOtg11IntMapVal {
    bits: u32,
}

impl UsbOtg11IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_USB_OTG11_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_usb_otg11_int_map(&self) -> u32 {
        (self.bits & UsbOtg11IntMap::CORE1_USB_OTG11_INT_MAP_MASK)
            >> UsbOtg11IntMap::CORE1_USB_OTG11_INT_MAP_OFFSET
    }

    /// Set **CORE1_USB_OTG11_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_usb_otg11_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !UsbOtg11IntMap::CORE1_USB_OTG11_INT_MAP_MASK)
                | ((value << UsbOtg11IntMap::CORE1_USB_OTG11_INT_MAP_OFFSET)
                    & UsbOtg11IntMap::CORE1_USB_OTG11_INT_MAP_MASK),
        }
    }
}

impl Default for UsbOtg11IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl UsbOtg11IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UsbOtg11IntMapVal {
        UsbOtg11IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UsbOtg11IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UsbOtg11IntMapVal) -> UsbOtg11IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA2D_IN_CH0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01a0
///
/// ## Fields
///
/// - **CORE1_DMA2D_IN_CH0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Dma2dInCh0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dma2dInCh0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dma2dInCh0IntMap {
    // ======== Field Constants ========

    /// **CORE1_DMA2D_IN_CH0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_DMA2D_IN_CH0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_DMA2D_IN_CH0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_DMA2D_IN_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_dma2d_in_ch0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_DMA2D_IN_CH0_INT_MAP_MASK)
            >> Self::CORE1_DMA2D_IN_CH0_INT_MAP_OFFSET
    }

    /// Write **CORE1_DMA2D_IN_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_dma2d_in_ch0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_DMA2D_IN_CH0_INT_MAP_MASK)
                | ((value << Self::CORE1_DMA2D_IN_CH0_INT_MAP_OFFSET)
                    & Self::CORE1_DMA2D_IN_CH0_INT_MAP_MASK)
        });
    }
}

/// DMA2D_IN_CH0_INT_MAP value type for batch read/modify/write operations
///
/// Use `dma2dinch0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dma2dinch0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Dma2dInCh0IntMapVal {
    bits: u32,
}

impl Dma2dInCh0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_DMA2D_IN_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_dma2d_in_ch0_int_map(&self) -> u32 {
        (self.bits & Dma2dInCh0IntMap::CORE1_DMA2D_IN_CH0_INT_MAP_MASK)
            >> Dma2dInCh0IntMap::CORE1_DMA2D_IN_CH0_INT_MAP_OFFSET
    }

    /// Set **CORE1_DMA2D_IN_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_dma2d_in_ch0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dma2dInCh0IntMap::CORE1_DMA2D_IN_CH0_INT_MAP_MASK)
                | ((value << Dma2dInCh0IntMap::CORE1_DMA2D_IN_CH0_INT_MAP_OFFSET)
                    & Dma2dInCh0IntMap::CORE1_DMA2D_IN_CH0_INT_MAP_MASK),
        }
    }
}

impl Default for Dma2dInCh0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Dma2dInCh0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Dma2dInCh0IntMapVal {
        Dma2dInCh0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Dma2dInCh0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Dma2dInCh0IntMapVal) -> Dma2dInCh0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA2D_IN_CH1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01a4
///
/// ## Fields
///
/// - **CORE1_DMA2D_IN_CH1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Dma2dInCh1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dma2dInCh1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dma2dInCh1IntMap {
    // ======== Field Constants ========

    /// **CORE1_DMA2D_IN_CH1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_DMA2D_IN_CH1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_DMA2D_IN_CH1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_DMA2D_IN_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_dma2d_in_ch1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_DMA2D_IN_CH1_INT_MAP_MASK)
            >> Self::CORE1_DMA2D_IN_CH1_INT_MAP_OFFSET
    }

    /// Write **CORE1_DMA2D_IN_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_dma2d_in_ch1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_DMA2D_IN_CH1_INT_MAP_MASK)
                | ((value << Self::CORE1_DMA2D_IN_CH1_INT_MAP_OFFSET)
                    & Self::CORE1_DMA2D_IN_CH1_INT_MAP_MASK)
        });
    }
}

/// DMA2D_IN_CH1_INT_MAP value type for batch read/modify/write operations
///
/// Use `dma2dinch1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dma2dinch1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Dma2dInCh1IntMapVal {
    bits: u32,
}

impl Dma2dInCh1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_DMA2D_IN_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_dma2d_in_ch1_int_map(&self) -> u32 {
        (self.bits & Dma2dInCh1IntMap::CORE1_DMA2D_IN_CH1_INT_MAP_MASK)
            >> Dma2dInCh1IntMap::CORE1_DMA2D_IN_CH1_INT_MAP_OFFSET
    }

    /// Set **CORE1_DMA2D_IN_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_dma2d_in_ch1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dma2dInCh1IntMap::CORE1_DMA2D_IN_CH1_INT_MAP_MASK)
                | ((value << Dma2dInCh1IntMap::CORE1_DMA2D_IN_CH1_INT_MAP_OFFSET)
                    & Dma2dInCh1IntMap::CORE1_DMA2D_IN_CH1_INT_MAP_MASK),
        }
    }
}

impl Default for Dma2dInCh1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Dma2dInCh1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Dma2dInCh1IntMapVal {
        Dma2dInCh1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Dma2dInCh1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Dma2dInCh1IntMapVal) -> Dma2dInCh1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA2D_OUT_CH0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01a8
///
/// ## Fields
///
/// - **CORE1_DMA2D_OUT_CH0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Dma2dOutCh0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dma2dOutCh0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dma2dOutCh0IntMap {
    // ======== Field Constants ========

    /// **CORE1_DMA2D_OUT_CH0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_DMA2D_OUT_CH0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_DMA2D_OUT_CH0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_DMA2D_OUT_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_dma2d_out_ch0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_DMA2D_OUT_CH0_INT_MAP_MASK)
            >> Self::CORE1_DMA2D_OUT_CH0_INT_MAP_OFFSET
    }

    /// Write **CORE1_DMA2D_OUT_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_dma2d_out_ch0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_DMA2D_OUT_CH0_INT_MAP_MASK)
                | ((value << Self::CORE1_DMA2D_OUT_CH0_INT_MAP_OFFSET)
                    & Self::CORE1_DMA2D_OUT_CH0_INT_MAP_MASK)
        });
    }
}

/// DMA2D_OUT_CH0_INT_MAP value type for batch read/modify/write operations
///
/// Use `dma2doutch0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dma2doutch0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Dma2dOutCh0IntMapVal {
    bits: u32,
}

impl Dma2dOutCh0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_DMA2D_OUT_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_dma2d_out_ch0_int_map(&self) -> u32 {
        (self.bits & Dma2dOutCh0IntMap::CORE1_DMA2D_OUT_CH0_INT_MAP_MASK)
            >> Dma2dOutCh0IntMap::CORE1_DMA2D_OUT_CH0_INT_MAP_OFFSET
    }

    /// Set **CORE1_DMA2D_OUT_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_dma2d_out_ch0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dma2dOutCh0IntMap::CORE1_DMA2D_OUT_CH0_INT_MAP_MASK)
                | ((value << Dma2dOutCh0IntMap::CORE1_DMA2D_OUT_CH0_INT_MAP_OFFSET)
                    & Dma2dOutCh0IntMap::CORE1_DMA2D_OUT_CH0_INT_MAP_MASK),
        }
    }
}

impl Default for Dma2dOutCh0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Dma2dOutCh0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Dma2dOutCh0IntMapVal {
        Dma2dOutCh0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Dma2dOutCh0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Dma2dOutCh0IntMapVal) -> Dma2dOutCh0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA2D_OUT_CH1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01ac
///
/// ## Fields
///
/// - **CORE1_DMA2D_OUT_CH1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Dma2dOutCh1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dma2dOutCh1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dma2dOutCh1IntMap {
    // ======== Field Constants ========

    /// **CORE1_DMA2D_OUT_CH1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_DMA2D_OUT_CH1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_DMA2D_OUT_CH1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_DMA2D_OUT_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_dma2d_out_ch1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_DMA2D_OUT_CH1_INT_MAP_MASK)
            >> Self::CORE1_DMA2D_OUT_CH1_INT_MAP_OFFSET
    }

    /// Write **CORE1_DMA2D_OUT_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_dma2d_out_ch1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_DMA2D_OUT_CH1_INT_MAP_MASK)
                | ((value << Self::CORE1_DMA2D_OUT_CH1_INT_MAP_OFFSET)
                    & Self::CORE1_DMA2D_OUT_CH1_INT_MAP_MASK)
        });
    }
}

/// DMA2D_OUT_CH1_INT_MAP value type for batch read/modify/write operations
///
/// Use `dma2doutch1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dma2doutch1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Dma2dOutCh1IntMapVal {
    bits: u32,
}

impl Dma2dOutCh1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_DMA2D_OUT_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_dma2d_out_ch1_int_map(&self) -> u32 {
        (self.bits & Dma2dOutCh1IntMap::CORE1_DMA2D_OUT_CH1_INT_MAP_MASK)
            >> Dma2dOutCh1IntMap::CORE1_DMA2D_OUT_CH1_INT_MAP_OFFSET
    }

    /// Set **CORE1_DMA2D_OUT_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_dma2d_out_ch1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dma2dOutCh1IntMap::CORE1_DMA2D_OUT_CH1_INT_MAP_MASK)
                | ((value << Dma2dOutCh1IntMap::CORE1_DMA2D_OUT_CH1_INT_MAP_OFFSET)
                    & Dma2dOutCh1IntMap::CORE1_DMA2D_OUT_CH1_INT_MAP_MASK),
        }
    }
}

impl Default for Dma2dOutCh1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Dma2dOutCh1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Dma2dOutCh1IntMapVal {
        Dma2dOutCh1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Dma2dOutCh1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Dma2dOutCh1IntMapVal) -> Dma2dOutCh1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA2D_OUT_CH2_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01b0
///
/// ## Fields
///
/// - **CORE1_DMA2D_OUT_CH2_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Dma2dOutCh2IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dma2dOutCh2IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dma2dOutCh2IntMap {
    // ======== Field Constants ========

    /// **CORE1_DMA2D_OUT_CH2_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_DMA2D_OUT_CH2_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_DMA2D_OUT_CH2_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_DMA2D_OUT_CH2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_dma2d_out_ch2_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_DMA2D_OUT_CH2_INT_MAP_MASK)
            >> Self::CORE1_DMA2D_OUT_CH2_INT_MAP_OFFSET
    }

    /// Write **CORE1_DMA2D_OUT_CH2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_dma2d_out_ch2_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_DMA2D_OUT_CH2_INT_MAP_MASK)
                | ((value << Self::CORE1_DMA2D_OUT_CH2_INT_MAP_OFFSET)
                    & Self::CORE1_DMA2D_OUT_CH2_INT_MAP_MASK)
        });
    }
}

/// DMA2D_OUT_CH2_INT_MAP value type for batch read/modify/write operations
///
/// Use `dma2doutch2intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dma2doutch2intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Dma2dOutCh2IntMapVal {
    bits: u32,
}

impl Dma2dOutCh2IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_DMA2D_OUT_CH2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_dma2d_out_ch2_int_map(&self) -> u32 {
        (self.bits & Dma2dOutCh2IntMap::CORE1_DMA2D_OUT_CH2_INT_MAP_MASK)
            >> Dma2dOutCh2IntMap::CORE1_DMA2D_OUT_CH2_INT_MAP_OFFSET
    }

    /// Set **CORE1_DMA2D_OUT_CH2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_dma2d_out_ch2_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dma2dOutCh2IntMap::CORE1_DMA2D_OUT_CH2_INT_MAP_MASK)
                | ((value << Dma2dOutCh2IntMap::CORE1_DMA2D_OUT_CH2_INT_MAP_OFFSET)
                    & Dma2dOutCh2IntMap::CORE1_DMA2D_OUT_CH2_INT_MAP_MASK),
        }
    }
}

impl Default for Dma2dOutCh2IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Dma2dOutCh2IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Dma2dOutCh2IntMapVal {
        Dma2dOutCh2IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Dma2dOutCh2IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Dma2dOutCh2IntMapVal) -> Dma2dOutCh2IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PSRAM_MSPI_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01b4
///
/// ## Fields
///
/// - **CORE1_PSRAM_MSPI_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PsramMspiIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PsramMspiIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PsramMspiIntMap {
    // ======== Field Constants ========

    /// **CORE1_PSRAM_MSPI_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_PSRAM_MSPI_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_PSRAM_MSPI_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_PSRAM_MSPI_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_psram_mspi_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_PSRAM_MSPI_INT_MAP_MASK)
            >> Self::CORE1_PSRAM_MSPI_INT_MAP_OFFSET
    }

    /// Write **CORE1_PSRAM_MSPI_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_psram_mspi_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_PSRAM_MSPI_INT_MAP_MASK)
                | ((value << Self::CORE1_PSRAM_MSPI_INT_MAP_OFFSET)
                    & Self::CORE1_PSRAM_MSPI_INT_MAP_MASK)
        });
    }
}

/// PSRAM_MSPI_INT_MAP value type for batch read/modify/write operations
///
/// Use `psrammspiintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `psrammspiintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PsramMspiIntMapVal {
    bits: u32,
}

impl PsramMspiIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_PSRAM_MSPI_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_psram_mspi_int_map(&self) -> u32 {
        (self.bits & PsramMspiIntMap::CORE1_PSRAM_MSPI_INT_MAP_MASK)
            >> PsramMspiIntMap::CORE1_PSRAM_MSPI_INT_MAP_OFFSET
    }

    /// Set **CORE1_PSRAM_MSPI_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_psram_mspi_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PsramMspiIntMap::CORE1_PSRAM_MSPI_INT_MAP_MASK)
                | ((value << PsramMspiIntMap::CORE1_PSRAM_MSPI_INT_MAP_OFFSET)
                    & PsramMspiIntMap::CORE1_PSRAM_MSPI_INT_MAP_MASK),
        }
    }
}

impl Default for PsramMspiIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PsramMspiIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PsramMspiIntMapVal {
        PsramMspiIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PsramMspiIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PsramMspiIntMapVal) -> PsramMspiIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HP_SYSREG_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01b8
///
/// ## Fields
///
/// - **CORE1_HP_SYSREG_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct HpSysregIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HpSysregIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HpSysregIntMap {
    // ======== Field Constants ========

    /// **CORE1_HP_SYSREG_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_HP_SYSREG_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_HP_SYSREG_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_HP_SYSREG_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_hp_sysreg_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_HP_SYSREG_INT_MAP_MASK)
            >> Self::CORE1_HP_SYSREG_INT_MAP_OFFSET
    }

    /// Write **CORE1_HP_SYSREG_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_hp_sysreg_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_HP_SYSREG_INT_MAP_MASK)
                | ((value << Self::CORE1_HP_SYSREG_INT_MAP_OFFSET)
                    & Self::CORE1_HP_SYSREG_INT_MAP_MASK)
        });
    }
}

/// HP_SYSREG_INT_MAP value type for batch read/modify/write operations
///
/// Use `hpsysregintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hpsysregintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HpSysregIntMapVal {
    bits: u32,
}

impl HpSysregIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_HP_SYSREG_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_hp_sysreg_int_map(&self) -> u32 {
        (self.bits & HpSysregIntMap::CORE1_HP_SYSREG_INT_MAP_MASK)
            >> HpSysregIntMap::CORE1_HP_SYSREG_INT_MAP_OFFSET
    }

    /// Set **CORE1_HP_SYSREG_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_hp_sysreg_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HpSysregIntMap::CORE1_HP_SYSREG_INT_MAP_MASK)
                | ((value << HpSysregIntMap::CORE1_HP_SYSREG_INT_MAP_OFFSET)
                    & HpSysregIntMap::CORE1_HP_SYSREG_INT_MAP_MASK),
        }
    }
}

impl Default for HpSysregIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl HpSysregIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HpSysregIntMapVal {
        HpSysregIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HpSysregIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HpSysregIntMapVal) -> HpSysregIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PCNT_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01bc
///
/// ## Fields
///
/// - **CORE1_PCNT_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct PcntIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PcntIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PcntIntMap {
    // ======== Field Constants ========

    /// **CORE1_PCNT_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_PCNT_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_PCNT_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_PCNT_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_pcnt_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_PCNT_INT_MAP_MASK) >> Self::CORE1_PCNT_INT_MAP_OFFSET
    }

    /// Write **CORE1_PCNT_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_pcnt_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_PCNT_INT_MAP_MASK)
                | ((value << Self::CORE1_PCNT_INT_MAP_OFFSET) & Self::CORE1_PCNT_INT_MAP_MASK)
        });
    }
}

/// PCNT_INT_MAP value type for batch read/modify/write operations
///
/// Use `pcntintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pcntintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PcntIntMapVal {
    bits: u32,
}

impl PcntIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_PCNT_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_pcnt_int_map(&self) -> u32 {
        (self.bits & PcntIntMap::CORE1_PCNT_INT_MAP_MASK) >> PcntIntMap::CORE1_PCNT_INT_MAP_OFFSET
    }

    /// Set **CORE1_PCNT_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_pcnt_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PcntIntMap::CORE1_PCNT_INT_MAP_MASK)
                | ((value << PcntIntMap::CORE1_PCNT_INT_MAP_OFFSET)
                    & PcntIntMap::CORE1_PCNT_INT_MAP_MASK),
        }
    }
}

impl Default for PcntIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PcntIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PcntIntMapVal {
        PcntIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PcntIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PcntIntMapVal) -> PcntIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HP_PAU_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01c0
///
/// ## Fields
///
/// - **CORE1_HP_PAU_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct HpPauIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HpPauIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HpPauIntMap {
    // ======== Field Constants ========

    /// **CORE1_HP_PAU_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_HP_PAU_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_HP_PAU_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_HP_PAU_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_hp_pau_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_HP_PAU_INT_MAP_MASK)
            >> Self::CORE1_HP_PAU_INT_MAP_OFFSET
    }

    /// Write **CORE1_HP_PAU_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_hp_pau_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_HP_PAU_INT_MAP_MASK)
                | ((value << Self::CORE1_HP_PAU_INT_MAP_OFFSET) & Self::CORE1_HP_PAU_INT_MAP_MASK)
        });
    }
}

/// HP_PAU_INT_MAP value type for batch read/modify/write operations
///
/// Use `hppauintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hppauintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HpPauIntMapVal {
    bits: u32,
}

impl HpPauIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_HP_PAU_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_hp_pau_int_map(&self) -> u32 {
        (self.bits & HpPauIntMap::CORE1_HP_PAU_INT_MAP_MASK)
            >> HpPauIntMap::CORE1_HP_PAU_INT_MAP_OFFSET
    }

    /// Set **CORE1_HP_PAU_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_hp_pau_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HpPauIntMap::CORE1_HP_PAU_INT_MAP_MASK)
                | ((value << HpPauIntMap::CORE1_HP_PAU_INT_MAP_OFFSET)
                    & HpPauIntMap::CORE1_HP_PAU_INT_MAP_MASK),
        }
    }
}

impl Default for HpPauIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl HpPauIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HpPauIntMapVal {
        HpPauIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HpPauIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HpPauIntMapVal) -> HpPauIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HP_PARLIO_RX_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01c4
///
/// ## Fields
///
/// - **CORE1_HP_PARLIO_RX_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct HpParlioRxIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HpParlioRxIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HpParlioRxIntMap {
    // ======== Field Constants ========

    /// **CORE1_HP_PARLIO_RX_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_HP_PARLIO_RX_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_HP_PARLIO_RX_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_HP_PARLIO_RX_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_hp_parlio_rx_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_HP_PARLIO_RX_INT_MAP_MASK)
            >> Self::CORE1_HP_PARLIO_RX_INT_MAP_OFFSET
    }

    /// Write **CORE1_HP_PARLIO_RX_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_hp_parlio_rx_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_HP_PARLIO_RX_INT_MAP_MASK)
                | ((value << Self::CORE1_HP_PARLIO_RX_INT_MAP_OFFSET)
                    & Self::CORE1_HP_PARLIO_RX_INT_MAP_MASK)
        });
    }
}

/// HP_PARLIO_RX_INT_MAP value type for batch read/modify/write operations
///
/// Use `hpparliorxintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hpparliorxintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HpParlioRxIntMapVal {
    bits: u32,
}

impl HpParlioRxIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_HP_PARLIO_RX_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_hp_parlio_rx_int_map(&self) -> u32 {
        (self.bits & HpParlioRxIntMap::CORE1_HP_PARLIO_RX_INT_MAP_MASK)
            >> HpParlioRxIntMap::CORE1_HP_PARLIO_RX_INT_MAP_OFFSET
    }

    /// Set **CORE1_HP_PARLIO_RX_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_hp_parlio_rx_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HpParlioRxIntMap::CORE1_HP_PARLIO_RX_INT_MAP_MASK)
                | ((value << HpParlioRxIntMap::CORE1_HP_PARLIO_RX_INT_MAP_OFFSET)
                    & HpParlioRxIntMap::CORE1_HP_PARLIO_RX_INT_MAP_MASK),
        }
    }
}

impl Default for HpParlioRxIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl HpParlioRxIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HpParlioRxIntMapVal {
        HpParlioRxIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HpParlioRxIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HpParlioRxIntMapVal) -> HpParlioRxIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HP_PARLIO_TX_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01c8
///
/// ## Fields
///
/// - **CORE1_HP_PARLIO_TX_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct HpParlioTxIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HpParlioTxIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HpParlioTxIntMap {
    // ======== Field Constants ========

    /// **CORE1_HP_PARLIO_TX_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_HP_PARLIO_TX_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_HP_PARLIO_TX_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_HP_PARLIO_TX_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_hp_parlio_tx_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_HP_PARLIO_TX_INT_MAP_MASK)
            >> Self::CORE1_HP_PARLIO_TX_INT_MAP_OFFSET
    }

    /// Write **CORE1_HP_PARLIO_TX_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_hp_parlio_tx_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_HP_PARLIO_TX_INT_MAP_MASK)
                | ((value << Self::CORE1_HP_PARLIO_TX_INT_MAP_OFFSET)
                    & Self::CORE1_HP_PARLIO_TX_INT_MAP_MASK)
        });
    }
}

/// HP_PARLIO_TX_INT_MAP value type for batch read/modify/write operations
///
/// Use `hpparliotxintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hpparliotxintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HpParlioTxIntMapVal {
    bits: u32,
}

impl HpParlioTxIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_HP_PARLIO_TX_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_hp_parlio_tx_int_map(&self) -> u32 {
        (self.bits & HpParlioTxIntMap::CORE1_HP_PARLIO_TX_INT_MAP_MASK)
            >> HpParlioTxIntMap::CORE1_HP_PARLIO_TX_INT_MAP_OFFSET
    }

    /// Set **CORE1_HP_PARLIO_TX_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_hp_parlio_tx_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HpParlioTxIntMap::CORE1_HP_PARLIO_TX_INT_MAP_MASK)
                | ((value << HpParlioTxIntMap::CORE1_HP_PARLIO_TX_INT_MAP_OFFSET)
                    & HpParlioTxIntMap::CORE1_HP_PARLIO_TX_INT_MAP_MASK),
        }
    }
}

impl Default for HpParlioTxIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl HpParlioTxIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HpParlioTxIntMapVal {
        HpParlioTxIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HpParlioTxIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HpParlioTxIntMapVal) -> HpParlioTxIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// H264_DMA2D_OUT_CH0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01cc
///
/// ## Fields
///
/// - **CORE1_H264_DMA2D_OUT_CH0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct H264Dma2dOutCh0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for H264Dma2dOutCh0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl H264Dma2dOutCh0IntMap {
    // ======== Field Constants ========

    /// **CORE1_H264_DMA2D_OUT_CH0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_H264_DMA2D_OUT_CH0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_H264_DMA2D_OUT_CH0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_H264_DMA2D_OUT_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_h264_dma2d_out_ch0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_H264_DMA2D_OUT_CH0_INT_MAP_MASK)
            >> Self::CORE1_H264_DMA2D_OUT_CH0_INT_MAP_OFFSET
    }

    /// Write **CORE1_H264_DMA2D_OUT_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_h264_dma2d_out_ch0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_H264_DMA2D_OUT_CH0_INT_MAP_MASK)
                | ((value << Self::CORE1_H264_DMA2D_OUT_CH0_INT_MAP_OFFSET)
                    & Self::CORE1_H264_DMA2D_OUT_CH0_INT_MAP_MASK)
        });
    }
}

/// H264_DMA2D_OUT_CH0_INT_MAP value type for batch read/modify/write operations
///
/// Use `h264dma2doutch0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `h264dma2doutch0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct H264Dma2dOutCh0IntMapVal {
    bits: u32,
}

impl H264Dma2dOutCh0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_H264_DMA2D_OUT_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_h264_dma2d_out_ch0_int_map(&self) -> u32 {
        (self.bits & H264Dma2dOutCh0IntMap::CORE1_H264_DMA2D_OUT_CH0_INT_MAP_MASK)
            >> H264Dma2dOutCh0IntMap::CORE1_H264_DMA2D_OUT_CH0_INT_MAP_OFFSET
    }

    /// Set **CORE1_H264_DMA2D_OUT_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_h264_dma2d_out_ch0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !H264Dma2dOutCh0IntMap::CORE1_H264_DMA2D_OUT_CH0_INT_MAP_MASK)
                | ((value << H264Dma2dOutCh0IntMap::CORE1_H264_DMA2D_OUT_CH0_INT_MAP_OFFSET)
                    & H264Dma2dOutCh0IntMap::CORE1_H264_DMA2D_OUT_CH0_INT_MAP_MASK),
        }
    }
}

impl Default for H264Dma2dOutCh0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl H264Dma2dOutCh0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> H264Dma2dOutCh0IntMapVal {
        H264Dma2dOutCh0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: H264Dma2dOutCh0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(H264Dma2dOutCh0IntMapVal) -> H264Dma2dOutCh0IntMapVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// H264_DMA2D_OUT_CH1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01d0
///
/// ## Fields
///
/// - **CORE1_H264_DMA2D_OUT_CH1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct H264Dma2dOutCh1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for H264Dma2dOutCh1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl H264Dma2dOutCh1IntMap {
    // ======== Field Constants ========

    /// **CORE1_H264_DMA2D_OUT_CH1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_H264_DMA2D_OUT_CH1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_H264_DMA2D_OUT_CH1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_H264_DMA2D_OUT_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_h264_dma2d_out_ch1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_H264_DMA2D_OUT_CH1_INT_MAP_MASK)
            >> Self::CORE1_H264_DMA2D_OUT_CH1_INT_MAP_OFFSET
    }

    /// Write **CORE1_H264_DMA2D_OUT_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_h264_dma2d_out_ch1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_H264_DMA2D_OUT_CH1_INT_MAP_MASK)
                | ((value << Self::CORE1_H264_DMA2D_OUT_CH1_INT_MAP_OFFSET)
                    & Self::CORE1_H264_DMA2D_OUT_CH1_INT_MAP_MASK)
        });
    }
}

/// H264_DMA2D_OUT_CH1_INT_MAP value type for batch read/modify/write operations
///
/// Use `h264dma2doutch1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `h264dma2doutch1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct H264Dma2dOutCh1IntMapVal {
    bits: u32,
}

impl H264Dma2dOutCh1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_H264_DMA2D_OUT_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_h264_dma2d_out_ch1_int_map(&self) -> u32 {
        (self.bits & H264Dma2dOutCh1IntMap::CORE1_H264_DMA2D_OUT_CH1_INT_MAP_MASK)
            >> H264Dma2dOutCh1IntMap::CORE1_H264_DMA2D_OUT_CH1_INT_MAP_OFFSET
    }

    /// Set **CORE1_H264_DMA2D_OUT_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_h264_dma2d_out_ch1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !H264Dma2dOutCh1IntMap::CORE1_H264_DMA2D_OUT_CH1_INT_MAP_MASK)
                | ((value << H264Dma2dOutCh1IntMap::CORE1_H264_DMA2D_OUT_CH1_INT_MAP_OFFSET)
                    & H264Dma2dOutCh1IntMap::CORE1_H264_DMA2D_OUT_CH1_INT_MAP_MASK),
        }
    }
}

impl Default for H264Dma2dOutCh1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl H264Dma2dOutCh1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> H264Dma2dOutCh1IntMapVal {
        H264Dma2dOutCh1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: H264Dma2dOutCh1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(H264Dma2dOutCh1IntMapVal) -> H264Dma2dOutCh1IntMapVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// H264_DMA2D_OUT_CH2_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01d4
///
/// ## Fields
///
/// - **CORE1_H264_DMA2D_OUT_CH2_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct H264Dma2dOutCh2IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for H264Dma2dOutCh2IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl H264Dma2dOutCh2IntMap {
    // ======== Field Constants ========

    /// **CORE1_H264_DMA2D_OUT_CH2_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_H264_DMA2D_OUT_CH2_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_H264_DMA2D_OUT_CH2_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_H264_DMA2D_OUT_CH2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_h264_dma2d_out_ch2_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_H264_DMA2D_OUT_CH2_INT_MAP_MASK)
            >> Self::CORE1_H264_DMA2D_OUT_CH2_INT_MAP_OFFSET
    }

    /// Write **CORE1_H264_DMA2D_OUT_CH2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_h264_dma2d_out_ch2_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_H264_DMA2D_OUT_CH2_INT_MAP_MASK)
                | ((value << Self::CORE1_H264_DMA2D_OUT_CH2_INT_MAP_OFFSET)
                    & Self::CORE1_H264_DMA2D_OUT_CH2_INT_MAP_MASK)
        });
    }
}

/// H264_DMA2D_OUT_CH2_INT_MAP value type for batch read/modify/write operations
///
/// Use `h264dma2doutch2intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `h264dma2doutch2intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct H264Dma2dOutCh2IntMapVal {
    bits: u32,
}

impl H264Dma2dOutCh2IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_H264_DMA2D_OUT_CH2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_h264_dma2d_out_ch2_int_map(&self) -> u32 {
        (self.bits & H264Dma2dOutCh2IntMap::CORE1_H264_DMA2D_OUT_CH2_INT_MAP_MASK)
            >> H264Dma2dOutCh2IntMap::CORE1_H264_DMA2D_OUT_CH2_INT_MAP_OFFSET
    }

    /// Set **CORE1_H264_DMA2D_OUT_CH2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_h264_dma2d_out_ch2_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !H264Dma2dOutCh2IntMap::CORE1_H264_DMA2D_OUT_CH2_INT_MAP_MASK)
                | ((value << H264Dma2dOutCh2IntMap::CORE1_H264_DMA2D_OUT_CH2_INT_MAP_OFFSET)
                    & H264Dma2dOutCh2IntMap::CORE1_H264_DMA2D_OUT_CH2_INT_MAP_MASK),
        }
    }
}

impl Default for H264Dma2dOutCh2IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl H264Dma2dOutCh2IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> H264Dma2dOutCh2IntMapVal {
        H264Dma2dOutCh2IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: H264Dma2dOutCh2IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(H264Dma2dOutCh2IntMapVal) -> H264Dma2dOutCh2IntMapVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// H264_DMA2D_OUT_CH3_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01d8
///
/// ## Fields
///
/// - **CORE1_H264_DMA2D_OUT_CH3_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct H264Dma2dOutCh3IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for H264Dma2dOutCh3IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl H264Dma2dOutCh3IntMap {
    // ======== Field Constants ========

    /// **CORE1_H264_DMA2D_OUT_CH3_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_H264_DMA2D_OUT_CH3_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_H264_DMA2D_OUT_CH3_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_H264_DMA2D_OUT_CH3_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_h264_dma2d_out_ch3_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_H264_DMA2D_OUT_CH3_INT_MAP_MASK)
            >> Self::CORE1_H264_DMA2D_OUT_CH3_INT_MAP_OFFSET
    }

    /// Write **CORE1_H264_DMA2D_OUT_CH3_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_h264_dma2d_out_ch3_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_H264_DMA2D_OUT_CH3_INT_MAP_MASK)
                | ((value << Self::CORE1_H264_DMA2D_OUT_CH3_INT_MAP_OFFSET)
                    & Self::CORE1_H264_DMA2D_OUT_CH3_INT_MAP_MASK)
        });
    }
}

/// H264_DMA2D_OUT_CH3_INT_MAP value type for batch read/modify/write operations
///
/// Use `h264dma2doutch3intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `h264dma2doutch3intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct H264Dma2dOutCh3IntMapVal {
    bits: u32,
}

impl H264Dma2dOutCh3IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_H264_DMA2D_OUT_CH3_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_h264_dma2d_out_ch3_int_map(&self) -> u32 {
        (self.bits & H264Dma2dOutCh3IntMap::CORE1_H264_DMA2D_OUT_CH3_INT_MAP_MASK)
            >> H264Dma2dOutCh3IntMap::CORE1_H264_DMA2D_OUT_CH3_INT_MAP_OFFSET
    }

    /// Set **CORE1_H264_DMA2D_OUT_CH3_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_h264_dma2d_out_ch3_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !H264Dma2dOutCh3IntMap::CORE1_H264_DMA2D_OUT_CH3_INT_MAP_MASK)
                | ((value << H264Dma2dOutCh3IntMap::CORE1_H264_DMA2D_OUT_CH3_INT_MAP_OFFSET)
                    & H264Dma2dOutCh3IntMap::CORE1_H264_DMA2D_OUT_CH3_INT_MAP_MASK),
        }
    }
}

impl Default for H264Dma2dOutCh3IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl H264Dma2dOutCh3IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> H264Dma2dOutCh3IntMapVal {
        H264Dma2dOutCh3IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: H264Dma2dOutCh3IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(H264Dma2dOutCh3IntMapVal) -> H264Dma2dOutCh3IntMapVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// H264_DMA2D_OUT_CH4_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01dc
///
/// ## Fields
///
/// - **CORE1_H264_DMA2D_OUT_CH4_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct H264Dma2dOutCh4IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for H264Dma2dOutCh4IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl H264Dma2dOutCh4IntMap {
    // ======== Field Constants ========

    /// **CORE1_H264_DMA2D_OUT_CH4_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_H264_DMA2D_OUT_CH4_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_H264_DMA2D_OUT_CH4_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_H264_DMA2D_OUT_CH4_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_h264_dma2d_out_ch4_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_H264_DMA2D_OUT_CH4_INT_MAP_MASK)
            >> Self::CORE1_H264_DMA2D_OUT_CH4_INT_MAP_OFFSET
    }

    /// Write **CORE1_H264_DMA2D_OUT_CH4_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_h264_dma2d_out_ch4_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_H264_DMA2D_OUT_CH4_INT_MAP_MASK)
                | ((value << Self::CORE1_H264_DMA2D_OUT_CH4_INT_MAP_OFFSET)
                    & Self::CORE1_H264_DMA2D_OUT_CH4_INT_MAP_MASK)
        });
    }
}

/// H264_DMA2D_OUT_CH4_INT_MAP value type for batch read/modify/write operations
///
/// Use `h264dma2doutch4intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `h264dma2doutch4intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct H264Dma2dOutCh4IntMapVal {
    bits: u32,
}

impl H264Dma2dOutCh4IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_H264_DMA2D_OUT_CH4_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_h264_dma2d_out_ch4_int_map(&self) -> u32 {
        (self.bits & H264Dma2dOutCh4IntMap::CORE1_H264_DMA2D_OUT_CH4_INT_MAP_MASK)
            >> H264Dma2dOutCh4IntMap::CORE1_H264_DMA2D_OUT_CH4_INT_MAP_OFFSET
    }

    /// Set **CORE1_H264_DMA2D_OUT_CH4_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_h264_dma2d_out_ch4_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !H264Dma2dOutCh4IntMap::CORE1_H264_DMA2D_OUT_CH4_INT_MAP_MASK)
                | ((value << H264Dma2dOutCh4IntMap::CORE1_H264_DMA2D_OUT_CH4_INT_MAP_OFFSET)
                    & H264Dma2dOutCh4IntMap::CORE1_H264_DMA2D_OUT_CH4_INT_MAP_MASK),
        }
    }
}

impl Default for H264Dma2dOutCh4IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl H264Dma2dOutCh4IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> H264Dma2dOutCh4IntMapVal {
        H264Dma2dOutCh4IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: H264Dma2dOutCh4IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(H264Dma2dOutCh4IntMapVal) -> H264Dma2dOutCh4IntMapVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// H264_DMA2D_IN_CH0_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01e0
///
/// ## Fields
///
/// - **CORE1_H264_DMA2D_IN_CH0_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct H264Dma2dInCh0IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for H264Dma2dInCh0IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl H264Dma2dInCh0IntMap {
    // ======== Field Constants ========

    /// **CORE1_H264_DMA2D_IN_CH0_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_H264_DMA2D_IN_CH0_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_H264_DMA2D_IN_CH0_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_H264_DMA2D_IN_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_h264_dma2d_in_ch0_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_H264_DMA2D_IN_CH0_INT_MAP_MASK)
            >> Self::CORE1_H264_DMA2D_IN_CH0_INT_MAP_OFFSET
    }

    /// Write **CORE1_H264_DMA2D_IN_CH0_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_h264_dma2d_in_ch0_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_H264_DMA2D_IN_CH0_INT_MAP_MASK)
                | ((value << Self::CORE1_H264_DMA2D_IN_CH0_INT_MAP_OFFSET)
                    & Self::CORE1_H264_DMA2D_IN_CH0_INT_MAP_MASK)
        });
    }
}

/// H264_DMA2D_IN_CH0_INT_MAP value type for batch read/modify/write operations
///
/// Use `h264dma2dinch0intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `h264dma2dinch0intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct H264Dma2dInCh0IntMapVal {
    bits: u32,
}

impl H264Dma2dInCh0IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_H264_DMA2D_IN_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_h264_dma2d_in_ch0_int_map(&self) -> u32 {
        (self.bits & H264Dma2dInCh0IntMap::CORE1_H264_DMA2D_IN_CH0_INT_MAP_MASK)
            >> H264Dma2dInCh0IntMap::CORE1_H264_DMA2D_IN_CH0_INT_MAP_OFFSET
    }

    /// Set **CORE1_H264_DMA2D_IN_CH0_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_h264_dma2d_in_ch0_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !H264Dma2dInCh0IntMap::CORE1_H264_DMA2D_IN_CH0_INT_MAP_MASK)
                | ((value << H264Dma2dInCh0IntMap::CORE1_H264_DMA2D_IN_CH0_INT_MAP_OFFSET)
                    & H264Dma2dInCh0IntMap::CORE1_H264_DMA2D_IN_CH0_INT_MAP_MASK),
        }
    }
}

impl Default for H264Dma2dInCh0IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl H264Dma2dInCh0IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> H264Dma2dInCh0IntMapVal {
        H264Dma2dInCh0IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: H264Dma2dInCh0IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(H264Dma2dInCh0IntMapVal) -> H264Dma2dInCh0IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// H264_DMA2D_IN_CH1_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01e4
///
/// ## Fields
///
/// - **CORE1_H264_DMA2D_IN_CH1_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct H264Dma2dInCh1IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for H264Dma2dInCh1IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl H264Dma2dInCh1IntMap {
    // ======== Field Constants ========

    /// **CORE1_H264_DMA2D_IN_CH1_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_H264_DMA2D_IN_CH1_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_H264_DMA2D_IN_CH1_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_H264_DMA2D_IN_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_h264_dma2d_in_ch1_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_H264_DMA2D_IN_CH1_INT_MAP_MASK)
            >> Self::CORE1_H264_DMA2D_IN_CH1_INT_MAP_OFFSET
    }

    /// Write **CORE1_H264_DMA2D_IN_CH1_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_h264_dma2d_in_ch1_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_H264_DMA2D_IN_CH1_INT_MAP_MASK)
                | ((value << Self::CORE1_H264_DMA2D_IN_CH1_INT_MAP_OFFSET)
                    & Self::CORE1_H264_DMA2D_IN_CH1_INT_MAP_MASK)
        });
    }
}

/// H264_DMA2D_IN_CH1_INT_MAP value type for batch read/modify/write operations
///
/// Use `h264dma2dinch1intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `h264dma2dinch1intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct H264Dma2dInCh1IntMapVal {
    bits: u32,
}

impl H264Dma2dInCh1IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_H264_DMA2D_IN_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_h264_dma2d_in_ch1_int_map(&self) -> u32 {
        (self.bits & H264Dma2dInCh1IntMap::CORE1_H264_DMA2D_IN_CH1_INT_MAP_MASK)
            >> H264Dma2dInCh1IntMap::CORE1_H264_DMA2D_IN_CH1_INT_MAP_OFFSET
    }

    /// Set **CORE1_H264_DMA2D_IN_CH1_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_h264_dma2d_in_ch1_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !H264Dma2dInCh1IntMap::CORE1_H264_DMA2D_IN_CH1_INT_MAP_MASK)
                | ((value << H264Dma2dInCh1IntMap::CORE1_H264_DMA2D_IN_CH1_INT_MAP_OFFSET)
                    & H264Dma2dInCh1IntMap::CORE1_H264_DMA2D_IN_CH1_INT_MAP_MASK),
        }
    }
}

impl Default for H264Dma2dInCh1IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl H264Dma2dInCh1IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> H264Dma2dInCh1IntMapVal {
        H264Dma2dInCh1IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: H264Dma2dInCh1IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(H264Dma2dInCh1IntMapVal) -> H264Dma2dInCh1IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// H264_DMA2D_IN_CH2_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01e8
///
/// ## Fields
///
/// - **CORE1_H264_DMA2D_IN_CH2_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct H264Dma2dInCh2IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for H264Dma2dInCh2IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl H264Dma2dInCh2IntMap {
    // ======== Field Constants ========

    /// **CORE1_H264_DMA2D_IN_CH2_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_H264_DMA2D_IN_CH2_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_H264_DMA2D_IN_CH2_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_H264_DMA2D_IN_CH2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_h264_dma2d_in_ch2_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_H264_DMA2D_IN_CH2_INT_MAP_MASK)
            >> Self::CORE1_H264_DMA2D_IN_CH2_INT_MAP_OFFSET
    }

    /// Write **CORE1_H264_DMA2D_IN_CH2_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_h264_dma2d_in_ch2_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_H264_DMA2D_IN_CH2_INT_MAP_MASK)
                | ((value << Self::CORE1_H264_DMA2D_IN_CH2_INT_MAP_OFFSET)
                    & Self::CORE1_H264_DMA2D_IN_CH2_INT_MAP_MASK)
        });
    }
}

/// H264_DMA2D_IN_CH2_INT_MAP value type for batch read/modify/write operations
///
/// Use `h264dma2dinch2intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `h264dma2dinch2intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct H264Dma2dInCh2IntMapVal {
    bits: u32,
}

impl H264Dma2dInCh2IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_H264_DMA2D_IN_CH2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_h264_dma2d_in_ch2_int_map(&self) -> u32 {
        (self.bits & H264Dma2dInCh2IntMap::CORE1_H264_DMA2D_IN_CH2_INT_MAP_MASK)
            >> H264Dma2dInCh2IntMap::CORE1_H264_DMA2D_IN_CH2_INT_MAP_OFFSET
    }

    /// Set **CORE1_H264_DMA2D_IN_CH2_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_h264_dma2d_in_ch2_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !H264Dma2dInCh2IntMap::CORE1_H264_DMA2D_IN_CH2_INT_MAP_MASK)
                | ((value << H264Dma2dInCh2IntMap::CORE1_H264_DMA2D_IN_CH2_INT_MAP_OFFSET)
                    & H264Dma2dInCh2IntMap::CORE1_H264_DMA2D_IN_CH2_INT_MAP_MASK),
        }
    }
}

impl Default for H264Dma2dInCh2IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl H264Dma2dInCh2IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> H264Dma2dInCh2IntMapVal {
        H264Dma2dInCh2IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: H264Dma2dInCh2IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(H264Dma2dInCh2IntMapVal) -> H264Dma2dInCh2IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// H264_DMA2D_IN_CH3_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01ec
///
/// ## Fields
///
/// - **CORE1_H264_DMA2D_IN_CH3_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct H264Dma2dInCh3IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for H264Dma2dInCh3IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl H264Dma2dInCh3IntMap {
    // ======== Field Constants ========

    /// **CORE1_H264_DMA2D_IN_CH3_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_H264_DMA2D_IN_CH3_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_H264_DMA2D_IN_CH3_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_H264_DMA2D_IN_CH3_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_h264_dma2d_in_ch3_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_H264_DMA2D_IN_CH3_INT_MAP_MASK)
            >> Self::CORE1_H264_DMA2D_IN_CH3_INT_MAP_OFFSET
    }

    /// Write **CORE1_H264_DMA2D_IN_CH3_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_h264_dma2d_in_ch3_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_H264_DMA2D_IN_CH3_INT_MAP_MASK)
                | ((value << Self::CORE1_H264_DMA2D_IN_CH3_INT_MAP_OFFSET)
                    & Self::CORE1_H264_DMA2D_IN_CH3_INT_MAP_MASK)
        });
    }
}

/// H264_DMA2D_IN_CH3_INT_MAP value type for batch read/modify/write operations
///
/// Use `h264dma2dinch3intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `h264dma2dinch3intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct H264Dma2dInCh3IntMapVal {
    bits: u32,
}

impl H264Dma2dInCh3IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_H264_DMA2D_IN_CH3_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_h264_dma2d_in_ch3_int_map(&self) -> u32 {
        (self.bits & H264Dma2dInCh3IntMap::CORE1_H264_DMA2D_IN_CH3_INT_MAP_MASK)
            >> H264Dma2dInCh3IntMap::CORE1_H264_DMA2D_IN_CH3_INT_MAP_OFFSET
    }

    /// Set **CORE1_H264_DMA2D_IN_CH3_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_h264_dma2d_in_ch3_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !H264Dma2dInCh3IntMap::CORE1_H264_DMA2D_IN_CH3_INT_MAP_MASK)
                | ((value << H264Dma2dInCh3IntMap::CORE1_H264_DMA2D_IN_CH3_INT_MAP_OFFSET)
                    & H264Dma2dInCh3IntMap::CORE1_H264_DMA2D_IN_CH3_INT_MAP_MASK),
        }
    }
}

impl Default for H264Dma2dInCh3IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl H264Dma2dInCh3IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> H264Dma2dInCh3IntMapVal {
        H264Dma2dInCh3IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: H264Dma2dInCh3IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(H264Dma2dInCh3IntMapVal) -> H264Dma2dInCh3IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// H264_DMA2D_IN_CH4_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01f0
///
/// ## Fields
///
/// - **CORE1_H264_DMA2D_IN_CH4_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct H264Dma2dInCh4IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for H264Dma2dInCh4IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl H264Dma2dInCh4IntMap {
    // ======== Field Constants ========

    /// **CORE1_H264_DMA2D_IN_CH4_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_H264_DMA2D_IN_CH4_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_H264_DMA2D_IN_CH4_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_H264_DMA2D_IN_CH4_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_h264_dma2d_in_ch4_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_H264_DMA2D_IN_CH4_INT_MAP_MASK)
            >> Self::CORE1_H264_DMA2D_IN_CH4_INT_MAP_OFFSET
    }

    /// Write **CORE1_H264_DMA2D_IN_CH4_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_h264_dma2d_in_ch4_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_H264_DMA2D_IN_CH4_INT_MAP_MASK)
                | ((value << Self::CORE1_H264_DMA2D_IN_CH4_INT_MAP_OFFSET)
                    & Self::CORE1_H264_DMA2D_IN_CH4_INT_MAP_MASK)
        });
    }
}

/// H264_DMA2D_IN_CH4_INT_MAP value type for batch read/modify/write operations
///
/// Use `h264dma2dinch4intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `h264dma2dinch4intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct H264Dma2dInCh4IntMapVal {
    bits: u32,
}

impl H264Dma2dInCh4IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_H264_DMA2D_IN_CH4_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_h264_dma2d_in_ch4_int_map(&self) -> u32 {
        (self.bits & H264Dma2dInCh4IntMap::CORE1_H264_DMA2D_IN_CH4_INT_MAP_MASK)
            >> H264Dma2dInCh4IntMap::CORE1_H264_DMA2D_IN_CH4_INT_MAP_OFFSET
    }

    /// Set **CORE1_H264_DMA2D_IN_CH4_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_h264_dma2d_in_ch4_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !H264Dma2dInCh4IntMap::CORE1_H264_DMA2D_IN_CH4_INT_MAP_MASK)
                | ((value << H264Dma2dInCh4IntMap::CORE1_H264_DMA2D_IN_CH4_INT_MAP_OFFSET)
                    & H264Dma2dInCh4IntMap::CORE1_H264_DMA2D_IN_CH4_INT_MAP_MASK),
        }
    }
}

impl Default for H264Dma2dInCh4IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl H264Dma2dInCh4IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> H264Dma2dInCh4IntMapVal {
        H264Dma2dInCh4IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: H264Dma2dInCh4IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(H264Dma2dInCh4IntMapVal) -> H264Dma2dInCh4IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// H264_DMA2D_IN_CH5_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01f4
///
/// ## Fields
///
/// - **CORE1_H264_DMA2D_IN_CH5_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct H264Dma2dInCh5IntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for H264Dma2dInCh5IntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl H264Dma2dInCh5IntMap {
    // ======== Field Constants ========

    /// **CORE1_H264_DMA2D_IN_CH5_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_H264_DMA2D_IN_CH5_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_H264_DMA2D_IN_CH5_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_H264_DMA2D_IN_CH5_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_h264_dma2d_in_ch5_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_H264_DMA2D_IN_CH5_INT_MAP_MASK)
            >> Self::CORE1_H264_DMA2D_IN_CH5_INT_MAP_OFFSET
    }

    /// Write **CORE1_H264_DMA2D_IN_CH5_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_h264_dma2d_in_ch5_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_H264_DMA2D_IN_CH5_INT_MAP_MASK)
                | ((value << Self::CORE1_H264_DMA2D_IN_CH5_INT_MAP_OFFSET)
                    & Self::CORE1_H264_DMA2D_IN_CH5_INT_MAP_MASK)
        });
    }
}

/// H264_DMA2D_IN_CH5_INT_MAP value type for batch read/modify/write operations
///
/// Use `h264dma2dinch5intmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `h264dma2dinch5intmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct H264Dma2dInCh5IntMapVal {
    bits: u32,
}

impl H264Dma2dInCh5IntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_H264_DMA2D_IN_CH5_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_h264_dma2d_in_ch5_int_map(&self) -> u32 {
        (self.bits & H264Dma2dInCh5IntMap::CORE1_H264_DMA2D_IN_CH5_INT_MAP_MASK)
            >> H264Dma2dInCh5IntMap::CORE1_H264_DMA2D_IN_CH5_INT_MAP_OFFSET
    }

    /// Set **CORE1_H264_DMA2D_IN_CH5_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_h264_dma2d_in_ch5_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !H264Dma2dInCh5IntMap::CORE1_H264_DMA2D_IN_CH5_INT_MAP_MASK)
                | ((value << H264Dma2dInCh5IntMap::CORE1_H264_DMA2D_IN_CH5_INT_MAP_OFFSET)
                    & H264Dma2dInCh5IntMap::CORE1_H264_DMA2D_IN_CH5_INT_MAP_MASK),
        }
    }
}

impl Default for H264Dma2dInCh5IntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl H264Dma2dInCh5IntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> H264Dma2dInCh5IntMapVal {
        H264Dma2dInCh5IntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: H264Dma2dInCh5IntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(H264Dma2dInCh5IntMapVal) -> H264Dma2dInCh5IntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// H264_REG_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01f8
///
/// ## Fields
///
/// - **CORE1_H264_REG_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct H264RegIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for H264RegIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl H264RegIntMap {
    // ======== Field Constants ========

    /// **CORE1_H264_REG_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_H264_REG_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_H264_REG_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_H264_REG_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_h264_reg_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_H264_REG_INT_MAP_MASK)
            >> Self::CORE1_H264_REG_INT_MAP_OFFSET
    }

    /// Write **CORE1_H264_REG_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_h264_reg_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_H264_REG_INT_MAP_MASK)
                | ((value << Self::CORE1_H264_REG_INT_MAP_OFFSET)
                    & Self::CORE1_H264_REG_INT_MAP_MASK)
        });
    }
}

/// H264_REG_INT_MAP value type for batch read/modify/write operations
///
/// Use `h264regintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `h264regintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct H264RegIntMapVal {
    bits: u32,
}

impl H264RegIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_H264_REG_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_h264_reg_int_map(&self) -> u32 {
        (self.bits & H264RegIntMap::CORE1_H264_REG_INT_MAP_MASK)
            >> H264RegIntMap::CORE1_H264_REG_INT_MAP_OFFSET
    }

    /// Set **CORE1_H264_REG_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_h264_reg_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !H264RegIntMap::CORE1_H264_REG_INT_MAP_MASK)
                | ((value << H264RegIntMap::CORE1_H264_REG_INT_MAP_OFFSET)
                    & H264RegIntMap::CORE1_H264_REG_INT_MAP_MASK),
        }
    }
}

impl Default for H264RegIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl H264RegIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> H264RegIntMapVal {
        H264RegIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: H264RegIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(H264RegIntMapVal) -> H264RegIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ASSIST_DEBUG_INT_MAP Register
///
/// NA
///
/// Address offset: 0x01fc
///
/// ## Fields
///
/// - **CORE1_ASSIST_DEBUG_INT_MAP** `[5:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct AssistDebugIntMap {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AssistDebugIntMap {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AssistDebugIntMap {
    // ======== Field Constants ========

    /// **CORE1_ASSIST_DEBUG_INT_MAP**
    ///
    /// NA
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const CORE1_ASSIST_DEBUG_INT_MAP_OFFSET: u32 = 0;
    pub const CORE1_ASSIST_DEBUG_INT_MAP_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **CORE1_ASSIST_DEBUG_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_assist_debug_int_map(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_ASSIST_DEBUG_INT_MAP_MASK)
            >> Self::CORE1_ASSIST_DEBUG_INT_MAP_OFFSET
    }

    /// Write **CORE1_ASSIST_DEBUG_INT_MAP** `[5:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_assist_debug_int_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_ASSIST_DEBUG_INT_MAP_MASK)
                | ((value << Self::CORE1_ASSIST_DEBUG_INT_MAP_OFFSET)
                    & Self::CORE1_ASSIST_DEBUG_INT_MAP_MASK)
        });
    }
}

/// ASSIST_DEBUG_INT_MAP value type for batch read/modify/write operations
///
/// Use `assistdebugintmap.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `assistdebugintmap.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AssistDebugIntMapVal {
    bits: u32,
}

impl AssistDebugIntMapVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_ASSIST_DEBUG_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_assist_debug_int_map(&self) -> u32 {
        (self.bits & AssistDebugIntMap::CORE1_ASSIST_DEBUG_INT_MAP_MASK)
            >> AssistDebugIntMap::CORE1_ASSIST_DEBUG_INT_MAP_OFFSET
    }

    /// Set **CORE1_ASSIST_DEBUG_INT_MAP** `[5:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_assist_debug_int_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AssistDebugIntMap::CORE1_ASSIST_DEBUG_INT_MAP_MASK)
                | ((value << AssistDebugIntMap::CORE1_ASSIST_DEBUG_INT_MAP_OFFSET)
                    & AssistDebugIntMap::CORE1_ASSIST_DEBUG_INT_MAP_MASK),
        }
    }
}

impl Default for AssistDebugIntMapVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AssistDebugIntMap {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AssistDebugIntMapVal {
        AssistDebugIntMapVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AssistDebugIntMapVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AssistDebugIntMapVal) -> AssistDebugIntMapVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTR_STATUS_REG_0 Register
///
/// NA
///
/// Address offset: 0x0200
///
/// ## Fields
///
/// - **CORE1_INTR_STATUS_0** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntrStatusReg0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntrStatusReg0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntrStatusReg0 {
    // ======== Field Constants ========

    /// **CORE1_INTR_STATUS_0**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE1_INTR_STATUS_0_OFFSET: u32 = 0;
    pub const CORE1_INTR_STATUS_0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE1_INTR_STATUS_0** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_intr_status_0(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_INTR_STATUS_0_MASK)
            >> Self::CORE1_INTR_STATUS_0_OFFSET
    }
}

/// INTR_STATUS_REG_0 value type for batch read/modify/write operations
///
/// Use `intrstatusreg0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intrstatusreg0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntrStatusReg0Val {
    bits: u32,
}

impl IntrStatusReg0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_INTR_STATUS_0** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_intr_status_0(&self) -> u32 {
        (self.bits & IntrStatusReg0::CORE1_INTR_STATUS_0_MASK)
            >> IntrStatusReg0::CORE1_INTR_STATUS_0_OFFSET
    }
}

impl Default for IntrStatusReg0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntrStatusReg0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntrStatusReg0Val {
        IntrStatusReg0Val::from_bits(self.value.read_bits())
    }
}

/// INTR_STATUS_REG_1 Register
///
/// NA
///
/// Address offset: 0x0204
///
/// ## Fields
///
/// - **CORE1_INTR_STATUS_1** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntrStatusReg1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntrStatusReg1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntrStatusReg1 {
    // ======== Field Constants ========

    /// **CORE1_INTR_STATUS_1**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE1_INTR_STATUS_1_OFFSET: u32 = 0;
    pub const CORE1_INTR_STATUS_1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE1_INTR_STATUS_1** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_intr_status_1(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_INTR_STATUS_1_MASK)
            >> Self::CORE1_INTR_STATUS_1_OFFSET
    }
}

/// INTR_STATUS_REG_1 value type for batch read/modify/write operations
///
/// Use `intrstatusreg1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intrstatusreg1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntrStatusReg1Val {
    bits: u32,
}

impl IntrStatusReg1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_INTR_STATUS_1** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_intr_status_1(&self) -> u32 {
        (self.bits & IntrStatusReg1::CORE1_INTR_STATUS_1_MASK)
            >> IntrStatusReg1::CORE1_INTR_STATUS_1_OFFSET
    }
}

impl Default for IntrStatusReg1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntrStatusReg1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntrStatusReg1Val {
        IntrStatusReg1Val::from_bits(self.value.read_bits())
    }
}

/// INTR_STATUS_REG_2 Register
///
/// NA
///
/// Address offset: 0x0208
///
/// ## Fields
///
/// - **CORE1_INTR_STATUS_2** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntrStatusReg2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntrStatusReg2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntrStatusReg2 {
    // ======== Field Constants ========

    /// **CORE1_INTR_STATUS_2**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE1_INTR_STATUS_2_OFFSET: u32 = 0;
    pub const CORE1_INTR_STATUS_2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE1_INTR_STATUS_2** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_intr_status_2(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_INTR_STATUS_2_MASK)
            >> Self::CORE1_INTR_STATUS_2_OFFSET
    }
}

/// INTR_STATUS_REG_2 value type for batch read/modify/write operations
///
/// Use `intrstatusreg2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intrstatusreg2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntrStatusReg2Val {
    bits: u32,
}

impl IntrStatusReg2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_INTR_STATUS_2** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_intr_status_2(&self) -> u32 {
        (self.bits & IntrStatusReg2::CORE1_INTR_STATUS_2_MASK)
            >> IntrStatusReg2::CORE1_INTR_STATUS_2_OFFSET
    }
}

impl Default for IntrStatusReg2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntrStatusReg2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntrStatusReg2Val {
        IntrStatusReg2Val::from_bits(self.value.read_bits())
    }
}

/// INTR_STATUS_REG_3 Register
///
/// NA
///
/// Address offset: 0x020c
///
/// ## Fields
///
/// - **CORE1_INTR_STATUS_3** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct IntrStatusReg3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntrStatusReg3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntrStatusReg3 {
    // ======== Field Constants ========

    /// **CORE1_INTR_STATUS_3**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const CORE1_INTR_STATUS_3_OFFSET: u32 = 0;
    pub const CORE1_INTR_STATUS_3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **CORE1_INTR_STATUS_3** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_intr_status_3(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_INTR_STATUS_3_MASK)
            >> Self::CORE1_INTR_STATUS_3_OFFSET
    }
}

/// INTR_STATUS_REG_3 value type for batch read/modify/write operations
///
/// Use `intrstatusreg3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intrstatusreg3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntrStatusReg3Val {
    bits: u32,
}

impl IntrStatusReg3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CORE1_INTR_STATUS_3** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_intr_status_3(&self) -> u32 {
        (self.bits & IntrStatusReg3::CORE1_INTR_STATUS_3_MASK)
            >> IntrStatusReg3::CORE1_INTR_STATUS_3_OFFSET
    }
}

impl Default for IntrStatusReg3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntrStatusReg3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IntrStatusReg3Val {
        IntrStatusReg3Val::from_bits(self.value.read_bits())
    }
}

/// CLOCK_GATE Register
///
/// NA
///
/// Address offset: 0x0210
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CORE1_REG_CLK_EN** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct ClockGate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClockGate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClockGate {
    // ======== Field Constants ========

    /// **CORE1_REG_CLK_EN**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE1_REG_CLK_EN_OFFSET: u32 = 0;
    pub const CORE1_REG_CLK_EN_MASK: u32 = 0x00000001;
    pub const CORE1_REG_CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE1_REG_CLK_EN** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_reg_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_REG_CLK_EN_MASK) != 0
    }

    /// Set CORE1_REG_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core1_reg_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::CORE1_REG_CLK_EN_MASK);
    }

    /// Clear CORE1_REG_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core1_reg_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::CORE1_REG_CLK_EN_MASK);
    }
}

/// CLOCK_GATE value type for batch read/modify/write operations
///
/// Use `clockgate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clockgate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClockGateVal {
    bits: u32,
}

impl ClockGateVal {
    /// Create from raw bits
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

    /// Read **CORE1_REG_CLK_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_reg_clk_en(&self) -> bool {
        (self.bits & ClockGate::CORE1_REG_CLK_EN_MASK) != 0
    }

    /// Set **CORE1_REG_CLK_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_reg_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClockGate::CORE1_REG_CLK_EN_MASK
            } else {
                self.bits & !ClockGate::CORE1_REG_CLK_EN_MASK
            },
        }
    }
}

impl Default for ClockGateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClockGate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClockGateVal {
        ClockGateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClockGateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClockGateVal) -> ClockGateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INTERRUPT_REG_DATE Register
///
/// NA
///
/// Address offset: 0x03fc
/// Reset value: 0x02003020
///
/// ## Fields
///
/// - **CORE1_INTERRUPT_REG_DATE** `[27:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct InterruptRegDate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InterruptRegDate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InterruptRegDate {
    // ======== Field Constants ========

    /// **CORE1_INTERRUPT_REG_DATE**
    ///
    /// NA
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const CORE1_INTERRUPT_REG_DATE_OFFSET: u32 = 0;
    pub const CORE1_INTERRUPT_REG_DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **CORE1_INTERRUPT_REG_DATE** `[27:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn core1_interrupt_reg_date(&self) -> u32 {
        (self.value.read_bits() & Self::CORE1_INTERRUPT_REG_DATE_MASK)
            >> Self::CORE1_INTERRUPT_REG_DATE_OFFSET
    }

    /// Write **CORE1_INTERRUPT_REG_DATE** `[27:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_core1_interrupt_reg_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE1_INTERRUPT_REG_DATE_MASK)
                | ((value << Self::CORE1_INTERRUPT_REG_DATE_OFFSET)
                    & Self::CORE1_INTERRUPT_REG_DATE_MASK)
        });
    }
}

/// INTERRUPT_REG_DATE value type for batch read/modify/write operations
///
/// Use `interruptregdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `interruptregdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct InterruptRegDateVal {
    bits: u32,
}

impl InterruptRegDateVal {
    /// Create from raw bits
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
        Self { bits: 0x02003020 }
    }

    /// Read **CORE1_INTERRUPT_REG_DATE** `[27:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn core1_interrupt_reg_date(&self) -> u32 {
        (self.bits & InterruptRegDate::CORE1_INTERRUPT_REG_DATE_MASK)
            >> InterruptRegDate::CORE1_INTERRUPT_REG_DATE_OFFSET
    }

    /// Set **CORE1_INTERRUPT_REG_DATE** `[27:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_core1_interrupt_reg_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !InterruptRegDate::CORE1_INTERRUPT_REG_DATE_MASK)
                | ((value << InterruptRegDate::CORE1_INTERRUPT_REG_DATE_OFFSET)
                    & InterruptRegDate::CORE1_INTERRUPT_REG_DATE_MASK),
        }
    }
}

impl Default for InterruptRegDateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl InterruptRegDate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> InterruptRegDateVal {
        InterruptRegDateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: InterruptRegDateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(InterruptRegDateVal) -> InterruptRegDateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// NA
    pub lp_rtc_int_map: LpRtcIntMap,
    /// 0x0004
    /// NA
    pub lp_wdt_int_map: LpWdtIntMap,
    /// 0x0008
    /// NA
    pub lp_timer_reg0_int_map: LpTimerReg0IntMap,
    /// 0x000c
    /// NA
    pub lp_timer_reg1_int_map: LpTimerReg1IntMap,
    /// 0x0010
    /// NA
    pub mb_hp_int_map: MbHpIntMap,
    /// 0x0014
    /// NA
    pub mb_lp_int_map: MbLpIntMap,
    /// 0x0018
    /// NA
    pub pmu_reg0_int_map: PmuReg0IntMap,
    /// 0x001c
    /// NA
    pub pmu_reg1_int_map: PmuReg1IntMap,
    /// 0x0020
    /// NA
    pub lp_anaperi_int_map: LpAnaperiIntMap,
    /// 0x0024
    /// NA
    pub lp_adc_int_map: LpAdcIntMap,
    /// 0x0028
    /// NA
    pub lp_gpio_int_map: LpGpioIntMap,
    /// 0x002c
    /// NA
    pub lp_i2c_int_map: LpI2cIntMap,
    /// 0x0030
    /// NA
    pub lp_i2s_int_map: LpI2sIntMap,
    /// 0x0034
    /// NA
    pub lp_spi_int_map: LpSpiIntMap,
    /// 0x0038
    /// NA
    pub lp_touch_int_map: LpTouchIntMap,
    /// 0x003c
    /// NA
    pub lp_tsens_int_map: LpTsensIntMap,
    /// 0x0040
    /// NA
    pub lp_uart_int_map: LpUartIntMap,
    /// 0x0044
    /// NA
    pub lp_efuse_int_map: LpEfuseIntMap,
    /// 0x0048
    /// NA
    pub lp_sw_int_map: LpSwIntMap,
    /// 0x004c
    /// NA
    pub lp_sysreg_int_map: LpSysregIntMap,
    /// 0x0050
    /// NA
    pub lp_huk_int_map: LpHukIntMap,
    /// 0x0054
    /// NA
    pub sys_icm_int_map: SysIcmIntMap,
    /// 0x0058
    /// NA
    pub usb_device_int_map: UsbDeviceIntMap,
    /// 0x005c
    /// NA
    pub sdio_host_int_map: SdioHostIntMap,
    /// 0x0060
    /// NA
    pub gdma_int_map: GdmaIntMap,
    /// 0x0064
    /// NA
    pub spi2_int_map: Spi2IntMap,
    /// 0x0068
    /// NA
    pub spi3_int_map: Spi3IntMap,
    /// 0x006c
    /// NA
    pub i2s0_int_map: I2s0IntMap,
    /// 0x0070
    /// NA
    pub i2s1_int_map: I2s1IntMap,
    /// 0x0074
    /// NA
    pub i2s2_int_map: I2s2IntMap,
    /// 0x0078
    /// NA
    pub uhci0_int_map: Uhci0IntMap,
    /// 0x007c
    /// NA
    pub uart0_int_map: Uart0IntMap,
    /// 0x0080
    /// NA
    pub uart1_int_map: Uart1IntMap,
    /// 0x0084
    /// NA
    pub uart2_int_map: Uart2IntMap,
    /// 0x0088
    /// NA
    pub uart3_int_map: Uart3IntMap,
    /// 0x008c
    /// NA
    pub uart4_int_map: Uart4IntMap,
    /// 0x0090
    /// NA
    pub lcd_cam_int_map: LcdCamIntMap,
    /// 0x0094
    /// NA
    pub adc_int_map: AdcIntMap,
    /// 0x0098
    /// NA
    pub pwm0_int_map: Pwm0IntMap,
    /// 0x009c
    /// NA
    pub pwm1_int_map: Pwm1IntMap,
    /// 0x00a0
    /// NA
    pub can0_int_map: Can0IntMap,
    /// 0x00a4
    /// NA
    pub can1_int_map: Can1IntMap,
    /// 0x00a8
    /// NA
    pub can2_int_map: Can2IntMap,
    /// 0x00ac
    /// NA
    pub rmt_int_map: RmtIntMap,
    /// 0x00b0
    /// NA
    pub i2c0_int_map: I2c0IntMap,
    /// 0x00b4
    /// NA
    pub i2c1_int_map: I2c1IntMap,
    /// 0x00b8
    /// NA
    pub timergrp0_t0_int_map: Timergrp0T0IntMap,
    /// 0x00bc
    /// NA
    pub timergrp0_t1_int_map: Timergrp0T1IntMap,
    /// 0x00c0
    /// NA
    pub timergrp0_wdt_int_map: Timergrp0WdtIntMap,
    /// 0x00c4
    /// NA
    pub timergrp1_t0_int_map: Timergrp1T0IntMap,
    /// 0x00c8
    /// NA
    pub timergrp1_t1_int_map: Timergrp1T1IntMap,
    /// 0x00cc
    /// NA
    pub timergrp1_wdt_int_map: Timergrp1WdtIntMap,
    /// 0x00d0
    /// NA
    pub ledc_int_map: LedcIntMap,
    /// 0x00d4
    /// NA
    pub systimer_target0_int_map: SystimerTarget0IntMap,
    /// 0x00d8
    /// NA
    pub systimer_target1_int_map: SystimerTarget1IntMap,
    /// 0x00dc
    /// NA
    pub systimer_target2_int_map: SystimerTarget2IntMap,
    /// 0x00e0
    /// NA
    pub ahb_pdma_in_ch0_int_map: AhbPdmaInCh0IntMap,
    /// 0x00e4
    /// NA
    pub ahb_pdma_in_ch1_int_map: AhbPdmaInCh1IntMap,
    /// 0x00e8
    /// NA
    pub ahb_pdma_in_ch2_int_map: AhbPdmaInCh2IntMap,
    /// 0x00ec
    /// NA
    pub ahb_pdma_out_ch0_int_map: AhbPdmaOutCh0IntMap,
    /// 0x00f0
    /// NA
    pub ahb_pdma_out_ch1_int_map: AhbPdmaOutCh1IntMap,
    /// 0x00f4
    /// NA
    pub ahb_pdma_out_ch2_int_map: AhbPdmaOutCh2IntMap,
    /// 0x00f8
    /// NA
    pub axi_pdma_in_ch0_int_map: AxiPdmaInCh0IntMap,
    /// 0x00fc
    /// NA
    pub axi_pdma_in_ch1_int_map: AxiPdmaInCh1IntMap,
    /// 0x0100
    /// NA
    pub axi_pdma_in_ch2_int_map: AxiPdmaInCh2IntMap,
    /// 0x0104
    /// NA
    pub axi_pdma_out_ch0_int_map: AxiPdmaOutCh0IntMap,
    /// 0x0108
    /// NA
    pub axi_pdma_out_ch1_int_map: AxiPdmaOutCh1IntMap,
    /// 0x010c
    /// NA
    pub axi_pdma_out_ch2_int_map: AxiPdmaOutCh2IntMap,
    /// 0x0110
    /// NA
    pub rsa_int_map: RsaIntMap,
    /// 0x0114
    /// NA
    pub aes_int_map: AesIntMap,
    /// 0x0118
    /// NA
    pub sha_int_map: ShaIntMap,
    /// 0x011c
    /// NA
    pub ecc_int_map: EccIntMap,
    /// 0x0120
    /// NA
    pub ecdsa_int_map: EcdsaIntMap,
    /// 0x0124
    /// NA
    pub km_int_map: KmIntMap,
    /// 0x0128
    /// NA
    pub gpio_int0_map: GpioInt0Map,
    /// 0x012c
    /// NA
    pub gpio_int1_map: GpioInt1Map,
    /// 0x0130
    /// NA
    pub gpio_int2_map: GpioInt2Map,
    /// 0x0134
    /// NA
    pub gpio_int3_map: GpioInt3Map,
    /// 0x0138
    /// NA
    pub gpio_pad_comp_int_map: GpioPadCompIntMap,
    /// 0x013c
    /// NA
    pub cpu_int_from_cpu0_map: CpuIntFromCpu0Map,
    /// 0x0140
    /// NA
    pub cpu_int_from_cpu1_map: CpuIntFromCpu1Map,
    /// 0x0144
    /// NA
    pub cpu_int_from_cpu2_map: CpuIntFromCpu2Map,
    /// 0x0148
    /// NA
    pub cpu_int_from_cpu3_map: CpuIntFromCpu3Map,
    /// 0x014c
    /// NA
    pub cache_int_map: CacheIntMap,
    /// 0x0150
    /// NA
    pub flash_mspi_int_map: FlashMspiIntMap,
    /// 0x0154
    /// NA
    pub csi_bridge_int_map: CsiBridgeIntMap,
    /// 0x0158
    /// NA
    pub dsi_bridge_int_map: DsiBridgeIntMap,
    /// 0x015c
    /// NA
    pub csi_int_map: CsiIntMap,
    /// 0x0160
    /// NA
    pub dsi_int_map: DsiIntMap,
    /// 0x0164
    /// NA
    pub gmii_phy_int_map: GmiiPhyIntMap,
    /// 0x0168
    /// NA
    pub lpi_int_map: LpiIntMap,
    /// 0x016c
    /// NA
    pub pmt_int_map: PmtIntMap,
    /// 0x0170
    /// NA
    pub sbd_int_map: SbdIntMap,
    /// 0x0174
    /// NA
    pub usb_otg_int_map: UsbOtgIntMap,
    /// 0x0178
    /// NA
    pub usb_otg_endp_multi_proc_int_map: UsbOtgEndpMultiProcIntMap,
    /// 0x017c
    /// NA
    pub jpeg_int_map: JpegIntMap,
    /// 0x0180
    /// NA
    pub ppa_int_map: PpaIntMap,
    /// 0x0184
    /// NA
    pub core0_trace_int_map: Core0TraceIntMap,
    /// 0x0188
    /// NA
    pub core1_trace_int_map: Core1TraceIntMap,
    /// 0x018c
    /// NA
    pub hp_core_ctrl_int_map: HpCoreCtrlIntMap,
    /// 0x0190
    /// NA
    pub isp_int_map: IspIntMap,
    /// 0x0194
    /// NA
    pub i3c_mst_int_map: I3cMstIntMap,
    /// 0x0198
    /// NA
    pub i3c_slv_int_map: I3cSlvIntMap,
    /// 0x019c
    /// NA
    pub usb_otg11_int_map: UsbOtg11IntMap,
    /// 0x01a0
    /// NA
    pub dma2d_in_ch0_int_map: Dma2dInCh0IntMap,
    /// 0x01a4
    /// NA
    pub dma2d_in_ch1_int_map: Dma2dInCh1IntMap,
    /// 0x01a8
    /// NA
    pub dma2d_out_ch0_int_map: Dma2dOutCh0IntMap,
    /// 0x01ac
    /// NA
    pub dma2d_out_ch1_int_map: Dma2dOutCh1IntMap,
    /// 0x01b0
    /// NA
    pub dma2d_out_ch2_int_map: Dma2dOutCh2IntMap,
    /// 0x01b4
    /// NA
    pub psram_mspi_int_map: PsramMspiIntMap,
    /// 0x01b8
    /// NA
    pub hp_sysreg_int_map: HpSysregIntMap,
    /// 0x01bc
    /// NA
    pub pcnt_int_map: PcntIntMap,
    /// 0x01c0
    /// NA
    pub hp_pau_int_map: HpPauIntMap,
    /// 0x01c4
    /// NA
    pub hp_parlio_rx_int_map: HpParlioRxIntMap,
    /// 0x01c8
    /// NA
    pub hp_parlio_tx_int_map: HpParlioTxIntMap,
    /// 0x01cc
    /// NA
    pub h264_dma2d_out_ch0_int_map: H264Dma2dOutCh0IntMap,
    /// 0x01d0
    /// NA
    pub h264_dma2d_out_ch1_int_map: H264Dma2dOutCh1IntMap,
    /// 0x01d4
    /// NA
    pub h264_dma2d_out_ch2_int_map: H264Dma2dOutCh2IntMap,
    /// 0x01d8
    /// NA
    pub h264_dma2d_out_ch3_int_map: H264Dma2dOutCh3IntMap,
    /// 0x01dc
    /// NA
    pub h264_dma2d_out_ch4_int_map: H264Dma2dOutCh4IntMap,
    /// 0x01e0
    /// NA
    pub h264_dma2d_in_ch0_int_map: H264Dma2dInCh0IntMap,
    /// 0x01e4
    /// NA
    pub h264_dma2d_in_ch1_int_map: H264Dma2dInCh1IntMap,
    /// 0x01e8
    /// NA
    pub h264_dma2d_in_ch2_int_map: H264Dma2dInCh2IntMap,
    /// 0x01ec
    /// NA
    pub h264_dma2d_in_ch3_int_map: H264Dma2dInCh3IntMap,
    /// 0x01f0
    /// NA
    pub h264_dma2d_in_ch4_int_map: H264Dma2dInCh4IntMap,
    /// 0x01f4
    /// NA
    pub h264_dma2d_in_ch5_int_map: H264Dma2dInCh5IntMap,
    /// 0x01f8
    /// NA
    pub h264_reg_int_map: H264RegIntMap,
    /// 0x01fc
    /// NA
    pub assist_debug_int_map: AssistDebugIntMap,
    /// 0x0200
    /// NA
    pub intr_status_reg0: IntrStatusReg0,
    /// 0x0204
    /// NA
    pub intr_status_reg1: IntrStatusReg1,
    /// 0x0208
    /// NA
    pub intr_status_reg2: IntrStatusReg2,
    /// 0x020c
    /// NA
    pub intr_status_reg3: IntrStatusReg3,
    /// 0x0210
    /// NA
    pub clock_gate: ClockGate,
    _reserved0: [u8; 488],
    /// 0x03fc
    /// NA
    pub interrupt_reg_date: InterruptRegDate,
}
