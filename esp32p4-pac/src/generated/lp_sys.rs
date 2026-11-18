//! LP_SYS
//! LP_SYS Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x50110000;

/// LP_SYS_VER_DATE Register
///
/// need_des
///
/// Address offset: 0x0000
/// Reset value: 0x20230509
///
/// ## Fields
///
/// - **VER_DATE** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpSysVerDate {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpSysVerDate {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpSysVerDate {
    // ======== Field Constants ========

    /// **VER_DATE**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const VER_DATE_OFFSET: u32 = 0;
    pub const VER_DATE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **VER_DATE** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn ver_date(&self) -> u32 {
        (self.value.read_bits() & Self::VER_DATE_MASK) >> Self::VER_DATE_OFFSET
    }

    /// Write **VER_DATE** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_ver_date(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VER_DATE_MASK) | ((value << Self::VER_DATE_OFFSET) & Self::VER_DATE_MASK)
        });
    }
}

/// LP_SYS_VER_DATE value type for batch read/modify/write operations
///
/// Use `lpsysverdate.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpsysverdate.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpSysVerDateVal {
    bits: u32,
}

impl LpSysVerDateVal {
    /// Create from raw bits
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
        Self { bits: 0x20230509 }
    }

    /// Read **VER_DATE** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ver_date(&self) -> u32 {
        (self.bits & LpSysVerDate::VER_DATE_MASK) >> LpSysVerDate::VER_DATE_OFFSET
    }

    /// Set **VER_DATE** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_ver_date(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpSysVerDate::VER_DATE_MASK)
                | ((value << LpSysVerDate::VER_DATE_OFFSET) & LpSysVerDate::VER_DATE_MASK),
        }
    }
}

impl Default for LpSysVerDateVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpSysVerDate {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpSysVerDateVal {
        LpSysVerDateVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpSysVerDateVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpSysVerDateVal) -> LpSysVerDateVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// CLK_SEL_CTRL Register
///
/// need_des
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **SW_SYS_CLK_SRC_SEL** `[17]` (RW)
///
///   reserved
///
/// - **ENA_SW_SEL_SYS_CLK** `[16]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct ClkSelCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ClkSelCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ClkSelCtrl {
    // ======== Field Constants ========

    /// **SW_SYS_CLK_SRC_SEL**
    ///
    /// reserved
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const SW_SYS_CLK_SRC_SEL_OFFSET: u32 = 17;
    pub const SW_SYS_CLK_SRC_SEL_MASK: u32 = 0x00020000;
    pub const SW_SYS_CLK_SRC_SEL: u32 = 1 << 17;

    /// **ENA_SW_SEL_SYS_CLK**
    ///
    /// reserved
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ENA_SW_SEL_SYS_CLK_OFFSET: u32 = 16;
    pub const ENA_SW_SEL_SYS_CLK_MASK: u32 = 0x00010000;
    pub const ENA_SW_SEL_SYS_CLK: u32 = 1 << 16;

    // ======== Field Accessors ========

    /// **SW_SYS_CLK_SRC_SEL** `[17]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn sw_sys_clk_src_sel(&self) -> bool {
        (self.value.read_bits() & Self::SW_SYS_CLK_SRC_SEL_MASK) != 0
    }

    /// Set SW_SYS_CLK_SRC_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sw_sys_clk_src_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::SW_SYS_CLK_SRC_SEL_MASK);
    }

    /// Clear SW_SYS_CLK_SRC_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sw_sys_clk_src_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::SW_SYS_CLK_SRC_SEL_MASK);
    }

    /// **ENA_SW_SEL_SYS_CLK** `[16]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn ena_sw_sel_sys_clk(&self) -> bool {
        (self.value.read_bits() & Self::ENA_SW_SEL_SYS_CLK_MASK) != 0
    }

    /// Set ENA_SW_SEL_SYS_CLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ena_sw_sel_sys_clk(&self) {
        self.value
            .modify_bits(|r| r | Self::ENA_SW_SEL_SYS_CLK_MASK);
    }

    /// Clear ENA_SW_SEL_SYS_CLK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ena_sw_sel_sys_clk(&self) {
        self.value
            .modify_bits(|r| r & !Self::ENA_SW_SEL_SYS_CLK_MASK);
    }
}

/// CLK_SEL_CTRL value type for batch read/modify/write operations
///
/// Use `clkselctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `clkselctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ClkSelCtrlVal {
    bits: u32,
}

impl ClkSelCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SW_SYS_CLK_SRC_SEL** `[17:17]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn sw_sys_clk_src_sel(&self) -> bool {
        (self.bits & ClkSelCtrl::SW_SYS_CLK_SRC_SEL_MASK) != 0
    }

    /// Read **ENA_SW_SEL_SYS_CLK** `[16:16]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn ena_sw_sel_sys_clk(&self) -> bool {
        (self.bits & ClkSelCtrl::ENA_SW_SEL_SYS_CLK_MASK) != 0
    }

    /// Set **SW_SYS_CLK_SRC_SEL** `[17:17]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_sw_sys_clk_src_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkSelCtrl::SW_SYS_CLK_SRC_SEL_MASK
            } else {
                self.bits & !ClkSelCtrl::SW_SYS_CLK_SRC_SEL_MASK
            },
        }
    }

    /// Set **ENA_SW_SEL_SYS_CLK** `[16:16]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_ena_sw_sel_sys_clk(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ClkSelCtrl::ENA_SW_SEL_SYS_CLK_MASK
            } else {
                self.bits & !ClkSelCtrl::ENA_SW_SEL_SYS_CLK_MASK
            },
        }
    }
}

impl Default for ClkSelCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ClkSelCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ClkSelCtrlVal {
        ClkSelCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ClkSelCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ClkSelCtrlVal) -> ClkSelCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// SYS_CTRL Register
///
/// need_des
///
/// Address offset: 0x0008
/// Reset value: 0x1fffc7f8
///
/// ## Fields
///
/// - **SYSTIMER_STALL_SEL** `[31]` (RW)
///
///   0: use systimer_stall signal from hp_core0, 1: use systimer_stall signal from hp_core1
///
/// - **LP_CORE_ETM_WAKEUP_FLAG** `[30]` (RW)
///
///   need_des
///
/// - **LP_CORE_ETM_WAKEUP_FLAG_CLR** `[29]` (WO)
///
///   need_des
///
/// - **LP_FIB_SEL** `[28:21]` (RW)
///
///   need_des
///
/// - **ANA_FIB** `[20:14]` (RO)
///
///   need_des
///
/// - **IO_MUX_RESET_DISABLE** `[11]` (RW)
///
///   reset disable bit for LP IOMUX
///
/// - **DIG_FIB** `[10:3]` (RW)
///
///   need_des
///
/// - **FORCE_DOWNLOAD_BOOT** `[2]` (RW)
///
///   need_des
///
/// - **SYS_SW_RST** `[1]` (WO)
///
///   digital system software reset bit
///
/// - **LP_CORE_DISABLE** `[0]` (RW)
///
///   lp cpu disable
#[repr(transparent)]
pub struct SysCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for SysCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl SysCtrl {
    // ======== Field Constants ========

    /// **SYSTIMER_STALL_SEL**
    ///
    /// 0: use systimer_stall signal from hp_core0, 1: use systimer_stall signal from hp_core1
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const SYSTIMER_STALL_SEL_OFFSET: u32 = 31;
    pub const SYSTIMER_STALL_SEL_MASK: u32 = 0x80000000;
    pub const SYSTIMER_STALL_SEL: u32 = 1 << 31;

    /// **LP_CORE_ETM_WAKEUP_FLAG**
    ///
    /// need_des
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const LP_CORE_ETM_WAKEUP_FLAG_OFFSET: u32 = 30;
    pub const LP_CORE_ETM_WAKEUP_FLAG_MASK: u32 = 0x40000000;
    pub const LP_CORE_ETM_WAKEUP_FLAG: u32 = 1 << 30;

    /// **LP_CORE_ETM_WAKEUP_FLAG_CLR**
    ///
    /// need_des
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const LP_CORE_ETM_WAKEUP_FLAG_CLR_OFFSET: u32 = 29;
    pub const LP_CORE_ETM_WAKEUP_FLAG_CLR_MASK: u32 = 0x20000000;
    pub const LP_CORE_ETM_WAKEUP_FLAG_CLR: u32 = 1 << 29;

    /// **LP_FIB_SEL**
    ///
    /// need_des
    ///
    /// Bits `[28:21]`, mask `0x1fe00000`
    pub const LP_FIB_SEL_OFFSET: u32 = 21;
    pub const LP_FIB_SEL_MASK: u32 = 0x1fe00000;

    /// **ANA_FIB**
    ///
    /// need_des
    ///
    /// Bits `[20:14]`, mask `0x001fc000`
    pub const ANA_FIB_OFFSET: u32 = 14;
    pub const ANA_FIB_MASK: u32 = 0x001fc000;

    /// **IO_MUX_RESET_DISABLE**
    ///
    /// reset disable bit for LP IOMUX
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const IO_MUX_RESET_DISABLE_OFFSET: u32 = 11;
    pub const IO_MUX_RESET_DISABLE_MASK: u32 = 0x00000800;
    pub const IO_MUX_RESET_DISABLE: u32 = 1 << 11;

    /// **DIG_FIB**
    ///
    /// need_des
    ///
    /// Bits `[10:3]`, mask `0x000007f8`
    pub const DIG_FIB_OFFSET: u32 = 3;
    pub const DIG_FIB_MASK: u32 = 0x000007f8;

    /// **FORCE_DOWNLOAD_BOOT**
    ///
    /// need_des
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const FORCE_DOWNLOAD_BOOT_OFFSET: u32 = 2;
    pub const FORCE_DOWNLOAD_BOOT_MASK: u32 = 0x00000004;
    pub const FORCE_DOWNLOAD_BOOT: u32 = 1 << 2;

    /// **SYS_SW_RST**
    ///
    /// digital system software reset bit
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SYS_SW_RST_OFFSET: u32 = 1;
    pub const SYS_SW_RST_MASK: u32 = 0x00000002;
    pub const SYS_SW_RST: u32 = 1 << 1;

    /// **LP_CORE_DISABLE**
    ///
    /// lp cpu disable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_CORE_DISABLE_OFFSET: u32 = 0;
    pub const LP_CORE_DISABLE_MASK: u32 = 0x00000001;
    pub const LP_CORE_DISABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SYSTIMER_STALL_SEL** `[31]` (RW)
    ///
    /// 0: use systimer_stall signal from hp_core0, 1: use systimer_stall signal from hp_core1
    #[inline(always)]
    pub fn systimer_stall_sel(&self) -> bool {
        (self.value.read_bits() & Self::SYSTIMER_STALL_SEL_MASK) != 0
    }

    /// Set SYSTIMER_STALL_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_systimer_stall_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::SYSTIMER_STALL_SEL_MASK);
    }

    /// Clear SYSTIMER_STALL_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_systimer_stall_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::SYSTIMER_STALL_SEL_MASK);
    }

    /// **LP_CORE_ETM_WAKEUP_FLAG** `[30]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_core_etm_wakeup_flag(&self) -> bool {
        (self.value.read_bits() & Self::LP_CORE_ETM_WAKEUP_FLAG_MASK) != 0
    }

    /// Set LP_CORE_ETM_WAKEUP_FLAG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_core_etm_wakeup_flag(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_CORE_ETM_WAKEUP_FLAG_MASK);
    }

    /// Clear LP_CORE_ETM_WAKEUP_FLAG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_core_etm_wakeup_flag(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_CORE_ETM_WAKEUP_FLAG_MASK);
    }

    /// Trigger **LP_CORE_ETM_WAKEUP_FLAG_CLR** (WO)
    ///
    /// need_des
    #[inline(always)]
    pub fn trigger_lp_core_etm_wakeup_flag_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_CORE_ETM_WAKEUP_FLAG_CLR_MASK);
    }

    /// **LP_FIB_SEL** `[28:21]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_fib_sel(&self) -> u32 {
        (self.value.read_bits() & Self::LP_FIB_SEL_MASK) >> Self::LP_FIB_SEL_OFFSET
    }

    /// Write **LP_FIB_SEL** `[28:21]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_fib_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_FIB_SEL_MASK)
                | ((value << Self::LP_FIB_SEL_OFFSET) & Self::LP_FIB_SEL_MASK)
        });
    }

    /// **ANA_FIB** `[20:14]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn ana_fib(&self) -> u32 {
        (self.value.read_bits() & Self::ANA_FIB_MASK) >> Self::ANA_FIB_OFFSET
    }

    /// **IO_MUX_RESET_DISABLE** `[11]` (RW)
    ///
    /// reset disable bit for LP IOMUX
    #[inline(always)]
    pub fn io_mux_reset_disable(&self) -> bool {
        (self.value.read_bits() & Self::IO_MUX_RESET_DISABLE_MASK) != 0
    }

    /// Set IO_MUX_RESET_DISABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_io_mux_reset_disable(&self) {
        self.value
            .modify_bits(|r| r | Self::IO_MUX_RESET_DISABLE_MASK);
    }

    /// Clear IO_MUX_RESET_DISABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_io_mux_reset_disable(&self) {
        self.value
            .modify_bits(|r| r & !Self::IO_MUX_RESET_DISABLE_MASK);
    }

    /// **DIG_FIB** `[10:3]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn dig_fib(&self) -> u32 {
        (self.value.read_bits() & Self::DIG_FIB_MASK) >> Self::DIG_FIB_OFFSET
    }

    /// Write **DIG_FIB** `[10:3]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_dig_fib(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DIG_FIB_MASK) | ((value << Self::DIG_FIB_OFFSET) & Self::DIG_FIB_MASK)
        });
    }

    /// **FORCE_DOWNLOAD_BOOT** `[2]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn force_download_boot(&self) -> bool {
        (self.value.read_bits() & Self::FORCE_DOWNLOAD_BOOT_MASK) != 0
    }

    /// Set FORCE_DOWNLOAD_BOOT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_force_download_boot(&self) {
        self.value
            .modify_bits(|r| r | Self::FORCE_DOWNLOAD_BOOT_MASK);
    }

    /// Clear FORCE_DOWNLOAD_BOOT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_force_download_boot(&self) {
        self.value
            .modify_bits(|r| r & !Self::FORCE_DOWNLOAD_BOOT_MASK);
    }

    /// Trigger **SYS_SW_RST** (WO)
    ///
    /// digital system software reset bit
    #[inline(always)]
    pub fn trigger_sys_sw_rst(&self) {
        self.value.modify_bits(|r| r | Self::SYS_SW_RST_MASK);
    }

    /// **LP_CORE_DISABLE** `[0]` (RW)
    ///
    /// lp cpu disable
    #[inline(always)]
    pub fn lp_core_disable(&self) -> bool {
        (self.value.read_bits() & Self::LP_CORE_DISABLE_MASK) != 0
    }

    /// Set LP_CORE_DISABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_core_disable(&self) {
        self.value.modify_bits(|r| r | Self::LP_CORE_DISABLE_MASK);
    }

    /// Clear LP_CORE_DISABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_core_disable(&self) {
        self.value.modify_bits(|r| r & !Self::LP_CORE_DISABLE_MASK);
    }
}

/// SYS_CTRL value type for batch read/modify/write operations
///
/// Use `sysctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `sysctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct SysCtrlVal {
    bits: u32,
}

impl SysCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x1fffc7f8 }
    }

    /// Read **SYSTIMER_STALL_SEL** `[31:31]`
    ///
    /// 0: use systimer_stall signal from hp_core0, 1: use systimer_stall signal from hp_core1
    #[inline(always)]
    pub const fn systimer_stall_sel(&self) -> bool {
        (self.bits & SysCtrl::SYSTIMER_STALL_SEL_MASK) != 0
    }

    /// Read **LP_CORE_ETM_WAKEUP_FLAG** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_core_etm_wakeup_flag(&self) -> bool {
        (self.bits & SysCtrl::LP_CORE_ETM_WAKEUP_FLAG_MASK) != 0
    }

    /// Read **LP_FIB_SEL** `[28:21]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_fib_sel(&self) -> u32 {
        (self.bits & SysCtrl::LP_FIB_SEL_MASK) >> SysCtrl::LP_FIB_SEL_OFFSET
    }

    /// Read **ANA_FIB** `[20:14]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn ana_fib(&self) -> u32 {
        (self.bits & SysCtrl::ANA_FIB_MASK) >> SysCtrl::ANA_FIB_OFFSET
    }

    /// Read **IO_MUX_RESET_DISABLE** `[11:11]`
    ///
    /// reset disable bit for LP IOMUX
    #[inline(always)]
    pub const fn io_mux_reset_disable(&self) -> bool {
        (self.bits & SysCtrl::IO_MUX_RESET_DISABLE_MASK) != 0
    }

    /// Read **DIG_FIB** `[10:3]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn dig_fib(&self) -> u32 {
        (self.bits & SysCtrl::DIG_FIB_MASK) >> SysCtrl::DIG_FIB_OFFSET
    }

    /// Read **FORCE_DOWNLOAD_BOOT** `[2:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn force_download_boot(&self) -> bool {
        (self.bits & SysCtrl::FORCE_DOWNLOAD_BOOT_MASK) != 0
    }

    /// Read **LP_CORE_DISABLE** `[0:0]`
    ///
    /// lp cpu disable
    #[inline(always)]
    pub const fn lp_core_disable(&self) -> bool {
        (self.bits & SysCtrl::LP_CORE_DISABLE_MASK) != 0
    }

    /// Set **SYSTIMER_STALL_SEL** `[31:31]`
    ///
    /// 0: use systimer_stall signal from hp_core0, 1: use systimer_stall signal from hp_core1
    #[inline(always)]
    pub const fn with_systimer_stall_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysCtrl::SYSTIMER_STALL_SEL_MASK
            } else {
                self.bits & !SysCtrl::SYSTIMER_STALL_SEL_MASK
            },
        }
    }

    /// Set **LP_CORE_ETM_WAKEUP_FLAG** `[30:30]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_core_etm_wakeup_flag(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysCtrl::LP_CORE_ETM_WAKEUP_FLAG_MASK
            } else {
                self.bits & !SysCtrl::LP_CORE_ETM_WAKEUP_FLAG_MASK
            },
        }
    }

    /// Set **LP_CORE_ETM_WAKEUP_FLAG_CLR** `[29:29]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_core_etm_wakeup_flag_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysCtrl::LP_CORE_ETM_WAKEUP_FLAG_CLR_MASK
            } else {
                self.bits & !SysCtrl::LP_CORE_ETM_WAKEUP_FLAG_CLR_MASK
            },
        }
    }

    /// Set **LP_FIB_SEL** `[28:21]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_fib_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SysCtrl::LP_FIB_SEL_MASK)
                | ((value << SysCtrl::LP_FIB_SEL_OFFSET) & SysCtrl::LP_FIB_SEL_MASK),
        }
    }

    /// Set **IO_MUX_RESET_DISABLE** `[11:11]`
    ///
    /// reset disable bit for LP IOMUX
    #[inline(always)]
    pub const fn with_io_mux_reset_disable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysCtrl::IO_MUX_RESET_DISABLE_MASK
            } else {
                self.bits & !SysCtrl::IO_MUX_RESET_DISABLE_MASK
            },
        }
    }

    /// Set **DIG_FIB** `[10:3]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_dig_fib(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !SysCtrl::DIG_FIB_MASK)
                | ((value << SysCtrl::DIG_FIB_OFFSET) & SysCtrl::DIG_FIB_MASK),
        }
    }

    /// Set **FORCE_DOWNLOAD_BOOT** `[2:2]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_force_download_boot(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysCtrl::FORCE_DOWNLOAD_BOOT_MASK
            } else {
                self.bits & !SysCtrl::FORCE_DOWNLOAD_BOOT_MASK
            },
        }
    }

    /// Set **SYS_SW_RST** `[1:1]`
    ///
    /// digital system software reset bit
    #[inline(always)]
    pub const fn with_sys_sw_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysCtrl::SYS_SW_RST_MASK
            } else {
                self.bits & !SysCtrl::SYS_SW_RST_MASK
            },
        }
    }

    /// Set **LP_CORE_DISABLE** `[0:0]`
    ///
    /// lp cpu disable
    #[inline(always)]
    pub const fn with_lp_core_disable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | SysCtrl::LP_CORE_DISABLE_MASK
            } else {
                self.bits & !SysCtrl::LP_CORE_DISABLE_MASK
            },
        }
    }
}

impl Default for SysCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl SysCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> SysCtrlVal {
        SysCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: SysCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(SysCtrlVal) -> SysCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_CLK_CTRL Register
///
/// need_des
///
/// Address offset: 0x000c
/// Reset value: 0x00004001
///
/// ## Fields
///
/// - **LP_FOSC_HP_CKEN** `[14]` (RW)
///
///   reserved
///
/// - **CLK_EN** `[0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpClkCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpClkCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpClkCtrl {
    // ======== Field Constants ========

    /// **LP_FOSC_HP_CKEN**
    ///
    /// reserved
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const LP_FOSC_HP_CKEN_OFFSET: u32 = 14;
    pub const LP_FOSC_HP_CKEN_MASK: u32 = 0x00004000;
    pub const LP_FOSC_HP_CKEN: u32 = 1 << 14;

    /// **CLK_EN**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CLK_EN_OFFSET: u32 = 0;
    pub const CLK_EN_MASK: u32 = 0x00000001;
    pub const CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LP_FOSC_HP_CKEN** `[14]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn lp_fosc_hp_cken(&self) -> bool {
        (self.value.read_bits() & Self::LP_FOSC_HP_CKEN_MASK) != 0
    }

    /// Set LP_FOSC_HP_CKEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_fosc_hp_cken(&self) {
        self.value.modify_bits(|r| r | Self::LP_FOSC_HP_CKEN_MASK);
    }

    /// Clear LP_FOSC_HP_CKEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_fosc_hp_cken(&self) {
        self.value.modify_bits(|r| r & !Self::LP_FOSC_HP_CKEN_MASK);
    }

    /// **CLK_EN** `[0]` (RW)
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
}

/// LP_CLK_CTRL value type for batch read/modify/write operations
///
/// Use `lpclkctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpclkctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpClkCtrlVal {
    bits: u32,
}

impl LpClkCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00004001 }
    }

    /// Read **LP_FOSC_HP_CKEN** `[14:14]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn lp_fosc_hp_cken(&self) -> bool {
        (self.bits & LpClkCtrl::LP_FOSC_HP_CKEN_MASK) != 0
    }

    /// Read **CLK_EN** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & LpClkCtrl::CLK_EN_MASK) != 0
    }

    /// Set **LP_FOSC_HP_CKEN** `[14:14]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_lp_fosc_hp_cken(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpClkCtrl::LP_FOSC_HP_CKEN_MASK
            } else {
                self.bits & !LpClkCtrl::LP_FOSC_HP_CKEN_MASK
            },
        }
    }

    /// Set **CLK_EN** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpClkCtrl::CLK_EN_MASK
            } else {
                self.bits & !LpClkCtrl::CLK_EN_MASK
            },
        }
    }
}

impl Default for LpClkCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpClkCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpClkCtrlVal {
        LpClkCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpClkCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpClkCtrlVal) -> LpClkCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_RST_CTRL Register
///
/// need_des
///
/// Address offset: 0x0010
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **EFUSE_FORCE_NORST** `[2]` (RW)
///
///   efuse force no reset control
///
/// - **SYS_RST_BYPASS** `[1]` (RW)
///
///   system source reset bypass : software reset,hp wdt,lp wdt,efuse
///
/// - **ANA_RST_BYPASS** `[0]` (RW)
///
///   analog source reset bypass : wdt,brown out,super wdt,glitch
#[repr(transparent)]
pub struct LpRstCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpRstCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpRstCtrl {
    // ======== Field Constants ========

    /// **EFUSE_FORCE_NORST**
    ///
    /// efuse force no reset control
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const EFUSE_FORCE_NORST_OFFSET: u32 = 2;
    pub const EFUSE_FORCE_NORST_MASK: u32 = 0x00000004;
    pub const EFUSE_FORCE_NORST: u32 = 1 << 2;

    /// **SYS_RST_BYPASS**
    ///
    /// system source reset bypass : software reset,hp wdt,lp wdt,efuse
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SYS_RST_BYPASS_OFFSET: u32 = 1;
    pub const SYS_RST_BYPASS_MASK: u32 = 0x00000002;
    pub const SYS_RST_BYPASS: u32 = 1 << 1;

    /// **ANA_RST_BYPASS**
    ///
    /// analog source reset bypass : wdt,brown out,super wdt,glitch
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const ANA_RST_BYPASS_OFFSET: u32 = 0;
    pub const ANA_RST_BYPASS_MASK: u32 = 0x00000001;
    pub const ANA_RST_BYPASS: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EFUSE_FORCE_NORST** `[2]` (RW)
    ///
    /// efuse force no reset control
    #[inline(always)]
    pub fn efuse_force_norst(&self) -> bool {
        (self.value.read_bits() & Self::EFUSE_FORCE_NORST_MASK) != 0
    }

    /// Set EFUSE_FORCE_NORST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_efuse_force_norst(&self) {
        self.value.modify_bits(|r| r | Self::EFUSE_FORCE_NORST_MASK);
    }

    /// Clear EFUSE_FORCE_NORST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_efuse_force_norst(&self) {
        self.value
            .modify_bits(|r| r & !Self::EFUSE_FORCE_NORST_MASK);
    }

    /// **SYS_RST_BYPASS** `[1]` (RW)
    ///
    /// system source reset bypass : software reset,hp wdt,lp wdt,efuse
    #[inline(always)]
    pub fn sys_rst_bypass(&self) -> bool {
        (self.value.read_bits() & Self::SYS_RST_BYPASS_MASK) != 0
    }

    /// Set SYS_RST_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sys_rst_bypass(&self) {
        self.value.modify_bits(|r| r | Self::SYS_RST_BYPASS_MASK);
    }

    /// Clear SYS_RST_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sys_rst_bypass(&self) {
        self.value.modify_bits(|r| r & !Self::SYS_RST_BYPASS_MASK);
    }

    /// **ANA_RST_BYPASS** `[0]` (RW)
    ///
    /// analog source reset bypass : wdt,brown out,super wdt,glitch
    #[inline(always)]
    pub fn ana_rst_bypass(&self) -> bool {
        (self.value.read_bits() & Self::ANA_RST_BYPASS_MASK) != 0
    }

    /// Set ANA_RST_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ana_rst_bypass(&self) {
        self.value.modify_bits(|r| r | Self::ANA_RST_BYPASS_MASK);
    }

    /// Clear ANA_RST_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ana_rst_bypass(&self) {
        self.value.modify_bits(|r| r & !Self::ANA_RST_BYPASS_MASK);
    }
}

/// LP_RST_CTRL value type for batch read/modify/write operations
///
/// Use `lprstctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lprstctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpRstCtrlVal {
    bits: u32,
}

impl LpRstCtrlVal {
    /// Create from raw bits
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

    /// Read **EFUSE_FORCE_NORST** `[2:2]`
    ///
    /// efuse force no reset control
    #[inline(always)]
    pub const fn efuse_force_norst(&self) -> bool {
        (self.bits & LpRstCtrl::EFUSE_FORCE_NORST_MASK) != 0
    }

    /// Read **SYS_RST_BYPASS** `[1:1]`
    ///
    /// system source reset bypass : software reset,hp wdt,lp wdt,efuse
    #[inline(always)]
    pub const fn sys_rst_bypass(&self) -> bool {
        (self.bits & LpRstCtrl::SYS_RST_BYPASS_MASK) != 0
    }

    /// Read **ANA_RST_BYPASS** `[0:0]`
    ///
    /// analog source reset bypass : wdt,brown out,super wdt,glitch
    #[inline(always)]
    pub const fn ana_rst_bypass(&self) -> bool {
        (self.bits & LpRstCtrl::ANA_RST_BYPASS_MASK) != 0
    }

    /// Set **EFUSE_FORCE_NORST** `[2:2]`
    ///
    /// efuse force no reset control
    #[inline(always)]
    pub const fn with_efuse_force_norst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpRstCtrl::EFUSE_FORCE_NORST_MASK
            } else {
                self.bits & !LpRstCtrl::EFUSE_FORCE_NORST_MASK
            },
        }
    }

    /// Set **SYS_RST_BYPASS** `[1:1]`
    ///
    /// system source reset bypass : software reset,hp wdt,lp wdt,efuse
    #[inline(always)]
    pub const fn with_sys_rst_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpRstCtrl::SYS_RST_BYPASS_MASK
            } else {
                self.bits & !LpRstCtrl::SYS_RST_BYPASS_MASK
            },
        }
    }

    /// Set **ANA_RST_BYPASS** `[0:0]`
    ///
    /// analog source reset bypass : wdt,brown out,super wdt,glitch
    #[inline(always)]
    pub const fn with_ana_rst_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpRstCtrl::ANA_RST_BYPASS_MASK
            } else {
                self.bits & !LpRstCtrl::ANA_RST_BYPASS_MASK
            },
        }
    }
}

impl Default for LpRstCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpRstCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpRstCtrlVal {
        LpRstCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpRstCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpRstCtrlVal) -> LpRstCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_CORE_BOOT_ADDR Register
///
/// need_des
///
/// Address offset: 0x0018
/// Reset value: 0x50100000
///
/// ## Fields
///
/// - **LP_CPU_BOOT_ADDR** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpCoreBootAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpCoreBootAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpCoreBootAddr {
    // ======== Field Constants ========

    /// **LP_CPU_BOOT_ADDR**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_CPU_BOOT_ADDR_OFFSET: u32 = 0;
    pub const LP_CPU_BOOT_ADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_CPU_BOOT_ADDR** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_cpu_boot_addr(&self) -> u32 {
        (self.value.read_bits() & Self::LP_CPU_BOOT_ADDR_MASK) >> Self::LP_CPU_BOOT_ADDR_OFFSET
    }

    /// Write **LP_CPU_BOOT_ADDR** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_cpu_boot_addr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_CPU_BOOT_ADDR_MASK)
                | ((value << Self::LP_CPU_BOOT_ADDR_OFFSET) & Self::LP_CPU_BOOT_ADDR_MASK)
        });
    }
}

/// LP_CORE_BOOT_ADDR value type for batch read/modify/write operations
///
/// Use `lpcorebootaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpcorebootaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpCoreBootAddrVal {
    bits: u32,
}

impl LpCoreBootAddrVal {
    /// Create from raw bits
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
        Self { bits: 0x50100000 }
    }

    /// Read **LP_CPU_BOOT_ADDR** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_cpu_boot_addr(&self) -> u32 {
        (self.bits & LpCoreBootAddr::LP_CPU_BOOT_ADDR_MASK)
            >> LpCoreBootAddr::LP_CPU_BOOT_ADDR_OFFSET
    }

    /// Set **LP_CPU_BOOT_ADDR** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_cpu_boot_addr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpCoreBootAddr::LP_CPU_BOOT_ADDR_MASK)
                | ((value << LpCoreBootAddr::LP_CPU_BOOT_ADDR_OFFSET)
                    & LpCoreBootAddr::LP_CPU_BOOT_ADDR_MASK),
        }
    }
}

