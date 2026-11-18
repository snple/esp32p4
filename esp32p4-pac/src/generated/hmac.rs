//! HMAC
//! HMAC (Hash-based Message Authentication Code) Accelerator

#![allow(dead_code)]

pub const BASE: u32 = 0x50095000;

/// SET_START Register
///
/// Process control register 0.
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **SET_START** `[0]` (WO)
///
///   Start hmac operation.
#[repr(transparent)]
pub struct SetStart {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetStart {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetStart {
    // ======== Field Constants ========

    /// **SET_START**
    ///
    /// Start hmac operation.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_START_OFFSET: u32 = 0;
    pub const SET_START_MASK: u32 = 0x00000001;
    pub const SET_START: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_START** (WO)
    ///
    /// Start hmac operation.
    #[inline(always)]
    pub fn trigger_set_start(&self) {
        self.value.write_bits(Self::SET_START_MASK);
    }
}

/// SET_START value type for batch read/modify/write operations
///
/// Use `setstart.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setstart.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetStartVal {
    bits: u32,
}

impl SetStartVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_START** `[0:0]`
    ///
    /// Start hmac operation.
    #[inline(always)]
    pub const fn with_set_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetStart::SET_START_MASK
            } else {
                self.bits & !SetStart::SET_START_MASK
            },
        }
    }
}

impl Default for SetStartVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetStart {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetStartVal) {
        self.value.write_bits(val.bits())
    }
}

/// SET_PARA_PURPOSE Register
///
/// Configure purpose.
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **PURPOSE_SET** `[3:0]` (WO)
///
///   Set hmac parameter purpose.
#[repr(transparent)]
pub struct SetParaPurpose {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetParaPurpose {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetParaPurpose {
    // ======== Field Constants ========

    /// **PURPOSE_SET**
    ///
    /// Set hmac parameter purpose.
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const PURPOSE_SET_OFFSET: u32 = 0;
    pub const PURPOSE_SET_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// Write **PURPOSE_SET** `[3:0]` (WO)
    ///
    /// Set hmac parameter purpose.
    #[inline(always)]
    pub fn write_purpose_set(&self, value: u32) {
        self.value
            .write_bits((value << Self::PURPOSE_SET_OFFSET) & Self::PURPOSE_SET_MASK);
    }
}

/// SET_PARA_PURPOSE value type for batch read/modify/write operations
///
/// Use `setparapurpose.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setparapurpose.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetParaPurposeVal {
    bits: u32,
}

impl SetParaPurposeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **PURPOSE_SET** `[3:0]`
    ///
    /// Set hmac parameter purpose.
    #[inline(always)]
    pub const fn with_purpose_set(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SetParaPurpose::PURPOSE_SET_MASK)
                | ((value << SetParaPurpose::PURPOSE_SET_OFFSET)
                    & SetParaPurpose::PURPOSE_SET_MASK),
        }
    }
}

impl Default for SetParaPurposeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetParaPurpose {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetParaPurposeVal) {
        self.value.write_bits(val.bits())
    }
}

/// SET_PARA_KEY Register
///
/// Configure key.
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **KEY_SET** `[2:0]` (WO)
///
///   Set hmac parameter key.
#[repr(transparent)]
pub struct SetParaKey {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetParaKey {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetParaKey {
    // ======== Field Constants ========

    /// **KEY_SET**
    ///
    /// Set hmac parameter key.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const KEY_SET_OFFSET: u32 = 0;
    pub const KEY_SET_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// Write **KEY_SET** `[2:0]` (WO)
    ///
    /// Set hmac parameter key.
    #[inline(always)]
    pub fn write_key_set(&self, value: u32) {
        self.value
            .write_bits((value << Self::KEY_SET_OFFSET) & Self::KEY_SET_MASK);
    }
}

/// SET_PARA_KEY value type for batch read/modify/write operations
///
/// Use `setparakey.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setparakey.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetParaKeyVal {
    bits: u32,
}

impl SetParaKeyVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **KEY_SET** `[2:0]`
    ///
    /// Set hmac parameter key.
    #[inline(always)]
    pub const fn with_key_set(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SetParaKey::KEY_SET_MASK)
                | ((value << SetParaKey::KEY_SET_OFFSET) & SetParaKey::KEY_SET_MASK),
        }
    }
}

