//! LP_ANA_PERI
//! LP_ANA_PERI Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x50113000;

/// LP_ANA_BOD_MODE0_CNTL Register
///
/// need_des
///
/// Address offset: 0x0000
/// Reset value: 0x0ffc0100
///
/// ## Fields
///
/// - **LP_ANA_BOD_MODE0_RESET_ENA** `[31]` (RW)
///
///   need_des
///
/// - **LP_ANA_BOD_MODE0_RESET_SEL** `[30]` (RW)
///
///   need_des
///
/// - **LP_ANA_BOD_MODE0_INTR_ENA** `[29]` (RW)
///
///   need_des
///
/// - **LP_ANA_BOD_MODE0_CNT_CLR** `[28]` (RW)
///
///   need_des
///
/// - **LP_ANA_BOD_MODE0_RESET_WAIT** `[27:18]` (RW)
///
///   need_des
///
/// - **LP_ANA_BOD_MODE0_INTR_WAIT** `[17:8]` (RW)
///
///   need_des
///
/// - **LP_ANA_BOD_MODE0_PD_RF_ENA** `[7]` (RW)
///
///   need_des
///
/// - **LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA** `[6]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaBodMode0Cntl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaBodMode0Cntl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaBodMode0Cntl {
    // ======== Field Constants ========

    /// **LP_ANA_BOD_MODE0_RESET_ENA**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_BOD_MODE0_RESET_ENA_OFFSET: u32 = 31;
    pub const LP_ANA_BOD_MODE0_RESET_ENA_MASK: u32 = 0x80000000;
    pub const LP_ANA_BOD_MODE0_RESET_ENA: u32 = 1 << 31;

    /// **LP_ANA_BOD_MODE0_RESET_SEL**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_ANA_BOD_MODE0_RESET_SEL_OFFSET: u32 = 30;
    pub const LP_ANA_BOD_MODE0_RESET_SEL_MASK: u32 = 0x40000000;
    pub const LP_ANA_BOD_MODE0_RESET_SEL: u32 = 1 << 30;

    /// **LP_ANA_BOD_MODE0_INTR_ENA**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_ANA_BOD_MODE0_INTR_ENA_OFFSET: u32 = 29;
    pub const LP_ANA_BOD_MODE0_INTR_ENA_MASK: u32 = 0x20000000;
    pub const LP_ANA_BOD_MODE0_INTR_ENA: u32 = 1 << 29;

    /// **LP_ANA_BOD_MODE0_CNT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_ANA_BOD_MODE0_CNT_CLR_OFFSET: u32 = 28;
    pub const LP_ANA_BOD_MODE0_CNT_CLR_MASK: u32 = 0x10000000;
    pub const LP_ANA_BOD_MODE0_CNT_CLR: u32 = 1 << 28;

    /// **LP_ANA_BOD_MODE0_RESET_WAIT**
    ///
    /// need_des
    ///
    /// Bits `[27:18]`, mask `0x0ffc0000`
    pub const LP_ANA_BOD_MODE0_RESET_WAIT_OFFSET: u32 = 18;
    pub const LP_ANA_BOD_MODE0_RESET_WAIT_MASK: u32 = 0x0ffc0000;

    /// **LP_ANA_BOD_MODE0_INTR_WAIT**
    ///
    /// need_des
    ///
    /// Bits `[17:8]`, mask `0x0003ff00`
    pub const LP_ANA_BOD_MODE0_INTR_WAIT_OFFSET: u32 = 8;
    pub const LP_ANA_BOD_MODE0_INTR_WAIT_MASK: u32 = 0x0003ff00;

    /// **LP_ANA_BOD_MODE0_PD_RF_ENA**
    ///
    /// need_des
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const LP_ANA_BOD_MODE0_PD_RF_ENA_OFFSET: u32 = 7;
    pub const LP_ANA_BOD_MODE0_PD_RF_ENA_MASK: u32 = 0x00000080;
    pub const LP_ANA_BOD_MODE0_PD_RF_ENA: u32 = 1 << 7;

    /// **LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA**
    ///
    /// need_des
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA_OFFSET: u32 = 6;
    pub const LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA_MASK: u32 = 0x00000040;
    pub const LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA: u32 = 1 << 6;

    // ======== Field Accessors ========

    /// **LP_ANA_BOD_MODE0_RESET_ENA** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode0_reset_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE0_RESET_ENA_MASK) != 0
    }

    /// Set LP_ANA_BOD_MODE0_RESET_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_bod_mode0_reset_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_BOD_MODE0_RESET_ENA_MASK);
    }

    /// Clear LP_ANA_BOD_MODE0_RESET_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_bod_mode0_reset_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_BOD_MODE0_RESET_ENA_MASK);
    }

    /// **LP_ANA_BOD_MODE0_RESET_SEL** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode0_reset_sel(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE0_RESET_SEL_MASK) != 0
    }

    /// Set LP_ANA_BOD_MODE0_RESET_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_bod_mode0_reset_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_BOD_MODE0_RESET_SEL_MASK);
    }

    /// Clear LP_ANA_BOD_MODE0_RESET_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_bod_mode0_reset_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_BOD_MODE0_RESET_SEL_MASK);
    }

    /// **LP_ANA_BOD_MODE0_INTR_ENA** `[29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode0_intr_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE0_INTR_ENA_MASK) != 0
    }

    /// Set LP_ANA_BOD_MODE0_INTR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_bod_mode0_intr_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_BOD_MODE0_INTR_ENA_MASK);
    }

    /// Clear LP_ANA_BOD_MODE0_INTR_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_bod_mode0_intr_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_BOD_MODE0_INTR_ENA_MASK);
    }

    /// **LP_ANA_BOD_MODE0_CNT_CLR** `[28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode0_cnt_clr(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE0_CNT_CLR_MASK) != 0
    }

    /// Set LP_ANA_BOD_MODE0_CNT_CLR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_bod_mode0_cnt_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_BOD_MODE0_CNT_CLR_MASK);
    }

    /// Clear LP_ANA_BOD_MODE0_CNT_CLR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_bod_mode0_cnt_clr(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_BOD_MODE0_CNT_CLR_MASK);
    }

    /// **LP_ANA_BOD_MODE0_RESET_WAIT** `[27:18]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode0_reset_wait(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE0_RESET_WAIT_MASK)
            >> Self::LP_ANA_BOD_MODE0_RESET_WAIT_OFFSET
    }

    /// Write **LP_ANA_BOD_MODE0_RESET_WAIT** `[27:18]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_bod_mode0_reset_wait(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_BOD_MODE0_RESET_WAIT_MASK)
                | ((value << Self::LP_ANA_BOD_MODE0_RESET_WAIT_OFFSET)
                    & Self::LP_ANA_BOD_MODE0_RESET_WAIT_MASK)
        });
    }

    /// **LP_ANA_BOD_MODE0_INTR_WAIT** `[17:8]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode0_intr_wait(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE0_INTR_WAIT_MASK)
            >> Self::LP_ANA_BOD_MODE0_INTR_WAIT_OFFSET
    }

    /// Write **LP_ANA_BOD_MODE0_INTR_WAIT** `[17:8]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_bod_mode0_intr_wait(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_BOD_MODE0_INTR_WAIT_MASK)
                | ((value << Self::LP_ANA_BOD_MODE0_INTR_WAIT_OFFSET)
                    & Self::LP_ANA_BOD_MODE0_INTR_WAIT_MASK)
        });
    }

    /// **LP_ANA_BOD_MODE0_PD_RF_ENA** `[7]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode0_pd_rf_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE0_PD_RF_ENA_MASK) != 0
    }

    /// Set LP_ANA_BOD_MODE0_PD_RF_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_bod_mode0_pd_rf_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_BOD_MODE0_PD_RF_ENA_MASK);
    }

    /// Clear LP_ANA_BOD_MODE0_PD_RF_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_bod_mode0_pd_rf_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_BOD_MODE0_PD_RF_ENA_MASK);
    }

    /// **LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA** `[6]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode0_close_flash_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA_MASK) != 0
    }

    /// Set LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_bod_mode0_close_flash_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA_MASK);
    }

    /// Clear LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_bod_mode0_close_flash_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA_MASK);
    }
}

/// LP_ANA_BOD_MODE0_CNTL value type for batch read/modify/write operations
///
/// Use `lpanabodmode0cntl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanabodmode0cntl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaBodMode0CntlVal {
    bits: u32,
}

impl LpAnaBodMode0CntlVal {
    /// Create from raw bits
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
        Self { bits: 0x0ffc0100 }
    }

    /// Read **LP_ANA_BOD_MODE0_RESET_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_reset_ena(&self) -> bool {
        (self.bits & LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_RESET_ENA_MASK) != 0
    }

    /// Read **LP_ANA_BOD_MODE0_RESET_SEL** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_reset_sel(&self) -> bool {
        (self.bits & LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_RESET_SEL_MASK) != 0
    }

    /// Read **LP_ANA_BOD_MODE0_INTR_ENA** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_intr_ena(&self) -> bool {
        (self.bits & LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_INTR_ENA_MASK) != 0
    }

    /// Read **LP_ANA_BOD_MODE0_CNT_CLR** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_cnt_clr(&self) -> bool {
        (self.bits & LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_CNT_CLR_MASK) != 0
    }

    /// Read **LP_ANA_BOD_MODE0_RESET_WAIT** `[27:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_reset_wait(&self) -> u32 {
        (self.bits & LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_RESET_WAIT_MASK)
            >> LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_RESET_WAIT_OFFSET
    }

    /// Read **LP_ANA_BOD_MODE0_INTR_WAIT** `[17:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_intr_wait(&self) -> u32 {
        (self.bits & LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_INTR_WAIT_MASK)
            >> LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_INTR_WAIT_OFFSET
    }

    /// Read **LP_ANA_BOD_MODE0_PD_RF_ENA** `[7:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_pd_rf_ena(&self) -> bool {
        (self.bits & LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_PD_RF_ENA_MASK) != 0
    }

    /// Read **LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA** `[6:6]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_close_flash_ena(&self) -> bool {
        (self.bits & LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA_MASK) != 0
    }

    /// Set **LP_ANA_BOD_MODE0_RESET_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode0_reset_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_RESET_ENA_MASK
            } else {
                self.bits & !LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_RESET_ENA_MASK
            },
        }
    }

    /// Set **LP_ANA_BOD_MODE0_RESET_SEL** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode0_reset_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_RESET_SEL_MASK
            } else {
                self.bits & !LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_RESET_SEL_MASK
            },
        }
    }

    /// Set **LP_ANA_BOD_MODE0_INTR_ENA** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode0_intr_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_INTR_ENA_MASK
            } else {
                self.bits & !LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_INTR_ENA_MASK
            },
        }
    }

    /// Set **LP_ANA_BOD_MODE0_CNT_CLR** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode0_cnt_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_CNT_CLR_MASK
            } else {
                self.bits & !LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_CNT_CLR_MASK
            },
        }
    }

    /// Set **LP_ANA_BOD_MODE0_RESET_WAIT** `[27:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode0_reset_wait(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_RESET_WAIT_MASK)
                | ((value << LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_RESET_WAIT_OFFSET)
                    & LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_RESET_WAIT_MASK),
        }
    }

    /// Set **LP_ANA_BOD_MODE0_INTR_WAIT** `[17:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode0_intr_wait(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_INTR_WAIT_MASK)
                | ((value << LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_INTR_WAIT_OFFSET)
                    & LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_INTR_WAIT_MASK),
        }
    }

    /// Set **LP_ANA_BOD_MODE0_PD_RF_ENA** `[7:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode0_pd_rf_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_PD_RF_ENA_MASK
            } else {
                self.bits & !LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_PD_RF_ENA_MASK
            },
        }
    }

    /// Set **LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA** `[6:6]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode0_close_flash_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA_MASK
            } else {
                self.bits & !LpAnaBodMode0Cntl::LP_ANA_BOD_MODE0_CLOSE_FLASH_ENA_MASK
            },
        }
    }
}

impl Default for LpAnaBodMode0CntlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaBodMode0Cntl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaBodMode0CntlVal {
        LpAnaBodMode0CntlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaBodMode0CntlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaBodMode0CntlVal) -> LpAnaBodMode0CntlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_BOD_MODE1_CNTL Register
///
/// need_des
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **LP_ANA_BOD_MODE1_RESET_ENA** `[31]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaBodMode1Cntl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaBodMode1Cntl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaBodMode1Cntl {
    // ======== Field Constants ========

    /// **LP_ANA_BOD_MODE1_RESET_ENA**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_BOD_MODE1_RESET_ENA_OFFSET: u32 = 31;
    pub const LP_ANA_BOD_MODE1_RESET_ENA_MASK: u32 = 0x80000000;
    pub const LP_ANA_BOD_MODE1_RESET_ENA: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **LP_ANA_BOD_MODE1_RESET_ENA** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode1_reset_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE1_RESET_ENA_MASK) != 0
    }

    /// Set LP_ANA_BOD_MODE1_RESET_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_bod_mode1_reset_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_BOD_MODE1_RESET_ENA_MASK);
    }

    /// Clear LP_ANA_BOD_MODE1_RESET_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_bod_mode1_reset_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_BOD_MODE1_RESET_ENA_MASK);
    }
}

/// LP_ANA_BOD_MODE1_CNTL value type for batch read/modify/write operations
///
/// Use `lpanabodmode1cntl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanabodmode1cntl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaBodMode1CntlVal {
    bits: u32,
}

impl LpAnaBodMode1CntlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_BOD_MODE1_RESET_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode1_reset_ena(&self) -> bool {
        (self.bits & LpAnaBodMode1Cntl::LP_ANA_BOD_MODE1_RESET_ENA_MASK) != 0
    }

    /// Set **LP_ANA_BOD_MODE1_RESET_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode1_reset_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaBodMode1Cntl::LP_ANA_BOD_MODE1_RESET_ENA_MASK
            } else {
                self.bits & !LpAnaBodMode1Cntl::LP_ANA_BOD_MODE1_RESET_ENA_MASK
            },
        }
    }
}

impl Default for LpAnaBodMode1CntlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaBodMode1Cntl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaBodMode1CntlVal {
        LpAnaBodMode1CntlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaBodMode1CntlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaBodMode1CntlVal) -> LpAnaBodMode1CntlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_VDD_SOURCE_CNTL Register
///
/// need_des
///
/// Address offset: 0x0008
/// Reset value: 0x040000ff
///
/// ## Fields
///
/// - **LP_ANA_BOD_SOURCE_ENA** `[31:24]` (RW)
///
///   need_des
///
/// - **LP_ANA_VBAT_EVENT_RECORD_CLR** `[23:16]` (WO)
///
///   need_des
///
/// - **LP_ANA_VGOOD_EVENT_RECORD** `[15:8]` (RO)
///
///   need_des
///
/// - **LP_ANA_DETMODE_SEL** `[7:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaVddSourceCntl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaVddSourceCntl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaVddSourceCntl {
    // ======== Field Constants ========

    /// **LP_ANA_BOD_SOURCE_ENA**
    ///
    /// need_des
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const LP_ANA_BOD_SOURCE_ENA_OFFSET: u32 = 24;
    pub const LP_ANA_BOD_SOURCE_ENA_MASK: u32 = 0xff000000;

    /// **LP_ANA_VBAT_EVENT_RECORD_CLR**
    ///
    /// need_des
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const LP_ANA_VBAT_EVENT_RECORD_CLR_OFFSET: u32 = 16;
    pub const LP_ANA_VBAT_EVENT_RECORD_CLR_MASK: u32 = 0x00ff0000;

    /// **LP_ANA_VGOOD_EVENT_RECORD**
    ///
    /// need_des
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const LP_ANA_VGOOD_EVENT_RECORD_OFFSET: u32 = 8;
    pub const LP_ANA_VGOOD_EVENT_RECORD_MASK: u32 = 0x0000ff00;

    /// **LP_ANA_DETMODE_SEL**
    ///
    /// need_des
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const LP_ANA_DETMODE_SEL_OFFSET: u32 = 0;
    pub const LP_ANA_DETMODE_SEL_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **LP_ANA_BOD_SOURCE_ENA** `[31:24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_source_ena(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_BOD_SOURCE_ENA_MASK)
            >> Self::LP_ANA_BOD_SOURCE_ENA_OFFSET
    }

    /// Write **LP_ANA_BOD_SOURCE_ENA** `[31:24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_bod_source_ena(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_BOD_SOURCE_ENA_MASK)
                | ((value << Self::LP_ANA_BOD_SOURCE_ENA_OFFSET) & Self::LP_ANA_BOD_SOURCE_ENA_MASK)
        });
    }

    /// Write **LP_ANA_VBAT_EVENT_RECORD_CLR** `[23:16]` (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_vbat_event_record_clr(&self, value: u32) {
        self.value.write_bits(
            (value << Self::LP_ANA_VBAT_EVENT_RECORD_CLR_OFFSET)
                & Self::LP_ANA_VBAT_EVENT_RECORD_CLR_MASK,
        );
    }

    /// **LP_ANA_VGOOD_EVENT_RECORD** `[15:8]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vgood_event_record(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_VGOOD_EVENT_RECORD_MASK)
            >> Self::LP_ANA_VGOOD_EVENT_RECORD_OFFSET
    }

    /// **LP_ANA_DETMODE_SEL** `[7:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_detmode_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_DETMODE_SEL_MASK) >> Self::LP_ANA_DETMODE_SEL_OFFSET
    }

    /// Write **LP_ANA_DETMODE_SEL** `[7:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_detmode_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_DETMODE_SEL_MASK)
                | ((value << Self::LP_ANA_DETMODE_SEL_OFFSET) & Self::LP_ANA_DETMODE_SEL_MASK)
        });
    }
}

/// LP_ANA_VDD_SOURCE_CNTL value type for batch read/modify/write operations
///
/// Use `lpanavddsourcecntl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanavddsourcecntl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaVddSourceCntlVal {
    bits: u32,
}

impl LpAnaVddSourceCntlVal {
    /// Create from raw bits
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
        Self { bits: 0x040000ff }
    }

    /// Read **LP_ANA_BOD_SOURCE_ENA** `[31:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_source_ena(&self) -> u32 {
        (self.bits & LpAnaVddSourceCntl::LP_ANA_BOD_SOURCE_ENA_MASK)
            >> LpAnaVddSourceCntl::LP_ANA_BOD_SOURCE_ENA_OFFSET
    }

    /// Read **LP_ANA_VGOOD_EVENT_RECORD** `[15:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vgood_event_record(&self) -> u32 {
        (self.bits & LpAnaVddSourceCntl::LP_ANA_VGOOD_EVENT_RECORD_MASK)
            >> LpAnaVddSourceCntl::LP_ANA_VGOOD_EVENT_RECORD_OFFSET
    }

    /// Read **LP_ANA_DETMODE_SEL** `[7:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_detmode_sel(&self) -> u32 {
        (self.bits & LpAnaVddSourceCntl::LP_ANA_DETMODE_SEL_MASK)
            >> LpAnaVddSourceCntl::LP_ANA_DETMODE_SEL_OFFSET
    }

    /// Set **LP_ANA_BOD_SOURCE_ENA** `[31:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_source_ena(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaVddSourceCntl::LP_ANA_BOD_SOURCE_ENA_MASK)
                | ((value << LpAnaVddSourceCntl::LP_ANA_BOD_SOURCE_ENA_OFFSET)
                    & LpAnaVddSourceCntl::LP_ANA_BOD_SOURCE_ENA_MASK),
        }
    }

    /// Set **LP_ANA_VBAT_EVENT_RECORD_CLR** `[23:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vbat_event_record_clr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaVddSourceCntl::LP_ANA_VBAT_EVENT_RECORD_CLR_MASK)
                | ((value << LpAnaVddSourceCntl::LP_ANA_VBAT_EVENT_RECORD_CLR_OFFSET)
                    & LpAnaVddSourceCntl::LP_ANA_VBAT_EVENT_RECORD_CLR_MASK),
        }
    }

    /// Set **LP_ANA_DETMODE_SEL** `[7:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_detmode_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaVddSourceCntl::LP_ANA_DETMODE_SEL_MASK)
                | ((value << LpAnaVddSourceCntl::LP_ANA_DETMODE_SEL_OFFSET)
                    & LpAnaVddSourceCntl::LP_ANA_DETMODE_SEL_MASK),
        }
    }
}

impl Default for LpAnaVddSourceCntlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaVddSourceCntl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaVddSourceCntlVal {
        LpAnaVddSourceCntlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaVddSourceCntlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaVddSourceCntlVal) -> LpAnaVddSourceCntlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_VDDBAT_BOD_CNTL Register
///
/// need_des
///
/// Address offset: 0x000c
/// Reset value: 0xffc00000
///
/// ## Fields
///
/// - **LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET** `[31:22]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_UPVOLTAGE_TARGET** `[21:12]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_CNT_CLR** `[11]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_CHARGER** `[10]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_UNDERVOLTAGE_FLAG** `[0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaVddbatBodCntl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaVddbatBodCntl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaVddbatBodCntl {
    // ======== Field Constants ========

    /// **LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET**
    ///
    /// need_des
    ///
    /// Bits `[31:22]`, mask `0xffc00000`
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_OFFSET: u32 = 22;
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_MASK: u32 = 0xffc00000;

    /// **LP_ANA_VDDBAT_UPVOLTAGE_TARGET**
    ///
    /// need_des
    ///
    /// Bits `[21:12]`, mask `0x003ff000`
    pub const LP_ANA_VDDBAT_UPVOLTAGE_TARGET_OFFSET: u32 = 12;
    pub const LP_ANA_VDDBAT_UPVOLTAGE_TARGET_MASK: u32 = 0x003ff000;

    /// **LP_ANA_VDDBAT_CNT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const LP_ANA_VDDBAT_CNT_CLR_OFFSET: u32 = 11;
    pub const LP_ANA_VDDBAT_CNT_CLR_MASK: u32 = 0x00000800;
    pub const LP_ANA_VDDBAT_CNT_CLR: u32 = 1 << 11;

    /// **LP_ANA_VDDBAT_CHARGER**
    ///
    /// need_des
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const LP_ANA_VDDBAT_CHARGER_OFFSET: u32 = 10;
    pub const LP_ANA_VDDBAT_CHARGER_MASK: u32 = 0x00000400;
    pub const LP_ANA_VDDBAT_CHARGER: u32 = 1 << 10;

    /// **LP_ANA_VDDBAT_UNDERVOLTAGE_FLAG**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_FLAG_OFFSET: u32 = 0;
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_FLAG_MASK: u32 = 0x00000001;
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_FLAG: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET** `[31:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_undervoltage_target(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_MASK)
            >> Self::LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_OFFSET
    }

    /// Write **LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET** `[31:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_vddbat_undervoltage_target(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_MASK)
                | ((value << Self::LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_OFFSET)
                    & Self::LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_MASK)
        });
    }

    /// **LP_ANA_VDDBAT_UPVOLTAGE_TARGET** `[21:12]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_upvoltage_target(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_UPVOLTAGE_TARGET_MASK)
            >> Self::LP_ANA_VDDBAT_UPVOLTAGE_TARGET_OFFSET
    }

    /// Write **LP_ANA_VDDBAT_UPVOLTAGE_TARGET** `[21:12]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_vddbat_upvoltage_target(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_VDDBAT_UPVOLTAGE_TARGET_MASK)
                | ((value << Self::LP_ANA_VDDBAT_UPVOLTAGE_TARGET_OFFSET)
                    & Self::LP_ANA_VDDBAT_UPVOLTAGE_TARGET_MASK)
        });
    }

    /// **LP_ANA_VDDBAT_CNT_CLR** `[11]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_cnt_clr(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_CNT_CLR_MASK) != 0
    }

    /// Set LP_ANA_VDDBAT_CNT_CLR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_vddbat_cnt_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_VDDBAT_CNT_CLR_MASK);
    }

    /// Clear LP_ANA_VDDBAT_CNT_CLR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_vddbat_cnt_clr(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_VDDBAT_CNT_CLR_MASK);
    }

    /// **LP_ANA_VDDBAT_CHARGER** `[10]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_charger(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_CHARGER_MASK) != 0
    }

    /// Set LP_ANA_VDDBAT_CHARGER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_vddbat_charger(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_VDDBAT_CHARGER_MASK);
    }

    /// Clear LP_ANA_VDDBAT_CHARGER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_vddbat_charger(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_VDDBAT_CHARGER_MASK);
    }

    /// **LP_ANA_VDDBAT_UNDERVOLTAGE_FLAG** `[0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_undervoltage_flag(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_UNDERVOLTAGE_FLAG_MASK) != 0
    }
}

/// LP_ANA_VDDBAT_BOD_CNTL value type for batch read/modify/write operations
///
/// Use `lpanavddbatbodcntl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanavddbatbodcntl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaVddbatBodCntlVal {
    bits: u32,
}

impl LpAnaVddbatBodCntlVal {
    /// Create from raw bits
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
        Self { bits: 0xffc00000 }
    }

    /// Read **LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET** `[31:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_undervoltage_target(&self) -> u32 {
        (self.bits & LpAnaVddbatBodCntl::LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_MASK)
            >> LpAnaVddbatBodCntl::LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_OFFSET
    }

    /// Read **LP_ANA_VDDBAT_UPVOLTAGE_TARGET** `[21:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_upvoltage_target(&self) -> u32 {
        (self.bits & LpAnaVddbatBodCntl::LP_ANA_VDDBAT_UPVOLTAGE_TARGET_MASK)
            >> LpAnaVddbatBodCntl::LP_ANA_VDDBAT_UPVOLTAGE_TARGET_OFFSET
    }

    /// Read **LP_ANA_VDDBAT_CNT_CLR** `[11:11]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_cnt_clr(&self) -> bool {
        (self.bits & LpAnaVddbatBodCntl::LP_ANA_VDDBAT_CNT_CLR_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_CHARGER** `[10:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_charger(&self) -> bool {
        (self.bits & LpAnaVddbatBodCntl::LP_ANA_VDDBAT_CHARGER_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_UNDERVOLTAGE_FLAG** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_undervoltage_flag(&self) -> bool {
        (self.bits & LpAnaVddbatBodCntl::LP_ANA_VDDBAT_UNDERVOLTAGE_FLAG_MASK) != 0
    }

    /// Set **LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET** `[31:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_undervoltage_target(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaVddbatBodCntl::LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_MASK)
                | ((value << LpAnaVddbatBodCntl::LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_OFFSET)
                    & LpAnaVddbatBodCntl::LP_ANA_VDDBAT_UNDERVOLTAGE_TARGET_MASK),
        }
    }

    /// Set **LP_ANA_VDDBAT_UPVOLTAGE_TARGET** `[21:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_upvoltage_target(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaVddbatBodCntl::LP_ANA_VDDBAT_UPVOLTAGE_TARGET_MASK)
                | ((value << LpAnaVddbatBodCntl::LP_ANA_VDDBAT_UPVOLTAGE_TARGET_OFFSET)
                    & LpAnaVddbatBodCntl::LP_ANA_VDDBAT_UPVOLTAGE_TARGET_MASK),
        }
    }

    /// Set **LP_ANA_VDDBAT_CNT_CLR** `[11:11]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_cnt_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaVddbatBodCntl::LP_ANA_VDDBAT_CNT_CLR_MASK
            } else {
                self.bits & !LpAnaVddbatBodCntl::LP_ANA_VDDBAT_CNT_CLR_MASK
            },
        }
    }

    /// Set **LP_ANA_VDDBAT_CHARGER** `[10:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_charger(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaVddbatBodCntl::LP_ANA_VDDBAT_CHARGER_MASK
            } else {
                self.bits & !LpAnaVddbatBodCntl::LP_ANA_VDDBAT_CHARGER_MASK
            },
        }
    }
}

impl Default for LpAnaVddbatBodCntlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaVddbatBodCntl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaVddbatBodCntlVal {
        LpAnaVddbatBodCntlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaVddbatBodCntlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaVddbatBodCntlVal) -> LpAnaVddbatBodCntlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_VDDBAT_CHARGE_CNTL Register
///
/// need_des
///
/// Address offset: 0x0010
/// Reset value: 0xffc00000
///
/// ## Fields
///
/// - **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET** `[31:22]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET** `[21:12]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_CHARGE_CNT_CLR** `[11]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_CHARGE_CHARGER** `[10]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_FLAG** `[0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaVddbatChargeCntl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaVddbatChargeCntl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaVddbatChargeCntl {
    // ======== Field Constants ========

    /// **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET**
    ///
    /// need_des
    ///
    /// Bits `[31:22]`, mask `0xffc00000`
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_OFFSET: u32 = 22;
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_MASK: u32 = 0xffc00000;

    /// **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET**
    ///
    /// need_des
    ///
    /// Bits `[21:12]`, mask `0x003ff000`
    pub const LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET_OFFSET: u32 = 12;
    pub const LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET_MASK: u32 = 0x003ff000;

    /// **LP_ANA_VDDBAT_CHARGE_CNT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const LP_ANA_VDDBAT_CHARGE_CNT_CLR_OFFSET: u32 = 11;
    pub const LP_ANA_VDDBAT_CHARGE_CNT_CLR_MASK: u32 = 0x00000800;
    pub const LP_ANA_VDDBAT_CHARGE_CNT_CLR: u32 = 1 << 11;

    /// **LP_ANA_VDDBAT_CHARGE_CHARGER**
    ///
    /// need_des
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const LP_ANA_VDDBAT_CHARGE_CHARGER_OFFSET: u32 = 10;
    pub const LP_ANA_VDDBAT_CHARGE_CHARGER_MASK: u32 = 0x00000400;
    pub const LP_ANA_VDDBAT_CHARGE_CHARGER: u32 = 1 << 10;

    /// **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_FLAG**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_FLAG_OFFSET: u32 = 0;
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_FLAG_MASK: u32 = 0x00000001;
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_FLAG: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET** `[31:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_charge_undervoltage_target(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_MASK)
            >> Self::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_OFFSET
    }

    /// Write **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET** `[31:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_vddbat_charge_undervoltage_target(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_MASK)
                | ((value << Self::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_OFFSET)
                    & Self::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_MASK)
        });
    }

    /// **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET** `[21:12]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_charge_upvoltage_target(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET_MASK)
            >> Self::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET_OFFSET
    }

    /// Write **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET** `[21:12]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_vddbat_charge_upvoltage_target(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET_MASK)
                | ((value << Self::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET_OFFSET)
                    & Self::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET_MASK)
        });
    }

    /// **LP_ANA_VDDBAT_CHARGE_CNT_CLR** `[11]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_charge_cnt_clr(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_CHARGE_CNT_CLR_MASK) != 0
    }

    /// Set LP_ANA_VDDBAT_CHARGE_CNT_CLR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_vddbat_charge_cnt_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_VDDBAT_CHARGE_CNT_CLR_MASK);
    }

    /// Clear LP_ANA_VDDBAT_CHARGE_CNT_CLR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_vddbat_charge_cnt_clr(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_VDDBAT_CHARGE_CNT_CLR_MASK);
    }

    /// **LP_ANA_VDDBAT_CHARGE_CHARGER** `[10]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_charge_charger(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_CHARGE_CHARGER_MASK) != 0
    }

    /// Set LP_ANA_VDDBAT_CHARGE_CHARGER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_vddbat_charge_charger(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_VDDBAT_CHARGE_CHARGER_MASK);
    }

    /// Clear LP_ANA_VDDBAT_CHARGE_CHARGER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_vddbat_charge_charger(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_VDDBAT_CHARGE_CHARGER_MASK);
    }

    /// **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_FLAG** `[0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_charge_undervoltage_flag(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_FLAG_MASK) != 0
    }
}

