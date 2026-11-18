//! LP_TOUCH
//! LP_TOUCH Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x50128000;

/// INT_RAW Register
///
/// need_des
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **APPROACH_LOOP_DONE_INT_RAW** `[5]` (RW)
///
///   need_des
///
/// - **TIMEOUT_INT_RAW** `[4]` (RW)
///
///   need_des
///
/// - **INACTIVE_INT_RAW** `[3]` (RW)
///
///   need_des
///
/// - **ACTIVE_INT_RAW** `[2]` (RW)
///
///   need_des
///
/// - **DONE_INT_RAW** `[1]` (RW)
///
///   need_des
///
/// - **SCAN_DONE_INT_RAW** `[0]` (RW)
///
///   need_des
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

    /// **APPROACH_LOOP_DONE_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const APPROACH_LOOP_DONE_INT_RAW_OFFSET: u32 = 5;
    pub const APPROACH_LOOP_DONE_INT_RAW_MASK: u32 = 0x00000020;
    pub const APPROACH_LOOP_DONE_INT_RAW: u32 = 1 << 5;

    /// **TIMEOUT_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TIMEOUT_INT_RAW_OFFSET: u32 = 4;
    pub const TIMEOUT_INT_RAW_MASK: u32 = 0x00000010;
    pub const TIMEOUT_INT_RAW: u32 = 1 << 4;

    /// **INACTIVE_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const INACTIVE_INT_RAW_OFFSET: u32 = 3;
    pub const INACTIVE_INT_RAW_MASK: u32 = 0x00000008;
    pub const INACTIVE_INT_RAW: u32 = 1 << 3;

    /// **ACTIVE_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ACTIVE_INT_RAW_OFFSET: u32 = 2;
    pub const ACTIVE_INT_RAW_MASK: u32 = 0x00000004;
    pub const ACTIVE_INT_RAW: u32 = 1 << 2;

    /// **DONE_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DONE_INT_RAW_OFFSET: u32 = 1;
    pub const DONE_INT_RAW_MASK: u32 = 0x00000002;
    pub const DONE_INT_RAW: u32 = 1 << 1;

    /// **SCAN_DONE_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SCAN_DONE_INT_RAW_OFFSET: u32 = 0;
    pub const SCAN_DONE_INT_RAW_MASK: u32 = 0x00000001;
    pub const SCAN_DONE_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **APPROACH_LOOP_DONE_INT_RAW** `[5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn approach_loop_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::APPROACH_LOOP_DONE_INT_RAW_MASK) != 0
    }

    /// Set APPROACH_LOOP_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_approach_loop_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::APPROACH_LOOP_DONE_INT_RAW_MASK);
    }

    /// Clear APPROACH_LOOP_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_approach_loop_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::APPROACH_LOOP_DONE_INT_RAW_MASK);
    }

    /// **TIMEOUT_INT_RAW** `[4]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn timeout_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Set TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timeout_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::TIMEOUT_INT_RAW_MASK);
    }

    /// Clear TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timeout_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::TIMEOUT_INT_RAW_MASK);
    }

    /// **INACTIVE_INT_RAW** `[3]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn inactive_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::INACTIVE_INT_RAW_MASK) != 0
    }

    /// Set INACTIVE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_inactive_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::INACTIVE_INT_RAW_MASK);
    }

    /// Clear INACTIVE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_inactive_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::INACTIVE_INT_RAW_MASK);
    }

    /// **ACTIVE_INT_RAW** `[2]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn active_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::ACTIVE_INT_RAW_MASK) != 0
    }

    /// Set ACTIVE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_active_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::ACTIVE_INT_RAW_MASK);
    }

    /// Clear ACTIVE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_active_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::ACTIVE_INT_RAW_MASK);
    }

    /// **DONE_INT_RAW** `[1]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::DONE_INT_RAW_MASK) != 0
    }

    /// Set DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_done_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::DONE_INT_RAW_MASK);
    }

    /// Clear DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_done_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::DONE_INT_RAW_MASK);
    }

    /// **SCAN_DONE_INT_RAW** `[0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn scan_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SCAN_DONE_INT_RAW_MASK) != 0
    }

    /// Set SCAN_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scan_done_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::SCAN_DONE_INT_RAW_MASK);
    }

    /// Clear SCAN_DONE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scan_done_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SCAN_DONE_INT_RAW_MASK);
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

    /// Read **APPROACH_LOOP_DONE_INT_RAW** `[5:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn approach_loop_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::APPROACH_LOOP_DONE_INT_RAW_MASK) != 0
    }

    /// Read **TIMEOUT_INT_RAW** `[4:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn timeout_int_raw(&self) -> bool {
        (self.bits & IntRaw::TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Read **INACTIVE_INT_RAW** `[3:3]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn inactive_int_raw(&self) -> bool {
        (self.bits & IntRaw::INACTIVE_INT_RAW_MASK) != 0
    }

    /// Read **ACTIVE_INT_RAW** `[2:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn active_int_raw(&self) -> bool {
        (self.bits & IntRaw::ACTIVE_INT_RAW_MASK) != 0
    }

    /// Read **DONE_INT_RAW** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn done_int_raw(&self) -> bool {
        (self.bits & IntRaw::DONE_INT_RAW_MASK) != 0
    }

    /// Read **SCAN_DONE_INT_RAW** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn scan_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::SCAN_DONE_INT_RAW_MASK) != 0
    }

    /// Set **APPROACH_LOOP_DONE_INT_RAW** `[5:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_approach_loop_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::APPROACH_LOOP_DONE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::APPROACH_LOOP_DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **TIMEOUT_INT_RAW** `[4:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_timeout_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TIMEOUT_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TIMEOUT_INT_RAW_MASK
            },
        }
    }

    /// Set **INACTIVE_INT_RAW** `[3:3]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_inactive_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::INACTIVE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::INACTIVE_INT_RAW_MASK
            },
        }
    }

    /// Set **ACTIVE_INT_RAW** `[2:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_active_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::ACTIVE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::ACTIVE_INT_RAW_MASK
            },
        }
    }

    /// Set **DONE_INT_RAW** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::DONE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::DONE_INT_RAW_MASK
            },
        }
    }

    /// Set **SCAN_DONE_INT_RAW** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_scan_done_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SCAN_DONE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SCAN_DONE_INT_RAW_MASK
            },
        }
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

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntRawVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IntRawVal) -> IntRawVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ST Register
///
/// need_des
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **APPROACH_LOOP_DONE_INT_ST** `[5]` (RO)
///
///   need_des
///
/// - **TIMEOUT_INT_ST** `[4]` (RO)
///
///   need_des
///
/// - **INACTIVE_INT_ST** `[3]` (RO)
///
///   need_des
///
/// - **ACTIVE_INT_ST** `[2]` (RO)
///
///   need_des
///
/// - **DONE_INT_ST** `[1]` (RO)
///
///   need_des
///
/// - **SCAN_DONE_INT_ST** `[0]` (RO)
///
///   need_des
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

    /// **APPROACH_LOOP_DONE_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const APPROACH_LOOP_DONE_INT_ST_OFFSET: u32 = 5;
    pub const APPROACH_LOOP_DONE_INT_ST_MASK: u32 = 0x00000020;
    pub const APPROACH_LOOP_DONE_INT_ST: u32 = 1 << 5;

    /// **TIMEOUT_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TIMEOUT_INT_ST_OFFSET: u32 = 4;
    pub const TIMEOUT_INT_ST_MASK: u32 = 0x00000010;
    pub const TIMEOUT_INT_ST: u32 = 1 << 4;

    /// **INACTIVE_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const INACTIVE_INT_ST_OFFSET: u32 = 3;
    pub const INACTIVE_INT_ST_MASK: u32 = 0x00000008;
    pub const INACTIVE_INT_ST: u32 = 1 << 3;

    /// **ACTIVE_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ACTIVE_INT_ST_OFFSET: u32 = 2;
    pub const ACTIVE_INT_ST_MASK: u32 = 0x00000004;
    pub const ACTIVE_INT_ST: u32 = 1 << 2;

    /// **DONE_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DONE_INT_ST_OFFSET: u32 = 1;
    pub const DONE_INT_ST_MASK: u32 = 0x00000002;
    pub const DONE_INT_ST: u32 = 1 << 1;

    /// **SCAN_DONE_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SCAN_DONE_INT_ST_OFFSET: u32 = 0;
    pub const SCAN_DONE_INT_ST_MASK: u32 = 0x00000001;
    pub const SCAN_DONE_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **APPROACH_LOOP_DONE_INT_ST** `[5]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn approach_loop_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::APPROACH_LOOP_DONE_INT_ST_MASK) != 0
    }

    /// **TIMEOUT_INT_ST** `[4]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn timeout_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TIMEOUT_INT_ST_MASK) != 0
    }

    /// **INACTIVE_INT_ST** `[3]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn inactive_int_st(&self) -> bool {
        (self.value.read_bits() & Self::INACTIVE_INT_ST_MASK) != 0
    }

    /// **ACTIVE_INT_ST** `[2]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn active_int_st(&self) -> bool {
        (self.value.read_bits() & Self::ACTIVE_INT_ST_MASK) != 0
    }

    /// **DONE_INT_ST** `[1]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::DONE_INT_ST_MASK) != 0
    }

    /// **SCAN_DONE_INT_ST** `[0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn scan_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SCAN_DONE_INT_ST_MASK) != 0
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

    /// Read **APPROACH_LOOP_DONE_INT_ST** `[5:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn approach_loop_done_int_st(&self) -> bool {
        (self.bits & IntSt::APPROACH_LOOP_DONE_INT_ST_MASK) != 0
    }

    /// Read **TIMEOUT_INT_ST** `[4:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn timeout_int_st(&self) -> bool {
        (self.bits & IntSt::TIMEOUT_INT_ST_MASK) != 0
    }

    /// Read **INACTIVE_INT_ST** `[3:3]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn inactive_int_st(&self) -> bool {
        (self.bits & IntSt::INACTIVE_INT_ST_MASK) != 0
    }

    /// Read **ACTIVE_INT_ST** `[2:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn active_int_st(&self) -> bool {
        (self.bits & IntSt::ACTIVE_INT_ST_MASK) != 0
    }

    /// Read **DONE_INT_ST** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn done_int_st(&self) -> bool {
        (self.bits & IntSt::DONE_INT_ST_MASK) != 0
    }

    /// Read **SCAN_DONE_INT_ST** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn scan_done_int_st(&self) -> bool {
        (self.bits & IntSt::SCAN_DONE_INT_ST_MASK) != 0
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
/// need_des
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **APPROACH_LOOP_DONE_INT_ENA** `[5]` (RW)
///
///   need_des
///
/// - **TIMEOUT_INT_ENA** `[4]` (RW)
///
///   need_des
///
/// - **INACTIVE_INT_ENA** `[3]` (RW)
///
///   need_des
///
/// - **ACTIVE_INT_ENA** `[2]` (RW)
///
///   need_des
///
/// - **DONE_INT_ENA** `[1]` (RW)
///
///   need_des
///
/// - **SCAN_DONE_INT_ENA** `[0]` (RW)
///
///   need_des
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

    /// **APPROACH_LOOP_DONE_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const APPROACH_LOOP_DONE_INT_ENA_OFFSET: u32 = 5;
    pub const APPROACH_LOOP_DONE_INT_ENA_MASK: u32 = 0x00000020;
    pub const APPROACH_LOOP_DONE_INT_ENA: u32 = 1 << 5;

    /// **TIMEOUT_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TIMEOUT_INT_ENA_OFFSET: u32 = 4;
    pub const TIMEOUT_INT_ENA_MASK: u32 = 0x00000010;
    pub const TIMEOUT_INT_ENA: u32 = 1 << 4;

    /// **INACTIVE_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const INACTIVE_INT_ENA_OFFSET: u32 = 3;
    pub const INACTIVE_INT_ENA_MASK: u32 = 0x00000008;
    pub const INACTIVE_INT_ENA: u32 = 1 << 3;

    /// **ACTIVE_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ACTIVE_INT_ENA_OFFSET: u32 = 2;
    pub const ACTIVE_INT_ENA_MASK: u32 = 0x00000004;
    pub const ACTIVE_INT_ENA: u32 = 1 << 2;

    /// **DONE_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DONE_INT_ENA_OFFSET: u32 = 1;
    pub const DONE_INT_ENA_MASK: u32 = 0x00000002;
    pub const DONE_INT_ENA: u32 = 1 << 1;

    /// **SCAN_DONE_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SCAN_DONE_INT_ENA_OFFSET: u32 = 0;
    pub const SCAN_DONE_INT_ENA_MASK: u32 = 0x00000001;
    pub const SCAN_DONE_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **APPROACH_LOOP_DONE_INT_ENA** `[5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn approach_loop_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::APPROACH_LOOP_DONE_INT_ENA_MASK) != 0
    }

    /// Set APPROACH_LOOP_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_approach_loop_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::APPROACH_LOOP_DONE_INT_ENA_MASK);
    }

    /// Clear APPROACH_LOOP_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_approach_loop_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::APPROACH_LOOP_DONE_INT_ENA_MASK);
    }

    /// **TIMEOUT_INT_ENA** `[4]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn timeout_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Set TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timeout_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::TIMEOUT_INT_ENA_MASK);
    }

    /// Clear TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timeout_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::TIMEOUT_INT_ENA_MASK);
    }

    /// **INACTIVE_INT_ENA** `[3]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn inactive_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::INACTIVE_INT_ENA_MASK) != 0
    }

    /// Set INACTIVE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_inactive_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::INACTIVE_INT_ENA_MASK);
    }

    /// Clear INACTIVE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_inactive_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::INACTIVE_INT_ENA_MASK);
    }

    /// **ACTIVE_INT_ENA** `[2]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn active_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::ACTIVE_INT_ENA_MASK) != 0
    }

    /// Set ACTIVE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_active_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::ACTIVE_INT_ENA_MASK);
    }

    /// Clear ACTIVE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_active_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::ACTIVE_INT_ENA_MASK);
    }

    /// **DONE_INT_ENA** `[1]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::DONE_INT_ENA_MASK) != 0
    }

    /// Set DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::DONE_INT_ENA_MASK);
    }

    /// Clear DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_done_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::DONE_INT_ENA_MASK);
    }

    /// **SCAN_DONE_INT_ENA** `[0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn scan_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SCAN_DONE_INT_ENA_MASK) != 0
    }

    /// Set SCAN_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_scan_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::SCAN_DONE_INT_ENA_MASK);
    }

    /// Clear SCAN_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_scan_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SCAN_DONE_INT_ENA_MASK);
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

    /// Read **APPROACH_LOOP_DONE_INT_ENA** `[5:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn approach_loop_done_int_ena(&self) -> bool {
        (self.bits & IntEna::APPROACH_LOOP_DONE_INT_ENA_MASK) != 0
    }

    /// Read **TIMEOUT_INT_ENA** `[4:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn timeout_int_ena(&self) -> bool {
        (self.bits & IntEna::TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Read **INACTIVE_INT_ENA** `[3:3]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn inactive_int_ena(&self) -> bool {
        (self.bits & IntEna::INACTIVE_INT_ENA_MASK) != 0
    }

    /// Read **ACTIVE_INT_ENA** `[2:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn active_int_ena(&self) -> bool {
        (self.bits & IntEna::ACTIVE_INT_ENA_MASK) != 0
    }

    /// Read **DONE_INT_ENA** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn done_int_ena(&self) -> bool {
        (self.bits & IntEna::DONE_INT_ENA_MASK) != 0
    }

    /// Read **SCAN_DONE_INT_ENA** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn scan_done_int_ena(&self) -> bool {
        (self.bits & IntEna::SCAN_DONE_INT_ENA_MASK) != 0
    }

    /// Set **APPROACH_LOOP_DONE_INT_ENA** `[5:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_approach_loop_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::APPROACH_LOOP_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::APPROACH_LOOP_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **TIMEOUT_INT_ENA** `[4:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_timeout_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TIMEOUT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TIMEOUT_INT_ENA_MASK
            },
        }
    }

    /// Set **INACTIVE_INT_ENA** `[3:3]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_inactive_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::INACTIVE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::INACTIVE_INT_ENA_MASK
            },
        }
    }

    /// Set **ACTIVE_INT_ENA** `[2:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_active_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::ACTIVE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::ACTIVE_INT_ENA_MASK
            },
        }
    }

    /// Set **DONE_INT_ENA** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **SCAN_DONE_INT_ENA** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_scan_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SCAN_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SCAN_DONE_INT_ENA_MASK
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
/// need_des
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **APPROACH_LOOP_DONE_INT_CLR** `[5]` (WO)
///
///   need_des
///
/// - **TIMEOUT_INT_CLR** `[4]` (WO)
///
///   need_des
///
/// - **INACTIVE_INT_CLR** `[3]` (WO)
///
///   need_des
///
/// - **ACTIVE_INT_CLR** `[2]` (WO)
///
///   need_des
///
/// - **DONE_INT_CLR** `[1]` (WO)
///
///   need_des
///
/// - **SCAN_DONE_INT_CLR** `[0]` (WO)
///
///   need_des
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

    /// **APPROACH_LOOP_DONE_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const APPROACH_LOOP_DONE_INT_CLR_OFFSET: u32 = 5;
    pub const APPROACH_LOOP_DONE_INT_CLR_MASK: u32 = 0x00000020;
    pub const APPROACH_LOOP_DONE_INT_CLR: u32 = 1 << 5;

    /// **TIMEOUT_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TIMEOUT_INT_CLR_OFFSET: u32 = 4;
    pub const TIMEOUT_INT_CLR_MASK: u32 = 0x00000010;
    pub const TIMEOUT_INT_CLR: u32 = 1 << 4;

    /// **INACTIVE_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const INACTIVE_INT_CLR_OFFSET: u32 = 3;
    pub const INACTIVE_INT_CLR_MASK: u32 = 0x00000008;
    pub const INACTIVE_INT_CLR: u32 = 1 << 3;

    /// **ACTIVE_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ACTIVE_INT_CLR_OFFSET: u32 = 2;
    pub const ACTIVE_INT_CLR_MASK: u32 = 0x00000004;
    pub const ACTIVE_INT_CLR: u32 = 1 << 2;

    /// **DONE_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DONE_INT_CLR_OFFSET: u32 = 1;
    pub const DONE_INT_CLR_MASK: u32 = 0x00000002;
    pub const DONE_INT_CLR: u32 = 1 << 1;

    /// **SCAN_DONE_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SCAN_DONE_INT_CLR_OFFSET: u32 = 0;
    pub const SCAN_DONE_INT_CLR_MASK: u32 = 0x00000001;
    pub const SCAN_DONE_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **APPROACH_LOOP_DONE_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_approach_loop_done_int_clr(&self) {
        self.value.write_bits(Self::APPROACH_LOOP_DONE_INT_CLR_MASK);
    }

    /// Trigger **TIMEOUT_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_timeout_int_clr(&self) {
        self.value.write_bits(Self::TIMEOUT_INT_CLR_MASK);
    }

    /// Trigger **INACTIVE_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_inactive_int_clr(&self) {
        self.value.write_bits(Self::INACTIVE_INT_CLR_MASK);
    }

    /// Trigger **ACTIVE_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_active_int_clr(&self) {
        self.value.write_bits(Self::ACTIVE_INT_CLR_MASK);
    }

    /// Trigger **DONE_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_done_int_clr(&self) {
        self.value.write_bits(Self::DONE_INT_CLR_MASK);
    }

    /// Trigger **SCAN_DONE_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_scan_done_int_clr(&self) {
        self.value.write_bits(Self::SCAN_DONE_INT_CLR_MASK);
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

    /// Set **APPROACH_LOOP_DONE_INT_CLR** `[5:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_approach_loop_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::APPROACH_LOOP_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::APPROACH_LOOP_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **TIMEOUT_INT_CLR** `[4:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_timeout_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TIMEOUT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TIMEOUT_INT_CLR_MASK
            },
        }
    }

    /// Set **INACTIVE_INT_CLR** `[3:3]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_inactive_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::INACTIVE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::INACTIVE_INT_CLR_MASK
            },
        }
    }

    /// Set **ACTIVE_INT_CLR** `[2:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_active_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::ACTIVE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::ACTIVE_INT_CLR_MASK
            },
        }
    }

    /// Set **DONE_INT_CLR** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **SCAN_DONE_INT_CLR** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_scan_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SCAN_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SCAN_DONE_INT_CLR_MASK
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

