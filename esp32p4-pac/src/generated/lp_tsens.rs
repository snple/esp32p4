//! LP_TSENS
//! Low-power Temperature Sensor

#![allow(dead_code)]

pub const BASE: u32 = 0x5012f000;

/// CTRL Register
///
/// Tsens configuration.
///
/// Address offset: 0x0000
/// Reset value: 0x00019400
///
/// ## Fields
///
/// - **POWER_UP_FORCE** `[23]` (RW)
///
///   1: dump out & power up controlled by SW, 0: by FSM.
///
/// - **POWER_UP** `[22]` (RW)
///
///   Temperature sensor power up.
///
/// - **CLK_DIV** `[21:14]` (RW)
///
///   Temperature sensor clock divider.
///
/// - **IN_INV** `[13]` (RW)
///
///   Invert temperature sensor data.
///
/// - **INT_EN** `[12]` (RW)
///
///   Enable temperature sensor to send out interrupt.
///
/// - **WAKEUP_MASK** `[10]` (RW)
///
///   Wake up signal mask.
///
/// - **SAMPLE_EN** `[9]` (RW)
///
///   Enable sample signal for wakeup module.
///
/// - **READY** `[8]` (RO)
///
///   Indicate temperature sensor out ready.
///
/// - **OUT** `[7:0]` (RO)
///
///   Temperature sensor data out.
#[repr(transparent)]
pub struct Ctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctrl {
    // ======== Field Constants ========

    /// **POWER_UP_FORCE**
    ///
    /// 1: dump out & power up controlled by SW, 0: by FSM.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const POWER_UP_FORCE_OFFSET: u32 = 23;
    pub const POWER_UP_FORCE_MASK: u32 = 0x00800000;
    pub const POWER_UP_FORCE: u32 = 1 << 23;

    /// **POWER_UP**
    ///
    /// Temperature sensor power up.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const POWER_UP_OFFSET: u32 = 22;
    pub const POWER_UP_MASK: u32 = 0x00400000;
    pub const POWER_UP: u32 = 1 << 22;

    /// **CLK_DIV**
    ///
    /// Temperature sensor clock divider.
    ///
    /// Bits `[21:14]`, mask `0x003fc000`
    pub const CLK_DIV_OFFSET: u32 = 14;
    pub const CLK_DIV_MASK: u32 = 0x003fc000;

    /// **IN_INV**
    ///
    /// Invert temperature sensor data.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const IN_INV_OFFSET: u32 = 13;
    pub const IN_INV_MASK: u32 = 0x00002000;
    pub const IN_INV: u32 = 1 << 13;

    /// **INT_EN**
    ///
    /// Enable temperature sensor to send out interrupt.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const INT_EN_OFFSET: u32 = 12;
    pub const INT_EN_MASK: u32 = 0x00001000;
    pub const INT_EN: u32 = 1 << 12;

    /// **WAKEUP_MASK**
    ///
    /// Wake up signal mask.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const WAKEUP_MASK_OFFSET: u32 = 10;
    pub const WAKEUP_MASK_MASK: u32 = 0x00000400;
    pub const WAKEUP_MASK: u32 = 1 << 10;

    /// **SAMPLE_EN**
    ///
    /// Enable sample signal for wakeup module.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SAMPLE_EN_OFFSET: u32 = 9;
    pub const SAMPLE_EN_MASK: u32 = 0x00000200;
    pub const SAMPLE_EN: u32 = 1 << 9;

    /// **READY**
    ///
    /// Indicate temperature sensor out ready.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const READY_OFFSET: u32 = 8;
    pub const READY_MASK: u32 = 0x00000100;
    pub const READY: u32 = 1 << 8;

    /// **OUT**
    ///
    /// Temperature sensor data out.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const OUT_OFFSET: u32 = 0;
    pub const OUT_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **POWER_UP_FORCE** `[23]` (RW)
    ///
    /// 1: dump out & power up controlled by SW, 0: by FSM.
    #[inline(always)]
    pub fn power_up_force(&self) -> bool {
        (self.value.read_bits() & Self::POWER_UP_FORCE_MASK) != 0
    }