/// LP_ANA_VDDBAT_CHARGE_CNTL value type for batch read/modify/write operations
///
/// Use `lpanavddbatchargecntl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanavddbatchargecntl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaVddbatChargeCntlVal {
    bits: u32,
}

impl LpAnaVddbatChargeCntlVal {
    /// Create from raw bits
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
        Self { bits: 0xffc00000 }
    }

    /// Read **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET** `[31:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_charge_undervoltage_target(&self) -> u32 {
        (self.bits & LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_MASK)
            >> LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_OFFSET
    }

    /// Read **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET** `[21:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_charge_upvoltage_target(&self) -> u32 {
        (self.bits & LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET_MASK)
            >> LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET_OFFSET
    }

    /// Read **LP_ANA_VDDBAT_CHARGE_CNT_CLR** `[11:11]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_charge_cnt_clr(&self) -> bool {
        (self.bits & LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_CNT_CLR_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_CHARGE_CHARGER** `[10:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_charge_charger(&self) -> bool {
        (self.bits & LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_CHARGER_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_FLAG** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_charge_undervoltage_flag(&self) -> bool {
        (self.bits & LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_FLAG_MASK) != 0
    }

    /// Set **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET** `[31:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_charge_undervoltage_target(self, value: u32) -> Self {
        Self {
            bits: (self.bits
                & !LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_MASK)
                | ((value
                    << LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_OFFSET)
                    & LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_MASK),
        }
    }

    /// Set **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET** `[21:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_charge_upvoltage_target(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET_MASK)
                | ((value << LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET_OFFSET)
                    & LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_TARGET_MASK),
        }
    }

    /// Set **LP_ANA_VDDBAT_CHARGE_CNT_CLR** `[11:11]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_charge_cnt_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_CNT_CLR_MASK
            } else {
                self.bits & !LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_CNT_CLR_MASK
            },
        }
    }

    /// Set **LP_ANA_VDDBAT_CHARGE_CHARGER** `[10:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_charge_charger(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_CHARGER_MASK
            } else {
                self.bits & !LpAnaVddbatChargeCntl::LP_ANA_VDDBAT_CHARGE_CHARGER_MASK
            },
        }
    }
}

impl Default for LpAnaVddbatChargeCntlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaVddbatChargeCntl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaVddbatChargeCntlVal {
        LpAnaVddbatChargeCntlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaVddbatChargeCntlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaVddbatChargeCntlVal) -> LpAnaVddbatChargeCntlVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_CK_GLITCH_CNTL Register
///
/// need_des
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **LP_ANA_CK_GLITCH_RESET_ENA** `[31]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaCkGlitchCntl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaCkGlitchCntl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaCkGlitchCntl {
    // ======== Field Constants ========

    /// **LP_ANA_CK_GLITCH_RESET_ENA**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_CK_GLITCH_RESET_ENA_OFFSET: u32 = 31;
    pub const LP_ANA_CK_GLITCH_RESET_ENA_MASK: u32 = 0x80000000;
    pub const LP_ANA_CK_GLITCH_RESET_ENA: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **LP_ANA_CK_GLITCH_RESET_ENA** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_ck_glitch_reset_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_CK_GLITCH_RESET_ENA_MASK) != 0
    }

    /// Set LP_ANA_CK_GLITCH_RESET_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_ck_glitch_reset_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_CK_GLITCH_RESET_ENA_MASK);
    }

    /// Clear LP_ANA_CK_GLITCH_RESET_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_ck_glitch_reset_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_CK_GLITCH_RESET_ENA_MASK);
    }
}

/// LP_ANA_CK_GLITCH_CNTL value type for batch read/modify/write operations
///
/// Use `lpanackglitchcntl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanackglitchcntl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaCkGlitchCntlVal {
    bits: u32,
}

impl LpAnaCkGlitchCntlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_CK_GLITCH_RESET_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_ck_glitch_reset_ena(&self) -> bool {
        (self.bits & LpAnaCkGlitchCntl::LP_ANA_CK_GLITCH_RESET_ENA_MASK) != 0
    }

    /// Set **LP_ANA_CK_GLITCH_RESET_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_ck_glitch_reset_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaCkGlitchCntl::LP_ANA_CK_GLITCH_RESET_ENA_MASK
            } else {
                self.bits & !LpAnaCkGlitchCntl::LP_ANA_CK_GLITCH_RESET_ENA_MASK
            },
        }
    }
}

impl Default for LpAnaCkGlitchCntlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaCkGlitchCntl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaCkGlitchCntlVal {
        LpAnaCkGlitchCntlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaCkGlitchCntlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaCkGlitchCntlVal) -> LpAnaCkGlitchCntlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_PG_GLITCH_CNTL Register
///
/// need_des
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **LP_ANA_POWER_GLITCH_RESET_ENA** `[31]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaPgGlitchCntl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaPgGlitchCntl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaPgGlitchCntl {
    // ======== Field Constants ========

    /// **LP_ANA_POWER_GLITCH_RESET_ENA**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_POWER_GLITCH_RESET_ENA_OFFSET: u32 = 31;
    pub const LP_ANA_POWER_GLITCH_RESET_ENA_MASK: u32 = 0x80000000;
    pub const LP_ANA_POWER_GLITCH_RESET_ENA: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **LP_ANA_POWER_GLITCH_RESET_ENA** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_power_glitch_reset_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_POWER_GLITCH_RESET_ENA_MASK) != 0
    }

    /// Set LP_ANA_POWER_GLITCH_RESET_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_power_glitch_reset_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_POWER_GLITCH_RESET_ENA_MASK);
    }

    /// Clear LP_ANA_POWER_GLITCH_RESET_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_power_glitch_reset_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_POWER_GLITCH_RESET_ENA_MASK);
    }
}

/// LP_ANA_PG_GLITCH_CNTL value type for batch read/modify/write operations
///
/// Use `lpanapgglitchcntl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanapgglitchcntl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaPgGlitchCntlVal {
    bits: u32,
}

impl LpAnaPgGlitchCntlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_POWER_GLITCH_RESET_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_power_glitch_reset_ena(&self) -> bool {
        (self.bits & LpAnaPgGlitchCntl::LP_ANA_POWER_GLITCH_RESET_ENA_MASK) != 0
    }

    /// Set **LP_ANA_POWER_GLITCH_RESET_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_power_glitch_reset_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaPgGlitchCntl::LP_ANA_POWER_GLITCH_RESET_ENA_MASK
            } else {
                self.bits & !LpAnaPgGlitchCntl::LP_ANA_POWER_GLITCH_RESET_ENA_MASK
            },
        }
    }
}

impl Default for LpAnaPgGlitchCntlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaPgGlitchCntl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaPgGlitchCntlVal {
        LpAnaPgGlitchCntlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaPgGlitchCntlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaPgGlitchCntlVal) -> LpAnaPgGlitchCntlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_FIB_ENABLE Register
///
/// need_des
///
/// Address offset: 0x001c
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **LP_ANA_ANA_FIB_ENA** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaFibEnable {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaFibEnable {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaFibEnable {
    // ======== Field Constants ========

    /// **LP_ANA_ANA_FIB_ENA**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_ANA_ANA_FIB_ENA_OFFSET: u32 = 0;
    pub const LP_ANA_ANA_FIB_ENA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_ANA_ANA_FIB_ENA** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_ana_fib_ena(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_ANA_FIB_ENA_MASK) >> Self::LP_ANA_ANA_FIB_ENA_OFFSET
    }

    /// Write **LP_ANA_ANA_FIB_ENA** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_ana_fib_ena(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_ANA_FIB_ENA_MASK)
                | ((value << Self::LP_ANA_ANA_FIB_ENA_OFFSET) & Self::LP_ANA_ANA_FIB_ENA_MASK)
        });
    }
}

/// LP_ANA_FIB_ENABLE value type for batch read/modify/write operations
///
/// Use `lpanafibenable.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanafibenable.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaFibEnableVal {
    bits: u32,
}

impl LpAnaFibEnableVal {
    /// Create from raw bits
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

    /// Read **LP_ANA_ANA_FIB_ENA** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_ana_fib_ena(&self) -> u32 {
        (self.bits & LpAnaFibEnable::LP_ANA_ANA_FIB_ENA_MASK)
            >> LpAnaFibEnable::LP_ANA_ANA_FIB_ENA_OFFSET
    }

    /// Set **LP_ANA_ANA_FIB_ENA** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_ana_fib_ena(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaFibEnable::LP_ANA_ANA_FIB_ENA_MASK)
                | ((value << LpAnaFibEnable::LP_ANA_ANA_FIB_ENA_OFFSET)
                    & LpAnaFibEnable::LP_ANA_ANA_FIB_ENA_MASK),
        }
    }
}

impl Default for LpAnaFibEnableVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaFibEnable {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaFibEnableVal {
        LpAnaFibEnableVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaFibEnableVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaFibEnableVal) -> LpAnaFibEnableVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_INT_RAW Register
///
/// need_des
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **LP_ANA_BOD_MODE0_INT_RAW** `[31]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW** `[30]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW** `[29]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW** `[28]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW** `[27]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaIntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaIntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaIntRaw {
    // ======== Field Constants ========

    /// **LP_ANA_BOD_MODE0_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_BOD_MODE0_INT_RAW_OFFSET: u32 = 31;
    pub const LP_ANA_BOD_MODE0_INT_RAW_MASK: u32 = 0x80000000;
    pub const LP_ANA_BOD_MODE0_INT_RAW: u32 = 1 << 31;

    /// **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW_OFFSET: u32 = 30;
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW_MASK: u32 = 0x40000000;
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW: u32 = 1 << 30;

    /// **LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW_OFFSET: u32 = 29;
    pub const LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW_MASK: u32 = 0x20000000;
    pub const LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW: u32 = 1 << 29;

    /// **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW_OFFSET: u32 = 28;
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW_MASK: u32 = 0x10000000;
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW: u32 = 1 << 28;

    /// **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW_OFFSET: u32 = 27;
    pub const LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW_MASK: u32 = 0x08000000;
    pub const LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW: u32 = 1 << 27;

    // ======== Field Accessors ========

    /// **LP_ANA_BOD_MODE0_INT_RAW** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode0_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE0_INT_RAW_MASK) != 0
    }

    /// Set LP_ANA_BOD_MODE0_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_bod_mode0_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_BOD_MODE0_INT_RAW_MASK);
    }

    /// Clear LP_ANA_BOD_MODE0_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_bod_mode0_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_BOD_MODE0_INT_RAW_MASK);
    }

    /// **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_undervoltage_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW_MASK) != 0
    }

    /// Set LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_vddbat_undervoltage_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW_MASK);
    }

    /// Clear LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_vddbat_undervoltage_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW_MASK);
    }

    /// **LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW** `[29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_upvoltage_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW_MASK) != 0
    }

    /// Set LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_vddbat_upvoltage_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW_MASK);
    }

    /// Clear LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_vddbat_upvoltage_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW_MASK);
    }

    /// **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW** `[28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_charge_undervoltage_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW_MASK) != 0
    }

    /// Set LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_vddbat_charge_undervoltage_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW_MASK);
    }

    /// Clear LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_vddbat_charge_undervoltage_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW_MASK);
    }

    /// **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW** `[27]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_charge_upvoltage_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW_MASK) != 0
    }

    /// Set LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_vddbat_charge_upvoltage_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW_MASK);
    }

    /// Clear LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_vddbat_charge_upvoltage_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW_MASK);
    }
}

/// LP_ANA_INT_RAW value type for batch read/modify/write operations
///
/// Use `lpanaintraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanaintraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaIntRawVal {
    bits: u32,
}

impl LpAnaIntRawVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_BOD_MODE0_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_int_raw(&self) -> bool {
        (self.bits & LpAnaIntRaw::LP_ANA_BOD_MODE0_INT_RAW_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_undervoltage_int_raw(&self) -> bool {
        (self.bits & LpAnaIntRaw::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_upvoltage_int_raw(&self) -> bool {
        (self.bits & LpAnaIntRaw::LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_charge_undervoltage_int_raw(&self) -> bool {
        (self.bits & LpAnaIntRaw::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_charge_upvoltage_int_raw(&self) -> bool {
        (self.bits & LpAnaIntRaw::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW_MASK) != 0
    }

    /// Set **LP_ANA_BOD_MODE0_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode0_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntRaw::LP_ANA_BOD_MODE0_INT_RAW_MASK
            } else {
                self.bits & !LpAnaIntRaw::LP_ANA_BOD_MODE0_INT_RAW_MASK
            },
        }
    }

    /// Set **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_undervoltage_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntRaw::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW_MASK
            } else {
                self.bits & !LpAnaIntRaw::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_RAW_MASK
            },
        }
    }

    /// Set **LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_upvoltage_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntRaw::LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW_MASK
            } else {
                self.bits & !LpAnaIntRaw::LP_ANA_VDDBAT_UPVOLTAGE_INT_RAW_MASK
            },
        }
    }

    /// Set **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_charge_undervoltage_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntRaw::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW_MASK
            } else {
                self.bits & !LpAnaIntRaw::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_RAW_MASK
            },
        }
    }

    /// Set **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_charge_upvoltage_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntRaw::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW_MASK
            } else {
                self.bits & !LpAnaIntRaw::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_RAW_MASK
            },
        }
    }
}

impl Default for LpAnaIntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaIntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaIntRawVal {
        LpAnaIntRawVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaIntRawVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaIntRawVal) -> LpAnaIntRawVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_INT_ST Register
///
/// need_des
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **LP_ANA_BOD_MODE0_INT_ST** `[31]` (RO)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ST** `[30]` (RO)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_UPVOLTAGE_INT_ST** `[29]` (RO)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ST** `[28]` (RO)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ST** `[27]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaIntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaIntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaIntSt {
    // ======== Field Constants ========

    /// **LP_ANA_BOD_MODE0_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_BOD_MODE0_INT_ST_OFFSET: u32 = 31;
    pub const LP_ANA_BOD_MODE0_INT_ST_MASK: u32 = 0x80000000;
    pub const LP_ANA_BOD_MODE0_INT_ST: u32 = 1 << 31;

    /// **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ST_OFFSET: u32 = 30;
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ST_MASK: u32 = 0x40000000;
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ST: u32 = 1 << 30;

    /// **LP_ANA_VDDBAT_UPVOLTAGE_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_ANA_VDDBAT_UPVOLTAGE_INT_ST_OFFSET: u32 = 29;
    pub const LP_ANA_VDDBAT_UPVOLTAGE_INT_ST_MASK: u32 = 0x20000000;
    pub const LP_ANA_VDDBAT_UPVOLTAGE_INT_ST: u32 = 1 << 29;

    /// **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ST_OFFSET: u32 = 28;
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ST_MASK: u32 = 0x10000000;
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ST: u32 = 1 << 28;

    /// **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ST_OFFSET: u32 = 27;
    pub const LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ST_MASK: u32 = 0x08000000;
    pub const LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ST: u32 = 1 << 27;

    // ======== Field Accessors ========

    /// **LP_ANA_BOD_MODE0_INT_ST** `[31]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode0_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE0_INT_ST_MASK) != 0
    }

    /// **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ST** `[30]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_undervoltage_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ST_MASK) != 0
    }

    /// **LP_ANA_VDDBAT_UPVOLTAGE_INT_ST** `[29]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_upvoltage_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_UPVOLTAGE_INT_ST_MASK) != 0
    }

    /// **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ST** `[28]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_charge_undervoltage_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ST_MASK) != 0
    }

    /// **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ST** `[27]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_charge_upvoltage_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ST_MASK) != 0
    }
}

/// LP_ANA_INT_ST value type for batch read/modify/write operations
///
/// Use `lpanaintst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanaintst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaIntStVal {
    bits: u32,
}

impl LpAnaIntStVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_BOD_MODE0_INT_ST** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_int_st(&self) -> bool {
        (self.bits & LpAnaIntSt::LP_ANA_BOD_MODE0_INT_ST_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ST** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_undervoltage_int_st(&self) -> bool {
        (self.bits & LpAnaIntSt::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ST_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_UPVOLTAGE_INT_ST** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_upvoltage_int_st(&self) -> bool {
        (self.bits & LpAnaIntSt::LP_ANA_VDDBAT_UPVOLTAGE_INT_ST_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ST** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_charge_undervoltage_int_st(&self) -> bool {
        (self.bits & LpAnaIntSt::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ST_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ST** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_charge_upvoltage_int_st(&self) -> bool {
        (self.bits & LpAnaIntSt::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ST_MASK) != 0
    }
}

impl Default for LpAnaIntStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaIntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaIntStVal {
        LpAnaIntStVal::from_bits(self.value.read_bits())
    }
}

/// LP_ANA_INT_ENA Register
///
/// need_des
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **LP_ANA_BOD_MODE0_INT_ENA** `[31]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA** `[30]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA** `[29]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA** `[28]` (RW)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA** `[27]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaIntEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaIntEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaIntEna {
    // ======== Field Constants ========

    /// **LP_ANA_BOD_MODE0_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_BOD_MODE0_INT_ENA_OFFSET: u32 = 31;
    pub const LP_ANA_BOD_MODE0_INT_ENA_MASK: u32 = 0x80000000;
    pub const LP_ANA_BOD_MODE0_INT_ENA: u32 = 1 << 31;

    /// **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA_OFFSET: u32 = 30;
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA_MASK: u32 = 0x40000000;
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA: u32 = 1 << 30;

    /// **LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA_OFFSET: u32 = 29;
    pub const LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA_MASK: u32 = 0x20000000;
    pub const LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA: u32 = 1 << 29;

    /// **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA_OFFSET: u32 = 28;
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA_MASK: u32 = 0x10000000;
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA: u32 = 1 << 28;

    /// **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA_OFFSET: u32 = 27;
    pub const LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA_MASK: u32 = 0x08000000;
    pub const LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA: u32 = 1 << 27;

    // ======== Field Accessors ========

    /// **LP_ANA_BOD_MODE0_INT_ENA** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode0_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE0_INT_ENA_MASK) != 0
    }

    /// Set LP_ANA_BOD_MODE0_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_bod_mode0_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_BOD_MODE0_INT_ENA_MASK);
    }

    /// Clear LP_ANA_BOD_MODE0_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_bod_mode0_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_BOD_MODE0_INT_ENA_MASK);
    }

    /// **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_undervoltage_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA_MASK) != 0
    }

    /// Set LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_vddbat_undervoltage_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA_MASK);
    }

    /// Clear LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_vddbat_undervoltage_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA_MASK);
    }

    /// **LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA** `[29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_upvoltage_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA_MASK) != 0
    }

    /// Set LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_vddbat_upvoltage_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA_MASK);
    }

    /// Clear LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_vddbat_upvoltage_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA_MASK);
    }

    /// **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA** `[28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_charge_undervoltage_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA_MASK) != 0
    }

    /// Set LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_vddbat_charge_undervoltage_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA_MASK);
    }

    /// Clear LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_vddbat_charge_undervoltage_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA_MASK);
    }

    /// **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA** `[27]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_vddbat_charge_upvoltage_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA_MASK) != 0
    }

    /// Set LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_vddbat_charge_upvoltage_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA_MASK);
    }

    /// Clear LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_vddbat_charge_upvoltage_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA_MASK);
    }
}

/// LP_ANA_INT_ENA value type for batch read/modify/write operations
///
/// Use `lpanaintena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanaintena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaIntEnaVal {
    bits: u32,
}

impl LpAnaIntEnaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_BOD_MODE0_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_int_ena(&self) -> bool {
        (self.bits & LpAnaIntEna::LP_ANA_BOD_MODE0_INT_ENA_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_undervoltage_int_ena(&self) -> bool {
        (self.bits & LpAnaIntEna::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_upvoltage_int_ena(&self) -> bool {
        (self.bits & LpAnaIntEna::LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_charge_undervoltage_int_ena(&self) -> bool {
        (self.bits & LpAnaIntEna::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA_MASK) != 0
    }

    /// Read **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_vddbat_charge_upvoltage_int_ena(&self) -> bool {
        (self.bits & LpAnaIntEna::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA_MASK) != 0
    }

    /// Set **LP_ANA_BOD_MODE0_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode0_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntEna::LP_ANA_BOD_MODE0_INT_ENA_MASK
            } else {
                self.bits & !LpAnaIntEna::LP_ANA_BOD_MODE0_INT_ENA_MASK
            },
        }
    }

    /// Set **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_undervoltage_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntEna::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA_MASK
            } else {
                self.bits & !LpAnaIntEna::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_ENA_MASK
            },
        }
    }

    /// Set **LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_upvoltage_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntEna::LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA_MASK
            } else {
                self.bits & !LpAnaIntEna::LP_ANA_VDDBAT_UPVOLTAGE_INT_ENA_MASK
            },
        }
    }

    /// Set **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_charge_undervoltage_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntEna::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA_MASK
            } else {
                self.bits & !LpAnaIntEna::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_ENA_MASK
            },
        }
    }

    /// Set **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_charge_upvoltage_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntEna::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA_MASK
            } else {
                self.bits & !LpAnaIntEna::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_ENA_MASK
            },
        }
    }
}

impl Default for LpAnaIntEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaIntEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaIntEnaVal {
        LpAnaIntEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaIntEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaIntEnaVal) -> LpAnaIntEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_INT_CLR Register
///
/// need_des
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **LP_ANA_BOD_MODE0_INT_CLR** `[31]` (WO)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_CLR** `[30]` (WO)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_UPVOLTAGE_INT_CLR** `[29]` (WO)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_CLR** `[28]` (WO)
///
///   need_des
///
/// - **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_CLR** `[27]` (WO)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaIntClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaIntClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaIntClr {
    // ======== Field Constants ========

    /// **LP_ANA_BOD_MODE0_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_BOD_MODE0_INT_CLR_OFFSET: u32 = 31;
    pub const LP_ANA_BOD_MODE0_INT_CLR_MASK: u32 = 0x80000000;
    pub const LP_ANA_BOD_MODE0_INT_CLR: u32 = 1 << 31;

    /// **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_INT_CLR_OFFSET: u32 = 30;
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_INT_CLR_MASK: u32 = 0x40000000;
    pub const LP_ANA_VDDBAT_UNDERVOLTAGE_INT_CLR: u32 = 1 << 30;

    /// **LP_ANA_VDDBAT_UPVOLTAGE_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_ANA_VDDBAT_UPVOLTAGE_INT_CLR_OFFSET: u32 = 29;
    pub const LP_ANA_VDDBAT_UPVOLTAGE_INT_CLR_MASK: u32 = 0x20000000;
    pub const LP_ANA_VDDBAT_UPVOLTAGE_INT_CLR: u32 = 1 << 29;

    /// **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_CLR_OFFSET: u32 = 28;
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_CLR_MASK: u32 = 0x10000000;
    pub const LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_CLR: u32 = 1 << 28;

    /// **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_CLR_OFFSET: u32 = 27;
    pub const LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_CLR_MASK: u32 = 0x08000000;
    pub const LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_CLR: u32 = 1 << 27;

    // ======== Field Accessors ========

    /// Trigger **LP_ANA_BOD_MODE0_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_ana_bod_mode0_int_clr(&self) {
        self.value.write_bits(Self::LP_ANA_BOD_MODE0_INT_CLR_MASK);
    }

    /// Trigger **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_ana_vddbat_undervoltage_int_clr(&self) {
        self.value
            .write_bits(Self::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_CLR_MASK);
    }

    /// Trigger **LP_ANA_VDDBAT_UPVOLTAGE_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_ana_vddbat_upvoltage_int_clr(&self) {
        self.value
            .write_bits(Self::LP_ANA_VDDBAT_UPVOLTAGE_INT_CLR_MASK);
    }

    /// Trigger **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_ana_vddbat_charge_undervoltage_int_clr(&self) {
        self.value
            .write_bits(Self::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_CLR_MASK);
    }

    /// Trigger **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_ana_vddbat_charge_upvoltage_int_clr(&self) {
        self.value
            .write_bits(Self::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_CLR_MASK);
    }
}

/// LP_ANA_INT_CLR value type for batch read/modify/write operations
///
/// Use `lpanaintclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanaintclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaIntClrVal {
    bits: u32,
}

impl LpAnaIntClrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **LP_ANA_BOD_MODE0_INT_CLR** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode0_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntClr::LP_ANA_BOD_MODE0_INT_CLR_MASK
            } else {
                self.bits & !LpAnaIntClr::LP_ANA_BOD_MODE0_INT_CLR_MASK
            },
        }
    }

    /// Set **LP_ANA_VDDBAT_UNDERVOLTAGE_INT_CLR** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_undervoltage_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntClr::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_CLR_MASK
            } else {
                self.bits & !LpAnaIntClr::LP_ANA_VDDBAT_UNDERVOLTAGE_INT_CLR_MASK
            },
        }
    }

    /// Set **LP_ANA_VDDBAT_UPVOLTAGE_INT_CLR** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_upvoltage_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntClr::LP_ANA_VDDBAT_UPVOLTAGE_INT_CLR_MASK
            } else {
                self.bits & !LpAnaIntClr::LP_ANA_VDDBAT_UPVOLTAGE_INT_CLR_MASK
            },
        }
    }

    /// Set **LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_CLR** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_charge_undervoltage_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntClr::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_CLR_MASK
            } else {
                self.bits & !LpAnaIntClr::LP_ANA_VDDBAT_CHARGE_UNDERVOLTAGE_INT_CLR_MASK
            },
        }
    }

    /// Set **LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_CLR** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_vddbat_charge_upvoltage_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaIntClr::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_CLR_MASK
            } else {
                self.bits & !LpAnaIntClr::LP_ANA_VDDBAT_CHARGE_UPVOLTAGE_INT_CLR_MASK
            },
        }
    }
}

impl Default for LpAnaIntClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaIntClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaIntClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// LP_ANA_LP_INT_RAW Register
///
/// need_des
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **LP_ANA_BOD_MODE0_LP_INT_RAW** `[31]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaLpIntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaLpIntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaLpIntRaw {
    // ======== Field Constants ========

    /// **LP_ANA_BOD_MODE0_LP_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_BOD_MODE0_LP_INT_RAW_OFFSET: u32 = 31;
    pub const LP_ANA_BOD_MODE0_LP_INT_RAW_MASK: u32 = 0x80000000;
    pub const LP_ANA_BOD_MODE0_LP_INT_RAW: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **LP_ANA_BOD_MODE0_LP_INT_RAW** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode0_lp_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE0_LP_INT_RAW_MASK) != 0
    }

    /// Set LP_ANA_BOD_MODE0_LP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_bod_mode0_lp_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_BOD_MODE0_LP_INT_RAW_MASK);
    }

    /// Clear LP_ANA_BOD_MODE0_LP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_bod_mode0_lp_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_BOD_MODE0_LP_INT_RAW_MASK);
    }
}

/// LP_ANA_LP_INT_RAW value type for batch read/modify/write operations
///
/// Use `lpanalpintraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanalpintraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaLpIntRawVal {
    bits: u32,
}

impl LpAnaLpIntRawVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_BOD_MODE0_LP_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_lp_int_raw(&self) -> bool {
        (self.bits & LpAnaLpIntRaw::LP_ANA_BOD_MODE0_LP_INT_RAW_MASK) != 0
    }

    /// Set **LP_ANA_BOD_MODE0_LP_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode0_lp_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaLpIntRaw::LP_ANA_BOD_MODE0_LP_INT_RAW_MASK
            } else {
                self.bits & !LpAnaLpIntRaw::LP_ANA_BOD_MODE0_LP_INT_RAW_MASK
            },
        }
    }
}

impl Default for LpAnaLpIntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaLpIntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaLpIntRawVal {
        LpAnaLpIntRawVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaLpIntRawVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaLpIntRawVal) -> LpAnaLpIntRawVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_LP_INT_ST Register
///
/// need_des
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **LP_ANA_BOD_MODE0_LP_INT_ST** `[31]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaLpIntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaLpIntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaLpIntSt {
    // ======== Field Constants ========

    /// **LP_ANA_BOD_MODE0_LP_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_BOD_MODE0_LP_INT_ST_OFFSET: u32 = 31;
    pub const LP_ANA_BOD_MODE0_LP_INT_ST_MASK: u32 = 0x80000000;
    pub const LP_ANA_BOD_MODE0_LP_INT_ST: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **LP_ANA_BOD_MODE0_LP_INT_ST** `[31]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode0_lp_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE0_LP_INT_ST_MASK) != 0
    }
}

/// LP_ANA_LP_INT_ST value type for batch read/modify/write operations
///
/// Use `lpanalpintst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanalpintst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaLpIntStVal {
    bits: u32,
}

impl LpAnaLpIntStVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_BOD_MODE0_LP_INT_ST** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_lp_int_st(&self) -> bool {
        (self.bits & LpAnaLpIntSt::LP_ANA_BOD_MODE0_LP_INT_ST_MASK) != 0
    }
}

