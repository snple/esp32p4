//! LP_TIMER
//! Low-power Timer

#![allow(dead_code)]

pub const BASE: u32 = 0x50112000;

/// TAR0_LOW Register
///
/// need_des
///
/// Address offset: 0x0000
///
/// ## Fields
///
/// - **MAIN_TIMER_TAR_LOW0** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Tar0Low {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Tar0Low {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Tar0Low {
    // ======== Field Constants ========

    /// **MAIN_TIMER_TAR_LOW0**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MAIN_TIMER_TAR_LOW0_OFFSET: u32 = 0;
    pub const MAIN_TIMER_TAR_LOW0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MAIN_TIMER_TAR_LOW0** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_tar_low0(&self) -> u32 {
        (self.value.read_bits() & Self::MAIN_TIMER_TAR_LOW0_MASK)
            >> Self::MAIN_TIMER_TAR_LOW0_OFFSET
    }

    /// Write **MAIN_TIMER_TAR_LOW0** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_main_timer_tar_low0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAIN_TIMER_TAR_LOW0_MASK)
                | ((value << Self::MAIN_TIMER_TAR_LOW0_OFFSET) & Self::MAIN_TIMER_TAR_LOW0_MASK)
        });
    }
}

/// TAR0_LOW value type for batch read/modify/write operations
///
/// Use `tar0low.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tar0low.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Tar0LowVal {
    bits: u32,
}

impl Tar0LowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAIN_TIMER_TAR_LOW0** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_tar_low0(&self) -> u32 {
        (self.bits & Tar0Low::MAIN_TIMER_TAR_LOW0_MASK) >> Tar0Low::MAIN_TIMER_TAR_LOW0_OFFSET
    }

    /// Set **MAIN_TIMER_TAR_LOW0** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_tar_low0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Tar0Low::MAIN_TIMER_TAR_LOW0_MASK)
                | ((value << Tar0Low::MAIN_TIMER_TAR_LOW0_OFFSET)
                    & Tar0Low::MAIN_TIMER_TAR_LOW0_MASK),
        }
    }
}

impl Default for Tar0LowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Tar0Low {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Tar0LowVal {
        Tar0LowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Tar0LowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Tar0LowVal) -> Tar0LowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TAR0_HIGH Register
///
/// need_des
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **MAIN_TIMER_TAR_EN0** `[31]` (WO)
///
///   need_des
///
/// - **MAIN_TIMER_TAR_HIGH0** `[15:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Tar0High {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Tar0High {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Tar0High {
    // ======== Field Constants ========

    /// **MAIN_TIMER_TAR_EN0**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const MAIN_TIMER_TAR_EN0_OFFSET: u32 = 31;
    pub const MAIN_TIMER_TAR_EN0_MASK: u32 = 0x80000000;
    pub const MAIN_TIMER_TAR_EN0: u32 = 1 << 31;

    /// **MAIN_TIMER_TAR_HIGH0**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAIN_TIMER_TAR_HIGH0_OFFSET: u32 = 0;
    pub const MAIN_TIMER_TAR_HIGH0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Trigger **MAIN_TIMER_TAR_EN0** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_main_timer_tar_en0(&self) {
        self.value
            .modify_bits(|r| r | Self::MAIN_TIMER_TAR_EN0_MASK);
    }

    /// **MAIN_TIMER_TAR_HIGH0** `[15:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_tar_high0(&self) -> u32 {
        (self.value.read_bits() & Self::MAIN_TIMER_TAR_HIGH0_MASK)
            >> Self::MAIN_TIMER_TAR_HIGH0_OFFSET
    }

    /// Write **MAIN_TIMER_TAR_HIGH0** `[15:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_main_timer_tar_high0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAIN_TIMER_TAR_HIGH0_MASK)
                | ((value << Self::MAIN_TIMER_TAR_HIGH0_OFFSET) & Self::MAIN_TIMER_TAR_HIGH0_MASK)
        });
    }
}

/// TAR0_HIGH value type for batch read/modify/write operations
///
/// Use `tar0high.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tar0high.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Tar0HighVal {
    bits: u32,
}

impl Tar0HighVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAIN_TIMER_TAR_HIGH0** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_tar_high0(&self) -> u32 {
        (self.bits & Tar0High::MAIN_TIMER_TAR_HIGH0_MASK) >> Tar0High::MAIN_TIMER_TAR_HIGH0_OFFSET
    }

    /// Set **MAIN_TIMER_TAR_EN0** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_tar_en0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Tar0High::MAIN_TIMER_TAR_EN0_MASK
            } else {
                self.bits & !Tar0High::MAIN_TIMER_TAR_EN0_MASK
            },
        }
    }

    /// Set **MAIN_TIMER_TAR_HIGH0** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_tar_high0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Tar0High::MAIN_TIMER_TAR_HIGH0_MASK)
                | ((value << Tar0High::MAIN_TIMER_TAR_HIGH0_OFFSET)
                    & Tar0High::MAIN_TIMER_TAR_HIGH0_MASK),
        }
    }
}

impl Default for Tar0HighVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Tar0High {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Tar0HighVal {
        Tar0HighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Tar0HighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Tar0HighVal) -> Tar0HighVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TAR1_LOW Register
///
/// need_des
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **MAIN_TIMER_TAR_LOW1** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Tar1Low {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Tar1Low {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Tar1Low {
    // ======== Field Constants ========

    /// **MAIN_TIMER_TAR_LOW1**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MAIN_TIMER_TAR_LOW1_OFFSET: u32 = 0;
    pub const MAIN_TIMER_TAR_LOW1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MAIN_TIMER_TAR_LOW1** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_tar_low1(&self) -> u32 {
        (self.value.read_bits() & Self::MAIN_TIMER_TAR_LOW1_MASK)
            >> Self::MAIN_TIMER_TAR_LOW1_OFFSET
    }

    /// Write **MAIN_TIMER_TAR_LOW1** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_main_timer_tar_low1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAIN_TIMER_TAR_LOW1_MASK)
                | ((value << Self::MAIN_TIMER_TAR_LOW1_OFFSET) & Self::MAIN_TIMER_TAR_LOW1_MASK)
        });
    }
}

