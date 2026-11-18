//! LP_IO_MUX
//! Low-power Input/Output Multiplexer

#![allow(dead_code)]

pub const BASE: u32 = 0x5012b000;

/// CLK_EN Register
///
/// Reserved
///
/// Address offset: 0x0000
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **REG_CLK_EN** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct ClkEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClkEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkEn {
    // ======== Field Constants ========

    /// **REG_CLK_EN**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_CLK_EN_OFFSET: u32 = 0;
    pub const REG_CLK_EN_MASK: u32 = 0x00000001;
    pub const REG_CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_CLK_EN** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_CLK_EN_MASK) != 0
    }

    /// Set REG_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::REG_CLK_EN_MASK);
    }

    /// Clear REG_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::REG_CLK_EN_MASK);
    }
}

/// CLK_EN value type for batch read/modify/write operations
///
/// Use `clken.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clken.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkEnVal {
    bits: u32,
}

impl ClkEnVal {
    /// Create from raw bits
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

    /// Read **REG_CLK_EN** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_clk_en(&self) -> bool {
        (self.bits & ClkEn::REG_CLK_EN_MASK) != 0
    }

    /// Set **REG_CLK_EN** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkEn::REG_CLK_EN_MASK
            } else {
                self.bits & !ClkEn::REG_CLK_EN_MASK
            },
        }
    }
}

impl Default for ClkEnVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl ClkEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkEnVal {
        ClkEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkEnVal) -> ClkEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// VER_DATE Register
///
/// Reserved
///
/// Address offset: 0x0004
/// Reset value: 0x00230313
///
/// ## Fields
///
/// - **REG_VER_DATE** `[27:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct VerDate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VerDate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VerDate {
    // ======== Field Constants ========

    /// **REG_VER_DATE**
    ///
    /// Reserved
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const REG_VER_DATE_OFFSET: u32 = 0;
    pub const REG_VER_DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **REG_VER_DATE** `[27:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_ver_date(&self) -> u32 {
        (self.value.read_bits() & Self::REG_VER_DATE_MASK) >> Self::REG_VER_DATE_OFFSET
    }

    /// Write **REG_VER_DATE** `[27:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_ver_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_VER_DATE_MASK)
                | ((value << Self::REG_VER_DATE_OFFSET) & Self::REG_VER_DATE_MASK)
        });
    }
}

/// VER_DATE value type for batch read/modify/write operations
///
/// Use `verdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `verdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VerDateVal {
    bits: u32,
}

impl VerDateVal {
    /// Create from raw bits
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
        Self { bits: 0x00230313 }
    }

    /// Read **REG_VER_DATE** `[27:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_ver_date(&self) -> u32 {
        (self.bits & VerDate::REG_VER_DATE_MASK) >> VerDate::REG_VER_DATE_OFFSET
    }

    /// Set **REG_VER_DATE** `[27:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_ver_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !VerDate::REG_VER_DATE_MASK)
                | ((value << VerDate::REG_VER_DATE_OFFSET) & VerDate::REG_VER_DATE_MASK),
        }
    }
}

impl Default for VerDateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl VerDate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VerDateVal {
        VerDateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VerDateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VerDateVal) -> VerDateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD0 Register
///
/// Reserved
///
/// Address offset: 0x0008
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD0_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD0_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD0_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD0_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD0_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD0_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD0_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD0_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD0_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD0_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad0 {
    // ======== Field Constants ========

    /// **REG_PAD0_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD0_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD0_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD0_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD0_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD0_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD0_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD0_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD0_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD0_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD0_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD0_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD0_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD0_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD0_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD0_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD0_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD0_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD0_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD0_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD0_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD0_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD0_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD0_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD0_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD0_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD0_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD0_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD0_RUE_OFFSET: u32 = 3;
    pub const REG_PAD0_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD0_RUE: u32 = 1 << 3;

    /// **REG_PAD0_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD0_RDE_OFFSET: u32 = 2;
    pub const REG_PAD0_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD0_RDE: u32 = 1 << 2;

    /// **REG_PAD0_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD0_DRV_OFFSET: u32 = 0;
    pub const REG_PAD0_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD0_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad0_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD0_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD0_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad0_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD0_FILTER_EN_MASK);
    }

    /// Clear REG_PAD0_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad0_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD0_FILTER_EN_MASK);
    }

    /// **REG_PAD0_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad0_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD0_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD0_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad0_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD0_FUN_IE_MASK);
    }

    /// Clear REG_PAD0_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad0_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD0_FUN_IE_MASK);
    }

    /// **REG_PAD0_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad0_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD0_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD0_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad0_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD0_SLP_OE_MASK);
    }

    /// Clear REG_PAD0_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad0_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD0_SLP_OE_MASK);
    }

    /// **REG_PAD0_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad0_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD0_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD0_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad0_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD0_SLP_IE_MASK);
    }

    /// Clear REG_PAD0_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad0_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD0_SLP_IE_MASK);
    }

    /// **REG_PAD0_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad0_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD0_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD0_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad0_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD0_SLP_SEL_MASK);
    }

    /// Clear REG_PAD0_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad0_slp_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD0_SLP_SEL_MASK);
    }

    /// **REG_PAD0_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad0_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD0_FUN_SEL_MASK) >> Self::REG_PAD0_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD0_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad0_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD0_FUN_SEL_MASK)
                | ((value << Self::REG_PAD0_FUN_SEL_OFFSET) & Self::REG_PAD0_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD0_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad0_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD0_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD0_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad0_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD0_MUX_SEL_MASK);
    }

    /// Clear REG_PAD0_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad0_mux_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD0_MUX_SEL_MASK);
    }

    /// **REG_PAD0_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad0_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD0_RUE_MASK) != 0
    }

    /// Set REG_PAD0_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad0_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD0_RUE_MASK);
    }

    /// Clear REG_PAD0_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad0_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD0_RUE_MASK);
    }

    /// **REG_PAD0_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad0_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD0_RDE_MASK) != 0
    }

    /// Set REG_PAD0_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad0_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD0_RDE_MASK);
    }

    /// Clear REG_PAD0_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad0_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD0_RDE_MASK);
    }

    /// **REG_PAD0_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad0_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD0_DRV_MASK) >> Self::REG_PAD0_DRV_OFFSET
    }

    /// Write **REG_PAD0_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad0_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD0_DRV_MASK)
                | ((value << Self::REG_PAD0_DRV_OFFSET) & Self::REG_PAD0_DRV_MASK)
        });
    }
}

/// PAD0 value type for batch read/modify/write operations
///
/// Use `pad0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad0Val {
    bits: u32,
}

impl Pad0Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD0_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad0_filter_en(&self) -> bool {
        (self.bits & Pad0::REG_PAD0_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD0_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad0_fun_ie(&self) -> bool {
        (self.bits & Pad0::REG_PAD0_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD0_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad0_slp_oe(&self) -> bool {
        (self.bits & Pad0::REG_PAD0_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD0_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad0_slp_ie(&self) -> bool {
        (self.bits & Pad0::REG_PAD0_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD0_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad0_slp_sel(&self) -> bool {
        (self.bits & Pad0::REG_PAD0_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD0_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad0_fun_sel(&self) -> u32 {
        (self.bits & Pad0::REG_PAD0_FUN_SEL_MASK) >> Pad0::REG_PAD0_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD0_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad0_mux_sel(&self) -> bool {
        (self.bits & Pad0::REG_PAD0_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD0_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad0_rue(&self) -> bool {
        (self.bits & Pad0::REG_PAD0_RUE_MASK) != 0
    }

    /// Read **REG_PAD0_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad0_rde(&self) -> bool {
        (self.bits & Pad0::REG_PAD0_RDE_MASK) != 0
    }

    /// Read **REG_PAD0_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad0_drv(&self) -> u32 {
        (self.bits & Pad0::REG_PAD0_DRV_MASK) >> Pad0::REG_PAD0_DRV_OFFSET
    }

    /// Set **REG_PAD0_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad0_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad0::REG_PAD0_FILTER_EN_MASK
            } else {
                self.bits & !Pad0::REG_PAD0_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD0_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad0_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad0::REG_PAD0_FUN_IE_MASK
            } else {
                self.bits & !Pad0::REG_PAD0_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD0_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad0_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad0::REG_PAD0_SLP_OE_MASK
            } else {
                self.bits & !Pad0::REG_PAD0_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD0_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad0_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad0::REG_PAD0_SLP_IE_MASK
            } else {
                self.bits & !Pad0::REG_PAD0_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD0_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad0_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad0::REG_PAD0_SLP_SEL_MASK
            } else {
                self.bits & !Pad0::REG_PAD0_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD0_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad0_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad0::REG_PAD0_FUN_SEL_MASK)
                | ((value << Pad0::REG_PAD0_FUN_SEL_OFFSET) & Pad0::REG_PAD0_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD0_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad0_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad0::REG_PAD0_MUX_SEL_MASK
            } else {
                self.bits & !Pad0::REG_PAD0_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD0_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad0_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad0::REG_PAD0_RUE_MASK
            } else {
                self.bits & !Pad0::REG_PAD0_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD0_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad0_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad0::REG_PAD0_RDE_MASK
            } else {
                self.bits & !Pad0::REG_PAD0_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD0_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad0_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad0::REG_PAD0_DRV_MASK)
                | ((value << Pad0::REG_PAD0_DRV_OFFSET) & Pad0::REG_PAD0_DRV_MASK),
        }
    }
}

impl Default for Pad0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad0Val {
        Pad0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad0Val) -> Pad0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD1 Register
///
/// Reserved
///
/// Address offset: 0x000c
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD1_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD1_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD1_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD1_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD1_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD1_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD1_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD1_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD1_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD1_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad1 {
    // ======== Field Constants ========

    /// **REG_PAD1_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD1_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD1_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD1_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD1_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD1_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD1_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD1_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD1_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD1_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD1_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD1_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD1_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD1_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD1_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD1_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD1_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD1_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD1_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD1_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD1_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD1_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD1_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD1_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD1_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD1_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD1_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD1_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD1_RUE_OFFSET: u32 = 3;
    pub const REG_PAD1_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD1_RUE: u32 = 1 << 3;

    /// **REG_PAD1_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD1_RDE_OFFSET: u32 = 2;
    pub const REG_PAD1_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD1_RDE: u32 = 1 << 2;

    /// **REG_PAD1_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD1_DRV_OFFSET: u32 = 0;
    pub const REG_PAD1_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD1_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad1_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD1_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD1_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad1_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD1_FILTER_EN_MASK);
    }

    /// Clear REG_PAD1_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad1_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD1_FILTER_EN_MASK);
    }

    /// **REG_PAD1_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad1_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD1_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD1_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad1_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD1_FUN_IE_MASK);
    }

    /// Clear REG_PAD1_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad1_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD1_FUN_IE_MASK);
    }

    /// **REG_PAD1_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad1_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD1_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD1_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad1_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD1_SLP_OE_MASK);
    }

    /// Clear REG_PAD1_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad1_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD1_SLP_OE_MASK);
    }

    /// **REG_PAD1_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad1_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD1_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD1_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad1_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD1_SLP_IE_MASK);
    }

    /// Clear REG_PAD1_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad1_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD1_SLP_IE_MASK);
    }

    /// **REG_PAD1_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad1_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD1_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD1_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad1_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD1_SLP_SEL_MASK);
    }

    /// Clear REG_PAD1_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad1_slp_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD1_SLP_SEL_MASK);
    }

    /// **REG_PAD1_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad1_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD1_FUN_SEL_MASK) >> Self::REG_PAD1_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD1_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad1_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD1_FUN_SEL_MASK)
                | ((value << Self::REG_PAD1_FUN_SEL_OFFSET) & Self::REG_PAD1_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD1_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad1_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD1_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD1_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad1_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD1_MUX_SEL_MASK);
    }

    /// Clear REG_PAD1_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad1_mux_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD1_MUX_SEL_MASK);
    }

    /// **REG_PAD1_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad1_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD1_RUE_MASK) != 0
    }

    /// Set REG_PAD1_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad1_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD1_RUE_MASK);
    }

    /// Clear REG_PAD1_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad1_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD1_RUE_MASK);
    }

    /// **REG_PAD1_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad1_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD1_RDE_MASK) != 0
    }

    /// Set REG_PAD1_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad1_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD1_RDE_MASK);
    }

    /// Clear REG_PAD1_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad1_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD1_RDE_MASK);
    }

    /// **REG_PAD1_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad1_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD1_DRV_MASK) >> Self::REG_PAD1_DRV_OFFSET
    }

    /// Write **REG_PAD1_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad1_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD1_DRV_MASK)
                | ((value << Self::REG_PAD1_DRV_OFFSET) & Self::REG_PAD1_DRV_MASK)
        });
    }
}

/// PAD1 value type for batch read/modify/write operations
///
/// Use `pad1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad1Val {
    bits: u32,
}

impl Pad1Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD1_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad1_filter_en(&self) -> bool {
        (self.bits & Pad1::REG_PAD1_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD1_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad1_fun_ie(&self) -> bool {
        (self.bits & Pad1::REG_PAD1_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD1_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad1_slp_oe(&self) -> bool {
        (self.bits & Pad1::REG_PAD1_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD1_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad1_slp_ie(&self) -> bool {
        (self.bits & Pad1::REG_PAD1_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD1_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad1_slp_sel(&self) -> bool {
        (self.bits & Pad1::REG_PAD1_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD1_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad1_fun_sel(&self) -> u32 {
        (self.bits & Pad1::REG_PAD1_FUN_SEL_MASK) >> Pad1::REG_PAD1_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD1_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad1_mux_sel(&self) -> bool {
        (self.bits & Pad1::REG_PAD1_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD1_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad1_rue(&self) -> bool {
        (self.bits & Pad1::REG_PAD1_RUE_MASK) != 0
    }

    /// Read **REG_PAD1_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad1_rde(&self) -> bool {
        (self.bits & Pad1::REG_PAD1_RDE_MASK) != 0
    }

    /// Read **REG_PAD1_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad1_drv(&self) -> u32 {
        (self.bits & Pad1::REG_PAD1_DRV_MASK) >> Pad1::REG_PAD1_DRV_OFFSET
    }

    /// Set **REG_PAD1_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad1_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad1::REG_PAD1_FILTER_EN_MASK
            } else {
                self.bits & !Pad1::REG_PAD1_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD1_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad1_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad1::REG_PAD1_FUN_IE_MASK
            } else {
                self.bits & !Pad1::REG_PAD1_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD1_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad1_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad1::REG_PAD1_SLP_OE_MASK
            } else {
                self.bits & !Pad1::REG_PAD1_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD1_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad1_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad1::REG_PAD1_SLP_IE_MASK
            } else {
                self.bits & !Pad1::REG_PAD1_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD1_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad1_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad1::REG_PAD1_SLP_SEL_MASK
            } else {
                self.bits & !Pad1::REG_PAD1_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD1_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad1_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad1::REG_PAD1_FUN_SEL_MASK)
                | ((value << Pad1::REG_PAD1_FUN_SEL_OFFSET) & Pad1::REG_PAD1_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD1_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad1_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad1::REG_PAD1_MUX_SEL_MASK
            } else {
                self.bits & !Pad1::REG_PAD1_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD1_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad1_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad1::REG_PAD1_RUE_MASK
            } else {
                self.bits & !Pad1::REG_PAD1_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD1_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad1_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad1::REG_PAD1_RDE_MASK
            } else {
                self.bits & !Pad1::REG_PAD1_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD1_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad1_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad1::REG_PAD1_DRV_MASK)
                | ((value << Pad1::REG_PAD1_DRV_OFFSET) & Pad1::REG_PAD1_DRV_MASK),
        }
    }
}

impl Default for Pad1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad1Val {
        Pad1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad1Val) -> Pad1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD2 Register
///
/// Reserved
///
/// Address offset: 0x0010
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD2_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD2_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD2_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD2_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD2_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD2_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD2_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD2_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD2_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD2_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad2 {
    // ======== Field Constants ========

    /// **REG_PAD2_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD2_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD2_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD2_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD2_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD2_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD2_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD2_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD2_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD2_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD2_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD2_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD2_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD2_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD2_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD2_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD2_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD2_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD2_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD2_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD2_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD2_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD2_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD2_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD2_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD2_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD2_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD2_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD2_RUE_OFFSET: u32 = 3;
    pub const REG_PAD2_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD2_RUE: u32 = 1 << 3;

    /// **REG_PAD2_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD2_RDE_OFFSET: u32 = 2;
    pub const REG_PAD2_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD2_RDE: u32 = 1 << 2;

    /// **REG_PAD2_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD2_DRV_OFFSET: u32 = 0;
    pub const REG_PAD2_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD2_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad2_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD2_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD2_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad2_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD2_FILTER_EN_MASK);
    }

    /// Clear REG_PAD2_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad2_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD2_FILTER_EN_MASK);
    }

    /// **REG_PAD2_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad2_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD2_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD2_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad2_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD2_FUN_IE_MASK);
    }

    /// Clear REG_PAD2_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad2_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD2_FUN_IE_MASK);
    }

    /// **REG_PAD2_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad2_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD2_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD2_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad2_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD2_SLP_OE_MASK);
    }

    /// Clear REG_PAD2_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad2_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD2_SLP_OE_MASK);
    }