    /// Set POWER_UP_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_power_up_force(&self) {
        self.value.modify_bits(|r| r | Self::POWER_UP_FORCE_MASK);
    }

    /// Clear POWER_UP_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_power_up_force(&self) {
        self.value.modify_bits(|r| r & !Self::POWER_UP_FORCE_MASK);
    }

    /// **POWER_UP** `[22]` (RW)
    ///
    /// Temperature sensor power up.
    #[inline(always)]
    pub fn power_up(&self) -> bool {
        (self.value.read_bits() & Self::POWER_UP_MASK) != 0
    }

    /// Set POWER_UP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_power_up(&self) {
        self.value.modify_bits(|r| r | Self::POWER_UP_MASK);
    }

    /// Clear POWER_UP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_power_up(&self) {
        self.value.modify_bits(|r| r & !Self::POWER_UP_MASK);
    }

    /// **CLK_DIV** `[21:14]` (RW)
    ///
    /// Temperature sensor clock divider.
    #[inline(always)]
    pub fn clk_div(&self) -> u32 {
        (self.value.read_bits() & Self::CLK_DIV_MASK) >> Self::CLK_DIV_OFFSET
    }

    /// Write **CLK_DIV** `[21:14]` (RW)
    ///
    /// Temperature sensor clock divider.
    #[inline(always)]
    pub fn write_clk_div(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CLK_DIV_MASK) | ((value << Self::CLK_DIV_OFFSET) & Self::CLK_DIV_MASK)
        });
    }

    /// **IN_INV** `[13]` (RW)
    ///
    /// Invert temperature sensor data.
    #[inline(always)]
    pub fn in_inv(&self) -> bool {
        (self.value.read_bits() & Self::IN_INV_MASK) != 0
    }

    /// Set IN_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_in_inv(&self) {
        self.value.modify_bits(|r| r | Self::IN_INV_MASK);
    }

    /// Clear IN_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_in_inv(&self) {
        self.value.modify_bits(|r| r & !Self::IN_INV_MASK);
    }

    /// **INT_EN** `[12]` (RW)
    ///
    /// Enable temperature sensor to send out interrupt.
    #[inline(always)]
    pub fn int_en(&self) -> bool {
        (self.value.read_bits() & Self::INT_EN_MASK) != 0
    }

    /// Set INT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_int_en(&self) {
        self.value.modify_bits(|r| r | Self::INT_EN_MASK);
    }

    /// Clear INT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_int_en(&self) {
        self.value.modify_bits(|r| r & !Self::INT_EN_MASK);
    }

    /// **WAKEUP_MASK** `[10]` (RW)
    ///
    /// Wake up signal mask.
    #[inline(always)]
    pub fn wakeup_mask(&self) -> bool {
        (self.value.read_bits() & Self::WAKEUP_MASK_MASK) != 0
    }

    /// Set WAKEUP_MASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wakeup_mask(&self) {
        self.value.modify_bits(|r| r | Self::WAKEUP_MASK_MASK);
    }

    /// Clear WAKEUP_MASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wakeup_mask(&self) {
        self.value.modify_bits(|r| r & !Self::WAKEUP_MASK_MASK);
    }

    /// **SAMPLE_EN** `[9]` (RW)
    ///
    /// Enable sample signal for wakeup module.
    #[inline(always)]
    pub fn sample_en(&self) -> bool {
        (self.value.read_bits() & Self::SAMPLE_EN_MASK) != 0
    }

    /// Set SAMPLE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sample_en(&self) {
        self.value.modify_bits(|r| r | Self::SAMPLE_EN_MASK);
    }

    /// Clear SAMPLE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sample_en(&self) {
        self.value.modify_bits(|r| r & !Self::SAMPLE_EN_MASK);
    }

    /// **READY** `[8]` (RO)
    ///
    /// Indicate temperature sensor out ready.
    #[inline(always)]
    pub fn ready(&self) -> bool {
        (self.value.read_bits() & Self::READY_MASK) != 0
    }

    /// **OUT** `[7:0]` (RO)
    ///
    /// Temperature sensor data out.
    #[inline(always)]
    pub fn out(&self) -> u32 {
        (self.value.read_bits() & Self::OUT_MASK) >> Self::OUT_OFFSET
    }
}

/// CTRL value type for batch read/modify/write operations
///
/// Use `ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CtrlVal {
    bits: u32,
}

impl CtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00019400 }
    }

    /// Read **POWER_UP_FORCE** `[23:23]`
    ///
    /// 1: dump out & power up controlled by SW, 0: by FSM.
    #[inline(always)]
    pub const fn power_up_force(&self) -> bool {
        (self.bits & Ctrl::POWER_UP_FORCE_MASK) != 0
    }

    /// Read **POWER_UP** `[22:22]`
    ///
    /// Temperature sensor power up.
    #[inline(always)]
    pub const fn power_up(&self) -> bool {
        (self.bits & Ctrl::POWER_UP_MASK) != 0
    }

    /// Read **CLK_DIV** `[21:14]`
    ///
    /// Temperature sensor clock divider.
    #[inline(always)]
    pub const fn clk_div(&self) -> u32 {
        (self.bits & Ctrl::CLK_DIV_MASK) >> Ctrl::CLK_DIV_OFFSET
    }

    /// Read **IN_INV** `[13:13]`
    ///
    /// Invert temperature sensor data.
    #[inline(always)]
    pub const fn in_inv(&self) -> bool {
        (self.bits & Ctrl::IN_INV_MASK) != 0
    }

    /// Read **INT_EN** `[12:12]`
    ///
    /// Enable temperature sensor to send out interrupt.
    #[inline(always)]
    pub const fn int_en(&self) -> bool {
        (self.bits & Ctrl::INT_EN_MASK) != 0
    }

    /// Read **WAKEUP_MASK** `[10:10]`
    ///
    /// Wake up signal mask.
    #[inline(always)]
    pub const fn wakeup_mask(&self) -> bool {
        (self.bits & Ctrl::WAKEUP_MASK_MASK) != 0
    }

    /// Read **SAMPLE_EN** `[9:9]`
    ///
    /// Enable sample signal for wakeup module.
    #[inline(always)]
    pub const fn sample_en(&self) -> bool {
        (self.bits & Ctrl::SAMPLE_EN_MASK) != 0
    }

    /// Read **READY** `[8:8]`
    ///
    /// Indicate temperature sensor out ready.
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        (self.bits & Ctrl::READY_MASK) != 0
    }

    /// Read **OUT** `[7:0]`
    ///
    /// Temperature sensor data out.
    #[inline(always)]
    pub const fn out(&self) -> u32 {
        (self.bits & Ctrl::OUT_MASK) >> Ctrl::OUT_OFFSET
    }

    /// Set **POWER_UP_FORCE** `[23:23]`
    ///
    /// 1: dump out & power up controlled by SW, 0: by FSM.
    #[inline(always)]
    pub const fn with_power_up_force(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::POWER_UP_FORCE_MASK
            } else {
                self.bits & !Ctrl::POWER_UP_FORCE_MASK
            },
        }
    }

    /// Set **POWER_UP** `[22:22]`
    ///
    /// Temperature sensor power up.
    #[inline(always)]
    pub const fn with_power_up(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::POWER_UP_MASK
            } else {
                self.bits & !Ctrl::POWER_UP_MASK
            },
        }
    }

    /// Set **CLK_DIV** `[21:14]`
    ///
    /// Temperature sensor clock divider.
    #[inline(always)]
    pub const fn with_clk_div(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl::CLK_DIV_MASK)
                | ((value << Ctrl::CLK_DIV_OFFSET) & Ctrl::CLK_DIV_MASK),
        }
    }

    /// Set **IN_INV** `[13:13]`
    ///
    /// Invert temperature sensor data.
    #[inline(always)]
    pub const fn with_in_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::IN_INV_MASK
            } else {
                self.bits & !Ctrl::IN_INV_MASK
            },
        }
    }

    /// Set **INT_EN** `[12:12]`
    ///
    /// Enable temperature sensor to send out interrupt.
    #[inline(always)]
    pub const fn with_int_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::INT_EN_MASK
            } else {
                self.bits & !Ctrl::INT_EN_MASK
            },
        }
    }

    /// Set **WAKEUP_MASK** `[10:10]`
    ///
    /// Wake up signal mask.
    #[inline(always)]
    pub const fn with_wakeup_mask(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::WAKEUP_MASK_MASK
            } else {
                self.bits & !Ctrl::WAKEUP_MASK_MASK
            },
        }
    }

    /// Set **SAMPLE_EN** `[9:9]`
    ///
    /// Enable sample signal for wakeup module.
    #[inline(always)]
    pub const fn with_sample_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl::SAMPLE_EN_MASK
            } else {
                self.bits & !Ctrl::SAMPLE_EN_MASK
            },
        }
    }
}

