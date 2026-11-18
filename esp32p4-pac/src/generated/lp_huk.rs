//! LP_HUK
//! LP_HUK Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x50114000;

/// CLK Register
///
/// HUK Generator clock gate control register
///
/// Address offset: 0x0004
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **MEM_CG_FORCE_ON** `[1]` (RW)
///
///   Write 1 to force on memory clock gate.
///
/// - **EN** `[0]` (RW)
///
///   Write 1 to force on register clock gate.
#[repr(transparent)]
pub struct Clk {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Clk {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Clk {
    // ======== Field Constants ========

    /// **MEM_CG_FORCE_ON**
    ///
    /// Write 1 to force on memory clock gate.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MEM_CG_FORCE_ON_OFFSET: u32 = 1;
    pub const MEM_CG_FORCE_ON_MASK: u32 = 0x00000002;
    pub const MEM_CG_FORCE_ON: u32 = 1 << 1;

    /// **EN**
    ///
    /// Write 1 to force on register clock gate.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MEM_CG_FORCE_ON** `[1]` (RW)
    ///
    /// Write 1 to force on memory clock gate.
    #[inline(always)]
    pub fn mem_cg_force_on(&self) -> bool {
        (self.value.read_bits() & Self::MEM_CG_FORCE_ON_MASK) != 0
    }

    /// Set MEM_CG_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mem_cg_force_on(&self) {
        self.value.modify_bits(|r| r | Self::MEM_CG_FORCE_ON_MASK);
    }

    /// Clear MEM_CG_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mem_cg_force_on(&self) {
        self.value.modify_bits(|r| r & !Self::MEM_CG_FORCE_ON_MASK);
    }

    /// **EN** `[0]` (RW)
    ///
    /// Write 1 to force on register clock gate.
    #[inline(always)]
    pub fn en(&self) -> bool {
        (self.value.read_bits() & Self::EN_MASK) != 0
    }

    /// Set EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_en(&self) {
        self.value.modify_bits(|r| r | Self::EN_MASK);
    }

    /// Clear EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_en(&self) {
        self.value.modify_bits(|r| r & !Self::EN_MASK);
    }
}

/// CLK value type for batch read/modify/write operations
///
/// Use `clk.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clk.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkVal {
    bits: u32,
}

impl ClkVal {
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

    /// Read **MEM_CG_FORCE_ON** `[1:1]`
    ///
    /// Write 1 to force on memory clock gate.
    #[inline(always)]
    pub const fn mem_cg_force_on(&self) -> bool {
        (self.bits & Clk::MEM_CG_FORCE_ON_MASK) != 0
    }

    /// Read **EN** `[0:0]`
    ///
    /// Write 1 to force on register clock gate.
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & Clk::EN_MASK) != 0
    }

    /// Set **MEM_CG_FORCE_ON** `[1:1]`
    ///
    /// Write 1 to force on memory clock gate.
    #[inline(always)]
    pub const fn with_mem_cg_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clk::MEM_CG_FORCE_ON_MASK
            } else {
                self.bits & !Clk::MEM_CG_FORCE_ON_MASK
            },
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// Write 1 to force on register clock gate.
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Clk::EN_MASK
            } else {
                self.bits & !Clk::EN_MASK
            },
        }
    }
}

impl Default for ClkVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Clk {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkVal {
        ClkVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkVal) -> ClkVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// INT_RAW Register
///
/// HUK Generator interrupt raw register, valid in level.
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **POST_DONE_INT_RAW** `[2]` (RO)
///
///   The raw interrupt status bit for the huk_post_done_int interrupt
///
/// - **PROC_DONE_INT_RAW** `[1]` (RO)
///
///   The raw interrupt status bit for the huk_proc_done_int interrupt
///
/// - **PREP_DONE_INT_RAW** `[0]` (RO)
///
///   The raw interrupt status bit for the huk_prep_done_int interrupt
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

    /// **POST_DONE_INT_RAW**
    ///
    /// The raw interrupt status bit for the huk_post_done_int interrupt
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const POST_DONE_INT_RAW_OFFSET: u32 = 2;
    pub const POST_DONE_INT_RAW_MASK: u32 = 0x00000004;
    pub const POST_DONE_INT_RAW: u32 = 1 << 2;

    /// **PROC_DONE_INT_RAW**
    ///
    /// The raw interrupt status bit for the huk_proc_done_int interrupt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PROC_DONE_INT_RAW_OFFSET: u32 = 1;
    pub const PROC_DONE_INT_RAW_MASK: u32 = 0x00000002;
    pub const PROC_DONE_INT_RAW: u32 = 1 << 1;

    /// **PREP_DONE_INT_RAW**
    ///
    /// The raw interrupt status bit for the huk_prep_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PREP_DONE_INT_RAW_OFFSET: u32 = 0;
    pub const PREP_DONE_INT_RAW_MASK: u32 = 0x00000001;
    pub const PREP_DONE_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **POST_DONE_INT_RAW** `[2]` (RO)
    ///
    /// The raw interrupt status bit for the huk_post_done_int interrupt
    #[inline(always)]
    pub fn post_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::POST_DONE_INT_RAW_MASK) != 0
    }

    /// **PROC_DONE_INT_RAW** `[1]` (RO)
    ///
    /// The raw interrupt status bit for the huk_proc_done_int interrupt
    #[inline(always)]
    pub fn proc_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::PROC_DONE_INT_RAW_MASK) != 0
    }

    /// **PREP_DONE_INT_RAW** `[0]` (RO)
    ///
    /// The raw interrupt status bit for the huk_prep_done_int interrupt
    #[inline(always)]
    pub fn prep_done_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::PREP_DONE_INT_RAW_MASK) != 0
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

    /// Read **POST_DONE_INT_RAW** `[2:2]`
    ///
    /// The raw interrupt status bit for the huk_post_done_int interrupt
    #[inline(always)]
    pub const fn post_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::POST_DONE_INT_RAW_MASK) != 0
    }

    /// Read **PROC_DONE_INT_RAW** `[1:1]`
    ///
    /// The raw interrupt status bit for the huk_proc_done_int interrupt
    #[inline(always)]
    pub const fn proc_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::PROC_DONE_INT_RAW_MASK) != 0
    }

    /// Read **PREP_DONE_INT_RAW** `[0:0]`
    ///
    /// The raw interrupt status bit for the huk_prep_done_int interrupt
    #[inline(always)]
    pub const fn prep_done_int_raw(&self) -> bool {
        (self.bits & IntRaw::PREP_DONE_INT_RAW_MASK) != 0
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
}

