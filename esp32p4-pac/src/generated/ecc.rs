//! ECC
//! ECC (ECC Hardware Accelerator)

#![allow(dead_code)]

pub const BASE: u32 = 0x50093000;

/// MULT_INT_RAW Register
///
/// ECC interrupt raw register, valid in level.
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **CALC_DONE_INT_RAW** `[0]` (RO)
///
///   The raw interrupt status bit for the ecc_calc_done_int interrupt
#[repr(transparent)]
pub struct MultIntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MultIntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MultIntRaw {
    // ======== Field Constants ========

    /// **CALC_DONE_INT_RAW**
    ///
    /// The raw interrupt status bit for the ecc_calc_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CALC_DONE_INT_RAW_OFFSET: u32 = 0;
    pub const CALC_DONE_INT_RAW_MASK: u32 = 0x00000001;
    pub const CALC_DONE_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CALC_DONE_INT_RAW** `[0]` (RO)
    ///
    /// The raw interrupt status bit for the ecc_calc_done_int interrupt
    #[inline(always)]
    pub fn calc_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CALC_DONE_INT_RAW_MASK) != 0
    }
}

/// MULT_INT_RAW value type for batch read/modify/write operations
///
/// Use `multintraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `multintraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MultIntRawVal {
    bits: u32,
}

impl MultIntRawVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CALC_DONE_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt status bit for the ecc_calc_done_int interrupt
    #[inline(always)]
    pub const fn calc_done_int_raw(&self) -> bool {
        (self.bits & MultIntRaw::CALC_DONE_INT_RAW_MASK) != 0
    }
}

impl Default for MultIntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MultIntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MultIntRawVal {
        MultIntRawVal::from_bits(self.value.read_bits())
    }
}

/// MULT_INT_ST Register
///
/// ECC interrupt status register.
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **CALC_DONE_INT_ST** `[0]` (RO)
///
///   The masked interrupt status bit for the ecc_calc_done_int interrupt
#[repr(transparent)]
pub struct MultIntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MultIntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MultIntSt {
    // ======== Field Constants ========

    /// **CALC_DONE_INT_ST**
    ///
    /// The masked interrupt status bit for the ecc_calc_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CALC_DONE_INT_ST_OFFSET: u32 = 0;
    pub const CALC_DONE_INT_ST_MASK: u32 = 0x00000001;
    pub const CALC_DONE_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CALC_DONE_INT_ST** `[0]` (RO)
    ///
    /// The masked interrupt status bit for the ecc_calc_done_int interrupt
    #[inline(always)]
    pub fn calc_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CALC_DONE_INT_ST_MASK) != 0
    }
}

/// MULT_INT_ST value type for batch read/modify/write operations
///
/// Use `multintst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `multintst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MultIntStVal {
    bits: u32,
}

impl MultIntStVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CALC_DONE_INT_ST** `[0:0]`
    ///
    /// The masked interrupt status bit for the ecc_calc_done_int interrupt
    #[inline(always)]
    pub const fn calc_done_int_st(&self) -> bool {
        (self.bits & MultIntSt::CALC_DONE_INT_ST_MASK) != 0
    }
}

impl Default for MultIntStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MultIntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MultIntStVal {
        MultIntStVal::from_bits(self.value.read_bits())
    }
}

/// MULT_INT_ENA Register
///
/// ECC interrupt enable register.
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **CALC_DONE_INT_ENA** `[0]` (RW)
///
///   The interrupt enable bit for the ecc_calc_done_int interrupt
#[repr(transparent)]
pub struct MultIntEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MultIntEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MultIntEna {
    // ======== Field Constants ========

    /// **CALC_DONE_INT_ENA**
    ///
    /// The interrupt enable bit for the ecc_calc_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CALC_DONE_INT_ENA_OFFSET: u32 = 0;
    pub const CALC_DONE_INT_ENA_MASK: u32 = 0x00000001;
    pub const CALC_DONE_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CALC_DONE_INT_ENA** `[0]` (RW)
    ///
    /// The interrupt enable bit for the ecc_calc_done_int interrupt
    #[inline(always)]
    pub fn calc_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CALC_DONE_INT_ENA_MASK) != 0
    }

    /// Set CALC_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_calc_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::CALC_DONE_INT_ENA_MASK);
    }

    /// Clear CALC_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_calc_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CALC_DONE_INT_ENA_MASK);
    }
}

