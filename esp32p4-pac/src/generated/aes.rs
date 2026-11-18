//! AES
//! AES (Advanced Encryption Standard) Accelerator

#![allow(dead_code)]

pub const BASE: u32 = 0x50090000;

/// KEY_0 Register
///
/// Key material key_0 configure register
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **KEY_0** `[31:0]` (RW)
///
///   This bits stores key_0 that is a part of key material.
#[repr(transparent)]
pub struct Key0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Key0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key0 {
    // ======== Field Constants ========

    /// **KEY_0**
    ///
    /// This bits stores key_0 that is a part of key material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY_0_OFFSET: u32 = 0;
    pub const KEY_0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY_0** `[31:0]` (RW)
    ///
    /// This bits stores key_0 that is a part of key material.
    #[inline(always)]
    pub fn key_0(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_0_MASK) >> Self::KEY_0_OFFSET
    }

    /// Write **KEY_0** `[31:0]` (RW)
    ///
    /// This bits stores key_0 that is a part of key material.
    #[inline(always)]
    pub fn write_key_0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::KEY_0_MASK) | ((value << Self::KEY_0_OFFSET) & Self::KEY_0_MASK)
        });
    }
}

/// KEY_0 value type for batch read/modify/write operations
///
/// Use `key0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key0Val {
    bits: u32,
}

impl Key0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY_0** `[31:0]`
    ///
    /// This bits stores key_0 that is a part of key material.
    #[inline(always)]
    pub const fn key_0(&self) -> u32 {
        (self.bits & Key0::KEY_0_MASK) >> Key0::KEY_0_OFFSET
    }

    /// Set **KEY_0** `[31:0]`
    ///
    /// This bits stores key_0 that is a part of key material.
    #[inline(always)]
    pub const fn with_key_0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Key0::KEY_0_MASK)
                | ((value << Key0::KEY_0_OFFSET) & Key0::KEY_0_MASK),
        }
    }
}

impl Default for Key0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key0Val {
        Key0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key0Val) -> Key0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// KEY_1 Register
///
/// Key material key_1 configure register
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **KEY_1** `[31:0]` (RW)
///
///   This bits stores key_1 that is a part of key material.
#[repr(transparent)]
pub struct Key1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Key1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key1 {
    // ======== Field Constants ========

    /// **KEY_1**
    ///
    /// This bits stores key_1 that is a part of key material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY_1_OFFSET: u32 = 0;
    pub const KEY_1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY_1** `[31:0]` (RW)
    ///
    /// This bits stores key_1 that is a part of key material.
    #[inline(always)]
    pub fn key_1(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_1_MASK) >> Self::KEY_1_OFFSET
    }

    /// Write **KEY_1** `[31:0]` (RW)
    ///
    /// This bits stores key_1 that is a part of key material.
    #[inline(always)]
    pub fn write_key_1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::KEY_1_MASK) | ((value << Self::KEY_1_OFFSET) & Self::KEY_1_MASK)
        });
    }
}

/// KEY_1 value type for batch read/modify/write operations
///
/// Use `key1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key1Val {
    bits: u32,
}

impl Key1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY_1** `[31:0]`
    ///
    /// This bits stores key_1 that is a part of key material.
    #[inline(always)]
    pub const fn key_1(&self) -> u32 {
        (self.bits & Key1::KEY_1_MASK) >> Key1::KEY_1_OFFSET
    }

    /// Set **KEY_1** `[31:0]`
    ///
    /// This bits stores key_1 that is a part of key material.
    #[inline(always)]
    pub const fn with_key_1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Key1::KEY_1_MASK)
                | ((value << Key1::KEY_1_OFFSET) & Key1::KEY_1_MASK),
        }
    }
}

impl Default for Key1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key1Val {
        Key1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key1Val) -> Key1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// KEY_2 Register
///
/// Key material key_2 configure register
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **KEY_2** `[31:0]` (RW)
///
///   This bits stores key_2 that is a part of key material.
#[repr(transparent)]
pub struct Key2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Key2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key2 {
    // ======== Field Constants ========

    /// **KEY_2**
    ///
    /// This bits stores key_2 that is a part of key material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY_2_OFFSET: u32 = 0;
    pub const KEY_2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY_2** `[31:0]` (RW)
    ///
    /// This bits stores key_2 that is a part of key material.
    #[inline(always)]
    pub fn key_2(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_2_MASK) >> Self::KEY_2_OFFSET
    }

    /// Write **KEY_2** `[31:0]` (RW)
    ///
    /// This bits stores key_2 that is a part of key material.
    #[inline(always)]
    pub fn write_key_2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::KEY_2_MASK) | ((value << Self::KEY_2_OFFSET) & Self::KEY_2_MASK)
        });
    }
}

/// KEY_2 value type for batch read/modify/write operations
///
/// Use `key2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key2Val {
    bits: u32,
}

impl Key2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY_2** `[31:0]`
    ///
    /// This bits stores key_2 that is a part of key material.
    #[inline(always)]
    pub const fn key_2(&self) -> u32 {
        (self.bits & Key2::KEY_2_MASK) >> Key2::KEY_2_OFFSET
    }

    /// Set **KEY_2** `[31:0]`
    ///
    /// This bits stores key_2 that is a part of key material.
    #[inline(always)]
    pub const fn with_key_2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Key2::KEY_2_MASK)
                | ((value << Key2::KEY_2_OFFSET) & Key2::KEY_2_MASK),
        }
    }
}

impl Default for Key2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key2Val {
        Key2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key2Val) -> Key2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// KEY_3 Register
///
/// Key material key_3 configure register
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **KEY_3** `[31:0]` (RW)
///
///   This bits stores key_3 that is a part of key material.
#[repr(transparent)]
pub struct Key3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Key3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key3 {
    // ======== Field Constants ========

    /// **KEY_3**
    ///
    /// This bits stores key_3 that is a part of key material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY_3_OFFSET: u32 = 0;
    pub const KEY_3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY_3** `[31:0]` (RW)
    ///
    /// This bits stores key_3 that is a part of key material.
    #[inline(always)]
    pub fn key_3(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_3_MASK) >> Self::KEY_3_OFFSET
    }

    /// Write **KEY_3** `[31:0]` (RW)
    ///
    /// This bits stores key_3 that is a part of key material.
    #[inline(always)]
    pub fn write_key_3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::KEY_3_MASK) | ((value << Self::KEY_3_OFFSET) & Self::KEY_3_MASK)
        });
    }
}

/// KEY_3 value type for batch read/modify/write operations
///
/// Use `key3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key3Val {
    bits: u32,
}

impl Key3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY_3** `[31:0]`
    ///
    /// This bits stores key_3 that is a part of key material.
    #[inline(always)]
    pub const fn key_3(&self) -> u32 {
        (self.bits & Key3::KEY_3_MASK) >> Key3::KEY_3_OFFSET
    }

    /// Set **KEY_3** `[31:0]`
    ///
    /// This bits stores key_3 that is a part of key material.
    #[inline(always)]
    pub const fn with_key_3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Key3::KEY_3_MASK)
                | ((value << Key3::KEY_3_OFFSET) & Key3::KEY_3_MASK),
        }
    }
}

impl Default for Key3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key3Val {
        Key3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key3Val) -> Key3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// KEY_4 Register
///
/// Key material key_4 configure register
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **KEY_4** `[31:0]` (RW)
///
///   This bits stores key_4 that is a part of key material.
#[repr(transparent)]
pub struct Key4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Key4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key4 {
    // ======== Field Constants ========

    /// **KEY_4**
    ///
    /// This bits stores key_4 that is a part of key material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY_4_OFFSET: u32 = 0;
    pub const KEY_4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY_4** `[31:0]` (RW)
    ///
    /// This bits stores key_4 that is a part of key material.
    #[inline(always)]
    pub fn key_4(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_4_MASK) >> Self::KEY_4_OFFSET
    }

    /// Write **KEY_4** `[31:0]` (RW)
    ///
    /// This bits stores key_4 that is a part of key material.
    #[inline(always)]
    pub fn write_key_4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::KEY_4_MASK) | ((value << Self::KEY_4_OFFSET) & Self::KEY_4_MASK)
        });
    }
}

/// KEY_4 value type for batch read/modify/write operations
///
/// Use `key4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key4Val {
    bits: u32,
}

impl Key4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY_4** `[31:0]`
    ///
    /// This bits stores key_4 that is a part of key material.
    #[inline(always)]
    pub const fn key_4(&self) -> u32 {
        (self.bits & Key4::KEY_4_MASK) >> Key4::KEY_4_OFFSET
    }

    /// Set **KEY_4** `[31:0]`
    ///
    /// This bits stores key_4 that is a part of key material.
    #[inline(always)]
    pub const fn with_key_4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Key4::KEY_4_MASK)
                | ((value << Key4::KEY_4_OFFSET) & Key4::KEY_4_MASK),
        }
    }
}

