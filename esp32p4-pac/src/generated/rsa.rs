//! RSA
//! RSA (Rivest Shamir Adleman) Accelerator

#![allow(dead_code)]

pub const BASE: u32 = 0x50092000;

/// M_MEM[0] Register
///
/// Represents M
///
/// Address offset: 0x0000
#[repr(transparent)]
pub struct MMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem0 {}

/// M_MEM[1] Register
///
/// Represents M
///
/// Address offset: 0x0004
#[repr(transparent)]
pub struct MMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem1 {}

/// M_MEM[2] Register
///
/// Represents M
///
/// Address offset: 0x0008
#[repr(transparent)]
pub struct MMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem2 {}

/// M_MEM[3] Register
///
/// Represents M
///
/// Address offset: 0x000c
#[repr(transparent)]
pub struct MMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MMem3 {}

/// Z_MEM[0] Register
///
/// Represents Z
///
/// Address offset: 0x0200
#[repr(transparent)]
pub struct ZMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem0 {}

/// Z_MEM[1] Register
///
/// Represents Z
///
/// Address offset: 0x0204
#[repr(transparent)]
pub struct ZMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem1 {}

/// Z_MEM[2] Register
///
/// Represents Z
///
/// Address offset: 0x0208
#[repr(transparent)]
pub struct ZMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem2 {}

/// Z_MEM[3] Register
///
/// Represents Z
///
/// Address offset: 0x020c
#[repr(transparent)]
pub struct ZMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ZMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ZMem3 {}

/// Y_MEM[0] Register
///
/// Represents Y
///
/// Address offset: 0x0400
#[repr(transparent)]
pub struct YMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem0 {}

/// Y_MEM[1] Register
///
/// Represents Y
///
/// Address offset: 0x0404
#[repr(transparent)]
pub struct YMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem1 {}

/// Y_MEM[2] Register
///
/// Represents Y
///
/// Address offset: 0x0408
#[repr(transparent)]
pub struct YMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem2 {}

/// Y_MEM[3] Register
///
/// Represents Y
///
/// Address offset: 0x040c
#[repr(transparent)]
pub struct YMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for YMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl YMem3 {}

/// X_MEM[0] Register
///
/// Represents X
///
/// Address offset: 0x0600
#[repr(transparent)]
pub struct XMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem0 {}

/// X_MEM[1] Register
///
/// Represents X
///
/// Address offset: 0x0604
#[repr(transparent)]
pub struct XMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem1 {}

/// X_MEM[2] Register
///
/// Represents X
///
/// Address offset: 0x0608
#[repr(transparent)]
pub struct XMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem2 {}

/// X_MEM[3] Register
///
/// Represents X
///
/// Address offset: 0x060c
#[repr(transparent)]
pub struct XMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for XMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl XMem3 {}

/// M_PRIME Register
///
/// Represents M’
///
/// Address offset: 0x0800
///
/// ## Fields
///
/// - **M_PRIME** `[31:0]` (RW)
///
///   Represents M’
#[repr(transparent)]
pub struct MPrime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MPrime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MPrime {
    // ======== Field Constants ========

    /// **M_PRIME**
    ///
    /// Represents M’
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const M_PRIME_OFFSET: u32 = 0;
    pub const M_PRIME_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **M_PRIME** `[31:0]` (RW)
    ///
    /// Represents M’
    #[inline(always)]
    pub fn m_prime(&self) -> u32 {
        (self.value.read_bits() & Self::M_PRIME_MASK) >> Self::M_PRIME_OFFSET
    }

    /// Write **M_PRIME** `[31:0]` (RW)
    ///
    /// Represents M’
    #[inline(always)]
    pub fn write_m_prime(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::M_PRIME_MASK) | ((value << Self::M_PRIME_OFFSET) & Self::M_PRIME_MASK)
        });
    }
}

/// M_PRIME value type for batch read/modify/write operations
///
/// Use `mprime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mprime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MPrimeVal {
    bits: u32,
}