    /// **REG_PAD2_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad2_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD2_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD2_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad2_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD2_SLP_IE_MASK);
    }

    /// Clear REG_PAD2_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad2_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD2_SLP_IE_MASK);
    }

    /// **REG_PAD2_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad2_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD2_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD2_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad2_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD2_SLP_SEL_MASK);
    }

    /// Clear REG_PAD2_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad2_slp_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD2_SLP_SEL_MASK);
    }

    /// **REG_PAD2_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad2_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD2_FUN_SEL_MASK) >> Self::REG_PAD2_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD2_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad2_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD2_FUN_SEL_MASK)
                | ((value << Self::REG_PAD2_FUN_SEL_OFFSET) & Self::REG_PAD2_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD2_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad2_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD2_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD2_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad2_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD2_MUX_SEL_MASK);
    }

    /// Clear REG_PAD2_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad2_mux_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD2_MUX_SEL_MASK);
    }

    /// **REG_PAD2_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad2_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD2_RUE_MASK) != 0
    }

    /// Set REG_PAD2_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad2_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD2_RUE_MASK);
    }

    /// Clear REG_PAD2_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad2_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD2_RUE_MASK);
    }

    /// **REG_PAD2_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad2_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD2_RDE_MASK) != 0
    }

    /// Set REG_PAD2_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad2_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD2_RDE_MASK);
    }

    /// Clear REG_PAD2_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad2_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD2_RDE_MASK);
    }

    /// **REG_PAD2_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad2_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD2_DRV_MASK) >> Self::REG_PAD2_DRV_OFFSET
    }

    /// Write **REG_PAD2_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad2_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD2_DRV_MASK)
                | ((value << Self::REG_PAD2_DRV_OFFSET) & Self::REG_PAD2_DRV_MASK)
        });
    }
}

/// PAD2 value type for batch read/modify/write operations
///
/// Use `pad2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad2Val {
    bits: u32,
}

impl Pad2Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD2_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad2_filter_en(&self) -> bool {
        (self.bits & Pad2::REG_PAD2_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD2_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad2_fun_ie(&self) -> bool {
        (self.bits & Pad2::REG_PAD2_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD2_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad2_slp_oe(&self) -> bool {
        (self.bits & Pad2::REG_PAD2_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD2_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad2_slp_ie(&self) -> bool {
        (self.bits & Pad2::REG_PAD2_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD2_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad2_slp_sel(&self) -> bool {
        (self.bits & Pad2::REG_PAD2_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD2_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad2_fun_sel(&self) -> u32 {
        (self.bits & Pad2::REG_PAD2_FUN_SEL_MASK) >> Pad2::REG_PAD2_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD2_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad2_mux_sel(&self) -> bool {
        (self.bits & Pad2::REG_PAD2_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD2_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad2_rue(&self) -> bool {
        (self.bits & Pad2::REG_PAD2_RUE_MASK) != 0
    }

    /// Read **REG_PAD2_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad2_rde(&self) -> bool {
        (self.bits & Pad2::REG_PAD2_RDE_MASK) != 0
    }

    /// Read **REG_PAD2_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad2_drv(&self) -> u32 {
        (self.bits & Pad2::REG_PAD2_DRV_MASK) >> Pad2::REG_PAD2_DRV_OFFSET
    }

    /// Set **REG_PAD2_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad2_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad2::REG_PAD2_FILTER_EN_MASK
            } else {
                self.bits & !Pad2::REG_PAD2_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD2_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad2_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad2::REG_PAD2_FUN_IE_MASK
            } else {
                self.bits & !Pad2::REG_PAD2_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD2_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad2_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad2::REG_PAD2_SLP_OE_MASK
            } else {
                self.bits & !Pad2::REG_PAD2_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD2_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad2_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad2::REG_PAD2_SLP_IE_MASK
            } else {
                self.bits & !Pad2::REG_PAD2_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD2_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad2_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad2::REG_PAD2_SLP_SEL_MASK
            } else {
                self.bits & !Pad2::REG_PAD2_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD2_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad2_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad2::REG_PAD2_FUN_SEL_MASK)
                | ((value << Pad2::REG_PAD2_FUN_SEL_OFFSET) & Pad2::REG_PAD2_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD2_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad2_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad2::REG_PAD2_MUX_SEL_MASK
            } else {
                self.bits & !Pad2::REG_PAD2_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD2_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad2_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad2::REG_PAD2_RUE_MASK
            } else {
                self.bits & !Pad2::REG_PAD2_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD2_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad2_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad2::REG_PAD2_RDE_MASK
            } else {
                self.bits & !Pad2::REG_PAD2_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD2_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad2_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad2::REG_PAD2_DRV_MASK)
                | ((value << Pad2::REG_PAD2_DRV_OFFSET) & Pad2::REG_PAD2_DRV_MASK),
        }
    }
}

impl Default for Pad2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad2Val {
        Pad2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad2Val) -> Pad2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD3 Register
///
/// Reserved
///
/// Address offset: 0x0014
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD3_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD3_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD3_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD3_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD3_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD3_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD3_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD3_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD3_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD3_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad3 {
    // ======== Field Constants ========

    /// **REG_PAD3_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD3_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD3_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD3_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD3_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD3_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD3_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD3_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD3_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD3_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD3_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD3_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD3_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD3_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD3_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD3_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD3_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD3_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD3_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD3_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD3_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD3_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD3_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD3_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD3_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD3_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD3_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD3_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD3_RUE_OFFSET: u32 = 3;
    pub const REG_PAD3_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD3_RUE: u32 = 1 << 3;

    /// **REG_PAD3_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD3_RDE_OFFSET: u32 = 2;
    pub const REG_PAD3_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD3_RDE: u32 = 1 << 2;

    /// **REG_PAD3_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD3_DRV_OFFSET: u32 = 0;
    pub const REG_PAD3_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD3_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad3_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD3_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD3_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad3_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD3_FILTER_EN_MASK);
    }

    /// Clear REG_PAD3_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad3_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD3_FILTER_EN_MASK);
    }

    /// **REG_PAD3_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad3_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD3_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD3_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad3_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD3_FUN_IE_MASK);
    }

    /// Clear REG_PAD3_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad3_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD3_FUN_IE_MASK);
    }

    /// **REG_PAD3_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad3_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD3_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD3_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad3_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD3_SLP_OE_MASK);
    }

    /// Clear REG_PAD3_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad3_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD3_SLP_OE_MASK);
    }

    /// **REG_PAD3_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad3_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD3_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD3_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad3_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD3_SLP_IE_MASK);
    }

    /// Clear REG_PAD3_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad3_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD3_SLP_IE_MASK);
    }

    /// **REG_PAD3_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad3_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD3_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD3_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad3_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD3_SLP_SEL_MASK);
    }

    /// Clear REG_PAD3_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad3_slp_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD3_SLP_SEL_MASK);
    }

    /// **REG_PAD3_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad3_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD3_FUN_SEL_MASK) >> Self::REG_PAD3_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD3_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad3_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD3_FUN_SEL_MASK)
                | ((value << Self::REG_PAD3_FUN_SEL_OFFSET) & Self::REG_PAD3_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD3_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad3_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD3_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD3_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad3_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD3_MUX_SEL_MASK);
    }

    /// Clear REG_PAD3_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad3_mux_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD3_MUX_SEL_MASK);
    }

    /// **REG_PAD3_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad3_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD3_RUE_MASK) != 0
    }

    /// Set REG_PAD3_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad3_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD3_RUE_MASK);
    }

    /// Clear REG_PAD3_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad3_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD3_RUE_MASK);
    }

    /// **REG_PAD3_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad3_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD3_RDE_MASK) != 0
    }

    /// Set REG_PAD3_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad3_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD3_RDE_MASK);
    }

    /// Clear REG_PAD3_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad3_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD3_RDE_MASK);
    }

    /// **REG_PAD3_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad3_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD3_DRV_MASK) >> Self::REG_PAD3_DRV_OFFSET
    }

    /// Write **REG_PAD3_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad3_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD3_DRV_MASK)
                | ((value << Self::REG_PAD3_DRV_OFFSET) & Self::REG_PAD3_DRV_MASK)
        });
    }
}

/// PAD3 value type for batch read/modify/write operations
///
/// Use `pad3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad3Val {
    bits: u32,
}

impl Pad3Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD3_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad3_filter_en(&self) -> bool {
        (self.bits & Pad3::REG_PAD3_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD3_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad3_fun_ie(&self) -> bool {
        (self.bits & Pad3::REG_PAD3_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD3_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad3_slp_oe(&self) -> bool {
        (self.bits & Pad3::REG_PAD3_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD3_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad3_slp_ie(&self) -> bool {
        (self.bits & Pad3::REG_PAD3_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD3_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad3_slp_sel(&self) -> bool {
        (self.bits & Pad3::REG_PAD3_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD3_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad3_fun_sel(&self) -> u32 {
        (self.bits & Pad3::REG_PAD3_FUN_SEL_MASK) >> Pad3::REG_PAD3_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD3_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad3_mux_sel(&self) -> bool {
        (self.bits & Pad3::REG_PAD3_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD3_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad3_rue(&self) -> bool {
        (self.bits & Pad3::REG_PAD3_RUE_MASK) != 0
    }

    /// Read **REG_PAD3_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad3_rde(&self) -> bool {
        (self.bits & Pad3::REG_PAD3_RDE_MASK) != 0
    }

    /// Read **REG_PAD3_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad3_drv(&self) -> u32 {
        (self.bits & Pad3::REG_PAD3_DRV_MASK) >> Pad3::REG_PAD3_DRV_OFFSET
    }

    /// Set **REG_PAD3_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad3_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad3::REG_PAD3_FILTER_EN_MASK
            } else {
                self.bits & !Pad3::REG_PAD3_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD3_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad3_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad3::REG_PAD3_FUN_IE_MASK
            } else {
                self.bits & !Pad3::REG_PAD3_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD3_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad3_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad3::REG_PAD3_SLP_OE_MASK
            } else {
                self.bits & !Pad3::REG_PAD3_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD3_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad3_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad3::REG_PAD3_SLP_IE_MASK
            } else {
                self.bits & !Pad3::REG_PAD3_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD3_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad3_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad3::REG_PAD3_SLP_SEL_MASK
            } else {
                self.bits & !Pad3::REG_PAD3_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD3_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad3_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad3::REG_PAD3_FUN_SEL_MASK)
                | ((value << Pad3::REG_PAD3_FUN_SEL_OFFSET) & Pad3::REG_PAD3_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD3_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad3_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad3::REG_PAD3_MUX_SEL_MASK
            } else {
                self.bits & !Pad3::REG_PAD3_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD3_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad3_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad3::REG_PAD3_RUE_MASK
            } else {
                self.bits & !Pad3::REG_PAD3_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD3_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad3_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad3::REG_PAD3_RDE_MASK
            } else {
                self.bits & !Pad3::REG_PAD3_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD3_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad3_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad3::REG_PAD3_DRV_MASK)
                | ((value << Pad3::REG_PAD3_DRV_OFFSET) & Pad3::REG_PAD3_DRV_MASK),
        }
    }
}

impl Default for Pad3Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad3Val {
        Pad3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad3Val) -> Pad3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD4 Register
///
/// Reserved
///
/// Address offset: 0x0018
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD4_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD4_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD4_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD4_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD4_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD4_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD4_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD4_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD4_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD4_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad4 {
    // ======== Field Constants ========

    /// **REG_PAD4_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD4_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD4_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD4_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD4_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD4_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD4_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD4_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD4_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD4_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD4_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD4_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD4_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD4_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD4_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD4_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD4_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD4_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD4_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD4_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD4_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD4_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD4_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD4_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD4_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD4_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD4_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD4_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD4_RUE_OFFSET: u32 = 3;
    pub const REG_PAD4_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD4_RUE: u32 = 1 << 3;

    /// **REG_PAD4_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD4_RDE_OFFSET: u32 = 2;
    pub const REG_PAD4_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD4_RDE: u32 = 1 << 2;

    /// **REG_PAD4_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD4_DRV_OFFSET: u32 = 0;
    pub const REG_PAD4_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD4_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad4_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD4_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD4_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad4_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD4_FILTER_EN_MASK);
    }

    /// Clear REG_PAD4_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad4_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD4_FILTER_EN_MASK);
    }

    /// **REG_PAD4_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad4_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD4_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD4_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad4_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD4_FUN_IE_MASK);
    }

    /// Clear REG_PAD4_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad4_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD4_FUN_IE_MASK);
    }

    /// **REG_PAD4_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad4_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD4_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD4_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad4_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD4_SLP_OE_MASK);
    }

    /// Clear REG_PAD4_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad4_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD4_SLP_OE_MASK);
    }