impl Default for CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CtrlVal {
        CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CtrlVal) -> CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CTRL2 Register
///
/// Tsens configuration.
///
/// Address offset: 0x0004
/// Reset value: 0x00004002
///
/// ## Fields
///
/// - **CLK_INV** `[14]` (RW)
///
///   N/A
///
/// - **XPD_FORCE** `[13:12]` (RW)
///
///   N/A
///
/// - **XPD_WAIT** `[11:0]` (RW)
///
///   N/A
#[repr(transparent)]
pub struct Ctrl2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ctrl2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ctrl2 {
    // ======== Field Constants ========

    /// **CLK_INV**
    ///
    /// N/A
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const CLK_INV_OFFSET: u32 = 14;
    pub const CLK_INV_MASK: u32 = 0x00004000;
    pub const CLK_INV: u32 = 1 << 14;

    /// **XPD_FORCE**
    ///
    /// N/A
    ///
    /// Bits `[13:12]`, mask `0x00003000`
    pub const XPD_FORCE_OFFSET: u32 = 12;
    pub const XPD_FORCE_MASK: u32 = 0x00003000;

    /// **XPD_WAIT**
    ///
    /// N/A
    ///
    /// Bits `[11:0]`, mask `0x00000fff`
    pub const XPD_WAIT_OFFSET: u32 = 0;
    pub const XPD_WAIT_MASK: u32 = 0x00000fff;

    // ======== Field Accessors ========

    /// **CLK_INV** `[14]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn clk_inv(&self) -> bool {
        (self.value.read_bits() & Self::CLK_INV_MASK) != 0
    }

    /// Set CLK_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_clk_inv(&self) {
        self.value.modify_bits(|r| r | Self::CLK_INV_MASK);
    }

    /// Clear CLK_INV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_clk_inv(&self) {
        self.value.modify_bits(|r| r & !Self::CLK_INV_MASK);
    }

    /// **XPD_FORCE** `[13:12]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn xpd_force(&self) -> u32 {
        (self.value.read_bits() & Self::XPD_FORCE_MASK) >> Self::XPD_FORCE_OFFSET
    }

    /// Write **XPD_FORCE** `[13:12]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_xpd_force(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::XPD_FORCE_MASK) | ((value << Self::XPD_FORCE_OFFSET) & Self::XPD_FORCE_MASK)
        });
    }

    /// **XPD_WAIT** `[11:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn xpd_wait(&self) -> u32 {
        (self.value.read_bits() & Self::XPD_WAIT_MASK) >> Self::XPD_WAIT_OFFSET
    }

    /// Write **XPD_WAIT** `[11:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_xpd_wait(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::XPD_WAIT_MASK) | ((value << Self::XPD_WAIT_OFFSET) & Self::XPD_WAIT_MASK)
        });
    }
}

/// CTRL2 value type for batch read/modify/write operations
///
/// Use `ctrl2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ctrl2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ctrl2Val {
    bits: u32,
}

impl Ctrl2Val {
    /// Create from raw bits
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
        Self { bits: 0x00004002 }
    }

    /// Read **CLK_INV** `[14:14]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn clk_inv(&self) -> bool {
        (self.bits & Ctrl2::CLK_INV_MASK) != 0
    }

    /// Read **XPD_FORCE** `[13:12]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn xpd_force(&self) -> u32 {
        (self.bits & Ctrl2::XPD_FORCE_MASK) >> Ctrl2::XPD_FORCE_OFFSET
    }

    /// Read **XPD_WAIT** `[11:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn xpd_wait(&self) -> u32 {
        (self.bits & Ctrl2::XPD_WAIT_MASK) >> Ctrl2::XPD_WAIT_OFFSET
    }

    /// Set **CLK_INV** `[14:14]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_clk_inv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ctrl2::CLK_INV_MASK
            } else {
                self.bits & !Ctrl2::CLK_INV_MASK
            },
        }
    }

    /// Set **XPD_FORCE** `[13:12]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_xpd_force(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl2::XPD_FORCE_MASK)
                | ((value << Ctrl2::XPD_FORCE_OFFSET) & Ctrl2::XPD_FORCE_MASK),
        }
    }

    /// Set **XPD_WAIT** `[11:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_xpd_wait(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Ctrl2::XPD_WAIT_MASK)
                | ((value << Ctrl2::XPD_WAIT_OFFSET) & Ctrl2::XPD_WAIT_MASK),
        }
    }
}

impl Default for Ctrl2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Ctrl2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ctrl2Val {
        Ctrl2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ctrl2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ctrl2Val) -> Ctrl2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_RAW Register
