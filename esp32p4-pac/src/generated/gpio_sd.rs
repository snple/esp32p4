//! GPIO_SD
//! Sigma-Delta Modulation

#![allow(dead_code)]

pub const BASE: u32 = 0x500e0f00;

/// SIGMADELTA0 Register
///
/// Duty Cycle Configure Register of SDM0
///
/// Address offset: 0x0000
/// Reset value: 0x0000ff00
///
/// ## Fields
///
/// - **SD_PRESCALE** `[15:8]` (RW)
///
///   This field is used to set a divider value to divide APB clock.
///
/// - **SD_IN** `[7:0]` (RW)
///
///   This field is used to configure the duty cycle of sigma delta modulation output.
#[repr(transparent)]
pub struct Sigmadelta0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sigmadelta0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sigmadelta0 {
    // ======== Field Constants ========

    /// **SD_PRESCALE**
    ///
    /// This field is used to set a divider value to divide APB clock.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SD_PRESCALE_OFFSET: u32 = 8;
    pub const SD_PRESCALE_MASK: u32 = 0x0000ff00;

    /// **SD_IN**
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SD_IN_OFFSET: u32 = 0;
    pub const SD_IN_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn sd_prescale(&self) -> u32 {
        (self.value.read_bits() & Self::SD_PRESCALE_MASK) >> Self::SD_PRESCALE_OFFSET
    }

    /// Write **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn write_sd_prescale(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_PRESCALE_MASK)
                | ((value << Self::SD_PRESCALE_OFFSET) & Self::SD_PRESCALE_MASK)
        });
    }

    /// **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn sd_in(&self) -> u32 {
        (self.value.read_bits() & Self::SD_IN_MASK) >> Self::SD_IN_OFFSET
    }

    /// Write **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn write_sd_in(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_IN_MASK) | ((value << Self::SD_IN_OFFSET) & Self::SD_IN_MASK)
        });
    }
}

/// SIGMADELTA0 value type for batch read/modify/write operations
///
/// Use `sigmadelta0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sigmadelta0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sigmadelta0Val {
    bits: u32,
}

impl Sigmadelta0Val {
    /// Create from raw bits
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
        Self { bits: 0x0000ff00 }
    }

    /// Read **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn sd_prescale(&self) -> u32 {
        (self.bits & Sigmadelta0::SD_PRESCALE_MASK) >> Sigmadelta0::SD_PRESCALE_OFFSET
    }

    /// Read **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn sd_in(&self) -> u32 {
        (self.bits & Sigmadelta0::SD_IN_MASK) >> Sigmadelta0::SD_IN_OFFSET
    }

    /// Set **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn with_sd_prescale(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta0::SD_PRESCALE_MASK)
                | ((value << Sigmadelta0::SD_PRESCALE_OFFSET) & Sigmadelta0::SD_PRESCALE_MASK),
        }
    }

    /// Set **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn with_sd_in(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta0::SD_IN_MASK)
                | ((value << Sigmadelta0::SD_IN_OFFSET) & Sigmadelta0::SD_IN_MASK),
        }
    }
}

impl Default for Sigmadelta0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sigmadelta0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sigmadelta0Val {
        Sigmadelta0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sigmadelta0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sigmadelta0Val) -> Sigmadelta0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SIGMADELTA1 Register
///
/// Duty Cycle Configure Register of SDM1
///
/// Address offset: 0x0004
/// Reset value: 0x0000ff00
///
/// ## Fields
///
/// - **SD_PRESCALE** `[15:8]` (RW)
///
///   This field is used to set a divider value to divide APB clock.
///
/// - **SD_IN** `[7:0]` (RW)
///
///   This field is used to configure the duty cycle of sigma delta modulation output.
#[repr(transparent)]
pub struct Sigmadelta1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sigmadelta1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sigmadelta1 {
    // ======== Field Constants ========

    /// **SD_PRESCALE**
    ///
    /// This field is used to set a divider value to divide APB clock.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SD_PRESCALE_OFFSET: u32 = 8;
    pub const SD_PRESCALE_MASK: u32 = 0x0000ff00;

    /// **SD_IN**
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SD_IN_OFFSET: u32 = 0;
    pub const SD_IN_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn sd_prescale(&self) -> u32 {
        (self.value.read_bits() & Self::SD_PRESCALE_MASK) >> Self::SD_PRESCALE_OFFSET
    }

    /// Write **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn write_sd_prescale(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_PRESCALE_MASK)
                | ((value << Self::SD_PRESCALE_OFFSET) & Self::SD_PRESCALE_MASK)
        });
    }

    /// **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn sd_in(&self) -> u32 {
        (self.value.read_bits() & Self::SD_IN_MASK) >> Self::SD_IN_OFFSET
    }

    /// Write **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn write_sd_in(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_IN_MASK) | ((value << Self::SD_IN_OFFSET) & Self::SD_IN_MASK)
        });
    }
}

/// SIGMADELTA1 value type for batch read/modify/write operations
///
/// Use `sigmadelta1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sigmadelta1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sigmadelta1Val {
    bits: u32,
}

impl Sigmadelta1Val {
    /// Create from raw bits
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
        Self { bits: 0x0000ff00 }
    }

    /// Read **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn sd_prescale(&self) -> u32 {
        (self.bits & Sigmadelta1::SD_PRESCALE_MASK) >> Sigmadelta1::SD_PRESCALE_OFFSET
    }

    /// Read **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn sd_in(&self) -> u32 {
        (self.bits & Sigmadelta1::SD_IN_MASK) >> Sigmadelta1::SD_IN_OFFSET
    }

    /// Set **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn with_sd_prescale(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta1::SD_PRESCALE_MASK)
                | ((value << Sigmadelta1::SD_PRESCALE_OFFSET) & Sigmadelta1::SD_PRESCALE_MASK),
        }
    }

    /// Set **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn with_sd_in(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta1::SD_IN_MASK)
                | ((value << Sigmadelta1::SD_IN_OFFSET) & Sigmadelta1::SD_IN_MASK),
        }
    }
}

impl Default for Sigmadelta1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sigmadelta1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sigmadelta1Val {
        Sigmadelta1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sigmadelta1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sigmadelta1Val) -> Sigmadelta1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SIGMADELTA2 Register
///
/// Duty Cycle Configure Register of SDM2
///
/// Address offset: 0x0008
/// Reset value: 0x0000ff00
///
/// ## Fields
///
/// - **SD_PRESCALE** `[15:8]` (RW)
///
///   This field is used to set a divider value to divide APB clock.
///
/// - **SD_IN** `[7:0]` (RW)
///
///   This field is used to configure the duty cycle of sigma delta modulation output.
#[repr(transparent)]
pub struct Sigmadelta2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sigmadelta2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sigmadelta2 {
    // ======== Field Constants ========

    /// **SD_PRESCALE**
    ///
    /// This field is used to set a divider value to divide APB clock.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SD_PRESCALE_OFFSET: u32 = 8;
    pub const SD_PRESCALE_MASK: u32 = 0x0000ff00;

    /// **SD_IN**
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SD_IN_OFFSET: u32 = 0;
    pub const SD_IN_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn sd_prescale(&self) -> u32 {
        (self.value.read_bits() & Self::SD_PRESCALE_MASK) >> Self::SD_PRESCALE_OFFSET
    }

    /// Write **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn write_sd_prescale(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_PRESCALE_MASK)
                | ((value << Self::SD_PRESCALE_OFFSET) & Self::SD_PRESCALE_MASK)
        });
    }

    /// **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn sd_in(&self) -> u32 {
        (self.value.read_bits() & Self::SD_IN_MASK) >> Self::SD_IN_OFFSET
    }

    /// Write **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn write_sd_in(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_IN_MASK) | ((value << Self::SD_IN_OFFSET) & Self::SD_IN_MASK)
        });
    }
}

/// SIGMADELTA2 value type for batch read/modify/write operations
///
/// Use `sigmadelta2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sigmadelta2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sigmadelta2Val {
    bits: u32,
}

impl Sigmadelta2Val {
    /// Create from raw bits
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
        Self { bits: 0x0000ff00 }
    }

    /// Read **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn sd_prescale(&self) -> u32 {
        (self.bits & Sigmadelta2::SD_PRESCALE_MASK) >> Sigmadelta2::SD_PRESCALE_OFFSET
    }

    /// Read **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn sd_in(&self) -> u32 {
        (self.bits & Sigmadelta2::SD_IN_MASK) >> Sigmadelta2::SD_IN_OFFSET
    }

    /// Set **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn with_sd_prescale(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta2::SD_PRESCALE_MASK)
                | ((value << Sigmadelta2::SD_PRESCALE_OFFSET) & Sigmadelta2::SD_PRESCALE_MASK),
        }
    }

    /// Set **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn with_sd_in(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta2::SD_IN_MASK)
                | ((value << Sigmadelta2::SD_IN_OFFSET) & Sigmadelta2::SD_IN_MASK),
        }
    }
}

impl Default for Sigmadelta2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sigmadelta2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sigmadelta2Val {
        Sigmadelta2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sigmadelta2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sigmadelta2Val) -> Sigmadelta2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SIGMADELTA3 Register
///
/// Duty Cycle Configure Register of SDM3
///
/// Address offset: 0x000c
/// Reset value: 0x0000ff00
///
/// ## Fields
///
/// - **SD_PRESCALE** `[15:8]` (RW)
///
///   This field is used to set a divider value to divide APB clock.
///
/// - **SD_IN** `[7:0]` (RW)
///
///   This field is used to configure the duty cycle of sigma delta modulation output.
#[repr(transparent)]
pub struct Sigmadelta3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sigmadelta3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sigmadelta3 {
    // ======== Field Constants ========

    /// **SD_PRESCALE**
    ///
    /// This field is used to set a divider value to divide APB clock.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SD_PRESCALE_OFFSET: u32 = 8;
    pub const SD_PRESCALE_MASK: u32 = 0x0000ff00;

    /// **SD_IN**
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SD_IN_OFFSET: u32 = 0;
    pub const SD_IN_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn sd_prescale(&self) -> u32 {
        (self.value.read_bits() & Self::SD_PRESCALE_MASK) >> Self::SD_PRESCALE_OFFSET
    }

    /// Write **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn write_sd_prescale(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_PRESCALE_MASK)
                | ((value << Self::SD_PRESCALE_OFFSET) & Self::SD_PRESCALE_MASK)
        });
    }

    /// **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn sd_in(&self) -> u32 {
        (self.value.read_bits() & Self::SD_IN_MASK) >> Self::SD_IN_OFFSET
    }

    /// Write **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn write_sd_in(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_IN_MASK) | ((value << Self::SD_IN_OFFSET) & Self::SD_IN_MASK)
        });
    }
}

/// SIGMADELTA3 value type for batch read/modify/write operations
///
/// Use `sigmadelta3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sigmadelta3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sigmadelta3Val {
    bits: u32,
}

impl Sigmadelta3Val {
    /// Create from raw bits
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
        Self { bits: 0x0000ff00 }
    }

    /// Read **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn sd_prescale(&self) -> u32 {
        (self.bits & Sigmadelta3::SD_PRESCALE_MASK) >> Sigmadelta3::SD_PRESCALE_OFFSET
    }

    /// Read **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn sd_in(&self) -> u32 {
        (self.bits & Sigmadelta3::SD_IN_MASK) >> Sigmadelta3::SD_IN_OFFSET
    }

    /// Set **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn with_sd_prescale(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta3::SD_PRESCALE_MASK)
                | ((value << Sigmadelta3::SD_PRESCALE_OFFSET) & Sigmadelta3::SD_PRESCALE_MASK),
        }
    }

    /// Set **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn with_sd_in(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta3::SD_IN_MASK)
                | ((value << Sigmadelta3::SD_IN_OFFSET) & Sigmadelta3::SD_IN_MASK),
        }
    }
}

impl Default for Sigmadelta3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sigmadelta3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sigmadelta3Val {
        Sigmadelta3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sigmadelta3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sigmadelta3Val) -> Sigmadelta3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SIGMADELTA4 Register
///
/// Duty Cycle Configure Register of SDM4
///
/// Address offset: 0x0010
/// Reset value: 0x0000ff00
///
/// ## Fields
///
/// - **SD_PRESCALE** `[15:8]` (RW)
///
///   This field is used to set a divider value to divide APB clock.
///
/// - **SD_IN** `[7:0]` (RW)
///
///   This field is used to configure the duty cycle of sigma delta modulation output.
#[repr(transparent)]
pub struct Sigmadelta4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sigmadelta4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sigmadelta4 {
    // ======== Field Constants ========

    /// **SD_PRESCALE**
    ///
    /// This field is used to set a divider value to divide APB clock.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SD_PRESCALE_OFFSET: u32 = 8;
    pub const SD_PRESCALE_MASK: u32 = 0x0000ff00;

    /// **SD_IN**
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SD_IN_OFFSET: u32 = 0;
    pub const SD_IN_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn sd_prescale(&self) -> u32 {
        (self.value.read_bits() & Self::SD_PRESCALE_MASK) >> Self::SD_PRESCALE_OFFSET
    }

    /// Write **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn write_sd_prescale(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_PRESCALE_MASK)
                | ((value << Self::SD_PRESCALE_OFFSET) & Self::SD_PRESCALE_MASK)
        });
    }

    /// **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn sd_in(&self) -> u32 {
        (self.value.read_bits() & Self::SD_IN_MASK) >> Self::SD_IN_OFFSET
    }

    /// Write **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn write_sd_in(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_IN_MASK) | ((value << Self::SD_IN_OFFSET) & Self::SD_IN_MASK)
        });
    }
}

/// SIGMADELTA4 value type for batch read/modify/write operations
///
/// Use `sigmadelta4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sigmadelta4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sigmadelta4Val {
    bits: u32,
}

impl Sigmadelta4Val {
    /// Create from raw bits
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
        Self { bits: 0x0000ff00 }
    }

    /// Read **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn sd_prescale(&self) -> u32 {
        (self.bits & Sigmadelta4::SD_PRESCALE_MASK) >> Sigmadelta4::SD_PRESCALE_OFFSET
    }

    /// Read **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn sd_in(&self) -> u32 {
        (self.bits & Sigmadelta4::SD_IN_MASK) >> Sigmadelta4::SD_IN_OFFSET
    }

    /// Set **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn with_sd_prescale(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta4::SD_PRESCALE_MASK)
                | ((value << Sigmadelta4::SD_PRESCALE_OFFSET) & Sigmadelta4::SD_PRESCALE_MASK),
        }
    }

    /// Set **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn with_sd_in(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta4::SD_IN_MASK)
                | ((value << Sigmadelta4::SD_IN_OFFSET) & Sigmadelta4::SD_IN_MASK),
        }
    }
}

impl Default for Sigmadelta4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sigmadelta4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sigmadelta4Val {
        Sigmadelta4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sigmadelta4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sigmadelta4Val) -> Sigmadelta4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SIGMADELTA5 Register
///
/// Duty Cycle Configure Register of SDM5
///
/// Address offset: 0x0014
/// Reset value: 0x0000ff00
///
/// ## Fields
///
/// - **SD_PRESCALE** `[15:8]` (RW)
///
///   This field is used to set a divider value to divide APB clock.
///
/// - **SD_IN** `[7:0]` (RW)
///
///   This field is used to configure the duty cycle of sigma delta modulation output.
#[repr(transparent)]
pub struct Sigmadelta5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sigmadelta5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sigmadelta5 {
    // ======== Field Constants ========

    /// **SD_PRESCALE**
    ///
    /// This field is used to set a divider value to divide APB clock.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SD_PRESCALE_OFFSET: u32 = 8;
    pub const SD_PRESCALE_MASK: u32 = 0x0000ff00;

    /// **SD_IN**
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SD_IN_OFFSET: u32 = 0;
    pub const SD_IN_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn sd_prescale(&self) -> u32 {
        (self.value.read_bits() & Self::SD_PRESCALE_MASK) >> Self::SD_PRESCALE_OFFSET
    }

    /// Write **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn write_sd_prescale(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_PRESCALE_MASK)
                | ((value << Self::SD_PRESCALE_OFFSET) & Self::SD_PRESCALE_MASK)
        });
    }

    /// **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn sd_in(&self) -> u32 {
        (self.value.read_bits() & Self::SD_IN_MASK) >> Self::SD_IN_OFFSET
    }

    /// Write **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn write_sd_in(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_IN_MASK) | ((value << Self::SD_IN_OFFSET) & Self::SD_IN_MASK)
        });
    }
}

/// SIGMADELTA5 value type for batch read/modify/write operations
///
/// Use `sigmadelta5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sigmadelta5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sigmadelta5Val {
    bits: u32,
}

impl Sigmadelta5Val {
    /// Create from raw bits
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
        Self { bits: 0x0000ff00 }
    }

    /// Read **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn sd_prescale(&self) -> u32 {
        (self.bits & Sigmadelta5::SD_PRESCALE_MASK) >> Sigmadelta5::SD_PRESCALE_OFFSET
    }

    /// Read **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn sd_in(&self) -> u32 {
        (self.bits & Sigmadelta5::SD_IN_MASK) >> Sigmadelta5::SD_IN_OFFSET
    }

    /// Set **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn with_sd_prescale(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta5::SD_PRESCALE_MASK)
                | ((value << Sigmadelta5::SD_PRESCALE_OFFSET) & Sigmadelta5::SD_PRESCALE_MASK),
        }
    }

    /// Set **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn with_sd_in(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta5::SD_IN_MASK)
                | ((value << Sigmadelta5::SD_IN_OFFSET) & Sigmadelta5::SD_IN_MASK),
        }
    }
}

impl Default for Sigmadelta5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sigmadelta5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sigmadelta5Val {
        Sigmadelta5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sigmadelta5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sigmadelta5Val) -> Sigmadelta5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SIGMADELTA6 Register
///
/// Duty Cycle Configure Register of SDM6
///
/// Address offset: 0x0018
/// Reset value: 0x0000ff00
///
/// ## Fields
///
/// - **SD_PRESCALE** `[15:8]` (RW)
///
///   This field is used to set a divider value to divide APB clock.
///
/// - **SD_IN** `[7:0]` (RW)
///
///   This field is used to configure the duty cycle of sigma delta modulation output.
#[repr(transparent)]
pub struct Sigmadelta6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sigmadelta6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sigmadelta6 {
    // ======== Field Constants ========

    /// **SD_PRESCALE**
    ///
    /// This field is used to set a divider value to divide APB clock.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SD_PRESCALE_OFFSET: u32 = 8;
    pub const SD_PRESCALE_MASK: u32 = 0x0000ff00;

    /// **SD_IN**
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SD_IN_OFFSET: u32 = 0;
    pub const SD_IN_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn sd_prescale(&self) -> u32 {
        (self.value.read_bits() & Self::SD_PRESCALE_MASK) >> Self::SD_PRESCALE_OFFSET
    }

    /// Write **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn write_sd_prescale(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_PRESCALE_MASK)
                | ((value << Self::SD_PRESCALE_OFFSET) & Self::SD_PRESCALE_MASK)
        });
    }

    /// **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn sd_in(&self) -> u32 {
        (self.value.read_bits() & Self::SD_IN_MASK) >> Self::SD_IN_OFFSET
    }

    /// Write **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn write_sd_in(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_IN_MASK) | ((value << Self::SD_IN_OFFSET) & Self::SD_IN_MASK)
        });
    }
}

/// SIGMADELTA6 value type for batch read/modify/write operations
///
/// Use `sigmadelta6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sigmadelta6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sigmadelta6Val {
    bits: u32,
}

impl Sigmadelta6Val {
    /// Create from raw bits
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
        Self { bits: 0x0000ff00 }
    }

    /// Read **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn sd_prescale(&self) -> u32 {
        (self.bits & Sigmadelta6::SD_PRESCALE_MASK) >> Sigmadelta6::SD_PRESCALE_OFFSET
    }

    /// Read **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn sd_in(&self) -> u32 {
        (self.bits & Sigmadelta6::SD_IN_MASK) >> Sigmadelta6::SD_IN_OFFSET
    }

    /// Set **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn with_sd_prescale(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta6::SD_PRESCALE_MASK)
                | ((value << Sigmadelta6::SD_PRESCALE_OFFSET) & Sigmadelta6::SD_PRESCALE_MASK),
        }
    }

    /// Set **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn with_sd_in(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta6::SD_IN_MASK)
                | ((value << Sigmadelta6::SD_IN_OFFSET) & Sigmadelta6::SD_IN_MASK),
        }
    }
}

impl Default for Sigmadelta6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sigmadelta6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sigmadelta6Val {
        Sigmadelta6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sigmadelta6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sigmadelta6Val) -> Sigmadelta6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SIGMADELTA7 Register
///
/// Duty Cycle Configure Register of SDM7
///
/// Address offset: 0x001c
/// Reset value: 0x0000ff00
///
/// ## Fields
///
/// - **SD_PRESCALE** `[15:8]` (RW)
///
///   This field is used to set a divider value to divide APB clock.
///
/// - **SD_IN** `[7:0]` (RW)
///
///   This field is used to configure the duty cycle of sigma delta modulation output.
#[repr(transparent)]
pub struct Sigmadelta7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Sigmadelta7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Sigmadelta7 {
    // ======== Field Constants ========

    /// **SD_PRESCALE**
    ///
    /// This field is used to set a divider value to divide APB clock.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const SD_PRESCALE_OFFSET: u32 = 8;
    pub const SD_PRESCALE_MASK: u32 = 0x0000ff00;

    /// **SD_IN**
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SD_IN_OFFSET: u32 = 0;
    pub const SD_IN_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn sd_prescale(&self) -> u32 {
        (self.value.read_bits() & Self::SD_PRESCALE_MASK) >> Self::SD_PRESCALE_OFFSET
    }

    /// Write **SD_PRESCALE** `[15:8]` (RW)
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn write_sd_prescale(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_PRESCALE_MASK)
                | ((value << Self::SD_PRESCALE_OFFSET) & Self::SD_PRESCALE_MASK)
        });
    }

    /// **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn sd_in(&self) -> u32 {
        (self.value.read_bits() & Self::SD_IN_MASK) >> Self::SD_IN_OFFSET
    }

    /// Write **SD_IN** `[7:0]` (RW)
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn write_sd_in(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SD_IN_MASK) | ((value << Self::SD_IN_OFFSET) & Self::SD_IN_MASK)
        });
    }
}

/// SIGMADELTA7 value type for batch read/modify/write operations
///
/// Use `sigmadelta7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sigmadelta7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Sigmadelta7Val {
    bits: u32,
}

impl Sigmadelta7Val {
    /// Create from raw bits
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
        Self { bits: 0x0000ff00 }
    }

    /// Read **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn sd_prescale(&self) -> u32 {
        (self.bits & Sigmadelta7::SD_PRESCALE_MASK) >> Sigmadelta7::SD_PRESCALE_OFFSET
    }

    /// Read **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn sd_in(&self) -> u32 {
        (self.bits & Sigmadelta7::SD_IN_MASK) >> Sigmadelta7::SD_IN_OFFSET
    }

    /// Set **SD_PRESCALE** `[15:8]`
    ///
    /// This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub const fn with_sd_prescale(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta7::SD_PRESCALE_MASK)
                | ((value << Sigmadelta7::SD_PRESCALE_OFFSET) & Sigmadelta7::SD_PRESCALE_MASK),
        }
    }

    /// Set **SD_IN** `[7:0]`
    ///
    /// This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub const fn with_sd_in(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Sigmadelta7::SD_IN_MASK)
                | ((value << Sigmadelta7::SD_IN_OFFSET) & Sigmadelta7::SD_IN_MASK),
        }
    }
}

impl Default for Sigmadelta7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Sigmadelta7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Sigmadelta7Val {
        Sigmadelta7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Sigmadelta7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Sigmadelta7Val) -> Sigmadelta7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLOCK_GATE Register
///
/// Clock Gating Configure Register
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **CLK_EN** `[0]` (RW)
///
///   Clock enable bit of configuration registers for sigma delta modulation.
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

    /// **CLK_EN**
    ///
    /// Clock enable bit of configuration registers for sigma delta modulation.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_EN_OFFSET: u32 = 0;
    pub const CLK_EN_MASK: u32 = 0x00000001;
    pub const CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[0]` (RW)
    ///
    /// Clock enable bit of configuration registers for sigma delta modulation.
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

    /// Read **CLK_EN** `[0:0]`
    ///
    /// Clock enable bit of configuration registers for sigma delta modulation.
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & ClockGate::CLK_EN_MASK) != 0
    }

    /// Set **CLK_EN** `[0:0]`
    ///
    /// Clock enable bit of configuration registers for sigma delta modulation.
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClockGate::CLK_EN_MASK
            } else {
                self.bits & !ClockGate::CLK_EN_MASK
            },
        }
    }
}

impl Default for ClockGateVal {
    fn default() -> Self {
        Self { bits: 0 }
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

/// SIGMADELTA_MISC Register
///
/// MISC Register
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **SPI_SWAP** `[31]` (RW)
///
///   Reserved.
///
/// - **FUNCTION_CLK_EN** `[30]` (RW)
///
///   Clock enable bit of sigma delta modulation.
#[repr(transparent)]
pub struct SigmadeltaMisc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SigmadeltaMisc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SigmadeltaMisc {
    // ======== Field Constants ========

    /// **SPI_SWAP**
    ///
    /// Reserved.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SPI_SWAP_OFFSET: u32 = 31;
    pub const SPI_SWAP_MASK: u32 = 0x80000000;
    pub const SPI_SWAP: u32 = 1 << 31;

    /// **FUNCTION_CLK_EN**
    ///
    /// Clock enable bit of sigma delta modulation.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const FUNCTION_CLK_EN_OFFSET: u32 = 30;
    pub const FUNCTION_CLK_EN_MASK: u32 = 0x40000000;
    pub const FUNCTION_CLK_EN: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// **SPI_SWAP** `[31]` (RW)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn spi_swap(&self) -> bool {
        (self.value.read_bits() & Self::SPI_SWAP_MASK) != 0
    }

    /// Set SPI_SWAP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_spi_swap(&self) {
        self.value.modify_bits(|r| r | Self::SPI_SWAP_MASK);
    }

    /// Clear SPI_SWAP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_spi_swap(&self) {
        self.value.modify_bits(|r| r & !Self::SPI_SWAP_MASK);
    }

    /// **FUNCTION_CLK_EN** `[30]` (RW)
    ///
    /// Clock enable bit of sigma delta modulation.
    #[inline(always)]
    pub fn function_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::FUNCTION_CLK_EN_MASK) != 0
    }

    /// Set FUNCTION_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_function_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::FUNCTION_CLK_EN_MASK);
    }

    /// Clear FUNCTION_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_function_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::FUNCTION_CLK_EN_MASK);
    }
}

/// SIGMADELTA_MISC value type for batch read/modify/write operations
///
/// Use `sigmadeltamisc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sigmadeltamisc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SigmadeltaMiscVal {
    bits: u32,
}

