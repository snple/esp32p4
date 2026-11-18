//! LP_AON_CLKRST
//! LP_AON_CLKRST Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x50111000;

/// LP_AONCLKRST_LP_CLK_CONF Register
///
/// need_des
///
/// Address offset: 0x0000
/// Reset value: 0x00000004
///
/// ## Fields
///
/// - **LP_AONCLKRST_ANA_SEL_REF_PLL8M** `[10]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_LP_PERI_DIV_NUM** `[9:4]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_FAST_CLK_SEL** `[3:2]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_SLOW_CLK_SEL** `[1:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAonclkrstLpClkConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstLpClkConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstLpClkConf {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_ANA_SEL_REF_PLL8M**
    ///
    /// need_des
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const LP_AONCLKRST_ANA_SEL_REF_PLL8M_OFFSET: u32 = 10;
    pub const LP_AONCLKRST_ANA_SEL_REF_PLL8M_MASK: u32 = 0x00000400;
    pub const LP_AONCLKRST_ANA_SEL_REF_PLL8M: u32 = 1 << 10;

    /// **LP_AONCLKRST_LP_PERI_DIV_NUM**
    ///
    /// need_des
    ///
    /// Bits `[9:4]`, mask `0x000003f0`
    pub const LP_AONCLKRST_LP_PERI_DIV_NUM_OFFSET: u32 = 4;
    pub const LP_AONCLKRST_LP_PERI_DIV_NUM_MASK: u32 = 0x000003f0;

    /// **LP_AONCLKRST_FAST_CLK_SEL**
    ///
    /// need_des
    ///
    /// Bits `[3:2]`, mask `0x0000000c`
    pub const LP_AONCLKRST_FAST_CLK_SEL_OFFSET: u32 = 2;
    pub const LP_AONCLKRST_FAST_CLK_SEL_MASK: u32 = 0x0000000c;

    /// **LP_AONCLKRST_SLOW_CLK_SEL**
    ///
    /// need_des
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const LP_AONCLKRST_SLOW_CLK_SEL_OFFSET: u32 = 0;
    pub const LP_AONCLKRST_SLOW_CLK_SEL_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_ANA_SEL_REF_PLL8M** `[10]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_ana_sel_ref_pll8m(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_ANA_SEL_REF_PLL8M_MASK) != 0
    }

    /// Set LP_AONCLKRST_ANA_SEL_REF_PLL8M bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_ana_sel_ref_pll8m(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_ANA_SEL_REF_PLL8M_MASK);
    }

    /// Clear LP_AONCLKRST_ANA_SEL_REF_PLL8M bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_ana_sel_ref_pll8m(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_ANA_SEL_REF_PLL8M_MASK);
    }

    /// **LP_AONCLKRST_LP_PERI_DIV_NUM** `[9:4]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_lp_peri_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_LP_PERI_DIV_NUM_MASK)
            >> Self::LP_AONCLKRST_LP_PERI_DIV_NUM_OFFSET
    }

    /// Write **LP_AONCLKRST_LP_PERI_DIV_NUM** `[9:4]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_aonclkrst_lp_peri_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_LP_PERI_DIV_NUM_MASK)
                | ((value << Self::LP_AONCLKRST_LP_PERI_DIV_NUM_OFFSET)
                    & Self::LP_AONCLKRST_LP_PERI_DIV_NUM_MASK)
        });
    }

    /// **LP_AONCLKRST_FAST_CLK_SEL** `[3:2]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_fast_clk_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_FAST_CLK_SEL_MASK)
            >> Self::LP_AONCLKRST_FAST_CLK_SEL_OFFSET
    }

    /// Write **LP_AONCLKRST_FAST_CLK_SEL** `[3:2]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_aonclkrst_fast_clk_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_FAST_CLK_SEL_MASK)
                | ((value << Self::LP_AONCLKRST_FAST_CLK_SEL_OFFSET)
                    & Self::LP_AONCLKRST_FAST_CLK_SEL_MASK)
        });
    }

    /// **LP_AONCLKRST_SLOW_CLK_SEL** `[1:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_slow_clk_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_SLOW_CLK_SEL_MASK)
            >> Self::LP_AONCLKRST_SLOW_CLK_SEL_OFFSET
    }

    /// Write **LP_AONCLKRST_SLOW_CLK_SEL** `[1:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_aonclkrst_slow_clk_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_SLOW_CLK_SEL_MASK)
                | ((value << Self::LP_AONCLKRST_SLOW_CLK_SEL_OFFSET)
                    & Self::LP_AONCLKRST_SLOW_CLK_SEL_MASK)
        });
    }
}

/// LP_AONCLKRST_LP_CLK_CONF value type for batch read/modify/write operations
///
/// Use `lpaonclkrstlpclkconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrstlpclkconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstLpClkConfVal {
    bits: u32,
}

impl LpAonclkrstLpClkConfVal {
    /// Create from raw bits
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
        Self { bits: 0x00000004 }
    }

    /// Read **LP_AONCLKRST_ANA_SEL_REF_PLL8M** `[10:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_ana_sel_ref_pll8m(&self) -> bool {
        (self.bits & LpAonclkrstLpClkConf::LP_AONCLKRST_ANA_SEL_REF_PLL8M_MASK) != 0
    }

    /// Read **LP_AONCLKRST_LP_PERI_DIV_NUM** `[9:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_peri_div_num(&self) -> u32 {
        (self.bits & LpAonclkrstLpClkConf::LP_AONCLKRST_LP_PERI_DIV_NUM_MASK)
            >> LpAonclkrstLpClkConf::LP_AONCLKRST_LP_PERI_DIV_NUM_OFFSET
    }

    /// Read **LP_AONCLKRST_FAST_CLK_SEL** `[3:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_fast_clk_sel(&self) -> u32 {
        (self.bits & LpAonclkrstLpClkConf::LP_AONCLKRST_FAST_CLK_SEL_MASK)
            >> LpAonclkrstLpClkConf::LP_AONCLKRST_FAST_CLK_SEL_OFFSET
    }

    /// Read **LP_AONCLKRST_SLOW_CLK_SEL** `[1:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_slow_clk_sel(&self) -> u32 {
        (self.bits & LpAonclkrstLpClkConf::LP_AONCLKRST_SLOW_CLK_SEL_MASK)
            >> LpAonclkrstLpClkConf::LP_AONCLKRST_SLOW_CLK_SEL_OFFSET
    }

    /// Set **LP_AONCLKRST_ANA_SEL_REF_PLL8M** `[10:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_ana_sel_ref_pll8m(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkConf::LP_AONCLKRST_ANA_SEL_REF_PLL8M_MASK
            } else {
                self.bits & !LpAonclkrstLpClkConf::LP_AONCLKRST_ANA_SEL_REF_PLL8M_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_LP_PERI_DIV_NUM** `[9:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_lp_peri_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstLpClkConf::LP_AONCLKRST_LP_PERI_DIV_NUM_MASK)
                | ((value << LpAonclkrstLpClkConf::LP_AONCLKRST_LP_PERI_DIV_NUM_OFFSET)
                    & LpAonclkrstLpClkConf::LP_AONCLKRST_LP_PERI_DIV_NUM_MASK),
        }
    }

    /// Set **LP_AONCLKRST_FAST_CLK_SEL** `[3:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_fast_clk_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstLpClkConf::LP_AONCLKRST_FAST_CLK_SEL_MASK)
                | ((value << LpAonclkrstLpClkConf::LP_AONCLKRST_FAST_CLK_SEL_OFFSET)
                    & LpAonclkrstLpClkConf::LP_AONCLKRST_FAST_CLK_SEL_MASK),
        }
    }

    /// Set **LP_AONCLKRST_SLOW_CLK_SEL** `[1:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_slow_clk_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstLpClkConf::LP_AONCLKRST_SLOW_CLK_SEL_MASK)
                | ((value << LpAonclkrstLpClkConf::LP_AONCLKRST_SLOW_CLK_SEL_OFFSET)
                    & LpAonclkrstLpClkConf::LP_AONCLKRST_SLOW_CLK_SEL_MASK),
        }
    }
}

impl Default for LpAonclkrstLpClkConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstLpClkConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstLpClkConfVal {
        LpAonclkrstLpClkConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstLpClkConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstLpClkConfVal) -> LpAonclkrstLpClkConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_LP_CLK_PO_EN Register
///
/// need_des
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **LP_AONCLKRST_CLK_SOSC_OEN** `[9]` (RW)
///
///   1'b1: probe sosc clk on
///   1'b0: probe sosc clk off
///
/// - **LP_AONCLKRST_CLK_SXTAL_OEN** `[8]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_CLK_RC32K_OEN** `[7]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_CLK_FOSC_OEN** `[6]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_CLK_FAST_OEN** `[5]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_CLK_SLOW_OEN** `[4]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_CLK_AON_FAST_OEN** `[3]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_CLK_AON_SLOW_OEN** `[2]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_CLK_LP_BUS_OEN** `[1]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_CLK_CORE_EFUSE_OEN** `[0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAonclkrstLpClkPoEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstLpClkPoEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstLpClkPoEn {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_CLK_SOSC_OEN**
    ///
    /// 1'b1: probe sosc clk on
    /// 1'b0: probe sosc clk off
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const LP_AONCLKRST_CLK_SOSC_OEN_OFFSET: u32 = 9;
    pub const LP_AONCLKRST_CLK_SOSC_OEN_MASK: u32 = 0x00000200;
    pub const LP_AONCLKRST_CLK_SOSC_OEN: u32 = 1 << 9;

    /// **LP_AONCLKRST_CLK_SXTAL_OEN**
    ///
    /// need_des
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const LP_AONCLKRST_CLK_SXTAL_OEN_OFFSET: u32 = 8;
    pub const LP_AONCLKRST_CLK_SXTAL_OEN_MASK: u32 = 0x00000100;
    pub const LP_AONCLKRST_CLK_SXTAL_OEN: u32 = 1 << 8;

    /// **LP_AONCLKRST_CLK_RC32K_OEN**
    ///
    /// need_des
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const LP_AONCLKRST_CLK_RC32K_OEN_OFFSET: u32 = 7;
    pub const LP_AONCLKRST_CLK_RC32K_OEN_MASK: u32 = 0x00000080;
    pub const LP_AONCLKRST_CLK_RC32K_OEN: u32 = 1 << 7;

    /// **LP_AONCLKRST_CLK_FOSC_OEN**
    ///
    /// need_des
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const LP_AONCLKRST_CLK_FOSC_OEN_OFFSET: u32 = 6;
    pub const LP_AONCLKRST_CLK_FOSC_OEN_MASK: u32 = 0x00000040;
    pub const LP_AONCLKRST_CLK_FOSC_OEN: u32 = 1 << 6;

    /// **LP_AONCLKRST_CLK_FAST_OEN**
    ///
    /// need_des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const LP_AONCLKRST_CLK_FAST_OEN_OFFSET: u32 = 5;
    pub const LP_AONCLKRST_CLK_FAST_OEN_MASK: u32 = 0x00000020;
    pub const LP_AONCLKRST_CLK_FAST_OEN: u32 = 1 << 5;

    /// **LP_AONCLKRST_CLK_SLOW_OEN**
    ///
    /// need_des
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_AONCLKRST_CLK_SLOW_OEN_OFFSET: u32 = 4;
    pub const LP_AONCLKRST_CLK_SLOW_OEN_MASK: u32 = 0x00000010;
    pub const LP_AONCLKRST_CLK_SLOW_OEN: u32 = 1 << 4;

    /// **LP_AONCLKRST_CLK_AON_FAST_OEN**
    ///
    /// need_des
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LP_AONCLKRST_CLK_AON_FAST_OEN_OFFSET: u32 = 3;
    pub const LP_AONCLKRST_CLK_AON_FAST_OEN_MASK: u32 = 0x00000008;
    pub const LP_AONCLKRST_CLK_AON_FAST_OEN: u32 = 1 << 3;

    /// **LP_AONCLKRST_CLK_AON_SLOW_OEN**
    ///
    /// need_des
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const LP_AONCLKRST_CLK_AON_SLOW_OEN_OFFSET: u32 = 2;
    pub const LP_AONCLKRST_CLK_AON_SLOW_OEN_MASK: u32 = 0x00000004;
    pub const LP_AONCLKRST_CLK_AON_SLOW_OEN: u32 = 1 << 2;

    /// **LP_AONCLKRST_CLK_LP_BUS_OEN**
    ///
    /// need_des
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const LP_AONCLKRST_CLK_LP_BUS_OEN_OFFSET: u32 = 1;
    pub const LP_AONCLKRST_CLK_LP_BUS_OEN_MASK: u32 = 0x00000002;
    pub const LP_AONCLKRST_CLK_LP_BUS_OEN: u32 = 1 << 1;

    /// **LP_AONCLKRST_CLK_CORE_EFUSE_OEN**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_AONCLKRST_CLK_CORE_EFUSE_OEN_OFFSET: u32 = 0;
    pub const LP_AONCLKRST_CLK_CORE_EFUSE_OEN_MASK: u32 = 0x00000001;
    pub const LP_AONCLKRST_CLK_CORE_EFUSE_OEN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_CLK_SOSC_OEN** `[9]` (RW)
    ///
    /// 1'b1: probe sosc clk on
    /// 1'b0: probe sosc clk off
    #[inline(always)]
    pub fn lp_aonclkrst_clk_sosc_oen(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_CLK_SOSC_OEN_MASK) != 0
    }

    /// Set LP_AONCLKRST_CLK_SOSC_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_clk_sosc_oen(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_CLK_SOSC_OEN_MASK);
    }

    /// Clear LP_AONCLKRST_CLK_SOSC_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_clk_sosc_oen(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_CLK_SOSC_OEN_MASK);
    }

    /// **LP_AONCLKRST_CLK_SXTAL_OEN** `[8]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_sxtal_oen(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_CLK_SXTAL_OEN_MASK) != 0
    }

    /// Set LP_AONCLKRST_CLK_SXTAL_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_clk_sxtal_oen(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_CLK_SXTAL_OEN_MASK);
    }

    /// Clear LP_AONCLKRST_CLK_SXTAL_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_clk_sxtal_oen(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_CLK_SXTAL_OEN_MASK);
    }

    /// **LP_AONCLKRST_CLK_RC32K_OEN** `[7]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_rc32k_oen(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_CLK_RC32K_OEN_MASK) != 0
    }

    /// Set LP_AONCLKRST_CLK_RC32K_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_clk_rc32k_oen(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_CLK_RC32K_OEN_MASK);
    }

    /// Clear LP_AONCLKRST_CLK_RC32K_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_clk_rc32k_oen(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_CLK_RC32K_OEN_MASK);
    }

    /// **LP_AONCLKRST_CLK_FOSC_OEN** `[6]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_fosc_oen(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_CLK_FOSC_OEN_MASK) != 0
    }

    /// Set LP_AONCLKRST_CLK_FOSC_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_clk_fosc_oen(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_CLK_FOSC_OEN_MASK);
    }

    /// Clear LP_AONCLKRST_CLK_FOSC_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_clk_fosc_oen(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_CLK_FOSC_OEN_MASK);
    }

    /// **LP_AONCLKRST_CLK_FAST_OEN** `[5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_fast_oen(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_CLK_FAST_OEN_MASK) != 0
    }

    /// Set LP_AONCLKRST_CLK_FAST_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_clk_fast_oen(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_CLK_FAST_OEN_MASK);
    }

    /// Clear LP_AONCLKRST_CLK_FAST_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_clk_fast_oen(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_CLK_FAST_OEN_MASK);
    }

    /// **LP_AONCLKRST_CLK_SLOW_OEN** `[4]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_slow_oen(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_CLK_SLOW_OEN_MASK) != 0
    }

    /// Set LP_AONCLKRST_CLK_SLOW_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_clk_slow_oen(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_CLK_SLOW_OEN_MASK);
    }

    /// Clear LP_AONCLKRST_CLK_SLOW_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_clk_slow_oen(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_CLK_SLOW_OEN_MASK);
    }

    /// **LP_AONCLKRST_CLK_AON_FAST_OEN** `[3]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_aon_fast_oen(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_CLK_AON_FAST_OEN_MASK) != 0
    }

    /// Set LP_AONCLKRST_CLK_AON_FAST_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_clk_aon_fast_oen(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_CLK_AON_FAST_OEN_MASK);
    }

    /// Clear LP_AONCLKRST_CLK_AON_FAST_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_clk_aon_fast_oen(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_CLK_AON_FAST_OEN_MASK);
    }

    /// **LP_AONCLKRST_CLK_AON_SLOW_OEN** `[2]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_aon_slow_oen(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_CLK_AON_SLOW_OEN_MASK) != 0
    }

    /// Set LP_AONCLKRST_CLK_AON_SLOW_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_clk_aon_slow_oen(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_CLK_AON_SLOW_OEN_MASK);
    }

    /// Clear LP_AONCLKRST_CLK_AON_SLOW_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_clk_aon_slow_oen(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_CLK_AON_SLOW_OEN_MASK);
    }

    /// **LP_AONCLKRST_CLK_LP_BUS_OEN** `[1]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_lp_bus_oen(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_CLK_LP_BUS_OEN_MASK) != 0
    }

    /// Set LP_AONCLKRST_CLK_LP_BUS_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_clk_lp_bus_oen(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_CLK_LP_BUS_OEN_MASK);
    }

    /// Clear LP_AONCLKRST_CLK_LP_BUS_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_clk_lp_bus_oen(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_CLK_LP_BUS_OEN_MASK);
    }

    /// **LP_AONCLKRST_CLK_CORE_EFUSE_OEN** `[0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_core_efuse_oen(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_CLK_CORE_EFUSE_OEN_MASK) != 0
    }

    /// Set LP_AONCLKRST_CLK_CORE_EFUSE_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_clk_core_efuse_oen(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_CLK_CORE_EFUSE_OEN_MASK);
    }

    /// Clear LP_AONCLKRST_CLK_CORE_EFUSE_OEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_clk_core_efuse_oen(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_CLK_CORE_EFUSE_OEN_MASK);
    }
}

/// LP_AONCLKRST_LP_CLK_PO_EN value type for batch read/modify/write operations
///
/// Use `lpaonclkrstlpclkpoen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrstlpclkpoen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstLpClkPoEnVal {
    bits: u32,
}

impl LpAonclkrstLpClkPoEnVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_AONCLKRST_CLK_SOSC_OEN** `[9:9]`
    ///
    /// 1'b1: probe sosc clk on
    /// 1'b0: probe sosc clk off
    #[inline(always)]
    pub const fn lp_aonclkrst_clk_sosc_oen(&self) -> bool {
        (self.bits & LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_SOSC_OEN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_CLK_SXTAL_OEN** `[8:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_clk_sxtal_oen(&self) -> bool {
        (self.bits & LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_SXTAL_OEN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_CLK_RC32K_OEN** `[7:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_clk_rc32k_oen(&self) -> bool {
        (self.bits & LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_RC32K_OEN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_CLK_FOSC_OEN** `[6:6]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_clk_fosc_oen(&self) -> bool {
        (self.bits & LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_FOSC_OEN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_CLK_FAST_OEN** `[5:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_clk_fast_oen(&self) -> bool {
        (self.bits & LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_FAST_OEN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_CLK_SLOW_OEN** `[4:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_clk_slow_oen(&self) -> bool {
        (self.bits & LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_SLOW_OEN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_CLK_AON_FAST_OEN** `[3:3]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_clk_aon_fast_oen(&self) -> bool {
        (self.bits & LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_AON_FAST_OEN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_CLK_AON_SLOW_OEN** `[2:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_clk_aon_slow_oen(&self) -> bool {
        (self.bits & LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_AON_SLOW_OEN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_CLK_LP_BUS_OEN** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_clk_lp_bus_oen(&self) -> bool {
        (self.bits & LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_LP_BUS_OEN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_CLK_CORE_EFUSE_OEN** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_clk_core_efuse_oen(&self) -> bool {
        (self.bits & LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_CORE_EFUSE_OEN_MASK) != 0
    }

    /// Set **LP_AONCLKRST_CLK_SOSC_OEN** `[9:9]`
    ///
    /// 1'b1: probe sosc clk on
    /// 1'b0: probe sosc clk off
    #[inline(always)]
    pub const fn with_lp_aonclkrst_clk_sosc_oen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_SOSC_OEN_MASK
            } else {
                self.bits & !LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_SOSC_OEN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_CLK_SXTAL_OEN** `[8:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_clk_sxtal_oen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_SXTAL_OEN_MASK
            } else {
                self.bits & !LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_SXTAL_OEN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_CLK_RC32K_OEN** `[7:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_clk_rc32k_oen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_RC32K_OEN_MASK
            } else {
                self.bits & !LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_RC32K_OEN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_CLK_FOSC_OEN** `[6:6]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_clk_fosc_oen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_FOSC_OEN_MASK
            } else {
                self.bits & !LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_FOSC_OEN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_CLK_FAST_OEN** `[5:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_clk_fast_oen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_FAST_OEN_MASK
            } else {
                self.bits & !LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_FAST_OEN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_CLK_SLOW_OEN** `[4:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_clk_slow_oen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_SLOW_OEN_MASK
            } else {
                self.bits & !LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_SLOW_OEN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_CLK_AON_FAST_OEN** `[3:3]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_clk_aon_fast_oen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_AON_FAST_OEN_MASK
            } else {
                self.bits & !LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_AON_FAST_OEN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_CLK_AON_SLOW_OEN** `[2:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_clk_aon_slow_oen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_AON_SLOW_OEN_MASK
            } else {
                self.bits & !LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_AON_SLOW_OEN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_CLK_LP_BUS_OEN** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_clk_lp_bus_oen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_LP_BUS_OEN_MASK
            } else {
                self.bits & !LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_LP_BUS_OEN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_CLK_CORE_EFUSE_OEN** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_clk_core_efuse_oen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_CORE_EFUSE_OEN_MASK
            } else {
                self.bits & !LpAonclkrstLpClkPoEn::LP_AONCLKRST_CLK_CORE_EFUSE_OEN_MASK
            },
        }
    }
}

impl Default for LpAonclkrstLpClkPoEnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAonclkrstLpClkPoEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstLpClkPoEnVal {
        LpAonclkrstLpClkPoEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstLpClkPoEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstLpClkPoEnVal) -> LpAonclkrstLpClkPoEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_LP_CLK_EN Register
///
/// need_des
///
/// Address offset: 0x0008
/// Reset value: 0x08000000
///
/// ## Fields
///
/// - **LP_AONCLKRST_FOSC_CLK_FORCE_ON** `[31]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_XTAL_CLK_FORCE_ON** `[30]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_PLL8M_CLK_FORCE_ON** `[29]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_ETM_EVENT_TICK_EN** `[28]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_CK_EN_LP_RAM** `[27]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON** `[26]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAonclkrstLpClkEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstLpClkEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstLpClkEn {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_FOSC_CLK_FORCE_ON**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_AONCLKRST_FOSC_CLK_FORCE_ON_OFFSET: u32 = 31;
    pub const LP_AONCLKRST_FOSC_CLK_FORCE_ON_MASK: u32 = 0x80000000;
    pub const LP_AONCLKRST_FOSC_CLK_FORCE_ON: u32 = 1 << 31;

    /// **LP_AONCLKRST_XTAL_CLK_FORCE_ON**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_AONCLKRST_XTAL_CLK_FORCE_ON_OFFSET: u32 = 30;
    pub const LP_AONCLKRST_XTAL_CLK_FORCE_ON_MASK: u32 = 0x40000000;
    pub const LP_AONCLKRST_XTAL_CLK_FORCE_ON: u32 = 1 << 30;

    /// **LP_AONCLKRST_PLL8M_CLK_FORCE_ON**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_AONCLKRST_PLL8M_CLK_FORCE_ON_OFFSET: u32 = 29;
    pub const LP_AONCLKRST_PLL8M_CLK_FORCE_ON_MASK: u32 = 0x20000000;
    pub const LP_AONCLKRST_PLL8M_CLK_FORCE_ON: u32 = 1 << 29;

    /// **LP_AONCLKRST_ETM_EVENT_TICK_EN**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_AONCLKRST_ETM_EVENT_TICK_EN_OFFSET: u32 = 28;
    pub const LP_AONCLKRST_ETM_EVENT_TICK_EN_MASK: u32 = 0x10000000;
    pub const LP_AONCLKRST_ETM_EVENT_TICK_EN: u32 = 1 << 28;

    /// **LP_AONCLKRST_CK_EN_LP_RAM**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LP_AONCLKRST_CK_EN_LP_RAM_OFFSET: u32 = 27;
    pub const LP_AONCLKRST_CK_EN_LP_RAM_MASK: u32 = 0x08000000;
    pub const LP_AONCLKRST_CK_EN_LP_RAM: u32 = 1 << 27;

    /// **LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON**
    ///
    /// need_des
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON_OFFSET: u32 = 26;
    pub const LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON_MASK: u32 = 0x04000000;
    pub const LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON: u32 = 1 << 26;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_FOSC_CLK_FORCE_ON** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_fosc_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_FOSC_CLK_FORCE_ON_MASK) != 0
    }

    /// Set LP_AONCLKRST_FOSC_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_fosc_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_FOSC_CLK_FORCE_ON_MASK);
    }

    /// Clear LP_AONCLKRST_FOSC_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_fosc_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_FOSC_CLK_FORCE_ON_MASK);
    }

    /// **LP_AONCLKRST_XTAL_CLK_FORCE_ON** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_xtal_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_XTAL_CLK_FORCE_ON_MASK) != 0
    }

    /// Set LP_AONCLKRST_XTAL_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_xtal_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_XTAL_CLK_FORCE_ON_MASK);
    }

    /// Clear LP_AONCLKRST_XTAL_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_xtal_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_XTAL_CLK_FORCE_ON_MASK);
    }

    /// **LP_AONCLKRST_PLL8M_CLK_FORCE_ON** `[29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_pll8m_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_PLL8M_CLK_FORCE_ON_MASK) != 0
    }

    /// Set LP_AONCLKRST_PLL8M_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_pll8m_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_PLL8M_CLK_FORCE_ON_MASK);
    }

    /// Clear LP_AONCLKRST_PLL8M_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_pll8m_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_PLL8M_CLK_FORCE_ON_MASK);
    }

    /// **LP_AONCLKRST_ETM_EVENT_TICK_EN** `[28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_etm_event_tick_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_ETM_EVENT_TICK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_ETM_EVENT_TICK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_etm_event_tick_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_ETM_EVENT_TICK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_ETM_EVENT_TICK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_etm_event_tick_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_ETM_EVENT_TICK_EN_MASK);
    }

    /// **LP_AONCLKRST_CK_EN_LP_RAM** `[27]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_ck_en_lp_ram(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_CK_EN_LP_RAM_MASK) != 0
    }

    /// Set LP_AONCLKRST_CK_EN_LP_RAM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_ck_en_lp_ram(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_CK_EN_LP_RAM_MASK);
    }

    /// Clear LP_AONCLKRST_CK_EN_LP_RAM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_ck_en_lp_ram(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_CK_EN_LP_RAM_MASK);
    }

    /// **LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON** `[26]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_lp_rtc_xtal_force_on(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON_MASK) != 0
    }

    /// Set LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_lp_rtc_xtal_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON_MASK);
    }

    /// Clear LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_lp_rtc_xtal_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON_MASK);
    }
}