    /// **REG_PAD4_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad4_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD4_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD4_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad4_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD4_SLP_IE_MASK);
    }

    /// Clear REG_PAD4_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad4_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD4_SLP_IE_MASK);
    }

    /// **REG_PAD4_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad4_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD4_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD4_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad4_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD4_SLP_SEL_MASK);
    }

    /// Clear REG_PAD4_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad4_slp_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD4_SLP_SEL_MASK);
    }

    /// **REG_PAD4_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad4_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD4_FUN_SEL_MASK) >> Self::REG_PAD4_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD4_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad4_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD4_FUN_SEL_MASK)
                | ((value << Self::REG_PAD4_FUN_SEL_OFFSET) & Self::REG_PAD4_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD4_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad4_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD4_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD4_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad4_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD4_MUX_SEL_MASK);
    }

    /// Clear REG_PAD4_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad4_mux_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD4_MUX_SEL_MASK);
    }

    /// **REG_PAD4_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad4_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD4_RUE_MASK) != 0
    }

    /// Set REG_PAD4_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad4_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD4_RUE_MASK);
    }

    /// Clear REG_PAD4_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad4_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD4_RUE_MASK);
    }

    /// **REG_PAD4_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad4_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD4_RDE_MASK) != 0
    }

    /// Set REG_PAD4_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad4_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD4_RDE_MASK);
    }

    /// Clear REG_PAD4_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad4_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD4_RDE_MASK);
    }

    /// **REG_PAD4_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad4_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD4_DRV_MASK) >> Self::REG_PAD4_DRV_OFFSET
    }

    /// Write **REG_PAD4_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad4_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD4_DRV_MASK)
                | ((value << Self::REG_PAD4_DRV_OFFSET) & Self::REG_PAD4_DRV_MASK)
        });
    }
}

/// PAD4 value type for batch read/modify/write operations
///
/// Use `pad4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad4Val {
    bits: u32,
}

impl Pad4Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD4_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad4_filter_en(&self) -> bool {
        (self.bits & Pad4::REG_PAD4_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD4_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad4_fun_ie(&self) -> bool {
        (self.bits & Pad4::REG_PAD4_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD4_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad4_slp_oe(&self) -> bool {
        (self.bits & Pad4::REG_PAD4_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD4_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad4_slp_ie(&self) -> bool {
        (self.bits & Pad4::REG_PAD4_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD4_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad4_slp_sel(&self) -> bool {
        (self.bits & Pad4::REG_PAD4_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD4_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad4_fun_sel(&self) -> u32 {
        (self.bits & Pad4::REG_PAD4_FUN_SEL_MASK) >> Pad4::REG_PAD4_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD4_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad4_mux_sel(&self) -> bool {
        (self.bits & Pad4::REG_PAD4_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD4_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad4_rue(&self) -> bool {
        (self.bits & Pad4::REG_PAD4_RUE_MASK) != 0
    }

    /// Read **REG_PAD4_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad4_rde(&self) -> bool {
        (self.bits & Pad4::REG_PAD4_RDE_MASK) != 0
    }

    /// Read **REG_PAD4_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad4_drv(&self) -> u32 {
        (self.bits & Pad4::REG_PAD4_DRV_MASK) >> Pad4::REG_PAD4_DRV_OFFSET
    }

    /// Set **REG_PAD4_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad4_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad4::REG_PAD4_FILTER_EN_MASK
            } else {
                self.bits & !Pad4::REG_PAD4_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD4_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad4_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad4::REG_PAD4_FUN_IE_MASK
            } else {
                self.bits & !Pad4::REG_PAD4_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD4_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad4_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad4::REG_PAD4_SLP_OE_MASK
            } else {
                self.bits & !Pad4::REG_PAD4_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD4_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad4_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad4::REG_PAD4_SLP_IE_MASK
            } else {
                self.bits & !Pad4::REG_PAD4_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD4_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad4_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad4::REG_PAD4_SLP_SEL_MASK
            } else {
                self.bits & !Pad4::REG_PAD4_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD4_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad4_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad4::REG_PAD4_FUN_SEL_MASK)
                | ((value << Pad4::REG_PAD4_FUN_SEL_OFFSET) & Pad4::REG_PAD4_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD4_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad4_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad4::REG_PAD4_MUX_SEL_MASK
            } else {
                self.bits & !Pad4::REG_PAD4_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD4_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad4_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad4::REG_PAD4_RUE_MASK
            } else {
                self.bits & !Pad4::REG_PAD4_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD4_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad4_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad4::REG_PAD4_RDE_MASK
            } else {
                self.bits & !Pad4::REG_PAD4_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD4_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad4_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad4::REG_PAD4_DRV_MASK)
                | ((value << Pad4::REG_PAD4_DRV_OFFSET) & Pad4::REG_PAD4_DRV_MASK),
        }
    }
}

impl Default for Pad4Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad4Val {
        Pad4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad4Val) -> Pad4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD5 Register
///
/// Reserved
///
/// Address offset: 0x001c
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD5_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD5_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD5_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD5_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD5_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD5_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD5_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD5_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD5_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD5_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad5 {
    // ======== Field Constants ========

    /// **REG_PAD5_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD5_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD5_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD5_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD5_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD5_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD5_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD5_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD5_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD5_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD5_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD5_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD5_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD5_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD5_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD5_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD5_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD5_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD5_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD5_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD5_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD5_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD5_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD5_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD5_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD5_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD5_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD5_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD5_RUE_OFFSET: u32 = 3;
    pub const REG_PAD5_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD5_RUE: u32 = 1 << 3;

    /// **REG_PAD5_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD5_RDE_OFFSET: u32 = 2;
    pub const REG_PAD5_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD5_RDE: u32 = 1 << 2;

    /// **REG_PAD5_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD5_DRV_OFFSET: u32 = 0;
    pub const REG_PAD5_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD5_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad5_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD5_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD5_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad5_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD5_FILTER_EN_MASK);
    }

    /// Clear REG_PAD5_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad5_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD5_FILTER_EN_MASK);
    }

    /// **REG_PAD5_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad5_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD5_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD5_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad5_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD5_FUN_IE_MASK);
    }

    /// Clear REG_PAD5_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad5_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD5_FUN_IE_MASK);
    }

    /// **REG_PAD5_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad5_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD5_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD5_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad5_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD5_SLP_OE_MASK);
    }

    /// Clear REG_PAD5_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad5_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD5_SLP_OE_MASK);
    }

    /// **REG_PAD5_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad5_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD5_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD5_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad5_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD5_SLP_IE_MASK);
    }

    /// Clear REG_PAD5_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad5_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD5_SLP_IE_MASK);
    }

    /// **REG_PAD5_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad5_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD5_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD5_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad5_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD5_SLP_SEL_MASK);
    }

    /// Clear REG_PAD5_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad5_slp_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD5_SLP_SEL_MASK);
    }

    /// **REG_PAD5_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad5_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD5_FUN_SEL_MASK) >> Self::REG_PAD5_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD5_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad5_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD5_FUN_SEL_MASK)
                | ((value << Self::REG_PAD5_FUN_SEL_OFFSET) & Self::REG_PAD5_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD5_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad5_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD5_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD5_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad5_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD5_MUX_SEL_MASK);
    }

    /// Clear REG_PAD5_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad5_mux_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD5_MUX_SEL_MASK);
    }

    /// **REG_PAD5_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad5_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD5_RUE_MASK) != 0
    }

    /// Set REG_PAD5_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad5_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD5_RUE_MASK);
    }

    /// Clear REG_PAD5_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad5_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD5_RUE_MASK);
    }

    /// **REG_PAD5_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad5_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD5_RDE_MASK) != 0
    }

    /// Set REG_PAD5_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad5_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD5_RDE_MASK);
    }

    /// Clear REG_PAD5_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad5_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD5_RDE_MASK);
    }

    /// **REG_PAD5_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad5_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD5_DRV_MASK) >> Self::REG_PAD5_DRV_OFFSET
    }

    /// Write **REG_PAD5_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad5_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD5_DRV_MASK)
                | ((value << Self::REG_PAD5_DRV_OFFSET) & Self::REG_PAD5_DRV_MASK)
        });
    }
}

/// PAD5 value type for batch read/modify/write operations
///
/// Use `pad5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad5Val {
    bits: u32,
}

impl Pad5Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD5_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad5_filter_en(&self) -> bool {
        (self.bits & Pad5::REG_PAD5_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD5_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad5_fun_ie(&self) -> bool {
        (self.bits & Pad5::REG_PAD5_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD5_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad5_slp_oe(&self) -> bool {
        (self.bits & Pad5::REG_PAD5_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD5_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad5_slp_ie(&self) -> bool {
        (self.bits & Pad5::REG_PAD5_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD5_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad5_slp_sel(&self) -> bool {
        (self.bits & Pad5::REG_PAD5_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD5_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad5_fun_sel(&self) -> u32 {
        (self.bits & Pad5::REG_PAD5_FUN_SEL_MASK) >> Pad5::REG_PAD5_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD5_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad5_mux_sel(&self) -> bool {
        (self.bits & Pad5::REG_PAD5_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD5_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad5_rue(&self) -> bool {
        (self.bits & Pad5::REG_PAD5_RUE_MASK) != 0
    }

    /// Read **REG_PAD5_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad5_rde(&self) -> bool {
        (self.bits & Pad5::REG_PAD5_RDE_MASK) != 0
    }

    /// Read **REG_PAD5_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad5_drv(&self) -> u32 {
        (self.bits & Pad5::REG_PAD5_DRV_MASK) >> Pad5::REG_PAD5_DRV_OFFSET
    }

    /// Set **REG_PAD5_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad5_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad5::REG_PAD5_FILTER_EN_MASK
            } else {
                self.bits & !Pad5::REG_PAD5_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD5_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad5_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad5::REG_PAD5_FUN_IE_MASK
            } else {
                self.bits & !Pad5::REG_PAD5_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD5_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad5_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad5::REG_PAD5_SLP_OE_MASK
            } else {
                self.bits & !Pad5::REG_PAD5_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD5_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad5_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad5::REG_PAD5_SLP_IE_MASK
            } else {
                self.bits & !Pad5::REG_PAD5_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD5_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad5_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad5::REG_PAD5_SLP_SEL_MASK
            } else {
                self.bits & !Pad5::REG_PAD5_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD5_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad5_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad5::REG_PAD5_FUN_SEL_MASK)
                | ((value << Pad5::REG_PAD5_FUN_SEL_OFFSET) & Pad5::REG_PAD5_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD5_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad5_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad5::REG_PAD5_MUX_SEL_MASK
            } else {
                self.bits & !Pad5::REG_PAD5_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD5_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad5_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad5::REG_PAD5_RUE_MASK
            } else {
                self.bits & !Pad5::REG_PAD5_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD5_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad5_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad5::REG_PAD5_RDE_MASK
            } else {
                self.bits & !Pad5::REG_PAD5_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD5_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad5_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad5::REG_PAD5_DRV_MASK)
                | ((value << Pad5::REG_PAD5_DRV_OFFSET) & Pad5::REG_PAD5_DRV_MASK),
        }
    }
}

impl Default for Pad5Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad5Val {
        Pad5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad5Val) -> Pad5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD6 Register
///
/// Reserved
///
/// Address offset: 0x0020
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD6_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD6_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD6_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD6_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD6_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD6_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD6_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD6_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD6_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD6_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad6 {
    // ======== Field Constants ========

    /// **REG_PAD6_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD6_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD6_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD6_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD6_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD6_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD6_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD6_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD6_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD6_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD6_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD6_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD6_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD6_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD6_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD6_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD6_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD6_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD6_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD6_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD6_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD6_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD6_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD6_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD6_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD6_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD6_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD6_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD6_RUE_OFFSET: u32 = 3;
    pub const REG_PAD6_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD6_RUE: u32 = 1 << 3;

    /// **REG_PAD6_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD6_RDE_OFFSET: u32 = 2;
    pub const REG_PAD6_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD6_RDE: u32 = 1 << 2;

    /// **REG_PAD6_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD6_DRV_OFFSET: u32 = 0;
    pub const REG_PAD6_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD6_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad6_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD6_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD6_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad6_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD6_FILTER_EN_MASK);
    }

    /// Clear REG_PAD6_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad6_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD6_FILTER_EN_MASK);
    }

    /// **REG_PAD6_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad6_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD6_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD6_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad6_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD6_FUN_IE_MASK);
    }

    /// Clear REG_PAD6_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad6_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD6_FUN_IE_MASK);
    }

    /// **REG_PAD6_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad6_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD6_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD6_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad6_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD6_SLP_OE_MASK);
    }

    /// Clear REG_PAD6_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad6_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD6_SLP_OE_MASK);
    }

    /// **REG_PAD6_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad6_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD6_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD6_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad6_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD6_SLP_IE_MASK);
    }

    /// Clear REG_PAD6_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad6_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD6_SLP_IE_MASK);
    }

    /// **REG_PAD6_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad6_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD6_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD6_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad6_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD6_SLP_SEL_MASK);
    }

    /// Clear REG_PAD6_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad6_slp_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD6_SLP_SEL_MASK);
    }

    /// **REG_PAD6_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad6_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD6_FUN_SEL_MASK) >> Self::REG_PAD6_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD6_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad6_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD6_FUN_SEL_MASK)
                | ((value << Self::REG_PAD6_FUN_SEL_OFFSET) & Self::REG_PAD6_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD6_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad6_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD6_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD6_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad6_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD6_MUX_SEL_MASK);
    }

    /// Clear REG_PAD6_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad6_mux_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD6_MUX_SEL_MASK);
    }

    /// **REG_PAD6_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad6_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD6_RUE_MASK) != 0
    }

    /// Set REG_PAD6_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad6_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD6_RUE_MASK);
    }

    /// Clear REG_PAD6_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad6_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD6_RUE_MASK);
    }

    /// **REG_PAD6_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad6_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD6_RDE_MASK) != 0
    }

    /// Set REG_PAD6_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad6_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD6_RDE_MASK);
    }

    /// Clear REG_PAD6_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad6_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD6_RDE_MASK);
    }

    /// **REG_PAD6_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad6_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD6_DRV_MASK) >> Self::REG_PAD6_DRV_OFFSET
    }

    /// Write **REG_PAD6_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad6_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD6_DRV_MASK)
                | ((value << Self::REG_PAD6_DRV_OFFSET) & Self::REG_PAD6_DRV_MASK)
        });
    }
}

/// PAD6 value type for batch read/modify/write operations
///
/// Use `pad6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad6Val {
    bits: u32,
}