impl Default for SetParaKeyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetParaKey {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetParaKeyVal) {
        self.value.write_bits(val.bits())
    }
}

/// SET_PARA_FINISH Register
///
/// Finish initial configuration.
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **SET_PARA_END** `[0]` (WO)
///
///   Finish hmac configuration.
#[repr(transparent)]
pub struct SetParaFinish {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetParaFinish {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetParaFinish {
    // ======== Field Constants ========

    /// **SET_PARA_END**
    ///
    /// Finish hmac configuration.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_PARA_END_OFFSET: u32 = 0;
    pub const SET_PARA_END_MASK: u32 = 0x00000001;
    pub const SET_PARA_END: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_PARA_END** (WO)
    ///
    /// Finish hmac configuration.
    #[inline(always)]
    pub fn trigger_set_para_end(&self) {
        self.value.write_bits(Self::SET_PARA_END_MASK);
    }
}

/// SET_PARA_FINISH value type for batch read/modify/write operations
///
/// Use `setparafinish.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setparafinish.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetParaFinishVal {
    bits: u32,
}

impl SetParaFinishVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_PARA_END** `[0:0]`
    ///
    /// Finish hmac configuration.
    #[inline(always)]
    pub const fn with_set_para_end(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetParaFinish::SET_PARA_END_MASK
            } else {
                self.bits & !SetParaFinish::SET_PARA_END_MASK
            },
        }
    }
}

impl Default for SetParaFinishVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetParaFinish {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetParaFinishVal) {
        self.value.write_bits(val.bits())
    }
}

/// SET_MESSAGE_ONE Register
///
/// Process control register 1.
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **SET_TEXT_ONE** `[0]` (WO)
///
///   Call SHA to calculate one message block.
#[repr(transparent)]
pub struct SetMessageOne {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetMessageOne {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetMessageOne {
    // ======== Field Constants ========

    /// **SET_TEXT_ONE**
    ///
    /// Call SHA to calculate one message block.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_TEXT_ONE_OFFSET: u32 = 0;
    pub const SET_TEXT_ONE_MASK: u32 = 0x00000001;
    pub const SET_TEXT_ONE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_TEXT_ONE** (WO)
    ///
    /// Call SHA to calculate one message block.
    #[inline(always)]
    pub fn trigger_set_text_one(&self) {
        self.value.write_bits(Self::SET_TEXT_ONE_MASK);
    }
}

/// SET_MESSAGE_ONE value type for batch read/modify/write operations
///
/// Use `setmessageone.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setmessageone.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetMessageOneVal {
    bits: u32,
}

impl SetMessageOneVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_TEXT_ONE** `[0:0]`
    ///
    /// Call SHA to calculate one message block.
    #[inline(always)]
    pub const fn with_set_text_one(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetMessageOne::SET_TEXT_ONE_MASK
            } else {
                self.bits & !SetMessageOne::SET_TEXT_ONE_MASK
            },
        }
    }
}

impl Default for SetMessageOneVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetMessageOne {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetMessageOneVal) {
        self.value.write_bits(val.bits())
    }
}

/// SET_MESSAGE_ING Register
///
/// Process control register 2.
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **SET_TEXT_ING** `[0]` (WO)
///
///   Continue typical hmac.
#[repr(transparent)]
pub struct SetMessageIng {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetMessageIng {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetMessageIng {
    // ======== Field Constants ========

    /// **SET_TEXT_ING**
    ///
    /// Continue typical hmac.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_TEXT_ING_OFFSET: u32 = 0;
    pub const SET_TEXT_ING_MASK: u32 = 0x00000001;
    pub const SET_TEXT_ING: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_TEXT_ING** (WO)
    ///
    /// Continue typical hmac.
    #[inline(always)]
    pub fn trigger_set_text_ing(&self) {
        self.value.write_bits(Self::SET_TEXT_ING_MASK);
    }
}

/// SET_MESSAGE_ING value type for batch read/modify/write operations
///
/// Use `setmessageing.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setmessageing.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetMessageIngVal {
    bits: u32,
}

impl SetMessageIngVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_TEXT_ING** `[0:0]`
    ///
    /// Continue typical hmac.
    #[inline(always)]
    pub const fn with_set_text_ing(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetMessageIng::SET_TEXT_ING_MASK
            } else {
                self.bits & !SetMessageIng::SET_TEXT_ING_MASK
            },
        }
    }
}

