//! EFUSE
//! eFuse Controller

#![allow(dead_code)]

pub const BASE: u32 = 0x5012d000;

/// PGM_DATA0 Register
///
/// Register 0 that stores data to be programmed.
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **PGM_DATA_0** `[31:0]` (RW)
///
///   Configures the 0th 32-bit data to be programmed.
#[repr(transparent)]
pub struct PgmData0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PgmData0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PgmData0 {
    // ======== Field Constants ========

    /// **PGM_DATA_0**
    ///
    /// Configures the 0th 32-bit data to be programmed.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PGM_DATA_0_OFFSET: u32 = 0;
    pub const PGM_DATA_0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PGM_DATA_0** `[31:0]` (RW)
    ///
    /// Configures the 0th 32-bit data to be programmed.
    #[inline(always)]
    pub fn pgm_data_0(&self) -> u32 {
        (self.value.read_bits() & Self::PGM_DATA_0_MASK) >> Self::PGM_DATA_0_OFFSET
    }

    /// Write **PGM_DATA_0** `[31:0]` (RW)
    ///
    /// Configures the 0th 32-bit data to be programmed.
    #[inline(always)]
    pub fn write_pgm_data_0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PGM_DATA_0_MASK)
                | ((value << Self::PGM_DATA_0_OFFSET) & Self::PGM_DATA_0_MASK)
        });
    }
}

/// PGM_DATA0 value type for batch read/modify/write operations
///
/// Use `pgmdata0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pgmdata0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PgmData0Val {
    bits: u32,
}

impl PgmData0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PGM_DATA_0** `[31:0]`
    ///
    /// Configures the 0th 32-bit data to be programmed.
    #[inline(always)]
    pub const fn pgm_data_0(&self) -> u32 {
        (self.bits & PgmData0::PGM_DATA_0_MASK) >> PgmData0::PGM_DATA_0_OFFSET
    }

    /// Set **PGM_DATA_0** `[31:0]`
    ///
    /// Configures the 0th 32-bit data to be programmed.
    #[inline(always)]
    pub const fn with_pgm_data_0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PgmData0::PGM_DATA_0_MASK)
                | ((value << PgmData0::PGM_DATA_0_OFFSET) & PgmData0::PGM_DATA_0_MASK),
        }
    }
}

impl Default for PgmData0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PgmData0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PgmData0Val {
        PgmData0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PgmData0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PgmData0Val) -> PgmData0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PGM_DATA1 Register
///
/// Register 1 that stores data to be programmed.
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **PGM_DATA_1** `[31:0]` (RW)
///
///   Configures the 1st 32-bit data to be programmed.
#[repr(transparent)]
pub struct PgmData1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PgmData1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PgmData1 {
    // ======== Field Constants ========

    /// **PGM_DATA_1**
    ///
    /// Configures the 1st 32-bit data to be programmed.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PGM_DATA_1_OFFSET: u32 = 0;
    pub const PGM_DATA_1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PGM_DATA_1** `[31:0]` (RW)
    ///
    /// Configures the 1st 32-bit data to be programmed.
    #[inline(always)]
    pub fn pgm_data_1(&self) -> u32 {
        (self.value.read_bits() & Self::PGM_DATA_1_MASK) >> Self::PGM_DATA_1_OFFSET
    }

    /// Write **PGM_DATA_1** `[31:0]` (RW)
    ///
    /// Configures the 1st 32-bit data to be programmed.
    #[inline(always)]
    pub fn write_pgm_data_1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PGM_DATA_1_MASK)
                | ((value << Self::PGM_DATA_1_OFFSET) & Self::PGM_DATA_1_MASK)
        });
    }
}

/// PGM_DATA1 value type for batch read/modify/write operations
///
/// Use `pgmdata1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pgmdata1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PgmData1Val {
    bits: u32,
}

impl PgmData1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PGM_DATA_1** `[31:0]`
    ///
    /// Configures the 1st 32-bit data to be programmed.
    #[inline(always)]
    pub const fn pgm_data_1(&self) -> u32 {
        (self.bits & PgmData1::PGM_DATA_1_MASK) >> PgmData1::PGM_DATA_1_OFFSET
    }

    /// Set **PGM_DATA_1** `[31:0]`
    ///
    /// Configures the 1st 32-bit data to be programmed.
    #[inline(always)]
    pub const fn with_pgm_data_1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PgmData1::PGM_DATA_1_MASK)
                | ((value << PgmData1::PGM_DATA_1_OFFSET) & PgmData1::PGM_DATA_1_MASK),
        }
    }
}

impl Default for PgmData1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PgmData1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PgmData1Val {
        PgmData1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PgmData1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PgmData1Val) -> PgmData1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PGM_DATA2 Register
///
/// Register 2 that stores data to be programmed.
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **PGM_DATA_2** `[31:0]` (RW)
///
///   Configures the 2nd 32-bit data to be programmed.
#[repr(transparent)]
pub struct PgmData2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PgmData2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PgmData2 {
    // ======== Field Constants ========

    /// **PGM_DATA_2**
    ///
    /// Configures the 2nd 32-bit data to be programmed.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PGM_DATA_2_OFFSET: u32 = 0;
    pub const PGM_DATA_2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PGM_DATA_2** `[31:0]` (RW)
    ///
    /// Configures the 2nd 32-bit data to be programmed.
    #[inline(always)]
    pub fn pgm_data_2(&self) -> u32 {
        (self.value.read_bits() & Self::PGM_DATA_2_MASK) >> Self::PGM_DATA_2_OFFSET
    }

    /// Write **PGM_DATA_2** `[31:0]` (RW)
    ///
    /// Configures the 2nd 32-bit data to be programmed.
    #[inline(always)]
    pub fn write_pgm_data_2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PGM_DATA_2_MASK)
                | ((value << Self::PGM_DATA_2_OFFSET) & Self::PGM_DATA_2_MASK)
        });
    }
}

/// PGM_DATA2 value type for batch read/modify/write operations
///
/// Use `pgmdata2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pgmdata2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PgmData2Val {
    bits: u32,
}

impl PgmData2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PGM_DATA_2** `[31:0]`
    ///
    /// Configures the 2nd 32-bit data to be programmed.
    #[inline(always)]
    pub const fn pgm_data_2(&self) -> u32 {
        (self.bits & PgmData2::PGM_DATA_2_MASK) >> PgmData2::PGM_DATA_2_OFFSET
    }

    /// Set **PGM_DATA_2** `[31:0]`
    ///
    /// Configures the 2nd 32-bit data to be programmed.
    #[inline(always)]
    pub const fn with_pgm_data_2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PgmData2::PGM_DATA_2_MASK)
                | ((value << PgmData2::PGM_DATA_2_OFFSET) & PgmData2::PGM_DATA_2_MASK),
        }
    }
}

impl Default for PgmData2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PgmData2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PgmData2Val {
        PgmData2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PgmData2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PgmData2Val) -> PgmData2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PGM_DATA3 Register
///
/// Register 3 that stores data to be programmed.
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **PGM_DATA_3** `[31:0]` (RW)
///
///   Configures the 3rd 32-bit data to be programmed.
#[repr(transparent)]
pub struct PgmData3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PgmData3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PgmData3 {
    // ======== Field Constants ========

    /// **PGM_DATA_3**
    ///
    /// Configures the 3rd 32-bit data to be programmed.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PGM_DATA_3_OFFSET: u32 = 0;
    pub const PGM_DATA_3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PGM_DATA_3** `[31:0]` (RW)
    ///
    /// Configures the 3rd 32-bit data to be programmed.
    #[inline(always)]
    pub fn pgm_data_3(&self) -> u32 {
        (self.value.read_bits() & Self::PGM_DATA_3_MASK) >> Self::PGM_DATA_3_OFFSET
    }

    /// Write **PGM_DATA_3** `[31:0]` (RW)
    ///
    /// Configures the 3rd 32-bit data to be programmed.
    #[inline(always)]
    pub fn write_pgm_data_3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PGM_DATA_3_MASK)
                | ((value << Self::PGM_DATA_3_OFFSET) & Self::PGM_DATA_3_MASK)
        });
    }
}

/// PGM_DATA3 value type for batch read/modify/write operations
///
/// Use `pgmdata3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pgmdata3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PgmData3Val {
    bits: u32,
}

impl PgmData3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PGM_DATA_3** `[31:0]`
    ///
    /// Configures the 3rd 32-bit data to be programmed.
    #[inline(always)]
    pub const fn pgm_data_3(&self) -> u32 {
        (self.bits & PgmData3::PGM_DATA_3_MASK) >> PgmData3::PGM_DATA_3_OFFSET
    }

    /// Set **PGM_DATA_3** `[31:0]`
    ///
    /// Configures the 3rd 32-bit data to be programmed.
    #[inline(always)]
    pub const fn with_pgm_data_3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PgmData3::PGM_DATA_3_MASK)
                | ((value << PgmData3::PGM_DATA_3_OFFSET) & PgmData3::PGM_DATA_3_MASK),
        }
    }
}

impl Default for PgmData3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PgmData3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PgmData3Val {
        PgmData3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PgmData3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PgmData3Val) -> PgmData3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PGM_DATA4 Register
///
/// Register 4 that stores data to be programmed.
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **PGM_DATA_4** `[31:0]` (RW)
///
///   Configures the 4th 32-bit data to be programmed.
#[repr(transparent)]
pub struct PgmData4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PgmData4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PgmData4 {
    // ======== Field Constants ========

    /// **PGM_DATA_4**
    ///
    /// Configures the 4th 32-bit data to be programmed.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PGM_DATA_4_OFFSET: u32 = 0;
    pub const PGM_DATA_4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PGM_DATA_4** `[31:0]` (RW)
    ///
    /// Configures the 4th 32-bit data to be programmed.
    #[inline(always)]
    pub fn pgm_data_4(&self) -> u32 {
        (self.value.read_bits() & Self::PGM_DATA_4_MASK) >> Self::PGM_DATA_4_OFFSET
    }

    /// Write **PGM_DATA_4** `[31:0]` (RW)
    ///
    /// Configures the 4th 32-bit data to be programmed.
    #[inline(always)]
    pub fn write_pgm_data_4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PGM_DATA_4_MASK)
                | ((value << Self::PGM_DATA_4_OFFSET) & Self::PGM_DATA_4_MASK)
        });
    }
}

/// PGM_DATA4 value type for batch read/modify/write operations
///
/// Use `pgmdata4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pgmdata4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PgmData4Val {
    bits: u32,
}

impl PgmData4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PGM_DATA_4** `[31:0]`
    ///
    /// Configures the 4th 32-bit data to be programmed.
    #[inline(always)]
    pub const fn pgm_data_4(&self) -> u32 {
        (self.bits & PgmData4::PGM_DATA_4_MASK) >> PgmData4::PGM_DATA_4_OFFSET
    }

    /// Set **PGM_DATA_4** `[31:0]`
    ///
    /// Configures the 4th 32-bit data to be programmed.
    #[inline(always)]
    pub const fn with_pgm_data_4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PgmData4::PGM_DATA_4_MASK)
                | ((value << PgmData4::PGM_DATA_4_OFFSET) & PgmData4::PGM_DATA_4_MASK),
        }
    }
}

impl Default for PgmData4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PgmData4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PgmData4Val {
        PgmData4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PgmData4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PgmData4Val) -> PgmData4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PGM_DATA5 Register
///
/// Register 5 that stores data to be programmed.
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **PGM_DATA_5** `[31:0]` (RW)
///
///   Configures the 5th 32-bit data to be programmed.
#[repr(transparent)]
pub struct PgmData5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PgmData5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PgmData5 {
    // ======== Field Constants ========

    /// **PGM_DATA_5**
    ///
    /// Configures the 5th 32-bit data to be programmed.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PGM_DATA_5_OFFSET: u32 = 0;
    pub const PGM_DATA_5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PGM_DATA_5** `[31:0]` (RW)
    ///
    /// Configures the 5th 32-bit data to be programmed.
    #[inline(always)]
    pub fn pgm_data_5(&self) -> u32 {
        (self.value.read_bits() & Self::PGM_DATA_5_MASK) >> Self::PGM_DATA_5_OFFSET
    }

    /// Write **PGM_DATA_5** `[31:0]` (RW)
    ///
    /// Configures the 5th 32-bit data to be programmed.
    #[inline(always)]
    pub fn write_pgm_data_5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PGM_DATA_5_MASK)
                | ((value << Self::PGM_DATA_5_OFFSET) & Self::PGM_DATA_5_MASK)
        });
    }
}

/// PGM_DATA5 value type for batch read/modify/write operations
///
/// Use `pgmdata5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pgmdata5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PgmData5Val {
    bits: u32,
}

impl PgmData5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PGM_DATA_5** `[31:0]`
    ///
    /// Configures the 5th 32-bit data to be programmed.
    #[inline(always)]
    pub const fn pgm_data_5(&self) -> u32 {
        (self.bits & PgmData5::PGM_DATA_5_MASK) >> PgmData5::PGM_DATA_5_OFFSET
    }

    /// Set **PGM_DATA_5** `[31:0]`
    ///
    /// Configures the 5th 32-bit data to be programmed.
    #[inline(always)]
    pub const fn with_pgm_data_5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PgmData5::PGM_DATA_5_MASK)
                | ((value << PgmData5::PGM_DATA_5_OFFSET) & PgmData5::PGM_DATA_5_MASK),
        }
    }
}

impl Default for PgmData5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PgmData5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PgmData5Val {
        PgmData5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PgmData5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PgmData5Val) -> PgmData5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PGM_DATA6 Register
///
/// Register 6 that stores data to be programmed.
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **PGM_DATA_6** `[31:0]` (RW)
///
///   Configures the 6th 32-bit data to be programmed.
#[repr(transparent)]
pub struct PgmData6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PgmData6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PgmData6 {
    // ======== Field Constants ========

    /// **PGM_DATA_6**
    ///
    /// Configures the 6th 32-bit data to be programmed.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PGM_DATA_6_OFFSET: u32 = 0;
    pub const PGM_DATA_6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PGM_DATA_6** `[31:0]` (RW)
    ///
    /// Configures the 6th 32-bit data to be programmed.
    #[inline(always)]
    pub fn pgm_data_6(&self) -> u32 {
        (self.value.read_bits() & Self::PGM_DATA_6_MASK) >> Self::PGM_DATA_6_OFFSET
    }

    /// Write **PGM_DATA_6** `[31:0]` (RW)
    ///
    /// Configures the 6th 32-bit data to be programmed.
    #[inline(always)]
    pub fn write_pgm_data_6(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PGM_DATA_6_MASK)
                | ((value << Self::PGM_DATA_6_OFFSET) & Self::PGM_DATA_6_MASK)
        });
    }
}

/// PGM_DATA6 value type for batch read/modify/write operations
///
/// Use `pgmdata6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pgmdata6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PgmData6Val {
    bits: u32,
}

impl PgmData6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PGM_DATA_6** `[31:0]`
    ///
    /// Configures the 6th 32-bit data to be programmed.
    #[inline(always)]
    pub const fn pgm_data_6(&self) -> u32 {
        (self.bits & PgmData6::PGM_DATA_6_MASK) >> PgmData6::PGM_DATA_6_OFFSET
    }

    /// Set **PGM_DATA_6** `[31:0]`
    ///
    /// Configures the 6th 32-bit data to be programmed.
    #[inline(always)]
    pub const fn with_pgm_data_6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PgmData6::PGM_DATA_6_MASK)
                | ((value << PgmData6::PGM_DATA_6_OFFSET) & PgmData6::PGM_DATA_6_MASK),
        }
    }
}

impl Default for PgmData6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PgmData6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PgmData6Val {
        PgmData6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PgmData6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PgmData6Val) -> PgmData6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PGM_DATA7 Register
///
/// Register 7 that stores data to be programmed.
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **PGM_DATA_7** `[31:0]` (RW)
///
///   Configures the 7th 32-bit data to be programmed.
#[repr(transparent)]
pub struct PgmData7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PgmData7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PgmData7 {
    // ======== Field Constants ========

    /// **PGM_DATA_7**
    ///
    /// Configures the 7th 32-bit data to be programmed.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PGM_DATA_7_OFFSET: u32 = 0;
    pub const PGM_DATA_7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PGM_DATA_7** `[31:0]` (RW)
    ///
    /// Configures the 7th 32-bit data to be programmed.
    #[inline(always)]
    pub fn pgm_data_7(&self) -> u32 {
        (self.value.read_bits() & Self::PGM_DATA_7_MASK) >> Self::PGM_DATA_7_OFFSET
    }

    /// Write **PGM_DATA_7** `[31:0]` (RW)
    ///
    /// Configures the 7th 32-bit data to be programmed.
    #[inline(always)]
    pub fn write_pgm_data_7(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PGM_DATA_7_MASK)
                | ((value << Self::PGM_DATA_7_OFFSET) & Self::PGM_DATA_7_MASK)
        });
    }
}

/// PGM_DATA7 value type for batch read/modify/write operations
///
/// Use `pgmdata7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pgmdata7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PgmData7Val {
    bits: u32,
}

impl PgmData7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PGM_DATA_7** `[31:0]`
    ///
    /// Configures the 7th 32-bit data to be programmed.
    #[inline(always)]
    pub const fn pgm_data_7(&self) -> u32 {
        (self.bits & PgmData7::PGM_DATA_7_MASK) >> PgmData7::PGM_DATA_7_OFFSET
    }

    /// Set **PGM_DATA_7** `[31:0]`
    ///
    /// Configures the 7th 32-bit data to be programmed.
    #[inline(always)]
    pub const fn with_pgm_data_7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PgmData7::PGM_DATA_7_MASK)
                | ((value << PgmData7::PGM_DATA_7_OFFSET) & PgmData7::PGM_DATA_7_MASK),
        }
    }
}

impl Default for PgmData7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PgmData7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PgmData7Val {
        PgmData7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PgmData7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PgmData7Val) -> PgmData7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PGM_CHECK_VALUE0 Register
///
/// Register 0 that stores the RS code to be programmed.
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **PGM_RS_DATA_0** `[31:0]` (RW)
///
///   Configures the 0th 32-bit RS code to be programmed.
#[repr(transparent)]
pub struct PgmCheckValue0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PgmCheckValue0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PgmCheckValue0 {
    // ======== Field Constants ========

    /// **PGM_RS_DATA_0**
    ///
    /// Configures the 0th 32-bit RS code to be programmed.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PGM_RS_DATA_0_OFFSET: u32 = 0;
    pub const PGM_RS_DATA_0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PGM_RS_DATA_0** `[31:0]` (RW)
    ///
    /// Configures the 0th 32-bit RS code to be programmed.
    #[inline(always)]
    pub fn pgm_rs_data_0(&self) -> u32 {
        (self.value.read_bits() & Self::PGM_RS_DATA_0_MASK) >> Self::PGM_RS_DATA_0_OFFSET
    }

    /// Write **PGM_RS_DATA_0** `[31:0]` (RW)
    ///
    /// Configures the 0th 32-bit RS code to be programmed.
    #[inline(always)]
    pub fn write_pgm_rs_data_0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PGM_RS_DATA_0_MASK)
                | ((value << Self::PGM_RS_DATA_0_OFFSET) & Self::PGM_RS_DATA_0_MASK)
        });
    }
}

/// PGM_CHECK_VALUE0 value type for batch read/modify/write operations
///
/// Use `pgmcheckvalue0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pgmcheckvalue0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PgmCheckValue0Val {
    bits: u32,
}

impl PgmCheckValue0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PGM_RS_DATA_0** `[31:0]`
    ///
    /// Configures the 0th 32-bit RS code to be programmed.
    #[inline(always)]
    pub const fn pgm_rs_data_0(&self) -> u32 {
        (self.bits & PgmCheckValue0::PGM_RS_DATA_0_MASK) >> PgmCheckValue0::PGM_RS_DATA_0_OFFSET
    }

    /// Set **PGM_RS_DATA_0** `[31:0]`
    ///
    /// Configures the 0th 32-bit RS code to be programmed.
    #[inline(always)]
    pub const fn with_pgm_rs_data_0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PgmCheckValue0::PGM_RS_DATA_0_MASK)
                | ((value << PgmCheckValue0::PGM_RS_DATA_0_OFFSET)
                    & PgmCheckValue0::PGM_RS_DATA_0_MASK),
        }
    }
}

impl Default for PgmCheckValue0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PgmCheckValue0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PgmCheckValue0Val {
        PgmCheckValue0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PgmCheckValue0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PgmCheckValue0Val) -> PgmCheckValue0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PGM_CHECK_VALUE1 Register
///
/// Register 1 that stores the RS code to be programmed.
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **PGM_RS_DATA_1** `[31:0]` (RW)
///
///   Configures the 1st 32-bit RS code to be programmed.
#[repr(transparent)]
pub struct PgmCheckValue1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PgmCheckValue1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PgmCheckValue1 {
    // ======== Field Constants ========

    /// **PGM_RS_DATA_1**
    ///
    /// Configures the 1st 32-bit RS code to be programmed.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PGM_RS_DATA_1_OFFSET: u32 = 0;
    pub const PGM_RS_DATA_1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PGM_RS_DATA_1** `[31:0]` (RW)
    ///
    /// Configures the 1st 32-bit RS code to be programmed.
    #[inline(always)]
    pub fn pgm_rs_data_1(&self) -> u32 {
        (self.value.read_bits() & Self::PGM_RS_DATA_1_MASK) >> Self::PGM_RS_DATA_1_OFFSET
    }

    /// Write **PGM_RS_DATA_1** `[31:0]` (RW)
    ///
    /// Configures the 1st 32-bit RS code to be programmed.
    #[inline(always)]
    pub fn write_pgm_rs_data_1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PGM_RS_DATA_1_MASK)
                | ((value << Self::PGM_RS_DATA_1_OFFSET) & Self::PGM_RS_DATA_1_MASK)
        });
    }
}

/// PGM_CHECK_VALUE1 value type for batch read/modify/write operations
///
/// Use `pgmcheckvalue1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pgmcheckvalue1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PgmCheckValue1Val {
    bits: u32,
}

impl PgmCheckValue1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PGM_RS_DATA_1** `[31:0]`
    ///
    /// Configures the 1st 32-bit RS code to be programmed.
    #[inline(always)]
    pub const fn pgm_rs_data_1(&self) -> u32 {
        (self.bits & PgmCheckValue1::PGM_RS_DATA_1_MASK) >> PgmCheckValue1::PGM_RS_DATA_1_OFFSET
    }

    /// Set **PGM_RS_DATA_1** `[31:0]`
    ///
    /// Configures the 1st 32-bit RS code to be programmed.
    #[inline(always)]
    pub const fn with_pgm_rs_data_1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PgmCheckValue1::PGM_RS_DATA_1_MASK)
                | ((value << PgmCheckValue1::PGM_RS_DATA_1_OFFSET)
                    & PgmCheckValue1::PGM_RS_DATA_1_MASK),
        }
    }
}

impl Default for PgmCheckValue1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PgmCheckValue1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PgmCheckValue1Val {
        PgmCheckValue1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PgmCheckValue1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PgmCheckValue1Val) -> PgmCheckValue1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PGM_CHECK_VALUE2 Register
///
/// Register 2 that stores the RS code to be programmed.
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **PGM_RS_DATA_2** `[31:0]` (RW)
///
///   Configures the 2nd 32-bit RS code to be programmed.
#[repr(transparent)]
pub struct PgmCheckValue2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PgmCheckValue2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PgmCheckValue2 {
    // ======== Field Constants ========

    /// **PGM_RS_DATA_2**
    ///
    /// Configures the 2nd 32-bit RS code to be programmed.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PGM_RS_DATA_2_OFFSET: u32 = 0;
    pub const PGM_RS_DATA_2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PGM_RS_DATA_2** `[31:0]` (RW)
    ///
    /// Configures the 2nd 32-bit RS code to be programmed.
    #[inline(always)]
    pub fn pgm_rs_data_2(&self) -> u32 {
        (self.value.read_bits() & Self::PGM_RS_DATA_2_MASK) >> Self::PGM_RS_DATA_2_OFFSET
    }

    /// Write **PGM_RS_DATA_2** `[31:0]` (RW)
    ///
    /// Configures the 2nd 32-bit RS code to be programmed.
    #[inline(always)]
    pub fn write_pgm_rs_data_2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PGM_RS_DATA_2_MASK)
                | ((value << Self::PGM_RS_DATA_2_OFFSET) & Self::PGM_RS_DATA_2_MASK)
        });
    }
}

/// PGM_CHECK_VALUE2 value type for batch read/modify/write operations
///
/// Use `pgmcheckvalue2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pgmcheckvalue2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PgmCheckValue2Val {
    bits: u32,
}

impl PgmCheckValue2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PGM_RS_DATA_2** `[31:0]`
    ///
    /// Configures the 2nd 32-bit RS code to be programmed.
    #[inline(always)]
    pub const fn pgm_rs_data_2(&self) -> u32 {
        (self.bits & PgmCheckValue2::PGM_RS_DATA_2_MASK) >> PgmCheckValue2::PGM_RS_DATA_2_OFFSET
    }

    /// Set **PGM_RS_DATA_2** `[31:0]`
    ///
    /// Configures the 2nd 32-bit RS code to be programmed.
    #[inline(always)]
    pub const fn with_pgm_rs_data_2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PgmCheckValue2::PGM_RS_DATA_2_MASK)
                | ((value << PgmCheckValue2::PGM_RS_DATA_2_OFFSET)
                    & PgmCheckValue2::PGM_RS_DATA_2_MASK),
        }
    }
}

impl Default for PgmCheckValue2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PgmCheckValue2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PgmCheckValue2Val {
        PgmCheckValue2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PgmCheckValue2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PgmCheckValue2Val) -> PgmCheckValue2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RD_WR_DIS Register
///
/// BLOCK0 data register 0.
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **WR_DIS** `[31:0]` (RO)
///
///   Represents whether programming of individual eFuse memory bit is disabled or enabled. 1: Disabled. 0 Enabled.
#[repr(transparent)]
pub struct RdWrDis {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdWrDis {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdWrDis {
    // ======== Field Constants ========

    /// **WR_DIS**
    ///
    /// Represents whether programming of individual eFuse memory bit is disabled or enabled. 1: Disabled. 0 Enabled.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WR_DIS_OFFSET: u32 = 0;
    pub const WR_DIS_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **WR_DIS** `[31:0]` (RO)
    ///
    /// Represents whether programming of individual eFuse memory bit is disabled or enabled. 1: Disabled. 0 Enabled.
    #[inline(always)]
    pub fn wr_dis(&self) -> u32 {
        (self.value.read_bits() & Self::WR_DIS_MASK) >> Self::WR_DIS_OFFSET
    }
}

/// RD_WR_DIS value type for batch read/modify/write operations
///
/// Use `rdwrdis.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdwrdis.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdWrDisVal {
    bits: u32,
}

impl RdWrDisVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **WR_DIS** `[31:0]`
    ///
    /// Represents whether programming of individual eFuse memory bit is disabled or enabled. 1: Disabled. 0 Enabled.
    #[inline(always)]
    pub const fn wr_dis(&self) -> u32 {
        (self.bits & RdWrDis::WR_DIS_MASK) >> RdWrDis::WR_DIS_OFFSET
    }
}

impl Default for RdWrDisVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdWrDis {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdWrDisVal {
        RdWrDisVal::from_bits(self.value.read_bits())
    }
}

/// RD_REPEAT_DATA0 Register
///
/// BLOCK0 data register 1.
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **KM_HUK_GEN_STATE_LOW** `[31:26]` (RO)
///
///   Set this bit to control validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid.
///
/// - **USB_PHY_SEL** `[25]` (RO)
///
///   TBD
///
/// - **USB_OTG11_DREFH** `[24:23]` (RO)
///
///   USB intphy of usb otg11 signle-end input high threshold, 1.76V to 2V. Step by 80mV
///
/// - **USB_DEVICE_DREFH** `[22:21]` (RO)
///
///   USB intphy of usb device signle-end input high threshold, 1.76V to 2V. Step by 80mV
///
/// - **DIS_DOWNLOAD_MANUAL_ENCRYPT** `[20]` (RO)
///
///   Represents whether flash encrypt function is disabled or enabled(except in SPI boot mode). 1: disabled. 0: enabled.
///
/// - **DIS_PAD_JTAG** `[19]` (RO)
///
///   Represents whether JTAG is disabled in the hard way(permanently). 1: disabled. 0: enabled.
///
/// - **SOFT_DIS_JTAG** `[18:16]` (RO)
///
///   Represents whether JTAG is disabled in soft way. Odd number: disabled. Even number: enabled.
///
/// - **JTAG_SEL_ENABLE** `[15]` (RO)
///
///   Represents whether the selection between usb_to_jtag and pad_to_jtag through strapping gpio15 when both EFUSE_DIS_PAD_JTAG and EFUSE_DIS_USB_JTAG are equal to 0 is enabled or disabled. 1: enabled. 0: disabled.
///
/// - **DIS_TWAI** `[14]` (RO)
///
///   Represents whether TWAI function is disabled or enabled. 1: disabled. 0: enabled.
///
/// - **SPI_DOWNLOAD_MSPI_DIS** `[13]` (RO)
///
///   Set this bit to disable accessing MSPI flash/MSPI ram by SYS AXI matrix during boot_mode_download.
///
/// - **DIS_FORCE_DOWNLOAD** `[12]` (RO)
///
///   Represents whether the function that forces chip into download mode is disabled or enabled. 1: disabled. 0: enabled.
///
/// - **DIS_USB_SERIAL_JTAG** `[11]` (RO)
///
///   Represents whether USB-Serial-JTAG is disabled or enabled. 1: disabled. 0: enabled.
///
/// - **POWERGLITCH_EN** `[10]` (RO)
///
///   Represents whether power glitch function is enabled. 1: enabled. 0: disabled.
///
/// - **DIS_USB_JTAG** `[9]` (RO)
///
///   Represents whether the function of usb switch to jtag is disabled or enabled. 1: disabled. 0: enabled.
///
/// - **USB_OTG11_EXCHG_PINS** `[8]` (RO)
///
///   Enable usb otg11 exchange pins of D+ and D-.
///
/// - **USB_DEVICE_EXCHG_PINS** `[7]` (RO)
///
///   Enable usb device exchange pins of D+ and D-.
///
/// - **RD_DIS** `[6:0]` (RO)
///
///   Represents whether reading of individual eFuse block(block4~block10) is disabled or enabled. 1: disabled. 0: enabled.
#[repr(transparent)]
pub struct RdRepeatData0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdRepeatData0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdRepeatData0 {
    // ======== Field Constants ========

    /// **KM_HUK_GEN_STATE_LOW**
    ///
    /// Set this bit to control validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid.
    ///
    /// Bits `[31:26]`, mask `0xfc000000`
    pub const KM_HUK_GEN_STATE_LOW_OFFSET: u32 = 26;
    pub const KM_HUK_GEN_STATE_LOW_MASK: u32 = 0xfc000000;

    /// **USB_PHY_SEL**
    ///
    /// TBD
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const USB_PHY_SEL_OFFSET: u32 = 25;
    pub const USB_PHY_SEL_MASK: u32 = 0x02000000;
    pub const USB_PHY_SEL: u32 = 1 << 25;

    /// **USB_OTG11_DREFH**
    ///
    /// USB intphy of usb otg11 signle-end input high threshold, 1.76V to 2V. Step by 80mV
    ///
    /// Bits `[24:23]`, mask `0x01800000`
    pub const USB_OTG11_DREFH_OFFSET: u32 = 23;
    pub const USB_OTG11_DREFH_MASK: u32 = 0x01800000;

    /// **USB_DEVICE_DREFH**
    ///
    /// USB intphy of usb device signle-end input high threshold, 1.76V to 2V. Step by 80mV
    ///
    /// Bits `[22:21]`, mask `0x00600000`
    pub const USB_DEVICE_DREFH_OFFSET: u32 = 21;
    pub const USB_DEVICE_DREFH_MASK: u32 = 0x00600000;

    /// **DIS_DOWNLOAD_MANUAL_ENCRYPT**
    ///
    /// Represents whether flash encrypt function is disabled or enabled(except in SPI boot mode). 1: disabled. 0: enabled.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const DIS_DOWNLOAD_MANUAL_ENCRYPT_OFFSET: u32 = 20;
    pub const DIS_DOWNLOAD_MANUAL_ENCRYPT_MASK: u32 = 0x00100000;
    pub const DIS_DOWNLOAD_MANUAL_ENCRYPT: u32 = 1 << 20;

    /// **DIS_PAD_JTAG**
    ///
    /// Represents whether JTAG is disabled in the hard way(permanently). 1: disabled. 0: enabled.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const DIS_PAD_JTAG_OFFSET: u32 = 19;
    pub const DIS_PAD_JTAG_MASK: u32 = 0x00080000;
    pub const DIS_PAD_JTAG: u32 = 1 << 19;

    /// **SOFT_DIS_JTAG**
    ///
    /// Represents whether JTAG is disabled in soft way. Odd number: disabled. Even number: enabled.
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const SOFT_DIS_JTAG_OFFSET: u32 = 16;
    pub const SOFT_DIS_JTAG_MASK: u32 = 0x00070000;

    /// **JTAG_SEL_ENABLE**
    ///
    /// Represents whether the selection between usb_to_jtag and pad_to_jtag through strapping gpio15 when both EFUSE_DIS_PAD_JTAG and EFUSE_DIS_USB_JTAG are equal to 0 is enabled or disabled. 1: enabled. 0: disabled.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const JTAG_SEL_ENABLE_OFFSET: u32 = 15;
    pub const JTAG_SEL_ENABLE_MASK: u32 = 0x00008000;
    pub const JTAG_SEL_ENABLE: u32 = 1 << 15;

    /// **DIS_TWAI**
    ///
    /// Represents whether TWAI function is disabled or enabled. 1: disabled. 0: enabled.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const DIS_TWAI_OFFSET: u32 = 14;
    pub const DIS_TWAI_MASK: u32 = 0x00004000;
    pub const DIS_TWAI: u32 = 1 << 14;

    /// **SPI_DOWNLOAD_MSPI_DIS**
    ///
    /// Set this bit to disable accessing MSPI flash/MSPI ram by SYS AXI matrix during boot_mode_download.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SPI_DOWNLOAD_MSPI_DIS_OFFSET: u32 = 13;
    pub const SPI_DOWNLOAD_MSPI_DIS_MASK: u32 = 0x00002000;
    pub const SPI_DOWNLOAD_MSPI_DIS: u32 = 1 << 13;

    /// **DIS_FORCE_DOWNLOAD**
    ///
    /// Represents whether the function that forces chip into download mode is disabled or enabled. 1: disabled. 0: enabled.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const DIS_FORCE_DOWNLOAD_OFFSET: u32 = 12;
    pub const DIS_FORCE_DOWNLOAD_MASK: u32 = 0x00001000;
    pub const DIS_FORCE_DOWNLOAD: u32 = 1 << 12;

    /// **DIS_USB_SERIAL_JTAG**
    ///
    /// Represents whether USB-Serial-JTAG is disabled or enabled. 1: disabled. 0: enabled.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const DIS_USB_SERIAL_JTAG_OFFSET: u32 = 11;
    pub const DIS_USB_SERIAL_JTAG_MASK: u32 = 0x00000800;
    pub const DIS_USB_SERIAL_JTAG: u32 = 1 << 11;

    /// **POWERGLITCH_EN**
    ///
    /// Represents whether power glitch function is enabled. 1: enabled. 0: disabled.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const POWERGLITCH_EN_OFFSET: u32 = 10;
    pub const POWERGLITCH_EN_MASK: u32 = 0x00000400;
    pub const POWERGLITCH_EN: u32 = 1 << 10;

    /// **DIS_USB_JTAG**
    ///
    /// Represents whether the function of usb switch to jtag is disabled or enabled. 1: disabled. 0: enabled.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DIS_USB_JTAG_OFFSET: u32 = 9;
    pub const DIS_USB_JTAG_MASK: u32 = 0x00000200;
    pub const DIS_USB_JTAG: u32 = 1 << 9;

    /// **USB_OTG11_EXCHG_PINS**
    ///
    /// Enable usb otg11 exchange pins of D+ and D-.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const USB_OTG11_EXCHG_PINS_OFFSET: u32 = 8;
    pub const USB_OTG11_EXCHG_PINS_MASK: u32 = 0x00000100;
    pub const USB_OTG11_EXCHG_PINS: u32 = 1 << 8;

    /// **USB_DEVICE_EXCHG_PINS**
    ///
    /// Enable usb device exchange pins of D+ and D-.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const USB_DEVICE_EXCHG_PINS_OFFSET: u32 = 7;
    pub const USB_DEVICE_EXCHG_PINS_MASK: u32 = 0x00000080;
    pub const USB_DEVICE_EXCHG_PINS: u32 = 1 << 7;

    /// **RD_DIS**
    ///
    /// Represents whether reading of individual eFuse block(block4~block10) is disabled or enabled. 1: disabled. 0: enabled.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const RD_DIS_OFFSET: u32 = 0;
    pub const RD_DIS_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **KM_HUK_GEN_STATE_LOW** `[31:26]` (RO)
    ///
    /// Set this bit to control validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid.
    #[inline(always)]
    pub fn km_huk_gen_state_low(&self) -> u32 {
        (self.value.read_bits() & Self::KM_HUK_GEN_STATE_LOW_MASK)
            >> Self::KM_HUK_GEN_STATE_LOW_OFFSET
    }

    /// **USB_PHY_SEL** `[25]` (RO)
    ///
    /// TBD
    #[inline(always)]
    pub fn usb_phy_sel(&self) -> bool {
        (self.value.read_bits() & Self::USB_PHY_SEL_MASK) != 0
    }

    /// **USB_OTG11_DREFH** `[24:23]` (RO)
    ///
    /// USB intphy of usb otg11 signle-end input high threshold, 1.76V to 2V. Step by 80mV
    #[inline(always)]
    pub fn usb_otg11_drefh(&self) -> u32 {
        (self.value.read_bits() & Self::USB_OTG11_DREFH_MASK) >> Self::USB_OTG11_DREFH_OFFSET
    }

    /// **USB_DEVICE_DREFH** `[22:21]` (RO)
    ///
    /// USB intphy of usb device signle-end input high threshold, 1.76V to 2V. Step by 80mV
    #[inline(always)]
    pub fn usb_device_drefh(&self) -> u32 {
        (self.value.read_bits() & Self::USB_DEVICE_DREFH_MASK) >> Self::USB_DEVICE_DREFH_OFFSET
    }

    /// **DIS_DOWNLOAD_MANUAL_ENCRYPT** `[20]` (RO)
    ///
    /// Represents whether flash encrypt function is disabled or enabled(except in SPI boot mode). 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn dis_download_manual_encrypt(&self) -> bool {
        (self.value.read_bits() & Self::DIS_DOWNLOAD_MANUAL_ENCRYPT_MASK) != 0
    }

    /// **DIS_PAD_JTAG** `[19]` (RO)
    ///
    /// Represents whether JTAG is disabled in the hard way(permanently). 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn dis_pad_jtag(&self) -> bool {
        (self.value.read_bits() & Self::DIS_PAD_JTAG_MASK) != 0
    }

    /// **SOFT_DIS_JTAG** `[18:16]` (RO)
    ///
    /// Represents whether JTAG is disabled in soft way. Odd number: disabled. Even number: enabled.
    #[inline(always)]
    pub fn soft_dis_jtag(&self) -> u32 {
        (self.value.read_bits() & Self::SOFT_DIS_JTAG_MASK) >> Self::SOFT_DIS_JTAG_OFFSET
    }

    /// **JTAG_SEL_ENABLE** `[15]` (RO)
    ///
    /// Represents whether the selection between usb_to_jtag and pad_to_jtag through strapping gpio15 when both EFUSE_DIS_PAD_JTAG and EFUSE_DIS_USB_JTAG are equal to 0 is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub fn jtag_sel_enable(&self) -> bool {
        (self.value.read_bits() & Self::JTAG_SEL_ENABLE_MASK) != 0
    }

    /// **DIS_TWAI** `[14]` (RO)
    ///
    /// Represents whether TWAI function is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn dis_twai(&self) -> bool {
        (self.value.read_bits() & Self::DIS_TWAI_MASK) != 0
    }

    /// **SPI_DOWNLOAD_MSPI_DIS** `[13]` (RO)
    ///
    /// Set this bit to disable accessing MSPI flash/MSPI ram by SYS AXI matrix during boot_mode_download.
    #[inline(always)]
    pub fn spi_download_mspi_dis(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DOWNLOAD_MSPI_DIS_MASK) != 0
    }

    /// **DIS_FORCE_DOWNLOAD** `[12]` (RO)
    ///
    /// Represents whether the function that forces chip into download mode is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn dis_force_download(&self) -> bool {
        (self.value.read_bits() & Self::DIS_FORCE_DOWNLOAD_MASK) != 0
    }

    /// **DIS_USB_SERIAL_JTAG** `[11]` (RO)
    ///
    /// Represents whether USB-Serial-JTAG is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn dis_usb_serial_jtag(&self) -> bool {
        (self.value.read_bits() & Self::DIS_USB_SERIAL_JTAG_MASK) != 0
    }

    /// **POWERGLITCH_EN** `[10]` (RO)
    ///
    /// Represents whether power glitch function is enabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub fn powerglitch_en(&self) -> bool {
        (self.value.read_bits() & Self::POWERGLITCH_EN_MASK) != 0
    }

    /// **DIS_USB_JTAG** `[9]` (RO)
    ///
    /// Represents whether the function of usb switch to jtag is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn dis_usb_jtag(&self) -> bool {
        (self.value.read_bits() & Self::DIS_USB_JTAG_MASK) != 0
    }

    /// **USB_OTG11_EXCHG_PINS** `[8]` (RO)
    ///
    /// Enable usb otg11 exchange pins of D+ and D-.
    #[inline(always)]
    pub fn usb_otg11_exchg_pins(&self) -> bool {
        (self.value.read_bits() & Self::USB_OTG11_EXCHG_PINS_MASK) != 0
    }

    /// **USB_DEVICE_EXCHG_PINS** `[7]` (RO)
    ///
    /// Enable usb device exchange pins of D+ and D-.
    #[inline(always)]
    pub fn usb_device_exchg_pins(&self) -> bool {
        (self.value.read_bits() & Self::USB_DEVICE_EXCHG_PINS_MASK) != 0
    }

    /// **RD_DIS** `[6:0]` (RO)
    ///
    /// Represents whether reading of individual eFuse block(block4~block10) is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn rd_dis(&self) -> u32 {
        (self.value.read_bits() & Self::RD_DIS_MASK) >> Self::RD_DIS_OFFSET
    }
}

/// RD_REPEAT_DATA0 value type for batch read/modify/write operations
///
/// Use `rdrepeatdata0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdrepeatdata0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdRepeatData0Val {
    bits: u32,
}

impl RdRepeatData0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KM_HUK_GEN_STATE_LOW** `[31:26]`
    ///
    /// Set this bit to control validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid.
    #[inline(always)]
    pub const fn km_huk_gen_state_low(&self) -> u32 {
        (self.bits & RdRepeatData0::KM_HUK_GEN_STATE_LOW_MASK)
            >> RdRepeatData0::KM_HUK_GEN_STATE_LOW_OFFSET
    }

    /// Read **USB_PHY_SEL** `[25:25]`
    ///
    /// TBD
    #[inline(always)]
    pub const fn usb_phy_sel(&self) -> bool {
        (self.bits & RdRepeatData0::USB_PHY_SEL_MASK) != 0
    }

    /// Read **USB_OTG11_DREFH** `[24:23]`
    ///
    /// USB intphy of usb otg11 signle-end input high threshold, 1.76V to 2V. Step by 80mV
    #[inline(always)]
    pub const fn usb_otg11_drefh(&self) -> u32 {
        (self.bits & RdRepeatData0::USB_OTG11_DREFH_MASK) >> RdRepeatData0::USB_OTG11_DREFH_OFFSET
    }

    /// Read **USB_DEVICE_DREFH** `[22:21]`
    ///
    /// USB intphy of usb device signle-end input high threshold, 1.76V to 2V. Step by 80mV
    #[inline(always)]
    pub const fn usb_device_drefh(&self) -> u32 {
        (self.bits & RdRepeatData0::USB_DEVICE_DREFH_MASK) >> RdRepeatData0::USB_DEVICE_DREFH_OFFSET
    }

    /// Read **DIS_DOWNLOAD_MANUAL_ENCRYPT** `[20:20]`
    ///
    /// Represents whether flash encrypt function is disabled or enabled(except in SPI boot mode). 1: disabled. 0: enabled.
    #[inline(always)]
    pub const fn dis_download_manual_encrypt(&self) -> bool {
        (self.bits & RdRepeatData0::DIS_DOWNLOAD_MANUAL_ENCRYPT_MASK) != 0
    }

    /// Read **DIS_PAD_JTAG** `[19:19]`
    ///
    /// Represents whether JTAG is disabled in the hard way(permanently). 1: disabled. 0: enabled.
    #[inline(always)]
    pub const fn dis_pad_jtag(&self) -> bool {
        (self.bits & RdRepeatData0::DIS_PAD_JTAG_MASK) != 0
    }

    /// Read **SOFT_DIS_JTAG** `[18:16]`
    ///
    /// Represents whether JTAG is disabled in soft way. Odd number: disabled. Even number: enabled.
    #[inline(always)]
    pub const fn soft_dis_jtag(&self) -> u32 {
        (self.bits & RdRepeatData0::SOFT_DIS_JTAG_MASK) >> RdRepeatData0::SOFT_DIS_JTAG_OFFSET
    }

    /// Read **JTAG_SEL_ENABLE** `[15:15]`
    ///
    /// Represents whether the selection between usb_to_jtag and pad_to_jtag through strapping gpio15 when both EFUSE_DIS_PAD_JTAG and EFUSE_DIS_USB_JTAG are equal to 0 is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub const fn jtag_sel_enable(&self) -> bool {
        (self.bits & RdRepeatData0::JTAG_SEL_ENABLE_MASK) != 0
    }

    /// Read **DIS_TWAI** `[14:14]`
    ///
    /// Represents whether TWAI function is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub const fn dis_twai(&self) -> bool {
        (self.bits & RdRepeatData0::DIS_TWAI_MASK) != 0
    }

    /// Read **SPI_DOWNLOAD_MSPI_DIS** `[13:13]`
    ///
    /// Set this bit to disable accessing MSPI flash/MSPI ram by SYS AXI matrix during boot_mode_download.
    #[inline(always)]
    pub const fn spi_download_mspi_dis(&self) -> bool {
        (self.bits & RdRepeatData0::SPI_DOWNLOAD_MSPI_DIS_MASK) != 0
    }

    /// Read **DIS_FORCE_DOWNLOAD** `[12:12]`
    ///
    /// Represents whether the function that forces chip into download mode is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub const fn dis_force_download(&self) -> bool {
        (self.bits & RdRepeatData0::DIS_FORCE_DOWNLOAD_MASK) != 0
    }

    /// Read **DIS_USB_SERIAL_JTAG** `[11:11]`
    ///
    /// Represents whether USB-Serial-JTAG is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub const fn dis_usb_serial_jtag(&self) -> bool {
        (self.bits & RdRepeatData0::DIS_USB_SERIAL_JTAG_MASK) != 0
    }

    /// Read **POWERGLITCH_EN** `[10:10]`
    ///
    /// Represents whether power glitch function is enabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub const fn powerglitch_en(&self) -> bool {
        (self.bits & RdRepeatData0::POWERGLITCH_EN_MASK) != 0
    }

    /// Read **DIS_USB_JTAG** `[9:9]`
    ///
    /// Represents whether the function of usb switch to jtag is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub const fn dis_usb_jtag(&self) -> bool {
        (self.bits & RdRepeatData0::DIS_USB_JTAG_MASK) != 0
    }

    /// Read **USB_OTG11_EXCHG_PINS** `[8:8]`
    ///
    /// Enable usb otg11 exchange pins of D+ and D-.
    #[inline(always)]
    pub const fn usb_otg11_exchg_pins(&self) -> bool {
        (self.bits & RdRepeatData0::USB_OTG11_EXCHG_PINS_MASK) != 0
    }

    /// Read **USB_DEVICE_EXCHG_PINS** `[7:7]`
    ///
    /// Enable usb device exchange pins of D+ and D-.
    #[inline(always)]
    pub const fn usb_device_exchg_pins(&self) -> bool {
        (self.bits & RdRepeatData0::USB_DEVICE_EXCHG_PINS_MASK) != 0
    }

    /// Read **RD_DIS** `[6:0]`
    ///
    /// Represents whether reading of individual eFuse block(block4~block10) is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub const fn rd_dis(&self) -> u32 {
        (self.bits & RdRepeatData0::RD_DIS_MASK) >> RdRepeatData0::RD_DIS_OFFSET
    }
}

impl Default for RdRepeatData0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdRepeatData0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdRepeatData0Val {
        RdRepeatData0Val::from_bits(self.value.read_bits())
    }
}

/// RD_REPEAT_DATA1 Register
///
/// BLOCK0 data register 2.
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **KEY_PURPOSE_1** `[31:28]` (RO)
///
///   Represents the purpose of Key1.
///
/// - **KEY_PURPOSE_0** `[27:24]` (RO)
///
///   Represents the purpose of Key0.
///
/// - **SECURE_BOOT_KEY_REVOKE2** `[23]` (RO)
///
///   Represents whether revoking third secure boot key is enabled or disabled. 1: enabled. 0: disabled.
///
/// - **SECURE_BOOT_KEY_REVOKE1** `[22]` (RO)
///
///   Represents whether revoking second secure boot key is enabled or disabled. 1: enabled. 0: disabled.
///
/// - **SECURE_BOOT_KEY_REVOKE0** `[21]` (RO)
///
///   Represents whether revoking first secure boot key is enabled or disabled. 1: enabled. 0: disabled.
///
/// - **SPI_BOOT_CRYPT_CNT** `[20:18]` (RO)
///
///   Represents whether SPI boot encrypt/decrypt is disabled or enabled. Odd number of 1: enabled. Even number of 1: disabled.
///
/// - **WDT_DELAY_SEL** `[17:16]` (RO)
///
///   Represents whether RTC watchdog timeout threshold is selected at startup. 1: selected. 0: not selected.
///
/// - **XTS_KEY_LENGTH_256** `[14]` (RO)
///
///   Set this bit to configure flash encryption use xts-128 key, else use xts-256 key.
///
/// - **FORCE_DISABLE_SW_INIT_KEY** `[13]` (RO)
///
///   Set this bit to disable software written init key, and force use efuse_init_key.
///
/// - **FORCE_USE_KEY_MANAGER_KEY** `[12:9]` (RO)
///
///   Set each bit to control whether corresponding key must come from key manager.. 1 is true, 0 is false. Bit0: ecdsa. Bit1: xts. Bit2: hmac. Bit3: ds.
///
/// - **KM_DEPLOY_ONLY_ONCE** `[8:5]` (RO)
///
///   Set each bit to control whether corresponding key can only be deployed once. 1 is true, 0 is false. Bit0: ecdsa. Bit1: xts. Bit2: hmac. Bit3: ds.
///
/// - **KM_RND_SWITCH_CYCLE** `[4:3]` (RO)
///
///   Set bits to control key manager random number switch cycle. 0: control by register. 1: 8 km clk cycles. 2: 16 km cycles. 3: 32 km cycles.
///
/// - **KM_HUK_GEN_STATE_HIGH** `[2:0]` (RO)
///
///   Set this bit to control validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid.
#[repr(transparent)]
pub struct RdRepeatData1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdRepeatData1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdRepeatData1 {
    // ======== Field Constants ========

    /// **KEY_PURPOSE_1**
    ///
    /// Represents the purpose of Key1.
    ///
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const KEY_PURPOSE_1_OFFSET: u32 = 28;
    pub const KEY_PURPOSE_1_MASK: u32 = 0xf0000000;

    /// **KEY_PURPOSE_0**
    ///
    /// Represents the purpose of Key0.
    ///
    /// Bits `[27:24]`, mask `0x0f000000`
    pub const KEY_PURPOSE_0_OFFSET: u32 = 24;
    pub const KEY_PURPOSE_0_MASK: u32 = 0x0f000000;

    /// **SECURE_BOOT_KEY_REVOKE2**
    ///
    /// Represents whether revoking third secure boot key is enabled or disabled. 1: enabled. 0: disabled.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const SECURE_BOOT_KEY_REVOKE2_OFFSET: u32 = 23;
    pub const SECURE_BOOT_KEY_REVOKE2_MASK: u32 = 0x00800000;
    pub const SECURE_BOOT_KEY_REVOKE2: u32 = 1 << 23;

    /// **SECURE_BOOT_KEY_REVOKE1**
    ///
    /// Represents whether revoking second secure boot key is enabled or disabled. 1: enabled. 0: disabled.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const SECURE_BOOT_KEY_REVOKE1_OFFSET: u32 = 22;
    pub const SECURE_BOOT_KEY_REVOKE1_MASK: u32 = 0x00400000;
    pub const SECURE_BOOT_KEY_REVOKE1: u32 = 1 << 22;

    /// **SECURE_BOOT_KEY_REVOKE0**
    ///
    /// Represents whether revoking first secure boot key is enabled or disabled. 1: enabled. 0: disabled.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SECURE_BOOT_KEY_REVOKE0_OFFSET: u32 = 21;
    pub const SECURE_BOOT_KEY_REVOKE0_MASK: u32 = 0x00200000;
    pub const SECURE_BOOT_KEY_REVOKE0: u32 = 1 << 21;

    /// **SPI_BOOT_CRYPT_CNT**
    ///
    /// Represents whether SPI boot encrypt/decrypt is disabled or enabled. Odd number of 1: enabled. Even number of 1: disabled.
    ///
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const SPI_BOOT_CRYPT_CNT_OFFSET: u32 = 18;
    pub const SPI_BOOT_CRYPT_CNT_MASK: u32 = 0x001c0000;

    /// **WDT_DELAY_SEL**
    ///
    /// Represents whether RTC watchdog timeout threshold is selected at startup. 1: selected. 0: not selected.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const WDT_DELAY_SEL_OFFSET: u32 = 16;
    pub const WDT_DELAY_SEL_MASK: u32 = 0x00030000;

    /// **XTS_KEY_LENGTH_256**
    ///
    /// Set this bit to configure flash encryption use xts-128 key, else use xts-256 key.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const XTS_KEY_LENGTH_256_OFFSET: u32 = 14;
    pub const XTS_KEY_LENGTH_256_MASK: u32 = 0x00004000;
    pub const XTS_KEY_LENGTH_256: u32 = 1 << 14;

    /// **FORCE_DISABLE_SW_INIT_KEY**
    ///
    /// Set this bit to disable software written init key, and force use efuse_init_key.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const FORCE_DISABLE_SW_INIT_KEY_OFFSET: u32 = 13;
    pub const FORCE_DISABLE_SW_INIT_KEY_MASK: u32 = 0x00002000;
    pub const FORCE_DISABLE_SW_INIT_KEY: u32 = 1 << 13;

    /// **FORCE_USE_KEY_MANAGER_KEY**
    ///
    /// Set each bit to control whether corresponding key must come from key manager.. 1 is true, 0 is false. Bit0: ecdsa. Bit1: xts. Bit2: hmac. Bit3: ds.
    ///
    /// Bits `[12:9]`, mask `0x00001e00`
    pub const FORCE_USE_KEY_MANAGER_KEY_OFFSET: u32 = 9;
    pub const FORCE_USE_KEY_MANAGER_KEY_MASK: u32 = 0x00001e00;

    /// **KM_DEPLOY_ONLY_ONCE**
    ///
    /// Set each bit to control whether corresponding key can only be deployed once. 1 is true, 0 is false. Bit0: ecdsa. Bit1: xts. Bit2: hmac. Bit3: ds.
    ///
    /// Bits `[8:5]`, mask `0x000001e0`
    pub const KM_DEPLOY_ONLY_ONCE_OFFSET: u32 = 5;
    pub const KM_DEPLOY_ONLY_ONCE_MASK: u32 = 0x000001e0;

    /// **KM_RND_SWITCH_CYCLE**
    ///
    /// Set bits to control key manager random number switch cycle. 0: control by register. 1: 8 km clk cycles. 2: 16 km cycles. 3: 32 km cycles.
    ///
    /// Bits `[4:3]`, mask `0x00000018`
    pub const KM_RND_SWITCH_CYCLE_OFFSET: u32 = 3;
    pub const KM_RND_SWITCH_CYCLE_MASK: u32 = 0x00000018;

    /// **KM_HUK_GEN_STATE_HIGH**
    ///
    /// Set this bit to control validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const KM_HUK_GEN_STATE_HIGH_OFFSET: u32 = 0;
    pub const KM_HUK_GEN_STATE_HIGH_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **KEY_PURPOSE_1** `[31:28]` (RO)
    ///
    /// Represents the purpose of Key1.
    #[inline(always)]
    pub fn key_purpose_1(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_PURPOSE_1_MASK) >> Self::KEY_PURPOSE_1_OFFSET
    }

    /// **KEY_PURPOSE_0** `[27:24]` (RO)
    ///
    /// Represents the purpose of Key0.
    #[inline(always)]
    pub fn key_purpose_0(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_PURPOSE_0_MASK) >> Self::KEY_PURPOSE_0_OFFSET
    }

    /// **SECURE_BOOT_KEY_REVOKE2** `[23]` (RO)
    ///
    /// Represents whether revoking third secure boot key is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub fn secure_boot_key_revoke2(&self) -> bool {
        (self.value.read_bits() & Self::SECURE_BOOT_KEY_REVOKE2_MASK) != 0
    }

    /// **SECURE_BOOT_KEY_REVOKE1** `[22]` (RO)
    ///
    /// Represents whether revoking second secure boot key is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub fn secure_boot_key_revoke1(&self) -> bool {
        (self.value.read_bits() & Self::SECURE_BOOT_KEY_REVOKE1_MASK) != 0
    }

    /// **SECURE_BOOT_KEY_REVOKE0** `[21]` (RO)
    ///
    /// Represents whether revoking first secure boot key is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub fn secure_boot_key_revoke0(&self) -> bool {
        (self.value.read_bits() & Self::SECURE_BOOT_KEY_REVOKE0_MASK) != 0
    }

    /// **SPI_BOOT_CRYPT_CNT** `[20:18]` (RO)
    ///
    /// Represents whether SPI boot encrypt/decrypt is disabled or enabled. Odd number of 1: enabled. Even number of 1: disabled.
    #[inline(always)]
    pub fn spi_boot_crypt_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BOOT_CRYPT_CNT_MASK) >> Self::SPI_BOOT_CRYPT_CNT_OFFSET
    }

    /// **WDT_DELAY_SEL** `[17:16]` (RO)
    ///
    /// Represents whether RTC watchdog timeout threshold is selected at startup. 1: selected. 0: not selected.
    #[inline(always)]
    pub fn wdt_delay_sel(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_DELAY_SEL_MASK) >> Self::WDT_DELAY_SEL_OFFSET
    }

    /// **XTS_KEY_LENGTH_256** `[14]` (RO)
    ///
    /// Set this bit to configure flash encryption use xts-128 key, else use xts-256 key.
    #[inline(always)]
    pub fn xts_key_length_256(&self) -> bool {
        (self.value.read_bits() & Self::XTS_KEY_LENGTH_256_MASK) != 0
    }

    /// **FORCE_DISABLE_SW_INIT_KEY** `[13]` (RO)
    ///
    /// Set this bit to disable software written init key, and force use efuse_init_key.
    #[inline(always)]
    pub fn force_disable_sw_init_key(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_DISABLE_SW_INIT_KEY_MASK) != 0
    }

    /// **FORCE_USE_KEY_MANAGER_KEY** `[12:9]` (RO)
    ///
    /// Set each bit to control whether corresponding key must come from key manager.. 1 is true, 0 is false. Bit0: ecdsa. Bit1: xts. Bit2: hmac. Bit3: ds.
    #[inline(always)]
    pub fn force_use_key_manager_key(&self) -> u32 {
        (self.value.read_bits() & Self::FORCE_USE_KEY_MANAGER_KEY_MASK)
            >> Self::FORCE_USE_KEY_MANAGER_KEY_OFFSET
    }

    /// **KM_DEPLOY_ONLY_ONCE** `[8:5]` (RO)
    ///
    /// Set each bit to control whether corresponding key can only be deployed once. 1 is true, 0 is false. Bit0: ecdsa. Bit1: xts. Bit2: hmac. Bit3: ds.
    #[inline(always)]
    pub fn km_deploy_only_once(&self) -> u32 {
        (self.value.read_bits() & Self::KM_DEPLOY_ONLY_ONCE_MASK)
            >> Self::KM_DEPLOY_ONLY_ONCE_OFFSET
    }

    /// **KM_RND_SWITCH_CYCLE** `[4:3]` (RO)
    ///
    /// Set bits to control key manager random number switch cycle. 0: control by register. 1: 8 km clk cycles. 2: 16 km cycles. 3: 32 km cycles.
    #[inline(always)]
    pub fn km_rnd_switch_cycle(&self) -> u32 {
        (self.value.read_bits() & Self::KM_RND_SWITCH_CYCLE_MASK)
            >> Self::KM_RND_SWITCH_CYCLE_OFFSET
    }

    /// **KM_HUK_GEN_STATE_HIGH** `[2:0]` (RO)
    ///
    /// Set this bit to control validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid.
    #[inline(always)]
    pub fn km_huk_gen_state_high(&self) -> u32 {
        (self.value.read_bits() & Self::KM_HUK_GEN_STATE_HIGH_MASK)
            >> Self::KM_HUK_GEN_STATE_HIGH_OFFSET
    }
}

/// RD_REPEAT_DATA1 value type for batch read/modify/write operations
///
/// Use `rdrepeatdata1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdrepeatdata1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdRepeatData1Val {
    bits: u32,
}

impl RdRepeatData1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY_PURPOSE_1** `[31:28]`
    ///
    /// Represents the purpose of Key1.
    #[inline(always)]
    pub const fn key_purpose_1(&self) -> u32 {
        (self.bits & RdRepeatData1::KEY_PURPOSE_1_MASK) >> RdRepeatData1::KEY_PURPOSE_1_OFFSET
    }

    /// Read **KEY_PURPOSE_0** `[27:24]`
    ///
    /// Represents the purpose of Key0.
    #[inline(always)]
    pub const fn key_purpose_0(&self) -> u32 {
        (self.bits & RdRepeatData1::KEY_PURPOSE_0_MASK) >> RdRepeatData1::KEY_PURPOSE_0_OFFSET
    }

    /// Read **SECURE_BOOT_KEY_REVOKE2** `[23:23]`
    ///
    /// Represents whether revoking third secure boot key is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub const fn secure_boot_key_revoke2(&self) -> bool {
        (self.bits & RdRepeatData1::SECURE_BOOT_KEY_REVOKE2_MASK) != 0
    }

    /// Read **SECURE_BOOT_KEY_REVOKE1** `[22:22]`
    ///
    /// Represents whether revoking second secure boot key is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub const fn secure_boot_key_revoke1(&self) -> bool {
        (self.bits & RdRepeatData1::SECURE_BOOT_KEY_REVOKE1_MASK) != 0
    }

    /// Read **SECURE_BOOT_KEY_REVOKE0** `[21:21]`
    ///
    /// Represents whether revoking first secure boot key is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub const fn secure_boot_key_revoke0(&self) -> bool {
        (self.bits & RdRepeatData1::SECURE_BOOT_KEY_REVOKE0_MASK) != 0
    }

    /// Read **SPI_BOOT_CRYPT_CNT** `[20:18]`
    ///
    /// Represents whether SPI boot encrypt/decrypt is disabled or enabled. Odd number of 1: enabled. Even number of 1: disabled.
    #[inline(always)]
    pub const fn spi_boot_crypt_cnt(&self) -> u32 {
        (self.bits & RdRepeatData1::SPI_BOOT_CRYPT_CNT_MASK)
            >> RdRepeatData1::SPI_BOOT_CRYPT_CNT_OFFSET
    }

    /// Read **WDT_DELAY_SEL** `[17:16]`
    ///
    /// Represents whether RTC watchdog timeout threshold is selected at startup. 1: selected. 0: not selected.
    #[inline(always)]
    pub const fn wdt_delay_sel(&self) -> u32 {
        (self.bits & RdRepeatData1::WDT_DELAY_SEL_MASK) >> RdRepeatData1::WDT_DELAY_SEL_OFFSET
    }

    /// Read **XTS_KEY_LENGTH_256** `[14:14]`
    ///
    /// Set this bit to configure flash encryption use xts-128 key, else use xts-256 key.
    #[inline(always)]
    pub const fn xts_key_length_256(&self) -> bool {
        (self.bits & RdRepeatData1::XTS_KEY_LENGTH_256_MASK) != 0
    }

    /// Read **FORCE_DISABLE_SW_INIT_KEY** `[13:13]`
    ///
    /// Set this bit to disable software written init key, and force use efuse_init_key.
    #[inline(always)]
    pub const fn force_disable_sw_init_key(&self) -> bool {
        (self.bits & RdRepeatData1::FORCE_DISABLE_SW_INIT_KEY_MASK) != 0
    }

    /// Read **FORCE_USE_KEY_MANAGER_KEY** `[12:9]`
    ///
    /// Set each bit to control whether corresponding key must come from key manager.. 1 is true, 0 is false. Bit0: ecdsa. Bit1: xts. Bit2: hmac. Bit3: ds.
    #[inline(always)]
    pub const fn force_use_key_manager_key(&self) -> u32 {
        (self.bits & RdRepeatData1::FORCE_USE_KEY_MANAGER_KEY_MASK)
            >> RdRepeatData1::FORCE_USE_KEY_MANAGER_KEY_OFFSET
    }

    /// Read **KM_DEPLOY_ONLY_ONCE** `[8:5]`
    ///
    /// Set each bit to control whether corresponding key can only be deployed once. 1 is true, 0 is false. Bit0: ecdsa. Bit1: xts. Bit2: hmac. Bit3: ds.
    #[inline(always)]
    pub const fn km_deploy_only_once(&self) -> u32 {
        (self.bits & RdRepeatData1::KM_DEPLOY_ONLY_ONCE_MASK)
            >> RdRepeatData1::KM_DEPLOY_ONLY_ONCE_OFFSET
    }

    /// Read **KM_RND_SWITCH_CYCLE** `[4:3]`
    ///
    /// Set bits to control key manager random number switch cycle. 0: control by register. 1: 8 km clk cycles. 2: 16 km cycles. 3: 32 km cycles.
    #[inline(always)]
    pub const fn km_rnd_switch_cycle(&self) -> u32 {
        (self.bits & RdRepeatData1::KM_RND_SWITCH_CYCLE_MASK)
            >> RdRepeatData1::KM_RND_SWITCH_CYCLE_OFFSET
    }

    /// Read **KM_HUK_GEN_STATE_HIGH** `[2:0]`
    ///
    /// Set this bit to control validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid.
    #[inline(always)]
    pub const fn km_huk_gen_state_high(&self) -> u32 {
        (self.bits & RdRepeatData1::KM_HUK_GEN_STATE_HIGH_MASK)
            >> RdRepeatData1::KM_HUK_GEN_STATE_HIGH_OFFSET
    }
}

impl Default for RdRepeatData1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdRepeatData1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdRepeatData1Val {
        RdRepeatData1Val::from_bits(self.value.read_bits())
    }
}

/// RD_REPEAT_DATA2 Register
///
/// BLOCK0 data register 3.
///
/// Address offset: 0x0038
/// Reset value: 0x00080000
///
/// ## Fields
///
/// - **FLASH_TPUW** `[31:28]` (RO)
///
///   Represents the flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the programmed value. Otherwise, the waiting time is 2 times the programmed value.
///
/// - **DIS_USB_OTG_DOWNLOAD_MODE** `[27]` (RO)
///
///   Set this bit to disable download via USB-OTG.
///
/// - **FLASH_ECC_EN** `[26]` (RO)
///
///   Set this bit to enable ecc for flash boot.
///
/// - **FLASH_PAGE_SIZE** `[25:24]` (RO)
///
///   Set flash page size.
///
/// - **FLASH_TYPE** `[23]` (RO)
///
///   The type of interfaced flash. 0: four data lines, 1: eight data lines.
///
/// - **SECURE_BOOT_AGGRESSIVE_REVOKE** `[21]` (RO)
///
///   Represents whether revoking aggressive secure boot is enabled or disabled. 1: enabled. 0: disabled.
///
/// - **SECURE_BOOT_EN** `[20]` (RO)
///
///   Represents whether secure boot is enabled or disabled. 1: enabled. 0: disabled.
///
/// - **CRYPT_DPA_ENABLE** `[19]` (RO)
///
///   Represents whether anti-dpa attack is enabled. 1:enabled. 0: disabled.
///
/// - **ECDSA_ENABLE_SOFT_K** `[18]` (RO)
///
///   Represents whether hardware random number k is forced used in ESDCA. 1: force used. 0: not force used.
///
/// - **SEC_DPA_LEVEL** `[17:16]` (RO)
///
///   Represents the spa secure level by configuring the clock random divide mode.
///
/// - **KEY_PURPOSE_5** `[15:12]` (RO)
///
///   Represents the purpose of Key5.
///
/// - **KEY_PURPOSE_4** `[11:8]` (RO)
///
///   Represents the purpose of Key4.
///
/// - **KEY_PURPOSE_3** `[7:4]` (RO)
///
///   Represents the purpose of Key3.
///
/// - **KEY_PURPOSE_2** `[3:0]` (RO)
///
///   Represents the purpose of Key2.
#[repr(transparent)]
pub struct RdRepeatData2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdRepeatData2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdRepeatData2 {
    // ======== Field Constants ========

    /// **FLASH_TPUW**
    ///
    /// Represents the flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the programmed value. Otherwise, the waiting time is 2 times the programmed value.
    ///
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const FLASH_TPUW_OFFSET: u32 = 28;
    pub const FLASH_TPUW_MASK: u32 = 0xf0000000;

    /// **DIS_USB_OTG_DOWNLOAD_MODE**
    ///
    /// Set this bit to disable download via USB-OTG.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const DIS_USB_OTG_DOWNLOAD_MODE_OFFSET: u32 = 27;
    pub const DIS_USB_OTG_DOWNLOAD_MODE_MASK: u32 = 0x08000000;
    pub const DIS_USB_OTG_DOWNLOAD_MODE: u32 = 1 << 27;

    /// **FLASH_ECC_EN**
    ///
    /// Set this bit to enable ecc for flash boot.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const FLASH_ECC_EN_OFFSET: u32 = 26;
    pub const FLASH_ECC_EN_MASK: u32 = 0x04000000;
    pub const FLASH_ECC_EN: u32 = 1 << 26;

    /// **FLASH_PAGE_SIZE**
    ///
    /// Set flash page size.
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const FLASH_PAGE_SIZE_OFFSET: u32 = 24;
    pub const FLASH_PAGE_SIZE_MASK: u32 = 0x03000000;

    /// **FLASH_TYPE**
    ///
    /// The type of interfaced flash. 0: four data lines, 1: eight data lines.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const FLASH_TYPE_OFFSET: u32 = 23;
    pub const FLASH_TYPE_MASK: u32 = 0x00800000;
    pub const FLASH_TYPE: u32 = 1 << 23;

    /// **SECURE_BOOT_AGGRESSIVE_REVOKE**
    ///
    /// Represents whether revoking aggressive secure boot is enabled or disabled. 1: enabled. 0: disabled.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SECURE_BOOT_AGGRESSIVE_REVOKE_OFFSET: u32 = 21;
    pub const SECURE_BOOT_AGGRESSIVE_REVOKE_MASK: u32 = 0x00200000;
    pub const SECURE_BOOT_AGGRESSIVE_REVOKE: u32 = 1 << 21;

    /// **SECURE_BOOT_EN**
    ///
    /// Represents whether secure boot is enabled or disabled. 1: enabled. 0: disabled.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SECURE_BOOT_EN_OFFSET: u32 = 20;
    pub const SECURE_BOOT_EN_MASK: u32 = 0x00100000;
    pub const SECURE_BOOT_EN: u32 = 1 << 20;

    /// **CRYPT_DPA_ENABLE**
    ///
    /// Represents whether anti-dpa attack is enabled. 1:enabled. 0: disabled.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CRYPT_DPA_ENABLE_OFFSET: u32 = 19;
    pub const CRYPT_DPA_ENABLE_MASK: u32 = 0x00080000;
    pub const CRYPT_DPA_ENABLE: u32 = 1 << 19;

    /// **ECDSA_ENABLE_SOFT_K**
    ///
    /// Represents whether hardware random number k is forced used in ESDCA. 1: force used. 0: not force used.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const ECDSA_ENABLE_SOFT_K_OFFSET: u32 = 18;
    pub const ECDSA_ENABLE_SOFT_K_MASK: u32 = 0x00040000;
    pub const ECDSA_ENABLE_SOFT_K: u32 = 1 << 18;

    /// **SEC_DPA_LEVEL**
    ///
    /// Represents the spa secure level by configuring the clock random divide mode.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const SEC_DPA_LEVEL_OFFSET: u32 = 16;
    pub const SEC_DPA_LEVEL_MASK: u32 = 0x00030000;

    /// **KEY_PURPOSE_5**
    ///
    /// Represents the purpose of Key5.
    ///
    /// Bits `[15:12]`, mask `0x0000f000`
    pub const KEY_PURPOSE_5_OFFSET: u32 = 12;
    pub const KEY_PURPOSE_5_MASK: u32 = 0x0000f000;

    /// **KEY_PURPOSE_4**
    ///
    /// Represents the purpose of Key4.
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const KEY_PURPOSE_4_OFFSET: u32 = 8;
    pub const KEY_PURPOSE_4_MASK: u32 = 0x00000f00;

    /// **KEY_PURPOSE_3**
    ///
    /// Represents the purpose of Key3.
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const KEY_PURPOSE_3_OFFSET: u32 = 4;
    pub const KEY_PURPOSE_3_MASK: u32 = 0x000000f0;

    /// **KEY_PURPOSE_2**
    ///
    /// Represents the purpose of Key2.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const KEY_PURPOSE_2_OFFSET: u32 = 0;
    pub const KEY_PURPOSE_2_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **FLASH_TPUW** `[31:28]` (RO)
    ///
    /// Represents the flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the programmed value. Otherwise, the waiting time is 2 times the programmed value.
    #[inline(always)]
    pub fn flash_tpuw(&self) -> u32 {
        (self.value.read_bits() & Self::FLASH_TPUW_MASK) >> Self::FLASH_TPUW_OFFSET
    }

    /// **DIS_USB_OTG_DOWNLOAD_MODE** `[27]` (RO)
    ///
    /// Set this bit to disable download via USB-OTG.
    #[inline(always)]
    pub fn dis_usb_otg_download_mode(&self) -> bool {
        (self.value.read_bits() & Self::DIS_USB_OTG_DOWNLOAD_MODE_MASK) != 0
    }

    /// **FLASH_ECC_EN** `[26]` (RO)
    ///
    /// Set this bit to enable ecc for flash boot.
    #[inline(always)]
    pub fn flash_ecc_en(&self) -> bool {
        (self.value.read_bits() & Self::FLASH_ECC_EN_MASK) != 0
    }

    /// **FLASH_PAGE_SIZE** `[25:24]` (RO)
    ///
    /// Set flash page size.
    #[inline(always)]
    pub fn flash_page_size(&self) -> u32 {
        (self.value.read_bits() & Self::FLASH_PAGE_SIZE_MASK) >> Self::FLASH_PAGE_SIZE_OFFSET
    }

    /// **FLASH_TYPE** `[23]` (RO)
    ///
    /// The type of interfaced flash. 0: four data lines, 1: eight data lines.
    #[inline(always)]
    pub fn flash_type(&self) -> bool {
        (self.value.read_bits() & Self::FLASH_TYPE_MASK) != 0
    }

    /// **SECURE_BOOT_AGGRESSIVE_REVOKE** `[21]` (RO)
    ///
    /// Represents whether revoking aggressive secure boot is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&self) -> bool {
        (self.value.read_bits() & Self::SECURE_BOOT_AGGRESSIVE_REVOKE_MASK) != 0
    }

    /// **SECURE_BOOT_EN** `[20]` (RO)
    ///
    /// Represents whether secure boot is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub fn secure_boot_en(&self) -> bool {
        (self.value.read_bits() & Self::SECURE_BOOT_EN_MASK) != 0
    }

    /// **CRYPT_DPA_ENABLE** `[19]` (RO)
    ///
    /// Represents whether anti-dpa attack is enabled. 1:enabled. 0: disabled.
    #[inline(always)]
    pub fn crypt_dpa_enable(&self) -> bool {
        (self.value.read_bits() & Self::CRYPT_DPA_ENABLE_MASK) != 0
    }

    /// **ECDSA_ENABLE_SOFT_K** `[18]` (RO)
    ///
    /// Represents whether hardware random number k is forced used in ESDCA. 1: force used. 0: not force used.
    #[inline(always)]
    pub fn ecdsa_enable_soft_k(&self) -> bool {
        (self.value.read_bits() & Self::ECDSA_ENABLE_SOFT_K_MASK) != 0
    }

    /// **SEC_DPA_LEVEL** `[17:16]` (RO)
    ///
    /// Represents the spa secure level by configuring the clock random divide mode.
    #[inline(always)]
    pub fn sec_dpa_level(&self) -> u32 {
        (self.value.read_bits() & Self::SEC_DPA_LEVEL_MASK) >> Self::SEC_DPA_LEVEL_OFFSET
    }

    /// **KEY_PURPOSE_5** `[15:12]` (RO)
    ///
    /// Represents the purpose of Key5.
    #[inline(always)]
    pub fn key_purpose_5(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_PURPOSE_5_MASK) >> Self::KEY_PURPOSE_5_OFFSET
    }

    /// **KEY_PURPOSE_4** `[11:8]` (RO)
    ///
    /// Represents the purpose of Key4.
    #[inline(always)]
    pub fn key_purpose_4(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_PURPOSE_4_MASK) >> Self::KEY_PURPOSE_4_OFFSET
    }

    /// **KEY_PURPOSE_3** `[7:4]` (RO)
    ///
    /// Represents the purpose of Key3.
    #[inline(always)]
    pub fn key_purpose_3(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_PURPOSE_3_MASK) >> Self::KEY_PURPOSE_3_OFFSET
    }

    /// **KEY_PURPOSE_2** `[3:0]` (RO)
    ///
    /// Represents the purpose of Key2.
    #[inline(always)]
    pub fn key_purpose_2(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_PURPOSE_2_MASK) >> Self::KEY_PURPOSE_2_OFFSET
    }
}

/// RD_REPEAT_DATA2 value type for batch read/modify/write operations
///
/// Use `rdrepeatdata2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdrepeatdata2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdRepeatData2Val {
    bits: u32,
}

impl RdRepeatData2Val {
    /// Create from raw bits
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
        Self { bits: 0x00080000 }
    }

    /// Read **FLASH_TPUW** `[31:28]`
    ///
    /// Represents the flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the programmed value. Otherwise, the waiting time is 2 times the programmed value.
    #[inline(always)]
    pub const fn flash_tpuw(&self) -> u32 {
        (self.bits & RdRepeatData2::FLASH_TPUW_MASK) >> RdRepeatData2::FLASH_TPUW_OFFSET
    }

    /// Read **DIS_USB_OTG_DOWNLOAD_MODE** `[27:27]`
    ///
    /// Set this bit to disable download via USB-OTG.
    #[inline(always)]
    pub const fn dis_usb_otg_download_mode(&self) -> bool {
        (self.bits & RdRepeatData2::DIS_USB_OTG_DOWNLOAD_MODE_MASK) != 0
    }

    /// Read **FLASH_ECC_EN** `[26:26]`
    ///
    /// Set this bit to enable ecc for flash boot.
    #[inline(always)]
    pub const fn flash_ecc_en(&self) -> bool {
        (self.bits & RdRepeatData2::FLASH_ECC_EN_MASK) != 0
    }

    /// Read **FLASH_PAGE_SIZE** `[25:24]`
    ///
    /// Set flash page size.
    #[inline(always)]
    pub const fn flash_page_size(&self) -> u32 {
        (self.bits & RdRepeatData2::FLASH_PAGE_SIZE_MASK) >> RdRepeatData2::FLASH_PAGE_SIZE_OFFSET
    }

    /// Read **FLASH_TYPE** `[23:23]`
    ///
    /// The type of interfaced flash. 0: four data lines, 1: eight data lines.
    #[inline(always)]
    pub const fn flash_type(&self) -> bool {
        (self.bits & RdRepeatData2::FLASH_TYPE_MASK) != 0
    }

    /// Read **SECURE_BOOT_AGGRESSIVE_REVOKE** `[21:21]`
    ///
    /// Represents whether revoking aggressive secure boot is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub const fn secure_boot_aggressive_revoke(&self) -> bool {
        (self.bits & RdRepeatData2::SECURE_BOOT_AGGRESSIVE_REVOKE_MASK) != 0
    }

    /// Read **SECURE_BOOT_EN** `[20:20]`
    ///
    /// Represents whether secure boot is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub const fn secure_boot_en(&self) -> bool {
        (self.bits & RdRepeatData2::SECURE_BOOT_EN_MASK) != 0
    }

    /// Read **CRYPT_DPA_ENABLE** `[19:19]`
    ///
    /// Represents whether anti-dpa attack is enabled. 1:enabled. 0: disabled.
    #[inline(always)]
    pub const fn crypt_dpa_enable(&self) -> bool {
        (self.bits & RdRepeatData2::CRYPT_DPA_ENABLE_MASK) != 0
    }

    /// Read **ECDSA_ENABLE_SOFT_K** `[18:18]`
    ///
    /// Represents whether hardware random number k is forced used in ESDCA. 1: force used. 0: not force used.
    #[inline(always)]
    pub const fn ecdsa_enable_soft_k(&self) -> bool {
        (self.bits & RdRepeatData2::ECDSA_ENABLE_SOFT_K_MASK) != 0
    }

    /// Read **SEC_DPA_LEVEL** `[17:16]`
    ///
    /// Represents the spa secure level by configuring the clock random divide mode.
    #[inline(always)]
    pub const fn sec_dpa_level(&self) -> u32 {
        (self.bits & RdRepeatData2::SEC_DPA_LEVEL_MASK) >> RdRepeatData2::SEC_DPA_LEVEL_OFFSET
    }

    /// Read **KEY_PURPOSE_5** `[15:12]`
    ///
    /// Represents the purpose of Key5.
    #[inline(always)]
    pub const fn key_purpose_5(&self) -> u32 {
        (self.bits & RdRepeatData2::KEY_PURPOSE_5_MASK) >> RdRepeatData2::KEY_PURPOSE_5_OFFSET
    }

    /// Read **KEY_PURPOSE_4** `[11:8]`
    ///
    /// Represents the purpose of Key4.
    #[inline(always)]
    pub const fn key_purpose_4(&self) -> u32 {
        (self.bits & RdRepeatData2::KEY_PURPOSE_4_MASK) >> RdRepeatData2::KEY_PURPOSE_4_OFFSET
    }

    /// Read **KEY_PURPOSE_3** `[7:4]`
    ///
    /// Represents the purpose of Key3.
    #[inline(always)]
    pub const fn key_purpose_3(&self) -> u32 {
        (self.bits & RdRepeatData2::KEY_PURPOSE_3_MASK) >> RdRepeatData2::KEY_PURPOSE_3_OFFSET
    }

    /// Read **KEY_PURPOSE_2** `[3:0]`
    ///
    /// Represents the purpose of Key2.
    #[inline(always)]
    pub const fn key_purpose_2(&self) -> u32 {
        (self.bits & RdRepeatData2::KEY_PURPOSE_2_MASK) >> RdRepeatData2::KEY_PURPOSE_2_OFFSET
    }
}

impl Default for RdRepeatData2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RdRepeatData2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdRepeatData2Val {
        RdRepeatData2Val::from_bits(self.value.read_bits())
    }
}

/// RD_REPEAT_DATA3 Register
///
/// BLOCK0 data register 4.
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **DCDC_VSET** `[31:27]` (RO)
///
///   Set the dcdc voltage default.
///
/// - **HYS_EN_PAD** `[26]` (RO)
///
///   Represents whether the hysteresis function of corresponding PAD is enabled. 1: enabled. 0:disabled.
///
/// - **SECURE_BOOT_DISABLE_FAST_WAKE** `[25]` (RO)
///
///   Represents whether FAST VERIFY ON WAKE is disabled or enabled when Secure Boot is enabled. 1: disabled. 0: enabled.
///
/// - **SECURE_VERSION** `[24:9]` (RO)
///
///   Represents the version used by ESP-IDF anti-rollback feature.
///
/// - **FORCE_SEND_RESUME** `[8]` (RO)
///
///   Represents whether ROM code is forced to send a resume command during SPI boot. 1: forced. 0:not forced.
///
/// - **UART_PRINT_CONTROL** `[7:6]` (RO)
///
///   Represents the type of UART printing. 00: force enable printing. 01: enable printing when GPIO8 is reset at low level. 10: enable printing when GPIO8 is reset at high level. 11: force disable printing.
///
/// - **ENABLE_SECURITY_DOWNLOAD** `[5]` (RO)
///
///   Represents whether security download is enabled or disabled. 1: enabled. 0: disabled.
///
/// - **DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE** `[4]` (RO)
///
///   Represents whether the USB-Serial-JTAG download function is disabled or enabled. 1: disabled. 0: enabled.
///
/// - **LOCK_KM_KEY** `[3]` (RO)
///
///   TBD
///
/// - **DIS_USB_SERIAL_JTAG_ROM_PRINT** `[2]` (RO)
///
///   Represents whether print from USB-Serial-JTAG is disabled or enabled. 1: disabled. 0: enabled.
///
/// - **DIS_DIRECT_BOOT** `[1]` (RO)
///
///   Represents whether direct boot mode is disabled or enabled. 1: disabled. 0: enabled.
///
/// - **DIS_DOWNLOAD_MODE** `[0]` (RO)
///
///   Represents whether Download mode is disabled or enabled. 1: disabled. 0: enabled.
#[repr(transparent)]
pub struct RdRepeatData3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdRepeatData3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdRepeatData3 {
    // ======== Field Constants ========

    /// **DCDC_VSET**
    ///
    /// Set the dcdc voltage default.
    ///
    /// Bits `[31:27]`, mask `0xf8000000`
    pub const DCDC_VSET_OFFSET: u32 = 27;
    pub const DCDC_VSET_MASK: u32 = 0xf8000000;

    /// **HYS_EN_PAD**
    ///
    /// Represents whether the hysteresis function of corresponding PAD is enabled. 1: enabled. 0:disabled.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const HYS_EN_PAD_OFFSET: u32 = 26;
    pub const HYS_EN_PAD_MASK: u32 = 0x04000000;
    pub const HYS_EN_PAD: u32 = 1 << 26;

    /// **SECURE_BOOT_DISABLE_FAST_WAKE**
    ///
    /// Represents whether FAST VERIFY ON WAKE is disabled or enabled when Secure Boot is enabled. 1: disabled. 0: enabled.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const SECURE_BOOT_DISABLE_FAST_WAKE_OFFSET: u32 = 25;
    pub const SECURE_BOOT_DISABLE_FAST_WAKE_MASK: u32 = 0x02000000;
    pub const SECURE_BOOT_DISABLE_FAST_WAKE: u32 = 1 << 25;

    /// **SECURE_VERSION**
    ///
    /// Represents the version used by ESP-IDF anti-rollback feature.
    ///
    /// Bits `[24:9]`, mask `0x01fffe00`
    pub const SECURE_VERSION_OFFSET: u32 = 9;
    pub const SECURE_VERSION_MASK: u32 = 0x01fffe00;

    /// **FORCE_SEND_RESUME**
    ///
    /// Represents whether ROM code is forced to send a resume command during SPI boot. 1: forced. 0:not forced.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const FORCE_SEND_RESUME_OFFSET: u32 = 8;
    pub const FORCE_SEND_RESUME_MASK: u32 = 0x00000100;
    pub const FORCE_SEND_RESUME: u32 = 1 << 8;

    /// **UART_PRINT_CONTROL**
    ///
    /// Represents the type of UART printing. 00: force enable printing. 01: enable printing when GPIO8 is reset at low level. 10: enable printing when GPIO8 is reset at high level. 11: force disable printing.
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const UART_PRINT_CONTROL_OFFSET: u32 = 6;
    pub const UART_PRINT_CONTROL_MASK: u32 = 0x000000c0;

    /// **ENABLE_SECURITY_DOWNLOAD**
    ///
    /// Represents whether security download is enabled or disabled. 1: enabled. 0: disabled.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ENABLE_SECURITY_DOWNLOAD_OFFSET: u32 = 5;
    pub const ENABLE_SECURITY_DOWNLOAD_MASK: u32 = 0x00000020;
    pub const ENABLE_SECURITY_DOWNLOAD: u32 = 1 << 5;

    /// **DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE**
    ///
    /// Represents whether the USB-Serial-JTAG download function is disabled or enabled. 1: disabled. 0: enabled.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_OFFSET: u32 = 4;
    pub const DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_MASK: u32 = 0x00000010;
    pub const DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE: u32 = 1 << 4;

    /// **LOCK_KM_KEY**
    ///
    /// TBD
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LOCK_KM_KEY_OFFSET: u32 = 3;
    pub const LOCK_KM_KEY_MASK: u32 = 0x00000008;
    pub const LOCK_KM_KEY: u32 = 1 << 3;

    /// **DIS_USB_SERIAL_JTAG_ROM_PRINT**
    ///
    /// Represents whether print from USB-Serial-JTAG is disabled or enabled. 1: disabled. 0: enabled.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DIS_USB_SERIAL_JTAG_ROM_PRINT_OFFSET: u32 = 2;
    pub const DIS_USB_SERIAL_JTAG_ROM_PRINT_MASK: u32 = 0x00000004;
    pub const DIS_USB_SERIAL_JTAG_ROM_PRINT: u32 = 1 << 2;

    /// **DIS_DIRECT_BOOT**
    ///
    /// Represents whether direct boot mode is disabled or enabled. 1: disabled. 0: enabled.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DIS_DIRECT_BOOT_OFFSET: u32 = 1;
    pub const DIS_DIRECT_BOOT_MASK: u32 = 0x00000002;
    pub const DIS_DIRECT_BOOT: u32 = 1 << 1;

    /// **DIS_DOWNLOAD_MODE**
    ///
    /// Represents whether Download mode is disabled or enabled. 1: disabled. 0: enabled.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DIS_DOWNLOAD_MODE_OFFSET: u32 = 0;
    pub const DIS_DOWNLOAD_MODE_MASK: u32 = 0x00000001;
    pub const DIS_DOWNLOAD_MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DCDC_VSET** `[31:27]` (RO)
    ///
    /// Set the dcdc voltage default.
    #[inline(always)]
    pub fn dcdc_vset(&self) -> u32 {
        (self.value.read_bits() & Self::DCDC_VSET_MASK) >> Self::DCDC_VSET_OFFSET
    }

    /// **HYS_EN_PAD** `[26]` (RO)
    ///
    /// Represents whether the hysteresis function of corresponding PAD is enabled. 1: enabled. 0:disabled.
    #[inline(always)]
    pub fn hys_en_pad(&self) -> bool {
        (self.value.read_bits() & Self::HYS_EN_PAD_MASK) != 0
    }

    /// **SECURE_BOOT_DISABLE_FAST_WAKE** `[25]` (RO)
    ///
    /// Represents whether FAST VERIFY ON WAKE is disabled or enabled when Secure Boot is enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn secure_boot_disable_fast_wake(&self) -> bool {
        (self.value.read_bits() & Self::SECURE_BOOT_DISABLE_FAST_WAKE_MASK) != 0
    }

    /// **SECURE_VERSION** `[24:9]` (RO)
    ///
    /// Represents the version used by ESP-IDF anti-rollback feature.
    #[inline(always)]
    pub fn secure_version(&self) -> u32 {
        (self.value.read_bits() & Self::SECURE_VERSION_MASK) >> Self::SECURE_VERSION_OFFSET
    }

    /// **FORCE_SEND_RESUME** `[8]` (RO)
    ///
    /// Represents whether ROM code is forced to send a resume command during SPI boot. 1: forced. 0:not forced.
    #[inline(always)]
    pub fn force_send_resume(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_SEND_RESUME_MASK) != 0
    }

    /// **UART_PRINT_CONTROL** `[7:6]` (RO)
    ///
    /// Represents the type of UART printing. 00: force enable printing. 01: enable printing when GPIO8 is reset at low level. 10: enable printing when GPIO8 is reset at high level. 11: force disable printing.
    #[inline(always)]
    pub fn uart_print_control(&self) -> u32 {
        (self.value.read_bits() & Self::UART_PRINT_CONTROL_MASK) >> Self::UART_PRINT_CONTROL_OFFSET
    }

    /// **ENABLE_SECURITY_DOWNLOAD** `[5]` (RO)
    ///
    /// Represents whether security download is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub fn enable_security_download(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_SECURITY_DOWNLOAD_MASK) != 0
    }

    /// **DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE** `[4]` (RO)
    ///
    /// Represents whether the USB-Serial-JTAG download function is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn dis_usb_serial_jtag_download_mode(&self) -> bool {
        (self.value.read_bits() & Self::DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_MASK) != 0
    }

    /// **LOCK_KM_KEY** `[3]` (RO)
    ///
    /// TBD
    #[inline(always)]
    pub fn lock_km_key(&self) -> bool {
        (self.value.read_bits() & Self::LOCK_KM_KEY_MASK) != 0
    }

    /// **DIS_USB_SERIAL_JTAG_ROM_PRINT** `[2]` (RO)
    ///
    /// Represents whether print from USB-Serial-JTAG is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn dis_usb_serial_jtag_rom_print(&self) -> bool {
        (self.value.read_bits() & Self::DIS_USB_SERIAL_JTAG_ROM_PRINT_MASK) != 0
    }

    /// **DIS_DIRECT_BOOT** `[1]` (RO)
    ///
    /// Represents whether direct boot mode is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn dis_direct_boot(&self) -> bool {
        (self.value.read_bits() & Self::DIS_DIRECT_BOOT_MASK) != 0
    }

    /// **DIS_DOWNLOAD_MODE** `[0]` (RO)
    ///
    /// Represents whether Download mode is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn dis_download_mode(&self) -> bool {
        (self.value.read_bits() & Self::DIS_DOWNLOAD_MODE_MASK) != 0
    }
}

/// RD_REPEAT_DATA3 value type for batch read/modify/write operations
///
/// Use `rdrepeatdata3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdrepeatdata3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdRepeatData3Val {
    bits: u32,
}

impl RdRepeatData3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCDC_VSET** `[31:27]`
    ///
    /// Set the dcdc voltage default.
    #[inline(always)]
    pub const fn dcdc_vset(&self) -> u32 {
        (self.bits & RdRepeatData3::DCDC_VSET_MASK) >> RdRepeatData3::DCDC_VSET_OFFSET
    }

    /// Read **HYS_EN_PAD** `[26:26]`
    ///
    /// Represents whether the hysteresis function of corresponding PAD is enabled. 1: enabled. 0:disabled.
    #[inline(always)]
    pub const fn hys_en_pad(&self) -> bool {
        (self.bits & RdRepeatData3::HYS_EN_PAD_MASK) != 0
    }

    /// Read **SECURE_BOOT_DISABLE_FAST_WAKE** `[25:25]`
    ///
    /// Represents whether FAST VERIFY ON WAKE is disabled or enabled when Secure Boot is enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub const fn secure_boot_disable_fast_wake(&self) -> bool {
        (self.bits & RdRepeatData3::SECURE_BOOT_DISABLE_FAST_WAKE_MASK) != 0
    }

    /// Read **SECURE_VERSION** `[24:9]`
    ///
    /// Represents the version used by ESP-IDF anti-rollback feature.
    #[inline(always)]
    pub const fn secure_version(&self) -> u32 {
        (self.bits & RdRepeatData3::SECURE_VERSION_MASK) >> RdRepeatData3::SECURE_VERSION_OFFSET
    }

    /// Read **FORCE_SEND_RESUME** `[8:8]`
    ///
    /// Represents whether ROM code is forced to send a resume command during SPI boot. 1: forced. 0:not forced.
    #[inline(always)]
    pub const fn force_send_resume(&self) -> bool {
        (self.bits & RdRepeatData3::FORCE_SEND_RESUME_MASK) != 0
    }

    /// Read **UART_PRINT_CONTROL** `[7:6]`
    ///
    /// Represents the type of UART printing. 00: force enable printing. 01: enable printing when GPIO8 is reset at low level. 10: enable printing when GPIO8 is reset at high level. 11: force disable printing.
    #[inline(always)]
    pub const fn uart_print_control(&self) -> u32 {
        (self.bits & RdRepeatData3::UART_PRINT_CONTROL_MASK)
            >> RdRepeatData3::UART_PRINT_CONTROL_OFFSET
    }

    /// Read **ENABLE_SECURITY_DOWNLOAD** `[5:5]`
    ///
    /// Represents whether security download is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub const fn enable_security_download(&self) -> bool {
        (self.bits & RdRepeatData3::ENABLE_SECURITY_DOWNLOAD_MASK) != 0
    }

    /// Read **DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE** `[4:4]`
    ///
    /// Represents whether the USB-Serial-JTAG download function is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub const fn dis_usb_serial_jtag_download_mode(&self) -> bool {
        (self.bits & RdRepeatData3::DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_MASK) != 0
    }

    /// Read **LOCK_KM_KEY** `[3:3]`
    ///
    /// TBD
    #[inline(always)]
    pub const fn lock_km_key(&self) -> bool {
        (self.bits & RdRepeatData3::LOCK_KM_KEY_MASK) != 0
    }

    /// Read **DIS_USB_SERIAL_JTAG_ROM_PRINT** `[2:2]`
    ///
    /// Represents whether print from USB-Serial-JTAG is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub const fn dis_usb_serial_jtag_rom_print(&self) -> bool {
        (self.bits & RdRepeatData3::DIS_USB_SERIAL_JTAG_ROM_PRINT_MASK) != 0
    }

    /// Read **DIS_DIRECT_BOOT** `[1:1]`
    ///
    /// Represents whether direct boot mode is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub const fn dis_direct_boot(&self) -> bool {
        (self.bits & RdRepeatData3::DIS_DIRECT_BOOT_MASK) != 0
    }

    /// Read **DIS_DOWNLOAD_MODE** `[0:0]`
    ///
    /// Represents whether Download mode is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub const fn dis_download_mode(&self) -> bool {
        (self.bits & RdRepeatData3::DIS_DOWNLOAD_MODE_MASK) != 0
    }
}

impl Default for RdRepeatData3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdRepeatData3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdRepeatData3Val {
        RdRepeatData3Val::from_bits(self.value.read_bits())
    }
}

/// RD_REPEAT_DATA4 Register
///
/// BLOCK0 data register 5.
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **DIS_SWD** `[21]` (RO)
///
///   Set this bit to disable super-watchdog.
///
/// - **DIS_WDT** `[20]` (RO)
///
///   Set this bit to disable watch dog.
///
/// - **DCDC_VSET_EN** `[19]` (RO)
///
///   Select dcdc vset use efuse_dcdc_vset.
///
/// - **HP_PWR_SRC_SEL** `[18]` (RO)
///
///   HP system power source select. 0:LDO. 1: DCDC.
///
/// - **USB_OTG11_DREFL** `[15:14]` (RO)
///
///   Represents the usb otg11 single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV.
///
/// - **USB_DEVICE_DREFL** `[13:12]` (RO)
///
///   Represents the usb device single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV.
///
/// - **KM_DISABLE_DEPLOY_MODE** `[11:8]` (RO)
///
///   TBD.
///
/// - **_0PXA_TIEH_SEL_3** `[7:6]` (RO)
///
///   TBD.
///
/// - **_0PXA_TIEH_SEL_2** `[5:4]` (RO)
///
///   TBD.
///
/// - **_0PXA_TIEH_SEL_1** `[3:2]` (RO)
///
///   TBD.
///
/// - **_0PXA_TIEH_SEL_0** `[1:0]` (RO)
///
///   TBD
#[repr(transparent)]
pub struct RdRepeatData4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdRepeatData4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdRepeatData4 {
    // ======== Field Constants ========

    /// **DIS_SWD**
    ///
    /// Set this bit to disable super-watchdog.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const DIS_SWD_OFFSET: u32 = 21;
    pub const DIS_SWD_MASK: u32 = 0x00200000;
    pub const DIS_SWD: u32 = 1 << 21;

    /// **DIS_WDT**
    ///
    /// Set this bit to disable watch dog.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const DIS_WDT_OFFSET: u32 = 20;
    pub const DIS_WDT_MASK: u32 = 0x00100000;
    pub const DIS_WDT: u32 = 1 << 20;

    /// **DCDC_VSET_EN**
    ///
    /// Select dcdc vset use efuse_dcdc_vset.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const DCDC_VSET_EN_OFFSET: u32 = 19;
    pub const DCDC_VSET_EN_MASK: u32 = 0x00080000;
    pub const DCDC_VSET_EN: u32 = 1 << 19;

    /// **HP_PWR_SRC_SEL**
    ///
    /// HP system power source select. 0:LDO. 1: DCDC.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const HP_PWR_SRC_SEL_OFFSET: u32 = 18;
    pub const HP_PWR_SRC_SEL_MASK: u32 = 0x00040000;
    pub const HP_PWR_SRC_SEL: u32 = 1 << 18;

    /// **USB_OTG11_DREFL**
    ///
    /// Represents the usb otg11 single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV.
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const USB_OTG11_DREFL_OFFSET: u32 = 14;
    pub const USB_OTG11_DREFL_MASK: u32 = 0x0000c000;

    /// **USB_DEVICE_DREFL**
    ///
    /// Represents the usb device single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV.
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const USB_DEVICE_DREFL_OFFSET: u32 = 12;
    pub const USB_DEVICE_DREFL_MASK: u32 = 0x00003000;

    /// **KM_DISABLE_DEPLOY_MODE**
    ///
    /// TBD.
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const KM_DISABLE_DEPLOY_MODE_OFFSET: u32 = 8;
    pub const KM_DISABLE_DEPLOY_MODE_MASK: u32 = 0x00000f00;

    /// **_0PXA_TIEH_SEL_3**
    ///
    /// TBD.
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const _0PXA_TIEH_SEL_3_OFFSET: u32 = 6;
    pub const _0PXA_TIEH_SEL_3_MASK: u32 = 0x000000c0;

    /// **_0PXA_TIEH_SEL_2**
    ///
    /// TBD.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const _0PXA_TIEH_SEL_2_OFFSET: u32 = 4;
    pub const _0PXA_TIEH_SEL_2_MASK: u32 = 0x00000030;

    /// **_0PXA_TIEH_SEL_1**
    ///
    /// TBD.
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const _0PXA_TIEH_SEL_1_OFFSET: u32 = 2;
    pub const _0PXA_TIEH_SEL_1_MASK: u32 = 0x0000000c;

    /// **_0PXA_TIEH_SEL_0**
    ///
    /// TBD
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const _0PXA_TIEH_SEL_0_OFFSET: u32 = 0;
    pub const _0PXA_TIEH_SEL_0_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **DIS_SWD** `[21]` (RO)
    ///
    /// Set this bit to disable super-watchdog.
    #[inline(always)]
    pub fn dis_swd(&self) -> bool {
        (self.value.read_bits() & Self::DIS_SWD_MASK) != 0
    }

    /// **DIS_WDT** `[20]` (RO)
    ///
    /// Set this bit to disable watch dog.
    #[inline(always)]
    pub fn dis_wdt(&self) -> bool {
        (self.value.read_bits() & Self::DIS_WDT_MASK) != 0
    }

    /// **DCDC_VSET_EN** `[19]` (RO)
    ///
    /// Select dcdc vset use efuse_dcdc_vset.
    #[inline(always)]
    pub fn dcdc_vset_en(&self) -> bool {
        (self.value.read_bits() & Self::DCDC_VSET_EN_MASK) != 0
    }

    /// **HP_PWR_SRC_SEL** `[18]` (RO)
    ///
    /// HP system power source select. 0:LDO. 1: DCDC.
    #[inline(always)]
    pub fn hp_pwr_src_sel(&self) -> bool {
        (self.value.read_bits() & Self::HP_PWR_SRC_SEL_MASK) != 0
    }

    /// **USB_OTG11_DREFL** `[15:14]` (RO)
    ///
    /// Represents the usb otg11 single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV.
    #[inline(always)]
    pub fn usb_otg11_drefl(&self) -> u32 {
        (self.value.read_bits() & Self::USB_OTG11_DREFL_MASK) >> Self::USB_OTG11_DREFL_OFFSET
    }

    /// **USB_DEVICE_DREFL** `[13:12]` (RO)
    ///
    /// Represents the usb device single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV.
    #[inline(always)]
    pub fn usb_device_drefl(&self) -> u32 {
        (self.value.read_bits() & Self::USB_DEVICE_DREFL_MASK) >> Self::USB_DEVICE_DREFL_OFFSET
    }

    /// **KM_DISABLE_DEPLOY_MODE** `[11:8]` (RO)
    ///
    /// TBD.
    #[inline(always)]
    pub fn km_disable_deploy_mode(&self) -> u32 {
        (self.value.read_bits() & Self::KM_DISABLE_DEPLOY_MODE_MASK)
            >> Self::KM_DISABLE_DEPLOY_MODE_OFFSET
    }

    /// **_0PXA_TIEH_SEL_3** `[7:6]` (RO)
    ///
    /// TBD.
    #[inline(always)]
    pub fn _0pxa_tieh_sel_3(&self) -> u32 {
        (self.value.read_bits() & Self::_0PXA_TIEH_SEL_3_MASK) >> Self::_0PXA_TIEH_SEL_3_OFFSET
    }

    /// **_0PXA_TIEH_SEL_2** `[5:4]` (RO)
    ///
    /// TBD.
    #[inline(always)]
    pub fn _0pxa_tieh_sel_2(&self) -> u32 {
        (self.value.read_bits() & Self::_0PXA_TIEH_SEL_2_MASK) >> Self::_0PXA_TIEH_SEL_2_OFFSET
    }

    /// **_0PXA_TIEH_SEL_1** `[3:2]` (RO)
    ///
    /// TBD.
    #[inline(always)]
    pub fn _0pxa_tieh_sel_1(&self) -> u32 {
        (self.value.read_bits() & Self::_0PXA_TIEH_SEL_1_MASK) >> Self::_0PXA_TIEH_SEL_1_OFFSET
    }

    /// **_0PXA_TIEH_SEL_0** `[1:0]` (RO)
    ///
    /// TBD
    #[inline(always)]
    pub fn _0pxa_tieh_sel_0(&self) -> u32 {
        (self.value.read_bits() & Self::_0PXA_TIEH_SEL_0_MASK) >> Self::_0PXA_TIEH_SEL_0_OFFSET
    }
}

/// RD_REPEAT_DATA4 value type for batch read/modify/write operations
///
/// Use `rdrepeatdata4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdrepeatdata4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdRepeatData4Val {
    bits: u32,
}

impl RdRepeatData4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DIS_SWD** `[21:21]`
    ///
    /// Set this bit to disable super-watchdog.
    #[inline(always)]
    pub const fn dis_swd(&self) -> bool {
        (self.bits & RdRepeatData4::DIS_SWD_MASK) != 0
    }

    /// Read **DIS_WDT** `[20:20]`
    ///
    /// Set this bit to disable watch dog.
    #[inline(always)]
    pub const fn dis_wdt(&self) -> bool {
        (self.bits & RdRepeatData4::DIS_WDT_MASK) != 0
    }

    /// Read **DCDC_VSET_EN** `[19:19]`
    ///
    /// Select dcdc vset use efuse_dcdc_vset.
    #[inline(always)]
    pub const fn dcdc_vset_en(&self) -> bool {
        (self.bits & RdRepeatData4::DCDC_VSET_EN_MASK) != 0
    }

    /// Read **HP_PWR_SRC_SEL** `[18:18]`
    ///
    /// HP system power source select. 0:LDO. 1: DCDC.
    #[inline(always)]
    pub const fn hp_pwr_src_sel(&self) -> bool {
        (self.bits & RdRepeatData4::HP_PWR_SRC_SEL_MASK) != 0
    }

    /// Read **USB_OTG11_DREFL** `[15:14]`
    ///
    /// Represents the usb otg11 single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV.
    #[inline(always)]
    pub const fn usb_otg11_drefl(&self) -> u32 {
        (self.bits & RdRepeatData4::USB_OTG11_DREFL_MASK) >> RdRepeatData4::USB_OTG11_DREFL_OFFSET
    }

    /// Read **USB_DEVICE_DREFL** `[13:12]`
    ///
    /// Represents the usb device single-end input low threhold, 0.8 V to 1.04 V with step of 80 mV.
    #[inline(always)]
    pub const fn usb_device_drefl(&self) -> u32 {
        (self.bits & RdRepeatData4::USB_DEVICE_DREFL_MASK) >> RdRepeatData4::USB_DEVICE_DREFL_OFFSET
    }

    /// Read **KM_DISABLE_DEPLOY_MODE** `[11:8]`
    ///
    /// TBD.
    #[inline(always)]
    pub const fn km_disable_deploy_mode(&self) -> u32 {
        (self.bits & RdRepeatData4::KM_DISABLE_DEPLOY_MODE_MASK)
            >> RdRepeatData4::KM_DISABLE_DEPLOY_MODE_OFFSET
    }

    /// Read **_0PXA_TIEH_SEL_3** `[7:6]`
    ///
    /// TBD.
    #[inline(always)]
    pub const fn _0pxa_tieh_sel_3(&self) -> u32 {
        (self.bits & RdRepeatData4::_0PXA_TIEH_SEL_3_MASK) >> RdRepeatData4::_0PXA_TIEH_SEL_3_OFFSET
    }

    /// Read **_0PXA_TIEH_SEL_2** `[5:4]`
    ///
    /// TBD.
    #[inline(always)]
    pub const fn _0pxa_tieh_sel_2(&self) -> u32 {
        (self.bits & RdRepeatData4::_0PXA_TIEH_SEL_2_MASK) >> RdRepeatData4::_0PXA_TIEH_SEL_2_OFFSET
    }

    /// Read **_0PXA_TIEH_SEL_1** `[3:2]`
    ///
    /// TBD.
    #[inline(always)]
    pub const fn _0pxa_tieh_sel_1(&self) -> u32 {
        (self.bits & RdRepeatData4::_0PXA_TIEH_SEL_1_MASK) >> RdRepeatData4::_0PXA_TIEH_SEL_1_OFFSET
    }

    /// Read **_0PXA_TIEH_SEL_0** `[1:0]`
    ///
    /// TBD
    #[inline(always)]
    pub const fn _0pxa_tieh_sel_0(&self) -> u32 {
        (self.bits & RdRepeatData4::_0PXA_TIEH_SEL_0_MASK) >> RdRepeatData4::_0PXA_TIEH_SEL_0_OFFSET
    }
}

impl Default for RdRepeatData4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdRepeatData4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdRepeatData4Val {
        RdRepeatData4Val::from_bits(self.value.read_bits())
    }
}

/// RD_MAC_SYS_0 Register
///
/// BLOCK1 data register $n.
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **MAC_0** `[31:0]` (RO)
///
///   Stores the low 32 bits of MAC address.
#[repr(transparent)]
pub struct RdMacSys0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdMacSys0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdMacSys0 {
    // ======== Field Constants ========

    /// **MAC_0**
    ///
    /// Stores the low 32 bits of MAC address.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MAC_0_OFFSET: u32 = 0;
    pub const MAC_0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MAC_0** `[31:0]` (RO)
    ///
    /// Stores the low 32 bits of MAC address.
    #[inline(always)]
    pub fn mac_0(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_0_MASK) >> Self::MAC_0_OFFSET
    }
}

/// RD_MAC_SYS_0 value type for batch read/modify/write operations
///
/// Use `rdmacsys0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdmacsys0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdMacSys0Val {
    bits: u32,
}

impl RdMacSys0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAC_0** `[31:0]`
    ///
    /// Stores the low 32 bits of MAC address.
    #[inline(always)]
    pub const fn mac_0(&self) -> u32 {
        (self.bits & RdMacSys0::MAC_0_MASK) >> RdMacSys0::MAC_0_OFFSET
    }
}

impl Default for RdMacSys0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdMacSys0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdMacSys0Val {
        RdMacSys0Val::from_bits(self.value.read_bits())
    }
}

/// RD_MAC_SYS_1 Register
///
/// BLOCK1 data register $n.
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **MAC_EXT** `[31:16]` (RO)
///
///   Stores the extended bits of MAC address.
///
/// - **MAC_1** `[15:0]` (RO)
///
///   Stores the high 16 bits of MAC address.
#[repr(transparent)]
pub struct RdMacSys1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdMacSys1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdMacSys1 {
    // ======== Field Constants ========

    /// **MAC_EXT**
    ///
    /// Stores the extended bits of MAC address.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const MAC_EXT_OFFSET: u32 = 16;
    pub const MAC_EXT_MASK: u32 = 0xffff0000;

    /// **MAC_1**
    ///
    /// Stores the high 16 bits of MAC address.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAC_1_OFFSET: u32 = 0;
    pub const MAC_1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **MAC_EXT** `[31:16]` (RO)
    ///
    /// Stores the extended bits of MAC address.
    #[inline(always)]
    pub fn mac_ext(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_EXT_MASK) >> Self::MAC_EXT_OFFSET
    }

    /// **MAC_1** `[15:0]` (RO)
    ///
    /// Stores the high 16 bits of MAC address.
    #[inline(always)]
    pub fn mac_1(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_1_MASK) >> Self::MAC_1_OFFSET
    }
}

/// RD_MAC_SYS_1 value type for batch read/modify/write operations
///
/// Use `rdmacsys1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdmacsys1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdMacSys1Val {
    bits: u32,
}

impl RdMacSys1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAC_EXT** `[31:16]`
    ///
    /// Stores the extended bits of MAC address.
    #[inline(always)]
    pub const fn mac_ext(&self) -> u32 {
        (self.bits & RdMacSys1::MAC_EXT_MASK) >> RdMacSys1::MAC_EXT_OFFSET
    }

    /// Read **MAC_1** `[15:0]`
    ///
    /// Stores the high 16 bits of MAC address.
    #[inline(always)]
    pub const fn mac_1(&self) -> u32 {
        (self.bits & RdMacSys1::MAC_1_MASK) >> RdMacSys1::MAC_1_OFFSET
    }
}

impl Default for RdMacSys1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdMacSys1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdMacSys1Val {
        RdMacSys1Val::from_bits(self.value.read_bits())
    }
}

/// RD_MAC_SYS_2 Register
///
/// BLOCK1 data register $n.
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **MAC_RESERVED_0** `[31:14]` (RO)
///
///   Reserved.
///
/// - **MAC_RESERVED_1** `[13:0]` (RO)
///
///   Reserved.
#[repr(transparent)]
pub struct RdMacSys2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdMacSys2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdMacSys2 {
    // ======== Field Constants ========

    /// **MAC_RESERVED_0**
    ///
    /// Reserved.
    ///
    /// Bits `[31:14]`, mask `0xffffc000`
    pub const MAC_RESERVED_0_OFFSET: u32 = 14;
    pub const MAC_RESERVED_0_MASK: u32 = 0xffffc000;

    /// **MAC_RESERVED_1**
    ///
    /// Reserved.
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const MAC_RESERVED_1_OFFSET: u32 = 0;
    pub const MAC_RESERVED_1_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **MAC_RESERVED_0** `[31:14]` (RO)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn mac_reserved_0(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_RESERVED_0_MASK) >> Self::MAC_RESERVED_0_OFFSET
    }

    /// **MAC_RESERVED_1** `[13:0]` (RO)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn mac_reserved_1(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_RESERVED_1_MASK) >> Self::MAC_RESERVED_1_OFFSET
    }
}

/// RD_MAC_SYS_2 value type for batch read/modify/write operations
///
/// Use `rdmacsys2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdmacsys2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdMacSys2Val {
    bits: u32,
}

impl RdMacSys2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAC_RESERVED_0** `[31:14]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn mac_reserved_0(&self) -> u32 {
        (self.bits & RdMacSys2::MAC_RESERVED_0_MASK) >> RdMacSys2::MAC_RESERVED_0_OFFSET
    }

    /// Read **MAC_RESERVED_1** `[13:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn mac_reserved_1(&self) -> u32 {
        (self.bits & RdMacSys2::MAC_RESERVED_1_MASK) >> RdMacSys2::MAC_RESERVED_1_OFFSET
    }
}

impl Default for RdMacSys2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdMacSys2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdMacSys2Val {
        RdMacSys2Val::from_bits(self.value.read_bits())
    }
}

/// RD_MAC_SYS_3 Register
///
/// BLOCK1 data register $n.
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **SYS_DATA_PART0_0** `[31:18]` (RO)
///
///   Stores the first 14 bits of the zeroth part of system data.
///
/// - **MAC_RESERVED_2** `[17:0]` (RO)
///
///   Reserved.
#[repr(transparent)]
pub struct RdMacSys3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdMacSys3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdMacSys3 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART0_0**
    ///
    /// Stores the first 14 bits of the zeroth part of system data.
    ///
    /// Bits `[31:18]`, mask `0xfffc0000`
    pub const SYS_DATA_PART0_0_OFFSET: u32 = 18;
    pub const SYS_DATA_PART0_0_MASK: u32 = 0xfffc0000;

    /// **MAC_RESERVED_2**
    ///
    /// Reserved.
    ///
    /// Bits `[17:0]`, mask `0x0003ffff`
    pub const MAC_RESERVED_2_OFFSET: u32 = 0;
    pub const MAC_RESERVED_2_MASK: u32 = 0x0003ffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART0_0** `[31:18]` (RO)
    ///
    /// Stores the first 14 bits of the zeroth part of system data.
    #[inline(always)]
    pub fn sys_data_part0_0(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART0_0_MASK) >> Self::SYS_DATA_PART0_0_OFFSET
    }

    /// **MAC_RESERVED_2** `[17:0]` (RO)
    ///
    /// Reserved.
    #[inline(always)]
    pub fn mac_reserved_2(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_RESERVED_2_MASK) >> Self::MAC_RESERVED_2_OFFSET
    }
}

/// RD_MAC_SYS_3 value type for batch read/modify/write operations
///
/// Use `rdmacsys3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdmacsys3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdMacSys3Val {
    bits: u32,
}

impl RdMacSys3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART0_0** `[31:18]`
    ///
    /// Stores the first 14 bits of the zeroth part of system data.
    #[inline(always)]
    pub const fn sys_data_part0_0(&self) -> u32 {
        (self.bits & RdMacSys3::SYS_DATA_PART0_0_MASK) >> RdMacSys3::SYS_DATA_PART0_0_OFFSET
    }

    /// Read **MAC_RESERVED_2** `[17:0]`
    ///
    /// Reserved.
    #[inline(always)]
    pub const fn mac_reserved_2(&self) -> u32 {
        (self.bits & RdMacSys3::MAC_RESERVED_2_MASK) >> RdMacSys3::MAC_RESERVED_2_OFFSET
    }
}

impl Default for RdMacSys3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdMacSys3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdMacSys3Val {
        RdMacSys3Val::from_bits(self.value.read_bits())
    }
}

/// RD_MAC_SYS_4 Register
///
/// BLOCK1 data register $n.
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **SYS_DATA_PART0_1** `[31:0]` (RO)
///
///   Stores the first 32 bits of the zeroth part of system data.
#[repr(transparent)]
pub struct RdMacSys4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdMacSys4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdMacSys4 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART0_1**
    ///
    /// Stores the first 32 bits of the zeroth part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART0_1_OFFSET: u32 = 0;
    pub const SYS_DATA_PART0_1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART0_1** `[31:0]` (RO)
    ///
    /// Stores the first 32 bits of the zeroth part of system data.
    #[inline(always)]
    pub fn sys_data_part0_1(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART0_1_MASK) >> Self::SYS_DATA_PART0_1_OFFSET
    }
}

/// RD_MAC_SYS_4 value type for batch read/modify/write operations
///
/// Use `rdmacsys4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdmacsys4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdMacSys4Val {
    bits: u32,
}

impl RdMacSys4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART0_1** `[31:0]`
    ///
    /// Stores the first 32 bits of the zeroth part of system data.
    #[inline(always)]
    pub const fn sys_data_part0_1(&self) -> u32 {
        (self.bits & RdMacSys4::SYS_DATA_PART0_1_MASK) >> RdMacSys4::SYS_DATA_PART0_1_OFFSET
    }
}

impl Default for RdMacSys4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdMacSys4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdMacSys4Val {
        RdMacSys4Val::from_bits(self.value.read_bits())
    }
}

/// RD_MAC_SYS_5 Register
///
/// BLOCK1 data register $n.
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **SYS_DATA_PART0_2** `[31:0]` (RO)
///
///   Stores the second 32 bits of the zeroth part of system data.
#[repr(transparent)]
pub struct RdMacSys5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdMacSys5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdMacSys5 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART0_2**
    ///
    /// Stores the second 32 bits of the zeroth part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART0_2_OFFSET: u32 = 0;
    pub const SYS_DATA_PART0_2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART0_2** `[31:0]` (RO)
    ///
    /// Stores the second 32 bits of the zeroth part of system data.
    #[inline(always)]
    pub fn sys_data_part0_2(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART0_2_MASK) >> Self::SYS_DATA_PART0_2_OFFSET
    }
}

/// RD_MAC_SYS_5 value type for batch read/modify/write operations
///
/// Use `rdmacsys5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdmacsys5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdMacSys5Val {
    bits: u32,
}

impl RdMacSys5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART0_2** `[31:0]`
    ///
    /// Stores the second 32 bits of the zeroth part of system data.
    #[inline(always)]
    pub const fn sys_data_part0_2(&self) -> u32 {
        (self.bits & RdMacSys5::SYS_DATA_PART0_2_MASK) >> RdMacSys5::SYS_DATA_PART0_2_OFFSET
    }
}

impl Default for RdMacSys5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdMacSys5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdMacSys5Val {
        RdMacSys5Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART1_DATA0 Register
///
/// Register $n of BLOCK2 (system).
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **SYS_DATA_PART1_0** `[31:0]` (RO)
///
///   Stores the zeroth 32 bits of the first part of system data.
#[repr(transparent)]
pub struct RdSysPart1Data0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart1Data0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart1Data0 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART1_0**
    ///
    /// Stores the zeroth 32 bits of the first part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART1_0_OFFSET: u32 = 0;
    pub const SYS_DATA_PART1_0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART1_0** `[31:0]` (RO)
    ///
    /// Stores the zeroth 32 bits of the first part of system data.
    #[inline(always)]
    pub fn sys_data_part1_0(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART1_0_MASK) >> Self::SYS_DATA_PART1_0_OFFSET
    }
}

/// RD_SYS_PART1_DATA0 value type for batch read/modify/write operations
///
/// Use `rdsyspart1data0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart1data0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart1Data0Val {
    bits: u32,
}

impl RdSysPart1Data0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART1_0** `[31:0]`
    ///
    /// Stores the zeroth 32 bits of the first part of system data.
    #[inline(always)]
    pub const fn sys_data_part1_0(&self) -> u32 {
        (self.bits & RdSysPart1Data0::SYS_DATA_PART1_0_MASK)
            >> RdSysPart1Data0::SYS_DATA_PART1_0_OFFSET
    }
}

impl Default for RdSysPart1Data0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart1Data0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart1Data0Val {
        RdSysPart1Data0Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART1_DATA1 Register
///
/// Register $n of BLOCK2 (system).
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **SYS_DATA_PART1_1** `[31:0]` (RO)
///
///   Stores the first 32 bits of the first part of system data.
#[repr(transparent)]
pub struct RdSysPart1Data1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart1Data1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart1Data1 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART1_1**
    ///
    /// Stores the first 32 bits of the first part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART1_1_OFFSET: u32 = 0;
    pub const SYS_DATA_PART1_1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART1_1** `[31:0]` (RO)
    ///
    /// Stores the first 32 bits of the first part of system data.
    #[inline(always)]
    pub fn sys_data_part1_1(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART1_1_MASK) >> Self::SYS_DATA_PART1_1_OFFSET
    }
}

/// RD_SYS_PART1_DATA1 value type for batch read/modify/write operations
///
/// Use `rdsyspart1data1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart1data1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart1Data1Val {
    bits: u32,
}

impl RdSysPart1Data1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART1_1** `[31:0]`
    ///
    /// Stores the first 32 bits of the first part of system data.
    #[inline(always)]
    pub const fn sys_data_part1_1(&self) -> u32 {
        (self.bits & RdSysPart1Data1::SYS_DATA_PART1_1_MASK)
            >> RdSysPart1Data1::SYS_DATA_PART1_1_OFFSET
    }
}

impl Default for RdSysPart1Data1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart1Data1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart1Data1Val {
        RdSysPart1Data1Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART1_DATA2 Register
///
/// Register $n of BLOCK2 (system).
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **SYS_DATA_PART1_2** `[31:0]` (RO)
///
///   Stores the second 32 bits of the first part of system data.
#[repr(transparent)]
pub struct RdSysPart1Data2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart1Data2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart1Data2 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART1_2**
    ///
    /// Stores the second 32 bits of the first part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART1_2_OFFSET: u32 = 0;
    pub const SYS_DATA_PART1_2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART1_2** `[31:0]` (RO)
    ///
    /// Stores the second 32 bits of the first part of system data.
    #[inline(always)]
    pub fn sys_data_part1_2(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART1_2_MASK) >> Self::SYS_DATA_PART1_2_OFFSET
    }
}

/// RD_SYS_PART1_DATA2 value type for batch read/modify/write operations
///
/// Use `rdsyspart1data2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart1data2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart1Data2Val {
    bits: u32,
}

impl RdSysPart1Data2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART1_2** `[31:0]`
    ///
    /// Stores the second 32 bits of the first part of system data.
    #[inline(always)]
    pub const fn sys_data_part1_2(&self) -> u32 {
        (self.bits & RdSysPart1Data2::SYS_DATA_PART1_2_MASK)
            >> RdSysPart1Data2::SYS_DATA_PART1_2_OFFSET
    }
}

impl Default for RdSysPart1Data2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart1Data2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart1Data2Val {
        RdSysPart1Data2Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART1_DATA3 Register
///
/// Register $n of BLOCK2 (system).
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **SYS_DATA_PART1_3** `[31:0]` (RO)
///
///   Stores the third 32 bits of the first part of system data.
#[repr(transparent)]
pub struct RdSysPart1Data3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart1Data3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart1Data3 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART1_3**
    ///
    /// Stores the third 32 bits of the first part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART1_3_OFFSET: u32 = 0;
    pub const SYS_DATA_PART1_3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART1_3** `[31:0]` (RO)
    ///
    /// Stores the third 32 bits of the first part of system data.
    #[inline(always)]
    pub fn sys_data_part1_3(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART1_3_MASK) >> Self::SYS_DATA_PART1_3_OFFSET
    }
}

/// RD_SYS_PART1_DATA3 value type for batch read/modify/write operations
///
/// Use `rdsyspart1data3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart1data3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart1Data3Val {
    bits: u32,
}

impl RdSysPart1Data3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART1_3** `[31:0]`
    ///
    /// Stores the third 32 bits of the first part of system data.
    #[inline(always)]
    pub const fn sys_data_part1_3(&self) -> u32 {
        (self.bits & RdSysPart1Data3::SYS_DATA_PART1_3_MASK)
            >> RdSysPart1Data3::SYS_DATA_PART1_3_OFFSET
    }
}

impl Default for RdSysPart1Data3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart1Data3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart1Data3Val {
        RdSysPart1Data3Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART1_DATA4 Register
///
/// Register $n of BLOCK2 (system).
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **SYS_DATA_PART1_4** `[31:0]` (RO)
///
///   Stores the fourth 32 bits of the first part of system data.
#[repr(transparent)]
pub struct RdSysPart1Data4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart1Data4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart1Data4 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART1_4**
    ///
    /// Stores the fourth 32 bits of the first part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART1_4_OFFSET: u32 = 0;
    pub const SYS_DATA_PART1_4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART1_4** `[31:0]` (RO)
    ///
    /// Stores the fourth 32 bits of the first part of system data.
    #[inline(always)]
    pub fn sys_data_part1_4(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART1_4_MASK) >> Self::SYS_DATA_PART1_4_OFFSET
    }
}

/// RD_SYS_PART1_DATA4 value type for batch read/modify/write operations
///
/// Use `rdsyspart1data4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart1data4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart1Data4Val {
    bits: u32,
}

impl RdSysPart1Data4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART1_4** `[31:0]`
    ///
    /// Stores the fourth 32 bits of the first part of system data.
    #[inline(always)]
    pub const fn sys_data_part1_4(&self) -> u32 {
        (self.bits & RdSysPart1Data4::SYS_DATA_PART1_4_MASK)
            >> RdSysPart1Data4::SYS_DATA_PART1_4_OFFSET
    }
}

impl Default for RdSysPart1Data4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart1Data4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart1Data4Val {
        RdSysPart1Data4Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART1_DATA5 Register
///
/// Register $n of BLOCK2 (system).
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **SYS_DATA_PART1_5** `[31:0]` (RO)
///
///   Stores the fifth 32 bits of the first part of system data.
#[repr(transparent)]
pub struct RdSysPart1Data5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart1Data5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart1Data5 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART1_5**
    ///
    /// Stores the fifth 32 bits of the first part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART1_5_OFFSET: u32 = 0;
    pub const SYS_DATA_PART1_5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART1_5** `[31:0]` (RO)
    ///
    /// Stores the fifth 32 bits of the first part of system data.
    #[inline(always)]
    pub fn sys_data_part1_5(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART1_5_MASK) >> Self::SYS_DATA_PART1_5_OFFSET
    }
}

/// RD_SYS_PART1_DATA5 value type for batch read/modify/write operations
///
/// Use `rdsyspart1data5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart1data5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart1Data5Val {
    bits: u32,
}

impl RdSysPart1Data5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART1_5** `[31:0]`
    ///
    /// Stores the fifth 32 bits of the first part of system data.
    #[inline(always)]
    pub const fn sys_data_part1_5(&self) -> u32 {
        (self.bits & RdSysPart1Data5::SYS_DATA_PART1_5_MASK)
            >> RdSysPart1Data5::SYS_DATA_PART1_5_OFFSET
    }
}

impl Default for RdSysPart1Data5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart1Data5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart1Data5Val {
        RdSysPart1Data5Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART1_DATA6 Register
///
/// Register $n of BLOCK2 (system).
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **SYS_DATA_PART1_6** `[31:0]` (RO)
///
///   Stores the sixth 32 bits of the first part of system data.
#[repr(transparent)]
pub struct RdSysPart1Data6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart1Data6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart1Data6 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART1_6**
    ///
    /// Stores the sixth 32 bits of the first part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART1_6_OFFSET: u32 = 0;
    pub const SYS_DATA_PART1_6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART1_6** `[31:0]` (RO)
    ///
    /// Stores the sixth 32 bits of the first part of system data.
    #[inline(always)]
    pub fn sys_data_part1_6(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART1_6_MASK) >> Self::SYS_DATA_PART1_6_OFFSET
    }
}

/// RD_SYS_PART1_DATA6 value type for batch read/modify/write operations
///
/// Use `rdsyspart1data6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart1data6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart1Data6Val {
    bits: u32,
}

impl RdSysPart1Data6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART1_6** `[31:0]`
    ///
    /// Stores the sixth 32 bits of the first part of system data.
    #[inline(always)]
    pub const fn sys_data_part1_6(&self) -> u32 {
        (self.bits & RdSysPart1Data6::SYS_DATA_PART1_6_MASK)
            >> RdSysPart1Data6::SYS_DATA_PART1_6_OFFSET
    }
}

impl Default for RdSysPart1Data6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart1Data6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart1Data6Val {
        RdSysPart1Data6Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART1_DATA7 Register
///
/// Register $n of BLOCK2 (system).
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **SYS_DATA_PART1_7** `[31:0]` (RO)
///
///   Stores the seventh 32 bits of the first part of system data.
#[repr(transparent)]
pub struct RdSysPart1Data7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart1Data7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart1Data7 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART1_7**
    ///
    /// Stores the seventh 32 bits of the first part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART1_7_OFFSET: u32 = 0;
    pub const SYS_DATA_PART1_7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART1_7** `[31:0]` (RO)
    ///
    /// Stores the seventh 32 bits of the first part of system data.
    #[inline(always)]
    pub fn sys_data_part1_7(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART1_7_MASK) >> Self::SYS_DATA_PART1_7_OFFSET
    }
}

/// RD_SYS_PART1_DATA7 value type for batch read/modify/write operations
///
/// Use `rdsyspart1data7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart1data7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart1Data7Val {
    bits: u32,
}

impl RdSysPart1Data7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART1_7** `[31:0]`
    ///
    /// Stores the seventh 32 bits of the first part of system data.
    #[inline(always)]
    pub const fn sys_data_part1_7(&self) -> u32 {
        (self.bits & RdSysPart1Data7::SYS_DATA_PART1_7_MASK)
            >> RdSysPart1Data7::SYS_DATA_PART1_7_OFFSET
    }
}

impl Default for RdSysPart1Data7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart1Data7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart1Data7Val {
        RdSysPart1Data7Val::from_bits(self.value.read_bits())
    }
}

/// RD_USR_DATA0 Register
///
/// Register $n of BLOCK3 (user).
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **USR_DATA0** `[31:0]` (RO)
///
///   Stores the zeroth 32 bits of BLOCK3 (user).
#[repr(transparent)]
pub struct RdUsrData0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdUsrData0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdUsrData0 {
    // ======== Field Constants ========

    /// **USR_DATA0**
    ///
    /// Stores the zeroth 32 bits of BLOCK3 (user).
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USR_DATA0_OFFSET: u32 = 0;
    pub const USR_DATA0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USR_DATA0** `[31:0]` (RO)
    ///
    /// Stores the zeroth 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub fn usr_data0(&self) -> u32 {
        (self.value.read_bits() & Self::USR_DATA0_MASK) >> Self::USR_DATA0_OFFSET
    }
}

/// RD_USR_DATA0 value type for batch read/modify/write operations
///
/// Use `rdusrdata0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdusrdata0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdUsrData0Val {
    bits: u32,
}

impl RdUsrData0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USR_DATA0** `[31:0]`
    ///
    /// Stores the zeroth 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub const fn usr_data0(&self) -> u32 {
        (self.bits & RdUsrData0::USR_DATA0_MASK) >> RdUsrData0::USR_DATA0_OFFSET
    }
}

impl Default for RdUsrData0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdUsrData0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdUsrData0Val {
        RdUsrData0Val::from_bits(self.value.read_bits())
    }
}

/// RD_USR_DATA1 Register
///
/// Register $n of BLOCK3 (user).
///
/// Address offset: 0x0080
///
/// ## Fields
///
/// - **USR_DATA1** `[31:0]` (RO)
///
///   Stores the first 32 bits of BLOCK3 (user).
#[repr(transparent)]
pub struct RdUsrData1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdUsrData1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdUsrData1 {
    // ======== Field Constants ========

    /// **USR_DATA1**
    ///
    /// Stores the first 32 bits of BLOCK3 (user).
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USR_DATA1_OFFSET: u32 = 0;
    pub const USR_DATA1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USR_DATA1** `[31:0]` (RO)
    ///
    /// Stores the first 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub fn usr_data1(&self) -> u32 {
        (self.value.read_bits() & Self::USR_DATA1_MASK) >> Self::USR_DATA1_OFFSET
    }
}

/// RD_USR_DATA1 value type for batch read/modify/write operations
///
/// Use `rdusrdata1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdusrdata1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdUsrData1Val {
    bits: u32,
}

impl RdUsrData1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USR_DATA1** `[31:0]`
    ///
    /// Stores the first 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub const fn usr_data1(&self) -> u32 {
        (self.bits & RdUsrData1::USR_DATA1_MASK) >> RdUsrData1::USR_DATA1_OFFSET
    }
}

impl Default for RdUsrData1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdUsrData1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdUsrData1Val {
        RdUsrData1Val::from_bits(self.value.read_bits())
    }
}

/// RD_USR_DATA2 Register
///
/// Register $n of BLOCK3 (user).
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **USR_DATA2** `[31:0]` (RO)
///
///   Stores the second 32 bits of BLOCK3 (user).
#[repr(transparent)]
pub struct RdUsrData2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdUsrData2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdUsrData2 {
    // ======== Field Constants ========

    /// **USR_DATA2**
    ///
    /// Stores the second 32 bits of BLOCK3 (user).
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USR_DATA2_OFFSET: u32 = 0;
    pub const USR_DATA2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USR_DATA2** `[31:0]` (RO)
    ///
    /// Stores the second 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub fn usr_data2(&self) -> u32 {
        (self.value.read_bits() & Self::USR_DATA2_MASK) >> Self::USR_DATA2_OFFSET
    }
}

/// RD_USR_DATA2 value type for batch read/modify/write operations
///
/// Use `rdusrdata2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdusrdata2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdUsrData2Val {
    bits: u32,
}

impl RdUsrData2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USR_DATA2** `[31:0]`
    ///
    /// Stores the second 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub const fn usr_data2(&self) -> u32 {
        (self.bits & RdUsrData2::USR_DATA2_MASK) >> RdUsrData2::USR_DATA2_OFFSET
    }
}

impl Default for RdUsrData2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdUsrData2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdUsrData2Val {
        RdUsrData2Val::from_bits(self.value.read_bits())
    }
}

/// RD_USR_DATA3 Register
///
/// Register $n of BLOCK3 (user).
///
/// Address offset: 0x0088
///
/// ## Fields
///
/// - **USR_DATA3** `[31:0]` (RO)
///
///   Stores the third 32 bits of BLOCK3 (user).
#[repr(transparent)]
pub struct RdUsrData3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdUsrData3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdUsrData3 {
    // ======== Field Constants ========

    /// **USR_DATA3**
    ///
    /// Stores the third 32 bits of BLOCK3 (user).
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USR_DATA3_OFFSET: u32 = 0;
    pub const USR_DATA3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USR_DATA3** `[31:0]` (RO)
    ///
    /// Stores the third 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub fn usr_data3(&self) -> u32 {
        (self.value.read_bits() & Self::USR_DATA3_MASK) >> Self::USR_DATA3_OFFSET
    }
}

/// RD_USR_DATA3 value type for batch read/modify/write operations
///
/// Use `rdusrdata3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdusrdata3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdUsrData3Val {
    bits: u32,
}

impl RdUsrData3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USR_DATA3** `[31:0]`
    ///
    /// Stores the third 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub const fn usr_data3(&self) -> u32 {
        (self.bits & RdUsrData3::USR_DATA3_MASK) >> RdUsrData3::USR_DATA3_OFFSET
    }
}

impl Default for RdUsrData3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdUsrData3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdUsrData3Val {
        RdUsrData3Val::from_bits(self.value.read_bits())
    }
}

/// RD_USR_DATA4 Register
///
/// Register $n of BLOCK3 (user).
///
/// Address offset: 0x008c
///
/// ## Fields
///
/// - **USR_DATA4** `[31:0]` (RO)
///
///   Stores the fourth 32 bits of BLOCK3 (user).
#[repr(transparent)]
pub struct RdUsrData4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdUsrData4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdUsrData4 {
    // ======== Field Constants ========

    /// **USR_DATA4**
    ///
    /// Stores the fourth 32 bits of BLOCK3 (user).
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USR_DATA4_OFFSET: u32 = 0;
    pub const USR_DATA4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USR_DATA4** `[31:0]` (RO)
    ///
    /// Stores the fourth 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub fn usr_data4(&self) -> u32 {
        (self.value.read_bits() & Self::USR_DATA4_MASK) >> Self::USR_DATA4_OFFSET
    }
}

/// RD_USR_DATA4 value type for batch read/modify/write operations
///
/// Use `rdusrdata4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdusrdata4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdUsrData4Val {
    bits: u32,
}

impl RdUsrData4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USR_DATA4** `[31:0]`
    ///
    /// Stores the fourth 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub const fn usr_data4(&self) -> u32 {
        (self.bits & RdUsrData4::USR_DATA4_MASK) >> RdUsrData4::USR_DATA4_OFFSET
    }
}

impl Default for RdUsrData4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdUsrData4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdUsrData4Val {
        RdUsrData4Val::from_bits(self.value.read_bits())
    }
}

/// RD_USR_DATA5 Register
///
/// Register $n of BLOCK3 (user).
///
/// Address offset: 0x0090
///
/// ## Fields
///
/// - **USR_DATA5** `[31:0]` (RO)
///
///   Stores the fifth 32 bits of BLOCK3 (user).
#[repr(transparent)]
pub struct RdUsrData5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdUsrData5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdUsrData5 {
    // ======== Field Constants ========

    /// **USR_DATA5**
    ///
    /// Stores the fifth 32 bits of BLOCK3 (user).
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USR_DATA5_OFFSET: u32 = 0;
    pub const USR_DATA5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USR_DATA5** `[31:0]` (RO)
    ///
    /// Stores the fifth 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub fn usr_data5(&self) -> u32 {
        (self.value.read_bits() & Self::USR_DATA5_MASK) >> Self::USR_DATA5_OFFSET
    }
}

/// RD_USR_DATA5 value type for batch read/modify/write operations
///
/// Use `rdusrdata5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdusrdata5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdUsrData5Val {
    bits: u32,
}

impl RdUsrData5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USR_DATA5** `[31:0]`
    ///
    /// Stores the fifth 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub const fn usr_data5(&self) -> u32 {
        (self.bits & RdUsrData5::USR_DATA5_MASK) >> RdUsrData5::USR_DATA5_OFFSET
    }
}

impl Default for RdUsrData5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdUsrData5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdUsrData5Val {
        RdUsrData5Val::from_bits(self.value.read_bits())
    }
}

/// RD_USR_DATA6 Register
///
/// Register $n of BLOCK3 (user).
///
/// Address offset: 0x0094
///
/// ## Fields
///
/// - **USR_DATA6** `[31:0]` (RO)
///
///   Stores the sixth 32 bits of BLOCK3 (user).
#[repr(transparent)]
pub struct RdUsrData6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdUsrData6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdUsrData6 {
    // ======== Field Constants ========

    /// **USR_DATA6**
    ///
    /// Stores the sixth 32 bits of BLOCK3 (user).
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USR_DATA6_OFFSET: u32 = 0;
    pub const USR_DATA6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USR_DATA6** `[31:0]` (RO)
    ///
    /// Stores the sixth 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub fn usr_data6(&self) -> u32 {
        (self.value.read_bits() & Self::USR_DATA6_MASK) >> Self::USR_DATA6_OFFSET
    }
}

/// RD_USR_DATA6 value type for batch read/modify/write operations
///
/// Use `rdusrdata6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdusrdata6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdUsrData6Val {
    bits: u32,
}

impl RdUsrData6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USR_DATA6** `[31:0]`
    ///
    /// Stores the sixth 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub const fn usr_data6(&self) -> u32 {
        (self.bits & RdUsrData6::USR_DATA6_MASK) >> RdUsrData6::USR_DATA6_OFFSET
    }
}

impl Default for RdUsrData6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdUsrData6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdUsrData6Val {
        RdUsrData6Val::from_bits(self.value.read_bits())
    }
}

/// RD_USR_DATA7 Register
///
/// Register $n of BLOCK3 (user).
///
/// Address offset: 0x0098
///
/// ## Fields
///
/// - **USR_DATA7** `[31:0]` (RO)
///
///   Stores the seventh 32 bits of BLOCK3 (user).
#[repr(transparent)]
pub struct RdUsrData7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdUsrData7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdUsrData7 {
    // ======== Field Constants ========

    /// **USR_DATA7**
    ///
    /// Stores the seventh 32 bits of BLOCK3 (user).
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USR_DATA7_OFFSET: u32 = 0;
    pub const USR_DATA7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USR_DATA7** `[31:0]` (RO)
    ///
    /// Stores the seventh 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub fn usr_data7(&self) -> u32 {
        (self.value.read_bits() & Self::USR_DATA7_MASK) >> Self::USR_DATA7_OFFSET
    }
}

/// RD_USR_DATA7 value type for batch read/modify/write operations
///
/// Use `rdusrdata7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdusrdata7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdUsrData7Val {
    bits: u32,
}

impl RdUsrData7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USR_DATA7** `[31:0]`
    ///
    /// Stores the seventh 32 bits of BLOCK3 (user).
    #[inline(always)]
    pub const fn usr_data7(&self) -> u32 {
        (self.bits & RdUsrData7::USR_DATA7_MASK) >> RdUsrData7::USR_DATA7_OFFSET
    }
}

impl Default for RdUsrData7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdUsrData7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdUsrData7Val {
        RdUsrData7Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY0_DATA0 Register
///
/// Register $n of BLOCK4 (KEY0).
///
/// Address offset: 0x009c
///
/// ## Fields
///
/// - **KEY0_DATA0** `[31:0]` (RO)
///
///   Stores the zeroth 32 bits of KEY0.
#[repr(transparent)]
pub struct RdKey0Data0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey0Data0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey0Data0 {
    // ======== Field Constants ========

    /// **KEY0_DATA0**
    ///
    /// Stores the zeroth 32 bits of KEY0.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY0_DATA0_OFFSET: u32 = 0;
    pub const KEY0_DATA0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY0_DATA0** `[31:0]` (RO)
    ///
    /// Stores the zeroth 32 bits of KEY0.
    #[inline(always)]
    pub fn key0_data0(&self) -> u32 {
        (self.value.read_bits() & Self::KEY0_DATA0_MASK) >> Self::KEY0_DATA0_OFFSET
    }
}

/// RD_KEY0_DATA0 value type for batch read/modify/write operations
///
/// Use `rdkey0data0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey0data0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey0Data0Val {
    bits: u32,
}

impl RdKey0Data0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY0_DATA0** `[31:0]`
    ///
    /// Stores the zeroth 32 bits of KEY0.
    #[inline(always)]
    pub const fn key0_data0(&self) -> u32 {
        (self.bits & RdKey0Data0::KEY0_DATA0_MASK) >> RdKey0Data0::KEY0_DATA0_OFFSET
    }
}

impl Default for RdKey0Data0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey0Data0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey0Data0Val {
        RdKey0Data0Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY0_DATA1 Register
///
/// Register $n of BLOCK4 (KEY0).
///
/// Address offset: 0x00a0
///
/// ## Fields
///
/// - **KEY0_DATA1** `[31:0]` (RO)
///
///   Stores the first 32 bits of KEY0.
#[repr(transparent)]
pub struct RdKey0Data1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey0Data1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey0Data1 {
    // ======== Field Constants ========

    /// **KEY0_DATA1**
    ///
    /// Stores the first 32 bits of KEY0.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY0_DATA1_OFFSET: u32 = 0;
    pub const KEY0_DATA1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY0_DATA1** `[31:0]` (RO)
    ///
    /// Stores the first 32 bits of KEY0.
    #[inline(always)]
    pub fn key0_data1(&self) -> u32 {
        (self.value.read_bits() & Self::KEY0_DATA1_MASK) >> Self::KEY0_DATA1_OFFSET
    }
}

/// RD_KEY0_DATA1 value type for batch read/modify/write operations
///
/// Use `rdkey0data1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey0data1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey0Data1Val {
    bits: u32,
}

impl RdKey0Data1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY0_DATA1** `[31:0]`
    ///
    /// Stores the first 32 bits of KEY0.
    #[inline(always)]
    pub const fn key0_data1(&self) -> u32 {
        (self.bits & RdKey0Data1::KEY0_DATA1_MASK) >> RdKey0Data1::KEY0_DATA1_OFFSET
    }
}

impl Default for RdKey0Data1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey0Data1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey0Data1Val {
        RdKey0Data1Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY0_DATA2 Register
///
/// Register $n of BLOCK4 (KEY0).
///
/// Address offset: 0x00a4
///
/// ## Fields
///
/// - **KEY0_DATA2** `[31:0]` (RO)
///
///   Stores the second 32 bits of KEY0.
#[repr(transparent)]
pub struct RdKey0Data2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey0Data2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey0Data2 {
    // ======== Field Constants ========

    /// **KEY0_DATA2**
    ///
    /// Stores the second 32 bits of KEY0.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY0_DATA2_OFFSET: u32 = 0;
    pub const KEY0_DATA2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY0_DATA2** `[31:0]` (RO)
    ///
    /// Stores the second 32 bits of KEY0.
    #[inline(always)]
    pub fn key0_data2(&self) -> u32 {
        (self.value.read_bits() & Self::KEY0_DATA2_MASK) >> Self::KEY0_DATA2_OFFSET
    }
}

/// RD_KEY0_DATA2 value type for batch read/modify/write operations
///
/// Use `rdkey0data2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey0data2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey0Data2Val {
    bits: u32,
}

impl RdKey0Data2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY0_DATA2** `[31:0]`
    ///
    /// Stores the second 32 bits of KEY0.
    #[inline(always)]
    pub const fn key0_data2(&self) -> u32 {
        (self.bits & RdKey0Data2::KEY0_DATA2_MASK) >> RdKey0Data2::KEY0_DATA2_OFFSET
    }
}

impl Default for RdKey0Data2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey0Data2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey0Data2Val {
        RdKey0Data2Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY0_DATA3 Register
///
/// Register $n of BLOCK4 (KEY0).
///
/// Address offset: 0x00a8
///
/// ## Fields
///
/// - **KEY0_DATA3** `[31:0]` (RO)
///
///   Stores the third 32 bits of KEY0.
#[repr(transparent)]
pub struct RdKey0Data3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey0Data3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey0Data3 {
    // ======== Field Constants ========

    /// **KEY0_DATA3**
    ///
    /// Stores the third 32 bits of KEY0.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY0_DATA3_OFFSET: u32 = 0;
    pub const KEY0_DATA3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY0_DATA3** `[31:0]` (RO)
    ///
    /// Stores the third 32 bits of KEY0.
    #[inline(always)]
    pub fn key0_data3(&self) -> u32 {
        (self.value.read_bits() & Self::KEY0_DATA3_MASK) >> Self::KEY0_DATA3_OFFSET
    }
}

/// RD_KEY0_DATA3 value type for batch read/modify/write operations
///
/// Use `rdkey0data3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey0data3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey0Data3Val {
    bits: u32,
}

impl RdKey0Data3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY0_DATA3** `[31:0]`
    ///
    /// Stores the third 32 bits of KEY0.
    #[inline(always)]
    pub const fn key0_data3(&self) -> u32 {
        (self.bits & RdKey0Data3::KEY0_DATA3_MASK) >> RdKey0Data3::KEY0_DATA3_OFFSET
    }
}

impl Default for RdKey0Data3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey0Data3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey0Data3Val {
        RdKey0Data3Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY0_DATA4 Register
///
/// Register $n of BLOCK4 (KEY0).
///
/// Address offset: 0x00ac
///
/// ## Fields
///
/// - **KEY0_DATA4** `[31:0]` (RO)
///
///   Stores the fourth 32 bits of KEY0.
#[repr(transparent)]
pub struct RdKey0Data4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey0Data4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey0Data4 {
    // ======== Field Constants ========

    /// **KEY0_DATA4**
    ///
    /// Stores the fourth 32 bits of KEY0.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY0_DATA4_OFFSET: u32 = 0;
    pub const KEY0_DATA4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY0_DATA4** `[31:0]` (RO)
    ///
    /// Stores the fourth 32 bits of KEY0.
    #[inline(always)]
    pub fn key0_data4(&self) -> u32 {
        (self.value.read_bits() & Self::KEY0_DATA4_MASK) >> Self::KEY0_DATA4_OFFSET
    }
}

/// RD_KEY0_DATA4 value type for batch read/modify/write operations
///
/// Use `rdkey0data4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey0data4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey0Data4Val {
    bits: u32,
}

impl RdKey0Data4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY0_DATA4** `[31:0]`
    ///
    /// Stores the fourth 32 bits of KEY0.
    #[inline(always)]
    pub const fn key0_data4(&self) -> u32 {
        (self.bits & RdKey0Data4::KEY0_DATA4_MASK) >> RdKey0Data4::KEY0_DATA4_OFFSET
    }
}

impl Default for RdKey0Data4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey0Data4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey0Data4Val {
        RdKey0Data4Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY0_DATA5 Register
///
/// Register $n of BLOCK4 (KEY0).
///
/// Address offset: 0x00b0
///
/// ## Fields
///
/// - **KEY0_DATA5** `[31:0]` (RO)
///
///   Stores the fifth 32 bits of KEY0.
#[repr(transparent)]
pub struct RdKey0Data5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey0Data5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey0Data5 {
    // ======== Field Constants ========

    /// **KEY0_DATA5**
    ///
    /// Stores the fifth 32 bits of KEY0.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY0_DATA5_OFFSET: u32 = 0;
    pub const KEY0_DATA5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY0_DATA5** `[31:0]` (RO)
    ///
    /// Stores the fifth 32 bits of KEY0.
    #[inline(always)]
    pub fn key0_data5(&self) -> u32 {
        (self.value.read_bits() & Self::KEY0_DATA5_MASK) >> Self::KEY0_DATA5_OFFSET
    }
}

/// RD_KEY0_DATA5 value type for batch read/modify/write operations
///
/// Use `rdkey0data5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey0data5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey0Data5Val {
    bits: u32,
}

impl RdKey0Data5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY0_DATA5** `[31:0]`
    ///
    /// Stores the fifth 32 bits of KEY0.
    #[inline(always)]
    pub const fn key0_data5(&self) -> u32 {
        (self.bits & RdKey0Data5::KEY0_DATA5_MASK) >> RdKey0Data5::KEY0_DATA5_OFFSET
    }
}

impl Default for RdKey0Data5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey0Data5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey0Data5Val {
        RdKey0Data5Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY0_DATA6 Register
///
/// Register $n of BLOCK4 (KEY0).
///
/// Address offset: 0x00b4
///
/// ## Fields
///
/// - **KEY0_DATA6** `[31:0]` (RO)
///
///   Stores the sixth 32 bits of KEY0.
#[repr(transparent)]
pub struct RdKey0Data6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey0Data6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey0Data6 {
    // ======== Field Constants ========

    /// **KEY0_DATA6**
    ///
    /// Stores the sixth 32 bits of KEY0.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY0_DATA6_OFFSET: u32 = 0;
    pub const KEY0_DATA6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY0_DATA6** `[31:0]` (RO)
    ///
    /// Stores the sixth 32 bits of KEY0.
    #[inline(always)]
    pub fn key0_data6(&self) -> u32 {
        (self.value.read_bits() & Self::KEY0_DATA6_MASK) >> Self::KEY0_DATA6_OFFSET
    }
}

/// RD_KEY0_DATA6 value type for batch read/modify/write operations
///
/// Use `rdkey0data6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey0data6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey0Data6Val {
    bits: u32,
}

impl RdKey0Data6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY0_DATA6** `[31:0]`
    ///
    /// Stores the sixth 32 bits of KEY0.
    #[inline(always)]
    pub const fn key0_data6(&self) -> u32 {
        (self.bits & RdKey0Data6::KEY0_DATA6_MASK) >> RdKey0Data6::KEY0_DATA6_OFFSET
    }
}

impl Default for RdKey0Data6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey0Data6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey0Data6Val {
        RdKey0Data6Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY0_DATA7 Register
///
/// Register $n of BLOCK4 (KEY0).
///
/// Address offset: 0x00b8
///
/// ## Fields
///
/// - **KEY0_DATA7** `[31:0]` (RO)
///
///   Stores the seventh 32 bits of KEY0.
#[repr(transparent)]
pub struct RdKey0Data7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey0Data7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey0Data7 {
    // ======== Field Constants ========

    /// **KEY0_DATA7**
    ///
    /// Stores the seventh 32 bits of KEY0.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY0_DATA7_OFFSET: u32 = 0;
    pub const KEY0_DATA7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY0_DATA7** `[31:0]` (RO)
    ///
    /// Stores the seventh 32 bits of KEY0.
    #[inline(always)]
    pub fn key0_data7(&self) -> u32 {
        (self.value.read_bits() & Self::KEY0_DATA7_MASK) >> Self::KEY0_DATA7_OFFSET
    }
}

/// RD_KEY0_DATA7 value type for batch read/modify/write operations
///
/// Use `rdkey0data7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey0data7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey0Data7Val {
    bits: u32,
}

impl RdKey0Data7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY0_DATA7** `[31:0]`
    ///
    /// Stores the seventh 32 bits of KEY0.
    #[inline(always)]
    pub const fn key0_data7(&self) -> u32 {
        (self.bits & RdKey0Data7::KEY0_DATA7_MASK) >> RdKey0Data7::KEY0_DATA7_OFFSET
    }
}

impl Default for RdKey0Data7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey0Data7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey0Data7Val {
        RdKey0Data7Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY1_DATA0 Register
///
/// Register $n of BLOCK5 (KEY1).
///
/// Address offset: 0x00bc
///
/// ## Fields
///
/// - **KEY1_DATA0** `[31:0]` (RO)
///
///   Stores the zeroth 32 bits of KEY1.
#[repr(transparent)]
pub struct RdKey1Data0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey1Data0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey1Data0 {
    // ======== Field Constants ========

    /// **KEY1_DATA0**
    ///
    /// Stores the zeroth 32 bits of KEY1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY1_DATA0_OFFSET: u32 = 0;
    pub const KEY1_DATA0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY1_DATA0** `[31:0]` (RO)
    ///
    /// Stores the zeroth 32 bits of KEY1.
    #[inline(always)]
    pub fn key1_data0(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_DATA0_MASK) >> Self::KEY1_DATA0_OFFSET
    }
}

/// RD_KEY1_DATA0 value type for batch read/modify/write operations
///
/// Use `rdkey1data0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey1data0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey1Data0Val {
    bits: u32,
}

impl RdKey1Data0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_DATA0** `[31:0]`
    ///
    /// Stores the zeroth 32 bits of KEY1.
    #[inline(always)]
    pub const fn key1_data0(&self) -> u32 {
        (self.bits & RdKey1Data0::KEY1_DATA0_MASK) >> RdKey1Data0::KEY1_DATA0_OFFSET
    }
}

impl Default for RdKey1Data0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey1Data0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey1Data0Val {
        RdKey1Data0Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY1_DATA1 Register
///
/// Register $n of BLOCK5 (KEY1).
///
/// Address offset: 0x00c0
///
/// ## Fields
///
/// - **KEY1_DATA1** `[31:0]` (RO)
///
///   Stores the first 32 bits of KEY1.
#[repr(transparent)]
pub struct RdKey1Data1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey1Data1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey1Data1 {
    // ======== Field Constants ========

    /// **KEY1_DATA1**
    ///
    /// Stores the first 32 bits of KEY1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY1_DATA1_OFFSET: u32 = 0;
    pub const KEY1_DATA1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY1_DATA1** `[31:0]` (RO)
    ///
    /// Stores the first 32 bits of KEY1.
    #[inline(always)]
    pub fn key1_data1(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_DATA1_MASK) >> Self::KEY1_DATA1_OFFSET
    }
}

/// RD_KEY1_DATA1 value type for batch read/modify/write operations
///
/// Use `rdkey1data1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey1data1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey1Data1Val {
    bits: u32,
}

impl RdKey1Data1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_DATA1** `[31:0]`
    ///
    /// Stores the first 32 bits of KEY1.
    #[inline(always)]
    pub const fn key1_data1(&self) -> u32 {
        (self.bits & RdKey1Data1::KEY1_DATA1_MASK) >> RdKey1Data1::KEY1_DATA1_OFFSET
    }
}

impl Default for RdKey1Data1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey1Data1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey1Data1Val {
        RdKey1Data1Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY1_DATA2 Register
///
/// Register $n of BLOCK5 (KEY1).
///
/// Address offset: 0x00c4
///
/// ## Fields
///
/// - **KEY1_DATA2** `[31:0]` (RO)
///
///   Stores the second 32 bits of KEY1.
#[repr(transparent)]
pub struct RdKey1Data2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey1Data2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey1Data2 {
    // ======== Field Constants ========

    /// **KEY1_DATA2**
    ///
    /// Stores the second 32 bits of KEY1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY1_DATA2_OFFSET: u32 = 0;
    pub const KEY1_DATA2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY1_DATA2** `[31:0]` (RO)
    ///
    /// Stores the second 32 bits of KEY1.
    #[inline(always)]
    pub fn key1_data2(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_DATA2_MASK) >> Self::KEY1_DATA2_OFFSET
    }
}

/// RD_KEY1_DATA2 value type for batch read/modify/write operations
///
/// Use `rdkey1data2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey1data2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey1Data2Val {
    bits: u32,
}

impl RdKey1Data2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_DATA2** `[31:0]`
    ///
    /// Stores the second 32 bits of KEY1.
    #[inline(always)]
    pub const fn key1_data2(&self) -> u32 {
        (self.bits & RdKey1Data2::KEY1_DATA2_MASK) >> RdKey1Data2::KEY1_DATA2_OFFSET
    }
}

impl Default for RdKey1Data2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey1Data2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey1Data2Val {
        RdKey1Data2Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY1_DATA3 Register
///
/// Register $n of BLOCK5 (KEY1).
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **KEY1_DATA3** `[31:0]` (RO)
///
///   Stores the third 32 bits of KEY1.
#[repr(transparent)]
pub struct RdKey1Data3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey1Data3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey1Data3 {
    // ======== Field Constants ========

    /// **KEY1_DATA3**
    ///
    /// Stores the third 32 bits of KEY1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY1_DATA3_OFFSET: u32 = 0;
    pub const KEY1_DATA3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY1_DATA3** `[31:0]` (RO)
    ///
    /// Stores the third 32 bits of KEY1.
    #[inline(always)]
    pub fn key1_data3(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_DATA3_MASK) >> Self::KEY1_DATA3_OFFSET
    }
}

/// RD_KEY1_DATA3 value type for batch read/modify/write operations
///
/// Use `rdkey1data3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey1data3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey1Data3Val {
    bits: u32,
}

impl RdKey1Data3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_DATA3** `[31:0]`
    ///
    /// Stores the third 32 bits of KEY1.
    #[inline(always)]
    pub const fn key1_data3(&self) -> u32 {
        (self.bits & RdKey1Data3::KEY1_DATA3_MASK) >> RdKey1Data3::KEY1_DATA3_OFFSET
    }
}

impl Default for RdKey1Data3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey1Data3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey1Data3Val {
        RdKey1Data3Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY1_DATA4 Register
///
/// Register $n of BLOCK5 (KEY1).
///
/// Address offset: 0x00cc
///
/// ## Fields
///
/// - **KEY1_DATA4** `[31:0]` (RO)
///
///   Stores the fourth 32 bits of KEY1.
#[repr(transparent)]
pub struct RdKey1Data4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey1Data4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey1Data4 {
    // ======== Field Constants ========

    /// **KEY1_DATA4**
    ///
    /// Stores the fourth 32 bits of KEY1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY1_DATA4_OFFSET: u32 = 0;
    pub const KEY1_DATA4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY1_DATA4** `[31:0]` (RO)
    ///
    /// Stores the fourth 32 bits of KEY1.
    #[inline(always)]
    pub fn key1_data4(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_DATA4_MASK) >> Self::KEY1_DATA4_OFFSET
    }
}

/// RD_KEY1_DATA4 value type for batch read/modify/write operations
///
/// Use `rdkey1data4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey1data4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey1Data4Val {
    bits: u32,
}

impl RdKey1Data4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_DATA4** `[31:0]`
    ///
    /// Stores the fourth 32 bits of KEY1.
    #[inline(always)]
    pub const fn key1_data4(&self) -> u32 {
        (self.bits & RdKey1Data4::KEY1_DATA4_MASK) >> RdKey1Data4::KEY1_DATA4_OFFSET
    }
}

impl Default for RdKey1Data4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey1Data4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey1Data4Val {
        RdKey1Data4Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY1_DATA5 Register
///
/// Register $n of BLOCK5 (KEY1).
///
/// Address offset: 0x00d0
///
/// ## Fields
///
/// - **KEY1_DATA5** `[31:0]` (RO)
///
///   Stores the fifth 32 bits of KEY1.
#[repr(transparent)]
pub struct RdKey1Data5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey1Data5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey1Data5 {
    // ======== Field Constants ========

    /// **KEY1_DATA5**
    ///
    /// Stores the fifth 32 bits of KEY1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY1_DATA5_OFFSET: u32 = 0;
    pub const KEY1_DATA5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY1_DATA5** `[31:0]` (RO)
    ///
    /// Stores the fifth 32 bits of KEY1.
    #[inline(always)]
    pub fn key1_data5(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_DATA5_MASK) >> Self::KEY1_DATA5_OFFSET
    }
}

/// RD_KEY1_DATA5 value type for batch read/modify/write operations
///
/// Use `rdkey1data5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey1data5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey1Data5Val {
    bits: u32,
}

impl RdKey1Data5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_DATA5** `[31:0]`
    ///
    /// Stores the fifth 32 bits of KEY1.
    #[inline(always)]
    pub const fn key1_data5(&self) -> u32 {
        (self.bits & RdKey1Data5::KEY1_DATA5_MASK) >> RdKey1Data5::KEY1_DATA5_OFFSET
    }
}

impl Default for RdKey1Data5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey1Data5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey1Data5Val {
        RdKey1Data5Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY1_DATA6 Register
///
/// Register $n of BLOCK5 (KEY1).
///
/// Address offset: 0x00d4
///
/// ## Fields
///
/// - **KEY1_DATA6** `[31:0]` (RO)
///
///   Stores the sixth 32 bits of KEY1.
#[repr(transparent)]
pub struct RdKey1Data6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey1Data6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey1Data6 {
    // ======== Field Constants ========

    /// **KEY1_DATA6**
    ///
    /// Stores the sixth 32 bits of KEY1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY1_DATA6_OFFSET: u32 = 0;
    pub const KEY1_DATA6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY1_DATA6** `[31:0]` (RO)
    ///
    /// Stores the sixth 32 bits of KEY1.
    #[inline(always)]
    pub fn key1_data6(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_DATA6_MASK) >> Self::KEY1_DATA6_OFFSET
    }
}

/// RD_KEY1_DATA6 value type for batch read/modify/write operations
///
/// Use `rdkey1data6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey1data6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey1Data6Val {
    bits: u32,
}

impl RdKey1Data6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_DATA6** `[31:0]`
    ///
    /// Stores the sixth 32 bits of KEY1.
    #[inline(always)]
    pub const fn key1_data6(&self) -> u32 {
        (self.bits & RdKey1Data6::KEY1_DATA6_MASK) >> RdKey1Data6::KEY1_DATA6_OFFSET
    }
}

impl Default for RdKey1Data6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey1Data6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey1Data6Val {
        RdKey1Data6Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY1_DATA7 Register
///
/// Register $n of BLOCK5 (KEY1).
///
/// Address offset: 0x00d8
///
/// ## Fields
///
/// - **KEY1_DATA7** `[31:0]` (RO)
///
///   Stores the seventh 32 bits of KEY1.
#[repr(transparent)]
pub struct RdKey1Data7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey1Data7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey1Data7 {
    // ======== Field Constants ========

    /// **KEY1_DATA7**
    ///
    /// Stores the seventh 32 bits of KEY1.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY1_DATA7_OFFSET: u32 = 0;
    pub const KEY1_DATA7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY1_DATA7** `[31:0]` (RO)
    ///
    /// Stores the seventh 32 bits of KEY1.
    #[inline(always)]
    pub fn key1_data7(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_DATA7_MASK) >> Self::KEY1_DATA7_OFFSET
    }
}

/// RD_KEY1_DATA7 value type for batch read/modify/write operations
///
/// Use `rdkey1data7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey1data7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey1Data7Val {
    bits: u32,
}

impl RdKey1Data7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY1_DATA7** `[31:0]`
    ///
    /// Stores the seventh 32 bits of KEY1.
    #[inline(always)]
    pub const fn key1_data7(&self) -> u32 {
        (self.bits & RdKey1Data7::KEY1_DATA7_MASK) >> RdKey1Data7::KEY1_DATA7_OFFSET
    }
}

impl Default for RdKey1Data7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey1Data7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey1Data7Val {
        RdKey1Data7Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY2_DATA0 Register
///
/// Register $n of BLOCK6 (KEY2).
///
/// Address offset: 0x00dc
///
/// ## Fields
///
/// - **KEY2_DATA0** `[31:0]` (RO)
///
///   Stores the zeroth 32 bits of KEY2.
#[repr(transparent)]
pub struct RdKey2Data0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey2Data0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey2Data0 {
    // ======== Field Constants ========

    /// **KEY2_DATA0**
    ///
    /// Stores the zeroth 32 bits of KEY2.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY2_DATA0_OFFSET: u32 = 0;
    pub const KEY2_DATA0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY2_DATA0** `[31:0]` (RO)
    ///
    /// Stores the zeroth 32 bits of KEY2.
    #[inline(always)]
    pub fn key2_data0(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_DATA0_MASK) >> Self::KEY2_DATA0_OFFSET
    }
}

/// RD_KEY2_DATA0 value type for batch read/modify/write operations
///
/// Use `rdkey2data0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey2data0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey2Data0Val {
    bits: u32,
}

impl RdKey2Data0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_DATA0** `[31:0]`
    ///
    /// Stores the zeroth 32 bits of KEY2.
    #[inline(always)]
    pub const fn key2_data0(&self) -> u32 {
        (self.bits & RdKey2Data0::KEY2_DATA0_MASK) >> RdKey2Data0::KEY2_DATA0_OFFSET
    }
}

impl Default for RdKey2Data0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey2Data0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey2Data0Val {
        RdKey2Data0Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY2_DATA1 Register
///
/// Register $n of BLOCK6 (KEY2).
///
/// Address offset: 0x00e0
///
/// ## Fields
///
/// - **KEY2_DATA1** `[31:0]` (RO)
///
///   Stores the first 32 bits of KEY2.
#[repr(transparent)]
pub struct RdKey2Data1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey2Data1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey2Data1 {
    // ======== Field Constants ========

    /// **KEY2_DATA1**
    ///
    /// Stores the first 32 bits of KEY2.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY2_DATA1_OFFSET: u32 = 0;
    pub const KEY2_DATA1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY2_DATA1** `[31:0]` (RO)
    ///
    /// Stores the first 32 bits of KEY2.
    #[inline(always)]
    pub fn key2_data1(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_DATA1_MASK) >> Self::KEY2_DATA1_OFFSET
    }
}

/// RD_KEY2_DATA1 value type for batch read/modify/write operations
///
/// Use `rdkey2data1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey2data1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey2Data1Val {
    bits: u32,
}

impl RdKey2Data1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_DATA1** `[31:0]`
    ///
    /// Stores the first 32 bits of KEY2.
    #[inline(always)]
    pub const fn key2_data1(&self) -> u32 {
        (self.bits & RdKey2Data1::KEY2_DATA1_MASK) >> RdKey2Data1::KEY2_DATA1_OFFSET
    }
}

impl Default for RdKey2Data1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey2Data1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey2Data1Val {
        RdKey2Data1Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY2_DATA2 Register
///
/// Register $n of BLOCK6 (KEY2).
///
/// Address offset: 0x00e4
///
/// ## Fields
///
/// - **KEY2_DATA2** `[31:0]` (RO)
///
///   Stores the second 32 bits of KEY2.
#[repr(transparent)]
pub struct RdKey2Data2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey2Data2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey2Data2 {
    // ======== Field Constants ========

    /// **KEY2_DATA2**
    ///
    /// Stores the second 32 bits of KEY2.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY2_DATA2_OFFSET: u32 = 0;
    pub const KEY2_DATA2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY2_DATA2** `[31:0]` (RO)
    ///
    /// Stores the second 32 bits of KEY2.
    #[inline(always)]
    pub fn key2_data2(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_DATA2_MASK) >> Self::KEY2_DATA2_OFFSET
    }
}

/// RD_KEY2_DATA2 value type for batch read/modify/write operations
///
/// Use `rdkey2data2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey2data2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey2Data2Val {
    bits: u32,
}

impl RdKey2Data2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_DATA2** `[31:0]`
    ///
    /// Stores the second 32 bits of KEY2.
    #[inline(always)]
    pub const fn key2_data2(&self) -> u32 {
        (self.bits & RdKey2Data2::KEY2_DATA2_MASK) >> RdKey2Data2::KEY2_DATA2_OFFSET
    }
}

impl Default for RdKey2Data2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey2Data2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey2Data2Val {
        RdKey2Data2Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY2_DATA3 Register
///
/// Register $n of BLOCK6 (KEY2).
///
/// Address offset: 0x00e8
///
/// ## Fields
///
/// - **KEY2_DATA3** `[31:0]` (RO)
///
///   Stores the third 32 bits of KEY2.
#[repr(transparent)]
pub struct RdKey2Data3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey2Data3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey2Data3 {
    // ======== Field Constants ========

    /// **KEY2_DATA3**
    ///
    /// Stores the third 32 bits of KEY2.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY2_DATA3_OFFSET: u32 = 0;
    pub const KEY2_DATA3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY2_DATA3** `[31:0]` (RO)
    ///
    /// Stores the third 32 bits of KEY2.
    #[inline(always)]
    pub fn key2_data3(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_DATA3_MASK) >> Self::KEY2_DATA3_OFFSET
    }
}

/// RD_KEY2_DATA3 value type for batch read/modify/write operations
///
/// Use `rdkey2data3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey2data3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey2Data3Val {
    bits: u32,
}

impl RdKey2Data3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_DATA3** `[31:0]`
    ///
    /// Stores the third 32 bits of KEY2.
    #[inline(always)]
    pub const fn key2_data3(&self) -> u32 {
        (self.bits & RdKey2Data3::KEY2_DATA3_MASK) >> RdKey2Data3::KEY2_DATA3_OFFSET
    }
}

impl Default for RdKey2Data3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey2Data3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey2Data3Val {
        RdKey2Data3Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY2_DATA4 Register
///
/// Register $n of BLOCK6 (KEY2).
///
/// Address offset: 0x00ec
///
/// ## Fields
///
/// - **KEY2_DATA4** `[31:0]` (RO)
///
///   Stores the fourth 32 bits of KEY2.
#[repr(transparent)]
pub struct RdKey2Data4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey2Data4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey2Data4 {
    // ======== Field Constants ========

    /// **KEY2_DATA4**
    ///
    /// Stores the fourth 32 bits of KEY2.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY2_DATA4_OFFSET: u32 = 0;
    pub const KEY2_DATA4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY2_DATA4** `[31:0]` (RO)
    ///
    /// Stores the fourth 32 bits of KEY2.
    #[inline(always)]
    pub fn key2_data4(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_DATA4_MASK) >> Self::KEY2_DATA4_OFFSET
    }
}

/// RD_KEY2_DATA4 value type for batch read/modify/write operations
///
/// Use `rdkey2data4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey2data4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey2Data4Val {
    bits: u32,
}

impl RdKey2Data4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_DATA4** `[31:0]`
    ///
    /// Stores the fourth 32 bits of KEY2.
    #[inline(always)]
    pub const fn key2_data4(&self) -> u32 {
        (self.bits & RdKey2Data4::KEY2_DATA4_MASK) >> RdKey2Data4::KEY2_DATA4_OFFSET
    }
}

impl Default for RdKey2Data4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey2Data4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey2Data4Val {
        RdKey2Data4Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY2_DATA5 Register
///
/// Register $n of BLOCK6 (KEY2).
///
/// Address offset: 0x00f0
///
/// ## Fields
///
/// - **KEY2_DATA5** `[31:0]` (RO)
///
///   Stores the fifth 32 bits of KEY2.
#[repr(transparent)]
pub struct RdKey2Data5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey2Data5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey2Data5 {
    // ======== Field Constants ========

    /// **KEY2_DATA5**
    ///
    /// Stores the fifth 32 bits of KEY2.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY2_DATA5_OFFSET: u32 = 0;
    pub const KEY2_DATA5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY2_DATA5** `[31:0]` (RO)
    ///
    /// Stores the fifth 32 bits of KEY2.
    #[inline(always)]
    pub fn key2_data5(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_DATA5_MASK) >> Self::KEY2_DATA5_OFFSET
    }
}

/// RD_KEY2_DATA5 value type for batch read/modify/write operations
///
/// Use `rdkey2data5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey2data5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey2Data5Val {
    bits: u32,
}

impl RdKey2Data5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_DATA5** `[31:0]`
    ///
    /// Stores the fifth 32 bits of KEY2.
    #[inline(always)]
    pub const fn key2_data5(&self) -> u32 {
        (self.bits & RdKey2Data5::KEY2_DATA5_MASK) >> RdKey2Data5::KEY2_DATA5_OFFSET
    }
}

impl Default for RdKey2Data5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey2Data5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey2Data5Val {
        RdKey2Data5Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY2_DATA6 Register
///
/// Register $n of BLOCK6 (KEY2).
///
/// Address offset: 0x00f4
///
/// ## Fields
///
/// - **KEY2_DATA6** `[31:0]` (RO)
///
///   Stores the sixth 32 bits of KEY2.
#[repr(transparent)]
pub struct RdKey2Data6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey2Data6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey2Data6 {
    // ======== Field Constants ========

    /// **KEY2_DATA6**
    ///
    /// Stores the sixth 32 bits of KEY2.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY2_DATA6_OFFSET: u32 = 0;
    pub const KEY2_DATA6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY2_DATA6** `[31:0]` (RO)
    ///
    /// Stores the sixth 32 bits of KEY2.
    #[inline(always)]
    pub fn key2_data6(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_DATA6_MASK) >> Self::KEY2_DATA6_OFFSET
    }
}

/// RD_KEY2_DATA6 value type for batch read/modify/write operations
///
/// Use `rdkey2data6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey2data6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey2Data6Val {
    bits: u32,
}

impl RdKey2Data6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_DATA6** `[31:0]`
    ///
    /// Stores the sixth 32 bits of KEY2.
    #[inline(always)]
    pub const fn key2_data6(&self) -> u32 {
        (self.bits & RdKey2Data6::KEY2_DATA6_MASK) >> RdKey2Data6::KEY2_DATA6_OFFSET
    }
}

impl Default for RdKey2Data6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey2Data6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey2Data6Val {
        RdKey2Data6Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY2_DATA7 Register
///
/// Register $n of BLOCK6 (KEY2).
///
/// Address offset: 0x00f8
///
/// ## Fields
///
/// - **KEY2_DATA7** `[31:0]` (RO)
///
///   Stores the seventh 32 bits of KEY2.
#[repr(transparent)]
pub struct RdKey2Data7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey2Data7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey2Data7 {
    // ======== Field Constants ========

    /// **KEY2_DATA7**
    ///
    /// Stores the seventh 32 bits of KEY2.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY2_DATA7_OFFSET: u32 = 0;
    pub const KEY2_DATA7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY2_DATA7** `[31:0]` (RO)
    ///
    /// Stores the seventh 32 bits of KEY2.
    #[inline(always)]
    pub fn key2_data7(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_DATA7_MASK) >> Self::KEY2_DATA7_OFFSET
    }
}

/// RD_KEY2_DATA7 value type for batch read/modify/write operations
///
/// Use `rdkey2data7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey2data7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey2Data7Val {
    bits: u32,
}

impl RdKey2Data7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY2_DATA7** `[31:0]`
    ///
    /// Stores the seventh 32 bits of KEY2.
    #[inline(always)]
    pub const fn key2_data7(&self) -> u32 {
        (self.bits & RdKey2Data7::KEY2_DATA7_MASK) >> RdKey2Data7::KEY2_DATA7_OFFSET
    }
}

impl Default for RdKey2Data7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey2Data7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey2Data7Val {
        RdKey2Data7Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY3_DATA0 Register
///
/// Register $n of BLOCK7 (KEY3).
///
/// Address offset: 0x00fc
///
/// ## Fields
///
/// - **KEY3_DATA0** `[31:0]` (RO)
///
///   Stores the zeroth 32 bits of KEY3.
#[repr(transparent)]
pub struct RdKey3Data0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey3Data0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey3Data0 {
    // ======== Field Constants ========

    /// **KEY3_DATA0**
    ///
    /// Stores the zeroth 32 bits of KEY3.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY3_DATA0_OFFSET: u32 = 0;
    pub const KEY3_DATA0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY3_DATA0** `[31:0]` (RO)
    ///
    /// Stores the zeroth 32 bits of KEY3.
    #[inline(always)]
    pub fn key3_data0(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_DATA0_MASK) >> Self::KEY3_DATA0_OFFSET
    }
}

/// RD_KEY3_DATA0 value type for batch read/modify/write operations
///
/// Use `rdkey3data0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey3data0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey3Data0Val {
    bits: u32,
}

impl RdKey3Data0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_DATA0** `[31:0]`
    ///
    /// Stores the zeroth 32 bits of KEY3.
    #[inline(always)]
    pub const fn key3_data0(&self) -> u32 {
        (self.bits & RdKey3Data0::KEY3_DATA0_MASK) >> RdKey3Data0::KEY3_DATA0_OFFSET
    }
}

impl Default for RdKey3Data0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey3Data0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey3Data0Val {
        RdKey3Data0Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY3_DATA1 Register
///
/// Register $n of BLOCK7 (KEY3).
///
/// Address offset: 0x0100
///
/// ## Fields
///
/// - **KEY3_DATA1** `[31:0]` (RO)
///
///   Stores the first 32 bits of KEY3.
#[repr(transparent)]
pub struct RdKey3Data1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey3Data1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey3Data1 {
    // ======== Field Constants ========

    /// **KEY3_DATA1**
    ///
    /// Stores the first 32 bits of KEY3.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY3_DATA1_OFFSET: u32 = 0;
    pub const KEY3_DATA1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY3_DATA1** `[31:0]` (RO)
    ///
    /// Stores the first 32 bits of KEY3.
    #[inline(always)]
    pub fn key3_data1(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_DATA1_MASK) >> Self::KEY3_DATA1_OFFSET
    }
}

/// RD_KEY3_DATA1 value type for batch read/modify/write operations
///
/// Use `rdkey3data1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey3data1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey3Data1Val {
    bits: u32,
}

impl RdKey3Data1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_DATA1** `[31:0]`
    ///
    /// Stores the first 32 bits of KEY3.
    #[inline(always)]
    pub const fn key3_data1(&self) -> u32 {
        (self.bits & RdKey3Data1::KEY3_DATA1_MASK) >> RdKey3Data1::KEY3_DATA1_OFFSET
    }
}

impl Default for RdKey3Data1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey3Data1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey3Data1Val {
        RdKey3Data1Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY3_DATA2 Register
///
/// Register $n of BLOCK7 (KEY3).
///
/// Address offset: 0x0104
///
/// ## Fields
///
/// - **KEY3_DATA2** `[31:0]` (RO)
///
///   Stores the second 32 bits of KEY3.
#[repr(transparent)]
pub struct RdKey3Data2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey3Data2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey3Data2 {
    // ======== Field Constants ========

    /// **KEY3_DATA2**
    ///
    /// Stores the second 32 bits of KEY3.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY3_DATA2_OFFSET: u32 = 0;
    pub const KEY3_DATA2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY3_DATA2** `[31:0]` (RO)
    ///
    /// Stores the second 32 bits of KEY3.
    #[inline(always)]
    pub fn key3_data2(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_DATA2_MASK) >> Self::KEY3_DATA2_OFFSET
    }
}

/// RD_KEY3_DATA2 value type for batch read/modify/write operations
///
/// Use `rdkey3data2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey3data2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey3Data2Val {
    bits: u32,
}

impl RdKey3Data2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_DATA2** `[31:0]`
    ///
    /// Stores the second 32 bits of KEY3.
    #[inline(always)]
    pub const fn key3_data2(&self) -> u32 {
        (self.bits & RdKey3Data2::KEY3_DATA2_MASK) >> RdKey3Data2::KEY3_DATA2_OFFSET
    }
}

impl Default for RdKey3Data2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey3Data2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey3Data2Val {
        RdKey3Data2Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY3_DATA3 Register
///
/// Register $n of BLOCK7 (KEY3).
///
/// Address offset: 0x0108
///
/// ## Fields
///
/// - **KEY3_DATA3** `[31:0]` (RO)
///
///   Stores the third 32 bits of KEY3.
#[repr(transparent)]
pub struct RdKey3Data3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey3Data3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey3Data3 {
    // ======== Field Constants ========

    /// **KEY3_DATA3**
    ///
    /// Stores the third 32 bits of KEY3.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY3_DATA3_OFFSET: u32 = 0;
    pub const KEY3_DATA3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY3_DATA3** `[31:0]` (RO)
    ///
    /// Stores the third 32 bits of KEY3.
    #[inline(always)]
    pub fn key3_data3(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_DATA3_MASK) >> Self::KEY3_DATA3_OFFSET
    }
}

/// RD_KEY3_DATA3 value type for batch read/modify/write operations
///
/// Use `rdkey3data3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey3data3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey3Data3Val {
    bits: u32,
}

impl RdKey3Data3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_DATA3** `[31:0]`
    ///
    /// Stores the third 32 bits of KEY3.
    #[inline(always)]
    pub const fn key3_data3(&self) -> u32 {
        (self.bits & RdKey3Data3::KEY3_DATA3_MASK) >> RdKey3Data3::KEY3_DATA3_OFFSET
    }
}

impl Default for RdKey3Data3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey3Data3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey3Data3Val {
        RdKey3Data3Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY3_DATA4 Register
///
/// Register $n of BLOCK7 (KEY3).
///
/// Address offset: 0x010c
///
/// ## Fields
///
/// - **KEY3_DATA4** `[31:0]` (RO)
///
///   Stores the fourth 32 bits of KEY3.
#[repr(transparent)]
pub struct RdKey3Data4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey3Data4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey3Data4 {
    // ======== Field Constants ========

    /// **KEY3_DATA4**
    ///
    /// Stores the fourth 32 bits of KEY3.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY3_DATA4_OFFSET: u32 = 0;
    pub const KEY3_DATA4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY3_DATA4** `[31:0]` (RO)
    ///
    /// Stores the fourth 32 bits of KEY3.
    #[inline(always)]
    pub fn key3_data4(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_DATA4_MASK) >> Self::KEY3_DATA4_OFFSET
    }
}

/// RD_KEY3_DATA4 value type for batch read/modify/write operations
///
/// Use `rdkey3data4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey3data4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey3Data4Val {
    bits: u32,
}

impl RdKey3Data4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_DATA4** `[31:0]`
    ///
    /// Stores the fourth 32 bits of KEY3.
    #[inline(always)]
    pub const fn key3_data4(&self) -> u32 {
        (self.bits & RdKey3Data4::KEY3_DATA4_MASK) >> RdKey3Data4::KEY3_DATA4_OFFSET
    }
}

impl Default for RdKey3Data4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey3Data4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey3Data4Val {
        RdKey3Data4Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY3_DATA5 Register
///
/// Register $n of BLOCK7 (KEY3).
///
/// Address offset: 0x0110
///
/// ## Fields
///
/// - **KEY3_DATA5** `[31:0]` (RO)
///
///   Stores the fifth 32 bits of KEY3.
#[repr(transparent)]
pub struct RdKey3Data5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey3Data5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey3Data5 {
    // ======== Field Constants ========

    /// **KEY3_DATA5**
    ///
    /// Stores the fifth 32 bits of KEY3.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY3_DATA5_OFFSET: u32 = 0;
    pub const KEY3_DATA5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY3_DATA5** `[31:0]` (RO)
    ///
    /// Stores the fifth 32 bits of KEY3.
    #[inline(always)]
    pub fn key3_data5(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_DATA5_MASK) >> Self::KEY3_DATA5_OFFSET
    }
}

/// RD_KEY3_DATA5 value type for batch read/modify/write operations
///
/// Use `rdkey3data5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey3data5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey3Data5Val {
    bits: u32,
}

impl RdKey3Data5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_DATA5** `[31:0]`
    ///
    /// Stores the fifth 32 bits of KEY3.
    #[inline(always)]
    pub const fn key3_data5(&self) -> u32 {
        (self.bits & RdKey3Data5::KEY3_DATA5_MASK) >> RdKey3Data5::KEY3_DATA5_OFFSET
    }
}

impl Default for RdKey3Data5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey3Data5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey3Data5Val {
        RdKey3Data5Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY3_DATA6 Register
///
/// Register $n of BLOCK7 (KEY3).
///
/// Address offset: 0x0114
///
/// ## Fields
///
/// - **KEY3_DATA6** `[31:0]` (RO)
///
///   Stores the sixth 32 bits of KEY3.
#[repr(transparent)]
pub struct RdKey3Data6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey3Data6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey3Data6 {
    // ======== Field Constants ========

    /// **KEY3_DATA6**
    ///
    /// Stores the sixth 32 bits of KEY3.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY3_DATA6_OFFSET: u32 = 0;
    pub const KEY3_DATA6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY3_DATA6** `[31:0]` (RO)
    ///
    /// Stores the sixth 32 bits of KEY3.
    #[inline(always)]
    pub fn key3_data6(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_DATA6_MASK) >> Self::KEY3_DATA6_OFFSET
    }
}

/// RD_KEY3_DATA6 value type for batch read/modify/write operations
///
/// Use `rdkey3data6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey3data6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey3Data6Val {
    bits: u32,
}

impl RdKey3Data6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_DATA6** `[31:0]`
    ///
    /// Stores the sixth 32 bits of KEY3.
    #[inline(always)]
    pub const fn key3_data6(&self) -> u32 {
        (self.bits & RdKey3Data6::KEY3_DATA6_MASK) >> RdKey3Data6::KEY3_DATA6_OFFSET
    }
}

impl Default for RdKey3Data6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey3Data6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey3Data6Val {
        RdKey3Data6Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY3_DATA7 Register
///
/// Register $n of BLOCK7 (KEY3).
///
/// Address offset: 0x0118
///
/// ## Fields
///
/// - **KEY3_DATA7** `[31:0]` (RO)
///
///   Stores the seventh 32 bits of KEY3.
#[repr(transparent)]
pub struct RdKey3Data7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey3Data7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey3Data7 {
    // ======== Field Constants ========

    /// **KEY3_DATA7**
    ///
    /// Stores the seventh 32 bits of KEY3.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY3_DATA7_OFFSET: u32 = 0;
    pub const KEY3_DATA7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY3_DATA7** `[31:0]` (RO)
    ///
    /// Stores the seventh 32 bits of KEY3.
    #[inline(always)]
    pub fn key3_data7(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_DATA7_MASK) >> Self::KEY3_DATA7_OFFSET
    }
}

/// RD_KEY3_DATA7 value type for batch read/modify/write operations
///
/// Use `rdkey3data7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey3data7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey3Data7Val {
    bits: u32,
}

impl RdKey3Data7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY3_DATA7** `[31:0]`
    ///
    /// Stores the seventh 32 bits of KEY3.
    #[inline(always)]
    pub const fn key3_data7(&self) -> u32 {
        (self.bits & RdKey3Data7::KEY3_DATA7_MASK) >> RdKey3Data7::KEY3_DATA7_OFFSET
    }
}

impl Default for RdKey3Data7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey3Data7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey3Data7Val {
        RdKey3Data7Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY4_DATA0 Register
///
/// Register $n of BLOCK8 (KEY4).
///
/// Address offset: 0x011c
///
/// ## Fields
///
/// - **KEY4_DATA0** `[31:0]` (RO)
///
///   Stores the zeroth 32 bits of KEY4.
#[repr(transparent)]
pub struct RdKey4Data0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey4Data0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey4Data0 {
    // ======== Field Constants ========

    /// **KEY4_DATA0**
    ///
    /// Stores the zeroth 32 bits of KEY4.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY4_DATA0_OFFSET: u32 = 0;
    pub const KEY4_DATA0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY4_DATA0** `[31:0]` (RO)
    ///
    /// Stores the zeroth 32 bits of KEY4.
    #[inline(always)]
    pub fn key4_data0(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_DATA0_MASK) >> Self::KEY4_DATA0_OFFSET
    }
}

/// RD_KEY4_DATA0 value type for batch read/modify/write operations
///
/// Use `rdkey4data0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey4data0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey4Data0Val {
    bits: u32,
}

impl RdKey4Data0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_DATA0** `[31:0]`
    ///
    /// Stores the zeroth 32 bits of KEY4.
    #[inline(always)]
    pub const fn key4_data0(&self) -> u32 {
        (self.bits & RdKey4Data0::KEY4_DATA0_MASK) >> RdKey4Data0::KEY4_DATA0_OFFSET
    }
}

impl Default for RdKey4Data0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey4Data0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey4Data0Val {
        RdKey4Data0Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY4_DATA1 Register
///
/// Register $n of BLOCK8 (KEY4).
///
/// Address offset: 0x0120
///
/// ## Fields
///
/// - **KEY4_DATA1** `[31:0]` (RO)
///
///   Stores the first 32 bits of KEY4.
#[repr(transparent)]
pub struct RdKey4Data1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey4Data1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey4Data1 {
    // ======== Field Constants ========

    /// **KEY4_DATA1**
    ///
    /// Stores the first 32 bits of KEY4.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY4_DATA1_OFFSET: u32 = 0;
    pub const KEY4_DATA1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY4_DATA1** `[31:0]` (RO)
    ///
    /// Stores the first 32 bits of KEY4.
    #[inline(always)]
    pub fn key4_data1(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_DATA1_MASK) >> Self::KEY4_DATA1_OFFSET
    }
}

/// RD_KEY4_DATA1 value type for batch read/modify/write operations
///
/// Use `rdkey4data1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey4data1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey4Data1Val {
    bits: u32,
}

impl RdKey4Data1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_DATA1** `[31:0]`
    ///
    /// Stores the first 32 bits of KEY4.
    #[inline(always)]
    pub const fn key4_data1(&self) -> u32 {
        (self.bits & RdKey4Data1::KEY4_DATA1_MASK) >> RdKey4Data1::KEY4_DATA1_OFFSET
    }
}

impl Default for RdKey4Data1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey4Data1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey4Data1Val {
        RdKey4Data1Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY4_DATA2 Register
///
/// Register $n of BLOCK8 (KEY4).
///
/// Address offset: 0x0124
///
/// ## Fields
///
/// - **KEY4_DATA2** `[31:0]` (RO)
///
///   Stores the second 32 bits of KEY4.
#[repr(transparent)]
pub struct RdKey4Data2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey4Data2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey4Data2 {
    // ======== Field Constants ========

    /// **KEY4_DATA2**
    ///
    /// Stores the second 32 bits of KEY4.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY4_DATA2_OFFSET: u32 = 0;
    pub const KEY4_DATA2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY4_DATA2** `[31:0]` (RO)
    ///
    /// Stores the second 32 bits of KEY4.
    #[inline(always)]
    pub fn key4_data2(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_DATA2_MASK) >> Self::KEY4_DATA2_OFFSET
    }
}

/// RD_KEY4_DATA2 value type for batch read/modify/write operations
///
/// Use `rdkey4data2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey4data2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey4Data2Val {
    bits: u32,
}

impl RdKey4Data2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_DATA2** `[31:0]`
    ///
    /// Stores the second 32 bits of KEY4.
    #[inline(always)]
    pub const fn key4_data2(&self) -> u32 {
        (self.bits & RdKey4Data2::KEY4_DATA2_MASK) >> RdKey4Data2::KEY4_DATA2_OFFSET
    }
}

impl Default for RdKey4Data2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey4Data2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey4Data2Val {
        RdKey4Data2Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY4_DATA3 Register
///
/// Register $n of BLOCK8 (KEY4).
///
/// Address offset: 0x0128
///
/// ## Fields
///
/// - **KEY4_DATA3** `[31:0]` (RO)
///
///   Stores the third 32 bits of KEY4.
#[repr(transparent)]
pub struct RdKey4Data3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey4Data3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey4Data3 {
    // ======== Field Constants ========

    /// **KEY4_DATA3**
    ///
    /// Stores the third 32 bits of KEY4.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY4_DATA3_OFFSET: u32 = 0;
    pub const KEY4_DATA3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY4_DATA3** `[31:0]` (RO)
    ///
    /// Stores the third 32 bits of KEY4.
    #[inline(always)]
    pub fn key4_data3(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_DATA3_MASK) >> Self::KEY4_DATA3_OFFSET
    }
}

/// RD_KEY4_DATA3 value type for batch read/modify/write operations
///
/// Use `rdkey4data3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey4data3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey4Data3Val {
    bits: u32,
}

impl RdKey4Data3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_DATA3** `[31:0]`
    ///
    /// Stores the third 32 bits of KEY4.
    #[inline(always)]
    pub const fn key4_data3(&self) -> u32 {
        (self.bits & RdKey4Data3::KEY4_DATA3_MASK) >> RdKey4Data3::KEY4_DATA3_OFFSET
    }
}

impl Default for RdKey4Data3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey4Data3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey4Data3Val {
        RdKey4Data3Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY4_DATA4 Register
///
/// Register $n of BLOCK8 (KEY4).
///
/// Address offset: 0x012c
///
/// ## Fields
///
/// - **KEY4_DATA4** `[31:0]` (RO)
///
///   Stores the fourth 32 bits of KEY4.
#[repr(transparent)]
pub struct RdKey4Data4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey4Data4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey4Data4 {
    // ======== Field Constants ========

    /// **KEY4_DATA4**
    ///
    /// Stores the fourth 32 bits of KEY4.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY4_DATA4_OFFSET: u32 = 0;
    pub const KEY4_DATA4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY4_DATA4** `[31:0]` (RO)
    ///
    /// Stores the fourth 32 bits of KEY4.
    #[inline(always)]
    pub fn key4_data4(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_DATA4_MASK) >> Self::KEY4_DATA4_OFFSET
    }
}

/// RD_KEY4_DATA4 value type for batch read/modify/write operations
///
/// Use `rdkey4data4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey4data4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey4Data4Val {
    bits: u32,
}

impl RdKey4Data4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_DATA4** `[31:0]`
    ///
    /// Stores the fourth 32 bits of KEY4.
    #[inline(always)]
    pub const fn key4_data4(&self) -> u32 {
        (self.bits & RdKey4Data4::KEY4_DATA4_MASK) >> RdKey4Data4::KEY4_DATA4_OFFSET
    }
}

impl Default for RdKey4Data4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey4Data4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey4Data4Val {
        RdKey4Data4Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY4_DATA5 Register
///
/// Register $n of BLOCK8 (KEY4).
///
/// Address offset: 0x0130
///
/// ## Fields
///
/// - **KEY4_DATA5** `[31:0]` (RO)
///
///   Stores the fifth 32 bits of KEY4.
#[repr(transparent)]
pub struct RdKey4Data5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey4Data5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey4Data5 {
    // ======== Field Constants ========

    /// **KEY4_DATA5**
    ///
    /// Stores the fifth 32 bits of KEY4.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY4_DATA5_OFFSET: u32 = 0;
    pub const KEY4_DATA5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY4_DATA5** `[31:0]` (RO)
    ///
    /// Stores the fifth 32 bits of KEY4.
    #[inline(always)]
    pub fn key4_data5(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_DATA5_MASK) >> Self::KEY4_DATA5_OFFSET
    }
}

/// RD_KEY4_DATA5 value type for batch read/modify/write operations
///
/// Use `rdkey4data5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey4data5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey4Data5Val {
    bits: u32,
}

impl RdKey4Data5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_DATA5** `[31:0]`
    ///
    /// Stores the fifth 32 bits of KEY4.
    #[inline(always)]
    pub const fn key4_data5(&self) -> u32 {
        (self.bits & RdKey4Data5::KEY4_DATA5_MASK) >> RdKey4Data5::KEY4_DATA5_OFFSET
    }
}

impl Default for RdKey4Data5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey4Data5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey4Data5Val {
        RdKey4Data5Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY4_DATA6 Register
///
/// Register $n of BLOCK8 (KEY4).
///
/// Address offset: 0x0134
///
/// ## Fields
///
/// - **KEY4_DATA6** `[31:0]` (RO)
///
///   Stores the sixth 32 bits of KEY4.
#[repr(transparent)]
pub struct RdKey4Data6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey4Data6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey4Data6 {
    // ======== Field Constants ========

    /// **KEY4_DATA6**
    ///
    /// Stores the sixth 32 bits of KEY4.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY4_DATA6_OFFSET: u32 = 0;
    pub const KEY4_DATA6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY4_DATA6** `[31:0]` (RO)
    ///
    /// Stores the sixth 32 bits of KEY4.
    #[inline(always)]
    pub fn key4_data6(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_DATA6_MASK) >> Self::KEY4_DATA6_OFFSET
    }
}

/// RD_KEY4_DATA6 value type for batch read/modify/write operations
///
/// Use `rdkey4data6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey4data6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey4Data6Val {
    bits: u32,
}

impl RdKey4Data6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_DATA6** `[31:0]`
    ///
    /// Stores the sixth 32 bits of KEY4.
    #[inline(always)]
    pub const fn key4_data6(&self) -> u32 {
        (self.bits & RdKey4Data6::KEY4_DATA6_MASK) >> RdKey4Data6::KEY4_DATA6_OFFSET
    }
}

impl Default for RdKey4Data6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey4Data6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey4Data6Val {
        RdKey4Data6Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY4_DATA7 Register
///
/// Register $n of BLOCK8 (KEY4).
///
/// Address offset: 0x0138
///
/// ## Fields
///
/// - **KEY4_DATA7** `[31:0]` (RO)
///
///   Stores the seventh 32 bits of KEY4.
#[repr(transparent)]
pub struct RdKey4Data7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey4Data7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey4Data7 {
    // ======== Field Constants ========

    /// **KEY4_DATA7**
    ///
    /// Stores the seventh 32 bits of KEY4.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY4_DATA7_OFFSET: u32 = 0;
    pub const KEY4_DATA7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY4_DATA7** `[31:0]` (RO)
    ///
    /// Stores the seventh 32 bits of KEY4.
    #[inline(always)]
    pub fn key4_data7(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_DATA7_MASK) >> Self::KEY4_DATA7_OFFSET
    }
}

/// RD_KEY4_DATA7 value type for batch read/modify/write operations
///
/// Use `rdkey4data7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey4data7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey4Data7Val {
    bits: u32,
}

impl RdKey4Data7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_DATA7** `[31:0]`
    ///
    /// Stores the seventh 32 bits of KEY4.
    #[inline(always)]
    pub const fn key4_data7(&self) -> u32 {
        (self.bits & RdKey4Data7::KEY4_DATA7_MASK) >> RdKey4Data7::KEY4_DATA7_OFFSET
    }
}

impl Default for RdKey4Data7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey4Data7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey4Data7Val {
        RdKey4Data7Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY5_DATA0 Register
///
/// Register $n of BLOCK9 (KEY5).
///
/// Address offset: 0x013c
///
/// ## Fields
///
/// - **KEY5_DATA0** `[31:0]` (RO)
///
///   Stores the zeroth 32 bits of KEY5.
#[repr(transparent)]
pub struct RdKey5Data0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey5Data0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey5Data0 {
    // ======== Field Constants ========

    /// **KEY5_DATA0**
    ///
    /// Stores the zeroth 32 bits of KEY5.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY5_DATA0_OFFSET: u32 = 0;
    pub const KEY5_DATA0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY5_DATA0** `[31:0]` (RO)
    ///
    /// Stores the zeroth 32 bits of KEY5.
    #[inline(always)]
    pub fn key5_data0(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_DATA0_MASK) >> Self::KEY5_DATA0_OFFSET
    }
}

/// RD_KEY5_DATA0 value type for batch read/modify/write operations
///
/// Use `rdkey5data0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey5data0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey5Data0Val {
    bits: u32,
}

impl RdKey5Data0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_DATA0** `[31:0]`
    ///
    /// Stores the zeroth 32 bits of KEY5.
    #[inline(always)]
    pub const fn key5_data0(&self) -> u32 {
        (self.bits & RdKey5Data0::KEY5_DATA0_MASK) >> RdKey5Data0::KEY5_DATA0_OFFSET
    }
}

impl Default for RdKey5Data0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey5Data0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey5Data0Val {
        RdKey5Data0Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY5_DATA1 Register
///
/// Register $n of BLOCK9 (KEY5).
///
/// Address offset: 0x0140
///
/// ## Fields
///
/// - **KEY5_DATA1** `[31:0]` (RO)
///
///   Stores the first 32 bits of KEY5.
#[repr(transparent)]
pub struct RdKey5Data1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey5Data1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey5Data1 {
    // ======== Field Constants ========

    /// **KEY5_DATA1**
    ///
    /// Stores the first 32 bits of KEY5.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY5_DATA1_OFFSET: u32 = 0;
    pub const KEY5_DATA1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY5_DATA1** `[31:0]` (RO)
    ///
    /// Stores the first 32 bits of KEY5.
    #[inline(always)]
    pub fn key5_data1(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_DATA1_MASK) >> Self::KEY5_DATA1_OFFSET
    }
}

/// RD_KEY5_DATA1 value type for batch read/modify/write operations
///
/// Use `rdkey5data1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey5data1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey5Data1Val {
    bits: u32,
}

impl RdKey5Data1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_DATA1** `[31:0]`
    ///
    /// Stores the first 32 bits of KEY5.
    #[inline(always)]
    pub const fn key5_data1(&self) -> u32 {
        (self.bits & RdKey5Data1::KEY5_DATA1_MASK) >> RdKey5Data1::KEY5_DATA1_OFFSET
    }
}

impl Default for RdKey5Data1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey5Data1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey5Data1Val {
        RdKey5Data1Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY5_DATA2 Register
///
/// Register $n of BLOCK9 (KEY5).
///
/// Address offset: 0x0144
///
/// ## Fields
///
/// - **KEY5_DATA2** `[31:0]` (RO)
///
///   Stores the second 32 bits of KEY5.
#[repr(transparent)]
pub struct RdKey5Data2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey5Data2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey5Data2 {
    // ======== Field Constants ========

    /// **KEY5_DATA2**
    ///
    /// Stores the second 32 bits of KEY5.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY5_DATA2_OFFSET: u32 = 0;
    pub const KEY5_DATA2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY5_DATA2** `[31:0]` (RO)
    ///
    /// Stores the second 32 bits of KEY5.
    #[inline(always)]
    pub fn key5_data2(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_DATA2_MASK) >> Self::KEY5_DATA2_OFFSET
    }
}

/// RD_KEY5_DATA2 value type for batch read/modify/write operations
///
/// Use `rdkey5data2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey5data2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey5Data2Val {
    bits: u32,
}

impl RdKey5Data2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_DATA2** `[31:0]`
    ///
    /// Stores the second 32 bits of KEY5.
    #[inline(always)]
    pub const fn key5_data2(&self) -> u32 {
        (self.bits & RdKey5Data2::KEY5_DATA2_MASK) >> RdKey5Data2::KEY5_DATA2_OFFSET
    }
}

impl Default for RdKey5Data2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey5Data2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey5Data2Val {
        RdKey5Data2Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY5_DATA3 Register
///
/// Register $n of BLOCK9 (KEY5).
///
/// Address offset: 0x0148
///
/// ## Fields
///
/// - **KEY5_DATA3** `[31:0]` (RO)
///
///   Stores the third 32 bits of KEY5.
#[repr(transparent)]
pub struct RdKey5Data3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey5Data3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey5Data3 {
    // ======== Field Constants ========

    /// **KEY5_DATA3**
    ///
    /// Stores the third 32 bits of KEY5.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY5_DATA3_OFFSET: u32 = 0;
    pub const KEY5_DATA3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY5_DATA3** `[31:0]` (RO)
    ///
    /// Stores the third 32 bits of KEY5.
    #[inline(always)]
    pub fn key5_data3(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_DATA3_MASK) >> Self::KEY5_DATA3_OFFSET
    }
}

/// RD_KEY5_DATA3 value type for batch read/modify/write operations
///
/// Use `rdkey5data3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey5data3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey5Data3Val {
    bits: u32,
}

impl RdKey5Data3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_DATA3** `[31:0]`
    ///
    /// Stores the third 32 bits of KEY5.
    #[inline(always)]
    pub const fn key5_data3(&self) -> u32 {
        (self.bits & RdKey5Data3::KEY5_DATA3_MASK) >> RdKey5Data3::KEY5_DATA3_OFFSET
    }
}

impl Default for RdKey5Data3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey5Data3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey5Data3Val {
        RdKey5Data3Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY5_DATA4 Register
///
/// Register $n of BLOCK9 (KEY5).
///
/// Address offset: 0x014c
///
/// ## Fields
///
/// - **KEY5_DATA4** `[31:0]` (RO)
///
///   Stores the fourth 32 bits of KEY5.
#[repr(transparent)]
pub struct RdKey5Data4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey5Data4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey5Data4 {
    // ======== Field Constants ========

    /// **KEY5_DATA4**
    ///
    /// Stores the fourth 32 bits of KEY5.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY5_DATA4_OFFSET: u32 = 0;
    pub const KEY5_DATA4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY5_DATA4** `[31:0]` (RO)
    ///
    /// Stores the fourth 32 bits of KEY5.
    #[inline(always)]
    pub fn key5_data4(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_DATA4_MASK) >> Self::KEY5_DATA4_OFFSET
    }
}

/// RD_KEY5_DATA4 value type for batch read/modify/write operations
///
/// Use `rdkey5data4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey5data4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey5Data4Val {
    bits: u32,
}

impl RdKey5Data4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_DATA4** `[31:0]`
    ///
    /// Stores the fourth 32 bits of KEY5.
    #[inline(always)]
    pub const fn key5_data4(&self) -> u32 {
        (self.bits & RdKey5Data4::KEY5_DATA4_MASK) >> RdKey5Data4::KEY5_DATA4_OFFSET
    }
}

impl Default for RdKey5Data4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey5Data4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey5Data4Val {
        RdKey5Data4Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY5_DATA5 Register
///
/// Register $n of BLOCK9 (KEY5).
///
/// Address offset: 0x0150
///
/// ## Fields
///
/// - **KEY5_DATA5** `[31:0]` (RO)
///
///   Stores the fifth 32 bits of KEY5.
#[repr(transparent)]
pub struct RdKey5Data5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey5Data5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey5Data5 {
    // ======== Field Constants ========

    /// **KEY5_DATA5**
    ///
    /// Stores the fifth 32 bits of KEY5.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY5_DATA5_OFFSET: u32 = 0;
    pub const KEY5_DATA5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY5_DATA5** `[31:0]` (RO)
    ///
    /// Stores the fifth 32 bits of KEY5.
    #[inline(always)]
    pub fn key5_data5(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_DATA5_MASK) >> Self::KEY5_DATA5_OFFSET
    }
}

/// RD_KEY5_DATA5 value type for batch read/modify/write operations
///
/// Use `rdkey5data5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey5data5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey5Data5Val {
    bits: u32,
}

impl RdKey5Data5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_DATA5** `[31:0]`
    ///
    /// Stores the fifth 32 bits of KEY5.
    #[inline(always)]
    pub const fn key5_data5(&self) -> u32 {
        (self.bits & RdKey5Data5::KEY5_DATA5_MASK) >> RdKey5Data5::KEY5_DATA5_OFFSET
    }
}

impl Default for RdKey5Data5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey5Data5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey5Data5Val {
        RdKey5Data5Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY5_DATA6 Register
///
/// Register $n of BLOCK9 (KEY5).
///
/// Address offset: 0x0154
///
/// ## Fields
///
/// - **KEY5_DATA6** `[31:0]` (RO)
///
///   Stores the sixth 32 bits of KEY5.
#[repr(transparent)]
pub struct RdKey5Data6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey5Data6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey5Data6 {
    // ======== Field Constants ========

    /// **KEY5_DATA6**
    ///
    /// Stores the sixth 32 bits of KEY5.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY5_DATA6_OFFSET: u32 = 0;
    pub const KEY5_DATA6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY5_DATA6** `[31:0]` (RO)
    ///
    /// Stores the sixth 32 bits of KEY5.
    #[inline(always)]
    pub fn key5_data6(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_DATA6_MASK) >> Self::KEY5_DATA6_OFFSET
    }
}

/// RD_KEY5_DATA6 value type for batch read/modify/write operations
///
/// Use `rdkey5data6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey5data6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey5Data6Val {
    bits: u32,
}

impl RdKey5Data6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_DATA6** `[31:0]`
    ///
    /// Stores the sixth 32 bits of KEY5.
    #[inline(always)]
    pub const fn key5_data6(&self) -> u32 {
        (self.bits & RdKey5Data6::KEY5_DATA6_MASK) >> RdKey5Data6::KEY5_DATA6_OFFSET
    }
}

impl Default for RdKey5Data6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey5Data6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey5Data6Val {
        RdKey5Data6Val::from_bits(self.value.read_bits())
    }
}

/// RD_KEY5_DATA7 Register
///
/// Register $n of BLOCK9 (KEY5).
///
/// Address offset: 0x0158
///
/// ## Fields
///
/// - **KEY5_DATA7** `[31:0]` (RO)
///
///   Stores the seventh 32 bits of KEY5.
#[repr(transparent)]
pub struct RdKey5Data7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdKey5Data7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdKey5Data7 {
    // ======== Field Constants ========

    /// **KEY5_DATA7**
    ///
    /// Stores the seventh 32 bits of KEY5.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY5_DATA7_OFFSET: u32 = 0;
    pub const KEY5_DATA7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY5_DATA7** `[31:0]` (RO)
    ///
    /// Stores the seventh 32 bits of KEY5.
    #[inline(always)]
    pub fn key5_data7(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_DATA7_MASK) >> Self::KEY5_DATA7_OFFSET
    }
}

/// RD_KEY5_DATA7 value type for batch read/modify/write operations
///
/// Use `rdkey5data7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdkey5data7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdKey5Data7Val {
    bits: u32,
}

impl RdKey5Data7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY5_DATA7** `[31:0]`
    ///
    /// Stores the seventh 32 bits of KEY5.
    #[inline(always)]
    pub const fn key5_data7(&self) -> u32 {
        (self.bits & RdKey5Data7::KEY5_DATA7_MASK) >> RdKey5Data7::KEY5_DATA7_OFFSET
    }
}

impl Default for RdKey5Data7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdKey5Data7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdKey5Data7Val {
        RdKey5Data7Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART2_DATA0 Register
///
/// Register $n of BLOCK10 (system).
///
/// Address offset: 0x015c
///
/// ## Fields
///
/// - **SYS_DATA_PART2_0** `[31:0]` (RO)
///
///   Stores the 0th 32 bits of the 2nd part of system data.
#[repr(transparent)]
pub struct RdSysPart2Data0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart2Data0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart2Data0 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART2_0**
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART2_0_OFFSET: u32 = 0;
    pub const SYS_DATA_PART2_0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART2_0** `[31:0]` (RO)
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub fn sys_data_part2_0(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART2_0_MASK) >> Self::SYS_DATA_PART2_0_OFFSET
    }
}

/// RD_SYS_PART2_DATA0 value type for batch read/modify/write operations
///
/// Use `rdsyspart2data0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart2data0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart2Data0Val {
    bits: u32,
}

impl RdSysPart2Data0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART2_0** `[31:0]`
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub const fn sys_data_part2_0(&self) -> u32 {
        (self.bits & RdSysPart2Data0::SYS_DATA_PART2_0_MASK)
            >> RdSysPart2Data0::SYS_DATA_PART2_0_OFFSET
    }
}

impl Default for RdSysPart2Data0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart2Data0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart2Data0Val {
        RdSysPart2Data0Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART2_DATA1 Register
///
/// Register $n of BLOCK9 (KEY5).
///
/// Address offset: 0x0160
///
/// ## Fields
///
/// - **SYS_DATA_PART2_1** `[31:0]` (RO)
///
///   Stores the 0th 32 bits of the 2nd part of system data.
#[repr(transparent)]
pub struct RdSysPart2Data1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart2Data1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart2Data1 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART2_1**
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART2_1_OFFSET: u32 = 0;
    pub const SYS_DATA_PART2_1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART2_1** `[31:0]` (RO)
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub fn sys_data_part2_1(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART2_1_MASK) >> Self::SYS_DATA_PART2_1_OFFSET
    }
}

/// RD_SYS_PART2_DATA1 value type for batch read/modify/write operations
///
/// Use `rdsyspart2data1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart2data1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart2Data1Val {
    bits: u32,
}

impl RdSysPart2Data1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART2_1** `[31:0]`
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub const fn sys_data_part2_1(&self) -> u32 {
        (self.bits & RdSysPart2Data1::SYS_DATA_PART2_1_MASK)
            >> RdSysPart2Data1::SYS_DATA_PART2_1_OFFSET
    }
}

impl Default for RdSysPart2Data1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart2Data1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart2Data1Val {
        RdSysPart2Data1Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART2_DATA2 Register
///
/// Register $n of BLOCK10 (system).
///
/// Address offset: 0x0164
///
/// ## Fields
///
/// - **SYS_DATA_PART2_2** `[31:0]` (RO)
///
///   Stores the 0th 32 bits of the 2nd part of system data.
#[repr(transparent)]
pub struct RdSysPart2Data2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart2Data2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart2Data2 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART2_2**
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART2_2_OFFSET: u32 = 0;
    pub const SYS_DATA_PART2_2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART2_2** `[31:0]` (RO)
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub fn sys_data_part2_2(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART2_2_MASK) >> Self::SYS_DATA_PART2_2_OFFSET
    }
}

/// RD_SYS_PART2_DATA2 value type for batch read/modify/write operations
///
/// Use `rdsyspart2data2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart2data2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart2Data2Val {
    bits: u32,
}

impl RdSysPart2Data2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART2_2** `[31:0]`
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub const fn sys_data_part2_2(&self) -> u32 {
        (self.bits & RdSysPart2Data2::SYS_DATA_PART2_2_MASK)
            >> RdSysPart2Data2::SYS_DATA_PART2_2_OFFSET
    }
}

impl Default for RdSysPart2Data2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart2Data2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart2Data2Val {
        RdSysPart2Data2Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART2_DATA3 Register
///
/// Register $n of BLOCK10 (system).
///
/// Address offset: 0x0168
///
/// ## Fields
///
/// - **SYS_DATA_PART2_3** `[31:0]` (RO)
///
///   Stores the 0th 32 bits of the 2nd part of system data.
#[repr(transparent)]
pub struct RdSysPart2Data3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart2Data3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart2Data3 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART2_3**
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART2_3_OFFSET: u32 = 0;
    pub const SYS_DATA_PART2_3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART2_3** `[31:0]` (RO)
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub fn sys_data_part2_3(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART2_3_MASK) >> Self::SYS_DATA_PART2_3_OFFSET
    }
}

/// RD_SYS_PART2_DATA3 value type for batch read/modify/write operations
///
/// Use `rdsyspart2data3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart2data3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart2Data3Val {
    bits: u32,
}

impl RdSysPart2Data3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART2_3** `[31:0]`
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub const fn sys_data_part2_3(&self) -> u32 {
        (self.bits & RdSysPart2Data3::SYS_DATA_PART2_3_MASK)
            >> RdSysPart2Data3::SYS_DATA_PART2_3_OFFSET
    }
}

impl Default for RdSysPart2Data3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart2Data3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart2Data3Val {
        RdSysPart2Data3Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART2_DATA4 Register
///
/// Register $n of BLOCK10 (system).
///
/// Address offset: 0x016c
///
/// ## Fields
///
/// - **SYS_DATA_PART2_4** `[31:0]` (RO)
///
///   Stores the 0th 32 bits of the 2nd part of system data.
#[repr(transparent)]
pub struct RdSysPart2Data4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart2Data4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart2Data4 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART2_4**
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART2_4_OFFSET: u32 = 0;
    pub const SYS_DATA_PART2_4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART2_4** `[31:0]` (RO)
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub fn sys_data_part2_4(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART2_4_MASK) >> Self::SYS_DATA_PART2_4_OFFSET
    }
}

/// RD_SYS_PART2_DATA4 value type for batch read/modify/write operations
///
/// Use `rdsyspart2data4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart2data4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart2Data4Val {
    bits: u32,
}

impl RdSysPart2Data4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART2_4** `[31:0]`
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub const fn sys_data_part2_4(&self) -> u32 {
        (self.bits & RdSysPart2Data4::SYS_DATA_PART2_4_MASK)
            >> RdSysPart2Data4::SYS_DATA_PART2_4_OFFSET
    }
}

impl Default for RdSysPart2Data4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart2Data4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart2Data4Val {
        RdSysPart2Data4Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART2_DATA5 Register
///
/// Register $n of BLOCK10 (system).
///
/// Address offset: 0x0170
///
/// ## Fields
///
/// - **SYS_DATA_PART2_5** `[31:0]` (RO)
///
///   Stores the 0th 32 bits of the 2nd part of system data.
#[repr(transparent)]
pub struct RdSysPart2Data5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart2Data5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart2Data5 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART2_5**
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART2_5_OFFSET: u32 = 0;
    pub const SYS_DATA_PART2_5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART2_5** `[31:0]` (RO)
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub fn sys_data_part2_5(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART2_5_MASK) >> Self::SYS_DATA_PART2_5_OFFSET
    }
}

/// RD_SYS_PART2_DATA5 value type for batch read/modify/write operations
///
/// Use `rdsyspart2data5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart2data5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart2Data5Val {
    bits: u32,
}

impl RdSysPart2Data5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART2_5** `[31:0]`
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub const fn sys_data_part2_5(&self) -> u32 {
        (self.bits & RdSysPart2Data5::SYS_DATA_PART2_5_MASK)
            >> RdSysPart2Data5::SYS_DATA_PART2_5_OFFSET
    }
}

impl Default for RdSysPart2Data5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart2Data5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart2Data5Val {
        RdSysPart2Data5Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART2_DATA6 Register
///
/// Register $n of BLOCK10 (system).
///
/// Address offset: 0x0174
///
/// ## Fields
///
/// - **SYS_DATA_PART2_6** `[31:0]` (RO)
///
///   Stores the 0th 32 bits of the 2nd part of system data.
#[repr(transparent)]
pub struct RdSysPart2Data6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart2Data6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart2Data6 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART2_6**
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART2_6_OFFSET: u32 = 0;
    pub const SYS_DATA_PART2_6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART2_6** `[31:0]` (RO)
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub fn sys_data_part2_6(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART2_6_MASK) >> Self::SYS_DATA_PART2_6_OFFSET
    }
}

/// RD_SYS_PART2_DATA6 value type for batch read/modify/write operations
///
/// Use `rdsyspart2data6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart2data6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart2Data6Val {
    bits: u32,
}

impl RdSysPart2Data6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART2_6** `[31:0]`
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub const fn sys_data_part2_6(&self) -> u32 {
        (self.bits & RdSysPart2Data6::SYS_DATA_PART2_6_MASK)
            >> RdSysPart2Data6::SYS_DATA_PART2_6_OFFSET
    }
}

impl Default for RdSysPart2Data6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart2Data6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart2Data6Val {
        RdSysPart2Data6Val::from_bits(self.value.read_bits())
    }
}

/// RD_SYS_PART2_DATA7 Register
///
/// Register $n of BLOCK10 (system).
///
/// Address offset: 0x0178
///
/// ## Fields
///
/// - **SYS_DATA_PART2_7** `[31:0]` (RO)
///
///   Stores the 0th 32 bits of the 2nd part of system data.
#[repr(transparent)]
pub struct RdSysPart2Data7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdSysPart2Data7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdSysPart2Data7 {
    // ======== Field Constants ========

    /// **SYS_DATA_PART2_7**
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const SYS_DATA_PART2_7_OFFSET: u32 = 0;
    pub const SYS_DATA_PART2_7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **SYS_DATA_PART2_7** `[31:0]` (RO)
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub fn sys_data_part2_7(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_DATA_PART2_7_MASK) >> Self::SYS_DATA_PART2_7_OFFSET
    }
}

/// RD_SYS_PART2_DATA7 value type for batch read/modify/write operations
///
/// Use `rdsyspart2data7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdsyspart2data7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdSysPart2Data7Val {
    bits: u32,
}

impl RdSysPart2Data7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_DATA_PART2_7** `[31:0]`
    ///
    /// Stores the 0th 32 bits of the 2nd part of system data.
    #[inline(always)]
    pub const fn sys_data_part2_7(&self) -> u32 {
        (self.bits & RdSysPart2Data7::SYS_DATA_PART2_7_MASK)
            >> RdSysPart2Data7::SYS_DATA_PART2_7_OFFSET
    }
}

impl Default for RdSysPart2Data7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdSysPart2Data7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdSysPart2Data7Val {
        RdSysPart2Data7Val::from_bits(self.value.read_bits())
    }
}

/// RD_REPEAT_ERR0 Register
///
/// Programming error record register 0 of BLOCK0.
///
/// Address offset: 0x017c
///
/// ## Fields
///
/// - **HUK_GEN_STATE_LOW_ERR** `[31:26]` (RO)
///
///   Indicates a programming error of HUK_GEN_STATE_LOW.
///
/// - **USB_PHY_SEL_ERR** `[25]` (RO)
///
///   Indicates a programming error of USB_PHY_SEL.
///
/// - **USB_OTG11_DREFH_ERR** `[24:23]` (RO)
///
///   Indicates a programming error of USB_OTG11_DREFH.
///
/// - **USB_DEVICE_DREFH_ERR** `[22:21]` (RO)
///
///   Indicates a programming error of USB_DEVICE_DREFH.
///
/// - **DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR** `[20]` (RO)
///
///   Indicates a programming error of DIS_DOWNLOAD_MANUAL_ENCRYPT.
///
/// - **DIS_PAD_JTAG_ERR** `[19]` (RO)
///
///   Indicates a programming error of DIS_PAD_JTAG.
///
/// - **SOFT_DIS_JTAG_ERR** `[18:16]` (RO)
///
///   Indicates a programming error of SOFT_DIS_JTAG.
///
/// - **JTAG_SEL_ENABLE_ERR** `[15]` (RO)
///
///   Indicates a programming error of JTAG_SEL_ENABLE.
///
/// - **DIS_TWAI_ERR** `[14]` (RO)
///
///   Indicates a programming error of DIS_TWAI.
///
/// - **SPI_DOWNLOAD_MSPI_DIS_ERR** `[13]` (RO)
///
///   Indicates a programming error of SPI_DOWNLOAD_MSPI_DIS.
///
/// - **DIS_FORCE_DOWNLOAD_ERR** `[12]` (RO)
///
///   Indicates a programming error of DIS_FORCE_DOWNLOAD.
///
/// - **DIS_USB_SERIAL_JTAG_ERR** `[11]` (RO)
///
///   Indicates a programming error of DIS_USB_SERIAL_JTAG.
///
/// - **POWERGLITCH_EN_ERR** `[10]` (RO)
///
///   Indicates a programming error of POWERGLITCH_EN.
///
/// - **DIS_USB_JTAG_ERR** `[9]` (RO)
///
///   Indicates a programming error of DIS_USB_JTAG.
///
/// - **DIS_USB_OTG11_EXCHG_PINS_ERR** `[8]` (RO)
///
///   Indicates a programming error of DIS_USB_OTG11_EXCHG_PINS.
///
/// - **DIS_USB_DEVICE_EXCHG_PINS_ERR** `[7]` (RO)
///
///   Indicates a programming error of DIS_USB_DEVICE_EXCHG_PINS.
///
/// - **RD_DIS_ERR** `[6:0]` (RO)
///
///   Indicates a programming error of RD_DIS.
#[repr(transparent)]
pub struct RdRepeatErr0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdRepeatErr0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdRepeatErr0 {
    // ======== Field Constants ========

    /// **HUK_GEN_STATE_LOW_ERR**
    ///
    /// Indicates a programming error of HUK_GEN_STATE_LOW.
    ///
    /// Bits `[31:26]`, mask `0xfc000000`
    pub const HUK_GEN_STATE_LOW_ERR_OFFSET: u32 = 26;
    pub const HUK_GEN_STATE_LOW_ERR_MASK: u32 = 0xfc000000;

    /// **USB_PHY_SEL_ERR**
    ///
    /// Indicates a programming error of USB_PHY_SEL.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const USB_PHY_SEL_ERR_OFFSET: u32 = 25;
    pub const USB_PHY_SEL_ERR_MASK: u32 = 0x02000000;
    pub const USB_PHY_SEL_ERR: u32 = 1 << 25;

    /// **USB_OTG11_DREFH_ERR**
    ///
    /// Indicates a programming error of USB_OTG11_DREFH.
    ///
    /// Bits `[24:23]`, mask `0x01800000`
    pub const USB_OTG11_DREFH_ERR_OFFSET: u32 = 23;
    pub const USB_OTG11_DREFH_ERR_MASK: u32 = 0x01800000;

    /// **USB_DEVICE_DREFH_ERR**
    ///
    /// Indicates a programming error of USB_DEVICE_DREFH.
    ///
    /// Bits `[22:21]`, mask `0x00600000`
    pub const USB_DEVICE_DREFH_ERR_OFFSET: u32 = 21;
    pub const USB_DEVICE_DREFH_ERR_MASK: u32 = 0x00600000;

    /// **DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR**
    ///
    /// Indicates a programming error of DIS_DOWNLOAD_MANUAL_ENCRYPT.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_OFFSET: u32 = 20;
    pub const DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_MASK: u32 = 0x00100000;
    pub const DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR: u32 = 1 << 20;

    /// **DIS_PAD_JTAG_ERR**
    ///
    /// Indicates a programming error of DIS_PAD_JTAG.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const DIS_PAD_JTAG_ERR_OFFSET: u32 = 19;
    pub const DIS_PAD_JTAG_ERR_MASK: u32 = 0x00080000;
    pub const DIS_PAD_JTAG_ERR: u32 = 1 << 19;

    /// **SOFT_DIS_JTAG_ERR**
    ///
    /// Indicates a programming error of SOFT_DIS_JTAG.
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const SOFT_DIS_JTAG_ERR_OFFSET: u32 = 16;
    pub const SOFT_DIS_JTAG_ERR_MASK: u32 = 0x00070000;

    /// **JTAG_SEL_ENABLE_ERR**
    ///
    /// Indicates a programming error of JTAG_SEL_ENABLE.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const JTAG_SEL_ENABLE_ERR_OFFSET: u32 = 15;
    pub const JTAG_SEL_ENABLE_ERR_MASK: u32 = 0x00008000;
    pub const JTAG_SEL_ENABLE_ERR: u32 = 1 << 15;

    /// **DIS_TWAI_ERR**
    ///
    /// Indicates a programming error of DIS_TWAI.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const DIS_TWAI_ERR_OFFSET: u32 = 14;
    pub const DIS_TWAI_ERR_MASK: u32 = 0x00004000;
    pub const DIS_TWAI_ERR: u32 = 1 << 14;

    /// **SPI_DOWNLOAD_MSPI_DIS_ERR**
    ///
    /// Indicates a programming error of SPI_DOWNLOAD_MSPI_DIS.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const SPI_DOWNLOAD_MSPI_DIS_ERR_OFFSET: u32 = 13;
    pub const SPI_DOWNLOAD_MSPI_DIS_ERR_MASK: u32 = 0x00002000;
    pub const SPI_DOWNLOAD_MSPI_DIS_ERR: u32 = 1 << 13;

    /// **DIS_FORCE_DOWNLOAD_ERR**
    ///
    /// Indicates a programming error of DIS_FORCE_DOWNLOAD.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const DIS_FORCE_DOWNLOAD_ERR_OFFSET: u32 = 12;
    pub const DIS_FORCE_DOWNLOAD_ERR_MASK: u32 = 0x00001000;
    pub const DIS_FORCE_DOWNLOAD_ERR: u32 = 1 << 12;

    /// **DIS_USB_SERIAL_JTAG_ERR**
    ///
    /// Indicates a programming error of DIS_USB_SERIAL_JTAG.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const DIS_USB_SERIAL_JTAG_ERR_OFFSET: u32 = 11;
    pub const DIS_USB_SERIAL_JTAG_ERR_MASK: u32 = 0x00000800;
    pub const DIS_USB_SERIAL_JTAG_ERR: u32 = 1 << 11;

    /// **POWERGLITCH_EN_ERR**
    ///
    /// Indicates a programming error of POWERGLITCH_EN.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const POWERGLITCH_EN_ERR_OFFSET: u32 = 10;
    pub const POWERGLITCH_EN_ERR_MASK: u32 = 0x00000400;
    pub const POWERGLITCH_EN_ERR: u32 = 1 << 10;

    /// **DIS_USB_JTAG_ERR**
    ///
    /// Indicates a programming error of DIS_USB_JTAG.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DIS_USB_JTAG_ERR_OFFSET: u32 = 9;
    pub const DIS_USB_JTAG_ERR_MASK: u32 = 0x00000200;
    pub const DIS_USB_JTAG_ERR: u32 = 1 << 9;

    /// **DIS_USB_OTG11_EXCHG_PINS_ERR**
    ///
    /// Indicates a programming error of DIS_USB_OTG11_EXCHG_PINS.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const DIS_USB_OTG11_EXCHG_PINS_ERR_OFFSET: u32 = 8;
    pub const DIS_USB_OTG11_EXCHG_PINS_ERR_MASK: u32 = 0x00000100;
    pub const DIS_USB_OTG11_EXCHG_PINS_ERR: u32 = 1 << 8;

    /// **DIS_USB_DEVICE_EXCHG_PINS_ERR**
    ///
    /// Indicates a programming error of DIS_USB_DEVICE_EXCHG_PINS.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DIS_USB_DEVICE_EXCHG_PINS_ERR_OFFSET: u32 = 7;
    pub const DIS_USB_DEVICE_EXCHG_PINS_ERR_MASK: u32 = 0x00000080;
    pub const DIS_USB_DEVICE_EXCHG_PINS_ERR: u32 = 1 << 7;

    /// **RD_DIS_ERR**
    ///
    /// Indicates a programming error of RD_DIS.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const RD_DIS_ERR_OFFSET: u32 = 0;
    pub const RD_DIS_ERR_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **HUK_GEN_STATE_LOW_ERR** `[31:26]` (RO)
    ///
    /// Indicates a programming error of HUK_GEN_STATE_LOW.
    #[inline(always)]
    pub fn huk_gen_state_low_err(&self) -> u32 {
        (self.value.read_bits() & Self::HUK_GEN_STATE_LOW_ERR_MASK)
            >> Self::HUK_GEN_STATE_LOW_ERR_OFFSET
    }

    /// **USB_PHY_SEL_ERR** `[25]` (RO)
    ///
    /// Indicates a programming error of USB_PHY_SEL.
    #[inline(always)]
    pub fn usb_phy_sel_err(&self) -> bool {
        (self.value.read_bits() & Self::USB_PHY_SEL_ERR_MASK) != 0
    }

    /// **USB_OTG11_DREFH_ERR** `[24:23]` (RO)
    ///
    /// Indicates a programming error of USB_OTG11_DREFH.
    #[inline(always)]
    pub fn usb_otg11_drefh_err(&self) -> u32 {
        (self.value.read_bits() & Self::USB_OTG11_DREFH_ERR_MASK)
            >> Self::USB_OTG11_DREFH_ERR_OFFSET
    }

    /// **USB_DEVICE_DREFH_ERR** `[22:21]` (RO)
    ///
    /// Indicates a programming error of USB_DEVICE_DREFH.
    #[inline(always)]
    pub fn usb_device_drefh_err(&self) -> u32 {
        (self.value.read_bits() & Self::USB_DEVICE_DREFH_ERR_MASK)
            >> Self::USB_DEVICE_DREFH_ERR_OFFSET
    }

    /// **DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR** `[20]` (RO)
    ///
    /// Indicates a programming error of DIS_DOWNLOAD_MANUAL_ENCRYPT.
    #[inline(always)]
    pub fn dis_download_manual_encrypt_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_MASK) != 0
    }

    /// **DIS_PAD_JTAG_ERR** `[19]` (RO)
    ///
    /// Indicates a programming error of DIS_PAD_JTAG.
    #[inline(always)]
    pub fn dis_pad_jtag_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_PAD_JTAG_ERR_MASK) != 0
    }

    /// **SOFT_DIS_JTAG_ERR** `[18:16]` (RO)
    ///
    /// Indicates a programming error of SOFT_DIS_JTAG.
    #[inline(always)]
    pub fn soft_dis_jtag_err(&self) -> u32 {
        (self.value.read_bits() & Self::SOFT_DIS_JTAG_ERR_MASK) >> Self::SOFT_DIS_JTAG_ERR_OFFSET
    }

    /// **JTAG_SEL_ENABLE_ERR** `[15]` (RO)
    ///
    /// Indicates a programming error of JTAG_SEL_ENABLE.
    #[inline(always)]
    pub fn jtag_sel_enable_err(&self) -> bool {
        (self.value.read_bits() & Self::JTAG_SEL_ENABLE_ERR_MASK) != 0
    }

    /// **DIS_TWAI_ERR** `[14]` (RO)
    ///
    /// Indicates a programming error of DIS_TWAI.
    #[inline(always)]
    pub fn dis_twai_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_TWAI_ERR_MASK) != 0
    }

    /// **SPI_DOWNLOAD_MSPI_DIS_ERR** `[13]` (RO)
    ///
    /// Indicates a programming error of SPI_DOWNLOAD_MSPI_DIS.
    #[inline(always)]
    pub fn spi_download_mspi_dis_err(&self) -> bool {
        (self.value.read_bits() & Self::SPI_DOWNLOAD_MSPI_DIS_ERR_MASK) != 0
    }

    /// **DIS_FORCE_DOWNLOAD_ERR** `[12]` (RO)
    ///
    /// Indicates a programming error of DIS_FORCE_DOWNLOAD.
    #[inline(always)]
    pub fn dis_force_download_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_FORCE_DOWNLOAD_ERR_MASK) != 0
    }

    /// **DIS_USB_SERIAL_JTAG_ERR** `[11]` (RO)
    ///
    /// Indicates a programming error of DIS_USB_SERIAL_JTAG.
    #[inline(always)]
    pub fn dis_usb_serial_jtag_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_USB_SERIAL_JTAG_ERR_MASK) != 0
    }

    /// **POWERGLITCH_EN_ERR** `[10]` (RO)
    ///
    /// Indicates a programming error of POWERGLITCH_EN.
    #[inline(always)]
    pub fn powerglitch_en_err(&self) -> bool {
        (self.value.read_bits() & Self::POWERGLITCH_EN_ERR_MASK) != 0
    }

    /// **DIS_USB_JTAG_ERR** `[9]` (RO)
    ///
    /// Indicates a programming error of DIS_USB_JTAG.
    #[inline(always)]
    pub fn dis_usb_jtag_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_USB_JTAG_ERR_MASK) != 0
    }

    /// **DIS_USB_OTG11_EXCHG_PINS_ERR** `[8]` (RO)
    ///
    /// Indicates a programming error of DIS_USB_OTG11_EXCHG_PINS.
    #[inline(always)]
    pub fn dis_usb_otg11_exchg_pins_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_USB_OTG11_EXCHG_PINS_ERR_MASK) != 0
    }

    /// **DIS_USB_DEVICE_EXCHG_PINS_ERR** `[7]` (RO)
    ///
    /// Indicates a programming error of DIS_USB_DEVICE_EXCHG_PINS.
    #[inline(always)]
    pub fn dis_usb_device_exchg_pins_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_USB_DEVICE_EXCHG_PINS_ERR_MASK) != 0
    }

    /// **RD_DIS_ERR** `[6:0]` (RO)
    ///
    /// Indicates a programming error of RD_DIS.
    #[inline(always)]
    pub fn rd_dis_err(&self) -> u32 {
        (self.value.read_bits() & Self::RD_DIS_ERR_MASK) >> Self::RD_DIS_ERR_OFFSET
    }
}

/// RD_REPEAT_ERR0 value type for batch read/modify/write operations
///
/// Use `rdrepeaterr0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdrepeaterr0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdRepeatErr0Val {
    bits: u32,
}

impl RdRepeatErr0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **HUK_GEN_STATE_LOW_ERR** `[31:26]`
    ///
    /// Indicates a programming error of HUK_GEN_STATE_LOW.
    #[inline(always)]
    pub const fn huk_gen_state_low_err(&self) -> u32 {
        (self.bits & RdRepeatErr0::HUK_GEN_STATE_LOW_ERR_MASK)
            >> RdRepeatErr0::HUK_GEN_STATE_LOW_ERR_OFFSET
    }

    /// Read **USB_PHY_SEL_ERR** `[25:25]`
    ///
    /// Indicates a programming error of USB_PHY_SEL.
    #[inline(always)]
    pub const fn usb_phy_sel_err(&self) -> bool {
        (self.bits & RdRepeatErr0::USB_PHY_SEL_ERR_MASK) != 0
    }

    /// Read **USB_OTG11_DREFH_ERR** `[24:23]`
    ///
    /// Indicates a programming error of USB_OTG11_DREFH.
    #[inline(always)]
    pub const fn usb_otg11_drefh_err(&self) -> u32 {
        (self.bits & RdRepeatErr0::USB_OTG11_DREFH_ERR_MASK)
            >> RdRepeatErr0::USB_OTG11_DREFH_ERR_OFFSET
    }

    /// Read **USB_DEVICE_DREFH_ERR** `[22:21]`
    ///
    /// Indicates a programming error of USB_DEVICE_DREFH.
    #[inline(always)]
    pub const fn usb_device_drefh_err(&self) -> u32 {
        (self.bits & RdRepeatErr0::USB_DEVICE_DREFH_ERR_MASK)
            >> RdRepeatErr0::USB_DEVICE_DREFH_ERR_OFFSET
    }

    /// Read **DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR** `[20:20]`
    ///
    /// Indicates a programming error of DIS_DOWNLOAD_MANUAL_ENCRYPT.
    #[inline(always)]
    pub const fn dis_download_manual_encrypt_err(&self) -> bool {
        (self.bits & RdRepeatErr0::DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_MASK) != 0
    }

    /// Read **DIS_PAD_JTAG_ERR** `[19:19]`
    ///
    /// Indicates a programming error of DIS_PAD_JTAG.
    #[inline(always)]
    pub const fn dis_pad_jtag_err(&self) -> bool {
        (self.bits & RdRepeatErr0::DIS_PAD_JTAG_ERR_MASK) != 0
    }

    /// Read **SOFT_DIS_JTAG_ERR** `[18:16]`
    ///
    /// Indicates a programming error of SOFT_DIS_JTAG.
    #[inline(always)]
    pub const fn soft_dis_jtag_err(&self) -> u32 {
        (self.bits & RdRepeatErr0::SOFT_DIS_JTAG_ERR_MASK) >> RdRepeatErr0::SOFT_DIS_JTAG_ERR_OFFSET
    }

    /// Read **JTAG_SEL_ENABLE_ERR** `[15:15]`
    ///
    /// Indicates a programming error of JTAG_SEL_ENABLE.
    #[inline(always)]
    pub const fn jtag_sel_enable_err(&self) -> bool {
        (self.bits & RdRepeatErr0::JTAG_SEL_ENABLE_ERR_MASK) != 0
    }

    /// Read **DIS_TWAI_ERR** `[14:14]`
    ///
    /// Indicates a programming error of DIS_TWAI.
    #[inline(always)]
    pub const fn dis_twai_err(&self) -> bool {
        (self.bits & RdRepeatErr0::DIS_TWAI_ERR_MASK) != 0
    }

    /// Read **SPI_DOWNLOAD_MSPI_DIS_ERR** `[13:13]`
    ///
    /// Indicates a programming error of SPI_DOWNLOAD_MSPI_DIS.
    #[inline(always)]
    pub const fn spi_download_mspi_dis_err(&self) -> bool {
        (self.bits & RdRepeatErr0::SPI_DOWNLOAD_MSPI_DIS_ERR_MASK) != 0
    }

    /// Read **DIS_FORCE_DOWNLOAD_ERR** `[12:12]`
    ///
    /// Indicates a programming error of DIS_FORCE_DOWNLOAD.
    #[inline(always)]
    pub const fn dis_force_download_err(&self) -> bool {
        (self.bits & RdRepeatErr0::DIS_FORCE_DOWNLOAD_ERR_MASK) != 0
    }

    /// Read **DIS_USB_SERIAL_JTAG_ERR** `[11:11]`
    ///
    /// Indicates a programming error of DIS_USB_SERIAL_JTAG.
    #[inline(always)]
    pub const fn dis_usb_serial_jtag_err(&self) -> bool {
        (self.bits & RdRepeatErr0::DIS_USB_SERIAL_JTAG_ERR_MASK) != 0
    }

    /// Read **POWERGLITCH_EN_ERR** `[10:10]`
    ///
    /// Indicates a programming error of POWERGLITCH_EN.
    #[inline(always)]
    pub const fn powerglitch_en_err(&self) -> bool {
        (self.bits & RdRepeatErr0::POWERGLITCH_EN_ERR_MASK) != 0
    }

    /// Read **DIS_USB_JTAG_ERR** `[9:9]`
    ///
    /// Indicates a programming error of DIS_USB_JTAG.
    #[inline(always)]
    pub const fn dis_usb_jtag_err(&self) -> bool {
        (self.bits & RdRepeatErr0::DIS_USB_JTAG_ERR_MASK) != 0
    }

    /// Read **DIS_USB_OTG11_EXCHG_PINS_ERR** `[8:8]`
    ///
    /// Indicates a programming error of DIS_USB_OTG11_EXCHG_PINS.
    #[inline(always)]
    pub const fn dis_usb_otg11_exchg_pins_err(&self) -> bool {
        (self.bits & RdRepeatErr0::DIS_USB_OTG11_EXCHG_PINS_ERR_MASK) != 0
    }

    /// Read **DIS_USB_DEVICE_EXCHG_PINS_ERR** `[7:7]`
    ///
    /// Indicates a programming error of DIS_USB_DEVICE_EXCHG_PINS.
    #[inline(always)]
    pub const fn dis_usb_device_exchg_pins_err(&self) -> bool {
        (self.bits & RdRepeatErr0::DIS_USB_DEVICE_EXCHG_PINS_ERR_MASK) != 0
    }

    /// Read **RD_DIS_ERR** `[6:0]`
    ///
    /// Indicates a programming error of RD_DIS.
    #[inline(always)]
    pub const fn rd_dis_err(&self) -> u32 {
        (self.bits & RdRepeatErr0::RD_DIS_ERR_MASK) >> RdRepeatErr0::RD_DIS_ERR_OFFSET
    }
}

impl Default for RdRepeatErr0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdRepeatErr0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdRepeatErr0Val {
        RdRepeatErr0Val::from_bits(self.value.read_bits())
    }
}

/// RD_REPEAT_ERR1 Register
///
/// Programming error record register 1 of BLOCK0.
///
/// Address offset: 0x0180
///
/// ## Fields
///
/// - **KEY_PURPOSE_1_ERR** `[31:28]` (RO)
///
///   Indicates a programming error of KEY_PURPOSE_1.
///
/// - **KEY_PURPOSE_0_ERR** `[27:24]` (RO)
///
///   Indicates a programming error of KEY_PURPOSE_0.
///
/// - **SECURE_BOOT_KEY_REVOKE2_ERR** `[23]` (RO)
///
///   Indicates a programming error of SECURE_BOOT_KEY_REVOKE2.
///
/// - **SECURE_BOOT_KEY_REVOKE1_ERR** `[22]` (RO)
///
///   Indicates a programming error of SECURE_BOOT_KEY_REVOKE1.
///
/// - **SECURE_BOOT_KEY_REVOKE0_ERR** `[21]` (RO)
///
///   Indicates a programming error of SECURE_BOOT_KEY_REVOKE0.
///
/// - **SPI_BOOT_CRYPT_CNT_ERR** `[20:18]` (RO)
///
///   Indicates a programming error of SPI_BOOT_CRYPT_CNT.
///
/// - **WDT_DELAY_SEL_ERR** `[17:16]` (RO)
///
///   Indicates a programming error of WDT_DELAY_SEL.
///
/// - **XTS_KEY_LENGTH_256_ERR** `[14]` (RO)
///
///   Indicates a programming error of XTS_KEY_LENGTH_256.
///
/// - **FORCE_DISABLE_SW_INIT_KEY_ERR** `[13]` (RO)
///
///   Indicates a programming error of FORCE_DISABLE_SW_INIT_KEY.
///
/// - **FORCE_USE_KEY_MANAGER_KEY_ERR** `[12:9]` (RO)
///
///   Indicates a programming error of FORCE_USE_KEY_MANAGER_KEY.
///
/// - **KM_DEPLOY_ONLY_ONCE_ERR** `[8:5]` (RO)
///
///   Indicates a programming error of KM_DEPLOY_ONLY_ONCE.
///
/// - **KM_RND_SWITCH_CYCLE_ERR** `[4:3]` (RO)
///
///   Indicates a programming error of KM_RND_SWITCH_CYCLE.
///
/// - **KM_HUK_GEN_STATE_HIGH_ERR** `[2:0]` (RO)
///
///   Indicates a programming error of HUK_GEN_STATE_HIGH.
#[repr(transparent)]
pub struct RdRepeatErr1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdRepeatErr1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdRepeatErr1 {
    // ======== Field Constants ========

    /// **KEY_PURPOSE_1_ERR**
    ///
    /// Indicates a programming error of KEY_PURPOSE_1.
    ///
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const KEY_PURPOSE_1_ERR_OFFSET: u32 = 28;
    pub const KEY_PURPOSE_1_ERR_MASK: u32 = 0xf0000000;

    /// **KEY_PURPOSE_0_ERR**
    ///
    /// Indicates a programming error of KEY_PURPOSE_0.
    ///
    /// Bits `[27:24]`, mask `0x0f000000`
    pub const KEY_PURPOSE_0_ERR_OFFSET: u32 = 24;
    pub const KEY_PURPOSE_0_ERR_MASK: u32 = 0x0f000000;

    /// **SECURE_BOOT_KEY_REVOKE2_ERR**
    ///
    /// Indicates a programming error of SECURE_BOOT_KEY_REVOKE2.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const SECURE_BOOT_KEY_REVOKE2_ERR_OFFSET: u32 = 23;
    pub const SECURE_BOOT_KEY_REVOKE2_ERR_MASK: u32 = 0x00800000;
    pub const SECURE_BOOT_KEY_REVOKE2_ERR: u32 = 1 << 23;

    /// **SECURE_BOOT_KEY_REVOKE1_ERR**
    ///
    /// Indicates a programming error of SECURE_BOOT_KEY_REVOKE1.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const SECURE_BOOT_KEY_REVOKE1_ERR_OFFSET: u32 = 22;
    pub const SECURE_BOOT_KEY_REVOKE1_ERR_MASK: u32 = 0x00400000;
    pub const SECURE_BOOT_KEY_REVOKE1_ERR: u32 = 1 << 22;

    /// **SECURE_BOOT_KEY_REVOKE0_ERR**
    ///
    /// Indicates a programming error of SECURE_BOOT_KEY_REVOKE0.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SECURE_BOOT_KEY_REVOKE0_ERR_OFFSET: u32 = 21;
    pub const SECURE_BOOT_KEY_REVOKE0_ERR_MASK: u32 = 0x00200000;
    pub const SECURE_BOOT_KEY_REVOKE0_ERR: u32 = 1 << 21;

    /// **SPI_BOOT_CRYPT_CNT_ERR**
    ///
    /// Indicates a programming error of SPI_BOOT_CRYPT_CNT.
    ///
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const SPI_BOOT_CRYPT_CNT_ERR_OFFSET: u32 = 18;
    pub const SPI_BOOT_CRYPT_CNT_ERR_MASK: u32 = 0x001c0000;

    /// **WDT_DELAY_SEL_ERR**
    ///
    /// Indicates a programming error of WDT_DELAY_SEL.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const WDT_DELAY_SEL_ERR_OFFSET: u32 = 16;
    pub const WDT_DELAY_SEL_ERR_MASK: u32 = 0x00030000;

    /// **XTS_KEY_LENGTH_256_ERR**
    ///
    /// Indicates a programming error of XTS_KEY_LENGTH_256.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const XTS_KEY_LENGTH_256_ERR_OFFSET: u32 = 14;
    pub const XTS_KEY_LENGTH_256_ERR_MASK: u32 = 0x00004000;
    pub const XTS_KEY_LENGTH_256_ERR: u32 = 1 << 14;

    /// **FORCE_DISABLE_SW_INIT_KEY_ERR**
    ///
    /// Indicates a programming error of FORCE_DISABLE_SW_INIT_KEY.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const FORCE_DISABLE_SW_INIT_KEY_ERR_OFFSET: u32 = 13;
    pub const FORCE_DISABLE_SW_INIT_KEY_ERR_MASK: u32 = 0x00002000;
    pub const FORCE_DISABLE_SW_INIT_KEY_ERR: u32 = 1 << 13;

    /// **FORCE_USE_KEY_MANAGER_KEY_ERR**
    ///
    /// Indicates a programming error of FORCE_USE_KEY_MANAGER_KEY.
    ///
    /// Bits `[12:9]`, mask `0x00001e00`
    pub const FORCE_USE_KEY_MANAGER_KEY_ERR_OFFSET: u32 = 9;
    pub const FORCE_USE_KEY_MANAGER_KEY_ERR_MASK: u32 = 0x00001e00;

    /// **KM_DEPLOY_ONLY_ONCE_ERR**
    ///
    /// Indicates a programming error of KM_DEPLOY_ONLY_ONCE.
    ///
    /// Bits `[8:5]`, mask `0x000001e0`
    pub const KM_DEPLOY_ONLY_ONCE_ERR_OFFSET: u32 = 5;
    pub const KM_DEPLOY_ONLY_ONCE_ERR_MASK: u32 = 0x000001e0;

    /// **KM_RND_SWITCH_CYCLE_ERR**
    ///
    /// Indicates a programming error of KM_RND_SWITCH_CYCLE.
    ///
    /// Bits `[4:3]`, mask `0x00000018`
    pub const KM_RND_SWITCH_CYCLE_ERR_OFFSET: u32 = 3;
    pub const KM_RND_SWITCH_CYCLE_ERR_MASK: u32 = 0x00000018;

    /// **KM_HUK_GEN_STATE_HIGH_ERR**
    ///
    /// Indicates a programming error of HUK_GEN_STATE_HIGH.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const KM_HUK_GEN_STATE_HIGH_ERR_OFFSET: u32 = 0;
    pub const KM_HUK_GEN_STATE_HIGH_ERR_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **KEY_PURPOSE_1_ERR** `[31:28]` (RO)
    ///
    /// Indicates a programming error of KEY_PURPOSE_1.
    #[inline(always)]
    pub fn key_purpose_1_err(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_PURPOSE_1_ERR_MASK) >> Self::KEY_PURPOSE_1_ERR_OFFSET
    }

    /// **KEY_PURPOSE_0_ERR** `[27:24]` (RO)
    ///
    /// Indicates a programming error of KEY_PURPOSE_0.
    #[inline(always)]
    pub fn key_purpose_0_err(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_PURPOSE_0_ERR_MASK) >> Self::KEY_PURPOSE_0_ERR_OFFSET
    }

    /// **SECURE_BOOT_KEY_REVOKE2_ERR** `[23]` (RO)
    ///
    /// Indicates a programming error of SECURE_BOOT_KEY_REVOKE2.
    #[inline(always)]
    pub fn secure_boot_key_revoke2_err(&self) -> bool {
        (self.value.read_bits() & Self::SECURE_BOOT_KEY_REVOKE2_ERR_MASK) != 0
    }

    /// **SECURE_BOOT_KEY_REVOKE1_ERR** `[22]` (RO)
    ///
    /// Indicates a programming error of SECURE_BOOT_KEY_REVOKE1.
    #[inline(always)]
    pub fn secure_boot_key_revoke1_err(&self) -> bool {
        (self.value.read_bits() & Self::SECURE_BOOT_KEY_REVOKE1_ERR_MASK) != 0
    }

    /// **SECURE_BOOT_KEY_REVOKE0_ERR** `[21]` (RO)
    ///
    /// Indicates a programming error of SECURE_BOOT_KEY_REVOKE0.
    #[inline(always)]
    pub fn secure_boot_key_revoke0_err(&self) -> bool {
        (self.value.read_bits() & Self::SECURE_BOOT_KEY_REVOKE0_ERR_MASK) != 0
    }

    /// **SPI_BOOT_CRYPT_CNT_ERR** `[20:18]` (RO)
    ///
    /// Indicates a programming error of SPI_BOOT_CRYPT_CNT.
    #[inline(always)]
    pub fn spi_boot_crypt_cnt_err(&self) -> u32 {
        (self.value.read_bits() & Self::SPI_BOOT_CRYPT_CNT_ERR_MASK)
            >> Self::SPI_BOOT_CRYPT_CNT_ERR_OFFSET
    }

    /// **WDT_DELAY_SEL_ERR** `[17:16]` (RO)
    ///
    /// Indicates a programming error of WDT_DELAY_SEL.
    #[inline(always)]
    pub fn wdt_delay_sel_err(&self) -> u32 {
        (self.value.read_bits() & Self::WDT_DELAY_SEL_ERR_MASK) >> Self::WDT_DELAY_SEL_ERR_OFFSET
    }

    /// **XTS_KEY_LENGTH_256_ERR** `[14]` (RO)
    ///
    /// Indicates a programming error of XTS_KEY_LENGTH_256.
    #[inline(always)]
    pub fn xts_key_length_256_err(&self) -> bool {
        (self.value.read_bits() & Self::XTS_KEY_LENGTH_256_ERR_MASK) != 0
    }

    /// **FORCE_DISABLE_SW_INIT_KEY_ERR** `[13]` (RO)
    ///
    /// Indicates a programming error of FORCE_DISABLE_SW_INIT_KEY.
    #[inline(always)]
    pub fn force_disable_sw_init_key_err(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_DISABLE_SW_INIT_KEY_ERR_MASK) != 0
    }

    /// **FORCE_USE_KEY_MANAGER_KEY_ERR** `[12:9]` (RO)
    ///
    /// Indicates a programming error of FORCE_USE_KEY_MANAGER_KEY.
    #[inline(always)]
    pub fn force_use_key_manager_key_err(&self) -> u32 {
        (self.value.read_bits() & Self::FORCE_USE_KEY_MANAGER_KEY_ERR_MASK)
            >> Self::FORCE_USE_KEY_MANAGER_KEY_ERR_OFFSET
    }

    /// **KM_DEPLOY_ONLY_ONCE_ERR** `[8:5]` (RO)
    ///
    /// Indicates a programming error of KM_DEPLOY_ONLY_ONCE.
    #[inline(always)]
    pub fn km_deploy_only_once_err(&self) -> u32 {
        (self.value.read_bits() & Self::KM_DEPLOY_ONLY_ONCE_ERR_MASK)
            >> Self::KM_DEPLOY_ONLY_ONCE_ERR_OFFSET
    }

    /// **KM_RND_SWITCH_CYCLE_ERR** `[4:3]` (RO)
    ///
    /// Indicates a programming error of KM_RND_SWITCH_CYCLE.
    #[inline(always)]
    pub fn km_rnd_switch_cycle_err(&self) -> u32 {
        (self.value.read_bits() & Self::KM_RND_SWITCH_CYCLE_ERR_MASK)
            >> Self::KM_RND_SWITCH_CYCLE_ERR_OFFSET
    }

    /// **KM_HUK_GEN_STATE_HIGH_ERR** `[2:0]` (RO)
    ///
    /// Indicates a programming error of HUK_GEN_STATE_HIGH.
    #[inline(always)]
    pub fn km_huk_gen_state_high_err(&self) -> u32 {
        (self.value.read_bits() & Self::KM_HUK_GEN_STATE_HIGH_ERR_MASK)
            >> Self::KM_HUK_GEN_STATE_HIGH_ERR_OFFSET
    }
}

/// RD_REPEAT_ERR1 value type for batch read/modify/write operations
///
/// Use `rdrepeaterr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdrepeaterr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdRepeatErr1Val {
    bits: u32,
}

impl RdRepeatErr1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY_PURPOSE_1_ERR** `[31:28]`
    ///
    /// Indicates a programming error of KEY_PURPOSE_1.
    #[inline(always)]
    pub const fn key_purpose_1_err(&self) -> u32 {
        (self.bits & RdRepeatErr1::KEY_PURPOSE_1_ERR_MASK) >> RdRepeatErr1::KEY_PURPOSE_1_ERR_OFFSET
    }

    /// Read **KEY_PURPOSE_0_ERR** `[27:24]`
    ///
    /// Indicates a programming error of KEY_PURPOSE_0.
    #[inline(always)]
    pub const fn key_purpose_0_err(&self) -> u32 {
        (self.bits & RdRepeatErr1::KEY_PURPOSE_0_ERR_MASK) >> RdRepeatErr1::KEY_PURPOSE_0_ERR_OFFSET
    }

    /// Read **SECURE_BOOT_KEY_REVOKE2_ERR** `[23:23]`
    ///
    /// Indicates a programming error of SECURE_BOOT_KEY_REVOKE2.
    #[inline(always)]
    pub const fn secure_boot_key_revoke2_err(&self) -> bool {
        (self.bits & RdRepeatErr1::SECURE_BOOT_KEY_REVOKE2_ERR_MASK) != 0
    }

    /// Read **SECURE_BOOT_KEY_REVOKE1_ERR** `[22:22]`
    ///
    /// Indicates a programming error of SECURE_BOOT_KEY_REVOKE1.
    #[inline(always)]
    pub const fn secure_boot_key_revoke1_err(&self) -> bool {
        (self.bits & RdRepeatErr1::SECURE_BOOT_KEY_REVOKE1_ERR_MASK) != 0
    }

    /// Read **SECURE_BOOT_KEY_REVOKE0_ERR** `[21:21]`
    ///
    /// Indicates a programming error of SECURE_BOOT_KEY_REVOKE0.
    #[inline(always)]
    pub const fn secure_boot_key_revoke0_err(&self) -> bool {
        (self.bits & RdRepeatErr1::SECURE_BOOT_KEY_REVOKE0_ERR_MASK) != 0
    }

    /// Read **SPI_BOOT_CRYPT_CNT_ERR** `[20:18]`
    ///
    /// Indicates a programming error of SPI_BOOT_CRYPT_CNT.
    #[inline(always)]
    pub const fn spi_boot_crypt_cnt_err(&self) -> u32 {
        (self.bits & RdRepeatErr1::SPI_BOOT_CRYPT_CNT_ERR_MASK)
            >> RdRepeatErr1::SPI_BOOT_CRYPT_CNT_ERR_OFFSET
    }

    /// Read **WDT_DELAY_SEL_ERR** `[17:16]`
    ///
    /// Indicates a programming error of WDT_DELAY_SEL.
    #[inline(always)]
    pub const fn wdt_delay_sel_err(&self) -> u32 {
        (self.bits & RdRepeatErr1::WDT_DELAY_SEL_ERR_MASK) >> RdRepeatErr1::WDT_DELAY_SEL_ERR_OFFSET
    }

    /// Read **XTS_KEY_LENGTH_256_ERR** `[14:14]`
    ///
    /// Indicates a programming error of XTS_KEY_LENGTH_256.
    #[inline(always)]
    pub const fn xts_key_length_256_err(&self) -> bool {
        (self.bits & RdRepeatErr1::XTS_KEY_LENGTH_256_ERR_MASK) != 0
    }

    /// Read **FORCE_DISABLE_SW_INIT_KEY_ERR** `[13:13]`
    ///
    /// Indicates a programming error of FORCE_DISABLE_SW_INIT_KEY.
    #[inline(always)]
    pub const fn force_disable_sw_init_key_err(&self) -> bool {
        (self.bits & RdRepeatErr1::FORCE_DISABLE_SW_INIT_KEY_ERR_MASK) != 0
    }

    /// Read **FORCE_USE_KEY_MANAGER_KEY_ERR** `[12:9]`
    ///
    /// Indicates a programming error of FORCE_USE_KEY_MANAGER_KEY.
    #[inline(always)]
    pub const fn force_use_key_manager_key_err(&self) -> u32 {
        (self.bits & RdRepeatErr1::FORCE_USE_KEY_MANAGER_KEY_ERR_MASK)
            >> RdRepeatErr1::FORCE_USE_KEY_MANAGER_KEY_ERR_OFFSET
    }

    /// Read **KM_DEPLOY_ONLY_ONCE_ERR** `[8:5]`
    ///
    /// Indicates a programming error of KM_DEPLOY_ONLY_ONCE.
    #[inline(always)]
    pub const fn km_deploy_only_once_err(&self) -> u32 {
        (self.bits & RdRepeatErr1::KM_DEPLOY_ONLY_ONCE_ERR_MASK)
            >> RdRepeatErr1::KM_DEPLOY_ONLY_ONCE_ERR_OFFSET
    }

    /// Read **KM_RND_SWITCH_CYCLE_ERR** `[4:3]`
    ///
    /// Indicates a programming error of KM_RND_SWITCH_CYCLE.
    #[inline(always)]
    pub const fn km_rnd_switch_cycle_err(&self) -> u32 {
        (self.bits & RdRepeatErr1::KM_RND_SWITCH_CYCLE_ERR_MASK)
            >> RdRepeatErr1::KM_RND_SWITCH_CYCLE_ERR_OFFSET
    }

    /// Read **KM_HUK_GEN_STATE_HIGH_ERR** `[2:0]`
    ///
    /// Indicates a programming error of HUK_GEN_STATE_HIGH.
    #[inline(always)]
    pub const fn km_huk_gen_state_high_err(&self) -> u32 {
        (self.bits & RdRepeatErr1::KM_HUK_GEN_STATE_HIGH_ERR_MASK)
            >> RdRepeatErr1::KM_HUK_GEN_STATE_HIGH_ERR_OFFSET
    }
}

impl Default for RdRepeatErr1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdRepeatErr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdRepeatErr1Val {
        RdRepeatErr1Val::from_bits(self.value.read_bits())
    }
}

/// RD_REPEAT_ERR2 Register
///
/// Programming error record register 2 of BLOCK0.
///
/// Address offset: 0x0184
///
/// ## Fields
///
/// - **FLASH_TPUW_ERR** `[31:28]` (RO)
///
///   Indicates a programming error of FLASH_TPUW.
///
/// - **DIS_USB_OTG_DOWNLOAD_MODE_ERR** `[27]` (RO)
///
///   Indicates a programming error of DIS_USB_OTG_DOWNLOAD_MODE.
///
/// - **FLASH_ECC_EN_ERR** `[26]` (RO)
///
///   Indicates a programming error of FLASH_ECC_EN.
///
/// - **FLASH_PAGE_SIZE_ERR** `[25:24]` (RO)
///
///   Indicates a programming error of FLASH_PAGE_SIZE.
///
/// - **FLASH_TYPE_ERR** `[23]` (RO)
///
///   Indicates a programming error of FLASH_TYPE.
///
/// - **SECURE_BOOT_AGGRESSIVE_REVOKE_ERR** `[21]` (RO)
///
///   Indicates a programming error of SECURE_BOOT_AGGRESSIVE_REVOKE.
///
/// - **SECURE_BOOT_EN_ERR** `[20]` (RO)
///
///   Indicates a programming error of SECURE_BOOT_EN.
///
/// - **CRYPT_DPA_ENABLE_ERR** `[19]` (RO)
///
///   Indicates a programming error of CRYPT_DPA_ENABLE.
///
/// - **ECDSA_ENABLE_SOFT_K_ERR** `[18]` (RO)
///
///   Indicates a programming error of ECDSA_FORCE_USE_HARDWARE_K.
///
/// - **SEC_DPA_LEVEL_ERR** `[17:16]` (RO)
///
///   Indicates a programming error of SEC_DPA_LEVEL.
///
/// - **KEY_PURPOSE_5_ERR** `[15:12]` (RO)
///
///   Indicates a programming error of KEY_PURPOSE_5.
///
/// - **KEY_PURPOSE_4_ERR** `[11:8]` (RO)
///
///   Indicates a programming error of KEY_PURPOSE_4.
///
/// - **KEY_PURPOSE_3_ERR** `[7:4]` (RO)
///
///   Indicates a programming error of KEY_PURPOSE_3.
///
/// - **KEY_PURPOSE_2_ERR** `[3:0]` (RO)
///
///   Indicates a programming error of KEY_PURPOSE_2.
#[repr(transparent)]
pub struct RdRepeatErr2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdRepeatErr2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdRepeatErr2 {
    // ======== Field Constants ========

    /// **FLASH_TPUW_ERR**
    ///
    /// Indicates a programming error of FLASH_TPUW.
    ///
    /// Bits `[31:28]`, mask `0xf0000000`
    pub const FLASH_TPUW_ERR_OFFSET: u32 = 28;
    pub const FLASH_TPUW_ERR_MASK: u32 = 0xf0000000;

    /// **DIS_USB_OTG_DOWNLOAD_MODE_ERR**
    ///
    /// Indicates a programming error of DIS_USB_OTG_DOWNLOAD_MODE.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const DIS_USB_OTG_DOWNLOAD_MODE_ERR_OFFSET: u32 = 27;
    pub const DIS_USB_OTG_DOWNLOAD_MODE_ERR_MASK: u32 = 0x08000000;
    pub const DIS_USB_OTG_DOWNLOAD_MODE_ERR: u32 = 1 << 27;

    /// **FLASH_ECC_EN_ERR**
    ///
    /// Indicates a programming error of FLASH_ECC_EN.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const FLASH_ECC_EN_ERR_OFFSET: u32 = 26;
    pub const FLASH_ECC_EN_ERR_MASK: u32 = 0x04000000;
    pub const FLASH_ECC_EN_ERR: u32 = 1 << 26;

    /// **FLASH_PAGE_SIZE_ERR**
    ///
    /// Indicates a programming error of FLASH_PAGE_SIZE.
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const FLASH_PAGE_SIZE_ERR_OFFSET: u32 = 24;
    pub const FLASH_PAGE_SIZE_ERR_MASK: u32 = 0x03000000;

    /// **FLASH_TYPE_ERR**
    ///
    /// Indicates a programming error of FLASH_TYPE.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const FLASH_TYPE_ERR_OFFSET: u32 = 23;
    pub const FLASH_TYPE_ERR_MASK: u32 = 0x00800000;
    pub const FLASH_TYPE_ERR: u32 = 1 << 23;

    /// **SECURE_BOOT_AGGRESSIVE_REVOKE_ERR**
    ///
    /// Indicates a programming error of SECURE_BOOT_AGGRESSIVE_REVOKE.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_OFFSET: u32 = 21;
    pub const SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_MASK: u32 = 0x00200000;
    pub const SECURE_BOOT_AGGRESSIVE_REVOKE_ERR: u32 = 1 << 21;

    /// **SECURE_BOOT_EN_ERR**
    ///
    /// Indicates a programming error of SECURE_BOOT_EN.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const SECURE_BOOT_EN_ERR_OFFSET: u32 = 20;
    pub const SECURE_BOOT_EN_ERR_MASK: u32 = 0x00100000;
    pub const SECURE_BOOT_EN_ERR: u32 = 1 << 20;

    /// **CRYPT_DPA_ENABLE_ERR**
    ///
    /// Indicates a programming error of CRYPT_DPA_ENABLE.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CRYPT_DPA_ENABLE_ERR_OFFSET: u32 = 19;
    pub const CRYPT_DPA_ENABLE_ERR_MASK: u32 = 0x00080000;
    pub const CRYPT_DPA_ENABLE_ERR: u32 = 1 << 19;

    /// **ECDSA_ENABLE_SOFT_K_ERR**
    ///
    /// Indicates a programming error of ECDSA_FORCE_USE_HARDWARE_K.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const ECDSA_ENABLE_SOFT_K_ERR_OFFSET: u32 = 18;
    pub const ECDSA_ENABLE_SOFT_K_ERR_MASK: u32 = 0x00040000;
    pub const ECDSA_ENABLE_SOFT_K_ERR: u32 = 1 << 18;

    /// **SEC_DPA_LEVEL_ERR**
    ///
    /// Indicates a programming error of SEC_DPA_LEVEL.
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const SEC_DPA_LEVEL_ERR_OFFSET: u32 = 16;
    pub const SEC_DPA_LEVEL_ERR_MASK: u32 = 0x00030000;

    /// **KEY_PURPOSE_5_ERR**
    ///
    /// Indicates a programming error of KEY_PURPOSE_5.
    ///
    /// Bits `[15:12]`, mask `0x0000f000`
    pub const KEY_PURPOSE_5_ERR_OFFSET: u32 = 12;
    pub const KEY_PURPOSE_5_ERR_MASK: u32 = 0x0000f000;

    /// **KEY_PURPOSE_4_ERR**
    ///
    /// Indicates a programming error of KEY_PURPOSE_4.
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const KEY_PURPOSE_4_ERR_OFFSET: u32 = 8;
    pub const KEY_PURPOSE_4_ERR_MASK: u32 = 0x00000f00;

    /// **KEY_PURPOSE_3_ERR**
    ///
    /// Indicates a programming error of KEY_PURPOSE_3.
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const KEY_PURPOSE_3_ERR_OFFSET: u32 = 4;
    pub const KEY_PURPOSE_3_ERR_MASK: u32 = 0x000000f0;

    /// **KEY_PURPOSE_2_ERR**
    ///
    /// Indicates a programming error of KEY_PURPOSE_2.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const KEY_PURPOSE_2_ERR_OFFSET: u32 = 0;
    pub const KEY_PURPOSE_2_ERR_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **FLASH_TPUW_ERR** `[31:28]` (RO)
    ///
    /// Indicates a programming error of FLASH_TPUW.
    #[inline(always)]
    pub fn flash_tpuw_err(&self) -> u32 {
        (self.value.read_bits() & Self::FLASH_TPUW_ERR_MASK) >> Self::FLASH_TPUW_ERR_OFFSET
    }

    /// **DIS_USB_OTG_DOWNLOAD_MODE_ERR** `[27]` (RO)
    ///
    /// Indicates a programming error of DIS_USB_OTG_DOWNLOAD_MODE.
    #[inline(always)]
    pub fn dis_usb_otg_download_mode_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_USB_OTG_DOWNLOAD_MODE_ERR_MASK) != 0
    }

    /// **FLASH_ECC_EN_ERR** `[26]` (RO)
    ///
    /// Indicates a programming error of FLASH_ECC_EN.
    #[inline(always)]
    pub fn flash_ecc_en_err(&self) -> bool {
        (self.value.read_bits() & Self::FLASH_ECC_EN_ERR_MASK) != 0
    }

    /// **FLASH_PAGE_SIZE_ERR** `[25:24]` (RO)
    ///
    /// Indicates a programming error of FLASH_PAGE_SIZE.
    #[inline(always)]
    pub fn flash_page_size_err(&self) -> u32 {
        (self.value.read_bits() & Self::FLASH_PAGE_SIZE_ERR_MASK)
            >> Self::FLASH_PAGE_SIZE_ERR_OFFSET
    }

    /// **FLASH_TYPE_ERR** `[23]` (RO)
    ///
    /// Indicates a programming error of FLASH_TYPE.
    #[inline(always)]
    pub fn flash_type_err(&self) -> bool {
        (self.value.read_bits() & Self::FLASH_TYPE_ERR_MASK) != 0
    }

    /// **SECURE_BOOT_AGGRESSIVE_REVOKE_ERR** `[21]` (RO)
    ///
    /// Indicates a programming error of SECURE_BOOT_AGGRESSIVE_REVOKE.
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke_err(&self) -> bool {
        (self.value.read_bits() & Self::SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_MASK) != 0
    }

    /// **SECURE_BOOT_EN_ERR** `[20]` (RO)
    ///
    /// Indicates a programming error of SECURE_BOOT_EN.
    #[inline(always)]
    pub fn secure_boot_en_err(&self) -> bool {
        (self.value.read_bits() & Self::SECURE_BOOT_EN_ERR_MASK) != 0
    }

    /// **CRYPT_DPA_ENABLE_ERR** `[19]` (RO)
    ///
    /// Indicates a programming error of CRYPT_DPA_ENABLE.
    #[inline(always)]
    pub fn crypt_dpa_enable_err(&self) -> bool {
        (self.value.read_bits() & Self::CRYPT_DPA_ENABLE_ERR_MASK) != 0
    }

    /// **ECDSA_ENABLE_SOFT_K_ERR** `[18]` (RO)
    ///
    /// Indicates a programming error of ECDSA_FORCE_USE_HARDWARE_K.
    #[inline(always)]
    pub fn ecdsa_enable_soft_k_err(&self) -> bool {
        (self.value.read_bits() & Self::ECDSA_ENABLE_SOFT_K_ERR_MASK) != 0
    }

    /// **SEC_DPA_LEVEL_ERR** `[17:16]` (RO)
    ///
    /// Indicates a programming error of SEC_DPA_LEVEL.
    #[inline(always)]
    pub fn sec_dpa_level_err(&self) -> u32 {
        (self.value.read_bits() & Self::SEC_DPA_LEVEL_ERR_MASK) >> Self::SEC_DPA_LEVEL_ERR_OFFSET
    }

    /// **KEY_PURPOSE_5_ERR** `[15:12]` (RO)
    ///
    /// Indicates a programming error of KEY_PURPOSE_5.
    #[inline(always)]
    pub fn key_purpose_5_err(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_PURPOSE_5_ERR_MASK) >> Self::KEY_PURPOSE_5_ERR_OFFSET
    }

    /// **KEY_PURPOSE_4_ERR** `[11:8]` (RO)
    ///
    /// Indicates a programming error of KEY_PURPOSE_4.
    #[inline(always)]
    pub fn key_purpose_4_err(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_PURPOSE_4_ERR_MASK) >> Self::KEY_PURPOSE_4_ERR_OFFSET
    }

    /// **KEY_PURPOSE_3_ERR** `[7:4]` (RO)
    ///
    /// Indicates a programming error of KEY_PURPOSE_3.
    #[inline(always)]
    pub fn key_purpose_3_err(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_PURPOSE_3_ERR_MASK) >> Self::KEY_PURPOSE_3_ERR_OFFSET
    }

    /// **KEY_PURPOSE_2_ERR** `[3:0]` (RO)
    ///
    /// Indicates a programming error of KEY_PURPOSE_2.
    #[inline(always)]
    pub fn key_purpose_2_err(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_PURPOSE_2_ERR_MASK) >> Self::KEY_PURPOSE_2_ERR_OFFSET
    }
}

/// RD_REPEAT_ERR2 value type for batch read/modify/write operations
///
/// Use `rdrepeaterr2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdrepeaterr2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdRepeatErr2Val {
    bits: u32,
}

impl RdRepeatErr2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **FLASH_TPUW_ERR** `[31:28]`
    ///
    /// Indicates a programming error of FLASH_TPUW.
    #[inline(always)]
    pub const fn flash_tpuw_err(&self) -> u32 {
        (self.bits & RdRepeatErr2::FLASH_TPUW_ERR_MASK) >> RdRepeatErr2::FLASH_TPUW_ERR_OFFSET
    }

    /// Read **DIS_USB_OTG_DOWNLOAD_MODE_ERR** `[27:27]`
    ///
    /// Indicates a programming error of DIS_USB_OTG_DOWNLOAD_MODE.
    #[inline(always)]
    pub const fn dis_usb_otg_download_mode_err(&self) -> bool {
        (self.bits & RdRepeatErr2::DIS_USB_OTG_DOWNLOAD_MODE_ERR_MASK) != 0
    }

    /// Read **FLASH_ECC_EN_ERR** `[26:26]`
    ///
    /// Indicates a programming error of FLASH_ECC_EN.
    #[inline(always)]
    pub const fn flash_ecc_en_err(&self) -> bool {
        (self.bits & RdRepeatErr2::FLASH_ECC_EN_ERR_MASK) != 0
    }

    /// Read **FLASH_PAGE_SIZE_ERR** `[25:24]`
    ///
    /// Indicates a programming error of FLASH_PAGE_SIZE.
    #[inline(always)]
    pub const fn flash_page_size_err(&self) -> u32 {
        (self.bits & RdRepeatErr2::FLASH_PAGE_SIZE_ERR_MASK)
            >> RdRepeatErr2::FLASH_PAGE_SIZE_ERR_OFFSET
    }

    /// Read **FLASH_TYPE_ERR** `[23:23]`
    ///
    /// Indicates a programming error of FLASH_TYPE.
    #[inline(always)]
    pub const fn flash_type_err(&self) -> bool {
        (self.bits & RdRepeatErr2::FLASH_TYPE_ERR_MASK) != 0
    }

    /// Read **SECURE_BOOT_AGGRESSIVE_REVOKE_ERR** `[21:21]`
    ///
    /// Indicates a programming error of SECURE_BOOT_AGGRESSIVE_REVOKE.
    #[inline(always)]
    pub const fn secure_boot_aggressive_revoke_err(&self) -> bool {
        (self.bits & RdRepeatErr2::SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_MASK) != 0
    }

    /// Read **SECURE_BOOT_EN_ERR** `[20:20]`
    ///
    /// Indicates a programming error of SECURE_BOOT_EN.
    #[inline(always)]
    pub const fn secure_boot_en_err(&self) -> bool {
        (self.bits & RdRepeatErr2::SECURE_BOOT_EN_ERR_MASK) != 0
    }

    /// Read **CRYPT_DPA_ENABLE_ERR** `[19:19]`
    ///
    /// Indicates a programming error of CRYPT_DPA_ENABLE.
    #[inline(always)]
    pub const fn crypt_dpa_enable_err(&self) -> bool {
        (self.bits & RdRepeatErr2::CRYPT_DPA_ENABLE_ERR_MASK) != 0
    }

    /// Read **ECDSA_ENABLE_SOFT_K_ERR** `[18:18]`
    ///
    /// Indicates a programming error of ECDSA_FORCE_USE_HARDWARE_K.
    #[inline(always)]
    pub const fn ecdsa_enable_soft_k_err(&self) -> bool {
        (self.bits & RdRepeatErr2::ECDSA_ENABLE_SOFT_K_ERR_MASK) != 0
    }

    /// Read **SEC_DPA_LEVEL_ERR** `[17:16]`
    ///
    /// Indicates a programming error of SEC_DPA_LEVEL.
    #[inline(always)]
    pub const fn sec_dpa_level_err(&self) -> u32 {
        (self.bits & RdRepeatErr2::SEC_DPA_LEVEL_ERR_MASK) >> RdRepeatErr2::SEC_DPA_LEVEL_ERR_OFFSET
    }

    /// Read **KEY_PURPOSE_5_ERR** `[15:12]`
    ///
    /// Indicates a programming error of KEY_PURPOSE_5.
    #[inline(always)]
    pub const fn key_purpose_5_err(&self) -> u32 {
        (self.bits & RdRepeatErr2::KEY_PURPOSE_5_ERR_MASK) >> RdRepeatErr2::KEY_PURPOSE_5_ERR_OFFSET
    }

    /// Read **KEY_PURPOSE_4_ERR** `[11:8]`
    ///
    /// Indicates a programming error of KEY_PURPOSE_4.
    #[inline(always)]
    pub const fn key_purpose_4_err(&self) -> u32 {
        (self.bits & RdRepeatErr2::KEY_PURPOSE_4_ERR_MASK) >> RdRepeatErr2::KEY_PURPOSE_4_ERR_OFFSET
    }

    /// Read **KEY_PURPOSE_3_ERR** `[7:4]`
    ///
    /// Indicates a programming error of KEY_PURPOSE_3.
    #[inline(always)]
    pub const fn key_purpose_3_err(&self) -> u32 {
        (self.bits & RdRepeatErr2::KEY_PURPOSE_3_ERR_MASK) >> RdRepeatErr2::KEY_PURPOSE_3_ERR_OFFSET
    }

    /// Read **KEY_PURPOSE_2_ERR** `[3:0]`
    ///
    /// Indicates a programming error of KEY_PURPOSE_2.
    #[inline(always)]
    pub const fn key_purpose_2_err(&self) -> u32 {
        (self.bits & RdRepeatErr2::KEY_PURPOSE_2_ERR_MASK) >> RdRepeatErr2::KEY_PURPOSE_2_ERR_OFFSET
    }
}

impl Default for RdRepeatErr2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdRepeatErr2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdRepeatErr2Val {
        RdRepeatErr2Val::from_bits(self.value.read_bits())
    }
}

/// RD_REPEAT_ERR3 Register
///
/// Programming error record register 3 of BLOCK0.
///
/// Address offset: 0x0188
///
/// ## Fields
///
/// - **DCDC_VSET_ERR** `[31:27]` (RO)
///
///   Indicates a programming error of DCDC_VSET.
///
/// - **HYS_EN_PAD_ERR** `[26]` (RO)
///
///   Indicates a programming error of HYS_EN_PAD.
///
/// - **SECURE_BOOT_DISABLE_FAST_WAKE_ERR** `[25]` (RO)
///
///   Indicates a programming error of SECURE_BOOT_DISABLE_FAST_WAKE.
///
/// - **SECURE_VERSION_ERR** `[24:9]` (RO)
///
///   Indicates a programming error of SECURE VERSION.
///
/// - **FORCE_SEND_RESUME_ERR** `[8]` (RO)
///
///   Indicates a programming error of FORCE_SEND_RESUME.
///
/// - **UART_PRINT_CONTROL_ERR** `[7:6]` (RO)
///
///   Indicates a programming error of UART_PRINT_CONTROL.
///
/// - **ENABLE_SECURITY_DOWNLOAD_ERR** `[5]` (RO)
///
///   Indicates a programming error of ENABLE_SECURITY_DOWNLOAD.
///
/// - **DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR** `[4]` (RO)
///
///   Indicates a programming error of DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE.
///
/// - **LOCK_KM_KEY_ERR** `[3]` (RO)
///
///   TBD
///
/// - **DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR** `[2]` (RO)
///
///   Indicates a programming error of DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR.
///
/// - **DIS_DIRECT_BOOT_ERR** `[1]` (RO)
///
///   Indicates a programming error of DIS_DIRECT_BOOT.
///
/// - **DIS_DOWNLOAD_MODE_ERR** `[0]` (RO)
///
///   Indicates a programming error of DIS_DOWNLOAD_MODE.
#[repr(transparent)]
pub struct RdRepeatErr3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdRepeatErr3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdRepeatErr3 {
    // ======== Field Constants ========

    /// **DCDC_VSET_ERR**
    ///
    /// Indicates a programming error of DCDC_VSET.
    ///
    /// Bits `[31:27]`, mask `0xf8000000`
    pub const DCDC_VSET_ERR_OFFSET: u32 = 27;
    pub const DCDC_VSET_ERR_MASK: u32 = 0xf8000000;

    /// **HYS_EN_PAD_ERR**
    ///
    /// Indicates a programming error of HYS_EN_PAD.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const HYS_EN_PAD_ERR_OFFSET: u32 = 26;
    pub const HYS_EN_PAD_ERR_MASK: u32 = 0x04000000;
    pub const HYS_EN_PAD_ERR: u32 = 1 << 26;

    /// **SECURE_BOOT_DISABLE_FAST_WAKE_ERR**
    ///
    /// Indicates a programming error of SECURE_BOOT_DISABLE_FAST_WAKE.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const SECURE_BOOT_DISABLE_FAST_WAKE_ERR_OFFSET: u32 = 25;
    pub const SECURE_BOOT_DISABLE_FAST_WAKE_ERR_MASK: u32 = 0x02000000;
    pub const SECURE_BOOT_DISABLE_FAST_WAKE_ERR: u32 = 1 << 25;

    /// **SECURE_VERSION_ERR**
    ///
    /// Indicates a programming error of SECURE VERSION.
    ///
    /// Bits `[24:9]`, mask `0x01fffe00`
    pub const SECURE_VERSION_ERR_OFFSET: u32 = 9;
    pub const SECURE_VERSION_ERR_MASK: u32 = 0x01fffe00;

    /// **FORCE_SEND_RESUME_ERR**
    ///
    /// Indicates a programming error of FORCE_SEND_RESUME.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const FORCE_SEND_RESUME_ERR_OFFSET: u32 = 8;
    pub const FORCE_SEND_RESUME_ERR_MASK: u32 = 0x00000100;
    pub const FORCE_SEND_RESUME_ERR: u32 = 1 << 8;

    /// **UART_PRINT_CONTROL_ERR**
    ///
    /// Indicates a programming error of UART_PRINT_CONTROL.
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const UART_PRINT_CONTROL_ERR_OFFSET: u32 = 6;
    pub const UART_PRINT_CONTROL_ERR_MASK: u32 = 0x000000c0;

    /// **ENABLE_SECURITY_DOWNLOAD_ERR**
    ///
    /// Indicates a programming error of ENABLE_SECURITY_DOWNLOAD.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ENABLE_SECURITY_DOWNLOAD_ERR_OFFSET: u32 = 5;
    pub const ENABLE_SECURITY_DOWNLOAD_ERR_MASK: u32 = 0x00000020;
    pub const ENABLE_SECURITY_DOWNLOAD_ERR: u32 = 1 << 5;

    /// **DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR**
    ///
    /// Indicates a programming error of DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR_OFFSET: u32 = 4;
    pub const DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR_MASK: u32 = 0x00000010;
    pub const DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR: u32 = 1 << 4;

    /// **LOCK_KM_KEY_ERR**
    ///
    /// TBD
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LOCK_KM_KEY_ERR_OFFSET: u32 = 3;
    pub const LOCK_KM_KEY_ERR_MASK: u32 = 0x00000008;
    pub const LOCK_KM_KEY_ERR: u32 = 1 << 3;

    /// **DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR**
    ///
    /// Indicates a programming error of DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR_OFFSET: u32 = 2;
    pub const DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR_MASK: u32 = 0x00000004;
    pub const DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR: u32 = 1 << 2;

    /// **DIS_DIRECT_BOOT_ERR**
    ///
    /// Indicates a programming error of DIS_DIRECT_BOOT.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DIS_DIRECT_BOOT_ERR_OFFSET: u32 = 1;
    pub const DIS_DIRECT_BOOT_ERR_MASK: u32 = 0x00000002;
    pub const DIS_DIRECT_BOOT_ERR: u32 = 1 << 1;

    /// **DIS_DOWNLOAD_MODE_ERR**
    ///
    /// Indicates a programming error of DIS_DOWNLOAD_MODE.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DIS_DOWNLOAD_MODE_ERR_OFFSET: u32 = 0;
    pub const DIS_DOWNLOAD_MODE_ERR_MASK: u32 = 0x00000001;
    pub const DIS_DOWNLOAD_MODE_ERR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DCDC_VSET_ERR** `[31:27]` (RO)
    ///
    /// Indicates a programming error of DCDC_VSET.
    #[inline(always)]
    pub fn dcdc_vset_err(&self) -> u32 {
        (self.value.read_bits() & Self::DCDC_VSET_ERR_MASK) >> Self::DCDC_VSET_ERR_OFFSET
    }

    /// **HYS_EN_PAD_ERR** `[26]` (RO)
    ///
    /// Indicates a programming error of HYS_EN_PAD.
    #[inline(always)]
    pub fn hys_en_pad_err(&self) -> bool {
        (self.value.read_bits() & Self::HYS_EN_PAD_ERR_MASK) != 0
    }

    /// **SECURE_BOOT_DISABLE_FAST_WAKE_ERR** `[25]` (RO)
    ///
    /// Indicates a programming error of SECURE_BOOT_DISABLE_FAST_WAKE.
    #[inline(always)]
    pub fn secure_boot_disable_fast_wake_err(&self) -> bool {
        (self.value.read_bits() & Self::SECURE_BOOT_DISABLE_FAST_WAKE_ERR_MASK) != 0
    }

    /// **SECURE_VERSION_ERR** `[24:9]` (RO)
    ///
    /// Indicates a programming error of SECURE VERSION.
    #[inline(always)]
    pub fn secure_version_err(&self) -> u32 {
        (self.value.read_bits() & Self::SECURE_VERSION_ERR_MASK) >> Self::SECURE_VERSION_ERR_OFFSET
    }

    /// **FORCE_SEND_RESUME_ERR** `[8]` (RO)
    ///
    /// Indicates a programming error of FORCE_SEND_RESUME.
    #[inline(always)]
    pub fn force_send_resume_err(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_SEND_RESUME_ERR_MASK) != 0
    }

    /// **UART_PRINT_CONTROL_ERR** `[7:6]` (RO)
    ///
    /// Indicates a programming error of UART_PRINT_CONTROL.
    #[inline(always)]
    pub fn uart_print_control_err(&self) -> u32 {
        (self.value.read_bits() & Self::UART_PRINT_CONTROL_ERR_MASK)
            >> Self::UART_PRINT_CONTROL_ERR_OFFSET
    }

    /// **ENABLE_SECURITY_DOWNLOAD_ERR** `[5]` (RO)
    ///
    /// Indicates a programming error of ENABLE_SECURITY_DOWNLOAD.
    #[inline(always)]
    pub fn enable_security_download_err(&self) -> bool {
        (self.value.read_bits() & Self::ENABLE_SECURITY_DOWNLOAD_ERR_MASK) != 0
    }

    /// **DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR** `[4]` (RO)
    ///
    /// Indicates a programming error of DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE.
    #[inline(always)]
    pub fn dis_usb_serial_jtag_download_mode_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR_MASK) != 0
    }

    /// **LOCK_KM_KEY_ERR** `[3]` (RO)
    ///
    /// TBD
    #[inline(always)]
    pub fn lock_km_key_err(&self) -> bool {
        (self.value.read_bits() & Self::LOCK_KM_KEY_ERR_MASK) != 0
    }

    /// **DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR** `[2]` (RO)
    ///
    /// Indicates a programming error of DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR.
    #[inline(always)]
    pub fn dis_usb_serial_jtag_rom_print_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR_MASK) != 0
    }

    /// **DIS_DIRECT_BOOT_ERR** `[1]` (RO)
    ///
    /// Indicates a programming error of DIS_DIRECT_BOOT.
    #[inline(always)]
    pub fn dis_direct_boot_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_DIRECT_BOOT_ERR_MASK) != 0
    }

    /// **DIS_DOWNLOAD_MODE_ERR** `[0]` (RO)
    ///
    /// Indicates a programming error of DIS_DOWNLOAD_MODE.
    #[inline(always)]
    pub fn dis_download_mode_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_DOWNLOAD_MODE_ERR_MASK) != 0
    }
}

/// RD_REPEAT_ERR3 value type for batch read/modify/write operations
///
/// Use `rdrepeaterr3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdrepeaterr3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdRepeatErr3Val {
    bits: u32,
}

impl RdRepeatErr3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DCDC_VSET_ERR** `[31:27]`
    ///
    /// Indicates a programming error of DCDC_VSET.
    #[inline(always)]
    pub const fn dcdc_vset_err(&self) -> u32 {
        (self.bits & RdRepeatErr3::DCDC_VSET_ERR_MASK) >> RdRepeatErr3::DCDC_VSET_ERR_OFFSET
    }

    /// Read **HYS_EN_PAD_ERR** `[26:26]`
    ///
    /// Indicates a programming error of HYS_EN_PAD.
    #[inline(always)]
    pub const fn hys_en_pad_err(&self) -> bool {
        (self.bits & RdRepeatErr3::HYS_EN_PAD_ERR_MASK) != 0
    }

    /// Read **SECURE_BOOT_DISABLE_FAST_WAKE_ERR** `[25:25]`
    ///
    /// Indicates a programming error of SECURE_BOOT_DISABLE_FAST_WAKE.
    #[inline(always)]
    pub const fn secure_boot_disable_fast_wake_err(&self) -> bool {
        (self.bits & RdRepeatErr3::SECURE_BOOT_DISABLE_FAST_WAKE_ERR_MASK) != 0
    }

    /// Read **SECURE_VERSION_ERR** `[24:9]`
    ///
    /// Indicates a programming error of SECURE VERSION.
    #[inline(always)]
    pub const fn secure_version_err(&self) -> u32 {
        (self.bits & RdRepeatErr3::SECURE_VERSION_ERR_MASK)
            >> RdRepeatErr3::SECURE_VERSION_ERR_OFFSET
    }

    /// Read **FORCE_SEND_RESUME_ERR** `[8:8]`
    ///
    /// Indicates a programming error of FORCE_SEND_RESUME.
    #[inline(always)]
    pub const fn force_send_resume_err(&self) -> bool {
        (self.bits & RdRepeatErr3::FORCE_SEND_RESUME_ERR_MASK) != 0
    }

    /// Read **UART_PRINT_CONTROL_ERR** `[7:6]`
    ///
    /// Indicates a programming error of UART_PRINT_CONTROL.
    #[inline(always)]
    pub const fn uart_print_control_err(&self) -> u32 {
        (self.bits & RdRepeatErr3::UART_PRINT_CONTROL_ERR_MASK)
            >> RdRepeatErr3::UART_PRINT_CONTROL_ERR_OFFSET
    }

    /// Read **ENABLE_SECURITY_DOWNLOAD_ERR** `[5:5]`
    ///
    /// Indicates a programming error of ENABLE_SECURITY_DOWNLOAD.
    #[inline(always)]
    pub const fn enable_security_download_err(&self) -> bool {
        (self.bits & RdRepeatErr3::ENABLE_SECURITY_DOWNLOAD_ERR_MASK) != 0
    }

    /// Read **DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR** `[4:4]`
    ///
    /// Indicates a programming error of DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE.
    #[inline(always)]
    pub const fn dis_usb_serial_jtag_download_mode_err(&self) -> bool {
        (self.bits & RdRepeatErr3::DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR_MASK) != 0
    }

    /// Read **LOCK_KM_KEY_ERR** `[3:3]`
    ///
    /// TBD
    #[inline(always)]
    pub const fn lock_km_key_err(&self) -> bool {
        (self.bits & RdRepeatErr3::LOCK_KM_KEY_ERR_MASK) != 0
    }

    /// Read **DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR** `[2:2]`
    ///
    /// Indicates a programming error of DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR.
    #[inline(always)]
    pub const fn dis_usb_serial_jtag_rom_print_err(&self) -> bool {
        (self.bits & RdRepeatErr3::DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR_MASK) != 0
    }

    /// Read **DIS_DIRECT_BOOT_ERR** `[1:1]`
    ///
    /// Indicates a programming error of DIS_DIRECT_BOOT.
    #[inline(always)]
    pub const fn dis_direct_boot_err(&self) -> bool {
        (self.bits & RdRepeatErr3::DIS_DIRECT_BOOT_ERR_MASK) != 0
    }

    /// Read **DIS_DOWNLOAD_MODE_ERR** `[0:0]`
    ///
    /// Indicates a programming error of DIS_DOWNLOAD_MODE.
    #[inline(always)]
    pub const fn dis_download_mode_err(&self) -> bool {
        (self.bits & RdRepeatErr3::DIS_DOWNLOAD_MODE_ERR_MASK) != 0
    }
}

impl Default for RdRepeatErr3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdRepeatErr3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdRepeatErr3Val {
        RdRepeatErr3Val::from_bits(self.value.read_bits())
    }
}

/// RD_REPEAT_ERR4 Register
///
/// Programming error record register 4 of BLOCK0.
///
/// Address offset: 0x018c
///
/// ## Fields
///
/// - **DIS_SWD_ERR** `[21]` (RO)
///
///   Indicates a programming error of DIS_SWD.
///
/// - **DIS_WDT_ERR** `[20]` (RO)
///
///   Indicates a programming error of DIS_WDT.
///
/// - **DCDC_VSET_EN_ERR** `[19]` (RO)
///
///   Indicates a programming error of DCDC_VSET_EN.
///
/// - **HP_PWR_SRC_SEL_ERR** `[18]` (RO)
///
///   Indicates a programming error of HP_PWR_SRC_SEL.
///
/// - **USB_OTG11_DREFL_ERR** `[15:14]` (RO)
///
///   Indicates a programming error of USB_OTG11_DREFL.
///
/// - **USB_DEVICE_DREFL_ERR** `[13:12]` (RO)
///
///   Indicates a programming error of USB_DEVICE_DREFL.
///
/// - **KM_DISABLE_DEPLOY_MODE_ERR** `[11:8]` (RO)
///
///   TBD.
///
/// - **_0PXA_TIEH_SEL_3_ERR** `[7:6]` (RO)
///
///   Indicates a programming error of 0PXA_TIEH_SEL_3.
///
/// - **_0PXA_TIEH_SEL_2_ERR** `[5:4]` (RO)
///
///   Indicates a programming error of 0PXA_TIEH_SEL_2.
///
/// - **_0PXA_TIEH_SEL_1_ERR** `[3:2]` (RO)
///
///   Indicates a programming error of 0PXA_TIEH_SEL_1.
///
/// - **_0PXA_TIEH_SEL_0_ERR** `[1:0]` (RO)
///
///   Indicates a programming error of 0PXA_TIEH_SEL_0.
#[repr(transparent)]
pub struct RdRepeatErr4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdRepeatErr4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdRepeatErr4 {
    // ======== Field Constants ========

    /// **DIS_SWD_ERR**
    ///
    /// Indicates a programming error of DIS_SWD.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const DIS_SWD_ERR_OFFSET: u32 = 21;
    pub const DIS_SWD_ERR_MASK: u32 = 0x00200000;
    pub const DIS_SWD_ERR: u32 = 1 << 21;

    /// **DIS_WDT_ERR**
    ///
    /// Indicates a programming error of DIS_WDT.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const DIS_WDT_ERR_OFFSET: u32 = 20;
    pub const DIS_WDT_ERR_MASK: u32 = 0x00100000;
    pub const DIS_WDT_ERR: u32 = 1 << 20;

    /// **DCDC_VSET_EN_ERR**
    ///
    /// Indicates a programming error of DCDC_VSET_EN.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const DCDC_VSET_EN_ERR_OFFSET: u32 = 19;
    pub const DCDC_VSET_EN_ERR_MASK: u32 = 0x00080000;
    pub const DCDC_VSET_EN_ERR: u32 = 1 << 19;

    /// **HP_PWR_SRC_SEL_ERR**
    ///
    /// Indicates a programming error of HP_PWR_SRC_SEL.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const HP_PWR_SRC_SEL_ERR_OFFSET: u32 = 18;
    pub const HP_PWR_SRC_SEL_ERR_MASK: u32 = 0x00040000;
    pub const HP_PWR_SRC_SEL_ERR: u32 = 1 << 18;

    /// **USB_OTG11_DREFL_ERR**
    ///
    /// Indicates a programming error of USB_OTG11_DREFL.
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const USB_OTG11_DREFL_ERR_OFFSET: u32 = 14;
    pub const USB_OTG11_DREFL_ERR_MASK: u32 = 0x0000c000;

    /// **USB_DEVICE_DREFL_ERR**
    ///
    /// Indicates a programming error of USB_DEVICE_DREFL.
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const USB_DEVICE_DREFL_ERR_OFFSET: u32 = 12;
    pub const USB_DEVICE_DREFL_ERR_MASK: u32 = 0x00003000;

    /// **KM_DISABLE_DEPLOY_MODE_ERR**
    ///
    /// TBD.
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const KM_DISABLE_DEPLOY_MODE_ERR_OFFSET: u32 = 8;
    pub const KM_DISABLE_DEPLOY_MODE_ERR_MASK: u32 = 0x00000f00;

    /// **_0PXA_TIEH_SEL_3_ERR**
    ///
    /// Indicates a programming error of 0PXA_TIEH_SEL_3.
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const _0PXA_TIEH_SEL_3_ERR_OFFSET: u32 = 6;
    pub const _0PXA_TIEH_SEL_3_ERR_MASK: u32 = 0x000000c0;

    /// **_0PXA_TIEH_SEL_2_ERR**
    ///
    /// Indicates a programming error of 0PXA_TIEH_SEL_2.
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const _0PXA_TIEH_SEL_2_ERR_OFFSET: u32 = 4;
    pub const _0PXA_TIEH_SEL_2_ERR_MASK: u32 = 0x00000030;

    /// **_0PXA_TIEH_SEL_1_ERR**
    ///
    /// Indicates a programming error of 0PXA_TIEH_SEL_1.
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const _0PXA_TIEH_SEL_1_ERR_OFFSET: u32 = 2;
    pub const _0PXA_TIEH_SEL_1_ERR_MASK: u32 = 0x0000000c;

    /// **_0PXA_TIEH_SEL_0_ERR**
    ///
    /// Indicates a programming error of 0PXA_TIEH_SEL_0.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const _0PXA_TIEH_SEL_0_ERR_OFFSET: u32 = 0;
    pub const _0PXA_TIEH_SEL_0_ERR_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **DIS_SWD_ERR** `[21]` (RO)
    ///
    /// Indicates a programming error of DIS_SWD.
    #[inline(always)]
    pub fn dis_swd_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_SWD_ERR_MASK) != 0
    }

    /// **DIS_WDT_ERR** `[20]` (RO)
    ///
    /// Indicates a programming error of DIS_WDT.
    #[inline(always)]
    pub fn dis_wdt_err(&self) -> bool {
        (self.value.read_bits() & Self::DIS_WDT_ERR_MASK) != 0
    }

    /// **DCDC_VSET_EN_ERR** `[19]` (RO)
    ///
    /// Indicates a programming error of DCDC_VSET_EN.
    #[inline(always)]
    pub fn dcdc_vset_en_err(&self) -> bool {
        (self.value.read_bits() & Self::DCDC_VSET_EN_ERR_MASK) != 0
    }

    /// **HP_PWR_SRC_SEL_ERR** `[18]` (RO)
    ///
    /// Indicates a programming error of HP_PWR_SRC_SEL.
    #[inline(always)]
    pub fn hp_pwr_src_sel_err(&self) -> bool {
        (self.value.read_bits() & Self::HP_PWR_SRC_SEL_ERR_MASK) != 0
    }

    /// **USB_OTG11_DREFL_ERR** `[15:14]` (RO)
    ///
    /// Indicates a programming error of USB_OTG11_DREFL.
    #[inline(always)]
    pub fn usb_otg11_drefl_err(&self) -> u32 {
        (self.value.read_bits() & Self::USB_OTG11_DREFL_ERR_MASK)
            >> Self::USB_OTG11_DREFL_ERR_OFFSET
    }

    /// **USB_DEVICE_DREFL_ERR** `[13:12]` (RO)
    ///
    /// Indicates a programming error of USB_DEVICE_DREFL.
    #[inline(always)]
    pub fn usb_device_drefl_err(&self) -> u32 {
        (self.value.read_bits() & Self::USB_DEVICE_DREFL_ERR_MASK)
            >> Self::USB_DEVICE_DREFL_ERR_OFFSET
    }

    /// **KM_DISABLE_DEPLOY_MODE_ERR** `[11:8]` (RO)
    ///
    /// TBD.
    #[inline(always)]
    pub fn km_disable_deploy_mode_err(&self) -> u32 {
        (self.value.read_bits() & Self::KM_DISABLE_DEPLOY_MODE_ERR_MASK)
            >> Self::KM_DISABLE_DEPLOY_MODE_ERR_OFFSET
    }

    /// **_0PXA_TIEH_SEL_3_ERR** `[7:6]` (RO)
    ///
    /// Indicates a programming error of 0PXA_TIEH_SEL_3.
    #[inline(always)]
    pub fn _0pxa_tieh_sel_3_err(&self) -> u32 {
        (self.value.read_bits() & Self::_0PXA_TIEH_SEL_3_ERR_MASK)
            >> Self::_0PXA_TIEH_SEL_3_ERR_OFFSET
    }

    /// **_0PXA_TIEH_SEL_2_ERR** `[5:4]` (RO)
    ///
    /// Indicates a programming error of 0PXA_TIEH_SEL_2.
    #[inline(always)]
    pub fn _0pxa_tieh_sel_2_err(&self) -> u32 {
        (self.value.read_bits() & Self::_0PXA_TIEH_SEL_2_ERR_MASK)
            >> Self::_0PXA_TIEH_SEL_2_ERR_OFFSET
    }

    /// **_0PXA_TIEH_SEL_1_ERR** `[3:2]` (RO)
    ///
    /// Indicates a programming error of 0PXA_TIEH_SEL_1.
    #[inline(always)]
    pub fn _0pxa_tieh_sel_1_err(&self) -> u32 {
        (self.value.read_bits() & Self::_0PXA_TIEH_SEL_1_ERR_MASK)
            >> Self::_0PXA_TIEH_SEL_1_ERR_OFFSET
    }

    /// **_0PXA_TIEH_SEL_0_ERR** `[1:0]` (RO)
    ///
    /// Indicates a programming error of 0PXA_TIEH_SEL_0.
    #[inline(always)]
    pub fn _0pxa_tieh_sel_0_err(&self) -> u32 {
        (self.value.read_bits() & Self::_0PXA_TIEH_SEL_0_ERR_MASK)
            >> Self::_0PXA_TIEH_SEL_0_ERR_OFFSET
    }
}

/// RD_REPEAT_ERR4 value type for batch read/modify/write operations
///
/// Use `rdrepeaterr4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdrepeaterr4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdRepeatErr4Val {
    bits: u32,
}

impl RdRepeatErr4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DIS_SWD_ERR** `[21:21]`
    ///
    /// Indicates a programming error of DIS_SWD.
    #[inline(always)]
    pub const fn dis_swd_err(&self) -> bool {
        (self.bits & RdRepeatErr4::DIS_SWD_ERR_MASK) != 0
    }

    /// Read **DIS_WDT_ERR** `[20:20]`
    ///
    /// Indicates a programming error of DIS_WDT.
    #[inline(always)]
    pub const fn dis_wdt_err(&self) -> bool {
        (self.bits & RdRepeatErr4::DIS_WDT_ERR_MASK) != 0
    }

    /// Read **DCDC_VSET_EN_ERR** `[19:19]`
    ///
    /// Indicates a programming error of DCDC_VSET_EN.
    #[inline(always)]
    pub const fn dcdc_vset_en_err(&self) -> bool {
        (self.bits & RdRepeatErr4::DCDC_VSET_EN_ERR_MASK) != 0
    }

    /// Read **HP_PWR_SRC_SEL_ERR** `[18:18]`
    ///
    /// Indicates a programming error of HP_PWR_SRC_SEL.
    #[inline(always)]
    pub const fn hp_pwr_src_sel_err(&self) -> bool {
        (self.bits & RdRepeatErr4::HP_PWR_SRC_SEL_ERR_MASK) != 0
    }

    /// Read **USB_OTG11_DREFL_ERR** `[15:14]`
    ///
    /// Indicates a programming error of USB_OTG11_DREFL.
    #[inline(always)]
    pub const fn usb_otg11_drefl_err(&self) -> u32 {
        (self.bits & RdRepeatErr4::USB_OTG11_DREFL_ERR_MASK)
            >> RdRepeatErr4::USB_OTG11_DREFL_ERR_OFFSET
    }

    /// Read **USB_DEVICE_DREFL_ERR** `[13:12]`
    ///
    /// Indicates a programming error of USB_DEVICE_DREFL.
    #[inline(always)]
    pub const fn usb_device_drefl_err(&self) -> u32 {
        (self.bits & RdRepeatErr4::USB_DEVICE_DREFL_ERR_MASK)
            >> RdRepeatErr4::USB_DEVICE_DREFL_ERR_OFFSET
    }

    /// Read **KM_DISABLE_DEPLOY_MODE_ERR** `[11:8]`
    ///
    /// TBD.
    #[inline(always)]
    pub const fn km_disable_deploy_mode_err(&self) -> u32 {
        (self.bits & RdRepeatErr4::KM_DISABLE_DEPLOY_MODE_ERR_MASK)
            >> RdRepeatErr4::KM_DISABLE_DEPLOY_MODE_ERR_OFFSET
    }

    /// Read **_0PXA_TIEH_SEL_3_ERR** `[7:6]`
    ///
    /// Indicates a programming error of 0PXA_TIEH_SEL_3.
    #[inline(always)]
    pub const fn _0pxa_tieh_sel_3_err(&self) -> u32 {
        (self.bits & RdRepeatErr4::_0PXA_TIEH_SEL_3_ERR_MASK)
            >> RdRepeatErr4::_0PXA_TIEH_SEL_3_ERR_OFFSET
    }

    /// Read **_0PXA_TIEH_SEL_2_ERR** `[5:4]`
    ///
    /// Indicates a programming error of 0PXA_TIEH_SEL_2.
    #[inline(always)]
    pub const fn _0pxa_tieh_sel_2_err(&self) -> u32 {
        (self.bits & RdRepeatErr4::_0PXA_TIEH_SEL_2_ERR_MASK)
            >> RdRepeatErr4::_0PXA_TIEH_SEL_2_ERR_OFFSET
    }

    /// Read **_0PXA_TIEH_SEL_1_ERR** `[3:2]`
    ///
    /// Indicates a programming error of 0PXA_TIEH_SEL_1.
    #[inline(always)]
    pub const fn _0pxa_tieh_sel_1_err(&self) -> u32 {
        (self.bits & RdRepeatErr4::_0PXA_TIEH_SEL_1_ERR_MASK)
            >> RdRepeatErr4::_0PXA_TIEH_SEL_1_ERR_OFFSET
    }

    /// Read **_0PXA_TIEH_SEL_0_ERR** `[1:0]`
    ///
    /// Indicates a programming error of 0PXA_TIEH_SEL_0.
    #[inline(always)]
    pub const fn _0pxa_tieh_sel_0_err(&self) -> u32 {
        (self.bits & RdRepeatErr4::_0PXA_TIEH_SEL_0_ERR_MASK)
            >> RdRepeatErr4::_0PXA_TIEH_SEL_0_ERR_OFFSET
    }
}

impl Default for RdRepeatErr4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdRepeatErr4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdRepeatErr4Val {
        RdRepeatErr4Val::from_bits(self.value.read_bits())
    }
}

/// RD_RS_ERR0 Register
///
/// Programming error record register 0 of BLOCK1-10.
///
/// Address offset: 0x01c0
///
/// ## Fields
///
/// - **KEY4_FAIL** `[31]` (RO)
///
///   0: Means no failure and that the data of key4 is reliable 1: Means that programming key4 failed and the number of error bytes is over 6.
///
/// - **KEY4_ERR_NUM** `[30:28]` (RO)
///
///   The value of this signal means the number of error bytes.
///
/// - **KEY3_FAIL** `[27]` (RO)
///
///   0: Means no failure and that the data of key3 is reliable 1: Means that programming key3 failed and the number of error bytes is over 6.
///
/// - **KEY3_ERR_NUM** `[26:24]` (RO)
///
///   The value of this signal means the number of error bytes.
///
/// - **KEY2_FAIL** `[23]` (RO)
///
///   0: Means no failure and that the data of key2 is reliable 1: Means that programming key2 failed and the number of error bytes is over 6.
///
/// - **KEY2_ERR_NUM** `[22:20]` (RO)
///
///   The value of this signal means the number of error bytes.
///
/// - **KEY1_FAIL** `[19]` (RO)
///
///   0: Means no failure and that the data of key1 is reliable 1: Means that programming key1 failed and the number of error bytes is over 6.
///
/// - **KEY1_ERR_NUM** `[18:16]` (RO)
///
///   The value of this signal means the number of error bytes.
///
/// - **KEY0_FAIL** `[15]` (RO)
///
///   0: Means no failure and that the data of key0 is reliable 1: Means that programming key0 failed and the number of error bytes is over 6.
///
/// - **KEY0_ERR_NUM** `[14:12]` (RO)
///
///   The value of this signal means the number of error bytes.
///
/// - **USR_DATA_FAIL** `[11]` (RO)
///
///   0: Means no failure and that the user data is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
///
/// - **USR_DATA_ERR_NUM** `[10:8]` (RO)
///
///   The value of this signal means the number of error bytes.
///
/// - **SYS_PART1_FAIL** `[7]` (RO)
///
///   0: Means no failure and that the data of system part1 is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
///
/// - **SYS_PART1_ERR_NUM** `[6:4]` (RO)
///
///   The value of this signal means the number of error bytes.
///
/// - **MAC_SYS_FAIL** `[3]` (RO)
///
///   0: Means no failure and that the data of MAC_SPI_8M is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
///
/// - **MAC_SYS_ERR_NUM** `[2:0]` (RO)
///
///   The value of this signal means the number of error bytes.
#[repr(transparent)]
pub struct RdRsErr0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdRsErr0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdRsErr0 {
    // ======== Field Constants ========

    /// **KEY4_FAIL**
    ///
    /// 0: Means no failure and that the data of key4 is reliable 1: Means that programming key4 failed and the number of error bytes is over 6.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const KEY4_FAIL_OFFSET: u32 = 31;
    pub const KEY4_FAIL_MASK: u32 = 0x80000000;
    pub const KEY4_FAIL: u32 = 1 << 31;

    /// **KEY4_ERR_NUM**
    ///
    /// The value of this signal means the number of error bytes.
    ///
    /// Bits `[30:28]`, mask `0x70000000`
    pub const KEY4_ERR_NUM_OFFSET: u32 = 28;
    pub const KEY4_ERR_NUM_MASK: u32 = 0x70000000;

    /// **KEY3_FAIL**
    ///
    /// 0: Means no failure and that the data of key3 is reliable 1: Means that programming key3 failed and the number of error bytes is over 6.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const KEY3_FAIL_OFFSET: u32 = 27;
    pub const KEY3_FAIL_MASK: u32 = 0x08000000;
    pub const KEY3_FAIL: u32 = 1 << 27;

    /// **KEY3_ERR_NUM**
    ///
    /// The value of this signal means the number of error bytes.
    ///
    /// Bits `[26:24]`, mask `0x07000000`
    pub const KEY3_ERR_NUM_OFFSET: u32 = 24;
    pub const KEY3_ERR_NUM_MASK: u32 = 0x07000000;

    /// **KEY2_FAIL**
    ///
    /// 0: Means no failure and that the data of key2 is reliable 1: Means that programming key2 failed and the number of error bytes is over 6.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const KEY2_FAIL_OFFSET: u32 = 23;
    pub const KEY2_FAIL_MASK: u32 = 0x00800000;
    pub const KEY2_FAIL: u32 = 1 << 23;

    /// **KEY2_ERR_NUM**
    ///
    /// The value of this signal means the number of error bytes.
    ///
    /// Bits `[22:20]`, mask `0x00700000`
    pub const KEY2_ERR_NUM_OFFSET: u32 = 20;
    pub const KEY2_ERR_NUM_MASK: u32 = 0x00700000;

    /// **KEY1_FAIL**
    ///
    /// 0: Means no failure and that the data of key1 is reliable 1: Means that programming key1 failed and the number of error bytes is over 6.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const KEY1_FAIL_OFFSET: u32 = 19;
    pub const KEY1_FAIL_MASK: u32 = 0x00080000;
    pub const KEY1_FAIL: u32 = 1 << 19;

    /// **KEY1_ERR_NUM**
    ///
    /// The value of this signal means the number of error bytes.
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const KEY1_ERR_NUM_OFFSET: u32 = 16;
    pub const KEY1_ERR_NUM_MASK: u32 = 0x00070000;

    /// **KEY0_FAIL**
    ///
    /// 0: Means no failure and that the data of key0 is reliable 1: Means that programming key0 failed and the number of error bytes is over 6.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const KEY0_FAIL_OFFSET: u32 = 15;
    pub const KEY0_FAIL_MASK: u32 = 0x00008000;
    pub const KEY0_FAIL: u32 = 1 << 15;

    /// **KEY0_ERR_NUM**
    ///
    /// The value of this signal means the number of error bytes.
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const KEY0_ERR_NUM_OFFSET: u32 = 12;
    pub const KEY0_ERR_NUM_MASK: u32 = 0x00007000;

    /// **USR_DATA_FAIL**
    ///
    /// 0: Means no failure and that the user data is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const USR_DATA_FAIL_OFFSET: u32 = 11;
    pub const USR_DATA_FAIL_MASK: u32 = 0x00000800;
    pub const USR_DATA_FAIL: u32 = 1 << 11;

    /// **USR_DATA_ERR_NUM**
    ///
    /// The value of this signal means the number of error bytes.
    ///
    /// Bits `[10:8]`, mask `0x00000700`
    pub const USR_DATA_ERR_NUM_OFFSET: u32 = 8;
    pub const USR_DATA_ERR_NUM_MASK: u32 = 0x00000700;

    /// **SYS_PART1_FAIL**
    ///
    /// 0: Means no failure and that the data of system part1 is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SYS_PART1_FAIL_OFFSET: u32 = 7;
    pub const SYS_PART1_FAIL_MASK: u32 = 0x00000080;
    pub const SYS_PART1_FAIL: u32 = 1 << 7;

    /// **SYS_PART1_ERR_NUM**
    ///
    /// The value of this signal means the number of error bytes.
    ///
    /// Bits `[6:4]`, mask `0x00000070`
    pub const SYS_PART1_ERR_NUM_OFFSET: u32 = 4;
    pub const SYS_PART1_ERR_NUM_MASK: u32 = 0x00000070;

    /// **MAC_SYS_FAIL**
    ///
    /// 0: Means no failure and that the data of MAC_SPI_8M is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MAC_SYS_FAIL_OFFSET: u32 = 3;
    pub const MAC_SYS_FAIL_MASK: u32 = 0x00000008;
    pub const MAC_SYS_FAIL: u32 = 1 << 3;

    /// **MAC_SYS_ERR_NUM**
    ///
    /// The value of this signal means the number of error bytes.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const MAC_SYS_ERR_NUM_OFFSET: u32 = 0;
    pub const MAC_SYS_ERR_NUM_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **KEY4_FAIL** `[31]` (RO)
    ///
    /// 0: Means no failure and that the data of key4 is reliable 1: Means that programming key4 failed and the number of error bytes is over 6.
    #[inline(always)]
    pub fn key4_fail(&self) -> bool {
        (self.value.read_bits() & Self::KEY4_FAIL_MASK) != 0
    }

    /// **KEY4_ERR_NUM** `[30:28]` (RO)
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub fn key4_err_num(&self) -> u32 {
        (self.value.read_bits() & Self::KEY4_ERR_NUM_MASK) >> Self::KEY4_ERR_NUM_OFFSET
    }

    /// **KEY3_FAIL** `[27]` (RO)
    ///
    /// 0: Means no failure and that the data of key3 is reliable 1: Means that programming key3 failed and the number of error bytes is over 6.
    #[inline(always)]
    pub fn key3_fail(&self) -> bool {
        (self.value.read_bits() & Self::KEY3_FAIL_MASK) != 0
    }

    /// **KEY3_ERR_NUM** `[26:24]` (RO)
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub fn key3_err_num(&self) -> u32 {
        (self.value.read_bits() & Self::KEY3_ERR_NUM_MASK) >> Self::KEY3_ERR_NUM_OFFSET
    }

    /// **KEY2_FAIL** `[23]` (RO)
    ///
    /// 0: Means no failure and that the data of key2 is reliable 1: Means that programming key2 failed and the number of error bytes is over 6.
    #[inline(always)]
    pub fn key2_fail(&self) -> bool {
        (self.value.read_bits() & Self::KEY2_FAIL_MASK) != 0
    }

    /// **KEY2_ERR_NUM** `[22:20]` (RO)
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub fn key2_err_num(&self) -> u32 {
        (self.value.read_bits() & Self::KEY2_ERR_NUM_MASK) >> Self::KEY2_ERR_NUM_OFFSET
    }

    /// **KEY1_FAIL** `[19]` (RO)
    ///
    /// 0: Means no failure and that the data of key1 is reliable 1: Means that programming key1 failed and the number of error bytes is over 6.
    #[inline(always)]
    pub fn key1_fail(&self) -> bool {
        (self.value.read_bits() & Self::KEY1_FAIL_MASK) != 0
    }

    /// **KEY1_ERR_NUM** `[18:16]` (RO)
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub fn key1_err_num(&self) -> u32 {
        (self.value.read_bits() & Self::KEY1_ERR_NUM_MASK) >> Self::KEY1_ERR_NUM_OFFSET
    }

    /// **KEY0_FAIL** `[15]` (RO)
    ///
    /// 0: Means no failure and that the data of key0 is reliable 1: Means that programming key0 failed and the number of error bytes is over 6.
    #[inline(always)]
    pub fn key0_fail(&self) -> bool {
        (self.value.read_bits() & Self::KEY0_FAIL_MASK) != 0
    }

    /// **KEY0_ERR_NUM** `[14:12]` (RO)
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub fn key0_err_num(&self) -> u32 {
        (self.value.read_bits() & Self::KEY0_ERR_NUM_MASK) >> Self::KEY0_ERR_NUM_OFFSET
    }

    /// **USR_DATA_FAIL** `[11]` (RO)
    ///
    /// 0: Means no failure and that the user data is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
    #[inline(always)]
    pub fn usr_data_fail(&self) -> bool {
        (self.value.read_bits() & Self::USR_DATA_FAIL_MASK) != 0
    }

    /// **USR_DATA_ERR_NUM** `[10:8]` (RO)
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub fn usr_data_err_num(&self) -> u32 {
        (self.value.read_bits() & Self::USR_DATA_ERR_NUM_MASK) >> Self::USR_DATA_ERR_NUM_OFFSET
    }

    /// **SYS_PART1_FAIL** `[7]` (RO)
    ///
    /// 0: Means no failure and that the data of system part1 is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
    #[inline(always)]
    pub fn sys_part1_fail(&self) -> bool {
        (self.value.read_bits() & Self::SYS_PART1_FAIL_MASK) != 0
    }

    /// **SYS_PART1_ERR_NUM** `[6:4]` (RO)
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub fn sys_part1_err_num(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_PART1_ERR_NUM_MASK) >> Self::SYS_PART1_ERR_NUM_OFFSET
    }

    /// **MAC_SYS_FAIL** `[3]` (RO)
    ///
    /// 0: Means no failure and that the data of MAC_SPI_8M is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
    #[inline(always)]
    pub fn mac_sys_fail(&self) -> bool {
        (self.value.read_bits() & Self::MAC_SYS_FAIL_MASK) != 0
    }

    /// **MAC_SYS_ERR_NUM** `[2:0]` (RO)
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub fn mac_sys_err_num(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_SYS_ERR_NUM_MASK) >> Self::MAC_SYS_ERR_NUM_OFFSET
    }
}

/// RD_RS_ERR0 value type for batch read/modify/write operations
///
/// Use `rdrserr0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdrserr0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdRsErr0Val {
    bits: u32,
}

impl RdRsErr0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY4_FAIL** `[31:31]`
    ///
    /// 0: Means no failure and that the data of key4 is reliable 1: Means that programming key4 failed and the number of error bytes is over 6.
    #[inline(always)]
    pub const fn key4_fail(&self) -> bool {
        (self.bits & RdRsErr0::KEY4_FAIL_MASK) != 0
    }

    /// Read **KEY4_ERR_NUM** `[30:28]`
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub const fn key4_err_num(&self) -> u32 {
        (self.bits & RdRsErr0::KEY4_ERR_NUM_MASK) >> RdRsErr0::KEY4_ERR_NUM_OFFSET
    }

    /// Read **KEY3_FAIL** `[27:27]`
    ///
    /// 0: Means no failure and that the data of key3 is reliable 1: Means that programming key3 failed and the number of error bytes is over 6.
    #[inline(always)]
    pub const fn key3_fail(&self) -> bool {
        (self.bits & RdRsErr0::KEY3_FAIL_MASK) != 0
    }

    /// Read **KEY3_ERR_NUM** `[26:24]`
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub const fn key3_err_num(&self) -> u32 {
        (self.bits & RdRsErr0::KEY3_ERR_NUM_MASK) >> RdRsErr0::KEY3_ERR_NUM_OFFSET
    }

    /// Read **KEY2_FAIL** `[23:23]`
    ///
    /// 0: Means no failure and that the data of key2 is reliable 1: Means that programming key2 failed and the number of error bytes is over 6.
    #[inline(always)]
    pub const fn key2_fail(&self) -> bool {
        (self.bits & RdRsErr0::KEY2_FAIL_MASK) != 0
    }

    /// Read **KEY2_ERR_NUM** `[22:20]`
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub const fn key2_err_num(&self) -> u32 {
        (self.bits & RdRsErr0::KEY2_ERR_NUM_MASK) >> RdRsErr0::KEY2_ERR_NUM_OFFSET
    }

    /// Read **KEY1_FAIL** `[19:19]`
    ///
    /// 0: Means no failure and that the data of key1 is reliable 1: Means that programming key1 failed and the number of error bytes is over 6.
    #[inline(always)]
    pub const fn key1_fail(&self) -> bool {
        (self.bits & RdRsErr0::KEY1_FAIL_MASK) != 0
    }

    /// Read **KEY1_ERR_NUM** `[18:16]`
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub const fn key1_err_num(&self) -> u32 {
        (self.bits & RdRsErr0::KEY1_ERR_NUM_MASK) >> RdRsErr0::KEY1_ERR_NUM_OFFSET
    }

    /// Read **KEY0_FAIL** `[15:15]`
    ///
    /// 0: Means no failure and that the data of key0 is reliable 1: Means that programming key0 failed and the number of error bytes is over 6.
    #[inline(always)]
    pub const fn key0_fail(&self) -> bool {
        (self.bits & RdRsErr0::KEY0_FAIL_MASK) != 0
    }

    /// Read **KEY0_ERR_NUM** `[14:12]`
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub const fn key0_err_num(&self) -> u32 {
        (self.bits & RdRsErr0::KEY0_ERR_NUM_MASK) >> RdRsErr0::KEY0_ERR_NUM_OFFSET
    }

    /// Read **USR_DATA_FAIL** `[11:11]`
    ///
    /// 0: Means no failure and that the user data is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
    #[inline(always)]
    pub const fn usr_data_fail(&self) -> bool {
        (self.bits & RdRsErr0::USR_DATA_FAIL_MASK) != 0
    }

    /// Read **USR_DATA_ERR_NUM** `[10:8]`
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub const fn usr_data_err_num(&self) -> u32 {
        (self.bits & RdRsErr0::USR_DATA_ERR_NUM_MASK) >> RdRsErr0::USR_DATA_ERR_NUM_OFFSET
    }

    /// Read **SYS_PART1_FAIL** `[7:7]`
    ///
    /// 0: Means no failure and that the data of system part1 is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
    #[inline(always)]
    pub const fn sys_part1_fail(&self) -> bool {
        (self.bits & RdRsErr0::SYS_PART1_FAIL_MASK) != 0
    }

    /// Read **SYS_PART1_ERR_NUM** `[6:4]`
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub const fn sys_part1_err_num(&self) -> u32 {
        (self.bits & RdRsErr0::SYS_PART1_ERR_NUM_MASK) >> RdRsErr0::SYS_PART1_ERR_NUM_OFFSET
    }

    /// Read **MAC_SYS_FAIL** `[3:3]`
    ///
    /// 0: Means no failure and that the data of MAC_SPI_8M is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
    #[inline(always)]
    pub const fn mac_sys_fail(&self) -> bool {
        (self.bits & RdRsErr0::MAC_SYS_FAIL_MASK) != 0
    }

    /// Read **MAC_SYS_ERR_NUM** `[2:0]`
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub const fn mac_sys_err_num(&self) -> u32 {
        (self.bits & RdRsErr0::MAC_SYS_ERR_NUM_MASK) >> RdRsErr0::MAC_SYS_ERR_NUM_OFFSET
    }
}

impl Default for RdRsErr0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdRsErr0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdRsErr0Val {
        RdRsErr0Val::from_bits(self.value.read_bits())
    }
}

/// RD_RS_ERR1 Register
///
/// Programming error record register 1 of BLOCK1-10.
///
/// Address offset: 0x01c4
///
/// ## Fields
///
/// - **SYS_PART2_FAIL** `[7]` (RO)
///
///   0: Means no failure and that the data of system part2 is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
///
/// - **SYS_PART2_ERR_NUM** `[6:4]` (RO)
///
///   The value of this signal means the number of error bytes.
///
/// - **KEY5_FAIL** `[3]` (RO)
///
///   0: Means no failure and that the data of key5 is reliable 1: Means that programming key5 failed and the number of error bytes is over 6.
///
/// - **KEY5_ERR_NUM** `[2:0]` (RO)
///
///   The value of this signal means the number of error bytes.
#[repr(transparent)]
pub struct RdRsErr1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdRsErr1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdRsErr1 {
    // ======== Field Constants ========

    /// **SYS_PART2_FAIL**
    ///
    /// 0: Means no failure and that the data of system part2 is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const SYS_PART2_FAIL_OFFSET: u32 = 7;
    pub const SYS_PART2_FAIL_MASK: u32 = 0x00000080;
    pub const SYS_PART2_FAIL: u32 = 1 << 7;

    /// **SYS_PART2_ERR_NUM**
    ///
    /// The value of this signal means the number of error bytes.
    ///
    /// Bits `[6:4]`, mask `0x00000070`
    pub const SYS_PART2_ERR_NUM_OFFSET: u32 = 4;
    pub const SYS_PART2_ERR_NUM_MASK: u32 = 0x00000070;

    /// **KEY5_FAIL**
    ///
    /// 0: Means no failure and that the data of key5 is reliable 1: Means that programming key5 failed and the number of error bytes is over 6.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const KEY5_FAIL_OFFSET: u32 = 3;
    pub const KEY5_FAIL_MASK: u32 = 0x00000008;
    pub const KEY5_FAIL: u32 = 1 << 3;

    /// **KEY5_ERR_NUM**
    ///
    /// The value of this signal means the number of error bytes.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const KEY5_ERR_NUM_OFFSET: u32 = 0;
    pub const KEY5_ERR_NUM_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **SYS_PART2_FAIL** `[7]` (RO)
    ///
    /// 0: Means no failure and that the data of system part2 is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
    #[inline(always)]
    pub fn sys_part2_fail(&self) -> bool {
        (self.value.read_bits() & Self::SYS_PART2_FAIL_MASK) != 0
    }

    /// **SYS_PART2_ERR_NUM** `[6:4]` (RO)
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub fn sys_part2_err_num(&self) -> u32 {
        (self.value.read_bits() & Self::SYS_PART2_ERR_NUM_MASK) >> Self::SYS_PART2_ERR_NUM_OFFSET
    }

    /// **KEY5_FAIL** `[3]` (RO)
    ///
    /// 0: Means no failure and that the data of key5 is reliable 1: Means that programming key5 failed and the number of error bytes is over 6.
    #[inline(always)]
    pub fn key5_fail(&self) -> bool {
        (self.value.read_bits() & Self::KEY5_FAIL_MASK) != 0
    }

    /// **KEY5_ERR_NUM** `[2:0]` (RO)
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub fn key5_err_num(&self) -> u32 {
        (self.value.read_bits() & Self::KEY5_ERR_NUM_MASK) >> Self::KEY5_ERR_NUM_OFFSET
    }
}

/// RD_RS_ERR1 value type for batch read/modify/write operations
///
/// Use `rdrserr1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdrserr1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdRsErr1Val {
    bits: u32,
}

impl RdRsErr1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SYS_PART2_FAIL** `[7:7]`
    ///
    /// 0: Means no failure and that the data of system part2 is reliable 1: Means that programming user data failed and the number of error bytes is over 6.
    #[inline(always)]
    pub const fn sys_part2_fail(&self) -> bool {
        (self.bits & RdRsErr1::SYS_PART2_FAIL_MASK) != 0
    }

    /// Read **SYS_PART2_ERR_NUM** `[6:4]`
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub const fn sys_part2_err_num(&self) -> u32 {
        (self.bits & RdRsErr1::SYS_PART2_ERR_NUM_MASK) >> RdRsErr1::SYS_PART2_ERR_NUM_OFFSET
    }

    /// Read **KEY5_FAIL** `[3:3]`
    ///
    /// 0: Means no failure and that the data of key5 is reliable 1: Means that programming key5 failed and the number of error bytes is over 6.
    #[inline(always)]
    pub const fn key5_fail(&self) -> bool {
        (self.bits & RdRsErr1::KEY5_FAIL_MASK) != 0
    }

    /// Read **KEY5_ERR_NUM** `[2:0]`
    ///
    /// The value of this signal means the number of error bytes.
    #[inline(always)]
    pub const fn key5_err_num(&self) -> u32 {
        (self.bits & RdRsErr1::KEY5_ERR_NUM_MASK) >> RdRsErr1::KEY5_ERR_NUM_OFFSET
    }
}

impl Default for RdRsErr1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdRsErr1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdRsErr1Val {
        RdRsErr1Val::from_bits(self.value.read_bits())
    }
}

/// CLK Register
///
/// eFuse clcok configuration register.
///
/// Address offset: 0x01c8
///
/// ## Fields
///
/// - **EN** `[16]` (RW)
///
///   Set this bit to force enable eFuse register configuration clock signal.
///
/// - **MEM_FORCE_PU** `[2]` (RW)
///
///   Set this bit to force eFuse SRAM into working mode.
///
/// - **MEM_CLK_FORCE_ON** `[1]` (RW)
///
///   Set this bit and force to activate clock signal of eFuse SRAM.
///
/// - **MEM_FORCE_PD** `[0]` (RW)
///
///   Set this bit to force eFuse SRAM into power-saving mode.
#[repr(transparent)]
pub struct Clk {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Clk {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Clk {
    // ======== Field Constants ========

    /// **EN**
    ///
    /// Set this bit to force enable eFuse register configuration clock signal.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const EN_OFFSET: u32 = 16;
    pub const EN_MASK: u32 = 0x00010000;
    pub const EN: u32 = 1 << 16;

    /// **MEM_FORCE_PU**
    ///
    /// Set this bit to force eFuse SRAM into working mode.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const MEM_FORCE_PU_OFFSET: u32 = 2;
    pub const MEM_FORCE_PU_MASK: u32 = 0x00000004;
    pub const MEM_FORCE_PU: u32 = 1 << 2;

    /// **MEM_CLK_FORCE_ON**
    ///
    /// Set this bit and force to activate clock signal of eFuse SRAM.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MEM_CLK_FORCE_ON_OFFSET: u32 = 1;
    pub const MEM_CLK_FORCE_ON_MASK: u32 = 0x00000002;
    pub const MEM_CLK_FORCE_ON: u32 = 1 << 1;

    /// **MEM_FORCE_PD**
    ///
    /// Set this bit to force eFuse SRAM into power-saving mode.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MEM_FORCE_PD_OFFSET: u32 = 0;
    pub const MEM_FORCE_PD_MASK: u32 = 0x00000001;
    pub const MEM_FORCE_PD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EN** `[16]` (RW)
    ///
    /// Set this bit to force enable eFuse register configuration clock signal.
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

    /// **MEM_FORCE_PU** `[2]` (RW)
    ///
    /// Set this bit to force eFuse SRAM into working mode.
    #[inline(always)]
    pub fn mem_force_pu(&self) -> bool {
        (self.value.read_bits() & Self::MEM_FORCE_PU_MASK) != 0
    }

    /// Set MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_force_pu(&self) {
        self.value.modify_bits(|r| r | Self::MEM_FORCE_PU_MASK);
    }

    /// Clear MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_force_pu(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_FORCE_PU_MASK);
    }

    /// **MEM_CLK_FORCE_ON** `[1]` (RW)
    ///
    /// Set this bit and force to activate clock signal of eFuse SRAM.
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_clk_force_on(&self) {
        self.value.modify_bits(|r| r | Self::MEM_CLK_FORCE_ON_MASK);
    }

    /// Clear MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_clk_force_on(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_CLK_FORCE_ON_MASK);
    }

    /// **MEM_FORCE_PD** `[0]` (RW)
    ///
    /// Set this bit to force eFuse SRAM into power-saving mode.
    #[inline(always)]
    pub fn mem_force_pd(&self) -> bool {
        (self.value.read_bits() & Self::MEM_FORCE_PD_MASK) != 0
    }

    /// Set MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_force_pd(&self) {
        self.value.modify_bits(|r| r | Self::MEM_FORCE_PD_MASK);
    }

    /// Clear MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_force_pd(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_FORCE_PD_MASK);
    }
}

/// CLK value type for batch read/modify/write operations
///
/// Use `clk.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clk.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkVal {
    bits: u32,
}

impl ClkVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **EN** `[16:16]`
    ///
    /// Set this bit to force enable eFuse register configuration clock signal.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Clk::EN_MASK) != 0
    }

    /// Read **MEM_FORCE_PU** `[2:2]`
    ///
    /// Set this bit to force eFuse SRAM into working mode.
    #[inline(always)]
    pub const fn mem_force_pu(&self) -> bool {
        (self.bits & Clk::MEM_FORCE_PU_MASK) != 0
    }

    /// Read **MEM_CLK_FORCE_ON** `[1:1]`
    ///
    /// Set this bit and force to activate clock signal of eFuse SRAM.
    #[inline(always)]
    pub const fn mem_clk_force_on(&self) -> bool {
        (self.bits & Clk::MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **MEM_FORCE_PD** `[0:0]`
    ///
    /// Set this bit to force eFuse SRAM into power-saving mode.
    #[inline(always)]
    pub const fn mem_force_pd(&self) -> bool {
        (self.bits & Clk::MEM_FORCE_PD_MASK) != 0
    }

    /// Set **EN** `[16:16]`
    ///
    /// Set this bit to force enable eFuse register configuration clock signal.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clk::EN_MASK
            } else {
                self.bits & !Clk::EN_MASK
            },
        }
    }

    /// Set **MEM_FORCE_PU** `[2:2]`
    ///
    /// Set this bit to force eFuse SRAM into working mode.
    #[inline(always)]
    pub const fn with_mem_force_pu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clk::MEM_FORCE_PU_MASK
            } else {
                self.bits & !Clk::MEM_FORCE_PU_MASK
            },
        }
    }

    /// Set **MEM_CLK_FORCE_ON** `[1:1]`
    ///
    /// Set this bit and force to activate clock signal of eFuse SRAM.
    #[inline(always)]
    pub const fn with_mem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clk::MEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !Clk::MEM_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **MEM_FORCE_PD** `[0:0]`
    ///
    /// Set this bit to force eFuse SRAM into power-saving mode.
    #[inline(always)]
    pub const fn with_mem_force_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clk::MEM_FORCE_PD_MASK
            } else {
                self.bits & !Clk::MEM_FORCE_PD_MASK
            },
        }
    }
}

impl Default for ClkVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Clk {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkVal {
        ClkVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkVal) -> ClkVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CONF Register
///
/// eFuse operation mode configuraiton register
///
/// Address offset: 0x01cc
///
/// ## Fields
///
/// - **CFG_ECDSA_BLK** `[19:16]` (RW)
///
///   Configures which block to use for ECDSA key output.
///
/// - **OP_CODE** `[15:0]` (RW)
///
///   0x5A5A: programming operation command 0x5AA5: read operation command.
#[repr(transparent)]
pub struct Conf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Conf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Conf {
    // ======== Field Constants ========

    /// **CFG_ECDSA_BLK**
    ///
    /// Configures which block to use for ECDSA key output.
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const CFG_ECDSA_BLK_OFFSET: u32 = 16;
    pub const CFG_ECDSA_BLK_MASK: u32 = 0x000f0000;

    /// **OP_CODE**
    ///
    /// 0x5A5A: programming operation command 0x5AA5: read operation command.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const OP_CODE_OFFSET: u32 = 0;
    pub const OP_CODE_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CFG_ECDSA_BLK** `[19:16]` (RW)
    ///
    /// Configures which block to use for ECDSA key output.
    #[inline(always)]
    pub fn cfg_ecdsa_blk(&self) -> u32 {
        (self.value.read_bits() & Self::CFG_ECDSA_BLK_MASK) >> Self::CFG_ECDSA_BLK_OFFSET
    }

    /// Write **CFG_ECDSA_BLK** `[19:16]` (RW)
    ///
    /// Configures which block to use for ECDSA key output.
    #[inline(always)]
    pub fn write_cfg_ecdsa_blk(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CFG_ECDSA_BLK_MASK)
                | ((value << Self::CFG_ECDSA_BLK_OFFSET) & Self::CFG_ECDSA_BLK_MASK)
        });
    }

    /// **OP_CODE** `[15:0]` (RW)
    ///
    /// 0x5A5A: programming operation command 0x5AA5: read operation command.
    #[inline(always)]
    pub fn op_code(&self) -> u32 {
        (self.value.read_bits() & Self::OP_CODE_MASK) >> Self::OP_CODE_OFFSET
    }

    /// Write **OP_CODE** `[15:0]` (RW)
    ///
    /// 0x5A5A: programming operation command 0x5AA5: read operation command.
    #[inline(always)]
    pub fn write_op_code(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OP_CODE_MASK) | ((value << Self::OP_CODE_OFFSET) & Self::OP_CODE_MASK)
        });
    }
}

/// CONF value type for batch read/modify/write operations
///
/// Use `conf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `conf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ConfVal {
    bits: u32,
}

impl ConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CFG_ECDSA_BLK** `[19:16]`
    ///
    /// Configures which block to use for ECDSA key output.
    #[inline(always)]
    pub const fn cfg_ecdsa_blk(&self) -> u32 {
        (self.bits & Conf::CFG_ECDSA_BLK_MASK) >> Conf::CFG_ECDSA_BLK_OFFSET
    }

    /// Read **OP_CODE** `[15:0]`
    ///
    /// 0x5A5A: programming operation command 0x5AA5: read operation command.
    #[inline(always)]
    pub const fn op_code(&self) -> u32 {
        (self.bits & Conf::OP_CODE_MASK) >> Conf::OP_CODE_OFFSET
    }

    /// Set **CFG_ECDSA_BLK** `[19:16]`
    ///
    /// Configures which block to use for ECDSA key output.
    #[inline(always)]
    pub const fn with_cfg_ecdsa_blk(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Conf::CFG_ECDSA_BLK_MASK)
                | ((value << Conf::CFG_ECDSA_BLK_OFFSET) & Conf::CFG_ECDSA_BLK_MASK),
        }
    }

    /// Set **OP_CODE** `[15:0]`
    ///
    /// 0x5A5A: programming operation command 0x5AA5: read operation command.
    #[inline(always)]
    pub const fn with_op_code(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Conf::OP_CODE_MASK)
                | ((value << Conf::OP_CODE_OFFSET) & Conf::OP_CODE_MASK),
        }
    }
}

impl Default for ConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Conf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ConfVal {
        ConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ConfVal) -> ConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// STATUS Register
///
/// eFuse status register.
///
/// Address offset: 0x01d0
///
/// ## Fields
///
/// - **CUR_ECDSA_BLK** `[23:20]` (RO)
///
///   Indicates which block is used for ECDSA key output.
///
/// - **BLK0_VALID_BIT_CNT** `[19:10]` (RO)
///
///   Indicates the number of block valid bit.
///
/// - **OTP_VDDQ_IS_SW** `[9]` (RO)
///
///   The value of OTP_VDDQ_IS_SW.
///
/// - **OTP_PGENB_SW** `[8]` (RO)
///
///   The value of OTP_PGENB_SW.
///
/// - **OTP_CSB_SW** `[7]` (RO)
///
///   The value of OTP_CSB_SW.
///
/// - **OTP_STROBE_SW** `[6]` (RO)
///
///   The value of OTP_STROBE_SW.
///
/// - **OTP_VDDQ_C_SYNC2** `[5]` (RO)
///
///   The value of OTP_VDDQ_C_SYNC2.
///
/// - **OTP_LOAD_SW** `[4]` (RO)
///
///   The value of OTP_LOAD_SW.
///
/// - **STATE** `[3:0]` (RO)
///
///   Indicates the state of the eFuse state machine.
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

    /// **CUR_ECDSA_BLK**
    ///
    /// Indicates which block is used for ECDSA key output.
    ///
    /// Bits `[23:20]`, mask `0x00f00000`
    pub const CUR_ECDSA_BLK_OFFSET: u32 = 20;
    pub const CUR_ECDSA_BLK_MASK: u32 = 0x00f00000;

    /// **BLK0_VALID_BIT_CNT**
    ///
    /// Indicates the number of block valid bit.
    ///
    /// Bits `[19:10]`, mask `0x000ffc00`
    pub const BLK0_VALID_BIT_CNT_OFFSET: u32 = 10;
    pub const BLK0_VALID_BIT_CNT_MASK: u32 = 0x000ffc00;

    /// **OTP_VDDQ_IS_SW**
    ///
    /// The value of OTP_VDDQ_IS_SW.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const OTP_VDDQ_IS_SW_OFFSET: u32 = 9;
    pub const OTP_VDDQ_IS_SW_MASK: u32 = 0x00000200;
    pub const OTP_VDDQ_IS_SW: u32 = 1 << 9;

    /// **OTP_PGENB_SW**
    ///
    /// The value of OTP_PGENB_SW.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const OTP_PGENB_SW_OFFSET: u32 = 8;
    pub const OTP_PGENB_SW_MASK: u32 = 0x00000100;
    pub const OTP_PGENB_SW: u32 = 1 << 8;

    /// **OTP_CSB_SW**
    ///
    /// The value of OTP_CSB_SW.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const OTP_CSB_SW_OFFSET: u32 = 7;
    pub const OTP_CSB_SW_MASK: u32 = 0x00000080;
    pub const OTP_CSB_SW: u32 = 1 << 7;

    /// **OTP_STROBE_SW**
    ///
    /// The value of OTP_STROBE_SW.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const OTP_STROBE_SW_OFFSET: u32 = 6;
    pub const OTP_STROBE_SW_MASK: u32 = 0x00000040;
    pub const OTP_STROBE_SW: u32 = 1 << 6;

    /// **OTP_VDDQ_C_SYNC2**
    ///
    /// The value of OTP_VDDQ_C_SYNC2.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const OTP_VDDQ_C_SYNC2_OFFSET: u32 = 5;
    pub const OTP_VDDQ_C_SYNC2_MASK: u32 = 0x00000020;
    pub const OTP_VDDQ_C_SYNC2: u32 = 1 << 5;

    /// **OTP_LOAD_SW**
    ///
    /// The value of OTP_LOAD_SW.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const OTP_LOAD_SW_OFFSET: u32 = 4;
    pub const OTP_LOAD_SW_MASK: u32 = 0x00000010;
    pub const OTP_LOAD_SW: u32 = 1 << 4;

    /// **STATE**
    ///
    /// Indicates the state of the eFuse state machine.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const STATE_OFFSET: u32 = 0;
    pub const STATE_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **CUR_ECDSA_BLK** `[23:20]` (RO)
    ///
    /// Indicates which block is used for ECDSA key output.
    #[inline(always)]
    pub fn cur_ecdsa_blk(&self) -> u32 {
        (self.value.read_bits() & Self::CUR_ECDSA_BLK_MASK) >> Self::CUR_ECDSA_BLK_OFFSET
    }

    /// **BLK0_VALID_BIT_CNT** `[19:10]` (RO)
    ///
    /// Indicates the number of block valid bit.
    #[inline(always)]
    pub fn blk0_valid_bit_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::BLK0_VALID_BIT_CNT_MASK) >> Self::BLK0_VALID_BIT_CNT_OFFSET
    }

    /// **OTP_VDDQ_IS_SW** `[9]` (RO)
    ///
    /// The value of OTP_VDDQ_IS_SW.
    #[inline(always)]
    pub fn otp_vddq_is_sw(&self) -> bool {
        (self.value.read_bits() & Self::OTP_VDDQ_IS_SW_MASK) != 0
    }

    /// **OTP_PGENB_SW** `[8]` (RO)
    ///
    /// The value of OTP_PGENB_SW.
    #[inline(always)]
    pub fn otp_pgenb_sw(&self) -> bool {
        (self.value.read_bits() & Self::OTP_PGENB_SW_MASK) != 0
    }

    /// **OTP_CSB_SW** `[7]` (RO)
    ///
    /// The value of OTP_CSB_SW.
    #[inline(always)]
    pub fn otp_csb_sw(&self) -> bool {
        (self.value.read_bits() & Self::OTP_CSB_SW_MASK) != 0
    }

    /// **OTP_STROBE_SW** `[6]` (RO)
    ///
    /// The value of OTP_STROBE_SW.
    #[inline(always)]
    pub fn otp_strobe_sw(&self) -> bool {
        (self.value.read_bits() & Self::OTP_STROBE_SW_MASK) != 0
    }

    /// **OTP_VDDQ_C_SYNC2** `[5]` (RO)
    ///
    /// The value of OTP_VDDQ_C_SYNC2.
    #[inline(always)]
    pub fn otp_vddq_c_sync2(&self) -> bool {
        (self.value.read_bits() & Self::OTP_VDDQ_C_SYNC2_MASK) != 0
    }

    /// **OTP_LOAD_SW** `[4]` (RO)
    ///
    /// The value of OTP_LOAD_SW.
    #[inline(always)]
    pub fn otp_load_sw(&self) -> bool {
        (self.value.read_bits() & Self::OTP_LOAD_SW_MASK) != 0
    }

    /// **STATE** `[3:0]` (RO)
    ///
    /// Indicates the state of the eFuse state machine.
    #[inline(always)]
    pub fn state(&self) -> u32 {
        (self.value.read_bits() & Self::STATE_MASK) >> Self::STATE_OFFSET
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

    /// Read **CUR_ECDSA_BLK** `[23:20]`
    ///
    /// Indicates which block is used for ECDSA key output.
    #[inline(always)]
    pub const fn cur_ecdsa_blk(&self) -> u32 {
        (self.bits & Status::CUR_ECDSA_BLK_MASK) >> Status::CUR_ECDSA_BLK_OFFSET
    }

    /// Read **BLK0_VALID_BIT_CNT** `[19:10]`
    ///
    /// Indicates the number of block valid bit.
    #[inline(always)]
    pub const fn blk0_valid_bit_cnt(&self) -> u32 {
        (self.bits & Status::BLK0_VALID_BIT_CNT_MASK) >> Status::BLK0_VALID_BIT_CNT_OFFSET
    }

    /// Read **OTP_VDDQ_IS_SW** `[9:9]`
    ///
    /// The value of OTP_VDDQ_IS_SW.
    #[inline(always)]
    pub const fn otp_vddq_is_sw(&self) -> bool {
        (self.bits & Status::OTP_VDDQ_IS_SW_MASK) != 0
    }

    /// Read **OTP_PGENB_SW** `[8:8]`
    ///
    /// The value of OTP_PGENB_SW.
    #[inline(always)]
    pub const fn otp_pgenb_sw(&self) -> bool {
        (self.bits & Status::OTP_PGENB_SW_MASK) != 0
    }

    /// Read **OTP_CSB_SW** `[7:7]`
    ///
    /// The value of OTP_CSB_SW.
    #[inline(always)]
    pub const fn otp_csb_sw(&self) -> bool {
        (self.bits & Status::OTP_CSB_SW_MASK) != 0
    }

    /// Read **OTP_STROBE_SW** `[6:6]`
    ///
    /// The value of OTP_STROBE_SW.
    #[inline(always)]
    pub const fn otp_strobe_sw(&self) -> bool {
        (self.bits & Status::OTP_STROBE_SW_MASK) != 0
    }

    /// Read **OTP_VDDQ_C_SYNC2** `[5:5]`
    ///
    /// The value of OTP_VDDQ_C_SYNC2.
    #[inline(always)]
    pub const fn otp_vddq_c_sync2(&self) -> bool {
        (self.bits & Status::OTP_VDDQ_C_SYNC2_MASK) != 0
    }

    /// Read **OTP_LOAD_SW** `[4:4]`
    ///
    /// The value of OTP_LOAD_SW.
    #[inline(always)]
    pub const fn otp_load_sw(&self) -> bool {
        (self.bits & Status::OTP_LOAD_SW_MASK) != 0
    }

    /// Read **STATE** `[3:0]`
    ///
    /// Indicates the state of the eFuse state machine.
    #[inline(always)]
    pub const fn state(&self) -> u32 {
        (self.bits & Status::STATE_MASK) >> Status::STATE_OFFSET
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

/// CMD Register
///
/// eFuse command register.
///
/// Address offset: 0x01d4
///
/// ## Fields
///
/// - **BLK_NUM** `[5:2]` (RW)
///
///   The serial number of the block to be programmed. Value 0-10 corresponds to block number 0-10, respectively.
///
/// - **PGM_CMD** `[1]` (RW)
///
///   Set this bit to send programming command.
///
/// - **READ_CMD** `[0]` (RW)
///
///   Set this bit to send read command.
#[repr(transparent)]
pub struct Cmd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Cmd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Cmd {
    // ======== Field Constants ========

    /// **BLK_NUM**
    ///
    /// The serial number of the block to be programmed. Value 0-10 corresponds to block number 0-10, respectively.
    ///
    /// Bits `[5:2]`, mask `0x0000003c`
    pub const BLK_NUM_OFFSET: u32 = 2;
    pub const BLK_NUM_MASK: u32 = 0x0000003c;

    /// **PGM_CMD**
    ///
    /// Set this bit to send programming command.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PGM_CMD_OFFSET: u32 = 1;
    pub const PGM_CMD_MASK: u32 = 0x00000002;
    pub const PGM_CMD: u32 = 1 << 1;

    /// **READ_CMD**
    ///
    /// Set this bit to send read command.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const READ_CMD_OFFSET: u32 = 0;
    pub const READ_CMD_MASK: u32 = 0x00000001;
    pub const READ_CMD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BLK_NUM** `[5:2]` (RW)
    ///
    /// The serial number of the block to be programmed. Value 0-10 corresponds to block number 0-10, respectively.
    #[inline(always)]
    pub fn blk_num(&self) -> u32 {
        (self.value.read_bits() & Self::BLK_NUM_MASK) >> Self::BLK_NUM_OFFSET
    }

    /// Write **BLK_NUM** `[5:2]` (RW)
    ///
    /// The serial number of the block to be programmed. Value 0-10 corresponds to block number 0-10, respectively.
    #[inline(always)]
    pub fn write_blk_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLK_NUM_MASK) | ((value << Self::BLK_NUM_OFFSET) & Self::BLK_NUM_MASK)
        });
    }

    /// **PGM_CMD** `[1]` (RW)
    ///
    /// Set this bit to send programming command.
    #[inline(always)]
    pub fn pgm_cmd(&self) -> bool {
        (self.value.read_bits() & Self::PGM_CMD_MASK) != 0
    }

    /// Set PGM_CMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pgm_cmd(&self) {
        self.value.modify_bits(|r| r | Self::PGM_CMD_MASK);
    }

    /// Clear PGM_CMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pgm_cmd(&self) {
        self.value.modify_bits(|r| r & !Self::PGM_CMD_MASK);
    }

    /// **READ_CMD** `[0]` (RW)
    ///
    /// Set this bit to send read command.
    #[inline(always)]
    pub fn read_cmd(&self) -> bool {
        (self.value.read_bits() & Self::READ_CMD_MASK) != 0
    }

    /// Set READ_CMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_read_cmd(&self) {
        self.value.modify_bits(|r| r | Self::READ_CMD_MASK);
    }

    /// Clear READ_CMD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_read_cmd(&self) {
        self.value.modify_bits(|r| r & !Self::READ_CMD_MASK);
    }
}

/// CMD value type for batch read/modify/write operations
///
/// Use `cmd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cmd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CmdVal {
    bits: u32,
}

impl CmdVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BLK_NUM** `[5:2]`
    ///
    /// The serial number of the block to be programmed. Value 0-10 corresponds to block number 0-10, respectively.
    #[inline(always)]
    pub const fn blk_num(&self) -> u32 {
        (self.bits & Cmd::BLK_NUM_MASK) >> Cmd::BLK_NUM_OFFSET
    }

    /// Read **PGM_CMD** `[1:1]`
    ///
    /// Set this bit to send programming command.
    #[inline(always)]
    pub const fn pgm_cmd(&self) -> bool {
        (self.bits & Cmd::PGM_CMD_MASK) != 0
    }

    /// Read **READ_CMD** `[0:0]`
    ///
    /// Set this bit to send read command.
    #[inline(always)]
    pub const fn read_cmd(&self) -> bool {
        (self.bits & Cmd::READ_CMD_MASK) != 0
    }

    /// Set **BLK_NUM** `[5:2]`
    ///
    /// The serial number of the block to be programmed. Value 0-10 corresponds to block number 0-10, respectively.
    #[inline(always)]
    pub const fn with_blk_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Cmd::BLK_NUM_MASK)
                | ((value << Cmd::BLK_NUM_OFFSET) & Cmd::BLK_NUM_MASK),
        }
    }

    /// Set **PGM_CMD** `[1:1]`
    ///
    /// Set this bit to send programming command.
    #[inline(always)]
    pub const fn with_pgm_cmd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::PGM_CMD_MASK
            } else {
                self.bits & !Cmd::PGM_CMD_MASK
            },
        }
    }

    /// Set **READ_CMD** `[0:0]`
    ///
    /// Set this bit to send read command.
    #[inline(always)]
    pub const fn with_read_cmd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Cmd::READ_CMD_MASK
            } else {
                self.bits & !Cmd::READ_CMD_MASK
            },
        }
    }
}

impl Default for CmdVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Cmd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CmdVal {
        CmdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CmdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CmdVal) -> CmdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_RAW Register
///
/// eFuse raw interrupt register.
///
/// Address offset: 0x01d8
///
/// ## Fields
///
/// - **PGM_DONE_INT_RAW** `[1]` (RO)
///
///   The raw bit signal for pgm_done interrupt.
///
/// - **READ_DONE_INT_RAW** `[0]` (RO)
///
///   The raw bit signal for read_done interrupt.
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

    /// **PGM_DONE_INT_RAW**
    ///
    /// The raw bit signal for pgm_done interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PGM_DONE_INT_RAW_OFFSET: u32 = 1;
    pub const PGM_DONE_INT_RAW_MASK: u32 = 0x00000002;
    pub const PGM_DONE_INT_RAW: u32 = 1 << 1;

    /// **READ_DONE_INT_RAW**
    ///
    /// The raw bit signal for read_done interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const READ_DONE_INT_RAW_OFFSET: u32 = 0;
    pub const READ_DONE_INT_RAW_MASK: u32 = 0x00000001;
    pub const READ_DONE_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PGM_DONE_INT_RAW** `[1]` (RO)
    ///
    /// The raw bit signal for pgm_done interrupt.
    #[inline(always)]
    pub fn pgm_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::PGM_DONE_INT_RAW_MASK) != 0
    }

    /// **READ_DONE_INT_RAW** `[0]` (RO)
    ///
    /// The raw bit signal for read_done interrupt.
    #[inline(always)]
    pub fn read_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::READ_DONE_INT_RAW_MASK) != 0
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

    /// Read **PGM_DONE_INT_RAW** `[1:1]`
    ///
    /// The raw bit signal for pgm_done interrupt.
    #[inline(always)]
    pub const fn pgm_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::PGM_DONE_INT_RAW_MASK) != 0
    }

    /// Read **READ_DONE_INT_RAW** `[0:0]`
    ///
    /// The raw bit signal for read_done interrupt.
    #[inline(always)]
    pub const fn read_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::READ_DONE_INT_RAW_MASK) != 0
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
}

/// INT_ST Register
///
/// eFuse interrupt status register.
///
/// Address offset: 0x01dc
///
/// ## Fields
///
/// - **PGM_DONE_INT_ST** `[1]` (RO)
///
///   The status signal for pgm_done interrupt.
///
/// - **READ_DONE_INT_ST** `[0]` (RO)
///
///   The status signal for read_done interrupt.
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

    /// **PGM_DONE_INT_ST**
    ///
    /// The status signal for pgm_done interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PGM_DONE_INT_ST_OFFSET: u32 = 1;
    pub const PGM_DONE_INT_ST_MASK: u32 = 0x00000002;
    pub const PGM_DONE_INT_ST: u32 = 1 << 1;

    /// **READ_DONE_INT_ST**
    ///
    /// The status signal for read_done interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const READ_DONE_INT_ST_OFFSET: u32 = 0;
    pub const READ_DONE_INT_ST_MASK: u32 = 0x00000001;
    pub const READ_DONE_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PGM_DONE_INT_ST** `[1]` (RO)
    ///
    /// The status signal for pgm_done interrupt.
    #[inline(always)]
    pub fn pgm_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::PGM_DONE_INT_ST_MASK) != 0
    }

    /// **READ_DONE_INT_ST** `[0]` (RO)
    ///
    /// The status signal for read_done interrupt.
    #[inline(always)]
    pub fn read_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::READ_DONE_INT_ST_MASK) != 0
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

    /// Read **PGM_DONE_INT_ST** `[1:1]`
    ///
    /// The status signal for pgm_done interrupt.
    #[inline(always)]
    pub const fn pgm_done_int_st(&self) -> bool {
        (self.bits & IntSt::PGM_DONE_INT_ST_MASK) != 0
    }

    /// Read **READ_DONE_INT_ST** `[0:0]`
    ///
    /// The status signal for read_done interrupt.
    #[inline(always)]
    pub const fn read_done_int_st(&self) -> bool {
        (self.bits & IntSt::READ_DONE_INT_ST_MASK) != 0
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

/// INT_ENA Register
///
/// eFuse interrupt enable register.
///
/// Address offset: 0x01e0
///
/// ## Fields
///
/// - **PGM_DONE_INT_ENA** `[1]` (RW)
///
///   The enable signal for pgm_done interrupt.
///
/// - **READ_DONE_INT_ENA** `[0]` (RW)
///
///   The enable signal for read_done interrupt.
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

    /// **PGM_DONE_INT_ENA**
    ///
    /// The enable signal for pgm_done interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PGM_DONE_INT_ENA_OFFSET: u32 = 1;
    pub const PGM_DONE_INT_ENA_MASK: u32 = 0x00000002;
    pub const PGM_DONE_INT_ENA: u32 = 1 << 1;

    /// **READ_DONE_INT_ENA**
    ///
    /// The enable signal for read_done interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const READ_DONE_INT_ENA_OFFSET: u32 = 0;
    pub const READ_DONE_INT_ENA_MASK: u32 = 0x00000001;
    pub const READ_DONE_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PGM_DONE_INT_ENA** `[1]` (RW)
    ///
    /// The enable signal for pgm_done interrupt.
    #[inline(always)]
    pub fn pgm_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::PGM_DONE_INT_ENA_MASK) != 0
    }

    /// Set PGM_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pgm_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::PGM_DONE_INT_ENA_MASK);
    }

    /// Clear PGM_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pgm_done_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::PGM_DONE_INT_ENA_MASK);
    }

    /// **READ_DONE_INT_ENA** `[0]` (RW)
    ///
    /// The enable signal for read_done interrupt.
    #[inline(always)]
    pub fn read_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::READ_DONE_INT_ENA_MASK) != 0
    }

    /// Set READ_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_read_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::READ_DONE_INT_ENA_MASK);
    }

    /// Clear READ_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_read_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::READ_DONE_INT_ENA_MASK);
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

    /// Read **PGM_DONE_INT_ENA** `[1:1]`
    ///
    /// The enable signal for pgm_done interrupt.
    #[inline(always)]
    pub const fn pgm_done_int_ena(&self) -> bool {
        (self.bits & IntEna::PGM_DONE_INT_ENA_MASK) != 0
    }

    /// Read **READ_DONE_INT_ENA** `[0:0]`
    ///
    /// The enable signal for read_done interrupt.
    #[inline(always)]
    pub const fn read_done_int_ena(&self) -> bool {
        (self.bits & IntEna::READ_DONE_INT_ENA_MASK) != 0
    }

    /// Set **PGM_DONE_INT_ENA** `[1:1]`
    ///
    /// The enable signal for pgm_done interrupt.
    #[inline(always)]
    pub const fn with_pgm_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::PGM_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::PGM_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **READ_DONE_INT_ENA** `[0:0]`
    ///
    /// The enable signal for read_done interrupt.
    #[inline(always)]
    pub const fn with_read_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::READ_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::READ_DONE_INT_ENA_MASK
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

/// INT_CLR Register
///
/// eFuse interrupt clear register.
///
/// Address offset: 0x01e4
///
/// ## Fields
///
/// - **PGM_DONE_INT_CLR** `[1]` (WO)
///
///   The clear signal for pgm_done interrupt.
///
/// - **READ_DONE_INT_CLR** `[0]` (WO)
///
///   The clear signal for read_done interrupt.
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

    /// **PGM_DONE_INT_CLR**
    ///
    /// The clear signal for pgm_done interrupt.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PGM_DONE_INT_CLR_OFFSET: u32 = 1;
    pub const PGM_DONE_INT_CLR_MASK: u32 = 0x00000002;
    pub const PGM_DONE_INT_CLR: u32 = 1 << 1;

    /// **READ_DONE_INT_CLR**
    ///
    /// The clear signal for read_done interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const READ_DONE_INT_CLR_OFFSET: u32 = 0;
    pub const READ_DONE_INT_CLR_MASK: u32 = 0x00000001;
    pub const READ_DONE_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **PGM_DONE_INT_CLR** (WO)
    ///
    /// The clear signal for pgm_done interrupt.
    #[inline(always)]
    pub fn trigger_pgm_done_int_clr(&self) {
        self.value.write_bits(Self::PGM_DONE_INT_CLR_MASK);
    }

    /// Trigger **READ_DONE_INT_CLR** (WO)
    ///
    /// The clear signal for read_done interrupt.
    #[inline(always)]
    pub fn trigger_read_done_int_clr(&self) {
        self.value.write_bits(Self::READ_DONE_INT_CLR_MASK);
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

    /// Set **PGM_DONE_INT_CLR** `[1:1]`
    ///
    /// The clear signal for pgm_done interrupt.
    #[inline(always)]
    pub const fn with_pgm_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::PGM_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::PGM_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **READ_DONE_INT_CLR** `[0:0]`
    ///
    /// The clear signal for read_done interrupt.
    #[inline(always)]
    pub const fn with_read_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::READ_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::READ_DONE_INT_CLR_MASK
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

/// DAC_CONF Register
///
/// Controls the eFuse programming voltage.
///
/// Address offset: 0x01e8
/// Reset value: 0x0001fe17
///
/// ## Fields
///
/// - **OE_CLR** `[17]` (RW)
///
///   Reduces the power supply of the programming voltage.
///
/// - **DAC_NUM** `[16:9]` (RW)
///
///   Controls the rising period of the programming voltage.
///
/// - **DAC_CLK_PAD_SEL** `[8]` (RW)
///
///   Don't care.
///
/// - **DAC_CLK_DIV** `[7:0]` (RW)
///
///   Controls the division factor of the rising clock of the programming voltage.
#[repr(transparent)]
pub struct DacConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DacConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DacConf {
    // ======== Field Constants ========

    /// **OE_CLR**
    ///
    /// Reduces the power supply of the programming voltage.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const OE_CLR_OFFSET: u32 = 17;
    pub const OE_CLR_MASK: u32 = 0x00020000;
    pub const OE_CLR: u32 = 1 << 17;

    /// **DAC_NUM**
    ///
    /// Controls the rising period of the programming voltage.
    ///
    /// Bits `[16:9]`, mask `0x0001fe00`
    pub const DAC_NUM_OFFSET: u32 = 9;
    pub const DAC_NUM_MASK: u32 = 0x0001fe00;

    /// **DAC_CLK_PAD_SEL**
    ///
    /// Don't care.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const DAC_CLK_PAD_SEL_OFFSET: u32 = 8;
    pub const DAC_CLK_PAD_SEL_MASK: u32 = 0x00000100;
    pub const DAC_CLK_PAD_SEL: u32 = 1 << 8;

    /// **DAC_CLK_DIV**
    ///
    /// Controls the division factor of the rising clock of the programming voltage.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const DAC_CLK_DIV_OFFSET: u32 = 0;
    pub const DAC_CLK_DIV_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **OE_CLR** `[17]` (RW)
    ///
    /// Reduces the power supply of the programming voltage.
    #[inline(always)]
    pub fn oe_clr(&self) -> bool {
        (self.value.read_bits() & Self::OE_CLR_MASK) != 0
    }

    /// Set OE_CLR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_oe_clr(&self) {
        self.value.modify_bits(|r| r | Self::OE_CLR_MASK);
    }

    /// Clear OE_CLR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_oe_clr(&self) {
        self.value.modify_bits(|r| r & !Self::OE_CLR_MASK);
    }

    /// **DAC_NUM** `[16:9]` (RW)
    ///
    /// Controls the rising period of the programming voltage.
    #[inline(always)]
    pub fn dac_num(&self) -> u32 {
        (self.value.read_bits() & Self::DAC_NUM_MASK) >> Self::DAC_NUM_OFFSET
    }

    /// Write **DAC_NUM** `[16:9]` (RW)
    ///
    /// Controls the rising period of the programming voltage.
    #[inline(always)]
    pub fn write_dac_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DAC_NUM_MASK) | ((value << Self::DAC_NUM_OFFSET) & Self::DAC_NUM_MASK)
        });
    }

    /// **DAC_CLK_PAD_SEL** `[8]` (RW)
    ///
    /// Don't care.
    #[inline(always)]
    pub fn dac_clk_pad_sel(&self) -> bool {
        (self.value.read_bits() & Self::DAC_CLK_PAD_SEL_MASK) != 0
    }

    /// Set DAC_CLK_PAD_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dac_clk_pad_sel(&self) {
        self.value.modify_bits(|r| r | Self::DAC_CLK_PAD_SEL_MASK);
    }

    /// Clear DAC_CLK_PAD_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dac_clk_pad_sel(&self) {
        self.value.modify_bits(|r| r & !Self::DAC_CLK_PAD_SEL_MASK);
    }

    /// **DAC_CLK_DIV** `[7:0]` (RW)
    ///
    /// Controls the division factor of the rising clock of the programming voltage.
    #[inline(always)]
    pub fn dac_clk_div(&self) -> u32 {
        (self.value.read_bits() & Self::DAC_CLK_DIV_MASK) >> Self::DAC_CLK_DIV_OFFSET
    }

    /// Write **DAC_CLK_DIV** `[7:0]` (RW)
    ///
    /// Controls the division factor of the rising clock of the programming voltage.
    #[inline(always)]
    pub fn write_dac_clk_div(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DAC_CLK_DIV_MASK)
                | ((value << Self::DAC_CLK_DIV_OFFSET) & Self::DAC_CLK_DIV_MASK)
        });
    }
}

/// DAC_CONF value type for batch read/modify/write operations
///
/// Use `dacconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dacconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DacConfVal {
    bits: u32,
}

impl DacConfVal {
    /// Create from raw bits
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
        Self { bits: 0x0001fe17 }
    }

    /// Read **OE_CLR** `[17:17]`
    ///
    /// Reduces the power supply of the programming voltage.
    #[inline(always)]
    pub const fn oe_clr(&self) -> bool {
        (self.bits & DacConf::OE_CLR_MASK) != 0
    }

    /// Read **DAC_NUM** `[16:9]`
    ///
    /// Controls the rising period of the programming voltage.
    #[inline(always)]
    pub const fn dac_num(&self) -> u32 {
        (self.bits & DacConf::DAC_NUM_MASK) >> DacConf::DAC_NUM_OFFSET
    }

    /// Read **DAC_CLK_PAD_SEL** `[8:8]`
    ///
    /// Don't care.
    #[inline(always)]
    pub const fn dac_clk_pad_sel(&self) -> bool {
        (self.bits & DacConf::DAC_CLK_PAD_SEL_MASK) != 0
    }

    /// Read **DAC_CLK_DIV** `[7:0]`
    ///
    /// Controls the division factor of the rising clock of the programming voltage.
    #[inline(always)]
    pub const fn dac_clk_div(&self) -> u32 {
        (self.bits & DacConf::DAC_CLK_DIV_MASK) >> DacConf::DAC_CLK_DIV_OFFSET
    }

    /// Set **OE_CLR** `[17:17]`
    ///
    /// Reduces the power supply of the programming voltage.
    #[inline(always)]
    pub const fn with_oe_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DacConf::OE_CLR_MASK
            } else {
                self.bits & !DacConf::OE_CLR_MASK
            },
        }
    }

    /// Set **DAC_NUM** `[16:9]`
    ///
    /// Controls the rising period of the programming voltage.
    #[inline(always)]
    pub const fn with_dac_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DacConf::DAC_NUM_MASK)
                | ((value << DacConf::DAC_NUM_OFFSET) & DacConf::DAC_NUM_MASK),
        }
    }

    /// Set **DAC_CLK_PAD_SEL** `[8:8]`
    ///
    /// Don't care.
    #[inline(always)]
    pub const fn with_dac_clk_pad_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DacConf::DAC_CLK_PAD_SEL_MASK
            } else {
                self.bits & !DacConf::DAC_CLK_PAD_SEL_MASK
            },
        }
    }

    /// Set **DAC_CLK_DIV** `[7:0]`
    ///
    /// Controls the division factor of the rising clock of the programming voltage.
    #[inline(always)]
    pub const fn with_dac_clk_div(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DacConf::DAC_CLK_DIV_MASK)
                | ((value << DacConf::DAC_CLK_DIV_OFFSET) & DacConf::DAC_CLK_DIV_MASK),
        }
    }
}

impl Default for DacConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DacConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DacConfVal {
        DacConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DacConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DacConfVal) -> DacConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RD_TIM_CONF Register
///
/// Configures read timing parameters.
///
/// Address offset: 0x01ec
/// Reset value: 0x0f010201
///
/// ## Fields
///
/// - **READ_INIT_NUM** `[31:24]` (RW)
///
///   Configures the waiting time of reading eFuse memory.
///
/// - **TSUR_A** `[23:16]` (RW)
///
///   Configures the read setup time.
///
/// - **TRD** `[15:8]` (RW)
///
///   Configures the read time.
///
/// - **THR_A** `[7:0]` (RW)
///
///   Configures the read hold time.
#[repr(transparent)]
pub struct RdTimConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdTimConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdTimConf {
    // ======== Field Constants ========

    /// **READ_INIT_NUM**
    ///
    /// Configures the waiting time of reading eFuse memory.
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const READ_INIT_NUM_OFFSET: u32 = 24;
    pub const READ_INIT_NUM_MASK: u32 = 0xff000000;

    /// **TSUR_A**
    ///
    /// Configures the read setup time.
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const TSUR_A_OFFSET: u32 = 16;
    pub const TSUR_A_MASK: u32 = 0x00ff0000;

    /// **TRD**
    ///
    /// Configures the read time.
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const TRD_OFFSET: u32 = 8;
    pub const TRD_MASK: u32 = 0x0000ff00;

    /// **THR_A**
    ///
    /// Configures the read hold time.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const THR_A_OFFSET: u32 = 0;
    pub const THR_A_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **READ_INIT_NUM** `[31:24]` (RW)
    ///
    /// Configures the waiting time of reading eFuse memory.
    #[inline(always)]
    pub fn read_init_num(&self) -> u32 {
        (self.value.read_bits() & Self::READ_INIT_NUM_MASK) >> Self::READ_INIT_NUM_OFFSET
    }

    /// Write **READ_INIT_NUM** `[31:24]` (RW)
    ///
    /// Configures the waiting time of reading eFuse memory.
    #[inline(always)]
    pub fn write_read_init_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::READ_INIT_NUM_MASK)
                | ((value << Self::READ_INIT_NUM_OFFSET) & Self::READ_INIT_NUM_MASK)
        });
    }

    /// **TSUR_A** `[23:16]` (RW)
    ///
    /// Configures the read setup time.
    #[inline(always)]
    pub fn tsur_a(&self) -> u32 {
        (self.value.read_bits() & Self::TSUR_A_MASK) >> Self::TSUR_A_OFFSET
    }

    /// Write **TSUR_A** `[23:16]` (RW)
    ///
    /// Configures the read setup time.
    #[inline(always)]
    pub fn write_tsur_a(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TSUR_A_MASK) | ((value << Self::TSUR_A_OFFSET) & Self::TSUR_A_MASK)
        });
    }

    /// **TRD** `[15:8]` (RW)
    ///
    /// Configures the read time.
    #[inline(always)]
    pub fn trd(&self) -> u32 {
        (self.value.read_bits() & Self::TRD_MASK) >> Self::TRD_OFFSET
    }

    /// Write **TRD** `[15:8]` (RW)
    ///
    /// Configures the read time.
    #[inline(always)]
    pub fn write_trd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TRD_MASK) | ((value << Self::TRD_OFFSET) & Self::TRD_MASK)
        });
    }

    /// **THR_A** `[7:0]` (RW)
    ///
    /// Configures the read hold time.
    #[inline(always)]
    pub fn thr_a(&self) -> u32 {
        (self.value.read_bits() & Self::THR_A_MASK) >> Self::THR_A_OFFSET
    }

    /// Write **THR_A** `[7:0]` (RW)
    ///
    /// Configures the read hold time.
    #[inline(always)]
    pub fn write_thr_a(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::THR_A_MASK) | ((value << Self::THR_A_OFFSET) & Self::THR_A_MASK)
        });
    }
}

/// RD_TIM_CONF value type for batch read/modify/write operations
///
/// Use `rdtimconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdtimconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdTimConfVal {
    bits: u32,
}

impl RdTimConfVal {
    /// Create from raw bits
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
        Self { bits: 0x0f010201 }
    }

    /// Read **READ_INIT_NUM** `[31:24]`
    ///
    /// Configures the waiting time of reading eFuse memory.
    #[inline(always)]
    pub const fn read_init_num(&self) -> u32 {
        (self.bits & RdTimConf::READ_INIT_NUM_MASK) >> RdTimConf::READ_INIT_NUM_OFFSET
    }

    /// Read **TSUR_A** `[23:16]`
    ///
    /// Configures the read setup time.
    #[inline(always)]
    pub const fn tsur_a(&self) -> u32 {
        (self.bits & RdTimConf::TSUR_A_MASK) >> RdTimConf::TSUR_A_OFFSET
    }

    /// Read **TRD** `[15:8]`
    ///
    /// Configures the read time.
    #[inline(always)]
    pub const fn trd(&self) -> u32 {
        (self.bits & RdTimConf::TRD_MASK) >> RdTimConf::TRD_OFFSET
    }

    /// Read **THR_A** `[7:0]`
    ///
    /// Configures the read hold time.
    #[inline(always)]
    pub const fn thr_a(&self) -> u32 {
        (self.bits & RdTimConf::THR_A_MASK) >> RdTimConf::THR_A_OFFSET
    }

    /// Set **READ_INIT_NUM** `[31:24]`
    ///
    /// Configures the waiting time of reading eFuse memory.
    #[inline(always)]
    pub const fn with_read_init_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RdTimConf::READ_INIT_NUM_MASK)
                | ((value << RdTimConf::READ_INIT_NUM_OFFSET) & RdTimConf::READ_INIT_NUM_MASK),
        }
    }

    /// Set **TSUR_A** `[23:16]`
    ///
    /// Configures the read setup time.
    #[inline(always)]
    pub const fn with_tsur_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RdTimConf::TSUR_A_MASK)
                | ((value << RdTimConf::TSUR_A_OFFSET) & RdTimConf::TSUR_A_MASK),
        }
    }

    /// Set **TRD** `[15:8]`
    ///
    /// Configures the read time.
    #[inline(always)]
    pub const fn with_trd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RdTimConf::TRD_MASK)
                | ((value << RdTimConf::TRD_OFFSET) & RdTimConf::TRD_MASK),
        }
    }

    /// Set **THR_A** `[7:0]`
    ///
    /// Configures the read hold time.
    #[inline(always)]
    pub const fn with_thr_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RdTimConf::THR_A_MASK)
                | ((value << RdTimConf::THR_A_OFFSET) & RdTimConf::THR_A_MASK),
        }
    }
}

impl Default for RdTimConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RdTimConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdTimConfVal {
        RdTimConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RdTimConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RdTimConfVal) -> RdTimConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WR_TIM_CONF1 Register
///
/// Configurarion register 1 of eFuse programming timing parameters.
///
/// Address offset: 0x01f0
/// Reset value: 0x01266701
///
/// ## Fields
///
/// - **THP_A** `[31:24]` (RW)
///
///   Configures the programming hold time.
///
/// - **PWR_ON_NUM** `[23:8]` (RW)
///
///   Configures the power up time for VDDQ.
///
/// - **TSUP_A** `[7:0]` (RW)
///
///   Configures the programming setup time.
#[repr(transparent)]
pub struct WrTimConf1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrTimConf1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrTimConf1 {
    // ======== Field Constants ========

    /// **THP_A**
    ///
    /// Configures the programming hold time.
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const THP_A_OFFSET: u32 = 24;
    pub const THP_A_MASK: u32 = 0xff000000;

    /// **PWR_ON_NUM**
    ///
    /// Configures the power up time for VDDQ.
    ///
    /// Bits `[23:8]`, mask `0x00ffff00`
    pub const PWR_ON_NUM_OFFSET: u32 = 8;
    pub const PWR_ON_NUM_MASK: u32 = 0x00ffff00;

    /// **TSUP_A**
    ///
    /// Configures the programming setup time.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const TSUP_A_OFFSET: u32 = 0;
    pub const TSUP_A_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **THP_A** `[31:24]` (RW)
    ///
    /// Configures the programming hold time.
    #[inline(always)]
    pub fn thp_a(&self) -> u32 {
        (self.value.read_bits() & Self::THP_A_MASK) >> Self::THP_A_OFFSET
    }

    /// Write **THP_A** `[31:24]` (RW)
    ///
    /// Configures the programming hold time.
    #[inline(always)]
    pub fn write_thp_a(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::THP_A_MASK) | ((value << Self::THP_A_OFFSET) & Self::THP_A_MASK)
        });
    }

    /// **PWR_ON_NUM** `[23:8]` (RW)
    ///
    /// Configures the power up time for VDDQ.
    #[inline(always)]
    pub fn pwr_on_num(&self) -> u32 {
        (self.value.read_bits() & Self::PWR_ON_NUM_MASK) >> Self::PWR_ON_NUM_OFFSET
    }

    /// Write **PWR_ON_NUM** `[23:8]` (RW)
    ///
    /// Configures the power up time for VDDQ.
    #[inline(always)]
    pub fn write_pwr_on_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PWR_ON_NUM_MASK)
                | ((value << Self::PWR_ON_NUM_OFFSET) & Self::PWR_ON_NUM_MASK)
        });
    }

    /// **TSUP_A** `[7:0]` (RW)
    ///
    /// Configures the programming setup time.
    #[inline(always)]
    pub fn tsup_a(&self) -> u32 {
        (self.value.read_bits() & Self::TSUP_A_MASK) >> Self::TSUP_A_OFFSET
    }

    /// Write **TSUP_A** `[7:0]` (RW)
    ///
    /// Configures the programming setup time.
    #[inline(always)]
    pub fn write_tsup_a(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TSUP_A_MASK) | ((value << Self::TSUP_A_OFFSET) & Self::TSUP_A_MASK)
        });
    }
}

/// WR_TIM_CONF1 value type for batch read/modify/write operations
///
/// Use `wrtimconf1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wrtimconf1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WrTimConf1Val {
    bits: u32,
}

impl WrTimConf1Val {
    /// Create from raw bits
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
        Self { bits: 0x01266701 }
    }

    /// Read **THP_A** `[31:24]`
    ///
    /// Configures the programming hold time.
    #[inline(always)]
    pub const fn thp_a(&self) -> u32 {
        (self.bits & WrTimConf1::THP_A_MASK) >> WrTimConf1::THP_A_OFFSET
    }

    /// Read **PWR_ON_NUM** `[23:8]`
    ///
    /// Configures the power up time for VDDQ.
    #[inline(always)]
    pub const fn pwr_on_num(&self) -> u32 {
        (self.bits & WrTimConf1::PWR_ON_NUM_MASK) >> WrTimConf1::PWR_ON_NUM_OFFSET
    }

    /// Read **TSUP_A** `[7:0]`
    ///
    /// Configures the programming setup time.
    #[inline(always)]
    pub const fn tsup_a(&self) -> u32 {
        (self.bits & WrTimConf1::TSUP_A_MASK) >> WrTimConf1::TSUP_A_OFFSET
    }

    /// Set **THP_A** `[31:24]`
    ///
    /// Configures the programming hold time.
    #[inline(always)]
    pub const fn with_thp_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !WrTimConf1::THP_A_MASK)
                | ((value << WrTimConf1::THP_A_OFFSET) & WrTimConf1::THP_A_MASK),
        }
    }

    /// Set **PWR_ON_NUM** `[23:8]`
    ///
    /// Configures the power up time for VDDQ.
    #[inline(always)]
    pub const fn with_pwr_on_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !WrTimConf1::PWR_ON_NUM_MASK)
                | ((value << WrTimConf1::PWR_ON_NUM_OFFSET) & WrTimConf1::PWR_ON_NUM_MASK),
        }
    }

    /// Set **TSUP_A** `[7:0]`
    ///
    /// Configures the programming setup time.
    #[inline(always)]
    pub const fn with_tsup_a(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !WrTimConf1::TSUP_A_MASK)
                | ((value << WrTimConf1::TSUP_A_OFFSET) & WrTimConf1::TSUP_A_MASK),
        }
    }
}

impl Default for WrTimConf1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl WrTimConf1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WrTimConf1Val {
        WrTimConf1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WrTimConf1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WrTimConf1Val) -> WrTimConf1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WR_TIM_CONF2 Register
///
/// Configurarion register 2 of eFuse programming timing parameters.
///
/// Address offset: 0x01f4
/// Reset value: 0x00a00140
///
/// ## Fields
///
/// - **TPGM** `[31:16]` (RW)
///
///   Configures the active programming time.
///
/// - **PWR_OFF_NUM** `[15:0]` (RW)
///
///   Configures the power outage time for VDDQ.
#[repr(transparent)]
pub struct WrTimConf2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrTimConf2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrTimConf2 {
    // ======== Field Constants ========

    /// **TPGM**
    ///
    /// Configures the active programming time.
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const TPGM_OFFSET: u32 = 16;
    pub const TPGM_MASK: u32 = 0xffff0000;

    /// **PWR_OFF_NUM**
    ///
    /// Configures the power outage time for VDDQ.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PWR_OFF_NUM_OFFSET: u32 = 0;
    pub const PWR_OFF_NUM_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **TPGM** `[31:16]` (RW)
    ///
    /// Configures the active programming time.
    #[inline(always)]
    pub fn tpgm(&self) -> u32 {
        (self.value.read_bits() & Self::TPGM_MASK) >> Self::TPGM_OFFSET
    }

    /// Write **TPGM** `[31:16]` (RW)
    ///
    /// Configures the active programming time.
    #[inline(always)]
    pub fn write_tpgm(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TPGM_MASK) | ((value << Self::TPGM_OFFSET) & Self::TPGM_MASK)
        });
    }

    /// **PWR_OFF_NUM** `[15:0]` (RW)
    ///
    /// Configures the power outage time for VDDQ.
    #[inline(always)]
    pub fn pwr_off_num(&self) -> u32 {
        (self.value.read_bits() & Self::PWR_OFF_NUM_MASK) >> Self::PWR_OFF_NUM_OFFSET
    }

    /// Write **PWR_OFF_NUM** `[15:0]` (RW)
    ///
    /// Configures the power outage time for VDDQ.
    #[inline(always)]
    pub fn write_pwr_off_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PWR_OFF_NUM_MASK)
                | ((value << Self::PWR_OFF_NUM_OFFSET) & Self::PWR_OFF_NUM_MASK)
        });
    }
}

/// WR_TIM_CONF2 value type for batch read/modify/write operations
///
/// Use `wrtimconf2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wrtimconf2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WrTimConf2Val {
    bits: u32,
}

impl WrTimConf2Val {
    /// Create from raw bits
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
        Self { bits: 0x00a00140 }
    }

    /// Read **TPGM** `[31:16]`
    ///
    /// Configures the active programming time.
    #[inline(always)]
    pub const fn tpgm(&self) -> u32 {
        (self.bits & WrTimConf2::TPGM_MASK) >> WrTimConf2::TPGM_OFFSET
    }

    /// Read **PWR_OFF_NUM** `[15:0]`
    ///
    /// Configures the power outage time for VDDQ.
    #[inline(always)]
    pub const fn pwr_off_num(&self) -> u32 {
        (self.bits & WrTimConf2::PWR_OFF_NUM_MASK) >> WrTimConf2::PWR_OFF_NUM_OFFSET
    }

    /// Set **TPGM** `[31:16]`
    ///
    /// Configures the active programming time.
    #[inline(always)]
    pub const fn with_tpgm(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !WrTimConf2::TPGM_MASK)
                | ((value << WrTimConf2::TPGM_OFFSET) & WrTimConf2::TPGM_MASK),
        }
    }

    /// Set **PWR_OFF_NUM** `[15:0]`
    ///
    /// Configures the power outage time for VDDQ.
    #[inline(always)]
    pub const fn with_pwr_off_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !WrTimConf2::PWR_OFF_NUM_MASK)
                | ((value << WrTimConf2::PWR_OFF_NUM_OFFSET) & WrTimConf2::PWR_OFF_NUM_MASK),
        }
    }
}

impl Default for WrTimConf2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl WrTimConf2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WrTimConf2Val {
        WrTimConf2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WrTimConf2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WrTimConf2Val) -> WrTimConf2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// WR_TIM_CONF0_RS_BYPASS Register
///
/// Configurarion register0 of eFuse programming time parameters and rs bypass operation.
///
/// Address offset: 0x01f8
/// Reset value: 0x00002000
///
/// ## Fields
///
/// - **TPGM_INACTIVE** `[20:13]` (RW)
///
///   Configures the inactive programming time.
///
/// - **UPDATE** `[12]` (WO)
///
///   Set this bit to update multi-bit register signals.
///
/// - **BYPASS_RS_BLK_NUM** `[11:1]` (RW)
///
///   Configures block number of programming twice operation.
///
/// - **BYPASS_RS_CORRECTION** `[0]` (RW)
///
///   Set this bit to bypass reed solomon correction step.
#[repr(transparent)]
pub struct WrTimConf0RsBypass {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrTimConf0RsBypass {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrTimConf0RsBypass {
    // ======== Field Constants ========

    /// **TPGM_INACTIVE**
    ///
    /// Configures the inactive programming time.
    ///
    /// Bits `[20:13]`, mask `0x001fe000`
    pub const TPGM_INACTIVE_OFFSET: u32 = 13;
    pub const TPGM_INACTIVE_MASK: u32 = 0x001fe000;

    /// **UPDATE**
    ///
    /// Set this bit to update multi-bit register signals.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const UPDATE_OFFSET: u32 = 12;
    pub const UPDATE_MASK: u32 = 0x00001000;
    pub const UPDATE: u32 = 1 << 12;

    /// **BYPASS_RS_BLK_NUM**
    ///
    /// Configures block number of programming twice operation.
    ///
    /// Bits `[11:1]`, mask `0x00000ffe`
    pub const BYPASS_RS_BLK_NUM_OFFSET: u32 = 1;
    pub const BYPASS_RS_BLK_NUM_MASK: u32 = 0x00000ffe;

    /// **BYPASS_RS_CORRECTION**
    ///
    /// Set this bit to bypass reed solomon correction step.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const BYPASS_RS_CORRECTION_OFFSET: u32 = 0;
    pub const BYPASS_RS_CORRECTION_MASK: u32 = 0x00000001;
    pub const BYPASS_RS_CORRECTION: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TPGM_INACTIVE** `[20:13]` (RW)
    ///
    /// Configures the inactive programming time.
    #[inline(always)]
    pub fn tpgm_inactive(&self) -> u32 {
        (self.value.read_bits() & Self::TPGM_INACTIVE_MASK) >> Self::TPGM_INACTIVE_OFFSET
    }

    /// Write **TPGM_INACTIVE** `[20:13]` (RW)
    ///
    /// Configures the inactive programming time.
    #[inline(always)]
    pub fn write_tpgm_inactive(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TPGM_INACTIVE_MASK)
                | ((value << Self::TPGM_INACTIVE_OFFSET) & Self::TPGM_INACTIVE_MASK)
        });
    }

    /// Trigger **UPDATE** (WO)
    ///
    /// Set this bit to update multi-bit register signals.
    #[inline(always)]
    pub fn trigger_update(&self) {
        self.value.modify_bits(|r| r | Self::UPDATE_MASK);
    }

    /// **BYPASS_RS_BLK_NUM** `[11:1]` (RW)
    ///
    /// Configures block number of programming twice operation.
    #[inline(always)]
    pub fn bypass_rs_blk_num(&self) -> u32 {
        (self.value.read_bits() & Self::BYPASS_RS_BLK_NUM_MASK) >> Self::BYPASS_RS_BLK_NUM_OFFSET
    }

    /// Write **BYPASS_RS_BLK_NUM** `[11:1]` (RW)
    ///
    /// Configures block number of programming twice operation.
    #[inline(always)]
    pub fn write_bypass_rs_blk_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BYPASS_RS_BLK_NUM_MASK)
                | ((value << Self::BYPASS_RS_BLK_NUM_OFFSET) & Self::BYPASS_RS_BLK_NUM_MASK)
        });
    }

    /// **BYPASS_RS_CORRECTION** `[0]` (RW)
    ///
    /// Set this bit to bypass reed solomon correction step.
    #[inline(always)]
    pub fn bypass_rs_correction(&self) -> bool {
        (self.value.read_bits() & Self::BYPASS_RS_CORRECTION_MASK) != 0
    }

    /// Set BYPASS_RS_CORRECTION bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bypass_rs_correction(&self) {
        self.value
            .modify_bits(|r| r | Self::BYPASS_RS_CORRECTION_MASK);
    }

    /// Clear BYPASS_RS_CORRECTION bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bypass_rs_correction(&self) {
        self.value
            .modify_bits(|r| r & !Self::BYPASS_RS_CORRECTION_MASK);
    }
}

/// WR_TIM_CONF0_RS_BYPASS value type for batch read/modify/write operations
///
/// Use `wrtimconf0rsbypass.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wrtimconf0rsbypass.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WrTimConf0RsBypassVal {
    bits: u32,
}

impl WrTimConf0RsBypassVal {
    /// Create from raw bits
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
        Self { bits: 0x00002000 }
    }

    /// Read **TPGM_INACTIVE** `[20:13]`
    ///
    /// Configures the inactive programming time.
    #[inline(always)]
    pub const fn tpgm_inactive(&self) -> u32 {
        (self.bits & WrTimConf0RsBypass::TPGM_INACTIVE_MASK)
            >> WrTimConf0RsBypass::TPGM_INACTIVE_OFFSET
    }

    /// Read **BYPASS_RS_BLK_NUM** `[11:1]`
    ///
    /// Configures block number of programming twice operation.
    #[inline(always)]
    pub const fn bypass_rs_blk_num(&self) -> u32 {
        (self.bits & WrTimConf0RsBypass::BYPASS_RS_BLK_NUM_MASK)
            >> WrTimConf0RsBypass::BYPASS_RS_BLK_NUM_OFFSET
    }

    /// Read **BYPASS_RS_CORRECTION** `[0:0]`
    ///
    /// Set this bit to bypass reed solomon correction step.
    #[inline(always)]
    pub const fn bypass_rs_correction(&self) -> bool {
        (self.bits & WrTimConf0RsBypass::BYPASS_RS_CORRECTION_MASK) != 0
    }

    /// Set **TPGM_INACTIVE** `[20:13]`
    ///
    /// Configures the inactive programming time.
    #[inline(always)]
    pub const fn with_tpgm_inactive(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !WrTimConf0RsBypass::TPGM_INACTIVE_MASK)
                | ((value << WrTimConf0RsBypass::TPGM_INACTIVE_OFFSET)
                    & WrTimConf0RsBypass::TPGM_INACTIVE_MASK),
        }
    }

    /// Set **UPDATE** `[12:12]`
    ///
    /// Set this bit to update multi-bit register signals.
    #[inline(always)]
    pub const fn with_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WrTimConf0RsBypass::UPDATE_MASK
            } else {
                self.bits & !WrTimConf0RsBypass::UPDATE_MASK
            },
        }
    }

    /// Set **BYPASS_RS_BLK_NUM** `[11:1]`
    ///
    /// Configures block number of programming twice operation.
    #[inline(always)]
    pub const fn with_bypass_rs_blk_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !WrTimConf0RsBypass::BYPASS_RS_BLK_NUM_MASK)
                | ((value << WrTimConf0RsBypass::BYPASS_RS_BLK_NUM_OFFSET)
                    & WrTimConf0RsBypass::BYPASS_RS_BLK_NUM_MASK),
        }
    }

    /// Set **BYPASS_RS_CORRECTION** `[0:0]`
    ///
    /// Set this bit to bypass reed solomon correction step.
    #[inline(always)]
    pub const fn with_bypass_rs_correction(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WrTimConf0RsBypass::BYPASS_RS_CORRECTION_MASK
            } else {
                self.bits & !WrTimConf0RsBypass::BYPASS_RS_CORRECTION_MASK
            },
        }
    }
}

impl Default for WrTimConf0RsBypassVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl WrTimConf0RsBypass {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WrTimConf0RsBypassVal {
        WrTimConf0RsBypassVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WrTimConf0RsBypassVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WrTimConf0RsBypassVal) -> WrTimConf0RsBypassVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// eFuse version register.
///
/// Address offset: 0x01fc
/// Reset value: 0x02305050
///
/// ## Fields
///
/// - **DATE** `[27:0]` (RW)
///
///   Stores eFuse version.
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
    /// Stores eFuse version.
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **DATE** `[27:0]` (RW)
    ///
    /// Stores eFuse version.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[27:0]` (RW)
    ///
    /// Stores eFuse version.
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
        Self { bits: 0x02305050 }
    }

    /// Read **DATE** `[27:0]`
    ///
    /// Stores eFuse version.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[27:0]`
    ///
    /// Stores eFuse version.
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

/// APB2OTP_WR_DIS Register
///
/// eFuse apb2otp block0 data register1.
///
/// Address offset: 0x0800
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_WR_DIS** `[31:0]` (RO)
///
///   Otp block0 write disable data.
#[repr(transparent)]
pub struct Apb2otpWrDis {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpWrDis {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpWrDis {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_WR_DIS**
    ///
    /// Otp block0 write disable data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_WR_DIS_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_WR_DIS_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_WR_DIS** `[31:0]` (RO)
    ///
    /// Otp block0 write disable data.
    #[inline(always)]
    pub fn apb2otp_block0_wr_dis(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_WR_DIS_MASK)
            >> Self::APB2OTP_BLOCK0_WR_DIS_OFFSET
    }
}

/// APB2OTP_WR_DIS value type for batch read/modify/write operations
///
/// Use `apb2otpwrdis.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpwrdis.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpWrDisVal {
    bits: u32,
}

impl Apb2otpWrDisVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_WR_DIS** `[31:0]`
    ///
    /// Otp block0 write disable data.
    #[inline(always)]
    pub const fn apb2otp_block0_wr_dis(&self) -> u32 {
        (self.bits & Apb2otpWrDis::APB2OTP_BLOCK0_WR_DIS_MASK)
            >> Apb2otpWrDis::APB2OTP_BLOCK0_WR_DIS_OFFSET
    }
}

impl Default for Apb2otpWrDisVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpWrDis {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpWrDisVal {
        Apb2otpWrDisVal::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP1_W1 Register
///
/// eFuse apb2otp block0 data register2.
///
/// Address offset: 0x0804
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP1_W1** `[31:0]` (RO)
///
///   Otp block0 backup1 word1 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup1W1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup1W1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup1W1 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP1_W1**
    ///
    /// Otp block0 backup1 word1 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP1_W1_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP1_W1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP1_W1** `[31:0]` (RO)
    ///
    /// Otp block0 backup1 word1 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup1_w1(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP1_W1_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP1_W1_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP1_W1 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup1w1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup1w1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup1W1Val {
    bits: u32,
}

impl Apb2otpBlk0Backup1W1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP1_W1** `[31:0]`
    ///
    /// Otp block0 backup1 word1 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup1_w1(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup1W1::APB2OTP_BLOCK0_BACKUP1_W1_MASK)
            >> Apb2otpBlk0Backup1W1::APB2OTP_BLOCK0_BACKUP1_W1_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup1W1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup1W1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup1W1Val {
        Apb2otpBlk0Backup1W1Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP1_W2 Register
///
/// eFuse apb2otp block0 data register3.
///
/// Address offset: 0x0808
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP1_W2** `[31:0]` (RO)
///
///   Otp block0 backup1 word2 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup1W2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup1W2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup1W2 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP1_W2**
    ///
    /// Otp block0 backup1 word2 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP1_W2_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP1_W2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP1_W2** `[31:0]` (RO)
    ///
    /// Otp block0 backup1 word2 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup1_w2(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP1_W2_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP1_W2_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP1_W2 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup1w2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup1w2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup1W2Val {
    bits: u32,
}

impl Apb2otpBlk0Backup1W2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP1_W2** `[31:0]`
    ///
    /// Otp block0 backup1 word2 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup1_w2(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup1W2::APB2OTP_BLOCK0_BACKUP1_W2_MASK)
            >> Apb2otpBlk0Backup1W2::APB2OTP_BLOCK0_BACKUP1_W2_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup1W2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup1W2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup1W2Val {
        Apb2otpBlk0Backup1W2Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP1_W3 Register
///
/// eFuse apb2otp block0 data register4.
///
/// Address offset: 0x080c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP1_W3** `[31:0]` (RO)
///
///   Otp block0 backup1 word3 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup1W3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup1W3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup1W3 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP1_W3**
    ///
    /// Otp block0 backup1 word3 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP1_W3_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP1_W3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP1_W3** `[31:0]` (RO)
    ///
    /// Otp block0 backup1 word3 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup1_w3(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP1_W3_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP1_W3_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP1_W3 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup1w3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup1w3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup1W3Val {
    bits: u32,
}

impl Apb2otpBlk0Backup1W3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP1_W3** `[31:0]`
    ///
    /// Otp block0 backup1 word3 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup1_w3(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup1W3::APB2OTP_BLOCK0_BACKUP1_W3_MASK)
            >> Apb2otpBlk0Backup1W3::APB2OTP_BLOCK0_BACKUP1_W3_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup1W3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup1W3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup1W3Val {
        Apb2otpBlk0Backup1W3Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP1_W4 Register
///
/// eFuse apb2otp block0 data register5.
///
/// Address offset: 0x0810
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP1_W4** `[31:0]` (RO)
///
///   Otp block0 backup1 word4 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup1W4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup1W4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup1W4 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP1_W4**
    ///
    /// Otp block0 backup1 word4 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP1_W4_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP1_W4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP1_W4** `[31:0]` (RO)
    ///
    /// Otp block0 backup1 word4 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup1_w4(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP1_W4_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP1_W4_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP1_W4 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup1w4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup1w4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup1W4Val {
    bits: u32,
}

impl Apb2otpBlk0Backup1W4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP1_W4** `[31:0]`
    ///
    /// Otp block0 backup1 word4 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup1_w4(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup1W4::APB2OTP_BLOCK0_BACKUP1_W4_MASK)
            >> Apb2otpBlk0Backup1W4::APB2OTP_BLOCK0_BACKUP1_W4_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup1W4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup1W4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup1W4Val {
        Apb2otpBlk0Backup1W4Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP1_W5 Register
///
/// eFuse apb2otp block0 data register6.
///
/// Address offset: 0x0814
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP1_W5** `[31:0]` (RO)
///
///   Otp block0 backup1 word5 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup1W5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup1W5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup1W5 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP1_W5**
    ///
    /// Otp block0 backup1 word5 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP1_W5_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP1_W5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP1_W5** `[31:0]` (RO)
    ///
    /// Otp block0 backup1 word5 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup1_w5(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP1_W5_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP1_W5_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP1_W5 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup1w5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup1w5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup1W5Val {
    bits: u32,
}

impl Apb2otpBlk0Backup1W5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP1_W5** `[31:0]`
    ///
    /// Otp block0 backup1 word5 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup1_w5(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup1W5::APB2OTP_BLOCK0_BACKUP1_W5_MASK)
            >> Apb2otpBlk0Backup1W5::APB2OTP_BLOCK0_BACKUP1_W5_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup1W5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup1W5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup1W5Val {
        Apb2otpBlk0Backup1W5Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP2_W1 Register
///
/// eFuse apb2otp block0 data register7.
///
/// Address offset: 0x0818
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP2_W1** `[31:0]` (RO)
///
///   Otp block0 backup2 word1 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup2W1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup2W1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup2W1 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP2_W1**
    ///
    /// Otp block0 backup2 word1 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP2_W1_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP2_W1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP2_W1** `[31:0]` (RO)
    ///
    /// Otp block0 backup2 word1 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup2_w1(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP2_W1_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP2_W1_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP2_W1 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup2w1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup2w1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup2W1Val {
    bits: u32,
}

impl Apb2otpBlk0Backup2W1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP2_W1** `[31:0]`
    ///
    /// Otp block0 backup2 word1 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup2_w1(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup2W1::APB2OTP_BLOCK0_BACKUP2_W1_MASK)
            >> Apb2otpBlk0Backup2W1::APB2OTP_BLOCK0_BACKUP2_W1_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup2W1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup2W1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup2W1Val {
        Apb2otpBlk0Backup2W1Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP2_W2 Register
///
/// eFuse apb2otp block0 data register8.
///
/// Address offset: 0x081c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP2_W2** `[31:0]` (RO)
///
///   Otp block0 backup2 word2 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup2W2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup2W2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup2W2 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP2_W2**
    ///
    /// Otp block0 backup2 word2 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP2_W2_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP2_W2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP2_W2** `[31:0]` (RO)
    ///
    /// Otp block0 backup2 word2 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup2_w2(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP2_W2_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP2_W2_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP2_W2 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup2w2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup2w2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup2W2Val {
    bits: u32,
}

impl Apb2otpBlk0Backup2W2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP2_W2** `[31:0]`
    ///
    /// Otp block0 backup2 word2 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup2_w2(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup2W2::APB2OTP_BLOCK0_BACKUP2_W2_MASK)
            >> Apb2otpBlk0Backup2W2::APB2OTP_BLOCK0_BACKUP2_W2_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup2W2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup2W2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup2W2Val {
        Apb2otpBlk0Backup2W2Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP2_W3 Register
///
/// eFuse apb2otp block0 data register9.
///
/// Address offset: 0x0820
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP2_W3** `[31:0]` (RO)
///
///   Otp block0 backup2 word3 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup2W3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup2W3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup2W3 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP2_W3**
    ///
    /// Otp block0 backup2 word3 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP2_W3_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP2_W3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP2_W3** `[31:0]` (RO)
    ///
    /// Otp block0 backup2 word3 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup2_w3(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP2_W3_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP2_W3_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP2_W3 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup2w3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup2w3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup2W3Val {
    bits: u32,
}

impl Apb2otpBlk0Backup2W3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP2_W3** `[31:0]`
    ///
    /// Otp block0 backup2 word3 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup2_w3(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup2W3::APB2OTP_BLOCK0_BACKUP2_W3_MASK)
            >> Apb2otpBlk0Backup2W3::APB2OTP_BLOCK0_BACKUP2_W3_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup2W3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup2W3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup2W3Val {
        Apb2otpBlk0Backup2W3Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP2_W4 Register
///
/// eFuse apb2otp block0 data register10.
///
/// Address offset: 0x0824
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP2_W4** `[31:0]` (RO)
///
///   Otp block0 backup2 word4 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup2W4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup2W4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup2W4 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP2_W4**
    ///
    /// Otp block0 backup2 word4 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP2_W4_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP2_W4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP2_W4** `[31:0]` (RO)
    ///
    /// Otp block0 backup2 word4 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup2_w4(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP2_W4_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP2_W4_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP2_W4 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup2w4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup2w4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup2W4Val {
    bits: u32,
}

impl Apb2otpBlk0Backup2W4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP2_W4** `[31:0]`
    ///
    /// Otp block0 backup2 word4 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup2_w4(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup2W4::APB2OTP_BLOCK0_BACKUP2_W4_MASK)
            >> Apb2otpBlk0Backup2W4::APB2OTP_BLOCK0_BACKUP2_W4_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup2W4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup2W4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup2W4Val {
        Apb2otpBlk0Backup2W4Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP2_W5 Register
///
/// eFuse apb2otp block0 data register11.
///
/// Address offset: 0x0828
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP2_W5** `[31:0]` (RO)
///
///   Otp block0 backup2 word5 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup2W5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup2W5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup2W5 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP2_W5**
    ///
    /// Otp block0 backup2 word5 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP2_W5_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP2_W5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP2_W5** `[31:0]` (RO)
    ///
    /// Otp block0 backup2 word5 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup2_w5(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP2_W5_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP2_W5_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP2_W5 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup2w5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup2w5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup2W5Val {
    bits: u32,
}

impl Apb2otpBlk0Backup2W5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP2_W5** `[31:0]`
    ///
    /// Otp block0 backup2 word5 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup2_w5(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup2W5::APB2OTP_BLOCK0_BACKUP2_W5_MASK)
            >> Apb2otpBlk0Backup2W5::APB2OTP_BLOCK0_BACKUP2_W5_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup2W5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup2W5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup2W5Val {
        Apb2otpBlk0Backup2W5Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP3_W1 Register
///
/// eFuse apb2otp block0 data register12.
///
/// Address offset: 0x082c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP3_W1** `[31:0]` (RO)
///
///   Otp block0 backup3 word1 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup3W1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup3W1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup3W1 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP3_W1**
    ///
    /// Otp block0 backup3 word1 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP3_W1_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP3_W1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP3_W1** `[31:0]` (RO)
    ///
    /// Otp block0 backup3 word1 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup3_w1(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP3_W1_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP3_W1_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP3_W1 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup3w1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup3w1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup3W1Val {
    bits: u32,
}

impl Apb2otpBlk0Backup3W1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP3_W1** `[31:0]`
    ///
    /// Otp block0 backup3 word1 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup3_w1(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup3W1::APB2OTP_BLOCK0_BACKUP3_W1_MASK)
            >> Apb2otpBlk0Backup3W1::APB2OTP_BLOCK0_BACKUP3_W1_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup3W1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup3W1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup3W1Val {
        Apb2otpBlk0Backup3W1Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP3_W2 Register
///
/// eFuse apb2otp block0 data register13.
///
/// Address offset: 0x0830
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP3_W2** `[31:0]` (RO)
///
///   Otp block0 backup3 word2 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup3W2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup3W2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup3W2 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP3_W2**
    ///
    /// Otp block0 backup3 word2 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP3_W2_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP3_W2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP3_W2** `[31:0]` (RO)
    ///
    /// Otp block0 backup3 word2 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup3_w2(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP3_W2_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP3_W2_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP3_W2 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup3w2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup3w2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup3W2Val {
    bits: u32,
}

impl Apb2otpBlk0Backup3W2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP3_W2** `[31:0]`
    ///
    /// Otp block0 backup3 word2 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup3_w2(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup3W2::APB2OTP_BLOCK0_BACKUP3_W2_MASK)
            >> Apb2otpBlk0Backup3W2::APB2OTP_BLOCK0_BACKUP3_W2_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup3W2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup3W2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup3W2Val {
        Apb2otpBlk0Backup3W2Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP3_W3 Register
///
/// eFuse apb2otp block0 data register14.
///
/// Address offset: 0x0834
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP3_W3** `[31:0]` (RO)
///
///   Otp block0 backup3 word3 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup3W3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup3W3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup3W3 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP3_W3**
    ///
    /// Otp block0 backup3 word3 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP3_W3_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP3_W3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP3_W3** `[31:0]` (RO)
    ///
    /// Otp block0 backup3 word3 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup3_w3(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP3_W3_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP3_W3_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP3_W3 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup3w3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup3w3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup3W3Val {
    bits: u32,
}

impl Apb2otpBlk0Backup3W3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP3_W3** `[31:0]`
    ///
    /// Otp block0 backup3 word3 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup3_w3(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup3W3::APB2OTP_BLOCK0_BACKUP3_W3_MASK)
            >> Apb2otpBlk0Backup3W3::APB2OTP_BLOCK0_BACKUP3_W3_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup3W3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup3W3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup3W3Val {
        Apb2otpBlk0Backup3W3Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP3_W4 Register
///
/// eFuse apb2otp block0 data register15.
///
/// Address offset: 0x0838
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP3_W4** `[31:0]` (RO)
///
///   Otp block0 backup3 word4 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup3W4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup3W4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup3W4 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP3_W4**
    ///
    /// Otp block0 backup3 word4 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP3_W4_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP3_W4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP3_W4** `[31:0]` (RO)
    ///
    /// Otp block0 backup3 word4 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup3_w4(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP3_W4_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP3_W4_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP3_W4 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup3w4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup3w4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup3W4Val {
    bits: u32,
}

impl Apb2otpBlk0Backup3W4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP3_W4** `[31:0]`
    ///
    /// Otp block0 backup3 word4 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup3_w4(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup3W4::APB2OTP_BLOCK0_BACKUP3_W4_MASK)
            >> Apb2otpBlk0Backup3W4::APB2OTP_BLOCK0_BACKUP3_W4_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup3W4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup3W4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup3W4Val {
        Apb2otpBlk0Backup3W4Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP3_W5 Register
///
/// eFuse apb2otp block0 data register16.
///
/// Address offset: 0x083c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP3_W5** `[31:0]` (RO)
///
///   Otp block0 backup3 word5 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup3W5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup3W5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup3W5 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP3_W5**
    ///
    /// Otp block0 backup3 word5 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP3_W5_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP3_W5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP3_W5** `[31:0]` (RO)
    ///
    /// Otp block0 backup3 word5 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup3_w5(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP3_W5_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP3_W5_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP3_W5 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup3w5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup3w5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup3W5Val {
    bits: u32,
}

impl Apb2otpBlk0Backup3W5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP3_W5** `[31:0]`
    ///
    /// Otp block0 backup3 word5 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup3_w5(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup3W5::APB2OTP_BLOCK0_BACKUP3_W5_MASK)
            >> Apb2otpBlk0Backup3W5::APB2OTP_BLOCK0_BACKUP3_W5_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup3W5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup3W5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup3W5Val {
        Apb2otpBlk0Backup3W5Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP4_W1 Register
///
/// eFuse apb2otp block0 data register17.
///
/// Address offset: 0x0840
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP4_W1** `[31:0]` (RO)
///
///   Otp block0 backup4 word1 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup4W1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup4W1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup4W1 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP4_W1**
    ///
    /// Otp block0 backup4 word1 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP4_W1_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP4_W1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP4_W1** `[31:0]` (RO)
    ///
    /// Otp block0 backup4 word1 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup4_w1(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP4_W1_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP4_W1_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP4_W1 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup4w1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup4w1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup4W1Val {
    bits: u32,
}

impl Apb2otpBlk0Backup4W1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP4_W1** `[31:0]`
    ///
    /// Otp block0 backup4 word1 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup4_w1(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup4W1::APB2OTP_BLOCK0_BACKUP4_W1_MASK)
            >> Apb2otpBlk0Backup4W1::APB2OTP_BLOCK0_BACKUP4_W1_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup4W1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup4W1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup4W1Val {
        Apb2otpBlk0Backup4W1Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP4_W2 Register
///
/// eFuse apb2otp block0 data register18.
///
/// Address offset: 0x0844
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP4_W2** `[31:0]` (RO)
///
///   Otp block0 backup4 word2 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup4W2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup4W2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup4W2 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP4_W2**
    ///
    /// Otp block0 backup4 word2 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP4_W2_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP4_W2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP4_W2** `[31:0]` (RO)
    ///
    /// Otp block0 backup4 word2 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup4_w2(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP4_W2_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP4_W2_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP4_W2 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup4w2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup4w2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup4W2Val {
    bits: u32,
}

impl Apb2otpBlk0Backup4W2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP4_W2** `[31:0]`
    ///
    /// Otp block0 backup4 word2 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup4_w2(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup4W2::APB2OTP_BLOCK0_BACKUP4_W2_MASK)
            >> Apb2otpBlk0Backup4W2::APB2OTP_BLOCK0_BACKUP4_W2_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup4W2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup4W2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup4W2Val {
        Apb2otpBlk0Backup4W2Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP4_W3 Register
///
/// eFuse apb2otp block0 data register19.
///
/// Address offset: 0x0848
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP4_W3** `[31:0]` (RO)
///
///   Otp block0 backup4 word3 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup4W3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup4W3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup4W3 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP4_W3**
    ///
    /// Otp block0 backup4 word3 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP4_W3_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP4_W3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP4_W3** `[31:0]` (RO)
    ///
    /// Otp block0 backup4 word3 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup4_w3(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP4_W3_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP4_W3_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP4_W3 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup4w3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup4w3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup4W3Val {
    bits: u32,
}

impl Apb2otpBlk0Backup4W3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP4_W3** `[31:0]`
    ///
    /// Otp block0 backup4 word3 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup4_w3(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup4W3::APB2OTP_BLOCK0_BACKUP4_W3_MASK)
            >> Apb2otpBlk0Backup4W3::APB2OTP_BLOCK0_BACKUP4_W3_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup4W3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup4W3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup4W3Val {
        Apb2otpBlk0Backup4W3Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP4_W4 Register
///
/// eFuse apb2otp block0 data register20.
///
/// Address offset: 0x084c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP4_W4** `[31:0]` (RO)
///
///   Otp block0 backup4 word4 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup4W4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup4W4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup4W4 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP4_W4**
    ///
    /// Otp block0 backup4 word4 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP4_W4_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP4_W4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP4_W4** `[31:0]` (RO)
    ///
    /// Otp block0 backup4 word4 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup4_w4(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP4_W4_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP4_W4_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP4_W4 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup4w4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup4w4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup4W4Val {
    bits: u32,
}

impl Apb2otpBlk0Backup4W4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP4_W4** `[31:0]`
    ///
    /// Otp block0 backup4 word4 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup4_w4(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup4W4::APB2OTP_BLOCK0_BACKUP4_W4_MASK)
            >> Apb2otpBlk0Backup4W4::APB2OTP_BLOCK0_BACKUP4_W4_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup4W4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup4W4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup4W4Val {
        Apb2otpBlk0Backup4W4Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK0_BACKUP4_W5 Register
///
/// eFuse apb2otp block0 data register21.
///
/// Address offset: 0x0850
///
/// ## Fields
///
/// - **APB2OTP_BLOCK0_BACKUP4_W5** `[31:0]` (RO)
///
///   Otp block0 backup4 word5 data.
#[repr(transparent)]
pub struct Apb2otpBlk0Backup4W5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk0Backup4W5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk0Backup4W5 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK0_BACKUP4_W5**
    ///
    /// Otp block0 backup4 word5 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK0_BACKUP4_W5_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK0_BACKUP4_W5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK0_BACKUP4_W5** `[31:0]` (RO)
    ///
    /// Otp block0 backup4 word5 data.
    #[inline(always)]
    pub fn apb2otp_block0_backup4_w5(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK0_BACKUP4_W5_MASK)
            >> Self::APB2OTP_BLOCK0_BACKUP4_W5_OFFSET
    }
}

/// APB2OTP_BLK0_BACKUP4_W5 value type for batch read/modify/write operations
///
/// Use `apb2otpblk0backup4w5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk0backup4w5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk0Backup4W5Val {
    bits: u32,
}

impl Apb2otpBlk0Backup4W5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK0_BACKUP4_W5** `[31:0]`
    ///
    /// Otp block0 backup4 word5 data.
    #[inline(always)]
    pub const fn apb2otp_block0_backup4_w5(&self) -> u32 {
        (self.bits & Apb2otpBlk0Backup4W5::APB2OTP_BLOCK0_BACKUP4_W5_MASK)
            >> Apb2otpBlk0Backup4W5::APB2OTP_BLOCK0_BACKUP4_W5_OFFSET
    }
}

impl Default for Apb2otpBlk0Backup4W5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk0Backup4W5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk0Backup4W5Val {
        Apb2otpBlk0Backup4W5Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK1_W1 Register
///
/// eFuse apb2otp block1 data register1.
///
/// Address offset: 0x0854
///
/// ## Fields
///
/// - **APB2OTP_BLOCK1_W1** `[31:0]` (RO)
///
///   Otp block1 word1 data.
#[repr(transparent)]
pub struct Apb2otpBlk1W1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk1W1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk1W1 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK1_W1**
    ///
    /// Otp block1 word1 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK1_W1_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK1_W1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK1_W1** `[31:0]` (RO)
    ///
    /// Otp block1 word1 data.
    #[inline(always)]
    pub fn apb2otp_block1_w1(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK1_W1_MASK) >> Self::APB2OTP_BLOCK1_W1_OFFSET
    }
}

/// APB2OTP_BLK1_W1 value type for batch read/modify/write operations
///
/// Use `apb2otpblk1w1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk1w1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk1W1Val {
    bits: u32,
}

impl Apb2otpBlk1W1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK1_W1** `[31:0]`
    ///
    /// Otp block1 word1 data.
    #[inline(always)]
    pub const fn apb2otp_block1_w1(&self) -> u32 {
        (self.bits & Apb2otpBlk1W1::APB2OTP_BLOCK1_W1_MASK)
            >> Apb2otpBlk1W1::APB2OTP_BLOCK1_W1_OFFSET
    }
}

impl Default for Apb2otpBlk1W1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk1W1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk1W1Val {
        Apb2otpBlk1W1Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK1_W2 Register
///
/// eFuse apb2otp block1 data register2.
///
/// Address offset: 0x0858
///
/// ## Fields
///
/// - **APB2OTP_BLOCK1_W2** `[31:0]` (RO)
///
///   Otp block1 word2 data.
#[repr(transparent)]
pub struct Apb2otpBlk1W2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk1W2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk1W2 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK1_W2**
    ///
    /// Otp block1 word2 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK1_W2_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK1_W2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK1_W2** `[31:0]` (RO)
    ///
    /// Otp block1 word2 data.
    #[inline(always)]
    pub fn apb2otp_block1_w2(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK1_W2_MASK) >> Self::APB2OTP_BLOCK1_W2_OFFSET
    }
}

/// APB2OTP_BLK1_W2 value type for batch read/modify/write operations
///
/// Use `apb2otpblk1w2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk1w2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk1W2Val {
    bits: u32,
}

impl Apb2otpBlk1W2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK1_W2** `[31:0]`
    ///
    /// Otp block1 word2 data.
    #[inline(always)]
    pub const fn apb2otp_block1_w2(&self) -> u32 {
        (self.bits & Apb2otpBlk1W2::APB2OTP_BLOCK1_W2_MASK)
            >> Apb2otpBlk1W2::APB2OTP_BLOCK1_W2_OFFSET
    }
}

impl Default for Apb2otpBlk1W2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk1W2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk1W2Val {
        Apb2otpBlk1W2Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK1_W3 Register
///
/// eFuse apb2otp block1 data register3.
///
/// Address offset: 0x085c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK1_W3** `[31:0]` (RO)
///
///   Otp block1 word3 data.
#[repr(transparent)]
pub struct Apb2otpBlk1W3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk1W3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk1W3 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK1_W3**
    ///
    /// Otp block1 word3 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK1_W3_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK1_W3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK1_W3** `[31:0]` (RO)
    ///
    /// Otp block1 word3 data.
    #[inline(always)]
    pub fn apb2otp_block1_w3(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK1_W3_MASK) >> Self::APB2OTP_BLOCK1_W3_OFFSET
    }
}

/// APB2OTP_BLK1_W3 value type for batch read/modify/write operations
///
/// Use `apb2otpblk1w3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk1w3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk1W3Val {
    bits: u32,
}

impl Apb2otpBlk1W3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK1_W3** `[31:0]`
    ///
    /// Otp block1 word3 data.
    #[inline(always)]
    pub const fn apb2otp_block1_w3(&self) -> u32 {
        (self.bits & Apb2otpBlk1W3::APB2OTP_BLOCK1_W3_MASK)
            >> Apb2otpBlk1W3::APB2OTP_BLOCK1_W3_OFFSET
    }
}

impl Default for Apb2otpBlk1W3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk1W3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk1W3Val {
        Apb2otpBlk1W3Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK1_W4 Register
///
/// eFuse apb2otp block1 data register4.
///
/// Address offset: 0x0860
///
/// ## Fields
///
/// - **APB2OTP_BLOCK1_W4** `[31:0]` (RO)
///
///   Otp block1 word4 data.
#[repr(transparent)]
pub struct Apb2otpBlk1W4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk1W4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk1W4 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK1_W4**
    ///
    /// Otp block1 word4 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK1_W4_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK1_W4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK1_W4** `[31:0]` (RO)
    ///
    /// Otp block1 word4 data.
    #[inline(always)]
    pub fn apb2otp_block1_w4(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK1_W4_MASK) >> Self::APB2OTP_BLOCK1_W4_OFFSET
    }
}

/// APB2OTP_BLK1_W4 value type for batch read/modify/write operations
///
/// Use `apb2otpblk1w4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk1w4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk1W4Val {
    bits: u32,
}

impl Apb2otpBlk1W4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK1_W4** `[31:0]`
    ///
    /// Otp block1 word4 data.
    #[inline(always)]
    pub const fn apb2otp_block1_w4(&self) -> u32 {
        (self.bits & Apb2otpBlk1W4::APB2OTP_BLOCK1_W4_MASK)
            >> Apb2otpBlk1W4::APB2OTP_BLOCK1_W4_OFFSET
    }
}

impl Default for Apb2otpBlk1W4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk1W4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk1W4Val {
        Apb2otpBlk1W4Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK1_W5 Register
///
/// eFuse apb2otp block1 data register5.
///
/// Address offset: 0x0864
///
/// ## Fields
///
/// - **APB2OTP_BLOCK1_W5** `[31:0]` (RO)
///
///   Otp block1 word5 data.
#[repr(transparent)]
pub struct Apb2otpBlk1W5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk1W5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk1W5 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK1_W5**
    ///
    /// Otp block1 word5 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK1_W5_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK1_W5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK1_W5** `[31:0]` (RO)
    ///
    /// Otp block1 word5 data.
    #[inline(always)]
    pub fn apb2otp_block1_w5(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK1_W5_MASK) >> Self::APB2OTP_BLOCK1_W5_OFFSET
    }
}

/// APB2OTP_BLK1_W5 value type for batch read/modify/write operations
///
/// Use `apb2otpblk1w5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk1w5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk1W5Val {
    bits: u32,
}

impl Apb2otpBlk1W5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK1_W5** `[31:0]`
    ///
    /// Otp block1 word5 data.
    #[inline(always)]
    pub const fn apb2otp_block1_w5(&self) -> u32 {
        (self.bits & Apb2otpBlk1W5::APB2OTP_BLOCK1_W5_MASK)
            >> Apb2otpBlk1W5::APB2OTP_BLOCK1_W5_OFFSET
    }
}

impl Default for Apb2otpBlk1W5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk1W5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk1W5Val {
        Apb2otpBlk1W5Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK1_W6 Register
///
/// eFuse apb2otp block1 data register6.
///
/// Address offset: 0x0868
///
/// ## Fields
///
/// - **APB2OTP_BLOCK1_W6** `[31:0]` (RO)
///
///   Otp block1 word6 data.
#[repr(transparent)]
pub struct Apb2otpBlk1W6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk1W6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk1W6 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK1_W6**
    ///
    /// Otp block1 word6 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK1_W6_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK1_W6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK1_W6** `[31:0]` (RO)
    ///
    /// Otp block1 word6 data.
    #[inline(always)]
    pub fn apb2otp_block1_w6(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK1_W6_MASK) >> Self::APB2OTP_BLOCK1_W6_OFFSET
    }
}

/// APB2OTP_BLK1_W6 value type for batch read/modify/write operations
///
/// Use `apb2otpblk1w6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk1w6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk1W6Val {
    bits: u32,
}

impl Apb2otpBlk1W6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK1_W6** `[31:0]`
    ///
    /// Otp block1 word6 data.
    #[inline(always)]
    pub const fn apb2otp_block1_w6(&self) -> u32 {
        (self.bits & Apb2otpBlk1W6::APB2OTP_BLOCK1_W6_MASK)
            >> Apb2otpBlk1W6::APB2OTP_BLOCK1_W6_OFFSET
    }
}

impl Default for Apb2otpBlk1W6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk1W6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk1W6Val {
        Apb2otpBlk1W6Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK1_W7 Register
///
/// eFuse apb2otp block1 data register7.
///
/// Address offset: 0x086c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK1_W7** `[31:0]` (RO)
///
///   Otp block1 word7 data.
#[repr(transparent)]
pub struct Apb2otpBlk1W7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk1W7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk1W7 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK1_W7**
    ///
    /// Otp block1 word7 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK1_W7_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK1_W7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK1_W7** `[31:0]` (RO)
    ///
    /// Otp block1 word7 data.
    #[inline(always)]
    pub fn apb2otp_block1_w7(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK1_W7_MASK) >> Self::APB2OTP_BLOCK1_W7_OFFSET
    }
}

/// APB2OTP_BLK1_W7 value type for batch read/modify/write operations
///
/// Use `apb2otpblk1w7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk1w7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk1W7Val {
    bits: u32,
}

impl Apb2otpBlk1W7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK1_W7** `[31:0]`
    ///
    /// Otp block1 word7 data.
    #[inline(always)]
    pub const fn apb2otp_block1_w7(&self) -> u32 {
        (self.bits & Apb2otpBlk1W7::APB2OTP_BLOCK1_W7_MASK)
            >> Apb2otpBlk1W7::APB2OTP_BLOCK1_W7_OFFSET
    }
}

impl Default for Apb2otpBlk1W7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk1W7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk1W7Val {
        Apb2otpBlk1W7Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK1_W8 Register
///
/// eFuse apb2otp block1 data register8.
///
/// Address offset: 0x0870
///
/// ## Fields
///
/// - **APB2OTP_BLOCK1_W8** `[31:0]` (RO)
///
///   Otp block1 word8 data.
#[repr(transparent)]
pub struct Apb2otpBlk1W8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk1W8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk1W8 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK1_W8**
    ///
    /// Otp block1 word8 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK1_W8_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK1_W8_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK1_W8** `[31:0]` (RO)
    ///
    /// Otp block1 word8 data.
    #[inline(always)]
    pub fn apb2otp_block1_w8(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK1_W8_MASK) >> Self::APB2OTP_BLOCK1_W8_OFFSET
    }
}

/// APB2OTP_BLK1_W8 value type for batch read/modify/write operations
///
/// Use `apb2otpblk1w8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk1w8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk1W8Val {
    bits: u32,
}

impl Apb2otpBlk1W8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK1_W8** `[31:0]`
    ///
    /// Otp block1 word8 data.
    #[inline(always)]
    pub const fn apb2otp_block1_w8(&self) -> u32 {
        (self.bits & Apb2otpBlk1W8::APB2OTP_BLOCK1_W8_MASK)
            >> Apb2otpBlk1W8::APB2OTP_BLOCK1_W8_OFFSET
    }
}

impl Default for Apb2otpBlk1W8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk1W8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk1W8Val {
        Apb2otpBlk1W8Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK1_W9 Register
///
/// eFuse apb2otp block1 data register9.
///
/// Address offset: 0x0874
///
/// ## Fields
///
/// - **APB2OTP_BLOCK1_W9** `[31:0]` (RO)
///
///   Otp block1 word9 data.
#[repr(transparent)]
pub struct Apb2otpBlk1W9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk1W9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk1W9 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK1_W9**
    ///
    /// Otp block1 word9 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK1_W9_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK1_W9_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK1_W9** `[31:0]` (RO)
    ///
    /// Otp block1 word9 data.
    #[inline(always)]
    pub fn apb2otp_block1_w9(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK1_W9_MASK) >> Self::APB2OTP_BLOCK1_W9_OFFSET
    }
}

/// APB2OTP_BLK1_W9 value type for batch read/modify/write operations
///
/// Use `apb2otpblk1w9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk1w9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk1W9Val {
    bits: u32,
}

impl Apb2otpBlk1W9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK1_W9** `[31:0]`
    ///
    /// Otp block1 word9 data.
    #[inline(always)]
    pub const fn apb2otp_block1_w9(&self) -> u32 {
        (self.bits & Apb2otpBlk1W9::APB2OTP_BLOCK1_W9_MASK)
            >> Apb2otpBlk1W9::APB2OTP_BLOCK1_W9_OFFSET
    }
}

impl Default for Apb2otpBlk1W9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk1W9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk1W9Val {
        Apb2otpBlk1W9Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK2_W1 Register
///
/// eFuse apb2otp block2 data register1.
///
/// Address offset: 0x0878
///
/// ## Fields
///
/// - **APB2OTP_BLOCK2_W1** `[31:0]` (RO)
///
///   Otp block2 word1 data.
#[repr(transparent)]
pub struct Apb2otpBlk2W1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk2W1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk2W1 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK2_W1**
    ///
    /// Otp block2 word1 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK2_W1_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK2_W1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK2_W1** `[31:0]` (RO)
    ///
    /// Otp block2 word1 data.
    #[inline(always)]
    pub fn apb2otp_block2_w1(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK2_W1_MASK) >> Self::APB2OTP_BLOCK2_W1_OFFSET
    }
}

/// APB2OTP_BLK2_W1 value type for batch read/modify/write operations
///
/// Use `apb2otpblk2w1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk2w1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk2W1Val {
    bits: u32,
}

impl Apb2otpBlk2W1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK2_W1** `[31:0]`
    ///
    /// Otp block2 word1 data.
    #[inline(always)]
    pub const fn apb2otp_block2_w1(&self) -> u32 {
        (self.bits & Apb2otpBlk2W1::APB2OTP_BLOCK2_W1_MASK)
            >> Apb2otpBlk2W1::APB2OTP_BLOCK2_W1_OFFSET
    }
}

impl Default for Apb2otpBlk2W1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk2W1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk2W1Val {
        Apb2otpBlk2W1Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK2_W2 Register
///
/// eFuse apb2otp block2 data register2.
///
/// Address offset: 0x087c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK2_W2** `[31:0]` (RO)
///
///   Otp block2 word2 data.
#[repr(transparent)]
pub struct Apb2otpBlk2W2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk2W2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk2W2 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK2_W2**
    ///
    /// Otp block2 word2 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK2_W2_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK2_W2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK2_W2** `[31:0]` (RO)
    ///
    /// Otp block2 word2 data.
    #[inline(always)]
    pub fn apb2otp_block2_w2(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK2_W2_MASK) >> Self::APB2OTP_BLOCK2_W2_OFFSET
    }
}

/// APB2OTP_BLK2_W2 value type for batch read/modify/write operations
///
/// Use `apb2otpblk2w2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk2w2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk2W2Val {
    bits: u32,
}

impl Apb2otpBlk2W2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK2_W2** `[31:0]`
    ///
    /// Otp block2 word2 data.
    #[inline(always)]
    pub const fn apb2otp_block2_w2(&self) -> u32 {
        (self.bits & Apb2otpBlk2W2::APB2OTP_BLOCK2_W2_MASK)
            >> Apb2otpBlk2W2::APB2OTP_BLOCK2_W2_OFFSET
    }
}

impl Default for Apb2otpBlk2W2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk2W2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk2W2Val {
        Apb2otpBlk2W2Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK2_W3 Register
///
/// eFuse apb2otp block2 data register3.
///
/// Address offset: 0x0880
///
/// ## Fields
///
/// - **APB2OTP_BLOCK2_W3** `[31:0]` (RO)
///
///   Otp block2 word3 data.
#[repr(transparent)]
pub struct Apb2otpBlk2W3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk2W3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk2W3 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK2_W3**
    ///
    /// Otp block2 word3 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK2_W3_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK2_W3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK2_W3** `[31:0]` (RO)
    ///
    /// Otp block2 word3 data.
    #[inline(always)]
    pub fn apb2otp_block2_w3(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK2_W3_MASK) >> Self::APB2OTP_BLOCK2_W3_OFFSET
    }
}

/// APB2OTP_BLK2_W3 value type for batch read/modify/write operations
///
/// Use `apb2otpblk2w3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk2w3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk2W3Val {
    bits: u32,
}

impl Apb2otpBlk2W3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK2_W3** `[31:0]`
    ///
    /// Otp block2 word3 data.
    #[inline(always)]
    pub const fn apb2otp_block2_w3(&self) -> u32 {
        (self.bits & Apb2otpBlk2W3::APB2OTP_BLOCK2_W3_MASK)
            >> Apb2otpBlk2W3::APB2OTP_BLOCK2_W3_OFFSET
    }
}

impl Default for Apb2otpBlk2W3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk2W3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk2W3Val {
        Apb2otpBlk2W3Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK2_W4 Register
///
/// eFuse apb2otp block2 data register4.
///
/// Address offset: 0x0884
///
/// ## Fields
///
/// - **APB2OTP_BLOCK2_W4** `[31:0]` (RO)
///
///   Otp block2 word4 data.
#[repr(transparent)]
pub struct Apb2otpBlk2W4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk2W4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk2W4 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK2_W4**
    ///
    /// Otp block2 word4 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK2_W4_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK2_W4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK2_W4** `[31:0]` (RO)
    ///
    /// Otp block2 word4 data.
    #[inline(always)]
    pub fn apb2otp_block2_w4(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK2_W4_MASK) >> Self::APB2OTP_BLOCK2_W4_OFFSET
    }
}

/// APB2OTP_BLK2_W4 value type for batch read/modify/write operations
///
/// Use `apb2otpblk2w4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk2w4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk2W4Val {
    bits: u32,
}

impl Apb2otpBlk2W4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK2_W4** `[31:0]`
    ///
    /// Otp block2 word4 data.
    #[inline(always)]
    pub const fn apb2otp_block2_w4(&self) -> u32 {
        (self.bits & Apb2otpBlk2W4::APB2OTP_BLOCK2_W4_MASK)
            >> Apb2otpBlk2W4::APB2OTP_BLOCK2_W4_OFFSET
    }
}

impl Default for Apb2otpBlk2W4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk2W4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk2W4Val {
        Apb2otpBlk2W4Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK2_W5 Register
///
/// eFuse apb2otp block2 data register5.
///
/// Address offset: 0x0888
///
/// ## Fields
///
/// - **APB2OTP_BLOCK2_W5** `[31:0]` (RO)
///
///   Otp block2 word5 data.
#[repr(transparent)]
pub struct Apb2otpBlk2W5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk2W5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk2W5 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK2_W5**
    ///
    /// Otp block2 word5 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK2_W5_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK2_W5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK2_W5** `[31:0]` (RO)
    ///
    /// Otp block2 word5 data.
    #[inline(always)]
    pub fn apb2otp_block2_w5(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK2_W5_MASK) >> Self::APB2OTP_BLOCK2_W5_OFFSET
    }
}

/// APB2OTP_BLK2_W5 value type for batch read/modify/write operations
///
/// Use `apb2otpblk2w5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk2w5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk2W5Val {
    bits: u32,
}

impl Apb2otpBlk2W5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK2_W5** `[31:0]`
    ///
    /// Otp block2 word5 data.
    #[inline(always)]
    pub const fn apb2otp_block2_w5(&self) -> u32 {
        (self.bits & Apb2otpBlk2W5::APB2OTP_BLOCK2_W5_MASK)
            >> Apb2otpBlk2W5::APB2OTP_BLOCK2_W5_OFFSET
    }
}

impl Default for Apb2otpBlk2W5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk2W5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk2W5Val {
        Apb2otpBlk2W5Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK2_W6 Register
///
/// eFuse apb2otp block2 data register6.
///
/// Address offset: 0x088c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK2_W6** `[31:0]` (RO)
///
///   Otp block2 word6 data.
#[repr(transparent)]
pub struct Apb2otpBlk2W6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk2W6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk2W6 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK2_W6**
    ///
    /// Otp block2 word6 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK2_W6_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK2_W6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK2_W6** `[31:0]` (RO)
    ///
    /// Otp block2 word6 data.
    #[inline(always)]
    pub fn apb2otp_block2_w6(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK2_W6_MASK) >> Self::APB2OTP_BLOCK2_W6_OFFSET
    }
}

/// APB2OTP_BLK2_W6 value type for batch read/modify/write operations
///
/// Use `apb2otpblk2w6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk2w6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk2W6Val {
    bits: u32,
}

impl Apb2otpBlk2W6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK2_W6** `[31:0]`
    ///
    /// Otp block2 word6 data.
    #[inline(always)]
    pub const fn apb2otp_block2_w6(&self) -> u32 {
        (self.bits & Apb2otpBlk2W6::APB2OTP_BLOCK2_W6_MASK)
            >> Apb2otpBlk2W6::APB2OTP_BLOCK2_W6_OFFSET
    }
}

impl Default for Apb2otpBlk2W6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk2W6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk2W6Val {
        Apb2otpBlk2W6Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK2_W7 Register
///
/// eFuse apb2otp block2 data register7.
///
/// Address offset: 0x0890
///
/// ## Fields
///
/// - **APB2OTP_BLOCK2_W7** `[31:0]` (RO)
///
///   Otp block2 word7 data.
#[repr(transparent)]
pub struct Apb2otpBlk2W7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk2W7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk2W7 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK2_W7**
    ///
    /// Otp block2 word7 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK2_W7_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK2_W7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK2_W7** `[31:0]` (RO)
    ///
    /// Otp block2 word7 data.
    #[inline(always)]
    pub fn apb2otp_block2_w7(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK2_W7_MASK) >> Self::APB2OTP_BLOCK2_W7_OFFSET
    }
}

/// APB2OTP_BLK2_W7 value type for batch read/modify/write operations
///
/// Use `apb2otpblk2w7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk2w7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk2W7Val {
    bits: u32,
}

impl Apb2otpBlk2W7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK2_W7** `[31:0]`
    ///
    /// Otp block2 word7 data.
    #[inline(always)]
    pub const fn apb2otp_block2_w7(&self) -> u32 {
        (self.bits & Apb2otpBlk2W7::APB2OTP_BLOCK2_W7_MASK)
            >> Apb2otpBlk2W7::APB2OTP_BLOCK2_W7_OFFSET
    }
}

impl Default for Apb2otpBlk2W7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk2W7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk2W7Val {
        Apb2otpBlk2W7Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK2_W8 Register
///
/// eFuse apb2otp block2 data register8.
///
/// Address offset: 0x0894
///
/// ## Fields
///
/// - **APB2OTP_BLOCK2_W8** `[31:0]` (RO)
///
///   Otp block2 word8 data.
#[repr(transparent)]
pub struct Apb2otpBlk2W8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk2W8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk2W8 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK2_W8**
    ///
    /// Otp block2 word8 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK2_W8_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK2_W8_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK2_W8** `[31:0]` (RO)
    ///
    /// Otp block2 word8 data.
    #[inline(always)]
    pub fn apb2otp_block2_w8(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK2_W8_MASK) >> Self::APB2OTP_BLOCK2_W8_OFFSET
    }
}

/// APB2OTP_BLK2_W8 value type for batch read/modify/write operations
///
/// Use `apb2otpblk2w8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk2w8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk2W8Val {
    bits: u32,
}

impl Apb2otpBlk2W8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK2_W8** `[31:0]`
    ///
    /// Otp block2 word8 data.
    #[inline(always)]
    pub const fn apb2otp_block2_w8(&self) -> u32 {
        (self.bits & Apb2otpBlk2W8::APB2OTP_BLOCK2_W8_MASK)
            >> Apb2otpBlk2W8::APB2OTP_BLOCK2_W8_OFFSET
    }
}

impl Default for Apb2otpBlk2W8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk2W8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk2W8Val {
        Apb2otpBlk2W8Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK2_W9 Register
///
/// eFuse apb2otp block2 data register9.
///
/// Address offset: 0x0898
///
/// ## Fields
///
/// - **APB2OTP_BLOCK2_W9** `[31:0]` (RO)
///
///   Otp block2 word9 data.
#[repr(transparent)]
pub struct Apb2otpBlk2W9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk2W9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk2W9 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK2_W9**
    ///
    /// Otp block2 word9 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK2_W9_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK2_W9_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK2_W9** `[31:0]` (RO)
    ///
    /// Otp block2 word9 data.
    #[inline(always)]
    pub fn apb2otp_block2_w9(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK2_W9_MASK) >> Self::APB2OTP_BLOCK2_W9_OFFSET
    }
}

/// APB2OTP_BLK2_W9 value type for batch read/modify/write operations
///
/// Use `apb2otpblk2w9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk2w9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk2W9Val {
    bits: u32,
}

impl Apb2otpBlk2W9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK2_W9** `[31:0]`
    ///
    /// Otp block2 word9 data.
    #[inline(always)]
    pub const fn apb2otp_block2_w9(&self) -> u32 {
        (self.bits & Apb2otpBlk2W9::APB2OTP_BLOCK2_W9_MASK)
            >> Apb2otpBlk2W9::APB2OTP_BLOCK2_W9_OFFSET
    }
}

impl Default for Apb2otpBlk2W9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk2W9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk2W9Val {
        Apb2otpBlk2W9Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK2_W10 Register
///
/// eFuse apb2otp block2 data register10.
///
/// Address offset: 0x089c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK2_W10** `[31:0]` (RO)
///
///   Otp block2 word10 data.
#[repr(transparent)]
pub struct Apb2otpBlk2W10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk2W10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk2W10 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK2_W10**
    ///
    /// Otp block2 word10 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK2_W10_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK2_W10_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK2_W10** `[31:0]` (RO)
    ///
    /// Otp block2 word10 data.
    #[inline(always)]
    pub fn apb2otp_block2_w10(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK2_W10_MASK) >> Self::APB2OTP_BLOCK2_W10_OFFSET
    }
}

/// APB2OTP_BLK2_W10 value type for batch read/modify/write operations
///
/// Use `apb2otpblk2w10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk2w10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk2W10Val {
    bits: u32,
}

impl Apb2otpBlk2W10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK2_W10** `[31:0]`
    ///
    /// Otp block2 word10 data.
    #[inline(always)]
    pub const fn apb2otp_block2_w10(&self) -> u32 {
        (self.bits & Apb2otpBlk2W10::APB2OTP_BLOCK2_W10_MASK)
            >> Apb2otpBlk2W10::APB2OTP_BLOCK2_W10_OFFSET
    }
}

impl Default for Apb2otpBlk2W10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk2W10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk2W10Val {
        Apb2otpBlk2W10Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK2_W11 Register
///
/// eFuse apb2otp block2 data register11.
///
/// Address offset: 0x08a0
///
/// ## Fields
///
/// - **APB2OTP_BLOCK2_W11** `[31:0]` (RO)
///
///   Otp block2 word11 data.
#[repr(transparent)]
pub struct Apb2otpBlk2W11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk2W11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk2W11 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK2_W11**
    ///
    /// Otp block2 word11 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK2_W11_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK2_W11_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK2_W11** `[31:0]` (RO)
    ///
    /// Otp block2 word11 data.
    #[inline(always)]
    pub fn apb2otp_block2_w11(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK2_W11_MASK) >> Self::APB2OTP_BLOCK2_W11_OFFSET
    }
}

/// APB2OTP_BLK2_W11 value type for batch read/modify/write operations
///
/// Use `apb2otpblk2w11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk2w11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk2W11Val {
    bits: u32,
}

impl Apb2otpBlk2W11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK2_W11** `[31:0]`
    ///
    /// Otp block2 word11 data.
    #[inline(always)]
    pub const fn apb2otp_block2_w11(&self) -> u32 {
        (self.bits & Apb2otpBlk2W11::APB2OTP_BLOCK2_W11_MASK)
            >> Apb2otpBlk2W11::APB2OTP_BLOCK2_W11_OFFSET
    }
}

impl Default for Apb2otpBlk2W11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk2W11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk2W11Val {
        Apb2otpBlk2W11Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK3_W1 Register
///
/// eFuse apb2otp block3 data register1.
///
/// Address offset: 0x08a4
///
/// ## Fields
///
/// - **APB2OTP_BLOCK3_W1** `[31:0]` (RO)
///
///   Otp block3 word1 data.
#[repr(transparent)]
pub struct Apb2otpBlk3W1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk3W1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk3W1 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK3_W1**
    ///
    /// Otp block3 word1 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK3_W1_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK3_W1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK3_W1** `[31:0]` (RO)
    ///
    /// Otp block3 word1 data.
    #[inline(always)]
    pub fn apb2otp_block3_w1(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK3_W1_MASK) >> Self::APB2OTP_BLOCK3_W1_OFFSET
    }
}

/// APB2OTP_BLK3_W1 value type for batch read/modify/write operations
///
/// Use `apb2otpblk3w1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk3w1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk3W1Val {
    bits: u32,
}

impl Apb2otpBlk3W1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK3_W1** `[31:0]`
    ///
    /// Otp block3 word1 data.
    #[inline(always)]
    pub const fn apb2otp_block3_w1(&self) -> u32 {
        (self.bits & Apb2otpBlk3W1::APB2OTP_BLOCK3_W1_MASK)
            >> Apb2otpBlk3W1::APB2OTP_BLOCK3_W1_OFFSET
    }
}

impl Default for Apb2otpBlk3W1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk3W1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk3W1Val {
        Apb2otpBlk3W1Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK3_W2 Register
///
/// eFuse apb2otp block3 data register2.
///
/// Address offset: 0x08a8
///
/// ## Fields
///
/// - **APB2OTP_BLOCK3_W2** `[31:0]` (RO)
///
///   Otp block3 word2 data.
#[repr(transparent)]
pub struct Apb2otpBlk3W2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk3W2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk3W2 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK3_W2**
    ///
    /// Otp block3 word2 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK3_W2_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK3_W2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK3_W2** `[31:0]` (RO)
    ///
    /// Otp block3 word2 data.
    #[inline(always)]
    pub fn apb2otp_block3_w2(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK3_W2_MASK) >> Self::APB2OTP_BLOCK3_W2_OFFSET
    }
}

/// APB2OTP_BLK3_W2 value type for batch read/modify/write operations
///
/// Use `apb2otpblk3w2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk3w2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk3W2Val {
    bits: u32,
}

impl Apb2otpBlk3W2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK3_W2** `[31:0]`
    ///
    /// Otp block3 word2 data.
    #[inline(always)]
    pub const fn apb2otp_block3_w2(&self) -> u32 {
        (self.bits & Apb2otpBlk3W2::APB2OTP_BLOCK3_W2_MASK)
            >> Apb2otpBlk3W2::APB2OTP_BLOCK3_W2_OFFSET
    }
}

impl Default for Apb2otpBlk3W2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk3W2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk3W2Val {
        Apb2otpBlk3W2Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK3_W3 Register
///
/// eFuse apb2otp block3 data register3.
///
/// Address offset: 0x08ac
///
/// ## Fields
///
/// - **APB2OTP_BLOCK3_W3** `[31:0]` (RO)
///
///   Otp block3 word3 data.
#[repr(transparent)]
pub struct Apb2otpBlk3W3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk3W3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk3W3 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK3_W3**
    ///
    /// Otp block3 word3 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK3_W3_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK3_W3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK3_W3** `[31:0]` (RO)
    ///
    /// Otp block3 word3 data.
    #[inline(always)]
    pub fn apb2otp_block3_w3(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK3_W3_MASK) >> Self::APB2OTP_BLOCK3_W3_OFFSET
    }
}

/// APB2OTP_BLK3_W3 value type for batch read/modify/write operations
///
/// Use `apb2otpblk3w3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk3w3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk3W3Val {
    bits: u32,
}

impl Apb2otpBlk3W3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK3_W3** `[31:0]`
    ///
    /// Otp block3 word3 data.
    #[inline(always)]
    pub const fn apb2otp_block3_w3(&self) -> u32 {
        (self.bits & Apb2otpBlk3W3::APB2OTP_BLOCK3_W3_MASK)
            >> Apb2otpBlk3W3::APB2OTP_BLOCK3_W3_OFFSET
    }
}

impl Default for Apb2otpBlk3W3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk3W3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk3W3Val {
        Apb2otpBlk3W3Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK3_W4 Register
///
/// eFuse apb2otp block3 data register4.
///
/// Address offset: 0x08b0
///
/// ## Fields
///
/// - **APB2OTP_BLOCK3_W4** `[31:0]` (RO)
///
///   Otp block3 word4 data.
#[repr(transparent)]
pub struct Apb2otpBlk3W4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk3W4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk3W4 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK3_W4**
    ///
    /// Otp block3 word4 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK3_W4_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK3_W4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK3_W4** `[31:0]` (RO)
    ///
    /// Otp block3 word4 data.
    #[inline(always)]
    pub fn apb2otp_block3_w4(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK3_W4_MASK) >> Self::APB2OTP_BLOCK3_W4_OFFSET
    }
}

/// APB2OTP_BLK3_W4 value type for batch read/modify/write operations
///
/// Use `apb2otpblk3w4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk3w4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk3W4Val {
    bits: u32,
}

impl Apb2otpBlk3W4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK3_W4** `[31:0]`
    ///
    /// Otp block3 word4 data.
    #[inline(always)]
    pub const fn apb2otp_block3_w4(&self) -> u32 {
        (self.bits & Apb2otpBlk3W4::APB2OTP_BLOCK3_W4_MASK)
            >> Apb2otpBlk3W4::APB2OTP_BLOCK3_W4_OFFSET
    }
}

impl Default for Apb2otpBlk3W4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk3W4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk3W4Val {
        Apb2otpBlk3W4Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK3_W5 Register
///
/// eFuse apb2otp block3 data register5.
///
/// Address offset: 0x08b4
///
/// ## Fields
///
/// - **APB2OTP_BLOCK3_W5** `[31:0]` (RO)
///
///   Otp block3 word5 data.
#[repr(transparent)]
pub struct Apb2otpBlk3W5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk3W5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk3W5 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK3_W5**
    ///
    /// Otp block3 word5 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK3_W5_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK3_W5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK3_W5** `[31:0]` (RO)
    ///
    /// Otp block3 word5 data.
    #[inline(always)]
    pub fn apb2otp_block3_w5(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK3_W5_MASK) >> Self::APB2OTP_BLOCK3_W5_OFFSET
    }
}

/// APB2OTP_BLK3_W5 value type for batch read/modify/write operations
///
/// Use `apb2otpblk3w5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk3w5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk3W5Val {
    bits: u32,
}

impl Apb2otpBlk3W5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK3_W5** `[31:0]`
    ///
    /// Otp block3 word5 data.
    #[inline(always)]
    pub const fn apb2otp_block3_w5(&self) -> u32 {
        (self.bits & Apb2otpBlk3W5::APB2OTP_BLOCK3_W5_MASK)
            >> Apb2otpBlk3W5::APB2OTP_BLOCK3_W5_OFFSET
    }
}

impl Default for Apb2otpBlk3W5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk3W5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk3W5Val {
        Apb2otpBlk3W5Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK3_W6 Register
///
/// eFuse apb2otp block3 data register6.
///
/// Address offset: 0x08b8
///
/// ## Fields
///
/// - **APB2OTP_BLOCK3_W6** `[31:0]` (RO)
///
///   Otp block3 word6 data.
#[repr(transparent)]
pub struct Apb2otpBlk3W6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk3W6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk3W6 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK3_W6**
    ///
    /// Otp block3 word6 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK3_W6_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK3_W6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK3_W6** `[31:0]` (RO)
    ///
    /// Otp block3 word6 data.
    #[inline(always)]
    pub fn apb2otp_block3_w6(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK3_W6_MASK) >> Self::APB2OTP_BLOCK3_W6_OFFSET
    }
}

/// APB2OTP_BLK3_W6 value type for batch read/modify/write operations
///
/// Use `apb2otpblk3w6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk3w6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk3W6Val {
    bits: u32,
}

impl Apb2otpBlk3W6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK3_W6** `[31:0]`
    ///
    /// Otp block3 word6 data.
    #[inline(always)]
    pub const fn apb2otp_block3_w6(&self) -> u32 {
        (self.bits & Apb2otpBlk3W6::APB2OTP_BLOCK3_W6_MASK)
            >> Apb2otpBlk3W6::APB2OTP_BLOCK3_W6_OFFSET
    }
}

impl Default for Apb2otpBlk3W6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk3W6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk3W6Val {
        Apb2otpBlk3W6Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK3_W7 Register
///
/// eFuse apb2otp block3 data register7.
///
/// Address offset: 0x08bc
///
/// ## Fields
///
/// - **APB2OTP_BLOCK3_W7** `[31:0]` (RO)
///
///   Otp block3 word7 data.
#[repr(transparent)]
pub struct Apb2otpBlk3W7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk3W7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk3W7 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK3_W7**
    ///
    /// Otp block3 word7 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK3_W7_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK3_W7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK3_W7** `[31:0]` (RO)
    ///
    /// Otp block3 word7 data.
    #[inline(always)]
    pub fn apb2otp_block3_w7(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK3_W7_MASK) >> Self::APB2OTP_BLOCK3_W7_OFFSET
    }
}

/// APB2OTP_BLK3_W7 value type for batch read/modify/write operations
///
/// Use `apb2otpblk3w7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk3w7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk3W7Val {
    bits: u32,
}

impl Apb2otpBlk3W7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK3_W7** `[31:0]`
    ///
    /// Otp block3 word7 data.
    #[inline(always)]
    pub const fn apb2otp_block3_w7(&self) -> u32 {
        (self.bits & Apb2otpBlk3W7::APB2OTP_BLOCK3_W7_MASK)
            >> Apb2otpBlk3W7::APB2OTP_BLOCK3_W7_OFFSET
    }
}

impl Default for Apb2otpBlk3W7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk3W7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk3W7Val {
        Apb2otpBlk3W7Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK3_W8 Register
///
/// eFuse apb2otp block3 data register8.
///
/// Address offset: 0x08c0
///
/// ## Fields
///
/// - **APB2OTP_BLOCK3_W8** `[31:0]` (RO)
///
///   Otp block3 word8 data.
#[repr(transparent)]
pub struct Apb2otpBlk3W8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk3W8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk3W8 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK3_W8**
    ///
    /// Otp block3 word8 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK3_W8_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK3_W8_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK3_W8** `[31:0]` (RO)
    ///
    /// Otp block3 word8 data.
    #[inline(always)]
    pub fn apb2otp_block3_w8(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK3_W8_MASK) >> Self::APB2OTP_BLOCK3_W8_OFFSET
    }
}

/// APB2OTP_BLK3_W8 value type for batch read/modify/write operations
///
/// Use `apb2otpblk3w8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk3w8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk3W8Val {
    bits: u32,
}

impl Apb2otpBlk3W8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK3_W8** `[31:0]`
    ///
    /// Otp block3 word8 data.
    #[inline(always)]
    pub const fn apb2otp_block3_w8(&self) -> u32 {
        (self.bits & Apb2otpBlk3W8::APB2OTP_BLOCK3_W8_MASK)
            >> Apb2otpBlk3W8::APB2OTP_BLOCK3_W8_OFFSET
    }
}

impl Default for Apb2otpBlk3W8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk3W8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk3W8Val {
        Apb2otpBlk3W8Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK3_W9 Register
///
/// eFuse apb2otp block3 data register9.
///
/// Address offset: 0x08c4
///
/// ## Fields
///
/// - **APB2OTP_BLOCK3_W9** `[31:0]` (RO)
///
///   Otp block3 word9 data.
#[repr(transparent)]
pub struct Apb2otpBlk3W9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk3W9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk3W9 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK3_W9**
    ///
    /// Otp block3 word9 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK3_W9_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK3_W9_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK3_W9** `[31:0]` (RO)
    ///
    /// Otp block3 word9 data.
    #[inline(always)]
    pub fn apb2otp_block3_w9(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK3_W9_MASK) >> Self::APB2OTP_BLOCK3_W9_OFFSET
    }
}

/// APB2OTP_BLK3_W9 value type for batch read/modify/write operations
///
/// Use `apb2otpblk3w9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk3w9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk3W9Val {
    bits: u32,
}

impl Apb2otpBlk3W9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK3_W9** `[31:0]`
    ///
    /// Otp block3 word9 data.
    #[inline(always)]
    pub const fn apb2otp_block3_w9(&self) -> u32 {
        (self.bits & Apb2otpBlk3W9::APB2OTP_BLOCK3_W9_MASK)
            >> Apb2otpBlk3W9::APB2OTP_BLOCK3_W9_OFFSET
    }
}

impl Default for Apb2otpBlk3W9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk3W9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk3W9Val {
        Apb2otpBlk3W9Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK3_W10 Register
///
/// eFuse apb2otp block3 data register10.
///
/// Address offset: 0x08c8
///
/// ## Fields
///
/// - **APB2OTP_BLOCK3_W10** `[31:0]` (RO)
///
///   Otp block3 word10 data.
#[repr(transparent)]
pub struct Apb2otpBlk3W10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk3W10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk3W10 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK3_W10**
    ///
    /// Otp block3 word10 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK3_W10_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK3_W10_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK3_W10** `[31:0]` (RO)
    ///
    /// Otp block3 word10 data.
    #[inline(always)]
    pub fn apb2otp_block3_w10(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK3_W10_MASK) >> Self::APB2OTP_BLOCK3_W10_OFFSET
    }
}

/// APB2OTP_BLK3_W10 value type for batch read/modify/write operations
///
/// Use `apb2otpblk3w10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk3w10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk3W10Val {
    bits: u32,
}

impl Apb2otpBlk3W10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK3_W10** `[31:0]`
    ///
    /// Otp block3 word10 data.
    #[inline(always)]
    pub const fn apb2otp_block3_w10(&self) -> u32 {
        (self.bits & Apb2otpBlk3W10::APB2OTP_BLOCK3_W10_MASK)
            >> Apb2otpBlk3W10::APB2OTP_BLOCK3_W10_OFFSET
    }
}

impl Default for Apb2otpBlk3W10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk3W10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk3W10Val {
        Apb2otpBlk3W10Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK3_W11 Register
///
/// eFuse apb2otp block3 data register11.
///
/// Address offset: 0x08cc
///
/// ## Fields
///
/// - **APB2OTP_BLOCK3_W11** `[31:0]` (RO)
///
///   Otp block3 word11 data.
#[repr(transparent)]
pub struct Apb2otpBlk3W11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk3W11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk3W11 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK3_W11**
    ///
    /// Otp block3 word11 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK3_W11_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK3_W11_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK3_W11** `[31:0]` (RO)
    ///
    /// Otp block3 word11 data.
    #[inline(always)]
    pub fn apb2otp_block3_w11(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK3_W11_MASK) >> Self::APB2OTP_BLOCK3_W11_OFFSET
    }
}

/// APB2OTP_BLK3_W11 value type for batch read/modify/write operations
///
/// Use `apb2otpblk3w11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk3w11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk3W11Val {
    bits: u32,
}

impl Apb2otpBlk3W11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK3_W11** `[31:0]`
    ///
    /// Otp block3 word11 data.
    #[inline(always)]
    pub const fn apb2otp_block3_w11(&self) -> u32 {
        (self.bits & Apb2otpBlk3W11::APB2OTP_BLOCK3_W11_MASK)
            >> Apb2otpBlk3W11::APB2OTP_BLOCK3_W11_OFFSET
    }
}

impl Default for Apb2otpBlk3W11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk3W11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk3W11Val {
        Apb2otpBlk3W11Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK4_W1 Register
///
/// eFuse apb2otp block4 data register1.
///
/// Address offset: 0x08d0
///
/// ## Fields
///
/// - **APB2OTP_BLOCK4_W1** `[31:0]` (RO)
///
///   Otp block4 word1 data.
#[repr(transparent)]
pub struct Apb2otpBlk4W1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk4W1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk4W1 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK4_W1**
    ///
    /// Otp block4 word1 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK4_W1_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK4_W1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK4_W1** `[31:0]` (RO)
    ///
    /// Otp block4 word1 data.
    #[inline(always)]
    pub fn apb2otp_block4_w1(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK4_W1_MASK) >> Self::APB2OTP_BLOCK4_W1_OFFSET
    }
}

/// APB2OTP_BLK4_W1 value type for batch read/modify/write operations
///
/// Use `apb2otpblk4w1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk4w1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk4W1Val {
    bits: u32,
}

impl Apb2otpBlk4W1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK4_W1** `[31:0]`
    ///
    /// Otp block4 word1 data.
    #[inline(always)]
    pub const fn apb2otp_block4_w1(&self) -> u32 {
        (self.bits & Apb2otpBlk4W1::APB2OTP_BLOCK4_W1_MASK)
            >> Apb2otpBlk4W1::APB2OTP_BLOCK4_W1_OFFSET
    }
}

impl Default for Apb2otpBlk4W1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk4W1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk4W1Val {
        Apb2otpBlk4W1Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK4_W2 Register
///
/// eFuse apb2otp block4 data register2.
///
/// Address offset: 0x08d4
///
/// ## Fields
///
/// - **APB2OTP_BLOCK4_W2** `[31:0]` (RO)
///
///   Otp block4 word2 data.
#[repr(transparent)]
pub struct Apb2otpBlk4W2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk4W2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk4W2 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK4_W2**
    ///
    /// Otp block4 word2 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK4_W2_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK4_W2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK4_W2** `[31:0]` (RO)
    ///
    /// Otp block4 word2 data.
    #[inline(always)]
    pub fn apb2otp_block4_w2(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK4_W2_MASK) >> Self::APB2OTP_BLOCK4_W2_OFFSET
    }
}

/// APB2OTP_BLK4_W2 value type for batch read/modify/write operations
///
/// Use `apb2otpblk4w2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk4w2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk4W2Val {
    bits: u32,
}

impl Apb2otpBlk4W2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK4_W2** `[31:0]`
    ///
    /// Otp block4 word2 data.
    #[inline(always)]
    pub const fn apb2otp_block4_w2(&self) -> u32 {
        (self.bits & Apb2otpBlk4W2::APB2OTP_BLOCK4_W2_MASK)
            >> Apb2otpBlk4W2::APB2OTP_BLOCK4_W2_OFFSET
    }
}

impl Default for Apb2otpBlk4W2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk4W2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk4W2Val {
        Apb2otpBlk4W2Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK4_W3 Register
///
/// eFuse apb2otp block4 data register3.
///
/// Address offset: 0x08d8
///
/// ## Fields
///
/// - **APB2OTP_BLOCK4_W3** `[31:0]` (RO)
///
///   Otp block4 word3 data.
#[repr(transparent)]
pub struct Apb2otpBlk4W3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk4W3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk4W3 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK4_W3**
    ///
    /// Otp block4 word3 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK4_W3_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK4_W3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK4_W3** `[31:0]` (RO)
    ///
    /// Otp block4 word3 data.
    #[inline(always)]
    pub fn apb2otp_block4_w3(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK4_W3_MASK) >> Self::APB2OTP_BLOCK4_W3_OFFSET
    }
}

/// APB2OTP_BLK4_W3 value type for batch read/modify/write operations
///
/// Use `apb2otpblk4w3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk4w3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk4W3Val {
    bits: u32,
}

impl Apb2otpBlk4W3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK4_W3** `[31:0]`
    ///
    /// Otp block4 word3 data.
    #[inline(always)]
    pub const fn apb2otp_block4_w3(&self) -> u32 {
        (self.bits & Apb2otpBlk4W3::APB2OTP_BLOCK4_W3_MASK)
            >> Apb2otpBlk4W3::APB2OTP_BLOCK4_W3_OFFSET
    }
}

impl Default for Apb2otpBlk4W3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk4W3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk4W3Val {
        Apb2otpBlk4W3Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK4_W4 Register
///
/// eFuse apb2otp block4 data register4.
///
/// Address offset: 0x08dc
///
/// ## Fields
///
/// - **APB2OTP_BLOCK4_W4** `[31:0]` (RO)
///
///   Otp block4 word4 data.
#[repr(transparent)]
pub struct Apb2otpBlk4W4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk4W4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk4W4 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK4_W4**
    ///
    /// Otp block4 word4 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK4_W4_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK4_W4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK4_W4** `[31:0]` (RO)
    ///
    /// Otp block4 word4 data.
    #[inline(always)]
    pub fn apb2otp_block4_w4(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK4_W4_MASK) >> Self::APB2OTP_BLOCK4_W4_OFFSET
    }
}

/// APB2OTP_BLK4_W4 value type for batch read/modify/write operations
///
/// Use `apb2otpblk4w4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk4w4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk4W4Val {
    bits: u32,
}

impl Apb2otpBlk4W4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK4_W4** `[31:0]`
    ///
    /// Otp block4 word4 data.
    #[inline(always)]
    pub const fn apb2otp_block4_w4(&self) -> u32 {
        (self.bits & Apb2otpBlk4W4::APB2OTP_BLOCK4_W4_MASK)
            >> Apb2otpBlk4W4::APB2OTP_BLOCK4_W4_OFFSET
    }
}

impl Default for Apb2otpBlk4W4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk4W4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk4W4Val {
        Apb2otpBlk4W4Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK4_W5 Register
///
/// eFuse apb2otp block4 data register5.
///
/// Address offset: 0x08e0
///
/// ## Fields
///
/// - **APB2OTP_BLOCK4_W5** `[31:0]` (RO)
///
///   Otp block4 word5 data.
#[repr(transparent)]
pub struct Apb2otpBlk4W5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk4W5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk4W5 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK4_W5**
    ///
    /// Otp block4 word5 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK4_W5_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK4_W5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK4_W5** `[31:0]` (RO)
    ///
    /// Otp block4 word5 data.
    #[inline(always)]
    pub fn apb2otp_block4_w5(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK4_W5_MASK) >> Self::APB2OTP_BLOCK4_W5_OFFSET
    }
}

/// APB2OTP_BLK4_W5 value type for batch read/modify/write operations
///
/// Use `apb2otpblk4w5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk4w5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk4W5Val {
    bits: u32,
}

impl Apb2otpBlk4W5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK4_W5** `[31:0]`
    ///
    /// Otp block4 word5 data.
    #[inline(always)]
    pub const fn apb2otp_block4_w5(&self) -> u32 {
        (self.bits & Apb2otpBlk4W5::APB2OTP_BLOCK4_W5_MASK)
            >> Apb2otpBlk4W5::APB2OTP_BLOCK4_W5_OFFSET
    }
}

impl Default for Apb2otpBlk4W5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk4W5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk4W5Val {
        Apb2otpBlk4W5Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK4_W6 Register
///
/// eFuse apb2otp block4 data register6.
///
/// Address offset: 0x08e4
///
/// ## Fields
///
/// - **APB2OTP_BLOCK4_W6** `[31:0]` (RO)
///
///   Otp block4 word6 data.
#[repr(transparent)]
pub struct Apb2otpBlk4W6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk4W6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk4W6 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK4_W6**
    ///
    /// Otp block4 word6 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK4_W6_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK4_W6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK4_W6** `[31:0]` (RO)
    ///
    /// Otp block4 word6 data.
    #[inline(always)]
    pub fn apb2otp_block4_w6(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK4_W6_MASK) >> Self::APB2OTP_BLOCK4_W6_OFFSET
    }
}

/// APB2OTP_BLK4_W6 value type for batch read/modify/write operations
///
/// Use `apb2otpblk4w6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk4w6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk4W6Val {
    bits: u32,
}

impl Apb2otpBlk4W6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK4_W6** `[31:0]`
    ///
    /// Otp block4 word6 data.
    #[inline(always)]
    pub const fn apb2otp_block4_w6(&self) -> u32 {
        (self.bits & Apb2otpBlk4W6::APB2OTP_BLOCK4_W6_MASK)
            >> Apb2otpBlk4W6::APB2OTP_BLOCK4_W6_OFFSET
    }
}

impl Default for Apb2otpBlk4W6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk4W6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk4W6Val {
        Apb2otpBlk4W6Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK4_W7 Register
///
/// eFuse apb2otp block4 data register7.
///
/// Address offset: 0x08e8
///
/// ## Fields
///
/// - **APB2OTP_BLOCK4_W7** `[31:0]` (RO)
///
///   Otp block4 word7 data.
#[repr(transparent)]
pub struct Apb2otpBlk4W7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk4W7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk4W7 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK4_W7**
    ///
    /// Otp block4 word7 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK4_W7_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK4_W7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK4_W7** `[31:0]` (RO)
    ///
    /// Otp block4 word7 data.
    #[inline(always)]
    pub fn apb2otp_block4_w7(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK4_W7_MASK) >> Self::APB2OTP_BLOCK4_W7_OFFSET
    }
}

/// APB2OTP_BLK4_W7 value type for batch read/modify/write operations
///
/// Use `apb2otpblk4w7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk4w7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk4W7Val {
    bits: u32,
}

impl Apb2otpBlk4W7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK4_W7** `[31:0]`
    ///
    /// Otp block4 word7 data.
    #[inline(always)]
    pub const fn apb2otp_block4_w7(&self) -> u32 {
        (self.bits & Apb2otpBlk4W7::APB2OTP_BLOCK4_W7_MASK)
            >> Apb2otpBlk4W7::APB2OTP_BLOCK4_W7_OFFSET
    }
}

impl Default for Apb2otpBlk4W7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk4W7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk4W7Val {
        Apb2otpBlk4W7Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK4_W8 Register
///
/// eFuse apb2otp block4 data register8.
///
/// Address offset: 0x08ec
///
/// ## Fields
///
/// - **APB2OTP_BLOCK4_W8** `[31:0]` (RO)
///
///   Otp block4 word8 data.
#[repr(transparent)]
pub struct Apb2otpBlk4W8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk4W8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk4W8 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK4_W8**
    ///
    /// Otp block4 word8 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK4_W8_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK4_W8_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK4_W8** `[31:0]` (RO)
    ///
    /// Otp block4 word8 data.
    #[inline(always)]
    pub fn apb2otp_block4_w8(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK4_W8_MASK) >> Self::APB2OTP_BLOCK4_W8_OFFSET
    }
}

/// APB2OTP_BLK4_W8 value type for batch read/modify/write operations
///
/// Use `apb2otpblk4w8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk4w8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk4W8Val {
    bits: u32,
}

impl Apb2otpBlk4W8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK4_W8** `[31:0]`
    ///
    /// Otp block4 word8 data.
    #[inline(always)]
    pub const fn apb2otp_block4_w8(&self) -> u32 {
        (self.bits & Apb2otpBlk4W8::APB2OTP_BLOCK4_W8_MASK)
            >> Apb2otpBlk4W8::APB2OTP_BLOCK4_W8_OFFSET
    }
}

impl Default for Apb2otpBlk4W8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk4W8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk4W8Val {
        Apb2otpBlk4W8Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK4_W9 Register
///
/// eFuse apb2otp block4 data register9.
///
/// Address offset: 0x08f0
///
/// ## Fields
///
/// - **APB2OTP_BLOCK4_W9** `[31:0]` (RO)
///
///   Otp block4 word9 data.
#[repr(transparent)]
pub struct Apb2otpBlk4W9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk4W9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk4W9 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK4_W9**
    ///
    /// Otp block4 word9 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK4_W9_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK4_W9_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK4_W9** `[31:0]` (RO)
    ///
    /// Otp block4 word9 data.
    #[inline(always)]
    pub fn apb2otp_block4_w9(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK4_W9_MASK) >> Self::APB2OTP_BLOCK4_W9_OFFSET
    }
}

/// APB2OTP_BLK4_W9 value type for batch read/modify/write operations
///
/// Use `apb2otpblk4w9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk4w9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk4W9Val {
    bits: u32,
}

impl Apb2otpBlk4W9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK4_W9** `[31:0]`
    ///
    /// Otp block4 word9 data.
    #[inline(always)]
    pub const fn apb2otp_block4_w9(&self) -> u32 {
        (self.bits & Apb2otpBlk4W9::APB2OTP_BLOCK4_W9_MASK)
            >> Apb2otpBlk4W9::APB2OTP_BLOCK4_W9_OFFSET
    }
}

impl Default for Apb2otpBlk4W9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk4W9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk4W9Val {
        Apb2otpBlk4W9Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK4_W10 Register
///
/// eFuse apb2otp block4 data registe10.
///
/// Address offset: 0x08f4
///
/// ## Fields
///
/// - **APB2OTP_BLOCK4_W10** `[31:0]` (RO)
///
///   Otp block4 word10 data.
#[repr(transparent)]
pub struct Apb2otpBlk4W10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk4W10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk4W10 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK4_W10**
    ///
    /// Otp block4 word10 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK4_W10_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK4_W10_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK4_W10** `[31:0]` (RO)
    ///
    /// Otp block4 word10 data.
    #[inline(always)]
    pub fn apb2otp_block4_w10(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK4_W10_MASK) >> Self::APB2OTP_BLOCK4_W10_OFFSET
    }
}

/// APB2OTP_BLK4_W10 value type for batch read/modify/write operations
///
/// Use `apb2otpblk4w10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk4w10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk4W10Val {
    bits: u32,
}

impl Apb2otpBlk4W10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK4_W10** `[31:0]`
    ///
    /// Otp block4 word10 data.
    #[inline(always)]
    pub const fn apb2otp_block4_w10(&self) -> u32 {
        (self.bits & Apb2otpBlk4W10::APB2OTP_BLOCK4_W10_MASK)
            >> Apb2otpBlk4W10::APB2OTP_BLOCK4_W10_OFFSET
    }
}

impl Default for Apb2otpBlk4W10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk4W10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk4W10Val {
        Apb2otpBlk4W10Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK4_W11 Register
///
/// eFuse apb2otp block4 data register11.
///
/// Address offset: 0x08f8
///
/// ## Fields
///
/// - **APB2OTP_BLOCK4_W11** `[31:0]` (RO)
///
///   Otp block4 word11 data.
#[repr(transparent)]
pub struct Apb2otpBlk4W11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk4W11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk4W11 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK4_W11**
    ///
    /// Otp block4 word11 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK4_W11_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK4_W11_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK4_W11** `[31:0]` (RO)
    ///
    /// Otp block4 word11 data.
    #[inline(always)]
    pub fn apb2otp_block4_w11(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK4_W11_MASK) >> Self::APB2OTP_BLOCK4_W11_OFFSET
    }
}

/// APB2OTP_BLK4_W11 value type for batch read/modify/write operations
///
/// Use `apb2otpblk4w11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk4w11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk4W11Val {
    bits: u32,
}

impl Apb2otpBlk4W11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK4_W11** `[31:0]`
    ///
    /// Otp block4 word11 data.
    #[inline(always)]
    pub const fn apb2otp_block4_w11(&self) -> u32 {
        (self.bits & Apb2otpBlk4W11::APB2OTP_BLOCK4_W11_MASK)
            >> Apb2otpBlk4W11::APB2OTP_BLOCK4_W11_OFFSET
    }
}

impl Default for Apb2otpBlk4W11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk4W11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk4W11Val {
        Apb2otpBlk4W11Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK5_W1 Register
///
/// eFuse apb2otp block5 data register1.
///
/// Address offset: 0x08fc
///
/// ## Fields
///
/// - **APB2OTP_BLOCK5_W1** `[31:0]` (RO)
///
///   Otp block5 word1 data.
#[repr(transparent)]
pub struct Apb2otpBlk5W1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk5W1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk5W1 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK5_W1**
    ///
    /// Otp block5 word1 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK5_W1_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK5_W1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK5_W1** `[31:0]` (RO)
    ///
    /// Otp block5 word1 data.
    #[inline(always)]
    pub fn apb2otp_block5_w1(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK5_W1_MASK) >> Self::APB2OTP_BLOCK5_W1_OFFSET
    }
}

/// APB2OTP_BLK5_W1 value type for batch read/modify/write operations
///
/// Use `apb2otpblk5w1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk5w1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk5W1Val {
    bits: u32,
}

impl Apb2otpBlk5W1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK5_W1** `[31:0]`
    ///
    /// Otp block5 word1 data.
    #[inline(always)]
    pub const fn apb2otp_block5_w1(&self) -> u32 {
        (self.bits & Apb2otpBlk5W1::APB2OTP_BLOCK5_W1_MASK)
            >> Apb2otpBlk5W1::APB2OTP_BLOCK5_W1_OFFSET
    }
}

impl Default for Apb2otpBlk5W1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk5W1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk5W1Val {
        Apb2otpBlk5W1Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK5_W2 Register
///
/// eFuse apb2otp block5 data register2.
///
/// Address offset: 0x0900
///
/// ## Fields
///
/// - **APB2OTP_BLOCK5_W2** `[31:0]` (RO)
///
///   Otp block5 word2 data.
#[repr(transparent)]
pub struct Apb2otpBlk5W2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk5W2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk5W2 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK5_W2**
    ///
    /// Otp block5 word2 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK5_W2_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK5_W2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK5_W2** `[31:0]` (RO)
    ///
    /// Otp block5 word2 data.
    #[inline(always)]
    pub fn apb2otp_block5_w2(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK5_W2_MASK) >> Self::APB2OTP_BLOCK5_W2_OFFSET
    }
}

/// APB2OTP_BLK5_W2 value type for batch read/modify/write operations
///
/// Use `apb2otpblk5w2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk5w2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk5W2Val {
    bits: u32,
}

impl Apb2otpBlk5W2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK5_W2** `[31:0]`
    ///
    /// Otp block5 word2 data.
    #[inline(always)]
    pub const fn apb2otp_block5_w2(&self) -> u32 {
        (self.bits & Apb2otpBlk5W2::APB2OTP_BLOCK5_W2_MASK)
            >> Apb2otpBlk5W2::APB2OTP_BLOCK5_W2_OFFSET
    }
}

impl Default for Apb2otpBlk5W2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk5W2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk5W2Val {
        Apb2otpBlk5W2Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK5_W3 Register
///
/// eFuse apb2otp block5 data register3.
///
/// Address offset: 0x0904
///
/// ## Fields
///
/// - **APB2OTP_BLOCK5_W3** `[31:0]` (RO)
///
///   Otp block5 word3 data.
#[repr(transparent)]
pub struct Apb2otpBlk5W3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk5W3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk5W3 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK5_W3**
    ///
    /// Otp block5 word3 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK5_W3_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK5_W3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK5_W3** `[31:0]` (RO)
    ///
    /// Otp block5 word3 data.
    #[inline(always)]
    pub fn apb2otp_block5_w3(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK5_W3_MASK) >> Self::APB2OTP_BLOCK5_W3_OFFSET
    }
}

/// APB2OTP_BLK5_W3 value type for batch read/modify/write operations
///
/// Use `apb2otpblk5w3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk5w3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk5W3Val {
    bits: u32,
}

impl Apb2otpBlk5W3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK5_W3** `[31:0]`
    ///
    /// Otp block5 word3 data.
    #[inline(always)]
    pub const fn apb2otp_block5_w3(&self) -> u32 {
        (self.bits & Apb2otpBlk5W3::APB2OTP_BLOCK5_W3_MASK)
            >> Apb2otpBlk5W3::APB2OTP_BLOCK5_W3_OFFSET
    }
}

impl Default for Apb2otpBlk5W3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk5W3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk5W3Val {
        Apb2otpBlk5W3Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK5_W4 Register
///
/// eFuse apb2otp block5 data register4.
///
/// Address offset: 0x0908
///
/// ## Fields
///
/// - **APB2OTP_BLOCK5_W4** `[31:0]` (RO)
///
///   Otp block5 word4 data.
#[repr(transparent)]
pub struct Apb2otpBlk5W4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk5W4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk5W4 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK5_W4**
    ///
    /// Otp block5 word4 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK5_W4_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK5_W4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK5_W4** `[31:0]` (RO)
    ///
    /// Otp block5 word4 data.
    #[inline(always)]
    pub fn apb2otp_block5_w4(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK5_W4_MASK) >> Self::APB2OTP_BLOCK5_W4_OFFSET
    }
}

/// APB2OTP_BLK5_W4 value type for batch read/modify/write operations
///
/// Use `apb2otpblk5w4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk5w4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk5W4Val {
    bits: u32,
}

impl Apb2otpBlk5W4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK5_W4** `[31:0]`
    ///
    /// Otp block5 word4 data.
    #[inline(always)]
    pub const fn apb2otp_block5_w4(&self) -> u32 {
        (self.bits & Apb2otpBlk5W4::APB2OTP_BLOCK5_W4_MASK)
            >> Apb2otpBlk5W4::APB2OTP_BLOCK5_W4_OFFSET
    }
}

impl Default for Apb2otpBlk5W4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk5W4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk5W4Val {
        Apb2otpBlk5W4Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK5_W5 Register
///
/// eFuse apb2otp block5 data register5.
///
/// Address offset: 0x090c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK5_W5** `[31:0]` (RO)
///
///   Otp block5 word5 data.
#[repr(transparent)]
pub struct Apb2otpBlk5W5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk5W5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk5W5 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK5_W5**
    ///
    /// Otp block5 word5 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK5_W5_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK5_W5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK5_W5** `[31:0]` (RO)
    ///
    /// Otp block5 word5 data.
    #[inline(always)]
    pub fn apb2otp_block5_w5(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK5_W5_MASK) >> Self::APB2OTP_BLOCK5_W5_OFFSET
    }
}

/// APB2OTP_BLK5_W5 value type for batch read/modify/write operations
///
/// Use `apb2otpblk5w5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk5w5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk5W5Val {
    bits: u32,
}

impl Apb2otpBlk5W5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK5_W5** `[31:0]`
    ///
    /// Otp block5 word5 data.
    #[inline(always)]
    pub const fn apb2otp_block5_w5(&self) -> u32 {
        (self.bits & Apb2otpBlk5W5::APB2OTP_BLOCK5_W5_MASK)
            >> Apb2otpBlk5W5::APB2OTP_BLOCK5_W5_OFFSET
    }
}

impl Default for Apb2otpBlk5W5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk5W5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk5W5Val {
        Apb2otpBlk5W5Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK5_W6 Register
///
/// eFuse apb2otp block5 data register6.
///
/// Address offset: 0x0910
///
/// ## Fields
///
/// - **APB2OTP_BLOCK5_W6** `[31:0]` (RO)
///
///   Otp block5 word6 data.
#[repr(transparent)]
pub struct Apb2otpBlk5W6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk5W6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk5W6 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK5_W6**
    ///
    /// Otp block5 word6 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK5_W6_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK5_W6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK5_W6** `[31:0]` (RO)
    ///
    /// Otp block5 word6 data.
    #[inline(always)]
    pub fn apb2otp_block5_w6(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK5_W6_MASK) >> Self::APB2OTP_BLOCK5_W6_OFFSET
    }
}

/// APB2OTP_BLK5_W6 value type for batch read/modify/write operations
///
/// Use `apb2otpblk5w6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk5w6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk5W6Val {
    bits: u32,
}

impl Apb2otpBlk5W6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK5_W6** `[31:0]`
    ///
    /// Otp block5 word6 data.
    #[inline(always)]
    pub const fn apb2otp_block5_w6(&self) -> u32 {
        (self.bits & Apb2otpBlk5W6::APB2OTP_BLOCK5_W6_MASK)
            >> Apb2otpBlk5W6::APB2OTP_BLOCK5_W6_OFFSET
    }
}

impl Default for Apb2otpBlk5W6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk5W6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk5W6Val {
        Apb2otpBlk5W6Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK5_W7 Register
///
/// eFuse apb2otp block5 data register7.
///
/// Address offset: 0x0914
///
/// ## Fields
///
/// - **APB2OTP_BLOCK5_W7** `[31:0]` (RO)
///
///   Otp block5 word7 data.
#[repr(transparent)]
pub struct Apb2otpBlk5W7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk5W7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk5W7 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK5_W7**
    ///
    /// Otp block5 word7 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK5_W7_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK5_W7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK5_W7** `[31:0]` (RO)
    ///
    /// Otp block5 word7 data.
    #[inline(always)]
    pub fn apb2otp_block5_w7(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK5_W7_MASK) >> Self::APB2OTP_BLOCK5_W7_OFFSET
    }
}

/// APB2OTP_BLK5_W7 value type for batch read/modify/write operations
///
/// Use `apb2otpblk5w7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk5w7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk5W7Val {
    bits: u32,
}

impl Apb2otpBlk5W7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK5_W7** `[31:0]`
    ///
    /// Otp block5 word7 data.
    #[inline(always)]
    pub const fn apb2otp_block5_w7(&self) -> u32 {
        (self.bits & Apb2otpBlk5W7::APB2OTP_BLOCK5_W7_MASK)
            >> Apb2otpBlk5W7::APB2OTP_BLOCK5_W7_OFFSET
    }
}

impl Default for Apb2otpBlk5W7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk5W7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk5W7Val {
        Apb2otpBlk5W7Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK5_W8 Register
///
/// eFuse apb2otp block5 data register8.
///
/// Address offset: 0x0918
///
/// ## Fields
///
/// - **APB2OTP_BLOCK5_W8** `[31:0]` (RO)
///
///   Otp block5 word8 data.
#[repr(transparent)]
pub struct Apb2otpBlk5W8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk5W8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk5W8 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK5_W8**
    ///
    /// Otp block5 word8 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK5_W8_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK5_W8_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK5_W8** `[31:0]` (RO)
    ///
    /// Otp block5 word8 data.
    #[inline(always)]
    pub fn apb2otp_block5_w8(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK5_W8_MASK) >> Self::APB2OTP_BLOCK5_W8_OFFSET
    }
}

/// APB2OTP_BLK5_W8 value type for batch read/modify/write operations
///
/// Use `apb2otpblk5w8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk5w8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk5W8Val {
    bits: u32,
}

impl Apb2otpBlk5W8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK5_W8** `[31:0]`
    ///
    /// Otp block5 word8 data.
    #[inline(always)]
    pub const fn apb2otp_block5_w8(&self) -> u32 {
        (self.bits & Apb2otpBlk5W8::APB2OTP_BLOCK5_W8_MASK)
            >> Apb2otpBlk5W8::APB2OTP_BLOCK5_W8_OFFSET
    }
}

impl Default for Apb2otpBlk5W8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk5W8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk5W8Val {
        Apb2otpBlk5W8Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK5_W9 Register
///
/// eFuse apb2otp block5 data register9.
///
/// Address offset: 0x091c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK5_W9** `[31:0]` (RO)
///
///   Otp block5 word9 data.
#[repr(transparent)]
pub struct Apb2otpBlk5W9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk5W9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk5W9 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK5_W9**
    ///
    /// Otp block5 word9 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK5_W9_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK5_W9_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK5_W9** `[31:0]` (RO)
    ///
    /// Otp block5 word9 data.
    #[inline(always)]
    pub fn apb2otp_block5_w9(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK5_W9_MASK) >> Self::APB2OTP_BLOCK5_W9_OFFSET
    }
}

/// APB2OTP_BLK5_W9 value type for batch read/modify/write operations
///
/// Use `apb2otpblk5w9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk5w9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk5W9Val {
    bits: u32,
}

impl Apb2otpBlk5W9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK5_W9** `[31:0]`
    ///
    /// Otp block5 word9 data.
    #[inline(always)]
    pub const fn apb2otp_block5_w9(&self) -> u32 {
        (self.bits & Apb2otpBlk5W9::APB2OTP_BLOCK5_W9_MASK)
            >> Apb2otpBlk5W9::APB2OTP_BLOCK5_W9_OFFSET
    }
}

impl Default for Apb2otpBlk5W9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk5W9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk5W9Val {
        Apb2otpBlk5W9Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK5_W10 Register
///
/// eFuse apb2otp block5 data register10.
///
/// Address offset: 0x0920
///
/// ## Fields
///
/// - **APB2OTP_BLOCK5_W10** `[31:0]` (RO)
///
///   Otp block5 word10 data.
#[repr(transparent)]
pub struct Apb2otpBlk5W10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk5W10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk5W10 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK5_W10**
    ///
    /// Otp block5 word10 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK5_W10_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK5_W10_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK5_W10** `[31:0]` (RO)
    ///
    /// Otp block5 word10 data.
    #[inline(always)]
    pub fn apb2otp_block5_w10(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK5_W10_MASK) >> Self::APB2OTP_BLOCK5_W10_OFFSET
    }
}

/// APB2OTP_BLK5_W10 value type for batch read/modify/write operations
///
/// Use `apb2otpblk5w10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk5w10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk5W10Val {
    bits: u32,
}

impl Apb2otpBlk5W10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK5_W10** `[31:0]`
    ///
    /// Otp block5 word10 data.
    #[inline(always)]
    pub const fn apb2otp_block5_w10(&self) -> u32 {
        (self.bits & Apb2otpBlk5W10::APB2OTP_BLOCK5_W10_MASK)
            >> Apb2otpBlk5W10::APB2OTP_BLOCK5_W10_OFFSET
    }
}

impl Default for Apb2otpBlk5W10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk5W10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk5W10Val {
        Apb2otpBlk5W10Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK5_W11 Register
///
/// eFuse apb2otp block5 data register11.
///
/// Address offset: 0x0924
///
/// ## Fields
///
/// - **APB2OTP_BLOCK5_W11** `[31:0]` (RO)
///
///   Otp block5 word11 data.
#[repr(transparent)]
pub struct Apb2otpBlk5W11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk5W11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk5W11 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK5_W11**
    ///
    /// Otp block5 word11 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK5_W11_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK5_W11_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK5_W11** `[31:0]` (RO)
    ///
    /// Otp block5 word11 data.
    #[inline(always)]
    pub fn apb2otp_block5_w11(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK5_W11_MASK) >> Self::APB2OTP_BLOCK5_W11_OFFSET
    }
}

/// APB2OTP_BLK5_W11 value type for batch read/modify/write operations
///
/// Use `apb2otpblk5w11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk5w11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk5W11Val {
    bits: u32,
}

impl Apb2otpBlk5W11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK5_W11** `[31:0]`
    ///
    /// Otp block5 word11 data.
    #[inline(always)]
    pub const fn apb2otp_block5_w11(&self) -> u32 {
        (self.bits & Apb2otpBlk5W11::APB2OTP_BLOCK5_W11_MASK)
            >> Apb2otpBlk5W11::APB2OTP_BLOCK5_W11_OFFSET
    }
}

impl Default for Apb2otpBlk5W11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk5W11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk5W11Val {
        Apb2otpBlk5W11Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK6_W1 Register
///
/// eFuse apb2otp block6 data register1.
///
/// Address offset: 0x0928
///
/// ## Fields
///
/// - **APB2OTP_BLOCK6_W1** `[31:0]` (RO)
///
///   Otp block6 word1 data.
#[repr(transparent)]
pub struct Apb2otpBlk6W1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk6W1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk6W1 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK6_W1**
    ///
    /// Otp block6 word1 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK6_W1_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK6_W1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK6_W1** `[31:0]` (RO)
    ///
    /// Otp block6 word1 data.
    #[inline(always)]
    pub fn apb2otp_block6_w1(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK6_W1_MASK) >> Self::APB2OTP_BLOCK6_W1_OFFSET
    }
}

/// APB2OTP_BLK6_W1 value type for batch read/modify/write operations
///
/// Use `apb2otpblk6w1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk6w1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk6W1Val {
    bits: u32,
}

impl Apb2otpBlk6W1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK6_W1** `[31:0]`
    ///
    /// Otp block6 word1 data.
    #[inline(always)]
    pub const fn apb2otp_block6_w1(&self) -> u32 {
        (self.bits & Apb2otpBlk6W1::APB2OTP_BLOCK6_W1_MASK)
            >> Apb2otpBlk6W1::APB2OTP_BLOCK6_W1_OFFSET
    }
}

impl Default for Apb2otpBlk6W1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk6W1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk6W1Val {
        Apb2otpBlk6W1Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK6_W2 Register
///
/// eFuse apb2otp block6 data register2.
///
/// Address offset: 0x092c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK6_W2** `[31:0]` (RO)
///
///   Otp block6 word2 data.
#[repr(transparent)]
pub struct Apb2otpBlk6W2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk6W2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk6W2 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK6_W2**
    ///
    /// Otp block6 word2 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK6_W2_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK6_W2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK6_W2** `[31:0]` (RO)
    ///
    /// Otp block6 word2 data.
    #[inline(always)]
    pub fn apb2otp_block6_w2(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK6_W2_MASK) >> Self::APB2OTP_BLOCK6_W2_OFFSET
    }
}

/// APB2OTP_BLK6_W2 value type for batch read/modify/write operations
///
/// Use `apb2otpblk6w2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk6w2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk6W2Val {
    bits: u32,
}

impl Apb2otpBlk6W2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK6_W2** `[31:0]`
    ///
    /// Otp block6 word2 data.
    #[inline(always)]
    pub const fn apb2otp_block6_w2(&self) -> u32 {
        (self.bits & Apb2otpBlk6W2::APB2OTP_BLOCK6_W2_MASK)
            >> Apb2otpBlk6W2::APB2OTP_BLOCK6_W2_OFFSET
    }
}

impl Default for Apb2otpBlk6W2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk6W2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk6W2Val {
        Apb2otpBlk6W2Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK6_W3 Register
///
/// eFuse apb2otp block6 data register3.
///
/// Address offset: 0x0930
///
/// ## Fields
///
/// - **APB2OTP_BLOCK6_W3** `[31:0]` (RO)
///
///   Otp block6 word3 data.
#[repr(transparent)]
pub struct Apb2otpBlk6W3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk6W3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk6W3 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK6_W3**
    ///
    /// Otp block6 word3 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK6_W3_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK6_W3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK6_W3** `[31:0]` (RO)
    ///
    /// Otp block6 word3 data.
    #[inline(always)]
    pub fn apb2otp_block6_w3(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK6_W3_MASK) >> Self::APB2OTP_BLOCK6_W3_OFFSET
    }
}

/// APB2OTP_BLK6_W3 value type for batch read/modify/write operations
///
/// Use `apb2otpblk6w3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk6w3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk6W3Val {
    bits: u32,
}

impl Apb2otpBlk6W3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK6_W3** `[31:0]`
    ///
    /// Otp block6 word3 data.
    #[inline(always)]
    pub const fn apb2otp_block6_w3(&self) -> u32 {
        (self.bits & Apb2otpBlk6W3::APB2OTP_BLOCK6_W3_MASK)
            >> Apb2otpBlk6W3::APB2OTP_BLOCK6_W3_OFFSET
    }
}

impl Default for Apb2otpBlk6W3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk6W3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk6W3Val {
        Apb2otpBlk6W3Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK6_W4 Register
///
/// eFuse apb2otp block6 data register4.
///
/// Address offset: 0x0934
///
/// ## Fields
///
/// - **APB2OTP_BLOCK6_W4** `[31:0]` (RO)
///
///   Otp block6 word4 data.
#[repr(transparent)]
pub struct Apb2otpBlk6W4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk6W4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk6W4 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK6_W4**
    ///
    /// Otp block6 word4 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK6_W4_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK6_W4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK6_W4** `[31:0]` (RO)
    ///
    /// Otp block6 word4 data.
    #[inline(always)]
    pub fn apb2otp_block6_w4(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK6_W4_MASK) >> Self::APB2OTP_BLOCK6_W4_OFFSET
    }
}

/// APB2OTP_BLK6_W4 value type for batch read/modify/write operations
///
/// Use `apb2otpblk6w4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk6w4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk6W4Val {
    bits: u32,
}

impl Apb2otpBlk6W4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK6_W4** `[31:0]`
    ///
    /// Otp block6 word4 data.
    #[inline(always)]
    pub const fn apb2otp_block6_w4(&self) -> u32 {
        (self.bits & Apb2otpBlk6W4::APB2OTP_BLOCK6_W4_MASK)
            >> Apb2otpBlk6W4::APB2OTP_BLOCK6_W4_OFFSET
    }
}

impl Default for Apb2otpBlk6W4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk6W4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk6W4Val {
        Apb2otpBlk6W4Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK6_W5 Register
///
/// eFuse apb2otp block6 data register5.
///
/// Address offset: 0x0938
///
/// ## Fields
///
/// - **APB2OTP_BLOCK6_W5** `[31:0]` (RO)
///
///   Otp block6 word5 data.
#[repr(transparent)]
pub struct Apb2otpBlk6W5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk6W5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk6W5 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK6_W5**
    ///
    /// Otp block6 word5 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK6_W5_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK6_W5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK6_W5** `[31:0]` (RO)
    ///
    /// Otp block6 word5 data.
    #[inline(always)]
    pub fn apb2otp_block6_w5(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK6_W5_MASK) >> Self::APB2OTP_BLOCK6_W5_OFFSET
    }
}

/// APB2OTP_BLK6_W5 value type for batch read/modify/write operations
///
/// Use `apb2otpblk6w5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk6w5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk6W5Val {
    bits: u32,
}

impl Apb2otpBlk6W5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK6_W5** `[31:0]`
    ///
    /// Otp block6 word5 data.
    #[inline(always)]
    pub const fn apb2otp_block6_w5(&self) -> u32 {
        (self.bits & Apb2otpBlk6W5::APB2OTP_BLOCK6_W5_MASK)
            >> Apb2otpBlk6W5::APB2OTP_BLOCK6_W5_OFFSET
    }
}

impl Default for Apb2otpBlk6W5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk6W5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk6W5Val {
        Apb2otpBlk6W5Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK6_W6 Register
///
/// eFuse apb2otp block6 data register6.
///
/// Address offset: 0x093c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK6_W6** `[31:0]` (RO)
///
///   Otp block6 word6 data.
#[repr(transparent)]
pub struct Apb2otpBlk6W6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk6W6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk6W6 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK6_W6**
    ///
    /// Otp block6 word6 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK6_W6_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK6_W6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK6_W6** `[31:0]` (RO)
    ///
    /// Otp block6 word6 data.
    #[inline(always)]
    pub fn apb2otp_block6_w6(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK6_W6_MASK) >> Self::APB2OTP_BLOCK6_W6_OFFSET
    }
}

/// APB2OTP_BLK6_W6 value type for batch read/modify/write operations
///
/// Use `apb2otpblk6w6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk6w6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk6W6Val {
    bits: u32,
}

impl Apb2otpBlk6W6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK6_W6** `[31:0]`
    ///
    /// Otp block6 word6 data.
    #[inline(always)]
    pub const fn apb2otp_block6_w6(&self) -> u32 {
        (self.bits & Apb2otpBlk6W6::APB2OTP_BLOCK6_W6_MASK)
            >> Apb2otpBlk6W6::APB2OTP_BLOCK6_W6_OFFSET
    }
}

impl Default for Apb2otpBlk6W6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk6W6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk6W6Val {
        Apb2otpBlk6W6Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK6_W7 Register
///
/// eFuse apb2otp block6 data register7.
///
/// Address offset: 0x0940
///
/// ## Fields
///
/// - **APB2OTP_BLOCK6_W7** `[31:0]` (RO)
///
///   Otp block6 word7 data.
#[repr(transparent)]
pub struct Apb2otpBlk6W7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk6W7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk6W7 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK6_W7**
    ///
    /// Otp block6 word7 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK6_W7_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK6_W7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK6_W7** `[31:0]` (RO)
    ///
    /// Otp block6 word7 data.
    #[inline(always)]
    pub fn apb2otp_block6_w7(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK6_W7_MASK) >> Self::APB2OTP_BLOCK6_W7_OFFSET
    }
}

/// APB2OTP_BLK6_W7 value type for batch read/modify/write operations
///
/// Use `apb2otpblk6w7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk6w7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk6W7Val {
    bits: u32,
}

impl Apb2otpBlk6W7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK6_W7** `[31:0]`
    ///
    /// Otp block6 word7 data.
    #[inline(always)]
    pub const fn apb2otp_block6_w7(&self) -> u32 {
        (self.bits & Apb2otpBlk6W7::APB2OTP_BLOCK6_W7_MASK)
            >> Apb2otpBlk6W7::APB2OTP_BLOCK6_W7_OFFSET
    }
}

impl Default for Apb2otpBlk6W7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk6W7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk6W7Val {
        Apb2otpBlk6W7Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK6_W8 Register
///
/// eFuse apb2otp block6 data register8.
///
/// Address offset: 0x0944
///
/// ## Fields
///
/// - **APB2OTP_BLOCK6_W8** `[31:0]` (RO)
///
///   Otp block6 word8 data.
#[repr(transparent)]
pub struct Apb2otpBlk6W8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk6W8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk6W8 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK6_W8**
    ///
    /// Otp block6 word8 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK6_W8_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK6_W8_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK6_W8** `[31:0]` (RO)
    ///
    /// Otp block6 word8 data.
    #[inline(always)]
    pub fn apb2otp_block6_w8(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK6_W8_MASK) >> Self::APB2OTP_BLOCK6_W8_OFFSET
    }
}

/// APB2OTP_BLK6_W8 value type for batch read/modify/write operations
///
/// Use `apb2otpblk6w8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk6w8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk6W8Val {
    bits: u32,
}

impl Apb2otpBlk6W8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK6_W8** `[31:0]`
    ///
    /// Otp block6 word8 data.
    #[inline(always)]
    pub const fn apb2otp_block6_w8(&self) -> u32 {
        (self.bits & Apb2otpBlk6W8::APB2OTP_BLOCK6_W8_MASK)
            >> Apb2otpBlk6W8::APB2OTP_BLOCK6_W8_OFFSET
    }
}

impl Default for Apb2otpBlk6W8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk6W8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk6W8Val {
        Apb2otpBlk6W8Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK6_W9 Register
///
/// eFuse apb2otp block6 data register9.
///
/// Address offset: 0x0948
///
/// ## Fields
///
/// - **APB2OTP_BLOCK6_W9** `[31:0]` (RO)
///
///   Otp block6 word9 data.
#[repr(transparent)]
pub struct Apb2otpBlk6W9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk6W9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk6W9 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK6_W9**
    ///
    /// Otp block6 word9 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK6_W9_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK6_W9_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK6_W9** `[31:0]` (RO)
    ///
    /// Otp block6 word9 data.
    #[inline(always)]
    pub fn apb2otp_block6_w9(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK6_W9_MASK) >> Self::APB2OTP_BLOCK6_W9_OFFSET
    }
}

/// APB2OTP_BLK6_W9 value type for batch read/modify/write operations
///
/// Use `apb2otpblk6w9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk6w9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk6W9Val {
    bits: u32,
}

impl Apb2otpBlk6W9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK6_W9** `[31:0]`
    ///
    /// Otp block6 word9 data.
    #[inline(always)]
    pub const fn apb2otp_block6_w9(&self) -> u32 {
        (self.bits & Apb2otpBlk6W9::APB2OTP_BLOCK6_W9_MASK)
            >> Apb2otpBlk6W9::APB2OTP_BLOCK6_W9_OFFSET
    }
}

impl Default for Apb2otpBlk6W9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk6W9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk6W9Val {
        Apb2otpBlk6W9Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK6_W10 Register
///
/// eFuse apb2otp block6 data register10.
///
/// Address offset: 0x094c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK6_W10** `[31:0]` (RO)
///
///   Otp block6 word10 data.
#[repr(transparent)]
pub struct Apb2otpBlk6W10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk6W10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk6W10 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK6_W10**
    ///
    /// Otp block6 word10 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK6_W10_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK6_W10_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK6_W10** `[31:0]` (RO)
    ///
    /// Otp block6 word10 data.
    #[inline(always)]
    pub fn apb2otp_block6_w10(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK6_W10_MASK) >> Self::APB2OTP_BLOCK6_W10_OFFSET
    }
}

/// APB2OTP_BLK6_W10 value type for batch read/modify/write operations
///
/// Use `apb2otpblk6w10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk6w10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk6W10Val {
    bits: u32,
}

impl Apb2otpBlk6W10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK6_W10** `[31:0]`
    ///
    /// Otp block6 word10 data.
    #[inline(always)]
    pub const fn apb2otp_block6_w10(&self) -> u32 {
        (self.bits & Apb2otpBlk6W10::APB2OTP_BLOCK6_W10_MASK)
            >> Apb2otpBlk6W10::APB2OTP_BLOCK6_W10_OFFSET
    }
}

impl Default for Apb2otpBlk6W10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk6W10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk6W10Val {
        Apb2otpBlk6W10Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK6_W11 Register
///
/// eFuse apb2otp block6 data register11.
///
/// Address offset: 0x0950
///
/// ## Fields
///
/// - **APB2OTP_BLOCK6_W11** `[31:0]` (RO)
///
///   Otp block6 word11 data.
#[repr(transparent)]
pub struct Apb2otpBlk6W11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk6W11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk6W11 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK6_W11**
    ///
    /// Otp block6 word11 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK6_W11_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK6_W11_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK6_W11** `[31:0]` (RO)
    ///
    /// Otp block6 word11 data.
    #[inline(always)]
    pub fn apb2otp_block6_w11(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK6_W11_MASK) >> Self::APB2OTP_BLOCK6_W11_OFFSET
    }
}

/// APB2OTP_BLK6_W11 value type for batch read/modify/write operations
///
/// Use `apb2otpblk6w11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk6w11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk6W11Val {
    bits: u32,
}

impl Apb2otpBlk6W11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK6_W11** `[31:0]`
    ///
    /// Otp block6 word11 data.
    #[inline(always)]
    pub const fn apb2otp_block6_w11(&self) -> u32 {
        (self.bits & Apb2otpBlk6W11::APB2OTP_BLOCK6_W11_MASK)
            >> Apb2otpBlk6W11::APB2OTP_BLOCK6_W11_OFFSET
    }
}

impl Default for Apb2otpBlk6W11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk6W11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk6W11Val {
        Apb2otpBlk6W11Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK7_W1 Register
///
/// eFuse apb2otp block7 data register1.
///
/// Address offset: 0x0954
///
/// ## Fields
///
/// - **APB2OTP_BLOCK7_W1** `[31:0]` (RO)
///
///   Otp block7 word1 data.
#[repr(transparent)]
pub struct Apb2otpBlk7W1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk7W1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk7W1 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK7_W1**
    ///
    /// Otp block7 word1 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK7_W1_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK7_W1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK7_W1** `[31:0]` (RO)
    ///
    /// Otp block7 word1 data.
    #[inline(always)]
    pub fn apb2otp_block7_w1(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK7_W1_MASK) >> Self::APB2OTP_BLOCK7_W1_OFFSET
    }
}

/// APB2OTP_BLK7_W1 value type for batch read/modify/write operations
///
/// Use `apb2otpblk7w1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk7w1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk7W1Val {
    bits: u32,
}

impl Apb2otpBlk7W1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK7_W1** `[31:0]`
    ///
    /// Otp block7 word1 data.
    #[inline(always)]
    pub const fn apb2otp_block7_w1(&self) -> u32 {
        (self.bits & Apb2otpBlk7W1::APB2OTP_BLOCK7_W1_MASK)
            >> Apb2otpBlk7W1::APB2OTP_BLOCK7_W1_OFFSET
    }
}

impl Default for Apb2otpBlk7W1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk7W1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk7W1Val {
        Apb2otpBlk7W1Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK7_W2 Register
///
/// eFuse apb2otp block7 data register2.
///
/// Address offset: 0x0958
///
/// ## Fields
///
/// - **APB2OTP_BLOCK7_W2** `[31:0]` (RO)
///
///   Otp block7 word2 data.
#[repr(transparent)]
pub struct Apb2otpBlk7W2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk7W2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk7W2 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK7_W2**
    ///
    /// Otp block7 word2 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK7_W2_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK7_W2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK7_W2** `[31:0]` (RO)
    ///
    /// Otp block7 word2 data.
    #[inline(always)]
    pub fn apb2otp_block7_w2(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK7_W2_MASK) >> Self::APB2OTP_BLOCK7_W2_OFFSET
    }
}

/// APB2OTP_BLK7_W2 value type for batch read/modify/write operations
///
/// Use `apb2otpblk7w2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk7w2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk7W2Val {
    bits: u32,
}

impl Apb2otpBlk7W2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK7_W2** `[31:0]`
    ///
    /// Otp block7 word2 data.
    #[inline(always)]
    pub const fn apb2otp_block7_w2(&self) -> u32 {
        (self.bits & Apb2otpBlk7W2::APB2OTP_BLOCK7_W2_MASK)
            >> Apb2otpBlk7W2::APB2OTP_BLOCK7_W2_OFFSET
    }
}

impl Default for Apb2otpBlk7W2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk7W2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk7W2Val {
        Apb2otpBlk7W2Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK7_W3 Register
///
/// eFuse apb2otp block7 data register3.
///
/// Address offset: 0x095c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK7_W3** `[31:0]` (RO)
///
///   Otp block7 word3 data.
#[repr(transparent)]
pub struct Apb2otpBlk7W3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk7W3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk7W3 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK7_W3**
    ///
    /// Otp block7 word3 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK7_W3_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK7_W3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK7_W3** `[31:0]` (RO)
    ///
    /// Otp block7 word3 data.
    #[inline(always)]
    pub fn apb2otp_block7_w3(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK7_W3_MASK) >> Self::APB2OTP_BLOCK7_W3_OFFSET
    }
}

/// APB2OTP_BLK7_W3 value type for batch read/modify/write operations
///
/// Use `apb2otpblk7w3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk7w3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk7W3Val {
    bits: u32,
}

impl Apb2otpBlk7W3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK7_W3** `[31:0]`
    ///
    /// Otp block7 word3 data.
    #[inline(always)]
    pub const fn apb2otp_block7_w3(&self) -> u32 {
        (self.bits & Apb2otpBlk7W3::APB2OTP_BLOCK7_W3_MASK)
            >> Apb2otpBlk7W3::APB2OTP_BLOCK7_W3_OFFSET
    }
}

impl Default for Apb2otpBlk7W3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk7W3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk7W3Val {
        Apb2otpBlk7W3Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK7_W4 Register
///
/// eFuse apb2otp block7 data register4.
///
/// Address offset: 0x0960
///
/// ## Fields
///
/// - **APB2OTP_BLOCK7_W4** `[31:0]` (RO)
///
///   Otp block7 word4 data.
#[repr(transparent)]
pub struct Apb2otpBlk7W4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk7W4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk7W4 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK7_W4**
    ///
    /// Otp block7 word4 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK7_W4_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK7_W4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK7_W4** `[31:0]` (RO)
    ///
    /// Otp block7 word4 data.
    #[inline(always)]
    pub fn apb2otp_block7_w4(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK7_W4_MASK) >> Self::APB2OTP_BLOCK7_W4_OFFSET
    }
}

/// APB2OTP_BLK7_W4 value type for batch read/modify/write operations
///
/// Use `apb2otpblk7w4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk7w4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk7W4Val {
    bits: u32,
}

impl Apb2otpBlk7W4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK7_W4** `[31:0]`
    ///
    /// Otp block7 word4 data.
    #[inline(always)]
    pub const fn apb2otp_block7_w4(&self) -> u32 {
        (self.bits & Apb2otpBlk7W4::APB2OTP_BLOCK7_W4_MASK)
            >> Apb2otpBlk7W4::APB2OTP_BLOCK7_W4_OFFSET
    }
}

impl Default for Apb2otpBlk7W4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk7W4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk7W4Val {
        Apb2otpBlk7W4Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK7_W5 Register
///
/// eFuse apb2otp block7 data register5.
///
/// Address offset: 0x0964
///
/// ## Fields
///
/// - **APB2OTP_BLOCK7_W5** `[31:0]` (RO)
///
///   Otp block7 word5 data.
#[repr(transparent)]
pub struct Apb2otpBlk7W5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk7W5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk7W5 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK7_W5**
    ///
    /// Otp block7 word5 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK7_W5_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK7_W5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK7_W5** `[31:0]` (RO)
    ///
    /// Otp block7 word5 data.
    #[inline(always)]
    pub fn apb2otp_block7_w5(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK7_W5_MASK) >> Self::APB2OTP_BLOCK7_W5_OFFSET
    }
}

/// APB2OTP_BLK7_W5 value type for batch read/modify/write operations
///
/// Use `apb2otpblk7w5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk7w5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk7W5Val {
    bits: u32,
}

impl Apb2otpBlk7W5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK7_W5** `[31:0]`
    ///
    /// Otp block7 word5 data.
    #[inline(always)]
    pub const fn apb2otp_block7_w5(&self) -> u32 {
        (self.bits & Apb2otpBlk7W5::APB2OTP_BLOCK7_W5_MASK)
            >> Apb2otpBlk7W5::APB2OTP_BLOCK7_W5_OFFSET
    }
}

impl Default for Apb2otpBlk7W5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk7W5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk7W5Val {
        Apb2otpBlk7W5Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK7_W6 Register
///
/// eFuse apb2otp block7 data register6.
///
/// Address offset: 0x0968
///
/// ## Fields
///
/// - **APB2OTP_BLOCK7_W6** `[31:0]` (RO)
///
///   Otp block7 word6 data.
#[repr(transparent)]
pub struct Apb2otpBlk7W6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk7W6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk7W6 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK7_W6**
    ///
    /// Otp block7 word6 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK7_W6_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK7_W6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK7_W6** `[31:0]` (RO)
    ///
    /// Otp block7 word6 data.
    #[inline(always)]
    pub fn apb2otp_block7_w6(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK7_W6_MASK) >> Self::APB2OTP_BLOCK7_W6_OFFSET
    }
}

/// APB2OTP_BLK7_W6 value type for batch read/modify/write operations
///
/// Use `apb2otpblk7w6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk7w6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk7W6Val {
    bits: u32,
}

impl Apb2otpBlk7W6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK7_W6** `[31:0]`
    ///
    /// Otp block7 word6 data.
    #[inline(always)]
    pub const fn apb2otp_block7_w6(&self) -> u32 {
        (self.bits & Apb2otpBlk7W6::APB2OTP_BLOCK7_W6_MASK)
            >> Apb2otpBlk7W6::APB2OTP_BLOCK7_W6_OFFSET
    }
}

impl Default for Apb2otpBlk7W6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk7W6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk7W6Val {
        Apb2otpBlk7W6Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK7_W7 Register
///
/// eFuse apb2otp block7 data register7.
///
/// Address offset: 0x096c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK7_W7** `[31:0]` (RO)
///
///   Otp block7 word7 data.
#[repr(transparent)]
pub struct Apb2otpBlk7W7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk7W7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk7W7 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK7_W7**
    ///
    /// Otp block7 word7 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK7_W7_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK7_W7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK7_W7** `[31:0]` (RO)
    ///
    /// Otp block7 word7 data.
    #[inline(always)]
    pub fn apb2otp_block7_w7(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK7_W7_MASK) >> Self::APB2OTP_BLOCK7_W7_OFFSET
    }
}

/// APB2OTP_BLK7_W7 value type for batch read/modify/write operations
///
/// Use `apb2otpblk7w7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk7w7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk7W7Val {
    bits: u32,
}

impl Apb2otpBlk7W7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK7_W7** `[31:0]`
    ///
    /// Otp block7 word7 data.
    #[inline(always)]
    pub const fn apb2otp_block7_w7(&self) -> u32 {
        (self.bits & Apb2otpBlk7W7::APB2OTP_BLOCK7_W7_MASK)
            >> Apb2otpBlk7W7::APB2OTP_BLOCK7_W7_OFFSET
    }
}

impl Default for Apb2otpBlk7W7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk7W7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk7W7Val {
        Apb2otpBlk7W7Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK7_W8 Register
///
/// eFuse apb2otp block7 data register8.
///
/// Address offset: 0x0970
///
/// ## Fields
///
/// - **APB2OTP_BLOCK7_W8** `[31:0]` (RO)
///
///   Otp block7 word8 data.
#[repr(transparent)]
pub struct Apb2otpBlk7W8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk7W8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk7W8 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK7_W8**
    ///
    /// Otp block7 word8 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK7_W8_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK7_W8_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK7_W8** `[31:0]` (RO)
    ///
    /// Otp block7 word8 data.
    #[inline(always)]
    pub fn apb2otp_block7_w8(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK7_W8_MASK) >> Self::APB2OTP_BLOCK7_W8_OFFSET
    }
}

/// APB2OTP_BLK7_W8 value type for batch read/modify/write operations
///
/// Use `apb2otpblk7w8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk7w8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk7W8Val {
    bits: u32,
}

impl Apb2otpBlk7W8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK7_W8** `[31:0]`
    ///
    /// Otp block7 word8 data.
    #[inline(always)]
    pub const fn apb2otp_block7_w8(&self) -> u32 {
        (self.bits & Apb2otpBlk7W8::APB2OTP_BLOCK7_W8_MASK)
            >> Apb2otpBlk7W8::APB2OTP_BLOCK7_W8_OFFSET
    }
}

impl Default for Apb2otpBlk7W8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk7W8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk7W8Val {
        Apb2otpBlk7W8Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK7_W9 Register
///
/// eFuse apb2otp block7 data register9.
///
/// Address offset: 0x0974
///
/// ## Fields
///
/// - **APB2OTP_BLOCK7_W9** `[31:0]` (RO)
///
///   Otp block7 word9 data.
#[repr(transparent)]
pub struct Apb2otpBlk7W9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk7W9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk7W9 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK7_W9**
    ///
    /// Otp block7 word9 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK7_W9_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK7_W9_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK7_W9** `[31:0]` (RO)
    ///
    /// Otp block7 word9 data.
    #[inline(always)]
    pub fn apb2otp_block7_w9(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK7_W9_MASK) >> Self::APB2OTP_BLOCK7_W9_OFFSET
    }
}

/// APB2OTP_BLK7_W9 value type for batch read/modify/write operations
///
/// Use `apb2otpblk7w9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk7w9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk7W9Val {
    bits: u32,
}

impl Apb2otpBlk7W9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK7_W9** `[31:0]`
    ///
    /// Otp block7 word9 data.
    #[inline(always)]
    pub const fn apb2otp_block7_w9(&self) -> u32 {
        (self.bits & Apb2otpBlk7W9::APB2OTP_BLOCK7_W9_MASK)
            >> Apb2otpBlk7W9::APB2OTP_BLOCK7_W9_OFFSET
    }
}

impl Default for Apb2otpBlk7W9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk7W9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk7W9Val {
        Apb2otpBlk7W9Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK7_W10 Register
///
/// eFuse apb2otp block7 data register10.
///
/// Address offset: 0x0978
///
/// ## Fields
///
/// - **APB2OTP_BLOCK7_W10** `[31:0]` (RO)
///
///   Otp block7 word10 data.
#[repr(transparent)]
pub struct Apb2otpBlk7W10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk7W10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk7W10 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK7_W10**
    ///
    /// Otp block7 word10 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK7_W10_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK7_W10_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK7_W10** `[31:0]` (RO)
    ///
    /// Otp block7 word10 data.
    #[inline(always)]
    pub fn apb2otp_block7_w10(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK7_W10_MASK) >> Self::APB2OTP_BLOCK7_W10_OFFSET
    }
}

/// APB2OTP_BLK7_W10 value type for batch read/modify/write operations
///
/// Use `apb2otpblk7w10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk7w10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk7W10Val {
    bits: u32,
}

impl Apb2otpBlk7W10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK7_W10** `[31:0]`
    ///
    /// Otp block7 word10 data.
    #[inline(always)]
    pub const fn apb2otp_block7_w10(&self) -> u32 {
        (self.bits & Apb2otpBlk7W10::APB2OTP_BLOCK7_W10_MASK)
            >> Apb2otpBlk7W10::APB2OTP_BLOCK7_W10_OFFSET
    }
}

impl Default for Apb2otpBlk7W10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk7W10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk7W10Val {
        Apb2otpBlk7W10Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK7_W11 Register
///
/// eFuse apb2otp block7 data register11.
///
/// Address offset: 0x097c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK7_W11** `[31:0]` (RO)
///
///   Otp block7 word11 data.
#[repr(transparent)]
pub struct Apb2otpBlk7W11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk7W11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk7W11 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK7_W11**
    ///
    /// Otp block7 word11 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK7_W11_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK7_W11_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK7_W11** `[31:0]` (RO)
    ///
    /// Otp block7 word11 data.
    #[inline(always)]
    pub fn apb2otp_block7_w11(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK7_W11_MASK) >> Self::APB2OTP_BLOCK7_W11_OFFSET
    }
}

/// APB2OTP_BLK7_W11 value type for batch read/modify/write operations
///
/// Use `apb2otpblk7w11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk7w11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk7W11Val {
    bits: u32,
}

impl Apb2otpBlk7W11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK7_W11** `[31:0]`
    ///
    /// Otp block7 word11 data.
    #[inline(always)]
    pub const fn apb2otp_block7_w11(&self) -> u32 {
        (self.bits & Apb2otpBlk7W11::APB2OTP_BLOCK7_W11_MASK)
            >> Apb2otpBlk7W11::APB2OTP_BLOCK7_W11_OFFSET
    }
}

impl Default for Apb2otpBlk7W11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk7W11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk7W11Val {
        Apb2otpBlk7W11Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK8_W1 Register
///
/// eFuse apb2otp block8 data register1.
///
/// Address offset: 0x0980
///
/// ## Fields
///
/// - **APB2OTP_BLOCK8_W1** `[31:0]` (RO)
///
///   Otp block8 word1 data.
#[repr(transparent)]
pub struct Apb2otpBlk8W1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk8W1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk8W1 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK8_W1**
    ///
    /// Otp block8 word1 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK8_W1_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK8_W1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK8_W1** `[31:0]` (RO)
    ///
    /// Otp block8 word1 data.
    #[inline(always)]
    pub fn apb2otp_block8_w1(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK8_W1_MASK) >> Self::APB2OTP_BLOCK8_W1_OFFSET
    }
}

/// APB2OTP_BLK8_W1 value type for batch read/modify/write operations
///
/// Use `apb2otpblk8w1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk8w1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk8W1Val {
    bits: u32,
}

impl Apb2otpBlk8W1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK8_W1** `[31:0]`
    ///
    /// Otp block8 word1 data.
    #[inline(always)]
    pub const fn apb2otp_block8_w1(&self) -> u32 {
        (self.bits & Apb2otpBlk8W1::APB2OTP_BLOCK8_W1_MASK)
            >> Apb2otpBlk8W1::APB2OTP_BLOCK8_W1_OFFSET
    }
}

impl Default for Apb2otpBlk8W1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk8W1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk8W1Val {
        Apb2otpBlk8W1Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK8_W2 Register
///
/// eFuse apb2otp block8 data register2.
///
/// Address offset: 0x0984
///
/// ## Fields
///
/// - **APB2OTP_BLOCK8_W2** `[31:0]` (RO)
///
///   Otp block8 word2 data.
#[repr(transparent)]
pub struct Apb2otpBlk8W2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk8W2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk8W2 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK8_W2**
    ///
    /// Otp block8 word2 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK8_W2_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK8_W2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK8_W2** `[31:0]` (RO)
    ///
    /// Otp block8 word2 data.
    #[inline(always)]
    pub fn apb2otp_block8_w2(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK8_W2_MASK) >> Self::APB2OTP_BLOCK8_W2_OFFSET
    }
}

/// APB2OTP_BLK8_W2 value type for batch read/modify/write operations
///
/// Use `apb2otpblk8w2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk8w2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk8W2Val {
    bits: u32,
}

impl Apb2otpBlk8W2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK8_W2** `[31:0]`
    ///
    /// Otp block8 word2 data.
    #[inline(always)]
    pub const fn apb2otp_block8_w2(&self) -> u32 {
        (self.bits & Apb2otpBlk8W2::APB2OTP_BLOCK8_W2_MASK)
            >> Apb2otpBlk8W2::APB2OTP_BLOCK8_W2_OFFSET
    }
}

impl Default for Apb2otpBlk8W2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk8W2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk8W2Val {
        Apb2otpBlk8W2Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK8_W3 Register
///
/// eFuse apb2otp block8 data register3.
///
/// Address offset: 0x0988
///
/// ## Fields
///
/// - **APB2OTP_BLOCK8_W3** `[31:0]` (RO)
///
///   Otp block8 word3 data.
#[repr(transparent)]
pub struct Apb2otpBlk8W3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk8W3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk8W3 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK8_W3**
    ///
    /// Otp block8 word3 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK8_W3_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK8_W3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK8_W3** `[31:0]` (RO)
    ///
    /// Otp block8 word3 data.
    #[inline(always)]
    pub fn apb2otp_block8_w3(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK8_W3_MASK) >> Self::APB2OTP_BLOCK8_W3_OFFSET
    }
}

/// APB2OTP_BLK8_W3 value type for batch read/modify/write operations
///
/// Use `apb2otpblk8w3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk8w3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk8W3Val {
    bits: u32,
}

impl Apb2otpBlk8W3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK8_W3** `[31:0]`
    ///
    /// Otp block8 word3 data.
    #[inline(always)]
    pub const fn apb2otp_block8_w3(&self) -> u32 {
        (self.bits & Apb2otpBlk8W3::APB2OTP_BLOCK8_W3_MASK)
            >> Apb2otpBlk8W3::APB2OTP_BLOCK8_W3_OFFSET
    }
}

impl Default for Apb2otpBlk8W3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk8W3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk8W3Val {
        Apb2otpBlk8W3Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK8_W4 Register
///
/// eFuse apb2otp block8 data register4.
///
/// Address offset: 0x098c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK8_W4** `[31:0]` (RO)
///
///   Otp block8 word4 data.
#[repr(transparent)]
pub struct Apb2otpBlk8W4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk8W4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk8W4 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK8_W4**
    ///
    /// Otp block8 word4 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK8_W4_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK8_W4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK8_W4** `[31:0]` (RO)
    ///
    /// Otp block8 word4 data.
    #[inline(always)]
    pub fn apb2otp_block8_w4(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK8_W4_MASK) >> Self::APB2OTP_BLOCK8_W4_OFFSET
    }
}

/// APB2OTP_BLK8_W4 value type for batch read/modify/write operations
///
/// Use `apb2otpblk8w4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk8w4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk8W4Val {
    bits: u32,
}

impl Apb2otpBlk8W4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK8_W4** `[31:0]`
    ///
    /// Otp block8 word4 data.
    #[inline(always)]
    pub const fn apb2otp_block8_w4(&self) -> u32 {
        (self.bits & Apb2otpBlk8W4::APB2OTP_BLOCK8_W4_MASK)
            >> Apb2otpBlk8W4::APB2OTP_BLOCK8_W4_OFFSET
    }
}

impl Default for Apb2otpBlk8W4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk8W4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk8W4Val {
        Apb2otpBlk8W4Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK8_W5 Register
///
/// eFuse apb2otp block8 data register5.
///
/// Address offset: 0x0990
///
/// ## Fields
///
/// - **APB2OTP_BLOCK8_W5** `[31:0]` (RO)
///
///   Otp block8 word5 data.
#[repr(transparent)]
pub struct Apb2otpBlk8W5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk8W5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk8W5 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK8_W5**
    ///
    /// Otp block8 word5 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK8_W5_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK8_W5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK8_W5** `[31:0]` (RO)
    ///
    /// Otp block8 word5 data.
    #[inline(always)]
    pub fn apb2otp_block8_w5(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK8_W5_MASK) >> Self::APB2OTP_BLOCK8_W5_OFFSET
    }
}

/// APB2OTP_BLK8_W5 value type for batch read/modify/write operations
///
/// Use `apb2otpblk8w5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk8w5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk8W5Val {
    bits: u32,
}

impl Apb2otpBlk8W5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK8_W5** `[31:0]`
    ///
    /// Otp block8 word5 data.
    #[inline(always)]
    pub const fn apb2otp_block8_w5(&self) -> u32 {
        (self.bits & Apb2otpBlk8W5::APB2OTP_BLOCK8_W5_MASK)
            >> Apb2otpBlk8W5::APB2OTP_BLOCK8_W5_OFFSET
    }
}

impl Default for Apb2otpBlk8W5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk8W5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk8W5Val {
        Apb2otpBlk8W5Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK8_W6 Register
///
/// eFuse apb2otp block8 data register6.
///
/// Address offset: 0x0994
///
/// ## Fields
///
/// - **APB2OTP_BLOCK8_W6** `[31:0]` (RO)
///
///   Otp block8 word6 data.
#[repr(transparent)]
pub struct Apb2otpBlk8W6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk8W6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk8W6 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK8_W6**
    ///
    /// Otp block8 word6 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK8_W6_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK8_W6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK8_W6** `[31:0]` (RO)
    ///
    /// Otp block8 word6 data.
    #[inline(always)]
    pub fn apb2otp_block8_w6(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK8_W6_MASK) >> Self::APB2OTP_BLOCK8_W6_OFFSET
    }
}

/// APB2OTP_BLK8_W6 value type for batch read/modify/write operations
///
/// Use `apb2otpblk8w6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk8w6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk8W6Val {
    bits: u32,
}

impl Apb2otpBlk8W6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK8_W6** `[31:0]`
    ///
    /// Otp block8 word6 data.
    #[inline(always)]
    pub const fn apb2otp_block8_w6(&self) -> u32 {
        (self.bits & Apb2otpBlk8W6::APB2OTP_BLOCK8_W6_MASK)
            >> Apb2otpBlk8W6::APB2OTP_BLOCK8_W6_OFFSET
    }
}

impl Default for Apb2otpBlk8W6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk8W6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk8W6Val {
        Apb2otpBlk8W6Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK8_W7 Register
///
/// eFuse apb2otp block8 data register7.
///
/// Address offset: 0x0998
///
/// ## Fields
///
/// - **APB2OTP_BLOCK8_W7** `[31:0]` (RO)
///
///   Otp block8 word7 data.
#[repr(transparent)]
pub struct Apb2otpBlk8W7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk8W7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk8W7 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK8_W7**
    ///
    /// Otp block8 word7 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK8_W7_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK8_W7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK8_W7** `[31:0]` (RO)
    ///
    /// Otp block8 word7 data.
    #[inline(always)]
    pub fn apb2otp_block8_w7(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK8_W7_MASK) >> Self::APB2OTP_BLOCK8_W7_OFFSET
    }
}

/// APB2OTP_BLK8_W7 value type for batch read/modify/write operations
///
/// Use `apb2otpblk8w7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk8w7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk8W7Val {
    bits: u32,
}

impl Apb2otpBlk8W7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK8_W7** `[31:0]`
    ///
    /// Otp block8 word7 data.
    #[inline(always)]
    pub const fn apb2otp_block8_w7(&self) -> u32 {
        (self.bits & Apb2otpBlk8W7::APB2OTP_BLOCK8_W7_MASK)
            >> Apb2otpBlk8W7::APB2OTP_BLOCK8_W7_OFFSET
    }
}

impl Default for Apb2otpBlk8W7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk8W7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk8W7Val {
        Apb2otpBlk8W7Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK8_W8 Register
///
/// eFuse apb2otp block8 data register8.
///
/// Address offset: 0x099c
///
/// ## Fields
///
/// - **APB2OTP_BLOCK8_W8** `[31:0]` (RO)
///
///   Otp block8 word8 data.
#[repr(transparent)]
pub struct Apb2otpBlk8W8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk8W8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk8W8 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK8_W8**
    ///
    /// Otp block8 word8 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK8_W8_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK8_W8_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK8_W8** `[31:0]` (RO)
    ///
    /// Otp block8 word8 data.
    #[inline(always)]
    pub fn apb2otp_block8_w8(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK8_W8_MASK) >> Self::APB2OTP_BLOCK8_W8_OFFSET
    }
}

/// APB2OTP_BLK8_W8 value type for batch read/modify/write operations
///
/// Use `apb2otpblk8w8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk8w8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk8W8Val {
    bits: u32,
}

impl Apb2otpBlk8W8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK8_W8** `[31:0]`
    ///
    /// Otp block8 word8 data.
    #[inline(always)]
    pub const fn apb2otp_block8_w8(&self) -> u32 {
        (self.bits & Apb2otpBlk8W8::APB2OTP_BLOCK8_W8_MASK)
            >> Apb2otpBlk8W8::APB2OTP_BLOCK8_W8_OFFSET
    }
}

impl Default for Apb2otpBlk8W8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk8W8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk8W8Val {
        Apb2otpBlk8W8Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK8_W9 Register
///
/// eFuse apb2otp block8 data register9.
///
/// Address offset: 0x09a0
///
/// ## Fields
///
/// - **APB2OTP_BLOCK8_W9** `[31:0]` (RO)
///
///   Otp block8 word9 data.
#[repr(transparent)]
pub struct Apb2otpBlk8W9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk8W9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk8W9 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK8_W9**
    ///
    /// Otp block8 word9 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK8_W9_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK8_W9_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK8_W9** `[31:0]` (RO)
    ///
    /// Otp block8 word9 data.
    #[inline(always)]
    pub fn apb2otp_block8_w9(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK8_W9_MASK) >> Self::APB2OTP_BLOCK8_W9_OFFSET
    }
}

/// APB2OTP_BLK8_W9 value type for batch read/modify/write operations
///
/// Use `apb2otpblk8w9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk8w9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk8W9Val {
    bits: u32,
}

impl Apb2otpBlk8W9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK8_W9** `[31:0]`
    ///
    /// Otp block8 word9 data.
    #[inline(always)]
    pub const fn apb2otp_block8_w9(&self) -> u32 {
        (self.bits & Apb2otpBlk8W9::APB2OTP_BLOCK8_W9_MASK)
            >> Apb2otpBlk8W9::APB2OTP_BLOCK8_W9_OFFSET
    }
}

impl Default for Apb2otpBlk8W9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk8W9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk8W9Val {
        Apb2otpBlk8W9Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK8_W10 Register
///
/// eFuse apb2otp block8 data register10.
///
/// Address offset: 0x09a4
///
/// ## Fields
///
/// - **APB2OTP_BLOCK8_W10** `[31:0]` (RO)
///
///   Otp block8 word10 data.
#[repr(transparent)]
pub struct Apb2otpBlk8W10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk8W10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk8W10 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK8_W10**
    ///
    /// Otp block8 word10 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK8_W10_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK8_W10_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK8_W10** `[31:0]` (RO)
    ///
    /// Otp block8 word10 data.
    #[inline(always)]
    pub fn apb2otp_block8_w10(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK8_W10_MASK) >> Self::APB2OTP_BLOCK8_W10_OFFSET
    }
}

/// APB2OTP_BLK8_W10 value type for batch read/modify/write operations
///
/// Use `apb2otpblk8w10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk8w10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk8W10Val {
    bits: u32,
}

impl Apb2otpBlk8W10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK8_W10** `[31:0]`
    ///
    /// Otp block8 word10 data.
    #[inline(always)]
    pub const fn apb2otp_block8_w10(&self) -> u32 {
        (self.bits & Apb2otpBlk8W10::APB2OTP_BLOCK8_W10_MASK)
            >> Apb2otpBlk8W10::APB2OTP_BLOCK8_W10_OFFSET
    }
}

impl Default for Apb2otpBlk8W10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk8W10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk8W10Val {
        Apb2otpBlk8W10Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK8_W11 Register
///
/// eFuse apb2otp block8 data register11.
///
/// Address offset: 0x09a8
///
/// ## Fields
///
/// - **APB2OTP_BLOCK8_W11** `[31:0]` (RO)
///
///   Otp block8 word11 data.
#[repr(transparent)]
pub struct Apb2otpBlk8W11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk8W11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk8W11 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK8_W11**
    ///
    /// Otp block8 word11 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK8_W11_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK8_W11_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK8_W11** `[31:0]` (RO)
    ///
    /// Otp block8 word11 data.
    #[inline(always)]
    pub fn apb2otp_block8_w11(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK8_W11_MASK) >> Self::APB2OTP_BLOCK8_W11_OFFSET
    }
}

/// APB2OTP_BLK8_W11 value type for batch read/modify/write operations
///
/// Use `apb2otpblk8w11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk8w11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk8W11Val {
    bits: u32,
}

impl Apb2otpBlk8W11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK8_W11** `[31:0]`
    ///
    /// Otp block8 word11 data.
    #[inline(always)]
    pub const fn apb2otp_block8_w11(&self) -> u32 {
        (self.bits & Apb2otpBlk8W11::APB2OTP_BLOCK8_W11_MASK)
            >> Apb2otpBlk8W11::APB2OTP_BLOCK8_W11_OFFSET
    }
}

impl Default for Apb2otpBlk8W11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk8W11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk8W11Val {
        Apb2otpBlk8W11Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK9_W1 Register
///
/// eFuse apb2otp block9 data register1.
///
/// Address offset: 0x09ac
///
/// ## Fields
///
/// - **APB2OTP_BLOCK9_W1** `[31:0]` (RO)
///
///   Otp block9 word1 data.
#[repr(transparent)]
pub struct Apb2otpBlk9W1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk9W1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk9W1 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK9_W1**
    ///
    /// Otp block9 word1 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK9_W1_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK9_W1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK9_W1** `[31:0]` (RO)
    ///
    /// Otp block9 word1 data.
    #[inline(always)]
    pub fn apb2otp_block9_w1(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK9_W1_MASK) >> Self::APB2OTP_BLOCK9_W1_OFFSET
    }
}

/// APB2OTP_BLK9_W1 value type for batch read/modify/write operations
///
/// Use `apb2otpblk9w1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk9w1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk9W1Val {
    bits: u32,
}

impl Apb2otpBlk9W1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK9_W1** `[31:0]`
    ///
    /// Otp block9 word1 data.
    #[inline(always)]
    pub const fn apb2otp_block9_w1(&self) -> u32 {
        (self.bits & Apb2otpBlk9W1::APB2OTP_BLOCK9_W1_MASK)
            >> Apb2otpBlk9W1::APB2OTP_BLOCK9_W1_OFFSET
    }
}

impl Default for Apb2otpBlk9W1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk9W1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk9W1Val {
        Apb2otpBlk9W1Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK9_W2 Register
///
/// eFuse apb2otp block9 data register2.
///
/// Address offset: 0x09b0
///
/// ## Fields
///
/// - **APB2OTP_BLOCK9_W2** `[31:0]` (RO)
///
///   Otp block9 word2 data.
#[repr(transparent)]
pub struct Apb2otpBlk9W2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk9W2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk9W2 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK9_W2**
    ///
    /// Otp block9 word2 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK9_W2_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK9_W2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK9_W2** `[31:0]` (RO)
    ///
    /// Otp block9 word2 data.
    #[inline(always)]
    pub fn apb2otp_block9_w2(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK9_W2_MASK) >> Self::APB2OTP_BLOCK9_W2_OFFSET
    }
}

/// APB2OTP_BLK9_W2 value type for batch read/modify/write operations
///
/// Use `apb2otpblk9w2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk9w2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk9W2Val {
    bits: u32,
}

impl Apb2otpBlk9W2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK9_W2** `[31:0]`
    ///
    /// Otp block9 word2 data.
    #[inline(always)]
    pub const fn apb2otp_block9_w2(&self) -> u32 {
        (self.bits & Apb2otpBlk9W2::APB2OTP_BLOCK9_W2_MASK)
            >> Apb2otpBlk9W2::APB2OTP_BLOCK9_W2_OFFSET
    }
}

impl Default for Apb2otpBlk9W2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk9W2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk9W2Val {
        Apb2otpBlk9W2Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK9_W3 Register
///
/// eFuse apb2otp block9 data register3.
///
/// Address offset: 0x09b4
///
/// ## Fields
///
/// - **APB2OTP_BLOCK9_W3** `[31:0]` (RO)
///
///   Otp block9 word3 data.
#[repr(transparent)]
pub struct Apb2otpBlk9W3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk9W3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk9W3 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK9_W3**
    ///
    /// Otp block9 word3 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK9_W3_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK9_W3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK9_W3** `[31:0]` (RO)
    ///
    /// Otp block9 word3 data.
    #[inline(always)]
    pub fn apb2otp_block9_w3(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK9_W3_MASK) >> Self::APB2OTP_BLOCK9_W3_OFFSET
    }
}

/// APB2OTP_BLK9_W3 value type for batch read/modify/write operations
///
/// Use `apb2otpblk9w3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk9w3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk9W3Val {
    bits: u32,
}

impl Apb2otpBlk9W3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK9_W3** `[31:0]`
    ///
    /// Otp block9 word3 data.
    #[inline(always)]
    pub const fn apb2otp_block9_w3(&self) -> u32 {
        (self.bits & Apb2otpBlk9W3::APB2OTP_BLOCK9_W3_MASK)
            >> Apb2otpBlk9W3::APB2OTP_BLOCK9_W3_OFFSET
    }
}

impl Default for Apb2otpBlk9W3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk9W3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk9W3Val {
        Apb2otpBlk9W3Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK9_W4 Register
///
/// eFuse apb2otp block9 data register4.
///
/// Address offset: 0x09b8
///
/// ## Fields
///
/// - **APB2OTP_BLOCK9_W4** `[31:0]` (RO)
///
///   Otp block9 word4 data.
#[repr(transparent)]
pub struct Apb2otpBlk9W4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk9W4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk9W4 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK9_W4**
    ///
    /// Otp block9 word4 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK9_W4_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK9_W4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK9_W4** `[31:0]` (RO)
    ///
    /// Otp block9 word4 data.
    #[inline(always)]
    pub fn apb2otp_block9_w4(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK9_W4_MASK) >> Self::APB2OTP_BLOCK9_W4_OFFSET
    }
}

/// APB2OTP_BLK9_W4 value type for batch read/modify/write operations
///
/// Use `apb2otpblk9w4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk9w4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk9W4Val {
    bits: u32,
}

impl Apb2otpBlk9W4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK9_W4** `[31:0]`
    ///
    /// Otp block9 word4 data.
    #[inline(always)]
    pub const fn apb2otp_block9_w4(&self) -> u32 {
        (self.bits & Apb2otpBlk9W4::APB2OTP_BLOCK9_W4_MASK)
            >> Apb2otpBlk9W4::APB2OTP_BLOCK9_W4_OFFSET
    }
}

impl Default for Apb2otpBlk9W4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk9W4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk9W4Val {
        Apb2otpBlk9W4Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK9_W5 Register
///
/// eFuse apb2otp block9 data register5.
///
/// Address offset: 0x09bc
///
/// ## Fields
///
/// - **APB2OTP_BLOCK9_W5** `[31:0]` (RO)
///
///   Otp block9 word5 data.
#[repr(transparent)]
pub struct Apb2otpBlk9W5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk9W5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk9W5 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK9_W5**
    ///
    /// Otp block9 word5 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK9_W5_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK9_W5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK9_W5** `[31:0]` (RO)
    ///
    /// Otp block9 word5 data.
    #[inline(always)]
    pub fn apb2otp_block9_w5(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK9_W5_MASK) >> Self::APB2OTP_BLOCK9_W5_OFFSET
    }
}

/// APB2OTP_BLK9_W5 value type for batch read/modify/write operations
///
/// Use `apb2otpblk9w5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk9w5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk9W5Val {
    bits: u32,
}

impl Apb2otpBlk9W5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK9_W5** `[31:0]`
    ///
    /// Otp block9 word5 data.
    #[inline(always)]
    pub const fn apb2otp_block9_w5(&self) -> u32 {
        (self.bits & Apb2otpBlk9W5::APB2OTP_BLOCK9_W5_MASK)
            >> Apb2otpBlk9W5::APB2OTP_BLOCK9_W5_OFFSET
    }
}

impl Default for Apb2otpBlk9W5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk9W5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk9W5Val {
        Apb2otpBlk9W5Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK9_W6 Register
///
/// eFuse apb2otp block9 data register6.
///
/// Address offset: 0x09c0
///
/// ## Fields
///
/// - **APB2OTP_BLOCK9_W6** `[31:0]` (RO)
///
///   Otp block9 word6 data.
#[repr(transparent)]
pub struct Apb2otpBlk9W6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk9W6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk9W6 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK9_W6**
    ///
    /// Otp block9 word6 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK9_W6_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK9_W6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK9_W6** `[31:0]` (RO)
    ///
    /// Otp block9 word6 data.
    #[inline(always)]
    pub fn apb2otp_block9_w6(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK9_W6_MASK) >> Self::APB2OTP_BLOCK9_W6_OFFSET
    }
}

/// APB2OTP_BLK9_W6 value type for batch read/modify/write operations
///
/// Use `apb2otpblk9w6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk9w6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk9W6Val {
    bits: u32,
}

impl Apb2otpBlk9W6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK9_W6** `[31:0]`
    ///
    /// Otp block9 word6 data.
    #[inline(always)]
    pub const fn apb2otp_block9_w6(&self) -> u32 {
        (self.bits & Apb2otpBlk9W6::APB2OTP_BLOCK9_W6_MASK)
            >> Apb2otpBlk9W6::APB2OTP_BLOCK9_W6_OFFSET
    }
}

impl Default for Apb2otpBlk9W6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk9W6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk9W6Val {
        Apb2otpBlk9W6Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK9_W7 Register
///
/// eFuse apb2otp block9 data register7.
///
/// Address offset: 0x09c4
///
/// ## Fields
///
/// - **APB2OTP_BLOCK9_W7** `[31:0]` (RO)
///
///   Otp block9 word7 data.
#[repr(transparent)]
pub struct Apb2otpBlk9W7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk9W7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk9W7 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK9_W7**
    ///
    /// Otp block9 word7 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK9_W7_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK9_W7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK9_W7** `[31:0]` (RO)
    ///
    /// Otp block9 word7 data.
    #[inline(always)]
    pub fn apb2otp_block9_w7(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK9_W7_MASK) >> Self::APB2OTP_BLOCK9_W7_OFFSET
    }
}

/// APB2OTP_BLK9_W7 value type for batch read/modify/write operations
///
/// Use `apb2otpblk9w7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk9w7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk9W7Val {
    bits: u32,
}

impl Apb2otpBlk9W7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK9_W7** `[31:0]`
    ///
    /// Otp block9 word7 data.
    #[inline(always)]
    pub const fn apb2otp_block9_w7(&self) -> u32 {
        (self.bits & Apb2otpBlk9W7::APB2OTP_BLOCK9_W7_MASK)
            >> Apb2otpBlk9W7::APB2OTP_BLOCK9_W7_OFFSET
    }
}

impl Default for Apb2otpBlk9W7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk9W7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk9W7Val {
        Apb2otpBlk9W7Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK9_W8 Register
///
/// eFuse apb2otp block9 data register8.
///
/// Address offset: 0x09c8
///
/// ## Fields
///
/// - **APB2OTP_BLOCK9_W8** `[31:0]` (RO)
///
///   Otp block9 word8 data.
#[repr(transparent)]
pub struct Apb2otpBlk9W8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk9W8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk9W8 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK9_W8**
    ///
    /// Otp block9 word8 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK9_W8_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK9_W8_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK9_W8** `[31:0]` (RO)
    ///
    /// Otp block9 word8 data.
    #[inline(always)]
    pub fn apb2otp_block9_w8(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK9_W8_MASK) >> Self::APB2OTP_BLOCK9_W8_OFFSET
    }
}

/// APB2OTP_BLK9_W8 value type for batch read/modify/write operations
///
/// Use `apb2otpblk9w8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk9w8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk9W8Val {
    bits: u32,
}

impl Apb2otpBlk9W8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK9_W8** `[31:0]`
    ///
    /// Otp block9 word8 data.
    #[inline(always)]
    pub const fn apb2otp_block9_w8(&self) -> u32 {
        (self.bits & Apb2otpBlk9W8::APB2OTP_BLOCK9_W8_MASK)
            >> Apb2otpBlk9W8::APB2OTP_BLOCK9_W8_OFFSET
    }
}

impl Default for Apb2otpBlk9W8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk9W8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk9W8Val {
        Apb2otpBlk9W8Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK9_W9 Register
///
/// eFuse apb2otp block9 data register9.
///
/// Address offset: 0x09cc
///
/// ## Fields
///
/// - **APB2OTP_BLOCK9_W9** `[31:0]` (RO)
///
///   Otp block9 word9 data.
#[repr(transparent)]
pub struct Apb2otpBlk9W9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk9W9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk9W9 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK9_W9**
    ///
    /// Otp block9 word9 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK9_W9_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK9_W9_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK9_W9** `[31:0]` (RO)
    ///
    /// Otp block9 word9 data.
    #[inline(always)]
    pub fn apb2otp_block9_w9(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK9_W9_MASK) >> Self::APB2OTP_BLOCK9_W9_OFFSET
    }
}

/// APB2OTP_BLK9_W9 value type for batch read/modify/write operations
///
/// Use `apb2otpblk9w9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk9w9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk9W9Val {
    bits: u32,
}

impl Apb2otpBlk9W9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK9_W9** `[31:0]`
    ///
    /// Otp block9 word9 data.
    #[inline(always)]
    pub const fn apb2otp_block9_w9(&self) -> u32 {
        (self.bits & Apb2otpBlk9W9::APB2OTP_BLOCK9_W9_MASK)
            >> Apb2otpBlk9W9::APB2OTP_BLOCK9_W9_OFFSET
    }
}

impl Default for Apb2otpBlk9W9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk9W9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk9W9Val {
        Apb2otpBlk9W9Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK9_W10 Register
///
/// eFuse apb2otp block9 data register10.
///
/// Address offset: 0x09d0
///
/// ## Fields
///
/// - **APB2OTP_BLOCK9_W10** `[31:0]` (RO)
///
///   Otp block9 word10 data.
#[repr(transparent)]
pub struct Apb2otpBlk9W10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk9W10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk9W10 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK9_W10**
    ///
    /// Otp block9 word10 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK9_W10_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK9_W10_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK9_W10** `[31:0]` (RO)
    ///
    /// Otp block9 word10 data.
    #[inline(always)]
    pub fn apb2otp_block9_w10(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK9_W10_MASK) >> Self::APB2OTP_BLOCK9_W10_OFFSET
    }
}

/// APB2OTP_BLK9_W10 value type for batch read/modify/write operations
///
/// Use `apb2otpblk9w10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk9w10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk9W10Val {
    bits: u32,
}

impl Apb2otpBlk9W10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK9_W10** `[31:0]`
    ///
    /// Otp block9 word10 data.
    #[inline(always)]
    pub const fn apb2otp_block9_w10(&self) -> u32 {
        (self.bits & Apb2otpBlk9W10::APB2OTP_BLOCK9_W10_MASK)
            >> Apb2otpBlk9W10::APB2OTP_BLOCK9_W10_OFFSET
    }
}

impl Default for Apb2otpBlk9W10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk9W10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk9W10Val {
        Apb2otpBlk9W10Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK9_W11 Register
///
/// eFuse apb2otp block9 data register11.
///
/// Address offset: 0x09d4
///
/// ## Fields
///
/// - **APB2OTP_BLOCK9_W11** `[31:0]` (RO)
///
///   Otp block9 word11 data.
#[repr(transparent)]
pub struct Apb2otpBlk9W11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk9W11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk9W11 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK9_W11**
    ///
    /// Otp block9 word11 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK9_W11_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK9_W11_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK9_W11** `[31:0]` (RO)
    ///
    /// Otp block9 word11 data.
    #[inline(always)]
    pub fn apb2otp_block9_w11(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK9_W11_MASK) >> Self::APB2OTP_BLOCK9_W11_OFFSET
    }
}

/// APB2OTP_BLK9_W11 value type for batch read/modify/write operations
///
/// Use `apb2otpblk9w11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk9w11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk9W11Val {
    bits: u32,
}

impl Apb2otpBlk9W11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK9_W11** `[31:0]`
    ///
    /// Otp block9 word11 data.
    #[inline(always)]
    pub const fn apb2otp_block9_w11(&self) -> u32 {
        (self.bits & Apb2otpBlk9W11::APB2OTP_BLOCK9_W11_MASK)
            >> Apb2otpBlk9W11::APB2OTP_BLOCK9_W11_OFFSET
    }
}

impl Default for Apb2otpBlk9W11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk9W11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk9W11Val {
        Apb2otpBlk9W11Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK10_W1 Register
///
/// eFuse apb2otp block10 data register1.
///
/// Address offset: 0x09d8
///
/// ## Fields
///
/// - **APB2OTP_BLOCK10_W1** `[31:0]` (RO)
///
///   Otp block10 word1 data.
#[repr(transparent)]
pub struct Apb2otpBlk10W1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk10W1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk10W1 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK10_W1**
    ///
    /// Otp block10 word1 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK10_W1_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK10_W1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK10_W1** `[31:0]` (RO)
    ///
    /// Otp block10 word1 data.
    #[inline(always)]
    pub fn apb2otp_block10_w1(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK10_W1_MASK) >> Self::APB2OTP_BLOCK10_W1_OFFSET
    }
}

/// APB2OTP_BLK10_W1 value type for batch read/modify/write operations
///
/// Use `apb2otpblk10w1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk10w1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk10W1Val {
    bits: u32,
}

impl Apb2otpBlk10W1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK10_W1** `[31:0]`
    ///
    /// Otp block10 word1 data.
    #[inline(always)]
    pub const fn apb2otp_block10_w1(&self) -> u32 {
        (self.bits & Apb2otpBlk10W1::APB2OTP_BLOCK10_W1_MASK)
            >> Apb2otpBlk10W1::APB2OTP_BLOCK10_W1_OFFSET
    }
}

impl Default for Apb2otpBlk10W1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk10W1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk10W1Val {
        Apb2otpBlk10W1Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK10_W2 Register
///
/// eFuse apb2otp block10 data register2.
///
/// Address offset: 0x09dc
///
/// ## Fields
///
/// - **APB2OTP_BLOCK10_W2** `[31:0]` (RO)
///
///   Otp block10 word2 data.
#[repr(transparent)]
pub struct Apb2otpBlk10W2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk10W2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk10W2 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK10_W2**
    ///
    /// Otp block10 word2 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK10_W2_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK10_W2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK10_W2** `[31:0]` (RO)
    ///
    /// Otp block10 word2 data.
    #[inline(always)]
    pub fn apb2otp_block10_w2(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK10_W2_MASK) >> Self::APB2OTP_BLOCK10_W2_OFFSET
    }
}

/// APB2OTP_BLK10_W2 value type for batch read/modify/write operations
///
/// Use `apb2otpblk10w2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk10w2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk10W2Val {
    bits: u32,
}

impl Apb2otpBlk10W2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK10_W2** `[31:0]`
    ///
    /// Otp block10 word2 data.
    #[inline(always)]
    pub const fn apb2otp_block10_w2(&self) -> u32 {
        (self.bits & Apb2otpBlk10W2::APB2OTP_BLOCK10_W2_MASK)
            >> Apb2otpBlk10W2::APB2OTP_BLOCK10_W2_OFFSET
    }
}

impl Default for Apb2otpBlk10W2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk10W2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk10W2Val {
        Apb2otpBlk10W2Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK10_W3 Register
///
/// eFuse apb2otp block10 data register3.
///
/// Address offset: 0x09e0
///
/// ## Fields
///
/// - **APB2OTP_BLOCK10_W3** `[31:0]` (RO)
///
///   Otp block10 word3 data.
#[repr(transparent)]
pub struct Apb2otpBlk10W3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk10W3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk10W3 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK10_W3**
    ///
    /// Otp block10 word3 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK10_W3_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK10_W3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK10_W3** `[31:0]` (RO)
    ///
    /// Otp block10 word3 data.
    #[inline(always)]
    pub fn apb2otp_block10_w3(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK10_W3_MASK) >> Self::APB2OTP_BLOCK10_W3_OFFSET
    }
}

/// APB2OTP_BLK10_W3 value type for batch read/modify/write operations
///
/// Use `apb2otpblk10w3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk10w3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk10W3Val {
    bits: u32,
}

impl Apb2otpBlk10W3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK10_W3** `[31:0]`
    ///
    /// Otp block10 word3 data.
    #[inline(always)]
    pub const fn apb2otp_block10_w3(&self) -> u32 {
        (self.bits & Apb2otpBlk10W3::APB2OTP_BLOCK10_W3_MASK)
            >> Apb2otpBlk10W3::APB2OTP_BLOCK10_W3_OFFSET
    }
}

impl Default for Apb2otpBlk10W3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk10W3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk10W3Val {
        Apb2otpBlk10W3Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK10_W4 Register
///
/// eFuse apb2otp block10 data register4.
///
/// Address offset: 0x09e4
///
/// ## Fields
///
/// - **APB2OTP_BLOCK10_W4** `[31:0]` (RO)
///
///   Otp block10 word4 data.
#[repr(transparent)]
pub struct Apb2otpBlk10W4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk10W4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk10W4 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK10_W4**
    ///
    /// Otp block10 word4 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK10_W4_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK10_W4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK10_W4** `[31:0]` (RO)
    ///
    /// Otp block10 word4 data.
    #[inline(always)]
    pub fn apb2otp_block10_w4(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK10_W4_MASK) >> Self::APB2OTP_BLOCK10_W4_OFFSET
    }
}

/// APB2OTP_BLK10_W4 value type for batch read/modify/write operations
///
/// Use `apb2otpblk10w4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk10w4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk10W4Val {
    bits: u32,
}

impl Apb2otpBlk10W4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK10_W4** `[31:0]`
    ///
    /// Otp block10 word4 data.
    #[inline(always)]
    pub const fn apb2otp_block10_w4(&self) -> u32 {
        (self.bits & Apb2otpBlk10W4::APB2OTP_BLOCK10_W4_MASK)
            >> Apb2otpBlk10W4::APB2OTP_BLOCK10_W4_OFFSET
    }
}

impl Default for Apb2otpBlk10W4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk10W4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk10W4Val {
        Apb2otpBlk10W4Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK10_W5 Register
///
/// eFuse apb2otp block10 data register5.
///
/// Address offset: 0x09e8
///
/// ## Fields
///
/// - **APB2OTP_BLOCK10_W5** `[31:0]` (RO)
///
///   Otp block10 word5 data.
#[repr(transparent)]
pub struct Apb2otpBlk10W5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk10W5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk10W5 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK10_W5**
    ///
    /// Otp block10 word5 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK10_W5_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK10_W5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK10_W5** `[31:0]` (RO)
    ///
    /// Otp block10 word5 data.
    #[inline(always)]
    pub fn apb2otp_block10_w5(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK10_W5_MASK) >> Self::APB2OTP_BLOCK10_W5_OFFSET
    }
}

/// APB2OTP_BLK10_W5 value type for batch read/modify/write operations
///
/// Use `apb2otpblk10w5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk10w5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk10W5Val {
    bits: u32,
}

impl Apb2otpBlk10W5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK10_W5** `[31:0]`
    ///
    /// Otp block10 word5 data.
    #[inline(always)]
    pub const fn apb2otp_block10_w5(&self) -> u32 {
        (self.bits & Apb2otpBlk10W5::APB2OTP_BLOCK10_W5_MASK)
            >> Apb2otpBlk10W5::APB2OTP_BLOCK10_W5_OFFSET
    }
}

impl Default for Apb2otpBlk10W5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk10W5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk10W5Val {
        Apb2otpBlk10W5Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK10_W6 Register
///
/// eFuse apb2otp block10 data register6.
///
/// Address offset: 0x09ec
///
/// ## Fields
///
/// - **APB2OTP_BLOCK10_W6** `[31:0]` (RO)
///
///   Otp block10 word6 data.
#[repr(transparent)]
pub struct Apb2otpBlk10W6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk10W6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk10W6 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK10_W6**
    ///
    /// Otp block10 word6 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK10_W6_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK10_W6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK10_W6** `[31:0]` (RO)
    ///
    /// Otp block10 word6 data.
    #[inline(always)]
    pub fn apb2otp_block10_w6(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK10_W6_MASK) >> Self::APB2OTP_BLOCK10_W6_OFFSET
    }
}

/// APB2OTP_BLK10_W6 value type for batch read/modify/write operations
///
/// Use `apb2otpblk10w6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk10w6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk10W6Val {
    bits: u32,
}

impl Apb2otpBlk10W6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK10_W6** `[31:0]`
    ///
    /// Otp block10 word6 data.
    #[inline(always)]
    pub const fn apb2otp_block10_w6(&self) -> u32 {
        (self.bits & Apb2otpBlk10W6::APB2OTP_BLOCK10_W6_MASK)
            >> Apb2otpBlk10W6::APB2OTP_BLOCK10_W6_OFFSET
    }
}

impl Default for Apb2otpBlk10W6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk10W6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk10W6Val {
        Apb2otpBlk10W6Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK10_W7 Register
///
/// eFuse apb2otp block10 data register7.
///
/// Address offset: 0x09f0
///
/// ## Fields
///
/// - **APB2OTP_BLOCK10_W7** `[31:0]` (RO)
///
///   Otp block10 word7 data.
#[repr(transparent)]
pub struct Apb2otpBlk10W7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk10W7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk10W7 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK10_W7**
    ///
    /// Otp block10 word7 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK10_W7_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK10_W7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK10_W7** `[31:0]` (RO)
    ///
    /// Otp block10 word7 data.
    #[inline(always)]
    pub fn apb2otp_block10_w7(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK10_W7_MASK) >> Self::APB2OTP_BLOCK10_W7_OFFSET
    }
}

/// APB2OTP_BLK10_W7 value type for batch read/modify/write operations
///
/// Use `apb2otpblk10w7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk10w7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk10W7Val {
    bits: u32,
}

impl Apb2otpBlk10W7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK10_W7** `[31:0]`
    ///
    /// Otp block10 word7 data.
    #[inline(always)]
    pub const fn apb2otp_block10_w7(&self) -> u32 {
        (self.bits & Apb2otpBlk10W7::APB2OTP_BLOCK10_W7_MASK)
            >> Apb2otpBlk10W7::APB2OTP_BLOCK10_W7_OFFSET
    }
}

impl Default for Apb2otpBlk10W7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk10W7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk10W7Val {
        Apb2otpBlk10W7Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK10_W8 Register
///
/// eFuse apb2otp block10 data register8.
///
/// Address offset: 0x09f4
///
/// ## Fields
///
/// - **APB2OTP_BLOCK10_W8** `[31:0]` (RO)
///
///   Otp block10 word8 data.
#[repr(transparent)]
pub struct Apb2otpBlk10W8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk10W8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk10W8 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK10_W8**
    ///
    /// Otp block10 word8 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK10_W8_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK10_W8_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK10_W8** `[31:0]` (RO)
    ///
    /// Otp block10 word8 data.
    #[inline(always)]
    pub fn apb2otp_block10_w8(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK10_W8_MASK) >> Self::APB2OTP_BLOCK10_W8_OFFSET
    }
}

/// APB2OTP_BLK10_W8 value type for batch read/modify/write operations
///
/// Use `apb2otpblk10w8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk10w8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk10W8Val {
    bits: u32,
}

impl Apb2otpBlk10W8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK10_W8** `[31:0]`
    ///
    /// Otp block10 word8 data.
    #[inline(always)]
    pub const fn apb2otp_block10_w8(&self) -> u32 {
        (self.bits & Apb2otpBlk10W8::APB2OTP_BLOCK10_W8_MASK)
            >> Apb2otpBlk10W8::APB2OTP_BLOCK10_W8_OFFSET
    }
}

impl Default for Apb2otpBlk10W8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk10W8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk10W8Val {
        Apb2otpBlk10W8Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK10_W9 Register
///
/// eFuse apb2otp block10 data register9.
///
/// Address offset: 0x09f8
///
/// ## Fields
///
/// - **APB2OTP_BLOCK10_W9** `[31:0]` (RO)
///
///   Otp block10 word9 data.
#[repr(transparent)]
pub struct Apb2otpBlk10W9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk10W9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk10W9 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK10_W9**
    ///
    /// Otp block10 word9 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK10_W9_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK10_W9_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK10_W9** `[31:0]` (RO)
    ///
    /// Otp block10 word9 data.
    #[inline(always)]
    pub fn apb2otp_block10_w9(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK10_W9_MASK) >> Self::APB2OTP_BLOCK10_W9_OFFSET
    }
}

/// APB2OTP_BLK10_W9 value type for batch read/modify/write operations
///
/// Use `apb2otpblk10w9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk10w9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk10W9Val {
    bits: u32,
}

impl Apb2otpBlk10W9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK10_W9** `[31:0]`
    ///
    /// Otp block10 word9 data.
    #[inline(always)]
    pub const fn apb2otp_block10_w9(&self) -> u32 {
        (self.bits & Apb2otpBlk10W9::APB2OTP_BLOCK10_W9_MASK)
            >> Apb2otpBlk10W9::APB2OTP_BLOCK10_W9_OFFSET
    }
}

impl Default for Apb2otpBlk10W9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk10W9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk10W9Val {
        Apb2otpBlk10W9Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK10_W10 Register
///
/// eFuse apb2otp block10 data register10.
///
/// Address offset: 0x09fc
///
/// ## Fields
///
/// - **APB2OTP_BLOCK19_W10** `[31:0]` (RO)
///
///   Otp block10 word10 data.
#[repr(transparent)]
pub struct Apb2otpBlk10W10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk10W10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk10W10 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK19_W10**
    ///
    /// Otp block10 word10 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK19_W10_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK19_W10_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK19_W10** `[31:0]` (RO)
    ///
    /// Otp block10 word10 data.
    #[inline(always)]
    pub fn apb2otp_block19_w10(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK19_W10_MASK)
            >> Self::APB2OTP_BLOCK19_W10_OFFSET
    }
}

/// APB2OTP_BLK10_W10 value type for batch read/modify/write operations
///
/// Use `apb2otpblk10w10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk10w10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk10W10Val {
    bits: u32,
}

impl Apb2otpBlk10W10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK19_W10** `[31:0]`
    ///
    /// Otp block10 word10 data.
    #[inline(always)]
    pub const fn apb2otp_block19_w10(&self) -> u32 {
        (self.bits & Apb2otpBlk10W10::APB2OTP_BLOCK19_W10_MASK)
            >> Apb2otpBlk10W10::APB2OTP_BLOCK19_W10_OFFSET
    }
}

impl Default for Apb2otpBlk10W10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk10W10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk10W10Val {
        Apb2otpBlk10W10Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_BLK10_W11 Register
///
/// eFuse apb2otp block10 data register11.
///
/// Address offset: 0x0a00
///
/// ## Fields
///
/// - **APB2OTP_BLOCK10_W11** `[31:0]` (RO)
///
///   Otp block10 word11 data.
#[repr(transparent)]
pub struct Apb2otpBlk10W11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpBlk10W11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpBlk10W11 {
    // ======== Field Constants ========

    /// **APB2OTP_BLOCK10_W11**
    ///
    /// Otp block10 word11 data.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const APB2OTP_BLOCK10_W11_OFFSET: u32 = 0;
    pub const APB2OTP_BLOCK10_W11_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **APB2OTP_BLOCK10_W11** `[31:0]` (RO)
    ///
    /// Otp block10 word11 data.
    #[inline(always)]
    pub fn apb2otp_block10_w11(&self) -> u32 {
        (self.value.read_bits() & Self::APB2OTP_BLOCK10_W11_MASK)
            >> Self::APB2OTP_BLOCK10_W11_OFFSET
    }
}

/// APB2OTP_BLK10_W11 value type for batch read/modify/write operations
///
/// Use `apb2otpblk10w11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpblk10w11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpBlk10W11Val {
    bits: u32,
}

impl Apb2otpBlk10W11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_BLOCK10_W11** `[31:0]`
    ///
    /// Otp block10 word11 data.
    #[inline(always)]
    pub const fn apb2otp_block10_w11(&self) -> u32 {
        (self.bits & Apb2otpBlk10W11::APB2OTP_BLOCK10_W11_MASK)
            >> Apb2otpBlk10W11::APB2OTP_BLOCK10_W11_OFFSET
    }
}

impl Default for Apb2otpBlk10W11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpBlk10W11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpBlk10W11Val {
        Apb2otpBlk10W11Val::from_bits(self.value.read_bits())
    }
}

/// APB2OTP_EN Register
///
/// eFuse apb2otp enable configuration register.
///
/// Address offset: 0x0a08
///
/// ## Fields
///
/// - **APB2OTP_APB2OTP_EN** `[0]` (RW)
///
///   Apb2otp mode enable signal.
#[repr(transparent)]
pub struct Apb2otpEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Apb2otpEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Apb2otpEn {
    // ======== Field Constants ========

    /// **APB2OTP_APB2OTP_EN**
    ///
    /// Apb2otp mode enable signal.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const APB2OTP_APB2OTP_EN_OFFSET: u32 = 0;
    pub const APB2OTP_APB2OTP_EN_MASK: u32 = 0x00000001;
    pub const APB2OTP_APB2OTP_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **APB2OTP_APB2OTP_EN** `[0]` (RW)
    ///
    /// Apb2otp mode enable signal.
    #[inline(always)]
    pub fn apb2otp_apb2otp_en(&self) -> bool {
        (self.value.read_bits() & Self::APB2OTP_APB2OTP_EN_MASK) != 0
    }

    /// Set APB2OTP_APB2OTP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_apb2otp_apb2otp_en(&self) {
        self.value
            .modify_bits(|r| r | Self::APB2OTP_APB2OTP_EN_MASK);
    }

    /// Clear APB2OTP_APB2OTP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_apb2otp_apb2otp_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::APB2OTP_APB2OTP_EN_MASK);
    }
}

/// APB2OTP_EN value type for batch read/modify/write operations
///
/// Use `apb2otpen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apb2otpen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Apb2otpEnVal {
    bits: u32,
}

impl Apb2otpEnVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **APB2OTP_APB2OTP_EN** `[0:0]`
    ///
    /// Apb2otp mode enable signal.
    #[inline(always)]
    pub const fn apb2otp_apb2otp_en(&self) -> bool {
        (self.bits & Apb2otpEn::APB2OTP_APB2OTP_EN_MASK) != 0
    }

    /// Set **APB2OTP_APB2OTP_EN** `[0:0]`
    ///
    /// Apb2otp mode enable signal.
    #[inline(always)]
    pub const fn with_apb2otp_apb2otp_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Apb2otpEn::APB2OTP_APB2OTP_EN_MASK
            } else {
                self.bits & !Apb2otpEn::APB2OTP_APB2OTP_EN_MASK
            },
        }
    }
}

impl Default for Apb2otpEnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Apb2otpEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Apb2otpEnVal {
        Apb2otpEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Apb2otpEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Apb2otpEnVal) -> Apb2otpEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Register 0 that stores data to be programmed.
    pub pgm_data0: PgmData0,
    /// 0x0004
    /// Register 1 that stores data to be programmed.
    pub pgm_data1: PgmData1,
    /// 0x0008
    /// Register 2 that stores data to be programmed.
    pub pgm_data2: PgmData2,
    /// 0x000c
    /// Register 3 that stores data to be programmed.
    pub pgm_data3: PgmData3,
    /// 0x0010
    /// Register 4 that stores data to be programmed.
    pub pgm_data4: PgmData4,
    /// 0x0014
    /// Register 5 that stores data to be programmed.
    pub pgm_data5: PgmData5,
    /// 0x0018
    /// Register 6 that stores data to be programmed.
    pub pgm_data6: PgmData6,
    /// 0x001c
    /// Register 7 that stores data to be programmed.
    pub pgm_data7: PgmData7,
    /// 0x0020
    /// Register 0 that stores the RS code to be programmed.
    pub pgm_check_value0: PgmCheckValue0,
    /// 0x0024
    /// Register 1 that stores the RS code to be programmed.
    pub pgm_check_value1: PgmCheckValue1,
    /// 0x0028
    /// Register 2 that stores the RS code to be programmed.
    pub pgm_check_value2: PgmCheckValue2,
    /// 0x002c
    /// BLOCK0 data register 0.
    pub rd_wr_dis: RdWrDis,
    /// 0x0030
    /// BLOCK0 data register 1.
    pub rd_repeat_data0: RdRepeatData0,
    /// 0x0034
    /// BLOCK0 data register 2.
    pub rd_repeat_data1: RdRepeatData1,
    /// 0x0038
    /// BLOCK0 data register 3.
    pub rd_repeat_data2: RdRepeatData2,
    /// 0x003c
    /// BLOCK0 data register 4.
    pub rd_repeat_data3: RdRepeatData3,
    /// 0x0040
    /// BLOCK0 data register 5.
    pub rd_repeat_data4: RdRepeatData4,
    /// 0x0044
    /// BLOCK1 data register $n.
    pub rd_mac_sys0: RdMacSys0,
    /// 0x0048
    /// BLOCK1 data register $n.
    pub rd_mac_sys1: RdMacSys1,
    /// 0x004c
    /// BLOCK1 data register $n.
    pub rd_mac_sys2: RdMacSys2,
    /// 0x0050
    /// BLOCK1 data register $n.
    pub rd_mac_sys3: RdMacSys3,
    /// 0x0054
    /// BLOCK1 data register $n.
    pub rd_mac_sys4: RdMacSys4,
    /// 0x0058
    /// BLOCK1 data register $n.
    pub rd_mac_sys5: RdMacSys5,
    /// 0x005c
    /// Register $n of BLOCK2 (system).
    pub rd_sys_part1_data0: RdSysPart1Data0,
    /// 0x0060
    /// Register $n of BLOCK2 (system).
    pub rd_sys_part1_data1: RdSysPart1Data1,
    /// 0x0064
    /// Register $n of BLOCK2 (system).
    pub rd_sys_part1_data2: RdSysPart1Data2,
    /// 0x0068
    /// Register $n of BLOCK2 (system).
    pub rd_sys_part1_data3: RdSysPart1Data3,
    /// 0x006c
    /// Register $n of BLOCK2 (system).
    pub rd_sys_part1_data4: RdSysPart1Data4,
    /// 0x0070
    /// Register $n of BLOCK2 (system).
    pub rd_sys_part1_data5: RdSysPart1Data5,
    /// 0x0074
    /// Register $n of BLOCK2 (system).
    pub rd_sys_part1_data6: RdSysPart1Data6,
    /// 0x0078
    /// Register $n of BLOCK2 (system).
    pub rd_sys_part1_data7: RdSysPart1Data7,
    /// 0x007c
    /// Register $n of BLOCK3 (user).
    pub rd_usr_data0: RdUsrData0,
    /// 0x0080
    /// Register $n of BLOCK3 (user).
    pub rd_usr_data1: RdUsrData1,
    /// 0x0084
    /// Register $n of BLOCK3 (user).
    pub rd_usr_data2: RdUsrData2,
    /// 0x0088
    /// Register $n of BLOCK3 (user).
    pub rd_usr_data3: RdUsrData3,
    /// 0x008c
    /// Register $n of BLOCK3 (user).
    pub rd_usr_data4: RdUsrData4,
    /// 0x0090
    /// Register $n of BLOCK3 (user).
    pub rd_usr_data5: RdUsrData5,
    /// 0x0094
    /// Register $n of BLOCK3 (user).
    pub rd_usr_data6: RdUsrData6,
    /// 0x0098
    /// Register $n of BLOCK3 (user).
    pub rd_usr_data7: RdUsrData7,
    /// 0x009c
    /// Register $n of BLOCK4 (KEY0).
    pub rd_key0_data0: RdKey0Data0,
    /// 0x00a0
    /// Register $n of BLOCK4 (KEY0).
    pub rd_key0_data1: RdKey0Data1,
    /// 0x00a4
    /// Register $n of BLOCK4 (KEY0).
    pub rd_key0_data2: RdKey0Data2,
    /// 0x00a8
    /// Register $n of BLOCK4 (KEY0).
    pub rd_key0_data3: RdKey0Data3,
    /// 0x00ac
    /// Register $n of BLOCK4 (KEY0).
    pub rd_key0_data4: RdKey0Data4,
    /// 0x00b0
    /// Register $n of BLOCK4 (KEY0).
    pub rd_key0_data5: RdKey0Data5,
    /// 0x00b4
    /// Register $n of BLOCK4 (KEY0).
    pub rd_key0_data6: RdKey0Data6,
    /// 0x00b8
    /// Register $n of BLOCK4 (KEY0).
    pub rd_key0_data7: RdKey0Data7,
    /// 0x00bc
    /// Register $n of BLOCK5 (KEY1).
    pub rd_key1_data0: RdKey1Data0,
    /// 0x00c0
    /// Register $n of BLOCK5 (KEY1).
    pub rd_key1_data1: RdKey1Data1,
    /// 0x00c4
    /// Register $n of BLOCK5 (KEY1).
    pub rd_key1_data2: RdKey1Data2,
    /// 0x00c8
    /// Register $n of BLOCK5 (KEY1).
    pub rd_key1_data3: RdKey1Data3,
    /// 0x00cc
    /// Register $n of BLOCK5 (KEY1).
    pub rd_key1_data4: RdKey1Data4,
    /// 0x00d0
    /// Register $n of BLOCK5 (KEY1).
    pub rd_key1_data5: RdKey1Data5,
    /// 0x00d4
    /// Register $n of BLOCK5 (KEY1).
    pub rd_key1_data6: RdKey1Data6,
    /// 0x00d8
    /// Register $n of BLOCK5 (KEY1).
    pub rd_key1_data7: RdKey1Data7,
    /// 0x00dc
    /// Register $n of BLOCK6 (KEY2).
    pub rd_key2_data0: RdKey2Data0,
    /// 0x00e0
    /// Register $n of BLOCK6 (KEY2).
    pub rd_key2_data1: RdKey2Data1,
    /// 0x00e4
    /// Register $n of BLOCK6 (KEY2).
    pub rd_key2_data2: RdKey2Data2,
    /// 0x00e8
    /// Register $n of BLOCK6 (KEY2).
    pub rd_key2_data3: RdKey2Data3,
    /// 0x00ec
    /// Register $n of BLOCK6 (KEY2).
    pub rd_key2_data4: RdKey2Data4,
    /// 0x00f0
    /// Register $n of BLOCK6 (KEY2).
    pub rd_key2_data5: RdKey2Data5,
    /// 0x00f4
    /// Register $n of BLOCK6 (KEY2).
    pub rd_key2_data6: RdKey2Data6,
    /// 0x00f8
    /// Register $n of BLOCK6 (KEY2).
    pub rd_key2_data7: RdKey2Data7,
    /// 0x00fc
    /// Register $n of BLOCK7 (KEY3).
    pub rd_key3_data0: RdKey3Data0,
    /// 0x0100
    /// Register $n of BLOCK7 (KEY3).
    pub rd_key3_data1: RdKey3Data1,
    /// 0x0104
    /// Register $n of BLOCK7 (KEY3).
    pub rd_key3_data2: RdKey3Data2,
    /// 0x0108
    /// Register $n of BLOCK7 (KEY3).
    pub rd_key3_data3: RdKey3Data3,
    /// 0x010c
    /// Register $n of BLOCK7 (KEY3).
    pub rd_key3_data4: RdKey3Data4,
    /// 0x0110
    /// Register $n of BLOCK7 (KEY3).
    pub rd_key3_data5: RdKey3Data5,
    /// 0x0114
    /// Register $n of BLOCK7 (KEY3).
    pub rd_key3_data6: RdKey3Data6,
    /// 0x0118
    /// Register $n of BLOCK7 (KEY3).
    pub rd_key3_data7: RdKey3Data7,
    /// 0x011c
    /// Register $n of BLOCK8 (KEY4).
    pub rd_key4_data0: RdKey4Data0,
    /// 0x0120
    /// Register $n of BLOCK8 (KEY4).
    pub rd_key4_data1: RdKey4Data1,
    /// 0x0124
    /// Register $n of BLOCK8 (KEY4).
    pub rd_key4_data2: RdKey4Data2,
    /// 0x0128
    /// Register $n of BLOCK8 (KEY4).
    pub rd_key4_data3: RdKey4Data3,
    /// 0x012c
    /// Register $n of BLOCK8 (KEY4).
    pub rd_key4_data4: RdKey4Data4,
    /// 0x0130
    /// Register $n of BLOCK8 (KEY4).
    pub rd_key4_data5: RdKey4Data5,
    /// 0x0134
    /// Register $n of BLOCK8 (KEY4).
    pub rd_key4_data6: RdKey4Data6,
    /// 0x0138
    /// Register $n of BLOCK8 (KEY4).
    pub rd_key4_data7: RdKey4Data7,
    /// 0x013c
    /// Register $n of BLOCK9 (KEY5).
    pub rd_key5_data0: RdKey5Data0,
    /// 0x0140
    /// Register $n of BLOCK9 (KEY5).
    pub rd_key5_data1: RdKey5Data1,
    /// 0x0144
    /// Register $n of BLOCK9 (KEY5).
    pub rd_key5_data2: RdKey5Data2,
    /// 0x0148
    /// Register $n of BLOCK9 (KEY5).
    pub rd_key5_data3: RdKey5Data3,
    /// 0x014c
    /// Register $n of BLOCK9 (KEY5).
    pub rd_key5_data4: RdKey5Data4,
    /// 0x0150
    /// Register $n of BLOCK9 (KEY5).
    pub rd_key5_data5: RdKey5Data5,
    /// 0x0154
    /// Register $n of BLOCK9 (KEY5).
    pub rd_key5_data6: RdKey5Data6,
    /// 0x0158
    /// Register $n of BLOCK9 (KEY5).
    pub rd_key5_data7: RdKey5Data7,
    /// 0x015c
    /// Register $n of BLOCK10 (system).
    pub rd_sys_part2_data0: RdSysPart2Data0,
    /// 0x0160
    /// Register $n of BLOCK9 (KEY5).
    pub rd_sys_part2_data1: RdSysPart2Data1,
    /// 0x0164
    /// Register $n of BLOCK10 (system).
    pub rd_sys_part2_data2: RdSysPart2Data2,
    /// 0x0168
    /// Register $n of BLOCK10 (system).
    pub rd_sys_part2_data3: RdSysPart2Data3,
    /// 0x016c
    /// Register $n of BLOCK10 (system).
    pub rd_sys_part2_data4: RdSysPart2Data4,
    /// 0x0170
    /// Register $n of BLOCK10 (system).
    pub rd_sys_part2_data5: RdSysPart2Data5,
    /// 0x0174
    /// Register $n of BLOCK10 (system).
    pub rd_sys_part2_data6: RdSysPart2Data6,
    /// 0x0178
    /// Register $n of BLOCK10 (system).
    pub rd_sys_part2_data7: RdSysPart2Data7,
    /// 0x017c
    /// Programming error record register 0 of BLOCK0.
    pub rd_repeat_err0: RdRepeatErr0,
    /// 0x0180
    /// Programming error record register 1 of BLOCK0.
    pub rd_repeat_err1: RdRepeatErr1,
    /// 0x0184
    /// Programming error record register 2 of BLOCK0.
    pub rd_repeat_err2: RdRepeatErr2,
    /// 0x0188
    /// Programming error record register 3 of BLOCK0.
    pub rd_repeat_err3: RdRepeatErr3,
    /// 0x018c
    /// Programming error record register 4 of BLOCK0.
    pub rd_repeat_err4: RdRepeatErr4,
    _reserved0: [u8; 48],
    /// 0x01c0
    /// Programming error record register 0 of BLOCK1-10.
    pub rd_rs_err0: RdRsErr0,
    /// 0x01c4
    /// Programming error record register 1 of BLOCK1-10.
    pub rd_rs_err1: RdRsErr1,
    /// 0x01c8
    /// eFuse clcok configuration register.
    pub clk: Clk,
    /// 0x01cc
    /// eFuse operation mode configuraiton register
    pub conf: Conf,
    /// 0x01d0
    /// eFuse status register.
    pub status: Status,
    /// 0x01d4
    /// eFuse command register.
    pub cmd: Cmd,
    /// 0x01d8
    /// eFuse raw interrupt register.
    pub int_raw: IntRaw,
    /// 0x01dc
    /// eFuse interrupt status register.
    pub int_st: IntSt,
    /// 0x01e0
    /// eFuse interrupt enable register.
    pub int_ena: IntEna,
    /// 0x01e4
    /// eFuse interrupt clear register.
    pub int_clr: IntClr,
    /// 0x01e8
    /// Controls the eFuse programming voltage.
    pub dac_conf: DacConf,
    /// 0x01ec
    /// Configures read timing parameters.
    pub rd_tim_conf: RdTimConf,
    /// 0x01f0
    /// Configurarion register 1 of eFuse programming timing parameters.
    pub wr_tim_conf1: WrTimConf1,
    /// 0x01f4
    /// Configurarion register 2 of eFuse programming timing parameters.
    pub wr_tim_conf2: WrTimConf2,
    /// 0x01f8
    /// Configurarion register0 of eFuse programming time parameters and rs bypass operation.
    pub wr_tim_conf0_rs_bypass: WrTimConf0RsBypass,
    /// 0x01fc
    /// eFuse version register.
    pub date: Date,
    _reserved1: [u8; 1536],
    /// 0x0800
    /// eFuse apb2otp block0 data register1.
    pub apb2otp_wr_dis: Apb2otpWrDis,
    /// 0x0804
    /// eFuse apb2otp block0 data register2.
    pub apb2otp_blk0_backup1_w1: Apb2otpBlk0Backup1W1,
    /// 0x0808
    /// eFuse apb2otp block0 data register3.
    pub apb2otp_blk0_backup1_w2: Apb2otpBlk0Backup1W2,
    /// 0x080c
    /// eFuse apb2otp block0 data register4.
    pub apb2otp_blk0_backup1_w3: Apb2otpBlk0Backup1W3,
    /// 0x0810
    /// eFuse apb2otp block0 data register5.
    pub apb2otp_blk0_backup1_w4: Apb2otpBlk0Backup1W4,
    /// 0x0814
    /// eFuse apb2otp block0 data register6.
    pub apb2otp_blk0_backup1_w5: Apb2otpBlk0Backup1W5,
    /// 0x0818
    /// eFuse apb2otp block0 data register7.
    pub apb2otp_blk0_backup2_w1: Apb2otpBlk0Backup2W1,
    /// 0x081c
    /// eFuse apb2otp block0 data register8.
    pub apb2otp_blk0_backup2_w2: Apb2otpBlk0Backup2W2,
    /// 0x0820
    /// eFuse apb2otp block0 data register9.
    pub apb2otp_blk0_backup2_w3: Apb2otpBlk0Backup2W3,
    /// 0x0824
    /// eFuse apb2otp block0 data register10.
    pub apb2otp_blk0_backup2_w4: Apb2otpBlk0Backup2W4,
    /// 0x0828
    /// eFuse apb2otp block0 data register11.
    pub apb2otp_blk0_backup2_w5: Apb2otpBlk0Backup2W5,
    /// 0x082c
    /// eFuse apb2otp block0 data register12.
    pub apb2otp_blk0_backup3_w1: Apb2otpBlk0Backup3W1,
    /// 0x0830
    /// eFuse apb2otp block0 data register13.
    pub apb2otp_blk0_backup3_w2: Apb2otpBlk0Backup3W2,
    /// 0x0834
    /// eFuse apb2otp block0 data register14.
    pub apb2otp_blk0_backup3_w3: Apb2otpBlk0Backup3W3,
    /// 0x0838
    /// eFuse apb2otp block0 data register15.
    pub apb2otp_blk0_backup3_w4: Apb2otpBlk0Backup3W4,
    /// 0x083c
    /// eFuse apb2otp block0 data register16.
    pub apb2otp_blk0_backup3_w5: Apb2otpBlk0Backup3W5,
    /// 0x0840
    /// eFuse apb2otp block0 data register17.
    pub apb2otp_blk0_backup4_w1: Apb2otpBlk0Backup4W1,
    /// 0x0844
    /// eFuse apb2otp block0 data register18.
    pub apb2otp_blk0_backup4_w2: Apb2otpBlk0Backup4W2,
    /// 0x0848
    /// eFuse apb2otp block0 data register19.
    pub apb2otp_blk0_backup4_w3: Apb2otpBlk0Backup4W3,
    /// 0x084c
    /// eFuse apb2otp block0 data register20.
    pub apb2otp_blk0_backup4_w4: Apb2otpBlk0Backup4W4,
    /// 0x0850
    /// eFuse apb2otp block0 data register21.
    pub apb2otp_blk0_backup4_w5: Apb2otpBlk0Backup4W5,
    /// 0x0854
    /// eFuse apb2otp block1 data register1.
    pub apb2otp_blk1_w1: Apb2otpBlk1W1,
    /// 0x0858
    /// eFuse apb2otp block1 data register2.
    pub apb2otp_blk1_w2: Apb2otpBlk1W2,
    /// 0x085c
    /// eFuse apb2otp block1 data register3.
    pub apb2otp_blk1_w3: Apb2otpBlk1W3,
    /// 0x0860
    /// eFuse apb2otp block1 data register4.
    pub apb2otp_blk1_w4: Apb2otpBlk1W4,
    /// 0x0864
    /// eFuse apb2otp block1 data register5.
    pub apb2otp_blk1_w5: Apb2otpBlk1W5,
    /// 0x0868
    /// eFuse apb2otp block1 data register6.
    pub apb2otp_blk1_w6: Apb2otpBlk1W6,
    /// 0x086c
    /// eFuse apb2otp block1 data register7.
    pub apb2otp_blk1_w7: Apb2otpBlk1W7,
    /// 0x0870
    /// eFuse apb2otp block1 data register8.
    pub apb2otp_blk1_w8: Apb2otpBlk1W8,
    /// 0x0874
    /// eFuse apb2otp block1 data register9.
    pub apb2otp_blk1_w9: Apb2otpBlk1W9,
    /// 0x0878
    /// eFuse apb2otp block2 data register1.
    pub apb2otp_blk2_w1: Apb2otpBlk2W1,
    /// 0x087c
    /// eFuse apb2otp block2 data register2.
    pub apb2otp_blk2_w2: Apb2otpBlk2W2,
    /// 0x0880
    /// eFuse apb2otp block2 data register3.
    pub apb2otp_blk2_w3: Apb2otpBlk2W3,
    /// 0x0884
    /// eFuse apb2otp block2 data register4.
    pub apb2otp_blk2_w4: Apb2otpBlk2W4,
    /// 0x0888
    /// eFuse apb2otp block2 data register5.
    pub apb2otp_blk2_w5: Apb2otpBlk2W5,
    /// 0x088c
    /// eFuse apb2otp block2 data register6.
    pub apb2otp_blk2_w6: Apb2otpBlk2W6,
    /// 0x0890
    /// eFuse apb2otp block2 data register7.
    pub apb2otp_blk2_w7: Apb2otpBlk2W7,
    /// 0x0894
    /// eFuse apb2otp block2 data register8.
    pub apb2otp_blk2_w8: Apb2otpBlk2W8,
    /// 0x0898
    /// eFuse apb2otp block2 data register9.
    pub apb2otp_blk2_w9: Apb2otpBlk2W9,
    /// 0x089c
    /// eFuse apb2otp block2 data register10.
    pub apb2otp_blk2_w10: Apb2otpBlk2W10,
    /// 0x08a0
    /// eFuse apb2otp block2 data register11.
    pub apb2otp_blk2_w11: Apb2otpBlk2W11,
    /// 0x08a4
    /// eFuse apb2otp block3 data register1.
    pub apb2otp_blk3_w1: Apb2otpBlk3W1,
    /// 0x08a8
    /// eFuse apb2otp block3 data register2.
    pub apb2otp_blk3_w2: Apb2otpBlk3W2,
    /// 0x08ac
    /// eFuse apb2otp block3 data register3.
    pub apb2otp_blk3_w3: Apb2otpBlk3W3,
    /// 0x08b0
    /// eFuse apb2otp block3 data register4.
    pub apb2otp_blk3_w4: Apb2otpBlk3W4,
    /// 0x08b4
    /// eFuse apb2otp block3 data register5.
    pub apb2otp_blk3_w5: Apb2otpBlk3W5,
    /// 0x08b8
    /// eFuse apb2otp block3 data register6.
    pub apb2otp_blk3_w6: Apb2otpBlk3W6,
    /// 0x08bc
    /// eFuse apb2otp block3 data register7.
    pub apb2otp_blk3_w7: Apb2otpBlk3W7,
    /// 0x08c0
    /// eFuse apb2otp block3 data register8.
    pub apb2otp_blk3_w8: Apb2otpBlk3W8,
    /// 0x08c4
    /// eFuse apb2otp block3 data register9.
    pub apb2otp_blk3_w9: Apb2otpBlk3W9,
    /// 0x08c8
    /// eFuse apb2otp block3 data register10.
    pub apb2otp_blk3_w10: Apb2otpBlk3W10,
    /// 0x08cc
    /// eFuse apb2otp block3 data register11.
    pub apb2otp_blk3_w11: Apb2otpBlk3W11,
    /// 0x08d0
    /// eFuse apb2otp block4 data register1.
    pub apb2otp_blk4_w1: Apb2otpBlk4W1,
    /// 0x08d4
    /// eFuse apb2otp block4 data register2.
    pub apb2otp_blk4_w2: Apb2otpBlk4W2,
    /// 0x08d8
    /// eFuse apb2otp block4 data register3.
    pub apb2otp_blk4_w3: Apb2otpBlk4W3,
    /// 0x08dc
    /// eFuse apb2otp block4 data register4.
    pub apb2otp_blk4_w4: Apb2otpBlk4W4,
    /// 0x08e0
    /// eFuse apb2otp block4 data register5.
    pub apb2otp_blk4_w5: Apb2otpBlk4W5,
    /// 0x08e4
    /// eFuse apb2otp block4 data register6.
    pub apb2otp_blk4_w6: Apb2otpBlk4W6,
    /// 0x08e8
    /// eFuse apb2otp block4 data register7.
    pub apb2otp_blk4_w7: Apb2otpBlk4W7,
    /// 0x08ec
    /// eFuse apb2otp block4 data register8.
    pub apb2otp_blk4_w8: Apb2otpBlk4W8,
    /// 0x08f0
    /// eFuse apb2otp block4 data register9.
    pub apb2otp_blk4_w9: Apb2otpBlk4W9,
    /// 0x08f4
    /// eFuse apb2otp block4 data registe10.
    pub apb2otp_blk4_w10: Apb2otpBlk4W10,
    /// 0x08f8
    /// eFuse apb2otp block4 data register11.
    pub apb2otp_blk4_w11: Apb2otpBlk4W11,
    /// 0x08fc
    /// eFuse apb2otp block5 data register1.
    pub apb2otp_blk5_w1: Apb2otpBlk5W1,
    /// 0x0900
    /// eFuse apb2otp block5 data register2.
    pub apb2otp_blk5_w2: Apb2otpBlk5W2,
    /// 0x0904
    /// eFuse apb2otp block5 data register3.
    pub apb2otp_blk5_w3: Apb2otpBlk5W3,
    /// 0x0908
    /// eFuse apb2otp block5 data register4.
    pub apb2otp_blk5_w4: Apb2otpBlk5W4,
    /// 0x090c
    /// eFuse apb2otp block5 data register5.
    pub apb2otp_blk5_w5: Apb2otpBlk5W5,
    /// 0x0910
    /// eFuse apb2otp block5 data register6.
    pub apb2otp_blk5_w6: Apb2otpBlk5W6,
    /// 0x0914
    /// eFuse apb2otp block5 data register7.
    pub apb2otp_blk5_w7: Apb2otpBlk5W7,
    /// 0x0918
    /// eFuse apb2otp block5 data register8.
    pub apb2otp_blk5_w8: Apb2otpBlk5W8,
    /// 0x091c
    /// eFuse apb2otp block5 data register9.
    pub apb2otp_blk5_w9: Apb2otpBlk5W9,
    /// 0x0920
    /// eFuse apb2otp block5 data register10.
    pub apb2otp_blk5_w10: Apb2otpBlk5W10,
    /// 0x0924
    /// eFuse apb2otp block5 data register11.
    pub apb2otp_blk5_w11: Apb2otpBlk5W11,
    /// 0x0928
    /// eFuse apb2otp block6 data register1.
    pub apb2otp_blk6_w1: Apb2otpBlk6W1,
    /// 0x092c
    /// eFuse apb2otp block6 data register2.
    pub apb2otp_blk6_w2: Apb2otpBlk6W2,
    /// 0x0930
    /// eFuse apb2otp block6 data register3.
    pub apb2otp_blk6_w3: Apb2otpBlk6W3,
    /// 0x0934
    /// eFuse apb2otp block6 data register4.
    pub apb2otp_blk6_w4: Apb2otpBlk6W4,
    /// 0x0938
    /// eFuse apb2otp block6 data register5.
    pub apb2otp_blk6_w5: Apb2otpBlk6W5,
    /// 0x093c
    /// eFuse apb2otp block6 data register6.
    pub apb2otp_blk6_w6: Apb2otpBlk6W6,
    /// 0x0940
    /// eFuse apb2otp block6 data register7.
    pub apb2otp_blk6_w7: Apb2otpBlk6W7,
    /// 0x0944
    /// eFuse apb2otp block6 data register8.
    pub apb2otp_blk6_w8: Apb2otpBlk6W8,
    /// 0x0948
    /// eFuse apb2otp block6 data register9.
    pub apb2otp_blk6_w9: Apb2otpBlk6W9,
    /// 0x094c
    /// eFuse apb2otp block6 data register10.
    pub apb2otp_blk6_w10: Apb2otpBlk6W10,
    /// 0x0950
    /// eFuse apb2otp block6 data register11.
    pub apb2otp_blk6_w11: Apb2otpBlk6W11,
    /// 0x0954
    /// eFuse apb2otp block7 data register1.
    pub apb2otp_blk7_w1: Apb2otpBlk7W1,
    /// 0x0958
    /// eFuse apb2otp block7 data register2.
    pub apb2otp_blk7_w2: Apb2otpBlk7W2,
    /// 0x095c
    /// eFuse apb2otp block7 data register3.
    pub apb2otp_blk7_w3: Apb2otpBlk7W3,
    /// 0x0960
    /// eFuse apb2otp block7 data register4.
    pub apb2otp_blk7_w4: Apb2otpBlk7W4,
    /// 0x0964
    /// eFuse apb2otp block7 data register5.
    pub apb2otp_blk7_w5: Apb2otpBlk7W5,
    /// 0x0968
    /// eFuse apb2otp block7 data register6.
    pub apb2otp_blk7_w6: Apb2otpBlk7W6,
    /// 0x096c
    /// eFuse apb2otp block7 data register7.
    pub apb2otp_blk7_w7: Apb2otpBlk7W7,
    /// 0x0970
    /// eFuse apb2otp block7 data register8.
    pub apb2otp_blk7_w8: Apb2otpBlk7W8,
    /// 0x0974
    /// eFuse apb2otp block7 data register9.
    pub apb2otp_blk7_w9: Apb2otpBlk7W9,
    /// 0x0978
    /// eFuse apb2otp block7 data register10.
    pub apb2otp_blk7_w10: Apb2otpBlk7W10,
    /// 0x097c
    /// eFuse apb2otp block7 data register11.
    pub apb2otp_blk7_w11: Apb2otpBlk7W11,
    /// 0x0980
    /// eFuse apb2otp block8 data register1.
    pub apb2otp_blk8_w1: Apb2otpBlk8W1,
    /// 0x0984
    /// eFuse apb2otp block8 data register2.
    pub apb2otp_blk8_w2: Apb2otpBlk8W2,
    /// 0x0988
    /// eFuse apb2otp block8 data register3.
    pub apb2otp_blk8_w3: Apb2otpBlk8W3,
    /// 0x098c
    /// eFuse apb2otp block8 data register4.
    pub apb2otp_blk8_w4: Apb2otpBlk8W4,
    /// 0x0990
    /// eFuse apb2otp block8 data register5.
    pub apb2otp_blk8_w5: Apb2otpBlk8W5,
    /// 0x0994
    /// eFuse apb2otp block8 data register6.
    pub apb2otp_blk8_w6: Apb2otpBlk8W6,
    /// 0x0998
    /// eFuse apb2otp block8 data register7.
    pub apb2otp_blk8_w7: Apb2otpBlk8W7,
    /// 0x099c
    /// eFuse apb2otp block8 data register8.
    pub apb2otp_blk8_w8: Apb2otpBlk8W8,
    /// 0x09a0
    /// eFuse apb2otp block8 data register9.
    pub apb2otp_blk8_w9: Apb2otpBlk8W9,
    /// 0x09a4
    /// eFuse apb2otp block8 data register10.
    pub apb2otp_blk8_w10: Apb2otpBlk8W10,
    /// 0x09a8
    /// eFuse apb2otp block8 data register11.
    pub apb2otp_blk8_w11: Apb2otpBlk8W11,
    /// 0x09ac
    /// eFuse apb2otp block9 data register1.
    pub apb2otp_blk9_w1: Apb2otpBlk9W1,
    /// 0x09b0
    /// eFuse apb2otp block9 data register2.
    pub apb2otp_blk9_w2: Apb2otpBlk9W2,
    /// 0x09b4
    /// eFuse apb2otp block9 data register3.
    pub apb2otp_blk9_w3: Apb2otpBlk9W3,
    /// 0x09b8
    /// eFuse apb2otp block9 data register4.
    pub apb2otp_blk9_w4: Apb2otpBlk9W4,
    /// 0x09bc
    /// eFuse apb2otp block9 data register5.
    pub apb2otp_blk9_w5: Apb2otpBlk9W5,
    /// 0x09c0
    /// eFuse apb2otp block9 data register6.
    pub apb2otp_blk9_w6: Apb2otpBlk9W6,
    /// 0x09c4
    /// eFuse apb2otp block9 data register7.
    pub apb2otp_blk9_w7: Apb2otpBlk9W7,
    /// 0x09c8
    /// eFuse apb2otp block9 data register8.
    pub apb2otp_blk9_w8: Apb2otpBlk9W8,
    /// 0x09cc
    /// eFuse apb2otp block9 data register9.
    pub apb2otp_blk9_w9: Apb2otpBlk9W9,
    /// 0x09d0
    /// eFuse apb2otp block9 data register10.
    pub apb2otp_blk9_w10: Apb2otpBlk9W10,
    /// 0x09d4
    /// eFuse apb2otp block9 data register11.
    pub apb2otp_blk9_w11: Apb2otpBlk9W11,
    /// 0x09d8
    /// eFuse apb2otp block10 data register1.
    pub apb2otp_blk10_w1: Apb2otpBlk10W1,
    /// 0x09dc
    /// eFuse apb2otp block10 data register2.
    pub apb2otp_blk10_w2: Apb2otpBlk10W2,
    /// 0x09e0
    /// eFuse apb2otp block10 data register3.
    pub apb2otp_blk10_w3: Apb2otpBlk10W3,
    /// 0x09e4
    /// eFuse apb2otp block10 data register4.
    pub apb2otp_blk10_w4: Apb2otpBlk10W4,
    /// 0x09e8
    /// eFuse apb2otp block10 data register5.
    pub apb2otp_blk10_w5: Apb2otpBlk10W5,
    /// 0x09ec
    /// eFuse apb2otp block10 data register6.
    pub apb2otp_blk10_w6: Apb2otpBlk10W6,
    /// 0x09f0
    /// eFuse apb2otp block10 data register7.
    pub apb2otp_blk10_w7: Apb2otpBlk10W7,
    /// 0x09f4
    /// eFuse apb2otp block10 data register8.
    pub apb2otp_blk10_w8: Apb2otpBlk10W8,
    /// 0x09f8
    /// eFuse apb2otp block10 data register9.
    pub apb2otp_blk10_w9: Apb2otpBlk10W9,
    /// 0x09fc
    /// eFuse apb2otp block10 data register10.
    pub apb2otp_blk10_w10: Apb2otpBlk10W10,
    /// 0x0a00
    /// eFuse apb2otp block10 data register11.
    pub apb2otp_blk10_w11: Apb2otpBlk10W11,
    _reserved2: [u8; 4],
    /// 0x0a08
    /// eFuse apb2otp enable configuration register.
    pub apb2otp_en: Apb2otpEn,
}