/// INT_ST Register
///
/// HUK Generator interrupt status register.
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **POST_DONE_INT_ST** `[2]` (RO)
///
///   The masked interrupt status bit for the huk_post_done_int interrupt
///
/// - **PROC_DONE_INT_ST** `[1]` (RO)
///
///   The masked interrupt status bit for the huk_proc_done_int interrupt
///
/// - **PREP_DONE_INT_ST** `[0]` (RO)
///
///   The masked interrupt status bit for the huk_prep_done_int interrupt
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

    /// **POST_DONE_INT_ST**
    ///
    /// The masked interrupt status bit for the huk_post_done_int interrupt
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const POST_DONE_INT_ST_OFFSET: u32 = 2;
    pub const POST_DONE_INT_ST_MASK: u32 = 0x00000004;
    pub const POST_DONE_INT_ST: u32 = 1 << 2;

    /// **PROC_DONE_INT_ST**
    ///
    /// The masked interrupt status bit for the huk_proc_done_int interrupt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PROC_DONE_INT_ST_OFFSET: u32 = 1;
    pub const PROC_DONE_INT_ST_MASK: u32 = 0x00000002;
    pub const PROC_DONE_INT_ST: u32 = 1 << 1;

    /// **PREP_DONE_INT_ST**
    ///
    /// The masked interrupt status bit for the huk_prep_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PREP_DONE_INT_ST_OFFSET: u32 = 0;
    pub const PREP_DONE_INT_ST_MASK: u32 = 0x00000001;
    pub const PREP_DONE_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **POST_DONE_INT_ST** `[2]` (RO)
    ///
    /// The masked interrupt status bit for the huk_post_done_int interrupt
    #[inline(always)]
    pub fn post_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::POST_DONE_INT_ST_MASK) != 0
    }

    /// **PROC_DONE_INT_ST** `[1]` (RO)
    ///
    /// The masked interrupt status bit for the huk_proc_done_int interrupt
    #[inline(always)]
    pub fn proc_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::PROC_DONE_INT_ST_MASK) != 0
    }

    /// **PREP_DONE_INT_ST** `[0]` (RO)
    ///
    /// The masked interrupt status bit for the huk_prep_done_int interrupt
    #[inline(always)]
    pub fn prep_done_int_st(&self) -> bool {
        (self.value.read_bits() & Self::PREP_DONE_INT_ST_MASK) != 0
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

    /// Read **POST_DONE_INT_ST** `[2:2]`
    ///
    /// The masked interrupt status bit for the huk_post_done_int interrupt
    #[inline(always)]
    pub const fn post_done_int_st(&self) -> bool {
        (self.bits & IntSt::POST_DONE_INT_ST_MASK) != 0
    }

    /// Read **PROC_DONE_INT_ST** `[1:1]`
    ///
    /// The masked interrupt status bit for the huk_proc_done_int interrupt
    #[inline(always)]
    pub const fn proc_done_int_st(&self) -> bool {
        (self.bits & IntSt::PROC_DONE_INT_ST_MASK) != 0
    }

    /// Read **PREP_DONE_INT_ST** `[0:0]`
    ///
    /// The masked interrupt status bit for the huk_prep_done_int interrupt
    #[inline(always)]
    pub const fn prep_done_int_st(&self) -> bool {
        (self.bits & IntSt::PREP_DONE_INT_ST_MASK) != 0
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
/// HUK Generator interrupt enable register.
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **POST_DONE_INT_ENA** `[2]` (RW)
///
///   The interrupt enable bit for the huk_post_done_int interrupt
///
/// - **PROC_DONE_INT_ENA** `[1]` (RW)
///
///   The interrupt enable bit for the huk_proc_done_int interrupt
///
/// - **PREP_DONE_INT_ENA** `[0]` (RW)
///
///   The interrupt enable bit for the huk_prep_done_int interrupt
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

    /// **POST_DONE_INT_ENA**
    ///
    /// The interrupt enable bit for the huk_post_done_int interrupt
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const POST_DONE_INT_ENA_OFFSET: u32 = 2;
    pub const POST_DONE_INT_ENA_MASK: u32 = 0x00000004;
    pub const POST_DONE_INT_ENA: u32 = 1 << 2;

    /// **PROC_DONE_INT_ENA**
    ///
    /// The interrupt enable bit for the huk_proc_done_int interrupt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PROC_DONE_INT_ENA_OFFSET: u32 = 1;
    pub const PROC_DONE_INT_ENA_MASK: u32 = 0x00000002;
    pub const PROC_DONE_INT_ENA: u32 = 1 << 1;

    /// **PREP_DONE_INT_ENA**
    ///
    /// The interrupt enable bit for the huk_prep_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PREP_DONE_INT_ENA_OFFSET: u32 = 0;
    pub const PREP_DONE_INT_ENA_MASK: u32 = 0x00000001;
    pub const PREP_DONE_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **POST_DONE_INT_ENA** `[2]` (RW)
    ///
    /// The interrupt enable bit for the huk_post_done_int interrupt
    #[inline(always)]
    pub fn post_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::POST_DONE_INT_ENA_MASK) != 0
    }

    /// Set POST_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_post_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::POST_DONE_INT_ENA_MASK);
    }

    /// Clear POST_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_post_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::POST_DONE_INT_ENA_MASK);
    }

    /// **PROC_DONE_INT_ENA** `[1]` (RW)
    ///
    /// The interrupt enable bit for the huk_proc_done_int interrupt
    #[inline(always)]
    pub fn proc_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::PROC_DONE_INT_ENA_MASK) != 0
    }

    /// Set PROC_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_proc_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::PROC_DONE_INT_ENA_MASK);
    }

    /// Clear PROC_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_proc_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::PROC_DONE_INT_ENA_MASK);
    }

    /// **PREP_DONE_INT_ENA** `[0]` (RW)
    ///
    /// The interrupt enable bit for the huk_prep_done_int interrupt
    #[inline(always)]
    pub fn prep_done_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::PREP_DONE_INT_ENA_MASK) != 0
    }

    /// Set PREP_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_prep_done_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::PREP_DONE_INT_ENA_MASK);
    }

    /// Clear PREP_DONE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_prep_done_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::PREP_DONE_INT_ENA_MASK);
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

    /// Read **POST_DONE_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the huk_post_done_int interrupt
    #[inline(always)]
    pub const fn post_done_int_ena(&self) -> bool {
        (self.bits & IntEna::POST_DONE_INT_ENA_MASK) != 0
    }

    /// Read **PROC_DONE_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the huk_proc_done_int interrupt
    #[inline(always)]
    pub const fn proc_done_int_ena(&self) -> bool {
        (self.bits & IntEna::PROC_DONE_INT_ENA_MASK) != 0
    }

    /// Read **PREP_DONE_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the huk_prep_done_int interrupt
    #[inline(always)]
    pub const fn prep_done_int_ena(&self) -> bool {
        (self.bits & IntEna::PREP_DONE_INT_ENA_MASK) != 0
    }

    /// Set **POST_DONE_INT_ENA** `[2:2]`
    ///
    /// The interrupt enable bit for the huk_post_done_int interrupt
    #[inline(always)]
    pub const fn with_post_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::POST_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::POST_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **PROC_DONE_INT_ENA** `[1:1]`
    ///
    /// The interrupt enable bit for the huk_proc_done_int interrupt
    #[inline(always)]
    pub const fn with_proc_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::PROC_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::PROC_DONE_INT_ENA_MASK
            },
        }
    }

    /// Set **PREP_DONE_INT_ENA** `[0:0]`
    ///
    /// The interrupt enable bit for the huk_prep_done_int interrupt
    #[inline(always)]
    pub const fn with_prep_done_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::PREP_DONE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::PREP_DONE_INT_ENA_MASK
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
/// HUK Generator interrupt clear register.
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **POST_DONE_INT_CLR** `[2]` (WO)
///
///   Set this bit to clear the huk_post_done_int interrupt
///
/// - **PROC_DONE_INT_CLR** `[1]` (WO)
///
///   Set this bit to clear the huk_proc_done_int interrupt
///
/// - **PREP_DONE_INT_CLR** `[0]` (WO)
///
///   Set this bit to clear the huk_prep_done_int interrupt
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

    /// **POST_DONE_INT_CLR**
    ///
    /// Set this bit to clear the huk_post_done_int interrupt
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const POST_DONE_INT_CLR_OFFSET: u32 = 2;
    pub const POST_DONE_INT_CLR_MASK: u32 = 0x00000004;
    pub const POST_DONE_INT_CLR: u32 = 1 << 2;

    /// **PROC_DONE_INT_CLR**
    ///
    /// Set this bit to clear the huk_proc_done_int interrupt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const PROC_DONE_INT_CLR_OFFSET: u32 = 1;
    pub const PROC_DONE_INT_CLR_MASK: u32 = 0x00000002;
    pub const PROC_DONE_INT_CLR: u32 = 1 << 1;

    /// **PREP_DONE_INT_CLR**
    ///
    /// Set this bit to clear the huk_prep_done_int interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PREP_DONE_INT_CLR_OFFSET: u32 = 0;
    pub const PREP_DONE_INT_CLR_MASK: u32 = 0x00000001;
    pub const PREP_DONE_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **POST_DONE_INT_CLR** (WO)
    ///
    /// Set this bit to clear the huk_post_done_int interrupt
    #[inline(always)]
    pub fn trigger_post_done_int_clr(&self) {
        self.value.write_bits(Self::POST_DONE_INT_CLR_MASK);
    }

    /// Trigger **PROC_DONE_INT_CLR** (WO)
    ///
    /// Set this bit to clear the huk_proc_done_int interrupt
    #[inline(always)]
    pub fn trigger_proc_done_int_clr(&self) {
        self.value.write_bits(Self::PROC_DONE_INT_CLR_MASK);
    }

    /// Trigger **PREP_DONE_INT_CLR** (WO)
    ///
    /// Set this bit to clear the huk_prep_done_int interrupt
    #[inline(always)]
    pub fn trigger_prep_done_int_clr(&self) {
        self.value.write_bits(Self::PREP_DONE_INT_CLR_MASK);
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

    /// Set **POST_DONE_INT_CLR** `[2:2]`
    ///
    /// Set this bit to clear the huk_post_done_int interrupt
    #[inline(always)]
    pub const fn with_post_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::POST_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::POST_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **PROC_DONE_INT_CLR** `[1:1]`
    ///
    /// Set this bit to clear the huk_proc_done_int interrupt
    #[inline(always)]
    pub const fn with_proc_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::PROC_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::PROC_DONE_INT_CLR_MASK
            },
        }
    }

    /// Set **PREP_DONE_INT_CLR** `[0:0]`
    ///
    /// Set this bit to clear the huk_prep_done_int interrupt
    #[inline(always)]
    pub const fn with_prep_done_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::PREP_DONE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::PREP_DONE_INT_CLR_MASK
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