impl Default for SetMessageIngVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetMessageIng {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetMessageIngVal) {
        self.value.write_bits(val.bits())
    }
}

/// SET_MESSAGE_END Register
///
/// Process control register 3.
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **SET_TEXT_END** `[0]` (WO)
///
///   Start hardware padding.
#[repr(transparent)]
pub struct SetMessageEnd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetMessageEnd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetMessageEnd {
    // ======== Field Constants ========

    /// **SET_TEXT_END**
    ///
    /// Start hardware padding.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_TEXT_END_OFFSET: u32 = 0;
    pub const SET_TEXT_END_MASK: u32 = 0x00000001;
    pub const SET_TEXT_END: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_TEXT_END** (WO)
    ///
    /// Start hardware padding.
    #[inline(always)]
    pub fn trigger_set_text_end(&self) {
        self.value.write_bits(Self::SET_TEXT_END_MASK);
    }
}

/// SET_MESSAGE_END value type for batch read/modify/write operations
///
/// Use `setmessageend.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setmessageend.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetMessageEndVal {
    bits: u32,
}

impl SetMessageEndVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_TEXT_END** `[0:0]`
    ///
    /// Start hardware padding.
    #[inline(always)]
    pub const fn with_set_text_end(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetMessageEnd::SET_TEXT_END_MASK
            } else {
                self.bits & !SetMessageEnd::SET_TEXT_END_MASK
            },
        }
    }
}

impl Default for SetMessageEndVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetMessageEnd {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetMessageEndVal) {
        self.value.write_bits(val.bits())
    }
}

/// SET_RESULT_FINISH Register
///
/// Process control register 4.
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **SET_RESULT_END** `[0]` (WO)
///
///   After read result from upstream, then let hmac back to idle.
#[repr(transparent)]
pub struct SetResultFinish {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetResultFinish {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetResultFinish {
    // ======== Field Constants ========

    /// **SET_RESULT_END**
    ///
    /// After read result from upstream, then let hmac back to idle.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_RESULT_END_OFFSET: u32 = 0;
    pub const SET_RESULT_END_MASK: u32 = 0x00000001;
    pub const SET_RESULT_END: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_RESULT_END** (WO)
    ///
    /// After read result from upstream, then let hmac back to idle.
    #[inline(always)]
    pub fn trigger_set_result_end(&self) {
        self.value.write_bits(Self::SET_RESULT_END_MASK);
    }
}

/// SET_RESULT_FINISH value type for batch read/modify/write operations
///
/// Use `setresultfinish.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setresultfinish.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetResultFinishVal {
    bits: u32,
}

impl SetResultFinishVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_RESULT_END** `[0:0]`
    ///
    /// After read result from upstream, then let hmac back to idle.
    #[inline(always)]
    pub const fn with_set_result_end(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetResultFinish::SET_RESULT_END_MASK
            } else {
                self.bits & !SetResultFinish::SET_RESULT_END_MASK
            },
        }
    }
}

impl Default for SetResultFinishVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetResultFinish {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetResultFinishVal) {
        self.value.write_bits(val.bits())
    }
}

/// SET_INVALIDATE_JTAG Register
///
/// Invalidate register 0.
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **SET_INVALIDATE_JTAG** `[0]` (WO)
///
///   Clear result from hmac downstream JTAG.
#[repr(transparent)]
pub struct SetInvalidateJtag {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetInvalidateJtag {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetInvalidateJtag {
    // ======== Field Constants ========

    /// **SET_INVALIDATE_JTAG**
    ///
    /// Clear result from hmac downstream JTAG.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_INVALIDATE_JTAG_OFFSET: u32 = 0;
    pub const SET_INVALIDATE_JTAG_MASK: u32 = 0x00000001;
    pub const SET_INVALIDATE_JTAG: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_INVALIDATE_JTAG** (WO)
    ///
    /// Clear result from hmac downstream JTAG.
    #[inline(always)]
    pub fn trigger_set_invalidate_jtag(&self) {
        self.value.write_bits(Self::SET_INVALIDATE_JTAG_MASK);
    }
}

/// SET_INVALIDATE_JTAG value type for batch read/modify/write operations
///
/// Use `setinvalidatejtag.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setinvalidatejtag.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetInvalidateJtagVal {
    bits: u32,
}

impl SetInvalidateJtagVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_INVALIDATE_JTAG** `[0:0]`
    ///
    /// Clear result from hmac downstream JTAG.
    #[inline(always)]
    pub const fn with_set_invalidate_jtag(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetInvalidateJtag::SET_INVALIDATE_JTAG_MASK
            } else {
                self.bits & !SetInvalidateJtag::SET_INVALIDATE_JTAG_MASK
            },
        }
    }
}