///
/// Tsens interrupt raw registers.
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **COCPU_TSENS_WAKE_INT_RAW** `[0]` (RW)
///
///   Tsens wakeup interrupt raw.
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

    /// **COCPU_TSENS_WAKE_INT_RAW**
    ///
    /// Tsens wakeup interrupt raw.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const COCPU_TSENS_WAKE_INT_RAW_OFFSET: u32 = 0;
    pub const COCPU_TSENS_WAKE_INT_RAW_MASK: u32 = 0x00000001;
    pub const COCPU_TSENS_WAKE_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **COCPU_TSENS_WAKE_INT_RAW** `[0]` (RW)
    ///
    /// Tsens wakeup interrupt raw.
    #[inline(always)]
    pub fn cocpu_tsens_wake_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_TSENS_WAKE_INT_RAW_MASK) != 0
    }

    /// Set COCPU_TSENS_WAKE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cocpu_tsens_wake_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::COCPU_TSENS_WAKE_INT_RAW_MASK);
    }

    /// Clear COCPU_TSENS_WAKE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cocpu_tsens_wake_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::COCPU_TSENS_WAKE_INT_RAW_MASK);
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

    /// Read **COCPU_TSENS_WAKE_INT_RAW** `[0:0]`
    ///
    /// Tsens wakeup interrupt raw.
    #[inline(always)]
    pub const fn cocpu_tsens_wake_int_raw(&self) -> bool {
        (self.bits & IntRaw::COCPU_TSENS_WAKE_INT_RAW_MASK) != 0
    }

    /// Set **COCPU_TSENS_WAKE_INT_RAW** `[0:0]`
    ///
    /// Tsens wakeup interrupt raw.
    #[inline(always)]
    pub const fn with_cocpu_tsens_wake_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::COCPU_TSENS_WAKE_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::COCPU_TSENS_WAKE_INT_RAW_MASK
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
/// Tsens interrupt status registers.
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **COCPU_TSENS_WAKE_INT_ST** `[0]` (RO)
///
///   Tsens wakeup interrupt status.
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

    /// **COCPU_TSENS_WAKE_INT_ST**
    ///
    /// Tsens wakeup interrupt status.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const COCPU_TSENS_WAKE_INT_ST_OFFSET: u32 = 0;
    pub const COCPU_TSENS_WAKE_INT_ST_MASK: u32 = 0x00000001;
    pub const COCPU_TSENS_WAKE_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **COCPU_TSENS_WAKE_INT_ST** `[0]` (RO)
    ///
    /// Tsens wakeup interrupt status.
    #[inline(always)]
    pub fn cocpu_tsens_wake_int_st(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_TSENS_WAKE_INT_ST_MASK) != 0
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

    /// Read **COCPU_TSENS_WAKE_INT_ST** `[0:0]`
    ///
    /// Tsens wakeup interrupt status.
    #[inline(always)]
    pub const fn cocpu_tsens_wake_int_st(&self) -> bool {
        (self.bits & IntSt::COCPU_TSENS_WAKE_INT_ST_MASK) != 0
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
/// Tsens interrupt enable registers.
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **COCPU_TSENS_WAKE_INT_ENA** `[0]` (RW)
///
///   Tsens wakeup interrupt enable.
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

    /// **COCPU_TSENS_WAKE_INT_ENA**
    ///
    /// Tsens wakeup interrupt enable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const COCPU_TSENS_WAKE_INT_ENA_OFFSET: u32 = 0;
    pub const COCPU_TSENS_WAKE_INT_ENA_MASK: u32 = 0x00000001;
    pub const COCPU_TSENS_WAKE_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **COCPU_TSENS_WAKE_INT_ENA** `[0]` (RW)
    ///
    /// Tsens wakeup interrupt enable.
    #[inline(always)]
    pub fn cocpu_tsens_wake_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::COCPU_TSENS_WAKE_INT_ENA_MASK) != 0
    }

    /// Set COCPU_TSENS_WAKE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cocpu_tsens_wake_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::COCPU_TSENS_WAKE_INT_ENA_MASK);
    }

    /// Clear COCPU_TSENS_WAKE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cocpu_tsens_wake_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::COCPU_TSENS_WAKE_INT_ENA_MASK);
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

    /// Read **COCPU_TSENS_WAKE_INT_ENA** `[0:0]`
    ///
    /// Tsens wakeup interrupt enable.
    #[inline(always)]
    pub const fn cocpu_tsens_wake_int_ena(&self) -> bool {
        (self.bits & IntEna::COCPU_TSENS_WAKE_INT_ENA_MASK) != 0
    }

    /// Set **COCPU_TSENS_WAKE_INT_ENA** `[0:0]`
    ///
    /// Tsens wakeup interrupt enable.
    #[inline(always)]
    pub const fn with_cocpu_tsens_wake_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::COCPU_TSENS_WAKE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::COCPU_TSENS_WAKE_INT_ENA_MASK
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
/// Tsens interrupt clear registers.
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **COCPU_TSENS_WAKE_INT_CLR** `[0]` (WO)
///
///   Tsens wakeup interrupt clear.
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

    /// **COCPU_TSENS_WAKE_INT_CLR**
    ///
    /// Tsens wakeup interrupt clear.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const COCPU_TSENS_WAKE_INT_CLR_OFFSET: u32 = 0;
    pub const COCPU_TSENS_WAKE_INT_CLR_MASK: u32 = 0x00000001;
    pub const COCPU_TSENS_WAKE_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **COCPU_TSENS_WAKE_INT_CLR** (WO)
    ///
    /// Tsens wakeup interrupt clear.
    #[inline(always)]
    pub fn trigger_cocpu_tsens_wake_int_clr(&self) {
        self.value.write_bits(Self::COCPU_TSENS_WAKE_INT_CLR_MASK);
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

    /// Set **COCPU_TSENS_WAKE_INT_CLR** `[0:0]`
    ///
    /// Tsens wakeup interrupt clear.
    #[inline(always)]
    pub const fn with_cocpu_tsens_wake_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::COCPU_TSENS_WAKE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::COCPU_TSENS_WAKE_INT_CLR_MASK
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

/// CLK_CONF Register
///
/// Tsens regbank configuration registers.
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **CLK_EN** `[0]` (RW)
///
///   Tsens regbank clock gating enable.
#[repr(transparent)]
pub struct ClkConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClkConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkConf {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// Tsens regbank clock gating enable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_EN_OFFSET: u32 = 0;
    pub const CLK_EN_MASK: u32 = 0x00000001;
    pub const CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[0]` (RW)
    ///
    /// Tsens regbank clock gating enable.
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

/// CLK_CONF value type for batch read/modify/write operations
///
/// Use `clkconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkConfVal {
    bits: u32,
}

impl ClkConfVal {
    /// Create from raw bits
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
    /// Tsens regbank clock gating enable.
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & ClkConf::CLK_EN_MASK) != 0
    }

    /// Set **CLK_EN** `[0:0]`
    ///
    /// Tsens regbank clock gating enable.
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkConf::CLK_EN_MASK
            } else {
                self.bits & !ClkConf::CLK_EN_MASK
            },
        }
    }
}