impl Pad6Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD6_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad6_filter_en(&self) -> bool {
        (self.bits & Pad6::REG_PAD6_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD6_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad6_fun_ie(&self) -> bool {
        (self.bits & Pad6::REG_PAD6_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD6_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad6_slp_oe(&self) -> bool {
        (self.bits & Pad6::REG_PAD6_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD6_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad6_slp_ie(&self) -> bool {
        (self.bits & Pad6::REG_PAD6_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD6_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad6_slp_sel(&self) -> bool {
        (self.bits & Pad6::REG_PAD6_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD6_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad6_fun_sel(&self) -> u32 {
        (self.bits & Pad6::REG_PAD6_FUN_SEL_MASK) >> Pad6::REG_PAD6_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD6_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad6_mux_sel(&self) -> bool {
        (self.bits & Pad6::REG_PAD6_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD6_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad6_rue(&self) -> bool {
        (self.bits & Pad6::REG_PAD6_RUE_MASK) != 0
    }

    /// Read **REG_PAD6_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad6_rde(&self) -> bool {
        (self.bits & Pad6::REG_PAD6_RDE_MASK) != 0
    }

    /// Read **REG_PAD6_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad6_drv(&self) -> u32 {
        (self.bits & Pad6::REG_PAD6_DRV_MASK) >> Pad6::REG_PAD6_DRV_OFFSET
    }

    /// Set **REG_PAD6_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad6_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad6::REG_PAD6_FILTER_EN_MASK
            } else {
                self.bits & !Pad6::REG_PAD6_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD6_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad6_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad6::REG_PAD6_FUN_IE_MASK
            } else {
                self.bits & !Pad6::REG_PAD6_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD6_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad6_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad6::REG_PAD6_SLP_OE_MASK
            } else {
                self.bits & !Pad6::REG_PAD6_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD6_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad6_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad6::REG_PAD6_SLP_IE_MASK
            } else {
                self.bits & !Pad6::REG_PAD6_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD6_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad6_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad6::REG_PAD6_SLP_SEL_MASK
            } else {
                self.bits & !Pad6::REG_PAD6_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD6_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad6_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad6::REG_PAD6_FUN_SEL_MASK)
                | ((value << Pad6::REG_PAD6_FUN_SEL_OFFSET) & Pad6::REG_PAD6_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD6_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad6_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad6::REG_PAD6_MUX_SEL_MASK
            } else {
                self.bits & !Pad6::REG_PAD6_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD6_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad6_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad6::REG_PAD6_RUE_MASK
            } else {
                self.bits & !Pad6::REG_PAD6_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD6_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad6_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad6::REG_PAD6_RDE_MASK
            } else {
                self.bits & !Pad6::REG_PAD6_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD6_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad6_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad6::REG_PAD6_DRV_MASK)
                | ((value << Pad6::REG_PAD6_DRV_OFFSET) & Pad6::REG_PAD6_DRV_MASK),
        }
    }
}

impl Default for Pad6Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad6Val {
        Pad6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad6Val) -> Pad6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD7 Register
///
/// Reserved
///
/// Address offset: 0x0024
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD7_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD7_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD7_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD7_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD7_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD7_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD7_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD7_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD7_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD7_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad7 {
    // ======== Field Constants ========

    /// **REG_PAD7_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD7_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD7_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD7_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD7_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD7_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD7_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD7_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD7_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD7_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD7_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD7_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD7_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD7_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD7_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD7_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD7_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD7_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD7_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD7_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD7_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD7_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD7_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD7_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD7_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD7_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD7_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD7_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD7_RUE_OFFSET: u32 = 3;
    pub const REG_PAD7_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD7_RUE: u32 = 1 << 3;

    /// **REG_PAD7_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD7_RDE_OFFSET: u32 = 2;
    pub const REG_PAD7_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD7_RDE: u32 = 1 << 2;

    /// **REG_PAD7_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD7_DRV_OFFSET: u32 = 0;
    pub const REG_PAD7_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD7_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad7_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD7_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD7_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad7_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD7_FILTER_EN_MASK);
    }

    /// Clear REG_PAD7_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad7_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD7_FILTER_EN_MASK);
    }

    /// **REG_PAD7_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad7_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD7_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD7_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad7_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD7_FUN_IE_MASK);
    }

    /// Clear REG_PAD7_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad7_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD7_FUN_IE_MASK);
    }

    /// **REG_PAD7_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad7_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD7_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD7_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad7_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD7_SLP_OE_MASK);
    }

    /// Clear REG_PAD7_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad7_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD7_SLP_OE_MASK);
    }

    /// **REG_PAD7_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad7_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD7_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD7_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad7_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD7_SLP_IE_MASK);
    }

    /// Clear REG_PAD7_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad7_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD7_SLP_IE_MASK);
    }

    /// **REG_PAD7_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad7_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD7_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD7_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad7_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD7_SLP_SEL_MASK);
    }

    /// Clear REG_PAD7_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad7_slp_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD7_SLP_SEL_MASK);
    }

    /// **REG_PAD7_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad7_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD7_FUN_SEL_MASK) >> Self::REG_PAD7_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD7_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad7_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD7_FUN_SEL_MASK)
                | ((value << Self::REG_PAD7_FUN_SEL_OFFSET) & Self::REG_PAD7_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD7_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad7_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD7_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD7_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad7_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD7_MUX_SEL_MASK);
    }

    /// Clear REG_PAD7_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad7_mux_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD7_MUX_SEL_MASK);
    }

    /// **REG_PAD7_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad7_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD7_RUE_MASK) != 0
    }

    /// Set REG_PAD7_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad7_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD7_RUE_MASK);
    }

    /// Clear REG_PAD7_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad7_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD7_RUE_MASK);
    }

    /// **REG_PAD7_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad7_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD7_RDE_MASK) != 0
    }

    /// Set REG_PAD7_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad7_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD7_RDE_MASK);
    }

    /// Clear REG_PAD7_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad7_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD7_RDE_MASK);
    }

    /// **REG_PAD7_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad7_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD7_DRV_MASK) >> Self::REG_PAD7_DRV_OFFSET
    }

    /// Write **REG_PAD7_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad7_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD7_DRV_MASK)
                | ((value << Self::REG_PAD7_DRV_OFFSET) & Self::REG_PAD7_DRV_MASK)
        });
    }
}

/// PAD7 value type for batch read/modify/write operations
///
/// Use `pad7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad7Val {
    bits: u32,
}

impl Pad7Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD7_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad7_filter_en(&self) -> bool {
        (self.bits & Pad7::REG_PAD7_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD7_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad7_fun_ie(&self) -> bool {
        (self.bits & Pad7::REG_PAD7_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD7_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad7_slp_oe(&self) -> bool {
        (self.bits & Pad7::REG_PAD7_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD7_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad7_slp_ie(&self) -> bool {
        (self.bits & Pad7::REG_PAD7_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD7_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad7_slp_sel(&self) -> bool {
        (self.bits & Pad7::REG_PAD7_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD7_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad7_fun_sel(&self) -> u32 {
        (self.bits & Pad7::REG_PAD7_FUN_SEL_MASK) >> Pad7::REG_PAD7_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD7_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad7_mux_sel(&self) -> bool {
        (self.bits & Pad7::REG_PAD7_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD7_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad7_rue(&self) -> bool {
        (self.bits & Pad7::REG_PAD7_RUE_MASK) != 0
    }

    /// Read **REG_PAD7_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad7_rde(&self) -> bool {
        (self.bits & Pad7::REG_PAD7_RDE_MASK) != 0
    }

    /// Read **REG_PAD7_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad7_drv(&self) -> u32 {
        (self.bits & Pad7::REG_PAD7_DRV_MASK) >> Pad7::REG_PAD7_DRV_OFFSET
    }

    /// Set **REG_PAD7_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad7_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad7::REG_PAD7_FILTER_EN_MASK
            } else {
                self.bits & !Pad7::REG_PAD7_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD7_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad7_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad7::REG_PAD7_FUN_IE_MASK
            } else {
                self.bits & !Pad7::REG_PAD7_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD7_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad7_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad7::REG_PAD7_SLP_OE_MASK
            } else {
                self.bits & !Pad7::REG_PAD7_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD7_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad7_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad7::REG_PAD7_SLP_IE_MASK
            } else {
                self.bits & !Pad7::REG_PAD7_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD7_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad7_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad7::REG_PAD7_SLP_SEL_MASK
            } else {
                self.bits & !Pad7::REG_PAD7_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD7_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad7_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad7::REG_PAD7_FUN_SEL_MASK)
                | ((value << Pad7::REG_PAD7_FUN_SEL_OFFSET) & Pad7::REG_PAD7_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD7_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad7_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad7::REG_PAD7_MUX_SEL_MASK
            } else {
                self.bits & !Pad7::REG_PAD7_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD7_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad7_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad7::REG_PAD7_RUE_MASK
            } else {
                self.bits & !Pad7::REG_PAD7_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD7_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad7_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad7::REG_PAD7_RDE_MASK
            } else {
                self.bits & !Pad7::REG_PAD7_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD7_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad7_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad7::REG_PAD7_DRV_MASK)
                | ((value << Pad7::REG_PAD7_DRV_OFFSET) & Pad7::REG_PAD7_DRV_MASK),
        }
    }
}

impl Default for Pad7Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad7Val {
        Pad7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad7Val) -> Pad7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD8 Register
///
/// Reserved
///
/// Address offset: 0x0028
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD8_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD8_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD8_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD8_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD8_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD8_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD8_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD8_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD8_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD8_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad8 {
    // ======== Field Constants ========

    /// **REG_PAD8_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD8_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD8_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD8_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD8_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD8_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD8_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD8_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD8_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD8_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD8_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD8_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD8_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD8_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD8_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD8_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD8_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD8_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD8_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD8_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD8_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD8_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD8_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD8_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD8_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD8_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD8_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD8_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD8_RUE_OFFSET: u32 = 3;
    pub const REG_PAD8_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD8_RUE: u32 = 1 << 3;

    /// **REG_PAD8_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD8_RDE_OFFSET: u32 = 2;
    pub const REG_PAD8_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD8_RDE: u32 = 1 << 2;

    /// **REG_PAD8_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD8_DRV_OFFSET: u32 = 0;
    pub const REG_PAD8_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD8_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad8_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD8_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD8_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad8_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD8_FILTER_EN_MASK);
    }

    /// Clear REG_PAD8_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad8_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD8_FILTER_EN_MASK);
    }

    /// **REG_PAD8_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad8_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD8_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD8_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad8_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD8_FUN_IE_MASK);
    }

    /// Clear REG_PAD8_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad8_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD8_FUN_IE_MASK);
    }

    /// **REG_PAD8_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad8_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD8_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD8_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad8_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD8_SLP_OE_MASK);
    }

    /// Clear REG_PAD8_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad8_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD8_SLP_OE_MASK);
    }

    /// **REG_PAD8_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad8_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD8_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD8_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad8_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD8_SLP_IE_MASK);
    }

    /// Clear REG_PAD8_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad8_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD8_SLP_IE_MASK);
    }

    /// **REG_PAD8_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad8_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD8_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD8_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad8_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD8_SLP_SEL_MASK);
    }

    /// Clear REG_PAD8_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad8_slp_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD8_SLP_SEL_MASK);
    }

    /// **REG_PAD8_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad8_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD8_FUN_SEL_MASK) >> Self::REG_PAD8_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD8_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad8_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD8_FUN_SEL_MASK)
                | ((value << Self::REG_PAD8_FUN_SEL_OFFSET) & Self::REG_PAD8_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD8_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad8_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD8_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD8_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad8_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD8_MUX_SEL_MASK);
    }

    /// Clear REG_PAD8_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad8_mux_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD8_MUX_SEL_MASK);
    }

    /// **REG_PAD8_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad8_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD8_RUE_MASK) != 0
    }

    /// Set REG_PAD8_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad8_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD8_RUE_MASK);
    }

    /// Clear REG_PAD8_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad8_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD8_RUE_MASK);
    }

    /// **REG_PAD8_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad8_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD8_RDE_MASK) != 0
    }

    /// Set REG_PAD8_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad8_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD8_RDE_MASK);
    }

    /// Clear REG_PAD8_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad8_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD8_RDE_MASK);
    }

    /// **REG_PAD8_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad8_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD8_DRV_MASK) >> Self::REG_PAD8_DRV_OFFSET
    }

    /// Write **REG_PAD8_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad8_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD8_DRV_MASK)
                | ((value << Self::REG_PAD8_DRV_OFFSET) & Self::REG_PAD8_DRV_MASK)
        });
    }
}

/// PAD8 value type for batch read/modify/write operations
///
/// Use `pad8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad8Val {
    bits: u32,
}

impl Pad8Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD8_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad8_filter_en(&self) -> bool {
        (self.bits & Pad8::REG_PAD8_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD8_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad8_fun_ie(&self) -> bool {
        (self.bits & Pad8::REG_PAD8_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD8_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad8_slp_oe(&self) -> bool {
        (self.bits & Pad8::REG_PAD8_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD8_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad8_slp_ie(&self) -> bool {
        (self.bits & Pad8::REG_PAD8_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD8_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad8_slp_sel(&self) -> bool {
        (self.bits & Pad8::REG_PAD8_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD8_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad8_fun_sel(&self) -> u32 {
        (self.bits & Pad8::REG_PAD8_FUN_SEL_MASK) >> Pad8::REG_PAD8_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD8_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad8_mux_sel(&self) -> bool {
        (self.bits & Pad8::REG_PAD8_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD8_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad8_rue(&self) -> bool {
        (self.bits & Pad8::REG_PAD8_RUE_MASK) != 0
    }

    /// Read **REG_PAD8_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad8_rde(&self) -> bool {
        (self.bits & Pad8::REG_PAD8_RDE_MASK) != 0
    }

    /// Read **REG_PAD8_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad8_drv(&self) -> u32 {
        (self.bits & Pad8::REG_PAD8_DRV_MASK) >> Pad8::REG_PAD8_DRV_OFFSET
    }

    /// Set **REG_PAD8_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad8_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad8::REG_PAD8_FILTER_EN_MASK
            } else {
                self.bits & !Pad8::REG_PAD8_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD8_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad8_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad8::REG_PAD8_FUN_IE_MASK
            } else {
                self.bits & !Pad8::REG_PAD8_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD8_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad8_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad8::REG_PAD8_SLP_OE_MASK
            } else {
                self.bits & !Pad8::REG_PAD8_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD8_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad8_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad8::REG_PAD8_SLP_IE_MASK
            } else {
                self.bits & !Pad8::REG_PAD8_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD8_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad8_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad8::REG_PAD8_SLP_SEL_MASK
            } else {
                self.bits & !Pad8::REG_PAD8_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD8_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad8_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad8::REG_PAD8_FUN_SEL_MASK)
                | ((value << Pad8::REG_PAD8_FUN_SEL_OFFSET) & Pad8::REG_PAD8_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD8_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad8_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad8::REG_PAD8_MUX_SEL_MASK
            } else {
                self.bits & !Pad8::REG_PAD8_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD8_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad8_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad8::REG_PAD8_RUE_MASK
            } else {
                self.bits & !Pad8::REG_PAD8_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD8_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad8_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad8::REG_PAD8_RDE_MASK
            } else {
                self.bits & !Pad8::REG_PAD8_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD8_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad8_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad8::REG_PAD8_DRV_MASK)
                | ((value << Pad8::REG_PAD8_DRV_OFFSET) & Pad8::REG_PAD8_DRV_MASK),
        }
    }
}

impl Default for Pad8Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad8Val {
        Pad8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad8Val) -> Pad8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD9 Register
///
/// Reserved
///
/// Address offset: 0x002c
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD9_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD9_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD9_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD9_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD9_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD9_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD9_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD9_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD9_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD9_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad9 {
    // ======== Field Constants ========

    /// **REG_PAD9_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD9_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD9_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD9_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD9_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD9_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD9_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD9_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD9_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD9_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD9_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD9_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD9_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD9_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD9_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD9_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD9_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD9_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD9_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD9_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD9_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD9_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD9_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD9_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD9_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD9_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD9_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD9_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD9_RUE_OFFSET: u32 = 3;
    pub const REG_PAD9_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD9_RUE: u32 = 1 << 3;

    /// **REG_PAD9_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD9_RDE_OFFSET: u32 = 2;
    pub const REG_PAD9_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD9_RDE: u32 = 1 << 2;

    /// **REG_PAD9_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD9_DRV_OFFSET: u32 = 0;
    pub const REG_PAD9_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD9_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad9_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD9_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD9_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad9_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD9_FILTER_EN_MASK);
    }

    /// Clear REG_PAD9_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad9_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD9_FILTER_EN_MASK);
    }

    /// **REG_PAD9_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad9_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD9_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD9_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad9_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD9_FUN_IE_MASK);
    }

    /// Clear REG_PAD9_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad9_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD9_FUN_IE_MASK);
    }

    /// **REG_PAD9_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad9_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD9_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD9_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad9_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD9_SLP_OE_MASK);
    }

    /// Clear REG_PAD9_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad9_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD9_SLP_OE_MASK);
    }