impl Default for SetInvalidateJtagVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetInvalidateJtag {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetInvalidateJtagVal) {
        self.value.write_bits(val.bits())
    }
}

/// SET_INVALIDATE_DS Register
///
/// Invalidate register 1.
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **SET_INVALIDATE_DS** `[0]` (WO)
///
///   Clear result from hmac downstream DS.
#[repr(transparent)]
pub struct SetInvalidateDs {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetInvalidateDs {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetInvalidateDs {
    // ======== Field Constants ========

    /// **SET_INVALIDATE_DS**
    ///
    /// Clear result from hmac downstream DS.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_INVALIDATE_DS_OFFSET: u32 = 0;
    pub const SET_INVALIDATE_DS_MASK: u32 = 0x00000001;
    pub const SET_INVALIDATE_DS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_INVALIDATE_DS** (WO)
    ///
    /// Clear result from hmac downstream DS.
    #[inline(always)]
    pub fn trigger_set_invalidate_ds(&self) {
        self.value.write_bits(Self::SET_INVALIDATE_DS_MASK);
    }
}

/// SET_INVALIDATE_DS value type for batch read/modify/write operations
///
/// Use `setinvalidateds.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setinvalidateds.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetInvalidateDsVal {
    bits: u32,
}

impl SetInvalidateDsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_INVALIDATE_DS** `[0:0]`
    ///
    /// Clear result from hmac downstream DS.
    #[inline(always)]
    pub const fn with_set_invalidate_ds(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetInvalidateDs::SET_INVALIDATE_DS_MASK
            } else {
                self.bits & !SetInvalidateDs::SET_INVALIDATE_DS_MASK
            },
        }
    }
}

impl Default for SetInvalidateDsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetInvalidateDs {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetInvalidateDsVal) {
        self.value.write_bits(val.bits())
    }
}

/// QUERY_ERROR Register
///
/// Error register.
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **QUERY_CHECK** `[0]` (RO)
///
///   Hmac configuration state. 0: key are agree with purpose. 1: error
#[repr(transparent)]
pub struct QueryError {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for QueryError {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl QueryError {
    // ======== Field Constants ========

    /// **QUERY_CHECK**
    ///
    /// Hmac configuration state. 0: key are agree with purpose. 1: error
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const QUERY_CHECK_OFFSET: u32 = 0;
    pub const QUERY_CHECK_MASK: u32 = 0x00000001;
    pub const QUERY_CHECK: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **QUERY_CHECK** `[0]` (RO)
    ///
    /// Hmac configuration state. 0: key are agree with purpose. 1: error
    #[inline(always)]
    pub fn query_check(&self) -> bool {
        (self.value.read_bits() & Self::QUERY_CHECK_MASK) != 0
    }
}

/// QUERY_ERROR value type for batch read/modify/write operations
///
/// Use `queryerror.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `queryerror.write_val(val)`.
#[derive(Clone, Copy)]
pub struct QueryErrorVal {
    bits: u32,
}

impl QueryErrorVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **QUERY_CHECK** `[0:0]`
    ///
    /// Hmac configuration state. 0: key are agree with purpose. 1: error
    #[inline(always)]
    pub const fn query_check(&self) -> bool {
        (self.bits & QueryError::QUERY_CHECK_MASK) != 0
    }
}

impl Default for QueryErrorVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl QueryError {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> QueryErrorVal {
        QueryErrorVal::from_bits(self.value.read_bits())
    }
}

/// QUERY_BUSY Register
///
/// Busy register.
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **BUSY_STATE** `[0]` (RO)
///
///   Hmac state. 1'b0: idle. 1'b1: busy
#[repr(transparent)]
pub struct QueryBusy {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for QueryBusy {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl QueryBusy {
    // ======== Field Constants ========