impl Default for Key4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key4Val {
        Key4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key4Val) -> Key4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// KEY_5 Register
///
/// Key material key_5 configure register
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **KEY_5** `[31:0]` (RW)
///
///   This bits stores key_5 that is a part of key material.
#[repr(transparent)]
pub struct Key5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Key5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key5 {
    // ======== Field Constants ========

    /// **KEY_5**
    ///
    /// This bits stores key_5 that is a part of key material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY_5_OFFSET: u32 = 0;
    pub const KEY_5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY_5** `[31:0]` (RW)
    ///
    /// This bits stores key_5 that is a part of key material.
    #[inline(always)]
    pub fn key_5(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_5_MASK) >> Self::KEY_5_OFFSET
    }

    /// Write **KEY_5** `[31:0]` (RW)
    ///
    /// This bits stores key_5 that is a part of key material.
    #[inline(always)]
    pub fn write_key_5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::KEY_5_MASK) | ((value << Self::KEY_5_OFFSET) & Self::KEY_5_MASK)
        });
    }
}

/// KEY_5 value type for batch read/modify/write operations
///
/// Use `key5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key5Val {
    bits: u32,
}

impl Key5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY_5** `[31:0]`
    ///
    /// This bits stores key_5 that is a part of key material.
    #[inline(always)]
    pub const fn key_5(&self) -> u32 {
        (self.bits & Key5::KEY_5_MASK) >> Key5::KEY_5_OFFSET
    }

    /// Set **KEY_5** `[31:0]`
    ///
    /// This bits stores key_5 that is a part of key material.
    #[inline(always)]
    pub const fn with_key_5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Key5::KEY_5_MASK)
                | ((value << Key5::KEY_5_OFFSET) & Key5::KEY_5_MASK),
        }
    }
}

impl Default for Key5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key5Val {
        Key5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key5Val) -> Key5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// KEY_6 Register
///
/// Key material key_6 configure register
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **KEY_6** `[31:0]` (RW)
///
///   This bits stores key_6 that is a part of key material.
#[repr(transparent)]
pub struct Key6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Key6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key6 {
    // ======== Field Constants ========

    /// **KEY_6**
    ///
    /// This bits stores key_6 that is a part of key material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY_6_OFFSET: u32 = 0;
    pub const KEY_6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY_6** `[31:0]` (RW)
    ///
    /// This bits stores key_6 that is a part of key material.
    #[inline(always)]
    pub fn key_6(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_6_MASK) >> Self::KEY_6_OFFSET
    }

    /// Write **KEY_6** `[31:0]` (RW)
    ///
    /// This bits stores key_6 that is a part of key material.
    #[inline(always)]
    pub fn write_key_6(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::KEY_6_MASK) | ((value << Self::KEY_6_OFFSET) & Self::KEY_6_MASK)
        });
    }
}

/// KEY_6 value type for batch read/modify/write operations
///
/// Use `key6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key6Val {
    bits: u32,
}

impl Key6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY_6** `[31:0]`
    ///
    /// This bits stores key_6 that is a part of key material.
    #[inline(always)]
    pub const fn key_6(&self) -> u32 {
        (self.bits & Key6::KEY_6_MASK) >> Key6::KEY_6_OFFSET
    }

    /// Set **KEY_6** `[31:0]`
    ///
    /// This bits stores key_6 that is a part of key material.
    #[inline(always)]
    pub const fn with_key_6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Key6::KEY_6_MASK)
                | ((value << Key6::KEY_6_OFFSET) & Key6::KEY_6_MASK),
        }
    }
}

impl Default for Key6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key6Val {
        Key6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key6Val) -> Key6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// KEY_7 Register
///
/// Key material key_7 configure register
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **KEY_7** `[31:0]` (RW)
///
///   This bits stores key_7 that is a part of key material.
#[repr(transparent)]
pub struct Key7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Key7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Key7 {
    // ======== Field Constants ========

    /// **KEY_7**
    ///
    /// This bits stores key_7 that is a part of key material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const KEY_7_OFFSET: u32 = 0;
    pub const KEY_7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **KEY_7** `[31:0]` (RW)
    ///
    /// This bits stores key_7 that is a part of key material.
    #[inline(always)]
    pub fn key_7(&self) -> u32 {
        (self.value.read_bits() & Self::KEY_7_MASK) >> Self::KEY_7_OFFSET
    }

    /// Write **KEY_7** `[31:0]` (RW)
    ///
    /// This bits stores key_7 that is a part of key material.
    #[inline(always)]
    pub fn write_key_7(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::KEY_7_MASK) | ((value << Self::KEY_7_OFFSET) & Self::KEY_7_MASK)
        });
    }
}

/// KEY_7 value type for batch read/modify/write operations
///
/// Use `key7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `key7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Key7Val {
    bits: u32,
}

impl Key7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **KEY_7** `[31:0]`
    ///
    /// This bits stores key_7 that is a part of key material.
    #[inline(always)]
    pub const fn key_7(&self) -> u32 {
        (self.bits & Key7::KEY_7_MASK) >> Key7::KEY_7_OFFSET
    }

    /// Set **KEY_7** `[31:0]`
    ///
    /// This bits stores key_7 that is a part of key material.
    #[inline(always)]
    pub const fn with_key_7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Key7::KEY_7_MASK)
                | ((value << Key7::KEY_7_OFFSET) & Key7::KEY_7_MASK),
        }
    }
}

impl Default for Key7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Key7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Key7Val {
        Key7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Key7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Key7Val) -> Key7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TEXT_IN_0 Register
///
/// source text material text_in_0 configure register
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **TEXT_IN_0** `[31:0]` (RW)
///
///   This bits stores text_in_0 that is a part of source text material.
#[repr(transparent)]
pub struct TextIn0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TextIn0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TextIn0 {
    // ======== Field Constants ========

    /// **TEXT_IN_0**
    ///
    /// This bits stores text_in_0 that is a part of source text material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TEXT_IN_0_OFFSET: u32 = 0;
    pub const TEXT_IN_0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TEXT_IN_0** `[31:0]` (RW)
    ///
    /// This bits stores text_in_0 that is a part of source text material.
    #[inline(always)]
    pub fn text_in_0(&self) -> u32 {
        (self.value.read_bits() & Self::TEXT_IN_0_MASK) >> Self::TEXT_IN_0_OFFSET
    }

    /// Write **TEXT_IN_0** `[31:0]` (RW)
    ///
    /// This bits stores text_in_0 that is a part of source text material.
    #[inline(always)]
    pub fn write_text_in_0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TEXT_IN_0_MASK) | ((value << Self::TEXT_IN_0_OFFSET) & Self::TEXT_IN_0_MASK)
        });
    }
}

/// TEXT_IN_0 value type for batch read/modify/write operations
///
/// Use `textin0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `textin0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TextIn0Val {
    bits: u32,
}

impl TextIn0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TEXT_IN_0** `[31:0]`
    ///
    /// This bits stores text_in_0 that is a part of source text material.
    #[inline(always)]
    pub const fn text_in_0(&self) -> u32 {
        (self.bits & TextIn0::TEXT_IN_0_MASK) >> TextIn0::TEXT_IN_0_OFFSET
    }

    /// Set **TEXT_IN_0** `[31:0]`
    ///
    /// This bits stores text_in_0 that is a part of source text material.
    #[inline(always)]
    pub const fn with_text_in_0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TextIn0::TEXT_IN_0_MASK)
                | ((value << TextIn0::TEXT_IN_0_OFFSET) & TextIn0::TEXT_IN_0_MASK),
        }
    }
}

impl Default for TextIn0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TextIn0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TextIn0Val {
        TextIn0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TextIn0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TextIn0Val) -> TextIn0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TEXT_IN_1 Register
///
/// source text material text_in_1 configure register
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **TEXT_IN_1** `[31:0]` (RW)
///
///   This bits stores text_in_1 that is a part of source text material.
#[repr(transparent)]
pub struct TextIn1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TextIn1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TextIn1 {
    // ======== Field Constants ========

    /// **TEXT_IN_1**
    ///
    /// This bits stores text_in_1 that is a part of source text material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TEXT_IN_1_OFFSET: u32 = 0;
    pub const TEXT_IN_1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TEXT_IN_1** `[31:0]` (RW)
    ///
    /// This bits stores text_in_1 that is a part of source text material.
    #[inline(always)]
    pub fn text_in_1(&self) -> u32 {
        (self.value.read_bits() & Self::TEXT_IN_1_MASK) >> Self::TEXT_IN_1_OFFSET
    }