/// TAR1_LOW value type for batch read/modify/write operations
///
/// Use `tar1low.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tar1low.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Tar1LowVal {
    bits: u32,
}

impl Tar1LowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAIN_TIMER_TAR_LOW1** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_tar_low1(&self) -> u32 {
        (self.bits & Tar1Low::MAIN_TIMER_TAR_LOW1_MASK) >> Tar1Low::MAIN_TIMER_TAR_LOW1_OFFSET
    }

    /// Set **MAIN_TIMER_TAR_LOW1** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_tar_low1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Tar1Low::MAIN_TIMER_TAR_LOW1_MASK)
                | ((value << Tar1Low::MAIN_TIMER_TAR_LOW1_OFFSET)
                    & Tar1Low::MAIN_TIMER_TAR_LOW1_MASK),
        }
    }
}

impl Default for Tar1LowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Tar1Low {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Tar1LowVal {
        Tar1LowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Tar1LowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Tar1LowVal) -> Tar1LowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TAR1_HIGH Register
///
/// need_des
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **MAIN_TIMER_TAR_EN1** `[31]` (WO)
///
///   need_des
///
/// - **MAIN_TIMER_TAR_HIGH1** `[15:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct Tar1High {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Tar1High {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Tar1High {
    // ======== Field Constants ========

    /// **MAIN_TIMER_TAR_EN1**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const MAIN_TIMER_TAR_EN1_OFFSET: u32 = 31;
    pub const MAIN_TIMER_TAR_EN1_MASK: u32 = 0x80000000;
    pub const MAIN_TIMER_TAR_EN1: u32 = 1 << 31;

    /// **MAIN_TIMER_TAR_HIGH1**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAIN_TIMER_TAR_HIGH1_OFFSET: u32 = 0;
    pub const MAIN_TIMER_TAR_HIGH1_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Trigger **MAIN_TIMER_TAR_EN1** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_main_timer_tar_en1(&self) {
        self.value
            .modify_bits(|r| r | Self::MAIN_TIMER_TAR_EN1_MASK);
    }

    /// **MAIN_TIMER_TAR_HIGH1** `[15:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_tar_high1(&self) -> u32 {
        (self.value.read_bits() & Self::MAIN_TIMER_TAR_HIGH1_MASK)
            >> Self::MAIN_TIMER_TAR_HIGH1_OFFSET
    }

    /// Write **MAIN_TIMER_TAR_HIGH1** `[15:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_main_timer_tar_high1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAIN_TIMER_TAR_HIGH1_MASK)
                | ((value << Self::MAIN_TIMER_TAR_HIGH1_OFFSET) & Self::MAIN_TIMER_TAR_HIGH1_MASK)
        });
    }
}

/// TAR1_HIGH value type for batch read/modify/write operations
///
/// Use `tar1high.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tar1high.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Tar1HighVal {
    bits: u32,
}

impl Tar1HighVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAIN_TIMER_TAR_HIGH1** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_tar_high1(&self) -> u32 {
        (self.bits & Tar1High::MAIN_TIMER_TAR_HIGH1_MASK) >> Tar1High::MAIN_TIMER_TAR_HIGH1_OFFSET
    }

    /// Set **MAIN_TIMER_TAR_EN1** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_tar_en1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Tar1High::MAIN_TIMER_TAR_EN1_MASK
            } else {
                self.bits & !Tar1High::MAIN_TIMER_TAR_EN1_MASK
            },
        }
    }

    /// Set **MAIN_TIMER_TAR_HIGH1** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_tar_high1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Tar1High::MAIN_TIMER_TAR_HIGH1_MASK)
                | ((value << Tar1High::MAIN_TIMER_TAR_HIGH1_OFFSET)
                    & Tar1High::MAIN_TIMER_TAR_HIGH1_MASK),
        }
    }
}

impl Default for Tar1HighVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Tar1High {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Tar1HighVal {
        Tar1HighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Tar1HighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Tar1HighVal) -> Tar1HighVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UPDATE Register
///
/// need_des
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **MAIN_TIMER_SYS_RST** `[31]` (RW)
///
///   need_des
///
/// - **MAIN_TIMER_SYS_STALL** `[30]` (RW)
///
///   need_des
///
/// - **MAIN_TIMER_XTAL_OFF** `[29]` (RW)
///
///   need_des
///
/// - **MAIN_TIMER_UPDATE** `[28]` (WO)
///
///   need_des
#[repr(transparent)]
pub struct Update {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Update {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Update {
    // ======== Field Constants ========

    /// **MAIN_TIMER_SYS_RST**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const MAIN_TIMER_SYS_RST_OFFSET: u32 = 31;
    pub const MAIN_TIMER_SYS_RST_MASK: u32 = 0x80000000;
    pub const MAIN_TIMER_SYS_RST: u32 = 1 << 31;

    /// **MAIN_TIMER_SYS_STALL**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const MAIN_TIMER_SYS_STALL_OFFSET: u32 = 30;
    pub const MAIN_TIMER_SYS_STALL_MASK: u32 = 0x40000000;
    pub const MAIN_TIMER_SYS_STALL: u32 = 1 << 30;