    /// **BUSY_STATE**
    ///
    /// Hmac state. 1'b0: idle. 1'b1: busy
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const BUSY_STATE_OFFSET: u32 = 0;
    pub const BUSY_STATE_MASK: u32 = 0x00000001;
    pub const BUSY_STATE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **BUSY_STATE** `[0]` (RO)
    ///
    /// Hmac state. 1'b0: idle. 1'b1: busy
    #[inline(always)]
    pub fn busy_state(&self) -> bool {
        (self.value.read_bits() & Self::BUSY_STATE_MASK) != 0
    }
}

/// QUERY_BUSY value type for batch read/modify/write operations
///
/// Use `querybusy.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `querybusy.write_val(val)`.
#[derive(Clone, Copy)]
pub struct QueryBusyVal {
    bits: u32,
}

impl QueryBusyVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BUSY_STATE** `[0:0]`
    ///
    /// Hmac state. 1'b0: idle. 1'b1: busy
    #[inline(always)]
    pub const fn busy_state(&self) -> bool {
        (self.bits & QueryBusy::BUSY_STATE_MASK) != 0
    }
}

impl Default for QueryBusyVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl QueryBusy {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> QueryBusyVal {
        QueryBusyVal::from_bits(self.value.read_bits())
    }
}

/// WR_MESSAGE_MEM[0] Register
///
/// Message block memory.
///
/// Address offset: 0x0080
#[repr(transparent)]
pub struct WrMessageMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem0 {}

/// WR_MESSAGE_MEM[1] Register
///
/// Message block memory.
///
/// Address offset: 0x0084
#[repr(transparent)]
pub struct WrMessageMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem1 {}

/// WR_MESSAGE_MEM[2] Register
///
/// Message block memory.
///
/// Address offset: 0x0088
#[repr(transparent)]
pub struct WrMessageMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem2 {}

/// WR_MESSAGE_MEM[3] Register
///
/// Message block memory.
///
/// Address offset: 0x008c
#[repr(transparent)]
pub struct WrMessageMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem3 {}

/// WR_MESSAGE_MEM[4] Register
///
/// Message block memory.
///
/// Address offset: 0x0090
#[repr(transparent)]
pub struct WrMessageMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem4 {}

/// WR_MESSAGE_MEM[5] Register
///
/// Message block memory.
///
/// Address offset: 0x0094
#[repr(transparent)]
pub struct WrMessageMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem5 {}

/// WR_MESSAGE_MEM[6] Register
///
/// Message block memory.
///
/// Address offset: 0x0098
#[repr(transparent)]
pub struct WrMessageMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem6 {}

/// WR_MESSAGE_MEM[7] Register
///
/// Message block memory.
///
/// Address offset: 0x009c
#[repr(transparent)]
pub struct WrMessageMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem7 {}

/// WR_MESSAGE_MEM[8] Register
///
/// Message block memory.
///
/// Address offset: 0x00a0
#[repr(transparent)]
pub struct WrMessageMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem8 {}

/// WR_MESSAGE_MEM[9] Register
///
/// Message block memory.
///
/// Address offset: 0x00a4
#[repr(transparent)]
pub struct WrMessageMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem9 {}

/// WR_MESSAGE_MEM[10] Register
///
/// Message block memory.
///
/// Address offset: 0x00a8
#[repr(transparent)]
pub struct WrMessageMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem10 {}

/// WR_MESSAGE_MEM[11] Register
///
/// Message block memory.
///
/// Address offset: 0x00ac
#[repr(transparent)]
pub struct WrMessageMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem11 {}

/// WR_MESSAGE_MEM[12] Register
///
/// Message block memory.
///
/// Address offset: 0x00b0
#[repr(transparent)]
pub struct WrMessageMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem12 {}

/// WR_MESSAGE_MEM[13] Register
///
/// Message block memory.
///
/// Address offset: 0x00b4
#[repr(transparent)]
pub struct WrMessageMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem13 {}

/// WR_MESSAGE_MEM[14] Register
///
/// Message block memory.
///
/// Address offset: 0x00b8
#[repr(transparent)]
pub struct WrMessageMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem14 {}

/// WR_MESSAGE_MEM[15] Register
///
/// Message block memory.
///
/// Address offset: 0x00bc
#[repr(transparent)]
pub struct WrMessageMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrMessageMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrMessageMem15 {}

/// RD_RESULT_MEM[0] Register
///
/// Result from upstream.
///
/// Address offset: 0x00c0
#[repr(transparent)]
pub struct RdResultMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdResultMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdResultMem0 {}

/// RD_RESULT_MEM[1] Register
///
/// Result from upstream.
///
/// Address offset: 0x00c4
#[repr(transparent)]
pub struct RdResultMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdResultMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdResultMem1 {}

/// RD_RESULT_MEM[2] Register
///
/// Result from upstream.
///
/// Address offset: 0x00c8
#[repr(transparent)]
pub struct RdResultMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdResultMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdResultMem2 {}

/// RD_RESULT_MEM[3] Register
///
/// Result from upstream.
///
/// Address offset: 0x00cc
#[repr(transparent)]
pub struct RdResultMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdResultMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdResultMem3 {}

/// RD_RESULT_MEM[4] Register
///
/// Result from upstream.
///
/// Address offset: 0x00d0
#[repr(transparent)]
pub struct RdResultMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdResultMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdResultMem4 {}

/// RD_RESULT_MEM[5] Register
///
/// Result from upstream.
///
/// Address offset: 0x00d4
#[repr(transparent)]
pub struct RdResultMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdResultMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdResultMem5 {}

/// RD_RESULT_MEM[6] Register
///
/// Result from upstream.
///
/// Address offset: 0x00d8
#[repr(transparent)]
pub struct RdResultMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdResultMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdResultMem6 {}

/// RD_RESULT_MEM[7] Register
///
/// Result from upstream.
///
/// Address offset: 0x00dc
#[repr(transparent)]
pub struct RdResultMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdResultMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdResultMem7 {}

/// SET_MESSAGE_PAD Register
///
/// Process control register 5.
///
/// Address offset: 0x00f0
///
/// ## Fields
///
/// - **SET_TEXT_PAD** `[0]` (WO)
///
///   Start software padding.
#[repr(transparent)]
pub struct SetMessagePad {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SetMessagePad {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SetMessagePad {
    // ======== Field Constants ========