    /// Write **TEXT_IN_1** `[31:0]` (RW)
    ///
    /// This bits stores text_in_1 that is a part of source text material.
    #[inline(always)]
    pub fn write_text_in_1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TEXT_IN_1_MASK) | ((value << Self::TEXT_IN_1_OFFSET) & Self::TEXT_IN_1_MASK)
        });
    }
}

/// TEXT_IN_1 value type for batch read/modify/write operations
///
/// Use `textin1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `textin1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TextIn1Val {
    bits: u32,
}

impl TextIn1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TEXT_IN_1** `[31:0]`
    ///
    /// This bits stores text_in_1 that is a part of source text material.
    #[inline(always)]
    pub const fn text_in_1(&self) -> u32 {
        (self.bits & TextIn1::TEXT_IN_1_MASK) >> TextIn1::TEXT_IN_1_OFFSET
    }

    /// Set **TEXT_IN_1** `[31:0]`
    ///
    /// This bits stores text_in_1 that is a part of source text material.
    #[inline(always)]
    pub const fn with_text_in_1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TextIn1::TEXT_IN_1_MASK)
                | ((value << TextIn1::TEXT_IN_1_OFFSET) & TextIn1::TEXT_IN_1_MASK),
        }
    }
}

impl Default for TextIn1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TextIn1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TextIn1Val {
        TextIn1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TextIn1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TextIn1Val) -> TextIn1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TEXT_IN_2 Register
///
/// source text material text_in_2 configure register
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **TEXT_IN_2** `[31:0]` (RW)
///
///   This bits stores text_in_2 that is a part of source text material.
#[repr(transparent)]
pub struct TextIn2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TextIn2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TextIn2 {
    // ======== Field Constants ========

    /// **TEXT_IN_2**
    ///
    /// This bits stores text_in_2 that is a part of source text material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TEXT_IN_2_OFFSET: u32 = 0;
    pub const TEXT_IN_2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TEXT_IN_2** `[31:0]` (RW)
    ///
    /// This bits stores text_in_2 that is a part of source text material.
    #[inline(always)]
    pub fn text_in_2(&self) -> u32 {
        (self.value.read_bits() & Self::TEXT_IN_2_MASK) >> Self::TEXT_IN_2_OFFSET
    }

    /// Write **TEXT_IN_2** `[31:0]` (RW)
    ///
    /// This bits stores text_in_2 that is a part of source text material.
    #[inline(always)]
    pub fn write_text_in_2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TEXT_IN_2_MASK) | ((value << Self::TEXT_IN_2_OFFSET) & Self::TEXT_IN_2_MASK)
        });
    }
}

/// TEXT_IN_2 value type for batch read/modify/write operations
///
/// Use `textin2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `textin2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TextIn2Val {
    bits: u32,
}

impl TextIn2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TEXT_IN_2** `[31:0]`
    ///
    /// This bits stores text_in_2 that is a part of source text material.
    #[inline(always)]
    pub const fn text_in_2(&self) -> u32 {
        (self.bits & TextIn2::TEXT_IN_2_MASK) >> TextIn2::TEXT_IN_2_OFFSET
    }

    /// Set **TEXT_IN_2** `[31:0]`
    ///
    /// This bits stores text_in_2 that is a part of source text material.
    #[inline(always)]
    pub const fn with_text_in_2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TextIn2::TEXT_IN_2_MASK)
                | ((value << TextIn2::TEXT_IN_2_OFFSET) & TextIn2::TEXT_IN_2_MASK),
        }
    }
}

impl Default for TextIn2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TextIn2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TextIn2Val {
        TextIn2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TextIn2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TextIn2Val) -> TextIn2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TEXT_IN_3 Register
///
/// source text material text_in_3 configure register
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **TEXT_IN_3** `[31:0]` (RW)
///
///   This bits stores text_in_3 that is a part of source text material.
#[repr(transparent)]
pub struct TextIn3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TextIn3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TextIn3 {
    // ======== Field Constants ========

    /// **TEXT_IN_3**
    ///
    /// This bits stores text_in_3 that is a part of source text material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TEXT_IN_3_OFFSET: u32 = 0;
    pub const TEXT_IN_3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TEXT_IN_3** `[31:0]` (RW)
    ///
    /// This bits stores text_in_3 that is a part of source text material.
    #[inline(always)]
    pub fn text_in_3(&self) -> u32 {
        (self.value.read_bits() & Self::TEXT_IN_3_MASK) >> Self::TEXT_IN_3_OFFSET
    }

    /// Write **TEXT_IN_3** `[31:0]` (RW)
    ///
    /// This bits stores text_in_3 that is a part of source text material.
    #[inline(always)]
    pub fn write_text_in_3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TEXT_IN_3_MASK) | ((value << Self::TEXT_IN_3_OFFSET) & Self::TEXT_IN_3_MASK)
        });
    }
}

/// TEXT_IN_3 value type for batch read/modify/write operations
///
/// Use `textin3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `textin3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TextIn3Val {
    bits: u32,
}

impl TextIn3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TEXT_IN_3** `[31:0]`
    ///
    /// This bits stores text_in_3 that is a part of source text material.
    #[inline(always)]
    pub const fn text_in_3(&self) -> u32 {
        (self.bits & TextIn3::TEXT_IN_3_MASK) >> TextIn3::TEXT_IN_3_OFFSET
    }

    /// Set **TEXT_IN_3** `[31:0]`
    ///
    /// This bits stores text_in_3 that is a part of source text material.
    #[inline(always)]
    pub const fn with_text_in_3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TextIn3::TEXT_IN_3_MASK)
                | ((value << TextIn3::TEXT_IN_3_OFFSET) & TextIn3::TEXT_IN_3_MASK),
        }
    }
}

impl Default for TextIn3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TextIn3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TextIn3Val {
        TextIn3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TextIn3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TextIn3Val) -> TextIn3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TEXT_OUT_0 Register
///
/// result text material text_out_0 configure register
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **TEXT_OUT_0** `[31:0]` (RW)
///
///   This bits stores text_out_0 that is a part of result text material.
#[repr(transparent)]
pub struct TextOut0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TextOut0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TextOut0 {
    // ======== Field Constants ========

    /// **TEXT_OUT_0**
    ///
    /// This bits stores text_out_0 that is a part of result text material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TEXT_OUT_0_OFFSET: u32 = 0;
    pub const TEXT_OUT_0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TEXT_OUT_0** `[31:0]` (RW)
    ///
    /// This bits stores text_out_0 that is a part of result text material.
    #[inline(always)]
    pub fn text_out_0(&self) -> u32 {
        (self.value.read_bits() & Self::TEXT_OUT_0_MASK) >> Self::TEXT_OUT_0_OFFSET
    }

    /// Write **TEXT_OUT_0** `[31:0]` (RW)
    ///
    /// This bits stores text_out_0 that is a part of result text material.
    #[inline(always)]
    pub fn write_text_out_0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TEXT_OUT_0_MASK)
                | ((value << Self::TEXT_OUT_0_OFFSET) & Self::TEXT_OUT_0_MASK)
        });
    }
}

/// TEXT_OUT_0 value type for batch read/modify/write operations
///
/// Use `textout0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `textout0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TextOut0Val {
    bits: u32,
}

impl TextOut0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TEXT_OUT_0** `[31:0]`
    ///
    /// This bits stores text_out_0 that is a part of result text material.
    #[inline(always)]
    pub const fn text_out_0(&self) -> u32 {
        (self.bits & TextOut0::TEXT_OUT_0_MASK) >> TextOut0::TEXT_OUT_0_OFFSET
    }

    /// Set **TEXT_OUT_0** `[31:0]`
    ///
    /// This bits stores text_out_0 that is a part of result text material.
    #[inline(always)]
    pub const fn with_text_out_0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TextOut0::TEXT_OUT_0_MASK)
                | ((value << TextOut0::TEXT_OUT_0_OFFSET) & TextOut0::TEXT_OUT_0_MASK),
        }
    }
}

impl Default for TextOut0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TextOut0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TextOut0Val {
        TextOut0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TextOut0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TextOut0Val) -> TextOut0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TEXT_OUT_1 Register
///
/// result text material text_out_1 configure register
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **TEXT_OUT_1** `[31:0]` (RW)
///
///   This bits stores text_out_1 that is a part of result text material.
#[repr(transparent)]
pub struct TextOut1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TextOut1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TextOut1 {
    // ======== Field Constants ========