impl Default for LpAnaLpIntStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaLpIntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaLpIntStVal {
        LpAnaLpIntStVal::from_bits(self.value.read_bits())
    }
}

/// LP_ANA_LP_INT_ENA Register
///
/// need_des
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **LP_ANA_BOD_MODE0_LP_INT_ENA** `[31]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaLpIntEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaLpIntEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaLpIntEna {
    // ======== Field Constants ========

    /// **LP_ANA_BOD_MODE0_LP_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_BOD_MODE0_LP_INT_ENA_OFFSET: u32 = 31;
    pub const LP_ANA_BOD_MODE0_LP_INT_ENA_MASK: u32 = 0x80000000;
    pub const LP_ANA_BOD_MODE0_LP_INT_ENA: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **LP_ANA_BOD_MODE0_LP_INT_ENA** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_bod_mode0_lp_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_BOD_MODE0_LP_INT_ENA_MASK) != 0
    }

    /// Set LP_ANA_BOD_MODE0_LP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_bod_mode0_lp_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_BOD_MODE0_LP_INT_ENA_MASK);
    }

    /// Clear LP_ANA_BOD_MODE0_LP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_bod_mode0_lp_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_BOD_MODE0_LP_INT_ENA_MASK);
    }
}

/// LP_ANA_LP_INT_ENA value type for batch read/modify/write operations
///
/// Use `lpanalpintena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanalpintena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaLpIntEnaVal {
    bits: u32,
}

impl LpAnaLpIntEnaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_BOD_MODE0_LP_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_lp_int_ena(&self) -> bool {
        (self.bits & LpAnaLpIntEna::LP_ANA_BOD_MODE0_LP_INT_ENA_MASK) != 0
    }

    /// Set **LP_ANA_BOD_MODE0_LP_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode0_lp_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaLpIntEna::LP_ANA_BOD_MODE0_LP_INT_ENA_MASK
            } else {
                self.bits & !LpAnaLpIntEna::LP_ANA_BOD_MODE0_LP_INT_ENA_MASK
            },
        }
    }
}

impl Default for LpAnaLpIntEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaLpIntEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaLpIntEnaVal {
        LpAnaLpIntEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaLpIntEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaLpIntEnaVal) -> LpAnaLpIntEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_LP_INT_CLR Register
///
/// need_des
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **LP_ANA_BOD_MODE0_LP_INT_CLR** `[31]` (WO)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaLpIntClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaLpIntClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaLpIntClr {
    // ======== Field Constants ========

    /// **LP_ANA_BOD_MODE0_LP_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_BOD_MODE0_LP_INT_CLR_OFFSET: u32 = 31;
    pub const LP_ANA_BOD_MODE0_LP_INT_CLR_MASK: u32 = 0x80000000;
    pub const LP_ANA_BOD_MODE0_LP_INT_CLR: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// Trigger **LP_ANA_BOD_MODE0_LP_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_ana_bod_mode0_lp_int_clr(&self) {
        self.value
            .write_bits(Self::LP_ANA_BOD_MODE0_LP_INT_CLR_MASK);
    }
}

/// LP_ANA_LP_INT_CLR value type for batch read/modify/write operations
///
/// Use `lpanalpintclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanalpintclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaLpIntClrVal {
    bits: u32,
}

impl LpAnaLpIntClrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **LP_ANA_BOD_MODE0_LP_INT_CLR** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_bod_mode0_lp_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaLpIntClr::LP_ANA_BOD_MODE0_LP_INT_CLR_MASK
            } else {
                self.bits & !LpAnaLpIntClr::LP_ANA_BOD_MODE0_LP_INT_CLR_MASK
            },
        }
    }
}

impl Default for LpAnaLpIntClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaLpIntClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaLpIntClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// LP_ANA_TOUCH_APPROACH_WORK_MEAS_NUM Register
///
/// need_des
///
/// Address offset: 0x00fc
/// Reset value: 0x06419064
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_APPROACH_MEAS_NUM0** `[29:20]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_APPROACH_MEAS_NUM1** `[19:10]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_APPROACH_MEAS_NUM2** `[9:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchApproachWorkMeasNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchApproachWorkMeasNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchApproachWorkMeasNum {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_APPROACH_MEAS_NUM0**
    ///
    /// need_des
    ///
    /// Bits `[29:20]`, mask `0x3ff00000`
    pub const LP_ANA_TOUCH_APPROACH_MEAS_NUM0_OFFSET: u32 = 20;
    pub const LP_ANA_TOUCH_APPROACH_MEAS_NUM0_MASK: u32 = 0x3ff00000;

    /// **LP_ANA_TOUCH_APPROACH_MEAS_NUM1**
    ///
    /// need_des
    ///
    /// Bits `[19:10]`, mask `0x000ffc00`
    pub const LP_ANA_TOUCH_APPROACH_MEAS_NUM1_OFFSET: u32 = 10;
    pub const LP_ANA_TOUCH_APPROACH_MEAS_NUM1_MASK: u32 = 0x000ffc00;

    /// **LP_ANA_TOUCH_APPROACH_MEAS_NUM2**
    ///
    /// need_des
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const LP_ANA_TOUCH_APPROACH_MEAS_NUM2_OFFSET: u32 = 0;
    pub const LP_ANA_TOUCH_APPROACH_MEAS_NUM2_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_APPROACH_MEAS_NUM0** `[29:20]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_approach_meas_num0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM0_MASK)
            >> Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_APPROACH_MEAS_NUM0** `[29:20]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_approach_meas_num0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM0_MASK)
                | ((value << Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM0_OFFSET)
                    & Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM0_MASK)
        });
    }

    /// **LP_ANA_TOUCH_APPROACH_MEAS_NUM1** `[19:10]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_approach_meas_num1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM1_MASK)
            >> Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_APPROACH_MEAS_NUM1** `[19:10]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_approach_meas_num1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM1_MASK)
                | ((value << Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM1_OFFSET)
                    & Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM1_MASK)
        });
    }

    /// **LP_ANA_TOUCH_APPROACH_MEAS_NUM2** `[9:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_approach_meas_num2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM2_MASK)
            >> Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_APPROACH_MEAS_NUM2** `[9:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_approach_meas_num2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM2_MASK)
                | ((value << Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM2_OFFSET)
                    & Self::LP_ANA_TOUCH_APPROACH_MEAS_NUM2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_APPROACH_WORK_MEAS_NUM value type for batch read/modify/write operations
///
/// Use `lpanatouchapproachworkmeasnum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchapproachworkmeasnum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchApproachWorkMeasNumVal {
    bits: u32,
}

impl LpAnaTouchApproachWorkMeasNumVal {
    /// Create from raw bits
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
        Self { bits: 0x06419064 }
    }

    /// Read **LP_ANA_TOUCH_APPROACH_MEAS_NUM0** `[29:20]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_approach_meas_num0(&self) -> u32 {
        (self.bits & LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM0_MASK)
            >> LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM0_OFFSET
    }

    /// Read **LP_ANA_TOUCH_APPROACH_MEAS_NUM1** `[19:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_approach_meas_num1(&self) -> u32 {
        (self.bits & LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM1_MASK)
            >> LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM1_OFFSET
    }

    /// Read **LP_ANA_TOUCH_APPROACH_MEAS_NUM2** `[9:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_approach_meas_num2(&self) -> u32 {
        (self.bits & LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM2_MASK)
            >> LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_APPROACH_MEAS_NUM0** `[29:20]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_approach_meas_num0(self, value: u32) -> Self {
        Self {
            bits: (self.bits
                & !LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM0_MASK)
                | ((value
                    << LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM0_OFFSET)
                    & LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM0_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_APPROACH_MEAS_NUM1** `[19:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_approach_meas_num1(self, value: u32) -> Self {
        Self {
            bits: (self.bits
                & !LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM1_MASK)
                | ((value
                    << LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM1_OFFSET)
                    & LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM1_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_APPROACH_MEAS_NUM2** `[9:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_approach_meas_num2(self, value: u32) -> Self {
        Self {
            bits: (self.bits
                & !LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM2_MASK)
                | ((value
                    << LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM2_OFFSET)
                    & LpAnaTouchApproachWorkMeasNum::LP_ANA_TOUCH_APPROACH_MEAS_NUM2_MASK),
        }
    }
}

impl Default for LpAnaTouchApproachWorkMeasNumVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaTouchApproachWorkMeasNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchApproachWorkMeasNumVal {
        LpAnaTouchApproachWorkMeasNumVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchApproachWorkMeasNumVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<
        F: FnOnce(LpAnaTouchApproachWorkMeasNumVal) -> LpAnaTouchApproachWorkMeasNumVal,
    >(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_SCAN_CTRL1 Register
///
/// need_des
///
/// Address offset: 0x0100
/// Reset value: 0x00080000
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_XPD_WAIT** `[31:17]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_SCAN_PAD_MAP** `[16:2]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_INACTIVE_CONNECTION** `[1]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_SHIELD_PAD_EN** `[0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchScanCtrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchScanCtrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchScanCtrl1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_XPD_WAIT**
    ///
    /// need_des
    ///
    /// Bits `[31:17]`, mask `0xfffe0000`
    pub const LP_ANA_TOUCH_XPD_WAIT_OFFSET: u32 = 17;
    pub const LP_ANA_TOUCH_XPD_WAIT_MASK: u32 = 0xfffe0000;

    /// **LP_ANA_TOUCH_SCAN_PAD_MAP**
    ///
    /// need_des
    ///
    /// Bits `[16:2]`, mask `0x0001fffc`
    pub const LP_ANA_TOUCH_SCAN_PAD_MAP_OFFSET: u32 = 2;
    pub const LP_ANA_TOUCH_SCAN_PAD_MAP_MASK: u32 = 0x0001fffc;

    /// **LP_ANA_TOUCH_INACTIVE_CONNECTION**
    ///
    /// need_des
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const LP_ANA_TOUCH_INACTIVE_CONNECTION_OFFSET: u32 = 1;
    pub const LP_ANA_TOUCH_INACTIVE_CONNECTION_MASK: u32 = 0x00000002;
    pub const LP_ANA_TOUCH_INACTIVE_CONNECTION: u32 = 1 << 1;

    /// **LP_ANA_TOUCH_SHIELD_PAD_EN**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_ANA_TOUCH_SHIELD_PAD_EN_OFFSET: u32 = 0;
    pub const LP_ANA_TOUCH_SHIELD_PAD_EN_MASK: u32 = 0x00000001;
    pub const LP_ANA_TOUCH_SHIELD_PAD_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_XPD_WAIT** `[31:17]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_xpd_wait(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_XPD_WAIT_MASK)
            >> Self::LP_ANA_TOUCH_XPD_WAIT_OFFSET
    }

    /// Write **LP_ANA_TOUCH_XPD_WAIT** `[31:17]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_xpd_wait(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_XPD_WAIT_MASK)
                | ((value << Self::LP_ANA_TOUCH_XPD_WAIT_OFFSET) & Self::LP_ANA_TOUCH_XPD_WAIT_MASK)
        });
    }

    /// **LP_ANA_TOUCH_SCAN_PAD_MAP** `[16:2]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_scan_pad_map(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_SCAN_PAD_MAP_MASK)
            >> Self::LP_ANA_TOUCH_SCAN_PAD_MAP_OFFSET
    }

    /// Write **LP_ANA_TOUCH_SCAN_PAD_MAP** `[16:2]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_scan_pad_map(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_SCAN_PAD_MAP_MASK)
                | ((value << Self::LP_ANA_TOUCH_SCAN_PAD_MAP_OFFSET)
                    & Self::LP_ANA_TOUCH_SCAN_PAD_MAP_MASK)
        });
    }

    /// **LP_ANA_TOUCH_INACTIVE_CONNECTION** `[1]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_inactive_connection(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_INACTIVE_CONNECTION_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_INACTIVE_CONNECTION bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_inactive_connection(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_INACTIVE_CONNECTION_MASK);
    }

    /// Clear LP_ANA_TOUCH_INACTIVE_CONNECTION bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_inactive_connection(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_INACTIVE_CONNECTION_MASK);
    }

    /// **LP_ANA_TOUCH_SHIELD_PAD_EN** `[0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_shield_pad_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_SHIELD_PAD_EN_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_SHIELD_PAD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_shield_pad_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_SHIELD_PAD_EN_MASK);
    }

    /// Clear LP_ANA_TOUCH_SHIELD_PAD_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_shield_pad_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_SHIELD_PAD_EN_MASK);
    }
}

/// LP_ANA_TOUCH_SCAN_CTRL1 value type for batch read/modify/write operations
///
/// Use `lpanatouchscanctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchscanctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchScanCtrl1Val {
    bits: u32,
}

impl LpAnaTouchScanCtrl1Val {
    /// Create from raw bits
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

    /// Read **LP_ANA_TOUCH_XPD_WAIT** `[31:17]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_xpd_wait(&self) -> u32 {
        (self.bits & LpAnaTouchScanCtrl1::LP_ANA_TOUCH_XPD_WAIT_MASK)
            >> LpAnaTouchScanCtrl1::LP_ANA_TOUCH_XPD_WAIT_OFFSET
    }

    /// Read **LP_ANA_TOUCH_SCAN_PAD_MAP** `[16:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_scan_pad_map(&self) -> u32 {
        (self.bits & LpAnaTouchScanCtrl1::LP_ANA_TOUCH_SCAN_PAD_MAP_MASK)
            >> LpAnaTouchScanCtrl1::LP_ANA_TOUCH_SCAN_PAD_MAP_OFFSET
    }

    /// Read **LP_ANA_TOUCH_INACTIVE_CONNECTION** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_inactive_connection(&self) -> bool {
        (self.bits & LpAnaTouchScanCtrl1::LP_ANA_TOUCH_INACTIVE_CONNECTION_MASK) != 0
    }

    /// Read **LP_ANA_TOUCH_SHIELD_PAD_EN** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_shield_pad_en(&self) -> bool {
        (self.bits & LpAnaTouchScanCtrl1::LP_ANA_TOUCH_SHIELD_PAD_EN_MASK) != 0
    }

    /// Set **LP_ANA_TOUCH_XPD_WAIT** `[31:17]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_xpd_wait(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchScanCtrl1::LP_ANA_TOUCH_XPD_WAIT_MASK)
                | ((value << LpAnaTouchScanCtrl1::LP_ANA_TOUCH_XPD_WAIT_OFFSET)
                    & LpAnaTouchScanCtrl1::LP_ANA_TOUCH_XPD_WAIT_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_SCAN_PAD_MAP** `[16:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_scan_pad_map(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchScanCtrl1::LP_ANA_TOUCH_SCAN_PAD_MAP_MASK)
                | ((value << LpAnaTouchScanCtrl1::LP_ANA_TOUCH_SCAN_PAD_MAP_OFFSET)
                    & LpAnaTouchScanCtrl1::LP_ANA_TOUCH_SCAN_PAD_MAP_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_INACTIVE_CONNECTION** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_inactive_connection(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchScanCtrl1::LP_ANA_TOUCH_INACTIVE_CONNECTION_MASK
            } else {
                self.bits & !LpAnaTouchScanCtrl1::LP_ANA_TOUCH_INACTIVE_CONNECTION_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_SHIELD_PAD_EN** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_shield_pad_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchScanCtrl1::LP_ANA_TOUCH_SHIELD_PAD_EN_MASK
            } else {
                self.bits & !LpAnaTouchScanCtrl1::LP_ANA_TOUCH_SHIELD_PAD_EN_MASK
            },
        }
    }
}

impl Default for LpAnaTouchScanCtrl1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaTouchScanCtrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchScanCtrl1Val {
        LpAnaTouchScanCtrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchScanCtrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchScanCtrl1Val) -> LpAnaTouchScanCtrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_SCAN_CTRL2 Register
///
/// need_des
///
/// Address offset: 0x0104
/// Reset value: 0x37bfffc0
///
/// ## Fields
///
/// - **LP_ANA_FREQ_SCAN_CNT_LIMIT** `[29:28]` (RW)
///
///   need_des
///
/// - **LP_ANA_FREQ_SCAN_EN** `[27]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_OUT_RING** `[26:23]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_TIMEOUT_EN** `[22]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_TIMEOUT_NUM** `[21:6]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchScanCtrl2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchScanCtrl2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchScanCtrl2 {
    // ======== Field Constants ========

    /// **LP_ANA_FREQ_SCAN_CNT_LIMIT**
    ///
    /// need_des
    ///
    /// Bits `[29:28]`, mask `0x30000000`
    pub const LP_ANA_FREQ_SCAN_CNT_LIMIT_OFFSET: u32 = 28;
    pub const LP_ANA_FREQ_SCAN_CNT_LIMIT_MASK: u32 = 0x30000000;

    /// **LP_ANA_FREQ_SCAN_EN**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LP_ANA_FREQ_SCAN_EN_OFFSET: u32 = 27;
    pub const LP_ANA_FREQ_SCAN_EN_MASK: u32 = 0x08000000;
    pub const LP_ANA_FREQ_SCAN_EN: u32 = 1 << 27;

    /// **LP_ANA_TOUCH_OUT_RING**
    ///
    /// need_des
    ///
    /// Bits `[26:23]`, mask `0x07800000`
    pub const LP_ANA_TOUCH_OUT_RING_OFFSET: u32 = 23;
    pub const LP_ANA_TOUCH_OUT_RING_MASK: u32 = 0x07800000;

    /// **LP_ANA_TOUCH_TIMEOUT_EN**
    ///
    /// need_des
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const LP_ANA_TOUCH_TIMEOUT_EN_OFFSET: u32 = 22;
    pub const LP_ANA_TOUCH_TIMEOUT_EN_MASK: u32 = 0x00400000;
    pub const LP_ANA_TOUCH_TIMEOUT_EN: u32 = 1 << 22;

    /// **LP_ANA_TOUCH_TIMEOUT_NUM**
    ///
    /// need_des
    ///
    /// Bits `[21:6]`, mask `0x003fffc0`
    pub const LP_ANA_TOUCH_TIMEOUT_NUM_OFFSET: u32 = 6;
    pub const LP_ANA_TOUCH_TIMEOUT_NUM_MASK: u32 = 0x003fffc0;

    // ======== Field Accessors ========

    /// **LP_ANA_FREQ_SCAN_CNT_LIMIT** `[29:28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_freq_scan_cnt_limit(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_FREQ_SCAN_CNT_LIMIT_MASK)
            >> Self::LP_ANA_FREQ_SCAN_CNT_LIMIT_OFFSET
    }

    /// Write **LP_ANA_FREQ_SCAN_CNT_LIMIT** `[29:28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_freq_scan_cnt_limit(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_FREQ_SCAN_CNT_LIMIT_MASK)
                | ((value << Self::LP_ANA_FREQ_SCAN_CNT_LIMIT_OFFSET)
                    & Self::LP_ANA_FREQ_SCAN_CNT_LIMIT_MASK)
        });
    }

    /// **LP_ANA_FREQ_SCAN_EN** `[27]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_freq_scan_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_FREQ_SCAN_EN_MASK) != 0
    }

    /// Set LP_ANA_FREQ_SCAN_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_freq_scan_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_FREQ_SCAN_EN_MASK);
    }

    /// Clear LP_ANA_FREQ_SCAN_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_freq_scan_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_FREQ_SCAN_EN_MASK);
    }

    /// **LP_ANA_TOUCH_OUT_RING** `[26:23]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_out_ring(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_OUT_RING_MASK)
            >> Self::LP_ANA_TOUCH_OUT_RING_OFFSET
    }

    /// Write **LP_ANA_TOUCH_OUT_RING** `[26:23]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_out_ring(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_OUT_RING_MASK)
                | ((value << Self::LP_ANA_TOUCH_OUT_RING_OFFSET) & Self::LP_ANA_TOUCH_OUT_RING_MASK)
        });
    }

    /// **LP_ANA_TOUCH_TIMEOUT_EN** `[22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_timeout_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_TIMEOUT_EN_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_TIMEOUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_timeout_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_TIMEOUT_EN_MASK);
    }

    /// Clear LP_ANA_TOUCH_TIMEOUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_timeout_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_TIMEOUT_EN_MASK);
    }

    /// **LP_ANA_TOUCH_TIMEOUT_NUM** `[21:6]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_timeout_num(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_TIMEOUT_NUM_MASK)
            >> Self::LP_ANA_TOUCH_TIMEOUT_NUM_OFFSET
    }

    /// Write **LP_ANA_TOUCH_TIMEOUT_NUM** `[21:6]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_timeout_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_TIMEOUT_NUM_MASK)
                | ((value << Self::LP_ANA_TOUCH_TIMEOUT_NUM_OFFSET)
                    & Self::LP_ANA_TOUCH_TIMEOUT_NUM_MASK)
        });
    }
}

/// LP_ANA_TOUCH_SCAN_CTRL2 value type for batch read/modify/write operations
///
/// Use `lpanatouchscanctrl2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchscanctrl2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchScanCtrl2Val {
    bits: u32,
}

impl LpAnaTouchScanCtrl2Val {
    /// Create from raw bits
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
        Self { bits: 0x37bfffc0 }
    }

    /// Read **LP_ANA_FREQ_SCAN_CNT_LIMIT** `[29:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_freq_scan_cnt_limit(&self) -> u32 {
        (self.bits & LpAnaTouchScanCtrl2::LP_ANA_FREQ_SCAN_CNT_LIMIT_MASK)
            >> LpAnaTouchScanCtrl2::LP_ANA_FREQ_SCAN_CNT_LIMIT_OFFSET
    }

    /// Read **LP_ANA_FREQ_SCAN_EN** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_freq_scan_en(&self) -> bool {
        (self.bits & LpAnaTouchScanCtrl2::LP_ANA_FREQ_SCAN_EN_MASK) != 0
    }

    /// Read **LP_ANA_TOUCH_OUT_RING** `[26:23]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_out_ring(&self) -> u32 {
        (self.bits & LpAnaTouchScanCtrl2::LP_ANA_TOUCH_OUT_RING_MASK)
            >> LpAnaTouchScanCtrl2::LP_ANA_TOUCH_OUT_RING_OFFSET
    }

    /// Read **LP_ANA_TOUCH_TIMEOUT_EN** `[22:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_timeout_en(&self) -> bool {
        (self.bits & LpAnaTouchScanCtrl2::LP_ANA_TOUCH_TIMEOUT_EN_MASK) != 0
    }

    /// Read **LP_ANA_TOUCH_TIMEOUT_NUM** `[21:6]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_timeout_num(&self) -> u32 {
        (self.bits & LpAnaTouchScanCtrl2::LP_ANA_TOUCH_TIMEOUT_NUM_MASK)
            >> LpAnaTouchScanCtrl2::LP_ANA_TOUCH_TIMEOUT_NUM_OFFSET
    }

    /// Set **LP_ANA_FREQ_SCAN_CNT_LIMIT** `[29:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_freq_scan_cnt_limit(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchScanCtrl2::LP_ANA_FREQ_SCAN_CNT_LIMIT_MASK)
                | ((value << LpAnaTouchScanCtrl2::LP_ANA_FREQ_SCAN_CNT_LIMIT_OFFSET)
                    & LpAnaTouchScanCtrl2::LP_ANA_FREQ_SCAN_CNT_LIMIT_MASK),
        }
    }

    /// Set **LP_ANA_FREQ_SCAN_EN** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_freq_scan_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchScanCtrl2::LP_ANA_FREQ_SCAN_EN_MASK
            } else {
                self.bits & !LpAnaTouchScanCtrl2::LP_ANA_FREQ_SCAN_EN_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_OUT_RING** `[26:23]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_out_ring(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchScanCtrl2::LP_ANA_TOUCH_OUT_RING_MASK)
                | ((value << LpAnaTouchScanCtrl2::LP_ANA_TOUCH_OUT_RING_OFFSET)
                    & LpAnaTouchScanCtrl2::LP_ANA_TOUCH_OUT_RING_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_TIMEOUT_EN** `[22:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_timeout_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchScanCtrl2::LP_ANA_TOUCH_TIMEOUT_EN_MASK
            } else {
                self.bits & !LpAnaTouchScanCtrl2::LP_ANA_TOUCH_TIMEOUT_EN_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_TIMEOUT_NUM** `[21:6]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_timeout_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchScanCtrl2::LP_ANA_TOUCH_TIMEOUT_NUM_MASK)
                | ((value << LpAnaTouchScanCtrl2::LP_ANA_TOUCH_TIMEOUT_NUM_OFFSET)
                    & LpAnaTouchScanCtrl2::LP_ANA_TOUCH_TIMEOUT_NUM_MASK),
        }
    }
}

impl Default for LpAnaTouchScanCtrl2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaTouchScanCtrl2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchScanCtrl2Val {
        LpAnaTouchScanCtrl2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchScanCtrl2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchScanCtrl2Val) -> LpAnaTouchScanCtrl2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_WORK Register
///
/// need_des
///
/// Address offset: 0x0108
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_OUT_GATE** `[27]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_OUT_RESET** `[26]` (WO)
///
///   need_des
///
/// - **LP_ANA_TOUCH_OUT_SEL** `[25]` (RW)
///
///   need_des
///
/// - **LP_ANA_DIV_NUM0** `[24:22]` (RW)
///
///   need_des
///
/// - **LP_ANA_DIV_NUM1** `[21:19]` (RW)
///
///   need_des
///
/// - **LP_ANA_DIV_NUM2** `[18:16]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchWork {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchWork {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchWork {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_OUT_GATE**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LP_ANA_TOUCH_OUT_GATE_OFFSET: u32 = 27;
    pub const LP_ANA_TOUCH_OUT_GATE_MASK: u32 = 0x08000000;
    pub const LP_ANA_TOUCH_OUT_GATE: u32 = 1 << 27;

    /// **LP_ANA_TOUCH_OUT_RESET**
    ///
    /// need_des
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const LP_ANA_TOUCH_OUT_RESET_OFFSET: u32 = 26;
    pub const LP_ANA_TOUCH_OUT_RESET_MASK: u32 = 0x04000000;
    pub const LP_ANA_TOUCH_OUT_RESET: u32 = 1 << 26;

    /// **LP_ANA_TOUCH_OUT_SEL**
    ///
    /// need_des
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const LP_ANA_TOUCH_OUT_SEL_OFFSET: u32 = 25;
    pub const LP_ANA_TOUCH_OUT_SEL_MASK: u32 = 0x02000000;
    pub const LP_ANA_TOUCH_OUT_SEL: u32 = 1 << 25;

    /// **LP_ANA_DIV_NUM0**
    ///
    /// need_des
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const LP_ANA_DIV_NUM0_OFFSET: u32 = 22;
    pub const LP_ANA_DIV_NUM0_MASK: u32 = 0x01c00000;

    /// **LP_ANA_DIV_NUM1**
    ///
    /// need_des
    ///
    /// Bits `[21:19]`, mask `0x00380000`
    pub const LP_ANA_DIV_NUM1_OFFSET: u32 = 19;
    pub const LP_ANA_DIV_NUM1_MASK: u32 = 0x00380000;

    /// **LP_ANA_DIV_NUM2**
    ///
    /// need_des
    ///
    /// Bits `[18:16]`, mask `0x00070000`
    pub const LP_ANA_DIV_NUM2_OFFSET: u32 = 16;
    pub const LP_ANA_DIV_NUM2_MASK: u32 = 0x00070000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_OUT_GATE** `[27]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_out_gate(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_OUT_GATE_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_OUT_GATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_out_gate(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_OUT_GATE_MASK);
    }

    /// Clear LP_ANA_TOUCH_OUT_GATE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_out_gate(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_OUT_GATE_MASK);
    }

    /// Trigger **LP_ANA_TOUCH_OUT_RESET** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_ana_touch_out_reset(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_OUT_RESET_MASK);
    }

    /// **LP_ANA_TOUCH_OUT_SEL** `[25]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_out_sel(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_OUT_SEL_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_OUT_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_out_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_OUT_SEL_MASK);
    }

    /// Clear LP_ANA_TOUCH_OUT_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_out_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_OUT_SEL_MASK);
    }

    /// **LP_ANA_DIV_NUM0** `[24:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_div_num0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_DIV_NUM0_MASK) >> Self::LP_ANA_DIV_NUM0_OFFSET
    }

    /// Write **LP_ANA_DIV_NUM0** `[24:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_div_num0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_DIV_NUM0_MASK)
                | ((value << Self::LP_ANA_DIV_NUM0_OFFSET) & Self::LP_ANA_DIV_NUM0_MASK)
        });
    }

    /// **LP_ANA_DIV_NUM1** `[21:19]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_div_num1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_DIV_NUM1_MASK) >> Self::LP_ANA_DIV_NUM1_OFFSET
    }

    /// Write **LP_ANA_DIV_NUM1** `[21:19]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_div_num1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_DIV_NUM1_MASK)
                | ((value << Self::LP_ANA_DIV_NUM1_OFFSET) & Self::LP_ANA_DIV_NUM1_MASK)
        });
    }

    /// **LP_ANA_DIV_NUM2** `[18:16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_div_num2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_DIV_NUM2_MASK) >> Self::LP_ANA_DIV_NUM2_OFFSET
    }

    /// Write **LP_ANA_DIV_NUM2** `[18:16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_div_num2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_DIV_NUM2_MASK)
                | ((value << Self::LP_ANA_DIV_NUM2_OFFSET) & Self::LP_ANA_DIV_NUM2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_WORK value type for batch read/modify/write operations
///
/// Use `lpanatouchwork.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchwork.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchWorkVal {
    bits: u32,
}