/// LP_AONCLKRST_LP_CLK_EN value type for batch read/modify/write operations
///
/// Use `lpaonclkrstlpclken.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrstlpclken.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstLpClkEnVal {
    bits: u32,
}

impl LpAonclkrstLpClkEnVal {
    /// Create from raw bits
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
        Self { bits: 0x08000000 }
    }

    /// Read **LP_AONCLKRST_FOSC_CLK_FORCE_ON** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_fosc_clk_force_on(&self) -> bool {
        (self.bits & LpAonclkrstLpClkEn::LP_AONCLKRST_FOSC_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **LP_AONCLKRST_XTAL_CLK_FORCE_ON** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_xtal_clk_force_on(&self) -> bool {
        (self.bits & LpAonclkrstLpClkEn::LP_AONCLKRST_XTAL_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **LP_AONCLKRST_PLL8M_CLK_FORCE_ON** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_pll8m_clk_force_on(&self) -> bool {
        (self.bits & LpAonclkrstLpClkEn::LP_AONCLKRST_PLL8M_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **LP_AONCLKRST_ETM_EVENT_TICK_EN** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_etm_event_tick_en(&self) -> bool {
        (self.bits & LpAonclkrstLpClkEn::LP_AONCLKRST_ETM_EVENT_TICK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_CK_EN_LP_RAM** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_ck_en_lp_ram(&self) -> bool {
        (self.bits & LpAonclkrstLpClkEn::LP_AONCLKRST_CK_EN_LP_RAM_MASK) != 0
    }

    /// Read **LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_rtc_xtal_force_on(&self) -> bool {
        (self.bits & LpAonclkrstLpClkEn::LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON_MASK) != 0
    }

    /// Set **LP_AONCLKRST_FOSC_CLK_FORCE_ON** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_fosc_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkEn::LP_AONCLKRST_FOSC_CLK_FORCE_ON_MASK
            } else {
                self.bits & !LpAonclkrstLpClkEn::LP_AONCLKRST_FOSC_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_XTAL_CLK_FORCE_ON** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_xtal_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkEn::LP_AONCLKRST_XTAL_CLK_FORCE_ON_MASK
            } else {
                self.bits & !LpAonclkrstLpClkEn::LP_AONCLKRST_XTAL_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_PLL8M_CLK_FORCE_ON** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_pll8m_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkEn::LP_AONCLKRST_PLL8M_CLK_FORCE_ON_MASK
            } else {
                self.bits & !LpAonclkrstLpClkEn::LP_AONCLKRST_PLL8M_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_ETM_EVENT_TICK_EN** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_etm_event_tick_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkEn::LP_AONCLKRST_ETM_EVENT_TICK_EN_MASK
            } else {
                self.bits & !LpAonclkrstLpClkEn::LP_AONCLKRST_ETM_EVENT_TICK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_CK_EN_LP_RAM** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_ck_en_lp_ram(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkEn::LP_AONCLKRST_CK_EN_LP_RAM_MASK
            } else {
                self.bits & !LpAonclkrstLpClkEn::LP_AONCLKRST_CK_EN_LP_RAM_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_lp_rtc_xtal_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpClkEn::LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON_MASK
            } else {
                self.bits & !LpAonclkrstLpClkEn::LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON_MASK
            },
        }
    }
}

impl Default for LpAonclkrstLpClkEnVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstLpClkEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstLpClkEnVal {
        LpAonclkrstLpClkEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstLpClkEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstLpClkEnVal) -> LpAonclkrstLpClkEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_LP_RST_EN Register
///
/// need_des
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **LP_AONCLKRST_RST_EN_LP_RAM** `[31]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_RST_EN_LP_AONEFUSEREG** `[30]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_RST_EN_LP_MAILBOX** `[29]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_RST_EN_LP_RTC** `[28]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_RST_EN_LP_TIMER** `[27]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_RST_EN_LP_WDT** `[26]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_RST_EN_LP_ANAPERI** `[25]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_RST_EN_LP_HUK** `[24]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAonclkrstLpRstEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstLpRstEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstLpRstEn {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_RST_EN_LP_RAM**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_AONCLKRST_RST_EN_LP_RAM_OFFSET: u32 = 31;
    pub const LP_AONCLKRST_RST_EN_LP_RAM_MASK: u32 = 0x80000000;
    pub const LP_AONCLKRST_RST_EN_LP_RAM: u32 = 1 << 31;

    /// **LP_AONCLKRST_RST_EN_LP_AONEFUSEREG**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_AONCLKRST_RST_EN_LP_AONEFUSEREG_OFFSET: u32 = 30;
    pub const LP_AONCLKRST_RST_EN_LP_AONEFUSEREG_MASK: u32 = 0x40000000;
    pub const LP_AONCLKRST_RST_EN_LP_AONEFUSEREG: u32 = 1 << 30;

    /// **LP_AONCLKRST_RST_EN_LP_MAILBOX**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_AONCLKRST_RST_EN_LP_MAILBOX_OFFSET: u32 = 29;
    pub const LP_AONCLKRST_RST_EN_LP_MAILBOX_MASK: u32 = 0x20000000;
    pub const LP_AONCLKRST_RST_EN_LP_MAILBOX: u32 = 1 << 29;

    /// **LP_AONCLKRST_RST_EN_LP_RTC**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_AONCLKRST_RST_EN_LP_RTC_OFFSET: u32 = 28;
    pub const LP_AONCLKRST_RST_EN_LP_RTC_MASK: u32 = 0x10000000;
    pub const LP_AONCLKRST_RST_EN_LP_RTC: u32 = 1 << 28;

    /// **LP_AONCLKRST_RST_EN_LP_TIMER**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LP_AONCLKRST_RST_EN_LP_TIMER_OFFSET: u32 = 27;
    pub const LP_AONCLKRST_RST_EN_LP_TIMER_MASK: u32 = 0x08000000;
    pub const LP_AONCLKRST_RST_EN_LP_TIMER: u32 = 1 << 27;

    /// **LP_AONCLKRST_RST_EN_LP_WDT**
    ///
    /// need_des
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const LP_AONCLKRST_RST_EN_LP_WDT_OFFSET: u32 = 26;
    pub const LP_AONCLKRST_RST_EN_LP_WDT_MASK: u32 = 0x04000000;
    pub const LP_AONCLKRST_RST_EN_LP_WDT: u32 = 1 << 26;

    /// **LP_AONCLKRST_RST_EN_LP_ANAPERI**
    ///
    /// need_des
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const LP_AONCLKRST_RST_EN_LP_ANAPERI_OFFSET: u32 = 25;
    pub const LP_AONCLKRST_RST_EN_LP_ANAPERI_MASK: u32 = 0x02000000;
    pub const LP_AONCLKRST_RST_EN_LP_ANAPERI: u32 = 1 << 25;

    /// **LP_AONCLKRST_RST_EN_LP_HUK**
    ///
    /// need_des
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const LP_AONCLKRST_RST_EN_LP_HUK_OFFSET: u32 = 24;
    pub const LP_AONCLKRST_RST_EN_LP_HUK_MASK: u32 = 0x01000000;
    pub const LP_AONCLKRST_RST_EN_LP_HUK: u32 = 1 << 24;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_RST_EN_LP_RAM** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_ram(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_LP_RAM_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_LP_RAM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_lp_ram(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_LP_RAM_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_LP_RAM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_lp_ram(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_LP_RAM_MASK);
    }

    /// **LP_AONCLKRST_RST_EN_LP_AONEFUSEREG** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_aonefusereg(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_LP_AONEFUSEREG_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_LP_AONEFUSEREG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_lp_aonefusereg(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_LP_AONEFUSEREG_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_LP_AONEFUSEREG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_lp_aonefusereg(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_LP_AONEFUSEREG_MASK);
    }

    /// **LP_AONCLKRST_RST_EN_LP_MAILBOX** `[29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_mailbox(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_LP_MAILBOX_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_LP_MAILBOX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_lp_mailbox(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_LP_MAILBOX_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_LP_MAILBOX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_lp_mailbox(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_LP_MAILBOX_MASK);
    }

    /// **LP_AONCLKRST_RST_EN_LP_RTC** `[28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_rtc(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_LP_RTC_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_LP_RTC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_lp_rtc(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_LP_RTC_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_LP_RTC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_lp_rtc(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_LP_RTC_MASK);
    }

    /// **LP_AONCLKRST_RST_EN_LP_TIMER** `[27]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_timer(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_LP_TIMER_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_LP_TIMER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_lp_timer(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_LP_TIMER_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_LP_TIMER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_lp_timer(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_LP_TIMER_MASK);
    }

    /// **LP_AONCLKRST_RST_EN_LP_WDT** `[26]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_wdt(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_LP_WDT_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_LP_WDT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_lp_wdt(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_LP_WDT_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_LP_WDT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_lp_wdt(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_LP_WDT_MASK);
    }

    /// **LP_AONCLKRST_RST_EN_LP_ANAPERI** `[25]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_anaperi(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_LP_ANAPERI_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_LP_ANAPERI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_lp_anaperi(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_LP_ANAPERI_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_LP_ANAPERI bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_lp_anaperi(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_LP_ANAPERI_MASK);
    }

    /// **LP_AONCLKRST_RST_EN_LP_HUK** `[24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_huk(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_LP_HUK_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_LP_HUK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_lp_huk(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_LP_HUK_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_LP_HUK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_lp_huk(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_LP_HUK_MASK);
    }
}

/// LP_AONCLKRST_LP_RST_EN value type for batch read/modify/write operations
///
/// Use `lpaonclkrstlprsten.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrstlprsten.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstLpRstEnVal {
    bits: u32,
}

impl LpAonclkrstLpRstEnVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_AONCLKRST_RST_EN_LP_RAM** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_lp_ram(&self) -> bool {
        (self.bits & LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_RAM_MASK) != 0
    }

    /// Read **LP_AONCLKRST_RST_EN_LP_AONEFUSEREG** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_lp_aonefusereg(&self) -> bool {
        (self.bits & LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_AONEFUSEREG_MASK) != 0
    }

    /// Read **LP_AONCLKRST_RST_EN_LP_MAILBOX** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_lp_mailbox(&self) -> bool {
        (self.bits & LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_MAILBOX_MASK) != 0
    }

    /// Read **LP_AONCLKRST_RST_EN_LP_RTC** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_lp_rtc(&self) -> bool {
        (self.bits & LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_RTC_MASK) != 0
    }

    /// Read **LP_AONCLKRST_RST_EN_LP_TIMER** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_lp_timer(&self) -> bool {
        (self.bits & LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_TIMER_MASK) != 0
    }

    /// Read **LP_AONCLKRST_RST_EN_LP_WDT** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_lp_wdt(&self) -> bool {
        (self.bits & LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_WDT_MASK) != 0
    }

    /// Read **LP_AONCLKRST_RST_EN_LP_ANAPERI** `[25:25]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_lp_anaperi(&self) -> bool {
        (self.bits & LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_ANAPERI_MASK) != 0
    }

    /// Read **LP_AONCLKRST_RST_EN_LP_HUK** `[24:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_lp_huk(&self) -> bool {
        (self.bits & LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_HUK_MASK) != 0
    }

    /// Set **LP_AONCLKRST_RST_EN_LP_RAM** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_lp_ram(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_RAM_MASK
            } else {
                self.bits & !LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_RAM_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_RST_EN_LP_AONEFUSEREG** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_lp_aonefusereg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_AONEFUSEREG_MASK
            } else {
                self.bits & !LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_AONEFUSEREG_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_RST_EN_LP_MAILBOX** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_lp_mailbox(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_MAILBOX_MASK
            } else {
                self.bits & !LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_MAILBOX_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_RST_EN_LP_RTC** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_lp_rtc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_RTC_MASK
            } else {
                self.bits & !LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_RTC_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_RST_EN_LP_TIMER** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_lp_timer(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_TIMER_MASK
            } else {
                self.bits & !LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_TIMER_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_RST_EN_LP_WDT** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_lp_wdt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_WDT_MASK
            } else {
                self.bits & !LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_WDT_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_RST_EN_LP_ANAPERI** `[25:25]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_lp_anaperi(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_ANAPERI_MASK
            } else {
                self.bits & !LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_ANAPERI_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_RST_EN_LP_HUK** `[24:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_lp_huk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_HUK_MASK
            } else {
                self.bits & !LpAonclkrstLpRstEn::LP_AONCLKRST_RST_EN_LP_HUK_MASK
            },
        }
    }
}

impl Default for LpAonclkrstLpRstEnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAonclkrstLpRstEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstLpRstEnVal {
        LpAonclkrstLpRstEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstLpRstEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstLpRstEnVal) -> LpAonclkrstLpRstEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_RESET_CAUSE Register
///
/// need_des
///
/// Address offset: 0x0010
/// Reset value: 0x02000000
///
/// ## Fields
///
/// - **LP_AONCLKRST_HPCORE1_RESET_FLAG_CLR** `[31]` (WO)
///
///   need_des
///
/// - **LP_AONCLKRST_HPCORE1_RESET_CAUSE_CLR** `[30]` (WO)
///
///   need_des
///
/// - **LP_AONCLKRST_HPCORE0_RESET_FLAG_CLR** `[29]` (WO)
///
///   need_des
///
/// - **LP_AONCLKRST_HPCORE0_RESET_CAUSE_CLR** `[28]` (WO)
///
///   need_des
///
/// - **LP_AONCLKRST_LPCORE_RESET_FLAG_CLR** `[27]` (WO)
///
///   need_des
///
/// - **LP_AONCLKRST_LPCORE_RESET_CAUSE_CLR** `[26]` (WO)
///
///   need_des
///
/// - **LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK** `[25]` (RW)
///
///   1'b0: enable lpcore pmu_lp_cpu_reset reset_cause, 1'b1: disable lpcore pmu_lp_cpu_reset reset_cause
///
/// - **LP_AONCLKRST_HPCORE1_RESET_FLAG** `[20]` (RO)
///
///   need_des
///
/// - **LP_AONCLKRST_HPCORE1_RESET_CAUSE** `[19:14]` (RO)
///
///   6'h1: POR reset
///   6'h3: digital system software reset
///   6'h5: PMU HP system power down reset
///   6'h7: HP system reset from HP watchdog
///   6'h9: HP system reset from LP watchdog
///   6'hb: HP core reset from HP watchdog
///   6'hc: HP core software reset
///   6'hd: HP core reset from LP watchdog
///   6'hf: brown out reset
///   6'h10: LP watchdog chip reset
///   6'h12: super watch dog reset
///   6'h13: glitch reset
///   6'h14: efuse crc error reset
///   6'h16: HP usb jtag chip reset
///   6'h17: HP usb uart chip reset
///   6'h18: HP jtag reset
///   6'h1a: HP core lockup
///
/// - **LP_AONCLKRST_HPCORE0_RESET_FLAG** `[13]` (RO)
///
///   need_des
///
/// - **LP_AONCLKRST_HPCORE0_RESET_CAUSE** `[12:7]` (RO)
///
///   6'h1: POR reset
///   6'h3: digital system software reset
///   6'h5: PMU HP system power down reset
///   6'h7: HP system reset from HP watchdog
///   6'h9: HP system reset from LP watchdog
///   6'hb: HP core reset from HP watchdog
///   6'hc: HP core software reset
///   6'hd: HP core reset from LP watchdog
///   6'hf: brown out reset
///   6'h10: LP watchdog chip reset
///   6'h12: super watch dog reset
///   6'h13: glitch reset
///   6'h14: efuse crc error reset
///   6'h16: HP usb jtag chip reset
///   6'h17: HP usb uart chip reset
///   6'h18: HP jtag reset
///   6'h1a: HP core lockup
///
/// - **LP_AONCLKRST_LPCORE_RESET_FLAG** `[6]` (RO)
///
///   need_des
///
/// - **LP_AONCLKRST_LPCORE_RESET_CAUSE** `[5:0]` (RO)
///
///   6'h1: POR reset
///   6'h9: PMU LP PERI power down reset
///   6'ha: PMU LP CPU reset
///   6'hf: brown out reset
///   6'h10: LP watchdog chip reset
///   6'h12: super watch dog reset
///   6'h13: glitch reset
///   6'h14: software reset
#[repr(transparent)]
pub struct LpAonclkrstResetCause {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstResetCause {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstResetCause {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_HPCORE1_RESET_FLAG_CLR**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_AONCLKRST_HPCORE1_RESET_FLAG_CLR_OFFSET: u32 = 31;
    pub const LP_AONCLKRST_HPCORE1_RESET_FLAG_CLR_MASK: u32 = 0x80000000;
    pub const LP_AONCLKRST_HPCORE1_RESET_FLAG_CLR: u32 = 1 << 31;

    /// **LP_AONCLKRST_HPCORE1_RESET_CAUSE_CLR**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_AONCLKRST_HPCORE1_RESET_CAUSE_CLR_OFFSET: u32 = 30;
    pub const LP_AONCLKRST_HPCORE1_RESET_CAUSE_CLR_MASK: u32 = 0x40000000;
    pub const LP_AONCLKRST_HPCORE1_RESET_CAUSE_CLR: u32 = 1 << 30;

    /// **LP_AONCLKRST_HPCORE0_RESET_FLAG_CLR**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_AONCLKRST_HPCORE0_RESET_FLAG_CLR_OFFSET: u32 = 29;
    pub const LP_AONCLKRST_HPCORE0_RESET_FLAG_CLR_MASK: u32 = 0x20000000;
    pub const LP_AONCLKRST_HPCORE0_RESET_FLAG_CLR: u32 = 1 << 29;

    /// **LP_AONCLKRST_HPCORE0_RESET_CAUSE_CLR**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_AONCLKRST_HPCORE0_RESET_CAUSE_CLR_OFFSET: u32 = 28;
    pub const LP_AONCLKRST_HPCORE0_RESET_CAUSE_CLR_MASK: u32 = 0x10000000;
    pub const LP_AONCLKRST_HPCORE0_RESET_CAUSE_CLR: u32 = 1 << 28;

    /// **LP_AONCLKRST_LPCORE_RESET_FLAG_CLR**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LP_AONCLKRST_LPCORE_RESET_FLAG_CLR_OFFSET: u32 = 27;
    pub const LP_AONCLKRST_LPCORE_RESET_FLAG_CLR_MASK: u32 = 0x08000000;
    pub const LP_AONCLKRST_LPCORE_RESET_FLAG_CLR: u32 = 1 << 27;

    /// **LP_AONCLKRST_LPCORE_RESET_CAUSE_CLR**
    ///
    /// need_des
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const LP_AONCLKRST_LPCORE_RESET_CAUSE_CLR_OFFSET: u32 = 26;
    pub const LP_AONCLKRST_LPCORE_RESET_CAUSE_CLR_MASK: u32 = 0x04000000;
    pub const LP_AONCLKRST_LPCORE_RESET_CAUSE_CLR: u32 = 1 << 26;

    /// **LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK**
    ///
    /// 1'b0: enable lpcore pmu_lp_cpu_reset reset_cause, 1'b1: disable lpcore pmu_lp_cpu_reset reset_cause
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK_OFFSET: u32 = 25;
    pub const LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK_MASK: u32 = 0x02000000;
    pub const LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK: u32 = 1 << 25;

    /// **LP_AONCLKRST_HPCORE1_RESET_FLAG**
    ///
    /// need_des
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const LP_AONCLKRST_HPCORE1_RESET_FLAG_OFFSET: u32 = 20;
    pub const LP_AONCLKRST_HPCORE1_RESET_FLAG_MASK: u32 = 0x00100000;
    pub const LP_AONCLKRST_HPCORE1_RESET_FLAG: u32 = 1 << 20;