impl Default for ClkConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ClkConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkConfVal {
        ClkConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkConfVal) -> ClkConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_ENA_W1TS Register
///
/// Tsens wakeup interrupt enable assert.
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **COCPU_TSENS_WAKE_INT_ENA_W1TS** `[0]` (WO)
///
///   Write 1 to this field to assert interrupt enable.
#[repr(transparent)]
pub struct IntEnaW1ts {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntEnaW1ts {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntEnaW1ts {
    // ======== Field Constants ========

    /// **COCPU_TSENS_WAKE_INT_ENA_W1TS**
    ///
    /// Write 1 to this field to assert interrupt enable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const COCPU_TSENS_WAKE_INT_ENA_W1TS_OFFSET: u32 = 0;
    pub const COCPU_TSENS_WAKE_INT_ENA_W1TS_MASK: u32 = 0x00000001;
    pub const COCPU_TSENS_WAKE_INT_ENA_W1TS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **COCPU_TSENS_WAKE_INT_ENA_W1TS** (WO)
    ///
    /// Write 1 to this field to assert interrupt enable.
    #[inline(always)]
    pub fn trigger_cocpu_tsens_wake_int_ena_w1ts(&self) {
        self.value
            .write_bits(Self::COCPU_TSENS_WAKE_INT_ENA_W1TS_MASK);
    }
}

/// INT_ENA_W1TS value type for batch read/modify/write operations
///
/// Use `intenaw1ts.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intenaw1ts.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntEnaW1tsVal {
    bits: u32,
}

impl IntEnaW1tsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **COCPU_TSENS_WAKE_INT_ENA_W1TS** `[0:0]`
    ///
    /// Write 1 to this field to assert interrupt enable.
    #[inline(always)]
    pub const fn with_cocpu_tsens_wake_int_ena_w1ts(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaW1ts::COCPU_TSENS_WAKE_INT_ENA_W1TS_MASK
            } else {
                self.bits & !IntEnaW1ts::COCPU_TSENS_WAKE_INT_ENA_W1TS_MASK
            },
        }
    }
}

impl Default for IntEnaW1tsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntEnaW1ts {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntEnaW1tsVal) {
        self.value.write_bits(val.bits())
    }
}

/// INT_ENA_W1TC Register
///
/// Tsens wakeup interrupt enable deassert.
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **COCPU_TSENS_WAKE_INT_ENA_W1TC** `[0]` (WO)
///
///   Write 1 to this field to deassert interrupt enable.
#[repr(transparent)]
pub struct IntEnaW1tc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IntEnaW1tc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IntEnaW1tc {
    // ======== Field Constants ========

    /// **COCPU_TSENS_WAKE_INT_ENA_W1TC**
    ///
    /// Write 1 to this field to deassert interrupt enable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const COCPU_TSENS_WAKE_INT_ENA_W1TC_OFFSET: u32 = 0;
    pub const COCPU_TSENS_WAKE_INT_ENA_W1TC_MASK: u32 = 0x00000001;
    pub const COCPU_TSENS_WAKE_INT_ENA_W1TC: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **COCPU_TSENS_WAKE_INT_ENA_W1TC** (WO)
    ///
    /// Write 1 to this field to deassert interrupt enable.
    #[inline(always)]
    pub fn trigger_cocpu_tsens_wake_int_ena_w1tc(&self) {
        self.value
            .write_bits(Self::COCPU_TSENS_WAKE_INT_ENA_W1TC_MASK);
    }
}

/// INT_ENA_W1TC value type for batch read/modify/write operations
///
/// Use `intenaw1tc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `intenaw1tc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IntEnaW1tcVal {
    bits: u32,
}

impl IntEnaW1tcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **COCPU_TSENS_WAKE_INT_ENA_W1TC** `[0:0]`
    ///
    /// Write 1 to this field to deassert interrupt enable.
    #[inline(always)]
    pub const fn with_cocpu_tsens_wake_int_ena_w1tc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEnaW1tc::COCPU_TSENS_WAKE_INT_ENA_W1TC_MASK
            } else {
                self.bits & !IntEnaW1tc::COCPU_TSENS_WAKE_INT_ENA_W1TC_MASK
            },
        }
    }
}

impl Default for IntEnaW1tcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IntEnaW1tc {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IntEnaW1tcVal) {
        self.value.write_bits(val.bits())
    }
}

/// WAKEUP_CTRL Register
///
/// Tsens wakeup control registers.
///
/// Address offset: 0x0024
/// Reset value: 0x003fc000
///
/// ## Fields
///
/// - **WAKEUP_MODE** `[31]` (RW)
///
///   0:absolute value comparison mode. 1: relative value comparison mode.
///
/// - **WAKEUP_EN** `[30]` (RW)
///
///   Tsens wakeup enable.
///
/// - **WAKEUP_OVER_UPPER_TH** `[29]` (RO)
///
///   Indicates that this wakeup event arose from exceeding upper threshold.
///
/// - **WAKEUP_TH_HIGH** `[21:14]` (RW)
///
///   Upper threshold.
///
/// - **WAKEUP_TH_LOW** `[7:0]` (RW)
///
///   Lower threshold.
#[repr(transparent)]
pub struct WakeupCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for WakeupCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WakeupCtrl {
    // ======== Field Constants ========

    /// **WAKEUP_MODE**
    ///
    /// 0:absolute value comparison mode. 1: relative value comparison mode.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const WAKEUP_MODE_OFFSET: u32 = 31;
    pub const WAKEUP_MODE_MASK: u32 = 0x80000000;
    pub const WAKEUP_MODE: u32 = 1 << 31;