    /// **REG_PAD9_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad9_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD9_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD9_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad9_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD9_SLP_IE_MASK);
    }

    /// Clear REG_PAD9_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad9_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD9_SLP_IE_MASK);
    }

    /// **REG_PAD9_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad9_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD9_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD9_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad9_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD9_SLP_SEL_MASK);
    }

    /// Clear REG_PAD9_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad9_slp_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD9_SLP_SEL_MASK);
    }

    /// **REG_PAD9_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad9_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD9_FUN_SEL_MASK) >> Self::REG_PAD9_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD9_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad9_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD9_FUN_SEL_MASK)
                | ((value << Self::REG_PAD9_FUN_SEL_OFFSET) & Self::REG_PAD9_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD9_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad9_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD9_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD9_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad9_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD9_MUX_SEL_MASK);
    }

    /// Clear REG_PAD9_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad9_mux_sel(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD9_MUX_SEL_MASK);
    }

    /// **REG_PAD9_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad9_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD9_RUE_MASK) != 0
    }

    /// Set REG_PAD9_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad9_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD9_RUE_MASK);
    }

    /// Clear REG_PAD9_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad9_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD9_RUE_MASK);
    }

    /// **REG_PAD9_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad9_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD9_RDE_MASK) != 0
    }

    /// Set REG_PAD9_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad9_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD9_RDE_MASK);
    }

    /// Clear REG_PAD9_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad9_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD9_RDE_MASK);
    }

    /// **REG_PAD9_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad9_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD9_DRV_MASK) >> Self::REG_PAD9_DRV_OFFSET
    }

    /// Write **REG_PAD9_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad9_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD9_DRV_MASK)
                | ((value << Self::REG_PAD9_DRV_OFFSET) & Self::REG_PAD9_DRV_MASK)
        });
    }
}

/// PAD9 value type for batch read/modify/write operations
///
/// Use `pad9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad9Val {
    bits: u32,
}

impl Pad9Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD9_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad9_filter_en(&self) -> bool {
        (self.bits & Pad9::REG_PAD9_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD9_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad9_fun_ie(&self) -> bool {
        (self.bits & Pad9::REG_PAD9_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD9_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad9_slp_oe(&self) -> bool {
        (self.bits & Pad9::REG_PAD9_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD9_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad9_slp_ie(&self) -> bool {
        (self.bits & Pad9::REG_PAD9_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD9_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad9_slp_sel(&self) -> bool {
        (self.bits & Pad9::REG_PAD9_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD9_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad9_fun_sel(&self) -> u32 {
        (self.bits & Pad9::REG_PAD9_FUN_SEL_MASK) >> Pad9::REG_PAD9_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD9_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad9_mux_sel(&self) -> bool {
        (self.bits & Pad9::REG_PAD9_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD9_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad9_rue(&self) -> bool {
        (self.bits & Pad9::REG_PAD9_RUE_MASK) != 0
    }

    /// Read **REG_PAD9_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad9_rde(&self) -> bool {
        (self.bits & Pad9::REG_PAD9_RDE_MASK) != 0
    }

    /// Read **REG_PAD9_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad9_drv(&self) -> u32 {
        (self.bits & Pad9::REG_PAD9_DRV_MASK) >> Pad9::REG_PAD9_DRV_OFFSET
    }

    /// Set **REG_PAD9_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad9_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad9::REG_PAD9_FILTER_EN_MASK
            } else {
                self.bits & !Pad9::REG_PAD9_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD9_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad9_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad9::REG_PAD9_FUN_IE_MASK
            } else {
                self.bits & !Pad9::REG_PAD9_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD9_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad9_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad9::REG_PAD9_SLP_OE_MASK
            } else {
                self.bits & !Pad9::REG_PAD9_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD9_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad9_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad9::REG_PAD9_SLP_IE_MASK
            } else {
                self.bits & !Pad9::REG_PAD9_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD9_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad9_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad9::REG_PAD9_SLP_SEL_MASK
            } else {
                self.bits & !Pad9::REG_PAD9_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD9_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad9_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad9::REG_PAD9_FUN_SEL_MASK)
                | ((value << Pad9::REG_PAD9_FUN_SEL_OFFSET) & Pad9::REG_PAD9_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD9_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad9_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad9::REG_PAD9_MUX_SEL_MASK
            } else {
                self.bits & !Pad9::REG_PAD9_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD9_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad9_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad9::REG_PAD9_RUE_MASK
            } else {
                self.bits & !Pad9::REG_PAD9_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD9_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad9_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad9::REG_PAD9_RDE_MASK
            } else {
                self.bits & !Pad9::REG_PAD9_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD9_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad9_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad9::REG_PAD9_DRV_MASK)
                | ((value << Pad9::REG_PAD9_DRV_OFFSET) & Pad9::REG_PAD9_DRV_MASK),
        }
    }
}

impl Default for Pad9Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad9Val {
        Pad9Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad9Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad9Val) -> Pad9Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD10 Register
///
/// Reserved
///
/// Address offset: 0x0030
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD10_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD10_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD10_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD10_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD10_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD10_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD10_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD10_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD10_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD10_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad10 {
    // ======== Field Constants ========

    /// **REG_PAD10_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD10_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD10_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD10_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD10_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD10_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD10_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD10_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD10_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD10_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD10_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD10_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD10_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD10_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD10_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD10_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD10_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD10_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD10_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD10_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD10_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD10_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD10_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD10_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD10_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD10_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD10_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD10_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD10_RUE_OFFSET: u32 = 3;
    pub const REG_PAD10_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD10_RUE: u32 = 1 << 3;

    /// **REG_PAD10_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD10_RDE_OFFSET: u32 = 2;
    pub const REG_PAD10_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD10_RDE: u32 = 1 << 2;

    /// **REG_PAD10_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD10_DRV_OFFSET: u32 = 0;
    pub const REG_PAD10_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD10_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad10_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD10_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD10_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad10_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD10_FILTER_EN_MASK);
    }

    /// Clear REG_PAD10_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad10_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD10_FILTER_EN_MASK);
    }

    /// **REG_PAD10_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad10_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD10_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD10_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad10_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD10_FUN_IE_MASK);
    }

    /// Clear REG_PAD10_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad10_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD10_FUN_IE_MASK);
    }

    /// **REG_PAD10_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad10_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD10_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD10_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad10_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD10_SLP_OE_MASK);
    }

    /// Clear REG_PAD10_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad10_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD10_SLP_OE_MASK);
    }

    /// **REG_PAD10_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad10_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD10_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD10_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad10_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD10_SLP_IE_MASK);
    }

    /// Clear REG_PAD10_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad10_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD10_SLP_IE_MASK);
    }

    /// **REG_PAD10_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad10_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD10_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD10_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad10_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD10_SLP_SEL_MASK);
    }

    /// Clear REG_PAD10_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad10_slp_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD10_SLP_SEL_MASK);
    }

    /// **REG_PAD10_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad10_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD10_FUN_SEL_MASK) >> Self::REG_PAD10_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD10_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad10_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD10_FUN_SEL_MASK)
                | ((value << Self::REG_PAD10_FUN_SEL_OFFSET) & Self::REG_PAD10_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD10_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad10_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD10_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD10_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad10_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD10_MUX_SEL_MASK);
    }

    /// Clear REG_PAD10_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad10_mux_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD10_MUX_SEL_MASK);
    }

    /// **REG_PAD10_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad10_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD10_RUE_MASK) != 0
    }

    /// Set REG_PAD10_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad10_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD10_RUE_MASK);
    }

    /// Clear REG_PAD10_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad10_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD10_RUE_MASK);
    }

    /// **REG_PAD10_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad10_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD10_RDE_MASK) != 0
    }

    /// Set REG_PAD10_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad10_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD10_RDE_MASK);
    }

    /// Clear REG_PAD10_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad10_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD10_RDE_MASK);
    }

    /// **REG_PAD10_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad10_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD10_DRV_MASK) >> Self::REG_PAD10_DRV_OFFSET
    }

    /// Write **REG_PAD10_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad10_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD10_DRV_MASK)
                | ((value << Self::REG_PAD10_DRV_OFFSET) & Self::REG_PAD10_DRV_MASK)
        });
    }
}

/// PAD10 value type for batch read/modify/write operations
///
/// Use `pad10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad10Val {
    bits: u32,
}

impl Pad10Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD10_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad10_filter_en(&self) -> bool {
        (self.bits & Pad10::REG_PAD10_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD10_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad10_fun_ie(&self) -> bool {
        (self.bits & Pad10::REG_PAD10_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD10_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad10_slp_oe(&self) -> bool {
        (self.bits & Pad10::REG_PAD10_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD10_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad10_slp_ie(&self) -> bool {
        (self.bits & Pad10::REG_PAD10_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD10_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad10_slp_sel(&self) -> bool {
        (self.bits & Pad10::REG_PAD10_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD10_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad10_fun_sel(&self) -> u32 {
        (self.bits & Pad10::REG_PAD10_FUN_SEL_MASK) >> Pad10::REG_PAD10_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD10_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad10_mux_sel(&self) -> bool {
        (self.bits & Pad10::REG_PAD10_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD10_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad10_rue(&self) -> bool {
        (self.bits & Pad10::REG_PAD10_RUE_MASK) != 0
    }

    /// Read **REG_PAD10_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad10_rde(&self) -> bool {
        (self.bits & Pad10::REG_PAD10_RDE_MASK) != 0
    }

    /// Read **REG_PAD10_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad10_drv(&self) -> u32 {
        (self.bits & Pad10::REG_PAD10_DRV_MASK) >> Pad10::REG_PAD10_DRV_OFFSET
    }

    /// Set **REG_PAD10_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad10_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad10::REG_PAD10_FILTER_EN_MASK
            } else {
                self.bits & !Pad10::REG_PAD10_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD10_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad10_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad10::REG_PAD10_FUN_IE_MASK
            } else {
                self.bits & !Pad10::REG_PAD10_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD10_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad10_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad10::REG_PAD10_SLP_OE_MASK
            } else {
                self.bits & !Pad10::REG_PAD10_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD10_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad10_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad10::REG_PAD10_SLP_IE_MASK
            } else {
                self.bits & !Pad10::REG_PAD10_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD10_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad10_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad10::REG_PAD10_SLP_SEL_MASK
            } else {
                self.bits & !Pad10::REG_PAD10_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD10_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad10_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad10::REG_PAD10_FUN_SEL_MASK)
                | ((value << Pad10::REG_PAD10_FUN_SEL_OFFSET) & Pad10::REG_PAD10_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD10_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad10_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad10::REG_PAD10_MUX_SEL_MASK
            } else {
                self.bits & !Pad10::REG_PAD10_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD10_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad10_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad10::REG_PAD10_RUE_MASK
            } else {
                self.bits & !Pad10::REG_PAD10_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD10_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad10_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad10::REG_PAD10_RDE_MASK
            } else {
                self.bits & !Pad10::REG_PAD10_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD10_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad10_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad10::REG_PAD10_DRV_MASK)
                | ((value << Pad10::REG_PAD10_DRV_OFFSET) & Pad10::REG_PAD10_DRV_MASK),
        }
    }
}

impl Default for Pad10Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad10Val {
        Pad10Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad10Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad10Val) -> Pad10Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD11 Register
///
/// Reserved
///
/// Address offset: 0x0034
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD11_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD11_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD11_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD11_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD11_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD11_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD11_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD11_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD11_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD11_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad11 {
    // ======== Field Constants ========

    /// **REG_PAD11_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD11_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD11_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD11_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD11_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD11_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD11_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD11_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD11_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD11_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD11_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD11_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD11_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD11_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD11_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD11_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD11_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD11_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD11_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD11_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD11_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD11_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD11_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD11_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD11_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD11_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD11_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD11_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD11_RUE_OFFSET: u32 = 3;
    pub const REG_PAD11_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD11_RUE: u32 = 1 << 3;

    /// **REG_PAD11_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD11_RDE_OFFSET: u32 = 2;
    pub const REG_PAD11_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD11_RDE: u32 = 1 << 2;

    /// **REG_PAD11_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD11_DRV_OFFSET: u32 = 0;
    pub const REG_PAD11_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD11_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad11_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD11_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD11_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad11_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD11_FILTER_EN_MASK);
    }

    /// Clear REG_PAD11_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad11_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD11_FILTER_EN_MASK);
    }

    /// **REG_PAD11_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad11_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD11_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD11_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad11_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD11_FUN_IE_MASK);
    }

    /// Clear REG_PAD11_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad11_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD11_FUN_IE_MASK);
    }

    /// **REG_PAD11_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad11_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD11_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD11_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad11_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD11_SLP_OE_MASK);
    }

    /// Clear REG_PAD11_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad11_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD11_SLP_OE_MASK);
    }

    /// **REG_PAD11_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad11_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD11_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD11_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad11_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD11_SLP_IE_MASK);
    }

    /// Clear REG_PAD11_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad11_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD11_SLP_IE_MASK);
    }

    /// **REG_PAD11_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad11_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD11_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD11_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad11_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD11_SLP_SEL_MASK);
    }

    /// Clear REG_PAD11_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad11_slp_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD11_SLP_SEL_MASK);
    }

    /// **REG_PAD11_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad11_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD11_FUN_SEL_MASK) >> Self::REG_PAD11_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD11_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad11_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD11_FUN_SEL_MASK)
                | ((value << Self::REG_PAD11_FUN_SEL_OFFSET) & Self::REG_PAD11_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD11_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad11_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD11_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD11_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad11_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD11_MUX_SEL_MASK);
    }

    /// Clear REG_PAD11_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad11_mux_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD11_MUX_SEL_MASK);
    }

    /// **REG_PAD11_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad11_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD11_RUE_MASK) != 0
    }

    /// Set REG_PAD11_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad11_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD11_RUE_MASK);
    }

    /// Clear REG_PAD11_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad11_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD11_RUE_MASK);
    }

    /// **REG_PAD11_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad11_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD11_RDE_MASK) != 0
    }

    /// Set REG_PAD11_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad11_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD11_RDE_MASK);
    }

    /// Clear REG_PAD11_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad11_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD11_RDE_MASK);
    }

    /// **REG_PAD11_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad11_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD11_DRV_MASK) >> Self::REG_PAD11_DRV_OFFSET
    }

    /// Write **REG_PAD11_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad11_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD11_DRV_MASK)
                | ((value << Self::REG_PAD11_DRV_OFFSET) & Self::REG_PAD11_DRV_MASK)
        });
    }
}