    /// **SET_TEXT_PAD**
    ///
    /// Start software padding.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_TEXT_PAD_OFFSET: u32 = 0;
    pub const SET_TEXT_PAD_MASK: u32 = 0x00000001;
    pub const SET_TEXT_PAD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_TEXT_PAD** (WO)
    ///
    /// Start software padding.
    #[inline(always)]
    pub fn trigger_set_text_pad(&self) {
        self.value.write_bits(Self::SET_TEXT_PAD_MASK);
    }
}

/// SET_MESSAGE_PAD value type for batch read/modify/write operations
///
/// Use `setmessagepad.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `setmessagepad.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SetMessagePadVal {
    bits: u32,
}

impl SetMessagePadVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_TEXT_PAD** `[0:0]`
    ///
    /// Start software padding.
    #[inline(always)]
    pub const fn with_set_text_pad(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SetMessagePad::SET_TEXT_PAD_MASK
            } else {
                self.bits & !SetMessagePad::SET_TEXT_PAD_MASK
            },
        }
    }
}

impl Default for SetMessagePadVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SetMessagePad {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SetMessagePadVal) {
        self.value.write_bits(val.bits())
    }
}

/// ONE_BLOCK Register
///
/// Process control register 6.
///
/// Address offset: 0x00f4
///
/// ## Fields
///
/// - **SET_ONE_BLOCK** `[0]` (WO)
///
///   Don't have to do padding.
#[repr(transparent)]
pub struct OneBlock {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OneBlock {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OneBlock {
    // ======== Field Constants ========

    /// **SET_ONE_BLOCK**
    ///
    /// Don't have to do padding.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SET_ONE_BLOCK_OFFSET: u32 = 0;
    pub const SET_ONE_BLOCK_MASK: u32 = 0x00000001;
    pub const SET_ONE_BLOCK: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SET_ONE_BLOCK** (WO)
    ///
    /// Don't have to do padding.
    #[inline(always)]
    pub fn trigger_set_one_block(&self) {
        self.value.write_bits(Self::SET_ONE_BLOCK_MASK);
    }
}

/// ONE_BLOCK value type for batch read/modify/write operations
///
/// Use `oneblock.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `oneblock.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OneBlockVal {
    bits: u32,
}

impl OneBlockVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SET_ONE_BLOCK** `[0:0]`
    ///
    /// Don't have to do padding.
    #[inline(always)]
    pub const fn with_set_one_block(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OneBlock::SET_ONE_BLOCK_MASK
            } else {
                self.bits & !OneBlock::SET_ONE_BLOCK_MASK
            },
        }
    }
}

