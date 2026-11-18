//! PVT
//! PVT Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x5009e000;

/// PMUP_BITMAP_HIGH0 Register
///
/// select valid pvt channel
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **PUMP_BITMAP_HIGH0** `[31:0]` (RW)
///
///   select valid high channel0
#[repr(transparent)]
pub struct PmupBitmapHigh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmupBitmapHigh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmupBitmapHigh0 {
    // ======== Field Constants ========

    /// **PUMP_BITMAP_HIGH0**
    ///
    /// select valid high channel0
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PUMP_BITMAP_HIGH0_OFFSET: u32 = 0;
    pub const PUMP_BITMAP_HIGH0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PUMP_BITMAP_HIGH0** `[31:0]` (RW)
    ///
    /// select valid high channel0
    #[inline(always)]
    pub fn pump_bitmap_high0(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_BITMAP_HIGH0_MASK) >> Self::PUMP_BITMAP_HIGH0_OFFSET
    }

    /// Write **PUMP_BITMAP_HIGH0** `[31:0]` (RW)
    ///
    /// select valid high channel0
    #[inline(always)]
    pub fn write_pump_bitmap_high0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_BITMAP_HIGH0_MASK)
                | ((value << Self::PUMP_BITMAP_HIGH0_OFFSET) & Self::PUMP_BITMAP_HIGH0_MASK)
        });
    }
}

/// PMUP_BITMAP_HIGH0 value type for batch read/modify/write operations
///
/// Use `pmupbitmaphigh0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmupbitmaphigh0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmupBitmapHigh0Val {
    bits: u32,
}

impl PmupBitmapHigh0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PUMP_BITMAP_HIGH0** `[31:0]`
    ///
    /// select valid high channel0
    #[inline(always)]
    pub const fn pump_bitmap_high0(&self) -> u32 {
        (self.bits & PmupBitmapHigh0::PUMP_BITMAP_HIGH0_MASK)
            >> PmupBitmapHigh0::PUMP_BITMAP_HIGH0_OFFSET
    }

    /// Set **PUMP_BITMAP_HIGH0** `[31:0]`
    ///
    /// select valid high channel0
    #[inline(always)]
    pub const fn with_pump_bitmap_high0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupBitmapHigh0::PUMP_BITMAP_HIGH0_MASK)
                | ((value << PmupBitmapHigh0::PUMP_BITMAP_HIGH0_OFFSET)
                    & PmupBitmapHigh0::PUMP_BITMAP_HIGH0_MASK),
        }
    }
}

impl Default for PmupBitmapHigh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmupBitmapHigh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmupBitmapHigh0Val {
        PmupBitmapHigh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmupBitmapHigh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmupBitmapHigh0Val) -> PmupBitmapHigh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMUP_BITMAP_HIGH1 Register
///
/// select valid pvt channel
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **PUMP_BITMAP_HIGH1** `[31:0]` (RW)
///
///   select valid high channel1
#[repr(transparent)]
pub struct PmupBitmapHigh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmupBitmapHigh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmupBitmapHigh1 {
    // ======== Field Constants ========

    /// **PUMP_BITMAP_HIGH1**
    ///
    /// select valid high channel1
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PUMP_BITMAP_HIGH1_OFFSET: u32 = 0;
    pub const PUMP_BITMAP_HIGH1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PUMP_BITMAP_HIGH1** `[31:0]` (RW)
    ///
    /// select valid high channel1
    #[inline(always)]
    pub fn pump_bitmap_high1(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_BITMAP_HIGH1_MASK) >> Self::PUMP_BITMAP_HIGH1_OFFSET
    }

    /// Write **PUMP_BITMAP_HIGH1** `[31:0]` (RW)
    ///
    /// select valid high channel1
    #[inline(always)]
    pub fn write_pump_bitmap_high1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_BITMAP_HIGH1_MASK)
                | ((value << Self::PUMP_BITMAP_HIGH1_OFFSET) & Self::PUMP_BITMAP_HIGH1_MASK)
        });
    }
}

/// PMUP_BITMAP_HIGH1 value type for batch read/modify/write operations
///
/// Use `pmupbitmaphigh1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmupbitmaphigh1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmupBitmapHigh1Val {
    bits: u32,
}

impl PmupBitmapHigh1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PUMP_BITMAP_HIGH1** `[31:0]`
    ///
    /// select valid high channel1
    #[inline(always)]
    pub const fn pump_bitmap_high1(&self) -> u32 {
        (self.bits & PmupBitmapHigh1::PUMP_BITMAP_HIGH1_MASK)
            >> PmupBitmapHigh1::PUMP_BITMAP_HIGH1_OFFSET
    }

    /// Set **PUMP_BITMAP_HIGH1** `[31:0]`
    ///
    /// select valid high channel1
    #[inline(always)]
    pub const fn with_pump_bitmap_high1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupBitmapHigh1::PUMP_BITMAP_HIGH1_MASK)
                | ((value << PmupBitmapHigh1::PUMP_BITMAP_HIGH1_OFFSET)
                    & PmupBitmapHigh1::PUMP_BITMAP_HIGH1_MASK),
        }
    }
}

impl Default for PmupBitmapHigh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmupBitmapHigh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmupBitmapHigh1Val {
        PmupBitmapHigh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmupBitmapHigh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmupBitmapHigh1Val) -> PmupBitmapHigh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMUP_BITMAP_HIGH2 Register
///
/// select valid pvt channel
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **PUMP_BITMAP_HIGH2** `[31:0]` (RW)
///
///   select valid high channel2
#[repr(transparent)]
pub struct PmupBitmapHigh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmupBitmapHigh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmupBitmapHigh2 {
    // ======== Field Constants ========

    /// **PUMP_BITMAP_HIGH2**
    ///
    /// select valid high channel2
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PUMP_BITMAP_HIGH2_OFFSET: u32 = 0;
    pub const PUMP_BITMAP_HIGH2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PUMP_BITMAP_HIGH2** `[31:0]` (RW)
    ///
    /// select valid high channel2
    #[inline(always)]
    pub fn pump_bitmap_high2(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_BITMAP_HIGH2_MASK) >> Self::PUMP_BITMAP_HIGH2_OFFSET
    }

    /// Write **PUMP_BITMAP_HIGH2** `[31:0]` (RW)
    ///
    /// select valid high channel2
    #[inline(always)]
    pub fn write_pump_bitmap_high2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_BITMAP_HIGH2_MASK)
                | ((value << Self::PUMP_BITMAP_HIGH2_OFFSET) & Self::PUMP_BITMAP_HIGH2_MASK)
        });
    }
}

/// PMUP_BITMAP_HIGH2 value type for batch read/modify/write operations
///
/// Use `pmupbitmaphigh2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmupbitmaphigh2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmupBitmapHigh2Val {
    bits: u32,
}

impl PmupBitmapHigh2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PUMP_BITMAP_HIGH2** `[31:0]`
    ///
    /// select valid high channel2
    #[inline(always)]
    pub const fn pump_bitmap_high2(&self) -> u32 {
        (self.bits & PmupBitmapHigh2::PUMP_BITMAP_HIGH2_MASK)
            >> PmupBitmapHigh2::PUMP_BITMAP_HIGH2_OFFSET
    }

    /// Set **PUMP_BITMAP_HIGH2** `[31:0]`
    ///
    /// select valid high channel2
    #[inline(always)]
    pub const fn with_pump_bitmap_high2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupBitmapHigh2::PUMP_BITMAP_HIGH2_MASK)
                | ((value << PmupBitmapHigh2::PUMP_BITMAP_HIGH2_OFFSET)
                    & PmupBitmapHigh2::PUMP_BITMAP_HIGH2_MASK),
        }
    }
}

impl Default for PmupBitmapHigh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmupBitmapHigh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmupBitmapHigh2Val {
        PmupBitmapHigh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmupBitmapHigh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmupBitmapHigh2Val) -> PmupBitmapHigh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMUP_BITMAP_HIGH3 Register
///
/// select valid pvt channel
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **PUMP_BITMAP_HIGH3** `[31:0]` (RW)
///
///   select valid high channel3
#[repr(transparent)]
pub struct PmupBitmapHigh3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmupBitmapHigh3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmupBitmapHigh3 {
    // ======== Field Constants ========

    /// **PUMP_BITMAP_HIGH3**
    ///
    /// select valid high channel3
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PUMP_BITMAP_HIGH3_OFFSET: u32 = 0;
    pub const PUMP_BITMAP_HIGH3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PUMP_BITMAP_HIGH3** `[31:0]` (RW)
    ///
    /// select valid high channel3
    #[inline(always)]
    pub fn pump_bitmap_high3(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_BITMAP_HIGH3_MASK) >> Self::PUMP_BITMAP_HIGH3_OFFSET
    }

    /// Write **PUMP_BITMAP_HIGH3** `[31:0]` (RW)
    ///
    /// select valid high channel3
    #[inline(always)]
    pub fn write_pump_bitmap_high3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_BITMAP_HIGH3_MASK)
                | ((value << Self::PUMP_BITMAP_HIGH3_OFFSET) & Self::PUMP_BITMAP_HIGH3_MASK)
        });
    }
}

/// PMUP_BITMAP_HIGH3 value type for batch read/modify/write operations
///
/// Use `pmupbitmaphigh3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmupbitmaphigh3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmupBitmapHigh3Val {
    bits: u32,
}

impl PmupBitmapHigh3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PUMP_BITMAP_HIGH3** `[31:0]`
    ///
    /// select valid high channel3
    #[inline(always)]
    pub const fn pump_bitmap_high3(&self) -> u32 {
        (self.bits & PmupBitmapHigh3::PUMP_BITMAP_HIGH3_MASK)
            >> PmupBitmapHigh3::PUMP_BITMAP_HIGH3_OFFSET
    }

    /// Set **PUMP_BITMAP_HIGH3** `[31:0]`
    ///
    /// select valid high channel3
    #[inline(always)]
    pub const fn with_pump_bitmap_high3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupBitmapHigh3::PUMP_BITMAP_HIGH3_MASK)
                | ((value << PmupBitmapHigh3::PUMP_BITMAP_HIGH3_OFFSET)
                    & PmupBitmapHigh3::PUMP_BITMAP_HIGH3_MASK),
        }
    }
}

impl Default for PmupBitmapHigh3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmupBitmapHigh3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmupBitmapHigh3Val {
        PmupBitmapHigh3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmupBitmapHigh3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmupBitmapHigh3Val) -> PmupBitmapHigh3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMUP_BITMAP_HIGH4 Register
///
/// select valid pvt channel
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **PUMP_BITMAP_HIGH4** `[31:0]` (RW)
///
///   select valid high channel4
#[repr(transparent)]
pub struct PmupBitmapHigh4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmupBitmapHigh4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmupBitmapHigh4 {
    // ======== Field Constants ========

    /// **PUMP_BITMAP_HIGH4**
    ///
    /// select valid high channel4
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PUMP_BITMAP_HIGH4_OFFSET: u32 = 0;
    pub const PUMP_BITMAP_HIGH4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PUMP_BITMAP_HIGH4** `[31:0]` (RW)
    ///
    /// select valid high channel4
    #[inline(always)]
    pub fn pump_bitmap_high4(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_BITMAP_HIGH4_MASK) >> Self::PUMP_BITMAP_HIGH4_OFFSET
    }

    /// Write **PUMP_BITMAP_HIGH4** `[31:0]` (RW)
    ///
    /// select valid high channel4
    #[inline(always)]
    pub fn write_pump_bitmap_high4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_BITMAP_HIGH4_MASK)
                | ((value << Self::PUMP_BITMAP_HIGH4_OFFSET) & Self::PUMP_BITMAP_HIGH4_MASK)
        });
    }
}

/// PMUP_BITMAP_HIGH4 value type for batch read/modify/write operations
///
/// Use `pmupbitmaphigh4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmupbitmaphigh4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmupBitmapHigh4Val {
    bits: u32,
}

impl PmupBitmapHigh4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PUMP_BITMAP_HIGH4** `[31:0]`
    ///
    /// select valid high channel4
    #[inline(always)]
    pub const fn pump_bitmap_high4(&self) -> u32 {
        (self.bits & PmupBitmapHigh4::PUMP_BITMAP_HIGH4_MASK)
            >> PmupBitmapHigh4::PUMP_BITMAP_HIGH4_OFFSET
    }

    /// Set **PUMP_BITMAP_HIGH4** `[31:0]`
    ///
    /// select valid high channel4
    #[inline(always)]
    pub const fn with_pump_bitmap_high4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupBitmapHigh4::PUMP_BITMAP_HIGH4_MASK)
                | ((value << PmupBitmapHigh4::PUMP_BITMAP_HIGH4_OFFSET)
                    & PmupBitmapHigh4::PUMP_BITMAP_HIGH4_MASK),
        }
    }
}

impl Default for PmupBitmapHigh4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmupBitmapHigh4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmupBitmapHigh4Val {
        PmupBitmapHigh4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmupBitmapHigh4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmupBitmapHigh4Val) -> PmupBitmapHigh4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMUP_BITMAP_LOW0 Register
///
/// select valid pvt channel
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **PUMP_BITMAP_LOW0** `[31:0]` (RW)
///
///   select valid low channel0
#[repr(transparent)]
pub struct PmupBitmapLow0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmupBitmapLow0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmupBitmapLow0 {
    // ======== Field Constants ========

    /// **PUMP_BITMAP_LOW0**
    ///
    /// select valid low channel0
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PUMP_BITMAP_LOW0_OFFSET: u32 = 0;
    pub const PUMP_BITMAP_LOW0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PUMP_BITMAP_LOW0** `[31:0]` (RW)
    ///
    /// select valid low channel0
    #[inline(always)]
    pub fn pump_bitmap_low0(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_BITMAP_LOW0_MASK) >> Self::PUMP_BITMAP_LOW0_OFFSET
    }

    /// Write **PUMP_BITMAP_LOW0** `[31:0]` (RW)
    ///
    /// select valid low channel0
    #[inline(always)]
    pub fn write_pump_bitmap_low0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_BITMAP_LOW0_MASK)
                | ((value << Self::PUMP_BITMAP_LOW0_OFFSET) & Self::PUMP_BITMAP_LOW0_MASK)
        });
    }
}

/// PMUP_BITMAP_LOW0 value type for batch read/modify/write operations
///
/// Use `pmupbitmaplow0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmupbitmaplow0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmupBitmapLow0Val {
    bits: u32,
}

impl PmupBitmapLow0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PUMP_BITMAP_LOW0** `[31:0]`
    ///
    /// select valid low channel0
    #[inline(always)]
    pub const fn pump_bitmap_low0(&self) -> u32 {
        (self.bits & PmupBitmapLow0::PUMP_BITMAP_LOW0_MASK)
            >> PmupBitmapLow0::PUMP_BITMAP_LOW0_OFFSET
    }

    /// Set **PUMP_BITMAP_LOW0** `[31:0]`
    ///
    /// select valid low channel0
    #[inline(always)]
    pub const fn with_pump_bitmap_low0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupBitmapLow0::PUMP_BITMAP_LOW0_MASK)
                | ((value << PmupBitmapLow0::PUMP_BITMAP_LOW0_OFFSET)
                    & PmupBitmapLow0::PUMP_BITMAP_LOW0_MASK),
        }
    }
}

impl Default for PmupBitmapLow0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmupBitmapLow0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmupBitmapLow0Val {
        PmupBitmapLow0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmupBitmapLow0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmupBitmapLow0Val) -> PmupBitmapLow0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMUP_BITMAP_LOW1 Register
///
/// select valid pvt channel
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **PUMP_BITMAP_LOW1** `[31:0]` (RW)
///
///   select valid low channel1
#[repr(transparent)]
pub struct PmupBitmapLow1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmupBitmapLow1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmupBitmapLow1 {
    // ======== Field Constants ========

    /// **PUMP_BITMAP_LOW1**
    ///
    /// select valid low channel1
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PUMP_BITMAP_LOW1_OFFSET: u32 = 0;
    pub const PUMP_BITMAP_LOW1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PUMP_BITMAP_LOW1** `[31:0]` (RW)
    ///
    /// select valid low channel1
    #[inline(always)]
    pub fn pump_bitmap_low1(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_BITMAP_LOW1_MASK) >> Self::PUMP_BITMAP_LOW1_OFFSET
    }

    /// Write **PUMP_BITMAP_LOW1** `[31:0]` (RW)
    ///
    /// select valid low channel1
    #[inline(always)]
    pub fn write_pump_bitmap_low1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_BITMAP_LOW1_MASK)
                | ((value << Self::PUMP_BITMAP_LOW1_OFFSET) & Self::PUMP_BITMAP_LOW1_MASK)
        });
    }
}

/// PMUP_BITMAP_LOW1 value type for batch read/modify/write operations
///
/// Use `pmupbitmaplow1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmupbitmaplow1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmupBitmapLow1Val {
    bits: u32,
}

impl PmupBitmapLow1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PUMP_BITMAP_LOW1** `[31:0]`
    ///
    /// select valid low channel1
    #[inline(always)]
    pub const fn pump_bitmap_low1(&self) -> u32 {
        (self.bits & PmupBitmapLow1::PUMP_BITMAP_LOW1_MASK)
            >> PmupBitmapLow1::PUMP_BITMAP_LOW1_OFFSET
    }

    /// Set **PUMP_BITMAP_LOW1** `[31:0]`
    ///
    /// select valid low channel1
    #[inline(always)]
    pub const fn with_pump_bitmap_low1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupBitmapLow1::PUMP_BITMAP_LOW1_MASK)
                | ((value << PmupBitmapLow1::PUMP_BITMAP_LOW1_OFFSET)
                    & PmupBitmapLow1::PUMP_BITMAP_LOW1_MASK),
        }
    }
}

impl Default for PmupBitmapLow1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmupBitmapLow1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmupBitmapLow1Val {
        PmupBitmapLow1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmupBitmapLow1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmupBitmapLow1Val) -> PmupBitmapLow1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMUP_BITMAP_LOW2 Register
///
/// select valid pvt channel
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **PUMP_BITMAP_LOW2** `[31:0]` (RW)
///
///   select valid low channel2
#[repr(transparent)]
pub struct PmupBitmapLow2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmupBitmapLow2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmupBitmapLow2 {
    // ======== Field Constants ========

    /// **PUMP_BITMAP_LOW2**
    ///
    /// select valid low channel2
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PUMP_BITMAP_LOW2_OFFSET: u32 = 0;
    pub const PUMP_BITMAP_LOW2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PUMP_BITMAP_LOW2** `[31:0]` (RW)
    ///
    /// select valid low channel2
    #[inline(always)]
    pub fn pump_bitmap_low2(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_BITMAP_LOW2_MASK) >> Self::PUMP_BITMAP_LOW2_OFFSET
    }

    /// Write **PUMP_BITMAP_LOW2** `[31:0]` (RW)
    ///
    /// select valid low channel2
    #[inline(always)]
    pub fn write_pump_bitmap_low2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_BITMAP_LOW2_MASK)
                | ((value << Self::PUMP_BITMAP_LOW2_OFFSET) & Self::PUMP_BITMAP_LOW2_MASK)
        });
    }
}

/// PMUP_BITMAP_LOW2 value type for batch read/modify/write operations
///
/// Use `pmupbitmaplow2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmupbitmaplow2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmupBitmapLow2Val {
    bits: u32,
}

impl PmupBitmapLow2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PUMP_BITMAP_LOW2** `[31:0]`
    ///
    /// select valid low channel2
    #[inline(always)]
    pub const fn pump_bitmap_low2(&self) -> u32 {
        (self.bits & PmupBitmapLow2::PUMP_BITMAP_LOW2_MASK)
            >> PmupBitmapLow2::PUMP_BITMAP_LOW2_OFFSET
    }

    /// Set **PUMP_BITMAP_LOW2** `[31:0]`
    ///
    /// select valid low channel2
    #[inline(always)]
    pub const fn with_pump_bitmap_low2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupBitmapLow2::PUMP_BITMAP_LOW2_MASK)
                | ((value << PmupBitmapLow2::PUMP_BITMAP_LOW2_OFFSET)
                    & PmupBitmapLow2::PUMP_BITMAP_LOW2_MASK),
        }
    }
}

impl Default for PmupBitmapLow2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmupBitmapLow2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmupBitmapLow2Val {
        PmupBitmapLow2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmupBitmapLow2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmupBitmapLow2Val) -> PmupBitmapLow2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMUP_BITMAP_LOW3 Register
///
/// select valid pvt channel
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **PUMP_BITMAP_LOW3** `[31:0]` (RW)
///
///   select valid low channel3
#[repr(transparent)]
pub struct PmupBitmapLow3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmupBitmapLow3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmupBitmapLow3 {
    // ======== Field Constants ========

    /// **PUMP_BITMAP_LOW3**
    ///
    /// select valid low channel3
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PUMP_BITMAP_LOW3_OFFSET: u32 = 0;
    pub const PUMP_BITMAP_LOW3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PUMP_BITMAP_LOW3** `[31:0]` (RW)
    ///
    /// select valid low channel3
    #[inline(always)]
    pub fn pump_bitmap_low3(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_BITMAP_LOW3_MASK) >> Self::PUMP_BITMAP_LOW3_OFFSET
    }

    /// Write **PUMP_BITMAP_LOW3** `[31:0]` (RW)
    ///
    /// select valid low channel3
    #[inline(always)]
    pub fn write_pump_bitmap_low3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_BITMAP_LOW3_MASK)
                | ((value << Self::PUMP_BITMAP_LOW3_OFFSET) & Self::PUMP_BITMAP_LOW3_MASK)
        });
    }
}

/// PMUP_BITMAP_LOW3 value type for batch read/modify/write operations
///
/// Use `pmupbitmaplow3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmupbitmaplow3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmupBitmapLow3Val {
    bits: u32,
}

impl PmupBitmapLow3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PUMP_BITMAP_LOW3** `[31:0]`
    ///
    /// select valid low channel3
    #[inline(always)]
    pub const fn pump_bitmap_low3(&self) -> u32 {
        (self.bits & PmupBitmapLow3::PUMP_BITMAP_LOW3_MASK)
            >> PmupBitmapLow3::PUMP_BITMAP_LOW3_OFFSET
    }

    /// Set **PUMP_BITMAP_LOW3** `[31:0]`
    ///
    /// select valid low channel3
    #[inline(always)]
    pub const fn with_pump_bitmap_low3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupBitmapLow3::PUMP_BITMAP_LOW3_MASK)
                | ((value << PmupBitmapLow3::PUMP_BITMAP_LOW3_OFFSET)
                    & PmupBitmapLow3::PUMP_BITMAP_LOW3_MASK),
        }
    }
}

impl Default for PmupBitmapLow3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmupBitmapLow3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmupBitmapLow3Val {
        PmupBitmapLow3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmupBitmapLow3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmupBitmapLow3Val) -> PmupBitmapLow3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMUP_BITMAP_LOW4 Register
///
/// select valid pvt channel
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **PUMP_BITMAP_LOW4** `[31:0]` (RW)
///
///   select valid low channel4
#[repr(transparent)]
pub struct PmupBitmapLow4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmupBitmapLow4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmupBitmapLow4 {
    // ======== Field Constants ========

    /// **PUMP_BITMAP_LOW4**
    ///
    /// select valid low channel4
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PUMP_BITMAP_LOW4_OFFSET: u32 = 0;
    pub const PUMP_BITMAP_LOW4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PUMP_BITMAP_LOW4** `[31:0]` (RW)
    ///
    /// select valid low channel4
    #[inline(always)]
    pub fn pump_bitmap_low4(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_BITMAP_LOW4_MASK) >> Self::PUMP_BITMAP_LOW4_OFFSET
    }

    /// Write **PUMP_BITMAP_LOW4** `[31:0]` (RW)
    ///
    /// select valid low channel4
    #[inline(always)]
    pub fn write_pump_bitmap_low4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_BITMAP_LOW4_MASK)
                | ((value << Self::PUMP_BITMAP_LOW4_OFFSET) & Self::PUMP_BITMAP_LOW4_MASK)
        });
    }
}

/// PMUP_BITMAP_LOW4 value type for batch read/modify/write operations
///
/// Use `pmupbitmaplow4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmupbitmaplow4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmupBitmapLow4Val {
    bits: u32,
}

impl PmupBitmapLow4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PUMP_BITMAP_LOW4** `[31:0]`
    ///
    /// select valid low channel4
    #[inline(always)]
    pub const fn pump_bitmap_low4(&self) -> u32 {
        (self.bits & PmupBitmapLow4::PUMP_BITMAP_LOW4_MASK)
            >> PmupBitmapLow4::PUMP_BITMAP_LOW4_OFFSET
    }

    /// Set **PUMP_BITMAP_LOW4** `[31:0]`
    ///
    /// select valid low channel4
    #[inline(always)]
    pub const fn with_pump_bitmap_low4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupBitmapLow4::PUMP_BITMAP_LOW4_MASK)
                | ((value << PmupBitmapLow4::PUMP_BITMAP_LOW4_OFFSET)
                    & PmupBitmapLow4::PUMP_BITMAP_LOW4_MASK),
        }
    }
}

impl Default for PmupBitmapLow4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmupBitmapLow4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmupBitmapLow4Val {
        PmupBitmapLow4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmupBitmapLow4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmupBitmapLow4Val) -> PmupBitmapLow4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMUP_DRV_CFG Register
///
/// configure pump drv
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **PUMP_DRV0** `[30:27]` (RW)
///
///   configure cmd0 drv
///
/// - **PUMP_DRV1** `[26:23]` (RW)
///
///   configure cmd1 drv
///
/// - **PUMP_DRV2** `[22:19]` (RW)
///
///   configure cmd2 drv
///
/// - **PUMP_DRV3** `[18:15]` (RW)
///
///   configure cmd3 drv
///
/// - **PUMP_DRV4** `[14:11]` (RW)
///
///   configure cmd4 drv
///
/// - **CLK_EN** `[10]` (RW)
///
///   force register clken
///
/// - **PUMP_EN** `[9]` (RW)
///
///   configure pvt charge xpd
#[repr(transparent)]
pub struct PmupDrvCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmupDrvCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmupDrvCfg {
    // ======== Field Constants ========

    /// **PUMP_DRV0**
    ///
    /// configure cmd0 drv
    ///
    /// Bits `[30:27]`, mask `0x78000000`
    pub const PUMP_DRV0_OFFSET: u32 = 27;
    pub const PUMP_DRV0_MASK: u32 = 0x78000000;

    /// **PUMP_DRV1**
    ///
    /// configure cmd1 drv
    ///
    /// Bits `[26:23]`, mask `0x07800000`
    pub const PUMP_DRV1_OFFSET: u32 = 23;
    pub const PUMP_DRV1_MASK: u32 = 0x07800000;

    /// **PUMP_DRV2**
    ///
    /// configure cmd2 drv
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PUMP_DRV2_OFFSET: u32 = 19;
    pub const PUMP_DRV2_MASK: u32 = 0x00780000;

    /// **PUMP_DRV3**
    ///
    /// configure cmd3 drv
    ///
    /// Bits `[18:15]`, mask `0x00078000`
    pub const PUMP_DRV3_OFFSET: u32 = 15;
    pub const PUMP_DRV3_MASK: u32 = 0x00078000;

    /// **PUMP_DRV4**
    ///
    /// configure cmd4 drv
    ///
    /// Bits `[14:11]`, mask `0x00007800`
    pub const PUMP_DRV4_OFFSET: u32 = 11;
    pub const PUMP_DRV4_MASK: u32 = 0x00007800;

    /// **CLK_EN**
    ///
    /// force register clken
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const CLK_EN_OFFSET: u32 = 10;
    pub const CLK_EN_MASK: u32 = 0x00000400;
    pub const CLK_EN: u32 = 1 << 10;

    /// **PUMP_EN**
    ///
    /// configure pvt charge xpd
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const PUMP_EN_OFFSET: u32 = 9;
    pub const PUMP_EN_MASK: u32 = 0x00000200;
    pub const PUMP_EN: u32 = 1 << 9;

    // ======== Field Accessors ========

    /// **PUMP_DRV0** `[30:27]` (RW)
    ///
    /// configure cmd0 drv
    #[inline(always)]
    pub fn pump_drv0(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_DRV0_MASK) >> Self::PUMP_DRV0_OFFSET
    }

    /// Write **PUMP_DRV0** `[30:27]` (RW)
    ///
    /// configure cmd0 drv
    #[inline(always)]
    pub fn write_pump_drv0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_DRV0_MASK) | ((value << Self::PUMP_DRV0_OFFSET) & Self::PUMP_DRV0_MASK)
        });
    }

    /// **PUMP_DRV1** `[26:23]` (RW)
    ///
    /// configure cmd1 drv
    #[inline(always)]
    pub fn pump_drv1(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_DRV1_MASK) >> Self::PUMP_DRV1_OFFSET
    }

    /// Write **PUMP_DRV1** `[26:23]` (RW)
    ///
    /// configure cmd1 drv
    #[inline(always)]
    pub fn write_pump_drv1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_DRV1_MASK) | ((value << Self::PUMP_DRV1_OFFSET) & Self::PUMP_DRV1_MASK)
        });
    }

    /// **PUMP_DRV2** `[22:19]` (RW)
    ///
    /// configure cmd2 drv
    #[inline(always)]
    pub fn pump_drv2(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_DRV2_MASK) >> Self::PUMP_DRV2_OFFSET
    }

    /// Write **PUMP_DRV2** `[22:19]` (RW)
    ///
    /// configure cmd2 drv
    #[inline(always)]
    pub fn write_pump_drv2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_DRV2_MASK) | ((value << Self::PUMP_DRV2_OFFSET) & Self::PUMP_DRV2_MASK)
        });
    }

    /// **PUMP_DRV3** `[18:15]` (RW)
    ///
    /// configure cmd3 drv
    #[inline(always)]
    pub fn pump_drv3(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_DRV3_MASK) >> Self::PUMP_DRV3_OFFSET
    }

    /// Write **PUMP_DRV3** `[18:15]` (RW)
    ///
    /// configure cmd3 drv
    #[inline(always)]
    pub fn write_pump_drv3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_DRV3_MASK) | ((value << Self::PUMP_DRV3_OFFSET) & Self::PUMP_DRV3_MASK)
        });
    }

    /// **PUMP_DRV4** `[14:11]` (RW)
    ///
    /// configure cmd4 drv
    #[inline(always)]
    pub fn pump_drv4(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_DRV4_MASK) >> Self::PUMP_DRV4_OFFSET
    }

    /// Write **PUMP_DRV4** `[14:11]` (RW)
    ///
    /// configure cmd4 drv
    #[inline(always)]
    pub fn write_pump_drv4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_DRV4_MASK) | ((value << Self::PUMP_DRV4_OFFSET) & Self::PUMP_DRV4_MASK)
        });
    }

    /// **CLK_EN** `[10]` (RW)
    ///
    /// force register clken
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

    /// **PUMP_EN** `[9]` (RW)
    ///
    /// configure pvt charge xpd
    #[inline(always)]
    pub fn pump_en(&self) -> bool {
        (self.value.read_bits() & Self::PUMP_EN_MASK) != 0
    }

    /// Set PUMP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pump_en(&self) {
        self.value.modify_bits(|r| r | Self::PUMP_EN_MASK);
    }

    /// Clear PUMP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pump_en(&self) {
        self.value.modify_bits(|r| r & !Self::PUMP_EN_MASK);
    }
}

/// PMUP_DRV_CFG value type for batch read/modify/write operations
///
/// Use `pmupdrvcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmupdrvcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmupDrvCfgVal {
    bits: u32,
}

impl PmupDrvCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PUMP_DRV0** `[30:27]`
    ///
    /// configure cmd0 drv
    #[inline(always)]
    pub const fn pump_drv0(&self) -> u32 {
        (self.bits & PmupDrvCfg::PUMP_DRV0_MASK) >> PmupDrvCfg::PUMP_DRV0_OFFSET
    }

    /// Read **PUMP_DRV1** `[26:23]`
    ///
    /// configure cmd1 drv
    #[inline(always)]
    pub const fn pump_drv1(&self) -> u32 {
        (self.bits & PmupDrvCfg::PUMP_DRV1_MASK) >> PmupDrvCfg::PUMP_DRV1_OFFSET
    }

    /// Read **PUMP_DRV2** `[22:19]`
    ///
    /// configure cmd2 drv
    #[inline(always)]
    pub const fn pump_drv2(&self) -> u32 {
        (self.bits & PmupDrvCfg::PUMP_DRV2_MASK) >> PmupDrvCfg::PUMP_DRV2_OFFSET
    }

    /// Read **PUMP_DRV3** `[18:15]`
    ///
    /// configure cmd3 drv
    #[inline(always)]
    pub const fn pump_drv3(&self) -> u32 {
        (self.bits & PmupDrvCfg::PUMP_DRV3_MASK) >> PmupDrvCfg::PUMP_DRV3_OFFSET
    }

    /// Read **PUMP_DRV4** `[14:11]`
    ///
    /// configure cmd4 drv
    #[inline(always)]
    pub const fn pump_drv4(&self) -> u32 {
        (self.bits & PmupDrvCfg::PUMP_DRV4_MASK) >> PmupDrvCfg::PUMP_DRV4_OFFSET
    }

    /// Read **CLK_EN** `[10:10]`
    ///
    /// force register clken
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & PmupDrvCfg::CLK_EN_MASK) != 0
    }

    /// Read **PUMP_EN** `[9:9]`
    ///
    /// configure pvt charge xpd
    #[inline(always)]
    pub const fn pump_en(&self) -> bool {
        (self.bits & PmupDrvCfg::PUMP_EN_MASK) != 0
    }

    /// Set **PUMP_DRV0** `[30:27]`
    ///
    /// configure cmd0 drv
    #[inline(always)]
    pub const fn with_pump_drv0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupDrvCfg::PUMP_DRV0_MASK)
                | ((value << PmupDrvCfg::PUMP_DRV0_OFFSET) & PmupDrvCfg::PUMP_DRV0_MASK),
        }
    }

    /// Set **PUMP_DRV1** `[26:23]`
    ///
    /// configure cmd1 drv
    #[inline(always)]
    pub const fn with_pump_drv1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupDrvCfg::PUMP_DRV1_MASK)
                | ((value << PmupDrvCfg::PUMP_DRV1_OFFSET) & PmupDrvCfg::PUMP_DRV1_MASK),
        }
    }

    /// Set **PUMP_DRV2** `[22:19]`
    ///
    /// configure cmd2 drv
    #[inline(always)]
    pub const fn with_pump_drv2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupDrvCfg::PUMP_DRV2_MASK)
                | ((value << PmupDrvCfg::PUMP_DRV2_OFFSET) & PmupDrvCfg::PUMP_DRV2_MASK),
        }
    }

    /// Set **PUMP_DRV3** `[18:15]`
    ///
    /// configure cmd3 drv
    #[inline(always)]
    pub const fn with_pump_drv3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupDrvCfg::PUMP_DRV3_MASK)
                | ((value << PmupDrvCfg::PUMP_DRV3_OFFSET) & PmupDrvCfg::PUMP_DRV3_MASK),
        }
    }

    /// Set **PUMP_DRV4** `[14:11]`
    ///
    /// configure cmd4 drv
    #[inline(always)]
    pub const fn with_pump_drv4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupDrvCfg::PUMP_DRV4_MASK)
                | ((value << PmupDrvCfg::PUMP_DRV4_OFFSET) & PmupDrvCfg::PUMP_DRV4_MASK),
        }
    }

    /// Set **CLK_EN** `[10:10]`
    ///
    /// force register clken
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PmupDrvCfg::CLK_EN_MASK
            } else {
                self.bits & !PmupDrvCfg::CLK_EN_MASK
            },
        }
    }

    /// Set **PUMP_EN** `[9:9]`
    ///
    /// configure pvt charge xpd
    #[inline(always)]
    pub const fn with_pump_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PmupDrvCfg::PUMP_EN_MASK
            } else {
                self.bits & !PmupDrvCfg::PUMP_EN_MASK
            },
        }
    }
}

impl Default for PmupDrvCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmupDrvCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmupDrvCfgVal {
        PmupDrvCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmupDrvCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmupDrvCfgVal) -> PmupDrvCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMUP_CHANNEL_CFG Register
///
/// configure the code of valid pump channel code
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **PUMP_CHANNEL_CODE0** `[31:27]` (RW)
///
///   configure cmd0 code
///
/// - **PUMP_CHANNEL_CODE1** `[26:22]` (RW)
///
///   configure cmd1 code
///
/// - **PUMP_CHANNEL_CODE2** `[21:17]` (RW)
///
///   configure cmd2 code
///
/// - **PUMP_CHANNEL_CODE3** `[16:12]` (RW)
///
///   configure cmd3 code
///
/// - **PUMP_CHANNEL_CODE4** `[11:7]` (RW)
///
///   configure cmd4 code
#[repr(transparent)]
pub struct PmupChannelCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmupChannelCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmupChannelCfg {
    // ======== Field Constants ========