impl MPrimeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **M_PRIME** `[31:0]`
    ///
    /// Represents M’
    #[inline(always)]
    pub const fn m_prime(&self) -> u32 {
        (self.bits & MPrime::M_PRIME_MASK) >> MPrime::M_PRIME_OFFSET
    }

    /// Set **M_PRIME** `[31:0]`
    ///
    /// Represents M’
    #[inline(always)]
    pub const fn with_m_prime(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !MPrime::M_PRIME_MASK)
                | ((value << MPrime::M_PRIME_OFFSET) & MPrime::M_PRIME_MASK),
        }
    }
}

impl Default for MPrimeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MPrime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MPrimeVal {
        MPrimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MPrimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MPrimeVal) -> MPrimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MODE Register
///
/// Configures RSA length
///
/// Address offset: 0x0804
///
/// ## Fields
///
/// - **MODE** `[6:0]` (RW)
///
///   Configures the RSA length.
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
    /// Configures the RSA length.
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const MODE_OFFSET: u32 = 0;
    pub const MODE_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **MODE** `[6:0]` (RW)
    ///
    /// Configures the RSA length.
    #[inline(always)]
    pub fn mode(&self) -> u32 {
        (self.value.read_bits() & Self::MODE_MASK) >> Self::MODE_OFFSET
    }

    /// Write **MODE** `[6:0]` (RW)
    ///
    /// Configures the RSA length.
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

    /// Read **MODE** `[6:0]`
    ///
    /// Configures the RSA length.
    #[inline(always)]
    pub const fn mode(&self) -> u32 {
        (self.bits & Mode::MODE_MASK) >> Mode::MODE_OFFSET
    }

    /// Set **MODE** `[6:0]`
    ///
    /// Configures the RSA length.
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

/// QUERY_CLEAN Register
///
/// RSA clean register
///
/// Address offset: 0x0808
///
/// ## Fields
///
/// - **QUERY_CLEAN** `[0]` (RO)
///
///   Represents whether or not the RSA memory completes initialization.
///   0: Not complete
///   1: Completed
#[repr(transparent)]
pub struct QueryClean {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for QueryClean {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl QueryClean {
    // ======== Field Constants ========

    /// **QUERY_CLEAN**
    ///
    /// Represents whether or not the RSA memory completes initialization.
    /// 0: Not complete
    /// 1: Completed
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const QUERY_CLEAN_OFFSET: u32 = 0;
    pub const QUERY_CLEAN_MASK: u32 = 0x00000001;
    pub const QUERY_CLEAN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **QUERY_CLEAN** `[0]` (RO)
    ///
    /// Represents whether or not the RSA memory completes initialization.
    /// 0: Not complete
    /// 1: Completed
    #[inline(always)]
    pub fn query_clean(&self) -> bool {
        (self.value.read_bits() & Self::QUERY_CLEAN_MASK) != 0
    }
}

/// QUERY_CLEAN value type for batch read/modify/write operations
///
/// Use `queryclean.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `queryclean.write_val(val)`.
#[derive(Clone, Copy)]
pub struct QueryCleanVal {
    bits: u32,
}

impl QueryCleanVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **QUERY_CLEAN** `[0:0]`
    ///
    /// Represents whether or not the RSA memory completes initialization.
    /// 0: Not complete
    /// 1: Completed
    #[inline(always)]
    pub const fn query_clean(&self) -> bool {
        (self.bits & QueryClean::QUERY_CLEAN_MASK) != 0
    }
}

impl Default for QueryCleanVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl QueryClean {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> QueryCleanVal {
        QueryCleanVal::from_bits(self.value.read_bits())
    }
}

/// SET_START_MODEXP Register
///
/// Starts modular exponentiation
///
/// Address offset: 0x080c
///
/// ## Fields
///
/// - **SET_START_MODEXP** `[0]` (WO)
///
///   Configure whether or not to start the modular exponentiation.
///   0: No effect
///   1: Start
#[repr(transparent)]
pub struct SetStartModexp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetStartModexp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetStartModexp {
    // ======== Field Constants ========