impl Default for OneBlockVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl OneBlock {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OneBlockVal) {
        self.value.write_bits(val.bits())
    }
}

/// SOFT_JTAG_CTRL Register
///
/// Jtag register 0.
///
/// Address offset: 0x00f8
///
/// ## Fields
///
/// - **SOFT_JTAG_CTRL** `[0]` (WO)
///
///   Turn on JTAG verification.
#[repr(transparent)]
pub struct SoftJtagCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SoftJtagCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SoftJtagCtrl {
    // ======== Field Constants ========

    /// **SOFT_JTAG_CTRL**
    ///
    /// Turn on JTAG verification.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SOFT_JTAG_CTRL_OFFSET: u32 = 0;
    pub const SOFT_JTAG_CTRL_MASK: u32 = 0x00000001;
    pub const SOFT_JTAG_CTRL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SOFT_JTAG_CTRL** (WO)
    ///
    /// Turn on JTAG verification.
    #[inline(always)]
    pub fn trigger_soft_jtag_ctrl(&self) {
        self.value.write_bits(Self::SOFT_JTAG_CTRL_MASK);
    }
}

/// SOFT_JTAG_CTRL value type for batch read/modify/write operations
///
/// Use `softjtagctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `softjtagctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SoftJtagCtrlVal {
    bits: u32,
}

impl SoftJtagCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **SOFT_JTAG_CTRL** `[0:0]`
    ///
    /// Turn on JTAG verification.
    #[inline(always)]
    pub const fn with_soft_jtag_ctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SoftJtagCtrl::SOFT_JTAG_CTRL_MASK
            } else {
                self.bits & !SoftJtagCtrl::SOFT_JTAG_CTRL_MASK
            },
        }
    }
}

impl Default for SoftJtagCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl SoftJtagCtrl {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SoftJtagCtrlVal) {
        self.value.write_bits(val.bits())
    }
}

/// WR_JTAG Register
///
/// Jtag register 1.
///
/// Address offset: 0x00fc
///
/// ## Fields
///
/// - **WR_JTAG** `[31:0]` (WO)
///
///   32-bit of key to be compared.
#[repr(transparent)]
pub struct WrJtag {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WrJtag {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WrJtag {
    // ======== Field Constants ========

    /// **WR_JTAG**
    ///
    /// 32-bit of key to be compared.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const WR_JTAG_OFFSET: u32 = 0;
    pub const WR_JTAG_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// Write **WR_JTAG** `[31:0]` (WO)
    ///
    /// 32-bit of key to be compared.
    #[inline(always)]
    pub fn write_wr_jtag(&self, value: u32) {
        self.value
            .write_bits((value << Self::WR_JTAG_OFFSET) & Self::WR_JTAG_MASK);
    }
}

/// WR_JTAG value type for batch read/modify/write operations
///
/// Use `wrjtag.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wrjtag.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WrJtagVal {
    bits: u32,
}

impl WrJtagVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **WR_JTAG** `[31:0]`
    ///
    /// 32-bit of key to be compared.
    #[inline(always)]
    pub const fn with_wr_jtag(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !WrJtag::WR_JTAG_MASK)
                | ((value << WrJtag::WR_JTAG_OFFSET) & WrJtag::WR_JTAG_MASK),
        }
    }
}

impl Default for WrJtagVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl WrJtag {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WrJtagVal) {
        self.value.write_bits(val.bits())
    }
}