    /// **PUMP_CHANNEL_CODE0**
    ///
    /// configure cmd0 code
    ///
    /// Bits `[31:27]`, mask `0xf8000000`
    pub const PUMP_CHANNEL_CODE0_OFFSET: u32 = 27;
    pub const PUMP_CHANNEL_CODE0_MASK: u32 = 0xf8000000;

    /// **PUMP_CHANNEL_CODE1**
    ///
    /// configure cmd1 code
    ///
    /// Bits `[26:22]`, mask `0x07c00000`
    pub const PUMP_CHANNEL_CODE1_OFFSET: u32 = 22;
    pub const PUMP_CHANNEL_CODE1_MASK: u32 = 0x07c00000;

    /// **PUMP_CHANNEL_CODE2**
    ///
    /// configure cmd2 code
    ///
    /// Bits `[21:17]`, mask `0x003e0000`
    pub const PUMP_CHANNEL_CODE2_OFFSET: u32 = 17;
    pub const PUMP_CHANNEL_CODE2_MASK: u32 = 0x003e0000;

    /// **PUMP_CHANNEL_CODE3**
    ///
    /// configure cmd3 code
    ///
    /// Bits `[16:12]`, mask `0x0001f000`
    pub const PUMP_CHANNEL_CODE3_OFFSET: u32 = 12;
    pub const PUMP_CHANNEL_CODE3_MASK: u32 = 0x0001f000;

    /// **PUMP_CHANNEL_CODE4**
    ///
    /// configure cmd4 code
    ///
    /// Bits `[11:7]`, mask `0x00000f80`
    pub const PUMP_CHANNEL_CODE4_OFFSET: u32 = 7;
    pub const PUMP_CHANNEL_CODE4_MASK: u32 = 0x00000f80;

    // ======== Field Accessors ========

    /// **PUMP_CHANNEL_CODE0** `[31:27]` (RW)
    ///
    /// configure cmd0 code
    #[inline(always)]
    pub fn pump_channel_code0(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_CHANNEL_CODE0_MASK) >> Self::PUMP_CHANNEL_CODE0_OFFSET
    }

    /// Write **PUMP_CHANNEL_CODE0** `[31:27]` (RW)
    ///
    /// configure cmd0 code
    #[inline(always)]
    pub fn write_pump_channel_code0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_CHANNEL_CODE0_MASK)
                | ((value << Self::PUMP_CHANNEL_CODE0_OFFSET) & Self::PUMP_CHANNEL_CODE0_MASK)
        });
    }

    /// **PUMP_CHANNEL_CODE1** `[26:22]` (RW)
    ///
    /// configure cmd1 code
    #[inline(always)]
    pub fn pump_channel_code1(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_CHANNEL_CODE1_MASK) >> Self::PUMP_CHANNEL_CODE1_OFFSET
    }

    /// Write **PUMP_CHANNEL_CODE1** `[26:22]` (RW)
    ///
    /// configure cmd1 code
    #[inline(always)]
    pub fn write_pump_channel_code1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_CHANNEL_CODE1_MASK)
                | ((value << Self::PUMP_CHANNEL_CODE1_OFFSET) & Self::PUMP_CHANNEL_CODE1_MASK)
        });
    }

    /// **PUMP_CHANNEL_CODE2** `[21:17]` (RW)
    ///
    /// configure cmd2 code
    #[inline(always)]
    pub fn pump_channel_code2(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_CHANNEL_CODE2_MASK) >> Self::PUMP_CHANNEL_CODE2_OFFSET
    }

    /// Write **PUMP_CHANNEL_CODE2** `[21:17]` (RW)
    ///
    /// configure cmd2 code
    #[inline(always)]
    pub fn write_pump_channel_code2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_CHANNEL_CODE2_MASK)
                | ((value << Self::PUMP_CHANNEL_CODE2_OFFSET) & Self::PUMP_CHANNEL_CODE2_MASK)
        });
    }

    /// **PUMP_CHANNEL_CODE3** `[16:12]` (RW)
    ///
    /// configure cmd3 code
    #[inline(always)]
    pub fn pump_channel_code3(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_CHANNEL_CODE3_MASK) >> Self::PUMP_CHANNEL_CODE3_OFFSET
    }

    /// Write **PUMP_CHANNEL_CODE3** `[16:12]` (RW)
    ///
    /// configure cmd3 code
    #[inline(always)]
    pub fn write_pump_channel_code3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_CHANNEL_CODE3_MASK)
                | ((value << Self::PUMP_CHANNEL_CODE3_OFFSET) & Self::PUMP_CHANNEL_CODE3_MASK)
        });
    }

    /// **PUMP_CHANNEL_CODE4** `[11:7]` (RW)
    ///
    /// configure cmd4 code
    #[inline(always)]
    pub fn pump_channel_code4(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_CHANNEL_CODE4_MASK) >> Self::PUMP_CHANNEL_CODE4_OFFSET
    }

    /// Write **PUMP_CHANNEL_CODE4** `[11:7]` (RW)
    ///
    /// configure cmd4 code
    #[inline(always)]
    pub fn write_pump_channel_code4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_CHANNEL_CODE4_MASK)
                | ((value << Self::PUMP_CHANNEL_CODE4_OFFSET) & Self::PUMP_CHANNEL_CODE4_MASK)
        });
    }
}

/// PMUP_CHANNEL_CFG value type for batch read/modify/write operations
///
/// Use `pmupchannelcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmupchannelcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmupChannelCfgVal {
    bits: u32,
}

impl PmupChannelCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PUMP_CHANNEL_CODE0** `[31:27]`
    ///
    /// configure cmd0 code
    #[inline(always)]
    pub const fn pump_channel_code0(&self) -> u32 {
        (self.bits & PmupChannelCfg::PUMP_CHANNEL_CODE0_MASK)
            >> PmupChannelCfg::PUMP_CHANNEL_CODE0_OFFSET
    }

    /// Read **PUMP_CHANNEL_CODE1** `[26:22]`
    ///
    /// configure cmd1 code
    #[inline(always)]
    pub const fn pump_channel_code1(&self) -> u32 {
        (self.bits & PmupChannelCfg::PUMP_CHANNEL_CODE1_MASK)
            >> PmupChannelCfg::PUMP_CHANNEL_CODE1_OFFSET
    }

    /// Read **PUMP_CHANNEL_CODE2** `[21:17]`
    ///
    /// configure cmd2 code
    #[inline(always)]
    pub const fn pump_channel_code2(&self) -> u32 {
        (self.bits & PmupChannelCfg::PUMP_CHANNEL_CODE2_MASK)
            >> PmupChannelCfg::PUMP_CHANNEL_CODE2_OFFSET
    }

    /// Read **PUMP_CHANNEL_CODE3** `[16:12]`
    ///
    /// configure cmd3 code
    #[inline(always)]
    pub const fn pump_channel_code3(&self) -> u32 {
        (self.bits & PmupChannelCfg::PUMP_CHANNEL_CODE3_MASK)
            >> PmupChannelCfg::PUMP_CHANNEL_CODE3_OFFSET
    }

    /// Read **PUMP_CHANNEL_CODE4** `[11:7]`
    ///
    /// configure cmd4 code
    #[inline(always)]
    pub const fn pump_channel_code4(&self) -> u32 {
        (self.bits & PmupChannelCfg::PUMP_CHANNEL_CODE4_MASK)
            >> PmupChannelCfg::PUMP_CHANNEL_CODE4_OFFSET
    }

    /// Set **PUMP_CHANNEL_CODE0** `[31:27]`
    ///
    /// configure cmd0 code
    #[inline(always)]
    pub const fn with_pump_channel_code0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupChannelCfg::PUMP_CHANNEL_CODE0_MASK)
                | ((value << PmupChannelCfg::PUMP_CHANNEL_CODE0_OFFSET)
                    & PmupChannelCfg::PUMP_CHANNEL_CODE0_MASK),
        }
    }

    /// Set **PUMP_CHANNEL_CODE1** `[26:22]`
    ///
    /// configure cmd1 code
    #[inline(always)]
    pub const fn with_pump_channel_code1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupChannelCfg::PUMP_CHANNEL_CODE1_MASK)
                | ((value << PmupChannelCfg::PUMP_CHANNEL_CODE1_OFFSET)
                    & PmupChannelCfg::PUMP_CHANNEL_CODE1_MASK),
        }
    }

    /// Set **PUMP_CHANNEL_CODE2** `[21:17]`
    ///
    /// configure cmd2 code
    #[inline(always)]
    pub const fn with_pump_channel_code2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupChannelCfg::PUMP_CHANNEL_CODE2_MASK)
                | ((value << PmupChannelCfg::PUMP_CHANNEL_CODE2_OFFSET)
                    & PmupChannelCfg::PUMP_CHANNEL_CODE2_MASK),
        }
    }

    /// Set **PUMP_CHANNEL_CODE3** `[16:12]`
    ///
    /// configure cmd3 code
    #[inline(always)]
    pub const fn with_pump_channel_code3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupChannelCfg::PUMP_CHANNEL_CODE3_MASK)
                | ((value << PmupChannelCfg::PUMP_CHANNEL_CODE3_OFFSET)
                    & PmupChannelCfg::PUMP_CHANNEL_CODE3_MASK),
        }
    }

    /// Set **PUMP_CHANNEL_CODE4** `[11:7]`
    ///
    /// configure cmd4 code
    #[inline(always)]
    pub const fn with_pump_channel_code4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmupChannelCfg::PUMP_CHANNEL_CODE4_MASK)
                | ((value << PmupChannelCfg::PUMP_CHANNEL_CODE4_OFFSET)
                    & PmupChannelCfg::PUMP_CHANNEL_CODE4_MASK),
        }
    }
}

impl Default for PmupChannelCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PmupChannelCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmupChannelCfgVal {
        PmupChannelCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmupChannelCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmupChannelCfgVal) -> PmupChannelCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_CFG Register
///
/// configure pvt clk
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **CLK_SEL** `[31]` (RW)
///
///   select pvt clk
///
/// - **MONITOR_CLK_PVT_EN** `[8]` (RW)
///
///   needs field desc
///
/// - **PUMP_CLK_DIV_NUM** `[7:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct ClkCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClkCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkCfg {
    // ======== Field Constants ========

    /// **CLK_SEL**
    ///
    /// select pvt clk
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_SEL_OFFSET: u32 = 31;
    pub const CLK_SEL_MASK: u32 = 0x80000000;
    pub const CLK_SEL: u32 = 1 << 31;

    /// **MONITOR_CLK_PVT_EN**
    ///
    /// needs field desc
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const MONITOR_CLK_PVT_EN_OFFSET: u32 = 8;
    pub const MONITOR_CLK_PVT_EN_MASK: u32 = 0x00000100;
    pub const MONITOR_CLK_PVT_EN: u32 = 1 << 8;

    /// **PUMP_CLK_DIV_NUM**
    ///
    /// needs field desc
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const PUMP_CLK_DIV_NUM_OFFSET: u32 = 0;
    pub const PUMP_CLK_DIV_NUM_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **CLK_SEL** `[31]` (RW)
    ///
    /// select pvt clk
    #[inline(always)]
    pub fn clk_sel(&self) -> bool {
        (self.value.read_bits() & Self::CLK_SEL_MASK) != 0
    }

    /// Set CLK_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_clk_sel(&self) {
        self.value.modify_bits(|r| r | Self::CLK_SEL_MASK);
    }

    /// Clear CLK_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_clk_sel(&self) {
        self.value.modify_bits(|r| r & !Self::CLK_SEL_MASK);
    }

    /// **MONITOR_CLK_PVT_EN** `[8]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_clk_pvt_en(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_CLK_PVT_EN_MASK) != 0
    }

    /// Set MONITOR_CLK_PVT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_clk_pvt_en(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_CLK_PVT_EN_MASK);
    }

    /// Clear MONITOR_CLK_PVT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_clk_pvt_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_CLK_PVT_EN_MASK);
    }

    /// **PUMP_CLK_DIV_NUM** `[7:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn pump_clk_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::PUMP_CLK_DIV_NUM_MASK) >> Self::PUMP_CLK_DIV_NUM_OFFSET
    }

    /// Write **PUMP_CLK_DIV_NUM** `[7:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_pump_clk_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PUMP_CLK_DIV_NUM_MASK)
                | ((value << Self::PUMP_CLK_DIV_NUM_OFFSET) & Self::PUMP_CLK_DIV_NUM_MASK)
        });
    }
}

/// CLK_CFG value type for batch read/modify/write operations
///
/// Use `clkcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkCfgVal {
    bits: u32,
}

impl ClkCfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CLK_SEL** `[31:31]`
    ///
    /// select pvt clk
    #[inline(always)]
    pub const fn clk_sel(&self) -> bool {
        (self.bits & ClkCfg::CLK_SEL_MASK) != 0
    }

    /// Read **MONITOR_CLK_PVT_EN** `[8:8]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_clk_pvt_en(&self) -> bool {
        (self.bits & ClkCfg::MONITOR_CLK_PVT_EN_MASK) != 0
    }

    /// Read **PUMP_CLK_DIV_NUM** `[7:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn pump_clk_div_num(&self) -> u32 {
        (self.bits & ClkCfg::PUMP_CLK_DIV_NUM_MASK) >> ClkCfg::PUMP_CLK_DIV_NUM_OFFSET
    }

    /// Set **CLK_SEL** `[31:31]`
    ///
    /// select pvt clk
    #[inline(always)]
    pub const fn with_clk_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkCfg::CLK_SEL_MASK
            } else {
                self.bits & !ClkCfg::CLK_SEL_MASK
            },
        }
    }

    /// Set **MONITOR_CLK_PVT_EN** `[8:8]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_clk_pvt_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkCfg::MONITOR_CLK_PVT_EN_MASK
            } else {
                self.bits & !ClkCfg::MONITOR_CLK_PVT_EN_MASK
            },
        }
    }

    /// Set **PUMP_CLK_DIV_NUM** `[7:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_pump_clk_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ClkCfg::PUMP_CLK_DIV_NUM_MASK)
                | ((value << ClkCfg::PUMP_CLK_DIV_NUM_OFFSET) & ClkCfg::PUMP_CLK_DIV_NUM_MASK),
        }
    }
}

impl Default for ClkCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ClkCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkCfgVal {
        ClkCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkCfgVal) -> ClkCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBIAS_CHANNEL_SEL0 Register
///
/// needs desc
///
/// Address offset: 0x0034
/// Reset value: 0x81020400
///
/// ## Fields
///
/// - **DBIAS_CHANNEL0_SEL** `[31:25]` (RW)
///
///   needs field desc
///
/// - **DBIAS_CHANNEL1_SEL** `[24:18]` (RW)
///
///   needs field desc
///
/// - **DBIAS_CHANNEL2_SEL** `[17:11]` (RW)
///
///   needs field desc
///
/// - **DBIAS_CHANNEL3_SEL** `[10:4]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct DbiasChannelSel0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiasChannelSel0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiasChannelSel0 {
    // ======== Field Constants ========

    /// **DBIAS_CHANNEL0_SEL**
    ///
    /// needs field desc
    ///
    /// Bits `[31:25]`, mask `0xfe000000`
    pub const DBIAS_CHANNEL0_SEL_OFFSET: u32 = 25;
    pub const DBIAS_CHANNEL0_SEL_MASK: u32 = 0xfe000000;

    /// **DBIAS_CHANNEL1_SEL**
    ///
    /// needs field desc
    ///
    /// Bits `[24:18]`, mask `0x01fc0000`
    pub const DBIAS_CHANNEL1_SEL_OFFSET: u32 = 18;
    pub const DBIAS_CHANNEL1_SEL_MASK: u32 = 0x01fc0000;

    /// **DBIAS_CHANNEL2_SEL**
    ///
    /// needs field desc
    ///
    /// Bits `[17:11]`, mask `0x0003f800`
    pub const DBIAS_CHANNEL2_SEL_OFFSET: u32 = 11;
    pub const DBIAS_CHANNEL2_SEL_MASK: u32 = 0x0003f800;

    /// **DBIAS_CHANNEL3_SEL**
    ///
    /// needs field desc
    ///
    /// Bits `[10:4]`, mask `0x000007f0`
    pub const DBIAS_CHANNEL3_SEL_OFFSET: u32 = 4;
    pub const DBIAS_CHANNEL3_SEL_MASK: u32 = 0x000007f0;

    // ======== Field Accessors ========

    /// **DBIAS_CHANNEL0_SEL** `[31:25]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_channel0_sel(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CHANNEL0_SEL_MASK) >> Self::DBIAS_CHANNEL0_SEL_OFFSET
    }

    /// Write **DBIAS_CHANNEL0_SEL** `[31:25]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_channel0_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CHANNEL0_SEL_MASK)
                | ((value << Self::DBIAS_CHANNEL0_SEL_OFFSET) & Self::DBIAS_CHANNEL0_SEL_MASK)
        });
    }

    /// **DBIAS_CHANNEL1_SEL** `[24:18]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_channel1_sel(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CHANNEL1_SEL_MASK) >> Self::DBIAS_CHANNEL1_SEL_OFFSET
    }

    /// Write **DBIAS_CHANNEL1_SEL** `[24:18]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_channel1_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CHANNEL1_SEL_MASK)
                | ((value << Self::DBIAS_CHANNEL1_SEL_OFFSET) & Self::DBIAS_CHANNEL1_SEL_MASK)
        });
    }

    /// **DBIAS_CHANNEL2_SEL** `[17:11]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_channel2_sel(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CHANNEL2_SEL_MASK) >> Self::DBIAS_CHANNEL2_SEL_OFFSET
    }

    /// Write **DBIAS_CHANNEL2_SEL** `[17:11]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_channel2_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CHANNEL2_SEL_MASK)
                | ((value << Self::DBIAS_CHANNEL2_SEL_OFFSET) & Self::DBIAS_CHANNEL2_SEL_MASK)
        });
    }

    /// **DBIAS_CHANNEL3_SEL** `[10:4]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_channel3_sel(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CHANNEL3_SEL_MASK) >> Self::DBIAS_CHANNEL3_SEL_OFFSET
    }

    /// Write **DBIAS_CHANNEL3_SEL** `[10:4]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_channel3_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CHANNEL3_SEL_MASK)
                | ((value << Self::DBIAS_CHANNEL3_SEL_OFFSET) & Self::DBIAS_CHANNEL3_SEL_MASK)
        });
    }
}

/// DBIAS_CHANNEL_SEL0 value type for batch read/modify/write operations
///
/// Use `dbiaschannelsel0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbiaschannelsel0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiasChannelSel0Val {
    bits: u32,
}

impl DbiasChannelSel0Val {
    /// Create from raw bits
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
        Self { bits: 0x81020400 }
    }

    /// Read **DBIAS_CHANNEL0_SEL** `[31:25]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_channel0_sel(&self) -> u32 {
        (self.bits & DbiasChannelSel0::DBIAS_CHANNEL0_SEL_MASK)
            >> DbiasChannelSel0::DBIAS_CHANNEL0_SEL_OFFSET
    }

    /// Read **DBIAS_CHANNEL1_SEL** `[24:18]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_channel1_sel(&self) -> u32 {
        (self.bits & DbiasChannelSel0::DBIAS_CHANNEL1_SEL_MASK)
            >> DbiasChannelSel0::DBIAS_CHANNEL1_SEL_OFFSET
    }

    /// Read **DBIAS_CHANNEL2_SEL** `[17:11]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_channel2_sel(&self) -> u32 {
        (self.bits & DbiasChannelSel0::DBIAS_CHANNEL2_SEL_MASK)
            >> DbiasChannelSel0::DBIAS_CHANNEL2_SEL_OFFSET
    }

    /// Read **DBIAS_CHANNEL3_SEL** `[10:4]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_channel3_sel(&self) -> u32 {
        (self.bits & DbiasChannelSel0::DBIAS_CHANNEL3_SEL_MASK)
            >> DbiasChannelSel0::DBIAS_CHANNEL3_SEL_OFFSET
    }

    /// Set **DBIAS_CHANNEL0_SEL** `[31:25]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_channel0_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasChannelSel0::DBIAS_CHANNEL0_SEL_MASK)
                | ((value << DbiasChannelSel0::DBIAS_CHANNEL0_SEL_OFFSET)
                    & DbiasChannelSel0::DBIAS_CHANNEL0_SEL_MASK),
        }
    }

    /// Set **DBIAS_CHANNEL1_SEL** `[24:18]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_channel1_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasChannelSel0::DBIAS_CHANNEL1_SEL_MASK)
                | ((value << DbiasChannelSel0::DBIAS_CHANNEL1_SEL_OFFSET)
                    & DbiasChannelSel0::DBIAS_CHANNEL1_SEL_MASK),
        }
    }

    /// Set **DBIAS_CHANNEL2_SEL** `[17:11]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_channel2_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasChannelSel0::DBIAS_CHANNEL2_SEL_MASK)
                | ((value << DbiasChannelSel0::DBIAS_CHANNEL2_SEL_OFFSET)
                    & DbiasChannelSel0::DBIAS_CHANNEL2_SEL_MASK),
        }
    }

    /// Set **DBIAS_CHANNEL3_SEL** `[10:4]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_channel3_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasChannelSel0::DBIAS_CHANNEL3_SEL_MASK)
                | ((value << DbiasChannelSel0::DBIAS_CHANNEL3_SEL_OFFSET)
                    & DbiasChannelSel0::DBIAS_CHANNEL3_SEL_MASK),
        }
    }
}

impl Default for DbiasChannelSel0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DbiasChannelSel0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiasChannelSel0Val {
        DbiasChannelSel0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiasChannelSel0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiasChannelSel0Val) -> DbiasChannelSel0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBIAS_CHANNEL_SEL1 Register
///
/// needs desc
///
/// Address offset: 0x0038
/// Reset value: 0x80000000
///
/// ## Fields
///
/// - **DBIAS_CHANNEL4_SEL** `[31:25]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct DbiasChannelSel1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiasChannelSel1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiasChannelSel1 {
    // ======== Field Constants ========

    /// **DBIAS_CHANNEL4_SEL**
    ///
    /// needs field desc
    ///
    /// Bits `[31:25]`, mask `0xfe000000`
    pub const DBIAS_CHANNEL4_SEL_OFFSET: u32 = 25;
    pub const DBIAS_CHANNEL4_SEL_MASK: u32 = 0xfe000000;

    // ======== Field Accessors ========

    /// **DBIAS_CHANNEL4_SEL** `[31:25]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_channel4_sel(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CHANNEL4_SEL_MASK) >> Self::DBIAS_CHANNEL4_SEL_OFFSET
    }

    /// Write **DBIAS_CHANNEL4_SEL** `[31:25]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_channel4_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CHANNEL4_SEL_MASK)
                | ((value << Self::DBIAS_CHANNEL4_SEL_OFFSET) & Self::DBIAS_CHANNEL4_SEL_MASK)
        });
    }
}

/// DBIAS_CHANNEL_SEL1 value type for batch read/modify/write operations
///
/// Use `dbiaschannelsel1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbiaschannelsel1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiasChannelSel1Val {
    bits: u32,
}

impl DbiasChannelSel1Val {
    /// Create from raw bits
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

    /// Read **DBIAS_CHANNEL4_SEL** `[31:25]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_channel4_sel(&self) -> u32 {
        (self.bits & DbiasChannelSel1::DBIAS_CHANNEL4_SEL_MASK)
            >> DbiasChannelSel1::DBIAS_CHANNEL4_SEL_OFFSET
    }

    /// Set **DBIAS_CHANNEL4_SEL** `[31:25]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_channel4_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasChannelSel1::DBIAS_CHANNEL4_SEL_MASK)
                | ((value << DbiasChannelSel1::DBIAS_CHANNEL4_SEL_OFFSET)
                    & DbiasChannelSel1::DBIAS_CHANNEL4_SEL_MASK),
        }
    }
}

impl Default for DbiasChannelSel1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DbiasChannelSel1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiasChannelSel1Val {
        DbiasChannelSel1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiasChannelSel1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiasChannelSel1Val) -> DbiasChannelSel1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBIAS_CHANNEL0_SEL Register
///
/// needs desc
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **DBIAS_CHANNEL0_CFG** `[16:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct DbiasChannel0Sel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiasChannel0Sel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiasChannel0Sel {
    // ======== Field Constants ========

    /// **DBIAS_CHANNEL0_CFG**
    ///
    /// needs field desc
    ///
    /// Bits `[16:0]`, mask `0x0001ffff`
    pub const DBIAS_CHANNEL0_CFG_OFFSET: u32 = 0;
    pub const DBIAS_CHANNEL0_CFG_MASK: u32 = 0x0001ffff;

    // ======== Field Accessors ========

    /// **DBIAS_CHANNEL0_CFG** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_channel0_cfg(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CHANNEL0_CFG_MASK) >> Self::DBIAS_CHANNEL0_CFG_OFFSET
    }

    /// Write **DBIAS_CHANNEL0_CFG** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_channel0_cfg(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CHANNEL0_CFG_MASK)
                | ((value << Self::DBIAS_CHANNEL0_CFG_OFFSET) & Self::DBIAS_CHANNEL0_CFG_MASK)
        });
    }
}

/// DBIAS_CHANNEL0_SEL value type for batch read/modify/write operations
///
/// Use `dbiaschannel0sel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbiaschannel0sel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiasChannel0SelVal {
    bits: u32,
}

impl DbiasChannel0SelVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DBIAS_CHANNEL0_CFG** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_channel0_cfg(&self) -> u32 {
        (self.bits & DbiasChannel0Sel::DBIAS_CHANNEL0_CFG_MASK)
            >> DbiasChannel0Sel::DBIAS_CHANNEL0_CFG_OFFSET
    }

    /// Set **DBIAS_CHANNEL0_CFG** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_channel0_cfg(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasChannel0Sel::DBIAS_CHANNEL0_CFG_MASK)
                | ((value << DbiasChannel0Sel::DBIAS_CHANNEL0_CFG_OFFSET)
                    & DbiasChannel0Sel::DBIAS_CHANNEL0_CFG_MASK),
        }
    }
}

impl Default for DbiasChannel0SelVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DbiasChannel0Sel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiasChannel0SelVal {
        DbiasChannel0SelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiasChannel0SelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiasChannel0SelVal) -> DbiasChannel0SelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBIAS_CHANNEL1_SEL Register
///
/// needs desc
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **DBIAS_CHANNEL1_CFG** `[16:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct DbiasChannel1Sel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiasChannel1Sel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiasChannel1Sel {
    // ======== Field Constants ========

    /// **DBIAS_CHANNEL1_CFG**
    ///
    /// needs field desc
    ///
    /// Bits `[16:0]`, mask `0x0001ffff`
    pub const DBIAS_CHANNEL1_CFG_OFFSET: u32 = 0;
    pub const DBIAS_CHANNEL1_CFG_MASK: u32 = 0x0001ffff;

    // ======== Field Accessors ========

    /// **DBIAS_CHANNEL1_CFG** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_channel1_cfg(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CHANNEL1_CFG_MASK) >> Self::DBIAS_CHANNEL1_CFG_OFFSET
    }

    /// Write **DBIAS_CHANNEL1_CFG** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_channel1_cfg(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CHANNEL1_CFG_MASK)
                | ((value << Self::DBIAS_CHANNEL1_CFG_OFFSET) & Self::DBIAS_CHANNEL1_CFG_MASK)
        });
    }
}

/// DBIAS_CHANNEL1_SEL value type for batch read/modify/write operations
///
/// Use `dbiaschannel1sel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbiaschannel1sel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiasChannel1SelVal {
    bits: u32,
}

impl DbiasChannel1SelVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DBIAS_CHANNEL1_CFG** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_channel1_cfg(&self) -> u32 {
        (self.bits & DbiasChannel1Sel::DBIAS_CHANNEL1_CFG_MASK)
            >> DbiasChannel1Sel::DBIAS_CHANNEL1_CFG_OFFSET
    }

    /// Set **DBIAS_CHANNEL1_CFG** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_channel1_cfg(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasChannel1Sel::DBIAS_CHANNEL1_CFG_MASK)
                | ((value << DbiasChannel1Sel::DBIAS_CHANNEL1_CFG_OFFSET)
                    & DbiasChannel1Sel::DBIAS_CHANNEL1_CFG_MASK),
        }
    }
}

impl Default for DbiasChannel1SelVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DbiasChannel1Sel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiasChannel1SelVal {
        DbiasChannel1SelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiasChannel1SelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiasChannel1SelVal) -> DbiasChannel1SelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBIAS_CHANNEL2_SEL Register
///
/// needs desc
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **DBIAS_CHANNEL2_CFG** `[16:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct DbiasChannel2Sel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiasChannel2Sel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiasChannel2Sel {
    // ======== Field Constants ========

    /// **DBIAS_CHANNEL2_CFG**
    ///
    /// needs field desc
    ///
    /// Bits `[16:0]`, mask `0x0001ffff`
    pub const DBIAS_CHANNEL2_CFG_OFFSET: u32 = 0;
    pub const DBIAS_CHANNEL2_CFG_MASK: u32 = 0x0001ffff;

    // ======== Field Accessors ========

    /// **DBIAS_CHANNEL2_CFG** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_channel2_cfg(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CHANNEL2_CFG_MASK) >> Self::DBIAS_CHANNEL2_CFG_OFFSET
    }

    /// Write **DBIAS_CHANNEL2_CFG** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_channel2_cfg(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CHANNEL2_CFG_MASK)
                | ((value << Self::DBIAS_CHANNEL2_CFG_OFFSET) & Self::DBIAS_CHANNEL2_CFG_MASK)
        });
    }
}

/// DBIAS_CHANNEL2_SEL value type for batch read/modify/write operations
///
/// Use `dbiaschannel2sel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbiaschannel2sel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiasChannel2SelVal {
    bits: u32,
}

impl DbiasChannel2SelVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DBIAS_CHANNEL2_CFG** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_channel2_cfg(&self) -> u32 {
        (self.bits & DbiasChannel2Sel::DBIAS_CHANNEL2_CFG_MASK)
            >> DbiasChannel2Sel::DBIAS_CHANNEL2_CFG_OFFSET
    }

    /// Set **DBIAS_CHANNEL2_CFG** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_channel2_cfg(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasChannel2Sel::DBIAS_CHANNEL2_CFG_MASK)
                | ((value << DbiasChannel2Sel::DBIAS_CHANNEL2_CFG_OFFSET)
                    & DbiasChannel2Sel::DBIAS_CHANNEL2_CFG_MASK),
        }
    }
}

impl Default for DbiasChannel2SelVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DbiasChannel2Sel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiasChannel2SelVal {
        DbiasChannel2SelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiasChannel2SelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiasChannel2SelVal) -> DbiasChannel2SelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBIAS_CHANNEL3_SEL Register
///
/// needs desc
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **DBIAS_CHANNEL3_CFG** `[16:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct DbiasChannel3Sel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiasChannel3Sel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiasChannel3Sel {
    // ======== Field Constants ========

    /// **DBIAS_CHANNEL3_CFG**
    ///
    /// needs field desc
    ///
    /// Bits `[16:0]`, mask `0x0001ffff`
    pub const DBIAS_CHANNEL3_CFG_OFFSET: u32 = 0;
    pub const DBIAS_CHANNEL3_CFG_MASK: u32 = 0x0001ffff;

    // ======== Field Accessors ========

    /// **DBIAS_CHANNEL3_CFG** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_channel3_cfg(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CHANNEL3_CFG_MASK) >> Self::DBIAS_CHANNEL3_CFG_OFFSET
    }

    /// Write **DBIAS_CHANNEL3_CFG** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_channel3_cfg(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CHANNEL3_CFG_MASK)
                | ((value << Self::DBIAS_CHANNEL3_CFG_OFFSET) & Self::DBIAS_CHANNEL3_CFG_MASK)
        });
    }
}

/// DBIAS_CHANNEL3_SEL value type for batch read/modify/write operations
///
/// Use `dbiaschannel3sel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbiaschannel3sel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiasChannel3SelVal {
    bits: u32,
}

impl DbiasChannel3SelVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DBIAS_CHANNEL3_CFG** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_channel3_cfg(&self) -> u32 {
        (self.bits & DbiasChannel3Sel::DBIAS_CHANNEL3_CFG_MASK)
            >> DbiasChannel3Sel::DBIAS_CHANNEL3_CFG_OFFSET
    }

    /// Set **DBIAS_CHANNEL3_CFG** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_channel3_cfg(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasChannel3Sel::DBIAS_CHANNEL3_CFG_MASK)
                | ((value << DbiasChannel3Sel::DBIAS_CHANNEL3_CFG_OFFSET)
                    & DbiasChannel3Sel::DBIAS_CHANNEL3_CFG_MASK),
        }
    }
}

impl Default for DbiasChannel3SelVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DbiasChannel3Sel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiasChannel3SelVal {
        DbiasChannel3SelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiasChannel3SelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiasChannel3SelVal) -> DbiasChannel3SelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBIAS_CHANNEL4_SEL Register
///
/// needs desc
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **DBIAS_CHANNEL4_CFG** `[16:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct DbiasChannel4Sel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiasChannel4Sel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiasChannel4Sel {
    // ======== Field Constants ========

    /// **DBIAS_CHANNEL4_CFG**
    ///
    /// needs field desc
    ///
    /// Bits `[16:0]`, mask `0x0001ffff`
    pub const DBIAS_CHANNEL4_CFG_OFFSET: u32 = 0;
    pub const DBIAS_CHANNEL4_CFG_MASK: u32 = 0x0001ffff;

    // ======== Field Accessors ========

    /// **DBIAS_CHANNEL4_CFG** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_channel4_cfg(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CHANNEL4_CFG_MASK) >> Self::DBIAS_CHANNEL4_CFG_OFFSET
    }

    /// Write **DBIAS_CHANNEL4_CFG** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_channel4_cfg(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CHANNEL4_CFG_MASK)
                | ((value << Self::DBIAS_CHANNEL4_CFG_OFFSET) & Self::DBIAS_CHANNEL4_CFG_MASK)
        });
    }
}

/// DBIAS_CHANNEL4_SEL value type for batch read/modify/write operations
///
/// Use `dbiaschannel4sel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbiaschannel4sel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiasChannel4SelVal {
    bits: u32,
}

impl DbiasChannel4SelVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DBIAS_CHANNEL4_CFG** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_channel4_cfg(&self) -> u32 {
        (self.bits & DbiasChannel4Sel::DBIAS_CHANNEL4_CFG_MASK)
            >> DbiasChannel4Sel::DBIAS_CHANNEL4_CFG_OFFSET
    }

    /// Set **DBIAS_CHANNEL4_CFG** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_channel4_cfg(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasChannel4Sel::DBIAS_CHANNEL4_CFG_MASK)
                | ((value << DbiasChannel4Sel::DBIAS_CHANNEL4_CFG_OFFSET)
                    & DbiasChannel4Sel::DBIAS_CHANNEL4_CFG_MASK),
        }
    }
}

impl Default for DbiasChannel4SelVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DbiasChannel4Sel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiasChannel4SelVal {
        DbiasChannel4SelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiasChannel4SelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiasChannel4SelVal) -> DbiasChannel4SelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBIAS_CMD0 Register
///
/// needs desc
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **DBIAS_CMD0** `[16:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct DbiasCmd0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiasCmd0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiasCmd0 {
    // ======== Field Constants ========

    /// **DBIAS_CMD0**
    ///
    /// needs field desc
    ///
    /// Bits `[16:0]`, mask `0x0001ffff`
    pub const DBIAS_CMD0_OFFSET: u32 = 0;
    pub const DBIAS_CMD0_MASK: u32 = 0x0001ffff;

    // ======== Field Accessors ========

    /// **DBIAS_CMD0** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_cmd0(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CMD0_MASK) >> Self::DBIAS_CMD0_OFFSET
    }

    /// Write **DBIAS_CMD0** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_cmd0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CMD0_MASK)
                | ((value << Self::DBIAS_CMD0_OFFSET) & Self::DBIAS_CMD0_MASK)
        });
    }
}

/// DBIAS_CMD0 value type for batch read/modify/write operations
///
/// Use `dbiascmd0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbiascmd0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiasCmd0Val {
    bits: u32,
}

impl DbiasCmd0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DBIAS_CMD0** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_cmd0(&self) -> u32 {
        (self.bits & DbiasCmd0::DBIAS_CMD0_MASK) >> DbiasCmd0::DBIAS_CMD0_OFFSET
    }

    /// Set **DBIAS_CMD0** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_cmd0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasCmd0::DBIAS_CMD0_MASK)
                | ((value << DbiasCmd0::DBIAS_CMD0_OFFSET) & DbiasCmd0::DBIAS_CMD0_MASK),
        }
    }
}

impl Default for DbiasCmd0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DbiasCmd0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiasCmd0Val {
        DbiasCmd0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiasCmd0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiasCmd0Val) -> DbiasCmd0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBIAS_CMD1 Register