/// CHN_STATUS Register
///
/// need_des
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **SCAN_CURR** `[19:16]` (RO)
///
///   need_des
///
/// - **MEAS_DONE** `[15]` (RO)
///
///   need_des
///
/// - **PAD_ACTIVE** `[14:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct ChnStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ChnStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ChnStatus {
    // ======== Field Constants ========

    /// **SCAN_CURR**
    ///
    /// need_des
    ///
    /// Bits `[19:16]`, mask `0x000f0000`
    pub const SCAN_CURR_OFFSET: u32 = 16;
    pub const SCAN_CURR_MASK: u32 = 0x000f0000;

    /// **MEAS_DONE**
    ///
    /// need_des
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const MEAS_DONE_OFFSET: u32 = 15;
    pub const MEAS_DONE_MASK: u32 = 0x00008000;
    pub const MEAS_DONE: u32 = 1 << 15;

    /// **PAD_ACTIVE**
    ///
    /// need_des
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const PAD_ACTIVE_OFFSET: u32 = 0;
    pub const PAD_ACTIVE_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **SCAN_CURR** `[19:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn scan_curr(&self) -> u32 {
        (self.value.read_bits() & Self::SCAN_CURR_MASK) >> Self::SCAN_CURR_OFFSET
    }

    /// **MEAS_DONE** `[15]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn meas_done(&self) -> bool {
        (self.value.read_bits() & Self::MEAS_DONE_MASK) != 0
    }

    /// **PAD_ACTIVE** `[14:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad_active(&self) -> u32 {
        (self.value.read_bits() & Self::PAD_ACTIVE_MASK) >> Self::PAD_ACTIVE_OFFSET
    }
}