impl SigmadeltaMiscVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SPI_SWAP** `[31:31]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn spi_swap(&self) -> bool {
        (self.bits & SigmadeltaMisc::SPI_SWAP_MASK) != 0
    }

    /// Read **FUNCTION_CLK_EN** `[30:30]`
    ///
    /// Clock enable bit of sigma delta modulation.
    #[inline(always)]
    pub const fn function_clk_en(&self) -> bool {
        (self.bits & SigmadeltaMisc::FUNCTION_CLK_EN_MASK) != 0
    }

    /// Set **SPI_SWAP** `[31:31]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn with_spi_swap(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SigmadeltaMisc::SPI_SWAP_MASK
            } else {
                self.bits & !SigmadeltaMisc::SPI_SWAP_MASK
            },
        }
    }

    /// Set **FUNCTION_CLK_EN** `[30:30]`
    ///
    /// Clock enable bit of sigma delta modulation.
    #[inline(always)]
    pub const fn with_function_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SigmadeltaMisc::FUNCTION_CLK_EN_MASK
            } else {
                self.bits & !SigmadeltaMisc::FUNCTION_CLK_EN_MASK
            },
        }
    }
}

impl Default for SigmadeltaMiscVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SigmadeltaMisc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SigmadeltaMiscVal {
        SigmadeltaMiscVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SigmadeltaMiscVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SigmadeltaMiscVal) -> SigmadeltaMiscVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GLITCH_FILTER_CH0 Register
///
/// Glitch Filter Configure Register of Channel0
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
///
///   Glitch Filter window width.
///
/// - **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
///
///   Glitch Filter window threshold.
///
/// - **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
///
///   Glitch Filter input io number.
///
/// - **FILTER_CH0_EN** `[0]` (RW)
///
///   Glitch Filter channel enable bit.
#[repr(transparent)]
pub struct GlitchFilterCh0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GlitchFilterCh0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GlitchFilterCh0 {
    // ======== Field Constants ========

    /// **FILTER_CH0_WINDOW_WIDTH**
    ///
    /// Glitch Filter window width.
    ///
    /// Bits `[18:13]`, mask `0x0007e000`
    pub const FILTER_CH0_WINDOW_WIDTH_OFFSET: u32 = 13;
    pub const FILTER_CH0_WINDOW_WIDTH_MASK: u32 = 0x0007e000;

    /// **FILTER_CH0_WINDOW_THRES**
    ///
    /// Glitch Filter window threshold.
    ///
    /// Bits `[12:7]`, mask `0x00001f80`
    pub const FILTER_CH0_WINDOW_THRES_OFFSET: u32 = 7;
    pub const FILTER_CH0_WINDOW_THRES_MASK: u32 = 0x00001f80;

    /// **FILTER_CH0_INPUT_IO_NUM**
    ///
    /// Glitch Filter input io number.
    ///
    /// Bits `[6:1]`, mask `0x0000007e`
    pub const FILTER_CH0_INPUT_IO_NUM_OFFSET: u32 = 1;
    pub const FILTER_CH0_INPUT_IO_NUM_MASK: u32 = 0x0000007e;

    /// **FILTER_CH0_EN**
    ///
    /// Glitch Filter channel enable bit.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FILTER_CH0_EN_OFFSET: u32 = 0;
    pub const FILTER_CH0_EN_MASK: u32 = 0x00000001;
    pub const FILTER_CH0_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn filter_ch0_window_width(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> Self::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn write_filter_ch0_window_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
        });
    }

    /// **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn filter_ch0_window_thres(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_THRES_MASK)
            >> Self::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn write_filter_ch0_window_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & Self::FILTER_CH0_WINDOW_THRES_MASK)
        });
    }

    /// **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn filter_ch0_input_io_num(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> Self::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Write **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn write_filter_ch0_input_io_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << Self::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
        });
    }

    /// **FILTER_CH0_EN** `[0]` (RW)
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub fn filter_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::FILTER_CH0_EN_MASK) != 0
    }

    /// Set FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r | Self::FILTER_CH0_EN_MASK);
    }

    /// Clear FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r & !Self::FILTER_CH0_EN_MASK);
    }
}

/// GLITCH_FILTER_CH0 value type for batch read/modify/write operations
///
/// Use `glitchfilterch0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `glitchfilterch0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GlitchFilterCh0Val {
    bits: u32,
}

impl GlitchFilterCh0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn filter_ch0_window_width(&self) -> u32 {
        (self.bits & GlitchFilterCh0::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> GlitchFilterCh0::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Read **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn filter_ch0_window_thres(&self) -> u32 {
        (self.bits & GlitchFilterCh0::FILTER_CH0_WINDOW_THRES_MASK)
            >> GlitchFilterCh0::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Read **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn filter_ch0_input_io_num(&self) -> u32 {
        (self.bits & GlitchFilterCh0::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> GlitchFilterCh0::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Read **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn filter_ch0_en(&self) -> bool {
        (self.bits & GlitchFilterCh0::FILTER_CH0_EN_MASK) != 0
    }

    /// Set **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn with_filter_ch0_window_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh0::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << GlitchFilterCh0::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & GlitchFilterCh0::FILTER_CH0_WINDOW_WIDTH_MASK),
        }
    }

    /// Set **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn with_filter_ch0_window_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh0::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << GlitchFilterCh0::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & GlitchFilterCh0::FILTER_CH0_WINDOW_THRES_MASK),
        }
    }

    /// Set **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn with_filter_ch0_input_io_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh0::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << GlitchFilterCh0::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & GlitchFilterCh0::FILTER_CH0_INPUT_IO_NUM_MASK),
        }
    }

    /// Set **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn with_filter_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GlitchFilterCh0::FILTER_CH0_EN_MASK
            } else {
                self.bits & !GlitchFilterCh0::FILTER_CH0_EN_MASK
            },
        }
    }
}

impl Default for GlitchFilterCh0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GlitchFilterCh0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GlitchFilterCh0Val {
        GlitchFilterCh0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GlitchFilterCh0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GlitchFilterCh0Val) -> GlitchFilterCh0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GLITCH_FILTER_CH1 Register
///
/// Glitch Filter Configure Register of Channel1
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
///
///   Glitch Filter window width.
///
/// - **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
///
///   Glitch Filter window threshold.
///
/// - **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
///
///   Glitch Filter input io number.
///
/// - **FILTER_CH0_EN** `[0]` (RW)
///
///   Glitch Filter channel enable bit.
#[repr(transparent)]
pub struct GlitchFilterCh1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GlitchFilterCh1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GlitchFilterCh1 {
    // ======== Field Constants ========

    /// **FILTER_CH0_WINDOW_WIDTH**
    ///
    /// Glitch Filter window width.
    ///
    /// Bits `[18:13]`, mask `0x0007e000`
    pub const FILTER_CH0_WINDOW_WIDTH_OFFSET: u32 = 13;
    pub const FILTER_CH0_WINDOW_WIDTH_MASK: u32 = 0x0007e000;

    /// **FILTER_CH0_WINDOW_THRES**
    ///
    /// Glitch Filter window threshold.
    ///
    /// Bits `[12:7]`, mask `0x00001f80`
    pub const FILTER_CH0_WINDOW_THRES_OFFSET: u32 = 7;
    pub const FILTER_CH0_WINDOW_THRES_MASK: u32 = 0x00001f80;

    /// **FILTER_CH0_INPUT_IO_NUM**
    ///
    /// Glitch Filter input io number.
    ///
    /// Bits `[6:1]`, mask `0x0000007e`
    pub const FILTER_CH0_INPUT_IO_NUM_OFFSET: u32 = 1;
    pub const FILTER_CH0_INPUT_IO_NUM_MASK: u32 = 0x0000007e;

    /// **FILTER_CH0_EN**
    ///
    /// Glitch Filter channel enable bit.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FILTER_CH0_EN_OFFSET: u32 = 0;
    pub const FILTER_CH0_EN_MASK: u32 = 0x00000001;
    pub const FILTER_CH0_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn filter_ch0_window_width(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> Self::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn write_filter_ch0_window_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
        });
    }

    /// **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn filter_ch0_window_thres(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_THRES_MASK)
            >> Self::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn write_filter_ch0_window_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & Self::FILTER_CH0_WINDOW_THRES_MASK)
        });
    }

    /// **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn filter_ch0_input_io_num(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> Self::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Write **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn write_filter_ch0_input_io_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << Self::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
        });
    }

    /// **FILTER_CH0_EN** `[0]` (RW)
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub fn filter_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::FILTER_CH0_EN_MASK) != 0
    }

    /// Set FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r | Self::FILTER_CH0_EN_MASK);
    }

    /// Clear FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r & !Self::FILTER_CH0_EN_MASK);
    }
}

/// GLITCH_FILTER_CH1 value type for batch read/modify/write operations
///
/// Use `glitchfilterch1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `glitchfilterch1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GlitchFilterCh1Val {
    bits: u32,
}

impl GlitchFilterCh1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn filter_ch0_window_width(&self) -> u32 {
        (self.bits & GlitchFilterCh1::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> GlitchFilterCh1::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Read **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn filter_ch0_window_thres(&self) -> u32 {
        (self.bits & GlitchFilterCh1::FILTER_CH0_WINDOW_THRES_MASK)
            >> GlitchFilterCh1::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Read **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn filter_ch0_input_io_num(&self) -> u32 {
        (self.bits & GlitchFilterCh1::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> GlitchFilterCh1::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Read **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn filter_ch0_en(&self) -> bool {
        (self.bits & GlitchFilterCh1::FILTER_CH0_EN_MASK) != 0
    }

    /// Set **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn with_filter_ch0_window_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh1::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << GlitchFilterCh1::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & GlitchFilterCh1::FILTER_CH0_WINDOW_WIDTH_MASK),
        }
    }

    /// Set **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn with_filter_ch0_window_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh1::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << GlitchFilterCh1::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & GlitchFilterCh1::FILTER_CH0_WINDOW_THRES_MASK),
        }
    }

    /// Set **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn with_filter_ch0_input_io_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh1::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << GlitchFilterCh1::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & GlitchFilterCh1::FILTER_CH0_INPUT_IO_NUM_MASK),
        }
    }

    /// Set **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn with_filter_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GlitchFilterCh1::FILTER_CH0_EN_MASK
            } else {
                self.bits & !GlitchFilterCh1::FILTER_CH0_EN_MASK
            },
        }
    }
}

impl Default for GlitchFilterCh1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GlitchFilterCh1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GlitchFilterCh1Val {
        GlitchFilterCh1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GlitchFilterCh1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GlitchFilterCh1Val) -> GlitchFilterCh1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GLITCH_FILTER_CH2 Register
///
/// Glitch Filter Configure Register of Channel2
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
///
///   Glitch Filter window width.
///
/// - **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
///
///   Glitch Filter window threshold.
///
/// - **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
///
///   Glitch Filter input io number.
///
/// - **FILTER_CH0_EN** `[0]` (RW)
///
///   Glitch Filter channel enable bit.
#[repr(transparent)]
pub struct GlitchFilterCh2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GlitchFilterCh2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GlitchFilterCh2 {
    // ======== Field Constants ========

    /// **FILTER_CH0_WINDOW_WIDTH**
    ///
    /// Glitch Filter window width.
    ///
    /// Bits `[18:13]`, mask `0x0007e000`
    pub const FILTER_CH0_WINDOW_WIDTH_OFFSET: u32 = 13;
    pub const FILTER_CH0_WINDOW_WIDTH_MASK: u32 = 0x0007e000;

    /// **FILTER_CH0_WINDOW_THRES**
    ///
    /// Glitch Filter window threshold.
    ///
    /// Bits `[12:7]`, mask `0x00001f80`
    pub const FILTER_CH0_WINDOW_THRES_OFFSET: u32 = 7;
    pub const FILTER_CH0_WINDOW_THRES_MASK: u32 = 0x00001f80;

    /// **FILTER_CH0_INPUT_IO_NUM**
    ///
    /// Glitch Filter input io number.
    ///
    /// Bits `[6:1]`, mask `0x0000007e`
    pub const FILTER_CH0_INPUT_IO_NUM_OFFSET: u32 = 1;
    pub const FILTER_CH0_INPUT_IO_NUM_MASK: u32 = 0x0000007e;

    /// **FILTER_CH0_EN**
    ///
    /// Glitch Filter channel enable bit.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FILTER_CH0_EN_OFFSET: u32 = 0;
    pub const FILTER_CH0_EN_MASK: u32 = 0x00000001;
    pub const FILTER_CH0_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn filter_ch0_window_width(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> Self::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn write_filter_ch0_window_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
        });
    }

    /// **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn filter_ch0_window_thres(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_THRES_MASK)
            >> Self::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn write_filter_ch0_window_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & Self::FILTER_CH0_WINDOW_THRES_MASK)
        });
    }

    /// **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn filter_ch0_input_io_num(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> Self::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Write **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn write_filter_ch0_input_io_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << Self::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
        });
    }

    /// **FILTER_CH0_EN** `[0]` (RW)
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub fn filter_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::FILTER_CH0_EN_MASK) != 0
    }

    /// Set FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r | Self::FILTER_CH0_EN_MASK);
    }

    /// Clear FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r & !Self::FILTER_CH0_EN_MASK);
    }
}

/// GLITCH_FILTER_CH2 value type for batch read/modify/write operations
///
/// Use `glitchfilterch2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `glitchfilterch2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GlitchFilterCh2Val {
    bits: u32,
}

impl GlitchFilterCh2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn filter_ch0_window_width(&self) -> u32 {
        (self.bits & GlitchFilterCh2::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> GlitchFilterCh2::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Read **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn filter_ch0_window_thres(&self) -> u32 {
        (self.bits & GlitchFilterCh2::FILTER_CH0_WINDOW_THRES_MASK)
            >> GlitchFilterCh2::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Read **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn filter_ch0_input_io_num(&self) -> u32 {
        (self.bits & GlitchFilterCh2::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> GlitchFilterCh2::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Read **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn filter_ch0_en(&self) -> bool {
        (self.bits & GlitchFilterCh2::FILTER_CH0_EN_MASK) != 0
    }

    /// Set **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn with_filter_ch0_window_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh2::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << GlitchFilterCh2::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & GlitchFilterCh2::FILTER_CH0_WINDOW_WIDTH_MASK),
        }
    }

    /// Set **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn with_filter_ch0_window_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh2::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << GlitchFilterCh2::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & GlitchFilterCh2::FILTER_CH0_WINDOW_THRES_MASK),
        }
    }

    /// Set **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn with_filter_ch0_input_io_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh2::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << GlitchFilterCh2::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & GlitchFilterCh2::FILTER_CH0_INPUT_IO_NUM_MASK),
        }
    }

    /// Set **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn with_filter_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GlitchFilterCh2::FILTER_CH0_EN_MASK
            } else {
                self.bits & !GlitchFilterCh2::FILTER_CH0_EN_MASK
            },
        }
    }
}

impl Default for GlitchFilterCh2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GlitchFilterCh2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GlitchFilterCh2Val {
        GlitchFilterCh2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GlitchFilterCh2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GlitchFilterCh2Val) -> GlitchFilterCh2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GLITCH_FILTER_CH3 Register
///
/// Glitch Filter Configure Register of Channel3
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
///
///   Glitch Filter window width.
///
/// - **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
///
///   Glitch Filter window threshold.
///
/// - **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
///
///   Glitch Filter input io number.
///
/// - **FILTER_CH0_EN** `[0]` (RW)
///
///   Glitch Filter channel enable bit.
#[repr(transparent)]
pub struct GlitchFilterCh3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GlitchFilterCh3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GlitchFilterCh3 {
    // ======== Field Constants ========

    /// **FILTER_CH0_WINDOW_WIDTH**
    ///
    /// Glitch Filter window width.
    ///
    /// Bits `[18:13]`, mask `0x0007e000`
    pub const FILTER_CH0_WINDOW_WIDTH_OFFSET: u32 = 13;
    pub const FILTER_CH0_WINDOW_WIDTH_MASK: u32 = 0x0007e000;

    /// **FILTER_CH0_WINDOW_THRES**
    ///
    /// Glitch Filter window threshold.
    ///
    /// Bits `[12:7]`, mask `0x00001f80`
    pub const FILTER_CH0_WINDOW_THRES_OFFSET: u32 = 7;
    pub const FILTER_CH0_WINDOW_THRES_MASK: u32 = 0x00001f80;

    /// **FILTER_CH0_INPUT_IO_NUM**
    ///
    /// Glitch Filter input io number.
    ///
    /// Bits `[6:1]`, mask `0x0000007e`
    pub const FILTER_CH0_INPUT_IO_NUM_OFFSET: u32 = 1;
    pub const FILTER_CH0_INPUT_IO_NUM_MASK: u32 = 0x0000007e;

    /// **FILTER_CH0_EN**
    ///
    /// Glitch Filter channel enable bit.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FILTER_CH0_EN_OFFSET: u32 = 0;
    pub const FILTER_CH0_EN_MASK: u32 = 0x00000001;
    pub const FILTER_CH0_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn filter_ch0_window_width(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> Self::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn write_filter_ch0_window_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
        });
    }

    /// **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn filter_ch0_window_thres(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_THRES_MASK)
            >> Self::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn write_filter_ch0_window_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & Self::FILTER_CH0_WINDOW_THRES_MASK)
        });
    }

    /// **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn filter_ch0_input_io_num(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> Self::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Write **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn write_filter_ch0_input_io_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << Self::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
        });
    }

    /// **FILTER_CH0_EN** `[0]` (RW)
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub fn filter_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::FILTER_CH0_EN_MASK) != 0
    }

    /// Set FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r | Self::FILTER_CH0_EN_MASK);
    }

    /// Clear FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r & !Self::FILTER_CH0_EN_MASK);
    }
}

/// GLITCH_FILTER_CH3 value type for batch read/modify/write operations
///
/// Use `glitchfilterch3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `glitchfilterch3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GlitchFilterCh3Val {
    bits: u32,
}

impl GlitchFilterCh3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn filter_ch0_window_width(&self) -> u32 {
        (self.bits & GlitchFilterCh3::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> GlitchFilterCh3::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Read **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn filter_ch0_window_thres(&self) -> u32 {
        (self.bits & GlitchFilterCh3::FILTER_CH0_WINDOW_THRES_MASK)
            >> GlitchFilterCh3::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Read **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn filter_ch0_input_io_num(&self) -> u32 {
        (self.bits & GlitchFilterCh3::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> GlitchFilterCh3::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Read **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn filter_ch0_en(&self) -> bool {
        (self.bits & GlitchFilterCh3::FILTER_CH0_EN_MASK) != 0
    }

    /// Set **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn with_filter_ch0_window_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh3::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << GlitchFilterCh3::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & GlitchFilterCh3::FILTER_CH0_WINDOW_WIDTH_MASK),
        }
    }

    /// Set **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn with_filter_ch0_window_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh3::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << GlitchFilterCh3::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & GlitchFilterCh3::FILTER_CH0_WINDOW_THRES_MASK),
        }
    }

    /// Set **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn with_filter_ch0_input_io_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh3::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << GlitchFilterCh3::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & GlitchFilterCh3::FILTER_CH0_INPUT_IO_NUM_MASK),
        }
    }

    /// Set **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn with_filter_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GlitchFilterCh3::FILTER_CH0_EN_MASK
            } else {
                self.bits & !GlitchFilterCh3::FILTER_CH0_EN_MASK
            },
        }
    }
}

impl Default for GlitchFilterCh3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GlitchFilterCh3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GlitchFilterCh3Val {
        GlitchFilterCh3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GlitchFilterCh3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GlitchFilterCh3Val) -> GlitchFilterCh3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GLITCH_FILTER_CH4 Register
///
/// Glitch Filter Configure Register of Channel4
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
///
///   Glitch Filter window width.
///
/// - **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
///
///   Glitch Filter window threshold.
///
/// - **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
///
///   Glitch Filter input io number.
///
/// - **FILTER_CH0_EN** `[0]` (RW)
///
///   Glitch Filter channel enable bit.
#[repr(transparent)]
pub struct GlitchFilterCh4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GlitchFilterCh4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GlitchFilterCh4 {
    // ======== Field Constants ========

    /// **FILTER_CH0_WINDOW_WIDTH**
    ///
    /// Glitch Filter window width.
    ///
    /// Bits `[18:13]`, mask `0x0007e000`
    pub const FILTER_CH0_WINDOW_WIDTH_OFFSET: u32 = 13;
    pub const FILTER_CH0_WINDOW_WIDTH_MASK: u32 = 0x0007e000;

    /// **FILTER_CH0_WINDOW_THRES**
    ///
    /// Glitch Filter window threshold.
    ///
    /// Bits `[12:7]`, mask `0x00001f80`
    pub const FILTER_CH0_WINDOW_THRES_OFFSET: u32 = 7;
    pub const FILTER_CH0_WINDOW_THRES_MASK: u32 = 0x00001f80;

    /// **FILTER_CH0_INPUT_IO_NUM**
    ///
    /// Glitch Filter input io number.
    ///
    /// Bits `[6:1]`, mask `0x0000007e`
    pub const FILTER_CH0_INPUT_IO_NUM_OFFSET: u32 = 1;
    pub const FILTER_CH0_INPUT_IO_NUM_MASK: u32 = 0x0000007e;

    /// **FILTER_CH0_EN**
    ///
    /// Glitch Filter channel enable bit.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FILTER_CH0_EN_OFFSET: u32 = 0;
    pub const FILTER_CH0_EN_MASK: u32 = 0x00000001;
    pub const FILTER_CH0_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn filter_ch0_window_width(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> Self::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn write_filter_ch0_window_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
        });
    }

    /// **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn filter_ch0_window_thres(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_THRES_MASK)
            >> Self::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn write_filter_ch0_window_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & Self::FILTER_CH0_WINDOW_THRES_MASK)
        });
    }

    /// **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn filter_ch0_input_io_num(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> Self::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Write **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn write_filter_ch0_input_io_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << Self::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
        });
    }

    /// **FILTER_CH0_EN** `[0]` (RW)
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub fn filter_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::FILTER_CH0_EN_MASK) != 0
    }

    /// Set FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r | Self::FILTER_CH0_EN_MASK);
    }

    /// Clear FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r & !Self::FILTER_CH0_EN_MASK);
    }
}

/// GLITCH_FILTER_CH4 value type for batch read/modify/write operations
///
/// Use `glitchfilterch4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `glitchfilterch4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GlitchFilterCh4Val {
    bits: u32,
}

impl GlitchFilterCh4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn filter_ch0_window_width(&self) -> u32 {
        (self.bits & GlitchFilterCh4::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> GlitchFilterCh4::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Read **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn filter_ch0_window_thres(&self) -> u32 {
        (self.bits & GlitchFilterCh4::FILTER_CH0_WINDOW_THRES_MASK)
            >> GlitchFilterCh4::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Read **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn filter_ch0_input_io_num(&self) -> u32 {
        (self.bits & GlitchFilterCh4::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> GlitchFilterCh4::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Read **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn filter_ch0_en(&self) -> bool {
        (self.bits & GlitchFilterCh4::FILTER_CH0_EN_MASK) != 0
    }

    /// Set **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn with_filter_ch0_window_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh4::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << GlitchFilterCh4::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & GlitchFilterCh4::FILTER_CH0_WINDOW_WIDTH_MASK),
        }
    }

    /// Set **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn with_filter_ch0_window_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh4::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << GlitchFilterCh4::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & GlitchFilterCh4::FILTER_CH0_WINDOW_THRES_MASK),
        }
    }

    /// Set **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn with_filter_ch0_input_io_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh4::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << GlitchFilterCh4::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & GlitchFilterCh4::FILTER_CH0_INPUT_IO_NUM_MASK),
        }
    }

    /// Set **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn with_filter_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GlitchFilterCh4::FILTER_CH0_EN_MASK
            } else {
                self.bits & !GlitchFilterCh4::FILTER_CH0_EN_MASK
            },
        }
    }
}

impl Default for GlitchFilterCh4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GlitchFilterCh4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GlitchFilterCh4Val {
        GlitchFilterCh4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GlitchFilterCh4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GlitchFilterCh4Val) -> GlitchFilterCh4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GLITCH_FILTER_CH5 Register
///
/// Glitch Filter Configure Register of Channel5
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
///
///   Glitch Filter window width.
///
/// - **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
///
///   Glitch Filter window threshold.
///
/// - **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
///
///   Glitch Filter input io number.
///
/// - **FILTER_CH0_EN** `[0]` (RW)
///
///   Glitch Filter channel enable bit.
#[repr(transparent)]
pub struct GlitchFilterCh5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GlitchFilterCh5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GlitchFilterCh5 {
    // ======== Field Constants ========

    /// **FILTER_CH0_WINDOW_WIDTH**
    ///
    /// Glitch Filter window width.
    ///
    /// Bits `[18:13]`, mask `0x0007e000`
    pub const FILTER_CH0_WINDOW_WIDTH_OFFSET: u32 = 13;
    pub const FILTER_CH0_WINDOW_WIDTH_MASK: u32 = 0x0007e000;