///
/// needs desc
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **DBIAS_CMD1** `[16:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct DbiasCmd1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiasCmd1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiasCmd1 {
    // ======== Field Constants ========

    /// **DBIAS_CMD1**
    ///
    /// needs field desc
    ///
    /// Bits `[16:0]`, mask `0x0001ffff`
    pub const DBIAS_CMD1_OFFSET: u32 = 0;
    pub const DBIAS_CMD1_MASK: u32 = 0x0001ffff;

    // ======== Field Accessors ========

    /// **DBIAS_CMD1** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_cmd1(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CMD1_MASK) >> Self::DBIAS_CMD1_OFFSET
    }

    /// Write **DBIAS_CMD1** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_cmd1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CMD1_MASK)
                | ((value << Self::DBIAS_CMD1_OFFSET) & Self::DBIAS_CMD1_MASK)
        });
    }
}

/// DBIAS_CMD1 value type for batch read/modify/write operations
///
/// Use `dbiascmd1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbiascmd1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiasCmd1Val {
    bits: u32,
}

impl DbiasCmd1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DBIAS_CMD1** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_cmd1(&self) -> u32 {
        (self.bits & DbiasCmd1::DBIAS_CMD1_MASK) >> DbiasCmd1::DBIAS_CMD1_OFFSET
    }

    /// Set **DBIAS_CMD1** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_cmd1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasCmd1::DBIAS_CMD1_MASK)
                | ((value << DbiasCmd1::DBIAS_CMD1_OFFSET) & DbiasCmd1::DBIAS_CMD1_MASK),
        }
    }
}

impl Default for DbiasCmd1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DbiasCmd1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiasCmd1Val {
        DbiasCmd1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiasCmd1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiasCmd1Val) -> DbiasCmd1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBIAS_CMD2 Register
///
/// needs desc
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **DBIAS_CMD2** `[16:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct DbiasCmd2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiasCmd2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiasCmd2 {
    // ======== Field Constants ========

    /// **DBIAS_CMD2**
    ///
    /// needs field desc
    ///
    /// Bits `[16:0]`, mask `0x0001ffff`
    pub const DBIAS_CMD2_OFFSET: u32 = 0;
    pub const DBIAS_CMD2_MASK: u32 = 0x0001ffff;

    // ======== Field Accessors ========

    /// **DBIAS_CMD2** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_cmd2(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CMD2_MASK) >> Self::DBIAS_CMD2_OFFSET
    }

    /// Write **DBIAS_CMD2** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_cmd2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CMD2_MASK)
                | ((value << Self::DBIAS_CMD2_OFFSET) & Self::DBIAS_CMD2_MASK)
        });
    }
}

/// DBIAS_CMD2 value type for batch read/modify/write operations
///
/// Use `dbiascmd2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbiascmd2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiasCmd2Val {
    bits: u32,
}

impl DbiasCmd2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DBIAS_CMD2** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_cmd2(&self) -> u32 {
        (self.bits & DbiasCmd2::DBIAS_CMD2_MASK) >> DbiasCmd2::DBIAS_CMD2_OFFSET
    }

    /// Set **DBIAS_CMD2** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_cmd2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasCmd2::DBIAS_CMD2_MASK)
                | ((value << DbiasCmd2::DBIAS_CMD2_OFFSET) & DbiasCmd2::DBIAS_CMD2_MASK),
        }
    }
}

impl Default for DbiasCmd2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DbiasCmd2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiasCmd2Val {
        DbiasCmd2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiasCmd2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiasCmd2Val) -> DbiasCmd2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBIAS_CMD3 Register
///
/// needs desc
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **DBIAS_CMD3** `[16:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct DbiasCmd3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiasCmd3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiasCmd3 {
    // ======== Field Constants ========

    /// **DBIAS_CMD3**
    ///
    /// needs field desc
    ///
    /// Bits `[16:0]`, mask `0x0001ffff`
    pub const DBIAS_CMD3_OFFSET: u32 = 0;
    pub const DBIAS_CMD3_MASK: u32 = 0x0001ffff;

    // ======== Field Accessors ========

    /// **DBIAS_CMD3** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_cmd3(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CMD3_MASK) >> Self::DBIAS_CMD3_OFFSET
    }

    /// Write **DBIAS_CMD3** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_cmd3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CMD3_MASK)
                | ((value << Self::DBIAS_CMD3_OFFSET) & Self::DBIAS_CMD3_MASK)
        });
    }
}

/// DBIAS_CMD3 value type for batch read/modify/write operations
///
/// Use `dbiascmd3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbiascmd3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiasCmd3Val {
    bits: u32,
}

impl DbiasCmd3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DBIAS_CMD3** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_cmd3(&self) -> u32 {
        (self.bits & DbiasCmd3::DBIAS_CMD3_MASK) >> DbiasCmd3::DBIAS_CMD3_OFFSET
    }

    /// Set **DBIAS_CMD3** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_cmd3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasCmd3::DBIAS_CMD3_MASK)
                | ((value << DbiasCmd3::DBIAS_CMD3_OFFSET) & DbiasCmd3::DBIAS_CMD3_MASK),
        }
    }
}

impl Default for DbiasCmd3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DbiasCmd3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiasCmd3Val {
        DbiasCmd3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiasCmd3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiasCmd3Val) -> DbiasCmd3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBIAS_CMD4 Register
///
/// needs desc
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **DBIAS_CMD4** `[16:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct DbiasCmd4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiasCmd4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiasCmd4 {
    // ======== Field Constants ========

    /// **DBIAS_CMD4**
    ///
    /// needs field desc
    ///
    /// Bits `[16:0]`, mask `0x0001ffff`
    pub const DBIAS_CMD4_OFFSET: u32 = 0;
    pub const DBIAS_CMD4_MASK: u32 = 0x0001ffff;

    // ======== Field Accessors ========

    /// **DBIAS_CMD4** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn dbias_cmd4(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_CMD4_MASK) >> Self::DBIAS_CMD4_OFFSET
    }

    /// Write **DBIAS_CMD4** `[16:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_dbias_cmd4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DBIAS_CMD4_MASK)
                | ((value << Self::DBIAS_CMD4_OFFSET) & Self::DBIAS_CMD4_MASK)
        });
    }
}

/// DBIAS_CMD4 value type for batch read/modify/write operations
///
/// Use `dbiascmd4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbiascmd4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiasCmd4Val {
    bits: u32,
}

impl DbiasCmd4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DBIAS_CMD4** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn dbias_cmd4(&self) -> u32 {
        (self.bits & DbiasCmd4::DBIAS_CMD4_MASK) >> DbiasCmd4::DBIAS_CMD4_OFFSET
    }

    /// Set **DBIAS_CMD4** `[16:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_dbias_cmd4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasCmd4::DBIAS_CMD4_MASK)
                | ((value << DbiasCmd4::DBIAS_CMD4_OFFSET) & DbiasCmd4::DBIAS_CMD4_MASK),
        }
    }
}

impl Default for DbiasCmd4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DbiasCmd4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiasCmd4Val {
        DbiasCmd4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiasCmd4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiasCmd4Val) -> DbiasCmd4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DBIAS_TIMER Register
///
/// needs desc
///
/// Address offset: 0x0064
/// Reset value: 0x7fff8000
///
/// ## Fields
///
/// - **TIMER_EN** `[31]` (RW)
///
///   needs field desc
///
/// - **TIMER_TARGET** `[30:15]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct DbiasTimer {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DbiasTimer {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DbiasTimer {
    // ======== Field Constants ========

    /// **TIMER_EN**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMER_EN_OFFSET: u32 = 31;
    pub const TIMER_EN_MASK: u32 = 0x80000000;
    pub const TIMER_EN: u32 = 1 << 31;

    /// **TIMER_TARGET**
    ///
    /// needs field desc
    ///
    /// Bits `[30:15]`, mask `0x7fff8000`
    pub const TIMER_TARGET_OFFSET: u32 = 15;
    pub const TIMER_TARGET_MASK: u32 = 0x7fff8000;

    // ======== Field Accessors ========

    /// **TIMER_EN** `[31]` (RW)
    ///
    /// needs field desc
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

    /// **TIMER_TARGET** `[30:15]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timer_target(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_TARGET_MASK) >> Self::TIMER_TARGET_OFFSET
    }

    /// Write **TIMER_TARGET** `[30:15]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_timer_target(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_TARGET_MASK)
                | ((value << Self::TIMER_TARGET_OFFSET) & Self::TIMER_TARGET_MASK)
        });
    }
}

/// DBIAS_TIMER value type for batch read/modify/write operations
///
/// Use `dbiastimer.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dbiastimer.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DbiasTimerVal {
    bits: u32,
}

impl DbiasTimerVal {
    /// Create from raw bits
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
        Self { bits: 0x7fff8000 }
    }

    /// Read **TIMER_EN** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timer_en(&self) -> bool {
        (self.bits & DbiasTimer::TIMER_EN_MASK) != 0
    }

    /// Read **TIMER_TARGET** `[30:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timer_target(&self) -> u32 {
        (self.bits & DbiasTimer::TIMER_TARGET_MASK) >> DbiasTimer::TIMER_TARGET_OFFSET
    }

    /// Set **TIMER_EN** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timer_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DbiasTimer::TIMER_EN_MASK
            } else {
                self.bits & !DbiasTimer::TIMER_EN_MASK
            },
        }
    }

    /// Set **TIMER_TARGET** `[30:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timer_target(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DbiasTimer::TIMER_TARGET_MASK)
                | ((value << DbiasTimer::TIMER_TARGET_OFFSET) & DbiasTimer::TIMER_TARGET_MASK),
        }
    }
}

impl Default for DbiasTimerVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DbiasTimer {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DbiasTimerVal {
        DbiasTimerVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DbiasTimerVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DbiasTimerVal) -> DbiasTimerVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT0_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0068
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE0_UNIT0** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE0_UNIT0** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE0_UNIT0** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT0** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE0_UNIT0** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit0Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit0Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit0Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE0_UNIT0_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE0_UNIT0_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE0_UNIT0: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE0_UNIT0_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE0_UNIT0_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE0_UNIT0_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE0_UNIT0_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT0_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT0_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT0: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE0_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE0_UNIT0_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE0_UNIT0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE0_UNIT0** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site0_unit0(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE0_UNIT0_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE0_UNIT0** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site0_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE0_UNIT0_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE0_UNIT0_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE0_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site0_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT0_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT0_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE0_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site0_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT0_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT0_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT0_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT0** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site0_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT0_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE0_UNIT0** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site0_unit0(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE0_UNIT0_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE0_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site0_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE0_UNIT0_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE0_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site0_unit0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE0_UNIT0_MASK);
    }
}

/// COMB_PD_SITE0_UNIT0_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit0vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit0vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit0Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite0Unit0Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE0_UNIT0** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site0_unit0(&self) -> bool {
        (self.bits & CombPdSite0Unit0Vt0Conf1::TIMING_ERR_VT0_PD_SITE0_UNIT0_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE0_UNIT0** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site0_unit0(&self) -> u32 {
        (self.bits & CombPdSite0Unit0Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE0_UNIT0_MASK)
            >> CombPdSite0Unit0Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE0_UNIT0_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE0_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site0_unit0(&self) -> u32 {
        (self.bits & CombPdSite0Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT0_MASK)
            >> CombPdSite0Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT0_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE0_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site0_unit0(&self) -> bool {
        (self.bits & CombPdSite0Unit0Vt0Conf1::MONITOR_EN_VT0_PD_SITE0_UNIT0_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE0_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site0_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT0_MASK)
                | ((value << CombPdSite0Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT0_OFFSET)
                    & CombPdSite0Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT0_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT0** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site0_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit0Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT0_MASK
            } else {
                self.bits & !CombPdSite0Unit0Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT0_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE0_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site0_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit0Vt0Conf1::MONITOR_EN_VT0_PD_SITE0_UNIT0_MASK
            } else {
                self.bits & !CombPdSite0Unit0Vt0Conf1::MONITOR_EN_VT0_PD_SITE0_UNIT0_MASK
            },
        }
    }
}

impl Default for CombPdSite0Unit0Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite0Unit0Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit0Vt0Conf1Val {
        CombPdSite0Unit0Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit0Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit0Vt0Conf1Val) -> CombPdSite0Unit0Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT1_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x006c
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE0_UNIT1** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE0_UNIT1** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE0_UNIT1** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT1** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE0_UNIT1** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit1Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit1Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit1Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE0_UNIT1_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE0_UNIT1_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE0_UNIT1: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE0_UNIT1_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE0_UNIT1_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE0_UNIT1_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE0_UNIT1_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT1_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT1_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT1: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE0_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE0_UNIT1_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE0_UNIT1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE0_UNIT1** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site0_unit1(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE0_UNIT1_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE0_UNIT1** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site0_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE0_UNIT1_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE0_UNIT1_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE0_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site0_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT1_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT1_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE0_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site0_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT1_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT1_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT1_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT1** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site0_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT1_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE0_UNIT1** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site0_unit1(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE0_UNIT1_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE0_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site0_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE0_UNIT1_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE0_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site0_unit1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE0_UNIT1_MASK);
    }
}

/// COMB_PD_SITE0_UNIT1_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit1vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit1vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit1Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite0Unit1Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE0_UNIT1** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site0_unit1(&self) -> bool {
        (self.bits & CombPdSite0Unit1Vt0Conf1::TIMING_ERR_VT0_PD_SITE0_UNIT1_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE0_UNIT1** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site0_unit1(&self) -> u32 {
        (self.bits & CombPdSite0Unit1Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE0_UNIT1_MASK)
            >> CombPdSite0Unit1Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE0_UNIT1_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE0_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site0_unit1(&self) -> u32 {
        (self.bits & CombPdSite0Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT1_MASK)
            >> CombPdSite0Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT1_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE0_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site0_unit1(&self) -> bool {
        (self.bits & CombPdSite0Unit1Vt0Conf1::MONITOR_EN_VT0_PD_SITE0_UNIT1_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE0_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site0_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT1_MASK)
                | ((value << CombPdSite0Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT1_OFFSET)
                    & CombPdSite0Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT1_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT1** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site0_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit1Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT1_MASK
            } else {
                self.bits & !CombPdSite0Unit1Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT1_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE0_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site0_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit1Vt0Conf1::MONITOR_EN_VT0_PD_SITE0_UNIT1_MASK
            } else {
                self.bits & !CombPdSite0Unit1Vt0Conf1::MONITOR_EN_VT0_PD_SITE0_UNIT1_MASK
            },
        }
    }
}

impl Default for CombPdSite0Unit1Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite0Unit1Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit1Vt0Conf1Val {
        CombPdSite0Unit1Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit1Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit1Vt0Conf1Val) -> CombPdSite0Unit1Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT2_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0070
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE0_UNIT2** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE0_UNIT2** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE0_UNIT2** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT2** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE0_UNIT2** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit2Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit2Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit2Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE0_UNIT2_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE0_UNIT2_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE0_UNIT2: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE0_UNIT2_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE0_UNIT2_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE0_UNIT2_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE0_UNIT2_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT2_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT2_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT2: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE0_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE0_UNIT2_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE0_UNIT2: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE0_UNIT2** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site0_unit2(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE0_UNIT2_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE0_UNIT2** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site0_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE0_UNIT2_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE0_UNIT2_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE0_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site0_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT2_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT2_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE0_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site0_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT2_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT2_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT2_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT2** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site0_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT2_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE0_UNIT2** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site0_unit2(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE0_UNIT2_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE0_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site0_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE0_UNIT2_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE0_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site0_unit2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE0_UNIT2_MASK);
    }
}

/// COMB_PD_SITE0_UNIT2_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit2vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit2vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit2Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite0Unit2Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE0_UNIT2** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site0_unit2(&self) -> bool {
        (self.bits & CombPdSite0Unit2Vt0Conf1::TIMING_ERR_VT0_PD_SITE0_UNIT2_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE0_UNIT2** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site0_unit2(&self) -> u32 {
        (self.bits & CombPdSite0Unit2Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE0_UNIT2_MASK)
            >> CombPdSite0Unit2Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE0_UNIT2_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE0_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site0_unit2(&self) -> u32 {
        (self.bits & CombPdSite0Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT2_MASK)
            >> CombPdSite0Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT2_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE0_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site0_unit2(&self) -> bool {
        (self.bits & CombPdSite0Unit2Vt0Conf1::MONITOR_EN_VT0_PD_SITE0_UNIT2_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE0_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site0_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT2_MASK)
                | ((value << CombPdSite0Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT2_OFFSET)
                    & CombPdSite0Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT2_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT2** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site0_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit2Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT2_MASK
            } else {
                self.bits & !CombPdSite0Unit2Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT2_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE0_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site0_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit2Vt0Conf1::MONITOR_EN_VT0_PD_SITE0_UNIT2_MASK
            } else {
                self.bits & !CombPdSite0Unit2Vt0Conf1::MONITOR_EN_VT0_PD_SITE0_UNIT2_MASK
            },
        }
    }
}

impl Default for CombPdSite0Unit2Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite0Unit2Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit2Vt0Conf1Val {
        CombPdSite0Unit2Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit2Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit2Vt0Conf1Val) -> CombPdSite0Unit2Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT3_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0074
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE0_UNIT3** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE0_UNIT3** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE0_UNIT3** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT3** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE0_UNIT3** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit3Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit3Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit3Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE0_UNIT3_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE0_UNIT3_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE0_UNIT3: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE0_UNIT3_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE0_UNIT3_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE0_UNIT3_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE0_UNIT3_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT3_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT3_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT3: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE0_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE0_UNIT3_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE0_UNIT3: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE0_UNIT3** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site0_unit3(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE0_UNIT3_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE0_UNIT3** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site0_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE0_UNIT3_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE0_UNIT3_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE0_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site0_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT3_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT3_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE0_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site0_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT3_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT3_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE0_UNIT3_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT3** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site0_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT3_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE0_UNIT3** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site0_unit3(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE0_UNIT3_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE0_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site0_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE0_UNIT3_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE0_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site0_unit3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE0_UNIT3_MASK);
    }
}

/// COMB_PD_SITE0_UNIT3_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit3vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit3vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit3Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite0Unit3Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE0_UNIT3** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site0_unit3(&self) -> bool {
        (self.bits & CombPdSite0Unit3Vt0Conf1::TIMING_ERR_VT0_PD_SITE0_UNIT3_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE0_UNIT3** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site0_unit3(&self) -> u32 {
        (self.bits & CombPdSite0Unit3Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE0_UNIT3_MASK)
            >> CombPdSite0Unit3Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE0_UNIT3_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE0_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site0_unit3(&self) -> u32 {
        (self.bits & CombPdSite0Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT3_MASK)
            >> CombPdSite0Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT3_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE0_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site0_unit3(&self) -> bool {
        (self.bits & CombPdSite0Unit3Vt0Conf1::MONITOR_EN_VT0_PD_SITE0_UNIT3_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE0_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site0_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT3_MASK)
                | ((value << CombPdSite0Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT3_OFFSET)
                    & CombPdSite0Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE0_UNIT3_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT3** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site0_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit3Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT3_MASK
            } else {
                self.bits & !CombPdSite0Unit3Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE0_UNIT3_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE0_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site0_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit3Vt0Conf1::MONITOR_EN_VT0_PD_SITE0_UNIT3_MASK
            } else {
                self.bits & !CombPdSite0Unit3Vt0Conf1::MONITOR_EN_VT0_PD_SITE0_UNIT3_MASK
            },
        }
    }
}

impl Default for CombPdSite0Unit3Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite0Unit3Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit3Vt0Conf1Val {
        CombPdSite0Unit3Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit3Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit3Vt0Conf1Val) -> CombPdSite0Unit3Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT0_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0078
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE0_UNIT0** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE0_UNIT0** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE0_UNIT0** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT0** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE0_UNIT0** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit0Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit0Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit0Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE0_UNIT0_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE0_UNIT0_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE0_UNIT0: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE0_UNIT0_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE0_UNIT0_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE0_UNIT0_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE0_UNIT0_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT0_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT0_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT0: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE0_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE0_UNIT0_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE0_UNIT0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE0_UNIT0** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site0_unit0(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE0_UNIT0_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE0_UNIT0** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site0_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE0_UNIT0_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE0_UNIT0_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE0_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site0_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT0_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT0_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE0_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site0_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT0_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT0_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT0_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT0** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site0_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT0_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE0_UNIT0** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site0_unit0(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE0_UNIT0_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE0_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site0_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE0_UNIT0_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE0_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site0_unit0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE0_UNIT0_MASK);
    }
}

/// COMB_PD_SITE0_UNIT0_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit0vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit0vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit0Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite0Unit0Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE0_UNIT0** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site0_unit0(&self) -> bool {
        (self.bits & CombPdSite0Unit0Vt1Conf1::TIMING_ERR_VT1_PD_SITE0_UNIT0_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE0_UNIT0** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site0_unit0(&self) -> u32 {
        (self.bits & CombPdSite0Unit0Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE0_UNIT0_MASK)
            >> CombPdSite0Unit0Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE0_UNIT0_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE0_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site0_unit0(&self) -> u32 {
        (self.bits & CombPdSite0Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT0_MASK)
            >> CombPdSite0Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT0_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE0_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site0_unit0(&self) -> bool {
        (self.bits & CombPdSite0Unit0Vt1Conf1::MONITOR_EN_VT1_PD_SITE0_UNIT0_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE0_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site0_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT0_MASK)
                | ((value << CombPdSite0Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT0_OFFSET)
                    & CombPdSite0Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT0_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT0** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site0_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit0Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT0_MASK
            } else {
                self.bits & !CombPdSite0Unit0Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT0_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE0_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site0_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit0Vt1Conf1::MONITOR_EN_VT1_PD_SITE0_UNIT0_MASK
            } else {
                self.bits & !CombPdSite0Unit0Vt1Conf1::MONITOR_EN_VT1_PD_SITE0_UNIT0_MASK
            },
        }
    }
}

impl Default for CombPdSite0Unit0Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite0Unit0Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit0Vt1Conf1Val {
        CombPdSite0Unit0Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit0Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit0Vt1Conf1Val) -> CombPdSite0Unit0Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT1_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x007c
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE0_UNIT1** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE0_UNIT1** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE0_UNIT1** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT1** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE0_UNIT1** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit1Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit1Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit1Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE0_UNIT1_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE0_UNIT1_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE0_UNIT1: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE0_UNIT1_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE0_UNIT1_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE0_UNIT1_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE0_UNIT1_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT1_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT1_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT1: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE0_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE0_UNIT1_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE0_UNIT1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE0_UNIT1** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site0_unit1(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE0_UNIT1_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE0_UNIT1** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site0_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE0_UNIT1_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE0_UNIT1_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE0_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site0_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT1_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT1_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE0_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site0_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT1_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT1_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT1_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT1** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site0_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT1_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE0_UNIT1** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site0_unit1(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE0_UNIT1_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE0_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site0_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE0_UNIT1_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE0_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site0_unit1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE0_UNIT1_MASK);
    }
}

/// COMB_PD_SITE0_UNIT1_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit1vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit1vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit1Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite0Unit1Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE0_UNIT1** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site0_unit1(&self) -> bool {
        (self.bits & CombPdSite0Unit1Vt1Conf1::TIMING_ERR_VT1_PD_SITE0_UNIT1_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE0_UNIT1** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site0_unit1(&self) -> u32 {
        (self.bits & CombPdSite0Unit1Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE0_UNIT1_MASK)
            >> CombPdSite0Unit1Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE0_UNIT1_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE0_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site0_unit1(&self) -> u32 {
        (self.bits & CombPdSite0Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT1_MASK)
            >> CombPdSite0Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT1_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE0_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site0_unit1(&self) -> bool {
        (self.bits & CombPdSite0Unit1Vt1Conf1::MONITOR_EN_VT1_PD_SITE0_UNIT1_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE0_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site0_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT1_MASK)
                | ((value << CombPdSite0Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT1_OFFSET)
                    & CombPdSite0Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT1_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT1** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site0_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit1Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT1_MASK
            } else {
                self.bits & !CombPdSite0Unit1Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT1_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE0_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site0_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit1Vt1Conf1::MONITOR_EN_VT1_PD_SITE0_UNIT1_MASK
            } else {
                self.bits & !CombPdSite0Unit1Vt1Conf1::MONITOR_EN_VT1_PD_SITE0_UNIT1_MASK
            },
        }
    }
}

impl Default for CombPdSite0Unit1Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite0Unit1Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit1Vt1Conf1Val {
        CombPdSite0Unit1Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit1Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit1Vt1Conf1Val) -> CombPdSite0Unit1Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT2_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0080
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE0_UNIT2** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE0_UNIT2** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE0_UNIT2** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT2** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE0_UNIT2** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit2Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit2Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit2Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE0_UNIT2_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE0_UNIT2_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE0_UNIT2: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE0_UNIT2_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE0_UNIT2_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE0_UNIT2_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE0_UNIT2_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT2_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT2_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT2: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE0_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE0_UNIT2_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE0_UNIT2: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE0_UNIT2** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site0_unit2(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE0_UNIT2_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE0_UNIT2** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site0_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE0_UNIT2_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE0_UNIT2_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE0_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site0_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT2_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT2_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE0_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site0_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT2_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT2_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT2_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT2** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site0_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT2_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE0_UNIT2** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site0_unit2(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE0_UNIT2_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE0_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site0_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE0_UNIT2_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE0_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site0_unit2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE0_UNIT2_MASK);
    }
}

/// COMB_PD_SITE0_UNIT2_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit2vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit2vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit2Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite0Unit2Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE0_UNIT2** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site0_unit2(&self) -> bool {
        (self.bits & CombPdSite0Unit2Vt1Conf1::TIMING_ERR_VT1_PD_SITE0_UNIT2_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE0_UNIT2** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site0_unit2(&self) -> u32 {
        (self.bits & CombPdSite0Unit2Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE0_UNIT2_MASK)
            >> CombPdSite0Unit2Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE0_UNIT2_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE0_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site0_unit2(&self) -> u32 {
        (self.bits & CombPdSite0Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT2_MASK)
            >> CombPdSite0Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT2_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE0_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site0_unit2(&self) -> bool {
        (self.bits & CombPdSite0Unit2Vt1Conf1::MONITOR_EN_VT1_PD_SITE0_UNIT2_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE0_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site0_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT2_MASK)
                | ((value << CombPdSite0Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT2_OFFSET)
                    & CombPdSite0Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT2_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT2** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site0_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit2Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT2_MASK
            } else {
                self.bits & !CombPdSite0Unit2Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT2_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE0_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site0_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit2Vt1Conf1::MONITOR_EN_VT1_PD_SITE0_UNIT2_MASK
            } else {
                self.bits & !CombPdSite0Unit2Vt1Conf1::MONITOR_EN_VT1_PD_SITE0_UNIT2_MASK
            },
        }
    }
}

impl Default for CombPdSite0Unit2Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite0Unit2Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit2Vt1Conf1Val {
        CombPdSite0Unit2Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit2Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit2Vt1Conf1Val) -> CombPdSite0Unit2Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT3_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0084
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE0_UNIT3** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE0_UNIT3** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE0_UNIT3** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT3** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE0_UNIT3** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit3Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit3Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit3Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE0_UNIT3_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE0_UNIT3_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE0_UNIT3: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE0_UNIT3_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE0_UNIT3_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE0_UNIT3_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE0_UNIT3_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT3_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT3_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT3: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE0_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE0_UNIT3_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE0_UNIT3: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE0_UNIT3** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site0_unit3(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE0_UNIT3_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE0_UNIT3** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site0_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE0_UNIT3_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE0_UNIT3_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE0_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site0_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT3_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT3_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE0_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site0_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT3_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT3_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE0_UNIT3_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT3** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site0_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT3_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE0_UNIT3** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site0_unit3(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE0_UNIT3_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE0_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site0_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE0_UNIT3_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE0_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site0_unit3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE0_UNIT3_MASK);
    }
}

/// COMB_PD_SITE0_UNIT3_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit3vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit3vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit3Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite0Unit3Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE0_UNIT3** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site0_unit3(&self) -> bool {
        (self.bits & CombPdSite0Unit3Vt1Conf1::TIMING_ERR_VT1_PD_SITE0_UNIT3_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE0_UNIT3** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site0_unit3(&self) -> u32 {
        (self.bits & CombPdSite0Unit3Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE0_UNIT3_MASK)
            >> CombPdSite0Unit3Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE0_UNIT3_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE0_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site0_unit3(&self) -> u32 {
        (self.bits & CombPdSite0Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT3_MASK)
            >> CombPdSite0Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT3_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE0_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site0_unit3(&self) -> bool {
        (self.bits & CombPdSite0Unit3Vt1Conf1::MONITOR_EN_VT1_PD_SITE0_UNIT3_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE0_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site0_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT3_MASK)
                | ((value << CombPdSite0Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT3_OFFSET)
                    & CombPdSite0Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE0_UNIT3_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT3** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site0_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit3Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT3_MASK
            } else {
                self.bits & !CombPdSite0Unit3Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE0_UNIT3_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE0_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site0_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit3Vt1Conf1::MONITOR_EN_VT1_PD_SITE0_UNIT3_MASK
            } else {
                self.bits & !CombPdSite0Unit3Vt1Conf1::MONITOR_EN_VT1_PD_SITE0_UNIT3_MASK
            },
        }
    }
}

impl Default for CombPdSite0Unit3Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite0Unit3Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit3Vt1Conf1Val {
        CombPdSite0Unit3Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit3Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit3Vt1Conf1Val) -> CombPdSite0Unit3Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT0_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0088
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE0_UNIT0** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE0_UNIT0** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE0_UNIT0** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT0** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE0_UNIT0** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit0Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit0Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit0Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE0_UNIT0_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE0_UNIT0_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE0_UNIT0: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE0_UNIT0_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE0_UNIT0_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE0_UNIT0_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE0_UNIT0_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT0_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT0_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT0: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE0_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE0_UNIT0_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE0_UNIT0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE0_UNIT0** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site0_unit0(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE0_UNIT0_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE0_UNIT0** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site0_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE0_UNIT0_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE0_UNIT0_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE0_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site0_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT0_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT0_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE0_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site0_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT0_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT0_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT0_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT0** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site0_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT0_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE0_UNIT0** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site0_unit0(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE0_UNIT0_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE0_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site0_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE0_UNIT0_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE0_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site0_unit0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE0_UNIT0_MASK);
    }
}

/// COMB_PD_SITE0_UNIT0_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit0vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit0vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit0Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite0Unit0Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE0_UNIT0** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site0_unit0(&self) -> bool {
        (self.bits & CombPdSite0Unit0Vt2Conf1::TIMING_ERR_VT2_PD_SITE0_UNIT0_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE0_UNIT0** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site0_unit0(&self) -> u32 {
        (self.bits & CombPdSite0Unit0Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE0_UNIT0_MASK)
            >> CombPdSite0Unit0Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE0_UNIT0_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE0_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site0_unit0(&self) -> u32 {
        (self.bits & CombPdSite0Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT0_MASK)
            >> CombPdSite0Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT0_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE0_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site0_unit0(&self) -> bool {
        (self.bits & CombPdSite0Unit0Vt2Conf1::MONITOR_EN_VT2_PD_SITE0_UNIT0_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE0_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site0_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT0_MASK)
                | ((value << CombPdSite0Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT0_OFFSET)
                    & CombPdSite0Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT0_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT0** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site0_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit0Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT0_MASK
            } else {
                self.bits & !CombPdSite0Unit0Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT0_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE0_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site0_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit0Vt2Conf1::MONITOR_EN_VT2_PD_SITE0_UNIT0_MASK
            } else {
                self.bits & !CombPdSite0Unit0Vt2Conf1::MONITOR_EN_VT2_PD_SITE0_UNIT0_MASK
            },
        }
    }
}

impl Default for CombPdSite0Unit0Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite0Unit0Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit0Vt2Conf1Val {
        CombPdSite0Unit0Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit0Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit0Vt2Conf1Val) -> CombPdSite0Unit0Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT1_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x008c
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE0_UNIT1** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE0_UNIT1** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE0_UNIT1** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT1** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE0_UNIT1** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit1Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit1Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit1Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE0_UNIT1_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE0_UNIT1_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE0_UNIT1: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE0_UNIT1_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE0_UNIT1_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE0_UNIT1_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE0_UNIT1_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT1_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT1_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT1: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE0_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE0_UNIT1_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE0_UNIT1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE0_UNIT1** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site0_unit1(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE0_UNIT1_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE0_UNIT1** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site0_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE0_UNIT1_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE0_UNIT1_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE0_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site0_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT1_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT1_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE0_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site0_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT1_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT1_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT1_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT1** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site0_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT1_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE0_UNIT1** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site0_unit1(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE0_UNIT1_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE0_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site0_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE0_UNIT1_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE0_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site0_unit1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE0_UNIT1_MASK);
    }
}

/// COMB_PD_SITE0_UNIT1_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit1vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit1vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit1Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite0Unit1Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE0_UNIT1** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site0_unit1(&self) -> bool {
        (self.bits & CombPdSite0Unit1Vt2Conf1::TIMING_ERR_VT2_PD_SITE0_UNIT1_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE0_UNIT1** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site0_unit1(&self) -> u32 {
        (self.bits & CombPdSite0Unit1Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE0_UNIT1_MASK)
            >> CombPdSite0Unit1Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE0_UNIT1_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE0_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site0_unit1(&self) -> u32 {
        (self.bits & CombPdSite0Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT1_MASK)
            >> CombPdSite0Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT1_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE0_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site0_unit1(&self) -> bool {
        (self.bits & CombPdSite0Unit1Vt2Conf1::MONITOR_EN_VT2_PD_SITE0_UNIT1_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE0_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site0_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT1_MASK)
                | ((value << CombPdSite0Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT1_OFFSET)
                    & CombPdSite0Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT1_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT1** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site0_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit1Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT1_MASK
            } else {
                self.bits & !CombPdSite0Unit1Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT1_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE0_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site0_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit1Vt2Conf1::MONITOR_EN_VT2_PD_SITE0_UNIT1_MASK
            } else {
                self.bits & !CombPdSite0Unit1Vt2Conf1::MONITOR_EN_VT2_PD_SITE0_UNIT1_MASK
            },
        }
    }
}

impl Default for CombPdSite0Unit1Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite0Unit1Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit1Vt2Conf1Val {
        CombPdSite0Unit1Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit1Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit1Vt2Conf1Val) -> CombPdSite0Unit1Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT2_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0090
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE0_UNIT2** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE0_UNIT2** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE0_UNIT2** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT2** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE0_UNIT2** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit2Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit2Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit2Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE0_UNIT2_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE0_UNIT2_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE0_UNIT2: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE0_UNIT2_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE0_UNIT2_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE0_UNIT2_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE0_UNIT2_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT2_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT2_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT2: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE0_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE0_UNIT2_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE0_UNIT2: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE0_UNIT2** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site0_unit2(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE0_UNIT2_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE0_UNIT2** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site0_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE0_UNIT2_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE0_UNIT2_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE0_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site0_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT2_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT2_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE0_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site0_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT2_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT2_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT2_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT2** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site0_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT2_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE0_UNIT2** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site0_unit2(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE0_UNIT2_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE0_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site0_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE0_UNIT2_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE0_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site0_unit2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE0_UNIT2_MASK);
    }
}

/// COMB_PD_SITE0_UNIT2_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit2vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit2vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit2Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite0Unit2Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE0_UNIT2** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site0_unit2(&self) -> bool {
        (self.bits & CombPdSite0Unit2Vt2Conf1::TIMING_ERR_VT2_PD_SITE0_UNIT2_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE0_UNIT2** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site0_unit2(&self) -> u32 {
        (self.bits & CombPdSite0Unit2Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE0_UNIT2_MASK)
            >> CombPdSite0Unit2Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE0_UNIT2_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE0_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site0_unit2(&self) -> u32 {
        (self.bits & CombPdSite0Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT2_MASK)
            >> CombPdSite0Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT2_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE0_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site0_unit2(&self) -> bool {
        (self.bits & CombPdSite0Unit2Vt2Conf1::MONITOR_EN_VT2_PD_SITE0_UNIT2_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE0_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site0_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT2_MASK)
                | ((value << CombPdSite0Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT2_OFFSET)
                    & CombPdSite0Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT2_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT2** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site0_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit2Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT2_MASK
            } else {
                self.bits & !CombPdSite0Unit2Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT2_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE0_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site0_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit2Vt2Conf1::MONITOR_EN_VT2_PD_SITE0_UNIT2_MASK
            } else {
                self.bits & !CombPdSite0Unit2Vt2Conf1::MONITOR_EN_VT2_PD_SITE0_UNIT2_MASK
            },
        }
    }
}

impl Default for CombPdSite0Unit2Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite0Unit2Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit2Vt2Conf1Val {
        CombPdSite0Unit2Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit2Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit2Vt2Conf1Val) -> CombPdSite0Unit2Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT3_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0094
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE0_UNIT3** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE0_UNIT3** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE0_UNIT3** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT3** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE0_UNIT3** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit3Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit3Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit3Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE0_UNIT3_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE0_UNIT3_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE0_UNIT3: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE0_UNIT3_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE0_UNIT3_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE0_UNIT3_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE0_UNIT3_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT3_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT3_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT3: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE0_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE0_UNIT3_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE0_UNIT3: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE0_UNIT3** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site0_unit3(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE0_UNIT3_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE0_UNIT3** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site0_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE0_UNIT3_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE0_UNIT3_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE0_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site0_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT3_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT3_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE0_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site0_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT3_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT3_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE0_UNIT3_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT3** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site0_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT3_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE0_UNIT3** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site0_unit3(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE0_UNIT3_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE0_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site0_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE0_UNIT3_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE0_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site0_unit3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE0_UNIT3_MASK);
    }
}