/// CHN_STATUS value type for batch read/modify/write operations
///
/// Use `chnstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `chnstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ChnStatusVal {
    bits: u32,
}

impl ChnStatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SCAN_CURR** `[19:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn scan_curr(&self) -> u32 {
        (self.bits & ChnStatus::SCAN_CURR_MASK) >> ChnStatus::SCAN_CURR_OFFSET
    }

    /// Read **MEAS_DONE** `[15:15]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn meas_done(&self) -> bool {
        (self.bits & ChnStatus::MEAS_DONE_MASK) != 0
    }

    /// Read **PAD_ACTIVE** `[14:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad_active(&self) -> u32 {
        (self.bits & ChnStatus::PAD_ACTIVE_MASK) >> ChnStatus::PAD_ACTIVE_OFFSET
    }
}

impl Default for ChnStatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ChnStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ChnStatusVal {
        ChnStatusVal::from_bits(self.value.read_bits())
    }
}

/// STATUS_0 Register
///
/// need_des
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **PAD0_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD0_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD0_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status0 {
    // ======== Field Constants ========

    /// **PAD0_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD0_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD0_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD0_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD0_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD0_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD0_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD0_DATA_OFFSET: u32 = 0;
    pub const PAD0_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD0_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad0_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD0_NEG_NOISE_CNT_MASK) >> Self::PAD0_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD0_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad0_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD0_DEBOUNCE_CNT_MASK) >> Self::PAD0_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD0_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad0_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD0_DATA_MASK) >> Self::PAD0_DATA_OFFSET
    }
}