    /// **FILTER_CH0_WINDOW_THRES**
    ///
    /// Glitch Filter window threshold.
    ///
    /// Bits `[12:7]`, mask `0x00001f80`
    pub const FILTER_CH0_WINDOW_THRES_OFFSET: u32 = 7;
    pub const FILTER_CH0_WINDOW_THRES_MASK: u32 = 0x00001f80;

    /// **FILTER_CH0_INPUT_IO_NUM**
    ///
    /// Glitch Filter input io number.
    ///
    /// Bits `[6:1]`, mask `0x0000007e`
    pub const FILTER_CH0_INPUT_IO_NUM_OFFSET: u32 = 1;
    pub const FILTER_CH0_INPUT_IO_NUM_MASK: u32 = 0x0000007e;

    /// **FILTER_CH0_EN**
    ///
    /// Glitch Filter channel enable bit.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FILTER_CH0_EN_OFFSET: u32 = 0;
    pub const FILTER_CH0_EN_MASK: u32 = 0x00000001;
    pub const FILTER_CH0_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn filter_ch0_window_width(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> Self::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn write_filter_ch0_window_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
        });
    }

    /// **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn filter_ch0_window_thres(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_THRES_MASK)
            >> Self::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn write_filter_ch0_window_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & Self::FILTER_CH0_WINDOW_THRES_MASK)
        });
    }

    /// **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn filter_ch0_input_io_num(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> Self::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Write **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn write_filter_ch0_input_io_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << Self::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
        });
    }

    /// **FILTER_CH0_EN** `[0]` (RW)
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub fn filter_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::FILTER_CH0_EN_MASK) != 0
    }

    /// Set FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r | Self::FILTER_CH0_EN_MASK);
    }

    /// Clear FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r & !Self::FILTER_CH0_EN_MASK);
    }
}

/// GLITCH_FILTER_CH5 value type for batch read/modify/write operations
///
/// Use `glitchfilterch5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `glitchfilterch5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GlitchFilterCh5Val {
    bits: u32,
}

impl GlitchFilterCh5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn filter_ch0_window_width(&self) -> u32 {
        (self.bits & GlitchFilterCh5::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> GlitchFilterCh5::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Read **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn filter_ch0_window_thres(&self) -> u32 {
        (self.bits & GlitchFilterCh5::FILTER_CH0_WINDOW_THRES_MASK)
            >> GlitchFilterCh5::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Read **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn filter_ch0_input_io_num(&self) -> u32 {
        (self.bits & GlitchFilterCh5::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> GlitchFilterCh5::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Read **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn filter_ch0_en(&self) -> bool {
        (self.bits & GlitchFilterCh5::FILTER_CH0_EN_MASK) != 0
    }

    /// Set **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn with_filter_ch0_window_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh5::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << GlitchFilterCh5::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & GlitchFilterCh5::FILTER_CH0_WINDOW_WIDTH_MASK),
        }
    }

    /// Set **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn with_filter_ch0_window_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh5::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << GlitchFilterCh5::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & GlitchFilterCh5::FILTER_CH0_WINDOW_THRES_MASK),
        }
    }

    /// Set **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn with_filter_ch0_input_io_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh5::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << GlitchFilterCh5::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & GlitchFilterCh5::FILTER_CH0_INPUT_IO_NUM_MASK),
        }
    }

    /// Set **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn with_filter_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GlitchFilterCh5::FILTER_CH0_EN_MASK
            } else {
                self.bits & !GlitchFilterCh5::FILTER_CH0_EN_MASK
            },
        }
    }
}

impl Default for GlitchFilterCh5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GlitchFilterCh5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GlitchFilterCh5Val {
        GlitchFilterCh5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GlitchFilterCh5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GlitchFilterCh5Val) -> GlitchFilterCh5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GLITCH_FILTER_CH6 Register
///
/// Glitch Filter Configure Register of Channel6
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
///
///   Glitch Filter window width.
///
/// - **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
///
///   Glitch Filter window threshold.
///
/// - **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
///
///   Glitch Filter input io number.
///
/// - **FILTER_CH0_EN** `[0]` (RW)
///
///   Glitch Filter channel enable bit.
#[repr(transparent)]
pub struct GlitchFilterCh6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GlitchFilterCh6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GlitchFilterCh6 {
    // ======== Field Constants ========

    /// **FILTER_CH0_WINDOW_WIDTH**
    ///
    /// Glitch Filter window width.
    ///
    /// Bits `[18:13]`, mask `0x0007e000`
    pub const FILTER_CH0_WINDOW_WIDTH_OFFSET: u32 = 13;
    pub const FILTER_CH0_WINDOW_WIDTH_MASK: u32 = 0x0007e000;

    /// **FILTER_CH0_WINDOW_THRES**
    ///
    /// Glitch Filter window threshold.
    ///
    /// Bits `[12:7]`, mask `0x00001f80`
    pub const FILTER_CH0_WINDOW_THRES_OFFSET: u32 = 7;
    pub const FILTER_CH0_WINDOW_THRES_MASK: u32 = 0x00001f80;

    /// **FILTER_CH0_INPUT_IO_NUM**
    ///
    /// Glitch Filter input io number.
    ///
    /// Bits `[6:1]`, mask `0x0000007e`
    pub const FILTER_CH0_INPUT_IO_NUM_OFFSET: u32 = 1;
    pub const FILTER_CH0_INPUT_IO_NUM_MASK: u32 = 0x0000007e;

    /// **FILTER_CH0_EN**
    ///
    /// Glitch Filter channel enable bit.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FILTER_CH0_EN_OFFSET: u32 = 0;
    pub const FILTER_CH0_EN_MASK: u32 = 0x00000001;
    pub const FILTER_CH0_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn filter_ch0_window_width(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> Self::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn write_filter_ch0_window_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
        });
    }

    /// **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn filter_ch0_window_thres(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_THRES_MASK)
            >> Self::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn write_filter_ch0_window_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & Self::FILTER_CH0_WINDOW_THRES_MASK)
        });
    }

    /// **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn filter_ch0_input_io_num(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> Self::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Write **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn write_filter_ch0_input_io_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << Self::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
        });
    }

    /// **FILTER_CH0_EN** `[0]` (RW)
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub fn filter_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::FILTER_CH0_EN_MASK) != 0
    }

    /// Set FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r | Self::FILTER_CH0_EN_MASK);
    }

    /// Clear FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r & !Self::FILTER_CH0_EN_MASK);
    }
}

/// GLITCH_FILTER_CH6 value type for batch read/modify/write operations
///
/// Use `glitchfilterch6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `glitchfilterch6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GlitchFilterCh6Val {
    bits: u32,
}

impl GlitchFilterCh6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn filter_ch0_window_width(&self) -> u32 {
        (self.bits & GlitchFilterCh6::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> GlitchFilterCh6::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Read **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn filter_ch0_window_thres(&self) -> u32 {
        (self.bits & GlitchFilterCh6::FILTER_CH0_WINDOW_THRES_MASK)
            >> GlitchFilterCh6::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Read **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn filter_ch0_input_io_num(&self) -> u32 {
        (self.bits & GlitchFilterCh6::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> GlitchFilterCh6::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Read **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn filter_ch0_en(&self) -> bool {
        (self.bits & GlitchFilterCh6::FILTER_CH0_EN_MASK) != 0
    }

    /// Set **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn with_filter_ch0_window_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh6::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << GlitchFilterCh6::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & GlitchFilterCh6::FILTER_CH0_WINDOW_WIDTH_MASK),
        }
    }

    /// Set **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn with_filter_ch0_window_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh6::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << GlitchFilterCh6::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & GlitchFilterCh6::FILTER_CH0_WINDOW_THRES_MASK),
        }
    }

    /// Set **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn with_filter_ch0_input_io_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh6::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << GlitchFilterCh6::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & GlitchFilterCh6::FILTER_CH0_INPUT_IO_NUM_MASK),
        }
    }

    /// Set **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn with_filter_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GlitchFilterCh6::FILTER_CH0_EN_MASK
            } else {
                self.bits & !GlitchFilterCh6::FILTER_CH0_EN_MASK
            },
        }
    }
}

impl Default for GlitchFilterCh6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GlitchFilterCh6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GlitchFilterCh6Val {
        GlitchFilterCh6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GlitchFilterCh6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GlitchFilterCh6Val) -> GlitchFilterCh6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GLITCH_FILTER_CH7 Register
///
/// Glitch Filter Configure Register of Channel7
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
///
///   Glitch Filter window width.
///
/// - **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
///
///   Glitch Filter window threshold.
///
/// - **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
///
///   Glitch Filter input io number.
///
/// - **FILTER_CH0_EN** `[0]` (RW)
///
///   Glitch Filter channel enable bit.
#[repr(transparent)]
pub struct GlitchFilterCh7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GlitchFilterCh7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GlitchFilterCh7 {
    // ======== Field Constants ========

    /// **FILTER_CH0_WINDOW_WIDTH**
    ///
    /// Glitch Filter window width.
    ///
    /// Bits `[18:13]`, mask `0x0007e000`
    pub const FILTER_CH0_WINDOW_WIDTH_OFFSET: u32 = 13;
    pub const FILTER_CH0_WINDOW_WIDTH_MASK: u32 = 0x0007e000;

    /// **FILTER_CH0_WINDOW_THRES**
    ///
    /// Glitch Filter window threshold.
    ///
    /// Bits `[12:7]`, mask `0x00001f80`
    pub const FILTER_CH0_WINDOW_THRES_OFFSET: u32 = 7;
    pub const FILTER_CH0_WINDOW_THRES_MASK: u32 = 0x00001f80;

    /// **FILTER_CH0_INPUT_IO_NUM**
    ///
    /// Glitch Filter input io number.
    ///
    /// Bits `[6:1]`, mask `0x0000007e`
    pub const FILTER_CH0_INPUT_IO_NUM_OFFSET: u32 = 1;
    pub const FILTER_CH0_INPUT_IO_NUM_MASK: u32 = 0x0000007e;

    /// **FILTER_CH0_EN**
    ///
    /// Glitch Filter channel enable bit.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FILTER_CH0_EN_OFFSET: u32 = 0;
    pub const FILTER_CH0_EN_MASK: u32 = 0x00000001;
    pub const FILTER_CH0_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn filter_ch0_window_width(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> Self::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_WIDTH** `[18:13]` (RW)
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub fn write_filter_ch0_window_width(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & Self::FILTER_CH0_WINDOW_WIDTH_MASK)
        });
    }

    /// **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn filter_ch0_window_thres(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_WINDOW_THRES_MASK)
            >> Self::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Write **FILTER_CH0_WINDOW_THRES** `[12:7]` (RW)
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub fn write_filter_ch0_window_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << Self::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & Self::FILTER_CH0_WINDOW_THRES_MASK)
        });
    }

    /// **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn filter_ch0_input_io_num(&self) -> u32 {
        (self.value.read_bits() & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> Self::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Write **FILTER_CH0_INPUT_IO_NUM** `[6:1]` (RW)
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub fn write_filter_ch0_input_io_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << Self::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & Self::FILTER_CH0_INPUT_IO_NUM_MASK)
        });
    }

    /// **FILTER_CH0_EN** `[0]` (RW)
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub fn filter_ch0_en(&self) -> bool {
        (self.value.read_bits() & Self::FILTER_CH0_EN_MASK) != 0
    }

    /// Set FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r | Self::FILTER_CH0_EN_MASK);
    }

    /// Clear FILTER_CH0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_filter_ch0_en(&self) {
        self.value.modify_bits(|r| r & !Self::FILTER_CH0_EN_MASK);
    }
}

/// GLITCH_FILTER_CH7 value type for batch read/modify/write operations
///
/// Use `glitchfilterch7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `glitchfilterch7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GlitchFilterCh7Val {
    bits: u32,
}

impl GlitchFilterCh7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn filter_ch0_window_width(&self) -> u32 {
        (self.bits & GlitchFilterCh7::FILTER_CH0_WINDOW_WIDTH_MASK)
            >> GlitchFilterCh7::FILTER_CH0_WINDOW_WIDTH_OFFSET
    }

    /// Read **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn filter_ch0_window_thres(&self) -> u32 {
        (self.bits & GlitchFilterCh7::FILTER_CH0_WINDOW_THRES_MASK)
            >> GlitchFilterCh7::FILTER_CH0_WINDOW_THRES_OFFSET
    }

    /// Read **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn filter_ch0_input_io_num(&self) -> u32 {
        (self.bits & GlitchFilterCh7::FILTER_CH0_INPUT_IO_NUM_MASK)
            >> GlitchFilterCh7::FILTER_CH0_INPUT_IO_NUM_OFFSET
    }

    /// Read **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn filter_ch0_en(&self) -> bool {
        (self.bits & GlitchFilterCh7::FILTER_CH0_EN_MASK) != 0
    }

    /// Set **FILTER_CH0_WINDOW_WIDTH** `[18:13]`
    ///
    /// Glitch Filter window width.
    #[inline(always)]
    pub const fn with_filter_ch0_window_width(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh7::FILTER_CH0_WINDOW_WIDTH_MASK)
                | ((value << GlitchFilterCh7::FILTER_CH0_WINDOW_WIDTH_OFFSET)
                    & GlitchFilterCh7::FILTER_CH0_WINDOW_WIDTH_MASK),
        }
    }

    /// Set **FILTER_CH0_WINDOW_THRES** `[12:7]`
    ///
    /// Glitch Filter window threshold.
    #[inline(always)]
    pub const fn with_filter_ch0_window_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh7::FILTER_CH0_WINDOW_THRES_MASK)
                | ((value << GlitchFilterCh7::FILTER_CH0_WINDOW_THRES_OFFSET)
                    & GlitchFilterCh7::FILTER_CH0_WINDOW_THRES_MASK),
        }
    }

    /// Set **FILTER_CH0_INPUT_IO_NUM** `[6:1]`
    ///
    /// Glitch Filter input io number.
    #[inline(always)]
    pub const fn with_filter_ch0_input_io_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GlitchFilterCh7::FILTER_CH0_INPUT_IO_NUM_MASK)
                | ((value << GlitchFilterCh7::FILTER_CH0_INPUT_IO_NUM_OFFSET)
                    & GlitchFilterCh7::FILTER_CH0_INPUT_IO_NUM_MASK),
        }
    }

    /// Set **FILTER_CH0_EN** `[0:0]`
    ///
    /// Glitch Filter channel enable bit.
    #[inline(always)]
    pub const fn with_filter_ch0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GlitchFilterCh7::FILTER_CH0_EN_MASK
            } else {
                self.bits & !GlitchFilterCh7::FILTER_CH0_EN_MASK
            },
        }
    }
}

impl Default for GlitchFilterCh7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GlitchFilterCh7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GlitchFilterCh7Val {
        GlitchFilterCh7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GlitchFilterCh7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GlitchFilterCh7Val) -> GlitchFilterCh7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_EVENT_CH0_CFG Register
///
/// Etm Config register of Channel0
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **ETM_CH0_EVENT_EN** `[7]` (RW)
///
///   Etm event send enable bit.
///
/// - **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
///
///   Etm event channel select gpio.
#[repr(transparent)]
pub struct EtmEventCh0Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmEventCh0Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmEventCh0Cfg {
    // ======== Field Constants ========

    /// **ETM_CH0_EVENT_EN**
    ///
    /// Etm event send enable bit.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ETM_CH0_EVENT_EN_OFFSET: u32 = 7;
    pub const ETM_CH0_EVENT_EN_MASK: u32 = 0x00000080;
    pub const ETM_CH0_EVENT_EN: u32 = 1 << 7;

    /// **ETM_CH0_EVENT_SEL**
    ///
    /// Etm event channel select gpio.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const ETM_CH0_EVENT_SEL_OFFSET: u32 = 0;
    pub const ETM_CH0_EVENT_SEL_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **ETM_CH0_EVENT_EN** `[7]` (RW)
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub fn etm_ch0_event_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Set ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// Clear ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r & !Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn etm_ch0_event_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_SEL_MASK) >> Self::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Write **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn write_etm_ch0_event_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_CH0_EVENT_SEL_MASK)
                | ((value << Self::ETM_CH0_EVENT_SEL_OFFSET) & Self::ETM_CH0_EVENT_SEL_MASK)
        });
    }
}

/// ETM_EVENT_CH0_CFG value type for batch read/modify/write operations
///
/// Use `etmeventch0cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmeventch0cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmEventCh0CfgVal {
    bits: u32,
}

impl EtmEventCh0CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn etm_ch0_event_en(&self) -> bool {
        (self.bits & EtmEventCh0Cfg::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Read **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn etm_ch0_event_sel(&self) -> u32 {
        (self.bits & EtmEventCh0Cfg::ETM_CH0_EVENT_SEL_MASK)
            >> EtmEventCh0Cfg::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Set **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn with_etm_ch0_event_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmEventCh0Cfg::ETM_CH0_EVENT_EN_MASK
            } else {
                self.bits & !EtmEventCh0Cfg::ETM_CH0_EVENT_EN_MASK
            },
        }
    }

    /// Set **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn with_etm_ch0_event_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmEventCh0Cfg::ETM_CH0_EVENT_SEL_MASK)
                | ((value << EtmEventCh0Cfg::ETM_CH0_EVENT_SEL_OFFSET)
                    & EtmEventCh0Cfg::ETM_CH0_EVENT_SEL_MASK),
        }
    }
}

impl Default for EtmEventCh0CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmEventCh0Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmEventCh0CfgVal {
        EtmEventCh0CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmEventCh0CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmEventCh0CfgVal) -> EtmEventCh0CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_EVENT_CH1_CFG Register
///
/// Etm Config register of Channel1
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **ETM_CH0_EVENT_EN** `[7]` (RW)
///
///   Etm event send enable bit.
///
/// - **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
///
///   Etm event channel select gpio.
#[repr(transparent)]
pub struct EtmEventCh1Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmEventCh1Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmEventCh1Cfg {
    // ======== Field Constants ========

    /// **ETM_CH0_EVENT_EN**
    ///
    /// Etm event send enable bit.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ETM_CH0_EVENT_EN_OFFSET: u32 = 7;
    pub const ETM_CH0_EVENT_EN_MASK: u32 = 0x00000080;
    pub const ETM_CH0_EVENT_EN: u32 = 1 << 7;

    /// **ETM_CH0_EVENT_SEL**
    ///
    /// Etm event channel select gpio.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const ETM_CH0_EVENT_SEL_OFFSET: u32 = 0;
    pub const ETM_CH0_EVENT_SEL_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **ETM_CH0_EVENT_EN** `[7]` (RW)
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub fn etm_ch0_event_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Set ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// Clear ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r & !Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn etm_ch0_event_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_SEL_MASK) >> Self::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Write **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn write_etm_ch0_event_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_CH0_EVENT_SEL_MASK)
                | ((value << Self::ETM_CH0_EVENT_SEL_OFFSET) & Self::ETM_CH0_EVENT_SEL_MASK)
        });
    }
}

/// ETM_EVENT_CH1_CFG value type for batch read/modify/write operations
///
/// Use `etmeventch1cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmeventch1cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmEventCh1CfgVal {
    bits: u32,
}

impl EtmEventCh1CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn etm_ch0_event_en(&self) -> bool {
        (self.bits & EtmEventCh1Cfg::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Read **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn etm_ch0_event_sel(&self) -> u32 {
        (self.bits & EtmEventCh1Cfg::ETM_CH0_EVENT_SEL_MASK)
            >> EtmEventCh1Cfg::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Set **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn with_etm_ch0_event_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmEventCh1Cfg::ETM_CH0_EVENT_EN_MASK
            } else {
                self.bits & !EtmEventCh1Cfg::ETM_CH0_EVENT_EN_MASK
            },
        }
    }

    /// Set **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn with_etm_ch0_event_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmEventCh1Cfg::ETM_CH0_EVENT_SEL_MASK)
                | ((value << EtmEventCh1Cfg::ETM_CH0_EVENT_SEL_OFFSET)
                    & EtmEventCh1Cfg::ETM_CH0_EVENT_SEL_MASK),
        }
    }
}

impl Default for EtmEventCh1CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmEventCh1Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmEventCh1CfgVal {
        EtmEventCh1CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmEventCh1CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmEventCh1CfgVal) -> EtmEventCh1CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_EVENT_CH2_CFG Register
///
/// Etm Config register of Channel2
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **ETM_CH0_EVENT_EN** `[7]` (RW)
///
///   Etm event send enable bit.
///
/// - **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
///
///   Etm event channel select gpio.
#[repr(transparent)]
pub struct EtmEventCh2Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmEventCh2Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmEventCh2Cfg {
    // ======== Field Constants ========

    /// **ETM_CH0_EVENT_EN**
    ///
    /// Etm event send enable bit.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ETM_CH0_EVENT_EN_OFFSET: u32 = 7;
    pub const ETM_CH0_EVENT_EN_MASK: u32 = 0x00000080;
    pub const ETM_CH0_EVENT_EN: u32 = 1 << 7;

    /// **ETM_CH0_EVENT_SEL**
    ///
    /// Etm event channel select gpio.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const ETM_CH0_EVENT_SEL_OFFSET: u32 = 0;
    pub const ETM_CH0_EVENT_SEL_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **ETM_CH0_EVENT_EN** `[7]` (RW)
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub fn etm_ch0_event_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Set ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// Clear ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r & !Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn etm_ch0_event_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_SEL_MASK) >> Self::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Write **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn write_etm_ch0_event_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_CH0_EVENT_SEL_MASK)
                | ((value << Self::ETM_CH0_EVENT_SEL_OFFSET) & Self::ETM_CH0_EVENT_SEL_MASK)
        });
    }
}

/// ETM_EVENT_CH2_CFG value type for batch read/modify/write operations
///
/// Use `etmeventch2cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmeventch2cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmEventCh2CfgVal {
    bits: u32,
}

impl EtmEventCh2CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn etm_ch0_event_en(&self) -> bool {
        (self.bits & EtmEventCh2Cfg::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Read **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn etm_ch0_event_sel(&self) -> u32 {
        (self.bits & EtmEventCh2Cfg::ETM_CH0_EVENT_SEL_MASK)
            >> EtmEventCh2Cfg::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Set **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn with_etm_ch0_event_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmEventCh2Cfg::ETM_CH0_EVENT_EN_MASK
            } else {
                self.bits & !EtmEventCh2Cfg::ETM_CH0_EVENT_EN_MASK
            },
        }
    }

    /// Set **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn with_etm_ch0_event_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmEventCh2Cfg::ETM_CH0_EVENT_SEL_MASK)
                | ((value << EtmEventCh2Cfg::ETM_CH0_EVENT_SEL_OFFSET)
                    & EtmEventCh2Cfg::ETM_CH0_EVENT_SEL_MASK),
        }
    }
}

impl Default for EtmEventCh2CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmEventCh2Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmEventCh2CfgVal {
        EtmEventCh2CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmEventCh2CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmEventCh2CfgVal) -> EtmEventCh2CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_EVENT_CH3_CFG Register
///
/// Etm Config register of Channel3
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **ETM_CH0_EVENT_EN** `[7]` (RW)
///
///   Etm event send enable bit.
///
/// - **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
///
///   Etm event channel select gpio.
#[repr(transparent)]
pub struct EtmEventCh3Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmEventCh3Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmEventCh3Cfg {
    // ======== Field Constants ========

    /// **ETM_CH0_EVENT_EN**
    ///
    /// Etm event send enable bit.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ETM_CH0_EVENT_EN_OFFSET: u32 = 7;
    pub const ETM_CH0_EVENT_EN_MASK: u32 = 0x00000080;
    pub const ETM_CH0_EVENT_EN: u32 = 1 << 7;

    /// **ETM_CH0_EVENT_SEL**
    ///
    /// Etm event channel select gpio.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const ETM_CH0_EVENT_SEL_OFFSET: u32 = 0;
    pub const ETM_CH0_EVENT_SEL_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **ETM_CH0_EVENT_EN** `[7]` (RW)
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub fn etm_ch0_event_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Set ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// Clear ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r & !Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn etm_ch0_event_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_SEL_MASK) >> Self::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Write **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn write_etm_ch0_event_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_CH0_EVENT_SEL_MASK)
                | ((value << Self::ETM_CH0_EVENT_SEL_OFFSET) & Self::ETM_CH0_EVENT_SEL_MASK)
        });
    }
}

/// ETM_EVENT_CH3_CFG value type for batch read/modify/write operations
///
/// Use `etmeventch3cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmeventch3cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmEventCh3CfgVal {
    bits: u32,
}

impl EtmEventCh3CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn etm_ch0_event_en(&self) -> bool {
        (self.bits & EtmEventCh3Cfg::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Read **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn etm_ch0_event_sel(&self) -> u32 {
        (self.bits & EtmEventCh3Cfg::ETM_CH0_EVENT_SEL_MASK)
            >> EtmEventCh3Cfg::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Set **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn with_etm_ch0_event_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmEventCh3Cfg::ETM_CH0_EVENT_EN_MASK
            } else {
                self.bits & !EtmEventCh3Cfg::ETM_CH0_EVENT_EN_MASK
            },
        }
    }

    /// Set **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn with_etm_ch0_event_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmEventCh3Cfg::ETM_CH0_EVENT_SEL_MASK)
                | ((value << EtmEventCh3Cfg::ETM_CH0_EVENT_SEL_OFFSET)
                    & EtmEventCh3Cfg::ETM_CH0_EVENT_SEL_MASK),
        }
    }
}

impl Default for EtmEventCh3CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmEventCh3Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmEventCh3CfgVal {
        EtmEventCh3CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmEventCh3CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmEventCh3CfgVal) -> EtmEventCh3CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_EVENT_CH4_CFG Register
///
/// Etm Config register of Channel4
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **ETM_CH0_EVENT_EN** `[7]` (RW)
///
///   Etm event send enable bit.
///
/// - **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
///
///   Etm event channel select gpio.
#[repr(transparent)]
pub struct EtmEventCh4Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmEventCh4Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmEventCh4Cfg {
    // ======== Field Constants ========