/// COMB_PD_SITE0_UNIT3_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit3vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit3vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit3Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite0Unit3Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE0_UNIT3** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site0_unit3(&self) -> bool {
        (self.bits & CombPdSite0Unit3Vt2Conf1::TIMING_ERR_VT2_PD_SITE0_UNIT3_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE0_UNIT3** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site0_unit3(&self) -> u32 {
        (self.bits & CombPdSite0Unit3Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE0_UNIT3_MASK)
            >> CombPdSite0Unit3Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE0_UNIT3_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE0_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site0_unit3(&self) -> u32 {
        (self.bits & CombPdSite0Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT3_MASK)
            >> CombPdSite0Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT3_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE0_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site0_unit3(&self) -> bool {
        (self.bits & CombPdSite0Unit3Vt2Conf1::MONITOR_EN_VT2_PD_SITE0_UNIT3_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE0_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site0_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT3_MASK)
                | ((value << CombPdSite0Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT3_OFFSET)
                    & CombPdSite0Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE0_UNIT3_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT3** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site0_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit3Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT3_MASK
            } else {
                self.bits & !CombPdSite0Unit3Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE0_UNIT3_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE0_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site0_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite0Unit3Vt2Conf1::MONITOR_EN_VT2_PD_SITE0_UNIT3_MASK
            } else {
                self.bits & !CombPdSite0Unit3Vt2Conf1::MONITOR_EN_VT2_PD_SITE0_UNIT3_MASK
            },
        }
    }
}

impl Default for CombPdSite0Unit3Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite0Unit3Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit3Vt2Conf1Val {
        CombPdSite0Unit3Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit3Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit3Vt2Conf1Val) -> CombPdSite0Unit3Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT0_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0098
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE1_UNIT0** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE1_UNIT0** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE1_UNIT0** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT0** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE1_UNIT0** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit0Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit0Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit0Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE1_UNIT0_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE1_UNIT0_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE1_UNIT0: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE1_UNIT0_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE1_UNIT0_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE1_UNIT0_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE1_UNIT0_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT0_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT0_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT0: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE1_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE1_UNIT0_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE1_UNIT0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE1_UNIT0** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site1_unit0(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE1_UNIT0_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE1_UNIT0** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site1_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE1_UNIT0_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE1_UNIT0_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE1_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site1_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT0_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT0_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE1_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site1_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT0_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT0_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT0_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT0** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site1_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT0_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE1_UNIT0** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site1_unit0(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE1_UNIT0_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE1_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site1_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE1_UNIT0_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE1_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site1_unit0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE1_UNIT0_MASK);
    }
}

/// COMB_PD_SITE1_UNIT0_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit0vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit0vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit0Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite1Unit0Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE1_UNIT0** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site1_unit0(&self) -> bool {
        (self.bits & CombPdSite1Unit0Vt0Conf1::TIMING_ERR_VT0_PD_SITE1_UNIT0_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE1_UNIT0** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site1_unit0(&self) -> u32 {
        (self.bits & CombPdSite1Unit0Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE1_UNIT0_MASK)
            >> CombPdSite1Unit0Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE1_UNIT0_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE1_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site1_unit0(&self) -> u32 {
        (self.bits & CombPdSite1Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT0_MASK)
            >> CombPdSite1Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT0_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE1_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site1_unit0(&self) -> bool {
        (self.bits & CombPdSite1Unit0Vt0Conf1::MONITOR_EN_VT0_PD_SITE1_UNIT0_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE1_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site1_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT0_MASK)
                | ((value << CombPdSite1Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT0_OFFSET)
                    & CombPdSite1Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT0_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT0** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site1_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit0Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT0_MASK
            } else {
                self.bits & !CombPdSite1Unit0Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT0_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE1_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site1_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit0Vt0Conf1::MONITOR_EN_VT0_PD_SITE1_UNIT0_MASK
            } else {
                self.bits & !CombPdSite1Unit0Vt0Conf1::MONITOR_EN_VT0_PD_SITE1_UNIT0_MASK
            },
        }
    }
}

impl Default for CombPdSite1Unit0Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite1Unit0Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit0Vt0Conf1Val {
        CombPdSite1Unit0Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit0Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit0Vt0Conf1Val) -> CombPdSite1Unit0Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT1_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x009c
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE1_UNIT1** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE1_UNIT1** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE1_UNIT1** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT1** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE1_UNIT1** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit1Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit1Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit1Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE1_UNIT1_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE1_UNIT1_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE1_UNIT1: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE1_UNIT1_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE1_UNIT1_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE1_UNIT1_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE1_UNIT1_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT1_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT1_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT1: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE1_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE1_UNIT1_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE1_UNIT1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE1_UNIT1** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site1_unit1(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE1_UNIT1_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE1_UNIT1** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site1_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE1_UNIT1_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE1_UNIT1_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE1_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site1_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT1_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT1_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE1_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site1_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT1_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT1_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT1_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT1** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site1_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT1_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE1_UNIT1** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site1_unit1(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE1_UNIT1_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE1_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site1_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE1_UNIT1_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE1_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site1_unit1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE1_UNIT1_MASK);
    }
}

/// COMB_PD_SITE1_UNIT1_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit1vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit1vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit1Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite1Unit1Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE1_UNIT1** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site1_unit1(&self) -> bool {
        (self.bits & CombPdSite1Unit1Vt0Conf1::TIMING_ERR_VT0_PD_SITE1_UNIT1_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE1_UNIT1** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site1_unit1(&self) -> u32 {
        (self.bits & CombPdSite1Unit1Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE1_UNIT1_MASK)
            >> CombPdSite1Unit1Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE1_UNIT1_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE1_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site1_unit1(&self) -> u32 {
        (self.bits & CombPdSite1Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT1_MASK)
            >> CombPdSite1Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT1_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE1_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site1_unit1(&self) -> bool {
        (self.bits & CombPdSite1Unit1Vt0Conf1::MONITOR_EN_VT0_PD_SITE1_UNIT1_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE1_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site1_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT1_MASK)
                | ((value << CombPdSite1Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT1_OFFSET)
                    & CombPdSite1Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT1_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT1** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site1_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit1Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT1_MASK
            } else {
                self.bits & !CombPdSite1Unit1Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT1_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE1_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site1_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit1Vt0Conf1::MONITOR_EN_VT0_PD_SITE1_UNIT1_MASK
            } else {
                self.bits & !CombPdSite1Unit1Vt0Conf1::MONITOR_EN_VT0_PD_SITE1_UNIT1_MASK
            },
        }
    }
}

impl Default for CombPdSite1Unit1Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite1Unit1Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit1Vt0Conf1Val {
        CombPdSite1Unit1Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit1Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit1Vt0Conf1Val) -> CombPdSite1Unit1Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT2_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00a0
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE1_UNIT2** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE1_UNIT2** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE1_UNIT2** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT2** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE1_UNIT2** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit2Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit2Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit2Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE1_UNIT2_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE1_UNIT2_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE1_UNIT2: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE1_UNIT2_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE1_UNIT2_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE1_UNIT2_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE1_UNIT2_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT2_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT2_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT2: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE1_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE1_UNIT2_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE1_UNIT2: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE1_UNIT2** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site1_unit2(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE1_UNIT2_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE1_UNIT2** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site1_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE1_UNIT2_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE1_UNIT2_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE1_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site1_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT2_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT2_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE1_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site1_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT2_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT2_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT2_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT2** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site1_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT2_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE1_UNIT2** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site1_unit2(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE1_UNIT2_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE1_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site1_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE1_UNIT2_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE1_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site1_unit2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE1_UNIT2_MASK);
    }
}

/// COMB_PD_SITE1_UNIT2_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit2vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit2vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit2Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite1Unit2Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE1_UNIT2** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site1_unit2(&self) -> bool {
        (self.bits & CombPdSite1Unit2Vt0Conf1::TIMING_ERR_VT0_PD_SITE1_UNIT2_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE1_UNIT2** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site1_unit2(&self) -> u32 {
        (self.bits & CombPdSite1Unit2Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE1_UNIT2_MASK)
            >> CombPdSite1Unit2Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE1_UNIT2_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE1_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site1_unit2(&self) -> u32 {
        (self.bits & CombPdSite1Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT2_MASK)
            >> CombPdSite1Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT2_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE1_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site1_unit2(&self) -> bool {
        (self.bits & CombPdSite1Unit2Vt0Conf1::MONITOR_EN_VT0_PD_SITE1_UNIT2_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE1_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site1_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT2_MASK)
                | ((value << CombPdSite1Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT2_OFFSET)
                    & CombPdSite1Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT2_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT2** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site1_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit2Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT2_MASK
            } else {
                self.bits & !CombPdSite1Unit2Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT2_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE1_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site1_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit2Vt0Conf1::MONITOR_EN_VT0_PD_SITE1_UNIT2_MASK
            } else {
                self.bits & !CombPdSite1Unit2Vt0Conf1::MONITOR_EN_VT0_PD_SITE1_UNIT2_MASK
            },
        }
    }
}

impl Default for CombPdSite1Unit2Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite1Unit2Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit2Vt0Conf1Val {
        CombPdSite1Unit2Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit2Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit2Vt0Conf1Val) -> CombPdSite1Unit2Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT3_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00a4
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE1_UNIT3** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE1_UNIT3** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE1_UNIT3** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT3** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE1_UNIT3** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit3Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit3Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit3Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE1_UNIT3_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE1_UNIT3_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE1_UNIT3: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE1_UNIT3_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE1_UNIT3_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE1_UNIT3_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE1_UNIT3_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT3_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT3_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT3: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE1_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE1_UNIT3_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE1_UNIT3: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE1_UNIT3** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site1_unit3(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE1_UNIT3_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE1_UNIT3** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site1_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE1_UNIT3_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE1_UNIT3_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE1_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site1_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT3_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT3_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE1_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site1_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT3_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT3_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE1_UNIT3_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT3** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site1_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT3_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE1_UNIT3** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site1_unit3(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE1_UNIT3_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE1_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site1_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE1_UNIT3_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE1_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site1_unit3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE1_UNIT3_MASK);
    }
}

/// COMB_PD_SITE1_UNIT3_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit3vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit3vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit3Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite1Unit3Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE1_UNIT3** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site1_unit3(&self) -> bool {
        (self.bits & CombPdSite1Unit3Vt0Conf1::TIMING_ERR_VT0_PD_SITE1_UNIT3_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE1_UNIT3** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site1_unit3(&self) -> u32 {
        (self.bits & CombPdSite1Unit3Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE1_UNIT3_MASK)
            >> CombPdSite1Unit3Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE1_UNIT3_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE1_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site1_unit3(&self) -> u32 {
        (self.bits & CombPdSite1Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT3_MASK)
            >> CombPdSite1Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT3_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE1_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site1_unit3(&self) -> bool {
        (self.bits & CombPdSite1Unit3Vt0Conf1::MONITOR_EN_VT0_PD_SITE1_UNIT3_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE1_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site1_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT3_MASK)
                | ((value << CombPdSite1Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT3_OFFSET)
                    & CombPdSite1Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE1_UNIT3_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT3** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site1_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit3Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT3_MASK
            } else {
                self.bits & !CombPdSite1Unit3Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE1_UNIT3_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE1_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site1_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit3Vt0Conf1::MONITOR_EN_VT0_PD_SITE1_UNIT3_MASK
            } else {
                self.bits & !CombPdSite1Unit3Vt0Conf1::MONITOR_EN_VT0_PD_SITE1_UNIT3_MASK
            },
        }
    }
}

impl Default for CombPdSite1Unit3Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite1Unit3Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit3Vt0Conf1Val {
        CombPdSite1Unit3Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit3Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit3Vt0Conf1Val) -> CombPdSite1Unit3Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT0_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00a8
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE1_UNIT0** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE1_UNIT0** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE1_UNIT0** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT0** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE1_UNIT0** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit0Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit0Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit0Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE1_UNIT0_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE1_UNIT0_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE1_UNIT0: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE1_UNIT0_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE1_UNIT0_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE1_UNIT0_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE1_UNIT0_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT0_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT0_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT0: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE1_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE1_UNIT0_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE1_UNIT0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE1_UNIT0** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site1_unit0(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE1_UNIT0_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE1_UNIT0** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site1_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE1_UNIT0_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE1_UNIT0_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE1_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site1_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT0_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT0_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE1_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site1_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT0_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT0_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT0_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT0** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site1_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT0_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE1_UNIT0** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site1_unit0(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE1_UNIT0_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE1_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site1_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE1_UNIT0_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE1_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site1_unit0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE1_UNIT0_MASK);
    }
}

/// COMB_PD_SITE1_UNIT0_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit0vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit0vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit0Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite1Unit0Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE1_UNIT0** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site1_unit0(&self) -> bool {
        (self.bits & CombPdSite1Unit0Vt1Conf1::TIMING_ERR_VT1_PD_SITE1_UNIT0_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE1_UNIT0** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site1_unit0(&self) -> u32 {
        (self.bits & CombPdSite1Unit0Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE1_UNIT0_MASK)
            >> CombPdSite1Unit0Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE1_UNIT0_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE1_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site1_unit0(&self) -> u32 {
        (self.bits & CombPdSite1Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT0_MASK)
            >> CombPdSite1Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT0_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE1_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site1_unit0(&self) -> bool {
        (self.bits & CombPdSite1Unit0Vt1Conf1::MONITOR_EN_VT1_PD_SITE1_UNIT0_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE1_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site1_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT0_MASK)
                | ((value << CombPdSite1Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT0_OFFSET)
                    & CombPdSite1Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT0_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT0** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site1_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit0Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT0_MASK
            } else {
                self.bits & !CombPdSite1Unit0Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT0_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE1_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site1_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit0Vt1Conf1::MONITOR_EN_VT1_PD_SITE1_UNIT0_MASK
            } else {
                self.bits & !CombPdSite1Unit0Vt1Conf1::MONITOR_EN_VT1_PD_SITE1_UNIT0_MASK
            },
        }
    }
}

impl Default for CombPdSite1Unit0Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite1Unit0Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit0Vt1Conf1Val {
        CombPdSite1Unit0Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit0Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit0Vt1Conf1Val) -> CombPdSite1Unit0Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT1_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00ac
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE1_UNIT1** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE1_UNIT1** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE1_UNIT1** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT1** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE1_UNIT1** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit1Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit1Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit1Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE1_UNIT1_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE1_UNIT1_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE1_UNIT1: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE1_UNIT1_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE1_UNIT1_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE1_UNIT1_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE1_UNIT1_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT1_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT1_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT1: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE1_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE1_UNIT1_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE1_UNIT1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE1_UNIT1** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site1_unit1(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE1_UNIT1_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE1_UNIT1** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site1_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE1_UNIT1_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE1_UNIT1_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE1_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site1_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT1_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT1_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE1_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site1_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT1_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT1_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT1_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT1** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site1_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT1_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE1_UNIT1** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site1_unit1(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE1_UNIT1_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE1_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site1_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE1_UNIT1_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE1_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site1_unit1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE1_UNIT1_MASK);
    }
}

/// COMB_PD_SITE1_UNIT1_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit1vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit1vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit1Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite1Unit1Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE1_UNIT1** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site1_unit1(&self) -> bool {
        (self.bits & CombPdSite1Unit1Vt1Conf1::TIMING_ERR_VT1_PD_SITE1_UNIT1_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE1_UNIT1** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site1_unit1(&self) -> u32 {
        (self.bits & CombPdSite1Unit1Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE1_UNIT1_MASK)
            >> CombPdSite1Unit1Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE1_UNIT1_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE1_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site1_unit1(&self) -> u32 {
        (self.bits & CombPdSite1Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT1_MASK)
            >> CombPdSite1Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT1_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE1_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site1_unit1(&self) -> bool {
        (self.bits & CombPdSite1Unit1Vt1Conf1::MONITOR_EN_VT1_PD_SITE1_UNIT1_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE1_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site1_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT1_MASK)
                | ((value << CombPdSite1Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT1_OFFSET)
                    & CombPdSite1Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT1_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT1** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site1_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit1Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT1_MASK
            } else {
                self.bits & !CombPdSite1Unit1Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT1_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE1_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site1_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit1Vt1Conf1::MONITOR_EN_VT1_PD_SITE1_UNIT1_MASK
            } else {
                self.bits & !CombPdSite1Unit1Vt1Conf1::MONITOR_EN_VT1_PD_SITE1_UNIT1_MASK
            },
        }
    }
}

impl Default for CombPdSite1Unit1Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite1Unit1Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit1Vt1Conf1Val {
        CombPdSite1Unit1Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit1Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit1Vt1Conf1Val) -> CombPdSite1Unit1Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT2_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00b0
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE1_UNIT2** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE1_UNIT2** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE1_UNIT2** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT2** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE1_UNIT2** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit2Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit2Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit2Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE1_UNIT2_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE1_UNIT2_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE1_UNIT2: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE1_UNIT2_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE1_UNIT2_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE1_UNIT2_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE1_UNIT2_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT2_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT2_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT2: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE1_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE1_UNIT2_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE1_UNIT2: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE1_UNIT2** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site1_unit2(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE1_UNIT2_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE1_UNIT2** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site1_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE1_UNIT2_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE1_UNIT2_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE1_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site1_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT2_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT2_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE1_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site1_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT2_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT2_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT2_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT2** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site1_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT2_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE1_UNIT2** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site1_unit2(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE1_UNIT2_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE1_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site1_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE1_UNIT2_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE1_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site1_unit2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE1_UNIT2_MASK);
    }
}

/// COMB_PD_SITE1_UNIT2_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit2vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit2vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit2Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite1Unit2Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE1_UNIT2** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site1_unit2(&self) -> bool {
        (self.bits & CombPdSite1Unit2Vt1Conf1::TIMING_ERR_VT1_PD_SITE1_UNIT2_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE1_UNIT2** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site1_unit2(&self) -> u32 {
        (self.bits & CombPdSite1Unit2Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE1_UNIT2_MASK)
            >> CombPdSite1Unit2Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE1_UNIT2_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE1_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site1_unit2(&self) -> u32 {
        (self.bits & CombPdSite1Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT2_MASK)
            >> CombPdSite1Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT2_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE1_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site1_unit2(&self) -> bool {
        (self.bits & CombPdSite1Unit2Vt1Conf1::MONITOR_EN_VT1_PD_SITE1_UNIT2_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE1_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site1_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT2_MASK)
                | ((value << CombPdSite1Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT2_OFFSET)
                    & CombPdSite1Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT2_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT2** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site1_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit2Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT2_MASK
            } else {
                self.bits & !CombPdSite1Unit2Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT2_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE1_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site1_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit2Vt1Conf1::MONITOR_EN_VT1_PD_SITE1_UNIT2_MASK
            } else {
                self.bits & !CombPdSite1Unit2Vt1Conf1::MONITOR_EN_VT1_PD_SITE1_UNIT2_MASK
            },
        }
    }
}

impl Default for CombPdSite1Unit2Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite1Unit2Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit2Vt1Conf1Val {
        CombPdSite1Unit2Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit2Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit2Vt1Conf1Val) -> CombPdSite1Unit2Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT3_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00b4
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE1_UNIT3** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE1_UNIT3** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE1_UNIT3** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT3** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE1_UNIT3** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit3Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit3Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit3Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE1_UNIT3_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE1_UNIT3_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE1_UNIT3: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE1_UNIT3_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE1_UNIT3_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE1_UNIT3_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE1_UNIT3_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT3_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT3_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT3: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE1_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE1_UNIT3_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE1_UNIT3: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE1_UNIT3** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site1_unit3(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE1_UNIT3_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE1_UNIT3** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site1_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE1_UNIT3_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE1_UNIT3_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE1_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site1_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT3_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT3_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE1_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site1_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT3_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT3_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE1_UNIT3_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT3** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site1_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT3_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE1_UNIT3** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site1_unit3(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE1_UNIT3_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE1_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site1_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE1_UNIT3_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE1_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site1_unit3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE1_UNIT3_MASK);
    }
}

/// COMB_PD_SITE1_UNIT3_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit3vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit3vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit3Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite1Unit3Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE1_UNIT3** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site1_unit3(&self) -> bool {
        (self.bits & CombPdSite1Unit3Vt1Conf1::TIMING_ERR_VT1_PD_SITE1_UNIT3_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE1_UNIT3** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site1_unit3(&self) -> u32 {
        (self.bits & CombPdSite1Unit3Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE1_UNIT3_MASK)
            >> CombPdSite1Unit3Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE1_UNIT3_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE1_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site1_unit3(&self) -> u32 {
        (self.bits & CombPdSite1Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT3_MASK)
            >> CombPdSite1Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT3_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE1_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site1_unit3(&self) -> bool {
        (self.bits & CombPdSite1Unit3Vt1Conf1::MONITOR_EN_VT1_PD_SITE1_UNIT3_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE1_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site1_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT3_MASK)
                | ((value << CombPdSite1Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT3_OFFSET)
                    & CombPdSite1Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE1_UNIT3_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT3** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site1_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit3Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT3_MASK
            } else {
                self.bits & !CombPdSite1Unit3Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE1_UNIT3_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE1_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site1_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit3Vt1Conf1::MONITOR_EN_VT1_PD_SITE1_UNIT3_MASK
            } else {
                self.bits & !CombPdSite1Unit3Vt1Conf1::MONITOR_EN_VT1_PD_SITE1_UNIT3_MASK
            },
        }
    }
}

impl Default for CombPdSite1Unit3Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite1Unit3Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit3Vt1Conf1Val {
        CombPdSite1Unit3Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit3Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit3Vt1Conf1Val) -> CombPdSite1Unit3Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT0_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00b8
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE1_UNIT0** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE1_UNIT0** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE1_UNIT0** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT0** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE1_UNIT0** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit0Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit0Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit0Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE1_UNIT0_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE1_UNIT0_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE1_UNIT0: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE1_UNIT0_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE1_UNIT0_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE1_UNIT0_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE1_UNIT0_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT0_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT0_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT0: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE1_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE1_UNIT0_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE1_UNIT0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE1_UNIT0** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site1_unit0(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE1_UNIT0_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE1_UNIT0** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site1_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE1_UNIT0_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE1_UNIT0_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE1_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site1_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT0_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT0_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE1_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site1_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT0_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT0_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT0_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT0** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site1_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT0_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE1_UNIT0** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site1_unit0(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE1_UNIT0_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE1_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site1_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE1_UNIT0_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE1_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site1_unit0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE1_UNIT0_MASK);
    }
}

/// COMB_PD_SITE1_UNIT0_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit0vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit0vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit0Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite1Unit0Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE1_UNIT0** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site1_unit0(&self) -> bool {
        (self.bits & CombPdSite1Unit0Vt2Conf1::TIMING_ERR_VT2_PD_SITE1_UNIT0_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE1_UNIT0** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site1_unit0(&self) -> u32 {
        (self.bits & CombPdSite1Unit0Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE1_UNIT0_MASK)
            >> CombPdSite1Unit0Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE1_UNIT0_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE1_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site1_unit0(&self) -> u32 {
        (self.bits & CombPdSite1Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT0_MASK)
            >> CombPdSite1Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT0_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE1_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site1_unit0(&self) -> bool {
        (self.bits & CombPdSite1Unit0Vt2Conf1::MONITOR_EN_VT2_PD_SITE1_UNIT0_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE1_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site1_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT0_MASK)
                | ((value << CombPdSite1Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT0_OFFSET)
                    & CombPdSite1Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT0_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT0** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site1_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit0Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT0_MASK
            } else {
                self.bits & !CombPdSite1Unit0Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT0_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE1_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site1_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit0Vt2Conf1::MONITOR_EN_VT2_PD_SITE1_UNIT0_MASK
            } else {
                self.bits & !CombPdSite1Unit0Vt2Conf1::MONITOR_EN_VT2_PD_SITE1_UNIT0_MASK
            },
        }
    }
}

impl Default for CombPdSite1Unit0Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite1Unit0Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit0Vt2Conf1Val {
        CombPdSite1Unit0Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit0Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit0Vt2Conf1Val) -> CombPdSite1Unit0Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT1_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00bc
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE1_UNIT1** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE1_UNIT1** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE1_UNIT1** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT1** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE1_UNIT1** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit1Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit1Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit1Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE1_UNIT1_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE1_UNIT1_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE1_UNIT1: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE1_UNIT1_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE1_UNIT1_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE1_UNIT1_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE1_UNIT1_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT1_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT1_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT1: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE1_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE1_UNIT1_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE1_UNIT1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE1_UNIT1** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site1_unit1(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE1_UNIT1_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE1_UNIT1** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site1_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE1_UNIT1_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE1_UNIT1_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE1_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site1_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT1_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT1_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE1_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site1_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT1_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT1_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT1_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT1** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site1_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT1_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE1_UNIT1** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site1_unit1(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE1_UNIT1_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE1_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site1_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE1_UNIT1_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE1_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site1_unit1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE1_UNIT1_MASK);
    }
}

/// COMB_PD_SITE1_UNIT1_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit1vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit1vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit1Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite1Unit1Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE1_UNIT1** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site1_unit1(&self) -> bool {
        (self.bits & CombPdSite1Unit1Vt2Conf1::TIMING_ERR_VT2_PD_SITE1_UNIT1_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE1_UNIT1** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site1_unit1(&self) -> u32 {
        (self.bits & CombPdSite1Unit1Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE1_UNIT1_MASK)
            >> CombPdSite1Unit1Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE1_UNIT1_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE1_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site1_unit1(&self) -> u32 {
        (self.bits & CombPdSite1Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT1_MASK)
            >> CombPdSite1Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT1_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE1_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site1_unit1(&self) -> bool {
        (self.bits & CombPdSite1Unit1Vt2Conf1::MONITOR_EN_VT2_PD_SITE1_UNIT1_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE1_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site1_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT1_MASK)
                | ((value << CombPdSite1Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT1_OFFSET)
                    & CombPdSite1Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT1_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT1** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site1_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit1Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT1_MASK
            } else {
                self.bits & !CombPdSite1Unit1Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT1_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE1_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site1_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit1Vt2Conf1::MONITOR_EN_VT2_PD_SITE1_UNIT1_MASK
            } else {
                self.bits & !CombPdSite1Unit1Vt2Conf1::MONITOR_EN_VT2_PD_SITE1_UNIT1_MASK
            },
        }
    }
}

impl Default for CombPdSite1Unit1Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite1Unit1Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit1Vt2Conf1Val {
        CombPdSite1Unit1Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit1Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit1Vt2Conf1Val) -> CombPdSite1Unit1Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT2_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00c0
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE1_UNIT2** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE1_UNIT2** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE1_UNIT2** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT2** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE1_UNIT2** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit2Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit2Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit2Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE1_UNIT2_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE1_UNIT2_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE1_UNIT2: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE1_UNIT2_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE1_UNIT2_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE1_UNIT2_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE1_UNIT2_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT2_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT2_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT2: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE1_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE1_UNIT2_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE1_UNIT2: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE1_UNIT2** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site1_unit2(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE1_UNIT2_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE1_UNIT2** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site1_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE1_UNIT2_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE1_UNIT2_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE1_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site1_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT2_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT2_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE1_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site1_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT2_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT2_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT2_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT2** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site1_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT2_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE1_UNIT2** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site1_unit2(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE1_UNIT2_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE1_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site1_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE1_UNIT2_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE1_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site1_unit2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE1_UNIT2_MASK);
    }
}

/// COMB_PD_SITE1_UNIT2_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit2vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit2vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit2Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite1Unit2Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE1_UNIT2** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site1_unit2(&self) -> bool {
        (self.bits & CombPdSite1Unit2Vt2Conf1::TIMING_ERR_VT2_PD_SITE1_UNIT2_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE1_UNIT2** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site1_unit2(&self) -> u32 {
        (self.bits & CombPdSite1Unit2Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE1_UNIT2_MASK)
            >> CombPdSite1Unit2Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE1_UNIT2_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE1_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site1_unit2(&self) -> u32 {
        (self.bits & CombPdSite1Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT2_MASK)
            >> CombPdSite1Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT2_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE1_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site1_unit2(&self) -> bool {
        (self.bits & CombPdSite1Unit2Vt2Conf1::MONITOR_EN_VT2_PD_SITE1_UNIT2_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE1_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site1_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT2_MASK)
                | ((value << CombPdSite1Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT2_OFFSET)
                    & CombPdSite1Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT2_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT2** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site1_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit2Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT2_MASK
            } else {
                self.bits & !CombPdSite1Unit2Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT2_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE1_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site1_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit2Vt2Conf1::MONITOR_EN_VT2_PD_SITE1_UNIT2_MASK
            } else {
                self.bits & !CombPdSite1Unit2Vt2Conf1::MONITOR_EN_VT2_PD_SITE1_UNIT2_MASK
            },
        }
    }
}

impl Default for CombPdSite1Unit2Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite1Unit2Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit2Vt2Conf1Val {
        CombPdSite1Unit2Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit2Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit2Vt2Conf1Val) -> CombPdSite1Unit2Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT3_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00c4
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE1_UNIT3** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE1_UNIT3** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE1_UNIT3** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT3** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE1_UNIT3** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit3Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit3Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit3Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE1_UNIT3_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE1_UNIT3_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE1_UNIT3: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE1_UNIT3_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE1_UNIT3_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE1_UNIT3_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE1_UNIT3_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT3_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT3_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT3: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE1_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE1_UNIT3_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE1_UNIT3: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE1_UNIT3** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site1_unit3(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE1_UNIT3_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE1_UNIT3** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site1_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE1_UNIT3_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE1_UNIT3_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE1_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site1_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT3_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT3_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE1_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site1_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT3_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT3_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE1_UNIT3_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT3** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site1_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT3_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE1_UNIT3** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site1_unit3(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE1_UNIT3_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE1_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site1_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE1_UNIT3_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE1_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site1_unit3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE1_UNIT3_MASK);
    }
}

/// COMB_PD_SITE1_UNIT3_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit3vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit3vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit3Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite1Unit3Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE1_UNIT3** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site1_unit3(&self) -> bool {
        (self.bits & CombPdSite1Unit3Vt2Conf1::TIMING_ERR_VT2_PD_SITE1_UNIT3_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE1_UNIT3** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site1_unit3(&self) -> u32 {
        (self.bits & CombPdSite1Unit3Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE1_UNIT3_MASK)
            >> CombPdSite1Unit3Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE1_UNIT3_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE1_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site1_unit3(&self) -> u32 {
        (self.bits & CombPdSite1Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT3_MASK)
            >> CombPdSite1Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT3_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE1_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site1_unit3(&self) -> bool {
        (self.bits & CombPdSite1Unit3Vt2Conf1::MONITOR_EN_VT2_PD_SITE1_UNIT3_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE1_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site1_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT3_MASK)
                | ((value << CombPdSite1Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT3_OFFSET)
                    & CombPdSite1Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE1_UNIT3_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT3** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site1_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit3Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT3_MASK
            } else {
                self.bits & !CombPdSite1Unit3Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE1_UNIT3_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE1_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site1_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite1Unit3Vt2Conf1::MONITOR_EN_VT2_PD_SITE1_UNIT3_MASK
            } else {
                self.bits & !CombPdSite1Unit3Vt2Conf1::MONITOR_EN_VT2_PD_SITE1_UNIT3_MASK
            },
        }
    }
}

impl Default for CombPdSite1Unit3Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite1Unit3Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit3Vt2Conf1Val {
        CombPdSite1Unit3Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit3Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit3Vt2Conf1Val) -> CombPdSite1Unit3Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT0_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00c8
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE2_UNIT0** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE2_UNIT0** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE2_UNIT0** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT0** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE2_UNIT0** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit0Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit0Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit0Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE2_UNIT0_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE2_UNIT0_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE2_UNIT0: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE2_UNIT0_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE2_UNIT0_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE2_UNIT0_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE2_UNIT0_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT0_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT0_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT0: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE2_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE2_UNIT0_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE2_UNIT0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE2_UNIT0** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site2_unit0(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE2_UNIT0_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE2_UNIT0** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site2_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE2_UNIT0_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE2_UNIT0_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE2_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site2_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT0_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT0_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE2_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site2_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT0_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT0_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT0_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT0** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site2_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT0_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE2_UNIT0** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site2_unit0(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE2_UNIT0_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE2_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site2_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE2_UNIT0_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE2_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site2_unit0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE2_UNIT0_MASK);
    }
}

/// COMB_PD_SITE2_UNIT0_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit0vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit0vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit0Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite2Unit0Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE2_UNIT0** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site2_unit0(&self) -> bool {
        (self.bits & CombPdSite2Unit0Vt0Conf1::TIMING_ERR_VT0_PD_SITE2_UNIT0_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE2_UNIT0** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site2_unit0(&self) -> u32 {
        (self.bits & CombPdSite2Unit0Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE2_UNIT0_MASK)
            >> CombPdSite2Unit0Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE2_UNIT0_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE2_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site2_unit0(&self) -> u32 {
        (self.bits & CombPdSite2Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT0_MASK)
            >> CombPdSite2Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT0_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE2_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site2_unit0(&self) -> bool {
        (self.bits & CombPdSite2Unit0Vt0Conf1::MONITOR_EN_VT0_PD_SITE2_UNIT0_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE2_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site2_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT0_MASK)
                | ((value << CombPdSite2Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT0_OFFSET)
                    & CombPdSite2Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT0_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT0** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site2_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit0Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT0_MASK
            } else {
                self.bits & !CombPdSite2Unit0Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT0_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE2_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site2_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit0Vt0Conf1::MONITOR_EN_VT0_PD_SITE2_UNIT0_MASK
            } else {
                self.bits & !CombPdSite2Unit0Vt0Conf1::MONITOR_EN_VT0_PD_SITE2_UNIT0_MASK
            },
        }
    }
}

impl Default for CombPdSite2Unit0Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite2Unit0Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit0Vt0Conf1Val {
        CombPdSite2Unit0Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit0Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit0Vt0Conf1Val) -> CombPdSite2Unit0Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT1_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00cc
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE2_UNIT1** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE2_UNIT1** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE2_UNIT1** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT1** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE2_UNIT1** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit1Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit1Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit1Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE2_UNIT1_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE2_UNIT1_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE2_UNIT1: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE2_UNIT1_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE2_UNIT1_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE2_UNIT1_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE2_UNIT1_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT1_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT1_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT1: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE2_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE2_UNIT1_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE2_UNIT1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE2_UNIT1** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site2_unit1(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE2_UNIT1_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE2_UNIT1** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site2_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE2_UNIT1_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE2_UNIT1_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE2_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site2_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT1_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT1_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE2_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site2_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT1_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT1_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT1_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT1** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site2_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT1_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE2_UNIT1** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site2_unit1(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE2_UNIT1_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE2_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site2_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE2_UNIT1_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE2_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site2_unit1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE2_UNIT1_MASK);
    }
}

/// COMB_PD_SITE2_UNIT1_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit1vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit1vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit1Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite2Unit1Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE2_UNIT1** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site2_unit1(&self) -> bool {
        (self.bits & CombPdSite2Unit1Vt0Conf1::TIMING_ERR_VT0_PD_SITE2_UNIT1_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE2_UNIT1** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site2_unit1(&self) -> u32 {
        (self.bits & CombPdSite2Unit1Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE2_UNIT1_MASK)
            >> CombPdSite2Unit1Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE2_UNIT1_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE2_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site2_unit1(&self) -> u32 {
        (self.bits & CombPdSite2Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT1_MASK)
            >> CombPdSite2Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT1_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE2_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site2_unit1(&self) -> bool {
        (self.bits & CombPdSite2Unit1Vt0Conf1::MONITOR_EN_VT0_PD_SITE2_UNIT1_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE2_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site2_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT1_MASK)
                | ((value << CombPdSite2Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT1_OFFSET)
                    & CombPdSite2Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT1_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT1** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site2_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit1Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT1_MASK
            } else {
                self.bits & !CombPdSite2Unit1Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT1_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE2_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site2_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit1Vt0Conf1::MONITOR_EN_VT0_PD_SITE2_UNIT1_MASK
            } else {
                self.bits & !CombPdSite2Unit1Vt0Conf1::MONITOR_EN_VT0_PD_SITE2_UNIT1_MASK
            },
        }
    }
}

impl Default for CombPdSite2Unit1Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite2Unit1Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit1Vt0Conf1Val {
        CombPdSite2Unit1Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit1Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit1Vt0Conf1Val) -> CombPdSite2Unit1Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT2_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00d0
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE2_UNIT2** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE2_UNIT2** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE2_UNIT2** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT2** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE2_UNIT2** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit2Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit2Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit2Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE2_UNIT2_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE2_UNIT2_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE2_UNIT2: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE2_UNIT2_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE2_UNIT2_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE2_UNIT2_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE2_UNIT2_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT2_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT2_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT2: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE2_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE2_UNIT2_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE2_UNIT2: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE2_UNIT2** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site2_unit2(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE2_UNIT2_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE2_UNIT2** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site2_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE2_UNIT2_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE2_UNIT2_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE2_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site2_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT2_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT2_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE2_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site2_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT2_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT2_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT2_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT2** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site2_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT2_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE2_UNIT2** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site2_unit2(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE2_UNIT2_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE2_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site2_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE2_UNIT2_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE2_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site2_unit2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE2_UNIT2_MASK);
    }
}