    /// **SET_START_MODEXP**
    ///
    /// Configure whether or not to start the modular exponentiation.
    /// 0: No effect
    /// 1: Start
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_START_MODEXP_OFFSET: u32 = 0;
    pub const SET_START_MODEXP_MASK: u32 = 0x00000001;
    pub const SET_START_MODEXP: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_START_MODEXP** (WO)
    ///
    /// Configure whether or not to start the modular exponentiation.
    /// 0: No effect
    /// 1: Start
    #[inline(always)]
    pub fn trigger_set_start_modexp(&self) {
        self.value.write_bits(Self::SET_START_MODEXP_MASK);
    }
}

/// SET_START_MODEXP value type for batch read/modify/write operations
///
/// Use `setstartmodexp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setstartmodexp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetStartModexpVal {
    bits: u32,
}

impl SetStartModexpVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_START_MODEXP** `[0:0]`
    ///
    /// Configure whether or not to start the modular exponentiation.
    /// 0: No effect
    /// 1: Start
    #[inline(always)]
    pub const fn with_set_start_modexp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetStartModexp::SET_START_MODEXP_MASK
            } else {
                self.bits & !SetStartModexp::SET_START_MODEXP_MASK
            },
        }
    }
}

impl Default for SetStartModexpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetStartModexp {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetStartModexpVal) {
        self.value.write_bits(val.bits())
    }
}

/// SET_START_MODMULT Register
///
/// Starts modular multiplication
///
/// Address offset: 0x0810
///
/// ## Fields
///
/// - **SET_START_MODMULT** `[0]` (WO)
///
///   Configure whether or not to start the modular multiplication.
///   0: No effect
///   1: Start
#[repr(transparent)]
pub struct SetStartModmult {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetStartModmult {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetStartModmult {
    // ======== Field Constants ========

    /// **SET_START_MODMULT**
    ///
    /// Configure whether or not to start the modular multiplication.
    /// 0: No effect
    /// 1: Start
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_START_MODMULT_OFFSET: u32 = 0;
    pub const SET_START_MODMULT_MASK: u32 = 0x00000001;
    pub const SET_START_MODMULT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_START_MODMULT** (WO)
    ///
    /// Configure whether or not to start the modular multiplication.
    /// 0: No effect
    /// 1: Start
    #[inline(always)]
    pub fn trigger_set_start_modmult(&self) {
        self.value.write_bits(Self::SET_START_MODMULT_MASK);
    }
}

/// SET_START_MODMULT value type for batch read/modify/write operations
///
/// Use `setstartmodmult.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setstartmodmult.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetStartModmultVal {
    bits: u32,
}

impl SetStartModmultVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_START_MODMULT** `[0:0]`
    ///
    /// Configure whether or not to start the modular multiplication.
    /// 0: No effect
    /// 1: Start
    #[inline(always)]
    pub const fn with_set_start_modmult(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetStartModmult::SET_START_MODMULT_MASK
            } else {
                self.bits & !SetStartModmult::SET_START_MODMULT_MASK
            },
        }
    }
}

impl Default for SetStartModmultVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetStartModmult {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetStartModmultVal) {
        self.value.write_bits(val.bits())
    }
}

/// SET_START_MULT Register
///
/// Starts multiplication
///
/// Address offset: 0x0814
///
/// ## Fields
///
/// - **SET_START_MULT** `[0]` (WO)
///
///   Configure whether or not to start the multiplication.
///   0: No effect
///   1: Start
#[repr(transparent)]
pub struct SetStartMult {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetStartMult {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetStartMult {
    // ======== Field Constants ========