    /// **ETM_CH0_EVENT_EN**
    ///
    /// Etm event send enable bit.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ETM_CH0_EVENT_EN_OFFSET: u32 = 7;
    pub const ETM_CH0_EVENT_EN_MASK: u32 = 0x00000080;
    pub const ETM_CH0_EVENT_EN: u32 = 1 << 7;

    /// **ETM_CH0_EVENT_SEL**
    ///
    /// Etm event channel select gpio.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const ETM_CH0_EVENT_SEL_OFFSET: u32 = 0;
    pub const ETM_CH0_EVENT_SEL_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **ETM_CH0_EVENT_EN** `[7]` (RW)
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub fn etm_ch0_event_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Set ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// Clear ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r & !Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn etm_ch0_event_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_SEL_MASK) >> Self::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Write **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn write_etm_ch0_event_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_CH0_EVENT_SEL_MASK)
                | ((value << Self::ETM_CH0_EVENT_SEL_OFFSET) & Self::ETM_CH0_EVENT_SEL_MASK)
        });
    }
}

/// ETM_EVENT_CH4_CFG value type for batch read/modify/write operations
///
/// Use `etmeventch4cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmeventch4cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmEventCh4CfgVal {
    bits: u32,
}

impl EtmEventCh4CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn etm_ch0_event_en(&self) -> bool {
        (self.bits & EtmEventCh4Cfg::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Read **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn etm_ch0_event_sel(&self) -> u32 {
        (self.bits & EtmEventCh4Cfg::ETM_CH0_EVENT_SEL_MASK)
            >> EtmEventCh4Cfg::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Set **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn with_etm_ch0_event_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmEventCh4Cfg::ETM_CH0_EVENT_EN_MASK
            } else {
                self.bits & !EtmEventCh4Cfg::ETM_CH0_EVENT_EN_MASK
            },
        }
    }

    /// Set **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn with_etm_ch0_event_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmEventCh4Cfg::ETM_CH0_EVENT_SEL_MASK)
                | ((value << EtmEventCh4Cfg::ETM_CH0_EVENT_SEL_OFFSET)
                    & EtmEventCh4Cfg::ETM_CH0_EVENT_SEL_MASK),
        }
    }
}

impl Default for EtmEventCh4CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmEventCh4Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmEventCh4CfgVal {
        EtmEventCh4CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmEventCh4CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmEventCh4CfgVal) -> EtmEventCh4CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_EVENT_CH5_CFG Register
///
/// Etm Config register of Channel5
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **ETM_CH0_EVENT_EN** `[7]` (RW)
///
///   Etm event send enable bit.
///
/// - **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
///
///   Etm event channel select gpio.
#[repr(transparent)]
pub struct EtmEventCh5Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmEventCh5Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmEventCh5Cfg {
    // ======== Field Constants ========

    /// **ETM_CH0_EVENT_EN**
    ///
    /// Etm event send enable bit.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ETM_CH0_EVENT_EN_OFFSET: u32 = 7;
    pub const ETM_CH0_EVENT_EN_MASK: u32 = 0x00000080;
    pub const ETM_CH0_EVENT_EN: u32 = 1 << 7;

    /// **ETM_CH0_EVENT_SEL**
    ///
    /// Etm event channel select gpio.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const ETM_CH0_EVENT_SEL_OFFSET: u32 = 0;
    pub const ETM_CH0_EVENT_SEL_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **ETM_CH0_EVENT_EN** `[7]` (RW)
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub fn etm_ch0_event_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Set ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// Clear ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r & !Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn etm_ch0_event_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_SEL_MASK) >> Self::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Write **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn write_etm_ch0_event_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_CH0_EVENT_SEL_MASK)
                | ((value << Self::ETM_CH0_EVENT_SEL_OFFSET) & Self::ETM_CH0_EVENT_SEL_MASK)
        });
    }
}

/// ETM_EVENT_CH5_CFG value type for batch read/modify/write operations
///
/// Use `etmeventch5cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmeventch5cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmEventCh5CfgVal {
    bits: u32,
}

impl EtmEventCh5CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn etm_ch0_event_en(&self) -> bool {
        (self.bits & EtmEventCh5Cfg::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Read **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn etm_ch0_event_sel(&self) -> u32 {
        (self.bits & EtmEventCh5Cfg::ETM_CH0_EVENT_SEL_MASK)
            >> EtmEventCh5Cfg::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Set **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn with_etm_ch0_event_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmEventCh5Cfg::ETM_CH0_EVENT_EN_MASK
            } else {
                self.bits & !EtmEventCh5Cfg::ETM_CH0_EVENT_EN_MASK
            },
        }
    }

    /// Set **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn with_etm_ch0_event_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmEventCh5Cfg::ETM_CH0_EVENT_SEL_MASK)
                | ((value << EtmEventCh5Cfg::ETM_CH0_EVENT_SEL_OFFSET)
                    & EtmEventCh5Cfg::ETM_CH0_EVENT_SEL_MASK),
        }
    }
}

impl Default for EtmEventCh5CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmEventCh5Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmEventCh5CfgVal {
        EtmEventCh5CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmEventCh5CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmEventCh5CfgVal) -> EtmEventCh5CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_EVENT_CH6_CFG Register
///
/// Etm Config register of Channel6
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **ETM_CH0_EVENT_EN** `[7]` (RW)
///
///   Etm event send enable bit.
///
/// - **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
///
///   Etm event channel select gpio.
#[repr(transparent)]
pub struct EtmEventCh6Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmEventCh6Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmEventCh6Cfg {
    // ======== Field Constants ========

    /// **ETM_CH0_EVENT_EN**
    ///
    /// Etm event send enable bit.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ETM_CH0_EVENT_EN_OFFSET: u32 = 7;
    pub const ETM_CH0_EVENT_EN_MASK: u32 = 0x00000080;
    pub const ETM_CH0_EVENT_EN: u32 = 1 << 7;

    /// **ETM_CH0_EVENT_SEL**
    ///
    /// Etm event channel select gpio.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const ETM_CH0_EVENT_SEL_OFFSET: u32 = 0;
    pub const ETM_CH0_EVENT_SEL_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **ETM_CH0_EVENT_EN** `[7]` (RW)
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub fn etm_ch0_event_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Set ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// Clear ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r & !Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn etm_ch0_event_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_SEL_MASK) >> Self::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Write **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn write_etm_ch0_event_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_CH0_EVENT_SEL_MASK)
                | ((value << Self::ETM_CH0_EVENT_SEL_OFFSET) & Self::ETM_CH0_EVENT_SEL_MASK)
        });
    }
}

/// ETM_EVENT_CH6_CFG value type for batch read/modify/write operations
///
/// Use `etmeventch6cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmeventch6cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmEventCh6CfgVal {
    bits: u32,
}

impl EtmEventCh6CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn etm_ch0_event_en(&self) -> bool {
        (self.bits & EtmEventCh6Cfg::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Read **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn etm_ch0_event_sel(&self) -> u32 {
        (self.bits & EtmEventCh6Cfg::ETM_CH0_EVENT_SEL_MASK)
            >> EtmEventCh6Cfg::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Set **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn with_etm_ch0_event_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmEventCh6Cfg::ETM_CH0_EVENT_EN_MASK
            } else {
                self.bits & !EtmEventCh6Cfg::ETM_CH0_EVENT_EN_MASK
            },
        }
    }

    /// Set **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn with_etm_ch0_event_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmEventCh6Cfg::ETM_CH0_EVENT_SEL_MASK)
                | ((value << EtmEventCh6Cfg::ETM_CH0_EVENT_SEL_OFFSET)
                    & EtmEventCh6Cfg::ETM_CH0_EVENT_SEL_MASK),
        }
    }
}

impl Default for EtmEventCh6CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmEventCh6Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmEventCh6CfgVal {
        EtmEventCh6CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmEventCh6CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmEventCh6CfgVal) -> EtmEventCh6CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_EVENT_CH7_CFG Register
///
/// Etm Config register of Channel7
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **ETM_CH0_EVENT_EN** `[7]` (RW)
///
///   Etm event send enable bit.
///
/// - **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
///
///   Etm event channel select gpio.
#[repr(transparent)]
pub struct EtmEventCh7Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmEventCh7Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmEventCh7Cfg {
    // ======== Field Constants ========

    /// **ETM_CH0_EVENT_EN**
    ///
    /// Etm event send enable bit.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ETM_CH0_EVENT_EN_OFFSET: u32 = 7;
    pub const ETM_CH0_EVENT_EN_MASK: u32 = 0x00000080;
    pub const ETM_CH0_EVENT_EN: u32 = 1 << 7;

    /// **ETM_CH0_EVENT_SEL**
    ///
    /// Etm event channel select gpio.
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const ETM_CH0_EVENT_SEL_OFFSET: u32 = 0;
    pub const ETM_CH0_EVENT_SEL_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **ETM_CH0_EVENT_EN** `[7]` (RW)
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub fn etm_ch0_event_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Set ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// Clear ETM_CH0_EVENT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_ch0_event_en(&self) {
        self.value.modify_bits(|r| r & !Self::ETM_CH0_EVENT_EN_MASK);
    }

    /// **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn etm_ch0_event_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_CH0_EVENT_SEL_MASK) >> Self::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Write **ETM_CH0_EVENT_SEL** `[5:0]` (RW)
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub fn write_etm_ch0_event_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_CH0_EVENT_SEL_MASK)
                | ((value << Self::ETM_CH0_EVENT_SEL_OFFSET) & Self::ETM_CH0_EVENT_SEL_MASK)
        });
    }
}

/// ETM_EVENT_CH7_CFG value type for batch read/modify/write operations
///
/// Use `etmeventch7cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmeventch7cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmEventCh7CfgVal {
    bits: u32,
}

impl EtmEventCh7CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn etm_ch0_event_en(&self) -> bool {
        (self.bits & EtmEventCh7Cfg::ETM_CH0_EVENT_EN_MASK) != 0
    }

    /// Read **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn etm_ch0_event_sel(&self) -> u32 {
        (self.bits & EtmEventCh7Cfg::ETM_CH0_EVENT_SEL_MASK)
            >> EtmEventCh7Cfg::ETM_CH0_EVENT_SEL_OFFSET
    }

    /// Set **ETM_CH0_EVENT_EN** `[7:7]`
    ///
    /// Etm event send enable bit.
    #[inline(always)]
    pub const fn with_etm_ch0_event_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmEventCh7Cfg::ETM_CH0_EVENT_EN_MASK
            } else {
                self.bits & !EtmEventCh7Cfg::ETM_CH0_EVENT_EN_MASK
            },
        }
    }

    /// Set **ETM_CH0_EVENT_SEL** `[5:0]`
    ///
    /// Etm event channel select gpio.
    #[inline(always)]
    pub const fn with_etm_ch0_event_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmEventCh7Cfg::ETM_CH0_EVENT_SEL_MASK)
                | ((value << EtmEventCh7Cfg::ETM_CH0_EVENT_SEL_OFFSET)
                    & EtmEventCh7Cfg::ETM_CH0_EVENT_SEL_MASK),
        }
    }
}

impl Default for EtmEventCh7CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmEventCh7Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmEventCh7CfgVal {
        EtmEventCh7CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmEventCh7CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmEventCh7CfgVal) -> EtmEventCh7CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_TASK_P0_CFG Register
///
/// Etm Configure Register to decide which GPIO been chosen
///
/// Address offset: 0x00a0
///
/// ## Fields
///
/// - **ETM_TASK_GPIO3_SEL** `[27:25]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO3_EN** `[24]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO2_SEL** `[19:17]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO2_EN** `[16]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO1_SEL** `[11:9]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO1_EN** `[8]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO0_SEL** `[3:1]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO0_EN** `[0]` (RW)
///
///   Enable bit of GPIO response etm task.
#[repr(transparent)]
pub struct EtmTaskP0Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmTaskP0Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmTaskP0Cfg {
    // ======== Field Constants ========

    /// **ETM_TASK_GPIO3_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const ETM_TASK_GPIO3_SEL_OFFSET: u32 = 25;
    pub const ETM_TASK_GPIO3_SEL_MASK: u32 = 0x0e000000;

    /// **ETM_TASK_GPIO3_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const ETM_TASK_GPIO3_EN_OFFSET: u32 = 24;
    pub const ETM_TASK_GPIO3_EN_MASK: u32 = 0x01000000;
    pub const ETM_TASK_GPIO3_EN: u32 = 1 << 24;

    /// **ETM_TASK_GPIO2_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const ETM_TASK_GPIO2_SEL_OFFSET: u32 = 17;
    pub const ETM_TASK_GPIO2_SEL_MASK: u32 = 0x000e0000;

    /// **ETM_TASK_GPIO2_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETM_TASK_GPIO2_EN_OFFSET: u32 = 16;
    pub const ETM_TASK_GPIO2_EN_MASK: u32 = 0x00010000;
    pub const ETM_TASK_GPIO2_EN: u32 = 1 << 16;

    /// **ETM_TASK_GPIO1_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const ETM_TASK_GPIO1_SEL_OFFSET: u32 = 9;
    pub const ETM_TASK_GPIO1_SEL_MASK: u32 = 0x00000e00;

    /// **ETM_TASK_GPIO1_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ETM_TASK_GPIO1_EN_OFFSET: u32 = 8;
    pub const ETM_TASK_GPIO1_EN_MASK: u32 = 0x00000100;
    pub const ETM_TASK_GPIO1_EN: u32 = 1 << 8;

    /// **ETM_TASK_GPIO0_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const ETM_TASK_GPIO0_SEL_OFFSET: u32 = 1;
    pub const ETM_TASK_GPIO0_SEL_MASK: u32 = 0x0000000e;

    /// **ETM_TASK_GPIO0_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_TASK_GPIO0_EN_OFFSET: u32 = 0;
    pub const ETM_TASK_GPIO0_EN_MASK: u32 = 0x00000001;
    pub const ETM_TASK_GPIO0_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ETM_TASK_GPIO3_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio3_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO3_SEL_MASK) >> Self::ETM_TASK_GPIO3_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO3_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio3_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO3_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO3_SEL_OFFSET) & Self::ETM_TASK_GPIO3_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO3_EN** `[24]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio3_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO3_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio3_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_TASK_GPIO3_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO3_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio3_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO3_EN_MASK);
    }

    /// **ETM_TASK_GPIO2_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio2_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO2_SEL_MASK) >> Self::ETM_TASK_GPIO2_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO2_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio2_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO2_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO2_SEL_OFFSET) & Self::ETM_TASK_GPIO2_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO2_EN** `[16]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio2_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO2_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio2_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_TASK_GPIO2_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO2_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio2_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO2_EN_MASK);
    }

    /// **ETM_TASK_GPIO1_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio1_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO1_SEL_MASK) >> Self::ETM_TASK_GPIO1_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO1_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio1_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO1_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO1_SEL_OFFSET) & Self::ETM_TASK_GPIO1_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO1_EN** `[8]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio1_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO1_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio1_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_TASK_GPIO1_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO1_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio1_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO1_EN_MASK);
    }

    /// **ETM_TASK_GPIO0_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio0_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO0_SEL_MASK) >> Self::ETM_TASK_GPIO0_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO0_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio0_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO0_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO0_SEL_OFFSET) & Self::ETM_TASK_GPIO0_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO0_EN** `[0]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio0_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO0_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio0_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_TASK_GPIO0_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO0_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio0_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO0_EN_MASK);
    }
}

/// ETM_TASK_P0_CFG value type for batch read/modify/write operations
///
/// Use `etmtaskp0cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmtaskp0cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmTaskP0CfgVal {
    bits: u32,
}

impl EtmTaskP0CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_TASK_GPIO3_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio3_sel(&self) -> u32 {
        (self.bits & EtmTaskP0Cfg::ETM_TASK_GPIO3_SEL_MASK)
            >> EtmTaskP0Cfg::ETM_TASK_GPIO3_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO3_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio3_en(&self) -> bool {
        (self.bits & EtmTaskP0Cfg::ETM_TASK_GPIO3_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO2_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio2_sel(&self) -> u32 {
        (self.bits & EtmTaskP0Cfg::ETM_TASK_GPIO2_SEL_MASK)
            >> EtmTaskP0Cfg::ETM_TASK_GPIO2_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO2_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio2_en(&self) -> bool {
        (self.bits & EtmTaskP0Cfg::ETM_TASK_GPIO2_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO1_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio1_sel(&self) -> u32 {
        (self.bits & EtmTaskP0Cfg::ETM_TASK_GPIO1_SEL_MASK)
            >> EtmTaskP0Cfg::ETM_TASK_GPIO1_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO1_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio1_en(&self) -> bool {
        (self.bits & EtmTaskP0Cfg::ETM_TASK_GPIO1_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO0_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio0_sel(&self) -> u32 {
        (self.bits & EtmTaskP0Cfg::ETM_TASK_GPIO0_SEL_MASK)
            >> EtmTaskP0Cfg::ETM_TASK_GPIO0_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO0_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio0_en(&self) -> bool {
        (self.bits & EtmTaskP0Cfg::ETM_TASK_GPIO0_EN_MASK) != 0
    }

    /// Set **ETM_TASK_GPIO3_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio3_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP0Cfg::ETM_TASK_GPIO3_SEL_MASK)
                | ((value << EtmTaskP0Cfg::ETM_TASK_GPIO3_SEL_OFFSET)
                    & EtmTaskP0Cfg::ETM_TASK_GPIO3_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO3_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio3_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP0Cfg::ETM_TASK_GPIO3_EN_MASK
            } else {
                self.bits & !EtmTaskP0Cfg::ETM_TASK_GPIO3_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO2_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio2_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP0Cfg::ETM_TASK_GPIO2_SEL_MASK)
                | ((value << EtmTaskP0Cfg::ETM_TASK_GPIO2_SEL_OFFSET)
                    & EtmTaskP0Cfg::ETM_TASK_GPIO2_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO2_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio2_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP0Cfg::ETM_TASK_GPIO2_EN_MASK
            } else {
                self.bits & !EtmTaskP0Cfg::ETM_TASK_GPIO2_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO1_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio1_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP0Cfg::ETM_TASK_GPIO1_SEL_MASK)
                | ((value << EtmTaskP0Cfg::ETM_TASK_GPIO1_SEL_OFFSET)
                    & EtmTaskP0Cfg::ETM_TASK_GPIO1_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO1_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio1_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP0Cfg::ETM_TASK_GPIO1_EN_MASK
            } else {
                self.bits & !EtmTaskP0Cfg::ETM_TASK_GPIO1_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO0_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio0_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP0Cfg::ETM_TASK_GPIO0_SEL_MASK)
                | ((value << EtmTaskP0Cfg::ETM_TASK_GPIO0_SEL_OFFSET)
                    & EtmTaskP0Cfg::ETM_TASK_GPIO0_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO0_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio0_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP0Cfg::ETM_TASK_GPIO0_EN_MASK
            } else {
                self.bits & !EtmTaskP0Cfg::ETM_TASK_GPIO0_EN_MASK
            },
        }
    }
}

impl Default for EtmTaskP0CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmTaskP0Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmTaskP0CfgVal {
        EtmTaskP0CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmTaskP0CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmTaskP0CfgVal) -> EtmTaskP0CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_TASK_P1_CFG Register
///
/// Etm Configure Register to decide which GPIO been chosen
///
/// Address offset: 0x00a4
///
/// ## Fields
///
/// - **ETM_TASK_GPIO7_SEL** `[27:25]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO7_EN** `[24]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO6_SEL** `[19:17]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO6_EN** `[16]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO5_SEL** `[11:9]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO5_EN** `[8]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO4_SEL** `[3:1]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO4_EN** `[0]` (RW)
///
///   Enable bit of GPIO response etm task.
#[repr(transparent)]
pub struct EtmTaskP1Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmTaskP1Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmTaskP1Cfg {
    // ======== Field Constants ========

    /// **ETM_TASK_GPIO7_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const ETM_TASK_GPIO7_SEL_OFFSET: u32 = 25;
    pub const ETM_TASK_GPIO7_SEL_MASK: u32 = 0x0e000000;

    /// **ETM_TASK_GPIO7_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const ETM_TASK_GPIO7_EN_OFFSET: u32 = 24;
    pub const ETM_TASK_GPIO7_EN_MASK: u32 = 0x01000000;
    pub const ETM_TASK_GPIO7_EN: u32 = 1 << 24;

    /// **ETM_TASK_GPIO6_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const ETM_TASK_GPIO6_SEL_OFFSET: u32 = 17;
    pub const ETM_TASK_GPIO6_SEL_MASK: u32 = 0x000e0000;

    /// **ETM_TASK_GPIO6_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETM_TASK_GPIO6_EN_OFFSET: u32 = 16;
    pub const ETM_TASK_GPIO6_EN_MASK: u32 = 0x00010000;
    pub const ETM_TASK_GPIO6_EN: u32 = 1 << 16;

    /// **ETM_TASK_GPIO5_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const ETM_TASK_GPIO5_SEL_OFFSET: u32 = 9;
    pub const ETM_TASK_GPIO5_SEL_MASK: u32 = 0x00000e00;

    /// **ETM_TASK_GPIO5_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ETM_TASK_GPIO5_EN_OFFSET: u32 = 8;
    pub const ETM_TASK_GPIO5_EN_MASK: u32 = 0x00000100;
    pub const ETM_TASK_GPIO5_EN: u32 = 1 << 8;

    /// **ETM_TASK_GPIO4_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const ETM_TASK_GPIO4_SEL_OFFSET: u32 = 1;
    pub const ETM_TASK_GPIO4_SEL_MASK: u32 = 0x0000000e;

    /// **ETM_TASK_GPIO4_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_TASK_GPIO4_EN_OFFSET: u32 = 0;
    pub const ETM_TASK_GPIO4_EN_MASK: u32 = 0x00000001;
    pub const ETM_TASK_GPIO4_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ETM_TASK_GPIO7_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio7_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO7_SEL_MASK) >> Self::ETM_TASK_GPIO7_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO7_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio7_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO7_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO7_SEL_OFFSET) & Self::ETM_TASK_GPIO7_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO7_EN** `[24]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio7_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO7_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio7_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_TASK_GPIO7_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO7_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio7_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO7_EN_MASK);
    }

    /// **ETM_TASK_GPIO6_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio6_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO6_SEL_MASK) >> Self::ETM_TASK_GPIO6_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO6_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio6_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO6_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO6_SEL_OFFSET) & Self::ETM_TASK_GPIO6_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO6_EN** `[16]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio6_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO6_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio6_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_TASK_GPIO6_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO6_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio6_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO6_EN_MASK);
    }

    /// **ETM_TASK_GPIO5_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio5_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO5_SEL_MASK) >> Self::ETM_TASK_GPIO5_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO5_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio5_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO5_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO5_SEL_OFFSET) & Self::ETM_TASK_GPIO5_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO5_EN** `[8]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio5_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO5_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio5_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_TASK_GPIO5_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO5_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio5_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO5_EN_MASK);
    }

    /// **ETM_TASK_GPIO4_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio4_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO4_SEL_MASK) >> Self::ETM_TASK_GPIO4_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO4_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio4_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO4_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO4_SEL_OFFSET) & Self::ETM_TASK_GPIO4_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO4_EN** `[0]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio4_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO4_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio4_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_TASK_GPIO4_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO4_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio4_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO4_EN_MASK);
    }
}

/// ETM_TASK_P1_CFG value type for batch read/modify/write operations
///
/// Use `etmtaskp1cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmtaskp1cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmTaskP1CfgVal {
    bits: u32,
}