impl LpAnaTouchWorkVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_OUT_GATE** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_out_gate(&self) -> bool {
        (self.bits & LpAnaTouchWork::LP_ANA_TOUCH_OUT_GATE_MASK) != 0
    }

    /// Read **LP_ANA_TOUCH_OUT_SEL** `[25:25]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_out_sel(&self) -> bool {
        (self.bits & LpAnaTouchWork::LP_ANA_TOUCH_OUT_SEL_MASK) != 0
    }

    /// Read **LP_ANA_DIV_NUM0** `[24:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_div_num0(&self) -> u32 {
        (self.bits & LpAnaTouchWork::LP_ANA_DIV_NUM0_MASK) >> LpAnaTouchWork::LP_ANA_DIV_NUM0_OFFSET
    }

    /// Read **LP_ANA_DIV_NUM1** `[21:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_div_num1(&self) -> u32 {
        (self.bits & LpAnaTouchWork::LP_ANA_DIV_NUM1_MASK) >> LpAnaTouchWork::LP_ANA_DIV_NUM1_OFFSET
    }

    /// Read **LP_ANA_DIV_NUM2** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_div_num2(&self) -> u32 {
        (self.bits & LpAnaTouchWork::LP_ANA_DIV_NUM2_MASK) >> LpAnaTouchWork::LP_ANA_DIV_NUM2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_OUT_GATE** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_out_gate(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchWork::LP_ANA_TOUCH_OUT_GATE_MASK
            } else {
                self.bits & !LpAnaTouchWork::LP_ANA_TOUCH_OUT_GATE_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_OUT_RESET** `[26:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_out_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchWork::LP_ANA_TOUCH_OUT_RESET_MASK
            } else {
                self.bits & !LpAnaTouchWork::LP_ANA_TOUCH_OUT_RESET_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_OUT_SEL** `[25:25]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_out_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchWork::LP_ANA_TOUCH_OUT_SEL_MASK
            } else {
                self.bits & !LpAnaTouchWork::LP_ANA_TOUCH_OUT_SEL_MASK
            },
        }
    }

    /// Set **LP_ANA_DIV_NUM0** `[24:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_div_num0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchWork::LP_ANA_DIV_NUM0_MASK)
                | ((value << LpAnaTouchWork::LP_ANA_DIV_NUM0_OFFSET)
                    & LpAnaTouchWork::LP_ANA_DIV_NUM0_MASK),
        }
    }

    /// Set **LP_ANA_DIV_NUM1** `[21:19]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_div_num1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchWork::LP_ANA_DIV_NUM1_MASK)
                | ((value << LpAnaTouchWork::LP_ANA_DIV_NUM1_OFFSET)
                    & LpAnaTouchWork::LP_ANA_DIV_NUM1_MASK),
        }
    }

    /// Set **LP_ANA_DIV_NUM2** `[18:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_div_num2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchWork::LP_ANA_DIV_NUM2_MASK)
                | ((value << LpAnaTouchWork::LP_ANA_DIV_NUM2_OFFSET)
                    & LpAnaTouchWork::LP_ANA_DIV_NUM2_MASK),
        }
    }
}

impl Default for LpAnaTouchWorkVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchWork {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchWorkVal {
        LpAnaTouchWorkVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchWorkVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchWorkVal) -> LpAnaTouchWorkVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_WORK_MEAS_NUM Register
///
/// need_des
///
/// Address offset: 0x010c
/// Reset value: 0x06419064
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_MEAS_NUM0** `[29:20]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_MEAS_NUM1** `[19:10]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_MEAS_NUM2** `[9:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchWorkMeasNum {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchWorkMeasNum {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchWorkMeasNum {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_MEAS_NUM0**
    ///
    /// need_des
    ///
    /// Bits `[29:20]`, mask `0x3ff00000`
    pub const LP_ANA_TOUCH_MEAS_NUM0_OFFSET: u32 = 20;
    pub const LP_ANA_TOUCH_MEAS_NUM0_MASK: u32 = 0x3ff00000;

    /// **LP_ANA_TOUCH_MEAS_NUM1**
    ///
    /// need_des
    ///
    /// Bits `[19:10]`, mask `0x000ffc00`
    pub const LP_ANA_TOUCH_MEAS_NUM1_OFFSET: u32 = 10;
    pub const LP_ANA_TOUCH_MEAS_NUM1_MASK: u32 = 0x000ffc00;

    /// **LP_ANA_TOUCH_MEAS_NUM2**
    ///
    /// need_des
    ///
    /// Bits `[9:0]`, mask `0x000003ff`
    pub const LP_ANA_TOUCH_MEAS_NUM2_OFFSET: u32 = 0;
    pub const LP_ANA_TOUCH_MEAS_NUM2_MASK: u32 = 0x000003ff;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_MEAS_NUM0** `[29:20]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_meas_num0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_MEAS_NUM0_MASK)
            >> Self::LP_ANA_TOUCH_MEAS_NUM0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_MEAS_NUM0** `[29:20]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_meas_num0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_MEAS_NUM0_MASK)
                | ((value << Self::LP_ANA_TOUCH_MEAS_NUM0_OFFSET)
                    & Self::LP_ANA_TOUCH_MEAS_NUM0_MASK)
        });
    }

    /// **LP_ANA_TOUCH_MEAS_NUM1** `[19:10]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_meas_num1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_MEAS_NUM1_MASK)
            >> Self::LP_ANA_TOUCH_MEAS_NUM1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_MEAS_NUM1** `[19:10]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_meas_num1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_MEAS_NUM1_MASK)
                | ((value << Self::LP_ANA_TOUCH_MEAS_NUM1_OFFSET)
                    & Self::LP_ANA_TOUCH_MEAS_NUM1_MASK)
        });
    }

    /// **LP_ANA_TOUCH_MEAS_NUM2** `[9:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_meas_num2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_MEAS_NUM2_MASK)
            >> Self::LP_ANA_TOUCH_MEAS_NUM2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_MEAS_NUM2** `[9:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_meas_num2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_MEAS_NUM2_MASK)
                | ((value << Self::LP_ANA_TOUCH_MEAS_NUM2_OFFSET)
                    & Self::LP_ANA_TOUCH_MEAS_NUM2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_WORK_MEAS_NUM value type for batch read/modify/write operations
///
/// Use `lpanatouchworkmeasnum.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchworkmeasnum.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchWorkMeasNumVal {
    bits: u32,
}

impl LpAnaTouchWorkMeasNumVal {
    /// Create from raw bits
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
        Self { bits: 0x06419064 }
    }

    /// Read **LP_ANA_TOUCH_MEAS_NUM0** `[29:20]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_meas_num0(&self) -> u32 {
        (self.bits & LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM0_MASK)
            >> LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM0_OFFSET
    }

    /// Read **LP_ANA_TOUCH_MEAS_NUM1** `[19:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_meas_num1(&self) -> u32 {
        (self.bits & LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM1_MASK)
            >> LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM1_OFFSET
    }

    /// Read **LP_ANA_TOUCH_MEAS_NUM2** `[9:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_meas_num2(&self) -> u32 {
        (self.bits & LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM2_MASK)
            >> LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_MEAS_NUM0** `[29:20]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_meas_num0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM0_MASK)
                | ((value << LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM0_OFFSET)
                    & LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM0_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_MEAS_NUM1** `[19:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_meas_num1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM1_MASK)
                | ((value << LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM1_OFFSET)
                    & LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM1_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_MEAS_NUM2** `[9:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_meas_num2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM2_MASK)
                | ((value << LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM2_OFFSET)
                    & LpAnaTouchWorkMeasNum::LP_ANA_TOUCH_MEAS_NUM2_MASK),
        }
    }
}

impl Default for LpAnaTouchWorkMeasNumVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaTouchWorkMeasNum {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchWorkMeasNumVal {
        LpAnaTouchWorkMeasNumVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchWorkMeasNumVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchWorkMeasNumVal) -> LpAnaTouchWorkMeasNumVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_FILTER1 Register
///
/// need_des
///
/// Address offset: 0x0110
/// Reset value: 0x6a0a0200
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_DEBOUNCE_LIMIT** `[31:29]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_APPROACH_LIMIT** `[28:21]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_NEG_NOISE_LIMIT** `[20:17]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FILTER_EN** `[16]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FILTER_MODE** `[15:13]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_JITTER_STEP** `[12:9]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_SMOOTH_LVL** `[8:7]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_NOISE_THRES** `[6:5]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_NEG_NOISE_THRES** `[4:3]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_HYSTERESIS** `[2:1]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN** `[0]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchFilter1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchFilter1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchFilter1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_DEBOUNCE_LIMIT**
    ///
    /// need_des
    ///
    /// Bits `[31:29]`, mask `0xe0000000`
    pub const LP_ANA_TOUCH_DEBOUNCE_LIMIT_OFFSET: u32 = 29;
    pub const LP_ANA_TOUCH_DEBOUNCE_LIMIT_MASK: u32 = 0xe0000000;

    /// **LP_ANA_TOUCH_APPROACH_LIMIT**
    ///
    /// need_des
    ///
    /// Bits `[28:21]`, mask `0x1fe00000`
    pub const LP_ANA_TOUCH_APPROACH_LIMIT_OFFSET: u32 = 21;
    pub const LP_ANA_TOUCH_APPROACH_LIMIT_MASK: u32 = 0x1fe00000;

    /// **LP_ANA_TOUCH_NEG_NOISE_LIMIT**
    ///
    /// need_des
    ///
    /// Bits `[20:17]`, mask `0x001e0000`
    pub const LP_ANA_TOUCH_NEG_NOISE_LIMIT_OFFSET: u32 = 17;
    pub const LP_ANA_TOUCH_NEG_NOISE_LIMIT_MASK: u32 = 0x001e0000;

    /// **LP_ANA_TOUCH_FILTER_EN**
    ///
    /// need_des
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const LP_ANA_TOUCH_FILTER_EN_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_FILTER_EN_MASK: u32 = 0x00010000;
    pub const LP_ANA_TOUCH_FILTER_EN: u32 = 1 << 16;

    /// **LP_ANA_TOUCH_FILTER_MODE**
    ///
    /// need_des
    ///
    /// Bits `[15:13]`, mask `0x0000e000`
    pub const LP_ANA_TOUCH_FILTER_MODE_OFFSET: u32 = 13;
    pub const LP_ANA_TOUCH_FILTER_MODE_MASK: u32 = 0x0000e000;

    /// **LP_ANA_TOUCH_JITTER_STEP**
    ///
    /// need_des
    ///
    /// Bits `[12:9]`, mask `0x00001e00`
    pub const LP_ANA_TOUCH_JITTER_STEP_OFFSET: u32 = 9;
    pub const LP_ANA_TOUCH_JITTER_STEP_MASK: u32 = 0x00001e00;

    /// **LP_ANA_TOUCH_SMOOTH_LVL**
    ///
    /// need_des
    ///
    /// Bits `[8:7]`, mask `0x00000180`
    pub const LP_ANA_TOUCH_SMOOTH_LVL_OFFSET: u32 = 7;
    pub const LP_ANA_TOUCH_SMOOTH_LVL_MASK: u32 = 0x00000180;

    /// **LP_ANA_TOUCH_NOISE_THRES**
    ///
    /// need_des
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const LP_ANA_TOUCH_NOISE_THRES_OFFSET: u32 = 5;
    pub const LP_ANA_TOUCH_NOISE_THRES_MASK: u32 = 0x00000060;

    /// **LP_ANA_TOUCH_NEG_NOISE_THRES**
    ///
    /// need_des
    ///
    /// Bits `[4:3]`, mask `0x00000018`
    pub const LP_ANA_TOUCH_NEG_NOISE_THRES_OFFSET: u32 = 3;
    pub const LP_ANA_TOUCH_NEG_NOISE_THRES_MASK: u32 = 0x00000018;

    /// **LP_ANA_TOUCH_HYSTERESIS**
    ///
    /// need_des
    ///
    /// Bits `[2:1]`, mask `0x00000006`
    pub const LP_ANA_TOUCH_HYSTERESIS_OFFSET: u32 = 1;
    pub const LP_ANA_TOUCH_HYSTERESIS_MASK: u32 = 0x00000006;

    /// **LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN**
    ///
    /// Reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_OFFSET: u32 = 0;
    pub const LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_MASK: u32 = 0x00000001;
    pub const LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_DEBOUNCE_LIMIT** `[31:29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_debounce_limit(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_DEBOUNCE_LIMIT_MASK)
            >> Self::LP_ANA_TOUCH_DEBOUNCE_LIMIT_OFFSET
    }

    /// Write **LP_ANA_TOUCH_DEBOUNCE_LIMIT** `[31:29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_debounce_limit(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_DEBOUNCE_LIMIT_MASK)
                | ((value << Self::LP_ANA_TOUCH_DEBOUNCE_LIMIT_OFFSET)
                    & Self::LP_ANA_TOUCH_DEBOUNCE_LIMIT_MASK)
        });
    }

    /// **LP_ANA_TOUCH_APPROACH_LIMIT** `[28:21]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_approach_limit(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_APPROACH_LIMIT_MASK)
            >> Self::LP_ANA_TOUCH_APPROACH_LIMIT_OFFSET
    }

    /// Write **LP_ANA_TOUCH_APPROACH_LIMIT** `[28:21]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_approach_limit(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_APPROACH_LIMIT_MASK)
                | ((value << Self::LP_ANA_TOUCH_APPROACH_LIMIT_OFFSET)
                    & Self::LP_ANA_TOUCH_APPROACH_LIMIT_MASK)
        });
    }

    /// **LP_ANA_TOUCH_NEG_NOISE_LIMIT** `[20:17]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_neg_noise_limit(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_NEG_NOISE_LIMIT_MASK)
            >> Self::LP_ANA_TOUCH_NEG_NOISE_LIMIT_OFFSET
    }

    /// Write **LP_ANA_TOUCH_NEG_NOISE_LIMIT** `[20:17]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_neg_noise_limit(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_NEG_NOISE_LIMIT_MASK)
                | ((value << Self::LP_ANA_TOUCH_NEG_NOISE_LIMIT_OFFSET)
                    & Self::LP_ANA_TOUCH_NEG_NOISE_LIMIT_MASK)
        });
    }

    /// **LP_ANA_TOUCH_FILTER_EN** `[16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_filter_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FILTER_EN_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_filter_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_FILTER_EN_MASK);
    }

    /// Clear LP_ANA_TOUCH_FILTER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_filter_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_FILTER_EN_MASK);
    }

    /// **LP_ANA_TOUCH_FILTER_MODE** `[15:13]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_filter_mode(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FILTER_MODE_MASK)
            >> Self::LP_ANA_TOUCH_FILTER_MODE_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FILTER_MODE** `[15:13]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_filter_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FILTER_MODE_MASK)
                | ((value << Self::LP_ANA_TOUCH_FILTER_MODE_OFFSET)
                    & Self::LP_ANA_TOUCH_FILTER_MODE_MASK)
        });
    }

    /// **LP_ANA_TOUCH_JITTER_STEP** `[12:9]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_jitter_step(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_JITTER_STEP_MASK)
            >> Self::LP_ANA_TOUCH_JITTER_STEP_OFFSET
    }

    /// Write **LP_ANA_TOUCH_JITTER_STEP** `[12:9]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_jitter_step(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_JITTER_STEP_MASK)
                | ((value << Self::LP_ANA_TOUCH_JITTER_STEP_OFFSET)
                    & Self::LP_ANA_TOUCH_JITTER_STEP_MASK)
        });
    }

    /// **LP_ANA_TOUCH_SMOOTH_LVL** `[8:7]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_smooth_lvl(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_SMOOTH_LVL_MASK)
            >> Self::LP_ANA_TOUCH_SMOOTH_LVL_OFFSET
    }

    /// Write **LP_ANA_TOUCH_SMOOTH_LVL** `[8:7]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_smooth_lvl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_SMOOTH_LVL_MASK)
                | ((value << Self::LP_ANA_TOUCH_SMOOTH_LVL_OFFSET)
                    & Self::LP_ANA_TOUCH_SMOOTH_LVL_MASK)
        });
    }

    /// **LP_ANA_TOUCH_NOISE_THRES** `[6:5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_noise_thres(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_NOISE_THRES_MASK)
            >> Self::LP_ANA_TOUCH_NOISE_THRES_OFFSET
    }

    /// Write **LP_ANA_TOUCH_NOISE_THRES** `[6:5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_noise_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_NOISE_THRES_MASK)
                | ((value << Self::LP_ANA_TOUCH_NOISE_THRES_OFFSET)
                    & Self::LP_ANA_TOUCH_NOISE_THRES_MASK)
        });
    }

    /// **LP_ANA_TOUCH_NEG_NOISE_THRES** `[4:3]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_neg_noise_thres(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_NEG_NOISE_THRES_MASK)
            >> Self::LP_ANA_TOUCH_NEG_NOISE_THRES_OFFSET
    }

    /// Write **LP_ANA_TOUCH_NEG_NOISE_THRES** `[4:3]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_neg_noise_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_NEG_NOISE_THRES_MASK)
                | ((value << Self::LP_ANA_TOUCH_NEG_NOISE_THRES_OFFSET)
                    & Self::LP_ANA_TOUCH_NEG_NOISE_THRES_MASK)
        });
    }

    /// **LP_ANA_TOUCH_HYSTERESIS** `[2:1]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_hysteresis(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_HYSTERESIS_MASK)
            >> Self::LP_ANA_TOUCH_HYSTERESIS_OFFSET
    }

    /// Write **LP_ANA_TOUCH_HYSTERESIS** `[2:1]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_hysteresis(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_HYSTERESIS_MASK)
                | ((value << Self::LP_ANA_TOUCH_HYSTERESIS_OFFSET)
                    & Self::LP_ANA_TOUCH_HYSTERESIS_MASK)
        });
    }

    /// **LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN** `[0]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_neg_noise_disupdate_baseline_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_neg_noise_disupdate_baseline_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_MASK);
    }

    /// Clear LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_neg_noise_disupdate_baseline_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_MASK);
    }
}

/// LP_ANA_TOUCH_FILTER1 value type for batch read/modify/write operations
///
/// Use `lpanatouchfilter1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchfilter1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchFilter1Val {
    bits: u32,
}

impl LpAnaTouchFilter1Val {
    /// Create from raw bits
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
        Self { bits: 0x6a0a0200 }
    }

    /// Read **LP_ANA_TOUCH_DEBOUNCE_LIMIT** `[31:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_debounce_limit(&self) -> u32 {
        (self.bits & LpAnaTouchFilter1::LP_ANA_TOUCH_DEBOUNCE_LIMIT_MASK)
            >> LpAnaTouchFilter1::LP_ANA_TOUCH_DEBOUNCE_LIMIT_OFFSET
    }

    /// Read **LP_ANA_TOUCH_APPROACH_LIMIT** `[28:21]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_approach_limit(&self) -> u32 {
        (self.bits & LpAnaTouchFilter1::LP_ANA_TOUCH_APPROACH_LIMIT_MASK)
            >> LpAnaTouchFilter1::LP_ANA_TOUCH_APPROACH_LIMIT_OFFSET
    }

    /// Read **LP_ANA_TOUCH_NEG_NOISE_LIMIT** `[20:17]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_neg_noise_limit(&self) -> u32 {
        (self.bits & LpAnaTouchFilter1::LP_ANA_TOUCH_NEG_NOISE_LIMIT_MASK)
            >> LpAnaTouchFilter1::LP_ANA_TOUCH_NEG_NOISE_LIMIT_OFFSET
    }

    /// Read **LP_ANA_TOUCH_FILTER_EN** `[16:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_filter_en(&self) -> bool {
        (self.bits & LpAnaTouchFilter1::LP_ANA_TOUCH_FILTER_EN_MASK) != 0
    }

    /// Read **LP_ANA_TOUCH_FILTER_MODE** `[15:13]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_filter_mode(&self) -> u32 {
        (self.bits & LpAnaTouchFilter1::LP_ANA_TOUCH_FILTER_MODE_MASK)
            >> LpAnaTouchFilter1::LP_ANA_TOUCH_FILTER_MODE_OFFSET
    }

    /// Read **LP_ANA_TOUCH_JITTER_STEP** `[12:9]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_jitter_step(&self) -> u32 {
        (self.bits & LpAnaTouchFilter1::LP_ANA_TOUCH_JITTER_STEP_MASK)
            >> LpAnaTouchFilter1::LP_ANA_TOUCH_JITTER_STEP_OFFSET
    }

    /// Read **LP_ANA_TOUCH_SMOOTH_LVL** `[8:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_smooth_lvl(&self) -> u32 {
        (self.bits & LpAnaTouchFilter1::LP_ANA_TOUCH_SMOOTH_LVL_MASK)
            >> LpAnaTouchFilter1::LP_ANA_TOUCH_SMOOTH_LVL_OFFSET
    }

    /// Read **LP_ANA_TOUCH_NOISE_THRES** `[6:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_noise_thres(&self) -> u32 {
        (self.bits & LpAnaTouchFilter1::LP_ANA_TOUCH_NOISE_THRES_MASK)
            >> LpAnaTouchFilter1::LP_ANA_TOUCH_NOISE_THRES_OFFSET
    }

    /// Read **LP_ANA_TOUCH_NEG_NOISE_THRES** `[4:3]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_neg_noise_thres(&self) -> u32 {
        (self.bits & LpAnaTouchFilter1::LP_ANA_TOUCH_NEG_NOISE_THRES_MASK)
            >> LpAnaTouchFilter1::LP_ANA_TOUCH_NEG_NOISE_THRES_OFFSET
    }

    /// Read **LP_ANA_TOUCH_HYSTERESIS** `[2:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_hysteresis(&self) -> u32 {
        (self.bits & LpAnaTouchFilter1::LP_ANA_TOUCH_HYSTERESIS_MASK)
            >> LpAnaTouchFilter1::LP_ANA_TOUCH_HYSTERESIS_OFFSET
    }

    /// Read **LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_neg_noise_disupdate_baseline_en(&self) -> bool {
        (self.bits & LpAnaTouchFilter1::LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_MASK) != 0
    }

    /// Set **LP_ANA_TOUCH_DEBOUNCE_LIMIT** `[31:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_debounce_limit(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFilter1::LP_ANA_TOUCH_DEBOUNCE_LIMIT_MASK)
                | ((value << LpAnaTouchFilter1::LP_ANA_TOUCH_DEBOUNCE_LIMIT_OFFSET)
                    & LpAnaTouchFilter1::LP_ANA_TOUCH_DEBOUNCE_LIMIT_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_APPROACH_LIMIT** `[28:21]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_approach_limit(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFilter1::LP_ANA_TOUCH_APPROACH_LIMIT_MASK)
                | ((value << LpAnaTouchFilter1::LP_ANA_TOUCH_APPROACH_LIMIT_OFFSET)
                    & LpAnaTouchFilter1::LP_ANA_TOUCH_APPROACH_LIMIT_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_NEG_NOISE_LIMIT** `[20:17]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_neg_noise_limit(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFilter1::LP_ANA_TOUCH_NEG_NOISE_LIMIT_MASK)
                | ((value << LpAnaTouchFilter1::LP_ANA_TOUCH_NEG_NOISE_LIMIT_OFFSET)
                    & LpAnaTouchFilter1::LP_ANA_TOUCH_NEG_NOISE_LIMIT_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_FILTER_EN** `[16:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_filter_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchFilter1::LP_ANA_TOUCH_FILTER_EN_MASK
            } else {
                self.bits & !LpAnaTouchFilter1::LP_ANA_TOUCH_FILTER_EN_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_FILTER_MODE** `[15:13]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_filter_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFilter1::LP_ANA_TOUCH_FILTER_MODE_MASK)
                | ((value << LpAnaTouchFilter1::LP_ANA_TOUCH_FILTER_MODE_OFFSET)
                    & LpAnaTouchFilter1::LP_ANA_TOUCH_FILTER_MODE_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_JITTER_STEP** `[12:9]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_jitter_step(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFilter1::LP_ANA_TOUCH_JITTER_STEP_MASK)
                | ((value << LpAnaTouchFilter1::LP_ANA_TOUCH_JITTER_STEP_OFFSET)
                    & LpAnaTouchFilter1::LP_ANA_TOUCH_JITTER_STEP_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_SMOOTH_LVL** `[8:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_smooth_lvl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFilter1::LP_ANA_TOUCH_SMOOTH_LVL_MASK)
                | ((value << LpAnaTouchFilter1::LP_ANA_TOUCH_SMOOTH_LVL_OFFSET)
                    & LpAnaTouchFilter1::LP_ANA_TOUCH_SMOOTH_LVL_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_NOISE_THRES** `[6:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_noise_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFilter1::LP_ANA_TOUCH_NOISE_THRES_MASK)
                | ((value << LpAnaTouchFilter1::LP_ANA_TOUCH_NOISE_THRES_OFFSET)
                    & LpAnaTouchFilter1::LP_ANA_TOUCH_NOISE_THRES_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_NEG_NOISE_THRES** `[4:3]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_neg_noise_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFilter1::LP_ANA_TOUCH_NEG_NOISE_THRES_MASK)
                | ((value << LpAnaTouchFilter1::LP_ANA_TOUCH_NEG_NOISE_THRES_OFFSET)
                    & LpAnaTouchFilter1::LP_ANA_TOUCH_NEG_NOISE_THRES_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_HYSTERESIS** `[2:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_hysteresis(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFilter1::LP_ANA_TOUCH_HYSTERESIS_MASK)
                | ((value << LpAnaTouchFilter1::LP_ANA_TOUCH_HYSTERESIS_OFFSET)
                    & LpAnaTouchFilter1::LP_ANA_TOUCH_HYSTERESIS_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN** `[0:0]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_neg_noise_disupdate_baseline_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchFilter1::LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_MASK
            } else {
                self.bits & !LpAnaTouchFilter1::LP_ANA_TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN_MASK
            },
        }
    }
}

impl Default for LpAnaTouchFilter1Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaTouchFilter1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchFilter1Val {
        LpAnaTouchFilter1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchFilter1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchFilter1Val) -> LpAnaTouchFilter1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_FILTER2 Register
///
/// need_des
///
/// Address offset: 0x0114
/// Reset value: 0x1fff8000
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES** `[31]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_BYPASS_NOISE_THRES** `[30]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_OUTEN** `[29:15]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchFilter2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchFilter2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchFilter2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES_OFFSET: u32 = 31;
    pub const LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES_MASK: u32 = 0x80000000;
    pub const LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES: u32 = 1 << 31;

    /// **LP_ANA_TOUCH_BYPASS_NOISE_THRES**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_ANA_TOUCH_BYPASS_NOISE_THRES_OFFSET: u32 = 30;
    pub const LP_ANA_TOUCH_BYPASS_NOISE_THRES_MASK: u32 = 0x40000000;
    pub const LP_ANA_TOUCH_BYPASS_NOISE_THRES: u32 = 1 << 30;

    /// **LP_ANA_TOUCH_OUTEN**
    ///
    /// need_des
    ///
    /// Bits `[29:15]`, mask `0x3fff8000`
    pub const LP_ANA_TOUCH_OUTEN_OFFSET: u32 = 15;
    pub const LP_ANA_TOUCH_OUTEN_MASK: u32 = 0x3fff8000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_bypass_neg_noise_thres(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_bypass_neg_noise_thres(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES_MASK);
    }

    /// Clear LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_bypass_neg_noise_thres(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES_MASK);
    }

    /// **LP_ANA_TOUCH_BYPASS_NOISE_THRES** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_bypass_noise_thres(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_BYPASS_NOISE_THRES_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_BYPASS_NOISE_THRES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_bypass_noise_thres(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_BYPASS_NOISE_THRES_MASK);
    }

    /// Clear LP_ANA_TOUCH_BYPASS_NOISE_THRES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_bypass_noise_thres(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_BYPASS_NOISE_THRES_MASK);
    }

    /// **LP_ANA_TOUCH_OUTEN** `[29:15]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_outen(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_OUTEN_MASK) >> Self::LP_ANA_TOUCH_OUTEN_OFFSET
    }

    /// Write **LP_ANA_TOUCH_OUTEN** `[29:15]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_outen(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_OUTEN_MASK)
                | ((value << Self::LP_ANA_TOUCH_OUTEN_OFFSET) & Self::LP_ANA_TOUCH_OUTEN_MASK)
        });
    }
}

/// LP_ANA_TOUCH_FILTER2 value type for batch read/modify/write operations
///
/// Use `lpanatouchfilter2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchfilter2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchFilter2Val {
    bits: u32,
}

impl LpAnaTouchFilter2Val {
    /// Create from raw bits
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
        Self { bits: 0x1fff8000 }
    }

    /// Read **LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_bypass_neg_noise_thres(&self) -> bool {
        (self.bits & LpAnaTouchFilter2::LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES_MASK) != 0
    }

    /// Read **LP_ANA_TOUCH_BYPASS_NOISE_THRES** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_bypass_noise_thres(&self) -> bool {
        (self.bits & LpAnaTouchFilter2::LP_ANA_TOUCH_BYPASS_NOISE_THRES_MASK) != 0
    }

    /// Read **LP_ANA_TOUCH_OUTEN** `[29:15]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_outen(&self) -> u32 {
        (self.bits & LpAnaTouchFilter2::LP_ANA_TOUCH_OUTEN_MASK)
            >> LpAnaTouchFilter2::LP_ANA_TOUCH_OUTEN_OFFSET
    }

    /// Set **LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_bypass_neg_noise_thres(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchFilter2::LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES_MASK
            } else {
                self.bits & !LpAnaTouchFilter2::LP_ANA_TOUCH_BYPASS_NEG_NOISE_THRES_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_BYPASS_NOISE_THRES** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_bypass_noise_thres(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchFilter2::LP_ANA_TOUCH_BYPASS_NOISE_THRES_MASK
            } else {
                self.bits & !LpAnaTouchFilter2::LP_ANA_TOUCH_BYPASS_NOISE_THRES_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_OUTEN** `[29:15]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_outen(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFilter2::LP_ANA_TOUCH_OUTEN_MASK)
                | ((value << LpAnaTouchFilter2::LP_ANA_TOUCH_OUTEN_OFFSET)
                    & LpAnaTouchFilter2::LP_ANA_TOUCH_OUTEN_MASK),
        }
    }
}

impl Default for LpAnaTouchFilter2Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaTouchFilter2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchFilter2Val {
        LpAnaTouchFilter2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchFilter2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchFilter2Val) -> LpAnaTouchFilter2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_FILTER3 Register
///
/// need_des
///
/// Address offset: 0x0118
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_UPDATE_BASELINE_SW** `[16]` (WO)
///
///   need_des
///
/// - **LP_ANA_TOUCH_BASELINE_SW** `[15:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchFilter3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchFilter3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchFilter3 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_UPDATE_BASELINE_SW**
    ///
    /// need_des
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const LP_ANA_TOUCH_UPDATE_BASELINE_SW_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_UPDATE_BASELINE_SW_MASK: u32 = 0x00010000;
    pub const LP_ANA_TOUCH_UPDATE_BASELINE_SW: u32 = 1 << 16;

    /// **LP_ANA_TOUCH_BASELINE_SW**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const LP_ANA_TOUCH_BASELINE_SW_OFFSET: u32 = 0;
    pub const LP_ANA_TOUCH_BASELINE_SW_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Trigger **LP_ANA_TOUCH_UPDATE_BASELINE_SW** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_ana_touch_update_baseline_sw(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_UPDATE_BASELINE_SW_MASK);
    }

    /// **LP_ANA_TOUCH_BASELINE_SW** `[15:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_baseline_sw(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_BASELINE_SW_MASK)
            >> Self::LP_ANA_TOUCH_BASELINE_SW_OFFSET
    }

    /// Write **LP_ANA_TOUCH_BASELINE_SW** `[15:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_baseline_sw(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_BASELINE_SW_MASK)
                | ((value << Self::LP_ANA_TOUCH_BASELINE_SW_OFFSET)
                    & Self::LP_ANA_TOUCH_BASELINE_SW_MASK)
        });
    }
}