impl Default for LpCoreBootAddrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpCoreBootAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpCoreBootAddrVal {
        LpCoreBootAddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpCoreBootAddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpCoreBootAddrVal) -> LpCoreBootAddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EXT_WAKEUP1 Register
///
/// need_des
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **STATUS_CLR** `[16]` (WO)
///
///   clear ext wakeup1 status
///
/// - **SEL** `[15:0]` (RW)
///
///   Bitmap to select RTC pads for ext wakeup1
#[repr(transparent)]
pub struct ExtWakeup1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ExtWakeup1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ExtWakeup1 {
    // ======== Field Constants ========

    /// **STATUS_CLR**
    ///
    /// clear ext wakeup1 status
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const STATUS_CLR_OFFSET: u32 = 16;
    pub const STATUS_CLR_MASK: u32 = 0x00010000;
    pub const STATUS_CLR: u32 = 1 << 16;

    /// **SEL**
    ///
    /// Bitmap to select RTC pads for ext wakeup1
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const SEL_OFFSET: u32 = 0;
    pub const SEL_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// Trigger **STATUS_CLR** (WO)
    ///
    /// clear ext wakeup1 status
    #[inline(always)]
    pub fn trigger_status_clr(&self) {
        self.value.modify_bits(|r| r | Self::STATUS_CLR_MASK);
    }

    /// **SEL** `[15:0]` (RW)
    ///
    /// Bitmap to select RTC pads for ext wakeup1
    #[inline(always)]
    pub fn sel(&self) -> u32 {
        (self.value.read_bits() & Self::SEL_MASK) >> Self::SEL_OFFSET
    }

    /// Write **SEL** `[15:0]` (RW)
    ///
    /// Bitmap to select RTC pads for ext wakeup1
    #[inline(always)]
    pub fn write_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SEL_MASK) | ((value << Self::SEL_OFFSET) & Self::SEL_MASK)
        });
    }
}

/// EXT_WAKEUP1 value type for batch read/modify/write operations
///
/// Use `extwakeup1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `extwakeup1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ExtWakeup1Val {
    bits: u32,
}

impl ExtWakeup1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **SEL** `[15:0]`
    ///
    /// Bitmap to select RTC pads for ext wakeup1
    #[inline(always)]
    pub const fn sel(&self) -> u32 {
        (self.bits & ExtWakeup1::SEL_MASK) >> ExtWakeup1::SEL_OFFSET
    }

    /// Set **STATUS_CLR** `[16:16]`
    ///
    /// clear ext wakeup1 status
    #[inline(always)]
    pub const fn with_status_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | ExtWakeup1::STATUS_CLR_MASK
            } else {
                self.bits & !ExtWakeup1::STATUS_CLR_MASK
            },
        }
    }

    /// Set **SEL** `[15:0]`
    ///
    /// Bitmap to select RTC pads for ext wakeup1
    #[inline(always)]
    pub const fn with_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !ExtWakeup1::SEL_MASK)
                | ((value << ExtWakeup1::SEL_OFFSET) & ExtWakeup1::SEL_MASK),
        }
    }
}

impl Default for ExtWakeup1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ExtWakeup1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ExtWakeup1Val {
        ExtWakeup1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: ExtWakeup1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(ExtWakeup1Val) -> ExtWakeup1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EXT_WAKEUP1_STATUS Register
///
/// need_des
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **EXT_WAKEUP1_STATUS** `[15:0]` (RO)
///
///   ext wakeup1 status
#[repr(transparent)]
pub struct ExtWakeup1Status {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for ExtWakeup1Status {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ExtWakeup1Status {
    // ======== Field Constants ========

    /// **EXT_WAKEUP1_STATUS**
    ///
    /// ext wakeup1 status
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const EXT_WAKEUP1_STATUS_OFFSET: u32 = 0;
    pub const EXT_WAKEUP1_STATUS_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **EXT_WAKEUP1_STATUS** `[15:0]` (RO)
    ///
    /// ext wakeup1 status
    #[inline(always)]
    pub fn ext_wakeup1_status(&self) -> u32 {
        (self.value.read_bits() & Self::EXT_WAKEUP1_STATUS_MASK) >> Self::EXT_WAKEUP1_STATUS_OFFSET
    }
}

/// EXT_WAKEUP1_STATUS value type for batch read/modify/write operations
///
/// Use `extwakeup1status.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `extwakeup1status.write_val(val)`.
#[derive(Clone, Copy)]
pub struct ExtWakeup1StatusVal {
    bits: u32,
}

impl ExtWakeup1StatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **EXT_WAKEUP1_STATUS** `[15:0]`
    ///
    /// ext wakeup1 status
    #[inline(always)]
    pub const fn ext_wakeup1_status(&self) -> u32 {
        (self.bits & ExtWakeup1Status::EXT_WAKEUP1_STATUS_MASK)
            >> ExtWakeup1Status::EXT_WAKEUP1_STATUS_OFFSET
    }
}

impl Default for ExtWakeup1StatusVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl ExtWakeup1Status {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> ExtWakeup1StatusVal {
        ExtWakeup1StatusVal::from_bits(self.value.read_bits())
    }
}

/// LP_TCM_PWR_CTRL Register
///
/// need_des
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **LP_TCM_RAM_CLK_FORCE_ON** `[7]` (RW)
///
///   need_des
///
/// - **LP_TCM_ROM_CLK_FORCE_ON** `[5]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpTcmPwrCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpTcmPwrCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpTcmPwrCtrl {
    // ======== Field Constants ========

    /// **LP_TCM_RAM_CLK_FORCE_ON**
    ///
    /// need_des
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const LP_TCM_RAM_CLK_FORCE_ON_OFFSET: u32 = 7;
    pub const LP_TCM_RAM_CLK_FORCE_ON_MASK: u32 = 0x00000080;
    pub const LP_TCM_RAM_CLK_FORCE_ON: u32 = 1 << 7;

    /// **LP_TCM_ROM_CLK_FORCE_ON**
    ///
    /// need_des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const LP_TCM_ROM_CLK_FORCE_ON_OFFSET: u32 = 5;
    pub const LP_TCM_ROM_CLK_FORCE_ON_MASK: u32 = 0x00000020;
    pub const LP_TCM_ROM_CLK_FORCE_ON: u32 = 1 << 5;

    // ======== Field Accessors ========

    /// **LP_TCM_RAM_CLK_FORCE_ON** `[7]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_tcm_ram_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::LP_TCM_RAM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set LP_TCM_RAM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_tcm_ram_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_TCM_RAM_CLK_FORCE_ON_MASK);
    }

    /// Clear LP_TCM_RAM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_tcm_ram_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_TCM_RAM_CLK_FORCE_ON_MASK);
    }

    /// **LP_TCM_ROM_CLK_FORCE_ON** `[5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_tcm_rom_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::LP_TCM_ROM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set LP_TCM_ROM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_tcm_rom_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_TCM_ROM_CLK_FORCE_ON_MASK);
    }

    /// Clear LP_TCM_ROM_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_tcm_rom_clk_force_on(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_TCM_ROM_CLK_FORCE_ON_MASK);
    }
}

/// LP_TCM_PWR_CTRL value type for batch read/modify/write operations
///
/// Use `lptcmpwrctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lptcmpwrctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpTcmPwrCtrlVal {
    bits: u32,
}

impl LpTcmPwrCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_TCM_RAM_CLK_FORCE_ON** `[7:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_tcm_ram_clk_force_on(&self) -> bool {
        (self.bits & LpTcmPwrCtrl::LP_TCM_RAM_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **LP_TCM_ROM_CLK_FORCE_ON** `[5:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_tcm_rom_clk_force_on(&self) -> bool {
        (self.bits & LpTcmPwrCtrl::LP_TCM_ROM_CLK_FORCE_ON_MASK) != 0
    }

    /// Set **LP_TCM_RAM_CLK_FORCE_ON** `[7:7]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_tcm_ram_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpTcmPwrCtrl::LP_TCM_RAM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !LpTcmPwrCtrl::LP_TCM_RAM_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **LP_TCM_ROM_CLK_FORCE_ON** `[5:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_tcm_rom_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpTcmPwrCtrl::LP_TCM_ROM_CLK_FORCE_ON_MASK
            } else {
                self.bits & !LpTcmPwrCtrl::LP_TCM_ROM_CLK_FORCE_ON_MASK
            },
        }
    }
}

impl Default for LpTcmPwrCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpTcmPwrCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpTcmPwrCtrlVal {
        LpTcmPwrCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpTcmPwrCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpTcmPwrCtrlVal) -> LpTcmPwrCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BOOT_ADDR_HP_LP Register
///
/// need_des
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **BOOT_ADDR_HP_LP** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct BootAddrHpLp {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BootAddrHpLp {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BootAddrHpLp {
    // ======== Field Constants ========

    /// **BOOT_ADDR_HP_LP**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const BOOT_ADDR_HP_LP_OFFSET: u32 = 0;
    pub const BOOT_ADDR_HP_LP_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **BOOT_ADDR_HP_LP** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn boot_addr_hp_lp(&self) -> u32 {
        (self.value.read_bits() & Self::BOOT_ADDR_HP_LP_MASK) >> Self::BOOT_ADDR_HP_LP_OFFSET
    }

    /// Write **BOOT_ADDR_HP_LP** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_boot_addr_hp_lp(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BOOT_ADDR_HP_LP_MASK)
                | ((value << Self::BOOT_ADDR_HP_LP_OFFSET) & Self::BOOT_ADDR_HP_LP_MASK)
        });
    }
}

/// BOOT_ADDR_HP_LP value type for batch read/modify/write operations
///
/// Use `bootaddrhplp.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootaddrhplp.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BootAddrHpLpVal {
    bits: u32,
}

impl BootAddrHpLpVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOT_ADDR_HP_LP** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn boot_addr_hp_lp(&self) -> u32 {
        (self.bits & BootAddrHpLp::BOOT_ADDR_HP_LP_MASK) >> BootAddrHpLp::BOOT_ADDR_HP_LP_OFFSET
    }

    /// Set **BOOT_ADDR_HP_LP** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_boot_addr_hp_lp(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BootAddrHpLp::BOOT_ADDR_HP_LP_MASK)
                | ((value << BootAddrHpLp::BOOT_ADDR_HP_LP_OFFSET)
                    & BootAddrHpLp::BOOT_ADDR_HP_LP_MASK),
        }
    }
}

impl Default for BootAddrHpLpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BootAddrHpLp {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BootAddrHpLpVal {
        BootAddrHpLpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BootAddrHpLpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BootAddrHpLpVal) -> BootAddrHpLpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE0 Register
///
/// need_des
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **LP_SCRATCH0** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore0 {
    // ======== Field Constants ========

    /// **LP_SCRATCH0**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH0_OFFSET: u32 = 0;
    pub const LP_SCRATCH0_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH0** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch0(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH0_MASK) >> Self::LP_SCRATCH0_OFFSET
    }

    /// Write **LP_SCRATCH0** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH0_MASK)
                | ((value << Self::LP_SCRATCH0_OFFSET) & Self::LP_SCRATCH0_MASK)
        });
    }
}

/// LP_STORE0 value type for batch read/modify/write operations
///
/// Use `lpstore0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore0Val {
    bits: u32,
}

impl LpStore0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH0** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch0(&self) -> u32 {
        (self.bits & LpStore0::LP_SCRATCH0_MASK) >> LpStore0::LP_SCRATCH0_OFFSET
    }

    /// Set **LP_SCRATCH0** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore0::LP_SCRATCH0_MASK)
                | ((value << LpStore0::LP_SCRATCH0_OFFSET) & LpStore0::LP_SCRATCH0_MASK),
        }
    }
}

impl Default for LpStore0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore0Val {
        LpStore0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore0Val) -> LpStore0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE1 Register
///
/// need_des
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **LP_SCRATCH1** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore1 {
    // ======== Field Constants ========

    /// **LP_SCRATCH1**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH1_OFFSET: u32 = 0;
    pub const LP_SCRATCH1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH1** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch1(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH1_MASK) >> Self::LP_SCRATCH1_OFFSET
    }

    /// Write **LP_SCRATCH1** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH1_MASK)
                | ((value << Self::LP_SCRATCH1_OFFSET) & Self::LP_SCRATCH1_MASK)
        });
    }
}

/// LP_STORE1 value type for batch read/modify/write operations
///
/// Use `lpstore1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore1Val {
    bits: u32,
}

impl LpStore1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH1** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch1(&self) -> u32 {
        (self.bits & LpStore1::LP_SCRATCH1_MASK) >> LpStore1::LP_SCRATCH1_OFFSET
    }

    /// Set **LP_SCRATCH1** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore1::LP_SCRATCH1_MASK)
                | ((value << LpStore1::LP_SCRATCH1_OFFSET) & LpStore1::LP_SCRATCH1_MASK),
        }
    }
}

impl Default for LpStore1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore1Val {
        LpStore1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore1Val) -> LpStore1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE2 Register
///
/// need_des
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **LP_SCRATCH2** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore2 {
    // ======== Field Constants ========

    /// **LP_SCRATCH2**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH2_OFFSET: u32 = 0;
    pub const LP_SCRATCH2_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH2** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch2(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH2_MASK) >> Self::LP_SCRATCH2_OFFSET
    }

    /// Write **LP_SCRATCH2** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH2_MASK)
                | ((value << Self::LP_SCRATCH2_OFFSET) & Self::LP_SCRATCH2_MASK)
        });
    }
}

/// LP_STORE2 value type for batch read/modify/write operations
///
/// Use `lpstore2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore2Val {
    bits: u32,
}

impl LpStore2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH2** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch2(&self) -> u32 {
        (self.bits & LpStore2::LP_SCRATCH2_MASK) >> LpStore2::LP_SCRATCH2_OFFSET
    }

    /// Set **LP_SCRATCH2** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore2::LP_SCRATCH2_MASK)
                | ((value << LpStore2::LP_SCRATCH2_OFFSET) & LpStore2::LP_SCRATCH2_MASK),
        }
    }
}

impl Default for LpStore2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore2Val {
        LpStore2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore2Val) -> LpStore2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE3 Register
///
/// need_des
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **LP_SCRATCH3** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore3 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore3 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore3 {
    // ======== Field Constants ========

    /// **LP_SCRATCH3**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH3_OFFSET: u32 = 0;
    pub const LP_SCRATCH3_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH3** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch3(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH3_MASK) >> Self::LP_SCRATCH3_OFFSET
    }

    /// Write **LP_SCRATCH3** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH3_MASK)
                | ((value << Self::LP_SCRATCH3_OFFSET) & Self::LP_SCRATCH3_MASK)
        });
    }
}

/// LP_STORE3 value type for batch read/modify/write operations
///
/// Use `lpstore3.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore3.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore3Val {
    bits: u32,
}

impl LpStore3Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH3** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch3(&self) -> u32 {
        (self.bits & LpStore3::LP_SCRATCH3_MASK) >> LpStore3::LP_SCRATCH3_OFFSET
    }

    /// Set **LP_SCRATCH3** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore3::LP_SCRATCH3_MASK)
                | ((value << LpStore3::LP_SCRATCH3_OFFSET) & LpStore3::LP_SCRATCH3_MASK),
        }
    }
}

impl Default for LpStore3Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore3 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore3Val {
        LpStore3Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore3Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore3Val) -> LpStore3Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE4 Register
///
/// need_des
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **LP_SCRATCH4** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore4 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore4 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore4 {
    // ======== Field Constants ========

    /// **LP_SCRATCH4**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH4_OFFSET: u32 = 0;
    pub const LP_SCRATCH4_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH4** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch4(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH4_MASK) >> Self::LP_SCRATCH4_OFFSET
    }

    /// Write **LP_SCRATCH4** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH4_MASK)
                | ((value << Self::LP_SCRATCH4_OFFSET) & Self::LP_SCRATCH4_MASK)
        });
    }
}

/// LP_STORE4 value type for batch read/modify/write operations
///
/// Use `lpstore4.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore4.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore4Val {
    bits: u32,
}

impl LpStore4Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH4** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch4(&self) -> u32 {
        (self.bits & LpStore4::LP_SCRATCH4_MASK) >> LpStore4::LP_SCRATCH4_OFFSET
    }

    /// Set **LP_SCRATCH4** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore4::LP_SCRATCH4_MASK)
                | ((value << LpStore4::LP_SCRATCH4_OFFSET) & LpStore4::LP_SCRATCH4_MASK),
        }
    }
}

impl Default for LpStore4Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore4 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore4Val {
        LpStore4Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore4Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore4Val) -> LpStore4Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE5 Register
///
/// need_des
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **LP_SCRATCH5** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore5 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore5 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore5 {
    // ======== Field Constants ========

    /// **LP_SCRATCH5**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH5_OFFSET: u32 = 0;
    pub const LP_SCRATCH5_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH5** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch5(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH5_MASK) >> Self::LP_SCRATCH5_OFFSET
    }

    /// Write **LP_SCRATCH5** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH5_MASK)
                | ((value << Self::LP_SCRATCH5_OFFSET) & Self::LP_SCRATCH5_MASK)
        });
    }
}

/// LP_STORE5 value type for batch read/modify/write operations
///
/// Use `lpstore5.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore5.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore5Val {
    bits: u32,
}

impl LpStore5Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH5** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch5(&self) -> u32 {
        (self.bits & LpStore5::LP_SCRATCH5_MASK) >> LpStore5::LP_SCRATCH5_OFFSET
    }

    /// Set **LP_SCRATCH5** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore5::LP_SCRATCH5_MASK)
                | ((value << LpStore5::LP_SCRATCH5_OFFSET) & LpStore5::LP_SCRATCH5_MASK),
        }
    }
}

impl Default for LpStore5Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore5 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore5Val {
        LpStore5Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore5Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore5Val) -> LpStore5Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE6 Register
///
/// need_des
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **LP_SCRATCH6** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore6 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore6 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore6 {
    // ======== Field Constants ========

    /// **LP_SCRATCH6**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH6_OFFSET: u32 = 0;
    pub const LP_SCRATCH6_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH6** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch6(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH6_MASK) >> Self::LP_SCRATCH6_OFFSET
    }

    /// Write **LP_SCRATCH6** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch6(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH6_MASK)
                | ((value << Self::LP_SCRATCH6_OFFSET) & Self::LP_SCRATCH6_MASK)
        });
    }
}

/// LP_STORE6 value type for batch read/modify/write operations
///
/// Use `lpstore6.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore6.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore6Val {
    bits: u32,
}

impl LpStore6Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH6** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch6(&self) -> u32 {
        (self.bits & LpStore6::LP_SCRATCH6_MASK) >> LpStore6::LP_SCRATCH6_OFFSET
    }

    /// Set **LP_SCRATCH6** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore6::LP_SCRATCH6_MASK)
                | ((value << LpStore6::LP_SCRATCH6_OFFSET) & LpStore6::LP_SCRATCH6_MASK),
        }
    }
}

impl Default for LpStore6Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore6 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore6Val {
        LpStore6Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore6Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore6Val) -> LpStore6Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE7 Register
///
/// need_des
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **LP_SCRATCH7** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore7 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore7 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore7 {
    // ======== Field Constants ========

    /// **LP_SCRATCH7**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH7_OFFSET: u32 = 0;
    pub const LP_SCRATCH7_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH7** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch7(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH7_MASK) >> Self::LP_SCRATCH7_OFFSET
    }

    /// Write **LP_SCRATCH7** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch7(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH7_MASK)
                | ((value << Self::LP_SCRATCH7_OFFSET) & Self::LP_SCRATCH7_MASK)
        });
    }
}

/// LP_STORE7 value type for batch read/modify/write operations
///
/// Use `lpstore7.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore7.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore7Val {
    bits: u32,
}

impl LpStore7Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH7** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch7(&self) -> u32 {
        (self.bits & LpStore7::LP_SCRATCH7_MASK) >> LpStore7::LP_SCRATCH7_OFFSET
    }

    /// Set **LP_SCRATCH7** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore7::LP_SCRATCH7_MASK)
                | ((value << LpStore7::LP_SCRATCH7_OFFSET) & LpStore7::LP_SCRATCH7_MASK),
        }
    }
}

impl Default for LpStore7Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore7 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore7Val {
        LpStore7Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore7Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore7Val) -> LpStore7Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE8 Register
///
/// need_des
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **LP_SCRATCH8** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore8 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore8 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore8 {
    // ======== Field Constants ========

    /// **LP_SCRATCH8**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH8_OFFSET: u32 = 0;
    pub const LP_SCRATCH8_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH8** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch8(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH8_MASK) >> Self::LP_SCRATCH8_OFFSET
    }

    /// Write **LP_SCRATCH8** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch8(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH8_MASK)
                | ((value << Self::LP_SCRATCH8_OFFSET) & Self::LP_SCRATCH8_MASK)
        });
    }
}

/// LP_STORE8 value type for batch read/modify/write operations
///
/// Use `lpstore8.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore8.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore8Val {
    bits: u32,
}

impl LpStore8Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH8** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch8(&self) -> u32 {
        (self.bits & LpStore8::LP_SCRATCH8_MASK) >> LpStore8::LP_SCRATCH8_OFFSET
    }

    /// Set **LP_SCRATCH8** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore8::LP_SCRATCH8_MASK)
                | ((value << LpStore8::LP_SCRATCH8_OFFSET) & LpStore8::LP_SCRATCH8_MASK),
        }
    }
}

impl Default for LpStore8Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore8 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore8Val {
        LpStore8Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore8Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore8Val) -> LpStore8Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE9 Register
///
/// need_des
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **LP_SCRATCH9** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore9 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore9 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore9 {
    // ======== Field Constants ========

    /// **LP_SCRATCH9**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH9_OFFSET: u32 = 0;
    pub const LP_SCRATCH9_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH9** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch9(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH9_MASK) >> Self::LP_SCRATCH9_OFFSET
    }

    /// Write **LP_SCRATCH9** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch9(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH9_MASK)
                | ((value << Self::LP_SCRATCH9_OFFSET) & Self::LP_SCRATCH9_MASK)
        });
    }
}

/// LP_STORE9 value type for batch read/modify/write operations
///
/// Use `lpstore9.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore9.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore9Val {
    bits: u32,
}

impl LpStore9Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH9** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch9(&self) -> u32 {
        (self.bits & LpStore9::LP_SCRATCH9_MASK) >> LpStore9::LP_SCRATCH9_OFFSET
    }

    /// Set **LP_SCRATCH9** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch9(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore9::LP_SCRATCH9_MASK)
                | ((value << LpStore9::LP_SCRATCH9_OFFSET) & LpStore9::LP_SCRATCH9_MASK),
        }
    }
}

impl Default for LpStore9Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore9 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore9Val {
        LpStore9Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore9Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore9Val) -> LpStore9Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE10 Register
///
/// need_des
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **LP_SCRATCH10** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore10 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore10 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore10 {
    // ======== Field Constants ========

    /// **LP_SCRATCH10**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH10_OFFSET: u32 = 0;
    pub const LP_SCRATCH10_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH10** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch10(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH10_MASK) >> Self::LP_SCRATCH10_OFFSET
    }

    /// Write **LP_SCRATCH10** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch10(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH10_MASK)
                | ((value << Self::LP_SCRATCH10_OFFSET) & Self::LP_SCRATCH10_MASK)
        });
    }
}

/// LP_STORE10 value type for batch read/modify/write operations
///
/// Use `lpstore10.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore10.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore10Val {
    bits: u32,
}

impl LpStore10Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH10** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch10(&self) -> u32 {
        (self.bits & LpStore10::LP_SCRATCH10_MASK) >> LpStore10::LP_SCRATCH10_OFFSET
    }

    /// Set **LP_SCRATCH10** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch10(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore10::LP_SCRATCH10_MASK)
                | ((value << LpStore10::LP_SCRATCH10_OFFSET) & LpStore10::LP_SCRATCH10_MASK),
        }
    }
}

impl Default for LpStore10Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore10 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore10Val {
        LpStore10Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore10Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore10Val) -> LpStore10Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE11 Register
///
/// need_des
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **LP_SCRATCH11** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore11 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore11 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore11 {
    // ======== Field Constants ========

    /// **LP_SCRATCH11**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH11_OFFSET: u32 = 0;
    pub const LP_SCRATCH11_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH11** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch11(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH11_MASK) >> Self::LP_SCRATCH11_OFFSET
    }

    /// Write **LP_SCRATCH11** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch11(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH11_MASK)
                | ((value << Self::LP_SCRATCH11_OFFSET) & Self::LP_SCRATCH11_MASK)
        });
    }
}

/// LP_STORE11 value type for batch read/modify/write operations
///
/// Use `lpstore11.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore11.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore11Val {
    bits: u32,
}

impl LpStore11Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH11** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch11(&self) -> u32 {
        (self.bits & LpStore11::LP_SCRATCH11_MASK) >> LpStore11::LP_SCRATCH11_OFFSET
    }

    /// Set **LP_SCRATCH11** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch11(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore11::LP_SCRATCH11_MASK)
                | ((value << LpStore11::LP_SCRATCH11_OFFSET) & LpStore11::LP_SCRATCH11_MASK),
        }
    }
}

impl Default for LpStore11Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore11 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore11Val {
        LpStore11Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore11Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore11Val) -> LpStore11Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE12 Register
///
/// need_des
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **LP_SCRATCH12** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore12 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore12 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore12 {
    // ======== Field Constants ========

    /// **LP_SCRATCH12**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH12_OFFSET: u32 = 0;
    pub const LP_SCRATCH12_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH12** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch12(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH12_MASK) >> Self::LP_SCRATCH12_OFFSET
    }

    /// Write **LP_SCRATCH12** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch12(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH12_MASK)
                | ((value << Self::LP_SCRATCH12_OFFSET) & Self::LP_SCRATCH12_MASK)
        });
    }
}

/// LP_STORE12 value type for batch read/modify/write operations
///
/// Use `lpstore12.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore12.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore12Val {
    bits: u32,
}

impl LpStore12Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH12** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch12(&self) -> u32 {
        (self.bits & LpStore12::LP_SCRATCH12_MASK) >> LpStore12::LP_SCRATCH12_OFFSET
    }

    /// Set **LP_SCRATCH12** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch12(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore12::LP_SCRATCH12_MASK)
                | ((value << LpStore12::LP_SCRATCH12_OFFSET) & LpStore12::LP_SCRATCH12_MASK),
        }
    }
}

impl Default for LpStore12Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore12 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore12Val {
        LpStore12Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore12Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore12Val) -> LpStore12Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE13 Register
///
/// need_des
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **LP_SCRATCH13** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore13 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore13 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore13 {
    // ======== Field Constants ========

    /// **LP_SCRATCH13**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH13_OFFSET: u32 = 0;
    pub const LP_SCRATCH13_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH13** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch13(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH13_MASK) >> Self::LP_SCRATCH13_OFFSET
    }

    /// Write **LP_SCRATCH13** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch13(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH13_MASK)
                | ((value << Self::LP_SCRATCH13_OFFSET) & Self::LP_SCRATCH13_MASK)
        });
    }
}

/// LP_STORE13 value type for batch read/modify/write operations
///
/// Use `lpstore13.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore13.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore13Val {
    bits: u32,
}

impl LpStore13Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH13** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch13(&self) -> u32 {
        (self.bits & LpStore13::LP_SCRATCH13_MASK) >> LpStore13::LP_SCRATCH13_OFFSET
    }

    /// Set **LP_SCRATCH13** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch13(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore13::LP_SCRATCH13_MASK)
                | ((value << LpStore13::LP_SCRATCH13_OFFSET) & LpStore13::LP_SCRATCH13_MASK),
        }
    }
}

impl Default for LpStore13Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore13 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore13Val {
        LpStore13Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore13Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore13Val) -> LpStore13Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE14 Register
///
/// need_des
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **LP_SCRATCH14** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore14 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore14 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore14 {
    // ======== Field Constants ========

    /// **LP_SCRATCH14**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH14_OFFSET: u32 = 0;
    pub const LP_SCRATCH14_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH14** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch14(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH14_MASK) >> Self::LP_SCRATCH14_OFFSET
    }

    /// Write **LP_SCRATCH14** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch14(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH14_MASK)
                | ((value << Self::LP_SCRATCH14_OFFSET) & Self::LP_SCRATCH14_MASK)
        });
    }
}

/// LP_STORE14 value type for batch read/modify/write operations
///
/// Use `lpstore14.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore14.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore14Val {
    bits: u32,
}

impl LpStore14Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH14** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch14(&self) -> u32 {
        (self.bits & LpStore14::LP_SCRATCH14_MASK) >> LpStore14::LP_SCRATCH14_OFFSET
    }

    /// Set **LP_SCRATCH14** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch14(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore14::LP_SCRATCH14_MASK)
                | ((value << LpStore14::LP_SCRATCH14_OFFSET) & LpStore14::LP_SCRATCH14_MASK),
        }
    }
}

impl Default for LpStore14Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore14 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore14Val {
        LpStore14Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore14Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore14Val) -> LpStore14Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_STORE15 Register
///
/// need_des
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **LP_SCRATCH15** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpStore15 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpStore15 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpStore15 {
    // ======== Field Constants ========

    /// **LP_SCRATCH15**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_SCRATCH15_OFFSET: u32 = 0;
    pub const LP_SCRATCH15_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_SCRATCH15** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_scratch15(&self) -> u32 {
        (self.value.read_bits() & Self::LP_SCRATCH15_MASK) >> Self::LP_SCRATCH15_OFFSET
    }

    /// Write **LP_SCRATCH15** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_scratch15(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_SCRATCH15_MASK)
                | ((value << Self::LP_SCRATCH15_OFFSET) & Self::LP_SCRATCH15_MASK)
        });
    }
}

/// LP_STORE15 value type for batch read/modify/write operations
///
/// Use `lpstore15.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpstore15.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpStore15Val {
    bits: u32,
}

impl LpStore15Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_SCRATCH15** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_scratch15(&self) -> u32 {
        (self.bits & LpStore15::LP_SCRATCH15_MASK) >> LpStore15::LP_SCRATCH15_OFFSET
    }

    /// Set **LP_SCRATCH15** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_scratch15(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpStore15::LP_SCRATCH15_MASK)
                | ((value << LpStore15::LP_SCRATCH15_OFFSET) & LpStore15::LP_SCRATCH15_MASK),
        }
    }
}

impl Default for LpStore15Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpStore15 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpStore15Val {
        LpStore15Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpStore15Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpStore15Val) -> LpStore15Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_PROBEA_CTRL Register