    /// **MAIN_TIMER_XTAL_OFF**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const MAIN_TIMER_XTAL_OFF_OFFSET: u32 = 29;
    pub const MAIN_TIMER_XTAL_OFF_MASK: u32 = 0x20000000;
    pub const MAIN_TIMER_XTAL_OFF: u32 = 1 << 29;

    /// **MAIN_TIMER_UPDATE**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const MAIN_TIMER_UPDATE_OFFSET: u32 = 28;
    pub const MAIN_TIMER_UPDATE_MASK: u32 = 0x10000000;
    pub const MAIN_TIMER_UPDATE: u32 = 1 << 28;

    // ======== Field Accessors ========

    /// **MAIN_TIMER_SYS_RST** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_sys_rst(&self) -> bool {
        (self.value.read_bits() & Self::MAIN_TIMER_SYS_RST_MASK) != 0
    }

    /// Set MAIN_TIMER_SYS_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_main_timer_sys_rst(&self) {
        self.value
            .modify_bits(|r| r | Self::MAIN_TIMER_SYS_RST_MASK);
    }

    /// Clear MAIN_TIMER_SYS_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_main_timer_sys_rst(&self) {
        self.value
            .modify_bits(|r| r & !Self::MAIN_TIMER_SYS_RST_MASK);
    }

    /// **MAIN_TIMER_SYS_STALL** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_sys_stall(&self) -> bool {
        (self.value.read_bits() & Self::MAIN_TIMER_SYS_STALL_MASK) != 0
    }

    /// Set MAIN_TIMER_SYS_STALL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_main_timer_sys_stall(&self) {
        self.value
            .modify_bits(|r| r | Self::MAIN_TIMER_SYS_STALL_MASK);
    }

    /// Clear MAIN_TIMER_SYS_STALL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_main_timer_sys_stall(&self) {
        self.value
            .modify_bits(|r| r & !Self::MAIN_TIMER_SYS_STALL_MASK);
    }

    /// **MAIN_TIMER_XTAL_OFF** `[29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_xtal_off(&self) -> bool {
        (self.value.read_bits() & Self::MAIN_TIMER_XTAL_OFF_MASK) != 0
    }

    /// Set MAIN_TIMER_XTAL_OFF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_main_timer_xtal_off(&self) {
        self.value
            .modify_bits(|r| r | Self::MAIN_TIMER_XTAL_OFF_MASK);
    }

    /// Clear MAIN_TIMER_XTAL_OFF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_main_timer_xtal_off(&self) {
        self.value
            .modify_bits(|r| r & !Self::MAIN_TIMER_XTAL_OFF_MASK);
    }

    /// Trigger **MAIN_TIMER_UPDATE** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_main_timer_update(&self) {
        self.value.modify_bits(|r| r | Self::MAIN_TIMER_UPDATE_MASK);
    }
}

/// UPDATE value type for batch read/modify/write operations
///
/// Use `update.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `update.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UpdateVal {
    bits: u32,
}

impl UpdateVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAIN_TIMER_SYS_RST** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_sys_rst(&self) -> bool {
        (self.bits & Update::MAIN_TIMER_SYS_RST_MASK) != 0
    }

    /// Read **MAIN_TIMER_SYS_STALL** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_sys_stall(&self) -> bool {
        (self.bits & Update::MAIN_TIMER_SYS_STALL_MASK) != 0
    }

    /// Read **MAIN_TIMER_XTAL_OFF** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_xtal_off(&self) -> bool {
        (self.bits & Update::MAIN_TIMER_XTAL_OFF_MASK) != 0
    }

    /// Set **MAIN_TIMER_SYS_RST** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_sys_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Update::MAIN_TIMER_SYS_RST_MASK
            } else {
                self.bits & !Update::MAIN_TIMER_SYS_RST_MASK
            },
        }
    }

    /// Set **MAIN_TIMER_SYS_STALL** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_sys_stall(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Update::MAIN_TIMER_SYS_STALL_MASK
            } else {
                self.bits & !Update::MAIN_TIMER_SYS_STALL_MASK
            },
        }
    }

    /// Set **MAIN_TIMER_XTAL_OFF** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_xtal_off(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Update::MAIN_TIMER_XTAL_OFF_MASK
            } else {
                self.bits & !Update::MAIN_TIMER_XTAL_OFF_MASK
            },
        }
    }

    /// Set **MAIN_TIMER_UPDATE** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Update::MAIN_TIMER_UPDATE_MASK
            } else {
                self.bits & !Update::MAIN_TIMER_UPDATE_MASK
            },
        }
    }
}

impl Default for UpdateVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Update {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UpdateVal {
        UpdateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UpdateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UpdateVal) -> UpdateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MAIN_BUF0_LOW Register
///
/// need_des
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **MAIN_TIMER_BUF0_LOW** `[31:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct MainBuf0Low {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MainBuf0Low {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MainBuf0Low {
    // ======== Field Constants ========

    /// **MAIN_TIMER_BUF0_LOW**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MAIN_TIMER_BUF0_LOW_OFFSET: u32 = 0;
    pub const MAIN_TIMER_BUF0_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MAIN_TIMER_BUF0_LOW** `[31:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_buf0_low(&self) -> u32 {
        (self.value.read_bits() & Self::MAIN_TIMER_BUF0_LOW_MASK)
            >> Self::MAIN_TIMER_BUF0_LOW_OFFSET
    }
}

/// MAIN_BUF0_LOW value type for batch read/modify/write operations
///
/// Use `mainbuf0low.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mainbuf0low.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MainBuf0LowVal {
    bits: u32,
}