    /// **TEXT_OUT_1**
    ///
    /// This bits stores text_out_1 that is a part of result text material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TEXT_OUT_1_OFFSET: u32 = 0;
    pub const TEXT_OUT_1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TEXT_OUT_1** `[31:0]` (RW)
    ///
    /// This bits stores text_out_1 that is a part of result text material.
    #[inline(always)]
    pub fn text_out_1(&self) -> u32 {
        (self.value.read_bits() & Self::TEXT_OUT_1_MASK) >> Self::TEXT_OUT_1_OFFSET
    }

    /// Write **TEXT_OUT_1** `[31:0]` (RW)
    ///
    /// This bits stores text_out_1 that is a part of result text material.
    #[inline(always)]
    pub fn write_text_out_1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TEXT_OUT_1_MASK)
                | ((value << Self::TEXT_OUT_1_OFFSET) & Self::TEXT_OUT_1_MASK)
        });
    }
}

/// TEXT_OUT_1 value type for batch read/modify/write operations
///
/// Use `textout1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `textout1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TextOut1Val {
    bits: u32,
}

impl TextOut1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TEXT_OUT_1** `[31:0]`
    ///
    /// This bits stores text_out_1 that is a part of result text material.
    #[inline(always)]
    pub const fn text_out_1(&self) -> u32 {
        (self.bits & TextOut1::TEXT_OUT_1_MASK) >> TextOut1::TEXT_OUT_1_OFFSET
    }

    /// Set **TEXT_OUT_1** `[31:0]`
    ///
    /// This bits stores text_out_1 that is a part of result text material.
    #[inline(always)]
    pub const fn with_text_out_1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TextOut1::TEXT_OUT_1_MASK)
                | ((value << TextOut1::TEXT_OUT_1_OFFSET) & TextOut1::TEXT_OUT_1_MASK),
        }
    }
}

impl Default for TextOut1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TextOut1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TextOut1Val {
        TextOut1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TextOut1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TextOut1Val) -> TextOut1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TEXT_OUT_2 Register
///
/// result text material text_out_2 configure register
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **TEXT_OUT_2** `[31:0]` (RW)
///
///   This bits stores text_out_2 that is a part of result text material.
#[repr(transparent)]
pub struct TextOut2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TextOut2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TextOut2 {
    // ======== Field Constants ========

    /// **TEXT_OUT_2**
    ///
    /// This bits stores text_out_2 that is a part of result text material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TEXT_OUT_2_OFFSET: u32 = 0;
    pub const TEXT_OUT_2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TEXT_OUT_2** `[31:0]` (RW)
    ///
    /// This bits stores text_out_2 that is a part of result text material.
    #[inline(always)]
    pub fn text_out_2(&self) -> u32 {
        (self.value.read_bits() & Self::TEXT_OUT_2_MASK) >> Self::TEXT_OUT_2_OFFSET
    }

    /// Write **TEXT_OUT_2** `[31:0]` (RW)
    ///
    /// This bits stores text_out_2 that is a part of result text material.
    #[inline(always)]
    pub fn write_text_out_2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TEXT_OUT_2_MASK)
                | ((value << Self::TEXT_OUT_2_OFFSET) & Self::TEXT_OUT_2_MASK)
        });
    }
}

/// TEXT_OUT_2 value type for batch read/modify/write operations
///
/// Use `textout2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `textout2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TextOut2Val {
    bits: u32,
}

impl TextOut2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TEXT_OUT_2** `[31:0]`
    ///
    /// This bits stores text_out_2 that is a part of result text material.
    #[inline(always)]
    pub const fn text_out_2(&self) -> u32 {
        (self.bits & TextOut2::TEXT_OUT_2_MASK) >> TextOut2::TEXT_OUT_2_OFFSET
    }

    /// Set **TEXT_OUT_2** `[31:0]`
    ///
    /// This bits stores text_out_2 that is a part of result text material.
    #[inline(always)]
    pub const fn with_text_out_2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TextOut2::TEXT_OUT_2_MASK)
                | ((value << TextOut2::TEXT_OUT_2_OFFSET) & TextOut2::TEXT_OUT_2_MASK),
        }
    }
}

impl Default for TextOut2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TextOut2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TextOut2Val {
        TextOut2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TextOut2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TextOut2Val) -> TextOut2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TEXT_OUT_3 Register
///
/// result text material text_out_3 configure register
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **TEXT_OUT_3** `[31:0]` (RW)
///
///   This bits stores text_out_3 that is a part of result text material.
#[repr(transparent)]
pub struct TextOut3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TextOut3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TextOut3 {
    // ======== Field Constants ========

    /// **TEXT_OUT_3**
    ///
    /// This bits stores text_out_3 that is a part of result text material.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TEXT_OUT_3_OFFSET: u32 = 0;
    pub const TEXT_OUT_3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TEXT_OUT_3** `[31:0]` (RW)
    ///
    /// This bits stores text_out_3 that is a part of result text material.
    #[inline(always)]
    pub fn text_out_3(&self) -> u32 {
        (self.value.read_bits() & Self::TEXT_OUT_3_MASK) >> Self::TEXT_OUT_3_OFFSET
    }

    /// Write **TEXT_OUT_3** `[31:0]` (RW)
    ///
    /// This bits stores text_out_3 that is a part of result text material.
    #[inline(always)]
    pub fn write_text_out_3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TEXT_OUT_3_MASK)
                | ((value << Self::TEXT_OUT_3_OFFSET) & Self::TEXT_OUT_3_MASK)
        });
    }
}

/// TEXT_OUT_3 value type for batch read/modify/write operations
///
/// Use `textout3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `textout3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TextOut3Val {
    bits: u32,
}

impl TextOut3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TEXT_OUT_3** `[31:0]`
    ///
    /// This bits stores text_out_3 that is a part of result text material.
    #[inline(always)]
    pub const fn text_out_3(&self) -> u32 {
        (self.bits & TextOut3::TEXT_OUT_3_MASK) >> TextOut3::TEXT_OUT_3_OFFSET
    }

    /// Set **TEXT_OUT_3** `[31:0]`
    ///
    /// This bits stores text_out_3 that is a part of result text material.
    #[inline(always)]
    pub const fn with_text_out_3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TextOut3::TEXT_OUT_3_MASK)
                | ((value << TextOut3::TEXT_OUT_3_OFFSET) & TextOut3::TEXT_OUT_3_MASK),
        }
    }
}

impl Default for TextOut3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TextOut3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TextOut3Val {
        TextOut3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TextOut3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TextOut3Val) -> TextOut3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MODE Register
///
/// AES Mode register
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **MODE** `[2:0]` (RW)
///
///   This bits decides which one operation mode will be used. 3'd0: AES-EN-128, 3'd1: AES-EN-192, 3'd2: AES-EN-256, 3'd4: AES-DE-128, 3'd5: AES-DE-192, 3'd6: AES-DE-256.
#[repr(transparent)]
pub struct Mode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Mode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Mode {
    // ======== Field Constants ========

    /// **MODE**
    ///
    /// This bits decides which one operation mode will be used. 3'd0: AES-EN-128, 3'd1: AES-EN-192, 3'd2: AES-EN-256, 3'd4: AES-DE-128, 3'd5: AES-DE-192, 3'd6: AES-DE-256.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const MODE_OFFSET: u32 = 0;
    pub const MODE_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **MODE** `[2:0]` (RW)
    ///
    /// This bits decides which one operation mode will be used. 3'd0: AES-EN-128, 3'd1: AES-EN-192, 3'd2: AES-EN-256, 3'd4: AES-DE-128, 3'd5: AES-DE-192, 3'd6: AES-DE-256.
    #[inline(always)]
    pub fn mode(&self) -> u32 {
        (self.value.read_bits() & Self::MODE_MASK) >> Self::MODE_OFFSET
    }

    /// Write **MODE** `[2:0]` (RW)
    ///
    /// This bits decides which one operation mode will be used. 3'd0: AES-EN-128, 3'd1: AES-EN-192, 3'd2: AES-EN-256, 3'd4: AES-DE-128, 3'd5: AES-DE-192, 3'd6: AES-DE-256.
    #[inline(always)]
    pub fn write_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MODE_MASK) | ((value << Self::MODE_OFFSET) & Self::MODE_MASK)
        });
    }
}

/// MODE value type for batch read/modify/write operations
///
/// Use `mode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ModeVal {
    bits: u32,
}