///
/// need_des
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **PROBE_GLOBAL_EN** `[28]` (RW)
///
///   need_des
///
/// - **PROBE_H_SEL** `[27:26]` (RW)
///
///   need_des
///
/// - **PROBE_L_SEL** `[25:24]` (RW)
///
///   need_des
///
/// - **PROBE_A_TOP_SEL** `[23:16]` (RW)
///
///   need_des
///
/// - **PROBE_A_MOD_SEL** `[15:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpProbeaCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpProbeaCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpProbeaCtrl {
    // ======== Field Constants ========

    /// **PROBE_GLOBAL_EN**
    ///
    /// need_des
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const PROBE_GLOBAL_EN_OFFSET: u32 = 28;
    pub const PROBE_GLOBAL_EN_MASK: u32 = 0x10000000;
    pub const PROBE_GLOBAL_EN: u32 = 1 << 28;

    /// **PROBE_H_SEL**
    ///
    /// need_des
    ///
    /// Bits `[27:26]`, mask `0x0c000000`
    pub const PROBE_H_SEL_OFFSET: u32 = 26;
    pub const PROBE_H_SEL_MASK: u32 = 0x0c000000;

    /// **PROBE_L_SEL**
    ///
    /// need_des
    ///
    /// Bits `[25:24]`, mask `0x03000000`
    pub const PROBE_L_SEL_OFFSET: u32 = 24;
    pub const PROBE_L_SEL_MASK: u32 = 0x03000000;

    /// **PROBE_A_TOP_SEL**
    ///
    /// need_des
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const PROBE_A_TOP_SEL_OFFSET: u32 = 16;
    pub const PROBE_A_TOP_SEL_MASK: u32 = 0x00ff0000;

    /// **PROBE_A_MOD_SEL**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PROBE_A_MOD_SEL_OFFSET: u32 = 0;
    pub const PROBE_A_MOD_SEL_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PROBE_GLOBAL_EN** `[28]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn probe_global_en(&self) -> bool {
        (self.value.read_bits() & Self::PROBE_GLOBAL_EN_MASK) != 0
    }

    /// Set PROBE_GLOBAL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_probe_global_en(&self) {
        self.value.modify_bits(|r| r | Self::PROBE_GLOBAL_EN_MASK);
    }

    /// Clear PROBE_GLOBAL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_probe_global_en(&self) {
        self.value.modify_bits(|r| r & !Self::PROBE_GLOBAL_EN_MASK);
    }

    /// **PROBE_H_SEL** `[27:26]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn probe_h_sel(&self) -> u32 {
        (self.value.read_bits() & Self::PROBE_H_SEL_MASK) >> Self::PROBE_H_SEL_OFFSET
    }

    /// Write **PROBE_H_SEL** `[27:26]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_probe_h_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PROBE_H_SEL_MASK)
                | ((value << Self::PROBE_H_SEL_OFFSET) & Self::PROBE_H_SEL_MASK)
        });
    }

    /// **PROBE_L_SEL** `[25:24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn probe_l_sel(&self) -> u32 {
        (self.value.read_bits() & Self::PROBE_L_SEL_MASK) >> Self::PROBE_L_SEL_OFFSET
    }

    /// Write **PROBE_L_SEL** `[25:24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_probe_l_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PROBE_L_SEL_MASK)
                | ((value << Self::PROBE_L_SEL_OFFSET) & Self::PROBE_L_SEL_MASK)
        });
    }

    /// **PROBE_A_TOP_SEL** `[23:16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn probe_a_top_sel(&self) -> u32 {
        (self.value.read_bits() & Self::PROBE_A_TOP_SEL_MASK) >> Self::PROBE_A_TOP_SEL_OFFSET
    }

    /// Write **PROBE_A_TOP_SEL** `[23:16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_probe_a_top_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PROBE_A_TOP_SEL_MASK)
                | ((value << Self::PROBE_A_TOP_SEL_OFFSET) & Self::PROBE_A_TOP_SEL_MASK)
        });
    }

    /// **PROBE_A_MOD_SEL** `[15:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn probe_a_mod_sel(&self) -> u32 {
        (self.value.read_bits() & Self::PROBE_A_MOD_SEL_MASK) >> Self::PROBE_A_MOD_SEL_OFFSET
    }

    /// Write **PROBE_A_MOD_SEL** `[15:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_probe_a_mod_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PROBE_A_MOD_SEL_MASK)
                | ((value << Self::PROBE_A_MOD_SEL_OFFSET) & Self::PROBE_A_MOD_SEL_MASK)
        });
    }
}

/// LP_PROBEA_CTRL value type for batch read/modify/write operations
///
/// Use `lpprobeactrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpprobeactrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpProbeaCtrlVal {
    bits: u32,
}

impl LpProbeaCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PROBE_GLOBAL_EN** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn probe_global_en(&self) -> bool {
        (self.bits & LpProbeaCtrl::PROBE_GLOBAL_EN_MASK) != 0
    }

    /// Read **PROBE_H_SEL** `[27:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn probe_h_sel(&self) -> u32 {
        (self.bits & LpProbeaCtrl::PROBE_H_SEL_MASK) >> LpProbeaCtrl::PROBE_H_SEL_OFFSET
    }

    /// Read **PROBE_L_SEL** `[25:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn probe_l_sel(&self) -> u32 {
        (self.bits & LpProbeaCtrl::PROBE_L_SEL_MASK) >> LpProbeaCtrl::PROBE_L_SEL_OFFSET
    }

    /// Read **PROBE_A_TOP_SEL** `[23:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn probe_a_top_sel(&self) -> u32 {
        (self.bits & LpProbeaCtrl::PROBE_A_TOP_SEL_MASK) >> LpProbeaCtrl::PROBE_A_TOP_SEL_OFFSET
    }

    /// Read **PROBE_A_MOD_SEL** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn probe_a_mod_sel(&self) -> u32 {
        (self.bits & LpProbeaCtrl::PROBE_A_MOD_SEL_MASK) >> LpProbeaCtrl::PROBE_A_MOD_SEL_OFFSET
    }

    /// Set **PROBE_GLOBAL_EN** `[28:28]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_probe_global_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpProbeaCtrl::PROBE_GLOBAL_EN_MASK
            } else {
                self.bits & !LpProbeaCtrl::PROBE_GLOBAL_EN_MASK
            },
        }
    }

    /// Set **PROBE_H_SEL** `[27:26]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_probe_h_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpProbeaCtrl::PROBE_H_SEL_MASK)
                | ((value << LpProbeaCtrl::PROBE_H_SEL_OFFSET) & LpProbeaCtrl::PROBE_H_SEL_MASK),
        }
    }

    /// Set **PROBE_L_SEL** `[25:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_probe_l_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpProbeaCtrl::PROBE_L_SEL_MASK)
                | ((value << LpProbeaCtrl::PROBE_L_SEL_OFFSET) & LpProbeaCtrl::PROBE_L_SEL_MASK),
        }
    }

    /// Set **PROBE_A_TOP_SEL** `[23:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_probe_a_top_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpProbeaCtrl::PROBE_A_TOP_SEL_MASK)
                | ((value << LpProbeaCtrl::PROBE_A_TOP_SEL_OFFSET)
                    & LpProbeaCtrl::PROBE_A_TOP_SEL_MASK),
        }
    }

    /// Set **PROBE_A_MOD_SEL** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_probe_a_mod_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpProbeaCtrl::PROBE_A_MOD_SEL_MASK)
                | ((value << LpProbeaCtrl::PROBE_A_MOD_SEL_OFFSET)
                    & LpProbeaCtrl::PROBE_A_MOD_SEL_MASK),
        }
    }
}

impl Default for LpProbeaCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpProbeaCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpProbeaCtrlVal {
        LpProbeaCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpProbeaCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpProbeaCtrlVal) -> LpProbeaCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_PROBEB_CTRL Register
///
/// need_des
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **PROBE_B_EN** `[24]` (RW)
///
///   need_des
///
/// - **PROBE_B_TOP_SEL** `[23:16]` (RW)
///
///   need_des
///
/// - **PROBE_B_MOD_SEL** `[15:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpProbebCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpProbebCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpProbebCtrl {
    // ======== Field Constants ========

    /// **PROBE_B_EN**
    ///
    /// need_des
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const PROBE_B_EN_OFFSET: u32 = 24;
    pub const PROBE_B_EN_MASK: u32 = 0x01000000;
    pub const PROBE_B_EN: u32 = 1 << 24;

    /// **PROBE_B_TOP_SEL**
    ///
    /// need_des
    ///
    /// Bits `[23:16]`, mask `0x00ff0000`
    pub const PROBE_B_TOP_SEL_OFFSET: u32 = 16;
    pub const PROBE_B_TOP_SEL_MASK: u32 = 0x00ff0000;

    /// **PROBE_B_MOD_SEL**
    ///
    /// need_des
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const PROBE_B_MOD_SEL_OFFSET: u32 = 0;
    pub const PROBE_B_MOD_SEL_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **PROBE_B_EN** `[24]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn probe_b_en(&self) -> bool {
        (self.value.read_bits() & Self::PROBE_B_EN_MASK) != 0
    }

    /// Set PROBE_B_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_probe_b_en(&self) {
        self.value.modify_bits(|r| r | Self::PROBE_B_EN_MASK);
    }

    /// Clear PROBE_B_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_probe_b_en(&self) {
        self.value.modify_bits(|r| r & !Self::PROBE_B_EN_MASK);
    }

    /// **PROBE_B_TOP_SEL** `[23:16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn probe_b_top_sel(&self) -> u32 {
        (self.value.read_bits() & Self::PROBE_B_TOP_SEL_MASK) >> Self::PROBE_B_TOP_SEL_OFFSET
    }

    /// Write **PROBE_B_TOP_SEL** `[23:16]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_probe_b_top_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PROBE_B_TOP_SEL_MASK)
                | ((value << Self::PROBE_B_TOP_SEL_OFFSET) & Self::PROBE_B_TOP_SEL_MASK)
        });
    }

    /// **PROBE_B_MOD_SEL** `[15:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn probe_b_mod_sel(&self) -> u32 {
        (self.value.read_bits() & Self::PROBE_B_MOD_SEL_MASK) >> Self::PROBE_B_MOD_SEL_OFFSET
    }

    /// Write **PROBE_B_MOD_SEL** `[15:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_probe_b_mod_sel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PROBE_B_MOD_SEL_MASK)
                | ((value << Self::PROBE_B_MOD_SEL_OFFSET) & Self::PROBE_B_MOD_SEL_MASK)
        });
    }
}

/// LP_PROBEB_CTRL value type for batch read/modify/write operations
///
/// Use `lpprobebctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpprobebctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpProbebCtrlVal {
    bits: u32,
}

impl LpProbebCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PROBE_B_EN** `[24:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn probe_b_en(&self) -> bool {
        (self.bits & LpProbebCtrl::PROBE_B_EN_MASK) != 0
    }

    /// Read **PROBE_B_TOP_SEL** `[23:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn probe_b_top_sel(&self) -> u32 {
        (self.bits & LpProbebCtrl::PROBE_B_TOP_SEL_MASK) >> LpProbebCtrl::PROBE_B_TOP_SEL_OFFSET
    }

    /// Read **PROBE_B_MOD_SEL** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn probe_b_mod_sel(&self) -> u32 {
        (self.bits & LpProbebCtrl::PROBE_B_MOD_SEL_MASK) >> LpProbebCtrl::PROBE_B_MOD_SEL_OFFSET
    }

    /// Set **PROBE_B_EN** `[24:24]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_probe_b_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpProbebCtrl::PROBE_B_EN_MASK
            } else {
                self.bits & !LpProbebCtrl::PROBE_B_EN_MASK
            },
        }
    }

    /// Set **PROBE_B_TOP_SEL** `[23:16]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_probe_b_top_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpProbebCtrl::PROBE_B_TOP_SEL_MASK)
                | ((value << LpProbebCtrl::PROBE_B_TOP_SEL_OFFSET)
                    & LpProbebCtrl::PROBE_B_TOP_SEL_MASK),
        }
    }

    /// Set **PROBE_B_MOD_SEL** `[15:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_probe_b_mod_sel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpProbebCtrl::PROBE_B_MOD_SEL_MASK)
                | ((value << LpProbebCtrl::PROBE_B_MOD_SEL_OFFSET)
                    & LpProbebCtrl::PROBE_B_MOD_SEL_MASK),
        }
    }
}

impl Default for LpProbebCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpProbebCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpProbebCtrlVal {
        LpProbebCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpProbebCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpProbebCtrlVal) -> LpProbebCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_PROBE_OUT Register
///
/// need_des
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **PROBE_TOP_OUT** `[31:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct LpProbeOut {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpProbeOut {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpProbeOut {
    // ======== Field Constants ========

    /// **PROBE_TOP_OUT**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PROBE_TOP_OUT_OFFSET: u32 = 0;
    pub const PROBE_TOP_OUT_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PROBE_TOP_OUT** `[31:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn probe_top_out(&self) -> u32 {
        (self.value.read_bits() & Self::PROBE_TOP_OUT_MASK) >> Self::PROBE_TOP_OUT_OFFSET
    }
}

/// LP_PROBE_OUT value type for batch read/modify/write operations
///
/// Use `lpprobeout.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpprobeout.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpProbeOutVal {
    bits: u32,
}

impl LpProbeOutVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PROBE_TOP_OUT** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn probe_top_out(&self) -> u32 {
        (self.bits & LpProbeOut::PROBE_TOP_OUT_MASK) >> LpProbeOut::PROBE_TOP_OUT_OFFSET
    }
}

impl Default for LpProbeOutVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpProbeOut {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpProbeOutVal {
        LpProbeOutVal::from_bits(self.value.read_bits())
    }
}

/// F2S_APB_BRG_CNTL Register
///
/// need_des
///
/// Address offset: 0x009c
///
/// ## Fields
///
/// - **F2S_APB_POSTW_EN** `[0]` (RW)
///
///   reserved
#[repr(transparent)]
pub struct F2sApbBrgCntl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for F2sApbBrgCntl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl F2sApbBrgCntl {
    // ======== Field Constants ========

    /// **F2S_APB_POSTW_EN**
    ///
    /// reserved
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const F2S_APB_POSTW_EN_OFFSET: u32 = 0;
    pub const F2S_APB_POSTW_EN_MASK: u32 = 0x00000001;
    pub const F2S_APB_POSTW_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **F2S_APB_POSTW_EN** `[0]` (RW)
    ///
    /// reserved
    #[inline(always)]
    pub fn f2s_apb_postw_en(&self) -> bool {
        (self.value.read_bits() & Self::F2S_APB_POSTW_EN_MASK) != 0
    }

    /// Set F2S_APB_POSTW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_f2s_apb_postw_en(&self) {
        self.value.modify_bits(|r| r | Self::F2S_APB_POSTW_EN_MASK);
    }

    /// Clear F2S_APB_POSTW_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_f2s_apb_postw_en(&self) {
        self.value.modify_bits(|r| r & !Self::F2S_APB_POSTW_EN_MASK);
    }
}

/// F2S_APB_BRG_CNTL value type for batch read/modify/write operations
///
/// Use `f2sapbbrgcntl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `f2sapbbrgcntl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct F2sApbBrgCntlVal {
    bits: u32,
}

impl F2sApbBrgCntlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **F2S_APB_POSTW_EN** `[0:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn f2s_apb_postw_en(&self) -> bool {
        (self.bits & F2sApbBrgCntl::F2S_APB_POSTW_EN_MASK) != 0
    }

    /// Set **F2S_APB_POSTW_EN** `[0:0]`
    ///
    /// reserved
    #[inline(always)]
    pub const fn with_f2s_apb_postw_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | F2sApbBrgCntl::F2S_APB_POSTW_EN_MASK
            } else {
                self.bits & !F2sApbBrgCntl::F2S_APB_POSTW_EN_MASK
            },
        }
    }
}

impl Default for F2sApbBrgCntlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl F2sApbBrgCntl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> F2sApbBrgCntlVal {
        F2sApbBrgCntlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: F2sApbBrgCntlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(F2sApbBrgCntlVal) -> F2sApbBrgCntlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// USB_CTRL Register
///
/// need_des
///
/// Address offset: 0x0100
///
/// ## Fields
///
/// - **USBOTG20_IN_SUSPEND** `[3]` (RW)
///
///   indicate usb otg2.0 is in suspend state.
///
/// - **USBOTG20_WAKEUP_CLR** `[2]` (WO)
///
///   clear usb wakeup to PMU.
///
/// - **SW_USB_PHY_SEL** `[1]` (RW)
///
///   need_des
///
/// - **SW_HW_USB_PHY_SEL** `[0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct UsbCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for UsbCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UsbCtrl {
    // ======== Field Constants ========

    /// **USBOTG20_IN_SUSPEND**
    ///
    /// indicate usb otg2.0 is in suspend state.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const USBOTG20_IN_SUSPEND_OFFSET: u32 = 3;
    pub const USBOTG20_IN_SUSPEND_MASK: u32 = 0x00000008;
    pub const USBOTG20_IN_SUSPEND: u32 = 1 << 3;

    /// **USBOTG20_WAKEUP_CLR**
    ///
    /// clear usb wakeup to PMU.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const USBOTG20_WAKEUP_CLR_OFFSET: u32 = 2;
    pub const USBOTG20_WAKEUP_CLR_MASK: u32 = 0x00000004;
    pub const USBOTG20_WAKEUP_CLR: u32 = 1 << 2;

    /// **SW_USB_PHY_SEL**
    ///
    /// need_des
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SW_USB_PHY_SEL_OFFSET: u32 = 1;
    pub const SW_USB_PHY_SEL_MASK: u32 = 0x00000002;
    pub const SW_USB_PHY_SEL: u32 = 1 << 1;

    /// **SW_HW_USB_PHY_SEL**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SW_HW_USB_PHY_SEL_OFFSET: u32 = 0;
    pub const SW_HW_USB_PHY_SEL_MASK: u32 = 0x00000001;
    pub const SW_HW_USB_PHY_SEL: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **USBOTG20_IN_SUSPEND** `[3]` (RW)
    ///
    /// indicate usb otg2.0 is in suspend state.
    #[inline(always)]
    pub fn usbotg20_in_suspend(&self) -> bool {
        (self.value.read_bits() & Self::USBOTG20_IN_SUSPEND_MASK) != 0
    }

    /// Set USBOTG20_IN_SUSPEND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usbotg20_in_suspend(&self) {
        self.value
            .modify_bits(|r| r | Self::USBOTG20_IN_SUSPEND_MASK);
    }

    /// Clear USBOTG20_IN_SUSPEND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usbotg20_in_suspend(&self) {
        self.value
            .modify_bits(|r| r & !Self::USBOTG20_IN_SUSPEND_MASK);
    }

    /// Trigger **USBOTG20_WAKEUP_CLR** (WO)
    ///
    /// clear usb wakeup to PMU.
    #[inline(always)]
    pub fn trigger_usbotg20_wakeup_clr(&self) {
        self.value
            .modify_bits(|r| r | Self::USBOTG20_WAKEUP_CLR_MASK);
    }

    /// **SW_USB_PHY_SEL** `[1]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn sw_usb_phy_sel(&self) -> bool {
        (self.value.read_bits() & Self::SW_USB_PHY_SEL_MASK) != 0
    }

    /// Set SW_USB_PHY_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sw_usb_phy_sel(&self) {
        self.value.modify_bits(|r| r | Self::SW_USB_PHY_SEL_MASK);
    }

    /// Clear SW_USB_PHY_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sw_usb_phy_sel(&self) {
        self.value.modify_bits(|r| r & !Self::SW_USB_PHY_SEL_MASK);
    }

    /// **SW_HW_USB_PHY_SEL** `[0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn sw_hw_usb_phy_sel(&self) -> bool {
        (self.value.read_bits() & Self::SW_HW_USB_PHY_SEL_MASK) != 0
    }

    /// Set SW_HW_USB_PHY_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sw_hw_usb_phy_sel(&self) {
        self.value.modify_bits(|r| r | Self::SW_HW_USB_PHY_SEL_MASK);
    }

    /// Clear SW_HW_USB_PHY_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sw_hw_usb_phy_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::SW_HW_USB_PHY_SEL_MASK);
    }
}

/// USB_CTRL value type for batch read/modify/write operations
///
/// Use `usbctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `usbctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct UsbCtrlVal {
    bits: u32,
}

impl UsbCtrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **USBOTG20_IN_SUSPEND** `[3:3]`
    ///
    /// indicate usb otg2.0 is in suspend state.
    #[inline(always)]
    pub const fn usbotg20_in_suspend(&self) -> bool {
        (self.bits & UsbCtrl::USBOTG20_IN_SUSPEND_MASK) != 0
    }

    /// Read **SW_USB_PHY_SEL** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn sw_usb_phy_sel(&self) -> bool {
        (self.bits & UsbCtrl::SW_USB_PHY_SEL_MASK) != 0
    }

    /// Read **SW_HW_USB_PHY_SEL** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn sw_hw_usb_phy_sel(&self) -> bool {
        (self.bits & UsbCtrl::SW_HW_USB_PHY_SEL_MASK) != 0
    }

    /// Set **USBOTG20_IN_SUSPEND** `[3:3]`
    ///
    /// indicate usb otg2.0 is in suspend state.
    #[inline(always)]
    pub const fn with_usbotg20_in_suspend(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbCtrl::USBOTG20_IN_SUSPEND_MASK
            } else {
                self.bits & !UsbCtrl::USBOTG20_IN_SUSPEND_MASK
            },
        }
    }

    /// Set **USBOTG20_WAKEUP_CLR** `[2:2]`
    ///
    /// clear usb wakeup to PMU.
    #[inline(always)]
    pub const fn with_usbotg20_wakeup_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbCtrl::USBOTG20_WAKEUP_CLR_MASK
            } else {
                self.bits & !UsbCtrl::USBOTG20_WAKEUP_CLR_MASK
            },
        }
    }

    /// Set **SW_USB_PHY_SEL** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_sw_usb_phy_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbCtrl::SW_USB_PHY_SEL_MASK
            } else {
                self.bits & !UsbCtrl::SW_USB_PHY_SEL_MASK
            },
        }
    }

    /// Set **SW_HW_USB_PHY_SEL** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_sw_hw_usb_phy_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | UsbCtrl::SW_HW_USB_PHY_SEL_MASK
            } else {
                self.bits & !UsbCtrl::SW_HW_USB_PHY_SEL_MASK
            },
        }
    }
}

impl Default for UsbCtrlVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl UsbCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> UsbCtrlVal {
        UsbCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: UsbCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(UsbCtrlVal) -> UsbCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// ANA_XPD_PAD_GROUP Register
///
/// need_des
///
/// Address offset: 0x010c
/// Reset value: 0x000000ff
///
/// ## Fields
///
/// - **ANA_REG_XPD_PAD_GROUP** `[7:0]` (RW)
///
///   Set 1 to power up pad group
#[repr(transparent)]
pub struct AnaXpdPadGroup {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for AnaXpdPadGroup {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AnaXpdPadGroup {
    // ======== Field Constants ========

    /// **ANA_REG_XPD_PAD_GROUP**
    ///
    /// Set 1 to power up pad group
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const ANA_REG_XPD_PAD_GROUP_OFFSET: u32 = 0;
    pub const ANA_REG_XPD_PAD_GROUP_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **ANA_REG_XPD_PAD_GROUP** `[7:0]` (RW)
    ///
    /// Set 1 to power up pad group
    #[inline(always)]
    pub fn ana_reg_xpd_pad_group(&self) -> u32 {
        (self.value.read_bits() & Self::ANA_REG_XPD_PAD_GROUP_MASK)
            >> Self::ANA_REG_XPD_PAD_GROUP_OFFSET
    }

    /// Write **ANA_REG_XPD_PAD_GROUP** `[7:0]` (RW)
    ///
    /// Set 1 to power up pad group
    #[inline(always)]
    pub fn write_ana_reg_xpd_pad_group(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ANA_REG_XPD_PAD_GROUP_MASK)
                | ((value << Self::ANA_REG_XPD_PAD_GROUP_OFFSET) & Self::ANA_REG_XPD_PAD_GROUP_MASK)
        });
    }
}

/// ANA_XPD_PAD_GROUP value type for batch read/modify/write operations
///
/// Use `anaxpdpadgroup.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `anaxpdpadgroup.write_val(val)`.
#[derive(Clone, Copy)]
pub struct AnaXpdPadGroupVal {
    bits: u32,
}

impl AnaXpdPadGroupVal {
    /// Create from raw bits
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

    /// Read **ANA_REG_XPD_PAD_GROUP** `[7:0]`
    ///
    /// Set 1 to power up pad group
    #[inline(always)]
    pub const fn ana_reg_xpd_pad_group(&self) -> u32 {
        (self.bits & AnaXpdPadGroup::ANA_REG_XPD_PAD_GROUP_MASK)
            >> AnaXpdPadGroup::ANA_REG_XPD_PAD_GROUP_OFFSET
    }

    /// Set **ANA_REG_XPD_PAD_GROUP** `[7:0]`
    ///
    /// Set 1 to power up pad group
    #[inline(always)]
    pub const fn with_ana_reg_xpd_pad_group(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !AnaXpdPadGroup::ANA_REG_XPD_PAD_GROUP_MASK)
                | ((value << AnaXpdPadGroup::ANA_REG_XPD_PAD_GROUP_OFFSET)
                    & AnaXpdPadGroup::ANA_REG_XPD_PAD_GROUP_MASK),
        }
    }
}

impl Default for AnaXpdPadGroupVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl AnaXpdPadGroup {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> AnaXpdPadGroupVal {
        AnaXpdPadGroupVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: AnaXpdPadGroupVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(AnaXpdPadGroupVal) -> AnaXpdPadGroupVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_TCM_RAM_RDN_ECO_CS Register
///
/// need_des
///
/// Address offset: 0x0110
///
/// ## Fields
///
/// - **LP_TCM_RAM_RDN_ECO_RESULT** `[1]` (RO)
///
///   need_des
///
/// - **LP_TCM_RAM_RDN_ECO_EN** `[0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpTcmRamRdnEcoCs {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpTcmRamRdnEcoCs {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpTcmRamRdnEcoCs {
    // ======== Field Constants ========

    /// **LP_TCM_RAM_RDN_ECO_RESULT**
    ///
    /// need_des
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const LP_TCM_RAM_RDN_ECO_RESULT_OFFSET: u32 = 1;
    pub const LP_TCM_RAM_RDN_ECO_RESULT_MASK: u32 = 0x00000002;
    pub const LP_TCM_RAM_RDN_ECO_RESULT: u32 = 1 << 1;

    /// **LP_TCM_RAM_RDN_ECO_EN**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_TCM_RAM_RDN_ECO_EN_OFFSET: u32 = 0;
    pub const LP_TCM_RAM_RDN_ECO_EN_MASK: u32 = 0x00000001;
    pub const LP_TCM_RAM_RDN_ECO_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LP_TCM_RAM_RDN_ECO_RESULT** `[1]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_tcm_ram_rdn_eco_result(&self) -> bool {
        (self.value.read_bits() & Self::LP_TCM_RAM_RDN_ECO_RESULT_MASK) != 0
    }

    /// **LP_TCM_RAM_RDN_ECO_EN** `[0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_tcm_ram_rdn_eco_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_TCM_RAM_RDN_ECO_EN_MASK) != 0
    }

    /// Set LP_TCM_RAM_RDN_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_tcm_ram_rdn_eco_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_TCM_RAM_RDN_ECO_EN_MASK);
    }

    /// Clear LP_TCM_RAM_RDN_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_tcm_ram_rdn_eco_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_TCM_RAM_RDN_ECO_EN_MASK);
    }
}

/// LP_TCM_RAM_RDN_ECO_CS value type for batch read/modify/write operations
///
/// Use `lptcmramrdnecocs.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lptcmramrdnecocs.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpTcmRamRdnEcoCsVal {
    bits: u32,
}

impl LpTcmRamRdnEcoCsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_TCM_RAM_RDN_ECO_RESULT** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_tcm_ram_rdn_eco_result(&self) -> bool {
        (self.bits & LpTcmRamRdnEcoCs::LP_TCM_RAM_RDN_ECO_RESULT_MASK) != 0
    }

    /// Read **LP_TCM_RAM_RDN_ECO_EN** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_tcm_ram_rdn_eco_en(&self) -> bool {
        (self.bits & LpTcmRamRdnEcoCs::LP_TCM_RAM_RDN_ECO_EN_MASK) != 0
    }

    /// Set **LP_TCM_RAM_RDN_ECO_EN** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_tcm_ram_rdn_eco_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpTcmRamRdnEcoCs::LP_TCM_RAM_RDN_ECO_EN_MASK
            } else {
                self.bits & !LpTcmRamRdnEcoCs::LP_TCM_RAM_RDN_ECO_EN_MASK
            },
        }
    }
}

impl Default for LpTcmRamRdnEcoCsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpTcmRamRdnEcoCs {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpTcmRamRdnEcoCsVal {
        LpTcmRamRdnEcoCsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpTcmRamRdnEcoCsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpTcmRamRdnEcoCsVal) -> LpTcmRamRdnEcoCsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_TCM_RAM_RDN_ECO_LOW Register
///
/// need_des
///
/// Address offset: 0x0114
///
/// ## Fields
///
/// - **LP_TCM_RAM_RDN_ECO_LOW** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpTcmRamRdnEcoLow {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpTcmRamRdnEcoLow {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpTcmRamRdnEcoLow {
    // ======== Field Constants ========

    /// **LP_TCM_RAM_RDN_ECO_LOW**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_TCM_RAM_RDN_ECO_LOW_OFFSET: u32 = 0;
    pub const LP_TCM_RAM_RDN_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_TCM_RAM_RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_tcm_ram_rdn_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::LP_TCM_RAM_RDN_ECO_LOW_MASK)
            >> Self::LP_TCM_RAM_RDN_ECO_LOW_OFFSET
    }

    /// Write **LP_TCM_RAM_RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_tcm_ram_rdn_eco_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_TCM_RAM_RDN_ECO_LOW_MASK)
                | ((value << Self::LP_TCM_RAM_RDN_ECO_LOW_OFFSET)
                    & Self::LP_TCM_RAM_RDN_ECO_LOW_MASK)
        });
    }
}

/// LP_TCM_RAM_RDN_ECO_LOW value type for batch read/modify/write operations
///
/// Use `lptcmramrdnecolow.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lptcmramrdnecolow.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpTcmRamRdnEcoLowVal {
    bits: u32,
}

impl LpTcmRamRdnEcoLowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_TCM_RAM_RDN_ECO_LOW** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_tcm_ram_rdn_eco_low(&self) -> u32 {
        (self.bits & LpTcmRamRdnEcoLow::LP_TCM_RAM_RDN_ECO_LOW_MASK)
            >> LpTcmRamRdnEcoLow::LP_TCM_RAM_RDN_ECO_LOW_OFFSET
    }

    /// Set **LP_TCM_RAM_RDN_ECO_LOW** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_tcm_ram_rdn_eco_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpTcmRamRdnEcoLow::LP_TCM_RAM_RDN_ECO_LOW_MASK)
                | ((value << LpTcmRamRdnEcoLow::LP_TCM_RAM_RDN_ECO_LOW_OFFSET)
                    & LpTcmRamRdnEcoLow::LP_TCM_RAM_RDN_ECO_LOW_MASK),
        }
    }
}

impl Default for LpTcmRamRdnEcoLowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpTcmRamRdnEcoLow {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpTcmRamRdnEcoLowVal {
        LpTcmRamRdnEcoLowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpTcmRamRdnEcoLowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpTcmRamRdnEcoLowVal) -> LpTcmRamRdnEcoLowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_TCM_RAM_RDN_ECO_HIGH Register
///
/// need_des
///
/// Address offset: 0x0118
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **LP_TCM_RAM_RDN_ECO_HIGH** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpTcmRamRdnEcoHigh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpTcmRamRdnEcoHigh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpTcmRamRdnEcoHigh {
    // ======== Field Constants ========

    /// **LP_TCM_RAM_RDN_ECO_HIGH**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_TCM_RAM_RDN_ECO_HIGH_OFFSET: u32 = 0;
    pub const LP_TCM_RAM_RDN_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_TCM_RAM_RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_tcm_ram_rdn_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::LP_TCM_RAM_RDN_ECO_HIGH_MASK)
            >> Self::LP_TCM_RAM_RDN_ECO_HIGH_OFFSET
    }

    /// Write **LP_TCM_RAM_RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_tcm_ram_rdn_eco_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_TCM_RAM_RDN_ECO_HIGH_MASK)
                | ((value << Self::LP_TCM_RAM_RDN_ECO_HIGH_OFFSET)
                    & Self::LP_TCM_RAM_RDN_ECO_HIGH_MASK)
        });
    }
}

/// LP_TCM_RAM_RDN_ECO_HIGH value type for batch read/modify/write operations
///
/// Use `lptcmramrdnecohigh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lptcmramrdnecohigh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpTcmRamRdnEcoHighVal {
    bits: u32,
}

impl LpTcmRamRdnEcoHighVal {
    /// Create from raw bits
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

    /// Read **LP_TCM_RAM_RDN_ECO_HIGH** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_tcm_ram_rdn_eco_high(&self) -> u32 {
        (self.bits & LpTcmRamRdnEcoHigh::LP_TCM_RAM_RDN_ECO_HIGH_MASK)
            >> LpTcmRamRdnEcoHigh::LP_TCM_RAM_RDN_ECO_HIGH_OFFSET
    }

    /// Set **LP_TCM_RAM_RDN_ECO_HIGH** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_tcm_ram_rdn_eco_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpTcmRamRdnEcoHigh::LP_TCM_RAM_RDN_ECO_HIGH_MASK)
                | ((value << LpTcmRamRdnEcoHigh::LP_TCM_RAM_RDN_ECO_HIGH_OFFSET)
                    & LpTcmRamRdnEcoHigh::LP_TCM_RAM_RDN_ECO_HIGH_MASK),
        }
    }
}

impl Default for LpTcmRamRdnEcoHighVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpTcmRamRdnEcoHigh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpTcmRamRdnEcoHighVal {
        LpTcmRamRdnEcoHighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpTcmRamRdnEcoHighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpTcmRamRdnEcoHighVal) -> LpTcmRamRdnEcoHighVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_TCM_ROM_RDN_ECO_CS Register
///
/// need_des
///
/// Address offset: 0x011c
///
/// ## Fields
///
/// - **LP_TCM_ROM_RDN_ECO_RESULT** `[1]` (RO)
///
///   need_des
///
/// - **LP_TCM_ROM_RDN_ECO_EN** `[0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpTcmRomRdnEcoCs {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpTcmRomRdnEcoCs {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpTcmRomRdnEcoCs {
    // ======== Field Constants ========

    /// **LP_TCM_ROM_RDN_ECO_RESULT**
    ///
    /// need_des
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const LP_TCM_ROM_RDN_ECO_RESULT_OFFSET: u32 = 1;
    pub const LP_TCM_ROM_RDN_ECO_RESULT_MASK: u32 = 0x00000002;
    pub const LP_TCM_ROM_RDN_ECO_RESULT: u32 = 1 << 1;

    /// **LP_TCM_ROM_RDN_ECO_EN**
    ///
    /// need_des
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_TCM_ROM_RDN_ECO_EN_OFFSET: u32 = 0;
    pub const LP_TCM_ROM_RDN_ECO_EN_MASK: u32 = 0x00000001;
    pub const LP_TCM_ROM_RDN_ECO_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LP_TCM_ROM_RDN_ECO_RESULT** `[1]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_tcm_rom_rdn_eco_result(&self) -> bool {
        (self.value.read_bits() & Self::LP_TCM_ROM_RDN_ECO_RESULT_MASK) != 0
    }

    /// **LP_TCM_ROM_RDN_ECO_EN** `[0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_tcm_rom_rdn_eco_en(&self) -> bool {
        (self.value.read_bits() & Self::LP_TCM_ROM_RDN_ECO_EN_MASK) != 0
    }

    /// Set LP_TCM_ROM_RDN_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_tcm_rom_rdn_eco_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_TCM_ROM_RDN_ECO_EN_MASK);
    }

    /// Clear LP_TCM_ROM_RDN_ECO_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_tcm_rom_rdn_eco_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_TCM_ROM_RDN_ECO_EN_MASK);
    }
}

/// LP_TCM_ROM_RDN_ECO_CS value type for batch read/modify/write operations
///
/// Use `lptcmromrdnecocs.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lptcmromrdnecocs.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpTcmRomRdnEcoCsVal {
    bits: u32,
}

impl LpTcmRomRdnEcoCsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_TCM_ROM_RDN_ECO_RESULT** `[1:1]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_tcm_rom_rdn_eco_result(&self) -> bool {
        (self.bits & LpTcmRomRdnEcoCs::LP_TCM_ROM_RDN_ECO_RESULT_MASK) != 0
    }

    /// Read **LP_TCM_ROM_RDN_ECO_EN** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_tcm_rom_rdn_eco_en(&self) -> bool {
        (self.bits & LpTcmRomRdnEcoCs::LP_TCM_ROM_RDN_ECO_EN_MASK) != 0
    }

    /// Set **LP_TCM_ROM_RDN_ECO_EN** `[0:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_tcm_rom_rdn_eco_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpTcmRomRdnEcoCs::LP_TCM_ROM_RDN_ECO_EN_MASK
            } else {
                self.bits & !LpTcmRomRdnEcoCs::LP_TCM_ROM_RDN_ECO_EN_MASK
            },
        }
    }
}

impl Default for LpTcmRomRdnEcoCsVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpTcmRomRdnEcoCs {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpTcmRomRdnEcoCsVal {
        LpTcmRomRdnEcoCsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpTcmRomRdnEcoCsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpTcmRomRdnEcoCsVal) -> LpTcmRomRdnEcoCsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_TCM_ROM_RDN_ECO_LOW Register
///
/// need_des
///
/// Address offset: 0x0120
///
/// ## Fields
///
/// - **LP_TCM_ROM_RDN_ECO_LOW** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpTcmRomRdnEcoLow {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpTcmRomRdnEcoLow {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpTcmRomRdnEcoLow {
    // ======== Field Constants ========

    /// **LP_TCM_ROM_RDN_ECO_LOW**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_TCM_ROM_RDN_ECO_LOW_OFFSET: u32 = 0;
    pub const LP_TCM_ROM_RDN_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_TCM_ROM_RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_tcm_rom_rdn_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::LP_TCM_ROM_RDN_ECO_LOW_MASK)
            >> Self::LP_TCM_ROM_RDN_ECO_LOW_OFFSET
    }

    /// Write **LP_TCM_ROM_RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_tcm_rom_rdn_eco_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_TCM_ROM_RDN_ECO_LOW_MASK)
                | ((value << Self::LP_TCM_ROM_RDN_ECO_LOW_OFFSET)
                    & Self::LP_TCM_ROM_RDN_ECO_LOW_MASK)
        });
    }
}

/// LP_TCM_ROM_RDN_ECO_LOW value type for batch read/modify/write operations
///
/// Use `lptcmromrdnecolow.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lptcmromrdnecolow.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpTcmRomRdnEcoLowVal {
    bits: u32,
}

impl LpTcmRomRdnEcoLowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_TCM_ROM_RDN_ECO_LOW** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_tcm_rom_rdn_eco_low(&self) -> u32 {
        (self.bits & LpTcmRomRdnEcoLow::LP_TCM_ROM_RDN_ECO_LOW_MASK)
            >> LpTcmRomRdnEcoLow::LP_TCM_ROM_RDN_ECO_LOW_OFFSET
    }

    /// Set **LP_TCM_ROM_RDN_ECO_LOW** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_tcm_rom_rdn_eco_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpTcmRomRdnEcoLow::LP_TCM_ROM_RDN_ECO_LOW_MASK)
                | ((value << LpTcmRomRdnEcoLow::LP_TCM_ROM_RDN_ECO_LOW_OFFSET)
                    & LpTcmRomRdnEcoLow::LP_TCM_ROM_RDN_ECO_LOW_MASK),
        }
    }
}

impl Default for LpTcmRomRdnEcoLowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpTcmRomRdnEcoLow {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpTcmRomRdnEcoLowVal {
        LpTcmRomRdnEcoLowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpTcmRomRdnEcoLowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpTcmRomRdnEcoLowVal) -> LpTcmRomRdnEcoLowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_TCM_ROM_RDN_ECO_HIGH Register
///
/// need_des
///
/// Address offset: 0x0124
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **LP_TCM_ROM_RDN_ECO_HIGH** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpTcmRomRdnEcoHigh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpTcmRomRdnEcoHigh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpTcmRomRdnEcoHigh {
    // ======== Field Constants ========

    /// **LP_TCM_ROM_RDN_ECO_HIGH**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_TCM_ROM_RDN_ECO_HIGH_OFFSET: u32 = 0;
    pub const LP_TCM_ROM_RDN_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_TCM_ROM_RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_tcm_rom_rdn_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::LP_TCM_ROM_RDN_ECO_HIGH_MASK)
            >> Self::LP_TCM_ROM_RDN_ECO_HIGH_OFFSET
    }

    /// Write **LP_TCM_ROM_RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_tcm_rom_rdn_eco_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_TCM_ROM_RDN_ECO_HIGH_MASK)
                | ((value << Self::LP_TCM_ROM_RDN_ECO_HIGH_OFFSET)
                    & Self::LP_TCM_ROM_RDN_ECO_HIGH_MASK)
        });
    }
}

/// LP_TCM_ROM_RDN_ECO_HIGH value type for batch read/modify/write operations
///
/// Use `lptcmromrdnecohigh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lptcmromrdnecohigh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpTcmRomRdnEcoHighVal {
    bits: u32,
}

impl LpTcmRomRdnEcoHighVal {
    /// Create from raw bits
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

    /// Read **LP_TCM_ROM_RDN_ECO_HIGH** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_tcm_rom_rdn_eco_high(&self) -> u32 {
        (self.bits & LpTcmRomRdnEcoHigh::LP_TCM_ROM_RDN_ECO_HIGH_MASK)
            >> LpTcmRomRdnEcoHigh::LP_TCM_ROM_RDN_ECO_HIGH_OFFSET
    }

    /// Set **LP_TCM_ROM_RDN_ECO_HIGH** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_tcm_rom_rdn_eco_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpTcmRomRdnEcoHigh::LP_TCM_ROM_RDN_ECO_HIGH_MASK)
                | ((value << LpTcmRomRdnEcoHigh::LP_TCM_ROM_RDN_ECO_HIGH_OFFSET)
                    & LpTcmRomRdnEcoHigh::LP_TCM_ROM_RDN_ECO_HIGH_MASK),
        }
    }
}

impl Default for LpTcmRomRdnEcoHighVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpTcmRomRdnEcoHigh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpTcmRomRdnEcoHighVal {
        LpTcmRomRdnEcoHighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpTcmRomRdnEcoHighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpTcmRomRdnEcoHighVal) -> LpTcmRomRdnEcoHighVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HP_ROOT_CLK_CTRL Register
///
/// need_des
///
/// Address offset: 0x0130
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **SYS_CLK_EN** `[1]` (RW)
///
///   clock gate enable for hp sys root 480M clk
///
/// - **CPU_CLK_EN** `[0]` (RW)
///
///   clock gate enable for hp cpu root 400M clk
#[repr(transparent)]
pub struct HpRootClkCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HpRootClkCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HpRootClkCtrl {
    // ======== Field Constants ========

    /// **SYS_CLK_EN**
    ///
    /// clock gate enable for hp sys root 480M clk
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SYS_CLK_EN_OFFSET: u32 = 1;
    pub const SYS_CLK_EN_MASK: u32 = 0x00000002;
    pub const SYS_CLK_EN: u32 = 1 << 1;

    /// **CPU_CLK_EN**
    ///
    /// clock gate enable for hp cpu root 400M clk
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const CPU_CLK_EN_OFFSET: u32 = 0;
    pub const CPU_CLK_EN_MASK: u32 = 0x00000001;
    pub const CPU_CLK_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SYS_CLK_EN** `[1]` (RW)
    ///
    /// clock gate enable for hp sys root 480M clk
    #[inline(always)]
    pub fn sys_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::SYS_CLK_EN_MASK) != 0
    }

    /// Set SYS_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sys_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::SYS_CLK_EN_MASK);
    }

    /// Clear SYS_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sys_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::SYS_CLK_EN_MASK);
    }

    /// **CPU_CLK_EN** `[0]` (RW)
    ///
    /// clock gate enable for hp cpu root 400M clk
    #[inline(always)]
    pub fn cpu_clk_en(&self) -> bool {
        (self.value.read_bits() & Self::CPU_CLK_EN_MASK) != 0
    }

    /// Set CPU_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_cpu_clk_en(&self) {
        self.value.modify_bits(|r| r | Self::CPU_CLK_EN_MASK);
    }

    /// Clear CPU_CLK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_cpu_clk_en(&self) {
        self.value.modify_bits(|r| r & !Self::CPU_CLK_EN_MASK);
    }
}

/// HP_ROOT_CLK_CTRL value type for batch read/modify/write operations
///
/// Use `hprootclkctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hprootclkctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HpRootClkCtrlVal {
    bits: u32,
}

impl HpRootClkCtrlVal {
    /// Create from raw bits
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

    /// Read **SYS_CLK_EN** `[1:1]`
    ///
    /// clock gate enable for hp sys root 480M clk
    #[inline(always)]
    pub const fn sys_clk_en(&self) -> bool {
        (self.bits & HpRootClkCtrl::SYS_CLK_EN_MASK) != 0
    }

    /// Read **CPU_CLK_EN** `[0:0]`
    ///
    /// clock gate enable for hp cpu root 400M clk
    #[inline(always)]
    pub const fn cpu_clk_en(&self) -> bool {
        (self.bits & HpRootClkCtrl::CPU_CLK_EN_MASK) != 0
    }

    /// Set **SYS_CLK_EN** `[1:1]`
    ///
    /// clock gate enable for hp sys root 480M clk
    #[inline(always)]
    pub const fn with_sys_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HpRootClkCtrl::SYS_CLK_EN_MASK
            } else {
                self.bits & !HpRootClkCtrl::SYS_CLK_EN_MASK
            },
        }
    }

    /// Set **CPU_CLK_EN** `[0:0]`
    ///
    /// clock gate enable for hp cpu root 400M clk
    #[inline(always)]
    pub const fn with_cpu_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | HpRootClkCtrl::CPU_CLK_EN_MASK
            } else {
                self.bits & !HpRootClkCtrl::CPU_CLK_EN_MASK
            },
        }
    }
}

impl Default for HpRootClkCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl HpRootClkCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HpRootClkCtrlVal {
        HpRootClkCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HpRootClkCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HpRootClkCtrlVal) -> HpRootClkCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_PMU_RDN_ECO_LOW Register
///
/// need_des
///
/// Address offset: 0x0138
///
/// ## Fields
///
/// - **PMU_RDN_ECO_LOW** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpPmuRdnEcoLow {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpPmuRdnEcoLow {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpPmuRdnEcoLow {
    // ======== Field Constants ========

    /// **PMU_RDN_ECO_LOW**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PMU_RDN_ECO_LOW_OFFSET: u32 = 0;
    pub const PMU_RDN_ECO_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PMU_RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn pmu_rdn_eco_low(&self) -> u32 {
        (self.value.read_bits() & Self::PMU_RDN_ECO_LOW_MASK) >> Self::PMU_RDN_ECO_LOW_OFFSET
    }

    /// Write **PMU_RDN_ECO_LOW** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_pmu_rdn_eco_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PMU_RDN_ECO_LOW_MASK)
                | ((value << Self::PMU_RDN_ECO_LOW_OFFSET) & Self::PMU_RDN_ECO_LOW_MASK)
        });
    }
}

/// LP_PMU_RDN_ECO_LOW value type for batch read/modify/write operations
///
/// Use `lppmurdnecolow.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lppmurdnecolow.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpPmuRdnEcoLowVal {
    bits: u32,
}

impl LpPmuRdnEcoLowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **PMU_RDN_ECO_LOW** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pmu_rdn_eco_low(&self) -> u32 {
        (self.bits & LpPmuRdnEcoLow::PMU_RDN_ECO_LOW_MASK) >> LpPmuRdnEcoLow::PMU_RDN_ECO_LOW_OFFSET
    }

    /// Set **PMU_RDN_ECO_LOW** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_pmu_rdn_eco_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpPmuRdnEcoLow::PMU_RDN_ECO_LOW_MASK)
                | ((value << LpPmuRdnEcoLow::PMU_RDN_ECO_LOW_OFFSET)
                    & LpPmuRdnEcoLow::PMU_RDN_ECO_LOW_MASK),
        }
    }
}

impl Default for LpPmuRdnEcoLowVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpPmuRdnEcoLow {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpPmuRdnEcoLowVal {
        LpPmuRdnEcoLowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpPmuRdnEcoLowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpPmuRdnEcoLowVal) -> LpPmuRdnEcoLowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_PMU_RDN_ECO_HIGH Register
///
/// need_des
///
/// Address offset: 0x013c
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **PMU_RDN_ECO_HIGH** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpPmuRdnEcoHigh {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpPmuRdnEcoHigh {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpPmuRdnEcoHigh {
    // ======== Field Constants ========

    /// **PMU_RDN_ECO_HIGH**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PMU_RDN_ECO_HIGH_OFFSET: u32 = 0;
    pub const PMU_RDN_ECO_HIGH_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PMU_RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn pmu_rdn_eco_high(&self) -> u32 {
        (self.value.read_bits() & Self::PMU_RDN_ECO_HIGH_MASK) >> Self::PMU_RDN_ECO_HIGH_OFFSET
    }

    /// Write **PMU_RDN_ECO_HIGH** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_pmu_rdn_eco_high(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PMU_RDN_ECO_HIGH_MASK)
                | ((value << Self::PMU_RDN_ECO_HIGH_OFFSET) & Self::PMU_RDN_ECO_HIGH_MASK)
        });
    }
}

/// LP_PMU_RDN_ECO_HIGH value type for batch read/modify/write operations
///
/// Use `lppmurdnecohigh.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lppmurdnecohigh.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpPmuRdnEcoHighVal {
    bits: u32,
}

impl LpPmuRdnEcoHighVal {
    /// Create from raw bits
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

    /// Read **PMU_RDN_ECO_HIGH** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn pmu_rdn_eco_high(&self) -> u32 {
        (self.bits & LpPmuRdnEcoHigh::PMU_RDN_ECO_HIGH_MASK)
            >> LpPmuRdnEcoHigh::PMU_RDN_ECO_HIGH_OFFSET
    }

    /// Set **PMU_RDN_ECO_HIGH** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_pmu_rdn_eco_high(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpPmuRdnEcoHigh::PMU_RDN_ECO_HIGH_MASK)
                | ((value << LpPmuRdnEcoHigh::PMU_RDN_ECO_HIGH_OFFSET)
                    & LpPmuRdnEcoHigh::PMU_RDN_ECO_HIGH_MASK),
        }
    }
}

impl Default for LpPmuRdnEcoHighVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpPmuRdnEcoHigh {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpPmuRdnEcoHighVal {
        LpPmuRdnEcoHighVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpPmuRdnEcoHighVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpPmuRdnEcoHighVal) -> LpPmuRdnEcoHighVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD_COMP0 Register
///
/// need_des
///
/// Address offset: 0x0148
///
/// ## Fields
///
/// - **XPD_COMP0** `[4]` (RW)
///
///   pad comp xpd
///
/// - **MODE_COMP0** `[3]` (RW)
///
///   pad comp mode
///
/// - **DREF_COMP0** `[2:0]` (RW)
///
///   pad comp dref
#[repr(transparent)]
pub struct PadComp0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PadComp0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PadComp0 {
    // ======== Field Constants ========

    /// **XPD_COMP0**
    ///
    /// pad comp xpd
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const XPD_COMP0_OFFSET: u32 = 4;
    pub const XPD_COMP0_MASK: u32 = 0x00000010;
    pub const XPD_COMP0: u32 = 1 << 4;

    /// **MODE_COMP0**
    ///
    /// pad comp mode
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MODE_COMP0_OFFSET: u32 = 3;
    pub const MODE_COMP0_MASK: u32 = 0x00000008;
    pub const MODE_COMP0: u32 = 1 << 3;

    /// **DREF_COMP0**
    ///
    /// pad comp dref
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const DREF_COMP0_OFFSET: u32 = 0;
    pub const DREF_COMP0_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **XPD_COMP0** `[4]` (RW)
    ///
    /// pad comp xpd
    #[inline(always)]
    pub fn xpd_comp0(&self) -> bool {
        (self.value.read_bits() & Self::XPD_COMP0_MASK) != 0
    }

    /// Set XPD_COMP0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_xpd_comp0(&self) {
        self.value.modify_bits(|r| r | Self::XPD_COMP0_MASK);
    }

    /// Clear XPD_COMP0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_xpd_comp0(&self) {
        self.value.modify_bits(|r| r & !Self::XPD_COMP0_MASK);
    }

    /// **MODE_COMP0** `[3]` (RW)
    ///
    /// pad comp mode
    #[inline(always)]
    pub fn mode_comp0(&self) -> bool {
        (self.value.read_bits() & Self::MODE_COMP0_MASK) != 0
    }

    /// Set MODE_COMP0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mode_comp0(&self) {
        self.value.modify_bits(|r| r | Self::MODE_COMP0_MASK);
    }

    /// Clear MODE_COMP0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mode_comp0(&self) {
        self.value.modify_bits(|r| r & !Self::MODE_COMP0_MASK);
    }

    /// **DREF_COMP0** `[2:0]` (RW)
    ///
    /// pad comp dref
    #[inline(always)]
    pub fn dref_comp0(&self) -> u32 {
        (self.value.read_bits() & Self::DREF_COMP0_MASK) >> Self::DREF_COMP0_OFFSET
    }

    /// Write **DREF_COMP0** `[2:0]` (RW)
    ///
    /// pad comp dref
    #[inline(always)]
    pub fn write_dref_comp0(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DREF_COMP0_MASK)
                | ((value << Self::DREF_COMP0_OFFSET) & Self::DREF_COMP0_MASK)
        });
    }
}

/// PAD_COMP0 value type for batch read/modify/write operations
///
/// Use `padcomp0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `padcomp0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PadComp0Val {
    bits: u32,
}

impl PadComp0Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **XPD_COMP0** `[4:4]`
    ///
    /// pad comp xpd
    #[inline(always)]
    pub const fn xpd_comp0(&self) -> bool {
        (self.bits & PadComp0::XPD_COMP0_MASK) != 0
    }

    /// Read **MODE_COMP0** `[3:3]`
    ///
    /// pad comp mode
    #[inline(always)]
    pub const fn mode_comp0(&self) -> bool {
        (self.bits & PadComp0::MODE_COMP0_MASK) != 0
    }

    /// Read **DREF_COMP0** `[2:0]`
    ///
    /// pad comp dref
    #[inline(always)]
    pub const fn dref_comp0(&self) -> u32 {
        (self.bits & PadComp0::DREF_COMP0_MASK) >> PadComp0::DREF_COMP0_OFFSET
    }

    /// Set **XPD_COMP0** `[4:4]`
    ///
    /// pad comp xpd
    #[inline(always)]
    pub const fn with_xpd_comp0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadComp0::XPD_COMP0_MASK
            } else {
                self.bits & !PadComp0::XPD_COMP0_MASK
            },
        }
    }

    /// Set **MODE_COMP0** `[3:3]`
    ///
    /// pad comp mode
    #[inline(always)]
    pub const fn with_mode_comp0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadComp0::MODE_COMP0_MASK
            } else {
                self.bits & !PadComp0::MODE_COMP0_MASK
            },
        }
    }

    /// Set **DREF_COMP0** `[2:0]`
    ///
    /// pad comp dref
    #[inline(always)]
    pub const fn with_dref_comp0(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PadComp0::DREF_COMP0_MASK)
                | ((value << PadComp0::DREF_COMP0_OFFSET) & PadComp0::DREF_COMP0_MASK),
        }
    }
}

impl Default for PadComp0Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PadComp0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PadComp0Val {
        PadComp0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PadComp0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PadComp0Val) -> PadComp0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PAD_COMP1 Register
///
/// need_des
///
/// Address offset: 0x014c
///
/// ## Fields
///
/// - **XPD_COMP1** `[4]` (RW)
///
///   pad comp xpd
///
/// - **MODE_COMP1** `[3]` (RW)
///
///   pad comp mode
///
/// - **DREF_COMP1** `[2:0]` (RW)
///
///   pad comp dref
#[repr(transparent)]
pub struct PadComp1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PadComp1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PadComp1 {
    // ======== Field Constants ========

    /// **XPD_COMP1**
    ///
    /// pad comp xpd
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const XPD_COMP1_OFFSET: u32 = 4;
    pub const XPD_COMP1_MASK: u32 = 0x00000010;
    pub const XPD_COMP1: u32 = 1 << 4;

    /// **MODE_COMP1**
    ///
    /// pad comp mode
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const MODE_COMP1_OFFSET: u32 = 3;
    pub const MODE_COMP1_MASK: u32 = 0x00000008;
    pub const MODE_COMP1: u32 = 1 << 3;

    /// **DREF_COMP1**
    ///
    /// pad comp dref
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const DREF_COMP1_OFFSET: u32 = 0;
    pub const DREF_COMP1_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **XPD_COMP1** `[4]` (RW)
    ///
    /// pad comp xpd
    #[inline(always)]
    pub fn xpd_comp1(&self) -> bool {
        (self.value.read_bits() & Self::XPD_COMP1_MASK) != 0
    }

    /// Set XPD_COMP1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_xpd_comp1(&self) {
        self.value.modify_bits(|r| r | Self::XPD_COMP1_MASK);
    }

    /// Clear XPD_COMP1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_xpd_comp1(&self) {
        self.value.modify_bits(|r| r & !Self::XPD_COMP1_MASK);
    }

    /// **MODE_COMP1** `[3]` (RW)
    ///
    /// pad comp mode
    #[inline(always)]
    pub fn mode_comp1(&self) -> bool {
        (self.value.read_bits() & Self::MODE_COMP1_MASK) != 0
    }

    /// Set MODE_COMP1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mode_comp1(&self) {
        self.value.modify_bits(|r| r | Self::MODE_COMP1_MASK);
    }

    /// Clear MODE_COMP1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mode_comp1(&self) {
        self.value.modify_bits(|r| r & !Self::MODE_COMP1_MASK);
    }

    /// **DREF_COMP1** `[2:0]` (RW)
    ///
    /// pad comp dref
    #[inline(always)]
    pub fn dref_comp1(&self) -> u32 {
        (self.value.read_bits() & Self::DREF_COMP1_MASK) >> Self::DREF_COMP1_OFFSET
    }

    /// Write **DREF_COMP1** `[2:0]` (RW)
    ///
    /// pad comp dref
    #[inline(always)]
    pub fn write_dref_comp1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DREF_COMP1_MASK)
                | ((value << Self::DREF_COMP1_OFFSET) & Self::DREF_COMP1_MASK)
        });
    }
}

/// PAD_COMP1 value type for batch read/modify/write operations
///
/// Use `padcomp1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `padcomp1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PadComp1Val {
    bits: u32,
}

impl PadComp1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **XPD_COMP1** `[4:4]`
    ///
    /// pad comp xpd
    #[inline(always)]
    pub const fn xpd_comp1(&self) -> bool {
        (self.bits & PadComp1::XPD_COMP1_MASK) != 0
    }

    /// Read **MODE_COMP1** `[3:3]`
    ///
    /// pad comp mode
    #[inline(always)]
    pub const fn mode_comp1(&self) -> bool {
        (self.bits & PadComp1::MODE_COMP1_MASK) != 0
    }

    /// Read **DREF_COMP1** `[2:0]`
    ///
    /// pad comp dref
    #[inline(always)]
    pub const fn dref_comp1(&self) -> u32 {
        (self.bits & PadComp1::DREF_COMP1_MASK) >> PadComp1::DREF_COMP1_OFFSET
    }

    /// Set **XPD_COMP1** `[4:4]`
    ///
    /// pad comp xpd
    #[inline(always)]
    pub const fn with_xpd_comp1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadComp1::XPD_COMP1_MASK
            } else {
                self.bits & !PadComp1::XPD_COMP1_MASK
            },
        }
    }

    /// Set **MODE_COMP1** `[3:3]`
    ///
    /// pad comp mode
    #[inline(always)]
    pub const fn with_mode_comp1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PadComp1::MODE_COMP1_MASK
            } else {
                self.bits & !PadComp1::MODE_COMP1_MASK
            },
        }
    }

    /// Set **DREF_COMP1** `[2:0]`
    ///
    /// pad comp dref
    #[inline(always)]
    pub const fn with_dref_comp1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PadComp1::DREF_COMP1_MASK)
                | ((value << PadComp1::DREF_COMP1_OFFSET) & PadComp1::DREF_COMP1_MASK),
        }
    }
}

impl Default for PadComp1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl PadComp1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PadComp1Val {
        PadComp1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PadComp1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PadComp1Val) -> PadComp1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BACKUP_DMA_CFG0 Register