impl MainBuf0LowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAIN_TIMER_BUF0_LOW** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_buf0_low(&self) -> u32 {
        (self.bits & MainBuf0Low::MAIN_TIMER_BUF0_LOW_MASK)
            >> MainBuf0Low::MAIN_TIMER_BUF0_LOW_OFFSET
    }
}

impl Default for MainBuf0LowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MainBuf0Low {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MainBuf0LowVal {
        MainBuf0LowVal::from_bits(self.value.read_bits())
    }
}

/// MAIN_BUF0_HIGH Register
///
/// need_des
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **MAIN_TIMER_BUF0_HIGH** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct MainBuf0High {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MainBuf0High {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MainBuf0High {
    // ======== Field Constants ========

    /// **MAIN_TIMER_BUF0_HIGH**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAIN_TIMER_BUF0_HIGH_OFFSET: u32 = 0;
    pub const MAIN_TIMER_BUF0_HIGH_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **MAIN_TIMER_BUF0_HIGH** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_buf0_high(&self) -> u32 {
        (self.value.read_bits() & Self::MAIN_TIMER_BUF0_HIGH_MASK)
            >> Self::MAIN_TIMER_BUF0_HIGH_OFFSET
    }
}

/// MAIN_BUF0_HIGH value type for batch read/modify/write operations
///
/// Use `mainbuf0high.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mainbuf0high.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MainBuf0HighVal {
    bits: u32,
}

impl MainBuf0HighVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAIN_TIMER_BUF0_HIGH** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_buf0_high(&self) -> u32 {
        (self.bits & MainBuf0High::MAIN_TIMER_BUF0_HIGH_MASK)
            >> MainBuf0High::MAIN_TIMER_BUF0_HIGH_OFFSET
    }
}

impl Default for MainBuf0HighVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MainBuf0High {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MainBuf0HighVal {
        MainBuf0HighVal::from_bits(self.value.read_bits())
    }
}

/// MAIN_BUF1_LOW Register
///
/// need_des
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **MAIN_TIMER_BUF1_LOW** `[31:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct MainBuf1Low {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MainBuf1Low {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MainBuf1Low {
    // ======== Field Constants ========

    /// **MAIN_TIMER_BUF1_LOW**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const MAIN_TIMER_BUF1_LOW_OFFSET: u32 = 0;
    pub const MAIN_TIMER_BUF1_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **MAIN_TIMER_BUF1_LOW** `[31:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_buf1_low(&self) -> u32 {
        (self.value.read_bits() & Self::MAIN_TIMER_BUF1_LOW_MASK)
            >> Self::MAIN_TIMER_BUF1_LOW_OFFSET
    }
}

/// MAIN_BUF1_LOW value type for batch read/modify/write operations
///
/// Use `mainbuf1low.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mainbuf1low.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MainBuf1LowVal {
    bits: u32,
}

impl MainBuf1LowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAIN_TIMER_BUF1_LOW** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_buf1_low(&self) -> u32 {
        (self.bits & MainBuf1Low::MAIN_TIMER_BUF1_LOW_MASK)
            >> MainBuf1Low::MAIN_TIMER_BUF1_LOW_OFFSET
    }
}

impl Default for MainBuf1LowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MainBuf1Low {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MainBuf1LowVal {
        MainBuf1LowVal::from_bits(self.value.read_bits())
    }
}

/// MAIN_BUF1_HIGH Register
///
/// need_des
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **MAIN_TIMER_BUF1_HIGH** `[15:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct MainBuf1High {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MainBuf1High {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MainBuf1High {
    // ======== Field Constants ========

    /// **MAIN_TIMER_BUF1_HIGH**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAIN_TIMER_BUF1_HIGH_OFFSET: u32 = 0;
    pub const MAIN_TIMER_BUF1_HIGH_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **MAIN_TIMER_BUF1_HIGH** `[15:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_buf1_high(&self) -> u32 {
        (self.value.read_bits() & Self::MAIN_TIMER_BUF1_HIGH_MASK)
            >> Self::MAIN_TIMER_BUF1_HIGH_OFFSET
    }
}

/// MAIN_BUF1_HIGH value type for batch read/modify/write operations
///
/// Use `mainbuf1high.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mainbuf1high.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MainBuf1HighVal {
    bits: u32,
}

impl MainBuf1HighVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAIN_TIMER_BUF1_HIGH** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_buf1_high(&self) -> u32 {
        (self.bits & MainBuf1High::MAIN_TIMER_BUF1_HIGH_MASK)
            >> MainBuf1High::MAIN_TIMER_BUF1_HIGH_OFFSET
    }
}

impl Default for MainBuf1HighVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MainBuf1High {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MainBuf1HighVal {
        MainBuf1HighVal::from_bits(self.value.read_bits())
    }
}

/// MAIN_OVERFLOW Register
///
/// need_des
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **MAIN_TIMER_ALARM_LOAD** `[31]` (WO)
///
///   need_des
#[repr(transparent)]
pub struct MainOverflow {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MainOverflow {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MainOverflow {
    // ======== Field Constants ========

    /// **MAIN_TIMER_ALARM_LOAD**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const MAIN_TIMER_ALARM_LOAD_OFFSET: u32 = 31;
    pub const MAIN_TIMER_ALARM_LOAD_MASK: u32 = 0x80000000;
    pub const MAIN_TIMER_ALARM_LOAD: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// Trigger **MAIN_TIMER_ALARM_LOAD** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_main_timer_alarm_load(&self) {
        self.value.write_bits(Self::MAIN_TIMER_ALARM_LOAD_MASK);
    }
}

/// MAIN_OVERFLOW value type for batch read/modify/write operations
///
/// Use `mainoverflow.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `mainoverflow.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MainOverflowVal {
    bits: u32,
}