    /// **SET_START_MULT**
    ///
    /// Configure whether or not to start the multiplication.
    /// 0: No effect
    /// 1: Start
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_START_MULT_OFFSET: u32 = 0;
    pub const SET_START_MULT_MASK: u32 = 0x00000001;
    pub const SET_START_MULT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_START_MULT** (WO)
    ///
    /// Configure whether or not to start the multiplication.
    /// 0: No effect
    /// 1: Start
    #[inline(always)]
    pub fn trigger_set_start_mult(&self) {
        self.value.write_bits(Self::SET_START_MULT_MASK);
    }
}

/// SET_START_MULT value type for batch read/modify/write operations
///
/// Use `setstartmult.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setstartmult.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetStartMultVal {
    bits: u32,
}

impl SetStartMultVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_START_MULT** `[0:0]`
    ///
    /// Configure whether or not to start the multiplication.
    /// 0: No effect
    /// 1: Start
    #[inline(always)]
    pub const fn with_set_start_mult(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetStartMult::SET_START_MULT_MASK
            } else {
                self.bits & !SetStartMult::SET_START_MULT_MASK
            },
        }
    }
}

impl Default for SetStartMultVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetStartMult {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetStartMultVal) {
        self.value.write_bits(val.bits())
    }
}

/// QUERY_IDLE Register
///
/// Represents the RSA status
///
/// Address offset: 0x0818
///
/// ## Fields
///
/// - **QUERY_IDLE** `[0]` (RO)
///
///   Represents the RSA status.
///   0: Busy
///   1: Idle
#[repr(transparent)]
pub struct QueryIdle {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for QueryIdle {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl QueryIdle {
    // ======== Field Constants ========

    /// **QUERY_IDLE**
    ///
    /// Represents the RSA status.
    /// 0: Busy
    /// 1: Idle
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const QUERY_IDLE_OFFSET: u32 = 0;
    pub const QUERY_IDLE_MASK: u32 = 0x00000001;
    pub const QUERY_IDLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **QUERY_IDLE** `[0]` (RO)
    ///
    /// Represents the RSA status.
    /// 0: Busy
    /// 1: Idle
    #[inline(always)]
    pub fn query_idle(&self) -> bool {
        (self.value.read_bits() & Self::QUERY_IDLE_MASK) != 0
    }
}

/// QUERY_IDLE value type for batch read/modify/write operations
///
/// Use `queryidle.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `queryidle.write_val(val)`.
#[derive(Clone, Copy)]
pub struct QueryIdleVal {
    bits: u32,
}

impl QueryIdleVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **QUERY_IDLE** `[0:0]`
    ///
    /// Represents the RSA status.
    /// 0: Busy
    /// 1: Idle
    #[inline(always)]
    pub const fn query_idle(&self) -> bool {
        (self.bits & QueryIdle::QUERY_IDLE_MASK) != 0
    }
}

impl Default for QueryIdleVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl QueryIdle {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> QueryIdleVal {
        QueryIdleVal::from_bits(self.value.read_bits())
    }
}

/// INT_CLR Register
///
/// Clears RSA interrupt
///
/// Address offset: 0x081c
///
/// ## Fields
///
/// - **CLEAR_INTERRUPT** `[0]` (WO)
///
///   Write 1 to clear the RSA interrupt.
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