///
/// need_des
///
/// Address offset: 0x0154
/// Reset value: 0x1906414a
///
/// ## Fields
///
/// - **LINK_TOUT_THRES_AON** `[31:22]` (RW)
///
///   need_des
///
/// - **LINK_BACKUP_TOUT_THRES_AON** `[21:12]` (RW)
///
///   need_des
///
/// - **READ_INTERVAL_AON** `[11:5]` (RW)
///
///   need_des
///
/// - **BURST_LIMIT_AON** `[4:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct BackupDmaCfg0 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BackupDmaCfg0 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BackupDmaCfg0 {
    // ======== Field Constants ========

    /// **LINK_TOUT_THRES_AON**
    ///
    /// need_des
    ///
    /// Bits `[31:22]`, mask `0xffc00000`
    pub const LINK_TOUT_THRES_AON_OFFSET: u32 = 22;
    pub const LINK_TOUT_THRES_AON_MASK: u32 = 0xffc00000;

    /// **LINK_BACKUP_TOUT_THRES_AON**
    ///
    /// need_des
    ///
    /// Bits `[21:12]`, mask `0x003ff000`
    pub const LINK_BACKUP_TOUT_THRES_AON_OFFSET: u32 = 12;
    pub const LINK_BACKUP_TOUT_THRES_AON_MASK: u32 = 0x003ff000;

    /// **READ_INTERVAL_AON**
    ///
    /// need_des
    ///
    /// Bits `[11:5]`, mask `0x00000fe0`
    pub const READ_INTERVAL_AON_OFFSET: u32 = 5;
    pub const READ_INTERVAL_AON_MASK: u32 = 0x00000fe0;

    /// **BURST_LIMIT_AON**
    ///
    /// need_des
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const BURST_LIMIT_AON_OFFSET: u32 = 0;
    pub const BURST_LIMIT_AON_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **LINK_TOUT_THRES_AON** `[31:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn link_tout_thres_aon(&self) -> u32 {
        (self.value.read_bits() & Self::LINK_TOUT_THRES_AON_MASK)
            >> Self::LINK_TOUT_THRES_AON_OFFSET
    }

    /// Write **LINK_TOUT_THRES_AON** `[31:22]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_link_tout_thres_aon(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LINK_TOUT_THRES_AON_MASK)
                | ((value << Self::LINK_TOUT_THRES_AON_OFFSET) & Self::LINK_TOUT_THRES_AON_MASK)
        });
    }

    /// **LINK_BACKUP_TOUT_THRES_AON** `[21:12]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn link_backup_tout_thres_aon(&self) -> u32 {
        (self.value.read_bits() & Self::LINK_BACKUP_TOUT_THRES_AON_MASK)
            >> Self::LINK_BACKUP_TOUT_THRES_AON_OFFSET
    }

    /// Write **LINK_BACKUP_TOUT_THRES_AON** `[21:12]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_link_backup_tout_thres_aon(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LINK_BACKUP_TOUT_THRES_AON_MASK)
                | ((value << Self::LINK_BACKUP_TOUT_THRES_AON_OFFSET)
                    & Self::LINK_BACKUP_TOUT_THRES_AON_MASK)
        });
    }

    /// **READ_INTERVAL_AON** `[11:5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn read_interval_aon(&self) -> u32 {
        (self.value.read_bits() & Self::READ_INTERVAL_AON_MASK) >> Self::READ_INTERVAL_AON_OFFSET
    }

    /// Write **READ_INTERVAL_AON** `[11:5]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_read_interval_aon(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::READ_INTERVAL_AON_MASK)
                | ((value << Self::READ_INTERVAL_AON_OFFSET) & Self::READ_INTERVAL_AON_MASK)
        });
    }

    /// **BURST_LIMIT_AON** `[4:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn burst_limit_aon(&self) -> u32 {
        (self.value.read_bits() & Self::BURST_LIMIT_AON_MASK) >> Self::BURST_LIMIT_AON_OFFSET
    }

    /// Write **BURST_LIMIT_AON** `[4:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_burst_limit_aon(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BURST_LIMIT_AON_MASK)
                | ((value << Self::BURST_LIMIT_AON_OFFSET) & Self::BURST_LIMIT_AON_MASK)
        });
    }
}

/// BACKUP_DMA_CFG0 value type for batch read/modify/write operations
///
/// Use `backupdmacfg0.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `backupdmacfg0.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BackupDmaCfg0Val {
    bits: u32,
}

impl BackupDmaCfg0Val {
    /// Create from raw bits
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
        Self { bits: 0x1906414a }
    }

    /// Read **LINK_TOUT_THRES_AON** `[31:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn link_tout_thres_aon(&self) -> u32 {
        (self.bits & BackupDmaCfg0::LINK_TOUT_THRES_AON_MASK)
            >> BackupDmaCfg0::LINK_TOUT_THRES_AON_OFFSET
    }

    /// Read **LINK_BACKUP_TOUT_THRES_AON** `[21:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn link_backup_tout_thres_aon(&self) -> u32 {
        (self.bits & BackupDmaCfg0::LINK_BACKUP_TOUT_THRES_AON_MASK)
            >> BackupDmaCfg0::LINK_BACKUP_TOUT_THRES_AON_OFFSET
    }

    /// Read **READ_INTERVAL_AON** `[11:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn read_interval_aon(&self) -> u32 {
        (self.bits & BackupDmaCfg0::READ_INTERVAL_AON_MASK)
            >> BackupDmaCfg0::READ_INTERVAL_AON_OFFSET
    }

    /// Read **BURST_LIMIT_AON** `[4:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn burst_limit_aon(&self) -> u32 {
        (self.bits & BackupDmaCfg0::BURST_LIMIT_AON_MASK) >> BackupDmaCfg0::BURST_LIMIT_AON_OFFSET
    }

    /// Set **LINK_TOUT_THRES_AON** `[31:22]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_link_tout_thres_aon(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BackupDmaCfg0::LINK_TOUT_THRES_AON_MASK)
                | ((value << BackupDmaCfg0::LINK_TOUT_THRES_AON_OFFSET)
                    & BackupDmaCfg0::LINK_TOUT_THRES_AON_MASK),
        }
    }

    /// Set **LINK_BACKUP_TOUT_THRES_AON** `[21:12]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_link_backup_tout_thres_aon(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BackupDmaCfg0::LINK_BACKUP_TOUT_THRES_AON_MASK)
                | ((value << BackupDmaCfg0::LINK_BACKUP_TOUT_THRES_AON_OFFSET)
                    & BackupDmaCfg0::LINK_BACKUP_TOUT_THRES_AON_MASK),
        }
    }

    /// Set **READ_INTERVAL_AON** `[11:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_read_interval_aon(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BackupDmaCfg0::READ_INTERVAL_AON_MASK)
                | ((value << BackupDmaCfg0::READ_INTERVAL_AON_OFFSET)
                    & BackupDmaCfg0::READ_INTERVAL_AON_MASK),
        }
    }

    /// Set **BURST_LIMIT_AON** `[4:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_burst_limit_aon(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BackupDmaCfg0::BURST_LIMIT_AON_MASK)
                | ((value << BackupDmaCfg0::BURST_LIMIT_AON_OFFSET)
                    & BackupDmaCfg0::BURST_LIMIT_AON_MASK),
        }
    }
}

impl Default for BackupDmaCfg0Val {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl BackupDmaCfg0 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BackupDmaCfg0Val {
        BackupDmaCfg0Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BackupDmaCfg0Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BackupDmaCfg0Val) -> BackupDmaCfg0Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BACKUP_DMA_CFG1 Register
///
/// need_des
///
/// Address offset: 0x0158
///
/// ## Fields
///
/// - **AON_BYPASS** `[31]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct BackupDmaCfg1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BackupDmaCfg1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BackupDmaCfg1 {
    // ======== Field Constants ========

    /// **AON_BYPASS**
    ///
    /// need_des
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const AON_BYPASS_OFFSET: u32 = 31;
    pub const AON_BYPASS_MASK: u32 = 0x80000000;
    pub const AON_BYPASS: u32 = 1 << 31;

    // ======== Field Accessors ========

    /// **AON_BYPASS** `[31]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn aon_bypass(&self) -> bool {
        (self.value.read_bits() & Self::AON_BYPASS_MASK) != 0
    }

    /// Set AON_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_aon_bypass(&self) {
        self.value.modify_bits(|r| r | Self::AON_BYPASS_MASK);
    }

    /// Clear AON_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_aon_bypass(&self) {
        self.value.modify_bits(|r| r & !Self::AON_BYPASS_MASK);
    }
}

/// BACKUP_DMA_CFG1 value type for batch read/modify/write operations
///
/// Use `backupdmacfg1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `backupdmacfg1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BackupDmaCfg1Val {
    bits: u32,
}

impl BackupDmaCfg1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **AON_BYPASS** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn aon_bypass(&self) -> bool {
        (self.bits & BackupDmaCfg1::AON_BYPASS_MASK) != 0
    }

    /// Set **AON_BYPASS** `[31:31]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_aon_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | BackupDmaCfg1::AON_BYPASS_MASK
            } else {
                self.bits & !BackupDmaCfg1::AON_BYPASS_MASK
            },
        }
    }
}

impl Default for BackupDmaCfg1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BackupDmaCfg1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BackupDmaCfg1Val {
        BackupDmaCfg1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BackupDmaCfg1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BackupDmaCfg1Val) -> BackupDmaCfg1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BACKUP_DMA_CFG2 Register
///
/// need_des
///
/// Address offset: 0x015c
///
/// ## Fields
///
/// - **LINK_ADDR_AON** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct BackupDmaCfg2 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BackupDmaCfg2 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BackupDmaCfg2 {
    // ======== Field Constants ========

    /// **LINK_ADDR_AON**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LINK_ADDR_AON_OFFSET: u32 = 0;
    pub const LINK_ADDR_AON_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LINK_ADDR_AON** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn link_addr_aon(&self) -> u32 {
        (self.value.read_bits() & Self::LINK_ADDR_AON_MASK) >> Self::LINK_ADDR_AON_OFFSET
    }

    /// Write **LINK_ADDR_AON** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_link_addr_aon(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LINK_ADDR_AON_MASK)
                | ((value << Self::LINK_ADDR_AON_OFFSET) & Self::LINK_ADDR_AON_MASK)
        });
    }
}

/// BACKUP_DMA_CFG2 value type for batch read/modify/write operations
///
/// Use `backupdmacfg2.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `backupdmacfg2.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BackupDmaCfg2Val {
    bits: u32,
}

impl BackupDmaCfg2Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LINK_ADDR_AON** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn link_addr_aon(&self) -> u32 {
        (self.bits & BackupDmaCfg2::LINK_ADDR_AON_MASK) >> BackupDmaCfg2::LINK_ADDR_AON_OFFSET
    }

    /// Set **LINK_ADDR_AON** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_link_addr_aon(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BackupDmaCfg2::LINK_ADDR_AON_MASK)
                | ((value << BackupDmaCfg2::LINK_ADDR_AON_OFFSET)
                    & BackupDmaCfg2::LINK_ADDR_AON_MASK),
        }
    }
}

impl Default for BackupDmaCfg2Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BackupDmaCfg2 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BackupDmaCfg2Val {
        BackupDmaCfg2Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BackupDmaCfg2Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BackupDmaCfg2Val) -> BackupDmaCfg2Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// BOOT_ADDR_HP_CORE1 Register
///
/// need_des
///
/// Address offset: 0x0164
///
/// ## Fields
///
/// - **BOOT_ADDR_HP_CORE1** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct BootAddrHpCore1 {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for BootAddrHpCore1 {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl BootAddrHpCore1 {
    // ======== Field Constants ========

    /// **BOOT_ADDR_HP_CORE1**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const BOOT_ADDR_HP_CORE1_OFFSET: u32 = 0;
    pub const BOOT_ADDR_HP_CORE1_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **BOOT_ADDR_HP_CORE1** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn boot_addr_hp_core1(&self) -> u32 {
        (self.value.read_bits() & Self::BOOT_ADDR_HP_CORE1_MASK) >> Self::BOOT_ADDR_HP_CORE1_OFFSET
    }

    /// Write **BOOT_ADDR_HP_CORE1** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_boot_addr_hp_core1(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::BOOT_ADDR_HP_CORE1_MASK)
                | ((value << Self::BOOT_ADDR_HP_CORE1_OFFSET) & Self::BOOT_ADDR_HP_CORE1_MASK)
        });
    }
}

/// BOOT_ADDR_HP_CORE1 value type for batch read/modify/write operations
///
/// Use `bootaddrhpcore1.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `bootaddrhpcore1.write_val(val)`.
#[derive(Clone, Copy)]
pub struct BootAddrHpCore1Val {
    bits: u32,
}

impl BootAddrHpCore1Val {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **BOOT_ADDR_HP_CORE1** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn boot_addr_hp_core1(&self) -> u32 {
        (self.bits & BootAddrHpCore1::BOOT_ADDR_HP_CORE1_MASK)
            >> BootAddrHpCore1::BOOT_ADDR_HP_CORE1_OFFSET
    }

    /// Set **BOOT_ADDR_HP_CORE1** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_boot_addr_hp_core1(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !BootAddrHpCore1::BOOT_ADDR_HP_CORE1_MASK)
                | ((value << BootAddrHpCore1::BOOT_ADDR_HP_CORE1_OFFSET)
                    & BootAddrHpCore1::BOOT_ADDR_HP_CORE1_MASK),
        }
    }
}

impl Default for BootAddrHpCore1Val {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl BootAddrHpCore1 {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> BootAddrHpCore1Val {
        BootAddrHpCore1Val::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: BootAddrHpCore1Val) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(BootAddrHpCore1Val) -> BootAddrHpCore1Val>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ADDRHOLE_ADDR Register
///
/// need_des
///
/// Address offset: 0x0168
///
/// ## Fields
///
/// - **LP_ADDRHOLE_ADDR** `[31:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct LpAddrholeAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAddrholeAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAddrholeAddr {
    // ======== Field Constants ========

    /// **LP_ADDRHOLE_ADDR**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_ADDRHOLE_ADDR_OFFSET: u32 = 0;
    pub const LP_ADDRHOLE_ADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_ADDRHOLE_ADDR** `[31:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_addrhole_addr(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ADDRHOLE_ADDR_MASK) >> Self::LP_ADDRHOLE_ADDR_OFFSET
    }
}

/// LP_ADDRHOLE_ADDR value type for batch read/modify/write operations
///
/// Use `lpaddrholeaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaddrholeaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAddrholeAddrVal {
    bits: u32,
}

impl LpAddrholeAddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ADDRHOLE_ADDR** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_addrhole_addr(&self) -> u32 {
        (self.bits & LpAddrholeAddr::LP_ADDRHOLE_ADDR_MASK)
            >> LpAddrholeAddr::LP_ADDRHOLE_ADDR_OFFSET
    }
}

impl Default for LpAddrholeAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAddrholeAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAddrholeAddrVal {
        LpAddrholeAddrVal::from_bits(self.value.read_bits())
    }
}

/// LP_ADDRHOLE_INFO Register
///
/// need_des
///
/// Address offset: 0x016c
///
/// ## Fields
///
/// - **LP_ADDRHOLE_SECURE** `[6]` (RO)
///
///   1: illegal address access, 0: access without permission
///
/// - **LP_ADDRHOLE_WR** `[5]` (RO)
///
///   1:write trans, 0: read trans.
///
/// - **LP_ADDRHOLE_ID** `[4:0]` (RO)
///
///   master id: 5'h0: hp core0, 5'h1:hp core1, 5'h2:lp core, 5'h3:usb otg11, 5'h4: regdma, 5'h5: gmac, 5'h5 sdmmc, 5'h7: usbotg20, 5'h8: trace0, 5'h9: trace1, 5'ha tcm monitor, 5'hb: l2mem monitor. 5'h10~5'h1f: ahb pdma.
#[repr(transparent)]
pub struct LpAddrholeInfo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpAddrholeInfo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpAddrholeInfo {
    // ======== Field Constants ========

    /// **LP_ADDRHOLE_SECURE**
    ///
    /// 1: illegal address access, 0: access without permission
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const LP_ADDRHOLE_SECURE_OFFSET: u32 = 6;
    pub const LP_ADDRHOLE_SECURE_MASK: u32 = 0x00000040;
    pub const LP_ADDRHOLE_SECURE: u32 = 1 << 6;

    /// **LP_ADDRHOLE_WR**
    ///
    /// 1:write trans, 0: read trans.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const LP_ADDRHOLE_WR_OFFSET: u32 = 5;
    pub const LP_ADDRHOLE_WR_MASK: u32 = 0x00000020;
    pub const LP_ADDRHOLE_WR: u32 = 1 << 5;

    /// **LP_ADDRHOLE_ID**
    ///
    /// master id: 5'h0: hp core0, 5'h1:hp core1, 5'h2:lp core, 5'h3:usb otg11, 5'h4: regdma, 5'h5: gmac, 5'h5 sdmmc, 5'h7: usbotg20, 5'h8: trace0, 5'h9: trace1, 5'ha tcm monitor, 5'hb: l2mem monitor. 5'h10~5'h1f: ahb pdma.
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const LP_ADDRHOLE_ID_OFFSET: u32 = 0;
    pub const LP_ADDRHOLE_ID_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **LP_ADDRHOLE_SECURE** `[6]` (RO)
    ///
    /// 1: illegal address access, 0: access without permission
    #[inline(always)]
    pub fn lp_addrhole_secure(&self) -> bool {
        (self.value.read_bits() & Self::LP_ADDRHOLE_SECURE_MASK) != 0
    }

    /// **LP_ADDRHOLE_WR** `[5]` (RO)
    ///
    /// 1:write trans, 0: read trans.
    #[inline(always)]
    pub fn lp_addrhole_wr(&self) -> bool {
        (self.value.read_bits() & Self::LP_ADDRHOLE_WR_MASK) != 0
    }

    /// **LP_ADDRHOLE_ID** `[4:0]` (RO)
    ///
    /// master id: 5'h0: hp core0, 5'h1:hp core1, 5'h2:lp core, 5'h3:usb otg11, 5'h4: regdma, 5'h5: gmac, 5'h5 sdmmc, 5'h7: usbotg20, 5'h8: trace0, 5'h9: trace1, 5'ha tcm monitor, 5'hb: l2mem monitor. 5'h10~5'h1f: ahb pdma.
    #[inline(always)]
    pub fn lp_addrhole_id(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ADDRHOLE_ID_MASK) >> Self::LP_ADDRHOLE_ID_OFFSET
    }
}

/// LP_ADDRHOLE_INFO value type for batch read/modify/write operations
///
/// Use `lpaddrholeinfo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpaddrholeinfo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpAddrholeInfoVal {
    bits: u32,
}

impl LpAddrholeInfoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_ADDRHOLE_SECURE** `[6:6]`
    ///
    /// 1: illegal address access, 0: access without permission
    #[inline(always)]
    pub const fn lp_addrhole_secure(&self) -> bool {
        (self.bits & LpAddrholeInfo::LP_ADDRHOLE_SECURE_MASK) != 0
    }

    /// Read **LP_ADDRHOLE_WR** `[5:5]`
    ///
    /// 1:write trans, 0: read trans.
    #[inline(always)]
    pub const fn lp_addrhole_wr(&self) -> bool {
        (self.bits & LpAddrholeInfo::LP_ADDRHOLE_WR_MASK) != 0
    }

    /// Read **LP_ADDRHOLE_ID** `[4:0]`
    ///
    /// master id: 5'h0: hp core0, 5'h1:hp core1, 5'h2:lp core, 5'h3:usb otg11, 5'h4: regdma, 5'h5: gmac, 5'h5 sdmmc, 5'h7: usbotg20, 5'h8: trace0, 5'h9: trace1, 5'ha tcm monitor, 5'hb: l2mem monitor. 5'h10~5'h1f: ahb pdma.
    #[inline(always)]
    pub const fn lp_addrhole_id(&self) -> u32 {
        (self.bits & LpAddrholeInfo::LP_ADDRHOLE_ID_MASK) >> LpAddrholeInfo::LP_ADDRHOLE_ID_OFFSET
    }
}

impl Default for LpAddrholeInfoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpAddrholeInfo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpAddrholeInfoVal {
        LpAddrholeInfoVal::from_bits(self.value.read_bits())
    }
}