/// STATUS_0 value type for batch read/modify/write operations
///
/// Use `status0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status0Val {
    bits: u32,
}

impl Status0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD0_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad0_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status0::PAD0_NEG_NOISE_CNT_MASK) >> Status0::PAD0_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD0_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad0_debounce_cnt(&self) -> u32 {
        (self.bits & Status0::PAD0_DEBOUNCE_CNT_MASK) >> Status0::PAD0_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD0_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad0_data(&self) -> u32 {
        (self.bits & Status0::PAD0_DATA_MASK) >> Status0::PAD0_DATA_OFFSET
    }
}

impl Default for Status0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status0Val {
        Status0Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_1 Register
///
/// need_des
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **PAD1_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD1_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD1_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status1 {
    // ======== Field Constants ========

    /// **PAD1_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD1_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD1_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD1_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD1_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD1_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD1_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD1_DATA_OFFSET: u32 = 0;
    pub const PAD1_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD1_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad1_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD1_NEG_NOISE_CNT_MASK) >> Self::PAD1_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD1_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad1_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD1_DEBOUNCE_CNT_MASK) >> Self::PAD1_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD1_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad1_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD1_DATA_MASK) >> Self::PAD1_DATA_OFFSET
    }
}

/// STATUS_1 value type for batch read/modify/write operations
///
/// Use `status1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status1Val {
    bits: u32,
}

impl Status1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD1_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad1_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status1::PAD1_NEG_NOISE_CNT_MASK) >> Status1::PAD1_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD1_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad1_debounce_cnt(&self) -> u32 {
        (self.bits & Status1::PAD1_DEBOUNCE_CNT_MASK) >> Status1::PAD1_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD1_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad1_data(&self) -> u32 {
        (self.bits & Status1::PAD1_DATA_MASK) >> Status1::PAD1_DATA_OFFSET
    }
}

impl Default for Status1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status1Val {
        Status1Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_2 Register
///
/// need_des
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **PAD2_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD2_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD2_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status2 {
    // ======== Field Constants ========

    /// **PAD2_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD2_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD2_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD2_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD2_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD2_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD2_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD2_DATA_OFFSET: u32 = 0;
    pub const PAD2_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD2_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad2_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD2_NEG_NOISE_CNT_MASK) >> Self::PAD2_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD2_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad2_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD2_DEBOUNCE_CNT_MASK) >> Self::PAD2_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD2_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad2_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD2_DATA_MASK) >> Self::PAD2_DATA_OFFSET
    }
}

/// STATUS_2 value type for batch read/modify/write operations
///
/// Use `status2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status2Val {
    bits: u32,
}

impl Status2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD2_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad2_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status2::PAD2_NEG_NOISE_CNT_MASK) >> Status2::PAD2_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD2_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad2_debounce_cnt(&self) -> u32 {
        (self.bits & Status2::PAD2_DEBOUNCE_CNT_MASK) >> Status2::PAD2_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD2_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad2_data(&self) -> u32 {
        (self.bits & Status2::PAD2_DATA_MASK) >> Status2::PAD2_DATA_OFFSET
    }
}

impl Default for Status2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status2Val {
        Status2Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_3 Register
///
/// need_des
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **PAD3_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD3_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD3_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status3 {
    // ======== Field Constants ========

    /// **PAD3_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD3_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD3_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD3_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD3_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD3_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD3_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD3_DATA_OFFSET: u32 = 0;
    pub const PAD3_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD3_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad3_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD3_NEG_NOISE_CNT_MASK) >> Self::PAD3_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD3_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad3_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD3_DEBOUNCE_CNT_MASK) >> Self::PAD3_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD3_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad3_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD3_DATA_MASK) >> Self::PAD3_DATA_OFFSET
    }
}

/// STATUS_3 value type for batch read/modify/write operations
///
/// Use `status3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status3Val {
    bits: u32,
}

impl Status3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD3_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad3_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status3::PAD3_NEG_NOISE_CNT_MASK) >> Status3::PAD3_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD3_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad3_debounce_cnt(&self) -> u32 {
        (self.bits & Status3::PAD3_DEBOUNCE_CNT_MASK) >> Status3::PAD3_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD3_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad3_data(&self) -> u32 {
        (self.bits & Status3::PAD3_DATA_MASK) >> Status3::PAD3_DATA_OFFSET
    }
}

impl Default for Status3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status3Val {
        Status3Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_4 Register
///
/// need_des
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **PAD4_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD4_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD4_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status4 {
    // ======== Field Constants ========

    /// **PAD4_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD4_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD4_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD4_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD4_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD4_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD4_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD4_DATA_OFFSET: u32 = 0;
    pub const PAD4_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD4_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad4_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD4_NEG_NOISE_CNT_MASK) >> Self::PAD4_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD4_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad4_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD4_DEBOUNCE_CNT_MASK) >> Self::PAD4_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD4_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad4_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD4_DATA_MASK) >> Self::PAD4_DATA_OFFSET
    }
}

/// STATUS_4 value type for batch read/modify/write operations
///
/// Use `status4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status4Val {
    bits: u32,
}

impl Status4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD4_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad4_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status4::PAD4_NEG_NOISE_CNT_MASK) >> Status4::PAD4_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD4_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad4_debounce_cnt(&self) -> u32 {
        (self.bits & Status4::PAD4_DEBOUNCE_CNT_MASK) >> Status4::PAD4_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD4_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad4_data(&self) -> u32 {
        (self.bits & Status4::PAD4_DATA_MASK) >> Status4::PAD4_DATA_OFFSET
    }
}

impl Default for Status4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status4Val {
        Status4Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_5 Register
///
/// need_des
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **PAD5_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD5_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD5_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status5 {
    // ======== Field Constants ========

    /// **PAD5_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD5_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD5_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD5_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD5_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD5_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD5_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD5_DATA_OFFSET: u32 = 0;
    pub const PAD5_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD5_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad5_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD5_NEG_NOISE_CNT_MASK) >> Self::PAD5_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD5_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad5_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD5_DEBOUNCE_CNT_MASK) >> Self::PAD5_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD5_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad5_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD5_DATA_MASK) >> Self::PAD5_DATA_OFFSET
    }
}