/// CONF Register
///
/// HUK Generator configuration register
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **MODE** `[0]` (RW)
///
///   Set this field to choose the huk process. 1: process huk generate mode. 0: process huk recovery mode.
#[repr(transparent)]
pub struct Conf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Conf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Conf {
    // ======== Field Constants ========

    /// **MODE**
    ///
    /// Set this field to choose the huk process. 1: process huk generate mode. 0: process huk recovery mode.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MODE_OFFSET: u32 = 0;
    pub const MODE_MASK: u32 = 0x00000001;
    pub const MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MODE** `[0]` (RW)
    ///
    /// Set this field to choose the huk process. 1: process huk generate mode. 0: process huk recovery mode.
    #[inline(always)]
    pub fn mode(&self) -> bool {
        (self.value.read_bits() & Self::MODE_MASK) != 0
    }

    /// Set MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mode(&self) {
        self.value.modify_bits(|r| r | Self::MODE_MASK);
    }

    /// Clear MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mode(&self) {
        self.value.modify_bits(|r| r & !Self::MODE_MASK);
    }
}

/// CONF value type for batch read/modify/write operations
///
/// Use `conf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `conf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ConfVal {
    bits: u32,
}

impl ConfVal {
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

    /// Read **MODE** `[0:0]`
    ///
    /// Set this field to choose the huk process. 1: process huk generate mode. 0: process huk recovery mode.
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        (self.bits & Conf::MODE_MASK) != 0
    }

    /// Set **MODE** `[0:0]`
    ///
    /// Set this field to choose the huk process. 1: process huk generate mode. 0: process huk recovery mode.
    #[inline(always)]
    pub const fn with_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::MODE_MASK
            } else {
                self.bits & !Conf::MODE_MASK
            },
        }
    }
}