    /// **LP_AONCLKRST_HPCORE1_RESET_CAUSE**
    ///
    /// 6'h1: POR reset
    /// 6'h3: digital system software reset
    /// 6'h5: PMU HP system power down reset
    /// 6'h7: HP system reset from HP watchdog
    /// 6'h9: HP system reset from LP watchdog
    /// 6'hb: HP core reset from HP watchdog
    /// 6'hc: HP core software reset
    /// 6'hd: HP core reset from LP watchdog
    /// 6'hf: brown out reset
    /// 6'h10: LP watchdog chip reset
    /// 6'h12: super watch dog reset
    /// 6'h13: glitch reset
    /// 6'h14: efuse crc error reset
    /// 6'h16: HP usb jtag chip reset
    /// 6'h17: HP usb uart chip reset
    /// 6'h18: HP jtag reset
    /// 6'h1a: HP core lockup
    ///
    /// Bits `[19:14]`, mask `0x000fc000`
    pub const LP_AONCLKRST_HPCORE1_RESET_CAUSE_OFFSET: u32 = 14;
    pub const LP_AONCLKRST_HPCORE1_RESET_CAUSE_MASK: u32 = 0x000fc000;

    /// **LP_AONCLKRST_HPCORE0_RESET_FLAG**
    ///
    /// need_des
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const LP_AONCLKRST_HPCORE0_RESET_FLAG_OFFSET: u32 = 13;
    pub const LP_AONCLKRST_HPCORE0_RESET_FLAG_MASK: u32 = 0x00002000;
    pub const LP_AONCLKRST_HPCORE0_RESET_FLAG: u32 = 1 << 13;

    /// **LP_AONCLKRST_HPCORE0_RESET_CAUSE**
    ///
    /// 6'h1: POR reset
    /// 6'h3: digital system software reset
    /// 6'h5: PMU HP system power down reset
    /// 6'h7: HP system reset from HP watchdog
    /// 6'h9: HP system reset from LP watchdog
    /// 6'hb: HP core reset from HP watchdog
    /// 6'hc: HP core software reset
    /// 6'hd: HP core reset from LP watchdog
    /// 6'hf: brown out reset
    /// 6'h10: LP watchdog chip reset
    /// 6'h12: super watch dog reset
    /// 6'h13: glitch reset
    /// 6'h14: efuse crc error reset
    /// 6'h16: HP usb jtag chip reset
    /// 6'h17: HP usb uart chip reset
    /// 6'h18: HP jtag reset
    /// 6'h1a: HP core lockup
    ///
    /// Bits `[12:7]`, mask `0x00001f80`
    pub const LP_AONCLKRST_HPCORE0_RESET_CAUSE_OFFSET: u32 = 7;
    pub const LP_AONCLKRST_HPCORE0_RESET_CAUSE_MASK: u32 = 0x00001f80;

    /// **LP_AONCLKRST_LPCORE_RESET_FLAG**
    ///
    /// need_des
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const LP_AONCLKRST_LPCORE_RESET_FLAG_OFFSET: u32 = 6;
    pub const LP_AONCLKRST_LPCORE_RESET_FLAG_MASK: u32 = 0x00000040;
    pub const LP_AONCLKRST_LPCORE_RESET_FLAG: u32 = 1 << 6;

    /// **LP_AONCLKRST_LPCORE_RESET_CAUSE**
    ///
    /// 6'h1: POR reset
    /// 6'h9: PMU LP PERI power down reset
    /// 6'ha: PMU LP CPU reset
    /// 6'hf: brown out reset
    /// 6'h10: LP watchdog chip reset
    /// 6'h12: super watch dog reset
    /// 6'h13: glitch reset
    /// 6'h14: software reset
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const LP_AONCLKRST_LPCORE_RESET_CAUSE_OFFSET: u32 = 0;
    pub const LP_AONCLKRST_LPCORE_RESET_CAUSE_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// Trigger **LP_AONCLKRST_HPCORE1_RESET_FLAG_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_aonclkrst_hpcore1_reset_flag_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HPCORE1_RESET_FLAG_CLR_MASK);
    }

    /// Trigger **LP_AONCLKRST_HPCORE1_RESET_CAUSE_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_aonclkrst_hpcore1_reset_cause_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HPCORE1_RESET_CAUSE_CLR_MASK);
    }

    /// Trigger **LP_AONCLKRST_HPCORE0_RESET_FLAG_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_aonclkrst_hpcore0_reset_flag_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HPCORE0_RESET_FLAG_CLR_MASK);
    }

    /// Trigger **LP_AONCLKRST_HPCORE0_RESET_CAUSE_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_aonclkrst_hpcore0_reset_cause_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HPCORE0_RESET_CAUSE_CLR_MASK);
    }

    /// Trigger **LP_AONCLKRST_LPCORE_RESET_FLAG_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_aonclkrst_lpcore_reset_flag_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_LPCORE_RESET_FLAG_CLR_MASK);
    }

    /// Trigger **LP_AONCLKRST_LPCORE_RESET_CAUSE_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_aonclkrst_lpcore_reset_cause_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_LPCORE_RESET_CAUSE_CLR_MASK);
    }

    /// **LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK** `[25]` (RW)
    ///
    /// 1'b0: enable lpcore pmu_lp_cpu_reset reset_cause, 1'b1: disable lpcore pmu_lp_cpu_reset reset_cause
    #[inline(always)]
    pub fn lp_aonclkrst_lpcore_reset_cause_pmu_lp_cpu_mask(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK_MASK) != 0
    }

    /// Set LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_lpcore_reset_cause_pmu_lp_cpu_mask(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK_MASK);
    }

    /// Clear LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_lpcore_reset_cause_pmu_lp_cpu_mask(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK_MASK);
    }

    /// **LP_AONCLKRST_HPCORE1_RESET_FLAG** `[20]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_reset_flag(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE1_RESET_FLAG_MASK) != 0
    }

    /// **LP_AONCLKRST_HPCORE1_RESET_CAUSE** `[19:14]` (RO)
    ///
    /// 6'h1: POR reset
    /// 6'h3: digital system software reset
    /// 6'h5: PMU HP system power down reset
    /// 6'h7: HP system reset from HP watchdog
    /// 6'h9: HP system reset from LP watchdog
    /// 6'hb: HP core reset from HP watchdog
    /// 6'hc: HP core software reset
    /// 6'hd: HP core reset from LP watchdog
    /// 6'hf: brown out reset
    /// 6'h10: LP watchdog chip reset
    /// 6'h12: super watch dog reset
    /// 6'h13: glitch reset
    /// 6'h14: efuse crc error reset
    /// 6'h16: HP usb jtag chip reset
    /// 6'h17: HP usb uart chip reset
    /// 6'h18: HP jtag reset
    /// 6'h1a: HP core lockup
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_reset_cause(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE1_RESET_CAUSE_MASK)
            >> Self::LP_AONCLKRST_HPCORE1_RESET_CAUSE_OFFSET
    }

    /// **LP_AONCLKRST_HPCORE0_RESET_FLAG** `[13]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_reset_flag(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE0_RESET_FLAG_MASK) != 0
    }

    /// **LP_AONCLKRST_HPCORE0_RESET_CAUSE** `[12:7]` (RO)
    ///
    /// 6'h1: POR reset
    /// 6'h3: digital system software reset
    /// 6'h5: PMU HP system power down reset
    /// 6'h7: HP system reset from HP watchdog
    /// 6'h9: HP system reset from LP watchdog
    /// 6'hb: HP core reset from HP watchdog
    /// 6'hc: HP core software reset
    /// 6'hd: HP core reset from LP watchdog
    /// 6'hf: brown out reset
    /// 6'h10: LP watchdog chip reset
    /// 6'h12: super watch dog reset
    /// 6'h13: glitch reset
    /// 6'h14: efuse crc error reset
    /// 6'h16: HP usb jtag chip reset
    /// 6'h17: HP usb uart chip reset
    /// 6'h18: HP jtag reset
    /// 6'h1a: HP core lockup
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_reset_cause(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE0_RESET_CAUSE_MASK)
            >> Self::LP_AONCLKRST_HPCORE0_RESET_CAUSE_OFFSET
    }

    /// **LP_AONCLKRST_LPCORE_RESET_FLAG** `[6]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_lpcore_reset_flag(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_LPCORE_RESET_FLAG_MASK) != 0
    }

    /// **LP_AONCLKRST_LPCORE_RESET_CAUSE** `[5:0]` (RO)
    ///
    /// 6'h1: POR reset
    /// 6'h9: PMU LP PERI power down reset
    /// 6'ha: PMU LP CPU reset
    /// 6'hf: brown out reset
    /// 6'h10: LP watchdog chip reset
    /// 6'h12: super watch dog reset
    /// 6'h13: glitch reset
    /// 6'h14: software reset
    #[inline(always)]
    pub fn lp_aonclkrst_lpcore_reset_cause(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_LPCORE_RESET_CAUSE_MASK)
            >> Self::LP_AONCLKRST_LPCORE_RESET_CAUSE_OFFSET
    }
}

/// LP_AONCLKRST_RESET_CAUSE value type for batch read/modify/write operations
///
/// Use `lpaonclkrstresetcause.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrstresetcause.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstResetCauseVal {
    bits: u32,
}

impl LpAonclkrstResetCauseVal {
    /// Create from raw bits
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
        Self { bits: 0x02000000 }
    }

    /// Read **LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK** `[25:25]`
    ///
    /// 1'b0: enable lpcore pmu_lp_cpu_reset reset_cause, 1'b1: disable lpcore pmu_lp_cpu_reset reset_cause
    #[inline(always)]
    pub const fn lp_aonclkrst_lpcore_reset_cause_pmu_lp_cpu_mask(&self) -> bool {
        (self.bits & LpAonclkrstResetCause::LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK_MASK)
            != 0
    }

    /// Read **LP_AONCLKRST_HPCORE1_RESET_FLAG** `[20:20]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore1_reset_flag(&self) -> bool {
        (self.bits & LpAonclkrstResetCause::LP_AONCLKRST_HPCORE1_RESET_FLAG_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HPCORE1_RESET_CAUSE** `[19:14]`
    ///
    /// 6'h1: POR reset
    /// 6'h3: digital system software reset
    /// 6'h5: PMU HP system power down reset
    /// 6'h7: HP system reset from HP watchdog
    /// 6'h9: HP system reset from LP watchdog
    /// 6'hb: HP core reset from HP watchdog
    /// 6'hc: HP core software reset
    /// 6'hd: HP core reset from LP watchdog
    /// 6'hf: brown out reset
    /// 6'h10: LP watchdog chip reset
    /// 6'h12: super watch dog reset
    /// 6'h13: glitch reset
    /// 6'h14: efuse crc error reset
    /// 6'h16: HP usb jtag chip reset
    /// 6'h17: HP usb uart chip reset
    /// 6'h18: HP jtag reset
    /// 6'h1a: HP core lockup
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore1_reset_cause(&self) -> u32 {
        (self.bits & LpAonclkrstResetCause::LP_AONCLKRST_HPCORE1_RESET_CAUSE_MASK)
            >> LpAonclkrstResetCause::LP_AONCLKRST_HPCORE1_RESET_CAUSE_OFFSET
    }

    /// Read **LP_AONCLKRST_HPCORE0_RESET_FLAG** `[13:13]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore0_reset_flag(&self) -> bool {
        (self.bits & LpAonclkrstResetCause::LP_AONCLKRST_HPCORE0_RESET_FLAG_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HPCORE0_RESET_CAUSE** `[12:7]`
    ///
    /// 6'h1: POR reset
    /// 6'h3: digital system software reset
    /// 6'h5: PMU HP system power down reset
    /// 6'h7: HP system reset from HP watchdog
    /// 6'h9: HP system reset from LP watchdog
    /// 6'hb: HP core reset from HP watchdog
    /// 6'hc: HP core software reset
    /// 6'hd: HP core reset from LP watchdog
    /// 6'hf: brown out reset
    /// 6'h10: LP watchdog chip reset
    /// 6'h12: super watch dog reset
    /// 6'h13: glitch reset
    /// 6'h14: efuse crc error reset
    /// 6'h16: HP usb jtag chip reset
    /// 6'h17: HP usb uart chip reset
    /// 6'h18: HP jtag reset
    /// 6'h1a: HP core lockup
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore0_reset_cause(&self) -> u32 {
        (self.bits & LpAonclkrstResetCause::LP_AONCLKRST_HPCORE0_RESET_CAUSE_MASK)
            >> LpAonclkrstResetCause::LP_AONCLKRST_HPCORE0_RESET_CAUSE_OFFSET
    }

    /// Read **LP_AONCLKRST_LPCORE_RESET_FLAG** `[6:6]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_lpcore_reset_flag(&self) -> bool {
        (self.bits & LpAonclkrstResetCause::LP_AONCLKRST_LPCORE_RESET_FLAG_MASK) != 0
    }

    /// Read **LP_AONCLKRST_LPCORE_RESET_CAUSE** `[5:0]`
    ///
    /// 6'h1: POR reset
    /// 6'h9: PMU LP PERI power down reset
    /// 6'ha: PMU LP CPU reset
    /// 6'hf: brown out reset
    /// 6'h10: LP watchdog chip reset
    /// 6'h12: super watch dog reset
    /// 6'h13: glitch reset
    /// 6'h14: software reset
    #[inline(always)]
    pub const fn lp_aonclkrst_lpcore_reset_cause(&self) -> u32 {
        (self.bits & LpAonclkrstResetCause::LP_AONCLKRST_LPCORE_RESET_CAUSE_MASK)
            >> LpAonclkrstResetCause::LP_AONCLKRST_LPCORE_RESET_CAUSE_OFFSET
    }

    /// Set **LP_AONCLKRST_HPCORE1_RESET_FLAG_CLR** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore1_reset_flag_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstResetCause::LP_AONCLKRST_HPCORE1_RESET_FLAG_CLR_MASK
            } else {
                self.bits & !LpAonclkrstResetCause::LP_AONCLKRST_HPCORE1_RESET_FLAG_CLR_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HPCORE1_RESET_CAUSE_CLR** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore1_reset_cause_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstResetCause::LP_AONCLKRST_HPCORE1_RESET_CAUSE_CLR_MASK
            } else {
                self.bits & !LpAonclkrstResetCause::LP_AONCLKRST_HPCORE1_RESET_CAUSE_CLR_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HPCORE0_RESET_FLAG_CLR** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore0_reset_flag_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstResetCause::LP_AONCLKRST_HPCORE0_RESET_FLAG_CLR_MASK
            } else {
                self.bits & !LpAonclkrstResetCause::LP_AONCLKRST_HPCORE0_RESET_FLAG_CLR_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HPCORE0_RESET_CAUSE_CLR** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore0_reset_cause_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstResetCause::LP_AONCLKRST_HPCORE0_RESET_CAUSE_CLR_MASK
            } else {
                self.bits & !LpAonclkrstResetCause::LP_AONCLKRST_HPCORE0_RESET_CAUSE_CLR_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_LPCORE_RESET_FLAG_CLR** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_lpcore_reset_flag_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstResetCause::LP_AONCLKRST_LPCORE_RESET_FLAG_CLR_MASK
            } else {
                self.bits & !LpAonclkrstResetCause::LP_AONCLKRST_LPCORE_RESET_FLAG_CLR_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_LPCORE_RESET_CAUSE_CLR** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_lpcore_reset_cause_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstResetCause::LP_AONCLKRST_LPCORE_RESET_CAUSE_CLR_MASK
            } else {
                self.bits & !LpAonclkrstResetCause::LP_AONCLKRST_LPCORE_RESET_CAUSE_CLR_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK** `[25:25]`
    ///
    /// 1'b0: enable lpcore pmu_lp_cpu_reset reset_cause, 1'b1: disable lpcore pmu_lp_cpu_reset reset_cause
    #[inline(always)]
    pub const fn with_lp_aonclkrst_lpcore_reset_cause_pmu_lp_cpu_mask(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits
                    | LpAonclkrstResetCause::LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK_MASK
            } else {
                self.bits
                    & !LpAonclkrstResetCause::LP_AONCLKRST_LPCORE_RESET_CAUSE_PMU_LP_CPU_MASK_MASK
            },
        }
    }
}

impl Default for LpAonclkrstResetCauseVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstResetCause {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstResetCauseVal {
        LpAonclkrstResetCauseVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstResetCauseVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstResetCauseVal) -> LpAonclkrstResetCauseVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_HPCPU_RESET_CTRL0 Register
///
/// need_des
///
/// Address offset: 0x0014
/// Reset value: 0x80028002
///
/// ## Fields
///
/// - **LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL** `[31]` (RW)
///
///   1'b1: boot from HP TCM ROM: 0x4FC00000
///   1'b0: boot from LP TCM RAM: 0x50108000
///
/// - **LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET** `[30]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_HPCORE1_SW_RESET** `[29]` (WO)
///
///   need_des
///
/// - **LP_AONCLKRST_HPCORE1_STALL_EN** `[28]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_HPCORE1_STALL_WAIT** `[27:21]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN** `[20]` (RW)
///
///   write 1 to enable lp_wdt reset hpcore1 feature, write 0 to disable lp_wdt reset hpcore1 feature
///
/// - **LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH** `[19:17]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN** `[16]` (RW)
///
///   write 1 to enable hpcore1 lockup reset feature, write 0 to disable hpcore1 lockup reset feature
///
/// - **LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL** `[15]` (RW)
///
///   1'b1: boot from HP TCM ROM: 0x4FC00000
///   1'b0: boot from LP TCM RAM: 0x50108000
///
/// - **LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET** `[14]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_HPCORE0_SW_RESET** `[13]` (WO)
///
///   need_des
///
/// - **LP_AONCLKRST_HPCORE0_STALL_EN** `[12]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_HPCORE0_STALL_WAIT** `[11:5]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN** `[4]` (RW)
///
///   write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature
///
/// - **LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH** `[3:1]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN** `[0]` (RW)
///
///   write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature
#[repr(transparent)]
pub struct LpAonclkrstHpcpuResetCtrl0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstHpcpuResetCtrl0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstHpcpuResetCtrl0 {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL**
    ///
    /// 1'b1: boot from HP TCM ROM: 0x4FC00000
    /// 1'b0: boot from LP TCM RAM: 0x50108000
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL_OFFSET: u32 = 31;
    pub const LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL_MASK: u32 = 0x80000000;
    pub const LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL: u32 = 1 << 31;

    /// **LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET_OFFSET: u32 = 30;
    pub const LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET_MASK: u32 = 0x40000000;
    pub const LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET: u32 = 1 << 30;

    /// **LP_AONCLKRST_HPCORE1_SW_RESET**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_AONCLKRST_HPCORE1_SW_RESET_OFFSET: u32 = 29;
    pub const LP_AONCLKRST_HPCORE1_SW_RESET_MASK: u32 = 0x20000000;
    pub const LP_AONCLKRST_HPCORE1_SW_RESET: u32 = 1 << 29;

    /// **LP_AONCLKRST_HPCORE1_STALL_EN**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_AONCLKRST_HPCORE1_STALL_EN_OFFSET: u32 = 28;
    pub const LP_AONCLKRST_HPCORE1_STALL_EN_MASK: u32 = 0x10000000;
    pub const LP_AONCLKRST_HPCORE1_STALL_EN: u32 = 1 << 28;

    /// **LP_AONCLKRST_HPCORE1_STALL_WAIT**
    ///
    /// need_des
    ///
    /// Bits `[27:21]`, mask `0x0fe00000`
    pub const LP_AONCLKRST_HPCORE1_STALL_WAIT_OFFSET: u32 = 21;
    pub const LP_AONCLKRST_HPCORE1_STALL_WAIT_MASK: u32 = 0x0fe00000;

    /// **LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN**
    ///
    /// write 1 to enable lp_wdt reset hpcore1 feature, write 0 to disable lp_wdt reset hpcore1 feature
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN_OFFSET: u32 = 20;
    pub const LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN_MASK: u32 = 0x00100000;
    pub const LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN: u32 = 1 << 20;

    /// **LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH**
    ///
    /// need_des
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_OFFSET: u32 = 17;
    pub const LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_MASK: u32 = 0x000e0000;

    /// **LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN**
    ///
    /// write 1 to enable hpcore1 lockup reset feature, write 0 to disable hpcore1 lockup reset feature
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN_OFFSET: u32 = 16;
    pub const LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN_MASK: u32 = 0x00010000;
    pub const LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN: u32 = 1 << 16;

    /// **LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL**
    ///
    /// 1'b1: boot from HP TCM ROM: 0x4FC00000
    /// 1'b0: boot from LP TCM RAM: 0x50108000
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_OFFSET: u32 = 15;
    pub const LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_MASK: u32 = 0x00008000;
    pub const LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL: u32 = 1 << 15;

    /// **LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET**
    ///
    /// need_des
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_OFFSET: u32 = 14;
    pub const LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_MASK: u32 = 0x00004000;
    pub const LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET: u32 = 1 << 14;

    /// **LP_AONCLKRST_HPCORE0_SW_RESET**
    ///
    /// need_des
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const LP_AONCLKRST_HPCORE0_SW_RESET_OFFSET: u32 = 13;
    pub const LP_AONCLKRST_HPCORE0_SW_RESET_MASK: u32 = 0x00002000;
    pub const LP_AONCLKRST_HPCORE0_SW_RESET: u32 = 1 << 13;

    /// **LP_AONCLKRST_HPCORE0_STALL_EN**
    ///
    /// need_des
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const LP_AONCLKRST_HPCORE0_STALL_EN_OFFSET: u32 = 12;
    pub const LP_AONCLKRST_HPCORE0_STALL_EN_MASK: u32 = 0x00001000;
    pub const LP_AONCLKRST_HPCORE0_STALL_EN: u32 = 1 << 12;

    /// **LP_AONCLKRST_HPCORE0_STALL_WAIT**
    ///
    /// need_des
    ///
    /// Bits `[11:5]`, mask `0x00000fe0`
    pub const LP_AONCLKRST_HPCORE0_STALL_WAIT_OFFSET: u32 = 5;
    pub const LP_AONCLKRST_HPCORE0_STALL_WAIT_MASK: u32 = 0x00000fe0;

    /// **LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN**
    ///
    /// write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN_OFFSET: u32 = 4;
    pub const LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN_MASK: u32 = 0x00000010;
    pub const LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN: u32 = 1 << 4;

    /// **LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH**
    ///
    /// need_des
    ///
    /// Bits `[3:1]`, mask `0x0000000e`
    pub const LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_OFFSET: u32 = 1;
    pub const LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_MASK: u32 = 0x0000000e;

    /// **LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN**
    ///
    /// write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_OFFSET: u32 = 0;
    pub const LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_MASK: u32 = 0x00000001;
    pub const LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL** `[31]` (RW)
    ///
    /// 1'b1: boot from HP TCM ROM: 0x4FC00000
    /// 1'b0: boot from LP TCM RAM: 0x50108000
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_stat_vector_sel(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL_MASK) != 0
    }