    /// **WAKEUP_EN**
    ///
    /// Tsens wakeup enable.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const WAKEUP_EN_OFFSET: u32 = 30;
    pub const WAKEUP_EN_MASK: u32 = 0x40000000;
    pub const WAKEUP_EN: u32 = 1 << 30;

    /// **WAKEUP_OVER_UPPER_TH**
    ///
    /// Indicates that this wakeup event arose from exceeding upper threshold.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const WAKEUP_OVER_UPPER_TH_OFFSET: u32 = 29;
    pub const WAKEUP_OVER_UPPER_TH_MASK: u32 = 0x20000000;
    pub const WAKEUP_OVER_UPPER_TH: u32 = 1 << 29;

    /// **WAKEUP_TH_HIGH**
    ///
    /// Upper threshold.
    ///
    /// Bits `[21:14]`, mask `0x003fc000`
    pub const WAKEUP_TH_HIGH_OFFSET: u32 = 14;
    pub const WAKEUP_TH_HIGH_MASK: u32 = 0x003fc000;

    /// **WAKEUP_TH_LOW**
    ///
    /// Lower threshold.
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const WAKEUP_TH_LOW_OFFSET: u32 = 0;
    pub const WAKEUP_TH_LOW_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **WAKEUP_MODE** `[31]` (RW)
    ///
    /// 0:absolute value comparison mode. 1: relative value comparison mode.
    #[inline(always)]
    pub fn wakeup_mode(&self) -> bool {
        (self.value.read_bits() & Self::WAKEUP_MODE_MASK) != 0
    }

    /// Set WAKEUP_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wakeup_mode(&self) {
        self.value.modify_bits(|r| r | Self::WAKEUP_MODE_MASK);
    }

    /// Clear WAKEUP_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wakeup_mode(&self) {
        self.value.modify_bits(|r| r & !Self::WAKEUP_MODE_MASK);
    }

    /// **WAKEUP_EN** `[30]` (RW)
    ///
    /// Tsens wakeup enable.
    #[inline(always)]
    pub fn wakeup_en(&self) -> bool {
        (self.value.read_bits() & Self::WAKEUP_EN_MASK) != 0
    }

    /// Set WAKEUP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_wakeup_en(&self) {
        self.value.modify_bits(|r| r | Self::WAKEUP_EN_MASK);
    }

    /// Clear WAKEUP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_wakeup_en(&self) {
        self.value.modify_bits(|r| r & !Self::WAKEUP_EN_MASK);
    }

    /// **WAKEUP_OVER_UPPER_TH** `[29]` (RO)
    ///
    /// Indicates that this wakeup event arose from exceeding upper threshold.
    #[inline(always)]
    pub fn wakeup_over_upper_th(&self) -> bool {
        (self.value.read_bits() & Self::WAKEUP_OVER_UPPER_TH_MASK) != 0
    }

    /// **WAKEUP_TH_HIGH** `[21:14]` (RW)
    ///
    /// Upper threshold.
    #[inline(always)]
    pub fn wakeup_th_high(&self) -> u32 {
        (self.value.read_bits() & Self::WAKEUP_TH_HIGH_MASK) >> Self::WAKEUP_TH_HIGH_OFFSET
    }

    /// Write **WAKEUP_TH_HIGH** `[21:14]` (RW)
    ///
    /// Upper threshold.
    #[inline(always)]
    pub fn write_wakeup_th_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WAKEUP_TH_HIGH_MASK)
                | ((value << Self::WAKEUP_TH_HIGH_OFFSET) & Self::WAKEUP_TH_HIGH_MASK)
        });
    }

    /// **WAKEUP_TH_LOW** `[7:0]` (RW)
    ///
    /// Lower threshold.
    #[inline(always)]
    pub fn wakeup_th_low(&self) -> u32 {
        (self.value.read_bits() & Self::WAKEUP_TH_LOW_MASK) >> Self::WAKEUP_TH_LOW_OFFSET
    }

    /// Write **WAKEUP_TH_LOW** `[7:0]` (RW)
    ///
    /// Lower threshold.
    #[inline(always)]
    pub fn write_wakeup_th_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WAKEUP_TH_LOW_MASK)
                | ((value << Self::WAKEUP_TH_LOW_OFFSET) & Self::WAKEUP_TH_LOW_MASK)
        });
    }
}

/// WAKEUP_CTRL value type for batch read/modify/write operations
///
/// Use `wakeupctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `wakeupctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct WakeupCtrlVal {
    bits: u32,
}