/// MULT_INT_ENA value type for batch read/modify/write operations
///
/// Use `multintena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `multintena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MultIntEnaVal {
    bits: u32,
}

impl MultIntEnaVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **CALC_DONE_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the ecc_calc_done_int interrupt
    #[inline(always)]
    pub const fn calc_done_int_ena(&self) -> bool {
        (self.bits & MultIntEna::CALC_DONE_INT_ENA_MASK) != 0
    }

    /// Set **CALC_DONE_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the ecc_calc_done_int interrupt
    #[inline(always)]
    pub const fn with_calc_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MultIntEna::CALC_DONE_INT_ENA_MASK
            } else {
                self.bits & !MultIntEna::CALC_DONE_INT_ENA_MASK
            },
        }
    }
}

impl Default for MultIntEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MultIntEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MultIntEnaVal {
        MultIntEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MultIntEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MultIntEnaVal) -> MultIntEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MULT_INT_CLR Register
///
/// ECC interrupt clear register.
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **CALC_DONE_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the ecc_calc_done_int interrupt
#[repr(transparent)]
pub struct MultIntClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MultIntClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MultIntClr {
    // ======== Field Constants ========

    /// **CALC_DONE_INT_CLR**
    ///
    /// Set this bit to clear the ecc_calc_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CALC_DONE_INT_CLR_OFFSET: u32 = 0;
    pub const CALC_DONE_INT_CLR_MASK: u32 = 0x00000001;
    pub const CALC_DONE_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CALC_DONE_INT_CLR** (WO)
    ///
    /// Set this bit to clear the ecc_calc_done_int interrupt
    #[inline(always)]
    pub fn trigger_calc_done_int_clr(&self) {
        self.value.write_bits(Self::CALC_DONE_INT_CLR_MASK);
    }
}

/// MULT_INT_CLR value type for batch read/modify/write operations
///
/// Use `multintclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `multintclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MultIntClrVal {
    bits: u32,
}

impl MultIntClrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **CALC_DONE_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the ecc_calc_done_int interrupt
    #[inline(always)]
    pub const fn with_calc_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MultIntClr::CALC_DONE_INT_CLR_MASK
            } else {
                self.bits & !MultIntClr::CALC_DONE_INT_CLR_MASK
            },
        }
    }
}

impl Default for MultIntClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MultIntClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MultIntClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// MULT_CONF Register
///
/// ECC configure register
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **MEM_CLOCK_GATE_FORCE_ON** `[31]` (RW)
///
///   ECC memory clock gate force on register
///
/// - **CLK_EN** `[30]` (RW)
///
///   Write 1 to force on register clock gate.
///
/// - **VERIFICATION_RESULT** `[29]` (RO)
///
///   The verification result bit of ECC Accelerator, only valid when calculation is done.
///
/// - **SECURITY_MODE** `[8]` (RW)
///
///   Reserved
///
/// - **WORK_MODE** `[7:4]` (RW)
///
///   The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Reserved. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Point Add Mode. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode. 8: mod addition. 9. mod substraction. 10: mod multiplication. 11: mod division.
///
/// - **MOD_BASE** `[3]` (RW)
///
///   The mod base of mod operation, only valid in work_mode 8-11. 0: n(order of curve). 1: p(mod base of curve)
///
/// - **KEY_LENGTH** `[2]` (RW)
///
///   The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256.
///
/// - **RESET** `[1]` (WO)
///
///   Write 1 to reset ECC Accelerator.
///
/// - **START** `[0]` (RW)
///
///   Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done.
#[repr(transparent)]
pub struct MultConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MultConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MultConf {
    // ======== Field Constants ========

    /// **MEM_CLOCK_GATE_FORCE_ON**
    ///
    /// ECC memory clock gate force on register
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const MEM_CLOCK_GATE_FORCE_ON_OFFSET: u32 = 31;
    pub const MEM_CLOCK_GATE_FORCE_ON_MASK: u32 = 0x80000000;
    pub const MEM_CLOCK_GATE_FORCE_ON: u32 = 1 << 31;