    /// **CLEAR_INTERRUPT**
    ///
    /// Write 1 to clear the RSA interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLEAR_INTERRUPT_OFFSET: u32 = 0;
    pub const CLEAR_INTERRUPT_MASK: u32 = 0x00000001;
    pub const CLEAR_INTERRUPT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CLEAR_INTERRUPT** (WO)
    ///
    /// Write 1 to clear the RSA interrupt.
    #[inline(always)]
    pub fn trigger_clear_interrupt(&self) {
        self.value.write_bits(Self::CLEAR_INTERRUPT_MASK);
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

    /// Set **CLEAR_INTERRUPT** `[0:0]`
    ///
    /// Write 1 to clear the RSA interrupt.
    #[inline(always)]
    pub const fn with_clear_interrupt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::CLEAR_INTERRUPT_MASK
            } else {
                self.bits & !IntClr::CLEAR_INTERRUPT_MASK
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

/// CONSTANT_TIME Register
///
/// Configures the constant_time option
///
/// Address offset: 0x0820
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CONSTANT_TIME** `[0]` (RW)
///
///   Configures the constant_time option.
///   0: Acceleration
///   1: No acceleration (default)
#[repr(transparent)]
pub struct ConstantTime {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ConstantTime {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ConstantTime {
    // ======== Field Constants ========

    /// **CONSTANT_TIME**
    ///
    /// Configures the constant_time option.
    /// 0: Acceleration
    /// 1: No acceleration (default)
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CONSTANT_TIME_OFFSET: u32 = 0;
    pub const CONSTANT_TIME_MASK: u32 = 0x00000001;
    pub const CONSTANT_TIME: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CONSTANT_TIME** `[0]` (RW)
    ///
    /// Configures the constant_time option.
    /// 0: Acceleration
    /// 1: No acceleration (default)
    #[inline(always)]
    pub fn constant_time(&self) -> bool {
        (self.value.read_bits() & Self::CONSTANT_TIME_MASK) != 0
    }

    /// Set CONSTANT_TIME bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_constant_time(&self) {
        self.value.modify_bits(|r| r | Self::CONSTANT_TIME_MASK);
    }

    /// Clear CONSTANT_TIME bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_constant_time(&self) {
        self.value.modify_bits(|r| r & !Self::CONSTANT_TIME_MASK);
    }
}

/// CONSTANT_TIME value type for batch read/modify/write operations
///
/// Use `constanttime.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `constanttime.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ConstantTimeVal {
    bits: u32,
}

impl ConstantTimeVal {
    /// Create from raw bits
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

    /// Read **CONSTANT_TIME** `[0:0]`
    ///
    /// Configures the constant_time option.
    /// 0: Acceleration
    /// 1: No acceleration (default)
    #[inline(always)]
    pub const fn constant_time(&self) -> bool {
        (self.bits & ConstantTime::CONSTANT_TIME_MASK) != 0
    }

    /// Set **CONSTANT_TIME** `[0:0]`
    ///
    /// Configures the constant_time option.
    /// 0: Acceleration
    /// 1: No acceleration (default)
    #[inline(always)]
    pub const fn with_constant_time(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ConstantTime::CONSTANT_TIME_MASK
            } else {
                self.bits & !ConstantTime::CONSTANT_TIME_MASK
            },
        }
    }
}

impl Default for ConstantTimeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ConstantTime {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ConstantTimeVal {
        ConstantTimeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ConstantTimeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ConstantTimeVal) -> ConstantTimeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SEARCH_ENABLE Register
///
/// Configures the search option
///
/// Address offset: 0x0824
///
/// ## Fields
///
/// - **SEARCH_ENABLE** `[0]` (RW)
///
///   Configure the search option.
///   0: No acceleration (default)
///   1: Acceleration
///   This option should be used together with RSA_SEARCH_POS.
#[repr(transparent)]
pub struct SearchEnable {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SearchEnable {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SearchEnable {
    // ======== Field Constants ========

    /// **SEARCH_ENABLE**
    ///
    /// Configure the search option.
    /// 0: No acceleration (default)
    /// 1: Acceleration
    /// This option should be used together with RSA_SEARCH_POS.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SEARCH_ENABLE_OFFSET: u32 = 0;
    pub const SEARCH_ENABLE_MASK: u32 = 0x00000001;
    pub const SEARCH_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SEARCH_ENABLE** `[0]` (RW)
    ///
    /// Configure the search option.
    /// 0: No acceleration (default)
    /// 1: Acceleration
    /// This option should be used together with RSA_SEARCH_POS.
    #[inline(always)]
    pub fn search_enable(&self) -> bool {
        (self.value.read_bits() & Self::SEARCH_ENABLE_MASK) != 0
    }

    /// Set SEARCH_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_search_enable(&self) {
        self.value.modify_bits(|r| r | Self::SEARCH_ENABLE_MASK);
    }

    /// Clear SEARCH_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_search_enable(&self) {
        self.value.modify_bits(|r| r & !Self::SEARCH_ENABLE_MASK);
    }
}

/// SEARCH_ENABLE value type for batch read/modify/write operations
///
/// Use `searchenable.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `searchenable.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SearchEnableVal {
    bits: u32,
}