impl WakeupCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x003fc000 }
    }

    /// Read **WAKEUP_MODE** `[31:31]`
    ///
    /// 0:absolute value comparison mode. 1: relative value comparison mode.
    #[inline(always)]
    pub const fn wakeup_mode(&self) -> bool {
        (self.bits & WakeupCtrl::WAKEUP_MODE_MASK) != 0
    }

    /// Read **WAKEUP_EN** `[30:30]`
    ///
    /// Tsens wakeup enable.
    #[inline(always)]
    pub const fn wakeup_en(&self) -> bool {
        (self.bits & WakeupCtrl::WAKEUP_EN_MASK) != 0
    }

    /// Read **WAKEUP_OVER_UPPER_TH** `[29:29]`
    ///
    /// Indicates that this wakeup event arose from exceeding upper threshold.
    #[inline(always)]
    pub const fn wakeup_over_upper_th(&self) -> bool {
        (self.bits & WakeupCtrl::WAKEUP_OVER_UPPER_TH_MASK) != 0
    }

    /// Read **WAKEUP_TH_HIGH** `[21:14]`
    ///
    /// Upper threshold.
    #[inline(always)]
    pub const fn wakeup_th_high(&self) -> u32 {
        (self.bits & WakeupCtrl::WAKEUP_TH_HIGH_MASK) >> WakeupCtrl::WAKEUP_TH_HIGH_OFFSET
    }

    /// Read **WAKEUP_TH_LOW** `[7:0]`
    ///
    /// Lower threshold.
    #[inline(always)]
    pub const fn wakeup_th_low(&self) -> u32 {
        (self.bits & WakeupCtrl::WAKEUP_TH_LOW_MASK) >> WakeupCtrl::WAKEUP_TH_LOW_OFFSET
    }

    /// Set **WAKEUP_MODE** `[31:31]`
    ///
    /// 0:absolute value comparison mode. 1: relative value comparison mode.
    #[inline(always)]
    pub const fn with_wakeup_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeupCtrl::WAKEUP_MODE_MASK
            } else {
                self.bits & !WakeupCtrl::WAKEUP_MODE_MASK
            },
        }
    }

    /// Set **WAKEUP_EN** `[30:30]`
    ///
    /// Tsens wakeup enable.
    #[inline(always)]
    pub const fn with_wakeup_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | WakeupCtrl::WAKEUP_EN_MASK
            } else {
                self.bits & !WakeupCtrl::WAKEUP_EN_MASK
            },
        }
    }

    /// Set **WAKEUP_TH_HIGH** `[21:14]`
    ///
    /// Upper threshold.
    #[inline(always)]
    pub const fn with_wakeup_th_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !WakeupCtrl::WAKEUP_TH_HIGH_MASK)
                | ((value << WakeupCtrl::WAKEUP_TH_HIGH_OFFSET) & WakeupCtrl::WAKEUP_TH_HIGH_MASK),
        }
    }

    /// Set **WAKEUP_TH_LOW** `[7:0]`
    ///
    /// Lower threshold.
    #[inline(always)]
    pub const fn with_wakeup_th_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !WakeupCtrl::WAKEUP_TH_LOW_MASK)
                | ((value << WakeupCtrl::WAKEUP_TH_LOW_OFFSET) & WakeupCtrl::WAKEUP_TH_LOW_MASK),
        }
    }
}

impl Default for WakeupCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl WakeupCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> WakeupCtrlVal {
        WakeupCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: WakeupCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(WakeupCtrlVal) -> WakeupCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SAMPLE_RATE Register
///
/// Hardware automatic sampling control registers.
///
/// Address offset: 0x0028
/// Reset value: 0x00000014
///
/// ## Fields
///
/// - **SAMPLE_RATE** `[15:0]` (RW)
///
///   Hardware automatic sampling rate.
#[repr(transparent)]
pub struct SampleRate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SampleRate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SampleRate {
    // ======== Field Constants ========

    /// **SAMPLE_RATE**
    ///
    /// Hardware automatic sampling rate.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SAMPLE_RATE_OFFSET: u32 = 0;
    pub const SAMPLE_RATE_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **SAMPLE_RATE** `[15:0]` (RW)
    ///
    /// Hardware automatic sampling rate.
    #[inline(always)]
    pub fn sample_rate(&self) -> u32 {
        (self.value.read_bits() & Self::SAMPLE_RATE_MASK) >> Self::SAMPLE_RATE_OFFSET
    }

    /// Write **SAMPLE_RATE** `[15:0]` (RW)
    ///
    /// Hardware automatic sampling rate.
    #[inline(always)]
    pub fn write_sample_rate(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAMPLE_RATE_MASK)
                | ((value << Self::SAMPLE_RATE_OFFSET) & Self::SAMPLE_RATE_MASK)
        });
    }
}

/// SAMPLE_RATE value type for batch read/modify/write operations
///
/// Use `samplerate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `samplerate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SampleRateVal {
    bits: u32,
}

impl SampleRateVal {
    /// Create from raw bits
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
        Self { bits: 0x00000014 }
    }

    /// Read **SAMPLE_RATE** `[15:0]`
    ///
    /// Hardware automatic sampling rate.
    #[inline(always)]
    pub const fn sample_rate(&self) -> u32 {
        (self.bits & SampleRate::SAMPLE_RATE_MASK) >> SampleRate::SAMPLE_RATE_OFFSET
    }

    /// Set **SAMPLE_RATE** `[15:0]`
    ///
    /// Hardware automatic sampling rate.
    #[inline(always)]
    pub const fn with_sample_rate(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SampleRate::SAMPLE_RATE_MASK)
                | ((value << SampleRate::SAMPLE_RATE_OFFSET) & SampleRate::SAMPLE_RATE_MASK),
        }
    }
}

impl Default for SampleRateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SampleRate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SampleRateVal {
        SampleRateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SampleRateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SampleRateVal) -> SampleRateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RND_ECO_LOW Register
///
/// N/A
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **RND_ECO_LOW** `[31:0]` (RW)
///
///   N/A
#[repr(transparent)]
pub struct RndEcoLow {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RndEcoLow {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RndEcoLow {
    // ======== Field Constants ========

    /// **RND_ECO_LOW**
    ///
    /// N/A
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RND_ECO_LOW_OFFSET: u32 = 0;
    pub const RND_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RND_ECO_LOW** `[31:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn rnd_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::RND_ECO_LOW_MASK) >> Self::RND_ECO_LOW_OFFSET
    }

    /// Write **RND_ECO_LOW** `[31:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_rnd_eco_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RND_ECO_LOW_MASK)
                | ((value << Self::RND_ECO_LOW_OFFSET) & Self::RND_ECO_LOW_MASK)
        });
    }
}

/// RND_ECO_LOW value type for batch read/modify/write operations
///
/// Use `rndecolow.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rndecolow.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RndEcoLowVal {
    bits: u32,
}

impl RndEcoLowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RND_ECO_LOW** `[31:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn rnd_eco_low(&self) -> u32 {
        (self.bits & RndEcoLow::RND_ECO_LOW_MASK) >> RndEcoLow::RND_ECO_LOW_OFFSET
    }

    /// Set **RND_ECO_LOW** `[31:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_rnd_eco_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RndEcoLow::RND_ECO_LOW_MASK)
                | ((value << RndEcoLow::RND_ECO_LOW_OFFSET) & RndEcoLow::RND_ECO_LOW_MASK),
        }
    }
}