    /// **CLK_EN**
    ///
    /// Write 1 to force on register clock gate.
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const CLK_EN_OFFSET: u32 = 30;
    pub const CLK_EN_MASK: u32 = 0x40000000;
    pub const CLK_EN: u32 = 1 << 30;

    /// **VERIFICATION_RESULT**
    ///
    /// The verification result bit of ECC Accelerator, only valid when calculation is done.
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const VERIFICATION_RESULT_OFFSET: u32 = 29;
    pub const VERIFICATION_RESULT_MASK: u32 = 0x20000000;
    pub const VERIFICATION_RESULT: u32 = 1 << 29;

    /// **SECURITY_MODE**
    ///
    /// Reserved
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SECURITY_MODE_OFFSET: u32 = 8;
    pub const SECURITY_MODE_MASK: u32 = 0x00000100;
    pub const SECURITY_MODE: u32 = 1 << 8;

    /// **WORK_MODE**
    ///
    /// The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Reserved. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Point Add Mode. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode. 8: mod addition. 9. mod substraction. 10: mod multiplication. 11: mod division.
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const WORK_MODE_OFFSET: u32 = 4;
    pub const WORK_MODE_MASK: u32 = 0x000000f0;

    /// **MOD_BASE**
    ///
    /// The mod base of mod operation, only valid in work_mode 8-11. 0: n(order of curve). 1: p(mod base of curve)
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MOD_BASE_OFFSET: u32 = 3;
    pub const MOD_BASE_MASK: u32 = 0x00000008;
    pub const MOD_BASE: u32 = 1 << 3;

    /// **KEY_LENGTH**
    ///
    /// The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const KEY_LENGTH_OFFSET: u32 = 2;
    pub const KEY_LENGTH_MASK: u32 = 0x00000004;
    pub const KEY_LENGTH: u32 = 1 << 2;

    /// **RESET**
    ///
    /// Write 1 to reset ECC Accelerator.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RESET_OFFSET: u32 = 1;
    pub const RESET_MASK: u32 = 0x00000002;
    pub const RESET: u32 = 1 << 1;

    /// **START**
    ///
    /// Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const START_OFFSET: u32 = 0;
    pub const START_MASK: u32 = 0x00000001;
    pub const START: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MEM_CLOCK_GATE_FORCE_ON** `[31]` (RW)
    ///
    /// ECC memory clock gate force on register
    #[inline(always)]
    pub fn mem_clock_gate_force_on(&self) -> bool {
        (self.value.read_bits() & Self::MEM_CLOCK_GATE_FORCE_ON_MASK) != 0
    }