/// COMB_PD_SITE2_UNIT2_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit2vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit2vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit2Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite2Unit2Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE2_UNIT2** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site2_unit2(&self) -> bool {
        (self.bits & CombPdSite2Unit2Vt0Conf1::TIMING_ERR_VT0_PD_SITE2_UNIT2_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE2_UNIT2** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site2_unit2(&self) -> u32 {
        (self.bits & CombPdSite2Unit2Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE2_UNIT2_MASK)
            >> CombPdSite2Unit2Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE2_UNIT2_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE2_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site2_unit2(&self) -> u32 {
        (self.bits & CombPdSite2Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT2_MASK)
            >> CombPdSite2Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT2_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE2_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site2_unit2(&self) -> bool {
        (self.bits & CombPdSite2Unit2Vt0Conf1::MONITOR_EN_VT0_PD_SITE2_UNIT2_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE2_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site2_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT2_MASK)
                | ((value << CombPdSite2Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT2_OFFSET)
                    & CombPdSite2Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT2_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT2** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site2_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit2Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT2_MASK
            } else {
                self.bits & !CombPdSite2Unit2Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT2_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE2_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site2_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit2Vt0Conf1::MONITOR_EN_VT0_PD_SITE2_UNIT2_MASK
            } else {
                self.bits & !CombPdSite2Unit2Vt0Conf1::MONITOR_EN_VT0_PD_SITE2_UNIT2_MASK
            },
        }
    }
}

impl Default for CombPdSite2Unit2Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite2Unit2Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit2Vt0Conf1Val {
        CombPdSite2Unit2Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit2Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit2Vt0Conf1Val) -> CombPdSite2Unit2Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT3_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00d4
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE2_UNIT3** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE2_UNIT3** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE2_UNIT3** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT3** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE2_UNIT3** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit3Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit3Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit3Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE2_UNIT3_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE2_UNIT3_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE2_UNIT3: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE2_UNIT3_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE2_UNIT3_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE2_UNIT3_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE2_UNIT3_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT3_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT3_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT3: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE2_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE2_UNIT3_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE2_UNIT3: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE2_UNIT3** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site2_unit3(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE2_UNIT3_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE2_UNIT3** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site2_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE2_UNIT3_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE2_UNIT3_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE2_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site2_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT3_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT3_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE2_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site2_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT3_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT3_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE2_UNIT3_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT3** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site2_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT3_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE2_UNIT3** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site2_unit3(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE2_UNIT3_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE2_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site2_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE2_UNIT3_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE2_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site2_unit3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE2_UNIT3_MASK);
    }
}

/// COMB_PD_SITE2_UNIT3_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit3vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit3vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit3Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite2Unit3Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE2_UNIT3** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site2_unit3(&self) -> bool {
        (self.bits & CombPdSite2Unit3Vt0Conf1::TIMING_ERR_VT0_PD_SITE2_UNIT3_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE2_UNIT3** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site2_unit3(&self) -> u32 {
        (self.bits & CombPdSite2Unit3Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE2_UNIT3_MASK)
            >> CombPdSite2Unit3Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE2_UNIT3_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE2_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site2_unit3(&self) -> u32 {
        (self.bits & CombPdSite2Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT3_MASK)
            >> CombPdSite2Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT3_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE2_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site2_unit3(&self) -> bool {
        (self.bits & CombPdSite2Unit3Vt0Conf1::MONITOR_EN_VT0_PD_SITE2_UNIT3_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE2_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site2_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT3_MASK)
                | ((value << CombPdSite2Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT3_OFFSET)
                    & CombPdSite2Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE2_UNIT3_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT3** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site2_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit3Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT3_MASK
            } else {
                self.bits & !CombPdSite2Unit3Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT3_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE2_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site2_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit3Vt0Conf1::MONITOR_EN_VT0_PD_SITE2_UNIT3_MASK
            } else {
                self.bits & !CombPdSite2Unit3Vt0Conf1::MONITOR_EN_VT0_PD_SITE2_UNIT3_MASK
            },
        }
    }
}

impl Default for CombPdSite2Unit3Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite2Unit3Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit3Vt0Conf1Val {
        CombPdSite2Unit3Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit3Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit3Vt0Conf1Val) -> CombPdSite2Unit3Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT0_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00d8
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE2_UNIT0** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE2_UNIT0** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE2_UNIT0** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT0** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE2_UNIT0** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit0Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit0Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit0Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE2_UNIT0_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE2_UNIT0_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE2_UNIT0: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE2_UNIT0_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE2_UNIT0_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE2_UNIT0_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE2_UNIT0_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT0_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT0_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT0: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE2_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE2_UNIT0_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE2_UNIT0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE2_UNIT0** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site2_unit0(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE2_UNIT0_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE2_UNIT0** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site2_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE2_UNIT0_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE2_UNIT0_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE2_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site2_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT0_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT0_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE2_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site2_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT0_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT0_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT0_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT0** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site2_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT0_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE2_UNIT0** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site2_unit0(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE2_UNIT0_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE2_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site2_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE2_UNIT0_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE2_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site2_unit0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE2_UNIT0_MASK);
    }
}

/// COMB_PD_SITE2_UNIT0_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit0vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit0vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit0Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite2Unit0Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE2_UNIT0** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site2_unit0(&self) -> bool {
        (self.bits & CombPdSite2Unit0Vt1Conf1::TIMING_ERR_VT1_PD_SITE2_UNIT0_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE2_UNIT0** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site2_unit0(&self) -> u32 {
        (self.bits & CombPdSite2Unit0Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE2_UNIT0_MASK)
            >> CombPdSite2Unit0Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE2_UNIT0_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE2_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site2_unit0(&self) -> u32 {
        (self.bits & CombPdSite2Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT0_MASK)
            >> CombPdSite2Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT0_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE2_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site2_unit0(&self) -> bool {
        (self.bits & CombPdSite2Unit0Vt1Conf1::MONITOR_EN_VT1_PD_SITE2_UNIT0_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE2_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site2_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT0_MASK)
                | ((value << CombPdSite2Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT0_OFFSET)
                    & CombPdSite2Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT0_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT0** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site2_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit0Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT0_MASK
            } else {
                self.bits & !CombPdSite2Unit0Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT0_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE2_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site2_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit0Vt1Conf1::MONITOR_EN_VT1_PD_SITE2_UNIT0_MASK
            } else {
                self.bits & !CombPdSite2Unit0Vt1Conf1::MONITOR_EN_VT1_PD_SITE2_UNIT0_MASK
            },
        }
    }
}

impl Default for CombPdSite2Unit0Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite2Unit0Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit0Vt1Conf1Val {
        CombPdSite2Unit0Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit0Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit0Vt1Conf1Val) -> CombPdSite2Unit0Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT1_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00dc
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE2_UNIT1** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE2_UNIT1** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE2_UNIT1** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT1** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE2_UNIT1** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit1Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit1Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit1Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE2_UNIT1_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE2_UNIT1_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE2_UNIT1: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE2_UNIT1_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE2_UNIT1_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE2_UNIT1_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE2_UNIT1_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT1_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT1_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT1: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE2_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE2_UNIT1_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE2_UNIT1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE2_UNIT1** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site2_unit1(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE2_UNIT1_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE2_UNIT1** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site2_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE2_UNIT1_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE2_UNIT1_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE2_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site2_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT1_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT1_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE2_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site2_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT1_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT1_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT1_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT1** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site2_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT1_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE2_UNIT1** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site2_unit1(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE2_UNIT1_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE2_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site2_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE2_UNIT1_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE2_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site2_unit1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE2_UNIT1_MASK);
    }
}

/// COMB_PD_SITE2_UNIT1_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit1vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit1vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit1Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite2Unit1Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE2_UNIT1** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site2_unit1(&self) -> bool {
        (self.bits & CombPdSite2Unit1Vt1Conf1::TIMING_ERR_VT1_PD_SITE2_UNIT1_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE2_UNIT1** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site2_unit1(&self) -> u32 {
        (self.bits & CombPdSite2Unit1Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE2_UNIT1_MASK)
            >> CombPdSite2Unit1Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE2_UNIT1_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE2_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site2_unit1(&self) -> u32 {
        (self.bits & CombPdSite2Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT1_MASK)
            >> CombPdSite2Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT1_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE2_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site2_unit1(&self) -> bool {
        (self.bits & CombPdSite2Unit1Vt1Conf1::MONITOR_EN_VT1_PD_SITE2_UNIT1_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE2_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site2_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT1_MASK)
                | ((value << CombPdSite2Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT1_OFFSET)
                    & CombPdSite2Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT1_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT1** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site2_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit1Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT1_MASK
            } else {
                self.bits & !CombPdSite2Unit1Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT1_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE2_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site2_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit1Vt1Conf1::MONITOR_EN_VT1_PD_SITE2_UNIT1_MASK
            } else {
                self.bits & !CombPdSite2Unit1Vt1Conf1::MONITOR_EN_VT1_PD_SITE2_UNIT1_MASK
            },
        }
    }
}

impl Default for CombPdSite2Unit1Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite2Unit1Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit1Vt1Conf1Val {
        CombPdSite2Unit1Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit1Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit1Vt1Conf1Val) -> CombPdSite2Unit1Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT2_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00e0
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE2_UNIT2** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE2_UNIT2** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE2_UNIT2** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT2** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE2_UNIT2** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit2Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit2Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit2Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE2_UNIT2_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE2_UNIT2_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE2_UNIT2: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE2_UNIT2_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE2_UNIT2_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE2_UNIT2_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE2_UNIT2_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT2_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT2_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT2: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE2_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE2_UNIT2_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE2_UNIT2: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE2_UNIT2** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site2_unit2(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE2_UNIT2_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE2_UNIT2** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site2_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE2_UNIT2_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE2_UNIT2_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE2_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site2_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT2_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT2_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE2_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site2_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT2_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT2_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT2_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT2** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site2_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT2_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE2_UNIT2** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site2_unit2(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE2_UNIT2_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE2_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site2_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE2_UNIT2_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE2_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site2_unit2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE2_UNIT2_MASK);
    }
}

/// COMB_PD_SITE2_UNIT2_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit2vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit2vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit2Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite2Unit2Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE2_UNIT2** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site2_unit2(&self) -> bool {
        (self.bits & CombPdSite2Unit2Vt1Conf1::TIMING_ERR_VT1_PD_SITE2_UNIT2_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE2_UNIT2** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site2_unit2(&self) -> u32 {
        (self.bits & CombPdSite2Unit2Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE2_UNIT2_MASK)
            >> CombPdSite2Unit2Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE2_UNIT2_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE2_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site2_unit2(&self) -> u32 {
        (self.bits & CombPdSite2Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT2_MASK)
            >> CombPdSite2Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT2_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE2_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site2_unit2(&self) -> bool {
        (self.bits & CombPdSite2Unit2Vt1Conf1::MONITOR_EN_VT1_PD_SITE2_UNIT2_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE2_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site2_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT2_MASK)
                | ((value << CombPdSite2Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT2_OFFSET)
                    & CombPdSite2Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT2_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT2** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site2_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit2Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT2_MASK
            } else {
                self.bits & !CombPdSite2Unit2Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT2_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE2_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site2_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit2Vt1Conf1::MONITOR_EN_VT1_PD_SITE2_UNIT2_MASK
            } else {
                self.bits & !CombPdSite2Unit2Vt1Conf1::MONITOR_EN_VT1_PD_SITE2_UNIT2_MASK
            },
        }
    }
}

impl Default for CombPdSite2Unit2Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite2Unit2Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit2Vt1Conf1Val {
        CombPdSite2Unit2Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit2Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit2Vt1Conf1Val) -> CombPdSite2Unit2Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT3_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00e4
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE2_UNIT3** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE2_UNIT3** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE2_UNIT3** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT3** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE2_UNIT3** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit3Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit3Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit3Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE2_UNIT3_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE2_UNIT3_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE2_UNIT3: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE2_UNIT3_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE2_UNIT3_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE2_UNIT3_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE2_UNIT3_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT3_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT3_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT3: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE2_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE2_UNIT3_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE2_UNIT3: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE2_UNIT3** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site2_unit3(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE2_UNIT3_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE2_UNIT3** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site2_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE2_UNIT3_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE2_UNIT3_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE2_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site2_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT3_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT3_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE2_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site2_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT3_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT3_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE2_UNIT3_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT3** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site2_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT3_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE2_UNIT3** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site2_unit3(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE2_UNIT3_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE2_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site2_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE2_UNIT3_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE2_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site2_unit3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE2_UNIT3_MASK);
    }
}

/// COMB_PD_SITE2_UNIT3_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit3vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit3vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit3Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite2Unit3Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE2_UNIT3** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site2_unit3(&self) -> bool {
        (self.bits & CombPdSite2Unit3Vt1Conf1::TIMING_ERR_VT1_PD_SITE2_UNIT3_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE2_UNIT3** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site2_unit3(&self) -> u32 {
        (self.bits & CombPdSite2Unit3Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE2_UNIT3_MASK)
            >> CombPdSite2Unit3Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE2_UNIT3_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE2_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site2_unit3(&self) -> u32 {
        (self.bits & CombPdSite2Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT3_MASK)
            >> CombPdSite2Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT3_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE2_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site2_unit3(&self) -> bool {
        (self.bits & CombPdSite2Unit3Vt1Conf1::MONITOR_EN_VT1_PD_SITE2_UNIT3_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE2_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site2_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT3_MASK)
                | ((value << CombPdSite2Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT3_OFFSET)
                    & CombPdSite2Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE2_UNIT3_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT3** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site2_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit3Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT3_MASK
            } else {
                self.bits & !CombPdSite2Unit3Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE2_UNIT3_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE2_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site2_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit3Vt1Conf1::MONITOR_EN_VT1_PD_SITE2_UNIT3_MASK
            } else {
                self.bits & !CombPdSite2Unit3Vt1Conf1::MONITOR_EN_VT1_PD_SITE2_UNIT3_MASK
            },
        }
    }
}

impl Default for CombPdSite2Unit3Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite2Unit3Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit3Vt1Conf1Val {
        CombPdSite2Unit3Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit3Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit3Vt1Conf1Val) -> CombPdSite2Unit3Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT0_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00e8
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE2_UNIT0** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE2_UNIT0** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE2_UNIT0** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT0** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE2_UNIT0** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit0Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit0Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit0Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE2_UNIT0_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE2_UNIT0_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE2_UNIT0: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE2_UNIT0_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE2_UNIT0_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE2_UNIT0_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE2_UNIT0_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT0_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT0_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT0: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE2_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE2_UNIT0_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE2_UNIT0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE2_UNIT0** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site2_unit0(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE2_UNIT0_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE2_UNIT0** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site2_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE2_UNIT0_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE2_UNIT0_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE2_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site2_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT0_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT0_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE2_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site2_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT0_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT0_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT0_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT0** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site2_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT0_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE2_UNIT0** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site2_unit0(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE2_UNIT0_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE2_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site2_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE2_UNIT0_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE2_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site2_unit0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE2_UNIT0_MASK);
    }
}

/// COMB_PD_SITE2_UNIT0_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit0vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit0vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit0Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite2Unit0Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE2_UNIT0** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site2_unit0(&self) -> bool {
        (self.bits & CombPdSite2Unit0Vt2Conf1::TIMING_ERR_VT2_PD_SITE2_UNIT0_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE2_UNIT0** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site2_unit0(&self) -> u32 {
        (self.bits & CombPdSite2Unit0Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE2_UNIT0_MASK)
            >> CombPdSite2Unit0Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE2_UNIT0_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE2_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site2_unit0(&self) -> u32 {
        (self.bits & CombPdSite2Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT0_MASK)
            >> CombPdSite2Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT0_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE2_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site2_unit0(&self) -> bool {
        (self.bits & CombPdSite2Unit0Vt2Conf1::MONITOR_EN_VT2_PD_SITE2_UNIT0_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE2_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site2_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT0_MASK)
                | ((value << CombPdSite2Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT0_OFFSET)
                    & CombPdSite2Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT0_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT0** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site2_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit0Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT0_MASK
            } else {
                self.bits & !CombPdSite2Unit0Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT0_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE2_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site2_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit0Vt2Conf1::MONITOR_EN_VT2_PD_SITE2_UNIT0_MASK
            } else {
                self.bits & !CombPdSite2Unit0Vt2Conf1::MONITOR_EN_VT2_PD_SITE2_UNIT0_MASK
            },
        }
    }
}

impl Default for CombPdSite2Unit0Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite2Unit0Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit0Vt2Conf1Val {
        CombPdSite2Unit0Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit0Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit0Vt2Conf1Val) -> CombPdSite2Unit0Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT1_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00ec
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE2_UNIT1** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE2_UNIT1** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE2_UNIT1** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT1** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE2_UNIT1** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit1Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit1Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit1Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE2_UNIT1_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE2_UNIT1_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE2_UNIT1: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE2_UNIT1_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE2_UNIT1_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE2_UNIT1_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE2_UNIT1_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT1_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT1_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT1: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE2_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE2_UNIT1_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE2_UNIT1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE2_UNIT1** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site2_unit1(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE2_UNIT1_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE2_UNIT1** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site2_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE2_UNIT1_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE2_UNIT1_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE2_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site2_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT1_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT1_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE2_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site2_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT1_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT1_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT1_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT1** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site2_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT1_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE2_UNIT1** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site2_unit1(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE2_UNIT1_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE2_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site2_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE2_UNIT1_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE2_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site2_unit1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE2_UNIT1_MASK);
    }
}

/// COMB_PD_SITE2_UNIT1_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit1vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit1vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit1Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite2Unit1Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE2_UNIT1** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site2_unit1(&self) -> bool {
        (self.bits & CombPdSite2Unit1Vt2Conf1::TIMING_ERR_VT2_PD_SITE2_UNIT1_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE2_UNIT1** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site2_unit1(&self) -> u32 {
        (self.bits & CombPdSite2Unit1Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE2_UNIT1_MASK)
            >> CombPdSite2Unit1Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE2_UNIT1_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE2_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site2_unit1(&self) -> u32 {
        (self.bits & CombPdSite2Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT1_MASK)
            >> CombPdSite2Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT1_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE2_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site2_unit1(&self) -> bool {
        (self.bits & CombPdSite2Unit1Vt2Conf1::MONITOR_EN_VT2_PD_SITE2_UNIT1_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE2_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site2_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT1_MASK)
                | ((value << CombPdSite2Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT1_OFFSET)
                    & CombPdSite2Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT1_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT1** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site2_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit1Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT1_MASK
            } else {
                self.bits & !CombPdSite2Unit1Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT1_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE2_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site2_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit1Vt2Conf1::MONITOR_EN_VT2_PD_SITE2_UNIT1_MASK
            } else {
                self.bits & !CombPdSite2Unit1Vt2Conf1::MONITOR_EN_VT2_PD_SITE2_UNIT1_MASK
            },
        }
    }
}

impl Default for CombPdSite2Unit1Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite2Unit1Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit1Vt2Conf1Val {
        CombPdSite2Unit1Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit1Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit1Vt2Conf1Val) -> CombPdSite2Unit1Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT2_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00f0
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE2_UNIT2** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE2_UNIT2** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE2_UNIT2** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT2** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE2_UNIT2** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit2Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit2Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit2Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE2_UNIT2_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE2_UNIT2_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE2_UNIT2: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE2_UNIT2_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE2_UNIT2_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE2_UNIT2_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE2_UNIT2_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT2_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT2_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT2: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE2_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE2_UNIT2_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE2_UNIT2: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE2_UNIT2** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site2_unit2(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE2_UNIT2_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE2_UNIT2** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site2_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE2_UNIT2_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE2_UNIT2_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE2_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site2_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT2_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT2_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE2_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site2_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT2_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT2_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT2_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT2** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site2_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT2_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE2_UNIT2** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site2_unit2(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE2_UNIT2_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE2_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site2_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE2_UNIT2_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE2_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site2_unit2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE2_UNIT2_MASK);
    }
}

/// COMB_PD_SITE2_UNIT2_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit2vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit2vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit2Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite2Unit2Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE2_UNIT2** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site2_unit2(&self) -> bool {
        (self.bits & CombPdSite2Unit2Vt2Conf1::TIMING_ERR_VT2_PD_SITE2_UNIT2_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE2_UNIT2** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site2_unit2(&self) -> u32 {
        (self.bits & CombPdSite2Unit2Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE2_UNIT2_MASK)
            >> CombPdSite2Unit2Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE2_UNIT2_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE2_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site2_unit2(&self) -> u32 {
        (self.bits & CombPdSite2Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT2_MASK)
            >> CombPdSite2Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT2_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE2_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site2_unit2(&self) -> bool {
        (self.bits & CombPdSite2Unit2Vt2Conf1::MONITOR_EN_VT2_PD_SITE2_UNIT2_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE2_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site2_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT2_MASK)
                | ((value << CombPdSite2Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT2_OFFSET)
                    & CombPdSite2Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT2_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT2** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site2_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit2Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT2_MASK
            } else {
                self.bits & !CombPdSite2Unit2Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT2_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE2_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site2_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit2Vt2Conf1::MONITOR_EN_VT2_PD_SITE2_UNIT2_MASK
            } else {
                self.bits & !CombPdSite2Unit2Vt2Conf1::MONITOR_EN_VT2_PD_SITE2_UNIT2_MASK
            },
        }
    }
}

impl Default for CombPdSite2Unit2Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite2Unit2Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit2Vt2Conf1Val {
        CombPdSite2Unit2Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit2Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit2Vt2Conf1Val) -> CombPdSite2Unit2Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT3_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00f4
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE2_UNIT3** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE2_UNIT3** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE2_UNIT3** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT3** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE2_UNIT3** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit3Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit3Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit3Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE2_UNIT3_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE2_UNIT3_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE2_UNIT3: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE2_UNIT3_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE2_UNIT3_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE2_UNIT3_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE2_UNIT3_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT3_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT3_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT3: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE2_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE2_UNIT3_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE2_UNIT3: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE2_UNIT3** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site2_unit3(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE2_UNIT3_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE2_UNIT3** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site2_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE2_UNIT3_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE2_UNIT3_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE2_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site2_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT3_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT3_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE2_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site2_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT3_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT3_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE2_UNIT3_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT3** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site2_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT3_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE2_UNIT3** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site2_unit3(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE2_UNIT3_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE2_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site2_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE2_UNIT3_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE2_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site2_unit3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE2_UNIT3_MASK);
    }
}

/// COMB_PD_SITE2_UNIT3_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit3vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit3vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit3Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite2Unit3Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE2_UNIT3** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site2_unit3(&self) -> bool {
        (self.bits & CombPdSite2Unit3Vt2Conf1::TIMING_ERR_VT2_PD_SITE2_UNIT3_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE2_UNIT3** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site2_unit3(&self) -> u32 {
        (self.bits & CombPdSite2Unit3Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE2_UNIT3_MASK)
            >> CombPdSite2Unit3Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE2_UNIT3_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE2_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site2_unit3(&self) -> u32 {
        (self.bits & CombPdSite2Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT3_MASK)
            >> CombPdSite2Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT3_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE2_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site2_unit3(&self) -> bool {
        (self.bits & CombPdSite2Unit3Vt2Conf1::MONITOR_EN_VT2_PD_SITE2_UNIT3_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE2_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site2_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT3_MASK)
                | ((value << CombPdSite2Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT3_OFFSET)
                    & CombPdSite2Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE2_UNIT3_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT3** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site2_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit3Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT3_MASK
            } else {
                self.bits & !CombPdSite2Unit3Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE2_UNIT3_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE2_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site2_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite2Unit3Vt2Conf1::MONITOR_EN_VT2_PD_SITE2_UNIT3_MASK
            } else {
                self.bits & !CombPdSite2Unit3Vt2Conf1::MONITOR_EN_VT2_PD_SITE2_UNIT3_MASK
            },
        }
    }
}

impl Default for CombPdSite2Unit3Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite2Unit3Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit3Vt2Conf1Val {
        CombPdSite2Unit3Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit3Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit3Vt2Conf1Val) -> CombPdSite2Unit3Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT0_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00f8
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE3_UNIT0** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE3_UNIT0** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE3_UNIT0** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT0** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE3_UNIT0** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit0Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit0Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit0Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE3_UNIT0_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE3_UNIT0_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE3_UNIT0: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE3_UNIT0_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE3_UNIT0_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE3_UNIT0_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE3_UNIT0_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT0_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT0_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT0: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE3_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE3_UNIT0_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE3_UNIT0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE3_UNIT0** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site3_unit0(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE3_UNIT0_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE3_UNIT0** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site3_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE3_UNIT0_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE3_UNIT0_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE3_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site3_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT0_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT0_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE3_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site3_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT0_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT0_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT0_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT0** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site3_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT0_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE3_UNIT0** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site3_unit0(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE3_UNIT0_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE3_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site3_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE3_UNIT0_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE3_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site3_unit0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE3_UNIT0_MASK);
    }
}

/// COMB_PD_SITE3_UNIT0_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit0vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit0vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit0Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite3Unit0Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE3_UNIT0** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site3_unit0(&self) -> bool {
        (self.bits & CombPdSite3Unit0Vt0Conf1::TIMING_ERR_VT0_PD_SITE3_UNIT0_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE3_UNIT0** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site3_unit0(&self) -> u32 {
        (self.bits & CombPdSite3Unit0Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE3_UNIT0_MASK)
            >> CombPdSite3Unit0Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE3_UNIT0_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE3_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site3_unit0(&self) -> u32 {
        (self.bits & CombPdSite3Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT0_MASK)
            >> CombPdSite3Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT0_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE3_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site3_unit0(&self) -> bool {
        (self.bits & CombPdSite3Unit0Vt0Conf1::MONITOR_EN_VT0_PD_SITE3_UNIT0_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE3_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site3_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT0_MASK)
                | ((value << CombPdSite3Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT0_OFFSET)
                    & CombPdSite3Unit0Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT0_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT0** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site3_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit0Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT0_MASK
            } else {
                self.bits & !CombPdSite3Unit0Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT0_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE3_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site3_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit0Vt0Conf1::MONITOR_EN_VT0_PD_SITE3_UNIT0_MASK
            } else {
                self.bits & !CombPdSite3Unit0Vt0Conf1::MONITOR_EN_VT0_PD_SITE3_UNIT0_MASK
            },
        }
    }
}

impl Default for CombPdSite3Unit0Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite3Unit0Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit0Vt0Conf1Val {
        CombPdSite3Unit0Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit0Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit0Vt0Conf1Val) -> CombPdSite3Unit0Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT1_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x00fc
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE3_UNIT1** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE3_UNIT1** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE3_UNIT1** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT1** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE3_UNIT1** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit1Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit1Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit1Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE3_UNIT1_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE3_UNIT1_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE3_UNIT1: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE3_UNIT1_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE3_UNIT1_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE3_UNIT1_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE3_UNIT1_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT1_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT1_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT1: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE3_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE3_UNIT1_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE3_UNIT1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE3_UNIT1** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site3_unit1(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE3_UNIT1_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE3_UNIT1** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site3_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE3_UNIT1_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE3_UNIT1_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE3_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site3_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT1_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT1_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE3_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site3_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT1_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT1_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT1_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT1** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site3_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT1_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE3_UNIT1** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site3_unit1(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE3_UNIT1_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE3_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site3_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE3_UNIT1_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE3_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site3_unit1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE3_UNIT1_MASK);
    }
}

/// COMB_PD_SITE3_UNIT1_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit1vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit1vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit1Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite3Unit1Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE3_UNIT1** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site3_unit1(&self) -> bool {
        (self.bits & CombPdSite3Unit1Vt0Conf1::TIMING_ERR_VT0_PD_SITE3_UNIT1_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE3_UNIT1** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site3_unit1(&self) -> u32 {
        (self.bits & CombPdSite3Unit1Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE3_UNIT1_MASK)
            >> CombPdSite3Unit1Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE3_UNIT1_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE3_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site3_unit1(&self) -> u32 {
        (self.bits & CombPdSite3Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT1_MASK)
            >> CombPdSite3Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT1_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE3_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site3_unit1(&self) -> bool {
        (self.bits & CombPdSite3Unit1Vt0Conf1::MONITOR_EN_VT0_PD_SITE3_UNIT1_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE3_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site3_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT1_MASK)
                | ((value << CombPdSite3Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT1_OFFSET)
                    & CombPdSite3Unit1Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT1_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT1** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site3_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit1Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT1_MASK
            } else {
                self.bits & !CombPdSite3Unit1Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT1_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE3_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site3_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit1Vt0Conf1::MONITOR_EN_VT0_PD_SITE3_UNIT1_MASK
            } else {
                self.bits & !CombPdSite3Unit1Vt0Conf1::MONITOR_EN_VT0_PD_SITE3_UNIT1_MASK
            },
        }
    }
}

impl Default for CombPdSite3Unit1Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite3Unit1Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit1Vt0Conf1Val {
        CombPdSite3Unit1Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit1Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit1Vt0Conf1Val) -> CombPdSite3Unit1Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT2_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0100
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE3_UNIT2** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE3_UNIT2** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE3_UNIT2** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT2** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE3_UNIT2** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit2Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit2Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit2Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE3_UNIT2_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE3_UNIT2_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE3_UNIT2: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE3_UNIT2_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE3_UNIT2_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE3_UNIT2_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE3_UNIT2_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT2_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT2_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT2: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE3_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE3_UNIT2_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE3_UNIT2: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE3_UNIT2** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site3_unit2(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE3_UNIT2_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE3_UNIT2** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site3_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE3_UNIT2_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE3_UNIT2_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE3_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site3_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT2_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT2_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE3_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site3_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT2_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT2_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT2_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT2** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site3_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT2_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE3_UNIT2** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site3_unit2(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE3_UNIT2_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE3_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site3_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE3_UNIT2_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE3_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site3_unit2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE3_UNIT2_MASK);
    }
}

/// COMB_PD_SITE3_UNIT2_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit2vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit2vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit2Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite3Unit2Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE3_UNIT2** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site3_unit2(&self) -> bool {
        (self.bits & CombPdSite3Unit2Vt0Conf1::TIMING_ERR_VT0_PD_SITE3_UNIT2_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE3_UNIT2** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site3_unit2(&self) -> u32 {
        (self.bits & CombPdSite3Unit2Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE3_UNIT2_MASK)
            >> CombPdSite3Unit2Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE3_UNIT2_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE3_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site3_unit2(&self) -> u32 {
        (self.bits & CombPdSite3Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT2_MASK)
            >> CombPdSite3Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT2_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE3_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site3_unit2(&self) -> bool {
        (self.bits & CombPdSite3Unit2Vt0Conf1::MONITOR_EN_VT0_PD_SITE3_UNIT2_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE3_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site3_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT2_MASK)
                | ((value << CombPdSite3Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT2_OFFSET)
                    & CombPdSite3Unit2Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT2_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT2** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site3_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit2Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT2_MASK
            } else {
                self.bits & !CombPdSite3Unit2Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT2_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE3_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site3_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit2Vt0Conf1::MONITOR_EN_VT0_PD_SITE3_UNIT2_MASK
            } else {
                self.bits & !CombPdSite3Unit2Vt0Conf1::MONITOR_EN_VT0_PD_SITE3_UNIT2_MASK
            },
        }
    }
}

impl Default for CombPdSite3Unit2Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite3Unit2Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit2Vt0Conf1Val {
        CombPdSite3Unit2Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit2Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit2Vt0Conf1Val) -> CombPdSite3Unit2Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT3_VT0_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0104
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT0_PD_SITE3_UNIT3** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT0_PD_SITE3_UNIT3** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT0_PD_SITE3_UNIT3** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT3** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT0_PD_SITE3_UNIT3** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit3Vt0Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit3Vt0Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit3Vt0Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT0_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT0_PD_SITE3_UNIT3_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT0_PD_SITE3_UNIT3_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT0_PD_SITE3_UNIT3: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT0_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT0_PD_SITE3_UNIT3_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT0_PD_SITE3_UNIT3_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT0_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT0_PD_SITE3_UNIT3_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT0_PD_SITE3_UNIT3_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT3_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT3_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT3: u32 = 1 << 1;

    /// **MONITOR_EN_VT0_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT0_PD_SITE3_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT0_PD_SITE3_UNIT3_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT0_PD_SITE3_UNIT3: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT0_PD_SITE3_UNIT3** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt0_pd_site3_unit3(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT0_PD_SITE3_UNIT3_MASK) != 0
    }

    /// **DELAY_NUM_O_VT0_PD_SITE3_UNIT3** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site3_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT0_PD_SITE3_UNIT3_MASK)
            >> Self::DELAY_NUM_O_VT0_PD_SITE3_UNIT3_OFFSET
    }

    /// **DELAY_LIMIT_VT0_PD_SITE3_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site3_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT3_MASK)
            >> Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT3_OFFSET
    }

    /// Write **DELAY_LIMIT_VT0_PD_SITE3_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt0_pd_site3_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT3_MASK)
                | ((value << Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT3_OFFSET)
                    & Self::DELAY_LIMIT_VT0_PD_SITE3_UNIT3_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT3** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt0_pd_site3_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT3_MASK);
    }

    /// **MONITOR_EN_VT0_PD_SITE3_UNIT3** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site3_unit3(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT0_PD_SITE3_UNIT3_MASK) != 0
    }

    /// Set MONITOR_EN_VT0_PD_SITE3_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt0_pd_site3_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT0_PD_SITE3_UNIT3_MASK);
    }

    /// Clear MONITOR_EN_VT0_PD_SITE3_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt0_pd_site3_unit3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT0_PD_SITE3_UNIT3_MASK);
    }
}

/// COMB_PD_SITE3_UNIT3_VT0_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit3vt0conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit3vt0conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit3Vt0Conf1Val {
    bits: u32,
}

impl CombPdSite3Unit3Vt0Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT0_PD_SITE3_UNIT3** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt0_pd_site3_unit3(&self) -> bool {
        (self.bits & CombPdSite3Unit3Vt0Conf1::TIMING_ERR_VT0_PD_SITE3_UNIT3_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT0_PD_SITE3_UNIT3** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt0_pd_site3_unit3(&self) -> u32 {
        (self.bits & CombPdSite3Unit3Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE3_UNIT3_MASK)
            >> CombPdSite3Unit3Vt0Conf1::DELAY_NUM_O_VT0_PD_SITE3_UNIT3_OFFSET
    }

    /// Read **DELAY_LIMIT_VT0_PD_SITE3_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt0_pd_site3_unit3(&self) -> u32 {
        (self.bits & CombPdSite3Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT3_MASK)
            >> CombPdSite3Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT3_OFFSET
    }

    /// Read **MONITOR_EN_VT0_PD_SITE3_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt0_pd_site3_unit3(&self) -> bool {
        (self.bits & CombPdSite3Unit3Vt0Conf1::MONITOR_EN_VT0_PD_SITE3_UNIT3_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT0_PD_SITE3_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt0_pd_site3_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT3_MASK)
                | ((value << CombPdSite3Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT3_OFFSET)
                    & CombPdSite3Unit3Vt0Conf1::DELAY_LIMIT_VT0_PD_SITE3_UNIT3_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT3** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt0_pd_site3_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit3Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT3_MASK
            } else {
                self.bits & !CombPdSite3Unit3Vt0Conf1::TIMING_ERR_CNT_CLR_VT0_PD_SITE3_UNIT3_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT0_PD_SITE3_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt0_pd_site3_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit3Vt0Conf1::MONITOR_EN_VT0_PD_SITE3_UNIT3_MASK
            } else {
                self.bits & !CombPdSite3Unit3Vt0Conf1::MONITOR_EN_VT0_PD_SITE3_UNIT3_MASK
            },
        }
    }
}

impl Default for CombPdSite3Unit3Vt0Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite3Unit3Vt0Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit3Vt0Conf1Val {
        CombPdSite3Unit3Vt0Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit3Vt0Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit3Vt0Conf1Val) -> CombPdSite3Unit3Vt0Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT0_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0108
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE3_UNIT0** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE3_UNIT0** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE3_UNIT0** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT0** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE3_UNIT0** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit0Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit0Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit0Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE3_UNIT0_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE3_UNIT0_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE3_UNIT0: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE3_UNIT0_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE3_UNIT0_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE3_UNIT0_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE3_UNIT0_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT0_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT0_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT0: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE3_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE3_UNIT0_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE3_UNIT0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE3_UNIT0** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site3_unit0(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE3_UNIT0_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE3_UNIT0** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site3_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE3_UNIT0_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE3_UNIT0_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE3_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site3_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT0_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT0_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE3_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site3_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT0_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT0_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT0_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT0** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site3_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT0_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE3_UNIT0** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site3_unit0(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE3_UNIT0_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE3_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site3_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE3_UNIT0_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE3_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site3_unit0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE3_UNIT0_MASK);
    }
}