/// STATUS_5 value type for batch read/modify/write operations
///
/// Use `status5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status5Val {
    bits: u32,
}

impl Status5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD5_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad5_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status5::PAD5_NEG_NOISE_CNT_MASK) >> Status5::PAD5_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD5_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad5_debounce_cnt(&self) -> u32 {
        (self.bits & Status5::PAD5_DEBOUNCE_CNT_MASK) >> Status5::PAD5_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD5_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad5_data(&self) -> u32 {
        (self.bits & Status5::PAD5_DATA_MASK) >> Status5::PAD5_DATA_OFFSET
    }
}

impl Default for Status5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status5Val {
        Status5Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_6 Register
///
/// need_des
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **PAD6_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD6_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD6_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status6 {
    // ======== Field Constants ========

    /// **PAD6_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD6_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD6_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD6_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD6_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD6_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD6_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD6_DATA_OFFSET: u32 = 0;
    pub const PAD6_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD6_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad6_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD6_NEG_NOISE_CNT_MASK) >> Self::PAD6_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD6_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad6_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD6_DEBOUNCE_CNT_MASK) >> Self::PAD6_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD6_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad6_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD6_DATA_MASK) >> Self::PAD6_DATA_OFFSET
    }
}

/// STATUS_6 value type for batch read/modify/write operations
///
/// Use `status6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status6Val {
    bits: u32,
}

impl Status6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD6_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad6_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status6::PAD6_NEG_NOISE_CNT_MASK) >> Status6::PAD6_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD6_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad6_debounce_cnt(&self) -> u32 {
        (self.bits & Status6::PAD6_DEBOUNCE_CNT_MASK) >> Status6::PAD6_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD6_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad6_data(&self) -> u32 {
        (self.bits & Status6::PAD6_DATA_MASK) >> Status6::PAD6_DATA_OFFSET
    }
}

impl Default for Status6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status6Val {
        Status6Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_7 Register
///
/// need_des
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **PAD7_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD7_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD7_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status7 {
    // ======== Field Constants ========

    /// **PAD7_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD7_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD7_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD7_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD7_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD7_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD7_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD7_DATA_OFFSET: u32 = 0;
    pub const PAD7_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD7_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad7_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD7_NEG_NOISE_CNT_MASK) >> Self::PAD7_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD7_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad7_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD7_DEBOUNCE_CNT_MASK) >> Self::PAD7_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD7_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad7_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD7_DATA_MASK) >> Self::PAD7_DATA_OFFSET
    }
}

/// STATUS_7 value type for batch read/modify/write operations
///
/// Use `status7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status7Val {
    bits: u32,
}

impl Status7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD7_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad7_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status7::PAD7_NEG_NOISE_CNT_MASK) >> Status7::PAD7_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD7_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad7_debounce_cnt(&self) -> u32 {
        (self.bits & Status7::PAD7_DEBOUNCE_CNT_MASK) >> Status7::PAD7_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD7_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad7_data(&self) -> u32 {
        (self.bits & Status7::PAD7_DATA_MASK) >> Status7::PAD7_DATA_OFFSET
    }
}

impl Default for Status7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status7Val {
        Status7Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_8 Register
///
/// need_des
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **PAD8_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD8_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD8_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status8 {
    // ======== Field Constants ========

    /// **PAD8_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD8_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD8_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD8_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD8_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD8_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD8_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD8_DATA_OFFSET: u32 = 0;
    pub const PAD8_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD8_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad8_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD8_NEG_NOISE_CNT_MASK) >> Self::PAD8_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD8_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad8_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD8_DEBOUNCE_CNT_MASK) >> Self::PAD8_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD8_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad8_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD8_DATA_MASK) >> Self::PAD8_DATA_OFFSET
    }
}

/// STATUS_8 value type for batch read/modify/write operations
///
/// Use `status8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status8Val {
    bits: u32,
}

impl Status8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD8_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad8_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status8::PAD8_NEG_NOISE_CNT_MASK) >> Status8::PAD8_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD8_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad8_debounce_cnt(&self) -> u32 {
        (self.bits & Status8::PAD8_DEBOUNCE_CNT_MASK) >> Status8::PAD8_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD8_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad8_data(&self) -> u32 {
        (self.bits & Status8::PAD8_DATA_MASK) >> Status8::PAD8_DATA_OFFSET
    }
}

impl Default for Status8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status8Val {
        Status8Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_9 Register
///
/// need_des
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **PAD9_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD9_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD9_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status9 {
    // ======== Field Constants ========

    /// **PAD9_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD9_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD9_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD9_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD9_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD9_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD9_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD9_DATA_OFFSET: u32 = 0;
    pub const PAD9_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD9_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad9_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD9_NEG_NOISE_CNT_MASK) >> Self::PAD9_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD9_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad9_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD9_DEBOUNCE_CNT_MASK) >> Self::PAD9_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD9_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad9_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD9_DATA_MASK) >> Self::PAD9_DATA_OFFSET
    }
}

/// STATUS_9 value type for batch read/modify/write operations
///
/// Use `status9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status9Val {
    bits: u32,
}

impl Status9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD9_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad9_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status9::PAD9_NEG_NOISE_CNT_MASK) >> Status9::PAD9_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD9_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad9_debounce_cnt(&self) -> u32 {
        (self.bits & Status9::PAD9_DEBOUNCE_CNT_MASK) >> Status9::PAD9_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD9_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad9_data(&self) -> u32 {
        (self.bits & Status9::PAD9_DATA_MASK) >> Status9::PAD9_DATA_OFFSET
    }
}

impl Default for Status9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status9Val {
        Status9Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_10 Register
///
/// need_des
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **PAD10_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD10_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD10_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status10 {
    // ======== Field Constants ========

    /// **PAD10_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD10_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD10_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD10_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD10_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD10_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD10_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD10_DATA_OFFSET: u32 = 0;
    pub const PAD10_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD10_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad10_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD10_NEG_NOISE_CNT_MASK)
            >> Self::PAD10_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD10_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad10_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD10_DEBOUNCE_CNT_MASK) >> Self::PAD10_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD10_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad10_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD10_DATA_MASK) >> Self::PAD10_DATA_OFFSET
    }
}

/// STATUS_10 value type for batch read/modify/write operations
///
/// Use `status10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status10Val {
    bits: u32,
}

impl Status10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD10_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad10_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status10::PAD10_NEG_NOISE_CNT_MASK) >> Status10::PAD10_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD10_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad10_debounce_cnt(&self) -> u32 {
        (self.bits & Status10::PAD10_DEBOUNCE_CNT_MASK) >> Status10::PAD10_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD10_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad10_data(&self) -> u32 {
        (self.bits & Status10::PAD10_DATA_MASK) >> Status10::PAD10_DATA_OFFSET
    }
}

impl Default for Status10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status10Val {
        Status10Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_11 Register
///
/// need_des
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **PAD11_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD11_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD11_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status11 {
    // ======== Field Constants ========

    /// **PAD11_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD11_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD11_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD11_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD11_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD11_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD11_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD11_DATA_OFFSET: u32 = 0;
    pub const PAD11_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD11_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad11_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD11_NEG_NOISE_CNT_MASK)
            >> Self::PAD11_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD11_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad11_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD11_DEBOUNCE_CNT_MASK) >> Self::PAD11_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD11_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad11_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD11_DATA_MASK) >> Self::PAD11_DATA_OFFSET
    }
}