/// LP_ANA_TOUCH_FILTER3 value type for batch read/modify/write operations
///
/// Use `lpanatouchfilter3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchfilter3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchFilter3Val {
    bits: u32,
}

impl LpAnaTouchFilter3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_BASELINE_SW** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_baseline_sw(&self) -> u32 {
        (self.bits & LpAnaTouchFilter3::LP_ANA_TOUCH_BASELINE_SW_MASK)
            >> LpAnaTouchFilter3::LP_ANA_TOUCH_BASELINE_SW_OFFSET
    }

    /// Set **LP_ANA_TOUCH_UPDATE_BASELINE_SW** `[16:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_update_baseline_sw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchFilter3::LP_ANA_TOUCH_UPDATE_BASELINE_SW_MASK
            } else {
                self.bits & !LpAnaTouchFilter3::LP_ANA_TOUCH_UPDATE_BASELINE_SW_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_BASELINE_SW** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_baseline_sw(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFilter3::LP_ANA_TOUCH_BASELINE_SW_MASK)
                | ((value << LpAnaTouchFilter3::LP_ANA_TOUCH_BASELINE_SW_OFFSET)
                    & LpAnaTouchFilter3::LP_ANA_TOUCH_BASELINE_SW_MASK),
        }
    }
}

impl Default for LpAnaTouchFilter3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchFilter3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchFilter3Val {
        LpAnaTouchFilter3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchFilter3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchFilter3Val) -> LpAnaTouchFilter3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_SLP0 Register
///
/// need_des
///
/// Address offset: 0x011c
/// Reset value: 0x001e0000
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_SLP_PAD** `[20:17]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_SLP_CHANNEL_CLR** `[16]` (WO)
///
///   need_des
///
/// - **LP_ANA_TOUCH_SLP_TH0** `[15:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchSlp0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchSlp0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchSlp0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_SLP_PAD**
    ///
    /// need_des
    ///
    /// Bits `[20:17]`, mask `0x001e0000`
    pub const LP_ANA_TOUCH_SLP_PAD_OFFSET: u32 = 17;
    pub const LP_ANA_TOUCH_SLP_PAD_MASK: u32 = 0x001e0000;

    /// **LP_ANA_TOUCH_SLP_CHANNEL_CLR**
    ///
    /// need_des
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const LP_ANA_TOUCH_SLP_CHANNEL_CLR_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_SLP_CHANNEL_CLR_MASK: u32 = 0x00010000;
    pub const LP_ANA_TOUCH_SLP_CHANNEL_CLR: u32 = 1 << 16;

    /// **LP_ANA_TOUCH_SLP_TH0**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const LP_ANA_TOUCH_SLP_TH0_OFFSET: u32 = 0;
    pub const LP_ANA_TOUCH_SLP_TH0_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_SLP_PAD** `[20:17]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_slp_pad(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_SLP_PAD_MASK)
            >> Self::LP_ANA_TOUCH_SLP_PAD_OFFSET
    }

    /// Write **LP_ANA_TOUCH_SLP_PAD** `[20:17]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_slp_pad(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_SLP_PAD_MASK)
                | ((value << Self::LP_ANA_TOUCH_SLP_PAD_OFFSET) & Self::LP_ANA_TOUCH_SLP_PAD_MASK)
        });
    }

    /// Trigger **LP_ANA_TOUCH_SLP_CHANNEL_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_ana_touch_slp_channel_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_SLP_CHANNEL_CLR_MASK);
    }

    /// **LP_ANA_TOUCH_SLP_TH0** `[15:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_slp_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_SLP_TH0_MASK)
            >> Self::LP_ANA_TOUCH_SLP_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_SLP_TH0** `[15:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_slp_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_SLP_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_SLP_TH0_OFFSET) & Self::LP_ANA_TOUCH_SLP_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_SLP0 value type for batch read/modify/write operations
///
/// Use `lpanatouchslp0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchslp0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchSlp0Val {
    bits: u32,
}

impl LpAnaTouchSlp0Val {
    /// Create from raw bits
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
        Self { bits: 0x001e0000 }
    }

    /// Read **LP_ANA_TOUCH_SLP_PAD** `[20:17]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_slp_pad(&self) -> u32 {
        (self.bits & LpAnaTouchSlp0::LP_ANA_TOUCH_SLP_PAD_MASK)
            >> LpAnaTouchSlp0::LP_ANA_TOUCH_SLP_PAD_OFFSET
    }

    /// Read **LP_ANA_TOUCH_SLP_TH0** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_slp_th0(&self) -> u32 {
        (self.bits & LpAnaTouchSlp0::LP_ANA_TOUCH_SLP_TH0_MASK)
            >> LpAnaTouchSlp0::LP_ANA_TOUCH_SLP_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_SLP_PAD** `[20:17]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_slp_pad(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchSlp0::LP_ANA_TOUCH_SLP_PAD_MASK)
                | ((value << LpAnaTouchSlp0::LP_ANA_TOUCH_SLP_PAD_OFFSET)
                    & LpAnaTouchSlp0::LP_ANA_TOUCH_SLP_PAD_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_SLP_CHANNEL_CLR** `[16:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_slp_channel_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchSlp0::LP_ANA_TOUCH_SLP_CHANNEL_CLR_MASK
            } else {
                self.bits & !LpAnaTouchSlp0::LP_ANA_TOUCH_SLP_CHANNEL_CLR_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_SLP_TH0** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_slp_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchSlp0::LP_ANA_TOUCH_SLP_TH0_MASK)
                | ((value << LpAnaTouchSlp0::LP_ANA_TOUCH_SLP_TH0_OFFSET)
                    & LpAnaTouchSlp0::LP_ANA_TOUCH_SLP_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchSlp0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaTouchSlp0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchSlp0Val {
        LpAnaTouchSlp0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchSlp0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchSlp0Val) -> LpAnaTouchSlp0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_SLP1 Register
///
/// need_des
///
/// Address offset: 0x0120
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_SLP_TH1** `[31:16]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_SLP_TH2** `[15:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchSlp1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchSlp1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchSlp1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_SLP_TH1**
    ///
    /// need_des
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_SLP_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_SLP_TH1_MASK: u32 = 0xffff0000;

    /// **LP_ANA_TOUCH_SLP_TH2**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const LP_ANA_TOUCH_SLP_TH2_OFFSET: u32 = 0;
    pub const LP_ANA_TOUCH_SLP_TH2_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_SLP_TH1** `[31:16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_slp_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_SLP_TH1_MASK)
            >> Self::LP_ANA_TOUCH_SLP_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_SLP_TH1** `[31:16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_slp_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_SLP_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_SLP_TH1_OFFSET) & Self::LP_ANA_TOUCH_SLP_TH1_MASK)
        });
    }

    /// **LP_ANA_TOUCH_SLP_TH2** `[15:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_slp_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_SLP_TH2_MASK)
            >> Self::LP_ANA_TOUCH_SLP_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_SLP_TH2** `[15:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_slp_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_SLP_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_SLP_TH2_OFFSET) & Self::LP_ANA_TOUCH_SLP_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_SLP1 value type for batch read/modify/write operations
///
/// Use `lpanatouchslp1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchslp1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchSlp1Val {
    bits: u32,
}

impl LpAnaTouchSlp1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_SLP_TH1** `[31:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_slp_th1(&self) -> u32 {
        (self.bits & LpAnaTouchSlp1::LP_ANA_TOUCH_SLP_TH1_MASK)
            >> LpAnaTouchSlp1::LP_ANA_TOUCH_SLP_TH1_OFFSET
    }

    /// Read **LP_ANA_TOUCH_SLP_TH2** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_slp_th2(&self) -> u32 {
        (self.bits & LpAnaTouchSlp1::LP_ANA_TOUCH_SLP_TH2_MASK)
            >> LpAnaTouchSlp1::LP_ANA_TOUCH_SLP_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_SLP_TH1** `[31:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_slp_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchSlp1::LP_ANA_TOUCH_SLP_TH1_MASK)
                | ((value << LpAnaTouchSlp1::LP_ANA_TOUCH_SLP_TH1_OFFSET)
                    & LpAnaTouchSlp1::LP_ANA_TOUCH_SLP_TH1_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_SLP_TH2** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_slp_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchSlp1::LP_ANA_TOUCH_SLP_TH2_MASK)
                | ((value << LpAnaTouchSlp1::LP_ANA_TOUCH_SLP_TH2_OFFSET)
                    & LpAnaTouchSlp1::LP_ANA_TOUCH_SLP_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchSlp1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchSlp1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchSlp1Val {
        LpAnaTouchSlp1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchSlp1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchSlp1Val) -> LpAnaTouchSlp1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_CLR Register
///
/// need_des
///
/// Address offset: 0x0124
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_STATUS_CLR** `[15]` (WO)
///
///   need_des
///
/// - **LP_ANA_TOUCH_CHANNEL_CLR** `[14:0]` (WO)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchClr {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_STATUS_CLR**
    ///
    /// need_des
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const LP_ANA_TOUCH_STATUS_CLR_OFFSET: u32 = 15;
    pub const LP_ANA_TOUCH_STATUS_CLR_MASK: u32 = 0x00008000;
    pub const LP_ANA_TOUCH_STATUS_CLR: u32 = 1 << 15;

    /// **LP_ANA_TOUCH_CHANNEL_CLR**
    ///
    /// need_des
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const LP_ANA_TOUCH_CHANNEL_CLR_OFFSET: u32 = 0;
    pub const LP_ANA_TOUCH_CHANNEL_CLR_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// Trigger **LP_ANA_TOUCH_STATUS_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_ana_touch_status_clr(&self) {
        self.value.write_bits(Self::LP_ANA_TOUCH_STATUS_CLR_MASK);
    }

    /// Write **LP_ANA_TOUCH_CHANNEL_CLR** `[14:0]` (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_channel_clr(&self, value: u32) {
        self.value.write_bits(
            (value << Self::LP_ANA_TOUCH_CHANNEL_CLR_OFFSET) & Self::LP_ANA_TOUCH_CHANNEL_CLR_MASK,
        );
    }
}

/// LP_ANA_TOUCH_CLR value type for batch read/modify/write operations
///
/// Use `lpanatouchclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchClrVal {
    bits: u32,
}

impl LpAnaTouchClrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **LP_ANA_TOUCH_STATUS_CLR** `[15:15]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_status_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchClr::LP_ANA_TOUCH_STATUS_CLR_MASK
            } else {
                self.bits & !LpAnaTouchClr::LP_ANA_TOUCH_STATUS_CLR_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_CHANNEL_CLR** `[14:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_channel_clr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchClr::LP_ANA_TOUCH_CHANNEL_CLR_MASK)
                | ((value << LpAnaTouchClr::LP_ANA_TOUCH_CHANNEL_CLR_OFFSET)
                    & LpAnaTouchClr::LP_ANA_TOUCH_CHANNEL_CLR_MASK),
        }
    }
}

impl Default for LpAnaTouchClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// LP_ANA_TOUCH_APPROACH Register
///
/// need_des
///
/// Address offset: 0x0128
/// Reset value: 0x00000fff
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_SLP_APPROACH_EN** `[12]` (RW)
///
///   need_des
///
/// - **PAD2** `[11:8]` (RW)
///
///   need_des
///
/// - **PAD1** `[7:4]` (RW)
///
///   need_des
///
/// - **PAD0** `[3:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchApproach {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchApproach {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchApproach {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_SLP_APPROACH_EN**
    ///
    /// need_des
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const LP_ANA_TOUCH_SLP_APPROACH_EN_OFFSET: u32 = 12;
    pub const LP_ANA_TOUCH_SLP_APPROACH_EN_MASK: u32 = 0x00001000;
    pub const LP_ANA_TOUCH_SLP_APPROACH_EN: u32 = 1 << 12;

    /// **PAD2**
    ///
    /// need_des
    ///
    /// Bits `[11:8]`, mask `0x00000f00`
    pub const PAD2_OFFSET: u32 = 8;
    pub const PAD2_MASK: u32 = 0x00000f00;

    /// **PAD1**
    ///
    /// need_des
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const PAD1_OFFSET: u32 = 4;
    pub const PAD1_MASK: u32 = 0x000000f0;

    /// **PAD0**
    ///
    /// need_des
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const PAD0_OFFSET: u32 = 0;
    pub const PAD0_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_SLP_APPROACH_EN** `[12]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_slp_approach_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_SLP_APPROACH_EN_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_SLP_APPROACH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_slp_approach_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_SLP_APPROACH_EN_MASK);
    }

    /// Clear LP_ANA_TOUCH_SLP_APPROACH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_slp_approach_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_SLP_APPROACH_EN_MASK);
    }

    /// **PAD2** `[11:8]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad2(&self) -> u32 {
        (self.value.read_bits() & Self::PAD2_MASK) >> Self::PAD2_OFFSET
    }

    /// Write **PAD2** `[11:8]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_pad2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PAD2_MASK) | ((value << Self::PAD2_OFFSET) & Self::PAD2_MASK)
        });
    }

    /// **PAD1** `[7:4]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad1(&self) -> u32 {
        (self.value.read_bits() & Self::PAD1_MASK) >> Self::PAD1_OFFSET
    }

    /// Write **PAD1** `[7:4]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_pad1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PAD1_MASK) | ((value << Self::PAD1_OFFSET) & Self::PAD1_MASK)
        });
    }

    /// **PAD0** `[3:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn pad0(&self) -> u32 {
        (self.value.read_bits() & Self::PAD0_MASK) >> Self::PAD0_OFFSET
    }

    /// Write **PAD0** `[3:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_pad0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PAD0_MASK) | ((value << Self::PAD0_OFFSET) & Self::PAD0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_APPROACH value type for batch read/modify/write operations
///
/// Use `lpanatouchapproach.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchapproach.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchApproachVal {
    bits: u32,
}

impl LpAnaTouchApproachVal {
    /// Create from raw bits
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
        Self { bits: 0x00000fff }
    }

    /// Read **LP_ANA_TOUCH_SLP_APPROACH_EN** `[12:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_slp_approach_en(&self) -> bool {
        (self.bits & LpAnaTouchApproach::LP_ANA_TOUCH_SLP_APPROACH_EN_MASK) != 0
    }

    /// Read **PAD2** `[11:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad2(&self) -> u32 {
        (self.bits & LpAnaTouchApproach::PAD2_MASK) >> LpAnaTouchApproach::PAD2_OFFSET
    }

    /// Read **PAD1** `[7:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad1(&self) -> u32 {
        (self.bits & LpAnaTouchApproach::PAD1_MASK) >> LpAnaTouchApproach::PAD1_OFFSET
    }

    /// Read **PAD0** `[3:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pad0(&self) -> u32 {
        (self.bits & LpAnaTouchApproach::PAD0_MASK) >> LpAnaTouchApproach::PAD0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_SLP_APPROACH_EN** `[12:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_slp_approach_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchApproach::LP_ANA_TOUCH_SLP_APPROACH_EN_MASK
            } else {
                self.bits & !LpAnaTouchApproach::LP_ANA_TOUCH_SLP_APPROACH_EN_MASK
            },
        }
    }

    /// Set **PAD2** `[11:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_pad2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchApproach::PAD2_MASK)
                | ((value << LpAnaTouchApproach::PAD2_OFFSET) & LpAnaTouchApproach::PAD2_MASK),
        }
    }

    /// Set **PAD1** `[7:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_pad1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchApproach::PAD1_MASK)
                | ((value << LpAnaTouchApproach::PAD1_OFFSET) & LpAnaTouchApproach::PAD1_MASK),
        }
    }

    /// Set **PAD0** `[3:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_pad0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchApproach::PAD0_MASK)
                | ((value << LpAnaTouchApproach::PAD0_OFFSET) & LpAnaTouchApproach::PAD0_MASK),
        }
    }
}

impl Default for LpAnaTouchApproachVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaTouchApproach {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchApproachVal {
        LpAnaTouchApproachVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchApproachVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchApproachVal) -> LpAnaTouchApproachVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_FREQ0_SCAN_PARA Register
///
/// need_des
///
/// Address offset: 0x012c
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_FREQ0_DBIAS** `[22:18]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FREQ0_DRV_HS** `[17:13]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FREQ0_DRV_LS** `[12:9]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FREQ0_DRES_LPF** `[8:7]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FREQ0_DCAP_LPF** `[6:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchFreq0ScanPara {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchFreq0ScanPara {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchFreq0ScanPara {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_FREQ0_DBIAS**
    ///
    /// need_des
    ///
    /// Bits `[22:18]`, mask `0x007c0000`
    pub const LP_ANA_TOUCH_FREQ0_DBIAS_OFFSET: u32 = 18;
    pub const LP_ANA_TOUCH_FREQ0_DBIAS_MASK: u32 = 0x007c0000;

    /// **LP_ANA_TOUCH_FREQ0_DRV_HS**
    ///
    /// need_des
    ///
    /// Bits `[17:13]`, mask `0x0003e000`
    pub const LP_ANA_TOUCH_FREQ0_DRV_HS_OFFSET: u32 = 13;
    pub const LP_ANA_TOUCH_FREQ0_DRV_HS_MASK: u32 = 0x0003e000;

    /// **LP_ANA_TOUCH_FREQ0_DRV_LS**
    ///
    /// need_des
    ///
    /// Bits `[12:9]`, mask `0x00001e00`
    pub const LP_ANA_TOUCH_FREQ0_DRV_LS_OFFSET: u32 = 9;
    pub const LP_ANA_TOUCH_FREQ0_DRV_LS_MASK: u32 = 0x00001e00;

    /// **LP_ANA_TOUCH_FREQ0_DRES_LPF**
    ///
    /// need_des
    ///
    /// Bits `[8:7]`, mask `0x00000180`
    pub const LP_ANA_TOUCH_FREQ0_DRES_LPF_OFFSET: u32 = 7;
    pub const LP_ANA_TOUCH_FREQ0_DRES_LPF_MASK: u32 = 0x00000180;

    /// **LP_ANA_TOUCH_FREQ0_DCAP_LPF**
    ///
    /// need_des
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const LP_ANA_TOUCH_FREQ0_DCAP_LPF_OFFSET: u32 = 0;
    pub const LP_ANA_TOUCH_FREQ0_DCAP_LPF_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_FREQ0_DBIAS** `[22:18]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq0_dbias(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ0_DBIAS_MASK)
            >> Self::LP_ANA_TOUCH_FREQ0_DBIAS_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ0_DBIAS** `[22:18]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq0_dbias(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ0_DBIAS_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ0_DBIAS_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ0_DBIAS_MASK)
        });
    }

    /// **LP_ANA_TOUCH_FREQ0_DRV_HS** `[17:13]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq0_drv_hs(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ0_DRV_HS_MASK)
            >> Self::LP_ANA_TOUCH_FREQ0_DRV_HS_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ0_DRV_HS** `[17:13]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq0_drv_hs(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ0_DRV_HS_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ0_DRV_HS_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ0_DRV_HS_MASK)
        });
    }

    /// **LP_ANA_TOUCH_FREQ0_DRV_LS** `[12:9]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq0_drv_ls(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ0_DRV_LS_MASK)
            >> Self::LP_ANA_TOUCH_FREQ0_DRV_LS_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ0_DRV_LS** `[12:9]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq0_drv_ls(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ0_DRV_LS_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ0_DRV_LS_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ0_DRV_LS_MASK)
        });
    }

    /// **LP_ANA_TOUCH_FREQ0_DRES_LPF** `[8:7]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq0_dres_lpf(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ0_DRES_LPF_MASK)
            >> Self::LP_ANA_TOUCH_FREQ0_DRES_LPF_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ0_DRES_LPF** `[8:7]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq0_dres_lpf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ0_DRES_LPF_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ0_DRES_LPF_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ0_DRES_LPF_MASK)
        });
    }

    /// **LP_ANA_TOUCH_FREQ0_DCAP_LPF** `[6:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq0_dcap_lpf(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ0_DCAP_LPF_MASK)
            >> Self::LP_ANA_TOUCH_FREQ0_DCAP_LPF_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ0_DCAP_LPF** `[6:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq0_dcap_lpf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ0_DCAP_LPF_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ0_DCAP_LPF_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ0_DCAP_LPF_MASK)
        });
    }
}

/// LP_ANA_TOUCH_FREQ0_SCAN_PARA value type for batch read/modify/write operations
///
/// Use `lpanatouchfreq0scanpara.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchfreq0scanpara.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchFreq0ScanParaVal {
    bits: u32,
}

impl LpAnaTouchFreq0ScanParaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_FREQ0_DBIAS** `[22:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq0_dbias(&self) -> u32 {
        (self.bits & LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DBIAS_MASK)
            >> LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DBIAS_OFFSET
    }

    /// Read **LP_ANA_TOUCH_FREQ0_DRV_HS** `[17:13]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq0_drv_hs(&self) -> u32 {
        (self.bits & LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRV_HS_MASK)
            >> LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRV_HS_OFFSET
    }

    /// Read **LP_ANA_TOUCH_FREQ0_DRV_LS** `[12:9]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq0_drv_ls(&self) -> u32 {
        (self.bits & LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRV_LS_MASK)
            >> LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRV_LS_OFFSET
    }

    /// Read **LP_ANA_TOUCH_FREQ0_DRES_LPF** `[8:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq0_dres_lpf(&self) -> u32 {
        (self.bits & LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRES_LPF_MASK)
            >> LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRES_LPF_OFFSET
    }

    /// Read **LP_ANA_TOUCH_FREQ0_DCAP_LPF** `[6:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq0_dcap_lpf(&self) -> u32 {
        (self.bits & LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DCAP_LPF_MASK)
            >> LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DCAP_LPF_OFFSET
    }

    /// Set **LP_ANA_TOUCH_FREQ0_DBIAS** `[22:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq0_dbias(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DBIAS_MASK)
                | ((value << LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DBIAS_OFFSET)
                    & LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DBIAS_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_FREQ0_DRV_HS** `[17:13]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq0_drv_hs(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRV_HS_MASK)
                | ((value << LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRV_HS_OFFSET)
                    & LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRV_HS_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_FREQ0_DRV_LS** `[12:9]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq0_drv_ls(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRV_LS_MASK)
                | ((value << LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRV_LS_OFFSET)
                    & LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRV_LS_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_FREQ0_DRES_LPF** `[8:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq0_dres_lpf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRES_LPF_MASK)
                | ((value << LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRES_LPF_OFFSET)
                    & LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DRES_LPF_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_FREQ0_DCAP_LPF** `[6:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq0_dcap_lpf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DCAP_LPF_MASK)
                | ((value << LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DCAP_LPF_OFFSET)
                    & LpAnaTouchFreq0ScanPara::LP_ANA_TOUCH_FREQ0_DCAP_LPF_MASK),
        }
    }
}

impl Default for LpAnaTouchFreq0ScanParaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchFreq0ScanPara {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchFreq0ScanParaVal {
        LpAnaTouchFreq0ScanParaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchFreq0ScanParaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchFreq0ScanParaVal) -> LpAnaTouchFreq0ScanParaVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_FREQ1_SCAN_PARA Register
///
/// need_des
///
/// Address offset: 0x0130
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_FREQ1_DBIAS** `[22:18]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FREQ1_DRV_HS** `[17:13]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FREQ1_DRV_LS** `[12:9]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FREQ1_DRES_LPF** `[8:7]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FREQ1_DCAP_LPF** `[6:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchFreq1ScanPara {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchFreq1ScanPara {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchFreq1ScanPara {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_FREQ1_DBIAS**
    ///
    /// need_des
    ///
    /// Bits `[22:18]`, mask `0x007c0000`
    pub const LP_ANA_TOUCH_FREQ1_DBIAS_OFFSET: u32 = 18;
    pub const LP_ANA_TOUCH_FREQ1_DBIAS_MASK: u32 = 0x007c0000;

    /// **LP_ANA_TOUCH_FREQ1_DRV_HS**
    ///
    /// need_des
    ///
    /// Bits `[17:13]`, mask `0x0003e000`
    pub const LP_ANA_TOUCH_FREQ1_DRV_HS_OFFSET: u32 = 13;
    pub const LP_ANA_TOUCH_FREQ1_DRV_HS_MASK: u32 = 0x0003e000;

    /// **LP_ANA_TOUCH_FREQ1_DRV_LS**
    ///
    /// need_des
    ///
    /// Bits `[12:9]`, mask `0x00001e00`
    pub const LP_ANA_TOUCH_FREQ1_DRV_LS_OFFSET: u32 = 9;
    pub const LP_ANA_TOUCH_FREQ1_DRV_LS_MASK: u32 = 0x00001e00;

    /// **LP_ANA_TOUCH_FREQ1_DRES_LPF**
    ///
    /// need_des
    ///
    /// Bits `[8:7]`, mask `0x00000180`
    pub const LP_ANA_TOUCH_FREQ1_DRES_LPF_OFFSET: u32 = 7;
    pub const LP_ANA_TOUCH_FREQ1_DRES_LPF_MASK: u32 = 0x00000180;

    /// **LP_ANA_TOUCH_FREQ1_DCAP_LPF**
    ///
    /// need_des
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const LP_ANA_TOUCH_FREQ1_DCAP_LPF_OFFSET: u32 = 0;
    pub const LP_ANA_TOUCH_FREQ1_DCAP_LPF_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_FREQ1_DBIAS** `[22:18]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq1_dbias(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ1_DBIAS_MASK)
            >> Self::LP_ANA_TOUCH_FREQ1_DBIAS_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ1_DBIAS** `[22:18]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq1_dbias(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ1_DBIAS_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ1_DBIAS_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ1_DBIAS_MASK)
        });
    }

    /// **LP_ANA_TOUCH_FREQ1_DRV_HS** `[17:13]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq1_drv_hs(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ1_DRV_HS_MASK)
            >> Self::LP_ANA_TOUCH_FREQ1_DRV_HS_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ1_DRV_HS** `[17:13]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq1_drv_hs(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ1_DRV_HS_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ1_DRV_HS_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ1_DRV_HS_MASK)
        });
    }

    /// **LP_ANA_TOUCH_FREQ1_DRV_LS** `[12:9]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq1_drv_ls(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ1_DRV_LS_MASK)
            >> Self::LP_ANA_TOUCH_FREQ1_DRV_LS_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ1_DRV_LS** `[12:9]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq1_drv_ls(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ1_DRV_LS_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ1_DRV_LS_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ1_DRV_LS_MASK)
        });
    }

    /// **LP_ANA_TOUCH_FREQ1_DRES_LPF** `[8:7]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq1_dres_lpf(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ1_DRES_LPF_MASK)
            >> Self::LP_ANA_TOUCH_FREQ1_DRES_LPF_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ1_DRES_LPF** `[8:7]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq1_dres_lpf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ1_DRES_LPF_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ1_DRES_LPF_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ1_DRES_LPF_MASK)
        });
    }

    /// **LP_ANA_TOUCH_FREQ1_DCAP_LPF** `[6:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq1_dcap_lpf(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ1_DCAP_LPF_MASK)
            >> Self::LP_ANA_TOUCH_FREQ1_DCAP_LPF_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ1_DCAP_LPF** `[6:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq1_dcap_lpf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ1_DCAP_LPF_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ1_DCAP_LPF_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ1_DCAP_LPF_MASK)
        });
    }
}

/// LP_ANA_TOUCH_FREQ1_SCAN_PARA value type for batch read/modify/write operations
///
/// Use `lpanatouchfreq1scanpara.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchfreq1scanpara.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchFreq1ScanParaVal {
    bits: u32,
}

impl LpAnaTouchFreq1ScanParaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_FREQ1_DBIAS** `[22:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq1_dbias(&self) -> u32 {
        (self.bits & LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DBIAS_MASK)
            >> LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DBIAS_OFFSET
    }

    /// Read **LP_ANA_TOUCH_FREQ1_DRV_HS** `[17:13]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq1_drv_hs(&self) -> u32 {
        (self.bits & LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRV_HS_MASK)
            >> LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRV_HS_OFFSET
    }

    /// Read **LP_ANA_TOUCH_FREQ1_DRV_LS** `[12:9]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq1_drv_ls(&self) -> u32 {
        (self.bits & LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRV_LS_MASK)
            >> LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRV_LS_OFFSET
    }

    /// Read **LP_ANA_TOUCH_FREQ1_DRES_LPF** `[8:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq1_dres_lpf(&self) -> u32 {
        (self.bits & LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRES_LPF_MASK)
            >> LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRES_LPF_OFFSET
    }

    /// Read **LP_ANA_TOUCH_FREQ1_DCAP_LPF** `[6:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq1_dcap_lpf(&self) -> u32 {
        (self.bits & LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DCAP_LPF_MASK)
            >> LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DCAP_LPF_OFFSET
    }

    /// Set **LP_ANA_TOUCH_FREQ1_DBIAS** `[22:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq1_dbias(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DBIAS_MASK)
                | ((value << LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DBIAS_OFFSET)
                    & LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DBIAS_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_FREQ1_DRV_HS** `[17:13]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq1_drv_hs(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRV_HS_MASK)
                | ((value << LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRV_HS_OFFSET)
                    & LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRV_HS_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_FREQ1_DRV_LS** `[12:9]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq1_drv_ls(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRV_LS_MASK)
                | ((value << LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRV_LS_OFFSET)
                    & LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRV_LS_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_FREQ1_DRES_LPF** `[8:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq1_dres_lpf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRES_LPF_MASK)
                | ((value << LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRES_LPF_OFFSET)
                    & LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DRES_LPF_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_FREQ1_DCAP_LPF** `[6:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq1_dcap_lpf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DCAP_LPF_MASK)
                | ((value << LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DCAP_LPF_OFFSET)
                    & LpAnaTouchFreq1ScanPara::LP_ANA_TOUCH_FREQ1_DCAP_LPF_MASK),
        }
    }
}