/// COMB_PD_SITE3_UNIT0_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit0vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit0vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit0Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite3Unit0Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE3_UNIT0** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site3_unit0(&self) -> bool {
        (self.bits & CombPdSite3Unit0Vt1Conf1::TIMING_ERR_VT1_PD_SITE3_UNIT0_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE3_UNIT0** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site3_unit0(&self) -> u32 {
        (self.bits & CombPdSite3Unit0Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE3_UNIT0_MASK)
            >> CombPdSite3Unit0Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE3_UNIT0_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE3_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site3_unit0(&self) -> u32 {
        (self.bits & CombPdSite3Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT0_MASK)
            >> CombPdSite3Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT0_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE3_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site3_unit0(&self) -> bool {
        (self.bits & CombPdSite3Unit0Vt1Conf1::MONITOR_EN_VT1_PD_SITE3_UNIT0_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE3_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site3_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT0_MASK)
                | ((value << CombPdSite3Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT0_OFFSET)
                    & CombPdSite3Unit0Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT0_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT0** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site3_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit0Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT0_MASK
            } else {
                self.bits & !CombPdSite3Unit0Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT0_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE3_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site3_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit0Vt1Conf1::MONITOR_EN_VT1_PD_SITE3_UNIT0_MASK
            } else {
                self.bits & !CombPdSite3Unit0Vt1Conf1::MONITOR_EN_VT1_PD_SITE3_UNIT0_MASK
            },
        }
    }
}

impl Default for CombPdSite3Unit0Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite3Unit0Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit0Vt1Conf1Val {
        CombPdSite3Unit0Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit0Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit0Vt1Conf1Val) -> CombPdSite3Unit0Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT1_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x010c
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE3_UNIT1** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE3_UNIT1** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE3_UNIT1** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT1** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE3_UNIT1** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit1Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit1Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit1Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE3_UNIT1_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE3_UNIT1_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE3_UNIT1: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE3_UNIT1_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE3_UNIT1_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE3_UNIT1_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE3_UNIT1_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT1_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT1_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT1: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE3_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE3_UNIT1_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE3_UNIT1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE3_UNIT1** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site3_unit1(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE3_UNIT1_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE3_UNIT1** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site3_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE3_UNIT1_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE3_UNIT1_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE3_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site3_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT1_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT1_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE3_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site3_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT1_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT1_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT1_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT1** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site3_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT1_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE3_UNIT1** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site3_unit1(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE3_UNIT1_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE3_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site3_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE3_UNIT1_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE3_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site3_unit1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE3_UNIT1_MASK);
    }
}

/// COMB_PD_SITE3_UNIT1_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit1vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit1vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit1Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite3Unit1Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE3_UNIT1** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site3_unit1(&self) -> bool {
        (self.bits & CombPdSite3Unit1Vt1Conf1::TIMING_ERR_VT1_PD_SITE3_UNIT1_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE3_UNIT1** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site3_unit1(&self) -> u32 {
        (self.bits & CombPdSite3Unit1Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE3_UNIT1_MASK)
            >> CombPdSite3Unit1Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE3_UNIT1_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE3_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site3_unit1(&self) -> u32 {
        (self.bits & CombPdSite3Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT1_MASK)
            >> CombPdSite3Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT1_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE3_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site3_unit1(&self) -> bool {
        (self.bits & CombPdSite3Unit1Vt1Conf1::MONITOR_EN_VT1_PD_SITE3_UNIT1_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE3_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site3_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT1_MASK)
                | ((value << CombPdSite3Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT1_OFFSET)
                    & CombPdSite3Unit1Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT1_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT1** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site3_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit1Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT1_MASK
            } else {
                self.bits & !CombPdSite3Unit1Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT1_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE3_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site3_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit1Vt1Conf1::MONITOR_EN_VT1_PD_SITE3_UNIT1_MASK
            } else {
                self.bits & !CombPdSite3Unit1Vt1Conf1::MONITOR_EN_VT1_PD_SITE3_UNIT1_MASK
            },
        }
    }
}

impl Default for CombPdSite3Unit1Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite3Unit1Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit1Vt1Conf1Val {
        CombPdSite3Unit1Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit1Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit1Vt1Conf1Val) -> CombPdSite3Unit1Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT2_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0110
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE3_UNIT2** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE3_UNIT2** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE3_UNIT2** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT2** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE3_UNIT2** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit2Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit2Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit2Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE3_UNIT2_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE3_UNIT2_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE3_UNIT2: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE3_UNIT2_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE3_UNIT2_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE3_UNIT2_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE3_UNIT2_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT2_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT2_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT2: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE3_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE3_UNIT2_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE3_UNIT2: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE3_UNIT2** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site3_unit2(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE3_UNIT2_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE3_UNIT2** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site3_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE3_UNIT2_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE3_UNIT2_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE3_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site3_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT2_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT2_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE3_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site3_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT2_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT2_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT2_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT2** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site3_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT2_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE3_UNIT2** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site3_unit2(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE3_UNIT2_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE3_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site3_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE3_UNIT2_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE3_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site3_unit2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE3_UNIT2_MASK);
    }
}

/// COMB_PD_SITE3_UNIT2_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit2vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit2vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit2Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite3Unit2Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE3_UNIT2** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site3_unit2(&self) -> bool {
        (self.bits & CombPdSite3Unit2Vt1Conf1::TIMING_ERR_VT1_PD_SITE3_UNIT2_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE3_UNIT2** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site3_unit2(&self) -> u32 {
        (self.bits & CombPdSite3Unit2Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE3_UNIT2_MASK)
            >> CombPdSite3Unit2Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE3_UNIT2_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE3_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site3_unit2(&self) -> u32 {
        (self.bits & CombPdSite3Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT2_MASK)
            >> CombPdSite3Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT2_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE3_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site3_unit2(&self) -> bool {
        (self.bits & CombPdSite3Unit2Vt1Conf1::MONITOR_EN_VT1_PD_SITE3_UNIT2_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE3_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site3_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT2_MASK)
                | ((value << CombPdSite3Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT2_OFFSET)
                    & CombPdSite3Unit2Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT2_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT2** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site3_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit2Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT2_MASK
            } else {
                self.bits & !CombPdSite3Unit2Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT2_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE3_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site3_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit2Vt1Conf1::MONITOR_EN_VT1_PD_SITE3_UNIT2_MASK
            } else {
                self.bits & !CombPdSite3Unit2Vt1Conf1::MONITOR_EN_VT1_PD_SITE3_UNIT2_MASK
            },
        }
    }
}

impl Default for CombPdSite3Unit2Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite3Unit2Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit2Vt1Conf1Val {
        CombPdSite3Unit2Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit2Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit2Vt1Conf1Val) -> CombPdSite3Unit2Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT3_VT1_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0114
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT1_PD_SITE3_UNIT3** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT1_PD_SITE3_UNIT3** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT1_PD_SITE3_UNIT3** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT3** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT1_PD_SITE3_UNIT3** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit3Vt1Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit3Vt1Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit3Vt1Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT1_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT1_PD_SITE3_UNIT3_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT1_PD_SITE3_UNIT3_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT1_PD_SITE3_UNIT3: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT1_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT1_PD_SITE3_UNIT3_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT1_PD_SITE3_UNIT3_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT1_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT1_PD_SITE3_UNIT3_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT1_PD_SITE3_UNIT3_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT3_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT3_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT3: u32 = 1 << 1;

    /// **MONITOR_EN_VT1_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT1_PD_SITE3_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT1_PD_SITE3_UNIT3_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT1_PD_SITE3_UNIT3: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT1_PD_SITE3_UNIT3** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt1_pd_site3_unit3(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT1_PD_SITE3_UNIT3_MASK) != 0
    }

    /// **DELAY_NUM_O_VT1_PD_SITE3_UNIT3** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt1_pd_site3_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT1_PD_SITE3_UNIT3_MASK)
            >> Self::DELAY_NUM_O_VT1_PD_SITE3_UNIT3_OFFSET
    }

    /// **DELAY_LIMIT_VT1_PD_SITE3_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt1_pd_site3_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT3_MASK)
            >> Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT3_OFFSET
    }

    /// Write **DELAY_LIMIT_VT1_PD_SITE3_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt1_pd_site3_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT3_MASK)
                | ((value << Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT3_OFFSET)
                    & Self::DELAY_LIMIT_VT1_PD_SITE3_UNIT3_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT3** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt1_pd_site3_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT3_MASK);
    }

    /// **MONITOR_EN_VT1_PD_SITE3_UNIT3** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt1_pd_site3_unit3(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT1_PD_SITE3_UNIT3_MASK) != 0
    }

    /// Set MONITOR_EN_VT1_PD_SITE3_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt1_pd_site3_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT1_PD_SITE3_UNIT3_MASK);
    }

    /// Clear MONITOR_EN_VT1_PD_SITE3_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt1_pd_site3_unit3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT1_PD_SITE3_UNIT3_MASK);
    }
}

/// COMB_PD_SITE3_UNIT3_VT1_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit3vt1conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit3vt1conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit3Vt1Conf1Val {
    bits: u32,
}

impl CombPdSite3Unit3Vt1Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT1_PD_SITE3_UNIT3** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt1_pd_site3_unit3(&self) -> bool {
        (self.bits & CombPdSite3Unit3Vt1Conf1::TIMING_ERR_VT1_PD_SITE3_UNIT3_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT1_PD_SITE3_UNIT3** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt1_pd_site3_unit3(&self) -> u32 {
        (self.bits & CombPdSite3Unit3Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE3_UNIT3_MASK)
            >> CombPdSite3Unit3Vt1Conf1::DELAY_NUM_O_VT1_PD_SITE3_UNIT3_OFFSET
    }

    /// Read **DELAY_LIMIT_VT1_PD_SITE3_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt1_pd_site3_unit3(&self) -> u32 {
        (self.bits & CombPdSite3Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT3_MASK)
            >> CombPdSite3Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT3_OFFSET
    }

    /// Read **MONITOR_EN_VT1_PD_SITE3_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt1_pd_site3_unit3(&self) -> bool {
        (self.bits & CombPdSite3Unit3Vt1Conf1::MONITOR_EN_VT1_PD_SITE3_UNIT3_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT1_PD_SITE3_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt1_pd_site3_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT3_MASK)
                | ((value << CombPdSite3Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT3_OFFSET)
                    & CombPdSite3Unit3Vt1Conf1::DELAY_LIMIT_VT1_PD_SITE3_UNIT3_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT3** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt1_pd_site3_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit3Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT3_MASK
            } else {
                self.bits & !CombPdSite3Unit3Vt1Conf1::TIMING_ERR_CNT_CLR_VT1_PD_SITE3_UNIT3_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT1_PD_SITE3_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt1_pd_site3_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit3Vt1Conf1::MONITOR_EN_VT1_PD_SITE3_UNIT3_MASK
            } else {
                self.bits & !CombPdSite3Unit3Vt1Conf1::MONITOR_EN_VT1_PD_SITE3_UNIT3_MASK
            },
        }
    }
}

impl Default for CombPdSite3Unit3Vt1Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite3Unit3Vt1Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit3Vt1Conf1Val {
        CombPdSite3Unit3Vt1Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit3Vt1Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit3Vt1Conf1Val) -> CombPdSite3Unit3Vt1Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT0_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0118
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE3_UNIT0** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE3_UNIT0** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE3_UNIT0** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT0** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE3_UNIT0** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit0Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit0Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit0Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE3_UNIT0_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE3_UNIT0_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE3_UNIT0: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE3_UNIT0_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE3_UNIT0_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE3_UNIT0_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE3_UNIT0_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT0_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT0_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT0: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE3_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE3_UNIT0_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE3_UNIT0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE3_UNIT0** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site3_unit0(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE3_UNIT0_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE3_UNIT0** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site3_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE3_UNIT0_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE3_UNIT0_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE3_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site3_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT0_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT0_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE3_UNIT0** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site3_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT0_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT0_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT0_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT0** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site3_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT0_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE3_UNIT0** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site3_unit0(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE3_UNIT0_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE3_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site3_unit0(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE3_UNIT0_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE3_UNIT0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site3_unit0(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE3_UNIT0_MASK);
    }
}

/// COMB_PD_SITE3_UNIT0_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit0vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit0vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit0Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite3Unit0Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE3_UNIT0** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site3_unit0(&self) -> bool {
        (self.bits & CombPdSite3Unit0Vt2Conf1::TIMING_ERR_VT2_PD_SITE3_UNIT0_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE3_UNIT0** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site3_unit0(&self) -> u32 {
        (self.bits & CombPdSite3Unit0Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE3_UNIT0_MASK)
            >> CombPdSite3Unit0Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE3_UNIT0_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE3_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site3_unit0(&self) -> u32 {
        (self.bits & CombPdSite3Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT0_MASK)
            >> CombPdSite3Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT0_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE3_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site3_unit0(&self) -> bool {
        (self.bits & CombPdSite3Unit0Vt2Conf1::MONITOR_EN_VT2_PD_SITE3_UNIT0_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE3_UNIT0** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site3_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT0_MASK)
                | ((value << CombPdSite3Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT0_OFFSET)
                    & CombPdSite3Unit0Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT0_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT0** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site3_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit0Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT0_MASK
            } else {
                self.bits & !CombPdSite3Unit0Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT0_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE3_UNIT0** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site3_unit0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit0Vt2Conf1::MONITOR_EN_VT2_PD_SITE3_UNIT0_MASK
            } else {
                self.bits & !CombPdSite3Unit0Vt2Conf1::MONITOR_EN_VT2_PD_SITE3_UNIT0_MASK
            },
        }
    }
}

impl Default for CombPdSite3Unit0Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite3Unit0Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit0Vt2Conf1Val {
        CombPdSite3Unit0Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit0Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit0Vt2Conf1Val) -> CombPdSite3Unit0Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT1_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x011c
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE3_UNIT1** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE3_UNIT1** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE3_UNIT1** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT1** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE3_UNIT1** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit1Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit1Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit1Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE3_UNIT1_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE3_UNIT1_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE3_UNIT1: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE3_UNIT1_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE3_UNIT1_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE3_UNIT1_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE3_UNIT1_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT1_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT1_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT1: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE3_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE3_UNIT1_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE3_UNIT1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE3_UNIT1** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site3_unit1(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE3_UNIT1_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE3_UNIT1** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site3_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE3_UNIT1_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE3_UNIT1_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE3_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site3_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT1_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT1_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE3_UNIT1** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site3_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT1_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT1_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT1_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT1** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site3_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT1_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE3_UNIT1** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site3_unit1(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE3_UNIT1_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE3_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site3_unit1(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE3_UNIT1_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE3_UNIT1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site3_unit1(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE3_UNIT1_MASK);
    }
}

/// COMB_PD_SITE3_UNIT1_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit1vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit1vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit1Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite3Unit1Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE3_UNIT1** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site3_unit1(&self) -> bool {
        (self.bits & CombPdSite3Unit1Vt2Conf1::TIMING_ERR_VT2_PD_SITE3_UNIT1_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE3_UNIT1** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site3_unit1(&self) -> u32 {
        (self.bits & CombPdSite3Unit1Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE3_UNIT1_MASK)
            >> CombPdSite3Unit1Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE3_UNIT1_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE3_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site3_unit1(&self) -> u32 {
        (self.bits & CombPdSite3Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT1_MASK)
            >> CombPdSite3Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT1_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE3_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site3_unit1(&self) -> bool {
        (self.bits & CombPdSite3Unit1Vt2Conf1::MONITOR_EN_VT2_PD_SITE3_UNIT1_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE3_UNIT1** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site3_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT1_MASK)
                | ((value << CombPdSite3Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT1_OFFSET)
                    & CombPdSite3Unit1Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT1_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT1** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site3_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit1Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT1_MASK
            } else {
                self.bits & !CombPdSite3Unit1Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT1_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE3_UNIT1** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site3_unit1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit1Vt2Conf1::MONITOR_EN_VT2_PD_SITE3_UNIT1_MASK
            } else {
                self.bits & !CombPdSite3Unit1Vt2Conf1::MONITOR_EN_VT2_PD_SITE3_UNIT1_MASK
            },
        }
    }
}

impl Default for CombPdSite3Unit1Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite3Unit1Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit1Vt2Conf1Val {
        CombPdSite3Unit1Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit1Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit1Vt2Conf1Val) -> CombPdSite3Unit1Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT2_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0120
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE3_UNIT2** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE3_UNIT2** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE3_UNIT2** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT2** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE3_UNIT2** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit2Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit2Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit2Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE3_UNIT2_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE3_UNIT2_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE3_UNIT2: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE3_UNIT2_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE3_UNIT2_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE3_UNIT2_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE3_UNIT2_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT2_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT2_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT2: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE3_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE3_UNIT2_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE3_UNIT2: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE3_UNIT2** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site3_unit2(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE3_UNIT2_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE3_UNIT2** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site3_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE3_UNIT2_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE3_UNIT2_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE3_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site3_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT2_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT2_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE3_UNIT2** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site3_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT2_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT2_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT2_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT2** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site3_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT2_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE3_UNIT2** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site3_unit2(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE3_UNIT2_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE3_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site3_unit2(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE3_UNIT2_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE3_UNIT2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site3_unit2(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE3_UNIT2_MASK);
    }
}

/// COMB_PD_SITE3_UNIT2_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit2vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit2vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit2Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite3Unit2Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE3_UNIT2** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site3_unit2(&self) -> bool {
        (self.bits & CombPdSite3Unit2Vt2Conf1::TIMING_ERR_VT2_PD_SITE3_UNIT2_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE3_UNIT2** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site3_unit2(&self) -> u32 {
        (self.bits & CombPdSite3Unit2Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE3_UNIT2_MASK)
            >> CombPdSite3Unit2Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE3_UNIT2_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE3_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site3_unit2(&self) -> u32 {
        (self.bits & CombPdSite3Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT2_MASK)
            >> CombPdSite3Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT2_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE3_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site3_unit2(&self) -> bool {
        (self.bits & CombPdSite3Unit2Vt2Conf1::MONITOR_EN_VT2_PD_SITE3_UNIT2_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE3_UNIT2** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site3_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT2_MASK)
                | ((value << CombPdSite3Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT2_OFFSET)
                    & CombPdSite3Unit2Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT2_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT2** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site3_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit2Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT2_MASK
            } else {
                self.bits & !CombPdSite3Unit2Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT2_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE3_UNIT2** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site3_unit2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit2Vt2Conf1::MONITOR_EN_VT2_PD_SITE3_UNIT2_MASK
            } else {
                self.bits & !CombPdSite3Unit2Vt2Conf1::MONITOR_EN_VT2_PD_SITE3_UNIT2_MASK
            },
        }
    }
}

impl Default for CombPdSite3Unit2Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite3Unit2Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit2Vt2Conf1Val {
        CombPdSite3Unit2Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit2Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit2Vt2Conf1Val) -> CombPdSite3Unit2Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT3_VT2_CONF1 Register
///
/// needs desc
///
/// Address offset: 0x0124
/// Reset value: 0x00000050
///
/// ## Fields
///
/// - **TIMING_ERR_VT2_PD_SITE3_UNIT3** `[31]` (RO)
///
///   needs field desc
///
/// - **DELAY_NUM_O_VT2_PD_SITE3_UNIT3** `[30:23]` (RO)
///
///   needs field desc
///
/// - **DELAY_LIMIT_VT2_PD_SITE3_UNIT3** `[9:2]` (RW)
///
///   needs field desc
///
/// - **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT3** `[1]` (WO)
///
///   needs field desc
///
/// - **MONITOR_EN_VT2_PD_SITE3_UNIT3** `[0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit3Vt2Conf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit3Vt2Conf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit3Vt2Conf1 {
    // ======== Field Constants ========

    /// **TIMING_ERR_VT2_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TIMING_ERR_VT2_PD_SITE3_UNIT3_OFFSET: u32 = 31;
    pub const TIMING_ERR_VT2_PD_SITE3_UNIT3_MASK: u32 = 0x80000000;
    pub const TIMING_ERR_VT2_PD_SITE3_UNIT3: u32 = 1 << 31;

    /// **DELAY_NUM_O_VT2_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[30:23]`, mask `0x7f800000`
    pub const DELAY_NUM_O_VT2_PD_SITE3_UNIT3_OFFSET: u32 = 23;
    pub const DELAY_NUM_O_VT2_PD_SITE3_UNIT3_MASK: u32 = 0x7f800000;

    /// **DELAY_LIMIT_VT2_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[9:2]`, mask `0x000003fc`
    pub const DELAY_LIMIT_VT2_PD_SITE3_UNIT3_OFFSET: u32 = 2;
    pub const DELAY_LIMIT_VT2_PD_SITE3_UNIT3_MASK: u32 = 0x000003fc;

    /// **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT3_OFFSET: u32 = 1;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT3_MASK: u32 = 0x00000002;
    pub const TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT3: u32 = 1 << 1;

    /// **MONITOR_EN_VT2_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MONITOR_EN_VT2_PD_SITE3_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EN_VT2_PD_SITE3_UNIT3_MASK: u32 = 0x00000001;
    pub const MONITOR_EN_VT2_PD_SITE3_UNIT3: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TIMING_ERR_VT2_PD_SITE3_UNIT3** `[31]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_vt2_pd_site3_unit3(&self) -> bool {
        (self.value.read_bits() & Self::TIMING_ERR_VT2_PD_SITE3_UNIT3_MASK) != 0
    }

    /// **DELAY_NUM_O_VT2_PD_SITE3_UNIT3** `[30:23]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site3_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_NUM_O_VT2_PD_SITE3_UNIT3_MASK)
            >> Self::DELAY_NUM_O_VT2_PD_SITE3_UNIT3_OFFSET
    }

    /// **DELAY_LIMIT_VT2_PD_SITE3_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site3_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT3_MASK)
            >> Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT3_OFFSET
    }

    /// Write **DELAY_LIMIT_VT2_PD_SITE3_UNIT3** `[9:2]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_delay_limit_vt2_pd_site3_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT3_MASK)
                | ((value << Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT3_OFFSET)
                    & Self::DELAY_LIMIT_VT2_PD_SITE3_UNIT3_MASK)
        });
    }

    /// Trigger **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT3** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_timing_err_cnt_clr_vt2_pd_site3_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT3_MASK);
    }

    /// **MONITOR_EN_VT2_PD_SITE3_UNIT3** `[0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site3_unit3(&self) -> bool {
        (self.value.read_bits() & Self::MONITOR_EN_VT2_PD_SITE3_UNIT3_MASK) != 0
    }

    /// Set MONITOR_EN_VT2_PD_SITE3_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_monitor_en_vt2_pd_site3_unit3(&self) {
        self.value
            .modify_bits(|r| r | Self::MONITOR_EN_VT2_PD_SITE3_UNIT3_MASK);
    }

    /// Clear MONITOR_EN_VT2_PD_SITE3_UNIT3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_monitor_en_vt2_pd_site3_unit3(&self) {
        self.value
            .modify_bits(|r| r & !Self::MONITOR_EN_VT2_PD_SITE3_UNIT3_MASK);
    }
}

/// COMB_PD_SITE3_UNIT3_VT2_CONF1 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit3vt2conf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit3vt2conf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit3Vt2Conf1Val {
    bits: u32,
}

impl CombPdSite3Unit3Vt2Conf1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000050 }
    }

    /// Read **TIMING_ERR_VT2_PD_SITE3_UNIT3** `[31:31]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_vt2_pd_site3_unit3(&self) -> bool {
        (self.bits & CombPdSite3Unit3Vt2Conf1::TIMING_ERR_VT2_PD_SITE3_UNIT3_MASK) != 0
    }

    /// Read **DELAY_NUM_O_VT2_PD_SITE3_UNIT3** `[30:23]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_num_o_vt2_pd_site3_unit3(&self) -> u32 {
        (self.bits & CombPdSite3Unit3Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE3_UNIT3_MASK)
            >> CombPdSite3Unit3Vt2Conf1::DELAY_NUM_O_VT2_PD_SITE3_UNIT3_OFFSET
    }

    /// Read **DELAY_LIMIT_VT2_PD_SITE3_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_limit_vt2_pd_site3_unit3(&self) -> u32 {
        (self.bits & CombPdSite3Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT3_MASK)
            >> CombPdSite3Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT3_OFFSET
    }

    /// Read **MONITOR_EN_VT2_PD_SITE3_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_en_vt2_pd_site3_unit3(&self) -> bool {
        (self.bits & CombPdSite3Unit3Vt2Conf1::MONITOR_EN_VT2_PD_SITE3_UNIT3_MASK) != 0
    }

    /// Set **DELAY_LIMIT_VT2_PD_SITE3_UNIT3** `[9:2]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_delay_limit_vt2_pd_site3_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT3_MASK)
                | ((value << CombPdSite3Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT3_OFFSET)
                    & CombPdSite3Unit3Vt2Conf1::DELAY_LIMIT_VT2_PD_SITE3_UNIT3_MASK),
        }
    }

    /// Set **TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT3** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_timing_err_cnt_clr_vt2_pd_site3_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit3Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT3_MASK
            } else {
                self.bits & !CombPdSite3Unit3Vt2Conf1::TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT3_MASK
            },
        }
    }

    /// Set **MONITOR_EN_VT2_PD_SITE3_UNIT3** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_en_vt2_pd_site3_unit3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CombPdSite3Unit3Vt2Conf1::MONITOR_EN_VT2_PD_SITE3_UNIT3_MASK
            } else {
                self.bits & !CombPdSite3Unit3Vt2Conf1::MONITOR_EN_VT2_PD_SITE3_UNIT3_MASK
            },
        }
    }
}

impl Default for CombPdSite3Unit3Vt2Conf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CombPdSite3Unit3Vt2Conf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit3Vt2Conf1Val {
        CombPdSite3Unit3Vt2Conf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit3Vt2Conf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit3Vt2Conf1Val) -> CombPdSite3Unit3Vt2Conf1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT0_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0128
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT0** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE0_UNIT0** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit0Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit0Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit0Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT0_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT0_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE0_UNIT0_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE0_UNIT0_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE0_UNIT0: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT0** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site0_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT0_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT0_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE0_UNIT0** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site0_unit0(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE0_UNIT0_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site0_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site0_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0_MASK)
        });
    }
}

/// COMB_PD_SITE0_UNIT0_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit0vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit0vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit0Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite0Unit0Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT0** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site0_unit0(&self) -> u32 {
        (self.bits & CombPdSite0Unit0Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT0_MASK)
            >> CombPdSite0Unit0Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT0_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE0_UNIT0** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site0_unit0(&self) -> bool {
        (self.bits & CombPdSite0Unit0Vt0Conf2::DELAY_OVF_VT0_PD_SITE0_UNIT0_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site0_unit0(&self) -> u32 {
        (self.bits & CombPdSite0Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0_MASK)
            >> CombPdSite0Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site0_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0_MASK)
                | ((value << CombPdSite0Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0_OFFSET)
                    & CombPdSite0Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT0_MASK),
        }
    }
}

impl Default for CombPdSite0Unit0Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite0Unit0Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit0Vt0Conf2Val {
        CombPdSite0Unit0Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit0Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit0Vt0Conf2Val) -> CombPdSite0Unit0Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT1_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x012c
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT1** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE0_UNIT1** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit1Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit1Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit1Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT1_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT1_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE0_UNIT1_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE0_UNIT1_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE0_UNIT1: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT1** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site0_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT1_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT1_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE0_UNIT1** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site0_unit1(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE0_UNIT1_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site0_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site0_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_MASK)
        });
    }
}

/// COMB_PD_SITE0_UNIT1_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit1vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit1vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit1Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite0Unit1Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT1** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site0_unit1(&self) -> u32 {
        (self.bits & CombPdSite0Unit1Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT1_MASK)
            >> CombPdSite0Unit1Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT1_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE0_UNIT1** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site0_unit1(&self) -> bool {
        (self.bits & CombPdSite0Unit1Vt0Conf2::DELAY_OVF_VT0_PD_SITE0_UNIT1_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site0_unit1(&self) -> u32 {
        (self.bits & CombPdSite0Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_MASK)
            >> CombPdSite0Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site0_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_MASK)
                | ((value << CombPdSite0Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_OFFSET)
                    & CombPdSite0Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT1_MASK),
        }
    }
}

impl Default for CombPdSite0Unit1Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite0Unit1Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit1Vt0Conf2Val {
        CombPdSite0Unit1Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit1Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit1Vt0Conf2Val) -> CombPdSite0Unit1Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT2_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0130
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT2** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE0_UNIT2** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit2Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit2Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit2Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT2_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT2_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE0_UNIT2_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE0_UNIT2_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE0_UNIT2: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT2** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site0_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT2_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT2_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE0_UNIT2** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site0_unit2(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE0_UNIT2_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site0_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site0_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2_MASK)
        });
    }
}

/// COMB_PD_SITE0_UNIT2_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit2vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit2vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit2Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite0Unit2Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT2** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site0_unit2(&self) -> u32 {
        (self.bits & CombPdSite0Unit2Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT2_MASK)
            >> CombPdSite0Unit2Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT2_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE0_UNIT2** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site0_unit2(&self) -> bool {
        (self.bits & CombPdSite0Unit2Vt0Conf2::DELAY_OVF_VT0_PD_SITE0_UNIT2_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site0_unit2(&self) -> u32 {
        (self.bits & CombPdSite0Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2_MASK)
            >> CombPdSite0Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site0_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2_MASK)
                | ((value << CombPdSite0Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2_OFFSET)
                    & CombPdSite0Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT2_MASK),
        }
    }
}

impl Default for CombPdSite0Unit2Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite0Unit2Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit2Vt0Conf2Val {
        CombPdSite0Unit2Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit2Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit2Vt0Conf2Val) -> CombPdSite0Unit2Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT3_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0134
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT3** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE0_UNIT3** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit3Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit3Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit3Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT3_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT3_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE0_UNIT3_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE0_UNIT3_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE0_UNIT3: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT3** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site0_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT3_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT3_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE0_UNIT3** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site0_unit3(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE0_UNIT3_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site0_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site0_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3_MASK)
        });
    }
}

/// COMB_PD_SITE0_UNIT3_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit3vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit3vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit3Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite0Unit3Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT3** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site0_unit3(&self) -> u32 {
        (self.bits & CombPdSite0Unit3Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT3_MASK)
            >> CombPdSite0Unit3Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE0_UNIT3_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE0_UNIT3** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site0_unit3(&self) -> bool {
        (self.bits & CombPdSite0Unit3Vt0Conf2::DELAY_OVF_VT0_PD_SITE0_UNIT3_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site0_unit3(&self) -> u32 {
        (self.bits & CombPdSite0Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3_MASK)
            >> CombPdSite0Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site0_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3_MASK)
                | ((value << CombPdSite0Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3_OFFSET)
                    & CombPdSite0Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE0_UNIT3_MASK),
        }
    }
}

impl Default for CombPdSite0Unit3Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite0Unit3Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit3Vt0Conf2Val {
        CombPdSite0Unit3Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit3Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit3Vt0Conf2Val) -> CombPdSite0Unit3Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT0_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0138
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT0** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE0_UNIT0** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit0Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit0Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit0Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT0_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT0_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE0_UNIT0_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE0_UNIT0_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE0_UNIT0: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT0** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site0_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT0_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT0_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE0_UNIT0** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site0_unit0(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE0_UNIT0_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site0_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site0_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0_MASK)
        });
    }
}

/// COMB_PD_SITE0_UNIT0_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit0vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit0vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit0Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite0Unit0Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT0** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site0_unit0(&self) -> u32 {
        (self.bits & CombPdSite0Unit0Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT0_MASK)
            >> CombPdSite0Unit0Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT0_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE0_UNIT0** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site0_unit0(&self) -> bool {
        (self.bits & CombPdSite0Unit0Vt1Conf2::DELAY_OVF_VT1_PD_SITE0_UNIT0_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site0_unit0(&self) -> u32 {
        (self.bits & CombPdSite0Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0_MASK)
            >> CombPdSite0Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site0_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0_MASK)
                | ((value << CombPdSite0Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0_OFFSET)
                    & CombPdSite0Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT0_MASK),
        }
    }
}

impl Default for CombPdSite0Unit0Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite0Unit0Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit0Vt1Conf2Val {
        CombPdSite0Unit0Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit0Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit0Vt1Conf2Val) -> CombPdSite0Unit0Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT1_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x013c
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT1** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE0_UNIT1** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit1Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit1Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit1Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT1_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT1_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE0_UNIT1_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE0_UNIT1_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE0_UNIT1: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT1** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site0_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT1_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT1_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE0_UNIT1** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site0_unit1(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE0_UNIT1_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site0_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site0_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1_MASK)
        });
    }
}

/// COMB_PD_SITE0_UNIT1_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit1vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit1vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit1Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite0Unit1Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT1** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site0_unit1(&self) -> u32 {
        (self.bits & CombPdSite0Unit1Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT1_MASK)
            >> CombPdSite0Unit1Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT1_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE0_UNIT1** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site0_unit1(&self) -> bool {
        (self.bits & CombPdSite0Unit1Vt1Conf2::DELAY_OVF_VT1_PD_SITE0_UNIT1_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site0_unit1(&self) -> u32 {
        (self.bits & CombPdSite0Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1_MASK)
            >> CombPdSite0Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site0_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1_MASK)
                | ((value << CombPdSite0Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1_OFFSET)
                    & CombPdSite0Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT1_MASK),
        }
    }
}

impl Default for CombPdSite0Unit1Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite0Unit1Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit1Vt1Conf2Val {
        CombPdSite0Unit1Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit1Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit1Vt1Conf2Val) -> CombPdSite0Unit1Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT2_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0140
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT2** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE0_UNIT2** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit2Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit2Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit2Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT2_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT2_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE0_UNIT2_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE0_UNIT2_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE0_UNIT2: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT2** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site0_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT2_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT2_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE0_UNIT2** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site0_unit2(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE0_UNIT2_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site0_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site0_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2_MASK)
        });
    }
}

/// COMB_PD_SITE0_UNIT2_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit2vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit2vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit2Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite0Unit2Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT2** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site0_unit2(&self) -> u32 {
        (self.bits & CombPdSite0Unit2Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT2_MASK)
            >> CombPdSite0Unit2Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT2_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE0_UNIT2** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site0_unit2(&self) -> bool {
        (self.bits & CombPdSite0Unit2Vt1Conf2::DELAY_OVF_VT1_PD_SITE0_UNIT2_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site0_unit2(&self) -> u32 {
        (self.bits & CombPdSite0Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2_MASK)
            >> CombPdSite0Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site0_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2_MASK)
                | ((value << CombPdSite0Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2_OFFSET)
                    & CombPdSite0Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT2_MASK),
        }
    }
}

impl Default for CombPdSite0Unit2Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite0Unit2Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit2Vt1Conf2Val {
        CombPdSite0Unit2Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit2Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit2Vt1Conf2Val) -> CombPdSite0Unit2Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT3_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0144
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT3** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE0_UNIT3** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit3Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit3Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit3Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT3_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT3_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE0_UNIT3_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE0_UNIT3_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE0_UNIT3: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT3** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site0_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT3_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT3_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE0_UNIT3** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site0_unit3(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE0_UNIT3_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site0_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site0_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3_MASK)
        });
    }
}

/// COMB_PD_SITE0_UNIT3_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit3vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit3vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit3Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite0Unit3Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT3** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site0_unit3(&self) -> u32 {
        (self.bits & CombPdSite0Unit3Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT3_MASK)
            >> CombPdSite0Unit3Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE0_UNIT3_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE0_UNIT3** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site0_unit3(&self) -> bool {
        (self.bits & CombPdSite0Unit3Vt1Conf2::DELAY_OVF_VT1_PD_SITE0_UNIT3_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site0_unit3(&self) -> u32 {
        (self.bits & CombPdSite0Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3_MASK)
            >> CombPdSite0Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site0_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3_MASK)
                | ((value << CombPdSite0Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3_OFFSET)
                    & CombPdSite0Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE0_UNIT3_MASK),
        }
    }
}

impl Default for CombPdSite0Unit3Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite0Unit3Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit3Vt1Conf2Val {
        CombPdSite0Unit3Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit3Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit3Vt1Conf2Val) -> CombPdSite0Unit3Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT0_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0148
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT0** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE0_UNIT0** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit0Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit0Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit0Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT0_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT0_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE0_UNIT0_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE0_UNIT0_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE0_UNIT0: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT0** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site0_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT0_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT0_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE0_UNIT0** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site0_unit0(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE0_UNIT0_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site0_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site0_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0_MASK)
        });
    }
}

/// COMB_PD_SITE0_UNIT0_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit0vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit0vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit0Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite0Unit0Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT0** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site0_unit0(&self) -> u32 {
        (self.bits & CombPdSite0Unit0Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT0_MASK)
            >> CombPdSite0Unit0Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT0_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE0_UNIT0** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site0_unit0(&self) -> bool {
        (self.bits & CombPdSite0Unit0Vt2Conf2::DELAY_OVF_VT2_PD_SITE0_UNIT0_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site0_unit0(&self) -> u32 {
        (self.bits & CombPdSite0Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0_MASK)
            >> CombPdSite0Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site0_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0_MASK)
                | ((value << CombPdSite0Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0_OFFSET)
                    & CombPdSite0Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0_MASK),
        }
    }
}

impl Default for CombPdSite0Unit0Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite0Unit0Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit0Vt2Conf2Val {
        CombPdSite0Unit0Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit0Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit0Vt2Conf2Val) -> CombPdSite0Unit0Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT1_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x014c
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT1** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE0_UNIT1** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit1Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit1Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit1Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT1_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT1_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE0_UNIT1_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE0_UNIT1_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE0_UNIT1: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT1** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site0_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT1_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT1_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE0_UNIT1** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site0_unit1(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE0_UNIT1_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site0_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site0_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1_MASK)
        });
    }
}

/// COMB_PD_SITE0_UNIT1_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit1vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit1vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit1Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite0Unit1Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT1** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site0_unit1(&self) -> u32 {
        (self.bits & CombPdSite0Unit1Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT1_MASK)
            >> CombPdSite0Unit1Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT1_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE0_UNIT1** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site0_unit1(&self) -> bool {
        (self.bits & CombPdSite0Unit1Vt2Conf2::DELAY_OVF_VT2_PD_SITE0_UNIT1_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site0_unit1(&self) -> u32 {
        (self.bits & CombPdSite0Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1_MASK)
            >> CombPdSite0Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site0_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1_MASK)
                | ((value << CombPdSite0Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1_OFFSET)
                    & CombPdSite0Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT1_MASK),
        }
    }
}