impl Default for ConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Conf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ConfVal {
        ConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ConfVal) -> ConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// START Register
///
/// HUK Generator control register
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **CONTINUE** `[1]` (WO)
///
///   Write 1 to start HUK Generator at IDLE state.
///
/// - **START** `[0]` (WO)
///
///   Write 1 to continue HUK Generator operation at LOAD/GAIN state.
#[repr(transparent)]
pub struct Start {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Start {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Start {
    // ======== Field Constants ========

    /// **CONTINUE**
    ///
    /// Write 1 to start HUK Generator at IDLE state.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CONTINUE_OFFSET: u32 = 1;
    pub const CONTINUE_MASK: u32 = 0x00000002;
    pub const CONTINUE: u32 = 1 << 1;

    /// **START**
    ///
    /// Write 1 to continue HUK Generator operation at LOAD/GAIN state.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const START_OFFSET: u32 = 0;
    pub const START_MASK: u32 = 0x00000001;
    pub const START: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CONTINUE** (WO)
    ///
    /// Write 1 to start HUK Generator at IDLE state.
    #[inline(always)]
    pub fn trigger_continue_(&self) {
        self.value.write_bits(Self::CONTINUE_MASK);
    }

    /// Trigger **START** (WO)
    ///
    /// Write 1 to continue HUK Generator operation at LOAD/GAIN state.
    #[inline(always)]
    pub fn trigger_start(&self) {
        self.value.write_bits(Self::START_MASK);
    }
}

/// START value type for batch read/modify/write operations
///
/// Use `start.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `start.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StartVal {
    bits: u32,
}

impl StartVal {
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

    /// Set **CONTINUE** `[1:1]`
    ///
    /// Write 1 to start HUK Generator at IDLE state.
    #[inline(always)]
    pub const fn with_continue_(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Start::CONTINUE_MASK
            } else {
                self.bits & !Start::CONTINUE_MASK
            },
        }
    }

    /// Set **START** `[0:0]`
    ///
    /// Write 1 to continue HUK Generator operation at LOAD/GAIN state.
    #[inline(always)]
    pub const fn with_start(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Start::START_MASK
            } else {
                self.bits & !Start::START_MASK
            },
        }
    }
}

impl Default for StartVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Start {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: StartVal) {
        self.value.write_bits(val.bits())
    }
}

/// STATE Register
///
/// HUK Generator state register
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **STATE** `[1:0]` (RO)
///
///   The state of HUK Generator. 0: IDLE. 1: LOAD. 2: GAIN. 3: BUSY.
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
    /// The state of HUK Generator. 0: IDLE. 1: LOAD. 2: GAIN. 3: BUSY.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const STATE_OFFSET: u32 = 0;
    pub const STATE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **STATE** `[1:0]` (RO)
    ///
    /// The state of HUK Generator. 0: IDLE. 1: LOAD. 2: GAIN. 3: BUSY.
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
    /// The state of HUK Generator. 0: IDLE. 1: LOAD. 2: GAIN. 3: BUSY.
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

/// STATUS Register
///
/// HUK Generator HUK status register
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **RISK_LEVEL** `[4:2]` (RO)
///
///   The risk level of HUK. 0-6: the higher the risk level is, the more error bits there are in the PUF SRAM. 7: Error Level, HUK is invalid.
///
/// - **STATUS** `[1:0]` (RO)
///
///   The HUK generation status. 0: HUK is not generated. 1: HUK is generated and valid. 2: HUK is generated but invalid. 3: reserved.
#[repr(transparent)]
pub struct Status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Status {
    // ======== Field Constants ========

    /// **RISK_LEVEL**
    ///
    /// The risk level of HUK. 0-6: the higher the risk level is, the more error bits there are in the PUF SRAM. 7: Error Level, HUK is invalid.
    ///
    /// Bits `[4:2]`, mask `0x0000001c`
    pub const RISK_LEVEL_OFFSET: u32 = 2;
    pub const RISK_LEVEL_MASK: u32 = 0x0000001c;

    /// **STATUS**
    ///
    /// The HUK generation status. 0: HUK is not generated. 1: HUK is generated and valid. 2: HUK is generated but invalid. 3: reserved.
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const STATUS_OFFSET: u32 = 0;
    pub const STATUS_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **RISK_LEVEL** `[4:2]` (RO)
    ///
    /// The risk level of HUK. 0-6: the higher the risk level is, the more error bits there are in the PUF SRAM. 7: Error Level, HUK is invalid.
    #[inline(always)]
    pub fn risk_level(&self) -> u32 {
        (self.value.read_bits() & Self::RISK_LEVEL_MASK) >> Self::RISK_LEVEL_OFFSET
    }

    /// **STATUS** `[1:0]` (RO)
    ///
    /// The HUK generation status. 0: HUK is not generated. 1: HUK is generated and valid. 2: HUK is generated but invalid. 3: reserved.
    #[inline(always)]
    pub fn status(&self) -> u32 {
        (self.value.read_bits() & Self::STATUS_MASK) >> Self::STATUS_OFFSET
    }
}