    /// Set MEM_CLOCK_GATE_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_clock_gate_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::MEM_CLOCK_GATE_FORCE_ON_MASK);
    }

    /// Clear MEM_CLOCK_GATE_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_clock_gate_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::MEM_CLOCK_GATE_FORCE_ON_MASK);
    }

    /// **CLK_EN** `[30]` (RW)
    ///
    /// Write 1 to force on register clock gate.
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

    /// **VERIFICATION_RESULT** `[29]` (RO)
    ///
    /// The verification result bit of ECC Accelerator, only valid when calculation is done.
    #[inline(always)]
    pub fn verification_result(&self) -> bool {
        (self.value.read_bits() & Self::VERIFICATION_RESULT_MASK) != 0
    }

    /// **SECURITY_MODE** `[8]` (RW)
    ///
    /// Reserved
    #[inline(always)]
    pub fn security_mode(&self) -> bool {
        (self.value.read_bits() & Self::SECURITY_MODE_MASK) != 0
    }

    /// Set SECURITY_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_security_mode(&self) {
        self.value.modify_bits(|r| r | Self::SECURITY_MODE_MASK);
    }

    /// Clear SECURITY_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_security_mode(&self) {
        self.value.modify_bits(|r| r & !Self::SECURITY_MODE_MASK);
    }

    /// **WORK_MODE** `[7:4]` (RW)
    ///
    /// The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Reserved. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Point Add Mode. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode. 8: mod addition. 9. mod substraction. 10: mod multiplication. 11: mod division.
    #[inline(always)]
    pub fn work_mode(&self) -> u32 {
        (self.value.read_bits() & Self::WORK_MODE_MASK) >> Self::WORK_MODE_OFFSET
    }

    /// Write **WORK_MODE** `[7:4]` (RW)
    ///
    /// The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Reserved. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Point Add Mode. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode. 8: mod addition. 9. mod substraction. 10: mod multiplication. 11: mod division.
    #[inline(always)]
    pub fn write_work_mode(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WORK_MODE_MASK) | ((value << Self::WORK_MODE_OFFSET) & Self::WORK_MODE_MASK)
        });
    }

    /// **MOD_BASE** `[3]` (RW)
    ///
    /// The mod base of mod operation, only valid in work_mode 8-11. 0: n(order of curve). 1: p(mod base of curve)
    #[inline(always)]
    pub fn mod_base(&self) -> bool {
        (self.value.read_bits() & Self::MOD_BASE_MASK) != 0
    }

    /// Set MOD_BASE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mod_base(&self) {
        self.value.modify_bits(|r| r | Self::MOD_BASE_MASK);
    }

    /// Clear MOD_BASE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mod_base(&self) {
        self.value.modify_bits(|r| r & !Self::MOD_BASE_MASK);
    }

    /// **KEY_LENGTH** `[2]` (RW)
    ///
    /// The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256.
    #[inline(always)]
    pub fn key_length(&self) -> bool {
        (self.value.read_bits() & Self::KEY_LENGTH_MASK) != 0
    }

    /// Set KEY_LENGTH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_key_length(&self) {
        self.value.modify_bits(|r| r | Self::KEY_LENGTH_MASK);
    }

    /// Clear KEY_LENGTH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_key_length(&self) {
        self.value.modify_bits(|r| r & !Self::KEY_LENGTH_MASK);
    }

    /// Trigger **RESET** (WO)
    ///
    /// Write 1 to reset ECC Accelerator.
    #[inline(always)]
    pub fn trigger_reset(&self) {
        self.value.modify_bits(|r| r | Self::RESET_MASK);
    }

    /// **START** `[0]` (RW)
    ///
    /// Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done.
    #[inline(always)]
    pub fn start(&self) -> bool {
        (self.value.read_bits() & Self::START_MASK) != 0
    }

    /// Set START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_start(&self) {
        self.value.modify_bits(|r| r | Self::START_MASK);
    }

    /// Clear START bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_start(&self) {
        self.value.modify_bits(|r| r & !Self::START_MASK);
    }
}

/// MULT_CONF value type for batch read/modify/write operations
///
/// Use `multconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `multconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MultConfVal {
    bits: u32,
}