impl EtmTaskP1CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_TASK_GPIO7_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio7_sel(&self) -> u32 {
        (self.bits & EtmTaskP1Cfg::ETM_TASK_GPIO7_SEL_MASK)
            >> EtmTaskP1Cfg::ETM_TASK_GPIO7_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO7_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio7_en(&self) -> bool {
        (self.bits & EtmTaskP1Cfg::ETM_TASK_GPIO7_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO6_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio6_sel(&self) -> u32 {
        (self.bits & EtmTaskP1Cfg::ETM_TASK_GPIO6_SEL_MASK)
            >> EtmTaskP1Cfg::ETM_TASK_GPIO6_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO6_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio6_en(&self) -> bool {
        (self.bits & EtmTaskP1Cfg::ETM_TASK_GPIO6_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO5_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio5_sel(&self) -> u32 {
        (self.bits & EtmTaskP1Cfg::ETM_TASK_GPIO5_SEL_MASK)
            >> EtmTaskP1Cfg::ETM_TASK_GPIO5_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO5_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio5_en(&self) -> bool {
        (self.bits & EtmTaskP1Cfg::ETM_TASK_GPIO5_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO4_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio4_sel(&self) -> u32 {
        (self.bits & EtmTaskP1Cfg::ETM_TASK_GPIO4_SEL_MASK)
            >> EtmTaskP1Cfg::ETM_TASK_GPIO4_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO4_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio4_en(&self) -> bool {
        (self.bits & EtmTaskP1Cfg::ETM_TASK_GPIO4_EN_MASK) != 0
    }

    /// Set **ETM_TASK_GPIO7_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio7_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP1Cfg::ETM_TASK_GPIO7_SEL_MASK)
                | ((value << EtmTaskP1Cfg::ETM_TASK_GPIO7_SEL_OFFSET)
                    & EtmTaskP1Cfg::ETM_TASK_GPIO7_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO7_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio7_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP1Cfg::ETM_TASK_GPIO7_EN_MASK
            } else {
                self.bits & !EtmTaskP1Cfg::ETM_TASK_GPIO7_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO6_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio6_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP1Cfg::ETM_TASK_GPIO6_SEL_MASK)
                | ((value << EtmTaskP1Cfg::ETM_TASK_GPIO6_SEL_OFFSET)
                    & EtmTaskP1Cfg::ETM_TASK_GPIO6_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO6_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio6_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP1Cfg::ETM_TASK_GPIO6_EN_MASK
            } else {
                self.bits & !EtmTaskP1Cfg::ETM_TASK_GPIO6_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO5_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio5_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP1Cfg::ETM_TASK_GPIO5_SEL_MASK)
                | ((value << EtmTaskP1Cfg::ETM_TASK_GPIO5_SEL_OFFSET)
                    & EtmTaskP1Cfg::ETM_TASK_GPIO5_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO5_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio5_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP1Cfg::ETM_TASK_GPIO5_EN_MASK
            } else {
                self.bits & !EtmTaskP1Cfg::ETM_TASK_GPIO5_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO4_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio4_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP1Cfg::ETM_TASK_GPIO4_SEL_MASK)
                | ((value << EtmTaskP1Cfg::ETM_TASK_GPIO4_SEL_OFFSET)
                    & EtmTaskP1Cfg::ETM_TASK_GPIO4_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO4_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio4_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP1Cfg::ETM_TASK_GPIO4_EN_MASK
            } else {
                self.bits & !EtmTaskP1Cfg::ETM_TASK_GPIO4_EN_MASK
            },
        }
    }
}

impl Default for EtmTaskP1CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmTaskP1Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmTaskP1CfgVal {
        EtmTaskP1CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmTaskP1CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmTaskP1CfgVal) -> EtmTaskP1CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_TASK_P2_CFG Register
///
/// Etm Configure Register to decide which GPIO been chosen
///
/// Address offset: 0x00a8
///
/// ## Fields
///
/// - **ETM_TASK_GPIO11_SEL** `[27:25]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO11_EN** `[24]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO10_SEL** `[19:17]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO10_EN** `[16]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO9_SEL** `[11:9]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO9_EN** `[8]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO8_SEL** `[3:1]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO8_EN** `[0]` (RW)
///
///   Enable bit of GPIO response etm task.
#[repr(transparent)]
pub struct EtmTaskP2Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmTaskP2Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmTaskP2Cfg {
    // ======== Field Constants ========

    /// **ETM_TASK_GPIO11_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const ETM_TASK_GPIO11_SEL_OFFSET: u32 = 25;
    pub const ETM_TASK_GPIO11_SEL_MASK: u32 = 0x0e000000;

    /// **ETM_TASK_GPIO11_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const ETM_TASK_GPIO11_EN_OFFSET: u32 = 24;
    pub const ETM_TASK_GPIO11_EN_MASK: u32 = 0x01000000;
    pub const ETM_TASK_GPIO11_EN: u32 = 1 << 24;

    /// **ETM_TASK_GPIO10_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const ETM_TASK_GPIO10_SEL_OFFSET: u32 = 17;
    pub const ETM_TASK_GPIO10_SEL_MASK: u32 = 0x000e0000;

    /// **ETM_TASK_GPIO10_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETM_TASK_GPIO10_EN_OFFSET: u32 = 16;
    pub const ETM_TASK_GPIO10_EN_MASK: u32 = 0x00010000;
    pub const ETM_TASK_GPIO10_EN: u32 = 1 << 16;

    /// **ETM_TASK_GPIO9_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const ETM_TASK_GPIO9_SEL_OFFSET: u32 = 9;
    pub const ETM_TASK_GPIO9_SEL_MASK: u32 = 0x00000e00;

    /// **ETM_TASK_GPIO9_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ETM_TASK_GPIO9_EN_OFFSET: u32 = 8;
    pub const ETM_TASK_GPIO9_EN_MASK: u32 = 0x00000100;
    pub const ETM_TASK_GPIO9_EN: u32 = 1 << 8;

    /// **ETM_TASK_GPIO8_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const ETM_TASK_GPIO8_SEL_OFFSET: u32 = 1;
    pub const ETM_TASK_GPIO8_SEL_MASK: u32 = 0x0000000e;

    /// **ETM_TASK_GPIO8_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_TASK_GPIO8_EN_OFFSET: u32 = 0;
    pub const ETM_TASK_GPIO8_EN_MASK: u32 = 0x00000001;
    pub const ETM_TASK_GPIO8_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ETM_TASK_GPIO11_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio11_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO11_SEL_MASK)
            >> Self::ETM_TASK_GPIO11_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO11_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio11_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO11_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO11_SEL_OFFSET) & Self::ETM_TASK_GPIO11_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO11_EN** `[24]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio11_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO11_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO11_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio11_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO11_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO11_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio11_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO11_EN_MASK);
    }

    /// **ETM_TASK_GPIO10_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio10_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO10_SEL_MASK)
            >> Self::ETM_TASK_GPIO10_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO10_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio10_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO10_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO10_SEL_OFFSET) & Self::ETM_TASK_GPIO10_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO10_EN** `[16]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio10_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO10_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO10_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio10_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO10_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO10_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio10_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO10_EN_MASK);
    }

    /// **ETM_TASK_GPIO9_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio9_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO9_SEL_MASK) >> Self::ETM_TASK_GPIO9_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO9_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio9_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO9_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO9_SEL_OFFSET) & Self::ETM_TASK_GPIO9_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO9_EN** `[8]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio9_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO9_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO9_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio9_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_TASK_GPIO9_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO9_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio9_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO9_EN_MASK);
    }

    /// **ETM_TASK_GPIO8_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio8_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO8_SEL_MASK) >> Self::ETM_TASK_GPIO8_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO8_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio8_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO8_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO8_SEL_OFFSET) & Self::ETM_TASK_GPIO8_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO8_EN** `[0]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio8_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO8_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO8_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio8_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_TASK_GPIO8_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO8_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio8_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO8_EN_MASK);
    }
}

/// ETM_TASK_P2_CFG value type for batch read/modify/write operations
///
/// Use `etmtaskp2cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmtaskp2cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmTaskP2CfgVal {
    bits: u32,
}

impl EtmTaskP2CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_TASK_GPIO11_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio11_sel(&self) -> u32 {
        (self.bits & EtmTaskP2Cfg::ETM_TASK_GPIO11_SEL_MASK)
            >> EtmTaskP2Cfg::ETM_TASK_GPIO11_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO11_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio11_en(&self) -> bool {
        (self.bits & EtmTaskP2Cfg::ETM_TASK_GPIO11_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO10_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio10_sel(&self) -> u32 {
        (self.bits & EtmTaskP2Cfg::ETM_TASK_GPIO10_SEL_MASK)
            >> EtmTaskP2Cfg::ETM_TASK_GPIO10_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO10_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio10_en(&self) -> bool {
        (self.bits & EtmTaskP2Cfg::ETM_TASK_GPIO10_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO9_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio9_sel(&self) -> u32 {
        (self.bits & EtmTaskP2Cfg::ETM_TASK_GPIO9_SEL_MASK)
            >> EtmTaskP2Cfg::ETM_TASK_GPIO9_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO9_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio9_en(&self) -> bool {
        (self.bits & EtmTaskP2Cfg::ETM_TASK_GPIO9_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO8_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio8_sel(&self) -> u32 {
        (self.bits & EtmTaskP2Cfg::ETM_TASK_GPIO8_SEL_MASK)
            >> EtmTaskP2Cfg::ETM_TASK_GPIO8_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO8_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio8_en(&self) -> bool {
        (self.bits & EtmTaskP2Cfg::ETM_TASK_GPIO8_EN_MASK) != 0
    }

    /// Set **ETM_TASK_GPIO11_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio11_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP2Cfg::ETM_TASK_GPIO11_SEL_MASK)
                | ((value << EtmTaskP2Cfg::ETM_TASK_GPIO11_SEL_OFFSET)
                    & EtmTaskP2Cfg::ETM_TASK_GPIO11_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO11_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio11_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP2Cfg::ETM_TASK_GPIO11_EN_MASK
            } else {
                self.bits & !EtmTaskP2Cfg::ETM_TASK_GPIO11_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO10_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio10_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP2Cfg::ETM_TASK_GPIO10_SEL_MASK)
                | ((value << EtmTaskP2Cfg::ETM_TASK_GPIO10_SEL_OFFSET)
                    & EtmTaskP2Cfg::ETM_TASK_GPIO10_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO10_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio10_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP2Cfg::ETM_TASK_GPIO10_EN_MASK
            } else {
                self.bits & !EtmTaskP2Cfg::ETM_TASK_GPIO10_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO9_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio9_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP2Cfg::ETM_TASK_GPIO9_SEL_MASK)
                | ((value << EtmTaskP2Cfg::ETM_TASK_GPIO9_SEL_OFFSET)
                    & EtmTaskP2Cfg::ETM_TASK_GPIO9_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO9_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio9_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP2Cfg::ETM_TASK_GPIO9_EN_MASK
            } else {
                self.bits & !EtmTaskP2Cfg::ETM_TASK_GPIO9_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO8_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio8_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP2Cfg::ETM_TASK_GPIO8_SEL_MASK)
                | ((value << EtmTaskP2Cfg::ETM_TASK_GPIO8_SEL_OFFSET)
                    & EtmTaskP2Cfg::ETM_TASK_GPIO8_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO8_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio8_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP2Cfg::ETM_TASK_GPIO8_EN_MASK
            } else {
                self.bits & !EtmTaskP2Cfg::ETM_TASK_GPIO8_EN_MASK
            },
        }
    }
}

impl Default for EtmTaskP2CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmTaskP2Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmTaskP2CfgVal {
        EtmTaskP2CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmTaskP2CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmTaskP2CfgVal) -> EtmTaskP2CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_TASK_P3_CFG Register
///
/// Etm Configure Register to decide which GPIO been chosen
///
/// Address offset: 0x00ac
///
/// ## Fields
///
/// - **ETM_TASK_GPIO15_SEL** `[27:25]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO15_EN** `[24]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO14_SEL** `[19:17]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO14_EN** `[16]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO13_SEL** `[11:9]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO13_EN** `[8]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO12_SEL** `[3:1]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO12_EN** `[0]` (RW)
///
///   Enable bit of GPIO response etm task.
#[repr(transparent)]
pub struct EtmTaskP3Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmTaskP3Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmTaskP3Cfg {
    // ======== Field Constants ========

    /// **ETM_TASK_GPIO15_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const ETM_TASK_GPIO15_SEL_OFFSET: u32 = 25;
    pub const ETM_TASK_GPIO15_SEL_MASK: u32 = 0x0e000000;

    /// **ETM_TASK_GPIO15_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const ETM_TASK_GPIO15_EN_OFFSET: u32 = 24;
    pub const ETM_TASK_GPIO15_EN_MASK: u32 = 0x01000000;
    pub const ETM_TASK_GPIO15_EN: u32 = 1 << 24;

    /// **ETM_TASK_GPIO14_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const ETM_TASK_GPIO14_SEL_OFFSET: u32 = 17;
    pub const ETM_TASK_GPIO14_SEL_MASK: u32 = 0x000e0000;

    /// **ETM_TASK_GPIO14_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETM_TASK_GPIO14_EN_OFFSET: u32 = 16;
    pub const ETM_TASK_GPIO14_EN_MASK: u32 = 0x00010000;
    pub const ETM_TASK_GPIO14_EN: u32 = 1 << 16;

    /// **ETM_TASK_GPIO13_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const ETM_TASK_GPIO13_SEL_OFFSET: u32 = 9;
    pub const ETM_TASK_GPIO13_SEL_MASK: u32 = 0x00000e00;

    /// **ETM_TASK_GPIO13_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ETM_TASK_GPIO13_EN_OFFSET: u32 = 8;
    pub const ETM_TASK_GPIO13_EN_MASK: u32 = 0x00000100;
    pub const ETM_TASK_GPIO13_EN: u32 = 1 << 8;

    /// **ETM_TASK_GPIO12_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const ETM_TASK_GPIO12_SEL_OFFSET: u32 = 1;
    pub const ETM_TASK_GPIO12_SEL_MASK: u32 = 0x0000000e;

    /// **ETM_TASK_GPIO12_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_TASK_GPIO12_EN_OFFSET: u32 = 0;
    pub const ETM_TASK_GPIO12_EN_MASK: u32 = 0x00000001;
    pub const ETM_TASK_GPIO12_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ETM_TASK_GPIO15_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio15_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO15_SEL_MASK)
            >> Self::ETM_TASK_GPIO15_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO15_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio15_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO15_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO15_SEL_OFFSET) & Self::ETM_TASK_GPIO15_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO15_EN** `[24]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio15_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO15_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO15_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio15_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO15_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO15_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio15_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO15_EN_MASK);
    }

    /// **ETM_TASK_GPIO14_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio14_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO14_SEL_MASK)
            >> Self::ETM_TASK_GPIO14_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO14_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio14_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO14_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO14_SEL_OFFSET) & Self::ETM_TASK_GPIO14_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO14_EN** `[16]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio14_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO14_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO14_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio14_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO14_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO14_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio14_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO14_EN_MASK);
    }

    /// **ETM_TASK_GPIO13_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio13_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO13_SEL_MASK)
            >> Self::ETM_TASK_GPIO13_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO13_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio13_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO13_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO13_SEL_OFFSET) & Self::ETM_TASK_GPIO13_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO13_EN** `[8]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio13_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO13_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO13_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio13_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO13_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO13_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio13_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO13_EN_MASK);
    }

    /// **ETM_TASK_GPIO12_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio12_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO12_SEL_MASK)
            >> Self::ETM_TASK_GPIO12_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO12_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio12_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO12_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO12_SEL_OFFSET) & Self::ETM_TASK_GPIO12_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO12_EN** `[0]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio12_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO12_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO12_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio12_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO12_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO12_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio12_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO12_EN_MASK);
    }
}

/// ETM_TASK_P3_CFG value type for batch read/modify/write operations
///
/// Use `etmtaskp3cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmtaskp3cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmTaskP3CfgVal {
    bits: u32,
}

impl EtmTaskP3CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_TASK_GPIO15_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio15_sel(&self) -> u32 {
        (self.bits & EtmTaskP3Cfg::ETM_TASK_GPIO15_SEL_MASK)
            >> EtmTaskP3Cfg::ETM_TASK_GPIO15_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO15_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio15_en(&self) -> bool {
        (self.bits & EtmTaskP3Cfg::ETM_TASK_GPIO15_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO14_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio14_sel(&self) -> u32 {
        (self.bits & EtmTaskP3Cfg::ETM_TASK_GPIO14_SEL_MASK)
            >> EtmTaskP3Cfg::ETM_TASK_GPIO14_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO14_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio14_en(&self) -> bool {
        (self.bits & EtmTaskP3Cfg::ETM_TASK_GPIO14_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO13_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio13_sel(&self) -> u32 {
        (self.bits & EtmTaskP3Cfg::ETM_TASK_GPIO13_SEL_MASK)
            >> EtmTaskP3Cfg::ETM_TASK_GPIO13_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO13_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio13_en(&self) -> bool {
        (self.bits & EtmTaskP3Cfg::ETM_TASK_GPIO13_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO12_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio12_sel(&self) -> u32 {
        (self.bits & EtmTaskP3Cfg::ETM_TASK_GPIO12_SEL_MASK)
            >> EtmTaskP3Cfg::ETM_TASK_GPIO12_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO12_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio12_en(&self) -> bool {
        (self.bits & EtmTaskP3Cfg::ETM_TASK_GPIO12_EN_MASK) != 0
    }

    /// Set **ETM_TASK_GPIO15_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio15_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP3Cfg::ETM_TASK_GPIO15_SEL_MASK)
                | ((value << EtmTaskP3Cfg::ETM_TASK_GPIO15_SEL_OFFSET)
                    & EtmTaskP3Cfg::ETM_TASK_GPIO15_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO15_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio15_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP3Cfg::ETM_TASK_GPIO15_EN_MASK
            } else {
                self.bits & !EtmTaskP3Cfg::ETM_TASK_GPIO15_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO14_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio14_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP3Cfg::ETM_TASK_GPIO14_SEL_MASK)
                | ((value << EtmTaskP3Cfg::ETM_TASK_GPIO14_SEL_OFFSET)
                    & EtmTaskP3Cfg::ETM_TASK_GPIO14_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO14_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio14_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP3Cfg::ETM_TASK_GPIO14_EN_MASK
            } else {
                self.bits & !EtmTaskP3Cfg::ETM_TASK_GPIO14_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO13_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio13_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP3Cfg::ETM_TASK_GPIO13_SEL_MASK)
                | ((value << EtmTaskP3Cfg::ETM_TASK_GPIO13_SEL_OFFSET)
                    & EtmTaskP3Cfg::ETM_TASK_GPIO13_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO13_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio13_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP3Cfg::ETM_TASK_GPIO13_EN_MASK
            } else {
                self.bits & !EtmTaskP3Cfg::ETM_TASK_GPIO13_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO12_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio12_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP3Cfg::ETM_TASK_GPIO12_SEL_MASK)
                | ((value << EtmTaskP3Cfg::ETM_TASK_GPIO12_SEL_OFFSET)
                    & EtmTaskP3Cfg::ETM_TASK_GPIO12_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO12_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio12_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP3Cfg::ETM_TASK_GPIO12_EN_MASK
            } else {
                self.bits & !EtmTaskP3Cfg::ETM_TASK_GPIO12_EN_MASK
            },
        }
    }
}

impl Default for EtmTaskP3CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmTaskP3Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmTaskP3CfgVal {
        EtmTaskP3CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmTaskP3CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmTaskP3CfgVal) -> EtmTaskP3CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_TASK_P4_CFG Register
///
/// Etm Configure Register to decide which GPIO been chosen
///
/// Address offset: 0x00b0
///
/// ## Fields
///
/// - **ETM_TASK_GPIO19_SEL** `[27:25]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO19_EN** `[24]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO18_SEL** `[19:17]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO18_EN** `[16]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO17_SEL** `[11:9]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO17_EN** `[8]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO16_SEL** `[3:1]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO16_EN** `[0]` (RW)
///
///   Enable bit of GPIO response etm task.
#[repr(transparent)]
pub struct EtmTaskP4Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmTaskP4Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmTaskP4Cfg {
    // ======== Field Constants ========

    /// **ETM_TASK_GPIO19_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const ETM_TASK_GPIO19_SEL_OFFSET: u32 = 25;
    pub const ETM_TASK_GPIO19_SEL_MASK: u32 = 0x0e000000;

    /// **ETM_TASK_GPIO19_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const ETM_TASK_GPIO19_EN_OFFSET: u32 = 24;
    pub const ETM_TASK_GPIO19_EN_MASK: u32 = 0x01000000;
    pub const ETM_TASK_GPIO19_EN: u32 = 1 << 24;

    /// **ETM_TASK_GPIO18_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const ETM_TASK_GPIO18_SEL_OFFSET: u32 = 17;
    pub const ETM_TASK_GPIO18_SEL_MASK: u32 = 0x000e0000;

    /// **ETM_TASK_GPIO18_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETM_TASK_GPIO18_EN_OFFSET: u32 = 16;
    pub const ETM_TASK_GPIO18_EN_MASK: u32 = 0x00010000;
    pub const ETM_TASK_GPIO18_EN: u32 = 1 << 16;

    /// **ETM_TASK_GPIO17_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const ETM_TASK_GPIO17_SEL_OFFSET: u32 = 9;
    pub const ETM_TASK_GPIO17_SEL_MASK: u32 = 0x00000e00;

    /// **ETM_TASK_GPIO17_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ETM_TASK_GPIO17_EN_OFFSET: u32 = 8;
    pub const ETM_TASK_GPIO17_EN_MASK: u32 = 0x00000100;
    pub const ETM_TASK_GPIO17_EN: u32 = 1 << 8;

    /// **ETM_TASK_GPIO16_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const ETM_TASK_GPIO16_SEL_OFFSET: u32 = 1;
    pub const ETM_TASK_GPIO16_SEL_MASK: u32 = 0x0000000e;

    /// **ETM_TASK_GPIO16_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_TASK_GPIO16_EN_OFFSET: u32 = 0;
    pub const ETM_TASK_GPIO16_EN_MASK: u32 = 0x00000001;
    pub const ETM_TASK_GPIO16_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ETM_TASK_GPIO19_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio19_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO19_SEL_MASK)
            >> Self::ETM_TASK_GPIO19_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO19_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio19_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO19_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO19_SEL_OFFSET) & Self::ETM_TASK_GPIO19_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO19_EN** `[24]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio19_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO19_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO19_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio19_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO19_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO19_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio19_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO19_EN_MASK);
    }

    /// **ETM_TASK_GPIO18_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio18_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO18_SEL_MASK)
            >> Self::ETM_TASK_GPIO18_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO18_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio18_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO18_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO18_SEL_OFFSET) & Self::ETM_TASK_GPIO18_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO18_EN** `[16]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio18_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO18_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO18_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio18_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO18_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO18_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio18_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO18_EN_MASK);
    }

    /// **ETM_TASK_GPIO17_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio17_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO17_SEL_MASK)
            >> Self::ETM_TASK_GPIO17_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO17_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio17_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO17_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO17_SEL_OFFSET) & Self::ETM_TASK_GPIO17_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO17_EN** `[8]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio17_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO17_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO17_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio17_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO17_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO17_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio17_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO17_EN_MASK);
    }

    /// **ETM_TASK_GPIO16_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio16_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO16_SEL_MASK)
            >> Self::ETM_TASK_GPIO16_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO16_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio16_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO16_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO16_SEL_OFFSET) & Self::ETM_TASK_GPIO16_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO16_EN** `[0]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio16_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO16_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO16_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio16_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO16_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO16_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio16_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO16_EN_MASK);
    }
}

/// ETM_TASK_P4_CFG value type for batch read/modify/write operations
///
/// Use `etmtaskp4cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmtaskp4cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmTaskP4CfgVal {
    bits: u32,
}

impl EtmTaskP4CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_TASK_GPIO19_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio19_sel(&self) -> u32 {
        (self.bits & EtmTaskP4Cfg::ETM_TASK_GPIO19_SEL_MASK)
            >> EtmTaskP4Cfg::ETM_TASK_GPIO19_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO19_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio19_en(&self) -> bool {
        (self.bits & EtmTaskP4Cfg::ETM_TASK_GPIO19_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO18_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio18_sel(&self) -> u32 {
        (self.bits & EtmTaskP4Cfg::ETM_TASK_GPIO18_SEL_MASK)
            >> EtmTaskP4Cfg::ETM_TASK_GPIO18_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO18_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio18_en(&self) -> bool {
        (self.bits & EtmTaskP4Cfg::ETM_TASK_GPIO18_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO17_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio17_sel(&self) -> u32 {
        (self.bits & EtmTaskP4Cfg::ETM_TASK_GPIO17_SEL_MASK)
            >> EtmTaskP4Cfg::ETM_TASK_GPIO17_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO17_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio17_en(&self) -> bool {
        (self.bits & EtmTaskP4Cfg::ETM_TASK_GPIO17_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO16_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio16_sel(&self) -> u32 {
        (self.bits & EtmTaskP4Cfg::ETM_TASK_GPIO16_SEL_MASK)
            >> EtmTaskP4Cfg::ETM_TASK_GPIO16_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO16_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio16_en(&self) -> bool {
        (self.bits & EtmTaskP4Cfg::ETM_TASK_GPIO16_EN_MASK) != 0
    }

    /// Set **ETM_TASK_GPIO19_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio19_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP4Cfg::ETM_TASK_GPIO19_SEL_MASK)
                | ((value << EtmTaskP4Cfg::ETM_TASK_GPIO19_SEL_OFFSET)
                    & EtmTaskP4Cfg::ETM_TASK_GPIO19_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO19_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio19_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP4Cfg::ETM_TASK_GPIO19_EN_MASK
            } else {
                self.bits & !EtmTaskP4Cfg::ETM_TASK_GPIO19_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO18_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio18_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP4Cfg::ETM_TASK_GPIO18_SEL_MASK)
                | ((value << EtmTaskP4Cfg::ETM_TASK_GPIO18_SEL_OFFSET)
                    & EtmTaskP4Cfg::ETM_TASK_GPIO18_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO18_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio18_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP4Cfg::ETM_TASK_GPIO18_EN_MASK
            } else {
                self.bits & !EtmTaskP4Cfg::ETM_TASK_GPIO18_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO17_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio17_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP4Cfg::ETM_TASK_GPIO17_SEL_MASK)
                | ((value << EtmTaskP4Cfg::ETM_TASK_GPIO17_SEL_OFFSET)
                    & EtmTaskP4Cfg::ETM_TASK_GPIO17_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO17_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio17_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP4Cfg::ETM_TASK_GPIO17_EN_MASK
            } else {
                self.bits & !EtmTaskP4Cfg::ETM_TASK_GPIO17_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO16_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio16_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP4Cfg::ETM_TASK_GPIO16_SEL_MASK)
                | ((value << EtmTaskP4Cfg::ETM_TASK_GPIO16_SEL_OFFSET)
                    & EtmTaskP4Cfg::ETM_TASK_GPIO16_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO16_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio16_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP4Cfg::ETM_TASK_GPIO16_EN_MASK
            } else {
                self.bits & !EtmTaskP4Cfg::ETM_TASK_GPIO16_EN_MASK
            },
        }
    }
}

impl Default for EtmTaskP4CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmTaskP4Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmTaskP4CfgVal {
        EtmTaskP4CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmTaskP4CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmTaskP4CfgVal) -> EtmTaskP4CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_TASK_P5_CFG Register
///
/// Etm Configure Register to decide which GPIO been chosen
///
/// Address offset: 0x00b4
///
/// ## Fields
///
/// - **ETM_TASK_GPIO23_SEL** `[27:25]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO23_EN** `[24]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO22_SEL** `[19:17]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO22_EN** `[16]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO21_SEL** `[11:9]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO21_EN** `[8]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO20_SEL** `[3:1]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO20_EN** `[0]` (RW)
///
///   Enable bit of GPIO response etm task.
#[repr(transparent)]
pub struct EtmTaskP5Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmTaskP5Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmTaskP5Cfg {
    // ======== Field Constants ========

    /// **ETM_TASK_GPIO23_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const ETM_TASK_GPIO23_SEL_OFFSET: u32 = 25;
    pub const ETM_TASK_GPIO23_SEL_MASK: u32 = 0x0e000000;

    /// **ETM_TASK_GPIO23_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const ETM_TASK_GPIO23_EN_OFFSET: u32 = 24;
    pub const ETM_TASK_GPIO23_EN_MASK: u32 = 0x01000000;
    pub const ETM_TASK_GPIO23_EN: u32 = 1 << 24;

    /// **ETM_TASK_GPIO22_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const ETM_TASK_GPIO22_SEL_OFFSET: u32 = 17;
    pub const ETM_TASK_GPIO22_SEL_MASK: u32 = 0x000e0000;

    /// **ETM_TASK_GPIO22_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETM_TASK_GPIO22_EN_OFFSET: u32 = 16;
    pub const ETM_TASK_GPIO22_EN_MASK: u32 = 0x00010000;
    pub const ETM_TASK_GPIO22_EN: u32 = 1 << 16;

    /// **ETM_TASK_GPIO21_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const ETM_TASK_GPIO21_SEL_OFFSET: u32 = 9;
    pub const ETM_TASK_GPIO21_SEL_MASK: u32 = 0x00000e00;

    /// **ETM_TASK_GPIO21_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ETM_TASK_GPIO21_EN_OFFSET: u32 = 8;
    pub const ETM_TASK_GPIO21_EN_MASK: u32 = 0x00000100;
    pub const ETM_TASK_GPIO21_EN: u32 = 1 << 8;

    /// **ETM_TASK_GPIO20_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const ETM_TASK_GPIO20_SEL_OFFSET: u32 = 1;
    pub const ETM_TASK_GPIO20_SEL_MASK: u32 = 0x0000000e;

    /// **ETM_TASK_GPIO20_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_TASK_GPIO20_EN_OFFSET: u32 = 0;
    pub const ETM_TASK_GPIO20_EN_MASK: u32 = 0x00000001;
    pub const ETM_TASK_GPIO20_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ETM_TASK_GPIO23_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio23_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO23_SEL_MASK)
            >> Self::ETM_TASK_GPIO23_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO23_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio23_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO23_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO23_SEL_OFFSET) & Self::ETM_TASK_GPIO23_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO23_EN** `[24]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio23_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO23_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO23_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio23_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO23_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO23_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio23_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO23_EN_MASK);
    }

    /// **ETM_TASK_GPIO22_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio22_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO22_SEL_MASK)
            >> Self::ETM_TASK_GPIO22_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO22_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio22_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO22_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO22_SEL_OFFSET) & Self::ETM_TASK_GPIO22_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO22_EN** `[16]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio22_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO22_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO22_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio22_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO22_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO22_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio22_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO22_EN_MASK);
    }

    /// **ETM_TASK_GPIO21_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio21_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO21_SEL_MASK)
            >> Self::ETM_TASK_GPIO21_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO21_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio21_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO21_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO21_SEL_OFFSET) & Self::ETM_TASK_GPIO21_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO21_EN** `[8]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio21_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO21_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO21_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio21_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO21_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO21_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio21_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO21_EN_MASK);
    }

    /// **ETM_TASK_GPIO20_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio20_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO20_SEL_MASK)
            >> Self::ETM_TASK_GPIO20_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO20_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio20_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO20_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO20_SEL_OFFSET) & Self::ETM_TASK_GPIO20_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO20_EN** `[0]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio20_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO20_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO20_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio20_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO20_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO20_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio20_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO20_EN_MASK);
    }
}