/// PAD11 value type for batch read/modify/write operations
///
/// Use `pad11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad11Val {
    bits: u32,
}

impl Pad11Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD11_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad11_filter_en(&self) -> bool {
        (self.bits & Pad11::REG_PAD11_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD11_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad11_fun_ie(&self) -> bool {
        (self.bits & Pad11::REG_PAD11_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD11_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad11_slp_oe(&self) -> bool {
        (self.bits & Pad11::REG_PAD11_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD11_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad11_slp_ie(&self) -> bool {
        (self.bits & Pad11::REG_PAD11_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD11_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad11_slp_sel(&self) -> bool {
        (self.bits & Pad11::REG_PAD11_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD11_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad11_fun_sel(&self) -> u32 {
        (self.bits & Pad11::REG_PAD11_FUN_SEL_MASK) >> Pad11::REG_PAD11_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD11_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad11_mux_sel(&self) -> bool {
        (self.bits & Pad11::REG_PAD11_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD11_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad11_rue(&self) -> bool {
        (self.bits & Pad11::REG_PAD11_RUE_MASK) != 0
    }

    /// Read **REG_PAD11_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad11_rde(&self) -> bool {
        (self.bits & Pad11::REG_PAD11_RDE_MASK) != 0
    }

    /// Read **REG_PAD11_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad11_drv(&self) -> u32 {
        (self.bits & Pad11::REG_PAD11_DRV_MASK) >> Pad11::REG_PAD11_DRV_OFFSET
    }

    /// Set **REG_PAD11_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad11_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad11::REG_PAD11_FILTER_EN_MASK
            } else {
                self.bits & !Pad11::REG_PAD11_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD11_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad11_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad11::REG_PAD11_FUN_IE_MASK
            } else {
                self.bits & !Pad11::REG_PAD11_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD11_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad11_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad11::REG_PAD11_SLP_OE_MASK
            } else {
                self.bits & !Pad11::REG_PAD11_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD11_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad11_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad11::REG_PAD11_SLP_IE_MASK
            } else {
                self.bits & !Pad11::REG_PAD11_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD11_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad11_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad11::REG_PAD11_SLP_SEL_MASK
            } else {
                self.bits & !Pad11::REG_PAD11_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD11_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad11_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad11::REG_PAD11_FUN_SEL_MASK)
                | ((value << Pad11::REG_PAD11_FUN_SEL_OFFSET) & Pad11::REG_PAD11_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD11_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad11_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad11::REG_PAD11_MUX_SEL_MASK
            } else {
                self.bits & !Pad11::REG_PAD11_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD11_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad11_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad11::REG_PAD11_RUE_MASK
            } else {
                self.bits & !Pad11::REG_PAD11_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD11_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad11_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad11::REG_PAD11_RDE_MASK
            } else {
                self.bits & !Pad11::REG_PAD11_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD11_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad11_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad11::REG_PAD11_DRV_MASK)
                | ((value << Pad11::REG_PAD11_DRV_OFFSET) & Pad11::REG_PAD11_DRV_MASK),
        }
    }
}

impl Default for Pad11Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad11Val {
        Pad11Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad11Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad11Val) -> Pad11Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD120 Register
///
/// Reserved
///
/// Address offset: 0x0038
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD12_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD12_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD12_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD12_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD12_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD12_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD12_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD12_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD12_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD12_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad120 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad120 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad120 {
    // ======== Field Constants ========

    /// **REG_PAD12_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD12_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD12_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD12_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD12_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD12_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD12_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD12_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD12_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD12_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD12_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD12_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD12_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD12_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD12_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD12_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD12_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD12_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD12_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD12_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD12_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD12_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD12_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD12_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD12_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD12_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD12_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD12_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD12_RUE_OFFSET: u32 = 3;
    pub const REG_PAD12_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD12_RUE: u32 = 1 << 3;

    /// **REG_PAD12_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD12_RDE_OFFSET: u32 = 2;
    pub const REG_PAD12_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD12_RDE: u32 = 1 << 2;

    /// **REG_PAD12_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD12_DRV_OFFSET: u32 = 0;
    pub const REG_PAD12_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD12_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad12_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD12_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD12_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad12_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD12_FILTER_EN_MASK);
    }

    /// Clear REG_PAD12_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad12_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD12_FILTER_EN_MASK);
    }

    /// **REG_PAD12_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad12_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD12_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD12_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad12_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD12_FUN_IE_MASK);
    }

    /// Clear REG_PAD12_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad12_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD12_FUN_IE_MASK);
    }

    /// **REG_PAD12_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad12_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD12_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD12_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad12_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD12_SLP_OE_MASK);
    }

    /// Clear REG_PAD12_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad12_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD12_SLP_OE_MASK);
    }

    /// **REG_PAD12_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad12_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD12_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD12_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad12_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD12_SLP_IE_MASK);
    }

    /// Clear REG_PAD12_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad12_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD12_SLP_IE_MASK);
    }

    /// **REG_PAD12_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad12_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD12_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD12_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad12_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD12_SLP_SEL_MASK);
    }

    /// Clear REG_PAD12_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad12_slp_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD12_SLP_SEL_MASK);
    }

    /// **REG_PAD12_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad12_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD12_FUN_SEL_MASK) >> Self::REG_PAD12_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD12_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad12_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD12_FUN_SEL_MASK)
                | ((value << Self::REG_PAD12_FUN_SEL_OFFSET) & Self::REG_PAD12_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD12_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad12_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD12_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD12_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad12_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD12_MUX_SEL_MASK);
    }

    /// Clear REG_PAD12_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad12_mux_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD12_MUX_SEL_MASK);
    }

    /// **REG_PAD12_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad12_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD12_RUE_MASK) != 0
    }

    /// Set REG_PAD12_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad12_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD12_RUE_MASK);
    }

    /// Clear REG_PAD12_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad12_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD12_RUE_MASK);
    }

    /// **REG_PAD12_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad12_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD12_RDE_MASK) != 0
    }

    /// Set REG_PAD12_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad12_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD12_RDE_MASK);
    }

    /// Clear REG_PAD12_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad12_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD12_RDE_MASK);
    }

    /// **REG_PAD12_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad12_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD12_DRV_MASK) >> Self::REG_PAD12_DRV_OFFSET
    }

    /// Write **REG_PAD12_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad12_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD12_DRV_MASK)
                | ((value << Self::REG_PAD12_DRV_OFFSET) & Self::REG_PAD12_DRV_MASK)
        });
    }
}

/// PAD120 value type for batch read/modify/write operations
///
/// Use `pad120.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad120.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad120Val {
    bits: u32,
}

impl Pad120Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD12_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad12_filter_en(&self) -> bool {
        (self.bits & Pad120::REG_PAD12_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD12_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad12_fun_ie(&self) -> bool {
        (self.bits & Pad120::REG_PAD12_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD12_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad12_slp_oe(&self) -> bool {
        (self.bits & Pad120::REG_PAD12_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD12_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad12_slp_ie(&self) -> bool {
        (self.bits & Pad120::REG_PAD12_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD12_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad12_slp_sel(&self) -> bool {
        (self.bits & Pad120::REG_PAD12_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD12_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad12_fun_sel(&self) -> u32 {
        (self.bits & Pad120::REG_PAD12_FUN_SEL_MASK) >> Pad120::REG_PAD12_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD12_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad12_mux_sel(&self) -> bool {
        (self.bits & Pad120::REG_PAD12_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD12_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad12_rue(&self) -> bool {
        (self.bits & Pad120::REG_PAD12_RUE_MASK) != 0
    }

    /// Read **REG_PAD12_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad12_rde(&self) -> bool {
        (self.bits & Pad120::REG_PAD12_RDE_MASK) != 0
    }

    /// Read **REG_PAD12_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad12_drv(&self) -> u32 {
        (self.bits & Pad120::REG_PAD12_DRV_MASK) >> Pad120::REG_PAD12_DRV_OFFSET
    }

    /// Set **REG_PAD12_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad12_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad120::REG_PAD12_FILTER_EN_MASK
            } else {
                self.bits & !Pad120::REG_PAD12_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD12_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad12_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad120::REG_PAD12_FUN_IE_MASK
            } else {
                self.bits & !Pad120::REG_PAD12_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD12_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad12_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad120::REG_PAD12_SLP_OE_MASK
            } else {
                self.bits & !Pad120::REG_PAD12_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD12_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad12_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad120::REG_PAD12_SLP_IE_MASK
            } else {
                self.bits & !Pad120::REG_PAD12_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD12_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad12_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad120::REG_PAD12_SLP_SEL_MASK
            } else {
                self.bits & !Pad120::REG_PAD12_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD12_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad12_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad120::REG_PAD12_FUN_SEL_MASK)
                | ((value << Pad120::REG_PAD12_FUN_SEL_OFFSET) & Pad120::REG_PAD12_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD12_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad12_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad120::REG_PAD12_MUX_SEL_MASK
            } else {
                self.bits & !Pad120::REG_PAD12_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD12_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad12_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad120::REG_PAD12_RUE_MASK
            } else {
                self.bits & !Pad120::REG_PAD12_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD12_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad12_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad120::REG_PAD12_RDE_MASK
            } else {
                self.bits & !Pad120::REG_PAD12_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD12_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad12_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad120::REG_PAD12_DRV_MASK)
                | ((value << Pad120::REG_PAD12_DRV_OFFSET) & Pad120::REG_PAD12_DRV_MASK),
        }
    }
}

impl Default for Pad120Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad120 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad120Val {
        Pad120Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad120Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad120Val) -> Pad120Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD13 Register
///
/// Reserved
///
/// Address offset: 0x003c
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD13_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD13_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD13_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD13_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD13_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD13_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD13_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD13_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD13_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD13_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad13 {
    // ======== Field Constants ========

    /// **REG_PAD13_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD13_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD13_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD13_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD13_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD13_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD13_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD13_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD13_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD13_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD13_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD13_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD13_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD13_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD13_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD13_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD13_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD13_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD13_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD13_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD13_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD13_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD13_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD13_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD13_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD13_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD13_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD13_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD13_RUE_OFFSET: u32 = 3;
    pub const REG_PAD13_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD13_RUE: u32 = 1 << 3;

    /// **REG_PAD13_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD13_RDE_OFFSET: u32 = 2;
    pub const REG_PAD13_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD13_RDE: u32 = 1 << 2;

    /// **REG_PAD13_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD13_DRV_OFFSET: u32 = 0;
    pub const REG_PAD13_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD13_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad13_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD13_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD13_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad13_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD13_FILTER_EN_MASK);
    }

    /// Clear REG_PAD13_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad13_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD13_FILTER_EN_MASK);
    }

    /// **REG_PAD13_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad13_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD13_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD13_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad13_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD13_FUN_IE_MASK);
    }

    /// Clear REG_PAD13_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad13_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD13_FUN_IE_MASK);
    }

    /// **REG_PAD13_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad13_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD13_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD13_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad13_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD13_SLP_OE_MASK);
    }

    /// Clear REG_PAD13_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad13_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD13_SLP_OE_MASK);
    }

    /// **REG_PAD13_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad13_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD13_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD13_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad13_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD13_SLP_IE_MASK);
    }

    /// Clear REG_PAD13_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad13_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD13_SLP_IE_MASK);
    }

    /// **REG_PAD13_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad13_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD13_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD13_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad13_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD13_SLP_SEL_MASK);
    }

    /// Clear REG_PAD13_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad13_slp_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD13_SLP_SEL_MASK);
    }

    /// **REG_PAD13_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad13_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD13_FUN_SEL_MASK) >> Self::REG_PAD13_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD13_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad13_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD13_FUN_SEL_MASK)
                | ((value << Self::REG_PAD13_FUN_SEL_OFFSET) & Self::REG_PAD13_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD13_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad13_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD13_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD13_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad13_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD13_MUX_SEL_MASK);
    }

    /// Clear REG_PAD13_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad13_mux_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD13_MUX_SEL_MASK);
    }

    /// **REG_PAD13_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad13_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD13_RUE_MASK) != 0
    }

    /// Set REG_PAD13_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad13_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD13_RUE_MASK);
    }

    /// Clear REG_PAD13_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad13_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD13_RUE_MASK);
    }

    /// **REG_PAD13_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad13_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD13_RDE_MASK) != 0
    }

    /// Set REG_PAD13_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad13_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD13_RDE_MASK);
    }

    /// Clear REG_PAD13_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad13_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD13_RDE_MASK);
    }

    /// **REG_PAD13_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad13_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD13_DRV_MASK) >> Self::REG_PAD13_DRV_OFFSET
    }

    /// Write **REG_PAD13_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad13_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD13_DRV_MASK)
                | ((value << Self::REG_PAD13_DRV_OFFSET) & Self::REG_PAD13_DRV_MASK)
        });
    }
}

/// PAD13 value type for batch read/modify/write operations
///
/// Use `pad13.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad13.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad13Val {
    bits: u32,
}