impl Default for LpAnaTouchFreq1ScanParaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchFreq1ScanPara {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchFreq1ScanParaVal {
        LpAnaTouchFreq1ScanParaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchFreq1ScanParaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchFreq1ScanParaVal) -> LpAnaTouchFreq1ScanParaVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_FREQ2_SCAN_PARA Register
///
/// need_des
///
/// Address offset: 0x0134
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_FREQ2_DBIAS** `[22:18]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FREQ2_DRV_HS** `[17:13]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FREQ2_DRV_LS** `[12:9]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FREQ2_DRES_LPF** `[8:7]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FREQ2_DCAP_LPF** `[6:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchFreq2ScanPara {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchFreq2ScanPara {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchFreq2ScanPara {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_FREQ2_DBIAS**
    ///
    /// need_des
    ///
    /// Bits `[22:18]`, mask `0x007c0000`
    pub const LP_ANA_TOUCH_FREQ2_DBIAS_OFFSET: u32 = 18;
    pub const LP_ANA_TOUCH_FREQ2_DBIAS_MASK: u32 = 0x007c0000;

    /// **LP_ANA_TOUCH_FREQ2_DRV_HS**
    ///
    /// need_des
    ///
    /// Bits `[17:13]`, mask `0x0003e000`
    pub const LP_ANA_TOUCH_FREQ2_DRV_HS_OFFSET: u32 = 13;
    pub const LP_ANA_TOUCH_FREQ2_DRV_HS_MASK: u32 = 0x0003e000;

    /// **LP_ANA_TOUCH_FREQ2_DRV_LS**
    ///
    /// need_des
    ///
    /// Bits `[12:9]`, mask `0x00001e00`
    pub const LP_ANA_TOUCH_FREQ2_DRV_LS_OFFSET: u32 = 9;
    pub const LP_ANA_TOUCH_FREQ2_DRV_LS_MASK: u32 = 0x00001e00;

    /// **LP_ANA_TOUCH_FREQ2_DRES_LPF**
    ///
    /// need_des
    ///
    /// Bits `[8:7]`, mask `0x00000180`
    pub const LP_ANA_TOUCH_FREQ2_DRES_LPF_OFFSET: u32 = 7;
    pub const LP_ANA_TOUCH_FREQ2_DRES_LPF_MASK: u32 = 0x00000180;

    /// **LP_ANA_TOUCH_FREQ2_DCAP_LPF**
    ///
    /// need_des
    ///
    /// Bits `[6:0]`, mask `0x0000007f`
    pub const LP_ANA_TOUCH_FREQ2_DCAP_LPF_OFFSET: u32 = 0;
    pub const LP_ANA_TOUCH_FREQ2_DCAP_LPF_MASK: u32 = 0x0000007f;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_FREQ2_DBIAS** `[22:18]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq2_dbias(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ2_DBIAS_MASK)
            >> Self::LP_ANA_TOUCH_FREQ2_DBIAS_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ2_DBIAS** `[22:18]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq2_dbias(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ2_DBIAS_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ2_DBIAS_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ2_DBIAS_MASK)
        });
    }

    /// **LP_ANA_TOUCH_FREQ2_DRV_HS** `[17:13]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq2_drv_hs(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ2_DRV_HS_MASK)
            >> Self::LP_ANA_TOUCH_FREQ2_DRV_HS_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ2_DRV_HS** `[17:13]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq2_drv_hs(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ2_DRV_HS_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ2_DRV_HS_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ2_DRV_HS_MASK)
        });
    }

    /// **LP_ANA_TOUCH_FREQ2_DRV_LS** `[12:9]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq2_drv_ls(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ2_DRV_LS_MASK)
            >> Self::LP_ANA_TOUCH_FREQ2_DRV_LS_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ2_DRV_LS** `[12:9]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq2_drv_ls(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ2_DRV_LS_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ2_DRV_LS_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ2_DRV_LS_MASK)
        });
    }

    /// **LP_ANA_TOUCH_FREQ2_DRES_LPF** `[8:7]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq2_dres_lpf(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ2_DRES_LPF_MASK)
            >> Self::LP_ANA_TOUCH_FREQ2_DRES_LPF_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ2_DRES_LPF** `[8:7]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq2_dres_lpf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ2_DRES_LPF_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ2_DRES_LPF_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ2_DRES_LPF_MASK)
        });
    }

    /// **LP_ANA_TOUCH_FREQ2_DCAP_LPF** `[6:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq2_dcap_lpf(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ2_DCAP_LPF_MASK)
            >> Self::LP_ANA_TOUCH_FREQ2_DCAP_LPF_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ2_DCAP_LPF** `[6:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq2_dcap_lpf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ2_DCAP_LPF_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ2_DCAP_LPF_OFFSET)
                    & Self::LP_ANA_TOUCH_FREQ2_DCAP_LPF_MASK)
        });
    }
}

/// LP_ANA_TOUCH_FREQ2_SCAN_PARA value type for batch read/modify/write operations
///
/// Use `lpanatouchfreq2scanpara.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchfreq2scanpara.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchFreq2ScanParaVal {
    bits: u32,
}

impl LpAnaTouchFreq2ScanParaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_FREQ2_DBIAS** `[22:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq2_dbias(&self) -> u32 {
        (self.bits & LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DBIAS_MASK)
            >> LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DBIAS_OFFSET
    }

    /// Read **LP_ANA_TOUCH_FREQ2_DRV_HS** `[17:13]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq2_drv_hs(&self) -> u32 {
        (self.bits & LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRV_HS_MASK)
            >> LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRV_HS_OFFSET
    }

    /// Read **LP_ANA_TOUCH_FREQ2_DRV_LS** `[12:9]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq2_drv_ls(&self) -> u32 {
        (self.bits & LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRV_LS_MASK)
            >> LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRV_LS_OFFSET
    }

    /// Read **LP_ANA_TOUCH_FREQ2_DRES_LPF** `[8:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq2_dres_lpf(&self) -> u32 {
        (self.bits & LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRES_LPF_MASK)
            >> LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRES_LPF_OFFSET
    }

    /// Read **LP_ANA_TOUCH_FREQ2_DCAP_LPF** `[6:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq2_dcap_lpf(&self) -> u32 {
        (self.bits & LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DCAP_LPF_MASK)
            >> LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DCAP_LPF_OFFSET
    }

    /// Set **LP_ANA_TOUCH_FREQ2_DBIAS** `[22:18]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq2_dbias(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DBIAS_MASK)
                | ((value << LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DBIAS_OFFSET)
                    & LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DBIAS_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_FREQ2_DRV_HS** `[17:13]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq2_drv_hs(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRV_HS_MASK)
                | ((value << LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRV_HS_OFFSET)
                    & LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRV_HS_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_FREQ2_DRV_LS** `[12:9]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq2_drv_ls(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRV_LS_MASK)
                | ((value << LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRV_LS_OFFSET)
                    & LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRV_LS_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_FREQ2_DRES_LPF** `[8:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq2_dres_lpf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRES_LPF_MASK)
                | ((value << LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRES_LPF_OFFSET)
                    & LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DRES_LPF_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_FREQ2_DCAP_LPF** `[6:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq2_dcap_lpf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DCAP_LPF_MASK)
                | ((value << LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DCAP_LPF_OFFSET)
                    & LpAnaTouchFreq2ScanPara::LP_ANA_TOUCH_FREQ2_DCAP_LPF_MASK),
        }
    }
}

impl Default for LpAnaTouchFreq2ScanParaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchFreq2ScanPara {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchFreq2ScanParaVal {
        LpAnaTouchFreq2ScanParaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchFreq2ScanParaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchFreq2ScanParaVal) -> LpAnaTouchFreq2ScanParaVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_ANA_PARA Register
///
/// need_des
///
/// Address offset: 0x0138
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_TOUCH_DCAP_CAL** `[10:4]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_TOUCH_EN_CAL** `[3]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_TOUCH_BUF_DRV** `[2:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchAnaPara {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchAnaPara {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchAnaPara {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_TOUCH_DCAP_CAL**
    ///
    /// need_des
    ///
    /// Bits `[10:4]`, mask `0x000007f0`
    pub const LP_ANA_TOUCH_TOUCH_DCAP_CAL_OFFSET: u32 = 4;
    pub const LP_ANA_TOUCH_TOUCH_DCAP_CAL_MASK: u32 = 0x000007f0;

    /// **LP_ANA_TOUCH_TOUCH_EN_CAL**
    ///
    /// need_des
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LP_ANA_TOUCH_TOUCH_EN_CAL_OFFSET: u32 = 3;
    pub const LP_ANA_TOUCH_TOUCH_EN_CAL_MASK: u32 = 0x00000008;
    pub const LP_ANA_TOUCH_TOUCH_EN_CAL: u32 = 1 << 3;

    /// **LP_ANA_TOUCH_TOUCH_BUF_DRV**
    ///
    /// need_des
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const LP_ANA_TOUCH_TOUCH_BUF_DRV_OFFSET: u32 = 0;
    pub const LP_ANA_TOUCH_TOUCH_BUF_DRV_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_TOUCH_DCAP_CAL** `[10:4]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_touch_dcap_cal(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_TOUCH_DCAP_CAL_MASK)
            >> Self::LP_ANA_TOUCH_TOUCH_DCAP_CAL_OFFSET
    }

    /// Write **LP_ANA_TOUCH_TOUCH_DCAP_CAL** `[10:4]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_touch_dcap_cal(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_TOUCH_DCAP_CAL_MASK)
                | ((value << Self::LP_ANA_TOUCH_TOUCH_DCAP_CAL_OFFSET)
                    & Self::LP_ANA_TOUCH_TOUCH_DCAP_CAL_MASK)
        });
    }

    /// **LP_ANA_TOUCH_TOUCH_EN_CAL** `[3]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_touch_en_cal(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_TOUCH_EN_CAL_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_TOUCH_EN_CAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_touch_en_cal(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_TOUCH_EN_CAL_MASK);
    }

    /// Clear LP_ANA_TOUCH_TOUCH_EN_CAL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_touch_en_cal(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_TOUCH_EN_CAL_MASK);
    }

    /// **LP_ANA_TOUCH_TOUCH_BUF_DRV** `[2:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_touch_buf_drv(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_TOUCH_BUF_DRV_MASK)
            >> Self::LP_ANA_TOUCH_TOUCH_BUF_DRV_OFFSET
    }

    /// Write **LP_ANA_TOUCH_TOUCH_BUF_DRV** `[2:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_touch_buf_drv(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_TOUCH_BUF_DRV_MASK)
                | ((value << Self::LP_ANA_TOUCH_TOUCH_BUF_DRV_OFFSET)
                    & Self::LP_ANA_TOUCH_TOUCH_BUF_DRV_MASK)
        });
    }
}

/// LP_ANA_TOUCH_ANA_PARA value type for batch read/modify/write operations
///
/// Use `lpanatouchanapara.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchanapara.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchAnaParaVal {
    bits: u32,
}

impl LpAnaTouchAnaParaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_TOUCH_DCAP_CAL** `[10:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_touch_dcap_cal(&self) -> u32 {
        (self.bits & LpAnaTouchAnaPara::LP_ANA_TOUCH_TOUCH_DCAP_CAL_MASK)
            >> LpAnaTouchAnaPara::LP_ANA_TOUCH_TOUCH_DCAP_CAL_OFFSET
    }

    /// Read **LP_ANA_TOUCH_TOUCH_EN_CAL** `[3:3]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_touch_en_cal(&self) -> bool {
        (self.bits & LpAnaTouchAnaPara::LP_ANA_TOUCH_TOUCH_EN_CAL_MASK) != 0
    }

    /// Read **LP_ANA_TOUCH_TOUCH_BUF_DRV** `[2:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_touch_buf_drv(&self) -> u32 {
        (self.bits & LpAnaTouchAnaPara::LP_ANA_TOUCH_TOUCH_BUF_DRV_MASK)
            >> LpAnaTouchAnaPara::LP_ANA_TOUCH_TOUCH_BUF_DRV_OFFSET
    }

    /// Set **LP_ANA_TOUCH_TOUCH_DCAP_CAL** `[10:4]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_touch_dcap_cal(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchAnaPara::LP_ANA_TOUCH_TOUCH_DCAP_CAL_MASK)
                | ((value << LpAnaTouchAnaPara::LP_ANA_TOUCH_TOUCH_DCAP_CAL_OFFSET)
                    & LpAnaTouchAnaPara::LP_ANA_TOUCH_TOUCH_DCAP_CAL_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_TOUCH_EN_CAL** `[3:3]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_touch_en_cal(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchAnaPara::LP_ANA_TOUCH_TOUCH_EN_CAL_MASK
            } else {
                self.bits & !LpAnaTouchAnaPara::LP_ANA_TOUCH_TOUCH_EN_CAL_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_TOUCH_BUF_DRV** `[2:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_touch_buf_drv(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchAnaPara::LP_ANA_TOUCH_TOUCH_BUF_DRV_MASK)
                | ((value << LpAnaTouchAnaPara::LP_ANA_TOUCH_TOUCH_BUF_DRV_OFFSET)
                    & LpAnaTouchAnaPara::LP_ANA_TOUCH_TOUCH_BUF_DRV_MASK),
        }
    }
}

impl Default for LpAnaTouchAnaParaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchAnaPara {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchAnaParaVal {
        LpAnaTouchAnaParaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchAnaParaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchAnaParaVal) -> LpAnaTouchAnaParaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_MUX0 Register
///
/// need_des
///
/// Address offset: 0x013c
/// Reset value: 0x20000000
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_START_FORCE** `[31]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_START_EN** `[30]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FSM_EN** `[29]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_DONE_FORCE** `[28]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_DONE_EN** `[27]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_BUFSEL** `[26:12]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_FREQ_SEL** `[11:10]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_DATA_SEL** `[9:8]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchMux0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchMux0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchMux0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_START_FORCE**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_TOUCH_START_FORCE_OFFSET: u32 = 31;
    pub const LP_ANA_TOUCH_START_FORCE_MASK: u32 = 0x80000000;
    pub const LP_ANA_TOUCH_START_FORCE: u32 = 1 << 31;

    /// **LP_ANA_TOUCH_START_EN**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_ANA_TOUCH_START_EN_OFFSET: u32 = 30;
    pub const LP_ANA_TOUCH_START_EN_MASK: u32 = 0x40000000;
    pub const LP_ANA_TOUCH_START_EN: u32 = 1 << 30;

    /// **LP_ANA_TOUCH_FSM_EN**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_ANA_TOUCH_FSM_EN_OFFSET: u32 = 29;
    pub const LP_ANA_TOUCH_FSM_EN_MASK: u32 = 0x20000000;
    pub const LP_ANA_TOUCH_FSM_EN: u32 = 1 << 29;

    /// **LP_ANA_TOUCH_DONE_FORCE**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const LP_ANA_TOUCH_DONE_FORCE_OFFSET: u32 = 28;
    pub const LP_ANA_TOUCH_DONE_FORCE_MASK: u32 = 0x10000000;
    pub const LP_ANA_TOUCH_DONE_FORCE: u32 = 1 << 28;

    /// **LP_ANA_TOUCH_DONE_EN**
    ///
    /// need_des
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const LP_ANA_TOUCH_DONE_EN_OFFSET: u32 = 27;
    pub const LP_ANA_TOUCH_DONE_EN_MASK: u32 = 0x08000000;
    pub const LP_ANA_TOUCH_DONE_EN: u32 = 1 << 27;

    /// **LP_ANA_TOUCH_BUFSEL**
    ///
    /// need_des
    ///
    /// Bits `[26:12]`, mask `0x07fff000`
    pub const LP_ANA_TOUCH_BUFSEL_OFFSET: u32 = 12;
    pub const LP_ANA_TOUCH_BUFSEL_MASK: u32 = 0x07fff000;

    /// **LP_ANA_TOUCH_FREQ_SEL**
    ///
    /// need_des
    ///
    /// Bits `[11:10]`, mask `0x00000c00`
    pub const LP_ANA_TOUCH_FREQ_SEL_OFFSET: u32 = 10;
    pub const LP_ANA_TOUCH_FREQ_SEL_MASK: u32 = 0x00000c00;

    /// **LP_ANA_TOUCH_DATA_SEL**
    ///
    /// need_des
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const LP_ANA_TOUCH_DATA_SEL_OFFSET: u32 = 8;
    pub const LP_ANA_TOUCH_DATA_SEL_MASK: u32 = 0x00000300;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_START_FORCE** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_start_force(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_START_FORCE_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_START_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_start_force(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_START_FORCE_MASK);
    }

    /// Clear LP_ANA_TOUCH_START_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_start_force(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_START_FORCE_MASK);
    }

    /// **LP_ANA_TOUCH_START_EN** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_start_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_START_EN_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_START_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_start_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_START_EN_MASK);
    }

    /// Clear LP_ANA_TOUCH_START_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_start_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_START_EN_MASK);
    }

    /// **LP_ANA_TOUCH_FSM_EN** `[29]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_fsm_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FSM_EN_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_FSM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_fsm_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_FSM_EN_MASK);
    }

    /// Clear LP_ANA_TOUCH_FSM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_fsm_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_FSM_EN_MASK);
    }

    /// **LP_ANA_TOUCH_DONE_FORCE** `[28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_done_force(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_DONE_FORCE_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_DONE_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_done_force(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_DONE_FORCE_MASK);
    }

    /// Clear LP_ANA_TOUCH_DONE_FORCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_done_force(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_DONE_FORCE_MASK);
    }

    /// **LP_ANA_TOUCH_DONE_EN** `[27]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_done_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_DONE_EN_MASK) != 0
    }

    /// Set LP_ANA_TOUCH_DONE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_touch_done_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ANA_TOUCH_DONE_EN_MASK);
    }

    /// Clear LP_ANA_TOUCH_DONE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_touch_done_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ANA_TOUCH_DONE_EN_MASK);
    }

    /// **LP_ANA_TOUCH_BUFSEL** `[26:12]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_bufsel(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_BUFSEL_MASK)
            >> Self::LP_ANA_TOUCH_BUFSEL_OFFSET
    }

    /// Write **LP_ANA_TOUCH_BUFSEL** `[26:12]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_bufsel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_BUFSEL_MASK)
                | ((value << Self::LP_ANA_TOUCH_BUFSEL_OFFSET) & Self::LP_ANA_TOUCH_BUFSEL_MASK)
        });
    }

    /// **LP_ANA_TOUCH_FREQ_SEL** `[11:10]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_freq_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_FREQ_SEL_MASK)
            >> Self::LP_ANA_TOUCH_FREQ_SEL_OFFSET
    }

    /// Write **LP_ANA_TOUCH_FREQ_SEL** `[11:10]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_freq_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_FREQ_SEL_MASK)
                | ((value << Self::LP_ANA_TOUCH_FREQ_SEL_OFFSET) & Self::LP_ANA_TOUCH_FREQ_SEL_MASK)
        });
    }

    /// **LP_ANA_TOUCH_DATA_SEL** `[9:8]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_data_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_DATA_SEL_MASK)
            >> Self::LP_ANA_TOUCH_DATA_SEL_OFFSET
    }

    /// Write **LP_ANA_TOUCH_DATA_SEL** `[9:8]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_data_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_DATA_SEL_MASK)
                | ((value << Self::LP_ANA_TOUCH_DATA_SEL_OFFSET) & Self::LP_ANA_TOUCH_DATA_SEL_MASK)
        });
    }
}

/// LP_ANA_TOUCH_MUX0 value type for batch read/modify/write operations
///
/// Use `lpanatouchmux0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchmux0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchMux0Val {
    bits: u32,
}

impl LpAnaTouchMux0Val {
    /// Create from raw bits
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
        Self { bits: 0x20000000 }
    }

    /// Read **LP_ANA_TOUCH_START_FORCE** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_start_force(&self) -> bool {
        (self.bits & LpAnaTouchMux0::LP_ANA_TOUCH_START_FORCE_MASK) != 0
    }

    /// Read **LP_ANA_TOUCH_START_EN** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_start_en(&self) -> bool {
        (self.bits & LpAnaTouchMux0::LP_ANA_TOUCH_START_EN_MASK) != 0
    }

    /// Read **LP_ANA_TOUCH_FSM_EN** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_fsm_en(&self) -> bool {
        (self.bits & LpAnaTouchMux0::LP_ANA_TOUCH_FSM_EN_MASK) != 0
    }

    /// Read **LP_ANA_TOUCH_DONE_FORCE** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_done_force(&self) -> bool {
        (self.bits & LpAnaTouchMux0::LP_ANA_TOUCH_DONE_FORCE_MASK) != 0
    }

    /// Read **LP_ANA_TOUCH_DONE_EN** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_done_en(&self) -> bool {
        (self.bits & LpAnaTouchMux0::LP_ANA_TOUCH_DONE_EN_MASK) != 0
    }

    /// Read **LP_ANA_TOUCH_BUFSEL** `[26:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_bufsel(&self) -> u32 {
        (self.bits & LpAnaTouchMux0::LP_ANA_TOUCH_BUFSEL_MASK)
            >> LpAnaTouchMux0::LP_ANA_TOUCH_BUFSEL_OFFSET
    }

    /// Read **LP_ANA_TOUCH_FREQ_SEL** `[11:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_freq_sel(&self) -> u32 {
        (self.bits & LpAnaTouchMux0::LP_ANA_TOUCH_FREQ_SEL_MASK)
            >> LpAnaTouchMux0::LP_ANA_TOUCH_FREQ_SEL_OFFSET
    }

    /// Read **LP_ANA_TOUCH_DATA_SEL** `[9:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_data_sel(&self) -> u32 {
        (self.bits & LpAnaTouchMux0::LP_ANA_TOUCH_DATA_SEL_MASK)
            >> LpAnaTouchMux0::LP_ANA_TOUCH_DATA_SEL_OFFSET
    }

    /// Set **LP_ANA_TOUCH_START_FORCE** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_start_force(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchMux0::LP_ANA_TOUCH_START_FORCE_MASK
            } else {
                self.bits & !LpAnaTouchMux0::LP_ANA_TOUCH_START_FORCE_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_START_EN** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_start_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchMux0::LP_ANA_TOUCH_START_EN_MASK
            } else {
                self.bits & !LpAnaTouchMux0::LP_ANA_TOUCH_START_EN_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_FSM_EN** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_fsm_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchMux0::LP_ANA_TOUCH_FSM_EN_MASK
            } else {
                self.bits & !LpAnaTouchMux0::LP_ANA_TOUCH_FSM_EN_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_DONE_FORCE** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_done_force(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchMux0::LP_ANA_TOUCH_DONE_FORCE_MASK
            } else {
                self.bits & !LpAnaTouchMux0::LP_ANA_TOUCH_DONE_FORCE_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_DONE_EN** `[27:27]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_done_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaTouchMux0::LP_ANA_TOUCH_DONE_EN_MASK
            } else {
                self.bits & !LpAnaTouchMux0::LP_ANA_TOUCH_DONE_EN_MASK
            },
        }
    }

    /// Set **LP_ANA_TOUCH_BUFSEL** `[26:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_bufsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchMux0::LP_ANA_TOUCH_BUFSEL_MASK)
                | ((value << LpAnaTouchMux0::LP_ANA_TOUCH_BUFSEL_OFFSET)
                    & LpAnaTouchMux0::LP_ANA_TOUCH_BUFSEL_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_FREQ_SEL** `[11:10]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_freq_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchMux0::LP_ANA_TOUCH_FREQ_SEL_MASK)
                | ((value << LpAnaTouchMux0::LP_ANA_TOUCH_FREQ_SEL_OFFSET)
                    & LpAnaTouchMux0::LP_ANA_TOUCH_FREQ_SEL_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_DATA_SEL** `[9:8]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_data_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchMux0::LP_ANA_TOUCH_DATA_SEL_MASK)
                | ((value << LpAnaTouchMux0::LP_ANA_TOUCH_DATA_SEL_OFFSET)
                    & LpAnaTouchMux0::LP_ANA_TOUCH_DATA_SEL_MASK),
        }
    }
}

impl Default for LpAnaTouchMux0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaTouchMux0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchMux0Val {
        LpAnaTouchMux0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchMux0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchMux0Val) -> LpAnaTouchMux0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_MUX1 Register
///
/// need_des
///
/// Address offset: 0x0140
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_XPD** `[29:15]` (RW)
///
///   need_des
///
/// - **LP_ANA_TOUCH_START** `[14:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaTouchMux1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchMux1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchMux1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_XPD**
    ///
    /// need_des
    ///
    /// Bits `[29:15]`, mask `0x3fff8000`
    pub const LP_ANA_TOUCH_XPD_OFFSET: u32 = 15;
    pub const LP_ANA_TOUCH_XPD_MASK: u32 = 0x3fff8000;

    /// **LP_ANA_TOUCH_START**
    ///
    /// need_des
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const LP_ANA_TOUCH_START_OFFSET: u32 = 0;
    pub const LP_ANA_TOUCH_START_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_XPD** `[29:15]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_xpd(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_XPD_MASK) >> Self::LP_ANA_TOUCH_XPD_OFFSET
    }

    /// Write **LP_ANA_TOUCH_XPD** `[29:15]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_xpd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_XPD_MASK)
                | ((value << Self::LP_ANA_TOUCH_XPD_OFFSET) & Self::LP_ANA_TOUCH_XPD_MASK)
        });
    }

    /// **LP_ANA_TOUCH_START** `[14:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_touch_start(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_START_MASK) >> Self::LP_ANA_TOUCH_START_OFFSET
    }

    /// Write **LP_ANA_TOUCH_START** `[14:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_touch_start(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_START_MASK)
                | ((value << Self::LP_ANA_TOUCH_START_OFFSET) & Self::LP_ANA_TOUCH_START_MASK)
        });
    }
}

/// LP_ANA_TOUCH_MUX1 value type for batch read/modify/write operations
///
/// Use `lpanatouchmux1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchmux1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchMux1Val {
    bits: u32,
}

impl LpAnaTouchMux1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_XPD** `[29:15]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_xpd(&self) -> u32 {
        (self.bits & LpAnaTouchMux1::LP_ANA_TOUCH_XPD_MASK)
            >> LpAnaTouchMux1::LP_ANA_TOUCH_XPD_OFFSET
    }

    /// Read **LP_ANA_TOUCH_START** `[14:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_touch_start(&self) -> u32 {
        (self.bits & LpAnaTouchMux1::LP_ANA_TOUCH_START_MASK)
            >> LpAnaTouchMux1::LP_ANA_TOUCH_START_OFFSET
    }

    /// Set **LP_ANA_TOUCH_XPD** `[29:15]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_xpd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchMux1::LP_ANA_TOUCH_XPD_MASK)
                | ((value << LpAnaTouchMux1::LP_ANA_TOUCH_XPD_OFFSET)
                    & LpAnaTouchMux1::LP_ANA_TOUCH_XPD_MASK),
        }
    }

    /// Set **LP_ANA_TOUCH_START** `[14:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_touch_start(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchMux1::LP_ANA_TOUCH_START_MASK)
                | ((value << LpAnaTouchMux1::LP_ANA_TOUCH_START_OFFSET)
                    & LpAnaTouchMux1::LP_ANA_TOUCH_START_MASK),
        }
    }
}

impl Default for LpAnaTouchMux1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchMux1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchMux1Val {
        LpAnaTouchMux1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchMux1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchMux1Val) -> LpAnaTouchMux1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD0_TH0 Register
///
/// need_des
///
/// Address offset: 0x0144
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD0_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad0Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad0Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad0Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD0_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD0_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD0_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD0_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad0_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD0_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD0_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD0_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad0_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD0_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD0_TH0_OFFSET) & Self::LP_ANA_TOUCH_PAD0_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD0_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad0th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad0th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad0Th0Val {
    bits: u32,
}

impl LpAnaTouchPad0Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD0_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad0_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad0Th0::LP_ANA_TOUCH_PAD0_TH0_MASK)
            >> LpAnaTouchPad0Th0::LP_ANA_TOUCH_PAD0_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD0_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad0_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad0Th0::LP_ANA_TOUCH_PAD0_TH0_MASK)
                | ((value << LpAnaTouchPad0Th0::LP_ANA_TOUCH_PAD0_TH0_OFFSET)
                    & LpAnaTouchPad0Th0::LP_ANA_TOUCH_PAD0_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad0Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad0Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad0Th0Val {
        LpAnaTouchPad0Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad0Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad0Th0Val) -> LpAnaTouchPad0Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD0_TH1 Register
///
/// need_des
///
/// Address offset: 0x0148
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD0_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad0Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad0Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad0Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD0_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD0_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD0_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD0_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad0_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD0_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD0_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD0_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad0_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD0_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD0_TH1_OFFSET) & Self::LP_ANA_TOUCH_PAD0_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD0_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad0th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad0th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad0Th1Val {
    bits: u32,
}

impl LpAnaTouchPad0Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD0_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad0_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad0Th1::LP_ANA_TOUCH_PAD0_TH1_MASK)
            >> LpAnaTouchPad0Th1::LP_ANA_TOUCH_PAD0_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD0_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad0_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad0Th1::LP_ANA_TOUCH_PAD0_TH1_MASK)
                | ((value << LpAnaTouchPad0Th1::LP_ANA_TOUCH_PAD0_TH1_OFFSET)
                    & LpAnaTouchPad0Th1::LP_ANA_TOUCH_PAD0_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad0Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad0Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad0Th1Val {
        LpAnaTouchPad0Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad0Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad0Th1Val) -> LpAnaTouchPad0Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD0_TH2 Register
///
/// need_des
///
/// Address offset: 0x014c
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD0_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad0Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad0Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad0Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD0_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD0_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD0_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD0_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad0_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD0_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD0_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD0_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad0_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD0_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD0_TH2_OFFSET) & Self::LP_ANA_TOUCH_PAD0_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD0_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad0th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad0th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad0Th2Val {
    bits: u32,
}