impl MainOverflowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **MAIN_TIMER_ALARM_LOAD** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_alarm_load(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MainOverflow::MAIN_TIMER_ALARM_LOAD_MASK
            } else {
                self.bits & !MainOverflow::MAIN_TIMER_ALARM_LOAD_MASK
            },
        }
    }
}

impl Default for MainOverflowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MainOverflow {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MainOverflowVal) {
        self.value.write_bits(val.bits())
    }
}

/// INT_RAW Register
///
/// need_des
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **SOC_WAKEUP_INT_RAW** `[31]` (RW)
///
///   need_des
///
/// - **OVERFLOW_RAW** `[30]` (RW)
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

    /// **SOC_WAKEUP_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SOC_WAKEUP_INT_RAW_OFFSET: u32 = 31;
    pub const SOC_WAKEUP_INT_RAW_MASK: u32 = 0x80000000;
    pub const SOC_WAKEUP_INT_RAW: u32 = 1 << 31;

    /// **OVERFLOW_RAW**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const OVERFLOW_RAW_OFFSET: u32 = 30;
    pub const OVERFLOW_RAW_MASK: u32 = 0x40000000;
    pub const OVERFLOW_RAW: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// **SOC_WAKEUP_INT_RAW** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn soc_wakeup_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SOC_WAKEUP_INT_RAW_MASK) != 0
    }

    /// Set SOC_WAKEUP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_soc_wakeup_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::SOC_WAKEUP_INT_RAW_MASK);
    }

    /// Clear SOC_WAKEUP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_soc_wakeup_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::SOC_WAKEUP_INT_RAW_MASK);
    }

    /// **OVERFLOW_RAW** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn overflow_raw(&self) -> bool {
        (self.value.read_bits() & Self::OVERFLOW_RAW_MASK) != 0
    }

    /// Set OVERFLOW_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_overflow_raw(&self) {
        self.value.modify_bits(|r| r | Self::OVERFLOW_RAW_MASK);
    }

    /// Clear OVERFLOW_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_overflow_raw(&self) {
        self.value.modify_bits(|r| r & !Self::OVERFLOW_RAW_MASK);
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

    /// Read **SOC_WAKEUP_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn soc_wakeup_int_raw(&self) -> bool {
        (self.bits & IntRaw::SOC_WAKEUP_INT_RAW_MASK) != 0
    }

    /// Read **OVERFLOW_RAW** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn overflow_raw(&self) -> bool {
        (self.bits & IntRaw::OVERFLOW_RAW_MASK) != 0
    }

    /// Set **SOC_WAKEUP_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_soc_wakeup_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::SOC_WAKEUP_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::SOC_WAKEUP_INT_RAW_MASK
            },
        }
    }

    /// Set **OVERFLOW_RAW** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_overflow_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::OVERFLOW_RAW_MASK
            } else {
                self.bits & !IntRaw::OVERFLOW_RAW_MASK
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
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **SOC_WAKEUP_INT_ST** `[31]` (RO)
///
///   need_des
///
/// - **OVERFLOW_ST** `[30]` (RO)
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

    /// **SOC_WAKEUP_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SOC_WAKEUP_INT_ST_OFFSET: u32 = 31;
    pub const SOC_WAKEUP_INT_ST_MASK: u32 = 0x80000000;
    pub const SOC_WAKEUP_INT_ST: u32 = 1 << 31;

    /// **OVERFLOW_ST**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const OVERFLOW_ST_OFFSET: u32 = 30;
    pub const OVERFLOW_ST_MASK: u32 = 0x40000000;
    pub const OVERFLOW_ST: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// **SOC_WAKEUP_INT_ST** `[31]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn soc_wakeup_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SOC_WAKEUP_INT_ST_MASK) != 0
    }

    /// **OVERFLOW_ST** `[30]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn overflow_st(&self) -> bool {
        (self.value.read_bits() & Self::OVERFLOW_ST_MASK) != 0
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

    /// Read **SOC_WAKEUP_INT_ST** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn soc_wakeup_int_st(&self) -> bool {
        (self.bits & IntSt::SOC_WAKEUP_INT_ST_MASK) != 0
    }

    /// Read **OVERFLOW_ST** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn overflow_st(&self) -> bool {
        (self.bits & IntSt::OVERFLOW_ST_MASK) != 0
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
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **SOC_WAKEUP_INT_ENA** `[31]` (RW)
///
///   need_des
///
/// - **OVERFLOW_ENA** `[30]` (RW)
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

    /// **SOC_WAKEUP_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SOC_WAKEUP_INT_ENA_OFFSET: u32 = 31;
    pub const SOC_WAKEUP_INT_ENA_MASK: u32 = 0x80000000;
    pub const SOC_WAKEUP_INT_ENA: u32 = 1 << 31;

    /// **OVERFLOW_ENA**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const OVERFLOW_ENA_OFFSET: u32 = 30;
    pub const OVERFLOW_ENA_MASK: u32 = 0x40000000;
    pub const OVERFLOW_ENA: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// **SOC_WAKEUP_INT_ENA** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn soc_wakeup_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SOC_WAKEUP_INT_ENA_MASK) != 0
    }

    /// Set SOC_WAKEUP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_soc_wakeup_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SOC_WAKEUP_INT_ENA_MASK);
    }

    /// Clear SOC_WAKEUP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_soc_wakeup_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SOC_WAKEUP_INT_ENA_MASK);
    }

    /// **OVERFLOW_ENA** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn overflow_ena(&self) -> bool {
        (self.value.read_bits() & Self::OVERFLOW_ENA_MASK) != 0
    }

    /// Set OVERFLOW_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_overflow_ena(&self) {
        self.value.modify_bits(|r| r | Self::OVERFLOW_ENA_MASK);
    }

    /// Clear OVERFLOW_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_overflow_ena(&self) {
        self.value.modify_bits(|r| r & !Self::OVERFLOW_ENA_MASK);
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

    /// Read **SOC_WAKEUP_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn soc_wakeup_int_ena(&self) -> bool {
        (self.bits & IntEna::SOC_WAKEUP_INT_ENA_MASK) != 0
    }

    /// Read **OVERFLOW_ENA** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn overflow_ena(&self) -> bool {
        (self.bits & IntEna::OVERFLOW_ENA_MASK) != 0
    }

    /// Set **SOC_WAKEUP_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_soc_wakeup_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SOC_WAKEUP_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SOC_WAKEUP_INT_ENA_MASK
            },
        }
    }

    /// Set **OVERFLOW_ENA** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_overflow_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::OVERFLOW_ENA_MASK
            } else {
                self.bits & !IntEna::OVERFLOW_ENA_MASK
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
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **SOC_WAKEUP_INT_CLR** `[31]` (WO)
///
///   need_des
///
/// - **OVERFLOW_CLR** `[30]` (WO)
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

    /// **SOC_WAKEUP_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SOC_WAKEUP_INT_CLR_OFFSET: u32 = 31;
    pub const SOC_WAKEUP_INT_CLR_MASK: u32 = 0x80000000;
    pub const SOC_WAKEUP_INT_CLR: u32 = 1 << 31;

    /// **OVERFLOW_CLR**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const OVERFLOW_CLR_OFFSET: u32 = 30;
    pub const OVERFLOW_CLR_MASK: u32 = 0x40000000;
    pub const OVERFLOW_CLR: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// Trigger **SOC_WAKEUP_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_soc_wakeup_int_clr(&self) {
        self.value.write_bits(Self::SOC_WAKEUP_INT_CLR_MASK);
    }

    /// Trigger **OVERFLOW_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_overflow_clr(&self) {
        self.value.write_bits(Self::OVERFLOW_CLR_MASK);
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

    /// Set **SOC_WAKEUP_INT_CLR** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_soc_wakeup_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SOC_WAKEUP_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SOC_WAKEUP_INT_CLR_MASK
            },
        }
    }

    /// Set **OVERFLOW_CLR** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_overflow_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::OVERFLOW_CLR_MASK
            } else {
                self.bits & !IntClr::OVERFLOW_CLR_MASK
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

/// LP_INT_RAW Register
///
/// need_des
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **MAIN_TIMER_LP_INT_RAW** `[31]` (RW)
///
///   need_des
///
/// - **MAIN_TIMER_OVERFLOW_LP_INT_RAW** `[30]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpIntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpIntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpIntRaw {
    // ======== Field Constants ========

    /// **MAIN_TIMER_LP_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const MAIN_TIMER_LP_INT_RAW_OFFSET: u32 = 31;
    pub const MAIN_TIMER_LP_INT_RAW_MASK: u32 = 0x80000000;
    pub const MAIN_TIMER_LP_INT_RAW: u32 = 1 << 31;

    /// **MAIN_TIMER_OVERFLOW_LP_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const MAIN_TIMER_OVERFLOW_LP_INT_RAW_OFFSET: u32 = 30;
    pub const MAIN_TIMER_OVERFLOW_LP_INT_RAW_MASK: u32 = 0x40000000;
    pub const MAIN_TIMER_OVERFLOW_LP_INT_RAW: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// **MAIN_TIMER_LP_INT_RAW** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_lp_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::MAIN_TIMER_LP_INT_RAW_MASK) != 0
    }

    /// Set MAIN_TIMER_LP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_main_timer_lp_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::MAIN_TIMER_LP_INT_RAW_MASK);
    }

    /// Clear MAIN_TIMER_LP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_main_timer_lp_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::MAIN_TIMER_LP_INT_RAW_MASK);
    }

    /// **MAIN_TIMER_OVERFLOW_LP_INT_RAW** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_overflow_lp_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::MAIN_TIMER_OVERFLOW_LP_INT_RAW_MASK) != 0
    }

    /// Set MAIN_TIMER_OVERFLOW_LP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_main_timer_overflow_lp_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::MAIN_TIMER_OVERFLOW_LP_INT_RAW_MASK);
    }

    /// Clear MAIN_TIMER_OVERFLOW_LP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_main_timer_overflow_lp_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::MAIN_TIMER_OVERFLOW_LP_INT_RAW_MASK);
    }
}