/// STATUS_11 value type for batch read/modify/write operations
///
/// Use `status11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status11Val {
    bits: u32,
}

impl Status11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD11_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad11_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status11::PAD11_NEG_NOISE_CNT_MASK) >> Status11::PAD11_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD11_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad11_debounce_cnt(&self) -> u32 {
        (self.bits & Status11::PAD11_DEBOUNCE_CNT_MASK) >> Status11::PAD11_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD11_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad11_data(&self) -> u32 {
        (self.bits & Status11::PAD11_DATA_MASK) >> Status11::PAD11_DATA_OFFSET
    }
}

impl Default for Status11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status11Val {
        Status11Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_12 Register
///
/// need_des
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **PAD12_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD12_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD12_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status12 {
    // ======== Field Constants ========

    /// **PAD12_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD12_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD12_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD12_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD12_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD12_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD12_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD12_DATA_OFFSET: u32 = 0;
    pub const PAD12_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD12_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad12_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD12_NEG_NOISE_CNT_MASK)
            >> Self::PAD12_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD12_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad12_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD12_DEBOUNCE_CNT_MASK) >> Self::PAD12_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD12_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad12_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD12_DATA_MASK) >> Self::PAD12_DATA_OFFSET
    }
}

/// STATUS_12 value type for batch read/modify/write operations
///
/// Use `status12.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status12.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status12Val {
    bits: u32,
}

impl Status12Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD12_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad12_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status12::PAD12_NEG_NOISE_CNT_MASK) >> Status12::PAD12_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD12_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad12_debounce_cnt(&self) -> u32 {
        (self.bits & Status12::PAD12_DEBOUNCE_CNT_MASK) >> Status12::PAD12_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD12_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad12_data(&self) -> u32 {
        (self.bits & Status12::PAD12_DATA_MASK) >> Status12::PAD12_DATA_OFFSET
    }
}

impl Default for Status12Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status12 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status12Val {
        Status12Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_13 Register
///
/// need_des
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **PAD13_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD13_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD13_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status13 {
    // ======== Field Constants ========

    /// **PAD13_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD13_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD13_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD13_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD13_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD13_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD13_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD13_DATA_OFFSET: u32 = 0;
    pub const PAD13_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD13_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad13_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD13_NEG_NOISE_CNT_MASK)
            >> Self::PAD13_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD13_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad13_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD13_DEBOUNCE_CNT_MASK) >> Self::PAD13_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD13_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad13_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD13_DATA_MASK) >> Self::PAD13_DATA_OFFSET
    }
}

/// STATUS_13 value type for batch read/modify/write operations
///
/// Use `status13.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status13.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status13Val {
    bits: u32,
}

impl Status13Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD13_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad13_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status13::PAD13_NEG_NOISE_CNT_MASK) >> Status13::PAD13_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD13_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad13_debounce_cnt(&self) -> u32 {
        (self.bits & Status13::PAD13_DEBOUNCE_CNT_MASK) >> Status13::PAD13_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD13_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad13_data(&self) -> u32 {
        (self.bits & Status13::PAD13_DATA_MASK) >> Status13::PAD13_DATA_OFFSET
    }
}

impl Default for Status13Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status13 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status13Val {
        Status13Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_14 Register
///
/// need_des
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **PAD14_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **PAD14_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **PAD14_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status14 {
    // ======== Field Constants ========

    /// **PAD14_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const PAD14_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const PAD14_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **PAD14_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const PAD14_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const PAD14_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **PAD14_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PAD14_DATA_OFFSET: u32 = 0;
    pub const PAD14_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PAD14_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad14_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD14_NEG_NOISE_CNT_MASK)
            >> Self::PAD14_NEG_NOISE_CNT_OFFSET
    }

    /// **PAD14_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad14_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::PAD14_DEBOUNCE_CNT_MASK) >> Self::PAD14_DEBOUNCE_CNT_OFFSET
    }

    /// **PAD14_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad14_data(&self) -> u32 {
        (self.value.read_bits() & Self::PAD14_DATA_MASK) >> Self::PAD14_DATA_OFFSET
    }
}

/// STATUS_14 value type for batch read/modify/write operations
///
/// Use `status14.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status14.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status14Val {
    bits: u32,
}

impl Status14Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD14_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad14_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status14::PAD14_NEG_NOISE_CNT_MASK) >> Status14::PAD14_NEG_NOISE_CNT_OFFSET
    }

    /// Read **PAD14_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad14_debounce_cnt(&self) -> u32 {
        (self.bits & Status14::PAD14_DEBOUNCE_CNT_MASK) >> Status14::PAD14_DEBOUNCE_CNT_OFFSET
    }

    /// Read **PAD14_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad14_data(&self) -> u32 {
        (self.bits & Status14::PAD14_DATA_MASK) >> Status14::PAD14_DATA_OFFSET
    }
}

impl Default for Status14Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status14 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status14Val {
        Status14Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_15 Register
///
/// need_des
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **SLP_NEG_NOISE_CNT** `[22:19]` (RO)
///
///   need_des
///
/// - **SLP_DEBOUNCE_CNT** `[18:16]` (RO)
///
///   need_des
///
/// - **SLP_DATA** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status15 {
    // ======== Field Constants ========

    /// **SLP_NEG_NOISE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[22:19]`, mask `0x00780000`
    pub const SLP_NEG_NOISE_CNT_OFFSET: u32 = 19;
    pub const SLP_NEG_NOISE_CNT_MASK: u32 = 0x00780000;

    /// **SLP_DEBOUNCE_CNT**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const SLP_DEBOUNCE_CNT_OFFSET: u32 = 16;
    pub const SLP_DEBOUNCE_CNT_MASK: u32 = 0x00070000;

    /// **SLP_DATA**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SLP_DATA_OFFSET: u32 = 0;
    pub const SLP_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SLP_NEG_NOISE_CNT** `[22:19]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn slp_neg_noise_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::SLP_NEG_NOISE_CNT_MASK) >> Self::SLP_NEG_NOISE_CNT_OFFSET
    }

    /// **SLP_DEBOUNCE_CNT** `[18:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn slp_debounce_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::SLP_DEBOUNCE_CNT_MASK) >> Self::SLP_DEBOUNCE_CNT_OFFSET
    }

    /// **SLP_DATA** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn slp_data(&self) -> u32 {
        (self.value.read_bits() & Self::SLP_DATA_MASK) >> Self::SLP_DATA_OFFSET
    }
}

/// STATUS_15 value type for batch read/modify/write operations
///
/// Use `status15.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status15.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status15Val {
    bits: u32,
}