/// DATE Register
///
/// Date register.
///
/// Address offset: 0x01fc
/// Reset value: 0x20200618
///
/// ## Fields
///
/// - **DATE** `[29:0]` (RW)
///
///   Hmac date information/ hmac version information.
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
    /// Hmac date information/ hmac version information.
    ///
    /// Bits `[29:0]`, mask `0x3fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x3fffffff;

    // ======== Field Accessors ========

    /// **DATE** `[29:0]` (RW)
    ///
    /// Hmac date information/ hmac version information.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[29:0]` (RW)
    ///
    /// Hmac date information/ hmac version information.
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
    /// Hmac date information/ hmac version information.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[29:0]`
    ///
    /// Hmac date information/ hmac version information.
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
    _reserved0: [u8; 64],
    /// 0x0040
    /// Process control register 0.
    pub set_start: SetStart,
    /// 0x0044
    /// Configure purpose.
    pub set_para_purpose: SetParaPurpose,
    /// 0x0048
    /// Configure key.
    pub set_para_key: SetParaKey,
    /// 0x004c
    /// Finish initial configuration.
    pub set_para_finish: SetParaFinish,
    /// 0x0050
    /// Process control register 1.
    pub set_message_one: SetMessageOne,
    /// 0x0054
    /// Process control register 2.
    pub set_message_ing: SetMessageIng,
    /// 0x0058
    /// Process control register 3.
    pub set_message_end: SetMessageEnd,
    /// 0x005c
    /// Process control register 4.
    pub set_result_finish: SetResultFinish,
    /// 0x0060
    /// Invalidate register 0.
    pub set_invalidate_jtag: SetInvalidateJtag,
    /// 0x0064
    /// Invalidate register 1.
    pub set_invalidate_ds: SetInvalidateDs,
    /// 0x0068
    /// Error register.
    pub query_error: QueryError,
    /// 0x006c
    /// Busy register.
    pub query_busy: QueryBusy,
    _reserved1: [u8; 16],
    /// 0x0080
    /// Message block memory.
    pub wr_message_mem0: WrMessageMem0,
    /// 0x0084
    /// Message block memory.
    pub wr_message_mem1: WrMessageMem1,
    /// 0x0088
    /// Message block memory.
    pub wr_message_mem2: WrMessageMem2,
    /// 0x008c
    /// Message block memory.
    pub wr_message_mem3: WrMessageMem3,
    /// 0x0090
    /// Message block memory.
    pub wr_message_mem4: WrMessageMem4,
    /// 0x0094
    /// Message block memory.
    pub wr_message_mem5: WrMessageMem5,
    /// 0x0098
    /// Message block memory.
    pub wr_message_mem6: WrMessageMem6,
    /// 0x009c
    /// Message block memory.
    pub wr_message_mem7: WrMessageMem7,
    /// 0x00a0
    /// Message block memory.
    pub wr_message_mem8: WrMessageMem8,
    /// 0x00a4
    /// Message block memory.
    pub wr_message_mem9: WrMessageMem9,
    /// 0x00a8
    /// Message block memory.
    pub wr_message_mem10: WrMessageMem10,
    /// 0x00ac
    /// Message block memory.
    pub wr_message_mem11: WrMessageMem11,
    /// 0x00b0
    /// Message block memory.
    pub wr_message_mem12: WrMessageMem12,
    /// 0x00b4
    /// Message block memory.
    pub wr_message_mem13: WrMessageMem13,
    /// 0x00b8
    /// Message block memory.
    pub wr_message_mem14: WrMessageMem14,
    /// 0x00bc
    /// Message block memory.
    pub wr_message_mem15: WrMessageMem15,
    /// 0x00c0
    /// Result from upstream.
    pub rd_result_mem0: RdResultMem0,
    /// 0x00c4
    /// Result from upstream.
    pub rd_result_mem1: RdResultMem1,
    /// 0x00c8
    /// Result from upstream.
    pub rd_result_mem2: RdResultMem2,
    /// 0x00cc
    /// Result from upstream.
    pub rd_result_mem3: RdResultMem3,
    /// 0x00d0
    /// Result from upstream.
    pub rd_result_mem4: RdResultMem4,
    /// 0x00d4
    /// Result from upstream.
    pub rd_result_mem5: RdResultMem5,
    /// 0x00d8
    /// Result from upstream.
    pub rd_result_mem6: RdResultMem6,
    /// 0x00dc
    /// Result from upstream.
    pub rd_result_mem7: RdResultMem7,
    _reserved2: [u8; 16],
    /// 0x00f0
    /// Process control register 5.
    pub set_message_pad: SetMessagePad,
    /// 0x00f4
    /// Process control register 6.
    pub one_block: OneBlock,
    /// 0x00f8
    /// Jtag register 0.
    pub soft_jtag_ctrl: SoftJtagCtrl,
    /// 0x00fc
    /// Jtag register 1.
    pub wr_jtag: WrJtag,
    _reserved3: [u8; 252],
    /// 0x01fc
    /// Date register.
    pub date: Date,
}