    /// Set LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hpcore1_stat_vector_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL_MASK);
    }

    /// Clear LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hpcore1_stat_vector_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL_MASK);
    }

    /// **LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_ocd_halt_on_reset(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET_MASK) != 0
    }

    /// Set LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hpcore1_ocd_halt_on_reset(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET_MASK);
    }

    /// Clear LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hpcore1_ocd_halt_on_reset(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET_MASK);
    }

    /// Trigger **LP_AONCLKRST_HPCORE1_SW_RESET** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_aonclkrst_hpcore1_sw_reset(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HPCORE1_SW_RESET_MASK);
    }

    /// **LP_AONCLKRST_HPCORE1_STALL_EN** `[28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_stall_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE1_STALL_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HPCORE1_STALL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hpcore1_stall_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HPCORE1_STALL_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HPCORE1_STALL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hpcore1_stall_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HPCORE1_STALL_EN_MASK);
    }

    /// **LP_AONCLKRST_HPCORE1_STALL_WAIT** `[27:21]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_stall_wait(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE1_STALL_WAIT_MASK)
            >> Self::LP_AONCLKRST_HPCORE1_STALL_WAIT_OFFSET
    }

    /// Write **LP_AONCLKRST_HPCORE1_STALL_WAIT** `[27:21]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_aonclkrst_hpcore1_stall_wait(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_HPCORE1_STALL_WAIT_MASK)
                | ((value << Self::LP_AONCLKRST_HPCORE1_STALL_WAIT_OFFSET)
                    & Self::LP_AONCLKRST_HPCORE1_STALL_WAIT_MASK)
        });
    }

    /// **LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN** `[20]` (RW)
    ///
    /// write 1 to enable lp_wdt reset hpcore1 feature, write 0 to disable lp_wdt reset hpcore1 feature
    #[inline(always)]
    pub fn lp_aonclkrst_lp_wdt_hpcore1_reset_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_lp_wdt_hpcore1_reset_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN_MASK);
    }

    /// Clear LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_lp_wdt_hpcore1_reset_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN_MASK);
    }

    /// **LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH** `[19:17]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_lp_wdt_hpcore1_reset_length(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_MASK)
            >> Self::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_OFFSET
    }

    /// Write **LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH** `[19:17]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_aonclkrst_lp_wdt_hpcore1_reset_length(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_MASK)
                | ((value << Self::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_OFFSET)
                    & Self::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_MASK)
        });
    }

    /// **LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN** `[16]` (RW)
    ///
    /// write 1 to enable hpcore1 lockup reset feature, write 0 to disable hpcore1 lockup reset feature
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_lockup_reset_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hpcore1_lockup_reset_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hpcore1_lockup_reset_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN_MASK);
    }

    /// **LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL** `[15]` (RW)
    ///
    /// 1'b1: boot from HP TCM ROM: 0x4FC00000
    /// 1'b0: boot from LP TCM RAM: 0x50108000
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stat_vector_sel(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_MASK) != 0
    }

    /// Set LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hpcore0_stat_vector_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_MASK);
    }

    /// Clear LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hpcore0_stat_vector_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_MASK);
    }

    /// **LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET** `[14]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_ocd_halt_on_reset(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_MASK) != 0
    }

    /// Set LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hpcore0_ocd_halt_on_reset(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_MASK);
    }

    /// Clear LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hpcore0_ocd_halt_on_reset(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_MASK);
    }

    /// Trigger **LP_AONCLKRST_HPCORE0_SW_RESET** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_aonclkrst_hpcore0_sw_reset(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HPCORE0_SW_RESET_MASK);
    }

    /// **LP_AONCLKRST_HPCORE0_STALL_EN** `[12]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stall_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE0_STALL_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HPCORE0_STALL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hpcore0_stall_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HPCORE0_STALL_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HPCORE0_STALL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hpcore0_stall_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HPCORE0_STALL_EN_MASK);
    }

    /// **LP_AONCLKRST_HPCORE0_STALL_WAIT** `[11:5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stall_wait(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE0_STALL_WAIT_MASK)
            >> Self::LP_AONCLKRST_HPCORE0_STALL_WAIT_OFFSET
    }

    /// Write **LP_AONCLKRST_HPCORE0_STALL_WAIT** `[11:5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_aonclkrst_hpcore0_stall_wait(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_HPCORE0_STALL_WAIT_MASK)
                | ((value << Self::LP_AONCLKRST_HPCORE0_STALL_WAIT_OFFSET)
                    & Self::LP_AONCLKRST_HPCORE0_STALL_WAIT_MASK)
        });
    }

    /// **LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN** `[4]` (RW)
    ///
    /// write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature
    #[inline(always)]
    pub fn lp_aonclkrst_lp_wdt_hpcore0_reset_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_lp_wdt_hpcore0_reset_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN_MASK);
    }

    /// Clear LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_lp_wdt_hpcore0_reset_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN_MASK);
    }

    /// **LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH** `[3:1]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_lp_wdt_hpcore0_reset_length(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_MASK)
            >> Self::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_OFFSET
    }

    /// Write **LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH** `[3:1]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_aonclkrst_lp_wdt_hpcore0_reset_length(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_MASK)
                | ((value << Self::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_OFFSET)
                    & Self::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_MASK)
        });
    }

    /// **LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN** `[0]` (RW)
    ///
    /// write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_lockup_reset_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hpcore0_lockup_reset_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hpcore0_lockup_reset_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_MASK);
    }
}

/// LP_AONCLKRST_HPCPU_RESET_CTRL0 value type for batch read/modify/write operations
///
/// Use `lpaonclkrsthpcpuresetctrl0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrsthpcpuresetctrl0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstHpcpuResetCtrl0Val {
    bits: u32,
}

impl LpAonclkrstHpcpuResetCtrl0Val {
    /// Create from raw bits
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
        Self { bits: 0x80028002 }
    }

    /// Read **LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL** `[31:31]`
    ///
    /// 1'b1: boot from HP TCM ROM: 0x4FC00000
    /// 1'b0: boot from LP TCM RAM: 0x50108000
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore1_stat_vector_sel(&self) -> bool {
        (self.bits & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore1_ocd_halt_on_reset(&self) -> bool {
        (self.bits & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HPCORE1_STALL_EN** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore1_stall_en(&self) -> bool {
        (self.bits & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_STALL_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HPCORE1_STALL_WAIT** `[27:21]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore1_stall_wait(&self) -> u32 {
        (self.bits & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_STALL_WAIT_MASK)
            >> LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_STALL_WAIT_OFFSET
    }

    /// Read **LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN** `[20:20]`
    ///
    /// write 1 to enable lp_wdt reset hpcore1 feature, write 0 to disable lp_wdt reset hpcore1 feature
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_wdt_hpcore1_reset_en(&self) -> bool {
        (self.bits & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH** `[19:17]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_wdt_hpcore1_reset_length(&self) -> u32 {
        (self.bits & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_MASK)
            >> LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_OFFSET
    }

    /// Read **LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN** `[16:16]`
    ///
    /// write 1 to enable hpcore1 lockup reset feature, write 0 to disable hpcore1 lockup reset feature
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore1_lockup_reset_en(&self) -> bool {
        (self.bits & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL** `[15:15]`
    ///
    /// 1'b1: boot from HP TCM ROM: 0x4FC00000
    /// 1'b0: boot from LP TCM RAM: 0x50108000
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore0_stat_vector_sel(&self) -> bool {
        (self.bits & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET** `[14:14]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore0_ocd_halt_on_reset(&self) -> bool {
        (self.bits & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HPCORE0_STALL_EN** `[12:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore0_stall_en(&self) -> bool {
        (self.bits & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_STALL_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HPCORE0_STALL_WAIT** `[11:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore0_stall_wait(&self) -> u32 {
        (self.bits & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_STALL_WAIT_MASK)
            >> LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_STALL_WAIT_OFFSET
    }

    /// Read **LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN** `[4:4]`
    ///
    /// write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_wdt_hpcore0_reset_en(&self) -> bool {
        (self.bits & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH** `[3:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_lp_wdt_hpcore0_reset_length(&self) -> u32 {
        (self.bits & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_MASK)
            >> LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_OFFSET
    }

    /// Read **LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN** `[0:0]`
    ///
    /// write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore0_lockup_reset_en(&self) -> bool {
        (self.bits & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_MASK) != 0
    }

    /// Set **LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL** `[31:31]`
    ///
    /// 1'b1: boot from HP TCM ROM: 0x4FC00000
    /// 1'b0: boot from LP TCM RAM: 0x50108000
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore1_stat_vector_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL_MASK
            } else {
                self.bits & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore1_ocd_halt_on_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET_MASK
            } else {
                self.bits & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HPCORE1_SW_RESET** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore1_sw_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_SW_RESET_MASK
            } else {
                self.bits & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_SW_RESET_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HPCORE1_STALL_EN** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore1_stall_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_STALL_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_STALL_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HPCORE1_STALL_WAIT** `[27:21]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore1_stall_wait(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_STALL_WAIT_MASK)
                | ((value << LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_STALL_WAIT_OFFSET)
                    & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_STALL_WAIT_MASK),
        }
    }

    /// Set **LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN** `[20:20]`
    ///
    /// write 1 to enable lp_wdt reset hpcore1 feature, write 0 to disable lp_wdt reset hpcore1 feature
    #[inline(always)]
    pub const fn with_lp_aonclkrst_lp_wdt_hpcore1_reset_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH** `[19:17]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_lp_wdt_hpcore1_reset_length(self, value: u32) -> Self {
        Self {
            bits: (self.bits
                & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_MASK)
                | ((value
                    << LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_OFFSET)
                    & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_MASK),
        }
    }

    /// Set **LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN** `[16:16]`
    ///
    /// write 1 to enable hpcore1 lockup reset feature, write 0 to disable hpcore1 lockup reset feature
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore1_lockup_reset_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL** `[15:15]`
    ///
    /// 1'b1: boot from HP TCM ROM: 0x4FC00000
    /// 1'b0: boot from LP TCM RAM: 0x50108000
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore0_stat_vector_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_MASK
            } else {
                self.bits & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET** `[14:14]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore0_ocd_halt_on_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_MASK
            } else {
                self.bits & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HPCORE0_SW_RESET** `[13:13]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore0_sw_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_SW_RESET_MASK
            } else {
                self.bits & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_SW_RESET_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HPCORE0_STALL_EN** `[12:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore0_stall_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_STALL_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_STALL_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HPCORE0_STALL_WAIT** `[11:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore0_stall_wait(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_STALL_WAIT_MASK)
                | ((value << LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_STALL_WAIT_OFFSET)
                    & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_STALL_WAIT_MASK),
        }
    }

    /// Set **LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN** `[4:4]`
    ///
    /// write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature
    #[inline(always)]
    pub const fn with_lp_aonclkrst_lp_wdt_hpcore0_reset_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH** `[3:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_lp_wdt_hpcore0_reset_length(self, value: u32) -> Self {
        Self {
            bits: (self.bits
                & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_MASK)
                | ((value
                    << LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_OFFSET)
                    & LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_MASK),
        }
    }

    /// Set **LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN** `[0:0]`
    ///
    /// write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore0_lockup_reset_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpcpuResetCtrl0::LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_MASK
            },
        }
    }
}

impl Default for LpAonclkrstHpcpuResetCtrl0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstHpcpuResetCtrl0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstHpcpuResetCtrl0Val {
        LpAonclkrstHpcpuResetCtrl0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstHpcpuResetCtrl0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstHpcpuResetCtrl0Val) -> LpAonclkrstHpcpuResetCtrl0Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_HPCPU_RESET_CTRL1 Register
///
/// need_des
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **LP_AONCLKRST_HPCORE1_SW_STALL_CODE** `[31:24]` (RW)
///
///   HP core1 software stall when set to 8'h86
///
/// - **LP_AONCLKRST_HPCORE0_SW_STALL_CODE** `[23:16]` (RW)
///
///   HP core0 software stall when set to 8'h86
#[repr(transparent)]
pub struct LpAonclkrstHpcpuResetCtrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstHpcpuResetCtrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstHpcpuResetCtrl1 {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_HPCORE1_SW_STALL_CODE**
    ///
    /// HP core1 software stall when set to 8'h86
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const LP_AONCLKRST_HPCORE1_SW_STALL_CODE_OFFSET: u32 = 24;
    pub const LP_AONCLKRST_HPCORE1_SW_STALL_CODE_MASK: u32 = 0xff000000;

    /// **LP_AONCLKRST_HPCORE0_SW_STALL_CODE**
    ///
    /// HP core0 software stall when set to 8'h86
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const LP_AONCLKRST_HPCORE0_SW_STALL_CODE_OFFSET: u32 = 16;
    pub const LP_AONCLKRST_HPCORE0_SW_STALL_CODE_MASK: u32 = 0x00ff0000;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_HPCORE1_SW_STALL_CODE** `[31:24]` (RW)
    ///
    /// HP core1 software stall when set to 8'h86
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_sw_stall_code(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE1_SW_STALL_CODE_MASK)
            >> Self::LP_AONCLKRST_HPCORE1_SW_STALL_CODE_OFFSET
    }

    /// Write **LP_AONCLKRST_HPCORE1_SW_STALL_CODE** `[31:24]` (RW)
    ///
    /// HP core1 software stall when set to 8'h86
    #[inline(always)]
    pub fn write_lp_aonclkrst_hpcore1_sw_stall_code(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_HPCORE1_SW_STALL_CODE_MASK)
                | ((value << Self::LP_AONCLKRST_HPCORE1_SW_STALL_CODE_OFFSET)
                    & Self::LP_AONCLKRST_HPCORE1_SW_STALL_CODE_MASK)
        });
    }

    /// **LP_AONCLKRST_HPCORE0_SW_STALL_CODE** `[23:16]` (RW)
    ///
    /// HP core0 software stall when set to 8'h86
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_sw_stall_code(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPCORE0_SW_STALL_CODE_MASK)
            >> Self::LP_AONCLKRST_HPCORE0_SW_STALL_CODE_OFFSET
    }

    /// Write **LP_AONCLKRST_HPCORE0_SW_STALL_CODE** `[23:16]` (RW)
    ///
    /// HP core0 software stall when set to 8'h86
    #[inline(always)]
    pub fn write_lp_aonclkrst_hpcore0_sw_stall_code(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_HPCORE0_SW_STALL_CODE_MASK)
                | ((value << Self::LP_AONCLKRST_HPCORE0_SW_STALL_CODE_OFFSET)
                    & Self::LP_AONCLKRST_HPCORE0_SW_STALL_CODE_MASK)
        });
    }
}

/// LP_AONCLKRST_HPCPU_RESET_CTRL1 value type for batch read/modify/write operations
///
/// Use `lpaonclkrsthpcpuresetctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrsthpcpuresetctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstHpcpuResetCtrl1Val {
    bits: u32,
}

impl LpAonclkrstHpcpuResetCtrl1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_AONCLKRST_HPCORE1_SW_STALL_CODE** `[31:24]`
    ///
    /// HP core1 software stall when set to 8'h86
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore1_sw_stall_code(&self) -> u32 {
        (self.bits & LpAonclkrstHpcpuResetCtrl1::LP_AONCLKRST_HPCORE1_SW_STALL_CODE_MASK)
            >> LpAonclkrstHpcpuResetCtrl1::LP_AONCLKRST_HPCORE1_SW_STALL_CODE_OFFSET
    }

    /// Read **LP_AONCLKRST_HPCORE0_SW_STALL_CODE** `[23:16]`
    ///
    /// HP core0 software stall when set to 8'h86
    #[inline(always)]
    pub const fn lp_aonclkrst_hpcore0_sw_stall_code(&self) -> u32 {
        (self.bits & LpAonclkrstHpcpuResetCtrl1::LP_AONCLKRST_HPCORE0_SW_STALL_CODE_MASK)
            >> LpAonclkrstHpcpuResetCtrl1::LP_AONCLKRST_HPCORE0_SW_STALL_CODE_OFFSET
    }

    /// Set **LP_AONCLKRST_HPCORE1_SW_STALL_CODE** `[31:24]`
    ///
    /// HP core1 software stall when set to 8'h86
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore1_sw_stall_code(self, value: u32) -> Self {
        Self {
            bits: (self.bits
                & !LpAonclkrstHpcpuResetCtrl1::LP_AONCLKRST_HPCORE1_SW_STALL_CODE_MASK)
                | ((value
                    << LpAonclkrstHpcpuResetCtrl1::LP_AONCLKRST_HPCORE1_SW_STALL_CODE_OFFSET)
                    & LpAonclkrstHpcpuResetCtrl1::LP_AONCLKRST_HPCORE1_SW_STALL_CODE_MASK),
        }
    }

    /// Set **LP_AONCLKRST_HPCORE0_SW_STALL_CODE** `[23:16]`
    ///
    /// HP core0 software stall when set to 8'h86
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpcore0_sw_stall_code(self, value: u32) -> Self {
        Self {
            bits: (self.bits
                & !LpAonclkrstHpcpuResetCtrl1::LP_AONCLKRST_HPCORE0_SW_STALL_CODE_MASK)
                | ((value
                    << LpAonclkrstHpcpuResetCtrl1::LP_AONCLKRST_HPCORE0_SW_STALL_CODE_OFFSET)
                    & LpAonclkrstHpcpuResetCtrl1::LP_AONCLKRST_HPCORE0_SW_STALL_CODE_MASK),
        }
    }
}

impl Default for LpAonclkrstHpcpuResetCtrl1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAonclkrstHpcpuResetCtrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstHpcpuResetCtrl1Val {
        LpAonclkrstHpcpuResetCtrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstHpcpuResetCtrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstHpcpuResetCtrl1Val) -> LpAonclkrstHpcpuResetCtrl1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_FOSC_CNTL Register
///
/// need_des
///
/// Address offset: 0x001c
/// Reset value: 0x64000000
///
/// ## Fields
///
/// - **LP_AONCLKRST_FOSC_DFREQ** `[31:22]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAonclkrstFoscCntl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstFoscCntl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstFoscCntl {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_FOSC_DFREQ**
    ///
    /// need_des
    ///
    /// Bits `[31:22]`, mask `0xffc00000`
    pub const LP_AONCLKRST_FOSC_DFREQ_OFFSET: u32 = 22;
    pub const LP_AONCLKRST_FOSC_DFREQ_MASK: u32 = 0xffc00000;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_FOSC_DFREQ** `[31:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_fosc_dfreq(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_FOSC_DFREQ_MASK)
            >> Self::LP_AONCLKRST_FOSC_DFREQ_OFFSET
    }

    /// Write **LP_AONCLKRST_FOSC_DFREQ** `[31:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_aonclkrst_fosc_dfreq(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_FOSC_DFREQ_MASK)
                | ((value << Self::LP_AONCLKRST_FOSC_DFREQ_OFFSET)
                    & Self::LP_AONCLKRST_FOSC_DFREQ_MASK)
        });
    }
}

/// LP_AONCLKRST_FOSC_CNTL value type for batch read/modify/write operations
///
/// Use `lpaonclkrstfosccntl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrstfosccntl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstFoscCntlVal {
    bits: u32,
}

impl LpAonclkrstFoscCntlVal {
    /// Create from raw bits
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
        Self { bits: 0x64000000 }
    }

    /// Read **LP_AONCLKRST_FOSC_DFREQ** `[31:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_fosc_dfreq(&self) -> u32 {
        (self.bits & LpAonclkrstFoscCntl::LP_AONCLKRST_FOSC_DFREQ_MASK)
            >> LpAonclkrstFoscCntl::LP_AONCLKRST_FOSC_DFREQ_OFFSET
    }

    /// Set **LP_AONCLKRST_FOSC_DFREQ** `[31:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_fosc_dfreq(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstFoscCntl::LP_AONCLKRST_FOSC_DFREQ_MASK)
                | ((value << LpAonclkrstFoscCntl::LP_AONCLKRST_FOSC_DFREQ_OFFSET)
                    & LpAonclkrstFoscCntl::LP_AONCLKRST_FOSC_DFREQ_MASK),
        }
    }
}

impl Default for LpAonclkrstFoscCntlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstFoscCntl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstFoscCntlVal {
        LpAonclkrstFoscCntlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstFoscCntlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstFoscCntlVal) -> LpAonclkrstFoscCntlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_RC32K_CNTL Register
///
/// need_des
///
/// Address offset: 0x0020
/// Reset value: 0x0000028a
///
/// ## Fields
///
/// - **LP_AONCLKRST_RC32K_DFREQ** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAonclkrstRc32kCntl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstRc32kCntl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstRc32kCntl {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_RC32K_DFREQ**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_AONCLKRST_RC32K_DFREQ_OFFSET: u32 = 0;
    pub const LP_AONCLKRST_RC32K_DFREQ_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_RC32K_DFREQ** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_rc32k_dfreq(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_RC32K_DFREQ_MASK)
            >> Self::LP_AONCLKRST_RC32K_DFREQ_OFFSET
    }

    /// Write **LP_AONCLKRST_RC32K_DFREQ** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_aonclkrst_rc32k_dfreq(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_RC32K_DFREQ_MASK)
                | ((value << Self::LP_AONCLKRST_RC32K_DFREQ_OFFSET)
                    & Self::LP_AONCLKRST_RC32K_DFREQ_MASK)
        });
    }
}

/// LP_AONCLKRST_RC32K_CNTL value type for batch read/modify/write operations
///
/// Use `lpaonclkrstrc32kcntl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrstrc32kcntl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstRc32kCntlVal {
    bits: u32,
}

impl LpAonclkrstRc32kCntlVal {
    /// Create from raw bits
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
        Self { bits: 0x0000028a }
    }

    /// Read **LP_AONCLKRST_RC32K_DFREQ** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_rc32k_dfreq(&self) -> u32 {
        (self.bits & LpAonclkrstRc32kCntl::LP_AONCLKRST_RC32K_DFREQ_MASK)
            >> LpAonclkrstRc32kCntl::LP_AONCLKRST_RC32K_DFREQ_OFFSET
    }

    /// Set **LP_AONCLKRST_RC32K_DFREQ** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rc32k_dfreq(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstRc32kCntl::LP_AONCLKRST_RC32K_DFREQ_MASK)
                | ((value << LpAonclkrstRc32kCntl::LP_AONCLKRST_RC32K_DFREQ_OFFSET)
                    & LpAonclkrstRc32kCntl::LP_AONCLKRST_RC32K_DFREQ_MASK),
        }
    }
}

impl Default for LpAonclkrstRc32kCntlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstRc32kCntl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstRc32kCntlVal {
        LpAonclkrstRc32kCntlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstRc32kCntlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstRc32kCntlVal) -> LpAonclkrstRc32kCntlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_SOSC_CNTL Register
///
/// need_des
///
/// Address offset: 0x0024
/// Reset value: 0x2b000000
///
/// ## Fields
///
/// - **LP_AONCLKRST_SOSC_DFREQ** `[31:22]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAonclkrstSoscCntl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstSoscCntl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstSoscCntl {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_SOSC_DFREQ**
    ///
    /// need_des
    ///
    /// Bits `[31:22]`, mask `0xffc00000`
    pub const LP_AONCLKRST_SOSC_DFREQ_OFFSET: u32 = 22;
    pub const LP_AONCLKRST_SOSC_DFREQ_MASK: u32 = 0xffc00000;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_SOSC_DFREQ** `[31:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_sosc_dfreq(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_SOSC_DFREQ_MASK)
            >> Self::LP_AONCLKRST_SOSC_DFREQ_OFFSET
    }

    /// Write **LP_AONCLKRST_SOSC_DFREQ** `[31:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_aonclkrst_sosc_dfreq(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_SOSC_DFREQ_MASK)
                | ((value << Self::LP_AONCLKRST_SOSC_DFREQ_OFFSET)
                    & Self::LP_AONCLKRST_SOSC_DFREQ_MASK)
        });
    }
}

/// LP_AONCLKRST_SOSC_CNTL value type for batch read/modify/write operations
///
/// Use `lpaonclkrstsosccntl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrstsosccntl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstSoscCntlVal {
    bits: u32,
}