impl Status15Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SLP_NEG_NOISE_CNT** `[22:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn slp_neg_noise_cnt(&self) -> u32 {
        (self.bits & Status15::SLP_NEG_NOISE_CNT_MASK) >> Status15::SLP_NEG_NOISE_CNT_OFFSET
    }

    /// Read **SLP_DEBOUNCE_CNT** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn slp_debounce_cnt(&self) -> u32 {
        (self.bits & Status15::SLP_DEBOUNCE_CNT_MASK) >> Status15::SLP_DEBOUNCE_CNT_OFFSET
    }

    /// Read **SLP_DATA** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn slp_data(&self) -> u32 {
        (self.bits & Status15::SLP_DATA_MASK) >> Status15::SLP_DATA_OFFSET
    }
}

impl Default for Status15Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status15 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status15Val {
        Status15Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_16 Register
///
/// need_des
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **SLP_APPROACH_CNT** `[31:24]` (RO)
///
///   need_des
///
/// - **APPROACH_PAD0_CNT** `[23:16]` (RO)
///
///   need_des
///
/// - **APPROACH_PAD1_CNT** `[15:8]` (RO)
///
///   need_des
///
/// - **APPROACH_PAD2_CNT** `[7:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct Status16 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status16 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status16 {
    // ======== Field Constants ========

    /// **SLP_APPROACH_CNT**
    ///
    /// need_des
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const SLP_APPROACH_CNT_OFFSET: u32 = 24;
    pub const SLP_APPROACH_CNT_MASK: u32 = 0xff000000;

    /// **APPROACH_PAD0_CNT**
    ///
    /// need_des
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const APPROACH_PAD0_CNT_OFFSET: u32 = 16;
    pub const APPROACH_PAD0_CNT_MASK: u32 = 0x00ff0000;

    /// **APPROACH_PAD1_CNT**
    ///
    /// need_des
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const APPROACH_PAD1_CNT_OFFSET: u32 = 8;
    pub const APPROACH_PAD1_CNT_MASK: u32 = 0x0000ff00;

    /// **APPROACH_PAD2_CNT**
    ///
    /// need_des
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const APPROACH_PAD2_CNT_OFFSET: u32 = 0;
    pub const APPROACH_PAD2_CNT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SLP_APPROACH_CNT** `[31:24]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn slp_approach_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::SLP_APPROACH_CNT_MASK) >> Self::SLP_APPROACH_CNT_OFFSET
    }

    /// **APPROACH_PAD0_CNT** `[23:16]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn approach_pad0_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::APPROACH_PAD0_CNT_MASK) >> Self::APPROACH_PAD0_CNT_OFFSET
    }

    /// **APPROACH_PAD1_CNT** `[15:8]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn approach_pad1_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::APPROACH_PAD1_CNT_MASK) >> Self::APPROACH_PAD1_CNT_OFFSET
    }

    /// **APPROACH_PAD2_CNT** `[7:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn approach_pad2_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::APPROACH_PAD2_CNT_MASK) >> Self::APPROACH_PAD2_CNT_OFFSET
    }
}

/// STATUS_16 value type for batch read/modify/write operations
///
/// Use `status16.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status16.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status16Val {
    bits: u32,
}

impl Status16Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SLP_APPROACH_CNT** `[31:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn slp_approach_cnt(&self) -> u32 {
        (self.bits & Status16::SLP_APPROACH_CNT_MASK) >> Status16::SLP_APPROACH_CNT_OFFSET
    }

    /// Read **APPROACH_PAD0_CNT** `[23:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn approach_pad0_cnt(&self) -> u32 {
        (self.bits & Status16::APPROACH_PAD0_CNT_MASK) >> Status16::APPROACH_PAD0_CNT_OFFSET
    }

    /// Read **APPROACH_PAD1_CNT** `[15:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn approach_pad1_cnt(&self) -> u32 {
        (self.bits & Status16::APPROACH_PAD1_CNT_MASK) >> Status16::APPROACH_PAD1_CNT_OFFSET
    }

    /// Read **APPROACH_PAD2_CNT** `[7:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn approach_pad2_cnt(&self) -> u32 {
        (self.bits & Status16::APPROACH_PAD2_CNT_MASK) >> Status16::APPROACH_PAD2_CNT_OFFSET
    }
}

impl Default for Status16Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status16 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status16Val {
        Status16Val::from_bits(self.value.read_bits())
    }
}

/// STATUS_17 Register
///
/// need_des
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **RTC_FREQ_SCAN_CNT** `[24:23]` (RO)
///
///   need_des
///
/// - **DBIAS** `[22:18]` (RO)
///
///   need_des
///
/// - **DRV_HS** `[17:13]` (RO)
///
///   need_des
///
/// - **DRV_LS** `[12:9]` (RO)
///
///   need_des
///
/// - **DRES_LPF** `[8:7]` (RO)
///
///   need_des
///
/// - **DCAP_LPF** `[6:0]` (RO)
///
///   Reserved
#[repr(transparent)]
pub struct Status17 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status17 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status17 {
    // ======== Field Constants ========

    /// **RTC_FREQ_SCAN_CNT**
    ///
    /// need_des
    ///
    /// Bits `[24:23]`, mask `0x01800000`
    pub const RTC_FREQ_SCAN_CNT_OFFSET: u32 = 23;
    pub const RTC_FREQ_SCAN_CNT_MASK: u32 = 0x01800000;

    /// **DBIAS**
    ///
    /// need_des
    ///
    /// Bits `[22:18]`, mask `0x007c0000`
    pub const DBIAS_OFFSET: u32 = 18;
    pub const DBIAS_MASK: u32 = 0x007c0000;

    /// **DRV_HS**
    ///
    /// need_des
    ///
    /// Bits `[17:13]`, mask `0x0003e000`
    pub const DRV_HS_OFFSET: u32 = 13;
    pub const DRV_HS_MASK: u32 = 0x0003e000;

    /// **DRV_LS**
    ///
    /// need_des
    ///
    /// Bits `[12:9]`, mask `0x00001e00`
    pub const DRV_LS_OFFSET: u32 = 9;
    pub const DRV_LS_MASK: u32 = 0x00001e00;

    /// **DRES_LPF**
    ///
    /// need_des
    ///
    /// Bits `[8:7]`, mask `0x00000180`
    pub const DRES_LPF_OFFSET: u32 = 7;
    pub const DRES_LPF_MASK: u32 = 0x00000180;

    /// **DCAP_LPF**
    ///
    /// Reserved
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const DCAP_LPF_OFFSET: u32 = 0;
    pub const DCAP_LPF_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **RTC_FREQ_SCAN_CNT** `[24:23]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn rtc_freq_scan_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::RTC_FREQ_SCAN_CNT_MASK) >> Self::RTC_FREQ_SCAN_CNT_OFFSET
    }

    /// **DBIAS** `[22:18]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn dbias(&self) -> u32 {
        (self.value.read_bits() & Self::DBIAS_MASK) >> Self::DBIAS_OFFSET
    }

    /// **DRV_HS** `[17:13]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn drv_hs(&self) -> u32 {
        (self.value.read_bits() & Self::DRV_HS_MASK) >> Self::DRV_HS_OFFSET
    }

    /// **DRV_LS** `[12:9]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn drv_ls(&self) -> u32 {
        (self.value.read_bits() & Self::DRV_LS_MASK) >> Self::DRV_LS_OFFSET
    }

    /// **DRES_LPF** `[8:7]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn dres_lpf(&self) -> u32 {
        (self.value.read_bits() & Self::DRES_LPF_MASK) >> Self::DRES_LPF_OFFSET
    }

    /// **DCAP_LPF** `[6:0]` (RO)
    ///
    /// Reserved
    #[inline(always)]
    pub fn dcap_lpf(&self) -> u32 {
        (self.value.read_bits() & Self::DCAP_LPF_MASK) >> Self::DCAP_LPF_OFFSET
    }
}

/// STATUS_17 value type for batch read/modify/write operations
///
/// Use `status17.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status17.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Status17Val {
    bits: u32,
}