/// STATUS value type for batch read/modify/write operations
///
/// Use `status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct StatusVal {
    bits: u32,
}

impl StatusVal {
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

    /// Read **RISK_LEVEL** `[4:2]`
    ///
    /// The risk level of HUK. 0-6: the higher the risk level is, the more error bits there are in the PUF SRAM. 7: Error Level, HUK is invalid.
    #[inline(always)]
    pub const fn risk_level(&self) -> u32 {
        (self.bits & Status::RISK_LEVEL_MASK) >> Status::RISK_LEVEL_OFFSET
    }

    /// Read **STATUS** `[1:0]`
    ///
    /// The HUK generation status. 0: HUK is not generated. 1: HUK is generated and valid. 2: HUK is generated but invalid. 3: reserved.
    #[inline(always)]
    pub const fn status(&self) -> u32 {
        (self.bits & Status::STATUS_MASK) >> Status::STATUS_OFFSET
    }
}

impl Default for StatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> StatusVal {
        StatusVal::from_bits(self.value.read_bits())
    }
}

/// DATE Register
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
///   HUK Generator version control register.
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
    /// HUK Generator version control register.
    ///
    /// Bits `[27:0]`, mask `0x0fffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0x0fffffff;

    // ======== Field Accessors ========

    /// **DATE** `[27:0]` (RW)
    ///
    /// HUK Generator version control register.
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[27:0]` (RW)
    ///
    /// HUK Generator version control register.
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
        Self { bits: 0x02305040 }
    }

    /// Read **DATE** `[27:0]`
    ///
    /// HUK Generator version control register.
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[27:0]`
    ///
    /// HUK Generator version control register.
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

/// INFO_MEM[0] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0100
#[repr(transparent)]
pub struct InfoMem0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem0 {}

/// INFO_MEM[1] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0104
#[repr(transparent)]
pub struct InfoMem1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem1 {}

/// INFO_MEM[2] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0108
#[repr(transparent)]
pub struct InfoMem2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem2 {}

/// INFO_MEM[3] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x010c
#[repr(transparent)]
pub struct InfoMem3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem3 {}

/// INFO_MEM[4] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0110
#[repr(transparent)]
pub struct InfoMem4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem4 {}

/// INFO_MEM[5] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0114
#[repr(transparent)]
pub struct InfoMem5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem5 {}

/// INFO_MEM[6] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0118
#[repr(transparent)]
pub struct InfoMem6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem6 {}

/// INFO_MEM[7] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x011c
#[repr(transparent)]
pub struct InfoMem7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem7 {}

/// INFO_MEM[8] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0120
#[repr(transparent)]
pub struct InfoMem8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem8 {}

/// INFO_MEM[9] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0124
#[repr(transparent)]
pub struct InfoMem9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem9 {}

/// INFO_MEM[10] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0128
#[repr(transparent)]
pub struct InfoMem10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem10 {}

/// INFO_MEM[11] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x012c
#[repr(transparent)]
pub struct InfoMem11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem11 {}

/// INFO_MEM[12] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0130
#[repr(transparent)]
pub struct InfoMem12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem12 {}

/// INFO_MEM[13] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0134
#[repr(transparent)]
pub struct InfoMem13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem13 {}

/// INFO_MEM[14] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0138
#[repr(transparent)]
pub struct InfoMem14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem14 {}

/// INFO_MEM[15] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x013c
#[repr(transparent)]
pub struct InfoMem15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem15 {}

/// INFO_MEM[16] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0140
#[repr(transparent)]
pub struct InfoMem16 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem16 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem16 {}

/// INFO_MEM[17] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0144
#[repr(transparent)]
pub struct InfoMem17 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem17 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem17 {}

/// INFO_MEM[18] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0148
#[repr(transparent)]
pub struct InfoMem18 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem18 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem18 {}

/// INFO_MEM[19] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x014c
#[repr(transparent)]
pub struct InfoMem19 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem19 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem19 {}

/// INFO_MEM[20] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0150
#[repr(transparent)]
pub struct InfoMem20 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem20 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem20 {}

/// INFO_MEM[21] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0154
#[repr(transparent)]
pub struct InfoMem21 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem21 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem21 {}

/// INFO_MEM[22] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0158
#[repr(transparent)]
pub struct InfoMem22 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem22 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem22 {}

/// INFO_MEM[23] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x015c
#[repr(transparent)]
pub struct InfoMem23 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem23 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem23 {}

/// INFO_MEM[24] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0160
#[repr(transparent)]
pub struct InfoMem24 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem24 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem24 {}

/// INFO_MEM[25] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0164
#[repr(transparent)]
pub struct InfoMem25 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem25 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem25 {}

/// INFO_MEM[26] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0168
#[repr(transparent)]
pub struct InfoMem26 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem26 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem26 {}

/// INFO_MEM[27] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x016c
#[repr(transparent)]
pub struct InfoMem27 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem27 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem27 {}

/// INFO_MEM[28] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0170
#[repr(transparent)]
pub struct InfoMem28 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem28 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem28 {}

/// INFO_MEM[29] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0174
#[repr(transparent)]
pub struct InfoMem29 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem29 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem29 {}

/// INFO_MEM[30] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0178
#[repr(transparent)]
pub struct InfoMem30 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem30 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem30 {}

/// INFO_MEM[31] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x017c
#[repr(transparent)]
pub struct InfoMem31 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem31 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem31 {}

/// INFO_MEM[32] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0180
#[repr(transparent)]
pub struct InfoMem32 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem32 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem32 {}

/// INFO_MEM[33] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0184
#[repr(transparent)]
pub struct InfoMem33 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem33 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem33 {}

/// INFO_MEM[34] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0188
#[repr(transparent)]
pub struct InfoMem34 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem34 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem34 {}

/// INFO_MEM[35] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x018c
#[repr(transparent)]
pub struct InfoMem35 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem35 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem35 {}

/// INFO_MEM[36] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0190
#[repr(transparent)]
pub struct InfoMem36 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem36 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem36 {}

/// INFO_MEM[37] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0194
#[repr(transparent)]
pub struct InfoMem37 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem37 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem37 {}

