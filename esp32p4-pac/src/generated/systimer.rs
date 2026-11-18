//! SYSTIMER
//! System Timer

#![allow(dead_code)]

pub const BASE: u32 = 0x500e2000;

/// CONF Register
///
/// Configure system timer clock
///
/// Address offset: 0x0000
/// Reset value: 0x46000000
///
/// ## Fields
///
/// - **CLK_EN** `[31]` (RW)
///
///   register file clk gating
///
/// - **TIMER_UNIT0_WORK_EN** `[30]` (RW)
///
///   timer unit0 work enable
///
/// - **TIMER_UNIT1_WORK_EN** `[29]` (RW)
///
///   timer unit1 work enable
///
/// - **TIMER_UNIT0_CORE0_STALL_EN** `[28]` (RW)
///
///   If timer unit0 is stalled when core0 stalled
///
/// - **TIMER_UNIT0_CORE1_STALL_EN** `[27]` (RW)
///
///   If timer unit0 is stalled when core1 stalled
///
/// - **TIMER_UNIT1_CORE0_STALL_EN** `[26]` (RW)
///
///   If timer unit1 is stalled when core0 stalled
///
/// - **TIMER_UNIT1_CORE1_STALL_EN** `[25]` (RW)
///
///   If timer unit1 is stalled when core1 stalled
///
/// - **TARGET0_WORK_EN** `[24]` (RW)
///
///   target0 work enable
///
/// - **TARGET1_WORK_EN** `[23]` (RW)
///
///   target1 work enable
///
/// - **TARGET2_WORK_EN** `[22]` (RW)
///
///   target2 work enable
///
/// - **ETM_EN** `[1]` (RW)
///
///   enable systimer's etm task and event
///
/// - **SYSTIMER_CLK_FO** `[0]` (RW)
///
///   systimer clock force on
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

    /// **CLK_EN**
    ///
    /// register file clk gating
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_EN_OFFSET: u32 = 31;
    pub const CLK_EN_MASK: u32 = 0x80000000;
    pub const CLK_EN: u32 = 1 << 31;

    /// **TIMER_UNIT0_WORK_EN**
    ///
    /// timer unit0 work enable
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const TIMER_UNIT0_WORK_EN_OFFSET: u32 = 30;
    pub const TIMER_UNIT0_WORK_EN_MASK: u32 = 0x40000000;
    pub const TIMER_UNIT0_WORK_EN: u32 = 1 << 30;

    /// **TIMER_UNIT1_WORK_EN**
    ///
    /// timer unit1 work enable
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const TIMER_UNIT1_WORK_EN_OFFSET: u32 = 29;
    pub const TIMER_UNIT1_WORK_EN_MASK: u32 = 0x20000000;
    pub const TIMER_UNIT1_WORK_EN: u32 = 1 << 29;

    /// **TIMER_UNIT0_CORE0_STALL_EN**
    ///
    /// If timer unit0 is stalled when core0 stalled
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const TIMER_UNIT0_CORE0_STALL_EN_OFFSET: u32 = 28;
    pub const TIMER_UNIT0_CORE0_STALL_EN_MASK: u32 = 0x10000000;
    pub const TIMER_UNIT0_CORE0_STALL_EN: u32 = 1 << 28;

    /// **TIMER_UNIT0_CORE1_STALL_EN**
    ///
    /// If timer unit0 is stalled when core1 stalled
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const TIMER_UNIT0_CORE1_STALL_EN_OFFSET: u32 = 27;
    pub const TIMER_UNIT0_CORE1_STALL_EN_MASK: u32 = 0x08000000;
    pub const TIMER_UNIT0_CORE1_STALL_EN: u32 = 1 << 27;

    /// **TIMER_UNIT1_CORE0_STALL_EN**
    ///
    /// If timer unit1 is stalled when core0 stalled
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const TIMER_UNIT1_CORE0_STALL_EN_OFFSET: u32 = 26;
    pub const TIMER_UNIT1_CORE0_STALL_EN_MASK: u32 = 0x04000000;
    pub const TIMER_UNIT1_CORE0_STALL_EN: u32 = 1 << 26;

    /// **TIMER_UNIT1_CORE1_STALL_EN**
    ///
    /// If timer unit1 is stalled when core1 stalled
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const TIMER_UNIT1_CORE1_STALL_EN_OFFSET: u32 = 25;
    pub const TIMER_UNIT1_CORE1_STALL_EN_MASK: u32 = 0x02000000;
    pub const TIMER_UNIT1_CORE1_STALL_EN: u32 = 1 << 25;

    /// **TARGET0_WORK_EN**
    ///
    /// target0 work enable
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const TARGET0_WORK_EN_OFFSET: u32 = 24;
    pub const TARGET0_WORK_EN_MASK: u32 = 0x01000000;
    pub const TARGET0_WORK_EN: u32 = 1 << 24;

    /// **TARGET1_WORK_EN**
    ///
    /// target1 work enable
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const TARGET1_WORK_EN_OFFSET: u32 = 23;
    pub const TARGET1_WORK_EN_MASK: u32 = 0x00800000;
    pub const TARGET1_WORK_EN: u32 = 1 << 23;

    /// **TARGET2_WORK_EN**
    ///
    /// target2 work enable
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const TARGET2_WORK_EN_OFFSET: u32 = 22;
    pub const TARGET2_WORK_EN_MASK: u32 = 0x00400000;
    pub const TARGET2_WORK_EN: u32 = 1 << 22;

    /// **ETM_EN**
    ///
    /// enable systimer's etm task and event
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const ETM_EN_OFFSET: u32 = 1;
    pub const ETM_EN_MASK: u32 = 0x00000002;
    pub const ETM_EN: u32 = 1 << 1;

    /// **SYSTIMER_CLK_FO**
    ///
    /// systimer clock force on
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SYSTIMER_CLK_FO_OFFSET: u32 = 0;
    pub const SYSTIMER_CLK_FO_MASK: u32 = 0x00000001;
    pub const SYSTIMER_CLK_FO: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[31]` (RW)
    ///
    /// register file clk gating
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

    /// **TIMER_UNIT0_WORK_EN** `[30]` (RW)
    ///
    /// timer unit0 work enable
    #[inline(always)]
    pub fn timer_unit0_work_en(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_UNIT0_WORK_EN_MASK) != 0
    }

    /// Set TIMER_UNIT0_WORK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_unit0_work_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER_UNIT0_WORK_EN_MASK);
    }

    /// Clear TIMER_UNIT0_WORK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_unit0_work_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER_UNIT0_WORK_EN_MASK);
    }

    /// **TIMER_UNIT1_WORK_EN** `[29]` (RW)
    ///
    /// timer unit1 work enable
    #[inline(always)]
    pub fn timer_unit1_work_en(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_UNIT1_WORK_EN_MASK) != 0
    }

    /// Set TIMER_UNIT1_WORK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_unit1_work_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER_UNIT1_WORK_EN_MASK);
    }

    /// Clear TIMER_UNIT1_WORK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_unit1_work_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER_UNIT1_WORK_EN_MASK);
    }

    /// **TIMER_UNIT0_CORE0_STALL_EN** `[28]` (RW)
    ///
    /// If timer unit0 is stalled when core0 stalled
    #[inline(always)]
    pub fn timer_unit0_core0_stall_en(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_UNIT0_CORE0_STALL_EN_MASK) != 0
    }

    /// Set TIMER_UNIT0_CORE0_STALL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_unit0_core0_stall_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER_UNIT0_CORE0_STALL_EN_MASK);
    }

    /// Clear TIMER_UNIT0_CORE0_STALL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_unit0_core0_stall_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER_UNIT0_CORE0_STALL_EN_MASK);
    }

    /// **TIMER_UNIT0_CORE1_STALL_EN** `[27]` (RW)
    ///
    /// If timer unit0 is stalled when core1 stalled
    #[inline(always)]
    pub fn timer_unit0_core1_stall_en(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_UNIT0_CORE1_STALL_EN_MASK) != 0
    }

    /// Set TIMER_UNIT0_CORE1_STALL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_unit0_core1_stall_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER_UNIT0_CORE1_STALL_EN_MASK);
    }

    /// Clear TIMER_UNIT0_CORE1_STALL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_unit0_core1_stall_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER_UNIT0_CORE1_STALL_EN_MASK);
    }

    /// **TIMER_UNIT1_CORE0_STALL_EN** `[26]` (RW)
    ///
    /// If timer unit1 is stalled when core0 stalled
    #[inline(always)]
    pub fn timer_unit1_core0_stall_en(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_UNIT1_CORE0_STALL_EN_MASK) != 0
    }

    /// Set TIMER_UNIT1_CORE0_STALL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_unit1_core0_stall_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER_UNIT1_CORE0_STALL_EN_MASK);
    }

    /// Clear TIMER_UNIT1_CORE0_STALL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_unit1_core0_stall_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER_UNIT1_CORE0_STALL_EN_MASK);
    }

    /// **TIMER_UNIT1_CORE1_STALL_EN** `[25]` (RW)
    ///
    /// If timer unit1 is stalled when core1 stalled
    #[inline(always)]
    pub fn timer_unit1_core1_stall_en(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_UNIT1_CORE1_STALL_EN_MASK) != 0
    }

    /// Set TIMER_UNIT1_CORE1_STALL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_timer_unit1_core1_stall_en(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER_UNIT1_CORE1_STALL_EN_MASK);
    }

    /// Clear TIMER_UNIT1_CORE1_STALL_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_timer_unit1_core1_stall_en(&self) {
        self.value
            .modify_bits(|r| r & !Self::TIMER_UNIT1_CORE1_STALL_EN_MASK);
    }

    /// **TARGET0_WORK_EN** `[24]` (RW)
    ///
    /// target0 work enable
    #[inline(always)]
    pub fn target0_work_en(&self) -> bool {
        (self.value.read_bits() & Self::TARGET0_WORK_EN_MASK) != 0
    }

    /// Set TARGET0_WORK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target0_work_en(&self) {
        self.value.modify_bits(|r| r | Self::TARGET0_WORK_EN_MASK);
    }

    /// Clear TARGET0_WORK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target0_work_en(&self) {
        self.value.modify_bits(|r| r & !Self::TARGET0_WORK_EN_MASK);
    }

    /// **TARGET1_WORK_EN** `[23]` (RW)
    ///
    /// target1 work enable
    #[inline(always)]
    pub fn target1_work_en(&self) -> bool {
        (self.value.read_bits() & Self::TARGET1_WORK_EN_MASK) != 0
    }

    /// Set TARGET1_WORK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target1_work_en(&self) {
        self.value.modify_bits(|r| r | Self::TARGET1_WORK_EN_MASK);
    }

    /// Clear TARGET1_WORK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target1_work_en(&self) {
        self.value.modify_bits(|r| r & !Self::TARGET1_WORK_EN_MASK);
    }

    /// **TARGET2_WORK_EN** `[22]` (RW)
    ///
    /// target2 work enable
    #[inline(always)]
    pub fn target2_work_en(&self) -> bool {
        (self.value.read_bits() & Self::TARGET2_WORK_EN_MASK) != 0
    }

    /// Set TARGET2_WORK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target2_work_en(&self) {
        self.value.modify_bits(|r| r | Self::TARGET2_WORK_EN_MASK);
    }

    /// Clear TARGET2_WORK_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target2_work_en(&self) {
        self.value.modify_bits(|r| r & !Self::TARGET2_WORK_EN_MASK);
    }

    /// **ETM_EN** `[1]` (RW)
    ///
    /// enable systimer's etm task and event
    #[inline(always)]
    pub fn etm_en(&self) -> bool {
        (self.value.read_bits() & Self::ETM_EN_MASK) != 0
    }

    /// Set ETM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etm_en(&self) {
        self.value.modify_bits(|r| r | Self::ETM_EN_MASK);
    }

    /// Clear ETM_EN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etm_en(&self) {
        self.value.modify_bits(|r| r & !Self::ETM_EN_MASK);
    }

    /// **SYSTIMER_CLK_FO** `[0]` (RW)
    ///
    /// systimer clock force on
    #[inline(always)]
    pub fn systimer_clk_fo(&self) -> bool {
        (self.value.read_bits() & Self::SYSTIMER_CLK_FO_MASK) != 0
    }

    /// Set SYSTIMER_CLK_FO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_systimer_clk_fo(&self) {
        self.value.modify_bits(|r| r | Self::SYSTIMER_CLK_FO_MASK);
    }

    /// Clear SYSTIMER_CLK_FO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_systimer_clk_fo(&self) {
        self.value.modify_bits(|r| r & !Self::SYSTIMER_CLK_FO_MASK);
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

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x46000000 }
    }

    /// Read **CLK_EN** `[31:31]`
    ///
    /// register file clk gating
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & Conf::CLK_EN_MASK) != 0
    }

    /// Read **TIMER_UNIT0_WORK_EN** `[30:30]`
    ///
    /// timer unit0 work enable
    #[inline(always)]
    pub const fn timer_unit0_work_en(&self) -> bool {
        (self.bits & Conf::TIMER_UNIT0_WORK_EN_MASK) != 0
    }

    /// Read **TIMER_UNIT1_WORK_EN** `[29:29]`
    ///
    /// timer unit1 work enable
    #[inline(always)]
    pub const fn timer_unit1_work_en(&self) -> bool {
        (self.bits & Conf::TIMER_UNIT1_WORK_EN_MASK) != 0
    }

    /// Read **TIMER_UNIT0_CORE0_STALL_EN** `[28:28]`
    ///
    /// If timer unit0 is stalled when core0 stalled
    #[inline(always)]
    pub const fn timer_unit0_core0_stall_en(&self) -> bool {
        (self.bits & Conf::TIMER_UNIT0_CORE0_STALL_EN_MASK) != 0
    }

    /// Read **TIMER_UNIT0_CORE1_STALL_EN** `[27:27]`
    ///
    /// If timer unit0 is stalled when core1 stalled
    #[inline(always)]
    pub const fn timer_unit0_core1_stall_en(&self) -> bool {
        (self.bits & Conf::TIMER_UNIT0_CORE1_STALL_EN_MASK) != 0
    }

    /// Read **TIMER_UNIT1_CORE0_STALL_EN** `[26:26]`
    ///
    /// If timer unit1 is stalled when core0 stalled
    #[inline(always)]
    pub const fn timer_unit1_core0_stall_en(&self) -> bool {
        (self.bits & Conf::TIMER_UNIT1_CORE0_STALL_EN_MASK) != 0
    }

    /// Read **TIMER_UNIT1_CORE1_STALL_EN** `[25:25]`
    ///
    /// If timer unit1 is stalled when core1 stalled
    #[inline(always)]
    pub const fn timer_unit1_core1_stall_en(&self) -> bool {
        (self.bits & Conf::TIMER_UNIT1_CORE1_STALL_EN_MASK) != 0
    }

    /// Read **TARGET0_WORK_EN** `[24:24]`
    ///
    /// target0 work enable
    #[inline(always)]
    pub const fn target0_work_en(&self) -> bool {
        (self.bits & Conf::TARGET0_WORK_EN_MASK) != 0
    }

    /// Read **TARGET1_WORK_EN** `[23:23]`
    ///
    /// target1 work enable
    #[inline(always)]
    pub const fn target1_work_en(&self) -> bool {
        (self.bits & Conf::TARGET1_WORK_EN_MASK) != 0
    }

    /// Read **TARGET2_WORK_EN** `[22:22]`
    ///
    /// target2 work enable
    #[inline(always)]
    pub const fn target2_work_en(&self) -> bool {
        (self.bits & Conf::TARGET2_WORK_EN_MASK) != 0
    }

    /// Read **ETM_EN** `[1:1]`
    ///
    /// enable systimer's etm task and event
    #[inline(always)]
    pub const fn etm_en(&self) -> bool {
        (self.bits & Conf::ETM_EN_MASK) != 0
    }

    /// Read **SYSTIMER_CLK_FO** `[0:0]`
    ///
    /// systimer clock force on
    #[inline(always)]
    pub const fn systimer_clk_fo(&self) -> bool {
        (self.bits & Conf::SYSTIMER_CLK_FO_MASK) != 0
    }

    /// Set **CLK_EN** `[31:31]`
    ///
    /// register file clk gating
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::CLK_EN_MASK
            } else {
                self.bits & !Conf::CLK_EN_MASK
            },
        }
    }

    /// Set **TIMER_UNIT0_WORK_EN** `[30:30]`
    ///
    /// timer unit0 work enable
    #[inline(always)]
    pub const fn with_timer_unit0_work_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::TIMER_UNIT0_WORK_EN_MASK
            } else {
                self.bits & !Conf::TIMER_UNIT0_WORK_EN_MASK
            },
        }
    }

    /// Set **TIMER_UNIT1_WORK_EN** `[29:29]`
    ///
    /// timer unit1 work enable
    #[inline(always)]
    pub const fn with_timer_unit1_work_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::TIMER_UNIT1_WORK_EN_MASK
            } else {
                self.bits & !Conf::TIMER_UNIT1_WORK_EN_MASK
            },
        }
    }

    /// Set **TIMER_UNIT0_CORE0_STALL_EN** `[28:28]`
    ///
    /// If timer unit0 is stalled when core0 stalled
    #[inline(always)]
    pub const fn with_timer_unit0_core0_stall_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::TIMER_UNIT0_CORE0_STALL_EN_MASK
            } else {
                self.bits & !Conf::TIMER_UNIT0_CORE0_STALL_EN_MASK
            },
        }
    }

    /// Set **TIMER_UNIT0_CORE1_STALL_EN** `[27:27]`
    ///
    /// If timer unit0 is stalled when core1 stalled
    #[inline(always)]
    pub const fn with_timer_unit0_core1_stall_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::TIMER_UNIT0_CORE1_STALL_EN_MASK
            } else {
                self.bits & !Conf::TIMER_UNIT0_CORE1_STALL_EN_MASK
            },
        }
    }

    /// Set **TIMER_UNIT1_CORE0_STALL_EN** `[26:26]`
    ///
    /// If timer unit1 is stalled when core0 stalled
    #[inline(always)]
    pub const fn with_timer_unit1_core0_stall_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::TIMER_UNIT1_CORE0_STALL_EN_MASK
            } else {
                self.bits & !Conf::TIMER_UNIT1_CORE0_STALL_EN_MASK
            },
        }
    }

    /// Set **TIMER_UNIT1_CORE1_STALL_EN** `[25:25]`
    ///
    /// If timer unit1 is stalled when core1 stalled
    #[inline(always)]
    pub const fn with_timer_unit1_core1_stall_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::TIMER_UNIT1_CORE1_STALL_EN_MASK
            } else {
                self.bits & !Conf::TIMER_UNIT1_CORE1_STALL_EN_MASK
            },
        }
    }

    /// Set **TARGET0_WORK_EN** `[24:24]`
    ///
    /// target0 work enable
    #[inline(always)]
    pub const fn with_target0_work_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::TARGET0_WORK_EN_MASK
            } else {
                self.bits & !Conf::TARGET0_WORK_EN_MASK
            },
        }
    }

    /// Set **TARGET1_WORK_EN** `[23:23]`
    ///
    /// target1 work enable
    #[inline(always)]
    pub const fn with_target1_work_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::TARGET1_WORK_EN_MASK
            } else {
                self.bits & !Conf::TARGET1_WORK_EN_MASK
            },
        }
    }

    /// Set **TARGET2_WORK_EN** `[22:22]`
    ///
    /// target2 work enable
    #[inline(always)]
    pub const fn with_target2_work_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::TARGET2_WORK_EN_MASK
            } else {
                self.bits & !Conf::TARGET2_WORK_EN_MASK
            },
        }
    }

    /// Set **ETM_EN** `[1:1]`
    ///
    /// enable systimer's etm task and event
    #[inline(always)]
    pub const fn with_etm_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::ETM_EN_MASK
            } else {
                self.bits & !Conf::ETM_EN_MASK
            },
        }
    }

    /// Set **SYSTIMER_CLK_FO** `[0:0]`
    ///
    /// systimer clock force on
    #[inline(always)]
    pub const fn with_systimer_clk_fo(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Conf::SYSTIMER_CLK_FO_MASK
            } else {
                self.bits & !Conf::SYSTIMER_CLK_FO_MASK
            },
        }
    }
}

impl Default for ConfVal {
    fn default() -> Self {
        Self::reset_val()
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

/// UNIT0_OP Register
///
/// system timer unit0 value update register
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **TIMER_UNIT0_UPDATE** `[30]` (WO)
///
///   update timer_unit0
///
/// - **TIMER_UNIT0_VALUE_VALID** `[29]` (RO)
///
///   timer value is sync and valid
#[repr(transparent)]
pub struct Unit0Op {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Unit0Op {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Unit0Op {
    // ======== Field Constants ========

    /// **TIMER_UNIT0_UPDATE**
    ///
    /// update timer_unit0
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const TIMER_UNIT0_UPDATE_OFFSET: u32 = 30;
    pub const TIMER_UNIT0_UPDATE_MASK: u32 = 0x40000000;
    pub const TIMER_UNIT0_UPDATE: u32 = 1 << 30;

    /// **TIMER_UNIT0_VALUE_VALID**
    ///
    /// timer value is sync and valid
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const TIMER_UNIT0_VALUE_VALID_OFFSET: u32 = 29;
    pub const TIMER_UNIT0_VALUE_VALID_MASK: u32 = 0x20000000;
    pub const TIMER_UNIT0_VALUE_VALID: u32 = 1 << 29;

    // ======== Field Accessors ========

    /// Trigger **TIMER_UNIT0_UPDATE** (WO)
    ///
    /// update timer_unit0
    #[inline(always)]
    pub fn trigger_timer_unit0_update(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER_UNIT0_UPDATE_MASK);
    }

    /// **TIMER_UNIT0_VALUE_VALID** `[29]` (RO)
    ///
    /// timer value is sync and valid
    #[inline(always)]
    pub fn timer_unit0_value_valid(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_UNIT0_VALUE_VALID_MASK) != 0
    }
}

/// UNIT0_OP value type for batch read/modify/write operations
///
/// Use `unit0op.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `unit0op.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Unit0OpVal {
    bits: u32,
}

impl Unit0OpVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_UNIT0_VALUE_VALID** `[29:29]`
    ///
    /// timer value is sync and valid
    #[inline(always)]
    pub const fn timer_unit0_value_valid(&self) -> bool {
        (self.bits & Unit0Op::TIMER_UNIT0_VALUE_VALID_MASK) != 0
    }

    /// Set **TIMER_UNIT0_UPDATE** `[30:30]`
    ///
    /// update timer_unit0
    #[inline(always)]
    pub const fn with_timer_unit0_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Unit0Op::TIMER_UNIT0_UPDATE_MASK
            } else {
                self.bits & !Unit0Op::TIMER_UNIT0_UPDATE_MASK
            },
        }
    }
}

impl Default for Unit0OpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Unit0Op {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Unit0OpVal {
        Unit0OpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Unit0OpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Unit0OpVal) -> Unit0OpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UNIT1_OP Register
///
/// system timer unit1 value update register
///
/// Address offset: 0x0008
///
/// ## Fields
///
/// - **TIMER_UNIT1_UPDATE** `[30]` (WO)
///
///   update timer unit1
///
/// - **TIMER_UNIT1_VALUE_VALID** `[29]` (RO)
///
///   timer value is sync and valid
#[repr(transparent)]
pub struct Unit1Op {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Unit1Op {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Unit1Op {
    // ======== Field Constants ========

    /// **TIMER_UNIT1_UPDATE**
    ///
    /// update timer unit1
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const TIMER_UNIT1_UPDATE_OFFSET: u32 = 30;
    pub const TIMER_UNIT1_UPDATE_MASK: u32 = 0x40000000;
    pub const TIMER_UNIT1_UPDATE: u32 = 1 << 30;

    /// **TIMER_UNIT1_VALUE_VALID**
    ///
    /// timer value is sync and valid
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const TIMER_UNIT1_VALUE_VALID_OFFSET: u32 = 29;
    pub const TIMER_UNIT1_VALUE_VALID_MASK: u32 = 0x20000000;
    pub const TIMER_UNIT1_VALUE_VALID: u32 = 1 << 29;

    // ======== Field Accessors ========

    /// Trigger **TIMER_UNIT1_UPDATE** (WO)
    ///
    /// update timer unit1
    #[inline(always)]
    pub fn trigger_timer_unit1_update(&self) {
        self.value
            .modify_bits(|r| r | Self::TIMER_UNIT1_UPDATE_MASK);
    }

    /// **TIMER_UNIT1_VALUE_VALID** `[29]` (RO)
    ///
    /// timer value is sync and valid
    #[inline(always)]
    pub fn timer_unit1_value_valid(&self) -> bool {
        (self.value.read_bits() & Self::TIMER_UNIT1_VALUE_VALID_MASK) != 0
    }
}

/// UNIT1_OP value type for batch read/modify/write operations
///
/// Use `unit1op.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `unit1op.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Unit1OpVal {
    bits: u32,
}

impl Unit1OpVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_UNIT1_VALUE_VALID** `[29:29]`
    ///
    /// timer value is sync and valid
    #[inline(always)]
    pub const fn timer_unit1_value_valid(&self) -> bool {
        (self.bits & Unit1Op::TIMER_UNIT1_VALUE_VALID_MASK) != 0
    }

    /// Set **TIMER_UNIT1_UPDATE** `[30:30]`
    ///
    /// update timer unit1
    #[inline(always)]
    pub const fn with_timer_unit1_update(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Unit1Op::TIMER_UNIT1_UPDATE_MASK
            } else {
                self.bits & !Unit1Op::TIMER_UNIT1_UPDATE_MASK
            },
        }
    }
}

impl Default for Unit1OpVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Unit1Op {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Unit1OpVal {
        Unit1OpVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Unit1OpVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Unit1OpVal) -> Unit1OpVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UNIT0_LOAD_HI Register
///
/// system timer unit0 value high load register
///
/// Address offset: 0x000c
///
/// ## Fields
///
/// - **TIMER_UNIT0_LOAD_HI** `[19:0]` (RW)
///
///   timer unit0 load high 20 bits
#[repr(transparent)]
pub struct Unit0LoadHi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Unit0LoadHi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Unit0LoadHi {
    // ======== Field Constants ========

    /// **TIMER_UNIT0_LOAD_HI**
    ///
    /// timer unit0 load high 20 bits
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_UNIT0_LOAD_HI_OFFSET: u32 = 0;
    pub const TIMER_UNIT0_LOAD_HI_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_UNIT0_LOAD_HI** `[19:0]` (RW)
    ///
    /// timer unit0 load high 20 bits
    #[inline(always)]
    pub fn timer_unit0_load_hi(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_UNIT0_LOAD_HI_MASK)
            >> Self::TIMER_UNIT0_LOAD_HI_OFFSET
    }

    /// Write **TIMER_UNIT0_LOAD_HI** `[19:0]` (RW)
    ///
    /// timer unit0 load high 20 bits
    #[inline(always)]
    pub fn write_timer_unit0_load_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_UNIT0_LOAD_HI_MASK)
                | ((value << Self::TIMER_UNIT0_LOAD_HI_OFFSET) & Self::TIMER_UNIT0_LOAD_HI_MASK)
        });
    }
}

/// UNIT0_LOAD_HI value type for batch read/modify/write operations
///
/// Use `unit0loadhi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `unit0loadhi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Unit0LoadHiVal {
    bits: u32,
}

impl Unit0LoadHiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_UNIT0_LOAD_HI** `[19:0]`
    ///
    /// timer unit0 load high 20 bits
    #[inline(always)]
    pub const fn timer_unit0_load_hi(&self) -> u32 {
        (self.bits & Unit0LoadHi::TIMER_UNIT0_LOAD_HI_MASK)
            >> Unit0LoadHi::TIMER_UNIT0_LOAD_HI_OFFSET
    }

    /// Set **TIMER_UNIT0_LOAD_HI** `[19:0]`
    ///
    /// timer unit0 load high 20 bits
    #[inline(always)]
    pub const fn with_timer_unit0_load_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Unit0LoadHi::TIMER_UNIT0_LOAD_HI_MASK)
                | ((value << Unit0LoadHi::TIMER_UNIT0_LOAD_HI_OFFSET)
                    & Unit0LoadHi::TIMER_UNIT0_LOAD_HI_MASK),
        }
    }
}

impl Default for Unit0LoadHiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Unit0LoadHi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Unit0LoadHiVal {
        Unit0LoadHiVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Unit0LoadHiVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Unit0LoadHiVal) -> Unit0LoadHiVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UNIT0_LOAD_LO Register
///
/// system timer unit0 value low load register
///
/// Address offset: 0x0010
///
/// ## Fields
///
/// - **TIMER_UNIT0_LOAD_LO** `[31:0]` (RW)
///
///   timer unit0 load low 32 bits
#[repr(transparent)]
pub struct Unit0LoadLo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Unit0LoadLo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Unit0LoadLo {
    // ======== Field Constants ========

    /// **TIMER_UNIT0_LOAD_LO**
    ///
    /// timer unit0 load low 32 bits
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TIMER_UNIT0_LOAD_LO_OFFSET: u32 = 0;
    pub const TIMER_UNIT0_LOAD_LO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TIMER_UNIT0_LOAD_LO** `[31:0]` (RW)
    ///
    /// timer unit0 load low 32 bits
    #[inline(always)]
    pub fn timer_unit0_load_lo(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_UNIT0_LOAD_LO_MASK)
            >> Self::TIMER_UNIT0_LOAD_LO_OFFSET
    }

    /// Write **TIMER_UNIT0_LOAD_LO** `[31:0]` (RW)
    ///
    /// timer unit0 load low 32 bits
    #[inline(always)]
    pub fn write_timer_unit0_load_lo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_UNIT0_LOAD_LO_MASK)
                | ((value << Self::TIMER_UNIT0_LOAD_LO_OFFSET) & Self::TIMER_UNIT0_LOAD_LO_MASK)
        });
    }
}

/// UNIT0_LOAD_LO value type for batch read/modify/write operations
///
/// Use `unit0loadlo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `unit0loadlo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Unit0LoadLoVal {
    bits: u32,
}

impl Unit0LoadLoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_UNIT0_LOAD_LO** `[31:0]`
    ///
    /// timer unit0 load low 32 bits
    #[inline(always)]
    pub const fn timer_unit0_load_lo(&self) -> u32 {
        (self.bits & Unit0LoadLo::TIMER_UNIT0_LOAD_LO_MASK)
            >> Unit0LoadLo::TIMER_UNIT0_LOAD_LO_OFFSET
    }

    /// Set **TIMER_UNIT0_LOAD_LO** `[31:0]`
    ///
    /// timer unit0 load low 32 bits
    #[inline(always)]
    pub const fn with_timer_unit0_load_lo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Unit0LoadLo::TIMER_UNIT0_LOAD_LO_MASK)
                | ((value << Unit0LoadLo::TIMER_UNIT0_LOAD_LO_OFFSET)
                    & Unit0LoadLo::TIMER_UNIT0_LOAD_LO_MASK),
        }
    }
}

impl Default for Unit0LoadLoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Unit0LoadLo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Unit0LoadLoVal {
        Unit0LoadLoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Unit0LoadLoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Unit0LoadLoVal) -> Unit0LoadLoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UNIT1_LOAD_HI Register
///
/// system timer unit1 value high load register
///
/// Address offset: 0x0014
///
/// ## Fields
///
/// - **TIMER_UNIT1_LOAD_HI** `[19:0]` (RW)
///
///   timer unit1 load high 20 bits
#[repr(transparent)]
pub struct Unit1LoadHi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Unit1LoadHi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Unit1LoadHi {
    // ======== Field Constants ========

    /// **TIMER_UNIT1_LOAD_HI**
    ///
    /// timer unit1 load high 20 bits
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_UNIT1_LOAD_HI_OFFSET: u32 = 0;
    pub const TIMER_UNIT1_LOAD_HI_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_UNIT1_LOAD_HI** `[19:0]` (RW)
    ///
    /// timer unit1 load high 20 bits
    #[inline(always)]
    pub fn timer_unit1_load_hi(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_UNIT1_LOAD_HI_MASK)
            >> Self::TIMER_UNIT1_LOAD_HI_OFFSET
    }

    /// Write **TIMER_UNIT1_LOAD_HI** `[19:0]` (RW)
    ///
    /// timer unit1 load high 20 bits
    #[inline(always)]
    pub fn write_timer_unit1_load_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_UNIT1_LOAD_HI_MASK)
                | ((value << Self::TIMER_UNIT1_LOAD_HI_OFFSET) & Self::TIMER_UNIT1_LOAD_HI_MASK)
        });
    }
}

/// UNIT1_LOAD_HI value type for batch read/modify/write operations
///
/// Use `unit1loadhi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `unit1loadhi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Unit1LoadHiVal {
    bits: u32,
}

impl Unit1LoadHiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_UNIT1_LOAD_HI** `[19:0]`
    ///
    /// timer unit1 load high 20 bits
    #[inline(always)]
    pub const fn timer_unit1_load_hi(&self) -> u32 {
        (self.bits & Unit1LoadHi::TIMER_UNIT1_LOAD_HI_MASK)
            >> Unit1LoadHi::TIMER_UNIT1_LOAD_HI_OFFSET
    }

    /// Set **TIMER_UNIT1_LOAD_HI** `[19:0]`
    ///
    /// timer unit1 load high 20 bits
    #[inline(always)]
    pub const fn with_timer_unit1_load_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Unit1LoadHi::TIMER_UNIT1_LOAD_HI_MASK)
                | ((value << Unit1LoadHi::TIMER_UNIT1_LOAD_HI_OFFSET)
                    & Unit1LoadHi::TIMER_UNIT1_LOAD_HI_MASK),
        }
    }
}

impl Default for Unit1LoadHiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Unit1LoadHi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Unit1LoadHiVal {
        Unit1LoadHiVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Unit1LoadHiVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Unit1LoadHiVal) -> Unit1LoadHiVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UNIT1_LOAD_LO Register
///
/// system timer unit1 value low load register
///
/// Address offset: 0x0018
///
/// ## Fields
///
/// - **TIMER_UNIT1_LOAD_LO** `[31:0]` (RW)
///
///   timer unit1 load low 32 bits
#[repr(transparent)]
pub struct Unit1LoadLo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Unit1LoadLo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Unit1LoadLo {
    // ======== Field Constants ========

    /// **TIMER_UNIT1_LOAD_LO**
    ///
    /// timer unit1 load low 32 bits
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TIMER_UNIT1_LOAD_LO_OFFSET: u32 = 0;
    pub const TIMER_UNIT1_LOAD_LO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TIMER_UNIT1_LOAD_LO** `[31:0]` (RW)
    ///
    /// timer unit1 load low 32 bits
    #[inline(always)]
    pub fn timer_unit1_load_lo(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_UNIT1_LOAD_LO_MASK)
            >> Self::TIMER_UNIT1_LOAD_LO_OFFSET
    }

    /// Write **TIMER_UNIT1_LOAD_LO** `[31:0]` (RW)
    ///
    /// timer unit1 load low 32 bits
    #[inline(always)]
    pub fn write_timer_unit1_load_lo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_UNIT1_LOAD_LO_MASK)
                | ((value << Self::TIMER_UNIT1_LOAD_LO_OFFSET) & Self::TIMER_UNIT1_LOAD_LO_MASK)
        });
    }
}

/// UNIT1_LOAD_LO value type for batch read/modify/write operations
///
/// Use `unit1loadlo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `unit1loadlo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Unit1LoadLoVal {
    bits: u32,
}

impl Unit1LoadLoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_UNIT1_LOAD_LO** `[31:0]`
    ///
    /// timer unit1 load low 32 bits
    #[inline(always)]
    pub const fn timer_unit1_load_lo(&self) -> u32 {
        (self.bits & Unit1LoadLo::TIMER_UNIT1_LOAD_LO_MASK)
            >> Unit1LoadLo::TIMER_UNIT1_LOAD_LO_OFFSET
    }

    /// Set **TIMER_UNIT1_LOAD_LO** `[31:0]`
    ///
    /// timer unit1 load low 32 bits
    #[inline(always)]
    pub const fn with_timer_unit1_load_lo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Unit1LoadLo::TIMER_UNIT1_LOAD_LO_MASK)
                | ((value << Unit1LoadLo::TIMER_UNIT1_LOAD_LO_OFFSET)
                    & Unit1LoadLo::TIMER_UNIT1_LOAD_LO_MASK),
        }
    }
}

impl Default for Unit1LoadLoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Unit1LoadLo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Unit1LoadLoVal {
        Unit1LoadLoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Unit1LoadLoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Unit1LoadLoVal) -> Unit1LoadLoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TARGET0_HI Register
///
/// system timer comp0 value high register
///
/// Address offset: 0x001c
///
/// ## Fields
///
/// - **TIMER_TARGET0_HI** `[19:0]` (RW)
///
///   timer taget0 high 20 bits
#[repr(transparent)]
pub struct Target0Hi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Target0Hi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Target0Hi {
    // ======== Field Constants ========

    /// **TIMER_TARGET0_HI**
    ///
    /// timer taget0 high 20 bits
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_TARGET0_HI_OFFSET: u32 = 0;
    pub const TIMER_TARGET0_HI_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_TARGET0_HI** `[19:0]` (RW)
    ///
    /// timer taget0 high 20 bits
    #[inline(always)]
    pub fn timer_target0_hi(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_TARGET0_HI_MASK) >> Self::TIMER_TARGET0_HI_OFFSET
    }

    /// Write **TIMER_TARGET0_HI** `[19:0]` (RW)
    ///
    /// timer taget0 high 20 bits
    #[inline(always)]
    pub fn write_timer_target0_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_TARGET0_HI_MASK)
                | ((value << Self::TIMER_TARGET0_HI_OFFSET) & Self::TIMER_TARGET0_HI_MASK)
        });
    }
}

/// TARGET0_HI value type for batch read/modify/write operations
///
/// Use `target0hi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `target0hi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Target0HiVal {
    bits: u32,
}

impl Target0HiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_TARGET0_HI** `[19:0]`
    ///
    /// timer taget0 high 20 bits
    #[inline(always)]
    pub const fn timer_target0_hi(&self) -> u32 {
        (self.bits & Target0Hi::TIMER_TARGET0_HI_MASK) >> Target0Hi::TIMER_TARGET0_HI_OFFSET
    }

    /// Set **TIMER_TARGET0_HI** `[19:0]`
    ///
    /// timer taget0 high 20 bits
    #[inline(always)]
    pub const fn with_timer_target0_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Target0Hi::TIMER_TARGET0_HI_MASK)
                | ((value << Target0Hi::TIMER_TARGET0_HI_OFFSET)
                    & Target0Hi::TIMER_TARGET0_HI_MASK),
        }
    }
}

impl Default for Target0HiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Target0Hi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Target0HiVal {
        Target0HiVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Target0HiVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Target0HiVal) -> Target0HiVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TARGET0_LO Register
///
/// system timer comp0 value low register
///
/// Address offset: 0x0020
///
/// ## Fields
///
/// - **TIMER_TARGET0_LO** `[31:0]` (RW)
///
///   timer taget0 low 32 bits
#[repr(transparent)]
pub struct Target0Lo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Target0Lo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Target0Lo {
    // ======== Field Constants ========

    /// **TIMER_TARGET0_LO**
    ///
    /// timer taget0 low 32 bits
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TIMER_TARGET0_LO_OFFSET: u32 = 0;
    pub const TIMER_TARGET0_LO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TIMER_TARGET0_LO** `[31:0]` (RW)
    ///
    /// timer taget0 low 32 bits
    #[inline(always)]
    pub fn timer_target0_lo(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_TARGET0_LO_MASK) >> Self::TIMER_TARGET0_LO_OFFSET
    }

    /// Write **TIMER_TARGET0_LO** `[31:0]` (RW)
    ///
    /// timer taget0 low 32 bits
    #[inline(always)]
    pub fn write_timer_target0_lo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_TARGET0_LO_MASK)
                | ((value << Self::TIMER_TARGET0_LO_OFFSET) & Self::TIMER_TARGET0_LO_MASK)
        });
    }
}

/// TARGET0_LO value type for batch read/modify/write operations
///
/// Use `target0lo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `target0lo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Target0LoVal {
    bits: u32,
}

impl Target0LoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_TARGET0_LO** `[31:0]`
    ///
    /// timer taget0 low 32 bits
    #[inline(always)]
    pub const fn timer_target0_lo(&self) -> u32 {
        (self.bits & Target0Lo::TIMER_TARGET0_LO_MASK) >> Target0Lo::TIMER_TARGET0_LO_OFFSET
    }

    /// Set **TIMER_TARGET0_LO** `[31:0]`
    ///
    /// timer taget0 low 32 bits
    #[inline(always)]
    pub const fn with_timer_target0_lo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Target0Lo::TIMER_TARGET0_LO_MASK)
                | ((value << Target0Lo::TIMER_TARGET0_LO_OFFSET)
                    & Target0Lo::TIMER_TARGET0_LO_MASK),
        }
    }
}

impl Default for Target0LoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Target0Lo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Target0LoVal {
        Target0LoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Target0LoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Target0LoVal) -> Target0LoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TARGET1_HI Register
///
/// system timer comp1 value high register
///
/// Address offset: 0x0024
///
/// ## Fields
///
/// - **TIMER_TARGET1_HI** `[19:0]` (RW)
///
///   timer taget1 high 20 bits
#[repr(transparent)]
pub struct Target1Hi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Target1Hi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Target1Hi {
    // ======== Field Constants ========

    /// **TIMER_TARGET1_HI**
    ///
    /// timer taget1 high 20 bits
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_TARGET1_HI_OFFSET: u32 = 0;
    pub const TIMER_TARGET1_HI_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_TARGET1_HI** `[19:0]` (RW)
    ///
    /// timer taget1 high 20 bits
    #[inline(always)]
    pub fn timer_target1_hi(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_TARGET1_HI_MASK) >> Self::TIMER_TARGET1_HI_OFFSET
    }

    /// Write **TIMER_TARGET1_HI** `[19:0]` (RW)
    ///
    /// timer taget1 high 20 bits
    #[inline(always)]
    pub fn write_timer_target1_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_TARGET1_HI_MASK)
                | ((value << Self::TIMER_TARGET1_HI_OFFSET) & Self::TIMER_TARGET1_HI_MASK)
        });
    }
}

/// TARGET1_HI value type for batch read/modify/write operations
///
/// Use `target1hi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `target1hi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Target1HiVal {
    bits: u32,
}

impl Target1HiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_TARGET1_HI** `[19:0]`
    ///
    /// timer taget1 high 20 bits
    #[inline(always)]
    pub const fn timer_target1_hi(&self) -> u32 {
        (self.bits & Target1Hi::TIMER_TARGET1_HI_MASK) >> Target1Hi::TIMER_TARGET1_HI_OFFSET
    }

    /// Set **TIMER_TARGET1_HI** `[19:0]`
    ///
    /// timer taget1 high 20 bits
    #[inline(always)]
    pub const fn with_timer_target1_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Target1Hi::TIMER_TARGET1_HI_MASK)
                | ((value << Target1Hi::TIMER_TARGET1_HI_OFFSET)
                    & Target1Hi::TIMER_TARGET1_HI_MASK),
        }
    }
}

impl Default for Target1HiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Target1Hi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Target1HiVal {
        Target1HiVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Target1HiVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Target1HiVal) -> Target1HiVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TARGET1_LO Register
///
/// system timer comp1 value low register
///
/// Address offset: 0x0028
///
/// ## Fields
///
/// - **TIMER_TARGET1_LO** `[31:0]` (RW)
///
///   timer taget1 low 32 bits
#[repr(transparent)]
pub struct Target1Lo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Target1Lo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Target1Lo {
    // ======== Field Constants ========

    /// **TIMER_TARGET1_LO**
    ///
    /// timer taget1 low 32 bits
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TIMER_TARGET1_LO_OFFSET: u32 = 0;
    pub const TIMER_TARGET1_LO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TIMER_TARGET1_LO** `[31:0]` (RW)
    ///
    /// timer taget1 low 32 bits
    #[inline(always)]
    pub fn timer_target1_lo(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_TARGET1_LO_MASK) >> Self::TIMER_TARGET1_LO_OFFSET
    }

    /// Write **TIMER_TARGET1_LO** `[31:0]` (RW)
    ///
    /// timer taget1 low 32 bits
    #[inline(always)]
    pub fn write_timer_target1_lo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_TARGET1_LO_MASK)
                | ((value << Self::TIMER_TARGET1_LO_OFFSET) & Self::TIMER_TARGET1_LO_MASK)
        });
    }
}

/// TARGET1_LO value type for batch read/modify/write operations
///
/// Use `target1lo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `target1lo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Target1LoVal {
    bits: u32,
}

impl Target1LoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_TARGET1_LO** `[31:0]`
    ///
    /// timer taget1 low 32 bits
    #[inline(always)]
    pub const fn timer_target1_lo(&self) -> u32 {
        (self.bits & Target1Lo::TIMER_TARGET1_LO_MASK) >> Target1Lo::TIMER_TARGET1_LO_OFFSET
    }

    /// Set **TIMER_TARGET1_LO** `[31:0]`
    ///
    /// timer taget1 low 32 bits
    #[inline(always)]
    pub const fn with_timer_target1_lo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Target1Lo::TIMER_TARGET1_LO_MASK)
                | ((value << Target1Lo::TIMER_TARGET1_LO_OFFSET)
                    & Target1Lo::TIMER_TARGET1_LO_MASK),
        }
    }
}

impl Default for Target1LoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Target1Lo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Target1LoVal {
        Target1LoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Target1LoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Target1LoVal) -> Target1LoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TARGET2_HI Register
///
/// system timer comp2 value high register
///
/// Address offset: 0x002c
///
/// ## Fields
///
/// - **TIMER_TARGET2_HI** `[19:0]` (RW)
///
///   timer taget2 high 20 bits
#[repr(transparent)]
pub struct Target2Hi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Target2Hi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Target2Hi {
    // ======== Field Constants ========

    /// **TIMER_TARGET2_HI**
    ///
    /// timer taget2 high 20 bits
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_TARGET2_HI_OFFSET: u32 = 0;
    pub const TIMER_TARGET2_HI_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_TARGET2_HI** `[19:0]` (RW)
    ///
    /// timer taget2 high 20 bits
    #[inline(always)]
    pub fn timer_target2_hi(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_TARGET2_HI_MASK) >> Self::TIMER_TARGET2_HI_OFFSET
    }

    /// Write **TIMER_TARGET2_HI** `[19:0]` (RW)
    ///
    /// timer taget2 high 20 bits
    #[inline(always)]
    pub fn write_timer_target2_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_TARGET2_HI_MASK)
                | ((value << Self::TIMER_TARGET2_HI_OFFSET) & Self::TIMER_TARGET2_HI_MASK)
        });
    }
}

/// TARGET2_HI value type for batch read/modify/write operations
///
/// Use `target2hi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `target2hi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Target2HiVal {
    bits: u32,
}

impl Target2HiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_TARGET2_HI** `[19:0]`
    ///
    /// timer taget2 high 20 bits
    #[inline(always)]
    pub const fn timer_target2_hi(&self) -> u32 {
        (self.bits & Target2Hi::TIMER_TARGET2_HI_MASK) >> Target2Hi::TIMER_TARGET2_HI_OFFSET
    }

    /// Set **TIMER_TARGET2_HI** `[19:0]`
    ///
    /// timer taget2 high 20 bits
    #[inline(always)]
    pub const fn with_timer_target2_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Target2Hi::TIMER_TARGET2_HI_MASK)
                | ((value << Target2Hi::TIMER_TARGET2_HI_OFFSET)
                    & Target2Hi::TIMER_TARGET2_HI_MASK),
        }
    }
}

impl Default for Target2HiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Target2Hi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Target2HiVal {
        Target2HiVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Target2HiVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Target2HiVal) -> Target2HiVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TARGET2_LO Register
///
/// system timer comp2 value low register
///
/// Address offset: 0x0030
///
/// ## Fields
///
/// - **TIMER_TARGET2_LO** `[31:0]` (RW)
///
///   timer taget2 low 32 bits
#[repr(transparent)]
pub struct Target2Lo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Target2Lo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Target2Lo {
    // ======== Field Constants ========

    /// **TIMER_TARGET2_LO**
    ///
    /// timer taget2 low 32 bits
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TIMER_TARGET2_LO_OFFSET: u32 = 0;
    pub const TIMER_TARGET2_LO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TIMER_TARGET2_LO** `[31:0]` (RW)
    ///
    /// timer taget2 low 32 bits
    #[inline(always)]
    pub fn timer_target2_lo(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_TARGET2_LO_MASK) >> Self::TIMER_TARGET2_LO_OFFSET
    }

    /// Write **TIMER_TARGET2_LO** `[31:0]` (RW)
    ///
    /// timer taget2 low 32 bits
    #[inline(always)]
    pub fn write_timer_target2_lo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TIMER_TARGET2_LO_MASK)
                | ((value << Self::TIMER_TARGET2_LO_OFFSET) & Self::TIMER_TARGET2_LO_MASK)
        });
    }
}

/// TARGET2_LO value type for batch read/modify/write operations
///
/// Use `target2lo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `target2lo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Target2LoVal {
    bits: u32,
}

impl Target2LoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_TARGET2_LO** `[31:0]`
    ///
    /// timer taget2 low 32 bits
    #[inline(always)]
    pub const fn timer_target2_lo(&self) -> u32 {
        (self.bits & Target2Lo::TIMER_TARGET2_LO_MASK) >> Target2Lo::TIMER_TARGET2_LO_OFFSET
    }

    /// Set **TIMER_TARGET2_LO** `[31:0]`
    ///
    /// timer taget2 low 32 bits
    #[inline(always)]
    pub const fn with_timer_target2_lo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Target2Lo::TIMER_TARGET2_LO_MASK)
                | ((value << Target2Lo::TIMER_TARGET2_LO_OFFSET)
                    & Target2Lo::TIMER_TARGET2_LO_MASK),
        }
    }
}

impl Default for Target2LoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Target2Lo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Target2LoVal {
        Target2LoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Target2LoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Target2LoVal) -> Target2LoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TARGET0_CONF Register
///
/// system timer comp0 target mode register
///
/// Address offset: 0x0034
///
/// ## Fields
///
/// - **TARGET0_TIMER_UNIT_SEL** `[31]` (RW)
///
///   select which unit to compare
///
/// - **TARGET0_PERIOD_MODE** `[30]` (RW)
///
///   Set target0 to period mode
///
/// - **TARGET0_PERIOD** `[25:0]` (RW)
///
///   target0 period
#[repr(transparent)]
pub struct Target0Conf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Target0Conf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Target0Conf {
    // ======== Field Constants ========

    /// **TARGET0_TIMER_UNIT_SEL**
    ///
    /// select which unit to compare
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TARGET0_TIMER_UNIT_SEL_OFFSET: u32 = 31;
    pub const TARGET0_TIMER_UNIT_SEL_MASK: u32 = 0x80000000;
    pub const TARGET0_TIMER_UNIT_SEL: u32 = 1 << 31;

    /// **TARGET0_PERIOD_MODE**
    ///
    /// Set target0 to period mode
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const TARGET0_PERIOD_MODE_OFFSET: u32 = 30;
    pub const TARGET0_PERIOD_MODE_MASK: u32 = 0x40000000;
    pub const TARGET0_PERIOD_MODE: u32 = 1 << 30;

    /// **TARGET0_PERIOD**
    ///
    /// target0 period
    ///
    /// Bits `[25:0]`, mask `0x03ffffff`
    pub const TARGET0_PERIOD_OFFSET: u32 = 0;
    pub const TARGET0_PERIOD_MASK: u32 = 0x03ffffff;

    // ======== Field Accessors ========

    /// **TARGET0_TIMER_UNIT_SEL** `[31]` (RW)
    ///
    /// select which unit to compare
    #[inline(always)]
    pub fn target0_timer_unit_sel(&self) -> bool {
        (self.value.read_bits() & Self::TARGET0_TIMER_UNIT_SEL_MASK) != 0
    }

    /// Set TARGET0_TIMER_UNIT_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target0_timer_unit_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::TARGET0_TIMER_UNIT_SEL_MASK);
    }

    /// Clear TARGET0_TIMER_UNIT_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target0_timer_unit_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::TARGET0_TIMER_UNIT_SEL_MASK);
    }

    /// **TARGET0_PERIOD_MODE** `[30]` (RW)
    ///
    /// Set target0 to period mode
    #[inline(always)]
    pub fn target0_period_mode(&self) -> bool {
        (self.value.read_bits() & Self::TARGET0_PERIOD_MODE_MASK) != 0
    }

    /// Set TARGET0_PERIOD_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target0_period_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::TARGET0_PERIOD_MODE_MASK);
    }

    /// Clear TARGET0_PERIOD_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target0_period_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::TARGET0_PERIOD_MODE_MASK);
    }

    /// **TARGET0_PERIOD** `[25:0]` (RW)
    ///
    /// target0 period
    #[inline(always)]
    pub fn target0_period(&self) -> u32 {
        (self.value.read_bits() & Self::TARGET0_PERIOD_MASK) >> Self::TARGET0_PERIOD_OFFSET
    }

    /// Write **TARGET0_PERIOD** `[25:0]` (RW)
    ///
    /// target0 period
    #[inline(always)]
    pub fn write_target0_period(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TARGET0_PERIOD_MASK)
                | ((value << Self::TARGET0_PERIOD_OFFSET) & Self::TARGET0_PERIOD_MASK)
        });
    }
}

/// TARGET0_CONF value type for batch read/modify/write operations
///
/// Use `target0conf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `target0conf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Target0ConfVal {
    bits: u32,
}

impl Target0ConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TARGET0_TIMER_UNIT_SEL** `[31:31]`
    ///
    /// select which unit to compare
    #[inline(always)]
    pub const fn target0_timer_unit_sel(&self) -> bool {
        (self.bits & Target0Conf::TARGET0_TIMER_UNIT_SEL_MASK) != 0
    }

    /// Read **TARGET0_PERIOD_MODE** `[30:30]`
    ///
    /// Set target0 to period mode
    #[inline(always)]
    pub const fn target0_period_mode(&self) -> bool {
        (self.bits & Target0Conf::TARGET0_PERIOD_MODE_MASK) != 0
    }

    /// Read **TARGET0_PERIOD** `[25:0]`
    ///
    /// target0 period
    #[inline(always)]
    pub const fn target0_period(&self) -> u32 {
        (self.bits & Target0Conf::TARGET0_PERIOD_MASK) >> Target0Conf::TARGET0_PERIOD_OFFSET
    }

    /// Set **TARGET0_TIMER_UNIT_SEL** `[31:31]`
    ///
    /// select which unit to compare
    #[inline(always)]
    pub const fn with_target0_timer_unit_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Target0Conf::TARGET0_TIMER_UNIT_SEL_MASK
            } else {
                self.bits & !Target0Conf::TARGET0_TIMER_UNIT_SEL_MASK
            },
        }
    }

    /// Set **TARGET0_PERIOD_MODE** `[30:30]`
    ///
    /// Set target0 to period mode
    #[inline(always)]
    pub const fn with_target0_period_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Target0Conf::TARGET0_PERIOD_MODE_MASK
            } else {
                self.bits & !Target0Conf::TARGET0_PERIOD_MODE_MASK
            },
        }
    }

    /// Set **TARGET0_PERIOD** `[25:0]`
    ///
    /// target0 period
    #[inline(always)]
    pub const fn with_target0_period(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Target0Conf::TARGET0_PERIOD_MASK)
                | ((value << Target0Conf::TARGET0_PERIOD_OFFSET)
                    & Target0Conf::TARGET0_PERIOD_MASK),
        }
    }
}

impl Default for Target0ConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Target0Conf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Target0ConfVal {
        Target0ConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Target0ConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Target0ConfVal) -> Target0ConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TARGET1_CONF Register
///
/// system timer comp1 target mode register
///
/// Address offset: 0x0038
///
/// ## Fields
///
/// - **TARGET1_TIMER_UNIT_SEL** `[31]` (RW)
///
///   select which unit to compare
///
/// - **TARGET1_PERIOD_MODE** `[30]` (RW)
///
///   Set target1 to period mode
///
/// - **TARGET1_PERIOD** `[25:0]` (RW)
///
///   target1 period
#[repr(transparent)]
pub struct Target1Conf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Target1Conf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Target1Conf {
    // ======== Field Constants ========

    /// **TARGET1_TIMER_UNIT_SEL**
    ///
    /// select which unit to compare
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TARGET1_TIMER_UNIT_SEL_OFFSET: u32 = 31;
    pub const TARGET1_TIMER_UNIT_SEL_MASK: u32 = 0x80000000;
    pub const TARGET1_TIMER_UNIT_SEL: u32 = 1 << 31;

    /// **TARGET1_PERIOD_MODE**
    ///
    /// Set target1 to period mode
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const TARGET1_PERIOD_MODE_OFFSET: u32 = 30;
    pub const TARGET1_PERIOD_MODE_MASK: u32 = 0x40000000;
    pub const TARGET1_PERIOD_MODE: u32 = 1 << 30;

    /// **TARGET1_PERIOD**
    ///
    /// target1 period
    ///
    /// Bits `[25:0]`, mask `0x03ffffff`
    pub const TARGET1_PERIOD_OFFSET: u32 = 0;
    pub const TARGET1_PERIOD_MASK: u32 = 0x03ffffff;

    // ======== Field Accessors ========

    /// **TARGET1_TIMER_UNIT_SEL** `[31]` (RW)
    ///
    /// select which unit to compare
    #[inline(always)]
    pub fn target1_timer_unit_sel(&self) -> bool {
        (self.value.read_bits() & Self::TARGET1_TIMER_UNIT_SEL_MASK) != 0
    }

    /// Set TARGET1_TIMER_UNIT_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target1_timer_unit_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::TARGET1_TIMER_UNIT_SEL_MASK);
    }

    /// Clear TARGET1_TIMER_UNIT_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target1_timer_unit_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::TARGET1_TIMER_UNIT_SEL_MASK);
    }

    /// **TARGET1_PERIOD_MODE** `[30]` (RW)
    ///
    /// Set target1 to period mode
    #[inline(always)]
    pub fn target1_period_mode(&self) -> bool {
        (self.value.read_bits() & Self::TARGET1_PERIOD_MODE_MASK) != 0
    }

    /// Set TARGET1_PERIOD_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target1_period_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::TARGET1_PERIOD_MODE_MASK);
    }

    /// Clear TARGET1_PERIOD_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target1_period_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::TARGET1_PERIOD_MODE_MASK);
    }

    /// **TARGET1_PERIOD** `[25:0]` (RW)
    ///
    /// target1 period
    #[inline(always)]
    pub fn target1_period(&self) -> u32 {
        (self.value.read_bits() & Self::TARGET1_PERIOD_MASK) >> Self::TARGET1_PERIOD_OFFSET
    }

    /// Write **TARGET1_PERIOD** `[25:0]` (RW)
    ///
    /// target1 period
    #[inline(always)]
    pub fn write_target1_period(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TARGET1_PERIOD_MASK)
                | ((value << Self::TARGET1_PERIOD_OFFSET) & Self::TARGET1_PERIOD_MASK)
        });
    }
}

/// TARGET1_CONF value type for batch read/modify/write operations
///
/// Use `target1conf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `target1conf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Target1ConfVal {
    bits: u32,
}

impl Target1ConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TARGET1_TIMER_UNIT_SEL** `[31:31]`
    ///
    /// select which unit to compare
    #[inline(always)]
    pub const fn target1_timer_unit_sel(&self) -> bool {
        (self.bits & Target1Conf::TARGET1_TIMER_UNIT_SEL_MASK) != 0
    }

    /// Read **TARGET1_PERIOD_MODE** `[30:30]`
    ///
    /// Set target1 to period mode
    #[inline(always)]
    pub const fn target1_period_mode(&self) -> bool {
        (self.bits & Target1Conf::TARGET1_PERIOD_MODE_MASK) != 0
    }

    /// Read **TARGET1_PERIOD** `[25:0]`
    ///
    /// target1 period
    #[inline(always)]
    pub const fn target1_period(&self) -> u32 {
        (self.bits & Target1Conf::TARGET1_PERIOD_MASK) >> Target1Conf::TARGET1_PERIOD_OFFSET
    }

    /// Set **TARGET1_TIMER_UNIT_SEL** `[31:31]`
    ///
    /// select which unit to compare
    #[inline(always)]
    pub const fn with_target1_timer_unit_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Target1Conf::TARGET1_TIMER_UNIT_SEL_MASK
            } else {
                self.bits & !Target1Conf::TARGET1_TIMER_UNIT_SEL_MASK
            },
        }
    }

    /// Set **TARGET1_PERIOD_MODE** `[30:30]`
    ///
    /// Set target1 to period mode
    #[inline(always)]
    pub const fn with_target1_period_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Target1Conf::TARGET1_PERIOD_MODE_MASK
            } else {
                self.bits & !Target1Conf::TARGET1_PERIOD_MODE_MASK
            },
        }
    }

    /// Set **TARGET1_PERIOD** `[25:0]`
    ///
    /// target1 period
    #[inline(always)]
    pub const fn with_target1_period(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Target1Conf::TARGET1_PERIOD_MASK)
                | ((value << Target1Conf::TARGET1_PERIOD_OFFSET)
                    & Target1Conf::TARGET1_PERIOD_MASK),
        }
    }
}

impl Default for Target1ConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Target1Conf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Target1ConfVal {
        Target1ConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Target1ConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Target1ConfVal) -> Target1ConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TARGET2_CONF Register
///
/// system timer comp2 target mode register
///
/// Address offset: 0x003c
///
/// ## Fields
///
/// - **TARGET2_TIMER_UNIT_SEL** `[31]` (RW)
///
///   select which unit to compare
///
/// - **TARGET2_PERIOD_MODE** `[30]` (RW)
///
///   Set target2 to period mode
///
/// - **TARGET2_PERIOD** `[25:0]` (RW)
///
///   target2 period
#[repr(transparent)]
pub struct Target2Conf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Target2Conf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Target2Conf {
    // ======== Field Constants ========

    /// **TARGET2_TIMER_UNIT_SEL**
    ///
    /// select which unit to compare
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const TARGET2_TIMER_UNIT_SEL_OFFSET: u32 = 31;
    pub const TARGET2_TIMER_UNIT_SEL_MASK: u32 = 0x80000000;
    pub const TARGET2_TIMER_UNIT_SEL: u32 = 1 << 31;

    /// **TARGET2_PERIOD_MODE**
    ///
    /// Set target2 to period mode
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const TARGET2_PERIOD_MODE_OFFSET: u32 = 30;
    pub const TARGET2_PERIOD_MODE_MASK: u32 = 0x40000000;
    pub const TARGET2_PERIOD_MODE: u32 = 1 << 30;

    /// **TARGET2_PERIOD**
    ///
    /// target2 period
    ///
    /// Bits `[25:0]`, mask `0x03ffffff`
    pub const TARGET2_PERIOD_OFFSET: u32 = 0;
    pub const TARGET2_PERIOD_MASK: u32 = 0x03ffffff;

    // ======== Field Accessors ========

    /// **TARGET2_TIMER_UNIT_SEL** `[31]` (RW)
    ///
    /// select which unit to compare
    #[inline(always)]
    pub fn target2_timer_unit_sel(&self) -> bool {
        (self.value.read_bits() & Self::TARGET2_TIMER_UNIT_SEL_MASK) != 0
    }

    /// Set TARGET2_TIMER_UNIT_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target2_timer_unit_sel(&self) {
        self.value
            .modify_bits(|r| r | Self::TARGET2_TIMER_UNIT_SEL_MASK);
    }

    /// Clear TARGET2_TIMER_UNIT_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target2_timer_unit_sel(&self) {
        self.value
            .modify_bits(|r| r & !Self::TARGET2_TIMER_UNIT_SEL_MASK);
    }

    /// **TARGET2_PERIOD_MODE** `[30]` (RW)
    ///
    /// Set target2 to period mode
    #[inline(always)]
    pub fn target2_period_mode(&self) -> bool {
        (self.value.read_bits() & Self::TARGET2_PERIOD_MODE_MASK) != 0
    }

    /// Set TARGET2_PERIOD_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target2_period_mode(&self) {
        self.value
            .modify_bits(|r| r | Self::TARGET2_PERIOD_MODE_MASK);
    }

    /// Clear TARGET2_PERIOD_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target2_period_mode(&self) {
        self.value
            .modify_bits(|r| r & !Self::TARGET2_PERIOD_MODE_MASK);
    }

    /// **TARGET2_PERIOD** `[25:0]` (RW)
    ///
    /// target2 period
    #[inline(always)]
    pub fn target2_period(&self) -> u32 {
        (self.value.read_bits() & Self::TARGET2_PERIOD_MASK) >> Self::TARGET2_PERIOD_OFFSET
    }

    /// Write **TARGET2_PERIOD** `[25:0]` (RW)
    ///
    /// target2 period
    #[inline(always)]
    pub fn write_target2_period(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TARGET2_PERIOD_MASK)
                | ((value << Self::TARGET2_PERIOD_OFFSET) & Self::TARGET2_PERIOD_MASK)
        });
    }
}

/// TARGET2_CONF value type for batch read/modify/write operations
///
/// Use `target2conf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `target2conf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Target2ConfVal {
    bits: u32,
}

impl Target2ConfVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TARGET2_TIMER_UNIT_SEL** `[31:31]`
    ///
    /// select which unit to compare
    #[inline(always)]
    pub const fn target2_timer_unit_sel(&self) -> bool {
        (self.bits & Target2Conf::TARGET2_TIMER_UNIT_SEL_MASK) != 0
    }

    /// Read **TARGET2_PERIOD_MODE** `[30:30]`
    ///
    /// Set target2 to period mode
    #[inline(always)]
    pub const fn target2_period_mode(&self) -> bool {
        (self.bits & Target2Conf::TARGET2_PERIOD_MODE_MASK) != 0
    }

    /// Read **TARGET2_PERIOD** `[25:0]`
    ///
    /// target2 period
    #[inline(always)]
    pub const fn target2_period(&self) -> u32 {
        (self.bits & Target2Conf::TARGET2_PERIOD_MASK) >> Target2Conf::TARGET2_PERIOD_OFFSET
    }

    /// Set **TARGET2_TIMER_UNIT_SEL** `[31:31]`
    ///
    /// select which unit to compare
    #[inline(always)]
    pub const fn with_target2_timer_unit_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Target2Conf::TARGET2_TIMER_UNIT_SEL_MASK
            } else {
                self.bits & !Target2Conf::TARGET2_TIMER_UNIT_SEL_MASK
            },
        }
    }

    /// Set **TARGET2_PERIOD_MODE** `[30:30]`
    ///
    /// Set target2 to period mode
    #[inline(always)]
    pub const fn with_target2_period_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Target2Conf::TARGET2_PERIOD_MODE_MASK
            } else {
                self.bits & !Target2Conf::TARGET2_PERIOD_MODE_MASK
            },
        }
    }

    /// Set **TARGET2_PERIOD** `[25:0]`
    ///
    /// target2 period
    #[inline(always)]
    pub const fn with_target2_period(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Target2Conf::TARGET2_PERIOD_MASK)
                | ((value << Target2Conf::TARGET2_PERIOD_OFFSET)
                    & Target2Conf::TARGET2_PERIOD_MASK),
        }
    }
}

impl Default for Target2ConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Target2Conf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Target2ConfVal {
        Target2ConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Target2ConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Target2ConfVal) -> Target2ConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// UNIT0_VALUE_HI Register
///
/// system timer unit0 value high register
///
/// Address offset: 0x0040
///
/// ## Fields
///
/// - **TIMER_UNIT0_VALUE_HI** `[19:0]` (RO)
///
///   timer read value high 20bits
#[repr(transparent)]
pub struct Unit0ValueHi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Unit0ValueHi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Unit0ValueHi {
    // ======== Field Constants ========

    /// **TIMER_UNIT0_VALUE_HI**
    ///
    /// timer read value high 20bits
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_UNIT0_VALUE_HI_OFFSET: u32 = 0;
    pub const TIMER_UNIT0_VALUE_HI_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_UNIT0_VALUE_HI** `[19:0]` (RO)
    ///
    /// timer read value high 20bits
    #[inline(always)]
    pub fn timer_unit0_value_hi(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_UNIT0_VALUE_HI_MASK)
            >> Self::TIMER_UNIT0_VALUE_HI_OFFSET
    }
}

/// UNIT0_VALUE_HI value type for batch read/modify/write operations
///
/// Use `unit0valuehi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `unit0valuehi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Unit0ValueHiVal {
    bits: u32,
}

impl Unit0ValueHiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_UNIT0_VALUE_HI** `[19:0]`
    ///
    /// timer read value high 20bits
    #[inline(always)]
    pub const fn timer_unit0_value_hi(&self) -> u32 {
        (self.bits & Unit0ValueHi::TIMER_UNIT0_VALUE_HI_MASK)
            >> Unit0ValueHi::TIMER_UNIT0_VALUE_HI_OFFSET
    }
}

impl Default for Unit0ValueHiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Unit0ValueHi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Unit0ValueHiVal {
        Unit0ValueHiVal::from_bits(self.value.read_bits())
    }
}

/// UNIT0_VALUE_LO Register
///
/// system timer unit0 value low register
///
/// Address offset: 0x0044
///
/// ## Fields
///
/// - **TIMER_UNIT0_VALUE_LO** `[31:0]` (RO)
///
///   timer read value low 32bits
#[repr(transparent)]
pub struct Unit0ValueLo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Unit0ValueLo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Unit0ValueLo {
    // ======== Field Constants ========

    /// **TIMER_UNIT0_VALUE_LO**
    ///
    /// timer read value low 32bits
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TIMER_UNIT0_VALUE_LO_OFFSET: u32 = 0;
    pub const TIMER_UNIT0_VALUE_LO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TIMER_UNIT0_VALUE_LO** `[31:0]` (RO)
    ///
    /// timer read value low 32bits
    #[inline(always)]
    pub fn timer_unit0_value_lo(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_UNIT0_VALUE_LO_MASK)
            >> Self::TIMER_UNIT0_VALUE_LO_OFFSET
    }
}

/// UNIT0_VALUE_LO value type for batch read/modify/write operations
///
/// Use `unit0valuelo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `unit0valuelo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Unit0ValueLoVal {
    bits: u32,
}

impl Unit0ValueLoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_UNIT0_VALUE_LO** `[31:0]`
    ///
    /// timer read value low 32bits
    #[inline(always)]
    pub const fn timer_unit0_value_lo(&self) -> u32 {
        (self.bits & Unit0ValueLo::TIMER_UNIT0_VALUE_LO_MASK)
            >> Unit0ValueLo::TIMER_UNIT0_VALUE_LO_OFFSET
    }
}

impl Default for Unit0ValueLoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Unit0ValueLo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Unit0ValueLoVal {
        Unit0ValueLoVal::from_bits(self.value.read_bits())
    }
}

/// UNIT1_VALUE_HI Register
///
/// system timer unit1 value high register
///
/// Address offset: 0x0048
///
/// ## Fields
///
/// - **TIMER_UNIT1_VALUE_HI** `[19:0]` (RO)
///
///   timer read value high 20bits
#[repr(transparent)]
pub struct Unit1ValueHi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Unit1ValueHi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Unit1ValueHi {
    // ======== Field Constants ========

    /// **TIMER_UNIT1_VALUE_HI**
    ///
    /// timer read value high 20bits
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TIMER_UNIT1_VALUE_HI_OFFSET: u32 = 0;
    pub const TIMER_UNIT1_VALUE_HI_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TIMER_UNIT1_VALUE_HI** `[19:0]` (RO)
    ///
    /// timer read value high 20bits
    #[inline(always)]
    pub fn timer_unit1_value_hi(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_UNIT1_VALUE_HI_MASK)
            >> Self::TIMER_UNIT1_VALUE_HI_OFFSET
    }
}

/// UNIT1_VALUE_HI value type for batch read/modify/write operations
///
/// Use `unit1valuehi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `unit1valuehi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Unit1ValueHiVal {
    bits: u32,
}

impl Unit1ValueHiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_UNIT1_VALUE_HI** `[19:0]`
    ///
    /// timer read value high 20bits
    #[inline(always)]
    pub const fn timer_unit1_value_hi(&self) -> u32 {
        (self.bits & Unit1ValueHi::TIMER_UNIT1_VALUE_HI_MASK)
            >> Unit1ValueHi::TIMER_UNIT1_VALUE_HI_OFFSET
    }
}

impl Default for Unit1ValueHiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Unit1ValueHi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Unit1ValueHiVal {
        Unit1ValueHiVal::from_bits(self.value.read_bits())
    }
}

/// UNIT1_VALUE_LO Register
///
/// system timer unit1 value low register
///
/// Address offset: 0x004c
///
/// ## Fields
///
/// - **TIMER_UNIT1_VALUE_LO** `[31:0]` (RO)
///
///   timer read value low 32bits
#[repr(transparent)]
pub struct Unit1ValueLo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Unit1ValueLo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Unit1ValueLo {
    // ======== Field Constants ========

    /// **TIMER_UNIT1_VALUE_LO**
    ///
    /// timer read value low 32bits
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TIMER_UNIT1_VALUE_LO_OFFSET: u32 = 0;
    pub const TIMER_UNIT1_VALUE_LO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TIMER_UNIT1_VALUE_LO** `[31:0]` (RO)
    ///
    /// timer read value low 32bits
    #[inline(always)]
    pub fn timer_unit1_value_lo(&self) -> u32 {
        (self.value.read_bits() & Self::TIMER_UNIT1_VALUE_LO_MASK)
            >> Self::TIMER_UNIT1_VALUE_LO_OFFSET
    }
}

/// UNIT1_VALUE_LO value type for batch read/modify/write operations
///
/// Use `unit1valuelo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `unit1valuelo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Unit1ValueLoVal {
    bits: u32,
}

impl Unit1ValueLoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TIMER_UNIT1_VALUE_LO** `[31:0]`
    ///
    /// timer read value low 32bits
    #[inline(always)]
    pub const fn timer_unit1_value_lo(&self) -> u32 {
        (self.bits & Unit1ValueLo::TIMER_UNIT1_VALUE_LO_MASK)
            >> Unit1ValueLo::TIMER_UNIT1_VALUE_LO_OFFSET
    }
}

impl Default for Unit1ValueLoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Unit1ValueLo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Unit1ValueLoVal {
        Unit1ValueLoVal::from_bits(self.value.read_bits())
    }
}

/// COMP0_LOAD Register
///
/// system timer comp0 conf sync register
///
/// Address offset: 0x0050
///
/// ## Fields
///
/// - **TIMER_COMP0_LOAD** `[0]` (WO)
///
///   timer comp0 sync enable signal
#[repr(transparent)]
pub struct Comp0Load {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Comp0Load {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Comp0Load {
    // ======== Field Constants ========

    /// **TIMER_COMP0_LOAD**
    ///
    /// timer comp0 sync enable signal
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TIMER_COMP0_LOAD_OFFSET: u32 = 0;
    pub const TIMER_COMP0_LOAD_MASK: u32 = 0x00000001;
    pub const TIMER_COMP0_LOAD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **TIMER_COMP0_LOAD** (WO)
    ///
    /// timer comp0 sync enable signal
    #[inline(always)]
    pub fn trigger_timer_comp0_load(&self) {
        self.value.write_bits(Self::TIMER_COMP0_LOAD_MASK);
    }
}

/// COMP0_LOAD value type for batch read/modify/write operations
///
/// Use `comp0load.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `comp0load.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Comp0LoadVal {
    bits: u32,
}

impl Comp0LoadVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **TIMER_COMP0_LOAD** `[0:0]`
    ///
    /// timer comp0 sync enable signal
    #[inline(always)]
    pub const fn with_timer_comp0_load(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Comp0Load::TIMER_COMP0_LOAD_MASK
            } else {
                self.bits & !Comp0Load::TIMER_COMP0_LOAD_MASK
            },
        }
    }
}

impl Default for Comp0LoadVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Comp0Load {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Comp0LoadVal) {
        self.value.write_bits(val.bits())
    }
}

/// COMP1_LOAD Register
///
/// system timer comp1 conf sync register
///
/// Address offset: 0x0054
///
/// ## Fields
///
/// - **TIMER_COMP1_LOAD** `[0]` (WO)
///
///   timer comp1 sync enable signal
#[repr(transparent)]
pub struct Comp1Load {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Comp1Load {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Comp1Load {
    // ======== Field Constants ========

    /// **TIMER_COMP1_LOAD**
    ///
    /// timer comp1 sync enable signal
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TIMER_COMP1_LOAD_OFFSET: u32 = 0;
    pub const TIMER_COMP1_LOAD_MASK: u32 = 0x00000001;
    pub const TIMER_COMP1_LOAD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **TIMER_COMP1_LOAD** (WO)
    ///
    /// timer comp1 sync enable signal
    #[inline(always)]
    pub fn trigger_timer_comp1_load(&self) {
        self.value.write_bits(Self::TIMER_COMP1_LOAD_MASK);
    }
}

/// COMP1_LOAD value type for batch read/modify/write operations
///
/// Use `comp1load.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `comp1load.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Comp1LoadVal {
    bits: u32,
}

impl Comp1LoadVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **TIMER_COMP1_LOAD** `[0:0]`
    ///
    /// timer comp1 sync enable signal
    #[inline(always)]
    pub const fn with_timer_comp1_load(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Comp1Load::TIMER_COMP1_LOAD_MASK
            } else {
                self.bits & !Comp1Load::TIMER_COMP1_LOAD_MASK
            },
        }
    }
}

impl Default for Comp1LoadVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Comp1Load {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Comp1LoadVal) {
        self.value.write_bits(val.bits())
    }
}

/// COMP2_LOAD Register
///
/// system timer comp2 conf sync register
///
/// Address offset: 0x0058
///
/// ## Fields
///
/// - **TIMER_COMP2_LOAD** `[0]` (WO)
///
///   timer comp2 sync enable signal
#[repr(transparent)]
pub struct Comp2Load {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Comp2Load {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Comp2Load {
    // ======== Field Constants ========

    /// **TIMER_COMP2_LOAD**
    ///
    /// timer comp2 sync enable signal
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TIMER_COMP2_LOAD_OFFSET: u32 = 0;
    pub const TIMER_COMP2_LOAD_MASK: u32 = 0x00000001;
    pub const TIMER_COMP2_LOAD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **TIMER_COMP2_LOAD** (WO)
    ///
    /// timer comp2 sync enable signal
    #[inline(always)]
    pub fn trigger_timer_comp2_load(&self) {
        self.value.write_bits(Self::TIMER_COMP2_LOAD_MASK);
    }
}

/// COMP2_LOAD value type for batch read/modify/write operations
///
/// Use `comp2load.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `comp2load.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Comp2LoadVal {
    bits: u32,
}

impl Comp2LoadVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **TIMER_COMP2_LOAD** `[0:0]`
    ///
    /// timer comp2 sync enable signal
    #[inline(always)]
    pub const fn with_timer_comp2_load(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Comp2Load::TIMER_COMP2_LOAD_MASK
            } else {
                self.bits & !Comp2Load::TIMER_COMP2_LOAD_MASK
            },
        }
    }
}

impl Default for Comp2LoadVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Comp2Load {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Comp2LoadVal) {
        self.value.write_bits(val.bits())
    }
}

/// UNIT0_LOAD Register
///
/// system timer unit0 conf sync register
///
/// Address offset: 0x005c
///
/// ## Fields
///
/// - **TIMER_UNIT0_LOAD** `[0]` (WO)
///
///   timer unit0 sync enable signal
#[repr(transparent)]
pub struct Unit0Load {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Unit0Load {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Unit0Load {
    // ======== Field Constants ========

    /// **TIMER_UNIT0_LOAD**
    ///
    /// timer unit0 sync enable signal
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TIMER_UNIT0_LOAD_OFFSET: u32 = 0;
    pub const TIMER_UNIT0_LOAD_MASK: u32 = 0x00000001;
    pub const TIMER_UNIT0_LOAD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **TIMER_UNIT0_LOAD** (WO)
    ///
    /// timer unit0 sync enable signal
    #[inline(always)]
    pub fn trigger_timer_unit0_load(&self) {
        self.value.write_bits(Self::TIMER_UNIT0_LOAD_MASK);
    }
}

/// UNIT0_LOAD value type for batch read/modify/write operations
///
/// Use `unit0load.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `unit0load.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Unit0LoadVal {
    bits: u32,
}

impl Unit0LoadVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **TIMER_UNIT0_LOAD** `[0:0]`
    ///
    /// timer unit0 sync enable signal
    #[inline(always)]
    pub const fn with_timer_unit0_load(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Unit0Load::TIMER_UNIT0_LOAD_MASK
            } else {
                self.bits & !Unit0Load::TIMER_UNIT0_LOAD_MASK
            },
        }
    }
}

impl Default for Unit0LoadVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Unit0Load {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Unit0LoadVal) {
        self.value.write_bits(val.bits())
    }
}

/// UNIT1_LOAD Register
///
/// system timer unit1 conf sync register
///
/// Address offset: 0x0060
///
/// ## Fields
///
/// - **TIMER_UNIT1_LOAD** `[0]` (WO)
///
///   timer unit1 sync enable signal
#[repr(transparent)]
pub struct Unit1Load {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Unit1Load {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Unit1Load {
    // ======== Field Constants ========

    /// **TIMER_UNIT1_LOAD**
    ///
    /// timer unit1 sync enable signal
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TIMER_UNIT1_LOAD_OFFSET: u32 = 0;
    pub const TIMER_UNIT1_LOAD_MASK: u32 = 0x00000001;
    pub const TIMER_UNIT1_LOAD: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **TIMER_UNIT1_LOAD** (WO)
    ///
    /// timer unit1 sync enable signal
    #[inline(always)]
    pub fn trigger_timer_unit1_load(&self) {
        self.value.write_bits(Self::TIMER_UNIT1_LOAD_MASK);
    }
}

/// UNIT1_LOAD value type for batch read/modify/write operations
///
/// Use `unit1load.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `unit1load.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Unit1LoadVal {
    bits: u32,
}

impl Unit1LoadVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Set **TIMER_UNIT1_LOAD** `[0:0]`
    ///
    /// timer unit1 sync enable signal
    #[inline(always)]
    pub const fn with_timer_unit1_load(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Unit1Load::TIMER_UNIT1_LOAD_MASK
            } else {
                self.bits & !Unit1Load::TIMER_UNIT1_LOAD_MASK
            },
        }
    }
}

impl Default for Unit1LoadVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl Unit1Load {
    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Unit1LoadVal) {
        self.value.write_bits(val.bits())
    }
}

/// INT_ENA Register
///
/// systimer interrupt enable register
///
/// Address offset: 0x0064
///
/// ## Fields
///
/// - **TARGET2_INT_ENA** `[2]` (RW)
///
///   interupt2 enable
///
/// - **TARGET1_INT_ENA** `[1]` (RW)
///
///   interupt1 enable
///
/// - **TARGET0_INT_ENA** `[0]` (RW)
///
///   interupt0 enable
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

    /// **TARGET2_INT_ENA**
    ///
    /// interupt2 enable
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TARGET2_INT_ENA_OFFSET: u32 = 2;
    pub const TARGET2_INT_ENA_MASK: u32 = 0x00000004;
    pub const TARGET2_INT_ENA: u32 = 1 << 2;

    /// **TARGET1_INT_ENA**
    ///
    /// interupt1 enable
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TARGET1_INT_ENA_OFFSET: u32 = 1;
    pub const TARGET1_INT_ENA_MASK: u32 = 0x00000002;
    pub const TARGET1_INT_ENA: u32 = 1 << 1;

    /// **TARGET0_INT_ENA**
    ///
    /// interupt0 enable
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TARGET0_INT_ENA_OFFSET: u32 = 0;
    pub const TARGET0_INT_ENA_MASK: u32 = 0x00000001;
    pub const TARGET0_INT_ENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TARGET2_INT_ENA** `[2]` (RW)
    ///
    /// interupt2 enable
    #[inline(always)]
    pub fn target2_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TARGET2_INT_ENA_MASK) != 0
    }

    /// Set TARGET2_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target2_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::TARGET2_INT_ENA_MASK);
    }

    /// Clear TARGET2_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target2_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::TARGET2_INT_ENA_MASK);
    }

    /// **TARGET1_INT_ENA** `[1]` (RW)
    ///
    /// interupt1 enable
    #[inline(always)]
    pub fn target1_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TARGET1_INT_ENA_MASK) != 0
    }

    /// Set TARGET1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target1_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::TARGET1_INT_ENA_MASK);
    }

    /// Clear TARGET1_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target1_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::TARGET1_INT_ENA_MASK);
    }

    /// **TARGET0_INT_ENA** `[0]` (RW)
    ///
    /// interupt0 enable
    #[inline(always)]
    pub fn target0_int_ena(&self) -> bool {
        (self.value.read_bits() & Self::TARGET0_INT_ENA_MASK) != 0
    }

    /// Set TARGET0_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target0_int_ena(&self) {
        self.value.modify_bits(|r| r | Self::TARGET0_INT_ENA_MASK);
    }

    /// Clear TARGET0_INT_ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target0_int_ena(&self) {
        self.value.modify_bits(|r| r & !Self::TARGET0_INT_ENA_MASK);
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

    /// Read **TARGET2_INT_ENA** `[2:2]`
    ///
    /// interupt2 enable
    #[inline(always)]
    pub const fn target2_int_ena(&self) -> bool {
        (self.bits & IntEna::TARGET2_INT_ENA_MASK) != 0
    }

    /// Read **TARGET1_INT_ENA** `[1:1]`
    ///
    /// interupt1 enable
    #[inline(always)]
    pub const fn target1_int_ena(&self) -> bool {
        (self.bits & IntEna::TARGET1_INT_ENA_MASK) != 0
    }

    /// Read **TARGET0_INT_ENA** `[0:0]`
    ///
    /// interupt0 enable
    #[inline(always)]
    pub const fn target0_int_ena(&self) -> bool {
        (self.bits & IntEna::TARGET0_INT_ENA_MASK) != 0
    }

    /// Set **TARGET2_INT_ENA** `[2:2]`
    ///
    /// interupt2 enable
    #[inline(always)]
    pub const fn with_target2_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TARGET2_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TARGET2_INT_ENA_MASK
            },
        }
    }

    /// Set **TARGET1_INT_ENA** `[1:1]`
    ///
    /// interupt1 enable
    #[inline(always)]
    pub const fn with_target1_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TARGET1_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TARGET1_INT_ENA_MASK
            },
        }
    }

    /// Set **TARGET0_INT_ENA** `[0:0]`
    ///
    /// interupt0 enable
    #[inline(always)]
    pub const fn with_target0_int_ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntEna::TARGET0_INT_ENA_MASK
            } else {
                self.bits & !IntEna::TARGET0_INT_ENA_MASK
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

/// INT_RAW Register
///
/// systimer interrupt raw register
///
/// Address offset: 0x0068
///
/// ## Fields
///
/// - **TARGET2_INT_RAW** `[2]` (RW)
///
///   interupt2 raw
///
/// - **TARGET1_INT_RAW** `[1]` (RW)
///
///   interupt1 raw
///
/// - **TARGET0_INT_RAW** `[0]` (RW)
///
///   interupt0 raw
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

    /// **TARGET2_INT_RAW**
    ///
    /// interupt2 raw
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TARGET2_INT_RAW_OFFSET: u32 = 2;
    pub const TARGET2_INT_RAW_MASK: u32 = 0x00000004;
    pub const TARGET2_INT_RAW: u32 = 1 << 2;

    /// **TARGET1_INT_RAW**
    ///
    /// interupt1 raw
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TARGET1_INT_RAW_OFFSET: u32 = 1;
    pub const TARGET1_INT_RAW_MASK: u32 = 0x00000002;
    pub const TARGET1_INT_RAW: u32 = 1 << 1;

    /// **TARGET0_INT_RAW**
    ///
    /// interupt0 raw
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TARGET0_INT_RAW_OFFSET: u32 = 0;
    pub const TARGET0_INT_RAW_MASK: u32 = 0x00000001;
    pub const TARGET0_INT_RAW: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TARGET2_INT_RAW** `[2]` (RW)
    ///
    /// interupt2 raw
    #[inline(always)]
    pub fn target2_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TARGET2_INT_RAW_MASK) != 0
    }

    /// Set TARGET2_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target2_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::TARGET2_INT_RAW_MASK);
    }

    /// Clear TARGET2_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target2_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::TARGET2_INT_RAW_MASK);
    }

    /// **TARGET1_INT_RAW** `[1]` (RW)
    ///
    /// interupt1 raw
    #[inline(always)]
    pub fn target1_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TARGET1_INT_RAW_MASK) != 0
    }

    /// Set TARGET1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target1_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::TARGET1_INT_RAW_MASK);
    }

    /// Clear TARGET1_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target1_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::TARGET1_INT_RAW_MASK);
    }

    /// **TARGET0_INT_RAW** `[0]` (RW)
    ///
    /// interupt0 raw
    #[inline(always)]
    pub fn target0_int_raw(&self) -> bool {
        (self.value.read_bits() & Self::TARGET0_INT_RAW_MASK) != 0
    }

    /// Set TARGET0_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_target0_int_raw(&self) {
        self.value.modify_bits(|r| r | Self::TARGET0_INT_RAW_MASK);
    }

    /// Clear TARGET0_INT_RAW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_target0_int_raw(&self) {
        self.value.modify_bits(|r| r & !Self::TARGET0_INT_RAW_MASK);
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

    /// Read **TARGET2_INT_RAW** `[2:2]`
    ///
    /// interupt2 raw
    #[inline(always)]
    pub const fn target2_int_raw(&self) -> bool {
        (self.bits & IntRaw::TARGET2_INT_RAW_MASK) != 0
    }

    /// Read **TARGET1_INT_RAW** `[1:1]`
    ///
    /// interupt1 raw
    #[inline(always)]
    pub const fn target1_int_raw(&self) -> bool {
        (self.bits & IntRaw::TARGET1_INT_RAW_MASK) != 0
    }

    /// Read **TARGET0_INT_RAW** `[0:0]`
    ///
    /// interupt0 raw
    #[inline(always)]
    pub const fn target0_int_raw(&self) -> bool {
        (self.bits & IntRaw::TARGET0_INT_RAW_MASK) != 0
    }

    /// Set **TARGET2_INT_RAW** `[2:2]`
    ///
    /// interupt2 raw
    #[inline(always)]
    pub const fn with_target2_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TARGET2_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TARGET2_INT_RAW_MASK
            },
        }
    }

    /// Set **TARGET1_INT_RAW** `[1:1]`
    ///
    /// interupt1 raw
    #[inline(always)]
    pub const fn with_target1_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TARGET1_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TARGET1_INT_RAW_MASK
            },
        }
    }

    /// Set **TARGET0_INT_RAW** `[0:0]`
    ///
    /// interupt0 raw
    #[inline(always)]
    pub const fn with_target0_int_raw(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntRaw::TARGET0_INT_RAW_MASK
            } else {
                self.bits & !IntRaw::TARGET0_INT_RAW_MASK
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

/// INT_CLR Register
///
/// systimer interrupt clear register
///
/// Address offset: 0x006c
///
/// ## Fields
///
/// - **TARGET2_INT_CLR** `[2]` (WO)
///
///   interupt2 clear
///
/// - **TARGET1_INT_CLR** `[1]` (WO)
///
///   interupt1 clear
///
/// - **TARGET0_INT_CLR** `[0]` (WO)
///
///   interupt0 clear
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

    /// **TARGET2_INT_CLR**
    ///
    /// interupt2 clear
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TARGET2_INT_CLR_OFFSET: u32 = 2;
    pub const TARGET2_INT_CLR_MASK: u32 = 0x00000004;
    pub const TARGET2_INT_CLR: u32 = 1 << 2;

    /// **TARGET1_INT_CLR**
    ///
    /// interupt1 clear
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TARGET1_INT_CLR_OFFSET: u32 = 1;
    pub const TARGET1_INT_CLR_MASK: u32 = 0x00000002;
    pub const TARGET1_INT_CLR: u32 = 1 << 1;

    /// **TARGET0_INT_CLR**
    ///
    /// interupt0 clear
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TARGET0_INT_CLR_OFFSET: u32 = 0;
    pub const TARGET0_INT_CLR_MASK: u32 = 0x00000001;
    pub const TARGET0_INT_CLR: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// Trigger **TARGET2_INT_CLR** (WO)
    ///
    /// interupt2 clear
    #[inline(always)]
    pub fn trigger_target2_int_clr(&self) {
        self.value.write_bits(Self::TARGET2_INT_CLR_MASK);
    }

    /// Trigger **TARGET1_INT_CLR** (WO)
    ///
    /// interupt1 clear
    #[inline(always)]
    pub fn trigger_target1_int_clr(&self) {
        self.value.write_bits(Self::TARGET1_INT_CLR_MASK);
    }

    /// Trigger **TARGET0_INT_CLR** (WO)
    ///
    /// interupt0 clear
    #[inline(always)]
    pub fn trigger_target0_int_clr(&self) {
        self.value.write_bits(Self::TARGET0_INT_CLR_MASK);
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

    /// Set **TARGET2_INT_CLR** `[2:2]`
    ///
    /// interupt2 clear
    #[inline(always)]
    pub const fn with_target2_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TARGET2_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TARGET2_INT_CLR_MASK
            },
        }
    }

    /// Set **TARGET1_INT_CLR** `[1:1]`
    ///
    /// interupt1 clear
    #[inline(always)]
    pub const fn with_target1_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TARGET1_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TARGET1_INT_CLR_MASK
            },
        }
    }

    /// Set **TARGET0_INT_CLR** `[0:0]`
    ///
    /// interupt0 clear
    #[inline(always)]
    pub const fn with_target0_int_clr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | IntClr::TARGET0_INT_CLR_MASK
            } else {
                self.bits & !IntClr::TARGET0_INT_CLR_MASK
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

/// INT_ST Register
///
/// systimer interrupt status register
///
/// Address offset: 0x0070
///
/// ## Fields
///
/// - **TARGET2_INT_ST** `[2]` (RO)
///
///   interupt2 status
///
/// - **TARGET1_INT_ST** `[1]` (RO)
///
///   interupt1 status
///
/// - **TARGET0_INT_ST** `[0]` (RO)
///
///   interupt0 status
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

    /// **TARGET2_INT_ST**
    ///
    /// interupt2 status
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TARGET2_INT_ST_OFFSET: u32 = 2;
    pub const TARGET2_INT_ST_MASK: u32 = 0x00000004;
    pub const TARGET2_INT_ST: u32 = 1 << 2;

    /// **TARGET1_INT_ST**
    ///
    /// interupt1 status
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TARGET1_INT_ST_OFFSET: u32 = 1;
    pub const TARGET1_INT_ST_MASK: u32 = 0x00000002;
    pub const TARGET1_INT_ST: u32 = 1 << 1;

    /// **TARGET0_INT_ST**
    ///
    /// interupt0 status
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TARGET0_INT_ST_OFFSET: u32 = 0;
    pub const TARGET0_INT_ST_MASK: u32 = 0x00000001;
    pub const TARGET0_INT_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TARGET2_INT_ST** `[2]` (RO)
    ///
    /// interupt2 status
    #[inline(always)]
    pub fn target2_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TARGET2_INT_ST_MASK) != 0
    }

    /// **TARGET1_INT_ST** `[1]` (RO)
    ///
    /// interupt1 status
    #[inline(always)]
    pub fn target1_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TARGET1_INT_ST_MASK) != 0
    }

    /// **TARGET0_INT_ST** `[0]` (RO)
    ///
    /// interupt0 status
    #[inline(always)]
    pub fn target0_int_st(&self) -> bool {
        (self.value.read_bits() & Self::TARGET0_INT_ST_MASK) != 0
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

    /// Read **TARGET2_INT_ST** `[2:2]`
    ///
    /// interupt2 status
    #[inline(always)]
    pub const fn target2_int_st(&self) -> bool {
        (self.bits & IntSt::TARGET2_INT_ST_MASK) != 0
    }

    /// Read **TARGET1_INT_ST** `[1:1]`
    ///
    /// interupt1 status
    #[inline(always)]
    pub const fn target1_int_st(&self) -> bool {
        (self.bits & IntSt::TARGET1_INT_ST_MASK) != 0
    }

    /// Read **TARGET0_INT_ST** `[0:0]`
    ///
    /// interupt0 status
    #[inline(always)]
    pub const fn target0_int_st(&self) -> bool {
        (self.bits & IntSt::TARGET0_INT_ST_MASK) != 0
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

/// REAL_TARGET0_LO Register
///
/// system timer comp0 actual target value low register
///
/// Address offset: 0x0074
///
/// ## Fields
///
/// - **TARGET0_LO_RO** `[31:0]` (RO)
///
///   actual target value value low 32bits
#[repr(transparent)]
pub struct RealTarget0Lo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RealTarget0Lo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RealTarget0Lo {
    // ======== Field Constants ========

    /// **TARGET0_LO_RO**
    ///
    /// actual target value value low 32bits
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TARGET0_LO_RO_OFFSET: u32 = 0;
    pub const TARGET0_LO_RO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TARGET0_LO_RO** `[31:0]` (RO)
    ///
    /// actual target value value low 32bits
    #[inline(always)]
    pub fn target0_lo_ro(&self) -> u32 {
        (self.value.read_bits() & Self::TARGET0_LO_RO_MASK) >> Self::TARGET0_LO_RO_OFFSET
    }
}

/// REAL_TARGET0_LO value type for batch read/modify/write operations
///
/// Use `realtarget0lo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `realtarget0lo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RealTarget0LoVal {
    bits: u32,
}

impl RealTarget0LoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TARGET0_LO_RO** `[31:0]`
    ///
    /// actual target value value low 32bits
    #[inline(always)]
    pub const fn target0_lo_ro(&self) -> u32 {
        (self.bits & RealTarget0Lo::TARGET0_LO_RO_MASK) >> RealTarget0Lo::TARGET0_LO_RO_OFFSET
    }
}

impl Default for RealTarget0LoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RealTarget0Lo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RealTarget0LoVal {
        RealTarget0LoVal::from_bits(self.value.read_bits())
    }
}

/// REAL_TARGET0_HI Register
///
/// system timer comp0 actual target value high register
///
/// Address offset: 0x0078
///
/// ## Fields
///
/// - **TARGET0_HI_RO** `[19:0]` (RO)
///
///   actual target value value high 20bits
#[repr(transparent)]
pub struct RealTarget0Hi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RealTarget0Hi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RealTarget0Hi {
    // ======== Field Constants ========

    /// **TARGET0_HI_RO**
    ///
    /// actual target value value high 20bits
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TARGET0_HI_RO_OFFSET: u32 = 0;
    pub const TARGET0_HI_RO_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TARGET0_HI_RO** `[19:0]` (RO)
    ///
    /// actual target value value high 20bits
    #[inline(always)]
    pub fn target0_hi_ro(&self) -> u32 {
        (self.value.read_bits() & Self::TARGET0_HI_RO_MASK) >> Self::TARGET0_HI_RO_OFFSET
    }
}

/// REAL_TARGET0_HI value type for batch read/modify/write operations
///
/// Use `realtarget0hi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `realtarget0hi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RealTarget0HiVal {
    bits: u32,
}

impl RealTarget0HiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TARGET0_HI_RO** `[19:0]`
    ///
    /// actual target value value high 20bits
    #[inline(always)]
    pub const fn target0_hi_ro(&self) -> u32 {
        (self.bits & RealTarget0Hi::TARGET0_HI_RO_MASK) >> RealTarget0Hi::TARGET0_HI_RO_OFFSET
    }
}

impl Default for RealTarget0HiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RealTarget0Hi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RealTarget0HiVal {
        RealTarget0HiVal::from_bits(self.value.read_bits())
    }
}

/// REAL_TARGET1_LO Register
///
/// system timer comp1 actual target value low register
///
/// Address offset: 0x007c
///
/// ## Fields
///
/// - **TARGET1_LO_RO** `[31:0]` (RO)
///
///   actual target value value low 32bits
#[repr(transparent)]
pub struct RealTarget1Lo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RealTarget1Lo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RealTarget1Lo {
    // ======== Field Constants ========

    /// **TARGET1_LO_RO**
    ///
    /// actual target value value low 32bits
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TARGET1_LO_RO_OFFSET: u32 = 0;
    pub const TARGET1_LO_RO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TARGET1_LO_RO** `[31:0]` (RO)
    ///
    /// actual target value value low 32bits
    #[inline(always)]
    pub fn target1_lo_ro(&self) -> u32 {
        (self.value.read_bits() & Self::TARGET1_LO_RO_MASK) >> Self::TARGET1_LO_RO_OFFSET
    }
}

/// REAL_TARGET1_LO value type for batch read/modify/write operations
///
/// Use `realtarget1lo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `realtarget1lo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RealTarget1LoVal {
    bits: u32,
}

impl RealTarget1LoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TARGET1_LO_RO** `[31:0]`
    ///
    /// actual target value value low 32bits
    #[inline(always)]
    pub const fn target1_lo_ro(&self) -> u32 {
        (self.bits & RealTarget1Lo::TARGET1_LO_RO_MASK) >> RealTarget1Lo::TARGET1_LO_RO_OFFSET
    }
}

impl Default for RealTarget1LoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RealTarget1Lo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RealTarget1LoVal {
        RealTarget1LoVal::from_bits(self.value.read_bits())
    }
}

/// REAL_TARGET1_HI Register
///
/// system timer comp1 actual target value high register
///
/// Address offset: 0x0080
///
/// ## Fields
///
/// - **TARGET1_HI_RO** `[19:0]` (RO)
///
///   actual target value value high 20bits
#[repr(transparent)]
pub struct RealTarget1Hi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RealTarget1Hi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RealTarget1Hi {
    // ======== Field Constants ========

    /// **TARGET1_HI_RO**
    ///
    /// actual target value value high 20bits
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TARGET1_HI_RO_OFFSET: u32 = 0;
    pub const TARGET1_HI_RO_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TARGET1_HI_RO** `[19:0]` (RO)
    ///
    /// actual target value value high 20bits
    #[inline(always)]
    pub fn target1_hi_ro(&self) -> u32 {
        (self.value.read_bits() & Self::TARGET1_HI_RO_MASK) >> Self::TARGET1_HI_RO_OFFSET
    }
}

/// REAL_TARGET1_HI value type for batch read/modify/write operations
///
/// Use `realtarget1hi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `realtarget1hi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RealTarget1HiVal {
    bits: u32,
}

impl RealTarget1HiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TARGET1_HI_RO** `[19:0]`
    ///
    /// actual target value value high 20bits
    #[inline(always)]
    pub const fn target1_hi_ro(&self) -> u32 {
        (self.bits & RealTarget1Hi::TARGET1_HI_RO_MASK) >> RealTarget1Hi::TARGET1_HI_RO_OFFSET
    }
}

impl Default for RealTarget1HiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RealTarget1Hi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RealTarget1HiVal {
        RealTarget1HiVal::from_bits(self.value.read_bits())
    }
}

/// REAL_TARGET2_LO Register
///
/// system timer comp2 actual target value low register
///
/// Address offset: 0x0084
///
/// ## Fields
///
/// - **TARGET2_LO_RO** `[31:0]` (RO)
///
///   actual target value value low 32bits
#[repr(transparent)]
pub struct RealTarget2Lo {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RealTarget2Lo {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RealTarget2Lo {
    // ======== Field Constants ========

    /// **TARGET2_LO_RO**
    ///
    /// actual target value value low 32bits
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TARGET2_LO_RO_OFFSET: u32 = 0;
    pub const TARGET2_LO_RO_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TARGET2_LO_RO** `[31:0]` (RO)
    ///
    /// actual target value value low 32bits
    #[inline(always)]
    pub fn target2_lo_ro(&self) -> u32 {
        (self.value.read_bits() & Self::TARGET2_LO_RO_MASK) >> Self::TARGET2_LO_RO_OFFSET
    }
}

/// REAL_TARGET2_LO value type for batch read/modify/write operations
///
/// Use `realtarget2lo.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `realtarget2lo.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RealTarget2LoVal {
    bits: u32,
}

impl RealTarget2LoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TARGET2_LO_RO** `[31:0]`
    ///
    /// actual target value value low 32bits
    #[inline(always)]
    pub const fn target2_lo_ro(&self) -> u32 {
        (self.bits & RealTarget2Lo::TARGET2_LO_RO_MASK) >> RealTarget2Lo::TARGET2_LO_RO_OFFSET
    }
}

impl Default for RealTarget2LoVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RealTarget2Lo {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RealTarget2LoVal {
        RealTarget2LoVal::from_bits(self.value.read_bits())
    }
}

/// REAL_TARGET2_HI Register
///
/// system timer comp2 actual target value high register
///
/// Address offset: 0x0088
///
/// ## Fields
///
/// - **TARGET2_HI_RO** `[19:0]` (RO)
///
///   actual target value value high 20bits
#[repr(transparent)]
pub struct RealTarget2Hi {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for RealTarget2Hi {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl RealTarget2Hi {
    // ======== Field Constants ========

    /// **TARGET2_HI_RO**
    ///
    /// actual target value value high 20bits
    ///
    /// Bits `[19:0]`, mask `0x000fffff`
    pub const TARGET2_HI_RO_OFFSET: u32 = 0;
    pub const TARGET2_HI_RO_MASK: u32 = 0x000fffff;

    // ======== Field Accessors ========

    /// **TARGET2_HI_RO** `[19:0]` (RO)
    ///
    /// actual target value value high 20bits
    #[inline(always)]
    pub fn target2_hi_ro(&self) -> u32 {
        (self.value.read_bits() & Self::TARGET2_HI_RO_MASK) >> Self::TARGET2_HI_RO_OFFSET
    }
}

/// REAL_TARGET2_HI value type for batch read/modify/write operations
///
/// Use `realtarget2hi.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `realtarget2hi.write_val(val)`.
#[derive(Clone, Copy)]
pub struct RealTarget2HiVal {
    bits: u32,
}

