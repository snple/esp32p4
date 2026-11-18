//! HP_SYS
//! High-Power System

#![allow(dead_code)]

pub const BASE: u32 = 0x500e5000;

/// VER_DATE Register
///
/// NA
///
/// Address offset: 0x0000
/// Reset value: 0x20230519
///
/// ## Fields
///
/// - **REG_VER_DATE** `[31:0]` (RW)
///
///   NA
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
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_VER_DATE_OFFSET: u32 = 0;
    pub const REG_VER_DATE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_VER_DATE** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_ver_date(&self) -> u32 {
        (self.value.read_bits() & Self::REG_VER_DATE_MASK) >> Self::REG_VER_DATE_OFFSET
    }

    /// Write **REG_VER_DATE** `[31:0]` (RW)
    ///
    /// NA
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
        Self { bits: 0x20230519 }
    }

    /// Read **REG_VER_DATE** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_ver_date(&self) -> u32 {
        (self.bits & VerDate::REG_VER_DATE_MASK) >> VerDate::REG_VER_DATE_OFFSET
    }

    /// Set **REG_VER_DATE** `[31:0]`
    ///
    /// NA
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

/// CLK_EN Register
///
/// NA
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **REG_CLK_EN** `[0]` (RW)
///
///   NA
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
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_CLK_EN_OFFSET: u32 = 0;
    pub const REG_CLK_EN_MASK: u32 = 0x00000001;
    pub const REG_CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_CLK_EN** `[0]` (RW)
    ///
    /// NA
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

    /// Read **REG_CLK_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_clk_en(&self) -> bool {
        (self.bits & ClkEn::REG_CLK_EN_MASK) != 0
    }

    /// Set **REG_CLK_EN** `[0:0]`
    ///
    /// NA
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
        Self { bits: 0 }
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

/// CPU_INTR_FROM_CPU_0 Register
///
/// NA
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **CPU_INTR_FROM_CPU_0** `[0]` (RW)
///
///   set 1 will triger a interrupt
#[repr(transparent)]
pub struct CpuIntrFromCpu0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CpuIntrFromCpu0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CpuIntrFromCpu0 {
    // ======== Field Constants ========

    /// **CPU_INTR_FROM_CPU_0**
    ///
    /// set 1 will triger a interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CPU_INTR_FROM_CPU_0_OFFSET: u32 = 0;
    pub const CPU_INTR_FROM_CPU_0_MASK: u32 = 0x00000001;
    pub const CPU_INTR_FROM_CPU_0: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CPU_INTR_FROM_CPU_0** `[0]` (RW)
    ///
    /// set 1 will triger a interrupt
    #[inline(always)]
    pub fn cpu_intr_from_cpu_0(&self) -> bool {
        (self.value.read_bits() & Self::CPU_INTR_FROM_CPU_0_MASK) != 0
    }

    /// Set CPU_INTR_FROM_CPU_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cpu_intr_from_cpu_0(&self) {
        self.value
            .modify_bits(|r| r | Self::CPU_INTR_FROM_CPU_0_MASK);
    }

    /// Clear CPU_INTR_FROM_CPU_0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cpu_intr_from_cpu_0(&self) {
        self.value
            .modify_bits(|r| r & !Self::CPU_INTR_FROM_CPU_0_MASK);
    }
}

/// CPU_INTR_FROM_CPU_0 value type for batch read/modify/write operations
///
/// Use `cpuintrfromcpu0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cpuintrfromcpu0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CpuIntrFromCpu0Val {
    bits: u32,
}

impl CpuIntrFromCpu0Val {
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

    /// Read **CPU_INTR_FROM_CPU_0** `[0:0]`
    ///
    /// set 1 will triger a interrupt
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_0(&self) -> bool {
        (self.bits & CpuIntrFromCpu0::CPU_INTR_FROM_CPU_0_MASK) != 0
    }

    /// Set **CPU_INTR_FROM_CPU_0** `[0:0]`
    ///
    /// set 1 will triger a interrupt
    #[inline(always)]
    pub const fn with_cpu_intr_from_cpu_0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CpuIntrFromCpu0::CPU_INTR_FROM_CPU_0_MASK
            } else {
                self.bits & !CpuIntrFromCpu0::CPU_INTR_FROM_CPU_0_MASK
            },
        }
    }
}

impl Default for CpuIntrFromCpu0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CpuIntrFromCpu0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CpuIntrFromCpu0Val {
        CpuIntrFromCpu0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CpuIntrFromCpu0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CpuIntrFromCpu0Val) -> CpuIntrFromCpu0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CPU_INTR_FROM_CPU_1 Register
///
/// NA
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **CPU_INTR_FROM_CPU_1** `[0]` (RW)
///
///   set 1 will triger a interrupt
#[repr(transparent)]
pub struct CpuIntrFromCpu1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CpuIntrFromCpu1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CpuIntrFromCpu1 {
    // ======== Field Constants ========

    /// **CPU_INTR_FROM_CPU_1**
    ///
    /// set 1 will triger a interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CPU_INTR_FROM_CPU_1_OFFSET: u32 = 0;
    pub const CPU_INTR_FROM_CPU_1_MASK: u32 = 0x00000001;
    pub const CPU_INTR_FROM_CPU_1: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CPU_INTR_FROM_CPU_1** `[0]` (RW)
    ///
    /// set 1 will triger a interrupt
    #[inline(always)]
    pub fn cpu_intr_from_cpu_1(&self) -> bool {
        (self.value.read_bits() & Self::CPU_INTR_FROM_CPU_1_MASK) != 0
    }

    /// Set CPU_INTR_FROM_CPU_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cpu_intr_from_cpu_1(&self) {
        self.value
            .modify_bits(|r| r | Self::CPU_INTR_FROM_CPU_1_MASK);
    }

    /// Clear CPU_INTR_FROM_CPU_1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cpu_intr_from_cpu_1(&self) {
        self.value
            .modify_bits(|r| r & !Self::CPU_INTR_FROM_CPU_1_MASK);
    }
}

/// CPU_INTR_FROM_CPU_1 value type for batch read/modify/write operations
///
/// Use `cpuintrfromcpu1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cpuintrfromcpu1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CpuIntrFromCpu1Val {
    bits: u32,
}

impl CpuIntrFromCpu1Val {
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

    /// Read **CPU_INTR_FROM_CPU_1** `[0:0]`
    ///
    /// set 1 will triger a interrupt
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_1(&self) -> bool {
        (self.bits & CpuIntrFromCpu1::CPU_INTR_FROM_CPU_1_MASK) != 0
    }

    /// Set **CPU_INTR_FROM_CPU_1** `[0:0]`
    ///
    /// set 1 will triger a interrupt
    #[inline(always)]
    pub const fn with_cpu_intr_from_cpu_1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CpuIntrFromCpu1::CPU_INTR_FROM_CPU_1_MASK
            } else {
                self.bits & !CpuIntrFromCpu1::CPU_INTR_FROM_CPU_1_MASK
            },
        }
    }
}

impl Default for CpuIntrFromCpu1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CpuIntrFromCpu1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CpuIntrFromCpu1Val {
        CpuIntrFromCpu1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CpuIntrFromCpu1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CpuIntrFromCpu1Val) -> CpuIntrFromCpu1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CPU_INTR_FROM_CPU_2 Register
///
/// NA
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **CPU_INTR_FROM_CPU_2** `[0]` (RW)
///
///   set 1 will triger a interrupt
#[repr(transparent)]
pub struct CpuIntrFromCpu2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CpuIntrFromCpu2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CpuIntrFromCpu2 {
    // ======== Field Constants ========

    /// **CPU_INTR_FROM_CPU_2**
    ///
    /// set 1 will triger a interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CPU_INTR_FROM_CPU_2_OFFSET: u32 = 0;
    pub const CPU_INTR_FROM_CPU_2_MASK: u32 = 0x00000001;
    pub const CPU_INTR_FROM_CPU_2: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CPU_INTR_FROM_CPU_2** `[0]` (RW)
    ///
    /// set 1 will triger a interrupt
    #[inline(always)]
    pub fn cpu_intr_from_cpu_2(&self) -> bool {
        (self.value.read_bits() & Self::CPU_INTR_FROM_CPU_2_MASK) != 0
    }

    /// Set CPU_INTR_FROM_CPU_2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cpu_intr_from_cpu_2(&self) {
        self.value
            .modify_bits(|r| r | Self::CPU_INTR_FROM_CPU_2_MASK);
    }

    /// Clear CPU_INTR_FROM_CPU_2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cpu_intr_from_cpu_2(&self) {
        self.value
            .modify_bits(|r| r & !Self::CPU_INTR_FROM_CPU_2_MASK);
    }
}

/// CPU_INTR_FROM_CPU_2 value type for batch read/modify/write operations
///
/// Use `cpuintrfromcpu2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cpuintrfromcpu2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CpuIntrFromCpu2Val {
    bits: u32,
}

impl CpuIntrFromCpu2Val {
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

    /// Read **CPU_INTR_FROM_CPU_2** `[0:0]`
    ///
    /// set 1 will triger a interrupt
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_2(&self) -> bool {
        (self.bits & CpuIntrFromCpu2::CPU_INTR_FROM_CPU_2_MASK) != 0
    }

    /// Set **CPU_INTR_FROM_CPU_2** `[0:0]`
    ///
    /// set 1 will triger a interrupt
    #[inline(always)]
    pub const fn with_cpu_intr_from_cpu_2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CpuIntrFromCpu2::CPU_INTR_FROM_CPU_2_MASK
            } else {
                self.bits & !CpuIntrFromCpu2::CPU_INTR_FROM_CPU_2_MASK
            },
        }
    }
}

impl Default for CpuIntrFromCpu2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CpuIntrFromCpu2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CpuIntrFromCpu2Val {
        CpuIntrFromCpu2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CpuIntrFromCpu2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CpuIntrFromCpu2Val) -> CpuIntrFromCpu2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CPU_INTR_FROM_CPU_3 Register
///
/// NA
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **CPU_INTR_FROM_CPU_3** `[0]` (RW)
///
///   set 1 will triger a interrupt
#[repr(transparent)]
pub struct CpuIntrFromCpu3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CpuIntrFromCpu3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CpuIntrFromCpu3 {
    // ======== Field Constants ========

    /// **CPU_INTR_FROM_CPU_3**
    ///
    /// set 1 will triger a interrupt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CPU_INTR_FROM_CPU_3_OFFSET: u32 = 0;
    pub const CPU_INTR_FROM_CPU_3_MASK: u32 = 0x00000001;
    pub const CPU_INTR_FROM_CPU_3: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CPU_INTR_FROM_CPU_3** `[0]` (RW)
    ///
    /// set 1 will triger a interrupt
    #[inline(always)]
    pub fn cpu_intr_from_cpu_3(&self) -> bool {
        (self.value.read_bits() & Self::CPU_INTR_FROM_CPU_3_MASK) != 0
    }

    /// Set CPU_INTR_FROM_CPU_3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cpu_intr_from_cpu_3(&self) {
        self.value
            .modify_bits(|r| r | Self::CPU_INTR_FROM_CPU_3_MASK);
    }

    /// Clear CPU_INTR_FROM_CPU_3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cpu_intr_from_cpu_3(&self) {
        self.value
            .modify_bits(|r| r & !Self::CPU_INTR_FROM_CPU_3_MASK);
    }
}

/// CPU_INTR_FROM_CPU_3 value type for batch read/modify/write operations
///
/// Use `cpuintrfromcpu3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cpuintrfromcpu3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CpuIntrFromCpu3Val {
    bits: u32,
}

impl CpuIntrFromCpu3Val {
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

    /// Read **CPU_INTR_FROM_CPU_3** `[0:0]`
    ///
    /// set 1 will triger a interrupt
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_3(&self) -> bool {
        (self.bits & CpuIntrFromCpu3::CPU_INTR_FROM_CPU_3_MASK) != 0
    }

    /// Set **CPU_INTR_FROM_CPU_3** `[0:0]`
    ///
    /// set 1 will triger a interrupt
    #[inline(always)]
    pub const fn with_cpu_intr_from_cpu_3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CpuIntrFromCpu3::CPU_INTR_FROM_CPU_3_MASK
            } else {
                self.bits & !CpuIntrFromCpu3::CPU_INTR_FROM_CPU_3_MASK
            },
        }
    }
}

impl Default for CpuIntrFromCpu3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CpuIntrFromCpu3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CpuIntrFromCpu3Val {
        CpuIntrFromCpu3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CpuIntrFromCpu3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CpuIntrFromCpu3Val) -> CpuIntrFromCpu3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CACHE_CLK_CONFIG Register
///
/// NA
///
/// Address offset: 0x0020
/// Reset value: 0x00000033
///
/// ## Fields
///
/// - **REG_L1_I0_CACHE_CLK_ON** `[5]` (RW)
///
///   l1 icahce0 clk enable
///
/// - **REG_L1_I1_CACHE_CLK_ON** `[4]` (RW)
///
///   l1 icahce1 clk enable
///
/// - **REG_L1_D_CACHE_CLK_ON** `[1]` (RW)
///
///   l1 dcahce clk enable
///
/// - **REG_L2_CACHE_CLK_ON** `[0]` (RW)
///
///   l2 cahce clk enable
#[repr(transparent)]
pub struct CacheClkConfig {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CacheClkConfig {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CacheClkConfig {
    // ======== Field Constants ========

    /// **REG_L1_I0_CACHE_CLK_ON**
    ///
    /// l1 icahce0 clk enable
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_L1_I0_CACHE_CLK_ON_OFFSET: u32 = 5;
    pub const REG_L1_I0_CACHE_CLK_ON_MASK: u32 = 0x00000020;
    pub const REG_L1_I0_CACHE_CLK_ON: u32 = 1 << 5;

    /// **REG_L1_I1_CACHE_CLK_ON**
    ///
    /// l1 icahce1 clk enable
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_L1_I1_CACHE_CLK_ON_OFFSET: u32 = 4;
    pub const REG_L1_I1_CACHE_CLK_ON_MASK: u32 = 0x00000010;
    pub const REG_L1_I1_CACHE_CLK_ON: u32 = 1 << 4;

    /// **REG_L1_D_CACHE_CLK_ON**
    ///
    /// l1 dcahce clk enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_L1_D_CACHE_CLK_ON_OFFSET: u32 = 1;
    pub const REG_L1_D_CACHE_CLK_ON_MASK: u32 = 0x00000002;
    pub const REG_L1_D_CACHE_CLK_ON: u32 = 1 << 1;

    /// **REG_L2_CACHE_CLK_ON**
    ///
    /// l2 cahce clk enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_L2_CACHE_CLK_ON_OFFSET: u32 = 0;
    pub const REG_L2_CACHE_CLK_ON_MASK: u32 = 0x00000001;
    pub const REG_L2_CACHE_CLK_ON: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_L1_I0_CACHE_CLK_ON** `[5]` (RW)
    ///
    /// l1 icahce0 clk enable
    #[inline(always)]
    pub fn reg_l1_i0_cache_clk_on(&self) -> bool {
        (self.value.read_bits() & Self::REG_L1_I0_CACHE_CLK_ON_MASK) != 0
    }

    /// Set REG_L1_I0_CACHE_CLK_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l1_i0_cache_clk_on(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L1_I0_CACHE_CLK_ON_MASK);
    }

    /// Clear REG_L1_I0_CACHE_CLK_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l1_i0_cache_clk_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L1_I0_CACHE_CLK_ON_MASK);
    }

    /// **REG_L1_I1_CACHE_CLK_ON** `[4]` (RW)
    ///
    /// l1 icahce1 clk enable
    #[inline(always)]
    pub fn reg_l1_i1_cache_clk_on(&self) -> bool {
        (self.value.read_bits() & Self::REG_L1_I1_CACHE_CLK_ON_MASK) != 0
    }

    /// Set REG_L1_I1_CACHE_CLK_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l1_i1_cache_clk_on(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L1_I1_CACHE_CLK_ON_MASK);
    }

    /// Clear REG_L1_I1_CACHE_CLK_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l1_i1_cache_clk_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L1_I1_CACHE_CLK_ON_MASK);
    }

    /// **REG_L1_D_CACHE_CLK_ON** `[1]` (RW)
    ///
    /// l1 dcahce clk enable
    #[inline(always)]
    pub fn reg_l1_d_cache_clk_on(&self) -> bool {
        (self.value.read_bits() & Self::REG_L1_D_CACHE_CLK_ON_MASK) != 0
    }

    /// Set REG_L1_D_CACHE_CLK_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l1_d_cache_clk_on(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L1_D_CACHE_CLK_ON_MASK);
    }

    /// Clear REG_L1_D_CACHE_CLK_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l1_d_cache_clk_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L1_D_CACHE_CLK_ON_MASK);
    }

    /// **REG_L2_CACHE_CLK_ON** `[0]` (RW)
    ///
    /// l2 cahce clk enable
    #[inline(always)]
    pub fn reg_l2_cache_clk_on(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_CACHE_CLK_ON_MASK) != 0
    }

    /// Set REG_L2_CACHE_CLK_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_cache_clk_on(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_CACHE_CLK_ON_MASK);
    }

    /// Clear REG_L2_CACHE_CLK_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_cache_clk_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_CACHE_CLK_ON_MASK);
    }
}

/// CACHE_CLK_CONFIG value type for batch read/modify/write operations
///
/// Use `cacheclkconfig.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cacheclkconfig.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CacheClkConfigVal {
    bits: u32,
}

impl CacheClkConfigVal {
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
        Self { bits: 0x00000033 }
    }

    /// Read **REG_L1_I0_CACHE_CLK_ON** `[5:5]`
    ///
    /// l1 icahce0 clk enable
    #[inline(always)]
    pub const fn reg_l1_i0_cache_clk_on(&self) -> bool {
        (self.bits & CacheClkConfig::REG_L1_I0_CACHE_CLK_ON_MASK) != 0
    }

    /// Read **REG_L1_I1_CACHE_CLK_ON** `[4:4]`
    ///
    /// l1 icahce1 clk enable
    #[inline(always)]
    pub const fn reg_l1_i1_cache_clk_on(&self) -> bool {
        (self.bits & CacheClkConfig::REG_L1_I1_CACHE_CLK_ON_MASK) != 0
    }

    /// Read **REG_L1_D_CACHE_CLK_ON** `[1:1]`
    ///
    /// l1 dcahce clk enable
    #[inline(always)]
    pub const fn reg_l1_d_cache_clk_on(&self) -> bool {
        (self.bits & CacheClkConfig::REG_L1_D_CACHE_CLK_ON_MASK) != 0
    }

    /// Read **REG_L2_CACHE_CLK_ON** `[0:0]`
    ///
    /// l2 cahce clk enable
    #[inline(always)]
    pub const fn reg_l2_cache_clk_on(&self) -> bool {
        (self.bits & CacheClkConfig::REG_L2_CACHE_CLK_ON_MASK) != 0
    }

    /// Set **REG_L1_I0_CACHE_CLK_ON** `[5:5]`
    ///
    /// l1 icahce0 clk enable
    #[inline(always)]
    pub const fn with_reg_l1_i0_cache_clk_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CacheClkConfig::REG_L1_I0_CACHE_CLK_ON_MASK
            } else {
                self.bits & !CacheClkConfig::REG_L1_I0_CACHE_CLK_ON_MASK
            },
        }
    }

    /// Set **REG_L1_I1_CACHE_CLK_ON** `[4:4]`
    ///
    /// l1 icahce1 clk enable
    #[inline(always)]
    pub const fn with_reg_l1_i1_cache_clk_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CacheClkConfig::REG_L1_I1_CACHE_CLK_ON_MASK
            } else {
                self.bits & !CacheClkConfig::REG_L1_I1_CACHE_CLK_ON_MASK
            },
        }
    }

    /// Set **REG_L1_D_CACHE_CLK_ON** `[1:1]`
    ///
    /// l1 dcahce clk enable
    #[inline(always)]
    pub const fn with_reg_l1_d_cache_clk_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CacheClkConfig::REG_L1_D_CACHE_CLK_ON_MASK
            } else {
                self.bits & !CacheClkConfig::REG_L1_D_CACHE_CLK_ON_MASK
            },
        }
    }

    /// Set **REG_L2_CACHE_CLK_ON** `[0:0]`
    ///
    /// l2 cahce clk enable
    #[inline(always)]
    pub const fn with_reg_l2_cache_clk_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CacheClkConfig::REG_L2_CACHE_CLK_ON_MASK
            } else {
                self.bits & !CacheClkConfig::REG_L2_CACHE_CLK_ON_MASK
            },
        }
    }
}

impl Default for CacheClkConfigVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CacheClkConfig {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CacheClkConfigVal {
        CacheClkConfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CacheClkConfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CacheClkConfigVal) -> CacheClkConfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CACHE_RESET_CONFIG Register
///
/// NA
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **REG_L1_I0_CACHE_RESET** `[5]` (RW)
///
///   set 1 to reset l1 icahce0
///
/// - **REG_L1_I1_CACHE_RESET** `[4]` (RW)
///
///   set 1 to reset l1 icahce1
///
/// - **REG_L1_D_CACHE_RESET** `[1]` (RW)
///
///   set 1 to reset l1 dcahce
#[repr(transparent)]
pub struct CacheResetConfig {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CacheResetConfig {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CacheResetConfig {
    // ======== Field Constants ========

    /// **REG_L1_I0_CACHE_RESET**
    ///
    /// set 1 to reset l1 icahce0
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_L1_I0_CACHE_RESET_OFFSET: u32 = 5;
    pub const REG_L1_I0_CACHE_RESET_MASK: u32 = 0x00000020;
    pub const REG_L1_I0_CACHE_RESET: u32 = 1 << 5;

    /// **REG_L1_I1_CACHE_RESET**
    ///
    /// set 1 to reset l1 icahce1
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_L1_I1_CACHE_RESET_OFFSET: u32 = 4;
    pub const REG_L1_I1_CACHE_RESET_MASK: u32 = 0x00000010;
    pub const REG_L1_I1_CACHE_RESET: u32 = 1 << 4;

    /// **REG_L1_D_CACHE_RESET**
    ///
    /// set 1 to reset l1 dcahce
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_L1_D_CACHE_RESET_OFFSET: u32 = 1;
    pub const REG_L1_D_CACHE_RESET_MASK: u32 = 0x00000002;
    pub const REG_L1_D_CACHE_RESET: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **REG_L1_I0_CACHE_RESET** `[5]` (RW)
    ///
    /// set 1 to reset l1 icahce0
    #[inline(always)]
    pub fn reg_l1_i0_cache_reset(&self) -> bool {
        (self.value.read_bits() & Self::REG_L1_I0_CACHE_RESET_MASK) != 0
    }

    /// Set REG_L1_I0_CACHE_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l1_i0_cache_reset(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L1_I0_CACHE_RESET_MASK);
    }

    /// Clear REG_L1_I0_CACHE_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l1_i0_cache_reset(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L1_I0_CACHE_RESET_MASK);
    }

    /// **REG_L1_I1_CACHE_RESET** `[4]` (RW)
    ///
    /// set 1 to reset l1 icahce1
    #[inline(always)]
    pub fn reg_l1_i1_cache_reset(&self) -> bool {
        (self.value.read_bits() & Self::REG_L1_I1_CACHE_RESET_MASK) != 0
    }

    /// Set REG_L1_I1_CACHE_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l1_i1_cache_reset(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L1_I1_CACHE_RESET_MASK);
    }

    /// Clear REG_L1_I1_CACHE_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l1_i1_cache_reset(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L1_I1_CACHE_RESET_MASK);
    }

    /// **REG_L1_D_CACHE_RESET** `[1]` (RW)
    ///
    /// set 1 to reset l1 dcahce
    #[inline(always)]
    pub fn reg_l1_d_cache_reset(&self) -> bool {
        (self.value.read_bits() & Self::REG_L1_D_CACHE_RESET_MASK) != 0
    }

    /// Set REG_L1_D_CACHE_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l1_d_cache_reset(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L1_D_CACHE_RESET_MASK);
    }

    /// Clear REG_L1_D_CACHE_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l1_d_cache_reset(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L1_D_CACHE_RESET_MASK);
    }
}

/// CACHE_RESET_CONFIG value type for batch read/modify/write operations
///
/// Use `cacheresetconfig.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cacheresetconfig.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CacheResetConfigVal {
    bits: u32,
}

impl CacheResetConfigVal {
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

    /// Read **REG_L1_I0_CACHE_RESET** `[5:5]`
    ///
    /// set 1 to reset l1 icahce0
    #[inline(always)]
    pub const fn reg_l1_i0_cache_reset(&self) -> bool {
        (self.bits & CacheResetConfig::REG_L1_I0_CACHE_RESET_MASK) != 0
    }

    /// Read **REG_L1_I1_CACHE_RESET** `[4:4]`
    ///
    /// set 1 to reset l1 icahce1
    #[inline(always)]
    pub const fn reg_l1_i1_cache_reset(&self) -> bool {
        (self.bits & CacheResetConfig::REG_L1_I1_CACHE_RESET_MASK) != 0
    }

    /// Read **REG_L1_D_CACHE_RESET** `[1:1]`
    ///
    /// set 1 to reset l1 dcahce
    #[inline(always)]
    pub const fn reg_l1_d_cache_reset(&self) -> bool {
        (self.bits & CacheResetConfig::REG_L1_D_CACHE_RESET_MASK) != 0
    }

    /// Set **REG_L1_I0_CACHE_RESET** `[5:5]`
    ///
    /// set 1 to reset l1 icahce0
    #[inline(always)]
    pub const fn with_reg_l1_i0_cache_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CacheResetConfig::REG_L1_I0_CACHE_RESET_MASK
            } else {
                self.bits & !CacheResetConfig::REG_L1_I0_CACHE_RESET_MASK
            },
        }
    }

    /// Set **REG_L1_I1_CACHE_RESET** `[4:4]`
    ///
    /// set 1 to reset l1 icahce1
    #[inline(always)]
    pub const fn with_reg_l1_i1_cache_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CacheResetConfig::REG_L1_I1_CACHE_RESET_MASK
            } else {
                self.bits & !CacheResetConfig::REG_L1_I1_CACHE_RESET_MASK
            },
        }
    }

    /// Set **REG_L1_D_CACHE_RESET** `[1:1]`
    ///
    /// set 1 to reset l1 dcahce
    #[inline(always)]
    pub const fn with_reg_l1_d_cache_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CacheResetConfig::REG_L1_D_CACHE_RESET_MASK
            } else {
                self.bits & !CacheResetConfig::REG_L1_D_CACHE_RESET_MASK
            },
        }
    }
}

impl Default for CacheResetConfigVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CacheResetConfig {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CacheResetConfigVal {
        CacheResetConfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CacheResetConfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CacheResetConfigVal) -> CacheResetConfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMA_ADDR_CTRL Register
///
/// NA
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **REG_SYS_DMA_ADDR_SEL** `[0]` (RW)
///
///   0 means dma access extmem use 8xxx_xxxx else use 4xxx_xxxx
#[repr(transparent)]
pub struct DmaAddrCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmaAddrCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmaAddrCtrl {
    // ======== Field Constants ========

    /// **REG_SYS_DMA_ADDR_SEL**
    ///
    /// 0 means dma access extmem use 8xxx_xxxx else use 4xxx_xxxx
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_SYS_DMA_ADDR_SEL_OFFSET: u32 = 0;
    pub const REG_SYS_DMA_ADDR_SEL_MASK: u32 = 0x00000001;
    pub const REG_SYS_DMA_ADDR_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_SYS_DMA_ADDR_SEL** `[0]` (RW)
    ///
    /// 0 means dma access extmem use 8xxx_xxxx else use 4xxx_xxxx
    #[inline(always)]
    pub fn reg_sys_dma_addr_sel(&self) -> bool {
        (self.value.read_bits() & Self::REG_SYS_DMA_ADDR_SEL_MASK) != 0
    }

    /// Set REG_SYS_DMA_ADDR_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_sys_dma_addr_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_SYS_DMA_ADDR_SEL_MASK);
    }

    /// Clear REG_SYS_DMA_ADDR_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_sys_dma_addr_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_SYS_DMA_ADDR_SEL_MASK);
    }
}

/// DMA_ADDR_CTRL value type for batch read/modify/write operations
///
/// Use `dmaaddrctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmaaddrctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaAddrCtrlVal {
    bits: u32,
}

impl DmaAddrCtrlVal {
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

    /// Read **REG_SYS_DMA_ADDR_SEL** `[0:0]`
    ///
    /// 0 means dma access extmem use 8xxx_xxxx else use 4xxx_xxxx
    #[inline(always)]
    pub const fn reg_sys_dma_addr_sel(&self) -> bool {
        (self.bits & DmaAddrCtrl::REG_SYS_DMA_ADDR_SEL_MASK) != 0
    }

    /// Set **REG_SYS_DMA_ADDR_SEL** `[0:0]`
    ///
    /// 0 means dma access extmem use 8xxx_xxxx else use 4xxx_xxxx
    #[inline(always)]
    pub const fn with_reg_sys_dma_addr_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaAddrCtrl::REG_SYS_DMA_ADDR_SEL_MASK
            } else {
                self.bits & !DmaAddrCtrl::REG_SYS_DMA_ADDR_SEL_MASK
            },
        }
    }
}

impl Default for DmaAddrCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DmaAddrCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaAddrCtrlVal {
        DmaAddrCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaAddrCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaAddrCtrlVal) -> DmaAddrCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TCM_RAM_WRR_CONFIG Register
///
/// NA
///
/// Address offset: 0x0034
/// Reset value: 0x826ed93f
///
/// ## Fields
///
/// - **REG_TCM_RAM_WRR_HIGH** `[31]` (RW)
///
///   enable weighted round robin arbitration
///
/// - **REG_TCM_RAM_DMA_WT** `[26:24]` (RW)
///
///   weight value of dma
///
/// - **REG_TCM_RAM_DBUS3_WT** `[23:21]` (RW)
///
///   weight value of dbus3
///
/// - **REG_TCM_RAM_DBUS2_WT** `[20:18]` (RW)
///
///   weight value of dbus2
///
/// - **REG_TCM_RAM_DBUS1_WT** `[17:15]` (RW)
///
///   weight value of dbus1
///
/// - **REG_TCM_RAM_DBUS0_WT** `[14:12]` (RW)
///
///   weight value of dbus0
///
/// - **REG_TCM_RAM_IBUS3_WT** `[11:9]` (RW)
///
///   weight value of ibus3
///
/// - **REG_TCM_RAM_IBUS2_WT** `[8:6]` (RW)
///
///   weight value of ibus2
///
/// - **REG_TCM_RAM_IBUS1_WT** `[5:3]` (RW)
///
///   weight value of ibus1
///
/// - **REG_TCM_RAM_IBUS0_WT** `[2:0]` (RW)
///
///   weight value of ibus0
#[repr(transparent)]
pub struct TcmRamWrrConfig {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TcmRamWrrConfig {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TcmRamWrrConfig {
    // ======== Field Constants ========

    /// **REG_TCM_RAM_WRR_HIGH**
    ///
    /// enable weighted round robin arbitration
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const REG_TCM_RAM_WRR_HIGH_OFFSET: u32 = 31;
    pub const REG_TCM_RAM_WRR_HIGH_MASK: u32 = 0x80000000;
    pub const REG_TCM_RAM_WRR_HIGH: u32 = 1 << 31;

    /// **REG_TCM_RAM_DMA_WT**
    ///
    /// weight value of dma
    ///
    /// Bits `[26:24]`, mask `0x07000000`
    pub const REG_TCM_RAM_DMA_WT_OFFSET: u32 = 24;
    pub const REG_TCM_RAM_DMA_WT_MASK: u32 = 0x07000000;

    /// **REG_TCM_RAM_DBUS3_WT**
    ///
    /// weight value of dbus3
    ///
    /// Bits `[23:21]`, mask `0x00e00000`
    pub const REG_TCM_RAM_DBUS3_WT_OFFSET: u32 = 21;
    pub const REG_TCM_RAM_DBUS3_WT_MASK: u32 = 0x00e00000;

    /// **REG_TCM_RAM_DBUS2_WT**
    ///
    /// weight value of dbus2
    ///
    /// Bits `[20:18]`, mask `0x001c0000`
    pub const REG_TCM_RAM_DBUS2_WT_OFFSET: u32 = 18;
    pub const REG_TCM_RAM_DBUS2_WT_MASK: u32 = 0x001c0000;

    /// **REG_TCM_RAM_DBUS1_WT**
    ///
    /// weight value of dbus1
    ///
    /// Bits `[17:15]`, mask `0x00038000`
    pub const REG_TCM_RAM_DBUS1_WT_OFFSET: u32 = 15;
    pub const REG_TCM_RAM_DBUS1_WT_MASK: u32 = 0x00038000;

    /// **REG_TCM_RAM_DBUS0_WT**
    ///
    /// weight value of dbus0
    ///
    /// Bits `[14:12]`, mask `0x00007000`
    pub const REG_TCM_RAM_DBUS0_WT_OFFSET: u32 = 12;
    pub const REG_TCM_RAM_DBUS0_WT_MASK: u32 = 0x00007000;

    /// **REG_TCM_RAM_IBUS3_WT**
    ///
    /// weight value of ibus3
    ///
    /// Bits `[11:9]`, mask `0x00000e00`
    pub const REG_TCM_RAM_IBUS3_WT_OFFSET: u32 = 9;
    pub const REG_TCM_RAM_IBUS3_WT_MASK: u32 = 0x00000e00;

    /// **REG_TCM_RAM_IBUS2_WT**
    ///
    /// weight value of ibus2
    ///
    /// Bits `[8:6]`, mask `0x000001c0`
    pub const REG_TCM_RAM_IBUS2_WT_OFFSET: u32 = 6;
    pub const REG_TCM_RAM_IBUS2_WT_MASK: u32 = 0x000001c0;

    /// **REG_TCM_RAM_IBUS1_WT**
    ///
    /// weight value of ibus1
    ///
    /// Bits `[5:3]`, mask `0x00000038`
    pub const REG_TCM_RAM_IBUS1_WT_OFFSET: u32 = 3;
    pub const REG_TCM_RAM_IBUS1_WT_MASK: u32 = 0x00000038;

    /// **REG_TCM_RAM_IBUS0_WT**
    ///
    /// weight value of ibus0
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const REG_TCM_RAM_IBUS0_WT_OFFSET: u32 = 0;
    pub const REG_TCM_RAM_IBUS0_WT_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **REG_TCM_RAM_WRR_HIGH** `[31]` (RW)
    ///
    /// enable weighted round robin arbitration
    #[inline(always)]
    pub fn reg_tcm_ram_wrr_high(&self) -> bool {
        (self.value.read_bits() & Self::REG_TCM_RAM_WRR_HIGH_MASK) != 0
    }

    /// Set REG_TCM_RAM_WRR_HIGH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_tcm_ram_wrr_high(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_TCM_RAM_WRR_HIGH_MASK);
    }

    /// Clear REG_TCM_RAM_WRR_HIGH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_tcm_ram_wrr_high(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_TCM_RAM_WRR_HIGH_MASK);
    }

    /// **REG_TCM_RAM_DMA_WT** `[26:24]` (RW)
    ///
    /// weight value of dma
    #[inline(always)]
    pub fn reg_tcm_ram_dma_wt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_TCM_RAM_DMA_WT_MASK) >> Self::REG_TCM_RAM_DMA_WT_OFFSET
    }

    /// Write **REG_TCM_RAM_DMA_WT** `[26:24]` (RW)
    ///
    /// weight value of dma
    #[inline(always)]
    pub fn write_reg_tcm_ram_dma_wt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_TCM_RAM_DMA_WT_MASK)
                | ((value << Self::REG_TCM_RAM_DMA_WT_OFFSET) & Self::REG_TCM_RAM_DMA_WT_MASK)
        });
    }

    /// **REG_TCM_RAM_DBUS3_WT** `[23:21]` (RW)
    ///
    /// weight value of dbus3
    #[inline(always)]
    pub fn reg_tcm_ram_dbus3_wt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_TCM_RAM_DBUS3_WT_MASK)
            >> Self::REG_TCM_RAM_DBUS3_WT_OFFSET
    }

    /// Write **REG_TCM_RAM_DBUS3_WT** `[23:21]` (RW)
    ///
    /// weight value of dbus3
    #[inline(always)]
    pub fn write_reg_tcm_ram_dbus3_wt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_TCM_RAM_DBUS3_WT_MASK)
                | ((value << Self::REG_TCM_RAM_DBUS3_WT_OFFSET) & Self::REG_TCM_RAM_DBUS3_WT_MASK)
        });
    }

    /// **REG_TCM_RAM_DBUS2_WT** `[20:18]` (RW)
    ///
    /// weight value of dbus2
    #[inline(always)]
    pub fn reg_tcm_ram_dbus2_wt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_TCM_RAM_DBUS2_WT_MASK)
            >> Self::REG_TCM_RAM_DBUS2_WT_OFFSET
    }

    /// Write **REG_TCM_RAM_DBUS2_WT** `[20:18]` (RW)
    ///
    /// weight value of dbus2
    #[inline(always)]
    pub fn write_reg_tcm_ram_dbus2_wt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_TCM_RAM_DBUS2_WT_MASK)
                | ((value << Self::REG_TCM_RAM_DBUS2_WT_OFFSET) & Self::REG_TCM_RAM_DBUS2_WT_MASK)
        });
    }

    /// **REG_TCM_RAM_DBUS1_WT** `[17:15]` (RW)
    ///
    /// weight value of dbus1
    #[inline(always)]
    pub fn reg_tcm_ram_dbus1_wt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_TCM_RAM_DBUS1_WT_MASK)
            >> Self::REG_TCM_RAM_DBUS1_WT_OFFSET
    }

    /// Write **REG_TCM_RAM_DBUS1_WT** `[17:15]` (RW)
    ///
    /// weight value of dbus1
    #[inline(always)]
    pub fn write_reg_tcm_ram_dbus1_wt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_TCM_RAM_DBUS1_WT_MASK)
                | ((value << Self::REG_TCM_RAM_DBUS1_WT_OFFSET) & Self::REG_TCM_RAM_DBUS1_WT_MASK)
        });
    }

    /// **REG_TCM_RAM_DBUS0_WT** `[14:12]` (RW)
    ///
    /// weight value of dbus0
    #[inline(always)]
    pub fn reg_tcm_ram_dbus0_wt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_TCM_RAM_DBUS0_WT_MASK)
            >> Self::REG_TCM_RAM_DBUS0_WT_OFFSET
    }

    /// Write **REG_TCM_RAM_DBUS0_WT** `[14:12]` (RW)
    ///
    /// weight value of dbus0
    #[inline(always)]
    pub fn write_reg_tcm_ram_dbus0_wt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_TCM_RAM_DBUS0_WT_MASK)
                | ((value << Self::REG_TCM_RAM_DBUS0_WT_OFFSET) & Self::REG_TCM_RAM_DBUS0_WT_MASK)
        });
    }

    /// **REG_TCM_RAM_IBUS3_WT** `[11:9]` (RW)
    ///
    /// weight value of ibus3
    #[inline(always)]
    pub fn reg_tcm_ram_ibus3_wt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_TCM_RAM_IBUS3_WT_MASK)
            >> Self::REG_TCM_RAM_IBUS3_WT_OFFSET
    }

    /// Write **REG_TCM_RAM_IBUS3_WT** `[11:9]` (RW)
    ///
    /// weight value of ibus3
    #[inline(always)]
    pub fn write_reg_tcm_ram_ibus3_wt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_TCM_RAM_IBUS3_WT_MASK)
                | ((value << Self::REG_TCM_RAM_IBUS3_WT_OFFSET) & Self::REG_TCM_RAM_IBUS3_WT_MASK)
        });
    }

    /// **REG_TCM_RAM_IBUS2_WT** `[8:6]` (RW)
    ///
    /// weight value of ibus2
    #[inline(always)]
    pub fn reg_tcm_ram_ibus2_wt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_TCM_RAM_IBUS2_WT_MASK)
            >> Self::REG_TCM_RAM_IBUS2_WT_OFFSET
    }

    /// Write **REG_TCM_RAM_IBUS2_WT** `[8:6]` (RW)
    ///
    /// weight value of ibus2
    #[inline(always)]
    pub fn write_reg_tcm_ram_ibus2_wt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_TCM_RAM_IBUS2_WT_MASK)
                | ((value << Self::REG_TCM_RAM_IBUS2_WT_OFFSET) & Self::REG_TCM_RAM_IBUS2_WT_MASK)
        });
    }

    /// **REG_TCM_RAM_IBUS1_WT** `[5:3]` (RW)
    ///
    /// weight value of ibus1
    #[inline(always)]
    pub fn reg_tcm_ram_ibus1_wt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_TCM_RAM_IBUS1_WT_MASK)
            >> Self::REG_TCM_RAM_IBUS1_WT_OFFSET
    }

    /// Write **REG_TCM_RAM_IBUS1_WT** `[5:3]` (RW)
    ///
    /// weight value of ibus1
    #[inline(always)]
    pub fn write_reg_tcm_ram_ibus1_wt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_TCM_RAM_IBUS1_WT_MASK)
                | ((value << Self::REG_TCM_RAM_IBUS1_WT_OFFSET) & Self::REG_TCM_RAM_IBUS1_WT_MASK)
        });
    }

    /// **REG_TCM_RAM_IBUS0_WT** `[2:0]` (RW)
    ///
    /// weight value of ibus0
    #[inline(always)]
    pub fn reg_tcm_ram_ibus0_wt(&self) -> u32 {
        (self.value.read_bits() & Self::REG_TCM_RAM_IBUS0_WT_MASK)
            >> Self::REG_TCM_RAM_IBUS0_WT_OFFSET
    }

    /// Write **REG_TCM_RAM_IBUS0_WT** `[2:0]` (RW)
    ///
    /// weight value of ibus0
    #[inline(always)]
    pub fn write_reg_tcm_ram_ibus0_wt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_TCM_RAM_IBUS0_WT_MASK)
                | ((value << Self::REG_TCM_RAM_IBUS0_WT_OFFSET) & Self::REG_TCM_RAM_IBUS0_WT_MASK)
        });
    }
}

/// TCM_RAM_WRR_CONFIG value type for batch read/modify/write operations
///
/// Use `tcmramwrrconfig.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcmramwrrconfig.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcmRamWrrConfigVal {
    bits: u32,
}

impl TcmRamWrrConfigVal {
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
        Self { bits: 0x826ed93f }
    }

    /// Read **REG_TCM_RAM_WRR_HIGH** `[31:31]`
    ///
    /// enable weighted round robin arbitration
    #[inline(always)]
    pub const fn reg_tcm_ram_wrr_high(&self) -> bool {
        (self.bits & TcmRamWrrConfig::REG_TCM_RAM_WRR_HIGH_MASK) != 0
    }

    /// Read **REG_TCM_RAM_DMA_WT** `[26:24]`
    ///
    /// weight value of dma
    #[inline(always)]
    pub const fn reg_tcm_ram_dma_wt(&self) -> u32 {
        (self.bits & TcmRamWrrConfig::REG_TCM_RAM_DMA_WT_MASK)
            >> TcmRamWrrConfig::REG_TCM_RAM_DMA_WT_OFFSET
    }

    /// Read **REG_TCM_RAM_DBUS3_WT** `[23:21]`
    ///
    /// weight value of dbus3
    #[inline(always)]
    pub const fn reg_tcm_ram_dbus3_wt(&self) -> u32 {
        (self.bits & TcmRamWrrConfig::REG_TCM_RAM_DBUS3_WT_MASK)
            >> TcmRamWrrConfig::REG_TCM_RAM_DBUS3_WT_OFFSET
    }

    /// Read **REG_TCM_RAM_DBUS2_WT** `[20:18]`
    ///
    /// weight value of dbus2
    #[inline(always)]
    pub const fn reg_tcm_ram_dbus2_wt(&self) -> u32 {
        (self.bits & TcmRamWrrConfig::REG_TCM_RAM_DBUS2_WT_MASK)
            >> TcmRamWrrConfig::REG_TCM_RAM_DBUS2_WT_OFFSET
    }

    /// Read **REG_TCM_RAM_DBUS1_WT** `[17:15]`
    ///
    /// weight value of dbus1
    #[inline(always)]
    pub const fn reg_tcm_ram_dbus1_wt(&self) -> u32 {
        (self.bits & TcmRamWrrConfig::REG_TCM_RAM_DBUS1_WT_MASK)
            >> TcmRamWrrConfig::REG_TCM_RAM_DBUS1_WT_OFFSET
    }

    /// Read **REG_TCM_RAM_DBUS0_WT** `[14:12]`
    ///
    /// weight value of dbus0
    #[inline(always)]
    pub const fn reg_tcm_ram_dbus0_wt(&self) -> u32 {
        (self.bits & TcmRamWrrConfig::REG_TCM_RAM_DBUS0_WT_MASK)
            >> TcmRamWrrConfig::REG_TCM_RAM_DBUS0_WT_OFFSET
    }

    /// Read **REG_TCM_RAM_IBUS3_WT** `[11:9]`
    ///
    /// weight value of ibus3
    #[inline(always)]
    pub const fn reg_tcm_ram_ibus3_wt(&self) -> u32 {
        (self.bits & TcmRamWrrConfig::REG_TCM_RAM_IBUS3_WT_MASK)
            >> TcmRamWrrConfig::REG_TCM_RAM_IBUS3_WT_OFFSET
    }

    /// Read **REG_TCM_RAM_IBUS2_WT** `[8:6]`
    ///
    /// weight value of ibus2
    #[inline(always)]
    pub const fn reg_tcm_ram_ibus2_wt(&self) -> u32 {
        (self.bits & TcmRamWrrConfig::REG_TCM_RAM_IBUS2_WT_MASK)
            >> TcmRamWrrConfig::REG_TCM_RAM_IBUS2_WT_OFFSET
    }

    /// Read **REG_TCM_RAM_IBUS1_WT** `[5:3]`
    ///
    /// weight value of ibus1
    #[inline(always)]
    pub const fn reg_tcm_ram_ibus1_wt(&self) -> u32 {
        (self.bits & TcmRamWrrConfig::REG_TCM_RAM_IBUS1_WT_MASK)
            >> TcmRamWrrConfig::REG_TCM_RAM_IBUS1_WT_OFFSET
    }

    /// Read **REG_TCM_RAM_IBUS0_WT** `[2:0]`
    ///
    /// weight value of ibus0
    #[inline(always)]
    pub const fn reg_tcm_ram_ibus0_wt(&self) -> u32 {
        (self.bits & TcmRamWrrConfig::REG_TCM_RAM_IBUS0_WT_MASK)
            >> TcmRamWrrConfig::REG_TCM_RAM_IBUS0_WT_OFFSET
    }

    /// Set **REG_TCM_RAM_WRR_HIGH** `[31:31]`
    ///
    /// enable weighted round robin arbitration
    #[inline(always)]
    pub const fn with_reg_tcm_ram_wrr_high(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TcmRamWrrConfig::REG_TCM_RAM_WRR_HIGH_MASK
            } else {
                self.bits & !TcmRamWrrConfig::REG_TCM_RAM_WRR_HIGH_MASK
            },
        }
    }

    /// Set **REG_TCM_RAM_DMA_WT** `[26:24]`
    ///
    /// weight value of dma
    #[inline(always)]
    pub const fn with_reg_tcm_ram_dma_wt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TcmRamWrrConfig::REG_TCM_RAM_DMA_WT_MASK)
                | ((value << TcmRamWrrConfig::REG_TCM_RAM_DMA_WT_OFFSET)
                    & TcmRamWrrConfig::REG_TCM_RAM_DMA_WT_MASK),
        }
    }

    /// Set **REG_TCM_RAM_DBUS3_WT** `[23:21]`
    ///
    /// weight value of dbus3
    #[inline(always)]
    pub const fn with_reg_tcm_ram_dbus3_wt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TcmRamWrrConfig::REG_TCM_RAM_DBUS3_WT_MASK)
                | ((value << TcmRamWrrConfig::REG_TCM_RAM_DBUS3_WT_OFFSET)
                    & TcmRamWrrConfig::REG_TCM_RAM_DBUS3_WT_MASK),
        }
    }

    /// Set **REG_TCM_RAM_DBUS2_WT** `[20:18]`
    ///
    /// weight value of dbus2
    #[inline(always)]
    pub const fn with_reg_tcm_ram_dbus2_wt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TcmRamWrrConfig::REG_TCM_RAM_DBUS2_WT_MASK)
                | ((value << TcmRamWrrConfig::REG_TCM_RAM_DBUS2_WT_OFFSET)
                    & TcmRamWrrConfig::REG_TCM_RAM_DBUS2_WT_MASK),
        }
    }

    /// Set **REG_TCM_RAM_DBUS1_WT** `[17:15]`
    ///
    /// weight value of dbus1
    #[inline(always)]
    pub const fn with_reg_tcm_ram_dbus1_wt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TcmRamWrrConfig::REG_TCM_RAM_DBUS1_WT_MASK)
                | ((value << TcmRamWrrConfig::REG_TCM_RAM_DBUS1_WT_OFFSET)
                    & TcmRamWrrConfig::REG_TCM_RAM_DBUS1_WT_MASK),
        }
    }

    /// Set **REG_TCM_RAM_DBUS0_WT** `[14:12]`
    ///
    /// weight value of dbus0
    #[inline(always)]
    pub const fn with_reg_tcm_ram_dbus0_wt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TcmRamWrrConfig::REG_TCM_RAM_DBUS0_WT_MASK)
                | ((value << TcmRamWrrConfig::REG_TCM_RAM_DBUS0_WT_OFFSET)
                    & TcmRamWrrConfig::REG_TCM_RAM_DBUS0_WT_MASK),
        }
    }

    /// Set **REG_TCM_RAM_IBUS3_WT** `[11:9]`
    ///
    /// weight value of ibus3
    #[inline(always)]
    pub const fn with_reg_tcm_ram_ibus3_wt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TcmRamWrrConfig::REG_TCM_RAM_IBUS3_WT_MASK)
                | ((value << TcmRamWrrConfig::REG_TCM_RAM_IBUS3_WT_OFFSET)
                    & TcmRamWrrConfig::REG_TCM_RAM_IBUS3_WT_MASK),
        }
    }

    /// Set **REG_TCM_RAM_IBUS2_WT** `[8:6]`
    ///
    /// weight value of ibus2
    #[inline(always)]
    pub const fn with_reg_tcm_ram_ibus2_wt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TcmRamWrrConfig::REG_TCM_RAM_IBUS2_WT_MASK)
                | ((value << TcmRamWrrConfig::REG_TCM_RAM_IBUS2_WT_OFFSET)
                    & TcmRamWrrConfig::REG_TCM_RAM_IBUS2_WT_MASK),
        }
    }

    /// Set **REG_TCM_RAM_IBUS1_WT** `[5:3]`
    ///
    /// weight value of ibus1
    #[inline(always)]
    pub const fn with_reg_tcm_ram_ibus1_wt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TcmRamWrrConfig::REG_TCM_RAM_IBUS1_WT_MASK)
                | ((value << TcmRamWrrConfig::REG_TCM_RAM_IBUS1_WT_OFFSET)
                    & TcmRamWrrConfig::REG_TCM_RAM_IBUS1_WT_MASK),
        }
    }

    /// Set **REG_TCM_RAM_IBUS0_WT** `[2:0]`
    ///
    /// weight value of ibus0
    #[inline(always)]
    pub const fn with_reg_tcm_ram_ibus0_wt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TcmRamWrrConfig::REG_TCM_RAM_IBUS0_WT_MASK)
                | ((value << TcmRamWrrConfig::REG_TCM_RAM_IBUS0_WT_OFFSET)
                    & TcmRamWrrConfig::REG_TCM_RAM_IBUS0_WT_MASK),
        }
    }
}

impl Default for TcmRamWrrConfigVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TcmRamWrrConfig {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcmRamWrrConfigVal {
        TcmRamWrrConfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TcmRamWrrConfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TcmRamWrrConfigVal) -> TcmRamWrrConfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TCM_SW_PARITY_BWE_MASK Register
///
/// NA
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **REG_TCM_SW_PARITY_BWE_MASK_CTRL** `[0]` (RW)
///
///   Set 1 to mask tcm bwe parity code bit
#[repr(transparent)]
pub struct TcmSwParityBweMask {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TcmSwParityBweMask {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TcmSwParityBweMask {
    // ======== Field Constants ========

    /// **REG_TCM_SW_PARITY_BWE_MASK_CTRL**
    ///
    /// Set 1 to mask tcm bwe parity code bit
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_TCM_SW_PARITY_BWE_MASK_CTRL_OFFSET: u32 = 0;
    pub const REG_TCM_SW_PARITY_BWE_MASK_CTRL_MASK: u32 = 0x00000001;
    pub const REG_TCM_SW_PARITY_BWE_MASK_CTRL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_TCM_SW_PARITY_BWE_MASK_CTRL** `[0]` (RW)
    ///
    /// Set 1 to mask tcm bwe parity code bit
    #[inline(always)]
    pub fn reg_tcm_sw_parity_bwe_mask_ctrl(&self) -> bool {
        (self.value.read_bits() & Self::REG_TCM_SW_PARITY_BWE_MASK_CTRL_MASK) != 0
    }

    /// Set REG_TCM_SW_PARITY_BWE_MASK_CTRL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_tcm_sw_parity_bwe_mask_ctrl(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_TCM_SW_PARITY_BWE_MASK_CTRL_MASK);
    }

    /// Clear REG_TCM_SW_PARITY_BWE_MASK_CTRL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_tcm_sw_parity_bwe_mask_ctrl(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_TCM_SW_PARITY_BWE_MASK_CTRL_MASK);
    }
}

/// TCM_SW_PARITY_BWE_MASK value type for batch read/modify/write operations
///
/// Use `tcmswparitybwemask.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcmswparitybwemask.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcmSwParityBweMaskVal {
    bits: u32,
}

impl TcmSwParityBweMaskVal {
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

    /// Read **REG_TCM_SW_PARITY_BWE_MASK_CTRL** `[0:0]`
    ///
    /// Set 1 to mask tcm bwe parity code bit
    #[inline(always)]
    pub const fn reg_tcm_sw_parity_bwe_mask_ctrl(&self) -> bool {
        (self.bits & TcmSwParityBweMask::REG_TCM_SW_PARITY_BWE_MASK_CTRL_MASK) != 0
    }

    /// Set **REG_TCM_SW_PARITY_BWE_MASK_CTRL** `[0:0]`
    ///
    /// Set 1 to mask tcm bwe parity code bit
    #[inline(always)]
    pub const fn with_reg_tcm_sw_parity_bwe_mask_ctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TcmSwParityBweMask::REG_TCM_SW_PARITY_BWE_MASK_CTRL_MASK
            } else {
                self.bits & !TcmSwParityBweMask::REG_TCM_SW_PARITY_BWE_MASK_CTRL_MASK
            },
        }
    }
}

impl Default for TcmSwParityBweMaskVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TcmSwParityBweMask {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcmSwParityBweMaskVal {
        TcmSwParityBweMaskVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TcmSwParityBweMaskVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TcmSwParityBweMaskVal) -> TcmSwParityBweMaskVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TCM_RAM_PWR_CTRL0 Register
///
/// NA
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **REG_HP_TCM_CLK_FORCE_ON** `[0]` (RW)
///
///   hp_tcm clk gatig force on
#[repr(transparent)]
pub struct TcmRamPwrCtrl0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TcmRamPwrCtrl0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TcmRamPwrCtrl0 {
    // ======== Field Constants ========

    /// **REG_HP_TCM_CLK_FORCE_ON**
    ///
    /// hp_tcm clk gatig force on
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_HP_TCM_CLK_FORCE_ON_OFFSET: u32 = 0;
    pub const REG_HP_TCM_CLK_FORCE_ON_MASK: u32 = 0x00000001;
    pub const REG_HP_TCM_CLK_FORCE_ON: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_HP_TCM_CLK_FORCE_ON** `[0]` (RW)
    ///
    /// hp_tcm clk gatig force on
    #[inline(always)]
    pub fn reg_hp_tcm_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::REG_HP_TCM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set REG_HP_TCM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_hp_tcm_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_HP_TCM_CLK_FORCE_ON_MASK);
    }

    /// Clear REG_HP_TCM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_hp_tcm_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_HP_TCM_CLK_FORCE_ON_MASK);
    }
}

/// TCM_RAM_PWR_CTRL0 value type for batch read/modify/write operations
///
/// Use `tcmrampwrctrl0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcmrampwrctrl0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcmRamPwrCtrl0Val {
    bits: u32,
}

impl TcmRamPwrCtrl0Val {
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

    /// Read **REG_HP_TCM_CLK_FORCE_ON** `[0:0]`
    ///
    /// hp_tcm clk gatig force on
    #[inline(always)]
    pub const fn reg_hp_tcm_clk_force_on(&self) -> bool {
        (self.bits & TcmRamPwrCtrl0::REG_HP_TCM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set **REG_HP_TCM_CLK_FORCE_ON** `[0:0]`
    ///
    /// hp_tcm clk gatig force on
    #[inline(always)]
    pub const fn with_reg_hp_tcm_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TcmRamPwrCtrl0::REG_HP_TCM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !TcmRamPwrCtrl0::REG_HP_TCM_CLK_FORCE_ON_MASK
            },
        }
    }
}

impl Default for TcmRamPwrCtrl0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TcmRamPwrCtrl0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcmRamPwrCtrl0Val {
        TcmRamPwrCtrl0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TcmRamPwrCtrl0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TcmRamPwrCtrl0Val) -> TcmRamPwrCtrl0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L2_ROM_PWR_CTRL0 Register
///
/// NA
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **REG_L2_ROM_CLK_FORCE_ON** `[0]` (RW)
///
///   l2_rom clk gating force on
#[repr(transparent)]
pub struct L2RomPwrCtrl0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2RomPwrCtrl0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2RomPwrCtrl0 {
    // ======== Field Constants ========

    /// **REG_L2_ROM_CLK_FORCE_ON**
    ///
    /// l2_rom clk gating force on
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_L2_ROM_CLK_FORCE_ON_OFFSET: u32 = 0;
    pub const REG_L2_ROM_CLK_FORCE_ON_MASK: u32 = 0x00000001;
    pub const REG_L2_ROM_CLK_FORCE_ON: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_L2_ROM_CLK_FORCE_ON** `[0]` (RW)
    ///
    /// l2_rom clk gating force on
    #[inline(always)]
    pub fn reg_l2_rom_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_ROM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set REG_L2_ROM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_rom_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_ROM_CLK_FORCE_ON_MASK);
    }

    /// Clear REG_L2_ROM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_rom_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_ROM_CLK_FORCE_ON_MASK);
    }
}

/// L2_ROM_PWR_CTRL0 value type for batch read/modify/write operations
///
/// Use `l2rompwrctrl0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2rompwrctrl0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2RomPwrCtrl0Val {
    bits: u32,
}

impl L2RomPwrCtrl0Val {
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

    /// Read **REG_L2_ROM_CLK_FORCE_ON** `[0:0]`
    ///
    /// l2_rom clk gating force on
    #[inline(always)]
    pub const fn reg_l2_rom_clk_force_on(&self) -> bool {
        (self.bits & L2RomPwrCtrl0::REG_L2_ROM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set **REG_L2_ROM_CLK_FORCE_ON** `[0:0]`
    ///
    /// l2_rom clk gating force on
    #[inline(always)]
    pub const fn with_reg_l2_rom_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2RomPwrCtrl0::REG_L2_ROM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !L2RomPwrCtrl0::REG_L2_ROM_CLK_FORCE_ON_MASK
            },
        }
    }
}

impl Default for L2RomPwrCtrl0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2RomPwrCtrl0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2RomPwrCtrl0Val {
        L2RomPwrCtrl0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2RomPwrCtrl0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2RomPwrCtrl0Val) -> L2RomPwrCtrl0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PROBEA_CTRL Register
///
/// NA
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **REG_PROBE_GLOBAL_EN** `[28]` (RW)
///
///   Set this bit to enable global debug probe in hp system.
///
/// - **REG_PROBE_H_SEL** `[27:26]` (RW)
///
///   Tihs field is used to selec probe_out[31:16]
///
/// - **REG_PROBE_L_SEL** `[25:24]` (RW)
///
///   Tihs field is used to selec probe_out[31:16]
///
/// - **REG_PROBE_A_TOP_SEL** `[23:16]` (RW)
///
///   Tihs field is used to selec module's probe_out[31:0] as probe out in a mode
///
/// - **REG_PROBE_A_MOD_SEL** `[15:0]` (RW)
///
///   Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out[31:0] in a mode
#[repr(transparent)]
pub struct ProbeaCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ProbeaCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ProbeaCtrl {
    // ======== Field Constants ========

    /// **REG_PROBE_GLOBAL_EN**
    ///
    /// Set this bit to enable global debug probe in hp system.
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const REG_PROBE_GLOBAL_EN_OFFSET: u32 = 28;
    pub const REG_PROBE_GLOBAL_EN_MASK: u32 = 0x10000000;
    pub const REG_PROBE_GLOBAL_EN: u32 = 1 << 28;

    /// **REG_PROBE_H_SEL**
    ///
    /// Tihs field is used to selec probe_out[31:16]
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const REG_PROBE_H_SEL_OFFSET: u32 = 26;
    pub const REG_PROBE_H_SEL_MASK: u32 = 0x0c000000;

    /// **REG_PROBE_L_SEL**
    ///
    /// Tihs field is used to selec probe_out[31:16]
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const REG_PROBE_L_SEL_OFFSET: u32 = 24;
    pub const REG_PROBE_L_SEL_MASK: u32 = 0x03000000;

    /// **REG_PROBE_A_TOP_SEL**
    ///
    /// Tihs field is used to selec module's probe_out[31:0] as probe out in a mode
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_PROBE_A_TOP_SEL_OFFSET: u32 = 16;
    pub const REG_PROBE_A_TOP_SEL_MASK: u32 = 0x00ff0000;

    /// **REG_PROBE_A_MOD_SEL**
    ///
    /// Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out[31:0] in a mode
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_PROBE_A_MOD_SEL_OFFSET: u32 = 0;
    pub const REG_PROBE_A_MOD_SEL_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **REG_PROBE_GLOBAL_EN** `[28]` (RW)
    ///
    /// Set this bit to enable global debug probe in hp system.
    #[inline(always)]
    pub fn reg_probe_global_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PROBE_GLOBAL_EN_MASK) != 0
    }

    /// Set REG_PROBE_GLOBAL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_probe_global_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_PROBE_GLOBAL_EN_MASK);
    }

    /// Clear REG_PROBE_GLOBAL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_probe_global_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_PROBE_GLOBAL_EN_MASK);
    }

    /// **REG_PROBE_H_SEL** `[27:26]` (RW)
    ///
    /// Tihs field is used to selec probe_out[31:16]
    #[inline(always)]
    pub fn reg_probe_h_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PROBE_H_SEL_MASK) >> Self::REG_PROBE_H_SEL_OFFSET
    }

    /// Write **REG_PROBE_H_SEL** `[27:26]` (RW)
    ///
    /// Tihs field is used to selec probe_out[31:16]
    #[inline(always)]
    pub fn write_reg_probe_h_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PROBE_H_SEL_MASK)
                | ((value << Self::REG_PROBE_H_SEL_OFFSET) & Self::REG_PROBE_H_SEL_MASK)
        });
    }

    /// **REG_PROBE_L_SEL** `[25:24]` (RW)
    ///
    /// Tihs field is used to selec probe_out[31:16]
    #[inline(always)]
    pub fn reg_probe_l_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PROBE_L_SEL_MASK) >> Self::REG_PROBE_L_SEL_OFFSET
    }

    /// Write **REG_PROBE_L_SEL** `[25:24]` (RW)
    ///
    /// Tihs field is used to selec probe_out[31:16]
    #[inline(always)]
    pub fn write_reg_probe_l_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PROBE_L_SEL_MASK)
                | ((value << Self::REG_PROBE_L_SEL_OFFSET) & Self::REG_PROBE_L_SEL_MASK)
        });
    }

    /// **REG_PROBE_A_TOP_SEL** `[23:16]` (RW)
    ///
    /// Tihs field is used to selec module's probe_out[31:0] as probe out in a mode
    #[inline(always)]
    pub fn reg_probe_a_top_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PROBE_A_TOP_SEL_MASK)
            >> Self::REG_PROBE_A_TOP_SEL_OFFSET
    }

    /// Write **REG_PROBE_A_TOP_SEL** `[23:16]` (RW)
    ///
    /// Tihs field is used to selec module's probe_out[31:0] as probe out in a mode
    #[inline(always)]
    pub fn write_reg_probe_a_top_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PROBE_A_TOP_SEL_MASK)
                | ((value << Self::REG_PROBE_A_TOP_SEL_OFFSET) & Self::REG_PROBE_A_TOP_SEL_MASK)
        });
    }

    /// **REG_PROBE_A_MOD_SEL** `[15:0]` (RW)
    ///
    /// Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out[31:0] in a mode
    #[inline(always)]
    pub fn reg_probe_a_mod_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PROBE_A_MOD_SEL_MASK)
            >> Self::REG_PROBE_A_MOD_SEL_OFFSET
    }

    /// Write **REG_PROBE_A_MOD_SEL** `[15:0]` (RW)
    ///
    /// Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out[31:0] in a mode
    #[inline(always)]
    pub fn write_reg_probe_a_mod_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PROBE_A_MOD_SEL_MASK)
                | ((value << Self::REG_PROBE_A_MOD_SEL_OFFSET) & Self::REG_PROBE_A_MOD_SEL_MASK)
        });
    }
}

/// PROBEA_CTRL value type for batch read/modify/write operations
///
/// Use `probeactrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `probeactrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ProbeaCtrlVal {
    bits: u32,
}

impl ProbeaCtrlVal {
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

    /// Read **REG_PROBE_GLOBAL_EN** `[28:28]`
    ///
    /// Set this bit to enable global debug probe in hp system.
    #[inline(always)]
    pub const fn reg_probe_global_en(&self) -> bool {
        (self.bits & ProbeaCtrl::REG_PROBE_GLOBAL_EN_MASK) != 0
    }

    /// Read **REG_PROBE_H_SEL** `[27:26]`
    ///
    /// Tihs field is used to selec probe_out[31:16]
    #[inline(always)]
    pub const fn reg_probe_h_sel(&self) -> u32 {
        (self.bits & ProbeaCtrl::REG_PROBE_H_SEL_MASK) >> ProbeaCtrl::REG_PROBE_H_SEL_OFFSET
    }

    /// Read **REG_PROBE_L_SEL** `[25:24]`
    ///
    /// Tihs field is used to selec probe_out[31:16]
    #[inline(always)]
    pub const fn reg_probe_l_sel(&self) -> u32 {
        (self.bits & ProbeaCtrl::REG_PROBE_L_SEL_MASK) >> ProbeaCtrl::REG_PROBE_L_SEL_OFFSET
    }

    /// Read **REG_PROBE_A_TOP_SEL** `[23:16]`
    ///
    /// Tihs field is used to selec module's probe_out[31:0] as probe out in a mode
    #[inline(always)]
    pub const fn reg_probe_a_top_sel(&self) -> u32 {
        (self.bits & ProbeaCtrl::REG_PROBE_A_TOP_SEL_MASK) >> ProbeaCtrl::REG_PROBE_A_TOP_SEL_OFFSET
    }

    /// Read **REG_PROBE_A_MOD_SEL** `[15:0]`
    ///
    /// Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out[31:0] in a mode
    #[inline(always)]
    pub const fn reg_probe_a_mod_sel(&self) -> u32 {
        (self.bits & ProbeaCtrl::REG_PROBE_A_MOD_SEL_MASK) >> ProbeaCtrl::REG_PROBE_A_MOD_SEL_OFFSET
    }

    /// Set **REG_PROBE_GLOBAL_EN** `[28:28]`
    ///
    /// Set this bit to enable global debug probe in hp system.
    #[inline(always)]
    pub const fn with_reg_probe_global_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ProbeaCtrl::REG_PROBE_GLOBAL_EN_MASK
            } else {
                self.bits & !ProbeaCtrl::REG_PROBE_GLOBAL_EN_MASK
            },
        }
    }

    /// Set **REG_PROBE_H_SEL** `[27:26]`
    ///
    /// Tihs field is used to selec probe_out[31:16]
    #[inline(always)]
    pub const fn with_reg_probe_h_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ProbeaCtrl::REG_PROBE_H_SEL_MASK)
                | ((value << ProbeaCtrl::REG_PROBE_H_SEL_OFFSET)
                    & ProbeaCtrl::REG_PROBE_H_SEL_MASK),
        }
    }

    /// Set **REG_PROBE_L_SEL** `[25:24]`
    ///
    /// Tihs field is used to selec probe_out[31:16]
    #[inline(always)]
    pub const fn with_reg_probe_l_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ProbeaCtrl::REG_PROBE_L_SEL_MASK)
                | ((value << ProbeaCtrl::REG_PROBE_L_SEL_OFFSET)
                    & ProbeaCtrl::REG_PROBE_L_SEL_MASK),
        }
    }

    /// Set **REG_PROBE_A_TOP_SEL** `[23:16]`
    ///
    /// Tihs field is used to selec module's probe_out[31:0] as probe out in a mode
    #[inline(always)]
    pub const fn with_reg_probe_a_top_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ProbeaCtrl::REG_PROBE_A_TOP_SEL_MASK)
                | ((value << ProbeaCtrl::REG_PROBE_A_TOP_SEL_OFFSET)
                    & ProbeaCtrl::REG_PROBE_A_TOP_SEL_MASK),
        }
    }

    /// Set **REG_PROBE_A_MOD_SEL** `[15:0]`
    ///
    /// Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out[31:0] in a mode
    #[inline(always)]
    pub const fn with_reg_probe_a_mod_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ProbeaCtrl::REG_PROBE_A_MOD_SEL_MASK)
                | ((value << ProbeaCtrl::REG_PROBE_A_MOD_SEL_OFFSET)
                    & ProbeaCtrl::REG_PROBE_A_MOD_SEL_MASK),
        }
    }
}

impl Default for ProbeaCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ProbeaCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ProbeaCtrlVal {
        ProbeaCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ProbeaCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ProbeaCtrlVal) -> ProbeaCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PROBEB_CTRL Register
///
/// NA
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **REG_PROBE_B_EN** `[24]` (RW)
///
///   Set this bit to enable b mode for debug probe. 1: b mode, 0: a mode.
///
/// - **REG_PROBE_B_TOP_SEL** `[23:16]` (RW)
///
///   Tihs field is used to select module's probe_out[31:0] as probe_out in b mode
///
/// - **REG_PROBE_B_MOD_SEL** `[15:0]` (RW)
///
///   Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out[31:0] in b mode.
#[repr(transparent)]
pub struct ProbebCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ProbebCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ProbebCtrl {
    // ======== Field Constants ========

    /// **REG_PROBE_B_EN**
    ///
    /// Set this bit to enable b mode for debug probe. 1: b mode, 0: a mode.
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const REG_PROBE_B_EN_OFFSET: u32 = 24;
    pub const REG_PROBE_B_EN_MASK: u32 = 0x01000000;
    pub const REG_PROBE_B_EN: u32 = 1 << 24;

    /// **REG_PROBE_B_TOP_SEL**
    ///
    /// Tihs field is used to select module's probe_out[31:0] as probe_out in b mode
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const REG_PROBE_B_TOP_SEL_OFFSET: u32 = 16;
    pub const REG_PROBE_B_TOP_SEL_MASK: u32 = 0x00ff0000;

    /// **REG_PROBE_B_MOD_SEL**
    ///
    /// Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out[31:0] in b mode.
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const REG_PROBE_B_MOD_SEL_OFFSET: u32 = 0;
    pub const REG_PROBE_B_MOD_SEL_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **REG_PROBE_B_EN** `[24]` (RW)
    ///
    /// Set this bit to enable b mode for debug probe. 1: b mode, 0: a mode.
    #[inline(always)]
    pub fn reg_probe_b_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_PROBE_B_EN_MASK) != 0
    }

    /// Set REG_PROBE_B_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_probe_b_en(&self) {
        self.value.modify_bits(|r| r | Self::REG_PROBE_B_EN_MASK);
    }

    /// Clear REG_PROBE_B_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_probe_b_en(&self) {
        self.value.modify_bits(|r| r & !Self::REG_PROBE_B_EN_MASK);
    }

    /// **REG_PROBE_B_TOP_SEL** `[23:16]` (RW)
    ///
    /// Tihs field is used to select module's probe_out[31:0] as probe_out in b mode
    #[inline(always)]
    pub fn reg_probe_b_top_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PROBE_B_TOP_SEL_MASK)
            >> Self::REG_PROBE_B_TOP_SEL_OFFSET
    }

    /// Write **REG_PROBE_B_TOP_SEL** `[23:16]` (RW)
    ///
    /// Tihs field is used to select module's probe_out[31:0] as probe_out in b mode
    #[inline(always)]
    pub fn write_reg_probe_b_top_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PROBE_B_TOP_SEL_MASK)
                | ((value << Self::REG_PROBE_B_TOP_SEL_OFFSET) & Self::REG_PROBE_B_TOP_SEL_MASK)
        });
    }

    /// **REG_PROBE_B_MOD_SEL** `[15:0]` (RW)
    ///
    /// Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out[31:0] in b mode.
    #[inline(always)]
    pub fn reg_probe_b_mod_sel(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PROBE_B_MOD_SEL_MASK)
            >> Self::REG_PROBE_B_MOD_SEL_OFFSET
    }

    /// Write **REG_PROBE_B_MOD_SEL** `[15:0]` (RW)
    ///
    /// Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out[31:0] in b mode.
    #[inline(always)]
    pub fn write_reg_probe_b_mod_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_PROBE_B_MOD_SEL_MASK)
                | ((value << Self::REG_PROBE_B_MOD_SEL_OFFSET) & Self::REG_PROBE_B_MOD_SEL_MASK)
        });
    }
}

/// PROBEB_CTRL value type for batch read/modify/write operations
///
/// Use `probebctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `probebctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ProbebCtrlVal {
    bits: u32,
}

impl ProbebCtrlVal {
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

    /// Read **REG_PROBE_B_EN** `[24:24]`
    ///
    /// Set this bit to enable b mode for debug probe. 1: b mode, 0: a mode.
    #[inline(always)]
    pub const fn reg_probe_b_en(&self) -> bool {
        (self.bits & ProbebCtrl::REG_PROBE_B_EN_MASK) != 0
    }

    /// Read **REG_PROBE_B_TOP_SEL** `[23:16]`
    ///
    /// Tihs field is used to select module's probe_out[31:0] as probe_out in b mode
    #[inline(always)]
    pub const fn reg_probe_b_top_sel(&self) -> u32 {
        (self.bits & ProbebCtrl::REG_PROBE_B_TOP_SEL_MASK) >> ProbebCtrl::REG_PROBE_B_TOP_SEL_OFFSET
    }

    /// Read **REG_PROBE_B_MOD_SEL** `[15:0]`
    ///
    /// Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out[31:0] in b mode.
    #[inline(always)]
    pub const fn reg_probe_b_mod_sel(&self) -> u32 {
        (self.bits & ProbebCtrl::REG_PROBE_B_MOD_SEL_MASK) >> ProbebCtrl::REG_PROBE_B_MOD_SEL_OFFSET
    }

    /// Set **REG_PROBE_B_EN** `[24:24]`
    ///
    /// Set this bit to enable b mode for debug probe. 1: b mode, 0: a mode.
    #[inline(always)]
    pub const fn with_reg_probe_b_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ProbebCtrl::REG_PROBE_B_EN_MASK
            } else {
                self.bits & !ProbebCtrl::REG_PROBE_B_EN_MASK
            },
        }
    }

    /// Set **REG_PROBE_B_TOP_SEL** `[23:16]`
    ///
    /// Tihs field is used to select module's probe_out[31:0] as probe_out in b mode
    #[inline(always)]
    pub const fn with_reg_probe_b_top_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ProbebCtrl::REG_PROBE_B_TOP_SEL_MASK)
                | ((value << ProbebCtrl::REG_PROBE_B_TOP_SEL_OFFSET)
                    & ProbebCtrl::REG_PROBE_B_TOP_SEL_MASK),
        }
    }

    /// Set **REG_PROBE_B_MOD_SEL** `[15:0]`
    ///
    /// Tihs field is used to selec probe_group from probe_group0 to probe_group15 for module's probe_out[31:0] in b mode.
    #[inline(always)]
    pub const fn with_reg_probe_b_mod_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ProbebCtrl::REG_PROBE_B_MOD_SEL_MASK)
                | ((value << ProbebCtrl::REG_PROBE_B_MOD_SEL_OFFSET)
                    & ProbebCtrl::REG_PROBE_B_MOD_SEL_MASK),
        }
    }
}

impl Default for ProbebCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ProbebCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ProbebCtrlVal {
        ProbebCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ProbebCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ProbebCtrlVal) -> ProbebCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PROBE_OUT Register
///
/// NA
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **REG_PROBE_TOP_OUT** `[31:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct ProbeOut {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ProbeOut {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ProbeOut {
    // ======== Field Constants ========

    /// **REG_PROBE_TOP_OUT**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_PROBE_TOP_OUT_OFFSET: u32 = 0;
    pub const REG_PROBE_TOP_OUT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_PROBE_TOP_OUT** `[31:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_probe_top_out(&self) -> u32 {
        (self.value.read_bits() & Self::REG_PROBE_TOP_OUT_MASK) >> Self::REG_PROBE_TOP_OUT_OFFSET
    }
}

/// PROBE_OUT value type for batch read/modify/write operations
///
/// Use `probeout.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `probeout.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ProbeOutVal {
    bits: u32,
}

impl ProbeOutVal {
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

    /// Read **REG_PROBE_TOP_OUT** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_probe_top_out(&self) -> u32 {
        (self.bits & ProbeOut::REG_PROBE_TOP_OUT_MASK) >> ProbeOut::REG_PROBE_TOP_OUT_OFFSET
    }
}

impl Default for ProbeOutVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ProbeOut {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ProbeOutVal {
        ProbeOutVal::from_bits(self.value.read_bits())
    }
}

/// L2_MEM_RAM_PWR_CTRL0 Register
///
/// NA
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **REG_L2_MEM_CLK_FORCE_ON** `[0]` (RW)
///
///   l2ram clk_gating force on
#[repr(transparent)]
pub struct L2MemRamPwrCtrl0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemRamPwrCtrl0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemRamPwrCtrl0 {
    // ======== Field Constants ========

    /// **REG_L2_MEM_CLK_FORCE_ON**
    ///
    /// l2ram clk_gating force on
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_L2_MEM_CLK_FORCE_ON_OFFSET: u32 = 0;
    pub const REG_L2_MEM_CLK_FORCE_ON_MASK: u32 = 0x00000001;
    pub const REG_L2_MEM_CLK_FORCE_ON: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_L2_MEM_CLK_FORCE_ON** `[0]` (RW)
    ///
    /// l2ram clk_gating force on
    #[inline(always)]
    pub fn reg_l2_mem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set REG_L2_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_CLK_FORCE_ON_MASK);
    }

    /// Clear REG_L2_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_CLK_FORCE_ON_MASK);
    }
}

/// L2_MEM_RAM_PWR_CTRL0 value type for batch read/modify/write operations
///
/// Use `l2memrampwrctrl0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memrampwrctrl0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemRamPwrCtrl0Val {
    bits: u32,
}

impl L2MemRamPwrCtrl0Val {
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

    /// Read **REG_L2_MEM_CLK_FORCE_ON** `[0:0]`
    ///
    /// l2ram clk_gating force on
    #[inline(always)]
    pub const fn reg_l2_mem_clk_force_on(&self) -> bool {
        (self.bits & L2MemRamPwrCtrl0::REG_L2_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set **REG_L2_MEM_CLK_FORCE_ON** `[0:0]`
    ///
    /// l2ram clk_gating force on
    #[inline(always)]
    pub const fn with_reg_l2_mem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemRamPwrCtrl0::REG_L2_MEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !L2MemRamPwrCtrl0::REG_L2_MEM_CLK_FORCE_ON_MASK
            },
        }
    }
}

impl Default for L2MemRamPwrCtrl0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemRamPwrCtrl0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemRamPwrCtrl0Val {
        L2MemRamPwrCtrl0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2MemRamPwrCtrl0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2MemRamPwrCtrl0Val) -> L2MemRamPwrCtrl0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CPU_CORESTALLED_ST Register
///
/// NA
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **REG_CORE1_CORESTALLED_ST** `[1]` (RO)
///
///   hp core1 corestalled status
///
/// - **REG_CORE0_CORESTALLED_ST** `[0]` (RO)
///
///   hp core0 corestalled status
#[repr(transparent)]
pub struct CpuCorestalledSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CpuCorestalledSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CpuCorestalledSt {
    // ======== Field Constants ========

    /// **REG_CORE1_CORESTALLED_ST**
    ///
    /// hp core1 corestalled status
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_CORE1_CORESTALLED_ST_OFFSET: u32 = 1;
    pub const REG_CORE1_CORESTALLED_ST_MASK: u32 = 0x00000002;
    pub const REG_CORE1_CORESTALLED_ST: u32 = 1 << 1;

    /// **REG_CORE0_CORESTALLED_ST**
    ///
    /// hp core0 corestalled status
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_CORE0_CORESTALLED_ST_OFFSET: u32 = 0;
    pub const REG_CORE0_CORESTALLED_ST_MASK: u32 = 0x00000001;
    pub const REG_CORE0_CORESTALLED_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_CORE1_CORESTALLED_ST** `[1]` (RO)
    ///
    /// hp core1 corestalled status
    #[inline(always)]
    pub fn reg_core1_corestalled_st(&self) -> bool {
        (self.value.read_bits() & Self::REG_CORE1_CORESTALLED_ST_MASK) != 0
    }

    /// **REG_CORE0_CORESTALLED_ST** `[0]` (RO)
    ///
    /// hp core0 corestalled status
    #[inline(always)]
    pub fn reg_core0_corestalled_st(&self) -> bool {
        (self.value.read_bits() & Self::REG_CORE0_CORESTALLED_ST_MASK) != 0
    }
}

/// CPU_CORESTALLED_ST value type for batch read/modify/write operations
///
/// Use `cpucorestalledst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cpucorestalledst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CpuCorestalledStVal {
    bits: u32,
}

impl CpuCorestalledStVal {
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

    /// Read **REG_CORE1_CORESTALLED_ST** `[1:1]`
    ///
    /// hp core1 corestalled status
    #[inline(always)]
    pub const fn reg_core1_corestalled_st(&self) -> bool {
        (self.bits & CpuCorestalledSt::REG_CORE1_CORESTALLED_ST_MASK) != 0
    }

    /// Read **REG_CORE0_CORESTALLED_ST** `[0:0]`
    ///
    /// hp core0 corestalled status
    #[inline(always)]
    pub const fn reg_core0_corestalled_st(&self) -> bool {
        (self.bits & CpuCorestalledSt::REG_CORE0_CORESTALLED_ST_MASK) != 0
    }
}

impl Default for CpuCorestalledStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CpuCorestalledSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CpuCorestalledStVal {
        CpuCorestalledStVal::from_bits(self.value.read_bits())
    }
}

/// CRYPTO_CTRL Register
///
/// NA
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT** `[3]` (RW)
///
///   NA
///
/// - **REG_ENABLE_DOWNLOAD_G0CB_DECRYPT** `[2]` (RW)
///
///   NA
///
/// - **REG_ENABLE_DOWNLOAD_DB_ENCRYPT** `[1]` (RW)
///
///   NA
///
/// - **REG_ENABLE_SPI_MANUAL_ENCRYPT** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct CryptoCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CryptoCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CryptoCtrl {
    // ======== Field Constants ========

    /// **REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_OFFSET: u32 = 3;
    pub const REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_MASK: u32 = 0x00000008;
    pub const REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT: u32 = 1 << 3;

    /// **REG_ENABLE_DOWNLOAD_G0CB_DECRYPT**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_ENABLE_DOWNLOAD_G0CB_DECRYPT_OFFSET: u32 = 2;
    pub const REG_ENABLE_DOWNLOAD_G0CB_DECRYPT_MASK: u32 = 0x00000004;
    pub const REG_ENABLE_DOWNLOAD_G0CB_DECRYPT: u32 = 1 << 2;

    /// **REG_ENABLE_DOWNLOAD_DB_ENCRYPT**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_ENABLE_DOWNLOAD_DB_ENCRYPT_OFFSET: u32 = 1;
    pub const REG_ENABLE_DOWNLOAD_DB_ENCRYPT_MASK: u32 = 0x00000002;
    pub const REG_ENABLE_DOWNLOAD_DB_ENCRYPT: u32 = 1 << 1;

    /// **REG_ENABLE_SPI_MANUAL_ENCRYPT**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_ENABLE_SPI_MANUAL_ENCRYPT_OFFSET: u32 = 0;
    pub const REG_ENABLE_SPI_MANUAL_ENCRYPT_MASK: u32 = 0x00000001;
    pub const REG_ENABLE_SPI_MANUAL_ENCRYPT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_enable_download_manual_encrypt(&self) -> bool {
        (self.value.read_bits() & Self::REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_MASK) != 0
    }

    /// Set REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_enable_download_manual_encrypt(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_MASK);
    }

    /// Clear REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_enable_download_manual_encrypt(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_MASK);
    }

    /// **REG_ENABLE_DOWNLOAD_G0CB_DECRYPT** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_enable_download_g0cb_decrypt(&self) -> bool {
        (self.value.read_bits() & Self::REG_ENABLE_DOWNLOAD_G0CB_DECRYPT_MASK) != 0
    }

    /// Set REG_ENABLE_DOWNLOAD_G0CB_DECRYPT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_enable_download_g0cb_decrypt(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_ENABLE_DOWNLOAD_G0CB_DECRYPT_MASK);
    }

    /// Clear REG_ENABLE_DOWNLOAD_G0CB_DECRYPT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_enable_download_g0cb_decrypt(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_ENABLE_DOWNLOAD_G0CB_DECRYPT_MASK);
    }

    /// **REG_ENABLE_DOWNLOAD_DB_ENCRYPT** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_enable_download_db_encrypt(&self) -> bool {
        (self.value.read_bits() & Self::REG_ENABLE_DOWNLOAD_DB_ENCRYPT_MASK) != 0
    }

    /// Set REG_ENABLE_DOWNLOAD_DB_ENCRYPT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_enable_download_db_encrypt(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_ENABLE_DOWNLOAD_DB_ENCRYPT_MASK);
    }

    /// Clear REG_ENABLE_DOWNLOAD_DB_ENCRYPT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_enable_download_db_encrypt(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_ENABLE_DOWNLOAD_DB_ENCRYPT_MASK);
    }

    /// **REG_ENABLE_SPI_MANUAL_ENCRYPT** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_enable_spi_manual_encrypt(&self) -> bool {
        (self.value.read_bits() & Self::REG_ENABLE_SPI_MANUAL_ENCRYPT_MASK) != 0
    }

    /// Set REG_ENABLE_SPI_MANUAL_ENCRYPT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_enable_spi_manual_encrypt(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_ENABLE_SPI_MANUAL_ENCRYPT_MASK);
    }

    /// Clear REG_ENABLE_SPI_MANUAL_ENCRYPT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_enable_spi_manual_encrypt(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_ENABLE_SPI_MANUAL_ENCRYPT_MASK);
    }
}

/// CRYPTO_CTRL value type for batch read/modify/write operations
///
/// Use `cryptoctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cryptoctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CryptoCtrlVal {
    bits: u32,
}

impl CryptoCtrlVal {
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

    /// Read **REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_enable_download_manual_encrypt(&self) -> bool {
        (self.bits & CryptoCtrl::REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_MASK) != 0
    }

    /// Read **REG_ENABLE_DOWNLOAD_G0CB_DECRYPT** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_enable_download_g0cb_decrypt(&self) -> bool {
        (self.bits & CryptoCtrl::REG_ENABLE_DOWNLOAD_G0CB_DECRYPT_MASK) != 0
    }

    /// Read **REG_ENABLE_DOWNLOAD_DB_ENCRYPT** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_enable_download_db_encrypt(&self) -> bool {
        (self.bits & CryptoCtrl::REG_ENABLE_DOWNLOAD_DB_ENCRYPT_MASK) != 0
    }

    /// Read **REG_ENABLE_SPI_MANUAL_ENCRYPT** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_enable_spi_manual_encrypt(&self) -> bool {
        (self.bits & CryptoCtrl::REG_ENABLE_SPI_MANUAL_ENCRYPT_MASK) != 0
    }

    /// Set **REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_enable_download_manual_encrypt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CryptoCtrl::REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_MASK
            } else {
                self.bits & !CryptoCtrl::REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_MASK
            },
        }
    }

    /// Set **REG_ENABLE_DOWNLOAD_G0CB_DECRYPT** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_enable_download_g0cb_decrypt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CryptoCtrl::REG_ENABLE_DOWNLOAD_G0CB_DECRYPT_MASK
            } else {
                self.bits & !CryptoCtrl::REG_ENABLE_DOWNLOAD_G0CB_DECRYPT_MASK
            },
        }
    }

    /// Set **REG_ENABLE_DOWNLOAD_DB_ENCRYPT** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_enable_download_db_encrypt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CryptoCtrl::REG_ENABLE_DOWNLOAD_DB_ENCRYPT_MASK
            } else {
                self.bits & !CryptoCtrl::REG_ENABLE_DOWNLOAD_DB_ENCRYPT_MASK
            },
        }
    }

    /// Set **REG_ENABLE_SPI_MANUAL_ENCRYPT** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_enable_spi_manual_encrypt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CryptoCtrl::REG_ENABLE_SPI_MANUAL_ENCRYPT_MASK
            } else {
                self.bits & !CryptoCtrl::REG_ENABLE_SPI_MANUAL_ENCRYPT_MASK
            },
        }
    }
}

impl Default for CryptoCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CryptoCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CryptoCtrlVal {
        CryptoCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CryptoCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CryptoCtrlVal) -> CryptoCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GPIO_O_HOLD_CTRL0 Register
///
/// NA
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **REG_GPIO_0_HOLD_LOW** `[31:0]` (RW)
///
///   hold control for gpio47~16
#[repr(transparent)]
pub struct GpioOHoldCtrl0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioOHoldCtrl0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioOHoldCtrl0 {
    // ======== Field Constants ========

    /// **REG_GPIO_0_HOLD_LOW**
    ///
    /// hold control for gpio47~16
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_GPIO_0_HOLD_LOW_OFFSET: u32 = 0;
    pub const REG_GPIO_0_HOLD_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_GPIO_0_HOLD_LOW** `[31:0]` (RW)
    ///
    /// hold control for gpio47~16
    #[inline(always)]
    pub fn reg_gpio_0_hold_low(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_0_HOLD_LOW_MASK)
            >> Self::REG_GPIO_0_HOLD_LOW_OFFSET
    }

    /// Write **REG_GPIO_0_HOLD_LOW** `[31:0]` (RW)
    ///
    /// hold control for gpio47~16
    #[inline(always)]
    pub fn write_reg_gpio_0_hold_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_0_HOLD_LOW_MASK)
                | ((value << Self::REG_GPIO_0_HOLD_LOW_OFFSET) & Self::REG_GPIO_0_HOLD_LOW_MASK)
        });
    }
}

/// GPIO_O_HOLD_CTRL0 value type for batch read/modify/write operations
///
/// Use `gpiooholdctrl0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiooholdctrl0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioOHoldCtrl0Val {
    bits: u32,
}

impl GpioOHoldCtrl0Val {
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

    /// Read **REG_GPIO_0_HOLD_LOW** `[31:0]`
    ///
    /// hold control for gpio47~16
    #[inline(always)]
    pub const fn reg_gpio_0_hold_low(&self) -> u32 {
        (self.bits & GpioOHoldCtrl0::REG_GPIO_0_HOLD_LOW_MASK)
            >> GpioOHoldCtrl0::REG_GPIO_0_HOLD_LOW_OFFSET
    }

    /// Set **REG_GPIO_0_HOLD_LOW** `[31:0]`
    ///
    /// hold control for gpio47~16
    #[inline(always)]
    pub const fn with_reg_gpio_0_hold_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioOHoldCtrl0::REG_GPIO_0_HOLD_LOW_MASK)
                | ((value << GpioOHoldCtrl0::REG_GPIO_0_HOLD_LOW_OFFSET)
                    & GpioOHoldCtrl0::REG_GPIO_0_HOLD_LOW_MASK),
        }
    }
}

impl Default for GpioOHoldCtrl0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GpioOHoldCtrl0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioOHoldCtrl0Val {
        GpioOHoldCtrl0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioOHoldCtrl0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioOHoldCtrl0Val) -> GpioOHoldCtrl0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GPIO_O_HOLD_CTRL1 Register
///
/// NA
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **REG_GPIO_0_HOLD_HIGH** `[8:0]` (RW)
///
///   hold control for gpio56~48
#[repr(transparent)]
pub struct GpioOHoldCtrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioOHoldCtrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioOHoldCtrl1 {
    // ======== Field Constants ========

    /// **REG_GPIO_0_HOLD_HIGH**
    ///
    /// hold control for gpio56~48
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const REG_GPIO_0_HOLD_HIGH_OFFSET: u32 = 0;
    pub const REG_GPIO_0_HOLD_HIGH_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **REG_GPIO_0_HOLD_HIGH** `[8:0]` (RW)
    ///
    /// hold control for gpio56~48
    #[inline(always)]
    pub fn reg_gpio_0_hold_high(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_0_HOLD_HIGH_MASK)
            >> Self::REG_GPIO_0_HOLD_HIGH_OFFSET
    }

    /// Write **REG_GPIO_0_HOLD_HIGH** `[8:0]` (RW)
    ///
    /// hold control for gpio56~48
    #[inline(always)]
    pub fn write_reg_gpio_0_hold_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_0_HOLD_HIGH_MASK)
                | ((value << Self::REG_GPIO_0_HOLD_HIGH_OFFSET) & Self::REG_GPIO_0_HOLD_HIGH_MASK)
        });
    }
}

/// GPIO_O_HOLD_CTRL1 value type for batch read/modify/write operations
///
/// Use `gpiooholdctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiooholdctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioOHoldCtrl1Val {
    bits: u32,
}

impl GpioOHoldCtrl1Val {
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

    /// Read **REG_GPIO_0_HOLD_HIGH** `[8:0]`
    ///
    /// hold control for gpio56~48
    #[inline(always)]
    pub const fn reg_gpio_0_hold_high(&self) -> u32 {
        (self.bits & GpioOHoldCtrl1::REG_GPIO_0_HOLD_HIGH_MASK)
            >> GpioOHoldCtrl1::REG_GPIO_0_HOLD_HIGH_OFFSET
    }

    /// Set **REG_GPIO_0_HOLD_HIGH** `[8:0]`
    ///
    /// hold control for gpio56~48
    #[inline(always)]
    pub const fn with_reg_gpio_0_hold_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioOHoldCtrl1::REG_GPIO_0_HOLD_HIGH_MASK)
                | ((value << GpioOHoldCtrl1::REG_GPIO_0_HOLD_HIGH_OFFSET)
                    & GpioOHoldCtrl1::REG_GPIO_0_HOLD_HIGH_MASK),
        }
    }
}

impl Default for GpioOHoldCtrl1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GpioOHoldCtrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioOHoldCtrl1Val {
        GpioOHoldCtrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioOHoldCtrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioOHoldCtrl1Val) -> GpioOHoldCtrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RDN_ECO_CS Register
///
/// NA
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **REG_HP_SYS_RDN_ECO_RESULT** `[1]` (RO)
///
///   NA
///
/// - **REG_HP_SYS_RDN_ECO_EN** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct RdnEcoCs {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RdnEcoCs {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RdnEcoCs {
    // ======== Field Constants ========

    /// **REG_HP_SYS_RDN_ECO_RESULT**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_HP_SYS_RDN_ECO_RESULT_OFFSET: u32 = 1;
    pub const REG_HP_SYS_RDN_ECO_RESULT_MASK: u32 = 0x00000002;
    pub const REG_HP_SYS_RDN_ECO_RESULT: u32 = 1 << 1;

    /// **REG_HP_SYS_RDN_ECO_EN**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_HP_SYS_RDN_ECO_EN_OFFSET: u32 = 0;
    pub const REG_HP_SYS_RDN_ECO_EN_MASK: u32 = 0x00000001;
    pub const REG_HP_SYS_RDN_ECO_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_HP_SYS_RDN_ECO_RESULT** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_hp_sys_rdn_eco_result(&self) -> bool {
        (self.value.read_bits() & Self::REG_HP_SYS_RDN_ECO_RESULT_MASK) != 0
    }

    /// **REG_HP_SYS_RDN_ECO_EN** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_hp_sys_rdn_eco_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_HP_SYS_RDN_ECO_EN_MASK) != 0
    }

    /// Set REG_HP_SYS_RDN_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_hp_sys_rdn_eco_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_HP_SYS_RDN_ECO_EN_MASK);
    }

    /// Clear REG_HP_SYS_RDN_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_hp_sys_rdn_eco_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_HP_SYS_RDN_ECO_EN_MASK);
    }
}

/// RDN_ECO_CS value type for batch read/modify/write operations
///
/// Use `rdnecocs.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rdnecocs.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RdnEcoCsVal {
    bits: u32,
}

impl RdnEcoCsVal {
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

    /// Read **REG_HP_SYS_RDN_ECO_RESULT** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_hp_sys_rdn_eco_result(&self) -> bool {
        (self.bits & RdnEcoCs::REG_HP_SYS_RDN_ECO_RESULT_MASK) != 0
    }

    /// Read **REG_HP_SYS_RDN_ECO_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_hp_sys_rdn_eco_en(&self) -> bool {
        (self.bits & RdnEcoCs::REG_HP_SYS_RDN_ECO_EN_MASK) != 0
    }

    /// Set **REG_HP_SYS_RDN_ECO_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_hp_sys_rdn_eco_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RdnEcoCs::REG_HP_SYS_RDN_ECO_EN_MASK
            } else {
                self.bits & !RdnEcoCs::REG_HP_SYS_RDN_ECO_EN_MASK
            },
        }
    }
}

impl Default for RdnEcoCsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RdnEcoCs {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RdnEcoCsVal {
        RdnEcoCsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RdnEcoCsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RdnEcoCsVal) -> RdnEcoCsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CACHE_APB_POSTW_EN Register
///
/// NA
///
/// Address offset: 0x0080
///
/// ## Fields
///
/// - **REG_CACHE_APB_POSTW_EN** `[0]` (RW)
///
///   cache apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
#[repr(transparent)]
pub struct CacheApbPostwEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CacheApbPostwEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CacheApbPostwEn {
    // ======== Field Constants ========

    /// **REG_CACHE_APB_POSTW_EN**
    ///
    /// cache apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_CACHE_APB_POSTW_EN_OFFSET: u32 = 0;
    pub const REG_CACHE_APB_POSTW_EN_MASK: u32 = 0x00000001;
    pub const REG_CACHE_APB_POSTW_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_CACHE_APB_POSTW_EN** `[0]` (RW)
    ///
    /// cache apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
    #[inline(always)]
    pub fn reg_cache_apb_postw_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_CACHE_APB_POSTW_EN_MASK) != 0
    }

    /// Set REG_CACHE_APB_POSTW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_cache_apb_postw_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_CACHE_APB_POSTW_EN_MASK);
    }

    /// Clear REG_CACHE_APB_POSTW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_cache_apb_postw_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_CACHE_APB_POSTW_EN_MASK);
    }
}

/// CACHE_APB_POSTW_EN value type for batch read/modify/write operations
///
/// Use `cacheapbpostwen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cacheapbpostwen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CacheApbPostwEnVal {
    bits: u32,
}

impl CacheApbPostwEnVal {
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

    /// Read **REG_CACHE_APB_POSTW_EN** `[0:0]`
    ///
    /// cache apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
    #[inline(always)]
    pub const fn reg_cache_apb_postw_en(&self) -> bool {
        (self.bits & CacheApbPostwEn::REG_CACHE_APB_POSTW_EN_MASK) != 0
    }

    /// Set **REG_CACHE_APB_POSTW_EN** `[0:0]`
    ///
    /// cache apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
    #[inline(always)]
    pub const fn with_reg_cache_apb_postw_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CacheApbPostwEn::REG_CACHE_APB_POSTW_EN_MASK
            } else {
                self.bits & !CacheApbPostwEn::REG_CACHE_APB_POSTW_EN_MASK
            },
        }
    }
}

impl Default for CacheApbPostwEnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CacheApbPostwEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CacheApbPostwEnVal {
        CacheApbPostwEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CacheApbPostwEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CacheApbPostwEnVal) -> CacheApbPostwEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L2_MEM_SUBSIZE Register
///
/// NA
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **REG_L2_MEM_SUB_BLKSIZE** `[1:0]` (RW)
///
///   l2mem sub block size 00=>32 01=>64 10=>128 11=>256
#[repr(transparent)]
pub struct L2MemSubsize {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemSubsize {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemSubsize {
    // ======== Field Constants ========

    /// **REG_L2_MEM_SUB_BLKSIZE**
    ///
    /// l2mem sub block size 00=>32 01=>64 10=>128 11=>256
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_L2_MEM_SUB_BLKSIZE_OFFSET: u32 = 0;
    pub const REG_L2_MEM_SUB_BLKSIZE_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_L2_MEM_SUB_BLKSIZE** `[1:0]` (RW)
    ///
    /// l2mem sub block size 00=>32 01=>64 10=>128 11=>256
    #[inline(always)]
    pub fn reg_l2_mem_sub_blksize(&self) -> u32 {
        (self.value.read_bits() & Self::REG_L2_MEM_SUB_BLKSIZE_MASK)
            >> Self::REG_L2_MEM_SUB_BLKSIZE_OFFSET
    }

    /// Write **REG_L2_MEM_SUB_BLKSIZE** `[1:0]` (RW)
    ///
    /// l2mem sub block size 00=>32 01=>64 10=>128 11=>256
    #[inline(always)]
    pub fn write_reg_l2_mem_sub_blksize(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_L2_MEM_SUB_BLKSIZE_MASK)
                | ((value << Self::REG_L2_MEM_SUB_BLKSIZE_OFFSET)
                    & Self::REG_L2_MEM_SUB_BLKSIZE_MASK)
        });
    }
}

/// L2_MEM_SUBSIZE value type for batch read/modify/write operations
///
/// Use `l2memsubsize.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memsubsize.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemSubsizeVal {
    bits: u32,
}

impl L2MemSubsizeVal {
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

    /// Read **REG_L2_MEM_SUB_BLKSIZE** `[1:0]`
    ///
    /// l2mem sub block size 00=>32 01=>64 10=>128 11=>256
    #[inline(always)]
    pub const fn reg_l2_mem_sub_blksize(&self) -> u32 {
        (self.bits & L2MemSubsize::REG_L2_MEM_SUB_BLKSIZE_MASK)
            >> L2MemSubsize::REG_L2_MEM_SUB_BLKSIZE_OFFSET
    }

    /// Set **REG_L2_MEM_SUB_BLKSIZE** `[1:0]`
    ///
    /// l2mem sub block size 00=>32 01=>64 10=>128 11=>256
    #[inline(always)]
    pub const fn with_reg_l2_mem_sub_blksize(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !L2MemSubsize::REG_L2_MEM_SUB_BLKSIZE_MASK)
                | ((value << L2MemSubsize::REG_L2_MEM_SUB_BLKSIZE_OFFSET)
                    & L2MemSubsize::REG_L2_MEM_SUB_BLKSIZE_MASK),
        }
    }
}

impl Default for L2MemSubsizeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemSubsize {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemSubsizeVal {
        L2MemSubsizeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2MemSubsizeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2MemSubsizeVal) -> L2MemSubsizeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L2_MEM_INT_RAW Register
///
/// NA
///
/// Address offset: 0x009c
///
/// ## Fields
///
/// - **REG_L2_MEM_ERR_RESP_INT_RAW** `[2]` (RW)
///
///   intr triggered when err response occurs
///
/// - **REG_L2_MEM_EXCEED_ADDR_INT_RAW** `[1]` (RW)
///
///   intr triggered when access addr exceeds 0xff9ffff at bypass mode or exceeds 0xff80000 at l2cache 128kb mode or exceeds 0xff60000 at l2cache 256kb mode
///
/// - **REG_L2_MEM_ECC_ERR_INT_RAW** `[0]` (RW)
///
///   intr triggered when two bit error detected and corrected from ecc
#[repr(transparent)]
pub struct L2MemIntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemIntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemIntRaw {
    // ======== Field Constants ========

    /// **REG_L2_MEM_ERR_RESP_INT_RAW**
    ///
    /// intr triggered when err response occurs
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_L2_MEM_ERR_RESP_INT_RAW_OFFSET: u32 = 2;
    pub const REG_L2_MEM_ERR_RESP_INT_RAW_MASK: u32 = 0x00000004;
    pub const REG_L2_MEM_ERR_RESP_INT_RAW: u32 = 1 << 2;

    /// **REG_L2_MEM_EXCEED_ADDR_INT_RAW**
    ///
    /// intr triggered when access addr exceeds 0xff9ffff at bypass mode or exceeds 0xff80000 at l2cache 128kb mode or exceeds 0xff60000 at l2cache 256kb mode
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_L2_MEM_EXCEED_ADDR_INT_RAW_OFFSET: u32 = 1;
    pub const REG_L2_MEM_EXCEED_ADDR_INT_RAW_MASK: u32 = 0x00000002;
    pub const REG_L2_MEM_EXCEED_ADDR_INT_RAW: u32 = 1 << 1;

    /// **REG_L2_MEM_ECC_ERR_INT_RAW**
    ///
    /// intr triggered when two bit error detected and corrected from ecc
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_L2_MEM_ECC_ERR_INT_RAW_OFFSET: u32 = 0;
    pub const REG_L2_MEM_ECC_ERR_INT_RAW_MASK: u32 = 0x00000001;
    pub const REG_L2_MEM_ECC_ERR_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_L2_MEM_ERR_RESP_INT_RAW** `[2]` (RW)
    ///
    /// intr triggered when err response occurs
    #[inline(always)]
    pub fn reg_l2_mem_err_resp_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_ERR_RESP_INT_RAW_MASK) != 0
    }

    /// Set REG_L2_MEM_ERR_RESP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_err_resp_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_ERR_RESP_INT_RAW_MASK);
    }

    /// Clear REG_L2_MEM_ERR_RESP_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_err_resp_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_ERR_RESP_INT_RAW_MASK);
    }

    /// **REG_L2_MEM_EXCEED_ADDR_INT_RAW** `[1]` (RW)
    ///
    /// intr triggered when access addr exceeds 0xff9ffff at bypass mode or exceeds 0xff80000 at l2cache 128kb mode or exceeds 0xff60000 at l2cache 256kb mode
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_EXCEED_ADDR_INT_RAW_MASK) != 0
    }

    /// Set REG_L2_MEM_EXCEED_ADDR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_exceed_addr_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_EXCEED_ADDR_INT_RAW_MASK);
    }

    /// Clear REG_L2_MEM_EXCEED_ADDR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_exceed_addr_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_EXCEED_ADDR_INT_RAW_MASK);
    }

    /// **REG_L2_MEM_ECC_ERR_INT_RAW** `[0]` (RW)
    ///
    /// intr triggered when two bit error detected and corrected from ecc
    #[inline(always)]
    pub fn reg_l2_mem_ecc_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_ECC_ERR_INT_RAW_MASK) != 0
    }

    /// Set REG_L2_MEM_ECC_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_ecc_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_ECC_ERR_INT_RAW_MASK);
    }

    /// Clear REG_L2_MEM_ECC_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_ecc_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_ECC_ERR_INT_RAW_MASK);
    }
}

/// L2_MEM_INT_RAW value type for batch read/modify/write operations
///
/// Use `l2memintraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memintraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemIntRawVal {
    bits: u32,
}

impl L2MemIntRawVal {
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

    /// Read **REG_L2_MEM_ERR_RESP_INT_RAW** `[2:2]`
    ///
    /// intr triggered when err response occurs
    #[inline(always)]
    pub const fn reg_l2_mem_err_resp_int_raw(&self) -> bool {
        (self.bits & L2MemIntRaw::REG_L2_MEM_ERR_RESP_INT_RAW_MASK) != 0
    }

    /// Read **REG_L2_MEM_EXCEED_ADDR_INT_RAW** `[1:1]`
    ///
    /// intr triggered when access addr exceeds 0xff9ffff at bypass mode or exceeds 0xff80000 at l2cache 128kb mode or exceeds 0xff60000 at l2cache 256kb mode
    #[inline(always)]
    pub const fn reg_l2_mem_exceed_addr_int_raw(&self) -> bool {
        (self.bits & L2MemIntRaw::REG_L2_MEM_EXCEED_ADDR_INT_RAW_MASK) != 0
    }

    /// Read **REG_L2_MEM_ECC_ERR_INT_RAW** `[0:0]`
    ///
    /// intr triggered when two bit error detected and corrected from ecc
    #[inline(always)]
    pub const fn reg_l2_mem_ecc_err_int_raw(&self) -> bool {
        (self.bits & L2MemIntRaw::REG_L2_MEM_ECC_ERR_INT_RAW_MASK) != 0
    }

    /// Set **REG_L2_MEM_ERR_RESP_INT_RAW** `[2:2]`
    ///
    /// intr triggered when err response occurs
    #[inline(always)]
    pub const fn with_reg_l2_mem_err_resp_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemIntRaw::REG_L2_MEM_ERR_RESP_INT_RAW_MASK
            } else {
                self.bits & !L2MemIntRaw::REG_L2_MEM_ERR_RESP_INT_RAW_MASK
            },
        }
    }

    /// Set **REG_L2_MEM_EXCEED_ADDR_INT_RAW** `[1:1]`
    ///
    /// intr triggered when access addr exceeds 0xff9ffff at bypass mode or exceeds 0xff80000 at l2cache 128kb mode or exceeds 0xff60000 at l2cache 256kb mode
    #[inline(always)]
    pub const fn with_reg_l2_mem_exceed_addr_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemIntRaw::REG_L2_MEM_EXCEED_ADDR_INT_RAW_MASK
            } else {
                self.bits & !L2MemIntRaw::REG_L2_MEM_EXCEED_ADDR_INT_RAW_MASK
            },
        }
    }

    /// Set **REG_L2_MEM_ECC_ERR_INT_RAW** `[0:0]`
    ///
    /// intr triggered when two bit error detected and corrected from ecc
    #[inline(always)]
    pub const fn with_reg_l2_mem_ecc_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemIntRaw::REG_L2_MEM_ECC_ERR_INT_RAW_MASK
            } else {
                self.bits & !L2MemIntRaw::REG_L2_MEM_ECC_ERR_INT_RAW_MASK
            },
        }
    }
}

impl Default for L2MemIntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemIntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemIntRawVal {
        L2MemIntRawVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2MemIntRawVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2MemIntRawVal) -> L2MemIntRawVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L2_MEM_INT_ST Register
///
/// NA
///
/// Address offset: 0x00a0
///
/// ## Fields
///
/// - **REG_L2_MEM_ERR_RESP_INT_ST** `[2]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_EXCEED_ADDR_INT_ST** `[1]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_ECC_ERR_INT_ST** `[0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct L2MemIntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemIntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemIntSt {
    // ======== Field Constants ========

    /// **REG_L2_MEM_ERR_RESP_INT_ST**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_L2_MEM_ERR_RESP_INT_ST_OFFSET: u32 = 2;
    pub const REG_L2_MEM_ERR_RESP_INT_ST_MASK: u32 = 0x00000004;
    pub const REG_L2_MEM_ERR_RESP_INT_ST: u32 = 1 << 2;

    /// **REG_L2_MEM_EXCEED_ADDR_INT_ST**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_L2_MEM_EXCEED_ADDR_INT_ST_OFFSET: u32 = 1;
    pub const REG_L2_MEM_EXCEED_ADDR_INT_ST_MASK: u32 = 0x00000002;
    pub const REG_L2_MEM_EXCEED_ADDR_INT_ST: u32 = 1 << 1;

    /// **REG_L2_MEM_ECC_ERR_INT_ST**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_L2_MEM_ECC_ERR_INT_ST_OFFSET: u32 = 0;
    pub const REG_L2_MEM_ECC_ERR_INT_ST_MASK: u32 = 0x00000001;
    pub const REG_L2_MEM_ECC_ERR_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_L2_MEM_ERR_RESP_INT_ST** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_err_resp_int_st(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_ERR_RESP_INT_ST_MASK) != 0
    }

    /// **REG_L2_MEM_EXCEED_ADDR_INT_ST** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_st(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_EXCEED_ADDR_INT_ST_MASK) != 0
    }

    /// **REG_L2_MEM_ECC_ERR_INT_ST** `[0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_ecc_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_ECC_ERR_INT_ST_MASK) != 0
    }
}

/// L2_MEM_INT_ST value type for batch read/modify/write operations
///
/// Use `l2memintst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memintst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemIntStVal {
    bits: u32,
}

impl L2MemIntStVal {
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

    /// Read **REG_L2_MEM_ERR_RESP_INT_ST** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_err_resp_int_st(&self) -> bool {
        (self.bits & L2MemIntSt::REG_L2_MEM_ERR_RESP_INT_ST_MASK) != 0
    }

    /// Read **REG_L2_MEM_EXCEED_ADDR_INT_ST** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_exceed_addr_int_st(&self) -> bool {
        (self.bits & L2MemIntSt::REG_L2_MEM_EXCEED_ADDR_INT_ST_MASK) != 0
    }

    /// Read **REG_L2_MEM_ECC_ERR_INT_ST** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_ecc_err_int_st(&self) -> bool {
        (self.bits & L2MemIntSt::REG_L2_MEM_ECC_ERR_INT_ST_MASK) != 0
    }
}

impl Default for L2MemIntStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemIntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemIntStVal {
        L2MemIntStVal::from_bits(self.value.read_bits())
    }
}

/// L2_MEM_INT_ENA Register
///
/// NA
///
/// Address offset: 0x00a4
///
/// ## Fields
///
/// - **REG_L2_MEM_ERR_RESP_INT_ENA** `[2]` (RW)
///
///   NA
///
/// - **REG_L2_MEM_EXCEED_ADDR_INT_ENA** `[1]` (RW)
///
///   NA
///
/// - **REG_L2_MEM_ECC_ERR_INT_ENA** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct L2MemIntEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemIntEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemIntEna {
    // ======== Field Constants ========

    /// **REG_L2_MEM_ERR_RESP_INT_ENA**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_L2_MEM_ERR_RESP_INT_ENA_OFFSET: u32 = 2;
    pub const REG_L2_MEM_ERR_RESP_INT_ENA_MASK: u32 = 0x00000004;
    pub const REG_L2_MEM_ERR_RESP_INT_ENA: u32 = 1 << 2;

    /// **REG_L2_MEM_EXCEED_ADDR_INT_ENA**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_L2_MEM_EXCEED_ADDR_INT_ENA_OFFSET: u32 = 1;
    pub const REG_L2_MEM_EXCEED_ADDR_INT_ENA_MASK: u32 = 0x00000002;
    pub const REG_L2_MEM_EXCEED_ADDR_INT_ENA: u32 = 1 << 1;

    /// **REG_L2_MEM_ECC_ERR_INT_ENA**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_L2_MEM_ECC_ERR_INT_ENA_OFFSET: u32 = 0;
    pub const REG_L2_MEM_ECC_ERR_INT_ENA_MASK: u32 = 0x00000001;
    pub const REG_L2_MEM_ECC_ERR_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_L2_MEM_ERR_RESP_INT_ENA** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_err_resp_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_ERR_RESP_INT_ENA_MASK) != 0
    }

    /// Set REG_L2_MEM_ERR_RESP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_err_resp_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_ERR_RESP_INT_ENA_MASK);
    }

    /// Clear REG_L2_MEM_ERR_RESP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_err_resp_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_ERR_RESP_INT_ENA_MASK);
    }

    /// **REG_L2_MEM_EXCEED_ADDR_INT_ENA** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_EXCEED_ADDR_INT_ENA_MASK) != 0
    }

    /// Set REG_L2_MEM_EXCEED_ADDR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_exceed_addr_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_EXCEED_ADDR_INT_ENA_MASK);
    }

    /// Clear REG_L2_MEM_EXCEED_ADDR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_exceed_addr_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_EXCEED_ADDR_INT_ENA_MASK);
    }

    /// **REG_L2_MEM_ECC_ERR_INT_ENA** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_ecc_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_ECC_ERR_INT_ENA_MASK) != 0
    }

    /// Set REG_L2_MEM_ECC_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_ecc_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_ECC_ERR_INT_ENA_MASK);
    }

    /// Clear REG_L2_MEM_ECC_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_ecc_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_ECC_ERR_INT_ENA_MASK);
    }
}

/// L2_MEM_INT_ENA value type for batch read/modify/write operations
///
/// Use `l2memintena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memintena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemIntEnaVal {
    bits: u32,
}

impl L2MemIntEnaVal {
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

    /// Read **REG_L2_MEM_ERR_RESP_INT_ENA** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_err_resp_int_ena(&self) -> bool {
        (self.bits & L2MemIntEna::REG_L2_MEM_ERR_RESP_INT_ENA_MASK) != 0
    }

    /// Read **REG_L2_MEM_EXCEED_ADDR_INT_ENA** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_exceed_addr_int_ena(&self) -> bool {
        (self.bits & L2MemIntEna::REG_L2_MEM_EXCEED_ADDR_INT_ENA_MASK) != 0
    }

    /// Read **REG_L2_MEM_ECC_ERR_INT_ENA** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_ecc_err_int_ena(&self) -> bool {
        (self.bits & L2MemIntEna::REG_L2_MEM_ECC_ERR_INT_ENA_MASK) != 0
    }

    /// Set **REG_L2_MEM_ERR_RESP_INT_ENA** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_err_resp_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemIntEna::REG_L2_MEM_ERR_RESP_INT_ENA_MASK
            } else {
                self.bits & !L2MemIntEna::REG_L2_MEM_ERR_RESP_INT_ENA_MASK
            },
        }
    }

    /// Set **REG_L2_MEM_EXCEED_ADDR_INT_ENA** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_exceed_addr_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemIntEna::REG_L2_MEM_EXCEED_ADDR_INT_ENA_MASK
            } else {
                self.bits & !L2MemIntEna::REG_L2_MEM_EXCEED_ADDR_INT_ENA_MASK
            },
        }
    }

    /// Set **REG_L2_MEM_ECC_ERR_INT_ENA** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_ecc_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemIntEna::REG_L2_MEM_ECC_ERR_INT_ENA_MASK
            } else {
                self.bits & !L2MemIntEna::REG_L2_MEM_ECC_ERR_INT_ENA_MASK
            },
        }
    }
}

impl Default for L2MemIntEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemIntEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemIntEnaVal {
        L2MemIntEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2MemIntEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2MemIntEnaVal) -> L2MemIntEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L2_MEM_INT_CLR Register
///
/// NA
///
/// Address offset: 0x00a8
///
/// ## Fields
///
/// - **REG_L2_MEM_ERR_RESP_INT_CLR** `[2]` (WO)
///
///   NA
///
/// - **REG_L2_MEM_EXCEED_ADDR_INT_CLR** `[1]` (WO)
///
///   NA
///
/// - **REG_L2_MEM_ECC_ERR_INT_CLR** `[0]` (WO)
///
///   NA
#[repr(transparent)]
pub struct L2MemIntClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemIntClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemIntClr {
    // ======== Field Constants ========

    /// **REG_L2_MEM_ERR_RESP_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_L2_MEM_ERR_RESP_INT_CLR_OFFSET: u32 = 2;
    pub const REG_L2_MEM_ERR_RESP_INT_CLR_MASK: u32 = 0x00000004;
    pub const REG_L2_MEM_ERR_RESP_INT_CLR: u32 = 1 << 2;

    /// **REG_L2_MEM_EXCEED_ADDR_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_L2_MEM_EXCEED_ADDR_INT_CLR_OFFSET: u32 = 1;
    pub const REG_L2_MEM_EXCEED_ADDR_INT_CLR_MASK: u32 = 0x00000002;
    pub const REG_L2_MEM_EXCEED_ADDR_INT_CLR: u32 = 1 << 1;

    /// **REG_L2_MEM_ECC_ERR_INT_CLR**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_L2_MEM_ECC_ERR_INT_CLR_OFFSET: u32 = 0;
    pub const REG_L2_MEM_ECC_ERR_INT_CLR_MASK: u32 = 0x00000001;
    pub const REG_L2_MEM_ECC_ERR_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **REG_L2_MEM_ERR_RESP_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_reg_l2_mem_err_resp_int_clr(&self) {
        self.value
            .write_bits(Self::REG_L2_MEM_ERR_RESP_INT_CLR_MASK);
    }

    /// Trigger **REG_L2_MEM_EXCEED_ADDR_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_reg_l2_mem_exceed_addr_int_clr(&self) {
        self.value
            .write_bits(Self::REG_L2_MEM_EXCEED_ADDR_INT_CLR_MASK);
    }

    /// Trigger **REG_L2_MEM_ECC_ERR_INT_CLR** (WO)
    ///
    /// NA
    #[inline(always)]
    pub fn trigger_reg_l2_mem_ecc_err_int_clr(&self) {
        self.value.write_bits(Self::REG_L2_MEM_ECC_ERR_INT_CLR_MASK);
    }
}

/// L2_MEM_INT_CLR value type for batch read/modify/write operations
///
/// Use `l2memintclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memintclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemIntClrVal {
    bits: u32,
}

impl L2MemIntClrVal {
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

    /// Set **REG_L2_MEM_ERR_RESP_INT_CLR** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_err_resp_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemIntClr::REG_L2_MEM_ERR_RESP_INT_CLR_MASK
            } else {
                self.bits & !L2MemIntClr::REG_L2_MEM_ERR_RESP_INT_CLR_MASK
            },
        }
    }

    /// Set **REG_L2_MEM_EXCEED_ADDR_INT_CLR** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_exceed_addr_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemIntClr::REG_L2_MEM_EXCEED_ADDR_INT_CLR_MASK
            } else {
                self.bits & !L2MemIntClr::REG_L2_MEM_EXCEED_ADDR_INT_CLR_MASK
            },
        }
    }

    /// Set **REG_L2_MEM_ECC_ERR_INT_CLR** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_ecc_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemIntClr::REG_L2_MEM_ECC_ERR_INT_CLR_MASK
            } else {
                self.bits & !L2MemIntClr::REG_L2_MEM_ECC_ERR_INT_CLR_MASK
            },
        }
    }
}

impl Default for L2MemIntClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemIntClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2MemIntClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// L2_MEM_L2_RAM_ECC Register
///
/// NA
///
/// Address offset: 0x00ac
///
/// ## Fields
///
/// - **REG_L2_RAM_UNIT5_ECC_EN** `[5]` (RW)
///
///   NA
///
/// - **REG_L2_RAM_UNIT4_ECC_EN** `[4]` (RW)
///
///   NA
///
/// - **REG_L2_RAM_UNIT3_ECC_EN** `[3]` (RW)
///
///   NA
///
/// - **REG_L2_RAM_UNIT2_ECC_EN** `[2]` (RW)
///
///   NA
///
/// - **REG_L2_RAM_UNIT1_ECC_EN** `[1]` (RW)
///
///   NA
///
/// - **REG_L2_RAM_UNIT0_ECC_EN** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct L2MemL2RamEcc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemL2RamEcc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemL2RamEcc {
    // ======== Field Constants ========

    /// **REG_L2_RAM_UNIT5_ECC_EN**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_L2_RAM_UNIT5_ECC_EN_OFFSET: u32 = 5;
    pub const REG_L2_RAM_UNIT5_ECC_EN_MASK: u32 = 0x00000020;
    pub const REG_L2_RAM_UNIT5_ECC_EN: u32 = 1 << 5;

    /// **REG_L2_RAM_UNIT4_ECC_EN**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_L2_RAM_UNIT4_ECC_EN_OFFSET: u32 = 4;
    pub const REG_L2_RAM_UNIT4_ECC_EN_MASK: u32 = 0x00000010;
    pub const REG_L2_RAM_UNIT4_ECC_EN: u32 = 1 << 4;

    /// **REG_L2_RAM_UNIT3_ECC_EN**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_L2_RAM_UNIT3_ECC_EN_OFFSET: u32 = 3;
    pub const REG_L2_RAM_UNIT3_ECC_EN_MASK: u32 = 0x00000008;
    pub const REG_L2_RAM_UNIT3_ECC_EN: u32 = 1 << 3;

    /// **REG_L2_RAM_UNIT2_ECC_EN**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_L2_RAM_UNIT2_ECC_EN_OFFSET: u32 = 2;
    pub const REG_L2_RAM_UNIT2_ECC_EN_MASK: u32 = 0x00000004;
    pub const REG_L2_RAM_UNIT2_ECC_EN: u32 = 1 << 2;

    /// **REG_L2_RAM_UNIT1_ECC_EN**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_L2_RAM_UNIT1_ECC_EN_OFFSET: u32 = 1;
    pub const REG_L2_RAM_UNIT1_ECC_EN_MASK: u32 = 0x00000002;
    pub const REG_L2_RAM_UNIT1_ECC_EN: u32 = 1 << 1;

    /// **REG_L2_RAM_UNIT0_ECC_EN**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_L2_RAM_UNIT0_ECC_EN_OFFSET: u32 = 0;
    pub const REG_L2_RAM_UNIT0_ECC_EN_MASK: u32 = 0x00000001;
    pub const REG_L2_RAM_UNIT0_ECC_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_L2_RAM_UNIT5_ECC_EN** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_ram_unit5_ecc_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_RAM_UNIT5_ECC_EN_MASK) != 0
    }

    /// Set REG_L2_RAM_UNIT5_ECC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_ram_unit5_ecc_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_RAM_UNIT5_ECC_EN_MASK);
    }

    /// Clear REG_L2_RAM_UNIT5_ECC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_ram_unit5_ecc_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_RAM_UNIT5_ECC_EN_MASK);
    }

    /// **REG_L2_RAM_UNIT4_ECC_EN** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_ram_unit4_ecc_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_RAM_UNIT4_ECC_EN_MASK) != 0
    }

    /// Set REG_L2_RAM_UNIT4_ECC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_ram_unit4_ecc_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_RAM_UNIT4_ECC_EN_MASK);
    }

    /// Clear REG_L2_RAM_UNIT4_ECC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_ram_unit4_ecc_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_RAM_UNIT4_ECC_EN_MASK);
    }

    /// **REG_L2_RAM_UNIT3_ECC_EN** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_ram_unit3_ecc_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_RAM_UNIT3_ECC_EN_MASK) != 0
    }

    /// Set REG_L2_RAM_UNIT3_ECC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_ram_unit3_ecc_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_RAM_UNIT3_ECC_EN_MASK);
    }

    /// Clear REG_L2_RAM_UNIT3_ECC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_ram_unit3_ecc_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_RAM_UNIT3_ECC_EN_MASK);
    }

    /// **REG_L2_RAM_UNIT2_ECC_EN** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_ram_unit2_ecc_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_RAM_UNIT2_ECC_EN_MASK) != 0
    }

    /// Set REG_L2_RAM_UNIT2_ECC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_ram_unit2_ecc_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_RAM_UNIT2_ECC_EN_MASK);
    }

    /// Clear REG_L2_RAM_UNIT2_ECC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_ram_unit2_ecc_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_RAM_UNIT2_ECC_EN_MASK);
    }

    /// **REG_L2_RAM_UNIT1_ECC_EN** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_ram_unit1_ecc_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_RAM_UNIT1_ECC_EN_MASK) != 0
    }

    /// Set REG_L2_RAM_UNIT1_ECC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_ram_unit1_ecc_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_RAM_UNIT1_ECC_EN_MASK);
    }

    /// Clear REG_L2_RAM_UNIT1_ECC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_ram_unit1_ecc_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_RAM_UNIT1_ECC_EN_MASK);
    }

    /// **REG_L2_RAM_UNIT0_ECC_EN** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_ram_unit0_ecc_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_RAM_UNIT0_ECC_EN_MASK) != 0
    }

    /// Set REG_L2_RAM_UNIT0_ECC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_ram_unit0_ecc_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_RAM_UNIT0_ECC_EN_MASK);
    }

    /// Clear REG_L2_RAM_UNIT0_ECC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_ram_unit0_ecc_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_RAM_UNIT0_ECC_EN_MASK);
    }
}

/// L2_MEM_L2_RAM_ECC value type for batch read/modify/write operations
///
/// Use `l2meml2ramecc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2meml2ramecc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemL2RamEccVal {
    bits: u32,
}

impl L2MemL2RamEccVal {
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

    /// Read **REG_L2_RAM_UNIT5_ECC_EN** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_ram_unit5_ecc_en(&self) -> bool {
        (self.bits & L2MemL2RamEcc::REG_L2_RAM_UNIT5_ECC_EN_MASK) != 0
    }

    /// Read **REG_L2_RAM_UNIT4_ECC_EN** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_ram_unit4_ecc_en(&self) -> bool {
        (self.bits & L2MemL2RamEcc::REG_L2_RAM_UNIT4_ECC_EN_MASK) != 0
    }

    /// Read **REG_L2_RAM_UNIT3_ECC_EN** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_ram_unit3_ecc_en(&self) -> bool {
        (self.bits & L2MemL2RamEcc::REG_L2_RAM_UNIT3_ECC_EN_MASK) != 0
    }

    /// Read **REG_L2_RAM_UNIT2_ECC_EN** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_ram_unit2_ecc_en(&self) -> bool {
        (self.bits & L2MemL2RamEcc::REG_L2_RAM_UNIT2_ECC_EN_MASK) != 0
    }

    /// Read **REG_L2_RAM_UNIT1_ECC_EN** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_ram_unit1_ecc_en(&self) -> bool {
        (self.bits & L2MemL2RamEcc::REG_L2_RAM_UNIT1_ECC_EN_MASK) != 0
    }

    /// Read **REG_L2_RAM_UNIT0_ECC_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_ram_unit0_ecc_en(&self) -> bool {
        (self.bits & L2MemL2RamEcc::REG_L2_RAM_UNIT0_ECC_EN_MASK) != 0
    }

    /// Set **REG_L2_RAM_UNIT5_ECC_EN** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_ram_unit5_ecc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemL2RamEcc::REG_L2_RAM_UNIT5_ECC_EN_MASK
            } else {
                self.bits & !L2MemL2RamEcc::REG_L2_RAM_UNIT5_ECC_EN_MASK
            },
        }
    }

    /// Set **REG_L2_RAM_UNIT4_ECC_EN** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_ram_unit4_ecc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemL2RamEcc::REG_L2_RAM_UNIT4_ECC_EN_MASK
            } else {
                self.bits & !L2MemL2RamEcc::REG_L2_RAM_UNIT4_ECC_EN_MASK
            },
        }
    }

    /// Set **REG_L2_RAM_UNIT3_ECC_EN** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_ram_unit3_ecc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemL2RamEcc::REG_L2_RAM_UNIT3_ECC_EN_MASK
            } else {
                self.bits & !L2MemL2RamEcc::REG_L2_RAM_UNIT3_ECC_EN_MASK
            },
        }
    }

    /// Set **REG_L2_RAM_UNIT2_ECC_EN** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_ram_unit2_ecc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemL2RamEcc::REG_L2_RAM_UNIT2_ECC_EN_MASK
            } else {
                self.bits & !L2MemL2RamEcc::REG_L2_RAM_UNIT2_ECC_EN_MASK
            },
        }
    }

    /// Set **REG_L2_RAM_UNIT1_ECC_EN** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_ram_unit1_ecc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemL2RamEcc::REG_L2_RAM_UNIT1_ECC_EN_MASK
            } else {
                self.bits & !L2MemL2RamEcc::REG_L2_RAM_UNIT1_ECC_EN_MASK
            },
        }
    }

    /// Set **REG_L2_RAM_UNIT0_ECC_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_ram_unit0_ecc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemL2RamEcc::REG_L2_RAM_UNIT0_ECC_EN_MASK
            } else {
                self.bits & !L2MemL2RamEcc::REG_L2_RAM_UNIT0_ECC_EN_MASK
            },
        }
    }
}

impl Default for L2MemL2RamEccVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemL2RamEcc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemL2RamEccVal {
        L2MemL2RamEccVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2MemL2RamEccVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2MemL2RamEccVal) -> L2MemL2RamEccVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L2_MEM_INT_RECORD0 Register
///
/// NA
///
/// Address offset: 0x00b0
///
/// ## Fields
///
/// - **REG_L2_MEM_EXCEED_ADDR_INT_MASTER** `[24:22]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_EXCEED_ADDR_INT_WE** `[21]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_EXCEED_ADDR_INT_ADDR** `[20:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct L2MemIntRecord0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemIntRecord0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemIntRecord0 {
    // ======== Field Constants ========

    /// **REG_L2_MEM_EXCEED_ADDR_INT_MASTER**
    ///
    /// NA
    ///
    /// Bits `[24:22]`, mask `0x01c00000`
    pub const REG_L2_MEM_EXCEED_ADDR_INT_MASTER_OFFSET: u32 = 22;
    pub const REG_L2_MEM_EXCEED_ADDR_INT_MASTER_MASK: u32 = 0x01c00000;

    /// **REG_L2_MEM_EXCEED_ADDR_INT_WE**
    ///
    /// NA
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const REG_L2_MEM_EXCEED_ADDR_INT_WE_OFFSET: u32 = 21;
    pub const REG_L2_MEM_EXCEED_ADDR_INT_WE_MASK: u32 = 0x00200000;
    pub const REG_L2_MEM_EXCEED_ADDR_INT_WE: u32 = 1 << 21;

    /// **REG_L2_MEM_EXCEED_ADDR_INT_ADDR**
    ///
    /// NA
    ///
    /// Bits `[20:0]`, mask `0x001fffff`
    pub const REG_L2_MEM_EXCEED_ADDR_INT_ADDR_OFFSET: u32 = 0;
    pub const REG_L2_MEM_EXCEED_ADDR_INT_ADDR_MASK: u32 = 0x001fffff;

    // ======== Field Accessors ========

    /// **REG_L2_MEM_EXCEED_ADDR_INT_MASTER** `[24:22]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_master(&self) -> u32 {
        (self.value.read_bits() & Self::REG_L2_MEM_EXCEED_ADDR_INT_MASTER_MASK)
            >> Self::REG_L2_MEM_EXCEED_ADDR_INT_MASTER_OFFSET
    }

    /// **REG_L2_MEM_EXCEED_ADDR_INT_WE** `[21]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_we(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_EXCEED_ADDR_INT_WE_MASK) != 0
    }

    /// **REG_L2_MEM_EXCEED_ADDR_INT_ADDR** `[20:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_L2_MEM_EXCEED_ADDR_INT_ADDR_MASK)
            >> Self::REG_L2_MEM_EXCEED_ADDR_INT_ADDR_OFFSET
    }
}

/// L2_MEM_INT_RECORD0 value type for batch read/modify/write operations
///
/// Use `l2memintrecord0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memintrecord0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemIntRecord0Val {
    bits: u32,
}

impl L2MemIntRecord0Val {
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

    /// Read **REG_L2_MEM_EXCEED_ADDR_INT_MASTER** `[24:22]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_exceed_addr_int_master(&self) -> u32 {
        (self.bits & L2MemIntRecord0::REG_L2_MEM_EXCEED_ADDR_INT_MASTER_MASK)
            >> L2MemIntRecord0::REG_L2_MEM_EXCEED_ADDR_INT_MASTER_OFFSET
    }

    /// Read **REG_L2_MEM_EXCEED_ADDR_INT_WE** `[21:21]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_exceed_addr_int_we(&self) -> bool {
        (self.bits & L2MemIntRecord0::REG_L2_MEM_EXCEED_ADDR_INT_WE_MASK) != 0
    }

    /// Read **REG_L2_MEM_EXCEED_ADDR_INT_ADDR** `[20:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_exceed_addr_int_addr(&self) -> u32 {
        (self.bits & L2MemIntRecord0::REG_L2_MEM_EXCEED_ADDR_INT_ADDR_MASK)
            >> L2MemIntRecord0::REG_L2_MEM_EXCEED_ADDR_INT_ADDR_OFFSET
    }
}

impl Default for L2MemIntRecord0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemIntRecord0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemIntRecord0Val {
        L2MemIntRecord0Val::from_bits(self.value.read_bits())
    }
}

/// L2_MEM_INT_RECORD1 Register
///
/// NA
///
/// Address offset: 0x00b4
///
/// ## Fields
///
/// - **REG_L2_CACHE_ERR_BANK** `[26]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_ECC_ERR_BIT** `[25:17]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_ECC_TWO_BIT_ERR** `[16]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_ECC_ONE_BIT_ERR** `[15]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_ECC_ERR_INT_ADDR** `[14:0]` (RO)
///
///   NA
#[repr(transparent)]
pub struct L2MemIntRecord1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemIntRecord1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemIntRecord1 {
    // ======== Field Constants ========

    /// **REG_L2_CACHE_ERR_BANK**
    ///
    /// NA
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const REG_L2_CACHE_ERR_BANK_OFFSET: u32 = 26;
    pub const REG_L2_CACHE_ERR_BANK_MASK: u32 = 0x04000000;
    pub const REG_L2_CACHE_ERR_BANK: u32 = 1 << 26;

    /// **REG_L2_MEM_ECC_ERR_BIT**
    ///
    /// NA
    ///
    /// Bits `[25:17]`, mask `0x03fe0000`
    pub const REG_L2_MEM_ECC_ERR_BIT_OFFSET: u32 = 17;
    pub const REG_L2_MEM_ECC_ERR_BIT_MASK: u32 = 0x03fe0000;

    /// **REG_L2_MEM_ECC_TWO_BIT_ERR**
    ///
    /// NA
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const REG_L2_MEM_ECC_TWO_BIT_ERR_OFFSET: u32 = 16;
    pub const REG_L2_MEM_ECC_TWO_BIT_ERR_MASK: u32 = 0x00010000;
    pub const REG_L2_MEM_ECC_TWO_BIT_ERR: u32 = 1 << 16;

    /// **REG_L2_MEM_ECC_ONE_BIT_ERR**
    ///
    /// NA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const REG_L2_MEM_ECC_ONE_BIT_ERR_OFFSET: u32 = 15;
    pub const REG_L2_MEM_ECC_ONE_BIT_ERR_MASK: u32 = 0x00008000;
    pub const REG_L2_MEM_ECC_ONE_BIT_ERR: u32 = 1 << 15;

    /// **REG_L2_MEM_ECC_ERR_INT_ADDR**
    ///
    /// NA
    ///
    /// Bits `[14:0]`, mask `0x00007fff`
    pub const REG_L2_MEM_ECC_ERR_INT_ADDR_OFFSET: u32 = 0;
    pub const REG_L2_MEM_ECC_ERR_INT_ADDR_MASK: u32 = 0x00007fff;

    // ======== Field Accessors ========

    /// **REG_L2_CACHE_ERR_BANK** `[26]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_cache_err_bank(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_CACHE_ERR_BANK_MASK) != 0
    }

    /// **REG_L2_MEM_ECC_ERR_BIT** `[25:17]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_ecc_err_bit(&self) -> u32 {
        (self.value.read_bits() & Self::REG_L2_MEM_ECC_ERR_BIT_MASK)
            >> Self::REG_L2_MEM_ECC_ERR_BIT_OFFSET
    }

    /// **REG_L2_MEM_ECC_TWO_BIT_ERR** `[16]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_ecc_two_bit_err(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_ECC_TWO_BIT_ERR_MASK) != 0
    }

    /// **REG_L2_MEM_ECC_ONE_BIT_ERR** `[15]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_ecc_one_bit_err(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_ECC_ONE_BIT_ERR_MASK) != 0
    }

    /// **REG_L2_MEM_ECC_ERR_INT_ADDR** `[14:0]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_ecc_err_int_addr(&self) -> u32 {
        (self.value.read_bits() & Self::REG_L2_MEM_ECC_ERR_INT_ADDR_MASK)
            >> Self::REG_L2_MEM_ECC_ERR_INT_ADDR_OFFSET
    }
}

/// L2_MEM_INT_RECORD1 value type for batch read/modify/write operations
///
/// Use `l2memintrecord1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memintrecord1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemIntRecord1Val {
    bits: u32,
}

impl L2MemIntRecord1Val {
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

    /// Read **REG_L2_CACHE_ERR_BANK** `[26:26]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_cache_err_bank(&self) -> bool {
        (self.bits & L2MemIntRecord1::REG_L2_CACHE_ERR_BANK_MASK) != 0
    }

    /// Read **REG_L2_MEM_ECC_ERR_BIT** `[25:17]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_ecc_err_bit(&self) -> u32 {
        (self.bits & L2MemIntRecord1::REG_L2_MEM_ECC_ERR_BIT_MASK)
            >> L2MemIntRecord1::REG_L2_MEM_ECC_ERR_BIT_OFFSET
    }

    /// Read **REG_L2_MEM_ECC_TWO_BIT_ERR** `[16:16]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_ecc_two_bit_err(&self) -> bool {
        (self.bits & L2MemIntRecord1::REG_L2_MEM_ECC_TWO_BIT_ERR_MASK) != 0
    }

    /// Read **REG_L2_MEM_ECC_ONE_BIT_ERR** `[15:15]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_ecc_one_bit_err(&self) -> bool {
        (self.bits & L2MemIntRecord1::REG_L2_MEM_ECC_ONE_BIT_ERR_MASK) != 0
    }

    /// Read **REG_L2_MEM_ECC_ERR_INT_ADDR** `[14:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_ecc_err_int_addr(&self) -> u32 {
        (self.bits & L2MemIntRecord1::REG_L2_MEM_ECC_ERR_INT_ADDR_MASK)
            >> L2MemIntRecord1::REG_L2_MEM_ECC_ERR_INT_ADDR_OFFSET
    }
}

impl Default for L2MemIntRecord1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemIntRecord1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemIntRecord1Val {
        L2MemIntRecord1Val::from_bits(self.value.read_bits())
    }
}

/// L2_MEM_L2_CACHE_ECC Register
///
/// NA
///
/// Address offset: 0x00c4
///
/// ## Fields
///
/// - **REG_L2_CACHE_ECC_EN** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct L2MemL2CacheEcc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemL2CacheEcc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemL2CacheEcc {
    // ======== Field Constants ========

    /// **REG_L2_CACHE_ECC_EN**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_L2_CACHE_ECC_EN_OFFSET: u32 = 0;
    pub const REG_L2_CACHE_ECC_EN_MASK: u32 = 0x00000001;
    pub const REG_L2_CACHE_ECC_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_L2_CACHE_ECC_EN** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_cache_ecc_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_CACHE_ECC_EN_MASK) != 0
    }

    /// Set REG_L2_CACHE_ECC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_cache_ecc_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_CACHE_ECC_EN_MASK);
    }

    /// Clear REG_L2_CACHE_ECC_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_cache_ecc_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_CACHE_ECC_EN_MASK);
    }
}

/// L2_MEM_L2_CACHE_ECC value type for batch read/modify/write operations
///
/// Use `l2meml2cacheecc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2meml2cacheecc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemL2CacheEccVal {
    bits: u32,
}

impl L2MemL2CacheEccVal {
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

    /// Read **REG_L2_CACHE_ECC_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_cache_ecc_en(&self) -> bool {
        (self.bits & L2MemL2CacheEcc::REG_L2_CACHE_ECC_EN_MASK) != 0
    }

    /// Set **REG_L2_CACHE_ECC_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_cache_ecc_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemL2CacheEcc::REG_L2_CACHE_ECC_EN_MASK
            } else {
                self.bits & !L2MemL2CacheEcc::REG_L2_CACHE_ECC_EN_MASK
            },
        }
    }
}

impl Default for L2MemL2CacheEccVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemL2CacheEcc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemL2CacheEccVal {
        L2MemL2CacheEccVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2MemL2CacheEccVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2MemL2CacheEccVal) -> L2MemL2CacheEccVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L1CACHE_BUS0_ID Register
///
/// NA
///
/// Address offset: 0x00c8
///
/// ## Fields
///
/// - **REG_L1_CACHE_BUS0_ID** `[3:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct L1cacheBus0Id {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L1cacheBus0Id {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L1cacheBus0Id {
    // ======== Field Constants ========

    /// **REG_L1_CACHE_BUS0_ID**
    ///
    /// NA
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const REG_L1_CACHE_BUS0_ID_OFFSET: u32 = 0;
    pub const REG_L1_CACHE_BUS0_ID_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **REG_L1_CACHE_BUS0_ID** `[3:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l1_cache_bus0_id(&self) -> u32 {
        (self.value.read_bits() & Self::REG_L1_CACHE_BUS0_ID_MASK)
            >> Self::REG_L1_CACHE_BUS0_ID_OFFSET
    }

    /// Write **REG_L1_CACHE_BUS0_ID** `[3:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_l1_cache_bus0_id(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_L1_CACHE_BUS0_ID_MASK)
                | ((value << Self::REG_L1_CACHE_BUS0_ID_OFFSET) & Self::REG_L1_CACHE_BUS0_ID_MASK)
        });
    }
}

/// L1CACHE_BUS0_ID value type for batch read/modify/write operations
///
/// Use `l1cachebus0id.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l1cachebus0id.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L1cacheBus0IdVal {
    bits: u32,
}

impl L1cacheBus0IdVal {
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

    /// Read **REG_L1_CACHE_BUS0_ID** `[3:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l1_cache_bus0_id(&self) -> u32 {
        (self.bits & L1cacheBus0Id::REG_L1_CACHE_BUS0_ID_MASK)
            >> L1cacheBus0Id::REG_L1_CACHE_BUS0_ID_OFFSET
    }

    /// Set **REG_L1_CACHE_BUS0_ID** `[3:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l1_cache_bus0_id(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !L1cacheBus0Id::REG_L1_CACHE_BUS0_ID_MASK)
                | ((value << L1cacheBus0Id::REG_L1_CACHE_BUS0_ID_OFFSET)
                    & L1cacheBus0Id::REG_L1_CACHE_BUS0_ID_MASK),
        }
    }
}

impl Default for L1cacheBus0IdVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L1cacheBus0Id {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L1cacheBus0IdVal {
        L1cacheBus0IdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L1cacheBus0IdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L1cacheBus0IdVal) -> L1cacheBus0IdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L1CACHE_BUS1_ID Register
///
/// NA
///
/// Address offset: 0x00cc
///
/// ## Fields
///
/// - **REG_L1_CACHE_BUS1_ID** `[3:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct L1cacheBus1Id {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L1cacheBus1Id {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L1cacheBus1Id {
    // ======== Field Constants ========

    /// **REG_L1_CACHE_BUS1_ID**
    ///
    /// NA
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const REG_L1_CACHE_BUS1_ID_OFFSET: u32 = 0;
    pub const REG_L1_CACHE_BUS1_ID_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **REG_L1_CACHE_BUS1_ID** `[3:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l1_cache_bus1_id(&self) -> u32 {
        (self.value.read_bits() & Self::REG_L1_CACHE_BUS1_ID_MASK)
            >> Self::REG_L1_CACHE_BUS1_ID_OFFSET
    }

    /// Write **REG_L1_CACHE_BUS1_ID** `[3:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_l1_cache_bus1_id(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_L1_CACHE_BUS1_ID_MASK)
                | ((value << Self::REG_L1_CACHE_BUS1_ID_OFFSET) & Self::REG_L1_CACHE_BUS1_ID_MASK)
        });
    }
}

/// L1CACHE_BUS1_ID value type for batch read/modify/write operations
///
/// Use `l1cachebus1id.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l1cachebus1id.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L1cacheBus1IdVal {
    bits: u32,
}

impl L1cacheBus1IdVal {
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

    /// Read **REG_L1_CACHE_BUS1_ID** `[3:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l1_cache_bus1_id(&self) -> u32 {
        (self.bits & L1cacheBus1Id::REG_L1_CACHE_BUS1_ID_MASK)
            >> L1cacheBus1Id::REG_L1_CACHE_BUS1_ID_OFFSET
    }

    /// Set **REG_L1_CACHE_BUS1_ID** `[3:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l1_cache_bus1_id(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !L1cacheBus1Id::REG_L1_CACHE_BUS1_ID_MASK)
                | ((value << L1cacheBus1Id::REG_L1_CACHE_BUS1_ID_OFFSET)
                    & L1cacheBus1Id::REG_L1_CACHE_BUS1_ID_MASK),
        }
    }
}

impl Default for L1cacheBus1IdVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L1cacheBus1Id {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L1cacheBus1IdVal {
        L1cacheBus1IdVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L1cacheBus1IdVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L1cacheBus1IdVal) -> L1cacheBus1IdVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L2_MEM_RDN_ECO_CS Register
///
/// NA
///
/// Address offset: 0x00d8
///
/// ## Fields
///
/// - **REG_L2_MEM_RDN_ECO_RESULT** `[1]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_RDN_ECO_EN** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct L2MemRdnEcoCs {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemRdnEcoCs {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemRdnEcoCs {
    // ======== Field Constants ========

    /// **REG_L2_MEM_RDN_ECO_RESULT**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_L2_MEM_RDN_ECO_RESULT_OFFSET: u32 = 1;
    pub const REG_L2_MEM_RDN_ECO_RESULT_MASK: u32 = 0x00000002;
    pub const REG_L2_MEM_RDN_ECO_RESULT: u32 = 1 << 1;

    /// **REG_L2_MEM_RDN_ECO_EN**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_L2_MEM_RDN_ECO_EN_OFFSET: u32 = 0;
    pub const REG_L2_MEM_RDN_ECO_EN_MASK: u32 = 0x00000001;
    pub const REG_L2_MEM_RDN_ECO_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_L2_MEM_RDN_ECO_RESULT** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_rdn_eco_result(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_RDN_ECO_RESULT_MASK) != 0
    }

    /// **REG_L2_MEM_RDN_ECO_EN** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_rdn_eco_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_RDN_ECO_EN_MASK) != 0
    }

    /// Set REG_L2_MEM_RDN_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_rdn_eco_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_RDN_ECO_EN_MASK);
    }

    /// Clear REG_L2_MEM_RDN_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_rdn_eco_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_RDN_ECO_EN_MASK);
    }
}

/// L2_MEM_RDN_ECO_CS value type for batch read/modify/write operations
///
/// Use `l2memrdnecocs.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memrdnecocs.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemRdnEcoCsVal {
    bits: u32,
}

impl L2MemRdnEcoCsVal {
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

    /// Read **REG_L2_MEM_RDN_ECO_RESULT** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_rdn_eco_result(&self) -> bool {
        (self.bits & L2MemRdnEcoCs::REG_L2_MEM_RDN_ECO_RESULT_MASK) != 0
    }

    /// Read **REG_L2_MEM_RDN_ECO_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_rdn_eco_en(&self) -> bool {
        (self.bits & L2MemRdnEcoCs::REG_L2_MEM_RDN_ECO_EN_MASK) != 0
    }

    /// Set **REG_L2_MEM_RDN_ECO_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_rdn_eco_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemRdnEcoCs::REG_L2_MEM_RDN_ECO_EN_MASK
            } else {
                self.bits & !L2MemRdnEcoCs::REG_L2_MEM_RDN_ECO_EN_MASK
            },
        }
    }
}

impl Default for L2MemRdnEcoCsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemRdnEcoCs {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemRdnEcoCsVal {
        L2MemRdnEcoCsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2MemRdnEcoCsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2MemRdnEcoCsVal) -> L2MemRdnEcoCsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L2_MEM_RDN_ECO_LOW Register
///
/// NA
///
/// Address offset: 0x00dc
///
/// ## Fields
///
/// - **REG_L2_MEM_RDN_ECO_LOW** `[31:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct L2MemRdnEcoLow {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemRdnEcoLow {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemRdnEcoLow {
    // ======== Field Constants ========

    /// **REG_L2_MEM_RDN_ECO_LOW**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_L2_MEM_RDN_ECO_LOW_OFFSET: u32 = 0;
    pub const REG_L2_MEM_RDN_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_L2_MEM_RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_rdn_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::REG_L2_MEM_RDN_ECO_LOW_MASK)
            >> Self::REG_L2_MEM_RDN_ECO_LOW_OFFSET
    }

    /// Write **REG_L2_MEM_RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_l2_mem_rdn_eco_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_L2_MEM_RDN_ECO_LOW_MASK)
                | ((value << Self::REG_L2_MEM_RDN_ECO_LOW_OFFSET)
                    & Self::REG_L2_MEM_RDN_ECO_LOW_MASK)
        });
    }
}

/// L2_MEM_RDN_ECO_LOW value type for batch read/modify/write operations
///
/// Use `l2memrdnecolow.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memrdnecolow.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemRdnEcoLowVal {
    bits: u32,
}

impl L2MemRdnEcoLowVal {
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

    /// Read **REG_L2_MEM_RDN_ECO_LOW** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_rdn_eco_low(&self) -> u32 {
        (self.bits & L2MemRdnEcoLow::REG_L2_MEM_RDN_ECO_LOW_MASK)
            >> L2MemRdnEcoLow::REG_L2_MEM_RDN_ECO_LOW_OFFSET
    }

    /// Set **REG_L2_MEM_RDN_ECO_LOW** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_rdn_eco_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !L2MemRdnEcoLow::REG_L2_MEM_RDN_ECO_LOW_MASK)
                | ((value << L2MemRdnEcoLow::REG_L2_MEM_RDN_ECO_LOW_OFFSET)
                    & L2MemRdnEcoLow::REG_L2_MEM_RDN_ECO_LOW_MASK),
        }
    }
}

impl Default for L2MemRdnEcoLowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemRdnEcoLow {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemRdnEcoLowVal {
        L2MemRdnEcoLowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2MemRdnEcoLowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2MemRdnEcoLowVal) -> L2MemRdnEcoLowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L2_MEM_RDN_ECO_HIGH Register
///
/// NA
///
/// Address offset: 0x00e0
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **REG_L2_MEM_RDN_ECO_HIGH** `[31:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct L2MemRdnEcoHigh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemRdnEcoHigh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemRdnEcoHigh {
    // ======== Field Constants ========

    /// **REG_L2_MEM_RDN_ECO_HIGH**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_L2_MEM_RDN_ECO_HIGH_OFFSET: u32 = 0;
    pub const REG_L2_MEM_RDN_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_L2_MEM_RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_rdn_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::REG_L2_MEM_RDN_ECO_HIGH_MASK)
            >> Self::REG_L2_MEM_RDN_ECO_HIGH_OFFSET
    }

    /// Write **REG_L2_MEM_RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_l2_mem_rdn_eco_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_L2_MEM_RDN_ECO_HIGH_MASK)
                | ((value << Self::REG_L2_MEM_RDN_ECO_HIGH_OFFSET)
                    & Self::REG_L2_MEM_RDN_ECO_HIGH_MASK)
        });
    }
}

/// L2_MEM_RDN_ECO_HIGH value type for batch read/modify/write operations
///
/// Use `l2memrdnecohigh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memrdnecohigh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemRdnEcoHighVal {
    bits: u32,
}

impl L2MemRdnEcoHighVal {
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

    /// Read **REG_L2_MEM_RDN_ECO_HIGH** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_rdn_eco_high(&self) -> u32 {
        (self.bits & L2MemRdnEcoHigh::REG_L2_MEM_RDN_ECO_HIGH_MASK)
            >> L2MemRdnEcoHigh::REG_L2_MEM_RDN_ECO_HIGH_OFFSET
    }

    /// Set **REG_L2_MEM_RDN_ECO_HIGH** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_rdn_eco_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !L2MemRdnEcoHigh::REG_L2_MEM_RDN_ECO_HIGH_MASK)
                | ((value << L2MemRdnEcoHigh::REG_L2_MEM_RDN_ECO_HIGH_OFFSET)
                    & L2MemRdnEcoHigh::REG_L2_MEM_RDN_ECO_HIGH_MASK),
        }
    }
}

impl Default for L2MemRdnEcoHighVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl L2MemRdnEcoHigh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemRdnEcoHighVal {
        L2MemRdnEcoHighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2MemRdnEcoHighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2MemRdnEcoHighVal) -> L2MemRdnEcoHighVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TCM_RDN_ECO_CS Register
///
/// NA
///
/// Address offset: 0x00e4
///
/// ## Fields
///
/// - **REG_HP_TCM_RDN_ECO_RESULT** `[1]` (RO)
///
///   NA
///
/// - **REG_HP_TCM_RDN_ECO_EN** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct TcmRdnEcoCs {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TcmRdnEcoCs {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TcmRdnEcoCs {
    // ======== Field Constants ========

    /// **REG_HP_TCM_RDN_ECO_RESULT**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_HP_TCM_RDN_ECO_RESULT_OFFSET: u32 = 1;
    pub const REG_HP_TCM_RDN_ECO_RESULT_MASK: u32 = 0x00000002;
    pub const REG_HP_TCM_RDN_ECO_RESULT: u32 = 1 << 1;

    /// **REG_HP_TCM_RDN_ECO_EN**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_HP_TCM_RDN_ECO_EN_OFFSET: u32 = 0;
    pub const REG_HP_TCM_RDN_ECO_EN_MASK: u32 = 0x00000001;
    pub const REG_HP_TCM_RDN_ECO_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_HP_TCM_RDN_ECO_RESULT** `[1]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_hp_tcm_rdn_eco_result(&self) -> bool {
        (self.value.read_bits() & Self::REG_HP_TCM_RDN_ECO_RESULT_MASK) != 0
    }

    /// **REG_HP_TCM_RDN_ECO_EN** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_hp_tcm_rdn_eco_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_HP_TCM_RDN_ECO_EN_MASK) != 0
    }

    /// Set REG_HP_TCM_RDN_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_hp_tcm_rdn_eco_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_HP_TCM_RDN_ECO_EN_MASK);
    }

    /// Clear REG_HP_TCM_RDN_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_hp_tcm_rdn_eco_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_HP_TCM_RDN_ECO_EN_MASK);
    }
}

/// TCM_RDN_ECO_CS value type for batch read/modify/write operations
///
/// Use `tcmrdnecocs.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcmrdnecocs.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcmRdnEcoCsVal {
    bits: u32,
}

impl TcmRdnEcoCsVal {
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

    /// Read **REG_HP_TCM_RDN_ECO_RESULT** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_hp_tcm_rdn_eco_result(&self) -> bool {
        (self.bits & TcmRdnEcoCs::REG_HP_TCM_RDN_ECO_RESULT_MASK) != 0
    }

    /// Read **REG_HP_TCM_RDN_ECO_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_hp_tcm_rdn_eco_en(&self) -> bool {
        (self.bits & TcmRdnEcoCs::REG_HP_TCM_RDN_ECO_EN_MASK) != 0
    }

    /// Set **REG_HP_TCM_RDN_ECO_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_hp_tcm_rdn_eco_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TcmRdnEcoCs::REG_HP_TCM_RDN_ECO_EN_MASK
            } else {
                self.bits & !TcmRdnEcoCs::REG_HP_TCM_RDN_ECO_EN_MASK
            },
        }
    }
}

impl Default for TcmRdnEcoCsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TcmRdnEcoCs {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcmRdnEcoCsVal {
        TcmRdnEcoCsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TcmRdnEcoCsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TcmRdnEcoCsVal) -> TcmRdnEcoCsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TCM_RDN_ECO_LOW Register
///
/// NA
///
/// Address offset: 0x00e8
///
/// ## Fields
///
/// - **REG_HP_TCM_RDN_ECO_LOW** `[31:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct TcmRdnEcoLow {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TcmRdnEcoLow {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TcmRdnEcoLow {
    // ======== Field Constants ========

    /// **REG_HP_TCM_RDN_ECO_LOW**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_HP_TCM_RDN_ECO_LOW_OFFSET: u32 = 0;
    pub const REG_HP_TCM_RDN_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_HP_TCM_RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_hp_tcm_rdn_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::REG_HP_TCM_RDN_ECO_LOW_MASK)
            >> Self::REG_HP_TCM_RDN_ECO_LOW_OFFSET
    }

    /// Write **REG_HP_TCM_RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_hp_tcm_rdn_eco_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_HP_TCM_RDN_ECO_LOW_MASK)
                | ((value << Self::REG_HP_TCM_RDN_ECO_LOW_OFFSET)
                    & Self::REG_HP_TCM_RDN_ECO_LOW_MASK)
        });
    }
}

/// TCM_RDN_ECO_LOW value type for batch read/modify/write operations
///
/// Use `tcmrdnecolow.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcmrdnecolow.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcmRdnEcoLowVal {
    bits: u32,
}

impl TcmRdnEcoLowVal {
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

    /// Read **REG_HP_TCM_RDN_ECO_LOW** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_hp_tcm_rdn_eco_low(&self) -> u32 {
        (self.bits & TcmRdnEcoLow::REG_HP_TCM_RDN_ECO_LOW_MASK)
            >> TcmRdnEcoLow::REG_HP_TCM_RDN_ECO_LOW_OFFSET
    }

    /// Set **REG_HP_TCM_RDN_ECO_LOW** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_hp_tcm_rdn_eco_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TcmRdnEcoLow::REG_HP_TCM_RDN_ECO_LOW_MASK)
                | ((value << TcmRdnEcoLow::REG_HP_TCM_RDN_ECO_LOW_OFFSET)
                    & TcmRdnEcoLow::REG_HP_TCM_RDN_ECO_LOW_MASK),
        }
    }
}

impl Default for TcmRdnEcoLowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TcmRdnEcoLow {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcmRdnEcoLowVal {
        TcmRdnEcoLowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TcmRdnEcoLowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TcmRdnEcoLowVal) -> TcmRdnEcoLowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TCM_RDN_ECO_HIGH Register
///
/// NA
///
/// Address offset: 0x00ec
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **REG_HP_TCM_RDN_ECO_HIGH** `[31:0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct TcmRdnEcoHigh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TcmRdnEcoHigh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TcmRdnEcoHigh {
    // ======== Field Constants ========

    /// **REG_HP_TCM_RDN_ECO_HIGH**
    ///
    /// NA
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_HP_TCM_RDN_ECO_HIGH_OFFSET: u32 = 0;
    pub const REG_HP_TCM_RDN_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_HP_TCM_RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_hp_tcm_rdn_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::REG_HP_TCM_RDN_ECO_HIGH_MASK)
            >> Self::REG_HP_TCM_RDN_ECO_HIGH_OFFSET
    }

    /// Write **REG_HP_TCM_RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn write_reg_hp_tcm_rdn_eco_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_HP_TCM_RDN_ECO_HIGH_MASK)
                | ((value << Self::REG_HP_TCM_RDN_ECO_HIGH_OFFSET)
                    & Self::REG_HP_TCM_RDN_ECO_HIGH_MASK)
        });
    }
}

/// TCM_RDN_ECO_HIGH value type for batch read/modify/write operations
///
/// Use `tcmrdnecohigh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcmrdnecohigh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcmRdnEcoHighVal {
    bits: u32,
}

impl TcmRdnEcoHighVal {
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

    /// Read **REG_HP_TCM_RDN_ECO_HIGH** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_hp_tcm_rdn_eco_high(&self) -> u32 {
        (self.bits & TcmRdnEcoHigh::REG_HP_TCM_RDN_ECO_HIGH_MASK)
            >> TcmRdnEcoHigh::REG_HP_TCM_RDN_ECO_HIGH_OFFSET
    }

    /// Set **REG_HP_TCM_RDN_ECO_HIGH** `[31:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_hp_tcm_rdn_eco_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !TcmRdnEcoHigh::REG_HP_TCM_RDN_ECO_HIGH_MASK)
                | ((value << TcmRdnEcoHigh::REG_HP_TCM_RDN_ECO_HIGH_OFFSET)
                    & TcmRdnEcoHigh::REG_HP_TCM_RDN_ECO_HIGH_MASK),
        }
    }
}

impl Default for TcmRdnEcoHighVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TcmRdnEcoHigh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcmRdnEcoHighVal {
        TcmRdnEcoHighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TcmRdnEcoHighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TcmRdnEcoHighVal) -> TcmRdnEcoHighVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GPIO_DED_HOLD_CTRL Register
///
/// NA
///
/// Address offset: 0x00f0
///
/// ## Fields
///
/// - **REG_GPIO_DED_HOLD** `[25:0]` (RW)
///
///   hold control for gpio63~56
#[repr(transparent)]
pub struct GpioDedHoldCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioDedHoldCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioDedHoldCtrl {
    // ======== Field Constants ========

    /// **REG_GPIO_DED_HOLD**
    ///
    /// hold control for gpio63~56
    ///
    /// Bits `[25:0]`, mask `0x03ffffff`
    pub const REG_GPIO_DED_HOLD_OFFSET: u32 = 0;
    pub const REG_GPIO_DED_HOLD_MASK: u32 = 0x03ffffff;

    // ======== Field Accessors ========

    /// **REG_GPIO_DED_HOLD** `[25:0]` (RW)
    ///
    /// hold control for gpio63~56
    #[inline(always)]
    pub fn reg_gpio_ded_hold(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_DED_HOLD_MASK) >> Self::REG_GPIO_DED_HOLD_OFFSET
    }

    /// Write **REG_GPIO_DED_HOLD** `[25:0]` (RW)
    ///
    /// hold control for gpio63~56
    #[inline(always)]
    pub fn write_reg_gpio_ded_hold(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_DED_HOLD_MASK)
                | ((value << Self::REG_GPIO_DED_HOLD_OFFSET) & Self::REG_GPIO_DED_HOLD_MASK)
        });
    }
}

/// GPIO_DED_HOLD_CTRL value type for batch read/modify/write operations
///
/// Use `gpiodedholdctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpiodedholdctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioDedHoldCtrlVal {
    bits: u32,
}

impl GpioDedHoldCtrlVal {
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

    /// Read **REG_GPIO_DED_HOLD** `[25:0]`
    ///
    /// hold control for gpio63~56
    #[inline(always)]
    pub const fn reg_gpio_ded_hold(&self) -> u32 {
        (self.bits & GpioDedHoldCtrl::REG_GPIO_DED_HOLD_MASK)
            >> GpioDedHoldCtrl::REG_GPIO_DED_HOLD_OFFSET
    }

    /// Set **REG_GPIO_DED_HOLD** `[25:0]`
    ///
    /// hold control for gpio63~56
    #[inline(always)]
    pub const fn with_reg_gpio_ded_hold(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioDedHoldCtrl::REG_GPIO_DED_HOLD_MASK)
                | ((value << GpioDedHoldCtrl::REG_GPIO_DED_HOLD_OFFSET)
                    & GpioDedHoldCtrl::REG_GPIO_DED_HOLD_MASK),
        }
    }
}

impl Default for GpioDedHoldCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GpioDedHoldCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioDedHoldCtrlVal {
        GpioDedHoldCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioDedHoldCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioDedHoldCtrlVal) -> GpioDedHoldCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L2_MEM_SW_ECC_BWE_MASK Register
///
/// NA
///
/// Address offset: 0x00f4
///
/// ## Fields
///
/// - **REG_L2_MEM_SW_ECC_BWE_MASK_CTRL** `[0]` (RW)
///
///   Set 1 to mask bwe hamming code bit
#[repr(transparent)]
pub struct L2MemSwEccBweMask {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemSwEccBweMask {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemSwEccBweMask {
    // ======== Field Constants ========

    /// **REG_L2_MEM_SW_ECC_BWE_MASK_CTRL**
    ///
    /// Set 1 to mask bwe hamming code bit
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_L2_MEM_SW_ECC_BWE_MASK_CTRL_OFFSET: u32 = 0;
    pub const REG_L2_MEM_SW_ECC_BWE_MASK_CTRL_MASK: u32 = 0x00000001;
    pub const REG_L2_MEM_SW_ECC_BWE_MASK_CTRL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_L2_MEM_SW_ECC_BWE_MASK_CTRL** `[0]` (RW)
    ///
    /// Set 1 to mask bwe hamming code bit
    #[inline(always)]
    pub fn reg_l2_mem_sw_ecc_bwe_mask_ctrl(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_SW_ECC_BWE_MASK_CTRL_MASK) != 0
    }

    /// Set REG_L2_MEM_SW_ECC_BWE_MASK_CTRL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_sw_ecc_bwe_mask_ctrl(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_SW_ECC_BWE_MASK_CTRL_MASK);
    }

    /// Clear REG_L2_MEM_SW_ECC_BWE_MASK_CTRL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_sw_ecc_bwe_mask_ctrl(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_SW_ECC_BWE_MASK_CTRL_MASK);
    }
}

/// L2_MEM_SW_ECC_BWE_MASK value type for batch read/modify/write operations
///
/// Use `l2memsweccbwemask.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memsweccbwemask.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemSwEccBweMaskVal {
    bits: u32,
}

impl L2MemSwEccBweMaskVal {
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

    /// Read **REG_L2_MEM_SW_ECC_BWE_MASK_CTRL** `[0:0]`
    ///
    /// Set 1 to mask bwe hamming code bit
    #[inline(always)]
    pub const fn reg_l2_mem_sw_ecc_bwe_mask_ctrl(&self) -> bool {
        (self.bits & L2MemSwEccBweMask::REG_L2_MEM_SW_ECC_BWE_MASK_CTRL_MASK) != 0
    }

    /// Set **REG_L2_MEM_SW_ECC_BWE_MASK_CTRL** `[0:0]`
    ///
    /// Set 1 to mask bwe hamming code bit
    #[inline(always)]
    pub const fn with_reg_l2_mem_sw_ecc_bwe_mask_ctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemSwEccBweMask::REG_L2_MEM_SW_ECC_BWE_MASK_CTRL_MASK
            } else {
                self.bits & !L2MemSwEccBweMask::REG_L2_MEM_SW_ECC_BWE_MASK_CTRL_MASK
            },
        }
    }
}

impl Default for L2MemSwEccBweMaskVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemSwEccBweMask {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemSwEccBweMaskVal {
        L2MemSwEccBweMaskVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2MemSwEccBweMaskVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2MemSwEccBweMaskVal) -> L2MemSwEccBweMaskVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USB20OTG_MEM_CTRL Register
///
/// NA
///
/// Address offset: 0x00f8
///
/// ## Fields
///
/// - **REG_USB20_MEM_CLK_FORCE_ON** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct Usb20otgMemCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Usb20otgMemCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Usb20otgMemCtrl {
    // ======== Field Constants ========

    /// **REG_USB20_MEM_CLK_FORCE_ON**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_USB20_MEM_CLK_FORCE_ON_OFFSET: u32 = 0;
    pub const REG_USB20_MEM_CLK_FORCE_ON_MASK: u32 = 0x00000001;
    pub const REG_USB20_MEM_CLK_FORCE_ON: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_USB20_MEM_CLK_FORCE_ON** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_usb20_mem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::REG_USB20_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set REG_USB20_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_usb20_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_USB20_MEM_CLK_FORCE_ON_MASK);
    }

    /// Clear REG_USB20_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_usb20_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_USB20_MEM_CLK_FORCE_ON_MASK);
    }
}

/// USB20OTG_MEM_CTRL value type for batch read/modify/write operations
///
/// Use `usb20otgmemctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usb20otgmemctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Usb20otgMemCtrlVal {
    bits: u32,
}

impl Usb20otgMemCtrlVal {
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

    /// Read **REG_USB20_MEM_CLK_FORCE_ON** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_usb20_mem_clk_force_on(&self) -> bool {
        (self.bits & Usb20otgMemCtrl::REG_USB20_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set **REG_USB20_MEM_CLK_FORCE_ON** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_usb20_mem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usb20otgMemCtrl::REG_USB20_MEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !Usb20otgMemCtrl::REG_USB20_MEM_CLK_FORCE_ON_MASK
            },
        }
    }
}

impl Default for Usb20otgMemCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Usb20otgMemCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Usb20otgMemCtrlVal {
        Usb20otgMemCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Usb20otgMemCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Usb20otgMemCtrlVal) -> Usb20otgMemCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TCM_INT_RAW Register
///
/// need_des
///
/// Address offset: 0x00fc
///
/// ## Fields
///
/// - **TCM_PARITY_ERR_INT_RAW** `[31]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct TcmIntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TcmIntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TcmIntRaw {
    // ======== Field Constants ========

    /// **TCM_PARITY_ERR_INT_RAW**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TCM_PARITY_ERR_INT_RAW_OFFSET: u32 = 31;
    pub const TCM_PARITY_ERR_INT_RAW_MASK: u32 = 0x80000000;
    pub const TCM_PARITY_ERR_INT_RAW: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **TCM_PARITY_ERR_INT_RAW** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn tcm_parity_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TCM_PARITY_ERR_INT_RAW_MASK) != 0
    }

    /// Set TCM_PARITY_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tcm_parity_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::TCM_PARITY_ERR_INT_RAW_MASK);
    }

    /// Clear TCM_PARITY_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tcm_parity_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::TCM_PARITY_ERR_INT_RAW_MASK);
    }
}

/// TCM_INT_RAW value type for batch read/modify/write operations
///
/// Use `tcmintraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcmintraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcmIntRawVal {
    bits: u32,
}

impl TcmIntRawVal {
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

    /// Read **TCM_PARITY_ERR_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn tcm_parity_err_int_raw(&self) -> bool {
        (self.bits & TcmIntRaw::TCM_PARITY_ERR_INT_RAW_MASK) != 0
    }

    /// Set **TCM_PARITY_ERR_INT_RAW** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_tcm_parity_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TcmIntRaw::TCM_PARITY_ERR_INT_RAW_MASK
            } else {
                self.bits & !TcmIntRaw::TCM_PARITY_ERR_INT_RAW_MASK
            },
        }
    }
}

impl Default for TcmIntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TcmIntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcmIntRawVal {
        TcmIntRawVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TcmIntRawVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TcmIntRawVal) -> TcmIntRawVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TCM_INT_ST Register
///
/// need_des
///
/// Address offset: 0x0100
///
/// ## Fields
///
/// - **TCM_PARITY_ERR_INT_ST** `[31]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct TcmIntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TcmIntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TcmIntSt {
    // ======== Field Constants ========

    /// **TCM_PARITY_ERR_INT_ST**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TCM_PARITY_ERR_INT_ST_OFFSET: u32 = 31;
    pub const TCM_PARITY_ERR_INT_ST_MASK: u32 = 0x80000000;
    pub const TCM_PARITY_ERR_INT_ST: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **TCM_PARITY_ERR_INT_ST** `[31]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn tcm_parity_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TCM_PARITY_ERR_INT_ST_MASK) != 0
    }
}

/// TCM_INT_ST value type for batch read/modify/write operations
///
/// Use `tcmintst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcmintst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcmIntStVal {
    bits: u32,
}

impl TcmIntStVal {
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

    /// Read **TCM_PARITY_ERR_INT_ST** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn tcm_parity_err_int_st(&self) -> bool {
        (self.bits & TcmIntSt::TCM_PARITY_ERR_INT_ST_MASK) != 0
    }
}

impl Default for TcmIntStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TcmIntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcmIntStVal {
        TcmIntStVal::from_bits(self.value.read_bits())
    }
}

/// TCM_INT_ENA Register
///
/// need_des
///
/// Address offset: 0x0104
///
/// ## Fields
///
/// - **TCM_PARITY_ERR_INT_ENA** `[31]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct TcmIntEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TcmIntEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TcmIntEna {
    // ======== Field Constants ========

    /// **TCM_PARITY_ERR_INT_ENA**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TCM_PARITY_ERR_INT_ENA_OFFSET: u32 = 31;
    pub const TCM_PARITY_ERR_INT_ENA_MASK: u32 = 0x80000000;
    pub const TCM_PARITY_ERR_INT_ENA: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **TCM_PARITY_ERR_INT_ENA** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn tcm_parity_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TCM_PARITY_ERR_INT_ENA_MASK) != 0
    }

    /// Set TCM_PARITY_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tcm_parity_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::TCM_PARITY_ERR_INT_ENA_MASK);
    }

    /// Clear TCM_PARITY_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tcm_parity_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::TCM_PARITY_ERR_INT_ENA_MASK);
    }
}

/// TCM_INT_ENA value type for batch read/modify/write operations
///
/// Use `tcmintena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcmintena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcmIntEnaVal {
    bits: u32,
}

impl TcmIntEnaVal {
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

    /// Read **TCM_PARITY_ERR_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn tcm_parity_err_int_ena(&self) -> bool {
        (self.bits & TcmIntEna::TCM_PARITY_ERR_INT_ENA_MASK) != 0
    }

    /// Set **TCM_PARITY_ERR_INT_ENA** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_tcm_parity_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TcmIntEna::TCM_PARITY_ERR_INT_ENA_MASK
            } else {
                self.bits & !TcmIntEna::TCM_PARITY_ERR_INT_ENA_MASK
            },
        }
    }
}

impl Default for TcmIntEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TcmIntEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcmIntEnaVal {
        TcmIntEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TcmIntEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TcmIntEnaVal) -> TcmIntEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TCM_INT_CLR Register
///
/// need_des
///
/// Address offset: 0x0108
///
/// ## Fields
///
/// - **TCM_PARITY_ERR_INT_CLR** `[31]` (WO)
///
///   need_des
#[repr(transparent)]
pub struct TcmIntClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TcmIntClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TcmIntClr {
    // ======== Field Constants ========

    /// **TCM_PARITY_ERR_INT_CLR**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TCM_PARITY_ERR_INT_CLR_OFFSET: u32 = 31;
    pub const TCM_PARITY_ERR_INT_CLR_MASK: u32 = 0x80000000;
    pub const TCM_PARITY_ERR_INT_CLR: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// Trigger **TCM_PARITY_ERR_INT_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_tcm_parity_err_int_clr(&self) {
        self.value.write_bits(Self::TCM_PARITY_ERR_INT_CLR_MASK);
    }
}

/// TCM_INT_CLR value type for batch read/modify/write operations
///
/// Use `tcmintclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcmintclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcmIntClrVal {
    bits: u32,
}

impl TcmIntClrVal {
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

    /// Set **TCM_PARITY_ERR_INT_CLR** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_tcm_parity_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TcmIntClr::TCM_PARITY_ERR_INT_CLR_MASK
            } else {
                self.bits & !TcmIntClr::TCM_PARITY_ERR_INT_CLR_MASK
            },
        }
    }
}

impl Default for TcmIntClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TcmIntClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TcmIntClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// TCM_PARITY_INT_RECORD Register
///
/// need_des
///
/// Address offset: 0x010c
///
/// ## Fields
///
/// - **TCM_PARITY_ERR_INT_ADDR** `[12:0]` (RO)
///
///   hp tcm_parity_err_addr
#[repr(transparent)]
pub struct TcmParityIntRecord {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TcmParityIntRecord {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TcmParityIntRecord {
    // ======== Field Constants ========

    /// **TCM_PARITY_ERR_INT_ADDR**
    ///
    /// hp tcm_parity_err_addr
    ///
    /// Bits `[12:0]`, mask `0x00001fff`
    pub const TCM_PARITY_ERR_INT_ADDR_OFFSET: u32 = 0;
    pub const TCM_PARITY_ERR_INT_ADDR_MASK: u32 = 0x00001fff;

    // ======== Field Accessors ========

    /// **TCM_PARITY_ERR_INT_ADDR** `[12:0]` (RO)
    ///
    /// hp tcm_parity_err_addr
    #[inline(always)]
    pub fn tcm_parity_err_int_addr(&self) -> u32 {
        (self.value.read_bits() & Self::TCM_PARITY_ERR_INT_ADDR_MASK)
            >> Self::TCM_PARITY_ERR_INT_ADDR_OFFSET
    }
}

/// TCM_PARITY_INT_RECORD value type for batch read/modify/write operations
///
/// Use `tcmparityintrecord.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcmparityintrecord.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcmParityIntRecordVal {
    bits: u32,
}

impl TcmParityIntRecordVal {
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

    /// Read **TCM_PARITY_ERR_INT_ADDR** `[12:0]`
    ///
    /// hp tcm_parity_err_addr
    #[inline(always)]
    pub const fn tcm_parity_err_int_addr(&self) -> u32 {
        (self.bits & TcmParityIntRecord::TCM_PARITY_ERR_INT_ADDR_MASK)
            >> TcmParityIntRecord::TCM_PARITY_ERR_INT_ADDR_OFFSET
    }
}

impl Default for TcmParityIntRecordVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TcmParityIntRecord {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcmParityIntRecordVal {
        TcmParityIntRecordVal::from_bits(self.value.read_bits())
    }
}

/// L1_CACHE_PWR_CTRL Register
///
/// NA
///
/// Address offset: 0x0110
///
/// ## Fields
///
/// - **REG_L1_CACHE_MEM_FO** `[5:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct L1CachePwrCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L1CachePwrCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L1CachePwrCtrl {
    // ======== Field Constants ========

    /// **REG_L1_CACHE_MEM_FO**
    ///
    /// need_des
    ///
    /// Bits `[5:0]`, mask `0x0000003f`
    pub const REG_L1_CACHE_MEM_FO_OFFSET: u32 = 0;
    pub const REG_L1_CACHE_MEM_FO_MASK: u32 = 0x0000003f;

    // ======== Field Accessors ========

    /// **REG_L1_CACHE_MEM_FO** `[5:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn reg_l1_cache_mem_fo(&self) -> u32 {
        (self.value.read_bits() & Self::REG_L1_CACHE_MEM_FO_MASK)
            >> Self::REG_L1_CACHE_MEM_FO_OFFSET
    }

    /// Write **REG_L1_CACHE_MEM_FO** `[5:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_reg_l1_cache_mem_fo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_L1_CACHE_MEM_FO_MASK)
                | ((value << Self::REG_L1_CACHE_MEM_FO_OFFSET) & Self::REG_L1_CACHE_MEM_FO_MASK)
        });
    }
}

/// L1_CACHE_PWR_CTRL value type for batch read/modify/write operations
///
/// Use `l1cachepwrctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l1cachepwrctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L1CachePwrCtrlVal {
    bits: u32,
}

impl L1CachePwrCtrlVal {
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

    /// Read **REG_L1_CACHE_MEM_FO** `[5:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn reg_l1_cache_mem_fo(&self) -> u32 {
        (self.bits & L1CachePwrCtrl::REG_L1_CACHE_MEM_FO_MASK)
            >> L1CachePwrCtrl::REG_L1_CACHE_MEM_FO_OFFSET
    }

    /// Set **REG_L1_CACHE_MEM_FO** `[5:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_reg_l1_cache_mem_fo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !L1CachePwrCtrl::REG_L1_CACHE_MEM_FO_MASK)
                | ((value << L1CachePwrCtrl::REG_L1_CACHE_MEM_FO_OFFSET)
                    & L1CachePwrCtrl::REG_L1_CACHE_MEM_FO_MASK),
        }
    }
}

impl Default for L1CachePwrCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L1CachePwrCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L1CachePwrCtrlVal {
        L1CachePwrCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L1CachePwrCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L1CachePwrCtrlVal) -> L1CachePwrCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L2_CACHE_PWR_CTRL Register
///
/// NA
///
/// Address offset: 0x0114
///
/// ## Fields
///
/// - **REG_L2_CACHE_MEM_FO** `[1:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct L2CachePwrCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2CachePwrCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2CachePwrCtrl {
    // ======== Field Constants ========

    /// **REG_L2_CACHE_MEM_FO**
    ///
    /// need_des
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const REG_L2_CACHE_MEM_FO_OFFSET: u32 = 0;
    pub const REG_L2_CACHE_MEM_FO_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **REG_L2_CACHE_MEM_FO** `[1:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn reg_l2_cache_mem_fo(&self) -> u32 {
        (self.value.read_bits() & Self::REG_L2_CACHE_MEM_FO_MASK)
            >> Self::REG_L2_CACHE_MEM_FO_OFFSET
    }

    /// Write **REG_L2_CACHE_MEM_FO** `[1:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_reg_l2_cache_mem_fo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_L2_CACHE_MEM_FO_MASK)
                | ((value << Self::REG_L2_CACHE_MEM_FO_OFFSET) & Self::REG_L2_CACHE_MEM_FO_MASK)
        });
    }
}

/// L2_CACHE_PWR_CTRL value type for batch read/modify/write operations
///
/// Use `l2cachepwrctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2cachepwrctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2CachePwrCtrlVal {
    bits: u32,
}

impl L2CachePwrCtrlVal {
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

    /// Read **REG_L2_CACHE_MEM_FO** `[1:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn reg_l2_cache_mem_fo(&self) -> u32 {
        (self.bits & L2CachePwrCtrl::REG_L2_CACHE_MEM_FO_MASK)
            >> L2CachePwrCtrl::REG_L2_CACHE_MEM_FO_OFFSET
    }

    /// Set **REG_L2_CACHE_MEM_FO** `[1:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_reg_l2_cache_mem_fo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !L2CachePwrCtrl::REG_L2_CACHE_MEM_FO_MASK)
                | ((value << L2CachePwrCtrl::REG_L2_CACHE_MEM_FO_OFFSET)
                    & L2CachePwrCtrl::REG_L2_CACHE_MEM_FO_MASK),
        }
    }
}

impl Default for L2CachePwrCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2CachePwrCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2CachePwrCtrlVal {
        L2CachePwrCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2CachePwrCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2CachePwrCtrlVal) -> L2CachePwrCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CPU_WAITI_CONF Register
///
/// CPU_WAITI configuration register
///
/// Address offset: 0x0118
/// Reset value: 0x00000001
///
/// ## Fields
///
/// - **CPU_WAITI_DELAY_NUM** `[4:1]` (RW)
///
///   This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close
///
/// - **CPU_WAIT_MODE_FORCE_ON** `[0]` (RW)
///
///   Set 1 to force cpu_waiti_clk enable.
#[repr(transparent)]
pub struct CpuWaitiConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CpuWaitiConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CpuWaitiConf {
    // ======== Field Constants ========

    /// **CPU_WAITI_DELAY_NUM**
    ///
    /// This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close
    ///
    /// Bits `[4:1]`, mask `0x0000001e`
    pub const CPU_WAITI_DELAY_NUM_OFFSET: u32 = 1;
    pub const CPU_WAITI_DELAY_NUM_MASK: u32 = 0x0000001e;

    /// **CPU_WAIT_MODE_FORCE_ON**
    ///
    /// Set 1 to force cpu_waiti_clk enable.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CPU_WAIT_MODE_FORCE_ON_OFFSET: u32 = 0;
    pub const CPU_WAIT_MODE_FORCE_ON_MASK: u32 = 0x00000001;
    pub const CPU_WAIT_MODE_FORCE_ON: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CPU_WAITI_DELAY_NUM** `[4:1]` (RW)
    ///
    /// This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close
    #[inline(always)]
    pub fn cpu_waiti_delay_num(&self) -> u32 {
        (self.value.read_bits() & Self::CPU_WAITI_DELAY_NUM_MASK)
            >> Self::CPU_WAITI_DELAY_NUM_OFFSET
    }

    /// Write **CPU_WAITI_DELAY_NUM** `[4:1]` (RW)
    ///
    /// This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close
    #[inline(always)]
    pub fn write_cpu_waiti_delay_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CPU_WAITI_DELAY_NUM_MASK)
                | ((value << Self::CPU_WAITI_DELAY_NUM_OFFSET) & Self::CPU_WAITI_DELAY_NUM_MASK)
        });
    }

    /// **CPU_WAIT_MODE_FORCE_ON** `[0]` (RW)
    ///
    /// Set 1 to force cpu_waiti_clk enable.
    #[inline(always)]
    pub fn cpu_wait_mode_force_on(&self) -> bool {
        (self.value.read_bits() & Self::CPU_WAIT_MODE_FORCE_ON_MASK) != 0
    }

    /// Set CPU_WAIT_MODE_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cpu_wait_mode_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::CPU_WAIT_MODE_FORCE_ON_MASK);
    }

    /// Clear CPU_WAIT_MODE_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cpu_wait_mode_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::CPU_WAIT_MODE_FORCE_ON_MASK);
    }
}

/// CPU_WAITI_CONF value type for batch read/modify/write operations
///
/// Use `cpuwaiticonf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `cpuwaiticonf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CpuWaitiConfVal {
    bits: u32,
}

impl CpuWaitiConfVal {
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

    /// Read **CPU_WAITI_DELAY_NUM** `[4:1]`
    ///
    /// This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close
    #[inline(always)]
    pub const fn cpu_waiti_delay_num(&self) -> u32 {
        (self.bits & CpuWaitiConf::CPU_WAITI_DELAY_NUM_MASK)
            >> CpuWaitiConf::CPU_WAITI_DELAY_NUM_OFFSET
    }

    /// Read **CPU_WAIT_MODE_FORCE_ON** `[0:0]`
    ///
    /// Set 1 to force cpu_waiti_clk enable.
    #[inline(always)]
    pub const fn cpu_wait_mode_force_on(&self) -> bool {
        (self.bits & CpuWaitiConf::CPU_WAIT_MODE_FORCE_ON_MASK) != 0
    }

    /// Set **CPU_WAITI_DELAY_NUM** `[4:1]`
    ///
    /// This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close
    #[inline(always)]
    pub const fn with_cpu_waiti_delay_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CpuWaitiConf::CPU_WAITI_DELAY_NUM_MASK)
                | ((value << CpuWaitiConf::CPU_WAITI_DELAY_NUM_OFFSET)
                    & CpuWaitiConf::CPU_WAITI_DELAY_NUM_MASK),
        }
    }

    /// Set **CPU_WAIT_MODE_FORCE_ON** `[0:0]`
    ///
    /// Set 1 to force cpu_waiti_clk enable.
    #[inline(always)]
    pub const fn with_cpu_wait_mode_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CpuWaitiConf::CPU_WAIT_MODE_FORCE_ON_MASK
            } else {
                self.bits & !CpuWaitiConf::CPU_WAIT_MODE_FORCE_ON_MASK
            },
        }
    }
}

impl Default for CpuWaitiConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CpuWaitiConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CpuWaitiConfVal {
        CpuWaitiConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CpuWaitiConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CpuWaitiConfVal) -> CpuWaitiConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_DEBUG_RUNSTALL_CONF Register
///
/// Core Debug runstall configure register
///
/// Address offset: 0x011c
///
/// ## Fields
///
/// - **CORE_DEBUG_RUNSTALL_ENABLE** `[0]` (RW)
///
///   Set this field to 1 to enable debug runstall feature between HP-core and LP-core.
#[repr(transparent)]
pub struct CoreDebugRunstallConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CoreDebugRunstallConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoreDebugRunstallConf {
    // ======== Field Constants ========

    /// **CORE_DEBUG_RUNSTALL_ENABLE**
    ///
    /// Set this field to 1 to enable debug runstall feature between HP-core and LP-core.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_DEBUG_RUNSTALL_ENABLE_OFFSET: u32 = 0;
    pub const CORE_DEBUG_RUNSTALL_ENABLE_MASK: u32 = 0x00000001;
    pub const CORE_DEBUG_RUNSTALL_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_DEBUG_RUNSTALL_ENABLE** `[0]` (RW)
    ///
    /// Set this field to 1 to enable debug runstall feature between HP-core and LP-core.
    #[inline(always)]
    pub fn core_debug_runstall_enable(&self) -> bool {
        (self.value.read_bits() & Self::CORE_DEBUG_RUNSTALL_ENABLE_MASK) != 0
    }

    /// Set CORE_DEBUG_RUNSTALL_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core_debug_runstall_enable(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE_DEBUG_RUNSTALL_ENABLE_MASK);
    }

    /// Clear CORE_DEBUG_RUNSTALL_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core_debug_runstall_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE_DEBUG_RUNSTALL_ENABLE_MASK);
    }
}

/// CORE_DEBUG_RUNSTALL_CONF value type for batch read/modify/write operations
///
/// Use `coredebugrunstallconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `coredebugrunstallconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoreDebugRunstallConfVal {
    bits: u32,
}

impl CoreDebugRunstallConfVal {
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

    /// Read **CORE_DEBUG_RUNSTALL_ENABLE** `[0:0]`
    ///
    /// Set this field to 1 to enable debug runstall feature between HP-core and LP-core.
    #[inline(always)]
    pub const fn core_debug_runstall_enable(&self) -> bool {
        (self.bits & CoreDebugRunstallConf::CORE_DEBUG_RUNSTALL_ENABLE_MASK) != 0
    }

    /// Set **CORE_DEBUG_RUNSTALL_ENABLE** `[0:0]`
    ///
    /// Set this field to 1 to enable debug runstall feature between HP-core and LP-core.
    #[inline(always)]
    pub const fn with_core_debug_runstall_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreDebugRunstallConf::CORE_DEBUG_RUNSTALL_ENABLE_MASK
            } else {
                self.bits & !CoreDebugRunstallConf::CORE_DEBUG_RUNSTALL_ENABLE_MASK
            },
        }
    }
}

impl Default for CoreDebugRunstallConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CoreDebugRunstallConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CoreDebugRunstallConfVal {
        CoreDebugRunstallConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CoreDebugRunstallConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CoreDebugRunstallConfVal) -> CoreDebugRunstallConfVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_AHB_TIMEOUT Register
///
/// need_des
///
/// Address offset: 0x0120
/// Reset value: 0x0001ffff
///
/// ## Fields
///
/// - **THRES** `[16:1]` (RW)
///
///   This field used to set hp core0&1 ahb bus timeout threshold
///
/// - **EN** `[0]` (RW)
///
///   set this field to 1 to enable hp core0&1 ahb timeout handle
#[repr(transparent)]
pub struct CoreAhbTimeout {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CoreAhbTimeout {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoreAhbTimeout {
    // ======== Field Constants ========

    /// **THRES**
    ///
    /// This field used to set hp core0&1 ahb bus timeout threshold
    ///
    /// Bits `[16:1]`, mask `0x0001fffe`
    pub const THRES_OFFSET: u32 = 1;
    pub const THRES_MASK: u32 = 0x0001fffe;

    /// **EN**
    ///
    /// set this field to 1 to enable hp core0&1 ahb timeout handle
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **THRES** `[16:1]` (RW)
    ///
    /// This field used to set hp core0&1 ahb bus timeout threshold
    #[inline(always)]
    pub fn thres(&self) -> u32 {
        (self.value.read_bits() & Self::THRES_MASK) >> Self::THRES_OFFSET
    }

    /// Write **THRES** `[16:1]` (RW)
    ///
    /// This field used to set hp core0&1 ahb bus timeout threshold
    #[inline(always)]
    pub fn write_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::THRES_MASK) | ((value << Self::THRES_OFFSET) & Self::THRES_MASK)
        });
    }

    /// **EN** `[0]` (RW)
    ///
    /// set this field to 1 to enable hp core0&1 ahb timeout handle
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

/// CORE_AHB_TIMEOUT value type for batch read/modify/write operations
///
/// Use `coreahbtimeout.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `coreahbtimeout.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoreAhbTimeoutVal {
    bits: u32,
}

impl CoreAhbTimeoutVal {
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
        Self { bits: 0x0001ffff }
    }

    /// Read **THRES** `[16:1]`
    ///
    /// This field used to set hp core0&1 ahb bus timeout threshold
    #[inline(always)]
    pub const fn thres(&self) -> u32 {
        (self.bits & CoreAhbTimeout::THRES_MASK) >> CoreAhbTimeout::THRES_OFFSET
    }

    /// Read **EN** `[0:0]`
    ///
    /// set this field to 1 to enable hp core0&1 ahb timeout handle
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & CoreAhbTimeout::EN_MASK) != 0
    }

    /// Set **THRES** `[16:1]`
    ///
    /// This field used to set hp core0&1 ahb bus timeout threshold
    #[inline(always)]
    pub const fn with_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CoreAhbTimeout::THRES_MASK)
                | ((value << CoreAhbTimeout::THRES_OFFSET) & CoreAhbTimeout::THRES_MASK),
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// set this field to 1 to enable hp core0&1 ahb timeout handle
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreAhbTimeout::EN_MASK
            } else {
                self.bits & !CoreAhbTimeout::EN_MASK
            },
        }
    }
}

impl Default for CoreAhbTimeoutVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CoreAhbTimeout {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CoreAhbTimeoutVal {
        CoreAhbTimeoutVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CoreAhbTimeoutVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CoreAhbTimeoutVal) -> CoreAhbTimeoutVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_IBUS_TIMEOUT Register
///
/// need_des
///
/// Address offset: 0x0124
/// Reset value: 0x0001ffff
///
/// ## Fields
///
/// - **THRES** `[16:1]` (RW)
///
///   This field used to set hp core0&1 ibus timeout threshold
///
/// - **EN** `[0]` (RW)
///
///   set this field to 1 to enable hp core0&1 ibus timeout handle
#[repr(transparent)]
pub struct CoreIbusTimeout {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CoreIbusTimeout {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoreIbusTimeout {
    // ======== Field Constants ========

    /// **THRES**
    ///
    /// This field used to set hp core0&1 ibus timeout threshold
    ///
    /// Bits `[16:1]`, mask `0x0001fffe`
    pub const THRES_OFFSET: u32 = 1;
    pub const THRES_MASK: u32 = 0x0001fffe;

    /// **EN**
    ///
    /// set this field to 1 to enable hp core0&1 ibus timeout handle
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **THRES** `[16:1]` (RW)
    ///
    /// This field used to set hp core0&1 ibus timeout threshold
    #[inline(always)]
    pub fn thres(&self) -> u32 {
        (self.value.read_bits() & Self::THRES_MASK) >> Self::THRES_OFFSET
    }

    /// Write **THRES** `[16:1]` (RW)
    ///
    /// This field used to set hp core0&1 ibus timeout threshold
    #[inline(always)]
    pub fn write_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::THRES_MASK) | ((value << Self::THRES_OFFSET) & Self::THRES_MASK)
        });
    }

    /// **EN** `[0]` (RW)
    ///
    /// set this field to 1 to enable hp core0&1 ibus timeout handle
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

/// CORE_IBUS_TIMEOUT value type for batch read/modify/write operations
///
/// Use `coreibustimeout.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `coreibustimeout.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoreIbusTimeoutVal {
    bits: u32,
}

impl CoreIbusTimeoutVal {
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
        Self { bits: 0x0001ffff }
    }

    /// Read **THRES** `[16:1]`
    ///
    /// This field used to set hp core0&1 ibus timeout threshold
    #[inline(always)]
    pub const fn thres(&self) -> u32 {
        (self.bits & CoreIbusTimeout::THRES_MASK) >> CoreIbusTimeout::THRES_OFFSET
    }

    /// Read **EN** `[0:0]`
    ///
    /// set this field to 1 to enable hp core0&1 ibus timeout handle
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & CoreIbusTimeout::EN_MASK) != 0
    }

    /// Set **THRES** `[16:1]`
    ///
    /// This field used to set hp core0&1 ibus timeout threshold
    #[inline(always)]
    pub const fn with_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CoreIbusTimeout::THRES_MASK)
                | ((value << CoreIbusTimeout::THRES_OFFSET) & CoreIbusTimeout::THRES_MASK),
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// set this field to 1 to enable hp core0&1 ibus timeout handle
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreIbusTimeout::EN_MASK
            } else {
                self.bits & !CoreIbusTimeout::EN_MASK
            },
        }
    }
}

impl Default for CoreIbusTimeoutVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CoreIbusTimeout {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CoreIbusTimeoutVal {
        CoreIbusTimeoutVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CoreIbusTimeoutVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CoreIbusTimeoutVal) -> CoreIbusTimeoutVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_DBUS_TIMEOUT Register
///
/// need_des
///
/// Address offset: 0x0128
/// Reset value: 0x0001ffff
///
/// ## Fields
///
/// - **THRES** `[16:1]` (RW)
///
///   This field used to set hp core0&1 dbus timeout threshold
///
/// - **EN** `[0]` (RW)
///
///   set this field to 1 to enable hp core0&1 dbus timeout handle
#[repr(transparent)]
pub struct CoreDbusTimeout {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CoreDbusTimeout {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoreDbusTimeout {
    // ======== Field Constants ========

    /// **THRES**
    ///
    /// This field used to set hp core0&1 dbus timeout threshold
    ///
    /// Bits `[16:1]`, mask `0x0001fffe`
    pub const THRES_OFFSET: u32 = 1;
    pub const THRES_MASK: u32 = 0x0001fffe;

    /// **EN**
    ///
    /// set this field to 1 to enable hp core0&1 dbus timeout handle
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **THRES** `[16:1]` (RW)
    ///
    /// This field used to set hp core0&1 dbus timeout threshold
    #[inline(always)]
    pub fn thres(&self) -> u32 {
        (self.value.read_bits() & Self::THRES_MASK) >> Self::THRES_OFFSET
    }

    /// Write **THRES** `[16:1]` (RW)
    ///
    /// This field used to set hp core0&1 dbus timeout threshold
    #[inline(always)]
    pub fn write_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::THRES_MASK) | ((value << Self::THRES_OFFSET) & Self::THRES_MASK)
        });
    }

    /// **EN** `[0]` (RW)
    ///
    /// set this field to 1 to enable hp core0&1 dbus timeout handle
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

/// CORE_DBUS_TIMEOUT value type for batch read/modify/write operations
///
/// Use `coredbustimeout.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `coredbustimeout.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoreDbusTimeoutVal {
    bits: u32,
}

impl CoreDbusTimeoutVal {
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
        Self { bits: 0x0001ffff }
    }

    /// Read **THRES** `[16:1]`
    ///
    /// This field used to set hp core0&1 dbus timeout threshold
    #[inline(always)]
    pub const fn thres(&self) -> u32 {
        (self.bits & CoreDbusTimeout::THRES_MASK) >> CoreDbusTimeout::THRES_OFFSET
    }

    /// Read **EN** `[0:0]`
    ///
    /// set this field to 1 to enable hp core0&1 dbus timeout handle
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & CoreDbusTimeout::EN_MASK) != 0
    }

    /// Set **THRES** `[16:1]`
    ///
    /// This field used to set hp core0&1 dbus timeout threshold
    #[inline(always)]
    pub const fn with_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CoreDbusTimeout::THRES_MASK)
                | ((value << CoreDbusTimeout::THRES_OFFSET) & CoreDbusTimeout::THRES_MASK),
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// set this field to 1 to enable hp core0&1 dbus timeout handle
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreDbusTimeout::EN_MASK
            } else {
                self.bits & !CoreDbusTimeout::EN_MASK
            },
        }
    }
}

impl Default for CoreDbusTimeoutVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl CoreDbusTimeout {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CoreDbusTimeoutVal {
        CoreDbusTimeoutVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CoreDbusTimeoutVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CoreDbusTimeoutVal) -> CoreDbusTimeoutVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ICM_CPU_H2X_CFG Register
///
/// need_des
///
/// Address offset: 0x0138
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **CPU_ICM_H2X_BRIDGE_BUSY** `[2]` (RO)
///
///   need_des
///
/// - **CPU_ICM_H2X_CUT_THROUGH_EN** `[1]` (RW)
///
///   need_des
///
/// - **CPU_ICM_H2X_POST_WR_EN** `[0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct IcmCpuH2xCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IcmCpuH2xCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IcmCpuH2xCfg {
    // ======== Field Constants ========

    /// **CPU_ICM_H2X_BRIDGE_BUSY**
    ///
    /// need_des
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CPU_ICM_H2X_BRIDGE_BUSY_OFFSET: u32 = 2;
    pub const CPU_ICM_H2X_BRIDGE_BUSY_MASK: u32 = 0x00000004;
    pub const CPU_ICM_H2X_BRIDGE_BUSY: u32 = 1 << 2;

    /// **CPU_ICM_H2X_CUT_THROUGH_EN**
    ///
    /// need_des
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CPU_ICM_H2X_CUT_THROUGH_EN_OFFSET: u32 = 1;
    pub const CPU_ICM_H2X_CUT_THROUGH_EN_MASK: u32 = 0x00000002;
    pub const CPU_ICM_H2X_CUT_THROUGH_EN: u32 = 1 << 1;

    /// **CPU_ICM_H2X_POST_WR_EN**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CPU_ICM_H2X_POST_WR_EN_OFFSET: u32 = 0;
    pub const CPU_ICM_H2X_POST_WR_EN_MASK: u32 = 0x00000001;
    pub const CPU_ICM_H2X_POST_WR_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CPU_ICM_H2X_BRIDGE_BUSY** `[2]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn cpu_icm_h2x_bridge_busy(&self) -> bool {
        (self.value.read_bits() & Self::CPU_ICM_H2X_BRIDGE_BUSY_MASK) != 0
    }

    /// **CPU_ICM_H2X_CUT_THROUGH_EN** `[1]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn cpu_icm_h2x_cut_through_en(&self) -> bool {
        (self.value.read_bits() & Self::CPU_ICM_H2X_CUT_THROUGH_EN_MASK) != 0
    }

    /// Set CPU_ICM_H2X_CUT_THROUGH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cpu_icm_h2x_cut_through_en(&self) {
        self.value
            .modify_bits(|r| r | Self::CPU_ICM_H2X_CUT_THROUGH_EN_MASK);
    }

    /// Clear CPU_ICM_H2X_CUT_THROUGH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cpu_icm_h2x_cut_through_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::CPU_ICM_H2X_CUT_THROUGH_EN_MASK);
    }

    /// **CPU_ICM_H2X_POST_WR_EN** `[0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn cpu_icm_h2x_post_wr_en(&self) -> bool {
        (self.value.read_bits() & Self::CPU_ICM_H2X_POST_WR_EN_MASK) != 0
    }

    /// Set CPU_ICM_H2X_POST_WR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cpu_icm_h2x_post_wr_en(&self) {
        self.value
            .modify_bits(|r| r | Self::CPU_ICM_H2X_POST_WR_EN_MASK);
    }

    /// Clear CPU_ICM_H2X_POST_WR_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cpu_icm_h2x_post_wr_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::CPU_ICM_H2X_POST_WR_EN_MASK);
    }
}

/// ICM_CPU_H2X_CFG value type for batch read/modify/write operations
///
/// Use `icmcpuh2xcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `icmcpuh2xcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IcmCpuH2xCfgVal {
    bits: u32,
}

impl IcmCpuH2xCfgVal {
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
        Self { bits: 0x00000003 }
    }

    /// Read **CPU_ICM_H2X_BRIDGE_BUSY** `[2:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn cpu_icm_h2x_bridge_busy(&self) -> bool {
        (self.bits & IcmCpuH2xCfg::CPU_ICM_H2X_BRIDGE_BUSY_MASK) != 0
    }

    /// Read **CPU_ICM_H2X_CUT_THROUGH_EN** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn cpu_icm_h2x_cut_through_en(&self) -> bool {
        (self.bits & IcmCpuH2xCfg::CPU_ICM_H2X_CUT_THROUGH_EN_MASK) != 0
    }

    /// Read **CPU_ICM_H2X_POST_WR_EN** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn cpu_icm_h2x_post_wr_en(&self) -> bool {
        (self.bits & IcmCpuH2xCfg::CPU_ICM_H2X_POST_WR_EN_MASK) != 0
    }

    /// Set **CPU_ICM_H2X_CUT_THROUGH_EN** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_cpu_icm_h2x_cut_through_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcmCpuH2xCfg::CPU_ICM_H2X_CUT_THROUGH_EN_MASK
            } else {
                self.bits & !IcmCpuH2xCfg::CPU_ICM_H2X_CUT_THROUGH_EN_MASK
            },
        }
    }

    /// Set **CPU_ICM_H2X_POST_WR_EN** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_cpu_icm_h2x_post_wr_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IcmCpuH2xCfg::CPU_ICM_H2X_POST_WR_EN_MASK
            } else {
                self.bits & !IcmCpuH2xCfg::CPU_ICM_H2X_POST_WR_EN_MASK
            },
        }
    }
}

impl Default for IcmCpuH2xCfgVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl IcmCpuH2xCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IcmCpuH2xCfgVal {
        IcmCpuH2xCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: IcmCpuH2xCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(IcmCpuH2xCfgVal) -> IcmCpuH2xCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PERI1_APB_POSTW_EN Register
///
/// NA
///
/// Address offset: 0x013c
///
/// ## Fields
///
/// - **PERI1_APB_POSTW_EN** `[0]` (RW)
///
///   hp_peri1 apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
#[repr(transparent)]
pub struct Peri1ApbPostwEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Peri1ApbPostwEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Peri1ApbPostwEn {
    // ======== Field Constants ========

    /// **PERI1_APB_POSTW_EN**
    ///
    /// hp_peri1 apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PERI1_APB_POSTW_EN_OFFSET: u32 = 0;
    pub const PERI1_APB_POSTW_EN_MASK: u32 = 0x00000001;
    pub const PERI1_APB_POSTW_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PERI1_APB_POSTW_EN** `[0]` (RW)
    ///
    /// hp_peri1 apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
    #[inline(always)]
    pub fn peri1_apb_postw_en(&self) -> bool {
        (self.value.read_bits() & Self::PERI1_APB_POSTW_EN_MASK) != 0
    }

    /// Set PERI1_APB_POSTW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_peri1_apb_postw_en(&self) {
        self.value
            .modify_bits(|r| r | Self::PERI1_APB_POSTW_EN_MASK);
    }

    /// Clear PERI1_APB_POSTW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_peri1_apb_postw_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::PERI1_APB_POSTW_EN_MASK);
    }
}

/// PERI1_APB_POSTW_EN value type for batch read/modify/write operations
///
/// Use `peri1apbpostwen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `peri1apbpostwen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Peri1ApbPostwEnVal {
    bits: u32,
}

impl Peri1ApbPostwEnVal {
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

    /// Read **PERI1_APB_POSTW_EN** `[0:0]`
    ///
    /// hp_peri1 apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
    #[inline(always)]
    pub const fn peri1_apb_postw_en(&self) -> bool {
        (self.bits & Peri1ApbPostwEn::PERI1_APB_POSTW_EN_MASK) != 0
    }

    /// Set **PERI1_APB_POSTW_EN** `[0:0]`
    ///
    /// hp_peri1 apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
    #[inline(always)]
    pub const fn with_peri1_apb_postw_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Peri1ApbPostwEn::PERI1_APB_POSTW_EN_MASK
            } else {
                self.bits & !Peri1ApbPostwEn::PERI1_APB_POSTW_EN_MASK
            },
        }
    }
}

impl Default for Peri1ApbPostwEnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Peri1ApbPostwEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Peri1ApbPostwEnVal {
        Peri1ApbPostwEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Peri1ApbPostwEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Peri1ApbPostwEnVal) -> Peri1ApbPostwEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BITSCRAMBLER_PERI_SEL Register
///
/// Bitscrambler Peri Sel
///
/// Address offset: 0x0140
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **BITSCRAMBLER_PERI_TX_SEL** `[7:4]` (RW)
///
///   Set this field to sel peri with DMA TX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none
///
/// - **BITSCRAMBLER_PERI_RX_SEL** `[3:0]` (RW)
///
///   Set this field to sel peri with DMA RX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none
#[repr(transparent)]
pub struct BitscramblerPeriSel {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BitscramblerPeriSel {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BitscramblerPeriSel {
    // ======== Field Constants ========

    /// **BITSCRAMBLER_PERI_TX_SEL**
    ///
    /// Set this field to sel peri with DMA TX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none
    ///
    /// Bits `[7:4]`, mask `0x000000f0`
    pub const BITSCRAMBLER_PERI_TX_SEL_OFFSET: u32 = 4;
    pub const BITSCRAMBLER_PERI_TX_SEL_MASK: u32 = 0x000000f0;

    /// **BITSCRAMBLER_PERI_RX_SEL**
    ///
    /// Set this field to sel peri with DMA RX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none
    ///
    /// Bits `[3:0]`, mask `0x0000000f`
    pub const BITSCRAMBLER_PERI_RX_SEL_OFFSET: u32 = 0;
    pub const BITSCRAMBLER_PERI_RX_SEL_MASK: u32 = 0x0000000f;

    // ======== Field Accessors ========

    /// **BITSCRAMBLER_PERI_TX_SEL** `[7:4]` (RW)
    ///
    /// Set this field to sel peri with DMA TX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none
    #[inline(always)]
    pub fn bitscrambler_peri_tx_sel(&self) -> u32 {
        (self.value.read_bits() & Self::BITSCRAMBLER_PERI_TX_SEL_MASK)
            >> Self::BITSCRAMBLER_PERI_TX_SEL_OFFSET
    }

    /// Write **BITSCRAMBLER_PERI_TX_SEL** `[7:4]` (RW)
    ///
    /// Set this field to sel peri with DMA TX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none
    #[inline(always)]
    pub fn write_bitscrambler_peri_tx_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BITSCRAMBLER_PERI_TX_SEL_MASK)
                | ((value << Self::BITSCRAMBLER_PERI_TX_SEL_OFFSET)
                    & Self::BITSCRAMBLER_PERI_TX_SEL_MASK)
        });
    }

    /// **BITSCRAMBLER_PERI_RX_SEL** `[3:0]` (RW)
    ///
    /// Set this field to sel peri with DMA RX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none
    #[inline(always)]
    pub fn bitscrambler_peri_rx_sel(&self) -> u32 {
        (self.value.read_bits() & Self::BITSCRAMBLER_PERI_RX_SEL_MASK)
            >> Self::BITSCRAMBLER_PERI_RX_SEL_OFFSET
    }

    /// Write **BITSCRAMBLER_PERI_RX_SEL** `[3:0]` (RW)
    ///
    /// Set this field to sel peri with DMA RX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none
    #[inline(always)]
    pub fn write_bitscrambler_peri_rx_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BITSCRAMBLER_PERI_RX_SEL_MASK)
                | ((value << Self::BITSCRAMBLER_PERI_RX_SEL_OFFSET)
                    & Self::BITSCRAMBLER_PERI_RX_SEL_MASK)
        });
    }
}

/// BITSCRAMBLER_PERI_SEL value type for batch read/modify/write operations
///
/// Use `bitscramblerperisel.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bitscramblerperisel.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BitscramblerPeriSelVal {
    bits: u32,
}

impl BitscramblerPeriSelVal {
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
        Self { bits: 0x000000ff }
    }

    /// Read **BITSCRAMBLER_PERI_TX_SEL** `[7:4]`
    ///
    /// Set this field to sel peri with DMA TX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none
    #[inline(always)]
    pub const fn bitscrambler_peri_tx_sel(&self) -> u32 {
        (self.bits & BitscramblerPeriSel::BITSCRAMBLER_PERI_TX_SEL_MASK)
            >> BitscramblerPeriSel::BITSCRAMBLER_PERI_TX_SEL_OFFSET
    }

    /// Read **BITSCRAMBLER_PERI_RX_SEL** `[3:0]`
    ///
    /// Set this field to sel peri with DMA RX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none
    #[inline(always)]
    pub const fn bitscrambler_peri_rx_sel(&self) -> u32 {
        (self.bits & BitscramblerPeriSel::BITSCRAMBLER_PERI_RX_SEL_MASK)
            >> BitscramblerPeriSel::BITSCRAMBLER_PERI_RX_SEL_OFFSET
    }

    /// Set **BITSCRAMBLER_PERI_TX_SEL** `[7:4]`
    ///
    /// Set this field to sel peri with DMA TX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none
    #[inline(always)]
    pub const fn with_bitscrambler_peri_tx_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BitscramblerPeriSel::BITSCRAMBLER_PERI_TX_SEL_MASK)
                | ((value << BitscramblerPeriSel::BITSCRAMBLER_PERI_TX_SEL_OFFSET)
                    & BitscramblerPeriSel::BITSCRAMBLER_PERI_TX_SEL_MASK),
        }
    }

    /// Set **BITSCRAMBLER_PERI_RX_SEL** `[3:0]`
    ///
    /// Set this field to sel peri with DMA RX interface to connec with bitscrambler: 4'h0 : lcd_cam, 4'h1: gpspi2, 4'h2: gpspi3, 4'h3: parl_io, 4'h4: aes, 4'h5: sha, 4'h6: adc, 4'h7: i2s0, 4'h8: i2s1, 4'h9: i2s2, 4'ha: i3c_mst, 4'hb: uhci0, 4'hc: RMT, else : none
    #[inline(always)]
    pub const fn with_bitscrambler_peri_rx_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BitscramblerPeriSel::BITSCRAMBLER_PERI_RX_SEL_MASK)
                | ((value << BitscramblerPeriSel::BITSCRAMBLER_PERI_RX_SEL_OFFSET)
                    & BitscramblerPeriSel::BITSCRAMBLER_PERI_RX_SEL_MASK),
        }
    }
}

impl Default for BitscramblerPeriSelVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BitscramblerPeriSel {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BitscramblerPeriSelVal {
        BitscramblerPeriSelVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BitscramblerPeriSelVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BitscramblerPeriSelVal) -> BitscramblerPeriSelVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// APB_SYNC_POSTW_EN Register
///
/// N/A
///
/// Address offset: 0x0144
///
/// ## Fields
///
/// - **CSI_HOST_APB_ASYNC_POSTW_EN** `[3]` (RW)
///
///   N/A
///
/// - **CSI_HOST_APB_SYNC_POSTW_EN** `[2]` (RW)
///
///   N/A
///
/// - **DSI_HOST_APB_POSTW_EN** `[1]` (RW)
///
///   N/A
///
/// - **GMAC_APB_POSTW_EN** `[0]` (RW)
///
///   N/A
#[repr(transparent)]
pub struct ApbSyncPostwEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ApbSyncPostwEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ApbSyncPostwEn {
    // ======== Field Constants ========

    /// **CSI_HOST_APB_ASYNC_POSTW_EN**
    ///
    /// N/A
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CSI_HOST_APB_ASYNC_POSTW_EN_OFFSET: u32 = 3;
    pub const CSI_HOST_APB_ASYNC_POSTW_EN_MASK: u32 = 0x00000008;
    pub const CSI_HOST_APB_ASYNC_POSTW_EN: u32 = 1 << 3;

    /// **CSI_HOST_APB_SYNC_POSTW_EN**
    ///
    /// N/A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CSI_HOST_APB_SYNC_POSTW_EN_OFFSET: u32 = 2;
    pub const CSI_HOST_APB_SYNC_POSTW_EN_MASK: u32 = 0x00000004;
    pub const CSI_HOST_APB_SYNC_POSTW_EN: u32 = 1 << 2;

    /// **DSI_HOST_APB_POSTW_EN**
    ///
    /// N/A
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DSI_HOST_APB_POSTW_EN_OFFSET: u32 = 1;
    pub const DSI_HOST_APB_POSTW_EN_MASK: u32 = 0x00000002;
    pub const DSI_HOST_APB_POSTW_EN: u32 = 1 << 1;

    /// **GMAC_APB_POSTW_EN**
    ///
    /// N/A
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const GMAC_APB_POSTW_EN_OFFSET: u32 = 0;
    pub const GMAC_APB_POSTW_EN_MASK: u32 = 0x00000001;
    pub const GMAC_APB_POSTW_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CSI_HOST_APB_ASYNC_POSTW_EN** `[3]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn csi_host_apb_async_postw_en(&self) -> bool {
        (self.value.read_bits() & Self::CSI_HOST_APB_ASYNC_POSTW_EN_MASK) != 0
    }

    /// Set CSI_HOST_APB_ASYNC_POSTW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_csi_host_apb_async_postw_en(&self) {
        self.value
            .modify_bits(|r| r | Self::CSI_HOST_APB_ASYNC_POSTW_EN_MASK);
    }

    /// Clear CSI_HOST_APB_ASYNC_POSTW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_csi_host_apb_async_postw_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::CSI_HOST_APB_ASYNC_POSTW_EN_MASK);
    }

    /// **CSI_HOST_APB_SYNC_POSTW_EN** `[2]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn csi_host_apb_sync_postw_en(&self) -> bool {
        (self.value.read_bits() & Self::CSI_HOST_APB_SYNC_POSTW_EN_MASK) != 0
    }

    /// Set CSI_HOST_APB_SYNC_POSTW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_csi_host_apb_sync_postw_en(&self) {
        self.value
            .modify_bits(|r| r | Self::CSI_HOST_APB_SYNC_POSTW_EN_MASK);
    }

    /// Clear CSI_HOST_APB_SYNC_POSTW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_csi_host_apb_sync_postw_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::CSI_HOST_APB_SYNC_POSTW_EN_MASK);
    }

    /// **DSI_HOST_APB_POSTW_EN** `[1]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn dsi_host_apb_postw_en(&self) -> bool {
        (self.value.read_bits() & Self::DSI_HOST_APB_POSTW_EN_MASK) != 0
    }

    /// Set DSI_HOST_APB_POSTW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dsi_host_apb_postw_en(&self) {
        self.value
            .modify_bits(|r| r | Self::DSI_HOST_APB_POSTW_EN_MASK);
    }

    /// Clear DSI_HOST_APB_POSTW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dsi_host_apb_postw_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::DSI_HOST_APB_POSTW_EN_MASK);
    }

    /// **GMAC_APB_POSTW_EN** `[0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn gmac_apb_postw_en(&self) -> bool {
        (self.value.read_bits() & Self::GMAC_APB_POSTW_EN_MASK) != 0
    }

    /// Set GMAC_APB_POSTW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gmac_apb_postw_en(&self) {
        self.value.modify_bits(|r| r | Self::GMAC_APB_POSTW_EN_MASK);
    }

    /// Clear GMAC_APB_POSTW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gmac_apb_postw_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::GMAC_APB_POSTW_EN_MASK);
    }
}

/// APB_SYNC_POSTW_EN value type for batch read/modify/write operations
///
/// Use `apbsyncpostwen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `apbsyncpostwen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ApbSyncPostwEnVal {
    bits: u32,
}

impl ApbSyncPostwEnVal {
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

    /// Read **CSI_HOST_APB_ASYNC_POSTW_EN** `[3:3]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn csi_host_apb_async_postw_en(&self) -> bool {
        (self.bits & ApbSyncPostwEn::CSI_HOST_APB_ASYNC_POSTW_EN_MASK) != 0
    }

    /// Read **CSI_HOST_APB_SYNC_POSTW_EN** `[2:2]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn csi_host_apb_sync_postw_en(&self) -> bool {
        (self.bits & ApbSyncPostwEn::CSI_HOST_APB_SYNC_POSTW_EN_MASK) != 0
    }

    /// Read **DSI_HOST_APB_POSTW_EN** `[1:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn dsi_host_apb_postw_en(&self) -> bool {
        (self.bits & ApbSyncPostwEn::DSI_HOST_APB_POSTW_EN_MASK) != 0
    }

    /// Read **GMAC_APB_POSTW_EN** `[0:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn gmac_apb_postw_en(&self) -> bool {
        (self.bits & ApbSyncPostwEn::GMAC_APB_POSTW_EN_MASK) != 0
    }

    /// Set **CSI_HOST_APB_ASYNC_POSTW_EN** `[3:3]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_csi_host_apb_async_postw_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ApbSyncPostwEn::CSI_HOST_APB_ASYNC_POSTW_EN_MASK
            } else {
                self.bits & !ApbSyncPostwEn::CSI_HOST_APB_ASYNC_POSTW_EN_MASK
            },
        }
    }

    /// Set **CSI_HOST_APB_SYNC_POSTW_EN** `[2:2]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_csi_host_apb_sync_postw_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ApbSyncPostwEn::CSI_HOST_APB_SYNC_POSTW_EN_MASK
            } else {
                self.bits & !ApbSyncPostwEn::CSI_HOST_APB_SYNC_POSTW_EN_MASK
            },
        }
    }

    /// Set **DSI_HOST_APB_POSTW_EN** `[1:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_dsi_host_apb_postw_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ApbSyncPostwEn::DSI_HOST_APB_POSTW_EN_MASK
            } else {
                self.bits & !ApbSyncPostwEn::DSI_HOST_APB_POSTW_EN_MASK
            },
        }
    }

    /// Set **GMAC_APB_POSTW_EN** `[0:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_gmac_apb_postw_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ApbSyncPostwEn::GMAC_APB_POSTW_EN_MASK
            } else {
                self.bits & !ApbSyncPostwEn::GMAC_APB_POSTW_EN_MASK
            },
        }
    }
}

impl Default for ApbSyncPostwEnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ApbSyncPostwEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ApbSyncPostwEnVal {
        ApbSyncPostwEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ApbSyncPostwEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ApbSyncPostwEnVal) -> ApbSyncPostwEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GDMA_CTRL Register
///
/// N/A
///
/// Address offset: 0x0148
///
/// ## Fields
///
/// - **DEBUG_CH_NUM** `[1:0]` (RW)
///
///   N/A
#[repr(transparent)]
pub struct GdmaCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GdmaCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GdmaCtrl {
    // ======== Field Constants ========

    /// **DEBUG_CH_NUM**
    ///
    /// N/A
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const DEBUG_CH_NUM_OFFSET: u32 = 0;
    pub const DEBUG_CH_NUM_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **DEBUG_CH_NUM** `[1:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn debug_ch_num(&self) -> u32 {
        (self.value.read_bits() & Self::DEBUG_CH_NUM_MASK) >> Self::DEBUG_CH_NUM_OFFSET
    }

    /// Write **DEBUG_CH_NUM** `[1:0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_debug_ch_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DEBUG_CH_NUM_MASK)
                | ((value << Self::DEBUG_CH_NUM_OFFSET) & Self::DEBUG_CH_NUM_MASK)
        });
    }
}

/// GDMA_CTRL value type for batch read/modify/write operations
///
/// Use `gdmactrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gdmactrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GdmaCtrlVal {
    bits: u32,
}

impl GdmaCtrlVal {
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

    /// Read **DEBUG_CH_NUM** `[1:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn debug_ch_num(&self) -> u32 {
        (self.bits & GdmaCtrl::DEBUG_CH_NUM_MASK) >> GdmaCtrl::DEBUG_CH_NUM_OFFSET
    }

    /// Set **DEBUG_CH_NUM** `[1:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_debug_ch_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GdmaCtrl::DEBUG_CH_NUM_MASK)
                | ((value << GdmaCtrl::DEBUG_CH_NUM_OFFSET) & GdmaCtrl::DEBUG_CH_NUM_MASK),
        }
    }
}

impl Default for GdmaCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GdmaCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GdmaCtrlVal {
        GdmaCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GdmaCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GdmaCtrlVal) -> GdmaCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GMAC_CTRL0 Register
///
/// N/A
///
/// Address offset: 0x014c
///
/// ## Fields
///
/// - **GMAC_RST_CLK_RX_N** `[7]` (RO)
///
///   N/A
///
/// - **GMAC_RST_CLK_TX_N** `[6]` (RO)
///
///   N/A
///
/// - **GMAC_MEM_CLK_FORCE_ON** `[5]` (RW)
///
///   N/A
///
/// - **PHY_INTF_SEL** `[4:2]` (RW)
///
///   N/A
///
/// - **SBD_FLOWCTRL** `[1]` (RW)
///
///   N/A
///
/// - **PTP_PPS** `[0]` (RO)
///
///   N/A
#[repr(transparent)]
pub struct GmacCtrl0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GmacCtrl0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GmacCtrl0 {
    // ======== Field Constants ========

    /// **GMAC_RST_CLK_RX_N**
    ///
    /// N/A
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const GMAC_RST_CLK_RX_N_OFFSET: u32 = 7;
    pub const GMAC_RST_CLK_RX_N_MASK: u32 = 0x00000080;
    pub const GMAC_RST_CLK_RX_N: u32 = 1 << 7;

    /// **GMAC_RST_CLK_TX_N**
    ///
    /// N/A
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const GMAC_RST_CLK_TX_N_OFFSET: u32 = 6;
    pub const GMAC_RST_CLK_TX_N_MASK: u32 = 0x00000040;
    pub const GMAC_RST_CLK_TX_N: u32 = 1 << 6;

    /// **GMAC_MEM_CLK_FORCE_ON**
    ///
    /// N/A
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const GMAC_MEM_CLK_FORCE_ON_OFFSET: u32 = 5;
    pub const GMAC_MEM_CLK_FORCE_ON_MASK: u32 = 0x00000020;
    pub const GMAC_MEM_CLK_FORCE_ON: u32 = 1 << 5;

    /// **PHY_INTF_SEL**
    ///
    /// N/A
    ///
    /// Bits `[4:2]`, mask `0x0000001c`
    pub const PHY_INTF_SEL_OFFSET: u32 = 2;
    pub const PHY_INTF_SEL_MASK: u32 = 0x0000001c;

    /// **SBD_FLOWCTRL**
    ///
    /// N/A
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SBD_FLOWCTRL_OFFSET: u32 = 1;
    pub const SBD_FLOWCTRL_MASK: u32 = 0x00000002;
    pub const SBD_FLOWCTRL: u32 = 1 << 1;

    /// **PTP_PPS**
    ///
    /// N/A
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PTP_PPS_OFFSET: u32 = 0;
    pub const PTP_PPS_MASK: u32 = 0x00000001;
    pub const PTP_PPS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **GMAC_RST_CLK_RX_N** `[7]` (RO)
    ///
    /// N/A
    #[inline(always)]
    pub fn gmac_rst_clk_rx_n(&self) -> bool {
        (self.value.read_bits() & Self::GMAC_RST_CLK_RX_N_MASK) != 0
    }

    /// **GMAC_RST_CLK_TX_N** `[6]` (RO)
    ///
    /// N/A
    #[inline(always)]
    pub fn gmac_rst_clk_tx_n(&self) -> bool {
        (self.value.read_bits() & Self::GMAC_RST_CLK_TX_N_MASK) != 0
    }

    /// **GMAC_MEM_CLK_FORCE_ON** `[5]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn gmac_mem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::GMAC_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set GMAC_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gmac_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::GMAC_MEM_CLK_FORCE_ON_MASK);
    }

    /// Clear GMAC_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gmac_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::GMAC_MEM_CLK_FORCE_ON_MASK);
    }

    /// **PHY_INTF_SEL** `[4:2]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn phy_intf_sel(&self) -> u32 {
        (self.value.read_bits() & Self::PHY_INTF_SEL_MASK) >> Self::PHY_INTF_SEL_OFFSET
    }

    /// Write **PHY_INTF_SEL** `[4:2]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_phy_intf_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PHY_INTF_SEL_MASK)
                | ((value << Self::PHY_INTF_SEL_OFFSET) & Self::PHY_INTF_SEL_MASK)
        });
    }

    /// **SBD_FLOWCTRL** `[1]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn sbd_flowctrl(&self) -> bool {
        (self.value.read_bits() & Self::SBD_FLOWCTRL_MASK) != 0
    }

    /// Set SBD_FLOWCTRL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sbd_flowctrl(&self) {
        self.value.modify_bits(|r| r | Self::SBD_FLOWCTRL_MASK);
    }

    /// Clear SBD_FLOWCTRL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sbd_flowctrl(&self) {
        self.value.modify_bits(|r| r & !Self::SBD_FLOWCTRL_MASK);
    }

    /// **PTP_PPS** `[0]` (RO)
    ///
    /// N/A
    #[inline(always)]
    pub fn ptp_pps(&self) -> bool {
        (self.value.read_bits() & Self::PTP_PPS_MASK) != 0
    }
}

/// GMAC_CTRL0 value type for batch read/modify/write operations
///
/// Use `gmacctrl0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gmacctrl0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GmacCtrl0Val {
    bits: u32,
}

impl GmacCtrl0Val {
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

    /// Read **GMAC_RST_CLK_RX_N** `[7:7]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn gmac_rst_clk_rx_n(&self) -> bool {
        (self.bits & GmacCtrl0::GMAC_RST_CLK_RX_N_MASK) != 0
    }

    /// Read **GMAC_RST_CLK_TX_N** `[6:6]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn gmac_rst_clk_tx_n(&self) -> bool {
        (self.bits & GmacCtrl0::GMAC_RST_CLK_TX_N_MASK) != 0
    }

    /// Read **GMAC_MEM_CLK_FORCE_ON** `[5:5]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn gmac_mem_clk_force_on(&self) -> bool {
        (self.bits & GmacCtrl0::GMAC_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **PHY_INTF_SEL** `[4:2]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn phy_intf_sel(&self) -> u32 {
        (self.bits & GmacCtrl0::PHY_INTF_SEL_MASK) >> GmacCtrl0::PHY_INTF_SEL_OFFSET
    }

    /// Read **SBD_FLOWCTRL** `[1:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn sbd_flowctrl(&self) -> bool {
        (self.bits & GmacCtrl0::SBD_FLOWCTRL_MASK) != 0
    }

    /// Read **PTP_PPS** `[0:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn ptp_pps(&self) -> bool {
        (self.bits & GmacCtrl0::PTP_PPS_MASK) != 0
    }

    /// Set **GMAC_MEM_CLK_FORCE_ON** `[5:5]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_gmac_mem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GmacCtrl0::GMAC_MEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !GmacCtrl0::GMAC_MEM_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **PHY_INTF_SEL** `[4:2]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_phy_intf_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GmacCtrl0::PHY_INTF_SEL_MASK)
                | ((value << GmacCtrl0::PHY_INTF_SEL_OFFSET) & GmacCtrl0::PHY_INTF_SEL_MASK),
        }
    }

    /// Set **SBD_FLOWCTRL** `[1:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_sbd_flowctrl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | GmacCtrl0::SBD_FLOWCTRL_MASK
            } else {
                self.bits & !GmacCtrl0::SBD_FLOWCTRL_MASK
            },
        }
    }
}

impl Default for GmacCtrl0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GmacCtrl0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GmacCtrl0Val {
        GmacCtrl0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GmacCtrl0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GmacCtrl0Val) -> GmacCtrl0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GMAC_CTRL1 Register
///
/// N/A
///
/// Address offset: 0x0150
///
/// ## Fields
///
/// - **PTP_TIMESTAMP_L** `[31:0]` (RO)
///
///   N/A
#[repr(transparent)]
pub struct GmacCtrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GmacCtrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GmacCtrl1 {
    // ======== Field Constants ========

    /// **PTP_TIMESTAMP_L**
    ///
    /// N/A
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PTP_TIMESTAMP_L_OFFSET: u32 = 0;
    pub const PTP_TIMESTAMP_L_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PTP_TIMESTAMP_L** `[31:0]` (RO)
    ///
    /// N/A
    #[inline(always)]
    pub fn ptp_timestamp_l(&self) -> u32 {
        (self.value.read_bits() & Self::PTP_TIMESTAMP_L_MASK) >> Self::PTP_TIMESTAMP_L_OFFSET
    }
}

/// GMAC_CTRL1 value type for batch read/modify/write operations
///
/// Use `gmacctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gmacctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GmacCtrl1Val {
    bits: u32,
}

impl GmacCtrl1Val {
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

    /// Read **PTP_TIMESTAMP_L** `[31:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn ptp_timestamp_l(&self) -> u32 {
        (self.bits & GmacCtrl1::PTP_TIMESTAMP_L_MASK) >> GmacCtrl1::PTP_TIMESTAMP_L_OFFSET
    }
}

impl Default for GmacCtrl1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GmacCtrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GmacCtrl1Val {
        GmacCtrl1Val::from_bits(self.value.read_bits())
    }
}

/// GMAC_CTRL2 Register
///
/// N/A
///
/// Address offset: 0x0154
///
/// ## Fields
///
/// - **PTP_TIMESTAMP_H** `[31:0]` (RO)
///
///   N/A
#[repr(transparent)]
pub struct GmacCtrl2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GmacCtrl2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GmacCtrl2 {
    // ======== Field Constants ========

    /// **PTP_TIMESTAMP_H**
    ///
    /// N/A
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PTP_TIMESTAMP_H_OFFSET: u32 = 0;
    pub const PTP_TIMESTAMP_H_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PTP_TIMESTAMP_H** `[31:0]` (RO)
    ///
    /// N/A
    #[inline(always)]
    pub fn ptp_timestamp_h(&self) -> u32 {
        (self.value.read_bits() & Self::PTP_TIMESTAMP_H_MASK) >> Self::PTP_TIMESTAMP_H_OFFSET
    }
}

/// GMAC_CTRL2 value type for batch read/modify/write operations
///
/// Use `gmacctrl2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gmacctrl2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GmacCtrl2Val {
    bits: u32,
}

impl GmacCtrl2Val {
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

    /// Read **PTP_TIMESTAMP_H** `[31:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn ptp_timestamp_h(&self) -> u32 {
        (self.bits & GmacCtrl2::PTP_TIMESTAMP_H_MASK) >> GmacCtrl2::PTP_TIMESTAMP_H_OFFSET
    }
}

impl Default for GmacCtrl2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GmacCtrl2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GmacCtrl2Val {
        GmacCtrl2Val::from_bits(self.value.read_bits())
    }
}

/// VPU_CTRL Register
///
/// N/A
///
/// Address offset: 0x0158
///
/// ## Fields
///
/// - **DMA2D_LSLP_MEM_PD** `[4]` (RW)
///
///   N/A
///
/// - **JPEG_DSLP_MEM_PD** `[3]` (RW)
///
///   N/A
///
/// - **JPEG_LSLP_MEM_PD** `[2]` (RW)
///
///   N/A
///
/// - **JPEG_SDSLP_MEM_PD** `[1]` (RW)
///
///   N/A
///
/// - **PPA_LSLP_MEM_PD** `[0]` (RW)
///
///   N/A
#[repr(transparent)]
pub struct VpuCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for VpuCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl VpuCtrl {
    // ======== Field Constants ========

    /// **DMA2D_LSLP_MEM_PD**
    ///
    /// N/A
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DMA2D_LSLP_MEM_PD_OFFSET: u32 = 4;
    pub const DMA2D_LSLP_MEM_PD_MASK: u32 = 0x00000010;
    pub const DMA2D_LSLP_MEM_PD: u32 = 1 << 4;

    /// **JPEG_DSLP_MEM_PD**
    ///
    /// N/A
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const JPEG_DSLP_MEM_PD_OFFSET: u32 = 3;
    pub const JPEG_DSLP_MEM_PD_MASK: u32 = 0x00000008;
    pub const JPEG_DSLP_MEM_PD: u32 = 1 << 3;

    /// **JPEG_LSLP_MEM_PD**
    ///
    /// N/A
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const JPEG_LSLP_MEM_PD_OFFSET: u32 = 2;
    pub const JPEG_LSLP_MEM_PD_MASK: u32 = 0x00000004;
    pub const JPEG_LSLP_MEM_PD: u32 = 1 << 2;

    /// **JPEG_SDSLP_MEM_PD**
    ///
    /// N/A
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const JPEG_SDSLP_MEM_PD_OFFSET: u32 = 1;
    pub const JPEG_SDSLP_MEM_PD_MASK: u32 = 0x00000002;
    pub const JPEG_SDSLP_MEM_PD: u32 = 1 << 1;

    /// **PPA_LSLP_MEM_PD**
    ///
    /// N/A
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PPA_LSLP_MEM_PD_OFFSET: u32 = 0;
    pub const PPA_LSLP_MEM_PD_MASK: u32 = 0x00000001;
    pub const PPA_LSLP_MEM_PD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DMA2D_LSLP_MEM_PD** `[4]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn dma2d_lslp_mem_pd(&self) -> bool {
        (self.value.read_bits() & Self::DMA2D_LSLP_MEM_PD_MASK) != 0
    }

    /// Set DMA2D_LSLP_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma2d_lslp_mem_pd(&self) {
        self.value.modify_bits(|r| r | Self::DMA2D_LSLP_MEM_PD_MASK);
    }

    /// Clear DMA2D_LSLP_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma2d_lslp_mem_pd(&self) {
        self.value
            .modify_bits(|r| r & !Self::DMA2D_LSLP_MEM_PD_MASK);
    }

    /// **JPEG_DSLP_MEM_PD** `[3]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn jpeg_dslp_mem_pd(&self) -> bool {
        (self.value.read_bits() & Self::JPEG_DSLP_MEM_PD_MASK) != 0
    }

    /// Set JPEG_DSLP_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_jpeg_dslp_mem_pd(&self) {
        self.value.modify_bits(|r| r | Self::JPEG_DSLP_MEM_PD_MASK);
    }

    /// Clear JPEG_DSLP_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_jpeg_dslp_mem_pd(&self) {
        self.value.modify_bits(|r| r & !Self::JPEG_DSLP_MEM_PD_MASK);
    }

    /// **JPEG_LSLP_MEM_PD** `[2]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn jpeg_lslp_mem_pd(&self) -> bool {
        (self.value.read_bits() & Self::JPEG_LSLP_MEM_PD_MASK) != 0
    }

    /// Set JPEG_LSLP_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_jpeg_lslp_mem_pd(&self) {
        self.value.modify_bits(|r| r | Self::JPEG_LSLP_MEM_PD_MASK);
    }

    /// Clear JPEG_LSLP_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_jpeg_lslp_mem_pd(&self) {
        self.value.modify_bits(|r| r & !Self::JPEG_LSLP_MEM_PD_MASK);
    }

    /// **JPEG_SDSLP_MEM_PD** `[1]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn jpeg_sdslp_mem_pd(&self) -> bool {
        (self.value.read_bits() & Self::JPEG_SDSLP_MEM_PD_MASK) != 0
    }

    /// Set JPEG_SDSLP_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_jpeg_sdslp_mem_pd(&self) {
        self.value.modify_bits(|r| r | Self::JPEG_SDSLP_MEM_PD_MASK);
    }

    /// Clear JPEG_SDSLP_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_jpeg_sdslp_mem_pd(&self) {
        self.value
            .modify_bits(|r| r & !Self::JPEG_SDSLP_MEM_PD_MASK);
    }

    /// **PPA_LSLP_MEM_PD** `[0]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn ppa_lslp_mem_pd(&self) -> bool {
        (self.value.read_bits() & Self::PPA_LSLP_MEM_PD_MASK) != 0
    }

    /// Set PPA_LSLP_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ppa_lslp_mem_pd(&self) {
        self.value.modify_bits(|r| r | Self::PPA_LSLP_MEM_PD_MASK);
    }

    /// Clear PPA_LSLP_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ppa_lslp_mem_pd(&self) {
        self.value.modify_bits(|r| r & !Self::PPA_LSLP_MEM_PD_MASK);
    }
}

/// VPU_CTRL value type for batch read/modify/write operations
///
/// Use `vpuctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `vpuctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct VpuCtrlVal {
    bits: u32,
}

impl VpuCtrlVal {
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

    /// Read **DMA2D_LSLP_MEM_PD** `[4:4]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn dma2d_lslp_mem_pd(&self) -> bool {
        (self.bits & VpuCtrl::DMA2D_LSLP_MEM_PD_MASK) != 0
    }

    /// Read **JPEG_DSLP_MEM_PD** `[3:3]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn jpeg_dslp_mem_pd(&self) -> bool {
        (self.bits & VpuCtrl::JPEG_DSLP_MEM_PD_MASK) != 0
    }

    /// Read **JPEG_LSLP_MEM_PD** `[2:2]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn jpeg_lslp_mem_pd(&self) -> bool {
        (self.bits & VpuCtrl::JPEG_LSLP_MEM_PD_MASK) != 0
    }

    /// Read **JPEG_SDSLP_MEM_PD** `[1:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn jpeg_sdslp_mem_pd(&self) -> bool {
        (self.bits & VpuCtrl::JPEG_SDSLP_MEM_PD_MASK) != 0
    }

    /// Read **PPA_LSLP_MEM_PD** `[0:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn ppa_lslp_mem_pd(&self) -> bool {
        (self.bits & VpuCtrl::PPA_LSLP_MEM_PD_MASK) != 0
    }

    /// Set **DMA2D_LSLP_MEM_PD** `[4:4]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_dma2d_lslp_mem_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VpuCtrl::DMA2D_LSLP_MEM_PD_MASK
            } else {
                self.bits & !VpuCtrl::DMA2D_LSLP_MEM_PD_MASK
            },
        }
    }

    /// Set **JPEG_DSLP_MEM_PD** `[3:3]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_jpeg_dslp_mem_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VpuCtrl::JPEG_DSLP_MEM_PD_MASK
            } else {
                self.bits & !VpuCtrl::JPEG_DSLP_MEM_PD_MASK
            },
        }
    }

    /// Set **JPEG_LSLP_MEM_PD** `[2:2]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_jpeg_lslp_mem_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VpuCtrl::JPEG_LSLP_MEM_PD_MASK
            } else {
                self.bits & !VpuCtrl::JPEG_LSLP_MEM_PD_MASK
            },
        }
    }

    /// Set **JPEG_SDSLP_MEM_PD** `[1:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_jpeg_sdslp_mem_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VpuCtrl::JPEG_SDSLP_MEM_PD_MASK
            } else {
                self.bits & !VpuCtrl::JPEG_SDSLP_MEM_PD_MASK
            },
        }
    }

    /// Set **PPA_LSLP_MEM_PD** `[0:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_ppa_lslp_mem_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | VpuCtrl::PPA_LSLP_MEM_PD_MASK
            } else {
                self.bits & !VpuCtrl::PPA_LSLP_MEM_PD_MASK
            },
        }
    }
}

impl Default for VpuCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl VpuCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> VpuCtrlVal {
        VpuCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: VpuCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(VpuCtrlVal) -> VpuCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USBOTG20_CTRL Register
///
/// N/A
///
/// Address offset: 0x015c
/// Reset value: 0x00822640
///
/// ## Fields
///
/// - **OTG_PHY_BISTEN** `[24]` (RW)
///
///   N/A
///
/// - **OTG_PHY_REFCLK_MODE** `[23]` (RW)
///
///   N/A
///
/// - **OTG_PHY_TXBITSTUFF_EN** `[22]` (RW)
///
///   N/A
///
/// - **OTG_SUSPENDM** `[21]` (RW)
///
///   N/A
///
/// - **PHY_PLL_EN** `[20]` (RW)
///
///   N/A
///
/// - **PHY_PLL_FORCE_EN** `[19]` (RW)
///
///   N/A
///
/// - **PHY_RESET_FORCE_EN** `[18]` (RW)
///
///   N/A
///
/// - **PHY_RSTN** `[17]` (RW)
///
///   N/A
///
/// - **PHY_SUSPEND_FORCE_EN** `[16]` (RW)
///
///   N/A
///
/// - **PHY_SUSPENDM** `[15]` (RW)
///
///   N/A
///
/// - **USB_MEM_AUX_CTRL** `[14:1]` (RW)
///
///   N/A
///
/// - **OTG_PHY_TEST_DONE** `[0]` (RO)
///
///   N/A
#[repr(transparent)]
pub struct Usbotg20Ctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Usbotg20Ctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Usbotg20Ctrl {
    // ======== Field Constants ========

    /// **OTG_PHY_BISTEN**
    ///
    /// N/A
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const OTG_PHY_BISTEN_OFFSET: u32 = 24;
    pub const OTG_PHY_BISTEN_MASK: u32 = 0x01000000;
    pub const OTG_PHY_BISTEN: u32 = 1 << 24;

    /// **OTG_PHY_REFCLK_MODE**
    ///
    /// N/A
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const OTG_PHY_REFCLK_MODE_OFFSET: u32 = 23;
    pub const OTG_PHY_REFCLK_MODE_MASK: u32 = 0x00800000;
    pub const OTG_PHY_REFCLK_MODE: u32 = 1 << 23;

    /// **OTG_PHY_TXBITSTUFF_EN**
    ///
    /// N/A
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const OTG_PHY_TXBITSTUFF_EN_OFFSET: u32 = 22;
    pub const OTG_PHY_TXBITSTUFF_EN_MASK: u32 = 0x00400000;
    pub const OTG_PHY_TXBITSTUFF_EN: u32 = 1 << 22;

    /// **OTG_SUSPENDM**
    ///
    /// N/A
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const OTG_SUSPENDM_OFFSET: u32 = 21;
    pub const OTG_SUSPENDM_MASK: u32 = 0x00200000;
    pub const OTG_SUSPENDM: u32 = 1 << 21;

    /// **PHY_PLL_EN**
    ///
    /// N/A
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const PHY_PLL_EN_OFFSET: u32 = 20;
    pub const PHY_PLL_EN_MASK: u32 = 0x00100000;
    pub const PHY_PLL_EN: u32 = 1 << 20;

    /// **PHY_PLL_FORCE_EN**
    ///
    /// N/A
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const PHY_PLL_FORCE_EN_OFFSET: u32 = 19;
    pub const PHY_PLL_FORCE_EN_MASK: u32 = 0x00080000;
    pub const PHY_PLL_FORCE_EN: u32 = 1 << 19;

    /// **PHY_RESET_FORCE_EN**
    ///
    /// N/A
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const PHY_RESET_FORCE_EN_OFFSET: u32 = 18;
    pub const PHY_RESET_FORCE_EN_MASK: u32 = 0x00040000;
    pub const PHY_RESET_FORCE_EN: u32 = 1 << 18;

    /// **PHY_RSTN**
    ///
    /// N/A
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const PHY_RSTN_OFFSET: u32 = 17;
    pub const PHY_RSTN_MASK: u32 = 0x00020000;
    pub const PHY_RSTN: u32 = 1 << 17;

    /// **PHY_SUSPEND_FORCE_EN**
    ///
    /// N/A
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const PHY_SUSPEND_FORCE_EN_OFFSET: u32 = 16;
    pub const PHY_SUSPEND_FORCE_EN_MASK: u32 = 0x00010000;
    pub const PHY_SUSPEND_FORCE_EN: u32 = 1 << 16;

    /// **PHY_SUSPENDM**
    ///
    /// N/A
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const PHY_SUSPENDM_OFFSET: u32 = 15;
    pub const PHY_SUSPENDM_MASK: u32 = 0x00008000;
    pub const PHY_SUSPENDM: u32 = 1 << 15;

    /// **USB_MEM_AUX_CTRL**
    ///
    /// N/A
    ///
    /// Bits `[14:1]`, mask `0x00007ffe`
    pub const USB_MEM_AUX_CTRL_OFFSET: u32 = 1;
    pub const USB_MEM_AUX_CTRL_MASK: u32 = 0x00007ffe;

    /// **OTG_PHY_TEST_DONE**
    ///
    /// N/A
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const OTG_PHY_TEST_DONE_OFFSET: u32 = 0;
    pub const OTG_PHY_TEST_DONE_MASK: u32 = 0x00000001;
    pub const OTG_PHY_TEST_DONE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **OTG_PHY_BISTEN** `[24]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn otg_phy_bisten(&self) -> bool {
        (self.value.read_bits() & Self::OTG_PHY_BISTEN_MASK) != 0
    }

    /// Set OTG_PHY_BISTEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_otg_phy_bisten(&self) {
        self.value.modify_bits(|r| r | Self::OTG_PHY_BISTEN_MASK);
    }

    /// Clear OTG_PHY_BISTEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_otg_phy_bisten(&self) {
        self.value.modify_bits(|r| r & !Self::OTG_PHY_BISTEN_MASK);
    }

    /// **OTG_PHY_REFCLK_MODE** `[23]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn otg_phy_refclk_mode(&self) -> bool {
        (self.value.read_bits() & Self::OTG_PHY_REFCLK_MODE_MASK) != 0
    }

    /// Set OTG_PHY_REFCLK_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_otg_phy_refclk_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::OTG_PHY_REFCLK_MODE_MASK);
    }

    /// Clear OTG_PHY_REFCLK_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_otg_phy_refclk_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::OTG_PHY_REFCLK_MODE_MASK);
    }

    /// **OTG_PHY_TXBITSTUFF_EN** `[22]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn otg_phy_txbitstuff_en(&self) -> bool {
        (self.value.read_bits() & Self::OTG_PHY_TXBITSTUFF_EN_MASK) != 0
    }

    /// Set OTG_PHY_TXBITSTUFF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_otg_phy_txbitstuff_en(&self) {
        self.value
            .modify_bits(|r| r | Self::OTG_PHY_TXBITSTUFF_EN_MASK);
    }

    /// Clear OTG_PHY_TXBITSTUFF_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_otg_phy_txbitstuff_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::OTG_PHY_TXBITSTUFF_EN_MASK);
    }

    /// **OTG_SUSPENDM** `[21]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn otg_suspendm(&self) -> bool {
        (self.value.read_bits() & Self::OTG_SUSPENDM_MASK) != 0
    }

    /// Set OTG_SUSPENDM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_otg_suspendm(&self) {
        self.value.modify_bits(|r| r | Self::OTG_SUSPENDM_MASK);
    }

    /// Clear OTG_SUSPENDM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_otg_suspendm(&self) {
        self.value.modify_bits(|r| r & !Self::OTG_SUSPENDM_MASK);
    }

    /// **PHY_PLL_EN** `[20]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn phy_pll_en(&self) -> bool {
        (self.value.read_bits() & Self::PHY_PLL_EN_MASK) != 0
    }

    /// Set PHY_PLL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_pll_en(&self) {
        self.value.modify_bits(|r| r | Self::PHY_PLL_EN_MASK);
    }

    /// Clear PHY_PLL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_pll_en(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_PLL_EN_MASK);
    }

    /// **PHY_PLL_FORCE_EN** `[19]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn phy_pll_force_en(&self) -> bool {
        (self.value.read_bits() & Self::PHY_PLL_FORCE_EN_MASK) != 0
    }

    /// Set PHY_PLL_FORCE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_pll_force_en(&self) {
        self.value.modify_bits(|r| r | Self::PHY_PLL_FORCE_EN_MASK);
    }

    /// Clear PHY_PLL_FORCE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_pll_force_en(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_PLL_FORCE_EN_MASK);
    }

    /// **PHY_RESET_FORCE_EN** `[18]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn phy_reset_force_en(&self) -> bool {
        (self.value.read_bits() & Self::PHY_RESET_FORCE_EN_MASK) != 0
    }

    /// Set PHY_RESET_FORCE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_reset_force_en(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_RESET_FORCE_EN_MASK);
    }

    /// Clear PHY_RESET_FORCE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_reset_force_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_RESET_FORCE_EN_MASK);
    }

    /// **PHY_RSTN** `[17]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn phy_rstn(&self) -> bool {
        (self.value.read_bits() & Self::PHY_RSTN_MASK) != 0
    }

    /// Set PHY_RSTN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_rstn(&self) {
        self.value.modify_bits(|r| r | Self::PHY_RSTN_MASK);
    }

    /// Clear PHY_RSTN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_rstn(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_RSTN_MASK);
    }

    /// **PHY_SUSPEND_FORCE_EN** `[16]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn phy_suspend_force_en(&self) -> bool {
        (self.value.read_bits() & Self::PHY_SUSPEND_FORCE_EN_MASK) != 0
    }

    /// Set PHY_SUSPEND_FORCE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_suspend_force_en(&self) {
        self.value
            .modify_bits(|r| r | Self::PHY_SUSPEND_FORCE_EN_MASK);
    }

    /// Clear PHY_SUSPEND_FORCE_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_suspend_force_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::PHY_SUSPEND_FORCE_EN_MASK);
    }

    /// **PHY_SUSPENDM** `[15]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn phy_suspendm(&self) -> bool {
        (self.value.read_bits() & Self::PHY_SUSPENDM_MASK) != 0
    }

    /// Set PHY_SUSPENDM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_suspendm(&self) {
        self.value.modify_bits(|r| r | Self::PHY_SUSPENDM_MASK);
    }

    /// Clear PHY_SUSPENDM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_suspendm(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_SUSPENDM_MASK);
    }

    /// **USB_MEM_AUX_CTRL** `[14:1]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn usb_mem_aux_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::USB_MEM_AUX_CTRL_MASK) >> Self::USB_MEM_AUX_CTRL_OFFSET
    }

    /// Write **USB_MEM_AUX_CTRL** `[14:1]` (RW)
    ///
    /// N/A
    #[inline(always)]
    pub fn write_usb_mem_aux_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::USB_MEM_AUX_CTRL_MASK)
                | ((value << Self::USB_MEM_AUX_CTRL_OFFSET) & Self::USB_MEM_AUX_CTRL_MASK)
        });
    }

    /// **OTG_PHY_TEST_DONE** `[0]` (RO)
    ///
    /// N/A
    #[inline(always)]
    pub fn otg_phy_test_done(&self) -> bool {
        (self.value.read_bits() & Self::OTG_PHY_TEST_DONE_MASK) != 0
    }
}

/// USBOTG20_CTRL value type for batch read/modify/write operations
///
/// Use `usbotg20ctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbotg20ctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Usbotg20CtrlVal {
    bits: u32,
}

impl Usbotg20CtrlVal {
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
        Self { bits: 0x00822640 }
    }

    /// Read **OTG_PHY_BISTEN** `[24:24]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn otg_phy_bisten(&self) -> bool {
        (self.bits & Usbotg20Ctrl::OTG_PHY_BISTEN_MASK) != 0
    }

    /// Read **OTG_PHY_REFCLK_MODE** `[23:23]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn otg_phy_refclk_mode(&self) -> bool {
        (self.bits & Usbotg20Ctrl::OTG_PHY_REFCLK_MODE_MASK) != 0
    }

    /// Read **OTG_PHY_TXBITSTUFF_EN** `[22:22]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn otg_phy_txbitstuff_en(&self) -> bool {
        (self.bits & Usbotg20Ctrl::OTG_PHY_TXBITSTUFF_EN_MASK) != 0
    }

    /// Read **OTG_SUSPENDM** `[21:21]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn otg_suspendm(&self) -> bool {
        (self.bits & Usbotg20Ctrl::OTG_SUSPENDM_MASK) != 0
    }

    /// Read **PHY_PLL_EN** `[20:20]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn phy_pll_en(&self) -> bool {
        (self.bits & Usbotg20Ctrl::PHY_PLL_EN_MASK) != 0
    }

    /// Read **PHY_PLL_FORCE_EN** `[19:19]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn phy_pll_force_en(&self) -> bool {
        (self.bits & Usbotg20Ctrl::PHY_PLL_FORCE_EN_MASK) != 0
    }

    /// Read **PHY_RESET_FORCE_EN** `[18:18]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn phy_reset_force_en(&self) -> bool {
        (self.bits & Usbotg20Ctrl::PHY_RESET_FORCE_EN_MASK) != 0
    }

    /// Read **PHY_RSTN** `[17:17]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn phy_rstn(&self) -> bool {
        (self.bits & Usbotg20Ctrl::PHY_RSTN_MASK) != 0
    }

    /// Read **PHY_SUSPEND_FORCE_EN** `[16:16]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn phy_suspend_force_en(&self) -> bool {
        (self.bits & Usbotg20Ctrl::PHY_SUSPEND_FORCE_EN_MASK) != 0
    }

    /// Read **PHY_SUSPENDM** `[15:15]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn phy_suspendm(&self) -> bool {
        (self.bits & Usbotg20Ctrl::PHY_SUSPENDM_MASK) != 0
    }

    /// Read **USB_MEM_AUX_CTRL** `[14:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn usb_mem_aux_ctrl(&self) -> u32 {
        (self.bits & Usbotg20Ctrl::USB_MEM_AUX_CTRL_MASK) >> Usbotg20Ctrl::USB_MEM_AUX_CTRL_OFFSET
    }

    /// Read **OTG_PHY_TEST_DONE** `[0:0]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn otg_phy_test_done(&self) -> bool {
        (self.bits & Usbotg20Ctrl::OTG_PHY_TEST_DONE_MASK) != 0
    }

    /// Set **OTG_PHY_BISTEN** `[24:24]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_otg_phy_bisten(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbotg20Ctrl::OTG_PHY_BISTEN_MASK
            } else {
                self.bits & !Usbotg20Ctrl::OTG_PHY_BISTEN_MASK
            },
        }
    }

    /// Set **OTG_PHY_REFCLK_MODE** `[23:23]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_otg_phy_refclk_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbotg20Ctrl::OTG_PHY_REFCLK_MODE_MASK
            } else {
                self.bits & !Usbotg20Ctrl::OTG_PHY_REFCLK_MODE_MASK
            },
        }
    }

    /// Set **OTG_PHY_TXBITSTUFF_EN** `[22:22]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_otg_phy_txbitstuff_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbotg20Ctrl::OTG_PHY_TXBITSTUFF_EN_MASK
            } else {
                self.bits & !Usbotg20Ctrl::OTG_PHY_TXBITSTUFF_EN_MASK
            },
        }
    }

    /// Set **OTG_SUSPENDM** `[21:21]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_otg_suspendm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbotg20Ctrl::OTG_SUSPENDM_MASK
            } else {
                self.bits & !Usbotg20Ctrl::OTG_SUSPENDM_MASK
            },
        }
    }

    /// Set **PHY_PLL_EN** `[20:20]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_phy_pll_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbotg20Ctrl::PHY_PLL_EN_MASK
            } else {
                self.bits & !Usbotg20Ctrl::PHY_PLL_EN_MASK
            },
        }
    }

    /// Set **PHY_PLL_FORCE_EN** `[19:19]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_phy_pll_force_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbotg20Ctrl::PHY_PLL_FORCE_EN_MASK
            } else {
                self.bits & !Usbotg20Ctrl::PHY_PLL_FORCE_EN_MASK
            },
        }
    }

    /// Set **PHY_RESET_FORCE_EN** `[18:18]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_phy_reset_force_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbotg20Ctrl::PHY_RESET_FORCE_EN_MASK
            } else {
                self.bits & !Usbotg20Ctrl::PHY_RESET_FORCE_EN_MASK
            },
        }
    }

    /// Set **PHY_RSTN** `[17:17]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_phy_rstn(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbotg20Ctrl::PHY_RSTN_MASK
            } else {
                self.bits & !Usbotg20Ctrl::PHY_RSTN_MASK
            },
        }
    }

    /// Set **PHY_SUSPEND_FORCE_EN** `[16:16]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_phy_suspend_force_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbotg20Ctrl::PHY_SUSPEND_FORCE_EN_MASK
            } else {
                self.bits & !Usbotg20Ctrl::PHY_SUSPEND_FORCE_EN_MASK
            },
        }
    }

    /// Set **PHY_SUSPENDM** `[15:15]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_phy_suspendm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Usbotg20Ctrl::PHY_SUSPENDM_MASK
            } else {
                self.bits & !Usbotg20Ctrl::PHY_SUSPENDM_MASK
            },
        }
    }

    /// Set **USB_MEM_AUX_CTRL** `[14:1]`
    ///
    /// N/A
    #[inline(always)]
    pub const fn with_usb_mem_aux_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Usbotg20Ctrl::USB_MEM_AUX_CTRL_MASK)
                | ((value << Usbotg20Ctrl::USB_MEM_AUX_CTRL_OFFSET)
                    & Usbotg20Ctrl::USB_MEM_AUX_CTRL_MASK),
        }
    }
}

impl Default for Usbotg20CtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Usbotg20Ctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Usbotg20CtrlVal {
        Usbotg20CtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Usbotg20CtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Usbotg20CtrlVal) -> Usbotg20CtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TCM_ERR_RESP_CTRL Register
///
/// need_des
///
/// Address offset: 0x0160
///
/// ## Fields
///
/// - **TCM_ERR_RESP_EN** `[0]` (RW)
///
///   Set 1 to turn on tcm error response
#[repr(transparent)]
pub struct TcmErrRespCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TcmErrRespCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TcmErrRespCtrl {
    // ======== Field Constants ========

    /// **TCM_ERR_RESP_EN**
    ///
    /// Set 1 to turn on tcm error response
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TCM_ERR_RESP_EN_OFFSET: u32 = 0;
    pub const TCM_ERR_RESP_EN_MASK: u32 = 0x00000001;
    pub const TCM_ERR_RESP_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TCM_ERR_RESP_EN** `[0]` (RW)
    ///
    /// Set 1 to turn on tcm error response
    #[inline(always)]
    pub fn tcm_err_resp_en(&self) -> bool {
        (self.value.read_bits() & Self::TCM_ERR_RESP_EN_MASK) != 0
    }

    /// Set TCM_ERR_RESP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tcm_err_resp_en(&self) {
        self.value.modify_bits(|r| r | Self::TCM_ERR_RESP_EN_MASK);
    }

    /// Clear TCM_ERR_RESP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tcm_err_resp_en(&self) {
        self.value.modify_bits(|r| r & !Self::TCM_ERR_RESP_EN_MASK);
    }
}

/// TCM_ERR_RESP_CTRL value type for batch read/modify/write operations
///
/// Use `tcmerrrespctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcmerrrespctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcmErrRespCtrlVal {
    bits: u32,
}

impl TcmErrRespCtrlVal {
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

    /// Read **TCM_ERR_RESP_EN** `[0:0]`
    ///
    /// Set 1 to turn on tcm error response
    #[inline(always)]
    pub const fn tcm_err_resp_en(&self) -> bool {
        (self.bits & TcmErrRespCtrl::TCM_ERR_RESP_EN_MASK) != 0
    }

    /// Set **TCM_ERR_RESP_EN** `[0:0]`
    ///
    /// Set 1 to turn on tcm error response
    #[inline(always)]
    pub const fn with_tcm_err_resp_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TcmErrRespCtrl::TCM_ERR_RESP_EN_MASK
            } else {
                self.bits & !TcmErrRespCtrl::TCM_ERR_RESP_EN_MASK
            },
        }
    }
}

impl Default for TcmErrRespCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TcmErrRespCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcmErrRespCtrlVal {
        TcmErrRespCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TcmErrRespCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TcmErrRespCtrlVal) -> TcmErrRespCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L2_MEM_REFRESH Register
///
/// NA
///
/// Address offset: 0x0164
/// Reset value: 0x00000040
///
/// ## Fields
///
/// - **REG_L2_MEM_UNIT5_REFRESH_DONE** `[12]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_UNIT4_REFRESH_DONE** `[11]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_UNIT3_REFRESH_DONE** `[10]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_UNIT2_REFRESH_DONE** `[9]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_UNIT1_REFRESH_DONE** `[8]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_UNIT0_REFRESH_DONE** `[7]` (RO)
///
///   NA
///
/// - **REG_L2_MEM_REFERSH_CNT_RESET** `[6]` (RW)
///
///   Set 1 to reset l2mem_refresh_cnt
///
/// - **REG_L2_MEM_UNIT5_REFERSH_EN** `[5]` (RW)
///
///   NA
///
/// - **REG_L2_MEM_UNIT4_REFERSH_EN** `[4]` (RW)
///
///   NA
///
/// - **REG_L2_MEM_UNIT3_REFERSH_EN** `[3]` (RW)
///
///   NA
///
/// - **REG_L2_MEM_UNIT2_REFERSH_EN** `[2]` (RW)
///
///   NA
///
/// - **REG_L2_MEM_UNIT1_REFERSH_EN** `[1]` (RW)
///
///   NA
///
/// - **REG_L2_MEM_UNIT0_REFERSH_EN** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct L2MemRefresh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemRefresh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemRefresh {
    // ======== Field Constants ========

    /// **REG_L2_MEM_UNIT5_REFRESH_DONE**
    ///
    /// NA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const REG_L2_MEM_UNIT5_REFRESH_DONE_OFFSET: u32 = 12;
    pub const REG_L2_MEM_UNIT5_REFRESH_DONE_MASK: u32 = 0x00001000;
    pub const REG_L2_MEM_UNIT5_REFRESH_DONE: u32 = 1 << 12;

    /// **REG_L2_MEM_UNIT4_REFRESH_DONE**
    ///
    /// NA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const REG_L2_MEM_UNIT4_REFRESH_DONE_OFFSET: u32 = 11;
    pub const REG_L2_MEM_UNIT4_REFRESH_DONE_MASK: u32 = 0x00000800;
    pub const REG_L2_MEM_UNIT4_REFRESH_DONE: u32 = 1 << 11;

    /// **REG_L2_MEM_UNIT3_REFRESH_DONE**
    ///
    /// NA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const REG_L2_MEM_UNIT3_REFRESH_DONE_OFFSET: u32 = 10;
    pub const REG_L2_MEM_UNIT3_REFRESH_DONE_MASK: u32 = 0x00000400;
    pub const REG_L2_MEM_UNIT3_REFRESH_DONE: u32 = 1 << 10;

    /// **REG_L2_MEM_UNIT2_REFRESH_DONE**
    ///
    /// NA
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const REG_L2_MEM_UNIT2_REFRESH_DONE_OFFSET: u32 = 9;
    pub const REG_L2_MEM_UNIT2_REFRESH_DONE_MASK: u32 = 0x00000200;
    pub const REG_L2_MEM_UNIT2_REFRESH_DONE: u32 = 1 << 9;

    /// **REG_L2_MEM_UNIT1_REFRESH_DONE**
    ///
    /// NA
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const REG_L2_MEM_UNIT1_REFRESH_DONE_OFFSET: u32 = 8;
    pub const REG_L2_MEM_UNIT1_REFRESH_DONE_MASK: u32 = 0x00000100;
    pub const REG_L2_MEM_UNIT1_REFRESH_DONE: u32 = 1 << 8;

    /// **REG_L2_MEM_UNIT0_REFRESH_DONE**
    ///
    /// NA
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const REG_L2_MEM_UNIT0_REFRESH_DONE_OFFSET: u32 = 7;
    pub const REG_L2_MEM_UNIT0_REFRESH_DONE_MASK: u32 = 0x00000080;
    pub const REG_L2_MEM_UNIT0_REFRESH_DONE: u32 = 1 << 7;

    /// **REG_L2_MEM_REFERSH_CNT_RESET**
    ///
    /// Set 1 to reset l2mem_refresh_cnt
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const REG_L2_MEM_REFERSH_CNT_RESET_OFFSET: u32 = 6;
    pub const REG_L2_MEM_REFERSH_CNT_RESET_MASK: u32 = 0x00000040;
    pub const REG_L2_MEM_REFERSH_CNT_RESET: u32 = 1 << 6;

    /// **REG_L2_MEM_UNIT5_REFERSH_EN**
    ///
    /// NA
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const REG_L2_MEM_UNIT5_REFERSH_EN_OFFSET: u32 = 5;
    pub const REG_L2_MEM_UNIT5_REFERSH_EN_MASK: u32 = 0x00000020;
    pub const REG_L2_MEM_UNIT5_REFERSH_EN: u32 = 1 << 5;

    /// **REG_L2_MEM_UNIT4_REFERSH_EN**
    ///
    /// NA
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const REG_L2_MEM_UNIT4_REFERSH_EN_OFFSET: u32 = 4;
    pub const REG_L2_MEM_UNIT4_REFERSH_EN_MASK: u32 = 0x00000010;
    pub const REG_L2_MEM_UNIT4_REFERSH_EN: u32 = 1 << 4;

    /// **REG_L2_MEM_UNIT3_REFERSH_EN**
    ///
    /// NA
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const REG_L2_MEM_UNIT3_REFERSH_EN_OFFSET: u32 = 3;
    pub const REG_L2_MEM_UNIT3_REFERSH_EN_MASK: u32 = 0x00000008;
    pub const REG_L2_MEM_UNIT3_REFERSH_EN: u32 = 1 << 3;

    /// **REG_L2_MEM_UNIT2_REFERSH_EN**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_L2_MEM_UNIT2_REFERSH_EN_OFFSET: u32 = 2;
    pub const REG_L2_MEM_UNIT2_REFERSH_EN_MASK: u32 = 0x00000004;
    pub const REG_L2_MEM_UNIT2_REFERSH_EN: u32 = 1 << 2;

    /// **REG_L2_MEM_UNIT1_REFERSH_EN**
    ///
    /// NA
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_L2_MEM_UNIT1_REFERSH_EN_OFFSET: u32 = 1;
    pub const REG_L2_MEM_UNIT1_REFERSH_EN_MASK: u32 = 0x00000002;
    pub const REG_L2_MEM_UNIT1_REFERSH_EN: u32 = 1 << 1;

    /// **REG_L2_MEM_UNIT0_REFERSH_EN**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_L2_MEM_UNIT0_REFERSH_EN_OFFSET: u32 = 0;
    pub const REG_L2_MEM_UNIT0_REFERSH_EN_MASK: u32 = 0x00000001;
    pub const REG_L2_MEM_UNIT0_REFERSH_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_L2_MEM_UNIT5_REFRESH_DONE** `[12]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_unit5_refresh_done(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_UNIT5_REFRESH_DONE_MASK) != 0
    }

    /// **REG_L2_MEM_UNIT4_REFRESH_DONE** `[11]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_unit4_refresh_done(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_UNIT4_REFRESH_DONE_MASK) != 0
    }

    /// **REG_L2_MEM_UNIT3_REFRESH_DONE** `[10]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_unit3_refresh_done(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_UNIT3_REFRESH_DONE_MASK) != 0
    }

    /// **REG_L2_MEM_UNIT2_REFRESH_DONE** `[9]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_unit2_refresh_done(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_UNIT2_REFRESH_DONE_MASK) != 0
    }

    /// **REG_L2_MEM_UNIT1_REFRESH_DONE** `[8]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_unit1_refresh_done(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_UNIT1_REFRESH_DONE_MASK) != 0
    }

    /// **REG_L2_MEM_UNIT0_REFRESH_DONE** `[7]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_unit0_refresh_done(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_UNIT0_REFRESH_DONE_MASK) != 0
    }

    /// **REG_L2_MEM_REFERSH_CNT_RESET** `[6]` (RW)
    ///
    /// Set 1 to reset l2mem_refresh_cnt
    #[inline(always)]
    pub fn reg_l2_mem_refersh_cnt_reset(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_REFERSH_CNT_RESET_MASK) != 0
    }

    /// Set REG_L2_MEM_REFERSH_CNT_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_refersh_cnt_reset(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_REFERSH_CNT_RESET_MASK);
    }

    /// Clear REG_L2_MEM_REFERSH_CNT_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_refersh_cnt_reset(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_REFERSH_CNT_RESET_MASK);
    }

    /// **REG_L2_MEM_UNIT5_REFERSH_EN** `[5]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_unit5_refersh_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_UNIT5_REFERSH_EN_MASK) != 0
    }

    /// Set REG_L2_MEM_UNIT5_REFERSH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_unit5_refersh_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_UNIT5_REFERSH_EN_MASK);
    }

    /// Clear REG_L2_MEM_UNIT5_REFERSH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_unit5_refersh_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_UNIT5_REFERSH_EN_MASK);
    }

    /// **REG_L2_MEM_UNIT4_REFERSH_EN** `[4]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_unit4_refersh_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_UNIT4_REFERSH_EN_MASK) != 0
    }

    /// Set REG_L2_MEM_UNIT4_REFERSH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_unit4_refersh_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_UNIT4_REFERSH_EN_MASK);
    }

    /// Clear REG_L2_MEM_UNIT4_REFERSH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_unit4_refersh_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_UNIT4_REFERSH_EN_MASK);
    }

    /// **REG_L2_MEM_UNIT3_REFERSH_EN** `[3]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_unit3_refersh_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_UNIT3_REFERSH_EN_MASK) != 0
    }

    /// Set REG_L2_MEM_UNIT3_REFERSH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_unit3_refersh_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_UNIT3_REFERSH_EN_MASK);
    }

    /// Clear REG_L2_MEM_UNIT3_REFERSH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_unit3_refersh_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_UNIT3_REFERSH_EN_MASK);
    }

    /// **REG_L2_MEM_UNIT2_REFERSH_EN** `[2]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_unit2_refersh_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_UNIT2_REFERSH_EN_MASK) != 0
    }

    /// Set REG_L2_MEM_UNIT2_REFERSH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_unit2_refersh_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_UNIT2_REFERSH_EN_MASK);
    }

    /// Clear REG_L2_MEM_UNIT2_REFERSH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_unit2_refersh_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_UNIT2_REFERSH_EN_MASK);
    }

    /// **REG_L2_MEM_UNIT1_REFERSH_EN** `[1]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_unit1_refersh_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_UNIT1_REFERSH_EN_MASK) != 0
    }

    /// Set REG_L2_MEM_UNIT1_REFERSH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_unit1_refersh_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_UNIT1_REFERSH_EN_MASK);
    }

    /// Clear REG_L2_MEM_UNIT1_REFERSH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_unit1_refersh_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_UNIT1_REFERSH_EN_MASK);
    }

    /// **REG_L2_MEM_UNIT0_REFERSH_EN** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_l2_mem_unit0_refersh_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_L2_MEM_UNIT0_REFERSH_EN_MASK) != 0
    }

    /// Set REG_L2_MEM_UNIT0_REFERSH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_l2_mem_unit0_refersh_en(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_L2_MEM_UNIT0_REFERSH_EN_MASK);
    }

    /// Clear REG_L2_MEM_UNIT0_REFERSH_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_l2_mem_unit0_refersh_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_L2_MEM_UNIT0_REFERSH_EN_MASK);
    }
}

/// L2_MEM_REFRESH value type for batch read/modify/write operations
///
/// Use `l2memrefresh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memrefresh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemRefreshVal {
    bits: u32,
}

impl L2MemRefreshVal {
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
        Self { bits: 0x00000040 }
    }

    /// Read **REG_L2_MEM_UNIT5_REFRESH_DONE** `[12:12]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_unit5_refresh_done(&self) -> bool {
        (self.bits & L2MemRefresh::REG_L2_MEM_UNIT5_REFRESH_DONE_MASK) != 0
    }

    /// Read **REG_L2_MEM_UNIT4_REFRESH_DONE** `[11:11]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_unit4_refresh_done(&self) -> bool {
        (self.bits & L2MemRefresh::REG_L2_MEM_UNIT4_REFRESH_DONE_MASK) != 0
    }

    /// Read **REG_L2_MEM_UNIT3_REFRESH_DONE** `[10:10]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_unit3_refresh_done(&self) -> bool {
        (self.bits & L2MemRefresh::REG_L2_MEM_UNIT3_REFRESH_DONE_MASK) != 0
    }

    /// Read **REG_L2_MEM_UNIT2_REFRESH_DONE** `[9:9]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_unit2_refresh_done(&self) -> bool {
        (self.bits & L2MemRefresh::REG_L2_MEM_UNIT2_REFRESH_DONE_MASK) != 0
    }

    /// Read **REG_L2_MEM_UNIT1_REFRESH_DONE** `[8:8]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_unit1_refresh_done(&self) -> bool {
        (self.bits & L2MemRefresh::REG_L2_MEM_UNIT1_REFRESH_DONE_MASK) != 0
    }

    /// Read **REG_L2_MEM_UNIT0_REFRESH_DONE** `[7:7]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_unit0_refresh_done(&self) -> bool {
        (self.bits & L2MemRefresh::REG_L2_MEM_UNIT0_REFRESH_DONE_MASK) != 0
    }

    /// Read **REG_L2_MEM_REFERSH_CNT_RESET** `[6:6]`
    ///
    /// Set 1 to reset l2mem_refresh_cnt
    #[inline(always)]
    pub const fn reg_l2_mem_refersh_cnt_reset(&self) -> bool {
        (self.bits & L2MemRefresh::REG_L2_MEM_REFERSH_CNT_RESET_MASK) != 0
    }

    /// Read **REG_L2_MEM_UNIT5_REFERSH_EN** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_unit5_refersh_en(&self) -> bool {
        (self.bits & L2MemRefresh::REG_L2_MEM_UNIT5_REFERSH_EN_MASK) != 0
    }

    /// Read **REG_L2_MEM_UNIT4_REFERSH_EN** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_unit4_refersh_en(&self) -> bool {
        (self.bits & L2MemRefresh::REG_L2_MEM_UNIT4_REFERSH_EN_MASK) != 0
    }

    /// Read **REG_L2_MEM_UNIT3_REFERSH_EN** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_unit3_refersh_en(&self) -> bool {
        (self.bits & L2MemRefresh::REG_L2_MEM_UNIT3_REFERSH_EN_MASK) != 0
    }

    /// Read **REG_L2_MEM_UNIT2_REFERSH_EN** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_unit2_refersh_en(&self) -> bool {
        (self.bits & L2MemRefresh::REG_L2_MEM_UNIT2_REFERSH_EN_MASK) != 0
    }

    /// Read **REG_L2_MEM_UNIT1_REFERSH_EN** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_unit1_refersh_en(&self) -> bool {
        (self.bits & L2MemRefresh::REG_L2_MEM_UNIT1_REFERSH_EN_MASK) != 0
    }

    /// Read **REG_L2_MEM_UNIT0_REFERSH_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_l2_mem_unit0_refersh_en(&self) -> bool {
        (self.bits & L2MemRefresh::REG_L2_MEM_UNIT0_REFERSH_EN_MASK) != 0
    }

    /// Set **REG_L2_MEM_REFERSH_CNT_RESET** `[6:6]`
    ///
    /// Set 1 to reset l2mem_refresh_cnt
    #[inline(always)]
    pub const fn with_reg_l2_mem_refersh_cnt_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemRefresh::REG_L2_MEM_REFERSH_CNT_RESET_MASK
            } else {
                self.bits & !L2MemRefresh::REG_L2_MEM_REFERSH_CNT_RESET_MASK
            },
        }
    }

    /// Set **REG_L2_MEM_UNIT5_REFERSH_EN** `[5:5]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_unit5_refersh_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemRefresh::REG_L2_MEM_UNIT5_REFERSH_EN_MASK
            } else {
                self.bits & !L2MemRefresh::REG_L2_MEM_UNIT5_REFERSH_EN_MASK
            },
        }
    }

    /// Set **REG_L2_MEM_UNIT4_REFERSH_EN** `[4:4]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_unit4_refersh_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemRefresh::REG_L2_MEM_UNIT4_REFERSH_EN_MASK
            } else {
                self.bits & !L2MemRefresh::REG_L2_MEM_UNIT4_REFERSH_EN_MASK
            },
        }
    }

    /// Set **REG_L2_MEM_UNIT3_REFERSH_EN** `[3:3]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_unit3_refersh_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemRefresh::REG_L2_MEM_UNIT3_REFERSH_EN_MASK
            } else {
                self.bits & !L2MemRefresh::REG_L2_MEM_UNIT3_REFERSH_EN_MASK
            },
        }
    }

    /// Set **REG_L2_MEM_UNIT2_REFERSH_EN** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_unit2_refersh_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemRefresh::REG_L2_MEM_UNIT2_REFERSH_EN_MASK
            } else {
                self.bits & !L2MemRefresh::REG_L2_MEM_UNIT2_REFERSH_EN_MASK
            },
        }
    }

    /// Set **REG_L2_MEM_UNIT1_REFERSH_EN** `[1:1]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_unit1_refersh_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemRefresh::REG_L2_MEM_UNIT1_REFERSH_EN_MASK
            } else {
                self.bits & !L2MemRefresh::REG_L2_MEM_UNIT1_REFERSH_EN_MASK
            },
        }
    }

    /// Set **REG_L2_MEM_UNIT0_REFERSH_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_l2_mem_unit0_refersh_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemRefresh::REG_L2_MEM_UNIT0_REFERSH_EN_MASK
            } else {
                self.bits & !L2MemRefresh::REG_L2_MEM_UNIT0_REFERSH_EN_MASK
            },
        }
    }
}

impl Default for L2MemRefreshVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl L2MemRefresh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemRefreshVal {
        L2MemRefreshVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2MemRefreshVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2MemRefreshVal) -> L2MemRefreshVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TCM_INIT Register
///
/// NA
///
/// Address offset: 0x0168
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **REG_TCM_INIT_DONE** `[2]` (RO)
///
///   NA
///
/// - **REG_TCM_INIT_CNT_RESET** `[1]` (RW)
///
///   Set 1 to reset tcm init cnt
///
/// - **REG_TCM_INIT_EN** `[0]` (RW)
///
///   NA
#[repr(transparent)]
pub struct TcmInit {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TcmInit {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TcmInit {
    // ======== Field Constants ========

    /// **REG_TCM_INIT_DONE**
    ///
    /// NA
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const REG_TCM_INIT_DONE_OFFSET: u32 = 2;
    pub const REG_TCM_INIT_DONE_MASK: u32 = 0x00000004;
    pub const REG_TCM_INIT_DONE: u32 = 1 << 2;

    /// **REG_TCM_INIT_CNT_RESET**
    ///
    /// Set 1 to reset tcm init cnt
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const REG_TCM_INIT_CNT_RESET_OFFSET: u32 = 1;
    pub const REG_TCM_INIT_CNT_RESET_MASK: u32 = 0x00000002;
    pub const REG_TCM_INIT_CNT_RESET: u32 = 1 << 1;

    /// **REG_TCM_INIT_EN**
    ///
    /// NA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const REG_TCM_INIT_EN_OFFSET: u32 = 0;
    pub const REG_TCM_INIT_EN_MASK: u32 = 0x00000001;
    pub const REG_TCM_INIT_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **REG_TCM_INIT_DONE** `[2]` (RO)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_tcm_init_done(&self) -> bool {
        (self.value.read_bits() & Self::REG_TCM_INIT_DONE_MASK) != 0
    }

    /// **REG_TCM_INIT_CNT_RESET** `[1]` (RW)
    ///
    /// Set 1 to reset tcm init cnt
    #[inline(always)]
    pub fn reg_tcm_init_cnt_reset(&self) -> bool {
        (self.value.read_bits() & Self::REG_TCM_INIT_CNT_RESET_MASK) != 0
    }

    /// Set REG_TCM_INIT_CNT_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_tcm_init_cnt_reset(&self) {
        self.value
            .modify_bits(|r| r | Self::REG_TCM_INIT_CNT_RESET_MASK);
    }

    /// Clear REG_TCM_INIT_CNT_RESET bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_tcm_init_cnt_reset(&self) {
        self.value
            .modify_bits(|r| r & !Self::REG_TCM_INIT_CNT_RESET_MASK);
    }

    /// **REG_TCM_INIT_EN** `[0]` (RW)
    ///
    /// NA
    #[inline(always)]
    pub fn reg_tcm_init_en(&self) -> bool {
        (self.value.read_bits() & Self::REG_TCM_INIT_EN_MASK) != 0
    }

    /// Set REG_TCM_INIT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_reg_tcm_init_en(&self) {
        self.value.modify_bits(|r| r | Self::REG_TCM_INIT_EN_MASK);
    }

    /// Clear REG_TCM_INIT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_reg_tcm_init_en(&self) {
        self.value.modify_bits(|r| r & !Self::REG_TCM_INIT_EN_MASK);
    }
}

/// TCM_INIT value type for batch read/modify/write operations
///
/// Use `tcminit.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcminit.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcmInitVal {
    bits: u32,
}

impl TcmInitVal {
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

    /// Read **REG_TCM_INIT_DONE** `[2:2]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_tcm_init_done(&self) -> bool {
        (self.bits & TcmInit::REG_TCM_INIT_DONE_MASK) != 0
    }

    /// Read **REG_TCM_INIT_CNT_RESET** `[1:1]`
    ///
    /// Set 1 to reset tcm init cnt
    #[inline(always)]
    pub const fn reg_tcm_init_cnt_reset(&self) -> bool {
        (self.bits & TcmInit::REG_TCM_INIT_CNT_RESET_MASK) != 0
    }

    /// Read **REG_TCM_INIT_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn reg_tcm_init_en(&self) -> bool {
        (self.bits & TcmInit::REG_TCM_INIT_EN_MASK) != 0
    }

    /// Set **REG_TCM_INIT_CNT_RESET** `[1:1]`
    ///
    /// Set 1 to reset tcm init cnt
    #[inline(always)]
    pub const fn with_reg_tcm_init_cnt_reset(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TcmInit::REG_TCM_INIT_CNT_RESET_MASK
            } else {
                self.bits & !TcmInit::REG_TCM_INIT_CNT_RESET_MASK
            },
        }
    }

    /// Set **REG_TCM_INIT_EN** `[0:0]`
    ///
    /// NA
    #[inline(always)]
    pub const fn with_reg_tcm_init_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TcmInit::REG_TCM_INIT_EN_MASK
            } else {
                self.bits & !TcmInit::REG_TCM_INIT_EN_MASK
            },
        }
    }
}

impl Default for TcmInitVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl TcmInit {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcmInitVal {
        TcmInitVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TcmInitVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TcmInitVal) -> TcmInitVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TCM_PARITY_CHECK_CTRL Register
///
/// need_des
///
/// Address offset: 0x016c
///
/// ## Fields
///
/// - **TCM_PARITY_CHECK_EN** `[0]` (RW)
///
///   Set 1 to turn on tcm parity check
#[repr(transparent)]
pub struct TcmParityCheckCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TcmParityCheckCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TcmParityCheckCtrl {
    // ======== Field Constants ========

    /// **TCM_PARITY_CHECK_EN**
    ///
    /// Set 1 to turn on tcm parity check
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TCM_PARITY_CHECK_EN_OFFSET: u32 = 0;
    pub const TCM_PARITY_CHECK_EN_MASK: u32 = 0x00000001;
    pub const TCM_PARITY_CHECK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TCM_PARITY_CHECK_EN** `[0]` (RW)
    ///
    /// Set 1 to turn on tcm parity check
    #[inline(always)]
    pub fn tcm_parity_check_en(&self) -> bool {
        (self.value.read_bits() & Self::TCM_PARITY_CHECK_EN_MASK) != 0
    }

    /// Set TCM_PARITY_CHECK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tcm_parity_check_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TCM_PARITY_CHECK_EN_MASK);
    }

    /// Clear TCM_PARITY_CHECK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tcm_parity_check_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TCM_PARITY_CHECK_EN_MASK);
    }
}

/// TCM_PARITY_CHECK_CTRL value type for batch read/modify/write operations
///
/// Use `tcmparitycheckctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `tcmparitycheckctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TcmParityCheckCtrlVal {
    bits: u32,
}

impl TcmParityCheckCtrlVal {
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

    /// Read **TCM_PARITY_CHECK_EN** `[0:0]`
    ///
    /// Set 1 to turn on tcm parity check
    #[inline(always)]
    pub const fn tcm_parity_check_en(&self) -> bool {
        (self.bits & TcmParityCheckCtrl::TCM_PARITY_CHECK_EN_MASK) != 0
    }

    /// Set **TCM_PARITY_CHECK_EN** `[0:0]`
    ///
    /// Set 1 to turn on tcm parity check
    #[inline(always)]
    pub const fn with_tcm_parity_check_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TcmParityCheckCtrl::TCM_PARITY_CHECK_EN_MASK
            } else {
                self.bits & !TcmParityCheckCtrl::TCM_PARITY_CHECK_EN_MASK
            },
        }
    }
}

impl Default for TcmParityCheckCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TcmParityCheckCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TcmParityCheckCtrlVal {
        TcmParityCheckCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TcmParityCheckCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TcmParityCheckCtrlVal) -> TcmParityCheckCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DESIGN_FOR_VERIFICATION0 Register
///
/// need_des
///
/// Address offset: 0x0170
///
/// ## Fields
///
/// - **DFV0** `[31:0]` (RW)
///
///   register for DV
#[repr(transparent)]
pub struct DesignForVerification0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DesignForVerification0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DesignForVerification0 {
    // ======== Field Constants ========

    /// **DFV0**
    ///
    /// register for DV
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DFV0_OFFSET: u32 = 0;
    pub const DFV0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DFV0** `[31:0]` (RW)
    ///
    /// register for DV
    #[inline(always)]
    pub fn dfv0(&self) -> u32 {
        (self.value.read_bits() & Self::DFV0_MASK) >> Self::DFV0_OFFSET
    }

    /// Write **DFV0** `[31:0]` (RW)
    ///
    /// register for DV
    #[inline(always)]
    pub fn write_dfv0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DFV0_MASK) | ((value << Self::DFV0_OFFSET) & Self::DFV0_MASK)
        });
    }
}

/// DESIGN_FOR_VERIFICATION0 value type for batch read/modify/write operations
///
/// Use `designforverification0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `designforverification0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DesignForVerification0Val {
    bits: u32,
}

impl DesignForVerification0Val {
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

    /// Read **DFV0** `[31:0]`
    ///
    /// register for DV
    #[inline(always)]
    pub const fn dfv0(&self) -> u32 {
        (self.bits & DesignForVerification0::DFV0_MASK) >> DesignForVerification0::DFV0_OFFSET
    }

    /// Set **DFV0** `[31:0]`
    ///
    /// register for DV
    #[inline(always)]
    pub const fn with_dfv0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DesignForVerification0::DFV0_MASK)
                | ((value << DesignForVerification0::DFV0_OFFSET)
                    & DesignForVerification0::DFV0_MASK),
        }
    }
}

impl Default for DesignForVerification0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DesignForVerification0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DesignForVerification0Val {
        DesignForVerification0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DesignForVerification0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DesignForVerification0Val) -> DesignForVerification0Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// DESIGN_FOR_VERIFICATION1 Register
///
/// need_des
///
/// Address offset: 0x0174
///
/// ## Fields
///
/// - **DFV1** `[31:0]` (RW)
///
///   register for DV
#[repr(transparent)]
pub struct DesignForVerification1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DesignForVerification1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DesignForVerification1 {
    // ======== Field Constants ========

    /// **DFV1**
    ///
    /// register for DV
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DFV1_OFFSET: u32 = 0;
    pub const DFV1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DFV1** `[31:0]` (RW)
    ///
    /// register for DV
    #[inline(always)]
    pub fn dfv1(&self) -> u32 {
        (self.value.read_bits() & Self::DFV1_MASK) >> Self::DFV1_OFFSET
    }

    /// Write **DFV1** `[31:0]` (RW)
    ///
    /// register for DV
    #[inline(always)]
    pub fn write_dfv1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DFV1_MASK) | ((value << Self::DFV1_OFFSET) & Self::DFV1_MASK)
        });
    }
}

/// DESIGN_FOR_VERIFICATION1 value type for batch read/modify/write operations
///
/// Use `designforverification1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `designforverification1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DesignForVerification1Val {
    bits: u32,
}

impl DesignForVerification1Val {
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

    /// Read **DFV1** `[31:0]`
    ///
    /// register for DV
    #[inline(always)]
    pub const fn dfv1(&self) -> u32 {
        (self.bits & DesignForVerification1::DFV1_MASK) >> DesignForVerification1::DFV1_OFFSET
    }

    /// Set **DFV1** `[31:0]`
    ///
    /// register for DV
    #[inline(always)]
    pub const fn with_dfv1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DesignForVerification1::DFV1_MASK)
                | ((value << DesignForVerification1::DFV1_OFFSET)
                    & DesignForVerification1::DFV1_MASK),
        }
    }
}

impl Default for DesignForVerification1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl DesignForVerification1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DesignForVerification1Val {
        DesignForVerification1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DesignForVerification1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DesignForVerification1Val) -> DesignForVerification1Val>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// PSRAM_FLASH_ADDR_INTERCHANGE Register
///
/// need_des
///
/// Address offset: 0x0180
///
/// ## Fields
///
/// - **DMA** `[1]` (RW)
///
///   Set 1 to enable addr interchange between psram and flash in axi matrix when dma device access, lp core access and hp core access through ahb
///
/// - **CPU** `[0]` (RW)
///
///   Set 1 to enable addr interchange between psram and flash in axi matrix when hp cpu access through cache
#[repr(transparent)]
pub struct PsramFlashAddrInterchange {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PsramFlashAddrInterchange {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PsramFlashAddrInterchange {
    // ======== Field Constants ========

    /// **DMA**
    ///
    /// Set 1 to enable addr interchange between psram and flash in axi matrix when dma device access, lp core access and hp core access through ahb
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DMA_OFFSET: u32 = 1;
    pub const DMA_MASK: u32 = 0x00000002;
    pub const DMA: u32 = 1 << 1;

    /// **CPU**
    ///
    /// Set 1 to enable addr interchange between psram and flash in axi matrix when hp cpu access through cache
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CPU_OFFSET: u32 = 0;
    pub const CPU_MASK: u32 = 0x00000001;
    pub const CPU: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DMA** `[1]` (RW)
    ///
    /// Set 1 to enable addr interchange between psram and flash in axi matrix when dma device access, lp core access and hp core access through ahb
    #[inline(always)]
    pub fn dma(&self) -> bool {
        (self.value.read_bits() & Self::DMA_MASK) != 0
    }

    /// Set DMA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma(&self) {
        self.value.modify_bits(|r| r | Self::DMA_MASK);
    }

    /// Clear DMA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma(&self) {
        self.value.modify_bits(|r| r & !Self::DMA_MASK);
    }

    /// **CPU** `[0]` (RW)
    ///
    /// Set 1 to enable addr interchange between psram and flash in axi matrix when hp cpu access through cache
    #[inline(always)]
    pub fn cpu(&self) -> bool {
        (self.value.read_bits() & Self::CPU_MASK) != 0
    }

    /// Set CPU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cpu(&self) {
        self.value.modify_bits(|r| r | Self::CPU_MASK);
    }

    /// Clear CPU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cpu(&self) {
        self.value.modify_bits(|r| r & !Self::CPU_MASK);
    }
}

/// PSRAM_FLASH_ADDR_INTERCHANGE value type for batch read/modify/write operations
///
/// Use `psramflashaddrinterchange.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `psramflashaddrinterchange.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PsramFlashAddrInterchangeVal {
    bits: u32,
}

impl PsramFlashAddrInterchangeVal {
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

    /// Read **DMA** `[1:1]`
    ///
    /// Set 1 to enable addr interchange between psram and flash in axi matrix when dma device access, lp core access and hp core access through ahb
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        (self.bits & PsramFlashAddrInterchange::DMA_MASK) != 0
    }

    /// Read **CPU** `[0:0]`
    ///
    /// Set 1 to enable addr interchange between psram and flash in axi matrix when hp cpu access through cache
    #[inline(always)]
    pub const fn cpu(&self) -> bool {
        (self.bits & PsramFlashAddrInterchange::CPU_MASK) != 0
    }

    /// Set **DMA** `[1:1]`
    ///
    /// Set 1 to enable addr interchange between psram and flash in axi matrix when dma device access, lp core access and hp core access through ahb
    #[inline(always)]
    pub const fn with_dma(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PsramFlashAddrInterchange::DMA_MASK
            } else {
                self.bits & !PsramFlashAddrInterchange::DMA_MASK
            },
        }
    }

    /// Set **CPU** `[0:0]`
    ///
    /// Set 1 to enable addr interchange between psram and flash in axi matrix when hp cpu access through cache
    #[inline(always)]
    pub const fn with_cpu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PsramFlashAddrInterchange::CPU_MASK
            } else {
                self.bits & !PsramFlashAddrInterchange::CPU_MASK
            },
        }
    }
}

impl Default for PsramFlashAddrInterchangeVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PsramFlashAddrInterchange {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PsramFlashAddrInterchangeVal {
        PsramFlashAddrInterchangeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PsramFlashAddrInterchangeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PsramFlashAddrInterchangeVal) -> PsramFlashAddrInterchangeVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// AHB2AXI_BRESP_ERR_INT_RAW Register
///
/// NA
///
/// Address offset: 0x0188
///
/// ## Fields
///
/// - **CPU_ICM_H2X_BRESP_ERR_INT_RAW** `[0]` (RW)
///
///   the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi.
#[repr(transparent)]
pub struct Ahb2axiBrespErrIntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ahb2axiBrespErrIntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ahb2axiBrespErrIntRaw {
    // ======== Field Constants ========

    /// **CPU_ICM_H2X_BRESP_ERR_INT_RAW**
    ///
    /// the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CPU_ICM_H2X_BRESP_ERR_INT_RAW_OFFSET: u32 = 0;
    pub const CPU_ICM_H2X_BRESP_ERR_INT_RAW_MASK: u32 = 0x00000001;
    pub const CPU_ICM_H2X_BRESP_ERR_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CPU_ICM_H2X_BRESP_ERR_INT_RAW** `[0]` (RW)
    ///
    /// the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi.
    #[inline(always)]
    pub fn cpu_icm_h2x_bresp_err_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CPU_ICM_H2X_BRESP_ERR_INT_RAW_MASK) != 0
    }

    /// Set CPU_ICM_H2X_BRESP_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cpu_icm_h2x_bresp_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CPU_ICM_H2X_BRESP_ERR_INT_RAW_MASK);
    }

    /// Clear CPU_ICM_H2X_BRESP_ERR_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cpu_icm_h2x_bresp_err_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CPU_ICM_H2X_BRESP_ERR_INT_RAW_MASK);
    }
}

/// AHB2AXI_BRESP_ERR_INT_RAW value type for batch read/modify/write operations
///
/// Use `ahb2axibresperrintraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ahb2axibresperrintraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ahb2axiBrespErrIntRawVal {
    bits: u32,
}

impl Ahb2axiBrespErrIntRawVal {
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

    /// Read **CPU_ICM_H2X_BRESP_ERR_INT_RAW** `[0:0]`
    ///
    /// the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi.
    #[inline(always)]
    pub const fn cpu_icm_h2x_bresp_err_int_raw(&self) -> bool {
        (self.bits & Ahb2axiBrespErrIntRaw::CPU_ICM_H2X_BRESP_ERR_INT_RAW_MASK) != 0
    }

    /// Set **CPU_ICM_H2X_BRESP_ERR_INT_RAW** `[0:0]`
    ///
    /// the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi.
    #[inline(always)]
    pub const fn with_cpu_icm_h2x_bresp_err_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ahb2axiBrespErrIntRaw::CPU_ICM_H2X_BRESP_ERR_INT_RAW_MASK
            } else {
                self.bits & !Ahb2axiBrespErrIntRaw::CPU_ICM_H2X_BRESP_ERR_INT_RAW_MASK
            },
        }
    }
}

impl Default for Ahb2axiBrespErrIntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ahb2axiBrespErrIntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ahb2axiBrespErrIntRawVal {
        Ahb2axiBrespErrIntRawVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ahb2axiBrespErrIntRawVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ahb2axiBrespErrIntRawVal) -> Ahb2axiBrespErrIntRawVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// AHB2AXI_BRESP_ERR_INT_ST Register
///
/// need_des
///
/// Address offset: 0x018c
///
/// ## Fields
///
/// - **CPU_ICM_H2X_BRESP_ERR_INT_ST** `[31]` (RO)
///
///   the masked interrupt status of cpu_icm_h2x_bresp_err
#[repr(transparent)]
pub struct Ahb2axiBrespErrIntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ahb2axiBrespErrIntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ahb2axiBrespErrIntSt {
    // ======== Field Constants ========

    /// **CPU_ICM_H2X_BRESP_ERR_INT_ST**
    ///
    /// the masked interrupt status of cpu_icm_h2x_bresp_err
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CPU_ICM_H2X_BRESP_ERR_INT_ST_OFFSET: u32 = 31;
    pub const CPU_ICM_H2X_BRESP_ERR_INT_ST_MASK: u32 = 0x80000000;
    pub const CPU_ICM_H2X_BRESP_ERR_INT_ST: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **CPU_ICM_H2X_BRESP_ERR_INT_ST** `[31]` (RO)
    ///
    /// the masked interrupt status of cpu_icm_h2x_bresp_err
    #[inline(always)]
    pub fn cpu_icm_h2x_bresp_err_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CPU_ICM_H2X_BRESP_ERR_INT_ST_MASK) != 0
    }
}

/// AHB2AXI_BRESP_ERR_INT_ST value type for batch read/modify/write operations
///
/// Use `ahb2axibresperrintst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ahb2axibresperrintst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ahb2axiBrespErrIntStVal {
    bits: u32,
}

impl Ahb2axiBrespErrIntStVal {
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

    /// Read **CPU_ICM_H2X_BRESP_ERR_INT_ST** `[31:31]`
    ///
    /// the masked interrupt status of cpu_icm_h2x_bresp_err
    #[inline(always)]
    pub const fn cpu_icm_h2x_bresp_err_int_st(&self) -> bool {
        (self.bits & Ahb2axiBrespErrIntSt::CPU_ICM_H2X_BRESP_ERR_INT_ST_MASK) != 0
    }
}

impl Default for Ahb2axiBrespErrIntStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ahb2axiBrespErrIntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ahb2axiBrespErrIntStVal {
        Ahb2axiBrespErrIntStVal::from_bits(self.value.read_bits())
    }
}

/// AHB2AXI_BRESP_ERR_INT_ENA Register
///
/// need_des
///
/// Address offset: 0x0190
///
/// ## Fields
///
/// - **CPU_ICM_H2X_BRESP_ERR_INT_ENA** `[31]` (RW)
///
///   Write 1 to enable cpu_icm_h2x_bresp_err int
#[repr(transparent)]
pub struct Ahb2axiBrespErrIntEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ahb2axiBrespErrIntEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ahb2axiBrespErrIntEna {
    // ======== Field Constants ========

    /// **CPU_ICM_H2X_BRESP_ERR_INT_ENA**
    ///
    /// Write 1 to enable cpu_icm_h2x_bresp_err int
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CPU_ICM_H2X_BRESP_ERR_INT_ENA_OFFSET: u32 = 31;
    pub const CPU_ICM_H2X_BRESP_ERR_INT_ENA_MASK: u32 = 0x80000000;
    pub const CPU_ICM_H2X_BRESP_ERR_INT_ENA: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **CPU_ICM_H2X_BRESP_ERR_INT_ENA** `[31]` (RW)
    ///
    /// Write 1 to enable cpu_icm_h2x_bresp_err int
    #[inline(always)]
    pub fn cpu_icm_h2x_bresp_err_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CPU_ICM_H2X_BRESP_ERR_INT_ENA_MASK) != 0
    }

    /// Set CPU_ICM_H2X_BRESP_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cpu_icm_h2x_bresp_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CPU_ICM_H2X_BRESP_ERR_INT_ENA_MASK);
    }

    /// Clear CPU_ICM_H2X_BRESP_ERR_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cpu_icm_h2x_bresp_err_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CPU_ICM_H2X_BRESP_ERR_INT_ENA_MASK);
    }
}

/// AHB2AXI_BRESP_ERR_INT_ENA value type for batch read/modify/write operations
///
/// Use `ahb2axibresperrintena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ahb2axibresperrintena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ahb2axiBrespErrIntEnaVal {
    bits: u32,
}

impl Ahb2axiBrespErrIntEnaVal {
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

    /// Read **CPU_ICM_H2X_BRESP_ERR_INT_ENA** `[31:31]`
    ///
    /// Write 1 to enable cpu_icm_h2x_bresp_err int
    #[inline(always)]
    pub const fn cpu_icm_h2x_bresp_err_int_ena(&self) -> bool {
        (self.bits & Ahb2axiBrespErrIntEna::CPU_ICM_H2X_BRESP_ERR_INT_ENA_MASK) != 0
    }

    /// Set **CPU_ICM_H2X_BRESP_ERR_INT_ENA** `[31:31]`
    ///
    /// Write 1 to enable cpu_icm_h2x_bresp_err int
    #[inline(always)]
    pub const fn with_cpu_icm_h2x_bresp_err_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ahb2axiBrespErrIntEna::CPU_ICM_H2X_BRESP_ERR_INT_ENA_MASK
            } else {
                self.bits & !Ahb2axiBrespErrIntEna::CPU_ICM_H2X_BRESP_ERR_INT_ENA_MASK
            },
        }
    }
}

impl Default for Ahb2axiBrespErrIntEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ahb2axiBrespErrIntEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Ahb2axiBrespErrIntEnaVal {
        Ahb2axiBrespErrIntEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ahb2axiBrespErrIntEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Ahb2axiBrespErrIntEnaVal) -> Ahb2axiBrespErrIntEnaVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// AHB2AXI_BRESP_ERR_INT_CLR Register
///
/// need_des
///
/// Address offset: 0x0194
///
/// ## Fields
///
/// - **CPU_ICM_H2X_BRESP_ERR_INT_CLR** `[31]` (WO)
///
///   Write 1 to clear cpu_icm_h2x_bresp_err int
#[repr(transparent)]
pub struct Ahb2axiBrespErrIntClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Ahb2axiBrespErrIntClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ahb2axiBrespErrIntClr {
    // ======== Field Constants ========

    /// **CPU_ICM_H2X_BRESP_ERR_INT_CLR**
    ///
    /// Write 1 to clear cpu_icm_h2x_bresp_err int
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CPU_ICM_H2X_BRESP_ERR_INT_CLR_OFFSET: u32 = 31;
    pub const CPU_ICM_H2X_BRESP_ERR_INT_CLR_MASK: u32 = 0x80000000;
    pub const CPU_ICM_H2X_BRESP_ERR_INT_CLR: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// Trigger **CPU_ICM_H2X_BRESP_ERR_INT_CLR** (WO)
    ///
    /// Write 1 to clear cpu_icm_h2x_bresp_err int
    #[inline(always)]
    pub fn trigger_cpu_icm_h2x_bresp_err_int_clr(&self) {
        self.value
            .write_bits(Self::CPU_ICM_H2X_BRESP_ERR_INT_CLR_MASK);
    }
}

/// AHB2AXI_BRESP_ERR_INT_CLR value type for batch read/modify/write operations
///
/// Use `ahb2axibresperrintclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `ahb2axibresperrintclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Ahb2axiBrespErrIntClrVal {
    bits: u32,
}

impl Ahb2axiBrespErrIntClrVal {
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

    /// Set **CPU_ICM_H2X_BRESP_ERR_INT_CLR** `[31:31]`
    ///
    /// Write 1 to clear cpu_icm_h2x_bresp_err int
    #[inline(always)]
    pub const fn with_cpu_icm_h2x_bresp_err_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Ahb2axiBrespErrIntClr::CPU_ICM_H2X_BRESP_ERR_INT_CLR_MASK
            } else {
                self.bits & !Ahb2axiBrespErrIntClr::CPU_ICM_H2X_BRESP_ERR_INT_CLR_MASK
            },
        }
    }
}

impl Default for Ahb2axiBrespErrIntClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Ahb2axiBrespErrIntClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Ahb2axiBrespErrIntClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// L2_MEM_ERR_RESP_CTRL Register
///
/// need_des
///
/// Address offset: 0x0198
///
/// ## Fields
///
/// - **L2_MEM_ERR_RESP_EN** `[0]` (RW)
///
///   Set 1 to turn on l2mem error response
#[repr(transparent)]
pub struct L2MemErrRespCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemErrRespCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemErrRespCtrl {
    // ======== Field Constants ========

    /// **L2_MEM_ERR_RESP_EN**
    ///
    /// Set 1 to turn on l2mem error response
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const L2_MEM_ERR_RESP_EN_OFFSET: u32 = 0;
    pub const L2_MEM_ERR_RESP_EN_MASK: u32 = 0x00000001;
    pub const L2_MEM_ERR_RESP_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **L2_MEM_ERR_RESP_EN** `[0]` (RW)
    ///
    /// Set 1 to turn on l2mem error response
    #[inline(always)]
    pub fn l2_mem_err_resp_en(&self) -> bool {
        (self.value.read_bits() & Self::L2_MEM_ERR_RESP_EN_MASK) != 0
    }

    /// Set L2_MEM_ERR_RESP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_l2_mem_err_resp_en(&self) {
        self.value
            .modify_bits(|r| r | Self::L2_MEM_ERR_RESP_EN_MASK);
    }

    /// Clear L2_MEM_ERR_RESP_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_l2_mem_err_resp_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::L2_MEM_ERR_RESP_EN_MASK);
    }
}

/// L2_MEM_ERR_RESP_CTRL value type for batch read/modify/write operations
///
/// Use `l2memerrrespctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memerrrespctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemErrRespCtrlVal {
    bits: u32,
}

impl L2MemErrRespCtrlVal {
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

    /// Read **L2_MEM_ERR_RESP_EN** `[0:0]`
    ///
    /// Set 1 to turn on l2mem error response
    #[inline(always)]
    pub const fn l2_mem_err_resp_en(&self) -> bool {
        (self.bits & L2MemErrRespCtrl::L2_MEM_ERR_RESP_EN_MASK) != 0
    }

    /// Set **L2_MEM_ERR_RESP_EN** `[0:0]`
    ///
    /// Set 1 to turn on l2mem error response
    #[inline(always)]
    pub const fn with_l2_mem_err_resp_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemErrRespCtrl::L2_MEM_ERR_RESP_EN_MASK
            } else {
                self.bits & !L2MemErrRespCtrl::L2_MEM_ERR_RESP_EN_MASK
            },
        }
    }
}

impl Default for L2MemErrRespCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemErrRespCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemErrRespCtrlVal {
        L2MemErrRespCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2MemErrRespCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2MemErrRespCtrlVal) -> L2MemErrRespCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// L2_MEM_AHB_BUFFER_CTRL Register
///
/// need_des
///
/// Address offset: 0x019c
///
/// ## Fields
///
/// - **L2_MEM_AHB_RDBUFFER_EN** `[1]` (RW)
///
///   Set 1 to turn on l2mem ahb rd buffer
///
/// - **L2_MEM_AHB_WRBUFFER_EN** `[0]` (RW)
///
///   Set 1 to turn on l2mem ahb wr buffer
#[repr(transparent)]
pub struct L2MemAhbBufferCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for L2MemAhbBufferCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl L2MemAhbBufferCtrl {
    // ======== Field Constants ========

    /// **L2_MEM_AHB_RDBUFFER_EN**
    ///
    /// Set 1 to turn on l2mem ahb rd buffer
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const L2_MEM_AHB_RDBUFFER_EN_OFFSET: u32 = 1;
    pub const L2_MEM_AHB_RDBUFFER_EN_MASK: u32 = 0x00000002;
    pub const L2_MEM_AHB_RDBUFFER_EN: u32 = 1 << 1;

    /// **L2_MEM_AHB_WRBUFFER_EN**
    ///
    /// Set 1 to turn on l2mem ahb wr buffer
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const L2_MEM_AHB_WRBUFFER_EN_OFFSET: u32 = 0;
    pub const L2_MEM_AHB_WRBUFFER_EN_MASK: u32 = 0x00000001;
    pub const L2_MEM_AHB_WRBUFFER_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **L2_MEM_AHB_RDBUFFER_EN** `[1]` (RW)
    ///
    /// Set 1 to turn on l2mem ahb rd buffer
    #[inline(always)]
    pub fn l2_mem_ahb_rdbuffer_en(&self) -> bool {
        (self.value.read_bits() & Self::L2_MEM_AHB_RDBUFFER_EN_MASK) != 0
    }

    /// Set L2_MEM_AHB_RDBUFFER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_l2_mem_ahb_rdbuffer_en(&self) {
        self.value
            .modify_bits(|r| r | Self::L2_MEM_AHB_RDBUFFER_EN_MASK);
    }

    /// Clear L2_MEM_AHB_RDBUFFER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_l2_mem_ahb_rdbuffer_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::L2_MEM_AHB_RDBUFFER_EN_MASK);
    }

    /// **L2_MEM_AHB_WRBUFFER_EN** `[0]` (RW)
    ///
    /// Set 1 to turn on l2mem ahb wr buffer
    #[inline(always)]
    pub fn l2_mem_ahb_wrbuffer_en(&self) -> bool {
        (self.value.read_bits() & Self::L2_MEM_AHB_WRBUFFER_EN_MASK) != 0
    }

    /// Set L2_MEM_AHB_WRBUFFER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_l2_mem_ahb_wrbuffer_en(&self) {
        self.value
            .modify_bits(|r| r | Self::L2_MEM_AHB_WRBUFFER_EN_MASK);
    }

    /// Clear L2_MEM_AHB_WRBUFFER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_l2_mem_ahb_wrbuffer_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::L2_MEM_AHB_WRBUFFER_EN_MASK);
    }
}

/// L2_MEM_AHB_BUFFER_CTRL value type for batch read/modify/write operations
///
/// Use `l2memahbbufferctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `l2memahbbufferctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct L2MemAhbBufferCtrlVal {
    bits: u32,
}

impl L2MemAhbBufferCtrlVal {
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

    /// Read **L2_MEM_AHB_RDBUFFER_EN** `[1:1]`
    ///
    /// Set 1 to turn on l2mem ahb rd buffer
    #[inline(always)]
    pub const fn l2_mem_ahb_rdbuffer_en(&self) -> bool {
        (self.bits & L2MemAhbBufferCtrl::L2_MEM_AHB_RDBUFFER_EN_MASK) != 0
    }

    /// Read **L2_MEM_AHB_WRBUFFER_EN** `[0:0]`
    ///
    /// Set 1 to turn on l2mem ahb wr buffer
    #[inline(always)]
    pub const fn l2_mem_ahb_wrbuffer_en(&self) -> bool {
        (self.bits & L2MemAhbBufferCtrl::L2_MEM_AHB_WRBUFFER_EN_MASK) != 0
    }

    /// Set **L2_MEM_AHB_RDBUFFER_EN** `[1:1]`
    ///
    /// Set 1 to turn on l2mem ahb rd buffer
    #[inline(always)]
    pub const fn with_l2_mem_ahb_rdbuffer_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemAhbBufferCtrl::L2_MEM_AHB_RDBUFFER_EN_MASK
            } else {
                self.bits & !L2MemAhbBufferCtrl::L2_MEM_AHB_RDBUFFER_EN_MASK
            },
        }
    }

    /// Set **L2_MEM_AHB_WRBUFFER_EN** `[0:0]`
    ///
    /// Set 1 to turn on l2mem ahb wr buffer
    #[inline(always)]
    pub const fn with_l2_mem_ahb_wrbuffer_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | L2MemAhbBufferCtrl::L2_MEM_AHB_WRBUFFER_EN_MASK
            } else {
                self.bits & !L2MemAhbBufferCtrl::L2_MEM_AHB_WRBUFFER_EN_MASK
            },
        }
    }
}

impl Default for L2MemAhbBufferCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl L2MemAhbBufferCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> L2MemAhbBufferCtrlVal {
        L2MemAhbBufferCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: L2MemAhbBufferCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(L2MemAhbBufferCtrlVal) -> L2MemAhbBufferCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_DMACTIVE_LPCORE Register
///
/// need_des
///
/// Address offset: 0x01a0
///
/// ## Fields
///
/// - **CORE_DMACTIVE_LPCORE** `[0]` (RO)
///
///   hp core dmactive_lpcore value
#[repr(transparent)]
pub struct CoreDmactiveLpcore {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CoreDmactiveLpcore {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoreDmactiveLpcore {
    // ======== Field Constants ========

    /// **CORE_DMACTIVE_LPCORE**
    ///
    /// hp core dmactive_lpcore value
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE_DMACTIVE_LPCORE_OFFSET: u32 = 0;
    pub const CORE_DMACTIVE_LPCORE_MASK: u32 = 0x00000001;
    pub const CORE_DMACTIVE_LPCORE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE_DMACTIVE_LPCORE** `[0]` (RO)
    ///
    /// hp core dmactive_lpcore value
    #[inline(always)]
    pub fn core_dmactive_lpcore(&self) -> bool {
        (self.value.read_bits() & Self::CORE_DMACTIVE_LPCORE_MASK) != 0
    }
}

/// CORE_DMACTIVE_LPCORE value type for batch read/modify/write operations
///
/// Use `coredmactivelpcore.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `coredmactivelpcore.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoreDmactiveLpcoreVal {
    bits: u32,
}

impl CoreDmactiveLpcoreVal {
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

    /// Read **CORE_DMACTIVE_LPCORE** `[0:0]`
    ///
    /// hp core dmactive_lpcore value
    #[inline(always)]
    pub const fn core_dmactive_lpcore(&self) -> bool {
        (self.bits & CoreDmactiveLpcore::CORE_DMACTIVE_LPCORE_MASK) != 0
    }
}

impl Default for CoreDmactiveLpcoreVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CoreDmactiveLpcore {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CoreDmactiveLpcoreVal {
        CoreDmactiveLpcoreVal::from_bits(self.value.read_bits())
    }
}

/// CORE_ERR_RESP_DIS Register
///
/// need_des
///
/// Address offset: 0x01a4
///
/// ## Fields
///
/// - **CORE_ERR_RESP_DIS** `[2:0]` (RW)
///
///   Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp.
#[repr(transparent)]
pub struct CoreErrRespDis {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CoreErrRespDis {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoreErrRespDis {
    // ======== Field Constants ========

    /// **CORE_ERR_RESP_DIS**
    ///
    /// Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const CORE_ERR_RESP_DIS_OFFSET: u32 = 0;
    pub const CORE_ERR_RESP_DIS_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **CORE_ERR_RESP_DIS** `[2:0]` (RW)
    ///
    /// Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp.
    #[inline(always)]
    pub fn core_err_resp_dis(&self) -> u32 {
        (self.value.read_bits() & Self::CORE_ERR_RESP_DIS_MASK) >> Self::CORE_ERR_RESP_DIS_OFFSET
    }

    /// Write **CORE_ERR_RESP_DIS** `[2:0]` (RW)
    ///
    /// Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp.
    #[inline(always)]
    pub fn write_core_err_resp_dis(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::CORE_ERR_RESP_DIS_MASK)
                | ((value << Self::CORE_ERR_RESP_DIS_OFFSET) & Self::CORE_ERR_RESP_DIS_MASK)
        });
    }
}

/// CORE_ERR_RESP_DIS value type for batch read/modify/write operations
///
/// Use `coreerrrespdis.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `coreerrrespdis.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoreErrRespDisVal {
    bits: u32,
}

impl CoreErrRespDisVal {
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

    /// Read **CORE_ERR_RESP_DIS** `[2:0]`
    ///
    /// Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp.
    #[inline(always)]
    pub const fn core_err_resp_dis(&self) -> u32 {
        (self.bits & CoreErrRespDis::CORE_ERR_RESP_DIS_MASK)
            >> CoreErrRespDis::CORE_ERR_RESP_DIS_OFFSET
    }

    /// Set **CORE_ERR_RESP_DIS** `[2:0]`
    ///
    /// Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp.
    #[inline(always)]
    pub const fn with_core_err_resp_dis(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !CoreErrRespDis::CORE_ERR_RESP_DIS_MASK)
                | ((value << CoreErrRespDis::CORE_ERR_RESP_DIS_OFFSET)
                    & CoreErrRespDis::CORE_ERR_RESP_DIS_MASK),
        }
    }
}

impl Default for CoreErrRespDisVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CoreErrRespDis {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CoreErrRespDisVal {
        CoreErrRespDisVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CoreErrRespDisVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CoreErrRespDisVal) -> CoreErrRespDisVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_TIMEOUT_INT_RAW Register
///
/// Hp core bus timeout interrupt raw register
///
/// Address offset: 0x01a8
///
/// ## Fields
///
/// - **CORE1_DBUS_TIMEOUT_INT_RAW** `[5]` (RW)
///
///   the raw interrupt status of hp core1 dbus timeout
///
/// - **CORE0_DBUS_TIMEOUT_INT_RAW** `[4]` (RW)
///
///   the raw interrupt status of hp core0 dbus timeout
///
/// - **CORE1_IBUS_TIMEOUT_INT_RAW** `[3]` (RW)
///
///   the raw interrupt status of hp core1 ibus timeout
///
/// - **CORE0_IBUS_TIMEOUT_INT_RAW** `[2]` (RW)
///
///   the raw interrupt status of hp core0 ibus timeout
///
/// - **CORE1_AHB_TIMEOUT_INT_RAW** `[1]` (RW)
///
///   the raw interrupt status of hp core1 ahb timeout
///
/// - **CORE0_AHB_TIMEOUT_INT_RAW** `[0]` (RW)
///
///   the raw interrupt status of hp core0 ahb timeout
#[repr(transparent)]
pub struct CoreTimeoutIntRaw {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CoreTimeoutIntRaw {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoreTimeoutIntRaw {
    // ======== Field Constants ========

    /// **CORE1_DBUS_TIMEOUT_INT_RAW**
    ///
    /// the raw interrupt status of hp core1 dbus timeout
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_DBUS_TIMEOUT_INT_RAW_OFFSET: u32 = 5;
    pub const CORE1_DBUS_TIMEOUT_INT_RAW_MASK: u32 = 0x00000020;
    pub const CORE1_DBUS_TIMEOUT_INT_RAW: u32 = 1 << 5;

    /// **CORE0_DBUS_TIMEOUT_INT_RAW**
    ///
    /// the raw interrupt status of hp core0 dbus timeout
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_DBUS_TIMEOUT_INT_RAW_OFFSET: u32 = 4;
    pub const CORE0_DBUS_TIMEOUT_INT_RAW_MASK: u32 = 0x00000010;
    pub const CORE0_DBUS_TIMEOUT_INT_RAW: u32 = 1 << 4;

    /// **CORE1_IBUS_TIMEOUT_INT_RAW**
    ///
    /// the raw interrupt status of hp core1 ibus timeout
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CORE1_IBUS_TIMEOUT_INT_RAW_OFFSET: u32 = 3;
    pub const CORE1_IBUS_TIMEOUT_INT_RAW_MASK: u32 = 0x00000008;
    pub const CORE1_IBUS_TIMEOUT_INT_RAW: u32 = 1 << 3;

    /// **CORE0_IBUS_TIMEOUT_INT_RAW**
    ///
    /// the raw interrupt status of hp core0 ibus timeout
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CORE0_IBUS_TIMEOUT_INT_RAW_OFFSET: u32 = 2;
    pub const CORE0_IBUS_TIMEOUT_INT_RAW_MASK: u32 = 0x00000004;
    pub const CORE0_IBUS_TIMEOUT_INT_RAW: u32 = 1 << 2;

    /// **CORE1_AHB_TIMEOUT_INT_RAW**
    ///
    /// the raw interrupt status of hp core1 ahb timeout
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE1_AHB_TIMEOUT_INT_RAW_OFFSET: u32 = 1;
    pub const CORE1_AHB_TIMEOUT_INT_RAW_MASK: u32 = 0x00000002;
    pub const CORE1_AHB_TIMEOUT_INT_RAW: u32 = 1 << 1;

    /// **CORE0_AHB_TIMEOUT_INT_RAW**
    ///
    /// the raw interrupt status of hp core0 ahb timeout
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE0_AHB_TIMEOUT_INT_RAW_OFFSET: u32 = 0;
    pub const CORE0_AHB_TIMEOUT_INT_RAW_MASK: u32 = 0x00000001;
    pub const CORE0_AHB_TIMEOUT_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE1_DBUS_TIMEOUT_INT_RAW** `[5]` (RW)
    ///
    /// the raw interrupt status of hp core1 dbus timeout
    #[inline(always)]
    pub fn core1_dbus_timeout_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_DBUS_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Set CORE1_DBUS_TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core1_dbus_timeout_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE1_DBUS_TIMEOUT_INT_RAW_MASK);
    }

    /// Clear CORE1_DBUS_TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core1_dbus_timeout_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE1_DBUS_TIMEOUT_INT_RAW_MASK);
    }

    /// **CORE0_DBUS_TIMEOUT_INT_RAW** `[4]` (RW)
    ///
    /// the raw interrupt status of hp core0 dbus timeout
    #[inline(always)]
    pub fn core0_dbus_timeout_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_DBUS_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Set CORE0_DBUS_TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core0_dbus_timeout_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE0_DBUS_TIMEOUT_INT_RAW_MASK);
    }

    /// Clear CORE0_DBUS_TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core0_dbus_timeout_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE0_DBUS_TIMEOUT_INT_RAW_MASK);
    }

    /// **CORE1_IBUS_TIMEOUT_INT_RAW** `[3]` (RW)
    ///
    /// the raw interrupt status of hp core1 ibus timeout
    #[inline(always)]
    pub fn core1_ibus_timeout_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_IBUS_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Set CORE1_IBUS_TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core1_ibus_timeout_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE1_IBUS_TIMEOUT_INT_RAW_MASK);
    }

    /// Clear CORE1_IBUS_TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core1_ibus_timeout_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE1_IBUS_TIMEOUT_INT_RAW_MASK);
    }

    /// **CORE0_IBUS_TIMEOUT_INT_RAW** `[2]` (RW)
    ///
    /// the raw interrupt status of hp core0 ibus timeout
    #[inline(always)]
    pub fn core0_ibus_timeout_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_IBUS_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Set CORE0_IBUS_TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core0_ibus_timeout_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE0_IBUS_TIMEOUT_INT_RAW_MASK);
    }

    /// Clear CORE0_IBUS_TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core0_ibus_timeout_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE0_IBUS_TIMEOUT_INT_RAW_MASK);
    }

    /// **CORE1_AHB_TIMEOUT_INT_RAW** `[1]` (RW)
    ///
    /// the raw interrupt status of hp core1 ahb timeout
    #[inline(always)]
    pub fn core1_ahb_timeout_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_AHB_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Set CORE1_AHB_TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core1_ahb_timeout_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE1_AHB_TIMEOUT_INT_RAW_MASK);
    }

    /// Clear CORE1_AHB_TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core1_ahb_timeout_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE1_AHB_TIMEOUT_INT_RAW_MASK);
    }

    /// **CORE0_AHB_TIMEOUT_INT_RAW** `[0]` (RW)
    ///
    /// the raw interrupt status of hp core0 ahb timeout
    #[inline(always)]
    pub fn core0_ahb_timeout_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_AHB_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Set CORE0_AHB_TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core0_ahb_timeout_int_raw(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE0_AHB_TIMEOUT_INT_RAW_MASK);
    }

    /// Clear CORE0_AHB_TIMEOUT_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core0_ahb_timeout_int_raw(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE0_AHB_TIMEOUT_INT_RAW_MASK);
    }
}

/// CORE_TIMEOUT_INT_RAW value type for batch read/modify/write operations
///
/// Use `coretimeoutintraw.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `coretimeoutintraw.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoreTimeoutIntRawVal {
    bits: u32,
}

impl CoreTimeoutIntRawVal {
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

    /// Read **CORE1_DBUS_TIMEOUT_INT_RAW** `[5:5]`
    ///
    /// the raw interrupt status of hp core1 dbus timeout
    #[inline(always)]
    pub const fn core1_dbus_timeout_int_raw(&self) -> bool {
        (self.bits & CoreTimeoutIntRaw::CORE1_DBUS_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Read **CORE0_DBUS_TIMEOUT_INT_RAW** `[4:4]`
    ///
    /// the raw interrupt status of hp core0 dbus timeout
    #[inline(always)]
    pub const fn core0_dbus_timeout_int_raw(&self) -> bool {
        (self.bits & CoreTimeoutIntRaw::CORE0_DBUS_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Read **CORE1_IBUS_TIMEOUT_INT_RAW** `[3:3]`
    ///
    /// the raw interrupt status of hp core1 ibus timeout
    #[inline(always)]
    pub const fn core1_ibus_timeout_int_raw(&self) -> bool {
        (self.bits & CoreTimeoutIntRaw::CORE1_IBUS_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Read **CORE0_IBUS_TIMEOUT_INT_RAW** `[2:2]`
    ///
    /// the raw interrupt status of hp core0 ibus timeout
    #[inline(always)]
    pub const fn core0_ibus_timeout_int_raw(&self) -> bool {
        (self.bits & CoreTimeoutIntRaw::CORE0_IBUS_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Read **CORE1_AHB_TIMEOUT_INT_RAW** `[1:1]`
    ///
    /// the raw interrupt status of hp core1 ahb timeout
    #[inline(always)]
    pub const fn core1_ahb_timeout_int_raw(&self) -> bool {
        (self.bits & CoreTimeoutIntRaw::CORE1_AHB_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Read **CORE0_AHB_TIMEOUT_INT_RAW** `[0:0]`
    ///
    /// the raw interrupt status of hp core0 ahb timeout
    #[inline(always)]
    pub const fn core0_ahb_timeout_int_raw(&self) -> bool {
        (self.bits & CoreTimeoutIntRaw::CORE0_AHB_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Set **CORE1_DBUS_TIMEOUT_INT_RAW** `[5:5]`
    ///
    /// the raw interrupt status of hp core1 dbus timeout
    #[inline(always)]
    pub const fn with_core1_dbus_timeout_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntRaw::CORE1_DBUS_TIMEOUT_INT_RAW_MASK
            } else {
                self.bits & !CoreTimeoutIntRaw::CORE1_DBUS_TIMEOUT_INT_RAW_MASK
            },
        }
    }

    /// Set **CORE0_DBUS_TIMEOUT_INT_RAW** `[4:4]`
    ///
    /// the raw interrupt status of hp core0 dbus timeout
    #[inline(always)]
    pub const fn with_core0_dbus_timeout_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntRaw::CORE0_DBUS_TIMEOUT_INT_RAW_MASK
            } else {
                self.bits & !CoreTimeoutIntRaw::CORE0_DBUS_TIMEOUT_INT_RAW_MASK
            },
        }
    }

    /// Set **CORE1_IBUS_TIMEOUT_INT_RAW** `[3:3]`
    ///
    /// the raw interrupt status of hp core1 ibus timeout
    #[inline(always)]
    pub const fn with_core1_ibus_timeout_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntRaw::CORE1_IBUS_TIMEOUT_INT_RAW_MASK
            } else {
                self.bits & !CoreTimeoutIntRaw::CORE1_IBUS_TIMEOUT_INT_RAW_MASK
            },
        }
    }

    /// Set **CORE0_IBUS_TIMEOUT_INT_RAW** `[2:2]`
    ///
    /// the raw interrupt status of hp core0 ibus timeout
    #[inline(always)]
    pub const fn with_core0_ibus_timeout_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntRaw::CORE0_IBUS_TIMEOUT_INT_RAW_MASK
            } else {
                self.bits & !CoreTimeoutIntRaw::CORE0_IBUS_TIMEOUT_INT_RAW_MASK
            },
        }
    }

    /// Set **CORE1_AHB_TIMEOUT_INT_RAW** `[1:1]`
    ///
    /// the raw interrupt status of hp core1 ahb timeout
    #[inline(always)]
    pub const fn with_core1_ahb_timeout_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntRaw::CORE1_AHB_TIMEOUT_INT_RAW_MASK
            } else {
                self.bits & !CoreTimeoutIntRaw::CORE1_AHB_TIMEOUT_INT_RAW_MASK
            },
        }
    }

    /// Set **CORE0_AHB_TIMEOUT_INT_RAW** `[0:0]`
    ///
    /// the raw interrupt status of hp core0 ahb timeout
    #[inline(always)]
    pub const fn with_core0_ahb_timeout_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntRaw::CORE0_AHB_TIMEOUT_INT_RAW_MASK
            } else {
                self.bits & !CoreTimeoutIntRaw::CORE0_AHB_TIMEOUT_INT_RAW_MASK
            },
        }
    }
}

impl Default for CoreTimeoutIntRawVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CoreTimeoutIntRaw {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CoreTimeoutIntRawVal {
        CoreTimeoutIntRawVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CoreTimeoutIntRawVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CoreTimeoutIntRawVal) -> CoreTimeoutIntRawVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_TIMEOUT_INT_ST Register
///
/// masked interrupt register
///
/// Address offset: 0x01ac
///
/// ## Fields
///
/// - **CORE1_DBUS_TIMEOUT_INT_ST** `[5]` (RO)
///
///   the masked interrupt status of hp core1 dbus timeout
///
/// - **CORE0_DBUS_TIMEOUT_INT_ST** `[4]` (RO)
///
///   the masked interrupt status of hp core0 dbus timeout
///
/// - **CORE1_IBUS_TIMEOUT_INT_ST** `[3]` (RO)
///
///   the masked interrupt status of hp core1 ibus timeout
///
/// - **CORE0_IBUS_TIMEOUT_INT_ST** `[2]` (RO)
///
///   the masked interrupt status of hp core0 ibus timeout
///
/// - **CORE1_AHB_TIMEOUT_INT_ST** `[1]` (RO)
///
///   the masked interrupt status of hp core1 ahb timeout
///
/// - **CORE0_AHB_TIMEOUT_INT_ST** `[0]` (RO)
///
///   the masked interrupt status of hp core0 ahb timeout
#[repr(transparent)]
pub struct CoreTimeoutIntSt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CoreTimeoutIntSt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoreTimeoutIntSt {
    // ======== Field Constants ========

    /// **CORE1_DBUS_TIMEOUT_INT_ST**
    ///
    /// the masked interrupt status of hp core1 dbus timeout
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_DBUS_TIMEOUT_INT_ST_OFFSET: u32 = 5;
    pub const CORE1_DBUS_TIMEOUT_INT_ST_MASK: u32 = 0x00000020;
    pub const CORE1_DBUS_TIMEOUT_INT_ST: u32 = 1 << 5;

    /// **CORE0_DBUS_TIMEOUT_INT_ST**
    ///
    /// the masked interrupt status of hp core0 dbus timeout
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_DBUS_TIMEOUT_INT_ST_OFFSET: u32 = 4;
    pub const CORE0_DBUS_TIMEOUT_INT_ST_MASK: u32 = 0x00000010;
    pub const CORE0_DBUS_TIMEOUT_INT_ST: u32 = 1 << 4;

    /// **CORE1_IBUS_TIMEOUT_INT_ST**
    ///
    /// the masked interrupt status of hp core1 ibus timeout
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CORE1_IBUS_TIMEOUT_INT_ST_OFFSET: u32 = 3;
    pub const CORE1_IBUS_TIMEOUT_INT_ST_MASK: u32 = 0x00000008;
    pub const CORE1_IBUS_TIMEOUT_INT_ST: u32 = 1 << 3;

    /// **CORE0_IBUS_TIMEOUT_INT_ST**
    ///
    /// the masked interrupt status of hp core0 ibus timeout
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CORE0_IBUS_TIMEOUT_INT_ST_OFFSET: u32 = 2;
    pub const CORE0_IBUS_TIMEOUT_INT_ST_MASK: u32 = 0x00000004;
    pub const CORE0_IBUS_TIMEOUT_INT_ST: u32 = 1 << 2;

    /// **CORE1_AHB_TIMEOUT_INT_ST**
    ///
    /// the masked interrupt status of hp core1 ahb timeout
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE1_AHB_TIMEOUT_INT_ST_OFFSET: u32 = 1;
    pub const CORE1_AHB_TIMEOUT_INT_ST_MASK: u32 = 0x00000002;
    pub const CORE1_AHB_TIMEOUT_INT_ST: u32 = 1 << 1;

    /// **CORE0_AHB_TIMEOUT_INT_ST**
    ///
    /// the masked interrupt status of hp core0 ahb timeout
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE0_AHB_TIMEOUT_INT_ST_OFFSET: u32 = 0;
    pub const CORE0_AHB_TIMEOUT_INT_ST_MASK: u32 = 0x00000001;
    pub const CORE0_AHB_TIMEOUT_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE1_DBUS_TIMEOUT_INT_ST** `[5]` (RO)
    ///
    /// the masked interrupt status of hp core1 dbus timeout
    #[inline(always)]
    pub fn core1_dbus_timeout_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_DBUS_TIMEOUT_INT_ST_MASK) != 0
    }

    /// **CORE0_DBUS_TIMEOUT_INT_ST** `[4]` (RO)
    ///
    /// the masked interrupt status of hp core0 dbus timeout
    #[inline(always)]
    pub fn core0_dbus_timeout_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_DBUS_TIMEOUT_INT_ST_MASK) != 0
    }

    /// **CORE1_IBUS_TIMEOUT_INT_ST** `[3]` (RO)
    ///
    /// the masked interrupt status of hp core1 ibus timeout
    #[inline(always)]
    pub fn core1_ibus_timeout_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_IBUS_TIMEOUT_INT_ST_MASK) != 0
    }

    /// **CORE0_IBUS_TIMEOUT_INT_ST** `[2]` (RO)
    ///
    /// the masked interrupt status of hp core0 ibus timeout
    #[inline(always)]
    pub fn core0_ibus_timeout_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_IBUS_TIMEOUT_INT_ST_MASK) != 0
    }

    /// **CORE1_AHB_TIMEOUT_INT_ST** `[1]` (RO)
    ///
    /// the masked interrupt status of hp core1 ahb timeout
    #[inline(always)]
    pub fn core1_ahb_timeout_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_AHB_TIMEOUT_INT_ST_MASK) != 0
    }

    /// **CORE0_AHB_TIMEOUT_INT_ST** `[0]` (RO)
    ///
    /// the masked interrupt status of hp core0 ahb timeout
    #[inline(always)]
    pub fn core0_ahb_timeout_int_st(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_AHB_TIMEOUT_INT_ST_MASK) != 0
    }
}

/// CORE_TIMEOUT_INT_ST value type for batch read/modify/write operations
///
/// Use `coretimeoutintst.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `coretimeoutintst.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoreTimeoutIntStVal {
    bits: u32,
}

impl CoreTimeoutIntStVal {
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

    /// Read **CORE1_DBUS_TIMEOUT_INT_ST** `[5:5]`
    ///
    /// the masked interrupt status of hp core1 dbus timeout
    #[inline(always)]
    pub const fn core1_dbus_timeout_int_st(&self) -> bool {
        (self.bits & CoreTimeoutIntSt::CORE1_DBUS_TIMEOUT_INT_ST_MASK) != 0
    }

    /// Read **CORE0_DBUS_TIMEOUT_INT_ST** `[4:4]`
    ///
    /// the masked interrupt status of hp core0 dbus timeout
    #[inline(always)]
    pub const fn core0_dbus_timeout_int_st(&self) -> bool {
        (self.bits & CoreTimeoutIntSt::CORE0_DBUS_TIMEOUT_INT_ST_MASK) != 0
    }

    /// Read **CORE1_IBUS_TIMEOUT_INT_ST** `[3:3]`
    ///
    /// the masked interrupt status of hp core1 ibus timeout
    #[inline(always)]
    pub const fn core1_ibus_timeout_int_st(&self) -> bool {
        (self.bits & CoreTimeoutIntSt::CORE1_IBUS_TIMEOUT_INT_ST_MASK) != 0
    }

    /// Read **CORE0_IBUS_TIMEOUT_INT_ST** `[2:2]`
    ///
    /// the masked interrupt status of hp core0 ibus timeout
    #[inline(always)]
    pub const fn core0_ibus_timeout_int_st(&self) -> bool {
        (self.bits & CoreTimeoutIntSt::CORE0_IBUS_TIMEOUT_INT_ST_MASK) != 0
    }

    /// Read **CORE1_AHB_TIMEOUT_INT_ST** `[1:1]`
    ///
    /// the masked interrupt status of hp core1 ahb timeout
    #[inline(always)]
    pub const fn core1_ahb_timeout_int_st(&self) -> bool {
        (self.bits & CoreTimeoutIntSt::CORE1_AHB_TIMEOUT_INT_ST_MASK) != 0
    }

    /// Read **CORE0_AHB_TIMEOUT_INT_ST** `[0:0]`
    ///
    /// the masked interrupt status of hp core0 ahb timeout
    #[inline(always)]
    pub const fn core0_ahb_timeout_int_st(&self) -> bool {
        (self.bits & CoreTimeoutIntSt::CORE0_AHB_TIMEOUT_INT_ST_MASK) != 0
    }
}

impl Default for CoreTimeoutIntStVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CoreTimeoutIntSt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CoreTimeoutIntStVal {
        CoreTimeoutIntStVal::from_bits(self.value.read_bits())
    }
}

/// CORE_TIMEOUT_INT_ENA Register
///
/// masked interrupt register
///
/// Address offset: 0x01b0
///
/// ## Fields
///
/// - **CORE1_DBUS_TIMEOUT_INT_ENA** `[5]` (RW)
///
///   Write 1 to enable hp_core1_dbus_timeout int
///
/// - **CORE0_DBUS_TIMEOUT_INT_ENA** `[4]` (RW)
///
///   Write 1 to enable hp_core0_dbus_timeout int
///
/// - **CORE1_IBUS_TIMEOUT_INT_ENA** `[3]` (RW)
///
///   Write 1 to enable hp_core1_ibus_timeout int
///
/// - **CORE0_IBUS_TIMEOUT_INT_ENA** `[2]` (RW)
///
///   Write 1 to enable hp_core0_ibus_timeout int
///
/// - **CORE1_AHB_TIMEOUT_INT_ENA** `[1]` (RW)
///
///   Write 1 to enable hp_core1_ahb_timeout int
///
/// - **CORE0_AHB_TIMEOUT_INT_ENA** `[0]` (RW)
///
///   Write 1 to enable hp_core0_ahb_timeout int
#[repr(transparent)]
pub struct CoreTimeoutIntEna {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CoreTimeoutIntEna {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoreTimeoutIntEna {
    // ======== Field Constants ========

    /// **CORE1_DBUS_TIMEOUT_INT_ENA**
    ///
    /// Write 1 to enable hp_core1_dbus_timeout int
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_DBUS_TIMEOUT_INT_ENA_OFFSET: u32 = 5;
    pub const CORE1_DBUS_TIMEOUT_INT_ENA_MASK: u32 = 0x00000020;
    pub const CORE1_DBUS_TIMEOUT_INT_ENA: u32 = 1 << 5;

    /// **CORE0_DBUS_TIMEOUT_INT_ENA**
    ///
    /// Write 1 to enable hp_core0_dbus_timeout int
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_DBUS_TIMEOUT_INT_ENA_OFFSET: u32 = 4;
    pub const CORE0_DBUS_TIMEOUT_INT_ENA_MASK: u32 = 0x00000010;
    pub const CORE0_DBUS_TIMEOUT_INT_ENA: u32 = 1 << 4;

    /// **CORE1_IBUS_TIMEOUT_INT_ENA**
    ///
    /// Write 1 to enable hp_core1_ibus_timeout int
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CORE1_IBUS_TIMEOUT_INT_ENA_OFFSET: u32 = 3;
    pub const CORE1_IBUS_TIMEOUT_INT_ENA_MASK: u32 = 0x00000008;
    pub const CORE1_IBUS_TIMEOUT_INT_ENA: u32 = 1 << 3;

    /// **CORE0_IBUS_TIMEOUT_INT_ENA**
    ///
    /// Write 1 to enable hp_core0_ibus_timeout int
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CORE0_IBUS_TIMEOUT_INT_ENA_OFFSET: u32 = 2;
    pub const CORE0_IBUS_TIMEOUT_INT_ENA_MASK: u32 = 0x00000004;
    pub const CORE0_IBUS_TIMEOUT_INT_ENA: u32 = 1 << 2;

    /// **CORE1_AHB_TIMEOUT_INT_ENA**
    ///
    /// Write 1 to enable hp_core1_ahb_timeout int
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE1_AHB_TIMEOUT_INT_ENA_OFFSET: u32 = 1;
    pub const CORE1_AHB_TIMEOUT_INT_ENA_MASK: u32 = 0x00000002;
    pub const CORE1_AHB_TIMEOUT_INT_ENA: u32 = 1 << 1;

    /// **CORE0_AHB_TIMEOUT_INT_ENA**
    ///
    /// Write 1 to enable hp_core0_ahb_timeout int
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE0_AHB_TIMEOUT_INT_ENA_OFFSET: u32 = 0;
    pub const CORE0_AHB_TIMEOUT_INT_ENA_MASK: u32 = 0x00000001;
    pub const CORE0_AHB_TIMEOUT_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CORE1_DBUS_TIMEOUT_INT_ENA** `[5]` (RW)
    ///
    /// Write 1 to enable hp_core1_dbus_timeout int
    #[inline(always)]
    pub fn core1_dbus_timeout_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_DBUS_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Set CORE1_DBUS_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core1_dbus_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE1_DBUS_TIMEOUT_INT_ENA_MASK);
    }

    /// Clear CORE1_DBUS_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core1_dbus_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE1_DBUS_TIMEOUT_INT_ENA_MASK);
    }

    /// **CORE0_DBUS_TIMEOUT_INT_ENA** `[4]` (RW)
    ///
    /// Write 1 to enable hp_core0_dbus_timeout int
    #[inline(always)]
    pub fn core0_dbus_timeout_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_DBUS_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Set CORE0_DBUS_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core0_dbus_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE0_DBUS_TIMEOUT_INT_ENA_MASK);
    }

    /// Clear CORE0_DBUS_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core0_dbus_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE0_DBUS_TIMEOUT_INT_ENA_MASK);
    }

    /// **CORE1_IBUS_TIMEOUT_INT_ENA** `[3]` (RW)
    ///
    /// Write 1 to enable hp_core1_ibus_timeout int
    #[inline(always)]
    pub fn core1_ibus_timeout_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_IBUS_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Set CORE1_IBUS_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core1_ibus_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE1_IBUS_TIMEOUT_INT_ENA_MASK);
    }

    /// Clear CORE1_IBUS_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core1_ibus_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE1_IBUS_TIMEOUT_INT_ENA_MASK);
    }

    /// **CORE0_IBUS_TIMEOUT_INT_ENA** `[2]` (RW)
    ///
    /// Write 1 to enable hp_core0_ibus_timeout int
    #[inline(always)]
    pub fn core0_ibus_timeout_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_IBUS_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Set CORE0_IBUS_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core0_ibus_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE0_IBUS_TIMEOUT_INT_ENA_MASK);
    }

    /// Clear CORE0_IBUS_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core0_ibus_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE0_IBUS_TIMEOUT_INT_ENA_MASK);
    }

    /// **CORE1_AHB_TIMEOUT_INT_ENA** `[1]` (RW)
    ///
    /// Write 1 to enable hp_core1_ahb_timeout int
    #[inline(always)]
    pub fn core1_ahb_timeout_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE1_AHB_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Set CORE1_AHB_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core1_ahb_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE1_AHB_TIMEOUT_INT_ENA_MASK);
    }

    /// Clear CORE1_AHB_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core1_ahb_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE1_AHB_TIMEOUT_INT_ENA_MASK);
    }

    /// **CORE0_AHB_TIMEOUT_INT_ENA** `[0]` (RW)
    ///
    /// Write 1 to enable hp_core0_ahb_timeout int
    #[inline(always)]
    pub fn core0_ahb_timeout_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::CORE0_AHB_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Set CORE0_AHB_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_core0_ahb_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::CORE0_AHB_TIMEOUT_INT_ENA_MASK);
    }

    /// Clear CORE0_AHB_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_core0_ahb_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::CORE0_AHB_TIMEOUT_INT_ENA_MASK);
    }
}

/// CORE_TIMEOUT_INT_ENA value type for batch read/modify/write operations
///
/// Use `coretimeoutintena.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `coretimeoutintena.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoreTimeoutIntEnaVal {
    bits: u32,
}

impl CoreTimeoutIntEnaVal {
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

    /// Read **CORE1_DBUS_TIMEOUT_INT_ENA** `[5:5]`
    ///
    /// Write 1 to enable hp_core1_dbus_timeout int
    #[inline(always)]
    pub const fn core1_dbus_timeout_int_ena(&self) -> bool {
        (self.bits & CoreTimeoutIntEna::CORE1_DBUS_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Read **CORE0_DBUS_TIMEOUT_INT_ENA** `[4:4]`
    ///
    /// Write 1 to enable hp_core0_dbus_timeout int
    #[inline(always)]
    pub const fn core0_dbus_timeout_int_ena(&self) -> bool {
        (self.bits & CoreTimeoutIntEna::CORE0_DBUS_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Read **CORE1_IBUS_TIMEOUT_INT_ENA** `[3:3]`
    ///
    /// Write 1 to enable hp_core1_ibus_timeout int
    #[inline(always)]
    pub const fn core1_ibus_timeout_int_ena(&self) -> bool {
        (self.bits & CoreTimeoutIntEna::CORE1_IBUS_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Read **CORE0_IBUS_TIMEOUT_INT_ENA** `[2:2]`
    ///
    /// Write 1 to enable hp_core0_ibus_timeout int
    #[inline(always)]
    pub const fn core0_ibus_timeout_int_ena(&self) -> bool {
        (self.bits & CoreTimeoutIntEna::CORE0_IBUS_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Read **CORE1_AHB_TIMEOUT_INT_ENA** `[1:1]`
    ///
    /// Write 1 to enable hp_core1_ahb_timeout int
    #[inline(always)]
    pub const fn core1_ahb_timeout_int_ena(&self) -> bool {
        (self.bits & CoreTimeoutIntEna::CORE1_AHB_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Read **CORE0_AHB_TIMEOUT_INT_ENA** `[0:0]`
    ///
    /// Write 1 to enable hp_core0_ahb_timeout int
    #[inline(always)]
    pub const fn core0_ahb_timeout_int_ena(&self) -> bool {
        (self.bits & CoreTimeoutIntEna::CORE0_AHB_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Set **CORE1_DBUS_TIMEOUT_INT_ENA** `[5:5]`
    ///
    /// Write 1 to enable hp_core1_dbus_timeout int
    #[inline(always)]
    pub const fn with_core1_dbus_timeout_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntEna::CORE1_DBUS_TIMEOUT_INT_ENA_MASK
            } else {
                self.bits & !CoreTimeoutIntEna::CORE1_DBUS_TIMEOUT_INT_ENA_MASK
            },
        }
    }

    /// Set **CORE0_DBUS_TIMEOUT_INT_ENA** `[4:4]`
    ///
    /// Write 1 to enable hp_core0_dbus_timeout int
    #[inline(always)]
    pub const fn with_core0_dbus_timeout_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntEna::CORE0_DBUS_TIMEOUT_INT_ENA_MASK
            } else {
                self.bits & !CoreTimeoutIntEna::CORE0_DBUS_TIMEOUT_INT_ENA_MASK
            },
        }
    }

    /// Set **CORE1_IBUS_TIMEOUT_INT_ENA** `[3:3]`
    ///
    /// Write 1 to enable hp_core1_ibus_timeout int
    #[inline(always)]
    pub const fn with_core1_ibus_timeout_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntEna::CORE1_IBUS_TIMEOUT_INT_ENA_MASK
            } else {
                self.bits & !CoreTimeoutIntEna::CORE1_IBUS_TIMEOUT_INT_ENA_MASK
            },
        }
    }

    /// Set **CORE0_IBUS_TIMEOUT_INT_ENA** `[2:2]`
    ///
    /// Write 1 to enable hp_core0_ibus_timeout int
    #[inline(always)]
    pub const fn with_core0_ibus_timeout_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntEna::CORE0_IBUS_TIMEOUT_INT_ENA_MASK
            } else {
                self.bits & !CoreTimeoutIntEna::CORE0_IBUS_TIMEOUT_INT_ENA_MASK
            },
        }
    }

    /// Set **CORE1_AHB_TIMEOUT_INT_ENA** `[1:1]`
    ///
    /// Write 1 to enable hp_core1_ahb_timeout int
    #[inline(always)]
    pub const fn with_core1_ahb_timeout_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntEna::CORE1_AHB_TIMEOUT_INT_ENA_MASK
            } else {
                self.bits & !CoreTimeoutIntEna::CORE1_AHB_TIMEOUT_INT_ENA_MASK
            },
        }
    }

    /// Set **CORE0_AHB_TIMEOUT_INT_ENA** `[0:0]`
    ///
    /// Write 1 to enable hp_core0_ahb_timeout int
    #[inline(always)]
    pub const fn with_core0_ahb_timeout_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntEna::CORE0_AHB_TIMEOUT_INT_ENA_MASK
            } else {
                self.bits & !CoreTimeoutIntEna::CORE0_AHB_TIMEOUT_INT_ENA_MASK
            },
        }
    }
}

impl Default for CoreTimeoutIntEnaVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CoreTimeoutIntEna {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> CoreTimeoutIntEnaVal {
        CoreTimeoutIntEnaVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CoreTimeoutIntEnaVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(CoreTimeoutIntEnaVal) -> CoreTimeoutIntEnaVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CORE_TIMEOUT_INT_CLR Register
///
/// interrupt clear register
///
/// Address offset: 0x01b4
///
/// ## Fields
///
/// - **CORE1_DBUS_TIMEOUT_INT_CLR** `[5]` (WO)
///
///   Write 1 to clear hp_core1_dbus_timeout int
///
/// - **CORE0_DBUS_TIMEOUT_INT_CLR** `[4]` (WO)
///
///   Write 1 to clear hp_core0_dbus_timeout int
///
/// - **CORE1_IBUS_TIMEOUT_INT_CLR** `[3]` (WO)
///
///   Write 1 to clear hp_core1_ibus_timeout int
///
/// - **CORE0_IBUS_TIMEOUT_INT_CLR** `[2]` (WO)
///
///   Write 1 to clear hp_core0_ibus_timeout int
///
/// - **CORE1_AHB_TIMEOUT_INT_CLR** `[1]` (WO)
///
///   Write 1 to clear hp_core1_ahb_timeout int
///
/// - **CORE0_AHB_TIMEOUT_INT_CLR** `[0]` (WO)
///
///   Write 1 to clear hp_core0_ahb_timeout int
#[repr(transparent)]
pub struct CoreTimeoutIntClr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for CoreTimeoutIntClr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl CoreTimeoutIntClr {
    // ======== Field Constants ========

    /// **CORE1_DBUS_TIMEOUT_INT_CLR**
    ///
    /// Write 1 to clear hp_core1_dbus_timeout int
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const CORE1_DBUS_TIMEOUT_INT_CLR_OFFSET: u32 = 5;
    pub const CORE1_DBUS_TIMEOUT_INT_CLR_MASK: u32 = 0x00000020;
    pub const CORE1_DBUS_TIMEOUT_INT_CLR: u32 = 1 << 5;

    /// **CORE0_DBUS_TIMEOUT_INT_CLR**
    ///
    /// Write 1 to clear hp_core0_dbus_timeout int
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const CORE0_DBUS_TIMEOUT_INT_CLR_OFFSET: u32 = 4;
    pub const CORE0_DBUS_TIMEOUT_INT_CLR_MASK: u32 = 0x00000010;
    pub const CORE0_DBUS_TIMEOUT_INT_CLR: u32 = 1 << 4;

    /// **CORE1_IBUS_TIMEOUT_INT_CLR**
    ///
    /// Write 1 to clear hp_core1_ibus_timeout int
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const CORE1_IBUS_TIMEOUT_INT_CLR_OFFSET: u32 = 3;
    pub const CORE1_IBUS_TIMEOUT_INT_CLR_MASK: u32 = 0x00000008;
    pub const CORE1_IBUS_TIMEOUT_INT_CLR: u32 = 1 << 3;

    /// **CORE0_IBUS_TIMEOUT_INT_CLR**
    ///
    /// Write 1 to clear hp_core0_ibus_timeout int
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const CORE0_IBUS_TIMEOUT_INT_CLR_OFFSET: u32 = 2;
    pub const CORE0_IBUS_TIMEOUT_INT_CLR_MASK: u32 = 0x00000004;
    pub const CORE0_IBUS_TIMEOUT_INT_CLR: u32 = 1 << 2;

    /// **CORE1_AHB_TIMEOUT_INT_CLR**
    ///
    /// Write 1 to clear hp_core1_ahb_timeout int
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const CORE1_AHB_TIMEOUT_INT_CLR_OFFSET: u32 = 1;
    pub const CORE1_AHB_TIMEOUT_INT_CLR_MASK: u32 = 0x00000002;
    pub const CORE1_AHB_TIMEOUT_INT_CLR: u32 = 1 << 1;

    /// **CORE0_AHB_TIMEOUT_INT_CLR**
    ///
    /// Write 1 to clear hp_core0_ahb_timeout int
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CORE0_AHB_TIMEOUT_INT_CLR_OFFSET: u32 = 0;
    pub const CORE0_AHB_TIMEOUT_INT_CLR_MASK: u32 = 0x00000001;
    pub const CORE0_AHB_TIMEOUT_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **CORE1_DBUS_TIMEOUT_INT_CLR** (WO)
    ///
    /// Write 1 to clear hp_core1_dbus_timeout int
    #[inline(always)]
    pub fn trigger_core1_dbus_timeout_int_clr(&self) {
        self.value.write_bits(Self::CORE1_DBUS_TIMEOUT_INT_CLR_MASK);
    }

    /// Trigger **CORE0_DBUS_TIMEOUT_INT_CLR** (WO)
    ///
    /// Write 1 to clear hp_core0_dbus_timeout int
    #[inline(always)]
    pub fn trigger_core0_dbus_timeout_int_clr(&self) {
        self.value.write_bits(Self::CORE0_DBUS_TIMEOUT_INT_CLR_MASK);
    }

    /// Trigger **CORE1_IBUS_TIMEOUT_INT_CLR** (WO)
    ///
    /// Write 1 to clear hp_core1_ibus_timeout int
    #[inline(always)]
    pub fn trigger_core1_ibus_timeout_int_clr(&self) {
        self.value.write_bits(Self::CORE1_IBUS_TIMEOUT_INT_CLR_MASK);
    }

    /// Trigger **CORE0_IBUS_TIMEOUT_INT_CLR** (WO)
    ///
    /// Write 1 to clear hp_core0_ibus_timeout int
    #[inline(always)]
    pub fn trigger_core0_ibus_timeout_int_clr(&self) {
        self.value.write_bits(Self::CORE0_IBUS_TIMEOUT_INT_CLR_MASK);
    }

    /// Trigger **CORE1_AHB_TIMEOUT_INT_CLR** (WO)
    ///
    /// Write 1 to clear hp_core1_ahb_timeout int
    #[inline(always)]
    pub fn trigger_core1_ahb_timeout_int_clr(&self) {
        self.value.write_bits(Self::CORE1_AHB_TIMEOUT_INT_CLR_MASK);
    }

    /// Trigger **CORE0_AHB_TIMEOUT_INT_CLR** (WO)
    ///
    /// Write 1 to clear hp_core0_ahb_timeout int
    #[inline(always)]
    pub fn trigger_core0_ahb_timeout_int_clr(&self) {
        self.value.write_bits(Self::CORE0_AHB_TIMEOUT_INT_CLR_MASK);
    }
}

/// CORE_TIMEOUT_INT_CLR value type for batch read/modify/write operations
///
/// Use `coretimeoutintclr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `coretimeoutintclr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct CoreTimeoutIntClrVal {
    bits: u32,
}

impl CoreTimeoutIntClrVal {
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

    /// Set **CORE1_DBUS_TIMEOUT_INT_CLR** `[5:5]`
    ///
    /// Write 1 to clear hp_core1_dbus_timeout int
    #[inline(always)]
    pub const fn with_core1_dbus_timeout_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntClr::CORE1_DBUS_TIMEOUT_INT_CLR_MASK
            } else {
                self.bits & !CoreTimeoutIntClr::CORE1_DBUS_TIMEOUT_INT_CLR_MASK
            },
        }
    }

    /// Set **CORE0_DBUS_TIMEOUT_INT_CLR** `[4:4]`
    ///
    /// Write 1 to clear hp_core0_dbus_timeout int
    #[inline(always)]
    pub const fn with_core0_dbus_timeout_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntClr::CORE0_DBUS_TIMEOUT_INT_CLR_MASK
            } else {
                self.bits & !CoreTimeoutIntClr::CORE0_DBUS_TIMEOUT_INT_CLR_MASK
            },
        }
    }

    /// Set **CORE1_IBUS_TIMEOUT_INT_CLR** `[3:3]`
    ///
    /// Write 1 to clear hp_core1_ibus_timeout int
    #[inline(always)]
    pub const fn with_core1_ibus_timeout_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntClr::CORE1_IBUS_TIMEOUT_INT_CLR_MASK
            } else {
                self.bits & !CoreTimeoutIntClr::CORE1_IBUS_TIMEOUT_INT_CLR_MASK
            },
        }
    }

    /// Set **CORE0_IBUS_TIMEOUT_INT_CLR** `[2:2]`
    ///
    /// Write 1 to clear hp_core0_ibus_timeout int
    #[inline(always)]
    pub const fn with_core0_ibus_timeout_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntClr::CORE0_IBUS_TIMEOUT_INT_CLR_MASK
            } else {
                self.bits & !CoreTimeoutIntClr::CORE0_IBUS_TIMEOUT_INT_CLR_MASK
            },
        }
    }

    /// Set **CORE1_AHB_TIMEOUT_INT_CLR** `[1:1]`
    ///
    /// Write 1 to clear hp_core1_ahb_timeout int
    #[inline(always)]
    pub const fn with_core1_ahb_timeout_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntClr::CORE1_AHB_TIMEOUT_INT_CLR_MASK
            } else {
                self.bits & !CoreTimeoutIntClr::CORE1_AHB_TIMEOUT_INT_CLR_MASK
            },
        }
    }

    /// Set **CORE0_AHB_TIMEOUT_INT_CLR** `[0:0]`
    ///
    /// Write 1 to clear hp_core0_ahb_timeout int
    #[inline(always)]
    pub const fn with_core0_ahb_timeout_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | CoreTimeoutIntClr::CORE0_AHB_TIMEOUT_INT_CLR_MASK
            } else {
                self.bits & !CoreTimeoutIntClr::CORE0_AHB_TIMEOUT_INT_CLR_MASK
            },
        }
    }
}

impl Default for CoreTimeoutIntClrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl CoreTimeoutIntClr {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: CoreTimeoutIntClrVal) {
        self.value.write_bits(val.bits())
    }
}

/// GPIO_O_HYS_CTRL0 Register
///
/// NA
///
/// Address offset: 0x01c0
///
/// ## Fields
///
/// - **REG_GPIO_0_HYS_LOW** `[31:0]` (RW)
///
///   hys control for gpio47~16
#[repr(transparent)]
pub struct GpioOHysCtrl0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioOHysCtrl0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioOHysCtrl0 {
    // ======== Field Constants ========

    /// **REG_GPIO_0_HYS_LOW**
    ///
    /// hys control for gpio47~16
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const REG_GPIO_0_HYS_LOW_OFFSET: u32 = 0;
    pub const REG_GPIO_0_HYS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **REG_GPIO_0_HYS_LOW** `[31:0]` (RW)
    ///
    /// hys control for gpio47~16
    #[inline(always)]
    pub fn reg_gpio_0_hys_low(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_0_HYS_LOW_MASK) >> Self::REG_GPIO_0_HYS_LOW_OFFSET
    }

    /// Write **REG_GPIO_0_HYS_LOW** `[31:0]` (RW)
    ///
    /// hys control for gpio47~16
    #[inline(always)]
    pub fn write_reg_gpio_0_hys_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_0_HYS_LOW_MASK)
                | ((value << Self::REG_GPIO_0_HYS_LOW_OFFSET) & Self::REG_GPIO_0_HYS_LOW_MASK)
        });
    }
}

/// GPIO_O_HYS_CTRL0 value type for batch read/modify/write operations
///
/// Use `gpioohysctrl0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioohysctrl0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioOHysCtrl0Val {
    bits: u32,
}

impl GpioOHysCtrl0Val {
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

    /// Read **REG_GPIO_0_HYS_LOW** `[31:0]`
    ///
    /// hys control for gpio47~16
    #[inline(always)]
    pub const fn reg_gpio_0_hys_low(&self) -> u32 {
        (self.bits & GpioOHysCtrl0::REG_GPIO_0_HYS_LOW_MASK)
            >> GpioOHysCtrl0::REG_GPIO_0_HYS_LOW_OFFSET
    }

    /// Set **REG_GPIO_0_HYS_LOW** `[31:0]`
    ///
    /// hys control for gpio47~16
    #[inline(always)]
    pub const fn with_reg_gpio_0_hys_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioOHysCtrl0::REG_GPIO_0_HYS_LOW_MASK)
                | ((value << GpioOHysCtrl0::REG_GPIO_0_HYS_LOW_OFFSET)
                    & GpioOHysCtrl0::REG_GPIO_0_HYS_LOW_MASK),
        }
    }
}

impl Default for GpioOHysCtrl0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GpioOHysCtrl0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioOHysCtrl0Val {
        GpioOHysCtrl0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioOHysCtrl0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioOHysCtrl0Val) -> GpioOHysCtrl0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// GPIO_O_HYS_CTRL1 Register
///
/// NA
///
/// Address offset: 0x01c4
///
/// ## Fields
///
/// - **REG_GPIO_0_HYS_HIGH** `[8:0]` (RW)
///
///   hys control for gpio56~48
#[repr(transparent)]
pub struct GpioOHysCtrl1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for GpioOHysCtrl1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl GpioOHysCtrl1 {
    // ======== Field Constants ========

    /// **REG_GPIO_0_HYS_HIGH**
    ///
    /// hys control for gpio56~48
    ///
    /// Bits `[8:0]`, mask `0x000001ff`
    pub const REG_GPIO_0_HYS_HIGH_OFFSET: u32 = 0;
    pub const REG_GPIO_0_HYS_HIGH_MASK: u32 = 0x000001ff;

    // ======== Field Accessors ========

    /// **REG_GPIO_0_HYS_HIGH** `[8:0]` (RW)
    ///
    /// hys control for gpio56~48
    #[inline(always)]
    pub fn reg_gpio_0_hys_high(&self) -> u32 {
        (self.value.read_bits() & Self::REG_GPIO_0_HYS_HIGH_MASK)
            >> Self::REG_GPIO_0_HYS_HIGH_OFFSET
    }

    /// Write **REG_GPIO_0_HYS_HIGH** `[8:0]` (RW)
    ///
    /// hys control for gpio56~48
    #[inline(always)]
    pub fn write_reg_gpio_0_hys_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::REG_GPIO_0_HYS_HIGH_MASK)
                | ((value << Self::REG_GPIO_0_HYS_HIGH_OFFSET) & Self::REG_GPIO_0_HYS_HIGH_MASK)
        });
    }
}

/// GPIO_O_HYS_CTRL1 value type for batch read/modify/write operations
///
/// Use `gpioohysctrl1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `gpioohysctrl1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct GpioOHysCtrl1Val {
    bits: u32,
}

impl GpioOHysCtrl1Val {
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

    /// Read **REG_GPIO_0_HYS_HIGH** `[8:0]`
    ///
    /// hys control for gpio56~48
    #[inline(always)]
    pub const fn reg_gpio_0_hys_high(&self) -> u32 {
        (self.bits & GpioOHysCtrl1::REG_GPIO_0_HYS_HIGH_MASK)
            >> GpioOHysCtrl1::REG_GPIO_0_HYS_HIGH_OFFSET
    }

    /// Set **REG_GPIO_0_HYS_HIGH** `[8:0]`
    ///
    /// hys control for gpio56~48
    #[inline(always)]
    pub const fn with_reg_gpio_0_hys_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !GpioOHysCtrl1::REG_GPIO_0_HYS_HIGH_MASK)
                | ((value << GpioOHysCtrl1::REG_GPIO_0_HYS_HIGH_OFFSET)
                    & GpioOHysCtrl1::REG_GPIO_0_HYS_HIGH_MASK),
        }
    }
}

impl Default for GpioOHysCtrl1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl GpioOHysCtrl1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> GpioOHysCtrl1Val {
        GpioOHysCtrl1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: GpioOHysCtrl1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(GpioOHysCtrl1Val) -> GpioOHysCtrl1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RSA_PD_CTRL Register
///
/// rsa pd ctrl register
///
/// Address offset: 0x01d0
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **RSA_MEM_PD** `[2]` (RW)
///
///   Set this bit to force power down rsa internal memory.
///
/// - **RSA_MEM_FORCE_PU** `[1]` (RW)
///
///   Set this bit to force power up rsa internal memory
///
/// - **RSA_MEM_FORCE_PD** `[0]` (RW)
///
///   Set this bit to power down rsa internal memory.
#[repr(transparent)]
pub struct RsaPdCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RsaPdCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RsaPdCtrl {
    // ======== Field Constants ========

    /// **RSA_MEM_PD**
    ///
    /// Set this bit to force power down rsa internal memory.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RSA_MEM_PD_OFFSET: u32 = 2;
    pub const RSA_MEM_PD_MASK: u32 = 0x00000004;
    pub const RSA_MEM_PD: u32 = 1 << 2;

    /// **RSA_MEM_FORCE_PU**
    ///
    /// Set this bit to force power up rsa internal memory
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const RSA_MEM_FORCE_PU_OFFSET: u32 = 1;
    pub const RSA_MEM_FORCE_PU_MASK: u32 = 0x00000002;
    pub const RSA_MEM_FORCE_PU: u32 = 1 << 1;

    /// **RSA_MEM_FORCE_PD**
    ///
    /// Set this bit to power down rsa internal memory.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RSA_MEM_FORCE_PD_OFFSET: u32 = 0;
    pub const RSA_MEM_FORCE_PD_MASK: u32 = 0x00000001;
    pub const RSA_MEM_FORCE_PD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RSA_MEM_PD** `[2]` (RW)
    ///
    /// Set this bit to force power down rsa internal memory.
    #[inline(always)]
    pub fn rsa_mem_pd(&self) -> bool {
        (self.value.read_bits() & Self::RSA_MEM_PD_MASK) != 0
    }

    /// Set RSA_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rsa_mem_pd(&self) {
        self.value.modify_bits(|r| r | Self::RSA_MEM_PD_MASK);
    }

    /// Clear RSA_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rsa_mem_pd(&self) {
        self.value.modify_bits(|r| r & !Self::RSA_MEM_PD_MASK);
    }

    /// **RSA_MEM_FORCE_PU** `[1]` (RW)
    ///
    /// Set this bit to force power up rsa internal memory
    #[inline(always)]
    pub fn rsa_mem_force_pu(&self) -> bool {
        (self.value.read_bits() & Self::RSA_MEM_FORCE_PU_MASK) != 0
    }

    /// Set RSA_MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rsa_mem_force_pu(&self) {
        self.value.modify_bits(|r| r | Self::RSA_MEM_FORCE_PU_MASK);
    }

    /// Clear RSA_MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rsa_mem_force_pu(&self) {
        self.value.modify_bits(|r| r & !Self::RSA_MEM_FORCE_PU_MASK);
    }

    /// **RSA_MEM_FORCE_PD** `[0]` (RW)
    ///
    /// Set this bit to power down rsa internal memory.
    #[inline(always)]
    pub fn rsa_mem_force_pd(&self) -> bool {
        (self.value.read_bits() & Self::RSA_MEM_FORCE_PD_MASK) != 0
    }

    /// Set RSA_MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rsa_mem_force_pd(&self) {
        self.value.modify_bits(|r| r | Self::RSA_MEM_FORCE_PD_MASK);
    }

    /// Clear RSA_MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rsa_mem_force_pd(&self) {
        self.value.modify_bits(|r| r & !Self::RSA_MEM_FORCE_PD_MASK);
    }
}

/// RSA_PD_CTRL value type for batch read/modify/write operations
///
/// Use `rsapdctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rsapdctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RsaPdCtrlVal {
    bits: u32,
}

impl RsaPdCtrlVal {
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

    /// Read **RSA_MEM_PD** `[2:2]`
    ///
    /// Set this bit to force power down rsa internal memory.
    #[inline(always)]
    pub const fn rsa_mem_pd(&self) -> bool {
        (self.bits & RsaPdCtrl::RSA_MEM_PD_MASK) != 0
    }

    /// Read **RSA_MEM_FORCE_PU** `[1:1]`
    ///
    /// Set this bit to force power up rsa internal memory
    #[inline(always)]
    pub const fn rsa_mem_force_pu(&self) -> bool {
        (self.bits & RsaPdCtrl::RSA_MEM_FORCE_PU_MASK) != 0
    }

    /// Read **RSA_MEM_FORCE_PD** `[0:0]`
    ///
    /// Set this bit to power down rsa internal memory.
    #[inline(always)]
    pub const fn rsa_mem_force_pd(&self) -> bool {
        (self.bits & RsaPdCtrl::RSA_MEM_FORCE_PD_MASK) != 0
    }

    /// Set **RSA_MEM_PD** `[2:2]`
    ///
    /// Set this bit to force power down rsa internal memory.
    #[inline(always)]
    pub const fn with_rsa_mem_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RsaPdCtrl::RSA_MEM_PD_MASK
            } else {
                self.bits & !RsaPdCtrl::RSA_MEM_PD_MASK
            },
        }
    }

    /// Set **RSA_MEM_FORCE_PU** `[1:1]`
    ///
    /// Set this bit to force power up rsa internal memory
    #[inline(always)]
    pub const fn with_rsa_mem_force_pu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RsaPdCtrl::RSA_MEM_FORCE_PU_MASK
            } else {
                self.bits & !RsaPdCtrl::RSA_MEM_FORCE_PU_MASK
            },
        }
    }

    /// Set **RSA_MEM_FORCE_PD** `[0:0]`
    ///
    /// Set this bit to power down rsa internal memory.
    #[inline(always)]
    pub const fn with_rsa_mem_force_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RsaPdCtrl::RSA_MEM_FORCE_PD_MASK
            } else {
                self.bits & !RsaPdCtrl::RSA_MEM_FORCE_PD_MASK
            },
        }
    }
}

impl Default for RsaPdCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl RsaPdCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RsaPdCtrlVal {
        RsaPdCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RsaPdCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RsaPdCtrlVal) -> RsaPdCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ECC_PD_CTRL Register
///
/// ecc pd ctrl register
///
/// Address offset: 0x01d4
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **ECC_MEM_PD** `[2]` (RW)
///
///   Set this bit to force power down ecc internal memory.
///
/// - **ECC_MEM_FORCE_PU** `[1]` (RW)
///
///   Set this bit to force power up ecc internal memory
///
/// - **ECC_MEM_FORCE_PD** `[0]` (RW)
///
///   Set this bit to power down ecc internal memory.
#[repr(transparent)]
pub struct EccPdCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EccPdCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EccPdCtrl {
    // ======== Field Constants ========

    /// **ECC_MEM_PD**
    ///
    /// Set this bit to force power down ecc internal memory.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const ECC_MEM_PD_OFFSET: u32 = 2;
    pub const ECC_MEM_PD_MASK: u32 = 0x00000004;
    pub const ECC_MEM_PD: u32 = 1 << 2;

    /// **ECC_MEM_FORCE_PU**
    ///
    /// Set this bit to force power up ecc internal memory
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ECC_MEM_FORCE_PU_OFFSET: u32 = 1;
    pub const ECC_MEM_FORCE_PU_MASK: u32 = 0x00000002;
    pub const ECC_MEM_FORCE_PU: u32 = 1 << 1;

    /// **ECC_MEM_FORCE_PD**
    ///
    /// Set this bit to power down ecc internal memory.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ECC_MEM_FORCE_PD_OFFSET: u32 = 0;
    pub const ECC_MEM_FORCE_PD_MASK: u32 = 0x00000001;
    pub const ECC_MEM_FORCE_PD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **ECC_MEM_PD** `[2]` (RW)
    ///
    /// Set this bit to force power down ecc internal memory.
    #[inline(always)]
    pub fn ecc_mem_pd(&self) -> bool {
        (self.value.read_bits() & Self::ECC_MEM_PD_MASK) != 0
    }

    /// Set ECC_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ecc_mem_pd(&self) {
        self.value.modify_bits(|r| r | Self::ECC_MEM_PD_MASK);
    }

    /// Clear ECC_MEM_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ecc_mem_pd(&self) {
        self.value.modify_bits(|r| r & !Self::ECC_MEM_PD_MASK);
    }

    /// **ECC_MEM_FORCE_PU** `[1]` (RW)
    ///
    /// Set this bit to force power up ecc internal memory
    #[inline(always)]
    pub fn ecc_mem_force_pu(&self) -> bool {
        (self.value.read_bits() & Self::ECC_MEM_FORCE_PU_MASK) != 0
    }

    /// Set ECC_MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ecc_mem_force_pu(&self) {
        self.value.modify_bits(|r| r | Self::ECC_MEM_FORCE_PU_MASK);
    }

    /// Clear ECC_MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ecc_mem_force_pu(&self) {
        self.value.modify_bits(|r| r & !Self::ECC_MEM_FORCE_PU_MASK);
    }

    /// **ECC_MEM_FORCE_PD** `[0]` (RW)
    ///
    /// Set this bit to power down ecc internal memory.
    #[inline(always)]
    pub fn ecc_mem_force_pd(&self) -> bool {
        (self.value.read_bits() & Self::ECC_MEM_FORCE_PD_MASK) != 0
    }

    /// Set ECC_MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ecc_mem_force_pd(&self) {
        self.value.modify_bits(|r| r | Self::ECC_MEM_FORCE_PD_MASK);
    }

    /// Clear ECC_MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ecc_mem_force_pd(&self) {
        self.value.modify_bits(|r| r & !Self::ECC_MEM_FORCE_PD_MASK);
    }
}

/// ECC_PD_CTRL value type for batch read/modify/write operations
///
/// Use `eccpdctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `eccpdctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EccPdCtrlVal {
    bits: u32,
}

impl EccPdCtrlVal {
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

    /// Read **ECC_MEM_PD** `[2:2]`
    ///
    /// Set this bit to force power down ecc internal memory.
    #[inline(always)]
    pub const fn ecc_mem_pd(&self) -> bool {
        (self.bits & EccPdCtrl::ECC_MEM_PD_MASK) != 0
    }

    /// Read **ECC_MEM_FORCE_PU** `[1:1]`
    ///
    /// Set this bit to force power up ecc internal memory
    #[inline(always)]
    pub const fn ecc_mem_force_pu(&self) -> bool {
        (self.bits & EccPdCtrl::ECC_MEM_FORCE_PU_MASK) != 0
    }

    /// Read **ECC_MEM_FORCE_PD** `[0:0]`
    ///
    /// Set this bit to power down ecc internal memory.
    #[inline(always)]
    pub const fn ecc_mem_force_pd(&self) -> bool {
        (self.bits & EccPdCtrl::ECC_MEM_FORCE_PD_MASK) != 0
    }

    /// Set **ECC_MEM_PD** `[2:2]`
    ///
    /// Set this bit to force power down ecc internal memory.
    #[inline(always)]
    pub const fn with_ecc_mem_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EccPdCtrl::ECC_MEM_PD_MASK
            } else {
                self.bits & !EccPdCtrl::ECC_MEM_PD_MASK
            },
        }
    }

    /// Set **ECC_MEM_FORCE_PU** `[1:1]`
    ///
    /// Set this bit to force power up ecc internal memory
    #[inline(always)]
    pub const fn with_ecc_mem_force_pu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EccPdCtrl::ECC_MEM_FORCE_PU_MASK
            } else {
                self.bits & !EccPdCtrl::ECC_MEM_FORCE_PU_MASK
            },
        }
    }

    /// Set **ECC_MEM_FORCE_PD** `[0:0]`
    ///
    /// Set this bit to power down ecc internal memory.
    #[inline(always)]
    pub const fn with_ecc_mem_force_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EccPdCtrl::ECC_MEM_FORCE_PD_MASK
            } else {
                self.bits & !EccPdCtrl::ECC_MEM_FORCE_PD_MASK
            },
        }
    }
}

impl Default for EccPdCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EccPdCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EccPdCtrlVal {
        EccPdCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EccPdCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EccPdCtrlVal) -> EccPdCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RNG_CFG Register
///
/// rng cfg register
///
/// Address offset: 0x01d8
///
/// ## Fields
///
/// - **RNG_SAMPLE_CNT** `[31:24]` (RO)
///
///   debug rng sample cnt
///
/// - **RNG_CHAIN_CLK_DIV_NUM** `[23:16]` (RW)
///
///   chain clk div num to pad for debug
///
/// - **RNG_SAMPLE_ENABLE** `[0]` (RW)
///
///   enable rng sample chain
#[repr(transparent)]
pub struct RngCfg {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RngCfg {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RngCfg {
    // ======== Field Constants ========

    /// **RNG_SAMPLE_CNT**
    ///
    /// debug rng sample cnt
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const RNG_SAMPLE_CNT_OFFSET: u32 = 24;
    pub const RNG_SAMPLE_CNT_MASK: u32 = 0xff000000;

    /// **RNG_CHAIN_CLK_DIV_NUM**
    ///
    /// chain clk div num to pad for debug
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const RNG_CHAIN_CLK_DIV_NUM_OFFSET: u32 = 16;
    pub const RNG_CHAIN_CLK_DIV_NUM_MASK: u32 = 0x00ff0000;

    /// **RNG_SAMPLE_ENABLE**
    ///
    /// enable rng sample chain
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RNG_SAMPLE_ENABLE_OFFSET: u32 = 0;
    pub const RNG_SAMPLE_ENABLE_MASK: u32 = 0x00000001;
    pub const RNG_SAMPLE_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RNG_SAMPLE_CNT** `[31:24]` (RO)
    ///
    /// debug rng sample cnt
    #[inline(always)]
    pub fn rng_sample_cnt(&self) -> u32 {
        (self.value.read_bits() & Self::RNG_SAMPLE_CNT_MASK) >> Self::RNG_SAMPLE_CNT_OFFSET
    }

    /// **RNG_CHAIN_CLK_DIV_NUM** `[23:16]` (RW)
    ///
    /// chain clk div num to pad for debug
    #[inline(always)]
    pub fn rng_chain_clk_div_num(&self) -> u32 {
        (self.value.read_bits() & Self::RNG_CHAIN_CLK_DIV_NUM_MASK)
            >> Self::RNG_CHAIN_CLK_DIV_NUM_OFFSET
    }

    /// Write **RNG_CHAIN_CLK_DIV_NUM** `[23:16]` (RW)
    ///
    /// chain clk div num to pad for debug
    #[inline(always)]
    pub fn write_rng_chain_clk_div_num(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RNG_CHAIN_CLK_DIV_NUM_MASK)
                | ((value << Self::RNG_CHAIN_CLK_DIV_NUM_OFFSET) & Self::RNG_CHAIN_CLK_DIV_NUM_MASK)
        });
    }

    /// **RNG_SAMPLE_ENABLE** `[0]` (RW)
    ///
    /// enable rng sample chain
    #[inline(always)]
    pub fn rng_sample_enable(&self) -> bool {
        (self.value.read_bits() & Self::RNG_SAMPLE_ENABLE_MASK) != 0
    }

    /// Set RNG_SAMPLE_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rng_sample_enable(&self) {
        self.value.modify_bits(|r| r | Self::RNG_SAMPLE_ENABLE_MASK);
    }

    /// Clear RNG_SAMPLE_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rng_sample_enable(&self) {
        self.value
            .modify_bits(|r| r & !Self::RNG_SAMPLE_ENABLE_MASK);
    }
}

/// RNG_CFG value type for batch read/modify/write operations
///
/// Use `rngcfg.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rngcfg.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RngCfgVal {
    bits: u32,
}

impl RngCfgVal {
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

    /// Read **RNG_SAMPLE_CNT** `[31:24]`
    ///
    /// debug rng sample cnt
    #[inline(always)]
    pub const fn rng_sample_cnt(&self) -> u32 {
        (self.bits & RngCfg::RNG_SAMPLE_CNT_MASK) >> RngCfg::RNG_SAMPLE_CNT_OFFSET
    }

    /// Read **RNG_CHAIN_CLK_DIV_NUM** `[23:16]`
    ///
    /// chain clk div num to pad for debug
    #[inline(always)]
    pub const fn rng_chain_clk_div_num(&self) -> u32 {
        (self.bits & RngCfg::RNG_CHAIN_CLK_DIV_NUM_MASK) >> RngCfg::RNG_CHAIN_CLK_DIV_NUM_OFFSET
    }

    /// Read **RNG_SAMPLE_ENABLE** `[0:0]`
    ///
    /// enable rng sample chain
    #[inline(always)]
    pub const fn rng_sample_enable(&self) -> bool {
        (self.bits & RngCfg::RNG_SAMPLE_ENABLE_MASK) != 0
    }

    /// Set **RNG_CHAIN_CLK_DIV_NUM** `[23:16]`
    ///
    /// chain clk div num to pad for debug
    #[inline(always)]
    pub const fn with_rng_chain_clk_div_num(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RngCfg::RNG_CHAIN_CLK_DIV_NUM_MASK)
                | ((value << RngCfg::RNG_CHAIN_CLK_DIV_NUM_OFFSET)
                    & RngCfg::RNG_CHAIN_CLK_DIV_NUM_MASK),
        }
    }

    /// Set **RNG_SAMPLE_ENABLE** `[0:0]`
    ///
    /// enable rng sample chain
    #[inline(always)]
    pub const fn with_rng_sample_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RngCfg::RNG_SAMPLE_ENABLE_MASK
            } else {
                self.bits & !RngCfg::RNG_SAMPLE_ENABLE_MASK
            },
        }
    }
}

impl Default for RngCfgVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RngCfg {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RngCfgVal {
        RngCfgVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: RngCfgVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(RngCfgVal) -> RngCfgVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UART_PD_CTRL Register
///
/// ecc pd ctrl register
///
/// Address offset: 0x01dc
/// Reset value: 0x00000002
///
/// ## Fields
///
/// - **UART_MEM_FORCE_PU** `[1]` (RW)
///
///   Set this bit to force power up hp uart internal memory
///
/// - **UART_MEM_FORCE_PD** `[0]` (RW)
///
///   Set this bit to power down hp uart internal memory.
#[repr(transparent)]
pub struct UartPdCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for UartPdCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UartPdCtrl {
    // ======== Field Constants ========

    /// **UART_MEM_FORCE_PU**
    ///
    /// Set this bit to force power up hp uart internal memory
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const UART_MEM_FORCE_PU_OFFSET: u32 = 1;
    pub const UART_MEM_FORCE_PU_MASK: u32 = 0x00000002;
    pub const UART_MEM_FORCE_PU: u32 = 1 << 1;

    /// **UART_MEM_FORCE_PD**
    ///
    /// Set this bit to power down hp uart internal memory.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const UART_MEM_FORCE_PD_OFFSET: u32 = 0;
    pub const UART_MEM_FORCE_PD_MASK: u32 = 0x00000001;
    pub const UART_MEM_FORCE_PD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **UART_MEM_FORCE_PU** `[1]` (RW)
    ///
    /// Set this bit to force power up hp uart internal memory
    #[inline(always)]
    pub fn uart_mem_force_pu(&self) -> bool {
        (self.value.read_bits() & Self::UART_MEM_FORCE_PU_MASK) != 0
    }

    /// Set UART_MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_uart_mem_force_pu(&self) {
        self.value.modify_bits(|r| r | Self::UART_MEM_FORCE_PU_MASK);
    }

    /// Clear UART_MEM_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_uart_mem_force_pu(&self) {
        self.value
            .modify_bits(|r| r & !Self::UART_MEM_FORCE_PU_MASK);
    }

    /// **UART_MEM_FORCE_PD** `[0]` (RW)
    ///
    /// Set this bit to power down hp uart internal memory.
    #[inline(always)]
    pub fn uart_mem_force_pd(&self) -> bool {
        (self.value.read_bits() & Self::UART_MEM_FORCE_PD_MASK) != 0
    }

    /// Set UART_MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_uart_mem_force_pd(&self) {
        self.value.modify_bits(|r| r | Self::UART_MEM_FORCE_PD_MASK);
    }

    /// Clear UART_MEM_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_uart_mem_force_pd(&self) {
        self.value
            .modify_bits(|r| r & !Self::UART_MEM_FORCE_PD_MASK);
    }
}

/// UART_PD_CTRL value type for batch read/modify/write operations
///
/// Use `uartpdctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `uartpdctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UartPdCtrlVal {
    bits: u32,
}

impl UartPdCtrlVal {
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

    /// Read **UART_MEM_FORCE_PU** `[1:1]`
    ///
    /// Set this bit to force power up hp uart internal memory
    #[inline(always)]
    pub const fn uart_mem_force_pu(&self) -> bool {
        (self.bits & UartPdCtrl::UART_MEM_FORCE_PU_MASK) != 0
    }

    /// Read **UART_MEM_FORCE_PD** `[0:0]`
    ///
    /// Set this bit to power down hp uart internal memory.
    #[inline(always)]
    pub const fn uart_mem_force_pd(&self) -> bool {
        (self.bits & UartPdCtrl::UART_MEM_FORCE_PD_MASK) != 0
    }

    /// Set **UART_MEM_FORCE_PU** `[1:1]`
    ///
    /// Set this bit to force power up hp uart internal memory
    #[inline(always)]
    pub const fn with_uart_mem_force_pu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UartPdCtrl::UART_MEM_FORCE_PU_MASK
            } else {
                self.bits & !UartPdCtrl::UART_MEM_FORCE_PU_MASK
            },
        }
    }

    /// Set **UART_MEM_FORCE_PD** `[0:0]`
    ///
    /// Set this bit to power down hp uart internal memory.
    #[inline(always)]
    pub const fn with_uart_mem_force_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UartPdCtrl::UART_MEM_FORCE_PD_MASK
            } else {
                self.bits & !UartPdCtrl::UART_MEM_FORCE_PD_MASK
            },
        }
    }
}

impl Default for UartPdCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl UartPdCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UartPdCtrlVal {
        UartPdCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UartPdCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UartPdCtrlVal) -> UartPdCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PERI_MEM_CLK_FORCE_ON Register
///
/// hp peri mem clk force on regpster
///
/// Address offset: 0x01e0
///
/// ## Fields
///
/// - **GDMA_MEM_CLK_FORCE_ON** `[3]` (RW)
///
///   Set this bit to force on mem clk in gdma
///
/// - **BITSCRAMBLER_RX_MEM_CLK_FORCE_ON** `[2]` (RW)
///
///   Set this bit to force on rx mem clk in bitscrambler
///
/// - **BITSCRAMBLER_TX_MEM_CLK_FORCE_ON** `[1]` (RW)
///
///   Set this bit to force on tx mem clk in bitscrambler
///
/// - **RMT_MEM_CLK_FORCE_ON** `[0]` (RW)
///
///   Set this bit to force on mem clk in rmt
#[repr(transparent)]
pub struct PeriMemClkForceOn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PeriMemClkForceOn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PeriMemClkForceOn {
    // ======== Field Constants ========

    /// **GDMA_MEM_CLK_FORCE_ON**
    ///
    /// Set this bit to force on mem clk in gdma
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const GDMA_MEM_CLK_FORCE_ON_OFFSET: u32 = 3;
    pub const GDMA_MEM_CLK_FORCE_ON_MASK: u32 = 0x00000008;
    pub const GDMA_MEM_CLK_FORCE_ON: u32 = 1 << 3;

    /// **BITSCRAMBLER_RX_MEM_CLK_FORCE_ON**
    ///
    /// Set this bit to force on rx mem clk in bitscrambler
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_OFFSET: u32 = 2;
    pub const BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_MASK: u32 = 0x00000004;
    pub const BITSCRAMBLER_RX_MEM_CLK_FORCE_ON: u32 = 1 << 2;

    /// **BITSCRAMBLER_TX_MEM_CLK_FORCE_ON**
    ///
    /// Set this bit to force on tx mem clk in bitscrambler
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_OFFSET: u32 = 1;
    pub const BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_MASK: u32 = 0x00000002;
    pub const BITSCRAMBLER_TX_MEM_CLK_FORCE_ON: u32 = 1 << 1;

    /// **RMT_MEM_CLK_FORCE_ON**
    ///
    /// Set this bit to force on mem clk in rmt
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RMT_MEM_CLK_FORCE_ON_OFFSET: u32 = 0;
    pub const RMT_MEM_CLK_FORCE_ON_MASK: u32 = 0x00000001;
    pub const RMT_MEM_CLK_FORCE_ON: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **GDMA_MEM_CLK_FORCE_ON** `[3]` (RW)
    ///
    /// Set this bit to force on mem clk in gdma
    #[inline(always)]
    pub fn gdma_mem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::GDMA_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set GDMA_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_gdma_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::GDMA_MEM_CLK_FORCE_ON_MASK);
    }

    /// Clear GDMA_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_gdma_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::GDMA_MEM_CLK_FORCE_ON_MASK);
    }

    /// **BITSCRAMBLER_RX_MEM_CLK_FORCE_ON** `[2]` (RW)
    ///
    /// Set this bit to force on rx mem clk in bitscrambler
    #[inline(always)]
    pub fn bitscrambler_rx_mem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set BITSCRAMBLER_RX_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bitscrambler_rx_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_MASK);
    }

    /// Clear BITSCRAMBLER_RX_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bitscrambler_rx_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_MASK);
    }

    /// **BITSCRAMBLER_TX_MEM_CLK_FORCE_ON** `[1]` (RW)
    ///
    /// Set this bit to force on tx mem clk in bitscrambler
    #[inline(always)]
    pub fn bitscrambler_tx_mem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set BITSCRAMBLER_TX_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_bitscrambler_tx_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_MASK);
    }

    /// Clear BITSCRAMBLER_TX_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_bitscrambler_tx_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_MASK);
    }

    /// **RMT_MEM_CLK_FORCE_ON** `[0]` (RW)
    ///
    /// Set this bit to force on mem clk in rmt
    #[inline(always)]
    pub fn rmt_mem_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::RMT_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set RMT_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rmt_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::RMT_MEM_CLK_FORCE_ON_MASK);
    }

    /// Clear RMT_MEM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rmt_mem_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::RMT_MEM_CLK_FORCE_ON_MASK);
    }
}

/// PERI_MEM_CLK_FORCE_ON value type for batch read/modify/write operations
///
/// Use `perimemclkforceon.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `perimemclkforceon.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PeriMemClkForceOnVal {
    bits: u32,
}

impl PeriMemClkForceOnVal {
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

    /// Read **GDMA_MEM_CLK_FORCE_ON** `[3:3]`
    ///
    /// Set this bit to force on mem clk in gdma
    #[inline(always)]
    pub const fn gdma_mem_clk_force_on(&self) -> bool {
        (self.bits & PeriMemClkForceOn::GDMA_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **BITSCRAMBLER_RX_MEM_CLK_FORCE_ON** `[2:2]`
    ///
    /// Set this bit to force on rx mem clk in bitscrambler
    #[inline(always)]
    pub const fn bitscrambler_rx_mem_clk_force_on(&self) -> bool {
        (self.bits & PeriMemClkForceOn::BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **BITSCRAMBLER_TX_MEM_CLK_FORCE_ON** `[1:1]`
    ///
    /// Set this bit to force on tx mem clk in bitscrambler
    #[inline(always)]
    pub const fn bitscrambler_tx_mem_clk_force_on(&self) -> bool {
        (self.bits & PeriMemClkForceOn::BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **RMT_MEM_CLK_FORCE_ON** `[0:0]`
    ///
    /// Set this bit to force on mem clk in rmt
    #[inline(always)]
    pub const fn rmt_mem_clk_force_on(&self) -> bool {
        (self.bits & PeriMemClkForceOn::RMT_MEM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set **GDMA_MEM_CLK_FORCE_ON** `[3:3]`
    ///
    /// Set this bit to force on mem clk in gdma
    #[inline(always)]
    pub const fn with_gdma_mem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PeriMemClkForceOn::GDMA_MEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !PeriMemClkForceOn::GDMA_MEM_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **BITSCRAMBLER_RX_MEM_CLK_FORCE_ON** `[2:2]`
    ///
    /// Set this bit to force on rx mem clk in bitscrambler
    #[inline(always)]
    pub const fn with_bitscrambler_rx_mem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PeriMemClkForceOn::BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !PeriMemClkForceOn::BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **BITSCRAMBLER_TX_MEM_CLK_FORCE_ON** `[1:1]`
    ///
    /// Set this bit to force on tx mem clk in bitscrambler
    #[inline(always)]
    pub const fn with_bitscrambler_tx_mem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PeriMemClkForceOn::BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !PeriMemClkForceOn::BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **RMT_MEM_CLK_FORCE_ON** `[0:0]`
    ///
    /// Set this bit to force on mem clk in rmt
    #[inline(always)]
    pub const fn with_rmt_mem_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PeriMemClkForceOn::RMT_MEM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !PeriMemClkForceOn::RMT_MEM_CLK_FORCE_ON_MASK
            },
        }
    }
}

impl Default for PeriMemClkForceOnVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PeriMemClkForceOn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PeriMemClkForceOnVal {
        PeriMemClkForceOnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PeriMemClkForceOnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PeriMemClkForceOnVal) -> PeriMemClkForceOnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// NA
    pub ver_date: VerDate,
    /// 0x0004
    /// NA
    pub clk_en: ClkEn,
    _reserved0: [u8; 8],
    /// 0x0010
    /// NA
    pub cpu_intr_from_cpu0: CpuIntrFromCpu0,
    /// 0x0014
    /// NA
    pub cpu_intr_from_cpu1: CpuIntrFromCpu1,
    /// 0x0018
    /// NA
    pub cpu_intr_from_cpu2: CpuIntrFromCpu2,
    /// 0x001c
    /// NA
    pub cpu_intr_from_cpu3: CpuIntrFromCpu3,
    /// 0x0020
    /// NA
    pub cache_clk_config: CacheClkConfig,
    /// 0x0024
    /// NA
    pub cache_reset_config: CacheResetConfig,
    _reserved1: [u8; 4],
    /// 0x002c
    /// NA
    pub dma_addr_ctrl: DmaAddrCtrl,
    _reserved2: [u8; 4],
    /// 0x0034
    /// NA
    pub tcm_ram_wrr_config: TcmRamWrrConfig,
    /// 0x0038
    /// NA
    pub tcm_sw_parity_bwe_mask: TcmSwParityBweMask,
    /// 0x003c
    /// NA
    pub tcm_ram_pwr_ctrl0: TcmRamPwrCtrl0,
    /// 0x0040
    /// NA
    pub l2_rom_pwr_ctrl0: L2RomPwrCtrl0,
    _reserved3: [u8; 12],
    /// 0x0050
    /// NA
    pub probea_ctrl: ProbeaCtrl,
    /// 0x0054
    /// NA
    pub probeb_ctrl: ProbebCtrl,
    _reserved4: [u8; 4],
    /// 0x005c
    /// NA
    pub probe_out: ProbeOut,
    /// 0x0060
    /// NA
    pub l2_mem_ram_pwr_ctrl0: L2MemRamPwrCtrl0,
    /// 0x0064
    /// NA
    pub cpu_corestalled_st: CpuCorestalledSt,
    _reserved5: [u8; 8],
    /// 0x0070
    /// NA
    pub crypto_ctrl: CryptoCtrl,
    /// 0x0074
    /// NA
    pub gpio_o_hold_ctrl0: GpioOHoldCtrl0,
    /// 0x0078
    /// NA
    pub gpio_o_hold_ctrl1: GpioOHoldCtrl1,
    /// 0x007c
    /// NA
    pub rdn_eco_cs: RdnEcoCs,
    /// 0x0080
    /// NA
    pub cache_apb_postw_en: CacheApbPostwEn,
    /// 0x0084
    /// NA
    pub l2_mem_subsize: L2MemSubsize,
    _reserved6: [u8; 20],
    /// 0x009c
    /// NA
    pub l2_mem_int_raw: L2MemIntRaw,
    /// 0x00a0
    /// NA
    pub l2_mem_int_st: L2MemIntSt,
    /// 0x00a4
    /// NA
    pub l2_mem_int_ena: L2MemIntEna,
    /// 0x00a8
    /// NA
    pub l2_mem_int_clr: L2MemIntClr,
    /// 0x00ac
    /// NA
    pub l2_mem_l2_ram_ecc: L2MemL2RamEcc,
    /// 0x00b0
    /// NA
    pub l2_mem_int_record0: L2MemIntRecord0,
    /// 0x00b4
    /// NA
    pub l2_mem_int_record1: L2MemIntRecord1,
    _reserved7: [u8; 12],
    /// 0x00c4
    /// NA
    pub l2_mem_l2_cache_ecc: L2MemL2CacheEcc,
    /// 0x00c8
    /// NA
    pub l1cache_bus0_id: L1cacheBus0Id,
    /// 0x00cc
    /// NA
    pub l1cache_bus1_id: L1cacheBus1Id,
    _reserved8: [u8; 8],
    /// 0x00d8
    /// NA
    pub l2_mem_rdn_eco_cs: L2MemRdnEcoCs,
    /// 0x00dc
    /// NA
    pub l2_mem_rdn_eco_low: L2MemRdnEcoLow,
    /// 0x00e0
    /// NA
    pub l2_mem_rdn_eco_high: L2MemRdnEcoHigh,
    /// 0x00e4
    /// NA
    pub tcm_rdn_eco_cs: TcmRdnEcoCs,
    /// 0x00e8
    /// NA
    pub tcm_rdn_eco_low: TcmRdnEcoLow,
    /// 0x00ec
    /// NA
    pub tcm_rdn_eco_high: TcmRdnEcoHigh,
    /// 0x00f0
    /// NA
    pub gpio_ded_hold_ctrl: GpioDedHoldCtrl,
    /// 0x00f4
    /// NA
    pub l2_mem_sw_ecc_bwe_mask: L2MemSwEccBweMask,
    /// 0x00f8
    /// NA
    pub usb20otg_mem_ctrl: Usb20otgMemCtrl,
    /// 0x00fc
    /// need_des
    pub tcm_int_raw: TcmIntRaw,
    /// 0x0100
    /// need_des
    pub tcm_int_st: TcmIntSt,
    /// 0x0104
    /// need_des
    pub tcm_int_ena: TcmIntEna,
    /// 0x0108
    /// need_des
    pub tcm_int_clr: TcmIntClr,
    /// 0x010c
    /// need_des
    pub tcm_parity_int_record: TcmParityIntRecord,
    /// 0x0110
    /// NA
    pub l1_cache_pwr_ctrl: L1CachePwrCtrl,
    /// 0x0114
    /// NA
    pub l2_cache_pwr_ctrl: L2CachePwrCtrl,
    /// 0x0118
    /// CPU_WAITI configuration register
    pub cpu_waiti_conf: CpuWaitiConf,
    /// 0x011c
    /// Core Debug runstall configure register
    pub core_debug_runstall_conf: CoreDebugRunstallConf,
    /// 0x0120
    /// need_des
    pub core_ahb_timeout: CoreAhbTimeout,
    /// 0x0124
    /// need_des
    pub core_ibus_timeout: CoreIbusTimeout,
    /// 0x0128
    /// need_des
    pub core_dbus_timeout: CoreDbusTimeout,
    _reserved9: [u8; 12],
    /// 0x0138
    /// need_des
    pub icm_cpu_h2x_cfg: IcmCpuH2xCfg,
    /// 0x013c
    /// NA
    pub peri1_apb_postw_en: Peri1ApbPostwEn,
    /// 0x0140
    /// Bitscrambler Peri Sel
    pub bitscrambler_peri_sel: BitscramblerPeriSel,
    /// 0x0144
    /// N/A
    pub apb_sync_postw_en: ApbSyncPostwEn,
    /// 0x0148
    /// N/A
    pub gdma_ctrl: GdmaCtrl,
    /// 0x014c
    /// N/A
    pub gmac_ctrl0: GmacCtrl0,
    /// 0x0150
    /// N/A
    pub gmac_ctrl1: GmacCtrl1,
    /// 0x0154
    /// N/A
    pub gmac_ctrl2: GmacCtrl2,
    /// 0x0158
    /// N/A
    pub vpu_ctrl: VpuCtrl,
    /// 0x015c
    /// N/A
    pub usbotg20_ctrl: Usbotg20Ctrl,
    /// 0x0160
    /// need_des
    pub tcm_err_resp_ctrl: TcmErrRespCtrl,
    /// 0x0164
    /// NA
    pub l2_mem_refresh: L2MemRefresh,
    /// 0x0168
    /// NA
    pub tcm_init: TcmInit,
    /// 0x016c
    /// need_des
    pub tcm_parity_check_ctrl: TcmParityCheckCtrl,
    /// 0x0170
    /// need_des
    pub design_for_verification0: DesignForVerification0,
    /// 0x0174
    /// need_des
    pub design_for_verification1: DesignForVerification1,
    _reserved10: [u8; 8],
    /// 0x0180
    /// need_des
    pub psram_flash_addr_interchange: PsramFlashAddrInterchange,
    _reserved11: [u8; 4],
    /// 0x0188
    /// NA
    pub ahb2axi_bresp_err_int_raw: Ahb2axiBrespErrIntRaw,
    /// 0x018c
    /// need_des
    pub ahb2axi_bresp_err_int_st: Ahb2axiBrespErrIntSt,
    /// 0x0190
    /// need_des
    pub ahb2axi_bresp_err_int_ena: Ahb2axiBrespErrIntEna,
    /// 0x0194
    /// need_des
    pub ahb2axi_bresp_err_int_clr: Ahb2axiBrespErrIntClr,
    /// 0x0198
    /// need_des
    pub l2_mem_err_resp_ctrl: L2MemErrRespCtrl,
    /// 0x019c
    /// need_des
    pub l2_mem_ahb_buffer_ctrl: L2MemAhbBufferCtrl,
    /// 0x01a0
    /// need_des
    pub core_dmactive_lpcore: CoreDmactiveLpcore,
    /// 0x01a4
    /// need_des
    pub core_err_resp_dis: CoreErrRespDis,
    /// 0x01a8
    /// Hp core bus timeout interrupt raw register
    pub core_timeout_int_raw: CoreTimeoutIntRaw,
    /// 0x01ac
    /// masked interrupt register
    pub core_timeout_int_st: CoreTimeoutIntSt,
    /// 0x01b0
    /// masked interrupt register
    pub core_timeout_int_ena: CoreTimeoutIntEna,
    /// 0x01b4
    /// interrupt clear register
    pub core_timeout_int_clr: CoreTimeoutIntClr,
    _reserved12: [u8; 8],
    /// 0x01c0
    /// NA
    pub gpio_o_hys_ctrl0: GpioOHysCtrl0,
    /// 0x01c4
    /// NA
    pub gpio_o_hys_ctrl1: GpioOHysCtrl1,
    _reserved13: [u8; 8],
    /// 0x01d0
    /// rsa pd ctrl register
    pub rsa_pd_ctrl: RsaPdCtrl,
    /// 0x01d4
    /// ecc pd ctrl register
    pub ecc_pd_ctrl: EccPdCtrl,
    /// 0x01d8
    /// rng cfg register
    pub rng_cfg: RngCfg,
    /// 0x01dc
    /// ecc pd ctrl register
    pub uart_pd_ctrl: UartPdCtrl,
    /// 0x01e0
    /// hp peri mem clk force on regpster
    pub peri_mem_clk_force_on: PeriMemClkForceOn,
}