impl Status17Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RTC_FREQ_SCAN_CNT** `[24:23]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rtc_freq_scan_cnt(&self) -> u32 {
        (self.bits & Status17::RTC_FREQ_SCAN_CNT_MASK) >> Status17::RTC_FREQ_SCAN_CNT_OFFSET
    }

    /// Read **DBIAS** `[22:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn dbias(&self) -> u32 {
        (self.bits & Status17::DBIAS_MASK) >> Status17::DBIAS_OFFSET
    }

    /// Read **DRV_HS** `[17:13]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn drv_hs(&self) -> u32 {
        (self.bits & Status17::DRV_HS_MASK) >> Status17::DRV_HS_OFFSET
    }

    /// Read **DRV_LS** `[12:9]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn drv_ls(&self) -> u32 {
        (self.bits & Status17::DRV_LS_MASK) >> Status17::DRV_LS_OFFSET
    }

    /// Read **DRES_LPF** `[8:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn dres_lpf(&self) -> u32 {
        (self.bits & Status17::DRES_LPF_MASK) >> Status17::DRES_LPF_OFFSET
    }

    /// Read **DCAP_LPF** `[6:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn dcap_lpf(&self) -> u32 {
        (self.bits & Status17::DCAP_LPF_MASK) >> Status17::DCAP_LPF_OFFSET
    }
}

impl Default for Status17Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status17 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Status17Val {
        Status17Val::from_bits(self.value.read_bits())
    }
}

/// CHN_TMP_STATUS Register
///
/// need_des
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **PAD_ACTIVE_STATUS** `[29:15]` (RO)
///
///   need_des
///
/// - **PAD_INACTIVE_STATUS** `[14:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct ChnTmpStatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ChnTmpStatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ChnTmpStatus {
    // ======== Field Constants ========

    /// **PAD_ACTIVE_STATUS**
    ///
    /// need_des
    ///
    /// Bits `[29:15]`, mask `0x3fff8000`
    pub const PAD_ACTIVE_STATUS_OFFSET: u32 = 15;
    pub const PAD_ACTIVE_STATUS_MASK: u32 = 0x3fff8000;

    /// **PAD_INACTIVE_STATUS**
    ///
    /// need_des
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const PAD_INACTIVE_STATUS_OFFSET: u32 = 0;
    pub const PAD_INACTIVE_STATUS_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **PAD_ACTIVE_STATUS** `[29:15]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad_active_status(&self) -> u32 {
        (self.value.read_bits() & Self::PAD_ACTIVE_STATUS_MASK) >> Self::PAD_ACTIVE_STATUS_OFFSET
    }

    /// **PAD_INACTIVE_STATUS** `[14:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad_inactive_status(&self) -> u32 {
        (self.value.read_bits() & Self::PAD_INACTIVE_STATUS_MASK)
            >> Self::PAD_INACTIVE_STATUS_OFFSET
    }
}

/// CHN_TMP_STATUS value type for batch read/modify/write operations
///
/// Use `chntmpstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `chntmpstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ChnTmpStatusVal {
    bits: u32,
}

impl ChnTmpStatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PAD_ACTIVE_STATUS** `[29:15]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad_active_status(&self) -> u32 {
        (self.bits & ChnTmpStatus::PAD_ACTIVE_STATUS_MASK) >> ChnTmpStatus::PAD_ACTIVE_STATUS_OFFSET
    }

    /// Read **PAD_INACTIVE_STATUS** `[14:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad_inactive_status(&self) -> u32 {
        (self.bits & ChnTmpStatus::PAD_INACTIVE_STATUS_MASK)
            >> ChnTmpStatus::PAD_INACTIVE_STATUS_OFFSET
    }
}

impl Default for ChnTmpStatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ChnTmpStatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ChnTmpStatusVal {
        ChnTmpStatusVal::from_bits(self.value.read_bits())
    }
}

/// DATE Register
///
/// need_des
///
/// Address offset: 0x0100
/// Reset value: 0x00230314
///
/// ## Fields
///
/// - **RTC_CLK_EN** `[31]` (RW)
///
///   need_des
///
/// - **RTC_DATE** `[27:0]` (RW)
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

    /// **RTC_CLK_EN**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const RTC_CLK_EN_OFFSET: u32 = 31;
    pub const RTC_CLK_EN_MASK: u32 = 0x80000000;
    pub const RTC_CLK_EN: u32 = 1 << 31;

    /// **RTC_DATE**
    ///
    /// need_des
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const RTC_DATE_OFFSET: u32 = 0;
    pub const RTC_DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **RTC_CLK_EN** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rtc_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::RTC_CLK_EN_MASK) != 0
    }

    /// Set RTC_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rtc_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::RTC_CLK_EN_MASK);
    }

    /// Clear RTC_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rtc_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::RTC_CLK_EN_MASK);
    }

    /// **RTC_DATE** `[27:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn rtc_date(&self) -> u32 {
        (self.value.read_bits() & Self::RTC_DATE_MASK) >> Self::RTC_DATE_OFFSET
    }

    /// Write **RTC_DATE** `[27:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_rtc_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RTC_DATE_MASK) | ((value << Self::RTC_DATE_OFFSET) & Self::RTC_DATE_MASK)
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
        Self { bits: 0x00230314 }
    }

    /// Read **RTC_CLK_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rtc_clk_en(&self) -> bool {
        (self.bits & Date::RTC_CLK_EN_MASK) != 0
    }

    /// Read **RTC_DATE** `[27:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn rtc_date(&self) -> u32 {
        (self.bits & Date::RTC_DATE_MASK) >> Date::RTC_DATE_OFFSET
    }

    /// Set **RTC_CLK_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rtc_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Date::RTC_CLK_EN_MASK
            } else {
                self.bits & !Date::RTC_CLK_EN_MASK
            },
        }
    }

    /// Set **RTC_DATE** `[27:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_rtc_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Date::RTC_DATE_MASK)
                | ((value << Date::RTC_DATE_OFFSET) & Date::RTC_DATE_MASK),
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
    /// need_des
    pub int_raw: IntRaw,
    /// 0x0004
    /// need_des
    pub int_st: IntSt,
    /// 0x0008
    /// need_des
    pub int_ena: IntEna,
    /// 0x000c
    /// need_des
    pub int_clr: IntClr,
    /// 0x0010
    /// need_des
    pub chn_status: ChnStatus,
    /// 0x0014
    /// need_des
    pub status0: Status0,
    /// 0x0018
    /// need_des
    pub status1: Status1,
    /// 0x001c
    /// need_des
    pub status2: Status2,
    /// 0x0020
    /// need_des
    pub status3: Status3,
    /// 0x0024
    /// need_des
    pub status4: Status4,
    /// 0x0028
    /// need_des
    pub status5: Status5,
    /// 0x002c
    /// need_des
    pub status6: Status6,
    /// 0x0030
    /// need_des
    pub status7: Status7,
    /// 0x0034
    /// need_des
    pub status8: Status8,
    /// 0x0038
    /// need_des
    pub status9: Status9,
    /// 0x003c
    /// need_des
    pub status10: Status10,
    /// 0x0040
    /// need_des
    pub status11: Status11,
    /// 0x0044
    /// need_des
    pub status12: Status12,
    /// 0x0048
    /// need_des
    pub status13: Status13,
    /// 0x004c
    /// need_des
    pub status14: Status14,
    /// 0x0050
    /// need_des
    pub status15: Status15,
    /// 0x0054
    /// need_des
    pub status16: Status16,
    /// 0x0058
    /// need_des
    pub status17: Status17,
    /// 0x005c
    /// need_des
    pub chn_tmp_status: ChnTmpStatus,
    _reserved0: [u8; 160],
    /// 0x0100
    /// need_des
    pub date: Date,
}