impl Pad13Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD13_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad13_filter_en(&self) -> bool {
        (self.bits & Pad13::REG_PAD13_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD13_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad13_fun_ie(&self) -> bool {
        (self.bits & Pad13::REG_PAD13_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD13_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad13_slp_oe(&self) -> bool {
        (self.bits & Pad13::REG_PAD13_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD13_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad13_slp_ie(&self) -> bool {
        (self.bits & Pad13::REG_PAD13_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD13_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad13_slp_sel(&self) -> bool {
        (self.bits & Pad13::REG_PAD13_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD13_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad13_fun_sel(&self) -> u32 {
        (self.bits & Pad13::REG_PAD13_FUN_SEL_MASK) >> Pad13::REG_PAD13_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD13_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad13_mux_sel(&self) -> bool {
        (self.bits & Pad13::REG_PAD13_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD13_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad13_rue(&self) -> bool {
        (self.bits & Pad13::REG_PAD13_RUE_MASK) != 0
    }

    /// Read **REG_PAD13_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad13_rde(&self) -> bool {
        (self.bits & Pad13::REG_PAD13_RDE_MASK) != 0
    }

    /// Read **REG_PAD13_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad13_drv(&self) -> u32 {
        (self.bits & Pad13::REG_PAD13_DRV_MASK) >> Pad13::REG_PAD13_DRV_OFFSET
    }

    /// Set **REG_PAD13_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad13_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad13::REG_PAD13_FILTER_EN_MASK
            } else {
                self.bits & !Pad13::REG_PAD13_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD13_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad13_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad13::REG_PAD13_FUN_IE_MASK
            } else {
                self.bits & !Pad13::REG_PAD13_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD13_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad13_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad13::REG_PAD13_SLP_OE_MASK
            } else {
                self.bits & !Pad13::REG_PAD13_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD13_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad13_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad13::REG_PAD13_SLP_IE_MASK
            } else {
                self.bits & !Pad13::REG_PAD13_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD13_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad13_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad13::REG_PAD13_SLP_SEL_MASK
            } else {
                self.bits & !Pad13::REG_PAD13_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD13_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad13_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad13::REG_PAD13_FUN_SEL_MASK)
                | ((value << Pad13::REG_PAD13_FUN_SEL_OFFSET) & Pad13::REG_PAD13_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD13_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad13_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad13::REG_PAD13_MUX_SEL_MASK
            } else {
                self.bits & !Pad13::REG_PAD13_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD13_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad13_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad13::REG_PAD13_RUE_MASK
            } else {
                self.bits & !Pad13::REG_PAD13_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD13_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad13_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad13::REG_PAD13_RDE_MASK
            } else {
                self.bits & !Pad13::REG_PAD13_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD13_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad13_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad13::REG_PAD13_DRV_MASK)
                | ((value << Pad13::REG_PAD13_DRV_OFFSET) & Pad13::REG_PAD13_DRV_MASK),
        }
    }
}

impl Default for Pad13Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad13 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad13Val {
        Pad13Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad13Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad13Val) -> Pad13Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD14 Register
///
/// Reserved
///
/// Address offset: 0x0040
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD14_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD14_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD14_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD14_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD14_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD14_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD14_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD14_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD14_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD14_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad14 {
    // ======== Field Constants ========

    /// **REG_PAD14_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD14_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD14_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD14_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD14_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD14_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD14_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD14_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD14_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD14_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD14_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD14_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD14_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD14_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD14_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD14_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD14_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD14_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD14_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD14_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD14_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD14_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD14_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD14_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD14_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD14_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD14_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD14_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD14_RUE_OFFSET: u32 = 3;
    pub const REG_PAD14_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD14_RUE: u32 = 1 << 3;

    /// **REG_PAD14_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD14_RDE_OFFSET: u32 = 2;
    pub const REG_PAD14_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD14_RDE: u32 = 1 << 2;

    /// **REG_PAD14_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD14_DRV_OFFSET: u32 = 0;
    pub const REG_PAD14_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD14_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad14_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD14_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD14_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad14_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD14_FILTER_EN_MASK);
    }

    /// Clear REG_PAD14_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad14_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD14_FILTER_EN_MASK);
    }

    /// **REG_PAD14_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad14_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD14_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD14_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad14_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD14_FUN_IE_MASK);
    }

    /// Clear REG_PAD14_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad14_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD14_FUN_IE_MASK);
    }

    /// **REG_PAD14_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad14_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD14_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD14_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad14_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD14_SLP_OE_MASK);
    }

    /// Clear REG_PAD14_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad14_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD14_SLP_OE_MASK);
    }

    /// **REG_PAD14_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad14_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD14_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD14_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad14_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD14_SLP_IE_MASK);
    }

    /// Clear REG_PAD14_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad14_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD14_SLP_IE_MASK);
    }

    /// **REG_PAD14_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad14_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD14_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD14_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad14_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD14_SLP_SEL_MASK);
    }

    /// Clear REG_PAD14_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad14_slp_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD14_SLP_SEL_MASK);
    }

    /// **REG_PAD14_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad14_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD14_FUN_SEL_MASK) >> Self::REG_PAD14_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD14_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad14_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD14_FUN_SEL_MASK)
                | ((value << Self::REG_PAD14_FUN_SEL_OFFSET) & Self::REG_PAD14_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD14_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad14_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD14_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD14_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad14_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD14_MUX_SEL_MASK);
    }

    /// Clear REG_PAD14_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad14_mux_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD14_MUX_SEL_MASK);
    }

    /// **REG_PAD14_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad14_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD14_RUE_MASK) != 0
    }

    /// Set REG_PAD14_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad14_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD14_RUE_MASK);
    }

    /// Clear REG_PAD14_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad14_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD14_RUE_MASK);
    }

    /// **REG_PAD14_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad14_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD14_RDE_MASK) != 0
    }

    /// Set REG_PAD14_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad14_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD14_RDE_MASK);
    }

    /// Clear REG_PAD14_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad14_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD14_RDE_MASK);
    }

    /// **REG_PAD14_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad14_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD14_DRV_MASK) >> Self::REG_PAD14_DRV_OFFSET
    }

    /// Write **REG_PAD14_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad14_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD14_DRV_MASK)
                | ((value << Self::REG_PAD14_DRV_OFFSET) & Self::REG_PAD14_DRV_MASK)
        });
    }
}

/// PAD14 value type for batch read/modify/write operations
///
/// Use `pad14.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad14.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad14Val {
    bits: u32,
}

impl Pad14Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD14_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad14_filter_en(&self) -> bool {
        (self.bits & Pad14::REG_PAD14_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD14_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad14_fun_ie(&self) -> bool {
        (self.bits & Pad14::REG_PAD14_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD14_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad14_slp_oe(&self) -> bool {
        (self.bits & Pad14::REG_PAD14_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD14_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad14_slp_ie(&self) -> bool {
        (self.bits & Pad14::REG_PAD14_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD14_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad14_slp_sel(&self) -> bool {
        (self.bits & Pad14::REG_PAD14_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD14_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad14_fun_sel(&self) -> u32 {
        (self.bits & Pad14::REG_PAD14_FUN_SEL_MASK) >> Pad14::REG_PAD14_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD14_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad14_mux_sel(&self) -> bool {
        (self.bits & Pad14::REG_PAD14_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD14_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad14_rue(&self) -> bool {
        (self.bits & Pad14::REG_PAD14_RUE_MASK) != 0
    }

    /// Read **REG_PAD14_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad14_rde(&self) -> bool {
        (self.bits & Pad14::REG_PAD14_RDE_MASK) != 0
    }

    /// Read **REG_PAD14_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad14_drv(&self) -> u32 {
        (self.bits & Pad14::REG_PAD14_DRV_MASK) >> Pad14::REG_PAD14_DRV_OFFSET
    }

    /// Set **REG_PAD14_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad14_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad14::REG_PAD14_FILTER_EN_MASK
            } else {
                self.bits & !Pad14::REG_PAD14_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD14_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad14_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad14::REG_PAD14_FUN_IE_MASK
            } else {
                self.bits & !Pad14::REG_PAD14_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD14_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad14_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad14::REG_PAD14_SLP_OE_MASK
            } else {
                self.bits & !Pad14::REG_PAD14_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD14_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad14_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad14::REG_PAD14_SLP_IE_MASK
            } else {
                self.bits & !Pad14::REG_PAD14_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD14_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad14_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad14::REG_PAD14_SLP_SEL_MASK
            } else {
                self.bits & !Pad14::REG_PAD14_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD14_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad14_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad14::REG_PAD14_FUN_SEL_MASK)
                | ((value << Pad14::REG_PAD14_FUN_SEL_OFFSET) & Pad14::REG_PAD14_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD14_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad14_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad14::REG_PAD14_MUX_SEL_MASK
            } else {
                self.bits & !Pad14::REG_PAD14_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD14_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad14_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad14::REG_PAD14_RUE_MASK
            } else {
                self.bits & !Pad14::REG_PAD14_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD14_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad14_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad14::REG_PAD14_RDE_MASK
            } else {
                self.bits & !Pad14::REG_PAD14_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD14_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad14_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad14::REG_PAD14_DRV_MASK)
                | ((value << Pad14::REG_PAD14_DRV_OFFSET) & Pad14::REG_PAD14_DRV_MASK),
        }
    }
}

impl Default for Pad14Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad14 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad14Val {
        Pad14Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad14Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad14Val) -> Pad14Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD15 Register
///
/// Reserved
///
/// Address offset: 0x0044
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_PAD15_FILTER_EN** `[11]` (RW)
///
///   need des
///
/// - **REG_PAD15_FUN_IE** `[10]` (RW)
///
///   input enable in work mode
///
/// - **REG_PAD15_SLP_OE** `[9]` (RW)
///
///   output enable in sleep mode
///
/// - **REG_PAD15_SLP_IE** `[8]` (RW)
///
///   input enable in sleep mode
///
/// - **REG_PAD15_SLP_SEL** `[7]` (RW)
///
///   1: enable sleep mode during sleep,0: no sleep mode
///
/// - **REG_PAD15_FUN_SEL** `[6:5]` (RW)
///
///   function sel
///
/// - **REG_PAD15_MUX_SEL** `[4]` (RW)
///
///   1:use LP GPIO,0: use digital GPIO
///
/// - **REG_PAD15_RUE** `[3]` (RW)
///
///   Reserved
///
/// - **REG_PAD15_RDE** `[2]` (RW)
///
///   Reserved
///
/// - **REG_PAD15_DRV** `[1:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct Pad15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Pad15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Pad15 {
    // ======== Field Constants ========

    /// **REG_PAD15_FILTER_EN**
    ///
    /// need des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_PAD15_FILTER_EN_OFFSET: u32 = 11;
    pub const REG_PAD15_FILTER_EN_MASK: u32 = 0x00000800;
    pub const REG_PAD15_FILTER_EN: u32 = 1 << 11;

    /// **REG_PAD15_FUN_IE**
    ///
    /// input enable in work mode
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_PAD15_FUN_IE_OFFSET: u32 = 10;
    pub const REG_PAD15_FUN_IE_MASK: u32 = 0x00000400;
    pub const REG_PAD15_FUN_IE: u32 = 1 << 10;

    /// **REG_PAD15_SLP_OE**
    ///
    /// output enable in sleep mode
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_PAD15_SLP_OE_OFFSET: u32 = 9;
    pub const REG_PAD15_SLP_OE_MASK: u32 = 0x00000200;
    pub const REG_PAD15_SLP_OE: u32 = 1 << 9;

    /// **REG_PAD15_SLP_IE**
    ///
    /// input enable in sleep mode
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_PAD15_SLP_IE_OFFSET: u32 = 8;
    pub const REG_PAD15_SLP_IE_MASK: u32 = 0x00000100;
    pub const REG_PAD15_SLP_IE: u32 = 1 << 8;

    /// **REG_PAD15_SLP_SEL**
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_PAD15_SLP_SEL_OFFSET: u32 = 7;
    pub const REG_PAD15_SLP_SEL_MASK: u32 = 0x00000080;
    pub const REG_PAD15_SLP_SEL: u32 = 1 << 7;

    /// **REG_PAD15_FUN_SEL**
    ///
    /// function sel
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const REG_PAD15_FUN_SEL_OFFSET: u32 = 5;
    pub const REG_PAD15_FUN_SEL_MASK: u32 = 0x00000060;

    /// **REG_PAD15_MUX_SEL**
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_PAD15_MUX_SEL_OFFSET: u32 = 4;
    pub const REG_PAD15_MUX_SEL_MASK: u32 = 0x00000010;
    pub const REG_PAD15_MUX_SEL: u32 = 1 << 4;

    /// **REG_PAD15_RUE**
    ///
    /// Reserved
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_PAD15_RUE_OFFSET: u32 = 3;
    pub const REG_PAD15_RUE_MASK: u32 = 0x00000008;
    pub const REG_PAD15_RUE: u32 = 1 << 3;

    /// **REG_PAD15_RDE**
    ///
    /// Reserved
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_PAD15_RDE_OFFSET: u32 = 2;
    pub const REG_PAD15_RDE_MASK: u32 = 0x00000004;
    pub const REG_PAD15_RDE: u32 = 1 << 2;

    /// **REG_PAD15_DRV**
    ///
    /// Reserved
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_PAD15_DRV_OFFSET: u32 = 0;
    pub const REG_PAD15_DRV_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_PAD15_FILTER_EN** `[11]` (RW)
    ///
    /// need des
    #[inline(always)]
    pub fn reg_pad15_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD15_FILTER_EN_MASK) != 0
    }

    /// Set REG_PAD15_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad15_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PAD15_FILTER_EN_MASK);
    }

    /// Clear REG_PAD15_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad15_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD15_FILTER_EN_MASK);
    }

    /// **REG_PAD15_FUN_IE** `[10]` (RW)
    ///
    /// input enable in work mode
    #[inline(always)]
    pub fn reg_pad15_fun_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD15_FUN_IE_MASK) != 0
    }

    /// Set REG_PAD15_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad15_fun_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD15_FUN_IE_MASK);
    }

    /// Clear REG_PAD15_FUN_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad15_fun_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD15_FUN_IE_MASK);
    }

    /// **REG_PAD15_SLP_OE** `[9]` (RW)
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad15_slp_oe(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD15_SLP_OE_MASK) != 0
    }

    /// Set REG_PAD15_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad15_slp_oe(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD15_SLP_OE_MASK);
    }

    /// Clear REG_PAD15_SLP_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad15_slp_oe(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD15_SLP_OE_MASK);
    }

    /// **REG_PAD15_SLP_IE** `[8]` (RW)
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad15_slp_ie(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD15_SLP_IE_MASK) != 0
    }

    /// Set REG_PAD15_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad15_slp_ie(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD15_SLP_IE_MASK);
    }

    /// Clear REG_PAD15_SLP_IE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad15_slp_ie(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD15_SLP_IE_MASK);
    }

    /// **REG_PAD15_SLP_SEL** `[7]` (RW)
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad15_slp_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD15_SLP_SEL_MASK) != 0
    }

    /// Set REG_PAD15_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad15_slp_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD15_SLP_SEL_MASK);
    }

    /// Clear REG_PAD15_SLP_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad15_slp_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD15_SLP_SEL_MASK);
    }

    /// **REG_PAD15_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn reg_pad15_fun_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD15_FUN_SEL_MASK) >> Self::REG_PAD15_FUN_SEL_OFFSET
    }

    /// Write **REG_PAD15_FUN_SEL** `[6:5]` (RW)
    ///
    /// function sel
    #[inline(always)]
    pub fn write_reg_pad15_fun_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD15_FUN_SEL_MASK)
                | ((value << Self::REG_PAD15_FUN_SEL_OFFSET) & Self::REG_PAD15_FUN_SEL_MASK)
        });
    }

    /// **REG_PAD15_MUX_SEL** `[4]` (RW)
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad15_mux_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD15_MUX_SEL_MASK) != 0
    }

    /// Set REG_PAD15_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad15_mux_sel(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD15_MUX_SEL_MASK);
    }

    /// Clear REG_PAD15_MUX_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad15_mux_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PAD15_MUX_SEL_MASK);
    }

    /// **REG_PAD15_RUE** `[3]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad15_rue(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD15_RUE_MASK) != 0
    }

    /// Set REG_PAD15_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad15_rue(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD15_RUE_MASK);
    }

    /// Clear REG_PAD15_RUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad15_rue(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD15_RUE_MASK);
    }

    /// **REG_PAD15_RDE** `[2]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad15_rde(&self) -> bool {
        (self.value.read_bits() & Self::REG_PAD15_RDE_MASK) != 0
    }

    /// Set REG_PAD15_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_pad15_rde(&self) {
        self.value.modify_bits(|r| r | Self::REG_PAD15_RDE_MASK);
    }

    /// Clear REG_PAD15_RDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_pad15_rde(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PAD15_RDE_MASK);
    }

    /// **REG_PAD15_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_pad15_drv(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PAD15_DRV_MASK) >> Self::REG_PAD15_DRV_OFFSET
    }

    /// Write **REG_PAD15_DRV** `[1:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_pad15_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PAD15_DRV_MASK)
                | ((value << Self::REG_PAD15_DRV_OFFSET) & Self::REG_PAD15_DRV_MASK)
        });
    }
}