/// INT_RAW Register
///
/// raw interrupt register
///
/// Address offset: 0x0170
///
/// ## Fields
///
/// - **SLOW_CLK_TICK_INT_RAW** `[6]` (RO)
///
///   the raw interrupt status of slow_clk_tick
///
/// - **ETM_TASK_ULP_INT_RAW** `[5]` (RO)
///
///   the raw interrupt status of etm task ulp
///
/// - **LP_CORE_DBUS_TIMEOUT_INT_RAW** `[4]` (RO)
///
///   the raw interrupt status of lp core dbus timeout
///
/// - **LP_CORE_IBUS_TIMEOUT_INT_RAW** `[3]` (RO)
///
///   the raw interrupt status of lp core ibus timeout
///
/// - **LP_CORE_AHB_TIMEOUT_INT_RAW** `[2]` (RO)
///
///   the raw interrupt status of lp core ahb bus timeout
///
/// - **IDBUS_ADDRHOLE_INT_RAW** `[1]` (RO)
///
///   the raw interrupt status of idbus addrhole(only for lp cpu ibus and dbus)
///
/// - **LP_ADDRHOLE_INT_RAW** `[0]` (RO)
///
///   the raw interrupt status of lp addrhole(for lp peri and lp ram tee apm, and lp matrix default slave)
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

    /// **SLOW_CLK_TICK_INT_RAW**
    ///
    /// the raw interrupt status of slow_clk_tick
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SLOW_CLK_TICK_INT_RAW_OFFSET: u32 = 6;
    pub const SLOW_CLK_TICK_INT_RAW_MASK: u32 = 0x00000040;
    pub const SLOW_CLK_TICK_INT_RAW: u32 = 1 << 6;

    /// **ETM_TASK_ULP_INT_RAW**
    ///
    /// the raw interrupt status of etm task ulp
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ETM_TASK_ULP_INT_RAW_OFFSET: u32 = 5;
    pub const ETM_TASK_ULP_INT_RAW_MASK: u32 = 0x00000020;
    pub const ETM_TASK_ULP_INT_RAW: u32 = 1 << 5;

    /// **LP_CORE_DBUS_TIMEOUT_INT_RAW**
    ///
    /// the raw interrupt status of lp core dbus timeout
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_CORE_DBUS_TIMEOUT_INT_RAW_OFFSET: u32 = 4;
    pub const LP_CORE_DBUS_TIMEOUT_INT_RAW_MASK: u32 = 0x00000010;
    pub const LP_CORE_DBUS_TIMEOUT_INT_RAW: u32 = 1 << 4;

    /// **LP_CORE_IBUS_TIMEOUT_INT_RAW**
    ///
    /// the raw interrupt status of lp core ibus timeout
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LP_CORE_IBUS_TIMEOUT_INT_RAW_OFFSET: u32 = 3;
    pub const LP_CORE_IBUS_TIMEOUT_INT_RAW_MASK: u32 = 0x00000008;
    pub const LP_CORE_IBUS_TIMEOUT_INT_RAW: u32 = 1 << 3;

    /// **LP_CORE_AHB_TIMEOUT_INT_RAW**
    ///
    /// the raw interrupt status of lp core ahb bus timeout
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const LP_CORE_AHB_TIMEOUT_INT_RAW_OFFSET: u32 = 2;
    pub const LP_CORE_AHB_TIMEOUT_INT_RAW_MASK: u32 = 0x00000004;
    pub const LP_CORE_AHB_TIMEOUT_INT_RAW: u32 = 1 << 2;

    /// **IDBUS_ADDRHOLE_INT_RAW**
    ///
    /// the raw interrupt status of idbus addrhole(only for lp cpu ibus and dbus)
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IDBUS_ADDRHOLE_INT_RAW_OFFSET: u32 = 1;
    pub const IDBUS_ADDRHOLE_INT_RAW_MASK: u32 = 0x00000002;
    pub const IDBUS_ADDRHOLE_INT_RAW: u32 = 1 << 1;

    /// **LP_ADDRHOLE_INT_RAW**
    ///
    /// the raw interrupt status of lp addrhole(for lp peri and lp ram tee apm, and lp matrix default slave)
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_ADDRHOLE_INT_RAW_OFFSET: u32 = 0;
    pub const LP_ADDRHOLE_INT_RAW_MASK: u32 = 0x00000001;
    pub const LP_ADDRHOLE_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SLOW_CLK_TICK_INT_RAW** `[6]` (RO)
    ///
    /// the raw interrupt status of slow_clk_tick
    #[inline(always)]
    pub fn slow_clk_tick_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::SLOW_CLK_TICK_INT_RAW_MASK) != 0
    }

    /// **ETM_TASK_ULP_INT_RAW** `[5]` (RO)
    ///
    /// the raw interrupt status of etm task ulp
    #[inline(always)]
    pub fn etm_task_ulp_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_ULP_INT_RAW_MASK) != 0
    }

    /// **LP_CORE_DBUS_TIMEOUT_INT_RAW** `[4]` (RO)
    ///
    /// the raw interrupt status of lp core dbus timeout
    #[inline(always)]
    pub fn lp_core_dbus_timeout_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::LP_CORE_DBUS_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// **LP_CORE_IBUS_TIMEOUT_INT_RAW** `[3]` (RO)
    ///
    /// the raw interrupt status of lp core ibus timeout
    #[inline(always)]
    pub fn lp_core_ibus_timeout_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::LP_CORE_IBUS_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// **LP_CORE_AHB_TIMEOUT_INT_RAW** `[2]` (RO)
    ///
    /// the raw interrupt status of lp core ahb bus timeout
    #[inline(always)]
    pub fn lp_core_ahb_timeout_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::LP_CORE_AHB_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// **IDBUS_ADDRHOLE_INT_RAW** `[1]` (RO)
    ///
    /// the raw interrupt status of idbus addrhole(only for lp cpu ibus and dbus)
    #[inline(always)]
    pub fn idbus_addrhole_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::IDBUS_ADDRHOLE_INT_RAW_MASK) != 0
    }

    /// **LP_ADDRHOLE_INT_RAW** `[0]` (RO)
    ///
    /// the raw interrupt status of lp addrhole(for lp peri and lp ram tee apm, and lp matrix default slave)
    #[inline(always)]
    pub fn lp_addrhole_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::LP_ADDRHOLE_INT_RAW_MASK) != 0
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

    /// Read **SLOW_CLK_TICK_INT_RAW** `[6:6]`
    ///
    /// the raw interrupt status of slow_clk_tick
    #[inline(always)]
    pub const fn slow_clk_tick_int_raw(&self) -> bool {
        (self.bits & IntRaw::SLOW_CLK_TICK_INT_RAW_MASK) != 0
    }

    /// Read **ETM_TASK_ULP_INT_RAW** `[5:5]`
    ///
    /// the raw interrupt status of etm task ulp
    #[inline(always)]
    pub const fn etm_task_ulp_int_raw(&self) -> bool {
        (self.bits & IntRaw::ETM_TASK_ULP_INT_RAW_MASK) != 0
    }

    /// Read **LP_CORE_DBUS_TIMEOUT_INT_RAW** `[4:4]`
    ///
    /// the raw interrupt status of lp core dbus timeout
    #[inline(always)]
    pub const fn lp_core_dbus_timeout_int_raw(&self) -> bool {
        (self.bits & IntRaw::LP_CORE_DBUS_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Read **LP_CORE_IBUS_TIMEOUT_INT_RAW** `[3:3]`
    ///
    /// the raw interrupt status of lp core ibus timeout
    #[inline(always)]
    pub const fn lp_core_ibus_timeout_int_raw(&self) -> bool {
        (self.bits & IntRaw::LP_CORE_IBUS_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Read **LP_CORE_AHB_TIMEOUT_INT_RAW** `[2:2]`
    ///
    /// the raw interrupt status of lp core ahb bus timeout
    #[inline(always)]
    pub const fn lp_core_ahb_timeout_int_raw(&self) -> bool {
        (self.bits & IntRaw::LP_CORE_AHB_TIMEOUT_INT_RAW_MASK) != 0
    }

    /// Read **IDBUS_ADDRHOLE_INT_RAW** `[1:1]`
    ///
    /// the raw interrupt status of idbus addrhole(only for lp cpu ibus and dbus)
    #[inline(always)]
    pub const fn idbus_addrhole_int_raw(&self) -> bool {
        (self.bits & IntRaw::IDBUS_ADDRHOLE_INT_RAW_MASK) != 0
    }

    /// Read **LP_ADDRHOLE_INT_RAW** `[0:0]`
    ///
    /// the raw interrupt status of lp addrhole(for lp peri and lp ram tee apm, and lp matrix default slave)
    #[inline(always)]
    pub const fn lp_addrhole_int_raw(&self) -> bool {
        (self.bits & IntRaw::LP_ADDRHOLE_INT_RAW_MASK) != 0
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
/// masked interrupt register
///
/// Address offset: 0x0174
///
/// ## Fields
///
/// - **SLOW_CLK_TICK_INT_ST** `[6]` (RO)
///
///   the masked interrupt status of slow_clk_tick
///
/// - **ETM_TASK_ULP_INT_ST** `[5]` (RO)
///
///   the masked interrupt status of etm task ulp
///
/// - **LP_CORE_DBUS_TIMEOUT_INT_ST** `[4]` (RO)
///
///   the masked interrupt status of lp core dbus timeout
///
/// - **LP_CORE_IBUS_TIMEOUT_INT_ST** `[3]` (RO)
///
///   the masked interrupt status of lp core ibus timeout
///
/// - **LP_CORE_AHB_TIMEOUT_INT_ST** `[2]` (RO)
///
///   the masked interrupt status of lp core ahb bus timeout
///
/// - **IDBUS_ADDRHOLE_INT_ST** `[1]` (RO)
///
///   the masked interrupt status of idbus addrhole(only for lp cpu ibus and dbus)
///
/// - **LP_ADDRHOLE_INT_ST** `[0]` (RO)
///
///   the masked interrupt status of lp addrhole (for lp peri and lp ram tee apm, and lp matrix default slave)
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

    /// **SLOW_CLK_TICK_INT_ST**
    ///
    /// the masked interrupt status of slow_clk_tick
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SLOW_CLK_TICK_INT_ST_OFFSET: u32 = 6;
    pub const SLOW_CLK_TICK_INT_ST_MASK: u32 = 0x00000040;
    pub const SLOW_CLK_TICK_INT_ST: u32 = 1 << 6;

    /// **ETM_TASK_ULP_INT_ST**
    ///
    /// the masked interrupt status of etm task ulp
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ETM_TASK_ULP_INT_ST_OFFSET: u32 = 5;
    pub const ETM_TASK_ULP_INT_ST_MASK: u32 = 0x00000020;
    pub const ETM_TASK_ULP_INT_ST: u32 = 1 << 5;

    /// **LP_CORE_DBUS_TIMEOUT_INT_ST**
    ///
    /// the masked interrupt status of lp core dbus timeout
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_CORE_DBUS_TIMEOUT_INT_ST_OFFSET: u32 = 4;
    pub const LP_CORE_DBUS_TIMEOUT_INT_ST_MASK: u32 = 0x00000010;
    pub const LP_CORE_DBUS_TIMEOUT_INT_ST: u32 = 1 << 4;

    /// **LP_CORE_IBUS_TIMEOUT_INT_ST**
    ///
    /// the masked interrupt status of lp core ibus timeout
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LP_CORE_IBUS_TIMEOUT_INT_ST_OFFSET: u32 = 3;
    pub const LP_CORE_IBUS_TIMEOUT_INT_ST_MASK: u32 = 0x00000008;
    pub const LP_CORE_IBUS_TIMEOUT_INT_ST: u32 = 1 << 3;

    /// **LP_CORE_AHB_TIMEOUT_INT_ST**
    ///
    /// the masked interrupt status of lp core ahb bus timeout
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const LP_CORE_AHB_TIMEOUT_INT_ST_OFFSET: u32 = 2;
    pub const LP_CORE_AHB_TIMEOUT_INT_ST_MASK: u32 = 0x00000004;
    pub const LP_CORE_AHB_TIMEOUT_INT_ST: u32 = 1 << 2;

    /// **IDBUS_ADDRHOLE_INT_ST**
    ///
    /// the masked interrupt status of idbus addrhole(only for lp cpu ibus and dbus)
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IDBUS_ADDRHOLE_INT_ST_OFFSET: u32 = 1;
    pub const IDBUS_ADDRHOLE_INT_ST_MASK: u32 = 0x00000002;
    pub const IDBUS_ADDRHOLE_INT_ST: u32 = 1 << 1;

    /// **LP_ADDRHOLE_INT_ST**
    ///
    /// the masked interrupt status of lp addrhole (for lp peri and lp ram tee apm, and lp matrix default slave)
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_ADDRHOLE_INT_ST_OFFSET: u32 = 0;
    pub const LP_ADDRHOLE_INT_ST_MASK: u32 = 0x00000001;
    pub const LP_ADDRHOLE_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SLOW_CLK_TICK_INT_ST** `[6]` (RO)
    ///
    /// the masked interrupt status of slow_clk_tick
    #[inline(always)]
    pub fn slow_clk_tick_int_st(&self) -> bool {
        (self.value.read_bits() & Self::SLOW_CLK_TICK_INT_ST_MASK) != 0
    }

    /// **ETM_TASK_ULP_INT_ST** `[5]` (RO)
    ///
    /// the masked interrupt status of etm task ulp
    #[inline(always)]
    pub fn etm_task_ulp_int_st(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_ULP_INT_ST_MASK) != 0
    }

    /// **LP_CORE_DBUS_TIMEOUT_INT_ST** `[4]` (RO)
    ///
    /// the masked interrupt status of lp core dbus timeout
    #[inline(always)]
    pub fn lp_core_dbus_timeout_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_CORE_DBUS_TIMEOUT_INT_ST_MASK) != 0
    }

    /// **LP_CORE_IBUS_TIMEOUT_INT_ST** `[3]` (RO)
    ///
    /// the masked interrupt status of lp core ibus timeout
    #[inline(always)]
    pub fn lp_core_ibus_timeout_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_CORE_IBUS_TIMEOUT_INT_ST_MASK) != 0
    }

    /// **LP_CORE_AHB_TIMEOUT_INT_ST** `[2]` (RO)
    ///
    /// the masked interrupt status of lp core ahb bus timeout
    #[inline(always)]
    pub fn lp_core_ahb_timeout_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_CORE_AHB_TIMEOUT_INT_ST_MASK) != 0
    }

    /// **IDBUS_ADDRHOLE_INT_ST** `[1]` (RO)
    ///
    /// the masked interrupt status of idbus addrhole(only for lp cpu ibus and dbus)
    #[inline(always)]
    pub fn idbus_addrhole_int_st(&self) -> bool {
        (self.value.read_bits() & Self::IDBUS_ADDRHOLE_INT_ST_MASK) != 0
    }

    /// **LP_ADDRHOLE_INT_ST** `[0]` (RO)
    ///
    /// the masked interrupt status of lp addrhole (for lp peri and lp ram tee apm, and lp matrix default slave)
    #[inline(always)]
    pub fn lp_addrhole_int_st(&self) -> bool {
        (self.value.read_bits() & Self::LP_ADDRHOLE_INT_ST_MASK) != 0
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

    /// Read **SLOW_CLK_TICK_INT_ST** `[6:6]`
    ///
    /// the masked interrupt status of slow_clk_tick
    #[inline(always)]
    pub const fn slow_clk_tick_int_st(&self) -> bool {
        (self.bits & IntSt::SLOW_CLK_TICK_INT_ST_MASK) != 0
    }

    /// Read **ETM_TASK_ULP_INT_ST** `[5:5]`
    ///
    /// the masked interrupt status of etm task ulp
    #[inline(always)]
    pub const fn etm_task_ulp_int_st(&self) -> bool {
        (self.bits & IntSt::ETM_TASK_ULP_INT_ST_MASK) != 0
    }

    /// Read **LP_CORE_DBUS_TIMEOUT_INT_ST** `[4:4]`
    ///
    /// the masked interrupt status of lp core dbus timeout
    #[inline(always)]
    pub const fn lp_core_dbus_timeout_int_st(&self) -> bool {
        (self.bits & IntSt::LP_CORE_DBUS_TIMEOUT_INT_ST_MASK) != 0
    }

    /// Read **LP_CORE_IBUS_TIMEOUT_INT_ST** `[3:3]`
    ///
    /// the masked interrupt status of lp core ibus timeout
    #[inline(always)]
    pub const fn lp_core_ibus_timeout_int_st(&self) -> bool {
        (self.bits & IntSt::LP_CORE_IBUS_TIMEOUT_INT_ST_MASK) != 0
    }

    /// Read **LP_CORE_AHB_TIMEOUT_INT_ST** `[2:2]`
    ///
    /// the masked interrupt status of lp core ahb bus timeout
    #[inline(always)]
    pub const fn lp_core_ahb_timeout_int_st(&self) -> bool {
        (self.bits & IntSt::LP_CORE_AHB_TIMEOUT_INT_ST_MASK) != 0
    }

    /// Read **IDBUS_ADDRHOLE_INT_ST** `[1:1]`
    ///
    /// the masked interrupt status of idbus addrhole(only for lp cpu ibus and dbus)
    #[inline(always)]
    pub const fn idbus_addrhole_int_st(&self) -> bool {
        (self.bits & IntSt::IDBUS_ADDRHOLE_INT_ST_MASK) != 0
    }

    /// Read **LP_ADDRHOLE_INT_ST** `[0:0]`
    ///
    /// the masked interrupt status of lp addrhole (for lp peri and lp ram tee apm, and lp matrix default slave)
    #[inline(always)]
    pub const fn lp_addrhole_int_st(&self) -> bool {
        (self.bits & IntSt::LP_ADDRHOLE_INT_ST_MASK) != 0
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
/// masked interrupt register
///
/// Address offset: 0x0178
///
/// ## Fields
///
/// - **SLOW_CLK_TICK_INT_ENA** `[6]` (RW)
///
///   Write 1 to enable slow_clk_tick int
///
/// - **ETM_TASK_ULP_INT_ENA** `[5]` (RW)
///
///   Write 1 to enable etm task ulp int
///
/// - **LP_CORE_DBUS_TIMEOUT_INT_ENA** `[4]` (RW)
///
///   Write 1 to enable lp_core_dbus_timeout int
///
/// - **LP_CORE_IBUS_TIMEOUT_INT_ENA** `[3]` (RW)
///
///   Write 1 to enable lp_core_ibus_timeout int
///
/// - **LP_CORE_AHB_TIMEOUT_INT_ENA** `[2]` (RW)
///
///   Write 1 to enable lp_core_ahb_timeout int
///
/// - **IDBUS_ADDRHOLE_INT_ENA** `[1]` (RW)
///
///   Write 1 to enable idbus addrhole int
///
/// - **LP_ADDRHOLE_INT_ENA** `[0]` (RW)
///
///   Write 1 to enable lp addrhole int
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

    /// **SLOW_CLK_TICK_INT_ENA**
    ///
    /// Write 1 to enable slow_clk_tick int
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SLOW_CLK_TICK_INT_ENA_OFFSET: u32 = 6;
    pub const SLOW_CLK_TICK_INT_ENA_MASK: u32 = 0x00000040;
    pub const SLOW_CLK_TICK_INT_ENA: u32 = 1 << 6;

    /// **ETM_TASK_ULP_INT_ENA**
    ///
    /// Write 1 to enable etm task ulp int
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ETM_TASK_ULP_INT_ENA_OFFSET: u32 = 5;
    pub const ETM_TASK_ULP_INT_ENA_MASK: u32 = 0x00000020;
    pub const ETM_TASK_ULP_INT_ENA: u32 = 1 << 5;

    /// **LP_CORE_DBUS_TIMEOUT_INT_ENA**
    ///
    /// Write 1 to enable lp_core_dbus_timeout int
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_CORE_DBUS_TIMEOUT_INT_ENA_OFFSET: u32 = 4;
    pub const LP_CORE_DBUS_TIMEOUT_INT_ENA_MASK: u32 = 0x00000010;
    pub const LP_CORE_DBUS_TIMEOUT_INT_ENA: u32 = 1 << 4;

    /// **LP_CORE_IBUS_TIMEOUT_INT_ENA**
    ///
    /// Write 1 to enable lp_core_ibus_timeout int
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LP_CORE_IBUS_TIMEOUT_INT_ENA_OFFSET: u32 = 3;
    pub const LP_CORE_IBUS_TIMEOUT_INT_ENA_MASK: u32 = 0x00000008;
    pub const LP_CORE_IBUS_TIMEOUT_INT_ENA: u32 = 1 << 3;

    /// **LP_CORE_AHB_TIMEOUT_INT_ENA**
    ///
    /// Write 1 to enable lp_core_ahb_timeout int
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const LP_CORE_AHB_TIMEOUT_INT_ENA_OFFSET: u32 = 2;
    pub const LP_CORE_AHB_TIMEOUT_INT_ENA_MASK: u32 = 0x00000004;
    pub const LP_CORE_AHB_TIMEOUT_INT_ENA: u32 = 1 << 2;

    /// **IDBUS_ADDRHOLE_INT_ENA**
    ///
    /// Write 1 to enable idbus addrhole int
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IDBUS_ADDRHOLE_INT_ENA_OFFSET: u32 = 1;
    pub const IDBUS_ADDRHOLE_INT_ENA_MASK: u32 = 0x00000002;
    pub const IDBUS_ADDRHOLE_INT_ENA: u32 = 1 << 1;

    /// **LP_ADDRHOLE_INT_ENA**
    ///
    /// Write 1 to enable lp addrhole int
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_ADDRHOLE_INT_ENA_OFFSET: u32 = 0;
    pub const LP_ADDRHOLE_INT_ENA_MASK: u32 = 0x00000001;
    pub const LP_ADDRHOLE_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **SLOW_CLK_TICK_INT_ENA** `[6]` (RW)
    ///
    /// Write 1 to enable slow_clk_tick int
    #[inline(always)]
    pub fn slow_clk_tick_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::SLOW_CLK_TICK_INT_ENA_MASK) != 0
    }

    /// Set SLOW_CLK_TICK_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_slow_clk_tick_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::SLOW_CLK_TICK_INT_ENA_MASK);
    }

    /// Clear SLOW_CLK_TICK_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_slow_clk_tick_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::SLOW_CLK_TICK_INT_ENA_MASK);
    }

    /// **ETM_TASK_ULP_INT_ENA** `[5]` (RW)
    ///
    /// Write 1 to enable etm task ulp int
    #[inline(always)]
    pub fn etm_task_ulp_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::ETM_TASK_ULP_INT_ENA_MASK) != 0
    }

    /// Set ETM_TASK_ULP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_task_ulp_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::ETM_TASK_ULP_INT_ENA_MASK);
    }

    /// Clear ETM_TASK_ULP_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_task_ulp_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::ETM_TASK_ULP_INT_ENA_MASK);
    }

    /// **LP_CORE_DBUS_TIMEOUT_INT_ENA** `[4]` (RW)
    ///
    /// Write 1 to enable lp_core_dbus_timeout int
    #[inline(always)]
    pub fn lp_core_dbus_timeout_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_CORE_DBUS_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Set LP_CORE_DBUS_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_core_dbus_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_CORE_DBUS_TIMEOUT_INT_ENA_MASK);
    }

    /// Clear LP_CORE_DBUS_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_core_dbus_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_CORE_DBUS_TIMEOUT_INT_ENA_MASK);
    }

    /// **LP_CORE_IBUS_TIMEOUT_INT_ENA** `[3]` (RW)
    ///
    /// Write 1 to enable lp_core_ibus_timeout int
    #[inline(always)]
    pub fn lp_core_ibus_timeout_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_CORE_IBUS_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Set LP_CORE_IBUS_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_core_ibus_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_CORE_IBUS_TIMEOUT_INT_ENA_MASK);
    }

    /// Clear LP_CORE_IBUS_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_core_ibus_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_CORE_IBUS_TIMEOUT_INT_ENA_MASK);
    }

    /// **LP_CORE_AHB_TIMEOUT_INT_ENA** `[2]` (RW)
    ///
    /// Write 1 to enable lp_core_ahb_timeout int
    #[inline(always)]
    pub fn lp_core_ahb_timeout_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_CORE_AHB_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Set LP_CORE_AHB_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_core_ahb_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_CORE_AHB_TIMEOUT_INT_ENA_MASK);
    }

    /// Clear LP_CORE_AHB_TIMEOUT_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_core_ahb_timeout_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_CORE_AHB_TIMEOUT_INT_ENA_MASK);
    }

    /// **IDBUS_ADDRHOLE_INT_ENA** `[1]` (RW)
    ///
    /// Write 1 to enable idbus addrhole int
    #[inline(always)]
    pub fn idbus_addrhole_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::IDBUS_ADDRHOLE_INT_ENA_MASK) != 0
    }

    /// Set IDBUS_ADDRHOLE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_idbus_addrhole_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::IDBUS_ADDRHOLE_INT_ENA_MASK);
    }

    /// Clear IDBUS_ADDRHOLE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_idbus_addrhole_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::IDBUS_ADDRHOLE_INT_ENA_MASK);
    }

    /// **LP_ADDRHOLE_INT_ENA** `[0]` (RW)
    ///
    /// Write 1 to enable lp addrhole int
    #[inline(always)]
    pub fn lp_addrhole_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::LP_ADDRHOLE_INT_ENA_MASK) != 0
    }

    /// Set LP_ADDRHOLE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp_addrhole_int_ena(&self) {
        self.value
            .modify_bits(|r| r | Self::LP_ADDRHOLE_INT_ENA_MASK);
    }

    /// Clear LP_ADDRHOLE_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp_addrhole_int_ena(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP_ADDRHOLE_INT_ENA_MASK);
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

    /// Read **SLOW_CLK_TICK_INT_ENA** `[6:6]`
    ///
    /// Write 1 to enable slow_clk_tick int
    #[inline(always)]
    pub const fn slow_clk_tick_int_ena(&self) -> bool {
        (self.bits & IntEna::SLOW_CLK_TICK_INT_ENA_MASK) != 0
    }

    /// Read **ETM_TASK_ULP_INT_ENA** `[5:5]`
    ///
    /// Write 1 to enable etm task ulp int
    #[inline(always)]
    pub const fn etm_task_ulp_int_ena(&self) -> bool {
        (self.bits & IntEna::ETM_TASK_ULP_INT_ENA_MASK) != 0
    }

    /// Read **LP_CORE_DBUS_TIMEOUT_INT_ENA** `[4:4]`
    ///
    /// Write 1 to enable lp_core_dbus_timeout int
    #[inline(always)]
    pub const fn lp_core_dbus_timeout_int_ena(&self) -> bool {
        (self.bits & IntEna::LP_CORE_DBUS_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Read **LP_CORE_IBUS_TIMEOUT_INT_ENA** `[3:3]`
    ///
    /// Write 1 to enable lp_core_ibus_timeout int
    #[inline(always)]
    pub const fn lp_core_ibus_timeout_int_ena(&self) -> bool {
        (self.bits & IntEna::LP_CORE_IBUS_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Read **LP_CORE_AHB_TIMEOUT_INT_ENA** `[2:2]`
    ///
    /// Write 1 to enable lp_core_ahb_timeout int
    #[inline(always)]
    pub const fn lp_core_ahb_timeout_int_ena(&self) -> bool {
        (self.bits & IntEna::LP_CORE_AHB_TIMEOUT_INT_ENA_MASK) != 0
    }

    /// Read **IDBUS_ADDRHOLE_INT_ENA** `[1:1]`
    ///
    /// Write 1 to enable idbus addrhole int
    #[inline(always)]
    pub const fn idbus_addrhole_int_ena(&self) -> bool {
        (self.bits & IntEna::IDBUS_ADDRHOLE_INT_ENA_MASK) != 0
    }

    /// Read **LP_ADDRHOLE_INT_ENA** `[0:0]`
    ///
    /// Write 1 to enable lp addrhole int
    #[inline(always)]
    pub const fn lp_addrhole_int_ena(&self) -> bool {
        (self.bits & IntEna::LP_ADDRHOLE_INT_ENA_MASK) != 0
    }

    /// Set **SLOW_CLK_TICK_INT_ENA** `[6:6]`
    ///
    /// Write 1 to enable slow_clk_tick int
    #[inline(always)]
    pub const fn with_slow_clk_tick_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::SLOW_CLK_TICK_INT_ENA_MASK
            } else {
                self.bits & !IntEna::SLOW_CLK_TICK_INT_ENA_MASK
            },
        }
    }

    /// Set **ETM_TASK_ULP_INT_ENA** `[5:5]`
    ///
    /// Write 1 to enable etm task ulp int
    #[inline(always)]
    pub const fn with_etm_task_ulp_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::ETM_TASK_ULP_INT_ENA_MASK
            } else {
                self.bits & !IntEna::ETM_TASK_ULP_INT_ENA_MASK
            },
        }
    }

    /// Set **LP_CORE_DBUS_TIMEOUT_INT_ENA** `[4:4]`
    ///
    /// Write 1 to enable lp_core_dbus_timeout int
    #[inline(always)]
    pub const fn with_lp_core_dbus_timeout_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::LP_CORE_DBUS_TIMEOUT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::LP_CORE_DBUS_TIMEOUT_INT_ENA_MASK
            },
        }
    }

    /// Set **LP_CORE_IBUS_TIMEOUT_INT_ENA** `[3:3]`
    ///
    /// Write 1 to enable lp_core_ibus_timeout int
    #[inline(always)]
    pub const fn with_lp_core_ibus_timeout_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::LP_CORE_IBUS_TIMEOUT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::LP_CORE_IBUS_TIMEOUT_INT_ENA_MASK
            },
        }
    }

    /// Set **LP_CORE_AHB_TIMEOUT_INT_ENA** `[2:2]`
    ///
    /// Write 1 to enable lp_core_ahb_timeout int
    #[inline(always)]
    pub const fn with_lp_core_ahb_timeout_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::LP_CORE_AHB_TIMEOUT_INT_ENA_MASK
            } else {
                self.bits & !IntEna::LP_CORE_AHB_TIMEOUT_INT_ENA_MASK
            },
        }
    }

    /// Set **IDBUS_ADDRHOLE_INT_ENA** `[1:1]`
    ///
    /// Write 1 to enable idbus addrhole int
    #[inline(always)]
    pub const fn with_idbus_addrhole_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::IDBUS_ADDRHOLE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::IDBUS_ADDRHOLE_INT_ENA_MASK
            },
        }
    }

    /// Set **LP_ADDRHOLE_INT_ENA** `[0:0]`
    ///
    /// Write 1 to enable lp addrhole int
    #[inline(always)]
    pub const fn with_lp_addrhole_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::LP_ADDRHOLE_INT_ENA_MASK
            } else {
                self.bits & !IntEna::LP_ADDRHOLE_INT_ENA_MASK
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
/// interrupt clear register
///
/// Address offset: 0x017c
///
/// ## Fields
///
/// - **SLOW_CLK_TICK_INT_CLR** `[6]` (WO)
///
///   Write 1 to clear slow_clk_tick int
///
/// - **ETM_TASK_ULP_INT_CLR** `[5]` (WO)
///
///   Write 1 to clear etm tasl ulp int
///
/// - **LP_CORE_DBUS_TIMEOUT_INT_CLR** `[4]` (WO)
///
///   Write 1 to clear lp_core_dbus_timeout int
///
/// - **LP_CORE_IBUS_TIMEOUT_INT_CLR** `[3]` (WO)
///
///   Write 1 to clear lp_core_ibus_timeout int
///
/// - **LP_CORE_AHB_TIMEOUT_INT_CLR** `[2]` (WO)
///
///   Write 1 to clear lp_core_ahb_timeout int
///
/// - **IDBUS_ADDRHOLE_INT_CLR** `[1]` (WO)
///
///   write 1 to clear idbus addrhole int
///
/// - **LP_ADDRHOLE_INT_CLR** `[0]` (WO)
///
///   write 1 to clear lp addrhole int
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

    /// **SLOW_CLK_TICK_INT_CLR**
    ///
    /// Write 1 to clear slow_clk_tick int
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const SLOW_CLK_TICK_INT_CLR_OFFSET: u32 = 6;
    pub const SLOW_CLK_TICK_INT_CLR_MASK: u32 = 0x00000040;
    pub const SLOW_CLK_TICK_INT_CLR: u32 = 1 << 6;

    /// **ETM_TASK_ULP_INT_CLR**
    ///
    /// Write 1 to clear etm tasl ulp int
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const ETM_TASK_ULP_INT_CLR_OFFSET: u32 = 5;
    pub const ETM_TASK_ULP_INT_CLR_MASK: u32 = 0x00000020;
    pub const ETM_TASK_ULP_INT_CLR: u32 = 1 << 5;

    /// **LP_CORE_DBUS_TIMEOUT_INT_CLR**
    ///
    /// Write 1 to clear lp_core_dbus_timeout int
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const LP_CORE_DBUS_TIMEOUT_INT_CLR_OFFSET: u32 = 4;
    pub const LP_CORE_DBUS_TIMEOUT_INT_CLR_MASK: u32 = 0x00000010;
    pub const LP_CORE_DBUS_TIMEOUT_INT_CLR: u32 = 1 << 4;

    /// **LP_CORE_IBUS_TIMEOUT_INT_CLR**
    ///
    /// Write 1 to clear lp_core_ibus_timeout int
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const LP_CORE_IBUS_TIMEOUT_INT_CLR_OFFSET: u32 = 3;
    pub const LP_CORE_IBUS_TIMEOUT_INT_CLR_MASK: u32 = 0x00000008;
    pub const LP_CORE_IBUS_TIMEOUT_INT_CLR: u32 = 1 << 3;

    /// **LP_CORE_AHB_TIMEOUT_INT_CLR**
    ///
    /// Write 1 to clear lp_core_ahb_timeout int
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const LP_CORE_AHB_TIMEOUT_INT_CLR_OFFSET: u32 = 2;
    pub const LP_CORE_AHB_TIMEOUT_INT_CLR_MASK: u32 = 0x00000004;
    pub const LP_CORE_AHB_TIMEOUT_INT_CLR: u32 = 1 << 2;

    /// **IDBUS_ADDRHOLE_INT_CLR**
    ///
    /// write 1 to clear idbus addrhole int
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const IDBUS_ADDRHOLE_INT_CLR_OFFSET: u32 = 1;
    pub const IDBUS_ADDRHOLE_INT_CLR_MASK: u32 = 0x00000002;
    pub const IDBUS_ADDRHOLE_INT_CLR: u32 = 1 << 1;

    /// **LP_ADDRHOLE_INT_CLR**
    ///
    /// write 1 to clear lp addrhole int
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LP_ADDRHOLE_INT_CLR_OFFSET: u32 = 0;
    pub const LP_ADDRHOLE_INT_CLR_MASK: u32 = 0x00000001;
    pub const LP_ADDRHOLE_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **SLOW_CLK_TICK_INT_CLR** (WO)
    ///
    /// Write 1 to clear slow_clk_tick int
    #[inline(always)]
    pub fn trigger_slow_clk_tick_int_clr(&self) {
        self.value.write_bits(Self::SLOW_CLK_TICK_INT_CLR_MASK);
    }

    /// Trigger **ETM_TASK_ULP_INT_CLR** (WO)
    ///
    /// Write 1 to clear etm tasl ulp int
    #[inline(always)]
    pub fn trigger_etm_task_ulp_int_clr(&self) {
        self.value.write_bits(Self::ETM_TASK_ULP_INT_CLR_MASK);
    }

    /// Trigger **LP_CORE_DBUS_TIMEOUT_INT_CLR** (WO)
    ///
    /// Write 1 to clear lp_core_dbus_timeout int
    #[inline(always)]
    pub fn trigger_lp_core_dbus_timeout_int_clr(&self) {
        self.value
            .write_bits(Self::LP_CORE_DBUS_TIMEOUT_INT_CLR_MASK);
    }

    /// Trigger **LP_CORE_IBUS_TIMEOUT_INT_CLR** (WO)
    ///
    /// Write 1 to clear lp_core_ibus_timeout int
    #[inline(always)]
    pub fn trigger_lp_core_ibus_timeout_int_clr(&self) {
        self.value
            .write_bits(Self::LP_CORE_IBUS_TIMEOUT_INT_CLR_MASK);
    }

    /// Trigger **LP_CORE_AHB_TIMEOUT_INT_CLR** (WO)
    ///
    /// Write 1 to clear lp_core_ahb_timeout int
    #[inline(always)]
    pub fn trigger_lp_core_ahb_timeout_int_clr(&self) {
        self.value
            .write_bits(Self::LP_CORE_AHB_TIMEOUT_INT_CLR_MASK);
    }

    /// Trigger **IDBUS_ADDRHOLE_INT_CLR** (WO)
    ///
    /// write 1 to clear idbus addrhole int
    #[inline(always)]
    pub fn trigger_idbus_addrhole_int_clr(&self) {
        self.value.write_bits(Self::IDBUS_ADDRHOLE_INT_CLR_MASK);
    }

    /// Trigger **LP_ADDRHOLE_INT_CLR** (WO)
    ///
    /// write 1 to clear lp addrhole int
    #[inline(always)]
    pub fn trigger_lp_addrhole_int_clr(&self) {
        self.value.write_bits(Self::LP_ADDRHOLE_INT_CLR_MASK);
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

    /// Set **SLOW_CLK_TICK_INT_CLR** `[6:6]`
    ///
    /// Write 1 to clear slow_clk_tick int
    #[inline(always)]
    pub const fn with_slow_clk_tick_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::SLOW_CLK_TICK_INT_CLR_MASK
            } else {
                self.bits & !IntClr::SLOW_CLK_TICK_INT_CLR_MASK
            },
        }
    }

    /// Set **ETM_TASK_ULP_INT_CLR** `[5:5]`
    ///
    /// Write 1 to clear etm tasl ulp int
    #[inline(always)]
    pub const fn with_etm_task_ulp_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::ETM_TASK_ULP_INT_CLR_MASK
            } else {
                self.bits & !IntClr::ETM_TASK_ULP_INT_CLR_MASK
            },
        }
    }

    /// Set **LP_CORE_DBUS_TIMEOUT_INT_CLR** `[4:4]`
    ///
    /// Write 1 to clear lp_core_dbus_timeout int
    #[inline(always)]
    pub const fn with_lp_core_dbus_timeout_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::LP_CORE_DBUS_TIMEOUT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::LP_CORE_DBUS_TIMEOUT_INT_CLR_MASK
            },
        }
    }

    /// Set **LP_CORE_IBUS_TIMEOUT_INT_CLR** `[3:3]`
    ///
    /// Write 1 to clear lp_core_ibus_timeout int
    #[inline(always)]
    pub const fn with_lp_core_ibus_timeout_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::LP_CORE_IBUS_TIMEOUT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::LP_CORE_IBUS_TIMEOUT_INT_CLR_MASK
            },
        }
    }

    /// Set **LP_CORE_AHB_TIMEOUT_INT_CLR** `[2:2]`
    ///
    /// Write 1 to clear lp_core_ahb_timeout int
    #[inline(always)]
    pub const fn with_lp_core_ahb_timeout_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::LP_CORE_AHB_TIMEOUT_INT_CLR_MASK
            } else {
                self.bits & !IntClr::LP_CORE_AHB_TIMEOUT_INT_CLR_MASK
            },
        }
    }

    /// Set **IDBUS_ADDRHOLE_INT_CLR** `[1:1]`
    ///
    /// write 1 to clear idbus addrhole int
    #[inline(always)]
    pub const fn with_idbus_addrhole_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::IDBUS_ADDRHOLE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::IDBUS_ADDRHOLE_INT_CLR_MASK
            },
        }
    }

    /// Set **LP_ADDRHOLE_INT_CLR** `[0:0]`
    ///
    /// write 1 to clear lp addrhole int
    #[inline(always)]
    pub const fn with_lp_addrhole_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::LP_ADDRHOLE_INT_CLR_MASK
            } else {
                self.bits & !IntClr::LP_ADDRHOLE_INT_CLR_MASK
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

/// HP_MEM_AUX_CTRL Register
///
/// need_des
///
/// Address offset: 0x0180
/// Reset value: 0x00002070
///
/// ## Fields
///
/// - **HP_MEM_AUX_CTRL** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct HpMemAuxCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HpMemAuxCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HpMemAuxCtrl {
    // ======== Field Constants ========

    /// **HP_MEM_AUX_CTRL**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const HP_MEM_AUX_CTRL_OFFSET: u32 = 0;
    pub const HP_MEM_AUX_CTRL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **HP_MEM_AUX_CTRL** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn hp_mem_aux_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::HP_MEM_AUX_CTRL_MASK) >> Self::HP_MEM_AUX_CTRL_OFFSET
    }

    /// Write **HP_MEM_AUX_CTRL** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_hp_mem_aux_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HP_MEM_AUX_CTRL_MASK)
                | ((value << Self::HP_MEM_AUX_CTRL_OFFSET) & Self::HP_MEM_AUX_CTRL_MASK)
        });
    }
}

/// HP_MEM_AUX_CTRL value type for batch read/modify/write operations
///
/// Use `hpmemauxctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hpmemauxctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HpMemAuxCtrlVal {
    bits: u32,
}

impl HpMemAuxCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00002070 }
    }

    /// Read **HP_MEM_AUX_CTRL** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn hp_mem_aux_ctrl(&self) -> u32 {
        (self.bits & HpMemAuxCtrl::HP_MEM_AUX_CTRL_MASK) >> HpMemAuxCtrl::HP_MEM_AUX_CTRL_OFFSET
    }

    /// Set **HP_MEM_AUX_CTRL** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_hp_mem_aux_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HpMemAuxCtrl::HP_MEM_AUX_CTRL_MASK)
                | ((value << HpMemAuxCtrl::HP_MEM_AUX_CTRL_OFFSET)
                    & HpMemAuxCtrl::HP_MEM_AUX_CTRL_MASK),
        }
    }
}