impl SearchEnableVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEARCH_ENABLE** `[0:0]`
    ///
    /// Configure the search option.
    /// 0: No acceleration (default)
    /// 1: Acceleration
    /// This option should be used together with RSA_SEARCH_POS.
    #[inline(always)]
    pub const fn search_enable(&self) -> bool {
        (self.bits & SearchEnable::SEARCH_ENABLE_MASK) != 0
    }

    /// Set **SEARCH_ENABLE** `[0:0]`
    ///
    /// Configure the search option.
    /// 0: No acceleration (default)
    /// 1: Acceleration
    /// This option should be used together with RSA_SEARCH_POS.
    #[inline(always)]
    pub const fn with_search_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SearchEnable::SEARCH_ENABLE_MASK
            } else {
                self.bits & !SearchEnable::SEARCH_ENABLE_MASK
            },
        }
    }
}

impl Default for SearchEnableVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SearchEnable {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SearchEnableVal {
        SearchEnableVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SearchEnableVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SearchEnableVal) -> SearchEnableVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SEARCH_POS Register
///
/// Configures the search position
///
/// Address offset: 0x0828
///
/// ## Fields
///
/// - **SEARCH_POS** `[11:0]` (RW)
///
///   Configures the starting address to start search. This field should be used together with RSA_SEARCH_ENABLE. The field is only valid when RSA_SEARCH_ENABLE is high.
#[repr(transparent)]
pub struct SearchPos {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SearchPos {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SearchPos {
    // ======== Field Constants ========

    /// **SEARCH_POS**
    ///
    /// Configures the starting address to start search. This field should be used together with RSA_SEARCH_ENABLE. The field is only valid when RSA_SEARCH_ENABLE is high.
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const SEARCH_POS_OFFSET: u32 = 0;
    pub const SEARCH_POS_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **SEARCH_POS** `[11:0]` (RW)
    ///
    /// Configures the starting address to start search. This field should be used together with RSA_SEARCH_ENABLE. The field is only valid when RSA_SEARCH_ENABLE is high.
    #[inline(always)]
    pub fn search_pos(&self) -> u32 {
        (self.value.read_bits() & Self::SEARCH_POS_MASK) >> Self::SEARCH_POS_OFFSET
    }

    /// Write **SEARCH_POS** `[11:0]` (RW)
    ///
    /// Configures the starting address to start search. This field should be used together with RSA_SEARCH_ENABLE. The field is only valid when RSA_SEARCH_ENABLE is high.
    #[inline(always)]
    pub fn write_search_pos(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEARCH_POS_MASK)
                | ((value << Self::SEARCH_POS_OFFSET) & Self::SEARCH_POS_MASK)
        });
    }
}

/// SEARCH_POS value type for batch read/modify/write operations
///
/// Use `searchpos.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `searchpos.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SearchPosVal {
    bits: u32,
}

impl SearchPosVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEARCH_POS** `[11:0]`
    ///
    /// Configures the starting address to start search. This field should be used together with RSA_SEARCH_ENABLE. The field is only valid when RSA_SEARCH_ENABLE is high.
    #[inline(always)]
    pub const fn search_pos(&self) -> u32 {
        (self.bits & SearchPos::SEARCH_POS_MASK) >> SearchPos::SEARCH_POS_OFFSET
    }

    /// Set **SEARCH_POS** `[11:0]`
    ///
    /// Configures the starting address to start search. This field should be used together with RSA_SEARCH_ENABLE. The field is only valid when RSA_SEARCH_ENABLE is high.
    #[inline(always)]
    pub const fn with_search_pos(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SearchPos::SEARCH_POS_MASK)
                | ((value << SearchPos::SEARCH_POS_OFFSET) & SearchPos::SEARCH_POS_MASK),
        }
    }
}