impl LpAnaTouchPad0Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD0_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad0_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad0Th2::LP_ANA_TOUCH_PAD0_TH2_MASK)
            >> LpAnaTouchPad0Th2::LP_ANA_TOUCH_PAD0_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD0_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad0_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad0Th2::LP_ANA_TOUCH_PAD0_TH2_MASK)
                | ((value << LpAnaTouchPad0Th2::LP_ANA_TOUCH_PAD0_TH2_OFFSET)
                    & LpAnaTouchPad0Th2::LP_ANA_TOUCH_PAD0_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad0Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad0Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad0Th2Val {
        LpAnaTouchPad0Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad0Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad0Th2Val) -> LpAnaTouchPad0Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD1_TH0 Register
///
/// need_des
///
/// Address offset: 0x0150
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD1_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad1Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad1Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad1Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD1_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD1_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD1_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD1_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad1_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD1_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD1_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD1_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad1_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD1_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD1_TH0_OFFSET) & Self::LP_ANA_TOUCH_PAD1_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD1_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad1th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad1th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad1Th0Val {
    bits: u32,
}

impl LpAnaTouchPad1Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD1_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad1_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad1Th0::LP_ANA_TOUCH_PAD1_TH0_MASK)
            >> LpAnaTouchPad1Th0::LP_ANA_TOUCH_PAD1_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD1_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad1_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad1Th0::LP_ANA_TOUCH_PAD1_TH0_MASK)
                | ((value << LpAnaTouchPad1Th0::LP_ANA_TOUCH_PAD1_TH0_OFFSET)
                    & LpAnaTouchPad1Th0::LP_ANA_TOUCH_PAD1_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad1Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad1Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad1Th0Val {
        LpAnaTouchPad1Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad1Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad1Th0Val) -> LpAnaTouchPad1Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD1_TH1 Register
///
/// need_des
///
/// Address offset: 0x0154
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD1_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad1Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad1Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad1Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD1_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD1_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD1_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD1_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad1_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD1_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD1_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD1_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad1_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD1_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD1_TH1_OFFSET) & Self::LP_ANA_TOUCH_PAD1_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD1_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad1th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad1th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad1Th1Val {
    bits: u32,
}

impl LpAnaTouchPad1Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD1_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad1_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad1Th1::LP_ANA_TOUCH_PAD1_TH1_MASK)
            >> LpAnaTouchPad1Th1::LP_ANA_TOUCH_PAD1_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD1_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad1_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad1Th1::LP_ANA_TOUCH_PAD1_TH1_MASK)
                | ((value << LpAnaTouchPad1Th1::LP_ANA_TOUCH_PAD1_TH1_OFFSET)
                    & LpAnaTouchPad1Th1::LP_ANA_TOUCH_PAD1_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad1Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad1Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad1Th1Val {
        LpAnaTouchPad1Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad1Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad1Th1Val) -> LpAnaTouchPad1Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD1_TH2 Register
///
/// need_des
///
/// Address offset: 0x0158
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD1_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad1Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad1Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad1Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD1_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD1_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD1_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD1_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad1_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD1_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD1_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD1_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad1_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD1_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD1_TH2_OFFSET) & Self::LP_ANA_TOUCH_PAD1_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD1_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad1th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad1th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad1Th2Val {
    bits: u32,
}

impl LpAnaTouchPad1Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD1_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad1_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad1Th2::LP_ANA_TOUCH_PAD1_TH2_MASK)
            >> LpAnaTouchPad1Th2::LP_ANA_TOUCH_PAD1_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD1_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad1_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad1Th2::LP_ANA_TOUCH_PAD1_TH2_MASK)
                | ((value << LpAnaTouchPad1Th2::LP_ANA_TOUCH_PAD1_TH2_OFFSET)
                    & LpAnaTouchPad1Th2::LP_ANA_TOUCH_PAD1_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad1Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad1Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad1Th2Val {
        LpAnaTouchPad1Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad1Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad1Th2Val) -> LpAnaTouchPad1Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD2_TH0 Register
///
/// need_des
///
/// Address offset: 0x015c
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD2_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad2Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad2Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad2Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD2_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD2_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD2_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD2_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad2_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD2_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD2_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD2_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad2_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD2_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD2_TH0_OFFSET) & Self::LP_ANA_TOUCH_PAD2_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD2_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad2th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad2th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad2Th0Val {
    bits: u32,
}

impl LpAnaTouchPad2Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD2_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad2_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad2Th0::LP_ANA_TOUCH_PAD2_TH0_MASK)
            >> LpAnaTouchPad2Th0::LP_ANA_TOUCH_PAD2_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD2_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad2_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad2Th0::LP_ANA_TOUCH_PAD2_TH0_MASK)
                | ((value << LpAnaTouchPad2Th0::LP_ANA_TOUCH_PAD2_TH0_OFFSET)
                    & LpAnaTouchPad2Th0::LP_ANA_TOUCH_PAD2_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad2Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad2Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad2Th0Val {
        LpAnaTouchPad2Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad2Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad2Th0Val) -> LpAnaTouchPad2Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD2_TH1 Register
///
/// need_des
///
/// Address offset: 0x0160
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD2_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad2Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad2Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad2Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD2_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD2_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD2_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD2_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad2_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD2_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD2_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD2_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad2_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD2_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD2_TH1_OFFSET) & Self::LP_ANA_TOUCH_PAD2_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD2_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad2th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad2th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad2Th1Val {
    bits: u32,
}

impl LpAnaTouchPad2Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD2_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad2_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad2Th1::LP_ANA_TOUCH_PAD2_TH1_MASK)
            >> LpAnaTouchPad2Th1::LP_ANA_TOUCH_PAD2_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD2_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad2_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad2Th1::LP_ANA_TOUCH_PAD2_TH1_MASK)
                | ((value << LpAnaTouchPad2Th1::LP_ANA_TOUCH_PAD2_TH1_OFFSET)
                    & LpAnaTouchPad2Th1::LP_ANA_TOUCH_PAD2_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad2Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad2Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad2Th1Val {
        LpAnaTouchPad2Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad2Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad2Th1Val) -> LpAnaTouchPad2Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD2_TH2 Register
///
/// need_des
///
/// Address offset: 0x0164
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD2_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad2Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad2Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad2Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD2_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD2_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD2_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD2_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad2_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD2_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD2_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD2_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad2_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD2_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD2_TH2_OFFSET) & Self::LP_ANA_TOUCH_PAD2_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD2_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad2th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad2th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad2Th2Val {
    bits: u32,
}

impl LpAnaTouchPad2Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD2_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad2_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad2Th2::LP_ANA_TOUCH_PAD2_TH2_MASK)
            >> LpAnaTouchPad2Th2::LP_ANA_TOUCH_PAD2_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD2_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad2_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad2Th2::LP_ANA_TOUCH_PAD2_TH2_MASK)
                | ((value << LpAnaTouchPad2Th2::LP_ANA_TOUCH_PAD2_TH2_OFFSET)
                    & LpAnaTouchPad2Th2::LP_ANA_TOUCH_PAD2_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad2Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad2Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad2Th2Val {
        LpAnaTouchPad2Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad2Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad2Th2Val) -> LpAnaTouchPad2Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD3_TH0 Register
///
/// need_des
///
/// Address offset: 0x0168
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD3_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad3Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad3Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad3Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD3_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD3_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD3_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD3_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad3_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD3_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD3_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD3_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad3_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD3_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD3_TH0_OFFSET) & Self::LP_ANA_TOUCH_PAD3_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD3_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad3th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad3th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad3Th0Val {
    bits: u32,
}

impl LpAnaTouchPad3Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD3_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad3_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad3Th0::LP_ANA_TOUCH_PAD3_TH0_MASK)
            >> LpAnaTouchPad3Th0::LP_ANA_TOUCH_PAD3_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD3_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad3_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad3Th0::LP_ANA_TOUCH_PAD3_TH0_MASK)
                | ((value << LpAnaTouchPad3Th0::LP_ANA_TOUCH_PAD3_TH0_OFFSET)
                    & LpAnaTouchPad3Th0::LP_ANA_TOUCH_PAD3_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad3Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad3Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad3Th0Val {
        LpAnaTouchPad3Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad3Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad3Th0Val) -> LpAnaTouchPad3Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD3_TH1 Register
///
/// need_des
///
/// Address offset: 0x016c
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD3_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad3Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad3Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad3Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD3_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD3_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD3_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD3_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad3_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD3_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD3_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD3_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad3_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD3_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD3_TH1_OFFSET) & Self::LP_ANA_TOUCH_PAD3_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD3_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad3th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad3th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad3Th1Val {
    bits: u32,
}

impl LpAnaTouchPad3Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD3_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad3_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad3Th1::LP_ANA_TOUCH_PAD3_TH1_MASK)
            >> LpAnaTouchPad3Th1::LP_ANA_TOUCH_PAD3_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD3_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad3_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad3Th1::LP_ANA_TOUCH_PAD3_TH1_MASK)
                | ((value << LpAnaTouchPad3Th1::LP_ANA_TOUCH_PAD3_TH1_OFFSET)
                    & LpAnaTouchPad3Th1::LP_ANA_TOUCH_PAD3_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad3Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad3Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad3Th1Val {
        LpAnaTouchPad3Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad3Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad3Th1Val) -> LpAnaTouchPad3Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD3_TH2 Register
///
/// need_des
///
/// Address offset: 0x0170
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD3_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad3Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad3Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad3Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD3_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD3_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD3_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD3_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad3_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD3_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD3_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD3_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad3_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD3_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD3_TH2_OFFSET) & Self::LP_ANA_TOUCH_PAD3_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD3_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad3th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad3th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad3Th2Val {
    bits: u32,
}

impl LpAnaTouchPad3Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD3_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad3_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad3Th2::LP_ANA_TOUCH_PAD3_TH2_MASK)
            >> LpAnaTouchPad3Th2::LP_ANA_TOUCH_PAD3_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD3_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad3_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad3Th2::LP_ANA_TOUCH_PAD3_TH2_MASK)
                | ((value << LpAnaTouchPad3Th2::LP_ANA_TOUCH_PAD3_TH2_OFFSET)
                    & LpAnaTouchPad3Th2::LP_ANA_TOUCH_PAD3_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad3Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad3Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad3Th2Val {
        LpAnaTouchPad3Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad3Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad3Th2Val) -> LpAnaTouchPad3Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD4_TH0 Register
///
/// need_des
///
/// Address offset: 0x0174
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD4_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad4Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad4Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad4Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD4_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD4_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD4_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD4_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad4_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD4_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD4_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD4_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad4_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD4_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD4_TH0_OFFSET) & Self::LP_ANA_TOUCH_PAD4_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD4_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad4th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad4th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad4Th0Val {
    bits: u32,
}

impl LpAnaTouchPad4Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD4_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad4_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad4Th0::LP_ANA_TOUCH_PAD4_TH0_MASK)
            >> LpAnaTouchPad4Th0::LP_ANA_TOUCH_PAD4_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD4_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad4_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad4Th0::LP_ANA_TOUCH_PAD4_TH0_MASK)
                | ((value << LpAnaTouchPad4Th0::LP_ANA_TOUCH_PAD4_TH0_OFFSET)
                    & LpAnaTouchPad4Th0::LP_ANA_TOUCH_PAD4_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad4Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad4Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad4Th0Val {
        LpAnaTouchPad4Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad4Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad4Th0Val) -> LpAnaTouchPad4Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD4_TH1 Register
///
/// need_des
///
/// Address offset: 0x0178
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD4_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad4Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad4Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad4Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD4_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD4_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD4_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD4_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad4_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD4_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD4_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD4_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad4_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD4_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD4_TH1_OFFSET) & Self::LP_ANA_TOUCH_PAD4_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD4_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad4th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad4th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad4Th1Val {
    bits: u32,
}

impl LpAnaTouchPad4Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD4_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad4_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad4Th1::LP_ANA_TOUCH_PAD4_TH1_MASK)
            >> LpAnaTouchPad4Th1::LP_ANA_TOUCH_PAD4_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD4_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad4_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad4Th1::LP_ANA_TOUCH_PAD4_TH1_MASK)
                | ((value << LpAnaTouchPad4Th1::LP_ANA_TOUCH_PAD4_TH1_OFFSET)
                    & LpAnaTouchPad4Th1::LP_ANA_TOUCH_PAD4_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad4Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad4Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad4Th1Val {
        LpAnaTouchPad4Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad4Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad4Th1Val) -> LpAnaTouchPad4Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD4_TH2 Register
///
/// need_des
///
/// Address offset: 0x017c
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD4_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad4Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad4Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad4Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD4_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD4_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD4_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD4_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad4_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD4_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD4_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD4_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad4_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD4_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD4_TH2_OFFSET) & Self::LP_ANA_TOUCH_PAD4_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD4_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad4th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad4th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad4Th2Val {
    bits: u32,
}

impl LpAnaTouchPad4Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD4_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad4_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad4Th2::LP_ANA_TOUCH_PAD4_TH2_MASK)
            >> LpAnaTouchPad4Th2::LP_ANA_TOUCH_PAD4_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD4_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad4_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad4Th2::LP_ANA_TOUCH_PAD4_TH2_MASK)
                | ((value << LpAnaTouchPad4Th2::LP_ANA_TOUCH_PAD4_TH2_OFFSET)
                    & LpAnaTouchPad4Th2::LP_ANA_TOUCH_PAD4_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad4Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad4Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad4Th2Val {
        LpAnaTouchPad4Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad4Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad4Th2Val) -> LpAnaTouchPad4Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD5_TH0 Register
///
/// need_des
///
/// Address offset: 0x0180
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD5_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad5Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad5Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad5Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD5_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD5_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD5_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD5_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad5_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD5_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD5_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD5_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad5_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD5_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD5_TH0_OFFSET) & Self::LP_ANA_TOUCH_PAD5_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD5_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad5th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad5th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad5Th0Val {
    bits: u32,
}

impl LpAnaTouchPad5Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD5_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad5_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad5Th0::LP_ANA_TOUCH_PAD5_TH0_MASK)
            >> LpAnaTouchPad5Th0::LP_ANA_TOUCH_PAD5_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD5_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad5_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad5Th0::LP_ANA_TOUCH_PAD5_TH0_MASK)
                | ((value << LpAnaTouchPad5Th0::LP_ANA_TOUCH_PAD5_TH0_OFFSET)
                    & LpAnaTouchPad5Th0::LP_ANA_TOUCH_PAD5_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad5Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad5Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad5Th0Val {
        LpAnaTouchPad5Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad5Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad5Th0Val) -> LpAnaTouchPad5Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD5_TH1 Register
///
/// need_des
///
/// Address offset: 0x0184
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD5_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad5Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad5Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad5Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD5_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD5_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD5_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD5_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad5_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD5_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD5_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD5_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad5_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD5_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD5_TH1_OFFSET) & Self::LP_ANA_TOUCH_PAD5_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD5_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad5th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad5th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad5Th1Val {
    bits: u32,
}

impl LpAnaTouchPad5Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD5_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad5_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad5Th1::LP_ANA_TOUCH_PAD5_TH1_MASK)
            >> LpAnaTouchPad5Th1::LP_ANA_TOUCH_PAD5_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD5_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad5_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad5Th1::LP_ANA_TOUCH_PAD5_TH1_MASK)
                | ((value << LpAnaTouchPad5Th1::LP_ANA_TOUCH_PAD5_TH1_OFFSET)
                    & LpAnaTouchPad5Th1::LP_ANA_TOUCH_PAD5_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad5Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad5Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad5Th1Val {
        LpAnaTouchPad5Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad5Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad5Th1Val) -> LpAnaTouchPad5Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD5_TH2 Register
///
/// need_des
///
/// Address offset: 0x0188
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD5_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad5Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad5Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad5Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD5_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD5_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD5_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD5_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad5_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD5_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD5_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD5_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad5_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD5_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD5_TH2_OFFSET) & Self::LP_ANA_TOUCH_PAD5_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD5_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad5th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad5th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad5Th2Val {
    bits: u32,
}

impl LpAnaTouchPad5Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD5_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad5_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad5Th2::LP_ANA_TOUCH_PAD5_TH2_MASK)
            >> LpAnaTouchPad5Th2::LP_ANA_TOUCH_PAD5_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD5_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad5_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad5Th2::LP_ANA_TOUCH_PAD5_TH2_MASK)
                | ((value << LpAnaTouchPad5Th2::LP_ANA_TOUCH_PAD5_TH2_OFFSET)
                    & LpAnaTouchPad5Th2::LP_ANA_TOUCH_PAD5_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad5Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad5Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad5Th2Val {
        LpAnaTouchPad5Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad5Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad5Th2Val) -> LpAnaTouchPad5Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD6_TH0 Register
///
/// need_des
///
/// Address offset: 0x018c
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD6_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad6Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad6Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad6Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD6_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD6_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD6_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD6_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad6_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD6_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD6_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD6_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad6_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD6_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD6_TH0_OFFSET) & Self::LP_ANA_TOUCH_PAD6_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD6_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad6th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad6th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad6Th0Val {
    bits: u32,
}

impl LpAnaTouchPad6Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD6_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad6_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad6Th0::LP_ANA_TOUCH_PAD6_TH0_MASK)
            >> LpAnaTouchPad6Th0::LP_ANA_TOUCH_PAD6_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD6_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad6_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad6Th0::LP_ANA_TOUCH_PAD6_TH0_MASK)
                | ((value << LpAnaTouchPad6Th0::LP_ANA_TOUCH_PAD6_TH0_OFFSET)
                    & LpAnaTouchPad6Th0::LP_ANA_TOUCH_PAD6_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad6Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad6Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad6Th0Val {
        LpAnaTouchPad6Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad6Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad6Th0Val) -> LpAnaTouchPad6Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD6_TH1 Register
///
/// need_des
///
/// Address offset: 0x0190
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD6_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad6Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad6Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad6Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD6_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD6_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD6_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD6_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad6_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD6_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD6_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD6_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad6_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD6_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD6_TH1_OFFSET) & Self::LP_ANA_TOUCH_PAD6_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD6_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad6th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad6th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad6Th1Val {
    bits: u32,
}

impl LpAnaTouchPad6Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD6_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad6_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad6Th1::LP_ANA_TOUCH_PAD6_TH1_MASK)
            >> LpAnaTouchPad6Th1::LP_ANA_TOUCH_PAD6_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD6_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad6_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad6Th1::LP_ANA_TOUCH_PAD6_TH1_MASK)
                | ((value << LpAnaTouchPad6Th1::LP_ANA_TOUCH_PAD6_TH1_OFFSET)
                    & LpAnaTouchPad6Th1::LP_ANA_TOUCH_PAD6_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad6Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad6Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad6Th1Val {
        LpAnaTouchPad6Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad6Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad6Th1Val) -> LpAnaTouchPad6Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD6_TH2 Register
///
/// need_des
///
/// Address offset: 0x0194
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD6_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad6Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad6Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad6Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD6_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD6_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD6_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD6_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad6_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD6_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD6_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD6_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad6_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD6_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD6_TH2_OFFSET) & Self::LP_ANA_TOUCH_PAD6_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD6_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad6th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad6th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad6Th2Val {
    bits: u32,
}

impl LpAnaTouchPad6Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD6_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad6_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad6Th2::LP_ANA_TOUCH_PAD6_TH2_MASK)
            >> LpAnaTouchPad6Th2::LP_ANA_TOUCH_PAD6_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD6_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad6_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad6Th2::LP_ANA_TOUCH_PAD6_TH2_MASK)
                | ((value << LpAnaTouchPad6Th2::LP_ANA_TOUCH_PAD6_TH2_OFFSET)
                    & LpAnaTouchPad6Th2::LP_ANA_TOUCH_PAD6_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad6Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad6Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad6Th2Val {
        LpAnaTouchPad6Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad6Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad6Th2Val) -> LpAnaTouchPad6Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD7_TH0 Register
///
/// need_des
///
/// Address offset: 0x0198
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD7_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad7Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad7Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad7Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD7_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD7_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD7_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD7_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad7_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD7_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD7_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD7_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad7_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD7_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD7_TH0_OFFSET) & Self::LP_ANA_TOUCH_PAD7_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD7_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad7th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad7th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad7Th0Val {
    bits: u32,
}

impl LpAnaTouchPad7Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD7_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad7_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad7Th0::LP_ANA_TOUCH_PAD7_TH0_MASK)
            >> LpAnaTouchPad7Th0::LP_ANA_TOUCH_PAD7_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD7_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad7_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad7Th0::LP_ANA_TOUCH_PAD7_TH0_MASK)
                | ((value << LpAnaTouchPad7Th0::LP_ANA_TOUCH_PAD7_TH0_OFFSET)
                    & LpAnaTouchPad7Th0::LP_ANA_TOUCH_PAD7_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad7Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad7Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad7Th0Val {
        LpAnaTouchPad7Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad7Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad7Th0Val) -> LpAnaTouchPad7Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD7_TH1 Register
///
/// need_des
///
/// Address offset: 0x019c
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD7_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad7Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad7Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad7Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD7_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD7_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD7_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD7_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad7_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD7_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD7_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD7_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad7_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD7_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD7_TH1_OFFSET) & Self::LP_ANA_TOUCH_PAD7_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD7_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad7th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad7th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad7Th1Val {
    bits: u32,
}

impl LpAnaTouchPad7Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD7_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad7_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad7Th1::LP_ANA_TOUCH_PAD7_TH1_MASK)
            >> LpAnaTouchPad7Th1::LP_ANA_TOUCH_PAD7_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD7_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad7_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad7Th1::LP_ANA_TOUCH_PAD7_TH1_MASK)
                | ((value << LpAnaTouchPad7Th1::LP_ANA_TOUCH_PAD7_TH1_OFFSET)
                    & LpAnaTouchPad7Th1::LP_ANA_TOUCH_PAD7_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad7Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad7Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad7Th1Val {
        LpAnaTouchPad7Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad7Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad7Th1Val) -> LpAnaTouchPad7Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD7_TH2 Register
///
/// need_des
///
/// Address offset: 0x01a0
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD7_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad7Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad7Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad7Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD7_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD7_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD7_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD7_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad7_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD7_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD7_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD7_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad7_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD7_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD7_TH2_OFFSET) & Self::LP_ANA_TOUCH_PAD7_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD7_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad7th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad7th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad7Th2Val {
    bits: u32,
}

impl LpAnaTouchPad7Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD7_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad7_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad7Th2::LP_ANA_TOUCH_PAD7_TH2_MASK)
            >> LpAnaTouchPad7Th2::LP_ANA_TOUCH_PAD7_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD7_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad7_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad7Th2::LP_ANA_TOUCH_PAD7_TH2_MASK)
                | ((value << LpAnaTouchPad7Th2::LP_ANA_TOUCH_PAD7_TH2_OFFSET)
                    & LpAnaTouchPad7Th2::LP_ANA_TOUCH_PAD7_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad7Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad7Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad7Th2Val {
        LpAnaTouchPad7Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad7Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad7Th2Val) -> LpAnaTouchPad7Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD8_TH0 Register
///
/// need_des
///
/// Address offset: 0x01a4
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD8_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad8Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad8Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad8Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD8_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD8_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD8_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD8_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad8_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD8_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD8_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD8_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad8_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD8_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD8_TH0_OFFSET) & Self::LP_ANA_TOUCH_PAD8_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD8_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad8th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad8th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad8Th0Val {
    bits: u32,
}

impl LpAnaTouchPad8Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD8_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad8_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad8Th0::LP_ANA_TOUCH_PAD8_TH0_MASK)
            >> LpAnaTouchPad8Th0::LP_ANA_TOUCH_PAD8_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD8_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad8_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad8Th0::LP_ANA_TOUCH_PAD8_TH0_MASK)
                | ((value << LpAnaTouchPad8Th0::LP_ANA_TOUCH_PAD8_TH0_OFFSET)
                    & LpAnaTouchPad8Th0::LP_ANA_TOUCH_PAD8_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad8Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad8Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad8Th0Val {
        LpAnaTouchPad8Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad8Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad8Th0Val) -> LpAnaTouchPad8Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD8_TH1 Register
///
/// need_des
///
/// Address offset: 0x01a8
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD8_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad8Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad8Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad8Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD8_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD8_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD8_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD8_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad8_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD8_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD8_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD8_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad8_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD8_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD8_TH1_OFFSET) & Self::LP_ANA_TOUCH_PAD8_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD8_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad8th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad8th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad8Th1Val {
    bits: u32,
}

impl LpAnaTouchPad8Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD8_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad8_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad8Th1::LP_ANA_TOUCH_PAD8_TH1_MASK)
            >> LpAnaTouchPad8Th1::LP_ANA_TOUCH_PAD8_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD8_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad8_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad8Th1::LP_ANA_TOUCH_PAD8_TH1_MASK)
                | ((value << LpAnaTouchPad8Th1::LP_ANA_TOUCH_PAD8_TH1_OFFSET)
                    & LpAnaTouchPad8Th1::LP_ANA_TOUCH_PAD8_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad8Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad8Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad8Th1Val {
        LpAnaTouchPad8Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad8Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad8Th1Val) -> LpAnaTouchPad8Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD8_TH2 Register
///
/// need_des
///
/// Address offset: 0x01ac
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD8_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad8Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad8Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad8Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD8_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD8_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD8_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD8_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad8_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD8_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD8_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD8_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad8_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD8_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD8_TH2_OFFSET) & Self::LP_ANA_TOUCH_PAD8_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD8_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad8th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad8th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad8Th2Val {
    bits: u32,
}

impl LpAnaTouchPad8Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD8_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad8_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad8Th2::LP_ANA_TOUCH_PAD8_TH2_MASK)
            >> LpAnaTouchPad8Th2::LP_ANA_TOUCH_PAD8_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD8_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad8_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad8Th2::LP_ANA_TOUCH_PAD8_TH2_MASK)
                | ((value << LpAnaTouchPad8Th2::LP_ANA_TOUCH_PAD8_TH2_OFFSET)
                    & LpAnaTouchPad8Th2::LP_ANA_TOUCH_PAD8_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad8Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad8Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad8Th2Val {
        LpAnaTouchPad8Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad8Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad8Th2Val) -> LpAnaTouchPad8Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD9_TH0 Register
///
/// need_des
///
/// Address offset: 0x01b0
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD9_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad9Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad9Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad9Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD9_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD9_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD9_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD9_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad9_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD9_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD9_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD9_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad9_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD9_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD9_TH0_OFFSET) & Self::LP_ANA_TOUCH_PAD9_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD9_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad9th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad9th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad9Th0Val {
    bits: u32,
}

impl LpAnaTouchPad9Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD9_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad9_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad9Th0::LP_ANA_TOUCH_PAD9_TH0_MASK)
            >> LpAnaTouchPad9Th0::LP_ANA_TOUCH_PAD9_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD9_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad9_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad9Th0::LP_ANA_TOUCH_PAD9_TH0_MASK)
                | ((value << LpAnaTouchPad9Th0::LP_ANA_TOUCH_PAD9_TH0_OFFSET)
                    & LpAnaTouchPad9Th0::LP_ANA_TOUCH_PAD9_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad9Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad9Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad9Th0Val {
        LpAnaTouchPad9Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad9Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad9Th0Val) -> LpAnaTouchPad9Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD9_TH1 Register
///
/// need_des
///
/// Address offset: 0x01b4
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD9_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad9Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad9Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad9Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD9_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD9_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD9_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD9_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad9_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD9_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD9_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD9_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad9_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD9_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD9_TH1_OFFSET) & Self::LP_ANA_TOUCH_PAD9_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD9_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad9th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad9th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad9Th1Val {
    bits: u32,
}

impl LpAnaTouchPad9Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD9_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad9_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad9Th1::LP_ANA_TOUCH_PAD9_TH1_MASK)
            >> LpAnaTouchPad9Th1::LP_ANA_TOUCH_PAD9_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD9_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad9_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad9Th1::LP_ANA_TOUCH_PAD9_TH1_MASK)
                | ((value << LpAnaTouchPad9Th1::LP_ANA_TOUCH_PAD9_TH1_OFFSET)
                    & LpAnaTouchPad9Th1::LP_ANA_TOUCH_PAD9_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad9Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad9Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad9Th1Val {
        LpAnaTouchPad9Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad9Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad9Th1Val) -> LpAnaTouchPad9Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD9_TH2 Register
///
/// need_des
///
/// Address offset: 0x01b8
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD9_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad9Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad9Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad9Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD9_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD9_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD9_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD9_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad9_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD9_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD9_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD9_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad9_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD9_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD9_TH2_OFFSET) & Self::LP_ANA_TOUCH_PAD9_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD9_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad9th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad9th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad9Th2Val {
    bits: u32,
}