/// LP_INT_RAW value type for batch read/modify/write operations
///
/// Use `lpintraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpintraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpIntRawVal {
    bits: u32,
}

impl LpIntRawVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAIN_TIMER_LP_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_lp_int_raw(&self) -> bool {
        (self.bits & LpIntRaw::MAIN_TIMER_LP_INT_RAW_MASK) != 0
    }

    /// Read **MAIN_TIMER_OVERFLOW_LP_INT_RAW** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_overflow_lp_int_raw(&self) -> bool {
        (self.bits & LpIntRaw::MAIN_TIMER_OVERFLOW_LP_INT_RAW_MASK) != 0
    }

    /// Set **MAIN_TIMER_LP_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_lp_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpIntRaw::MAIN_TIMER_LP_INT_RAW_MASK
            } else {
                self.bits & !LpIntRaw::MAIN_TIMER_LP_INT_RAW_MASK
            },
        }
    }

    /// Set **MAIN_TIMER_OVERFLOW_LP_INT_RAW** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_overflow_lp_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpIntRaw::MAIN_TIMER_OVERFLOW_LP_INT_RAW_MASK
            } else {
                self.bits & !LpIntRaw::MAIN_TIMER_OVERFLOW_LP_INT_RAW_MASK
            },
        }
    }
}