impl Default for CombPdSite0Unit1Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite0Unit1Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit1Vt2Conf2Val {
        CombPdSite0Unit1Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit1Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit1Vt2Conf2Val) -> CombPdSite0Unit1Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT2_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0150
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT2** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE0_UNIT2** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit2Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit2Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit2Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT2_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT2_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE0_UNIT2_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE0_UNIT2_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE0_UNIT2: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT2** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site0_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT2_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT2_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE0_UNIT2** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site0_unit2(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE0_UNIT2_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site0_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site0_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2_MASK)
        });
    }
}

/// COMB_PD_SITE0_UNIT2_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit2vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit2vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit2Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite0Unit2Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT2** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site0_unit2(&self) -> u32 {
        (self.bits & CombPdSite0Unit2Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT2_MASK)
            >> CombPdSite0Unit2Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT2_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE0_UNIT2** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site0_unit2(&self) -> bool {
        (self.bits & CombPdSite0Unit2Vt2Conf2::DELAY_OVF_VT2_PD_SITE0_UNIT2_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site0_unit2(&self) -> u32 {
        (self.bits & CombPdSite0Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2_MASK)
            >> CombPdSite0Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site0_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2_MASK)
                | ((value << CombPdSite0Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2_OFFSET)
                    & CombPdSite0Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT2_MASK),
        }
    }
}

impl Default for CombPdSite0Unit2Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite0Unit2Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit2Vt2Conf2Val {
        CombPdSite0Unit2Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit2Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit2Vt2Conf2Val) -> CombPdSite0Unit2Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE0_UNIT3_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0154
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT3** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE0_UNIT3** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite0Unit3Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite0Unit3Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite0Unit3Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT3_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT3_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE0_UNIT3_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE0_UNIT3_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE0_UNIT3: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT3** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site0_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT3_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT3_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE0_UNIT3** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site0_unit3(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE0_UNIT3_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site0_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site0_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_MASK)
        });
    }
}

/// COMB_PD_SITE0_UNIT3_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite0unit3vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite0unit3vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite0Unit3Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite0Unit3Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT3** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site0_unit3(&self) -> u32 {
        (self.bits & CombPdSite0Unit3Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT3_MASK)
            >> CombPdSite0Unit3Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT3_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE0_UNIT3** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site0_unit3(&self) -> bool {
        (self.bits & CombPdSite0Unit3Vt2Conf2::DELAY_OVF_VT2_PD_SITE0_UNIT3_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site0_unit3(&self) -> u32 {
        (self.bits & CombPdSite0Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_MASK)
            >> CombPdSite0Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site0_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite0Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_MASK)
                | ((value << CombPdSite0Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_OFFSET)
                    & CombPdSite0Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT3_MASK),
        }
    }
}

impl Default for CombPdSite0Unit3Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite0Unit3Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite0Unit3Vt2Conf2Val {
        CombPdSite0Unit3Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite0Unit3Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite0Unit3Vt2Conf2Val) -> CombPdSite0Unit3Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT0_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0158
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT0** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE1_UNIT0** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit0Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit0Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit0Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT0_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT0_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE1_UNIT0_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE1_UNIT0_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE1_UNIT0: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT0** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site1_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT0_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT0_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE1_UNIT0** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site1_unit0(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE1_UNIT0_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site1_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site1_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0_MASK)
        });
    }
}

/// COMB_PD_SITE1_UNIT0_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit0vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit0vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit0Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite1Unit0Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT0** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site1_unit0(&self) -> u32 {
        (self.bits & CombPdSite1Unit0Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT0_MASK)
            >> CombPdSite1Unit0Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT0_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE1_UNIT0** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site1_unit0(&self) -> bool {
        (self.bits & CombPdSite1Unit0Vt0Conf2::DELAY_OVF_VT0_PD_SITE1_UNIT0_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site1_unit0(&self) -> u32 {
        (self.bits & CombPdSite1Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0_MASK)
            >> CombPdSite1Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site1_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0_MASK)
                | ((value << CombPdSite1Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0_OFFSET)
                    & CombPdSite1Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0_MASK),
        }
    }
}

impl Default for CombPdSite1Unit0Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite1Unit0Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit0Vt0Conf2Val {
        CombPdSite1Unit0Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit0Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit0Vt0Conf2Val) -> CombPdSite1Unit0Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT1_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x015c
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT1** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE1_UNIT1** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit1Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit1Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit1Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT1_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT1_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE1_UNIT1_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE1_UNIT1_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE1_UNIT1: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT1** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site1_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT1_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT1_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE1_UNIT1** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site1_unit1(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE1_UNIT1_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site1_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site1_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1_MASK)
        });
    }
}

/// COMB_PD_SITE1_UNIT1_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit1vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit1vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit1Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite1Unit1Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT1** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site1_unit1(&self) -> u32 {
        (self.bits & CombPdSite1Unit1Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT1_MASK)
            >> CombPdSite1Unit1Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT1_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE1_UNIT1** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site1_unit1(&self) -> bool {
        (self.bits & CombPdSite1Unit1Vt0Conf2::DELAY_OVF_VT0_PD_SITE1_UNIT1_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site1_unit1(&self) -> u32 {
        (self.bits & CombPdSite1Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1_MASK)
            >> CombPdSite1Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site1_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1_MASK)
                | ((value << CombPdSite1Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1_OFFSET)
                    & CombPdSite1Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT1_MASK),
        }
    }
}

impl Default for CombPdSite1Unit1Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite1Unit1Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit1Vt0Conf2Val {
        CombPdSite1Unit1Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit1Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit1Vt0Conf2Val) -> CombPdSite1Unit1Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT2_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0160
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT2** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE1_UNIT2** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit2Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit2Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit2Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT2_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT2_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE1_UNIT2_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE1_UNIT2_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE1_UNIT2: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT2** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site1_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT2_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT2_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE1_UNIT2** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site1_unit2(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE1_UNIT2_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site1_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site1_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2_MASK)
        });
    }
}

/// COMB_PD_SITE1_UNIT2_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit2vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit2vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit2Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite1Unit2Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT2** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site1_unit2(&self) -> u32 {
        (self.bits & CombPdSite1Unit2Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT2_MASK)
            >> CombPdSite1Unit2Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT2_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE1_UNIT2** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site1_unit2(&self) -> bool {
        (self.bits & CombPdSite1Unit2Vt0Conf2::DELAY_OVF_VT0_PD_SITE1_UNIT2_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site1_unit2(&self) -> u32 {
        (self.bits & CombPdSite1Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2_MASK)
            >> CombPdSite1Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site1_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2_MASK)
                | ((value << CombPdSite1Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2_OFFSET)
                    & CombPdSite1Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT2_MASK),
        }
    }
}

impl Default for CombPdSite1Unit2Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite1Unit2Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit2Vt0Conf2Val {
        CombPdSite1Unit2Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit2Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit2Vt0Conf2Val) -> CombPdSite1Unit2Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT3_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0164
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT3** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE1_UNIT3** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit3Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit3Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit3Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT3_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT3_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE1_UNIT3_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE1_UNIT3_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE1_UNIT3: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT3** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site1_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT3_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT3_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE1_UNIT3** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site1_unit3(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE1_UNIT3_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site1_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site1_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_MASK)
        });
    }
}

/// COMB_PD_SITE1_UNIT3_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit3vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit3vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit3Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite1Unit3Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT3** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site1_unit3(&self) -> u32 {
        (self.bits & CombPdSite1Unit3Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT3_MASK)
            >> CombPdSite1Unit3Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT3_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE1_UNIT3** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site1_unit3(&self) -> bool {
        (self.bits & CombPdSite1Unit3Vt0Conf2::DELAY_OVF_VT0_PD_SITE1_UNIT3_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site1_unit3(&self) -> u32 {
        (self.bits & CombPdSite1Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_MASK)
            >> CombPdSite1Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site1_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_MASK)
                | ((value << CombPdSite1Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_OFFSET)
                    & CombPdSite1Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT3_MASK),
        }
    }
}

impl Default for CombPdSite1Unit3Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite1Unit3Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit3Vt0Conf2Val {
        CombPdSite1Unit3Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit3Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit3Vt0Conf2Val) -> CombPdSite1Unit3Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT0_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0168
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT0** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE1_UNIT0** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit0Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit0Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit0Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT0_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT0_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE1_UNIT0_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE1_UNIT0_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE1_UNIT0: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT0** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site1_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT0_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT0_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE1_UNIT0** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site1_unit0(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE1_UNIT0_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site1_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site1_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0_MASK)
        });
    }
}

/// COMB_PD_SITE1_UNIT0_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit0vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit0vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit0Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite1Unit0Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT0** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site1_unit0(&self) -> u32 {
        (self.bits & CombPdSite1Unit0Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT0_MASK)
            >> CombPdSite1Unit0Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT0_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE1_UNIT0** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site1_unit0(&self) -> bool {
        (self.bits & CombPdSite1Unit0Vt1Conf2::DELAY_OVF_VT1_PD_SITE1_UNIT0_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site1_unit0(&self) -> u32 {
        (self.bits & CombPdSite1Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0_MASK)
            >> CombPdSite1Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site1_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0_MASK)
                | ((value << CombPdSite1Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0_OFFSET)
                    & CombPdSite1Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT0_MASK),
        }
    }
}

impl Default for CombPdSite1Unit0Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite1Unit0Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit0Vt1Conf2Val {
        CombPdSite1Unit0Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit0Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit0Vt1Conf2Val) -> CombPdSite1Unit0Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT1_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x016c
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT1** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE1_UNIT1** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit1Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit1Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit1Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT1_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT1_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE1_UNIT1_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE1_UNIT1_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE1_UNIT1: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT1** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site1_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT1_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT1_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE1_UNIT1** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site1_unit1(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE1_UNIT1_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site1_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site1_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1_MASK)
        });
    }
}

/// COMB_PD_SITE1_UNIT1_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit1vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit1vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit1Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite1Unit1Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT1** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site1_unit1(&self) -> u32 {
        (self.bits & CombPdSite1Unit1Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT1_MASK)
            >> CombPdSite1Unit1Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT1_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE1_UNIT1** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site1_unit1(&self) -> bool {
        (self.bits & CombPdSite1Unit1Vt1Conf2::DELAY_OVF_VT1_PD_SITE1_UNIT1_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site1_unit1(&self) -> u32 {
        (self.bits & CombPdSite1Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1_MASK)
            >> CombPdSite1Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site1_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1_MASK)
                | ((value << CombPdSite1Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1_OFFSET)
                    & CombPdSite1Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT1_MASK),
        }
    }
}

impl Default for CombPdSite1Unit1Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite1Unit1Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit1Vt1Conf2Val {
        CombPdSite1Unit1Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit1Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit1Vt1Conf2Val) -> CombPdSite1Unit1Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT2_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0170
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT2** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE1_UNIT2** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit2Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit2Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit2Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT2_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT2_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE1_UNIT2_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE1_UNIT2_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE1_UNIT2: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT2** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site1_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT2_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT2_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE1_UNIT2** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site1_unit2(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE1_UNIT2_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site1_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site1_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2_MASK)
        });
    }
}

/// COMB_PD_SITE1_UNIT2_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit2vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit2vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit2Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite1Unit2Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT2** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site1_unit2(&self) -> u32 {
        (self.bits & CombPdSite1Unit2Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT2_MASK)
            >> CombPdSite1Unit2Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT2_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE1_UNIT2** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site1_unit2(&self) -> bool {
        (self.bits & CombPdSite1Unit2Vt1Conf2::DELAY_OVF_VT1_PD_SITE1_UNIT2_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site1_unit2(&self) -> u32 {
        (self.bits & CombPdSite1Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2_MASK)
            >> CombPdSite1Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site1_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2_MASK)
                | ((value << CombPdSite1Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2_OFFSET)
                    & CombPdSite1Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT2_MASK),
        }
    }
}

impl Default for CombPdSite1Unit2Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite1Unit2Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit2Vt1Conf2Val {
        CombPdSite1Unit2Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit2Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit2Vt1Conf2Val) -> CombPdSite1Unit2Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT3_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0174
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT3** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE1_UNIT3** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit3Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit3Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit3Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT3_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT3_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE1_UNIT3_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE1_UNIT3_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE1_UNIT3: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT3** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site1_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT3_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT3_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE1_UNIT3** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site1_unit3(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE1_UNIT3_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site1_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site1_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3_MASK)
        });
    }
}

/// COMB_PD_SITE1_UNIT3_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit3vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit3vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit3Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite1Unit3Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT3** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site1_unit3(&self) -> u32 {
        (self.bits & CombPdSite1Unit3Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT3_MASK)
            >> CombPdSite1Unit3Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE1_UNIT3_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE1_UNIT3** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site1_unit3(&self) -> bool {
        (self.bits & CombPdSite1Unit3Vt1Conf2::DELAY_OVF_VT1_PD_SITE1_UNIT3_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site1_unit3(&self) -> u32 {
        (self.bits & CombPdSite1Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3_MASK)
            >> CombPdSite1Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site1_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3_MASK)
                | ((value << CombPdSite1Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3_OFFSET)
                    & CombPdSite1Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE1_UNIT3_MASK),
        }
    }
}

impl Default for CombPdSite1Unit3Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite1Unit3Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit3Vt1Conf2Val {
        CombPdSite1Unit3Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit3Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit3Vt1Conf2Val) -> CombPdSite1Unit3Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT0_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0178
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT0** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE1_UNIT0** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit0Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit0Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit0Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT0_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT0_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE1_UNIT0_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE1_UNIT0_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE1_UNIT0: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT0** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site1_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT0_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT0_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE1_UNIT0** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site1_unit0(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE1_UNIT0_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site1_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site1_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0_MASK)
        });
    }
}

/// COMB_PD_SITE1_UNIT0_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit0vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit0vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit0Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite1Unit0Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT0** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site1_unit0(&self) -> u32 {
        (self.bits & CombPdSite1Unit0Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT0_MASK)
            >> CombPdSite1Unit0Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT0_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE1_UNIT0** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site1_unit0(&self) -> bool {
        (self.bits & CombPdSite1Unit0Vt2Conf2::DELAY_OVF_VT2_PD_SITE1_UNIT0_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site1_unit0(&self) -> u32 {
        (self.bits & CombPdSite1Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0_MASK)
            >> CombPdSite1Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site1_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0_MASK)
                | ((value << CombPdSite1Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0_OFFSET)
                    & CombPdSite1Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT0_MASK),
        }
    }
}

impl Default for CombPdSite1Unit0Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite1Unit0Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit0Vt2Conf2Val {
        CombPdSite1Unit0Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit0Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit0Vt2Conf2Val) -> CombPdSite1Unit0Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT1_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x017c
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT1** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE1_UNIT1** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit1Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit1Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit1Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT1_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT1_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE1_UNIT1_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE1_UNIT1_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE1_UNIT1: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT1** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site1_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT1_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT1_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE1_UNIT1** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site1_unit1(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE1_UNIT1_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site1_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site1_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1_MASK)
        });
    }
}

/// COMB_PD_SITE1_UNIT1_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit1vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit1vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit1Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite1Unit1Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT1** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site1_unit1(&self) -> u32 {
        (self.bits & CombPdSite1Unit1Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT1_MASK)
            >> CombPdSite1Unit1Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT1_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE1_UNIT1** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site1_unit1(&self) -> bool {
        (self.bits & CombPdSite1Unit1Vt2Conf2::DELAY_OVF_VT2_PD_SITE1_UNIT1_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site1_unit1(&self) -> u32 {
        (self.bits & CombPdSite1Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1_MASK)
            >> CombPdSite1Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site1_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1_MASK)
                | ((value << CombPdSite1Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1_OFFSET)
                    & CombPdSite1Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT1_MASK),
        }
    }
}

impl Default for CombPdSite1Unit1Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite1Unit1Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit1Vt2Conf2Val {
        CombPdSite1Unit1Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit1Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit1Vt2Conf2Val) -> CombPdSite1Unit1Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT2_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0180
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT2** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE1_UNIT2** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit2Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit2Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit2Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT2_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT2_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE1_UNIT2_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE1_UNIT2_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE1_UNIT2: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT2** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site1_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT2_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT2_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE1_UNIT2** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site1_unit2(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE1_UNIT2_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site1_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site1_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_MASK)
        });
    }
}

/// COMB_PD_SITE1_UNIT2_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit2vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit2vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit2Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite1Unit2Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT2** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site1_unit2(&self) -> u32 {
        (self.bits & CombPdSite1Unit2Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT2_MASK)
            >> CombPdSite1Unit2Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT2_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE1_UNIT2** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site1_unit2(&self) -> bool {
        (self.bits & CombPdSite1Unit2Vt2Conf2::DELAY_OVF_VT2_PD_SITE1_UNIT2_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site1_unit2(&self) -> u32 {
        (self.bits & CombPdSite1Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_MASK)
            >> CombPdSite1Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site1_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_MASK)
                | ((value << CombPdSite1Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_OFFSET)
                    & CombPdSite1Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT2_MASK),
        }
    }
}

impl Default for CombPdSite1Unit2Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite1Unit2Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit2Vt2Conf2Val {
        CombPdSite1Unit2Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit2Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit2Vt2Conf2Val) -> CombPdSite1Unit2Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE1_UNIT3_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0184
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT3** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE1_UNIT3** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite1Unit3Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite1Unit3Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite1Unit3Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT3_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT3_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE1_UNIT3_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE1_UNIT3_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE1_UNIT3: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT3** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site1_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT3_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT3_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE1_UNIT3** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site1_unit3(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE1_UNIT3_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site1_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site1_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3_MASK)
        });
    }
}

/// COMB_PD_SITE1_UNIT3_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite1unit3vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite1unit3vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite1Unit3Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite1Unit3Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT3** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site1_unit3(&self) -> u32 {
        (self.bits & CombPdSite1Unit3Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT3_MASK)
            >> CombPdSite1Unit3Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE1_UNIT3_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE1_UNIT3** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site1_unit3(&self) -> bool {
        (self.bits & CombPdSite1Unit3Vt2Conf2::DELAY_OVF_VT2_PD_SITE1_UNIT3_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site1_unit3(&self) -> u32 {
        (self.bits & CombPdSite1Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3_MASK)
            >> CombPdSite1Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site1_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite1Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3_MASK)
                | ((value << CombPdSite1Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3_OFFSET)
                    & CombPdSite1Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE1_UNIT3_MASK),
        }
    }
}

impl Default for CombPdSite1Unit3Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite1Unit3Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite1Unit3Vt2Conf2Val {
        CombPdSite1Unit3Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite1Unit3Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite1Unit3Vt2Conf2Val) -> CombPdSite1Unit3Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT0_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0188
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT0** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE2_UNIT0** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit0Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit0Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit0Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT0_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT0_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE2_UNIT0_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE2_UNIT0_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE2_UNIT0: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT0** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site2_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT0_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT0_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE2_UNIT0** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site2_unit0(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE2_UNIT0_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site2_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site2_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0_MASK)
        });
    }
}

/// COMB_PD_SITE2_UNIT0_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit0vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit0vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit0Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite2Unit0Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT0** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site2_unit0(&self) -> u32 {
        (self.bits & CombPdSite2Unit0Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT0_MASK)
            >> CombPdSite2Unit0Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT0_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE2_UNIT0** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site2_unit0(&self) -> bool {
        (self.bits & CombPdSite2Unit0Vt0Conf2::DELAY_OVF_VT0_PD_SITE2_UNIT0_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site2_unit0(&self) -> u32 {
        (self.bits & CombPdSite2Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0_MASK)
            >> CombPdSite2Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site2_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0_MASK)
                | ((value << CombPdSite2Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0_OFFSET)
                    & CombPdSite2Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT0_MASK),
        }
    }
}

impl Default for CombPdSite2Unit0Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite2Unit0Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit0Vt0Conf2Val {
        CombPdSite2Unit0Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit0Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit0Vt0Conf2Val) -> CombPdSite2Unit0Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT1_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x018c
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT1** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE2_UNIT1** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit1Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit1Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit1Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT1_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT1_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE2_UNIT1_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE2_UNIT1_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE2_UNIT1: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT1** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site2_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT1_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT1_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE2_UNIT1** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site2_unit1(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE2_UNIT1_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site2_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site2_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1_MASK)
        });
    }
}

/// COMB_PD_SITE2_UNIT1_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit1vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit1vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit1Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite2Unit1Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT1** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site2_unit1(&self) -> u32 {
        (self.bits & CombPdSite2Unit1Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT1_MASK)
            >> CombPdSite2Unit1Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT1_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE2_UNIT1** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site2_unit1(&self) -> bool {
        (self.bits & CombPdSite2Unit1Vt0Conf2::DELAY_OVF_VT0_PD_SITE2_UNIT1_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site2_unit1(&self) -> u32 {
        (self.bits & CombPdSite2Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1_MASK)
            >> CombPdSite2Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site2_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1_MASK)
                | ((value << CombPdSite2Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1_OFFSET)
                    & CombPdSite2Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT1_MASK),
        }
    }
}

impl Default for CombPdSite2Unit1Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite2Unit1Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit1Vt0Conf2Val {
        CombPdSite2Unit1Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit1Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit1Vt0Conf2Val) -> CombPdSite2Unit1Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT2_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0190
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT2** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE2_UNIT2** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit2Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit2Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit2Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT2_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT2_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE2_UNIT2_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE2_UNIT2_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE2_UNIT2: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT2** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site2_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT2_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT2_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE2_UNIT2** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site2_unit2(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE2_UNIT2_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site2_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site2_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2_MASK)
        });
    }
}

/// COMB_PD_SITE2_UNIT2_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit2vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit2vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit2Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite2Unit2Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT2** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site2_unit2(&self) -> u32 {
        (self.bits & CombPdSite2Unit2Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT2_MASK)
            >> CombPdSite2Unit2Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT2_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE2_UNIT2** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site2_unit2(&self) -> bool {
        (self.bits & CombPdSite2Unit2Vt0Conf2::DELAY_OVF_VT0_PD_SITE2_UNIT2_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site2_unit2(&self) -> u32 {
        (self.bits & CombPdSite2Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2_MASK)
            >> CombPdSite2Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site2_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2_MASK)
                | ((value << CombPdSite2Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2_OFFSET)
                    & CombPdSite2Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT2_MASK),
        }
    }
}

impl Default for CombPdSite2Unit2Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite2Unit2Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit2Vt0Conf2Val {
        CombPdSite2Unit2Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit2Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit2Vt0Conf2Val) -> CombPdSite2Unit2Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT3_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0194
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT3** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE2_UNIT3** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit3Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit3Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit3Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT3_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT3_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE2_UNIT3_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE2_UNIT3_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE2_UNIT3: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT3** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site2_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT3_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT3_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE2_UNIT3** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site2_unit3(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE2_UNIT3_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site2_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site2_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3_MASK)
        });
    }
}

/// COMB_PD_SITE2_UNIT3_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit3vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit3vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit3Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite2Unit3Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT3** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site2_unit3(&self) -> u32 {
        (self.bits & CombPdSite2Unit3Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT3_MASK)
            >> CombPdSite2Unit3Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE2_UNIT3_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE2_UNIT3** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site2_unit3(&self) -> bool {
        (self.bits & CombPdSite2Unit3Vt0Conf2::DELAY_OVF_VT0_PD_SITE2_UNIT3_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site2_unit3(&self) -> u32 {
        (self.bits & CombPdSite2Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3_MASK)
            >> CombPdSite2Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site2_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3_MASK)
                | ((value << CombPdSite2Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3_OFFSET)
                    & CombPdSite2Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE2_UNIT3_MASK),
        }
    }
}

impl Default for CombPdSite2Unit3Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite2Unit3Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit3Vt0Conf2Val {
        CombPdSite2Unit3Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit3Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit3Vt0Conf2Val) -> CombPdSite2Unit3Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT0_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x0198
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT0** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE2_UNIT0** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit0Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit0Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit0Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT0_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT0_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE2_UNIT0_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE2_UNIT0_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE2_UNIT0: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT0** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site2_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT0_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT0_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE2_UNIT0** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site2_unit0(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE2_UNIT0_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site2_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site2_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0_MASK)
        });
    }
}

/// COMB_PD_SITE2_UNIT0_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit0vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit0vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit0Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite2Unit0Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT0** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site2_unit0(&self) -> u32 {
        (self.bits & CombPdSite2Unit0Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT0_MASK)
            >> CombPdSite2Unit0Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT0_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE2_UNIT0** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site2_unit0(&self) -> bool {
        (self.bits & CombPdSite2Unit0Vt1Conf2::DELAY_OVF_VT1_PD_SITE2_UNIT0_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site2_unit0(&self) -> u32 {
        (self.bits & CombPdSite2Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0_MASK)
            >> CombPdSite2Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site2_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0_MASK)
                | ((value << CombPdSite2Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0_OFFSET)
                    & CombPdSite2Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT0_MASK),
        }
    }
}

impl Default for CombPdSite2Unit0Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite2Unit0Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit0Vt1Conf2Val {
        CombPdSite2Unit0Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit0Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit0Vt1Conf2Val) -> CombPdSite2Unit0Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT1_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x019c
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT1** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE2_UNIT1** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit1Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit1Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit1Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT1_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT1_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE2_UNIT1_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE2_UNIT1_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE2_UNIT1: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT1** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site2_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT1_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT1_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE2_UNIT1** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site2_unit1(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE2_UNIT1_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site2_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site2_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1_MASK)
        });
    }
}

/// COMB_PD_SITE2_UNIT1_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit1vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit1vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit1Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite2Unit1Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT1** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site2_unit1(&self) -> u32 {
        (self.bits & CombPdSite2Unit1Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT1_MASK)
            >> CombPdSite2Unit1Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT1_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE2_UNIT1** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site2_unit1(&self) -> bool {
        (self.bits & CombPdSite2Unit1Vt1Conf2::DELAY_OVF_VT1_PD_SITE2_UNIT1_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site2_unit1(&self) -> u32 {
        (self.bits & CombPdSite2Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1_MASK)
            >> CombPdSite2Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site2_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1_MASK)
                | ((value << CombPdSite2Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1_OFFSET)
                    & CombPdSite2Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT1_MASK),
        }
    }
}

impl Default for CombPdSite2Unit1Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite2Unit1Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit1Vt1Conf2Val {
        CombPdSite2Unit1Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit1Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit1Vt1Conf2Val) -> CombPdSite2Unit1Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT2_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01a0
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT2** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE2_UNIT2** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit2Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit2Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit2Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT2_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT2_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE2_UNIT2_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE2_UNIT2_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE2_UNIT2: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT2** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site2_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT2_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT2_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE2_UNIT2** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site2_unit2(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE2_UNIT2_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site2_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site2_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2_MASK)
        });
    }
}

/// COMB_PD_SITE2_UNIT2_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit2vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit2vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit2Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite2Unit2Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT2** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site2_unit2(&self) -> u32 {
        (self.bits & CombPdSite2Unit2Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT2_MASK)
            >> CombPdSite2Unit2Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT2_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE2_UNIT2** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site2_unit2(&self) -> bool {
        (self.bits & CombPdSite2Unit2Vt1Conf2::DELAY_OVF_VT1_PD_SITE2_UNIT2_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site2_unit2(&self) -> u32 {
        (self.bits & CombPdSite2Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2_MASK)
            >> CombPdSite2Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site2_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2_MASK)
                | ((value << CombPdSite2Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2_OFFSET)
                    & CombPdSite2Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT2_MASK),
        }
    }
}

impl Default for CombPdSite2Unit2Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite2Unit2Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit2Vt1Conf2Val {
        CombPdSite2Unit2Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit2Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit2Vt1Conf2Val) -> CombPdSite2Unit2Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT3_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01a4
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT3** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE2_UNIT3** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit3Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit3Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit3Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT3_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT3_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE2_UNIT3_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE2_UNIT3_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE2_UNIT3: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT3** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site2_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT3_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT3_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE2_UNIT3** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site2_unit3(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE2_UNIT3_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site2_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site2_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3_MASK)
        });
    }
}

/// COMB_PD_SITE2_UNIT3_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit3vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit3vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit3Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite2Unit3Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT3** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site2_unit3(&self) -> u32 {
        (self.bits & CombPdSite2Unit3Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT3_MASK)
            >> CombPdSite2Unit3Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE2_UNIT3_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE2_UNIT3** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site2_unit3(&self) -> bool {
        (self.bits & CombPdSite2Unit3Vt1Conf2::DELAY_OVF_VT1_PD_SITE2_UNIT3_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site2_unit3(&self) -> u32 {
        (self.bits & CombPdSite2Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3_MASK)
            >> CombPdSite2Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site2_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3_MASK)
                | ((value << CombPdSite2Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3_OFFSET)
                    & CombPdSite2Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE2_UNIT3_MASK),
        }
    }
}

impl Default for CombPdSite2Unit3Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite2Unit3Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit3Vt1Conf2Val {
        CombPdSite2Unit3Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit3Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit3Vt1Conf2Val) -> CombPdSite2Unit3Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT0_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01a8
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT0** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE2_UNIT0** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit0Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit0Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit0Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT0_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT0_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE2_UNIT0_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE2_UNIT0_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE2_UNIT0: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT0** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site2_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT0_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT0_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE2_UNIT0** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site2_unit0(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE2_UNIT0_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site2_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site2_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0_MASK)
        });
    }
}

/// COMB_PD_SITE2_UNIT0_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit0vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit0vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit0Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite2Unit0Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT0** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site2_unit0(&self) -> u32 {
        (self.bits & CombPdSite2Unit0Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT0_MASK)
            >> CombPdSite2Unit0Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT0_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE2_UNIT0** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site2_unit0(&self) -> bool {
        (self.bits & CombPdSite2Unit0Vt2Conf2::DELAY_OVF_VT2_PD_SITE2_UNIT0_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site2_unit0(&self) -> u32 {
        (self.bits & CombPdSite2Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0_MASK)
            >> CombPdSite2Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site2_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0_MASK)
                | ((value << CombPdSite2Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0_OFFSET)
                    & CombPdSite2Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT0_MASK),
        }
    }
}

impl Default for CombPdSite2Unit0Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite2Unit0Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit0Vt2Conf2Val {
        CombPdSite2Unit0Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit0Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit0Vt2Conf2Val) -> CombPdSite2Unit0Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT1_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01ac
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT1** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE2_UNIT1** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit1Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit1Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit1Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT1_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT1_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE2_UNIT1_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE2_UNIT1_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE2_UNIT1: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT1** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site2_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT1_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT1_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE2_UNIT1** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site2_unit1(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE2_UNIT1_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site2_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site2_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1_MASK)
        });
    }
}

/// COMB_PD_SITE2_UNIT1_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit1vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit1vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit1Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite2Unit1Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT1** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site2_unit1(&self) -> u32 {
        (self.bits & CombPdSite2Unit1Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT1_MASK)
            >> CombPdSite2Unit1Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT1_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE2_UNIT1** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site2_unit1(&self) -> bool {
        (self.bits & CombPdSite2Unit1Vt2Conf2::DELAY_OVF_VT2_PD_SITE2_UNIT1_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site2_unit1(&self) -> u32 {
        (self.bits & CombPdSite2Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1_MASK)
            >> CombPdSite2Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site2_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1_MASK)
                | ((value << CombPdSite2Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1_OFFSET)
                    & CombPdSite2Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT1_MASK),
        }
    }
}

impl Default for CombPdSite2Unit1Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite2Unit1Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit1Vt2Conf2Val {
        CombPdSite2Unit1Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit1Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit1Vt2Conf2Val) -> CombPdSite2Unit1Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT2_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01b0
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT2** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE2_UNIT2** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit2Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit2Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit2Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT2_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT2_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE2_UNIT2_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE2_UNIT2_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE2_UNIT2: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT2** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site2_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT2_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT2_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE2_UNIT2** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site2_unit2(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE2_UNIT2_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site2_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site2_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2_MASK)
        });
    }
}

/// COMB_PD_SITE2_UNIT2_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit2vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit2vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit2Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite2Unit2Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT2** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site2_unit2(&self) -> u32 {
        (self.bits & CombPdSite2Unit2Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT2_MASK)
            >> CombPdSite2Unit2Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT2_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE2_UNIT2** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site2_unit2(&self) -> bool {
        (self.bits & CombPdSite2Unit2Vt2Conf2::DELAY_OVF_VT2_PD_SITE2_UNIT2_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site2_unit2(&self) -> u32 {
        (self.bits & CombPdSite2Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2_MASK)
            >> CombPdSite2Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site2_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2_MASK)
                | ((value << CombPdSite2Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2_OFFSET)
                    & CombPdSite2Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT2_MASK),
        }
    }
}

impl Default for CombPdSite2Unit2Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite2Unit2Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit2Vt2Conf2Val {
        CombPdSite2Unit2Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit2Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit2Vt2Conf2Val) -> CombPdSite2Unit2Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE2_UNIT3_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01b4
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT3** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE2_UNIT3** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite2Unit3Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite2Unit3Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite2Unit3Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT3_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT3_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE2_UNIT3_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE2_UNIT3_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE2_UNIT3: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT3** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site2_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT3_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT3_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE2_UNIT3** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site2_unit3(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE2_UNIT3_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site2_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site2_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3_MASK)
        });
    }
}

/// COMB_PD_SITE2_UNIT3_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite2unit3vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite2unit3vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite2Unit3Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite2Unit3Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT3** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site2_unit3(&self) -> u32 {
        (self.bits & CombPdSite2Unit3Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT3_MASK)
            >> CombPdSite2Unit3Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE2_UNIT3_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE2_UNIT3** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site2_unit3(&self) -> bool {
        (self.bits & CombPdSite2Unit3Vt2Conf2::DELAY_OVF_VT2_PD_SITE2_UNIT3_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site2_unit3(&self) -> u32 {
        (self.bits & CombPdSite2Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3_MASK)
            >> CombPdSite2Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site2_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite2Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3_MASK)
                | ((value << CombPdSite2Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3_OFFSET)
                    & CombPdSite2Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE2_UNIT3_MASK),
        }
    }
}

impl Default for CombPdSite2Unit3Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite2Unit3Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite2Unit3Vt2Conf2Val {
        CombPdSite2Unit3Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite2Unit3Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite2Unit3Vt2Conf2Val) -> CombPdSite2Unit3Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT0_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01b8
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT0** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE3_UNIT0** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit0Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit0Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit0Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT0_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT0_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE3_UNIT0_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE3_UNIT0_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE3_UNIT0: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT0** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site3_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT0_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT0_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE3_UNIT0** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site3_unit0(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE3_UNIT0_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site3_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site3_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0_MASK)
        });
    }
}

/// COMB_PD_SITE3_UNIT0_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit0vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit0vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit0Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite3Unit0Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT0** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site3_unit0(&self) -> u32 {
        (self.bits & CombPdSite3Unit0Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT0_MASK)
            >> CombPdSite3Unit0Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT0_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE3_UNIT0** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site3_unit0(&self) -> bool {
        (self.bits & CombPdSite3Unit0Vt0Conf2::DELAY_OVF_VT0_PD_SITE3_UNIT0_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site3_unit0(&self) -> u32 {
        (self.bits & CombPdSite3Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0_MASK)
            >> CombPdSite3Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site3_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0_MASK)
                | ((value << CombPdSite3Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0_OFFSET)
                    & CombPdSite3Unit0Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT0_MASK),
        }
    }
}

impl Default for CombPdSite3Unit0Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite3Unit0Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit0Vt0Conf2Val {
        CombPdSite3Unit0Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit0Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit0Vt0Conf2Val) -> CombPdSite3Unit0Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT1_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01bc
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT1** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE3_UNIT1** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit1Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit1Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit1Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT1_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT1_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE3_UNIT1_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE3_UNIT1_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE3_UNIT1: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT1** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site3_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT1_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT1_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE3_UNIT1** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site3_unit1(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE3_UNIT1_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site3_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site3_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1_MASK)
        });
    }
}

/// COMB_PD_SITE3_UNIT1_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit1vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit1vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit1Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite3Unit1Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT1** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site3_unit1(&self) -> u32 {
        (self.bits & CombPdSite3Unit1Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT1_MASK)
            >> CombPdSite3Unit1Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT1_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE3_UNIT1** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site3_unit1(&self) -> bool {
        (self.bits & CombPdSite3Unit1Vt0Conf2::DELAY_OVF_VT0_PD_SITE3_UNIT1_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site3_unit1(&self) -> u32 {
        (self.bits & CombPdSite3Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1_MASK)
            >> CombPdSite3Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site3_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1_MASK)
                | ((value << CombPdSite3Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1_OFFSET)
                    & CombPdSite3Unit1Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT1_MASK),
        }
    }
}

impl Default for CombPdSite3Unit1Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite3Unit1Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit1Vt0Conf2Val {
        CombPdSite3Unit1Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit1Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit1Vt0Conf2Val) -> CombPdSite3Unit1Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT2_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01c0
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT2** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE3_UNIT2** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit2Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit2Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit2Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT2_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT2_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE3_UNIT2_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE3_UNIT2_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE3_UNIT2: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT2** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site3_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT2_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT2_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE3_UNIT2** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site3_unit2(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE3_UNIT2_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site3_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site3_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2_MASK)
        });
    }
}

/// COMB_PD_SITE3_UNIT2_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit2vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit2vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit2Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite3Unit2Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT2** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site3_unit2(&self) -> u32 {
        (self.bits & CombPdSite3Unit2Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT2_MASK)
            >> CombPdSite3Unit2Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT2_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE3_UNIT2** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site3_unit2(&self) -> bool {
        (self.bits & CombPdSite3Unit2Vt0Conf2::DELAY_OVF_VT0_PD_SITE3_UNIT2_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site3_unit2(&self) -> u32 {
        (self.bits & CombPdSite3Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2_MASK)
            >> CombPdSite3Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site3_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2_MASK)
                | ((value << CombPdSite3Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2_OFFSET)
                    & CombPdSite3Unit2Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT2_MASK),
        }
    }
}