impl LpAonclkrstSoscCntlVal {
    /// Create from raw bits
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
        Self { bits: 0x2b000000 }
    }

    /// Read **LP_AONCLKRST_SOSC_DFREQ** `[31:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_sosc_dfreq(&self) -> u32 {
        (self.bits & LpAonclkrstSoscCntl::LP_AONCLKRST_SOSC_DFREQ_MASK)
            >> LpAonclkrstSoscCntl::LP_AONCLKRST_SOSC_DFREQ_OFFSET
    }

    /// Set **LP_AONCLKRST_SOSC_DFREQ** `[31:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_sosc_dfreq(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstSoscCntl::LP_AONCLKRST_SOSC_DFREQ_MASK)
                | ((value << LpAonclkrstSoscCntl::LP_AONCLKRST_SOSC_DFREQ_OFFSET)
                    & LpAonclkrstSoscCntl::LP_AONCLKRST_SOSC_DFREQ_MASK),
        }
    }
}

impl Default for LpAonclkrstSoscCntlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstSoscCntl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstSoscCntlVal {
        LpAonclkrstSoscCntlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstSoscCntlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstSoscCntlVal) -> LpAonclkrstSoscCntlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_CLK_TO_HP Register
///
/// need_des
///
/// Address offset: 0x0028
/// Reset value: 0xf0000000
///
/// ## Fields
///
/// - **LP_AONCLKRST_ICG_HP_FOSC** `[31]` (RW)
///
///   reserved
///
/// - **LP_AONCLKRST_ICG_HP_OSC32K** `[30]` (RW)
///
///   reserved
///
/// - **LP_AONCLKRST_ICG_HP_SOSC** `[29]` (RW)
///
///   reserved
///
/// - **LP_AONCLKRST_ICG_HP_XTAL32K** `[28]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct LpAonclkrstClkToHp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstClkToHp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstClkToHp {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_ICG_HP_FOSC**
    ///
    /// reserved
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_AONCLKRST_ICG_HP_FOSC_OFFSET: u32 = 31;
    pub const LP_AONCLKRST_ICG_HP_FOSC_MASK: u32 = 0x80000000;
    pub const LP_AONCLKRST_ICG_HP_FOSC: u32 = 1 << 31;

    /// **LP_AONCLKRST_ICG_HP_OSC32K**
    ///
    /// reserved
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_AONCLKRST_ICG_HP_OSC32K_OFFSET: u32 = 30;
    pub const LP_AONCLKRST_ICG_HP_OSC32K_MASK: u32 = 0x40000000;
    pub const LP_AONCLKRST_ICG_HP_OSC32K: u32 = 1 << 30;

    /// **LP_AONCLKRST_ICG_HP_SOSC**
    ///
    /// reserved
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_AONCLKRST_ICG_HP_SOSC_OFFSET: u32 = 29;
    pub const LP_AONCLKRST_ICG_HP_SOSC_MASK: u32 = 0x20000000;
    pub const LP_AONCLKRST_ICG_HP_SOSC: u32 = 1 << 29;

    /// **LP_AONCLKRST_ICG_HP_XTAL32K**
    ///
    /// reserved
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_AONCLKRST_ICG_HP_XTAL32K_OFFSET: u32 = 28;
    pub const LP_AONCLKRST_ICG_HP_XTAL32K_MASK: u32 = 0x10000000;
    pub const LP_AONCLKRST_ICG_HP_XTAL32K: u32 = 1 << 28;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_ICG_HP_FOSC** `[31]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_fosc(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_ICG_HP_FOSC_MASK) != 0
    }

    /// Set LP_AONCLKRST_ICG_HP_FOSC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_icg_hp_fosc(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_ICG_HP_FOSC_MASK);
    }

    /// Clear LP_AONCLKRST_ICG_HP_FOSC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_icg_hp_fosc(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_ICG_HP_FOSC_MASK);
    }

    /// **LP_AONCLKRST_ICG_HP_OSC32K** `[30]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_osc32k(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_ICG_HP_OSC32K_MASK) != 0
    }

    /// Set LP_AONCLKRST_ICG_HP_OSC32K bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_icg_hp_osc32k(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_ICG_HP_OSC32K_MASK);
    }

    /// Clear LP_AONCLKRST_ICG_HP_OSC32K bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_icg_hp_osc32k(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_ICG_HP_OSC32K_MASK);
    }

    /// **LP_AONCLKRST_ICG_HP_SOSC** `[29]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_sosc(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_ICG_HP_SOSC_MASK) != 0
    }

    /// Set LP_AONCLKRST_ICG_HP_SOSC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_icg_hp_sosc(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_ICG_HP_SOSC_MASK);
    }

    /// Clear LP_AONCLKRST_ICG_HP_SOSC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_icg_hp_sosc(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_ICG_HP_SOSC_MASK);
    }

    /// **LP_AONCLKRST_ICG_HP_XTAL32K** `[28]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_xtal32k(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_ICG_HP_XTAL32K_MASK) != 0
    }

    /// Set LP_AONCLKRST_ICG_HP_XTAL32K bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_icg_hp_xtal32k(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_ICG_HP_XTAL32K_MASK);
    }

    /// Clear LP_AONCLKRST_ICG_HP_XTAL32K bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_icg_hp_xtal32k(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_ICG_HP_XTAL32K_MASK);
    }
}

/// LP_AONCLKRST_CLK_TO_HP value type for batch read/modify/write operations
///
/// Use `lpaonclkrstclktohp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrstclktohp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstClkToHpVal {
    bits: u32,
}

impl LpAonclkrstClkToHpVal {
    /// Create from raw bits
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
        Self { bits: 0xf0000000 }
    }

    /// Read **LP_AONCLKRST_ICG_HP_FOSC** `[31:31]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn lp_aonclkrst_icg_hp_fosc(&self) -> bool {
        (self.bits & LpAonclkrstClkToHp::LP_AONCLKRST_ICG_HP_FOSC_MASK) != 0
    }

    /// Read **LP_AONCLKRST_ICG_HP_OSC32K** `[30:30]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn lp_aonclkrst_icg_hp_osc32k(&self) -> bool {
        (self.bits & LpAonclkrstClkToHp::LP_AONCLKRST_ICG_HP_OSC32K_MASK) != 0
    }

    /// Read **LP_AONCLKRST_ICG_HP_SOSC** `[29:29]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn lp_aonclkrst_icg_hp_sosc(&self) -> bool {
        (self.bits & LpAonclkrstClkToHp::LP_AONCLKRST_ICG_HP_SOSC_MASK) != 0
    }

    /// Read **LP_AONCLKRST_ICG_HP_XTAL32K** `[28:28]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn lp_aonclkrst_icg_hp_xtal32k(&self) -> bool {
        (self.bits & LpAonclkrstClkToHp::LP_AONCLKRST_ICG_HP_XTAL32K_MASK) != 0
    }

    /// Set **LP_AONCLKRST_ICG_HP_FOSC** `[31:31]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_lp_aonclkrst_icg_hp_fosc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstClkToHp::LP_AONCLKRST_ICG_HP_FOSC_MASK
            } else {
                self.bits & !LpAonclkrstClkToHp::LP_AONCLKRST_ICG_HP_FOSC_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_ICG_HP_OSC32K** `[30:30]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_lp_aonclkrst_icg_hp_osc32k(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstClkToHp::LP_AONCLKRST_ICG_HP_OSC32K_MASK
            } else {
                self.bits & !LpAonclkrstClkToHp::LP_AONCLKRST_ICG_HP_OSC32K_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_ICG_HP_SOSC** `[29:29]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_lp_aonclkrst_icg_hp_sosc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstClkToHp::LP_AONCLKRST_ICG_HP_SOSC_MASK
            } else {
                self.bits & !LpAonclkrstClkToHp::LP_AONCLKRST_ICG_HP_SOSC_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_ICG_HP_XTAL32K** `[28:28]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_lp_aonclkrst_icg_hp_xtal32k(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstClkToHp::LP_AONCLKRST_ICG_HP_XTAL32K_MASK
            } else {
                self.bits & !LpAonclkrstClkToHp::LP_AONCLKRST_ICG_HP_XTAL32K_MASK
            },
        }
    }
}

impl Default for LpAonclkrstClkToHpVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstClkToHp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstClkToHpVal {
        LpAonclkrstClkToHpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstClkToHpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstClkToHpVal) -> LpAonclkrstClkToHpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_LPMEM_FORCE Register
///
/// need_des
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **LP_AONCLKRST_LPMEM_CLK_FORCE_ON** `[31]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct LpAonclkrstLpmemForce {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstLpmemForce {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstLpmemForce {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_LPMEM_CLK_FORCE_ON**
    ///
    /// reserved
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_AONCLKRST_LPMEM_CLK_FORCE_ON_OFFSET: u32 = 31;
    pub const LP_AONCLKRST_LPMEM_CLK_FORCE_ON_MASK: u32 = 0x80000000;
    pub const LP_AONCLKRST_LPMEM_CLK_FORCE_ON: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_LPMEM_CLK_FORCE_ON** `[31]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn lp_aonclkrst_lpmem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_LPMEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set LP_AONCLKRST_LPMEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_lpmem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_LPMEM_CLK_FORCE_ON_MASK);
    }

    /// Clear LP_AONCLKRST_LPMEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_lpmem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_LPMEM_CLK_FORCE_ON_MASK);
    }
}

/// LP_AONCLKRST_LPMEM_FORCE value type for batch read/modify/write operations
///
/// Use `lpaonclkrstlpmemforce.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrstlpmemforce.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstLpmemForceVal {
    bits: u32,
}

impl LpAonclkrstLpmemForceVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_AONCLKRST_LPMEM_CLK_FORCE_ON** `[31:31]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn lp_aonclkrst_lpmem_clk_force_on(&self) -> bool {
        (self.bits & LpAonclkrstLpmemForce::LP_AONCLKRST_LPMEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set **LP_AONCLKRST_LPMEM_CLK_FORCE_ON** `[31:31]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_lp_aonclkrst_lpmem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstLpmemForce::LP_AONCLKRST_LPMEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !LpAonclkrstLpmemForce::LP_AONCLKRST_LPMEM_CLK_FORCE_ON_MASK
            },
        }
    }
}

impl Default for LpAonclkrstLpmemForceVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAonclkrstLpmemForce {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstLpmemForceVal {
        LpAonclkrstLpmemForceVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstLpmemForceVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstLpmemForceVal) -> LpAonclkrstLpmemForceVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_XTAL32K Register
///
/// need_des
///
/// Address offset: 0x0030
/// Reset value: 0x66c00000
///
/// ## Fields
///
/// - **LP_AONCLKRST_DAC_XTAL32K** `[31:29]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_DBUF_XTAL32K** `[28]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_DGM_XTAL32K** `[27:25]` (RW)
///
///   need_des
///
/// - **LP_AONCLKRST_DRES_XTAL32K** `[24:22]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAonclkrstXtal32k {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstXtal32k {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstXtal32k {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_DAC_XTAL32K**
    ///
    /// need_des
    ///
    /// Bits `[31:29]`, mask `0xe0000000`
    pub const LP_AONCLKRST_DAC_XTAL32K_OFFSET: u32 = 29;
    pub const LP_AONCLKRST_DAC_XTAL32K_MASK: u32 = 0xe0000000;

    /// **LP_AONCLKRST_DBUF_XTAL32K**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_AONCLKRST_DBUF_XTAL32K_OFFSET: u32 = 28;
    pub const LP_AONCLKRST_DBUF_XTAL32K_MASK: u32 = 0x10000000;
    pub const LP_AONCLKRST_DBUF_XTAL32K: u32 = 1 << 28;

    /// **LP_AONCLKRST_DGM_XTAL32K**
    ///
    /// need_des
    ///
    /// Bits `[27:25]`, mask `0x0e000000`
    pub const LP_AONCLKRST_DGM_XTAL32K_OFFSET: u32 = 25;
    pub const LP_AONCLKRST_DGM_XTAL32K_MASK: u32 = 0x0e000000;

    /// **LP_AONCLKRST_DRES_XTAL32K**
    ///
    /// need_des
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const LP_AONCLKRST_DRES_XTAL32K_OFFSET: u32 = 22;
    pub const LP_AONCLKRST_DRES_XTAL32K_MASK: u32 = 0x01c00000;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_DAC_XTAL32K** `[31:29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_dac_xtal32k(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_DAC_XTAL32K_MASK)
            >> Self::LP_AONCLKRST_DAC_XTAL32K_OFFSET
    }

    /// Write **LP_AONCLKRST_DAC_XTAL32K** `[31:29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_aonclkrst_dac_xtal32k(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_DAC_XTAL32K_MASK)
                | ((value << Self::LP_AONCLKRST_DAC_XTAL32K_OFFSET)
                    & Self::LP_AONCLKRST_DAC_XTAL32K_MASK)
        });
    }

    /// **LP_AONCLKRST_DBUF_XTAL32K** `[28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_dbuf_xtal32k(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_DBUF_XTAL32K_MASK) != 0
    }

    /// Set LP_AONCLKRST_DBUF_XTAL32K bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_dbuf_xtal32k(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_DBUF_XTAL32K_MASK);
    }

    /// Clear LP_AONCLKRST_DBUF_XTAL32K bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_dbuf_xtal32k(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_DBUF_XTAL32K_MASK);
    }

    /// **LP_AONCLKRST_DGM_XTAL32K** `[27:25]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_dgm_xtal32k(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_DGM_XTAL32K_MASK)
            >> Self::LP_AONCLKRST_DGM_XTAL32K_OFFSET
    }

    /// Write **LP_AONCLKRST_DGM_XTAL32K** `[27:25]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_aonclkrst_dgm_xtal32k(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_DGM_XTAL32K_MASK)
                | ((value << Self::LP_AONCLKRST_DGM_XTAL32K_OFFSET)
                    & Self::LP_AONCLKRST_DGM_XTAL32K_MASK)
        });
    }

    /// **LP_AONCLKRST_DRES_XTAL32K** `[24:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_dres_xtal32k(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_DRES_XTAL32K_MASK)
            >> Self::LP_AONCLKRST_DRES_XTAL32K_OFFSET
    }

    /// Write **LP_AONCLKRST_DRES_XTAL32K** `[24:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_aonclkrst_dres_xtal32k(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_DRES_XTAL32K_MASK)
                | ((value << Self::LP_AONCLKRST_DRES_XTAL32K_OFFSET)
                    & Self::LP_AONCLKRST_DRES_XTAL32K_MASK)
        });
    }
}

/// LP_AONCLKRST_XTAL32K value type for batch read/modify/write operations
///
/// Use `lpaonclkrstxtal32k.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrstxtal32k.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstXtal32kVal {
    bits: u32,
}

impl LpAonclkrstXtal32kVal {
    /// Create from raw bits
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
        Self { bits: 0x66c00000 }
    }

    /// Read **LP_AONCLKRST_DAC_XTAL32K** `[31:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_dac_xtal32k(&self) -> u32 {
        (self.bits & LpAonclkrstXtal32k::LP_AONCLKRST_DAC_XTAL32K_MASK)
            >> LpAonclkrstXtal32k::LP_AONCLKRST_DAC_XTAL32K_OFFSET
    }

    /// Read **LP_AONCLKRST_DBUF_XTAL32K** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_dbuf_xtal32k(&self) -> bool {
        (self.bits & LpAonclkrstXtal32k::LP_AONCLKRST_DBUF_XTAL32K_MASK) != 0
    }

    /// Read **LP_AONCLKRST_DGM_XTAL32K** `[27:25]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_dgm_xtal32k(&self) -> u32 {
        (self.bits & LpAonclkrstXtal32k::LP_AONCLKRST_DGM_XTAL32K_MASK)
            >> LpAonclkrstXtal32k::LP_AONCLKRST_DGM_XTAL32K_OFFSET
    }

    /// Read **LP_AONCLKRST_DRES_XTAL32K** `[24:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_dres_xtal32k(&self) -> u32 {
        (self.bits & LpAonclkrstXtal32k::LP_AONCLKRST_DRES_XTAL32K_MASK)
            >> LpAonclkrstXtal32k::LP_AONCLKRST_DRES_XTAL32K_OFFSET
    }

    /// Set **LP_AONCLKRST_DAC_XTAL32K** `[31:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_dac_xtal32k(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstXtal32k::LP_AONCLKRST_DAC_XTAL32K_MASK)
                | ((value << LpAonclkrstXtal32k::LP_AONCLKRST_DAC_XTAL32K_OFFSET)
                    & LpAonclkrstXtal32k::LP_AONCLKRST_DAC_XTAL32K_MASK),
        }
    }

    /// Set **LP_AONCLKRST_DBUF_XTAL32K** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_dbuf_xtal32k(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstXtal32k::LP_AONCLKRST_DBUF_XTAL32K_MASK
            } else {
                self.bits & !LpAonclkrstXtal32k::LP_AONCLKRST_DBUF_XTAL32K_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_DGM_XTAL32K** `[27:25]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_dgm_xtal32k(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstXtal32k::LP_AONCLKRST_DGM_XTAL32K_MASK)
                | ((value << LpAonclkrstXtal32k::LP_AONCLKRST_DGM_XTAL32K_OFFSET)
                    & LpAonclkrstXtal32k::LP_AONCLKRST_DGM_XTAL32K_MASK),
        }
    }

    /// Set **LP_AONCLKRST_DRES_XTAL32K** `[24:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_dres_xtal32k(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstXtal32k::LP_AONCLKRST_DRES_XTAL32K_MASK)
                | ((value << LpAonclkrstXtal32k::LP_AONCLKRST_DRES_XTAL32K_OFFSET)
                    & LpAonclkrstXtal32k::LP_AONCLKRST_DRES_XTAL32K_MASK),
        }
    }
}

impl Default for LpAonclkrstXtal32kVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstXtal32k {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstXtal32kVal {
        LpAonclkrstXtal32kVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstXtal32kVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstXtal32kVal) -> LpAonclkrstXtal32kVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS Register
///
/// need_des
///
/// Address offset: 0x0034
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct LpAonclkrstMuxHpsysResetBypass {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstMuxHpsysResetBypass {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstMuxHpsysResetBypass {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_OFFSET: u32 = 0;
    pub const LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn lp_aonclkrst_mux_hpsys_reset_bypass(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_MASK)
            >> Self::LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_OFFSET
    }

    /// Write **LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_lp_aonclkrst_mux_hpsys_reset_bypass(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_MASK)
                | ((value << Self::LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_OFFSET)
                    & Self::LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_MASK)
        });
    }
}

/// LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS value type for batch read/modify/write operations
///
/// Use `lpaonclkrstmuxhpsysresetbypass.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrstmuxhpsysresetbypass.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstMuxHpsysResetBypassVal {
    bits: u32,
}

impl LpAonclkrstMuxHpsysResetBypassVal {
    /// Create from raw bits
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

    /// Read **LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn lp_aonclkrst_mux_hpsys_reset_bypass(&self) -> u32 {
        (self.bits & LpAonclkrstMuxHpsysResetBypass::LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_MASK)
            >> LpAonclkrstMuxHpsysResetBypass::LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_OFFSET
    }

    /// Set **LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_lp_aonclkrst_mux_hpsys_reset_bypass(self, value: u32) -> Self {
        Self {
            bits: (self.bits
                & !LpAonclkrstMuxHpsysResetBypass::LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_MASK)
                | ((value
                    << LpAonclkrstMuxHpsysResetBypass::LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_OFFSET)
                    & LpAonclkrstMuxHpsysResetBypass::LP_AONCLKRST_MUX_HPSYS_RESET_BYPASS_MASK),
        }
    }
}

impl Default for LpAonclkrstMuxHpsysResetBypassVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstMuxHpsysResetBypass {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstMuxHpsysResetBypassVal {
        LpAonclkrstMuxHpsysResetBypassVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstMuxHpsysResetBypassVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<
        F: FnOnce(LpAonclkrstMuxHpsysResetBypassVal) -> LpAonclkrstMuxHpsysResetBypassVal,
    >(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_HPSYS_0_RESET_BYPASS Register
///
/// need_des
///
/// Address offset: 0x0038
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **LP_AONCLKRST_HPSYS_0_RESET_BYPASS** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct LpAonclkrstHpsys0ResetBypass {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstHpsys0ResetBypass {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstHpsys0ResetBypass {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_HPSYS_0_RESET_BYPASS**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_AONCLKRST_HPSYS_0_RESET_BYPASS_OFFSET: u32 = 0;
    pub const LP_AONCLKRST_HPSYS_0_RESET_BYPASS_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_HPSYS_0_RESET_BYPASS** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn lp_aonclkrst_hpsys_0_reset_bypass(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPSYS_0_RESET_BYPASS_MASK)
            >> Self::LP_AONCLKRST_HPSYS_0_RESET_BYPASS_OFFSET
    }

    /// Write **LP_AONCLKRST_HPSYS_0_RESET_BYPASS** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_lp_aonclkrst_hpsys_0_reset_bypass(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_HPSYS_0_RESET_BYPASS_MASK)
                | ((value << Self::LP_AONCLKRST_HPSYS_0_RESET_BYPASS_OFFSET)
                    & Self::LP_AONCLKRST_HPSYS_0_RESET_BYPASS_MASK)
        });
    }
}

/// LP_AONCLKRST_HPSYS_0_RESET_BYPASS value type for batch read/modify/write operations
///
/// Use `lpaonclkrsthpsys0resetbypass.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrsthpsys0resetbypass.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstHpsys0ResetBypassVal {
    bits: u32,
}

impl LpAonclkrstHpsys0ResetBypassVal {
    /// Create from raw bits
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

    /// Read **LP_AONCLKRST_HPSYS_0_RESET_BYPASS** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn lp_aonclkrst_hpsys_0_reset_bypass(&self) -> u32 {
        (self.bits & LpAonclkrstHpsys0ResetBypass::LP_AONCLKRST_HPSYS_0_RESET_BYPASS_MASK)
            >> LpAonclkrstHpsys0ResetBypass::LP_AONCLKRST_HPSYS_0_RESET_BYPASS_OFFSET
    }

    /// Set **LP_AONCLKRST_HPSYS_0_RESET_BYPASS** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpsys_0_reset_bypass(self, value: u32) -> Self {
        Self {
            bits: (self.bits
                & !LpAonclkrstHpsys0ResetBypass::LP_AONCLKRST_HPSYS_0_RESET_BYPASS_MASK)
                | ((value
                    << LpAonclkrstHpsys0ResetBypass::LP_AONCLKRST_HPSYS_0_RESET_BYPASS_OFFSET)
                    & LpAonclkrstHpsys0ResetBypass::LP_AONCLKRST_HPSYS_0_RESET_BYPASS_MASK),
        }
    }
}