/// INFO_MEM[38] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0198
#[repr(transparent)]
pub struct InfoMem38 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem38 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem38 {}

/// INFO_MEM[39] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x019c
#[repr(transparent)]
pub struct InfoMem39 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem39 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem39 {}

/// INFO_MEM[40] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01a0
#[repr(transparent)]
pub struct InfoMem40 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem40 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem40 {}

/// INFO_MEM[41] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01a4
#[repr(transparent)]
pub struct InfoMem41 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem41 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem41 {}

/// INFO_MEM[42] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01a8
#[repr(transparent)]
pub struct InfoMem42 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem42 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem42 {}

/// INFO_MEM[43] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01ac
#[repr(transparent)]
pub struct InfoMem43 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem43 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem43 {}

/// INFO_MEM[44] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01b0
#[repr(transparent)]
pub struct InfoMem44 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem44 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem44 {}

/// INFO_MEM[45] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01b4
#[repr(transparent)]
pub struct InfoMem45 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem45 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem45 {}

/// INFO_MEM[46] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01b8
#[repr(transparent)]
pub struct InfoMem46 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem46 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem46 {}

/// INFO_MEM[47] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01bc
#[repr(transparent)]
pub struct InfoMem47 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem47 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem47 {}

/// INFO_MEM[48] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01c0
#[repr(transparent)]
pub struct InfoMem48 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem48 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem48 {}

/// INFO_MEM[49] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01c4
#[repr(transparent)]
pub struct InfoMem49 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem49 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem49 {}

/// INFO_MEM[50] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01c8
#[repr(transparent)]
pub struct InfoMem50 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem50 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem50 {}

/// INFO_MEM[51] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01cc
#[repr(transparent)]
pub struct InfoMem51 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem51 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem51 {}

/// INFO_MEM[52] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01d0
#[repr(transparent)]
pub struct InfoMem52 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem52 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem52 {}

/// INFO_MEM[53] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01d4
#[repr(transparent)]
pub struct InfoMem53 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem53 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem53 {}

/// INFO_MEM[54] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01d8
#[repr(transparent)]
pub struct InfoMem54 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem54 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem54 {}

/// INFO_MEM[55] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01dc
#[repr(transparent)]
pub struct InfoMem55 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem55 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem55 {}

/// INFO_MEM[56] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01e0
#[repr(transparent)]
pub struct InfoMem56 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem56 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem56 {}

/// INFO_MEM[57] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01e4
#[repr(transparent)]
pub struct InfoMem57 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem57 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem57 {}

/// INFO_MEM[58] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01e8
#[repr(transparent)]
pub struct InfoMem58 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem58 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem58 {}

/// INFO_MEM[59] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01ec
#[repr(transparent)]
pub struct InfoMem59 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem59 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem59 {}

/// INFO_MEM[60] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01f0
#[repr(transparent)]
pub struct InfoMem60 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem60 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem60 {}

/// INFO_MEM[61] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01f4
#[repr(transparent)]
pub struct InfoMem61 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem61 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem61 {}

/// INFO_MEM[62] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01f8
#[repr(transparent)]
pub struct InfoMem62 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem62 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem62 {}

/// INFO_MEM[63] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x01fc
#[repr(transparent)]
pub struct InfoMem63 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem63 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem63 {}

/// INFO_MEM[64] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0200
#[repr(transparent)]
pub struct InfoMem64 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem64 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem64 {}

/// INFO_MEM[65] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0204
#[repr(transparent)]
pub struct InfoMem65 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem65 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem65 {}

/// INFO_MEM[66] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0208
#[repr(transparent)]
pub struct InfoMem66 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem66 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem66 {}

/// INFO_MEM[67] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x020c
#[repr(transparent)]
pub struct InfoMem67 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem67 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem67 {}

/// INFO_MEM[68] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0210
#[repr(transparent)]
pub struct InfoMem68 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem68 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem68 {}

/// INFO_MEM[69] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0214
#[repr(transparent)]
pub struct InfoMem69 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem69 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem69 {}

/// INFO_MEM[70] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0218
#[repr(transparent)]
pub struct InfoMem70 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem70 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem70 {}

/// INFO_MEM[71] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x021c
#[repr(transparent)]
pub struct InfoMem71 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem71 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem71 {}

/// INFO_MEM[72] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0220
#[repr(transparent)]
pub struct InfoMem72 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem72 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem72 {}

/// INFO_MEM[73] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0224
#[repr(transparent)]
pub struct InfoMem73 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem73 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem73 {}

/// INFO_MEM[74] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0228
#[repr(transparent)]
pub struct InfoMem74 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem74 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem74 {}

/// INFO_MEM[75] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x022c
#[repr(transparent)]
pub struct InfoMem75 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem75 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem75 {}

/// INFO_MEM[76] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0230
#[repr(transparent)]
pub struct InfoMem76 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem76 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem76 {}

/// INFO_MEM[77] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0234
#[repr(transparent)]
pub struct InfoMem77 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem77 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem77 {}

/// INFO_MEM[78] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0238
#[repr(transparent)]
pub struct InfoMem78 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem78 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem78 {}

/// INFO_MEM[79] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x023c
#[repr(transparent)]
pub struct InfoMem79 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem79 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem79 {}

/// INFO_MEM[80] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0240
#[repr(transparent)]
pub struct InfoMem80 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem80 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem80 {}

/// INFO_MEM[81] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0244
#[repr(transparent)]
pub struct InfoMem81 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem81 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem81 {}

/// INFO_MEM[82] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0248
#[repr(transparent)]
pub struct InfoMem82 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem82 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem82 {}

/// INFO_MEM[83] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x024c
#[repr(transparent)]
pub struct InfoMem83 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem83 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem83 {}

/// INFO_MEM[84] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0250
#[repr(transparent)]
pub struct InfoMem84 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem84 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem84 {}

/// INFO_MEM[85] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0254
#[repr(transparent)]
pub struct InfoMem85 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem85 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem85 {}

/// INFO_MEM[86] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0258
#[repr(transparent)]
pub struct InfoMem86 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem86 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem86 {}