impl Default for CombPdSite3Unit2Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite3Unit2Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit2Vt0Conf2Val {
        CombPdSite3Unit2Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit2Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit2Vt0Conf2Val) -> CombPdSite3Unit2Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT3_VT0_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01c4
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT3** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT0_PD_SITE3_UNIT3** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit3Vt0Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit3Vt0Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit3Vt0Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT3_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT3_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT0_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT0_PD_SITE3_UNIT3_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT0_PD_SITE3_UNIT3_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT0_PD_SITE3_UNIT3: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT3** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site3_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT3_MASK)
            >> Self::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT3_OFFSET
    }

    /// **DELAY_OVF_VT0_PD_SITE3_UNIT3** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site3_unit3(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT0_PD_SITE3_UNIT3_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site3_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3_MASK)
            >> Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt0_pd_site3_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3_MASK)
        });
    }
}

/// COMB_PD_SITE3_UNIT3_VT0_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit3vt0conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit3vt0conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit3Vt0Conf2Val {
    bits: u32,
}

impl CombPdSite3Unit3Vt0Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT3** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt0_pd_site3_unit3(&self) -> u32 {
        (self.bits & CombPdSite3Unit3Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT3_MASK)
            >> CombPdSite3Unit3Vt0Conf2::TIMING_ERR_CNT_O_VT0_PD_SITE3_UNIT3_OFFSET
    }

    /// Read **DELAY_OVF_VT0_PD_SITE3_UNIT3** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt0_pd_site3_unit3(&self) -> bool {
        (self.bits & CombPdSite3Unit3Vt0Conf2::DELAY_OVF_VT0_PD_SITE3_UNIT3_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt0_pd_site3_unit3(&self) -> u32 {
        (self.bits & CombPdSite3Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3_MASK)
            >> CombPdSite3Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt0_pd_site3_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3_MASK)
                | ((value << CombPdSite3Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3_OFFSET)
                    & CombPdSite3Unit3Vt0Conf2::MONITOR_EDG_MOD_VT0_PD_SITE3_UNIT3_MASK),
        }
    }
}

impl Default for CombPdSite3Unit3Vt0Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite3Unit3Vt0Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit3Vt0Conf2Val {
        CombPdSite3Unit3Vt0Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit3Vt0Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit3Vt0Conf2Val) -> CombPdSite3Unit3Vt0Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT0_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01c8
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT0** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE3_UNIT0** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit0Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit0Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit0Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT0_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT0_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE3_UNIT0_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE3_UNIT0_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE3_UNIT0: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT0** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site3_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT0_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT0_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE3_UNIT0** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site3_unit0(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE3_UNIT0_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site3_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site3_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0_MASK)
        });
    }
}

/// COMB_PD_SITE3_UNIT0_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit0vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit0vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit0Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite3Unit0Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT0** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site3_unit0(&self) -> u32 {
        (self.bits & CombPdSite3Unit0Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT0_MASK)
            >> CombPdSite3Unit0Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT0_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE3_UNIT0** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site3_unit0(&self) -> bool {
        (self.bits & CombPdSite3Unit0Vt1Conf2::DELAY_OVF_VT1_PD_SITE3_UNIT0_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site3_unit0(&self) -> u32 {
        (self.bits & CombPdSite3Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0_MASK)
            >> CombPdSite3Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site3_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0_MASK)
                | ((value << CombPdSite3Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0_OFFSET)
                    & CombPdSite3Unit0Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT0_MASK),
        }
    }
}

impl Default for CombPdSite3Unit0Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite3Unit0Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit0Vt1Conf2Val {
        CombPdSite3Unit0Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit0Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit0Vt1Conf2Val) -> CombPdSite3Unit0Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT1_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01cc
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT1** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE3_UNIT1** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit1Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit1Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit1Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT1_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT1_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE3_UNIT1_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE3_UNIT1_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE3_UNIT1: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT1** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site3_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT1_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT1_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE3_UNIT1** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site3_unit1(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE3_UNIT1_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site3_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site3_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1_MASK)
        });
    }
}

/// COMB_PD_SITE3_UNIT1_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit1vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit1vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit1Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite3Unit1Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT1** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site3_unit1(&self) -> u32 {
        (self.bits & CombPdSite3Unit1Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT1_MASK)
            >> CombPdSite3Unit1Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT1_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE3_UNIT1** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site3_unit1(&self) -> bool {
        (self.bits & CombPdSite3Unit1Vt1Conf2::DELAY_OVF_VT1_PD_SITE3_UNIT1_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site3_unit1(&self) -> u32 {
        (self.bits & CombPdSite3Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1_MASK)
            >> CombPdSite3Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site3_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1_MASK)
                | ((value << CombPdSite3Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1_OFFSET)
                    & CombPdSite3Unit1Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT1_MASK),
        }
    }
}

impl Default for CombPdSite3Unit1Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite3Unit1Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit1Vt1Conf2Val {
        CombPdSite3Unit1Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit1Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit1Vt1Conf2Val) -> CombPdSite3Unit1Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT2_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01d0
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT2** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE3_UNIT2** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit2Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit2Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit2Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT2_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT2_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE3_UNIT2_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE3_UNIT2_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE3_UNIT2: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT2** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site3_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT2_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT2_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE3_UNIT2** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site3_unit2(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE3_UNIT2_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site3_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site3_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2_MASK)
        });
    }
}

/// COMB_PD_SITE3_UNIT2_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit2vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit2vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit2Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite3Unit2Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT2** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site3_unit2(&self) -> u32 {
        (self.bits & CombPdSite3Unit2Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT2_MASK)
            >> CombPdSite3Unit2Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT2_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE3_UNIT2** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site3_unit2(&self) -> bool {
        (self.bits & CombPdSite3Unit2Vt1Conf2::DELAY_OVF_VT1_PD_SITE3_UNIT2_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site3_unit2(&self) -> u32 {
        (self.bits & CombPdSite3Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2_MASK)
            >> CombPdSite3Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site3_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2_MASK)
                | ((value << CombPdSite3Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2_OFFSET)
                    & CombPdSite3Unit2Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT2_MASK),
        }
    }
}

impl Default for CombPdSite3Unit2Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite3Unit2Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit2Vt1Conf2Val {
        CombPdSite3Unit2Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit2Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit2Vt1Conf2Val) -> CombPdSite3Unit2Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT3_VT1_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01d4
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT3** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT1_PD_SITE3_UNIT3** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit3Vt1Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit3Vt1Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit3Vt1Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT3_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT3_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT1_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT1_PD_SITE3_UNIT3_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT1_PD_SITE3_UNIT3_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT1_PD_SITE3_UNIT3: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT3** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt1_pd_site3_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT3_MASK)
            >> Self::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT3_OFFSET
    }

    /// **DELAY_OVF_VT1_PD_SITE3_UNIT3** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt1_pd_site3_unit3(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT1_PD_SITE3_UNIT3_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt1_pd_site3_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3_MASK)
            >> Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt1_pd_site3_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3_MASK)
        });
    }
}

/// COMB_PD_SITE3_UNIT3_VT1_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit3vt1conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit3vt1conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit3Vt1Conf2Val {
    bits: u32,
}

impl CombPdSite3Unit3Vt1Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT3** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt1_pd_site3_unit3(&self) -> u32 {
        (self.bits & CombPdSite3Unit3Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT3_MASK)
            >> CombPdSite3Unit3Vt1Conf2::TIMING_ERR_CNT_O_VT1_PD_SITE3_UNIT3_OFFSET
    }

    /// Read **DELAY_OVF_VT1_PD_SITE3_UNIT3** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt1_pd_site3_unit3(&self) -> bool {
        (self.bits & CombPdSite3Unit3Vt1Conf2::DELAY_OVF_VT1_PD_SITE3_UNIT3_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt1_pd_site3_unit3(&self) -> u32 {
        (self.bits & CombPdSite3Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3_MASK)
            >> CombPdSite3Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt1_pd_site3_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3_MASK)
                | ((value << CombPdSite3Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3_OFFSET)
                    & CombPdSite3Unit3Vt1Conf2::MONITOR_EDG_MOD_VT1_PD_SITE3_UNIT3_MASK),
        }
    }
}

impl Default for CombPdSite3Unit3Vt1Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite3Unit3Vt1Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit3Vt1Conf2Val {
        CombPdSite3Unit3Vt1Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit3Vt1Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit3Vt1Conf2Val) -> CombPdSite3Unit3Vt1Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT0_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01d8
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT0** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE3_UNIT0** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit0Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit0Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit0Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT0_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT0_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE3_UNIT0_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE3_UNIT0_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE3_UNIT0: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT0** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site3_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT0_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT0_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE3_UNIT0** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site3_unit0(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE3_UNIT0_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site3_unit0(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site3_unit0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0_MASK)
        });
    }
}

/// COMB_PD_SITE3_UNIT0_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit0vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit0vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit0Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite3Unit0Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT0** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site3_unit0(&self) -> u32 {
        (self.bits & CombPdSite3Unit0Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT0_MASK)
            >> CombPdSite3Unit0Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT0_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE3_UNIT0** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site3_unit0(&self) -> bool {
        (self.bits & CombPdSite3Unit0Vt2Conf2::DELAY_OVF_VT2_PD_SITE3_UNIT0_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site3_unit0(&self) -> u32 {
        (self.bits & CombPdSite3Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0_MASK)
            >> CombPdSite3Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site3_unit0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0_MASK)
                | ((value << CombPdSite3Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0_OFFSET)
                    & CombPdSite3Unit0Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT0_MASK),
        }
    }
}

impl Default for CombPdSite3Unit0Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite3Unit0Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit0Vt2Conf2Val {
        CombPdSite3Unit0Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit0Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit0Vt2Conf2Val) -> CombPdSite3Unit0Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT1_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01dc
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT1** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE3_UNIT1** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit1Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit1Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit1Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT1_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT1_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE3_UNIT1_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE3_UNIT1_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE3_UNIT1: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT1** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site3_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT1_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT1_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE3_UNIT1** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site3_unit1(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE3_UNIT1_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site3_unit1(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site3_unit1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1_MASK)
        });
    }
}

/// COMB_PD_SITE3_UNIT1_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit1vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit1vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit1Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite3Unit1Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT1** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site3_unit1(&self) -> u32 {
        (self.bits & CombPdSite3Unit1Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT1_MASK)
            >> CombPdSite3Unit1Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT1_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE3_UNIT1** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site3_unit1(&self) -> bool {
        (self.bits & CombPdSite3Unit1Vt2Conf2::DELAY_OVF_VT2_PD_SITE3_UNIT1_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site3_unit1(&self) -> u32 {
        (self.bits & CombPdSite3Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1_MASK)
            >> CombPdSite3Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site3_unit1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1_MASK)
                | ((value << CombPdSite3Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1_OFFSET)
                    & CombPdSite3Unit1Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT1_MASK),
        }
    }
}

impl Default for CombPdSite3Unit1Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite3Unit1Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit1Vt2Conf2Val {
        CombPdSite3Unit1Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit1Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit1Vt2Conf2Val) -> CombPdSite3Unit1Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT2_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01e0
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT2** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE3_UNIT2** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit2Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit2Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit2Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT2_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT2_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE3_UNIT2_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE3_UNIT2_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE3_UNIT2: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT2** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site3_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT2_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT2_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE3_UNIT2** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site3_unit2(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE3_UNIT2_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site3_unit2(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site3_unit2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2_MASK)
        });
    }
}

/// COMB_PD_SITE3_UNIT2_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit2vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit2vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit2Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite3Unit2Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT2** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site3_unit2(&self) -> u32 {
        (self.bits & CombPdSite3Unit2Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT2_MASK)
            >> CombPdSite3Unit2Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT2_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE3_UNIT2** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site3_unit2(&self) -> bool {
        (self.bits & CombPdSite3Unit2Vt2Conf2::DELAY_OVF_VT2_PD_SITE3_UNIT2_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site3_unit2(&self) -> u32 {
        (self.bits & CombPdSite3Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2_MASK)
            >> CombPdSite3Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site3_unit2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2_MASK)
                | ((value << CombPdSite3Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2_OFFSET)
                    & CombPdSite3Unit2Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT2_MASK),
        }
    }
}

impl Default for CombPdSite3Unit2Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite3Unit2Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit2Vt2Conf2Val {
        CombPdSite3Unit2Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit2Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit2Vt2Conf2Val) -> CombPdSite3Unit2Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// COMB_PD_SITE3_UNIT3_VT2_CONF2 Register
///
/// needs desc
///
/// Address offset: 0x01e4
///
/// ## Fields
///
/// - **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT3** `[31:16]` (RO)
///
///   needs field desc
///
/// - **DELAY_OVF_VT2_PD_SITE3_UNIT3** `[15]` (RO)
///
///   needs field desc
///
/// - **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3** `[1:0]` (RW)
///
///   needs field desc
#[repr(transparent)]
pub struct CombPdSite3Unit3Vt2Conf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CombPdSite3Unit3Vt2Conf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CombPdSite3Unit3Vt2Conf2 {
    // ======== Field Constants ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT3_OFFSET: u32 = 16;
    pub const TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT3_MASK: u32 = 0xffff0000;

    /// **DELAY_OVF_VT2_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DELAY_OVF_VT2_PD_SITE3_UNIT3_OFFSET: u32 = 15;
    pub const DELAY_OVF_VT2_PD_SITE3_UNIT3_MASK: u32 = 0x00008000;
    pub const DELAY_OVF_VT2_PD_SITE3_UNIT3: u32 = 1 << 15;

    /// **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3**
    ///
    /// needs field desc
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3_OFFSET: u32 = 0;
    pub const MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT3** `[31:16]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site3_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT3_MASK)
            >> Self::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT3_OFFSET
    }

    /// **DELAY_OVF_VT2_PD_SITE3_UNIT3** `[15]` (RO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site3_unit3(&self) -> bool {
        (self.value.read_bits() & Self::DELAY_OVF_VT2_PD_SITE3_UNIT3_MASK) != 0
    }

    /// **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site3_unit3(&self) -> u32 {
        (self.value.read_bits() & Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3_MASK)
            >> Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3_OFFSET
    }

    /// Write **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3** `[1:0]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn write_monitor_edg_mod_vt2_pd_site3_unit3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3_MASK)
                | ((value << Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3_OFFSET)
                    & Self::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3_MASK)
        });
    }
}

/// COMB_PD_SITE3_UNIT3_VT2_CONF2 value type for batch read/modify/write operations
///
/// Use `combpdsite3unit3vt2conf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `combpdsite3unit3vt2conf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CombPdSite3Unit3Vt2Conf2Val {
    bits: u32,
}

impl CombPdSite3Unit3Vt2Conf2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT3** `[31:16]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn timing_err_cnt_o_vt2_pd_site3_unit3(&self) -> u32 {
        (self.bits & CombPdSite3Unit3Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT3_MASK)
            >> CombPdSite3Unit3Vt2Conf2::TIMING_ERR_CNT_O_VT2_PD_SITE3_UNIT3_OFFSET
    }

    /// Read **DELAY_OVF_VT2_PD_SITE3_UNIT3** `[15:15]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn delay_ovf_vt2_pd_site3_unit3(&self) -> bool {
        (self.bits & CombPdSite3Unit3Vt2Conf2::DELAY_OVF_VT2_PD_SITE3_UNIT3_MASK) != 0
    }

    /// Read **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn monitor_edg_mod_vt2_pd_site3_unit3(&self) -> u32 {
        (self.bits & CombPdSite3Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3_MASK)
            >> CombPdSite3Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3_OFFSET
    }

    /// Set **MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3** `[1:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_monitor_edg_mod_vt2_pd_site3_unit3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CombPdSite3Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3_MASK)
                | ((value << CombPdSite3Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3_OFFSET)
                    & CombPdSite3Unit3Vt2Conf2::MONITOR_EDG_MOD_VT2_PD_SITE3_UNIT3_MASK),
        }
    }
}

impl Default for CombPdSite3Unit3Vt2Conf2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CombPdSite3Unit3Vt2Conf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CombPdSite3Unit3Vt2Conf2Val {
        CombPdSite3Unit3Vt2Conf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CombPdSite3Unit3Vt2Conf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CombPdSite3Unit3Vt2Conf2Val) -> CombPdSite3Unit3Vt2Conf2Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// VALUE_UPDATE Register
///
/// needs field desc
///
/// Address offset: 0x01e8
///
/// ## Fields
///
/// - **BYPASS** `[1]` (RW)
///
///   needs field desc
///
/// - **VALUE_UPDATE** `[0]` (WO)
///
///   needs field desc
#[repr(transparent)]
pub struct ValueUpdate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ValueUpdate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ValueUpdate {
    // ======== Field Constants ========

    /// **BYPASS**
    ///
    /// needs field desc
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BYPASS_OFFSET: u32 = 1;
    pub const BYPASS_MASK: u32 = 0x00000002;
    pub const BYPASS: u32 = 1 << 1;

    /// **VALUE_UPDATE**
    ///
    /// needs field desc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const VALUE_UPDATE_OFFSET: u32 = 0;
    pub const VALUE_UPDATE_MASK: u32 = 0x00000001;
    pub const VALUE_UPDATE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BYPASS** `[1]` (RW)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn bypass(&self) -> bool {
        (self.value.read_bits() & Self::BYPASS_MASK) != 0
    }

    /// Set BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bypass(&self) {
        self.value.modify_bits(|r| r | Self::BYPASS_MASK);
    }

    /// Clear BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bypass(&self) {
        self.value.modify_bits(|r| r & !Self::BYPASS_MASK);
    }

    /// Trigger **VALUE_UPDATE** (WO)
    ///
    /// needs field desc
    #[inline(always)]
    pub fn trigger_value_update(&self) {
        self.value.modify_bits(|r| r | Self::VALUE_UPDATE_MASK);
    }
}

/// VALUE_UPDATE value type for batch read/modify/write operations
///
/// Use `valueupdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `valueupdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ValueUpdateVal {
    bits: u32,
}

impl ValueUpdateVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BYPASS** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        (self.bits & ValueUpdate::BYPASS_MASK) != 0
    }

    /// Set **BYPASS** `[1:1]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ValueUpdate::BYPASS_MASK
            } else {
                self.bits & !ValueUpdate::BYPASS_MASK
            },
        }
    }

    /// Set **VALUE_UPDATE** `[0:0]`
    ///
    /// needs field desc
    #[inline(always)]
    pub const fn with_value_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ValueUpdate::VALUE_UPDATE_MASK
            } else {
                self.bits & !ValueUpdate::VALUE_UPDATE_MASK
            },
        }
    }
}

impl Default for ValueUpdateVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ValueUpdate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ValueUpdateVal {
        ValueUpdateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ValueUpdateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ValueUpdateVal) -> ValueUpdateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// version register
///
/// Address offset: 0x0ffc
/// Reset value: 0x02112130
///
/// ## Fields
///
/// - **DATE** `[31:0]` (RW)
///
///   version register
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

    /// **DATE**
    ///
    /// version register
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DATE** `[31:0]` (RW)
    ///
    /// version register
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[31:0]` (RW)
    ///
    /// version register
    #[inline(always)]
    pub fn write_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATE_MASK) | ((value << Self::DATE_OFFSET) & Self::DATE_MASK)
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
        Self { bits: 0x02112130 }
    }

    /// Read **DATE** `[31:0]`
    ///
    /// version register
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[31:0]`
    ///
    /// version register
    #[inline(always)]
    pub const fn with_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Date::DATE_MASK) | ((value << Date::DATE_OFFSET) & Date::DATE_MASK),
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
    /// select valid pvt channel
    pub pmup_bitmap_high0: PmupBitmapHigh0,
    /// 0x0004
    /// select valid pvt channel
    pub pmup_bitmap_high1: PmupBitmapHigh1,
    /// 0x0008
    /// select valid pvt channel
    pub pmup_bitmap_high2: PmupBitmapHigh2,
    /// 0x000c
    /// select valid pvt channel
    pub pmup_bitmap_high3: PmupBitmapHigh3,
    /// 0x0010
    /// select valid pvt channel
    pub pmup_bitmap_high4: PmupBitmapHigh4,
    /// 0x0014
    /// select valid pvt channel
    pub pmup_bitmap_low0: PmupBitmapLow0,
    /// 0x0018
    /// select valid pvt channel
    pub pmup_bitmap_low1: PmupBitmapLow1,
    /// 0x001c
    /// select valid pvt channel
    pub pmup_bitmap_low2: PmupBitmapLow2,
    /// 0x0020
    /// select valid pvt channel
    pub pmup_bitmap_low3: PmupBitmapLow3,
    /// 0x0024
    /// select valid pvt channel
    pub pmup_bitmap_low4: PmupBitmapLow4,
    /// 0x0028
    /// configure pump drv
    pub pmup_drv_cfg: PmupDrvCfg,
    /// 0x002c
    /// configure the code of valid pump channel code
    pub pmup_channel_cfg: PmupChannelCfg,
    /// 0x0030
    /// configure pvt clk
    pub clk_cfg: ClkCfg,
    /// 0x0034
    /// needs desc
    pub dbias_channel_sel0: DbiasChannelSel0,
    /// 0x0038
    /// needs desc
    pub dbias_channel_sel1: DbiasChannelSel1,
    /// 0x003c
    /// needs desc
    pub dbias_channel0_sel: DbiasChannel0Sel,
    /// 0x0040
    /// needs desc
    pub dbias_channel1_sel: DbiasChannel1Sel,
    /// 0x0044
    /// needs desc
    pub dbias_channel2_sel: DbiasChannel2Sel,
    /// 0x0048
    /// needs desc
    pub dbias_channel3_sel: DbiasChannel3Sel,
    /// 0x004c
    /// needs desc
    pub dbias_channel4_sel: DbiasChannel4Sel,
    /// 0x0050
    /// needs desc
    pub dbias_cmd0: DbiasCmd0,
    /// 0x0054
    /// needs desc
    pub dbias_cmd1: DbiasCmd1,
    /// 0x0058
    /// needs desc
    pub dbias_cmd2: DbiasCmd2,
    /// 0x005c
    /// needs desc
    pub dbias_cmd3: DbiasCmd3,
    /// 0x0060
    /// needs desc
    pub dbias_cmd4: DbiasCmd4,
    /// 0x0064
    /// needs desc
    pub dbias_timer: DbiasTimer,
    /// 0x0068
    /// needs desc
    pub comb_pd_site0_unit0_vt0_conf1: CombPdSite0Unit0Vt0Conf1,
    /// 0x006c
    /// needs desc
    pub comb_pd_site0_unit1_vt0_conf1: CombPdSite0Unit1Vt0Conf1,
    /// 0x0070
    /// needs desc
    pub comb_pd_site0_unit2_vt0_conf1: CombPdSite0Unit2Vt0Conf1,
    /// 0x0074
    /// needs desc
    pub comb_pd_site0_unit3_vt0_conf1: CombPdSite0Unit3Vt0Conf1,
    /// 0x0078
    /// needs desc
    pub comb_pd_site0_unit0_vt1_conf1: CombPdSite0Unit0Vt1Conf1,
    /// 0x007c
    /// needs desc
    pub comb_pd_site0_unit1_vt1_conf1: CombPdSite0Unit1Vt1Conf1,
    /// 0x0080
    /// needs desc
    pub comb_pd_site0_unit2_vt1_conf1: CombPdSite0Unit2Vt1Conf1,
    /// 0x0084
    /// needs desc
    pub comb_pd_site0_unit3_vt1_conf1: CombPdSite0Unit3Vt1Conf1,
    /// 0x0088
    /// needs desc
    pub comb_pd_site0_unit0_vt2_conf1: CombPdSite0Unit0Vt2Conf1,
    /// 0x008c
    /// needs desc
    pub comb_pd_site0_unit1_vt2_conf1: CombPdSite0Unit1Vt2Conf1,
    /// 0x0090
    /// needs desc
    pub comb_pd_site0_unit2_vt2_conf1: CombPdSite0Unit2Vt2Conf1,
    /// 0x0094
    /// needs desc
    pub comb_pd_site0_unit3_vt2_conf1: CombPdSite0Unit3Vt2Conf1,
    /// 0x0098
    /// needs desc
    pub comb_pd_site1_unit0_vt0_conf1: CombPdSite1Unit0Vt0Conf1,
    /// 0x009c
    /// needs desc
    pub comb_pd_site1_unit1_vt0_conf1: CombPdSite1Unit1Vt0Conf1,
    /// 0x00a0
    /// needs desc
    pub comb_pd_site1_unit2_vt0_conf1: CombPdSite1Unit2Vt0Conf1,
    /// 0x00a4
    /// needs desc
    pub comb_pd_site1_unit3_vt0_conf1: CombPdSite1Unit3Vt0Conf1,
    /// 0x00a8
    /// needs desc
    pub comb_pd_site1_unit0_vt1_conf1: CombPdSite1Unit0Vt1Conf1,
    /// 0x00ac
    /// needs desc
    pub comb_pd_site1_unit1_vt1_conf1: CombPdSite1Unit1Vt1Conf1,
    /// 0x00b0
    /// needs desc
    pub comb_pd_site1_unit2_vt1_conf1: CombPdSite1Unit2Vt1Conf1,
    /// 0x00b4
    /// needs desc
    pub comb_pd_site1_unit3_vt1_conf1: CombPdSite1Unit3Vt1Conf1,
    /// 0x00b8
    /// needs desc
    pub comb_pd_site1_unit0_vt2_conf1: CombPdSite1Unit0Vt2Conf1,
    /// 0x00bc
    /// needs desc
    pub comb_pd_site1_unit1_vt2_conf1: CombPdSite1Unit1Vt2Conf1,
    /// 0x00c0
    /// needs desc
    pub comb_pd_site1_unit2_vt2_conf1: CombPdSite1Unit2Vt2Conf1,
    /// 0x00c4
    /// needs desc
    pub comb_pd_site1_unit3_vt2_conf1: CombPdSite1Unit3Vt2Conf1,
    /// 0x00c8
    /// needs desc
    pub comb_pd_site2_unit0_vt0_conf1: CombPdSite2Unit0Vt0Conf1,
    /// 0x00cc
    /// needs desc
    pub comb_pd_site2_unit1_vt0_conf1: CombPdSite2Unit1Vt0Conf1,
    /// 0x00d0
    /// needs desc
    pub comb_pd_site2_unit2_vt0_conf1: CombPdSite2Unit2Vt0Conf1,
    /// 0x00d4
    /// needs desc
    pub comb_pd_site2_unit3_vt0_conf1: CombPdSite2Unit3Vt0Conf1,
    /// 0x00d8
    /// needs desc
    pub comb_pd_site2_unit0_vt1_conf1: CombPdSite2Unit0Vt1Conf1,
    /// 0x00dc
    /// needs desc
    pub comb_pd_site2_unit1_vt1_conf1: CombPdSite2Unit1Vt1Conf1,
    /// 0x00e0
    /// needs desc
    pub comb_pd_site2_unit2_vt1_conf1: CombPdSite2Unit2Vt1Conf1,
    /// 0x00e4
    /// needs desc
    pub comb_pd_site2_unit3_vt1_conf1: CombPdSite2Unit3Vt1Conf1,
    /// 0x00e8
    /// needs desc
    pub comb_pd_site2_unit0_vt2_conf1: CombPdSite2Unit0Vt2Conf1,
    /// 0x00ec
    /// needs desc
    pub comb_pd_site2_unit1_vt2_conf1: CombPdSite2Unit1Vt2Conf1,
    /// 0x00f0
    /// needs desc
    pub comb_pd_site2_unit2_vt2_conf1: CombPdSite2Unit2Vt2Conf1,
    /// 0x00f4
    /// needs desc
    pub comb_pd_site2_unit3_vt2_conf1: CombPdSite2Unit3Vt2Conf1,
    /// 0x00f8
    /// needs desc
    pub comb_pd_site3_unit0_vt0_conf1: CombPdSite3Unit0Vt0Conf1,
    /// 0x00fc
    /// needs desc
    pub comb_pd_site3_unit1_vt0_conf1: CombPdSite3Unit1Vt0Conf1,
    /// 0x0100
    /// needs desc
    pub comb_pd_site3_unit2_vt0_conf1: CombPdSite3Unit2Vt0Conf1,
    /// 0x0104
    /// needs desc
    pub comb_pd_site3_unit3_vt0_conf1: CombPdSite3Unit3Vt0Conf1,
    /// 0x0108
    /// needs desc
    pub comb_pd_site3_unit0_vt1_conf1: CombPdSite3Unit0Vt1Conf1,
    /// 0x010c
    /// needs desc
    pub comb_pd_site3_unit1_vt1_conf1: CombPdSite3Unit1Vt1Conf1,
    /// 0x0110
    /// needs desc
    pub comb_pd_site3_unit2_vt1_conf1: CombPdSite3Unit2Vt1Conf1,
    /// 0x0114
    /// needs desc
    pub comb_pd_site3_unit3_vt1_conf1: CombPdSite3Unit3Vt1Conf1,
    /// 0x0118
    /// needs desc
    pub comb_pd_site3_unit0_vt2_conf1: CombPdSite3Unit0Vt2Conf1,
    /// 0x011c
    /// needs desc
    pub comb_pd_site3_unit1_vt2_conf1: CombPdSite3Unit1Vt2Conf1,
    /// 0x0120
    /// needs desc
    pub comb_pd_site3_unit2_vt2_conf1: CombPdSite3Unit2Vt2Conf1,
    /// 0x0124
    /// needs desc
    pub comb_pd_site3_unit3_vt2_conf1: CombPdSite3Unit3Vt2Conf1,
    /// 0x0128
    /// needs desc
    pub comb_pd_site0_unit0_vt0_conf2: CombPdSite0Unit0Vt0Conf2,
    /// 0x012c
    /// needs desc
    pub comb_pd_site0_unit1_vt0_conf2: CombPdSite0Unit1Vt0Conf2,
    /// 0x0130
    /// needs desc
    pub comb_pd_site0_unit2_vt0_conf2: CombPdSite0Unit2Vt0Conf2,
    /// 0x0134
    /// needs desc
    pub comb_pd_site0_unit3_vt0_conf2: CombPdSite0Unit3Vt0Conf2,
    /// 0x0138
    /// needs desc
    pub comb_pd_site0_unit0_vt1_conf2: CombPdSite0Unit0Vt1Conf2,
    /// 0x013c
    /// needs desc
    pub comb_pd_site0_unit1_vt1_conf2: CombPdSite0Unit1Vt1Conf2,
    /// 0x0140
    /// needs desc
    pub comb_pd_site0_unit2_vt1_conf2: CombPdSite0Unit2Vt1Conf2,
    /// 0x0144
    /// needs desc
    pub comb_pd_site0_unit3_vt1_conf2: CombPdSite0Unit3Vt1Conf2,
    /// 0x0148
    /// needs desc
    pub comb_pd_site0_unit0_vt2_conf2: CombPdSite0Unit0Vt2Conf2,
    /// 0x014c
    /// needs desc
    pub comb_pd_site0_unit1_vt2_conf2: CombPdSite0Unit1Vt2Conf2,
    /// 0x0150
    /// needs desc
    pub comb_pd_site0_unit2_vt2_conf2: CombPdSite0Unit2Vt2Conf2,
    /// 0x0154
    /// needs desc
    pub comb_pd_site0_unit3_vt2_conf2: CombPdSite0Unit3Vt2Conf2,
    /// 0x0158
    /// needs desc
    pub comb_pd_site1_unit0_vt0_conf2: CombPdSite1Unit0Vt0Conf2,
    /// 0x015c
    /// needs desc
    pub comb_pd_site1_unit1_vt0_conf2: CombPdSite1Unit1Vt0Conf2,
    /// 0x0160
    /// needs desc
    pub comb_pd_site1_unit2_vt0_conf2: CombPdSite1Unit2Vt0Conf2,
    /// 0x0164
    /// needs desc
    pub comb_pd_site1_unit3_vt0_conf2: CombPdSite1Unit3Vt0Conf2,
    /// 0x0168
    /// needs desc
    pub comb_pd_site1_unit0_vt1_conf2: CombPdSite1Unit0Vt1Conf2,
    /// 0x016c
    /// needs desc
    pub comb_pd_site1_unit1_vt1_conf2: CombPdSite1Unit1Vt1Conf2,
    /// 0x0170
    /// needs desc
    pub comb_pd_site1_unit2_vt1_conf2: CombPdSite1Unit2Vt1Conf2,
    /// 0x0174
    /// needs desc
    pub comb_pd_site1_unit3_vt1_conf2: CombPdSite1Unit3Vt1Conf2,
    /// 0x0178
    /// needs desc
    pub comb_pd_site1_unit0_vt2_conf2: CombPdSite1Unit0Vt2Conf2,
    /// 0x017c
    /// needs desc
    pub comb_pd_site1_unit1_vt2_conf2: CombPdSite1Unit1Vt2Conf2,
    /// 0x0180
    /// needs desc
    pub comb_pd_site1_unit2_vt2_conf2: CombPdSite1Unit2Vt2Conf2,
    /// 0x0184
    /// needs desc
    pub comb_pd_site1_unit3_vt2_conf2: CombPdSite1Unit3Vt2Conf2,
    /// 0x0188
    /// needs desc
    pub comb_pd_site2_unit0_vt0_conf2: CombPdSite2Unit0Vt0Conf2,
    /// 0x018c
    /// needs desc
    pub comb_pd_site2_unit1_vt0_conf2: CombPdSite2Unit1Vt0Conf2,
    /// 0x0190
    /// needs desc
    pub comb_pd_site2_unit2_vt0_conf2: CombPdSite2Unit2Vt0Conf2,
    /// 0x0194
    /// needs desc
    pub comb_pd_site2_unit3_vt0_conf2: CombPdSite2Unit3Vt0Conf2,
    /// 0x0198
    /// needs desc
    pub comb_pd_site2_unit0_vt1_conf2: CombPdSite2Unit0Vt1Conf2,
    /// 0x019c
    /// needs desc
    pub comb_pd_site2_unit1_vt1_conf2: CombPdSite2Unit1Vt1Conf2,
    /// 0x01a0
    /// needs desc
    pub comb_pd_site2_unit2_vt1_conf2: CombPdSite2Unit2Vt1Conf2,
    /// 0x01a4
    /// needs desc
    pub comb_pd_site2_unit3_vt1_conf2: CombPdSite2Unit3Vt1Conf2,
    /// 0x01a8
    /// needs desc
    pub comb_pd_site2_unit0_vt2_conf2: CombPdSite2Unit0Vt2Conf2,
    /// 0x01ac
    /// needs desc
    pub comb_pd_site2_unit1_vt2_conf2: CombPdSite2Unit1Vt2Conf2,
    /// 0x01b0
    /// needs desc
    pub comb_pd_site2_unit2_vt2_conf2: CombPdSite2Unit2Vt2Conf2,
    /// 0x01b4
    /// needs desc
    pub comb_pd_site2_unit3_vt2_conf2: CombPdSite2Unit3Vt2Conf2,
    /// 0x01b8
    /// needs desc
    pub comb_pd_site3_unit0_vt0_conf2: CombPdSite3Unit0Vt0Conf2,
    /// 0x01bc
    /// needs desc
    pub comb_pd_site3_unit1_vt0_conf2: CombPdSite3Unit1Vt0Conf2,
    /// 0x01c0
    /// needs desc
    pub comb_pd_site3_unit2_vt0_conf2: CombPdSite3Unit2Vt0Conf2,
    /// 0x01c4
    /// needs desc
    pub comb_pd_site3_unit3_vt0_conf2: CombPdSite3Unit3Vt0Conf2,
    /// 0x01c8
    /// needs desc
    pub comb_pd_site3_unit0_vt1_conf2: CombPdSite3Unit0Vt1Conf2,
    /// 0x01cc
    /// needs desc
    pub comb_pd_site3_unit1_vt1_conf2: CombPdSite3Unit1Vt1Conf2,
    /// 0x01d0
    /// needs desc
    pub comb_pd_site3_unit2_vt1_conf2: CombPdSite3Unit2Vt1Conf2,
    /// 0x01d4
    /// needs desc
    pub comb_pd_site3_unit3_vt1_conf2: CombPdSite3Unit3Vt1Conf2,
    /// 0x01d8
    /// needs desc
    pub comb_pd_site3_unit0_vt2_conf2: CombPdSite3Unit0Vt2Conf2,
    /// 0x01dc
    /// needs desc
    pub comb_pd_site3_unit1_vt2_conf2: CombPdSite3Unit1Vt2Conf2,
    /// 0x01e0
    /// needs desc
    pub comb_pd_site3_unit2_vt2_conf2: CombPdSite3Unit2Vt2Conf2,
    /// 0x01e4
    /// needs desc
    pub comb_pd_site3_unit3_vt2_conf2: CombPdSite3Unit3Vt2Conf2,
    /// 0x01e8
    /// needs field desc
    pub value_update: ValueUpdate,
    _reserved0: [u8; 3600],
    /// 0x0ffc
    /// version register
    pub date: Date,
}