impl ModeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MODE** `[2:0]`
    ///
    /// This bits decides which one operation mode will be used. 3'd0: AES-EN-128, 3'd1: AES-EN-192, 3'd2: AES-EN-256, 3'd4: AES-DE-128, 3'd5: AES-DE-192, 3'd6: AES-DE-256.
    #[inline(always)]
    pub const fn mode(&self) -> u32 {
        (self.bits & Mode::MODE_MASK) >> Mode::MODE_OFFSET
    }

    /// Set **MODE** `[2:0]`
    ///
    /// This bits decides which one operation mode will be used. 3'd0: AES-EN-128, 3'd1: AES-EN-192, 3'd2: AES-EN-256, 3'd4: AES-DE-128, 3'd5: AES-DE-192, 3'd6: AES-DE-256.
    #[inline(always)]
    pub const fn with_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Mode::MODE_MASK) | ((value << Mode::MODE_OFFSET) & Mode::MODE_MASK),
        }
    }
}

impl Default for ModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Mode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ModeVal {
        ModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ModeVal) -> ModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ENDIAN Register
///
/// AES Endian configure register
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **ENDIAN** `[5:0]` (RW)
///
///   endian. [1:0] key endian, [3:2] text_in endian or in_stream endian, [5:4] text_out endian or out_stream endian
#[repr(transparent)]
pub struct Endian {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Endian {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Endian {
    // ======== Field Constants ========

    /// **ENDIAN**
    ///
    /// endian. [1:0] key endian, [3:2] text_in endian or in_stream endian, [5:4] text_out endian or out_stream endian
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const ENDIAN_OFFSET: u32 = 0;
    pub const ENDIAN_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **ENDIAN** `[5:0]` (RW)
    ///
    /// endian. [1:0] key endian, [3:2] text_in endian or in_stream endian, [5:4] text_out endian or out_stream endian
    #[inline(always)]
    pub fn endian(&self) -> u32 {
        (self.value.read_bits() & Self::ENDIAN_MASK) >> Self::ENDIAN_OFFSET
    }

    /// Write **ENDIAN** `[5:0]` (RW)
    ///
    /// endian. [1:0] key endian, [3:2] text_in endian or in_stream endian, [5:4] text_out endian or out_stream endian
    #[inline(always)]
    pub fn write_endian(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ENDIAN_MASK) | ((value << Self::ENDIAN_OFFSET) & Self::ENDIAN_MASK)
        });
    }
}

/// ENDIAN value type for batch read/modify/write operations
///
/// Use `endian.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `endian.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EndianVal {
    bits: u32,
}

impl EndianVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **ENDIAN** `[5:0]`
    ///
    /// endian. [1:0] key endian, [3:2] text_in endian or in_stream endian, [5:4] text_out endian or out_stream endian
    #[inline(always)]
    pub const fn endian(&self) -> u32 {
        (self.bits & Endian::ENDIAN_MASK) >> Endian::ENDIAN_OFFSET
    }

    /// Set **ENDIAN** `[5:0]`
    ///
    /// endian. [1:0] key endian, [3:2] text_in endian or in_stream endian, [5:4] text_out endian or out_stream endian
    #[inline(always)]
    pub const fn with_endian(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Endian::ENDIAN_MASK)
                | ((value << Endian::ENDIAN_OFFSET) & Endian::ENDIAN_MASK),
        }
    }
}

impl Default for EndianVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Endian {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EndianVal {
        EndianVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EndianVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EndianVal) -> EndianVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TRIGGER Register
///
/// AES trigger register
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **TRIGGER** `[0]` (WO)
///
///   Set this bit to start AES calculation.
#[repr(transparent)]
pub struct Trigger {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Trigger {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Trigger {
    // ======== Field Constants ========

    /// **TRIGGER**
    ///
    /// Set this bit to start AES calculation.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TRIGGER_OFFSET: u32 = 0;
    pub const TRIGGER_MASK: u32 = 0x00000001;
    pub const TRIGGER: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **TRIGGER** (WO)
    ///
    /// Set this bit to start AES calculation.
    #[inline(always)]
    pub fn trigger_trigger(&self) {
        self.value.write_bits(Self::TRIGGER_MASK);
    }
}

/// TRIGGER value type for batch read/modify/write operations
///
/// Use `trigger.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `trigger.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TriggerVal {
    bits: u32,
}

impl TriggerVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **TRIGGER** `[0:0]`
    ///
    /// Set this bit to start AES calculation.
    #[inline(always)]
    pub const fn with_trigger(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Trigger::TRIGGER_MASK
            } else {
                self.bits & !Trigger::TRIGGER_MASK
            },
        }
    }
}

impl Default for TriggerVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Trigger {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TriggerVal) {
        self.value.write_bits(val.bits())
    }
}

/// STATE Register
///
/// AES state register
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **STATE** `[1:0]` (RO)
///
///   Those bits shows AES status. For typical AES, 0: idle, 1: busy. For DMA-AES, 0: idle, 1: busy, 2: calculation_done.
#[repr(transparent)]
pub struct State {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for State {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl State {
    // ======== Field Constants ========

    /// **STATE**
    ///
    /// Those bits shows AES status. For typical AES, 0: idle, 1: busy. For DMA-AES, 0: idle, 1: busy, 2: calculation_done.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const STATE_OFFSET: u32 = 0;
    pub const STATE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **STATE** `[1:0]` (RO)
    ///
    /// Those bits shows AES status. For typical AES, 0: idle, 1: busy. For DMA-AES, 0: idle, 1: busy, 2: calculation_done.
    #[inline(always)]
    pub fn state(&self) -> u32 {
        (self.value.read_bits() & Self::STATE_MASK) >> Self::STATE_OFFSET
    }
}

/// STATE value type for batch read/modify/write operations
///
/// Use `state.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `state.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StateVal {
    bits: u32,
}

impl StateVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **STATE** `[1:0]`
    ///
    /// Those bits shows AES status. For typical AES, 0: idle, 1: busy. For DMA-AES, 0: idle, 1: busy, 2: calculation_done.
    #[inline(always)]
    pub const fn state(&self) -> u32 {
        (self.bits & State::STATE_MASK) >> State::STATE_OFFSET
    }
}

impl Default for StateVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl State {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StateVal {
        StateVal::from_bits(self.value.read_bits())
    }
}

/// IV_MEM[0] Register
///
/// The memory that stores initialization vector
///
/// Address offset: 0x0050
#[repr(transparent)]
pub struct IvMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IvMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IvMem0 {}

/// IV_MEM[1] Register
///
/// The memory that stores initialization vector
///
/// Address offset: 0x0054
#[repr(transparent)]
pub struct IvMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IvMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IvMem1 {}

/// IV_MEM[2] Register
///
/// The memory that stores initialization vector
///
/// Address offset: 0x0058
#[repr(transparent)]
pub struct IvMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IvMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IvMem2 {}

/// IV_MEM[3] Register
///
/// The memory that stores initialization vector
///
/// Address offset: 0x005c
#[repr(transparent)]
pub struct IvMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IvMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IvMem3 {}

/// H_MEM[0] Register
///
/// The memory that stores GCM hash subkey
///
/// Address offset: 0x0060
#[repr(transparent)]
pub struct HMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem0 {}

/// H_MEM[1] Register
///
/// The memory that stores GCM hash subkey
///
/// Address offset: 0x0064
#[repr(transparent)]
pub struct HMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem1 {}

/// H_MEM[2] Register
///
/// The memory that stores GCM hash subkey
///
/// Address offset: 0x0068
#[repr(transparent)]
pub struct HMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem2 {}

/// H_MEM[3] Register
///
/// The memory that stores GCM hash subkey
///
/// Address offset: 0x006c
#[repr(transparent)]
pub struct HMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HMem3 {}

/// J0_MEM[0] Register
///
/// The memory that stores J0
///
/// Address offset: 0x0070
#[repr(transparent)]
pub struct J0Mem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for J0Mem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl J0Mem0 {}

/// J0_MEM[1] Register
///
/// The memory that stores J0
///
/// Address offset: 0x0074
#[repr(transparent)]
pub struct J0Mem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for J0Mem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl J0Mem1 {}

/// J0_MEM[2] Register
///
/// The memory that stores J0
///
/// Address offset: 0x0078
#[repr(transparent)]
pub struct J0Mem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for J0Mem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl J0Mem2 {}

/// J0_MEM[3] Register
///
/// The memory that stores J0
///
/// Address offset: 0x007c
#[repr(transparent)]
pub struct J0Mem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for J0Mem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl J0Mem3 {}

/// T0_MEM[0] Register
///
/// The memory that stores T0
///
/// Address offset: 0x0080
#[repr(transparent)]
pub struct T0Mem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T0Mem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T0Mem0 {}

/// T0_MEM[1] Register
///
/// The memory that stores T0
///
/// Address offset: 0x0084
#[repr(transparent)]
pub struct T0Mem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T0Mem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T0Mem1 {}