impl Default for LpIntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpIntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpIntRawVal {
        LpIntRawVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpIntRawVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpIntRawVal) -> LpIntRawVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_INT_ST Register
///
/// need_des
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **MAIN_TIMER_LP_INT_ST** `[31]` (RO)
///
///   need_des
///
/// - **MAIN_TIMER_OVERFLOW_LP_INT_ST** `[30]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct LpIntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpIntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpIntSt {
    // ======== Field Constants ========

    /// **MAIN_TIMER_LP_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const MAIN_TIMER_LP_INT_ST_OFFSET: u32 = 31;
    pub const MAIN_TIMER_LP_INT_ST_MASK: u32 = 0x80000000;
    pub const MAIN_TIMER_LP_INT_ST: u32 = 1 << 31;

    /// **MAIN_TIMER_OVERFLOW_LP_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const MAIN_TIMER_OVERFLOW_LP_INT_ST_OFFSET: u32 = 30;
    pub const MAIN_TIMER_OVERFLOW_LP_INT_ST_MASK: u32 = 0x40000000;
    pub const MAIN_TIMER_OVERFLOW_LP_INT_ST: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// **MAIN_TIMER_LP_INT_ST** `[31]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_lp_int_st(&self) -> bool {
        (self.value.read_bits() & Self::MAIN_TIMER_LP_INT_ST_MASK) != 0
    }

    /// **MAIN_TIMER_OVERFLOW_LP_INT_ST** `[30]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_overflow_lp_int_st(&self) -> bool {
        (self.value.read_bits() & Self::MAIN_TIMER_OVERFLOW_LP_INT_ST_MASK) != 0
    }
}

/// LP_INT_ST value type for batch read/modify/write operations
///
/// Use `lpintst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpintst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpIntStVal {
    bits: u32,
}

impl LpIntStVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAIN_TIMER_LP_INT_ST** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_lp_int_st(&self) -> bool {
        (self.bits & LpIntSt::MAIN_TIMER_LP_INT_ST_MASK) != 0
    }

    /// Read **MAIN_TIMER_OVERFLOW_LP_INT_ST** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_overflow_lp_int_st(&self) -> bool {
        (self.bits & LpIntSt::MAIN_TIMER_OVERFLOW_LP_INT_ST_MASK) != 0
    }
}

impl Default for LpIntStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpIntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpIntStVal {
        LpIntStVal::from_bits(self.value.read_bits())
    }
}

/// LP_INT_ENA Register
///
/// need_des
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **MAIN_TIMER_LP_INT_ENA** `[31]` (RW)
///
///   need_des
///
/// - **MAIN_TIMER_OVERFLOW_LP_INT_ENA** `[30]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpIntEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpIntEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpIntEna {
    // ======== Field Constants ========

    /// **MAIN_TIMER_LP_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const MAIN_TIMER_LP_INT_ENA_OFFSET: u32 = 31;
    pub const MAIN_TIMER_LP_INT_ENA_MASK: u32 = 0x80000000;
    pub const MAIN_TIMER_LP_INT_ENA: u32 = 1 << 31;

    /// **MAIN_TIMER_OVERFLOW_LP_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const MAIN_TIMER_OVERFLOW_LP_INT_ENA_OFFSET: u32 = 30;
    pub const MAIN_TIMER_OVERFLOW_LP_INT_ENA_MASK: u32 = 0x40000000;
    pub const MAIN_TIMER_OVERFLOW_LP_INT_ENA: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// **MAIN_TIMER_LP_INT_ENA** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_lp_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::MAIN_TIMER_LP_INT_ENA_MASK) != 0
    }

    /// Set MAIN_TIMER_LP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_main_timer_lp_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::MAIN_TIMER_LP_INT_ENA_MASK);
    }

    /// Clear MAIN_TIMER_LP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_main_timer_lp_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::MAIN_TIMER_LP_INT_ENA_MASK);
    }

    /// **MAIN_TIMER_OVERFLOW_LP_INT_ENA** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn main_timer_overflow_lp_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::MAIN_TIMER_OVERFLOW_LP_INT_ENA_MASK) != 0
    }

    /// Set MAIN_TIMER_OVERFLOW_LP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_main_timer_overflow_lp_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::MAIN_TIMER_OVERFLOW_LP_INT_ENA_MASK);
    }

    /// Clear MAIN_TIMER_OVERFLOW_LP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_main_timer_overflow_lp_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::MAIN_TIMER_OVERFLOW_LP_INT_ENA_MASK);
    }
}

/// LP_INT_ENA value type for batch read/modify/write operations
///
/// Use `lpintena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpintena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpIntEnaVal {
    bits: u32,
}

impl LpIntEnaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MAIN_TIMER_LP_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_lp_int_ena(&self) -> bool {
        (self.bits & LpIntEna::MAIN_TIMER_LP_INT_ENA_MASK) != 0
    }

    /// Read **MAIN_TIMER_OVERFLOW_LP_INT_ENA** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn main_timer_overflow_lp_int_ena(&self) -> bool {
        (self.bits & LpIntEna::MAIN_TIMER_OVERFLOW_LP_INT_ENA_MASK) != 0
    }

    /// Set **MAIN_TIMER_LP_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_lp_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpIntEna::MAIN_TIMER_LP_INT_ENA_MASK
            } else {
                self.bits & !LpIntEna::MAIN_TIMER_LP_INT_ENA_MASK
            },
        }
    }

    /// Set **MAIN_TIMER_OVERFLOW_LP_INT_ENA** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_overflow_lp_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpIntEna::MAIN_TIMER_OVERFLOW_LP_INT_ENA_MASK
            } else {
                self.bits & !LpIntEna::MAIN_TIMER_OVERFLOW_LP_INT_ENA_MASK
            },
        }
    }
}