impl RealTarget2HiVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Read **TARGET2_HI_RO** `[19:0]`
    ///
    /// actual target value value high 20bits
    #[inline(always)]
    pub const fn target2_hi_ro(&self) -> u32 {
        (self.bits & RealTarget2Hi::TARGET2_HI_RO_MASK) >> RealTarget2Hi::TARGET2_HI_RO_OFFSET
    }
}

impl Default for RealTarget2HiVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl RealTarget2Hi {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> RealTarget2HiVal {
        RealTarget2HiVal::from_bits(self.value.read_bits())
    }
}

/// DATE Register
///
/// system timer version control register
///
/// Address offset: 0x00fc
/// Reset value: 0x02201073
///
/// ## Fields
///
/// - **DATE** `[31:0]` (RW)
///
///   systimer register version
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
    /// systimer register version
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const DATE_OFFSET: u32 = 0;
    pub const DATE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **DATE** `[31:0]` (RW)
    ///
    /// systimer register version
    #[inline(always)]
    pub fn date(&self) -> u32 {
        (self.value.read_bits() & Self::DATE_MASK) >> Self::DATE_OFFSET
    }

    /// Write **DATE** `[31:0]` (RW)
    ///
    /// systimer register version
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
        Self { bits: 0x02201073 }
    }

    /// Read **DATE** `[31:0]`
    ///
    /// systimer register version
    #[inline(always)]
    pub const fn date(&self) -> u32 {
        (self.bits & Date::DATE_MASK) >> Date::DATE_OFFSET
    }

    /// Set **DATE** `[31:0]`
    ///
    /// systimer register version
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
    /// Configure system timer clock
    pub conf: Conf,
    /// 0x0004
    /// system timer unit0 value update register
    pub unit0_op: Unit0Op,
    /// 0x0008
    /// system timer unit1 value update register
    pub unit1_op: Unit1Op,
    /// 0x000c
    /// system timer unit0 value high load register
    pub unit0_load_hi: Unit0LoadHi,
    /// 0x0010
    /// system timer unit0 value low load register
    pub unit0_load_lo: Unit0LoadLo,
    /// 0x0014
    /// system timer unit1 value high load register
    pub unit1_load_hi: Unit1LoadHi,
    /// 0x0018
    /// system timer unit1 value low load register
    pub unit1_load_lo: Unit1LoadLo,
    /// 0x001c
    /// system timer comp0 value high register
    pub target0_hi: Target0Hi,
    /// 0x0020
    /// system timer comp0 value low register
    pub target0_lo: Target0Lo,
    /// 0x0024
    /// system timer comp1 value high register
    pub target1_hi: Target1Hi,
    /// 0x0028
    /// system timer comp1 value low register
    pub target1_lo: Target1Lo,
    /// 0x002c
    /// system timer comp2 value high register
    pub target2_hi: Target2Hi,
    /// 0x0030
    /// system timer comp2 value low register
    pub target2_lo: Target2Lo,
    /// 0x0034
    /// system timer comp0 target mode register
    pub target0_conf: Target0Conf,
    /// 0x0038
    /// system timer comp1 target mode register
    pub target1_conf: Target1Conf,
    /// 0x003c
    /// system timer comp2 target mode register
    pub target2_conf: Target2Conf,
    /// 0x0040
    /// system timer unit0 value high register
    pub unit0_value_hi: Unit0ValueHi,
    /// 0x0044
    /// system timer unit0 value low register
    pub unit0_value_lo: Unit0ValueLo,
    /// 0x0048
    /// system timer unit1 value high register
    pub unit1_value_hi: Unit1ValueHi,
    /// 0x004c
    /// system timer unit1 value low register
    pub unit1_value_lo: Unit1ValueLo,
    /// 0x0050
    /// system timer comp0 conf sync register
    pub comp0_load: Comp0Load,
    /// 0x0054
    /// system timer comp1 conf sync register
    pub comp1_load: Comp1Load,
    /// 0x0058
    /// system timer comp2 conf sync register
    pub comp2_load: Comp2Load,
    /// 0x005c
    /// system timer unit0 conf sync register
    pub unit0_load: Unit0Load,
    /// 0x0060
    /// system timer unit1 conf sync register
    pub unit1_load: Unit1Load,
    /// 0x0064
    /// systimer interrupt enable register
    pub int_ena: IntEna,
    /// 0x0068
    /// systimer interrupt raw register
    pub int_raw: IntRaw,
    /// 0x006c
    /// systimer interrupt clear register
    pub int_clr: IntClr,
    /// 0x0070
    /// systimer interrupt status register
    pub int_st: IntSt,
    /// 0x0074
    /// system timer comp0 actual target value low register
    pub real_target0_lo: RealTarget0Lo,
    /// 0x0078
    /// system timer comp0 actual target value high register
    pub real_target0_hi: RealTarget0Hi,
    /// 0x007c
    /// system timer comp1 actual target value low register
    pub real_target1_lo: RealTarget1Lo,
    /// 0x0080
    /// system timer comp1 actual target value high register
    pub real_target1_hi: RealTarget1Hi,
    /// 0x0084
    /// system timer comp2 actual target value low register
    pub real_target2_lo: RealTarget2Lo,
    /// 0x0088
    /// system timer comp2 actual target value high register
    pub real_target2_hi: RealTarget2Hi,
    _reserved0: [u8; 112],
    /// 0x00fc
    /// system timer version control register
    pub date: Date,
}