/// ETM_TASK_P5_CFG value type for batch read/modify/write operations
///
/// Use `etmtaskp5cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmtaskp5cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmTaskP5CfgVal {
    bits: u32,
}

impl EtmTaskP5CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_TASK_GPIO23_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio23_sel(&self) -> u32 {
        (self.bits & EtmTaskP5Cfg::ETM_TASK_GPIO23_SEL_MASK)
            >> EtmTaskP5Cfg::ETM_TASK_GPIO23_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO23_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio23_en(&self) -> bool {
        (self.bits & EtmTaskP5Cfg::ETM_TASK_GPIO23_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO22_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio22_sel(&self) -> u32 {
        (self.bits & EtmTaskP5Cfg::ETM_TASK_GPIO22_SEL_MASK)
            >> EtmTaskP5Cfg::ETM_TASK_GPIO22_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO22_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio22_en(&self) -> bool {
        (self.bits & EtmTaskP5Cfg::ETM_TASK_GPIO22_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO21_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio21_sel(&self) -> u32 {
        (self.bits & EtmTaskP5Cfg::ETM_TASK_GPIO21_SEL_MASK)
            >> EtmTaskP5Cfg::ETM_TASK_GPIO21_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO21_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio21_en(&self) -> bool {
        (self.bits & EtmTaskP5Cfg::ETM_TASK_GPIO21_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO20_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio20_sel(&self) -> u32 {
        (self.bits & EtmTaskP5Cfg::ETM_TASK_GPIO20_SEL_MASK)
            >> EtmTaskP5Cfg::ETM_TASK_GPIO20_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO20_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio20_en(&self) -> bool {
        (self.bits & EtmTaskP5Cfg::ETM_TASK_GPIO20_EN_MASK) != 0
    }

    /// Set **ETM_TASK_GPIO23_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio23_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP5Cfg::ETM_TASK_GPIO23_SEL_MASK)
                | ((value << EtmTaskP5Cfg::ETM_TASK_GPIO23_SEL_OFFSET)
                    & EtmTaskP5Cfg::ETM_TASK_GPIO23_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO23_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio23_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP5Cfg::ETM_TASK_GPIO23_EN_MASK
            } else {
                self.bits & !EtmTaskP5Cfg::ETM_TASK_GPIO23_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO22_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio22_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP5Cfg::ETM_TASK_GPIO22_SEL_MASK)
                | ((value << EtmTaskP5Cfg::ETM_TASK_GPIO22_SEL_OFFSET)
                    & EtmTaskP5Cfg::ETM_TASK_GPIO22_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO22_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio22_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP5Cfg::ETM_TASK_GPIO22_EN_MASK
            } else {
                self.bits & !EtmTaskP5Cfg::ETM_TASK_GPIO22_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO21_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio21_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP5Cfg::ETM_TASK_GPIO21_SEL_MASK)
                | ((value << EtmTaskP5Cfg::ETM_TASK_GPIO21_SEL_OFFSET)
                    & EtmTaskP5Cfg::ETM_TASK_GPIO21_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO21_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio21_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP5Cfg::ETM_TASK_GPIO21_EN_MASK
            } else {
                self.bits & !EtmTaskP5Cfg::ETM_TASK_GPIO21_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO20_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio20_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP5Cfg::ETM_TASK_GPIO20_SEL_MASK)
                | ((value << EtmTaskP5Cfg::ETM_TASK_GPIO20_SEL_OFFSET)
                    & EtmTaskP5Cfg::ETM_TASK_GPIO20_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO20_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio20_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP5Cfg::ETM_TASK_GPIO20_EN_MASK
            } else {
                self.bits & !EtmTaskP5Cfg::ETM_TASK_GPIO20_EN_MASK
            },
        }
    }
}

impl Default for EtmTaskP5CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmTaskP5Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmTaskP5CfgVal {
        EtmTaskP5CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmTaskP5CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmTaskP5CfgVal) -> EtmTaskP5CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_TASK_P6_CFG Register
///
/// Etm Configure Register to decide which GPIO been chosen
///
/// Address offset: 0x00b8
///
/// ## Fields
///
/// - **ETM_TASK_GPIO27_SEL** `[27:25]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO27_EN** `[24]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO26_SEL** `[19:17]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO26_EN** `[16]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO25_SEL** `[11:9]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO25_EN** `[8]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO24_SEL** `[3:1]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO24_EN** `[0]` (RW)
///
///   Enable bit of GPIO response etm task.
#[repr(transparent)]
pub struct EtmTaskP6Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmTaskP6Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmTaskP6Cfg {
    // ======== Field Constants ========

    /// **ETM_TASK_GPIO27_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const ETM_TASK_GPIO27_SEL_OFFSET: u32 = 25;
    pub const ETM_TASK_GPIO27_SEL_MASK: u32 = 0x0e000000;

    /// **ETM_TASK_GPIO27_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const ETM_TASK_GPIO27_EN_OFFSET: u32 = 24;
    pub const ETM_TASK_GPIO27_EN_MASK: u32 = 0x01000000;
    pub const ETM_TASK_GPIO27_EN: u32 = 1 << 24;

    /// **ETM_TASK_GPIO26_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const ETM_TASK_GPIO26_SEL_OFFSET: u32 = 17;
    pub const ETM_TASK_GPIO26_SEL_MASK: u32 = 0x000e0000;

    /// **ETM_TASK_GPIO26_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETM_TASK_GPIO26_EN_OFFSET: u32 = 16;
    pub const ETM_TASK_GPIO26_EN_MASK: u32 = 0x00010000;
    pub const ETM_TASK_GPIO26_EN: u32 = 1 << 16;

    /// **ETM_TASK_GPIO25_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const ETM_TASK_GPIO25_SEL_OFFSET: u32 = 9;
    pub const ETM_TASK_GPIO25_SEL_MASK: u32 = 0x00000e00;

    /// **ETM_TASK_GPIO25_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ETM_TASK_GPIO25_EN_OFFSET: u32 = 8;
    pub const ETM_TASK_GPIO25_EN_MASK: u32 = 0x00000100;
    pub const ETM_TASK_GPIO25_EN: u32 = 1 << 8;

    /// **ETM_TASK_GPIO24_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const ETM_TASK_GPIO24_SEL_OFFSET: u32 = 1;
    pub const ETM_TASK_GPIO24_SEL_MASK: u32 = 0x0000000e;

    /// **ETM_TASK_GPIO24_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_TASK_GPIO24_EN_OFFSET: u32 = 0;
    pub const ETM_TASK_GPIO24_EN_MASK: u32 = 0x00000001;
    pub const ETM_TASK_GPIO24_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ETM_TASK_GPIO27_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio27_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO27_SEL_MASK)
            >> Self::ETM_TASK_GPIO27_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO27_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio27_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO27_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO27_SEL_OFFSET) & Self::ETM_TASK_GPIO27_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO27_EN** `[24]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio27_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO27_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO27_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio27_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO27_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO27_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio27_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO27_EN_MASK);
    }

    /// **ETM_TASK_GPIO26_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio26_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO26_SEL_MASK)
            >> Self::ETM_TASK_GPIO26_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO26_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio26_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO26_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO26_SEL_OFFSET) & Self::ETM_TASK_GPIO26_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO26_EN** `[16]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio26_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO26_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO26_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio26_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO26_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO26_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio26_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO26_EN_MASK);
    }

    /// **ETM_TASK_GPIO25_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio25_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO25_SEL_MASK)
            >> Self::ETM_TASK_GPIO25_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO25_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio25_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO25_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO25_SEL_OFFSET) & Self::ETM_TASK_GPIO25_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO25_EN** `[8]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio25_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO25_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO25_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio25_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO25_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO25_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio25_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO25_EN_MASK);
    }

    /// **ETM_TASK_GPIO24_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio24_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO24_SEL_MASK)
            >> Self::ETM_TASK_GPIO24_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO24_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio24_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO24_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO24_SEL_OFFSET) & Self::ETM_TASK_GPIO24_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO24_EN** `[0]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio24_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO24_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO24_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio24_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO24_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO24_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio24_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO24_EN_MASK);
    }
}

/// ETM_TASK_P6_CFG value type for batch read/modify/write operations
///
/// Use `etmtaskp6cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmtaskp6cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmTaskP6CfgVal {
    bits: u32,
}

impl EtmTaskP6CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_TASK_GPIO27_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio27_sel(&self) -> u32 {
        (self.bits & EtmTaskP6Cfg::ETM_TASK_GPIO27_SEL_MASK)
            >> EtmTaskP6Cfg::ETM_TASK_GPIO27_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO27_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio27_en(&self) -> bool {
        (self.bits & EtmTaskP6Cfg::ETM_TASK_GPIO27_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO26_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio26_sel(&self) -> u32 {
        (self.bits & EtmTaskP6Cfg::ETM_TASK_GPIO26_SEL_MASK)
            >> EtmTaskP6Cfg::ETM_TASK_GPIO26_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO26_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio26_en(&self) -> bool {
        (self.bits & EtmTaskP6Cfg::ETM_TASK_GPIO26_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO25_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio25_sel(&self) -> u32 {
        (self.bits & EtmTaskP6Cfg::ETM_TASK_GPIO25_SEL_MASK)
            >> EtmTaskP6Cfg::ETM_TASK_GPIO25_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO25_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio25_en(&self) -> bool {
        (self.bits & EtmTaskP6Cfg::ETM_TASK_GPIO25_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO24_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio24_sel(&self) -> u32 {
        (self.bits & EtmTaskP6Cfg::ETM_TASK_GPIO24_SEL_MASK)
            >> EtmTaskP6Cfg::ETM_TASK_GPIO24_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO24_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio24_en(&self) -> bool {
        (self.bits & EtmTaskP6Cfg::ETM_TASK_GPIO24_EN_MASK) != 0
    }

    /// Set **ETM_TASK_GPIO27_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio27_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP6Cfg::ETM_TASK_GPIO27_SEL_MASK)
                | ((value << EtmTaskP6Cfg::ETM_TASK_GPIO27_SEL_OFFSET)
                    & EtmTaskP6Cfg::ETM_TASK_GPIO27_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO27_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio27_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP6Cfg::ETM_TASK_GPIO27_EN_MASK
            } else {
                self.bits & !EtmTaskP6Cfg::ETM_TASK_GPIO27_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO26_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio26_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP6Cfg::ETM_TASK_GPIO26_SEL_MASK)
                | ((value << EtmTaskP6Cfg::ETM_TASK_GPIO26_SEL_OFFSET)
                    & EtmTaskP6Cfg::ETM_TASK_GPIO26_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO26_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio26_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP6Cfg::ETM_TASK_GPIO26_EN_MASK
            } else {
                self.bits & !EtmTaskP6Cfg::ETM_TASK_GPIO26_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO25_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio25_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP6Cfg::ETM_TASK_GPIO25_SEL_MASK)
                | ((value << EtmTaskP6Cfg::ETM_TASK_GPIO25_SEL_OFFSET)
                    & EtmTaskP6Cfg::ETM_TASK_GPIO25_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO25_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio25_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP6Cfg::ETM_TASK_GPIO25_EN_MASK
            } else {
                self.bits & !EtmTaskP6Cfg::ETM_TASK_GPIO25_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO24_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio24_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP6Cfg::ETM_TASK_GPIO24_SEL_MASK)
                | ((value << EtmTaskP6Cfg::ETM_TASK_GPIO24_SEL_OFFSET)
                    & EtmTaskP6Cfg::ETM_TASK_GPIO24_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO24_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio24_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP6Cfg::ETM_TASK_GPIO24_EN_MASK
            } else {
                self.bits & !EtmTaskP6Cfg::ETM_TASK_GPIO24_EN_MASK
            },
        }
    }
}

impl Default for EtmTaskP6CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmTaskP6Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmTaskP6CfgVal {
        EtmTaskP6CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmTaskP6CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmTaskP6CfgVal) -> EtmTaskP6CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_TASK_P7_CFG Register
///
/// Etm Configure Register to decide which GPIO been chosen
///
/// Address offset: 0x00bc
///
/// ## Fields
///
/// - **ETM_TASK_GPIO31_SEL** `[27:25]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO31_EN** `[24]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO30_SEL** `[19:17]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO30_EN** `[16]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO29_SEL** `[11:9]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO29_EN** `[8]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO28_SEL** `[3:1]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO28_EN** `[0]` (RW)
///
///   Enable bit of GPIO response etm task.
#[repr(transparent)]
pub struct EtmTaskP7Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmTaskP7Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmTaskP7Cfg {
    // ======== Field Constants ========

    /// **ETM_TASK_GPIO31_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const ETM_TASK_GPIO31_SEL_OFFSET: u32 = 25;
    pub const ETM_TASK_GPIO31_SEL_MASK: u32 = 0x0e000000;

    /// **ETM_TASK_GPIO31_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const ETM_TASK_GPIO31_EN_OFFSET: u32 = 24;
    pub const ETM_TASK_GPIO31_EN_MASK: u32 = 0x01000000;
    pub const ETM_TASK_GPIO31_EN: u32 = 1 << 24;

    /// **ETM_TASK_GPIO30_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const ETM_TASK_GPIO30_SEL_OFFSET: u32 = 17;
    pub const ETM_TASK_GPIO30_SEL_MASK: u32 = 0x000e0000;

    /// **ETM_TASK_GPIO30_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETM_TASK_GPIO30_EN_OFFSET: u32 = 16;
    pub const ETM_TASK_GPIO30_EN_MASK: u32 = 0x00010000;
    pub const ETM_TASK_GPIO30_EN: u32 = 1 << 16;

    /// **ETM_TASK_GPIO29_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const ETM_TASK_GPIO29_SEL_OFFSET: u32 = 9;
    pub const ETM_TASK_GPIO29_SEL_MASK: u32 = 0x00000e00;

    /// **ETM_TASK_GPIO29_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ETM_TASK_GPIO29_EN_OFFSET: u32 = 8;
    pub const ETM_TASK_GPIO29_EN_MASK: u32 = 0x00000100;
    pub const ETM_TASK_GPIO29_EN: u32 = 1 << 8;

    /// **ETM_TASK_GPIO28_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const ETM_TASK_GPIO28_SEL_OFFSET: u32 = 1;
    pub const ETM_TASK_GPIO28_SEL_MASK: u32 = 0x0000000e;

    /// **ETM_TASK_GPIO28_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_TASK_GPIO28_EN_OFFSET: u32 = 0;
    pub const ETM_TASK_GPIO28_EN_MASK: u32 = 0x00000001;
    pub const ETM_TASK_GPIO28_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ETM_TASK_GPIO31_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio31_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO31_SEL_MASK)
            >> Self::ETM_TASK_GPIO31_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO31_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio31_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO31_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO31_SEL_OFFSET) & Self::ETM_TASK_GPIO31_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO31_EN** `[24]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio31_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO31_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO31_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio31_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO31_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO31_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio31_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO31_EN_MASK);
    }

    /// **ETM_TASK_GPIO30_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio30_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO30_SEL_MASK)
            >> Self::ETM_TASK_GPIO30_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO30_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio30_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO30_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO30_SEL_OFFSET) & Self::ETM_TASK_GPIO30_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO30_EN** `[16]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio30_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO30_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO30_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio30_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO30_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO30_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio30_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO30_EN_MASK);
    }

    /// **ETM_TASK_GPIO29_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio29_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO29_SEL_MASK)
            >> Self::ETM_TASK_GPIO29_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO29_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio29_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO29_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO29_SEL_OFFSET) & Self::ETM_TASK_GPIO29_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO29_EN** `[8]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio29_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO29_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO29_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio29_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO29_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO29_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio29_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO29_EN_MASK);
    }

    /// **ETM_TASK_GPIO28_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio28_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO28_SEL_MASK)
            >> Self::ETM_TASK_GPIO28_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO28_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio28_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO28_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO28_SEL_OFFSET) & Self::ETM_TASK_GPIO28_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO28_EN** `[0]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio28_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO28_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO28_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio28_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO28_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO28_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio28_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO28_EN_MASK);
    }
}

/// ETM_TASK_P7_CFG value type for batch read/modify/write operations
///
/// Use `etmtaskp7cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmtaskp7cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmTaskP7CfgVal {
    bits: u32,
}

impl EtmTaskP7CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_TASK_GPIO31_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio31_sel(&self) -> u32 {
        (self.bits & EtmTaskP7Cfg::ETM_TASK_GPIO31_SEL_MASK)
            >> EtmTaskP7Cfg::ETM_TASK_GPIO31_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO31_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio31_en(&self) -> bool {
        (self.bits & EtmTaskP7Cfg::ETM_TASK_GPIO31_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO30_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio30_sel(&self) -> u32 {
        (self.bits & EtmTaskP7Cfg::ETM_TASK_GPIO30_SEL_MASK)
            >> EtmTaskP7Cfg::ETM_TASK_GPIO30_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO30_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio30_en(&self) -> bool {
        (self.bits & EtmTaskP7Cfg::ETM_TASK_GPIO30_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO29_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio29_sel(&self) -> u32 {
        (self.bits & EtmTaskP7Cfg::ETM_TASK_GPIO29_SEL_MASK)
            >> EtmTaskP7Cfg::ETM_TASK_GPIO29_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO29_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio29_en(&self) -> bool {
        (self.bits & EtmTaskP7Cfg::ETM_TASK_GPIO29_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO28_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio28_sel(&self) -> u32 {
        (self.bits & EtmTaskP7Cfg::ETM_TASK_GPIO28_SEL_MASK)
            >> EtmTaskP7Cfg::ETM_TASK_GPIO28_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO28_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio28_en(&self) -> bool {
        (self.bits & EtmTaskP7Cfg::ETM_TASK_GPIO28_EN_MASK) != 0
    }

    /// Set **ETM_TASK_GPIO31_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio31_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP7Cfg::ETM_TASK_GPIO31_SEL_MASK)
                | ((value << EtmTaskP7Cfg::ETM_TASK_GPIO31_SEL_OFFSET)
                    & EtmTaskP7Cfg::ETM_TASK_GPIO31_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO31_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio31_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP7Cfg::ETM_TASK_GPIO31_EN_MASK
            } else {
                self.bits & !EtmTaskP7Cfg::ETM_TASK_GPIO31_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO30_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio30_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP7Cfg::ETM_TASK_GPIO30_SEL_MASK)
                | ((value << EtmTaskP7Cfg::ETM_TASK_GPIO30_SEL_OFFSET)
                    & EtmTaskP7Cfg::ETM_TASK_GPIO30_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO30_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio30_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP7Cfg::ETM_TASK_GPIO30_EN_MASK
            } else {
                self.bits & !EtmTaskP7Cfg::ETM_TASK_GPIO30_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO29_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio29_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP7Cfg::ETM_TASK_GPIO29_SEL_MASK)
                | ((value << EtmTaskP7Cfg::ETM_TASK_GPIO29_SEL_OFFSET)
                    & EtmTaskP7Cfg::ETM_TASK_GPIO29_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO29_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio29_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP7Cfg::ETM_TASK_GPIO29_EN_MASK
            } else {
                self.bits & !EtmTaskP7Cfg::ETM_TASK_GPIO29_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO28_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio28_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP7Cfg::ETM_TASK_GPIO28_SEL_MASK)
                | ((value << EtmTaskP7Cfg::ETM_TASK_GPIO28_SEL_OFFSET)
                    & EtmTaskP7Cfg::ETM_TASK_GPIO28_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO28_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio28_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP7Cfg::ETM_TASK_GPIO28_EN_MASK
            } else {
                self.bits & !EtmTaskP7Cfg::ETM_TASK_GPIO28_EN_MASK
            },
        }
    }
}

impl Default for EtmTaskP7CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmTaskP7Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmTaskP7CfgVal {
        EtmTaskP7CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmTaskP7CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmTaskP7CfgVal) -> EtmTaskP7CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_TASK_P8_CFG Register
///
/// Etm Configure Register to decide which GPIO been chosen
///
/// Address offset: 0x00c0
///
/// ## Fields
///
/// - **ETM_TASK_GPIO35_SEL** `[27:25]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO35_EN** `[24]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO34_SEL** `[19:17]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO34_EN** `[16]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO33_SEL** `[11:9]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO33_EN** `[8]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO32_SEL** `[3:1]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO32_EN** `[0]` (RW)
///
///   Enable bit of GPIO response etm task.
#[repr(transparent)]
pub struct EtmTaskP8Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmTaskP8Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmTaskP8Cfg {
    // ======== Field Constants ========

    /// **ETM_TASK_GPIO35_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const ETM_TASK_GPIO35_SEL_OFFSET: u32 = 25;
    pub const ETM_TASK_GPIO35_SEL_MASK: u32 = 0x0e000000;

    /// **ETM_TASK_GPIO35_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const ETM_TASK_GPIO35_EN_OFFSET: u32 = 24;
    pub const ETM_TASK_GPIO35_EN_MASK: u32 = 0x01000000;
    pub const ETM_TASK_GPIO35_EN: u32 = 1 << 24;

    /// **ETM_TASK_GPIO34_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const ETM_TASK_GPIO34_SEL_OFFSET: u32 = 17;
    pub const ETM_TASK_GPIO34_SEL_MASK: u32 = 0x000e0000;

    /// **ETM_TASK_GPIO34_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETM_TASK_GPIO34_EN_OFFSET: u32 = 16;
    pub const ETM_TASK_GPIO34_EN_MASK: u32 = 0x00010000;
    pub const ETM_TASK_GPIO34_EN: u32 = 1 << 16;

    /// **ETM_TASK_GPIO33_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const ETM_TASK_GPIO33_SEL_OFFSET: u32 = 9;
    pub const ETM_TASK_GPIO33_SEL_MASK: u32 = 0x00000e00;

    /// **ETM_TASK_GPIO33_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ETM_TASK_GPIO33_EN_OFFSET: u32 = 8;
    pub const ETM_TASK_GPIO33_EN_MASK: u32 = 0x00000100;
    pub const ETM_TASK_GPIO33_EN: u32 = 1 << 8;

    /// **ETM_TASK_GPIO32_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const ETM_TASK_GPIO32_SEL_OFFSET: u32 = 1;
    pub const ETM_TASK_GPIO32_SEL_MASK: u32 = 0x0000000e;

    /// **ETM_TASK_GPIO32_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_TASK_GPIO32_EN_OFFSET: u32 = 0;
    pub const ETM_TASK_GPIO32_EN_MASK: u32 = 0x00000001;
    pub const ETM_TASK_GPIO32_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ETM_TASK_GPIO35_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio35_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO35_SEL_MASK)
            >> Self::ETM_TASK_GPIO35_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO35_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio35_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO35_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO35_SEL_OFFSET) & Self::ETM_TASK_GPIO35_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO35_EN** `[24]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio35_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO35_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO35_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio35_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO35_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO35_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio35_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO35_EN_MASK);
    }

    /// **ETM_TASK_GPIO34_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio34_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO34_SEL_MASK)
            >> Self::ETM_TASK_GPIO34_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO34_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio34_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO34_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO34_SEL_OFFSET) & Self::ETM_TASK_GPIO34_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO34_EN** `[16]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio34_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO34_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO34_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio34_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO34_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO34_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio34_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO34_EN_MASK);
    }

    /// **ETM_TASK_GPIO33_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio33_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO33_SEL_MASK)
            >> Self::ETM_TASK_GPIO33_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO33_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio33_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO33_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO33_SEL_OFFSET) & Self::ETM_TASK_GPIO33_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO33_EN** `[8]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio33_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO33_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO33_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio33_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO33_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO33_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio33_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO33_EN_MASK);
    }

    /// **ETM_TASK_GPIO32_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio32_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO32_SEL_MASK)
            >> Self::ETM_TASK_GPIO32_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO32_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio32_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO32_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO32_SEL_OFFSET) & Self::ETM_TASK_GPIO32_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO32_EN** `[0]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio32_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO32_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO32_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio32_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO32_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO32_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio32_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO32_EN_MASK);
    }
}