/// PAD15 value type for batch read/modify/write operations
///
/// Use `pad15.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pad15.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Pad15Val {
    bits: u32,
}

impl Pad15Val {
    /// Create from raw bits
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
        Self { bits: 0x00000002 }
    }

    /// Read **REG_PAD15_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn reg_pad15_filter_en(&self) -> bool {
        (self.bits & Pad15::REG_PAD15_FILTER_EN_MASK) != 0
    }

    /// Read **REG_PAD15_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn reg_pad15_fun_ie(&self) -> bool {
        (self.bits & Pad15::REG_PAD15_FUN_IE_MASK) != 0
    }

    /// Read **REG_PAD15_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad15_slp_oe(&self) -> bool {
        (self.bits & Pad15::REG_PAD15_SLP_OE_MASK) != 0
    }

    /// Read **REG_PAD15_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn reg_pad15_slp_ie(&self) -> bool {
        (self.bits & Pad15::REG_PAD15_SLP_IE_MASK) != 0
    }

    /// Read **REG_PAD15_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn reg_pad15_slp_sel(&self) -> bool {
        (self.bits & Pad15::REG_PAD15_SLP_SEL_MASK) != 0
    }

    /// Read **REG_PAD15_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn reg_pad15_fun_sel(&self) -> u32 {
        (self.bits & Pad15::REG_PAD15_FUN_SEL_MASK) >> Pad15::REG_PAD15_FUN_SEL_OFFSET
    }

    /// Read **REG_PAD15_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn reg_pad15_mux_sel(&self) -> bool {
        (self.bits & Pad15::REG_PAD15_MUX_SEL_MASK) != 0
    }

    /// Read **REG_PAD15_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad15_rue(&self) -> bool {
        (self.bits & Pad15::REG_PAD15_RUE_MASK) != 0
    }

    /// Read **REG_PAD15_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad15_rde(&self) -> bool {
        (self.bits & Pad15::REG_PAD15_RDE_MASK) != 0
    }

    /// Read **REG_PAD15_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_pad15_drv(&self) -> u32 {
        (self.bits & Pad15::REG_PAD15_DRV_MASK) >> Pad15::REG_PAD15_DRV_OFFSET
    }

    /// Set **REG_PAD15_FILTER_EN** `[11:11]`
    ///
    /// need des
    #[inline(always)]
    pub const fn with_reg_pad15_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad15::REG_PAD15_FILTER_EN_MASK
            } else {
                self.bits & !Pad15::REG_PAD15_FILTER_EN_MASK
            },
        }
    }

    /// Set **REG_PAD15_FUN_IE** `[10:10]`
    ///
    /// input enable in work mode
    #[inline(always)]
    pub const fn with_reg_pad15_fun_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad15::REG_PAD15_FUN_IE_MASK
            } else {
                self.bits & !Pad15::REG_PAD15_FUN_IE_MASK
            },
        }
    }

    /// Set **REG_PAD15_SLP_OE** `[9:9]`
    ///
    /// output enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad15_slp_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad15::REG_PAD15_SLP_OE_MASK
            } else {
                self.bits & !Pad15::REG_PAD15_SLP_OE_MASK
            },
        }
    }

    /// Set **REG_PAD15_SLP_IE** `[8:8]`
    ///
    /// input enable in sleep mode
    #[inline(always)]
    pub const fn with_reg_pad15_slp_ie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad15::REG_PAD15_SLP_IE_MASK
            } else {
                self.bits & !Pad15::REG_PAD15_SLP_IE_MASK
            },
        }
    }

    /// Set **REG_PAD15_SLP_SEL** `[7:7]`
    ///
    /// 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub const fn with_reg_pad15_slp_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad15::REG_PAD15_SLP_SEL_MASK
            } else {
                self.bits & !Pad15::REG_PAD15_SLP_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD15_FUN_SEL** `[6:5]`
    ///
    /// function sel
    #[inline(always)]
    pub const fn with_reg_pad15_fun_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad15::REG_PAD15_FUN_SEL_MASK)
                | ((value << Pad15::REG_PAD15_FUN_SEL_OFFSET) & Pad15::REG_PAD15_FUN_SEL_MASK),
        }
    }

    /// Set **REG_PAD15_MUX_SEL** `[4:4]`
    ///
    /// 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub const fn with_reg_pad15_mux_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad15::REG_PAD15_MUX_SEL_MASK
            } else {
                self.bits & !Pad15::REG_PAD15_MUX_SEL_MASK
            },
        }
    }

    /// Set **REG_PAD15_RUE** `[3:3]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad15_rue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad15::REG_PAD15_RUE_MASK
            } else {
                self.bits & !Pad15::REG_PAD15_RUE_MASK
            },
        }
    }

    /// Set **REG_PAD15_RDE** `[2:2]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad15_rde(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Pad15::REG_PAD15_RDE_MASK
            } else {
                self.bits & !Pad15::REG_PAD15_RDE_MASK
            },
        }
    }

    /// Set **REG_PAD15_DRV** `[1:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_pad15_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Pad15::REG_PAD15_DRV_MASK)
                | ((value << Pad15::REG_PAD15_DRV_OFFSET) & Pad15::REG_PAD15_DRV_MASK),
        }
    }
}

impl Default for Pad15Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Pad15 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Pad15Val {
        Pad15Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Pad15Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Pad15Val) -> Pad15Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EXT_WAKEUP0_SEL Register
///
/// Reserved
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **REG_EXT_WAKEUP0_SEL** `[9:5]` (RW)
///
///   Reserved
///
/// - **REG_XTL_EXT_CTR_SEL** `[4:0]` (RW)
///
///   select LP GPIO 0 ~ 15 to control XTAL
#[repr(transparent)]
pub struct ExtWakeup0Sel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ExtWakeup0Sel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ExtWakeup0Sel {
    // ======== Field Constants ========

    /// **REG_EXT_WAKEUP0_SEL**
    ///
    /// Reserved
    ///
    /// Bits `[9:5]`, mask `0x000003e0`
    pub const REG_EXT_WAKEUP0_SEL_OFFSET: u32 = 5;
    pub const REG_EXT_WAKEUP0_SEL_MASK: u32 = 0x000003e0;

    /// **REG_XTL_EXT_CTR_SEL**
    ///
    /// select LP GPIO 0 ~ 15 to control XTAL
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const REG_XTL_EXT_CTR_SEL_OFFSET: u32 = 0;
    pub const REG_XTL_EXT_CTR_SEL_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **REG_EXT_WAKEUP0_SEL** `[9:5]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_ext_wakeup0_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_EXT_WAKEUP0_SEL_MASK)
            >> Self::REG_EXT_WAKEUP0_SEL_OFFSET
    }

    /// Write **REG_EXT_WAKEUP0_SEL** `[9:5]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_ext_wakeup0_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_EXT_WAKEUP0_SEL_MASK)
                | ((value << Self::REG_EXT_WAKEUP0_SEL_OFFSET) & Self::REG_EXT_WAKEUP0_SEL_MASK)
        });
    }

    /// **REG_XTL_EXT_CTR_SEL** `[4:0]` (RW)
    ///
    /// select LP GPIO 0 ~ 15 to control XTAL
    #[inline(always)]
    pub fn reg_xtl_ext_ctr_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_XTL_EXT_CTR_SEL_MASK)
            >> Self::REG_XTL_EXT_CTR_SEL_OFFSET
    }

    /// Write **REG_XTL_EXT_CTR_SEL** `[4:0]` (RW)
    ///
    /// select LP GPIO 0 ~ 15 to control XTAL
    #[inline(always)]
    pub fn write_reg_xtl_ext_ctr_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_XTL_EXT_CTR_SEL_MASK)
                | ((value << Self::REG_XTL_EXT_CTR_SEL_OFFSET) & Self::REG_XTL_EXT_CTR_SEL_MASK)
        });
    }
}

/// EXT_WAKEUP0_SEL value type for batch read/modify/write operations
///
/// Use `extwakeup0sel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `extwakeup0sel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ExtWakeup0SelVal {
    bits: u32,
}

impl ExtWakeup0SelVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_EXT_WAKEUP0_SEL** `[9:5]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_ext_wakeup0_sel(&self) -> u32 {
        (self.bits & ExtWakeup0Sel::REG_EXT_WAKEUP0_SEL_MASK)
            >> ExtWakeup0Sel::REG_EXT_WAKEUP0_SEL_OFFSET
    }

    /// Read **REG_XTL_EXT_CTR_SEL** `[4:0]`
    ///
    /// select LP GPIO 0 ~ 15 to control XTAL
    #[inline(always)]
    pub const fn reg_xtl_ext_ctr_sel(&self) -> u32 {
        (self.bits & ExtWakeup0Sel::REG_XTL_EXT_CTR_SEL_MASK)
            >> ExtWakeup0Sel::REG_XTL_EXT_CTR_SEL_OFFSET
    }

    /// Set **REG_EXT_WAKEUP0_SEL** `[9:5]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_ext_wakeup0_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExtWakeup0Sel::REG_EXT_WAKEUP0_SEL_MASK)
                | ((value << ExtWakeup0Sel::REG_EXT_WAKEUP0_SEL_OFFSET)
                    & ExtWakeup0Sel::REG_EXT_WAKEUP0_SEL_MASK),
        }
    }

    /// Set **REG_XTL_EXT_CTR_SEL** `[4:0]`
    ///
    /// select LP GPIO 0 ~ 15 to control XTAL
    #[inline(always)]
    pub const fn with_reg_xtl_ext_ctr_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExtWakeup0Sel::REG_XTL_EXT_CTR_SEL_MASK)
                | ((value << ExtWakeup0Sel::REG_XTL_EXT_CTR_SEL_OFFSET)
                    & ExtWakeup0Sel::REG_XTL_EXT_CTR_SEL_MASK),
        }
    }
}

impl Default for ExtWakeup0SelVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ExtWakeup0Sel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ExtWakeup0SelVal {
        ExtWakeup0SelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ExtWakeup0SelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ExtWakeup0SelVal) -> ExtWakeup0SelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_PAD_HOLD Register
///
/// Reserved
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **REG_LP_GPIO_HOLD** `[15:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpPadHold {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpPadHold {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpPadHold {
    // ======== Field Constants ========

    /// **REG_LP_GPIO_HOLD**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_LP_GPIO_HOLD_OFFSET: u32 = 0;
    pub const REG_LP_GPIO_HOLD_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **REG_LP_GPIO_HOLD** `[15:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_lp_gpio_hold(&self) -> u32 {
        (self.value.read_bits() & Self::REG_LP_GPIO_HOLD_MASK) >> Self::REG_LP_GPIO_HOLD_OFFSET
    }

    /// Write **REG_LP_GPIO_HOLD** `[15:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_lp_gpio_hold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_LP_GPIO_HOLD_MASK)
                | ((value << Self::REG_LP_GPIO_HOLD_OFFSET) & Self::REG_LP_GPIO_HOLD_MASK)
        });
    }
}

/// LP_PAD_HOLD value type for batch read/modify/write operations
///
/// Use `lppadhold.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lppadhold.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpPadHoldVal {
    bits: u32,
}

impl LpPadHoldVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_LP_GPIO_HOLD** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_lp_gpio_hold(&self) -> u32 {
        (self.bits & LpPadHold::REG_LP_GPIO_HOLD_MASK) >> LpPadHold::REG_LP_GPIO_HOLD_OFFSET
    }

    /// Set **REG_LP_GPIO_HOLD** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_lp_gpio_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpPadHold::REG_LP_GPIO_HOLD_MASK)
                | ((value << LpPadHold::REG_LP_GPIO_HOLD_OFFSET)
                    & LpPadHold::REG_LP_GPIO_HOLD_MASK),
        }
    }
}

impl Default for LpPadHoldVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpPadHold {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpPadHoldVal {
        LpPadHoldVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpPadHoldVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpPadHoldVal) -> LpPadHoldVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_PAD_HYS Register
///
/// Reserved
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **REG_LP_GPIO_HYS** `[15:0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpPadHys {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpPadHys {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpPadHys {
    // ======== Field Constants ========

    /// **REG_LP_GPIO_HYS**
    ///
    /// Reserved
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_LP_GPIO_HYS_OFFSET: u32 = 0;
    pub const REG_LP_GPIO_HYS_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **REG_LP_GPIO_HYS** `[15:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn reg_lp_gpio_hys(&self) -> u32 {
        (self.value.read_bits() & Self::REG_LP_GPIO_HYS_MASK) >> Self::REG_LP_GPIO_HYS_OFFSET
    }

    /// Write **REG_LP_GPIO_HYS** `[15:0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_reg_lp_gpio_hys(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_LP_GPIO_HYS_MASK)
                | ((value << Self::REG_LP_GPIO_HYS_OFFSET) & Self::REG_LP_GPIO_HYS_MASK)
        });
    }
}

/// LP_PAD_HYS value type for batch read/modify/write operations
///
/// Use `lppadhys.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lppadhys.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpPadHysVal {
    bits: u32,
}

impl LpPadHysVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **REG_LP_GPIO_HYS** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn reg_lp_gpio_hys(&self) -> u32 {
        (self.bits & LpPadHys::REG_LP_GPIO_HYS_MASK) >> LpPadHys::REG_LP_GPIO_HYS_OFFSET
    }

    /// Set **REG_LP_GPIO_HYS** `[15:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_reg_lp_gpio_hys(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpPadHys::REG_LP_GPIO_HYS_MASK)
                | ((value << LpPadHys::REG_LP_GPIO_HYS_OFFSET) & LpPadHys::REG_LP_GPIO_HYS_MASK),
        }
    }
}

impl Default for LpPadHysVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpPadHys {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpPadHysVal {
        LpPadHysVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpPadHysVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpPadHysVal) -> LpPadHysVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// Reserved
    pub clk_en: ClkEn,
    /// 0x0004
    /// Reserved
    pub ver_date: VerDate,
    /// 0x0008
    /// Reserved
    pub pad0: Pad0,
    /// 0x000c
    /// Reserved
    pub pad1: Pad1,
    /// 0x0010
    /// Reserved
    pub pad2: Pad2,
    /// 0x0014
    /// Reserved
    pub pad3: Pad3,
    /// 0x0018
    /// Reserved
    pub pad4: Pad4,
    /// 0x001c
    /// Reserved
    pub pad5: Pad5,
    /// 0x0020
    /// Reserved
    pub pad6: Pad6,
    /// 0x0024
    /// Reserved
    pub pad7: Pad7,
    /// 0x0028
    /// Reserved
    pub pad8: Pad8,
    /// 0x002c
    /// Reserved
    pub pad9: Pad9,
    /// 0x0030
    /// Reserved
    pub pad10: Pad10,
    /// 0x0034
    /// Reserved
    pub pad11: Pad11,
    /// 0x0038
    /// Reserved
    pub pad120: Pad120,
    /// 0x003c
    /// Reserved
    pub pad13: Pad13,
    /// 0x0040
    /// Reserved
    pub pad14: Pad14,
    /// 0x0044
    /// Reserved
    pub pad15: Pad15,
    /// 0x0048
    /// Reserved
    pub ext_wakeup0_sel: ExtWakeup0Sel,
    /// 0x004c
    /// Reserved
    pub lp_pad_hold: LpPadHold,
    /// 0x0050
    /// Reserved
    pub lp_pad_hys: LpPadHys,
}