impl LpAnaTouchPad9Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD9_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad9_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad9Th2::LP_ANA_TOUCH_PAD9_TH2_MASK)
            >> LpAnaTouchPad9Th2::LP_ANA_TOUCH_PAD9_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD9_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad9_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad9Th2::LP_ANA_TOUCH_PAD9_TH2_MASK)
                | ((value << LpAnaTouchPad9Th2::LP_ANA_TOUCH_PAD9_TH2_OFFSET)
                    & LpAnaTouchPad9Th2::LP_ANA_TOUCH_PAD9_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad9Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad9Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad9Th2Val {
        LpAnaTouchPad9Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad9Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad9Th2Val) -> LpAnaTouchPad9Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD10_TH0 Register
///
/// need_des
///
/// Address offset: 0x01bc
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD10_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad10Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad10Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad10Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD10_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD10_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD10_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD10_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad10_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD10_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD10_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD10_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad10_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD10_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD10_TH0_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD10_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD10_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad10th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad10th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad10Th0Val {
    bits: u32,
}

impl LpAnaTouchPad10Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD10_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad10_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad10Th0::LP_ANA_TOUCH_PAD10_TH0_MASK)
            >> LpAnaTouchPad10Th0::LP_ANA_TOUCH_PAD10_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD10_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad10_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad10Th0::LP_ANA_TOUCH_PAD10_TH0_MASK)
                | ((value << LpAnaTouchPad10Th0::LP_ANA_TOUCH_PAD10_TH0_OFFSET)
                    & LpAnaTouchPad10Th0::LP_ANA_TOUCH_PAD10_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad10Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad10Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad10Th0Val {
        LpAnaTouchPad10Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad10Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad10Th0Val) -> LpAnaTouchPad10Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD10_TH1 Register
///
/// need_des
///
/// Address offset: 0x01c0
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD10_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad10Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad10Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad10Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD10_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD10_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD10_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD10_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad10_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD10_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD10_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD10_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad10_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD10_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD10_TH1_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD10_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD10_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad10th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad10th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad10Th1Val {
    bits: u32,
}

impl LpAnaTouchPad10Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD10_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad10_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad10Th1::LP_ANA_TOUCH_PAD10_TH1_MASK)
            >> LpAnaTouchPad10Th1::LP_ANA_TOUCH_PAD10_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD10_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad10_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad10Th1::LP_ANA_TOUCH_PAD10_TH1_MASK)
                | ((value << LpAnaTouchPad10Th1::LP_ANA_TOUCH_PAD10_TH1_OFFSET)
                    & LpAnaTouchPad10Th1::LP_ANA_TOUCH_PAD10_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad10Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad10Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad10Th1Val {
        LpAnaTouchPad10Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad10Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad10Th1Val) -> LpAnaTouchPad10Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD10_TH2 Register
///
/// need_des
///
/// Address offset: 0x01c4
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD10_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad10Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad10Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad10Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD10_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD10_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD10_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD10_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad10_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD10_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD10_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD10_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad10_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD10_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD10_TH2_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD10_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD10_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad10th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad10th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad10Th2Val {
    bits: u32,
}

impl LpAnaTouchPad10Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD10_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad10_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad10Th2::LP_ANA_TOUCH_PAD10_TH2_MASK)
            >> LpAnaTouchPad10Th2::LP_ANA_TOUCH_PAD10_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD10_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad10_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad10Th2::LP_ANA_TOUCH_PAD10_TH2_MASK)
                | ((value << LpAnaTouchPad10Th2::LP_ANA_TOUCH_PAD10_TH2_OFFSET)
                    & LpAnaTouchPad10Th2::LP_ANA_TOUCH_PAD10_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad10Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad10Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad10Th2Val {
        LpAnaTouchPad10Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad10Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad10Th2Val) -> LpAnaTouchPad10Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD11_TH0 Register
///
/// need_des
///
/// Address offset: 0x01c8
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD11_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad11Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad11Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad11Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD11_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD11_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD11_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD11_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad11_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD11_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD11_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD11_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad11_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD11_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD11_TH0_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD11_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD11_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad11th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad11th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad11Th0Val {
    bits: u32,
}

impl LpAnaTouchPad11Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD11_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad11_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad11Th0::LP_ANA_TOUCH_PAD11_TH0_MASK)
            >> LpAnaTouchPad11Th0::LP_ANA_TOUCH_PAD11_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD11_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad11_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad11Th0::LP_ANA_TOUCH_PAD11_TH0_MASK)
                | ((value << LpAnaTouchPad11Th0::LP_ANA_TOUCH_PAD11_TH0_OFFSET)
                    & LpAnaTouchPad11Th0::LP_ANA_TOUCH_PAD11_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad11Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad11Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad11Th0Val {
        LpAnaTouchPad11Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad11Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad11Th0Val) -> LpAnaTouchPad11Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD11_TH1 Register
///
/// need_des
///
/// Address offset: 0x01cc
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD11_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad11Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad11Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad11Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD11_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD11_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD11_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD11_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad11_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD11_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD11_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD11_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad11_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD11_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD11_TH1_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD11_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD11_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad11th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad11th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad11Th1Val {
    bits: u32,
}

impl LpAnaTouchPad11Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD11_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad11_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad11Th1::LP_ANA_TOUCH_PAD11_TH1_MASK)
            >> LpAnaTouchPad11Th1::LP_ANA_TOUCH_PAD11_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD11_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad11_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad11Th1::LP_ANA_TOUCH_PAD11_TH1_MASK)
                | ((value << LpAnaTouchPad11Th1::LP_ANA_TOUCH_PAD11_TH1_OFFSET)
                    & LpAnaTouchPad11Th1::LP_ANA_TOUCH_PAD11_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad11Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad11Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad11Th1Val {
        LpAnaTouchPad11Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad11Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad11Th1Val) -> LpAnaTouchPad11Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD11_TH2 Register
///
/// need_des
///
/// Address offset: 0x01d0
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD11_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad11Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad11Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad11Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD11_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD11_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD11_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD11_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad11_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD11_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD11_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD11_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad11_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD11_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD11_TH2_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD11_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD11_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad11th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad11th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad11Th2Val {
    bits: u32,
}

impl LpAnaTouchPad11Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD11_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad11_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad11Th2::LP_ANA_TOUCH_PAD11_TH2_MASK)
            >> LpAnaTouchPad11Th2::LP_ANA_TOUCH_PAD11_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD11_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad11_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad11Th2::LP_ANA_TOUCH_PAD11_TH2_MASK)
                | ((value << LpAnaTouchPad11Th2::LP_ANA_TOUCH_PAD11_TH2_OFFSET)
                    & LpAnaTouchPad11Th2::LP_ANA_TOUCH_PAD11_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad11Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad11Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad11Th2Val {
        LpAnaTouchPad11Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad11Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad11Th2Val) -> LpAnaTouchPad11Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD12_TH0 Register
///
/// need_des
///
/// Address offset: 0x01d4
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD12_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad12Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad12Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad12Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD12_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD12_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD12_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD12_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad12_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD12_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD12_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD12_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad12_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD12_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD12_TH0_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD12_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD12_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad12th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad12th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad12Th0Val {
    bits: u32,
}

impl LpAnaTouchPad12Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD12_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad12_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad12Th0::LP_ANA_TOUCH_PAD12_TH0_MASK)
            >> LpAnaTouchPad12Th0::LP_ANA_TOUCH_PAD12_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD12_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad12_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad12Th0::LP_ANA_TOUCH_PAD12_TH0_MASK)
                | ((value << LpAnaTouchPad12Th0::LP_ANA_TOUCH_PAD12_TH0_OFFSET)
                    & LpAnaTouchPad12Th0::LP_ANA_TOUCH_PAD12_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad12Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad12Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad12Th0Val {
        LpAnaTouchPad12Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad12Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad12Th0Val) -> LpAnaTouchPad12Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD12_TH1 Register
///
/// need_des
///
/// Address offset: 0x01d8
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD12_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad12Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad12Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad12Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD12_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD12_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD12_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD12_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad12_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD12_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD12_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD12_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad12_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD12_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD12_TH1_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD12_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD12_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad12th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad12th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad12Th1Val {
    bits: u32,
}

impl LpAnaTouchPad12Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD12_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad12_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad12Th1::LP_ANA_TOUCH_PAD12_TH1_MASK)
            >> LpAnaTouchPad12Th1::LP_ANA_TOUCH_PAD12_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD12_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad12_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad12Th1::LP_ANA_TOUCH_PAD12_TH1_MASK)
                | ((value << LpAnaTouchPad12Th1::LP_ANA_TOUCH_PAD12_TH1_OFFSET)
                    & LpAnaTouchPad12Th1::LP_ANA_TOUCH_PAD12_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad12Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad12Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad12Th1Val {
        LpAnaTouchPad12Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad12Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad12Th1Val) -> LpAnaTouchPad12Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD12_TH2 Register
///
/// need_des
///
/// Address offset: 0x01dc
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD12_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad12Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad12Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad12Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD12_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD12_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD12_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD12_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad12_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD12_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD12_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD12_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad12_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD12_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD12_TH2_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD12_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD12_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad12th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad12th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad12Th2Val {
    bits: u32,
}

impl LpAnaTouchPad12Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD12_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad12_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad12Th2::LP_ANA_TOUCH_PAD12_TH2_MASK)
            >> LpAnaTouchPad12Th2::LP_ANA_TOUCH_PAD12_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD12_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad12_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad12Th2::LP_ANA_TOUCH_PAD12_TH2_MASK)
                | ((value << LpAnaTouchPad12Th2::LP_ANA_TOUCH_PAD12_TH2_OFFSET)
                    & LpAnaTouchPad12Th2::LP_ANA_TOUCH_PAD12_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad12Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad12Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad12Th2Val {
        LpAnaTouchPad12Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad12Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad12Th2Val) -> LpAnaTouchPad12Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD13_TH0 Register
///
/// need_des
///
/// Address offset: 0x01e0
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD13_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad13Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad13Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad13Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD13_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD13_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD13_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD13_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad13_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD13_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD13_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD13_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad13_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD13_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD13_TH0_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD13_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD13_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad13th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad13th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad13Th0Val {
    bits: u32,
}

impl LpAnaTouchPad13Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD13_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad13_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad13Th0::LP_ANA_TOUCH_PAD13_TH0_MASK)
            >> LpAnaTouchPad13Th0::LP_ANA_TOUCH_PAD13_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD13_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad13_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad13Th0::LP_ANA_TOUCH_PAD13_TH0_MASK)
                | ((value << LpAnaTouchPad13Th0::LP_ANA_TOUCH_PAD13_TH0_OFFSET)
                    & LpAnaTouchPad13Th0::LP_ANA_TOUCH_PAD13_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad13Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad13Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad13Th0Val {
        LpAnaTouchPad13Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad13Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad13Th0Val) -> LpAnaTouchPad13Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD13_TH1 Register
///
/// need_des
///
/// Address offset: 0x01e4
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD13_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad13Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad13Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad13Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD13_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD13_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD13_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD13_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad13_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD13_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD13_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD13_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad13_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD13_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD13_TH1_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD13_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD13_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad13th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad13th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad13Th1Val {
    bits: u32,
}

impl LpAnaTouchPad13Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD13_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad13_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad13Th1::LP_ANA_TOUCH_PAD13_TH1_MASK)
            >> LpAnaTouchPad13Th1::LP_ANA_TOUCH_PAD13_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD13_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad13_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad13Th1::LP_ANA_TOUCH_PAD13_TH1_MASK)
                | ((value << LpAnaTouchPad13Th1::LP_ANA_TOUCH_PAD13_TH1_OFFSET)
                    & LpAnaTouchPad13Th1::LP_ANA_TOUCH_PAD13_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad13Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad13Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad13Th1Val {
        LpAnaTouchPad13Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad13Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad13Th1Val) -> LpAnaTouchPad13Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD13_TH2 Register
///
/// need_des
///
/// Address offset: 0x01e8
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD13_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad13Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad13Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad13Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD13_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD13_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD13_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD13_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad13_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD13_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD13_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD13_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad13_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD13_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD13_TH2_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD13_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD13_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad13th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad13th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad13Th2Val {
    bits: u32,
}

impl LpAnaTouchPad13Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD13_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad13_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad13Th2::LP_ANA_TOUCH_PAD13_TH2_MASK)
            >> LpAnaTouchPad13Th2::LP_ANA_TOUCH_PAD13_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD13_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad13_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad13Th2::LP_ANA_TOUCH_PAD13_TH2_MASK)
                | ((value << LpAnaTouchPad13Th2::LP_ANA_TOUCH_PAD13_TH2_OFFSET)
                    & LpAnaTouchPad13Th2::LP_ANA_TOUCH_PAD13_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad13Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad13Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad13Th2Val {
        LpAnaTouchPad13Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad13Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad13Th2Val) -> LpAnaTouchPad13Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD14_TH0 Register
///
/// need_des
///
/// Address offset: 0x01ec
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD14_TH0** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad14Th0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad14Th0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad14Th0 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD14_TH0**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD14_TH0_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD14_TH0_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD14_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad14_th0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD14_TH0_MASK)
            >> Self::LP_ANA_TOUCH_PAD14_TH0_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD14_TH0** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad14_th0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD14_TH0_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD14_TH0_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD14_TH0_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD14_TH0 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad14th0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad14th0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad14Th0Val {
    bits: u32,
}

impl LpAnaTouchPad14Th0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD14_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad14_th0(&self) -> u32 {
        (self.bits & LpAnaTouchPad14Th0::LP_ANA_TOUCH_PAD14_TH0_MASK)
            >> LpAnaTouchPad14Th0::LP_ANA_TOUCH_PAD14_TH0_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD14_TH0** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad14_th0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad14Th0::LP_ANA_TOUCH_PAD14_TH0_MASK)
                | ((value << LpAnaTouchPad14Th0::LP_ANA_TOUCH_PAD14_TH0_OFFSET)
                    & LpAnaTouchPad14Th0::LP_ANA_TOUCH_PAD14_TH0_MASK),
        }
    }
}

impl Default for LpAnaTouchPad14Th0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad14Th0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad14Th0Val {
        LpAnaTouchPad14Th0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad14Th0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad14Th0Val) -> LpAnaTouchPad14Th0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD14_TH1 Register
///
/// need_des
///
/// Address offset: 0x01f0
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD14_TH1** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad14Th1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad14Th1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad14Th1 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD14_TH1**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD14_TH1_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD14_TH1_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD14_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad14_th1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD14_TH1_MASK)
            >> Self::LP_ANA_TOUCH_PAD14_TH1_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD14_TH1** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad14_th1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD14_TH1_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD14_TH1_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD14_TH1_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD14_TH1 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad14th1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad14th1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad14Th1Val {
    bits: u32,
}

impl LpAnaTouchPad14Th1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD14_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad14_th1(&self) -> u32 {
        (self.bits & LpAnaTouchPad14Th1::LP_ANA_TOUCH_PAD14_TH1_MASK)
            >> LpAnaTouchPad14Th1::LP_ANA_TOUCH_PAD14_TH1_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD14_TH1** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad14_th1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad14Th1::LP_ANA_TOUCH_PAD14_TH1_MASK)
                | ((value << LpAnaTouchPad14Th1::LP_ANA_TOUCH_PAD14_TH1_OFFSET)
                    & LpAnaTouchPad14Th1::LP_ANA_TOUCH_PAD14_TH1_MASK),
        }
    }
}

impl Default for LpAnaTouchPad14Th1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad14Th1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad14Th1Val {
        LpAnaTouchPad14Th1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad14Th1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad14Th1Val) -> LpAnaTouchPad14Th1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_TOUCH_PAD14_TH2 Register
///
/// need_des
///
/// Address offset: 0x01f4
///
/// ## Fields
///
/// - **LP_ANA_TOUCH_PAD14_TH2** `[31:16]` (RW)
///
///   Reserved
#[repr(transparent)]
pub struct LpAnaTouchPad14Th2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaTouchPad14Th2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaTouchPad14Th2 {
    // ======== Field Constants ========

    /// **LP_ANA_TOUCH_PAD14_TH2**
    ///
    /// Reserved
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const LP_ANA_TOUCH_PAD14_TH2_OFFSET: u32 = 16;
    pub const LP_ANA_TOUCH_PAD14_TH2_MASK: u32 = 0xffff0000;

    // ======== Field Accessors ========

    /// **LP_ANA_TOUCH_PAD14_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn lp_ana_touch_pad14_th2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_TOUCH_PAD14_TH2_MASK)
            >> Self::LP_ANA_TOUCH_PAD14_TH2_OFFSET
    }

    /// Write **LP_ANA_TOUCH_PAD14_TH2** `[31:16]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn write_lp_ana_touch_pad14_th2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_TOUCH_PAD14_TH2_MASK)
                | ((value << Self::LP_ANA_TOUCH_PAD14_TH2_OFFSET)
                    & Self::LP_ANA_TOUCH_PAD14_TH2_MASK)
        });
    }
}

/// LP_ANA_TOUCH_PAD14_TH2 value type for batch read/modify/write operations
///
/// Use `lpanatouchpad14th2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanatouchpad14th2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaTouchPad14Th2Val {
    bits: u32,
}

impl LpAnaTouchPad14Th2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ANA_TOUCH_PAD14_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn lp_ana_touch_pad14_th2(&self) -> u32 {
        (self.bits & LpAnaTouchPad14Th2::LP_ANA_TOUCH_PAD14_TH2_MASK)
            >> LpAnaTouchPad14Th2::LP_ANA_TOUCH_PAD14_TH2_OFFSET
    }

    /// Set **LP_ANA_TOUCH_PAD14_TH2** `[31:16]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_lp_ana_touch_pad14_th2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaTouchPad14Th2::LP_ANA_TOUCH_PAD14_TH2_MASK)
                | ((value << LpAnaTouchPad14Th2::LP_ANA_TOUCH_PAD14_TH2_OFFSET)
                    & LpAnaTouchPad14Th2::LP_ANA_TOUCH_PAD14_TH2_MASK),
        }
    }
}

impl Default for LpAnaTouchPad14Th2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAnaTouchPad14Th2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaTouchPad14Th2Val {
        LpAnaTouchPad14Th2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaTouchPad14Th2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaTouchPad14Th2Val) -> LpAnaTouchPad14Th2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ANA_DATE Register
///
/// need_des
///
/// Address offset: 0x03fc
/// Reset value: 0x00230420
///
/// ## Fields
///
/// - **LP_ANA_CLK_EN** `[31]` (RW)
///
///   need_des
///
/// - **LP_ANA_LP_ANA_DATE** `[30:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpAnaDate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAnaDate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAnaDate {
    // ======== Field Constants ========

    /// **LP_ANA_CLK_EN**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const LP_ANA_CLK_EN_OFFSET: u32 = 31;
    pub const LP_ANA_CLK_EN_MASK: u32 = 0x80000000;
    pub const LP_ANA_CLK_EN: u32 = 1 << 31;

    /// **LP_ANA_LP_ANA_DATE**
    ///
    /// need_des
    ///
    /// Bits `[30:0]`, mask `0x7fffffff`
    pub const LP_ANA_LP_ANA_DATE_OFFSET: u32 = 0;
    pub const LP_ANA_LP_ANA_DATE_MASK: u32 = 0x7fffffff;

    // ======== Field Accessors ========

    /// **LP_ANA_CLK_EN** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_ANA_CLK_EN_MASK) != 0
    }

    /// Set LP_ANA_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_ana_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::LP_ANA_CLK_EN_MASK);
    }

    /// Clear LP_ANA_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_ana_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::LP_ANA_CLK_EN_MASK);
    }

    /// **LP_ANA_LP_ANA_DATE** `[30:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_ana_lp_ana_date(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ANA_LP_ANA_DATE_MASK) >> Self::LP_ANA_LP_ANA_DATE_OFFSET
    }

    /// Write **LP_ANA_LP_ANA_DATE** `[30:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_ana_lp_ana_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ANA_LP_ANA_DATE_MASK)
                | ((value << Self::LP_ANA_LP_ANA_DATE_OFFSET) & Self::LP_ANA_LP_ANA_DATE_MASK)
        });
    }
}

/// LP_ANA_DATE value type for batch read/modify/write operations
///
/// Use `lpanadate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpanadate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAnaDateVal {
    bits: u32,
}

impl LpAnaDateVal {
    /// Create from raw bits
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
        Self { bits: 0x00230420 }
    }

    /// Read **LP_ANA_CLK_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_clk_en(&self) -> bool {
        (self.bits & LpAnaDate::LP_ANA_CLK_EN_MASK) != 0
    }

    /// Read **LP_ANA_LP_ANA_DATE** `[30:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_ana_lp_ana_date(&self) -> u32 {
        (self.bits & LpAnaDate::LP_ANA_LP_ANA_DATE_MASK) >> LpAnaDate::LP_ANA_LP_ANA_DATE_OFFSET
    }

    /// Set **LP_ANA_CLK_EN** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpAnaDate::LP_ANA_CLK_EN_MASK
            } else {
                self.bits & !LpAnaDate::LP_ANA_CLK_EN_MASK
            },
        }
    }

    /// Set **LP_ANA_LP_ANA_DATE** `[30:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_ana_lp_ana_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpAnaDate::LP_ANA_LP_ANA_DATE_MASK)
                | ((value << LpAnaDate::LP_ANA_LP_ANA_DATE_OFFSET)
                    & LpAnaDate::LP_ANA_LP_ANA_DATE_MASK),
        }
    }
}

impl Default for LpAnaDateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpAnaDate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAnaDateVal {
        LpAnaDateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpAnaDateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpAnaDateVal) -> LpAnaDateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// need_des
    pub lp_ana_bod_mode0_cntl: LpAnaBodMode0Cntl,
    /// 0x0004
    /// need_des
    pub lp_ana_bod_mode1_cntl: LpAnaBodMode1Cntl,
    /// 0x0008
    /// need_des
    pub lp_ana_vdd_source_cntl: LpAnaVddSourceCntl,
    /// 0x000c
    /// need_des
    pub lp_ana_vddbat_bod_cntl: LpAnaVddbatBodCntl,
    /// 0x0010
    /// need_des
    pub lp_ana_vddbat_charge_cntl: LpAnaVddbatChargeCntl,
    /// 0x0014
    /// need_des
    pub lp_ana_ck_glitch_cntl: LpAnaCkGlitchCntl,
    /// 0x0018
    /// need_des
    pub lp_ana_pg_glitch_cntl: LpAnaPgGlitchCntl,
    /// 0x001c
    /// need_des
    pub lp_ana_fib_enable: LpAnaFibEnable,
    /// 0x0020
    /// need_des
    pub lp_ana_int_raw: LpAnaIntRaw,
    /// 0x0024
    /// need_des
    pub lp_ana_int_st: LpAnaIntSt,
    /// 0x0028
    /// need_des
    pub lp_ana_int_ena: LpAnaIntEna,
    /// 0x002c
    /// need_des
    pub lp_ana_int_clr: LpAnaIntClr,
    /// 0x0030
    /// need_des
    pub lp_ana_lp_int_raw: LpAnaLpIntRaw,
    /// 0x0034
    /// need_des
    pub lp_ana_lp_int_st: LpAnaLpIntSt,
    /// 0x0038
    /// need_des
    pub lp_ana_lp_int_ena: LpAnaLpIntEna,
    /// 0x003c
    /// need_des
    pub lp_ana_lp_int_clr: LpAnaLpIntClr,
    _reserved0: [u8; 188],
    /// 0x00fc
    /// need_des
    pub lp_ana_touch_approach_work_meas_num: LpAnaTouchApproachWorkMeasNum,
    /// 0x0100
    /// need_des
    pub lp_ana_touch_scan_ctrl1: LpAnaTouchScanCtrl1,
    /// 0x0104
    /// need_des
    pub lp_ana_touch_scan_ctrl2: LpAnaTouchScanCtrl2,
    /// 0x0108
    /// need_des
    pub lp_ana_touch_work: LpAnaTouchWork,
    /// 0x010c
    /// need_des
    pub lp_ana_touch_work_meas_num: LpAnaTouchWorkMeasNum,
    /// 0x0110
    /// need_des
    pub lp_ana_touch_filter1: LpAnaTouchFilter1,
    /// 0x0114
    /// need_des
    pub lp_ana_touch_filter2: LpAnaTouchFilter2,
    /// 0x0118
    /// need_des
    pub lp_ana_touch_filter3: LpAnaTouchFilter3,
    /// 0x011c
    /// need_des
    pub lp_ana_touch_slp0: LpAnaTouchSlp0,
    /// 0x0120
    /// need_des
    pub lp_ana_touch_slp1: LpAnaTouchSlp1,
    /// 0x0124
    /// need_des
    pub lp_ana_touch_clr: LpAnaTouchClr,
    /// 0x0128
    /// need_des
    pub lp_ana_touch_approach: LpAnaTouchApproach,
    /// 0x012c
    /// need_des
    pub lp_ana_touch_freq0_scan_para: LpAnaTouchFreq0ScanPara,
    /// 0x0130
    /// need_des
    pub lp_ana_touch_freq1_scan_para: LpAnaTouchFreq1ScanPara,
    /// 0x0134
    /// need_des
    pub lp_ana_touch_freq2_scan_para: LpAnaTouchFreq2ScanPara,
    /// 0x0138
    /// need_des
    pub lp_ana_touch_ana_para: LpAnaTouchAnaPara,
    /// 0x013c
    /// need_des
    pub lp_ana_touch_mux0: LpAnaTouchMux0,
    /// 0x0140
    /// need_des
    pub lp_ana_touch_mux1: LpAnaTouchMux1,
    /// 0x0144
    /// need_des
    pub lp_ana_touch_pad0_th0: LpAnaTouchPad0Th0,
    /// 0x0148
    /// need_des
    pub lp_ana_touch_pad0_th1: LpAnaTouchPad0Th1,
    /// 0x014c
    /// need_des
    pub lp_ana_touch_pad0_th2: LpAnaTouchPad0Th2,
    /// 0x0150
    /// need_des
    pub lp_ana_touch_pad1_th0: LpAnaTouchPad1Th0,
    /// 0x0154
    /// need_des
    pub lp_ana_touch_pad1_th1: LpAnaTouchPad1Th1,
    /// 0x0158
    /// need_des
    pub lp_ana_touch_pad1_th2: LpAnaTouchPad1Th2,
    /// 0x015c
    /// need_des
    pub lp_ana_touch_pad2_th0: LpAnaTouchPad2Th0,
    /// 0x0160
    /// need_des
    pub lp_ana_touch_pad2_th1: LpAnaTouchPad2Th1,
    /// 0x0164
    /// need_des
    pub lp_ana_touch_pad2_th2: LpAnaTouchPad2Th2,
    /// 0x0168
    /// need_des
    pub lp_ana_touch_pad3_th0: LpAnaTouchPad3Th0,
    /// 0x016c
    /// need_des
    pub lp_ana_touch_pad3_th1: LpAnaTouchPad3Th1,
    /// 0x0170
    /// need_des
    pub lp_ana_touch_pad3_th2: LpAnaTouchPad3Th2,
    /// 0x0174
    /// need_des
    pub lp_ana_touch_pad4_th0: LpAnaTouchPad4Th0,
    /// 0x0178
    /// need_des
    pub lp_ana_touch_pad4_th1: LpAnaTouchPad4Th1,
    /// 0x017c
    /// need_des
    pub lp_ana_touch_pad4_th2: LpAnaTouchPad4Th2,
    /// 0x0180
    /// need_des
    pub lp_ana_touch_pad5_th0: LpAnaTouchPad5Th0,
    /// 0x0184
    /// need_des
    pub lp_ana_touch_pad5_th1: LpAnaTouchPad5Th1,
    /// 0x0188
    /// need_des
    pub lp_ana_touch_pad5_th2: LpAnaTouchPad5Th2,
    /// 0x018c
    /// need_des
    pub lp_ana_touch_pad6_th0: LpAnaTouchPad6Th0,
    /// 0x0190
    /// need_des
    pub lp_ana_touch_pad6_th1: LpAnaTouchPad6Th1,
    /// 0x0194
    /// need_des
    pub lp_ana_touch_pad6_th2: LpAnaTouchPad6Th2,
    /// 0x0198
    /// need_des
    pub lp_ana_touch_pad7_th0: LpAnaTouchPad7Th0,
    /// 0x019c
    /// need_des
    pub lp_ana_touch_pad7_th1: LpAnaTouchPad7Th1,
    /// 0x01a0
    /// need_des
    pub lp_ana_touch_pad7_th2: LpAnaTouchPad7Th2,
    /// 0x01a4
    /// need_des
    pub lp_ana_touch_pad8_th0: LpAnaTouchPad8Th0,
    /// 0x01a8
    /// need_des
    pub lp_ana_touch_pad8_th1: LpAnaTouchPad8Th1,
    /// 0x01ac
    /// need_des
    pub lp_ana_touch_pad8_th2: LpAnaTouchPad8Th2,
    /// 0x01b0
    /// need_des
    pub lp_ana_touch_pad9_th0: LpAnaTouchPad9Th0,
    /// 0x01b4
    /// need_des
    pub lp_ana_touch_pad9_th1: LpAnaTouchPad9Th1,
    /// 0x01b8
    /// need_des
    pub lp_ana_touch_pad9_th2: LpAnaTouchPad9Th2,
    /// 0x01bc
    /// need_des
    pub lp_ana_touch_pad10_th0: LpAnaTouchPad10Th0,
    /// 0x01c0
    /// need_des
    pub lp_ana_touch_pad10_th1: LpAnaTouchPad10Th1,
    /// 0x01c4
    /// need_des
    pub lp_ana_touch_pad10_th2: LpAnaTouchPad10Th2,
    /// 0x01c8
    /// need_des
    pub lp_ana_touch_pad11_th0: LpAnaTouchPad11Th0,
    /// 0x01cc
    /// need_des
    pub lp_ana_touch_pad11_th1: LpAnaTouchPad11Th1,
    /// 0x01d0
    /// need_des
    pub lp_ana_touch_pad11_th2: LpAnaTouchPad11Th2,
    /// 0x01d4
    /// need_des
    pub lp_ana_touch_pad12_th0: LpAnaTouchPad12Th0,
    /// 0x01d8
    /// need_des
    pub lp_ana_touch_pad12_th1: LpAnaTouchPad12Th1,
    /// 0x01dc
    /// need_des
    pub lp_ana_touch_pad12_th2: LpAnaTouchPad12Th2,
    /// 0x01e0
    /// need_des
    pub lp_ana_touch_pad13_th0: LpAnaTouchPad13Th0,
    /// 0x01e4
    /// need_des
    pub lp_ana_touch_pad13_th1: LpAnaTouchPad13Th1,
    /// 0x01e8
    /// need_des
    pub lp_ana_touch_pad13_th2: LpAnaTouchPad13Th2,
    /// 0x01ec
    /// need_des
    pub lp_ana_touch_pad14_th0: LpAnaTouchPad14Th0,
    /// 0x01f0
    /// need_des
    pub lp_ana_touch_pad14_th1: LpAnaTouchPad14Th1,
    /// 0x01f4
    /// need_des
    pub lp_ana_touch_pad14_th2: LpAnaTouchPad14Th2,
    _reserved1: [u8; 516],
    /// 0x03fc
    /// need_des
    pub lp_ana_date: LpAnaDate,
}