impl MultConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **MEM_CLOCK_GATE_FORCE_ON** `[31:31]`
    ///
    /// ECC memory clock gate force on register
    #[inline(always)]
    pub const fn mem_clock_gate_force_on(&self) -> bool {
        (self.bits & MultConf::MEM_CLOCK_GATE_FORCE_ON_MASK) != 0
    }

    /// Read **CLK_EN** `[30:30]`
    ///
    /// Write 1 to force on register clock gate.
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & MultConf::CLK_EN_MASK) != 0
    }

    /// Read **VERIFICATION_RESULT** `[29:29]`
    ///
    /// The verification result bit of ECC Accelerator, only valid when calculation is done.
    #[inline(always)]
    pub const fn verification_result(&self) -> bool {
        (self.bits & MultConf::VERIFICATION_RESULT_MASK) != 0
    }

    /// Read **SECURITY_MODE** `[8:8]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn security_mode(&self) -> bool {
        (self.bits & MultConf::SECURITY_MODE_MASK) != 0
    }

    /// Read **WORK_MODE** `[7:4]`
    ///
    /// The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Reserved. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Point Add Mode. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode. 8: mod addition. 9. mod substraction. 10: mod multiplication. 11: mod division.
    #[inline(always)]
    pub const fn work_mode(&self) -> u32 {
        (self.bits & MultConf::WORK_MODE_MASK) >> MultConf::WORK_MODE_OFFSET
    }

    /// Read **MOD_BASE** `[3:3]`
    ///
    /// The mod base of mod operation, only valid in work_mode 8-11. 0: n(order of curve). 1: p(mod base of curve)
    #[inline(always)]
    pub const fn mod_base(&self) -> bool {
        (self.bits & MultConf::MOD_BASE_MASK) != 0
    }

    /// Read **KEY_LENGTH** `[2:2]`
    ///
    /// The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256.
    #[inline(always)]
    pub const fn key_length(&self) -> bool {
        (self.bits & MultConf::KEY_LENGTH_MASK) != 0
    }

    /// Read **START** `[0:0]`
    ///
    /// Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done.
    #[inline(always)]
    pub const fn start(&self) -> bool {
        (self.bits & MultConf::START_MASK) != 0
    }

    /// Set **MEM_CLOCK_GATE_FORCE_ON** `[31:31]`
    ///
    /// ECC memory clock gate force on register
    #[inline(always)]
    pub const fn with_mem_clock_gate_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MultConf::MEM_CLOCK_GATE_FORCE_ON_MASK
            } else {
                self.bits & !MultConf::MEM_CLOCK_GATE_FORCE_ON_MASK
            },
        }
    }

    /// Set **CLK_EN** `[30:30]`
    ///
    /// Write 1 to force on register clock gate.
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MultConf::CLK_EN_MASK
            } else {
                self.bits & !MultConf::CLK_EN_MASK
            },
        }
    }

    /// Set **SECURITY_MODE** `[8:8]`
    ///
    /// Reserved
    #[inline(always)]
    pub const fn with_security_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MultConf::SECURITY_MODE_MASK
            } else {
                self.bits & !MultConf::SECURITY_MODE_MASK
            },
        }
    }

    /// Set **WORK_MODE** `[7:4]`
    ///
    /// The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Reserved. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Point Add Mode. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode. 8: mod addition. 9. mod substraction. 10: mod multiplication. 11: mod division.
    #[inline(always)]
    pub const fn with_work_mode(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !MultConf::WORK_MODE_MASK)
                | ((value << MultConf::WORK_MODE_OFFSET) & MultConf::WORK_MODE_MASK),
        }
    }

    /// Set **MOD_BASE** `[3:3]`
    ///
    /// The mod base of mod operation, only valid in work_mode 8-11. 0: n(order of curve). 1: p(mod base of curve)
    #[inline(always)]
    pub const fn with_mod_base(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MultConf::MOD_BASE_MASK
            } else {
                self.bits & !MultConf::MOD_BASE_MASK
            },
        }
    }

    /// Set **KEY_LENGTH** `[2:2]`
    ///
    /// The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256.
    #[inline(always)]
    pub const fn with_key_length(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MultConf::KEY_LENGTH_MASK
            } else {
                self.bits & !MultConf::KEY_LENGTH_MASK
            },
        }
    }

    /// Set **RESET** `[1:1]`
    ///
    /// Write 1 to reset ECC Accelerator.
    #[inline(always)]
    pub const fn with_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MultConf::RESET_MASK
            } else {
                self.bits & !MultConf::RESET_MASK
            },
        }
    }

    /// Set **START** `[0:0]`
    ///
    /// Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done.
    #[inline(always)]
    pub const fn with_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | MultConf::START_MASK
            } else {
                self.bits & !MultConf::START_MASK
            },
        }
    }
}

impl Default for MultConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl MultConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MultConfVal {
        MultConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MultConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MultConfVal) -> MultConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// MULT_DATE Register
///
/// Version control register
///
/// Address offset: 0x00fc
/// Reset value: 0x02305040
///
/// ## Fields
///
/// - **DATE** `[27:0]` (RW)
///
///   ECC mult version control register
#[repr(transparent)]
pub struct MultDate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for MultDate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl MultDate {
    // ======== Field Constants ========