/// T0_MEM[2] Register
///
/// The memory that stores T0
///
/// Address offset: 0x0088
#[repr(transparent)]
pub struct T0Mem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T0Mem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T0Mem2 {}

/// T0_MEM[3] Register
///
/// The memory that stores T0
///
/// Address offset: 0x008c
#[repr(transparent)]
pub struct T0Mem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for T0Mem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl T0Mem3 {}

/// DMA_ENABLE Register
///
/// DMA-AES working mode register
///
/// Address offset: 0x0090
///
/// ## Fields
///
/// - **DMA_ENABLE** `[0]` (RW)
///
///   1'b0: typical AES working mode, 1'b1: DMA-AES working mode.
#[repr(transparent)]
pub struct DmaEnable {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmaEnable {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmaEnable {
    // ======== Field Constants ========

    /// **DMA_ENABLE**
    ///
    /// 1'b0: typical AES working mode, 1'b1: DMA-AES working mode.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DMA_ENABLE_OFFSET: u32 = 0;
    pub const DMA_ENABLE_MASK: u32 = 0x00000001;
    pub const DMA_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DMA_ENABLE** `[0]` (RW)
    ///
    /// 1'b0: typical AES working mode, 1'b1: DMA-AES working mode.
    #[inline(always)]
    pub fn dma_enable(&self) -> bool {
        (self.value.read_bits() & Self::DMA_ENABLE_MASK) != 0
    }

    /// Set DMA_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma_enable(&self) {
        self.value.modify_bits(|r| r | Self::DMA_ENABLE_MASK);
    }

    /// Clear DMA_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma_enable(&self) {
        self.value.modify_bits(|r| r & !Self::DMA_ENABLE_MASK);
    }
}

/// DMA_ENABLE value type for batch read/modify/write operations
///
/// Use `dmaenable.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmaenable.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaEnableVal {
    bits: u32,
}

impl DmaEnableVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **DMA_ENABLE** `[0:0]`
    ///
    /// 1'b0: typical AES working mode, 1'b1: DMA-AES working mode.
    #[inline(always)]
    pub const fn dma_enable(&self) -> bool {
        (self.bits & DmaEnable::DMA_ENABLE_MASK) != 0
    }

    /// Set **DMA_ENABLE** `[0:0]`
    ///
    /// 1'b0: typical AES working mode, 1'b1: DMA-AES working mode.
    #[inline(always)]
    pub const fn with_dma_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaEnable::DMA_ENABLE_MASK
            } else {
                self.bits & !DmaEnable::DMA_ENABLE_MASK
            },
        }
    }
}

impl Default for DmaEnableVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DmaEnable {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaEnableVal {
        DmaEnableVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaEnableVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaEnableVal) -> DmaEnableVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BLOCK_MODE Register
///
/// AES cipher block mode register
///
/// Address offset: 0x0094
///
/// ## Fields
///
/// - **BLOCK_MODE** `[2:0]` (RW)
///
///   Those bits decides which block mode will be used. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: GCM, 0x7: reserved.
#[repr(transparent)]
pub struct BlockMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BlockMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BlockMode {
    // ======== Field Constants ========

    /// **BLOCK_MODE**
    ///
    /// Those bits decides which block mode will be used. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: GCM, 0x7: reserved.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const BLOCK_MODE_OFFSET: u32 = 0;
    pub const BLOCK_MODE_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **BLOCK_MODE** `[2:0]` (RW)
    ///
    /// Those bits decides which block mode will be used. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: GCM, 0x7: reserved.
    #[inline(always)]
    pub fn block_mode(&self) -> u32 {
        (self.value.read_bits() & Self::BLOCK_MODE_MASK) >> Self::BLOCK_MODE_OFFSET
    }

    /// Write **BLOCK_MODE** `[2:0]` (RW)
    ///
    /// Those bits decides which block mode will be used. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: GCM, 0x7: reserved.
    #[inline(always)]
    pub fn write_block_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLOCK_MODE_MASK)
                | ((value << Self::BLOCK_MODE_OFFSET) & Self::BLOCK_MODE_MASK)
        });
    }
}

/// BLOCK_MODE value type for batch read/modify/write operations
///
/// Use `blockmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `blockmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BlockModeVal {
    bits: u32,
}

impl BlockModeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BLOCK_MODE** `[2:0]`
    ///
    /// Those bits decides which block mode will be used. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: GCM, 0x7: reserved.
    #[inline(always)]
    pub const fn block_mode(&self) -> u32 {
        (self.bits & BlockMode::BLOCK_MODE_MASK) >> BlockMode::BLOCK_MODE_OFFSET
    }

    /// Set **BLOCK_MODE** `[2:0]`
    ///
    /// Those bits decides which block mode will be used. 0x0: ECB, 0x1: CBC, 0x2: OFB, 0x3: CTR, 0x4: CFB-8, 0x5: CFB-128, 0x6: GCM, 0x7: reserved.
    #[inline(always)]
    pub const fn with_block_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlockMode::BLOCK_MODE_MASK)
                | ((value << BlockMode::BLOCK_MODE_OFFSET) & BlockMode::BLOCK_MODE_MASK),
        }
    }
}

impl Default for BlockModeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BlockMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BlockModeVal {
        BlockModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BlockModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BlockModeVal) -> BlockModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BLOCK_NUM Register
///
/// AES block number register
///
/// Address offset: 0x0098
///
/// ## Fields
///
/// - **BLOCK_NUM** `[31:0]` (RW)
///
///   Those bits stores the number of Plaintext/ciphertext block.
#[repr(transparent)]
pub struct BlockNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BlockNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BlockNum {
    // ======== Field Constants ========

    /// **BLOCK_NUM**
    ///
    /// Those bits stores the number of Plaintext/ciphertext block.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const BLOCK_NUM_OFFSET: u32 = 0;
    pub const BLOCK_NUM_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **BLOCK_NUM** `[31:0]` (RW)
    ///
    /// Those bits stores the number of Plaintext/ciphertext block.
    #[inline(always)]
    pub fn block_num(&self) -> u32 {
        (self.value.read_bits() & Self::BLOCK_NUM_MASK) >> Self::BLOCK_NUM_OFFSET
    }

    /// Write **BLOCK_NUM** `[31:0]` (RW)
    ///
    /// Those bits stores the number of Plaintext/ciphertext block.
    #[inline(always)]
    pub fn write_block_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BLOCK_NUM_MASK) | ((value << Self::BLOCK_NUM_OFFSET) & Self::BLOCK_NUM_MASK)
        });
    }
}

/// BLOCK_NUM value type for batch read/modify/write operations
///
/// Use `blocknum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `blocknum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BlockNumVal {
    bits: u32,
}

impl BlockNumVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BLOCK_NUM** `[31:0]`
    ///
    /// Those bits stores the number of Plaintext/ciphertext block.
    #[inline(always)]
    pub const fn block_num(&self) -> u32 {
        (self.bits & BlockNum::BLOCK_NUM_MASK) >> BlockNum::BLOCK_NUM_OFFSET
    }

    /// Set **BLOCK_NUM** `[31:0]`
    ///
    /// Those bits stores the number of Plaintext/ciphertext block.
    #[inline(always)]
    pub const fn with_block_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BlockNum::BLOCK_NUM_MASK)
                | ((value << BlockNum::BLOCK_NUM_OFFSET) & BlockNum::BLOCK_NUM_MASK),
        }
    }
}

impl Default for BlockNumVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BlockNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BlockNumVal {
        BlockNumVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BlockNumVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BlockNumVal) -> BlockNumVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INC_SEL Register
///
/// Standard incrementing function configure register
///
/// Address offset: 0x009c
///
/// ## Fields
///
/// - **INC_SEL** `[0]` (RW)
///
///   This bit decides the standard incrementing function. 0: INC32. 1: INC128.
#[repr(transparent)]
pub struct IncSel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IncSel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IncSel {
    // ======== Field Constants ========

    /// **INC_SEL**
    ///
    /// This bit decides the standard incrementing function. 0: INC32. 1: INC128.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const INC_SEL_OFFSET: u32 = 0;
    pub const INC_SEL_MASK: u32 = 0x00000001;
    pub const INC_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INC_SEL** `[0]` (RW)
    ///
    /// This bit decides the standard incrementing function. 0: INC32. 1: INC128.
    #[inline(always)]
    pub fn inc_sel(&self) -> bool {
        (self.value.read_bits() & Self::INC_SEL_MASK) != 0
    }

    /// Set INC_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_inc_sel(&self) {
        self.value.modify_bits(|r| r | Self::INC_SEL_MASK);
    }

    /// Clear INC_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_inc_sel(&self) {
        self.value.modify_bits(|r| r & !Self::INC_SEL_MASK);
    }
}