impl Default for HpMemAuxCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl HpMemAuxCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HpMemAuxCtrlVal {
        HpMemAuxCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HpMemAuxCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HpMemAuxCtrlVal) -> HpMemAuxCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_MEM_AUX_CTRL Register
///
/// need_des
///
/// Address offset: 0x0184
/// Reset value: 0x00002070
///
/// ## Fields
///
/// - **LP_MEM_AUX_CTRL** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpMemAuxCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpMemAuxCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpMemAuxCtrl {
    // ======== Field Constants ========

    /// **LP_MEM_AUX_CTRL**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_MEM_AUX_CTRL_OFFSET: u32 = 0;
    pub const LP_MEM_AUX_CTRL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_MEM_AUX_CTRL** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_mem_aux_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::LP_MEM_AUX_CTRL_MASK) >> Self::LP_MEM_AUX_CTRL_OFFSET
    }

    /// Write **LP_MEM_AUX_CTRL** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_mem_aux_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_MEM_AUX_CTRL_MASK)
                | ((value << Self::LP_MEM_AUX_CTRL_OFFSET) & Self::LP_MEM_AUX_CTRL_MASK)
        });
    }
}

/// LP_MEM_AUX_CTRL value type for batch read/modify/write operations
///
/// Use `lpmemauxctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpmemauxctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpMemAuxCtrlVal {
    bits: u32,
}

impl LpMemAuxCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00002070 }
    }

    /// Read **LP_MEM_AUX_CTRL** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_mem_aux_ctrl(&self) -> u32 {
        (self.bits & LpMemAuxCtrl::LP_MEM_AUX_CTRL_MASK) >> LpMemAuxCtrl::LP_MEM_AUX_CTRL_OFFSET
    }

    /// Set **LP_MEM_AUX_CTRL** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_mem_aux_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpMemAuxCtrl::LP_MEM_AUX_CTRL_MASK)
                | ((value << LpMemAuxCtrl::LP_MEM_AUX_CTRL_OFFSET)
                    & LpMemAuxCtrl::LP_MEM_AUX_CTRL_MASK),
        }
    }
}

impl Default for LpMemAuxCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpMemAuxCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpMemAuxCtrlVal {
        LpMemAuxCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpMemAuxCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpMemAuxCtrlVal) -> LpMemAuxCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// HP_ROM_AUX_CTRL Register
///
/// need_des
///
/// Address offset: 0x0188
/// Reset value: 0x00000070
///
/// ## Fields
///
/// - **HP_ROM_AUX_CTRL** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct HpRomAuxCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HpRomAuxCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HpRomAuxCtrl {
    // ======== Field Constants ========

    /// **HP_ROM_AUX_CTRL**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const HP_ROM_AUX_CTRL_OFFSET: u32 = 0;
    pub const HP_ROM_AUX_CTRL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **HP_ROM_AUX_CTRL** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn hp_rom_aux_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::HP_ROM_AUX_CTRL_MASK) >> Self::HP_ROM_AUX_CTRL_OFFSET
    }

    /// Write **HP_ROM_AUX_CTRL** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_hp_rom_aux_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HP_ROM_AUX_CTRL_MASK)
                | ((value << Self::HP_ROM_AUX_CTRL_OFFSET) & Self::HP_ROM_AUX_CTRL_MASK)
        });
    }
}

/// HP_ROM_AUX_CTRL value type for batch read/modify/write operations
///
/// Use `hpromauxctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hpromauxctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HpRomAuxCtrlVal {
    bits: u32,
}

impl HpRomAuxCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00000070 }
    }

    /// Read **HP_ROM_AUX_CTRL** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn hp_rom_aux_ctrl(&self) -> u32 {
        (self.bits & HpRomAuxCtrl::HP_ROM_AUX_CTRL_MASK) >> HpRomAuxCtrl::HP_ROM_AUX_CTRL_OFFSET
    }

    /// Set **HP_ROM_AUX_CTRL** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_hp_rom_aux_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HpRomAuxCtrl::HP_ROM_AUX_CTRL_MASK)
                | ((value << HpRomAuxCtrl::HP_ROM_AUX_CTRL_OFFSET)
                    & HpRomAuxCtrl::HP_ROM_AUX_CTRL_MASK),
        }
    }
}

impl Default for HpRomAuxCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl HpRomAuxCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HpRomAuxCtrlVal {
        HpRomAuxCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HpRomAuxCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HpRomAuxCtrlVal) -> HpRomAuxCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_ROM_AUX_CTRL Register
///
/// need_des
///
/// Address offset: 0x018c
/// Reset value: 0x00000070
///
/// ## Fields
///
/// - **LP_ROM_AUX_CTRL** `[31:0]` (RW)
///
///   need_des
#[repr(transparent)]
pub struct LpRomAuxCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpRomAuxCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpRomAuxCtrl {
    // ======== Field Constants ========

    /// **LP_ROM_AUX_CTRL**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_ROM_AUX_CTRL_OFFSET: u32 = 0;
    pub const LP_ROM_AUX_CTRL_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_ROM_AUX_CTRL** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_rom_aux_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::LP_ROM_AUX_CTRL_MASK) >> Self::LP_ROM_AUX_CTRL_OFFSET
    }

    /// Write **LP_ROM_AUX_CTRL** `[31:0]` (RW)
    ///
    /// need_des
    #[inline(always)]
    pub fn write_lp_rom_aux_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_ROM_AUX_CTRL_MASK)
                | ((value << Self::LP_ROM_AUX_CTRL_OFFSET) & Self::LP_ROM_AUX_CTRL_MASK)
        });
    }
}

/// LP_ROM_AUX_CTRL value type for batch read/modify/write operations
///
/// Use `lpromauxctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpromauxctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpRomAuxCtrlVal {
    bits: u32,
}

impl LpRomAuxCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0x00000070 }
    }

    /// Read **LP_ROM_AUX_CTRL** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_rom_aux_ctrl(&self) -> u32 {
        (self.bits & LpRomAuxCtrl::LP_ROM_AUX_CTRL_MASK) >> LpRomAuxCtrl::LP_ROM_AUX_CTRL_OFFSET
    }

    /// Set **LP_ROM_AUX_CTRL** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn with_lp_rom_aux_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpRomAuxCtrl::LP_ROM_AUX_CTRL_MASK)
                | ((value << LpRomAuxCtrl::LP_ROM_AUX_CTRL_OFFSET)
                    & LpRomAuxCtrl::LP_ROM_AUX_CTRL_MASK),
        }
    }
}

impl Default for LpRomAuxCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpRomAuxCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpRomAuxCtrlVal {
        LpRomAuxCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpRomAuxCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpRomAuxCtrlVal) -> LpRomAuxCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_CPU_DBG_PC Register
///
/// need_des
///
/// Address offset: 0x0190
///
/// ## Fields
///
/// - **LP_CPU_DBG_PC** `[31:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct LpCpuDbgPc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpCpuDbgPc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpCpuDbgPc {
    // ======== Field Constants ========

    /// **LP_CPU_DBG_PC**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_CPU_DBG_PC_OFFSET: u32 = 0;
    pub const LP_CPU_DBG_PC_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_CPU_DBG_PC** `[31:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_cpu_dbg_pc(&self) -> u32 {
        (self.value.read_bits() & Self::LP_CPU_DBG_PC_MASK) >> Self::LP_CPU_DBG_PC_OFFSET
    }
}

/// LP_CPU_DBG_PC value type for batch read/modify/write operations
///
/// Use `lpcpudbgpc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpcpudbgpc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpCpuDbgPcVal {
    bits: u32,
}

impl LpCpuDbgPcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_CPU_DBG_PC** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_cpu_dbg_pc(&self) -> u32 {
        (self.bits & LpCpuDbgPc::LP_CPU_DBG_PC_MASK) >> LpCpuDbgPc::LP_CPU_DBG_PC_OFFSET
    }
}

impl Default for LpCpuDbgPcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpCpuDbgPc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpCpuDbgPcVal {
        LpCpuDbgPcVal::from_bits(self.value.read_bits())
    }
}

/// LP_CPU_EXC_PC Register
///
/// need_des
///
/// Address offset: 0x0194
///
/// ## Fields
///
/// - **LP_CPU_EXC_PC** `[31:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct LpCpuExcPc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpCpuExcPc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpCpuExcPc {
    // ======== Field Constants ========

    /// **LP_CPU_EXC_PC**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const LP_CPU_EXC_PC_OFFSET: u32 = 0;
    pub const LP_CPU_EXC_PC_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **LP_CPU_EXC_PC** `[31:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn lp_cpu_exc_pc(&self) -> u32 {
        (self.value.read_bits() & Self::LP_CPU_EXC_PC_MASK) >> Self::LP_CPU_EXC_PC_OFFSET
    }
}

/// LP_CPU_EXC_PC value type for batch read/modify/write operations
///
/// Use `lpcpuexcpc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpcpuexcpc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpCpuExcPcVal {
    bits: u32,
}

impl LpCpuExcPcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_CPU_EXC_PC** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn lp_cpu_exc_pc(&self) -> u32 {
        (self.bits & LpCpuExcPc::LP_CPU_EXC_PC_MASK) >> LpCpuExcPc::LP_CPU_EXC_PC_OFFSET
    }
}

impl Default for LpCpuExcPcVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpCpuExcPc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpCpuExcPcVal {
        LpCpuExcPcVal::from_bits(self.value.read_bits())
    }
}

/// IDBUS_ADDRHOLE_ADDR Register
///
/// need_des
///
/// Address offset: 0x0198
///
/// ## Fields
///
/// - **IDBUS_ADDRHOLE_ADDR** `[31:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct IdbusAddrholeAddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IdbusAddrholeAddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IdbusAddrholeAddr {
    // ======== Field Constants ========

    /// **IDBUS_ADDRHOLE_ADDR**
    ///
    /// need_des
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const IDBUS_ADDRHOLE_ADDR_OFFSET: u32 = 0;
    pub const IDBUS_ADDRHOLE_ADDR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **IDBUS_ADDRHOLE_ADDR** `[31:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn idbus_addrhole_addr(&self) -> u32 {
        (self.value.read_bits() & Self::IDBUS_ADDRHOLE_ADDR_MASK)
            >> Self::IDBUS_ADDRHOLE_ADDR_OFFSET
    }
}

/// IDBUS_ADDRHOLE_ADDR value type for batch read/modify/write operations
///
/// Use `idbusaddrholeaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `idbusaddrholeaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IdbusAddrholeAddrVal {
    bits: u32,
}

impl IdbusAddrholeAddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **IDBUS_ADDRHOLE_ADDR** `[31:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn idbus_addrhole_addr(&self) -> u32 {
        (self.bits & IdbusAddrholeAddr::IDBUS_ADDRHOLE_ADDR_MASK)
            >> IdbusAddrholeAddr::IDBUS_ADDRHOLE_ADDR_OFFSET
    }
}

impl Default for IdbusAddrholeAddrVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IdbusAddrholeAddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IdbusAddrholeAddrVal {
        IdbusAddrholeAddrVal::from_bits(self.value.read_bits())
    }
}

/// IDBUS_ADDRHOLE_INFO Register
///
/// need_des
///
/// Address offset: 0x019c
///
/// ## Fields
///
/// - **IDBUS_ADDRHOLE_SECURE** `[6]` (RO)
///
///   need_des
///
/// - **IDBUS_ADDRHOLE_WR** `[5]` (RO)
///
///   need_des
///
/// - **IDBUS_ADDRHOLE_ID** `[4:0]` (RO)
///
///   need_des
#[repr(transparent)]
pub struct IdbusAddrholeInfo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for IdbusAddrholeInfo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl IdbusAddrholeInfo {
    // ======== Field Constants ========

    /// **IDBUS_ADDRHOLE_SECURE**
    ///
    /// need_des
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const IDBUS_ADDRHOLE_SECURE_OFFSET: u32 = 6;
    pub const IDBUS_ADDRHOLE_SECURE_MASK: u32 = 0x00000040;
    pub const IDBUS_ADDRHOLE_SECURE: u32 = 1 << 6;

    /// **IDBUS_ADDRHOLE_WR**
    ///
    /// need_des
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const IDBUS_ADDRHOLE_WR_OFFSET: u32 = 5;
    pub const IDBUS_ADDRHOLE_WR_MASK: u32 = 0x00000020;
    pub const IDBUS_ADDRHOLE_WR: u32 = 1 << 5;

    /// **IDBUS_ADDRHOLE_ID**
    ///
    /// need_des
    ///
    /// Bits `[4:0]`, mask `0x0000001f`
    pub const IDBUS_ADDRHOLE_ID_OFFSET: u32 = 0;
    pub const IDBUS_ADDRHOLE_ID_MASK: u32 = 0x0000001f;

    // ======== Field Accessors ========

    /// **IDBUS_ADDRHOLE_SECURE** `[6]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn idbus_addrhole_secure(&self) -> bool {
        (self.value.read_bits() & Self::IDBUS_ADDRHOLE_SECURE_MASK) != 0
    }

    /// **IDBUS_ADDRHOLE_WR** `[5]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn idbus_addrhole_wr(&self) -> bool {
        (self.value.read_bits() & Self::IDBUS_ADDRHOLE_WR_MASK) != 0
    }

    /// **IDBUS_ADDRHOLE_ID** `[4:0]` (RO)
    ///
    /// need_des
    #[inline(always)]
    pub fn idbus_addrhole_id(&self) -> u32 {
        (self.value.read_bits() & Self::IDBUS_ADDRHOLE_ID_MASK) >> Self::IDBUS_ADDRHOLE_ID_OFFSET
    }
}

/// IDBUS_ADDRHOLE_INFO value type for batch read/modify/write operations
///
/// Use `idbusaddrholeinfo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `idbusaddrholeinfo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct IdbusAddrholeInfoVal {
    bits: u32,
}

impl IdbusAddrholeInfoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **IDBUS_ADDRHOLE_SECURE** `[6:6]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn idbus_addrhole_secure(&self) -> bool {
        (self.bits & IdbusAddrholeInfo::IDBUS_ADDRHOLE_SECURE_MASK) != 0
    }

    /// Read **IDBUS_ADDRHOLE_WR** `[5:5]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn idbus_addrhole_wr(&self) -> bool {
        (self.bits & IdbusAddrholeInfo::IDBUS_ADDRHOLE_WR_MASK) != 0
    }

    /// Read **IDBUS_ADDRHOLE_ID** `[4:0]`
    ///
    /// need_des
    #[inline(always)]
    pub const fn idbus_addrhole_id(&self) -> u32 {
        (self.bits & IdbusAddrholeInfo::IDBUS_ADDRHOLE_ID_MASK)
            >> IdbusAddrholeInfo::IDBUS_ADDRHOLE_ID_OFFSET
    }
}

impl Default for IdbusAddrholeInfoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl IdbusAddrholeInfo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> IdbusAddrholeInfoVal {
        IdbusAddrholeInfoVal::from_bits(self.value.read_bits())
    }
}

/// HP_POR_RST_BYPASS_CTRL Register
///
/// need_des
///
/// Address offset: 0x01a0
/// Reset value: 0xff00ff00
///
/// ## Fields
///
/// - **HP_PO_RSTN_BYPASS_CTRL** `[31:24]` (RW)
///
///   [31] 1'b1: po_rstn bypass sys_sw_rstn
///   [30] 1'b1: po_rstn bypass hp_wdt_sys_rstn
///   [29] 1'b1: po_rstn bypass hp_cpu_intrusion_rstn
///   [28] 1'b1: po_rstn bypass hp_sdio_sys_rstn
///   [27] 1'b1: po_rstn bypass usb_jtag_chip_rst
///   [26] 1'b1: po_rstn bypass usb_uart_chip_rst
///   [25] 1'b1: po_rstn bypass lp_wdt_hp_sys_rstn
///   [24] 1'b1: po_rstn bypass efuse_err_rstn
///
/// - **HP_PO_CNNT_RSTN_BYPASS_CTRL** `[15:8]` (RW)
///
///   [15] 1'b1: po_cnnt_rstn bypass sys_sw_rstn
///   [14] 1'b1: po_cnnt_rstn bypass hp_wdt_sys_rstn
///   [13] 1'b1: po_cnnt_rstn bypass hp_cpu_intrusion_rstn
///   [12] 1'b1: po_cnnt_rstn bypass hp_sdio_sys_rstn
///   [11] 1'b1: po_cnnt_rstn bypass usb_jtag_chip_rst
///   [10] 1'b1: po_cnnt_rstn bypass usb_uart_chip_rst
///   [9] 1'b1: po_cnnt_rstn bypass lp_wdt_hp_sys_rstn
///   [8] 1'b1: po_cnnt_rstn bypass efuse_err_rstn
#[repr(transparent)]
pub struct HpPorRstBypassCtrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for HpPorRstBypassCtrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HpPorRstBypassCtrl {
    // ======== Field Constants ========

    /// **HP_PO_RSTN_BYPASS_CTRL**
    ///
    /// [31] 1'b1: po_rstn bypass sys_sw_rstn
    /// [30] 1'b1: po_rstn bypass hp_wdt_sys_rstn
    /// [29] 1'b1: po_rstn bypass hp_cpu_intrusion_rstn
    /// [28] 1'b1: po_rstn bypass hp_sdio_sys_rstn
    /// [27] 1'b1: po_rstn bypass usb_jtag_chip_rst
    /// [26] 1'b1: po_rstn bypass usb_uart_chip_rst
    /// [25] 1'b1: po_rstn bypass lp_wdt_hp_sys_rstn
    /// [24] 1'b1: po_rstn bypass efuse_err_rstn
    ///
    /// Bits `[31:24]`, mask `0xff000000`
    pub const HP_PO_RSTN_BYPASS_CTRL_OFFSET: u32 = 24;
    pub const HP_PO_RSTN_BYPASS_CTRL_MASK: u32 = 0xff000000;

    /// **HP_PO_CNNT_RSTN_BYPASS_CTRL**
    ///
    /// [15] 1'b1: po_cnnt_rstn bypass sys_sw_rstn
    /// [14] 1'b1: po_cnnt_rstn bypass hp_wdt_sys_rstn
    /// [13] 1'b1: po_cnnt_rstn bypass hp_cpu_intrusion_rstn
    /// [12] 1'b1: po_cnnt_rstn bypass hp_sdio_sys_rstn
    /// [11] 1'b1: po_cnnt_rstn bypass usb_jtag_chip_rst
    /// [10] 1'b1: po_cnnt_rstn bypass usb_uart_chip_rst
    /// [9] 1'b1: po_cnnt_rstn bypass lp_wdt_hp_sys_rstn
    /// [8] 1'b1: po_cnnt_rstn bypass efuse_err_rstn
    ///
    /// Bits `[15:8]`, mask `0x0000ff00`
    pub const HP_PO_CNNT_RSTN_BYPASS_CTRL_OFFSET: u32 = 8;
    pub const HP_PO_CNNT_RSTN_BYPASS_CTRL_MASK: u32 = 0x0000ff00;

    // ======== Field Accessors ========

    /// **HP_PO_RSTN_BYPASS_CTRL** `[31:24]` (RW)
    ///
    /// [31] 1'b1: po_rstn bypass sys_sw_rstn
    /// [30] 1'b1: po_rstn bypass hp_wdt_sys_rstn
    /// [29] 1'b1: po_rstn bypass hp_cpu_intrusion_rstn
    /// [28] 1'b1: po_rstn bypass hp_sdio_sys_rstn
    /// [27] 1'b1: po_rstn bypass usb_jtag_chip_rst
    /// [26] 1'b1: po_rstn bypass usb_uart_chip_rst
    /// [25] 1'b1: po_rstn bypass lp_wdt_hp_sys_rstn
    /// [24] 1'b1: po_rstn bypass efuse_err_rstn
    #[inline(always)]
    pub fn hp_po_rstn_bypass_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::HP_PO_RSTN_BYPASS_CTRL_MASK)
            >> Self::HP_PO_RSTN_BYPASS_CTRL_OFFSET
    }

    /// Write **HP_PO_RSTN_BYPASS_CTRL** `[31:24]` (RW)
    ///
    /// [31] 1'b1: po_rstn bypass sys_sw_rstn
    /// [30] 1'b1: po_rstn bypass hp_wdt_sys_rstn
    /// [29] 1'b1: po_rstn bypass hp_cpu_intrusion_rstn
    /// [28] 1'b1: po_rstn bypass hp_sdio_sys_rstn
    /// [27] 1'b1: po_rstn bypass usb_jtag_chip_rst
    /// [26] 1'b1: po_rstn bypass usb_uart_chip_rst
    /// [25] 1'b1: po_rstn bypass lp_wdt_hp_sys_rstn
    /// [24] 1'b1: po_rstn bypass efuse_err_rstn
    #[inline(always)]
    pub fn write_hp_po_rstn_bypass_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HP_PO_RSTN_BYPASS_CTRL_MASK)
                | ((value << Self::HP_PO_RSTN_BYPASS_CTRL_OFFSET)
                    & Self::HP_PO_RSTN_BYPASS_CTRL_MASK)
        });
    }

    /// **HP_PO_CNNT_RSTN_BYPASS_CTRL** `[15:8]` (RW)
    ///
    /// [15] 1'b1: po_cnnt_rstn bypass sys_sw_rstn
    /// [14] 1'b1: po_cnnt_rstn bypass hp_wdt_sys_rstn
    /// [13] 1'b1: po_cnnt_rstn bypass hp_cpu_intrusion_rstn
    /// [12] 1'b1: po_cnnt_rstn bypass hp_sdio_sys_rstn
    /// [11] 1'b1: po_cnnt_rstn bypass usb_jtag_chip_rst
    /// [10] 1'b1: po_cnnt_rstn bypass usb_uart_chip_rst
    /// [9] 1'b1: po_cnnt_rstn bypass lp_wdt_hp_sys_rstn
    /// [8] 1'b1: po_cnnt_rstn bypass efuse_err_rstn
    #[inline(always)]
    pub fn hp_po_cnnt_rstn_bypass_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::HP_PO_CNNT_RSTN_BYPASS_CTRL_MASK)
            >> Self::HP_PO_CNNT_RSTN_BYPASS_CTRL_OFFSET
    }

    /// Write **HP_PO_CNNT_RSTN_BYPASS_CTRL** `[15:8]` (RW)
    ///
    /// [15] 1'b1: po_cnnt_rstn bypass sys_sw_rstn
    /// [14] 1'b1: po_cnnt_rstn bypass hp_wdt_sys_rstn
    /// [13] 1'b1: po_cnnt_rstn bypass hp_cpu_intrusion_rstn
    /// [12] 1'b1: po_cnnt_rstn bypass hp_sdio_sys_rstn
    /// [11] 1'b1: po_cnnt_rstn bypass usb_jtag_chip_rst
    /// [10] 1'b1: po_cnnt_rstn bypass usb_uart_chip_rst
    /// [9] 1'b1: po_cnnt_rstn bypass lp_wdt_hp_sys_rstn
    /// [8] 1'b1: po_cnnt_rstn bypass efuse_err_rstn
    #[inline(always)]
    pub fn write_hp_po_cnnt_rstn_bypass_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::HP_PO_CNNT_RSTN_BYPASS_CTRL_MASK)
                | ((value << Self::HP_PO_CNNT_RSTN_BYPASS_CTRL_OFFSET)
                    & Self::HP_PO_CNNT_RSTN_BYPASS_CTRL_MASK)
        });
    }
}

/// HP_POR_RST_BYPASS_CTRL value type for batch read/modify/write operations
///
/// Use `hpporrstbypassctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `hpporrstbypassctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct HpPorRstBypassCtrlVal {
    bits: u32,
}

impl HpPorRstBypassCtrlVal {
    /// Create from raw bits
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
        Self { bits: 0xff00ff00 }
    }

    /// Read **HP_PO_RSTN_BYPASS_CTRL** `[31:24]`
    ///
    /// [31] 1'b1: po_rstn bypass sys_sw_rstn
    /// [30] 1'b1: po_rstn bypass hp_wdt_sys_rstn
    /// [29] 1'b1: po_rstn bypass hp_cpu_intrusion_rstn
    /// [28] 1'b1: po_rstn bypass hp_sdio_sys_rstn
    /// [27] 1'b1: po_rstn bypass usb_jtag_chip_rst
    /// [26] 1'b1: po_rstn bypass usb_uart_chip_rst
    /// [25] 1'b1: po_rstn bypass lp_wdt_hp_sys_rstn
    /// [24] 1'b1: po_rstn bypass efuse_err_rstn
    #[inline(always)]
    pub const fn hp_po_rstn_bypass_ctrl(&self) -> u32 {
        (self.bits & HpPorRstBypassCtrl::HP_PO_RSTN_BYPASS_CTRL_MASK)
            >> HpPorRstBypassCtrl::HP_PO_RSTN_BYPASS_CTRL_OFFSET
    }

    /// Read **HP_PO_CNNT_RSTN_BYPASS_CTRL** `[15:8]`
    ///
    /// [15] 1'b1: po_cnnt_rstn bypass sys_sw_rstn
    /// [14] 1'b1: po_cnnt_rstn bypass hp_wdt_sys_rstn
    /// [13] 1'b1: po_cnnt_rstn bypass hp_cpu_intrusion_rstn
    /// [12] 1'b1: po_cnnt_rstn bypass hp_sdio_sys_rstn
    /// [11] 1'b1: po_cnnt_rstn bypass usb_jtag_chip_rst
    /// [10] 1'b1: po_cnnt_rstn bypass usb_uart_chip_rst
    /// [9] 1'b1: po_cnnt_rstn bypass lp_wdt_hp_sys_rstn
    /// [8] 1'b1: po_cnnt_rstn bypass efuse_err_rstn
    #[inline(always)]
    pub const fn hp_po_cnnt_rstn_bypass_ctrl(&self) -> u32 {
        (self.bits & HpPorRstBypassCtrl::HP_PO_CNNT_RSTN_BYPASS_CTRL_MASK)
            >> HpPorRstBypassCtrl::HP_PO_CNNT_RSTN_BYPASS_CTRL_OFFSET
    }

    /// Set **HP_PO_RSTN_BYPASS_CTRL** `[31:24]`
    ///
    /// [31] 1'b1: po_rstn bypass sys_sw_rstn
    /// [30] 1'b1: po_rstn bypass hp_wdt_sys_rstn
    /// [29] 1'b1: po_rstn bypass hp_cpu_intrusion_rstn
    /// [28] 1'b1: po_rstn bypass hp_sdio_sys_rstn
    /// [27] 1'b1: po_rstn bypass usb_jtag_chip_rst
    /// [26] 1'b1: po_rstn bypass usb_uart_chip_rst
    /// [25] 1'b1: po_rstn bypass lp_wdt_hp_sys_rstn
    /// [24] 1'b1: po_rstn bypass efuse_err_rstn
    #[inline(always)]
    pub const fn with_hp_po_rstn_bypass_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HpPorRstBypassCtrl::HP_PO_RSTN_BYPASS_CTRL_MASK)
                | ((value << HpPorRstBypassCtrl::HP_PO_RSTN_BYPASS_CTRL_OFFSET)
                    & HpPorRstBypassCtrl::HP_PO_RSTN_BYPASS_CTRL_MASK),
        }
    }

    /// Set **HP_PO_CNNT_RSTN_BYPASS_CTRL** `[15:8]`
    ///
    /// [15] 1'b1: po_cnnt_rstn bypass sys_sw_rstn
    /// [14] 1'b1: po_cnnt_rstn bypass hp_wdt_sys_rstn
    /// [13] 1'b1: po_cnnt_rstn bypass hp_cpu_intrusion_rstn
    /// [12] 1'b1: po_cnnt_rstn bypass hp_sdio_sys_rstn
    /// [11] 1'b1: po_cnnt_rstn bypass usb_jtag_chip_rst
    /// [10] 1'b1: po_cnnt_rstn bypass usb_uart_chip_rst
    /// [9] 1'b1: po_cnnt_rstn bypass lp_wdt_hp_sys_rstn
    /// [8] 1'b1: po_cnnt_rstn bypass efuse_err_rstn
    #[inline(always)]
    pub const fn with_hp_po_cnnt_rstn_bypass_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !HpPorRstBypassCtrl::HP_PO_CNNT_RSTN_BYPASS_CTRL_MASK)
                | ((value << HpPorRstBypassCtrl::HP_PO_CNNT_RSTN_BYPASS_CTRL_OFFSET)
                    & HpPorRstBypassCtrl::HP_PO_CNNT_RSTN_BYPASS_CTRL_MASK),
        }
    }
}

impl Default for HpPorRstBypassCtrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl HpPorRstBypassCtrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> HpPorRstBypassCtrlVal {
        HpPorRstBypassCtrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: HpPorRstBypassCtrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(HpPorRstBypassCtrlVal) -> HpPorRstBypassCtrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RNG_DATA Register
///
/// rng data register
///
/// Address offset: 0x01a4
///
/// ## Fields
///
/// - **RND_DATA** `[31:0]` (RO)
///
///   result of rng output
#[repr(transparent)]
pub struct RngData {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RngData {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RngData {
    // ======== Field Constants ========

    /// **RND_DATA**
    ///
    /// result of rng output
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RND_DATA_OFFSET: u32 = 0;
    pub const RND_DATA_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RND_DATA** `[31:0]` (RO)
    ///
    /// result of rng output
    #[inline(always)]
    pub fn rnd_data(&self) -> u32 {
        (self.value.read_bits() & Self::RND_DATA_MASK) >> Self::RND_DATA_OFFSET
    }
}

/// RNG_DATA value type for batch read/modify/write operations
///
/// Use `rngdata.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `rngdata.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RngDataVal {
    bits: u32,
}

impl RngDataVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **RND_DATA** `[31:0]`
    ///
    /// result of rng output
    #[inline(always)]
    pub const fn rnd_data(&self) -> u32 {
        (self.bits & RngData::RND_DATA_MASK) >> RngData::RND_DATA_OFFSET
    }
}

impl Default for RngDataVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RngData {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RngDataVal {
        RngDataVal::from_bits(self.value.read_bits())
    }
}

/// LP_CORE_AHB_TIMEOUT Register
///
/// need_des
///
/// Address offset: 0x01b0
/// Reset value: 0x007fffff
///
/// ## Fields
///
/// - **LP2HP_AHB_TIMEOUT_THRES** `[22:18]` (RW)
///
///   This field used to set lp2hp ahb bus timeout threshold
///
/// - **LP2HP_AHB_TIMEOUT_EN** `[17]` (RW)
///
///   set this field to 1 to enable lp2hp ahb timeout handle
///
/// - **THRES** `[16:1]` (RW)
///
///   This field used to set lp core ahb bus timeout threshold
///
/// - **EN** `[0]` (RW)
///
///   set this field to 1 to enable lp core ahb timeout handle
#[repr(transparent)]
pub struct LpCoreAhbTimeout {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpCoreAhbTimeout {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpCoreAhbTimeout {
    // ======== Field Constants ========