impl Default for LpAonclkrstHpsys0ResetBypassVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstHpsys0ResetBypass {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstHpsys0ResetBypassVal {
        LpAonclkrstHpsys0ResetBypassVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstHpsys0ResetBypassVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<
        F: FnOnce(LpAonclkrstHpsys0ResetBypassVal) -> LpAonclkrstHpsys0ResetBypassVal,
    >(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_HPSYS_APM_RESET_BYPASS Register
///
/// need_des
///
/// Address offset: 0x003c
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **LP_AONCLKRST_HPSYS_APM_RESET_BYPASS** `[31:0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct LpAonclkrstHpsysApmResetBypass {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstHpsysApmResetBypass {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstHpsysApmResetBypass {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_HPSYS_APM_RESET_BYPASS**
    ///
    /// reserved
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_AONCLKRST_HPSYS_APM_RESET_BYPASS_OFFSET: u32 = 0;
    pub const LP_AONCLKRST_HPSYS_APM_RESET_BYPASS_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_HPSYS_APM_RESET_BYPASS** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn lp_aonclkrst_hpsys_apm_reset_bypass(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_HPSYS_APM_RESET_BYPASS_MASK)
            >> Self::LP_AONCLKRST_HPSYS_APM_RESET_BYPASS_OFFSET
    }

    /// Write **LP_AONCLKRST_HPSYS_APM_RESET_BYPASS** `[31:0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn write_lp_aonclkrst_hpsys_apm_reset_bypass(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_HPSYS_APM_RESET_BYPASS_MASK)
                | ((value << Self::LP_AONCLKRST_HPSYS_APM_RESET_BYPASS_OFFSET)
                    & Self::LP_AONCLKRST_HPSYS_APM_RESET_BYPASS_MASK)
        });
    }
}

/// LP_AONCLKRST_HPSYS_APM_RESET_BYPASS value type for batch read/modify/write operations
///
/// Use `lpaonclkrsthpsysapmresetbypass.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrsthpsysapmresetbypass.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstHpsysApmResetBypassVal {
    bits: u32,
}

impl LpAonclkrstHpsysApmResetBypassVal {
    /// Create from raw bits
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

    /// Read **LP_AONCLKRST_HPSYS_APM_RESET_BYPASS** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn lp_aonclkrst_hpsys_apm_reset_bypass(&self) -> u32 {
        (self.bits & LpAonclkrstHpsysApmResetBypass::LP_AONCLKRST_HPSYS_APM_RESET_BYPASS_MASK)
            >> LpAonclkrstHpsysApmResetBypass::LP_AONCLKRST_HPSYS_APM_RESET_BYPASS_OFFSET
    }

    /// Set **LP_AONCLKRST_HPSYS_APM_RESET_BYPASS** `[31:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hpsys_apm_reset_bypass(self, value: u32) -> Self {
        Self {
            bits: (self.bits
                & !LpAonclkrstHpsysApmResetBypass::LP_AONCLKRST_HPSYS_APM_RESET_BYPASS_MASK)
                | ((value
                    << LpAonclkrstHpsysApmResetBypass::LP_AONCLKRST_HPSYS_APM_RESET_BYPASS_OFFSET)
                    & LpAonclkrstHpsysApmResetBypass::LP_AONCLKRST_HPSYS_APM_RESET_BYPASS_MASK),
        }
    }
}

impl Default for LpAonclkrstHpsysApmResetBypassVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstHpsysApmResetBypass {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstHpsysApmResetBypassVal {
        LpAonclkrstHpsysApmResetBypassVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstHpsysApmResetBypassVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<
        F: FnOnce(LpAonclkrstHpsysApmResetBypassVal) -> LpAonclkrstHpsysApmResetBypassVal,
    >(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_HP_CLK_CTRL Register
///
/// HP Clock Control Register.
///
/// Address offset: 0x0040
/// Reset value: 0x1ffffffc
///
/// ## Fields
///
/// - **LP_AONCLKRST_HP_MPLL_500M_CLK_EN** `[28]` (RW)
///
///   MPLL 500M Clock Enable.
///
/// - **LP_AONCLKRST_HP_SPLL_480M_CLK_EN** `[27]` (RW)
///
///   SPLL 480M Clock Enable.
///
/// - **LP_AONCLKRST_HP_CPLL_400M_CLK_EN** `[26]` (RW)
///
///   CPLL 400M Clock Enable.
///
/// - **LP_AONCLKRST_HP_XTAL_40M_CLK_EN** `[25]` (RW)
///
///   XTAL 40M Clock Enalbe.
///
/// - **LP_AONCLKRST_HP_FOSC_20M_CLK_EN** `[24]` (RW)
///
///   FOSC 20M Clock Enable.
///
/// - **LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN** `[23]` (RW)
///
///   SDIO PLL0 Clock Enable.
///
/// - **LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN** `[22]` (RW)
///
///   SDIO PLL1 Clock Enable.
///
/// - **LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN** `[21]` (RW)
///
///   SDIO PLL2 Clock Enable.
///
/// - **LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN** `[20]` (RW)
///
///   AUDIO PLL Clock Enable.
///
/// - **LP_AONCLKRST_HP_PLL_8M_CLK_EN** `[19]` (RW)
///
///   PLL 8M Clock Enable.
///
/// - **LP_AONCLKRST_HP_SOSC_150K_CLK_EN** `[18]` (RW)
///
///   SOSC 150K Clock Enable.
///
/// - **LP_AONCLKRST_HP_RC_32K_CLK_EN** `[17]` (RW)
///
///   RC 32K Clock Enable.
///
/// - **LP_AONCLKRST_HP_XTAL_32K_CLK_EN** `[16]` (RW)
///
///   XTAL 32K Clock Enable.
///
/// - **LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN** `[15]` (RW)
///
///   EMAC TXRX Clock From Pad Enable.
///
/// - **LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN** `[14]` (RW)
///
///   EMAC TX Clock From Pad Enable.
///
/// - **LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN** `[13]` (RW)
///
///   EMAC RX Clock From Pad Enable.
///
/// - **LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN** `[12]` (RW)
///
///   I2S0 MCLK Clock From Pad Enable.
///
/// - **LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN** `[11]` (RW)
///
///   I2S1 MCLK Clock From Pad Enable.
///
/// - **LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN** `[10]` (RW)
///
///   I2S2 MCLK Clock From Pad Enable.
///
/// - **LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN** `[9]` (RW)
///
///   UART0 SLP Clock From Pad Enable.
///
/// - **LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN** `[8]` (RW)
///
///   UART1 SLP Clock From Pad Enable.
///
/// - **LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN** `[7]` (RW)
///
///   UART2 SLP Clock From Pad Enable.
///
/// - **LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN** `[6]` (RW)
///
///   UART3 SLP Clock From Pad Enable.
///
/// - **LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN** `[5]` (RW)
///
///   UART4 SLP Clock From Pad Enable.
///
/// - **LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN** `[4]` (RW)
///
///   PARLIO RX Clock From Pad Enable.
///
/// - **LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN** `[3]` (RW)
///
///   PARLIO TX Clock From Pad Enable.
///
/// - **LP_AONCLKRST_HP_ROOT_CLK_EN** `[2]` (RW)
///
///   HP SoC Root Clock Enable.
///
/// - **LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL** `[1:0]` (RW)
///
///   HP SoC Root Clock Source Select. 2'd0: xtal_40m, 2'd1: cpll_400m, 2'd2: fosc_20m.
#[repr(transparent)]
pub struct LpAonclkrstHpClkCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstHpClkCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstHpClkCtrl {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_HP_MPLL_500M_CLK_EN**
    ///
    /// MPLL 500M Clock Enable.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_AONCLKRST_HP_MPLL_500M_CLK_EN_OFFSET: u32 = 28;
    pub const LP_AONCLKRST_HP_MPLL_500M_CLK_EN_MASK: u32 = 0x10000000;
    pub const LP_AONCLKRST_HP_MPLL_500M_CLK_EN: u32 = 1 << 28;

    /// **LP_AONCLKRST_HP_SPLL_480M_CLK_EN**
    ///
    /// SPLL 480M Clock Enable.
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LP_AONCLKRST_HP_SPLL_480M_CLK_EN_OFFSET: u32 = 27;
    pub const LP_AONCLKRST_HP_SPLL_480M_CLK_EN_MASK: u32 = 0x08000000;
    pub const LP_AONCLKRST_HP_SPLL_480M_CLK_EN: u32 = 1 << 27;

    /// **LP_AONCLKRST_HP_CPLL_400M_CLK_EN**
    ///
    /// CPLL 400M Clock Enable.
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const LP_AONCLKRST_HP_CPLL_400M_CLK_EN_OFFSET: u32 = 26;
    pub const LP_AONCLKRST_HP_CPLL_400M_CLK_EN_MASK: u32 = 0x04000000;
    pub const LP_AONCLKRST_HP_CPLL_400M_CLK_EN: u32 = 1 << 26;

    /// **LP_AONCLKRST_HP_XTAL_40M_CLK_EN**
    ///
    /// XTAL 40M Clock Enalbe.
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const LP_AONCLKRST_HP_XTAL_40M_CLK_EN_OFFSET: u32 = 25;
    pub const LP_AONCLKRST_HP_XTAL_40M_CLK_EN_MASK: u32 = 0x02000000;
    pub const LP_AONCLKRST_HP_XTAL_40M_CLK_EN: u32 = 1 << 25;

    /// **LP_AONCLKRST_HP_FOSC_20M_CLK_EN**
    ///
    /// FOSC 20M Clock Enable.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const LP_AONCLKRST_HP_FOSC_20M_CLK_EN_OFFSET: u32 = 24;
    pub const LP_AONCLKRST_HP_FOSC_20M_CLK_EN_MASK: u32 = 0x01000000;
    pub const LP_AONCLKRST_HP_FOSC_20M_CLK_EN: u32 = 1 << 24;

    /// **LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN**
    ///
    /// SDIO PLL0 Clock Enable.
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN_OFFSET: u32 = 23;
    pub const LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN_MASK: u32 = 0x00800000;
    pub const LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN: u32 = 1 << 23;

    /// **LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN**
    ///
    /// SDIO PLL1 Clock Enable.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN_OFFSET: u32 = 22;
    pub const LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN_MASK: u32 = 0x00400000;
    pub const LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN: u32 = 1 << 22;

    /// **LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN**
    ///
    /// SDIO PLL2 Clock Enable.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN_OFFSET: u32 = 21;
    pub const LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN_MASK: u32 = 0x00200000;
    pub const LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN: u32 = 1 << 21;

    /// **LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN**
    ///
    /// AUDIO PLL Clock Enable.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN_OFFSET: u32 = 20;
    pub const LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN_MASK: u32 = 0x00100000;
    pub const LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN: u32 = 1 << 20;

    /// **LP_AONCLKRST_HP_PLL_8M_CLK_EN**
    ///
    /// PLL 8M Clock Enable.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const LP_AONCLKRST_HP_PLL_8M_CLK_EN_OFFSET: u32 = 19;
    pub const LP_AONCLKRST_HP_PLL_8M_CLK_EN_MASK: u32 = 0x00080000;
    pub const LP_AONCLKRST_HP_PLL_8M_CLK_EN: u32 = 1 << 19;

    /// **LP_AONCLKRST_HP_SOSC_150K_CLK_EN**
    ///
    /// SOSC 150K Clock Enable.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const LP_AONCLKRST_HP_SOSC_150K_CLK_EN_OFFSET: u32 = 18;
    pub const LP_AONCLKRST_HP_SOSC_150K_CLK_EN_MASK: u32 = 0x00040000;
    pub const LP_AONCLKRST_HP_SOSC_150K_CLK_EN: u32 = 1 << 18;

    /// **LP_AONCLKRST_HP_RC_32K_CLK_EN**
    ///
    /// RC 32K Clock Enable.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const LP_AONCLKRST_HP_RC_32K_CLK_EN_OFFSET: u32 = 17;
    pub const LP_AONCLKRST_HP_RC_32K_CLK_EN_MASK: u32 = 0x00020000;
    pub const LP_AONCLKRST_HP_RC_32K_CLK_EN: u32 = 1 << 17;

    /// **LP_AONCLKRST_HP_XTAL_32K_CLK_EN**
    ///
    /// XTAL 32K Clock Enable.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const LP_AONCLKRST_HP_XTAL_32K_CLK_EN_OFFSET: u32 = 16;
    pub const LP_AONCLKRST_HP_XTAL_32K_CLK_EN_MASK: u32 = 0x00010000;
    pub const LP_AONCLKRST_HP_XTAL_32K_CLK_EN: u32 = 1 << 16;

    /// **LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN**
    ///
    /// EMAC TXRX Clock From Pad Enable.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN_OFFSET: u32 = 15;
    pub const LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN_MASK: u32 = 0x00008000;
    pub const LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN: u32 = 1 << 15;

    /// **LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN**
    ///
    /// EMAC TX Clock From Pad Enable.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN_OFFSET: u32 = 14;
    pub const LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN_MASK: u32 = 0x00004000;
    pub const LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN: u32 = 1 << 14;

    /// **LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN**
    ///
    /// EMAC RX Clock From Pad Enable.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN_OFFSET: u32 = 13;
    pub const LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN_MASK: u32 = 0x00002000;
    pub const LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN: u32 = 1 << 13;

    /// **LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN**
    ///
    /// I2S0 MCLK Clock From Pad Enable.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN_OFFSET: u32 = 12;
    pub const LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN_MASK: u32 = 0x00001000;
    pub const LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN: u32 = 1 << 12;

    /// **LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN**
    ///
    /// I2S1 MCLK Clock From Pad Enable.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN_OFFSET: u32 = 11;
    pub const LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN_MASK: u32 = 0x00000800;
    pub const LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN: u32 = 1 << 11;

    /// **LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN**
    ///
    /// I2S2 MCLK Clock From Pad Enable.
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN_OFFSET: u32 = 10;
    pub const LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN_MASK: u32 = 0x00000400;
    pub const LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN: u32 = 1 << 10;

    /// **LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN**
    ///
    /// UART0 SLP Clock From Pad Enable.
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN_OFFSET: u32 = 9;
    pub const LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN_MASK: u32 = 0x00000200;
    pub const LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN: u32 = 1 << 9;

    /// **LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN**
    ///
    /// UART1 SLP Clock From Pad Enable.
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN_OFFSET: u32 = 8;
    pub const LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN_MASK: u32 = 0x00000100;
    pub const LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN: u32 = 1 << 8;

    /// **LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN**
    ///
    /// UART2 SLP Clock From Pad Enable.
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN_OFFSET: u32 = 7;
    pub const LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN_MASK: u32 = 0x00000080;
    pub const LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN: u32 = 1 << 7;

    /// **LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN**
    ///
    /// UART3 SLP Clock From Pad Enable.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN_OFFSET: u32 = 6;
    pub const LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN_MASK: u32 = 0x00000040;
    pub const LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN: u32 = 1 << 6;

    /// **LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN**
    ///
    /// UART4 SLP Clock From Pad Enable.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN_OFFSET: u32 = 5;
    pub const LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN_MASK: u32 = 0x00000020;
    pub const LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN: u32 = 1 << 5;

    /// **LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN**
    ///
    /// PARLIO RX Clock From Pad Enable.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN_OFFSET: u32 = 4;
    pub const LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN_MASK: u32 = 0x00000010;
    pub const LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN: u32 = 1 << 4;

    /// **LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN**
    ///
    /// PARLIO TX Clock From Pad Enable.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN_OFFSET: u32 = 3;
    pub const LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN_MASK: u32 = 0x00000008;
    pub const LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN: u32 = 1 << 3;

    /// **LP_AONCLKRST_HP_ROOT_CLK_EN**
    ///
    /// HP SoC Root Clock Enable.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const LP_AONCLKRST_HP_ROOT_CLK_EN_OFFSET: u32 = 2;
    pub const LP_AONCLKRST_HP_ROOT_CLK_EN_MASK: u32 = 0x00000004;
    pub const LP_AONCLKRST_HP_ROOT_CLK_EN: u32 = 1 << 2;

    /// **LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL**
    ///
    /// HP SoC Root Clock Source Select. 2'd0: xtal_40m, 2'd1: cpll_400m, 2'd2: fosc_20m.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_OFFSET: u32 = 0;
    pub const LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_HP_MPLL_500M_CLK_EN** `[28]` (RW)
    ///
    /// MPLL 500M Clock Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_mpll_500m_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_MPLL_500M_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_MPLL_500M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_mpll_500m_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_MPLL_500M_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_MPLL_500M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_mpll_500m_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_MPLL_500M_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_SPLL_480M_CLK_EN** `[27]` (RW)
    ///
    /// SPLL 480M Clock Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_spll_480m_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_SPLL_480M_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_SPLL_480M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_spll_480m_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_SPLL_480M_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_SPLL_480M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_spll_480m_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_SPLL_480M_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_CPLL_400M_CLK_EN** `[26]` (RW)
    ///
    /// CPLL 400M Clock Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_cpll_400m_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_CPLL_400M_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_CPLL_400M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_cpll_400m_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_CPLL_400M_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_CPLL_400M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_cpll_400m_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_CPLL_400M_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_XTAL_40M_CLK_EN** `[25]` (RW)
    ///
    /// XTAL 40M Clock Enalbe.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_xtal_40m_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_XTAL_40M_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_XTAL_40M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_xtal_40m_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_XTAL_40M_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_XTAL_40M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_xtal_40m_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_XTAL_40M_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_FOSC_20M_CLK_EN** `[24]` (RW)
    ///
    /// FOSC 20M Clock Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_fosc_20m_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_FOSC_20M_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_FOSC_20M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_fosc_20m_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_FOSC_20M_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_FOSC_20M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_fosc_20m_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_FOSC_20M_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN** `[23]` (RW)
    ///
    /// SDIO PLL0 Clock Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_sdio_pll0_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_sdio_pll0_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_sdio_pll0_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN** `[22]` (RW)
    ///
    /// SDIO PLL1 Clock Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_sdio_pll1_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_sdio_pll1_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_sdio_pll1_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN** `[21]` (RW)
    ///
    /// SDIO PLL2 Clock Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_sdio_pll2_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_sdio_pll2_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_sdio_pll2_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN** `[20]` (RW)
    ///
    /// AUDIO PLL Clock Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_audio_pll_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_audio_pll_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_audio_pll_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_PLL_8M_CLK_EN** `[19]` (RW)
    ///
    /// PLL 8M Clock Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pll_8m_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_PLL_8M_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_PLL_8M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_pll_8m_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_PLL_8M_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_PLL_8M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_pll_8m_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_PLL_8M_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_SOSC_150K_CLK_EN** `[18]` (RW)
    ///
    /// SOSC 150K Clock Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_sosc_150k_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_SOSC_150K_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_SOSC_150K_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_sosc_150k_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_SOSC_150K_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_SOSC_150K_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_sosc_150k_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_SOSC_150K_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_RC_32K_CLK_EN** `[17]` (RW)
    ///
    /// RC 32K Clock Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_rc_32k_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_RC_32K_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_RC_32K_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_rc_32k_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_RC_32K_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_RC_32K_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_rc_32k_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_RC_32K_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_XTAL_32K_CLK_EN** `[16]` (RW)
    ///
    /// XTAL 32K Clock Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_xtal_32k_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_XTAL_32K_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_XTAL_32K_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_xtal_32k_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_XTAL_32K_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_XTAL_32K_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_xtal_32k_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_XTAL_32K_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN** `[15]` (RW)
    ///
    /// EMAC TXRX Clock From Pad Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_emac_txrx_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_pad_emac_txrx_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_pad_emac_txrx_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN** `[14]` (RW)
    ///
    /// EMAC TX Clock From Pad Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_emac_rx_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_pad_emac_rx_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_pad_emac_rx_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN** `[13]` (RW)
    ///
    /// EMAC RX Clock From Pad Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_emac_tx_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_pad_emac_tx_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_pad_emac_tx_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN** `[12]` (RW)
    ///
    /// I2S0 MCLK Clock From Pad Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_i2s0_mclk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_pad_i2s0_mclk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_pad_i2s0_mclk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN** `[11]` (RW)
    ///
    /// I2S1 MCLK Clock From Pad Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_i2s1_mclk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_pad_i2s1_mclk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_pad_i2s1_mclk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN** `[10]` (RW)
    ///
    /// I2S2 MCLK Clock From Pad Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_i2s2_mclk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_pad_i2s2_mclk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_pad_i2s2_mclk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN** `[9]` (RW)
    ///
    /// UART0 SLP Clock From Pad Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_uart0_slp_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_pad_uart0_slp_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_pad_uart0_slp_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN** `[8]` (RW)
    ///
    /// UART1 SLP Clock From Pad Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_uart1_slp_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_pad_uart1_slp_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_pad_uart1_slp_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN** `[7]` (RW)
    ///
    /// UART2 SLP Clock From Pad Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_uart2_slp_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_pad_uart2_slp_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_pad_uart2_slp_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN** `[6]` (RW)
    ///
    /// UART3 SLP Clock From Pad Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_uart3_slp_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_pad_uart3_slp_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_pad_uart3_slp_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN** `[5]` (RW)
    ///
    /// UART4 SLP Clock From Pad Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_uart4_slp_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_pad_uart4_slp_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_pad_uart4_slp_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN** `[4]` (RW)
    ///
    /// PARLIO RX Clock From Pad Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_parlio_rx_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_pad_parlio_rx_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_pad_parlio_rx_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN** `[3]` (RW)
    ///
    /// PARLIO TX Clock From Pad Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_pad_parlio_tx_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_pad_parlio_tx_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_pad_parlio_tx_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_ROOT_CLK_EN** `[2]` (RW)
    ///
    /// HP SoC Root Clock Enable.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_root_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_ROOT_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_HP_ROOT_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_hp_root_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_HP_ROOT_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_HP_ROOT_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_hp_root_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_HP_ROOT_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL** `[1:0]` (RW)
    ///
    /// HP SoC Root Clock Source Select. 2'd0: xtal_40m, 2'd1: cpll_400m, 2'd2: fosc_20m.
    #[inline(always)]
    pub fn lp_aonclkrst_hp_root_clk_src_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_MASK)
            >> Self::LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_OFFSET
    }

    /// Write **LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL** `[1:0]` (RW)
    ///
    /// HP SoC Root Clock Source Select. 2'd0: xtal_40m, 2'd1: cpll_400m, 2'd2: fosc_20m.
    #[inline(always)]
    pub fn write_lp_aonclkrst_hp_root_clk_src_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_MASK)
                | ((value << Self::LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_OFFSET)
                    & Self::LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_MASK)
        });
    }
}

/// LP_AONCLKRST_HP_CLK_CTRL value type for batch read/modify/write operations
///
/// Use `lpaonclkrsthpclkctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrsthpclkctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstHpClkCtrlVal {
    bits: u32,
}

impl LpAonclkrstHpClkCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x1ffffffc }
    }

    /// Read **LP_AONCLKRST_HP_MPLL_500M_CLK_EN** `[28:28]`
    ///
    /// MPLL 500M Clock Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_mpll_500m_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_MPLL_500M_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_SPLL_480M_CLK_EN** `[27:27]`
    ///
    /// SPLL 480M Clock Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_spll_480m_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SPLL_480M_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_CPLL_400M_CLK_EN** `[26:26]`
    ///
    /// CPLL 400M Clock Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_cpll_400m_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_CPLL_400M_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_XTAL_40M_CLK_EN** `[25:25]`
    ///
    /// XTAL 40M Clock Enalbe.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_xtal_40m_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_XTAL_40M_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_FOSC_20M_CLK_EN** `[24:24]`
    ///
    /// FOSC 20M Clock Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_fosc_20m_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_FOSC_20M_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN** `[23:23]`
    ///
    /// SDIO PLL0 Clock Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_sdio_pll0_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN** `[22:22]`
    ///
    /// SDIO PLL1 Clock Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_sdio_pll1_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN** `[21:21]`
    ///
    /// SDIO PLL2 Clock Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_sdio_pll2_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN** `[20:20]`
    ///
    /// AUDIO PLL Clock Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_audio_pll_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_PLL_8M_CLK_EN** `[19:19]`
    ///
    /// PLL 8M Clock Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_pll_8m_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PLL_8M_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_SOSC_150K_CLK_EN** `[18:18]`
    ///
    /// SOSC 150K Clock Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_sosc_150k_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SOSC_150K_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_RC_32K_CLK_EN** `[17:17]`
    ///
    /// RC 32K Clock Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_rc_32k_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_RC_32K_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_XTAL_32K_CLK_EN** `[16:16]`
    ///
    /// XTAL 32K Clock Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_xtal_32k_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_XTAL_32K_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN** `[15:15]`
    ///
    /// EMAC TXRX Clock From Pad Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_pad_emac_txrx_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN** `[14:14]`
    ///
    /// EMAC TX Clock From Pad Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_pad_emac_rx_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN** `[13:13]`
    ///
    /// EMAC RX Clock From Pad Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_pad_emac_tx_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN** `[12:12]`
    ///
    /// I2S0 MCLK Clock From Pad Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_pad_i2s0_mclk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN** `[11:11]`
    ///
    /// I2S1 MCLK Clock From Pad Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_pad_i2s1_mclk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN** `[10:10]`
    ///
    /// I2S2 MCLK Clock From Pad Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_pad_i2s2_mclk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN** `[9:9]`
    ///
    /// UART0 SLP Clock From Pad Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_pad_uart0_slp_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN** `[8:8]`
    ///
    /// UART1 SLP Clock From Pad Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_pad_uart1_slp_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN** `[7:7]`
    ///
    /// UART2 SLP Clock From Pad Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_pad_uart2_slp_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN** `[6:6]`
    ///
    /// UART3 SLP Clock From Pad Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_pad_uart3_slp_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN** `[5:5]`
    ///
    /// UART4 SLP Clock From Pad Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_pad_uart4_slp_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN** `[4:4]`
    ///
    /// PARLIO RX Clock From Pad Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_pad_parlio_rx_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN** `[3:3]`
    ///
    /// PARLIO TX Clock From Pad Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_pad_parlio_tx_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_ROOT_CLK_EN** `[2:2]`
    ///
    /// HP SoC Root Clock Enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_root_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_ROOT_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL** `[1:0]`
    ///
    /// HP SoC Root Clock Source Select. 2'd0: xtal_40m, 2'd1: cpll_400m, 2'd2: fosc_20m.
    #[inline(always)]
    pub const fn lp_aonclkrst_hp_root_clk_src_sel(&self) -> u32 {
        (self.bits & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_MASK)
            >> LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_OFFSET
    }

    /// Set **LP_AONCLKRST_HP_MPLL_500M_CLK_EN** `[28:28]`
    ///
    /// MPLL 500M Clock Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_mpll_500m_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_MPLL_500M_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_MPLL_500M_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_SPLL_480M_CLK_EN** `[27:27]`
    ///
    /// SPLL 480M Clock Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_spll_480m_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SPLL_480M_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SPLL_480M_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_CPLL_400M_CLK_EN** `[26:26]`
    ///
    /// CPLL 400M Clock Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_cpll_400m_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_CPLL_400M_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_CPLL_400M_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_XTAL_40M_CLK_EN** `[25:25]`
    ///
    /// XTAL 40M Clock Enalbe.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_xtal_40m_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_XTAL_40M_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_XTAL_40M_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_FOSC_20M_CLK_EN** `[24:24]`
    ///
    /// FOSC 20M Clock Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_fosc_20m_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_FOSC_20M_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_FOSC_20M_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN** `[23:23]`
    ///
    /// SDIO PLL0 Clock Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_sdio_pll0_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SDIO_PLL0_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN** `[22:22]`
    ///
    /// SDIO PLL1 Clock Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_sdio_pll1_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SDIO_PLL1_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN** `[21:21]`
    ///
    /// SDIO PLL2 Clock Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_sdio_pll2_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SDIO_PLL2_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN** `[20:20]`
    ///
    /// AUDIO PLL Clock Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_audio_pll_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_AUDIO_PLL_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_PLL_8M_CLK_EN** `[19:19]`
    ///
    /// PLL 8M Clock Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_pll_8m_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PLL_8M_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PLL_8M_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_SOSC_150K_CLK_EN** `[18:18]`
    ///
    /// SOSC 150K Clock Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_sosc_150k_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SOSC_150K_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_SOSC_150K_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_RC_32K_CLK_EN** `[17:17]`
    ///
    /// RC 32K Clock Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_rc_32k_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_RC_32K_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_RC_32K_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_XTAL_32K_CLK_EN** `[16:16]`
    ///
    /// XTAL 32K Clock Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_xtal_32k_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_XTAL_32K_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_XTAL_32K_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN** `[15:15]`
    ///
    /// EMAC TXRX Clock From Pad Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_pad_emac_txrx_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_EMAC_TXRX_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN** `[14:14]`
    ///
    /// EMAC TX Clock From Pad Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_pad_emac_rx_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_EMAC_RX_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN** `[13:13]`
    ///
    /// EMAC RX Clock From Pad Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_pad_emac_tx_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_EMAC_TX_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN** `[12:12]`
    ///
    /// I2S0 MCLK Clock From Pad Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_pad_i2s0_mclk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_I2S0_MCLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN** `[11:11]`
    ///
    /// I2S1 MCLK Clock From Pad Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_pad_i2s1_mclk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_I2S1_MCLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN** `[10:10]`
    ///
    /// I2S2 MCLK Clock From Pad Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_pad_i2s2_mclk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_I2S2_MCLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN** `[9:9]`
    ///
    /// UART0 SLP Clock From Pad Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_pad_uart0_slp_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART0_SLP_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN** `[8:8]`
    ///
    /// UART1 SLP Clock From Pad Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_pad_uart1_slp_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART1_SLP_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN** `[7:7]`
    ///
    /// UART2 SLP Clock From Pad Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_pad_uart2_slp_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART2_SLP_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN** `[6:6]`
    ///
    /// UART3 SLP Clock From Pad Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_pad_uart3_slp_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART3_SLP_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN** `[5:5]`
    ///
    /// UART4 SLP Clock From Pad Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_pad_uart4_slp_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_UART4_SLP_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN** `[4:4]`
    ///
    /// PARLIO RX Clock From Pad Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_pad_parlio_rx_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_PARLIO_RX_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN** `[3:3]`
    ///
    /// PARLIO TX Clock From Pad Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_pad_parlio_tx_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_PAD_PARLIO_TX_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_ROOT_CLK_EN** `[2:2]`
    ///
    /// HP SoC Root Clock Enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_root_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_ROOT_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_ROOT_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL** `[1:0]`
    ///
    /// HP SoC Root Clock Source Select. 2'd0: xtal_40m, 2'd1: cpll_400m, 2'd2: fosc_20m.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_hp_root_clk_src_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_MASK)
                | ((value << LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_OFFSET)
                    & LpAonclkrstHpClkCtrl::LP_AONCLKRST_HP_ROOT_CLK_SRC_SEL_MASK),
        }
    }
}

impl Default for LpAonclkrstHpClkCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstHpClkCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstHpClkCtrlVal {
        LpAonclkrstHpClkCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstHpClkCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstHpClkCtrlVal) -> LpAonclkrstHpClkCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_HP_USB_CLKRST_CTRL0 Register
///
/// HP USB Clock Reset Control Register.
///
/// Address offset: 0x0044
/// Reset value: 0x09c4c27a
///
/// ## Fields
///
/// - **LP_AONCLKRST_USB_12M_DIV_NUM** `[29:22]` (RW)
///
///   usb 480m to 12m divide number.
///
/// - **LP_AONCLKRST_USB_25M_DIV_NUM** `[21:14]` (RW)
///
///   usb 500m to 25m divide number.
///
/// - **LP_AONCLKRST_USB_48M_DIV_NUM** `[13:6]` (RW)
///
///   usb 480m to 25m divide number.
///
/// - **LP_AONCLKRST_USB_DEVICE_48M_CLK_EN** `[5]` (RW)
///
///   usb device fs phy clock enable.
///
/// - **LP_AONCLKRST_USB_OTG11_48M_CLK_EN** `[4]` (RW)
///
///   usb otg11 fs phy clock enable.
///
/// - **LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN** `[3]` (RW)
///
///   unused.
///
/// - **LP_AONCLKRST_USB_OTG11_SLEEP_MODE** `[2]` (RW)
///
///   unused.
///
/// - **LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN** `[1]` (RW)
///
///   unused.
///
/// - **LP_AONCLKRST_USB_OTG20_SLEEP_MODE** `[0]` (RW)
///
///   unused.
#[repr(transparent)]
pub struct LpAonclkrstHpUsbClkrstCtrl0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstHpUsbClkrstCtrl0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstHpUsbClkrstCtrl0 {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_USB_12M_DIV_NUM**
    ///
    /// usb 480m to 12m divide number.
    ///
    /// Bits `[29:22]`, mask `0x3fc00000`
    pub const LP_AONCLKRST_USB_12M_DIV_NUM_OFFSET: u32 = 22;
    pub const LP_AONCLKRST_USB_12M_DIV_NUM_MASK: u32 = 0x3fc00000;

    /// **LP_AONCLKRST_USB_25M_DIV_NUM**
    ///
    /// usb 500m to 25m divide number.
    ///
    /// Bits `[21:14]`, mask `0x003fc000`
    pub const LP_AONCLKRST_USB_25M_DIV_NUM_OFFSET: u32 = 14;
    pub const LP_AONCLKRST_USB_25M_DIV_NUM_MASK: u32 = 0x003fc000;

    /// **LP_AONCLKRST_USB_48M_DIV_NUM**
    ///
    /// usb 480m to 25m divide number.
    ///
    /// Bits `[13:6]`, mask `0x00003fc0`
    pub const LP_AONCLKRST_USB_48M_DIV_NUM_OFFSET: u32 = 6;
    pub const LP_AONCLKRST_USB_48M_DIV_NUM_MASK: u32 = 0x00003fc0;

    /// **LP_AONCLKRST_USB_DEVICE_48M_CLK_EN**
    ///
    /// usb device fs phy clock enable.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const LP_AONCLKRST_USB_DEVICE_48M_CLK_EN_OFFSET: u32 = 5;
    pub const LP_AONCLKRST_USB_DEVICE_48M_CLK_EN_MASK: u32 = 0x00000020;
    pub const LP_AONCLKRST_USB_DEVICE_48M_CLK_EN: u32 = 1 << 5;

    /// **LP_AONCLKRST_USB_OTG11_48M_CLK_EN**
    ///
    /// usb otg11 fs phy clock enable.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_AONCLKRST_USB_OTG11_48M_CLK_EN_OFFSET: u32 = 4;
    pub const LP_AONCLKRST_USB_OTG11_48M_CLK_EN_MASK: u32 = 0x00000010;
    pub const LP_AONCLKRST_USB_OTG11_48M_CLK_EN: u32 = 1 << 4;

    /// **LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN**
    ///
    /// unused.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN_OFFSET: u32 = 3;
    pub const LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN_MASK: u32 = 0x00000008;
    pub const LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN: u32 = 1 << 3;

    /// **LP_AONCLKRST_USB_OTG11_SLEEP_MODE**
    ///
    /// unused.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const LP_AONCLKRST_USB_OTG11_SLEEP_MODE_OFFSET: u32 = 2;
    pub const LP_AONCLKRST_USB_OTG11_SLEEP_MODE_MASK: u32 = 0x00000004;
    pub const LP_AONCLKRST_USB_OTG11_SLEEP_MODE: u32 = 1 << 2;

    /// **LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN**
    ///
    /// unused.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN_OFFSET: u32 = 1;
    pub const LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN_MASK: u32 = 0x00000002;
    pub const LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN: u32 = 1 << 1;

    /// **LP_AONCLKRST_USB_OTG20_SLEEP_MODE**
    ///
    /// unused.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_AONCLKRST_USB_OTG20_SLEEP_MODE_OFFSET: u32 = 0;
    pub const LP_AONCLKRST_USB_OTG20_SLEEP_MODE_MASK: u32 = 0x00000001;
    pub const LP_AONCLKRST_USB_OTG20_SLEEP_MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_USB_12M_DIV_NUM** `[29:22]` (RW)
    ///
    /// usb 480m to 12m divide number.
    #[inline(always)]
    pub fn lp_aonclkrst_usb_12m_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_USB_12M_DIV_NUM_MASK)
            >> Self::LP_AONCLKRST_USB_12M_DIV_NUM_OFFSET
    }

    /// Write **LP_AONCLKRST_USB_12M_DIV_NUM** `[29:22]` (RW)
    ///
    /// usb 480m to 12m divide number.
    #[inline(always)]
    pub fn write_lp_aonclkrst_usb_12m_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_USB_12M_DIV_NUM_MASK)
                | ((value << Self::LP_AONCLKRST_USB_12M_DIV_NUM_OFFSET)
                    & Self::LP_AONCLKRST_USB_12M_DIV_NUM_MASK)
        });
    }

    /// **LP_AONCLKRST_USB_25M_DIV_NUM** `[21:14]` (RW)
    ///
    /// usb 500m to 25m divide number.
    #[inline(always)]
    pub fn lp_aonclkrst_usb_25m_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_USB_25M_DIV_NUM_MASK)
            >> Self::LP_AONCLKRST_USB_25M_DIV_NUM_OFFSET
    }

    /// Write **LP_AONCLKRST_USB_25M_DIV_NUM** `[21:14]` (RW)
    ///
    /// usb 500m to 25m divide number.
    #[inline(always)]
    pub fn write_lp_aonclkrst_usb_25m_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_USB_25M_DIV_NUM_MASK)
                | ((value << Self::LP_AONCLKRST_USB_25M_DIV_NUM_OFFSET)
                    & Self::LP_AONCLKRST_USB_25M_DIV_NUM_MASK)
        });
    }

    /// **LP_AONCLKRST_USB_48M_DIV_NUM** `[13:6]` (RW)
    ///
    /// usb 480m to 25m divide number.
    #[inline(always)]
    pub fn lp_aonclkrst_usb_48m_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_USB_48M_DIV_NUM_MASK)
            >> Self::LP_AONCLKRST_USB_48M_DIV_NUM_OFFSET
    }

    /// Write **LP_AONCLKRST_USB_48M_DIV_NUM** `[13:6]` (RW)
    ///
    /// usb 480m to 25m divide number.
    #[inline(always)]
    pub fn write_lp_aonclkrst_usb_48m_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_USB_48M_DIV_NUM_MASK)
                | ((value << Self::LP_AONCLKRST_USB_48M_DIV_NUM_OFFSET)
                    & Self::LP_AONCLKRST_USB_48M_DIV_NUM_MASK)
        });
    }

    /// **LP_AONCLKRST_USB_DEVICE_48M_CLK_EN** `[5]` (RW)
    ///
    /// usb device fs phy clock enable.
    #[inline(always)]
    pub fn lp_aonclkrst_usb_device_48m_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_USB_DEVICE_48M_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_USB_DEVICE_48M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_usb_device_48m_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_USB_DEVICE_48M_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_USB_DEVICE_48M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_usb_device_48m_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_USB_DEVICE_48M_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_USB_OTG11_48M_CLK_EN** `[4]` (RW)
    ///
    /// usb otg11 fs phy clock enable.
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg11_48m_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_USB_OTG11_48M_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_USB_OTG11_48M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_usb_otg11_48m_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_USB_OTG11_48M_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_USB_OTG11_48M_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_usb_otg11_48m_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_USB_OTG11_48M_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN** `[3]` (RW)
    ///
    /// unused.
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg11_bk_sys_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_usb_otg11_bk_sys_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_usb_otg11_bk_sys_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_USB_OTG11_SLEEP_MODE** `[2]` (RW)
    ///
    /// unused.
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg11_sleep_mode(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_USB_OTG11_SLEEP_MODE_MASK) != 0
    }

    /// Set LP_AONCLKRST_USB_OTG11_SLEEP_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_usb_otg11_sleep_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_USB_OTG11_SLEEP_MODE_MASK);
    }

    /// Clear LP_AONCLKRST_USB_OTG11_SLEEP_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_usb_otg11_sleep_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_USB_OTG11_SLEEP_MODE_MASK);
    }

    /// **LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN** `[1]` (RW)
    ///
    /// unused.
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg20_bk_sys_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_usb_otg20_bk_sys_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_usb_otg20_bk_sys_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_USB_OTG20_SLEEP_MODE** `[0]` (RW)
    ///
    /// unused.
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg20_sleep_mode(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_USB_OTG20_SLEEP_MODE_MASK) != 0
    }

    /// Set LP_AONCLKRST_USB_OTG20_SLEEP_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_usb_otg20_sleep_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_USB_OTG20_SLEEP_MODE_MASK);
    }

    /// Clear LP_AONCLKRST_USB_OTG20_SLEEP_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_usb_otg20_sleep_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_USB_OTG20_SLEEP_MODE_MASK);
    }
}

/// LP_AONCLKRST_HP_USB_CLKRST_CTRL0 value type for batch read/modify/write operations
///
/// Use `lpaonclkrsthpusbclkrstctrl0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrsthpusbclkrstctrl0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstHpUsbClkrstCtrl0Val {
    bits: u32,
}

impl LpAonclkrstHpUsbClkrstCtrl0Val {
    /// Create from raw bits
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
        Self { bits: 0x09c4c27a }
    }

    /// Read **LP_AONCLKRST_USB_12M_DIV_NUM** `[29:22]`
    ///
    /// usb 480m to 12m divide number.
    #[inline(always)]
    pub const fn lp_aonclkrst_usb_12m_div_num(&self) -> u32 {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_12M_DIV_NUM_MASK)
            >> LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_12M_DIV_NUM_OFFSET
    }

    /// Read **LP_AONCLKRST_USB_25M_DIV_NUM** `[21:14]`
    ///
    /// usb 500m to 25m divide number.
    #[inline(always)]
    pub const fn lp_aonclkrst_usb_25m_div_num(&self) -> u32 {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_25M_DIV_NUM_MASK)
            >> LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_25M_DIV_NUM_OFFSET
    }

    /// Read **LP_AONCLKRST_USB_48M_DIV_NUM** `[13:6]`
    ///
    /// usb 480m to 25m divide number.
    #[inline(always)]
    pub const fn lp_aonclkrst_usb_48m_div_num(&self) -> u32 {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_48M_DIV_NUM_MASK)
            >> LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_48M_DIV_NUM_OFFSET
    }

    /// Read **LP_AONCLKRST_USB_DEVICE_48M_CLK_EN** `[5:5]`
    ///
    /// usb device fs phy clock enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_usb_device_48m_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_DEVICE_48M_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_USB_OTG11_48M_CLK_EN** `[4:4]`
    ///
    /// usb otg11 fs phy clock enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_usb_otg11_48m_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG11_48M_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN** `[3:3]`
    ///
    /// unused.
    #[inline(always)]
    pub const fn lp_aonclkrst_usb_otg11_bk_sys_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_USB_OTG11_SLEEP_MODE** `[2:2]`
    ///
    /// unused.
    #[inline(always)]
    pub const fn lp_aonclkrst_usb_otg11_sleep_mode(&self) -> bool {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG11_SLEEP_MODE_MASK) != 0
    }

    /// Read **LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN** `[1:1]`
    ///
    /// unused.
    #[inline(always)]
    pub const fn lp_aonclkrst_usb_otg20_bk_sys_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_USB_OTG20_SLEEP_MODE** `[0:0]`
    ///
    /// unused.
    #[inline(always)]
    pub const fn lp_aonclkrst_usb_otg20_sleep_mode(&self) -> bool {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG20_SLEEP_MODE_MASK) != 0
    }

    /// Set **LP_AONCLKRST_USB_12M_DIV_NUM** `[29:22]`
    ///
    /// usb 480m to 12m divide number.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_usb_12m_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_12M_DIV_NUM_MASK)
                | ((value << LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_12M_DIV_NUM_OFFSET)
                    & LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_12M_DIV_NUM_MASK),
        }
    }

    /// Set **LP_AONCLKRST_USB_25M_DIV_NUM** `[21:14]`
    ///
    /// usb 500m to 25m divide number.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_usb_25m_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_25M_DIV_NUM_MASK)
                | ((value << LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_25M_DIV_NUM_OFFSET)
                    & LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_25M_DIV_NUM_MASK),
        }
    }

    /// Set **LP_AONCLKRST_USB_48M_DIV_NUM** `[13:6]`
    ///
    /// usb 480m to 25m divide number.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_usb_48m_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_48M_DIV_NUM_MASK)
                | ((value << LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_48M_DIV_NUM_OFFSET)
                    & LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_48M_DIV_NUM_MASK),
        }
    }

    /// Set **LP_AONCLKRST_USB_DEVICE_48M_CLK_EN** `[5:5]`
    ///
    /// usb device fs phy clock enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_usb_device_48m_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_DEVICE_48M_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_DEVICE_48M_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_USB_OTG11_48M_CLK_EN** `[4:4]`
    ///
    /// usb otg11 fs phy clock enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_usb_otg11_48m_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG11_48M_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG11_48M_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN** `[3:3]`
    ///
    /// unused.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_usb_otg11_bk_sys_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_USB_OTG11_SLEEP_MODE** `[2:2]`
    ///
    /// unused.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_usb_otg11_sleep_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG11_SLEEP_MODE_MASK
            } else {
                self.bits & !LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG11_SLEEP_MODE_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN** `[1:1]`
    ///
    /// unused.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_usb_otg20_bk_sys_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_USB_OTG20_SLEEP_MODE** `[0:0]`
    ///
    /// unused.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_usb_otg20_sleep_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG20_SLEEP_MODE_MASK
            } else {
                self.bits & !LpAonclkrstHpUsbClkrstCtrl0::LP_AONCLKRST_USB_OTG20_SLEEP_MODE_MASK
            },
        }
    }
}