impl Default for LpIntEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpIntEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpIntEnaVal {
        LpIntEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpIntEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpIntEnaVal) -> LpIntEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_INT_CLR Register
///
/// need_des
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **MAIN_TIMER_LP_INT_CLR** `[31]` (WO)
///
///   need_des
///
/// - **MAIN_TIMER_OVERFLOW_LP_INT_CLR** `[30]` (WO)
///
///   need_des
#[repr(transparent)]
pub struct LpIntClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpIntClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpIntClr {
    // ======== Field Constants ========

    /// **MAIN_TIMER_LP_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const MAIN_TIMER_LP_INT_CLR_OFFSET: u32 = 31;
    pub const MAIN_TIMER_LP_INT_CLR_MASK: u32 = 0x80000000;
    pub const MAIN_TIMER_LP_INT_CLR: u32 = 1 << 31;

    /// **MAIN_TIMER_OVERFLOW_LP_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const MAIN_TIMER_OVERFLOW_LP_INT_CLR_OFFSET: u32 = 30;
    pub const MAIN_TIMER_OVERFLOW_LP_INT_CLR_MASK: u32 = 0x40000000;
    pub const MAIN_TIMER_OVERFLOW_LP_INT_CLR: u32 = 1 << 30;

    // ======== Field Accessors ========

    /// Trigger **MAIN_TIMER_LP_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_main_timer_lp_int_clr(&self) {
        self.value.write_bits(Self::MAIN_TIMER_LP_INT_CLR_MASK);
    }

    /// Trigger **MAIN_TIMER_OVERFLOW_LP_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_main_timer_overflow_lp_int_clr(&self) {
        self.value
            .write_bits(Self::MAIN_TIMER_OVERFLOW_LP_INT_CLR_MASK);
    }
}

/// LP_INT_CLR value type for batch read/modify/write operations
///
/// Use `lpintclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpintclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpIntClrVal {
    bits: u32,
}

impl LpIntClrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **MAIN_TIMER_LP_INT_CLR** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_lp_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpIntClr::MAIN_TIMER_LP_INT_CLR_MASK
            } else {
                self.bits & !LpIntClr::MAIN_TIMER_LP_INT_CLR_MASK
            },
        }
    }

    /// Set **MAIN_TIMER_OVERFLOW_LP_INT_CLR** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_main_timer_overflow_lp_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpIntClr::MAIN_TIMER_OVERFLOW_LP_INT_CLR_MASK
            } else {
                self.bits & !LpIntClr::MAIN_TIMER_OVERFLOW_LP_INT_CLR_MASK
            },
        }
    }
}

impl Default for LpIntClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpIntClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpIntClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// DATE Register
///
/// need_des
///
/// Address offset: 0x03fc
/// Reset value: 0x02111150
///
/// ## Fields
///
/// - **CLK_EN** `[31]` (RW)
///
///   need_des
///
/// - **DATE** `[30:0]` (RW)
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

    /// **CLK_EN**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_EN_OFFSET: u32 = 31;
    pub const CLK_EN_MASK: u32 = 0x80000000;
    pub const CLK_EN: u32 = 1 << 31;

    /// **DATE**
    ///
    /// need_des
    ///
    /// Bits `[30:0]`, mask `0x7fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x7fffffff;

    // ======== Field Accessors ========

    /// **CLK_EN** `[31]` (RW)
    ///
    /// need_des
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

    /// **DATE** `[30:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[30:0]` (RW)
    ///
    /// need_des
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
        Self { bits: 0x02111150 }
    }

    /// Read **CLK_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & Date::CLK_EN_MASK) != 0
    }

    /// Read **DATE** `[30:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **CLK_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Date::CLK_EN_MASK
            } else {
                self.bits & !Date::CLK_EN_MASK
            },
        }
    }

    /// Set **DATE** `[30:0]`
    ///
    /// need_des
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
    /// need_des
    pub tar0_low: Tar0Low,
    /// 0x0004
    /// need_des
    pub tar0_high: Tar0High,
    /// 0x0008
    /// need_des
    pub tar1_low: Tar1Low,
    /// 0x000c
    /// need_des
    pub tar1_high: Tar1High,
    /// 0x0010
    /// need_des
    pub update: Update,
    /// 0x0014
    /// need_des
    pub main_buf0_low: MainBuf0Low,
    /// 0x0018
    /// need_des
    pub main_buf0_high: MainBuf0High,
    /// 0x001c
    /// need_des
    pub main_buf1_low: MainBuf1Low,
    /// 0x0020
    /// need_des
    pub main_buf1_high: MainBuf1High,
    /// 0x0024
    /// need_des
    pub main_overflow: MainOverflow,
    /// 0x0028
    /// need_des
    pub int_raw: IntRaw,
    /// 0x002c
    /// need_des
    pub int_st: IntSt,
    /// 0x0030
    /// need_des
    pub int_ena: IntEna,
    /// 0x0034
    /// need_des
    pub int_clr: IntClr,
    /// 0x0038
    /// need_des
    pub lp_int_raw: LpIntRaw,
    /// 0x003c
    /// need_des
    pub lp_int_st: LpIntSt,
    /// 0x0040
    /// need_des
    pub lp_int_ena: LpIntEna,
    /// 0x0044
    /// need_des
    pub lp_int_clr: LpIntClr,
    _reserved0: [u8; 948],
    /// 0x03fc
    /// need_des
    pub date: Date,
}