/// ETM_TASK_P8_CFG value type for batch read/modify/write operations
///
/// Use `etmtaskp8cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmtaskp8cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmTaskP8CfgVal {
    bits: u32,
}

impl EtmTaskP8CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_TASK_GPIO35_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio35_sel(&self) -> u32 {
        (self.bits & EtmTaskP8Cfg::ETM_TASK_GPIO35_SEL_MASK)
            >> EtmTaskP8Cfg::ETM_TASK_GPIO35_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO35_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio35_en(&self) -> bool {
        (self.bits & EtmTaskP8Cfg::ETM_TASK_GPIO35_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO34_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio34_sel(&self) -> u32 {
        (self.bits & EtmTaskP8Cfg::ETM_TASK_GPIO34_SEL_MASK)
            >> EtmTaskP8Cfg::ETM_TASK_GPIO34_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO34_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio34_en(&self) -> bool {
        (self.bits & EtmTaskP8Cfg::ETM_TASK_GPIO34_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO33_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio33_sel(&self) -> u32 {
        (self.bits & EtmTaskP8Cfg::ETM_TASK_GPIO33_SEL_MASK)
            >> EtmTaskP8Cfg::ETM_TASK_GPIO33_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO33_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio33_en(&self) -> bool {
        (self.bits & EtmTaskP8Cfg::ETM_TASK_GPIO33_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO32_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio32_sel(&self) -> u32 {
        (self.bits & EtmTaskP8Cfg::ETM_TASK_GPIO32_SEL_MASK)
            >> EtmTaskP8Cfg::ETM_TASK_GPIO32_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO32_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio32_en(&self) -> bool {
        (self.bits & EtmTaskP8Cfg::ETM_TASK_GPIO32_EN_MASK) != 0
    }

    /// Set **ETM_TASK_GPIO35_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio35_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP8Cfg::ETM_TASK_GPIO35_SEL_MASK)
                | ((value << EtmTaskP8Cfg::ETM_TASK_GPIO35_SEL_OFFSET)
                    & EtmTaskP8Cfg::ETM_TASK_GPIO35_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO35_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio35_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP8Cfg::ETM_TASK_GPIO35_EN_MASK
            } else {
                self.bits & !EtmTaskP8Cfg::ETM_TASK_GPIO35_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO34_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio34_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP8Cfg::ETM_TASK_GPIO34_SEL_MASK)
                | ((value << EtmTaskP8Cfg::ETM_TASK_GPIO34_SEL_OFFSET)
                    & EtmTaskP8Cfg::ETM_TASK_GPIO34_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO34_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio34_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP8Cfg::ETM_TASK_GPIO34_EN_MASK
            } else {
                self.bits & !EtmTaskP8Cfg::ETM_TASK_GPIO34_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO33_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio33_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP8Cfg::ETM_TASK_GPIO33_SEL_MASK)
                | ((value << EtmTaskP8Cfg::ETM_TASK_GPIO33_SEL_OFFSET)
                    & EtmTaskP8Cfg::ETM_TASK_GPIO33_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO33_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio33_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP8Cfg::ETM_TASK_GPIO33_EN_MASK
            } else {
                self.bits & !EtmTaskP8Cfg::ETM_TASK_GPIO33_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO32_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio32_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP8Cfg::ETM_TASK_GPIO32_SEL_MASK)
                | ((value << EtmTaskP8Cfg::ETM_TASK_GPIO32_SEL_OFFSET)
                    & EtmTaskP8Cfg::ETM_TASK_GPIO32_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO32_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio32_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP8Cfg::ETM_TASK_GPIO32_EN_MASK
            } else {
                self.bits & !EtmTaskP8Cfg::ETM_TASK_GPIO32_EN_MASK
            },
        }
    }
}

impl Default for EtmTaskP8CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmTaskP8Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmTaskP8CfgVal {
        EtmTaskP8CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmTaskP8CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmTaskP8CfgVal) -> EtmTaskP8CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_TASK_P9_CFG Register
///
/// Etm Configure Register to decide which GPIO been chosen
///
/// Address offset: 0x00c4
///
/// ## Fields
///
/// - **ETM_TASK_GPIO39_SEL** `[27:25]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO39_EN** `[24]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO38_SEL** `[19:17]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO38_EN** `[16]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO37_SEL** `[11:9]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO37_EN** `[8]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO36_SEL** `[3:1]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO36_EN** `[0]` (RW)
///
///   Enable bit of GPIO response etm task.
#[repr(transparent)]
pub struct EtmTaskP9Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmTaskP9Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmTaskP9Cfg {
    // ======== Field Constants ========

    /// **ETM_TASK_GPIO39_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const ETM_TASK_GPIO39_SEL_OFFSET: u32 = 25;
    pub const ETM_TASK_GPIO39_SEL_MASK: u32 = 0x0e000000;

    /// **ETM_TASK_GPIO39_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const ETM_TASK_GPIO39_EN_OFFSET: u32 = 24;
    pub const ETM_TASK_GPIO39_EN_MASK: u32 = 0x01000000;
    pub const ETM_TASK_GPIO39_EN: u32 = 1 << 24;

    /// **ETM_TASK_GPIO38_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const ETM_TASK_GPIO38_SEL_OFFSET: u32 = 17;
    pub const ETM_TASK_GPIO38_SEL_MASK: u32 = 0x000e0000;

    /// **ETM_TASK_GPIO38_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETM_TASK_GPIO38_EN_OFFSET: u32 = 16;
    pub const ETM_TASK_GPIO38_EN_MASK: u32 = 0x00010000;
    pub const ETM_TASK_GPIO38_EN: u32 = 1 << 16;

    /// **ETM_TASK_GPIO37_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const ETM_TASK_GPIO37_SEL_OFFSET: u32 = 9;
    pub const ETM_TASK_GPIO37_SEL_MASK: u32 = 0x00000e00;

    /// **ETM_TASK_GPIO37_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ETM_TASK_GPIO37_EN_OFFSET: u32 = 8;
    pub const ETM_TASK_GPIO37_EN_MASK: u32 = 0x00000100;
    pub const ETM_TASK_GPIO37_EN: u32 = 1 << 8;

    /// **ETM_TASK_GPIO36_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const ETM_TASK_GPIO36_SEL_OFFSET: u32 = 1;
    pub const ETM_TASK_GPIO36_SEL_MASK: u32 = 0x0000000e;

    /// **ETM_TASK_GPIO36_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_TASK_GPIO36_EN_OFFSET: u32 = 0;
    pub const ETM_TASK_GPIO36_EN_MASK: u32 = 0x00000001;
    pub const ETM_TASK_GPIO36_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ETM_TASK_GPIO39_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio39_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO39_SEL_MASK)
            >> Self::ETM_TASK_GPIO39_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO39_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio39_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO39_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO39_SEL_OFFSET) & Self::ETM_TASK_GPIO39_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO39_EN** `[24]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio39_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO39_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO39_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio39_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO39_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO39_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio39_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO39_EN_MASK);
    }

    /// **ETM_TASK_GPIO38_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio38_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO38_SEL_MASK)
            >> Self::ETM_TASK_GPIO38_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO38_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio38_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO38_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO38_SEL_OFFSET) & Self::ETM_TASK_GPIO38_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO38_EN** `[16]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio38_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO38_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO38_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio38_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO38_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO38_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio38_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO38_EN_MASK);
    }

    /// **ETM_TASK_GPIO37_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio37_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO37_SEL_MASK)
            >> Self::ETM_TASK_GPIO37_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO37_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio37_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO37_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO37_SEL_OFFSET) & Self::ETM_TASK_GPIO37_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO37_EN** `[8]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio37_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO37_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO37_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio37_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO37_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO37_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio37_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO37_EN_MASK);
    }

    /// **ETM_TASK_GPIO36_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio36_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO36_SEL_MASK)
            >> Self::ETM_TASK_GPIO36_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO36_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio36_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO36_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO36_SEL_OFFSET) & Self::ETM_TASK_GPIO36_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO36_EN** `[0]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio36_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO36_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO36_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio36_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO36_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO36_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio36_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO36_EN_MASK);
    }
}

/// ETM_TASK_P9_CFG value type for batch read/modify/write operations
///
/// Use `etmtaskp9cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmtaskp9cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmTaskP9CfgVal {
    bits: u32,
}

impl EtmTaskP9CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_TASK_GPIO39_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio39_sel(&self) -> u32 {
        (self.bits & EtmTaskP9Cfg::ETM_TASK_GPIO39_SEL_MASK)
            >> EtmTaskP9Cfg::ETM_TASK_GPIO39_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO39_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio39_en(&self) -> bool {
        (self.bits & EtmTaskP9Cfg::ETM_TASK_GPIO39_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO38_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio38_sel(&self) -> u32 {
        (self.bits & EtmTaskP9Cfg::ETM_TASK_GPIO38_SEL_MASK)
            >> EtmTaskP9Cfg::ETM_TASK_GPIO38_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO38_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio38_en(&self) -> bool {
        (self.bits & EtmTaskP9Cfg::ETM_TASK_GPIO38_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO37_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio37_sel(&self) -> u32 {
        (self.bits & EtmTaskP9Cfg::ETM_TASK_GPIO37_SEL_MASK)
            >> EtmTaskP9Cfg::ETM_TASK_GPIO37_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO37_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio37_en(&self) -> bool {
        (self.bits & EtmTaskP9Cfg::ETM_TASK_GPIO37_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO36_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio36_sel(&self) -> u32 {
        (self.bits & EtmTaskP9Cfg::ETM_TASK_GPIO36_SEL_MASK)
            >> EtmTaskP9Cfg::ETM_TASK_GPIO36_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO36_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio36_en(&self) -> bool {
        (self.bits & EtmTaskP9Cfg::ETM_TASK_GPIO36_EN_MASK) != 0
    }

    /// Set **ETM_TASK_GPIO39_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio39_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP9Cfg::ETM_TASK_GPIO39_SEL_MASK)
                | ((value << EtmTaskP9Cfg::ETM_TASK_GPIO39_SEL_OFFSET)
                    & EtmTaskP9Cfg::ETM_TASK_GPIO39_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO39_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio39_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP9Cfg::ETM_TASK_GPIO39_EN_MASK
            } else {
                self.bits & !EtmTaskP9Cfg::ETM_TASK_GPIO39_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO38_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio38_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP9Cfg::ETM_TASK_GPIO38_SEL_MASK)
                | ((value << EtmTaskP9Cfg::ETM_TASK_GPIO38_SEL_OFFSET)
                    & EtmTaskP9Cfg::ETM_TASK_GPIO38_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO38_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio38_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP9Cfg::ETM_TASK_GPIO38_EN_MASK
            } else {
                self.bits & !EtmTaskP9Cfg::ETM_TASK_GPIO38_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO37_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio37_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP9Cfg::ETM_TASK_GPIO37_SEL_MASK)
                | ((value << EtmTaskP9Cfg::ETM_TASK_GPIO37_SEL_OFFSET)
                    & EtmTaskP9Cfg::ETM_TASK_GPIO37_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO37_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio37_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP9Cfg::ETM_TASK_GPIO37_EN_MASK
            } else {
                self.bits & !EtmTaskP9Cfg::ETM_TASK_GPIO37_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO36_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio36_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP9Cfg::ETM_TASK_GPIO36_SEL_MASK)
                | ((value << EtmTaskP9Cfg::ETM_TASK_GPIO36_SEL_OFFSET)
                    & EtmTaskP9Cfg::ETM_TASK_GPIO36_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO36_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio36_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP9Cfg::ETM_TASK_GPIO36_EN_MASK
            } else {
                self.bits & !EtmTaskP9Cfg::ETM_TASK_GPIO36_EN_MASK
            },
        }
    }
}

impl Default for EtmTaskP9CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmTaskP9Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmTaskP9CfgVal {
        EtmTaskP9CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmTaskP9CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmTaskP9CfgVal) -> EtmTaskP9CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_TASK_P10_CFG Register
///
/// Etm Configure Register to decide which GPIO been chosen
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **ETM_TASK_GPIO43_SEL** `[27:25]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO43_EN** `[24]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO42_SEL** `[19:17]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO42_EN** `[16]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO41_SEL** `[11:9]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO41_EN** `[8]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO40_SEL** `[3:1]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO40_EN** `[0]` (RW)
///
///   Enable bit of GPIO response etm task.
#[repr(transparent)]
pub struct EtmTaskP10Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmTaskP10Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmTaskP10Cfg {
    // ======== Field Constants ========

    /// **ETM_TASK_GPIO43_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const ETM_TASK_GPIO43_SEL_OFFSET: u32 = 25;
    pub const ETM_TASK_GPIO43_SEL_MASK: u32 = 0x0e000000;

    /// **ETM_TASK_GPIO43_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const ETM_TASK_GPIO43_EN_OFFSET: u32 = 24;
    pub const ETM_TASK_GPIO43_EN_MASK: u32 = 0x01000000;
    pub const ETM_TASK_GPIO43_EN: u32 = 1 << 24;

    /// **ETM_TASK_GPIO42_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const ETM_TASK_GPIO42_SEL_OFFSET: u32 = 17;
    pub const ETM_TASK_GPIO42_SEL_MASK: u32 = 0x000e0000;

    /// **ETM_TASK_GPIO42_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETM_TASK_GPIO42_EN_OFFSET: u32 = 16;
    pub const ETM_TASK_GPIO42_EN_MASK: u32 = 0x00010000;
    pub const ETM_TASK_GPIO42_EN: u32 = 1 << 16;

    /// **ETM_TASK_GPIO41_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const ETM_TASK_GPIO41_SEL_OFFSET: u32 = 9;
    pub const ETM_TASK_GPIO41_SEL_MASK: u32 = 0x00000e00;

    /// **ETM_TASK_GPIO41_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ETM_TASK_GPIO41_EN_OFFSET: u32 = 8;
    pub const ETM_TASK_GPIO41_EN_MASK: u32 = 0x00000100;
    pub const ETM_TASK_GPIO41_EN: u32 = 1 << 8;

    /// **ETM_TASK_GPIO40_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const ETM_TASK_GPIO40_SEL_OFFSET: u32 = 1;
    pub const ETM_TASK_GPIO40_SEL_MASK: u32 = 0x0000000e;

    /// **ETM_TASK_GPIO40_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_TASK_GPIO40_EN_OFFSET: u32 = 0;
    pub const ETM_TASK_GPIO40_EN_MASK: u32 = 0x00000001;
    pub const ETM_TASK_GPIO40_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ETM_TASK_GPIO43_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio43_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO43_SEL_MASK)
            >> Self::ETM_TASK_GPIO43_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO43_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio43_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO43_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO43_SEL_OFFSET) & Self::ETM_TASK_GPIO43_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO43_EN** `[24]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio43_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO43_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO43_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio43_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO43_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO43_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio43_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO43_EN_MASK);
    }

    /// **ETM_TASK_GPIO42_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio42_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO42_SEL_MASK)
            >> Self::ETM_TASK_GPIO42_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO42_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio42_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO42_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO42_SEL_OFFSET) & Self::ETM_TASK_GPIO42_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO42_EN** `[16]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio42_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO42_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO42_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio42_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO42_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO42_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio42_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO42_EN_MASK);
    }

    /// **ETM_TASK_GPIO41_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio41_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO41_SEL_MASK)
            >> Self::ETM_TASK_GPIO41_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO41_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio41_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO41_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO41_SEL_OFFSET) & Self::ETM_TASK_GPIO41_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO41_EN** `[8]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio41_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO41_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO41_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio41_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO41_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO41_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio41_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO41_EN_MASK);
    }

    /// **ETM_TASK_GPIO40_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio40_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO40_SEL_MASK)
            >> Self::ETM_TASK_GPIO40_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO40_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio40_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO40_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO40_SEL_OFFSET) & Self::ETM_TASK_GPIO40_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO40_EN** `[0]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio40_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO40_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO40_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio40_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO40_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO40_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio40_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO40_EN_MASK);
    }
}

/// ETM_TASK_P10_CFG value type for batch read/modify/write operations
///
/// Use `etmtaskp10cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmtaskp10cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmTaskP10CfgVal {
    bits: u32,
}

impl EtmTaskP10CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_TASK_GPIO43_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio43_sel(&self) -> u32 {
        (self.bits & EtmTaskP10Cfg::ETM_TASK_GPIO43_SEL_MASK)
            >> EtmTaskP10Cfg::ETM_TASK_GPIO43_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO43_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio43_en(&self) -> bool {
        (self.bits & EtmTaskP10Cfg::ETM_TASK_GPIO43_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO42_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio42_sel(&self) -> u32 {
        (self.bits & EtmTaskP10Cfg::ETM_TASK_GPIO42_SEL_MASK)
            >> EtmTaskP10Cfg::ETM_TASK_GPIO42_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO42_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio42_en(&self) -> bool {
        (self.bits & EtmTaskP10Cfg::ETM_TASK_GPIO42_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO41_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio41_sel(&self) -> u32 {
        (self.bits & EtmTaskP10Cfg::ETM_TASK_GPIO41_SEL_MASK)
            >> EtmTaskP10Cfg::ETM_TASK_GPIO41_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO41_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio41_en(&self) -> bool {
        (self.bits & EtmTaskP10Cfg::ETM_TASK_GPIO41_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO40_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio40_sel(&self) -> u32 {
        (self.bits & EtmTaskP10Cfg::ETM_TASK_GPIO40_SEL_MASK)
            >> EtmTaskP10Cfg::ETM_TASK_GPIO40_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO40_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio40_en(&self) -> bool {
        (self.bits & EtmTaskP10Cfg::ETM_TASK_GPIO40_EN_MASK) != 0
    }

    /// Set **ETM_TASK_GPIO43_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio43_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP10Cfg::ETM_TASK_GPIO43_SEL_MASK)
                | ((value << EtmTaskP10Cfg::ETM_TASK_GPIO43_SEL_OFFSET)
                    & EtmTaskP10Cfg::ETM_TASK_GPIO43_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO43_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio43_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP10Cfg::ETM_TASK_GPIO43_EN_MASK
            } else {
                self.bits & !EtmTaskP10Cfg::ETM_TASK_GPIO43_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO42_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio42_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP10Cfg::ETM_TASK_GPIO42_SEL_MASK)
                | ((value << EtmTaskP10Cfg::ETM_TASK_GPIO42_SEL_OFFSET)
                    & EtmTaskP10Cfg::ETM_TASK_GPIO42_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO42_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio42_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP10Cfg::ETM_TASK_GPIO42_EN_MASK
            } else {
                self.bits & !EtmTaskP10Cfg::ETM_TASK_GPIO42_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO41_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio41_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP10Cfg::ETM_TASK_GPIO41_SEL_MASK)
                | ((value << EtmTaskP10Cfg::ETM_TASK_GPIO41_SEL_OFFSET)
                    & EtmTaskP10Cfg::ETM_TASK_GPIO41_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO41_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio41_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP10Cfg::ETM_TASK_GPIO41_EN_MASK
            } else {
                self.bits & !EtmTaskP10Cfg::ETM_TASK_GPIO41_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO40_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio40_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP10Cfg::ETM_TASK_GPIO40_SEL_MASK)
                | ((value << EtmTaskP10Cfg::ETM_TASK_GPIO40_SEL_OFFSET)
                    & EtmTaskP10Cfg::ETM_TASK_GPIO40_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO40_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio40_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP10Cfg::ETM_TASK_GPIO40_EN_MASK
            } else {
                self.bits & !EtmTaskP10Cfg::ETM_TASK_GPIO40_EN_MASK
            },
        }
    }
}

impl Default for EtmTaskP10CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmTaskP10Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmTaskP10CfgVal {
        EtmTaskP10CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmTaskP10CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmTaskP10CfgVal) -> EtmTaskP10CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_TASK_P11_CFG Register
///
/// Etm Configure Register to decide which GPIO been chosen
///
/// Address offset: 0x00cc
///
/// ## Fields
///
/// - **ETM_TASK_GPIO47_SEL** `[27:25]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO47_EN** `[24]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO46_SEL** `[19:17]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO46_EN** `[16]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO45_SEL** `[11:9]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO45_EN** `[8]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO44_SEL** `[3:1]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO44_EN** `[0]` (RW)
///
///   Enable bit of GPIO response etm task.
#[repr(transparent)]
pub struct EtmTaskP11Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmTaskP11Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmTaskP11Cfg {
    // ======== Field Constants ========

    /// **ETM_TASK_GPIO47_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const ETM_TASK_GPIO47_SEL_OFFSET: u32 = 25;
    pub const ETM_TASK_GPIO47_SEL_MASK: u32 = 0x0e000000;

    /// **ETM_TASK_GPIO47_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const ETM_TASK_GPIO47_EN_OFFSET: u32 = 24;
    pub const ETM_TASK_GPIO47_EN_MASK: u32 = 0x01000000;
    pub const ETM_TASK_GPIO47_EN: u32 = 1 << 24;

    /// **ETM_TASK_GPIO46_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const ETM_TASK_GPIO46_SEL_OFFSET: u32 = 17;
    pub const ETM_TASK_GPIO46_SEL_MASK: u32 = 0x000e0000;

    /// **ETM_TASK_GPIO46_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETM_TASK_GPIO46_EN_OFFSET: u32 = 16;
    pub const ETM_TASK_GPIO46_EN_MASK: u32 = 0x00010000;
    pub const ETM_TASK_GPIO46_EN: u32 = 1 << 16;

    /// **ETM_TASK_GPIO45_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const ETM_TASK_GPIO45_SEL_OFFSET: u32 = 9;
    pub const ETM_TASK_GPIO45_SEL_MASK: u32 = 0x00000e00;

    /// **ETM_TASK_GPIO45_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ETM_TASK_GPIO45_EN_OFFSET: u32 = 8;
    pub const ETM_TASK_GPIO45_EN_MASK: u32 = 0x00000100;
    pub const ETM_TASK_GPIO45_EN: u32 = 1 << 8;

    /// **ETM_TASK_GPIO44_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const ETM_TASK_GPIO44_SEL_OFFSET: u32 = 1;
    pub const ETM_TASK_GPIO44_SEL_MASK: u32 = 0x0000000e;

    /// **ETM_TASK_GPIO44_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_TASK_GPIO44_EN_OFFSET: u32 = 0;
    pub const ETM_TASK_GPIO44_EN_MASK: u32 = 0x00000001;
    pub const ETM_TASK_GPIO44_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ETM_TASK_GPIO47_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio47_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO47_SEL_MASK)
            >> Self::ETM_TASK_GPIO47_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO47_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio47_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO47_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO47_SEL_OFFSET) & Self::ETM_TASK_GPIO47_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO47_EN** `[24]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio47_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO47_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO47_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio47_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO47_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO47_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio47_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO47_EN_MASK);
    }

    /// **ETM_TASK_GPIO46_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio46_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO46_SEL_MASK)
            >> Self::ETM_TASK_GPIO46_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO46_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio46_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO46_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO46_SEL_OFFSET) & Self::ETM_TASK_GPIO46_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO46_EN** `[16]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio46_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO46_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO46_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio46_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO46_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO46_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio46_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO46_EN_MASK);
    }

    /// **ETM_TASK_GPIO45_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio45_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO45_SEL_MASK)
            >> Self::ETM_TASK_GPIO45_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO45_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio45_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO45_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO45_SEL_OFFSET) & Self::ETM_TASK_GPIO45_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO45_EN** `[8]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio45_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO45_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO45_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio45_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO45_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO45_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio45_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO45_EN_MASK);
    }

    /// **ETM_TASK_GPIO44_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio44_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO44_SEL_MASK)
            >> Self::ETM_TASK_GPIO44_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO44_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio44_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO44_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO44_SEL_OFFSET) & Self::ETM_TASK_GPIO44_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO44_EN** `[0]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio44_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO44_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO44_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio44_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO44_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO44_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio44_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO44_EN_MASK);
    }
}

/// ETM_TASK_P11_CFG value type for batch read/modify/write operations
///
/// Use `etmtaskp11cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmtaskp11cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmTaskP11CfgVal {
    bits: u32,
}