impl Default for LpAonclkrstHpUsbClkrstCtrl0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstHpUsbClkrstCtrl0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstHpUsbClkrstCtrl0Val {
        LpAonclkrstHpUsbClkrstCtrl0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstHpUsbClkrstCtrl0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<
        F: FnOnce(LpAonclkrstHpUsbClkrstCtrl0Val) -> LpAonclkrstHpUsbClkrstCtrl0Val,
    >(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_HP_USB_CLKRST_CTRL1 Register
///
/// HP USB Clock Reset Control Register.
///
/// Address offset: 0x0048
/// Reset value: 0xc0000000
///
/// ## Fields
///
/// - **LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN** `[31]` (RW)
///
///   usb otg20 ulpi clock enable.
///
/// - **LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN** `[30]` (RW)
///
///   usb otg20 hs phy refclk enable.
///
/// - **LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL** `[29:28]` (RW)
///
///   usb otg20 hs phy src sel. 2'd0: 12m, 2'd1: 25m, 2'd2: pad_hsphy_refclk.
///
/// - **LP_AONCLKRST_RST_EN_USB_DEVICE** `[4]` (RW)
///
///   usb device reset en
///
/// - **LP_AONCLKRST_RST_EN_USB_OTG11** `[3]` (RW)
///
///   usb org11 reset en
///
/// - **LP_AONCLKRST_RST_EN_USB_OTG20** `[2]` (RW)
///
///   usb otg20 reset en
///
/// - **LP_AONCLKRST_RST_EN_USB_OTG20_PHY** `[1]` (RW)
///
///   usb otg20 phy reset en
///
/// - **LP_AONCLKRST_RST_EN_USB_OTG20_ADP** `[0]` (RW)
///
///   usb otg20 adp reset en
#[repr(transparent)]
pub struct LpAonclkrstHpUsbClkrstCtrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstHpUsbClkrstCtrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstHpUsbClkrstCtrl1 {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN**
    ///
    /// usb otg20 ulpi clock enable.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN_OFFSET: u32 = 31;
    pub const LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN_MASK: u32 = 0x80000000;
    pub const LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN: u32 = 1 << 31;

    /// **LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN**
    ///
    /// usb otg20 hs phy refclk enable.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN_OFFSET: u32 = 30;
    pub const LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN_MASK: u32 = 0x40000000;
    pub const LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN: u32 = 1 << 30;

    /// **LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL**
    ///
    /// usb otg20 hs phy src sel. 2'd0: 12m, 2'd1: 25m, 2'd2: pad_hsphy_refclk.
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_OFFSET: u32 = 28;
    pub const LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_MASK: u32 = 0x30000000;

    /// **LP_AONCLKRST_RST_EN_USB_DEVICE**
    ///
    /// usb device reset en
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_AONCLKRST_RST_EN_USB_DEVICE_OFFSET: u32 = 4;
    pub const LP_AONCLKRST_RST_EN_USB_DEVICE_MASK: u32 = 0x00000010;
    pub const LP_AONCLKRST_RST_EN_USB_DEVICE: u32 = 1 << 4;

    /// **LP_AONCLKRST_RST_EN_USB_OTG11**
    ///
    /// usb org11 reset en
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LP_AONCLKRST_RST_EN_USB_OTG11_OFFSET: u32 = 3;
    pub const LP_AONCLKRST_RST_EN_USB_OTG11_MASK: u32 = 0x00000008;
    pub const LP_AONCLKRST_RST_EN_USB_OTG11: u32 = 1 << 3;

    /// **LP_AONCLKRST_RST_EN_USB_OTG20**
    ///
    /// usb otg20 reset en
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const LP_AONCLKRST_RST_EN_USB_OTG20_OFFSET: u32 = 2;
    pub const LP_AONCLKRST_RST_EN_USB_OTG20_MASK: u32 = 0x00000004;
    pub const LP_AONCLKRST_RST_EN_USB_OTG20: u32 = 1 << 2;

    /// **LP_AONCLKRST_RST_EN_USB_OTG20_PHY**
    ///
    /// usb otg20 phy reset en
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const LP_AONCLKRST_RST_EN_USB_OTG20_PHY_OFFSET: u32 = 1;
    pub const LP_AONCLKRST_RST_EN_USB_OTG20_PHY_MASK: u32 = 0x00000002;
    pub const LP_AONCLKRST_RST_EN_USB_OTG20_PHY: u32 = 1 << 1;

    /// **LP_AONCLKRST_RST_EN_USB_OTG20_ADP**
    ///
    /// usb otg20 adp reset en
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_AONCLKRST_RST_EN_USB_OTG20_ADP_OFFSET: u32 = 0;
    pub const LP_AONCLKRST_RST_EN_USB_OTG20_ADP_MASK: u32 = 0x00000001;
    pub const LP_AONCLKRST_RST_EN_USB_OTG20_ADP: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN** `[31]` (RW)
    ///
    /// usb otg20 ulpi clock enable.
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg20_ulpi_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_usb_otg20_ulpi_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_usb_otg20_ulpi_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN** `[30]` (RW)
    ///
    /// usb otg20 hs phy refclk enable.
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg20_phyref_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_usb_otg20_phyref_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_usb_otg20_phyref_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN_MASK);
    }

    /// **LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL** `[29:28]` (RW)
    ///
    /// usb otg20 hs phy src sel. 2'd0: 12m, 2'd1: 25m, 2'd2: pad_hsphy_refclk.
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg20_phyref_clk_src_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_MASK)
            >> Self::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_OFFSET
    }

    /// Write **LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL** `[29:28]` (RW)
    ///
    /// usb otg20 hs phy src sel. 2'd0: 12m, 2'd1: 25m, 2'd2: pad_hsphy_refclk.
    #[inline(always)]
    pub fn write_lp_aonclkrst_usb_otg20_phyref_clk_src_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_MASK)
                | ((value << Self::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_OFFSET)
                    & Self::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_MASK)
        });
    }

    /// **LP_AONCLKRST_RST_EN_USB_DEVICE** `[4]` (RW)
    ///
    /// usb device reset en
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_usb_device(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_USB_DEVICE_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_USB_DEVICE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_usb_device(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_USB_DEVICE_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_USB_DEVICE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_usb_device(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_USB_DEVICE_MASK);
    }

    /// **LP_AONCLKRST_RST_EN_USB_OTG11** `[3]` (RW)
    ///
    /// usb org11 reset en
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_usb_otg11(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_USB_OTG11_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_USB_OTG11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_usb_otg11(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_USB_OTG11_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_USB_OTG11 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_usb_otg11(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_USB_OTG11_MASK);
    }

    /// **LP_AONCLKRST_RST_EN_USB_OTG20** `[2]` (RW)
    ///
    /// usb otg20 reset en
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_usb_otg20(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_USB_OTG20_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_USB_OTG20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_usb_otg20(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_USB_OTG20_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_USB_OTG20 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_usb_otg20(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_USB_OTG20_MASK);
    }

    /// **LP_AONCLKRST_RST_EN_USB_OTG20_PHY** `[1]` (RW)
    ///
    /// usb otg20 phy reset en
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_usb_otg20_phy(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_USB_OTG20_PHY_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_USB_OTG20_PHY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_usb_otg20_phy(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_USB_OTG20_PHY_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_USB_OTG20_PHY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_usb_otg20_phy(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_USB_OTG20_PHY_MASK);
    }

    /// **LP_AONCLKRST_RST_EN_USB_OTG20_ADP** `[0]` (RW)
    ///
    /// usb otg20 adp reset en
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_usb_otg20_adp(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_USB_OTG20_ADP_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_USB_OTG20_ADP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_usb_otg20_adp(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_USB_OTG20_ADP_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_USB_OTG20_ADP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_usb_otg20_adp(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_USB_OTG20_ADP_MASK);
    }
}

/// LP_AONCLKRST_HP_USB_CLKRST_CTRL1 value type for batch read/modify/write operations
///
/// Use `lpaonclkrsthpusbclkrstctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrsthpusbclkrstctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstHpUsbClkrstCtrl1Val {
    bits: u32,
}

impl LpAonclkrstHpUsbClkrstCtrl1Val {
    /// Create from raw bits
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
        Self { bits: 0xc0000000 }
    }

    /// Read **LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN** `[31:31]`
    ///
    /// usb otg20 ulpi clock enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_usb_otg20_ulpi_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN** `[30:30]`
    ///
    /// usb otg20 hs phy refclk enable.
    #[inline(always)]
    pub const fn lp_aonclkrst_usb_otg20_phyref_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN_MASK) != 0
    }

    /// Read **LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL** `[29:28]`
    ///
    /// usb otg20 hs phy src sel. 2'd0: 12m, 2'd1: 25m, 2'd2: pad_hsphy_refclk.
    #[inline(always)]
    pub const fn lp_aonclkrst_usb_otg20_phyref_clk_src_sel(&self) -> u32 {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_MASK)
            >> LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_OFFSET
    }

    /// Read **LP_AONCLKRST_RST_EN_USB_DEVICE** `[4:4]`
    ///
    /// usb device reset en
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_usb_device(&self) -> bool {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_DEVICE_MASK) != 0
    }

    /// Read **LP_AONCLKRST_RST_EN_USB_OTG11** `[3:3]`
    ///
    /// usb org11 reset en
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_usb_otg11(&self) -> bool {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_OTG11_MASK) != 0
    }

    /// Read **LP_AONCLKRST_RST_EN_USB_OTG20** `[2:2]`
    ///
    /// usb otg20 reset en
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_usb_otg20(&self) -> bool {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_OTG20_MASK) != 0
    }

    /// Read **LP_AONCLKRST_RST_EN_USB_OTG20_PHY** `[1:1]`
    ///
    /// usb otg20 phy reset en
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_usb_otg20_phy(&self) -> bool {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_OTG20_PHY_MASK) != 0
    }

    /// Read **LP_AONCLKRST_RST_EN_USB_OTG20_ADP** `[0:0]`
    ///
    /// usb otg20 adp reset en
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_usb_otg20_adp(&self) -> bool {
        (self.bits & LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_OTG20_ADP_MASK) != 0
    }

    /// Set **LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN** `[31:31]`
    ///
    /// usb otg20 ulpi clock enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_usb_otg20_ulpi_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_USB_OTG20_ULPI_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN** `[30:30]`
    ///
    /// usb otg20 hs phy refclk enable.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_usb_otg20_phyref_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL** `[29:28]`
    ///
    /// usb otg20 hs phy src sel. 2'd0: 12m, 2'd1: 25m, 2'd2: pad_hsphy_refclk.
    #[inline(always)]
    pub const fn with_lp_aonclkrst_usb_otg20_phyref_clk_src_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_MASK) | ((value << LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_OFFSET) & LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_USB_OTG20_PHYREF_CLK_SRC_SEL_MASK)
        }
    }

    /// Set **LP_AONCLKRST_RST_EN_USB_DEVICE** `[4:4]`
    ///
    /// usb device reset en
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_usb_device(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_DEVICE_MASK
            } else {
                self.bits & !LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_DEVICE_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_RST_EN_USB_OTG11** `[3:3]`
    ///
    /// usb org11 reset en
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_usb_otg11(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_OTG11_MASK
            } else {
                self.bits & !LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_OTG11_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_RST_EN_USB_OTG20** `[2:2]`
    ///
    /// usb otg20 reset en
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_usb_otg20(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_OTG20_MASK
            } else {
                self.bits & !LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_OTG20_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_RST_EN_USB_OTG20_PHY** `[1:1]`
    ///
    /// usb otg20 phy reset en
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_usb_otg20_phy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_OTG20_PHY_MASK
            } else {
                self.bits & !LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_OTG20_PHY_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_RST_EN_USB_OTG20_ADP** `[0:0]`
    ///
    /// usb otg20 adp reset en
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_usb_otg20_adp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_OTG20_ADP_MASK
            } else {
                self.bits & !LpAonclkrstHpUsbClkrstCtrl1::LP_AONCLKRST_RST_EN_USB_OTG20_ADP_MASK
            },
        }
    }
}

impl Default for LpAonclkrstHpUsbClkrstCtrl1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAonclkrstHpUsbClkrstCtrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstHpUsbClkrstCtrl1Val {
        LpAonclkrstHpUsbClkrstCtrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstHpUsbClkrstCtrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<
        F: FnOnce(LpAonclkrstHpUsbClkrstCtrl1Val) -> LpAonclkrstHpUsbClkrstCtrl1Val,
    >(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL Register
///
/// need_des
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **LP_AONCLKRST_FORCE_NORST_EMAC** `[31]` (RW)
///
///   hp emac force norst
///
/// - **LP_AONCLKRST_RST_EN_EMAC** `[30]` (RW)
///
///   hp emac reset en
///
/// - **LP_AONCLKRST_FORCE_NORST_SDMMC** `[29]` (RW)
///
///   hp sdmmc force norst
///
/// - **LP_AONCLKRST_RST_EN_SDMMC** `[28]` (RW)
///
///   hp sdmmc reset en
#[repr(transparent)]
pub struct LpAonclkrstHpSdmmcEmacRstCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstHpSdmmcEmacRstCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstHpSdmmcEmacRstCtrl {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_FORCE_NORST_EMAC**
    ///
    /// hp emac force norst
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_AONCLKRST_FORCE_NORST_EMAC_OFFSET: u32 = 31;
    pub const LP_AONCLKRST_FORCE_NORST_EMAC_MASK: u32 = 0x80000000;
    pub const LP_AONCLKRST_FORCE_NORST_EMAC: u32 = 1 << 31;

    /// **LP_AONCLKRST_RST_EN_EMAC**
    ///
    /// hp emac reset en
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_AONCLKRST_RST_EN_EMAC_OFFSET: u32 = 30;
    pub const LP_AONCLKRST_RST_EN_EMAC_MASK: u32 = 0x40000000;
    pub const LP_AONCLKRST_RST_EN_EMAC: u32 = 1 << 30;

    /// **LP_AONCLKRST_FORCE_NORST_SDMMC**
    ///
    /// hp sdmmc force norst
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_AONCLKRST_FORCE_NORST_SDMMC_OFFSET: u32 = 29;
    pub const LP_AONCLKRST_FORCE_NORST_SDMMC_MASK: u32 = 0x20000000;
    pub const LP_AONCLKRST_FORCE_NORST_SDMMC: u32 = 1 << 29;

    /// **LP_AONCLKRST_RST_EN_SDMMC**
    ///
    /// hp sdmmc reset en
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_AONCLKRST_RST_EN_SDMMC_OFFSET: u32 = 28;
    pub const LP_AONCLKRST_RST_EN_SDMMC_MASK: u32 = 0x10000000;
    pub const LP_AONCLKRST_RST_EN_SDMMC: u32 = 1 << 28;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_FORCE_NORST_EMAC** `[31]` (RW)
    ///
    /// hp emac force norst
    #[inline(always)]
    pub fn lp_aonclkrst_force_norst_emac(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_FORCE_NORST_EMAC_MASK) != 0
    }

    /// Set LP_AONCLKRST_FORCE_NORST_EMAC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_force_norst_emac(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_FORCE_NORST_EMAC_MASK);
    }

    /// Clear LP_AONCLKRST_FORCE_NORST_EMAC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_force_norst_emac(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_FORCE_NORST_EMAC_MASK);
    }

    /// **LP_AONCLKRST_RST_EN_EMAC** `[30]` (RW)
    ///
    /// hp emac reset en
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_emac(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_EMAC_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_EMAC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_emac(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_EMAC_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_EMAC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_emac(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_EMAC_MASK);
    }

    /// **LP_AONCLKRST_FORCE_NORST_SDMMC** `[29]` (RW)
    ///
    /// hp sdmmc force norst
    #[inline(always)]
    pub fn lp_aonclkrst_force_norst_sdmmc(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_FORCE_NORST_SDMMC_MASK) != 0
    }

    /// Set LP_AONCLKRST_FORCE_NORST_SDMMC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_force_norst_sdmmc(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_FORCE_NORST_SDMMC_MASK);
    }

    /// Clear LP_AONCLKRST_FORCE_NORST_SDMMC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_force_norst_sdmmc(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_FORCE_NORST_SDMMC_MASK);
    }

    /// **LP_AONCLKRST_RST_EN_SDMMC** `[28]` (RW)
    ///
    /// hp sdmmc reset en
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_sdmmc(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_RST_EN_SDMMC_MASK) != 0
    }

    /// Set LP_AONCLKRST_RST_EN_SDMMC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_rst_en_sdmmc(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_RST_EN_SDMMC_MASK);
    }

    /// Clear LP_AONCLKRST_RST_EN_SDMMC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_rst_en_sdmmc(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_RST_EN_SDMMC_MASK);
    }
}

/// LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL value type for batch read/modify/write operations
///
/// Use `lpaonclkrsthpsdmmcemacrstctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrsthpsdmmcemacrstctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstHpSdmmcEmacRstCtrlVal {
    bits: u32,
}

impl LpAonclkrstHpSdmmcEmacRstCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_AONCLKRST_FORCE_NORST_EMAC** `[31:31]`
    ///
    /// hp emac force norst
    #[inline(always)]
    pub const fn lp_aonclkrst_force_norst_emac(&self) -> bool {
        (self.bits & LpAonclkrstHpSdmmcEmacRstCtrl::LP_AONCLKRST_FORCE_NORST_EMAC_MASK) != 0
    }

    /// Read **LP_AONCLKRST_RST_EN_EMAC** `[30:30]`
    ///
    /// hp emac reset en
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_emac(&self) -> bool {
        (self.bits & LpAonclkrstHpSdmmcEmacRstCtrl::LP_AONCLKRST_RST_EN_EMAC_MASK) != 0
    }

    /// Read **LP_AONCLKRST_FORCE_NORST_SDMMC** `[29:29]`
    ///
    /// hp sdmmc force norst
    #[inline(always)]
    pub const fn lp_aonclkrst_force_norst_sdmmc(&self) -> bool {
        (self.bits & LpAonclkrstHpSdmmcEmacRstCtrl::LP_AONCLKRST_FORCE_NORST_SDMMC_MASK) != 0
    }

    /// Read **LP_AONCLKRST_RST_EN_SDMMC** `[28:28]`
    ///
    /// hp sdmmc reset en
    #[inline(always)]
    pub const fn lp_aonclkrst_rst_en_sdmmc(&self) -> bool {
        (self.bits & LpAonclkrstHpSdmmcEmacRstCtrl::LP_AONCLKRST_RST_EN_SDMMC_MASK) != 0
    }

    /// Set **LP_AONCLKRST_FORCE_NORST_EMAC** `[31:31]`
    ///
    /// hp emac force norst
    #[inline(always)]
    pub const fn with_lp_aonclkrst_force_norst_emac(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpSdmmcEmacRstCtrl::LP_AONCLKRST_FORCE_NORST_EMAC_MASK
            } else {
                self.bits & !LpAonclkrstHpSdmmcEmacRstCtrl::LP_AONCLKRST_FORCE_NORST_EMAC_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_RST_EN_EMAC** `[30:30]`
    ///
    /// hp emac reset en
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_emac(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpSdmmcEmacRstCtrl::LP_AONCLKRST_RST_EN_EMAC_MASK
            } else {
                self.bits & !LpAonclkrstHpSdmmcEmacRstCtrl::LP_AONCLKRST_RST_EN_EMAC_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_FORCE_NORST_SDMMC** `[29:29]`
    ///
    /// hp sdmmc force norst
    #[inline(always)]
    pub const fn with_lp_aonclkrst_force_norst_sdmmc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpSdmmcEmacRstCtrl::LP_AONCLKRST_FORCE_NORST_SDMMC_MASK
            } else {
                self.bits & !LpAonclkrstHpSdmmcEmacRstCtrl::LP_AONCLKRST_FORCE_NORST_SDMMC_MASK
            },
        }
    }

    /// Set **LP_AONCLKRST_RST_EN_SDMMC** `[28:28]`
    ///
    /// hp sdmmc reset en
    #[inline(always)]
    pub const fn with_lp_aonclkrst_rst_en_sdmmc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstHpSdmmcEmacRstCtrl::LP_AONCLKRST_RST_EN_SDMMC_MASK
            } else {
                self.bits & !LpAonclkrstHpSdmmcEmacRstCtrl::LP_AONCLKRST_RST_EN_SDMMC_MASK
            },
        }
    }
}

impl Default for LpAonclkrstHpSdmmcEmacRstCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAonclkrstHpSdmmcEmacRstCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstHpSdmmcEmacRstCtrlVal {
        LpAonclkrstHpSdmmcEmacRstCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstHpSdmmcEmacRstCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<
        F: FnOnce(LpAonclkrstHpSdmmcEmacRstCtrlVal) -> LpAonclkrstHpSdmmcEmacRstCtrlVal,
    >(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_AONCLKRST_DATE Register
///
/// need_des
///
/// Address offset: 0x03fc
///
/// ## Fields
///
/// - **LP_AONCLKRST_CLK_EN** `[31]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAonclkrstDate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAonclkrstDate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAonclkrstDate {
    // ======== Field Constants ========

    /// **LP_AONCLKRST_CLK_EN**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_AONCLKRST_CLK_EN_OFFSET: u32 = 31;
    pub const LP_AONCLKRST_CLK_EN_MASK: u32 = 0x80000000;
    pub const LP_AONCLKRST_CLK_EN: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **LP_AONCLKRST_CLK_EN** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_aonclkrst_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_AONCLKRST_CLK_EN_MASK) != 0
    }

    /// Set LP_AONCLKRST_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_aonclkrst_clk_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_AONCLKRST_CLK_EN_MASK);
    }

    /// Clear LP_AONCLKRST_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_aonclkrst_clk_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_AONCLKRST_CLK_EN_MASK);
    }
}

/// LP_AONCLKRST_DATE value type for batch read/modify/write operations
///
/// Use `lpaonclkrstdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaonclkrstdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAonclkrstDateVal {
    bits: u32,
}

impl LpAonclkrstDateVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_AONCLKRST_CLK_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_aonclkrst_clk_en(&self) -> bool {
        (self.bits & LpAonclkrstDate::LP_AONCLKRST_CLK_EN_MASK) != 0
    }

    /// Set **LP_AONCLKRST_CLK_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_aonclkrst_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAonclkrstDate::LP_AONCLKRST_CLK_EN_MASK
            } else {
                self.bits & !LpAonclkrstDate::LP_AONCLKRST_CLK_EN_MASK
            },
        }
    }
}

impl Default for LpAonclkrstDateVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAonclkrstDate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAonclkrstDateVal {
        LpAonclkrstDateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAonclkrstDateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAonclkrstDateVal) -> LpAonclkrstDateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// need_des
    pub lp_aonclkrst_lp_clk_conf: LpAonclkrstLpClkConf,
    /// 0x0004
    /// need_des
    pub lp_aonclkrst_lp_clk_po_en: LpAonclkrstLpClkPoEn,
    /// 0x0008
    /// need_des
    pub lp_aonclkrst_lp_clk_en: LpAonclkrstLpClkEn,
    /// 0x000c
    /// need_des
    pub lp_aonclkrst_lp_rst_en: LpAonclkrstLpRstEn,
    /// 0x0010
    /// need_des
    pub lp_aonclkrst_reset_cause: LpAonclkrstResetCause,
    /// 0x0014
    /// need_des
    pub lp_aonclkrst_hpcpu_reset_ctrl0: LpAonclkrstHpcpuResetCtrl0,
    /// 0x0018
    /// need_des
    pub lp_aonclkrst_hpcpu_reset_ctrl1: LpAonclkrstHpcpuResetCtrl1,
    /// 0x001c
    /// need_des
    pub lp_aonclkrst_fosc_cntl: LpAonclkrstFoscCntl,
    /// 0x0020
    /// need_des
    pub lp_aonclkrst_rc32k_cntl: LpAonclkrstRc32kCntl,
    /// 0x0024
    /// need_des
    pub lp_aonclkrst_sosc_cntl: LpAonclkrstSoscCntl,
    /// 0x0028
    /// need_des
    pub lp_aonclkrst_clk_to_hp: LpAonclkrstClkToHp,
    /// 0x002c
    /// need_des
    pub lp_aonclkrst_lpmem_force: LpAonclkrstLpmemForce,
    /// 0x0030
    /// need_des
    pub lp_aonclkrst_xtal32k: LpAonclkrstXtal32k,
    /// 0x0034
    /// need_des
    pub lp_aonclkrst_mux_hpsys_reset_bypass: LpAonclkrstMuxHpsysResetBypass,
    /// 0x0038
    /// need_des
    pub lp_aonclkrst_hpsys0_reset_bypass: LpAonclkrstHpsys0ResetBypass,
    /// 0x003c
    /// need_des
    pub lp_aonclkrst_hpsys_apm_reset_bypass: LpAonclkrstHpsysApmResetBypass,
    /// 0x0040
    /// HP Clock Control Register.
    pub lp_aonclkrst_hp_clk_ctrl: LpAonclkrstHpClkCtrl,
    /// 0x0044
    /// HP USB Clock Reset Control Register.
    pub lp_aonclkrst_hp_usb_clkrst_ctrl0: LpAonclkrstHpUsbClkrstCtrl0,
    /// 0x0048
    /// HP USB Clock Reset Control Register.
    pub lp_aonclkrst_hp_usb_clkrst_ctrl1: LpAonclkrstHpUsbClkrstCtrl1,
    /// 0x004c
    /// need_des
    pub lp_aonclkrst_hp_sdmmc_emac_rst_ctrl: LpAonclkrstHpSdmmcEmacRstCtrl,
    _reserved0: [u8; 940],
    /// 0x03fc
    /// need_des
    pub lp_aonclkrst_date: LpAonclkrstDate,
}