/// INC_SEL value type for batch read/modify/write operations
///
/// Use `incsel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `incsel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IncSelVal {
    bits: u32,
}

impl IncSelVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **INC_SEL** `[0:0]`
    ///
    /// This bit decides the standard incrementing function. 0: INC32. 1: INC128.
    #[inline(always)]
    pub const fn inc_sel(&self) -> bool {
        (self.bits & IncSel::INC_SEL_MASK) != 0
    }

    /// Set **INC_SEL** `[0:0]`
    ///
    /// This bit decides the standard incrementing function. 0: INC32. 1: INC128.
    #[inline(always)]
    pub const fn with_inc_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IncSel::INC_SEL_MASK
            } else {
                self.bits & !IncSel::INC_SEL_MASK
            },
        }
    }
}

impl Default for IncSelVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IncSel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IncSelVal {
        IncSelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IncSelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IncSelVal) -> IncSelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// AAD_BLOCK_NUM Register
///
/// Additional Authential Data block number register
///
/// Address offset: 0x00a0
///
/// ## Fields
///
/// - **AAD_BLOCK_NUM** `[31:0]` (RW)
///
///   Those bits stores the number of AAD block.
#[repr(transparent)]
pub struct AadBlockNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AadBlockNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AadBlockNum {
    // ======== Field Constants ========

    /// **AAD_BLOCK_NUM**
    ///
    /// Those bits stores the number of AAD block.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const AAD_BLOCK_NUM_OFFSET: u32 = 0;
    pub const AAD_BLOCK_NUM_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **AAD_BLOCK_NUM** `[31:0]` (RW)
    ///
    /// Those bits stores the number of AAD block.
    #[inline(always)]
    pub fn aad_block_num(&self) -> u32 {
        (self.value.read_bits() & Self::AAD_BLOCK_NUM_MASK) >> Self::AAD_BLOCK_NUM_OFFSET
    }

    /// Write **AAD_BLOCK_NUM** `[31:0]` (RW)
    ///
    /// Those bits stores the number of AAD block.
    #[inline(always)]
    pub fn write_aad_block_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::AAD_BLOCK_NUM_MASK)
                | ((value << Self::AAD_BLOCK_NUM_OFFSET) & Self::AAD_BLOCK_NUM_MASK)
        });
    }
}

/// AAD_BLOCK_NUM value type for batch read/modify/write operations
///
/// Use `aadblocknum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `aadblocknum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AadBlockNumVal {
    bits: u32,
}

impl AadBlockNumVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **AAD_BLOCK_NUM** `[31:0]`
    ///
    /// Those bits stores the number of AAD block.
    #[inline(always)]
    pub const fn aad_block_num(&self) -> u32 {
        (self.bits & AadBlockNum::AAD_BLOCK_NUM_MASK) >> AadBlockNum::AAD_BLOCK_NUM_OFFSET
    }

    /// Set **AAD_BLOCK_NUM** `[31:0]`
    ///
    /// Those bits stores the number of AAD block.
    #[inline(always)]
    pub const fn with_aad_block_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AadBlockNum::AAD_BLOCK_NUM_MASK)
                | ((value << AadBlockNum::AAD_BLOCK_NUM_OFFSET) & AadBlockNum::AAD_BLOCK_NUM_MASK),
        }
    }
}

impl Default for AadBlockNumVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl AadBlockNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AadBlockNumVal {
        AadBlockNumVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AadBlockNumVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AadBlockNumVal) -> AadBlockNumVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// REMAINDER_BIT_NUM Register
///
/// AES remainder bit number register
///
/// Address offset: 0x00a4
///
/// ## Fields
///
/// - **REMAINDER_BIT_NUM** `[6:0]` (RW)
///
///   Those bits stores the number of remainder bit.
#[repr(transparent)]
pub struct RemainderBitNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RemainderBitNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RemainderBitNum {
    // ======== Field Constants ========

    /// **REMAINDER_BIT_NUM**
    ///
    /// Those bits stores the number of remainder bit.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const REMAINDER_BIT_NUM_OFFSET: u32 = 0;
    pub const REMAINDER_BIT_NUM_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **REMAINDER_BIT_NUM** `[6:0]` (RW)
    ///
    /// Those bits stores the number of remainder bit.
    #[inline(always)]
    pub fn remainder_bit_num(&self) -> u32 {
        (self.value.read_bits() & Self::REMAINDER_BIT_NUM_MASK) >> Self::REMAINDER_BIT_NUM_OFFSET
    }

    /// Write **REMAINDER_BIT_NUM** `[6:0]` (RW)
    ///
    /// Those bits stores the number of remainder bit.
    #[inline(always)]
    pub fn write_remainder_bit_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REMAINDER_BIT_NUM_MASK)
                | ((value << Self::REMAINDER_BIT_NUM_OFFSET) & Self::REMAINDER_BIT_NUM_MASK)
        });
    }
}

/// REMAINDER_BIT_NUM value type for batch read/modify/write operations
///
/// Use `remainderbitnum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `remainderbitnum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RemainderBitNumVal {
    bits: u32,
}

impl RemainderBitNumVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REMAINDER_BIT_NUM** `[6:0]`
    ///
    /// Those bits stores the number of remainder bit.
    #[inline(always)]
    pub const fn remainder_bit_num(&self) -> u32 {
        (self.bits & RemainderBitNum::REMAINDER_BIT_NUM_MASK)
            >> RemainderBitNum::REMAINDER_BIT_NUM_OFFSET
    }

    /// Set **REMAINDER_BIT_NUM** `[6:0]`
    ///
    /// Those bits stores the number of remainder bit.
    #[inline(always)]
    pub const fn with_remainder_bit_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RemainderBitNum::REMAINDER_BIT_NUM_MASK)
                | ((value << RemainderBitNum::REMAINDER_BIT_NUM_OFFSET)
                    & RemainderBitNum::REMAINDER_BIT_NUM_MASK),
        }
    }
}

impl Default for RemainderBitNumVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RemainderBitNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RemainderBitNumVal {
        RemainderBitNumVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RemainderBitNumVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RemainderBitNumVal) -> RemainderBitNumVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CONTINUE Register
///
/// AES continue register
///
/// Address offset: 0x00a8
///
/// ## Fields
///
/// - **CONTINUE** `[0]` (WO)
///
///   Set this bit to continue GCM operation.
#[repr(transparent)]
pub struct Continue {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Continue {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Continue {
    // ======== Field Constants ========

    /// **CONTINUE**
    ///
    /// Set this bit to continue GCM operation.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CONTINUE_OFFSET: u32 = 0;
    pub const CONTINUE_MASK: u32 = 0x00000001;
    pub const CONTINUE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CONTINUE** (WO)
    ///
    /// Set this bit to continue GCM operation.
    #[inline(always)]
    pub fn trigger_continue_(&self) {
        self.value.write_bits(Self::CONTINUE_MASK);
    }
}

/// CONTINUE value type for batch read/modify/write operations
///
/// Use `continue.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `continue.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ContinueVal {
    bits: u32,
}

impl ContinueVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **CONTINUE** `[0:0]`
    ///
    /// Set this bit to continue GCM operation.
    #[inline(always)]
    pub const fn with_continue_(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Continue::CONTINUE_MASK
            } else {
                self.bits & !Continue::CONTINUE_MASK
            },
        }
    }
}

impl Default for ContinueVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Continue {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ContinueVal) {
        self.value.write_bits(val.bits())
    }
}

/// INT_CLEAR Register
///
/// AES Interrupt clear register
///
/// Address offset: 0x00ac
///
/// ## Fields
///
/// - **INT_CLEAR** `[0]` (WO)
///
///   Set this bit to clear the AES interrupt.
#[repr(transparent)]
pub struct IntClear {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntClear {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntClear {
    // ======== Field Constants ========

    /// **INT_CLEAR**
    ///
    /// Set this bit to clear the AES interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const INT_CLEAR_OFFSET: u32 = 0;
    pub const INT_CLEAR_MASK: u32 = 0x00000001;
    pub const INT_CLEAR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **INT_CLEAR** (WO)
    ///
    /// Set this bit to clear the AES interrupt.
    #[inline(always)]
    pub fn trigger_int_clear(&self) {
        self.value.write_bits(Self::INT_CLEAR_MASK);
    }
}

/// INT_CLEAR value type for batch read/modify/write operations
///
/// Use `intclear.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intclear.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntClearVal {
    bits: u32,
}

impl IntClearVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **INT_CLEAR** `[0:0]`
    ///
    /// Set this bit to clear the AES interrupt.
    #[inline(always)]
    pub const fn with_int_clear(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClear::INT_CLEAR_MASK
            } else {
                self.bits & !IntClear::INT_CLEAR_MASK
            },
        }
    }
}