    /// **LP2HP_AHB_TIMEOUT_THRES**
    ///
    /// This field used to set lp2hp ahb bus timeout threshold
    ///
    /// Bits `[22:18]`, mask `0x007c0000`
    pub const LP2HP_AHB_TIMEOUT_THRES_OFFSET: u32 = 18;
    pub const LP2HP_AHB_TIMEOUT_THRES_MASK: u32 = 0x007c0000;

    /// **LP2HP_AHB_TIMEOUT_EN**
    ///
    /// set this field to 1 to enable lp2hp ahb timeout handle
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const LP2HP_AHB_TIMEOUT_EN_OFFSET: u32 = 17;
    pub const LP2HP_AHB_TIMEOUT_EN_MASK: u32 = 0x00020000;
    pub const LP2HP_AHB_TIMEOUT_EN: u32 = 1 << 17;

    /// **THRES**
    ///
    /// This field used to set lp core ahb bus timeout threshold
    ///
    /// Bits `[16:1]`, mask `0x0001fffe`
    pub const THRES_OFFSET: u32 = 1;
    pub const THRES_MASK: u32 = 0x0001fffe;

    /// **EN**
    ///
    /// set this field to 1 to enable lp core ahb timeout handle
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LP2HP_AHB_TIMEOUT_THRES** `[22:18]` (RW)
    ///
    /// This field used to set lp2hp ahb bus timeout threshold
    #[inline(always)]
    pub fn lp2hp_ahb_timeout_thres(&self) -> u32 {
        (self.value.read_bits() & Self::LP2HP_AHB_TIMEOUT_THRES_MASK)
            >> Self::LP2HP_AHB_TIMEOUT_THRES_OFFSET
    }

    /// Write **LP2HP_AHB_TIMEOUT_THRES** `[22:18]` (RW)
    ///
    /// This field used to set lp2hp ahb bus timeout threshold
    #[inline(always)]
    pub fn write_lp2hp_ahb_timeout_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP2HP_AHB_TIMEOUT_THRES_MASK)
                | ((value << Self::LP2HP_AHB_TIMEOUT_THRES_OFFSET)
                    & Self::LP2HP_AHB_TIMEOUT_THRES_MASK)
        });
    }

    /// **LP2HP_AHB_TIMEOUT_EN** `[17]` (RW)
    ///
    /// set this field to 1 to enable lp2hp ahb timeout handle
    #[inline(always)]
    pub fn lp2hp_ahb_timeout_en(&self) -> bool {
        (self.value.read_bits() & Self::LP2HP_AHB_TIMEOUT_EN_MASK) != 0
    }

    /// Set LP2HP_AHB_TIMEOUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lp2hp_ahb_timeout_en(&self) {
        self.value
            .modify_bits(|r| r | Self::LP2HP_AHB_TIMEOUT_EN_MASK);
    }

    /// Clear LP2HP_AHB_TIMEOUT_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lp2hp_ahb_timeout_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::LP2HP_AHB_TIMEOUT_EN_MASK);
    }

    /// **THRES** `[16:1]` (RW)
    ///
    /// This field used to set lp core ahb bus timeout threshold
    #[inline(always)]
    pub fn thres(&self) -> u32 {
        (self.value.read_bits() & Self::THRES_MASK) >> Self::THRES_OFFSET
    }

    /// Write **THRES** `[16:1]` (RW)
    ///
    /// This field used to set lp core ahb bus timeout threshold
    #[inline(always)]
    pub fn write_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::THRES_MASK) | ((value << Self::THRES_OFFSET) & Self::THRES_MASK)
        });
    }

    /// **EN** `[0]` (RW)
    ///
    /// set this field to 1 to enable lp core ahb timeout handle
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

/// LP_CORE_AHB_TIMEOUT value type for batch read/modify/write operations
///
/// Use `lpcoreahbtimeout.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpcoreahbtimeout.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpCoreAhbTimeoutVal {
    bits: u32,
}

impl LpCoreAhbTimeoutVal {
    /// Create from raw bits
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
        Self { bits: 0x007fffff }
    }

    /// Read **LP2HP_AHB_TIMEOUT_THRES** `[22:18]`
    ///
    /// This field used to set lp2hp ahb bus timeout threshold
    #[inline(always)]
    pub const fn lp2hp_ahb_timeout_thres(&self) -> u32 {
        (self.bits & LpCoreAhbTimeout::LP2HP_AHB_TIMEOUT_THRES_MASK)
            >> LpCoreAhbTimeout::LP2HP_AHB_TIMEOUT_THRES_OFFSET
    }

    /// Read **LP2HP_AHB_TIMEOUT_EN** `[17:17]`
    ///
    /// set this field to 1 to enable lp2hp ahb timeout handle
    #[inline(always)]
    pub const fn lp2hp_ahb_timeout_en(&self) -> bool {
        (self.bits & LpCoreAhbTimeout::LP2HP_AHB_TIMEOUT_EN_MASK) != 0
    }

    /// Read **THRES** `[16:1]`
    ///
    /// This field used to set lp core ahb bus timeout threshold
    #[inline(always)]
    pub const fn thres(&self) -> u32 {
        (self.bits & LpCoreAhbTimeout::THRES_MASK) >> LpCoreAhbTimeout::THRES_OFFSET
    }

    /// Read **EN** `[0:0]`
    ///
    /// set this field to 1 to enable lp core ahb timeout handle
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & LpCoreAhbTimeout::EN_MASK) != 0
    }

    /// Set **LP2HP_AHB_TIMEOUT_THRES** `[22:18]`
    ///
    /// This field used to set lp2hp ahb bus timeout threshold
    #[inline(always)]
    pub const fn with_lp2hp_ahb_timeout_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpCoreAhbTimeout::LP2HP_AHB_TIMEOUT_THRES_MASK)
                | ((value << LpCoreAhbTimeout::LP2HP_AHB_TIMEOUT_THRES_OFFSET)
                    & LpCoreAhbTimeout::LP2HP_AHB_TIMEOUT_THRES_MASK),
        }
    }

    /// Set **LP2HP_AHB_TIMEOUT_EN** `[17:17]`
    ///
    /// set this field to 1 to enable lp2hp ahb timeout handle
    #[inline(always)]
    pub const fn with_lp2hp_ahb_timeout_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpCoreAhbTimeout::LP2HP_AHB_TIMEOUT_EN_MASK
            } else {
                self.bits & !LpCoreAhbTimeout::LP2HP_AHB_TIMEOUT_EN_MASK
            },
        }
    }

    /// Set **THRES** `[16:1]`
    ///
    /// This field used to set lp core ahb bus timeout threshold
    #[inline(always)]
    pub const fn with_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpCoreAhbTimeout::THRES_MASK)
                | ((value << LpCoreAhbTimeout::THRES_OFFSET) & LpCoreAhbTimeout::THRES_MASK),
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// set this field to 1 to enable lp core ahb timeout handle
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpCoreAhbTimeout::EN_MASK
            } else {
                self.bits & !LpCoreAhbTimeout::EN_MASK
            },
        }
    }
}

impl Default for LpCoreAhbTimeoutVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpCoreAhbTimeout {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpCoreAhbTimeoutVal {
        LpCoreAhbTimeoutVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpCoreAhbTimeoutVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpCoreAhbTimeoutVal) -> LpCoreAhbTimeoutVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_CORE_IBUS_TIMEOUT Register
///
/// need_des
///
/// Address offset: 0x01b4
/// Reset value: 0x0001ffff
///
/// ## Fields
///
/// - **THRES** `[16:1]` (RW)
///
///   This field used to set lp core ibus timeout threshold
///
/// - **EN** `[0]` (RW)
///
///   set this field to 1 to enable lp core ibus timeout handle
#[repr(transparent)]
pub struct LpCoreIbusTimeout {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpCoreIbusTimeout {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpCoreIbusTimeout {
    // ======== Field Constants ========

    /// **THRES**
    ///
    /// This field used to set lp core ibus timeout threshold
    ///
    /// Bits `[16:1]`, mask `0x0001fffe`
    pub const THRES_OFFSET: u32 = 1;
    pub const THRES_MASK: u32 = 0x0001fffe;

    /// **EN**
    ///
    /// set this field to 1 to enable lp core ibus timeout handle
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **THRES** `[16:1]` (RW)
    ///
    /// This field used to set lp core ibus timeout threshold
    #[inline(always)]
    pub fn thres(&self) -> u32 {
        (self.value.read_bits() & Self::THRES_MASK) >> Self::THRES_OFFSET
    }

    /// Write **THRES** `[16:1]` (RW)
    ///
    /// This field used to set lp core ibus timeout threshold
    #[inline(always)]
    pub fn write_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::THRES_MASK) | ((value << Self::THRES_OFFSET) & Self::THRES_MASK)
        });
    }

    /// **EN** `[0]` (RW)
    ///
    /// set this field to 1 to enable lp core ibus timeout handle
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

/// LP_CORE_IBUS_TIMEOUT value type for batch read/modify/write operations
///
/// Use `lpcoreibustimeout.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpcoreibustimeout.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpCoreIbusTimeoutVal {
    bits: u32,
}

impl LpCoreIbusTimeoutVal {
    /// Create from raw bits
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
    /// This field used to set lp core ibus timeout threshold
    #[inline(always)]
    pub const fn thres(&self) -> u32 {
        (self.bits & LpCoreIbusTimeout::THRES_MASK) >> LpCoreIbusTimeout::THRES_OFFSET
    }

    /// Read **EN** `[0:0]`
    ///
    /// set this field to 1 to enable lp core ibus timeout handle
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & LpCoreIbusTimeout::EN_MASK) != 0
    }

    /// Set **THRES** `[16:1]`
    ///
    /// This field used to set lp core ibus timeout threshold
    #[inline(always)]
    pub const fn with_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpCoreIbusTimeout::THRES_MASK)
                | ((value << LpCoreIbusTimeout::THRES_OFFSET) & LpCoreIbusTimeout::THRES_MASK),
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// set this field to 1 to enable lp core ibus timeout handle
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpCoreIbusTimeout::EN_MASK
            } else {
                self.bits & !LpCoreIbusTimeout::EN_MASK
            },
        }
    }
}

impl Default for LpCoreIbusTimeoutVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpCoreIbusTimeout {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpCoreIbusTimeoutVal {
        LpCoreIbusTimeoutVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpCoreIbusTimeoutVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpCoreIbusTimeoutVal) -> LpCoreIbusTimeoutVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_CORE_DBUS_TIMEOUT Register
///
/// need_des
///
/// Address offset: 0x01b8
/// Reset value: 0x0001ffff
///
/// ## Fields
///
/// - **THRES** `[16:1]` (RW)
///
///   This field used to set lp core dbus timeout threshold
///
/// - **EN** `[0]` (RW)
///
///   set this field to 1 to enable lp core dbus timeout handle
#[repr(transparent)]
pub struct LpCoreDbusTimeout {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpCoreDbusTimeout {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpCoreDbusTimeout {
    // ======== Field Constants ========

    /// **THRES**
    ///
    /// This field used to set lp core dbus timeout threshold
    ///
    /// Bits `[16:1]`, mask `0x0001fffe`
    pub const THRES_OFFSET: u32 = 1;
    pub const THRES_MASK: u32 = 0x0001fffe;

    /// **EN**
    ///
    /// set this field to 1 to enable lp core dbus timeout handle
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const EN_OFFSET: u32 = 0;
    pub const EN_MASK: u32 = 0x00000001;
    pub const EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **THRES** `[16:1]` (RW)
    ///
    /// This field used to set lp core dbus timeout threshold
    #[inline(always)]
    pub fn thres(&self) -> u32 {
        (self.value.read_bits() & Self::THRES_MASK) >> Self::THRES_OFFSET
    }

    /// Write **THRES** `[16:1]` (RW)
    ///
    /// This field used to set lp core dbus timeout threshold
    #[inline(always)]
    pub fn write_thres(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::THRES_MASK) | ((value << Self::THRES_OFFSET) & Self::THRES_MASK)
        });
    }

    /// **EN** `[0]` (RW)
    ///
    /// set this field to 1 to enable lp core dbus timeout handle
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

/// LP_CORE_DBUS_TIMEOUT value type for batch read/modify/write operations
///
/// Use `lpcoredbustimeout.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpcoredbustimeout.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpCoreDbusTimeoutVal {
    bits: u32,
}

impl LpCoreDbusTimeoutVal {
    /// Create from raw bits
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
    /// This field used to set lp core dbus timeout threshold
    #[inline(always)]
    pub const fn thres(&self) -> u32 {
        (self.bits & LpCoreDbusTimeout::THRES_MASK) >> LpCoreDbusTimeout::THRES_OFFSET
    }

    /// Read **EN** `[0:0]`
    ///
    /// set this field to 1 to enable lp core dbus timeout handle
    #[inline(always)]
    pub const fn en(&self) -> bool {
        (self.bits & LpCoreDbusTimeout::EN_MASK) != 0
    }

    /// Set **THRES** `[16:1]`
    ///
    /// This field used to set lp core dbus timeout threshold
    #[inline(always)]
    pub const fn with_thres(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpCoreDbusTimeout::THRES_MASK)
                | ((value << LpCoreDbusTimeout::THRES_OFFSET) & LpCoreDbusTimeout::THRES_MASK),
        }
    }

    /// Set **EN** `[0:0]`
    ///
    /// set this field to 1 to enable lp core dbus timeout handle
    #[inline(always)]
    pub const fn with_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | LpCoreDbusTimeout::EN_MASK
            } else {
                self.bits & !LpCoreDbusTimeout::EN_MASK
            },
        }
    }
}

impl Default for LpCoreDbusTimeoutVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl LpCoreDbusTimeout {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpCoreDbusTimeoutVal {
        LpCoreDbusTimeoutVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpCoreDbusTimeoutVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpCoreDbusTimeoutVal) -> LpCoreDbusTimeoutVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// LP_CORE_ERR_RESP_DIS Register
///
/// need_des
///
/// Address offset: 0x01bc
///
/// ## Fields
///
/// - **LP_CORE_ERR_RESP_DIS** `[2:0]` (RW)
///
///   Set bit0 to disable ibus err resp;Set bit1 to disable dbus err resp; Set bit 2 to disable ahb err resp.
#[repr(transparent)]
pub struct LpCoreErrRespDis {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for LpCoreErrRespDis {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl LpCoreErrRespDis {
    // ======== Field Constants ========

    /// **LP_CORE_ERR_RESP_DIS**
    ///
    /// Set bit0 to disable ibus err resp;Set bit1 to disable dbus err resp; Set bit 2 to disable ahb err resp.
    ///
    /// Bits `[2:0]`, mask `0x00000007`
    pub const LP_CORE_ERR_RESP_DIS_OFFSET: u32 = 0;
    pub const LP_CORE_ERR_RESP_DIS_MASK: u32 = 0x00000007;

    // ======== Field Accessors ========

    /// **LP_CORE_ERR_RESP_DIS** `[2:0]` (RW)
    ///
    /// Set bit0 to disable ibus err resp;Set bit1 to disable dbus err resp; Set bit 2 to disable ahb err resp.
    #[inline(always)]
    pub fn lp_core_err_resp_dis(&self) -> u32 {
        (self.value.read_bits() & Self::LP_CORE_ERR_RESP_DIS_MASK)
            >> Self::LP_CORE_ERR_RESP_DIS_OFFSET
    }

    /// Write **LP_CORE_ERR_RESP_DIS** `[2:0]` (RW)
    ///
    /// Set bit0 to disable ibus err resp;Set bit1 to disable dbus err resp; Set bit 2 to disable ahb err resp.
    #[inline(always)]
    pub fn write_lp_core_err_resp_dis(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LP_CORE_ERR_RESP_DIS_MASK)
                | ((value << Self::LP_CORE_ERR_RESP_DIS_OFFSET) & Self::LP_CORE_ERR_RESP_DIS_MASK)
        });
    }
}

/// LP_CORE_ERR_RESP_DIS value type for batch read/modify/write operations
///
/// Use `lpcoreerrrespdis.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `lpcoreerrrespdis.write_val(val)`.
#[derive(Clone, Copy)]
pub struct LpCoreErrRespDisVal {
    bits: u32,
}

impl LpCoreErrRespDisVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **LP_CORE_ERR_RESP_DIS** `[2:0]`
    ///
    /// Set bit0 to disable ibus err resp;Set bit1 to disable dbus err resp; Set bit 2 to disable ahb err resp.
    #[inline(always)]
    pub const fn lp_core_err_resp_dis(&self) -> u32 {
        (self.bits & LpCoreErrRespDis::LP_CORE_ERR_RESP_DIS_MASK)
            >> LpCoreErrRespDis::LP_CORE_ERR_RESP_DIS_OFFSET
    }

    /// Set **LP_CORE_ERR_RESP_DIS** `[2:0]`
    ///
    /// Set bit0 to disable ibus err resp;Set bit1 to disable dbus err resp; Set bit 2 to disable ahb err resp.
    #[inline(always)]
    pub const fn with_lp_core_err_resp_dis(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !LpCoreErrRespDis::LP_CORE_ERR_RESP_DIS_MASK)
                | ((value << LpCoreErrRespDis::LP_CORE_ERR_RESP_DIS_OFFSET)
                    & LpCoreErrRespDis::LP_CORE_ERR_RESP_DIS_MASK),
        }
    }
}

impl Default for LpCoreErrRespDisVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl LpCoreErrRespDis {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> LpCoreErrRespDisVal {
        LpCoreErrRespDisVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: LpCoreErrRespDisVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(LpCoreErrRespDisVal) -> LpCoreErrRespDisVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// RNG_CFG Register
///
/// rng cfg register
///
/// Address offset: 0x01c0
/// Reset value: 0x00000003
///
/// ## Fields
///
/// - **RNG_SAR_DATA** `[28:16]` (RO)
///
///   debug rng sar sample cnt
///
/// - **RNG_SAR_ENABLE** `[9]` (RW)
///
///   enable rng_saradc
///
/// - **RNG_TIMER_PSCALE** `[8:1]` (RW)
///
///   configure ng timer pscale
///
/// - **RNG_TIMER_EN** `[0]` (RW)
///
///   enable rng timer
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

    /// **RNG_SAR_DATA**
    ///
    /// debug rng sar sample cnt
    ///
    /// Bits `[28:16]`, mask `0x1fff0000`
    pub const RNG_SAR_DATA_OFFSET: u32 = 16;
    pub const RNG_SAR_DATA_MASK: u32 = 0x1fff0000;

    /// **RNG_SAR_ENABLE**
    ///
    /// enable rng_saradc
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const RNG_SAR_ENABLE_OFFSET: u32 = 9;
    pub const RNG_SAR_ENABLE_MASK: u32 = 0x00000200;
    pub const RNG_SAR_ENABLE: u32 = 1 << 9;

    /// **RNG_TIMER_PSCALE**
    ///
    /// configure ng timer pscale
    ///
    /// Bits `[8:1]`, mask `0x000001fe`
    pub const RNG_TIMER_PSCALE_OFFSET: u32 = 1;
    pub const RNG_TIMER_PSCALE_MASK: u32 = 0x000001fe;

    /// **RNG_TIMER_EN**
    ///
    /// enable rng timer
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const RNG_TIMER_EN_OFFSET: u32 = 0;
    pub const RNG_TIMER_EN_MASK: u32 = 0x00000001;
    pub const RNG_TIMER_EN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RNG_SAR_DATA** `[28:16]` (RO)
    ///
    /// debug rng sar sample cnt
    #[inline(always)]
    pub fn rng_sar_data(&self) -> u32 {
        (self.value.read_bits() & Self::RNG_SAR_DATA_MASK) >> Self::RNG_SAR_DATA_OFFSET
    }

    /// **RNG_SAR_ENABLE** `[9]` (RW)
    ///
    /// enable rng_saradc
    #[inline(always)]
    pub fn rng_sar_enable(&self) -> bool {
        (self.value.read_bits() & Self::RNG_SAR_ENABLE_MASK) != 0
    }

    /// Set RNG_SAR_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rng_sar_enable(&self) {
        self.value.modify_bits(|r| r | Self::RNG_SAR_ENABLE_MASK);
    }

    /// Clear RNG_SAR_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rng_sar_enable(&self) {
        self.value.modify_bits(|r| r & !Self::RNG_SAR_ENABLE_MASK);
    }

    /// **RNG_TIMER_PSCALE** `[8:1]` (RW)
    ///
    /// configure ng timer pscale
    #[inline(always)]
    pub fn rng_timer_pscale(&self) -> u32 {
        (self.value.read_bits() & Self::RNG_TIMER_PSCALE_MASK) >> Self::RNG_TIMER_PSCALE_OFFSET
    }

    /// Write **RNG_TIMER_PSCALE** `[8:1]` (RW)
    ///
    /// configure ng timer pscale
    #[inline(always)]
    pub fn write_rng_timer_pscale(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RNG_TIMER_PSCALE_MASK)
                | ((value << Self::RNG_TIMER_PSCALE_OFFSET) & Self::RNG_TIMER_PSCALE_MASK)
        });
    }

    /// **RNG_TIMER_EN** `[0]` (RW)
    ///
    /// enable rng timer
    #[inline(always)]
    pub fn rng_timer_en(&self) -> bool {
        (self.value.read_bits() & Self::RNG_TIMER_EN_MASK) != 0
    }

    /// Set RNG_TIMER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rng_timer_en(&self) {
        self.value.modify_bits(|r| r | Self::RNG_TIMER_EN_MASK);
    }

    /// Clear RNG_TIMER_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rng_timer_en(&self) {
        self.value.modify_bits(|r| r & !Self::RNG_TIMER_EN_MASK);
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

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000003 }
    }

    /// Read **RNG_SAR_DATA** `[28:16]`
    ///
    /// debug rng sar sample cnt
    #[inline(always)]
    pub const fn rng_sar_data(&self) -> u32 {
        (self.bits & RngCfg::RNG_SAR_DATA_MASK) >> RngCfg::RNG_SAR_DATA_OFFSET
    }

    /// Read **RNG_SAR_ENABLE** `[9:9]`
    ///
    /// enable rng_saradc
    #[inline(always)]
    pub const fn rng_sar_enable(&self) -> bool {
        (self.bits & RngCfg::RNG_SAR_ENABLE_MASK) != 0
    }

    /// Read **RNG_TIMER_PSCALE** `[8:1]`
    ///
    /// configure ng timer pscale
    #[inline(always)]
    pub const fn rng_timer_pscale(&self) -> u32 {
        (self.bits & RngCfg::RNG_TIMER_PSCALE_MASK) >> RngCfg::RNG_TIMER_PSCALE_OFFSET
    }

    /// Read **RNG_TIMER_EN** `[0:0]`
    ///
    /// enable rng timer
    #[inline(always)]
    pub const fn rng_timer_en(&self) -> bool {
        (self.bits & RngCfg::RNG_TIMER_EN_MASK) != 0
    }

    /// Set **RNG_SAR_ENABLE** `[9:9]`
    ///
    /// enable rng_saradc
    #[inline(always)]
    pub const fn with_rng_sar_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RngCfg::RNG_SAR_ENABLE_MASK
            } else {
                self.bits & !RngCfg::RNG_SAR_ENABLE_MASK
            },
        }
    }

    /// Set **RNG_TIMER_PSCALE** `[8:1]`
    ///
    /// configure ng timer pscale
    #[inline(always)]
    pub const fn with_rng_timer_pscale(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !RngCfg::RNG_TIMER_PSCALE_MASK)
                | ((value << RngCfg::RNG_TIMER_PSCALE_OFFSET) & RngCfg::RNG_TIMER_PSCALE_MASK),
        }
    }

    /// Set **RNG_TIMER_EN** `[0:0]`
    ///
    /// enable rng timer
    #[inline(always)]
    pub const fn with_rng_timer_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | RngCfg::RNG_TIMER_EN_MASK
            } else {
                self.bits & !RngCfg::RNG_TIMER_EN_MASK
            },
        }
    }
}

impl Default for RngCfgVal {
    fn default() -> Self {
        Self::reset_val()
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

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// need_des
    pub lp_sys_ver_date: LpSysVerDate,
    /// 0x0004
    /// need_des
    pub clk_sel_ctrl: ClkSelCtrl,
    /// 0x0008
    /// need_des
    pub sys_ctrl: SysCtrl,
    /// 0x000c
    /// need_des
    pub lp_clk_ctrl: LpClkCtrl,
    /// 0x0010
    /// need_des
    pub lp_rst_ctrl: LpRstCtrl,
    _reserved0: [u8; 4],
    /// 0x0018
    /// need_des
    pub lp_core_boot_addr: LpCoreBootAddr,
    /// 0x001c
    /// need_des
    pub ext_wakeup1: ExtWakeup1,
    /// 0x0020
    /// need_des
    pub ext_wakeup1_status: ExtWakeup1Status,
    /// 0x0024
    /// need_des
    pub lp_tcm_pwr_ctrl: LpTcmPwrCtrl,
    /// 0x0028
    /// need_des
    pub boot_addr_hp_lp: BootAddrHpLp,
    /// 0x002c
    /// need_des
    pub lp_store0: LpStore0,
    /// 0x0030
    /// need_des
    pub lp_store1: LpStore1,
    /// 0x0034
    /// need_des
    pub lp_store2: LpStore2,
    /// 0x0038
    /// need_des
    pub lp_store3: LpStore3,
    /// 0x003c
    /// need_des
    pub lp_store4: LpStore4,
    /// 0x0040
    /// need_des
    pub lp_store5: LpStore5,
    /// 0x0044
    /// need_des
    pub lp_store6: LpStore6,
    /// 0x0048
    /// need_des
    pub lp_store7: LpStore7,
    /// 0x004c
    /// need_des
    pub lp_store8: LpStore8,
    /// 0x0050
    /// need_des
    pub lp_store9: LpStore9,
    /// 0x0054
    /// need_des
    pub lp_store10: LpStore10,
    /// 0x0058
    /// need_des
    pub lp_store11: LpStore11,
    /// 0x005c
    /// need_des
    pub lp_store12: LpStore12,
    /// 0x0060
    /// need_des
    pub lp_store13: LpStore13,
    /// 0x0064
    /// need_des
    pub lp_store14: LpStore14,
    /// 0x0068
    /// need_des
    pub lp_store15: LpStore15,
    /// 0x006c
    /// need_des
    pub lp_probea_ctrl: LpProbeaCtrl,
    /// 0x0070
    /// need_des
    pub lp_probeb_ctrl: LpProbebCtrl,
    /// 0x0074
    /// need_des
    pub lp_probe_out: LpProbeOut,
    _reserved1: [u8; 36],
    /// 0x009c
    /// need_des
    pub f2s_apb_brg_cntl: F2sApbBrgCntl,
    _reserved2: [u8; 96],
    /// 0x0100
    /// need_des
    pub usb_ctrl: UsbCtrl,
    _reserved3: [u8; 8],
    /// 0x010c
    /// need_des
    pub ana_xpd_pad_group: AnaXpdPadGroup,
    /// 0x0110
    /// need_des
    pub lp_tcm_ram_rdn_eco_cs: LpTcmRamRdnEcoCs,
    /// 0x0114
    /// need_des
    pub lp_tcm_ram_rdn_eco_low: LpTcmRamRdnEcoLow,
    /// 0x0118
    /// need_des
    pub lp_tcm_ram_rdn_eco_high: LpTcmRamRdnEcoHigh,
    /// 0x011c
    /// need_des
    pub lp_tcm_rom_rdn_eco_cs: LpTcmRomRdnEcoCs,
    /// 0x0120
    /// need_des
    pub lp_tcm_rom_rdn_eco_low: LpTcmRomRdnEcoLow,
    /// 0x0124
    /// need_des
    pub lp_tcm_rom_rdn_eco_high: LpTcmRomRdnEcoHigh,
    _reserved4: [u8; 8],
    /// 0x0130
    /// need_des
    pub hp_root_clk_ctrl: HpRootClkCtrl,
    _reserved5: [u8; 4],
    /// 0x0138
    /// need_des
    pub lp_pmu_rdn_eco_low: LpPmuRdnEcoLow,
    /// 0x013c
    /// need_des
    pub lp_pmu_rdn_eco_high: LpPmuRdnEcoHigh,
    _reserved6: [u8; 8],
    /// 0x0148
    /// need_des
    pub pad_comp0: PadComp0,
    /// 0x014c
    /// need_des
    pub pad_comp1: PadComp1,
    _reserved7: [u8; 4],
    /// 0x0154
    /// need_des
    pub backup_dma_cfg0: BackupDmaCfg0,
    /// 0x0158
    /// need_des
    pub backup_dma_cfg1: BackupDmaCfg1,
    /// 0x015c
    /// need_des
    pub backup_dma_cfg2: BackupDmaCfg2,
    _reserved8: [u8; 4],
    /// 0x0164
    /// need_des
    pub boot_addr_hp_core1: BootAddrHpCore1,
    /// 0x0168
    /// need_des
    pub lp_addrhole_addr: LpAddrholeAddr,
    /// 0x016c
    /// need_des
    pub lp_addrhole_info: LpAddrholeInfo,
    /// 0x0170
    /// raw interrupt register
    pub int_raw: IntRaw,
    /// 0x0174
    /// masked interrupt register
    pub int_st: IntSt,
    /// 0x0178
    /// masked interrupt register
    pub int_ena: IntEna,
    /// 0x017c
    /// interrupt clear register
    pub int_clr: IntClr,
    /// 0x0180
    /// need_des
    pub hp_mem_aux_ctrl: HpMemAuxCtrl,
    /// 0x0184
    /// need_des
    pub lp_mem_aux_ctrl: LpMemAuxCtrl,
    /// 0x0188
    /// need_des
    pub hp_rom_aux_ctrl: HpRomAuxCtrl,
    /// 0x018c
    /// need_des
    pub lp_rom_aux_ctrl: LpRomAuxCtrl,
    /// 0x0190
    /// need_des
    pub lp_cpu_dbg_pc: LpCpuDbgPc,
    /// 0x0194
    /// need_des
    pub lp_cpu_exc_pc: LpCpuExcPc,
    /// 0x0198
    /// need_des
    pub idbus_addrhole_addr: IdbusAddrholeAddr,
    /// 0x019c
    /// need_des
    pub idbus_addrhole_info: IdbusAddrholeInfo,
    /// 0x01a0
    /// need_des
    pub hp_por_rst_bypass_ctrl: HpPorRstBypassCtrl,
    /// 0x01a4
    /// rng data register
    pub rng_data: RngData,
    _reserved9: [u8; 8],
    /// 0x01b0
    /// need_des
    pub lp_core_ahb_timeout: LpCoreAhbTimeout,
    /// 0x01b4
    /// need_des
    pub lp_core_ibus_timeout: LpCoreIbusTimeout,
    /// 0x01b8
    /// need_des
    pub lp_core_dbus_timeout: LpCoreDbusTimeout,
    /// 0x01bc
    /// need_des
    pub lp_core_err_resp_dis: LpCoreErrRespDis,
    /// 0x01c0
    /// rng cfg register
    pub rng_cfg: RngCfg,
}