impl EtmTaskP11CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_TASK_GPIO47_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio47_sel(&self) -> u32 {
        (self.bits & EtmTaskP11Cfg::ETM_TASK_GPIO47_SEL_MASK)
            >> EtmTaskP11Cfg::ETM_TASK_GPIO47_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO47_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio47_en(&self) -> bool {
        (self.bits & EtmTaskP11Cfg::ETM_TASK_GPIO47_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO46_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio46_sel(&self) -> u32 {
        (self.bits & EtmTaskP11Cfg::ETM_TASK_GPIO46_SEL_MASK)
            >> EtmTaskP11Cfg::ETM_TASK_GPIO46_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO46_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio46_en(&self) -> bool {
        (self.bits & EtmTaskP11Cfg::ETM_TASK_GPIO46_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO45_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio45_sel(&self) -> u32 {
        (self.bits & EtmTaskP11Cfg::ETM_TASK_GPIO45_SEL_MASK)
            >> EtmTaskP11Cfg::ETM_TASK_GPIO45_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO45_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio45_en(&self) -> bool {
        (self.bits & EtmTaskP11Cfg::ETM_TASK_GPIO45_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO44_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio44_sel(&self) -> u32 {
        (self.bits & EtmTaskP11Cfg::ETM_TASK_GPIO44_SEL_MASK)
            >> EtmTaskP11Cfg::ETM_TASK_GPIO44_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO44_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio44_en(&self) -> bool {
        (self.bits & EtmTaskP11Cfg::ETM_TASK_GPIO44_EN_MASK) != 0
    }

    /// Set **ETM_TASK_GPIO47_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio47_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP11Cfg::ETM_TASK_GPIO47_SEL_MASK)
                | ((value << EtmTaskP11Cfg::ETM_TASK_GPIO47_SEL_OFFSET)
                    & EtmTaskP11Cfg::ETM_TASK_GPIO47_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO47_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio47_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP11Cfg::ETM_TASK_GPIO47_EN_MASK
            } else {
                self.bits & !EtmTaskP11Cfg::ETM_TASK_GPIO47_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO46_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio46_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP11Cfg::ETM_TASK_GPIO46_SEL_MASK)
                | ((value << EtmTaskP11Cfg::ETM_TASK_GPIO46_SEL_OFFSET)
                    & EtmTaskP11Cfg::ETM_TASK_GPIO46_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO46_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio46_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP11Cfg::ETM_TASK_GPIO46_EN_MASK
            } else {
                self.bits & !EtmTaskP11Cfg::ETM_TASK_GPIO46_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO45_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio45_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP11Cfg::ETM_TASK_GPIO45_SEL_MASK)
                | ((value << EtmTaskP11Cfg::ETM_TASK_GPIO45_SEL_OFFSET)
                    & EtmTaskP11Cfg::ETM_TASK_GPIO45_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO45_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio45_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP11Cfg::ETM_TASK_GPIO45_EN_MASK
            } else {
                self.bits & !EtmTaskP11Cfg::ETM_TASK_GPIO45_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO44_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio44_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP11Cfg::ETM_TASK_GPIO44_SEL_MASK)
                | ((value << EtmTaskP11Cfg::ETM_TASK_GPIO44_SEL_OFFSET)
                    & EtmTaskP11Cfg::ETM_TASK_GPIO44_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO44_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio44_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP11Cfg::ETM_TASK_GPIO44_EN_MASK
            } else {
                self.bits & !EtmTaskP11Cfg::ETM_TASK_GPIO44_EN_MASK
            },
        }
    }
}

impl Default for EtmTaskP11CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmTaskP11Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmTaskP11CfgVal {
        EtmTaskP11CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmTaskP11CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmTaskP11CfgVal) -> EtmTaskP11CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_TASK_P12_CFG Register
///
/// Etm Configure Register to decide which GPIO been chosen
///
/// Address offset: 0x00d0
///
/// ## Fields
///
/// - **ETM_TASK_GPIO51_SEL** `[27:25]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO51_EN** `[24]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO50_SEL** `[19:17]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO50_EN** `[16]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO49_SEL** `[11:9]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO49_EN** `[8]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO48_SEL** `[3:1]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO48_EN** `[0]` (RW)
///
///   Enable bit of GPIO response etm task.
#[repr(transparent)]
pub struct EtmTaskP12Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmTaskP12Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmTaskP12Cfg {
    // ======== Field Constants ========

    /// **ETM_TASK_GPIO51_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const ETM_TASK_GPIO51_SEL_OFFSET: u32 = 25;
    pub const ETM_TASK_GPIO51_SEL_MASK: u32 = 0x0e000000;

    /// **ETM_TASK_GPIO51_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const ETM_TASK_GPIO51_EN_OFFSET: u32 = 24;
    pub const ETM_TASK_GPIO51_EN_MASK: u32 = 0x01000000;
    pub const ETM_TASK_GPIO51_EN: u32 = 1 << 24;

    /// **ETM_TASK_GPIO50_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const ETM_TASK_GPIO50_SEL_OFFSET: u32 = 17;
    pub const ETM_TASK_GPIO50_SEL_MASK: u32 = 0x000e0000;

    /// **ETM_TASK_GPIO50_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETM_TASK_GPIO50_EN_OFFSET: u32 = 16;
    pub const ETM_TASK_GPIO50_EN_MASK: u32 = 0x00010000;
    pub const ETM_TASK_GPIO50_EN: u32 = 1 << 16;

    /// **ETM_TASK_GPIO49_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const ETM_TASK_GPIO49_SEL_OFFSET: u32 = 9;
    pub const ETM_TASK_GPIO49_SEL_MASK: u32 = 0x00000e00;

    /// **ETM_TASK_GPIO49_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ETM_TASK_GPIO49_EN_OFFSET: u32 = 8;
    pub const ETM_TASK_GPIO49_EN_MASK: u32 = 0x00000100;
    pub const ETM_TASK_GPIO49_EN: u32 = 1 << 8;

    /// **ETM_TASK_GPIO48_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const ETM_TASK_GPIO48_SEL_OFFSET: u32 = 1;
    pub const ETM_TASK_GPIO48_SEL_MASK: u32 = 0x0000000e;

    /// **ETM_TASK_GPIO48_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_TASK_GPIO48_EN_OFFSET: u32 = 0;
    pub const ETM_TASK_GPIO48_EN_MASK: u32 = 0x00000001;
    pub const ETM_TASK_GPIO48_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ETM_TASK_GPIO51_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio51_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO51_SEL_MASK)
            >> Self::ETM_TASK_GPIO51_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO51_SEL** `[27:25]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio51_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO51_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO51_SEL_OFFSET) & Self::ETM_TASK_GPIO51_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO51_EN** `[24]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio51_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO51_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO51_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio51_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO51_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO51_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio51_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO51_EN_MASK);
    }

    /// **ETM_TASK_GPIO50_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio50_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO50_SEL_MASK)
            >> Self::ETM_TASK_GPIO50_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO50_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio50_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO50_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO50_SEL_OFFSET) & Self::ETM_TASK_GPIO50_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO50_EN** `[16]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio50_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO50_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO50_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio50_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO50_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO50_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio50_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO50_EN_MASK);
    }

    /// **ETM_TASK_GPIO49_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio49_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO49_SEL_MASK)
            >> Self::ETM_TASK_GPIO49_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO49_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio49_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO49_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO49_SEL_OFFSET) & Self::ETM_TASK_GPIO49_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO49_EN** `[8]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio49_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO49_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO49_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio49_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO49_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO49_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio49_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO49_EN_MASK);
    }

    /// **ETM_TASK_GPIO48_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio48_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO48_SEL_MASK)
            >> Self::ETM_TASK_GPIO48_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO48_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio48_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO48_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO48_SEL_OFFSET) & Self::ETM_TASK_GPIO48_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO48_EN** `[0]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio48_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO48_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO48_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio48_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO48_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO48_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio48_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO48_EN_MASK);
    }
}

/// ETM_TASK_P12_CFG value type for batch read/modify/write operations
///
/// Use `etmtaskp12cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmtaskp12cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmTaskP12CfgVal {
    bits: u32,
}

impl EtmTaskP12CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_TASK_GPIO51_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio51_sel(&self) -> u32 {
        (self.bits & EtmTaskP12Cfg::ETM_TASK_GPIO51_SEL_MASK)
            >> EtmTaskP12Cfg::ETM_TASK_GPIO51_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO51_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio51_en(&self) -> bool {
        (self.bits & EtmTaskP12Cfg::ETM_TASK_GPIO51_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO50_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio50_sel(&self) -> u32 {
        (self.bits & EtmTaskP12Cfg::ETM_TASK_GPIO50_SEL_MASK)
            >> EtmTaskP12Cfg::ETM_TASK_GPIO50_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO50_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio50_en(&self) -> bool {
        (self.bits & EtmTaskP12Cfg::ETM_TASK_GPIO50_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO49_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio49_sel(&self) -> u32 {
        (self.bits & EtmTaskP12Cfg::ETM_TASK_GPIO49_SEL_MASK)
            >> EtmTaskP12Cfg::ETM_TASK_GPIO49_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO49_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio49_en(&self) -> bool {
        (self.bits & EtmTaskP12Cfg::ETM_TASK_GPIO49_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO48_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio48_sel(&self) -> u32 {
        (self.bits & EtmTaskP12Cfg::ETM_TASK_GPIO48_SEL_MASK)
            >> EtmTaskP12Cfg::ETM_TASK_GPIO48_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO48_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio48_en(&self) -> bool {
        (self.bits & EtmTaskP12Cfg::ETM_TASK_GPIO48_EN_MASK) != 0
    }

    /// Set **ETM_TASK_GPIO51_SEL** `[27:25]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio51_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP12Cfg::ETM_TASK_GPIO51_SEL_MASK)
                | ((value << EtmTaskP12Cfg::ETM_TASK_GPIO51_SEL_OFFSET)
                    & EtmTaskP12Cfg::ETM_TASK_GPIO51_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO51_EN** `[24:24]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio51_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP12Cfg::ETM_TASK_GPIO51_EN_MASK
            } else {
                self.bits & !EtmTaskP12Cfg::ETM_TASK_GPIO51_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO50_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio50_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP12Cfg::ETM_TASK_GPIO50_SEL_MASK)
                | ((value << EtmTaskP12Cfg::ETM_TASK_GPIO50_SEL_OFFSET)
                    & EtmTaskP12Cfg::ETM_TASK_GPIO50_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO50_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio50_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP12Cfg::ETM_TASK_GPIO50_EN_MASK
            } else {
                self.bits & !EtmTaskP12Cfg::ETM_TASK_GPIO50_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO49_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio49_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP12Cfg::ETM_TASK_GPIO49_SEL_MASK)
                | ((value << EtmTaskP12Cfg::ETM_TASK_GPIO49_SEL_OFFSET)
                    & EtmTaskP12Cfg::ETM_TASK_GPIO49_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO49_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio49_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP12Cfg::ETM_TASK_GPIO49_EN_MASK
            } else {
                self.bits & !EtmTaskP12Cfg::ETM_TASK_GPIO49_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO48_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio48_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP12Cfg::ETM_TASK_GPIO48_SEL_MASK)
                | ((value << EtmTaskP12Cfg::ETM_TASK_GPIO48_SEL_OFFSET)
                    & EtmTaskP12Cfg::ETM_TASK_GPIO48_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO48_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio48_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP12Cfg::ETM_TASK_GPIO48_EN_MASK
            } else {
                self.bits & !EtmTaskP12Cfg::ETM_TASK_GPIO48_EN_MASK
            },
        }
    }
}

impl Default for EtmTaskP12CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmTaskP12Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmTaskP12CfgVal {
        EtmTaskP12CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmTaskP12CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmTaskP12CfgVal) -> EtmTaskP12CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ETM_TASK_P13_CFG Register
///
/// Etm Configure Register to decide which GPIO been chosen
///
/// Address offset: 0x00d4
///
/// ## Fields
///
/// - **ETM_TASK_GPIO54_SEL** `[19:17]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO54_EN** `[16]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO53_SEL** `[11:9]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO53_EN** `[8]` (RW)
///
///   Enable bit of GPIO response etm task.
///
/// - **ETM_TASK_GPIO52_SEL** `[3:1]` (RW)
///
///   GPIO choose a etm task channel.
///
/// - **ETM_TASK_GPIO52_EN** `[0]` (RW)
///
///   Enable bit of GPIO response etm task.
#[repr(transparent)]
pub struct EtmTaskP13Cfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EtmTaskP13Cfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EtmTaskP13Cfg {
    // ======== Field Constants ========

    /// **ETM_TASK_GPIO54_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const ETM_TASK_GPIO54_SEL_OFFSET: u32 = 17;
    pub const ETM_TASK_GPIO54_SEL_MASK: u32 = 0x000e0000;

    /// **ETM_TASK_GPIO54_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETM_TASK_GPIO54_EN_OFFSET: u32 = 16;
    pub const ETM_TASK_GPIO54_EN_MASK: u32 = 0x00010000;
    pub const ETM_TASK_GPIO54_EN: u32 = 1 << 16;

    /// **ETM_TASK_GPIO53_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const ETM_TASK_GPIO53_SEL_OFFSET: u32 = 9;
    pub const ETM_TASK_GPIO53_SEL_MASK: u32 = 0x00000e00;

    /// **ETM_TASK_GPIO53_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const ETM_TASK_GPIO53_EN_OFFSET: u32 = 8;
    pub const ETM_TASK_GPIO53_EN_MASK: u32 = 0x00000100;
    pub const ETM_TASK_GPIO53_EN: u32 = 1 << 8;

    /// **ETM_TASK_GPIO52_SEL**
    ///
    /// GPIO choose a etm task channel.
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const ETM_TASK_GPIO52_SEL_OFFSET: u32 = 1;
    pub const ETM_TASK_GPIO52_SEL_MASK: u32 = 0x0000000e;

    /// **ETM_TASK_GPIO52_EN**
    ///
    /// Enable bit of GPIO response etm task.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ETM_TASK_GPIO52_EN_OFFSET: u32 = 0;
    pub const ETM_TASK_GPIO52_EN_MASK: u32 = 0x00000001;
    pub const ETM_TASK_GPIO52_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ETM_TASK_GPIO54_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio54_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO54_SEL_MASK)
            >> Self::ETM_TASK_GPIO54_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO54_SEL** `[19:17]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio54_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO54_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO54_SEL_OFFSET) & Self::ETM_TASK_GPIO54_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO54_EN** `[16]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio54_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO54_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO54_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio54_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO54_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO54_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio54_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO54_EN_MASK);
    }

    /// **ETM_TASK_GPIO53_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio53_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO53_SEL_MASK)
            >> Self::ETM_TASK_GPIO53_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO53_SEL** `[11:9]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio53_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO53_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO53_SEL_OFFSET) & Self::ETM_TASK_GPIO53_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO53_EN** `[8]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio53_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO53_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO53_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio53_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO53_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO53_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio53_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO53_EN_MASK);
    }

    /// **ETM_TASK_GPIO52_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn etm_task_gpio52_sel(&self) -> u32 {
        (self.value.read_bits() & Self::ETM_TASK_GPIO52_SEL_MASK)
            >> Self::ETM_TASK_GPIO52_SEL_OFFSET
    }

    /// Write **ETM_TASK_GPIO52_SEL** `[3:1]` (RW)
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub fn write_etm_task_gpio52_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ETM_TASK_GPIO52_SEL_MASK)
                | ((value << Self::ETM_TASK_GPIO52_SEL_OFFSET) & Self::ETM_TASK_GPIO52_SEL_MASK)
        });
    }

    /// **ETM_TASK_GPIO52_EN** `[0]` (RW)
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn etm_task_gpio52_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_GPIO52_EN_MASK) != 0
    }

    /// Set ETM_TASK_GPIO52_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_gpio52_en(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_GPIO52_EN_MASK);
    }

    /// Clear ETM_TASK_GPIO52_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_gpio52_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_GPIO52_EN_MASK);
    }
}

/// ETM_TASK_P13_CFG value type for batch read/modify/write operations
///
/// Use `etmtaskp13cfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `etmtaskp13cfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EtmTaskP13CfgVal {
    bits: u32,
}

impl EtmTaskP13CfgVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ETM_TASK_GPIO54_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio54_sel(&self) -> u32 {
        (self.bits & EtmTaskP13Cfg::ETM_TASK_GPIO54_SEL_MASK)
            >> EtmTaskP13Cfg::ETM_TASK_GPIO54_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO54_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio54_en(&self) -> bool {
        (self.bits & EtmTaskP13Cfg::ETM_TASK_GPIO54_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO53_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio53_sel(&self) -> u32 {
        (self.bits & EtmTaskP13Cfg::ETM_TASK_GPIO53_SEL_MASK)
            >> EtmTaskP13Cfg::ETM_TASK_GPIO53_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO53_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio53_en(&self) -> bool {
        (self.bits & EtmTaskP13Cfg::ETM_TASK_GPIO53_EN_MASK) != 0
    }

    /// Read **ETM_TASK_GPIO52_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn etm_task_gpio52_sel(&self) -> u32 {
        (self.bits & EtmTaskP13Cfg::ETM_TASK_GPIO52_SEL_MASK)
            >> EtmTaskP13Cfg::ETM_TASK_GPIO52_SEL_OFFSET
    }

    /// Read **ETM_TASK_GPIO52_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn etm_task_gpio52_en(&self) -> bool {
        (self.bits & EtmTaskP13Cfg::ETM_TASK_GPIO52_EN_MASK) != 0
    }

    /// Set **ETM_TASK_GPIO54_SEL** `[19:17]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio54_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP13Cfg::ETM_TASK_GPIO54_SEL_MASK)
                | ((value << EtmTaskP13Cfg::ETM_TASK_GPIO54_SEL_OFFSET)
                    & EtmTaskP13Cfg::ETM_TASK_GPIO54_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO54_EN** `[16:16]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio54_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP13Cfg::ETM_TASK_GPIO54_EN_MASK
            } else {
                self.bits & !EtmTaskP13Cfg::ETM_TASK_GPIO54_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO53_SEL** `[11:9]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio53_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP13Cfg::ETM_TASK_GPIO53_SEL_MASK)
                | ((value << EtmTaskP13Cfg::ETM_TASK_GPIO53_SEL_OFFSET)
                    & EtmTaskP13Cfg::ETM_TASK_GPIO53_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO53_EN** `[8:8]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio53_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP13Cfg::ETM_TASK_GPIO53_EN_MASK
            } else {
                self.bits & !EtmTaskP13Cfg::ETM_TASK_GPIO53_EN_MASK
            },
        }
    }

    /// Set **ETM_TASK_GPIO52_SEL** `[3:1]`
    ///
    /// GPIO choose a etm task channel.
    #[inline(always)]
    pub const fn with_etm_task_gpio52_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !EtmTaskP13Cfg::ETM_TASK_GPIO52_SEL_MASK)
                | ((value << EtmTaskP13Cfg::ETM_TASK_GPIO52_SEL_OFFSET)
                    & EtmTaskP13Cfg::ETM_TASK_GPIO52_SEL_MASK),
        }
    }

    /// Set **ETM_TASK_GPIO52_EN** `[0:0]`
    ///
    /// Enable bit of GPIO response etm task.
    #[inline(always)]
    pub const fn with_etm_task_gpio52_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EtmTaskP13Cfg::ETM_TASK_GPIO52_EN_MASK
            } else {
                self.bits & !EtmTaskP13Cfg::ETM_TASK_GPIO52_EN_MASK
            },
        }
    }
}

impl Default for EtmTaskP13CfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl EtmTaskP13Cfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EtmTaskP13CfgVal {
        EtmTaskP13CfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EtmTaskP13CfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EtmTaskP13CfgVal) -> EtmTaskP13CfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VERSION Register
///
/// Version Control Register
///
/// Address offset: 0x00fc
/// Reset value: 0x02203050
///
/// ## Fields
///
/// - **GPIO_SD_DATE** `[27:0]` (RW)
///
///   Version control register.
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

    /// **GPIO_SD_DATE**
    ///
    /// Version control register.
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const GPIO_SD_DATE_OFFSET: u32 = 0;
    pub const GPIO_SD_DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **GPIO_SD_DATE** `[27:0]` (RW)
    ///
    /// Version control register.
    #[inline(always)]
    pub fn gpio_sd_date(&self) -> u32 {
        (self.value.read_bits() & Self::GPIO_SD_DATE_MASK) >> Self::GPIO_SD_DATE_OFFSET
    }

    /// Write **GPIO_SD_DATE** `[27:0]` (RW)
    ///
    /// Version control register.
    #[inline(always)]
    pub fn write_gpio_sd_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::GPIO_SD_DATE_MASK)
                | ((value << Self::GPIO_SD_DATE_OFFSET) & Self::GPIO_SD_DATE_MASK)
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
        Self { bits: 0x02203050 }
    }

    /// Read **GPIO_SD_DATE** `[27:0]`
    ///
    /// Version control register.
    #[inline(always)]
    pub const fn gpio_sd_date(&self) -> u32 {
        (self.bits & Version::GPIO_SD_DATE_MASK) >> Version::GPIO_SD_DATE_OFFSET
    }

    /// Set **GPIO_SD_DATE** `[27:0]`
    ///
    /// Version control register.
    #[inline(always)]
    pub const fn with_gpio_sd_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Version::GPIO_SD_DATE_MASK)
                | ((value << Version::GPIO_SD_DATE_OFFSET) & Version::GPIO_SD_DATE_MASK),
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
    /// Duty Cycle Configure Register of SDM0
    pub sigmadelta0: Sigmadelta0,
    /// 0x0004
    /// Duty Cycle Configure Register of SDM1
    pub sigmadelta1: Sigmadelta1,
    /// 0x0008
    /// Duty Cycle Configure Register of SDM2
    pub sigmadelta2: Sigmadelta2,
    /// 0x000c
    /// Duty Cycle Configure Register of SDM3
    pub sigmadelta3: Sigmadelta3,
    /// 0x0010
    /// Duty Cycle Configure Register of SDM4
    pub sigmadelta4: Sigmadelta4,
    /// 0x0014
    /// Duty Cycle Configure Register of SDM5
    pub sigmadelta5: Sigmadelta5,
    /// 0x0018
    /// Duty Cycle Configure Register of SDM6
    pub sigmadelta6: Sigmadelta6,
    /// 0x001c
    /// Duty Cycle Configure Register of SDM7
    pub sigmadelta7: Sigmadelta7,
    /// 0x0020
    /// Clock Gating Configure Register
    pub clock_gate: ClockGate,
    /// 0x0024
    /// MISC Register
    pub sigmadelta_misc: SigmadeltaMisc,
    _reserved0: [u8; 8],
    /// 0x0030
    /// Glitch Filter Configure Register of Channel0
    pub glitch_filter_ch0: GlitchFilterCh0,
    /// 0x0034
    /// Glitch Filter Configure Register of Channel1
    pub glitch_filter_ch1: GlitchFilterCh1,
    /// 0x0038
    /// Glitch Filter Configure Register of Channel2
    pub glitch_filter_ch2: GlitchFilterCh2,
    /// 0x003c
    /// Glitch Filter Configure Register of Channel3
    pub glitch_filter_ch3: GlitchFilterCh3,
    /// 0x0040
    /// Glitch Filter Configure Register of Channel4
    pub glitch_filter_ch4: GlitchFilterCh4,
    /// 0x0044
    /// Glitch Filter Configure Register of Channel5
    pub glitch_filter_ch5: GlitchFilterCh5,
    /// 0x0048
    /// Glitch Filter Configure Register of Channel6
    pub glitch_filter_ch6: GlitchFilterCh6,
    /// 0x004c
    /// Glitch Filter Configure Register of Channel7
    pub glitch_filter_ch7: GlitchFilterCh7,
    _reserved1: [u8; 16],
    /// 0x0060
    /// Etm Config register of Channel0
    pub etm_event_ch0_cfg: EtmEventCh0Cfg,
    /// 0x0064
    /// Etm Config register of Channel1
    pub etm_event_ch1_cfg: EtmEventCh1Cfg,
    /// 0x0068
    /// Etm Config register of Channel2
    pub etm_event_ch2_cfg: EtmEventCh2Cfg,
    /// 0x006c
    /// Etm Config register of Channel3
    pub etm_event_ch3_cfg: EtmEventCh3Cfg,
    /// 0x0070
    /// Etm Config register of Channel4
    pub etm_event_ch4_cfg: EtmEventCh4Cfg,
    /// 0x0074
    /// Etm Config register of Channel5
    pub etm_event_ch5_cfg: EtmEventCh5Cfg,
    /// 0x0078
    /// Etm Config register of Channel6
    pub etm_event_ch6_cfg: EtmEventCh6Cfg,
    /// 0x007c
    /// Etm Config register of Channel7
    pub etm_event_ch7_cfg: EtmEventCh7Cfg,
    _reserved2: [u8; 32],
    /// 0x00a0
    /// Etm Configure Register to decide which GPIO been chosen
    pub etm_task_p0_cfg: EtmTaskP0Cfg,
    /// 0x00a4
    /// Etm Configure Register to decide which GPIO been chosen
    pub etm_task_p1_cfg: EtmTaskP1Cfg,
    /// 0x00a8
    /// Etm Configure Register to decide which GPIO been chosen
    pub etm_task_p2_cfg: EtmTaskP2Cfg,
    /// 0x00ac
    /// Etm Configure Register to decide which GPIO been chosen
    pub etm_task_p3_cfg: EtmTaskP3Cfg,
    /// 0x00b0
    /// Etm Configure Register to decide which GPIO been chosen
    pub etm_task_p4_cfg: EtmTaskP4Cfg,
    /// 0x00b4
    /// Etm Configure Register to decide which GPIO been chosen
    pub etm_task_p5_cfg: EtmTaskP5Cfg,
    /// 0x00b8
    /// Etm Configure Register to decide which GPIO been chosen
    pub etm_task_p6_cfg: EtmTaskP6Cfg,
    /// 0x00bc
    /// Etm Configure Register to decide which GPIO been chosen
    pub etm_task_p7_cfg: EtmTaskP7Cfg,
    /// 0x00c0
    /// Etm Configure Register to decide which GPIO been chosen
    pub etm_task_p8_cfg: EtmTaskP8Cfg,
    /// 0x00c4
    /// Etm Configure Register to decide which GPIO been chosen
    pub etm_task_p9_cfg: EtmTaskP9Cfg,
    /// 0x00c8
    /// Etm Configure Register to decide which GPIO been chosen
    pub etm_task_p10_cfg: EtmTaskP10Cfg,
    /// 0x00cc
    /// Etm Configure Register to decide which GPIO been chosen
    pub etm_task_p11_cfg: EtmTaskP11Cfg,
    /// 0x00d0
    /// Etm Configure Register to decide which GPIO been chosen
    pub etm_task_p12_cfg: EtmTaskP12Cfg,
    /// 0x00d4
    /// Etm Configure Register to decide which GPIO been chosen
    pub etm_task_p13_cfg: EtmTaskP13Cfg,
    _reserved3: [u8; 36],
    /// 0x00fc
    /// Version Control Register
    pub version: Version,
}