impl Default for IntClearVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntClear {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntClearVal) {
        self.value.write_bits(val.bits())
    }
}

/// INT_ENA Register
///
/// AES Interrupt enable register
///
/// Address offset: 0x00b0
///
/// ## Fields
///
/// - **INT_ENA** `[0]` (RW)
///
///   Set this bit to enable interrupt that occurs when DMA-AES calculation is done.
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

    /// **INT_ENA**
    ///
    /// Set this bit to enable interrupt that occurs when DMA-AES calculation is done.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const INT_ENA_OFFSET: u32 = 0;
    pub const INT_ENA_MASK: u32 = 0x00000001;
    pub const INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INT_ENA** `[0]` (RW)
    ///
    /// Set this bit to enable interrupt that occurs when DMA-AES calculation is done.
    #[inline(always)]
    pub fn int_ena(&self) -> bool {
        (self.value.read_bits() & Self::INT_ENA_MASK) != 0
    }

    /// Set INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::INT_ENA_MASK);
    }

    /// Clear INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::INT_ENA_MASK);
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

    /// Read **INT_ENA** `[0:0]`
    ///
    /// Set this bit to enable interrupt that occurs when DMA-AES calculation is done.
    #[inline(always)]
    pub const fn int_ena(&self) -> bool {
        (self.bits & IntEna::INT_ENA_MASK) != 0
    }

    /// Set **INT_ENA** `[0:0]`
    ///
    /// Set this bit to enable interrupt that occurs when DMA-AES calculation is done.
    #[inline(always)]
    pub const fn with_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::INT_ENA_MASK
            } else {
                self.bits & !IntEna::INT_ENA_MASK
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

/// DATE Register
///
/// AES version control register
///
/// Address offset: 0x00b4
/// Reset value: 0x20191210
///
/// ## Fields
///
/// - **DATE** `[29:0]` (RW)
///
///   This bits stores the version information of AES.
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
    /// This bits stores the version information of AES.
    ///
    /// Bits `[29:0]`, mask `0x3fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x3fffffff;

    // ======== Field Accessors ========

    /// **DATE** `[29:0]` (RW)
    ///
    /// This bits stores the version information of AES.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[29:0]` (RW)
    ///
    /// This bits stores the version information of AES.
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
        Self { bits: 0x20191210 }
    }

    /// Read **DATE** `[29:0]`
    ///
    /// This bits stores the version information of AES.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[29:0]`
    ///
    /// This bits stores the version information of AES.
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

/// DMA_EXIT Register
///
/// AES-DMA exit config
///
/// Address offset: 0x00b8
///
/// ## Fields
///
/// - **DMA_EXIT** `[0]` (WO)
///
///   Set this register to leave calculation done stage. Recommend to use it after software finishes reading DMA's output buffer.
#[repr(transparent)]
pub struct DmaExit {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmaExit {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmaExit {
    // ======== Field Constants ========

    /// **DMA_EXIT**
    ///
    /// Set this register to leave calculation done stage. Recommend to use it after software finishes reading DMA's output buffer.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DMA_EXIT_OFFSET: u32 = 0;
    pub const DMA_EXIT_MASK: u32 = 0x00000001;
    pub const DMA_EXIT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **DMA_EXIT** (WO)
    ///
    /// Set this register to leave calculation done stage. Recommend to use it after software finishes reading DMA's output buffer.
    #[inline(always)]
    pub fn trigger_dma_exit(&self) {
        self.value.write_bits(Self::DMA_EXIT_MASK);
    }
}

/// DMA_EXIT value type for batch read/modify/write operations
///
/// Use `dmaexit.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmaexit.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaExitVal {
    bits: u32,
}

impl DmaExitVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **DMA_EXIT** `[0:0]`
    ///
    /// Set this register to leave calculation done stage. Recommend to use it after software finishes reading DMA's output buffer.
    #[inline(always)]
    pub const fn with_dma_exit(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaExit::DMA_EXIT_MASK
            } else {
                self.bits & !DmaExit::DMA_EXIT_MASK
            },
        }
    }
}

impl Default for DmaExitVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DmaExit {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaExitVal) {
        self.value.write_bits(val.bits())
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Key material key_0 configure register
    pub key0: Key0,
    /// 0x0004
    /// Key material key_1 configure register
    pub key1: Key1,
    /// 0x0008
    /// Key material key_2 configure register
    pub key2: Key2,
    /// 0x000c
    /// Key material key_3 configure register
    pub key3: Key3,
    /// 0x0010
    /// Key material key_4 configure register
    pub key4: Key4,
    /// 0x0014
    /// Key material key_5 configure register
    pub key5: Key5,
    /// 0x0018
    /// Key material key_6 configure register
    pub key6: Key6,
    /// 0x001c
    /// Key material key_7 configure register
    pub key7: Key7,
    /// 0x0020
    /// source text material text_in_0 configure register
    pub text_in0: TextIn0,
    /// 0x0024
    /// source text material text_in_1 configure register
    pub text_in1: TextIn1,
    /// 0x0028
    /// source text material text_in_2 configure register
    pub text_in2: TextIn2,
    /// 0x002c
    /// source text material text_in_3 configure register
    pub text_in3: TextIn3,
    /// 0x0030
    /// result text material text_out_0 configure register
    pub text_out0: TextOut0,
    /// 0x0034
    /// result text material text_out_1 configure register
    pub text_out1: TextOut1,
    /// 0x0038
    /// result text material text_out_2 configure register
    pub text_out2: TextOut2,
    /// 0x003c
    /// result text material text_out_3 configure register
    pub text_out3: TextOut3,
    /// 0x0040
    /// AES Mode register
    pub mode: Mode,
    /// 0x0044
    /// AES Endian configure register
    pub endian: Endian,
    /// 0x0048
    /// AES trigger register
    pub trigger: Trigger,
    /// 0x004c
    /// AES state register
    pub state: State,
    /// 0x0050
    /// The memory that stores initialization vector
    pub iv_mem0: IvMem0,
    /// 0x0054
    /// The memory that stores initialization vector
    pub iv_mem1: IvMem1,
    /// 0x0058
    /// The memory that stores initialization vector
    pub iv_mem2: IvMem2,
    /// 0x005c
    /// The memory that stores initialization vector
    pub iv_mem3: IvMem3,
    /// 0x0060
    /// The memory that stores GCM hash subkey
    pub h_mem0: HMem0,
    /// 0x0064
    /// The memory that stores GCM hash subkey
    pub h_mem1: HMem1,
    /// 0x0068
    /// The memory that stores GCM hash subkey
    pub h_mem2: HMem2,
    /// 0x006c
    /// The memory that stores GCM hash subkey
    pub h_mem3: HMem3,
    /// 0x0070
    /// The memory that stores J0
    pub j0_mem0: J0Mem0,
    /// 0x0074
    /// The memory that stores J0
    pub j0_mem1: J0Mem1,
    /// 0x0078
    /// The memory that stores J0
    pub j0_mem2: J0Mem2,
    /// 0x007c
    /// The memory that stores J0
    pub j0_mem3: J0Mem3,
    /// 0x0080
    /// The memory that stores T0
    pub t0_mem0: T0Mem0,
    /// 0x0084
    /// The memory that stores T0
    pub t0_mem1: T0Mem1,
    /// 0x0088
    /// The memory that stores T0
    pub t0_mem2: T0Mem2,
    /// 0x008c
    /// The memory that stores T0
    pub t0_mem3: T0Mem3,
    /// 0x0090
    /// DMA-AES working mode register
    pub dma_enable: DmaEnable,
    /// 0x0094
    /// AES cipher block mode register
    pub block_mode: BlockMode,
    /// 0x0098
    /// AES block number register
    pub block_num: BlockNum,
    /// 0x009c
    /// Standard incrementing function configure register
    pub inc_sel: IncSel,
    /// 0x00a0
    /// Additional Authential Data block number register
    pub aad_block_num: AadBlockNum,
    /// 0x00a4
    /// AES remainder bit number register
    pub remainder_bit_num: RemainderBitNum,
    /// 0x00a8
    /// AES continue register
    pub r#continue: Continue,
    /// 0x00ac
    /// AES Interrupt clear register
    pub int_clear: IntClear,
    /// 0x00b0
    /// AES Interrupt enable register
    pub int_ena: IntEna,
    /// 0x00b4
    /// AES version control register
    pub date: Date,
    /// 0x00b8
    /// AES-DMA exit config
    pub dma_exit: DmaExit,
}