/// INFO_MEM[87] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x025c
#[repr(transparent)]
pub struct InfoMem87 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem87 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem87 {}

/// INFO_MEM[88] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0260
#[repr(transparent)]
pub struct InfoMem88 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem88 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem88 {}

/// INFO_MEM[89] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0264
#[repr(transparent)]
pub struct InfoMem89 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem89 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem89 {}

/// INFO_MEM[90] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0268
#[repr(transparent)]
pub struct InfoMem90 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem90 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem90 {}

/// INFO_MEM[91] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x026c
#[repr(transparent)]
pub struct InfoMem91 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem91 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem91 {}

/// INFO_MEM[92] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0270
#[repr(transparent)]
pub struct InfoMem92 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem92 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem92 {}

/// INFO_MEM[93] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0274
#[repr(transparent)]
pub struct InfoMem93 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem93 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem93 {}

/// INFO_MEM[94] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x0278
#[repr(transparent)]
pub struct InfoMem94 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem94 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem94 {}

/// INFO_MEM[95] Register
///
/// The memory that stores HUK info.
///
/// Address offset: 0x027c
#[repr(transparent)]
pub struct InfoMem95 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for InfoMem95 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl InfoMem95 {}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4],
    /// 0x0004
    /// HUK Generator clock gate control register
    pub clk: Clk,
    /// 0x0008
    /// HUK Generator interrupt raw register, valid in level.
    pub int_raw: IntRaw,
    /// 0x000c
    /// HUK Generator interrupt status register.
    pub int_st: IntSt,
    /// 0x0010
    /// HUK Generator interrupt enable register.
    pub int_ena: IntEna,
    /// 0x0014
    /// HUK Generator interrupt clear register.
    pub int_clr: IntClr,
    _reserved1: [u8; 8],
    /// 0x0020
    /// HUK Generator configuration register
    pub conf: Conf,
    /// 0x0024
    /// HUK Generator control register
    pub start: Start,
    /// 0x0028
    /// HUK Generator state register
    pub state: State,
    _reserved2: [u8; 8],
    /// 0x0034
    /// HUK Generator HUK status register
    pub status: Status,
    _reserved3: [u8; 196],
    /// 0x00fc
    /// Version control register
    pub date: Date,
    /// 0x0100
    /// The memory that stores HUK info.
    pub info_mem0: InfoMem0,
    /// 0x0104
    /// The memory that stores HUK info.
    pub info_mem1: InfoMem1,
    /// 0x0108
    /// The memory that stores HUK info.
    pub info_mem2: InfoMem2,
    /// 0x010c
    /// The memory that stores HUK info.
    pub info_mem3: InfoMem3,
    /// 0x0110
    /// The memory that stores HUK info.
    pub info_mem4: InfoMem4,
    /// 0x0114
    /// The memory that stores HUK info.
    pub info_mem5: InfoMem5,
    /// 0x0118
    /// The memory that stores HUK info.
    pub info_mem6: InfoMem6,
    /// 0x011c
    /// The memory that stores HUK info.
    pub info_mem7: InfoMem7,
    /// 0x0120
    /// The memory that stores HUK info.
    pub info_mem8: InfoMem8,
    /// 0x0124
    /// The memory that stores HUK info.
    pub info_mem9: InfoMem9,
    /// 0x0128
    /// The memory that stores HUK info.
    pub info_mem10: InfoMem10,
    /// 0x012c
    /// The memory that stores HUK info.
    pub info_mem11: InfoMem11,
    /// 0x0130
    /// The memory that stores HUK info.
    pub info_mem12: InfoMem12,
    /// 0x0134
    /// The memory that stores HUK info.
    pub info_mem13: InfoMem13,
    /// 0x0138
    /// The memory that stores HUK info.
    pub info_mem14: InfoMem14,
    /// 0x013c
    /// The memory that stores HUK info.
    pub info_mem15: InfoMem15,
    /// 0x0140
    /// The memory that stores HUK info.
    pub info_mem16: InfoMem16,
    /// 0x0144
    /// The memory that stores HUK info.
    pub info_mem17: InfoMem17,
    /// 0x0148
    /// The memory that stores HUK info.
    pub info_mem18: InfoMem18,
    /// 0x014c
    /// The memory that stores HUK info.
    pub info_mem19: InfoMem19,
    /// 0x0150
    /// The memory that stores HUK info.
    pub info_mem20: InfoMem20,
    /// 0x0154
    /// The memory that stores HUK info.
    pub info_mem21: InfoMem21,
    /// 0x0158
    /// The memory that stores HUK info.
    pub info_mem22: InfoMem22,
    /// 0x015c
    /// The memory that stores HUK info.
    pub info_mem23: InfoMem23,
    /// 0x0160
    /// The memory that stores HUK info.
    pub info_mem24: InfoMem24,
    /// 0x0164
    /// The memory that stores HUK info.
    pub info_mem25: InfoMem25,
    /// 0x0168
    /// The memory that stores HUK info.
    pub info_mem26: InfoMem26,
    /// 0x016c
    /// The memory that stores HUK info.
    pub info_mem27: InfoMem27,
    /// 0x0170
    /// The memory that stores HUK info.
    pub info_mem28: InfoMem28,
    /// 0x0174
    /// The memory that stores HUK info.
    pub info_mem29: InfoMem29,
    /// 0x0178
    /// The memory that stores HUK info.
    pub info_mem30: InfoMem30,
    /// 0x017c
    /// The memory that stores HUK info.
    pub info_mem31: InfoMem31,
    /// 0x0180
    /// The memory that stores HUK info.
    pub info_mem32: InfoMem32,
    /// 0x0184
    /// The memory that stores HUK info.
    pub info_mem33: InfoMem33,
    /// 0x0188
    /// The memory that stores HUK info.
    pub info_mem34: InfoMem34,
    /// 0x018c
    /// The memory that stores HUK info.
    pub info_mem35: InfoMem35,
    /// 0x0190
    /// The memory that stores HUK info.
    pub info_mem36: InfoMem36,
    /// 0x0194
    /// The memory that stores HUK info.
    pub info_mem37: InfoMem37,
    /// 0x0198
    /// The memory that stores HUK info.
    pub info_mem38: InfoMem38,
    /// 0x019c
    /// The memory that stores HUK info.
    pub info_mem39: InfoMem39,
    /// 0x01a0
    /// The memory that stores HUK info.
    pub info_mem40: InfoMem40,
    /// 0x01a4
    /// The memory that stores HUK info.
    pub info_mem41: InfoMem41,
    /// 0x01a8
    /// The memory that stores HUK info.
    pub info_mem42: InfoMem42,
    /// 0x01ac
    /// The memory that stores HUK info.
    pub info_mem43: InfoMem43,
    /// 0x01b0
    /// The memory that stores HUK info.
    pub info_mem44: InfoMem44,
    /// 0x01b4
    /// The memory that stores HUK info.
    pub info_mem45: InfoMem45,
    /// 0x01b8
    /// The memory that stores HUK info.
    pub info_mem46: InfoMem46,
    /// 0x01bc
    /// The memory that stores HUK info.
    pub info_mem47: InfoMem47,
    /// 0x01c0
    /// The memory that stores HUK info.
    pub info_mem48: InfoMem48,
    /// 0x01c4
    /// The memory that stores HUK info.
    pub info_mem49: InfoMem49,
    /// 0x01c8
    /// The memory that stores HUK info.
    pub info_mem50: InfoMem50,
    /// 0x01cc
    /// The memory that stores HUK info.
    pub info_mem51: InfoMem51,
    /// 0x01d0
    /// The memory that stores HUK info.
    pub info_mem52: InfoMem52,
    /// 0x01d4
    /// The memory that stores HUK info.
    pub info_mem53: InfoMem53,
    /// 0x01d8
    /// The memory that stores HUK info.
    pub info_mem54: InfoMem54,
    /// 0x01dc
    /// The memory that stores HUK info.
    pub info_mem55: InfoMem55,
    /// 0x01e0
    /// The memory that stores HUK info.
    pub info_mem56: InfoMem56,
    /// 0x01e4
    /// The memory that stores HUK info.
    pub info_mem57: InfoMem57,
    /// 0x01e8
    /// The memory that stores HUK info.
    pub info_mem58: InfoMem58,
    /// 0x01ec
    /// The memory that stores HUK info.
    pub info_mem59: InfoMem59,
    /// 0x01f0
    /// The memory that stores HUK info.
    pub info_mem60: InfoMem60,
    /// 0x01f4
    /// The memory that stores HUK info.
    pub info_mem61: InfoMem61,
    /// 0x01f8
    /// The memory that stores HUK info.
    pub info_mem62: InfoMem62,
    /// 0x01fc
    /// The memory that stores HUK info.
    pub info_mem63: InfoMem63,
    /// 0x0200
    /// The memory that stores HUK info.
    pub info_mem64: InfoMem64,
    /// 0x0204
    /// The memory that stores HUK info.
    pub info_mem65: InfoMem65,
    /// 0x0208
    /// The memory that stores HUK info.
    pub info_mem66: InfoMem66,
    /// 0x020c
    /// The memory that stores HUK info.
    pub info_mem67: InfoMem67,
    /// 0x0210
    /// The memory that stores HUK info.
    pub info_mem68: InfoMem68,
    /// 0x0214
    /// The memory that stores HUK info.
    pub info_mem69: InfoMem69,
    /// 0x0218
    /// The memory that stores HUK info.
    pub info_mem70: InfoMem70,
    /// 0x021c
    /// The memory that stores HUK info.
    pub info_mem71: InfoMem71,
    /// 0x0220
    /// The memory that stores HUK info.
    pub info_mem72: InfoMem72,
    /// 0x0224
    /// The memory that stores HUK info.
    pub info_mem73: InfoMem73,
    /// 0x0228
    /// The memory that stores HUK info.
    pub info_mem74: InfoMem74,
    /// 0x022c
    /// The memory that stores HUK info.
    pub info_mem75: InfoMem75,
    /// 0x0230
    /// The memory that stores HUK info.
    pub info_mem76: InfoMem76,
    /// 0x0234
    /// The memory that stores HUK info.
    pub info_mem77: InfoMem77,
    /// 0x0238
    /// The memory that stores HUK info.
    pub info_mem78: InfoMem78,
    /// 0x023c
    /// The memory that stores HUK info.
    pub info_mem79: InfoMem79,
    /// 0x0240
    /// The memory that stores HUK info.
    pub info_mem80: InfoMem80,
    /// 0x0244
    /// The memory that stores HUK info.
    pub info_mem81: InfoMem81,
    /// 0x0248
    /// The memory that stores HUK info.
    pub info_mem82: InfoMem82,
    /// 0x024c
    /// The memory that stores HUK info.
    pub info_mem83: InfoMem83,
    /// 0x0250
    /// The memory that stores HUK info.
    pub info_mem84: InfoMem84,
    /// 0x0254
    /// The memory that stores HUK info.
    pub info_mem85: InfoMem85,
    /// 0x0258
    /// The memory that stores HUK info.
    pub info_mem86: InfoMem86,
    /// 0x025c
    /// The memory that stores HUK info.
    pub info_mem87: InfoMem87,
    /// 0x0260
    /// The memory that stores HUK info.
    pub info_mem88: InfoMem88,
    /// 0x0264
    /// The memory that stores HUK info.
    pub info_mem89: InfoMem89,
    /// 0x0268
    /// The memory that stores HUK info.
    pub info_mem90: InfoMem90,
    /// 0x026c
    /// The memory that stores HUK info.
    pub info_mem91: InfoMem91,
    /// 0x0270
    /// The memory that stores HUK info.
    pub info_mem92: InfoMem92,
    /// 0x0274
    /// The memory that stores HUK info.
    pub info_mem93: InfoMem93,
    /// 0x0278
    /// The memory that stores HUK info.
    pub info_mem94: InfoMem94,
    /// 0x027c
    /// The memory that stores HUK info.
    pub info_mem95: InfoMem95,
}