impl Default for SearchPosVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SearchPos {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SearchPosVal {
        SearchPosVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SearchPosVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SearchPosVal) -> SearchPosVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ENA Register
///
/// Enables the RSA interrupt
///
/// Address offset: 0x082c
///
/// ## Fields
///
/// - **INT_ENA** `[0]` (RW)
///
///   Write 1 to enable the RSA interrupt.
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
    /// Write 1 to enable the RSA interrupt.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const INT_ENA_OFFSET: u32 = 0;
    pub const INT_ENA_MASK: u32 = 0x00000001;
    pub const INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **INT_ENA** `[0]` (RW)
    ///
    /// Write 1 to enable the RSA interrupt.
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
    /// Write 1 to enable the RSA interrupt.
    #[inline(always)]
    pub const fn int_ena(&self) -> bool {
        (self.bits & IntEna::INT_ENA_MASK) != 0
    }

    /// Set **INT_ENA** `[0:0]`
    ///
    /// Write 1 to enable the RSA interrupt.
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
/// Version control register
///
/// Address offset: 0x0830
/// Reset value: 0x20200618
///
/// ## Fields
///
/// - **DATE** `[29:0]` (RW)
///
///   Version control register.
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
    /// Version control register.
    ///
    /// Bits `[29:0]`, mask `0x3fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x3fffffff;

    // ======== Field Accessors ========

    /// **DATE** `[29:0]` (RW)
    ///
    /// Version control register.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[29:0]` (RW)
    ///
    /// Version control register.
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
        Self { bits: 0x20200618 }
    }

    /// Read **DATE** `[29:0]`
    ///
    /// Version control register.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[29:0]`
    ///
    /// Version control register.
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
    /// Represents M
    pub m_mem0: MMem0,
    /// 0x0004
    /// Represents M
    pub m_mem1: MMem1,
    /// 0x0008
    /// Represents M
    pub m_mem2: MMem2,
    /// 0x000c
    /// Represents M
    pub m_mem3: MMem3,
    _reserved0: [u8; 496],
    /// 0x0200
    /// Represents Z
    pub z_mem0: ZMem0,
    /// 0x0204
    /// Represents Z
    pub z_mem1: ZMem1,
    /// 0x0208
    /// Represents Z
    pub z_mem2: ZMem2,
    /// 0x020c
    /// Represents Z
    pub z_mem3: ZMem3,
    _reserved1: [u8; 496],
    /// 0x0400
    /// Represents Y
    pub y_mem0: YMem0,
    /// 0x0404
    /// Represents Y
    pub y_mem1: YMem1,
    /// 0x0408
    /// Represents Y
    pub y_mem2: YMem2,
    /// 0x040c
    /// Represents Y
    pub y_mem3: YMem3,
    _reserved2: [u8; 496],
    /// 0x0600
    /// Represents X
    pub x_mem0: XMem0,
    /// 0x0604
    /// Represents X
    pub x_mem1: XMem1,
    /// 0x0608
    /// Represents X
    pub x_mem2: XMem2,
    /// 0x060c
    /// Represents X
    pub x_mem3: XMem3,
    _reserved3: [u8; 496],
    /// 0x0800
    /// Represents M’
    pub m_prime: MPrime,
    /// 0x0804
    /// Configures RSA length
    pub mode: Mode,
    /// 0x0808
    /// RSA clean register
    pub query_clean: QueryClean,
    /// 0x080c
    /// Starts modular exponentiation
    pub set_start_modexp: SetStartModexp,
    /// 0x0810
    /// Starts modular multiplication
    pub set_start_modmult: SetStartModmult,
    /// 0x0814
    /// Starts multiplication
    pub set_start_mult: SetStartMult,
    /// 0x0818
    /// Represents the RSA status
    pub query_idle: QueryIdle,
    /// 0x081c
    /// Clears RSA interrupt
    pub int_clr: IntClr,
    /// 0x0820
    /// Configures the constant_time option
    pub constant_time: ConstantTime,
    /// 0x0824
    /// Configures the search option
    pub search_enable: SearchEnable,
    /// 0x0828
    /// Configures the search position
    pub search_pos: SearchPos,
    /// 0x082c
    /// Enables the RSA interrupt
    pub int_ena: IntEna,
    /// 0x0830
    /// Version control register
    pub date: Date,
}