    /// **DATE**
    ///
    /// ECC mult version control register
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **DATE** `[27:0]` (RW)
    ///
    /// ECC mult version control register
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[27:0]` (RW)
    ///
    /// ECC mult version control register
    #[inline(always)]
    pub fn write_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DATE_MASK) | ((value << Self::DATE_OFFSET) & Self::DATE_MASK)
        });
    }
}

/// MULT_DATE value type for batch read/modify/write operations
///
/// Use `multdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `multdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct MultDateVal {
    bits: u32,
}

impl MultDateVal {
    /// Create from raw bits
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
        Self { bits: 0x02305040 }
    }

    /// Read **DATE** `[27:0]`
    ///
    /// ECC mult version control register
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & MultDate::DATE_MASK) >> MultDate::DATE_OFFSET
    }

    /// Set **DATE** `[27:0]`
    ///
    /// ECC mult version control register
    #[inline(always)]
    pub const fn with_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !MultDate::DATE_MASK)
                | ((value << MultDate::DATE_OFFSET) & MultDate::DATE_MASK),
        }
    }
}

impl Default for MultDateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl MultDate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> MultDateVal {
        MultDateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: MultDateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(MultDateVal) -> MultDateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// K_MEM[0] Register
///
/// The memory that stores k.
///
/// Address offset: 0x0100
#[repr(transparent)]
pub struct KMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for KMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl KMem0 {}

/// K_MEM[1] Register
///
/// The memory that stores k.
///
/// Address offset: 0x0104
#[repr(transparent)]
pub struct KMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for KMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl KMem1 {}

/// K_MEM[2] Register
///
/// The memory that stores k.
///
/// Address offset: 0x0108
#[repr(transparent)]
pub struct KMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for KMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl KMem2 {}

/// K_MEM[3] Register
///
/// The memory that stores k.
///
/// Address offset: 0x010c
#[repr(transparent)]
pub struct KMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for KMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl KMem3 {}

/// K_MEM[4] Register
///
/// The memory that stores k.
///
/// Address offset: 0x0110
#[repr(transparent)]
pub struct KMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for KMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl KMem4 {}

/// K_MEM[5] Register
///
/// The memory that stores k.
///
/// Address offset: 0x0114
#[repr(transparent)]
pub struct KMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for KMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl KMem5 {}

/// K_MEM[6] Register
///
/// The memory that stores k.
///
/// Address offset: 0x0118
#[repr(transparent)]
pub struct KMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for KMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl KMem6 {}

/// K_MEM[7] Register
///
/// The memory that stores k.
///
/// Address offset: 0x011c
#[repr(transparent)]
pub struct KMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for KMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl KMem7 {}

/// PX_MEM[0] Register
///
/// The memory that stores Px.
///
/// Address offset: 0x0120
#[repr(transparent)]
pub struct PxMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PxMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PxMem0 {}

/// PX_MEM[1] Register
///
/// The memory that stores Px.
///
/// Address offset: 0x0124
#[repr(transparent)]
pub struct PxMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PxMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PxMem1 {}

/// PX_MEM[2] Register
///
/// The memory that stores Px.
///
/// Address offset: 0x0128
#[repr(transparent)]
pub struct PxMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PxMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PxMem2 {}

/// PX_MEM[3] Register
///
/// The memory that stores Px.
///
/// Address offset: 0x012c
#[repr(transparent)]
pub struct PxMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PxMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PxMem3 {}

/// PX_MEM[4] Register
///
/// The memory that stores Px.
///
/// Address offset: 0x0130
#[repr(transparent)]
pub struct PxMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PxMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PxMem4 {}

/// PX_MEM[5] Register
///
/// The memory that stores Px.
///
/// Address offset: 0x0134
#[repr(transparent)]
pub struct PxMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PxMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PxMem5 {}

/// PX_MEM[6] Register
///
/// The memory that stores Px.
///
/// Address offset: 0x0138
#[repr(transparent)]
pub struct PxMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PxMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PxMem6 {}

/// PX_MEM[7] Register
///
/// The memory that stores Px.
///
/// Address offset: 0x013c
#[repr(transparent)]
pub struct PxMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PxMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PxMem7 {}

/// PY_MEM[0] Register
///
/// The memory that stores Py.
///
/// Address offset: 0x0140
#[repr(transparent)]
pub struct PyMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PyMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PyMem0 {}

/// PY_MEM[1] Register
///
/// The memory that stores Py.
///
/// Address offset: 0x0144
#[repr(transparent)]
pub struct PyMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PyMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PyMem1 {}

/// PY_MEM[2] Register
///
/// The memory that stores Py.
///
/// Address offset: 0x0148
#[repr(transparent)]
pub struct PyMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PyMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PyMem2 {}

/// PY_MEM[3] Register
///
/// The memory that stores Py.
///
/// Address offset: 0x014c
#[repr(transparent)]
pub struct PyMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PyMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PyMem3 {}

/// PY_MEM[4] Register
///
/// The memory that stores Py.
///
/// Address offset: 0x0150
#[repr(transparent)]
pub struct PyMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PyMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PyMem4 {}

/// PY_MEM[5] Register
///
/// The memory that stores Py.
///
/// Address offset: 0x0154
#[repr(transparent)]
pub struct PyMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PyMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PyMem5 {}

/// PY_MEM[6] Register
///
/// The memory that stores Py.
///
/// Address offset: 0x0158
#[repr(transparent)]
pub struct PyMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PyMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PyMem6 {}

/// PY_MEM[7] Register
///
/// The memory that stores Py.
///
/// Address offset: 0x015c
#[repr(transparent)]
pub struct PyMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PyMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PyMem7 {}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 12],
    /// 0x000c
    /// ECC interrupt raw register, valid in level.
    pub mult_int_raw: MultIntRaw,
    /// 0x0010
    /// ECC interrupt status register.
    pub mult_int_st: MultIntSt,
    /// 0x0014
    /// ECC interrupt enable register.
    pub mult_int_ena: MultIntEna,
    /// 0x0018
    /// ECC interrupt clear register.
    pub mult_int_clr: MultIntClr,
    /// 0x001c
    /// ECC configure register
    pub mult_conf: MultConf,
    _reserved1: [u8; 220],
    /// 0x00fc
    /// Version control register
    pub mult_date: MultDate,
    /// 0x0100
    /// The memory that stores k.
    pub k_mem0: KMem0,
    /// 0x0104
    /// The memory that stores k.
    pub k_mem1: KMem1,
    /// 0x0108
    /// The memory that stores k.
    pub k_mem2: KMem2,
    /// 0x010c
    /// The memory that stores k.
    pub k_mem3: KMem3,
    /// 0x0110
    /// The memory that stores k.
    pub k_mem4: KMem4,
    /// 0x0114
    /// The memory that stores k.
    pub k_mem5: KMem5,
    /// 0x0118
    /// The memory that stores k.
    pub k_mem6: KMem6,
    /// 0x011c
    /// The memory that stores k.
    pub k_mem7: KMem7,
    /// 0x0120
    /// The memory that stores Px.
    pub px_mem0: PxMem0,
    /// 0x0124
    /// The memory that stores Px.
    pub px_mem1: PxMem1,
    /// 0x0128
    /// The memory that stores Px.
    pub px_mem2: PxMem2,
    /// 0x012c
    /// The memory that stores Px.
    pub px_mem3: PxMem3,
    /// 0x0130
    /// The memory that stores Px.
    pub px_mem4: PxMem4,
    /// 0x0134
    /// The memory that stores Px.
    pub px_mem5: PxMem5,
    /// 0x0138
    /// The memory that stores Px.
    pub px_mem6: PxMem6,
    /// 0x013c
    /// The memory that stores Px.
    pub px_mem7: PxMem7,
    /// 0x0140
    /// The memory that stores Py.
    pub py_mem0: PyMem0,
    /// 0x0144
    /// The memory that stores Py.
    pub py_mem1: PyMem1,
    /// 0x0148
    /// The memory that stores Py.
    pub py_mem2: PyMem2,
    /// 0x014c
    /// The memory that stores Py.
    pub py_mem3: PyMem3,
    /// 0x0150
    /// The memory that stores Py.
    pub py_mem4: PyMem4,
    /// 0x0154
    /// The memory that stores Py.
    pub py_mem5: PyMem5,
    /// 0x0158
    /// The memory that stores Py.
    pub py_mem6: PyMem6,
    /// 0x015c
    /// The memory that stores Py.
    pub py_mem7: PyMem7,
}