impl Default for RndEcoLowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RndEcoLow {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RndEcoLowVal {
        RndEcoLowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RndEcoLowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RndEcoLowVal) -> RndEcoLowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RND_ECO_HIGH Register
///
/// N/A
///
/// Address offset: 0x0030
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **RND_ECO_HIGH** `[31:0]` (RW)
///
///   N/A
#[repr(transparent)]
pub struct RndEcoHigh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RndEcoHigh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RndEcoHigh {
    // ======== Field Constants ========

    /// **RND_ECO_HIGH**
    ///
    /// N/A
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RND_ECO_HIGH_OFFSET: u32 = 0;
    pub const RND_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RND_ECO_HIGH** `[31:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn rnd_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::RND_ECO_HIGH_MASK) >> Self::RND_ECO_HIGH_OFFSET
    }

    /// Write **RND_ECO_HIGH** `[31:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_rnd_eco_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RND_ECO_HIGH_MASK)
                | ((value << Self::RND_ECO_HIGH_OFFSET) & Self::RND_ECO_HIGH_MASK)
        });
    }
}

/// RND_ECO_HIGH value type for batch read/modify/write operations
///
/// Use `rndecohigh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rndecohigh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RndEcoHighVal {
    bits: u32,
}

impl RndEcoHighVal {
    /// Create from raw bits
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
        Self { bits: 0xffffffff }
    }

    /// Read **RND_ECO_HIGH** `[31:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn rnd_eco_high(&self) -> u32 {
        (self.bits & RndEcoHigh::RND_ECO_HIGH_MASK) >> RndEcoHigh::RND_ECO_HIGH_OFFSET
    }

    /// Set **RND_ECO_HIGH** `[31:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_rnd_eco_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RndEcoHigh::RND_ECO_HIGH_MASK)
                | ((value << RndEcoHigh::RND_ECO_HIGH_OFFSET) & RndEcoHigh::RND_ECO_HIGH_MASK),
        }
    }
}

impl Default for RndEcoHighVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RndEcoHigh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RndEcoHighVal {
        RndEcoHighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RndEcoHighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RndEcoHighVal) -> RndEcoHighVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RND_ECO_CS Register
///
/// N/A
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **RND_ECO_RESULT** `[1]` (RO)
///
///   N/A
///
/// - **RND_ECO_EN** `[0]` (RW)
///
///   N/A
#[repr(transparent)]
pub struct RndEcoCs {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RndEcoCs {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RndEcoCs {
    // ======== Field Constants ========

    /// **RND_ECO_RESULT**
    ///
    /// N/A
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RND_ECO_RESULT_OFFSET: u32 = 1;
    pub const RND_ECO_RESULT_MASK: u32 = 0x00000002;
    pub const RND_ECO_RESULT: u32 = 1 << 1;

    /// **RND_ECO_EN**
    ///
    /// N/A
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RND_ECO_EN_OFFSET: u32 = 0;
    pub const RND_ECO_EN_MASK: u32 = 0x00000001;
    pub const RND_ECO_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RND_ECO_RESULT** `[1]` (RO)
    ///
    /// N/A
    #[inline(always)]
    pub fn rnd_eco_result(&self) -> bool {
        (self.value.read_bits() & Self::RND_ECO_RESULT_MASK) != 0
    }

    /// **RND_ECO_EN** `[0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn rnd_eco_en(&self) -> bool {
        (self.value.read_bits() & Self::RND_ECO_EN_MASK) != 0
    }

    /// Set RND_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rnd_eco_en(&self) {
        self.value.modify_bits(|r| r | Self::RND_ECO_EN_MASK);
    }

    /// Clear RND_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rnd_eco_en(&self) {
        self.value.modify_bits(|r| r & !Self::RND_ECO_EN_MASK);
    }
}

/// RND_ECO_CS value type for batch read/modify/write operations
///
/// Use `rndecocs.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rndecocs.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RndEcoCsVal {
    bits: u32,
}

impl RndEcoCsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RND_ECO_RESULT** `[1:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn rnd_eco_result(&self) -> bool {
        (self.bits & RndEcoCs::RND_ECO_RESULT_MASK) != 0
    }

    /// Read **RND_ECO_EN** `[0:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn rnd_eco_en(&self) -> bool {
        (self.bits & RndEcoCs::RND_ECO_EN_MASK) != 0
    }

    /// Set **RND_ECO_EN** `[0:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_rnd_eco_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RndEcoCs::RND_ECO_EN_MASK
            } else {
                self.bits & !RndEcoCs::RND_ECO_EN_MASK
            },
        }
    }
}

impl Default for RndEcoCsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RndEcoCs {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RndEcoCsVal {
        RndEcoCsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RndEcoCsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RndEcoCsVal) -> RndEcoCsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Tsens configuration.
    pub ctrl: Ctrl,
    /// 0x0004
    /// Tsens configuration.
    pub ctrl2: Ctrl2,
    /// 0x0008
    /// Tsens interrupt raw registers.
    pub int_raw: IntRaw,
    /// 0x000c
    /// Tsens interrupt status registers.
    pub int_st: IntSt,
    /// 0x0010
    /// Tsens interrupt enable registers.
    pub int_ena: IntEna,
    /// 0x0014
    /// Tsens interrupt clear registers.
    pub int_clr: IntClr,
    /// 0x0018
    /// Tsens regbank configuration registers.
    pub clk_conf: ClkConf,
    /// 0x001c
    /// Tsens wakeup interrupt enable assert.
    pub int_ena_w1ts: IntEnaW1ts,
    /// 0x0020
    /// Tsens wakeup interrupt enable deassert.
    pub int_ena_w1tc: IntEnaW1tc,
    /// 0x0024
    /// Tsens wakeup control registers.
    pub wakeup_ctrl: WakeupCtrl,
    /// 0x0028
    /// Hardware automatic sampling control registers.
    pub sample_rate: SampleRate,
    /// 0x002c
    /// N/A
    pub rnd_eco_low: RndEcoLow,
    /// 0x0030
    /// N/A
    pub rnd_eco_high: RndEcoHigh,
    /// 0x0034
    /// N/A
    pub rnd_eco_cs: RndEcoCs,
}
